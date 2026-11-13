use super::*;

/// Has `clube` already clinched first place in this league stage (`this`) after match `partida`? Only for counting
/// competitions, with bit 1 of +0xd9 and round +0xa9 >= 24: on a copy of the table updated with the match and sorted,
/// the club must be top and no other club can reach its points (+0x19) with its remaining games (+0xa7 - +0x11) times
/// points per win (+0xc2). Returns 1 if decided, else 0.
pub(crate) unsafe extern "thiscall" fn group_decided(this: usize, club: usize, fixture: usize) -> u32 {
    let fixture = Fixture(fixture);
    let this = League(this);
    if fixture.stage_index() != stage_index::MAIN {
        return 0;
    }
    if exe::comp_util_is_league_division(read_i32(this.competition().addr())) == 0 {
        return 0;
    }
    if (this.league_flags() as u8) & 1 == 0 || this.round_count() < 24 {
        return 0;
    }
    let rows = this.club_count();
    let table = this.table();
    let row = |base: usize, k: i16| base.wrapping_add(k as usize * 0x3b);
    let Some(k) = (0..rows).find(|&k| read_usize(row(table, k)) == club) else {
        return 0;
    };
    let r = LeagueTableRow(row(table, k));
    if r.is_null() || matches!(r.status() as i8 , 5 | 2 | 3 | -2 | -4 | -5) {
        return 0;
    }
    let t = LeagueTableRow(exe::league_copy_table(this));
    if t.is_null() {
        return 0;
    }
    exe::league_apply_result_to_table(this, fixture, t, ((this.club_count() as u16) as i32) as u32, 0, (0) as usize, (0) as usize);
    exe::league_sort_table(this, t);
    let n = this.club_count();
    // the original reads +4 from address 0 if the club is not in the copy (never happens in the game)
    let found = (0..n).find(|&k| read_usize(row(t.addr(), k)) == club).map_or(0, |k| row(t.addr(), k));
    if read_u8(found + 4) != 0 {
        exe::league_free_table(this, t);
        return 0;
    }
    let rounds = this.games_per_club();
    let points = (this.win_points() as i8) as i16;
    for k in 1..this.club_count() {
        let o = LeagueTableRow(row(t.addr(), k));
        let games = o.played() as i16;
        let reach = rounds.wrapping_sub(games).wrapping_mul(points).wrapping_add(o.points());
        if reach >= t.points() && (t.played() as i16 != rounds || games != rounds) {
            exe::league_free_table(this, t);
            return 0;
        }
    }
    exe::league_free_table(this, t);
    1
}

/// `exe::cm3_match_engine_post_period_end_events` (ret 4, match_eng.cpp) with the commentary injected: commentates the side's table/tie situation.
///
/// Group decided (`exe::league_would_result_clinch_title`) or knockout already resolved (`exe::cm3_match_engine_get_goal_margin` in round 0x82 or 0x96): 0x2171/0x2170. With +0x8ebb:
/// 0x2120 if the side is ahead on aggregate, otherwise 0x2121 when the average of +0x1b of the starters is low and the
/// other club's strength (+0x80) weighs.
///
/// At the end, for side 1 without +0x1a8: 0x2001 if the match is still not decided.
pub(crate) unsafe fn comment_standing_with(this: Engine, side: u32, call: AddEventCall) {
    let t = || this.r#match();
    let count_t = || exe::comp_util_is_league_division(read_i32(t()));
    let object = || read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add((read_i32(t()) as usize).wrapping_mul(4)));
    let commentator = || this.commentator();
    let say = |c: &mut dyn FnMut(usize, [u32; 7]), code: u32, l: u32| c(commentator(), [code, this.addr() as u32, u32::MAX, l, u32::MAX, u32::MAX, 0]);
    // the aggregate score with the 5th output (fase3): in the original it lands on the argument itself
    let resolved = |l: i8| -> bool {
        let (mut a, mut b, mut c, mut d, mut e) = (0i8, 0i8, 0u8, 0u8, 0u8);
        exe::cm3_match_engine_get_goal_margin(this, l, &mut a, &mut b, &mut c, &mut d, &mut e, 0) > 0 && e == 0
    };
    let bl = side as u8 as i8;
    if read_i16(t() + fixture::ROUND_PREFIX) == 0 && count_t() == 0 && read_i16(t() + fixture::ROUND_NAME) == 0x82 && resolved(bl) {
        say(call, event::THROUGH_TO_FINAL, side);
        return;
    }
    if count_t() != 0 {
        let club = read_usize(t() + if bl != 0 { 0x20 } else { 0x1c });
        if exe::league_would_result_clinch_title(object(), club, t()) != 0 {
            say(call, event::WON_COMPETITION, side);
            return;
        }
    } else if read_i16(t() + fixture::ROUND_NAME) == 0x96 && resolved(bl) {
        say(call, event::WON_COMPETITION, side);
        return;
    }
    if this.live_match() != 0 {
        let (mut casa, mut fora, mut classificado, mut x, mut y) = (0i8, 0i8, 0u8, 0u8, 0u8);
        exe::cm3_match_engine_get_goal_margin(this, bl, &mut casa, &mut fora, &mut classificado, &mut x, &mut y, 0);
        if casa > fora || (casa == fora && classificado != 0) {
            say(call, event::SILENT_WON_MATCH, side);
        } else {
            let mut sum = 0.0f64;
            let mut n = 0i8;
            for k in 0..20usize {
                let j = this.player(bl, k);
                if k <= 10 || (j.minute_on() as u8) != 0xff {
                    sum += j.match_grade() as f64;
                    n = n.wrapping_add(1);
                }
            }
            // with no starter the average would be whatever was in the argument (does not happen)
            let average = if n != 0 { (sum / n as f64) as f32 as f64 } else { 0.0 };
            let reputation = |o: usize| read_i16(read_usize(t() + o) + club::REPUTATION) as f64;
            let x = average * read_f64(rdata::F64_0_5) - read_f64(rdata::F64_2);
            if bl == 0 {
                if average < read_f64(rdata::F64_7) && x * reputation(fixture::AWAY_CLUB) < reputation(fixture::HOME_CLUB) {
                    say(call, event::BOO_OFF_PITCH, 0);
                }
                return;
            }
            if average < read_f64(rdata::F64_6_67) && x * reputation(fixture::HOME_CLUB) < reputation(fixture::AWAY_CLUB) * read_f64(rdata::F64_0_8) {
                say(call, event::BOO_OFF_PITCH, side);
            }
        }
    }
    if bl != 1 || this.golden_goal_scored() != 0 {
        return;
    }
    if read_i16(t() + fixture::ROUND_PREFIX) == 0 && count_t() == 0 && read_i16(t() + fixture::ROUND_NAME) == 0x82 && resolved(0) {
        return;
    }
    if count_t() != 0 {
        if exe::league_would_result_clinch_title(object(), read_usize(t() + fixture::HOME_CLUB), t()) != 0 {
            return;
        }
    } else if read_i16(t() + fixture::ROUND_NAME) == 0x96 && resolved(0) {
        return;
    }
    say(call, event::FINAL_WHISTLE, 0);
}

/// Adds the match commentary about side `lado`'s qualification situation (group decided, tie already resolved, ahead on
/// aggregate, game still open); thin wrapper over comment_standing_with with the real commentary call.
pub(crate) unsafe extern "thiscall" fn narrate_qualification(this: usize, side: u32) {
    comment_standing_with(Engine(this), side, &mut |t, a| add_match_event(t, a))
}

/// The dates of the 6 rounds of the group stages, by half of the stage.
///
/// Each via `0x68a160` (round, day, month 0..11, years to add, weekday; code 2, value 0): [0] first stage, groups 0..3;
/// [1] first stage, 4..7; [2] second stage, 8..9; [3] second stage, 10..11.
pub(crate) const GROUP_STAGE_DATES: [[(i32, i32, i32, i32, i32); 6]; 4] = [
    [(0, 0xc, 8, 0, 1), (1, 0x14, 8, 0, 2), (2, 0x1b, 8, 0, 2), (3, 0x11, 9, 0, 1), (4, 0x19, 9, 0, 2), (5, 7, 0xa, 0, 1)], // cqh4: the first stage, groups A to D
    [(0, 0xd, 8, 0, 2), (1, 0x13, 8, 0, 1), (2, 0x1a, 8, 0, 1), (3, 0x12, 9, 0, 2), (4, 0x18, 9, 0, 1), (5, 8, 0xa, 0, 2)], // cqh4: the first stage, groups E to H
    [(0, 0x15, 0xa, 0, 1), (1, 6, 0xb, 0, 2), (2, 0xe, 1, 1, 2), (3, 0x14, 1, 1, 1), (4, 7, 2, 1, 2), (5, 0xd, 2, 1, 1)],
    [(0, 0x16, 0xa, 0, 2), (1, 5, 0xb, 0, 1), (2, 0xd, 1, 1, 1), (3, 0x15, 1, 1, 2), (4, 6, 2, 1, 1), (5, 0xe, 2, 1, 2)], // cqh4: a segunda fase, grupos C e D
];

/// Collects the bytes the fixture table actually writes: per round +0..+0xb and +0x3d..+0x40, for 6 rounds of 0x41
/// bytes.
pub(crate) unsafe fn league_written_table_bytes(t: usize) -> Vec<u8> {
    let mut v = Vec::new();
    for r in 0..6usize {
        let e = t + r * 0x41;
        v.extend_from_slice(core::slice::from_raw_parts(e as *const u8, 0xc));
        v.extend_from_slice(core::slice::from_raw_parts((e + league_round_date::VENUE_VALUE) as *const u8, 4));
    }
    v
}

/// Sets the season expectations of each club in the table, through the manager manager, with the news.
///
/// Ret 0, a league method: `League::set_club_season_expectations()`: with the manager manager (`G_MANAGER_MANAGER`),
/// the season expectations of each club in the table (`table()`, rows of `league_table_row::SIZE` bytes, `club_count()`
/// clubs) through `exe::manager_manager_set_season_expectations` (push 1; push club; mov ecx, `G_MANAGER_MANAGER` ->
/// thiscall, ret 8: with the news).
/// Original: `League::set_club_season_expectations` (0x68aa80, league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn set_club_season_expectations(this: usize) {
    let this = League(this);
    if read_usize(G_MANAGER_MANAGER) == 0 {
        return;
    }
    let mut i = 0i16;
    while i < this.club_count() {
        covered!("cqh_4 club expectation");
        let club = read_usize(this.table() + i as usize * league_table_row::SIZE);
        exe::manager_manager_set_season_expectations(read_usize(G_MANAGER_MANAGER), club, 1); // cqh4: with the news
        i += 1; // cqh4: the next club
    }
}

/// Moves the club down to the division (keeping the previous one), applies the relegation clauses and makes the human
/// manager leave when the division is not played.
///
/// Ret 0xc; this is not read: `club.relegate_club_to_division(division, _)`: the club's competition (`club::DIVISION`)
/// becomes the division and the previous one goes to `club::LAST_DIVISION`; the relegation clauses
/// (`exe::contract_manager_trigger_relegation_clauses` push old; push club; mov ecx, `G_CONTRACT_MANAGER` -> thiscall,
/// ret 8); if the division is not played (no competition table `G_COMP_OBJECT_TABLE`, no division or no competition
/// with its id) and the club has a human manager who decides (`exe::human_manager_is_human_club` push 0; push 0; push
/// 1; push 1; push club; add esp, 0x14 -> cdecl), the manager leaves (`exe::manager_manager_remove_club_manager` push
/// 0; push 7; push 1; push club; mov ecx, `G_MANAGER_MANAGER` -> thiscall, ret 0x10).
/// Original: `League::relegate_club_to_division` (0x6831a0, league.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn relegate_club_to_division(_this: usize, club: usize, division: usize, _x: u32) {
    let club = Club(club);
    let old = club.division().addr();
    write_u32(club.addr() + club::DIVISION, division as u32);
    write_u32(club.addr() + club::LAST_DIVISION, old as u32);
    exe::contract_manager_trigger_relegation_clauses(read_usize(G_CONTRACT_MANAGER), club, old);
    let comps = read_usize(G_COMP_OBJECT_TABLE);
    if comps != 0 && division != 0 && read_usize(comps + read_usize(division) * 4) != 0 {
        return;
    }
    if exe::human_manager_is_human_club(club, 1, 1, 0, 0) != 0 {
        covered!("cqh_4 manager leaves");
        exe::manager_manager_remove_club_manager(read_usize(G_MANAGER_MANAGER), club, 1, 7, 0);
    }
}

/// The league TV date table entry (0x41 bytes at [+0xba], count +0xa9) for a date, found by bsearch as in `move_fixtures_for_tv`.
///
/// None when there are no entries, when the date is after the last one (year +2 + the league's +0x40, unsigned; day +0
/// signed), when the search does not find the entry (`exe::crt_bsearch`: push `league_compare_dates`; push 0x41; push count; push table; push
/// &key; add esp, 0x14 -> cdecl bsearch; the key is the day and the year minus the league's) or when the first slot is
/// empty (+0xb = 0xff).
pub(crate) unsafe fn tv_date_entry(this: League, data: usize) -> Option<usize> {
    let n = this.round_count();
    if n == 0 {
        return None;
    }
    let table = this.round_dates();
    let base = this.year();
    let ult = (table as isize + n as isize * 0x41 - 0x41) as usize;
    let ult_year = read_i16(ult + 2) as u16 as i32 + base as i32;
    let year = read_i16(data + 2) as u16 as i32;
    if year > ult_year || (year == ult_year && read_i16(data) > read_i16(ult)) {
        return None;
    }
    let key = [read_i16(data), read_i16(data + 2).wrapping_sub(base)];
    let e = exe::crt_bsearch((key.as_ptr()) as usize, table, (n as i32) as u32, 0x41, 0x685cf0);
    if e == 0 || read_u8(e + league_round_date::TV_SLOT_KIND) == 0xff {
        return None;
    }
    Some(e)
}

/// Moves fixtures for TV: on the TV table dates, each slot takes the highest-rated fixture (a random draw by
/// reputation, zeroed when either club plays 2..5 days before or after; 100/50 in national team slots) to the weekday
/// and time slot of the slot, with the news, the stadium and the calendar.
///
/// Ret 4; SEH frame with no destructor on the normal path (the local dates and list): `date.move_fixtures_for_tv()`,
/// from the league day (`league_update_for_date`) and from building the calendar (0x683e30). Through the date's entry
/// (tv_date_entry), the league matches on the entry's day and time slot (+4) go to a local list (`exe::new_llist`;
/// `exe::fixture_manager_collect_comp_fixtures_on_day`: push list; push league `stage_index()`; push -1; push id; push
/// slot; push day; push year; mov ecx, `G_FIXTURE_MANAGER` -> thiscall, ret 0x1c). With matches: the entry's slots (7
/// bytes from +5, up to the first with `league_round_date::TV_SLOT_KIND` = 0xff) are sorted (`exe::crt_qsort` with
/// 0x8b4e00) if any has bits 0x7fc0 in dword +5. For each slot with time slot (`league_round_date::TV_SLOT_TIME_SLICE`)
/// 0..2: the date moved to the slot's weekday (`league_round_date::TV_SLOT_WEEKDAY`; `exe::date_get_nearest_weekday`);
/// the clash window: 2..5 days after the requested date if the TV day falls after it, otherwise 5..2 days before
/// (`exe::date_add_days`/`exe::date_sub_days` + `exe::date_operator_3d`). Among the matches in the list (of the
/// league's type, `fixture::STAGE_INDEX`), the one with the highest rating (signed word; the first on a tie): slot of
/// type 3, 100 for a national side match (`exe::database_is_national_team` on the home side) in the stadium
/// (`fixture::STADIUM` → `city()` → `city::NATION`) of the nation of either side, otherwise 50; otherwise the random
/// draw (`exe::utils_random`) of the away side's reputation (`club::REPUTATION`) + 1500 plus that of the home side's,
/// zeroed if either has a match in the window (`FN_FIXTURE_MANAGER_FIND_FIXTURES`). The chosen one: outside type 3,
/// news 3 to both clubs; it leaves the calendar (`exe::fixture_manager_remove_fixture`), takes the TV date, the time
/// slot and the slot's type, the stadium (`FN_STADIUM_SET_FIXTURE_VENUE`) and goes back into the calendar
/// (`exe::fixture_manager_add_fixture`); the slot becomes 0xff and the match leaves the list (`exe::llist_remove`).
/// Original: `League::move_fixtures_for_tv` (0x685640, league.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn move_fixtures_for_tv(this: usize, data: usize) {
    let this = League(this);
    let (mut a, mut b, mut c) = ([0u8; 8], [0u8; 8], [0u8; 8]);
    // the dates from the dwords the original reads from the stack (day|year, year|leap)
    let (pa, pb, pc) = (a.as_mut_ptr() as usize, b.as_mut_ptr() as usize, c.as_mut_ptr() as usize);
    exe::date_date_54a110(pa);
    exe::date_date_54a110(pb);
    exe::date_date_54a110(pc);
    let e = match tv_date_entry(this, data) {
        Some(e) => e,
        None => return,
    };
    let mut list = [0u8; 0x10];
    let l = Llist(list.as_mut_ptr() as usize);
    exe::new_llist(l);
    // simplification: the original pushes the year, the day, the slice and the type in registers with leftover in the high bytes; the
    // 0x5abd90 only reads the words and the bytes
    let n = (exe::fixture_manager_collect_comp_fixtures_on_day(read_usize(G_FIXTURE_MANAGER), read_i16(data + 2) as u16 as u32, read_i16(data) as u16 as u32, read_u8(e + 4) as u32,
        read_usize(this.competition().addr()) as u32, u32::MAX, this.stage_index() as u32, l) as u32) & 0xffff;
    if n != 0 {
        let mut k = 0usize;
        let mut sort = false;
        while k < 8 {
            if read_u8(e + k * 7 + league_round_date::TV_SLOT_KIND) == 0xff {
                break;
            }
            if read_usize(e + k * 7 + 5) & 0x7fc0 != 0 {
                sort = true;
            }
            k += 1;
        }
        if sort {
            exe::crt_qsort(e + 5, k as u32, 7, 0x8b4e00);
        }
        let find: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, usize, u32, u32, u32) -> u32 =
            core::mem::transmute(FN_FIXTURE_MANAGER_FIND_FIXTURES);
        for k in 0..8usize {
            let v = e + k * 7;
            if read_u8(v + league_round_date::TV_SLOT_KIND) == 0xff {
                break;
            }
            let slice = read_u8(v + league_round_date::TV_SLOT_TIME_SLICE) as i8;
            if !(0..=2).contains(&slice) {
                continue;
            }
            let mut x = [0u8; 8];
            let px = x.as_mut_ptr() as usize;
            exe::new_date(px, read_i16(data) as u16 as u32, read_i16(data + 2) as u16 as u32);
            exe::date_get_nearest_weekday(px, read_u8(v + league_round_date::TV_SLOT_WEEKDAY) as i8 as i16 as u16 as u32, pa);
            let (xa, aa) = (read_i16(px + 2) as u16, read_i16(pa + 2) as u16);
            let (mut t1, mut t2, mut t3, mut t4) = ([0u8; 8], [0u8; 8], [0u8; 8], [0u8; 8]);
            if xa < aa || (xa == aa && read_i16(px) < read_i16(pa)) {
                let r = exe::date_add_days(px, t1.as_mut_ptr() as usize, 2);
                exe::date_operator_3d(pb, t2.as_mut_ptr() as usize, r);
                let r = exe::date_add_days(px, t3.as_mut_ptr() as usize, 5);
                exe::date_operator_3d(pc, t4.as_mut_ptr() as usize, r);
            } else {
                let r = exe::date_sub_days(px, t1.as_mut_ptr() as usize, 5);
                exe::date_operator_3d(pb, t2.as_mut_ptr() as usize, r);
                let r = exe::date_sub_days(px, t3.as_mut_ptr() as usize, 2);
                exe::date_operator_3d(pc, t4.as_mut_ptr() as usize, r);
            }
            l.set_cursor(l.head());
            let mut best = 0usize;
            let mut rating: i16 = -1;
            let mut f = exe::llist_get_next(l);
            while f != 0 {
                if this.stage_index() == read_u8(f + fixture::STAGE_INDEX) {
                    let mut s: i32;
                    if read_u8(v + league_round_date::TV_SLOT_KIND) == 3 {
                        let est = Stadium(read_usize(f + fixture::STADIUM));
                        s = 50;
                        if exe::database_is_national_team(read_usize(f + fixture::HOME_CLUB)) != 0 && !est.is_null() && est.city() != 0 {
                            let g = read_usize(est.city() + city::NATION);
                            if read_usize(read_usize(f + fixture::HOME_CLUB) + club::NATION) == g || read_usize(read_usize(f + fixture::AWAY_CLUB) + club::NATION) == g {
                                s = 100;
                            }
                        }
                    } else {
                        s = exe::utils_random(read_i16(read_usize(f + fixture::AWAY_CLUB) + club::REPUTATION) as i32 + 0x5dc); // cqc6: the away side with the bonus
                        s = s.wrapping_add(exe::utils_random(read_i16(read_usize(f + fixture::HOME_CLUB) + club::REPUTATION) as i32));
                    }
                    if read_u8(v + league_round_date::TV_SLOT_KIND) != 3 {
                        let window = |club: usize| {
                            find(read_usize(G_FIXTURE_MANAGER), read_usize(pb) as u32, read_usize(pb + 2) as u32, read_usize(pc) as u32, read_usize(pc + 2) as u32,
                                read_usize(club) as u32, 0, u32::MAX, 0xffff_fffe, u32::MAX) as u16 != 0
                        };
                        if window(read_usize(f + fixture::HOME_CLUB)) || window(read_usize(f + fixture::AWAY_CLUB)) {
                            covered!("cqc_6 conflict in the window");
                            s = 0; // cqc6: conflito na janela
                        }
                    }
                    if s as i16 > rating {
                        best = f;
                        rating = s as i16;
                    }
                }
                f = exe::llist_get_next(l);
            }
            if best == 0 {
                continue;
            }
            let mut buf = [0u8; 0x50];
            let r = buf.as_mut_ptr() as usize;
            core::ptr::copy_nonoverlapping(best as *const u8, r as *mut u8, 0x4f);
            if read_u8(v + league_round_date::TV_SLOT_KIND) != 3 {
                let mut news = [0u32; 0x38];
                let n = news.as_mut_ptr() as usize;
                exe::new_newsitem(n, 3, 0);
                exe::newsitem_set_slot(n, 0, read_usize(r + fixture::HOME_CLUB_ID) as u32);
                exe::newsitem_set_slot(n, 1, read_usize(r + fixture::AWAY_CLUB_ID) as u32);
                exe::newsitem_set_slot(n, 2, read_usize(r) as u32);
                exe::newsitem_set_slot(n, 3, read_i16(r + fixture::ROUND_PREFIX) as i32 as u32);
                exe::newsitem_set_slot(n, 4, read_i16(r + fixture::ROUND_NAME) as i32 as u32);
                exe::newsitem_set_slot(n, 5, read_i16(r + fixture::DAY) as u16 as u32);
                exe::newsitem_set_slot(n, 6, read_i16(r + fixture::YEAR) as u16 as u32);
                exe::newsitem_set_slot(n, 7, read_u8(r + fixture::TIME_SLICE) as u32);
                exe::newsitem_set_slot(n, 8, read_i16(pa) as i32 as u32);
                exe::newsitem_set_slot(n, 9, read_i16(pa + 2) as u16 as u32);
                exe::newsitem_set_slot(n, 10, read_u8(v + league_round_date::TV_SLOT_TIME_SLICE) as i8 as i32 as u32);
                exe::newsitem_set_slot(n, 11, read_u8(v + league_round_date::TV_SLOT_KIND) as i8 as i32 as u32);
                exe::news_send_to_club_manager(read_usize(G_NEWS), n, read_usize(r + fixture::HOME_CLUB));
                exe::news_send_to_club_manager(read_usize(G_NEWS), n, read_usize(r + fixture::AWAY_CLUB));
            }
            exe::fixture_manager_remove_fixture(read_usize(G_FIXTURE_MANAGER), r);
            covered!("cqc_6 match for TV");
            write_u16(r + fixture::YEAR, read_i16(pa + 2) as u16);
            write_u16(r + fixture::DAY, read_i16(pa) as u16); // cqc6: the TV day
            write_u8(r + fixture::TIME_SLICE, read_u8(v + league_round_date::TV_SLOT_TIME_SLICE));
            write_u8(r + fixture::STAGE, read_u8(v + league_round_date::TV_SLOT_KIND));
            let stadium: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32 = core::mem::transmute(FN_STADIUM_SET_FIXTURE_VENUE);
            stadium(read_usize(G_STADIUM), read_usize(v + league_round_date::TV_SLOT_DATA), r, this.fixed_stadium_id());
            exe::fixture_manager_add_fixture(read_usize(G_FIXTURE_MANAGER), r, 1);
            write_u8(v + league_round_date::TV_SLOT_KIND, 0xff);
            exe::llist_remove(l, best);
        }
    }
    exe::drop_llist(l);
}

