use super::*;

/// Writes one cup round (a 0x68-byte record).
///
/// The draw (day, month with 0 = January, years to add to the cup year +0x40, weekday or −1) goes through `cup_set_round_draw_date`; the
/// match (the same, plus the leg at +6 and the venue at +0x13) through `cup_set_round_match_date`.
///
/// Then the fields: +7 the round name, +9 the index, +0xb the draw bits, +0xd and +0xf the tie-breakers (single match
/// and two legs), +0x17 the category copied into each match, +0x18/+0x1a/+0x1c/+0x1e teams, matches, new teams and
/// entrants already in, +0x20/+0x21/+0x22 replay, matches per tie and days between them, +0x5c/+0x60/+0x64 the prize
/// money (reaching, winning, losing).
#[derive(Clone, Copy)]
pub(crate) struct FraSuperCupRound {
    pub(crate) draw: (u32, u32, u32, u32),
    pub(crate) game: (u32, u32, u32, u32, u32, u32),
    pub(crate) code: u16,
    pub(crate) bits: (u16, u16, u16),
    pub(crate) category: u8,
    pub(crate) teams: (u16, u16, u16, u16),
    pub(crate) tie: (u8, u8, u8),
    pub(crate) prizes: (u32, u32, u32),
}

/// A linha da tabela: (sorteio), (jogo), nome, (bits do sorteio, desempate, desempate de ida e volta), categoria,
/// (times, jogos, novos, inscritos), (replay, jogos por confronto, dias entre eles), (prêmios).
pub(crate) const fn cql12r(sorteio: (u32, u32, u32, u32), jogo: (u32, u32, u32, u32, u32, u32), codigo: u16, bits: (u16, u16, u16), categoria: u8,
    times: (u16, u16, u16, u16), confronto: (u8, u8, u8), premios: (u32, u32, u32)) -> FraSuperCupRound {
    FraSuperCupRound { draw: sorteio, game: jogo, code: codigo, bits, category: categoria, teams: times, tie: confronto, prizes: premios }
}

/// Writes round i of a cup round table: draw and match dates and the round fields.
pub(crate) unsafe fn fra_super_write_cup_round(t: usize, i: u32, r: &FraSuperCupRound, year: u32) {
    exe::cup_set_round_draw_date(t, i, r.draw.0, r.draw.1, r.draw.2, r.draw.3, year);
    exe::cup_set_round_match_date(t, i, r.game.0, r.game.1, r.game.2, r.game.3, r.game.4, year, r.game.5);
    let e = t + i as usize * 0x68;
    let w = |o: usize, v: u16| write_u16(e + o, v);
    let d = |o: usize, v: u32| write_u32(e + o, v);
    w(cup_round::ROUND_CODE, r.code);
    w(cup_round::ROUND_NUMBER, i as u16);
    w(cup_round::DRAW_RULES, r.bits.0);
    w(cup_round::TIE_RULES, r.bits.1);
    w(cup_round::TWO_LEG_TIE_RULES, r.bits.2);
    write_u8(e + 0x17, r.category);
    w(cup_round::TEAMS, r.teams.0);
    w(cup_round::TIES, r.teams.1);
    w(cup_round::NEW_TEAMS, r.teams.2);
    w(cup_round::FIRST_NEW_TEAM, r.teams.3);
    write_u8(e + 0x20, r.tie.0);
    write_u8(e + 0x21, r.tie.1);
    write_u8(e + 0x22, r.tie.2);
    d(0x5c, r.prizes.0);
    d(cup_round::WINNER_PRIZE, r.prizes.1);
    d(cup_round::LOSER_PRIZE, r.prizes.2);
}

/// Shared body of virtual +0x3c (`create_round_table(fase, &rodadas, &extra, &saida)`, thiscall, ret 0x10) for
/// single-stage cups.
///
/// Outside stage −1 (the byte), returns 0 without writing anything. At −1: saida (if requested) = 0, rodadas = the
/// table size, extra = 0; the table is allocated (`exe::crt_malloc` of 0x68 × rodadas; out of memory, the error box `linha` of
/// `arquivo` and returns 0) and filled with each round; returns the table.
///
/// `ajuste` modifies the first round after allocation (the drawn venue of the French Super Cup).
pub(crate) unsafe fn fra_super_create_single_stage_round_table(this: Cup, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32, table: &[FraSuperCupRound], file: usize, row: u32, name: &'static str,
    adjust: impl FnOnce(&mut FraSuperCupRound)) -> usize {
    if stage as u8 != 0xff {
        return 0;
    }
    if !out.is_null() {
        *out = 0;
    }
    *rounds = table.len() as i16;
    *extra = 0;
    let t = exe::crt_malloc(table.len() as u32 * 0x68);
    if t == 0 {
        show_error_at(file, row);
        return 0;
    }
    covered!(name);
    let mut t0 = table[0];
    adjust(&mut t0);
    let year = this.year() as i32 as u32;
    for (i, r) in table.iter().enumerate() {
        fra_super_write_cup_round(t, i as u32, if i == 0 { &t0 } else { r }, year);
    }
    t
}

