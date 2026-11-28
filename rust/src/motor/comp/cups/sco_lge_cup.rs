use super::*;

/// The Scottish League Cup at career creation.
///
/// Cup::Cup (`exe::new_cup`); the record, the vtable 0x96f568, the year; `cup::CURRENT_ROUND` = 0, `cup::STAGE_INDEX` =
/// 0xff, `cup::CURRENT_STAGE` = −1, `cup::STAGE_COUNT` = 0, `cup::CLUB_SIDES` = 1, 5 substitutes named and 3 allowed,
/// the discipline rule of the Scottish cups (0x19), `cup::RUN_BY_STAGES` = 0, `cup::SEASON_SET_UP` = 0 and the European
/// entrant count (+0xb2) = 0. If the saved state loads (Cup::load_from_disk 0x51fc00 with 1), returns this.
///
/// Otherwise: 40 teams (`cup::TEAM_COUNT`) and an empty team table (`exe::crt_calloc(0x28, 6)`; null: the box of line
/// 0x4a and this); the prize fund of 1,000,000; the round table of stage −1 (virtual +0x3c) into `cup::ROUND_TABLE`; the
/// fixture exe::cup_map_fixture_tree(Cup::map_fixture_tree 0x518790; 0 = the box of line 0x58); the statistics (operator new 0x5ce and
/// Comp_Stats::Comp_Stats; null: the box of line 0x5f). Unlike the other Scottish cups, neither the entrants (they come
/// when the season starts: `ScoLgeCup::update_for_date`) nor the predicted positions. Returns this.
/// Original: `ScoLgeCup::ScoLgeCup` (0x7ef9e0, sco_lge_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_sco_lge_cup(this: usize, year: u32, record: usize) -> usize {
    let this = League(this);
    let cup = Cup(this.addr());
    exe::new_cup(cup);
    write_usize(this.addr() + league::COMPETITION, record);
    write_usize(this.addr(), 0x96f568);
    write_u16(this.addr() + cup::YEAR, year as u16);
    write_u8(this.addr() + cup::CURRENT_ROUND, 0);
    write_u8(this.addr() + cup::STAGE_INDEX, 0xff);
    write_u32(this.addr() + cup::CURRENT_STAGE, 0xffff_ffff);
    write_u32(this.addr() + cup::STAGE_COUNT, 0);
    write_u8(this.addr() + cup::CLUB_SIDES, 1);
    write_u8(this.addr() + cup::SUBSTITUTES_NAMED, 5);
    write_u8(this.addr() + cup::SUBSTITUTIONS_ALLOWED, 3); // mwb16: the League Cup substitutions
    write_u8(this.addr() + cup::DISCIPLINE_INDEX, 0x19);
    write_u8(this.addr() + cup::RUN_BY_STAGES, 0);
    write_u32(this.addr() + cup::SEASON_SET_UP, 0);
    write_u8(this.addr() + eng_cc_cup::EUROPEAN_ENTRANT_COUNT, 0);
    if exe::cup_load_from_disk(this, 1) != 0 {
        return this.addr();
    }
    covered!("mwb_16 new scottish league cup");
    cup.set_team_count(0x28);
    let l = exe::crt_calloc(0x28, 6);
    cup.set_team_table(l);
    if l == 0 {
        show_error_at(0xa7af38, 0x4a);
        return this.addr();
    }
    write_u32(this.addr() + cup::PRIZE_FUND, 0xf4240);
    let rounds: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let table = rounds(this.addr(), 0xffff_ffff, this.addr() + cup::ROUND_COUNT, this.addr() + cup::STAGE_LABEL, 0);
    write_usize(this.addr() + cup::ROUND_TABLE, table);
    if exe::cup_map_fixture_tree(this) == 0 {
        show_error_at(0xa7af38, 0x58);
        return this.addr();
    }
    let n = exe::crt_operator_new(0x5ce);
    let s = if n != 0 { exe::new_comp_stats(n, cup) } else { 0 };
    this.set_comp_stats(s);
    if s == 0 {
        show_error_at(0xa7af38, 0x5f);
    }
    this.addr()
}

