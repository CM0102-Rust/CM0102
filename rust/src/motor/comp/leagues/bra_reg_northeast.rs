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
pub(crate) struct StateLeague {
    pub(crate) vtabela: usize,
    pub(crate) stages: u32,
    pub(crate) rules: usize,
    pub(crate) table: usize,
    pub(crate) predicted: usize,
    pub(crate) file: usize,
    pub(crate) rows: [u32; 5],
}

/// The constructor of a Brazilian state championship (the same 544-byte template in all twelve, checked instruction by
/// instruction: only the vtable, the stages, the three own routines, the source file and the lines change).
///
/// League::League (`exe::new_league`, thiscall, ret 0); +4 = the competition record; the vtable; the year (+0x40, word) = `ano`;
/// the discipline rule (+0x50) = 7. If the saved state loads (League::load_from_temp_file `exe::league_load_from_temp_file`, thiscall(this, 1),
/// ret 4; returns ≠ 0), returns this.
///
/// Otherwise (a new career): +0x51 = 0xf (the importance), +0x44 = 0xff, +0x30 = −1, +0x2c = the stages and their
/// vector (malloc `exe::crt_malloc`, unchecked); then, each with its own box (and returning this) if it returns 0: the rules,
/// the table, the fixtures (League::create_fixtures `create_fixtures`, thiscall, ret 0) and the order (League::sort_table
/// `sort_table`, thiscall(this, 0), ret 4); the stats (operator new `exe::crt_operator_new` of 0x5ce and Comp_Stats::Comp_Stats `exe::new_comp_stats`,
/// thiscall(new, this), ret 4; the original's SEH frame covers only that new) in +8 (without them: the box); finally
/// the predicted positions.
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
pub(crate) unsafe fn bra_reg_northeast_construct_state_league(this: League, ano: u32, registro: usize, e: &StateLeague, disciplina: u8, predicted_yes: bool, ramo: &'static str) -> usize {
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
    if predicted_yes {
        previstas(this.addr());
    }
    this.addr()
}

/// Comparer of the state league constructor: runs only the original and checks return, vtable, year, record, discipline
/// and new-league fields.
///
/// [pt] O comparador dos três: alocam e sorteiam os jogos: só a original, conferindo o que ela grava: a devolução, a
/// vtable, o ano, o registro e a disciplina; na liga nova (+0x44 = 0xff, com as estatísticas), +0x2c, o vetor das
/// fases e a importância +0x51 = 0xf.
pub(crate) unsafe fn bra_reg_northeast_compare_state_league_ctor(name: &str, this: League, year: u32, record: usize, e: &StateLeague, original: impl FnOnce() -> usize) -> usize {
    let r = original();
    let mut ok = r == this.addr() && read_usize(this.addr()) == e.vtabela && (this.year() as u16) == year as u16
        && this.competition().addr() == record && this.discipline_rule() == 7;
    if this.stage_index() == 0xff && this.comp_stats() != 0 {
        ok = ok && this.stage_count() == e.stages as usize && this.stages() != 0 && this.importance() == 0xf;
    }
    SCOREBOARD.record(ok, || format!("{}({:#x}): devolveu {:#x}, vtable {:#x}, +0x2c = {}, +0xc = {:#x}, +8 = {:#x}", name, this.addr(), r, read_usize(this.addr()), this.stage_count(), this.stages(), this.comp_stats()));
    r
}

pub(crate) const NORDESTE_STATE_LEAGUE: StateLeague = StateLeague {
    vtabela: 0x9683ec,
    stages: 1,
    rules: 0x4322d0,
    table: 0x436d90,
    predicted: 0x437290,
    file: 0x98af54,
    rows: [0x4d, 0x54, 0x5b, 0x64, 0x6b],
};

pub(crate) const PARANA_CHAMPIONSHIP: StateLeague = StateLeague {
    vtabela: 0x9684a0,
    stages: 3,
    rules: 0x437fc0,
    table: 0x437ec0,
    predicted: 0x438590,
    file: 0x98af98,
    rows: [0x4e, 0x55, 0x5b, 0x62, 0x69],
};

pub(crate) const PERNAMBUCANO_STATE_CHAMPIONSHIP: StateLeague = StateLeague {
    vtabela: 0x968554,
    stages: 1,
    rules: 0x439430,
    table: 0x439330,
    predicted: 0x4398d0,
    file: 0x98afd8,
    rows: [0x4d, 0x54, 0x5b, 0x64, 0x6b],
};