/// Sends the group draw news 0x15 (the competition, the first date, the opponents) to each human-managed club in the
/// group.
///
/// Ret 0; SEH frame with no destructor on the normal path: `League::send_group_draw_news()`, from the competitions'
/// group draw. A group of up to 8 teams (`club_count()`; the `league_table_row::SIZE`-byte teams in `table()`, +0 the
/// club): for each club with a human manager (`cup_has_human_manager`), news 0x15 (allocated once,
/// `exe::crt_operator_new`(0xdf), built by `exe::new_newsitem`, freed at the end by `exe::crt_operator_delete` without
/// a destructor) with the competition id, `stage_id()`, the first date (that of the first entry of table
/// `round_dates()` moved to the weekday of each slot, `exe::date_get_nearest_weekday`, the earliest), the number of
/// opponents; per club, its own id (2) and those of the others (6 onwards), delivered to it
/// (`exe::news_send_to_club_manager`).
/// Original: `League::send_group_draw_news` (0x684230, league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn send_group_draw_news(this: usize) {
    let this = League(this);
    let n = this.club_count();
    if n > 8 || n <= 0 {
        return;
    }
    let team = |i: u8| read_usize(this.table() + (i as i8 as isize * (league_table_row::SIZE as isize)) as usize);
    let mut news = 0usize;
    let mut i: u8 = 0;
    loop {
        if cup_has_human_manager(team(i)) {
            if news == 0 {
                let p = exe::crt_operator_new(0xdf);
                news = if p != 0 { exe::new_newsitem(p, 0x15, 0) } else { 0 };
                let (mut d1, mut d2, mut d3, mut t) = ([0u8; 8], [0u8; 8], [0u8; 8], [0u8; 8]);
                let (p1, p2, p3) = (d1.as_mut_ptr() as usize, d2.as_mut_ptr() as usize, d3.as_mut_ptr() as usize);
                let table = this.round_dates();
                // simplification: the day and the year go in registers with leftovers in the high 16 bits; 0x549ef0 only reads the words
                exe::new_date(p1, read_i16(table) as u16 as u32, read_i16(table + 2).wrapping_add(this.year()) as u16 as u32);
                exe::date_date_54a110(p2);
                exe::date_date_54a110(p3);
                exe::date_operator_3d(p2, t.as_mut_ptr() as usize, p1);
                for j in 0..8usize {
                    let v = this.round_dates() + j * 7;
                    if read_u8(v + league_round_date::TV_SLOT_KIND) == 0xff {
                        break;
                    }
                    exe::date_get_nearest_weekday(p1, read_u8(v + league_round_date::TV_SLOT_WEEKDAY) as i8 as i16 as u16 as u32, p3);
                    let (y2, y3) = (read_i16(p2 + 2) as u16, read_i16(p3 + 2) as u16);
                    if y2 > y3 || (y2 == y3 && read_i16(p2) > read_i16(p3)) {
                        let mut t2 = [0u8; 8];
                        exe::date_operator_3d(p2, t2.as_mut_ptr() as usize, p3);
                    }
                }
                exe::newsitem_set_slot(news, 0, read_usize(this.competition().addr()) as u32);
                exe::newsitem_set_slot(news, 1, this.stage_id() as i32 as u32);
                exe::newsitem_set_slot(news, 3, read_i16(p2) as i32 as u32);
                exe::newsitem_set_slot(news, 4, read_i16(p2 + 2) as u16 as u32);
                exe::newsitem_set_slot(news, 5, (this.club_count() as i32 - 1) as u32);
            }
            exe::newsitem_set_slot(news, 2, read_usize(team(i)) as u32);
            let mut k: u8 = 6;
            let mut j: u8 = 0;
            while (j as i8 as i16) < this.club_count() {
                if j != i {
                    exe::newsitem_set_slot(news, k as i8, read_usize(team(j)) as u32);
                    k = k.wrapping_add(1);
                }
                j = j.wrapping_add(1);
            }
            covered!("cqc_6 group news");
            exe::news_send_to_club_manager(read_usize(G_NEWS), news, team(i));
        }
        i = i.wrapping_add(1);
        if (i as i8 as i16) >= this.club_count() {
            break;
        }
    }
    if news != 0 {
        exe::crt_operator_delete(news);
    }
}

/// Sends news 0xd with the first three fixtures (date, opponent, venue) to each human-managed club not yet told, and
/// does the same in the following stages.
///
/// Ret 4; SEH frame with no destructor on the normal path (the local lists): `list.send_fixture_list_news()`, from
/// creating the calendar (0x5cd1e0) and from the league day in its year. Uses the local list (`exe::new_llist`) if the
/// league is a first stage (`stage_index()` = `stage_index::MAIN`) or there is no list. With the competition counting
/// (`exe::comp_util_is_league_division`: push id; add esp, 4 -> cdecl) and a first stage: the date of the first entry
/// of table `round_dates()` minus 5 days (`exe::date_sub_days`, `exe::date_operator_3d`); for each club with a human
/// manager (`cup_has_human_manager`; `league_table_row::SIZE`-byte teams in `table()`) that is not in the list (4-byte
/// ids): its first 3 matches from that date (`FN_FIXTURE_MANAGER_SCAN_FIXTURES` into a local list, 18 arguments: push
/// 1; push -1; push `stage_index()`; push 0; push list; push 3; push -1 ×4; push club id; push -1; push -1; push
/// competition id; push 1; push &slot; push &day; push &year; mov ecx, `G_FIXTURE_MANAGER` -> thiscall, ret 0x48);
/// finding 3: news 0xd (competition id, club id, the league year and, per match, the day, the year, the opponent and 1
/// at home or 0 away) to the club, which enters the list (`exe::llist_add_tail`: push 4; push club; mov ecx, list ->
/// thiscall, ret 8). Then, in the stages (`stages()`, up to `current_stage()`) with `league::COMP_TYPE` = 2, the same
/// routine with the list (`exe::league_send_fixture_list_news`).
/// Original: `League::send_fixture_list_news` (0x683ee0, league.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn send_fixture_list_news(this: usize, list: usize) {
    let this = League(this);
    let mut l0 = [0u8; 0x10];
    let p0 = l0.as_mut_ptr() as usize;
    exe::new_llist(p0);
    let list = if this.stage_index() == stage_index::MAIN || list == 0 { p0 } else { list };
    if exe::comp_util_is_league_division((read_usize(this.competition().addr()) as u32) as i32) == 0 || this.stage_index() != stage_index::MAIN {
        exe::drop_llist(p0);
        return;
    }
    let (mut d1, mut t, mut t2) = ([0u8; 8], [0u8; 8], [0u8; 8]);
    let p1 = d1.as_mut_ptr() as usize;
    let table = this.round_dates();
    // simplification: the day and the year go in registers with leftovers in the high 16 bits; 0x549ef0 only reads the words
    exe::new_date(p1, read_i16(table) as u16 as u32, read_i16(table + 2).wrapping_add(this.year()) as u16 as u32);
    let r = exe::date_sub_days(p1, t.as_mut_ptr() as usize, 5);
    exe::date_operator_3d(p1, t2.as_mut_ptr() as usize, r);
    let team = |i: u32| read_usize(this.table() + (i as i16 as isize * (league_table_row::SIZE as isize)) as usize);
    let mut i: u32 = 0;
    if this.club_count() > 0 {
        loop {
            if cup_has_human_manager(team(i)) {
                write_usize(list + llist::CURSOR, read_usize(list + llist::HEAD));
                let mut ja = false;
                let mut it = exe::llist_get_next(list);
                while it != 0 {
                    if read_usize(it) == read_usize(team(i)) {
                        ja = true;
                        break;
                    }
                    it = exe::llist_get_next(list);
                }
                if !ja {
                    let mut v_year = read_usize(p1 + 2) as u32;
                    let mut v_day = read_usize(p1) as u32;
                    let mut slice: u8 = 0;
                    let mut l2 = [0u8; 0x10];
                    let p2 = Llist(l2.as_mut_ptr() as usize);
                    exe::new_llist(p2);
                    let search: unsafe extern "thiscall" fn(usize, *mut u32, *mut u32, *mut u8, u32, u32, i32, i32, u32, i32, i32, i32,
                                                            i32, u32, usize, u32, u32, i32, u32) -> u32 =
                        core::mem::transmute(FN_FIXTURE_MANAGER_SCAN_FIXTURES);
                    let found = search(read_usize(G_FIXTURE_MANAGER), &mut v_year, &mut v_day, &mut slice, 1, read_usize(this.competition().addr()) as u32, -1, -1,
                        read_usize(team(i)) as u32, -1, -1, -1, -1, 3, p2.addr(), 0, this.stage_index() as u32, -1, 1) as u16;
                    if found == 3 {
                        covered!("cqc_6 fixture list news");
                        let mut news = [0u32; 0x38];
                        let n = news.as_mut_ptr() as usize;
                        exe::new_newsitem(n, 0xd, 0);
                        let mut k: u8 = 3;
                        exe::newsitem_set_slot(n, 0, read_usize(this.competition().addr()) as u32);
                        exe::newsitem_set_slot(n, 1, read_usize(team(i)) as u32);
                        exe::newsitem_set_slot(n, 2, this.year() as i32 as u32);
                        p2.set_cursor(p2.head());
                        let mut f = exe::llist_get_next(p2);
                        while f != 0 {
                            exe::newsitem_set_slot(n, k as i8, read_i16(f + fixture::DAY) as u16 as u32);
                            k = k.wrapping_add(1);
                            exe::newsitem_set_slot(n, k as i8, read_i16(f + fixture::YEAR) as u16 as u32);
                            k = k.wrapping_add(1);
                            if team(i) == read_usize(f + fixture::HOME_CLUB) {
                                exe::newsitem_set_slot(n, k as i8, read_usize(read_usize(f + fixture::AWAY_CLUB)) as u32);
                                k = k.wrapping_add(1);
                                exe::newsitem_set_slot(n, k as i8, 1);
                            } else {
                                exe::newsitem_set_slot(n, k as i8, read_usize(read_usize(f + fixture::HOME_CLUB)) as u32);
                                k = k.wrapping_add(1);
                                exe::newsitem_set_slot(n, k as i8, 0);
                            }
                            k = k.wrapping_add(1);
                            f = exe::llist_get_next(p2);
                        }
                        exe::news_send_to_club_manager(read_usize(G_NEWS), n, team(i));
                        exe::llist_add_tail(list, team(i), 4);
                    }
                    exe::drop_llist(p2);
                }
            }
            i = i.wrapping_add(1);
            if (i as i16) >= this.club_count() {
                break;
            }
        }
    }
    if this.stage_index() == stage_index::MAIN && (this.current_stage() as i32) >= 0 {
        let mut j: u8 = 0;
        loop {
            let stage = League(read_usize(this.stages() + (j as i8 as isize * 4) as usize));
            if stage.comp_type() == 2 {
                exe::league_send_fixture_list_news(stage, list);
            }
            j = j.wrapping_add(1);
            if j as i8 as i32 > this.current_stage() as i32 {
                break;
            }
        }
    }
    exe::drop_llist(p0);
}

/// Returns the method pointer at offset k of object o's vtable.
pub(crate) unsafe fn vtable_method(o: League, k: usize) -> usize {
    read_usize(read_usize(o.addr()) + k)
}

/// True if two dates have the same day and year words.
pub(crate) unsafe fn same_date(a: usize, b: usize) -> bool {
    read_i16(a + 2) == read_i16(b + 2) && read_i16(a) == read_i16(b)
}

/// The league day: the fixture list news on its day, each stage's day, the `league::LAST_RESULT_DATE` date event, the
/// season end (next stage, history, records), the TV fixtures four weeks ahead, the registration deadline 5/20/14 days
/// before the first date and the `league::LAST_UPDATE_DATE` date.
///
/// Ret 8; SEH frame with no destructor on the normal path (the local dates): `date.update_for_date(full)`, the day of
/// each league (through the competitions, 46522 calls up to 2002-06-01). League stopped (`stopped()`) or without clubs
/// (`active()` = 0): 0. Otherwise (returns 1, in the byte):
///
/// - `games_per_club()` = -1 with teams (`club_count()`) > 0: `games_per_club()` = (teams − 1) · legs (`legs()`), in
/// the word;
///
/// - in the first stage (`stage_index()` = `stage_index::MAIN`) with a table (`table()`, `played()` = 0), full and a
/// new date (≠ `league::LAST_UPDATE_DATE`): on the day after the key nation's Cup day (`exe::key_nation_index_of_comp`;
/// `[0xb63d75 + 72·k]` + 1, 0 after 364) and in the league year (`year()`), the table news
/// (`exe::league_send_fixture_list_news`: push 0; mov ecx, this -> thiscall, ret 4);
///
/// - in the first stage, each stage's day (`stages()`, up to `current_stage()`; method `comp_vtable::UPDATE_FOR_DATE`
/// with the date and the full flag);
///
/// - on date `league::LAST_RESULT_DATE` (without `complete()`): `set_table_column(0)` and method
/// `league_vtable::UPDATE_POSITIONS_AND_STATUS`;
///
/// - in the first stage with a table and a new date: method `comp_vtable::CHECK_COMPLETE` (0, 0); season end
/// (`complete()` = 1) with all stages finished (method `comp_vtable::CHECK_COMPLETE` with (0, 1) ≠ 0): the next stage
/// (method `comp_vtable::ADVANCE_STAGE`) if `current_stage()` < `stage_count()` − 1, otherwise (once,
/// `season_closed()`) the history (`exe::club_history_record_comp_season`: push league; mov ecx, `G_CLUB_HISTORY` ->
/// thiscall, ret 4), the records with 1 (`exe::club_records_update_league_position_records`: push 1; push league; mov
/// ecx, `G_CLUB_RECORDS` -> thiscall, ret 8) if the competition counts (`exe::comp_util_is_league_division`: push id;
/// add esp, 4 -> cdecl), the box at line 0x74c if `tiebreak_matches()`, method `comp_vtable::RECORD_SEASON_HONOURS` and
/// the end of attachment `comp_stats()` (`exe::comp_stats_update_season_end_records`: mov ecx, `[+8]` -> thiscall, ret
/// 0); then the records with 0 and the attachment (`FN_COMP_STATS_SEND_RECORD_NEWS`: mov ecx, `[+8]` -> thiscall, ret
/// 0);
///
/// - with a table and full, without `complete()`: the TV fixtures four weeks ahead (`exe::date_add_weeks` +
/// `exe::league_move_fixtures_for_tv`: push &date; mov ecx, this -> thiscall, ret 4);
///
/// - full: in the first stage with registrations (`squad_flags()` & `squad_flags::FIXED_SQUAD_NUMBERS`), a new date,
/// without `complete()` and the table in progress: if the first date of the TV table (`round_dates()`; day, year − the
/// league's) falls 5 days from now (Argentina `KEY_NATION_ARGENTINA`), 20 (Italy `KEY_NATION_ITALY`) or 14 days, the
/// registration deadline (`exe::squad_manager_assign_league_squad_numbers`: push 0; push 0; push league; mov ecx,
/// `G_SQUAD_MANAGER` -> thiscall, ret 0xc); and `league::LAST_UPDATE_DATE` takes the date (`exe::date_operator_3d`).
/// Original: `League::update_for_date` (0x6847c0, league.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn league_update_for_date(this: usize, data: usize, full: u32) -> u32 {
    let this = League(this);
    let anexo: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_COMP_STATS_SEND_RECORD_NEWS);
    // the requested date copied at the start (the original keeps the day|year dword and the leap-year one)
    let d0 = [read_usize(data) as u32, read_usize(data + 4) as u32];
    let pd0 = d0.as_ptr() as usize;
    let (mut t, mut x, mut w) = ([0u8; 8], [0u8; 8], [0u8; 8]);
    let (pt, px, pw) = (t.as_mut_ptr() as usize, x.as_mut_ptr() as usize, w.as_mut_ptr() as usize);
    if this.stopped() != 0 || this.active() == 0 {
        return 0;
    }
    if this.games_per_club() == -1 {
        let n = this.club_count();
        if n > 0 {
            this.set_games_per_club((n.wrapping_sub(1).wrapping_mul(this.legs()) as u16) as i16);
        }
    }
    let table = LeagueTableRow(this.table());
    if !table.is_null() && (table.played() as u8) == 0 && this.stage_index() == stage_index::MAIN && full != 0 && !same_date(this.addr() + league::LAST_UPDATE_DATE, data) {
        let c = this.competition().addr();
        if read_usize(c + competition::NATION) != 0 {
            let k = exe::key_nation_index_of_comp(c) as i8 as isize;
            let mut day = (read_i16((0xb63d75 + k * 72) as usize) as u16).wrapping_add(1);
            if day > 0x16c {
                day = 0;
            }
            if day as i32 == read_i16(G_CURRENT_DATE) as i32 && this.year() as i32 == read_i16(G_CURRENT_DATE_YEAR) as u16 as i32 {
                exe::league_send_fixture_list_news(this, 0);
            }
        }
    }
    if this.stage_index() == stage_index::MAIN {
        let mut i = 0i32;
        while i <= this.current_stage() as i32 {
            let stage = read_usize(this.stages() + i as usize * 4);
            let f: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(vtable_method(League(stage), comp_vtable::UPDATE_FOR_DATE));
            f(stage, data, full);
            i += 1;
        }
    }
    if read_i16(this.addr() + league::LAST_RESULT_DATE + 2) == read_i16(G_CURRENT_DATE_YEAR) && this.last_result_date() == read_i16(G_CURRENT_DATE) && this.complete() == 0 {
        this.set_table_column(0);
        let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(vtable_method(this, league_vtable::UPDATE_POSITIONS_AND_STATUS));
        f(this.addr());
    }
    let mut follow_tv = true;
    if this.stage_index() == stage_index::MAIN {
        if this.table() == 0 {
            follow_tv = false;
        } else if !same_date(this.addr() + league::LAST_UPDATE_DATE, data) {
            let f: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(vtable_method(this, comp_vtable::CHECK_COMPLETE));
            f(this.addr(), 0, 0);
            if this.complete() == 1 {
                let mut acabadas = true;
                let mut i = 0i32;
                while i <= this.current_stage() as i32 {
                    let stage = read_usize(this.stages() + i as usize * 4);
                    let g: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(vtable_method(League(stage), comp_vtable::CHECK_COMPLETE));
                    if g(stage, 0, 1) as u8 == 0 {
                        acabadas = false;
                        break;
                    }
                    i += 1;
                }
                if acabadas {
                    if (this.current_stage() as i32) < (this.stage_count() as i32).wrapping_sub(1) {
                        let h: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(vtable_method(this, comp_vtable::ADVANCE_STAGE));
                        h(this.addr());
                    } else if this.season_closed() == 0 {
                        exe::club_history_record_comp_season(read_usize(G_CLUB_HISTORY), this);
                        if exe::comp_util_is_league_division((read_usize(this.competition().addr()) as u32) as i32) != 0 {
                            exe::club_records_update_league_position_records(read_usize(G_CLUB_RECORDS), this, 1);
                        }
                        if this.tiebreak_matches() != 0 {
                            show_error_at(0x9f15f4, 0x74c);
                        }
                        let h: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(vtable_method(this, comp_vtable::RECORD_SEASON_HONOURS));
                        h(this.addr());
                        if this.comp_stats() != 0 {
                            exe::comp_stats_update_season_end_records(this.comp_stats());
                        }
                        this.set_season_closed(1);
                    }
                }
            }
            if exe::comp_util_is_league_division((read_usize(this.competition().addr()) as u32) as i32) != 0 {
                exe::club_records_update_league_position_records(read_usize(G_CLUB_RECORDS), this, 0);
            }
            if this.comp_stats() != 0 {
                anexo(this.comp_stats());
            }
        }
    }
    if follow_tv && this.table() != 0 {
        if full == 0 {
            return 1;
        }
        if this.complete() == 0 {
            exe::date_add_weeks(pd0, (px) as *mut [u8; 8], 4); // cqd5: a TV quatro semanas adiante
            covered!("cqd_5 matches for tv");
            exe::league_move_fixtures_for_tv(this, px);
        }
    }
    if full == 0 {
        return 1;
    }
    if this.squad_flags() & squad_flags::FIXED_SQUAD_NUMBERS != 0 && this.stage_index() == stage_index::MAIN && !same_date(this.addr() + league::LAST_UPDATE_DATE, data)
        && this.complete() == 0 && read_u8(this.table() + league_table_row::PLAYED) == 0 {
        let e = this.round_dates();
        // simplification: the original pushes the day and the year in registers with leftovers in the high bytes; 0x549ef0 only reads the words
        exe::new_date(pt, read_i16(e) as u16 as u32, read_i16(e + 2).wrapping_add(this.year()) as u16 as u32);
        let c = this.competition().addr();
        let nation = if c != 0 && read_usize(c + competition::NATION) != 0 { read_usize(read_usize(c + competition::NATION)) } else { 0 };
        let has = c != 0 && read_usize(c + competition::NATION) != 0;
        let days = if has && nation == read_usize(KEY_NATION_ARGENTINA) {
            5
        } else if has && nation == read_usize(KEY_NATION_ITALY) {
            0x14
        } else {
            0xe
        };
        let r = exe::date_add_days(pd0, pw, days);
        if same_date(pt, r) {
            exe::squad_manager_assign_league_squad_numbers(read_usize(G_SQUAD_MANAGER), this, 0, 0);
        }
    }
    exe::date_operator_3d(this.addr() + league::LAST_UPDATE_DATE, pw, data);
    1
}

/// Fills one league fixture in record `r` (0x4f bytes) and adds it to the calendar.
///
/// The clubs come from entries `ea` (home) and `eb` (0x3b bytes in table +0xb1: +0 club, +0x38 mark); marks +0x40/+0x41
/// (mark 0); ids +0xc/+0x10 (−1 with no club); the stadium (`set_fixture_venue`: push +0x24; push &match; push [+0x3d of the
/// date]; mov ecx, [0xdd7ef4] -> thiscall, ret 0xc); the date (+0x28 year + the league's, +0x2a day) and the slice
/// (+0x3f) from entry `off` of the date table (+0xba); bits 0..1 of +0x4e cleared and bit 3 set in the last round
/// (+0x34 = +0xa9 − 1; or with `tipo4`, a type-4 date).
///
/// The match then goes to the calendar (`add_fixture`: push 1; push &match; mov ecx, [0xae2a58] -> thiscall, ret 8).
pub(crate) unsafe fn add_league_fixture(this: League, r: Fixture, ea: usize, eb: usize, off: usize, tipo4: bool) {
    let stadium: unsafe extern "thiscall" fn(usize, u32, usize, u32) = core::mem::transmute(FN_STADIUM_SET_FIXTURE_VENUE);
    let (ea, eb) = (ea, eb); // cqd5: the home and away sides
    let (ca, cb) = (read_usize(ea), read_usize(eb));
    r.set_home_club(Club(ca));
    r.set_away_club(Club(cb));
    r.set_home_side_mark((read_u8(ea + league_table_row::MARK) == 0) as u8);
    r.set_away_side_mark(((read_u8(eb + league_table_row::MARK) == 0) as u8) as i8);
    r.set_home_club_id(if ca != 0 { read_usize(ca) } else { u32::MAX as usize });
    r.set_away_club_id(if cb != 0 { read_usize(cb) } else { u32::MAX as usize });
    stadium(read_usize(G_STADIUM), read_usize(this.round_dates() + off + league_round_date::VENUE_VALUE) as u32, r.addr(), this.fixed_stadium_id() as u32);
    let e = this.round_dates() + off;
    r.set_year(read_i16(e + league_round_date::YEAR_OFFSET).wrapping_add(this.year()) as u16);
    r.set_day(read_i16(e) as u16);
    r.set_time_slice(read_u8(e + league_round_date::TIME_SLICE));
    r.set_squad_rules((r.squad_rules() as i16) as u16 & 0xfcff);
    if r.round_number() as i32 == this.round_count() as i32 - 1 || (tipo4 && read_u8(e + league_round_date::TV_SLOT_KIND) == 4) {
        r.set_flags(r.flags() | 8);
    } else {
        r.set_squad_rules((r.squad_rules() as i16) as u16 & 0xf7ff);
    }
    covered!("cqd_5 fixture list match");
    exe::fixture_manager_add_fixture(read_usize(G_FIXTURE_MANAGER), r, 1);
}

