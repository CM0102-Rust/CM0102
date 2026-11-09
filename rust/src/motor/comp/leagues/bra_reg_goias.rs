use super::*;

/// Describes one state league.
///
/// It holds the vtable, the stages beyond the league (+0x2c: 1 in the round-robin ones with a knockout, 3 in the ones
/// with two rounds and a final; the vector [+0xc] = malloc of 4 × stages), the rules (thiscall, ret 0), the table
/// (thiscall, ret 0), the predicted positions (thiscall, ret 0), the source file and the error-box lines (rules, table,
/// fixtures, order, statistics).
///
/// [pt] Um estadual: a vtable, as fases além da liga (+0x2c: 1 nos de pontos corridos com mata-mata, 3 nos de dois turnos e
/// final; o vetor [+0xc] = malloc de 4 × fases), as regras (thiscall, ret 0), a tabela (thiscall, ret 0), as posições
/// previstas (thiscall, ret 0), o fonte e as linhas das caixas (regras, tabela, jogos, ordem, estatísticas).
pub(crate) struct BraRegGoiasStateChampionship {
    pub(crate) vtabela: usize,
    pub(crate) stages: u32,
    pub(crate) rules: usize,
    pub(crate) table: usize,
    pub(crate) predicted: usize,
    pub(crate) file: usize,
    pub(crate) rows: [u32; 5],
}

/// The constructor of a Brazilian state league (the same 544-byte template in all twelve, checked instruction by
/// instruction: only the vtable, the stages, the three own routines, the source file and the line numbers change).
///
/// League::League (`exe::new_league`, thiscall, ret 0); +4 = the competition record; the vtable; the year (+0x40, word) = `ano`;
/// the discipline rule (+0x50) = 7. If the saved state loads (League::load_from_temp_file `exe::league_load_from_temp_file`, thiscall(this, 1),
/// ret 4; returns ≠ 0), returns this.
///
/// Otherwise (new career): +0x51 = 0xf (the importance), +0x44 = 0xff, +0x30 = −1, +0x2c = the stages and their array
/// (malloc `exe::crt_malloc`, unchecked); then, each with its own error box (returning this) if it returns 0: the rules, the
/// table, the fixtures (League::create_fixtures `create_fixtures`, thiscall, ret 0) and the sort (League::sort_table `sort_table`,
/// thiscall(this, 0), ret 4); the stats (operator new `exe::crt_operator_new` of 0x5ce and Comp_Stats::Comp_Stats `exe::new_comp_stats`,
/// thiscall(new, this), ret 4; the original's SEH frame covers only that new) at +8 (without them: the error box);
/// finally the predicted positions.
///
/// [pt] O construtor de um estadual (o mesmo molde de 544 bytes nos doze, conferido instrução a instrução: só mudam a
/// vtable, as fases, as três rotinas próprias, o fonte e as linhas). League::League (0x682200, thiscall, ret 0); +4 = o
/// registro da competição; a vtable; o ano (+0x40, word) = `ano`; a regra de disciplina (+0x50) = 7. Se o estado
/// salvo carrega (League::load_from_temp_file 0x687b10, thiscall(this, 1), ret 4; devolve ≠ 0), devolve this. Senão
/// (a carreira nova): +0x51 = 0xf (a importância), +0x44 = 0xff, +0x30 = −1, +0x2c = as fases e o vetor delas (malloc
/// 0x944e46, sem conferir); então, cada um com a sua caixa (e devolvendo this) se devolver 0: as regras, a tabela, os
/// jogos (League::create_fixtures 0x6835c0, thiscall, ret 0) e a ordem (League::sort_table 0x6827d0, thiscall(this,
/// 0), ret 4); as estatísticas (operator new 0x944cf1 de 0x5ce e Comp_Stats::Comp_Stats 0x49ee70, thiscall(novo,
/// this), ret 4; o quadro SEH da original cobre só esse new) em +8 (sem elas: a caixa); por fim as posições previstas.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn bra_reg_goias_construct_state_league(this: League, ano: u32, registro: usize, e: &BraRegGoiasStateChampionship, disciplina: u8, previstas_sim: bool, ramo: &'static str) -> usize {
    let regras: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(e.rules);
    let tabela: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(e.table);
    let previstas: unsafe extern "thiscall" fn(usize) = core::mem::transmute(e.predicted);
    exe::new_league(this);
    core::ptr::write_unaligned((this.addr() + league::COMPETITION) as *mut usize, registro);
    core::ptr::write_unaligned(this.addr() as *mut usize, e.vtabela);
    core::ptr::write_unaligned((this.addr() + league::YEAR) as *mut u16, ano as u16);
    this.set_discipline_rule(disciplina);
    if exe::league_load_from_temp_file(this, 1) != 0 {
        return this.addr();
    }
    crate::comparison::branch(ramo);
    this.set_importance(0xf);
    this.set_stage_index(0xff);
    core::ptr::write_unaligned((this.addr() + league::CURRENT_STAGE) as *mut i32, -1);
    core::ptr::write_unaligned((this.addr() + league::STAGE_COUNT) as *mut u32, e.stages);
    let v = exe::crt_malloc(e.stages * 4);
    core::ptr::write_unaligned((this.addr() + league::STAGES) as *mut usize, v);
    if regras(this.addr()) == 0 {
        show_error_at(e.file, e.rows[0]);
        return this.addr();
    }
    if tabela(this.addr()) == 0 {
        show_error_at(e.file, e.rows[1]);
        return this.addr();
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(e.file, e.rows[2]);
        return this.addr();
    }
    if exe::league_sort_table(this, (0) as u32 as usize) == 0 {
        show_error_at(e.file, e.rows[3]);
        return this.addr();
    }
    let n = exe::crt_operator_new(0x5ce);
    let s = if n != 0 { exe::new_comp_stats(n, this) } else { 0 };
    core::ptr::write_unaligned((this.addr() + league::COMP_STATS) as *mut usize, s);
    if s == 0 {
        show_error_at(e.file, e.rows[4]);
        return this.addr();
    }
    if previstas_sim {
        previstas(this.addr());
    }
    this.addr()
}

