// ---------- sbF_8: the Dutch Cup constructor and turnover (hol_cup.cpp) ----------

/// The Dutch Cup vtable.
pub(crate) const DUTCH_CUP_VTABLE: usize = 0x96b7b8;

/// Rule: the Dutch Cup is a cup played as a league with the cup's own group, 19 group stages and the knockout (20
/// stage slots), with 7 substitutes on the bench and 3 substitutions under the Dutch discipline rules; with nothing
/// saved, the statistics and the rules are made and the cup stays empty (no clubs, no table) until its first full day.
///
/// Over League::League (`exe::new_league`, thiscall, ret 0): `league::COMPETITION` = the record, the vtable, the year,
/// the entrant list (`ita_cup_fields::ENTRANT_CLUBS`) = 0, 3 substitutions allowed, 7 substitutes named,
/// `discipline_rule::HOLLAND`, importance 0xc. If the saved state loads (`exe::league_load_from_temp_file`(this, 1) ≠ 0)
/// it returns this. Otherwise: `league::LEAGUE_FLAGS` = 0x28, `league::STAGE_INDEX` = `stage_index::MAIN`,
/// `league::CURRENT_STAGE` = −1, 20 stages and their array (`exe::crt_malloc` of 0x50, unchecked); the statistics
/// (`ita_cups_create_comp_stats`; without them the box at line 0x54); the rules (HolCup::set_up_rules 0x5f3e70,
/// thiscall, ret 0; 0: the box at line 0x5b); then `league::ACTIVE` = 0, `league::CLUB_COUNT` = 0 and `league::TABLE` =
/// 0. Returns this.
/// Original: `HolCup::HolCup` (0x5f3450, hol_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_hol_cup(this: usize, year: u32, record: usize) -> usize {
    let this = League(this);
    exe::new_league(this);
    write_usize(this.addr() + league::COMPETITION, record);
    write_usize(this.addr(), DUTCH_CUP_VTABLE);
    this.set_year(year as u16 as i16);
    write_usize(this.addr() + ita_cup_fields::ENTRANT_CLUBS, 0);
    this.set_substitutions_allowed(3);
    this.set_substitutes_named(7);
    this.set_discipline_rule(discipline_rule::HOLLAND);
    this.set_importance(0xc);
    if exe::league_load_from_temp_file(this, 1) != 0 {
        return this.addr();
    }
    covered!("sbF_8 dutch cup new");
    this.set_league_flags(0x28);
    this.set_stage_index(stage_index::MAIN);
    write_i32(this.addr() + league::CURRENT_STAGE, -1);
    this.set_stage_count(0x14); // sbF8: the own group, 19 group stages and the knockout share 20 slots
    this.set_stages(exe::crt_malloc(0x50));
    if !ita_cups_create_comp_stats(this) {
        show_error_at(0x9cedd4, 0x54);
        return this.addr();
    }
    let rules: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x5f3e70usize); // HolCup::set_up_rules
    if rules(this.addr()) == 0 {
        show_error_at(0x9cedd4, 0x5b);
        return this.addr();
    }
    this.set_active(0);
    this.set_club_count(0);
    this.set_table(0);
    this.addr()
}

/// Only the original (it allocates and reads the save file), checking the plan (`ita_cups_check_constructor_plan`:
/// this, the vtable, the record, the year and, on a new cup, the fixed fields and the statistics; also no clubs and no
/// table yet).
pub(crate) unsafe extern "thiscall" fn compare_construct_hol_cup(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%)(this, year, record);
    let fields = [(league::STAGE_COUNT, 4u8, 0x14), (league::STAGE_INDEX, 1, 0xff), (league::SUBSTITUTES_NAMED, 1, 7), (league::SUBSTITUTIONS_ALLOWED, 1, 3),
        (league::DISCIPLINE_RULE, 1, discipline_rule::HOLLAND as u32), (league::IMPORTANCE, 1, 0xc), (league::LEAGUE_FLAGS, 2, 0x28), (league::CLUB_COUNT, 2, 0), (league::TABLE, 4, 0)];
    let error = ita_cups_check_constructor_plan(this, r, year, record, DUTCH_CUP_VTABLE, &fields, false);
    SCOREBOARD.record(error.is_none(), || format!("construct_hol_cup({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
    r
}

/// Rule: at the season turn the Dutch Cup is emptied: last edition's table, round dates and entrants are freed, the
/// statistics archived, the stages destroyed, the year goes up by one and the rules are made again; the clubs come on
/// the cup's first full day (`hol_cup_update_for_date`).
///
/// `league::SEASON_CLOSED` = 0; the table freed (`exe::league_free_table`(this, 0)); the round dates freed and zeroed;
/// the season archived in the statistics (`exe::comp_stats_archive_season_and_reset`(stats, 1), when there); the
/// entrant list freed (when there) and zeroed with its count; with a current stage ≥ 0, stages 0 to the current one
/// destroyed (their virtual 0 with 1; the slots are not zeroed); the year up by one; `league::CURRENT_STAGE` = −1; the
/// rules (virtual `comp_vtable::SET_UP_RULES`); 0: the box at line 0x204 (the original then returns the message box's
/// eax; the copy returns 0: the callers ignore it); otherwise `league::ACTIVE` = 0 and the rules' return.
/// Original: `HolCup::start_new_season` (0x5f3f10, hol_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn hol_cup_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    exe::league_free_table(this, 0);
    if this.round_dates() != 0 {
        exe::crt_free(this.round_dates());
        this.set_round_dates(0);
    }
    if this.comp_stats() != 0 {
        exe::comp_stats_archive_season_and_reset(this.comp_stats(), 1);
    }
    let list = read_usize(this.addr() + ita_cup_fields::ENTRANT_CLUBS);
    if list != 0 {
        exe::crt_free(list);
    }
    write_usize(this.addr() + ita_cup_fields::ENTRANT_CLUBS, 0);
    write_u32(this.addr() + ita_cup_fields::ENTRANT_CLUB_COUNT, 0);
    let mut i = 0i32;
    while i <= this.current_stage() as i32 {
        let f = read_usize(this.stages() + i as usize * 4);
        if f != 0 {
            let destroy: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(f)));
            destroy(f, 1);
        }
        i += 1;
    }
    covered!("sbF_8 dutch cup turnover");
    this.set_year(this.year().wrapping_add(1)); // sbF8: the next Dutch Cup
    write_i32(this.addr() + league::CURRENT_STAGE, -1);
    let rules: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::SET_UP_RULES));
    let r = rules(this.addr());
    if r == 0 {
        show_error_at(0x9cedd4, 0x204);
        return 0;
    }
    this.set_active(0);
    r
}

/// Comparer for `hol_cup_start_new_season`: only the original (it frees and destroys the old edition); the year up by
/// one (`compare_state_league_new_season`).
pub(crate) unsafe extern "thiscall" fn compare_hol_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    compare_state_league_new_season("hol_cup_start_new_season", League(this), || o(this))
}
