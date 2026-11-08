use super::*;


/// Um estadual: a vtable, as fases além da liga (+0x2c: 1 nos de pontos corridos com mata-mata, 3 nos de dois turnos e
/// final; o vetor [+0xc] = malloc de 4 × fases), as regras (thiscall, ret 0), a tabela (thiscall, ret 0), as posições
/// previstas (thiscall, ret 0), o fonte e as linhas das caixas (regras, tabela, jogos, ordem, estatísticas).
pub(crate) struct StateLeague {
    pub(crate) vtabela: usize,
    pub(crate) fases: u32,
    pub(crate) regras: usize,
    pub(crate) tabela: usize,
    pub(crate) previstas: usize,
    pub(crate) arquivo: usize,
    pub(crate) linhas: [u32; 5],
}

/// Constructor of a Brazilian state league: League base, record, vtable, year, discipline; on a new career sets fields,
/// stages, rules, table, fixtures, order, stats and predictions.
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
pub(crate) unsafe fn bra_reg_northeast_construct_state_league(this: usize, ano: u32, registro: usize, e: &StateLeague, disciplina: u8, previstas_sim: bool, ramo: &'static str) -> usize {
    let base: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LEAGUE_LEAGUE);
    let carregar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_LEAGUE_LOAD_FROM_TEMP_FILE);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let regras: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(e.regras);
    let tabela: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(e.tabela);
    let jogos: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_LEAGUE_CREATE_FIXTURES);
    let ordenar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_LEAGUE_SORT_TABLE);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let estatisticas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_COMP_STATS_COMP_STATS);
    let previstas: unsafe extern "thiscall" fn(usize) = core::mem::transmute(e.previstas);
    base(this);
    core::ptr::write_unaligned((this + 4) as *mut usize, registro);
    core::ptr::write_unaligned(this as *mut usize, e.vtabela);
    core::ptr::write_unaligned((this + league::YEAR) as *mut u16, ano as u16);
    *((this + league::DISCIPLINE_RULE) as *mut u8) = disciplina;
    if carregar(this, 1) != 0 {
        return this;
    }
    crate::comparison::branch(ramo);
    *((this + league::IMPORTANCE) as *mut u8) = 0xf;
    *((this + league::STAGE_INDEX) as *mut u8) = 0xff;
    core::ptr::write_unaligned((this + league::CURRENT_STAGE) as *mut i32, -1);
    core::ptr::write_unaligned((this + league::STAGE_COUNT) as *mut u32, e.fases);
    let v = alocar(e.fases * 4);
    core::ptr::write_unaligned((this + league::STAGES) as *mut usize, v);
    if regras(this) == 0 {
        show_error_at(e.arquivo, e.linhas[0]);
        return this;
    }
    if tabela(this) == 0 {
        show_error_at(e.arquivo, e.linhas[1]);
        return this;
    }
    if jogos(this) == 0 {
        show_error_at(e.arquivo, e.linhas[2]);
        return this;
    }
    if ordenar(this, 0) == 0 {
        show_error_at(e.arquivo, e.linhas[3]);
        return this;
    }
    let n = novo(0x5ce);
    let s = if n != 0 { estatisticas(n, this) } else { 0 };
    core::ptr::write_unaligned((this + 8) as *mut usize, s);
    if s == 0 {
        show_error_at(e.arquivo, e.linhas[4]);
        return this;
    }
    if previstas_sim {
        previstas(this);
    }
    this
}

/// Comparer of the state league constructor: runs only the original and checks return, vtable, year, record, discipline
/// and new-league fields.
///
/// [pt] O comparador dos três: alocam e sorteiam os jogos: só a original, conferindo o que ela grava: a devolução, a
/// vtable, o ano, o registro e a disciplina; na liga nova (+0x44 = 0xff, com as estatísticas), +0x2c, o vetor das
/// fases e a importância +0x51 = 0xf.
pub(crate) unsafe fn bra_reg_northeast_compare_state_league_ctor(name: &str, this: usize, year: u32, record: usize, e: &StateLeague, original: impl FnOnce() -> usize) -> usize {
    let r = original();
    let mut ok = r == this && read_usize(this) == e.vtabela && core::ptr::read_unaligned((this + league::YEAR) as *const u16) == year as u16
        && read_usize(this + 4) == record && *((this + league::DISCIPLINE_RULE) as *const u8) == 7;
    if *((this + league::STAGE_INDEX) as *const u8) == 0xff && read_usize(this + 8) != 0 {
        ok = ok && read_usize(this + league::STAGE_COUNT) == e.fases as usize && read_usize(this + league::STAGES) != 0 && *((this + league::IMPORTANCE) as *const u8) == 0xf;
    }
    SCOREBOARD.record(ok, || format!("{}({:#x}): devolveu {:#x}, vtable {:#x}, +0x2c = {}, +0xc = {:#x}, +8 = {:#x}", name, this, r, read_usize(this), read_usize(this + league::STAGE_COUNT), read_usize(this + league::STAGES), read_usize(this + 8)));
    r
}

pub(crate) const NORDESTE_STATE_LEAGUE: StateLeague = StateLeague {
    vtabela: 0x9683ec,
    fases: 1,
    regras: 0x4322d0,
    tabela: 0x436d90,
    previstas: 0x437290,
    arquivo: 0x98af54,
    linhas: [0x4d, 0x54, 0x5b, 0x64, 0x6b],
};