/// Comparer for state league constructors: runs only the original and checks return, vtable, year, record, discipline
/// and, for a new league, stages, stage vector and importance.
///
/// [pt] O comparador dos três: alocam e sorteiam os jogos: só a original, conferindo o que ela grava: a devolução, a
/// vtable, o ano, o registro e a disciplina; na liga nova (+0x44 = 0xff, com as estatísticas), +0x2c, o vetor das
/// fases e a importância +0x51 = 0xf.
pub(crate) unsafe fn bra_reg_goias_compare_state_league_ctor(name: &str, this: League, year: u32, record: usize, e: &BraRegGoiasStateChampionship, original: impl FnOnce() -> usize) -> usize {
    let r = original();
    let mut ok = r == this.addr() && read_usize(this.addr()) == e.vtabela && (this.year() as u16) == year as u16
        && this.competition().addr() == record && this.discipline_rule() == 7;
    if this.stage_index() == 0xff && this.comp_stats() != 0 {
        ok = ok && this.stage_count() == e.stages as usize && this.stages() != 0 && this.importance() == 0xf;
    }
    SCOREBOARD.record(ok, || format!("{}({:#x}): devolveu {:#x}, vtable {:#x}, +0x2c = {}, +0xc = {:#x}, +8 = {:#x}", name, this.addr(), r, read_usize(this.addr()), this.stage_count(), this.stages(), this.comp_stats()));
    r
}

pub(crate) const GOIANO_STATE_CHAMPIONSHIP: BraRegGoiasStateChampionship = BraRegGoiasStateChampionship {
    vtabela: 0x9681d0,
    stages: 1,
    rules: 0x433640,
    table: 0x433540,
    predicted: 0x433b40,
    file: 0x98ae90,
    rows: [0x4c, 0x53, 0x59, 0x60, 0x67],
};

pub(crate) const MINEIRO_STATE_LEAGUE: BraRegGoiasStateChampionship = BraRegGoiasStateChampionship {
    vtabela: 0x968284,
    stages: 3,
    rules: 0x43a600,
    table: 0x434810,
    predicted: 0x434f80,
    file: 0x98aed0,
    rows: [0x4e, 0x55, 0x5b, 0x62, 0x69],
};

pub(crate) const NORTH_CHAMPIONSHIP: BraRegGoiasStateChampionship = BraRegGoiasStateChampionship {
    vtabela: 0x968338,
    stages: 1,
    rules: 0x439430,
    table: 0x435d80,
    predicted: 0x436280,
    file: 0x98af14,
    rows: [0x4d, 0x54, 0x5b, 0x64, 0x6b],
};

/// The Goias state league at career creation: the year and the database competition, disciplinary rule 7 and, with no
/// saved state, importance 15, a round robin and the knockout (one stage besides the league), the rules, the table with
/// the state clubs, the fixtures, the order, the statistics and the predicted positions
/// Original: `BraRegGoias::BraRegGoias` (0x432b50, bra_reg_goias.cpp, thiscall, ret 8).
///
/// [pt] { 0x432b50 } (thiscall, ret 8, bra_reg_goias.cpp): `BraRegGoias::BraRegGoias(ano, registro)`: o Goiano (uma fase além da liga).
pub(crate) unsafe extern "thiscall" fn construct_bra_reg_goias(this: usize, year: u32, record: usize) -> usize {
    bra_reg_goias_construct_state_league(League(this), year, record, &GOIANO_STATE_CHAMPIONSHIP, 7, true, "cqt_2 goiano novo") // cqt2: the discipline and expected positions of the Goiano
}