/// The Northeast state league at career creation: the year and the database competition, disciplinary rule 7 and, with
/// no saved state, importance 15, a round robin and the knockout (one stage besides the league), the rules, the table
/// with the state clubs, the fixtures, the order, the statistics and the predicted positions
/// Original: `BraRegNortheast::BraRegNortheast` (0x4363f0, bra_reg_northeast.cpp, thiscall, ret 8).
///
/// [pt] { 0x4363f0 } (thiscall, ret 8, bra_reg_northeast.cpp): `BraRegNortheast::BraRegNortheast(ano, registro)`: o Nordeste (uma fase além da liga).
pub(crate) unsafe extern "thiscall" fn construct_bra_reg_northeast(this: usize, year: u32, record: usize) -> usize {
    bra_reg_northeast_construct_state_league(League(this), year, record, &NORDESTE_STATE_LEAGUE, 7, true, "cqt_3 nordeste novo") // cqt3: the discipline and expected positions of the Nordeste
}

/// the Northeast league has 16 clubs: those with it as their reserve division in the database (the field that holds the
/// state or regional league), in database order; the table only counts when full; no TV money nor prize fund
/// Original: `BraRegNortheast::create_table` (0x436d90, bra_reg_northeast.cpp, thiscall, ret 0).
///
/// [pt] { 0x436d90 } (thiscall, ret 0, bra_reg_northeast.cpp): `BraRegNortheast::create_table()`: o Nordeste tem 16 clubes (divisão reserva +0x60 = [0x9cf9a4]).
/// Fonte 0x98af54, linha 0x159.
pub(crate) unsafe extern "thiscall" fn bra_reg_northeast_create_table(this: usize) -> u32 {
    fill_state_league_table(League(this), 0x10, 0x98af54, 0x159, KEY_CLUB_COMP_BRAZILIAN_NORTHEAST_STATE_CHAMPIONSHIP, 0, "cqm_7 nordeste cheio")
}

/// Northeast: after the round robin, the top 4 go to the semi-finals: 1st v 4th, 2nd v 3rd (the better placed is the
/// seed; the ties follow the seeds [inference: two legs, the decider at the better placed club])
/// Original: `BraRegNortheast::create_semi_final_stage` (0x4370d0, bra_reg_northeast.cpp, thiscall, ret 0).
///
/// [pt] { 0x4370d0 } (thiscall, ret 0, bra_reg_northeast.cpp): `BraRegNortheast::create_semi_final_stage()`, da fase seguinte (advance_stage, quando a fase passa de −1 a
/// 0). As semifinais: 1º × 4º, 2º × 3º (linhas 0, 3, 1, 2). Fonte 0x98af54, linhas 0x20d (sem lista) e 0x221 (sem fase).
pub(crate) unsafe extern "thiscall" fn bra_reg_northeast_create_semi_final_stage(this: usize) {
    create_knockout_from_table(League(this), &[0, 3, 1, 2], &[0, 3, 1, 2], 0x98af54, 0x20d, 0x221, "cqn_7 nordeste"); // cqn7: 1º × 4º, 2º × 3º Nordeste
}

/// The Northeast turnover: archives the season in the statistics, clears the previous table and stages, moves to the
/// next year and rebuilds the rules, the table with the state clubs, the fixtures (round robin) and the table order;
/// then the predicted positions and the renewal of the state referees
/// Original: `BraRegNortheast::start_new_season` (0x436e90, bra_reg_northeast.cpp, thiscall, ret 0).
///
/// [pt] { 0x436e90 } (thiscall, ret 0, bra_reg_northeast.cpp): `BraRegNortheast::start_new_season()`, a virada da temporada do Nordeste (a tabela:
/// 0x436d90; caixas nas linhas 0x1bc, 0x1c3, 0x1c9 e 0x1d0).
pub(crate) unsafe extern "thiscall" fn bra_reg_northeast_start_new_season(this: usize) -> u32 {
    state_league_new_season(League(this), 0x436d90, 0x98af54, [0x1bc, 0x1c3, 0x1c9, 0x1d0], 1, "cqp_8 virada do nordeste") // cqp8: the next year of the Nordeste
}

/// The Northeast next stage: the round robin is stage −1; when it ends, the stage becomes 0 and the top 4 go to the
/// semi-finals (1st v 4th, 2nd v 3rd); past the last stage it is an error (the box)
/// Original: `BraRegNortheast::advance_stage` (0x437040, bra_reg_northeast.cpp, thiscall, ret 0).
///
/// [pt] { 0x437040 } (thiscall, ret 0, bra_reg_northeast.cpp): `BraRegNortheast::advance_stage()`, a fase seguinte do Nordeste: no fim dos pontos corridos,
/// o mata-mata de 4 (0x4370d0); a caixa da linha 0x1ea.
pub(crate) unsafe extern "thiscall" fn bra_reg_northeast_advance_stage(this: usize) {
    state_league_advance_stage(League(this), 0x98af54, 0x1ea, 0x4370d0);
}