pub(crate) const PARANA_CHAMPIONSHIP: StateLeague = StateLeague {
    vtabela: 0x9684a0,
    fases: 3,
    regras: 0x437fc0,
    tabela: 0x437ec0,
    previstas: 0x438590,
    arquivo: 0x98af98,
    linhas: [0x4e, 0x55, 0x5b, 0x62, 0x69],
};

pub(crate) const PERNAMBUCANO_STATE_CHAMPIONSHIP: StateLeague = StateLeague {
    vtabela: 0x968554,
    fases: 1,
    regras: 0x439430,
    tabela: 0x439330,
    previstas: 0x4398d0,
    arquivo: 0x98afd8,
    linhas: [0x4d, 0x54, 0x5b, 0x64, 0x6b],
};

/// The Northeast state league at career creation: the year and the database competition, disciplinary rule 7 and, with
/// no saved state, importance 15, a round robin and the knockout (one stage besides the league), the rules, the table
/// with the state clubs, the fixtures, the order, the statistics and the predicted positions
/// Original: `BraRegNortheast::BraRegNortheast` (0x4363f0, bra_reg_northeast.cpp, thiscall, ret 8).
///
/// [pt] { 0x4363f0 } (thiscall, ret 8, bra_reg_northeast.cpp): `BraRegNortheast::BraRegNortheast(ano, registro)`: o Nordeste (uma fase além da liga).
pub(crate) unsafe extern "thiscall" fn construct_bra_reg_northeast(this: usize, year: u32, record: usize) -> usize {
    bra_reg_northeast_construct_state_league(this, year, record, &NORDESTE_STATE_LEAGUE, 7, true, "cqt_3 nordeste novo") // cqt3: a disciplina e as posições previstas do Nordeste
}










/// the Northeast league has 16 clubs: those with it as their reserve division in the database (the field that holds the
/// state or regional league), in database order; the table only counts when full; no TV money nor prize fund
/// Original: `BraRegNortheast::create_table` (0x436d90, bra_reg_northeast.cpp, thiscall, ret 0).
///
/// [pt] { 0x436d90 } (thiscall, ret 0, bra_reg_northeast.cpp): `BraRegNortheast::create_table()`: o Nordeste tem 16 clubes (divisão reserva +0x60 = [0x9cf9a4]).
/// Fonte 0x98af54, linha 0x159.
pub(crate) unsafe extern "thiscall" fn bra_reg_northeast_create_table(this: usize) -> u32 {
    fill_state_league_table(this, 0x10, 0x98af54, 0x159, KEY_CLUB_COMP_BRAZILIAN_NORTHEAST_STATE_CHAMPIONSHIP, 0, "cqm_7 nordeste cheio")
}





/// Northeast: after the round robin, the top 4 go to the semi-finals: 1st v 4th, 2nd v 3rd (the better placed is the
/// seed; the ties follow the seeds [inference: two legs, the decider at the better placed club])
/// Original: `BraRegNortheast::create_semi_final_stage` (0x4370d0, bra_reg_northeast.cpp, thiscall, ret 0).
///
/// [pt] { 0x4370d0 } (thiscall, ret 0, bra_reg_northeast.cpp): `BraRegNortheast::create_semi_final_stage()`, da fase seguinte (advance_stage, quando a fase passa de −1 a
/// 0). As semifinais: 1º × 4º, 2º × 3º (linhas 0, 3, 1, 2). Fonte 0x98af54, linhas 0x20d (sem lista) e 0x221 (sem fase).
pub(crate) unsafe extern "thiscall" fn bra_reg_northeast_create_semi_final_stage(this: usize) {
    create_knockout_from_table(this, &[0, 3, 1, 2], &[0, 3, 1, 2], 0x98af54, 0x20d, 0x221, "cqn_7 nordeste"); // cqn7: 1º × 4º, 2º × 3º Nordeste
}


/// The Northeast turnover: archives the season in the statistics, clears the previous table and stages, moves to the
/// next year and rebuilds the rules, the table with the state clubs, the fixtures (round robin) and the table order;
/// then the predicted positions and the renewal of the state referees
/// Original: `BraRegNortheast::start_new_season` (0x436e90, bra_reg_northeast.cpp, thiscall, ret 0).
///
/// [pt] { 0x436e90 } (thiscall, ret 0, bra_reg_northeast.cpp): `BraRegNortheast::start_new_season()`, a virada da temporada do Nordeste (a tabela:
/// 0x436d90; caixas nas linhas 0x1bc, 0x1c3, 0x1c9 e 0x1d0).
pub(crate) unsafe extern "thiscall" fn bra_reg_northeast_start_new_season(this: usize) -> u32 {
    state_league_new_season(this, 0x436d90, 0x98af54, [0x1bc, 0x1c3, 0x1c9, 0x1d0], 1, "cqp_8 virada do nordeste") // cqp8: o ano seguinte do Nordeste
}


/// The Northeast next stage: the round robin is stage −1; when it ends, the stage becomes 0 and the top 4 go to the
/// semi-finals (1st v 4th, 2nd v 3rd); past the last stage it is an error (the box)
/// Original: `BraRegNortheast::advance_stage` (0x437040, bra_reg_northeast.cpp, thiscall, ret 0).
///
/// [pt] { 0x437040 } (thiscall, ret 0, bra_reg_northeast.cpp): `BraRegNortheast::advance_stage()`, a fase seguinte do Nordeste: no fim dos pontos corridos,
/// o mata-mata de 4 (0x4370d0); a caixa da linha 0x1ea.
pub(crate) unsafe extern "thiscall" fn bra_reg_northeast_advance_stage(this: usize) {
    state_league_advance_stage(this, 0x98af54, 0x1ea, 0x4370d0);
}

