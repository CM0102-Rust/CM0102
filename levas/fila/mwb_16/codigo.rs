// ---------- mwb_16: the Scottish League Cup (sco_lge_cup.cpp): constructor, round table and day update ----------

/// The Scottish League Cup at career creation.
///
/// Cup::Cup (`exe::new_cup`); the record, the vtable 0x96f568, the year; `cup::CURRENT_ROUND` = 0, `cup::STAGE_INDEX` =
/// 0xff, `cup::CURRENT_STAGE` = −1, `cup::STAGE_COUNT` = 0, `cup::CLUB_SIDES` = 1, 5 substitutes named and 3 allowed,
/// the discipline rule of the Scottish cups (0x19), `cup::RUN_BY_STAGES` = 0, `cup::SEASON_SET_UP` = 0 and the European
/// entrant count (+0xb2) = 0. If the saved state loads (Cup::load_from_disk 0x51fc00 with 1), returns this.
///
/// Otherwise: 40 teams (`cup::TEAM_COUNT`) and an empty team table (`exe::crt_calloc(0x28, 6)`; null: the box of line
/// 0x4a and this); the prize fund of 1,000,000; the round table of stage −1 (virtual +0x3c) into `cup::ROUND_TABLE`; the
/// fixture tree (Cup::map_fixture_tree 0x518790; 0 = the box of line 0x58); the statistics (operator new 0x5ce and
/// Comp_Stats::Comp_Stats; null: the box of line 0x5f). Unlike the other Scottish cups, neither the entrants (they come
/// when the season starts: `ScoLgeCup::update_for_date`) nor the predicted positions. Returns this.
/// Original: `ScoLgeCup::ScoLgeCup` (0x7ef9e0, sco_lge_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_sco_lge_cup(this: usize, year: u32, record: usize) -> usize {
    let cup = Cup(this);
    exe::new_cup(cup);
    write_usize(this + league::COMPETITION, record);
    write_usize(this, 0x96f568);
    write_u16(this + cup::YEAR, year as u16);
    write_u8(this + cup::CURRENT_ROUND, 0);
    write_u8(this + cup::STAGE_INDEX, 0xff);
    write_u32(this + cup::CURRENT_STAGE, 0xffff_ffff);
    write_u32(this + cup::STAGE_COUNT, 0);
    write_u8(this + cup::CLUB_SIDES, 1);
    write_u8(this + cup::SUBSTITUTES_NAMED, 5);
    write_u8(this + cup::SUBSTITUTIONS_ALLOWED, 3); // mwb16: the League Cup substitutions
    write_u8(this + cup::DISCIPLINE_INDEX, 0x19);
    write_u8(this + cup::RUN_BY_STAGES, 0);
    write_u32(this + cup::SEASON_SET_UP, 0);
    write_u8(this + eng_cc_cup::EUROPEAN_ENTRANT_COUNT, 0);
    let load_from_disk: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize);
    if load_from_disk(this, 1) != 0 {
        return this;
    }
    covered!("mwb_16 new scottish league cup");
    cup.set_team_count(0x28);
    let l = exe::crt_calloc(0x28, 6);
    cup.set_team_table(l);
    if l == 0 {
        show_error_at(0xa7af38, 0x4a);
        return this;
    }
    write_u32(this + cup::PRIZE_FUND, 0xf4240);
    let rounds: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::CREATE_SCHEDULE));
    let table = rounds(this, 0xffff_ffff, this + cup::ROUND_COUNT, this + cup::STAGE_LABEL, 0);
    write_usize(this + cup::ROUND_TABLE, table);
    let tree: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize);
    if tree(this) == 0 {
        show_error_at(0xa7af38, 0x58);
        return this;
    }
    let n = exe::crt_operator_new(0x5ce);
    let s = if n != 0 { exe::new_comp_stats(n, cup) } else { 0 };
    write_usize(this + league::COMP_STATS, s);
    if s == 0 {
        show_error_at(0xa7af38, 0x5f);
    }
    this
}

/// Comparer for `construct_sco_lge_cup`: only the original runs (it allocates); then the plan: this, the vtable, the
/// year, the record, the discipline rule, the bench and, in a new career, the 40 teams, the prize fund and the rounds.
pub(crate) unsafe extern "thiscall" fn compare_construct_sco_lge_cup(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
    let loaded = league_will_load_saved_state(record);
    let r = o(this, year, record);
    let mut ok = r == this && read_usize(this) == 0x96f568 && read_u16(this + cup::YEAR) == year as u16 && read_usize(this + league::COMPETITION) == record
        && read_u8(this + cup::DISCIPLINE_INDEX) == 0x19;
    if !loaded && read_usize(this + league::COMP_STATS) != 0 {
        ok = ok && read_u8(this + cup::SUBSTITUTIONS_ALLOWED) == 3 && Cup(this).team_count() == 0x28 && read_u32(this + cup::PRIZE_FUND) == 0xf4240 && read_usize(this + cup::ROUND_TABLE) != 0;
    }
    SCOREBOARD.record(ok, || format!("construct_sco_lge_cup({:#x}) [year arg {}, record arg {:#x}, loaded {}]: returned {:#x}, vtable {:#x}, teams {}, prize fund {}, rounds {:#x}",
        this, year as u16, record, loaded, r, read_usize(this), Cup(this).team_count(), read_u32(this + cup::PRIZE_FUND), read_usize(this + cup::ROUND_TABLE)));
    r
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I1%%);
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
        let entrants: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x7f0400usize);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I2%%);
    let before = read_u32(this + cup::SEASON_SET_UP);
    let due = before == 0 && read_i16(read_usize(G_UEFA_SEEDING) + uefa_seeding::SEASON_YEAR) as u16 as i32 == Cup(this).year() as i32 && stage != 0;
    let r = o(this, today, stage);
    let after = read_u32(this + cup::SEASON_SET_UP);
    SCOREBOARD.record(after == if due { 1 } else { before }, || format!("sco_lge_cup_update_for_date({:#x}) due {} +0x45 {} -> {}", this, due, before, after));
    r
}