/// A Supercopa da França: um jogo em julho; o prêmio de 60 mil por chegar. O mando vem à parte (`french_super_cup_venue`).
pub(crate) const FRENCH_SUPER_CUP_ROUNDS: [FraSuperCupRound; 1] = [
    cql12r((5, 6, 0, 5), (0x18, 6, 0, 5, 2, 0x400), 0, (0, 1, 0), 6, (2, 1, 2, 0), (0, 1, 0), (0xea60, 0, 0)), // cql12: the French Super Cup match
];

/// Venue flag of the French Super Cup: 0x400 in 2001, otherwise one of six flags drawn at random (FN_UTILS_RANDOM(6)).
pub(crate) unsafe fn french_super_cup_venue(this: Cup) -> u32 {
    if this.year() == 0x7d1 {
        return 0x400;
    }
    const MANDOS: [i16; 6] = [0x200, 0x400, 0x800, 0x1000, 0x2000, 0x4000];
    MANDOS[exe::utils_random((6) as i32) as usize] as i32 as u32
}

/// rule: the French Super Cup (Trophée des Champions) is a single match between 2 clubs, drawn on 5/7 and played on
/// 24/7 (moved to weekday 5), decided on penalties [inference: tiebreak 1]; in 2001 at venue 0x400, in other years at
/// one of six drawn venues (0x200 to 0x4000: [inference] the country's largest grounds); 60 thousand to each club for
/// taking part; category 6
/// Original: `FraSuper::create_round_table` (0x5bf5d0, fra_super.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn fra_super_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    fra_super_create_single_stage_round_table(Cup(this), stage, rounds, extra, out, &FRENCH_SUPER_CUP_ROUNDS, 0x9ccc48, 0x97, "cql_12 tabela da supercopa da franca",
        |r| r.game.5 = french_super_cup_venue(Cup(this)))
}

// simplification: outside 2001 the original rolls home advantage (0x90cfc0): only it runs in that case.

/// A Supercopa da Espanha: ida e volta (7 dias entre os jogos) em agosto, sem sorteio (+0xd = 0x200), desempate de ida e
/// volta 1.
pub(crate) const SPANISH_SUPER_CUP: [FraSuperCupRound; 1] = [
    cql12r((5, 6, 0, 5), (0x14, 7, 0, 6, 1, 0), 0, (0, 0x200, 1), 6, (2, 1, 2, 0), (0, 2, 7), (0, 0, 0)), // cql12: the first leg of the Spanish Super Cup
];

/// A Supercopa do Japão: um jogo em março no estádio neutro, 150 mil por vencer.
pub(crate) const JAPAN_SUPER_CUP_ROUNDS: [FraSuperCupRound; 1] = [
    cql12r((1, 1, 0, 5), (4, 2, 0, 5, 1, 4), 0, (0, 1, 0), 6, (2, 1, 2, 0), (0, 1, 0), (0, 0x249f0, 0)), // cql12: the Japanese Super Cup match
];

/// rule: the French Super Cup is between the First Division champion and the Coupe de France winner; if the same club
/// won both, the First Division runner-up takes its place; if one is missing, the error box
/// Original: `FraSuper::select_teams` (0x5bf770, fra_super.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn fra_super_select_teams(this: usize) -> i32 {
    french_super_cup_select_teams(Cup(this)).0
}

/// The season turnover of the French Super Cup: last season's entrant, round and tie tables freed, the season archived in the
/// statistics, the year moved on by one and the entrants, the round table and the tie tree rebuilt; no predicted positions: it returns the eax of the tie tree.
///
/// The cup turnover template (`usa_mls_all_stars_cup_start_new_season`, checked instruction by instruction against the sisters): `cup::SEASON_CLOSED` = 0;
/// the team table (`cup::TEAM_TABLE`) and the round table (`cup::ROUND_TABLE`) freed (`exe::crt_free`) and zeroed; the
/// ties of each round (`cup::TIE_TABLES`, `cup::ROUND_COUNT` rounds) freed and zeroed, then the array; with statistics,
/// Comp_Stats::archive_season_and_reset (`exe::comp_stats_archive_season_and_reset`, thiscall(stats, 1), ret 4); the
/// year (`cup::YEAR`) up by one; `cup::CURRENT_ROUND` = 0 and `cup::RUN_BY_STAGES` = 0. Then, each with its own box
/// (fra_super.cpp, lines 0x127, 0x12e and 0x135) when it returns a negative value: the entrants (FraSuper::select_teams at 0x5bf770, thiscall, ret 0), the round
/// table (virtual `comp_vtable::SET_UP_RULES`) and the tie tree (virtual `cup_vtable::MAP_FIXTURE_TREE`).
/// After a box the copy returns 0 (the original returns the box's eax; the turnover's caller does not read it).
/// Original: `FraSuper::start_new_season` (0x5bf9a0, fra_super.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn fra_super_start_new_season(this: usize) -> u32 {
    usa_mls_all_stars_cup_start_new_season(Cup(this), 0x5bf770, 0x9ccc48, [0x127, 0x12e, 0x135], 1, false, "mwf_3 French Super Cup rollover") // mwf3: the turnover of the French Super Cup
}

