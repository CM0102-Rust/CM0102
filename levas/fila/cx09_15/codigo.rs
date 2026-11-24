// ---------- cx09_15: the English Premier Division turnover and the promotion and relegation chain of the Football League (eng_prm.cpp) ----------

/// The eng_prm.cpp source path, for the error boxes.
const ENG_PRM_SOURCE: usize = 0x9c43b0;

/// The English season turnover, led by the Premier Division: `league::SEASON_CLOSED` = 0; the promotion and relegation
/// chain (virtual `league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN` with 1); the Conference exchange
/// (EngPrm::exchange_conference_feeder_clubs 0x5750a0 when the Conference ([`KEY_CLUB_COMP_ENGLISH_CONFERENCE`]) is
/// loaded, otherwise EngPrm::exchange_conference_clubs_unplayed 0x574e60); League::free_table (`exe::league_free_table`
/// with 0); the round dates freed and zeroed; with statistics, `exe::comp_stats_archive_season_and_reset`(1); the year
/// + 1; then, each with the eng_prm.cpp box of its line and an early end: the rules (virtual
/// `comp_vtable::SET_UP_RULES`, 0x267), the table (EngPrm::create_table 0x574a20, 0x26e), the fixtures
/// (`exe::league_create_fixtures`, 0x274) and the order (`exe::league_sort_table`(0), 0x27b). Then the predicted
/// positions (virtual `comp_vtable::SET_PREDICTED_POSITIONS`); the division tier of the Third Division clubs capped
/// (`exe::league_limit_club_division_tier`(Third, 1, 3, −3, 1)) and, with the Conference, of its clubs (2, 0, −3, 1);
/// the First, Second and Third Divisions' turnovers (virtual `comp_vtable::START_NEW_SEASON`) and the Conference's
/// when loaded; the board expectations and the referee renewal, whose value is returned (0 after a box: the original
/// returns the box's eax, unread).
/// Original: `EngPrm::start_new_season` (0x574c10, eng_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn eng_prm_start_new_season(this: usize) -> u32 {
    let l = League(this);
    l.set_season_closed(0);
    let chain: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(read_usize(read_usize(this) + league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN));
    chain(this, 1);
    let exchange = if comp_by_global(KEY_CLUB_COMP_ENGLISH_CONFERENCE) != 0 { 0x5750a0usize } else { 0x574e60 };
    let exchange: unsafe extern "thiscall" fn(usize) = core::mem::transmute(exchange);
    exchange(this);
    exe::league_free_table(this, 0);
    if l.round_dates() != 0 {
        exe::crt_free(l.round_dates());
        l.set_round_dates(0);
    }
    if l.comp_stats() != 0 {
        exe::comp_stats_archive_season_and_reset(l.comp_stats(), 1);
    }
    l.set_year(l.year().wrapping_add(1));
    if bra_nat_first_call_virtual_no_args(this, comp_vtable::SET_UP_RULES) == 0 {
        show_error_at(ENG_PRM_SOURCE, 0x267);
        return 0;
    }
    let create_table: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x574a20usize);
    if create_table(this) == 0 {
        show_error_at(ENG_PRM_SOURCE, 0x26e);
        return 0;
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(ENG_PRM_SOURCE, 0x274);
        return 0;
    }
    if exe::league_sort_table(this, 0usize) == 0 {
        show_error_at(ENG_PRM_SOURCE, 0x27b);
        return 0;
    }
    covered!("cx09_15 english turnover");
    bra_nat_first_call_virtual_no_args(this, comp_vtable::SET_PREDICTED_POSITIONS);
    exe::league_limit_club_division_tier(comp_by_global(KEY_CLUB_COMP_ENGLISH_THIRD_DIVISION), 1, 3, -3, 1);
    let conference = comp_by_global(KEY_CLUB_COMP_ENGLISH_CONFERENCE);
    if conference != 0 {
        exe::league_limit_club_division_tier(conference, 2, 0, -3, 1);
    }
    for key in [KEY_CLUB_COMP_ENGLISH_FIRST_DIVISION, KEY_CLUB_COMP_ENGLISH_SECOND_DIVISION, KEY_CLUB_COMP_ENGLISH_THIRD_DIVISION, KEY_CLUB_COMP_ENGLISH_CONFERENCE] {
        let division = comp_by_global(key);
        if key != KEY_CLUB_COMP_ENGLISH_CONFERENCE || division != 0 {
            bra_nat_first_call_virtual_no_args(division, comp_vtable::START_NEW_SEASON);
        }
    }
    set_expectations_and_renew_officials(l)
}