/// Creates the season fixtures: the round-robin grid (with the club order drawn) or the year's own list, home and away
/// by the parity of the half and the year, the stadium, date and slot of each round, and the calendar; then the TV
/// dates.
///
/// Ret 0; SEH frame with no destructor on the normal path (the local dates): `League::create_fixtures()`, from building
/// the competitions (44 calls up to 2002-06-01). With n = teams (`club_count()`) + teams % 2: `set_complete(0)`; the
/// grid of n rows of n dwords (`exe::crt_operator_new`; out of memory: the box at line 0x3fc or 0x405 and 0), built by
/// `exe::league_build_round_robin_grid` (push n; push grid -> stdcall, ret 8: it does not read the mov ecx, this before
/// it) and, outside competitions `KEY_CLUB_COMP_FIFA_WORLD_CUP` and `KEY_CLUB_COMP_EUROPEAN_FOOTBALL_CHAMPIONSHIP`, the
/// club order drawn (`exe::league_draw_team_order`: push n; mov ecx, this -> thiscall, ret 4). The match record (0x4f
/// bytes on the stack): +0 competition id, `fixture::MATCH_STATS_SLOT` = -1, `fixture::COMPETITION` the competition,
/// `fixture::NEXT_ROUND_YEAR`/`fixture::NEXT_ROUND_DAY` the last date of the table (`round_dates()`, entry
/// `round_count()` − 1), `fixture::SEASON_YEAR` year (`year()`), `fixture::ROUND_NAME` = 0 (`round_name::NONE`),
/// `fixture::ROUND_PREFIX` (`stage_id()`), `fixture::ROUND_DRAW_RULES` (`round_draw_rules()`), `fixture::TIE_RULES`
/// (`match_rules()`), `fixture::CATEGORY` (`match_category()`), `fixture::STAGE` = 0xff,
/// `fixture::HOME_RESULT_MARK`/`fixture::AWAY_RESULT_MARK` = 0, `fixture::STAGE_INDEX` stage (`stage_index()`),
/// +0x43..+0x4c = 0xff, and the league's method `comp_vtable::SET_FIXTURE_SUBSTITUTES` with it. With its own list
/// (`fixed_fixtures()`, 0x10-byte entries: +8/`fixed_fixture::AWAY_CLUB_ID` the club ids) for the league year: in each
/// round k (`round_count()`) the teams/2 matches of the list (the indices through table `table()`), and the list is
/// freed (`exe::crt_vector_destructor_iterator` + `exe::crt_operator_delete`). Otherwise the round robin: for j =
/// 1..legs (`legs()`)·(n − 1), the round through `exe::league_next_schedule_round` (push `league_flags()`; push
/// `round_count()`; push `legs()`; push `club_count()`; push id; push &step; push round; add esp, 0x1c -> cdecl) and
/// column j % (n − 1) (n − 1 at zero): each team i with opponent v ≠ 0 in the grid clears the opponent's slot; home
/// advantage by the parity of v > 0, of the half ((j − 1)/(n − 1)) and of year + stage; with a fixed stadium
/// (`fixed_stadium_id()`), the club whose stadium (`club::STADIUM`) it is stays at home. Each match goes to the
/// calendar (`add_league_fixture`). At the end, the grid is freed and the type 3 dates of the table
/// (`league_round_date::TV_SLOT_KIND`) take the matches to TV (`exe::league_move_fixtures_for_tv`). Returns 1.
/// Original: `League::create_fixtures` (0x6835c0, league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn create_fixtures(this: usize) -> u32 {
    let this = League(this);
    let mut round: i32 = -1;
    let teams = this.club_count() as i32;
    let n = teams + teams % 2;
    let mut step_towards: i8 = 0;
    let (mut x, mut y, mut z) = ([0u8; 8], [0u8; 8], [0u8; 8]);
    let (px, py, pz) = (x.as_mut_ptr() as usize, y.as_mut_ptr() as usize, z.as_mut_ptr() as usize);
    exe::date_date_54a110(px);
    this.set_complete(0);
    let grid = exe::crt_operator_new((n * 4 + 4) as u32);
    if grid == 0 {
        show_error_at(0x9f15f4, 0x3fc);
        return 0;
    }
    let mut i = 1i32;
    while i <= n {
        let l = exe::crt_operator_new((n * 4) as u32);
        if l == 0 {
            show_error_at(0x9f15f4, 0x405);
            exe::crt_operator_delete(grid);
            return 0;
        }
        write_usize(grid + i as usize * 4, l);
        i += 1;
    }
    exe::league_build_round_robin_grid(grid, n);
    let c = this.competition().addr();
    if c == 0 || (read_usize(c) != read_usize(KEY_CLUB_COMP_FIFA_WORLD_CUP) && read_usize(c) != read_usize(KEY_CLUB_COMP_EUROPEAN_FOOTBALL_CHAMPIONSHIP)) {
        covered!("cqd_5 order drawn");
        exe::league_draw_team_order(this, n as u32); // cqd5: the order of the clubs
    }
    // the match record; the bytes the original does not write (+4..+7, +0x18, +0x2e, +0x39, +0x4d and the high bits of
    // +0x4e: leftover from its stack, which method 0x7c and 0x85c260 may fill) stay 0, a value the game produces
    let mut reg = [0u8; 0x60];
    let r = Fixture(reg.as_mut_ptr() as usize);
    let c = this.competition().addr();
    r.set_competition(Competition(c));
    r.set_round_prefix((this.stage_id() as u16) as i16);
    write_usize(r.addr(), read_usize(c));
    r.set_season_year(this.year() as u16);
    r.set_tie_rules(this.match_rules() as u16);
    r.set_squad_rules((r.squad_rules() as i16) as u16 & 0xfbff);
    r.set_category(this.match_category());
    r.set_round_draw_rules(this.round_draw_rules() as u16);
    r.set_round_name((round_name::NONE) as i16);
    for k in 0x43..=0x4cusize {
        write_u8(r.addr() + k, 0xff);
    }
    r.set_match_stats_slot(u32::MAX as usize);
    r.set_stage_index(this.stage_index());
    r.set_home_result_mark((result_mark::NONE) as i8);
    r.set_away_result_mark((result_mark::NONE) as i8);
    r.set_stage(0xff);
    let prepare: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(vtable_method(this, comp_vtable::SET_FIXTURE_SUBSTITUTES));
    prepare(this.addr(), r.addr());
    let ult = this.round_dates().wrapping_add((this.round_count() as i32 * 0x41) as usize);
    // simplification: the original pushes the day and the year with leftover in the high bytes; 0x549ef0 only reads the words
    let yy = exe::new_date(py, read_i16(ult - 0x41) as u16 as u32, read_i16(ult - 0x3f).wrapping_add(this.year()) as u16 as u32);
    exe::date_operator_3d(px, pz, yy);
    r.set_next_round_day(read_i16(px) as u16);
    r.set_next_round_year(read_i16(px + 2) as u16);
    let list = this.fixed_fixtures();
    if list != 0 && this.year() as i32 == read_i16(list + 2) as u16 as i32 {
        let mut k = 0i32;
        let mut off = 0usize;
        while k < this.round_count() as i32 {
            let mut i = 0i32;
            loop {
                let teams = this.club_count() as i32;
                let half = teams / 2;
                if i >= half {
                    break;
                }
                r.set_round_number((k as u16) as i16);
                let e = this.fixed_fixtures() + ((half * k + i) as usize) * 0x10;
                let (mut ia, mut ib) = (-1i32, -1i32);
                if teams > 0 {
                    let (a, b) = (read_usize(e + 8), read_usize(e + fixed_fixture::AWAY_CLUB_ID));
                    let tb = this.table();
                    for p in 0..teams {
                        let id = read_usize(read_usize(tb + p as usize * 0x3b));
                        if a == id {
                            ia = p;
                        }
                        if b == id {
                            ib = p;
                        }
                    }
                }
                let tb = this.table();
                add_league_fixture(this, r, tb.wrapping_add((ia as isize * 0x3b) as usize), tb.wrapping_add((ib as isize * 0x3b) as usize), off, true);
                i += 1;
            }
            k += 1;
            off += 0x41;
        }
        let list = this.fixed_fixtures();
        if list != 0 {
            exe::crt_vector_destructor_iterator(list, 0x10, read_usize(list - 4) as u32, 0x5a53c0);
            exe::crt_operator_delete(list - 4);
        }
        this.set_fixed_fixtures(0);
    } else {
        let mut j = 1i32;
        while j <= this.legs() as i32 * (n - 1) {
            // simplification: the original pushes the words +0x3e, +0x3c, +0xa9 and +0xd9 with leftovers in the high bytes; 0x68a030
            // only reads the words and the byte
            round = exe::league_next_schedule_round(round, &mut step_towards, read_usize(this.competition().addr()) as u32, this.club_count() as u16 as u32,
                this.legs() as u16 as u32, this.round_count() as u16 as u32, this.league_flags() as u16 as u32);
            let mut col = j % (n - 1);
            if col == 0 {
                col = n - 1;
            }
            r.set_round_number((round as u16) as i16);
            if this.club_count() >= 1 {
                let mut i = 0i32;
                loop {
                    let v = read_usize(read_usize(grid + 4 + i as usize * 4) + col as usize * 4) as i32;
                    if v != 0 {
                        let turno = ((j - 1) / (n - 1)) % 2 != 0;
                        let parity = (this.year() as i32 + this.stage_index() as i8 as i32) % 2 != 0;
                        let idx = if v > 0 { v - 1 } else { -1 - v };
                        let row = read_usize(grid + 4 + (idx as isize * 4) as usize);
                        let (a, b) = if (v > 0) ^ turno ^ parity { (idx, i) } else { (i, idx) };
                        write_usize(row + col as usize * 4, 0);
                        let ult = this.club_count() as i32 - 1;
                        if a >= 0 && a <= ult && b >= 0 && b <= ult {
                            let tb = this.table();
                            let (ea, eb) = (tb + a as usize * 0x3b, tb + b as usize * 0x3b);
                            let sel = this.fixed_stadium_id() as u32;
                            let (m, f) = if sel != u32::MAX {
                                let stadium = read_usize(read_usize(eb) + club::STADIUM);
                                if stadium != 0 && read_usize(stadium) as u32 == sel { (eb, ea) } else { (ea, eb) }
                            } else {
                                (ea, eb)
                            };
                            add_league_fixture(this, r, m, f, (round as isize * 0x41) as usize, false);
                        }
                    }
                    i += 1;
                    if i + 1 > this.club_count() as i32 {
                        break;
                    }
                }
            }
            j += 1;
        }
    }
    let mut i = 0i32;
    while i < n {
        exe::crt_operator_delete(read_usize(grid + 4 + i as usize * 4));
        i += 1;
    }
    exe::crt_operator_delete(grid);
    let mut k = 0i32;
    while k < this.round_count() as i32 {
        let e = this.round_dates() + k as usize * 0x41;
        if read_u8(e + league_round_date::TV_SLOT_KIND) == 3 {
            exe::new_date(py, read_i16(e) as u16 as u32, read_i16(e + 2).wrapping_add(this.year()) as u16 as u32);
            exe::league_move_fixtures_for_tv(this, py);
        }
        k += 1;
    }
    1
}

/// Promotes and relegates between two leagues: the non-promotion clauses, the relegated clubs go down (relegation
/// clauses, human manager sacked in a competition with no active record) and the promoted clubs go up, in pairs or all;
/// the box if the counts differ from the expected ones.
///
/// Ret 0x18: `league_promote_and_relegate(above, below, contracts, aos_pairs, sobem, descem)`, from the end of the
/// leagues (after 2002-06-01). It receives the caller's ecx and passes it to `exe::league_move_club_to_division`, which
/// does not read it: thiscall here to keep it. With `contracts`, the clubs of the lower league that do not go up
/// (`league_table_row::STATUS` ≠ 0 and ≠ 5) trigger the non-promotion clauses
/// (`exe::contract_manager_trigger_non_promotion_clauses`: push league; push club; mov ecx, `G_CONTRACT_MANAGER` ->
/// thiscall, ret 8). In pairs: each relegated club from above (`league_table_row::STATUS` = 3) swaps with the first one
/// below that goes up (`league_table_row::STATUS` = 0 or 5): that one goes up (`exe::league_move_club_to_division`:
/// push 0; push upper competition; push club; caller's ecx -> thiscall, ret 0xc), the relegated one keeps the old
/// competition (`club::LAST_DIVISION`) and takes the lower one (`club::DIVISION`), triggers the relegation clauses
/// (`exe::contract_manager_trigger_relegation_clauses`: push old; push club; mov ecx, `G_CONTRACT_MANAGER` -> thiscall,
/// ret 8) and, in a competition with no active record and with a human manager (`exe::human_manager_is_human_club`),
/// loses the manager (`exe::manager_manager_remove_club_manager`: push 0; push 7; push 1; push club; mov ecx,
/// `G_MANAGER_MANAGER` -> thiscall, ret 0x10); both end with `league_table_row::STATUS` = 0xff. Otherwise, all
/// relegated clubs go down this way and then all the lower ones that go up go up. Counts that differ from the expected
/// ones (0xff = any) give the box at line 0x15dc (going up) or 0x15e2 (going down).
/// Original: `league_promote_and_relegate` (0x689c80, league.cpp, thiscall (ecx repassado), ret 0x18).
pub(crate) unsafe extern "thiscall" fn league_promote_and_relegate(ecx: usize, above: usize, below: usize, contracts: u32, aos_pairs: u32, sobem: u32, descem: u32) {
    let below = League(below);
    let above = League(above);
    let (mut desceram, mut subiram) = (0u8, 0u8);
    if contracts != 0 {
        let mut i = 0i32;
        while i < below.club_count() as i32 {
            let e = LeagueTableRow(below.table() + i as usize * league_table_row::SIZE);
            let s = e.status();
            if s != 5 && s != 0 {
                exe::contract_manager_trigger_non_promotion_clauses(read_usize(G_CONTRACT_MANAGER), read_usize(e.addr()), below.competition().addr());
            }
            i += 1;
        }
    }
    // the relegated club goes down to the lower competition and loses the human manager in a competition with no active registration
    let descer = |club: Club| {
        let comp = below.competition().addr();
        let old = club.division().addr();
        write_usize(club.addr() + club::LAST_DIVISION, old);
        write_usize(club.addr() + club::DIVISION, comp);
        exe::contract_manager_trigger_relegation_clauses(read_usize(G_CONTRACT_MANAGER), club, old);
        let table = read_usize(G_COMP_OBJECT_TABLE);
        if table == 0 || comp == 0 || read_usize(table + read_usize(comp) * 4) == 0 {
            if exe::human_manager_is_human_club(club, 1, 1, 0, 0) != 0 {
                exe::manager_manager_remove_club_manager(read_usize(G_MANAGER_MANAGER), club, 1, 7, 0);
            }
        }
    };
    if aos_pairs != 0 {
        let mut i = 0i32;
        while i < above.club_count() as i32 {
            let ue = above.table() + i as usize * league_table_row::SIZE;
            if read_u8(ue + league_table_row::STATUS) == 3 {
                let n = below.club_count() as i32;
                let mut j = 0i32;
                while j < n {
                    let s = read_u8(below.table() + j as usize * league_table_row::SIZE + league_table_row::STATUS);
                    if s == 0 || s == 5 {
                        break;
                    }
                    j += 1;
                }
                if j < n {
                    let le = below.table() + j as usize * league_table_row::SIZE;
                    exe::league_move_club_to_division(ecx, read_usize(le), above.competition().addr(), 0);
                    descer(Club(read_usize(ue)));
                    desceram = desceram.wrapping_add(1);
                    subiram = subiram.wrapping_add(1);
                    write_u8(le + league_table_row::STATUS, 0xff);
                    write_u8(ue + league_table_row::STATUS, 0xff);
                }
            }
            i += 1;
        }
    } else {
        let mut i = 0i32;
        while i < above.club_count() as i32 {
            let ue = above.table() + i as usize * league_table_row::SIZE;
            if read_u8(ue + league_table_row::STATUS) == 3 {
                descer(Club(read_usize(ue)));
                desceram = desceram.wrapping_add(1);
                write_u8(ue + league_table_row::STATUS, 0xff);
            }
            i += 1;
        }
        let mut i = 0i32;
        while i < below.club_count() as i32 {
            let le = below.table() + i as usize * league_table_row::SIZE;
            let s = read_u8(le + league_table_row::STATUS);
            if s == 0 || s == 5 {
                exe::league_move_club_to_division(ecx, read_usize(le), above.competition().addr(), 0);
                subiram = subiram.wrapping_add(1);
                write_u8(le + league_table_row::STATUS, 0xff);
            }
            i += 1;
        }
    }
    if sobem as u8 != 0xff && subiram != sobem as u8 {
        show_error_at(0x9f15f4, 0x15dc);
    }
    if descem as u8 != 0xff && desceram != descem as u8 {
        show_error_at(0x9f15f4, 0x15e2);
    }
}

/// In the dry run (the comparer of `0x684cd0`) the copy does not call virtuals `+0x40` (the status announcement, which
/// sends news and writes the records) and `+0x68` (the status in the statistics), nor the relegation clauses
/// (`0x4cd030`).
pub(crate) static POSITIONS_STATUS_DRY_RUN: AtomicBool = AtomicBool::new(false);

/// The tape of the comparer of `update_positions_and_status`: the original routine runs first and each of its announcements (+0x40 =
/// `announce_team_status`, through its comparer) stores the club, the status and the bytes that the competition's virtual +0x44 wrote
/// in the regions (BraNatFirst `bra_nat_first_update_staff_records_for_status`, BraNatSecond 0x42d040, BelThird 0x4230d0 and FinFirst 0x594810 put 5 or 0xfe
/// in the row's +0x37).
///
/// In the dry run, the copy's announcement that matches the next one on the tape restores those bytes.
pub(crate) static mut STATUS_ANNOUNCE_RECORD: Option<Vec<(usize, usize)>> = None;
pub(crate) static mut STATUS_ANNOUNCE_TAPE: Vec<(usize, u8, Vec<(usize, u8)>)> = Vec::new();
pub(crate) static STATUS_ANNOUNCE_TAPE_POS: AtomicUsize = AtomicUsize::new(0);

/// Returns row i of the league table (+0xb1, 0x3b bytes).
pub(crate) unsafe fn league_table_row(league: League, i: i32) -> usize {
    league.table().wrapping_add((i as usize).wrapping_mul(league_table_row::SIZE))
}