/// Comparer for `construct_sco_lge_cup`: only the original runs (it allocates); then the plan: this, the vtable, the
/// year, the record, the discipline rule, the bench and, in a new career, the 40 teams, the prize fund and the rounds.
pub(crate) unsafe extern "thiscall" fn compare_construct_sco_lge_cup(this: usize, year: u32, record: usize) -> usize {
    let this = League(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_SCO_LGE_CUP);
    let loaded = league_will_load_saved_state(record);
    let r = o(this.addr(), year, record);
    let mut ok = r == this.addr() && read_usize(this.addr()) == 0x96f568 && built_year_ok(read_u16(this.addr() + cup::YEAR), year as u16) && this.competition().addr() == record
        && read_u8(this.addr() + cup::DISCIPLINE_INDEX) == 0x19;
    if !loaded && this.comp_stats() != 0 {
        ok = ok && read_u8(this.addr() + cup::SUBSTITUTIONS_ALLOWED) == 3 && Cup(this.addr()).team_count() == 0x28 && read_u32(this.addr() + cup::PRIZE_FUND) == 0xf4240 && read_usize(this.addr() + cup::ROUND_TABLE) != 0;
    }
    SCOREBOARD.record(ok, || format!("construct_sco_lge_cup({:#x}) [year arg {}, record arg {:#x}, loaded {}]: returned {:#x}, vtable {:#x}, teams {}, prize fund {}, rounds {:#x}",
        this.addr(), year as u16, record, loaded, r, read_usize(this.addr()), Cup(this.addr()).team_count(), read_u32(this.addr() + cup::PRIZE_FUND), read_usize(this.addr() + cup::ROUND_TABLE)));
    r
}

/// rule: the Scottish League Cup has 6 single-match rounds from July to March: 1st round drawn 10/7, played 31/7, 2nd
/// 1/8 and 17/8, 3rd 19/8 and 22/9, quarter-finals 23/9 and 1/12, semi-finals 2/12 and 16/2 at a neutral ground, final
/// drawn 17/2 and played on 19/3 at the national stadium; the clubs in the European cups (+0xb2) only enter in the 3rd
/// round (from 2000: 2b + 20 teams in the 1st round, (16 − b) × 2 in the 2nd); category 4 in the first three rounds, 6
/// after; no prize per round. Outside stage −1 it writes nothing and returns 0; out of memory, the box of line 0xa0 and
/// 0 (`eng_cc_cup_create_single_stage_round_table`)
/// Original: `ScoLgeCup::create_round_table` (0x7efcb0, sco_lge_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn sco_lge_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let t = scottish_league_cup_rounds(read_i8(this + eng_cc_cup::EUROPEAN_ENTRANT_COUNT) as i16, Cup(this).year() == 0x7cf);
    eng_cc_cup_create_single_stage_round_table(Cup(this), stage, rounds, extra, out, &t, 0xa7af38, 0xa0, "mwb_16 scottish league cup rounds")
}

/// Comparer for `sco_lge_cup_create_round_table` (side by side, the written rounds).
pub(crate) unsafe extern "thiscall" fn compare_sco_lge_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_SCO_LGE_CUP_CREATE_ROUND_TABLE);
    compare_cup_round_table("sco_lge_cup_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out),
        |r, e, s| sco_lge_cup_create_round_table(this, stage, r, e, s))
}

/// rule: the Scottish League Cup is rebuilt once per season (`cup::SEASON_SET_UP`), on the first day with results when
/// the game year (the UEFA seeding's season year) reaches the cup year: the old tables are freed (the pointers left as
/// they are) and the entrants (ScoLgeCup::collect_entrants 0x7f0400, thiscall, ret 0; negative: the box of line 0x15e),
/// the rounds (virtual +0x8c; 0x165), the fixture tree (virtual +0x94; 0x16c) and the predicted positions (virtual
/// +0x5c) are built, after a box returning 0; then, every day, the cup update (Cup::update_for_date 0x51f890, the same
/// arguments), whose eax it returns
/// Original: `ScoLgeCup::update_for_date` (0x7f0230, sco_lge_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn sco_lge_cup_update_for_date(this: usize, today: usize, stage: u32) -> u32 {
    let cup = Cup(this);
    if read_u32(this + cup::SEASON_SET_UP) == 0 && read_i16(read_usize(G_UEFA_SEEDING) + uefa_seeding::SEASON_YEAR) as u16 as i32 == cup.year() as i32 && stage != 0 {
        free_season_tables(cup);
        covered!("mwb_16 scottish league cup season");
        write_u32(this + cup::SEASON_SET_UP, 1);
        // simplification: in the message boxes the original zeroes only al (the rest of eax comes from the box); the
        // caller reads the byte
        let entrants: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(FN_SCOLGECUP_COLLECT_ENTRANTS);
        if entrants(this) < 0 {
            show_error_at(0xa7af38, 0x15e);
            return 0;
        }
        if call_cup_virtual(cup, comp_vtable::SET_UP_RULES) < 0 {
            show_error_at(0xa7af38, 0x165);
            return 0;
        }
        if call_cup_virtual(cup, cup_vtable::MAP_FIXTURE_TREE) < 0 {
            show_error_at(0xa7af38, 0x16c);
            return 0;
        }
        call_cup_virtual(cup, comp_vtable::SET_PREDICTED_POSITIONS); // mwb16: the League Cup predicted positions
    }
    exe::cup_update_for_date(cup, today, stage)
}

