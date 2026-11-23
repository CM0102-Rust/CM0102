use super::*;

/// Rule: the League of Ireland Premier Division plays 36 matchdays (34 when the clubs meet three times), from 15
/// August to 23 April of the next year, almost all on Saturdays [inference: weekday 6], with two midweek rounds in a
/// 36-round season (17 August and 8 February) and one on 7 March; the Christmas round is on 26 December, or midweek on
/// 22 December when Christmas or Christmas Eve is the Saturday; only the league stage.
///
/// Only stage −1 (the byte; otherwise 0 with nothing written): `irish_league_schedule` with the boxes at lines 0xad and
/// 0x156 of ire_prm.cpp.
/// Original: `IrePrm::create_schedule` (0x639e90, ire_prm.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn ire_prm_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    if stage as u8 != 0xff {
        return 0;
    }
    irish_league_schedule(League(this), rounds, extra, out, 0x9ea080, (0xad, 0x156), "mwa_27 irish premier schedule")
}

/// Comparer for `ire_prm_create_schedule` (side by side: `compare_c1_schedule`).
pub(crate) unsafe extern "thiscall" fn compare_ire_prm_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_IRE_PRM_CREATE_SCHEDULE);
    compare_c1_schedule("ire_prm_create_schedule", stage, rounds, extra, out, || o(this, stage, rounds, extra, out), |r, e, l| ire_prm_create_schedule(this, stage, r, e, l))
}

/// Rule: the Irish turnover is led by the Premier Division: promotion and relegation, the statistics archived, the next
/// year, the rules, the table, the fixtures and the order rebuilt; then the First Division turns over, and the predicted
/// positions, the board expectations and the referee renewal follow.
///
/// `start_irish_premier_season` with the chain argument 0, IrePrm::create_table (0x63a740), the Irish First Division and
/// the error box lines 0x1f3, 0x1fa, 0x200 and 0x207 of ire_prm.cpp (checked instruction by instruction against
/// NirPrm's).
/// Original: `IrePrm::start_new_season` (0x63a9b0, ire_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ire_prm_start_new_season(this: usize) -> u32 {
    start_irish_premier_season(League(this), 0, 0x63a740, KEY_CLUB_COMP_IRISH_FIRST_DIVISION, 0x9ea080, [0x1f3, 0x1fa, 0x200, 0x207], 1) // mwa15: the Irish turnover
}

pub(crate) unsafe extern "thiscall" fn compare_ire_prm_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_IRE_PRM_START_NEW_SEASON);
    compare_state_league_new_season("ire_prm_start_new_season", League(this), || o(this))
}

/// Rule: the Irish Premier Division at career creation: the year and the database competition, the Irish discipline
/// rules, its ten own slots cleared and, with nothing saved, a plain league: the rules, the table, the fixtures, the
/// order, the statistics and the predicted positions.
///
/// `construct_irish_league` with `IRISH_PREMIER_CONSTRUCTOR` (checked instruction by instruction against JapJ2::JapJ2:
/// the same steps with the ten 5-byte slots from +0xee set to −1 before loading).
/// Original: `IrePrm::IrePrm` (0x639bb0, ire_prm.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_ire_prm(this: usize, year: u32, record: usize) -> usize {
    construct_irish_league(League(this), year, record, &IRISH_PREMIER_CONSTRUCTOR, "mwa_11 irish premier new") // mwa11: the Irish Premier
}

pub(crate) unsafe extern "thiscall" fn compare_construct_ire_prm(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_IRE_PRM);
    compare_irish_league_constructor("construct_ire_prm", League(this), year, record, &IRISH_PREMIER_CONSTRUCTOR, || o(this, year, record))
}

/// Rule: the Irish Premier Division: 3 points for a win and 1 for a draw, ties split by goal difference and goals for;
/// the last goes down to the First Division and the second last plays the play-off (with twelve clubs, three go down
/// and one plays the play-off); 5 substitutes on the bench and 3 substitutions; no division above.
///
/// `set_up_irish_league_rules` with `IRISH_PREMIER_RULES`. Returns 1.
/// Original: `IrePrm::set_up_rules` (0x63a900, ire_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ire_prm_set_up_rules(this: usize) -> u32 {
    covered!("mwa_11 irish premier rules");
    set_up_irish_league_rules(League(this), &IRISH_PREMIER_RULES)
}

