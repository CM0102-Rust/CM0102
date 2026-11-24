// ---------- cx06_13: the European Championship qualifiers' new season, season stop and finals record (euro_champ_qual.cpp) ----------

/// Starts the qualifiers' next edition: `season_closed` 0, the table freed (`exe::league_free_table` with 0), the round
/// dates freed and cleared, every stage up to the current one destroyed (virtual 0 with 1; the stage vector is kept),
/// `.current_stage()` −1, the stored group results emptied (`exe::llist_clear`), the entrants freed and cleared with
/// their count; the edition `year()` moved on by 4 until it is not before the current year (`G_CURRENT_DATE_YEAR`);
/// `active`, `stopped` and the qualified flag (+0xfb) 0; the statistics archived (`exe::comp_stats_archive_season_and_reset`
/// with 1) and the rules set up again (`comp_vtable::SET_UP_RULES`; 0: the box at line 0x378 and 0). Returns what the
/// rules returned.
/// Original: `EuroChampQual::start_new_season` (0x57eb20, euro_champ_qual.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn euro_champ_qual_start_new_season(this: usize) -> u32 {
    let l = League(this);
    covered!("cx06_13 qualifiers new season");
    l.set_season_closed(0);
    exe::league_free_table(this, 0);
    if l.round_dates() != 0 {
        exe::crt_free(l.round_dates());
        l.set_round_dates(0);
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
    l.set_current_stage(0xffff_ffff);
    if read_usize(this + 0x10) != 0 { // the stored group results
        exe::llist_clear(read_usize(this + 0x10));
    }
    if read_usize(this + 0xee) != 0 { // the entrants
        exe::crt_free(read_usize(this + 0xee));
        write_usize(this + 0xee, 0);
        write_u32(this + 0xf2, 0);
    }
    while (l.year() as i32) < read_u16(G_CURRENT_DATE_YEAR) as i32 {
        l.set_year(l.year().wrapping_add(4));
    }
    l.set_active(0);
    l.set_stopped(0);
    write_u32(this + 0xfb, 0); // the qualified flag
    if l.comp_stats() != 0 {
        exe::comp_stats_archive_season_and_reset(l.comp_stats(), 1);
    }
    let set_up_rules: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::SET_UP_RULES));
    let r = set_up_rules(this);
    if r == 0 {
        show_error_at(0x9c4674, 0x378); // euro_champ_qual.cpp
    }
    r
}

/// Stops the qualifiers at the season's turn (`stopped` 1) and returns 1, so that the daily pass does not restart the
/// competition while it spans two seasons.
/// Original: `EuroChampQual::skip_season_reset` (0x57eb10, euro_champ_qual.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn euro_champ_qual_skip_season_reset(this: usize) -> u32 {
    covered!("cx06_13 qualifiers season stop");
    League(this).set_stopped(1);
    1
}

/// The competition record of the European Championship finals, the competition the qualifiers feed
/// (`comp_record_of_key` with `KEY_CLUB_COMP_EUROPEAN_FOOTBALL_CHAMPIONSHIP`; `this` is not read).
/// Original: `EuroChampQual::get_finals_comp` (0x57d060, euro_champ_qual.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn euro_champ_qual_get_finals_comp(_this: usize) -> usize {
    covered!("cx06_13 qualifiers finals record");
    comp_record_of_key(KEY_CLUB_COMP_EUROPEAN_FOOTBALL_CHAMPIONSHIP)
}

/// Comparer for `euro_champ_qual_start_new_season`: it frees the stages and lists (it cannot run twice), so only the
/// original runs; the plan: the edition not before the current year and on the 4-year step, `.current_stage()` −1, not
/// active and the qualified flag clear.
pub(crate) unsafe extern "thiscall" fn compare_euro_champ_qual_start_new_season(this: usize) -> u32 {
    let l = League(this);
    let before = l.year() as i32;
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%)(this);
    let year = l.year() as i32;
    let ok = (year == before || year >= read_u16(G_CURRENT_DATE_YEAR) as i32) && (year - before) % 4 == 0 && l.current_stage() as i32 == -1 && l.active() == 0
        && read_u32(this + 0xfb) == 0;
    SCOREBOARD.record(ok, || format!("euro_champ_qual_start_new_season({:#x}): year {} -> {}", this, before, year));
    r
}

/// Comparer for `euro_champ_qual_skip_season_reset`: side by side on `stopped`; the return values must match.
pub(crate) unsafe extern "thiscall" fn compare_euro_champ_qual_skip_season_reset(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    let (r, c, same, _, _) = crate::comparison::side_by_side_light(&[(this + league::STOPPED, 1)], || o(this), || euro_champ_qual_skip_season_reset(this));
    SCOREBOARD.record(same && r == c, || format!("euro_champ_qual_skip_season_reset({:#x}): {} x {}", this, r, c));
    r
}

/// Comparer for `euro_champ_qual_get_finals_comp`: side by side (it only reads); the records must match.
pub(crate) unsafe extern "thiscall" fn compare_euro_champ_qual_get_finals_comp(this: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I2%%)(this);
    let c = euro_champ_qual_get_finals_comp(this);
    SCOREBOARD.record(r == c, || format!("euro_champ_qual_get_finals_comp: original {:#x} rust {:#x}", r, c));
    r
}