/// Calls the league's virtual +0x40 (League::announce_team_status: thiscall(league, club, situation, stage, 0, 0), ret
/// 0x14) with the club of row `i`.
///
/// `alto` = the upper bits of the register that the original routine pushes with the stage (+0x44) in the low byte (the
/// row pointer or the clauses' return value: reproduced). In the rehearsal the recorded tape is replayed.
pub(crate) unsafe fn league_announce_team_status(league: League, i: i32, status: u32, high: u32) {
    if POSITIONS_STATUS_DRY_RUN.load(Relaxed) {
        let tape_lock = &*core::ptr::addr_of!(STATUS_ANNOUNCE_TAPE);
        if let Some((clube, s, bytes)) = tape_lock.get(STATUS_ANNOUNCE_TAPE_POS.load(Relaxed)) {
            STATUS_ANNOUNCE_TAPE_POS.fetch_add(1, Relaxed);
            if *clube == read_usize(league_table_row(league, i)) && *s == status as u8 {
                for &(a, v) in bytes {
                    write_u8(a, v);
                }
            }
        }
        return;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(read_usize(read_usize(league.addr()) + comp_vtable::ANNOUNCE_TEAM_STATUS));
    f(league.addr(), read_usize(league_table_row(league, i)), status, high | league.stage_index() as u32, 0, 0);
}

/// Writes status v at +0x37 of table row i and announces it.
pub(crate) unsafe fn set_row_status(league: League, i: i32, v: u8, high: u32) {
    write_u8(league_table_row(league, i) + league_table_row::STATUS, v);
    league_announce_team_status(league, i, v as i8 as i32 as u32, high);
}

/// Maximum reachable points of a table row: (season games +0xa7 − played +0x11) × win points (+0xc2) + points (+0x19).
pub(crate) unsafe fn max_points(league: League, l: LeagueTableRow) -> i32 {
    ((league.games_per_club() as i32) - (l.played() as i32)).wrapping_mul((league.win_points() as i8) as i32).wrapping_add(l.points() as i32)
}

/// Sorts the table (`exe::league_sort_table`), marks the bands (promotion, play-offs, 2 and relegation) from the top
/// and the bottom, works out the best (`league_table_row::BEST_POSSIBLE_POSITION`) and worst
/// (`league_table_row::WORST_POSSIBLE_POSITION`) possible position from the maximum points and decides the status
/// (`league_table_row::STATUS`): title (5), promotion (0, 7 or 6), play-offs (1), relegation (3, with the clauses
/// `exe::contract_manager_trigger_relegation_clauses`), 2 and 0xfe, each announced
/// (`comp_vtable::ANNOUNCE_TEAM_STATUS`); the European place (`G_CLUB_STATE_TABLE`) from the nation places
/// (`exe::uefaseeding_get_nation_places`); the stats progress (`comp_vtable::SET_CLUB_PROGRESS`).
///
/// Ret 0: `League::update_positions_and_status()`, virtual `league_vtable::UPDATE_POSITIONS_AND_STATUS` of the league
/// (from the day pass 0x684946). `games_per_club()` (the season's matches) = (`club_count()` − 1) × `legs()` (16 bits)
/// if still −1, with rows and `active()`. With `complete()`, nothing more. Otherwise sorts the table
/// (League::sort_table `exe::league_sort_table`, thiscall(league, 0), ret 4) and builds the band map (malloc
/// `exe::crt_malloc` of `club_count()` bytes set to 0xff; out of memory: the box at line 0x7c9): from the top, counts
/// `promoted_count()` (band 0, promotion) and `promotion_playoff_count()` (band 1, play-offs) on the rows not in 0xfe,
/// 0xfc or 3; from the bottom, `relegated_count()` (band 3, relegation) and `relegation_playoff_count()` (band 2) on
/// those not in 0, 1 or 5. With `league_flags::CHAMPION_STATUS` (bit 0 of `league_flags()`), the first in the table
/// outside 0xfe, 0xfc and 3. The best position (`league_table_row::BEST_POSSIBLE_POSITION`) and the worst
/// (`league_table_row::WORST_POSSIBLE_POSITION`) of each row not yet decided, from the maximum points (`max_points`)
/// and the matches played (with `tiebreak_matches()` on a tie among those that finished). Then, row by row: worst =
/// first + 1, the title (5); band 0 at the worst, promotion (0 with `league_flags()` bit 1; 7 with bit 2; 6 with bit 3)
/// for those in 0xff or 1; band 1, play-offs (1) for those in 0xff; band 3 at the best, the relegation clauses
/// (CONTRACT_MANAGER::trigger_relegation_clauses `exe::contract_manager_trigger_relegation_clauses`,
/// thiscall(`G_CONTRACT_MANAGER`, club, `competition()`), ret 8) and relegation (3) for those in 0xff, 2 or 0xfc; band
/// 2 at the best, 2 for those in 0xff or 0xfe. Each new status is announced (`set_row_status`,
/// `comp_vtable::ANNOUNCE_TEAM_STATUS`). With `league_flags()` bit 0x20, those between the bands with no band in the
/// middle and in 0xff become 0xfe (announced with −2). Best = worst below 9, top league (`stage_index()` =
/// `stage_index::MAIN`) that counts (`exe::comp_util_is_league_division`, cdecl(id)), without an upper division
/// (`upper_division_id()` = −1) and a club with no European place (`G_CLUB_STATE_TABLE` + 6 × id = −1) from a European
/// nation (`KEY_CONTINENT_EUROPE`) outside `KEY_NATION_SCOTLAND`: the nation's places (UefaSeeding::get_nation_places
/// `exe::uefaseeding_get_nation_places`, thiscall(`G_UEFA_SEEDING`, nation, &cl, &uefa), ret 0xc) give the Champions
/// League (`KEY_CLUB_COMP_UEFA_EUROPEAN_CHAMPIONS_LEAGUE`) or the UEFA Cup (`KEY_CLUB_COMP_UEFA_CUP`). Finally virtual
/// `comp_vtable::SET_CLUB_PROGRESS` (the status in the stats: club, `stage_index()`, position, best, worst) and the
/// free (`exe::crt_free`) of the map.
/// Original: `League::update_positions_and_status` (0x684cd0, league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn update_positions_and_status(this: usize) {
    let this = League(this);
    let n0 = this.club_count();
    if this.games_per_club() == -1 && n0 > 0 && this.active() != 0 {
        this.set_games_per_club((n0 - 1).wrapping_mul(this.legs()));
    }
    if this.complete() != 0 {
        return;
    }
    exe::league_sort_table(this, 0);
    let map = exe::crt_malloc(this.club_count() as i32 as u32);
    if map == 0 {
        return show_error_at(0x9f15f4, 0x7c9);
    }
    let n = this.club_count();
    core::ptr::write_bytes(map as *mut u8, 0xff, n as i32 as usize);
    let m = |i: i32| (map as isize + i as isize) as *mut u8;
    let mut contas = [this.promoted_count(), this.promotion_playoff_count(), this.relegation_playoff_count(), this.relegated_count()];
    let mut first: i8 = -1;
    if (this.league_flags() as u8) & (league_flags::CHAMPION_STATUS as u8) != 0 {
        let mut i: i16 = 0;
        while i < n {
            let s = read_u8(league_table_row(this, i as i32) + league_table_row::STATUS);
            if s != 0xfe && s != 0xfc && s != 3 {
                first = i as i8;
                break;
            }
            i += 1;
        }
    }
    // upper: bands 0 and 1
    let mut i: i32 = 0;
    if n > 0 {
        let mut k: i8 = 0;
        while k <= 1 {
            if contas[k as usize] != 0 {
                while (i as i16) < n {
                    let s = read_u8(league_table_row(this, i as i16 as i32) + league_table_row::STATUS);
                    if s != 0xfe && s != 0xfc && s != 3 {
                        *m(i as i16 as i32) = k as u8;
                        contas[k as usize] = contas[k as usize].wrapping_sub(1);
                    }
                    i += 1;
                    if contas[k as usize] == 0 {
                        break;
                    }
                }
            }
            k += 1;
            if (i as i16) >= n {
                break;
            }
        }
    }
    // from below: bands 3 and 2
    let mut i: i32 = (n as i32) - 1;
    let mut k: i8 = 3;
    while (i as i16) >= 0 && k >= 2 {
        if contas[k as usize] != 0 {
            while (i as i16) >= 0 {
                let s = read_u8(league_table_row(this, i as i16 as i32) + league_table_row::STATUS);
                if s != 0 && s != 1 && s != 5 {
                    *m(i as i16 as i32) = k as u8;
                    contas[k as usize] = contas[k as usize].wrapping_sub(1);
                }
                i -= 1;
                if contas[k as usize] == 0 {
                    break;
                }
            }
        }
        k -= 1;
    }
    // the best (+0x39) and the worst (+0x3a) possible position
    let mut i: i32 = 0;
    while (i as i16) < this.club_count() {
        let li = league_table_row(this, i as i16 as i32);
        if read_u8(li + league_table_row::BEST_POSSIBLE_POSITION) != read_u8(li + league_table_row::WORST_POSSIBLE_POSITION) {
            let pos = (i as u8).wrapping_add(1);
            write_u8(li + league_table_row::BEST_POSSIBLE_POSITION, pos);
            // the upper ones (j < i): the best drops one for each one that i’s maximum reaches
            let max_i = max_points(this, LeagueTableRow(li));
            let mut j: i32 = 0;
            while (j as i16) < (i as i16) {
                let lj = league_table_row(this, j);
                let games = this.games_per_club();
                let end_i = (read_i8(li + league_table_row::PLAYED) as i16) >= games;
                let end_j = (read_i8(lj + league_table_row::PLAYED) as i16) >= games;
                let pj = read_i16(lj + league_table_row::POINTS) as i32;
                let cai = if !end_i && !end_j {
                    max_i >= pj
                } else if !end_j {
                    max_i == pj
                } else if !end_i {
                    max_i >= pj
                } else {
                    max_i == pj && this.tiebreak_matches() != 0
                };
                if cai {
                    write_u8(li + league_table_row::BEST_POSSIBLE_POSITION, read_u8(li + league_table_row::BEST_POSSIBLE_POSITION).wrapping_sub(1));
                }
                j += 1;
            }
            // the lower ones (j > i): the worst moves up one for each whose maximum reaches i
            write_u8(li + league_table_row::WORST_POSSIBLE_POSITION, pos);
            let mut j: i32 = pos as i8 as i32;
            while (j as i16) < this.club_count() {
                let lj = league_table_row(this, j as i16 as i32);
                let games = this.games_per_club();
                let pj = read_i16(lj + league_table_row::POINTS) as i32;
                let max_j = max_points(this, LeagueTableRow(lj));
                let end_i = (read_i8(li + league_table_row::PLAYED) as i16) >= games;
                let end_j = (read_i8(lj + league_table_row::PLAYED) as i16) >= games;
                let pi = read_i16(li + league_table_row::POINTS) as i32;
                let sobe = if !end_i && !end_j {
                    max_j >= pi
                } else if !end_j {
                    max_j >= pi
                } else if !end_i {
                    max_j == pi
                } else {
                    max_j == pj && this.tiebreak_matches() != 0
                };
                if sobe {
                    write_u8(li + league_table_row::WORST_POSSIBLE_POSITION, read_u8(li + league_table_row::WORST_POSSIBLE_POSITION).wrapping_add(1));
                }
                j += 1;
            }
        }
        i += 1;
    }
    // the situation of each row
    let one_after = (first as i32) + 1;
    let mut i: i32 = 0;
    while (i as i16) < this.club_count() {
        let ii = i as i16 as i32;
        let li = league_table_row(this, ii);
        let high_row = (li as u32) & !0xff;
        let worst = read_i8(li + league_table_row::WORST_POSSIBLE_POSITION) as i32;
        let s = read_u8(li + league_table_row::STATUS);
        if worst == one_after {
            if s != 3 && s != 0xfe && s != 0xfc && s != 5 {
                covered!("cpz_3 title");
                set_row_status(this, ii, 5, high_row); // cpz3: the title in the worst position = the first + 1
            }
        } else {
            let f = *m(worst - 1);
            if f == 0 {
                if s == 0xff || s == 1 {
                    let d9 = this.league_flags() as u8;
                    if d9 & 2 != 0 {
                        if s != 0 {
                            set_row_status(this, ii, 0, high_row);
                        }
                    } else if d9 & 4 != 0 {
                        if s != 7 {
                            set_row_status(this, ii, 7, high_row);
                        }
                    } else if d9 & 8 != 0 {
                        if s != 6 {
                            set_row_status(this, ii, 6, high_row);
                        }
                    }
                }
            } else if f == 1 {
                if s == 0xff {
                    covered!("cpz_3 knockout slot");
                    set_row_status(this, ii, 1, high_row); // cpz3: the knockout slot (situation 1)
                }
            } else {
                let g = *m(read_i8(li + league_table_row::BEST_POSSIBLE_POSITION) as i32 - 1);
                if g == 3 {
                    let mut high = high_row;
                    if !POSITIONS_STATUS_DRY_RUN.load(Relaxed) {
                        covered!("cpz_3 relegation clauses");
                        high = exe::contract_manager_trigger_relegation_clauses(read_usize(G_CONTRACT_MANAGER), read_usize(li), this.competition().addr()) & !0xff; // cpz3: the relegation clauses
                    }
                    let li = league_table_row(this, ii);
                    let s = read_u8(li + league_table_row::STATUS);
                    if s == 0xff || s == 2 || s == 0xfc {
                        set_row_status(this, ii, 3, high);
                    }
                } else if g == 2 {
                    if s == 0xff || s == 0xfe {
                        set_row_status(this, ii, 2, high_row);
                    }
                }
            }
        }
        // between bands, no band in the middle: 0xfe
        if (this.league_flags() as u8) & 0x20 != 0 {
            let li = league_table_row(this, ii);
            let c0 = (this.promoted_count() as i8) as i32;
            let c2 = (this.relegation_playoff_count() as i8) as i32;
            let best = read_i8(li + league_table_row::BEST_POSSIBLE_POSITION);
            if (best as i32) > c0 + c2 {
                let c3 = (this.relegated_count() as i8) as i32;
                let worst = read_i8(li + league_table_row::WORST_POSSIBLE_POSITION);
                if (worst as i32) <= (this.club_count() as i32) - c3 - c2 {
                    let mut free = true;
                    let mut a: i16 = (best as i16) - 1;
                    while a < worst as i16 {
                        if !free {
                            break;
                        }
                        if *m(a as i32) != 0xff {
                            free = false;
                        }
                        a += 1;
                    }
                    if free && read_u8(li + league_table_row::STATUS) == 0xff {
                        write_u8(li + league_table_row::STATUS, 0xfe);
                        league_announce_team_status(this, ii, 0xffff_fffe, (li as u32) & 0xffff_0000);
                    }
                }
            }
        }
        // the European place of those whose position is already decided
        let li = league_table_row(this, ii);
        let best = read_i8(li + league_table_row::BEST_POSSIBLE_POSITION);
        if best == read_i8(li + league_table_row::WORST_POSSIBLE_POSITION) && best < 9 && this.stage_index() == stage_index::MAIN {
            if exe::comp_util_is_league_division(read_i32(this.competition().addr())) != 0 && this.upper_division_id() == -1 {
                let slot = read_usize(G_CLUB_STATE_TABLE) + (read_i32(read_usize(li)) as usize).wrapping_mul(6);
                let reg = this.competition();
                let nation = reg.nation();
                if read_i32(slot) == -1 && !nation.is_null() && read_i32(nation.addr()) != read_i32(KEY_NATION_SCOTLAND) {
                    let count = nation.continent();
                    if count != 0 && read_i32(count) == read_i32(KEY_CONTINENT_EUROPE) {
                        let (mut cl, mut uefa) = (0i8, 0i8);
                        exe::uefaseeding_get_nation_places(read_usize(G_UEFA_SEEDING), nation, &mut cl, &mut uefa);
                        let li = league_table_row(this, ii);
                        let best = read_i8(li + league_table_row::BEST_POSSIBLE_POSITION) as i32;
                        let slot = read_usize(G_CLUB_STATE_TABLE) + (read_i32(read_usize(li)) as usize).wrapping_mul(6);
                        if best <= cl as i32 {
                            write_i32(slot, read_i32(KEY_CLUB_COMP_UEFA_EUROPEAN_CHAMPIONS_LEAGUE));
                        } else if best <= (cl as i32) + (uefa as i32) - 1 {
                            write_i32(slot, read_i32(KEY_CLUB_COMP_UEFA_CUP));
                        }
                    }
                }
            }
        }
        // the situation in the statistics (virtual +0x68)
        if !POSITIONS_STATUS_DRY_RUN.load(Relaxed) {
            let li = league_table_row(this, ii);
            // simplification: the original pushes the worst, the best, the position and the stage in registers with garbage in the bits of
            // top; the +0x68 ones (0x573260, 0x578660, Comp::set_club_progress 0x48e380) only read the byte.
            let f: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::SET_CLUB_PROGRESS));
            f(this.addr(), read_usize(li), this.stage_index() as u32, (i as u8).wrapping_add(1) as u32, read_u8(li + league_table_row::BEST_POSSIBLE_POSITION) as u32, read_u8(li + league_table_row::WORST_POSSIBLE_POSITION) as u32);
        }
        i += 1;
    }
    exe::crt_free(map);
}

/// Memory regions `update_positions_and_status` writes: the whole table, +0xa7 and each club's European slot in G_CLUB_STATE_TABLE.
pub(crate) unsafe fn table_status_regions(this: League) -> Vec<(usize, usize)> {
    let n = this.club_count().max(0) as usize;
    let mut v = vec![(this.addr() + league::GAMES_PER_CLUB, 2)];
    let table = this.table();
    if n > 0 && table != 0 {
        v.push((table, n * 0x3b));
        let slots = read_usize(G_CLUB_STATE_TABLE);
        if slots != 0 {
            for i in 0..n {
                let c = read_usize(table + i * 0x3b);
                if c != 0 {
                    let a = slots + (read_i32(c) as usize).wrapping_mul(6);
                    if !v.iter().any(|r| r.0 == a) {
                        v.push((a, 4));
                    }
                }
            }
        }
    }
    v
}

/// Advances a league with knockouts to the next stage (+0x30 + 1), calling the knockout routine when it reaches 0;
/// warns at the last stage.
pub(crate) unsafe fn advance_league_stage(this: League, file: usize, row: u32, knockout_knockout: usize, name: &'static str) {
    let f = this.current_stage() as i32;
    if f >= (this.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(file, row);
    }
    let f = f.wrapping_add(1);
    this.set_current_stage((f) as usize);
    if f == 0 {
        covered!(name);
        let create: unsafe extern "thiscall" fn(usize) = core::mem::transmute(knockout_knockout);
        create(this.addr());
    }
}

/// Comparer for the stage-advance routines: side by side on +0x30 when the stage does not reach 0; only the original at
/// the last stage or when the knockout is created.
pub(crate) unsafe fn compare_stage_advance(name: &str, this: League, original: impl FnOnce(), copy: impl FnOnce()) {
    let f = this.current_stage() as i32;
    if f >= (this.stage_count() as i32).wrapping_sub(1) || f.wrapping_add(1) == 0 {
        return original();
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side_light(&[(this.addr() + league::CURRENT_STAGE, 4)], original, copy);
    SCOREBOARD.record(igual, || format!("{}({:#x}) original={:x?} rust={:x?}", name, this.addr(), mo, mr));
}

/// The dry run of the comparer of `0x682e10`.
///
/// The copy runs without the prize news (`0x48e3e0`, which allocates and cannot run twice).
pub(crate) static LEAGUE_PRIZE_MONEY_DRY_RUN: AtomicBool = AtomicBool::new(false);

/// Prize divisor 1 + 2 + … + n, truncated to 16 bits like the original.
pub(crate) unsafe fn sum_of_positions(n: i16) -> i32 {
    let (mut s, mut k) = (0i32, 0i32);
    if n > 0 {
        loop {
            s = s.wrapping_add(k).wrapping_add(1);
            k += 1;
            if k as i16 >= n {
                break;
            }
        }
    }
    s as i16 as i32
}

/// Pays the prize money: divides the prize fund (`prize_fund()`) by 1 + 2 + ... + n and pays (n − i) shares to the club
/// in table row i (`exe::finance_add_prize_money` on the finance record), with the prize news item above 1000; with
/// `prize_mode()` = 3 the competition pays through virtual `comp_vtable::PAY_STAGE_PRIZE_MONEY`; with `prize_mode()` =
/// 2, nothing.
///
/// Ret 0: `League::pay_prize_money()`: with `prize_mode()` = 2, nothing. With `prize_mode()` = 3, the competition
/// (`G_COMP_OBJECT_TABLE` by the id of `competition()`) pays through virtual `comp_vtable::PAY_STAGE_PRIZE_MONEY`
/// (without the competition table: the box at line 0x1df). Otherwise the fund (`prize_fund()`) divided by 1 + 2 + … + n
/// (n = `club_count()` table rows) is the share; the club of row i (`table()`, `league_table_row::SIZE` each) receives
/// (n − i) shares in the finance record (`european_cup_club_finance`; `exe::finance_add_prize_money` adds to the
/// balance, in +0xa8 and +0x148) and, above 1000, the prize news (`FN_COMP_SEND_PRIZE_MONEY_NEWS` with position i + 1).
/// Original: `League::pay_prize_money` (0x682e10, league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn pay_prize_money(league: usize) {
    let league = League(league);
    let e1 = league.prize_mode();
    if e1 == 2 {
        return;
    }
    if e1 == 3 {
        let t = read_usize(G_COMP_OBJECT_TABLE);
        if t == 0 {
            return show_error_at(0x9f15f4, 0x1df);
        }
        let comp = read_usize(t.wrapping_add(read_usize(league.competition().addr()).wrapping_mul(4)));
        let pay: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(read_usize(read_usize(comp) + comp_vtable::PAY_STAGE_PRIZE_MONEY));
        return pay(comp, league.addr());
    }
    let s = sum_of_positions(league.club_count());
    let bottom = (league.prize_fund() as usize) as i32;
    if s == 0 || (bottom == i32::MIN && s == -1) {
        // simplification: the original divides by zero (or overflows) and crashes; original only, to crash the same way
        return original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_PAY_PRIZE_MONEY)(league.addr());
    }
    let part = bottom / s;
    if league.club_count() <= 0 {
        return;
    }
    let news: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32, i32) = core::mem::transmute(FN_COMP_SEND_PRIZE_MONEY_NEWS);
    let mut i: i16 = 0;
    loop {
        let value = (league.club_count() as i32).wrapping_sub(i as i32).wrapping_mul(part);
        let club = read_usize(league.table().wrapping_add(league_table_row::SIZE * i as usize));
        let fin = european_cup_club_finance(club);
        covered!("cpx_11 bonus paid");
        exe::finance_add_prize_money(fin, value); // cpx11: o prêmio
        if value > 1000 && !LEAGUE_PRIZE_MONEY_DRY_RUN.load(Relaxed) {
            // simplification: the original pushes the position in dl and the rest of edx (29 × i, leftover from the row calculation); the
            // 0x48e3e0 reads only the byte
            let pos = ((29 * i as i32 as u32) & !0xff) | (i as u8).wrapping_add(1) as u32;
            news(league.addr(), club, value as u32, (value >> 31) as u32, pos, 0, 0, -2);
        }
        i = i.wrapping_add(1);
        if i >= league.club_count() {
            break;
        }
    }
}

/// Updates the staff records by status: 5 the title (`exe::staff_records_add_league_champions_record`), 0 promotion
/// (`exe::staff_records_add_promotion_record`, weight 100), 3 relegation (`exe::staff_records_add_relegation_record`);
/// returns 0.
///
/// Ret 0x18: `club.update_staff_records_for_status(status, stage, out, _, _)` (virtual
/// `comp_vtable::UPDATE_STAFF_RECORDS_FOR_STATUS` of the league): status 5, the title (Staff_Records
/// `exe::staff_records_add_league_champions_record` of `G_STAFF_RECORDS` with the club and competition +4); 0,
/// promotion (`exe::staff_records_add_promotion_record` with weight 100); 3, relegation
/// (`exe::staff_records_add_relegation_record`). Returns 0 and does not write the output.
/// Original: `League::update_staff_records_for_status` (0x686940, league.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn league_update_staff_records_for_status(league: usize, club: usize, status: u32, _stage: u32, _out: usize, _a: u32, _b: u32) -> u32 {
    let records = read_usize(G_STAFF_RECORDS);
    let comp = read_usize(league + 4);
    match status as u8 {
        5 => {
            covered!("cpx_11 title record");
            exe::staff_records_add_league_champions_record(records, club, comp);
        }
        0 => {
            covered!("cpx_11 promotion record");
            exe::staff_records_add_promotion_record(records, club, comp, 0x64);
        }
        3 => {
            covered!("cpx_11 relegation record");
            exe::staff_records_add_relegation_record(records, club, comp);
        }
        _ => {}
    }
    0
}

// simplification: the three records append to the people’s history and change reputation (they allocate): original only; the
// the copy takes the branches.

/// Announces a team's status: outside friendlies, calls the competition's virtual
/// `comp_vtable::UPDATE_STAFF_RECORDS_FOR_STATUS` and builds news item 0xc (club, competition, stage, status, round,
/// what the virtual returned), delivered by scope or, by status, to the nation, the division (and the promotion or
/// relegation one) or the club's manager.
///
/// Ret 0x14; SEH frame with no destructor: `club.announce_team_status(status, stage, _, _)`: friendly
/// (`exe::friendly_is_friendly_competition`), 0. Virtual `comp_vtable::UPDATE_STAFF_RECORDS_FOR_STATUS` of the
/// competition (`G_COMP_OBJECT_TABLE` by the id of `competition()`) with the club, the status, the stage and the output
/// (0xff); −1 from it: 0. The round (`stage_id()`; with stages, `stage_index()` = `stage_index::MAIN`, and stage ≠
/// 0xff: that of the stage, `league::STAGE_ID`). News 0xc (local): 0 the club, 1 the competition, 2 the stage, 3 the
/// status, 4 the round, 5 what the virtual returned. Output ≠ 0xff: delivery by scope (`exe::news_send_by_scope` with
/// the club's nation and the continent). Otherwise by status: 5, to the competition's nation
/// (`FN_NEWS_SEND_TO_NATION_MANAGERS`) without an upper division (`upper_division_id()`), to the division
/// (`exe::news_send_to_division_managers`) with it; 0 and 3, to the division and to the competition's promotion (+0x1c)
/// or relegation (+0x20) division, if any; 1 and 2, to the division; 6, 7 and 0xfe, to the club's manager
/// (`exe::news_send_to_club_manager`). Returns 1.
/// Original: `League::announce_team_status` (0x686670, league.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn announce_team_status(league: usize, club: usize, status: u32, stage: u32, _a: u32, _b: u32) -> u32 {
    let league = League(league);
    let club = Club(club);
    let nation: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_NATION_MANAGERS);
    // simplification: the original writes only the output byte in a stack dword and pushes the whole dword to 0x786090, which
    // reads the byte; the copy passes the high part zeroed
    let mut out: u32 = 0xff;
    let mut round = league.stage_id();
    if exe::friendly_is_friendly_competition(read_usize(G_FRIENDLY), league.competition().addr()) != 0 {
        return 0;
    }
    let comp = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(league.competition().addr()).wrapping_mul(4)));
    let virtual44: unsafe extern "thiscall" fn(usize, usize, u32, u32, *mut u32, u32, u32) -> i32 = core::mem::transmute(read_usize(read_usize(comp) + comp_vtable::UPDATE_STAFF_RECORDS_FOR_STATUS));
    let r = virtual44(comp, club.addr(), status, stage, &mut out, 0, 0);
    if r == -1 {
        return 0;
    }
    if league.stage_index() == stage_index::MAIN && stage as u8 != 0xff {
        let f = League(read_usize(league.stages().wrapping_add((stage as u8 as i8 as isize as usize).wrapping_mul(4))));
        round = f.stage_id();
    }
    covered!("cpx_11 situation news");
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0xc, 0);
    exe::newsitem_set_slot(n, 0, read_usize(club.addr()) as u32);
    exe::newsitem_set_slot(n, 1, read_usize(league.competition().addr()) as u32);
    exe::newsitem_set_slot(n, 2, stage as u8 as i8 as i32 as u32);
    exe::newsitem_set_slot(n, 3, status as u8 as i8 as i32 as u32);
    exe::newsitem_set_slot(n, 4, round as i32 as u32);
    exe::newsitem_set_slot(n, 5, r as u32);
    let news = read_usize(G_NEWS);
    if out as u8 != 0xff {
        let nacao = club.nation();
        let continent = if !nacao.is_null() { nacao.continent() } else { 0 };
        exe::news_send_by_scope(news, n, club, league.competition().addr(), nacao, continent, out);
        return 1;
    }
    // the competition’s promotion (+0x1c) or relegation (+0x20) division, 0x6b records in [0xae23d0]
    let other = |desl: usize| {
        let c = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(league.competition().addr()).wrapping_mul(4)));
        let e = read_usize(c + desl) as i32;
        if e != -1 {
            exe::news_send_to_division_managers(news, n, read_usize(G_CLUB_COMP_TABLE).wrapping_add((e as usize).wrapping_mul(0x6b)));
        }
    };
    match status as u8 {
        5 => {
            if (league.upper_division_id() as usize) as i32 == -1 {
                nation(news, n, league.competition().nation().addr());
            } else {
                exe::news_send_to_division_managers(news, n, league.competition().addr());
            }
        }
        0 => {
            exe::news_send_to_division_managers(news, n, league.competition().addr());
            other(0x1c);
        }
        7 | 6 | 0xfe => {
            exe::news_send_to_club_manager(news, n, club);
        }
        1 | 2 => {
            exe::news_send_to_division_managers(news, n, league.competition().addr());
        }
        3 => {
            exe::news_send_to_division_managers(news, n, league.competition().addr());
            other(0x20);
        }
        _ => {}
    }
    1
}

// simplification: virtual +0x44 appends records and the news goes to the managers’ inboxes (they allocate): original only; the
// copy takes the branch.
// Inside the comparer of 0x684cd0 (STATUS_ANNOUNCE_RECORD), stores on its tape what the announcement wrote to the regions.

/// The dry run of the comparer of `0x685d30`.
///
/// The copy runs without the competition statistic (`0x49f570`, which finds or creates the players' records and cannot
/// run twice).
pub(crate) static LEAGUE_ADD_RESULT_DRY_RUN: AtomicBool = AtomicBool::new(false);

/// Adds a result to the league: stamps today's date on the league; a stage fixture goes to its stage; otherwise adds
/// the fixture to the table (`exe::league_apply_result_to_table`) and to the competition stats
/// (`exe::comp_stats_record_match_result`) and returns 1.
///
/// Ret 8: `fixture.add_result(match)`: stores the day's date in `league::LAST_RESULT_DATE` (`exe::date_operator_3d` of
/// `G_CURRENT_DATE`); league with stages (`stage_index()` = 0xff) and a fixture of a stage (`fixture::STAGE_INDEX` ≠
/// 0xff): the stage (`stages()` + 4 × stage, up to `current_stage()`) receives the fixture through virtual
/// `comp_vtable::ADD_RESULT` and what it returns is returned; stage out of range: the boxes and 0. Fixture of another
/// stage (`fixture::STAGE_INDEX` ≠ `stage_index()`, which is not 0xfe): 0. Otherwise adds the fixture to the table
/// (`exe::league_apply_result_to_table` on table `table()` with the `club_count()` rows, marking) and, outside stages
/// 0xfe (`stage_index::SPECIAL_STAGE`), with the competition (`G_COMP_OBJECT_TABLE` by the id of `competition()`) and
/// its stats (+8) and with the match: `exe::comp_stats_record_match_result` with the points of both; returns 1. Null
/// fixture: the box and −1.
/// Original: `League::add_result` (0x685d30, league.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn league_add_result(league: usize, p: usize, game: usize) -> i32 {
    let p = Fixture(p);
    let league = League(league);
    if p.is_null() {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> i32>(IDX_LEAGUE_ADD_RESULT)(league.addr(), p.addr(), game);
    }
    let mut local = [0u8; 8];
    exe::date_operator_3d(league.addr() + league::LAST_RESULT_DATE, (local.as_mut_ptr()) as *mut u8 as usize, G_CURRENT_DATE);
    let stage_of_league = league.stage_index();
    let stage = p.stage_index();
    if stage_of_league == 0xff && stage != 0xff {
        let k = stage as i8 as i32;
        if k > league.current_stage() as i32 || k < 0 {
            // the stage’s message boxes outside the range (lines 0xc8e and 0xc8f): the original (the date is already written, the same)
            return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> i32>(IDX_LEAGUE_ADD_RESULT)(league.addr(), p.addr(), game);
        }
        let filha = read_usize(league.stages().wrapping_add(4 * k as usize));
        let add: unsafe extern "thiscall" fn(usize, usize, usize) -> i32 = core::mem::transmute(read_usize(read_usize(filha) + comp_vtable::ADD_RESULT));
        return add(filha, p.addr(), game);
    }
    if stage_of_league != stage && stage_of_league != 0xfe {
        return 0;
    }
    let (mut pts_casa, mut pts_fora) = (0i16, 0i16);
    // simplification: the original pushes +0x3e with the high part of ecx (leftover from the previous call); 0x685f60 reads only the short
    covered!("cpw_11 result applied");
    exe::league_apply_result_to_table(league, p, league.table(), league.club_count() as u16 as u32, 1, (&mut pts_casa) as *mut i16 as usize, (&mut pts_fora) as *mut i16 as usize); // cpw11: a tabela
    if league.stage_index() == stage_index::SPECIAL_STAGE || read_usize(G_COMP_OBJECT_TABLE) == 0 {
        return 1;
    }
    let comp = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(league.competition().addr()).wrapping_mul(4)));
    if comp == 0 || read_usize(comp + 8) == 0 || game == 0 {
        return 1;
    }
    if !LEAGUE_ADD_RESULT_DRY_RUN.load(Relaxed) {
        // simplification: the original pushes the points as dwords of the locals with an uninitialized high part; 0x49f570 reads the shorts
        exe::comp_stats_record_match_result(read_usize(comp + 8), p, game, pts_casa as i32, pts_fora as i32);
    }
    1
}

