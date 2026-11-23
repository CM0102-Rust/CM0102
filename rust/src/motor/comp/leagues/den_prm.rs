use super::*;

/// Rule: the Danish turnover, led by the Superliga: the season reopened; promotion and relegation along the chain; the
/// four best Danish clubs of the lowest division (the best twelve shuffled) up into the Second Division and its
/// relegated clubs down (`den_prm_refresh_second_division_clubs`); the division tier of the clubs capped (the
/// Superliga and the First Division with (1, 3, −3, 1), the Second Division with (2, −3, 3, 0)); the Superliga rebuilt
/// for the next year (rules, table, fixtures, order); the First and Second Divisions turned over; the predicted
/// positions, the board expectations and the officials.
///
/// `top_division_turnover` with, before, virtual `league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN`(1),
/// `den_prm_refresh_second_division_clubs` (direct, ecx = this) and League::limit_club_division_tier
/// (`exe::league_limit_club_division_tier`: 4 pushes -> thiscall, ret 0x10) on this, the First Division and the Second
/// Division (the objects of `KEY_CLUB_COMP_DANISH_FIRST_DIVISION` and `KEY_CLUB_COMP_DANISH_SECOND_DIVISION`); the table
/// DenPrm::create_table (0x5538d0) and the boxes of den_prm.cpp (0x9b8e6c) at 0x21b, 0x222, 0x228, 0x22f; after, the
/// First and Second Divisions' start_new_season (virtual `comp_vtable::START_NEW_SEASON`, no null check). After a box: 0.
/// Original: `DenPrm::start_new_season` (0x553aa0, den_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn den_prm_start_new_season(this: usize) -> u32 {
    let this = League(this);
    top_division_turnover(this, || {
        promotion_relegation_chain(this, 1);
        call_on_league(0x553cb0, this);
        exe::league_limit_club_division_tier(this, 1, 3, -3, 1);
        exe::league_limit_club_division_tier(comp_by_global(KEY_CLUB_COMP_DANISH_FIRST_DIVISION), 1, 3, -3, 1);
        exe::league_limit_club_division_tier(comp_by_global(KEY_CLUB_COMP_DANISH_SECOND_DIVISION), 2, -3i32 as u32, 3, 0);
    }, 0x5538d0, 0x9b8e6c, [0x21b, 0x222, 0x228, 0x22f], || {
        covered!("mwc_20 danish superliga turnover");
        bra_nat_first_call_virtual_no_args(comp_by_global(KEY_CLUB_COMP_DANISH_FIRST_DIVISION), comp_vtable::START_NEW_SEASON);
        bra_nat_first_call_virtual_no_args(comp_by_global(KEY_CLUB_COMP_DANISH_SECOND_DIVISION), comp_vtable::START_NEW_SEASON);
    })
}

