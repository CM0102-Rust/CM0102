// ---------- cx06_14: the Confederations Cup's new season (fifa_confed.cpp) ----------

/// Starts the next Confederations Cup: `season_closed` 0, the table freed (`exe::league_free_table` with 0), the round
/// dates freed and cleared, the statistics archived (`exe::comp_stats_archive_season_and_reset` with 1, when there are
/// any), every stage up to the current one destroyed (virtual 0 with 1; the stage vector is kept), the edition `year()`
/// two years on, `.current_stage()` −1, `active` 0 and the rules set up again (`comp_vtable::SET_UP_RULES`), whose
/// result it returns.
/// Original: `FifaConfed::start_new_season` (0x58bff0, fifa_confed.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn fifa_confed_start_new_season(this: usize) -> u32 {
    let l = League(this);
    covered!("cx06_14 confed new season");
    l.set_season_closed(0);
    exe::league_free_table(this, 0);
    if l.round_dates() != 0 {
        exe::crt_free(l.round_dates());
        l.set_round_dates(0);
    }
    if l.comp_stats() != 0 {
        exe::comp_stats_archive_season_and_reset(l.comp_stats(), 1);
    }
    let mut k = 0i32;
    while k <= l.current_stage() as i32 {
        let stage = read_usize(l.stages() + 4 * k as usize);
        if stage != 0 {
            let destroy: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(stage) + comp_vtable::SCALAR_DELETING_DESTRUCTOR));
            destroy(stage, 1);
        }
        k += 1;
    }
    l.set_year(l.year().wrapping_add(2));
    l.set_current_stage(0xffff_ffff);
    l.set_active(0);
    let set_up_rules: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::SET_UP_RULES));
    set_up_rules(this)
}

/// Comparer for `fifa_confed_start_new_season`: it frees the stages (it cannot run twice), so only the original runs;
/// the plan: the edition two years on, `.current_stage()` −1 and not active.
pub(crate) unsafe extern "thiscall" fn compare_fifa_confed_start_new_season(this: usize) -> u32 {
    let l = League(this);
    let before = l.year();
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%)(this);
    let ok = l.year() == before.wrapping_add(2) && l.current_stage() as i32 == -1 && l.active() == 0;
    SCOREBOARD.record(ok, || format!("fifa_confed_start_new_season({:#x}): year {} -> {}", this, before, l.year()));
    r
}
