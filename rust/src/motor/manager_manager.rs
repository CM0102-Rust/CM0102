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
    if comp.addr() == fixture.home_club().division().addr() {
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
        } else if club.stadium().addr() == ev.addr() {
            aj = -20;
        } else if other.stadium().addr() == ev.addr() {
            aj = 20;
        } else if city(ev.addr()) && read_usize(ev.city() + city::NATION) == other.nation().addr() {
            aj = 20;
        }
    } else if club.stadium().addr() == ev.addr() {
        aj = -20;
    } else if other.stadium().addr() == ev.addr() {
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
    let f1 = Stadium(club.stadium().addr());
    let same = !f1.is_null() && other.stadium().addr() != 0 && f1.city() == other.stadium().city();
    if same {
        fl |= if rival { 0x40 } else { 0x80 };
    } else if rival {
        fl |= 0x80;
        if !f1.is_null() {
            let e1 = f1.city();
            let f2 = Stadium(other.stadium().addr());
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
    fn b(&mut self, o: usize, v: u8) {
        self.0[o] = v;
    }
    fn d(&mut self, o: usize, v: i32) {
        self.0[o..o + 4].copy_from_slice(&v.to_le_bytes());
    }
    fn rd(&self, o: usize) -> i32 {
        i32::from_le_bytes([self.0[o], self.0[o + 1], self.0[o + 2], self.0[o + 3]])
    }
    fn w(&mut self, o: usize, v: u16) {
        self.0[o..o + 2].copy_from_slice(&v.to_le_bytes());
    }
    fn rw(&self, o: usize) -> u16 {
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
            exe::llist_remove(list(), item.addr());
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
        let ct = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), p);
        if ct == 0 {
            continue;
        }
        for (m, f) in CONTRACT_FLAGS_BY_READER {
            if exe::staff_contract_get_flags(ct) & m != 0 {
                let erase: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(f);
                erase(ct, m);
            }
        }
        for m in CONTRACT_FLAGS_AT_45 {
            if read_usize(ct + contract::GRIEVANCES) as u32 & m != 0 {
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
            let c = exe::contract_manager_get_contract_for_club(read_u32(G_CONTRACT_MANAGER) as usize, p, club);
            if c != 0 && read_u32(c + contract::GRIEVANCES) & 0x1000_0000 != 0 {
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
    let days: unsafe extern "thiscall" fn(usize, usize, usize) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
    let mut k = 0i32;
    while k < read_i16(read_usize(this + manager_manager_fields::ULTIMATUM_LIST)) as i32 {
        let item = exe::llist_get_next(read_usize(this + manager_manager_fields::ULTIMATUM_LIST));
        if item != 0 && read_usize(club.addr()) == read_usize(item) {
            exe::llist_remove(read_usize(this + manager_manager_fields::ULTIMATUM_LIST), item);
            let manager = club.manager();
            if !manager.is_null() && days(G_CURRENT_DATE, manager.date_joined_club(), manager.date_joined_club_leap_year()) > 0x64 {
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