/// Rule: at the Danish turnover, the four best Danish clubs of the lowest division (`KEY_CLUB_COMP_A_LOWER_DIVISION`,
/// sorted by strength with the best twelve shuffled) go up to the Second Division, and the Second Division's relegated
/// clubs (status 3) go down to the lowest division; with no candidate nobody moves.
///
/// A list of `G_CLUB_COUNT` pointers (`exe::crt_malloc`; out of memory: the box of den_prm.cpp at line 0x256 and
/// return) with the clubs in database order, stopping at the first national team (`exe::database_is_national_team`,
/// cdecl: the national teams come after the clubs), whose division (`club::DIVISION`) is the lowest one and whose nation
/// (`club::NATION`) is Denmark (`KEY_NATION_DENMARK`) (a 16-bit index). With at least one:
/// `exe::comp_util_sort_and_shuffle_top_clubs`(list, n, 4) (cdecl) and the first four moved to the Second Division
/// (League::move_club_to_division(this, club, the Second Division's record, 1)); with fewer than four the original
/// moves the list's unwritten slots as well (a defect of the original, kept); then each row i of the Second Division
/// (its club count read again on each step) with status 3 goes down (League::get_table_row, called twice as the
/// original, and League::relegate_club_to_division(this, club, the lowest division's record, 1)). The list is freed.
/// Original: `DenPrm::refresh_second_division_clubs` (0x553cb0, den_prm.cpp, thiscall: ecx = the Superliga, ret 0).
pub(crate) unsafe extern "thiscall" fn den_prm_refresh_second_division_clubs(this: usize) {
    let second = comp_by_global(KEY_CLUB_COMP_DANISH_SECOND_DIVISION);
    let list = exe::crt_malloc((read_usize(G_CLUB_COUNT) as u32).wrapping_mul(4));
    if list == 0 {
        show_error_at(0x9b8e6c, 0x256);
        return;
    }
    let mut n: u32 = 0;
    let mut club = read_usize(G_CLUB_TABLE);
    let mut i = 0i32;
    while i < read_usize(G_CLUB_COUNT) as i32 {
        if exe::database_is_national_team(club) != 0 {
            break;
        }
        let (d, nation) = (read_usize(club + club::DIVISION), read_usize(club + club::NATION));
        if d != 0 && nation != 0 && read_usize(nation) == read_usize(KEY_NATION_DENMARK) && read_usize(d) == read_usize(KEY_CLUB_COMP_A_LOWER_DIVISION) {
            write_usize(list + (n as u16 as i16 as isize as usize).wrapping_mul(4), club);
            n = n.wrapping_add(1);
        }
        i += 1;
        club = club.wrapping_add(club::SIZE);
    }
    if n as u16 != 0 {
        covered!("mwc_20 danish lowest division up");
        exe::comp_util_sort_and_shuffle_top_clubs(list, n, 4); // mwc20: the best twelve shuffled
        for k in 0..4 {
            exe::league_move_club_to_division(this, read_usize(list + 4 * k), comp_record_of(KEY_CLUB_COMP_DANISH_SECOND_DIVISION), 1);
        }
        let mut i = 0i32;
        while i < League(second).club_count() as i32 {
            if read_u8(exe::league_get_table_row(second, i) + league_table_row::STATUS) == 3 {
                let record = comp_record_of(KEY_CLUB_COMP_A_LOWER_DIVISION);
                let club = read_usize(exe::league_get_table_row(second, i));
                exe::league_relegate_club_to_division(this, club, record, 1);
            }
            i += 1;
        }
    }
    exe::crt_free(list);
}

/// Comparer for `den_prm_start_new_season`: only the original runs (it draws and rebuilds); the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_den_prm_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_DEN_PRM_START_NEW_SEASON);
    compare_state_league_new_season("den_prm_start_new_season", League(this), || o(this))
}

/// Comparer for `den_prm_refresh_second_division_clubs`: only the original runs (it draws); with at least one
/// candidate, the Second Division's clubs of status 3 must end in the lowest division.
pub(crate) unsafe extern "thiscall" fn compare_den_prm_refresh_second_division_clubs(this: usize) {
    let mut candidates = 0;
    for i in 0..read_usize(G_CLUB_COUNT) {
        let c = read_usize(G_CLUB_TABLE) + i * club::SIZE;
        if exe::database_is_national_team(c) != 0 {
            break;
        }
        let (d, nation) = (read_usize(c + club::DIVISION), read_usize(c + club::NATION));
        if d != 0 && nation != 0 && read_usize(nation) == read_usize(KEY_NATION_DENMARK) && read_usize(d) == read_usize(KEY_CLUB_COMP_A_LOWER_DIVISION) {
            candidates += 1;
        }
    }
    let l = League(comp_by_global(KEY_CLUB_COMP_DANISH_SECOND_DIVISION));
    let mut planned: Vec<usize> = Vec::new();
    if candidates >= 4 && l.table() != 0 {
        for i in 0..l.club_count() as usize {
            let row = l.table() + i * league_table_row::SIZE;
            if read_u8(row + league_table_row::STATUS) == 3 {
                planned.push(read_usize(row));
            }
        }
    }
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_DEN_PRM_REFRESH_SECOND_DIVISION_CLUBS)(this);
    let lower = comp_record_of(KEY_CLUB_COMP_A_LOWER_DIVISION);
    let wrong: Vec<usize> = planned.iter().copied().filter(|&c| read_usize(c + club::DIVISION) != lower).collect();
    SCOREBOARD.record(wrong.is_empty(), || format!("den_prm_refresh_second_division_clubs({:#x}): relegated clubs not in the lowest division {:x?}", this, wrong));
}