pub(crate) unsafe extern "thiscall" fn compare_ire_prm_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_IRE_PRM_SET_UP_RULES);
    compare_irish_league_rules("ire_prm_set_up_rules", this, &IRISH_PREMIER_RULES, || o(this))
}

/// Rule: the Irish Premier Division table takes the division's clubs in database order; only ten (meeting four times)
/// or twelve (three times, with three relegated and one in the play-off) are accepted; when full, the prize fund for a
/// 30,000 share per place, paid by table position at the season's end.
/// Original: `IrePrm::create_table` (0x63a740, ire_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ire_prm_create_table(this: usize) -> u32 {
    create_irish_premier_table(League(this))
}

pub(crate) unsafe extern "thiscall" fn compare_ire_prm_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_IRE_PRM_CREATE_TABLE);
    compare_irish_counted_table("ire_prm_create_table", League(this), KEY_CLUB_COMP_IRISH_PREMIER_DIVISION, |n| match n { 0xa => Some(4), 0xc => Some(3), _ => None }, Some(0x7530), || o(this))
}

/// The turnover of an Irish Premier Division, which leads its nation's league turnover.
///
/// `league::SEASON_CLOSED` = 0; the promotion and relegation chain (the virtual
/// `league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN`(`chain`), thiscall, ret 4: 1 in Northern Ireland, 0 in Ireland);
/// the table freed (League::free_table `exe::league_free_table`(this, 0)); the round table (`league::ROUND_DATES`) freed
/// (`exe::crt_free`) and zeroed; the season archived in the statistics (Comp_Stats::archive_season_and_reset
/// `exe::comp_stats_archive_season_and_reset`(stats, 1), when there); the year (`league::YEAR`) up by one. Then, each
/// with its own error box (returning 0) when it returns 0: the rules (`comp_vtable::SET_UP_RULES`), the table (`table`,
/// thiscall, ret 0), the fixtures (`exe::league_create_fixtures`) and the order (`exe::league_sort_table`(this, 0)).
/// Then the First Division's turnover (the competition object of the key global `first`, its
/// `comp_vtable::START_NEW_SEASON`), the predicted positions (`comp_vtable::SET_PREDICTED_POSITIONS`), the board
/// expectations and the referee renewal (`set_expectations_and_renew_officials`, whose return it passes on).
pub(crate) unsafe fn start_irish_premier_season(this: League, chain: u32, table: usize, first: usize, file: usize, rows: [u32; 4], year: i16) -> u32 {
    this.set_season_closed(0);
    let promote: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(read_usize(read_usize(this.addr()) + league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN));
    promote(this.addr(), chain);
    exe::league_free_table(this, 0);
    let l = this.round_dates();
    if l != 0 {
        exe::crt_free(l);
        this.set_round_dates(0);
    }
    let stats = this.comp_stats();
    if stats != 0 {
        exe::comp_stats_archive_season_and_reset(stats, 1);
    }
    this.set_year(this.year().wrapping_add(year));
    // simplification: in the boxes the original returns the box's eax; the caller of the turnover does not read it
    if bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_UP_RULES) == 0 {
        show_error_at(file, rows[0]);
        return 0;
    }
    let table: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(table);
    if table(this.addr()) == 0 {
        show_error_at(file, rows[1]);
        return 0;
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(file, rows[2]);
        return 0;
    }
    if exe::league_sort_table(this, 0usize) == 0 {
        show_error_at(file, rows[3]);
        return 0;
    }
    covered!("mwa_15 irish premier turnover");
    bra_nat_first_call_virtual_no_args(comp_by_global(first), comp_vtable::START_NEW_SEASON); // mwa15: the First Division turns over with the Premier
    bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_PREDICTED_POSITIONS);
    set_expectations_and_renew_officials(this)
}