/// Allocates and returns a copy of the table rows, with new head-to-head and form buffers; sets `games_per_club()` when
/// it is −1.
///
/// Ret 0: `League::copy_table()`: no table (`table()`) or no rows (`club_count()`): 0. With `games_per_club()` = −1,
/// writes (rows − 1) × `legs()` into it (16 bits; more than 50 rows: the box at line 0x1059 first, as in the original).
/// Allocates the rows (`exe::crt_malloc`, 0x3b each; out of memory: box 0x1061 and 0) and copies them; with
/// head-to-head on the first row (+5), each row gets a new vector of (rows − 1) × 14 bytes with a copy of its own; with
/// form (+9), one of `games_per_club()` bytes. Returns the copy. The vector allocations are not checked (as in the
/// original).
/// Original: `League::copy_table` (0x687710, league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn copy_table(league: usize) -> usize {
    let league = League(league);
    let table = league.table();
    let n = league.club_count();
    if table == 0 || n == 0 {
        return 0;
    }
    if league.games_per_club() == -1 && n > 0 {
        if n > 0x32 {
            return original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_COPY_TABLE)(league.addr()); // simplification: the box, original only
        }
        let v = n.wrapping_sub(1).wrapping_mul(league.legs());
        league.set_games_per_club(v);
    }
    let row = |i: i32| (i as usize).wrapping_mul(0x3b);
    let c = exe::crt_malloc((n as i32 * 0x3b) as u32);
    if c == 0 {
        show_error_at(0x9f15f4, 0x1061);
        return 0;
    }
    core::ptr::copy_nonoverlapping(table as *const u8, c as *mut u8, n as i32 as usize * 0x3b);
    if read_usize(table + 5) != 0 && n > 0 {
        let mut i: i16 = 0;
        while i < league.club_count() {
            let m = league.club_count() as i32;
            let h = exe::crt_malloc((14 * m - 14) as u32);
            write_usize(c + row(i as i32) + 5, h);
            let of = read_usize(league.table() + row(i as i32) + 5);
            core::ptr::copy_nonoverlapping(of as *const u8, h as *mut u8, (14 * league.club_count() as i32 - 14) as usize);
            i += 1;
        }
    }
    if read_usize(league.table() + 9) != 0 && league.club_count() > 0 {
        let mut i: i16 = 0;
        while i < league.club_count() {
            let t = league.games_per_club() as i32;
            let h = exe::crt_malloc(t as u32);
            write_usize(c + row(i as i32) + 9, h);
            let of = read_usize(league.table() + row(i as i32) + 9);
            core::ptr::copy_nonoverlapping(of as *const u8, h as *mut u8, league.games_per_club() as i32 as usize);
            i += 1;
        }
    }
    c
}

/// Sorts the table's clubs by strength and stores each one's predicted position in the competition stats.
///
/// Ret 0: `League::set_predicted_positions()`: without stats (`comp_stats()`): the box (line 0x189e). Copies the clubs
/// of the `club_count()` table rows (`table()`, `league_table_row::SIZE` each) into a vector (`exe::crt_malloc`; out of
/// memory: box 0x18a6, without freeing), sorts by strength (`exe::comp_util_sort_clubs_by_strength`) and stores
/// position i + 1 of each non-null club in the stats (`exe::comp_stats_set_club_final_position`); frees the vector
/// (`exe::crt_free`).
/// Original: `League::set_predicted_positions` (0x68a850, league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn league_set_predicted_positions(league: usize) {
    let league = League(league);
    if league.comp_stats() == 0 {
        return show_error_at(0x9f15f4, 0x189e);
    }
    let v = exe::crt_malloc(((league.club_count() as i32) << 2) as u32);
    if v == 0 {
        return show_error_at(0x9f15f4, 0x18a6);
    }
    let mut i = 0i32;
    while i < league.club_count() as i32 {
        write_u32(v + 4 * i as usize, read_usize(league.table() + league_table_row::SIZE * i as usize) as u32);
        i += 1;
    }
    exe::comp_util_sort_clubs_by_strength(v, league.club_count() as i32);
    let mut i = 0i32;
    while i < league.club_count() as i32 {
        let c = read_usize(v + 4 * i as usize);
        if c != 0 {
            // simplification: the original pushes ecx with the position in the low byte (the rest is leftover); 0x4a2540 reads the byte
            exe::comp_stats_set_club_final_position(league.comp_stats(), c, (i as u8).wrapping_add(1) as u32);
        }
        i += 1;
    }
    exe::crt_free(v);
}

/// Returns entry i (0xe bytes: club, points per game, points, projected points, games) of the table projection buffer.
pub(crate) unsafe fn projection_entry(buf: usize, i: i32) -> usize {
    buf.wrapping_add((i as isize * 0xe) as usize)
}

/// qsort comparator of projection entries by club reputation (`club::REPUTATION`), highest first
/// Original: `compare_projection_by_reputation_desc` (0x68afb0, league.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_projection_by_reputation_desc(a: usize, b: usize) -> i32 {
    let ra = read_i16(read_usize(a) + club::REPUTATION) as i32;
    let rb = read_i16(read_usize(b) + club::REPUTATION) as i32;
    rb - ra // the club with the highest reputation first
}

/// qsort comparator of projection entries by projected points (+0xa), highest first
/// Original: `compare_projection_by_points_desc` (0x68afd0, league.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_projection_by_points_desc(a: usize, b: usize) -> i32 {
    read_i16(b + league_projection::PROJECTED_POINTS) as i32 - read_i16(a + league_projection::PROJECTED_POINTS) as i32 // most points first
}

/// Index of the club's first entry in the sorted projection, or None.
pub(crate) unsafe fn projection_position(buf: usize, n: i16, club: usize) -> Option<i32> {
    (0..n as i32).find(|&i| read_usize(projection_entry(buf, i)) == club)
}

/// Multiplies the entry's points per game (+4, float) by factor and recomputes its projected points (+0xa); returns the
/// remaining games.
pub(crate) unsafe fn apply_form_factor(this: League, e: usize, factor: f64) -> i32 {
    let r = read_f32(e + league_projection::POINTS_PER_GAME) as f64 * factor;
    write_f32(e + league_projection::POINTS_PER_GAME, r as f32); // fst: stores the float and continues with the 53-bit one
    let left = this.games_per_club() as i32 - read_i16(e + league_projection::PLAYED) as i32;
    write_i16(e + league_projection::PROJECTED_POINTS, (ftol(left as f64 * r) as i16).wrapping_add(read_i16(e + league_projection::POINTS)));
    left
}

/// Projects each table club's end-of-season points (in the first third of the season, by a reputation scale), sorts
/// them and hands the club's positions in normal, better (× 1.5) and worse (× 0.444) form to virtual
/// `comp_vtable::SET_CLUB_PROGRESS`.
///
/// Ret 4; virtual +0x84 of the leagues, called only by Comp_Stats `exe::comp_stats_get_club_projected_positions`, which
/// the board uses in the weekly confidence 0x697c40 and in the reason for the manager leaving `get_departure_reason`:
/// `club.update_projected_positions()`: with the club in the table (`table()`, `league_table_row::SIZE` bytes, count
/// `club_count()`: +0 the club, `league_table_row::PLAYED` the matches, `league_table_row::POINTS` the points), builds
/// in a block (`exe::crt_operator_new`) one 0xe-byte entry per row with the points per match
/// (`league_projection::POINTS_PER_GAME`); in the first third of the season (club's matches × 3 < `games_per_club()`),
/// replaces them with a scale by reputation (0.83 × `win_points()` (`rdata::F64_0_83`) for the first down to 0.23 ×
/// `win_points()` (`rdata::F64_0_23`) below the last). Projects the end-of-season points
/// (`league_projection::PROJECTED_POINTS`), sorts and finds the club's position in normal, better (× 1.5,
/// `rdata::F64_1_5`) and worse (× 0.444, `rdata::F64_0_4444444444`) form; frees the block and hands the three to
/// virtual `comp_vtable::SET_CLUB_PROGRESS` (`set_club_progress(club, competition stage_index(), normal, better,
/// worse)`, which writes bytes +0x73..+0x75 of the club's record in Comp_Stats). Returns 1; outside the table, 0.
/// Original: `League::update_projected_positions` (0x68ac40, league.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn update_projected_positions(this: usize, club: usize) -> u32 {
    let this = League(this);
    let n = this.club_count();
    let table = this.table();
    let row = LeagueTableRow(match (0..n as i32).map(|i| table.wrapping_add((i as isize * (league_table_row::SIZE as isize)) as usize)).find(|&l| read_usize(l) == club) {
        Some(l) => l,
        None => return 0,
    });
    if row.is_null() || n == 0 {
        return 0;
    }
    let buf = exe::crt_operator_new((n as i32 * 0xe) as u32);
    let mut i: i16 = 0;
    while i < this.club_count() {
        let l = this.table().wrapping_add((i as isize * (league_table_row::SIZE as isize)) as usize);
        let e = projection_entry(buf, i as i32);
        write_usize(e, read_usize(l));
        write_i16(e + league_projection::POINTS, read_i16(l + league_table_row::POINTS));
        let games = read_i8(l + league_table_row::PLAYED);
        write_i16(e + league_projection::PLAYED, games as i16);
        write_f32(e + league_projection::POINTS_PER_GAME, if games > 0 { (read_i16(l + league_table_row::POINTS) as f64 / games as f64) as f32 } else { 0.0 });
        i += 1;
    }
    // the original’s three locals (dwords): the low byte of each becomes the position; the high bytes are leftovers
    // in them (float A and the row count on the reputation scale, the club’s remaining matches in the third)
    let (mut normal, mut melhor, mut pior) = (0u32, 0u32, 0u32);
    if (row.played() as i32) * 3 < this.games_per_club() as i32 {
        covered!("cpr_11 projection by injury_word_0c");
        let n = this.club_count();
        exe::crt_qsort(buf, n as i32 as u32, 0xe, 0x68afb0);
        let c = (this.win_points() as i8) as f64;
        let a = (c * read_f64(rdata::F64_0_83)) as f32; // fstp into the local: the first rounds to float;
        normal = a.to_bits();
        let b = c * read_f64(rdata::F64_0_23);
        if n > 0 {
            let d = a as f64 - b;
            let step_towards = d / n as f64;
            for k in 0..n as i32 {
                write_f32(projection_entry(buf, k) + 4, (a as f64 - step_towards * k as f64) as f32); // from largest to smallest
            }
            melhor = n as i32 as u32;
        }
    }
    let mut i: i16 = 0;
    while i < this.club_count() {
        let e = projection_entry(buf, i as i32);
        let left = this.games_per_club() as i32 - read_i16(e + league_projection::PLAYED) as i32;
        pior = left as u32;
        write_i16(e + league_projection::PROJECTED_POINTS, (ftol(left as f64 * read_f32(e + league_projection::POINTS_PER_GAME) as f64) as i16).wrapping_add(read_i16(e + league_projection::POINTS)));
        i += 1;
    }
    exe::crt_qsort(buf, this.club_count() as i32 as u32, 0xe, 0x68afd0);
    if let Some(k) = projection_position(buf, this.club_count(), club) {
        normal = (normal & !0xff) | (k as u8).wrapping_add(1) as u32; // the position in normal form
        pior = apply_form_factor(this, projection_entry(buf, k), read_f64(rdata::F64_1_5)) as u32;
    }
    exe::crt_qsort(buf, this.club_count() as i32 as u32, 0xe, 0x68afd0);
    if let Some(k) = projection_position(buf, this.club_count(), club) {
        melhor = (melhor & !0xff) | (k as u8).wrapping_add(1) as u32;
        pior = apply_form_factor(this, projection_entry(buf, k), read_f64(rdata::F64_0_4444444444)) as u32;
    }
    exe::crt_qsort(buf, this.club_count() as i32 as u32, 0xe, 0x68afd0);
    for k in 0..this.club_count() as i32 {
        if read_usize(projection_entry(buf, k)) == club {
            pior = (pior & !0xff) | (k as u8).wrapping_add(1) as u32; // the last one (the club appears only once)
        }
    }
    exe::crt_operator_delete(buf);
    covered!("cpr_11 projection");
    // simplification: the high bytes the original leaves in the three locals outside the reputation scale are stack leftovers
    // (here, zero); the 69 versions of virtual +0x68 only read the low byte of each argument (or pass the dwords on to
    // another one that reads)
    let progresso: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) =
        core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::SET_CLUB_PROGRESS));
    let competition = (melhor & !0xff) | this.stage_index() as u32; // dl over the edx of the second local
    progresso(this.addr(), club, competition, normal, melhor, pior);
    1
}

/// League table rows: 0x3b bytes.
///
/// +0 club, +4 position, +5 head-to-head records of 14 bytes, +9 position history, +0x11/+0x1f/+0x2d the totals, home
/// and away, +0x37 status, +0x38 mark, +0x39, +0x3a clubs.
pub(crate) const TABLE_ROW_SIZE: usize = 0x3b;

/// True if tiebreak rule 4 (head-to-head) appears among the league's 4 tiebreak rules (+0xc5) before the first 0.
pub(crate) unsafe fn uses_head_to_head_rule(this: League) -> bool {
    for k in 0..4 {
        match read_u8(this.addr() + league::TIEBREAKS + k) {
            0 => return false,
            4 => return true,
            _ => {}
        }
    }
    false
}

/// Puts the club in table row i: totals cleared, status 0xff, mark, total rounds (`games_per_club()`) if still −1,
/// position history and, under rule 4, the head-to-head entries (the last row fills every row's opponents).
///
/// Ret 0x10: `i.init_table_row(club, mark, rows)`: puts the club in row `i` (char) of table `rows` (null: the league's,
/// `table()`): a club with status `professional_status()` = 3 (`professional_status::AMATEUR`) changes to 2
/// (`professional_status::SEMI_PROFESSIONAL`); total rounds `games_per_club()` still −1 becomes (clubs `club_count()` −
/// 1) × legs `legs()` (more than 50 clubs: box and continues); the row gets the club, position `i`, the three totals
/// blocks zeroed, the mark (`set_mark`), status 0xff (`table_status::UNDECIDED`), 1 in `set_best_possible_position` and
/// the clubs (`set_worst_possible_position`); with legs, the position history (`set_position_history`,
/// `games_per_club()` bytes; out of memory, box and end); with rule 4 (`uses_head_to_head_rule`), the head-to-head
/// entries (`set_h2h_entries`, 14 bytes per opponent, zeroed). On the last row with head-to-head, each row receives the
/// opponents in table order.
/// Original: `League::init_table_row` (0x687430, league.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn init_table_row(this: usize, i: u32, club: usize, mark: u32, rows: usize) {
    let this = League(this);
    let club = Club(club);
    if !club.is_null() && club.professional_status() == professional_status::AMATEUR {
        club.set_professional_status(professional_status::SEMI_PROFESSIONAL);
    }
    if this.games_per_club() == -1 && this.club_count() > 0 {
        if this.club_count() > 0x32 {
            show_error_at(0x9f15f4, 0xfe9); // more than 50 clubs: the box, and it continues
        }
        let total = (this.club_count() - 1).wrapping_mul(this.legs());
        this.set_games_per_club(total);
    }
    let table = if rows == 0 { this.table() } else { rows };
    let idx = i as u8 as i8 as isize;
    let l = LeagueTableRow(table.wrapping_add((idx * TABLE_ROW_SIZE as isize) as usize));
    write_usize(l.addr(), club.addr());
    l.set_position((i as u8) as i8);
    for k in 0..3 {
        let e = l.addr() + league_table_row::PLAYED + 14 * k;
        write_usize(e - 4, 0);
        core::ptr::write_bytes(e as *mut u8, 0, 4);
        core::ptr::write_bytes((e + 4) as *mut u8, 0, 6);
    }
    l.set_mark(mark as u8);
    l.set_status(table_status::UNDECIDED);
    l.set_best_possible_position(1);
    l.set_worst_possible_position(this.club_count() as u8);
    l.set_h2h_entries(0);
    if this.legs() > 0 {
        let p = exe::crt_malloc(this.games_per_club() as i32 as u32);
        l.set_position_history(p);
        if p == 0 {
            show_error_at(0x9f15f4, 0x100c);
            return;
        }
    } else {
        l.set_position_history(0);
    }
    if uses_head_to_head_rule(this) {
        let n = this.club_count() as i32;
        l.set_h2h_entries(exe::crt_malloc((n * 14 - 14) as u32));
        // the fixtures zeroed (+4..+0xd of each; the original writes +4 twice, without checking memory)
        let mut k = 0i32;
        while k < this.club_count() as i32 - 1 {
            let e = l.h2h_entries() + 14 * k as usize;
            core::ptr::write_bytes((e + 4) as *mut u8, 0, 10);
            k += 1;
        }
    }
    let n = this.club_count() as isize;
    if idx == n - 1 && l.h2h_entries() != 0 {
        covered!("cpm_2 head to head");
        let mut a = 0isize;
        while a < this.club_count() as isize {
            let mut k = 0usize;
            let mut j = 0isize;
            while j < this.club_count() as isize {
                if a != j {
                    let p = read_usize(table + a as usize * TABLE_ROW_SIZE + 5);
                    write_usize(p + 14 * k, read_usize(table + j as usize * TABLE_ROW_SIZE)); // the opposing club
                    k += 1;
                }
                j += 1;
            }
            a += 1;
        }
    }
}

/// a new array with the table clubs and the count; without memory, error box and count 0
/// Original: `League::get_club_array` (0x687a20, league.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn league_get_club_array(this: usize, out: *mut usize, n: *mut i16) {
    let this = League(this);
    let p = exe::crt_malloc((this.club_count() as i32 * 4) as u32);
    *out = p;
    if p == 0 {
        show_error_at(0x9f15f4, 0x10cc);
        *n = 0;
        return;
    }
    covered!("cpm_2 clubs array");
    let mut i = 0i16;
    while i < this.club_count() {
        write_usize(p + 4 * i as usize, read_usize(this.table() + i as usize * TABLE_ROW_SIZE));
        i += 1;
    }
    *n = this.club_count(); // the count
}

/// each club still in the league division stores last season's division (`club::LAST_DIVISION`) and the final position
/// (`set_last_position`)
/// Original: `League::record_final_positions` (0x68aa30, league.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn record_final_positions(this: usize) {
    let this = League(this);
    let mut i = 0i16;
    while i < this.club_count() {
        let division = this.competition().addr();
        let c = Club(read_usize(this.table() + i as usize * TABLE_ROW_SIZE));
        if c.division().addr() == division {
            covered!("cpm_2 final position");
            write_usize(c.addr() + club::LAST_DIVISION, division);
            c.set_last_position((i as u8).wrapping_add(1)); // the final position
        }
        i += 1;
    }
}

/// Adds 1 to the byte at address a.
pub(crate) unsafe fn increment_u8(a: usize) {
    write_u8(a, read_u8(a).wrapping_add(1));
}

/// Adds v to the i16 at address a.
pub(crate) unsafe fn league_add_i16(a: usize, v: i16) {
    write_i16(a, read_i16(a).wrapping_add(v));
}

/// Linhas da tabela: 0x3b bytes; +0 clube, +5 confrontos (entradas de 14), +0x11..+0x1a os totais, +0x1f..+0x28 em
/// casa (jogos, vitórias, empates, derrotas, gols pró, contra, pontos), +0x2d..+0x36 fora.
pub(crate) const RESULT_TABLE_ROW_SIZE: usize = 0x3b;

/// Applies a result to the table: finds the home and away rows and adds the game at home and away (played, goals,
/// win/draw/loss, by extra time and penalties, marking `fixture::HOME_RESULT_MARK`/`fixture::AWAY_RESULT_MARK`), the
/// win points by the `league_vtable::POINTS_FOR_RESULT` virtual and the draw points `draw_points()`; redoes the totals,
/// returns both rows' points and adds the head-to-head (`exe::league_update_head_to_head`).
///
/// Ret 0x18: `fixture.apply_result_to_table(table, n, mark, pts_home*, pts_away*)`: finds the rows of the home side
/// (`home_club()`) and the away side (`away_club()`) among the `n` (short) of the table (stops at the first club that
/// is neither after finding both) and adds the game: matches
/// (`league_table_row::HOME_PLAYED`/`league_table_row::AWAY_PLAYED`), goals
/// (`league_table_row::HOME_GOALS_FOR`/`league_table_row::HOME_GOALS_AGAINST` and
/// `league_table_row::AWAY_GOALS_FOR`/`league_table_row::AWAY_GOALS_AGAINST`: the score of
/// `home_goals_extra_time()`/`away_goals_extra_time()` when ≠ 0xff, otherwise that of `home_goals()`/`away_goals()`).
/// The result from score `home_goals()`/`away_goals()`; level: by extra time (`tie_rules()` &
/// `fixture_tie_rules::EXTRA_TIME`, `home_goals_extra_time()`/`away_goals_extra_time()`; `mark` writes 3
/// (`result_mark::WON_EXTRA_TIME`) in `fixture::HOME_RESULT_MARK`/`fixture::AWAY_RESULT_MARK`) and by penalties (bits
/// 0x20/1, `home_penalties()`/`away_penalties()`; a home win counts as a draw for it; 5 with 0x20
/// (`fixture_tie_rules::SHOOT_OUT_35_YARD`), otherwise 4); a pure draw with the league's draw points (`draw_points()`).
/// The win points through virtual `league_vtable::POINTS_FOR_RESULT` (`points_for_result`) of the type 2 competition
/// (`G_COMP_OBJECT_TABLE`, `league::COMP_TYPE`) or of the league itself, read on each call. Redoes the totals of both
/// rows and returns the points (`league_table_row::POINTS`) in `pts_home`/`pts_away` (0 without the row). With
/// head-to-head on the first row (+5), finds the opponent in each one and adds the tie
/// (`exe::league_update_head_to_head`). No SEH, no random draw, no box, all locals written before being read.
/// Original: `League::apply_result_to_table` (0x685f60, league.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn apply_result_to_table(league: usize, p: usize, table: usize, n: u32, mark: u32, pts_home: *mut i16, pts_away: *mut i16) {
    let p = Fixture(p);
    let league = League(league);
    let mut pontuador = league.addr();
    let id = league.competition().addr();
    if id != 0 && read_usize(G_COMP_OBJECT_TABLE) != 0 {
        let comp = League(read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(id).wrapping_mul(4))));
        if comp.comp_type() == 2 {
            pontuador = comp.addr();
        }
    }
    let n = n as i16;
    let row = |k: i16| table.wrapping_add(RESULT_TABLE_ROW_SIZE.wrapping_mul(k as i32 as usize));
    let (mut casa, mut fora): (i16, i16) = (-1, -1);
    let mut k: i16 = 0;
    if n > 0 {
        loop {
            let c = read_usize(row(k));
            if c == p.home_club().addr() {
                casa = k;
            } else if c == p.away_club().addr() {
                fora = k;
            } else if casa != -1 && fora != -1 {
                break;
            }
            k = k.wrapping_add(1);
            if k >= n {
                break;
            }
        }
    }
    let noventa = || (p.home_goals_extra_time() as u8) != 0xff;
    if casa > -1 {
        let r = row(casa);
        increment_u8(r + league_table_row::HOME_PLAYED);
        let (pro, contra) = if noventa() { (p.home_goals_extra_time(), p.away_goals_extra_time()) } else { (p.home_goals(), p.away_goals()) };
        league_add_i16(r + league_table_row::HOME_GOALS_FOR, pro as i16);
        league_add_i16(r + league_table_row::HOME_GOALS_AGAINST, contra as i16);
    }
    if fora > -1 {
        let r = row(fora);
        increment_u8(r + league_table_row::AWAY_PLAYED);
        let (pro, contra) = if noventa() { (p.away_goals_extra_time(), p.home_goals_extra_time()) } else { (p.away_goals(), p.home_goals()) };
        league_add_i16(r + league_table_row::AWAY_GOALS_FOR, pro as i16);
        league_add_i16(r + league_table_row::AWAY_GOALS_AGAINST, contra as i16);
    }
    let points = |club: usize| -> i16 {
        let f: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(read_usize(read_usize(pontuador) + league_vtable::POINTS_FOR_RESULT));
        f(pontuador, club, p.addr()) as u8 as i8 as i16
    };
    // home team wins: its +0x20 and points (+0x27), defeat (+0x30) for the away team; loses: +0x22 and +0x2e with the points (+0x35)
    let wins_home = |win: usize| {
        if casa > -1 {
            let r = row(casa);
            increment_u8(r + win);
            league_add_i16(r + league_table_row::HOME_POINTS, points(p.home_club().addr())); // the home team’s points
        }
        if fora > -1 {
            increment_u8(row(fora) + league_table_row::AWAY_LOST);
        }
    };
    let wins_away = |win: usize| {
        if casa > -1 {
            increment_u8(row(casa) + league_table_row::HOME_LOST);
        }
        if fora > -1 {
            let r = row(fora);
            increment_u8(r + win);
            league_add_i16(r + league_table_row::AWAY_POINTS, points(p.away_club().addr()));
        }
    };
    let (gc, gf) = (p.home_goals(), p.away_goals());
    let flags = p.tie_rules();
    let penalties = if (p.tie_rules() as u8) & (fixture_tie_rules::SHOOT_OUT_35_YARD as u8) != 0 { 5u8 } else { 4 };
    if gc > gf {
        wins_home(0x20);
    } else if gc < gf {
        wins_away(0x2e);
    } else if flags & fixture_tie_rules::EXTRA_TIME != 0 && p.home_goals_extra_time() > p.away_goals_extra_time() {
        wins_home(0x20);
        if mark != 0 {
            p.set_home_result_mark((result_mark::WON_EXTRA_TIME) as i8);
        }
    } else if flags & 2 != 0 && p.home_goals_extra_time() < p.away_goals_extra_time() {
        wins_away(0x2e);
        if mark != 0 {
            p.set_away_result_mark((result_mark::WON_EXTRA_TIME) as i8);
        }
    } else if flags & 0x21 != 0 && p.home_penalties() > p.away_penalties() {
        wins_home(0x21);
        if mark != 0 {
            p.set_home_result_mark((penalties) as i8);
        }
    } else if flags & 0x21 != 0 && p.home_penalties() < p.away_penalties() {
        wins_away(0x2f);
        if mark != 0 {
            p.set_away_result_mark((penalties) as i8);
        }
    } else if flags & 0x21 == 0 {
        let empate = (league.draw_points() as i8) as i16; // o empate puro
        if casa > -1 {
            increment_u8(row(casa) + league_table_row::HOME_DRAWN);
            league_add_i16(row(casa) + league_table_row::HOME_POINTS, empate);
        }
        if fora > -1 {
            increment_u8(row(fora) + league_table_row::AWAY_DRAWN);
            league_add_i16(row(fora) + league_table_row::AWAY_POINTS, empate);
        }
    }
    let totais = |r: usize| -> i16 {
        for (t, f, c) in [(0x11usize, 0x2d, 0x1f), (0x12, 0x2e, 0x20), (0x13, 0x2f, 0x21), (0x14, 0x30, 0x22)] {
            write_u8(r + t, read_u8(r + f).wrapping_add(read_u8(r + c)));
        }
        for (t, f, c) in [(0x15usize, 0x31, 0x23), (0x17, 0x33, 0x25), (0x19, 0x35, 0x27)] {
            let v = read_i16(r + f).wrapping_add(read_i16(r + c));
            write_i16(r + t, v);
        }
        read_i16(r + league_table_row::POINTS)
    };
    for (k, pts) in [(casa, pts_home), (fora, pts_away)] {
        let v = if k > -1 { totais(row(k)) } else { 0 };
        if !pts.is_null() {
            core::ptr::write_unaligned(pts, v);
        }
    }
    if read_usize(table + 5) != 0 {
        let m = n as i32 - 1;
        let (mut ia, mut ib) = (-1i32, -1i32);
        let entry = |linha_: i16, k: i16| read_usize(read_usize(row(linha_) + 5).wrapping_add(14 * k as i32 as usize));
        let mut k: i16 = 0;
        if m > 0 {
            loop {
                if casa > -1 && entry(casa, k) == p.away_club().addr() {
                    ia = k as i32;
                    if ib > -1 {
                        break;
                    }
                }
                if fora > -1 && entry(fora, k) == p.home_club().addr() {
                    ib = k as i32;
                    if ia > -1 {
                        break;
                    }
                }
                k = k.wrapping_add(1);
                if k as i32 >= m {
                    break;
                }
            }
        }
        exe::league_update_head_to_head(league, table, casa as i32, ia, gc, gf);
        exe::league_update_head_to_head(league, table, fora as i32, ib, gf, gc);
    }
}

