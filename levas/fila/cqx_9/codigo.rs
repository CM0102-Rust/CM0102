// ---------- batch %%LEVA%%: the season turnover of the Asian Super Cup and of the CONCACAF Champions' Cup; the Asian Club Championship records (asia_super_cup.cpp, con_champ.cpp, asia_club_champ.cpp) ----------

/// The first half of a cup's season turnover (the same block in AsiaSuperCup and ConChamp): `cup::SEASON_CLOSED` = 0;
/// the team table (`cup::TEAM_TABLE`) and the round table (`cup::ROUND_TABLE`) freed (`exe::crt_free`: push; add esp, 4
/// -> cdecl) and zeroed; with the tie tables (`cup::TIE_TABLES`), each of the `cup::ROUND_COUNT` rounds' (a signed
/// word) freed and zeroed, then the array freed and zeroed; the season archived in the statistics (+8, when there:
/// Comp_Stats::archive_season_and_reset `exe::comp_stats_archive_season_and_reset`(stats, 1), thiscall, ret 4); the
/// year (`cup::YEAR`) up by one; `cup::CURRENT_ROUND` and `cup::RUN_BY_STAGES` = 0.
unsafe fn cup_turnover_free_season(cup: Cup) {
    cup.set_season_closed(0);
    if cup.team_table() != 0 {
        exe::crt_free(cup.team_table());
        cup.set_team_table(0);
    }
    if cup.round_table() != 0 {
        exe::crt_free(cup.round_table());
        cup.set_round_table(0);
    }
    if cup.tie_tables() != 0 {
        let mut i = 0i32;
        while i < cup.round_count() as i32 {
            let p = read_usize(cup.tie_tables() + i as usize * 4);
            if p != 0 {
                exe::crt_free(p);
                write_usize(cup.tie_tables() + i as usize * 4, 0);
            }
            i += 1;
        }
        exe::crt_free(cup.tie_tables());
        cup.set_tie_tables(0);
    }
    let stats = read_usize(cup.addr() + 8);
    if stats != 0 {
        exe::comp_stats_archive_season_and_reset(stats, 1);
    }
    cup.set_year(cup.year().wrapping_add(1)); // cqx9: the next season's cup
    cup.set_current_round(0);
    cup.set_run_by_stages(0);
}

/// A cup virtual (thiscall, ret 0) at slot offset `o`; its signed return.
unsafe fn cup_turnover_virtual(cup: Cup, o: usize) -> i32 {
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(read_usize(read_usize(cup.addr()) + o));
    f(cup.addr())
}

/// Rule: the Asian Super Cup turns over only in June (month 5, 0-based): the finished cup is cleared, its statistics
/// archived and the year moves on; then the two entrants, the rules, the tie tree, the first round and the predicted
/// positions are made again.
///
/// `AsiaSuperCup::start_new_season()` (virtual +8). The month of today (Date::get_month `exe::date_get_month`
/// ([`G_CURRENT_DATE`]), thiscall, ret 0: 0 to 11, or −1) other than 5: nothing (it returns the month). Otherwise the
/// first half (`cup_turnover_free_season`); the entrants (AsiaSuperCup 0x410a50: mov ecx -> thiscall, ret 0; negative:
/// the error box at line 0x11f and return), the rules (virtual `comp_vtable::SET_UP_RULES`; negative: the box at line
/// 0x126), the tie tree (virtual `cup_vtable::MAP_FIXTURE_TREE`; negative: the box at line 0x12d), the first round
/// (Cup::set_up_round `exe::cup_set_up_round`(this, 0): push 0; mov ecx -> thiscall, ret 4) and the predicted positions
/// (virtual `comp_vtable::SET_PREDICTED_POSITIONS`), whose return it passes on.
/// Original: `AsiaSuperCup::start_new_season` (0x410c50, asia_super_cup.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn asia_super_cup_start_new_season(this: usize) -> u32 {
    let cup = Cup(this);
    let month = exe::date_get_month(G_CURRENT_DATE);
    // simplification: the original returns the whole eax of get_month (the month, or −1); the caller does not read it
    if month != 5 { // cqx9: the Asian Super Cup turns over in June
        return month as i8 as i32 as u32;
    }
    covered!("cqx_9 asian super cup turnover");
    cup_turnover_free_season(cup);
    let entrants: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x410a50usize);
    // simplification: in the boxes the original returns the box's eax; the caller does not read it
    if entrants(this) < 0 {
        show_error_at(0x987be0, 0x11f);
        return 0;
    }
    if cup_turnover_virtual(cup, comp_vtable::SET_UP_RULES) < 0 {
        show_error_at(0x987be0, 0x126);
        return 0;
    }
    if cup_turnover_virtual(cup, cup_vtable::MAP_FIXTURE_TREE) < 0 {
        show_error_at(0x987be0, 0x12d);
        return 0;
    }
    exe::cup_set_up_round(cup, 0); // cqx9: the first round of the new Super Cup
    cup_turnover_virtual(cup, comp_vtable::SET_PREDICTED_POSITIONS) as u32
}

