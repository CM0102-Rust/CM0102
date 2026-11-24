// ---------- cx09_16: the Brazilian Serie B staff records and progress, and the Serie C stage advance (bra_nat_second.cpp, bra_nat_third.cpp) ----------

/// Updates the staff records by a club's outcome in the Brazilian Serie B, on the Staff_Records object
/// (`G_STAFF_RECORDS`) with this competition. In the league (stage 0xff): status 0 promotion (weight 0x64), status 3
/// relegation. In the play-off (stage 0), status 1: the club's row (League::find_club_row `exe::league_find_club_row`,
/// not checked: a club without a row would crash the original) and the champions record, then the row's status 5.
/// Returns 0.
/// Original: `BraNatSecond::update_staff_records_for_status` (0x42d040, bra_nat_second.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn bra_nat_second_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, _a4: u32, _a5: u32, _a6: u32) -> u32 {
    let l = League(this);
    let (records, comp) = (read_usize(G_STAFF_RECORDS), l.competition().addr());
    match (stage as u8, status as u8) {
        (0xff, 0) => {
            covered!("cx09_16 serie b promotion");
            exe::staff_records_add_promotion_record(records, club, comp, 0x64);
        }
        (0xff, 3) => exe::staff_records_add_relegation_record(records, club, comp),
        (0, 1) => {
            covered!("cx09_16 serie b champions");
            let row = exe::league_find_club_row(this, club, 0);
            exe::staff_records_add_league_champions_record(records, club, comp);
            LeagueTableRow(row).set_status(5);
        }
        _ => {}
    }
    0
}

/// The progress codes of the Brazilian leagues that only rewrite them in the league stage: the club's Comp_Stats entry
/// (`bra_reg_minas_gerais_comp_stats_club_entry`, unchecked); at stage 0xff a best code below `limit` becomes 1 and a
/// worst code below `limit` becomes `low_worst` (unsigned bytes); then the three codes (`write_progress_codes`).
/// Returns the entry.
unsafe fn league_stage_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32, limit: u8, low_worst: u8) -> usize {
    let e = bra_reg_minas_gerais_comp_stats_club_entry(this, club);
    let (c, mut b, mut w) = (current as u8, best as u8, worst as u8);
    if stage as u8 == 0xff {
        if b < limit {
            b = 1;
        }
        if w < limit {
            w = low_worst;
        }
    }
    write_progress_codes(e, c, b, w);
    e
}

/// The comparer of `league_stage_progress`: side by side on the entry's 3 bytes (`compare_group_championship_progress`),
/// returning the original's value.
unsafe fn compare_league_stage_progress(name: &str, this: usize, club: usize, original: impl FnOnce() -> usize, copy: impl FnOnce() -> usize) -> usize {
    let mut r = 0;
    compare_group_championship_progress(name, this, club, || r = original(), || {
        copy();
    });
    r
}

/// Stores a club's progress in the Brazilian Serie B (`league_stage_progress`: in the league a best code below 5 becomes
/// 1 and a worst code below 5 becomes 4).
/// Original: `BraNatSecond::set_club_progress` (0x42d260, bra_nat_second.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn bra_nat_second_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) -> usize {
    covered!("cx09_16 serie b progress");
    league_stage_progress(this, club, stage, current, best, worst, 5, 4)
}

/// Moves the Brazilian Serie C to its next stage: past the last stage (`league::CURRENT_STAGE` at or past
/// `league::STAGE_COUNT` − 1) the bra_nat_third.cpp box of line 0x2b7; otherwise the current stage goes up by one and
/// stage 7 builds the second round (0x42e430), stage 8 the third-round groups (0x42e700) and stage 10 the final
/// (0x42e980), through their detours; stage 9 builds nothing.
/// Original: `BraNatThird::advance_stage` (0x42e380, bra_nat_third.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_nat_third_advance_stage(this: usize) {
    let l = League(this);
    if l.current_stage() as i32 >= l.stage_count() as i32 - 1 {
        return show_error_at(0x98aba4, 0x2b7); // bra_nat_third.cpp
    }
    l.set_current_stage((l.current_stage() as i32 + 1) as usize);
    let build = match l.current_stage() as i32 {
        7 => 0x42e430usize,
        8 => 0x42e700,
        10 => 0x42e980,
        _ => return,
    };
    covered!("cx09_16 serie c next stage");
    let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(build);
    f(this);
}

/// Comparer for `bra_nat_second_update_staff_records_for_status`: only the original (it adds the records); it must
/// return 0 and, for the play-off champion, leave the row with status 5.
pub(crate) unsafe extern "thiscall" fn compare_bra_nat_second_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, a4: u32, a5: u32, a6: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) -> u32>(%%I0%%)(this, club, status, stage, a4, a5, a6);
    let row_ok = (stage as u8, status as u8) != (0, 1) || LeagueTableRow(exe::league_find_club_row(this, club, 0)).status() == 5;
    SCOREBOARD.record(r == 0 && row_ok, || format!("bra_nat_second_update_staff_records_for_status({:#x}, {:#x}, status {}, stage {:#x}): returned {}, row ok {}", this, club, status as u8, stage as u8, r, row_ok));
    r
}

pub(crate) unsafe extern "thiscall" fn compare_bra_nat_second_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) -> usize>(%%I1%%);
    compare_league_stage_progress("bra_nat_second_set_club_progress", this, club, || o(this, club, stage, current, best, worst),
        || bra_nat_second_set_club_progress(this, club, stage, current, best, worst))
}

/// Comparer for `bra_nat_third_advance_stage` (`compare_state_league_advance_stage`).
pub(crate) unsafe extern "thiscall" fn compare_bra_nat_third_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    compare_state_league_advance_stage("bra_nat_third_advance_stage", League(this), || o(this))
}