/// Memory regions `apply_result_to_table` writes: the n table rows, each row's head-to-head records, match +0x3d/+0x3e and the two
/// returned points.
pub(crate) unsafe fn result_update_regions(p: Fixture, table: usize, n: u32, pts_home: *mut i16, pts_away: *mut i16) -> Vec<(usize, usize)> {
    let n = (n as i16).max(0) as usize;
    let mut reg = vec![(p.addr() + fixture::HOME_RESULT_MARK, 2)];
    if table != 0 && n > 0 {
        reg.push((table, RESULT_TABLE_ROW_SIZE * n));
        if read_usize(table + 5) != 0 && n > 1 {
            for i in 0..n {
                let h = read_usize(table + RESULT_TABLE_ROW_SIZE * i + 5);
                if h != 0 {
                    reg.push((h, 14 * (n - 1)));
                }
            }
        }
    }
    for pts in [pts_home, pts_away] {
        if !pts.is_null() {
            reg.push((pts as usize, 2));
        }
    }
    reg
}

/// Returns the points for a result: the win points (`win_points()`) to the winner, 0 to the loser; when level, by extra
/// time (`match_rules()` bit 2), by penalties (bits 0x21: the draw points `draw_points()` to the winner) or the draw
/// points.
///
/// Ret 8: `club.points_for_result(fixture)`: the win points (`win_points()`) if the club won (home side `home_club()`
/// with `home_goals()` > `away_goals()`, away side `away_club()` with fewer), otherwise 0; level: by extra time
/// (`match_rules()` bit 2, `home_goals_extra_time()`/`away_goals_extra_time()`, the win points), by penalties (bits
/// 0x21, `home_penalties()`/`away_penalties()`: the draw points `draw_points()` to the winner, 0 to the loser) or the
/// draw points. Only al is written: the 3 high bytes of eax are those of the fixture.
/// Original: `League::points_for_result` (0x68aad0, league.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn points_for_result(league: usize, club: usize, p: usize) -> u32 {
    let p = Fixture(p);
    let league = League(league);
    let (casa, fora) = (p.home_club(), p.away_club());
    let won = |a: i8, b: i8| (club == casa.addr() && a > b) || (club == fora.addr() && a < b);
    let flags = league.match_rules() as u8;
    let v = if p.home_goals() != p.away_goals() {
        if won(p.home_goals(), p.away_goals()) { league.win_points() } else { 0 } // the points for a win
    } else if flags & 2 != 0 && p.home_goals_extra_time() != p.away_goals_extra_time() {
        if won(p.home_goals_extra_time(), p.away_goals_extra_time()) { league.win_points() } else { 0 }
    } else if flags & 0x21 != 0 && p.home_penalties() != p.away_penalties() {
        if won(p.home_penalties(), p.away_penalties()) { league.draw_points() } else { 0 }
    } else {
        league.draw_points()
    };
    (p.addr() as u32 & 0xffff_ff00) | v as u32
}

compare_value!(compare_points_for_result, points_for_result, IDX_POINTS_FOR_RESULT, (clube: usize, p: usize) -> u32);

/// qsort callback comparing table rows with head-to-head: the own compare, the points, the club id with no games, the
/// stored head-to-head position (+4) and the name.
///
/// `league_compare_table_rows_h2h(a, b)` (qsort callback of `sort_table` under rule 4 and of
/// `ita_league_sort_clubs_level_on_points` of the Italian Serie B): an empty row ties; the own comparison
/// `G_LEAGUE_SORT_CUSTOM_COMPARE_FUNC` when it does not give 0 (called again); the points (`league_table_row::POINTS`
/// of column `G_LEAGUE_SORT_CURRENT_LEAGUE` → `league::TABLE_COLUMN`, b − a); both without matches
/// (`league_table_row::PLAYED`): the club id (a − b); the head-to-head position stored in +4 (b − a); finally the
/// clubs' name (+4, strcmp: −1, 0 or 1).
/// Original: `league_compare_table_rows_h2h` (0x683410, league.cpp, cdecl).
pub(crate) unsafe extern "C" fn league_compare_table_rows_h2h(a: usize, b: usize) -> i32 {
    let col = read_usize(read_usize(G_LEAGUE_SORT_CURRENT_LEAGUE) + league::TABLE_COLUMN);
    if read_usize(a) == 0 || read_usize(b) == 0 {
        return 0;
    }
    if read_usize(G_LEAGUE_SORT_CUSTOM_COMPARE_FUNC) != 0 {
        let f: unsafe extern "C" fn(usize, usize) -> i32 = core::mem::transmute(read_usize(G_LEAGUE_SORT_CUSTOM_COMPARE_FUNC));
        if f(a, b) != 0 {
            let f: unsafe extern "C" fn(usize, usize) -> i32 = core::mem::transmute(read_usize(G_LEAGUE_SORT_CUSTOM_COMPARE_FUNC));
            return f(a, b);
        }
    }
    let o = LeagueTableRow(col.wrapping_mul(14));
    let (pa, pb) = (read_i16(a + o.addr() + league_table_row::POINTS), read_i16(b + o.addr() + league_table_row::POINTS));
    if pa != pb {
        return pb as i32 - pa as i32;
    }
    if read_u8(a + o.addr() + league_table_row::PLAYED) == 0 && *((b + o.addr() + league_table_row::PLAYED) as *const u8) == 0 {
        return (read_usize(read_usize(a)) as i32).wrapping_sub(read_usize(read_usize(b)) as i32);
    }
    let dif = read_i8(b + 4) as i32 - read_i8(a + 4) as i32; // the tie’s position
    if dif != 0 {
        return dif;
    }
    let (mut x, mut y) = (read_usize(a) + 4, read_usize(b) + 4);
    loop {
        let (cx, cy) = (read_u8(x), read_u8(y));
        if cx != cy {
            return if cx < cy { -1 } else { 1 };
        }
        if cx == 0 {
            return 0;
        }
        x += 1;
        y += 1;
    }
}

/// bsearch callback: the year (+2, unsigned) and the day (+0): −1, 0 or 1
/// Original: `league_compare_dates` (0x685cf0, league.cpp, cdecl).
pub(crate) unsafe extern "C" fn league_compare_dates(a: usize, b: usize) -> i32 {
    let (ya, yb) = (read_i16(a + 2) as u16, read_i16(b + 2) as u16);
    if ya < yb { // o ano antes
        return -1;
    }
    if ya > yb {
        return 1;
    }
    let (da, db) = (read_i16(a), read_i16(b));
    if da < db {
        -1
    } else {
        (da > db) as i32
    }
}

/// the club row in the given rows (null: the league table), or 0
/// Original: `League::find_club_row` (0x68ab70, league.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn find_club_row(league: usize, club: usize, rows: usize) -> usize {
    let league = League(league);
    let t = if rows == 0 { league.table() } else { rows };
    let n = league.club_count();
    let mut i: i16 = 0;
    if n > 0 {
        loop {
            let r = t.wrapping_add(0x3b * i as i32 as usize);
            if read_usize(r) == club { // the club’s row
                return r;
            }
            i = i.wrapping_add(1);
            if i >= n {
                break;
            }
        }
    }
    0
}

compare_value!(compare_find_club_row, find_club_row, IDX_FIND_CLUB_ROW, (clube: usize, linhas: usize) -> usize);

/// 1 when the club has a row in the league table
/// Original: `League::contains_club` (0x68abd0, league.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn contains_club(league: usize, club: usize) -> u32 {
    let league = League(league);
    let t = league.table();
    let n = league.club_count();
    let mut i: i16 = 0;
    if n > 0 {
        loop {
            let r = t.wrapping_add(0x3b * i as i32 as usize);
            if read_usize(r) == club {
                return (r != 0) as u32; // the league has the club
            }
            i = i.wrapping_add(1);
            if i >= n {
                break;
            }
        }
    }
    0
}

compare_value!(compare_contains_club, contains_club, IDX_CONTAINS_CLUB, (clube: usize) -> u32);

/// League table rows: 0x3b bytes each.
///
/// `+0` club, `+4` position, `+5` head-to-head results (14-byte entries), `+9` position history (one byte per round),
/// `+0x11` matches and, every 14 bytes per column (`+0xad`), the points at `+0x19`.
pub(crate) const LEAGUE_TABLE_ROW_SIZE: usize = 0x3b;

/// Sorts the table by the rules (`league::TIEBREAKS`): rule 6 only sorts (`league_compare_table_rows`); rule 4 builds
/// the mini table and, when first, sorts by head-to-head (`league_compare_table_rows_h2h`) and bubbles the ties; stores
/// the position (`set_position`) and the position history (`position_history()`); returns 1.
///
/// `rows.sort_table()`: sorts the table (`rows` null: the league's, `table()`). First fixes total rounds
/// `games_per_club()` (−1: (`club_count()` − 1) × `legs()`, with clubs and `active()`). Rule 6 (`league::TIEBREAKS`,
/// without column `table_column()`) only sorts (qsort `exe::crt_qsort` with `league_compare_table_rows`); rule 4 with
/// head-to-head builds the mini table (`exe::league_rank_by_head_to_head`) and, being the first rule, sorts with
/// `league_compare_table_rows_h2h` and runs a bubble pass over the league rows (`table()`) while there are swaps: a tie
/// on points (or on the own comparison `custom_compare()`) decided by `exe::league_compare_table_rows`; otherwise qsort
/// with `league_compare_table_rows`. Finally (`stage_index()` ≠ `stage_index::SPECIAL_STAGE`, without column) stores
/// the position in `set_position` and in history `position_history()` (round `played()` up to `games_per_club()`).
/// Keeps the league in `G_LEAGUE_SORT_CURRENT_LEAGUE` and the own comparison in `G_LEAGUE_SORT_CUSTOM_COMPARE_FUNC`.
/// Returns 1. No SEH, no random draw.
/// Original: `League::sort_table` (0x6827d0, league.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn sort_table(league: usize, rows: usize) -> u32 {
    let league = League(league);
    let n = || league.club_count();
    let rows = if rows == 0 { league.table() } else { rows };
    if league.games_per_club() == -1 && n() > 0 && league.active() != 0 {
        league.set_games_per_club((n() - 1).wrapping_mul(league.legs()));
    }
    if rows == 0 {
        return 1;
    }
    write_usize(G_LEAGUE_SORT_CURRENT_LEAGUE, league.addr());
    let column = league.table_column();
    let mut so_sort = false;
    let mut tie = false;
    for k in 0..4usize {
        let rule = read_u8(league.addr() + league::TIEBREAKS + k);
        if column == 0 && rule == 6 {
            so_sort = true;
            break;
        }
        if rule == 0 {
            break;
        }
        if rule == 4 && n() != 0 && read_usize(rows + 5) != 0 {
            tie = k == 0;
            exe::league_rank_by_head_to_head(league, rows);
            break;
        }
    }
    if so_sort {
        exe::crt_qsort(rows, (n() as i32) as u32, LEAGUE_TABLE_ROW_SIZE as u32, 0x683210);
    } else {
        write_usize(G_LEAGUE_SORT_CUSTOM_COMPARE_FUNC, league.custom_compare());
        if tie {
            exe::crt_qsort(rows, (n() as i32) as u32, LEAGUE_TABLE_ROW_SIZE as u32, 0x683410);
            loop {
                let mut trocou = false;
                if n() as i32 - 1 <= 0 {
                    break;
                }
                let mut i: i8 = 0;
                loop {
                    let (oa, ob) = (LEAGUE_TABLE_ROW_SIZE * i as i32 as usize, LEAGUE_TABLE_ROW_SIZE * (i as i32 + 1) as usize);
                    let (a, b) = (league.table() + oa, league.table() + ob);
                    let col = read_usize(read_usize(G_LEAGUE_SORT_CURRENT_LEAGUE) + league::TABLE_COLUMN);
                    let decides = read_usize(a) == 0 || read_usize(b) == 0 || is_sort_tie(a, b, col);
                    if decides && exe::league_compare_table_rows(league.table() + oa, league.table() + ob) > 0 {
                        let t = league.table();
                        core::ptr::swap_nonoverlapping((t + oa) as *mut u8, (t + ob) as *mut u8, LEAGUE_TABLE_ROW_SIZE); // the row swap
                        trocou = true;
                    }
                    i = i.wrapping_add(1);
                    if i as i32 >= n() as i32 - 1 {
                        break;
                    }
                }
                if !trocou {
                    break;
                }
            }
        } else {
            exe::crt_qsort(rows, (n() as i32) as u32, LEAGUE_TABLE_ROW_SIZE as u32, 0x683210);
        }
    }
    if league.stage_index() != stage_index::SPECIAL_STAGE && league.table_column() == 0 && n() > 0 {
        let mut c: i8 = 0;
        loop {
            let r = LeagueTableRow(rows + LEAGUE_TABLE_ROW_SIZE * c as i32 as usize);
            r.set_position(c); // the position
            let hist = r.position_history();
            let round = (r.played() as u8) as i8;
            if hist != 0 && round > 0 && round as i16 <= league.games_per_club() {
                write_i8(hist + round as usize - 1, c);
            }
            c = c.wrapping_add(1);
            if c as i16 >= n() {
                break;
            }
        }
    }
    1
}

/// Whether the `sort_table` bubble sort treats rows a and b as tied in `league_compare_table_rows`: custom compare func, else points (+0x19),
/// club ids without games, or head-to-head goals.
pub(crate) unsafe fn is_sort_tie(a: usize, b: usize, col: usize) -> bool {
    let own = read_usize(G_LEAGUE_SORT_CUSTOM_COMPARE_FUNC);
    if own != 0 {
        let f: unsafe extern "C" fn(usize, usize) -> i32 = core::mem::transmute(own);
        if f(a, b) != 0 {
            let f: unsafe extern "C" fn(usize, usize) -> i32 = core::mem::transmute(read_usize(G_LEAGUE_SORT_CUSTOM_COMPARE_FUNC));
            return f(a, b) == 0;
        }
    }
    let o = LeagueTableRow(col.wrapping_mul(14));
    let (pa, pb) = (read_i16(a + o.addr() + league_table_row::POINTS), read_i16(b + o.addr() + league_table_row::POINTS));
    if pa != pb {
        return false;
    }
    if read_u8(a + o.addr() + league_table_row::PLAYED) == 0 && read_u8(b + o.addr() + league_table_row::PLAYED) == 0 {
        let (ca, cb) = (read_usize(a), read_usize(b));
        if ca == 0 || cb == 0 {
            return true;
        }
        return read_i32(ca) == read_i32(cb);
    }
    let m = read_i16(read_usize(G_LEAGUE_SORT_CURRENT_LEAGUE) + league::CLUB_COUNT) as i32 - 1;
    if m <= 0 {
        return true;
    }
    let (h, cb) = (read_usize(a + 5), read_usize(b));
    let mut k: i16 = 0;
    loop {
        let e = h.wrapping_add(14 * k as i32 as usize);
        if read_usize(e) == cb {
            if e == 0 {
                return true;
            }
            return read_u8(e + 5) == read_u8(e + 7) && read_i16(e + 8) == read_i16(e + h2h_entry::GOALS_AGAINST);
        }
        k = k.wrapping_add(1);
        if k as i32 >= m {
            return true;
        }
    }
}

/// Memory regions of the table sort: the rows (requested and league's), total rounds, the two sort globals and each
/// row's position history.
pub(crate) unsafe fn sort_regions(league: League, rows: usize) -> Vec<(usize, usize)> {
    let n = league.club_count().max(0) as usize;
    let rows = if rows == 0 { league.table() } else { rows };
    let mut reg = vec![(league.addr() + league::GAMES_PER_CLUB, 2), (G_LEAGUE_SORT_CURRENT_LEAGUE, 4), (G_LEAGUE_SORT_CUSTOM_COMPARE_FUNC, 4)];
    if rows == 0 {
        return reg;
    }
    reg.push((rows, LEAGUE_TABLE_ROW_SIZE * n));
    let own = league.table();
    if own != 0 && own != rows {
        reg.push((own, LEAGUE_TABLE_ROW_SIZE * n));
    }
    let mut total = league.games_per_club();
    if total == -1 && n > 0 && league.active() != 0 {
        total = (n as i16 - 1).wrapping_mul(league.legs());
    }
    let size = total.clamp(0, 127) as usize;
    if size > 0 {
        for i in 0..n {
            let hist = read_usize(rows + LEAGUE_TABLE_ROW_SIZE * i + 9);
            if hist != 0 {
                reg.push((hist, size));
            }
        }
    }
    reg
}

/// Builds the head-to-head mini table of the clubs level on points with their summed head-to-head results, sorted by
/// `exe::league_compare_h2h_rows`; the position (+4) starts at the club count and drops at each separated pair.
///
/// `rows.build_h2h_table()` (unnamed in the community map): a mini table (calloc `exe::crt_calloc` of `club_count()`
/// rows) with the club of each row and, for each other club with the same points (`league_table_row::POINTS`), the sum
/// of the head-to-head against it (entry of `league_table_row::H2H_ENTRIES`: bytes +4..+7 into +0x11..+0x14, words
/// +8/+0xa/+0xc into +0x15/+0x17/+0x19); sorts with `league_compare_h2h_rows` and writes in +4 of each row the
/// position, which starts at the number of clubs and drops at each pair of the mini table that
/// `exe::league_compare_h2h_rows` separates. Frees the mini table (`exe::crt_free`). Out of memory: the original
/// routine (the league.cpp box). TapeCall through `sort_table` under rule 4.
/// Original: `League::build_h2h_table` (0x682b30, league.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn build_h2h_table(league: usize, rows: usize) {
    let league = League(league);
    let n = || league.club_count();
    let t = exe::crt_calloc((n() as i32) as u32, LEAGUE_TABLE_ROW_SIZE as u32);
    if t == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_BUILD_H2H_TABLE)(league.addr(), rows); // message box: original only
    }
    let row = |base: usize, i: i16| base + LEAGUE_TABLE_ROW_SIZE * i as i32 as usize;
    let mut i: i16 = 0;
    if n() > 0 {
        loop {
            let (ri, mi) = (row(rows, i), row(t, i));
            write_usize(mi, read_usize(ri));
            let mut j: i16 = 0;
            if n() > 0 {
                loop {
                    let rj = LeagueTableRow(row(rows, j));
                    if i != j && read_i16(ri + league_table_row::POINTS) == rj.points() {
                        let m = n() as i32 - 1;
                        let (h, cj) = (read_usize(ri + league_table_row::H2H_ENTRIES), read_usize(rj.addr()));
                        let mut k: i16 = 0;
                        while (k as i32) < m {
                            let e = h + 14 * k as i32 as usize;
                            if read_usize(e) == cj {
                                for (de, para) in [(4, 0x11), (5, 0x12), (6, 0x13), (7, 0x14)] {
                                    write_u8(mi + para, read_u8(mi + para).wrapping_add(read_u8(e + de))); // o confronto somado
                                }
                                for (de, para) in [(8, 0x15), (0xa, 0x17), (0xc, 0x19)] {
                                    let v = read_i16(mi + para).wrapping_add(read_i16(e + de));
                                    write_i16(mi + para, v);
                                }
                                break;
                            }
                            k = k.wrapping_add(1);
                        }
                    }
                    j = j.wrapping_add(1);
                    if j >= n() {
                        break;
                    }
                }
            }
            i = i.wrapping_add(1);
            if i >= n() {
                break;
            }
        }
    }
    exe::crt_qsort(t, (n() as i32) as u32, LEAGUE_TABLE_ROW_SIZE as u32, 0x6834f0);
    let mut pos = n() as u8;
    let mut a: i16 = 0;
    if n() > 0 {
        loop {
            let club = read_usize(row(t, a));
            let mut c: i16 = 0;
            while c < n() {
                if club == read_usize(row(rows, c)) {
                    write_u8(row(rows, c) + 4, pos);
                    break;
                }
                c = c.wrapping_add(1);
            }
            if (a as i32 + 1) < n() as i32 && exe::league_compare_h2h_rows(row(t, a), row(t, a + 1)) != 0 {
                pos = pos.wrapping_sub(1); // the position goes down
            }
            a = a.wrapping_add(1);
            if a >= n() {
                break;
            }
        }
    }
    exe::crt_free(t);
}

/// Frees the +5 and +9 row buffers and the table (null: the league's, then clears `club_count()` and `table()`).
///
/// `table.free_table()`: a null `table` = the league's (`table()`). With clubs (`club_count()` ≥ 1) and a table: frees
/// (`exe::crt_free`) the +5 vectors of each row (if the first row's is not null), then the +9 ones (likewise) and the
/// table; if it is the league's, zeroes `club_count()` and `table()` (`set_club_count(0)`, `set_table(0)`). No SEH, no
/// random draw.
/// Original: `League::free_table` (0x687970, league.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn free_table(league: usize, table: usize) {
    let league = League(league);
    let (t, propria) = if table == 0 { (league.table(), true) } else { (table, false) };
    if league.club_count() < 1 || t == 0 {
        return;
    }
    covered!("league free table");
    for campo in [5usize, 9] {
        if read_usize(t + campo) != 0 {
            let mut i = 0usize;
            while (i as i32) < league.club_count() as i32 {
                let p = read_usize(t + campo + LEAGUE_TABLE_ROW_SIZE * i);
                if p != 0 {
                    exe::crt_free(p);
                }
                i += 1;
            }
        }
    }
    exe::crt_free(t);
    if propria { // the league one: clears the count and the table
        league.set_club_count(0);
        league.set_table(0);
    }
}

/// Comparador: liberar duas vezes soltaria os mesmos blocos duas vezes.
// simplification: original only (like the releases of the 232nd); the proof is the career and the copy’s branch in Rust mode.