/// The Danish Superliga schedule: 33 rounds, from 27 Jul to 24 May; 30 with matches in TV slots; the spring changes
/// with the parity of the season year (from round 18: odd, as 2001/02, and even).
///
/// `DenPrm::create_schedule(stage, &rounds, &extra, &is_league)` (`create_league_schedule`): only the main stage (-1;
/// the others return 0 without writing); is_league = 1, rounds = 33, extra = 0 and the table of 33 x 0x41 bytes
/// (`exe::crt_malloc`; out of memory, the error box of line 0xa6 and 0) with each round's date and TV slots
/// (`league_round_set_date`, `league_round_set_match_slot`, cdecl).
/// Original: `DenPrm::create_schedule` (0x552900, den_prm.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn den_prm_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let year = League(this).year();
    let p = if (year as i32) % 2 != 0 { &DEN_PRM_ODD_YEAR } else { &DEN_PRM_EVEN_YEAR };
    create_league_schedule(League(this), stage, rounds, extra, league, p)
}

/// Comparer for `den_prm_create_schedule`: side by side, the copy's table freed (`compare_cfa_schedule`).
pub(crate) unsafe extern "thiscall" fn compare_den_prm_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_DEN_PRM_CREATE_SCHEDULE);
    compare_cfa_schedule("den_prm_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| den_prm_create_schedule(this, stage, r, e, l))
}

/// The Danish Superliga: discipline rule 8 (`DENMARK`), only the league; before the load, the Kvalifikationsraekken
/// clubs move to the generic lower division (`KEY_CLUB_COMP_A_LOWER_DIVISION`); with no saved state, the rules, the
/// table, the fixtures, the order, the statistics and the predicted positions (`construct_nation_division`).
/// Original: `DenPrm::DenPrm` (0x5525e0, den_prm.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_den_prm(this: usize, year: u32, record: usize) -> usize {
    construct_nation_division(League(this), year, record, &DEN_PRM_DIVISION, || {
        // mwc5: the Kvalifikationsraekken clubs go to the lower division before the load
        let kval = read_usize(KEY_CLUB_COMP_DANISH_KVALIFIKATIONSRAEKKEN);
        let mut i = 0i32;
        while i < read_usize(G_CLUB_COUNT) as i32 {
            let club = Club(read_usize(G_CLUB_TABLE) + i as usize * club::SIZE);
            let comp = club.division().addr();
            if comp != 0 && read_usize(comp) == kval {
                covered!("mwc_5 kvalifikationsraekken club moved");
                write_usize(club.addr() + club::DIVISION, comp_record_of(KEY_CLUB_COMP_A_LOWER_DIVISION));
            }
            i += 1;
        }
    })
}

/// Comparer for `construct_den_prm`: only the original runs (`compare_nation_division_ctor`).
pub(crate) unsafe extern "thiscall" fn compare_construct_den_prm(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_DEN_PRM);
    compare_nation_division_ctor("construct_den_prm", League(this), year, record, &DEN_PRM_DIVISION, || o(this, year, record))
}

/// Sets up the rules of the Danish Superliga: 3 rounds, 3 points for a win and 1 for a draw; 0 go up, 0 to the
/// promotion play-off, 0 to the relegation play-off and 2 go down; above: none; below:
/// `KEY_CLUB_COMP_DANISH_FIRST_DIVISION`; tie-breaker codes 1, 2, 0 (`+0xc5`..); 4 substitutes named and 3
/// substitutions; then builds the main-stage round table through virtual +0x3c into `league::ROUND_DATES`
/// (`fra_first_apply_league_rules`). Returns 1.
/// Original: `DenPrm::set_up_rules` (0x553a00, den_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn den_prm_set_up_rules(this: usize) -> u32 {
    covered!("mwc_5 den prm rules");
    fra_first_apply_league_rules(League(this), &den_prm_rules(this))
}