// simplification: the turnover frees the tables, archives the statistics and draws: it cannot run twice. Only the
// original; the plan: in June the year goes up by one (`compare_cup_new_season`), in the other months nothing changes.
unsafe extern "thiscall" fn compare_asia_super_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    if exe::date_get_month(G_CURRENT_DATE) != 5 {
        let year = Cup(this).year();
        let r = o(this);
        SCOREBOARD.record(Cup(this).year() == year, || format!("asia_super_cup_start_new_season({:#x}) outside June: year {} -> {}", this, year, Cup(this).year()));
        return r;
    }
    compare_cup_new_season("asia_super_cup_start_new_season", Cup(this), || o(this))
}

/// Rule: the CONCACAF Champions' Cup turnover: the finished cup is cleared, its statistics archived and the year moves
/// on; an empty team table is made for the entrants (chosen later) and the rules and the tie tree are made again.
///
/// `ConChamp::start_new_season()` (virtual +8): the first half (`cup_turnover_free_season`); the team table
/// (`exe::crt_calloc`(`cup::TEAM_COUNT`, 6): push 6; push n; add esp, 8 -> cdecl) in `cup::TEAM_TABLE`; without it, the
/// error box at line 0x184 and return; the rules (virtual `comp_vtable::SET_UP_RULES`; negative: the box at line
/// 0x18b) and the tie tree (virtual `cup_vtable::MAP_FIXTURE_TREE`; negative: the box at line 0x192).
/// Original: `ConChamp::start_new_season` (0x4c1440, con_champ.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn con_champ_start_new_season(this: usize) {
    let cup = Cup(this);
    covered!("cqx_9 concacaf champions cup turnover");
    cup_turnover_free_season(cup);
    let t = exe::crt_calloc(cup.team_count() as i32 as u32, 6);
    cup.set_team_table(t);
    if t == 0 {
        return show_error_at(0x9a7d30, 0x184);
    }
    if cup_turnover_virtual(cup, comp_vtable::SET_UP_RULES) < 0 {
        return show_error_at(0x9a7d30, 0x18b);
    }
    if cup_turnover_virtual(cup, cup_vtable::MAP_FIXTURE_TREE) < 0 { // cqx9: the CONCACAF tie tree
        show_error_at(0x9a7d30, 0x192);
    }
}

// simplification: the turnover frees the tables and archives the statistics: it cannot run twice. Only the original;
// the plan: the year goes up by one and the team table is there.
unsafe extern "thiscall" fn compare_con_champ_start_new_season(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    let year = Cup(this).year();
    o(this);
    SCOREBOARD.record(Cup(this).year() == year.wrapping_add(1) && Cup(this).team_table() != 0, || format!("con_champ_start_new_season({:#x}): year {} -> {}, team table {:#x}", this, year, Cup(this).year(), Cup(this).team_table()));
}