/// The Goias turnover: archives the season in the statistics, clears the previous table and stages, moves to the next
/// year and rebuilds the rules, the table with the state clubs, the fixtures (round robin) and the table order; then
/// the predicted positions and the renewal of the state referees
/// Original: `BraRegGoias::start_new_season` (0x4336f0, bra_reg_goias.cpp, thiscall, ret 0).
///
/// [pt] { 0x4336f0 } (thiscall, ret 0, bra_reg_goias.cpp): `BraRegGoias::start_new_season()`, a virada da temporada do Goiano (a tabela:
/// 0x433540; caixas nas linhas 0x1c3, 0x1ca, 0x1d0 e 0x1d7).
pub(crate) unsafe extern "thiscall" fn bra_reg_goias_start_new_season(this: usize) -> u32 {
    state_league_new_season(League(this), 0x433540, 0x98ae90, [0x1c3, 0x1ca, 0x1d0, 0x1d7], 1, "cqp_8 virada do goiano") // cqp8: the next year of the Goiano
}

/// The Goias next stage: the round robin is stage −1; when it ends, the stage becomes 0 and the top 8 go to the
/// quarter-finals (1st v 8th, 3rd v 6th, 7th v 2nd, 5th v 4th); past the last stage it is an error (the box)
/// Original: `BraRegGoias::advance_stage` (0x4338a0, bra_reg_goias.cpp, thiscall, ret 0).
///
/// [pt] { 0x4338a0 } (thiscall, ret 0, bra_reg_goias.cpp): `BraRegGoias::advance_stage()`, a fase seguinte do Goiano: no fim dos pontos corridos,
/// o mata-mata de 8 (0x433930); a caixa da linha 0x1f1.
pub(crate) unsafe extern "thiscall" fn bra_reg_goias_advance_stage(this: usize) {
    state_league_advance_stage(League(this), 0x98ae90, 0x1f1, 0x433930);
}

/// the Goias league has 14 clubs: those with it as their reserve division in the database (the field that holds the
/// state or regional league), in database order; the table only counts when full; no TV money nor prize fund
/// Original: `BraRegGoias::create_table` (0x433540, bra_reg_goias.cpp, thiscall, ret 0).
///
/// [pt] { 0x433540 } (thiscall, ret 0, bra_reg_goias.cpp): `BraRegGoias::create_table()`: o Goiano tem 14 clubes (divisão reserva +0x60 = [0x9cf9b4]).
/// Fonte 0x98ae90, linha 0x160.
pub(crate) unsafe extern "thiscall" fn bra_reg_goias_create_table(this: usize) -> u32 {
    fill_state_league_table(League(this), 0xe, 0x98ae90, 0x160, KEY_CLUB_COMP_BRAZILIAN_GOIAS_STATE_CHAMPIONSHIP, 0, "cql_9 goiano cheio")
}

/// Writes the Goiano league rules (what `bra_reg_goias_set_up_rules` writes before the calendar): points, tiebreaks, 8 qualifiers,
/// substitutions.
///
/// [pt] As regras do Goiano (o que a 0x433640 grava antes do calendário).
pub(crate) unsafe fn goiano_rules(o: usize) {
    write_u16_at(o, 0xab, 0);
    write_u16_at(o, 0xdb, 0);
    write_u8_at(o, 0xc7, 0);
    write_u8_at(o, 0xbf, 0);
    write_u8_at(o, 0xc0, 0);
    write_u8_at(o, 0xc2, 3);
    write_u8_at(o, 0xc5, 3); // cqj7: the first tiebreaker of the Goiano
    write_u8_at(o, 0x4a, 3);
    write_u8_at(o, 0xbe, 8); // the 8 qualified
    write_u16_at(o, 0xd9, 8);
    write_u16_at(o, 0x3c, 1);
    write_u8_at(o, 0xc3, 1);
    write_u8_at(o, 0xc4, 2);
    write_u8_at(o, 0x42, 1);
    write_u8_at(o, 0xc6, 1);
    write_u8_at(o, 0xc1, 4);
    write_u32_at(o, 0x1c, 0xffff_ffff);
    write_u32_at(o, 0x20, 0xffff_ffff);
    write_u8_at(o, 0x49, 7);
}

