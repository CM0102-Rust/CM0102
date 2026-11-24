// ---------- cx13_11: the Russian Premier League's title play-off (rus_prm.cpp) ----------

/// Whether the top two rows of a league table are level on points (`league_table_row::POINTS`, words).
pub(crate) unsafe fn top_two_level(l: League) -> bool {
    read_i16(l.table() + league_table_row::POINTS) == read_i16(l.table() + league_table_row::SIZE + league_table_row::POINTS)
}

/// The next stage of the Russian Premier League: past the last stage, the rus_prm.cpp box of line 0x1ec; otherwise the
/// current stage goes up by one and, on reaching 0, the title play-off is built (`advance_league_stage` with
/// RusPrm::create_title_playoff_stage, through its address).
/// Original: `RusPrm::advance_stage` (0x7eba00, rus_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn rus_prm_advance_stage(this: usize) {
    advance_league_stage(League(this), 0xa7acc8, 0x1ec, 0x7eba90, "cx13_11 russian title play-off stage") // cx13_11: the title play-off
}

/// Comparer for `rus_prm_advance_stage` (`compare_stage_advance`: side by side on the current stage, only the
/// original at the last stage or when the play-off is built).
pub(crate) unsafe extern "thiscall" fn compare_rus_prm_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    compare_stage_advance("rus_prm_advance_stage", League(this), || o(this), || rus_prm_advance_stage(this))
}

/// The Russian title play-off. When the top two rows are level on points (`top_two_level`), their clubs are the pair;
/// the first row gets the worst possible position 2 (`league_table_row::WORST_POSSIBLE_POSITION`) and the second the
/// best possible 1 (`league_table_row::BEST_POSSIBLE_POSITION`), both the status 1. The pair count (the byte +0xee) is
/// 1 or 0.
///
/// Without a pair: no stages (`league::STAGE_COUNT` = 0, `league::CURRENT_STAGE` = −1), no tie-break matches
/// (`league::TIEBREAK_MATCHES` = 0), and the positions and statuses are worked out again (virtual
/// `league_vtable::UPDATE_POSITIONS_AND_STATUS`).
///
/// With the pair: the round table of stage 0 (virtual `comp_vtable::CREATE_SCHEDULE`(0, &rounds, &extra, null)); the
/// stage (operator new `exe::crt_operator_new` of 0xb2; Cup_Stage::Cup_Stage `FN_CUP_STAGE_CUP_STAGE`(this, 2, the pair,
/// the rounds, the competition record, the round table, the year, 0, 1, the extra, 0x14, 0, 0, 0, 0)) into the first
/// stage slot. Without a stage, the box of line 0x22e (the round table is not freed: a leak of the original);
/// otherwise the round table freed, the round set up (Cup::set_up_round `exe::cup_set_up_round` with 0), the current
/// stage 0 and the first tie-break code 6 (sort only, `league::TIEBREAKS`).
/// Original: `RusPrm::create_title_playoff_stage` (0x7eba90, rus_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn rus_prm_create_title_playoff_stage(this: usize) {
    let l = League(this);
    let mut pair = [0usize; 2];
    let mut n = 0u32;
    if top_two_level(l) {
        let t = l.table();
        pair = [read_usize(t), read_usize(t + league_table_row::SIZE)];
        n = 2;
        write_u8(t + league_table_row::WORST_POSSIBLE_POSITION, 2);
        write_u8(t + league_table_row::SIZE + league_table_row::BEST_POSSIBLE_POSITION, 1);
        write_u8(t + league_table_row::STATUS, 1);
        write_u8(t + league_table_row::SIZE + league_table_row::STATUS, 1);
    }
    write_u8(this + 0xee, (n / 2) as u8); // the play-off pair count
    if n == 0 {
        l.set_stage_count(0);
        l.set_current_stage(0xffff_ffff);
        l.set_tiebreak_matches(0);
        bra_nat_first_call_virtual_no_args(this, league_vtable::UPDATE_POSITIONS_AND_STATUS);
        return;
    }
    covered!("cx13_11 russian title play-off");
    let schedule: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut extra) = (0i16, 0i16);
    let table = schedule(this, 0, &mut rounds, &mut extra, 0);
    let p = exe::crt_operator_new(0xb2);
    let stage = if p != 0 {
        let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construct(p, this, n, pair.as_ptr() as usize, rounds as u16 as u32, l.competition().addr(), table, l.year() as u16 as u32, 0, 1, extra as u16 as u32, 0x14, 0, 0, 0, 0)
    } else {
        0
    };
    write_usize(l.stages(), stage);
    if read_usize(l.stages()) == 0 {
        return show_error_at(0xa7acc8, 0x22e); // rus_prm.cpp
    }
    exe::crt_free(table);
    exe::cup_set_up_round(read_usize(l.stages()), 0);
    l.set_current_stage(0);
    write_u8(this + league::TIEBREAKS, 6); // sort only
}

/// Comparer for `rus_prm_create_title_playoff_stage`: only the original (it builds the stage); the pair count
/// against the plan, and with a pair a stage in the first slot.
pub(crate) unsafe extern "thiscall" fn compare_rus_prm_create_title_playoff_stage(this: usize) {
    let l = League(this);
    let plan = top_two_level(l) as u8;
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this);
    let n = read_u8(this + 0xee);
    let ok = n == plan && (plan == 0 || read_usize(l.stages()) != 0);
    SCOREBOARD.record(ok, || format!("rus_prm_create_title_playoff_stage({:#x}) pairs {} plan {}", this, n, plan));
}

/// The Russian Premier League's daily update: League::update_for_date (`exe::league_update_for_date`) first, whose
/// result it returns. Then, when the league stage is complete (`league::COMPLETE`), it has no stages yet, `flag` is set
/// and a points tie was left to tie-break matches (`league::TIEBREAK_MATCHES`): the season is reopened
/// (`league::SEASON_CLOSED` = 0) and, with the top two level (`top_two_level`), one stage is made (`league::STAGES` =
/// malloc of 4, `league::STAGE_COUNT` = 1) and the league moves to it (virtual `comp_vtable::ADVANCE_STAGE`);
/// otherwise the tie is dropped (`league::COMPLETE` and `league::TIEBREAK_MATCHES` = 0) and the positions and statuses
/// are worked out again (virtual `league_vtable::UPDATE_POSITIONS_AND_STATUS`).
/// Original: `RusPrm::update_for_date` (0x7ec160, rus_prm.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn rus_prm_update_for_date(this: usize, date: usize, flag: u32) -> u8 {
    let l = League(this);
    let r = exe::league_update_for_date(this, date, flag);
    if l.complete() == 0 || l.stage_count() != 0 || flag == 0 || l.tiebreak_matches() == 0 {
        return r;
    }
    covered!("cx13_11 russian season tie");
    l.set_season_closed(0);
    if top_two_level(l) { // cx13_11: the title tie
        l.set_stage_count(1);
        l.set_stages(exe::crt_malloc(4));
        bra_nat_first_call_virtual_no_args(this, comp_vtable::ADVANCE_STAGE);
        return r;
    }
    l.set_complete(0);
    l.set_tiebreak_matches(0);
    bra_nat_first_call_virtual_no_args(this, league_vtable::UPDATE_POSITIONS_AND_STATUS);
    r
}

/// Comparer for `rus_prm_update_for_date`: only the original (the day's matches and news run inside it).
pub(crate) unsafe extern "thiscall" fn compare_rus_prm_update_for_date(this: usize, date: usize, flag: u32) -> u8 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u8>(%%I2%%)(this, date, flag)
}
