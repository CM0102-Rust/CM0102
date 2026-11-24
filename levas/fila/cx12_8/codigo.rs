// ---------- cx12_8: the J1 League season honours (jap_j1.cpp) ----------

/// Records the J1 League's season honours. When only two stages ran (`league::STAGE_COUNT` = 2: the same club won both
/// stages, no final), the history record (`exe::comp_util_add_history_record`(this, champion, runner-up, 0, 0)) with the
/// first stage's top club as champion and as runner-up the club second in whichever stage table has more points in its
/// second row (the first stage's on a tie; `exe::league_get_table_row`(stage, 1)). Otherwise the final stage (stage 2)
/// records them: its virtual `comp_vtable::RECORD_SEASON_HONOURS` (a tail jump in the original), whose value is
/// returned; the history path returns the record's value.
/// Original: `JapJ1::record_season_honours` (0x664390, jap_j1.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn jap_j1_record_season_honours(this: usize) -> usize {
    let l = League(this);
    let stages = l.stages();
    if l.stage_count() != 2 {
        let last = read_usize(stages + 8);
        let honours: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(read_usize(read_usize(last) + comp_vtable::RECORD_SEASON_HONOURS));
        return honours(last);
    }
    covered!("cx12_8 j1 won both stages");
    let (first, second) = (read_usize(stages), read_usize(stages + 4));
    let champion = read_usize(League(first).table());
    let second_row = LeagueTableRow(exe::league_get_table_row(second, 1));
    let first_row = LeagueTableRow(exe::league_get_table_row(first, 1));
    let runner_up_stage = if second_row.points() > first_row.points() { second } else { first }; // cx12_8: the better second place
    let runner_up = LeagueTableRow(exe::league_get_table_row(runner_up_stage, 1)).club() as usize;
    exe::comp_util_add_history_record(this, champion, runner_up, 0, 0)
}

/// Comparer for `jap_j1_record_season_honours`: only the original (it writes the history).
pub(crate) unsafe extern "thiscall" fn compare_jap_j1_record_season_honours(this: usize) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I0%%)(this)
}