/// Returns 1 when every club has played the `games_per_club()` rounds (and the stages, if asked); at the end of the
/// league, the records (`exe::staff_records_add_league_finish_records`), the prize money
/// (`exe::league_pay_prize_money`), the notice (`league_vtable::UPDATE_POSITIONS_AND_STATUS`) and `complete()`.
///
/// `stages.check_complete(warn)`: 0 without a table, closed (`stopped()`) or without `active()`. Outside the single
/// stage (`stage_index()` ≠ `stage_index::MAIN`) already complete (`complete()`): 1. Not complete: fixes total rounds
/// `games_per_club()` (−1) and, if some club has not yet played them all (`league_table_row::PLAYED` <
/// `games_per_club()`), 0; otherwise, in league divisions (`exe::comp_util_is_league_division`) with `round_count()` >
/// 8 and without `tiebreak_matches()`, the end-of-league records (`exe::staff_records_add_league_finish_records`), and
/// without `tiebreak_matches()` the prizes (`exe::league_pay_prize_money`). With `stages` = 1 in the single stage: each
/// stage of `stages()` (0..=`current_stage()`) must be complete (virtual `comp_vtable::CHECK_COMPLETE` with 0, 1) and
/// `current_stage()` ≥ `stage_count()` − 1, otherwise 0. Marks `complete()` (before that, with `warn`, virtual
/// `league_vtable::UPDATE_POSITIONS_AND_STATUS`) and returns 1. No SEH, no random draw.
/// Original: `League::check_complete` (0x689ad0, league.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn league_check_complete(league: usize, stages: u32, warn: u32) -> u8 {
    let league = League(league);
    let table = league.table();
    if table == 0 || league.stopped() != 0 || league.active() == 0 {
        return 0;
    }
    let unica = league.stage_index() == stage_index::MAIN;
    if league.complete() == 0 {
        if league.games_per_club() == -1 && league.club_count() > 0 {
            league.set_games_per_club((league.club_count() - 1).wrapping_mul(league.legs()));
        }
        let rounds = league.games_per_club();
        for i in 0..league.club_count().max(0) as usize {
            if ((read_u8(table + LEAGUE_TABLE_ROW_SIZE * i + league_table_row::PLAYED) as i8) as i16) < rounds { // ainda falta jogar
                return 0;
            }
        }
        covered!("league complete: end of league");
        let id = read_u32(league.competition().addr());
        let mut prizes = true;
        if exe::comp_util_is_league_division((id) as i32) != 0 && league.round_count() > 8 {
            if league.tiebreak_matches() != 0 {
                prizes = false;
            } else {
                exe::staff_records_add_league_finish_records(read_usize(G_STAFF_RECORDS), league);
            }
        }
        if prizes && league.tiebreak_matches() == 0 {
            exe::league_pay_prize_money(league);
        }
    } else if !unica {
        return 1;
    }
    if stages == 1 && league.stage_index() == stage_index::MAIN {
        covered!("league complete: stages");
        let mut k = 0usize;
        if league.current_stage() as i32 >= 0 {
            loop {
                let stage = read_usize(league.stages() + 4 * k);
                let completa: unsafe extern "thiscall" fn(usize, u32, u32) -> u8 = core::mem::transmute(read_usize(read_usize(stage) + comp_vtable::CHECK_COMPLETE));
                if completa(stage, 0, 1) == 0 {
                    return 0;
                }
                k += 1;
                if k as i32 > league.current_stage() as i32 {
                    break;
                }
            }
        }
        if (league.current_stage() as i32) < (league.stage_count() as i32).wrapping_sub(1) {
            return 0;
        }
    }
    if league.complete() == 0 {
        if warn != 0 {
            let end: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(league.addr()) + league_vtable::UPDATE_POSITIONS_AND_STATUS));
            end(league.addr());
        }
        league.set_complete((1) as usize);
    }
    1
}

/// True if `league_check_complete` takes a read-only path (no table, finished, no +0x45, complete outside single stage, or some club
/// short of games).
pub(crate) unsafe fn is_complete_check_read_only(league: League, stages: u32) -> bool {
    let table = league.table();
    if table == 0 || league.stopped() != 0 || league.active() == 0 {
        return true;
    }
    if league.complete() != 0 {
        return league.stage_index() != stage_index::MAIN || stages != 1;
    }
    let n = league.club_count();
    let mut total = league.games_per_club();
    if total == -1 && n > 0 {
        total = (n - 1).wrapping_mul(league.legs());
    }
    (0..n.max(0) as usize).any(|i| ((read_u8(table + league_table_row::SIZE * i + league_table_row::PLAYED) as i8) as i16) < total)
}

/// Pays TV revenue (FN_FINANCE_ADD_TV_REVENUE) to each league table club (optionally filtered by last division), with
/// the TV rights news unless disabled.
pub(crate) unsafe fn pay_tv_money(league: League, value: i32, filter: usize, news: bool) {
    let finances = read_usize(G_FINANCE_TABLE);
    if finances == 0 || league.club_count() <= 0 {
        return;
    }
    covered!("league tv rights");
    let mut i: i16 = 0;
    loop {
        let row = league.table() + league_table_row::SIZE * i as i32 as usize;
        let club = Club(read_usize(row));
        if filter == 0 || club.last_division().addr() == filter {
            exe::finance_add_tv_revenue(exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club), value); // the TV money
            if news {
                exe::comp_send_tv_rights_news(league, read_usize(league.table() + league_table_row::SIZE * i as i32 as usize), value);
            }
        }
        i = i.wrapping_add(1);
        if i >= league.club_count() {
            break;
        }
    }
}

/// Pays TV rights: adds the amount to the TV revenue of every club in the table (with a filter, only those with
/// `last_division()` equal to it) and sends the news (`exe::comp_send_tv_rights_news`).
///
/// `value.pay_tv_rights(filter)`: with the finance table `G_FINANCE_TABLE`, for each club in the table (`table()`; with
/// `filter`, only those whose `last_division()` equals it) adds `value` to the TV revenue
/// (`exe::financetable_get_club_finance` → `exe::finance_add_tv_revenue`) and sends the news (Comp::send_tv_rights_news
/// `exe::comp_send_tv_rights_news`, only for a club with a human manager). No SEH, no random draw, no locals.
/// Original: `League::pay_tv_rights` (0x683010, league.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn pay_tv_rights(league: usize, value: i32, filter: usize) {
    pay_tv_money(League(league), value, filter, true)
}

/// Records the season honours: champion, runner-up and third into the history (`exe::comp_util_add_history_record`),
/// with the host nation's national side as the featured entry when the competition asks
/// (`comp_vtable::HAS_HOST_COUNTRY`, `exe::hostcountry_find_hosts`, `exe::database_get_national_team`); a champion of
/// this division clears `club::LAST_DIVISION` of the clubs pointing at it.
///
/// `League::record_season_honours()`: champion, runner-up and (with more than 2 clubs) third of the table (`table()`).
/// If the champion's division (`division()`) is this competition (`competition()`), zeroes in the clubs (`club_at`,
/// `G_CLUB_COUNT` − `G_NATION_COUNT`) the `club::LAST_DIVISION` that points at it. If virtual
/// `comp_vtable::HAS_HOST_COUNTRY` of the competition's league (`G_COMP_OBJECT_TABLE`[id]) asks, the featured entry is
/// the national side (`exe::database_get_national_team`) of the host country (HostCountry::find_hosts
/// `exe::hostcountry_find_hosts` with year `year()`): with two hosts, the one with the higher `nation::REPUTATION`
/// (tie: the first). Writes into the history (`exe::comp_util_add_history_record`) and returns what it returns. No SEH,
/// no random draw; the two host locals are only read when `exe::hostcountry_find_hosts` writes them.
/// Original: `League::record_season_honours` (0x684640, league.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn league_record_season_honours(league: usize) -> usize {
    let league = League(league);
    covered!("league season honours");
    let table = league.table();
    let runner_up = read_usize(table + 0x3b);
    let champion = Club(read_usize(table));
    let third = if league.club_count() > 2 { read_usize(table + 0x76) } else { 0 };
    if champion.division() == league.competition() {
        let how_many = || read_i32(G_CLUB_COUNT).wrapping_sub(read_i32(G_NATION_COUNT));
        let mut k = 0i32;
        while k < how_many() {
            let p = club_at(k as isize).addr() + club::LAST_DIVISION;
            if read_usize(p) == league.competition().addr() {
                write_usize(p, 0); // the previous division
            }
            k += 1;
        }
    }
    let comp = league.competition().addr();
    let target = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(4 * read_u32(comp) as usize));
    let pede: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(target) + comp_vtable::HAS_HOST_COUNTRY));
    let mut highlight = 0usize;
    if pede(target) != 0 {
        let (mut a, mut b) = (0i32, 0i32);
        let q = exe::hostcountry_find_hosts(read_usize(G_HOST_COUNTRY), read_u32(league.competition().addr()) as usize, league.year() as u16 as u32, &mut a, &mut b, 0) as u8;
        let nation = |k: i32| read_usize(G_NATION_TABLE).wrapping_add(k.wrapping_mul(0x122) as usize);
        if q == 2 {
            highlight = if read_i16(nation(a) + nation::REPUTATION) < read_i16(nation(b) + nation::REPUTATION) { exe::database_get_national_team(nation(b)) } else { exe::database_get_national_team(nation(a)) };
        } else if q != 0 {
            highlight = exe::database_get_national_team(nation(a));
        }
    }
    exe::comp_util_add_history_record(league, champion, runner_up, third, highlight) // champion, runner-up, third
}

/// Memory regions of the club competition history: count, pointer and the 26-byte rows.
pub(crate) unsafe fn comp_history_regions() -> Vec<(usize, usize)> {
    let n = read_i32(G_CLUB_COMP_HISTORY_COUNT).max(0) as usize;
    vec![(G_CLUB_COMP_HISTORY_COUNT, 4), (G_CLUB_COMP_HISTORY_TABLE, 4), (read_usize(G_CLUB_COMP_HISTORY_TABLE), 26 * n)]
}

/// Next free row of the club competition history after the last filled one (+4 competition); 0 if none filled or the
/// table is full.
pub(crate) unsafe fn free_comp_history_row() -> usize {
    let n = read_i32(G_CLUB_COMP_HISTORY_COUNT);
    let t = read_usize(G_CLUB_COMP_HISTORY_TABLE);
    let mut i = n - 1;
    while i >= 0 {
        if read_usize(t.wrapping_add(26 * i as usize + 4)) != 0 {
            break;
        }
        i -= 1;
    }
    if i < 0 || i >= n - 1 {
        return 0;
    }
    t.wrapping_add(26 * (i + 1) as usize)
}

/// whether the league has relegation places at the bottom of the table
/// Original: `League::has_relegation` (0x6847b0, league.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn has_relegation(league: usize) -> u32 {
    let league = League(league);
    (league.relegated_count() != 0) as u32 // tem rebaixamento
}

/// Stores in a league schedule round record the date with a relative year, the code and a value.
///
/// The date (Date::Date, `exe::date_date_549f70`, into a local Date) from `day`/`month`/(`years` + `base`)/`week` in
/// record `i` (signed word, 0x41 bytes) of `rounds`: day of the year in word +0, year − `base` in word
/// `league_round_date::YEAR_OFFSET`, `code` in byte `league_round_date::TIME_SLICE` and `value` in
/// `league_round_date::VENUE_VALUE`. Returns the record.
/// Original: `league_round_set_date` (0x68a160, league.cpp, cdecl).
pub(crate) unsafe extern "C" fn league_round_set_date(rounds: usize, i: u32, day: u32, month: u32, years: u32, week: u32, code: u32, base: u32, value: u32) -> usize {
    let mut d = [0u16; 4];
    exe::date_date_549f70(&mut d, day, month, years.wrapping_add(base), week);
    let r = rounds.wrapping_add((i as i16 as isize * 0x41) as usize);
    write_u16(r, d[0]);
    write_u16(r + league_round_date::YEAR_OFFSET, d[1].wrapping_sub(base as u16)); // ano relativo
    write_u8(r + league_round_date::TIME_SLICE, code as u8);
    write_u32(r + league_round_date::VENUE_VALUE, value);
    r
}

/// Returns the league match importance for the club: the league base, +1 early in the season or with the final-day
/// margin.
///
/// Ret 0xc: the importance of the `fixture` for the club on `side` (0 = home, +0x1c; otherwise +0x20): (byte
/// `importance()` of the league − 10) / 4 + 10. Writes 0xff into `out`; with the club in the table (`table()`,
/// `league_table_row::SIZE`-byte rows, `club_count()` clubs) and more than 10 rounds (`round_count()`): +1 with
/// `round_number()` of the fixture below 3; otherwise, a row with no mark (`status()` = `table_status::UNDECIDED`), +1
/// and the last-day margin (`exe::league_final_day_margin_needed`, which can change the importance) in `out`. Only al
/// is returned (the caller uses al).
/// Original: `League::get_match_importance` (0x68a3e0, league.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn get_match_importance(league: usize, fixture: usize, side: u32, out: *mut u8) -> u8 {
    let fixture = Fixture(fixture);
    let league = League(league);
    *out = 0xff;
    let club = read_usize(fixture.addr() + if side as u8 == 0 { 0x1c } else { 0x20 });
    let mut imp = (((league.importance() as i8) as i32 - 10) / 4) as u8;
    imp = imp.wrapping_add(10); // base importance
    let clubs = league.club_count();
    let row = |i: i16| (league.table()).wrapping_add((i as isize * (league_table_row::SIZE as isize)) as usize);
    // simplification: the original tests i == −1 after the search; i starts at 0 and stops before `clubes`, it is never −1
    let mut i = 0i16;
    loop {
        if i >= clubs {
            return imp;
        }
        if read_usize(row(i)) == club {
            break;
        }
        i = i.wrapping_add(1);
    }
    if league.round_count() <= 10 {
        return imp;
    }
    if fixture.round_number() < 3 {
        return imp.wrapping_add(1);
    }
    let r = LeagueTableRow(row(i));
    if r.status() != table_status::UNDECIDED {
        return imp;
    }
    let mut local = imp.wrapping_add(1);
    *out = exe::league_final_day_margin_needed(league, r, &mut local);
    local
}

/// The qsort comparison of the table rows: points, the league tie-break rules and the club id.
///
/// Compares table rows `a` and `b` (`[+0]` club). Current league in `G_LEAGUE_SORT_CURRENT_LEAGUE`, the table column in
/// `table_column()` (14 bytes per column in the rows). Row without a club: 0. With the private comparison in
/// `G_LEAGUE_SORT_CUSTOM_COMPARE_FUNC` and its answer non-zero: it again. Otherwise points (+0x19) descending; both
/// without matches (+0x11): by club id. Then the league's four criteria (`league::TIEBREAKS`): 1 goal difference, 2
/// goals for, 3 byte +0x12, 4 byte +4 (`position()`), 5 word `away_goals_for()` (in column 0, criterion 6 = byte +4 and
/// stop); in column 0 with both at the league's number of matches (`played()` = `games_per_club()`), the id of the
/// clubs' `club::STADIUM`; finally the club id.
/// Original: `league_compare_table_rows` (0x683210, league.cpp, cdecl).
pub(crate) unsafe extern "C" fn league_compare_table_rows(a: usize, b: usize) -> i32 {
    let a = LeagueTableRow(a);
    let b = LeagueTableRow(b);
    let league = League(read_usize(G_LEAGUE_SORT_CURRENT_LEAGUE));
    let column = league.table_column() as i32;
    if read_usize(a.addr()) == 0 || read_usize(b.addr()) == 0 {
        return 0;
    }
    let own = read_usize(G_LEAGUE_SORT_CUSTOM_COMPARE_FUNC);
    if own != 0 {
        let f: unsafe extern "C" fn(usize, usize) -> i32 = core::mem::transmute(own);
        if f(a.addr(), b.addr()) != 0 {
            let f: unsafe extern "C" fn(usize, usize) -> i32 = core::mem::transmute(read_usize(G_LEAGUE_SORT_CUSTOM_COMPARE_FUNC));
            return f(a.addr(), b.addr());
        }
    }
    let c = (column as usize).wrapping_mul(14);
    let w = |r: usize, o: usize| read_i16(r.wrapping_add(c).wrapping_add(o)) as i32;
    let y = |r: usize, o: usize| read_i8(r.wrapping_add(c).wrapping_add(o)) as i32;
    let (pa, pb) = (w(a.addr(), 0x19), w(b.addr(), 0x19));
    if pa != pb {
        return pb - pa; // pontos
    }
    let id = |r: usize| read_i32(read_usize(r));
    if y(a.addr(), 0x11) == 0 && y(b.addr(), 0x11) == 0 {
        return id(a.addr()).wrapping_sub(id(b.addr()));
    }
    for k in 0..4usize {
        let criterio = read_i8(league.addr() + league::TIEBREAKS + k);
        if column == 0 && criterio == 6 {
            return a.position() as i32 - b.position() as i32;
        }
        if criterio == 0 {
            break;
        }
        let (va, vb) = match criterio {
            1 => (w(a.addr(), 0x15) - w(a.addr(), 0x17), w(b.addr(), 0x15) - w(b.addr(), 0x17)),
            2 => (w(a.addr(), 0x15), w(b.addr(), 0x15)),
            3 => (y(a.addr(), 0x12), y(b.addr(), 0x12)),
            4 => (a.position() as i32, b.position() as i32),
            5 => (a.away_goals_for() as i32, b.away_goals_for() as i32),
            _ => continue,
        };
        if va != vb {
            return vb - va;
        }
    }
    if column == 0 {
        let games = league.games_per_club();
        if a.played() as i16 == games && b.played() as i16 == games {
            let xb = read_usize(read_usize(b.addr()) + club::STADIUM);
            let xa = read_usize(read_usize(a.addr()) + club::STADIUM);
            if xb != 0 && xa != 0 {
                let d = read_i32(xb).wrapping_sub(read_i32(xa));
                if d != 0 {
                    return d;
                }
            }
        }
    }
    id(a.addr()).wrapping_sub(id(b.addr()))
}

/// Builds the round-robin grid where every club meets each other once, with balanced home and away games.
///
/// Stdcall, ret 8: the league's round grid for `n` clubs: `grid[1..=n]` are rows of integers (columns 1..n−1 = rounds)
/// with each club's opponent in the round (positive at home, negative away). Zeroes everything; club `n` faces 1..n−1
/// in a round that advances 2 at a time (at home for the first half); then each club k from n−1 down to 2 fills the
/// empty slots with clubs 1..n, alternating home advantage starting from the parity of n−1−k. Returns 1.
/// Original: `league_build_round_robin_grid` (0x6844b0, league.cpp, stdcall).
pub(crate) unsafe extern "stdcall" fn league_build_round_robin_grid(grid: *const *mut i32, n: i32) -> u32 {
    let cell = |i: i32, j: i32| (*grid.offset(i as isize)).offset(j as isize);
    for i in 1..=n {
        for j in 1..n {
            *cell(i, j) = 0;
        }
    }
    let half = n / 2;
    let mut col = n - 1;
    for i in 1..n {
        if i <= half {
            *cell(i, col) = n;
            *cell(n, col) = -i;
        } else {
            *cell(i, col) = -n;
            *cell(n, col) = i;
        }
        col = if col == n - 2 { 1 } else if col == n - 1 { 2 } else { col + 2 };
    }
    let mut k = n - 1;
    while k >= 2 {
        let minus_k = -k;
        let mut away = (n - 1 + minus_k) % 2;
        let mut col = k - 1;
        let mut contra = -1;
        for j in 1..=n {
            let c = cell(j, col);
            if *c == 0 {
                if away != 0 {
                    *c = k; // place filled
                    *cell(k, col) = contra;
                } else {
                    *c = minus_k;
                    *cell(k, col) = j;
                }
                away = 1 - away;
            }
            col = if col == n - 1 { 1 } else { col + 1 };
            contra -= 1;
        }
        k -= 1;
    }
    1
}

/// Returns row i (59 bytes) of the league table [+0xb1].
pub(crate) unsafe fn league_row(league: League, i: i32) -> usize {
    (league.table()).wrapping_add((i as isize * (league_table_row::SIZE as isize)) as usize)
}

/// Goal margin (+1, min 1, else 0xff) a row needs to reach the target row by the league criterion (+0xc5): goal
/// difference or goals scored.
pub(crate) unsafe fn goal_margin_to_reach(league: League, row: LeagueTableRow, target: LeagueTableRow) -> u8 {
    let c = if (league.tiebreaks() as u8) == 1 {
        (target.goals_for() as u8).wrapping_sub(target.goals_against() as u8).wrapping_sub(row.goals_for() as u8).wrapping_add(row.goals_against() as u8)
    } else {
        (target.goals_for() as u8).wrapping_sub(row.goals_for() as u8)
    }
    .wrapping_add(1);
    if (c as i8) < 1 { 0xff } else { c }
}

/// On the last matchday, returns the goal margin a club needs for the title, the top places or survival, and sets the
/// deciding-match flag.
///
/// Ret 8: on the last day (matches `played()` ≥ league rounds `round_count()` − 1) and a row with no mark (`status()` =
/// `table_status::UNDECIDED`), the goal margin the club needs for the title, the top places
/// (`promoted_count()`/`promotion_playoff_count()`) or to escape the bottom ones
/// (`relegation_playoff_count()`/`relegated_count()`); 0xff if none. With points equal to the club ahead and no
/// possible margin, writes 0x14 into `out` (deciding) and returns 0 or 0xff. Only al is returned (the caller uses al).
/// Original: `League::final_day_margin_needed` (0x68a4d0, league.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn final_day_margin_needed(league: usize, row: usize, out: *mut u8) -> u8 {
    let row = LeagueTableRow(row);
    let league = League(league);
    let p = |r: usize| read_i16(r + league_table_row::POINTS) as i32;
    let extra = (league.win_points() as i8) as i32;
    let pos = row.position();
    let clubs = league.club_count();
    let table = LeagueTableRow(league.table());
    let criterio = league.tiebreaks() as u8;
    if (row.played() as i32) < league.round_count() as i32 - 1 || row.status() != table_status::UNDECIDED {
        return 0xff; // outside the last day
    }
    // the `alvo` row ahead, unmarked: points + extra above it, 0xff; equal (extra 0), the margin
    let ver = |target: usize| -> Option<u8> {
        if p(row.addr()) + extra > p(target) {
            return Some(0xff);
        }
        if extra == 0 && (criterio == 1 || criterio == 2) {
            return Some(goal_margin_to_reach(league, row, LeagueTableRow(target)));
        }
        None
    };
    // the first unmarked row from `de` to `ate` (exclusive), with points + extra against those of `linha`
    let decides = |of: i16, ate: i16| -> Option<u8> {
        let mut i = of;
        while i < ate {
            let r = league_row(league, i as i32);
            if read_u8(r + league_table_row::STATUS) == 0xff {
                let v = p(r) + extra;
                if v > p(row.addr()) {
                    *out = 0x14;
                    return Some(0xff);
                }
                if v != p(row.addr()) {
                    return Some(0xff);
                }
                *out = 0x14;
                return Some(0);
            }
            i += 1;
        }
        None
    };
    if (league.league_flags() as u8) & (league_flags::CHAMPION_STATUS as u8) != 0 && table.status() != table_status::CHAMPIONS {
        if pos > 0 {
            let v = p(row.addr()) + extra;
            if v > p(table.addr()) {
                return 0xff;
            }
            if v == p(table.addr()) && (criterio == 1 || criterio == 2) {
                return goal_margin_to_reach(league, row, table);
            }
        } else if clubs > 1 {
            let v = read_i16(table.addr() + 0x54) as i32 + extra;
            if v > p(table.addr()) {
                *out = 0x14;
                return 0xff;
            }
            if v != p(table.addr()) {
                return 0xff;
            }
            *out = 0x14;
            return 0;
        }
    }
    let (s1, s2) = (league.promoted_count(), league.promotion_playoff_count());
    if s1 != 0 || s2 != 0 {
        let slots = s2.wrapping_add(s1) as i8;
        if pos < slots {
            if let Some(r) = decides(slots as i16, clubs) {
                return r;
            }
        } else {
            let mut i = 0i16;
            while i < slots as i16 {
                let r = league_row(league, i as i32);
                if read_u8(r + league_table_row::STATUS) == 0xff {
                    if let Some(m) = ver(r) {
                        return m;
                    }
                }
                i += 1;
            }
        }
    }
    let (d1, d2) = (league.relegated_count(), league.relegation_playoff_count());
    if d1 == 0 && d2 == 0 {
        return 0xff;
    }
    let descem = d2.wrapping_add(d1) as i8;
    let limit = clubs as i32 - descem as i32;
    if (pos as i32) < limit {
        return decides(clubs.wrapping_sub(descem as i16), clubs).unwrap_or(0xff);
    }
    let mut i = 0i16;
    while (i as i32) < limit {
        let r = league_row(league, i as i32);
        if read_u8(r + league_table_row::STATUS) == 0xff {
            if let Some(m) = ver(r) {
                return m;
            }
        }
        i += 1;
    }
    0xff
}