/// Comparer for `den_prm_set_up_rules`: a dry run (`compare_nation_league_rules`).
pub(crate) unsafe extern "thiscall" fn compare_den_prm_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_DEN_PRM_SET_UP_RULES);
    compare_nation_league_rules("den_prm_set_up_rules", League(this), den_prm_rules(this), || o(this), || den_prm_set_up_rules(this))
}

/// The Danish Superliga has 12 clubs; prize fund by the share of 50,000 per place; TV pays 100,000 to each club.
///
/// `DenPrm::create_table()`: `+0x3e` = 12, the table (`exe::crt_malloc` of 12 x 0x3b; out of memory, the error box of
/// line 0x1ab and 0) with the clubs of `KEY_CLUB_COMP_DANISH_PREMIER_DIVISION` in database order
/// (League::init_table_row); not full: 0. Full: the fund `+0x28` (League::calc_prize_fund, share 0xc350), the prize
/// mode `+0xe1` = 1, the TV money (League::pay_tv_rights, 0x186a0 to every club) and 1.
/// Original: `DenPrm::create_table` (0x5538d0, den_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn den_prm_create_table(this: usize) -> u32 {
    eng_prm_fill_league_table(League(this), 0xc, 0x9b8e6c, 0x1ab, KEY_CLUB_COMP_DANISH_PREMIER_DIVISION, 0xc350, || {
        eng_prm_pay_tv_rights(this, 0x186a0, 0); // mwc5: the TV money of each club
    })
}

/// Comparer for `den_prm_create_table`: only the original runs, against the plan (`compare_nation_league_table`).
pub(crate) unsafe extern "thiscall" fn compare_den_prm_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_DEN_PRM_CREATE_TABLE);
    compare_nation_league_table("den_prm_create_table", League(this), 0xc, KEY_CLUB_COMP_DANISH_PREMIER_DIVISION, Some(0xc350), || o(this))
}

/// The rule writes of the Danish Superliga (offset, size, value).
pub(crate) unsafe fn den_prm_rules(this: usize) -> Vec<(usize, u8, u32)> {
    let _ = this;
    let mut v = vec![(0x3c, 2, 3), (0xc3, 1, 1), (0xc4, 1, 2), (0xab, 2, 0), (0xdb, 2, 0), (0x42, 1, 1), (0xc5, 1, 1), (0xc6, 1, 2), (0xc7, 1, 0), (0xbe, 1, 0), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 2), (0x1c, 4, 0xffff_ffff), (0x52, 1, 2), (0x4a, 1, 3), (0x20, 4, read_usize(KEY_CLUB_COMP_DANISH_FIRST_DIVISION) as u32), (0x49, 1, 4)];
    v.push((0xc2, 1, 3)); // mwc5: the Danish Superliga points for a win
    v
}

pub(crate) const DEN_PRM_ODD_YEAR: FraThirdSingleStageLeague = FraThirdSingleStageLeague { rounds: 33, file: 0x9b8e6c, row: 0xa6, table: &DEN_PRM_ODD_YEAR_ROUNDS, branch: "mwc_3 den prm schedule" };

pub(crate) const DEN_PRM_EVEN_YEAR: FraThirdSingleStageLeague = FraThirdSingleStageLeague { rounds: 33, file: 0x9b8e6c, row: 0xa6, table: &DEN_PRM_EVEN_YEAR_ROUNDS, branch: "mwc_3 den prm schedule" };

pub(crate) const DEN_PRM_DIVISION: NationDivision = NationDivision {
    vtable: 0x9696e4,
    bench: None,
    discipline: discipline_rule::DENMARK, // mwc5: the Danish Superliga discipline rule
    stages: 0,
    set_up_rules: 0x553a00,
    create_table: 0x5538d0,
    source_file: 0x9b8e6c,
    box_lines: [0x50, 0x57, 0x5e, 0x67, 0x6e],
};

