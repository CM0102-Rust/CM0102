use super::*;

/// Returns the club's board row in the MANAGER_MANAGER board table: [this] + club id × 0x49.
pub(crate) unsafe fn board_row(this: usize, club: Club) -> usize {
    read_usize(this).wrapping_add((read_i32(club.addr()) as usize).wrapping_mul(0x49))
}

/// Calls `exe::database_is_national_team` (FN_DATABASE_IS_NATIONAL_TEAM): nonzero if the club is a national team.
pub(crate) unsafe fn manager_manager_is_national_team(c: Club) -> u32 {
    exe::database_is_national_team(c)
}

/// The weight of a match for board confidence (the double at [esp+0x2c]) and the factor (the double at [esp+0x34], 0.5
/// initially), by the match's competition.
///
/// No competition: 0.01; outside the club's competition (`exe::comp_is_club_still_in_contention`): 0.2; friendly (`exe::friendly_is_friendly_competition`): 0.67 if the home side is
/// a national team, otherwise 0.05; not played (+0x40): 0.1; in the home side's league: 1, with factor 0.06 × (league
/// +0x69) + 0.27; otherwise 1 − |500 × (+0x69) − reputation| / 15000, × 1.25 in the cup (+0x3a = 8) and by stage
/// (+0x30: 0x78 × 1.25, 0x82 × 1.5, 0x8c × 0.75, 0x96 and 0xa0 × 2).
pub(crate) unsafe fn match_weight(club: Club, fixture: Fixture, factor: &mut f64) -> f64 {
    let comp = fixture.competition();
    if comp.is_null() {
        return read_f64(rdata::F64_0_01);
    }
    let obj = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add((read_i32(fixture.addr()) as usize).wrapping_mul(4)));
    if exe::comp_is_club_still_in_contention(obj, club) == 0 {
        return read_f64(rdata::F64_0_2);
    }
    if exe::friendly_is_friendly_competition(read_usize(G_FRIENDLY), comp) != 0 {
        return if manager_manager_is_national_team(fixture.home_club()) != 0 { read_f64(rdata::F64_0_67) } else { read_f64(rdata::F64_0_05) };
    }
    if fixture.home_side_mark() == 0 {
        return read_f64(rdata::F64_0_1);
    }
    if comp == fixture.home_club().division() {
        *factor = comp.reputation() as i32 as f64 * read_f64(rdata::F64_0_06) + read_f64(rdata::F64_0_27);
        return read_f64(rdata::F64_1);
    }
    let x = (comp.reputation() as i32).wrapping_mul(500).wrapping_sub(club.reputation() as i32);
    let mut p = read_f64(rdata::F64_1) - (x as f64).abs() * read_f64(rdata::F64_6_666666667ENEG_05);
    if fixture.category() == match_category::CONTINENTAL_LATE_ROUND {
        p *= read_f64(rdata::F64_1_25);
    }
    match fixture.round_name() as u16 {
        0x78 => p *= read_f64(rdata::F64_1_25),
        0x82 => p *= read_f64(rdata::F64_1_5),
        0x8c => p *= read_f64(rdata::F64_0_75),
        0x96 | 0xa0 => p = p + p,
        _ => {}
    }
    p
}

/// The rating of one line-up row (0x49 bytes; `e` points to the rating byte at +0x1c0/+0x1c4): the byte that enters the
/// averages (0 = left out); None when not used.
///
/// With the match's +0x4c, the 11 starters count as 90; substitutes (k ≥ 11) only with the entry byte (e − 2 ≠ 0xff)
/// and minus it.
///
/// `nota`, `troca` are the offsets of the two bytes tried in order (the first ≥ 0 counts; otherwise the match's +0x4d).
pub(crate) unsafe fn lineup_rating(game: MatchRecord, e: usize, k: i32, rating: usize, swap: usize, entry: usize) -> Option<i8> {
    if game.no_statistics() != 0 && k < 11 {
        return Some(0x5a);
    }
    let choose = || {
        let v = read_i8(e + rating);
        if v > -1 {
            return v;
        }
        let v = read_i8(e + swap);
        if v > -1 { v } else { game.minute() as i8  }
    };
    if k < 11 {
        return Some(choose());
    }
    let dl = read_i8(e + entry);
    if dl == -1 {
        return None;
    }
    Some(choose().wrapping_sub(dl))
}

/// Adjusts the board's confidence after the match: the rivalry, the reputation gap or the team and opponent ratings in
/// the match, the score, the aggregate, the derby and rival, the competition weight; writes the four confidence values
/// (1 to 10000, with floors by stage), raises the media stories and the fan protest, and decides whether to consider
/// sacking the manager.
///
/// `MANAGER_MANAGER::update_board_confidence_after_match(club, fixture, game, home)`, called from the board after the
/// match (`update_board_after_fixture`, one side at a time). The adjustment starts from the rivalry (±20/30 against the
/// opponent or by the fixture's national side `.stadium()`), adds the reputation difference / 35 (without a match) or,
/// with the match, the team rating (average of the player ratings weighted by the minutes on the pitch,
/// `match_record::ROW_MINUTE_LEFT_PITCH`; the side's shots and shots on target,
/// `.home_shots()`/`.home_shots_on_target()` of its block; the shot difference with the opponent; the sum of +0x4d −
/// rating; x87: (... + 0.1) × 67 − 470, × factor) plus the average reputation of the opponents (by nation,
/// `exe::database_is_staff_foreign`) against the club's / 40, up to ±100. By the score
/// (`fixture::HOME_GOALS_EXTRA_TIME`/`fixture::AWAY_GOALS_EXTRA_TIME` or `fixture::HOME_GOALS`/`fixture::AWAY_GOALS`):
/// win 0x800, defeat 0x1000 (outside friendlies, × 2 or × 5 with a thrashing; below −50 with an AI manager, the request
/// to the FA `exe::manager_manager_handle_fa_request` by random draw), draw 0x400 (in the cup, +15 and the stages
/// 0x7b/0x7c/0x85/0x86/0x99/0x9a). The aggregate (`.home_aggregate()`/`.away_aggregate()`,
/// `.home_penalties()`/`.away_penalties()`, `.home_goals()`/`.away_goals()`) gives 0x100/0x200 and ±10/15/20; the derby
/// (0x40, same city or a rival within 25 km) or the rival/clash (0x80) multiply the weight (2.25 or 1.5) and give
/// ±12/±5. Weight × 0.5 for the B national team, × 2 for the senior one, × 2 or 1.5 in the cup (flag
/// `expectation_flags_byte1::EUROPEAN_SUCCESS` (0x400) of the row's `board_record::EXPECTATION_FLAGS`); the adjustment
/// × weight, ÷ 3 or ÷ 2 by the flags. The flags 1/2/4/8/0x10/0x20 (by the score, the adjustment and confidence
/// `board_record::CONFIDENCE`) go into the text (`FN_MANAGER_MANAGER_BUILD_BOARD_MATCH_REACTION_TEXT`) and into the
/// reaction news (`exe::manager_manager_post_board_match_reaction_news`, human manager only). The media story 0x3d
/// (`exe::media_post_team_media_story`, −250 at `board_record::FAVOURITES_MOOD`) by random draw at a manageable club;
/// story 0x5c in the league with the club in danger. The confidence: `board_record::CONFIDENCE` += 0.7 ×,
/// `board_record::STAFF_CONFIDENCE` += 0.5 ×, `board_record::SUPPORTERS_CONFIDENCE` += 1.15 ×,
/// `board_record::CONFIDENCE_4` += 0.2 × (half with flag `exe::player_regen_get_retirement_status` when negative),
/// between 1 and 10000; floors by stage and by the competition's strength; the fan protest
/// (`exe::manager_manager_post_supporter_protest_news`); in the league or when negative, consider sacking
/// (`exe::manager_manager_consider_sacking_manager`, 1 on a defeat); otherwise it leaves the ultimatum list
/// (`manager_manager_fields::ULTIMATUM_LIST`, `exe::llist_remove`).
/// Original: `MANAGER_MANAGER::update_board_confidence_after_match` (0x695240, manager_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn update_board_confidence_after_match(this: usize, club: usize, fixture: usize, game: usize, home: u32) {
    let game = MatchRecord(game);
    let fixture = Fixture(fixture);
    let club = Club(club);
    let other = Club(if home != 0 { fixture.away_club().addr() } else { fixture.home_club().addr() });
    let vis = home == 0;
    let mut factor: f64 = 0.5;
    let mut weight = match_weight(club, fixture, &mut factor);
    // the contest (by the match’s national team +0x18)
    let ev = Stadium(fixture.stadium());
    let city = |p: usize| p != 0 && read_usize(p + stadium::CITY) != 0;
    let mut aj: i32 = 0;
    if manager_manager_is_national_team(club) != 0 {
        if city(ev.addr()) && read_usize(ev.city() + city::NATION) == club.nation().addr() {
            aj = -20;
        } else if club.stadium() == ev {
            aj = -20;
        } else if other.stadium() == ev {
            aj = 20;
        } else if city(ev.addr()) && read_usize(ev.city() + city::NATION) == other.nation().addr() {
            aj = 20;
        }
    } else if club.stadium() == ev {
        aj = -20;
    } else if other.stadium() == ev {
        aj = if other.nation() != club.nation() { 30 } else { 20 };
    }
    let limitar = |q: i32| {
        let d = q as i16;
        if d < -100 { -100 } else if d > 100 { 100 } else { q }
    };
    let c: i32;
    if game.is_null() {
        let x = (club.reputation() as i32).wrapping_sub(other.reputation() as i32);
        c = limitar(x / -35);
        aj = aj.wrapping_add(c);
    } else {
        let base = MatchRecord(game.addr() + match_record::SIDE_SIZE * vis as usize);
        let (mut s1, mut s2, mut di) = (0i32, 0i32, 0u16);
        for k in 0..20i32 {
            let e = base.addr() + match_record::ROW_MINUTE_LEFT_PITCH + k as usize * 0x49;
            if read_i32(e - 0x34) == -1 {
                continue;
            }
            // the sum of +0x4d − rating, only when the rating (e) is valid
            let quatro_d = game.minute();
            let al = if game.no_statistics() != 0 && k < 11 {
                0x5a
            } else if k < 11 {
                let v = read_i8(e);
                if v > -1 {
                    di = di.wrapping_add(quatro_d.wrapping_sub(v as i16) as u16);
                    v
                } else {
                    let v = read_i8(e - 4);
                    if v > -1 { v } else { game.minute() as i8  }
                }
            } else {
                let dl = read_i8(e - 2);
                if dl == -1 {
                    continue;
                }
                let v = read_i8(e);
                if v > -1 {
                    di = di.wrapping_add(quatro_d.wrapping_sub(v as i16) as u16);
                    v.wrapping_sub(dl)
                } else {
                    let v = read_i8(e - 4);
                    (if v > -1 { v } else { game.minute() as i8  }).wrapping_sub(dl)
                }
            };
            if al == 0 {
                continue;
            }
            s1 = s1.wrapping_add(al as i32);
            s2 = s2.wrapping_add((read_u8(e + 1) as i32).wrapping_mul(al as i32));
        }
        let average = if s1 != 0 { s2 as f64 / s1 as f64 } else { factor };
        let ob = MatchRecord(game.addr() + match_record::SIDE_SIZE * (!vis) as usize);
        let b127 = base.home_shots() as i32;
        let b128 = base.home_shots_on_target() as i32;
        let o127 = ob.home_shots() as i32;
        covered!("cqc_8 match rating");
        let mut v = (b127 - 10) as f64 * read_f64(rdata::F64_0_03333333333) + average;
        v += (b128 - 10) as f64 * read_f64(rdata::F64_0_03333333333);
        v += (b127 - o127) as f64 * read_f64(rdata::F64_0_02);
        v += (di as i16) as f64 * read_f64(rdata::F64_0_006666666667);
        v += read_f64(rdata::F64_0_1);
        v = v * read_f64(rdata::F64_67) - read_f64(rdata::F64_470); // cqc8: the match rating (× 67 − 470)
        v *= factor;
        aj = aj.wrapping_add(ftol(v));
        // the average reputation of the opponents selected (by the club’s nation)
        let (mut s1, mut s2) = (0i32, 0i32);
        for k in 0..20i32 {
            let e = ob.addr() + match_record::ROW_MINUTE_OFF + k as usize * 0x49;
            let id = read_i32(e - 0x30);
            if id == -1 {
                continue;
            }
            let p = read_usize(G_STAFF_TABLE).wrapping_add((id as usize).wrapping_mul(staff::SIZE));
            let nation = club.nation().addr();
            let sheet = PlayerSheet(read_usize(p + staff::PLAYER));
            let rep = if other.nation().addr() == nation {
                sheet.current_reputation()
            } else if read_usize(p + staff::NATION) == nation {
                sheet.home_reputation()
            } else if exe::database_is_staff_foreign(p, nation, 1) == 0 {
                read_i16(read_usize(p + staff::PLAYER) + player_sheet::HOME_REPUTATION)
            } else {
                read_i16(read_usize(p + staff::PLAYER) + player_sheet::WORLD_REPUTATION)
            };
            let al = match lineup_rating(game, e, k, 4, 0, 2) {
                Some(a) => a,
                None => continue,
            };
            if al == 0 {
                continue;
            }
            s1 = s1.wrapping_add(al as i32);
            s2 = s2.wrapping_add((rep as i32).wrapping_mul(al as i32));
        }
        // simplification: with nobody selected the original adds the match pointer (the leftover ecx): reproduced
        let m = if s1 != 0 { s2.wrapping_div(s1) } else { fixture.addr() as i32 };
        let x = (club.reputation() as i32).wrapping_sub((other.reputation() as i32).wrapping_add(m) / 2);
        c = limitar(x / -40);
        aj = aj.wrapping_add(c);
    }
    // the score: A and B (the ratings of whoever the club is), C and D (the aggregate), as the original stores them
    let p8 = |o: usize| read_i8(fixture.addr() + o) as i32;
    let (a, b, cc, dd) = if fixture.home_goals_extra_time() > -1 {
        if home != 0 { (p8(fixture::HOME_GOALS_EXTRA_TIME), p8(fixture::AWAY_GOALS_EXTRA_TIME), p8(fixture::HOME_AGGREGATE), p8(fixture::AWAY_AGGREGATE)) } else { (p8(fixture::AWAY_GOALS_EXTRA_TIME), p8(fixture::HOME_GOALS_EXTRA_TIME), p8(fixture::AWAY_AGGREGATE), p8(fixture::HOME_AGGREGATE)) }
    } else if home != 0 {
        (p8(fixture::HOME_GOALS), p8(fixture::AWAY_GOALS), p8(fixture::HOME_AGGREGATE), p8(fixture::AWAY_AGGREGATE))
    } else {
        (p8(fixture::AWAY_GOALS), p8(fixture::HOME_GOALS), p8(fixture::AWAY_AGGREGATE), p8(fixture::HOME_AGGREGATE))
    };
    let c16 = c as i16 as i32;
    let mut fl: u32;
    if a > b {
        fl = 0x800;
        covered!("cqc_8 win");
        aj = aj.wrapping_add((c16 / 10 + 20).wrapping_mul(a - b)); // cqc8: a vitória
    } else if a < b {
        aj = aj.wrapping_add((20 - c16 / 10).wrapping_mul(a - b));
        fl = 0x1000;
        let comp = fixture.competition();
        let mut end = false;
        if !comp.is_null() && exe::friendly_is_friendly_competition(read_usize(G_FRIENDLY), comp) == 0 {
            let dif = b - a;
            if dif > 10 {
                aj = aj.wrapping_mul(5);
                if (aj as i16) < 0x4e2 {
                    aj = 0x4e2;
                    end = true;
                }
            } else if dif > 5 {
                aj = aj.wrapping_add(aj);
            }
        }
        if !end && (aj as i16) < -0x32 && !club.is_null() {
            let manager = club.manager().addr();
            let human: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
            if manager != 0 && read_usize(manager + staff::NON_PLAYER) != 0 && human(club.addr(), 0, 1, 0, 0) == 0 {
                let manager = club.manager().addr();
                let r = friendly_random(6);
                if read_i8(manager + staff::TEMPERAMENT) < (r as u8).wrapping_add(6) as i8
                    && read_i8(read_usize(G_CLUB_STATE_TABLE).wrapping_add((read_i32(club.addr()) as usize).wrapping_mul(6)) + 4) < 0
                    && friendly_random(read_i8(read_usize(manager + staff::NON_PLAYER) + non_player::PATIENCE) as i32 + 1) == 0
                {
                    exe::manager_manager_handle_fa_request(this, club, 1);
                }
            }
        }
    } else {
        fl = 0x400;
        if fixture.category() == match_category::CONTINENTAL_LATE_ROUND {
            let f = fixture.round_prefix() as i32;
            if (0x41f..=0x426).contains(&f) && vis {
                aj = aj.wrapping_add(0xf);
            }
            let s = (fixture.round_name() as i32).wrapping_sub(0x7b);
            if (0..=0x1f).contains(&s) && matches!(s, 0 | 1 | 10 | 11 | 30 | 31) {
                aj = aj.wrapping_add(if vis { a.wrapping_mul(10) } else { b.wrapping_mul(-5) });
            }
        }
    }
    // o agregado
    let g4b = fixture.home_aggregate();
    if g4b != -1 {
        let g4c = fixture.away_aggregate();
        // (mark, adjustment) per side
        let more = |fl: &mut u32, aj: &mut i32, n: i32| {
            *fl |= 0x200;
            *aj = aj.wrapping_add(n);
        };
        let minus = |fl: &mut u32, aj: &mut i32, n: i32| {
            *fl |= 0x100;
            *aj = aj.wrapping_sub(n);
        };
        if g4b > g4c {
            if home != 0 { more(&mut fl, &mut aj, 20) } else { minus(&mut fl, &mut aj, 20) }
        } else if g4b < g4c {
            if home == 0 { more(&mut fl, &mut aj, 20) } else { minus(&mut fl, &mut aj, 20) }
        } else {
            let (g49, g4a) = (fixture.home_penalties(), fixture.away_penalties());
            if g49 > g4a {
                if home != 0 { more(&mut fl, &mut aj, 10) } else { minus(&mut fl, &mut aj, 10) }
            } else if g49 < g4a {
                if home == 0 { more(&mut fl, &mut aj, 10) } else { minus(&mut fl, &mut aj, 10) }
            } else if (fixture.tie_rules() as u8) & 4 == 0 {
                let x = g4b as i32 - fixture.home_goals() as i32;
                let away = x > fixture.away_goals() as i32;
                if away == (home != 0) { minus(&mut fl, &mut aj, 15) } else { more(&mut fl, &mut aj, 15) }
            }
        }
    }
    // the derby and the rival
    let rival = club.rival1() == other.addr() || club.rival2() == other.addr() || club.rival3() == other.addr();
    let f1 = club.stadium();
    let same = !f1.is_null() && other.stadium().addr() != 0 && f1.city() == other.stadium().city();
    if same {
        fl |= if rival { 0x40 } else { 0x80 };
    } else if rival {
        fl |= 0x80;
        if !f1.is_null() {
            let e1 = f1.city();
            let f2 = other.stadium();
            if e1 != 0 && !f2.is_null() {
                let e2 = f2.city();
                if e2 != 0 {
                    let x = read_f64(e1 + city::LONGITUDE) - read_f64(e2 + city::LONGITUDE);
                    let y = read_f64(e1 + city::LATITUDE) - read_f64(e2 + city::LATITUDE);
                    if (y * y + x * x).sqrt() * read_f64(rdata::F64_55) <= read_f64(rdata::F64_25) {
                        fl |= 0x40;
                    }
                }
            }
        }
    } else {
        if exe::comp_util_get_league_clash_type(club, other) != 0 {
            fl |= 0x80;
        }
    }
    if fl & 0x40 != 0 {
        weight *= read_f64(rdata::F64_2_25);
        if fl & 0x800 != 0 {
            aj = aj.wrapping_add(0xc);
        } else if fl & 0x1000 != 0 {
            aj = aj.wrapping_sub(0xc);
        }
    } else if fl & 0x80 != 0 {
        weight *= read_f64(rdata::F64_1_5);
        if fl & 0x800 != 0 {
            aj = aj.wrapping_add(5);
        } else if fl & 0x1000 != 0 {
            aj = aj.wrapping_sub(5);
        }
    }
    let mut mult = if exe::club_is_national_team(club) != 0 {
        weight * read_f64(rdata::F64_0_5)
    } else {
        let s = manager_manager_is_national_team(club);
        if s != 0 { weight + weight } else { weight }
    };
    if fixture.category() == match_category::CONTINENTAL_LATE_ROUND {
        if read_u8(board_row(this, club) + board_record::EXPECTATION_FLAGS + 1) & expectation_flags_byte1::EUROPEAN_SUCCESS != 0 { // a palavra +0xe & 0x400
            mult = mult + mult;
            if (aj as i16) < 0 {
                aj = ftol((aj as i16) as f64 * read_f64(rdata::F64_1_5));
            }
        } else {
            mult *= read_f64(rdata::F64_1_5);
        }
    }
    let mut dx = ftol((aj as i16) as f64 * mult);
    if fl & 0x800 != 0 && (dx as i16) < 0 {
        dx = (dx as i16 as i32) / 3;
    }
    if fl & 0x1000 != 0 && (dx as i16) > 0 {
        dx = (dx as i16 as i32) / 3;
    }
    if fl & 0x200 != 0 && (dx as i16) < 0 {
        dx = (dx as i16 as i32) / 2;
    }
    if fl & 0x100 != 0 && (dx as i16) > 0 {
        dx = (dx as i16 as i32) / 2;
    }
    let aj = dx;
    let d16 = aj as i16;
    let conf6 = || read_i16(board_row(this, club) + board_record::CONFIDENCE);
    if a + 5 < b {
        fl |= if club.is_null() || conf6() < 0x10b3 { 1 } else { 2 };
    } else if a - 5 > b {
        fl |= 8;
    } else if d16 < -0x78 && (club.is_null() || conf6() < 0x10b3) {
        fl |= 1;
    } else if d16 < -0x4b && (club.is_null() || conf6() < 0x1964) {
        fl |= 2;
    } else if d16 < -0x1e {
        fl |= 4;
    } else if d16 > 0x78 && !club.is_null() && conf6() > 0x1964 {
        fl |= 8;
    } else if d16 > 0x4b {
        fl |= 0x20;
    } else if d16 > 0x1e {
        fl |= 0x10;
    }
    // the text and the news of the reaction
    let marks = (fixture.tie_rules() as i16) as u16;
    if marks & 0x2810 == 0 {
        let text: unsafe extern "thiscall" fn(usize, usize, usize, usize, i32, usize, i32, i32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(FN_MANAGER_MANAGER_BUILD_BOARD_MATCH_REACTION_TEXT);
        let human: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_NEWS_CLUB_HAS_HUMAN_MANAGER);
        if text(this, club.addr(), other.addr(), fixture.competition().addr(), fl as i16 as i32, 0xad9c64, -1, -1, 0, 0, 0, 0, 0, 0, 0, 0) != 0
            && human(read_usize(G_NEWS), club.addr()) != 0
        {
            let c6 = conf6();
            if c6 > 0x1a5e {
                if fl & 1 != 0 {
                    fl ^= 5;
                } else if fl & 2 != 0 {
                    fl ^= 6;
                }
            } else if c6 > 0x1482 {
                if fl & 1 != 0 {
                    fl ^= 3;
                }
            } else if c6 < 0x1482 && fl & 8 != 0 {
                fl ^= 0x28;
            }
            let lim = ((vis as u16).wrapping_mul(0x4e2)).wrapping_add(other.reputation() as u16).wrapping_add(0x1f4) as i16;
            if fl & 7 == 0 || club.reputation() >= lim || fixture.home_goals() != fixture.away_goals() {
                exe::manager_manager_post_board_match_reaction_news(this, club, other, fixture, fl, a, b, cc, dd);
            }
        }
    }
    // media story 0x3d
    let marks = (fixture.tie_rules() as i16) as u16;
    if fl & 0xa00 == 0 && marks & 0x2810 == 0 && exe::club_can_be_managed(club) != 0 {
        let s = aj as i16 as i32;
        let oito = || read_i16(board_row(this, club) + board_record::SUPPORTERS_CONFIDENCE) as i32;
        let r = friendly_random(0x5a);
        let mut sai = false;
        if s < -0x5a - r {
            let r2 = friendly_random(0x7d0);
            sai = oito().wrapping_add(s.wrapping_mul(10)) < 0xfa0 - r2;
        }
        if !sai {
            let r3 = friendly_random(0x78);
            sai = s < -0x1e - r3 && oito().wrapping_add(s.wrapping_mul(10)) < 0x898;
        }
        if sai {
            exe::media_post_team_media_story(read_usize(G_MEDIA), club, 0x3d, 0, fixture, 0, -1);
            let l = board_row(this, club) + board_record::FAVOURITES_MOOD;
            write_i16(l, read_i16(l).wrapping_add(-0xfa));
        }
    }
    // story 0x5c: the club in danger in the league
    let id = read_i32(fixture.addr());
    if id > -1 && id == (club.continental_comp_id() as i32) && id != read_i32(KEY_CLUB_COMP_INTER_TOTO_CUP) && fixture.category() == match_category::CONTINENTAL_LATE_ROUND
        && (fixture.round_name() as i32 / 10) * 10 != 0x96
    {
        let co = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add((id as usize).wrapping_mul(4)));
        if co != 0 && read_usize(co + 8) != 0 {
            let ln = CompStatsClub(exe::comp_stats_find_club_entry(read_usize(co + 8), club));
            if fl & 0x100 == 0 {
                let n = ln.final_position();
                if n < 9 && n != 0 && ln.best_position() == 1 && (aj as i16 as i32) > (n as i32 * n as i32) / 2 + 0x32 {
                    exe::media_post_team_media_story(read_usize(G_MEDIA), club, 0x5c, 0, fixture, 0, -1);
                }
            }
        }
    }
    let mut aj = aj;
    if (aj as i16) < 0 {
        let manager = club.manager().addr();
        if manager != 0 && exe::player_regen_get_retirement_status(read_usize(G_PLAYER_REGEN), manager) != 0 {
            aj = ftol((aj as i16) as f64 * read_f64(rdata::F64_0_5));
        }
    }
    // the confidence
    let v = (aj as i16) as f64;
    covered!("cqc_8 confidence");
    for (o, k) in [(board_record::CONFIDENCE, 0x968e40usize), (board_record::STAFF_CONFIDENCE, 0x968968), (board_record::SUPPORTERS_CONFIDENCE, 0x968980), (board_record::CONFIDENCE_4, 0x968918)] {
        let l = board_row(this, club) + o;
        let t = ftol(v * read_f64(k)); // cqc8: the confidence by the adjustment
        write_i16(l, read_i16(l).wrapping_add(t as i16));
    }
    for o in [board_record::CONFIDENCE, board_record::STAFF_CONFIDENCE, board_record::SUPPORTERS_CONFIDENCE, board_record::CONFIDENCE_4] {
        let l = board_row(this, club) + o;
        let x = read_i16(l);
        if x > 0x2710 {
            write_i16(l, 0x2710);
        } else if x < 1 {
            write_i16(l, 1);
        }
    }
    let floor = |o: usize, v: i16| {
        let l = board_row(this, club) + o;
        if read_i16(l) < v {
            write_i16(l, v);
        }
    };
    let comp = fixture.competition();
    if (aj as i16) > 0 && club.manager().addr() != 0 {
        if !comp.is_null() {
            let n = comp.reputation();
            if n > 5 {
                let rep = club.reputation();
                let stage = fixture.round_name() as u16;
                if (n as i32) * 500 > rep as i32 {
                    if stage == 0x82 || stage == 0x78 {
                        floor(board_record::CONFIDENCE, 0x1964);
                        floor(board_record::SUPPORTERS_CONFIDENCE, 0x1964);
                    } else if stage == 0x96 {
                        floor(board_record::CONFIDENCE, 0x2134);
                        floor(board_record::SUPPORTERS_CONFIDENCE, 0x2134);
                    } else if comp != club.division() && (rep < 0xea6 || fixture.category() == match_category::CONTINENTAL_LATE_ROUND) {
                        floor(board_record::CONFIDENCE, 0x10b3);
                        floor(board_record::SUPPORTERS_CONFIDENCE, 0x1964);
                    }
                } else if stage == 0x82 || stage == 0x96 {
                    floor(board_record::CONFIDENCE, 0x1964);
                    floor(board_record::SUPPORTERS_CONFIDENCE, 0x1964);
                } else if comp != club.division() && (stage == 0x78 || fixture.category() == match_category::CONTINENTAL_LATE_ROUND) {
                    floor(board_record::CONFIDENCE, 0x10b3);
                    floor(board_record::SUPPORTERS_CONFIDENCE, 0x10b3);
                }
            }
        }
    } else if (aj as i16) < 0 && !comp.is_null() && fixture.round_name() == round_name::FINAL && fixture.category() == match_category::LEAGUE_PLAYOFF {
        let l = board_row(this, club);
        if read_u8(l + board_record::EXPECTATION_FLAGS) & (expectation_flags::PROMOTION as u8) != 0 {
            if read_i16(l + board_record::CONFIDENCE) > 0x10b3 {
                write_i16(l + board_record::CONFIDENCE, 0x10b3);
            } else if read_i16(l + board_record::SUPPORTERS_CONFIDENCE) > 0xdac {
                write_i16(l + board_record::CONFIDENCE, 0xdac);
            }
            let x = read_i16(l + board_record::SUPPORTERS_CONFIDENCE);
            if x > 0x10b3 {
                write_i16(l + board_record::SUPPORTERS_CONFIDENCE, 0x10b3);
            } else if x > 0xdac {
                write_i16(l + board_record::SUPPORTERS_CONFIDENCE, 0xdac);
            }
        }
    }
    if club.manager().addr() != 0 && (aj as i16) < -0x4b && read_i16(board_row(this, club) + board_record::SUPPORTERS_CONFIDENCE) < 0xbb8 {
        exe::manager_manager_post_supporter_protest_news(this, club);
    }
    if fixture.competition() == club.division() || (aj as i16) < 0 {
        if fixture.round_name() != round_name::PLAYOFF {
            exe::manager_manager_consider_sacking_manager(this, club, (a < b) as u32);
        }
        return;
    }
    let list = Llist(read_usize(this + manager_manager_fields::ULTIMATUM_LIST));
    if read_i16(list.addr()) == 0 {
        return;
    }
    list.set_cursor((list.head() as u32) as usize);
    let mut i = 0i32;
    while i < read_i16(read_usize(this + manager_manager_fields::ULTIMATUM_LIST)) as i32 {
        let e = exe::llist_get_next(read_usize(this + manager_manager_fields::ULTIMATUM_LIST));
        if e != 0 && read_i32(club.addr()) == read_i32(e) {
            exe::llist_remove(read_usize(this + manager_manager_fields::ULTIMATUM_LIST), e);
            return;
        }
        i += 1;
    }
}

// simplification: calls the request to the federation (0x6a9e60), the reaction news (0x6a5fa0), the media stories (0x74b060), the
// protest, the dismissal (0x69aba0) and removing an item from the ultimatum list (0x68d1f0): allocate, free and roll random numbers depending on the
// path; does not run twice. Original only; the copy takes the branches and is proven by the career and the mutations.

/// Posts news item 0x138c with the board's reaction to the match for the club's manager: the flags, the clubs, the
/// scores, the named players and the match record.
///
/// SEH frame with no destructor on the normal path. `MANAGER_MANAGER::post_board_match_reaction_news(club, other,
/// fixture, marks, a, b, c, d)`: with club, opponent, fixture and competition (`.competition()`), news 0x138c
/// (`exe::new_newsitem`) with the flags, the two ids, the scores a and b, the aggregate c and d and the fixture's
/// `.match_stats_slot()`; the up to 3 players the board names
/// (`FN_MANAGER_MANAGER_COLLECT_BOARD_MATCH_REACTION_PLAYERS`, with defeat = a > b), a random(4), the defeat and their
/// 5 flags; the club's match record (`exe::fill_match_record_from_fixture`) written into the news
/// (`exe::write_match_record_to_news`); delivered to the club's manager (`exe::news_send_to_club_manager`).
/// Original: `MANAGER_MANAGER::post_board_match_reaction_news` (0x6a5fa0, manager_manager.cpp, thiscall).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn post_board_match_reaction_news(this: usize, club: usize, other: usize, fixture: usize, marks: u32, a: i32, b: i32, c: i32, d: i32) {
    let fixture = Fixture(fixture);
    if club == 0 || other == 0 || fixture.is_null() || fixture.competition().is_null() {
        return;
    }
    let citados: unsafe extern "thiscall" fn(usize, usize, usize, *mut i32, usize, *mut u8, *mut u8, *mut u8, *mut u8, *mut u8, *mut u8) -> u8 =
        core::mem::transmute(FN_MANAGER_MANAGER_COLLECT_BOARD_MATCH_REACTION_PLAYERS);
    covered!("cqc_8 reaction news");
    let mut news = [0u32; 0x3a]; // the local news item (0xe0 bytes in the original)
    let n = news.as_mut_ptr() as usize;
    let mut reg = [0u8; 0x20]; // the match record
    let r = reg.as_mut_ptr() as usize;
    // the defeat and the 5 marks of those cited (bytes +0xe, +0x11, +0x13, +0x12, +0xf and +0x10 of the original)
    let (mut m1, mut m2, mut m3, mut m4, mut m5) = (0u8, 0u8, 0u8, 0u8, 0u8);
    exe::new_newsitem(n, 0x138c, 0);
    let mut k: u8 = 0;
    exe::date_date_54a110(r);
    let mut ids = [-1i32; 3];
    for v in [marks as i16 as i32 as u32, read_usize(club) as u32, read_usize(other) as u32, a as u32, b as u32, c as u32, d as u32, fixture.match_stats_slot() as u32] {
        exe::newsitem_set_slot(n, k as i8, v);
        k = k.wrapping_add(1);
    }
    let mut loss = (a > b) as u8;
    citados(this, fixture.addr(), club, ids.as_mut_ptr(), fixture.match_stats_slot(), &mut loss, &mut m1, &mut m2, &mut m3, &mut m4, &mut m5);
    for v in ids {
        exe::newsitem_set_slot(n, k as i8, v as u32);
        k = k.wrapping_add(1);
    }
    let s = friendly_random(4);
    exe::newsitem_set_slot(n, k as i8, s as u32);
    k = k.wrapping_add(1);
    for v in [loss, m1, m2, m3, m4, m5] {
        exe::newsitem_set_slot(n, k as i8, v as i8 as i32 as u32);
        k = k.wrapping_add(1);
    }
    exe::fill_match_record_from_fixture(fixture, club, r);
    exe::write_match_record_to_news(r, n, &mut k);
    exe::news_send_to_club_manager(read_usize(G_NEWS), n, club);
}

// simplification: the news item goes to the manager (allocates): original only; the copy carries the branch.

/// Posts news item 0x1781 with the FA's assessment of the national team manager in a competition, from the game start.
///
/// SEH frame with no destructor on the normal path. `MANAGER_MANAGER::post_fa_performance_news(comp, national_team,
/// rating)`: from the game start date (`G_GAME_START_DATE`) on, news 0x1781 (`exe::new_newsitem`) with the national
/// team's id, the competition's id (*comp) and the FA's rating, through the reach (`exe::news_send_by_scope` with the
/// national team and 0, 0, 0, 0).
/// Original: `MANAGER_MANAGER::post_fa_performance_news` (0x6a5a60, manager_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn post_fa_performance_news(_this: usize, comp: usize, national_team: usize, rating: u32) {
    let day = read_i16(G_GAME_START_DATE);
    let year = read_i16(G_GAME_START_DATE_YEAR) as u16;
    let today = read_i16(G_CURRENT_DATE_YEAR) as u16;
    if year > today || (year == today && day > read_i16(G_CURRENT_DATE)) {
        return;
    }
    covered!("cqc_8 federation evaluation");
    let mut local = [0u32; 0x3a]; // the local news item (0xe8 bytes)
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0x1781, 0);
    exe::newsitem_set_slot(n, 0, read_usize(national_team) as u32);
    exe::newsitem_set_slot(n, 1, read_usize(comp) as u32);
    exe::newsitem_set_slot(n, 2, rating as i8 as i32 as u32);
    exe::news_send_by_scope(read_usize(G_NEWS), n, national_team, (0) as usize, (0) as usize, (0) as usize, 0);
}

// simplification: the news item goes by reach (allocates): original only; the copy takes the branch.

/// Returns the competition object of the club's division: [[G_COMP_OBJECT_TABLE] + division id × 4].
pub(crate) unsafe fn club_division_comp(club: Club) -> usize {
    read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add((read_i32(club.division().addr()) as usize).wrapping_mul(4)))
}

/// Calls a no-argument thiscall virtual method of a competition object at the given vtable offset (+0x98 or +0x9c).
pub(crate) unsafe fn call_comp_virtual(obj: usize, desloc: usize) -> u32 {
    let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(obj) + desloc));
    f(obj)
}

/// Sets the board's season expectations: the target in the league (danger position, wealth, previous division, manager
/// tenure) and the expectation flags (title, European places, top half, avoid relegation), the chairman draws, the
/// departure by tenure, the confidence values pulled back to 6500 and the manager record; a national team takes those
/// of its competition.
///
/// `MANAGER_MANAGER::set_season_expectations(club, news)`. Clears the flags (`board_record::EXPECTATION_FLAGS`) and the
/// target (`board_record::TARGET_POSITION`), `board_record::SPENDING` = reputation² / 4,
/// `board_record::SQUAD_INVESTMENT` ÷ 4, +0 = 0. Without flag `board_record_flags::NEW_MANAGER` of
/// `board_record::FLAGS`, with a manager and outside national teams: by the manager's days in the job
/// (`exe::date_days_since`), above 3650 with confidence `board_record::CONFIDENCE` < 3500 (random(30) above the
/// chairman's `non_player::PATIENCE`, or 10) or above 1825 with `board_record::CONFIDENCE` < 3000 (random(20)), the
/// item {1, 6, id} in list `manager_manager_fields::DEPARTURE_LIST` (`exe::llist_add_tail`). National team: the B team
/// nothing; the senior one takes its competition's expectations (`exe::nationalteams_get_current_competition` and
/// `comp_vtable::GET_QUALIFICATION_STATUS`) and the FA news (`exe::manager_manager_post_fa_expectations_news`). Club:
/// the nation's UEFA places (`exe::uefaseeding_get_nation_places` on `[G_UEFA_SEEDING]`); without a manager nothing;
/// without a division with a competition: `expectation_flags::AVOID_RELEGATION`. The target = the danger position of
/// the league row (`.final_position()`; without it, 8 with `expectation_flags::RESPECTABLE_POSITION` and the news, and
/// it returns), minus 1 if it is the last one; by wealth (`exe::finance_get_wealth_band`: 2 → −3/−2, 1 → −2/−1, 0 with
/// budget (`exe::finance_get_transfer_budget`) above 250 × reputation → −1, −1 → +1 below 5, −2 → +1); by the previous
/// division (`.last_division()`: stronger +4/+2, weaker −7/−3/−2) or by the manager's days (below 365 +1; above 1095 −1
/// above 10); between 1 and the number of clubs − 1; with flag `board_record_flags::NEW_MANAGER`, the average with the
/// row's `.current_position()` between `.best_position()` and `.worst_position()`. The flags:
/// `expectation_flags::TITLE_CHALLENGE` (champion's place), `expectation_flags::EUROPEAN_QUALIFICATION` (UEFA places,
/// outside Australia), `expectation_flags::PROMOTION` (`league::PROMOTED_COUNT`), `expectation_flags::PLAYOFFS`
/// (`league::PROMOTION_PLAYOFF_COUNT`), `expectation_flags::RESPECTABLE_POSITION` (top half),
/// `expectation_flags::RELEGATION_BATTLE` (avoid relegation, `league::RELEGATED_COUNT`) or
/// `expectation_flags::AVOID_RELEGATION`/0; chairman draws (0x80, 0x100 `expectation_flags_byte1::CUP_SUCCESS`, 0x200
/// by the target) and the cup league (`[G_CLUB_COMP_TABLE]`, 0x400 `expectation_flags_byte1::EUROPEAN_SUCCESS` or the
/// swap 0x80/0x200). The news (`exe::manager_manager_post_board_expectations_news`). Confidence values below 6500 rise
/// to (5x + 27500 or 25000) / 10 (`finish_board_update`); the wealth into `board_record::WEALTH_BAND`; the manager's
/// record (`non_player::CURRENT_REPUTATION`, `non_player::WORLD_REPUTATION`) rises to the club's reputation; the new
/// manager (of the last 16) gets 1 in `staff::LOYALTY` up to 20; clears `board_record::FLAGS`,
/// `board_record::REQUEST_STATE`, `board_record::REQUESTS_MADE`, `board_record::WAGE_RISE_GRANTED`,
/// `board_record::LEAGUE_CONFIDENCE`..`board_record::FAVOURITES_MOOD`, `board_record::EXTRA_FUNDS_GRANTED` and
/// `board_record::DISCIPLINE_COUNTERS` & 7; the fines (`exe::fine_halve_club_fine_count` on `[G_FINE]`).
/// Original: `MANAGER_MANAGER::set_season_expectations` (0x6a3770, manager_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn set_season_expectations(this: usize, club: usize, news: u32) {
    let club = Club(club);
    let ln = || board_row(this, club);
    write_i16(ln() + board_record::EXPECTATION_FLAGS, 0);
    write_u32(ln() + board_record::TARGET_POSITION, 0);
    let rep = club.reputation() as i32;
    write_f64(ln() + board_record::SPENDING, rep.wrapping_mul(rep) as f64 * read_f64(rdata::F64_0_25));
    let l38 = ln() + board_record::SQUAD_INVESTMENT;
    write_f64(l38, read_f64(l38) * read_f64(rdata::F64_0_25));
    write_u8(ln(), 0);
    let mut days: i32 = 0;
    if read_u8(ln() + board_record::FLAGS) & board_record_flags::NEW_MANAGER == 0 && club.manager().addr() != 0 && manager_manager_is_national_team(club) == 0 {
        let manager = club.manager();
        days = exe::date_days_since(G_CURRENT_DATE, manager.date_joined_club() as u32, manager.date_joined_club_leap_year() as u32);
        let presence = || {
            let bf = club.chairman().addr();
            if bf != 0 && read_usize(bf + staff::NON_PLAYER) != 0 { Some(read_i8(read_usize(bf + staff::NON_PLAYER) + non_player::PATIENCE) as i32) } else { None }
        };
        let exit = if days > 0xe42 && read_i16(ln() + board_record::CONFIDENCE) < 0xdac {
            match presence() {
                Some(_) => {
                    let r = friendly_random(0x1e);
                    r > presence().unwrap_or(0)
                }
                None => friendly_random(0x1e) > 10,
            }
        } else if days > 0x721 && read_i16(ln() + board_record::CONFIDENCE) < 0xbb8 {
            match presence() {
                Some(_) => {
                    let r = friendly_random(0x14);
                    r > presence().unwrap_or(0)
                }
                None => friendly_random(0x14) > 10,
            }
        } else {
            false
        };
        if exit {
            covered!("cqc_9 departure by time");
            // the 6-byte item {1, 6, club id} copied into the list [this+4]
            let mut item = [0u8; 8];
            item[0] = 1;
            item[1] = 6;
            item[2..6].copy_from_slice(&(read_usize(club.addr()) as u32).to_le_bytes());
            exe::llist_add_tail(read_usize(this + manager_manager_fields::DEPARTURE_LIST), (item.as_mut_ptr()) as *mut u8 as usize, 6);
        }
    }
    if manager_manager_is_national_team(club) != 0 {
        let mut feita = false;
        if exe::club_is_national_team(club) == 0 {
            let ent = exe::nationalteams_get_current_competition(read_usize(G_NATIONAL_TEAMS), club.nation().addr());
            let co = if ent != 0 { read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add((read_i32(ent) as usize).wrapping_mul(4))) } else { 0 };
            if co != 0 {
                let exp: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(read_usize(read_usize(co) + comp_vtable::GET_QUALIFICATION_STATUS));
                let w = exp(co, club.addr()) as u16;
                write_i16(ln() + board_record::EXPECTATION_FLAGS, w as i16);
                write_u32(ln() + board_record::TARGET_POSITION, read_usize(ent) as u32);
                if news as u8 != 0 {
                    // simplification: the original pushes the word with the high half of the register (garbage); 0x6a44a0 reads the word
                    exe::manager_manager_post_fa_expectations_news(this, club, read_i16(ln() + board_record::EXPECTATION_FLAGS) as u16 as u32, read_usize(ln() + board_record::TARGET_POSITION) as u32);
                }
                feita = true;
            } else {
                write_i16(ln() + board_record::EXPECTATION_FLAGS, 0);
                write_u32(ln() + board_record::TARGET_POSITION, 0xffff_ffff);
            }
        }
        if !feita && news as u8 != 0 {
            exe::manager_manager_post_fa_expectations_news(this, club, 0, 0xffff_ffff);
        }
        finish_board_update(this, club);
        return;
    }
    // the nation’s UEFA places: champion (the byte of the 2nd argument) and the others (the 3rd)
    let mut slots: u32 = club.addr() as u32; // the original writes into the slot of the `clube` argument (only the byte is read)
    let mut outras: u8 = 0;
    exe::uefaseeding_get_nation_places(read_usize(G_UEFA_SEEDING), club.nation().addr(), (&mut slots) as *mut u32 as *mut i8, (&mut outras) as *mut u8 as *mut i8);
    let slots = slots as u8 as i8 as i32;
    let outras = outras as i8 as i32;
    if club.manager().addr() == 0 {
        finish_board_update(this, club);
        return;
    }
    let div = club.division().addr();
    let nation = club.nation();
    if div == 0 || nation.is_null() || nation.league_selected() == 0 || club_division_comp(club) == 0 {
        write_i16(ln() + board_record::EXPECTATION_FLAGS, expectation_flags::AVOID_RELEGATION);
        post_expectations_news(this, club, news);
        finish_board_update(this, club);
        return;
    }
    let lg = CompStatsClub(exe::comp_stats_find_club_entry(read_usize(club_division_comp(club) + 8), club));
    let mut cup: usize = 0;
    let id = club.continental_comp_id() as i32;
    if id > -1 && id != read_i32(KEY_CLUB_COMP_INTER_TOTO_CUP) {
        let cid = read_i32(read_usize(G_CLUB_COMP_TABLE).wrapping_add((id as usize).wrapping_mul(0x6b)));
        let co = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add((cid as usize).wrapping_mul(4)));
        cup = exe::comp_stats_find_club_entry(read_usize(co + 8), club);
    }
    if lg.is_null() || lg.final_position() < 1 {
        write_u32(ln() + board_record::TARGET_POSITION, 8);
        write_u8(ln() + board_record::EXPECTATION_FLAGS, read_u8(ln() + board_record::EXPECTATION_FLAGS) | (expectation_flags::RESPECTABLE_POSITION as u8));
        // simplification: the original pushes the word with the high half of the register (garbage); 0x6a4a20 reads the word
        exe::manager_manager_post_board_expectations_news(this, club, read_i16(ln() + board_record::EXPECTATION_FLAGS) as u16 as u32);
        return;
    }
    let meta = || read_i32(ln() + board_record::TARGET_POSITION);
    let por = |v: i32| write_u32(ln() + board_record::TARGET_POSITION, v as u32);
    por(lg.final_position() as i32); // cqc9: the target by the danger position
    let n = read_i16(club_division_comp(club) + league::CLUB_COUNT) as i32;
    if meta() == n {
        por(n - 1);
    }
    let fin = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club);
    covered!("cqc_9 target");
    match exe::finance_get_wealth_band(fin) as i8 {
        2 => por(if meta() > 5 { meta() - 3 } else { meta() - 2 }),
        1 => por(if meta() > 5 { meta() - 2 } else { meta() - 1 }),
        0 => {
            let v = exe::finance_get_transfer_budget(fin, 0, 0, 0, 1, 0);
            if v > (club.reputation() as i32 * 250) as i64 && meta() > 5 {
                por(meta() - 1);
            }
        }
        -1 => {
            if meta() < 5 {
                por(meta() + 1);
            }
        }
        -2 => por(meta() + 1),
        _ => {}
    }
    let (d57, d5b) = (club.division().addr(), club.last_division().addr());
    if d57 != 0 && d5b != 0 && d57 != d5b {
        if read_i16(d57 + competition::REPUTATION) > read_i16(d5b + competition::REPUTATION) {
            if meta() > 10 {
                por(meta() + 1);
            }
            por(if meta() > 5 { meta() + 2 } else { meta() + 4 });
        } else {
            let x = meta();
            if x > 10 {
                por(x - 7);
            } else if x > 5 {
                por(x - 3);
            } else if x > 2 {
                por(x - 2);
            }
        }
    } else if days < 0x16d {
        por(meta() + 1);
    } else if days > 0x447 {
        if meta() > 10 {
            por(meta() - 1);
        }
    } else if days > 0x721 {
        // (unreachable: the original tests after days ≤ 1095)
        if meta() > 10 {
            por(meta() - 2);
        }
        if meta() > 5 {
            por(meta() - 1);
        }
    }
    let x = meta();
    let x = if x < 1 {
        1
    } else {
        let lim = read_i16(club_division_comp(club) + league::CLUB_COUNT) as i32 - 1;
        if lim < x { lim } else { x }
    };
    por(x as i8 as i32);
    if read_u8(ln() + board_record::FLAGS) & board_record_flags::NEW_MANAGER != 0 && lg.best_position() >= 1 && lg.worst_position() >= 1 {
        let v = (lg.current_position() as i32).wrapping_add(meta());
        por(ftol(v as f64 * read_f64(rdata::F64_0_5)) as i8 as i32);
        let (lo, hi) = (lg.best_position() as i32, lg.worst_position() as i32);
        let x = meta();
        let x = if lo > x { lo } else if hi < x { hi } else { x };
        por(x as i8 as i32);
    }
    // the marks by target
    let co = club_division_comp(club);
    let ou = |m: u8| write_u8(ln() + board_record::EXPECTATION_FLAGS, read_u8(ln() + board_record::EXPECTATION_FLAGS) | m);
    'marcas: {
        if call_comp_virtual(co, league_vtable::HAS_AUTOMATIC_PROMOTION) == 0 && meta() <= slots {
            ou(expectation_flags::TITLE_CHALLENGE as u8);
            break 'marcas;
        }
        let na = club.nation().addr();
        if na != 0 && read_i32(na) != read_i32(KEY_NATION_AUSTRALIA) && call_comp_virtual(club_division_comp(club), league_vtable::HAS_AUTOMATIC_PROMOTION) == 0 && meta() <= slots + outras {
            ou(expectation_flags::EUROPEAN_QUALIFICATION as u8);
            break 'marcas;
        }
        let co = club_division_comp(club);
        if call_comp_virtual(co, league_vtable::HAS_AUTOMATIC_PROMOTION) != 0 && meta() <= read_i8(club_division_comp(club) + league::PROMOTED_COUNT) as i32 {
            ou(expectation_flags::PROMOTION as u8);
            break 'marcas;
        }
        let co = club_division_comp(club);
        let bf = read_i8(co + league::PROMOTION_PLAYOFF_COUNT);
        if bf != 0 && meta() <= bf as i32 {
            ou(expectation_flags::PLAYOFFS as u8);
            break 'marcas;
        }
        if meta() <= read_i16(co + league::CLUB_COUNT) as i32 / 2 {
            write_i16(ln() + board_record::EXPECTATION_FLAGS, expectation_flags::RESPECTABLE_POSITION);
            break 'marcas;
        }
        if call_comp_virtual(co, league_vtable::HAS_RELEGATION) != 0 {
            let co = club_division_comp(club);
            if meta() >= read_i16(co + league::CLUB_COUNT) as i32 - read_i8(co + league::RELEGATED_COUNT) as i32 {
                ou(expectation_flags::RELEGATION_BATTLE as u8);
                break 'marcas;
            }
        }
        let r = call_comp_virtual(club_division_comp(club), league_vtable::HAS_RELEGATION);
        write_i16(ln() + board_record::EXPECTATION_FLAGS, if r != 0 { expectation_flags::AVOID_RELEGATION } else { 0 });
    }
    // the chairman’s random rolls and the cup’s league
    let bf = club.chairman().addr();
    if read_u8(ln() + board_record::FLAGS) & board_record_flags::NEW_MANAGER == 0 && call_comp_virtual(club_division_comp(club), league_vtable::HAS_AUTOMATIC_PROMOTION) == 0 && bf != 0 && read_usize(bf + staff::NON_PLAYER) != 0 {
        let p1d = || (club.chairman().non_player().patience() as i8) as i32;
        covered!("cqc_9 chairman draw");
        let r = friendly_random(0x32); // cqc9: the chairman random draw
        if r > p1d() + meta() * 5 {
            ou(0x80);
        } else {
            let r = friendly_random(0x4b);
            if r > p1d() + meta() * 5 {
                write_u8(ln() + board_record::EXPECTATION_FLAGS + 1, read_u8(ln() + board_record::EXPECTATION_FLAGS + 1) | expectation_flags_byte1::CUP_SUCCESS);
            } else {
                let r = friendly_random(100);
                if r > p1d() + meta() * 5 {
                    write_u8(ln() + board_record::EXPECTATION_FLAGS + 1, read_u8(ln() + board_record::EXPECTATION_FLAGS + 1) | 2);
                }
            }
        }
        if cup != 0 && read_u8(cup + comp_stats_club::BEST_POSITION) <= 1 {
            let t = (0x1d - p1d()) / 5;
            let p = read_u8(cup + comp_stats_club::FINAL_POSITION) as i32;
            if p <= t {
                write_u8(ln() + board_record::EXPECTATION_FLAGS + 1, read_u8(ln() + board_record::EXPECTATION_FLAGS + 1) | expectation_flags_byte1::EUROPEAN_SUCCESS);
            } else if p <= t + 4 {
                write_i16(ln() + board_record::EXPECTATION_FLAGS, read_i16(ln() + board_record::EXPECTATION_FLAGS) ^ 0x80);
                write_i16(ln() + board_record::EXPECTATION_FLAGS, read_i16(ln() + board_record::EXPECTATION_FLAGS) ^ 0x200);
                write_u8(ln() + board_record::EXPECTATION_FLAGS + 1, read_u8(ln() + board_record::EXPECTATION_FLAGS + 1) | expectation_flags_byte1::CUP_SUCCESS);
            }
        }
    }
    post_expectations_news(this, club, news);
    finish_board_update(this, club);
}

/// When requested, posts the board expectations news (`post_board_expectations_news`) with the flags word at +0xe of the club's board row.
pub(crate) unsafe fn post_expectations_news(this: usize, club: Club, news: u32) {
    if news as u8 != 0 {
        // simplification: the original pushes the word with the high half of the register (garbage); 0x6a4a20 reads the word
        exe::manager_manager_post_board_expectations_news(this, club, read_i16(board_row(this, club) + board_record::EXPECTATION_FLAGS) as u16 as u32);
    }
}

/// Common tail of `set_season_expectations`: restores confidences, writes the wealth band (+1), adjusts the manager record, clears flags
/// and applies fines on the club's board row.
pub(crate) unsafe fn finish_board_update(this: usize, club: Club) {
    let ln = || board_row(this, club);
    if read_i16(ln() + board_record::CONFIDENCE) < 0x1964 {
        covered!("cqc_9 confidence back");
        for (o, k) in [(board_record::CONFIDENCE, 0x96d288usize), (board_record::SUPPORTERS_CONFIDENCE, 0x9688d0), (board_record::STAFF_CONFIDENCE, 0x9688d0), (board_record::CONFIDENCE_4, 0x9688d0)] {
            let x = read_i16(ln() + o) as i32;
            let v = (x as f64 * read_f64(rdata::F64_5) + read_f64(k)) * read_f64(rdata::F64_0_1); // cqc9: the confidence restored
            write_i16(ln() + o, ftol(v) as i16);
        }
    }
    let r = if manager_manager_is_national_team(club) == 0 {
        let fin = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club);
        if fin != 0 { exe::finance_get_wealth_band(fin) } else { 0 }
    } else {
        0
    };
    write_u8(ln() + board_record::WEALTH_BAND, r);
    let rep = || club.reputation();
    let manager = club.manager().addr();
    if manager != 0 && read_usize(manager + staff::NON_PLAYER) != 0 {
        let fx = read_usize(manager + staff::NON_PLAYER);
        let a = read_i16(fx + non_player::CURRENT_REPUTATION);
        if a < rep() {
            let t = ftol((rep() as i32 - a as i32) as f64 * read_f64(rdata::F64_NEG_0_1));
            write_i16(fx + non_player::CURRENT_REPUTATION, a.wrapping_sub(t as i16));
            let fx = club.manager().non_player();
            if fx.current_reputation() > 0x2710 {
                fx.set_current_reputation(0x2710);
            }
            let fx = club.manager().non_player();
            let c = fx.world_reputation();
            let t = ftol((rep() as i32 - c as i32) as f64 * read_f64(rdata::F64_NEG_0_05));
            fx.set_world_reputation(c.wrapping_sub(t as i16));
            let fx = club.manager().non_player();
            if fx.world_reputation() > 0x2710 {
                fx.set_world_reputation(0x2710);
            }
        }
    }
    let manager = club.manager().addr();
    if manager != 0 && read_usize(manager + staff::NON_PLAYER) != 0 {
        let fx = read_usize(manager + staff::NON_PLAYER);
        let a = read_i16(fx + non_player::CURRENT_REPUTATION);
        if rep() > a {
            let t = ftol((rep() as i32 - a as i32) as f64 * read_f64(rdata::F64_NEG_0_1));
            write_i16(fx + non_player::CURRENT_REPUTATION, a.wrapping_sub(t as i16));
        }
    }
    let manager = club.manager().addr();
    if manager != 0 && read_i32(manager) >= read_i32(G_STAFF_COUNT).wrapping_sub(0x10) && read_i8(manager + staff::LOYALTY) < 0x14 {
        write_u8(manager + staff::LOYALTY, read_u8(manager + staff::LOYALTY).wrapping_add(1));
    }
    write_u8(ln() + board_record::FLAGS, read_u8(ln() + board_record::FLAGS) & !board_record_flags::NEW_MANAGER);
    write_u8(ln() + board_record::REQUESTS_MADE, 0);
    write_u8(ln() + board_record::REQUEST_STATE, read_u8(ln() + board_record::REQUEST_STATE) & (!request_state::LAST_REQUEST as u8));
    write_u8(ln() + board_record::FLAGS, read_u8(ln() + board_record::FLAGS) & 0xdf);
    write_u32(ln() + board_record::EXTRA_FUNDS_GRANTED, 0);
    write_u8(ln() + board_record::WAGE_RISE_GRANTED, 0);
    write_u8(ln() + board_record::REQUEST_STATE, read_u8(ln() + board_record::REQUEST_STATE) & (!request_state::EXTRA_TIME_GRANTED as u8));
    write_u8(ln() + board_record::DISCIPLINE_COUNTERS, read_u8(ln() + board_record::DISCIPLINE_COUNTERS) & !discipline_counters::APPEALS_MADE);
    write_i16(ln() + board_record::LEAGUE_CONFIDENCE, 0);
    write_i16(ln() + board_record::TRANSFER_MOOD, 0);
    write_i16(ln() + board_record::FAVOURITES_MOOD, 0);
    let multas = read_usize(G_FINE);
    if multas != 0 {
        exe::fine_halve_club_fine_count(multas, club);
    }
}

// simplification: appends to the exits list (0x68d0d0, allocates), posts the news (0x6a44a0, 0x6a4a20) and rolls random numbers: does not run
// twice. Original only; the copy carries the branches and is proven by the career and by the mutations.

/// Returns the staff record of index k: [G_STAFF_TABLE] + k × 0x6e.
pub(crate) unsafe fn manager_manager_staff_by_index(k: i32) -> usize {
    read_usize(G_STAFF_TABLE).wrapping_add((k as usize).wrapping_mul(staff::SIZE))
}

/// Returns byte +0xb of the staff state record of index k ([G_STAFF_STATE_TABLE] + k × 0x4f).
pub(crate) unsafe fn staff_state_byte_b(k: i32) -> u8 {
    read_u8(read_usize(G_STAFF_STATE_TABLE).wrapping_add((k as usize).wrapping_mul(staff_state::SIZE)) + staff_state::STATUS_FLAGS)
}

/// Calls the original FN_SCORE_STAFF_FOR_JOB (`exe::score_staff_for_job`) with (person, club, 0, 5): the candidate's job score.
pub(crate) unsafe fn score_candidate(this: usize, p: usize, club: Club) -> i32 {
    exe::score_staff_for_job(this, p, club, 0, 5)
}

/// Calls `is_staff_suitable_for_job` (FN_MANAGER_MANAGER_IS_STAFF_SUITABLE_FOR_JOB) with person, club and the minimum score as a double.
pub(crate) unsafe fn is_staff_suitable(this: usize, p: usize, club: Club, minimum: f64) -> u8 {
    exe::manager_manager_is_staff_suitable_for_job(this, p, club, minimum)
}

/// Calls `contract_record` on G_CONTRACT_MANAGER (FN_CONTRACT_MANAGER_GET_LOAN_CONTRACT): the person's contract record or 0.
pub(crate) unsafe fn loan_contract_of(p: usize) -> usize {
    exe::contract_manager_get_loan_contract(read_usize(G_CONTRACT_MANAGER), p)
}

/// Calls `person_in_list` (FN_MANAGER_MANAGER_IS_VACANCY_CANDIDATE): nonzero if the person is already on a vacancy list.
pub(crate) unsafe fn is_vacancy_candidate(this: usize, p: usize) -> u8 {
    exe::manager_manager_is_vacancy_candidate(this, p)
}

/// O item de 0x26 bytes da vaga (o que a original monta em [esp+0x1c]): +0 achou um livre, +1 0, +2 o id dele, +6 o
/// índice do escolhido, +0xa o id do técnico que saiu, +0xe o id do clube, +0x12 a nota anterior, +0x16 a data da
/// chegada e +0x1e a data do anúncio (8 bytes cada, `exe::date_date_54a110`).
pub(crate) struct ManagerVacancy(pub(crate) [u8; 0x28]);

impl ManagerVacancy {
    pub(crate) fn b(&mut self, o: usize, v: u8) {
        self.0[o] = v;
    }
    pub(crate) fn d(&mut self, o: usize, v: i32) {
        self.0[o..o + 4].copy_from_slice(&v.to_le_bytes());
    }
    pub(crate) fn rd(&self, o: usize) -> i32 {
        i32::from_le_bytes([self.0[o], self.0[o + 1], self.0[o + 2], self.0[o + 3]])
    }
    pub(crate) fn w(&mut self, o: usize, v: u16) {
        self.0[o..o + 2].copy_from_slice(&v.to_le_bytes());
    }
    pub(crate) fn rw(&self, o: usize) -> u16 {
        u16::from_le_bytes([self.0[o], self.0[o + 1]])
    }
}

/// Resets the club board and picks the manager for the vacancy: the best score among the free people who qualify (or
/// the assistant, or the first free one counting backwards), sometimes a better drawn one; the announcement and arrival
/// dates and the item in the vacancy list; the human managers apply.
///
/// SEH frame with no destructor on the normal path. `MANAGER_MANAGER::add_job_vacancy(club, left)`. Club without a
/// division and with reputation below 5000: nothing; B national team whose nation has up to 1000 people
/// (`.number_staff()`), or senior team whose nation has up to 100: nothing. The board starts over (the four confidence
/// values 5000, `board_record::EXPECTATION_FLAGS` = `expectation_flags::AVOID_RELEGATION`, the flags cleared). The
/// chosen one: from all people with a manager record (`staff::NON_PLAYER`; the last 16, the humans, only with
/// `staff::CLASSIFICATION`), except the one who left, without state 0x30 (`staff_state_byte_b`), without a contract
/// (`loan_contract_of`), off the lists (`is_vacancy_candidate`), of the same nation or with
/// `non_player::CURRENT_REPUTATION` ≥ 3750, of the same continent (`nation::REGION`) or with
/// `non_player::WORLD_REPUTATION` ≥ 4250 and the club with reputation ≥ 4750, suited to the job (`is_staff_suitable`
/// with 3.5), excluding players with a contract in force in jobs 11/12/13/15, only national team managers
/// (`staff::JOB_FOR_CLUB` = 5) for a national team: the highest score (`score_candidate`, `exe::score_staff_for_job`).
/// With nobody: the assistant (`.assistant_manager()`), the first of `.coaches(0)` or, from back to front starting at
/// the 17th from last, the first free person in jobs 0, 5, 6, 7, 8, 11, 12, 15 (without wrapping around: the "Can't
/// find a manager" box and nothing). National team or big club (reputation above 4500 + random(2000)): draws as many
/// people as their count and keeps the best free one (with 150 days in the job, 3.15). The human rejected by the club
/// (`exe::staff_add_disliked_club`). The dates: before the game start, 1 to 7 days after it (the announcement) and the
/// arrival 1 to 7 days later; otherwise, the announcement in 1 to 3 days and the arrival in 4 to 8 days (7 to 11 if the
/// one who left is human; 10 to 14 with the offer news `exe::manager_manager_post_job_offer_news` to the chosen one's
/// human club), sometimes 0 to 19 more; the turn of the year. The item goes into list
/// `manager_manager_fields::VACANCY_LIST` (`exe::llist_add_tail`, the cursor `llist::CURSOR` saved); the humans
/// (`exe::human_manager_is_on_holiday`, `exe::human_manager_get_holiday_option`) of smaller clubs apply
/// (`exe::manager_manager_apply_for_job`).
/// Original: `MANAGER_MANAGER::add_job_vacancy` (0x69ba50, manager_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn add_job_vacancy(this: usize, club: usize, left: usize) {
    let club = Club(club);
    // simplification: the previous rating (+0x12) is uninitialized in the original when the chosen one is the assistant or the one at +0x19f (leftover of
    // stack); here 0, a value the game also produces
    let mut v = ManagerVacancy([0; 0x28]);
    exe::date_date_54a110((v.0.as_mut_ptr().add(0x16)) as usize);
    exe::date_date_54a110((v.0.as_mut_ptr().add(0x1e)) as usize);
    let mut best: i32 = -1_000_000;
    if manager_manager_is_national_team(club) == 0 && club.division().addr() == 0 && club.reputation() < 0x1388 {
        return;
    }
    if manager_manager_is_national_team(club) != 0 {
        if exe::club_is_national_team(club) != 0 && club.nation().number_staff() <= 0x3e8 {
            return;
        }
        if exe::club_is_national_team(club) == 0 && club.nation().number_staff() <= 0x64 {
            return;
        }
    }
    v.d(0xe, read_i32(club.addr()));
    v.d(6, -1);
    let ln = || board_row(this, club);
    for o in [board_record::CONFIDENCE, board_record::SUPPORTERS_CONFIDENCE, board_record::STAFF_CONFIDENCE, board_record::CONFIDENCE_4] {
        write_i16(ln() + o, 0x1388);
    }
    write_u8(ln(), 0);
    write_i16(ln() + board_record::EXPECTATION_FLAGS, expectation_flags::AVOID_RELEGATION);
    for o in [0x10usize, board_record::LEAGUE_CONFIDENCE, board_record::TRANSFER_MOOD, board_record::FAVOURITES_MOOD] {
        write_i16(ln() + o, 0);
    }
    for m in [0xfeu8, 0xfd, 0xfb, 0xf7, 0xef] {
        write_u8(ln() + board_record::FLAGS, read_u8(ln() + board_record::FLAGS) & m);
    }
    write_u8(ln() + board_record::REQUESTS_MADE, 0);
    write_u8(ln() + board_record::REQUEST_STATE, read_u8(ln() + board_record::REQUEST_STATE) & (!request_state::LAST_REQUEST as u8));
    for m in [0xdfu8, 0xbf, 0x7f] {
        write_u8(ln() + board_record::FLAGS, read_u8(ln() + board_record::FLAGS) & m);
    }
    write_u32(ln() + board_record::EXTRA_FUNDS_GRANTED, 0);
    write_u8(ln() + board_record::WAGE_RISE_GRANTED, 0);
    write_u8(ln() + board_record::REQUEST_STATE, read_u8(ln() + board_record::REQUEST_STATE) & (!request_state::EXTRA_TIME_GRANTED as u8));
    write_u8(ln() + board_record::DISCIPLINE_COUNTERS, read_u8(ln() + board_record::DISCIPLINE_COUNTERS) & !discipline_counters::APPEALS_MADE);
    v.d(0xa, if left != 0 { read_i32(left) } else { -1 });
    // the best of all
    let mut k = 0i32;
    while k < read_i32(G_STAFF_COUNT) {
        let p = manager_manager_staff_by_index(k);
        let last = read_i32(G_STAFF_COUNT) - 0x10;
        let ok = 'c: {
            if read_usize(p + staff::NON_PLAYER) == 0 || (k >= last && read_u8(p + staff::CLASSIFICATION) == 0) || left == p {
                break 'c false;
            }
            if k >= last && manager_manager_is_national_team(club) != 0 && read_usize(p + staff::NATION) != club.nation().addr()
                && read_i16(read_usize(p + staff::NON_PLAYER) + non_player::WORLD_REPUTATION) < 0xfa0
            {
                break 'c false;
            }
            if k >= read_i32(G_STAFF_COUNT) - 0x10 && exe::club_can_be_managed(club) == 0 {
                break 'c false;
            }
            if staff_state_byte_b(k) & 0x30 != 0 || loan_contract_of(p) != 0 || is_vacancy_candidate(this, p) != 0 {
                break 'c false;
            }
            let (n1, n2) = (club.nation().addr(), read_usize(p + staff::NATION));
            if n1 == 0 || n2 == 0 || (n1 != n2 && read_i16(read_usize(p + staff::NON_PLAYER) + non_player::CURRENT_REPUTATION) < 0xea6) {
                break 'c false;
            }
            if read_u8(n1 + nation::REGION) != read_u8(n2 + nation::REGION)
                && (read_i16(read_usize(p + staff::NON_PLAYER) + non_player::WORLD_REPUTATION) < 0x109a || club.reputation() < 0x128e)
            {
                break 'c false;
            }
            if is_staff_suitable(this, p, club, 3.5) == 0 {
                break 'c false;
            }
            if read_usize(p + staff::PLAYER) != 0 && read_usize(p + staff::CLUB_JOB) != 0 && matches!(read_u8(p + staff::JOB_FOR_CLUB), 0xb | 0xc | 0xd | 0xf) {
                let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), p);
                if c != 0 && (read_u8(c + contract::LEAVING_ON_BOSMAN) != 0 || read_u8(c + contract::MANAGER_JOB_RC) == 0) {
                    break 'c false;
                }
            }
            if manager_manager_is_national_team(club) != 0 && read_u8(p + staff::JOB_FOR_CLUB) != 5 {
                break 'c false;
            }
            true
        };
        if ok {
            let s = score_candidate(this, p, club);
            if s > best {
                covered!("cqc_9 better candidate");
                v.d(6, k); // cqc9: o candidato escolhido
                v.d(0x12, best);
                best = s;
            }
        }
        k += 1;
    }
    if v.rd(6) == -1 {
        let ajudante = club.assistant_manager();
        let list = club.coaches(0);
        if ajudante != 0 {
            v.d(6, read_i32(ajudante));
        } else if list != 0 {
            v.d(6, read_i32(list));
        } else {
            // back to front, from the 17th of the last ones (the original starts from the pointer index of who left)
            let mut first = true;
            let mut i = left as i32;
            let mut start = 0i32;
            loop {
                if first {
                    first = false;
                    i = read_i32(G_STAFF_COUNT) - 0x11;
                    start = i;
                } else {
                    i -= 1;
                    if i < 0 {
                        i = read_i32(G_STAFF_COUNT) - 0x10;
                    }
                    if i == start {
                        let fmt: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
                        fmt(0xde1f64, if exe::club_is_national_team(club) != 0 { 0xa04d8c } else { 0xa04d28 }, club.addr() + 4);
                        return;
                    }
                }
                v.d(6, i);
                if staff_state_byte_b(i) & 0x30 != 0 || i >= read_i32(G_STAFF_COUNT) - 0x10 {
                    v.d(6, -1);
                    continue;
                }
                let p = manager_manager_staff_by_index(i);
                if read_usize(p + staff::NON_PLAYER) == 0 || !matches!(read_u8(p + staff::JOB_FOR_CLUB), 0 | 0xb | 0xc | 5 | 8 | 0xf | 7 | 6) {
                    v.d(6, -1);
                    continue;
                }
                if loan_contract_of(p) != 0 || is_vacancy_candidate(this, manager_manager_staff_by_index(v.rd(6))) != 0 {
                    v.d(6, -1);
                    continue;
                }
                let q = manager_manager_staff_by_index(v.rd(6));
                if read_usize(q + staff::NATIONAL_JOB) != 0 && read_u8(q + staff::JOB_FOR_NATION) == 5 {
                    v.d(6, -1);
                    continue;
                }
                if manager_manager_is_national_team(club) == 0 {
                    let q = manager_manager_staff_by_index(v.rd(6));
                    let c = read_usize(q + staff::CLUB_JOB);
                    if c != 0 {
                        let na = Nation(read_usize(c + club::NATION));
                        let d57 = read_usize(c + club::DIVISION);
                        let strong = (!na.is_null() && na.league_selected() != 0 && d57 != 0
                            && read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add((read_i32(d57) as usize).wrapping_mul(4))) != 0)
                            || read_i16(c + club::REPUTATION) > 0x1964;
                        if strong && read_u8(q + staff::JOB_FOR_CLUB) == 5 {
                            v.d(6, -1);
                            continue;
                        }
                    }
                }
                v.d(0x12, 100);
                if v.rd(6) != -1 {
                    break;
                }
            }
        }
    }
    v.b(1, 0);
    v.b(0, 0);
    v.d(2, -1);
    let grande = manager_manager_is_national_team(club) != 0 || (club.reputation() > 0x1194 && {
        let r = friendly_random(0x7d0);
        club.reputation() as i32 > r + 0x1194
    });
    if grande {
        let mut best: i32 = -1_000_000;
        let mut k = 0i32;
        while k < read_i32(G_STAFF_COUNT) {
            let r = friendly_random(read_i32(G_STAFF_COUNT));
            let p = manager_manager_staff_by_index(r);
            // simplification: the state read is that of counter `k`, not that of the drawn person, as in the original
            if read_usize(p + staff::NON_PLAYER) != 0 && left != p && staff_state_byte_b(k) & 0x30 == 0
                && is_staff_suitable(this, p, club, f64::from_bits(0x4009_3333_3333_3333)) != 0
                && (read_i32(p) < read_i32(G_STAFF_COUNT) - 0x10 || read_u8(p + staff::CLASSIFICATION) != 0)
            {
                if exe::date_days_since(G_CURRENT_DATE, read_usize(p + staff::DATE_JOINED_CLUB) as u32, read_usize(p + staff::DATE_JOINED_CLUB_LEAP_YEAR) as u32) >= 0x96 && is_vacancy_candidate(this, p) == 0 {
                    let s = score_candidate(this, p, club);
                    if s > best {
                        v.b(0, 1);
                        v.d(2, read_i32(p));
                        best = s;
                    }
                }
            }
            k += 1;
        }
    }
    let last = read_i32(G_STAFF_COUNT) - 0x10;
    if read_i32(manager_manager_staff_by_index(v.rd(6))) >= last && left != 0 && read_i32(left) < last {
        exe::staff_add_disliked_club(left, club, 1);
    }
    // the dates: +0x16 the arrival (day +0x16, year +0x18), +0x1e the announcement (day +0x1e, year +0x20)
    let (dia0, ano0) = (read_i16(G_GAME_START_DATE), read_i16(G_GAME_START_DATE_YEAR) as u16);
    let (dia, ano) = (read_i16(G_CURRENT_DATE), read_i16(G_CURRENT_DATE_YEAR) as u16);
    let antes_do_comeco = ano0 > ano || (ano0 == ano && dia0 > dia);
    if antes_do_comeco && exe::club_is_national_team(club) == 0 {
        v.w(0x20, ano0);
        let r = friendly_random(7);
        v.w(0x1e, (r as u16).wrapping_add(dia0 as u16).wrapping_add(1));
        if v.rw(0x1e) as i16 > 0x16c {
            let r = friendly_random(5);
            v.w(0x20, v.rw(0x20).wrapping_add(1));
            v.w(0x1e, r as u16);
        }
        v.w(0x18, v.rw(0x20));
        let r = friendly_random(7);
        v.w(0x16, (r as u16).wrapping_add(v.rw(0x1e)).wrapping_add(1));
        if friendly_random(200) == 0 {
            let r = friendly_random(0x14);
            v.w(0x16, v.rw(0x16).wrapping_add(r as u16));
        }
    } else {
        v.w(0x20, ano);
        let r = friendly_random(3);
        v.w(0x1e, (r as u16).wrapping_add(dia as u16).wrapping_add(1));
        if v.rw(0x1e) as i16 > 0x16c {
            let r = friendly_random(5);
            v.w(0x20, v.rw(0x20).wrapping_add(1));
            v.w(0x1e, r as u16);
        }
        v.w(0x18, v.rw(0x20));
        let q = manager_manager_staff_by_index(v.rd(6));
        let c = read_usize(q + staff::CLUB_JOB);
        if c != 0 && exe::human_manager_is_human_club(c, 0, 1, 0, 0) != 0 && read_u8(manager_manager_staff_by_index(v.rd(6)) + staff::JOB_FOR_CLUB) != 5 {
            exe::manager_manager_post_job_offer_news(this, manager_manager_staff_by_index(v.rd(6)), club);
            let r = friendly_random(5);
            v.w(0x16, (r as u16).wrapping_add(v.rw(0x1e)).wrapping_add(10));
        }
        let r = friendly_random(5);
        let more = if left != 0 && read_i32(left) >= read_i32(G_STAFF_COUNT) - 0x10 { 7 } else { 4 };
        v.w(0x16, (r as u16).wrapping_add(v.rw(0x1e)).wrapping_add(more)); // cqc9: the arrival of the manager
        if friendly_random(500) == 0 {
            let r = friendly_random(0x14);
            v.w(0x16, v.rw(0x16).wrapping_add(r as u16));
        }
    }
    if v.rw(0x16) as i16 > 0x16c {
        let r = friendly_random(5);
        v.w(0x18, v.rw(0x18).wrapping_add(1));
        v.w(0x16, (r as u16).wrapping_add(4));
    }
    let list = read_usize(this + manager_manager_fields::VACANCY_LIST);
    let cursor = read_usize(list + llist::CURSOR);
    exe::llist_add_tail(list, (v.0.as_mut_ptr()) as *mut u8 as usize, 0x26);
    write_u32(read_usize(this + manager_manager_fields::VACANCY_LIST) + llist::CURSOR, cursor as u32);
    if exe::database_get_club_manager(club) == 0 && exe::club_can_be_managed(club) != 0 && manager_manager_is_national_team(club) == 0 {
        let mut i = read_i32(G_STAFF_COUNT) - 0x10;
        while i < read_i32(G_STAFF_COUNT) {
            let p = manager_manager_staff_by_index(i);
            if read_u8(p + staff::CLASSIFICATION) != 0 && exe::human_manager_is_on_holiday(read_usize(G_HUMAN_MANAGER), p) != 0 && exe::human_manager_get_holiday_option(read_usize(G_HUMAN_MANAGER), 1, p) != 0 {
                let c = read_usize(p + staff::CLUB_JOB);
                if c == 0 || read_i16(c + club::REPUTATION) < club.reputation() {
                    exe::manager_manager_apply_for_job(this, club, p);
                }
            }
            i += 1;
        }
    }
}

// simplification: adds the slot to the list [this+8] (0x68d0d0, allocates), draws, posts the offer news and calls the
// applications (0x69ca00): does not run twice. Original only; the copy carries the branches and is proven by the career and by the
// mutations.

/// Posts news item 0x1775 telling the club's manager that another club offered one of his staff the manager job.
///
/// SEH frame with no destructor on the normal path. `MANAGER_MANAGER::post_job_offer_news(person, club)`: with the
/// person employed (`.club_job()`) and the club, from the game start (`G_GAME_START_DATE`) on, news 0x1775
/// (`exe::new_newsitem`) with the person's id, the three names (`names::ID` of `.first_name()`, `.second_name()`,
/// `.common_name()`), the id of the club where they are, that of the offering club and the job (`.job_for_club()`), to
/// the manager of their club (`exe::news_send_to_club_manager`).
/// Original: `MANAGER_MANAGER::post_job_offer_news` (0x6a0540, manager_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn post_job_offer_news(_this: usize, person: usize, club: usize) {
    let person = Staff(person);
    if person.is_null() || person.club_job().is_null() || club == 0 {
        return;
    }
    let day = read_i16(G_GAME_START_DATE);
    let year = read_i16(G_GAME_START_DATE_YEAR) as u16;
    let today = read_i16(G_CURRENT_DATE_YEAR) as u16;
    if year > today || (year == today && day > read_i16(G_CURRENT_DATE)) {
        return;
    }
    covered!("cqc_9 offer news");
    let mut local = [0u32; 0x3a]; // the local news item (0xe8 bytes)
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0x1775, 0);
    exe::newsitem_set_slot(n, 0, read_usize(person.addr()) as u32);
    exe::newsitem_set_slot(n, 1, read_usize(person.first_name() + names::ID) as u32);
    exe::newsitem_set_slot(n, 2, read_usize(person.second_name() + names::ID) as u32);
    exe::newsitem_set_slot(n, 3, read_usize(person.common_name() + names::ID) as u32);
    exe::newsitem_set_slot(n, 4, read_usize(person.club_job().addr()) as u32);
    exe::newsitem_set_slot(n, 5, read_usize(club) as u32);
    exe::newsitem_set_slot(n, 6, (person.job_for_club() as i8) as i32 as u32);
    exe::news_send_to_club_manager(read_usize(G_NEWS), n, person.club_job());
}

// simplification: the news item goes to the manager (allocates): original only; the copy carries the branch.

/// After the match, by a random draw, gathers the league position for the news and the chairman's favoured or rejected
/// players who played badly, with the board confidence rising or falling because of them.
///
/// Called from the board's news after the match (`post_board_match_reaction_news`): `(game, club, ids, fixture, elogio,
/// a6..a10)`. Sets the 3 ids to −1. Club outside the national teams with a chairman (`.chairman()`) with a non-player
/// record (`.non_player()`): random(the record's `.interference()`) against random(10); if it passes, it clears the 5
/// outputs and, in a match of the club's division, a10 = 1 and, from the league table
/// (`exe::comp_stats_find_club_entry`, with more than 8 matches, `.matches_played()`), a9/a8 (the row's
/// `.best_position()`/`.current_position()`), a7 (the board's `board_record::TARGET_POSITION`) and a6 (2 or 0xfe by the
/// row's `.winning_run()`/`.losing_run()`). The players (job 11, with a record) that the chairman rejects
/// (`FN_STAFF_GET_DISLIKED_STAFF`, if `elogio` = 0) or favours (`FN_STAFF_GET_FAVOURED_STAFF`), of the club by contract
/// (`exe::contract_manager_get_contract`, the result kept in `G_CONTRACT_LOOKUP_RESULT`), below the club's reputation
/// (rejected) or up to 35 years old and above it − 750 (favoured), with a rating below 7 in the match
/// (`exe::get_player_match_rating_verdict` = −1): their ids go into `ids` and the board confidence
/// (`board_record::CONFIDENCE`) rises (favoured) or falls (rejected) by 10 × the chairman's `.ambition()`. Returns the
/// sum of the two counts (the caller does not read it).
/// Original: `board_named_players_after_match` (0x6aaee0, manager_manager.cpp, thiscall).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn board_named_players_after_match(this: usize, game: usize, club: usize, ids: *mut i32, fixture: u32, elogio: *const u8,
                                                         a6: *mut u8, a7: *mut u8, a8: *mut u8, a9: *mut u8, a10: *mut u8) -> u8 {
    let game = Fixture(game);
    let club = Club(club);
    if !ids.is_null() {
        for k in 0..3 {
            *ids.add(k) = -1;
        }
    }
    if club.is_null() {
        return 0;
    }
    let t = club.chairman();
    if t.is_null() || t.non_player().is_null() || exe::database_is_national_team(club) != 0 {
        return 0;
    }
    covered!("cqa_10 the board draws");
    let r1 = exe::utils_random(t.non_player().interference() as i32); // cqa10: the random draw by the player sheet of the manager
    let r2 = exe::utils_random(10); // cqa10: the board random draw
    if r1 < r2 {
        return 0;
    }
    *a10 = 0;
    *a6 = 0;
    *a7 = 0;
    *a8 = 0;
    *a9 = 0;
    let comp = game.competition();
    if !comp.is_null() && club.division() == comp {
        *a10 = 1;
        let div = club.division().addr();
        let c = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(div).wrapping_mul(4)));
        if c != 0 && read_usize(c + 8) != 0 {
            let r = CompStatsClub(exe::comp_stats_find_club_entry(read_usize(c + 8), club));
            if !r.is_null() && r.matches_played() > 8 {
                *a9 = r.best_position();
                *a8 = r.current_position();
                *a7 = read_u8(club_board_row(this, club) + board_record::TARGET_POSITION);
                if r.winning_run() == 2 {
                    *a6 = 2;
                } else if r.losing_run() == 2 {
                    *a6 = 0xfe;
                }
            }
        }
    }
    // the original keeps the chosen ones in 3 dwords just before the list: m[0..3] are the chosen ones, m[3..] the list
    let mut m = [0usize; 53];
    let mut n: i8 = 0;
    let (mut da, mut d9) = (0i8, 0i8);
    let preferidos = *elogio != 0;
    if preferidos {
        let f: unsafe extern "C" fn(usize, usize, *mut i8) -> usize = core::mem::transmute(FN_STAFF_GET_FAVOURED_STAFF);
        f(t.addr(), m.as_mut_ptr().add(3) as usize, &mut n);
    } else {
        let f: unsafe extern "C" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_STAFF_GET_DISLIKED_STAFF);
        f(t.addr(), m.as_mut_ptr().add(3) as usize, &mut n as *mut i8 as usize);
    }
    let mut k = 0i8;
    while k < n {
        let p = Staff(m[3 + k as usize]);
        k += 1;
        if p.is_null() || p.player().is_null() || (p.job_for_club() as i8) != 0xb {
            continue;
        }
        let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), p);
        write_usize(G_CONTRACT_LOOKUP_RESULT, c);
        if c == 0 {
            continue;
        }
        let cl = read_usize(G_CLUB_TABLE).wrapping_add(read_usize(c + 4).wrapping_mul(club::SIZE));
        if cl == 0 || cl != club.addr() {
            continue;
        }
        let rating = p.player().current_reputation();
        let rep = club.reputation();
        let passes = if preferidos { (p.age() as i8) < 0x24 && rating as i32 > rep as i32 - 0x2ee } else { rating < rep };
        if passes && exe::get_player_match_rating_verdict(this, p, fixture) == 0xff {
            let i = if preferidos { &mut da } else { &mut d9 };
            m[*i as usize] = p.addr();
            *i += 1;
        }
    }
    if !ids.is_null() {
        for i in 0..da.max(0) as usize {
            *ids.add(i) = read_usize(m[i]) as i32;
        }
        let base = da.max(0) as usize;
        for j in 0..d9.max(0) as usize {
            *ids.add(base + j) = read_usize(m[j]) as i32;
        }
    }
    let x = club.chairman().ambition() as i16;
    let conf = (club_board_row(this, club) + board_record::CONFIDENCE) as *mut i16;
    if da != 0 {
        covered!("cqa_10 board praises");
        core::ptr::write_unaligned(conf, core::ptr::read_unaligned(conf).wrapping_add(x.wrapping_mul(10))); // cqa10: a confiança pelos preferidos
    } else if d9 != 0 {
        covered!("cqa_10 board blames");
        core::ptr::write_unaligned(conf, core::ptr::read_unaligned(conf).wrapping_sub(x.wrapping_mul(10))); // cqa10: a confiança pelos rejeitados
    }
    (d9 as u8).wrapping_add(da as u8)
}

/// Finds the player among the starters of the match record: −1 for a rating under 7, 1 above, otherwise 0.
///
/// SEH frame for the local array. `get_player_match_rating_verdict(person, fixture)`: reads the match record
/// (`exe::match_stats_read_match_record` on `[G_MATCH_STATS]`, with the two 0x625-byte sides constructed and destroyed
/// by the iterators `exe::crt_vector_constructor_iterator`/`exe::crt_vector_destructor_iterator`) and looks for the
/// person among the first 11 of each side (ids at `match_record::HOME_TEAM` + `match_record_team::ROWS`,
/// `match_record_row::SIZE` bytes): a rating (`.match_grade()`) below 7 gives −1, above 7 gives 1, 7 gives 0; not found
/// (or no person), 0. The ecx is irrelevant.
/// Original: `get_player_match_rating_verdict` (0x6ab670, manager_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn get_player_match_rating_verdict(_this: usize, person: usize, fixture: u32) -> u8 {
    let mut record = [0u32; 0x360]; // 0xd7a bytes of the match record
    let r = MatchRecord(record.as_mut_ptr() as usize);
    let sides = r.addr() + match_record::HOME_TEAM;
    exe::crt_vector_constructor_iterator(sides, (0x625) as u32, 2, 0x453c30, 0x453c10);
    let mut v = 0u8;
    if person != 0 && exe::match_stats_read_match_record(read_usize(G_MATCH_STATS), fixture, r) != 0 {
        'busca: for lado in 0..2usize {
            for k in 0..20usize {
                let s = MatchRecordRow(sides + lado * 0x625 + match_record_team::ROWS + k * match_record_row::SIZE);
                let id = read_usize(s.addr()) as i32;
                if id != -1 && person == month_ratings_staff_by_id(id) && k < 11 {
                    let rating = s.match_grade();
                    v = if rating < 7 { 0xff } else if rating > 7 { 1 } else { 0 };
                    break 'busca;
                }
            }
        }
    }
    exe::crt_vector_destructor_iterator(sides, (0x625) as u32, 2, 0x453c10);
    v
}

/// Builds the board sentence praising or blaming the named player, by variant.
///
/// `build_board_player_performance_text(ids, win, out, variant)`, called from the board text (0x6a69c0). The original
/// keeps only the first id that is not −1 (in its own place, 0, 1 or 2), counts 1, and the texts always read place 0.
/// Win: with the first one, the name (`exe::database_format_staff_name`, 200, style 1) in one of the three praises by
/// variant (0, 1, other); then, since no player counts on a defeat, the output stays empty (the original clears the
/// first byte). Defeat: the output empty and, with the first one, one of the four blames by variant (0, 1, 2, other),
/// with 100 and the name. Without ids, nothing.
/// Original: `build_board_player_performance_text` (0x6ab2d0, manager_manager.cpp, stdcall).
pub(crate) unsafe extern "stdcall" fn build_board_player_performance_text(ids: *const i32, win: u32, out: usize, variant: u32) {
    if ids.is_null() {
        return;
    }
    let (i0, i1, i2) = (*ids, *ids.add(1), *ids.add(2));
    if i0 == -1 && (i1 != -1 || i2 != -1) {
        // simplification: the original reads place 0 without value (the stack): that case stays with it
        return original_fn::<unsafe extern "stdcall" fn(*const i32, u32, usize, u32)>(IDX_BUILD_BOARD_PLAYER_PERFORMANCE_TEXT)(ids, win, out, variant);
    }
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let mut buf = [0u8; 0xc8];
    let b = buf.as_mut_ptr() as usize;
    covered!("cqa_10 board text");
    if win as u8 != 0 {
        if i0 != -1 {
            exe::database_format_staff_name(month_ratings_staff_by_id(i0), b, 0xc8, 1);
            let f = match variant {
                0 => 0xa12558usize,
                1 => 0xa125ac,
                _ => 0xa125f8,
            };
            format(out, f, b);
        }
        write_u8(out, 0);
    } else {
        write_u8(out, 0);
        if i0 != -1 {
            exe::database_format_staff_name(month_ratings_staff_by_id(i0), b, 0xc8, 1);
            let f = match variant {
                0 => 0xa12294usize,
                1 => 0xa12310,
                2 => 0xa12384,
                _ => 0xa123ec,
            };
            format(out, f, 0x64u32, b);
        }
    }
}

/// Adds delta to the i16 at offset o (+6, +0x14, +0x16: confidence and moods) of the club's board row.
pub(crate) unsafe fn add_board_confidence(this: usize, club: Club, o: usize, delta: i16) {
    let a = (club_board_row(this, club) + o) as *mut i16;
    core::ptr::write_unaligned(a, core::ptr::read_unaligned(a).wrapping_add(delta));
}

/// Raises board confidence (+6 of the club's board row) to at least 1 and returns the veto flag 0x800.
pub(crate) unsafe fn clamp_confidence_floor(this: usize, club: Club) -> u32 {
    let a = (club_board_row(this, club) + board_record::CONFIDENCE) as *mut i16;
    if core::ptr::read_unaligned(a) < 1 {
        core::ptr::write_unaligned(a, 1);
    }
    0x800
}

/// Calls CONTRACT_MANAGER::calc_player_value (`calc_player_value`) on G_CONTRACT_MANAGER with (player, club, 0, -1, mode).
pub(crate) unsafe fn player_value(j: Staff, club: usize, mode: u32) -> i32 {
    exe::contract_manager_calc_player_value(read_usize(G_CONTRACT_MANAGER), j, club, 0, -1, mode)
}

/// Looks up the player's contract (`exe::contract_manager_get_contract`), stores it in G_CONTRACT_LOOKUP_RESULT and returns the contract's club (id
/// at +4) or 0.
pub(crate) unsafe fn contract_club_cached(j: Staff) -> usize {
    let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), j);
    write_usize(G_CONTRACT_LOOKUP_RESULT, c);
    if c == 0 { 0 } else { (friendly_club_by_id(read_usize(c + 4))).addr() }
}

/// Whether the copy seeded the CRT `rand` from the clock (the physiotherapists were assessed).
///
/// If so, the comparer does not check the `rand` state (the clock moves between the original routine and the copy in
/// the real game).
pub(crate) static BOARD_VETO_SEEDED_RAND: AtomicBool = AtomicBool::new(false);

/// Checks whether the board vetoes a transfer: when buying an injured player, the physios draw the veto; at a human
/// club, the board vetoes on budget, asking price or fee and loses confidence.
///
/// `MANAGER_MANAGER::check_board_transfer_veto(j, club, value, comprando)`, called from `check_board_veto` (offer
/// without a swap). When buying a player with an injury (`exe::injury_get_player_injury`) of severity `.days_out()` >
/// 20, each of the club's three physios (`.physios(k)`, with a record `.non_player()`) seeds the CRT rand with the two
/// ids and draws: retirement announced because of the injury (`exe::player_regen_find_retirement_entry`) against his
/// rating (`FN_NON_PLAYER_ATTRIBUTE_RATING` of byte `.physiotherapy()`), the severity against random(200) + 25 and (41
/// − rating) × 5, the relapse (`exe::injury_get_recurring_injury_level`), rating³ + 100 (pow `game_pow`) and the rating
/// against the player's (`.player().injury_proneness()`): any of them vetoes (0x2000); at the end the rand goes back to
/// the clock (`exe::at_946ebb`). Then, only with a contract outside types 1 and 3 (`contract::CONTRACT_TYPE`), a
/// current club (`.club_job()`) and a human club (`exe::human_manager_is_human_club`): the limit (reputation
/// `.reputation()` × 0.2)² between 150 thousand and 2 million. Selling: a league with a division and a squad below 15,
/// the board loses 1250/1250/500 (+6/+0x14/+0x16, `add_board_confidence`, floor 1 on +6, `clamp_confidence_floor`) and
/// vetoes 0x200; otherwise the asking price (`FN_CONTRACT_MANAGER_CALC_ASKING_PRICE`, at most the value for the
/// contract's club) above the limit and the value for the current club not below `value`: asking > 2 × value with the
/// contract's `contract::HAPPINESS` > 25, or asking > 5 million with `contract::HAPPINESS` > 0 and > 2.5 × value, lose
/// 1250/1250/500; > 6 × value, 500/500/250; veto 0x800. Buying: an amount above 1.2 × the budget
/// (`exe::finance_get_transfer_budget`) vetoes 0x800; above the limit, the lowest value (for the club and for the
/// contract's club) + 0.65 × limit + the wealth band (`exe::finance_get_wealth_band`: 1.25 million, 650 thousand or 65
/// thousand), truncated, below 0.275 × value loses 250/500/500 and vetoes 0x800. Otherwise 0.
/// Original: `check_board_transfer_veto` (0x696b10, manager_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn check_board_transfer_veto(this: usize, j: usize, club: usize, value: i32, comprando: u32) -> u32 {
    let j = Staff(j);
    let club = Club(club);
    let rating: unsafe extern "C" fn(usize, u32) -> i8 = core::mem::transmute(FN_NON_PLAYER_ATTRIBUTE_RATING);
    BOARD_VETO_SEEDED_RAND.store(false, Relaxed);
    if comprando as u8 != 0 {
        let l = InjuryEntry(exe::injury_get_player_injury(read_usize(G_INJURY), j));
        if !l.is_null() && l.days_out() > 0x14 {
            let mut semeou = false;
            for k in 0..3usize {
                let f = Staff(club.physios(k));
                if f.is_null() || f.non_player().is_null() {
                    continue;
                }
                exe::crt_srand((read_usize(j.addr()) as u32).wrapping_add(1).wrapping_mul((read_usize(f.addr()) as u32).wrapping_add(1)));
                semeou = true;
                BOARD_VETO_SEEDED_RAND.store(true, Relaxed);
                covered!("cpy_12 physio evaluates");
                let np = f.non_player();
                // the original pushes the whole register with byte +0x1e in the low part; 0x544410 only reads the byte
                let n = || rating(np.addr(), np.physiotherapy() as u32) as i32;
                if exe::player_regen_find_retirement_entry(read_usize(G_PLAYER_REGEN), j, 1) != 0 {
                    if exe::utils_random(n()) != 0 {
                        return 0x2000;
                    }
                    if exe::player_regen_find_retirement_entry(read_usize(G_PLAYER_REGEN), j, 0) != 0 {
                        return 0x2000;
                    }
                }
                let r = exe::utils_random(200); // cpy12: the severity random draw
                if l.days_out() as i32 > r.wrapping_add(0x19) {
                    let a = exe::utils_random((0x29 - n()) * 5);
                    let b = exe::utils_random(l.days_out() as i32);
                    if a < b {
                        return 0x2000;
                    }
                }
                if exe::injury_get_recurring_injury_level(read_usize(G_INJURY), j) != 0 {
                    let c = n();
                    let rr = exe::injury_get_recurring_injury_level(read_usize(G_INJURY), j) as i8 as i32;
                    let a = exe::utils_random(c + rr);
                    let b = exe::utils_random(10);
                    if a > b {
                        return 0x2000;
                    }
                }
                let c = n();
                if exe::utils_random(ftol(game_pow(c as f64, read_f64(rdata::F64_3)) + read_f64(rdata::F64_100))) == 0 {
                    return 0x2000;
                }
                let a = exe::utils_random(n());
                let b = exe::utils_random(200);
                if a > b {
                    let a = exe::utils_random((j.player().injury_proneness() as u8) as i8 as i32);
                    let b = exe::utils_random(200);
                    if a > b {
                        return 0x2000;
                    }
                }
            }
            if semeou {
                let mut t = 0i32;
                exe::crt_srand(exe::at_946ebb(&mut t) as u32);
            }
        }
    }
    let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), j);
    if c == 0 {
        return 0;
    }
    let kind = read_u8(c + contract::CONTRACT_TYPE) & 0x3f;
    if kind == 1 || kind == 3 || j.club_job().is_null() || exe::human_manager_is_human_club(club, 1, 1, (0) as u32, 0) == 0 {
        return 0;
    }
    let x = club.reputation() as f64 * read_f64(rdata::F64_0_2);
    let mut limit = ftol(x * x);
    if limit < 0x249f0 {
        limit = 0x249f0;
    } else if limit > 0x1e8480 {
        limit = 0x1e8480;
    }
    if comprando as u8 == 0 {
        let nation = club.nation();
        let division = club.division().addr();
        if !nation.is_null() && nation.league_selected() != 0 && division != 0 && read_usize(division) != read_usize(KEY_CLUB_COMP_A_LOWER_DIVISION) && (exe::database_count_club_squad(club, 0, 0) as i8) < 0xf {
            add_board_confidence(this, club, 6, -0x4e2);
            add_board_confidence(this, club, 0x14, -0x4e2);
            add_board_confidence(this, club, 0x16, -500);
            clamp_confidence_floor(this, club);
            return 0x200;
        }
        let price: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, u32) -> i32 = core::mem::transmute(FN_CONTRACT_MANAGER_CALC_ASKING_PRICE);
        let mut request = price(read_usize(G_CONTRACT_MANAGER), j.club_job().addr(), club.addr(), j.addr(), 0, 0, 1);
        let by_contract = player_value(j, contract_club_cached(j), 1);
        if request > by_contract {
            request = by_contract;
        }
        let by_current = player_value(j, j.club_job().addr(), 0);
        if request <= limit || by_current < value {
            return 0;
        }
        let (p, v) = (request as f64, value as f64);
        let more = read_u8(c + contract::HAPPINESS) as i8;
        covered!("cpy_12 board selling");
        if p > v + v && more > 0x19 {
            add_board_confidence(this, club, 6, -0x4e2);
            add_board_confidence(this, club, 0x14, -0x4e2);
        } else if request > 0x4c4b40 && more > 0 && p > v * read_f64(rdata::F64_2_5) {
            add_board_confidence(this, club, 6, -0x4e2);
            add_board_confidence(this, club, 0x14, -0x4e2);
        } else if p > v * read_f64(rdata::F64_6) {
            add_board_confidence(this, club, 6, -500);
            add_board_confidence(this, club, 0x14, -500);
            add_board_confidence(this, club, 0x16, -0xfa);
            return clamp_confidence_floor(this, club);
        } else {
            return 0;
        }
        add_board_confidence(this, club, 0x16, -500);
        return clamp_confidence_floor(this, club);
    }
    let budget = exe::finance_get_transfer_budget(exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club), j.addr() as u32, 0, 0, 1, 0);
    if value as i64 > budget.wrapping_mul(12) / 10 { // cpy12: the board budget
        covered!("cpy_12 board without funds");
        return 0x800;
    }
    if value <= limit {
        return 0;
    }
    let by_club = player_value(j, club.addr(), 0);
    let by_contract = player_value(j, contract_club_cached(j), 1);
    let smaller = if by_club > by_contract { by_contract } else { by_club };
    let fin = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club);
    if fin == 0 {
        return 0;
    }
    let k = match exe::finance_get_wealth_band(fin) {
        2 => read_f64(rdata::F64_1250000),
        1 => read_f64(rdata::F64_650000),
        _ => read_f64(rdata::F64_65000),
    };
    let ceiling = ftol(limit as f64 * read_f64(rdata::F64_0_65) + smaller as f64 + k);
    if !(value as f64 * read_f64(rdata::F64_0_275) > ceiling as f64) {
        return 0;
    }
    covered!("cpy_12 board finds it expensive");
    add_board_confidence(this, club, 6, -0xfa);
    add_board_confidence(this, club, 0x14, -500);
    add_board_confidence(this, club, 0x16, -500);
    clamp_confidence_floor(this, club)
}

/// Posts the 0x178d news of the ultimatum before the match, to the club's manager or, by a random draw, to those of the
/// division or the country.
///
/// SEH frame for the local news item. `MANAGER_MANAGER::post_board_ultimatum_news(club, opponent, comp)`, called from
/// the board ultimatum (`issue_board_ultimatum`). Club with a division (`.division()`), a manager (`.manager()`) and
/// the competition: news 0x178d (`exe::new_newsitem`) with the manager (the id and the three keys, `names::ID` of
/// `.first_name()`, `.second_name()`, `.common_name()`), the club, the opponent, the competition and a random(5); a
/// club with reputation (`.reputation()`) below 5750, to the club's manager (`exe::news_send_to_club_manager`);
/// otherwise, by random(3), to the division's managers (`exe::news_send_to_division_managers`) or to the country's
/// (`.nation()`, `exe::news_send_to_nation_managers`).
/// Original: `post_board_ultimatum_news` (0x6ac700, manager_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn post_board_ultimatum_news(_this: usize, club: usize, opponent: usize, comp: usize) {
    let club = Club(club);
    if club.is_null() || club.division().addr() == 0 || club.manager().addr() == 0 || comp == 0 {
        return;
    }
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    exe::new_newsitem(nt, 0x178d, 0);
    let t = club.manager();
    exe::newsitem_set_slot(nt, 0, read_usize(t.addr()) as u32);
    exe::newsitem_set_slot(nt, 1, read_usize(t.first_name() + names::ID) as u32);
    exe::newsitem_set_slot(nt, 2, read_usize(t.second_name() + names::ID) as u32);
    exe::newsitem_set_slot(nt, 3, read_usize(t.common_name() + names::ID) as u32);
    exe::newsitem_set_slot(nt, 4, read_usize(club.addr()) as u32);
    exe::newsitem_set_slot(nt, 5, read_usize(opponent) as u32);
    exe::newsitem_set_slot(nt, 6, read_usize(comp) as u32);
    covered!("cpy_12 ultimatum news");
    exe::newsitem_set_slot(nt, 7, exe::utils_random(5) as u32); // cpy12: the ultimatum random draw
    if club.reputation() < 0x1676 {
        exe::news_send_to_club_manager(read_usize(G_NEWS), nt, club);
    } else if exe::utils_random(3) != 0 {
        exe::news_send_to_division_managers(read_usize(G_NEWS), nt, club.division().addr());
    } else {
        exe::news_send_to_nation_managers(read_usize(G_NEWS), nt, club.nation().addr());
    }
}

// simplification: the news goes to the managers’ inboxes (allocates) and rolls: original only; the copy takes the branch.

/// Writes the headline or body of the ultimatum news, with the names and links of the manager and the clubs.
///
/// `news_board_ultimatum_text(news, body, out)`: reads fields 0 to 7 of the news (`exe::newsitem_get_slot`: the
/// manager, the three keys, the club, the opponent, the competition); the manager's name
/// (`exe::database_format_staff_name`, 500, style 1). Body: text 0xa12dac in 0xde1f64 (`FN_LANGLIB_SPRINTF`) with the
/// genders (the clubs' `.gender_name()`, the competition's `.short_gender_name()`), the names (the clubs' +4 /
/// `club::NAME`, the competition's `competition::SHORT_NAME`) and the manager's, into `out` (`exe::text_set_string`),
/// and the links (`exe::text_add_link`): the manager (0x7da, if the names match via
/// `exe::database_staff_matches_names`), the club and the opponent (0x7d5). Headline: 0xa12d64 with the club's gender
/// (`.short_gender_name()`) and short name (`club::SHORT_NAME`).
/// Original: `news_board_ultimatum_text` (0x6ac880, manager_manager.cpp, stdcall).
pub(crate) unsafe extern "stdcall" fn news_board_ultimatum_text(news: usize, body: u32, out: usize) {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    covered!("cpy_12 ultimatum text");
    let v: [u32; 7] = core::array::from_fn(|k| exe::newsitem_get_slot(news, (k as i8) as u32));
    exe::newsitem_get_slot(news, (7) as u32);
    let person = read_usize(G_STAFF_TABLE).wrapping_add((v[0] as usize).wrapping_mul(staff::SIZE));
    let club = friendly_club_by_id(v[4] as usize);
    let opponent = friendly_club_by_id(v[5] as usize);
    let comp = Competition(read_usize(G_CLUB_COMP_TABLE).wrapping_add((v[6] as usize).wrapping_mul(0x6b)));
    let mut name = [0u8; 0x200];
    let buf = name.as_mut_ptr() as usize;
    exe::database_format_staff_name(person, buf, 0x1f4, 1);
    if body != 0 {
        let gc = comp.short_gender_name() as i8 as i32;
        let ga = (opponent.gender_name() as u8) as i8 as i32;
        let gl = (club.gender_name() as u8) as i8 as i32;
        format(0xde1f64, 0xa12dac, gl, gl, ga, ga, gc, gc, club.addr() + 4, buf, opponent.addr() + club::NAME, comp.addr() + competition::SHORT_NAME);
        exe::text_set_string(out, (0xde1f64) as usize as *const u8);
        if exe::database_staff_matches_names(person, v[1], v[2], v[3], (0) as usize) != 0 {
            exe::text_add_link(out, (buf) as usize as *const u8, 0x7da, person);
        }
        exe::text_add_link(out, (club.addr() + 4) as usize as *const u8, 0x7d5, club);
        exe::text_add_link(out, (opponent.addr() + club::NAME) as usize as *const u8, 0x7d5, opponent);
    } else {
        let g = club.short_gender_name() as i8 as i32;
        format(0xde1f64, 0xa12d64, g, g, club.addr() + club::SHORT_NAME); // "... manager given ultimatum by board"
        exe::text_set_string(out, (0xde1f64) as usize as *const u8);
    }
}

/// True while the manager-news start date (G_GAME_START_DATE: day +0, year +2) is still after today (G_CURRENT_DATE),
/// so no news is posted.
pub(crate) unsafe fn before_news_start_date() -> bool {
    let (dia, ano) = (read_i16(G_GAME_START_DATE), read_i16(G_GAME_START_DATE_YEAR) as u16);
    let today = read_i16(G_CURRENT_DATE_YEAR) as u16;
    ano > today || (ano == today && dia > read_i16(G_CURRENT_DATE))
}

/// Returns the club's row in the board table: [this] + club id × 0x49 (duplicate of board_row).
pub(crate) unsafe fn board_row_of(this: usize, club: Club) -> usize {
    read_usize(this).wrapping_add(read_i32(club.addr()).wrapping_mul(0x49) as usize)
}

/// Writes value v into data slot k of a news item (FN_NEWSITEM_SET_SLOT).
pub(crate) unsafe fn news_set_slot(n: usize, k: u32, v: u32) {
    exe::newsitem_set_slot(n, (k) as u32 as i8, v);
}

/// Writes the person's id and the three keys at +0x33 of its +4, +8 and +0xc pointers into news data slots k..k+3.
pub(crate) unsafe fn news_set_person(n: usize, k: u32, p: Staff) {
    news_set_slot(n, k, read_usize(p.addr()) as u32);
    news_set_slot(n, k + 1, read_usize(p.first_name() + names::ID) as u32);
    news_set_slot(n, k + 2, read_usize(p.second_name() + names::ID) as u32);
    news_set_slot(n, k + 3, read_usize(p.common_name() + names::ID) as u32);
}

/// True if the person is one of the 16 human managers (id ≥ [0xae2364] − 16).
pub(crate) unsafe fn is_human_manager(p: Staff) -> bool {
    read_i32(p.addr()) >= read_i32(G_STAFF_COUNT).wrapping_sub(0x10)
}

/// Writes the human manager's two name strings (+4, +8) into a news item from slot `first` (`set_slot_string`) and stores the
/// next free slot in slot k_next.
pub(crate) unsafe fn news_set_manager_names(n: usize, p: Staff, first: u8, k_next: u32) {
    // simplification: the original stores the next datum in the low byte of a local that held the date [0xdd6ec8] and pushes the
    // whole dword as a field; 0x7858c0 reads only the low byte, and the copy passes the clean value
    let mut next = first;
    exe::newsitem_set_slot_string(n, first as u32, p.first_name(), &mut next);
    news_set_slot(n, k_next, next as i8 as i32 as u32);
    exe::newsitem_set_slot_string(n, next as u32, p.second_name(), &mut next);
}

/// The comparer's rehearsal: the copy runs without delivering the news item (deliveries copy the news item into the
/// human managers' inboxes, allocate and may trigger the automatic reply) and keeps the reach it would have (+0xde: 4
/// everyone, 2 those of the country, 1 those of the division; 0 none) and the news item built.
pub(crate) static MANAGER_NEWS_DRY_RUN: AtomicBool = AtomicBool::new(false);
pub(crate) static MANAGER_NEWS_REACH: AtomicU8 = AtomicU8::new(0);
pub(crate) static mut MANAGER_NEWS_BUILT: [u8; 0xe0] = [0; 0xe0];

/// Delivers news item n by reach (4 all managers, 2 nation managers, else division managers) through G_NEWS; in
/// rehearsal mode only records the plan.
pub(crate) unsafe fn deliver_news(n: usize, reach: u8, target: usize) {
    if MANAGER_NEWS_DRY_RUN.load(Relaxed) {
        MANAGER_NEWS_REACH.store(reach, Relaxed);
        core::ptr::copy_nonoverlapping(n as *const u8, core::ptr::addr_of_mut!(MANAGER_NEWS_BUILT) as *mut u8, 0xe0);
        return;
    }
    let news = read_usize(G_NEWS);
    match reach {
        4 => {
            let f: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_ALL_MANAGERS);
            f(news, n);
        }
        2 => {
            let f: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_NATION_MANAGERS);
            f(news, n, target);
        }
        _ => {
            let f: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_DIVISION_MANAGERS);
            f(news, n, target);
        }
    }
}

/// Delivers a manager news item by the club: only manageable clubs; national team or reputation above 7250 to all,
/// otherwise to the club's nation managers.
pub(crate) unsafe fn deliver_news_for_club(n: usize, club: Club) {
    if exe::club_can_be_managed(club) == 0 {
        return;
    }
    if exe::database_is_national_team(club) != 0 || club.reputation() > 0x1c52 {
        deliver_news(n, 4, 0);
    } else {
        let nation = club.nation().addr();
        if nation != 0 {
            deliver_news(n, 2, nation);
        }
    }
}

/// The comparer of the three manager-news routines.
///
/// The copy runs as a dry run (without delivering) and the original routine from the same state (the `regioes` and the
/// random draw). Must be equal: the `regioes`; the copy's random draws are the beginning of the original's (the
/// automatic reply of the delivery may draw afterwards); and each human manager whose inbox (News `+8`: 12 bytes per
/// human, start of the ring at `+0`, next slot at `+4`) advanced received the copy's news: type (`+0`), data and date
/// (`+5`..`+0xd6`) and the scope (`+0xde`); with no planned delivery, no inbox advances.
pub(crate) unsafe fn compare_manager_news(name: &str, regions: Vec<(usize, usize)>, original: impl FnOnce(), copy: impl FnOnce()) {
    let news = read_usize(G_NEWS);
    let caixas = if news != 0 { read_usize(news + 8) } else { 0 };
    let antes_caixas: Vec<(i32, i32)> = if caixas != 0 { (0..0x10).map(|k| (read_i32(caixas + 12 * k), read_i32(caixas + 12 * k + 4))).collect() } else { Vec::new() };
    let mut all = regions.clone();
    all.extend(crate::rng::STATE_REGIONS);
    let before = crate::comparison::MemorySnapshot::read(&all);
    MANAGER_NEWS_DRY_RUN.store(true, Relaxed);
    MANAGER_NEWS_REACH.store(0, Relaxed);
    let (_, rr) = crate::rng::trace(|| crate::rng::without_counting(copy));
    MANAGER_NEWS_DRY_RUN.store(false, Relaxed);
    let mr = crate::comparison::MemorySnapshot::read(&regions);
    let (alcance, noticia) = (MANAGER_NEWS_REACH.load(Relaxed), *core::ptr::addr_of!(MANAGER_NEWS_BUILT));
    before.write_back();
    let (_, ro) = crate::rng::trace(original);
    let mo = crate::comparison::MemorySnapshot::read(&regions);
    let mut entregues = 0;
    let mut news_same = true;
    let caixas = if news != 0 { read_usize(news + 8) } else { 0 };
    if caixas != 0 && antes_caixas.len() == 0x10 {
        for (k, &(inicio, vaga)) in antes_caixas.iter().enumerate() {
            let e = caixas + 12 * k;
            if read_i32(e + 4) == vaga && read_i32(e) == inicio {
                continue;
            }
            entregues += 1;
            let v = if inicio == -1 { read_i32(e) } else { vaga }; // the first news item opens the ring at the start
            let s = read_usize(news).wrapping_add((v as usize).wrapping_mul(0xdf));
            let g = core::slice::from_raw_parts(s as *const u8, 0xdf);
            if g[0..4] != noticia[0..4] || g[5..0xd6] != noticia[5..0xd6] || g[0xde] != alcance {
                news_same = false;
            }
        }
    }
    let same = mo == mr && ro.len() >= rr.len() && ro[..rr.len()] == rr[..] && news_same && (alcance != 0 || entregues == 0);
    SCOREBOARD.record(same, || format!("{} original {:x?} sorteios {:?} rust {:x?} sorteios {:?}; reach {} entregues {} news equal {}", name, mo, ro, mr, rr, alcance, entregues, news_same));
}

/// Posts the 0x1777 news with the manager, the club and the chairman; the reason by the board ratings below confidence
/// − 500 (random draws of 3 and 5) and the rumour (random draw of 10, or 1 when the manager knows the club well);
/// delivered to all, to the nation or to the division's managers; today's date into `board_record::LAST_REVIEW_DATE` of
/// the board row.
///
/// `MANAGER_MANAGER::post_manager_under_pressure_news(club, kind)`, called when considering sacking the manager
/// (`consider_sacking_manager`: kind 1 when swapping for the best candidate, 2 after 14 days). Club without a manager
/// (`.manager()`) or before the date `[G_GAME_START_DATE]` (`before_news_start_date`): nothing. News 0x1777
/// (`exe::new_newsitem`): the manager (0..3), the club (4), the chairman (`.chairman()`: 5..7 and the id in 10; without
/// one, −1). The reason (8) from the board row (`.confidence()`): rating `.supporters_confidence()`,
/// `.staff_confidence()` or `.confidence_4()` below confidence − 500 with random(3) zero gives 2, 1 or 3 (+1 with
/// random(5) non-zero); otherwise 0. The kind (9) and the rumour (11): random(10), or 1 if the manager knows the club
/// well (`exe::non_player_attribute_rating` of the non-player `.non_player()` with `.motivating()` above 10). Delivery
/// (`deliver_news`): human manager, to all; club with reputation above 5000, to the country's; otherwise to those of
/// the division (`.division()`) unless it is `[KEY_CLUB_COMP_A_LOWER_DIVISION]`. Finally today's date into the row's
/// `board_record::LAST_REVIEW_DATE` (`exe::date_operator_3d`).
/// Original: `MANAGER_MANAGER::post_manager_under_pressure_news` (0x6a0e70, manager_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn post_manager_under_pressure_news(this: usize, club: usize, kind: u32) {
    let club = Club(club);
    if club.is_null() {
        return;
    }
    let manager = club.manager();
    if manager.is_null() || before_news_start_date() {
        return;
    }
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0x1777, 0);
    news_set_person(n, 0, manager);
    news_set_slot(n, 4, read_usize(club.addr()) as u32);
    let presence = club.chairman();
    if !presence.is_null() {
        news_set_slot(n, 5, read_usize(presence.first_name() + names::ID) as u32);
        news_set_slot(n, 6, read_usize(presence.second_name() + names::ID) as u32);
        news_set_slot(n, 7, read_usize(presence.common_name() + names::ID) as u32);
        news_set_slot(n, 0xa, read_usize(presence.addr()) as u32);
    } else {
        for k in [5, 6, 7, 0xa] {
            news_set_slot(n, k, u32::MAX);
        }
    }
    let l = BoardRecord(board_row_of(this, club));
    let reason: i8 = if (l.supporters_confidence() as i32) < l.confidence() as i32 - 500 /* cpx10: o limit pela avaliação +8 */ && {
        covered!("cpx_10 reason by evaluation");
        friendly_random(3) == 0
    } {
        2
    } else if (l.staff_confidence() as i32) < l.confidence() as i32 - 500 && friendly_random(3) == 0 {
        1
    } else if (l.confidence_4() as i32) < l.confidence() as i32 - 500 && friendly_random(3) == 0 {
        3 + (friendly_random(5) != 0) as i8
    } else {
        0
    };
    news_set_slot(n, 8, reason as i32 as u32);
    news_set_slot(n, 9, kind as u8 as i8 as i32 as u32);
    covered!("cpx_10 rumour draw");
    let mut boato = friendly_random(10); // cpx10: the rumour random draw
    let nj = manager.non_player();
    if !nj.is_null() {
        if exe::non_player_attribute_rating(nj, nj.motivating() as i8) as u8 as i8 > 10 {
            boato = 1;
        }
    }
    news_set_slot(n, 0xb, boato as u32);
    if is_human_manager(manager) {
        deliver_news(n, 4, 0);
    } else if club.reputation() > 0x1388 {
        deliver_news(n, 2, club.nation().addr());
    } else {
        let div = club.division().addr();
        if div != 0 && read_i32(div) != read_i32(KEY_CLUB_COMP_A_LOWER_DIVISION) {
            deliver_news(n, 1, div);
        }
    }
    let mut data = [0u32; 2];
    covered!("cpx_10 pressure date");
    exe::date_operator_3d(board_row_of(this, club) + board_record::LAST_REVIEW_DATE, data.as_mut_ptr() as usize, G_CURRENT_DATE); // cpx10: the news date in the row
}

/// Posts the 6000 news with the manager, the kind, the club, the job, the chairman and the reason; a human manager with
/// the names and to all; otherwise by the club (national team or reputation above 7250 to all, otherwise to the
/// nation).
///
/// `MANAGER_MANAGER::post_manager_departure_news(manager, kind, reason, club, job)`, called when removing the manager
/// from the job (`remove_club_manager`). Null manager or club, before the date `[G_GAME_START_DATE]`
/// (`before_news_start_date`) or kind 5: nothing. News 6000 (`exe::new_newsitem`): the manager (0..3), the kind (4),
/// the club (5), the job (7), the chairman's keys (`.chairman()`: 8..10; without one, −1) and the reason (11). Human
/// manager: his names from data slot 13 on (the next free one in 12, `news_set_manager_names`) and to all; otherwise by
/// the club (`deliver_news_for_club`, `club_can_be_managed`; national team or reputation above 7250 to all, otherwise
/// to the country's).
/// Original: `MANAGER_MANAGER::post_manager_departure_news` (0x6a1b00, manager_manager.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn post_manager_departure_news(this: usize, manager: usize, kind: u32, reason: u32, club: usize, job: u32) {
    let club = Club(club);
    let _ = this;
    if manager == 0 || club.is_null() || before_news_start_date() {
        return;
    }
    let t = kind as u8 as i8;
    if t == 5 {
        return;
    }
    let mut local = [0u32; 0x38];
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0x1770, 0);
    news_set_person(n, 0, Staff(manager));
    news_set_slot(n, 4, t as i32 as u32);
    news_set_slot(n, 5, read_usize(club.addr()) as u32);
    news_set_slot(n, 7, job as u8 as i8 as i32 as u32);
    let presence = club.chairman().addr();
    for (k, d) in [(8, 4), (9, 8), (0xa, 0xc)] {
        news_set_slot(n, k, if presence != 0 { read_usize(read_usize(presence + d) + names::ID) as u32 } else { u32::MAX });
    }
    news_set_slot(n, 0xb, reason as u8 as i8 as i32 as u32);
    covered!("cpx_10 departure news");
    if is_human_manager(Staff(manager)) {
        news_set_manager_names(n, Staff(manager), 0xd, 0xc);
        deliver_news(n, 4, 0);
    } else {
        deliver_news_for_club(n, club);
    }
}

/// Posts the 0x1787 news of a manager appointment, only for a club of an active nation with an active division, or a
/// national team of an active nation: the manager, the previous club, the club, the job and the previous role;
/// delivered like the departure news.
///
/// `MANAGER_MANAGER::post_manager_appointment_news(manager, previous, club, job)`, called from the manager market's day
/// (0x68f9c0) and from the appointment (0x69ebc0). Null manager or club, before the date `[G_GAME_START_DATE]`
/// (`before_news_start_date`) or job 5: nothing. Only a club of an active nation (the nation's `.league_selected()`)
/// with its division in the array `[G_COMP_OBJECT_TABLE]`, or a national team (`exe::database_is_national_team`) of an
/// active nation. News 0x1787 (`exe::new_newsitem`): the manager (0..3), the previous club (4; −1), the club (5), the
/// job (6) and the previous role (7: `.job_for_nation()` for a national team, `.job_for_club()` for a club; without a
/// previous one, −1). Human manager: the names from data slot 8 on (the next free one in 7, `news_set_manager_names`)
/// and to all; otherwise by the club (`deliver_news_for_club`), like the departure news.
/// Original: `MANAGER_MANAGER::post_manager_appointment_news` (0x6a2e00, manager_manager.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn post_manager_appointment_news(this: usize, manager: usize, previous: usize, club: usize, job: u32) {
    let manager = Staff(manager);
    let club = Club(club);
    let _ = this;
    if manager.is_null() || club.is_null() || before_news_start_date() {
        return;
    }
    let c = job as u8 as i8;
    if c == 5 {
        return;
    }
    let nation = club.nation();
    let active = !nation.is_null() && nation.league_selected() != 0 && {
        let div = club.division().addr();
        div != 0 && read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_i32(div).wrapping_mul(4) as usize)) != 0
    };
    // the original reads +0x11c of the national team’s nation without checking the nation (every national team has one)
    if !active && !(exe::database_is_national_team(club) != 0 && club.nation().league_selected() != 0) {
        return;
    }
    let mut local = [0u32; 0x38];
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0x1787, 0);
    news_set_person(n, 0, manager);
    news_set_slot(n, 4, if previous != 0 { read_usize(previous) as u32 } else { u32::MAX });
    news_set_slot(n, 5, read_usize(club.addr()) as u32);
    news_set_slot(n, 6, c as i32 as u32);
    let function = if previous == 0 {
        u32::MAX
    } else if exe::database_is_national_team(previous) != 0 {
        manager.job_for_nation() as i8 as i32 as u32
    } else {
        manager.job_for_club() as i8 as i32 as u32
    };
    news_set_slot(n, 7, function);
    covered!("cpx_10 new manager news");
    if is_human_manager(manager) {
        news_set_manager_names(n, manager, 8, 7);
        deliver_news(n, 4, 0);
    } else {
        deliver_news_for_club(n, club);
    }
}

/// Copies date `origin` into `destination` via the Date assignment operator (`exe::date_operator_3d`).
pub(crate) unsafe fn manager_manager_copy_date(target: usize, source: usize) {
    let mut t = [0u8; 8];
    exe::date_operator_3d(target, (t.as_mut_ptr()) as usize, source);
}

/// Days between today (0xae2c90) and date d (`exe::date_get_difference`): days (+0) plus 365 × years (+2).
pub(crate) unsafe fn days_since(d: usize) -> i32 {
    let mut r = [0u8; 8];
    exe::date_get_difference(G_CURRENT_DATE, r.as_mut_ptr(), read_usize(d) as u32, read_usize(d + 4) as u32);
    let p = r.as_ptr() as usize;
    (read_u16(p + 2) as i32).wrapping_mul(365).wrapping_add(read_i16(p) as i32)
}

/// The comparer's dry run: the copy runs without the end of the injury (`end_player_injury`: history in file and news item), which
/// cannot run twice; on reaching it, it sets `END_INJURY_REACHED` and the comparer keeps the original alone.
pub(crate) static END_INJURY_DRY_RUN: AtomicBool = AtomicBool::new(false);
pub(crate) static END_INJURY_REACHED: AtomicBool = AtomicBool::new(false);

/// Ends the person's injury (FN_INJURY_END_PLAYER_INJURY with 1); in rehearsal mode only records that it would.
pub(crate) unsafe fn end_injury(this: usize, person: Staff) {
    if END_INJURY_DRY_RUN.load(Relaxed) {
        END_INJURY_REACHED.store(true, Relaxed);
        return;
    }
    exe::injury_end_player_injury(this, person, 1);
}

/// One-time init of a function-local static Date guarded by a bit of G_FITNESS_UPDATE_STATIC_INIT_FLAGS: Date()
/// (`exe::date_date_54a110`) and its destructor registered with atexit.
pub(crate) unsafe fn init_static_date(bit: u8, data: usize, destrutor: usize) {
    let g = G_FITNESS_UPDATE_STATIC_INIT_FLAGS as *mut u8;
    if *g & bit == 0 {
        *g |= bit;
        exe::date_date_54a110(data);
        exe::crt_atexit(destrutor);
    }
}

/// Scales the part of x above base by the exe double constant at k, truncates to i16 and adds base back.
pub(crate) unsafe fn scale_above_base(x: i32, base: i32, k: usize) -> i32 {
    (ftol(x.wrapping_sub(base) as f64 * read_f64(k)) as i16 as i32).wrapping_add(base)
}

/// Returns the board's league confidence (`board_record::LEAGUE_CONFIDENCE` of the club's 0x49-byte record) in bands 0
/// to 6 (−2000, −1000, −500, 500, 1000, 2000).
///
/// `MANAGER_MANAGER::get_league_confidence_band(club)`: the board's confidence in the league (signed word
/// `board_record::LEAGUE_CONFIDENCE` of the club's 0x49-byte record in `[this]`) in bands: below −2000: 0; −1000: 1;
/// −500: 2; 500: 3; 1000: 4; 2000: 5; otherwise 6. Used by the confidence text (0x690d80). Only `al` counts (the rest
/// of eax is what was left from the address computation).
/// Original: `MANAGER_MANAGER::get_league_confidence_band` (0x690bd0, manager_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn get_league_confidence_band(this: usize, club: usize) -> u8 {
    let v = read_i16(read_usize(this).wrapping_add((read_i32(club) as u32).wrapping_mul(0x49) as usize) + board_record::LEAGUE_CONFIDENCE);
    if v < -2000 {
        0
    } else if v < -1000 {
        1
    } else if v < -500 {
        2
    } else if v < 500 {
        3
    } else if v < 1000 {
        4
    } else if v < 2000 {
        5
    } else {
        6
    }
}

/// Sets the four board ratings (+6, +8, +0xc, +0xa of the club's 0x49-byte record) to the value.
///
/// `MANAGER_MANAGER::set_board_ratings(club, value)`: the four board ratings (+6, +8, +0xc and +0xa of the club's
/// 0x49-byte record in `[this]`, words, in that order) receive the value (16 bits). Called from the players' initial
/// setup (0x7a46a6).
/// Original: `MANAGER_MANAGER::set_board_ratings` (0x69f7c0, manager_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn set_board_ratings(this: usize, club: usize, value: u32) {
    for off in [6usize, 8, 0xc, 0xa] {
        let r = read_usize(this).wrapping_add((read_i32(club) as u32).wrapping_mul(0x49) as usize);
        write_u16(r + off, value as u16);
    }
}

/// True if the transfer offer status (+0x2c) is 2, 3, 4, 0x10 or 0x14, or 0xf with +0x2e ≥ 3.
pub(crate) unsafe fn is_offer_resolved(o: Offer) -> bool {
    match o.status() {
        2 | 3 | 4 | 0x10 | 0x14 => true,
        0xf => o.response_round() >= 3,
        _ => false,
    }
}

/// True while the news start date (G_GAME_START_DATE) is still after today (G_CURRENT_DATE) (duplicate of
/// before_news_start_date).
pub(crate) unsafe fn news_date_not_reached() -> bool {
    let day = read_i16(G_GAME_START_DATE);
    let year = read_u16(G_GAME_START_DATE_YEAR);
    let today_year = read_u16(G_CURRENT_DATE_YEAR);
    year > today_year || (year == today_year && day > read_i16(G_CURRENT_DATE))
}

/// Posts the 0x1772 board confidence news with the manager's days in the job.
///
/// SEH frame with no destructor. `MANAGER_MANAGER::post_board_confidence_update_news(club)`: before the news date
/// (`news_date_not_reached`), nothing. News 0x1772 (local, `exe::new_newsitem`): 0 = the club's id, 1 = the manager's
/// (`.manager()`) days in the job (since date `.date_joined_club()`/`.date_joined_club_year()`, 365 days per year) or
/// 100 without a manager; through the reach (`exe::news_send_by_scope`) with the club.
/// Original: `post_board_confidence_update_news` (0x6a4c40, manager_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn post_board_confidence_update_news(_this: usize, club: usize) {
    let club = Club(club);
    if news_date_not_reached() {
        return;
    }
    let mut local = [0u32; 0x3a]; // the local news item (0xe8 bytes)
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0x1772, 0);
    exe::newsitem_set_slot(n, 0, read_usize(club.addr()) as u32);
    let manager = club.manager();
    let days = if !manager.is_null() {
        let years = (read_u16(G_CURRENT_DATE_YEAR) as i32).wrapping_sub(manager.date_joined_club_year() as i32);
        years.wrapping_mul(365).wrapping_sub((manager.date_joined_club() as i16) as i32).wrapping_add(read_i16(G_CURRENT_DATE) as i32)
    } else {
        100
    };
    exe::newsitem_set_slot(n, 1, days as u32);
    exe::news_send_by_scope(read_usize(G_NEWS), n, club, (0) as usize, (0) as usize, (0) as usize, 0);
}

/// Posts the 0x1771 board expectations news to the club's manager.
///
/// SEH frame with no destructor. `MANAGER_MANAGER::post_board_expectations_news(club, expectations)`: before the news
/// date (`news_date_not_reached`), nothing. News 0x1771 (local, `exe::new_newsitem`): 0 = the club's id, 1 = the
/// expectations (signed word); to the club's manager (`exe::news_send_to_club_manager`).
/// Original: `post_board_expectations_news` (0x6a4a20, manager_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn post_board_expectations_news(_this: usize, club: usize, expectations: u32) {
    if news_date_not_reached() {
        return;
    }
    covered!("cpv_2 expectations news");
    let mut local = [0u32; 0x3a]; // the local news item (0xe8 bytes)
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0x1771, 0);
    exe::newsitem_set_slot(n, 0, read_usize(club) as u32);
    exe::newsitem_set_slot(n, 1, expectations as i16 as i32 as u32);
    exe::news_send_to_club_manager(read_usize(G_NEWS), n, club);
}

// simplification: reschedules, touches the lists and the reputation, may free the swap list and calls the clubs’ response and the
// next offer (0x8c2f50): does not run twice. Original only; the copy carries the branches and is proven by the career and
// by the mutations.

// simplification: the news goes by reach (allocates): original only. Hot (40 thousand calls): no counter.

// simplification: the news goes to the manager’s inbox (allocates): original only; the copy takes the branch.

/// Returns the contract's club (id at +4), or 0; with filter, 0 for contract type 5 (+0x35 & 0x3f) (duplicate of
/// offer_contract_club).
pub(crate) unsafe fn offer_contract_club(c: Contract, filter: bool) -> usize {
    if !c.is_null() && (!filter || c.contract_type() & 0x3f != 5) {
        (friendly_club_by_id(c.club())).addr()
    } else {
        0
    }
}

/// Puts a transfer offer on hold: status 1, flag bits at +0x30, removes the buyer's contract offer (`remove_contract_offer`), clears
/// terms and the per-player offers of the swap list.
pub(crate) unsafe fn put_offer_on_hold(o: Offer) {
    let mark = (o.completion_timing() & 0xf1) | 1;
    o.set_status(offer_status::ON_HOLD);
    o.set_completion_timing(mark);
    exe::contract_manager_remove_contract_offer(read_usize(G_CONTRACT_MANAGER), staff_records_staff_by_id(o.target_staff_id()), friendly_club_by_id(o.buyer_club_id()));
    o.set_terms_state(terms_state::NONE);
    let l = o.exchange_players();
    if !l.is_null() {
        l.set_cursor(l.head());
        loop {
            let y = exe::llist_get_next(o.exchange_players());
            if y == 0 {
                break;
            }
            exe::contract_manager_remove_contract_offer_for_transfer(read_usize(G_CONTRACT_MANAGER), staff_records_staff_by_id(read_usize(y)), o.slot_index());
            write_u8(y + 4, 0);
        }
    }
}

/// Posts the 0x177e news of the FA's expectation for the national team in a competition.
///
/// SEH frame with no destructor. `MANAGER_MANAGER::post_fa_expectations_news(national_team, expectation, competition)`:
/// news 0x177e (local, `exe::new_newsitem`): 0 = the national team's id, 1 = the expectation (signed word), 2 = the
/// competition; through the reach (`exe::news_send_by_scope`) with the national team.
/// Original: `post_fa_expectations_news` (0x6a44a0, manager_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn post_fa_expectations_news(_this: usize, national_team: usize, expectativa: u32, competition: u32) {
    covered!("cpv_3 federation news");
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0x177e, 0);
    exe::newsitem_set_slot(n, 0, read_usize(national_team) as u32);
    exe::newsitem_set_slot(n, 1, expectativa as i16 as i32 as u32);
    exe::newsitem_set_slot(n, 2, competition);
    exe::news_send_by_scope(read_usize(G_NEWS), n, national_team, (0) as usize, (0) as usize, (0) as usize, 0);
}

/// Posts the 0x1782 news of the supporters protesting against the manager.
///
/// SEH frame with no destructor. `MANAGER_MANAGER::post_supporter_protest_news(club)`: before the news date
/// (`[G_GAME_START_DATE]`), nothing. News 0x1782 (local, `exe::new_newsitem`): 0 = the club's id, 1..4 the manager
/// (`.manager()`: the id and the three keys `names::ID` of `.first_name()`, `.second_name()`, `.common_name()`);
/// through the reach (`exe::news_send_by_scope`).
/// Original: `post_supporter_protest_news` (0x6a5d10, manager_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn post_supporter_protest_news(_this: usize, club: usize) {
    let club = Club(club);
    let day = read_i16(G_GAME_START_DATE);
    let year = read_u16(G_GAME_START_DATE_YEAR);
    let today_year = read_u16(G_CURRENT_DATE_YEAR);
    if year > today_year || (year == today_year && day > read_i16(G_CURRENT_DATE)) {
        return;
    }
    covered!("cpv_3 protest news");
    let mut local = [0u32; 0x3a]; // the local news item (0xe8 bytes)
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0x1782, 0);
    exe::newsitem_set_slot(n, 0, read_usize(club.addr()) as u32);
    exe::newsitem_set_slot(n, 1, read_usize(club.manager().addr()) as u32);
    exe::newsitem_set_slot(n, 2, read_usize(club.manager().first_name() + names::ID) as u32);
    exe::newsitem_set_slot(n, 3, read_usize(club.manager().second_name() + names::ID) as u32);
    exe::newsitem_set_slot(n, 4, read_usize(club.manager().common_name() + names::ID) as u32);
    exe::news_send_by_scope(read_usize(G_NEWS), n, club, (0) as usize, (0) as usize, (0) as usize, 0);
}

// simplification: completes, withdraws, rejects and reschedules offers (the calls free and allocate): does not run twice. Only the
// original; the copy takes the branches and is proven by the career and the mutations.

// simplification: the news item goes by reach (allocates): original only; the copy takes the branch.

// simplification: the news item goes by reach (allocates): original only; the copy takes the branch.

/// Calls `database_get_club_manager` (FN_DATABASE_GET_CLUB_MANAGER): the club's manager or national coach.
pub(crate) unsafe fn club_manager(club: usize) -> usize {
    exe::database_get_club_manager(club)
}

/// Calls FN_HUMAN_MANAGER_IS_HUMAN_CLUB (`exe::human_manager_is_human_club`) with (club, 0, any, person, 0).
pub(crate) unsafe fn manager_manager_is_human_club(club: usize, any: u8, person: usize) -> u32 {
    exe::human_manager_is_human_club(club, 0, (any) as u32, (person) as u32, 0)
}

/// Writes one board confidence/mood i16: random(n1) + h as float, minus random(500), truncated.
pub(crate) unsafe fn roll_confidence(c: usize, n1: i32, h: f32) {
    let r1 = friendly_random(n1);
    let s = (r1 as f64 + h as f64) as f32;
    let r2 = friendly_random(0x1f4);
    write_i16(c, ftol(s as f64 - r2 as f64) as i16);
}

/// Clamps an i16 confidence: below 5500 becomes 5500 + random(250), above 10000 becomes 10000.
pub(crate) unsafe fn clamp_confidence_5500(c: usize) {
    if read_i16(c) < 0x157c {
        write_i16(c, (friendly_random(0xfa) + 0x157c) as i16);
    } else if read_i16(c) > 0x2710 {
        write_i16(c, 0x2710);
    }
}

/// Clears the board-row fields both departure paths reset (+0x10..+0x16, bits of +2..+5, +0x28 and +0x40).
pub(crate) unsafe fn clear_board_flags(l: usize) {
    let l = BoardRecord(l);
    for k in [0x10usize, board_record::LEAGUE_CONFIDENCE, board_record::TRANSFER_MOOD, board_record::FAVOURITES_MOOD] {
        write_i16(l.addr() + k, 0);
    }
    l.set_flags(l.flags() & 0xe0);
    l.set_requests_made(0);
    l.set_request_state(l.request_state() & (!request_state::LAST_REQUEST as u8));
    l.set_flags(l.flags() & 0x1f);
    l.set_extra_funds_granted(0);
    l.set_wage_rise_granted(0);
    l.set_request_state(l.request_state() & (!request_state::EXTRA_TIME_GRANTED as u8));
    l.set_discipline_counters(l.discipline_counters() & !discipline_counters::APPEALS_MADE);
    l.set_discipline_counters(l.discipline_counters() & !discipline_counters::FINE_LEVEL);
}

/// Fixes the national team jobs of staff (manager at the team's `club::MANAGER`) and builds each club's board row:
/// dates, reputation squared, the four confidence values drawn from the expectation (board rating or reputation) and
/// the reputation, held to 5750/5500..10000, or 5000 without a manager; the season expectations
/// (`exe::manager_manager_set_season_expectations`) and, without a manager, the vacancy
/// (`exe::manager_manager_add_job_vacancy`).
///
/// `MANAGER_MANAGER::init_club_board_records()`: at the start of the career (from the constructor 0x68dbf0). (1) Each
/// person with a national team job (`.national_job()`, the team's club record): function (`.job_for_nation()`) 5 or 12
/// becomes that team's manager (`club::MANAGER`); 11 stays; 13 and 15 become 11 (`job_kind::PLAYER`, with a player
/// record `.player()`) or 8 (`job_kind::COACH`); the others lose the national team job; without a club (`.club_job()`)
/// and without a club function (`.job_for_club()` = `job_kind::UNEMPLOYED`), the national function goes to
/// `.job_for_club()`; outside 11 the national function is cleared (`job_kind::UNEMPLOYED`). (2) Each club (national
/// teams only with a nation and `nation::CONTINENT`): the second national team has its reputation (`club::REPUTATION`)
/// capped at 200 × the nation's (`nation::REPUTATION`) and the slots cleared; its row (73 bytes in `[this]`) gets the
/// dates (23/07/1971 at `board_record::LAST_ULTIMATUM_DATE`, today at `board_record::LAST_REVIEW_DATE`), reputation² ×
/// 0.25 (double at `board_record::SPENDING`) and zeros. With a manager: the date 23/08/1971 at
/// `board_record::JOB_APPLICATION_DATE` (by the id's row), the expectation `f` (the reputation, the board rating
/// `non_player::CURRENT_REPUTATION` or 4500, which drops to 0.75 × reputation), `h` = f / (reputation × 0.5 + 2500) ×
/// 7500 and the four confidence values (`board_record::CONFIDENCE`, `board_record::SUPPORTERS_CONFIDENCE`,
/// `board_record::STAFF_CONFIDENCE`, `board_record::CONFIDENCE_4`) by random draws (`roll_confidence`), clamped below
/// (5750, or 5500 with a draw) and above (10000; `.confidence()` above 9500 becomes 10000 − random(500)); clears the
/// flags (`clear_board_flags`) and sets the season expectations (`exe::manager_manager_set_season_expectations` with
/// 1). Without a manager (outside the partner club, `exe::club_get_linked_b_team`): confidence 5000,
/// `.set_expectation_flags(expectation_flags::AVOID_RELEGATION)`, flags cleared; a manageable club
/// (`exe::club_can_be_managed`, or one with a competition and a nation with `nation::LEAGUE_SELECTED` & 3) gets the
/// expectations and the vacancy (`exe::manager_manager_add_job_vacancy`).
/// Original: `MANAGER_MANAGER::init_club_board_records` (0x68f130, manager_manager.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn init_club_board_records(this: usize) {
    // (1) the people
    let mut k = 0i32;
    while k < read_usize(G_STAFF_COUNT) as i32 {
        let p = Staff(read_usize(G_STAFF_TABLE).wrapping_add(k as usize * staff::SIZE));
        k += 1;
        let club = p.national_job();
        if club.is_null() {
            continue;
        }
        let f = p.job_for_nation();
        match f {
            5 | 12 => {
                write_u32(club.addr() + club::MANAGER, p.addr() as u32);
                continue;
            }
            11 => continue,
            _ => {}
        }
        let mut tira = true;
        if f == 0xf || f == 0xd {
            if !p.player().is_null() {
                p.set_job_for_nation(job_kind::PLAYER);
                tira = false;
            } else {
                p.set_job_for_nation(job_kind::COACH);
            }
        }
        if tira {
            p.set_national_job(Club((0) as usize));
        }
        if p.club_job().is_null() && p.job_for_club() == job_kind::UNEMPLOYED {
            p.set_job_for_club(p.job_for_nation());
        }
        if p.job_for_nation() != job_kind::PLAYER {
            p.set_job_for_nation(job_kind::UNEMPLOYED);
        }
    }
    // (2) the clubs
    let mut partner = 0u32; // the original keeps it in a single variable, from club to club
    let mut i = 0i32;
    let mut ofs = 0usize;
    while i < read_usize(G_CLUB_COUNT) as i32 {
        let club = read_usize(G_CLUB_TABLE).wrapping_add(i as usize * club::SIZE);
        let l = BoardRecord(read_usize(this).wrapping_add(ofs));
        i += 1;
        ofs += 0x49;
        if exe::database_is_national_team(club) != 0 {
            let n = read_usize(club + club::NATION);
            if n == 0 || read_usize(n + nation::CONTINENT) == 0 {
                continue;
            }
        }
        if exe::club_is_national_team(club) != 0 {
            let n = read_usize(club + club::NATION);
            if n != 0 && read_i16(club + club::REPUTATION) as i32 > read_i16(n + nation::REPUTATION) as i32 * 200 {
                write_i16(club + club::REPUTATION, read_i16(n + nation::REPUTATION).wrapping_mul(200));
            }
            write_u32(club + club::MANAGER, 0);
            write_u32(club + club::CHAIRMAN, 0);
            write_u32(club + club::ASSISTANT_MANAGER, 0);
            core::ptr::write_bytes((club + club::DIRECTORS) as *mut u8, 0, 12);
            core::ptr::write_bytes((club + club::COACHES) as *mut u8, 0, 0x14 + 0x1c + 12); // +0x19f..+0x1db
        }
        let (mut d, mut t) = ([0u8; 8], [0u8; 8]);
        let r = exe::date_date_549f70((d.as_mut_ptr() as usize) as *mut [u16; 4], 0x17, 6, 0x7b3, 0xffff_ffff);
        exe::date_operator_3d(l.addr() + board_record::LAST_ULTIMATUM_DATE, t.as_mut_ptr() as usize, r);
        l.set_expectation_flags(0);
        l.set_target_position(0);
        exe::date_operator_3d(l.addr() + board_record::LAST_REVIEW_DATE, t.as_mut_ptr() as usize, G_CURRENT_DATE);
        write_u8(l.addr(), 0);
        let rep = read_i16(club + club::REPUTATION) as i32;
        l.set_spending(rep.wrapping_mul(rep) as f64 * 0.25);
        write_u32(l.addr() + board_record::SQUAD_INVESTMENT, 0);
        write_u32(l.addr() + board_record::SQUAD_INVESTMENT + 4, 0);
        if club_manager(club) == 0 {
            // without a manager
            if exe::club_get_linked_b_team(club, &mut partner, 1) != 0 && partner != 0 {
                continue;
            }
            covered!("cpr_5 board without manager");
            for c in [board_record::CONFIDENCE, board_record::SUPPORTERS_CONFIDENCE, board_record::STAFF_CONFIDENCE, board_record::CONFIDENCE_4] {
                write_i16(l.addr() + c, 0x1388);
            }
            l.set_expectation_flags(expectation_flags::AVOID_RELEGATION);
            clear_board_flags(l.addr());
            if exe::club_can_be_managed(club) == 0 {
                if exe::database_is_national_team(club) != 0 {
                    continue;
                }
                let objs = read_usize(G_COMP_OBJECT_TABLE);
                let comp = read_usize(club + club::DIVISION);
                let n = read_usize(club + club::NATION);
                if objs == 0 || comp == 0 || n == 0 || read_usize(objs + read_usize(comp) * 4) == 0 || read_u8(n + nation::LEAGUE_SELECTED) & 3 == 0 {
                    continue;
                }
            }
            exe::manager_manager_set_season_expectations(this, club, 1);
            exe::manager_manager_add_job_vacancy(this, club, 0);
            continue;
        }
        // with a manager
        let r = exe::date_date_549f70((d.as_mut_ptr() as usize) as *mut [u16; 4], 0x17, 7, 0x7b3, 0xffff_ffff);
        exe::date_operator_3d(read_usize(this).wrapping_add(read_usize(club).wrapping_mul(0x49)) + board_record::JOB_APPLICATION_DATE, t.as_mut_ptr() as usize, r);
        let mut f: f32;
        let mut direct = false; // the 4500 skips the zero and 500 test
        if read_usize(club_manager(club) + staff::NON_PLAYER) == 0 && manager_manager_is_human_club(club, 1, 0) == 0 {
            f = rep as f32;
        } else if manager_manager_is_human_club(club, 1, 0) != 0 {
            if read_usize(club + club::MANAGER) != 0 && read_usize(club_manager(club) + staff::NON_PLAYER) != 0 {
                f = read_i16(read_usize(club_manager(club) + staff::NON_PLAYER) + non_player::CURRENT_REPUTATION) as f32;
            } else {
                f = 4500.0;
                direct = true;
            }
        } else {
            f = read_i16(read_usize(club_manager(club) + staff::NON_PLAYER) + non_player::CURRENT_REPUTATION) as f32;
            let ceiling = rep as f64 * 0.75;
            if (read_i16(read_usize(club_manager(club) + staff::NON_PLAYER) + non_player::CURRENT_REPUTATION) as f64) < ceiling {
                let v = ftol(ceiling) as i16;
                write_i16(read_usize(club_manager(club) + staff::NON_PLAYER) + non_player::CURRENT_REPUTATION, v); // the board’s rating drops to 0.75 × reputation
            }
        }
        if !direct {
            if f == 0.0 {
                let r80 = read_i16(club + club::REPUTATION);
                f = if r80 != 0 { (r80 as f64 * 0.75) as f32 } else { 5000.0 };
            } else if f < 500.0 {
                f = (friendly_random(0x3e8) + 0x1f4) as f32;
            }
        }
        let mut g = rep as f64 * 0.5 + 2500.0;
        if g == 0.0 {
            g = f as f64 * 1.5;
        } else if g < 500.0 {
            g = (friendly_random(0x2ee) + 0xfa) as f64;
        }
        covered!("cpr_5 board with manager");
        let h = ((f as f64 / g) * 7500.0) as f32;
        roll_confidence(l.addr() + board_record::CONFIDENCE, 0x1f4, h);
        let c = l.confidence();
        if c < 0x1676 {
            covered!("cpr_5 low confidence");
            l.set_confidence((friendly_random(0x2ee) + 0x1676) as i16);
        } else if c > 0x251c {
            l.set_confidence((0x2710 - friendly_random(0x1f4)) as i16);
        }
        roll_confidence(l.addr() + board_record::SUPPORTERS_CONFIDENCE, 0x1f4, h);
        if l.supporters_confidence() < 0x1676 {
            l.set_supporters_confidence((friendly_random(0x1f4) + 0x1676) as i16);
        } else if l.supporters_confidence() > 0x2710 {
            l.set_supporters_confidence(0x2710);
        }
        roll_confidence(l.addr() + board_record::STAFF_CONFIDENCE, 0x6d6, h);
        clamp_confidence_5500(l.addr() + board_record::STAFF_CONFIDENCE);
        roll_confidence(l.addr() + board_record::CONFIDENCE_4, 0x9c4, h);
        clamp_confidence_5500(l.addr() + board_record::CONFIDENCE_4);
        clear_board_flags(l.addr());
        exe::manager_manager_set_season_expectations(this, club, 1);
    }
}

// simplification: draws, sets the expectations (0x6a3770) and opens slots (0x69ba50, allocates): original only; the copy carries the
// branches (runs once, at the start of the career).

/// Processes the ban appeals: for each appeal (`manager_manager_fields::BAN_APPEAL_LIST`) whose date has come, the
/// result is applied on up to 4 discipline rule sets (`exe::discipline_apply_appeal_result`), the news is sent
/// (`exe::manager_manager_post_ban_appeal_result_news`) and the appeal leaves the list.
///
/// `MANAGER_MANAGER::process_ban_appeals()`: walks the appeals list (`manager_manager_fields::BAN_APPEAL_LIST`, the
/// count a signed word re-read on each iteration) and, for each one whose date (`ban_appeal::DATE`) is no longer after
/// today (`exe::date_operator_3e`), with a result (`.result()`): the up to 4 rules (`ban_appeal::RULES`, ≥ 0) receive
/// the result in the discipline (`exe::discipline_apply_appeal_result` with fields `.revoke_by_length()`,
/// `.ban_matches()`, `.ban_days()`, `.matches_cut()`, `.days_cut()` and +0x18); then the result news
/// (`exe::manager_manager_post_ban_appeal_result_news`) and the appeal leaves (`exe::llist_remove`). Null item: the
/// "Error" box (0x57f5) and it stops. Called from the board's daily pass (0x68f9c0); only the human's appeals enter the
/// list.
/// Original: `MANAGER_MANAGER::process_ban_appeals` (0x6abef0, manager_manager.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn process_ban_appeals(this: usize) {
    let list = || read_usize(this + manager_manager_fields::BAN_APPEAL_LIST);
    write_u32(list() + llist::CURSOR, read_usize(list() + 2) as u32);
    let mut k = 0i32;
    while k < read_i16(list()) as i32 {
        let item = BanAppeal(exe::llist_get_next(list()));
        if item.is_null() {
            return show_error_at(0x9f18a4, 0x57f5);
        }
        if exe::date_operator_3e(item.addr() + ban_appeal::DATE, read_usize(G_CURRENT_DATE) as u32, read_usize(G_CURRENT_DATE_LEAP) as u32) == 0 {
            covered!("cpr_5 appeal decided");
            let p = read_usize(G_STAFF_TABLE).wrapping_add(read_usize(item.addr()).wrapping_mul(staff::SIZE));
            if item.result() != 0 {
                for j in 0..4usize {
                    let rule = read_u8(item.addr() + ban_appeal::RULES + j) as i8;
                    if rule > -1 {
                        // simplification: the original pushes the fields in registers with garbage on top; 0x557000 reads only
                        // the byte or the word of each.
                        exe::discipline_apply_appeal_result(read_usize(G_DISCIPLINE), read_usize(p) as u32, rule, item.revoke_by_length(), item.ban_matches(),
                                item.ban_days(), item.matches_cut(), item.days_cut(), read_u8(item.addr() + 0x18));
                    }
                }
            }
            exe::manager_manager_post_ban_appeal_result_news(this, p, item.result(), item.matches_cut(), item.days_cut(), read_u8(item.addr() + 0x18));
            exe::llist_remove(list(), item);
        }
        k += 1;
    }
}

// simplification: applies the discipline, releases the news and frees the appeal: original only; the copy takes the branch.

/// Returns the club's board row (73 bytes) in the MANAGER_MANAGER table: [this] + club id × 0x49 (duplicate of
/// board_row).
pub(crate) unsafe fn club_board_row(this: usize, club: Club) -> usize {
    read_usize(this).wrapping_add(read_usize(club.addr()).wrapping_mul(0x49))
}

/// Returns the non-player record (+0x69) of the club's chairman (club::CHAIRMAN), or 0.
pub(crate) unsafe fn chairman_record(club: Club) -> usize {
    let p = club.chairman();
    if p.is_null() {
        0
    } else {
        p.non_player().addr()
    }
}

/// Days from date (d0, d1), passed by value, to the current game date (`exe::date_days_since`).
pub(crate) unsafe fn days_until_today(d0: usize, d1: usize) -> i32 {
    exe::date_days_since(G_CURRENT_DATE, (d0) as u32, (d1) as u32)
}

/// Appends an item of n bytes to the end of a linked list (FN_LLIST_ADD_TAIL).
pub(crate) unsafe fn list_add_tail(list: usize, item: usize, n: u32) {
    exe::llist_add_tail(list, item, n);
}

/// Appends a manager departure (type, reason from `get_departure_reason`, club id) to the departures list [this+4] and sets bit 2 at
/// +2 of the club's board row.
pub(crate) unsafe fn record_departure(this: usize, club: Club, kind: u8) {
    let id = read_usize(club.addr()) as u32;
    let mut reg = [0u8; 8];
    reg[0] = kind;
    reg[2..6].copy_from_slice(&id.to_le_bytes());
    reg[1] = exe::manager_manager_get_departure_reason(this, id);
    list_add_tail(read_usize(this + manager_manager_fields::DEPARTURE_LIST), reg.as_mut_ptr() as usize, 6);
    *((club_board_row(this, club) + board_record::FLAGS) as *mut u8) |= board_record_flags::DEPARTURE_QUEUED;
}

/// Compares the current manager's score with the best candidate's; if it exceeds by 450, cuts confidence to 90% (min 1)
/// and after 30 days posts the pressure news. Returns whether it did.
pub(crate) unsafe fn check_better_candidate(this: usize, club: Club) -> bool {
    let manager = club.manager();
    let rating = if !manager.is_null() && !manager.non_player().is_null() { exe::score_staff_for_job(this, manager, club, 0, 5) } else { 0 };
    let m = exe::manager_manager_best_manager_candidate_score(this, club, club.manager().addr());
    if rating <= m.wrapping_add(0x1c2) {
        return false;
    }
    let c = club_board_row(this, club) + board_record::CONFIDENCE;
    write_i16(c, ftol(read_i16(c) as i32 as f64 * read_f64(rdata::F64_0_9)) as i16); // 0,9
    if read_i16(c) < 1 {
        write_i16(c, 1);
    }
    let l = BoardRecord(club_board_row(this, club));
    if days_until_today(l.last_review_date(), l.last_review_date_leap_year()) > 0x1e {
        exe::manager_manager_post_manager_under_pressure_news(this, club, 1);
    }
    true
}

/// Considers sacking the manager: with low board confidence and the manager in the job for 150 days, the limit comes
/// from the division table, the runs and the manager flag; chairman draws hold back, issue the ultimatum, let the
/// manager leave or sack him (the departure queued, reason `get_departure_reason`), or look for a better candidate with
/// the pressure story.
///
/// `MANAGER_MANAGER::consider_sacking_manager(club, check)`: before the date `[G_GAME_START_DATE]`
/// (`exe::date_operator_3c`), with bit `board_record_flags::DEPARTURE_QUEUED` in `board_record::FLAGS` of the board
/// row, with the ultimatum resolved (`exe::manager_manager_resolve_board_ultimatum`, only with `check`), with
/// confidence (`board_record::CONFIDENCE`) above 4275 or with the manager in the job for less than 150 days: nothing. A
/// well-regarded manager (the non-player's `non_player::CURRENT_REPUTATION` above 5000 and above the club's reputation
/// `.reputation()` − 750) for less than 400 days: with random(chairman's patience `non_player::PATIENCE`, or 10) ≠ 0
/// the confidence goes to 4275. The limit (10 above 3500, 500 up to 3000, 200 in between) changes by the club's row in
/// the division table (`exe::comp_stats_find_club_entry`): more than 30 − patience/2 matches played
/// (`.matches_played()`) in the top places (`.best_position()` within `league::PROMOTION_PLAYOFF_COUNT`, or
/// `league::PROMOTED_COUNT` by the competition's `league_vtable::HAS_AUTOMATIC_PROMOTION`) holds; the last 5 results
/// (+0x14..+0x18) averaging above 2 hold; the runs `.winning_run()`, `.unbeaten_run()`, `.winless_run()` and
/// `.losing_run()` move the limit (×1.5 on `.winless_run()`). With the manager flagged
/// (`exe::player_regen_get_retirement_status`), the limit ×0.25. random(patience × 50, or 500) above the limit holds;
/// random(patience + 1, or 11) = 0 gives the ultimatum (`exe::manager_manager_issue_board_ultimatum`; once given,
/// nothing more). An AI manager (`exe::human_manager_is_human_club`), without the flag, with random(`.pressure()`) and
/// random(4) zero leaves (departure 2, `record_departure`). With a chairman: at a league club or national team,
/// random(patience) < 5 and random(20) ≠ 0 tries to swap for the best candidate (`check_better_candidate`); otherwise
/// the pressure news (`exe::manager_manager_post_manager_under_pressure_news` with 2) after 14 days
/// (`.last_review_date()`), and that is all. Without a chairman: random(3) = 0 tries the swap. Without a swap, the
/// non-player's confidence (`.current_reputation()`) drops by 250 (minimum 1500 with a chairman, 1 without) and the
/// manager is sacked (departure 1).
/// Original: `MANAGER_MANAGER::consider_sacking_manager` (0x69aba0, manager_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn consider_sacking_manager(this: usize, club: usize, check: u32) {
    let club = Club(club);
    if exe::date_operator_3c(G_CURRENT_DATE, read_usize(G_GAME_START_DATE) as u32, read_usize(G_GAME_START_DATE_FLAGS) as u32) != 0 {
        return;
    }
    if read_u8(club_board_row(this, club) + board_record::FLAGS) & board_record_flags::DEPARTURE_QUEUED != 0 {
        return;
    }
    if check as u8 != 0 {
        if exe::manager_manager_resolve_board_ultimatum(this, club) != 0 {
            return;
        }
    }
    if read_i16(club_board_row(this, club) + board_record::CONFIDENCE) > 0x10b3 {
        return;
    }
    let manager = club.manager().addr();
    if days_until_today(read_usize(manager + staff::DATE_JOINED_CLUB), read_usize(manager + staff::DATE_JOINED_CLUB_LEAP_YEAR)) < 0x96 {
        return;
    }
    let manager = club.manager().addr();
    if manager != 0 && read_usize(manager + staff::NON_PLAYER) != 0 {
        let manager_reputation = read_i16(read_usize(manager + staff::NON_PLAYER) + non_player::CURRENT_REPUTATION);
        if manager_reputation > 0x1388 && club.reputation().wrapping_sub(0x2ee) < manager_reputation
            && days_until_today(read_usize(manager + staff::DATE_JOINED_CLUB), read_usize(manager + staff::DATE_JOINED_CLUB_LEAP_YEAR)) < 0x190 {
            let presence = chairman_record(club);
            let n = if presence != 0 { read_u8(presence + non_player::PATIENCE) as i8 as i32 } else { 10 };
            if friendly_random(n) != 0 {
                write_i16(club_board_row(this, club) + board_record::CONFIDENCE, 0x10b3);
                return;
            }
        }
    }
    let v = read_i16(club_board_row(this, club) + board_record::CONFIDENCE);
    let mut limit: i32 = if v > 0xdac { 10 } else if v <= 0xbb8 { 500 } else { 200 };
    let div = club.division().addr();
    let comp = if div != 0 { read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(div).wrapping_mul(4))) } else { 0 };
    if comp != 0 && read_usize(comp + 8) != 0 {
        let ln = CompStatsClub(exe::comp_stats_find_club_entry(read_usize(comp + 8), club));
        if !ln.is_null() {
            let presence = chairman_record(club);
            let paciencia: i8 = if presence != 0 { (read_u8(presence + non_player::PATIENCE) as i8) >> 1 } else { 5 };
            let ceiling = 0x1e - paciencia as i32;
            let comp = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(club.division().addr()).wrapping_mul(4)));
            let slots = read_u8(comp + league::PROMOTION_PLAYOFF_COUNT) as i8;
            if slots != 0 && ln.best_position() as i32 <= slots as i32 && ln.matches_played() as i32 > ceiling {
                return;
            }
            let virtual_98: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(comp) + league_vtable::HAS_AUTOMATIC_PROMOTION));
            if virtual_98(comp) != 0 {
                let comp = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(club.division().addr()).wrapping_mul(4)));
                if ln.best_position() as i32 <= read_u8(comp + league::PROMOTED_COUNT) as i8 as i32 && ln.matches_played() as i32 > ceiling {
                    return;
                }
            }
            let (mut pontos, mut jogos) = (0i32, 0i32);
            for k in (0x14..=0x18usize).rev() {
                match read_u8(ln.addr() + k) {
                    3 => {
                        pontos += 3;
                        jogos += 1;
                    }
                    1 | 2 => {
                        pontos += 1;
                        jogos += 1;
                    }
                    0xfd => jogos += 1,
                    _ => {}
                }
            }
            if jogos as i16 > 0 && ((pontos as i16 as i32) / (jogos as i16 as i32)) as f64 > read_f64(rdata::F64_2) {
                return; // average above 2
            }
            let s19 = ln.winning_run() as i32;
            if s19 > 1 {
                if limit as i16 == 10 || s19 > 2 {
                    return;
                }
                if limit as i16 == 200 {
                    limit = 0x32;
                }
            } else {
                let s41 = ln.unbeaten_run();
                if s41 > 1 {
                    if limit as i16 == 10 || s41 > 3 {
                        return;
                    }
                    if limit as i16 == 200 {
                        limit = 100;
                    }
                } else if ln.winless_run() > 1 {
                    limit = ftol(limit as i16 as i32 as f64 * read_f64(rdata::F64_1_5)); // 1,5
                } else if (ln.losing_run() as i32) > 1 {
                    if limit as i16 == 10 {
                        limit = 200;
                    } else if limit as i16 == 200 {
                        limit = 300;
                    }
                }
            }
        }
    }
    let ceiling = if exe::player_regen_get_retirement_status(read_usize(G_PLAYER_REGEN), club.manager().addr()) != 0 {
        ftol(limit as i16 as i32 as f64 * read_f64(rdata::F64_0_25)) // 0,25
    } else {
        limit
    };
    let presence = chairman_record(club);
    let r = if presence != 0 {
        friendly_random(((read_u8(presence + non_player::PATIENCE) as i8 as i32) * 50) as i16 as i32)
    } else {
        friendly_random(500)
    };
    if r > ceiling as i16 as i32 {
        return;
    }
    let presence = chairman_record(club);
    let ultimato = if presence != 0 { friendly_random(read_u8(presence + non_player::PATIENCE) as i8 as i32 + 1) == 0 } else { friendly_random(0xb) == 0 };
    if ultimato {
        if exe::manager_manager_issue_board_ultimatum(this, club) != 0 {
            return;
        }
    }
    if club.manager().addr() != 0 {
        if exe::human_manager_is_human_club(club, 0, 1, 0, 0) == 0 && exe::player_regen_get_retirement_status(read_usize(G_PLAYER_REGEN), club.manager().addr()) == 0
            && friendly_random((club.manager().pressure() as u8) as i8 as i32) == 0 && friendly_random(4) == 0 {
            covered!("cpq_1 the manager leaves");
            record_departure(this, club, 2);
            return;
        }
    }
    let has_chairman = chairman_record(club) != 0;
    if has_chairman {
        if (exe::database_is_national_team(club) != 0 || club.division().addr() != 0)
            && friendly_random(read_u8(chairman_record(club) + non_player::PATIENCE) as i8 as i32) < 5 && friendly_random(0x14) != 0 {
            if check_better_candidate(this, club) {
                return;
            }
        } else {
            let l = BoardRecord(club_board_row(this, club));
            if days_until_today(l.last_review_date(), l.last_review_date_leap_year()) > 0xe {
                exe::manager_manager_post_manager_under_pressure_news(this, club, 2);
            }
            return;
        }
    } else if (exe::database_is_national_team(club) != 0 || club.division().addr() != 0) && friendly_random(3) == 0 && check_better_candidate(this, club) {
        return;
    }
    let manager = club.manager().addr();
    if manager != 0 && read_usize(manager + staff::NON_PLAYER) != 0 {
        let nj = NonPlayer(read_usize(manager + staff::NON_PLAYER));
        nj.set_current_reputation(nj.current_reputation().wrapping_sub(250));
        let floor: i16 = if has_chairman { 0x5dc } else { 1 };
        if nj.current_reputation() < floor {
            nj.set_current_reputation(floor);
        }
    }
    covered!("cpq_1 sacked");
    record_departure(this, club, 1);
}

// simplification: rolls and, depending on the roll, allocates (the output in the list, the ultimatum, the news): the path is only known
// running. Original only; the copy takes the branches and the career proves it (the sackings change everything).

/// Finds the club's first upcoming competitive fixture within 8 weeks, posts the ultimatum news against the opponent
/// and adds the club id to the ultimatum list [this+0x10]; returns 1 if issued.
pub(crate) unsafe fn issue_ultimatum(this: usize, club: Club, l: Llist) -> u8 {
    if club.is_null() || club.manager().addr() == 0 || club.division().addr() == 0 {
        return 0;
    }
    (exe::key_nation_index_of_club(club) as u32); // the original routine discards the value
    let (mut d1, mut d2) = ([0u8; 8], [0u8; 8]);
    let p1 = exe::date_add_weeks(G_CURRENT_DATE, (d1.as_mut_ptr()) as *mut [u8; 8], 8);
    let year_end = read_i16(p1 + 2) as u16 as u32;
    let p2 = exe::date_add_weeks(G_CURRENT_DATE, (d2.as_mut_ptr()) as *mut [u8; 8], 8);
    let day_end = read_i16(p2) as u16 as u32;
    // simplification: the original pushes the words with the high half of the register: for today’s day and the end day that of the
    // pointer of the second date, in today’s year that of the list pointer (reproduced by the ones here); in the end year that of the
    // edi of the caller, which 0x5acbe0 discards (it reads only the last words).
    let high = |p: usize| (p as u32) & 0xffff_0000;
    let find: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, usize, i32, i32, i32) -> u32 =
        core::mem::transmute(FN_FIXTURE_MANAGER_FIND_FIXTURES);
    find(read_usize(G_FIXTURE_MANAGER), high(p2) | read_i16(G_CURRENT_DATE) as u16 as u32, high(l.addr()) | read_i16(G_CURRENT_DATE_YEAR) as u16 as u32,
          high(p2) | day_end, year_end, read_usize(club.addr()) as u32, l.addr(), -1, -2, 1);
    if read_i16(l.addr()) == 0 {
        return 0;
    }
    l.set_cursor((l.head() as u32) as usize);
    let mut e = exe::llist_get_next(l);
    while e != 0 {
        let today = read_i16(e + fixture::DAY) as u16 as i32 == read_i16(G_CURRENT_DATE) as i32 && read_i16(e + fixture::YEAR) == read_i16(G_CURRENT_DATE_YEAR);
        if !today && read_u8(e + fixture::HOME_SIDE_MARK) != 0 && read_u8(e + fixture::HOME_GOALS) == 0xff {
            break;
        }
        e = exe::llist_get_next(l);
    }
    if e == 0 || read_usize(e + fixture::COMPETITION) == 0 {
        return 0;
    }
    let opponent = if read_usize(e + fixture::HOME_CLUB) == club.addr() { read_usize(e + fixture::AWAY_CLUB) } else { read_usize(e + fixture::HOME_CLUB) };
    exe::manager_manager_post_board_ultimatum_news(this, club, opponent, read_usize(e + fixture::COMPETITION));
    covered!("cpq_1 ultimatum given");
    let id = read_usize(club.addr()) as u32;
    list_add_tail(read_usize(this + manager_manager_fields::ULTIMATUM_LIST), &id as *const u32 as usize, 4);
    1
}

/// Issues the board ultimatum for the club's first match in the next 8 weeks (not today, still to play): the ultimatum
/// story with the opponent and the club in the ultimatum list; returns 1, or 0 without a match.
///
/// SEH frame for the local list. `MANAGER_MANAGER::issue_board_ultimatum(club)`: club with a manager (`.manager()`) and
/// a division (`.division()`): the local list (LLIST `exe::new_llist`, released by `exe::drop_llist`) receives the
/// matches of the next 8 weeks (`issue_ultimatum`); the first valid one gives the ultimatum news
/// (`exe::manager_manager_post_board_ultimatum_news`) and puts the club in the ultimatum list
/// `manager_manager_fields::ULTIMATUM_LIST`: returns 1. Otherwise 0.
/// Original: `MANAGER_MANAGER::issue_board_ultimatum` (0x6ac560, manager_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn issue_board_ultimatum(this: usize, club: usize) -> u8 {
    let mut list = [0u8; 16];
    let l = list.as_mut_ptr() as usize;
    exe::new_llist(l);
    let r = issue_ultimatum(this, Club(club), Llist(l));
    exe::drop_llist(l);
    r
}

// simplification: allocates the match list, the news item and the ultimatum list item: original only, checked by the plan:
// the ultimatum list [this+0x10] grows by 1 when it returns 1 and stays the same with 0.

/// The contract flags that the manager's departure clears: read by `contract_flags` (+0x40; 0x10000000 is cleared by `exe::staff_contract_clear_grievances`,
/// which touches +0x45) and read in +0x45 (cleared by `exe::staff_contract_clear_grievances`, which also updates the person).
pub(crate) const CONTRACT_FLAGS_BY_READER: [(u32, usize); 3] = [(0x1000_0000, 0x8674e0), (0x80, 0x8676d0), (0x100, 0x8676d0)];
pub(crate) const CONTRACT_FLAGS_AT_45: [u32; 8] = [0x800_0000, 8, 0x200, 0x400, 0x2000, 0x1_0000, 0x8_0000, 0x80_0000];

/// Returns the club's manager and his role byte (club job +0x39/+0x3d or national job +0x24/+0x28), if he holds the
/// post.
pub(crate) unsafe fn manager_and_role(club: Club) -> Option<(usize, u8)> {
    let manager = club.manager();
    if manager.is_null() {
        return None;
    }
    if manager.club_job() == club {
        Some((manager.addr(), manager.job_for_club()))
    } else if manager.national_job() == club {
        Some((manager.addr(), manager.job_for_nation()))
    } else {
        None
    }
}

/// Removes the manager from the club: the departure date, the squad and the staff, the contract (outside national
/// teams), the story (outside type 3), confidence at 3500 (unless kept), the job change record, the fines and the squad
/// contracts' grievances and flags.
///
/// `MANAGER_MANAGER::remove_club_manager(club, kind, reason, keep)`: with the manager in the job: at the human club
/// (`exe::human_manager_is_human_club`) the automatic replies (`exe::news_auto_respond_all`); the departure date
/// (`staff::DATE_JOINED_NATION` for a national team, `staff::DATE_JOINED_CLUB` for a club) becomes today's; with a
/// manager job (12, 13 or 15) he leaves the squad (`exe::manager_manager_remove_player_from_squad`); he leaves the
/// staff (`exe::manager_manager_remove_staff_from_club`); outside national teams he loses the contract
/// (`exe::contract_manager_remove_contract`); outside type 3 the departure news
/// (`exe::manager_manager_post_manager_departure_news` with 2 for manager jobs, 3 for the others); without `keep` the
/// confidence (`board_record::CONFIDENCE` of the row) goes to 3500. The change record
/// (`exe::staff_records_record_manager_job_change`, with 0 for kind 2 and 2 for the others), the club's fines
/// (`exe::fine_clear_club_fine_count`) and, outside national teams, the grievances (`contract::GRIEVANCES`,
/// `exe::staff_contract_clear_grievances`) and flags (`exe::staff_contract_get_flags`) of each squad player with a
/// contract (`exe::contract_manager_get_contract`) are removed.
/// Original: `MANAGER_MANAGER::remove_club_manager` (0x69b790, manager_manager.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn remove_club_manager(this: usize, club: usize, kind: u32, reason: u32, keep: u32) {
    let club = Club(club);
    let (tec, cargo) = match manager_and_role(club) {
        Some(t) => t,
        None => return,
    };
    covered!("cpq_1 remove the manager");
    if exe::human_manager_is_human_club(club, 1, 1, 0, 0) != 0 {
        exe::news_auto_respond_all(read_usize(G_NEWS), tec, club);
    }
    let mut t = [0u8; 8];
    exe::date_operator_3d(if exe::database_is_national_team(club) != 0 { tec + staff::DATE_JOINED_NATION } else { tec + staff::DATE_JOINED_CLUB }, (t.as_mut_ptr()) as usize, G_CURRENT_DATE);
    let c = cargo as i8;
    let of_manager = c >= 0xc && (c <= 0xd || c == 0xf);
    if of_manager {
        exe::manager_manager_remove_player_from_squad(this, tec, club);
    }
    exe::manager_manager_remove_staff_from_club(this, tec, club);
    if exe::database_is_national_team(club) == 0 {
        exe::contract_manager_remove_contract(read_usize(G_CONTRACT_MANAGER), tec);
    }
    if kind as u8 != 3 {
        exe::manager_manager_post_manager_departure_news(this, tec, kind, reason, club, if of_manager { 2 } else { 3 });
    }
    if keep == 0 {
        write_i16(club_board_row(this, club) + board_record::CONFIDENCE, 0xdac);
    }
    exe::staff_records_record_manager_job_change(read_usize(G_STAFF_RECORDS), tec, club, if kind as u8 == 2 { 0 } else { 2 });
    if read_usize(G_FINE) != 0 {
        exe::fine_clear_club_fine_count(read_usize(G_FINE), club);
    }
    if exe::database_is_national_team(club) != 0 {
        return;
    }
    for k in 0..50usize {
        let p = Staff(club.squad(k));
        if p.is_null() || p.player().is_null() {
            continue;
        }
        let ct = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), p));
        if ct.is_null() {
            continue;
        }
        for (m, f) in CONTRACT_FLAGS_BY_READER {
            if exe::staff_contract_get_flags(ct) & m != 0 {
                let erase: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(f);
                erase(ct.addr(), m);
            }
        }
        for m in CONTRACT_FLAGS_AT_45 {
            if (ct.grievances() as usize) as u32 & m != 0 {
                exe::staff_contract_clear_grievances(ct, m);
            }
        }
    }
}

// simplification: removes the contract, releases the news item and writes the history to a file (does not run twice): original only,
// checked afterwards: without `manter` the confidence at 3500 and, outside the national team, no squad contract with the marks.

/// Adds v to the i16 confidence at c, clamped to [floor, ceiling] (ceiling checked first).
pub(crate) unsafe fn add_clamped_i16(c: usize, v: i16, floor: i16, ceiling: i16) {
    write_i16(c, read_i16(c).wrapping_add(v));
    if read_i16(c) > ceiling {
        write_i16(c, ceiling);
    } else if read_i16(c) < floor {
        write_i16(c, floor);
    }
}

/// Appends a national-team manager departure (1, reason, national team id) to the departures list [this+4].
pub(crate) unsafe fn record_national_departure(this: usize, national_team: Club, reason: u8) {
    let mut reg = [0u8; 8];
    reg[0] = 1;
    reg[1] = reason;
    reg[2..6].copy_from_slice(&(read_usize(national_team.addr()) as u32).to_le_bytes());
    exe::llist_add_tail(read_usize(this + manager_manager_fields::DEPARTURE_LIST), reg.as_mut_ptr() as usize, 6);
}

/// Calls `consider_sacking_manager` (FN_MANAGER_MANAGER_CONSIDER_SACKING_MANAGER) without the ultimatum check: the federation considers
/// sacking.
pub(crate) unsafe fn consider_sacking(this: usize, national_team: Club) {
    covered!("cpq_2 the federation considers");
    exe::manager_manager_consider_sacking_manager(this, national_team, 0);
}

/// Reviews the national team's performance: the FA confidence from the manager's grade in the competition (−3..3,
/// `comp_vtable::GET_QUALIFICATION_STATUS_CHANGE`), the assessment story and, by time in the job, reputation and random
/// draws, the manager's departure or the FA considering the sacking (`consider_sacking`).
///
/// `MANAGER_MANAGER::review_national_team_performance(comp, national_team)`: a national team with a manager
/// (`.manager()`) and a competition in `[G_COMP_OBJECT_TABLE]`: with the manager there for less than 365 days, the FA
/// confidence (`board_record::CONFIDENCE` of the row) rises to 6500. The manager's grade in the competition (its
/// `comp_vtable::GET_QUALIFICATION_STATUS_CHANGE`) changes the confidence: 3: +1250 (ceiling 10000, floor 8500); 2:
/// +750 (ceiling 10000, floor 6500), and the AI national team with the manager there for more than 3650 days lets him
/// go if random(15000) + 3750 < reputation (`.reputation()`); 0: −250 and, only with the row's
/// `board_record::EXPECTATION_FLAGS` = −3, 3500 and the departure (more than 3650 days) or, with reputation above 6750
/// and more than 1460 days, random(15000) against the reputation (below: departure; otherwise the FA considers it,
/// `consider_sacking`); −1: −750 (1..3500), departure after 1460 days, random(20000) after 730; −2: −1250 (1..3000),
/// departure after 730 days, otherwise random(15000) + 1250; −3: −1750 (1..3000) and the departure. The assessment goes
/// into the news (`exe::manager_manager_post_fa_performance_news`) right after the change. The departures
/// (`record_national_departure`) carry reason 6 (2, 0 and −1 with longer tenure) or 0.
/// Original: `MANAGER_MANAGER::review_national_team_performance` (0x6a5610, manager_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn review_national_team_performance(this: usize, comp: usize, national_team: usize) {
    let national_team = Club(national_team);
    let manager = national_team.manager();
    let object = || read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(comp).wrapping_mul(4)));
    if manager.is_null() || object() == 0 {
        return;
    }
    let days = exe::date_days_since(G_CURRENT_DATE, (manager.date_joined_club()) as u32, (manager.date_joined_club_leap_year()) as u32);
    if days < 0x16d && read_i16(club_board_row(this, national_team) + board_record::CONFIDENCE) < 0x1964 {
        write_i16(club_board_row(this, national_team) + board_record::CONFIDENCE, 0x1964); // new manager
    }
    let o = object();
    let rating_of: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(read_usize(read_usize(o) + comp_vtable::GET_QUALIFICATION_STATUS_CHANGE));
    let rating = rating_of(o, national_team.addr()) as u8 as i8;
    covered!("cpq_2 national team evaluated");
    let c = club_board_row(this, national_team) + board_record::CONFIDENCE;
    let injury_word_0c = || national_team.reputation();
    match rating {
        3 => {
            add_clamped_i16(c, 0x4e2, 0x2134, 0x2710);
            exe::manager_manager_post_fa_performance_news(this, comp, national_team, 3);
        }
        2 => {
            add_clamped_i16(c, 0x2ee, 0x1964, 0x2710);
            exe::manager_manager_post_fa_performance_news(this, comp, national_team, 2);
            if exe::human_manager_is_human_club(national_team, 0, 1, 0, 0) != 0 || days <= 0xe42 {
                return;
            }
            if (friendly_random(0x3a98) + 0xea6) as i16 >= injury_word_0c() {
                return;
            }
            record_national_departure(this, national_team, 6);
        }
        0 => {
            write_i16(c, read_i16(c).wrapping_sub(250));
            if read_i16(club_board_row(this, national_team) + board_record::EXPECTATION_FLAGS) != -3 {
                return;
            }
            write_i16(c, 0xdac);
            exe::manager_manager_post_fa_performance_news(this, comp, national_team, 0);
            if days > 0xe42 {
                return record_national_departure(this, national_team, 6);
            }
            if injury_word_0c() <= 0x1a5e || days <= 0x5b4 {
                return;
            }
            if friendly_random(0x3a98) >= injury_word_0c() as i32 {
                return consider_sacking(this, national_team);
            }
            record_national_departure(this, national_team, 6);
        }
        -1 => {
            add_clamped_i16(c, -750, 1, 0xdac);
            exe::manager_manager_post_fa_performance_news(this, comp, national_team, -1);
            if days > 0x5b4 {
                return record_national_departure(this, national_team, 6);
            }
            if days <= 0x2da {
                return;
            }
            if friendly_random(0x4e20) >= injury_word_0c() as i32 {
                return consider_sacking(this, national_team);
            }
            record_national_departure(this, national_team, 0);
        }
        -2 => {
            add_clamped_i16(c, -1250, 1, 0xbb8);
            exe::manager_manager_post_fa_performance_news(this, comp, national_team, -2);
            if days <= 0x2da && friendly_random(0x3a98) + 0x4e2 >= injury_word_0c() as i32 {
                return consider_sacking(this, national_team);
            }
            record_national_departure(this, national_team, 0);
        }
        -3 => {
            add_clamped_i16(c, -1750, 1, 0xbb8);
            exe::manager_manager_post_fa_performance_news(this, comp, national_team, -3);
            record_national_departure(this, national_team, 0);
        }
        _ => {}
    }
}

// simplification: releases the evaluation news (0x6a5a60), rolls and allocates the output (or calls 0x69aba0): original only;
// the copy takes the branches.

/// Returns 1 for a manageable club, not a national team, with board confidence at row
/// `board_record::SUPPORTERS_CONFIDENCE` under 3000 and a negative balance.
///
/// `MANAGER_MANAGER::is_club_in_financial_trouble(club)`: a manageable club (`exe::club_can_be_managed`), not a
/// national team (`exe::database_is_national_team`), with the board confidence at `board_record::SUPPORTERS_CONFIDENCE`
/// of the row below 3000 and the balance (64 bits in the finance record, `exe::financetable_get_club_finance` on
/// `[G_FINANCE_TABLE]`) negative: 1. Otherwise 0. Used by the match engine (crowd chants).
/// Original: `MANAGER_MANAGER::is_club_in_financial_trouble` (0x69eb50, manager_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn is_club_in_financial_trouble(this: usize, club: usize) -> u8 {
    if club == 0 {
        return 0;
    }
    if (exe::club_can_be_managed(club) as u32) as u8 == 0 || exe::database_is_national_team(club) != 0 {
        return 0;
    }
    let f = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club);
    if read_i16(club_board_row(this, Club(club)) + board_record::SUPPORTERS_CONFIDENCE) >= 0xbb8 || f == 0 {
        return 0;
    }
    (read_i64(f) < 0) as u8
}

/// Returns the 0..6 band of the board confidence (limits 3000, 3500, 4275, 6500, 8500, 9500).
///
/// `MANAGER_MANAGER::get_board_confidence_band(club)`: the band (0..6) of the board confidence
/// (`board_record::CONFIDENCE` of the row) by the limits 3000, 3500, 4275, 6500, 8500 and 9500. Used by the board
/// report text (0x690d80).
/// Original: `MANAGER_MANAGER::get_board_confidence_band` (0x690b70, manager_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn get_board_confidence_band(this: usize, club: usize) -> u8 {
    let v = read_i16(club_board_row(this, Club(club)) + board_record::CONFIDENCE);
    [0xbb8i16, 0xdac, 0x10b3, 0x1964, 0x2134, 0x251c].iter().take_while(|&&l| v >= l).count() as u8
}

/// Returns the board row of club id in the MANAGER_MANAGER table ([this] + 0x49 × id): wealth band, flags, confidence,
/// moods (takes the club id instead of the club).
pub(crate) unsafe fn board_row_by_id(this: usize, id: u32) -> usize {
    (read_u32(this) as usize).wrapping_add((id as usize).wrapping_mul(0x49))
}

/// New mood value: ftol(f32(random(500) + q) − random(500)) as i16.
pub(crate) unsafe fn roll_mood(q: f32) -> i16 {
    let s1 = random(0x1f4);
    let base = (s1 as f64 + q as f64) as f32; // fstp dword
    let s2 = random(0x1f4);
    ftol(base as f64 - s2 as f64) as i16
}

/// Resets the board for a new manager: recalculates the expectations, moves the manager's reputation towards the club's
/// and draws the board confidence and moods from the reputation; clears the changes and flags, records the manager and
/// stores the wealth band.
///
/// turned off (DESLIGADAS) from 2026-10-30 to 2026-10-31, when the test criterion changed and the list was emptied:
/// with the detour, the career in Rust mode left the reference be54a138 (bisection of the 39 routines of batch 252);
/// the cause (stack residue or a difference in the copy) is not isolated yet; since then the routine is hooked
/// again.
///
/// `MANAGER_MANAGER::reset_board_for_new_manager(club)`: sets bit `board_record_flags::NEW_MANAGER` of
/// `board_record::FLAGS` of the board row, recalculates the season expectations
/// (`exe::manager_manager_set_season_expectations` with the news) and sets it again; the club list of the
/// Shortlist_Manager (`shortlist_manager_fields::CLUB_SHORTLISTS`, 9 bytes per club, +4) is set to 2. The manager's
/// (`.manager()`, record `.non_player()`) reputation: in the same nation (`.nation()` = the manager's `.nation()`) the
/// current (`non_player::CURRENT_REPUTATION`) and domestic (`non_player::HOME_REPUTATION`) move 10% towards the club's
/// (`.reputation()`); otherwise the current moves 10% starting from the other-nation one
/// (`non_player::WORLD_REPUTATION`). The base: the current reputation (zero: half the club's, or 5000 without; below
/// 500: 500 + a random draw of 1000) over the club's (zero: itself; below 500: 250 + a random draw of 750) × 7500, as a
/// float; above the double at `rdata::F64_5_811479333ENEG_315` (read as a double: a positive denormal) it becomes
/// 10000, below 6500, 6500. The confidence (`board_record::CONFIDENCE`) and the moods
/// `board_record::SUPPORTERS_CONFIDENCE`/`board_record::STAFF_CONFIDENCE`/`board_record::CONFIDENCE_4` come from the
/// base with two random draws of 500 each (`roll_mood`; limits 6250..9500, 6250..10000, 5250..10000, 5750..10000, with
/// random draws on the lower ones); clears the changes +0x10..`board_record::FAVOURITES_MOOD`, the doubles
/// `board_record::SQUAD_INVESTMENT`, `board_record::EXTRA_FUNDS_GRANTED`, `board_record::REQUESTS_MADE`,
/// `board_record::WAGE_RISE_GRANTED` and flags (`board_record::FLAGS` &~0x2e, `board_record::REQUEST_STATE` &0xe0,
/// `board_record::DISCIPLINE_COUNTERS` &7). Records the new manager (`exe::staff_records_record_manager_job_change`
/// with 1). With a human manager (id ≥ `[G_STAFF_COUNT]` − 16): cancels the club's offers
/// (`exe::transfer_manager_cancel_club_offers`), rebuilds the club's list
/// (`exe::shortlist_manager_rebuild_club_shortlist`) and the tactics of the 50 slots
/// (`exe::tactics_remove_player_from_selection`). `board_record::WEALTH_BAND` = the finances' wealth band
/// (`exe::finance_get_wealth_band`), 0 for national teams or without finances.
/// Original: `MANAGER_MANAGER::reset_board_for_new_manager` (0x694c80, manager_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn reset_board_for_new_manager(this: usize, club: usize) {
    let club = Club(club);
    let id = read_u32(club.addr());
    *(((board_row_by_id(this, id)) + board_record::FLAGS) as *mut u8) |= board_record_flags::NEW_MANAGER;
    exe::manager_manager_set_season_expectations(this, club, 1);
    *(((board_row_by_id(this, id)) + board_record::FLAGS) as *mut u8) |= board_record_flags::NEW_MANAGER;
    let list = read_u32(read_u32(read_u32(G_TRANSFER_MANAGER) as usize + transfer_manager_fields::SHORTLIST_MANAGER) as usize + shortlist_manager_fields::CLUB_SHORTLISTS) as usize;
    *(list.wrapping_add((id as usize).wrapping_mul(9)) as *mut u8).add(4) = 2;
    let k = read_f64(rdata::F64_0_1); // 0,1;
    let rep = club.reputation() as i32;
    let manager = Staff((club.manager().addr() as u32) as usize);
    let sheet = (manager.non_player().addr() as u32) as usize;
    if (club.nation().addr() as u32) == (manager.nation().addr() as u32) {
        let a = read_i16(sheet + non_player::CURRENT_REPUTATION) as i32;
        write_i16(sheet + non_player::CURRENT_REPUTATION, ftol((rep - a) as f64 * k + a as f64) as i16);
        let sheet = read_u32((club.manager().addr() as u32) as usize + staff::NON_PLAYER) as usize;
        let b = read_i16(sheet + non_player::HOME_REPUTATION) as i32;
        write_i16(sheet + non_player::HOME_REPUTATION, ftol((rep - b) as f64 * k + b as f64) as i16);
    } else {
        let c = read_i16(sheet + non_player::WORLD_REPUTATION) as i32;
        let a = read_i16(sheet + non_player::CURRENT_REPUTATION) as i32;
        write_i16(sheet + non_player::CURRENT_REPUTATION, ftol((rep - c) as f64 * k + a as f64) as i16);
    }
    let sheet = read_u32((club.manager().addr() as u32) as usize + staff::NON_PLAYER) as usize;
    let v = read_i16(sheet + non_player::CURRENT_REPUTATION) as i32;
    let mut current = v as f32; // fst dword
    let zero = read_f32(rdata::F32_0) as f64;
    if v as f64 == zero {
        let r = club.reputation() as i32;
        current = if r != 0 { (r / 2) as f32 } else { 5000.0 };
    } else if (current as f64) < read_f32(rdata::F32_500) as f64 {
        current = (random(0x3e8) + 0x1f4) as f32;
    }
    let d = club.reputation() as i32;
    let divisor = if d as f64 == zero {
        current as f64
    } else if (d as f64) < read_f32(rdata::F32_500) as f64 {
        (random(0x2ee) + 0xfa) as f64
    } else {
        d as f64
    };
    let mut q = ((current as f64 / divisor) * read_f32(rdata::F32_7500) as f64) as f32; // fstp dword;
    if q as f64 > read_f64(rdata::F64_5_811479333ENEG_315) {
        covered!("cpp_3 base at the ceiling");
        q = 10000.0;
    } else if !(q as f64 >= read_f32(rdata::F32_6500) as f64) {
        q = 6500.0;
    }
    // confidence and moods, each with its own limits
    let h = roll_mood(q);
    write_i16(board_row_by_id(this, id) + board_record::CONFIDENCE, h);
    let c = read_i16(board_row_by_id(this, id) + board_record::CONFIDENCE);
    if c < 0x186a {
        write_i16(board_row_by_id(this, id) + board_record::CONFIDENCE, (random(0x4e2) + 0x186a) as i16);
    } else if c > 0x251c {
        write_i16(board_row_by_id(this, id) + board_record::CONFIDENCE, (0x2710 - random(0x1f4)) as i16);
    }
    let s1 = random(0x1f4);
    let parcial = read_i16(board_row_by_id(this, id) + board_record::CONFIDENCE).wrapping_sub(s1 as i16);
    let s2 = random(0x1f4);
    write_i16(board_row_by_id(this, id) + board_record::SUPPORTERS_CONFIDENCE, parcial.wrapping_add(s2 as i16));
    let c = read_i16(board_row_by_id(this, id) + board_record::SUPPORTERS_CONFIDENCE);
    if c < 0x186a {
        write_i16(board_row_by_id(this, id) + board_record::SUPPORTERS_CONFIDENCE, (random(0x4e2) + 0x186a) as i16);
    } else if c > 0x2710 {
        write_i16(board_row_by_id(this, id) + board_record::SUPPORTERS_CONFIDENCE, 0x2710);
    }
    let h = roll_mood(q);
    write_i16(board_row_by_id(this, id) + board_record::STAFF_CONFIDENCE, h);
    let c = read_i16(board_row_by_id(this, id) + board_record::STAFF_CONFIDENCE);
    if c < 0x1482 {
        write_i16(board_row_by_id(this, id) + board_record::STAFF_CONFIDENCE, (random(0xfa) + 0x1482) as i16);
    } else if c > 0x2710 {
        write_i16(board_row_by_id(this, id) + board_record::STAFF_CONFIDENCE, 0x2710);
    }
    let h = roll_mood(q);
    write_i16(board_row_by_id(this, id) + board_record::CONFIDENCE_4, h);
    let c = read_i16(board_row_by_id(this, id) + board_record::CONFIDENCE_4);
    if c < 0x1676 {
        write_i16(board_row_by_id(this, id) + board_record::CONFIDENCE_4, (random(0xfa) + 0x1676) as i16);
    } else if c > 0x2710 {
        write_i16(board_row_by_id(this, id) + board_record::CONFIDENCE_4, 0x2710);
    }
    let e = BoardRecord(board_row_by_id(this, id));
    for o in [0x10, board_record::LEAGUE_CONFIDENCE, board_record::TRANSFER_MOOD, board_record::FAVOURITES_MOOD] {
        write_i16(e.addr() + o, 0); // the changes
    }
    let m = |o: usize, mask: u8| *((board_row_by_id(this, id) + o) as *mut u8) &= mask;
    m(board_record::FLAGS, !board_record_flags::PROJECTED_BELOW_TARGET);
    m(board_record::FLAGS, !board_record_flags::PROJECTED_ABOVE_TARGET);
    m(board_record::FLAGS, !board_record_flags::DEPARTURE_QUEUED);
    write_u32(e.addr() + board_record::SQUAD_INVESTMENT, 0);
    write_u32(e.addr() + board_record::SQUAD_INVESTMENT + 4, 0);
    e.set_requests_made(0);
    m(board_record::REQUEST_STATE, !request_state::LAST_REQUEST as u8);
    m(board_record::FLAGS, 0xdf);
    e.set_extra_funds_granted(0);
    e.set_wage_rise_granted(0);
    m(board_record::REQUEST_STATE, !request_state::EXTRA_TIME_GRANTED as u8);
    m(board_record::DISCIPLINE_COUNTERS, !discipline_counters::APPEALS_MADE);
    exe::staff_records_record_manager_job_change(read_u32(G_STAFF_RECORDS) as usize, (club.manager().addr() as u32) as usize, club, 1);
    if read_u32((club.manager().addr() as u32) as usize) as i32 >= (read_u32(G_STAFF_COUNT) as i32).wrapping_sub(0x10) {
        exe::transfer_manager_cancel_club_offers(read_u32(G_TRANSFER_MANAGER) as usize, club);
        exe::shortlist_manager_rebuild_club_shortlist(read_u32(read_u32(G_TRANSFER_MANAGER) as usize + transfer_manager_fields::SHORTLIST_MANAGER) as usize, club);
        for k in 0..0x32 {
            let p = (club.squad(k) as u32) as usize;
            if p != 0 {
                exe::tactics_remove_player_from_selection(read_u32(G_TACTICS) as usize, club, p, 1, 1);
            }
        }
    }
    let mut band = 0u8;
    if exe::database_is_national_team(club) == 0 {
        let fin = exe::financetable_get_club_finance(read_u32(G_FINANCE_TABLE) as usize, club);
        if fin != 0 {
            band = exe::finance_get_wealth_band(fin);
        }
    }
    write_u8(board_row_by_id(this, read_u32(club.addr())) + board_record::WEALTH_BAND, band);
}

/// Returns the reason (0 to 11) for the manager's departure: unsettled players, league position, debt, board changes
/// and moods (with random draws) and time in the job.
///
/// `MANAGER_MANAGER::get_departure_reason(id)`: the reason (0..11) for the departure of the manager of club `id`.
/// National team (`exe::database_is_national_team`): 0. More than 2 squad players with grievance 0x10000000 in the
/// contract (`contract::GRIEVANCES`): 9. In the season (`exe::club_is_in_season`): in the league with the nation
/// flagged (`.league_selected()`), a table row with more than 25 matches played (`.matches_played()`) and a winless run
/// (`.winless_run()`) above 1, by the competition's position 0x9c: progress (`.current_position()`) inside and the
/// projected position (`exe::comp_stats_get_club_projected_positions`) outside the (`league::CLUB_COUNT` −
/// `league::RELEGATED_COUNT`) places: 11; debt (double `board_record::SPENDING` < `board_record::SQUAD_INVESTMENT`)
/// with the change `board_record::LEAGUE_CONFIDENCE` below −300: 10. Then: `board_record::LEAGUE_CONFIDENCE` below −750
/// outside a type 1 run (`exe::manager_manager_get_current_run`) with a random draw of 3 = 0: 2; in the season with bit
/// `board_record_flags::PROJECTED_BELOW_TARGET` of `board_record::FLAGS`: 7; +0x10 < −350: 3;
/// `board_record::TRANSFER_MOOD` < −750: 5; mood `board_record::STAFF_CONFIDENCE` below confidence
/// `board_record::CONFIDENCE` and random(`board_record::STAFF_CONFIDENCE`) < random(`board_record::CONFIDENCE`): 8;
/// `board_record::FAVOURITES_MOOD` < −750 with a random draw of 3 ≠ 0: 4; the manager in the job for more days
/// (`exe::date_days_since` since `.date_joined_club()`) than 1000 + a random draw of 2000: 6. Otherwise 0.
/// Original: `MANAGER_MANAGER::get_departure_reason` (0x69b350, manager_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn get_departure_reason(this: usize, id: u32) -> u8 {
    let club = Club((read_u32(G_CLUB_TABLE) as usize).wrapping_add((id as usize).wrapping_mul(club::SIZE)));
    if exe::database_is_national_team(club) != 0 {
        return 0;
    }
    covered!("cpp_3 reason for leaving");
    let season = exe::club_is_in_season(club, G_CURRENT_DATE);
    let (mut tipo, mut tamanho) = (0u8, 0u8);
    exe::manager_manager_get_current_run(this, club, &mut tipo, &mut tamanho);
    let mut queixosos = 0u8;
    for k in 0..0x32 {
        let p = Staff((club.squad(k) as u32) as usize);
        if !p.is_null() && (p.player().addr() as u32) != 0 {
            let c = Contract(exe::contract_manager_get_contract_for_club(read_u32(G_CONTRACT_MANAGER) as usize, p, club));
            if !c.is_null() && c.grievances() & 0x1000_0000 != 0 {
                queixosos = queixosos.wrapping_add(1);
            }
        }
    }
    if queixosos as i8 > 2 {
        return 9;
    }
    if season != 0 {
        let league = (club.division().addr() as u32) as usize;
        let nation = Nation((club.nation().addr() as u32) as usize);
        if league != 0 && !nation.is_null() && nation.league_selected() != 0 {
            let comp = read_u32((read_u32(G_COMP_OBJECT_TABLE) as usize).wrapping_add((read_u32(league) as usize).wrapping_mul(4))) as usize;
            if comp != 0 {
                let row = CompStatsClub(exe::comp_stats_find_club_entry(read_u32(comp + 8) as usize, club));
                let (mut pa, mut pb, mut pc) = (0u8, 0u8, 0u8); // 0x4ae300 writes all three (with the row found)
                let comp = read_u32((read_u32(G_COMP_OBJECT_TABLE) as usize).wrapping_add((read_u32((club.division().addr() as u32) as usize) as usize).wrapping_mul(4))) as usize;
                exe::comp_stats_get_club_projected_positions(read_u32(comp + 8) as usize, club, &mut pa, &mut pb, &mut pc);
                if row.matches_played() > 0x19 && (row.winless_run() as u32) as i32 > 1 {
                    let comp = read_u32((read_u32(G_COMP_OBJECT_TABLE) as usize).wrapping_add((read_u32((club.division().addr() as u32) as usize) as usize).wrapping_mul(4))) as usize;
                    let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_u32(read_u32(comp) as usize + 0x9c) as usize);
                    if f(comp) != 0 {
                        let comp = read_u32((read_u32(G_COMP_OBJECT_TABLE) as usize).wrapping_add((read_u32((club.division().addr() as u32) as usize) as usize).wrapping_mul(4))) as usize;
                        let slots = read_i16(comp + league::CLUB_COUNT) as i32 - read_u8(comp + league::RELEGATED_COUNT) as i8 as i32;
                        if row.current_position() as i32 >= slots && (pa as i32) < slots {
                            return 0xb;
                        }
                    }
                }
            }
        }
        let e = board_row_by_id(this, id);
        let (gasto, teto) = (read_f64(e + board_record::SPENDING), read_f64(e + board_record::SQUAD_INVESTMENT));
        if !(gasto >= teto) && read_i16(e + board_record::LEAGUE_CONFIDENCE) < -300 {
            return 0xa;
        }
    }
    let e = board_row_by_id(this, id);
    if read_i16(e + board_record::LEAGUE_CONFIDENCE) < -750 && tipo != 1 && random(3) == 0 {
        covered!("cpp_3 reason 2");
        return 2;
    }
    if season != 0 && read_u8(e + board_record::FLAGS) & board_record_flags::PROJECTED_BELOW_TARGET != 0 {
        return 7;
    }
    if read_i16(e + 0x10) < -350 {
        return 3;
    }
    if read_i16(e + board_record::TRANSFER_MOOD) < -750 {
        return 5;
    }
    let humor = read_i16(e + board_record::STAFF_CONFIDENCE);
    if humor < read_i16(e + board_record::CONFIDENCE) {
        let a = random(read_i16(board_row_by_id(this, id) + board_record::CONFIDENCE) as i32);
        let b = random(humor as i32);
        if b < a {
            return 8;
        }
    }
    if read_i16(board_row_by_id(this, id) + board_record::FAVOURITES_MOOD) < -750 && random(3) != 0 {
        return 4;
    }
    let manager = Staff((club.manager().addr() as u32) as usize);
    if !manager.is_null() {
        let d = exe::date_days_since(G_CURRENT_DATE, manager.date_joined_club() as u32 , manager.date_joined_club_leap_year() as u32);
        if d > random(0x7d0) + 0x3e8 {
            return 6;
        }
    }
    0
}

/// Returns the 210-byte manager record read by `get_manager_record`: humans by their slot after the non-player records, others by
/// the non-player record id; None without one.
pub(crate) unsafe fn manager_manager_manager_record(this: usize, person: Staff) -> Option<usize> {
    let n = read_u32(G_STAFF_COUNT) as i32;
    let id = read_u32(person.addr()) as i32;
    let k = if id >= n.wrapping_sub(0x10) && id < n {
        let s = person.classification();
        if s == 0 || s == 5 {
            return None;
        }
        id.wrapping_sub(n).wrapping_add(read_u32(G_NON_PLAYER_COUNT) as i32).wrapping_add(0x10)
    } else {
        let sheet = (person.non_player().addr() as u32) as usize;
        if sheet == 0 {
            return None;
        }
        read_u32(sheet) as i32
    };
    Some((read_u32(this + staff_records_fields::MANAGER_RECORDS) as usize).wrapping_add((k as usize).wrapping_mul(0xd2)))
}

/// ORs v into the byte at address a.
pub(crate) unsafe fn or_byte(a: usize, v: u8) {
    *(a as *mut u8) |= v;
}

/// Adds v to the i16 at address a with wrap-around.
pub(crate) unsafe fn manager_manager_add_i16(a: usize, v: i16) {
    write_i16(a, read_i16(a).wrapping_add(v));
}

/// Reads a double constant from the exe at address a.
pub(crate) unsafe fn exe_f64(a: usize) -> f64 {
    read_f64(a)
}

/// Returns the club's board row in the MANAGER_MANAGER table ([this] + 0x49 × id): flags, confidence, moods, net
/// spending (takes the club id instead of the club).
pub(crate) unsafe fn board_row_at(this: usize, id: usize) -> usize {
    read_usize(this).wrapping_add(id.wrapping_mul(0x49))
}

/// Player value via CONTRACT_MANAGER `calc_player_value`, using the player's current club if any, else the given club, with the
/// last argument 1 for human clubs.
pub(crate) unsafe fn player_value_for_club(person: Staff, club: Club) -> i32 {
    let h = exe::human_manager_is_human_club(club, 0, 1, 0, 0) as i32;
    let current = person.club_job();
    let (alvo, ultimo) = if h != 0 {
        (if !current.is_null() { current } else { club }, 1)
    } else if !current.is_null() {
        (current, 1)
    } else {
        (club, 0)
    };
    exe::contract_manager_calc_player_value(read_usize(G_CONTRACT_MANAGER), person, alvo, 0, -1, ultimo)
}

/// Fee limit from club reputation: (reputation × 0.2)² truncated.
pub(crate) unsafe fn fee_limit(club: Club) -> i32 {
    let x = club.reputation() as i32 as f64 * exe_f64(0x968918);
    ftol(x * x)
}

/// Flags the deal for buyer and seller by the fee/player value ratio (bands for values above or up to 250,000; a fee up
/// to (reputation × 0.2)² is fair), adds and subtracts the value from each one's net spend and flags the club's and
/// chairman's favoured or disliked player.
///
/// `MANAGER_MANAGER::assess_transfer_deal(transf, f1, f2)`: with the player (`.target_staff_id()`) who has a record
/// (`.player()`), clears the two flag words and: for the buying club (`.buyer_club_id()`, 0x245-byte record) adds the
/// player's value (`player_value_for_club`, 1 if 0) to the net spend (`board_record::SQUAD_INVESTMENT`) and flags `f1`
/// by the ratio fee (`exe::transfer_offer_get_total_value`) / value: fee up to the limit (reputation × 0.2)²
/// (`fee_limit`) = 4; value above 250,000: < 0.2 = 0x10, < 0.5 = 8, < 2 = 4, < 6 = 2, otherwise 1; up to 250,000: <
/// 0.125 = 8, < 8 = 4, otherwise 2; the favoured or rejected player of the club or of the chairman (`.chairman()`;
/// `exe::club_favours_staff`/`exe::club_dislikes_staff`/`exe::staff_is_favoured_by`/`exe::staff_is_disliked_by`) = 0x40
/// or 0x100. For the selling club (the player's `.club_job()`) it subtracts the value from the spend and flags `f2`
/// with the mirrored bands (0.25/0.5/2/4; 0.125/8) and 0x20 or 0x80. A loan offer (`.offer_type()` = 1): nothing (the
/// flags stay as they are).
/// Original: `MANAGER_MANAGER::assess_transfer_deal` (0x696570, manager_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn assess_transfer_deal(this: usize, transf: usize, f1: usize, f2: usize) {
    let transf = Offer(transf);
    if transf.offer_type() == 1 {
        return;
    }
    write_u16(f2, 0);
    write_u16(f1, 0);
    let person = Staff(read_usize(G_STAFF_TABLE).wrapping_add(transf.target_staff_id().wrapping_mul(staff::SIZE)));
    if person.player().is_null() {
        return;
    }
    let buyer = Club(read_usize(G_CLUB_TABLE).wrapping_add(transf.buyer_club_id().wrapping_mul(club::SIZE)));
    let mut value = player_value_for_club(person, buyer);
    if value == 0 {
        value = 1;
    }
    let taxa = exe::transfer_offer_get_total_value(transf, 0, 0, 0, 1);
    if !buyer.is_null() {
        let g = board_row_at(this, read_usize(buyer.addr())) + board_record::SQUAD_INVESTMENT;
        write_f64(g, value as f64 + read_f64(g));
        let limit = fee_limit(buyer);
        let ratio = if value == 0 { read_f32(rdata::F32_0_5) as f64 } else { taxa as f64 / value as f64 };
        if taxa <= limit {
            covered!("cpo_1 buyer at the limit");
            or_byte(f1, 4); // the buyer: fee within the limit
        } else if value > 0x3d090 {
            let m = if ratio < exe_f64(0x968918) {
                0x10
            } else if ratio < exe_f64(0x968968) {
                8
            } else if ratio < exe_f64(0x968950) {
                4
            } else if ratio < exe_f64(0x969000) {
                2
            } else {
                1
            };
            or_byte(f1, m);
        } else {
            let m = if ratio < exe_f64(0x968fd0) { 8 } else if ratio < exe_f64(0x968f10) { 4 } else { 2 };
            or_byte(f1, m);
        }
        if exe::club_favours_staff(person, buyer) != 0 {
            or_byte(f1, 0x40);
        } else if exe::club_dislikes_staff(person, buyer) != 0 {
            or_byte(f1 + 1, 1);
        }
        if exe::staff_is_favoured_by(person, buyer.chairman().addr()) != 0 {
            or_byte(f1, 0x40);
        } else if exe::staff_is_disliked_by(person, buyer.chairman().addr()) != 0 {
            or_byte(f1 + 1, 1);
        }
    }
    let seller = person.club_job();
    if seller.is_null() {
        return;
    }
    let v2 = player_value_for_club(person, seller); // here without the minimum of 1
    let g = board_row_at(this, read_usize(seller.addr())) + board_record::SQUAD_INVESTMENT;
    write_f64(g, read_f64(g) - v2 as f64);
    let limit = fee_limit(seller);
    let ratio = if v2 == 0 { read_f32(rdata::F32_0_5) as f64 } else { taxa as f64 / v2 as f64 };
    if taxa <= limit {
        covered!("cpo_1 seller at the limit");
        or_byte(f2, 4); // the seller: fee within the limit
    } else if v2 > 0x3d090 {
        let m = if ratio < exe_f64(0x9689e0) {
            1
        } else if ratio < exe_f64(0x968968) {
            2
        } else if ratio < exe_f64(0x968950) {
            4
        } else if ratio < exe_f64(0x968938) {
            8
        } else {
            0x10
        };
        or_byte(f2, m);
    } else {
        let m = if ratio < exe_f64(0x968fd0) { 2 } else if ratio < exe_f64(0x968f10) { 4 } else { 8 };
        or_byte(f2, m);
    }
    if exe::club_favours_staff(person, seller) != 0 {
        or_byte(f2, 0x20);
        return;
    }
    if exe::club_dislikes_staff(person, seller) != 0 {
        or_byte(f2, 0x80);
        return;
    }
    if exe::staff_is_favoured_by(person, seller.chairman().addr()) != 0 {
        or_byte(f2, 0x20);
        return;
    }
    if exe::staff_is_disliked_by(person, seller.chairman().addr()) != 0 {
        or_byte(f2, 0x80);
    }
}

/// Clamps board confidence (+6) to 1..10000 and mood +0x16 to at least −10000.
pub(crate) unsafe fn clamp_board_row(e: usize) {
    let e = BoardRecord(e);
    if e.confidence() < 1 {
        e.set_confidence(1);
    }
    if e.confidence() > 10000 {
        e.set_confidence(10000);
    }
    if e.favourites_mood() <= -10000 {
        e.set_favourites_mood(-10000);
    }
}

/// Adjusts confidence (+6) and mood (+0x14) by the first set reason flag (bits 1..0x10) of a transfer deal.
pub(crate) unsafe fn apply_reason_flags(e: usize, f: u32) {
    let e = BoardRecord(e);
    if f & 1 != 0 {
        manager_manager_add_i16(e.addr() + board_record::CONFIDENCE, -0x7d);
        manager_manager_add_i16(e.addr() + board_record::TRANSFER_MOOD, -500);
    } else if f & 2 != 0 {
        manager_manager_add_i16(e.addr() + board_record::CONFIDENCE, -0x19);
        manager_manager_add_i16(e.addr() + board_record::TRANSFER_MOOD, -250);
    } else if f & 4 != 0 {
        // the fair deal: nothing
    } else if f & 8 != 0 {
        manager_manager_add_i16(e.addr() + board_record::CONFIDENCE, 0x19);
        manager_manager_add_i16(e.addr() + board_record::TRANSFER_MOOD, 0x19);
    } else if f & 0x10 != 0 {
        manager_manager_add_i16(e.addr() + board_record::CONFIDENCE, 0x4b);
        manager_manager_add_i16(e.addr() + board_record::TRANSFER_MOOD, 0x4b);
    }
}

/// Changes the board confidence (`board_record::CONFIDENCE`, 1..10000) and moods (`board_record::TRANSFER_MOOD`,
/// `board_record::FAVOURITES_MOOD`) of the buyer and the seller from the deal flags and removes the player from the
/// seller chairman's favoured people.
///
/// `MANAGER_MANAGER::apply_transfer_confidence_changes(transf)`: assesses the deal
/// (`exe::manager_manager_assess_transfer_deal`) with the buyer's flags in the slot of the argument itself and the
/// seller's in the slot of the pushed `ecx` (a loan offer, `.offer_type()` = 1: the pointer's bits stay, as in the
/// original) and changes the board row of the buyer (`.buyer_club_id()`) and, if the seller (the player's
/// `.club_job()`) is not a national team (`exe::database_is_national_team`) and does not have finances with
/// `.crisis_state()` = 1 (`exe::financetable_get_club_finance`), the seller's: ratio 1: −125/−500, 2: −25/−250, 8:
/// +25/+25, 0x10: +75/+75 (confidence `board_record::CONFIDENCE` and mood `board_record::TRANSFER_MOOD`;
/// `apply_reason_flags`); favoured and rejected on mood `board_record::FAVOURITES_MOOD` (and
/// `board_record::TRANSFER_MOOD`). Clamps the confidence to 1..10000 and `board_record::FAVOURITES_MOOD` to −10000 or
/// more (`clamp_board_row`); at the end the player is removed from the seller chairman's favoured people
/// (`exe::staff_remove_favoured_staff`).
/// Original: `MANAGER_MANAGER::apply_transfer_confidence_changes` (0x6978f0, manager_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn apply_transfer_confidence_changes(this: usize, transf: usize) {
    let transf = Offer(transf);
    let person = Staff(read_usize(G_STAFF_TABLE).wrapping_add(transf.target_staff_id().wrapping_mul(staff::SIZE)));
    let mut fc: usize = transf.addr(); // the argument’s slot
    let mut fv: usize = this; // the ecx pushed in the prologue
    exe::manager_manager_assess_transfer_deal(this, transf, &mut fc as *mut usize as usize, &mut fv as *mut usize as usize);
    let f = fc as u32;
    let e = board_row_at(this, transf.buyer_club_id());
    apply_reason_flags(e, f);
    if f & 0x20 != 0 {
        manager_manager_add_i16(e + board_record::TRANSFER_MOOD, -100);
        manager_manager_add_i16(e + board_record::FAVOURITES_MOOD, -100);
    } else if f & 0x40 != 0 {
        manager_manager_add_i16(e + board_record::FAVOURITES_MOOD, 0x32);
    } else if f & 0x80 != 0 {
        manager_manager_add_i16(e + board_record::FAVOURITES_MOOD, 0xa);
    } else if f & 0x100 != 0 {
        manager_manager_add_i16(e + board_record::FAVOURITES_MOOD, -100);
        manager_manager_add_i16(e + board_record::TRANSFER_MOOD, -0x32);
    }
    clamp_board_row(e);
    let seller = person.club_job();
    if seller.is_null() {
        return;
    }
    let id = read_usize(seller.addr());
    if exe::database_is_national_team(seller) != 0 {
        return;
    }
    let fin = Finance(exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), person.club_job()));
    if !fin.is_null() && fin.crisis_state() == 1 {
        return;
    }
    let f = fv as u32;
    let e = board_row_at(this, id);
    apply_reason_flags(e, f);
    if f & 0x20 != 0 {
        manager_manager_add_i16(e + board_record::FAVOURITES_MOOD, -100);
        manager_manager_add_i16(e + board_record::TRANSFER_MOOD, -0x19);
    } else if f & 0x40 != 0 {
        manager_manager_add_i16(e + board_record::FAVOURITES_MOOD, 0x32);
        manager_manager_add_i16(e + board_record::TRANSFER_MOOD, 0x19);
    } else if f & 0x80 != 0 {
        manager_manager_add_i16(e + board_record::FAVOURITES_MOOD, 0xa);
    } else if f & 0x100 != 0 {
        manager_manager_add_i16(e + board_record::FAVOURITES_MOOD, -100);
    }
    clamp_board_row(e);
    covered!("cpo_1 remove favourite");
    exe::staff_remove_favoured_staff(person.club_job().chairman().addr(), person); // the seller’s chairman
}

/// Removes the club from the ultimatum list and, when the manager has been in the job over 100 days, queues his
/// departure (reason 2, or 7 without a winless run in the table) and returns 1.
///
/// `MANAGER_MANAGER::resolve_board_ultimatum(club)`: walks the ultimatum list
/// (`manager_manager_fields::ULTIMATUM_LIST`, up to the count, which drops with each removal) and removes the club's
/// item (`exe::llist_remove`); if the manager (`.manager()`) has been in the job for more than 100 days
/// (`FN_DATE_DAYS_SINCE` since `.date_joined_club()`), it puts the departure in the list
/// (`manager_manager_fields::DEPARTURE_LIST`, `exe::llist_add_tail`, 6 bytes: 1, reason 2, or 7 if the club's row in
/// the division table (`exe::comp_stats_find_club_entry`) has no winless run (`.winless_run()` ≤ 0), and the club's
/// id), sets bit `board_record_flags::DEPARTURE_QUEUED` in `board_record::FLAGS` of the board row and returns 1.
/// Otherwise 0.
/// Original: `MANAGER_MANAGER::resolve_board_ultimatum` (0x6ac470, manager_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn resolve_board_ultimatum(this: usize, club: usize) -> u8 {
    let club = Club(club);
    let list = Llist(read_usize(this + manager_manager_fields::ULTIMATUM_LIST));
    if read_i16(list.addr()) == 0 {
        return 0;
    }
    list.set_cursor(list.head());
    let mut k = 0i32;
    while k < read_i16(read_usize(this + manager_manager_fields::ULTIMATUM_LIST)) as i32 {
        let item = exe::llist_get_next(read_usize(this + manager_manager_fields::ULTIMATUM_LIST));
        if item != 0 && read_usize(club.addr()) == read_usize(item) {
            exe::llist_remove(read_usize(this + manager_manager_fields::ULTIMATUM_LIST), item);
            let manager = club.manager();
            if !manager.is_null() && exe::date_days_since(G_CURRENT_DATE, (manager.date_joined_club()) as usize as u32, (manager.date_joined_club_leap_year()) as usize as u32) > 0x64 {
                covered!("cpo_1 ultimatum met");
                let mut reg = [0u8; 8];
                reg[0] = 1;
                reg[2..6].copy_from_slice(&(read_usize(club.addr()) as u32).to_le_bytes());
                let mut reason = 2u8;
                let division = club.division().addr();
                if division != 0 {
                    let comp = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(division).wrapping_mul(4)));
                    if comp != 0 && read_usize(comp + 8) != 0 {
                        let row = CompStatsClub(exe::comp_stats_find_club_entry(read_usize(comp + 8), club));
                        if row.is_null() || row.winless_run() <= 0 {
                            reason = 7;
                        }
                    }
                }
                reg[1] = reason;
                exe::llist_add_tail(read_usize(this + manager_manager_fields::DEPARTURE_LIST), reg.as_mut_ptr() as usize, 6);
                or_byte(board_row_at(this, read_usize(club.addr())) + board_record::FLAGS, board_record_flags::DEPARTURE_QUEUED);
                return 1;
            }
        }
        k += 1;
    }
    0
}

/// Walks the pending departure list and, at each club with a manager, carries out his departure and, when the club
/// opens a vacancy, the vacancy; removes each item from the list.
///
/// `MANAGER_MANAGER::process_pending_departures()`: walks the pending departures list
/// (`manager_manager_fields::DEPARTURE_LIST`, 6-byte items: two bytes and the club's id at +2) up to the count (which
/// drops with each removal: the original goes through only part of the items) and, for each item, with a manager at the
/// club (`.manager()`) calls the departure (`exe::manager_manager_remove_club_manager` with the two bytes pushed in the
/// integer registers: the upper bits are those of the id and of 145 × id) and, if the club opens a vacancy
/// (`exe::club_can_be_managed`), the vacancy (`exe::manager_manager_add_job_vacancy` with the departing manager);
/// removes the item (`exe::llist_remove`). Null item: the "Error" box.
/// Original: `MANAGER_MANAGER::process_pending_departures` (0x69b650, manager_manager.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn process_pending_departures(this: usize) {
    let list = Llist(read_usize(this + manager_manager_fields::DEPARTURE_LIST));
    if list.is_null() {
        return;
    }
    list.set_cursor(list.head());
    let mut k = 0i32;
    while k < read_i16(read_usize(this + manager_manager_fields::DEPARTURE_LIST)) as i32 {
        let item = exe::llist_get_next(read_usize(this + manager_manager_fields::DEPARTURE_LIST));
        if item == 0 {
            show_error_at(0x9f18a4, 0x2b70);
        } else {
            let id = read_usize(item + 2) as u32;
            let club = friendly_club_by_id(id as usize);
            let manager = club.manager().addr();
            if manager != 0 {
                let a = (id & 0xffff_ff00) | read_u8(item) as u32;
                let b = (id.wrapping_mul(145) & 0xffff_ff00) | read_u8(item + 1) as u32;
                exe::manager_manager_remove_club_manager(this, club, a, b, 0);
                if (exe::club_can_be_managed(club) as u32) as u8 != 0 {
                    covered!("cpo_4 open the slot");
                    exe::manager_manager_add_job_vacancy(this, club, manager); // the club’s slot
                }
            }
            exe::llist_remove(read_usize(this + manager_manager_fields::DEPARTURE_LIST), item);
        }
        k += 1;
    }
}

/// Maps a player-staff role to the role after leaving the staff post (12→5, 13→6, 15→8, staying at the club) or 0xb
/// (leaves).
pub(crate) fn role_after_leaving(f: u8) -> (u8, bool) {
    match f {
        0xc => (5, true),
        0xd => (6, true),
        0xf => (8, true),
        _ => (0xb, false),
    }
}

/// Clears the first of the 50 club squad slots (club::SQUAD) holding the person.
pub(crate) unsafe fn remove_from_squad(club: Club, person: Staff) {
    for k in 0..0x32usize {
        let v = club.addr() + club::SQUAD + 4 * k;
        if read_usize(v) == person.addr() {
            covered!("cpo_4 remove from squad");
            write_usize(v, 0); // the squad slot
            return;
        }
    }
}

/// Takes the player out of the club (or the national team): a player-manager job goes back to the staff job (otherwise
/// 0xb and he leaves), out of the lists, the media, the tactics and the 50 squad slots; the loan gets state 4.
///
/// `MANAGER_MANAGER::remove_player_from_squad(person, club)`: at the person's club (`.club_job()`): the player-manager
/// function (`.job_for_club()`) goes back to the manager one (12 → 5, 13 → 6, 15 → 8; `role_after_leaving`) or becomes
/// 0xb and the person leaves the club; removed from the lists (`exe::playerstats_archive_player_stats`), the media
/// (`exe::shortlist_manager_update_staff_status` with 4, 2), the club's two tactics
/// (`exe::tactics_remove_player_from_selection`) and the 50 squad slots (`remove_from_squad`). For the national team
/// (`.national_job()`, function `.job_for_nation()`) the same without `archive_player_stats` and `update_staff_status`.
/// At the end the loan contract (`exe::contract_manager_get_contract`) gets state `.set_transfer_status(4)`. Returns
/// the result of `exe::contract_manager_get_contract`.
/// Original: `MANAGER_MANAGER::remove_player_from_squad` (0x690730, manager_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn manager_manager_remove_player_from_squad(_this: usize, person: usize, club: usize) -> usize {
    let person = Staff(person);
    if person.club_job() == Club(club) {
        let (f, fica) = role_after_leaving(person.job_for_club());
        person.set_job_for_club(f);
        exe::playerstats_archive_player_stats(read_usize(G_PLAYER_STATS), person, -1, 0, 0, -1, 0);
        (exe::shortlist_manager_update_staff_status(read_usize(read_usize(G_TRANSFER_MANAGER) + transfer_manager_fields::SHORTLIST_MANAGER), person, club, 4, 2) as usize);
        exe::tactics_remove_player_from_selection(read_usize(G_TACTICS), club, person, 0, 1);
        exe::tactics_remove_player_from_selection(read_usize(G_TACTICS), club, person, 1, 1);
        remove_from_squad(Club(club), person);
        if !fica {
            person.set_club_job(Club(0));
        }
    }
    if person.national_job() == Club(club) {
        let (f, fica) = role_after_leaving(person.job_for_nation());
        person.set_job_for_nation(f);
        exe::tactics_remove_player_from_selection(read_usize(G_TACTICS), club, person, 0, 1);
        exe::tactics_remove_player_from_selection(read_usize(G_TACTICS), club, person, 1, 1);
        remove_from_squad(Club(club), person);
        if !fica {
            person.set_national_job(Club(0));
        }
    }
    let r = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), person));
    if !r.is_null() {
        r.set_transfer_status(4);
    }
    r.addr()
}

/// Clears every one of the n pointer slots at c + base that hold the person.
pub(crate) unsafe fn clear_slots_holding(c: Club, base: usize, n: usize, person: usize) {
    for k in 0..n {
        if read_usize(c.addr() + base + 4 * k) == person {
            write_usize(c.addr() + base + 4 * k, 0);
        }
    }
}

/// Body of `remove_staff_from_club` without the calls: person `pessoa` (fields in `s`) leaves his post at club `clube` (fields in `c`),
/// by role.
///
/// `s` and `c` are the records themselves in the copy and copies of them in the comparer's plan.
///
/// By role at the club (+0x3d, table 0x690af8): 1 chairman (+0xbf), 2 directors (+0xc3, 3), 5 manager (+0xcf, if it is
/// him), 6 assistant (+0xd3), 8 coaches (+0x19f, 5), 9 scouts (+0x1b3, 7), 10 physios (+0x1cf, 3): the slot zeroed and
/// +0x39 = 0; 12, 13, 15 (player-manager, player-assistant, player-coach): the slot zeroed and role 0xb (stays at the
/// club); others: +0x39 = 0 only if the low byte of the person pointer is 0 (the original routine's `[esp+0x10]`).
///
/// At the national team (+0x24, role +0x28, table 0x690b34) the same with +0x24, without the manager check, and the
/// other roles leave.
pub(crate) unsafe fn detach_staff_from_club(person: usize, s: Staff, club: usize, c: Club) {
    if s.club_job() == Club(club) {
        match (s.job_for_club() as i8 as i32).wrapping_sub(1) as u32 {
            0 => {
                write_usize(c.addr() + club::CHAIRMAN, 0);
                s.set_club_job(Club(0));
            }
            1 => {
                clear_slots_holding(c, 0xc3, 3, person);
                s.set_club_job(Club(0));
            }
            4 => {
                if c.manager().addr() == person {
                    covered!("cpo_4 remove the manager");
                    write_usize(c.addr() + club::MANAGER, 0); // the club’s manager
                }
                s.set_club_job(Club(0));
            }
            5 => {
                c.set_assistant_manager(0);
                s.set_club_job(Club(0));
            }
            7 => {
                clear_slots_holding(c, 0x19f, 5, person);
                s.set_club_job(Club(0));
            }
            8 => {
                clear_slots_holding(c, 0x1b3, 7, person);
                s.set_club_job(Club(0));
            }
            9 => {
                clear_slots_holding(c, 0x1cf, 3, person);
                s.set_club_job(Club(0));
            }
            11 => {
                s.set_job_for_club(job_kind::PLAYER);
                if c.manager().addr() == person {
                    write_usize(c.addr() + club::MANAGER, 0);
                }
            }
            12 => {
                s.set_job_for_club(job_kind::PLAYER);
                c.set_assistant_manager(0);
            }
            14 => {
                s.set_job_for_club(job_kind::PLAYER);
                clear_slots_holding(c, 0x19f, 5, person);
            }
            _ => {
                if person as u8 == 0 {
                    s.set_club_job(Club(0));
                }
            }
        }
    } else if s.national_job() == Club(club) {
        match (s.job_for_nation() as i8 as i32).wrapping_sub(1) as u32 {
            0 => write_usize(c.addr() + club::CHAIRMAN, 0),
            1 => clear_slots_holding(c, 0xc3, 3, person),
            4 => write_usize(c.addr() + club::MANAGER, 0),
            5 => c.set_assistant_manager(0),
            7 => clear_slots_holding(c, 0x19f, 5, person),
            8 => clear_slots_holding(c, 0x1b3, 7, person),
            9 => clear_slots_holding(c, 0x1cf, 3, person),
            11 => {
                write_usize(c.addr() + club::MANAGER, 0);
                s.set_job_for_nation(job_kind::PLAYER);
                return;
            }
            12 => {
                s.set_job_for_nation(job_kind::PLAYER);
                c.set_assistant_manager(0);
                return;
            }
            14 => {
                s.set_job_for_nation(job_kind::PLAYER);
                clear_slots_holding(c, 0x19f, 5, person);
                return;
            }
            _ => {}
        }
        s.set_national_job(Club(0));
    }
}

/// removes the contract and the person from the job slot at the club or national team (chairman, director, manager,
/// assistant, coach, scout, physio; the playing jobs become 0xb), from the media and, if human, from the human's lists
/// Original: `MANAGER_MANAGER::remove_staff_from_club` (0x690890, manager_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn remove_staff_from_club(_this: usize, person: usize, club: usize) -> usize {
    exe::contract_manager_remove_contract(read_usize(G_CONTRACT_MANAGER), person);
    detach_staff_from_club(person, Staff(person), club, Club(club));
    let mut r = exe::shortlist_manager_update_staff_status(read_usize(read_usize(G_TRANSFER_MANAGER) + transfer_manager_fields::SHORTLIST_MANAGER), person, club, 4, 2) as usize;
    if read_i32(person) >= read_i32(G_STAFF_COUNT).wrapping_sub(0x10) {
        (exe::scout_manager_sync_scout_lists(read_usize(G_SCOUT_MANAGER), person) as usize);
        r = exe::training_manager_reassign_squad_schedules(read_usize(G_TRAINING_MANAGER), person, club, 0);
    }
    r
}

/// Comparador: a saída e a vaga (`remove_club_manager`, `add_job_vacancy`) demitem e contratam: roda só a original.
// simplification: no plan (0x69ba50 may append outputs to the same list); the proof is the career and Rust mode.

/// Copies the fields `manager_manager_remove_player_from_squad`/`remove_staff_from_club` write: the person's national job (6 bytes), club job (5 bytes) and the club's
/// staff slots (+0xbf..+0x1db).
pub(crate) unsafe fn staff_job_fields(person: Staff, club: Club) -> ([u8; 6], [u8; 5], [u8; 0x11c]) {
    let mut a = [0u8; 6];
    let mut b = [0u8; 5];
    let mut c = [0u8; 0x11c];
    core::ptr::copy_nonoverlapping((person.addr() + staff::NATIONAL_JOB) as *const u8, a.as_mut_ptr(), 6);
    core::ptr::copy_nonoverlapping((person.addr() + staff::CLUB_JOB) as *const u8, b.as_mut_ptr(), 5);
    core::ptr::copy_nonoverlapping((club.addr() + club::CHAIRMAN) as *const u8, c.as_mut_ptr(), 0x11c);
    (a, b, c)
}

/// Walks the vacancy list [this+8] looking for the club's id at +0xe; None if the list ends before its count.
pub(crate) unsafe fn club_has_vacancy(this: usize, club: usize) -> Option<bool> {
    let l = Llist(read_usize(this + manager_manager_fields::VACANCY_LIST));
    l.set_cursor(l.head());
    if read_i16(read_usize(this + manager_manager_fields::VACANCY_LIST)) <= 0 {
        return Some(false);
    }
    let mut j = 0i32;
    loop {
        let v = exe::llist_get_next(read_usize(this + manager_manager_fields::VACANCY_LIST));
        if v == 0 {
            return None;
        }
        if read_usize(v + job_vacancy::CLUB_ID) == read_usize(club) {
            covered!("cpn_5 club with open vacancy");
            return Some(true);
        }
        j += 1;
        if j >= read_i16(read_usize(this + manager_manager_fields::VACANCY_LIST)) as i32 {
            return Some(false);
        }
    }
}

/// True if the club has no exe::database_get_club_manager(`database_get_club_manager`), can be managed, is not the B team of a linked pair and has no open
/// vacancy yet; None if the vacancy list ends early.
pub(crate) unsafe fn club_needs_vacancy(this: usize, club: usize, eh_a: &mut u32) -> Option<bool> {
    if exe::database_get_club_manager(club) != 0 || exe::club_can_be_managed(club) == 0 {
        return Some(false);
    }
    if exe::club_get_linked_b_team(club, eh_a, 1) != 0 && *eh_a == 0 {
        return Some(false);
    }
    club_has_vacancy(this, club).map(|tem| !tem)
}

/// Gives each club without a manager (`exe::database_get_club_manager`), manageable (`exe::club_can_be_managed`), not a
/// B team and without an entry in the `manager_manager_fields::VACANCY_LIST` list the "Job not found" note and a job
/// vacancy (`exe::manager_manager_add_job_vacancy`).
///
/// `MANAGER_MANAGER::fix_missing_jobs()`: each club (`[G_CLUB_TABLE]`, 0x245 bytes, `[G_CLUB_COUNT]` re-read on each
/// iteration) without a manager, manageable, not a B team and without an open vacancy (`club_needs_vacancy`) gets the
/// note "Job not found for %s" (name +4) in `[0xde1f64]` and the vacancy (`exe::manager_manager_add_job_vacancy`
/// without a previous manager). A list that ends before the count: the "Error" box (line 0x311e) and the routine ends.
/// No SEH, no random draw; the partner local starts at 0 only once and is read only when `exe::club_get_linked_b_team`
/// finds it (and writes it).
/// Original: `MANAGER_MANAGER::fix_missing_jobs` (0x69d060, manager_manager.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn fix_missing_jobs(this: usize) {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let mut eh_a = 0u32;
    let mut club = read_usize(G_CLUB_TABLE);
    let mut i = 0i32;
    while i < read_usize(G_CLUB_COUNT) as i32 {
        match club_needs_vacancy(this, club, &mut eh_a) {
            None => {
                show_error_at(0x9f18a4, 0x311e);
                return;
            }
            Some(true) => {
                format(0xde1f64, 0xa05298, club + 4);
                exe::manager_manager_add_job_vacancy(this, club, 0);
            }
            Some(false) => {}
        }
        i += 1;
        club += club::SIZE;
    }
}

/// Returns the highest manager score (`exe::score_staff_for_job`, job 5) among the available staff accepted for the
/// club (nation, continent, reputation, contract, `exe::manager_manager_is_staff_suitable_for_job`), ignoring the
/// current manager; nobody: −10000.
///
/// `MANAGER_MANAGER::best_manager_candidate_score(club, current)`: the highest manager score
/// (`exe::score_staff_for_job` with 0 and job 5) among the people (`[G_STAFF_TABLE]`, 0x6e bytes, `[G_STAFF_COUNT]`
/// re-read on each iteration) with a staff record (`staff::NON_PLAYER`), of the last 16 only the human ones
/// (`staff::CLASSIFICATION`), who are not `current`, without flags 0x30 in byte `staff_state::STATUS_FLAGS` of the
/// 79-byte record (`[G_STAFF_STATE_TABLE]`), without a loan contract (`exe::contract_manager_get_loan_contract`), who
/// are not candidates for another vacancy (`exe::manager_manager_is_vacancy_candidate`), with a nation
/// (`staff::NATION`) and the club with a nation (`.nation()`): from another nation only with reputation
/// (`staff::NON_PLAYER` → `non_player::CURRENT_REPUTATION`) from 3750; from another continent (`nation::REGION`) with
/// `staff::NON_PLAYER` → `non_player::WORLD_REPUTATION` from 4250 and the club with reputation (`.reputation()`) from
/// 4750; a player (`staff::PLAYER`) with a club (`staff::CLUB_JOB`) and job (`staff::JOB_FOR_CLUB`) 11, 12, 13 or 15
/// cannot have a contract with the club (`exe::contract_manager_get_contract_for_club`) flagged in
/// `.leaving_on_bosman()`; and accepted for the job (`exe::manager_manager_is_staff_suitable_for_job` with 3.5).
/// Nobody: −10000.
/// Original: `MANAGER_MANAGER::best_manager_candidate_score` (0x69c540, manager_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn best_manager_candidate_score(this: usize, club: usize, current: usize) -> i32 {
    let club = Club(club);
    let mut best = -10000i32;
    let mut p = read_usize(G_STAFF_TABLE);
    let mut i = 0i32;
    while i < read_usize(G_STAFF_COUNT) as i32 {
        let reg79 = read_usize(G_STAFF_STATE_TABLE).wrapping_add((i as usize).wrapping_mul(staff_state::SIZE));
        let skips = 'f: {
            if read_usize(p + staff::NON_PLAYER) == 0 {
                break 'f true;
            }
            if i >= (read_usize(G_STAFF_COUNT) as i32).wrapping_sub(0x10) && read_u8(p + staff::CLASSIFICATION) == 0 {
                break 'f true;
            }
            if current == p {
                covered!("cpn_5 the current manager is left out");
                break 'f true; // the current one is left out
            }
            if read_u8(reg79 + staff_state::STATUS_FLAGS) & 0x30 != 0 || exe::contract_manager_get_loan_contract(read_usize(G_CONTRACT_MANAGER), p) != 0 || exe::manager_manager_is_vacancy_candidate(this, p) != 0 {
                break 'f true;
            }
            let (nc, np) = (club.nation().addr(), read_usize(p + staff::NATION));
            if nc != np && read_i16(read_usize(p + staff::NON_PLAYER) + non_player::CURRENT_REPUTATION) < 0xea6 {
                break 'f true;
            }
            if nc == 0 || np == 0 {
                break 'f true;
            }
            if read_u8(nc + nation::REGION) != read_u8(np + nation::REGION) && (read_i16(read_usize(p + staff::NON_PLAYER) + non_player::WORLD_REPUTATION) < 0x109a || club.reputation() < 0x128e) {
                break 'f true;
            }
            if read_usize(p + staff::PLAYER) != 0 && read_usize(p + staff::CLUB_JOB) != 0 && matches!(read_u8(p + staff::JOB_FOR_CLUB), 0xb | 0xc | 0xd | 0xf) {
                let k = Contract(exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), p, club));
                if !k.is_null() && k.leaving_on_bosman() != 0 {
                    break 'f true;
                }
            }
            exe::manager_manager_is_staff_suitable_for_job(this, p, club, 3.5) == 0
        };
        if !skips {
            let s = exe::score_staff_for_job(this, p, club, 0, 5);
            if s > best {
                best = s;
            }
        }
        i += 1;
        p += 0x6e;
    }
    best
}

/// The read-only scheduling plan: the offer date and the bucket list address.
///
/// The offer date is today ([0xae2c90]) + `dias` (with `dias` ≥ 50 the original shows the error box and uses 10), with
/// the slice [0xae2c88] + `fatias` brought into 0..2 (every 3 slices, one more day). The list bucket: the days since
/// the base date (+0x84d; with no year at +0x84f yet, today) plus the start (+0x84c), modulo 50 from 50 to 127, × 3 +
/// slice, in the 14-byte lists starting at +0x18. Read-only (`exe::date_add_days` and `exe::date_days_since` on copies of the dates).
pub(crate) unsafe fn vacancy_schedule_plan(this: TransferManagerFields, days: u8, slices: u8) -> ([u8; 8], i8, usize) {
    let base = if this.current_year() == 0 { G_CURRENT_DATE } else { this.addr() + transfer_manager_fields::CURRENT_DATE };
    let (b0, b1) = (read_usize(base) as u32, read_usize(base + 4) as u32);
    let days = if days as i8 >= 0x32 { 10 } else { days as i8 };
    let (mut data, mut tmp) = ([0u8; 8], [0u8; 8]);
    // 0x54c5b0 reads the days as 16 bits (the original pushes the register with garbage in the high bits)
    let r = exe::date_add_days(G_CURRENT_DATE, (tmp.as_mut_ptr()) as usize, days as i16 as i32 as u32);
    core::ptr::copy_nonoverlapping(r as *const u8, data.as_mut_ptr(), 8);
    let mut slice = read_u8(G_CURRENT_DAY_SLICE).wrapping_add(slices) as i8;
    while slice > 2 {
        slice -= 3;
        let r = exe::date_add_days(data.as_ptr() as usize, (tmp.as_mut_ptr()) as usize, 1);
        core::ptr::copy_nonoverlapping(r as *const u8, data.as_mut_ptr(), 8);
    }
    let mut c = (exe::date_days_since(data.as_ptr() as usize, b0, b1) as u8).wrapping_add(this.day_cursor());
    if c as i8 >= 0x32 {
        let q = (c as i8 as i32 as u32) / 50;
        c = c.wrapping_add((q as u8).wrapping_mul(0xce)); // − 50 × q
    }
    let balde = (c as i8 as i32).wrapping_mul(3).wrapping_add(slice as i32);
    (data, slice, this.addr().wrapping_add(0x18).wrapping_add((balde as usize).wrapping_mul(14)))
}

/// For a played match, changes the board confidence of each side with a manager
/// (`exe::manager_manager_update_board_after_match`, home with 1).
///
/// `MANAGER_MANAGER::update_board_after_fixture(fixture, game)`: a played fixture (+0 ≠ −2, `.home_side_mark()` ≠ 0):
/// the board confidence of each side with a manager (`.manager()`) changes through
/// `exe::manager_manager_update_board_after_match` (home +0x1c with 1, away +0x20 with 0).
/// Original: `MANAGER_MANAGER::update_board_after_fixture` (0x6951e0, manager_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn update_board_after_fixture(this: usize, fixture: usize, game: usize) {
    let fixture = Fixture(fixture);
    if read_usize(fixture.addr()) as i32 == -2 || fixture.home_side_mark() == 0 {
        return;
    }
    for (lado, mandante) in [(0x1cusize, 1u32), (0x20, 0)] {
        let club = Club(read_usize(fixture.addr() + lado));
        if club.manager().addr() != 0 { // the side has a manager
            exe::manager_manager_update_board_after_match(this, club, fixture, game, mandante);
        }
    }
}

/// Returns the disciplinary tribunal for competition id (vector [this] indexed by byte +0x50 of the competition
/// object), if any.
pub(crate) unsafe fn comp_tribunal(this: usize, id: i32) -> Option<usize> {
    let comp = League(read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add((id as usize).wrapping_mul(4))));
    if comp.is_null() {
        return None;
    }
    let t = comp.discipline_rule() as i8;
    if t == -1 {
        return None;
    }
    let b = read_usize(read_usize(this).wrapping_add((t as isize as usize).wrapping_mul(4)));
    (b != 0).then_some(b)
}

/// Binary-searches the shortlist (+4, count +0x14, sorted by club) and returns all 8-read_i8(person + club) entries of
/// club id.
pub(crate) unsafe fn club_shortlist_entries(this: usize, id: u32) -> Vec<usize> {
    let key = [u32::MAX, id];
    let (base, n) = (read_usize(this + shortlist_manager_fields::PAIRS_BY_CLUB), read_usize(this + shortlist_manager_fields::PAIRS_BY_CLUB_COUNT));
    let mut e = exe::crt_bsearch((key.as_ptr()) as usize, base, (n) as u32, (8) as u32, 0x844740);
    if e == 0 {
        return Vec::new();
    }
    let last = base.wrapping_add(n.wrapping_mul(8)).wrapping_sub(8);
    while e > base && read_usize(e - 4) as u32 == id {
        e -= 8; // the previous one also belongs to the club
    }
    let mut v = Vec::new();
    if read_usize(e + 4) as u32 != id {
        return v;
    }
    loop {
        v.push(e);
        if e == last || read_usize(e + shortlist_pair::CLUB_ID + 8) as u32 != id {
            return v;
        }
        e += 8;
    }
}

/// True if the offer belongs to the club (+8) or is flagged (+0x2d = 1), and its status (+0x2c) is 2.
pub(crate) unsafe fn is_offer_eligible(o: Offer, club: u32) -> bool {
    (o.buyer_club_id() as u32 == club || o.player_aware() == 1) && o.status() == offer_status::REJECTED
}

/// Is the person in the list (iterator `next_list_item`, restored at the end)?
///
/// Is the person `*p` in the list `[this+8]` (count word at +0, iterator `next_list_item` with the position at +0xa, restarted
/// from +2 and restored at the end)? A list that ends before the count: the original (error box). Returns only `al`.
/// Original: `person_in_list` (0x69fc60, manager_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn person_in_list(t: usize, p: *const i32) -> u8 {
    let list = Llist(read_usize(t + 8));
    let stored = list.cursor() as u32;
    list.set_cursor(((list.head() as u32)) as usize);
    let mut i = 0i32;
    while i < read_i16(list.addr()) as i32 {
        let e = exe::llist_get_next(list);
        if e == 0 {
            list.set_cursor((stored) as usize);
            return original_fn::<unsafe extern "thiscall" fn(usize, *const i32) -> u8>(IDX_PERSON_IN_LIST)(t, p);
        }
        if read_i32(e + 6) == *p {
            list.set_cursor((stored) as usize);
            return 1;
        }
        i += 1;
    }
    list.set_cursor((stored) as usize);
    0
}

compare_value!(compare_person_in_list, person_in_list, IDX_PERSON_IN_LIST, (p: *const i32) -> u8);
compare_value!(compare_contract_record, contract_record, IDX_CONTRACT_RECORD, (p: *const i32) -> usize);

/// Decides whether the person is a realistic candidate for the club's manager job: vetoes and accepts by reputation,
/// nation, contract and time in the job; otherwise their interest in the job against the minimum score (double).
///
/// `MANAGER_MANAGER::is_staff_suitable_for_job(person, club, rating_minimum)`: without a non-player record
/// (`.non_player()`) or without a club: 0. The vetoes (0): other-nation reputation (`non_player::WORLD_REPUTATION`)
/// above 4250 from another nation with weight (`nation::LEAGUE_STANDARD`) below 10; domestic
/// (`non_player::HOME_REPUTATION`) above 6250, a manager (`staff::JOB_FOR_CLUB` = 5) from another nation and a club
/// below 5750; a national team that is one of the three clubs they dislike (`.staff_preferences()`,
/// `preferences::STAFF_DISLIKED_CLUBS1`..`preferences::STAFF_DISLIKED_CLUBS3`); a player (0xb) with a rating
/// (`.player()`, `.current_reputation()`) above 4750 at a club below 6750, with a contract
/// (`exe::contract_manager_get_contract`) of band 1..3 (`contract::SQUAD_STATUS` >> 4), less than 180 days
/// (`contract::DATE_STARTED`), `contract::MATCHES_LEFT_OUT` above 1, `contract::UNHAPPY_FLAG` zero, younger than 35
/// (`staff::AGE`) with `contract::HAPPINESS` positive, or from another nation at a club below 7250 that is not their
/// own; a club manager in a better division (`club::DIVISION` `.reputation()`), or a stronger one, or recently
/// appointed (days since `.date_joined_club()` below (`staff::LOYALTY` + 21) × 10) at a club above 5250; a national
/// team manager (`.national_job()`, `staff::JOB_FOR_NATION` = 5); a club below 2250 (with reputation
/// `non_player::CURRENT_REPUTATION` above 4750 or without a record); `staff::INT_APPS` above 75 at a club below 3250; a
/// non-player with retirement pending (`exe::player_regen_is_retirement_pending`). The yeses (1): a club below 2500 of
/// their nation, domestic below 1500, without a club or national team; their nation's national team with reputation
/// below 6750, without a club, or with their club weaker (below 7750 and 750 below the national team) for a while; a
/// club above 7750 with the perceived reputation (`perceived_club_reputation`) above their club's + 25 ×
/// `staff::LOYALTY` + 250. Otherwise, their interest in the job (`exe::staff_interest_in_job_at_club`, float) not below
/// `rating_minimum`.
/// Original: `MANAGER_MANAGER::is_staff_suitable_for_job` (0x69f810, manager_manager.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn is_staff_suitable_for_job(_this: usize, person: usize, club: usize, rating_minimum: f64) -> u8 {
    let person = Staff(person);
    let club = Club(club);
    let sheet = (person.non_player().addr() as u32) as usize;
    if sheet == 0 || club.is_null() {
        return 0;
    }
    let rep = |c: usize| read_i16(c + club::REPUTATION);
    if read_i16(sheet + non_player::WORLD_REPUTATION) > 0x109a {
        let (n1, n2) = ((person.nation().addr() as u32) as usize, (club.nation().addr() as u32) as usize);
        if n1 != 0 && n2 != 0 && n1 != n2 && (read_u8_volatile(n2 + nation::LEAGUE_STANDARD) as i8) < 0xa {
            return 0;
        }
    }
    if read_i16(sheet + non_player::HOME_REPUTATION) > 0x186a && (person.nation().addr() as u32) != (club.nation().addr() as u32) && read_u8_volatile(person.addr() + staff::JOB_FOR_CLUB) == 5 && rep(club.addr()) < 0x1676 {
        return 0;
    }
    if exe::database_is_national_team(club) != 0 {
        let s = (person.staff_preferences() as u32) as usize;
        if s != 0 && (read_u32(s + preferences::STAFF_DISLIKED_CLUBS1) as usize == club.addr() || read_u32(s + preferences::STAFF_DISLIKED_CLUBS2) as usize == club.addr() || read_u32(s + preferences::STAFF_DISLIKED_CLUBS3) as usize == club.addr()) {
            return 0;
        }
    }
    let team = |p: Staff| exe::date_days_since(G_CURRENT_DATE, p.date_joined_club() as u32 , p.date_joined_club_leap_year() as u32);
    let limit = |p: Staff| (read_u8_volatile(p.addr() + staff::LOYALTY) as i8 as i32 + 0x15) * 10;
    if read_u8_volatile(person.addr() + staff::JOB_FOR_CLUB) == 0xb {
        let player = PlayerSheet((person.player().addr() as u32) as usize);
        if !player.is_null() && player.current_reputation() > 0x128e && rep(club.addr()) < 0x1a5e {
            let c = exe::contract_manager_get_contract(read_u32(G_CONTRACT_MANAGER) as usize, person);
            if c != 0 {
                let band = read_u8_volatile(c + contract::SQUAD_STATUS) >> 4;
                if band != 0 && band <= 3 {
                    return 0;
                }
                if exe::date_days_since(G_CURRENT_DATE, read_u32(c + contract::DATE_STARTED), read_u32(c + contract::DATE_STARTED_LEAP_YEAR)) < 0xb4 {
                    return 0;
                }
                if read_u8_volatile(c + contract::MATCHES_LEFT_OUT) as i8 > 1 || read_u8_volatile(c + contract::UNHAPPY_FLAG) == 0 {
                    return 0;
                }
                if (read_u8_volatile(person.addr() + staff::AGE) as i8) < 0x23 && read_u8_volatile(c + contract::HAPPINESS) as i8 > 0 {
                    return 0;
                }
                if (person.nation().addr() as u32) != (club.nation().addr() as u32) && rep(club.addr()) < 0x1c52 && (person.club_job().addr() as u32) as usize != club.addr() {
                    return 0;
                }
            }
        }
    }
    if read_u8_volatile(person.addr() + staff::JOB_FOR_CLUB) == 5 {
        let league = Competition((club.division().addr() as u32) as usize);
        let current = (person.club_job().addr() as u32) as usize;
        if !league.is_null() && current != 0 && read_u32(current + club::DIVISION) != 0 {
            let liga2 = Competition(read_u32(current + club::DIVISION) as usize);
            if league.reputation() < liga2.reputation() {
                return 0; // his club is in a better division
            }
            if league == liga2 && (rep(club.addr()) as i32 - 0x4e2) < rep(current) as i32 {
                return 0;
            }
            if rep(current) > rep(club.addr()) {
                return 0;
            }
            if team(person) < limit(person) && rep((person.club_job().addr() as u32) as usize) > 0x1482 {
                return 0;
            }
        }
    }
    if (person.national_job().addr() as u32) != 0 && read_u8_volatile(person.addr() + staff::JOB_FOR_NATION) == 5 {
        return 0;
    }
    let sheet = (person.non_player().addr() as u32) as usize;
    if (sheet == 0 || read_i16(sheet + non_player::CURRENT_REPUTATION) > 0x128e) && rep(club.addr()) < 0x8ca {
        return 0;
    }
    if read_u8_volatile(person.addr() + staff::INT_APPS) > 0x4b && rep(club.addr()) < 0xcb2 {
        return 0;
    }
    if (person.player().addr() as u32) == 0 {
        if exe::player_regen_is_retirement_pending(read_u32(G_PLAYER_REGEN) as usize, person) != 0 {
            return 0;
        }
    }
    if rep(club.addr()) < 0x9c4 && (person.nation().addr() as u32) == (club.nation().addr() as u32) && read_i16((person.non_player().addr() as u32) as usize + 8) < 0x5dc
        && (person.club_job().addr() as u32) == 0 && (person.national_job().addr() as u32) == 0 {
        return 1;
    }
    if exe::database_is_national_team(club) != 0 && (person.nation().addr() as u32) == (club.nation().addr() as u32) {
        if read_i16((person.non_player().addr() as u32) as usize + non_player::CURRENT_REPUTATION) < 0x1a5e {
            return 1;
        }
        if (person.club_job().addr() as u32) == 0 {
            return 1;
        }
        if team(person) > limit(person) {
            let r = rep((person.club_job().addr() as u32) as usize);
            if r < 0x1e46 && (r as i32 + 0x2ee) < rep(club.addr()) as i32 {
                return 1;
            }
        }
    }
    if rep(club.addr()) > 0x1e46 {
        let vista = perceived_club_reputation(club);
        let current = (person.club_job().addr() as u32) as usize;
        if current != 0 {
            let dele = perceived_club_reputation(Club(current));
            let ceiling = (read_u8_volatile(person.addr() + staff::LOYALTY) as i8 as i16).wrapping_mul(25) as i32 + dele as i32 + 0xfa;
            if vista as i32 > ceiling {
                return 1;
            }
        }
    }
    let rating = exe::staff_interest_in_job_at_club(read_u32(read_u32(G_TRANSFER_MANAGER) as usize + transfer_manager_fields::SHORTLIST_MANAGER) as usize, person, club, 0, 5, 0);
    if rating >= rating_minimum { 1 } else { 0 } // fcomp: below (or unordered) is 0
}

/// Removes from the list the first node whose item has the same bytes as the data (head, tail, count and cursor fixed)
/// and frees it with its item; 1 when removed.
///
/// `LLIST::remove(data)`: the first node with the bytes of `data` (find_list_node_by_bytes) leaves the list: the head
/// (+2) or the previous node's next, the tail (+6) or the next node's previous, the count (+0, word) − 1, the cursor
/// (+0xa) moves on if it was on it; the item (`exe::drop_llist_node`) and the node (`exe::crt_operator_delete`) are freed. Returns 1; not found, 0.
/// Used by ~77 callers (lists of offers, absences, ultimatums, manager vacancies, friendlies...).
/// Original: `LLIST::remove` (0x68d1f0, manager_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn manager_manager_remove(this: usize, given: usize) -> u32 {
    let this = Llist(this);
    let in_the = find_list_node_by_bytes(this, given);
    if in_the == 0 {
        return 0;
    }
    covered!("cqb_10 removed");
    let (prox, ant) = (read_usize(in_the + llist_node::NEXT), read_usize(in_the + llist_node::PREV));
    if in_the == this.head() {
        this.set_head(prox);
    } else {
        write_usize(ant + llist_node::NEXT, prox);
    }
    if in_the == this.tail() {
        covered!("cqb_10 tail");
        this.set_tail(ant); // cqb10: the tail goes back to the previous one
    } else {
        write_usize(prox + llist_node::PREV, ant);
    }
    let count = read_i16(this.addr());
    write_i16(this.addr(), count.wrapping_sub(1));
    if in_the == this.cursor() {
        covered!("cqb_10 cursor");
        this.set_cursor(read_usize(in_the + llist_node::NEXT)); // cqb10: o cursor anda
    }
    exe::drop_llist_node(in_the);
    exe::crt_operator_delete(in_the);
    1
}

/// frees every node and item of the list and zeroes the count, head, tail and cursor
/// Original: `LLIST::clear` (0x68d080, manager_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn manager_manager_clear(this: usize) {
    let this = Llist(this);
    while this.head() != 0 {
        let in_the = this.head();
        let next = read_usize(in_the + llist_node::NEXT);
        exe::drop_llist_node(in_the);
        exe::crt_operator_delete(in_the);
        this.set_head(next);
    }
    write_u16(this.addr(), 0);
    this.set_head(0);
    this.set_tail(0);
    this.set_cursor(0);
}

/// the item of the cursor node, without moving; with no cursor, 0
/// Original: `LLIST::get_current` (0x68d2e0, manager_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn get_current(this: usize) -> usize {
    let this = Llist(this);
    let in_the = this.cursor();
    if in_the == 0 {
        return 0;
    }
    read_usize(in_the)
}

/// Returns the first run (types 1, 3, 4, 2) of more than 1 match with the last one under 10 days ago, from the club's
/// row in the league statistics; otherwise 5 and 0.
///
/// `MANAGER_MANAGER::get_current_run(club, kind, size)`: the club's current run in the league (`.division()`; the
/// competition by id in `[G_COMP_OBJECT_TABLE]`, statistics at +8, the club's row via
/// `exe::comp_stats_find_club_entry`): the first of more than 1 match whose last one was less than 10 days ago
/// (`exe::date_days_since` from the match date), in the order +0x19 (type 1, date +0x25), +0x2d (3, +0x39), +0x55 (4,
/// +0x61) and +0x41 (2, +0x4d): the type and the length (byte). None (or without league, competition, statistics or
/// row): type 5 and length 0. `this` is not used.
/// Original: `MANAGER_MANAGER::get_current_run` (0x690c30, manager_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn get_current_run(_this: usize, club: usize, kind: *mut u8, size: *mut u8) {
    let club = Club(club);
    let league = (club.division().addr() as u32) as usize;
    let comp = if league != 0 { read_u32((read_u32(G_COMP_OBJECT_TABLE) as usize).wrapping_add((read_u32(league) as usize).wrapping_mul(4))) as usize } else { 0 };
    let stats = if comp != 0 { read_u32(comp + 8) as usize } else { 0 };
    if stats != 0 {
        let e = exe::comp_stats_find_club_entry(stats, club);
        if e != 0 {
            for (n, data, t) in [(0x19usize, 0x25usize, 1u8), (0x2d, 0x39, 3), (0x55, 0x61, 4), (0x41, 0x4d, 2)] {
                if read_u32(e + n) as i32 > 1 && exe::date_days_since(G_CURRENT_DATE, read_u32(e + data), read_u32(e + data + 4)) < 10 { // less than 10 days ago
                    *kind = t;
                    *size = read_u8(e + n);
                    return;
                }
            }
        }
    }
    *kind = 5; // nenhuma
    *size = 0;
}

/// On a one-in-four random draw, posts the 0x1776 news of a rumour linking a manager with another club.
///
/// SEH frame with no destructor. `MANAGER_MANAGER::post_job_rumour_news(person, club)`: without a person or without
/// their club (`.club_job()`), nothing; a random(4) = 0, nothing; before the news date (`[G_GAME_START_DATE]`),
/// nothing. News 0x1776 (local, `exe::new_newsitem`): 0..3 the person, 4 = the job (`.job_for_club()`), 5 = the id of
/// their club, 6 = the id of the rumoured club, 7 = random(4); to the recipients
/// (`exe::transfer_manager_collect_player_news_recipients` of the transfer manager `[G_TRANSFER_MANAGER]`, with 0x52,
/// up to 30; without a manager, none) through the delivery to the manager (`exe::news_send_to_club_manager`).
/// Original: `post_job_rumour_news` (0x69fd50, manager_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn post_job_rumour_news(_this: usize, person: usize, club: usize) {
    let person = Staff(person);
    if person.is_null() || person.club_job().is_null() || exe::utils_random(4) == 0 {
        return;
    }
    let day = read_i16(G_GAME_START_DATE);
    let year = read_u16(G_GAME_START_DATE_YEAR);
    let today_year = read_u16(G_CURRENT_DATE_YEAR);
    if year > today_year || (year == today_year && day > read_i16(G_CURRENT_DATE)) {
        return;
    }
    covered!("cpv_4 manager rumour");
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0x1776, 0);
    exe::newsitem_set_slot(n, 0, read_usize(person.addr()) as u32);
    exe::newsitem_set_slot(n, 1, read_usize(person.first_name() + names::ID) as u32);
    exe::newsitem_set_slot(n, 2, read_usize(person.second_name() + names::ID) as u32);
    exe::newsitem_set_slot(n, 3, read_usize(person.common_name() + names::ID) as u32);
    exe::newsitem_set_slot(n, 4, person.job_for_club() as i8 as i32 as u32);
    exe::newsitem_set_slot(n, 5, read_usize(person.club_job().addr()) as u32);
    exe::newsitem_set_slot(n, 6, read_usize(club) as u32);
    let s = exe::utils_random(4) as i16 as i32; // cpv4: the rumour variant
    exe::newsitem_set_slot(n, 7, s as u32);
    let mut clubs = [0usize; 30];
    let manager = read_usize(G_TRANSFER_MANAGER);
    let q = if manager != 0 { exe::transfer_manager_collect_player_news_recipients(manager, person, 0x52, clubs.as_mut_ptr(), 0x1e) as i8  } else { 0 };
    for &c in clubs.iter().take(if q > 0 { q as usize } else { 0 }) {
        exe::news_send_to_club_manager(read_usize(G_NEWS), n, c);
    }
}

/// `MANAGER_MANAGER::weekly_board_confidence_update()`: every Sunday (daily_update 0x68f9c0), for each club with a
/// manager and his non-player sheet that can be managed (`exe::club_can_be_managed`; a national team only with a
/// nation on a continent), the board's, supporters' and other confidences in the manager, the board stories in the
/// media and the manager's reputations.
///
/// Per club (row = [this] + loop index × 0x49): `decay_board_week`; the national team's finals stories
/// (`national_finals_stories`); otherwise the league week (`league_board_week`, which may post one story and moves the
/// confidence by the form, the projected position against the target and the chairman); then
/// `board_week_floors_and_reputation` (the continental floor, the 0..10000 clamps, the chairman's and the club's likes
/// and dislikes, the manager's reputations and the wealth band). Defect of the original (`docs/DEFEITOS_DO_ORIGINAL.md`,
/// approved to fix only in the all-Rust exe): the pre-season stories read the league size N ([esp+0x2b]) before it is
/// written (0x6984ad, 0x6986ff, 0x6987d6): the first club reads stack garbage, the next ones the N of the previous
/// club's league. Ported faithfully: N survives from club to club; the first one gets the last N of the previous call
/// (`BOARD_WEEK_LEAGUE_SIZE`; the comparer passes the byte the original really reads).
/// Original: `MANAGER_MANAGER::weekly_board_confidence_update` (0x697c40, manager_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn weekly_board_confidence_update(this: usize) {
    let n = board_week_run(this, BOARD_WEEK_LEAGUE_SIZE.load(Relaxed) as i8);
    BOARD_WEEK_LEAGUE_SIZE.store(n as u8, Relaxed);
}

/// Comparer for `weekly_board_confidence_update`: the routine posts stories (news, the log, the manager's answer) and
/// cannot run twice. The original runs as the game's, with a recorder on the story entry (the `jmp` of the hook at
/// 0x74b060 pointed at `board_story_recorder` meanwhile) that tapes, for every story, its arguments, the compared state
/// before it and what it changed; the compared state is the board table ([this], clubs × 0x49), the random generator and
/// every club's chairman patience and manager reputations. Then the state before is put back and the copy runs from
/// it with the league size the original found on its stack: each story of the copy must be the next one taped, with
/// the same arguments and the same state before it, and gets the taped changes; at the end every story must have been
/// replayed and the state must equal the original's (which stays).
pub(crate) unsafe extern "thiscall" fn compare_weekly_board_confidence_update(this: usize) {
    let o = original_fn::<usize>(IDX_WEEKLY_BOARD_CONFIDENCE_UPDATE);
    let entry = board_week_fns::MEDIA_STORY_ENTRY;
    if read_u8(entry) != 0xe9 || read_usize(this) == 0 {
        board_week_call_original(o, this); // ponytail: no hook on the story entry (or no table): only the original
        return;
    }
    let target = entry.wrapping_add(5).wrapping_add(read_i32(entry + 1) as usize);
    let count = read_i32(G_CLUB_COUNT).max(0) as usize;
    let mut regions: Vec<(usize, usize)> = vec![(read_usize(this), count * 0x49)];
    regions.extend_from_slice(&crate::rng::STATE_REGIONS);
    let base = read_usize(G_CLUB_TABLE);
    for i in 0..count {
        let club = Club(base + i * club::SIZE);
        let sheet = board_week_chairman_sheet(club.addr());
        if sheet != 0 {
            regions.push((sheet + non_player::PATIENCE, 1));
        }
        let manager = club.manager();
        if !manager.is_null() && manager.non_player().addr() != 0 {
            regions.push((manager.non_player().addr() + non_player::HOME_REPUTATION, 6));
        }
    }
    let before = board_story_flat(&regions);
    *core::ptr::addr_of_mut!(BOARD_STORY_TAPE) = Some(BoardStoryTape { regions, calls: Vec::new(), next: 0, broken: None, recording: true, depth: 0, target });
    let patch = |to: usize| {
        let mut old = 0u32;
        crate::win::VirtualProtect(entry as *mut core::ffi::c_void, 5, crate::win::PAGE_EXECUTE_READWRITE, &mut old);
        write_i32(entry + 1, to.wrapping_sub(entry + 5) as i32);
        crate::win::VirtualProtect(entry as *mut core::ffi::c_void, 5, old, &mut old);
        crate::win::FlushInstructionCache(crate::win::GetCurrentProcess(), entry as *const core::ffi::c_void, 5);
    };
    patch(board_story_recorder as *const () as usize);
    let residue = board_week_call_original(o, this);
    patch(target);
    let tape = (*core::ptr::addr_of_mut!(BOARD_STORY_TAPE)).as_mut().unwrap();
    tape.recording = false;
    let after = board_story_flat(&tape.regions);
    board_story_write(&tape.regions, &before);
    let n = crate::rng::without_counting(|| board_week_run(this, residue as i8));
    let tape = (*core::ptr::addr_of_mut!(BOARD_STORY_TAPE)).take().unwrap();
    let copy = board_story_flat(&tape.regions);
    let difference = board_story_first_difference(&tape.regions, &copy, &after);
    board_story_write(&tape.regions, &after);
    BOARD_WEEK_LEAGUE_SIZE.store(n as u8, Relaxed);
    let ok = tape.broken.is_none() && tape.next == tape.calls.len() && difference.is_none();
    SCOREBOARD.record(ok, || {
        let place = difference.map(|(i, o, x, y)| {
            let what = if i == 0 { format!("board row of club {} +{:#x}", o / 0x49, o % 0x49) } else if i < 3 { format!("random generator +{:#x}", o) } else { format!("region {} +{:#x}", i, o) };
            format!("{} copy {:02x} original {:02x}", what, x, y)
        });
        format!("weekly_board_confidence_update(residue N {}): stories {} replayed {} {:?}; state {:?}", residue as i8, tape.calls.len(), tape.next, tape.broken, place)
    });
}

/// Writes the headline or body of the news that a club or national team manager resigned, retired or was sacked.
///
/// `news_manager_departure_text(news, body, out)`: slots 0 (the manager's id), 1..3 (his name ids), 4 (how he left: 1
/// sacked, 2 resigned, 3 retired), 5 (the club id), 7 (the job: 1 manager, 2 player-manager, 3 with a successor or
/// reason), 8..10 (a second person's name ids, −1 none) and, in the body, 11 (the reason code). The name
/// (`departure_news_name`: style 5 in the headline, 1 in the body); the second name (`exe::database_format_name_from_ids`
/// with −1; empty without one; also computed in the headline, where it is not used). Headline (with the club's short
/// gender and short name, `club::SHORT_GENDER_NAME`, `club::SHORT_NAME`): sacked as manager 0xa09bb0, as
/// player-manager 0xa09c08, job 3 0xa09c68, other jobs nothing; resigned as manager 0xa09cb8, as player-manager
/// 0xa09d10, job 3 0xa09d6c, other jobs nothing; retired 0xa09dc4; any other way 0xa09d6c. Body (with the club's gender
/// `club::GENDER_NAME`, its name `club::NAME`, and the nationality adjective of `club::NATION`,
/// `exe::nation_get_nationality_adjective` language 9, or "Sedgefordian" without a nation): a national team
/// (`exe::database_is_national_team`): the second national team (`exe::club_is_national_team`) sacked by the reason
/// (2 0xa096d0, 4 0xa09788, 6 0xa09848, others 0xa09950), retired 0xa099e8, otherwise 0xa09ac8; the first one sacked by
/// the reason (2 0xa09210, 4 0xa092d0, 6 0xa09394, others 0xa09494), retired 0xa09534, otherwise 0xa09608. A club:
/// sacked: as manager with a second name 0xa071a4, without 0xa070b0; as player-manager 0xa0736c / 0xa072b0; job 3 by
/// the reason 0..15 from two tables (with a second name: 0xa08624, 0xa08434, 0xa08338, 0xa080f0, 0xa08528, 0xa08208,
/// 0xa087f8, 0xa0772c, 0xa086f8, 0xa08c48, 0xa08b04, 0xa08934, 0xa08dfc, others 0xa080f0; without: 0xa07a08, 0xa07848,
/// 0xa0764c, 0xa07924, 0xa07540, 0xa07bbc, 0xa0772c, 0xa07ac4, 0xa07e7c, 0xa07ccc, 0xa07fdc, reason 13 nothing, others
/// 0xa07440), other jobs nothing; resigned as manager 0xa08f40, as player-manager 0xa09030, otherwise 0xa090f0; any
/// other way 0xa091a8. "Nothing" leaves the buffer 0xde1f64 as it was. Then the text (`exe::text_set_string`), the
/// person's link (0x7d7) when the name ids still match (`exe::database_staff_matches_names`) and, in the body, the
/// club's link (0x7d5) on its name and on the adjective when they appear in the text (`exe::crt_strstr`); in the
/// headline, the club's link on its short name always.
/// Original: `news_manager_departure_text` (0x6a1d60, manager_manager.cpp, stdcall, ret 0xc).
pub(crate) unsafe extern "stdcall" fn news_manager_departure_text(news: usize, body: u32, out: usize) {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let crt_format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let s: [u32; 12] = core::array::from_fn(|k| if k == 6 || (k == 11 && body == 0) { 0 } else { exe::newsitem_get_slot(news, k as u32) });
    let id = s[0];
    let ids = [s[1], s[2], s[3]];
    let how = s[4] as u8 as i8;
    let job = s[7] as u8 as i8;
    let club = Club(read_usize(G_CLUB_TABLE).wrapping_add((s[5] as usize).wrapping_mul(club::SIZE)));
    let mut name = [0u8; 0x190];
    let mut second = [0u8; 0x190];
    let mut club_name = [0u8; 0x190];
    let mut adjective = [0u8; 0x190];
    let (n, n2, c, adj) = (name.as_ptr() as usize, second.as_ptr() as usize, club_name.as_mut_ptr() as usize, adjective.as_mut_ptr() as usize);
    let buf = 0xde1f64usize;
    let pct = 0x991944usize; // "%s"
    departure_news_name(news, id, ids, &mut name, if body == 0 { 5 } else { 1 });
    if body == 0 {
        if s[8] as i32 != -1 {
            exe::database_format_name_from_ids(s[8], s[9], s[10], second.as_mut_ptr(), 0x190, 5, u32::MAX);
        }
        crt_format(c, pct, club.addr() + club::SHORT_NAME);
        let g = (club.short_gender_name() as i8) as i32;
        match how {
            1 => match job {
                1 => { format(buf, 0xa09bb0, g, g, c, n); }
                2 => { format(buf, 0xa09c08, g, g, c, n); }
                3 => { format(buf, 0xa09c68, g, g, c, n); }
                _ => {}
            },
            2 => match job {
                1 => { format(buf, 0xa09cb8, g, g, c, n); }
                2 => { format(buf, 0xa09d10, g, g, c, n); }
                3 => { format(buf, 0xa09d6c, g, g, n, c); }
                _ => {}
            },
            3 => { format(buf, 0xa09dc4, g, g, c, n); }
            _ => { format(buf, 0xa09d6c, g, g, n, c); }
        }
        exe::text_set_string(out, buf as *const u8);
        let person = read_usize(G_STAFF_TABLE).wrapping_add((id as usize).wrapping_mul(staff::SIZE));
        if exe::database_staff_matches_names(person, ids[0], ids[1], ids[2], 0usize) != 0 {
            exe::text_add_link(out, name.as_ptr(), 0x7d7, person);
        }
        exe::text_add_link(out, club_name.as_ptr(), 0x7d5, club);
        return;
    }
    covered!("sma_10 departure body");
    let reason = s[11] as u8 as i8;
    crt_format(c, pct, club.addr() + club::NAME);
    let nation = club.nation().addr();
    if nation != 0 {
        exe::nation_get_nationality_adjective(nation, adj, 0x190, 9);
    } else {
        crt_format(adj, pct, 0xa09ba0usize); // "Sedgefordian"
    }
    if s[8] as i32 != -1 {
        exe::database_format_name_from_ids(s[8], s[9], s[10], second.as_mut_ptr(), 0x190, 1, u32::MAX);
    }
    let g = club.gender_name() as i32;
    let has_second = second[0] != 0;
    if exe::database_is_national_team(club) != 0 {
        covered!("sma_10 national team departure");
        let second_team = exe::club_is_national_team(club) != 0;
        match (how, second_team) {
            (1, true) => match reason {
                2 => { format(buf, 0xa096d0, 0x64, 0x64, n, adj); }
                4 => { format(buf, 0xa09788, 0x64, 0x64, n, adj); }
                6 => { format(buf, 0xa09848, g, g, 0x64, 0x64, n, c, adj); }
                _ => { format(buf, 0xa09950, 0x64, 0x64, n, adj); }
            },
            (3, true) => { format(buf, 0xa099e8, g, g, 0x64, 0x64, n, c, adj); }
            (_, true) => { format(buf, 0xa09ac8, g, g, 0x64, 0x64, n, c, adj); }
            (1, false) => match reason {
                2 => { format(buf, 0xa09210, 0x64, 0x64, n, adj); }
                4 => { format(buf, 0xa092d0, 0x64, 0x64, n, adj); }
                6 => { format(buf, 0xa09394, g, g, 0x64, 0x64, n, c, adj); }
                _ => { format(buf, 0xa09494, 0x64, 0x64, n, adj); }
            },
            (3, false) => { format(buf, 0xa09534, g, g, 0x64, 0x64, n, c, adj); }
            (_, false) => { format(buf, 0xa09608, g, g, 0x64, 0x64, n, c, adj); }
        }
    } else {
        match how {
            1 => match job {
                1 => {
                    if has_second { format(buf, 0xa071a4, g, g, g, g, n, c, n2, c); } else { format(buf, 0xa070b0, g, g, g, g, n, c, c); }
                }
                2 => {
                    if has_second { format(buf, 0xa0736c, g, g, g, g, n, c, n2, c); } else { format(buf, 0xa072b0, g, g, g, g, n, c, c); }
                }
                3 => {
                    covered!("sma_10 sacked with a reason");
                    if has_second {
                        match reason {
                            0 | 1 | 3 => { format(buf, 0xa08624, g, g, g, g, n, c, n2, c); }
                            2 => { format(buf, 0xa08434, g, g, g, g, n, c, n2, c); }
                            4 => { format(buf, 0xa08338, g, g, g, g, n, c, n2, c); }
                            5 => { format(buf, 0xa08624, g, g, g, g, n, c, n2, c); }
                            7 => { format(buf, 0xa08528, g, g, g, g, n, c, n2, c); }
                            8 => { format(buf, 0xa08208, g, g, g, g, c, n2, n, c); }
                            9 => { format(buf, 0xa087f8, g, g, 0x64, n, c, n2, n); }
                            10 => { format(buf, 0xa0772c, g, g, g, g, n, c, n2, c); }
                            11 => { format(buf, 0xa086f8, g, g, g, g, n, c, n2, c); }
                            12 => { format(buf, 0xa08c48, g, g, 0x64, c, n2, n, n); }
                            13 => { format(buf, 0xa08b04, g, g, 0x64, c, n2, n, n); }
                            14 => { format(buf, 0xa08934, g, g, c, n2, n, n); }
                            15 => { format(buf, 0xa08dfc, g, g, n, c, n2, n); }
                            _ => { format(buf, 0xa080f0, g, g, g, g, n, c, n2, c); } // sma10: the default reason
                        }
                    } else {
                        match reason {
                            0 | 1 | 5 => { format(buf, 0xa07a08, g, g, g, g, n, c, c); }
                            2 => { format(buf, 0xa07848, g, g, g, g, n, c, c); }
                            3 => { format(buf, 0xa07a08, g, g, g, g, n, c, c); }
                            4 => { format(buf, 0xa0764c, g, g, g, g, n, c, c); }
                            7 => { format(buf, 0xa07924, g, g, g, g, n, c, c); }
                            8 => { format(buf, 0xa07540, g, g, g, g, c, n, c); }
                            9 => { format(buf, 0xa07bbc, g, g, 0x64, n, c, n); }
                            10 => { format(buf, 0xa0772c, g, g, g, g, n, c, n2, c); }
                            11 => { format(buf, 0xa07ac4, g, g, g, g, n, c, c); }
                            12 => { format(buf, 0xa07e7c, g, g, c, n); }
                            13 => {}
                            14 => { format(buf, 0xa07ccc, g, g, c, n, n); }
                            15 => { format(buf, 0xa07fdc, g, g, n, c, n); }
                            _ => { format(buf, 0xa07440, g, g, g, g, n, c, c); }
                        }
                    }
                }
                _ => {}
            },
            2 => match job {
                1 => { format(buf, 0xa08f40, g, g, g, g, n, c, c); }
                2 => { format(buf, 0xa09030, g, g, g, g, n, c, c); }
                _ => { format(buf, 0xa090f0, g, g, g, g, n, c, c); }
            },
            _ => { format(buf, 0xa091a8, g, g, n, c); }
        }
    }
    exe::text_set_string(out, buf as *const u8);
    let person = read_usize(G_STAFF_TABLE).wrapping_add((id as usize).wrapping_mul(staff::SIZE));
    if exe::database_staff_matches_names(person, ids[0], ids[1], ids[2], 0usize) != 0 {
        exe::text_add_link(out, name.as_ptr(), 0x7d7, person);
    }
    if exe::crt_strstr(buf, c) != 0 { // sma10: the club's link when its name is in the text
        exe::text_add_link(out, club_name.as_ptr(), 0x7d5, club);
    }
    if exe::crt_strstr(buf, adj) != 0 {
        exe::text_add_link(out, adjective.as_ptr(), 0x7d5, club);
    }
}

/// Comparer for `news_manager_departure_text` (`compare_text_writer`).
pub(crate) unsafe extern "stdcall" fn compare_news_manager_departure_text(news: usize, body: u32, out: usize) {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, u32, usize)>(IDX_NEWS_MANAGER_DEPARTURE_TEXT);
    compare_text_writer(out, || o(news, body, out), |t| news_manager_departure_text(news, body, t),
        || format!("news_manager_departure_text(body {})", body));
}

/// How well `staff` fits job `job` at `club`, as seen by `viewer` (a staff member; by default the club's chairman):
/// −10000 for "never". Used by the manager market (`consider_sacking_manager`, `add_job_vacancy`,
/// `best_manager_candidate_score`, 0x69ca00) and by the shortlists of the staff search.
///
/// Without a non-player sheet (`.non_player()`): −10000. A viewer without a sheet is replaced by the club's chairman
/// (`.chairman()`), and without one either, no viewer. The person's role: `.job_for_club()`, else `.job_for_nation()`,
/// else 11. The score starts at the reputation as the viewer sees it (`exe::database_get_staff_reputation_for_viewer`,
/// non-player; without a viewer, a local staff record with id 0, the club's nation and the club as its club job, as the
/// original builds it). In role 12 or 5 at a club with reputation (`.reputation()`) above 4250: + current ability × 25,
/// and with world reputation above 3250 also + potential ability × 5. Already at the club (`.club_job()`): + 100 for a
/// human manager (id ≥ `G_STAFF_COUNT` − 16), otherwise + loyalty². A favourite of the club
/// (`exe::club_favours_staff`): max(× 1.1, + 2000); otherwise rejected by it (`exe::club_dislikes_staff`): × 0.25. The
/// same with the viewer (`exe::staff_is_favoured_by`, `exe::staff_is_disliked_by`).
///
/// By the job (other jobs and 11: −10000):
/// - 5 and 12 (manager jobs): a human manager skips straight to the common part. By the role: 12 × 0.95; 6 and 13 at
///   the same club or national team (`.national_job()`) × 0.75, otherwise × 0.6 and −10000 above reputation 5750; 7 ×
///   0.5 / × 0.3 and 5250; 8 × 0.6 / × 0.475 and 5250; 11 × 0.95 / × 0.75 and 5250; 15 × 0.6 / × 0.4 and 5750; 16 ×
///   0.75 and, outside the club, 5750; 9 × 0.2 and 3250; 10, 14 and the others × 0.001 and 5750; 5 nothing. Age
///   (`.age()`) above 59: × 0.875 at a national team above reputation 7000, × 0.9 above 7250, else × 0.95; below 38
///   with world reputation below 4250: × (age × 0.05 − 0.95). Below 45 at a club below reputation 7000: + min(potential
///   − current, 50)² × the viewer's patience (10 without a viewer) × 0.1. With a viewer, the similarity (difference
///   divided by F × 100, 250, 100, 150, 150, 300, 300, 200, 150, F = 1.25 − the viewer's interference × 0.025); the
///   weights coaching goalkeepers and coaching (× 0.000833), judgement, judging potential, man handling, motivating
///   (× 0.002), tactics (× 0.008 above reputation 7250, × 0.004 above 5750, else × 0.002); the attitudes × 0.005.
/// - 6, 7, 13, 14 (coaches): similarity × 0.00667, 0.00667, 0.01, 0.00444, 0.00667, 0.01, 0.01, 0.00667, 0.004;
///   weights × 0.002, 0.002, 0.00133, 0.00133, 0.000667, 0.00133, tactics × 0.002; attitudes × 0.005.
/// - 8, 15 (assistant jobs): similarity as the coaches but coaching technique × 0.01; weights × 0.005, 0.005, 0.00133,
///   0.00133, 0.00133, 0.000833, tactics × 0.004; attitudes × 0.02.
/// - 9 (scout): similarity × 0.002 each; judgement and judging potential × 0.005; attitudes × 0.002.
/// - 10 (physio): physiotherapy × 0.01; attitudes × 0.02.
///
/// The common part. A club at the person's nation, at the nation of the club job or of the national job (the first one
/// whose nation is the club's), else the person's nation. A foreigner (`exe::database_is_staff_foreign`, groups) at a
/// club above reputation 4750 whose club job is not in the club's country, coming from a weaker league
/// (`.league_standard()` of his nation or of his club's nation, the larger, below the club nation's) and not from a
/// club with reputation 6750 or more: −10000 outside roles 12 and 5, there × max(1 + difference × 0.025, 0.75).
/// National team (`exe::database_is_national_team`): its own nation max(× 1.2, + 2000); not foreign × (0.75 −
/// reputation × 0.000025); foreign × 0.35; above reputation 6250 × (world reputation × 0.000025 + 0.5) outside 12/5,
/// foreign × (× 0.00005 + 0.5), otherwise × (× 0.000025 + 0.75). A human manager there: by the reputation the viewer
/// sees, below 4250 the score is 0, below 6750 × 0.75; below 7250 and before 23/6/2003 × 0.75. Club above reputation
/// 7250 with a division and a nation of league standard above 10: + (the club job's reputation, or the world
/// reputation, − the club's) × 0.15 (truncated to a word); above 8250 with world reputation below 4250: −10000 for a
/// foreigner, someone without a club job or division, or from a division below the club's (`.reputation()` of
/// `.division()`); then, outside job 9, the reputation factor as at the national team. Other clubs: a foreigner outside
/// job 9 × (world reputation × 0.00005 + 0.5). Then × (language fluency × 0.05 + 0.5) (`exe::playerstats_get_language_fluency`
/// in the club nation's `.first_language()`, by contract). Last, the chosen nation's `.state_of_development()`: 1 as
/// is; 2 × 0.75 when the club's nation is 1; 3 × 0.25 when the club's nation is 1 or 2; other × 0.05; no nation × 0.25.
/// Defect of the original: without any of those nations the routine reads the development of a stack value (the
/// person's professionalism − 10, or garbage) as a nation; every staff member has a nation, so the copy takes "no
/// nation" there.
/// Original: `score_staff_for_job` (0x69d1b0, manager_manager.cpp, stdcall, ret 0x10; ecx unused).
pub(crate) unsafe extern "thiscall" fn score_staff_for_job(_this: usize, staff: usize, club: usize, viewer: usize, job: u32) -> i32 {
    use staff_job_score_rdata::*;
    const NEVER: i32 = -10000;
    let p = Staff(staff);
    let club = Club(club);
    let np = p.non_player().addr();
    if np == 0 {
        return NEVER;
    }
    let mut viewer = Staff(viewer);
    if viewer.is_null() || viewer.non_player().is_null() {
        viewer = club.chairman();
        if viewer.is_null() || viewer.non_player().is_null() {
            viewer = Staff(0);
        }
    }
    let age = p.age() as i8;
    let mut role = p.job_for_club();
    if role == 0 {
        role = p.job_for_nation();
        if role == 0 {
            role = 11;
        }
    }
    let manager_role = role == 12 || role == 5;
    let human = read_i32(p.addr()) >= read_i32(G_STAFF_COUNT).wrapping_sub(0x10);
    let mut v = if !viewer.is_null() {
        exe::database_get_staff_reputation_for_viewer(p, viewer, 1) as i32
    } else {
        // the local staff record of the original: the dates constructed, id 0, the club's nation, the club as club job
        let mut local = [0u8; staff::SIZE];
        let l = local.as_mut_ptr() as usize;
        for d in [staff::DATE_OF_BIRTH, staff::DATE_JOINED_NATION, 0x31, staff::DATE_JOINED_CLUB, staff::DATE_EXPIRES_CLUB] {
            exe::date_date_54a110(l + d);
        }
        write_u32(l + staff::NATION, club.nation().addr() as u32);
        write_u32(l + staff::CLUB_JOB, club.addr() as u32);
        exe::database_get_staff_reputation_for_viewer(p, l, 1) as i32
    };
    let npr = NonPlayer(np);
    if manager_role && club.reputation() > 0x109a {
        v += npr.current_ability() as i32 * 25;
        if npr.world_reputation() > 0xcb2 {
            v += read_i16(np + non_player::POTENTIAL_ABILITY) as i32 * 5;
        }
    }
    if p.club_job() == club {
        if human {
            v += 100;
        } else {
            let l = p.loyalty() as i32;
            v += l * l;
        }
    }
    if exe::club_favours_staff(p, club) != 0 {
        v = staff_job_bonus(v, rdata::F64_1_1);
    } else if exe::club_dislikes_staff(p, club) != 0 {
        v = ftol(v as f64 * read_f64(rdata::F64_0_25));
    }
    if exe::staff_is_favoured_by(p, viewer) != 0 {
        v = staff_job_bonus(v, rdata::F64_1_1);
    } else if exe::staff_is_disliked_by(p, viewer) != 0 {
        v = ftol(v as f64 * read_f64(rdata::F64_0_25));
    }
    let mul = |v: i32, k: usize| ftol(v as f64 * read_f64(k));
    let rep = club.reputation();
    let at_club = p.club_job() == club || p.national_job() == club;
    let vnp = if viewer.is_null() { 0 } else { viewer.non_player().addr() };
    use non_player as n;
    let job = job as u8;
    match job {
        5 | 12 => {
            if !human {
                covered!("sma_1 manager job");
                // the role: (factor at the club or national team, factor elsewhere, reputation limit elsewhere)
                let (here, away, limit): (usize, usize, i16) = match role {
                    5 => (0, 0, 0),
                    6 | 13 => (rdata::F64_0_75, rdata::F64_0_6, 0x1676),
                    7 => (rdata::F64_0_5, rdata::F64_0_3, 0x1482),
                    8 => (rdata::F64_0_6, F64_0_475, 0x1482),
                    11 => (rdata::F64_0_95, rdata::F64_0_75, 0x1482),
                    15 => (rdata::F64_0_6, rdata::F64_0_4, 0x1676),
                    12 => (rdata::F64_0_95, rdata::F64_0_95, i16::MAX),
                    16 => (rdata::F64_0_75, rdata::F64_0_75, 0x1676),
                    9 => (rdata::F64_0_2, rdata::F64_0_2, 0xcb2), // also at the club
                    _ => (rdata::F64_0_001, rdata::F64_0_001, 0x1676), // 10, 14 and the others, also at the club
                };
                if role != 5 {
                    let checked_here = matches!(role, 9 | 10 | 14) || !(5..=16).contains(&role);
                    if at_club && !checked_here {
                        v = mul(v, here);
                    } else {
                        v = mul(v, away);
                        if rep > limit {
                            return NEVER;
                        }
                    }
                }
                if age > 0x3b {
                    v = if exe::database_is_national_team(club) != 0 && rep > 0x1b58 {
                        mul(v, rdata::F64_0_875)
                    } else if rep > 0x1c52 {
                        mul(v, rdata::F64_0_9)
                    } else {
                        mul(v, rdata::F64_0_95)
                    };
                } else if age < 0x26 && npr.world_reputation() < 0x109a {
                    v = ftol((age as i32 as f64 * read_f64(rdata::F64_0_05) - read_f64(rdata::F64_0_95)) * v as f64);
                }
                if age < 0x2d && rep < 0x1b58 {
                    covered!("sma_1 young manager potential");
                    let gap = (read_i16(np + n::POTENTIAL_ABILITY) as i32 - npr.current_ability() as i32).min(50);
                    let patience = if vnp != 0 { read_i8(vnp + n::PATIENCE) as i32 } else { 10 };
                    v += ftol(game_pow(gap as f64, read_f64(rdata::F64_2)) * patience as f64 * read_f64(rdata::F64_0_1)); // sma1: the potential bonus
                }
                if vnp != 0 {
                    let f = read_f64(rdata::F64_1_25) - read_i8(vnp + n::INTERFERENCE) as i32 as f64 * read_f64(rdata::F64_0_025);
                    let ds = [rdata::F64_100, F64_250, rdata::F64_100, rdata::F64_150, rdata::F64_150, rdata::F64_300, rdata::F64_300, rdata::F64_200, rdata::F64_150];
                    v = staff_job_similarity(v, np, vnp, &ds, Some((f, ds)));
                }
                for (a, k) in [(n::COACHING_GKS, rdata::F64_0_0008333333333), (n::COACHING, rdata::F64_0_0008333333333), (n::JUDGEMENT, rdata::F64_0_002),
                    (n::JUDGING_POTENTIAL, rdata::F64_0_002), (n::MAN_HANDLING, rdata::F64_0_002), (n::MOTIVATING, rdata::F64_0_002)] {
                    v = staff_job_weight(v, read_i8(np + a), k);
                }
                let k = if rep > 0x1c52 { rdata::F64_0_008 } else if rep > 0x1676 { rdata::F64_0_004 } else { rdata::F64_0_002 };
                v = staff_job_weight(v, read_i8(np + n::TACTICS), k); // sma1: the manager's tactics weight
                v = staff_job_attitudes(p, v, rdata::F64_0_005);
            }
        }
        6 | 7 | 13 | 14 | 8 | 15 => {
            let assistant = job == 8 || job == 15;
            if vnp != 0 {
                let t = if assistant { rdata::F64_0_01 } else { rdata::F64_0_006666666667 };
                let ks = [rdata::F64_0_006666666667, t, rdata::F64_0_01, F64_0_0044444444444, rdata::F64_0_006666666667, rdata::F64_0_01, rdata::F64_0_01,
                    rdata::F64_0_006666666667, rdata::F64_0_004];
                v = staff_job_similarity(v, np, vnp, &ks, None);
            }
            let ws: [(usize, usize); 7] = if assistant {
                [(n::COACHING_GKS, rdata::F64_0_005), (n::COACHING, rdata::F64_0_005), (n::JUDGEMENT, F64_0_0013333333333), (n::JUDGING_POTENTIAL, F64_0_0013333333333),
                    (n::MAN_HANDLING, F64_0_0013333333333), (n::MOTIVATING, rdata::F64_0_0008333333333), (n::TACTICS, rdata::F64_0_004)]
            } else {
                [(n::COACHING_GKS, rdata::F64_0_002), (n::COACHING, rdata::F64_0_002), (n::JUDGEMENT, F64_0_0013333333333), (n::JUDGING_POTENTIAL, F64_0_0013333333333),
                    (n::MAN_HANDLING, rdata::F64_0_0006666666667), (n::MOTIVATING, F64_0_0013333333333), (n::TACTICS, rdata::F64_0_002)]
            };
            for (a, k) in ws {
                v = staff_job_weight(v, read_i8(np + a), k);
            }
            v = staff_job_attitudes(p, v, if assistant { rdata::F64_0_02 } else { rdata::F64_0_005 }); // sma1: the coaches' attitudes
        }
        9 => {
            if vnp != 0 {
                v = staff_job_similarity(v, np, vnp, &[rdata::F64_0_002; 9], None);
            }
            v = staff_job_weight(v, read_i8(np + n::JUDGEMENT), rdata::F64_0_005);
            v = staff_job_weight(v, read_i8(np + n::JUDGING_POTENTIAL), rdata::F64_0_005);
            v = staff_job_attitudes(p, v, rdata::F64_0_002);
        }
        10 => {
            v = staff_job_weight(v, read_i8(np + n::PHYSIOTHERAPY), rdata::F64_0_01);
            v = staff_job_attitudes(p, v, rdata::F64_0_02);
        }
        _ => return NEVER,
    }
    // the nation that sets the development factor at the end
    let cn = club.nation();
    let cj = p.club_job();
    let nj = p.national_job();
    let nation = if !cj.is_null() && cj.nation() == cn {
        cn.addr()
    } else if !nj.is_null() && nj.nation() == cn {
        cn.addr()
    } else {
        p.nation().addr()
    };
    // a foreigner from a weaker league
    if rep > 0x128e && !cn.is_null() && !p.nation().is_null() && exe::database_is_staff_foreign(p, cn, 1) != 0 && (cj.is_null() || cj.nation() != cn) {
        let mut own = p.nation().league_standard() as i8;
        let mut weaker = true;
        if !cj.is_null() {
            let jn = cj.nation();
            if !jn.is_null() && jn.league_standard() as i8 > own {
                own = jn.league_standard() as i8;
            }
            weaker = cj.reputation() < 0x1a5e;
        }
        let theirs = cn.league_standard() as i8;
        if weaker && own < theirs {
            covered!("sma_1 foreigner from a weaker league");
            let mut f = (own as i32 - theirs as i32) as f64 * read_f64(rdata::F64_0_025) + read_f64(rdata::F64_1);
            if f < read_f64(rdata::F64_0_75) {
                f = read_f32(rdata::F32_0_75) as f64;
            }
            if !manager_role {
                return NEVER;
            }
            v = ftol(v as f64 * f);
        }
    }
    let foreign = || exe::database_is_staff_foreign(p, cn, 1) != 0;
    // the reputation factor: (world reputation × k1 + k2) by the role and the nationality
    let rep_factor = |v: i32| {
        let w = npr.world_reputation() as i32 as f64;
        let f = if !manager_role {
            w * read_f64(rdata::F64_2_5ENEG_05) + read_f64(rdata::F64_0_5)
        } else if foreign() {
            w * read_f64(rdata::F64_5ENEG_05) + read_f64(rdata::F64_0_5)
        } else {
            w * read_f64(rdata::F64_2_5ENEG_05) + read_f64(rdata::F64_0_75)
        };
        ftol(f * v as f64)
    };
    if exe::database_is_national_team(club) != 0 {
        covered!("sma_1 national team");
        if p.nation() == cn {
            v = staff_job_bonus(v, rdata::F64_1_2); // sma1: the national team's own nation
        } else if !foreign() {
            v = ftol((read_f64(rdata::F64_0_75) - rep as i32 as f64 * read_f64(rdata::F64_2_5ENEG_05)) * v as f64);
        } else {
            v = mul(v, rdata::F64_0_35);
        }
        if rep > 0x186a {
            v = rep_factor(v);
        }
        if human {
            let r = exe::database_get_staff_reputation_for_viewer(p, viewer, 1);
            if r < 0x109a {
                v = 0;
            } else if r < 0x1a5e {
                v = mul(v, rdata::F64_0_75);
            }
            if r < 0x1c52 {
                let mut d = [0u16; 4];
                exe::date_date_549f70(&mut d, 0x17, 6, 0x7d3, u32::MAX);
                let today = read_u16(G_CURRENT_DATE_YEAR);
                if d[1] > today || (d[1] == today && d[0] as i16 > read_i16(G_CURRENT_DATE)) {
                    v = mul(v, rdata::F64_0_75);
                }
            }
        }
    } else if rep > 0x1c52 && !club.division().is_null() && !cn.is_null() && cn.league_standard() as i8 > 10 {
        covered!("sma_1 big club");
        let d = if !cj.is_null() { cj.reputation() as i32 - rep as i32 } else { npr.world_reputation() as i32 - rep as i32 };
        v += ftol(d as f64 * read_f64(rdata::F64_0_15)) as i16 as i32;
        if rep > 0x203a && npr.world_reputation() < 0x109a {
            if foreign() || cj.is_null() || cj.division().is_null() || cj.division().reputation() < club.division().reputation() {
                return NEVER;
            }
        }
        if job != 9 {
            v = rep_factor(v);
        }
    } else if foreign() && job != 9 {
        v = ftol((npr.world_reputation() as i32 as f64 * read_f64(rdata::F64_5ENEG_05) + read_f64(rdata::F64_0_5)) * v as f64);
    }
    if !cn.is_null() {
        let fl = exe::playerstats_get_language_fluency(read_usize(G_PLAYER_STATS), p, club, cn.first_language() as u8 as u32, 1) as i8;
        v = ftol((fl as i32 as f64 * read_f64(rdata::F64_0_05) + read_f64(rdata::F64_0_5)) * v as f64); // sma1: the language factor
    }
    if nation == 0 {
        return mul(v, rdata::F64_0_25);
    }
    let club_dev = || if cn.is_null() { None } else { Some(cn.state_of_development() as i8) };
    match Nation(nation).state_of_development() as i8 {
        1 => v,
        2 => if club_dev() == Some(1) { mul(v, rdata::F64_0_75) } else { v },
        3 => if matches!(club_dev(), Some(1) | Some(2)) { mul(v, rdata::F64_0_25) } else { v },
        _ => mul(v, rdata::F64_0_05),
    }
}

/// Moves `manager` to `club` as its manager: vacancies at the old job, the appointment news, the new job, the squad
/// slot of a player-manager, the board reset and the grievances of the squad.
///
/// `MANAGER_MANAGER::appoint_manager_to_club(club, manager)`. The old job: at a national team, the person's national
/// job (`.national_job()`) when he manages one (`.job_for_nation()` 5 or 12); at a club, his club job (`.club_job()`).
/// With an old job: leaving a club he managed (`.job_for_club()` 5 or 12) for a club, the old club gets a vacancy
/// (`exe::manager_manager_add_job_vacancy`) when its nation's league is selected (`nation::LEAGUE_SELECTED`) and its
/// division (`.division()`) has a competition object (`G_COMP_OBJECT_TABLE`); leaving a national team he managed for
/// a national team, when the old one has reputation (`.reputation()`) above 3500 and its nation no continent
/// (`.continent()`); moving to a club, a player-manager or role 15 loses his playing contract
/// (`exe::contract_manager_remove_contract`). The board's `board_record::LAST_REVIEW_DATE`-like date at +0x20 of the
/// club's board row becomes 23/7/1971 (`exe::date_date_549f70`, `exe::date_operator_3d`); without a non-player sheet,
/// the error box (line 0x3480). The old job is left by role (`appoint_leave_old_job`) and the appointment news posted
/// (`post_manager_appointment_news`, code 1..3; without an old job of that kind, with no old club and code 2); at a
/// national team `.job_for_nation()` becomes 5. A human manager: the automatic replies (`exe::news_auto_respond_all`)
/// and, when the club's nation is selected (bit 1 of `nation::LEAGUE_SELECTED`), the old club's nation stops being
/// "the human's" (bit 2 cleared, bit 1 set) unless another human manages in a division of that nation, and the new
/// nation gets bit 2 instead of bit 1. The joining date (`staff::DATE_JOINED_NATION` or `staff::DATE_JOINED_CLUB`)
/// becomes today, the club's `.manager()` the person; at a national team his `.national_job()` the club and job 5; at
/// a club his `.club_job()` the club and job 5, or 12 (player-manager, in the first free of the 50 `.squad()` slots)
/// when his player reputation (`.current_reputation()`) + 750 is above the club's. A human: the scout lists
/// (`exe::scout_manager_sync_scout_lists`) and the training schedules (`exe::training_manager_reassign_squad_schedules`).
/// Then the board reset (`reset_board_for_new_manager`) and, at a club, each squad player's contract
/// (`exe::contract_manager_get_contract`) loses the grievance bits of `NEW_MANAGER_CLEARED_GRIEVANCES`
/// (`exe::staff_contract_clear_grievances`).
/// Original: `MANAGER_MANAGER::appoint_manager_to_club` (0x69ebc0, manager_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn appoint_manager_to_club(this: usize, club: usize, manager: usize) {
    let club = Club(club);
    let p = Staff(manager);
    let national = exe::database_is_national_team(club) != 0;
    let manages = |role: u8| role == 5 || role == 12;
    let old = if national && !p.national_job().is_null() && manages(p.job_for_nation()) {
        p.national_job()
    } else if !national && !p.club_job().is_null() {
        p.club_job()
    } else {
        Club(0)
    };
    if !old.is_null() {
        if !national && manages(p.job_for_club()) && old == p.club_job() {
            let n = old.nation();
            let d = old.division();
            if !n.is_null() && n.league_selected() != 0 && !d.is_null() && read_usize(read_usize(G_COMP_OBJECT_TABLE) + read_u32(d.addr()) as usize * 4) != 0 {
                covered!("sma_2 vacancy at the old club");
                exe::manager_manager_add_job_vacancy(this, old, p); // sma2: the old club's vacancy
            }
        } else if national && manages(p.job_for_nation()) && old == p.national_job() && old.reputation() > 0xdac && old.nation().continent() == 0 {
            exe::manager_manager_add_job_vacancy(this, old, p);
        }
        if !national && (p.job_for_club() == 15 || p.job_for_club() == 12) {
            exe::contract_manager_remove_contract(read_usize(G_CONTRACT_MANAGER), p);
        }
    }
    let mut d = [0u16; 4];
    let mut tmp = [0u8; 8];
    exe::date_date_549f70(&mut d, 0x17, 7, 0x7b3, u32::MAX);
    exe::date_operator_3d(board_row(this, club) + 0x20, tmp.as_mut_ptr() as usize, d.as_ptr() as usize);
    if p.non_player().is_null() {
        show_error_at(appoint_fns::MANAGER_MANAGER_CPP, 0x3480);
    }
    let news: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) = core::mem::transmute(appoint_fns::POST_MANAGER_APPOINTMENT_NEWS);
    if !national {
        if old.is_null() || old != p.club_job() {
            news(this, p.addr(), 0, club.addr(), 2);
        } else {
            let code = appoint_leave_old_job(this, p, old, p.job_for_club(), false);
            news(this, p.addr(), old.addr(), club.addr(), code);
        }
    } else {
        if old.is_null() || old != p.national_job() {
            news(this, p.addr(), 0, club.addr(), 2);
        } else {
            let code = appoint_leave_old_job(this, p, old, p.job_for_nation(), true);
            news(this, p.addr(), old.addr(), club.addr(), code);
        }
        if p.job_for_nation() != 5 {
            p.set_job_for_nation(5);
        }
    }
    let human = read_i32(p.addr()) >= read_i32(G_STAFF_COUNT).wrapping_sub(0x10);
    if human {
        covered!("sma_2 human manager");
        exe::news_auto_respond_all(read_usize(G_NEWS), p, old);
        let cn = club.nation();
        if !cn.is_null() && cn.league_selected() & 1 != 0 {
            if !old.is_null() {
                let on = old.nation();
                if !on.is_null() {
                    let count = read_i32(G_STAFF_COUNT);
                    let other = (count - 0x10..count).any(|i| {
                        let s = Staff(read_usize(G_STAFF_TABLE).wrapping_add((i as usize).wrapping_mul(staff::SIZE)));
                        let c = s.club_job();
                        s.classification() != 0 && s != p && !c.is_null() && !c.division().is_null() && c.division().nation() == on
                    });
                    if !other {
                        on.set_league_selected(on.league_selected() & 0xfd);
                        old.nation().set_league_selected(old.nation().league_selected() | 1);
                    }
                }
            }
            cn.set_league_selected(cn.league_selected() & 0xfe);
            cn.set_league_selected(cn.league_selected() | 2); // sma2: the human's league mark
        }
    }
    let joined = if national { p.addr() + staff::DATE_JOINED_NATION } else { p.addr() + staff::DATE_JOINED_CLUB };
    exe::date_operator_3d(joined, tmp.as_mut_ptr() as usize, G_CURRENT_DATE);
    club.set_manager(p);
    if !national {
        p.set_club_job(club);
        let mut job = 5;
        let pl = p.player();
        if !pl.is_null() && (club.reputation() as i32) < pl.current_reputation() as i32 + 0x2ee {
            if let Some(i) = (0..50).find(|&i| club.squad(i) == 0) {
                covered!("sma_2 player-manager");
                club.set_squad(i, p.addr());
                job = 0xc; // sma2: the player-manager
            }
        }
        p.set_job_for_club(job);
    } else {
        p.set_national_job(club);
        p.set_job_for_nation(5);
    }
    if human {
        exe::scout_manager_sync_scout_lists(read_usize(G_SCOUT_MANAGER), p);
        exe::training_manager_reassign_squad_schedules(read_usize(G_TRAINING_MANAGER), p, old, club.addr() as u32); // sma2: the human's training
    }
    let reset: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(appoint_fns::RESET_BOARD_FOR_NEW_MANAGER);
    reset(this, club.addr()); // sma2: the new board
    if !national {
        for i in 0..50 {
            let s = Staff(club.squad(i));
            if s.is_null() || s.player().is_null() {
                continue;
            }
            let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), s);
            if c == 0 {
                continue;
            }
            for bit in NEW_MANAGER_CLEARED_GRIEVANCES {
                if read_u32(c + 0x45) & bit != 0 {
                    exe::staff_contract_clear_grievances(c, bit); // sma2: the grievances the new manager clears
                }
            }
        }
    }
}

/// Comparer for `appoint_manager_to_club`.
///
/// The move cannot run twice (news, vacancies, contracts): only the original runs, and the outcome it guarantees is
/// checked: the club's `.manager()` is the person, with his job at the club or national team set (5, or 12 for a
/// player-manager in the club's squad).
pub(crate) unsafe extern "thiscall" fn compare_appoint_manager_to_club(this: usize, club: usize, manager: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_APPOINT_MANAGER_TO_CLUB)(this, club, manager);
    let c = Club(club);
    let p = Staff(manager);
    let ok = if exe::database_is_national_team(c) != 0 {
        p.national_job() == c && p.job_for_nation() == 5
    } else {
        p.club_job() == c && (p.job_for_club() == 5 || (p.job_for_club() == 12 && (0..50).any(|i| c.squad(i) == manager)))
    };
    SCOREBOARD.record(c.manager() == p && ok, || format!("appoint_manager_to_club(club {:x}, manager {:x}): manager {:x}, club job {:x}/{}, national job {:x}/{}",
        club, manager, c.manager().addr(), p.club_job().addr(), p.job_for_club(), p.national_job().addr(), p.job_for_nation()));
}

/// Writes the board's season expectation sentence for a club from the expectation flag bits into `buf`.
///
/// `build_board_expectation_text(club, flags, buf, size)` (the size is not used; `this` neither). Each sentence is a
/// language format (`FN_LANGLIB_SPRINTF`) with the club's gender twice (`club::GENDER_NAME`) and its name
/// (`club::NAME`). Flag 1 (the title): by 0x400 / neither 0x400 nor 0x100 / 0x100 and by 0x800, with a second version
/// when the club's nation (`.nation()`) has a continent (`.continent()`) other than Europe (`KEY_CONTINENT_EUROPE`).
/// Flag 2: with
/// 0x400 or 0x100 one text by 0x800; otherwise by 0x800 and by the virtual +0x98 of the division's competition object
/// (`G_COMP_OBJECT_TABLE`, the top-division test). Then the first of 4, 8, 0x40, 0x10 set (`EXPECTATION_TEXTS`);
/// without 0x20, the "improve the squad" text 0xa03038; with it, the 0x20 texts.
/// Original: `MANAGER_MANAGER::build_board_expectation_text` (0x694440, manager_manager.cpp, thiscall, ret 0x10; ecx
/// unused).
pub(crate) unsafe extern "thiscall" fn build_board_expectation_text(_this: usize, club: usize, flags: u32, buf: usize, _size: u32) {
    let club = Club(club);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let c = club;
    write_u8(buf, 0);
    let g = club.gender_name() as i32;
    let say = |text: usize| {
        format(buf, text, g, g, club.addr() + club::NAME);
    };
    let f = |bit: u32| flags & bit != 0;
    let col = if f(0x400) { 0 } else if !f(0x100) { 1 } else { 2 };
    let late = f(0x800) as usize;
    if f(1) {
        covered!("sma_6 title expectation");
        let n = c.nation();
        let outside_europe = !n.is_null() && n.continent() != 0 && read_u32(n.continent()) != read_u32(KEY_CONTINENT_EUROPE);
        // (continental, other) by the column and 0x800
        let t: [[[usize; 2]; 2]; 3] = [[[0xa04a60, 0xa04ae8], [0xa04b80, 0xa04c4c]], [[0xa046a0, 0xa046f0], [0xa04744, 0xa047c0]],
            [[0xa04840, 0xa048a4], [0xa0490c, 0xa049b4]]];
        say(t[col][late][outside_europe as usize]); // sma6: the title sentence
        return;
    }
    if f(2) {
        if col != 1 {
            say(if late == 1 { 0xa04390 } else { 0xa0432c });
            return;
        }
        let d = c.division();
        let top = !d.is_null() && call_comp_virtual(read_usize(read_usize(G_COMP_OBJECT_TABLE) + read_u32(d.addr()) as usize * 4), 0x98) != 0;
        say(match (late, top) {
            (0, true) => 0xa044b0,
            (0, false) => 0xa04438,
            (_, true) => 0xa045e4,
            _ => 0xa04528,
        });
        return;
    }
    for (bit, texts) in EXPECTATION_TEXTS {
        if bit == 0x20 && !f(0x20) {
            say(0xa03038);
            return;
        }
        if f(bit) {
            say(texts[col][late]);
            return;
        }
    }
}

/// Comparer for `build_board_expectation_text`: side by side over the buffer (the size the caller gives, at most 0x1388).
pub(crate) unsafe extern "thiscall" fn compare_build_board_expectation_text(this: usize, club: usize, flags: u32, buf: usize, size: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize, u32)>(IDX_BUILD_BOARD_EXPECTATION_TEXT);
    let n = (size as usize).clamp(1, 0x1388);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side_light(&[(buf, n)], || o(this, club, flags, buf, size), || build_board_expectation_text(this, club, flags, buf, size));
    SCOREBOARD.record(igual, || format!("build_board_expectation_text(flags {:#x}) {:?} x {:?}", flags,
        String::from_utf8_lossy(&mo.0[0].1[..0x80.min(n)]), String::from_utf8_lossy(&mr.0[0].1[..0x80.min(n)])));
}

/// Writes the headline or body of a board's season expectations.
///
/// `MANAGER_MANAGER::news_board_expectations_text(news, body, out)`: the news slots 0 (club id) and 1 (the flags,
/// `exe::newsitem_get_slot`). Body: the expectation sentence (`build_board_expectation_text`, through the exe).
/// Headline (`FN_LANGLIB_SPRINTF` of a fixed text): a national team (id above clubs − 2 × nations) "F.A. Announce
/// Expectations"; flag 0x400 "Board expect glory", 2 "want the title", 0x85 "hope for successful season", 0x30
/// "expecting difficult season", 0x308 "look forward to new season", otherwise "announce expectations". Then the text
/// into `out` (`exe::text_set_string`) and the club's link (`exe::text_add_link`, 0x7d5, name `club::NAME`), also on
/// the headline.
/// Original: `MANAGER_MANAGER::news_board_expectations_text` (0x6a4ae0, manager_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn news_board_expectations_text(this: usize, news: usize, body: u32, out: usize) {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let mut text = vec![0u8; 0x1388];
    let t = text.as_mut_ptr() as usize;
    let id = exe::newsitem_get_slot(news, 0);
    let flags = exe::newsitem_get_slot(news, 1);
    let club = Club(read_usize(G_CLUB_TABLE).wrapping_add((id as usize).wrapping_mul(club::SIZE)));
    if body != 0 {
        covered!("sma_6 expectations body");
        let build: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32) = core::mem::transmute(board_text_fns::BUILD_BOARD_EXPECTATION_TEXT);
        build(this, club.addr(), flags, t, 0x1388);
    } else {
        let h = if id as i32 > read_i32(G_CLUB_COUNT) - 2 * read_i32(G_NATION_COUNT) {
            0xa0abcc
        } else if flags & 0x400 != 0 {
            0xa0abb8
        } else if flags & 2 != 0 {
            0xa0aba0
        } else if flags & 0x85 != 0 {
            0xa0ab7c
        } else if flags & 0x30 != 0 {
            0xa0ab58
        } else if flags & 0x308 != 0 {
            0xa0ab34
        } else {
            0xa0ab18
        };
        format(t, h); // sma6: the expectations headline
    }
    exe::text_set_string(out, t as *const u8);
    exe::text_add_link(out, (club.addr() + club::NAME) as *const u8, 0x7d5, club);
}

/// Comparer for `news_board_expectations_text` (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_news_board_expectations_text(this: usize, news: usize, body: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(IDX_NEWS_BOARD_EXPECTATIONS_TEXT);
    compare_text_writer(out, || o(this, news, body, out), |t| news_board_expectations_text(this, news, body, t),
        || format!("news_board_expectations_text(body {})", body));
}

/// Writes the headline or body of the board's (or FA's) confidence update, with links to the players it names.
///
/// `MANAGER_MANAGER::news_board_confidence_update_text(news, body, out)`: slots 0 (club id) and 1 (the flags).
/// Headline: "Football Association Confidence Update" for a national team (`exe::database_is_national_team`), "Board
/// Confidence Update" otherwise. Body: the confidence sentence (0x690d80) and the named players' sentence (0x693850,
/// up to 15 ids, returns how many); with players, both joined by "%s\n\n%s", otherwise the first one. The text into
/// `out` (`exe::text_set_string`); each named player (id not −1) whose formatted name (`exe::database_format_staff_name`,
/// 0x96, style 1) is in the text (`exe::crt_strstr`) gets a link (0x7db); the club gets its link (0x7d5) when its name
/// is in the text.
/// Original: `MANAGER_MANAGER::news_board_confidence_update_text` (0x6a4d40, manager_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn news_board_confidence_update_text(this: usize, news: usize, body: u32, out: usize) {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let mut ids = [-1i32; 15];
    let mut name = [0u8; 0x98];
    let mut text = vec![0u8; 0x1388];
    let mut first = vec![0u8; 0x1388];
    let mut players = vec![0u8; 0x5dc];
    let t = text.as_mut_ptr() as usize;
    let id = exe::newsitem_get_slot(news, 0);
    let flags = exe::newsitem_get_slot(news, 1);
    let club = Club(read_usize(G_CLUB_TABLE).wrapping_add((id as usize).wrapping_mul(club::SIZE)));
    let mut count: i32 = 0;
    if body == 0 {
        format(t, if exe::database_is_national_team(club) != 0 { 0xa0ac00 } else { 0xa0abe8 });
    } else {
        covered!("sma_6 confidence body");
        let conf: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32) = core::mem::transmute(board_text_fns::BUILD_BOARD_CONFIDENCE_TEXT);
        conf(this, club.addr(), flags, first.as_mut_ptr() as usize, 0x1388);
        let named: unsafe extern "thiscall" fn(usize, usize, usize, *mut i32) -> u8 = core::mem::transmute(board_text_fns::BUILD_BOARD_NAMED_PLAYERS_TEXT);
        count = named(this, players.as_mut_ptr() as usize, club.addr(), ids.as_mut_ptr()) as i8 as i32;
        if count != 0 {
            sprintf(t, 0x991948, first.as_ptr() as usize, players.as_ptr() as usize); // sma6: two sentences
        } else {
            sprintf(t, 0x991944, first.as_ptr() as usize);
        }
    }
    exe::text_set_string(out, t as *const u8);
    for &p in ids.iter().take(count.max(0) as usize) {
        if p == -1 {
            continue;
        }
        let person = read_usize(G_STAFF_TABLE).wrapping_add((p as usize).wrapping_mul(staff::SIZE));
        exe::database_format_staff_name(person, name.as_mut_ptr() as usize, 0x96, 1);
        if exe::crt_strstr(t, name.as_ptr() as usize) != 0 {
            exe::text_add_link(out, name.as_ptr(), 0x7db, person);
        }
    }
    if exe::crt_strstr(t, club.addr() + club::NAME) != 0 {
        exe::text_add_link(out, (club.addr() + club::NAME) as *const u8, 0x7d5, club);
    }
}

/// Comparer for `news_board_confidence_update_text` (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_news_board_confidence_update_text(this: usize, news: usize, body: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(IDX_NEWS_BOARD_CONFIDENCE_UPDATE_TEXT);
    compare_text_writer(out, || o(this, news, body, out), |t| news_board_confidence_update_text(this, news, body, t),
        || format!("news_board_confidence_update_text(body {})", body));
}

/// Writes the headline or body of the news that a club appointed or promoted someone to be its manager.
///
/// `news_manager_appointment_text(news, body, out)`: the slots 0 (the person's id), 1..3 (the name ids), 4 (the old
/// club id, −1 none), 5 (the club id), 6 (the news code: 1 player, 2 staff, 3 manager) and 7 (the old job). The name
/// (`manager_news_name`: style 5 in the headline, 1 in the body). Headline: 0xa0a3a8 with the club's short gender and
/// short name (`club::SHORT_GENDER_NAME`, `club::SHORT_NAME`) and the name; links: the person (0x7da) when the name
/// ids still match (`exe::database_staff_matches_names`) and the club (0x7d5). Body (formats into 0xde1f64,
/// `FN_LANGLIB_SPRINTF`, with the genders `club::GENDER_NAME` and names `club::NAME`): no old club 0xa09e18; code 1: the
/// same club 0xa09f14, otherwise 0xa09e78 (both clubs); code 2: without a club job (`.job_for_club()`) 0xa09f8c, at the
/// same club by the old job: 0 0xa0a184, 11 0xa0a1f4, other 0xa0a274 with the job's name and gender
/// (`database_get_job_name`); from another club: 11 0xa0a0e4, other 0xa09fe4; code 3: 0xa0a2f8; any other code leaves
/// the buffer as it was (the original's). Links: the person when the ids match (0x7dc for a national team club, 0x7d7
/// otherwise), the club and the old club (0x7d5).
/// Original: `news_manager_appointment_text` (0x6a3060, manager_manager.cpp, stdcall, ret 0xc).
pub(crate) unsafe extern "stdcall" fn news_manager_appointment_text(news: usize, body: u32, out: usize) {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let s: [u32; 8] = core::array::from_fn(|k| exe::newsitem_get_slot(news, k as u32));
    let id = s[0];
    let ids = [s[1], s[2], s[3]];
    let person = read_usize(G_STAFF_TABLE).wrapping_add((id as usize).wrapping_mul(staff::SIZE));
    let club_at = |k: u32| read_usize(G_CLUB_TABLE).wrapping_add((k as usize).wrapping_mul(club::SIZE));
    let new = Club(club_at(s[5]));
    let mut name = [0u8; 0x190];
    let nm = name.as_ptr() as usize;
    let buf = 0xde1f64usize;
    if body == 0 {
        manager_news_name(news, id, ids, &mut name, 5);
        let g = (new.short_gender_name() as i8) as i32;
        format(buf, 0xa0a3a8, g, g, new.addr() + club::SHORT_NAME, nm);
        exe::text_set_string(out, buf as *const u8);
        if exe::database_staff_matches_names(person, ids[0], ids[1], ids[2], 0usize) != 0 {
            exe::text_add_link(out, name.as_ptr(), 0x7da, person);
        }
        exe::text_add_link(out, (new.addr() + club::NAME) as *const u8, 0x7d5, new);
        return;
    }
    covered!("sma_7 appointment body");
    manager_news_name(news, id, ids, &mut name, 1);
    let old_id = s[4] as i32;
    let old = Club(club_at(s[4]));
    let gn = new.gender_name() as i32;
    let go = || old.gender_name() as i32;
    let (newn, oldn) = (new.addr() + club::NAME, old.addr() + club::NAME);
    if old_id == -1 {
        format(buf, 0xa09e18, gn, gn, newn, nm);
    } else {
        let job_name = |gender: &mut i8| {
            // 0x53ad00 writes the gender only in language 4: the original formats with a stack byte otherwise
            if exe::langlib_get_language() != 4 {
                PRAISE_JOB_GENDER_UNSET.store(true, Relaxed);
            }
            let f: unsafe extern "C" fn(u32, u32, u32, *mut i8) -> usize = core::mem::transmute(manager_text_fns::GET_JOB_NAME);
            f(s[7] as u8 as u32, 0, 0, gender)
        };
        match s[6] as u8 as i8 {
            1 => {
                if s[5] == s[4] {
                    format(buf, 0xa09f14, gn, gn, newn, nm);
                } else {
                    format(buf, 0xa09e78, gn, gn, go(), go(), newn, oldn, nm);
                }
            }
            2 => {
                let old_job = s[7] as u8;
                if Staff(person).job_for_club() == 0 {
                    format(buf, 0xa09f8c, gn, gn, newn, nm);
                } else if s[5] == s[4] {
                    match old_job {
                        0 => {
                            format(buf, 0xa0a184, gn, gn, newn, nm);
                        }
                        0xb => {
                            format(buf, 0xa0a1f4, gn, gn, newn, nm);
                        }
                        _ => {
                            let mut gt: i8 = 0;
                            let title = job_name(&mut gt);
                            let gt = gt as i32;
                            format(buf, 0xa0a274, gn, gn, gt, gt, newn, title, nm); // sma7: promoted at the club
                        }
                    }
                } else {
                    let mut gt: i8 = 0;
                    let title = job_name(&mut gt);
                    let gt = gt as i32;
                    if old_job == 0xb {
                        format(buf, 0xa0a0e4, gn, gn, go(), go(), gt, gt, newn, oldn, title, nm);
                    } else {
                        format(buf, 0xa09fe4, gn, gn, go(), go(), gt, gt, go(), go(), gt, gt, newn, oldn, title, nm, oldn, title);
                    }
                }
            }
            3 => {
                format(buf, 0xa0a2f8, gn, gn, go(), go(), newn, nm, oldn);
            }
            _ => {}
        }
    }
    exe::text_set_string(out, buf as *const u8);
    if exe::database_staff_matches_names(person, ids[0], ids[1], ids[2], 0usize) != 0 {
        let kind = if s[5] as i32 > -1 && (s[5] as i32) < read_i32(G_CLUB_COUNT) && exe::database_is_national_team(new) != 0 { 0x7dc } else { 0x7d7 };
        exe::text_add_link(out, name.as_ptr(), kind, person);
    }
    exe::text_add_link(out, newn as *const u8, 0x7d5, new);
    if old_id != -1 {
        exe::text_add_link(out, oldn as *const u8, 0x7d5, old);
    }
}

/// Comparer for `news_manager_appointment_text` (`compare_text_writer`).
pub(crate) unsafe extern "stdcall" fn compare_news_manager_appointment_text(news: usize, body: u32, out: usize) {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, u32, usize)>(IDX_NEWS_MANAGER_APPOINTMENT_TEXT);
    // the old job's title gender is the original's unset stack byte outside language 4 (PRAISE_JOB_GENDER_UNSET)
    PRAISE_JOB_GENDER_UNSET.store(false, Relaxed);
    compare_text_writer_tolerant(out, || o(news, body, out), |t| news_manager_appointment_text(news, body, t),
        || format!("news_manager_appointment_text(body {})", body), || PRAISE_JOB_GENDER_UNSET.load(Relaxed));
}

/// Writes the headline or body of a story about board pressure on a manager: a shortlist for his job, a rumour, or the
/// FA's view of a national team manager.
///
/// `news_manager_under_pressure_text(news, body, out)`: slots 0 (the manager's id), 1..3 (his name ids), 4 (the club
/// id), 5..7 (the chairman's name ids, 5 = −1 without one), 8 (the reason, 1..4), 9 (the kind: 1 rumour), 10 (the
/// chairman's id), 11 (the rumour). The names (`exe::database_format_name_from_ids`, style 5 in the headline, 1 in the
/// body), the club's name (`club::SHORT_NAME` in the headline, `club::NAME` in the body, through "%s") and its
/// nation's adjective (`exe::nation_get_nationality_adjective`, 100, 9). A national team with a nation (`club::NATION`)
/// takes the FA texts (genders 100). Headline: kind 1 0xa06ff8 / FA 0xa07054; otherwise (after 0xa06f98 when there is
/// no reason, a chairman name and no rumour: written and then overwritten, as in the original) 0xa06ed8 / FA 0xa06f38.
/// Body: kind 1: FA 0xa06e50; rumour 1 0xa06d18, other rumour 0xa06dc0; none 0xa06bec. Reason 1..4: with a chairman
/// 0xa0669c, 0xa06444, 0xa068f0, 0xa06b28 (and his name); without one the club text (0xa064f8, 0xa062f4, 0xa06778,
/// 0xa069b8) or the FA one (0xa065c4, 0xa06394, 0xa0682c, 0xa06a68). Other reasons: without a chairman 0xa06058 / FA
/// 0xa060d4; with a rumour 0xa06160; otherwise 0xa061f0. Then the text (`exe::text_set_string`), the manager's link
/// (0x7d7) when the ids match (`exe::database_staff_matches_names`) and the club's link (0x7d5) on the adjective (FA,
/// body) or the club name.
/// Original: `news_manager_under_pressure_text` (0x6a1190, manager_manager.cpp, stdcall, ret 0xc).
pub(crate) unsafe extern "stdcall" fn news_manager_under_pressure_text(news: usize, body: u32, out: usize) {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let s: [u32; 12] = core::array::from_fn(|k| exe::newsitem_get_slot(news, k as u32));
    let style = if body != 0 { 1 } else { 5 };
    let (mut name, mut chair, mut club_name, mut adj) = ([0u8; 0x190], [0u8; 0x190], [0u8; 0x190], [0u8; 0x64]);
    exe::database_format_name_from_ids(s[1], s[2], s[3], name.as_mut_ptr(), 0x190, style, s[0]);
    if s[5] as i32 == -1 {
        chair[0] = 0;
    } else {
        exe::database_format_name_from_ids(s[5], s[6], s[7], chair.as_mut_ptr(), 0x190, style, s[10]);
    }
    let club = Club(read_usize(G_CLUB_TABLE).wrapping_add((s[4] as usize).wrapping_mul(club::SIZE)));
    sprintf(club_name.as_mut_ptr() as usize, 0x991944, club.addr() + if body != 0 { club::NAME } else { club::SHORT_NAME });
    exe::nation_get_nationality_adjective(club.nation().addr(), adj.as_mut_ptr() as usize, 0x64, 9);
    let (nm, ch, cl, ad) = (name.as_ptr() as usize, chair.as_ptr() as usize, club_name.as_ptr() as usize, adj.as_ptr() as usize);
    let fa = exe::database_is_national_team(club) != 0 && club.nation().addr() != 0;
    let buf = 0xde1f64usize;
    let (reason, kind, rumour) = (s[8] as u8, s[9] as u8, s[11]);
    if body == 0 {
        let g = (club.short_gender_name() as i8) as i32;
        if kind == 1 {
            if fa {
                format(buf, 0xa07054, 100, 100, 100, ad, nm);
            } else {
                format(buf, 0xa06ff8, g, g, 100, cl, nm);
            }
        } else {
            if reason == 0 && s[5] != 0 && rumour == 0 {
                format(buf, 0xa06f98, g, g, cl, nm);
            }
            if fa {
                format(buf, 0xa06f38, 100, 100, ad, nm);
            } else {
                format(buf, 0xa06ed8, g, g, cl, nm);
            }
        }
    } else {
        covered!("sma_7 pressure body");
        let g = club.gender_name() as i32;
        if kind == 1 {
            if fa {
                format(buf, 0xa06e50, 100, 100, ad, nm);
            } else if rumour != 0 {
                format(buf, if rumour == 1 { 0xa06d18 } else { 0xa06dc0 }, g, g, cl, nm);
            } else {
                format(buf, 0xa06bec, g, g, cl, nm, nm);
            }
        } else if (1..=4).contains(&reason) {
            // (with the chairman, the club, the FA) by the reason
            let t = [[0xa0669c, 0xa064f8, 0xa065c4], [0xa06444, 0xa062f4, 0xa06394], [0xa068f0, 0xa06778, 0xa0682c], [0xa06b28, 0xa069b8, 0xa06a68]][reason as usize - 1];
            if s[5] as i32 != -1 {
                format(buf, t[0], g, g, 100, cl, ch, nm); // sma7: the reason with the chairman
            } else if fa {
                format(buf, t[2], 100, 100, 100, ad, nm);
            } else {
                format(buf, t[1], g, g, 100, cl, nm);
            }
        } else if s[5] as i32 == -1 {
            if fa {
                format(buf, 0xa060d4, 100, 100, 100, ad, nm);
            } else {
                format(buf, 0xa06058, g, g, 100, cl, nm);
            }
        } else if rumour != 0 {
            format(buf, 0xa06160, g, g, 100, cl, ch, nm);
        } else {
            format(buf, 0xa061f0, g, g, nm, cl, ch);
        }
    }
    exe::text_set_string(out, buf as *const u8);
    let person = read_usize(G_STAFF_TABLE).wrapping_add((s[0] as usize).wrapping_mul(staff::SIZE));
    if exe::database_staff_matches_names(person, s[1], s[2], s[3], 0usize) != 0 {
        exe::text_add_link(out, name.as_ptr(), 0x7d7, person);
    }
    if body != 0 && fa {
        exe::text_add_link(out, adj.as_ptr(), 0x7d5, club);
    } else {
        exe::text_add_link(out, club_name.as_ptr(), 0x7d5, club);
    }
}

/// Comparer for `news_manager_under_pressure_text` (`compare_text_writer`).
pub(crate) unsafe extern "stdcall" fn compare_news_manager_under_pressure_text(news: usize, body: u32, out: usize) {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, u32, usize)>(IDX_NEWS_MANAGER_UNDER_PRESSURE_TEXT);
    compare_text_writer(out, || o(news, body, out), |t| news_manager_under_pressure_text(news, body, t),
        || format!("news_manager_under_pressure_text(body {})", body));
}

/// The non-player sheet of the club's chairman (0 without chairman or sheet).
pub(crate) unsafe fn board_week_chairman_sheet(club: usize) -> usize {
    let club = Club(club);
    let c = club.chairman();
    if c.is_null() { 0 } else { c.non_player().addr() }
}

/// The club loop of `weekly_board_confidence_update` from the given stale league size; returns the last N.
pub(crate) unsafe fn board_week_run(this: usize, league_size: i8) -> i8 {
    let mut s = BoardWeek {
        this, index: 0, club: 0, manager: 0, sheet: 0, confidence_before: 0, league_size,
        told_finals_hopes: false, told_finals_doubts: false, entry: 0, division_object: 0,
        proj_best: 0, proj_normal: 0, proj_worst: 0, x: 0.0, weight: 0.0,
    };
    let base = read_usize(G_CLUB_TABLE);
    while (s.index as i32) < read_i32(G_CLUB_COUNT) {
        s.club = base.wrapping_add(s.index.wrapping_mul(club::SIZE));
        board_week_club(&mut s);
        s.index += 1;
    }
    s.league_size
}

/// The weekly pull of the supporters' confidence towards the board's: the supporters (`board_record::SUPPORTERS_CONFIDENCE`)
/// kept within half and double the board's (`board_record::CONFIDENCE`), then the board's moves by 10% of the gap
/// (confidence − truncate((supporters − confidence) × −0.1)); the change also goes to `board_record::FAVOURITES_MOOD`.
pub(crate) unsafe fn decay_board_week(s: &mut BoardWeek) {
    let row = s.row();
    let confidence = read_i16(row + board_record::CONFIDENCE) as i32;
    let supporters = read_i16(row + board_record::SUPPORTERS_CONFIDENCE) as i32;
    covered!("gra_17 decay");
    let half = confidence / 2;
    let v = if half > supporters { half } else if confidence * 2 < supporters { confidence * 2 } else { supporters };
    write_i16(row + board_record::SUPPORTERS_CONFIDENCE, v as i16);
    let row = s.row();
    let supporters = read_i16(row + board_record::SUPPORTERS_CONFIDENCE) as i32;
    s.confidence_before = read_i16(row + board_record::CONFIDENCE);
    let d = ftol((supporters - s.confidence_before as i32) as f64 * read_f64(rdata::F64_NEG_0_1));
    write_i16(row + board_record::CONFIDENCE, s.confidence_before.wrapping_sub(d as i16));
    s.add(board_record::FAVOURITES_MOOD, s.confidence().wrapping_sub(s.confidence_before));
}

/// The national team's finals stories: only the nation's main national team (`exe::database_get_national_team`), on a
/// draw(10) of 0, with its current competition (`exe::nationalteams_get_current_competition`) a finals one
/// (`exe::national_teams_is_finals_comp`) with an object, starting (`Comp::get_first_schedule_date`) more than 10 and
/// less than 100 days from today. Returns false otherwise (the club goes to the league week). With it, nothing more
/// without the competition's statistics, the team's entry or with a match played; N = the competition's entrant count
/// (low byte). Once per call each: with the best position 1 and the previous final position below N / 5 − draw(3) + 2,
/// story 0x5a (the hopes; the person by `board_week_story_person` with a draw(4)); otherwise, with the final position
/// above draw(N² / 200 + 1) + 1 and below N / 5 + N / 10 + 2 and the team's reputation above 350 × the competition's,
/// story 0x44 (the doubts).
pub(crate) unsafe fn national_finals_stories(s: &mut BoardWeek) -> bool {
    let club = Club(s.club);
    if exe::database_get_national_team(club.nation().addr()) != club.addr() || random(10) != 0 {
        return false;
    }
    let comp = exe::nationalteams_get_current_competition(read_usize(G_NATIONAL_TEAMS), club.nation().addr());
    if exe::national_teams_is_finals_comp(read_usize(G_NATIONAL_TEAMS), comp) == 0 || board_week_comp_object(comp) == 0 {
        return false;
    }
    let first_schedule = |out: &mut [u32; 2]| -> usize {
        exe::comp_get_first_schedule_date(board_week_comp_object(comp), out.as_mut_ptr() as usize)
    };
    let (mut soon, mut first) = ([0u32; 2], [0u32; 2]);
    let d = exe::date_add_days(G_CURRENT_DATE, soon.as_mut_ptr() as usize, 10);
    let soon_copy = [read_u32(d), read_u32(d + 4)];
    if board_week_date_order(soon_copy.as_ptr() as usize, first_schedule(&mut first)) >= 0 {
        return false;
    }
    let (mut later, mut first2) = ([0u32; 2], [0u32; 2]);
    let d = exe::date_add_days(G_CURRENT_DATE, later.as_mut_ptr() as usize, 100);
    let later_copy = [read_u32(d), read_u32(d + 4)];
    if board_week_date_order(later_copy.as_ptr() as usize, first_schedule(&mut first2)) <= 0 {
        return false;
    }
    let stats = read_usize(board_week_comp_object(comp) + league::COMP_STATS);
    if stats == 0 {
        return true;
    }
    let entry = exe::comp_stats_find_club_entry(stats, club);
    if entry == 0 || read_u8(entry + comp_stats_club::MATCHES_PLAYED) != 0 {
        return true;
    }
    let n = read_i8(board_week_comp_object(comp) + league::ENTRANT_COUNT);
    s.league_size = n;
    let n = n as i32;
    if !s.told_finals_hopes && read_u8(entry + comp_stats_club::BEST_POSITION) == 1 {
        let r = random(3);
        if (read_u8(entry + comp_stats_club::FINAL_POSITION) as i32) < n / 5 - r + 2 {
            covered!("gra_17 finals hopes");
            let person = board_week_story_person(club.addr(), 4);
            board_week_post_story(club.addr(), 0x5a, person, -1);
            s.told_finals_hopes = true;
            return true;
        }
    }
    if s.told_finals_doubts {
        return true;
    }
    let last = read_u8(entry + comp_stats_club::FINAL_POSITION) as i32;
    if last <= random(n * n / 200 + 1) + 1 || last >= n / 5 + n / 10 + 2
        || club.reputation() as i32 <= read_i16(comp + competition::REPUTATION) as i32 * 350
    {
        return true;
    }
    covered!("gra_17 finals doubts");
    let person = board_week_story_person(club.addr(), 4);
    board_week_post_story(club.addr(), 0x44, person, -1);
    s.told_finals_doubts = true;
    true
}

/// The league week: nothing without a division with an object, its statistics (`league::COMP_STATS`) and the club's
/// entry (`exe::comp_stats_find_club_entry`). The projected positions (`exe::comp_stats_get_club_projected_positions`:
/// best, normal, worst), the form factor (`board_week_form_factor`), with best ≠ worst in the entry the form scaled by
/// the position against the previous final position, otherwise (no game played) the pre-season stories
/// (`preseason_board_stories`); the chairman's patience and the new manager's grace (`board_week_chairman_and_grace`);
/// then the change goes in (`apply_board_week_form`) and, with best ≠ worst and outside the new manager's grace, the
/// position against the target moves the confidence (`board_week_position`).
pub(crate) unsafe fn league_board_week(s: &mut BoardWeek) {
    let club = Club(s.club);
    let division = club.division().addr();
    if division == 0 || board_week_comp_object(division) == 0 {
        return;
    }
    let stats = read_usize(board_week_comp_object(division) + league::COMP_STATS);
    if stats == 0 {
        return;
    }
    s.entry = exe::comp_stats_find_club_entry(stats, club);
    if s.entry == 0 {
        return;
    }
    let stats = read_usize(board_week_comp_object(club.division().addr()) + league::COMP_STATS);
    exe::comp_stats_get_club_projected_positions(stats, club, &mut s.proj_best, &mut s.proj_normal, &mut s.proj_worst);
    s.x = board_week_form_factor(s);
    let division = club.division().addr();
    let division_rep = read_i16(division + competition::REPUTATION);
    let club_rep = club.reputation();
    if division_rep != 0 && club_rep != 0 {
        if division_rep > 7 {
            let q = ((division_rep as i32 + 1) * 500) as f64 / club_rep as i32 as f64;
            let q = if q > read_f64(rdata::F64_1_25) {
                read_f32(rdata::F32_1_25) as f64
            } else if q < read_f64(rdata::F64_0_25) {
                read_f32(rdata::F32_0_25) as f64
            } else {
                q
            };
            let x = s.x as f64;
            s.x = if x < 0.0 && (q < read_f64(rdata::F64_1) || q > read_f64(rdata::F64_1)) { (x / q) as f32 } else { (q * x) as f32 };
        }
        if s.x < 0.0 && club.professional_status() != 1 {
            s.x = (s.x as f64 * read_f64(rdata::F64_0_75)) as f32;
        }
        let e = s.entry;
        if read_u8(e + comp_stats_club::BEST_POSITION) != read_u8(e + comp_stats_club::WORST_POSITION) {
            let (current, last) = (read_u8(e + comp_stats_club::CURRENT_POSITION), read_u8(e + comp_stats_club::FINAL_POSITION));
            if current < last && s.x < 0.0 {
                s.x = (s.x as f64 * read_f64(rdata::F64_0_05)) as f32;
            } else if current == last && s.x < 0.0 {
                s.x = (s.x as f64 * read_f64(rdata::F64_0_1)) as f32;
            } else if current > last && s.x > 0.0 {
                s.x = (s.x as f64 * read_f64(rdata::F64_0_25)) as f32;
            }
        } else if read_u8(e + comp_stats_club::MATCHES_PLAYED) == 0 {
            preseason_board_stories(s);
        }
        board_week_chairman_and_grace(s);
    }
    apply_board_week_form(s);
    board_week_position(s);
}

/// The end of every club's pass: with a continental competition other than the Intertoto Cup
/// (`KEY_CLUB_COMP_INTER_TOTO_CUP`; the club read again by index from `G_CLUB_TABLE`) the board at least 6500, or
/// 4275 when European success is expected (`expectation_flags_byte1::EUROPEAN_SUCCESS`) and the projection is below
/// the target (`board_record_flags::PROJECTED_BELOW_TARGET`); the board within 0..10000; at most 5250 when the manager
/// is disliked by the chairman (`exe::staff_is_disliked_by`), else at least 4250 when favoured
/// (`exe::staff_is_favoured_by`); the supporters within 0..10000, at most 3000 when the club dislikes the manager
/// (`exe::club_dislikes_staff`), else at least 7000 when it favours him (`exe::club_favours_staff`). Without the flags
/// 1, 4 and 8 of `board_record::FLAGS`: the manager's reputations (`update_board_week_reputation`) and, outside
/// national teams, the wealth band (`board_week_wealth_band`).
pub(crate) unsafe fn board_week_floors_and_reputation(s: &mut BoardWeek) {
    let club = Club(s.club);
    let row = s.row();
    let european = read_u16(row + board_record::EXPECTATION_FLAGS) & ((expectation_flags_byte1::EUROPEAN_SUCCESS as u16) << 8);
    let continental = read_i32(read_usize(G_CLUB_TABLE).wrapping_add(s.index.wrapping_mul(club::SIZE)) + club::CONTINENTAL_COMP_ID) != -1
        && (club.continental_comp_id() as i32) != read_i32(KEY_CLUB_COMP_INTER_TOTO_CUP);
    if continental {
        let floor = if european != 0 && read_u8(row + board_record::FLAGS) & board_record_flags::PROJECTED_BELOW_TARGET != 0 { 4275 } else { 6500 };
        if read_i16(row + board_record::CONFIDENCE) < floor {
            write_i16(row + board_record::CONFIDENCE, floor);
        }
    }
    let conf = s.confidence();
    let change = conf.wrapping_sub(s.confidence_before);
    if conf > 10000 {
        s.set_confidence(10000);
    } else if conf < 0 {
        s.set_confidence(0);
    }
    let chairman = club.chairman().addr();
    if exe::staff_is_disliked_by(s.manager, chairman) != 0 {
        s.set_confidence(s.confidence().min(5250));
    } else if exe::staff_is_favoured_by(s.manager, chairman) != 0 {
        s.set_confidence(s.confidence().max(4250));
    }
    let a = s.row() + board_record::SUPPORTERS_CONFIDENCE;
    let v = read_i16(a);
    if v > 10000 {
        write_i16(a, 10000);
    } else if v < 0 {
        write_i16(a, 0);
    }
    if exe::club_dislikes_staff(s.manager, club) != 0 {
        let a = s.row() + board_record::SUPPORTERS_CONFIDENCE;
        write_i16(a, read_i16(a).min(3000));
    } else if exe::club_favours_staff(s.manager, club) != 0 {
        let a = s.row() + board_record::SUPPORTERS_CONFIDENCE;
        write_i16(a, read_i16(a).max(7000));
    }
    let flags = read_u8(s.row() + board_record::FLAGS);
    if flags & (board_record_flags::NEW_MANAGER | board_record_flags::PROJECTED_ABOVE_TARGET | board_record_flags::PROJECTED_BELOW_TARGET) != 0 {
        return;
    }
    let mut d = change;
    if d < 0 {
        let rep = club.reputation();
        d = if rep > 5000 {
            ftol(read_f64(board_week_rdata::F64_2500) / rep as i32 as f64 * d as i32 as f64)
        } else {
            ftol(d as i32 as f64 * read_f64(rdata::F64_0_5))
        } as i16;
    }
    update_board_week_reputation(s, d);
    if exe::database_is_national_team(club) == 0 {
        board_week_wealth_band(s);
    }
}

pub(crate) unsafe fn board_story_flat(regions: &[(usize, usize)]) -> Vec<u8> {
    let mut v = Vec::with_capacity(regions.iter().map(|r| r.1).sum());
    for &(a, n) in regions {
        v.extend_from_slice(core::slice::from_raw_parts(a as *const u8, n));
    }
    v
}

pub(crate) unsafe fn board_story_write(regions: &[(usize, usize)], flat: &[u8]) {
    let mut k = 0;
    for &(a, n) in regions {
        core::ptr::copy_nonoverlapping(flat.as_ptr().add(k), a as *mut u8, n);
        k += n;
    }
}

/// The first difference between two flat copies, by region (index, offset).
pub(crate) fn board_story_first_difference(regions: &[(usize, usize)], a: &[u8], b: &[u8]) -> Option<(usize, usize, u8, u8)> {
    let mut k = 0;
    for (i, &(_, n)) in regions.iter().enumerate() {
        if let Some(o) = (0..n).find(|&o| a[k + o] != b[k + o]) {
            return Some((i, o, a[k + o], b[k + o]));
        }
        k += n;
    }
    None
}

/// The recorder the comparer puts at the story entry (0x74b060) while the original runs: tapes the state before, calls
/// the hooked story and tapes what changed.
pub(crate) unsafe extern "thiscall" fn board_story_recorder(this: usize, club: usize, kind: u32, person: usize, game: usize, variant: u32, extra: u32) {
    let tape = (*core::ptr::addr_of_mut!(BOARD_STORY_TAPE)).as_mut().unwrap();
    let f: unsafe extern "thiscall" fn(usize, usize, u32, usize, usize, u32, u32) = core::mem::transmute(tape.target);
    if tape.depth > 0 || !tape.recording {
        return f(this, club, kind, person, game, variant, extra);
    }
    tape.depth += 1;
    let before = board_story_flat(&tape.regions);
    f(this, club, kind, person, game, variant, extra);
    let tape = (*core::ptr::addr_of_mut!(BOARD_STORY_TAPE)).as_mut().unwrap();
    tape.depth -= 1;
    let after = board_story_flat(&tape.regions);
    let mut changes = Vec::new();
    let mut k = 0;
    for &(a, n) in &tape.regions {
        for o in 0..n {
            if before[k + o] != after[k + o] {
                changes.push((a + o, after[k + o]));
            }
        }
        k += n;
    }
    tape.calls.push(BoardStoryCall { args: [club as u32, kind, person as u32, extra], before, changes });
}

/// Calls the original as the game would and returns the byte it finds at its league size local ([esp+0x2b] of its
/// frame: the aligned frame base − 0x2b1), read just before the call.
pub(crate) unsafe fn board_week_call_original(f: usize, this: usize) -> u8 {
    let mut residue: u32 = 0;
    core::arch::asm!(
        "lea eax, [esp - 8]",
        "and eax, -8",
        "movzx eax, byte ptr [eax - 0x2b1]",
        "mov dword ptr [edi], eax",
        "call edx",
        in("edi") &mut residue as *mut u32,
        in("edx") f,
        in("ecx") this,
        clobber_abi("C"),
    );
    residue as u8
}

/// The departing manager's name of a departure news into `name` (0x190 bytes): a human manager (id ≥ `G_STAFF_COUNT`
/// − 16) from the two strings of the news (slot 13, then the slot whose index slot 12 holds; `get_slot_string`, 0x33
/// each; `exe::database_format_name_from_strings`, style 1, −1); anyone else from the three name ids (`ids`,
/// `exe::database_format_name_from_ids`, style `style`).
pub(crate) unsafe fn departure_news_name(news: usize, id: u32, ids: [u32; 3], name: &mut [u8; 0x190], style: u32) {
    if id as i32 >= read_i32(G_STAFF_COUNT) - 0x10 {
        let get: unsafe extern "thiscall" fn(usize, u32, *mut u8, i32) -> u32 = core::mem::transmute(manager_text_fns::GET_SLOT_STRING);
        let (mut first, mut second) = ([0u8; 0x34], [0u8; 0x34]);
        get(news, 0xd, first.as_mut_ptr(), 0x33);
        get(news, exe::newsitem_get_slot(news, 0xc), second.as_mut_ptr(), 0x33);
        exe::database_format_name_from_strings(first.as_ptr() as usize, second.as_ptr() as usize, 0usize, name.as_mut_ptr() as usize, 0x190, 1, -1);
    } else {
        exe::database_format_name_from_ids(ids[0], ids[1], ids[2], name.as_mut_ptr(), 0x190, style, id);
    }
}

/// `v` × (`a` × k + 1), truncated (`ftol`): one attribute weight of the score (k read from the exe at `k`).
pub(crate) unsafe fn staff_job_weight(v: i32, a: i8, k: usize) -> i32 {
    ftol((a as i32 as f64 * read_f64(k) + read_f64(rdata::F64_1)) * v as i32 as f64)
}

/// The person's four attitudes (loyalty, determination, pressure, professionalism), each × ((value − 10) × k + 1),
/// truncated after each.
pub(crate) unsafe fn staff_job_attitudes(p: Staff, mut v: i32, k: usize) -> i32 {
    for a in [p.loyalty(), p.determination(), p.pressure(), p.professionalism()] {
        v = staff_job_weight(v, a.wrapping_sub(10), k);
    }
    v
}

/// The similarity with the viewer: `v` × 1.5 (truncated), then for each of `STAFF_JOB_SIMILARITY` × (1 − |difference|
/// × k) (`ks[i]`, an exe address), or, with `div` (the manager jobs), × (1 − |difference| / (F × div[i])), truncated
/// after each.
pub(crate) unsafe fn staff_job_similarity(v: i32, np: usize, viewer_np: usize, ks: &[usize; 9], div: Option<(f64, [usize; 9])>) -> i32 {
    let mut v = ftol(v as f64 * read_f64(rdata::F64_1_5));
    for (i, &a) in STAFF_JOB_SIMILARITY.iter().enumerate() {
        let d = ((read_i8(viewer_np + a) as i32) - (read_i8(np + a) as i32)).abs() as f64;
        let s = match div {
            Some((f, ds)) => read_f64(rdata::F64_1) - d / (f * read_f64(ds[i])),
            None => read_f64(rdata::F64_1) - d * read_f64(ks[i]),
        };
        v = ftol(v as f64 * s);
    }
    v
}

/// `max(v × k, v + 2000)` truncated: the bonus of a favourite (k = 1.1) or of a national team's own nation (k = 1.2).
pub(crate) unsafe fn staff_job_bonus(v: i32, k: usize) -> i32 {
    let a = v as f64 * read_f64(k);
    let b = v.wrapping_add(2000) as f64;
    ftol(if a > b { a } else { b })
}

/// Takes the person out of the old job before the move: the player data from the old club's squad
/// (`exe::manager_manager_remove_player_from_squad`) and the staff data from its staff
/// (`exe::manager_manager_remove_staff_from_club`), by the old role, and returns the code of the appointment news
/// (1 player, 2 staff, 3 manager). Without the data the role needs, the game's error box with the line of the
/// original (`show_error_at`). `lines` are the error lines of the club or of the national team version.
pub(crate) unsafe fn appoint_leave_old_job(this: usize, p: Staff, old: Club, role: u8, national: bool) -> u32 {
    use appoint_fns::MANAGER_MANAGER_CPP as FILE;
    let staff_out = |line: u32| {
        if !p.non_player().is_null() {
            exe::manager_manager_remove_staff_from_club(this, p, old);
        } else {
            show_error_at(FILE, line);
        }
    };
    let player_out = |line: u32| {
        if !p.player().is_null() {
            exe::manager_manager_remove_player_from_squad(this, p, old);
        } else {
            show_error_at(FILE, line);
        }
    };
    let l = |club: u32, nation: u32| if national { nation } else { club };
    match role {
        5 => {
            staff_out(l(0x34c2, 0x3526));
            3
        }
        6 | 8 => {
            staff_out(l(0x34b9, 0x351d));
            2
        }
        11 => {
            player_out(l(0x34a0, 0x3504));
            1
        }
        12 => {
            player_out(l(0x34a9, 0x350d));
            staff_out(l(0x34af, 0x3513));
            if national { 2 } else { 3 } // the player-manager of a club is news 3, of a national team 2
        }
        15 => {
            player_out(l(0x3491, 0x34f5));
            staff_out(l(0x3497, 0x34fb));
            2
        }
        _ => {
            if !p.non_player().is_null() {
                exe::manager_manager_remove_staff_from_club(this, p, old);
            }
            if !p.player().is_null() {
                exe::manager_manager_remove_player_from_squad(this, p, old);
            }
            2
        }
    }
}

/// The manager's name of a manager news into `name` (0x190 bytes): a human manager (id ≥ `G_STAFF_COUNT` − 16) from
/// the two strings of the news (slot 8, then the slot whose index slot 7 holds; `get_slot_string`, 0x33 each;
/// `exe::database_format_name_from_strings`, style 1, −1); anyone else from the three name ids (`ids`,
/// `exe::database_format_name_from_ids`, style `style`).
pub(crate) unsafe fn manager_news_name(news: usize, id: u32, ids: [u32; 3], name: &mut [u8; 0x190], style: u32) {
    if id as i32 >= read_i32(G_STAFF_COUNT) - 0x10 {
        let get: unsafe extern "thiscall" fn(usize, u32, *mut u8, i32) -> u32 = core::mem::transmute(manager_text_fns::GET_SLOT_STRING);
        let (mut first, mut second) = ([0u8; 0x34], [0u8; 0x34]);
        get(news, 8, first.as_mut_ptr(), 0x33);
        get(news, exe::newsitem_get_slot(news, 7), second.as_mut_ptr(), 0x33);
        exe::database_format_name_from_strings(first.as_ptr() as usize, second.as_ptr() as usize, 0usize, name.as_mut_ptr() as usize, 0x190, 1, -1);
    } else {
        exe::database_format_name_from_ids(ids[0], ids[1], ids[2], name.as_mut_ptr(), 0x190, style, id);
    }
}

/// The competition object of a competition record: [[G_COMP_OBJECT_TABLE] + record id × 4].
pub(crate) unsafe fn board_week_comp_object(record: usize) -> usize {
    read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add((read_i32(record) as usize).wrapping_mul(4)))
}

/// Who comments a board story: on a draw(n) other than 0 a former international (`pick_former_international_pundit`),
/// otherwise a rival's manager (`pick_rival_manager_id`, none when below 0).
pub(crate) unsafe fn board_week_story_person(club: usize, n: i32) -> usize {
    if random(n) != 0 {
        let f: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_MEDIA_PICK_FORMER_INTERNATIONAL_PUNDIT);
        f(read_usize(G_MEDIA), club, 0)
    } else {
        let f: unsafe extern "thiscall" fn(usize, usize) -> i32 = core::mem::transmute(FN_MEDIA_PICK_RIVAL_MANAGER_ID);
        let id = f(read_usize(G_MEDIA), club);
        if id > -1 { staff_at(id as isize).addr() } else { 0 }
    }
}

/// Posts a board story about the club (`Media::post_team_media_story(club, kind, person, 0, 0, extra)`); in the
/// comparer's replay it checks the call against the original's tape and applies the story's recorded effects instead.
pub(crate) unsafe fn board_week_post_story(club: usize, kind: u32, person: usize, extra: i32) {
    if board_week_replay_story(club, kind, person, extra) {
        return;
    }
    exe::media_post_team_media_story(read_usize(G_MEDIA), club, kind, person as u32, 0, 0, extra);
}

/// One club of the weekly update (see `weekly_board_confidence_update`).
pub(crate) unsafe fn board_week_club(s: &mut BoardWeek) {
    let club = Club(s.club);
    if exe::database_is_national_team(club) != 0 {
        let nation = club.nation();
        if nation.is_null() || (nation.continent() as u32) == 0 {
            return;
        }
    }
    let manager = club.manager();
    if manager.is_null() || manager.non_player().addr() == 0 || exe::club_can_be_managed(club) == 0 {
        return;
    }
    s.manager = manager.addr();
    s.sheet = manager.non_player().addr();
    decay_board_week(s);
    if !national_finals_stories(s) {
        league_board_week(s);
    }
    board_week_floors_and_reputation(s);
}

/// Compares two dates by year (unsigned) then day (signed), as the original's inline tests: −1, 0 or 1.
pub(crate) unsafe fn board_week_date_order(a: usize, b: usize) -> i32 {
    let (ya, yb) = (read_u16(a + 2), read_u16(b + 2));
    if ya != yb {
        return if ya < yb { -1 } else { 1 };
    }
    let (da, db) = (read_i16(a), read_i16(b));
    if da < db { -1 } else if da > db { 1 } else { 0 }
}

/// The form factor x (f32): the current run in the league, counted only when its last game was less than 7 days ago
/// (`exe::date_days_since`): wins above 1 (× 25), defeats above 1 (× −30), winless above 2 (× −5) or unbeaten above 2
/// (× 5), in that order; the run n weighs n² below 4, otherwise min(n, 5)² × 0.2 + 9. A negative x is scaled by the
/// division's reputation × 0.1 (within 0.25..2) and, outside national teams, with the squad investment
/// (`board_record::SQUAD_INVESTMENT`) above the spending (`board_record::SPENDING`), by their ratio (at most 2.5), the
/// transfer mood (`board_record::TRANSFER_MOOD`) losing 150.
pub(crate) unsafe fn board_week_form_factor(s: &mut BoardWeek) -> f32 {
    let e = s.entry;
    let recent = |date: usize| exe::date_days_since(G_CURRENT_DATE, read_u32(e + date), read_u32(e + date + 4)) < 7;
    let (n, weight): (i16, i32) = if read_i32(e + comp_stats_club::WINNING_RUN) > 1 && recent(board_week_fields::WINNING_RUN_DATE) {
        covered!("gra_17 winning run");
        (read_i16(e + comp_stats_club::WINNING_RUN), 25)
    } else if read_i32(e + comp_stats_club::LOSING_RUN) > 1 && recent(board_week_fields::LOSING_RUN_DATE) {
        (read_i16(e + comp_stats_club::LOSING_RUN), -30)
    } else if read_i32(e + comp_stats_club::WINLESS_RUN) > 2 && recent(board_week_fields::WINLESS_RUN_DATE) {
        (read_i16(e + comp_stats_club::WINLESS_RUN), -5)
    } else if read_i32(e + comp_stats_club::UNBEATEN_RUN) > 2 && recent(board_week_fields::UNBEATEN_RUN_DATE) {
        (read_i16(e + comp_stats_club::UNBEATEN_RUN), 5)
    } else {
        (0, 0)
    };
    let f = if n >= 4 {
        let m = n.min(5) as i32;
        (m * m) as f64 * read_f64(rdata::F64_0_2) + read_f64(board_week_rdata::F64_9)
    } else {
        (n as i32 * n as i32) as f64
    };
    let mut x = (weight as f64 * f) as f32;
    if x < 0.0 {
        let m = read_i16(read_usize(s.club + club::DIVISION) + competition::REPUTATION) as i32 as f64 * read_f64(rdata::F64_0_1);
        let m = if m > read_f64(rdata::F64_2) {
            read_f32(rdata::F32_2) as f64
        } else if m < read_f64(rdata::F64_0_25) {
            read_f32(rdata::F32_0_25) as f64
        } else {
            m
        };
        x = (m * x as f64) as f32;
        if exe::database_is_national_team(s.club) == 0 {
            let row = s.row();
            if read_f64(row + board_record::SQUAD_INVESTMENT) > read_f64(row + board_record::SPENDING) {
                let r = read_f64(row + board_record::SQUAD_INVESTMENT) / read_f64(row + board_record::SPENDING);
                let r = if r > read_f64(rdata::F64_2_5) { read_f32(rdata::F32_2_5) as f64 } else { r };
                s.add(board_record::TRANSFER_MOOD, -150);
                x = (r * x as f64) as f32;
            }
        }
    }
    x
}

/// The pre-season stories (no game played, best = worst in the entry), with the division in a selected nation's league
/// (`nation::LEAGUE_SELECTED`), a manager and a top division (`league::UPPER_DIVISION_ID` = −1). N is the stale league
/// size (the defect, see `weekly_board_confidence_update`); positions are the previous final position. With the
/// position below N / 4 or the reputation above 6500 + draw(2000): the spending story (`board_week_spending_story`).
/// Otherwise, below N / 5 − draw(3) + 1: story 0x5d (title favourites); otherwise, above 1 and below N / 5 + N / 10:
/// story 0x42 (European hopes); the person by `board_week_story_person` with a draw(2).
pub(crate) unsafe fn preseason_board_stories(s: &mut BoardWeek) {
    let club = Club(s.club);
    let in_league = || {
        let nation = club.nation();
        club.division().addr() != 0 && !nation.is_null() && nation.league_selected() != 0
            && club.manager().addr() != 0
            && read_i32(board_week_comp_object(club.division().addr()) + league::UPPER_DIVISION_ID) == -1
    };
    let last = read_u8(s.entry + comp_stats_club::FINAL_POSITION) as i32;
    let n = s.league_size as i32;
    if in_league() && (last < n / 4 || club.reputation() as i32 > random(2000) + 6500) {
        board_week_spending_story(s);
        return;
    }
    if in_league() && last < n / 5 - random(3) + 1 {
        covered!("gra_17 preseason favourites");
        let person = board_week_story_person(club.addr(), 2);
        board_week_post_story(club.addr(), 0x5d, person, -1);
        return;
    }
    if in_league() && last > 1 && last < n / 5 + n / 10 {
        covered!("gra_17 preseason european hopes");
        let person = board_week_story_person(club.addr(), 2);
        board_week_post_story(club.addr(), 0x42, person, -1);
    }
}

/// The chairman's patience on a negative x (the patience reset to 10 below 1; factor 4 at 1, 0.25 at 20, otherwise
/// (21 − patience) × 0.1 within 0.5..2); without a chairman's sheet, and after it, a negative x with the board's
/// new-manager flag (`board_record_flags::NEW_MANAGER`) is × 0.2 in the manager's first 180 days, otherwise × 0.75.
pub(crate) unsafe fn board_week_chairman_and_grace(s: &mut BoardWeek) {
    let club = Club(s.club);
    let sheet = board_week_chairman_sheet(club.addr());
    if sheet != 0 {
        if !(s.x < 0.0) {
            return;
        }
        if read_i8(sheet + non_player::PATIENCE) < 1 {
            write_u8(sheet + non_player::PATIENCE, 10);
        }
        let m = board_week_patience_factor(read_i8(board_week_chairman_sheet(club.addr()) + non_player::PATIENCE), rdata::F32_4);
        s.x = (m * s.x as f64) as f32;
    }
    let manager = club.manager();
    if s.x < 0.0 && !manager.is_null() && read_u8(s.row() + board_record::FLAGS) & board_record_flags::NEW_MANAGER != 0 {
        let days = exe::date_days_since(G_CURRENT_DATE, manager.date_joined_club() as u32, read_u32(manager.addr() + staff::DATE_JOINED_CLUB + 4));
        s.x = (s.x as f64 * if days < 180 { read_f64(rdata::F64_0_2) } else { read_f64(rdata::F64_0_75) }) as f32;
    }
}

/// The form goes in: d = truncate(x) on the board (`board_record::CONFIDENCE`), the supporters and the season's league
/// confidence (`board_record::LEAGUE_CONFIDENCE`), truncate(x × 1.25) on `board_record::FAVOURITES_MOOD`. Then N = the
/// division's club count (`league::CLUB_COUNT`, low byte) and the position weight w = (22 / N, or 1 without clubs),
/// with a division of reputation 10+ and the club's reputation above 500 × it, × ((rep − 500 × div) + (rep − 5000)) ×
/// 0.0002 + 1; w = w^1.5 (`game_pow`).
pub(crate) unsafe fn apply_board_week_form(s: &mut BoardWeek) {
    let d = ftol(s.x as f64) as i16;
    s.add(board_record::CONFIDENCE, d);
    s.add(board_record::SUPPORTERS_CONFIDENCE, d);
    s.add(board_record::FAVOURITES_MOOD, ftol(s.x as f64 * read_f64(rdata::F64_1_25)) as i16);
    s.add(board_record::LEAGUE_CONFIDENCE, d);
    let division = read_usize(s.club + club::DIVISION);
    s.division_object = board_week_comp_object(division);
    s.league_size = read_i8(s.division_object + league::CLUB_COUNT);
    let mut w = if s.league_size > 0 { read_f64(board_week_rdata::F64_22) / s.league_size as f64 } else { 1.0 };
    let division_rep = read_i16(division + competition::REPUTATION) as i32;
    let club_rep = read_i16(s.club + club::REPUTATION) as i32;
    if division_rep >= 10 && club_rep > division_rep * 500 {
        w = (((club_rep - division_rep * 500) as f64 + (club_rep - 5000) as f64) * read_f64(rdata::F64_0_0002) + read_f64(rdata::F64_1)) * w;
    }
    s.weight = game_pow(w, read_f64(rdata::F64_1_5));
}

/// The position against the target (only with best ≠ worst in the entry and without the new-manager flag): with even
/// the worst projection better than the previous final position, the flag `board_record_flags::PROJECTED_ABOVE_TARGET`
/// is set with +750 (already set: +750 on the board below 4000); with even the best one worse, the flag
/// `board_record_flags::PROJECTED_BELOW_TARGET` with −750 (already set: −750 above 6500); otherwise up to 4 games
/// played +5 above the final position, +1 on it and −5 below; after that the league stories
/// (`league_board_stories`) and the expectation (`board_week_expectation`). The change goes in by
/// `apply_board_week_position`.
pub(crate) unsafe fn board_week_position(s: &mut BoardWeek) {
    let e = s.entry;
    if read_u8(e + comp_stats_club::BEST_POSITION) == read_u8(e + comp_stats_club::WORST_POSITION) {
        return;
    }
    let flags = read_u8(s.row() + board_record::FLAGS);
    if flags & board_record_flags::NEW_MANAGER != 0 {
        return;
    }
    let last = read_u8(e + comp_stats_club::FINAL_POSITION);
    let delta: i32 = if read_u8(e + comp_stats_club::WORST_POSITION) < last {
        if flags & board_record_flags::PROJECTED_ABOVE_TARGET == 0 {
            write_u8(s.row() + board_record::FLAGS, flags | board_record_flags::PROJECTED_ABOVE_TARGET);
            750
        } else {
            if s.confidence() < 4000 {
                s.set_confidence(s.confidence().wrapping_add(750));
            }
            0
        }
    } else if read_u8(e + comp_stats_club::BEST_POSITION) > last {
        if flags & board_record_flags::PROJECTED_BELOW_TARGET == 0 {
            write_u8(s.row() + board_record::FLAGS, flags | board_record_flags::PROJECTED_BELOW_TARGET);
            -750
        } else {
            if s.confidence() > 6500 {
                s.set_confidence(s.confidence().wrapping_sub(750));
            }
            0
        }
    } else if read_u8(e + comp_stats_club::MATCHES_PLAYED) <= 4 {
        covered!("gra_17 early weeks");
        let current = read_u8(e + comp_stats_club::CURRENT_POSITION);
        if current < last { 5 } else if current == last { 1 } else { -5 }
    } else {
        league_board_stories(s);
        board_week_expectation(s)
    };
    apply_board_week_position(s, delta as i16);
}

/// The manager's reputations (`non_player::CURRENT_REPUTATION`, `non_player::HOME_REPUTATION`,
/// `non_player::WORLD_REPUTATION`) move with the board's change d (16 bits; a fall scaled by 2500 / reputation above
/// 5000, otherwise halved): targets = reputation + d, pulled within the club's reputation ± 3000 (world: −5000..+1000).
/// Each one's speed n = its band (`board_week_band`) − 19; for the home one 23 − club reputation / 500 when the manager
/// is foreign to the club's nation (`staff::NATION`), and for the world one 23 − club reputation / 500 outside national
/// teams (n 0 counts as 1). New = ((n − 1) × old + target) / n, kept within old − f × 250 and old + f × 300 (home:
/// 375; world: ± f × 500 rounded to f32) with f = 2 − old × 0.0001; capped at the club's reputation (or 50 × its
/// division's, the larger) within 5000..10000 but not below the old value (world: 2/3 of it within 3000..10000), and at
/// least 1. Each reputation becomes new / 2 + old / 2.
pub(crate) unsafe fn update_board_week_reputation(s: &mut BoardWeek, d: i16) {
    let club = Club(s.club);
    let sheet = s.sheet;
    let home = read_i16(sheet + non_player::HOME_REPUTATION);
    let current = read_i16(sheet + non_player::CURRENT_REPUTATION);
    let world = read_i16(sheet + non_player::WORLD_REPUTATION);
    let club_rep = club.reputation() as i32;
    let current_target = board_week_pull(current.wrapping_add(d) as i32, club_rep - 3000, club_rep + 3000);
    let home_target = board_week_pull(home.wrapping_add(d) as i32, club_rep - 3000, club_rep + 3000);
    let world_target = board_week_pull(world.wrapping_add(d) as i32, club_rep - 5000, club_rep + 1000);
    covered!("gra_17 manager reputation");
    let (current_band, home_band, world_band) = (board_week_band(current), board_week_band(home), board_week_band(world));
    let nonzero = |n: i16| if n == 0 { 1 } else { n };
    let n_current = nonzero((current_band as i16).wrapping_sub(19));
    let foreign = exe::database_is_national_team(club) == 0
        && (club.nation().addr() as u32) != read_u32(s.manager + staff::NATION);
    let n_home = nonzero(if foreign { (23 - club.reputation() as i32 / 500) as i16 } else { (home_band as i16).wrapping_sub(19) });
    let n_world = nonzero(if exe::database_is_national_team(club) != 0 {
        (world_band as i16).wrapping_sub(19)
    } else {
        (23 - club.reputation() as i32 / 500) as i16
    });
    let step = |n: i16, old: i16, target: i32| ((n as i32 - 1).wrapping_mul(old as i32).wrapping_add(target as i16 as i32)) / n as i32;
    let new_current = step(n_current, current, current_target) as i16;
    let new_home = step(n_home, home, home_target) as i16;
    let new_world = step(n_world, world, world_target) as i16;
    let limit = |old: i16, new: i16, f: f32, down: f64, up: f64| {
        let o = old as i32 as f32 as f64;
        let nf = new as i32 as f32 as f64;
        let lo = o - f as f64 * down;
        if lo > nf {
            return ftol(lo);
        }
        let hi = f as f64 * up + o;
        if hi < nf { ftol(hi) } else { ftol(nf) }
    };
    let factor = |v: i16| read_f64(rdata::F64_2) - v as i32 as f64 * read_f64(rdata::F64_0_0001);
    let final_current = limit(current, new_current, factor(current) as f32, read_f32(rdata::F32_250) as f64, read_f32(rdata::F32_300) as f64) as i16;
    let final_home = limit(home, new_home, factor(home) as f32, read_f32(rdata::F32_250) as f64, read_f32(rdata::F32_375) as f64) as i16;
    let k = (factor(world) * read_f32(rdata::F32_500) as f64) as f32 as f64;
    let wf = world as i32 as f32 as f64;
    let nwf = new_world as i32 as f32 as f64;
    let final_world = if wf - k > nwf { ftol(wf - k) } else if k + wf < nwf { ftol(k + wf) } else { ftol(nwf) } as i16;
    let mut cap_rep = club.reputation();
    let division = club.division().addr();
    if division != 0 {
        let e = read_i16(division + competition::REPUTATION) as i32 * 50;
        if e > cap_rep as i32 {
            cap_rep = e as i16;
        }
    }
    let band = |v: i16| if v < 5000 { 5000 } else if v > 10000 { 10000 } else { v };
    let cap = |c: i16, old: i16, v: i16| {
        let c = if c > old { c } else { old };
        let v = if v > c { c } else { v };
        if v < 1 { 1 } else { v }
    };
    let final_home = cap(band(cap_rep), home, final_home);
    let final_current = cap(band(cap_rep), current, final_current);
    let w = cap_rep as i32 as f64 * read_f64(rdata::F64_0_6666666667);
    let w = if !(read_f64(board_week_rdata::F64_3000) <= w) {
        read_f64(board_week_rdata::F64_3000)
    } else if read_f64(rdata::F64_10000) < w {
        read_f64(rdata::F64_10000)
    } else {
        w
    };
    let final_world = cap(ftol(w) as i16, world, final_world);
    write_i16(sheet + non_player::CURRENT_REPUTATION, (final_current as i32 / 2 + current as i32 / 2) as i16);
    write_i16(sheet + non_player::HOME_REPUTATION, (final_home as i32 / 2 + home as i32 / 2) as i16);
    write_i16(sheet + non_player::WORLD_REPUTATION, (final_world as i32 / 2 + world as i32 / 2) as i16);
}

/// The wealth band (`exe::finance_get_wealth_band` of `exe::financetable_get_club_finance`) against the one the board
/// took at the season's start (`board_record::WEALTH_BAND`, row by club id), on the board and on
/// `board_week_fields::FINANCE_MOOD`: band 2 below 2 → +35 (reputation below 3500) or +15; band 1 below 1 → +30 or
/// +10; band 0: from below 0 → +25 or +5, from above → −5; band −1: from below 0 → +5, otherwise −50 / −125 for a plc
/// (`club::PLC`) or −25 / −50; band −2: from −2 → +5, otherwise −75 / −175 (plc) or −50 / −75; another band shows the
/// manager_manager.cpp box (line 0x28aa). Then the board within 0..10000.
pub(crate) unsafe fn board_week_wealth_band(s: &mut BoardWeek) {
    let club = Club(s.club);
    let finance = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club);
    if finance != 0 {
        let band = exe::finance_get_wealth_band(finance) as i8;
        let taken = read_i8(read_usize(s.this).wrapping_add((read_i32(club.addr()) as usize).wrapping_mul(0x49)) + board_record::WEALTH_BAND);
        let low = club.reputation() < 3500;
        let plc = club.plc() != 0;
        let (board, mood): (i16, i16) = match band {
            2 => if taken >= 2 { (0, 0) } else if low { (35, 35) } else { (15, 15) },
            1 => if taken >= 1 { (0, 0) } else if low { (30, 30) } else { (10, 10) },
            0 => if taken < 0 { if low { (25, 25) } else { (5, 5) } } else if taken > 0 { (-5, -5) } else { (0, 0) },
            -1 => if taken < 0 { (5, 5) } else if plc { (-50, -125) } else { (-25, -50) },
            -2 => if taken == -2 { (5, 5) } else if plc { (-75, -175) } else { (-50, -75) },
            _ => {
                show_error_at(0x9f18a4, 0x28aa);
                (0, 0)
            }
        };
        if board != 0 || mood != 0 {
            covered!("gra_17 wealth band");
            s.add(board_record::CONFIDENCE, board);
            s.add(board_week_fields::FINANCE_MOOD, mood);
        }
    }
    let conf = s.confidence();
    if conf > 10000 {
        s.set_confidence(10000);
    } else if conf < 0 {
        s.set_confidence(0);
    }
}

/// The chairman's patience factor: 1 → `one`, 20 → 0.25, otherwise (21 − patience) × 0.1 kept within 0.5..2.
pub(crate) unsafe fn board_week_patience_factor(patience: i8, one: usize) -> f64 {
    if patience == 1 {
        return read_f32(one) as f64;
    }
    if patience == 20 {
        return read_f32(rdata::F32_0_25) as f64;
    }
    let m = (21 - patience as i32) as f64 * read_f64(rdata::F64_0_1);
    if m > read_f64(rdata::F64_2) {
        read_f32(rdata::F32_2) as f64
    } else if m < read_f64(rdata::F64_0_5) {
        read_f32(rdata::F32_0_5) as f64
    } else {
        m
    }
}

/// The spending story: the club's transfers (`TRANSFER_MANAGER::get_past_transfers`) from the later of today − 100
/// days and the manager's arrival (`staff::DATE_JOINED_CLUB`) to the current year: what it paid minus what it got;
/// each purchase above 2,500,000 + draw(5,000,000) is a big one, and above 7,500,000 + draw(5,000,000) its player is
/// the star (the last such). With more than draw(2) + 1 big ones and the net above 15,000,000 + draw(10,000,000):
/// story 0x5b with the net as the extra (the star only with at least draw(2) + 2 big ones).
pub(crate) unsafe fn board_week_spending_story(s: &mut BoardWeek) {
    let club = Club(s.club);
    let mut since = [0u32; 2];
    let d = exe::date_sub_days(G_CURRENT_DATE, since.as_mut_ptr() as usize, 100);
    let mut start = [read_u32(d), read_u32(d + 4)];
    let joined = club.manager().addr() + staff::DATE_JOINED_CLUB;
    let joined_copy = [read_u32(joined), read_u32(joined + 4)];
    if board_week_date_order(joined_copy.as_ptr() as usize, start.as_ptr() as usize) > 0 {
        let mut tmp = [0u32; 2];
        exe::date_operator_3d(start.as_mut_ptr() as usize, tmp.as_mut_ptr() as usize, joined);
    }
    let start_year = read_u32(start.as_ptr() as usize + 2);
    let year = read_u16(G_CURRENT_DATE + 2) as u32;
    let (mut list, mut count) = (0usize, 0i32);
    let today_month = exe::date_get_month(G_CURRENT_DATE) as u32;
    let start_month = exe::date_get_month(start.as_ptr() as usize) as u32;
    exe::transfer_manager_get_past_transfers(read_usize(G_TRANSFER_MANAGER), start_month, start_year, today_month, year, &mut list, &mut count);
    let (mut net, mut big, mut star) = (0i32, 0i32, 0usize);
    let id = read_u32(club.addr());
    for k in 0..count.max(0) as usize {
        let t = list + k * 0x25;
        if read_u32(t + 0x10) == id {
            let fee = read_i32(t + 0x18);
            net = net.wrapping_add(fee);
            if fee > random(5_000_000) + 2_500_000 {
                big += 1;
            }
            if read_i32(t + 0x18) > random(5_000_000) + 7_500_000 {
                star = staff_at(read_i32(t) as isize).addr();
            }
        } else if read_u32(t + 0x14) == id {
            net = net.wrapping_sub(read_i32(t + 0x18));
        }
    }
    if list != 0 {
        exe::crt_free(list);
    }
    if big > random(2) + 1 && net > random(10_000_000) + 15_000_000 {
        covered!("gra_17 spending story");
        if big < random(2) + 2 {
            star = 0;
        }
        board_week_post_story(club.addr(), 0x5b, star, net);
    }
}

/// The league stories (from the 5th game; at most one, in this order; "in a league" = division, a selected nation's
/// league, a manager; top = `league::UPPER_DIVISION_ID` −1; N = the division's club count; positions: current and final
/// of the entry, best/normal/worst projected; "runs" = wins above draw(a) + 2 or unbeaten above draw(b) + 4, "bad runs"
/// = defeats above draw(a) + 2 or winless above draw(b) + 4; the person by `board_week_story_person` with a draw(2)
/// unless "none"):
/// 0x5f below the top, 10 + draw(10) to 25 + draw(5) games, current below 4, final below 6 − draw(3), best within the
/// promotion and play-off places, worst not 1, runs (5, 5), draw(3) of 0; 0x60 more than 20 + draw(10) games, wins
/// above 1 or unbeaten above draw(3) + 2, a league with relegation (`league_vtable::HAS_RELEGATION`), current − draw(3)
/// + draw(3) above N − relegated, best below N − N / 2 − relegated, worst at least N − relegated; 0x43 more than
/// 10 + draw(200) games, unbeaten and winless below 3, (draw(6) × 0.1 + 1.6) × games above the entry's two stat bytes,
/// current − draw(4) + draw(4) = N / 2, best above N / 5, worst − draw(2) + draw(2) below N − relegated; at the top:
/// 0x63 (current 2..3, final 4..5) and 0x61 (current 2..3, final above 5), 10 + draw(10) to 25 + draw(5) games, best 1,
/// worst not 1, runs (5, 5); 0x62 10 + draw(10) to 29 games, current, final and best 1, worst not 1, runs (3, 4);
/// below the top 0x40, 25 + draw(10) games, current 2..8, final below 7, best within and worst outside the promotion
/// places, bad runs (3, 4); at the top 0x41, 25 + draw(10) games, current below 6, final below 5, normal 2 or 3, best
/// 1, worst not 1, bad runs (3, 4); 0x5e (none) reputation below 7250 + draw(2000), more than 5 + draw(10) and fewer
/// than 25 − draw(5) games, current above the final and below 5, best 1, worst not 1, wins non-zero and above 4 or
/// unbeaten above 7; 0x3e (none) current below the final, 5 + draw(10) to 25 − draw(5) games, defeats non-zero and above
/// 4 or winless above 7, relegation, current at least N − relegated and best above it; 0x3f more than 15 + draw(15)
/// games, defeats above 1 or winless above draw(3) + 2, relegation, draw(2) + current above N − relegated, normal at
/// least that, best below it and above N + N / 2 − relegated (never: the original's test); 0x3c (none, −250 on
/// `board_record::FAVOURITES_MOOD`) more than 20 games, final − current below −5, best below the final, the club
/// disliking the manager (`exe::club_dislikes_staff`) or a draw(4) of 0, the supporters (row by club id) below 4275,
/// no continental competition, defeats above draw(4) + 2 or winless above draw(7) + 3.
pub(crate) unsafe fn league_board_stories(s: &mut BoardWeek) {
    let club = Club(s.club);
    let e = s.entry;
    let played = read_u8(e + comp_stats_club::MATCHES_PLAYED) as i32;
    let current = read_u8(e + comp_stats_club::CURRENT_POSITION) as i32;
    let last = read_u8(e + comp_stats_club::FINAL_POSITION) as i32;
    let (best, worst) = (s.proj_best as i32, s.proj_worst as i32);
    let division = || club.division().addr();
    let object = || board_week_comp_object(division());
    let nation_ok = || {
        let nation = club.nation();
        !nation.is_null() && nation.league_selected() != 0 && club.manager().addr() != 0
    };
    let run = |field: usize| read_i32(e + field);
    let n = || s.league_size as i32;
    let relegated = || read_i8(object() + league::RELEGATED_COUNT) as i32;
    let promotion_places = || read_i8(object() + league::PROMOTION_PLAYOFF_COUNT) as i32 + read_i8(object() + league::PROMOTED_COUNT) as i32;
    let story = |kind: u32| {
        let person = board_week_story_person(club.addr(), 2);
        board_week_post_story(club.addr(), kind, person, -1);
    };
    // 0x5f: the promotion push
    if division() != 0 && nation_ok() && read_i32(s.division_object + league::UPPER_DIVISION_ID) != -1
        && played >= random(10) + 10 && played < random(5) + 25 && current < 4 && last < 6 - random(3)
        && best <= promotion_places() && worst != 1
        && (run(comp_stats_club::WINNING_RUN) > random(5) + 2 || run(comp_stats_club::UNBEATEN_RUN) > random(5) + 4)
        && random(3) == 0
    {
        covered!("gra_17 story 0x5f");
        return story(0x5f);
    }
    if division() == 0 {
        return board_week_story_0x63(s);
    }
    // 0x60: relegation trouble
    if nation_ok() && played > random(10) + 20
        && (run(comp_stats_club::WINNING_RUN) > 1 || run(comp_stats_club::UNBEATEN_RUN) > random(3) + 2)
        && call_comp_virtual(object(), league_vtable::HAS_RELEGATION) != 0
    {
        let safe = n() - relegated();
        let v = current - random(3);
        let v = v + random(3);
        if v > safe && best < n() - n() / 2 - relegated() && worst >= safe {
            covered!("gra_17 story 0x60");
            return story(0x60);
        }
    }
    // 0x43: the steady mid-table
    if division() != 0 && nation_ok() && played > random(200) + 10
        && run(comp_stats_club::UNBEATEN_RUN) < 3 && run(comp_stats_club::WINLESS_RUN) < 3
    {
        let stat = (read_u8(e + board_week_fields::STAT_6A) as i32 + read_u8(e + board_week_fields::STAT_69) as i32) as f64;
        let r = random(6);
        if (r as f64 * read_f64(rdata::F64_0_1) + read_f64(rdata::F64_1_6)) * played as f64 > stat {
            let a = current - random(4);
            let a = a + random(4);
            if a == n() / 2 && best > n() / 5 {
                let b = worst - random(2);
                let b = b + random(2);
                if b < n() - relegated() {
                    covered!("gra_17 story 0x43");
                    return story(0x43);
                }
            }
        }
    }
    board_week_story_0x63(s)
}

/// The expectation against the previous final position (from the 5th game, after the stories): below it (better)
/// by more than 4, +15 a place + 40 (the board raised to 6500 instead, with no change, when below it); otherwise +10 a
/// place (raised to 4275 instead); on it, +5 (below 4275: raised to 4275 instead on a draw(patience) other than 0, or a
/// draw(10) without a chairman's sheet); above it (worse) by `board_week_overshoot`.
pub(crate) unsafe fn board_week_expectation(s: &mut BoardWeek) -> i32 {
    let e = s.entry;
    let (current, last) = (read_u8(e + comp_stats_club::CURRENT_POSITION), read_u8(e + comp_stats_club::FINAL_POSITION));
    if current < last {
        let d = (last as i16).wrapping_sub(current as i16);
        if d > 4 {
            if s.confidence() < 6500 {
                s.set_confidence(6500);
                return 0;
            }
            return d as i32 * 15 + 40;
        }
        if s.confidence() < 4275 {
            s.set_confidence(4275);
            return 0;
        }
        return d as i32 * 10;
    }
    if current == last {
        if s.confidence() < 4275 && board_week_patience_draw(s.club, 10) {
            s.set_confidence(4275);
            return 0;
        }
        return 5;
    }
    board_week_overshoot(s, current, last)
}

/// The position change goes in: d = truncate(change × the division's reputation × 0.1, within 0.5..2); for a club of
/// reputation above 5000 and above 400 × the division's, with final × w − current × w below −4, the final position
/// below 5 / w and d negative: d = truncate(min(2d, games × gap × 1.5)). A negative d × the chairman's patience factor
/// (patience outside 1..20 reset to 10; 3 at 1, 0.25 at 20). Outside national teams: with the squad investment above
/// the spending, a negative d × their ratio (at most 3) and the transfer mood −75; otherwise a positive d × 1.05 and the
/// transfer mood +5. A negative d × 0.75 for a club not fully professional, and again for a Welsh or Irish club (the
/// nations `KEY_NATION_WALES`, `KEY_NATION_REPUBLIC_OF_IRELAND`, `KEY_NATION_NORTHERN_IRELAND`). With a continental
/// competition and d negative, board and supporters lose truncate(d × −0.25); otherwise the board takes d and the
/// supporters truncate(d × 1.05).
pub(crate) unsafe fn apply_board_week_position(s: &mut BoardWeek, delta: i16) {
    let club = Club(s.club);
    let e = s.entry;
    let division_rep = read_i16(club.division().addr() + competition::REPUTATION) as i32;
    let m = division_rep as f64 * read_f64(rdata::F64_0_1);
    let m = if m > read_f64(rdata::F64_2) {
        read_f32(rdata::F32_2) as f64
    } else if m < read_f64(rdata::F64_0_5) {
        read_f32(rdata::F32_0_5) as f64
    } else {
        m
    };
    let mut d = ftol(delta as i32 as f64 * m) as i16;
    let club_rep = club.reputation() as i32;
    if club_rep > 5000 && club_rep > division_rep * 400 {
        let (last, current) = (read_u8(e + comp_stats_club::FINAL_POSITION) as i32, read_u8(e + comp_stats_club::CURRENT_POSITION) as i32);
        let gap = last as f64 * s.weight - current as f64 * s.weight;
        if gap < read_f64(rdata::F64_NEG_4) && (last as f64) < read_f64(rdata::F64_5) / s.weight && d < 0 {
            let a = (d as i32 * 2) as f64;
            let u = read_u8(e + comp_stats_club::MATCHES_PLAYED) as i32 as f64 * gap * read_f64(rdata::F64_1_5);
            d = ftol(if a < u { a } else { u }) as i16;
        }
    }
    let sheet = board_week_chairman_sheet(club.addr());
    if sheet != 0 && d < 0 {
        let p = read_i8(sheet + non_player::PATIENCE);
        if p < 1 || p > 20 {
            write_u8(sheet + non_player::PATIENCE, 10);
        }
        let f = board_week_patience_factor(read_i8(board_week_chairman_sheet(club.addr()) + non_player::PATIENCE), rdata::F32_3);
        d = ftol(d as i32 as f64 * f) as i16;
    }
    if exe::database_is_national_team(club) == 0 {
        let row = s.row();
        if read_f64(row + board_record::SQUAD_INVESTMENT) > read_f64(row + board_record::SPENDING) {
            if d < 0 {
                let r = read_f64(row + board_record::SQUAD_INVESTMENT) / read_f64(row + board_record::SPENDING);
                let r = if r > read_f64(rdata::F64_3) { read_f32(rdata::F32_3) as f64 } else { r };
                d = ftol(d as i32 as f64 * r) as i16;
            }
            s.add(board_record::TRANSFER_MOOD, -75);
        } else if d > 0 {
            d = ftol(d as i32 as f64 * read_f64(rdata::F64_1_05)) as i16;
            s.add(board_record::TRANSFER_MOOD, 5);
        }
    }
    if d < 0 {
        if club.professional_status() != 1 {
            d = ftol(d as i32 as f64 * read_f64(rdata::F64_0_75)) as i16;
        }
        let nation = club.nation().addr();
        if d < 0 && nation != 0 {
            let id = read_u32(nation);
            if id == read_u32(KEY_NATION_WALES) || id == read_u32(KEY_NATION_REPUBLIC_OF_IRELAND) || id == read_u32(KEY_NATION_NORTHERN_IRELAND) {
                d = ftol(d as i32 as f64 * read_f64(rdata::F64_0_75)) as i16;
            }
        }
    }
    if (club.continental_comp_id() as i32) != -1 && d < 0 {
        let a = ftol(d as i32 as f64 * read_f64(board_week_rdata::F64_NEG_0_25)) as i16;
        s.add(board_record::CONFIDENCE, a.wrapping_neg());
        s.add(board_record::SUPPORTERS_CONFIDENCE, a.wrapping_neg());
    } else {
        s.add(board_record::CONFIDENCE, d);
        s.add(board_record::SUPPORTERS_CONFIDENCE, ftol(d as i32 as f64 * read_f64(rdata::F64_1_05)) as i16);
    }
}

/// The reputation band of a reputation: v / 1000 + 16, at least 22 (signed byte).
pub(crate) fn board_week_band(v: i16) -> i8 {
    let b = ((v as i32 / 1000) as i8).wrapping_add(16);
    if b < 22 { 22 } else { b }
}

/// Keeps a value pulled outside [lo, hi] at the nearer bound, the bounds limited to 1..10000 (as the original's tests:
/// only a value outside is touched).
pub(crate) fn board_week_pull(v: i32, lo: i32, hi: i32) -> i32 {
    if v > hi {
        if hi > 10000 { 10000 } else { hi }
    } else if v < lo {
        if lo < 1 { 1 } else { lo }
    } else {
        v
    }
}

/// In the comparer's replay, the copy's story: the next taped call must have the same arguments and the same state
/// before it; its recorded changes are applied. Returns false outside the replay.
pub(crate) unsafe fn board_week_replay_story(club: usize, kind: u32, person: usize, extra: i32) -> bool {
    let tape = match (*core::ptr::addr_of_mut!(BOARD_STORY_TAPE)).as_mut() {
        Some(t) if !t.recording => t,
        _ => return false,
    };
    if tape.broken.is_some() {
        return true;
    }
    let args = [club as u32, kind, person as u32, extra as u32];
    let Some(call) = tape.calls.get(tape.next) else {
        tape.broken = Some(format!("story {} {:x?} the original did not post", tape.next, args));
        return true;
    };
    if call.args != args {
        tape.broken = Some(format!("story {}: copy {:x?} original {:x?}", tape.next, args, call.args));
        return true;
    }
    let now = board_story_flat(&tape.regions);
    if let Some((i, o, x, y)) = board_story_first_difference(&tape.regions, &now, &call.before) {
        tape.broken = Some(format!("state before story {} {:x?}: region {} (+{:#x}) copy {:02x} original {:02x}", tape.next, args, i, o, x, y));
        return true;
    }
    for &(a, b) in &call.changes {
        write_u8(a, b);
    }
    tape.next += 1;
    true
}

/// The league stories from 0x63 on (see `league_board_stories`).
pub(crate) unsafe fn board_week_story_0x63(s: &mut BoardWeek) {
    let club = Club(s.club);
    let e = s.entry;
    let played = read_u8(e + comp_stats_club::MATCHES_PLAYED) as i32;
    let current = read_u8(e + comp_stats_club::CURRENT_POSITION) as i32;
    let last = read_u8(e + comp_stats_club::FINAL_POSITION) as i32;
    let (best, normal, worst) = (s.proj_best as i32, s.proj_normal as i32, s.proj_worst as i32);
    let division = || club.division().addr();
    let object = || board_week_comp_object(division());
    let in_league = || {
        let nation = club.nation();
        division() != 0 && !nation.is_null() && nation.league_selected() != 0 && club.manager().addr() != 0
    };
    let top = || read_i32(object() + league::UPPER_DIVISION_ID) == -1;
    let run = |field: usize| read_i32(e + field);
    let n = || s.league_size as i32;
    let relegated = || read_i8(object() + league::RELEGATED_COUNT) as i32;
    let promotion_places = || read_i8(object() + league::PROMOTION_PLAYOFF_COUNT) as i32 + read_i8(object() + league::PROMOTED_COUNT) as i32;
    let story = |kind: u32| {
        let person = board_week_story_person(club.addr(), 2);
        board_week_post_story(club.addr(), kind, person, -1);
    };
    let good_runs = |a: i32, b: i32| run(comp_stats_club::WINNING_RUN) > random(a) + 2 || run(comp_stats_club::UNBEATEN_RUN) > random(b) + 4;
    let bad_runs = |a: i32, b: i32| run(comp_stats_club::LOSING_RUN) > random(a) + 2 || run(comp_stats_club::WINLESS_RUN) > random(b) + 4;
    // 0x63 and 0x61: chasing the title from the second or third place
    let title_chase: [(u32, fn(i32) -> bool); 2] = [(0x63, |l| l > 3 && l < 6), (0x61, |l| l > 5)];
    for (kind, last_ok) in title_chase {
        if in_league() && top() && played >= random(10) + 10 && played < random(5) + 25 && current < 4 && current > 1
            && last_ok(last) && best == 1 && worst != 1 && good_runs(5, 5)
        {
            covered!("gra_17 story 0x63 0x61");
            return story(kind);
        }
    }
    // 0x62: top of the table
    if in_league() && top() && played >= random(10) + 10 && played < 30 && current == 1 && last == 1 && best == 1 && worst != 1
        && good_runs(3, 4)
    {
        covered!("gra_17 story 0x62");
        return story(0x62);
    }
    // 0x40: slipping from the promotion places
    if in_league() && !top() && played >= random(10) + 25 && current < 9 && current > 1 && last < 7
        && best <= promotion_places() && worst > promotion_places() && bad_runs(3, 4)
    {
        covered!("gra_17 story 0x40");
        return story(0x40);
    }
    // 0x41: slipping from the title race
    if in_league() && top() && played >= random(10) + 25 && current < 6 && last < 5 && (normal == 2 || normal == 3) && best == 1
        && worst != 1 && bad_runs(3, 4)
    {
        covered!("gra_17 story 0x41");
        return story(0x41);
    }
    if division() == 0 {
        return board_week_story_0x3c(s);
    }
    // 0x5e: the surprise package
    if in_league() && (club.reputation() < (random(2000) + 7250) as i16) && played > random(10) + 5
        && played < 25 - random(5) && current < last && current < 5 && best == 1 && worst != 1
        && run(comp_stats_club::WINNING_RUN) != 0 && (run(comp_stats_club::WINNING_RUN) > 4 || run(comp_stats_club::UNBEATEN_RUN) > 7)
    {
        covered!("gra_17 story 0x5e");
        return board_week_post_story(club.addr(), 0x5e, 0, -1);
    }
    // 0x3e: the relegation zone
    if in_league() && current > last && played > random(10) + 5 && played < 25 - random(5)
        && run(comp_stats_club::LOSING_RUN) != 0 && (run(comp_stats_club::LOSING_RUN) > 4 || run(comp_stats_club::WINLESS_RUN) > 7)
        && call_comp_virtual(object(), league_vtable::HAS_RELEGATION) != 0
    {
        let safe = n() - relegated();
        if current >= safe && best < safe {
            covered!("gra_17 story 0x3e");
            return board_week_post_story(club.addr(), 0x3e, 0, -1);
        }
    }
    // 0x3f: the relegation fight (the original's last test can never hold)
    if in_league() && played > random(15) + 15
        && (run(comp_stats_club::LOSING_RUN) > 1 || run(comp_stats_club::WINLESS_RUN) > random(3) + 2)
        && call_comp_virtual(object(), league_vtable::HAS_RELEGATION) != 0
    {
        let safe = n() - relegated();
        if random(2) + current > safe && normal >= safe && best < safe && best > n() / 2 - relegated() + n() {
            covered!("gra_17 story 0x3f");
            return story(0x3f);
        }
    }
    board_week_story_0x3c(s)
}

/// A draw(the chairman's patience) other than 0, or a draw(`n`) other than 0 without the chairman's sheet.
pub(crate) unsafe fn board_week_patience_draw(club: usize, n: i32) -> bool {
    let sheet = board_week_chairman_sheet(club);
    if sheet != 0 { random(read_i8(sheet + non_player::PATIENCE) as i32) != 0 } else { random(n) != 0 }
}

/// Worse than the final position: c = max((2 − games) × 2, truncate(final × w − current × w)) (16 bits). With the
/// title or promotion expected (`expectation_flags::TITLE_CHALLENGE`, `expectation_flags::PROMOTION`) and c below 0,
/// the current position within the division's play-off places (`league::PROMOTION_PLAYOFF_COUNT`, non-zero) counts as
/// c = 0; else with the play-offs expected (`expectation_flags::PLAYOFFS`) the same, the board raised to 6500. Then by
/// c: above −3, −5 (above 9500 the board pulled to 9500 instead when games ≥ patience / 2 + 2c + 10; below 4275 raised
/// to 4275 on the patience draw); −3 and −4, 10c − 25 (above 6500 to 6500 when games ≥ patience / 2 + c + 12; below
/// 3500 to 3500 on the patience draw); −5 to −9, 25c + 60 (above 6500 to 6500 when games ≥ patience / 2 + 10; above
/// 4275 to 4275 when games ≥ patience / 2 + 15; below 3000 to 3000 on the patience draw); −10 and below, 30c + 105 (the
/// same, but below 3500 to 3500 when draw(patience) > draw(30) + 1, or on a draw(5) of 0 without the chairman's sheet).
/// Each pull replaces the change (0).
pub(crate) unsafe fn board_week_overshoot(s: &mut BoardWeek, current: u8, last: u8) -> i32 {
    let club = Club(s.club);
    let played = read_u8(s.entry + comp_stats_club::MATCHES_PLAYED) as i32;
    let v = ftol(last as i32 as f64 * s.weight - current as i32 as f64 * s.weight) as i16 as i32;
    let mut c = (2 - played) * 2;
    if !(c > v) {
        c = v;
    }
    let flags = read_u16(s.row() + board_record::EXPECTATION_FLAGS) as u8;
    let division = club.division().addr();
    let in_playoffs = || {
        let o = League(board_week_comp_object(club.division().addr()));
        !o.is_null() && (o.promotion_playoff_count() as i8) != 0 && current as i32 <= (o.promotion_playoff_count() as i8) as i32
    };
    let mut pulled = false;
    if flags & (expectation_flags::TITLE_CHALLENGE | expectation_flags::PROMOTION) as u8 != 0 && (c as i16) < 0 && division != 0 {
        if in_playoffs() {
            c = 0;
            pulled = true;
        }
    } else if flags & expectation_flags::PLAYOFFS as u8 != 0 && (c as i16) < 0 && division != 0 && in_playoffs() {
        if s.confidence() < 6500 {
            s.set_confidence(6500);
        }
        c = 0;
        pulled = true;
    }
    let c16 = c as i16;
    let patience = || read_i8(board_week_chairman_sheet(club.addr()) + non_player::PATIENCE) as i32 / 2;
    let has_chairman = || board_week_chairman_sheet(club.addr()) != 0;
    if pulled || c16 > -3 {
        let conf = s.confidence();
        if conf > 9500 {
            if has_chairman() && played >= patience() + c16 as i32 * 2 + 10 {
                s.set_confidence(9500);
                return 0;
            }
        } else if conf < 4275 && board_week_patience_draw(club.addr(), 10) {
            s.set_confidence(4275);
            return 0;
        }
        return -5;
    }
    if c16 > -5 {
        let conf = s.confidence();
        if conf > 6500 {
            if has_chairman() && played >= patience() + c16 as i32 + 12 {
                s.set_confidence(6500);
                return 0;
            }
        } else if conf < 3500 && board_week_patience_draw(club.addr(), 10) {
            s.set_confidence(3500);
            return 0;
        }
        return c * 10 - 25;
    }
    let (delta, low) = if c16 > -10 { (c * 25 + 60, 3000) } else { (c * 30 + 105, 3500) };
    let conf = s.confidence();
    if conf > 6500 && has_chairman() && played >= patience() + 10 {
        s.set_confidence(6500);
        return 0;
    }
    if conf > 4275 {
        if has_chairman() && played >= patience() + 15 {
            s.set_confidence(4275);
            return 0;
        }
    } else if conf < low {
        let pull = if c16 > -10 {
            board_week_patience_draw(club.addr(), 10)
        } else {
            let sheet = board_week_chairman_sheet(club.addr());
            if sheet != 0 {
                let a = random(read_i8(sheet + non_player::PATIENCE) as i32);
                a > random(30) + 1
            } else {
                random(5) == 0
            }
        };
        if pull {
            s.set_confidence(low as i16);
            return 0;
        }
    }
    delta
}

/// The last league story, 0x3c (see `league_board_stories`).
pub(crate) unsafe fn board_week_story_0x3c(s: &mut BoardWeek) {
    let club = Club(s.club);
    let e = s.entry;
    let last = read_u8(e + comp_stats_club::FINAL_POSITION);
    let current = read_u8(e + comp_stats_club::CURRENT_POSITION);
    if read_u8(e + comp_stats_club::MATCHES_PLAYED) > 20 && (last as i16 - current as i16) < -5 && s.proj_best > last
        && (exe::club_dislikes_staff(club.manager().addr(), club) != 0 || random(4) == 0)
        && (club.is_null() || read_i16(read_usize(s.this).wrapping_add((read_i32(club.addr()) as usize).wrapping_mul(0x49)) + board_record::SUPPORTERS_CONFIDENCE) < 4275)
        && (club.continental_comp_id() as i32) == -1
        && (read_i32(e + comp_stats_club::LOSING_RUN) > random(4) + 2 || read_i32(e + comp_stats_club::WINLESS_RUN) > random(7) + 3)
    {
        covered!("gra_17 story 0x3c");
        board_week_post_story(club.addr(), 0x3c, 0, -1);
        s.add(board_record::FAVOURITES_MOOD, -250);
    }
}

/// The league size N the board's pre-season stories read before it is written (see `weekly_board_confidence_update`):
/// the last one of the previous call, the copy's stand-in for the original's stack garbage.
pub(crate) static BOARD_WEEK_LEAGUE_SIZE: AtomicU8 = AtomicU8::new(0);

/// One club's pass of `weekly_board_confidence_update` (the original's locals).
pub(crate) struct BoardWeek {
    pub(crate) this: usize,
    pub(crate) index: usize,
    pub(crate) club: usize,
    pub(crate) manager: usize,
    pub(crate) sheet: usize,
    pub(crate) confidence_before: i16,
    pub(crate) league_size: i8,
    pub(crate) told_finals_hopes: bool,
    pub(crate) told_finals_doubts: bool,
    pub(crate) entry: usize,
    pub(crate) division_object: usize,
    pub(crate) proj_best: u8,
    pub(crate) proj_normal: u8,
    pub(crate) proj_worst: u8,
    pub(crate) x: f32,
    pub(crate) weight: f64,
}

impl BoardWeek {
    /// The club's board row: [this] + loop index × 0x49 (re-read every time, as the original).
    pub(crate) unsafe fn row(&self) -> usize {
        read_usize(self.this).wrapping_add(self.index.wrapping_mul(0x49))
    }
    pub(crate) unsafe fn add(&self, field: usize, v: i16) {
        let a = self.row() + field;
        write_i16(a, read_i16(a).wrapping_add(v));
    }
    pub(crate) unsafe fn confidence(&self) -> i16 {
        read_i16(self.row() + board_record::CONFIDENCE)
    }
    pub(crate) unsafe fn set_confidence(&self, v: i16) {
        write_i16(self.row() + board_record::CONFIDENCE, v);
    }
}

pub(crate) static mut BOARD_STORY_TAPE: Option<BoardStoryTape> = None;

/// The nine non-player attributes the score compares between the person and the viewer (the club's chairman), in the
/// order the original goes through them: attacking, coaching technique, directness, discipline, free roles, marking,
/// offside, pressing, youngsters.
pub(crate) const STAFF_JOB_SIMILARITY: [usize; 9] = [non_player::ATTACKING, non_player::COACHING_TECHNIQUE, non_player::DIRECTNESS, non_player::DISCIPLINE,
    non_player::FREE_ROLES, non_player::MARKING, non_player::OFFSIDE, non_player::PRESSING, non_player::YOUNGSTERS];

/// The bits of the contract's grievance flags (`+0x45`) the new manager clears for each player of the squad, in the
/// original's order.
pub(crate) const NEW_MANAGER_CLEARED_GRIEVANCES: [u32; 7] = [8, 0x200, 0x400, 0x2000, 0x10000, 0x80000, 0x800000];

/// The expectation sentence of the board, by the flags: (title flag 1, the six format pairs of the other flags).
/// Each entry is the text without 0x800 and with 0x800; the three columns are flag 0x400, neither 0x400 nor 0x100,
/// and 0x100.
pub(crate) const EXPECTATION_TEXTS: [(u32, [[usize; 2]; 3]); 5] = [
    (4, [[0xa041e8, 0xa04268], [0xa03f98, 0xa03ffc], [0xa040a0, 0xa04124]]),
    (8, [[0xa03e58, 0xa03edc], [0xa03c18, 0xa03c7c], [0xa03d18, 0xa03d9c]]),
    (0x40, [[0xa03ab0, 0xa03b44], [0xa03828, 0xa038a0], [0xa03958, 0xa039e8]]),
    (0x10, [[0xa036d8, 0xa03760], [0xa03480, 0xa034ec], [0xa03598, 0xa0361c]]),
    (0x20, [[0xa0331c, 0xa033ac], [0xa030a4, 0xa03118], [0xa031cc, 0xa03258]]),
];

/// The tape of the stories: the compared regions, the calls, the next one to replay and the first mismatch.
pub(crate) struct BoardStoryTape {
    pub(crate) regions: Vec<(usize, usize)>,
    pub(crate) calls: Vec<BoardStoryCall>,
    pub(crate) next: usize,
    pub(crate) broken: Option<String>,
    pub(crate) recording: bool,
    pub(crate) depth: u32,
    pub(crate) target: usize,
}

/// One story the original posted during the weekly update: the arguments, the compared state just before it and the
/// bytes it changed (address, new value).
pub(crate) struct BoardStoryCall {
    pub(crate) args: [u32; 4],
    pub(crate) before: Vec<u8>,
    pub(crate) changes: Vec<(usize, u8)>,
}

/// Writes the board's (or the FA's) verdict on a match into the game's text buffer (`NEWS_TEXT_BUFFER`), by the
/// match marks, with a sentence on the league run or on the players it named; returns 1, or 0 when only the plain
/// default verdict was written.
///
/// `build_board_match_reaction_text(club, other, comp, marks, score, _, _, ids, variant, loss, run, target, current,
/// best, in_league)`, called after every match by `update_board_confidence_after_match` (the text is not used there:
/// only the return value) and by the news text `news_board_match_reaction_text`. The board: "The <club> board of
/// directors" (0xa0fc54, the club's `.gender_name()` and name), or for a national team (`exe::database_is_national_team`)
/// "The <nationality> Football Association" (0xa0fc8c, the adjective of the club's `.nation()` by
/// `exe::nation_get_nationality_adjective`, with 100 as the two genders). The verdict by `marks`
/// (`board_verdict_template`): the cases that ask whether the competition is a friendly
/// (`exe::friendly_is_friendly_competition` of `G_FRIENDLY`) take the friendly template; the others, their template
/// with the opponent's gender and name, the competition's `.short_gender_name()` and short name and the score.
/// Marks without a case write straight into the buffer with the score and return 0: 0xa0e94c with 0x800, 0xa0be98
/// with 0x400, else 0xa0bdd0.
///
/// The run sentence (`board_run_sentence`), only with `in_league`: a winning run (2) with the club at or above its
/// target by at most 3 places gives "We are very impressed..."; a losing run (−2) with
/// the target not above the best position: more than 4 places below the target "If this losing run isn't turned
/// around...", more than 4 above it in a division (`league::ENTRANT_COUNT` low byte, `league::RELEGATED_COUNT`,
/// `league_vtable::HAS_RELEGATION`) "danger of getting relegated" when the board expects a relegation battle
/// (`expectation_flags::RELEGATION_BATTLE` of `board_record::EXPECTATION_FLAGS`) and the club is in the relegation
/// places, else "danger of losing our good league position"; more than 2 places below the target, the "worried"
/// sentence of the first expectation flag (European places, title, promotion, play-offs, 0x40, relegation battle,
/// else the comfortable position). Without one and with marks 0x1800, the named players' sentence
/// (`build_board_player_performance_text`, with "won" = marks bit 0x800 in the low byte of `loss`). The buffer gets
/// "%s %s" (verdict and sentence) or "%s" (0x991948 / 0x991944, `story_crt!`).
///
/// Defects of the original: without a competition it writes 0xa0bd50 with the board string before building it
/// (uninitialised stack; the callers always pass one); the winning-run tests make "Hopefully this run..." and the
/// seven "This run of victories is keeping us on track..." sentences unreachable (they need the club both at or above
/// and below its target); the two aggregate arguments are never read.
/// Original: `MANAGER_MANAGER::build_board_match_reaction_text` (0x6a69c0, manager_manager.cpp, thiscall, ret 0x3c).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn build_board_match_reaction_text(this: usize, club: usize, other: usize, comp: usize, marks: i32, score: usize, _aggregate_for: i32,
                                                                       _aggregate_against: i32, ids: usize, variant: u32, loss: u32, run: u32, target: u32,
                                                                       current: u32, best: u32, in_league: u32) -> u32 {
    let (club, other, comp) = (Club(club), Club(other), Competition(comp));
    let mut board = [0u8; 200];
    let b = board.as_mut_ptr() as usize;
    if comp.is_null() {
        // simplification: the original quotes the board string before writing it (stack garbage); the copy an empty one
        story_lang!(NEWS_TEXT_BUFFER, 0xa0bd50, other.gender_name() as i32, other.gender_name() as i32, b, other.name_addr());
        return 0;
    }
    if exe::database_is_national_team(club) != 0 {
        let mut adjective = [0u8; 100];
        exe::nation_get_nationality_adjective(club.nation(), adjective.as_mut_ptr() as usize, 100, 9);
        story_lang!(b, 0xa0fc8c, 100, 100, adjective.as_ptr()); // "The <nationality> Football Association"
    } else {
        let g = club.gender_name() as i32;
        story_lang!(b, 0xa0fc54, g, g, club.name_addr()); // "The <club> board of directors"
    }
    let mut verdict = [0u8; 1500];
    let v = verdict.as_mut_ptr() as usize;
    match board_verdict_template(marks) {
        Some((friendly, template, shape)) => {
            covered!("sx14_1 verdict");
            match friendly {
                Some(t) if exe::friendly_is_friendly_competition(read_usize(G_FRIENDLY), comp) != 0 => {
                    covered!("sx14_1 friendly verdict");
                    format_board_verdict(v, t, BoardVerdictArgs::Friendly, other, comp, b, score)
                }
                _ => format_board_verdict(v, template, shape, other, comp, b, score),
            }
        }
        None => {
            covered!("sx14_1 default verdict");
            let template = if marks & 0x800 != 0 { 0xa0e94c } else if marks & 0x400 != 0 { 0xa0be98 } else { 0xa0bdd0 };
            format_board_verdict(NEWS_TEXT_BUFFER, template, BoardVerdictArgs::OpponentInCompWithScore, other, comp, b, score);
            return 0;
        }
    }
    let mut sentence = [0u8; 1500];
    let s = sentence.as_mut_ptr() as usize;
    if in_league as u8 != 0 {
        let t = board_run_sentence(this, club, run as i8, target as i8, current as i8, best as i8);
        if t != 0 {
            covered!("sx14_1 run sentence");
            story_lang!(s, t);
        }
    }
    if sentence[0] == 0 && marks & 0x1800 != 0 {
        covered!("sx14_1 named players");
        let won = (loss & 0xffff_ff00) | ((marks as u32 >> 11) & 1);
        build_board_player_performance_text(ids as *const i32, won, s, variant);
    }
    if sentence[0] != 0 {
        story_crt!(NEWS_TEXT_BUFFER, 0x991948, v, s); // "%s %s"
    } else {
        story_crt!(NEWS_TEXT_BUFFER, 0x991944, v); // "%s"
    }
    1
}

/// Comparer for `build_board_match_reaction_text`: the original and the copy side by side on the game's text buffer
/// (`NEWS_TEXT_BUFFER`, 2000 bytes) and the return value. Without a competition only the original runs (it quotes an
/// uninitialised string).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_build_board_match_reaction_text(this: usize, club: usize, other: usize, comp: usize, marks: i32, score: usize, aggregate_for: i32,
                                                                               aggregate_against: i32, ids: usize, variant: u32, loss: u32, run: u32, target: u32,
                                                                               current: u32, best: u32, in_league: u32) -> u32 {
    type F = unsafe extern "thiscall" fn(usize, usize, usize, usize, i32, usize, i32, i32, usize, u32, u32, u32, u32, u32, u32, u32) -> u32;
    let o = original_fn::<F>(IDX_BUILD_BOARD_MATCH_REACTION_TEXT);
    let original = || o(this, club, other, comp, marks, score, aggregate_for, aggregate_against, ids, variant, loss, run, target, current, best, in_league);
    if comp == 0 {
        return original();
    }
    let (ro, rr, same, mo, mr) = crate::comparison::side_by_side(&[(NEWS_TEXT_BUFFER, 2000)], original, || {
        build_board_match_reaction_text(this, club, other, comp, marks, score, aggregate_for, aggregate_against, ids, variant, loss, run, target, current, best, in_league)
    });
    let text = |m: &crate::comparison::MemorySnapshot| {
        let b = &m.0[0].1;
        String::from_utf8_lossy(&b[..b.iter().position(|&c| c == 0).unwrap_or(b.len())]).into_owned()
    };
    SCOREBOARD.record(same && ro == rr, || {
        format!("build_board_match_reaction_text(marks {:#x}, run {} target {} current {} best {} league {}) original {} {:?} rust {} {:?}",
                marks, run as i8, target as i8, current as i8, best as i8, in_league as u8, ro, text(&mo), rr, text(&mr))
    });
    ro
}

/// Writes the headline or the body of the board's reaction to a match (news 0x138c, posted by
/// `post_board_match_reaction_news`).
///
/// `news_board_match_reaction_text(news, body, out)`. The headline (`board_reaction_headline`) from slots 0 (the
/// marks), 1 (the club) and 2 (the opponent), with the links to the two clubs' short names (0x7d5). The body reads
/// the 18 slots in order (the marks, the clubs, the score, the aggregate, the fixture's stats slot, the three named
/// players, the variant, the defeat, the run, the target, the current and best positions and the league flag) and
/// the match record behind them (`exe::read_match_record_from_news`, a `RecordMatch`); the score "<higher>-<lower>"
/// (0xa0bd30) and the verdict (`FN_MANAGER_MANAGER_BUILD_BOARD_MATCH_REACTION_TEXT`, with the record's competition)
/// into the game's text buffer, which goes into `out` (`exe::text_set_string`); the links: the two clubs' names
/// (0x7d5), the competition's short name (0x7d0), each named player still set (0x7db, the name by
/// `exe::database_format_staff_name`, mode 1) and the score (0x7d3) when the fixture is still in the match stats
/// (`RecordMatch::link_score`).
/// Original: `MANAGER_MANAGER::news_board_match_reaction_text` (0x6a6310, manager_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn news_board_match_reaction_text(this: usize, news: usize, body: u32, out: usize) {
    if body == 0 {
        covered!("sx14_2 headline");
        let (marks, club, other) = (exe::newsitem_get_slot(news, 0), exe::newsitem_get_slot(news, 1), exe::newsitem_get_slot(news, 2));
        let (club, other) = (club_at(club as i32 as isize), club_at(other as i32 as isize));
        board_reaction_headline(marks, club, other);
        exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
        exe::text_add_link(out, (club.addr() + club::SHORT_NAME) as *const u8, 0x7d5, club);
        exe::text_add_link(out, (other.addr() + club::SHORT_NAME) as *const u8, 0x7d5, other);
        return;
    }
    covered!("sx14_2 body");
    let mut k: u8 = 0;
    let mut slots = [0u32; 18];
    for v in slots.iter_mut() {
        *v = exe::newsitem_get_slot(news, k as i8 as u32);
        k += 1;
    }
    let mut record = [0u8; 0x20];
    let rec = record.as_mut_ptr() as usize;
    exe::date_date_54a110(rec);
    exe::read_match_record_from_news(news, &mut k, record.as_mut_ptr());
    let record = RecordMatch(rec);
    let [marks, club, other, goals_a, goals_b, aggregate_for, aggregate_against, _stats_slot, id1, id2, id3, variant, loss, run, target, current, best, in_league] = slots;
    let (club, other) = (club_at(club as i32 as isize), club_at(other as i32 as isize));
    let (goals_a, goals_b) = (goals_a as i32, goals_b as i32);
    let mut score = [0u8; 100];
    story_lang!(score.as_mut_ptr(), 0xa0bd30, goals_a.max(goals_b), goals_a.min(goals_b)); // "<goals>-<opp goals>"
    let ids = [id1 as i32, id2 as i32, id3 as i32];
    let verdict: unsafe extern "thiscall" fn(usize, usize, usize, usize, i32, usize, i32, i32, usize, u32, u32, u32, u32, u32, u32, u32) -> u32 =
        core::mem::transmute(FN_MANAGER_MANAGER_BUILD_BOARD_MATCH_REACTION_TEXT);
    verdict(this, club.addr(), other.addr(), record.comp(), marks as i32, score.as_ptr() as usize, aggregate_for as i32, aggregate_against as i32,
            ids.as_ptr() as usize, variant, loss, run, target, current, best, in_league);
    exe::text_set_string(out, NEWS_TEXT_BUFFER as *const u8);
    exe::text_add_link(out, club.name_addr() as *const u8, 0x7d5, club);
    exe::text_add_link(out, other.name_addr() as *const u8, 0x7d5, other);
    let comp = record.comp();
    exe::text_add_link(out, (comp + competition::SHORT_NAME) as *const u8, 0x7d0, comp);
    let mut name = [0u8; 200];
    for id in ids {
        if id != -1 {
            let p = staff_at(id as isize);
            exe::database_format_staff_name(p, name.as_mut_ptr() as usize, 200, 1);
            exe::text_add_link(out, name.as_ptr(), 0x7db, p);
        }
    }
    record.link_score(out, &score);
}

/// Comparer for `news_board_match_reaction_text`: the text side by side (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_news_board_match_reaction_text(this: usize, news: usize, body: u32, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(IDX_NEWS_BOARD_MATCH_REACTION_TEXT);
    compare_text_writer(out, || o(this, news, body, out), |t| news_board_match_reaction_text(this, news, body, t),
                        || format!("news_board_match_reaction_text(news {:#x}, body {})", news, body));
}

/// The board's daily pass over the clubs and the manager market.
///
/// `MANAGER_MANAGER::daily_update()`, every day (the game's day loop). When the month changes (`exe::date_get_month`
/// against the last month seen at 0xb67a40; the first day after loading only records it, flag 0x9f18a0), each club that
/// is not a national team, or a national team whose nation has +0x71 set, gets the board's confidence news
/// (`post_board_confidence_update_news`), then `fix_missing_jobs`. The human managers settle in after 180 days
/// (`settle_new_human_managers`); on Sundays (day of the year divisible by 7) `weekly_board_confidence_update`; the
/// pending departures (`process_pending_departures`). Then every vacancy of `manager_manager_fields::VACANCY_LIST`
/// from the head: past its arrival date, the appointment (`appoint_from_vacancy`); before it, the rumour
/// (`vacancy_rumour`). Finally the ban appeals (`process_ban_appeals`).
///
/// Defects of the original: a person moving to a club from his national team is released by his national job, but
/// the default case writes his club job (0xb, 0 or 8); the "manager record" check after the appointment can only show
/// its error box (line 0x5e2) if a call in between cleared the record.
/// Original: `MANAGER_MANAGER::daily_update` (0x68f9c0, manager_manager.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn daily_update(this: usize) {
    const FIRST_DAY_AFTER_LOAD: usize = 0x9f18a0;
    const LAST_MONTH: usize = 0xb67a40;
    let month = exe::date_get_month(G_CURRENT_DATE);
    if read_u8(FIRST_DAY_AFTER_LOAD) != 0 {
        write_u8(LAST_MONTH, month);
        write_u8(FIRST_DAY_AFTER_LOAD, 0);
    } else if month != read_u8(LAST_MONTH) {
        covered!("sx14_3 new month");
        write_u8(LAST_MONTH, exe::date_get_month(G_CURRENT_DATE));
        let mut k = 0;
        while k < read_i32(G_CLUB_COUNT) {
            let club = club_at(k as isize);
            let nation = club.nation();
            if exe::database_is_national_team(club) == 0 || (!nation.is_null() && read_u32(nation.addr() + 0x71) != 0) {
                post_board_confidence_update_news(this, club.addr());
            }
            k += 1;
        }
        fix_missing_jobs(this);
    }
    settle_new_human_managers(this);
    if read_i16(G_CURRENT_DATE) % 7 == 0 {
        weekly_board_confidence_update(this);
    }
    process_pending_departures(this);
    let list = read_usize(this + manager_manager_fields::VACANCY_LIST);
    list_rewind(list);
    loop {
        let e = exe::llist_get_next(list);
        if e == 0 {
            break;
        }
        let item = VacancyItem(e);
        if item.date_passed(0x16) {
            appoint_from_vacancy(this, list, item, item.person(), item.club());
        } else {
            vacancy_rumour(this, item);
        }
    }
    process_ban_appeals(this);
}

/// Comparer for `daily_update`: it moves managers, posts news and draws, so only the original runs (its effects are
/// checked by the career).
pub(crate) unsafe extern "thiscall" fn compare_daily_update(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_DAILY_UPDATE)(this)
}

/// The verdict template of a match by its marks: the friendly template (form `Friendly`) when the case asks whether
/// the competition is a friendly, the other template and its argument shape; `None` for the marks without a case of
/// their own (the 0x800 / 0x400 / plain defaults).
pub(crate) fn board_verdict_template(marks: i32) -> Option<(Option<usize>, usize, BoardVerdictArgs)> {
    use BoardVerdictArgs::*;
    Some(match marks {
        0x401 | 0x501 => (Some(0xa0f5f4), 0xa0f548, OpponentInComp),
        0x402 | 0x502 => (Some(0xa0f8ac), 0xa0f7f8, OpponentInComp),
        0x408 | 0x4a0 | 0x608 | 0x6a0 => (Some(0xa0eab4), 0xa0ea10, OpponentInComp),
        0x420 | 0x620 => (Some(0xa0f204), 0xa0f164, OpponentInComp),
        0x441 | 0x541 => (Some(0xa0f750), 0xa0f694, OpponentInComp),
        0x442 | 0x542 => (Some(0xa0fa34), 0xa0f958, OpponentInCompWithScore),
        0x444 | 0x544 => (Some(0xa0fbac), 0xa0fad8, OpponentInCompWithScore),
        0x448 | 0x648 => (Some(0xa0f350), 0xa0f2a0, OpponentInComp),
        0x450 | 0x650 => (Some(0xa0ee98), 0xa0ede4, OpponentInComp),
        0x460 | 0x660 => (Some(0xa0ec04), 0xa0eb54, OpponentInComp),
        0x481 | 0x581 => (Some(0xa0f49c), 0xa0f3f0, OpponentInComp),
        0x482 | 0x582 => (Some(0xa0f5f4), 0xa0ef3c, OpponentInComp), // the friendly text of 0x401
        0x484 | 0x584 => (Some(0xa0f0bc), 0xa0efec, OpponentInCompWithScore),
        0x488 | 0x688 => (Some(0xa0ed4c), 0xa0ecac, OpponentInComp),
        0x808 | 0xa08 => (None, 0xa0e6cc, CompThenOpponentWithScore),
        0x810 | 0xa10 => (None, 0xa0e168, CompThenOpponentWithScore),
        0x820 | 0xa20 => (None, 0xa0e350, CompThenOpponentWithScore),
        0x848 | 0xa48 => (Some(0xa0e62c), 0xa0e55c, OpponentInCompWithScore),
        0x850 | 0xa50 => (Some(0xa0e2c8), 0xa0e210, OpponentInCompWithScore),
        0x860 | 0xa60 => (Some(0xa0e4c8), 0xa0e404, OpponentInCompWithScore),
        0x888 | 0xa88 => (Some(0xa0e0d0), 0xa0e008, OpponentInCompWithScore),
        0x890 | 0xa90 => (Some(0xa0de38), 0xa0dd88, OpponentInCompWithScore),
        0x8a0 | 0xaa0 => (Some(0xa0df7c), 0xa0dec0, OpponentInCompWithScore),
        0x948 => (None, 0xa0e86c, OpponentInCompWithScore),
        0x988 => (None, 0xa0e790, OpponentInCompWithScore),
        0x1001 | 0x1101 => (Some(0xa0d030), 0xa0cf74, OpponentInCompWithScore),
        0x1002 | 0x1102 => (None, 0xa0d3cc, CompThenOpponentWithScore),
        0x1008 => (Some(0xa0dcd4), 0xa0dc08, OpponentInCompWithScore),
        0x1020 | 0x1120 => (Some(0xa0d85c), 0xa0d794, OpponentInCompWithScore),
        0x1041 | 0x1141 => (Some(0xa0d314), 0xa0d244, OpponentInCompWithScore),
        0x1042 | 0x1142 => (Some(0xa0d560), 0xa0d490, OpponentInCompWithScore),
        0x1044 | 0x1144 => (Some(0xa0d6f0), 0xa0d624, OpponentInCompWithScore),
        0x1048 | 0x1148 => (Some(0xa0db68), 0xa0da90, OpponentInCompWithScore),
        0x1060 | 0x1160 => (Some(0xa0d9dc), 0xa0d908, OpponentInCompWithScore),
        0x1081 | 0x1181 => (Some(0xa0d194), 0xa0d0d0, OpponentInCompWithScore),
        0x1082 | 0x1182 => (Some(0xa0ceb4), 0xa0cdec, OpponentInCompWithScore),
        0x1084 | 0x1184 => (Some(0xa0c9b8), 0xa0c8f4, OpponentInCompWithScore),
        0x1088 | 0x1188 => (Some(0xa0cd48), 0xa0cc70, OpponentInCompWithScore),
        0x10a0 | 0x11a0 => (Some(0xa0cb30), 0xa0ca60, OpponentInCompWithScore),
        0x1208 => (None, 0xa0cbd0, OpponentInComp),
        0x1220 => (None, 0xa0c854, OpponentInComp),
        0x1248 => (None, 0xa0c780, OpponentInComp),
        0x1260 => (None, 0xa0c610, OpponentInComp),
        0x1288 => (None, 0xa0c6b8, OpponentInComp),
        0x12a0 => (None, 0xa0c560, OpponentInComp),
        _ => return None,
    })
}

/// Formats one board verdict (`story_lang!`) into `out` with the argument shape of its case.
pub(crate) unsafe fn format_board_verdict(out: usize, template: usize, shape: BoardVerdictArgs, other: Club, comp: Competition, board: usize, score: usize) {
    let go = other.gender_name() as i32;
    let gc = comp.short_gender_name() as i8 as i32;
    let (name, short) = (other.name_addr(), comp.short_name_addr());
    match shape {
        BoardVerdictArgs::Friendly => story_lang!(out, template, go, go, board, score, name),
        BoardVerdictArgs::OpponentInComp => story_lang!(out, template, go, go, gc, gc, board, name, short),
        BoardVerdictArgs::OpponentInCompWithScore => story_lang!(out, template, go, go, gc, gc, board, score, name, short),
        BoardVerdictArgs::CompThenOpponentWithScore => story_lang!(out, template, gc, gc, go, go, board, score, short, name),
    }
}

/// The board's sentence on the league run after the verdict (0 when there is none): `run` 2 (a winning run) or −2 (a
/// losing run), the board's target, the club's current and best positions.
pub(crate) unsafe fn board_run_sentence(this: usize, club: Club, run: i8, target: i8, current: i8, best: i8) -> usize {
    let (target, current) = (target as i32, current as i32);
    let flags = read_u16(board_row(this, club) + board_record::EXPECTATION_FLAGS);
    // the first expectation flag (in this order) picks the sentence of the run of victories / of defeats
    let by_expectation = |sentences: [usize; 7]| {
        let bits = [1u16, 2, 4, 8, 0x40, 0x20];
        bits.iter().position(|&b| flags & b != 0).map_or(sentences[6], |k| sentences[k])
    };
    if run == 2 {
        if target < current {
            return 0;
        }
        if current > target + 4 {
            return 0xa0c514; // "Hopefully this run of results indicates a change in the club's fortunes."
        }
        if current > target - 4 {
            return 0xa0c4c4; // "We are very impressed with the way that the team is performing at the moment."
        }
        if current <= target {
            return 0;
        }
        return by_expectation([0xa0c474, 0xa0c42c, 0xa0c3e8, 0xa0c3a0, 0xa0c354, 0xa0c310, 0xa0c2c4]); // "This run of victories is keeping us on track to ..."
    }
    if run != -2 || target < best as i32 {
        return 0;
    }
    if current > target + 4 {
        return 0xa0c270; // "If this losing run isn't turned around soon we may have to look for a new manager."
    }
    if current < target - 4 && !club.division().is_null() {
        let league = League(read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add((read_i32(club.division().addr()) as usize).wrapping_mul(4))));
        if !league.is_null() {
            let clubs = (league.entrant_count() as i8) as i32; // a byte of the entrant count
            let relegation_zone = flags as i16 & expectation_flags::RELEGATION_BATTLE != 0
                && call_comp_virtual(league.addr(), league_vtable::HAS_RELEGATION) != 0
                && current > clubs - (league.relegated_count() as i8) as i32;
            return if relegation_zone {
                0xa0c22c // "We are in danger of getting relegated unless results improve soon."
            } else {
                0xa0c1d8 // "We are in danger of losing our good league position unless results improve soon."
            };
        }
    }
    if current <= target + 2 {
        return 0;
    }
    by_expectation([0xa0c174, 0xa0c118, 0xa0c0c0, 0xa0c068, 0xa0c008, 0xa0bfc0, 0xa0bf5c]) // "We are worried that this run of defeats ..."
}

/// The headline of the board's match reaction (news 0x138c): the derby headlines (marks 0x40) have no club; the
/// others quote the opponent, except the fury one, which quotes the club itself.
pub(crate) unsafe fn board_reaction_headline(marks: u32, club: Club, other: Club) {
    if marks & 0x40 != 0 {
        let template = if marks & 8 != 0 {
            0xa0bd0c // "Board delighted at derby showing"
        } else if marks & 0x20 != 0 {
            0xa0bcec // "Board happy at derby display"
        } else if marks & 1 != 0 {
            0xa0bccc // "Board furious at derby display"
        } else if marks & 2 != 0 {
            0xa0bca8 // "Board unhappy with derby display"
        } else {
            0xa0bc88 // "Board reaction to derby match"
        };
        story_lang!(NEWS_TEXT_BUFFER, template);
        return;
    }
    let (template, quoted) = if marks & 8 != 0 {
        (0xa0bc48, other) // "Board delighted after <club> game"
    } else if marks & 0x20 != 0 {
        (0xa0bc0c, other) // "<club> match pleases board"
    } else if marks & 1 != 0 {
        (0xa0bbcc, club) // "Board fury at <club> performance"
    } else if marks & 2 != 0 && marks & 0x1000 != 0 {
        (0xa0bb8c, other) // "Board unhappy after <club> defeat"
    } else {
        (0xa0bb50, other) // "Board reaction to <club> game"
    };
    let g = quoted.short_gender_name() as i8 as i32;
    story_lang!(NEWS_TEXT_BUFFER, template, g, g, quoted.addr() + club::SHORT_NAME);
}

/// The 16 human managers who joined their club more than 180 days ago lose the club board's
/// `board_record_flags::NEW_MANAGER` bit (bit 0 of `board_record::FLAGS`).
pub(crate) unsafe fn settle_new_human_managers(this: usize) {
    let mut since = [0u16; 4];
    exe::date_sub_days(G_CURRENT_DATE, since.as_mut_ptr() as usize, 0xb4);
    for k in 0..0x10 {
        let person = staff_at((read_i32(G_STAFF_COUNT) - 0x10 + k) as isize);
        let club = person.club_job();
        if club.is_null() || person.classification() == 0 {
            continue;
        }
        let year = person.date_joined_club_year();
        if since[1] > year || (since[1] == year && (since[0] as i16) > (person.date_joined_club() as i16)) {
            let flags = board_row(this, club) + board_record::FLAGS;
            write_u8(flags, read_u8(flags) & 0xfe);
        }
    }
}

/// An AI person's vacancy whose arrival date came: without a manager record the vacancy reopens with him; on loan or
/// retiring (staff state 0x30) without both records it reopens without him; a club that already has a manager, or
/// a person under offer, just drops the item. Otherwise the appointment: the club he leaves
/// (`previous_club`; if he managed it and it can be managed, its season expectations and a vacancy there), the job
/// application date cleared, the release and the appointment news (`release_appointed_person`), the past transfer of a
/// player between two clubs (`record_manager_move`), the installation (`install_manager`), the item off the list and
/// the board reset (`reset_board_for_new_manager`).
pub(crate) unsafe fn appoint_from_vacancy(this: usize, list: usize, item: VacancyItem, person: Staff, club: Club) {
    if is_human_id(person) {
        human_vacancy_reached(this, list, item, person, club);
        return;
    }
    let contracts = read_usize(G_CONTRACT_MANAGER);
    let reopen = |left: usize| {
        clear_job_application_date(this, club);
        exe::llist_remove(list, item.0);
        add_job_vacancy(this, club.addr(), left);
    };
    if person.non_player().is_null() {
        clear_job_application_date(this, club);
        exe::llist_remove(list, item.0);
        add_job_vacancy(this, club.addr(), person.addr());
        return;
    }
    if exe::contract_manager_get_loan_contract(contracts, person) != 0
        || (staff_state_byte_b(read_i32(person.addr())) & 0x30 != 0 && (person.non_player().is_null() || person.player().is_null()))
    {
        reopen(0);
        return;
    }
    if exe::database_get_club_manager(club) != 0 || exe::transfer_manager_is_player_under_offer(read_usize(G_TRANSFER_MANAGER), person, 0) != 0 {
        clear_job_application_date(this, club);
        exe::llist_remove(list, item.0);
        return;
    }
    covered!("sx14_3 appointment");
    let previous = previous_club(person, club);
    if let Some(from) = previous {
        if from.manager() == person && exe::club_can_be_managed(from) != 0 {
            set_season_expectations(this, from.addr(), 1);
            add_job_vacancy(this, from.addr(), person.addr());
        }
    }
    clear_job_application_date(this, club);
    if person.non_player().is_null() {
        show_error_at(0x9f18a4, 0x5e2);
    }
    release_appointed_person(this, person, previous, club);
    if let Some(from) = previous {
        if !person.player().is_null() && exe::database_is_national_team(from) == 0 && !club.is_null() && exe::database_is_national_team(club) == 0 {
            record_manager_move(person, from, club);
        }
    }
    install_manager(this, person, club);
    // simplification: the original then syncs a human's scout lists and training schedules (0x7f7270, 0x8b2470), a
    // test that never holds here (the humans took `human_vacancy_reached`)
    exe::llist_remove(list, item.0);
    reset_board_for_new_manager(this, club.addr());
}

/// A vacancy still before its arrival date: once its rumour date passed, the rumour linking the candidate with the
/// club is posted once (`post_job_rumour_news`), while the club has no manager.
pub(crate) unsafe fn vacancy_rumour(this: usize, item: VacancyItem) {
    let e = item.0;
    if read_u8(e) == 0 || !item.date_passed(0x1e) || read_u8(e + 1) != 0 || read_i32(e + 2) == -1 {
        return;
    }
    write_u8(e + 1, 1);
    let club = item.club();
    if club.manager().is_null() {
        covered!("sx14_3 rumour");
        post_job_rumour_news(this, staff_at(read_i32(e + 2) as isize).addr(), club.addr());
    }
}

/// The base daily update of a nation's awards (`NationAwards::daily_update`: the month change).
pub(crate) unsafe fn nation_awards_base_daily_update(this: usize, today: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_NATION_AWARDS_BASE_DAILY_UPDATE);
    f(this, today)
}

/// Sets the club's `board_record::JOB_APPLICATION_DATE` to "never" (23/07/1971).
pub(crate) unsafe fn clear_job_application_date(this: usize, club: Club) {
    let mut never = [0u16; 4];
    let d = exe::date_date_549f70(&mut never, 0x17, 7, 0x7b3, u32::MAX);
    manager_manager_copy_date(board_row(this, club) + board_record::JOB_APPLICATION_DATE, d);
}

/// True for the 16 human managers (the last ids of the staff table).
pub(crate) unsafe fn is_human_id(person: Staff) -> bool {
    read_i32(person.addr()) >= read_i32(G_STAFF_COUNT) - 0x10
}

/// A human manager's vacancy whose arrival date came: without a classification the vacancy reopens; without a club or
/// out of work for 90 days the club offers him the job (`FN_POST_JOB_OFFER_TO_HUMAN_NEWS`) and the vacancy reopens;
/// otherwise the vacancy reopens too (the order of the list calls is the original's).
pub(crate) unsafe fn human_vacancy_reached(this: usize, list: usize, item: VacancyItem, person: Staff, club: Club) {
    if person.classification() == 0 {
        clear_job_application_date(this, club);
        exe::llist_remove(list, item.0);
        add_job_vacancy(this, club.addr(), person.addr());
    } else if person.club_job().is_null()
        || exe::date_days_since(G_CURRENT_DATE, person.date_joined_club() as u32, person.date_joined_club_leap_year() as u32) >= 0x5a
    {
        covered!("sx14_3 offer to the human");
        let offer: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_POST_JOB_OFFER_TO_HUMAN_NEWS);
        offer(this, club.addr(), person.addr());
        exe::llist_remove(list, item.0);
        add_job_vacancy(this, club.addr(), person.addr());
    } else {
        clear_job_application_date(this, club);
        add_job_vacancy(this, club.addr(), person.addr());
        exe::llist_remove(list, item.0);
    }
}

/// The club the appointed person leaves: his national team (for a national team, unless his job there is 11), else
/// his club; an AI manager going from a club to a national team with reputation above 6000 that is not a B team
/// (`exe::club_is_national_team`) leaves his club too. None otherwise.
pub(crate) unsafe fn previous_club(person: Staff, club: Club) -> Option<Club> {
    let national = exe::database_is_national_team(club) != 0;
    if national && !person.national_job().is_null() && person.job_for_nation() != 0xb {
        return Some(person.national_job());
    }
    if !national && !person.club_job().is_null() {
        return Some(person.club_job());
    }
    if !is_human_id(person) && national && exe::club_is_national_team(club) == 0 && club.reputation() > 0x1770 && !person.club_job().is_null() {
        return Some(person.club_job());
    }
    None
}

/// Posts the appointment and frees the person from the club he leaves (`leave_*`), ending his contract
/// (`exe::contract_manager_remove_contract`).
pub(crate) unsafe fn release_appointed_person(this: usize, person: Staff, previous: Option<Club>, club: Club) {
    let contracts = read_usize(G_CONTRACT_MANAGER);
    let appoint = |from: usize, kind: u32| post_manager_appointment_news(this, person.addr(), from, club.addr(), kind);
    if exe::database_is_national_team(club) == 0 {
        match previous {
            Some(from) if from == person.club_job() => {
                let kind = leave_club_for_club(this, person, from);
                appoint(from.addr(), kind);
                exe::contract_manager_remove_contract(contracts, person);
            }
            Some(from) if !is_human_id(person) && from == person.national_job() => {
                let kind = leave_national_team_for_club(this, person, from);
                appoint(from.addr(), kind);
                exe::contract_manager_remove_contract(contracts, person);
            }
            _ => appoint(0, 2),
        }
    } else {
        match previous {
            Some(from) if from == person.national_job() || from == person.club_job() => {
                covered!("sx14_3 to a national team");
                let kind = leave_for_national_team(this, person, from);
                appoint(from.addr(), kind);
                if exe::database_is_national_team(from) == 0 {
                    exe::contract_manager_remove_contract(contracts, person);
                }
            }
            _ => appoint(0, 2),
        }
    }
}

/// A player moving between two clubs goes into the past transfers (`exe::transfer_manager_add_past_transfer`): the
/// person, his three name ids (−1 without a name), the new and the old club, no fee, type 5 and today.
pub(crate) unsafe fn record_manager_move(person: Staff, previous: Club, club: Club) {
    let mut rec = [0u8; 0x25];
    let r = rec.as_mut_ptr() as usize;
    exe::date_date_54a110(r + 0x1d);
    write_i32(r, read_i32(person.addr()));
    for (k, name) in [person.first_name(), person.second_name(), person.common_name()].into_iter().enumerate() {
        write_i32(r + 4 + 4 * k, if name == 0 { -1 } else { read_i32(name + names::ID) });
    }
    write_i32(r + 0x10, read_i32(club.addr()));
    write_i32(r + 0x14, read_i32(previous.addr()));
    write_i32(r + 0x18, 0); // the fee
    write_u8(r + 0x1c, 5); // a non-player job
    manager_manager_copy_date(r + 0x1d, G_CURRENT_DATE);
    exe::transfer_manager_add_past_transfer(read_usize(G_TRANSFER_MANAGER), rec.as_mut_ptr());
}

/// The person takes the club: the joining date, the club's manager, and at a club the job (a player-manager, job 0xc,
/// with a free squad place, his reputation + 750 above the club's, fewer than 45 in the squad and outside Italy; else
/// manager, job 5) and the board's wealth band; at a national team the national job (5).
pub(crate) unsafe fn install_manager(this: usize, person: Staff, club: Club) {
    if exe::database_is_national_team(club) != 0 {
        manager_manager_copy_date(person.date_joined_nation_addr(), G_CURRENT_DATE);
    } else {
        manager_manager_copy_date(person.addr() + staff::DATE_JOINED_CLUB, G_CURRENT_DATE);
    }
    club.set_manager(person);
    let row = board_row(this, club);
    if exe::database_is_national_team(club) != 0 {
        person.set_national_job(club);
        write_u8(row + board_record::WEALTH_BAND, 0);
        person.set_job_for_nation(5);
        return;
    }
    person.set_club_job(club);
    let player = person.player();
    let nation = club.nation();
    let free_place = if !player.is_null()
        && (club.reputation() as i32) < player.current_reputation() as i32 + 0x2ee
        && (exe::database_count_club_squad(club, 0, 0) as i8) < 0x2d
        && !nation.is_null()
        && read_i32(nation.addr()) != read_i32(KEY_NATION_ITALY)
    {
        (0..0x32).find(|&k| club.squad(k) == 0)
    } else {
        None
    };
    match free_place {
        Some(k) => {
            covered!("sx14_3 player-manager");
            club.set_squad(k, person.addr());
            person.set_job_for_club(0xc);
            if exe::database_is_national_team(club) == 0 {
                let contracts = read_usize(G_CONTRACT_MANAGER);
                exe::contract_manager_remove_contract(contracts, person);
                exe::contract_manager_add_staff_contract(contracts, person, person.value() as u32);
                let squads = read_usize(G_SQUAD_MANAGER);
                exe::squad_manager_set_squad_number(squads, person, club, 0);
                if exe::squad_manager_club_uses_squad_numbers(squads, club, 0) != 0 {
                    exe::squad_manager_allocate_squad_number(squads, club, 0usize);
                }
            }
        }
        None => person.set_job_for_club(5),
    }
    let finance = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club);
    write_u8(row + board_record::WEALTH_BAND, if finance != 0 { exe::finance_get_wealth_band(finance) } else { 0 });
}

/// Moving to a club from his own club: frees his jobs there by his club job (`staff::JOB_FOR_CLUB`) and returns the
/// appointment kind (1 a player, 3 a manager or player-manager, 2 otherwise).
pub(crate) unsafe fn leave_club_for_club(this: usize, person: Staff, from: Club) -> u32 {
    match person.job_for_club() as i8 {
        5 => {
            remove_staff(this, person, from);
            3
        }
        6 | 8 => {
            remove_staff(this, person, from);
            2
        }
        0xb => {
            remove_player(this, person, from);
            1
        }
        0xc => {
            remove_player(this, person, from);
            remove_staff(this, person, from);
            3
        }
        0xf => {
            remove_player(this, person, from);
            remove_staff(this, person, from);
            2
        }
        _ => {
            if person.non_player().is_null() {
                person.set_job_for_club(0xb);
            } else {
                remove_staff(this, person, from);
            }
            if person.player().is_null() {
                person.set_job_for_club(if person.non_player().is_null() { 0 } else { 8 });
            } else {
                remove_player(this, person, from);
            }
            2
        }
    }
}

/// An AI person moving to a club from his national team: frees his jobs by his national job
/// (`staff::JOB_FOR_NATION`), although the default case writes his club job, and returns the appointment kind.
pub(crate) unsafe fn leave_national_team_for_club(this: usize, person: Staff, from: Club) -> u32 {
    match person.job_for_nation() as i8 {
        5 | 0xc => {
            remove_staff(this, person, from);
            3
        }
        6 | 8 | 0xf => {
            remove_staff(this, person, from);
            2
        }
        0xb => 1,
        _ => {
            if person.non_player().is_null() {
                person.set_job_for_club(0xb);
            } else {
                remove_staff(this, person, from);
            }
            if !person.player().is_null() {
                remove_player(this, person, from);
            } else {
                person.set_job_for_club(if person.non_player().is_null() { 0 } else { 8 });
            }
            2
        }
    }
}

/// Moving to a national team: frees his jobs at the club or national team he leaves by his national job and returns
/// the appointment kind; a job 15 without a player record shows the original's error box (line 0x6bb).
pub(crate) unsafe fn leave_for_national_team(this: usize, person: Staff, from: Club) -> u32 {
    match person.job_for_nation() as i8 {
        5 => {
            remove_staff(this, person, from);
            3
        }
        6 | 8 => {
            remove_staff(this, person, from);
            2
        }
        0xb => {
            remove_player(this, person, from);
            1
        }
        0xc => {
            remove_player(this, person, from);
            remove_staff(this, person, from);
            2
        }
        0xf => {
            if person.player().is_null() {
                show_error_at(0x9f18a4, 0x6bb);
            } else {
                remove_player(this, person, from);
            }
            remove_staff(this, person, from);
            2
        }
        _ => {
            if person.non_player().is_null() {
                person.set_job_for_nation(0xb);
            } else {
                remove_staff(this, person, from);
            }
            if !person.player().is_null() {
                remove_player(this, person, from);
            } else {
                person.set_job_for_club(if person.non_player().is_null() { 0 } else { 8 });
            }
            2
        }
    }
}

/// Frees the person's staff job at `from` when he has a manager record (`remove_staff_from_club`).
pub(crate) unsafe fn remove_staff(this: usize, person: Staff, from: Club) {
    if !person.non_player().is_null() {
        remove_staff_from_club(this, person.addr(), from.addr());
    }
}

/// The argument shapes of the board's match verdicts (the format call of each case of
/// `build_board_match_reaction_text`): who comes first and whether the score is quoted.
#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) enum BoardVerdictArgs {
    /// friendly: the opponent's gender twice, the board, the score and the opponent's name
    Friendly,
    /// the opponent and the competition, the board, the opponent's name and the competition's short name
    OpponentInComp,
    /// as `OpponentInComp`, with the score after the board
    OpponentInCompWithScore,
    /// the competition first: its gender, the opponent's, the board, the score, the competition's short name and the
    /// opponent's name
    CompThenOpponentWithScore,
}

/// `MANAGER_MANAGER::post_job_offer_to_human_news` (0x6a4f80; thiscall(this, club, person), ret 8): the job offer to a
/// human manager.
pub(crate) const FN_POST_JOB_OFFER_TO_HUMAN_NEWS: usize = 0x6a4f80;

/// The vacancy item (`manager_manager_fields::VACANCY_LIST`, 0x26 bytes): the rumour flags (+0 posted, +1 done), the
/// linked candidate (+2), the chosen person (+6), the club (`job_vacancy::CLUB_ID`), the arrival date (+0x16 day,
/// +0x18 year) and the rumour date (+0x1e day, +0x20 year).
#[derive(Clone, Copy)]
pub(crate) struct VacancyItem(pub(crate) usize);

impl VacancyItem {
    pub(crate) unsafe fn person(self) -> Staff {
        staff_at(read_i32(self.0 + 6) as isize)
    }
    pub(crate) unsafe fn club(self) -> Club {
        club_at(read_i32(self.0 + job_vacancy::CLUB_ID) as isize)
    }
    /// True once today is past the date at `day` (+0 day, +2 year).
    pub(crate) unsafe fn date_passed(self, day: usize) -> bool {
        let (today, year) = (read_i16(G_CURRENT_DATE), read_u16(G_CURRENT_DATE + 2));
        let year_due = read_u16(self.0 + day + 2);
        year > year_due || (year == year_due && today > read_i16(self.0 + day))
    }
}

pub(crate) const FN_NATION_AWARDS_BASE_DAILY_UPDATE: usize = 0x768310;

/// Builds the board object (MANAGER_MANAGER, `G_MANAGER_MANAGER`): waits for the game CD, creates the five lists and,
/// with a save, reads them and the board records from man_conf.dat (`load_man_conf`); without a save, or when the
/// reading fails (everything it built freed, as `drop_manager_manager`, and the lists made again after the table),
/// the table for every club (`manager_manager_new_table`, fatal box at line 0x69 or 0x9f) filled fresh by
/// `init_club_board_records`. Returns the object.
/// Original: `MANAGER_MANAGER::MANAGER_MANAGER` (0x68dbf0, manager_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn new_manager_manager(this: usize, save: usize) -> usize {
    manager_manager_find_cd();
    manager_manager_new_lists(this);
    if save == 0 {
        manager_manager_new_table(this, 0x69);
    } else {
        if load_man_conf(this, save) != 0 {
            covered!("sx14b_2 board from the save");
            return this;
        }
        covered!("sx14b_2 board save unreadable");
        drop_manager_manager(this);
        manager_manager_new_table(this, 0x9f);
        manager_manager_new_lists(this);
    }
    init_club_board_records(this);
    this
}

/// Comparer for `new_manager_manager`: it allocates and reads the save item, so it cannot run twice; only the original
/// runs. The copy is proven by the new career (Rust mode) and by loading the holiday save (levas/N/CARGA).
pub(crate) unsafe extern "thiscall" fn compare_new_manager_manager(this: usize, save: usize) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_NEW_MANAGER_MANAGER)(this, save)
}

/// Frees the board object: the board record table (`free`) and the five lists (`LLIST::~LLIST` and `operator
/// delete`), in the order +8, +4, +0xc, +0x10, +0x14; each present one freed and its pointer zeroed.
/// Original: `MANAGER_MANAGER::~MANAGER_MANAGER` (0x68e0e0, manager_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn drop_manager_manager(this: usize) {
    use manager_manager_fields as f;
    let table = read_usize(this + f::BOARD_RECORDS);
    if table != 0 {
        exe::crt_free(table);
        write_usize(this + f::BOARD_RECORDS, 0);
    }
    for field in [f::VACANCY_LIST, f::DEPARTURE_LIST, 0xc, f::ULTIMATUM_LIST, f::BAN_APPEAL_LIST] {
        let list = read_usize(this + field);
        if list != 0 {
            exe::drop_llist(list);
            exe::crt_operator_delete(list);
            write_usize(this + field, 0);
        }
    }
}

/// Comparer for `drop_manager_manager`: it frees, so it cannot run twice; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_drop_manager_manager(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_DROP_MANAGER_MANAGER)(this)
}

/// Reads the board object from the save's "man_conf.dat": nothing (0) without a save or the item. The club count and
/// the board records (`board_record`, `malloc`; 0x41-byte records before `MAN_CONF_49_BYTE_RECORDS_VERSION`,
/// converted), then the lists: vacancies (0x26-byte items), departures (6), +0xc (0xe) and, from
/// `MAN_CONF_LISTS_VERSION`, the ultimatums (4) and the ban appeals (0x19, or 0x15 converted before
/// `MAN_CONF_25_BYTE_APPEALS_VERSION`). The item is closed in every case after it opened; 1 when everything was read,
/// 0 on a short read, an empty table (club count 0) or a failed allocation (with no box: the constructor then builds
/// the board fresh).
/// Original: `MANAGER_MANAGER::load_man_conf` (0x68e180, manager_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn load_man_conf(this: usize, save: usize) -> u32 {
    if save == 0 {
        return 0;
    }
    let mut z = exe::zopen(save, MAN_CONF_DAT, 0, 0x16, 4);
    if z == 0 {
        return 0;
    }
    covered!("sx14b_2 load man_conf.dat");
    // the local dates the original builds with Date::Date (0x54a110) and never reads are left out
    let ok = man_conf_read(this, z);
    exe::zclose(&mut z);
    ok as u32
}

/// Comparer for `load_man_conf`: it reads the save item and allocates, so it cannot run twice; only the original runs.
/// The copy is proven by loading the holiday save in the save proof (levas/N/CARGA).
pub(crate) unsafe extern "thiscall" fn compare_load_man_conf(this: usize, save: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_LOAD_MAN_CONF)(this, save)
}

/// Writes the board object to the save's "man_conf.dat" (`exe::zopen` for writing): the club count, the board
/// records (`G_CLUB_COUNT` × 0x49 bytes) and the five lists (vacancies 0x26-byte items, departures 6, +0xc 0xe,
/// ultimatums 4, ban appeals 0x19), each with its count. Without a save (row 0x21a) or the item (0x222): the
/// manager_manager.cpp "Error" box; any later failure (club count not written 0x228, no clubs 0x22e, records 0x234,
/// then the rows of each list) shows the box and closes the item; done, the item is closed.
/// Original: `MANAGER_MANAGER::save_man_conf` (0x68e920, manager_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn save_man_conf(this: usize, save: usize) {
    const MANAGER_MANAGER_CPP: usize = 0x9f18a4;
    if save == 0 {
        return show_error_box(MANAGER_MANAGER_CPP, 0x21a);
    }
    let mut z = exe::zopen(save, 0x9f18d8, 1, 0x16, 4); // "man_conf.dat"
    if z == 0 {
        return show_error_box(MANAGER_MANAGER_CPP, 0x222);
    }
    covered!("sx14b_3 save man_conf.dat");
    if let Err(row) = man_conf_write(this, z) {
        show_error_box(MANAGER_MANAGER_CPP, row);
    }
    exe::zclose(&mut z);
}

/// Comparer for `save_man_conf`: it writes the save item, so it cannot run twice; only the original runs. The copy is
/// proven by the save proof (the .qst written with and without the routine, compared item by item).
pub(crate) unsafe extern "thiscall" fn compare_save_man_conf(this: usize, save: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_SAVE_MAN_CONF)(this, save)
}

/// The CD search at the start of the constructor: each CD drive (`GetLogicalDriveStringsA`, `GetDriveTypeA` = 5) is
/// looked at for "<root>cm0102.exe" (`exe::file_find_first`) until one has it, with the "Insert CD" box (OK) in
/// between. A no-CD exe patched in this routine (as cm0102-1080) is honoured by reading those bytes.
pub(crate) unsafe fn manager_manager_find_cd() {
    let drives: unsafe extern "system" fn(u32, *mut u8) -> u32 = core::mem::transmute(read_usize(0x9670f8));
    let kind: unsafe extern "system" fn(usize) -> u32 = core::mem::transmute(read_usize(0x967100));
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let any_drive = read_u8(MANAGER_MANAGER_CD_TYPE_JUMP) == 0x90;
    let always_found = read_u8(MANAGER_MANAGER_CD_FOUND_TEST) != 0x85;
    loop {
        // simplification: the buffer starts zeroed (as check_game_cd)
        let mut list = [0u8; 0x100];
        if drives(200, list.as_mut_ptr()) != 0 && list[0] != 0 {
            let mut i = 0usize;
            loop {
                let root = list.as_ptr() as usize + i;
                let t = kind(root);
                if any_drive || t == 5 {
                    let (mut name, mut found) = ([0u8; 0x104], [0u8; 0x114]);
                    format(name.as_mut_ptr() as usize, 0x987e00, root, 0x63u32, 0x6du32, 0x30u32, 0x31u32, 0x30u32, 0x32u32, 0x2eu32, 0x65u32, 0x78u32, 0x65u32);
                    if exe::file_find_first(name.as_ptr() as usize, found.as_mut_ptr() as usize, 0) == 0 || always_found {
                        return;
                    }
                    exe::file_find_close(found.as_mut_ptr() as usize);
                }
                // simplification: the original steps with a signed byte (only past 32 drives would it differ)
                i += 4;
                if list[i] == 0 {
                    break;
                }
            }
        }
        covered!("sx14b_2 board insert CD");
        let (mut title, mut message) = ([0u8; 0x100], [0u8; 0x100]);
        text(title.as_mut_ptr() as usize, 0x987df4); // "Insert CD"
        text(message.as_mut_ptr() as usize, 0x987dc4); // "Please insert the CM01/02 CD and click on OK"
        exe::gui_show_message_box(title.as_ptr() as usize, message.as_ptr() as usize, 1);
    }
}

/// The five owned lists, in the original's order: vacancies (+8), departures (+4), +0xc (0xe-byte items), ultimatums
/// (+0x10) and ban appeals (+0x14); each `operator new(0xe)` + `LLIST::LLIST`, 0 when the allocation fails.
pub(crate) unsafe fn manager_manager_new_lists(this: usize) {
    use manager_manager_fields as f;
    for field in [f::VACANCY_LIST, f::DEPARTURE_LIST, 0xc, f::ULTIMATUM_LIST, f::BAN_APPEAL_LIST] {
        let list = exe::crt_operator_new(0xe);
        write_usize(this + field, if list != 0 { exe::new_llist(list) } else { 0 });
    }
}

/// The board record table for every club (`G_CLUB_COUNT` × 0x49 bytes, `malloc`); out of memory, the fatal
/// manager_manager.cpp box at `row` (the box, `game_fatal_cleanup`, exit(−1)).
pub(crate) unsafe fn manager_manager_new_table(this: usize, row: u32) {
    let table = exe::crt_malloc(read_u32(G_CLUB_COUNT).wrapping_mul(0x49));
    write_usize(this + manager_manager_fields::BOARD_RECORDS, table);
    if table == 0 {
        let exit: unsafe extern "C" fn(i32) -> ! = core::mem::transmute(FN_CRT_EXIT);
        show_error_box(MANAGER_MANAGER_CPP_PATH, row);
        exe::game_fatal_cleanup();
        exe::shared_noop_ret0_5a8f60();
        exit(-1);
    }
}

/// The body of `load_man_conf` with the item open: false on a short read, an empty table or a failed allocation.
pub(crate) unsafe fn man_conf_read(this: usize, z: usize) -> bool {
    use manager_manager_fields as f;
    let mut count = 0i32;
    if exe::zread(&mut count as *mut i32 as usize, 4, 1, z) != 1 || count == 0 {
        return false;
    }
    let table = exe::crt_malloc((count as u32).wrapping_mul(0x49));
    write_usize(this + f::BOARD_RECORDS, table);
    if table == 0 {
        return false;
    }
    let version = read_i32(G_GAME_DATA_VERSION).wrapping_abs();
    if version > MAN_CONF_49_BYTE_RECORDS_VERSION {
        if exe::zread(table, 0x49, count as u32, z) != count as u32 {
            return false;
        }
    } else {
        covered!("sx14b_2 old board records");
        let old = exe::crt_malloc((count as u32).wrapping_mul(0x41));
        if old == 0 {
            return false;
        }
        if exe::zread(old, 0x41, count as u32, z) != count as u32 {
            exe::crt_free(old);
            return false;
        }
        man_conf_convert_old_records(table, old, count);
        exe::crt_free(old);
    }
    if !man_conf_read_list(z, read_usize(this + f::VACANCY_LIST), 0x26)
        || !man_conf_read_list(z, read_usize(this + f::DEPARTURE_LIST), 6)
        || !man_conf_read_list(z, read_usize(this + 0xc), 0xe)
    {
        return false;
    }
    if version <= MAN_CONF_LISTS_VERSION {
        return true;
    }
    if !man_conf_read_list(z, read_usize(this + f::ULTIMATUM_LIST), 4) {
        return false;
    }
    let appeals = read_usize(this + f::BAN_APPEAL_LIST);
    if version >= MAN_CONF_25_BYTE_APPEALS_VERSION {
        return man_conf_read_list(z, appeals, 0x19);
    }
    let mut n = 0i32;
    if exe::zread(&mut n as *mut i32 as usize, 4, 1, z) != 1 {
        return false;
    }
    covered!("sx14b_2 old ban appeals");
    man_conf_read_old_appeals(z, appeals, n)
}

/// The body of `save_man_conf` with the item open: the error row on a failure.
pub(crate) unsafe fn man_conf_write(this: usize, z: usize) -> Result<(), u32> {
    use manager_manager_fields as f;
    if exe::zwrite(G_CLUB_COUNT, 4, 1, z) != 1 {
        return Err(0x228);
    }
    let clubs = read_u32(G_CLUB_COUNT);
    if clubs == 0 {
        return Err(0x22e);
    }
    if exe::zwrite(read_usize(this + f::BOARD_RECORDS), 0x49, clubs, z) != read_u32(G_CLUB_COUNT) {
        return Err(0x234);
    }
    man_conf_write_list(z, read_usize(this + f::VACANCY_LIST), 0x26, (0x23d, 0x247, 0x24d))?;
    man_conf_write_list(z, read_usize(this + f::DEPARTURE_LIST), 6, (0x256, 0x260, 0x266))?;
    man_conf_write_list(z, read_usize(this + 0xc), 0xe, (0x26f, 0x279, 0x27f))?;
    man_conf_write_list(z, read_usize(this + f::ULTIMATUM_LIST), 4, (0x288, 0x292, 0x298))?;
    man_conf_write_list(z, read_usize(this + f::BAN_APPEAL_LIST), 0x19, (0x2a2, 0x2ac, 0x2b2))
}

/// Converts the old 0x41-byte board records into the 0x49-byte `board_record`: the bytes as they are (the date pairs
/// through `Date::operator=`), except that +3 takes only its 5 low bits (the 3 high bits keep what `malloc` left, as
/// in the original), and `board_record::LAST_ULTIMATUM_DATE` becomes "never" (23/06/1971).
pub(crate) unsafe fn man_conf_convert_old_records(table: usize, old: usize, count: i32) {
    for k in 0..count.max(0) as usize {
        let (o, n) = (old + k * 0x41, table + k * 0x49);
        write_u8(n, read_u8(o));
        write_u8(n + 1, read_u8(o + 1));
        write_u8(n + 2, read_u8(o + 2));
        write_u8(n + 3, (read_u8(n + 3) & 0xe0) | (read_u8(o + 3) & 0x1f));
        core::ptr::copy_nonoverlapping((o + 4) as *const u8, (n + 4) as *mut u8, 0x14);
        let mut tmp = [0u8; 8];
        exe::date_operator_3d(n + board_record::LAST_REVIEW_DATE, tmp.as_mut_ptr() as usize, o + board_record::LAST_REVIEW_DATE);
        exe::date_operator_3d(n + board_record::JOB_APPLICATION_DATE, tmp.as_mut_ptr() as usize, o + board_record::JOB_APPLICATION_DATE);
        core::ptr::copy_nonoverlapping((o + 0x28) as *const u8, (n + 0x28) as *mut u8, 0x18);
        write_u8(n + board_record::DISCIPLINE_COUNTERS, read_u8(o + board_record::DISCIPLINE_COUNTERS));
        let mut never = [0u16; 4];
        let d = exe::date_date_549f70(&mut never, 0x17, 6, 0x7b3, u32::MAX);
        exe::date_operator_3d(n + board_record::LAST_ULTIMATUM_DATE, tmp.as_mut_ptr() as usize, d);
    }
}

/// Reads one list of man_conf.dat: the count (dword), then each item of `size` bytes appended to `list`
/// (`LLIST::add_tail`); false on a short read.
pub(crate) unsafe fn man_conf_read_list(z: usize, list: usize, size: u32) -> bool {
    let mut count = 0i32;
    if exe::zread(&mut count as *mut i32 as usize, 4, 1, z) != 1 {
        return false;
    }
    let mut item = [0u8; 0x26];
    for _ in 0..count {
        if exe::zread(item.as_mut_ptr() as usize, size, 1, z) != 1 {
            return false;
        }
        exe::llist_add_tail(list, item.as_mut_ptr() as usize, size);
    }
    true
}

/// The ban appeal list with the old 0x15-byte items (before `MAN_CONF_25_BYTE_APPEALS_VERSION`): each becomes the
/// 0x19-byte item with its 0x15 bytes (the date through `Date::operator=`) and 0xff in the four new ones.
pub(crate) unsafe fn man_conf_read_old_appeals(z: usize, list: usize, count: i32) -> bool {
    for _ in 0..count {
        let (mut old, mut item) = ([0u8; 0x15], [0u8; 0x19]);
        if exe::zread(old.as_mut_ptr() as usize, 0x15, 1, z) != 1 {
            return false;
        }
        item[..5].copy_from_slice(&old[..5]);
        let mut tmp = [0u8; 8];
        exe::date_operator_3d(item.as_mut_ptr() as usize + 5, tmp.as_mut_ptr() as usize, old.as_mut_ptr() as usize + 5);
        item[0xd..0x15].copy_from_slice(&old[0xd..0x15]);
        item[0x15..].fill(0xff);
        exe::llist_add_tail(list, item.as_mut_ptr() as usize, 0x19);
    }
    true
}

/// Writes one list to man_conf.dat: the count (`LLIST` +0, a signed word, as a dword) and each item of `size` bytes
/// from the head. Answers the manager_manager.cpp box row of the failure: `rows` = (count not written, list shorter
/// than its count, item not written).
pub(crate) unsafe fn man_conf_write_list(z: usize, list: usize, size: u32, rows: (u32, u32, u32)) -> Result<(), u32> {
    let mut count = read_i16(list) as i32;
    if exe::zwrite(&mut count as *mut i32 as usize, 4, 1, z) != 1 {
        return Err(rows.0);
    }
    write_u32(list + 0xa, read_u32(list + 2)); // the cursor at the head
    let mut k = 0i32;
    while k < read_i16(list) as i32 {
        let item = exe::llist_get_next(list);
        if item == 0 {
            return Err(rows.1);
        }
        if exe::zwrite(item, size, 1, z) != 1 {
            return Err(rows.2);
        }
        k += 1;
    }
    Ok(())
}

/// The source path of manager_manager.cpp in the "Error" box ("E:\dev\CM3\cm3 00-01\cm3\code\manager_manager.cpp").
pub(crate) const MANAGER_MANAGER_CPP_PATH: usize = 0x9f18a4;

/// The save item "man_conf.dat".
pub(crate) const MAN_CONF_DAT: usize = 0x9f18d8;

/// The game data versions of the man_conf.dat layout: board records of 0x49 bytes above the first (before, 0x41 bytes
/// without the last ultimatum date); the ultimatum and ban appeal lists above the second; 0x19-byte ban appeals from
/// the third (before, 0x15 bytes).
pub(crate) const MAN_CONF_49_BYTE_RECORDS_VERSION: i32 = 0x3e040;

pub(crate) const MAN_CONF_LISTS_VERSION: i32 = 0x3e033;

pub(crate) const MAN_CONF_25_BYTE_APPEALS_VERSION: i32 = 0x3e035;

/// The bytes the original patches to drop the CD requirement in this routine: the drive type's `jne` (75 56, `nop`s in
/// cm0102-1080) and the found test (85 c0, `xor` in cm0102-1080).
pub(crate) const MANAGER_MANAGER_CD_TYPE_JUMP: usize = 0x68dc41;

pub(crate) const MANAGER_MANAGER_CD_FOUND_TEST: usize = 0x68dc85;

/// Handles a manager's request to the FA: 1 a referee complaint, 2 a postponement.
///
/// `MANAGER_MANAGER::handle_fa_request(club, request)`, called after a match for the machine's clubs (request 1,
/// `update_board_confidence_after_match`) and from the FA request screen. SEH frame for the locals: two unused match
/// team blocks (`exe::crt_vector_constructor_iterator`, 0x625 bytes each), a fixture list (`exe::new_llist`) and a
/// date (`exe::date_date_54a110`), destroyed at the end. A club without a league (`.division()`), a manager or a
/// non-player record, or a national team: nothing. The club's request count (board record +0) goes up; the limit is
/// 3 above reputation 3250, else 2. A referee complaint at the limit with the club state +4 below −1: the media story
/// of the complaint (kind 4). Over the limit, for a human club or at the limit + 1: the sanction (the story of the
/// outburst, the 25000 fine, the board confidence and the sacking check). Otherwise (other requests, but 5) at the
/// limit the reply with outcome 3; over it, outcome 5 and the board confidence − 100. Below the limit, only a human
/// club's request goes on: the referee complaint (`fa_referee_complaint`) or the postponement
/// (`fa_postponement_request`).
///
/// Defects of the original: the reputation bands are tested from the lowest, so the limits 4 (above 5250) and 7
/// (above 7750) never apply; the rescheduling news reads the fixture after `remove_fixture` took it out of the
/// calendar.
/// Original: `MANAGER_MANAGER::handle_fa_request` (0x6a9e60, manager_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn handle_fa_request(this: usize, club: usize, request: u32) {
    let mut blocks = [0u8; 2 * 0x625];
    let blocks = blocks.as_mut_ptr() as usize;
    exe::crt_vector_constructor_iterator(blocks, 0x625, 2, FA_REQUEST_TEAM_BLOCK_CONSTRUCT, FA_REQUEST_TEAM_BLOCK_DESTRUCT);
    let mut list = [0u8; 0x20];
    let list = list.as_mut_ptr() as usize;
    exe::new_llist(list);
    let mut date = [0u8; 8];
    let date = date.as_mut_ptr() as usize;
    exe::date_date_54a110(date);
    fa_request(this, Club(club), request as i8, list, date);
    exe::drop_llist(list);
    exe::crt_vector_destructor_iterator(blocks, 0x625, 2, FA_REQUEST_TEAM_BLOCK_DESTRUCT);
}

/// Comparer for `handle_fa_request`: it moves fixtures, renames referees, fines, posts news and draws, so only the
/// original runs (its effects are checked by the career).
pub(crate) unsafe extern "thiscall" fn compare_handle_fa_request(this: usize, club: usize, request: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_HANDLE_FA_REQUEST)(this, club, request)
}

/// Posts the 0x1789 news of the FA's reply to a club manager's request.
///
/// SEH frame with no destructor. `MANAGER_MANAGER::post_fa_request_reply_news(club, request, outcome, standing)`: no
/// club or a club without a nation (`.nation()`): nothing. News 0x1789 (local, `exe::new_newsitem`): 0 the club's id,
/// 1 the request, 2 the outcome, 3 the nation's id, 4 the referee's standing (the three as signed bytes); to the club's
/// manager (`exe::news_send_to_club_manager`, which only delivers to a human).
/// Original: `MANAGER_MANAGER::post_fa_request_reply_news` (0x6aaa90, manager_manager.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn post_fa_request_reply_news(_this: usize, club: usize, request: u32, outcome: u32, standing: u32) {
    let club = Club(club);
    if club.is_null() || club.nation().is_null() {
        return;
    }
    covered!("sx14b_1 FA reply news");
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0x1789, 0);
    exe::newsitem_set_slot(n, 0, read_u32(club.addr()));
    exe::newsitem_set_slot(n, 1, request as i8 as i32 as u32);
    exe::newsitem_set_slot(n, 2, outcome as i8 as i32 as u32);
    exe::newsitem_set_slot(n, 3, read_u32(club.nation().addr()));
    exe::newsitem_set_slot(n, 4, standing as i8 as i32 as u32);
    exe::news_send_to_club_manager(read_usize(G_NEWS), n, club);
}

/// Comparer for `post_fa_request_reply_news`: the news goes to the manager's inbox (allocates), so only the original
/// runs.
pub(crate) unsafe extern "thiscall" fn compare_post_fa_request_reply_news(this: usize, club: usize, request: u32, outcome: u32, standing: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32)>(IDX_POST_FA_REQUEST_REPLY_NEWS)(this, club, request, outcome, standing)
}

/// Posts the 0x29 news that the FA moved a fixture, to the managers of both clubs.
///
/// SEH frame with no destructor. `MANAGER_MANAGER::post_match_rescheduled_news(fixture, club id, count)`: no fixture,
/// or before the news start date (`before_news_start_date`): nothing. News 0x29 (local, `exe::new_newsitem`): 0 and 1
/// the home and away clubs' ids, 2 the competition's id, 3 the requesting club's id, 4 the id of the competition's
/// nation, 5 the count (signed byte); to the home club's manager and to the away club's
/// (`exe::news_send_to_club_manager`).
/// Original: `MANAGER_MANAGER::post_match_rescheduled_news` (0x6aa7d0, manager_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn post_match_rescheduled_news(_this: usize, fixture: usize, club: u32, count: u32) {
    let fixture = Fixture(fixture);
    if fixture.is_null() || before_news_start_date() {
        return;
    }
    covered!("sx14b_1 rescheduled news");
    let mut local = [0u32; 0x3a]; // the local news item (0xe8 bytes)
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0x29, 0);
    exe::newsitem_set_slot(n, 0, read_u32(fixture.home_club().addr()));
    exe::newsitem_set_slot(n, 1, read_u32(fixture.away_club().addr()));
    exe::newsitem_set_slot(n, 2, read_u32(fixture.competition().addr()));
    exe::newsitem_set_slot(n, 3, club);
    exe::newsitem_set_slot(n, 4, read_u32(fixture.competition().nation().addr()));
    exe::newsitem_set_slot(n, 5, count as i8 as i32 as u32);
    exe::news_send_to_club_manager(read_usize(G_NEWS), n, fixture.home_club());
    exe::news_send_to_club_manager(read_usize(G_NEWS), n, fixture.away_club());
}

/// Comparer for `post_match_rescheduled_news`: the news goes to the managers' inboxes (allocates), so only the original
/// runs.
pub(crate) unsafe extern "thiscall" fn compare_post_match_rescheduled_news(this: usize, fixture: usize, club: u32, count: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32)>(IDX_POST_MATCH_RESCHEDULED_NEWS)(this, fixture, club, count)
}

/// A postponement request: the first fixture in the next 8 weeks that is not today and not played. A fixture of another
/// competition than the club's league, or the key nation's season ending within 60 days of today (or, when it ends in
/// the first 60 days of the year, today past day 340): outcome 6. Granted with more than 2 squad players on
/// international duty that day, or with 2 of them or more than 4 happy (happiness above 20) and injured players when
/// the club's reputation beats 4750 + random(1250): the fixture is moved (removed, a new date, added again) with the
/// reply (outcome 0) and the rescheduling news; otherwise outcome 1.
pub(crate) unsafe fn fa_postponement_request(this: usize, club: Club, request: i8, list: usize, date: usize) {
    let key = exe::key_nation_index_of_club(club) as i8;
    let (mut a, mut b) = ([0u8; 8], [0u8; 8]);
    let year = read_u16(exe::date_add_weeks(G_CURRENT_DATE, &mut a, 8) + 2);
    let day = read_u16(exe::date_add_weeks(G_CURRENT_DATE, &mut b, 8));
    let (today, this_year) = (read_i16(G_CURRENT_DATE), read_u16(G_CURRENT_DATE_YEAR));
    // simplification: the high halves of the date words pushed to find_fixtures are register garbage in the original
    exe::fixture_manager_find_fixtures(read_usize(G_FIXTURE_MANAGER), today as u16 as u32, this_year as u32, day as u32, year as u32,
        read_u32(club.addr()), list, u32::MAX, (-2i32) as u32, 1);
    if read_u16(list) == 0 {
        return;
    }
    write_u32(list + 0xa, read_u32(list + 2)); // the cursor at the head
    let fixture = loop {
        let f = Fixture(exe::llist_get_next(list));
        if f.is_null() {
            return;
        }
        let is_today = f.day() as i32 == today as i32 && f.year() == this_year;
        if !is_today && f.home_goals() == -1 {
            break f;
        }
    };
    covered!("sx14b_1 postponement");
    let (mut made, mut out) = ([0u8; 8], [0u8; 8]);
    let d = exe::new_date(made.as_mut_ptr() as usize, fixture.day() as u32, fixture.year() as u32);
    exe::date_operator_3d(date, out.as_mut_ptr() as usize, d);
    let mut on_duty = 0u8;
    for k in 0..50 {
        let p = club.squad(k);
        if p != 0 && exe::nationalteams_get_international_duty_club(read_usize(G_NATIONAL_TEAMS), p, date, club, NEWS_TEXT_BUFFER as u32, 0) != 0 {
            on_duty = on_duty.wrapping_add(1);
        }
    }
    let mut injured = 0u8;
    for k in 0..50 {
        let p = club.squad(k);
        if p == 0 {
            continue;
        }
        let c = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), p));
        if !c.is_null() && c.happiness() > 20 && exe::injury_get_player_injury_status(read_usize(G_INJURY), p) == 2 {
            injured = injured.wrapping_add(1);
        }
    }
    if fixture.competition() != club.division() {
        return fa_request_reply(this, club, request, 6, 0);
    }
    if (0..0x23).contains(&key) {
        let e = KeyNationRecord(G_LEAGUE_TABLE + key as usize * 0x48);
        let end = e.end_date();
        if e.end_date_year() as u16 == this_year && if end > 0x3c { (end as i32 - 0x3c) < today as i32 } else { today > 0x154 } {
            covered!("sx14b_1 postponement too late");
            return fa_request_reply(this, club, request, 6, 0);
        }
    }
    let on_duty = on_duty as i8;
    let granted = on_duty > 2 || ((on_duty > 1 || injured as i8 > 4) && club.reputation() > (exe::utils_random(0x4e2) + 0x128e) as i16);
    if !granted {
        return fa_request_reply(this, club, request, 1, 0);
    }
    covered!("sx14b_1 postponement granted");
    let mut copy = [0u8; fixture::SIZE];
    core::ptr::copy_nonoverlapping(fixture.addr() as *const u8, copy.as_mut_ptr(), fixture::SIZE);
    let fm = read_usize(G_FIXTURE_MANAGER);
    exe::fixture_manager_remove_fixture(fm, fixture);
    fa_request_reply(this, club, request, 0, 0);
    exe::fixture_manager_find_new_fixture_date(fm, copy.as_mut_ptr() as usize);
    exe::fixture_manager_add_fixture(fm, copy.as_mut_ptr() as usize, 1);
    // defect of the original: the news reads the fixture that remove_fixture just took out of the calendar
    let news: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(FN_MANAGER_MANAGER_POST_MATCH_RESCHEDULED_NEWS);
    news(this, fixture.addr(), read_u32(club.addr()), on_duty as u8 as u32);
}

/// A referee complaint: the last played fixture of the past 8 weeks; its referee with an average rating below 5.5 and
/// a last match rating below 7 is replaced in 1 of 200 cases (a new name from his nation, discipline random(16) + 5 or
/// random(11) + random(11), born in a random day of (this year − 27 + random(5)), his stats reset, and the news 0x2b to
/// the managers of the club's nation); otherwise the reply (outcome 1) with his last match rating.
pub(crate) unsafe fn fa_referee_complaint(this: usize, club: Club, request: i8, list: usize) {
    let (mut a, mut b) = ([0u8; 8], [0u8; 8]);
    let year = read_u16(exe::date_sub_weeks(G_CURRENT_DATE, &mut a, 8) + 2);
    let day = read_u16(exe::date_sub_weeks(G_CURRENT_DATE, &mut b, 8));
    let (today, this_year) = (read_u16(G_CURRENT_DATE), read_u16(G_CURRENT_DATE_YEAR));
    exe::fixture_manager_find_fixtures(read_usize(G_FIXTURE_MANAGER), day as u32, year as u32, today as u32, this_year as u32,
        read_u32(club.addr()), list, u32::MAX, (-2i32) as u32, 1);
    if read_u16(list) == 0 {
        return;
    }
    write_u32(list + 0xa, read_u32(list + 6)); // the cursor at the tail
    let fixture = loop {
        let f = Fixture(exe::llist_get_prev(list));
        if f.is_null() {
            return;
        }
        if f.home_goals() > -1 {
            break f;
        }
    };
    covered!("sx14b_1 referee complaint");
    let official = Official(read_usize(G_OFFICIAL_TABLE).wrapping_add((fixture.referee() as i32 as usize).wrapping_mul(FA_REQUEST_OFFICIAL_SIZE)));
    let om = read_usize(G_OFFICIALS_MANAGER);
    let average: unsafe extern "thiscall" fn(usize, usize) -> f32 = core::mem::transmute(FN_OFFICIALSMANAGER_GET_AVERAGE_RATING);
    let last: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_OFFICIALSMANAGER_GET_LAST_MATCH_RATING);
    if !((average(om, official.addr()) as f64) < read_f64(rdata::F64_5_5) && (last(om, official.addr()) as i8) < 7) || exe::utils_random(200) != 0 {
        return fa_request_reply(this, club, request, 1, last(om, official.addr()) as u32);
    }
    covered!("sx14b_1 referee replaced");
    let old = read_u32(official.second_name() + names::ID);
    official.set_first_name(exe::database_pick_first_name_for_nation(official.nation()));
    official.set_second_name(exe::database_pick_second_name_for_nation(official.nation()));
    let discipline = if exe::utils_random(10) != 0 {
        (exe::utils_random(0x10) as u8).wrapping_add(5)
    } else {
        let r = exe::utils_random(0xb) as u8;
        r.wrapping_add(exe::utils_random(0xb) as u8)
    };
    official.set_discipline(discipline);
    let r = exe::utils_random(5) as i16;
    official.set_birth_year(r.wrapping_add((this_year as i16).wrapping_sub(0x1b)));
    official.set_birth_day(exe::utils_random(0x154) as i16);
    let reset: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_OFFICIALSMANAGER_RESET_OFFICIAL_STATS);
    reset(om, official.addr());
    let new = read_u32(official.second_name() + names::ID);
    let (nation, manager) = (club.nation(), club.manager());
    if nation.is_null() {
        return;
    }
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0x2b, 0);
    exe::newsitem_set_slot(n, 0, read_u32(nation.addr()));
    exe::newsitem_set_slot(n, 1, old);
    exe::newsitem_set_slot(n, 2, new);
    exe::newsitem_set_slot(n, 3, read_u32(manager.addr()));
    exe::news_send_to_nation_managers(read_usize(G_NEWS), n, nation);
}

/// The request itself (inside the frame of `handle_fa_request`).
pub(crate) unsafe fn fa_request(this: usize, club: Club, request: i8, list: usize, date: usize) {
    if club.is_null() || club.division().is_null() || club.manager().is_null() || club.manager().non_player().is_null() {
        return;
    }
    if exe::database_is_national_team(club) != 0 {
        return;
    }
    let row = BoardRecord(board_row(this, club));
    write_i8(row.addr(), fa_request_count(row).wrapping_add(1));
    // defect of the original: the bands are tested from the lowest, so above 3250 the limit is always 3 (the 4 above
    // 5250 and the 7 above 7750 never apply)
    let limit: i8 = if club.reputation() > 0xcb2 { 3 } else { 2 };
    if request == 1 {
        let count = fa_request_count(row);
        if count == limit && read_i8(club_state_record(club.addr()) + 4) < -1 {
            covered!("sx14b_1 FA complaint story");
            let story: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32) = core::mem::transmute(FN_MEDIA_QUEUE_MANAGER_FA_COMPLAINT_STORY);
            return story(read_usize(G_MEDIA), club.addr(), club.manager().addr(), 4, u32::MAX, u32::MAX, u32::MAX);
        }
        if count > limit && (manager_manager_is_human_club(club.addr(), 1, 0) != 0 || count as i32 == limit as i32 + 1) {
            return fa_request_sanction(this, club, limit);
        }
    }
    let count = fa_request_count(row);
    if request != 5 && count == limit {
        covered!("sx14b_1 request at the limit");
        return fa_request_reply(this, club, request, 3, 0);
    }
    if request != 5 && count > limit {
        covered!("sx14b_1 request over the limit");
        fa_request_reply(this, club, request, 5, 0);
        let row = BoardRecord(board_row(this, club));
        return row.set_confidence(row.confidence().wrapping_sub(100));
    }
    if manager_manager_is_human_club(club.addr(), 1, 0) == 0 {
        return;
    }
    match request {
        1 => fa_referee_complaint(this, club, request, list),
        2 => fa_postponement_request(this, club, request, list, date),
        _ => {}
    }
}

/// The FA's reply news to `club` (through the exe address, so the hook and its comparer run).
pub(crate) unsafe fn fa_request_reply(this: usize, club: Club, request: i8, outcome: u32, standing: u32) {
    let post: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) = core::mem::transmute(FN_MANAGER_MANAGER_POST_FA_REQUEST_REPLY_NEWS);
    post(this, club.addr(), request as i32 as u32, outcome, standing);
}

/// The board's count of the club's FA requests (board record +0, unnamed; signed byte).
pub(crate) unsafe fn fa_request_count(row: BoardRecord) -> i8 {
    read_i8(row.addr())
}

/// The club's sanction after one complaint too many: the media story of the outburst (kind 5, random(3) + 1), a
/// 25000 league fine, the board confidence (in `G_MANAGER_MANAGER`'s table) moved by 250 (or, with the club state +4
/// below 0, by limit × 250 − the count), kept within 1..10000, and the sacking check.
pub(crate) unsafe fn fa_request_sanction(this: usize, club: Club, limit: i8) {
    covered!("sx14b_1 FA sanction");
    let story: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32) = core::mem::transmute(FN_MEDIA_QUEUE_MANAGER_FA_COMPLAINT_STORY);
    let r = exe::utils_random(3);
    story(read_usize(G_MEDIA), club.addr(), club.manager().addr(), 5, u32::MAX, u32::MAX, (r as u8).wrapping_add(1) as u32);
    let finance = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club);
    if finance != 0 {
        let fine: unsafe extern "thiscall" fn(usize, i32) = core::mem::transmute(FN_FINANCE_PAY_LEAGUE_FINE);
        fine(finance, 25000);
    }
    let manager_manager = read_usize(G_MANAGER_MANAGER);
    let row = BoardRecord(board_row(manager_manager, club));
    let delta = if read_i8(club_state_record(club.addr()) + 4) < 0 {
        // the count from this object's table, the confidence in G_MANAGER_MANAGER's (the same object in the game)
        (limit as i16).wrapping_mul(250).wrapping_sub(fa_request_count(BoardRecord(board_row(this, club))) as i16)
    } else {
        250
    };
    row.set_confidence(row.confidence().wrapping_add(delta).clamp(1, 10000));
    exe::manager_manager_consider_sacking_manager(manager_manager, club, 0);
}

/// `Media::queue_manager_fa_complaint_story` (0x758de0; thiscall(media, club, person, kind, a, b, c), ret 0x18).
pub(crate) const FN_MEDIA_QUEUE_MANAGER_FA_COMPLAINT_STORY: usize = 0x758de0;

/// `Finance::pay_league_fine` (0x599850; thiscall(finance, amount), ret 4).
pub(crate) const FN_FINANCE_PAY_LEAGUE_FINE: usize = 0x599850;

/// `OfficialsManager::get_average_rating` (0x79d3e0; thiscall(om, official), ret 4; the float in st0).
pub(crate) const FN_OFFICIALSMANAGER_GET_AVERAGE_RATING: usize = 0x79d3e0;

/// `OfficialsManager::get_last_match_rating` (0x79d3b0; thiscall(om, official), ret 4; the char in al).
pub(crate) const FN_OFFICIALSMANAGER_GET_LAST_MATCH_RATING: usize = 0x79d3b0;

/// `OfficialsManager::reset_official_stats` (0x79d190; thiscall(om, official), ret 4).
pub(crate) const FN_OFFICIALSMANAGER_RESET_OFFICIAL_STATS: usize = 0x79d190;

/// `MANAGER_MANAGER::post_fa_request_reply_news` (0x6aaa90; thiscall(this, club, request, outcome, standing), ret 0x10).
pub(crate) const FN_MANAGER_MANAGER_POST_FA_REQUEST_REPLY_NEWS: usize = 0x6aaa90;

/// `MANAGER_MANAGER::post_match_rescheduled_news` (0x6aa7d0; thiscall(this, fixture, club id, count), ret 0xc).
pub(crate) const FN_MANAGER_MANAGER_POST_MATCH_RESCHEDULED_NEWS: usize = 0x6aa7d0;

/// The two 0x625-byte match team blocks the routine builds and destroys without using them.
pub(crate) const FA_REQUEST_TEAM_BLOCK_CONSTRUCT: usize = 0x453c30;

pub(crate) const FA_REQUEST_TEAM_BLOCK_DESTRUCT: usize = 0x453c10;

/// Size of an official record in `G_OFFICIAL_TABLE`.
pub(crate) const FA_REQUEST_OFFICIAL_SIZE: usize = 0x2b;