/// The ordering of the head-to-head mini table: points and the league tie-breakers.
///
/// qsort callback: an empty row ties; otherwise points (`points()`), and then the league's criteria
/// (`G_LEAGUE_SORT_CURRENT_LEAGUE` + `league::TIEBREAKS`, up to 4; 0 ends): 1 goal difference (`goals_for()` −
/// `goals_against()`), 2 goals for (`goals_for()`), 3 byte `won()`, 5 word `away_goals_for()`; 4 and the others do not
/// decide. Returns b − a at the first criterion that differs.
/// Original: `league_compare_h2h_rows` (0x6834f0, league.cpp, cdecl).
pub(crate) unsafe extern "C" fn league_compare_h2h_rows(a: usize, b: usize) -> i32 {
    let a = LeagueTableRow(a);
    let b = LeagueTableRow(b);
    if read_u32(a.addr()) == 0 || read_u32(b.addr()) == 0 {
        return 0;
    }
    if (a.points() as i32) != (b.points() as i32) {
        return (b.points() as i32) - (a.points() as i32);
    }
    let criterios = read_usize(G_LEAGUE_SORT_CURRENT_LEAGUE) + league::TIEBREAKS;
    for k in 0..4 {
        let (x, y) = match read_i8(criterios + k) {
            0 => return 0,
            1 => ((a.goals_for() as i32) - (a.goals_against() as i32), (b.goals_for() as i32) - (b.goals_against() as i32)),
            2 => ((a.goals_for() as i32), (b.goals_for() as i32)),
            3 => (a.won() as i32, b.won() as i32),
            5 => ((a.away_goals_for() as i32), (b.away_goals_for() as i32)),
            _ => continue,
        };
        if x != y {
            return y - x;
        }
    }
    0
}

/// Returns the 14-byte head-to-head record of table row `row` against opponent (pointer at +5 of the 59-byte row).
pub(crate) unsafe fn head_to_head_record(table: usize, row: i32, opponent: i32) -> usize {
    let confrontos = core::ptr::read_unaligned((table as isize + row as isize * 59 + 5) as *const usize);
    (confrontos as isize + opponent as isize * 14) as usize
}

/// Adds one match to the head-to-head record (played, result, goals and points).
///
/// Ret 0x14: adds a match to the head-to-head entry: matches (+4), wins (+5, with the league's win points
/// `win_points()`), draws (+6, draw points `draw_points()`) or defeats (+7), goals for (`h2h_entry::GOALS_FOR`) and
/// against (`h2h_entry::GOALS_AGAINST`); the points in `h2h_entry::POINTS`. Negative row or opponent: nothing.
/// Original: `League::update_head_to_head` (0x6869a0, league.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn update_head_to_head(league: usize, table: usize, row: i32, opponent: i32, pro: i8, contra: i8) {
    let league = League(league);
    if row < 0 || opponent < 0 {
        return;
    }
    let e = head_to_head_record(table, row, opponent);
    let add = |o: usize, v: i16| write_i16(e + o, core::ptr::read_unaligned((e + o) as *mut i16).wrapping_add(v));
    write_u8(e + 4, read_u8(e + 4).wrapping_add(1));
    if pro > contra {
        write_u8(e + 5, read_u8(e + 5).wrapping_add(1));
        add(h2h_entry::POINTS, (league.win_points() as i8) as i16);
    } else if pro == contra {
        write_u8(e + 6, read_u8(e + 6).wrapping_add(1));
        add(h2h_entry::POINTS, (league.draw_points() as i8) as i16);
    } else {
        write_u8(e + 7, read_u8(e + 7).wrapping_add(1));
    }
    add(h2h_entry::GOALS_FOR, pro as i16);
    add(h2h_entry::GOALS_AGAINST, contra as i16);
}

/// does the competition have `promoted_count()` and flag 2 of `league_flags()`?
/// Original: `competition_flagged` (0x684790, league.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn competition_flagged(t: usize) -> u32 {
    let t = League(t);
    (t.promoted_count() != 0 && (t.league_flags() as u8) & 2 != 0) as u32
}

/// the record (`league_table_row::SIZE` bytes) of table `table()`
/// Original: `league_record` (0x6826d0, league.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn league_record(t: usize, i: i32) -> usize {
    let t = League(t);
    t.table().wrapping_add((i as usize).wrapping_mul(league_table_row::SIZE))
}

compare_value!(compare_competition_flagged, competition_flagged, IDX_COMPETITION_FLAGGED, () -> u32);
compare_value!(compare_league_record, league_record, IDX_LEAGUE_RECORD, (i: i32) -> usize);

/// Returns the next position in the calendar rotation (special league, jumps of 3 by random draw, wrapping at n).
///
/// The next position `pos` in the calendar rotation of `n` (word) clubs. In the special league (`league` =
/// `KEY_CLUB_COMP_UEFA_EUROPEAN_CHAMPIONS_LEAGUE`): 2 becomes 4, 5 becomes 3, the rest advance 1. With fewer than 2
/// legs (`turnos`) or 8 rounds (`rounds`), or the 0x40 flag (`marks`), advances 1; with 0x80, advances 1 swapping n/2
/// for n−1 and n for n/2. Otherwise the state `*step_towards` (0..3) alternates jumps: 1 and 2 go back 1 (below 0, n−1)
/// and move to 2 and 3; 3 jumps 3 and goes back to 0; 0 jumps 3 (moving to 1) if `pos` < n−5 and random(4)
/// (`exe::utils_random`) ≥ 2, otherwise advances 1; the jumps wrap around at n.
/// Original: `rotate_round` (0x68a030, league.cpp, cdecl).
pub(crate) unsafe extern "C" fn rotate_round(pos: i32, step_towards: *mut i8, league: i32, rounds: i16, turnos: i16, n: i16, marks: u8) -> i32 {
    if league == read_i32(KEY_CLUB_COMP_UEFA_EUROPEAN_CHAMPIONS_LEAGUE) {
        return match pos {
            2 => 4,
            5 => 3,
            _ => pos + 1,
        };
    }
    if turnos < 2 || rounds < 8 || marks & 0x40 != 0 {
        return pos + 1;
    }
    let n = n as i32;
    if marks & 0x80 != 0 {
        let p = pos + 1;
        return if p == n / 2 { n - 1 } else if p == n { n / 2 } else { p };
    }
    let volta = |p: i32| if p < n { p } else { p - n };
    match *step_towards {
        1 | 2 => {
            *step_towards += 1;
            if pos - 1 >= 0 { pos - 1 } else { n - 1 }
        }
        3 => {
            *step_towards = 0;
            volta(pos + 3)
        }
        _ => {
            if pos < n - 5 && exe::utils_random(4) >= 2 {
                *step_towards = 1;
                volta(pos + 3)
            } else {
                volta(pos + 1)
            }
        }
    }
}

/// Stores a round match (three bytes and a dword).
///
/// Writes match `j` 0..7 of round `round` (0x41 bytes each, 7 per match starting at `base`):
/// `league_round_date::TV_SLOT_WEEKDAY` `a` (−1..6), `league_round_date::TV_SLOT_TIME_SLICE` `b` (−1..2),
/// `league_round_date::TV_SLOT_KIND` `c` (−1..4) and `league_round_date::TV_SLOT_DATA` the dword `value`. Out of range:
/// the original routine (error boxes, and it writes anyway).
/// Original: `store_round_match` (0x68a1c0, league.cpp, cdecl).
pub(crate) unsafe extern "C" fn store_round_match(base: usize, round: i16, j: i16, a: i16, b: i8, c: i8, value: u32) {
    if !(0..=7).contains(&j) || !(-1..=6).contains(&a) || !(-1..=2).contains(&b) || !(-1..=4).contains(&c) {
        return original_fn::<unsafe extern "C" fn(usize, i16, i16, i16, i8, i8, u32)>(IDX_STORE_ROUND_MATCH)(base, round, j, a, b, c, value);
    }
    let e = base.wrapping_add((round as isize * 0x41 + j as isize * 7) as usize);
    write_i8(e + league_round_date::TV_SLOT_WEEKDAY, a as i8);
    write_i8(e + league_round_date::TV_SLOT_TIME_SLICE, b);
    write_i8(e + league_round_date::TV_SLOT_KIND, c);
    write_u32(e + league_round_date::TV_SLOT_DATA, value);
}

/// the league total prize fund (one more share per place above the last)
/// Original: `League::calc_prize_fund` (0x682f70, league.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn calc_prize_fund(league: usize, quota: i32) -> i32 {
    let league = League(league);
    let teams = league.club_count();
    if teams == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, i32) -> i32>(IDX_CALC_PRIZE_FUND)(league.addr(), quota);
    }
    let mut total = 0i32;
    let mut parcela = quota;
    for _ in 0..teams.max(0) {
        total = total.wrapping_add(parcela);
        parcela = parcela.wrapping_add(quota);
    }
    total
}

/// at the season end, only the top league of the pyramid (no league above) closes its table and walks down the chain of
/// divisions: for each pair (upper and lower), closes the lower one and swaps the promoted and relegated clubs
/// (`league_promote_and_relegate`, with contract clauses, pairwise or as a block, without checking counts)
/// Original: `League::process_promotion_relegation_chain` (0x689c20, league.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn league_process_promotion_relegation_chain(this: usize, aos_pairs: u32) {
    let this = League(this);
    if ((this.upper_division_id() as usize) as i32) >= 0 {
        return;
    }
    covered!("cqp_1 tv network");
    let mut above = this.addr();
    finalize_league_table(this); // cqp1: closes the top league
    let mut i = this.lower_division_id() as i32;
    while i >= 0 {
        let below = League(read_usize(read_usize(G_COMP_OBJECT_TABLE) + i as usize * 4));
        finalize_league_table(below); // cqp1: closes the lower one
        exe::league_promote_and_relegate(this, above, below, 1, aos_pairs, (u32::MAX) as u32 as i32, (u32::MAX) as u32 as i32); // cqp1: sobem e descem
        i = below.lower_division_id() as i32;
        above = below.addr();
    }
}

/// Moves the club to a division: the current division becomes last season's and the new one is set; clears the division
/// clauses (and the squad numbers when the new one uses them), opens the requested manager vacancy, upgrades the
/// stadium to the league minimum and turns status 3 into 2.
///
/// Ret 0xc; `this` is not read: `club.move_club_to_division(division, slot)`: the current division (`club::DIVISION`)
/// becomes last season's (`club::LAST_DIVISION`) and the new one goes in; the players' division clauses are cleared
/// (`exe::contract_manager_reset_division_clauses` on `G_CONTRACT_MANAGER`), also the squad numbers if the old division
/// did not use them and the new one does (`exe::squad_manager_club_uses_squad_numbers` on `G_SQUAD_MANAGER`, before and
/// after); with `slot` and no manager (`manager()`), the manager vacancy (`exe::manager_manager_add_job_vacancy` on
/// `G_MANAGER_MANAGER`); in a league division (`comp_type()` = `comp_type::LEAGUE` in competition
/// `G_COMP_OBJECT_TABLE`[id]) with a minimum stadium (`required_capacity()` ≠ −1) and a stadium record (`stadium()`),
/// the stadium works (`exe::finance_upgrade_stadium_for_league` on the finances `exe::financetable_get_club_finance` of
/// `G_FINANCE_TABLE`, seats `required_seats()` and `required_capacity()`); and, with the competition loaded, status
/// `professional_status()` = 3 (`professional_status::AMATEUR`) changes to 2
/// (`professional_status::SEMI_PROFESSIONAL`).
/// Original: `League::move_club_to_division` (0x6830b0, league.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn move_club_to_division(_this: usize, club: usize, division: usize, slot: u32) {
    let club = Club(club);
    covered!("cpm_3 change division");
    let before = exe::squad_manager_club_uses_squad_numbers(read_usize(G_SQUAD_MANAGER), club, (0) as u32);
    write_usize(club.addr() + club::LAST_DIVISION, club.division().addr()); // the division of the last season
    write_usize(club.addr() + club::DIVISION, division);
    let numeros = (before == 0 && exe::squad_manager_club_uses_squad_numbers(read_usize(G_SQUAD_MANAGER), club, (0) as u32) != 0) as u32;
    exe::contract_manager_reset_division_clauses(read_usize(G_CONTRACT_MANAGER), club, numeros);
    if slot != 0 && club.manager().addr() == 0 {
        exe::manager_manager_add_job_vacancy(read_usize(G_MANAGER_MANAGER), club, 0);
    }
    if division == 0 {
        return;
    }
    let comps = read_usize(G_COMP_OBJECT_TABLE);
    let id = read_usize(division);
    if comps != 0 {
        let comp = League(read_usize(comps + 4 * id));
        if !comp.is_null() && club.stadium().addr() != 0 && comp.comp_type() == comp_type::LEAGUE {
            let required_capacity = comp.required_capacity() as i16;
            if required_capacity != -1 {
                covered!("cpm_3 division stadium");
                let required_seats = comp.required_seats() as i16;
                let f = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club);
                (exe::finance_upgrade_stadium_for_league(f, required_seats as i32, required_capacity as i32, id, 0) as u32); // the stadium works
            }
        }
    }
    // the original rereads [0xadadfc] after the construction work (without it, the same value; if null, it also reads from address 0)
    if read_usize(read_usize(G_COMP_OBJECT_TABLE) + 4 * id) != 0 && club.professional_status() == professional_status::AMATEUR {
        club.set_professional_status(professional_status::SEMI_PROFESSIONAL);
    }
}

/// Shuffles the league table with the seeded rand and puts clubs sharing a ground, clubs with linked grounds, and
/// Celtic and Rangers in paired slots.
///
/// Ret 4: `clubs.draw_team_order()`: shuffles the league table (`table()`, `club_count()` records of 59 bytes,
/// `CUP_LEAGUE_TABLE_ROW_SIZE`) for the fixture grid: keeps a random draw (`exe::utils_random`, 30000), seeds the CRT
/// rand (`exe::crt_srand`) with `year()` + `G_GAME_RANDOM_SEED` and swaps n random pairs (`exe::crt_rand` × n /
/// 0x8000), then restores the seed (draw + 1). League with flag 0x100 (`league_flags::FIXED_TABLE_SLOTS`) in
/// `league_flags()`: each club goes to the slot that virtual table +0xa8 says (if free), the others to the first free
/// ones, and +0xac receives each slot. Otherwise: in Scotland (`competition()` =
/// `KEY_CLUB_COMP_SCOTTISH_PREMIER_DIVISION`) Celtic and Rangers (`G_KEY_CLUB_GLASGOW_CELTIC`,
/// `G_KEY_CLUB_GLASGOW_RANGERS`) go to slots 0 and clubs / 2, in random order (`exe::utils_random`, 2); clubs of the
/// same stadium (`club::STADIUM`) and then those of linked stadiums (`stadium::NEARBY_STADIUM`) go in pairs to slots k
/// and clubs / 2 + k (past half of the clubs, the box); the others to the first free slots. The grid
/// (`exe::crt_operator_new`, freed with `exe::crt_operator_delete`) goes back to the table. Out of memory: the box
/// (line 0xed3).
/// Original: `League::draw_team_order` (0x686a70, league.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn draw_team_order(this: usize, clubs: i32) {
    let this = League(this);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let n = || this.club_count() as i32;
    let table = || this.table();
    let half_n = n() / 2;
    let used = exe::crt_operator_new((n() * 4) as u32) as *mut u32;
    let posto = exe::crt_operator_new((n() << 2) as u32) as *mut u32;
    let grid = exe::crt_operator_new((n() * CUP_LEAGUE_TABLE_ROW_SIZE as i32) as u32);
    let mut k: i32 = 0;
    if grid == 0 || used.is_null() || posto.is_null() {
        show_error_at(0x9f15f4, 0xed3);
        exe::crt_operator_delete(posto as usize);
        exe::crt_operator_delete(used as usize);
        exe::crt_operator_delete(grid);
        return;
    }
    let mut i = 0;
    while i < n() {
        *used.add(i as usize) = 0;
        *posto.add(i as usize) = 0;
        i += 1;
    }
    let semente = sorteio(30000).wrapping_add(1);
    exe::crt_srand((this.year() as i32).wrapping_add(read_i32(G_GAME_RANDOM_SEED)) as u32); // cpu12: a semente
    covered!("cpu_12 drawn order");
    let mut i = 0;
    while i < n() {
        let a = ((exe::crt_rand() as i64 * n() as i64) / 0x8000) as i32 as usize;
        let b = ((exe::crt_rand() as i64 * n() as i64) / 0x8000) as i32 as usize;
        let mut tmp = [0u8; CUP_LEAGUE_TABLE_ROW_SIZE];
        let t = table();
        core::ptr::copy(( t + a * CUP_LEAGUE_TABLE_ROW_SIZE) as *const u8, tmp.as_mut_ptr(), CUP_LEAGUE_TABLE_ROW_SIZE);
        copy_row_59(t + a * CUP_LEAGUE_TABLE_ROW_SIZE, t + b * CUP_LEAGUE_TABLE_ROW_SIZE);
        core::ptr::copy(tmp.as_ptr(), (table() + b * CUP_LEAGUE_TABLE_ROW_SIZE) as *mut u8, CUP_LEAGUE_TABLE_ROW_SIZE); // cpu12: a troca
        i += 1;
    }
    exe::crt_srand(semente as u32);
    let half = clubs / 2;
    if this.league_flags() as u16 & (league_flags::FIXED_TABLE_SLOTS as u16) != 0 {
        let mut i = 0;
        while i < n() {
            write_u32(grid + i as usize * CUP_LEAGUE_TABLE_ROW_SIZE, 0);
            i += 1;
        }
        let vt = read_usize(this.addr());
        let slot_of: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(read_usize(vt + 0xa8));
        let mut i = 0usize;
        while (i as i32) < n() {
            let s = slot_of(this.addr(), read_usize(table() + i * CUP_LEAGUE_TABLE_ROW_SIZE)) as i8 as i32;
            if s >= 0 && s < n() && read_usize(grid + s as usize * CUP_LEAGUE_TABLE_ROW_SIZE) == 0 {
                copy_row_59(grid + s as usize * CUP_LEAGUE_TABLE_ROW_SIZE, table() + i * CUP_LEAGUE_TABLE_ROW_SIZE);
                write_u32(table() + i * CUP_LEAGUE_TABLE_ROW_SIZE, 0);
            }
            i += 1;
        }
        let mut i = 0usize;
        while (i as i32) < n() {
            let row = table() + i * CUP_LEAGUE_TABLE_ROW_SIZE;
            if read_usize(row) != 0 {
                if let Some(j) = (0..n().max(0) as usize).find(|&j| read_usize(grid + j * CUP_LEAGUE_TABLE_ROW_SIZE) == 0) {
                    copy_row_59(grid + j * CUP_LEAGUE_TABLE_ROW_SIZE, row);
                }
            }
            i += 1;
        }
        let write: unsafe extern "thiscall" fn(usize, usize, i32) = core::mem::transmute(read_usize(vt + 0xac));
        let mut i = 0;
        while i < n() {
            write(this.addr(), read_usize(grid + i as usize * CUP_LEAGUE_TABLE_ROW_SIZE), i);
            i += 1;
        }
    } else {
        if read_usize(this.competition().addr()) == read_usize(KEY_CLUB_COMP_SCOTTISH_PREMIER_DIVISION) {
            let (mut rangers, mut celtic): (i8, i8) = (-1, -1); // bl e [esp+0x17]
            let total = this.club_count();
            let mut c: i8 = 0;
            while total > 0 && (c as i16) < total {
                let id = read_usize(read_usize(table() + c as isize as usize * CUP_LEAGUE_TABLE_ROW_SIZE));
                if id == read_usize(G_KEY_CLUB_GLASGOW_CELTIC) {
                    celtic = c;
                }
                if id == read_usize(G_KEY_CLUB_GLASGOW_RANGERS) {
                    rangers = c;
                }
                c = c.wrapping_add(1);
            }
            if rangers != -1 && celtic != -1 {
                let (primeiro, segundo) = if sorteio(2) != 0 { (celtic, rangers) } else { (rangers, celtic) };
                copy_row_59(grid, table() + primeiro as isize as usize * CUP_LEAGUE_TABLE_ROW_SIZE);
                copy_row_59(grid + half as usize * CUP_LEAGUE_TABLE_ROW_SIZE, table() + segundo as isize as usize * CUP_LEAGUE_TABLE_ROW_SIZE);
                k = 1;
                *posto.add(half as usize) = 1;
                *posto = 1;
                *used.add(celtic as isize as usize) = 1;
                *used.add(rangers as isize as usize) = 1;
            }
        }
        // the clubs of the same stadium (+0x69)
        let mut i = 0usize;
        while (i as i32) < n() - 1 {
            if *used.add(i) == 0 {
                let mut j = i + 1;
                while (j as i32) < n() {
                    if *used.add(j) == 0 {
                        let ei = read_usize(read_usize(table() + i * CUP_LEAGUE_TABLE_ROW_SIZE) + club::STADIUM);
                        if ei != 0 {
                            let ej = read_usize(read_usize(table() + j * CUP_LEAGUE_TABLE_ROW_SIZE) + club::STADIUM);
                            if ej != 0 && ei == ej {
                                if k >= half_n {
                                    show_error_at(0x9f15f4, 0xf76);
                                } else {
                                    place_pair(grid, posto, used, table(), i, j, &mut k, half);
                                }
                                break;
                            }
                        }
                    }
                    j += 1;
                }
            }
            i += 1;
        }
        // the clubs with linked stadiums (+0x48 of one’s stadium is the other’s)
        let mut i = 0usize;
        while (i as i32) < n() - 1 {
            if *used.add(i) == 0 {
                let mut j = i + 1;
                while (j as i32) < n() {
                    if *used.add(j) == 0 {
                        let ei = read_usize(read_usize(table() + i * CUP_LEAGUE_TABLE_ROW_SIZE) + club::STADIUM);
                        if ei == 0 {
                            break;
                        }
                        let ej = read_usize(read_usize(table() + j * CUP_LEAGUE_TABLE_ROW_SIZE) + club::STADIUM);
                        if ej == 0 {
                            break;
                        }
                        if read_usize(ei + stadium::NEARBY_STADIUM) == ej || read_usize(ej + stadium::NEARBY_STADIUM) == ei {
                            if k >= half_n {
                                show_error_at(0x9f15f4, 0xfa3);
                            } else {
                                place_pair(grid, posto, used, table(), i, j, &mut k, half);
                            }
                            break;
                        }
                    }
                    j += 1;
                }
            }
            i += 1;
        }
        // the others, to the first free slots
        let mut i = 0usize;
        while (i as i32) < n() {
            if *used.add(i) == 0 {
                if let Some(j) = (0..n().max(0) as usize).find(|&j| *posto.add(j) == 0) { // cpu12: the first free place
                    covered!("cpu_12 club in the free slot");
                    copy_row_59(grid + j * CUP_LEAGUE_TABLE_ROW_SIZE, table() + i * CUP_LEAGUE_TABLE_ROW_SIZE);
                    *posto.add(j) = 1;
                    *used.add(i) = 1;
                }
            }
            i += 1;
        }
    }
    let mut i = 0usize;
    while (i as i32) < n() {
        copy_row_59(table() + i * CUP_LEAGUE_TABLE_ROW_SIZE, grid + i * CUP_LEAGUE_TABLE_ROW_SIZE);
        i += 1;
    }
    exe::crt_operator_delete(posto as usize);
    exe::crt_operator_delete(used as usize);
    exe::crt_operator_delete(grid);
}

/// For each table row, moves the club's professional status (`club::PROFESSIONAL_STATUS`) to the value (down or up)
/// when the row status is not the excluded one and when it is the included one (0xfd = none).
///
/// Ret 0x10: `value.limit_club_division_tier(away, inside, baixar)` (called by the season end of each league:
/// Argentina, Belgium, Brazil, Croatia, Denmark, England, France, Germany, Netherlands, Italy, Russia, Scotland, Sweden
/// and Turkey): for each table row (`table()`, `league_table_row::SIZE` bytes, count `club_count()`), adjusts the
/// professional status of the club (+0) to `value` (`cap_professional_status`) when the row status
/// (`league_table_row::STATUS`) is not `away` (0xfd = none) and again when it is `inside` (0xfd = none). The first
/// three arguments are bytes pushed as dwords: only the low byte counts.
/// Original: `League::limit_club_division_tier` (0x68a980, league.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn limit_club_division_tier(this: usize, value: u32, away: u32, inside: u32, baixar: u32) {
    let this = League(this);
    let (value, away, inside) = (value as u8 as i8, away as u8, inside as u8);
    let mut i: i16 = 0;
    while i < this.club_count() {
        let row = || this.table().wrapping_add(i as isize as usize * league_table_row::SIZE);
        if away != 0xfd && *((row() + league_table_row::STATUS) as *const u8) != away {
            cap_professional_status(Club(read_usize(row())), value, baixar);
        }
        if inside != 0xfd && *((row() + league_table_row::STATUS) as *const u8) == inside {
            cap_professional_status(Club(read_usize(row())), value, baixar);
        }
        i = i.wrapping_add(1);
    }
}


/// Whether a league about to be constructed will load its state from a saved game: League::load_from_temp_file
/// (0x687b10, called by every league constructor with 1) opens `<record name>_<record id>.tmp` in the game folder ('/'
/// and '.' in the name become '_'), which exists only while a save is being loaded. The constructor comparers use it to
/// check the new-career values only in a new career (a loaded Mineiro can have 2 stages where a new one has 3; found
/// 11/11/2026 on Lucas's save).
pub(crate) unsafe fn league_will_load_saved_state(record: usize) -> bool {
    if LOADING_SAVED_GAME.load(core::sync::atomic::Ordering::Relaxed) {
        return true; // a saved game is loading: the competitions (cups too, read from the save) take their saved state
    }
    if record == 0 {
        return false;
    }
    let mut name = Vec::new();
    let mut p = record + 4;
    while read_u8(p) != 0 && name.len() < 256 {
        let c = read_u8(p);
        name.push(if c == b'/' || c == b'.' { b'_' } else { c });
        p += 1;
    }
    let file = format!("{}_{}.tmp", String::from_utf8_lossy(&name), read_i32(record));
    std::path::Path::new(&file).exists()
}