/// The body of `ire_prm_create_table`.
///
/// `league::CLUB_COUNT` = 0 and `league::GAMES_PER_CLUB` = −1; the division's clubs are counted into
/// `league::CLUB_COUNT`. Only ten or twelve are accepted (otherwise 0): ten meet four times (`league::LEGS` = 4), twelve
/// three times. The table (`ita_ser_a_fill_league_table`: n × 0x3b bytes; out of memory, the error box at line 0x186 of
/// ire_prm.cpp and 0) with the clubs in database order; with twelve rows filled, one club to the relegation play-off and
/// three relegated; full, the prize fund for a 30,000 share per place (`set_prize_fund`) and 1; otherwise 0.
pub(crate) unsafe fn create_irish_premier_table(this: League) -> u32 {
    this.set_club_count(0);
    this.set_games_per_club(-1);
    let n = irish_division_club_count(KEY_CLUB_COMP_IRISH_PREMIER_DIVISION);
    this.set_club_count(n);
    match n {
        0xa => this.set_legs(4),
        0xc => this.set_legs(3),
        _ => return 0,
    }
    let full = match ita_ser_a_fill_league_table(this, n, 0x9ea080, 0x186, KEY_CLUB_COMP_IRISH_PREMIER_DIVISION) {
        Some(f) => f,
        None => return 0,
    };
    if full && n == 0xc {
        covered!("mwa_11 irish premier of twelve");
        this.set_relegation_playoff_count(1);
        this.set_relegated_count(3); // mwa11: three go down from a twelve-club Premier
    }
    if !full {
        return 0;
    }
    covered!("mwa_11 irish premier table");
    set_prize_fund(this, 0x7530); // mwa11: the Irish Premier quota
    1
}

/// The Irish Premier Division's constructor: the vtable 0x96c014, discipline rule `discipline_rule::IRELAND`, no extra
/// stage, the ten slots from +0xee reset, IrePrm::set_up_rules (0x63a900) and IrePrm::create_table (0x63a740), the
/// lines 0x4b, 0x52, 0x59, 0x62 and 0x69 of ire_prm.cpp.
pub(crate) const IRISH_PREMIER_CONSTRUCTOR: IrishLeagueConstructor = IrishLeagueConstructor { vtable: 0x96c014, discipline: discipline_rule::IRELAND,
    play_off_stage: false, reset_ten_slots: true, rules: 0x63a900, table: 0x63a740, file: 0x9ea080, rows: [0x4b, 0x52, 0x59, 0x62, 0x69] };

/// The Irish Premier Division's rules: three meetings (`league::LEGS` = 3; the table sets 4 with ten clubs), 3 points
/// for a win and 1 for a draw, match category 2, `league::MATCH_RULES` and `league::ROUND_DRAW_RULES` 0, club sides,
/// the tie-breaks 1 and 2 (goal difference, goals for), no promotion, one club to the relegation play-off and one
/// relegated (the table sets 1 and 3 with twelve clubs), the squad flags 0, 5 substitutes on the bench and 3
/// substitutions, `league::LEAGUE_FLAGS` 0x103; no division above, the First Division below.
pub(crate) const IRISH_PREMIER_RULES: IrishLeagueRules = IrishLeagueRules {
    writes: &[(league::LEGS, 2, 3), (league::WIN_POINTS, 1, 3), (league::DRAW_POINTS, 1, 1), (league::MATCH_CATEGORY, 1, 2), (league::MATCH_RULES, 2, 0),
        (league::ROUND_DRAW_RULES, 2, 0), (league::CLUB_SIDES, 1, 1), (league::TIEBREAKS, 1, 1), (league::TIEBREAKS + 1, 1, 2), (league::TIEBREAKS + 2, 1, 0),
        (league::PROMOTED_COUNT, 1, 0), (league::PROMOTION_PLAYOFF_COUNT, 1, 0), (league::RELEGATION_PLAYOFF_COUNT, 1, 1), (league::RELEGATED_COUNT, 1, 1), // mwa11: one relegated, one in the play-off
        (league::SQUAD_FLAGS, 1, 0), (league::SUBSTITUTIONS_ALLOWED, 1, 3), (league::SUBSTITUTES_NAMED, 1, 5), (league::LEAGUE_FLAGS, 2, 0x103)],
    upper: None,
    lower: Some(KEY_CLUB_COMP_IRISH_FIRST_DIVISION),
};