/// Comparer for `sco_lge_cup_update_for_date`: only the original runs (it frees, rebuilds, draws and plays); the plan:
/// `cup::SEASON_SET_UP` goes to 1 when the season was due, otherwise it stays.
pub(crate) unsafe extern "thiscall" fn compare_sco_lge_cup_update_for_date(this: usize, today: usize, stage: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_SCO_LGE_CUP_UPDATE_FOR_DATE);
    let before = read_u32(this + cup::SEASON_SET_UP);
    let due = before == 0 && read_i16(read_usize(G_UEFA_SEEDING) + uefa_seeding::SEASON_YEAR) as u16 as i32 == Cup(this).year() as i32 && stage != 0;
    let r = o(this, today, stage);
    let after = read_u32(this + cup::SEASON_SET_UP);
    SCOREBOARD.record(after == if due { 1 } else { before }, || format!("sco_lge_cup_update_for_date({:#x}) due {} +0x45 {} -> {}", this, due, before, after));
    r
}

/// rule: at the turnover the Scottish League Cup frees last season's teams, rounds and ties (zeroing the pointers),
/// archives the statistics, moves to the next year and goes back to 40 empty places (`exe::crt_calloc(0x28, 6)`; null:
/// the box of line 0x281 and 0), with no current round, not run by stages and the season not set up (the entrants come
/// in the day update); then the rounds (virtual +0x8c; negative: the box of line 0x288 and 0) and the fixture tree
/// (virtual +0x94; negative: 0x28f and 0), whose eax it returns (no predicted positions here)
/// Original: `ScoLgeCup::start_new_season` (0x7f0930, sco_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn sco_lge_cup_start_new_season(this: usize) -> u32 {
    let this = League(this);
    let cup = Cup(this.addr());
    write_u32(this.addr() + cup::SEASON_CLOSED, 0);
    for o in [cup::TEAM_TABLE, cup::ROUND_TABLE] {
        let l = read_usize(this.addr() + o);
        if l != 0 {
            exe::crt_free(l);
            write_u32(this.addr() + o, 0);
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
    let stats = this.comp_stats();
    if stats != 0 {
        exe::comp_stats_archive_season_and_reset(stats, 1);
    }
    cup.set_year(cup.year().wrapping_add(1)); // mwb17: the next League Cup
    covered!("mwb_17 scottish league cup turnover");
    write_u8(this.addr() + cup::CURRENT_ROUND, 0);
    write_u8(this.addr() + cup::RUN_BY_STAGES, 0);
    write_u32(this.addr() + cup::SEASON_SET_UP, 0);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SCO_LGE_CUP_START_NEW_SEASON);
    compare_cup_new_season("sco_lge_cup_start_new_season", Cup(this), || o(this))
}

