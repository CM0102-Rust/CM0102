// ---------- mwb_17: the Scottish League Cup (sco_lge_cup.cpp): the turnover ----------

/// rule: at the turnover the Scottish League Cup frees last season's teams, rounds and ties (zeroing the pointers),
/// archives the statistics, moves to the next year and goes back to 40 empty places (`exe::crt_calloc(0x28, 6)`; null:
/// the box of line 0x281 and 0), with no current round, not run by stages and the season not set up (the entrants come
/// in the day update); then the rounds (virtual +0x8c; negative: the box of line 0x288 and 0) and the fixture tree
/// (virtual +0x94; negative: 0x28f and 0), whose eax it returns (no predicted positions here)
/// Original: `ScoLgeCup::start_new_season` (0x7f0930, sco_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn sco_lge_cup_start_new_season(this: usize) -> u32 {
    let cup = Cup(this);
    write_u32(this + cup::SEASON_CLOSED, 0);
    for o in [cup::TEAM_TABLE, cup::ROUND_TABLE] {
        let l = read_usize(this + o);
        if l != 0 {
            exe::crt_free(l);
            write_u32(this + o, 0);
        }
    }
    if cup.tie_tables() != 0 {
        let mut i = 0i32;
        while i < cup.round_count() as i32 {
            let p = read_usize(cup.tie_tables() + i as usize * 4);
            if p != 0 {
                exe::crt_free(p);
                write_u32(cup.tie_tables() + i as usize * 4, 0);
            }
            i += 1;
        }
        exe::crt_free(cup.tie_tables());
        cup.set_tie_tables(0);
    }
    let stats = read_usize(this + league::COMP_STATS);
    if stats != 0 {
        exe::comp_stats_archive_season_and_reset(stats, 1);
    }
    cup.set_year(cup.year().wrapping_add(1)); // mwb17: the next League Cup
    covered!("mwb_17 scottish league cup turnover");
    write_u8(this + cup::CURRENT_ROUND, 0);
    write_u8(this + cup::RUN_BY_STAGES, 0);
    write_u32(this + cup::SEASON_SET_UP, 0);
    cup.set_team_count(0x28);
    let l = exe::crt_calloc(0x28, 6);
    cup.set_team_table(l);
    if l == 0 {
        show_error_at(0xa7af38, 0x281);
        return 0;
    }
    // simplification: in the boxes the original returns the box's eax (0x5e8290); the caller of the turnover does not
    // read it
    if call_cup_virtual(cup, comp_vtable::SET_UP_RULES) < 0 {
        show_error_at(0xa7af38, 0x288);
        return 0;
    }
    let r = call_cup_virtual(cup, cup_vtable::MAP_FIXTURE_TREE);
    if r < 0 {
        show_error_at(0xa7af38, 0x28f);
        return 0;
    }
    r as u32
}

/// Comparer for `sco_lge_cup_start_new_season`: only the original runs (it frees and reallocates); the year went up by
/// one.
pub(crate) unsafe extern "thiscall" fn compare_sco_lge_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    compare_cup_new_season("sco_lge_cup_start_new_season", Cup(this), || o(this))
}
