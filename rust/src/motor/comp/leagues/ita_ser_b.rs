use super::*;

/// the Serie B schedule: 38 rounds (20 clubs), from 29 August to 14 May, on the same weekday (6) except one (6
/// January); in nearly all, two matches in each of two TV slots (days 4 and 0, slot 2). The play-off match as in the
/// Serie A, on 15 and 21 May
/// Original: `ItaSerB::create_schedule` (0x645af0, ita_ser_b.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn ita_ser_b_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    italian_schedule(League(this), stage, rounds, extra, league, &SERIE_B_WITH_TIEBREAK)
}

/// `ItaSerB::update_for_date(today, full)` (slot +0x14): the Serie B day. Like the Serie A one, with the fixed Serie B
/// bands (the champion, 3 more promotion places so 4 go up, and the 4 relegated of 20) and without stopping at the
/// first pair: each tied pair of neighbours on a boundary is ordered by head-to-head
/// (`exe::ita_league_sort_clubs_level_on_points`: push edi, the points in the low word and the rest of `full` in the
/// high bits, which `exe::ita_league_sort_clubs_level_on_points` does not read; mov ecx -> thiscall, ret 4); if there
/// is any, the tie-break stage (`.stage_count()` = 1, `.stages()` = malloc of 4, virtual `comp_vtable::ADVANCE_STAGE`).
/// With none: `.complete()` = 0, `.tiebreak_matches()` = 0 and virtual `league_vtable::UPDATE_POSITIONS_AND_STATUS`.
/// Original: `ItaSerB::update_for_date` (0x647050, ita_ser_b.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn ita_ser_b_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let this = League(this);
    let r = exe::league_update_for_date(this, today, full) as u32;
    if this.complete() == 0 || this.stage_count() != 0 || full == 0 || this.tiebreak_matches() == 0 {
        return r;
    }
    let z = serie_b_zones();
    this.set_season_closed(0);
    covered!("cql_4 serie b end");
    let mut found = false;
    let mut i: i16 = 0;
    while (i as i32) < this.club_count() as i32 - 1 {
        let l = LeagueTableRow(ita_ser_a_table_row(this, i as usize));
        let points = l.points();
        if points == read_i16(l.addr() + 0x54) && z[(i as usize).min(63)] != z[(i as usize + 1).min(63)] { // cql4: the Serie B pair at the boundary
            found = true;
            exe::ita_league_sort_clubs_level_on_points(this, points as u16 as u32);
        }
        i = i.wrapping_add(1);
    }
    if found {
        this.set_stage_count(1);
        let v = exe::crt_malloc(4);
        this.set_stages(v);
        let proxima: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::ADVANCE_STAGE));
        proxima(this.addr());
        return r;
    }
    this.set_complete(0);
    this.set_tiebreak_matches(0);
    let status: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(this.addr()) + league_vtable::UPDATE_POSITIONS_AND_STATUS));
    status(this.addr());
    r
}

/// `ita_league_sort_clubs_level_on_points(points)` (also called by the Serie A one): clubs level on points are ordered
/// by the head-to-head among them and the table positions are renumbered. Builds the head-to-head at +4 of each row
/// (League::build_h2h_table `exe::league_rank_by_head_to_head`: push `.table()`; mov ecx, this -> thiscall, ret 4),
/// finds the block of rows with those points (the first one and how many; the table is already sorted by points), sorts
/// the block by the head-to-head comparison (qsort `exe::crt_qsort` with `league_compare_table_rows_h2h`, rows of 0x3b:
/// push; add esp, 0x10 -> cdecl; with no row, qsort receives 0 and 0) and renumbers the position (+4) of all rows.
/// Original: `ita_league_sort_clubs_level_on_points` (0x6471a0, ita_ser_b.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn ita_league_sort_clubs_level_on_points(this: usize, points: u32) {
    let this = League(this);
    exe::league_rank_by_head_to_head(this, this.table());
    let n = this.club_count();
    let mut first = 0usize;
    let mut how_many: u32 = 0;
    if n > 0 {
        let mut l = this.table();
        for _ in 0..n as i32 {
            if read_i16(l + league_table_row::POINTS) == points as u16 as i16 {
                if first == 0 {
                    first = l;
                }
                how_many = how_many.wrapping_add(1);
            }
            l += 0x3b;
        }
    }
    exe::crt_qsort(first, (how_many as u16 as i16 as i32) as u32, 0x3b, 0x683410); // cql4: the block of tied clubs
    let mut i: i16 = 0;
    while i < this.club_count() {
        write_u8(ita_ser_a_table_row(this, i as usize) + 4, i as u8);
        i += 1;
    }
}

/// Serie B: discipline 0x11, only the league, the real 2001/02 fixture list
/// Original: `ItaSerB::ItaSerB` (0x645810, ita_ser_b.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_ita_ser_b(this: usize, year: u32, record: usize) -> usize {
    construct_italian_division(League(this), year, record, &SERIE_B_DIVISION)
}

/// the 2001/02 Serie B: the 380 matches (38 rounds of 10) fixed, with the same rule
/// Original: `ItaSerB::set_up_fixed_fixtures` (0x647230, ita_ser_b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ita_ser_b_set_up_fixed_fixtures(this: usize) {
    set_up_fixed_fixtures(League(this), &SERIE_B_FIXTURES, 0x9ea384, 0x39a)
}

/// Serie B (20): the top 4 go up to A, the bottom 4 go down to C1; the rest as in A (tie-break match too)
/// Original: `ItaSerB::set_up_rules` (0x6468a0, ita_ser_b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ita_ser_b_set_up_rules(this: usize) -> u32 {
    covered!("cqi_6 serie b rules");
    ita_ser_a_apply_league_rules(League(this), &serie_b_rules(this))
}

/// Serie B has 20 clubs; prize fund of 70,000 per place; no TV money
/// Original: `ItaSerB::create_table` (0x646780, ita_ser_b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ita_ser_b_create_table(this: usize) -> u32 {
    match ita_ser_a_fill_league_table(League(this), 0x14, 0x9ea384, 0x19d, KEY_CLUB_COMP_ITALIAN_SERIE_B) {
        Some(true) => {}
        _ => return 0,
    }
    set_prize_fund(League(this), 0x11170);
    1
}
