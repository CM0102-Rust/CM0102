// ---------- mwb_22: the Welsh League Cup (wel_lge_cup.cpp): the entrants, the predicted positions and the season turnover ----------

/// The body of `wel_lge_cup_collect_entrants`; returns (the routine's return, the entries to compare).
///
/// `ita_cup_fields::ENTRANT_CLUB_COUNT` = 18; the pool (`exe::crt_calloc`(4, 18) into `ita_cup_fields::ENTRANT_CLUBS`;
/// none: the box of line 0x2cd and 0). One pass over the clubs (`G_CLUB_COUNT`, a signed word index, in table order):
/// each club whose `club::DIVISION` is the League of Wales ([`KEY_CLUB_COMP_WELSH_PREMIER_DIVISION`]) goes into the
/// next entry (no bound check, as the original). The entries found are sorted by reputation (`exe::crt_qsort`(pool, n,
/// 4, 0x4b07d0), cdecl) and each of the six bands of three is shuffled (`exe::comp_util_shuffle_pointer_array`(band,
/// 3)); with fewer than 18 found, the box of line 0x2e7 (and still 1). Returns 1.
pub(crate) unsafe fn collect_welsh_league_cup_entrants(this: League) -> (i32, i32) {
    let t = this.addr();
    write_i32(t + ita_cup_fields::ENTRANT_CLUB_COUNT, 0x12);
    let v = exe::crt_calloc(4, 0x12);
    write_usize(t + ita_cup_fields::ENTRANT_CLUBS, v);
    if v == 0 {
        show_error_at(0xad56f4, 0x2cd);
        return (0, 0);
    }
    let mut n = 0u32;
    let mut i = 0i16;
    while (i as i32) < read_i32(G_CLUB_COUNT) {
        let c = club_at(i as isize);
        let d = c.division();
        if !d.is_null() && read_usize(d.addr()) == read_usize(KEY_CLUB_COMP_WELSH_PREMIER_DIVISION) {
            write_usize(read_usize(t + ita_cup_fields::ENTRANT_CLUBS) + 4 * n as usize, c.addr());
            n += 1;
        }
        i = i.wrapping_add(1);
    }
    covered!("mwb_22 welsh league cup entrants");
    exe::crt_qsort(read_usize(t + ita_cup_fields::ENTRANT_CLUBS), n, 4, 0x4b07d0usize); // mwb22: the entrants by reputation
    for b in 0..6usize {
        exe::comp_util_shuffle_pointer_array(read_usize(t + ita_cup_fields::ENTRANT_CLUBS) + 0xc * b, 3);
    }
    if (n as i32) < read_i32(t + ita_cup_fields::ENTRANT_CLUB_COUNT) {
        show_error_at(0xad56f4, 0x2e7);
    }
    (1, 0x12)
}

/// Rule: the Welsh League Cup takes the 18 clubs of the League of Wales, ranked by reputation and shuffled within each
/// band of three (the band is a draw pot: the six groups take one club from each of the three pots [inference]); fewer
/// than 18 clubs in the league shows the error box.
/// Original: `WelLgeCup::collect_entrants` (0x927010, wel_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wel_lge_cup_collect_entrants(this: usize) -> i32 {
    collect_welsh_league_cup_entrants(League(this)).0
}

/// Comparer for `wel_lge_cup_collect_entrants` (side by side with the random generator restored: `compare_select_entrants`).
pub(crate) unsafe extern "thiscall" fn compare_wel_lge_cup_collect_entrants(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I0%%);
    compare_select_entrants("wel_lge_cup_collect_entrants", this, || o(this), || collect_welsh_league_cup_entrants(League(this)))
}

/// Rule: the Welsh League Cup's predicted positions (at creation and at the turnover): the 18 entrants by reputation;
/// the knockout (stage 5) gives 1 to the winner and 2 to the runner-up, then a shared place to the losers of each
/// earlier round; the rest in bands of 6; the board and press expectation of each club [inference].
///
/// `italian_cup_predicted_positions` with the knockout stage 5, bands of 6 and the boxes at lines 0x3b5 and 0x3bd of
/// wel_lge_cup.cpp (read instruction by instruction against ItaCCup::set_predicted_positions: only the file, the lines,
/// the stage and the band change).
/// Original: `WelLgeCup::set_predicted_positions` (0x9274f0, wel_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wel_lge_cup_set_predicted_positions(this: usize) {
    italian_cup_predicted_positions(this, 0xad56f4, (0x3b5, 0x3bd), 5, 6, "mwb_22 welsh league cup predicted") // mwb22: the Welsh League Cup bands of 6
}

/// Comparer for `wel_lge_cup_set_predicted_positions` (`compare_italian_cup_predicted`, side by side).
pub(crate) unsafe extern "thiscall" fn compare_wel_lge_cup_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    compare_italian_cup_predicted("wel_lge_cup_set_predicted_positions", this, wel_lge_cup_set_predicted_positions, || o(this));
}

/// Rule: at the season turn the Welsh League Cup is rebuilt: the table, the round table, the entrants and the stages of
/// the last edition are freed, the statistics archived, the year moves on; then the 18 entrants, the predicted
/// positions, the rules, the first group's table, its fixtures and order and the stages of the five other groups.
///
/// `start_irish_league_cup_season` with WelLgeCup::collect_entrants (0x927010), create_table (0x926700),
/// create_group_stage (0x926b00), five groups and the error box lines 0x1f4, 0x1fb, 0x201 and 0x208 of wel_lge_cup.cpp
/// (read instruction by instruction against IreLgeCup::start_new_season: only the routines, the group count, the file
/// and the lines change).
/// Original: `WelLgeCup::start_new_season` (0x9268a0, wel_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wel_lge_cup_start_new_season(this: usize) -> u32 {
    start_irish_league_cup_season(League(this), 0x927010, 0x926700, 0x926b00, 5, 0xad56f4, [0x1f4, 0x1fb, 0x201, 0x208], 1) // mwb22: the Welsh League Cup turnover
}

/// Comparer for `wel_lge_cup_start_new_season` (original only: `compare_state_league_new_season`, the year up by one).
pub(crate) unsafe extern "thiscall" fn compare_wel_lge_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    compare_state_league_new_season("wel_lge_cup_start_new_season", League(this), || o(this))
}