/// Rule: the Asian Club Championship records: in the zone knockouts (stage 0), elimination and qualification weigh by
/// round (1 to 80 over the 2 rounds) and the zone winner gets the record of reaching the finals (round 0x78, weight
/// 0x5a); in the stages 1 and 2 the qualified club gets the record of round 0x82 (weight 0x5a); in the finals (stage 3)
/// the winner, the runner-up and the third get the final records.
///
/// `AsiaClubChamp::update_staff_records_for_status(club, outcome, stage, _, fixture, final)` (the virtual +0x44).
/// Stage (signed byte) below 1, weight = `scale_by_round`(1, 0x50, the fixture's `round_number()`, 2): outcome 0xff or
/// 2, the elimination (`european_cup_add_knocked_out_record`: `round_prefix()`, `round_name()`, the weight); 0, the
/// qualification (`european_cup_add_qualified_record`: `round_prefix()`, the name word +0x6f of the fixture's round in
/// the cup's own round table `cup::ROUND_TABLE`, the weight); 1, the qualification (0, 0x78, 0x5a); returns 0. Stages 1
/// and 2: outcome 6, the qualification (0, 0x82, 0x5a), returns 0; otherwise −1. Stage 3: outcome 1 the winner
/// (Staff_Records 0x86a800), 2 the runner-up (0x86b0b0), 3 the third (0x86b710), returns 0; otherwise −1. Above 3: 0.
/// (The high 16 bits of the words the original pushes come from small products of the round: zero.)
/// Original: `AsiaClubChamp::update_staff_records_for_status` (0x40c5b0, asia_club_champ.cpp, thiscall, ret 0x18).
unsafe extern "thiscall" fn asia_club_champ_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, _a4: u32, game: usize, final_: u32) -> u32 {
    let g = Fixture(game);
    let (d, f) = (outcome as u8, stage as u8 as i8);
    let comp = read_usize(this + 4);
    if f < 1 {
        match d {
            0xff | 2 => {
                let weight = scale_by_round(1, 0x50, g.round_number(), 2);
                european_cup_add_knocked_out_record(club, comp, g.round_prefix() as u16 as u32, g.round_name() as u16 as u32, weight);
            }
            0 => {
                covered!("cqx_9 asian club championship zone qualified");
                let weight = scale_by_round(1, 0x50, g.round_number(), 2);
                let name = read_u16(Cup(this).round_table().wrapping_add((g.round_number() as isize * cup_round::SIZE as isize) as usize) + 0x6f) as u32;
                european_cup_add_qualified_record(club, comp, g.round_prefix() as u16 as u32, name, weight);
            }
            1 => european_cup_add_qualified_record(club, comp, 0, 0x78, 0x5a),
            _ => {}
        }
        0
    } else if f < 3 {
        if d == 6 {
            european_cup_add_qualified_record(club, comp, 0, 0x82, 0x5a);
            0
        } else {
            u32::MAX
        }
    } else if f == 3 {
        match d {
            1 => cup_record_final_result(0x86a800, club, g, final_),
            2 => cup_record_final_result(0x86b0b0, club, g, final_),
            3 => cup_record_final_result(0x86b710, club, g, final_),
            _ => return u32::MAX,
        }
        0
    } else {
        0
    }
}

// simplification: the records go to the history (and the reputation and morale): they cannot run twice. Only the
// original; the plan: the return by the stage and outcome (0, or −1 where the copy gives −1).
unsafe extern "thiscall" fn compare_asia_club_champ_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, a4: u32, game: usize, final_: u32) -> u32 {
    let (d, f) = (outcome as u8, stage as u8 as i8);
    let plan = if (1..3).contains(&f) && d != 6 || f == 3 && !(1..=3).contains(&d) { u32::MAX } else { 0 };
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(%%I2%%)(this, club, outcome, stage, a4, game, final_);
    SCOREBOARD.record(r == plan, || format!("asia_club_champ_update_staff_records_for_status({:#x}, {}, {}) returned {:#x}, plan {:#x}", club, d, f, r, plan));
    r
}
