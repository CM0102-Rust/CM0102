// ---------- %%LEVA%%a leva: a virada da temporada das copas da CONMEBOL: a Libertadores, a Mercosul e a Merconorte (conmebol_liber.cpp, conmebol_merc.cpp, con_merc_cup.cpp) ----------

/// The first half of the three cups' season turnover (the same body): `league::SEASON_CLOSED` = 0 and the table freed
/// (`exe::league_free_table`(this, 0): push 0; mov ecx -> thiscall, ret 4); the calendar `league::ROUND_DATES` freed
/// (`exe::crt_free`: push; add esp, 4 -> cdecl) and zeroed; with statistics (`league::COMP_STATS`),
/// `exe::comp_stats_archive_season_and_reset`(stats, 1) (push 1; mov ecx -> thiscall, ret 4); the invited clubs list
/// +0xee freed (not zeroed); with the current stage (`league::CURRENT_STAGE`) >= 0, the existing stages 0..=current
/// destroyed (the virtual `comp_vtable::SCALAR_DELETING_DESTRUCTOR` with 1: push 1; mov ecx -> thiscall).
unsafe fn conmebol_free_season(this: League) {
    write_u32(this.addr() + league::SEASON_CLOSED, 0);
    exe::league_free_table(this, 0);
    let l = this.round_dates();
    if l != 0 {
        exe::crt_free(l);
        write_u32(this.addr() + league::ROUND_DATES, 0);
    }
    let stats = this.comp_stats();
    if stats != 0 {
        exe::comp_stats_archive_season_and_reset(stats, 1);
    }
    let c = read_usize(this.addr() + 0xee);
    if c != 0 {
        exe::crt_free(c);
    }
    if (this.current_stage() as i32) >= 0 {
        let mut i = 0i32;
        loop {
            let f = read_usize(this.stages() + 4 * i as usize);
            if f != 0 {
                let destroy: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(f) + comp_vtable::SCALAR_DELETING_DESTRUCTOR));
                destroy(f, 1);
            }
            i += 1;
            if i > this.current_stage() as i32 {
                break;
            }
        }
    }
}

/// The year (`league::YEAR`, word) goes up by one and the current stage (`league::CURRENT_STAGE`) goes back to -1.
unsafe fn conmebol_next_year(this: League) {
    this.set_year(this.year().wrapping_add(1));
    write_u32(this.addr() + league::CURRENT_STAGE, u32::MAX);
}

/// The rules again (the virtual `comp_vtable::SET_UP_RULES`, thiscall, ret 0); 0: the box for line `row` of `file`
/// and 0 (the original returns the box's eax; the caller does not read it). Returns what the rules returned.
unsafe fn conmebol_set_up_rules_again(this: League, file: usize, row: u32) -> u32 {
    let rules: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::SET_UP_RULES));
    let r = rules(this.addr());
    if r == 0 {
        show_error_at(file, row);
    }
    r
}

/// The Libertadores season turnover: the first half (`conmebol_free_season`), the next year (`conmebol_next_year`),
/// the CONMEBOL seeding for the new season (ConmebolSeeding::update_for_new_season 0x4c61b0(`G_CONMEBOL_SEEDING`, 0):
/// push 0; mov ecx -> thiscall, ret 4: the nations' places), `league::ACTIVE` = 0 (the draw still to be made) and the
/// rules (`conmebol_set_up_rules_again`, the box for line 0x1e3).
/// Original: `ConmebolLiber::start_new_season` (0x4c3980, conmebol_liber.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn conmebol_liber_start_new_season(this: usize) -> u32 {
    let league = League(this);
    let seed: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x4c61b0usize);
    conmebol_free_season(league);
    covered!("cqv_12 libertadores turnover");
    conmebol_next_year(league); // cqv12: the Libertadores of the next year
    seed(read_usize(G_CONMEBOL_SEEDING), 0); // cqv12: the places of the new Libertadores
    write_u32(this + league::ACTIVE, 0); // cqv12: the new Libertadores not drawn yet
    conmebol_set_up_rules_again(league, 0x9a7de4, 0x1e3)
}

// simplification: the turnover frees tables and stages and archives the statistics: it cannot run twice. Only the
// original; the plan: `league::SEASON_CLOSED` = 0, the year + 1 and (Libertadores and Mercosur) `league::ACTIVE` = 0
// (the new rules rebuild the calendar `league::ROUND_DATES`).
unsafe fn conmebol_compare_new_season(name: &str, this: usize, clears_active: bool, o: impl FnOnce() -> u32) -> u32 {
    let league = League(this);
    let year = league.year();
    let r = o();
    let new = league.year();
    let ok = read_usize(this + league::SEASON_CLOSED) == 0 && new == year.wrapping_add(1) && (!clears_active || league.active() == 0);
    SCOREBOARD.record(ok, || format!("{}({:#x}): year {} -> {}, +0x4c {}, +0xba {:#x}, +0x45 {}", name, this, year, new, read_usize(this + league::SEASON_CLOSED), league.round_dates(), league.active()));
    r
}

unsafe extern "thiscall" fn compare_conmebol_liber_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    conmebol_compare_new_season("conmebol_liber_start_new_season", this, true, || o(this))
}

/// The Mercosur Cup season turnover: the first half (`conmebol_free_season`), the next year, `league::ACTIVE` = 0 and
/// the rules (the box for line 0x1c6).
/// Original: `ConmebolMerc::start_new_season` (0x4c5040, conmebol_merc.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn conmebol_merc_start_new_season(this: usize) -> u32 {
    let league = League(this);
    conmebol_free_season(league);
    conmebol_next_year(league);
    write_u32(this + league::ACTIVE, 0);
    conmebol_set_up_rules_again(league, 0x9a7e5c, 0x1c6)
}

unsafe extern "thiscall" fn compare_conmebol_merc_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    conmebol_compare_new_season("conmebol_merc_start_new_season", this, true, || o(this))
}

/// The Merconorte season turnover: the first half (`conmebol_free_season`), the next year and the rules; only with them
/// `league::ACTIVE` = 0 (the box for line 0x1b1 leaves it as it was).
/// Original: `ConMercCup::start_new_season` (0x4c1fb0, con_merc_cup.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn con_merc_cup_start_new_season(this: usize) -> u32 {
    let league = League(this);
    conmebol_free_season(league);
    conmebol_next_year(league);
    let r = conmebol_set_up_rules_again(league, 0x9a7d6c, 0x1b1);
    if r != 0 {
        write_u32(this + league::ACTIVE, 0);
    }
    r
}

unsafe extern "thiscall" fn compare_con_merc_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    conmebol_compare_new_season("con_merc_cup_start_new_season", this, false, || o(this))
}