/// The Scottish League Cup round table with the team counts adjusted by `b` = the European entrant count (+0xb2, signed
/// byte; arithmetic in 16 bits as in the original).
///
/// From 2000: 1st round 2b + 20 teams, b + 10 matches, 2b + 20 entering; 2nd round (16 − b) × 2 teams, 16 − b matches,
/// 22 − 3b entering and 2b + 20 already in; 3rd round b entering and 42 − b already in. In 1999 (30 clubs): 2b + 16, b + 8,
/// 2b + 16; (16 − b) × 2, 16 − b, (8 − b) × 3 and 2b + 16; b and 40 − b.
pub(crate) fn scottish_league_cup_rounds(b: i16, y1999: bool) -> [EngCcCupCupRound; 6] {
    let mut t = SCOTTISH_LEAGUE_CUP_ROUNDS;
    let x = |v: i16| v as u16;
    let (base, carried) = if y1999 { (0x10i16, 0x28i16) } else { (0x14, 0x2a) };
    t[0].teams = (x(b.wrapping_mul(2).wrapping_add(base)), x(b.wrapping_add(base / 2)), x(b.wrapping_mul(2).wrapping_add(base)), 0);
    let new2 = if y1999 { 8i16.wrapping_sub(b).wrapping_mul(3) } else { 0x16i16.wrapping_sub(b.wrapping_mul(3)) };
    t[1].teams = (x(0x10i16.wrapping_sub(b).wrapping_mul(2)), x(0x10i16.wrapping_sub(b)), x(new2), x(b.wrapping_mul(2).wrapping_add(base)));
    t[2].teams.2 = x(b);
    t[2].teams.3 = x(carried.wrapping_sub(b));
    t
}

/// The Scottish League Cup rounds with no European entrants: the first round drawn on 10 July and played on 31 July,
/// the second on 1/8 and 17/8 (code 0x403 for the draw), the third on 19/8 and 22/9, the quarter-finals on 23/9 and
/// 1/12 (a replay 4 days later), the semi-finals on 2/12 and 16/2 at a neutral ground (venue 0x28) and the final drawn
/// on 17/2 and played on 19/3 at the national stadium (venue 4); single matches, tie-break code 3; category 4 in the
/// first three rounds, 6 after; no prize in the first two rounds, none (−1) after (`scottish_league_cup_rounds` fills
/// the team counts).
pub(crate) const SCOTTISH_LEAGUE_CUP_ROUNDS: [EngCcCupCupRound; 6] = [
    cql10r((0xa, 6, 0, 6), (0x1f, 6, 0, 5, 1, 0), 0xa, (0, 3, 0), 4, (0, 0, 0, 0), (0, 1, 0), (0, 0, 0)),
    cql10r((1, 7, 0, 6), (0x11, 7, 0, 1, 2, 0), 0x14, (0, 0x403, 0), 4, (0, 0, 0, 0), (0, 1, 0), (0, 0, 0)),
    cql10r((0x13, 7, 0, 3), (0x16, 8, 0, 2, 2, 0), 0x1e, (0, 3, 0), 4, (0x10, 8, 0, 0), (0, 1, 0), (0, 0xffff_ffff, 0)),
    cql10r((0x17, 8, 0, 3), (1, 0xb, 0, 2, 2, 0), 0x78, (0, 3, 0), 6, (8, 4, 0, 0), (0, 1, 4), (0, 0xffff_ffff, 0)), // mwb16: the League Cup quarter-finals
    cql10r((2, 0xb, 0, 3), (0x10, 1, 1, 2, 2, 0x28), 0x82, (0, 3, 0), 6, (4, 2, 0, 0), (0, 1, 0), (0, 0xffff_ffff, 0)),
    cql10r((0x11, 1, 1, 3), (0x13, 2, 1, 6, 1, 4), 0x96, (0, 3, 0), 6, (2, 1, 0, 0), (0, 1, 0), (0, 0xffff_ffff, 0)),
];

/// Reads the Scottish League Cup's own data from its temporary file: the byes byte (+0xb2, the clubs in the European
/// cups) (`exe::crt_fread` of 1 byte, cdecl); a short read: the box of line 0x2b8 of sco_lge_cup.cpp and 0. Returns 1.
/// Original: `ScoLgeCup::load_extra_data` (0x7f0ae0, sco_lge_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn sco_lge_cup_load_extra_data(this: usize, file: usize) -> u32 {
    if exe::crt_fread(this + 0xb2, 1, 1, file) != 1 {
        show_error_at(0xa7af38, 0x2b8);
        return 0;
    }
    covered!("cx03_11 scottish league cup data loaded");
    1
}

pub(crate) unsafe extern "thiscall" fn compare_sco_lge_cup_load_extra_data(this: usize, file: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_SCO_LGE_CUP_LOAD_EXTRA_DATA);
    compare_cup_extra_data_load("sco_lge_cup_load_extra_data", this, file, &[(0xb2, 1)], || o(this, file), || sco_lge_cup_load_extra_data(this, file))
}