/// Comparer for `fra_super_start_new_season`: only the original runs (it frees and rebuilds the cup); the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_fra_super_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_FRA_SUPER_START_NEW_SEASON);
    compare_cup_new_season("fra_super_start_new_season", Cup(this), || o(this))
}

/// The constructor of the French Super Cup: with no entrant table of its own; type 1 (clubs of one nation), no prize
/// fund (`cup::PRIZE_FUND` stays as the base left it), bench and substitutions from the base, discipline rule 12; on a
/// new cup, in order: the two clubs (FraSuper::select_teams, `fra_super_select_teams`, 0x5bf770, thiscall, ret 0; the
/// error box of line 0x3f if it returns a negative value); the rounds (the cup's own virtual
/// `comp_vtable::CREATE_SCHEDULE`, into `cup::ROUND_TABLE`); the tie tree (Cup::map_fixture_tree, `map_fixture_tree`,
/// 0x518790, thiscall, ret 0; the error box of line 0x4d if it returns a negative value); the statistics at Comp +8
/// (the error box of line 0x54 without them). Over the base Cup::Cup (`exe::new_cup`, thiscall, ret 0) it writes the
/// fixed fields in the original's order (the vtable 0x96ace0, the competition record at Comp +4, `cup::YEAR` and those
/// of `FRENCH_SUPER_CUP_CONSTRUCTOR_FIELDS`); if the saved state loads (Cup::load_from_disk 0x51fc00: push 1; mov ecx,
/// this -> thiscall, ret 4; returns ≠ 0) it returns this. Each step that fails shows its own error box
/// (`show_error_at`) and returns this. Returns this.
/// Original: `FraSuper::FraSuper` (0x5bf320, fra_super.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_fra_super(this: usize, year: u32, record: usize) -> usize {
    construct_fra_super_body(Cup(this), year, record) // cqu7: the French Super Cup
}

/// Comparer for `construct_fra_super`: only the original runs (it allocates, draws and reads the save file), and the plan
/// is checked (`fra_cup_check_constructor_plan`).
pub(crate) unsafe extern "thiscall" fn compare_construct_fra_super(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_FRA_SUPER)(this, year, record);
    let error = fra_cup_check_constructor_plan(Cup(this), r, year, record, 0x96ace0, &FRENCH_SUPER_CUP_CONSTRUCTOR_FIELDS, true);
    SCOREBOARD.record(error.is_none(), || format!("construct_fra_super({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
    r
}

/// The body of `construct_fra_super` (its arguments are what the mutation tests change).
pub(crate) unsafe fn construct_fra_super_body(this: Cup, year: u32, record: usize) -> usize {
    exe::new_cup(this);
    write_usize(this.addr() + 4, record); // the competition record (Comp +4)
    write_usize(this.addr(), 0x96ace0); // the vtable
    write_u16(this.addr() + cup::YEAR, year as u16);
    this.set_current_round(0);
    this.set_stage_index(stage_index::MAIN);
    this.set_stage_count(0);
    write_i32(this.addr() + cup::CURRENT_STAGE, -1);
    this.set_club_sides(1);
    this.set_discipline_index(discipline_rule::FRANCE);
    this.set_run_by_stages(0);
    let load: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_CUP_LOAD_FROM_DISK); // Cup::load_from_disk
    if load(this.addr(), 1) != 0 {
        return this.addr();
    }
    covered!("cqu_7 french super cup new");
    let select_teams: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(FN_FRASUPER_SELECT_TEAMS); // FraSuper::select_teams
    let r = select_teams(this.addr());
    if r < 0 {
        show_error_at(0x9ccc48, 0x3f);
        return this.addr();
    }
    fra_cup_create_rounds(this);
    let map_fixture_tree: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(FN_CUP_MAP_FIXTURE_TREE); // Cup::map_fixture_tree
    let r = map_fixture_tree(this.addr());
    if r < 0 {
        show_error_at(0x9ccc48, 0x4d);
        return this.addr();
    }
    if !fra_cup_create_comp_stats(this) {
        show_error_at(0x9ccc48, 0x54);
        return this.addr();
    }
    this.addr()
}

/// The fixed fields `construct_fra_super` writes on a new cup, for the comparer: (offset, width, value).
pub(crate) const FRENCH_SUPER_CUP_CONSTRUCTOR_FIELDS: [(usize, u8, u32); 7] = [(cup::STAGE_COUNT, 4, 0x0), (cup::CURRENT_STAGE, 4, 0xffffffff), (cup::CLUB_SIDES, 1, 0x1), (cup::STAGE_INDEX, 1, 0xff), (cup::DISCIPLINE_INDEX, 1, 0xc), (cup::CURRENT_ROUND, 1, 0x0), (cup::RUN_BY_STAGES, 1, 0x0)];