/// Writes the Japanese J1 rules (`jap_j1_set_up_rules`): two rounds, points, tiebreaks, two relegated to J2.
///
/// [pt] As regras da J1 (0x663620).
pub(crate) unsafe fn j1_rules(o: usize) {
    write_u16_at(o, 0x3c, 2);
    write_u8_at(o, 0xc2, 3);
    write_u8_at(o, 0xc3, 1); // cqj7: the J1 points per draw
    write_u8_at(o, 0xc4, 2);
    write_u16_at(o, 0xab, 0x82);
    write_u16_at(o, 0xdb, 0);
    write_u8_at(o, 0x42, 1);
    write_u8_at(o, 0xc5, 1); // tiebreakers: goal difference, goals for, away goals, none
    write_u8_at(o, 0xc6, 2);
    write_u8_at(o, 0xc7, 4);
    write_u8_at(o, 0xc8, 0);
    write_u8_at(o, 0xbe, 0);
    write_u8_at(o, 0xbf, 0);
    write_u8_at(o, 0xc0, 0);
    write_u8_at(o, 0xc1, 2); // two go down to J2
    write_u32_at(o, 0x1c, 0xffff_ffff);
    write_u8_at(o, 0x52, 2);
    write_u8_at(o, 0x4a, 3);
    write_u32_at(o, 0x20, *(KEY_CLUB_COMP_JAPANESE_J_LEAGUE_2 as *const u32)); // falls to J2
    write_u16_at(o, 0xd9, 8);
    write_u8_at(o, 0x49, 5);
}

/// Writes the Japanese J2 rules (`jap_j2_set_up_rules`): four rounds, points, tiebreaks, two promoted to J1; leaves +0x52 and +0xd9
/// as League::League set them.
///
/// [pt] As regras da J2 (0x665140): não grava +0x52 nem a palavra +0xd9 (fica a de `League::League`).
pub(crate) unsafe fn j2_rules(o: usize) {
    write_u16_at(o, 0x3c, 4); // quatro turnos
    write_u8_at(o, 0xc2, 3); // cqj7: the J2 points per win
    write_u8_at(o, 0xc3, 1);
    write_u8_at(o, 0xc4, 2);
    write_u16_at(o, 0xab, 0x82);
    write_u16_at(o, 0xdb, 0);
    write_u8_at(o, 0x42, 1);
    write_u8_at(o, 0xc5, 1);
    write_u8_at(o, 0xc6, 2);
    write_u8_at(o, 0xc7, 4);
    write_u8_at(o, 0xc8, 0);
    write_u8_at(o, 0xbe, 2); // two go up to the J1
    write_u8_at(o, 0xbf, 0);
    write_u8_at(o, 0xc0, 0);
    write_u8_at(o, 0xc1, 0);
    write_u32_at(o, 0x1c, *(KEY_CLUB_COMP_JAPANESE_J_LEAGUE_1 as *const u32)); // promoted to J1
    write_u8_at(o, 0x4a, 3);
    write_u32_at(o, 0x20, 0xffff_ffff);
    write_u8_at(o, 0x49, 5);
}

/// Goias: single round robin; win 3, draw 1; tie-break by head-to-head, then goal difference; 8 top places (rule word
/// 8: they lead to the next stage, the knockout [inference]); 4 relegated, no league above or below; 7 bench
/// substitutes, 3 substitutions; category 2 matches. Then the schedule
/// Original: `BraRegGoias::set_up_rules` (0x433640, bra_reg_goias.cpp, thiscall).
///
/// [pt] { 0x433640 } (thiscall, bra_reg_goias.cpp): `BraRegGoias::set_up_rules` (slot 36), as regras do Goiano e o
/// calendário; devolve 1.
pub(crate) unsafe extern "thiscall" fn bra_reg_goias_set_up_rules(this: usize) -> u32 {
    goiano_rules(this);
    crate::comparison::branch("cqj_7 goiano rules");
    create_main_schedule(League(this));
    1
}

/// Goias: after the round robin, the top 8 go to the quarter-finals: 1st v 8th, 3rd v 6th, 7th v 2nd, 5th v 4th (the
/// seeds are the positions [inference: two legs])
/// Original: `BraRegGoias::create_knockout_stage` (0x433930, bra_reg_goias.cpp, thiscall, ret 0).
///
/// [pt] { 0x433930 } (thiscall, ret 0, bra_reg_goias.cpp): `BraRegGoias::create_knockout_stage()`, da fase seguinte (advance_stage, quando a fase passa de −1 a
/// 0). As quartas de final: 1º × 8º, 3º × 6º, 7º × 2º, 5º × 4º (linhas 0, 7, 2, 5, 6, 1, 4, 3). Fonte 0x98ae90, linhas 0x214 (sem lista) e 0x22c (sem fase).
pub(crate) unsafe extern "thiscall" fn bra_reg_goias_create_knockout_stage(this: usize) {
    create_knockout_from_table(League(this), &[0, 7, 2, 5, 6, 1, 4, 3], &[0, 7, 2, 5, 6, 1, 4, 3], 0x98ae90, 0x214, 0x22c, "cqn_9 goiano"); // cqn9: 1º × 8º, 3º × 6º, 7º × 2º, 5º × 4º Goiano
}