/// The promotion and relegation chain of the Football League, run by the Premier Division: the final positions of the
/// Premier, First, Second and Third Divisions (virtual `league_vtable::RECORD_FINAL_POSITIONS`, in that order); the
/// exchanges Premier–First, First–Second and Second–Third (`exe::league_promote_and_relegate`(upper, lower, 1, `arg`,
/// −1, −1) on this league). With the Conference loaded: its final positions; then its champion (table row 0) must have
/// a ground fit for the Third Division (`exe::financetable_get_club_finance` on `G_FINANCE_TABLE`, then
/// `exe::finance_upgrade_stadium_for_league`(the Third Division's `league::REQUIRED_SEATS` and
/// `league::REQUIRED_CAPACITY`, signed words, its competition id, 1)): fit, the Third–Conference exchange as above;
/// otherwise the Third Division's last row (`league::CLUB_COUNT` − 1) gets `league_table_row::STATUS` 0xfe (it stays
/// up) and the override news (Comp::send_promotion_relegation_override_news on the Third Division with the Conference
/// record, the champion, 0 and 2).
/// Original: `EngPrm::process_promotion_relegation_chain` (0x5752e0, eng_prm.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn eng_prm_process_promotion_relegation_chain(this: usize, arg: u32) {
    let conference = comp_by_global(KEY_CLUB_COMP_ENGLISH_CONFERENCE);
    let third = League(comp_by_global(KEY_CLUB_COMP_ENGLISH_THIRD_DIVISION));
    let final_positions = |league: usize| {
        let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(league) + league_vtable::RECORD_FINAL_POSITIONS));
        f(league);
    };
    final_positions(this);
    for key in [KEY_CLUB_COMP_ENGLISH_FIRST_DIVISION, KEY_CLUB_COMP_ENGLISH_SECOND_DIVISION, KEY_CLUB_COMP_ENGLISH_THIRD_DIVISION] {
        final_positions(comp_by_global(key));
    }
    let ladder = [KEY_CLUB_COMP_ENGLISH_PREMIER_DIVISION, KEY_CLUB_COMP_ENGLISH_FIRST_DIVISION, KEY_CLUB_COMP_ENGLISH_SECOND_DIVISION, KEY_CLUB_COMP_ENGLISH_THIRD_DIVISION];
    for pair in ladder.windows(2) {
        exe::league_promote_and_relegate(this, comp_by_global(pair[0]), comp_by_global(pair[1]), 1, arg, -1, -1);
    }
    if conference == 0 {
        return;
    }
    final_positions(comp_by_global(KEY_CLUB_COMP_ENGLISH_CONFERENCE));
    let champion = read_usize(exe::league_get_table_row(conference, 0));
    let finance = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), champion);
    let seats = third.required_seats() as i16 as i32;
    let capacity = third.required_capacity() as i16 as i32;
    if exe::finance_upgrade_stadium_for_league(finance, seats, capacity, read_usize(third.competition().addr()), 1) != 0 {
        covered!("cx09_15 conference champion promoted");
        exe::league_promote_and_relegate(this, third, comp_by_global(KEY_CLUB_COMP_ENGLISH_CONFERENCE), 1, arg, -1, -1);
        return;
    }
    covered!("cx09_15 conference champion refused");
    write_u8(exe::league_get_table_row(third, third.club_count() as i32 - 1) + league_table_row::STATUS, 0xfe);
    let champion = read_usize(exe::league_get_table_row(conference, 0));
    exe::comp_send_promotion_relegation_override_news(third, comp_record_of_key(KEY_CLUB_COMP_ENGLISH_CONFERENCE), champion, 0usize, 2);
}

/// Comparer for `eng_prm_start_new_season`: only the original (it rebuilds the leagues); the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_eng_prm_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    compare_state_league_new_season("eng_prm_start_new_season", League(this), || o(this))
}

/// Comparer for `eng_prm_process_promotion_relegation_chain`: only the original (it moves the clubs).
pub(crate) unsafe extern "thiscall" fn compare_eng_prm_process_promotion_relegation_chain(this: usize, arg: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, u32)>(%%I1%%)(this, arg)
}
