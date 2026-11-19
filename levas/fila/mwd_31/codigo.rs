// ---------- batch %%LEVA%%: the Turkish second division category B turnover and stages, and the Polish First Division stages (tur_second_b.cpp, pol_first.cpp) ----------

/// Rule: the Turkish second division category B turnover (called by the First Division's): the season reopened, last
/// season's stages destroyed, the next year; the rules, the table, the order and the fixtures (in this order); the
/// four group stages built again; the predicted positions and the officials (no board expectations).
///
/// `league::SEASON_CLOSED` = 0; League::free_table (`exe::league_free_table`(0)); the round dates freed and cleared;
/// with statistics, `exe::comp_stats_archive_season_and_reset`(1); with a current stage ≥ 0, the stages 0..=current
/// (re-read on each step) destroyed by their virtual 0 with 1; the year + 1 and the current stage −1. Then, each with
/// the box of tur_second_b.cpp (0xaa18b8) at its line and 0 returned when it returns 0: the rules (virtual
/// `comp_vtable::SET_UP_RULES`, 0x233), TurSecondB::create_table (0x9002c0, thiscall, ret 0; 0x23a), League::sort_table
/// (`exe::league_sort_table`(0), 0x241) and League::create_fixtures (0x247). Then TurSecondB::create_group_stage(i)
/// (0x900780, push i -> thiscall, ret 4) for i = 0..3, the predicted positions (virtual
/// `comp_vtable::SET_PREDICTED_POSITIONS`) and OfficialsManager::renew_officials on `G_OFFICIALS_MANAGER` with the
/// record, whose eax is returned.
/// Original: `TurSecondB::start_new_season` (0x900500, tur_second_b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn tur_second_b_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
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
    if (this.current_stage() as i32) >= 0 {
        let mut i = 0i32;
        loop {
            let f = read_usize(this.stages() + 4 * i as usize);
            if f != 0 {
                let destroy: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(f)));
                destroy(f, 1);
            }
            i += 1;
            if i > this.current_stage() as i32 {
                break;
            }
        }
    }
    this.set_year(this.year().wrapping_add(1));
    write_i32(this.addr() + league::CURRENT_STAGE, -1);
    let file = 0xaa18b8;
    if bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_UP_RULES) == 0 {
        show_error_at(file, 0x233);
        return 0;
    }
    let table: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x9002c0usize);
    if table(this.addr()) == 0 {
        show_error_at(file, 0x23a);
        return 0;
    }
    if exe::league_sort_table(this, 0) == 0 {
        show_error_at(file, 0x241);
        return 0;
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(file, 0x247);
        return 0;
    }
    covered!("mwd_31 turkish category b turnover");
    let group: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x900780usize);
    for i in 0..4 {
        group(this.addr(), i);
    }
    bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_PREDICTED_POSITIONS);
    exe::officialsmanager_renew_officials(read_usize(G_OFFICIALS_MANAGER), this.competition().addr()) as u32
}

/// Rule: the next stage of the Turkish second division category B: past the last stage, the error box; otherwise the
/// current stage goes up by one; on reaching stage 4 the promotion group and the classification groups are built, on
/// reaching stage 10 the play-off.
///
/// At the last stage (`league::CURRENT_STAGE` ≥ `league::STAGE_COUNT` − 1, signed) the box of tur_second_b.cpp at line
/// 0x265 and nothing else. Otherwise the current stage + 1; at 4, TurSecondB::create_promotion_group (0x900ab0) and
/// TurSecondB::create_classification_groups (0x900d00); at 10, TurSecondB::create_playoff_stage (0x901040) (all
/// thiscall, ret 0, not ported).
/// Original: `TurSecondB::advance_stage` (0x9006c0, tur_second_b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn tur_second_b_advance_stage(this: usize) {
    let l = League(this);
    let stage = l.current_stage() as i32;
    if stage >= (l.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(0xaa18b8, 0x265);
    }
    let stage = stage + 1;
    l.set_current_stage(stage as usize);
    covered!("mwd_31 turkish category b next stage");
    if stage == 4 { // mwd31: the promotion and classification groups after the four groups
        call_on_league(0x900ab0, l);
        call_on_league(0x900d00, l);
    } else if stage == 10 {
        call_on_league(0x901040, l);
    }
}

/// Rule: the next stage of the Polish First Division: past the last stage, the error box; otherwise the current stage
/// goes up by one; leaving stage 0 (reaching 1) the final stage is built.
///
/// At the last stage (`league::CURRENT_STAGE` ≥ `league::STAGE_COUNT` − 1, signed) the box of pol_first.cpp at line
/// 0x24f and nothing else. Otherwise the current stage + 1; when the old one was 0, PolFirst::create_final_stage
/// (0x7c9b60, thiscall, ret 0, not ported).
/// Original: `PolFirst::advance_stage` (0x7c9ad0, pol_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn pol_first_advance_stage(this: usize) {
    let l = League(this);
    let stage = l.current_stage() as i32;
    if stage >= (l.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(0xa78598, 0x24f);
    }
    l.set_current_stage((stage + 1) as usize);
    covered!("mwd_31 polish first division next stage");
    if stage == 0 {
        call_on_league(0x7c9b60, l);
    }
}

/// Side by side on the current stage of a league's advance_stage when it builds nothing (`builds`(new stage) false and
/// not at the last stage); otherwise only the original (a stage is built once).
pub(crate) unsafe fn compare_advance_stage_building(name: &str, this: League, builds: impl Fn(i32) -> bool, original: impl FnOnce(), copy: impl FnOnce()) {
    let stage = this.current_stage() as i32;
    if stage >= (this.stage_count() as i32).wrapping_sub(1) || builds(stage + 1) {
        return original();
    }
    let (_, _, same, mo, mr) = crate::comparison::side_by_side_light(&[(this.addr() + league::CURRENT_STAGE, 4)], original, copy);
    SCOREBOARD.record(same, || format!("{}({:#x}) original={:x?} rust={:x?}", name, this.addr(), mo, mr));
}

/// Comparer for `tur_second_b_start_new_season`: only the original runs (it frees and rebuilds the league); the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_tur_second_b_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    compare_state_league_new_season("tur_second_b_start_new_season", League(this), || o(this))
}

/// Comparer for `tur_second_b_advance_stage` (`compare_advance_stage_building`: stages 4 and 10 build).
pub(crate) unsafe extern "thiscall" fn compare_tur_second_b_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    compare_advance_stage_building("tur_second_b_advance_stage", League(this), |s| s == 4 || s == 10, || o(this), || tur_second_b_advance_stage(this))
}

/// Comparer for `pol_first_advance_stage` (`compare_advance_stage_building`: stage 1 builds).
pub(crate) unsafe extern "thiscall" fn compare_pol_first_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    compare_advance_stage_building("pol_first_advance_stage", League(this), |s| s == 1, || o(this), || pol_first_advance_stage(this))
}