/// The Danish Superliga in a season starting in an odd year (2001/02): the spring from round 18 differs.
pub(crate) const DEN_PRM_ODD_YEAR_ROUNDS: [FraThirdLeagueRound; 33] = [
    (27, 6, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 0: 27 Jul
    (3, 7, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 1: 3 Aug
    (10, 7, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 2: 10 Aug
    (17, 7, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 3: 17 Aug
    (24, 7, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 4: 24 Aug
    (31, 7, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 5: 31 Aug (mwc3 den_prm)
    (7, 8, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 6: 7 Sep
    (14, 8, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 7: 14 Sep
    (21, 8, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 8: 21 Sep
    (28, 8, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 9: 28 Sep
    (5, 9, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 10: 5 Oct
    (12, 9, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 11: 12 Oct
    (19, 9, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 12: 19 Oct
    (26, 9, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 13: 26 Oct
    (2, 10, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 14: 2 Nov
    (9, 10, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 15: 9 Nov
    (16, 10, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 16: 16 Nov
    (23, 10, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 17: 23 Nov
    (1, 2, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 18: 1 Mar (+1 year)
    (4, 2, 1, 2, 2, &[(-1, -1, -1)]), // 19: 4 Mar (+1 year)
    (8, 2, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 20: 8 Mar (+1 year)
    (11, 3, 1, 2, 2, &[(-1, -1, -1)]), // 21: 11 Apr (+1 year)
    (15, 2, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 22: 15 Mar (+1 year)
    (22, 2, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 23: 22 Mar (+1 year)
    (29, 2, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 24: 29 Mar (+1 year)
    (5, 3, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 25: 5 Apr (+1 year)
    (12, 3, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 26: 12 Apr (+1 year)
    (19, 3, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 27: 19 Apr (+1 year)
    (26, 3, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 28: 26 Apr (+1 year)
    (3, 4, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 29: 3 May (+1 year)
    (10, 4, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 30: 10 May (+1 year)
    (17, 4, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 31: 17 May (+1 year)
    (24, 4, 1, 6, 1, &[(-1, -1, -1)]), // 32: 24 May (+1 year)
];

/// The Danish Superliga in a season starting in an even year.
pub(crate) const DEN_PRM_EVEN_YEAR_ROUNDS: [FraThirdLeagueRound; 33] = [
    (27, 6, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 0: 27 Jul
    (3, 7, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 1: 3 Aug
    (10, 7, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 2: 10 Aug
    (17, 7, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 3: 17 Aug
    (24, 7, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 4: 24 Aug
    (31, 7, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 5: 31 Aug
    (7, 8, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 6: 7 Sep
    (14, 8, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 7: 14 Sep
    (21, 8, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 8: 21 Sep
    (28, 8, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 9: 28 Sep
    (5, 9, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 10: 5 Oct
    (12, 9, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 11: 12 Oct
    (19, 9, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 12: 19 Oct
    (26, 9, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 13: 26 Oct
    (2, 10, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 14: 2 Nov
    (9, 10, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 15: 9 Nov
    (16, 10, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 16: 16 Nov
    (23, 10, 0, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 17: 23 Nov
    (15, 2, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 18: 15 Mar (+1 year)
    (22, 2, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 19: 22 Mar (+1 year)
    (29, 2, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 20: 29 Mar (+1 year)
    (5, 3, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 21: 5 Apr (+1 year)
    (12, 3, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 22: 12 Apr (+1 year)
    (19, 3, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 23: 19 Apr (+1 year)
    (26, 3, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 24: 26 Apr (+1 year)
    (3, 4, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 25: 3 May (+1 year)
    (10, 4, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 26: 10 May (+1 year)
    (17, 4, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 27: 17 May (+1 year)
    (24, 4, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 28: 24 May (+1 year)
    (28, 4, 1, 2, 2, &[(-1, -1, -1)]), // 29: 28 May (+1 year)
    (31, 4, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 30: 31 May (+1 year)
    (7, 5, 1, 6, 1, &[(6, 2, 1), (0, 2, 1), (-1, -1, -1)]), // 31: 7 Jun (+1 year)
    (14, 5, 1, 6, 1, &[(-1, -1, -1)]), // 32: 14 Jun (+1 year)
];
