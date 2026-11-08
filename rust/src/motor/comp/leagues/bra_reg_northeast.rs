use super::*;

// ---------- 283a leva: os construtores dos estaduais: Nordeste, Paranaense e Pernambucano (as regras, a tabela, os jogos, as estatísticas e as posições previstas na criação) (bra_reg_northeast.cpp, bra_reg_parana.cpp, bra_reg_pern.cpp) ----------


/// Um estadual: a vtable, as fases além da liga (+0x2c: 1 nos de pontos corridos com mata-mata, 3 nos de dois turnos e
/// final; o vetor [+0xc] = malloc de 4 × fases), as regras (thiscall, ret 0), a tabela (thiscall, ret 0), as posições
/// previstas (thiscall, ret 0), o fonte e as linhas das caixas (regras, tabela, jogos, ordem, estatísticas).
pub(crate) struct Cqt3Estadual {
    pub(crate) vtabela: usize,
    pub(crate) fases: u32,
    pub(crate) regras: usize,
    pub(crate) tabela: usize,
    pub(crate) previstas: usize,
    pub(crate) arquivo: usize,
    pub(crate) linhas: [u32; 5],
}

/// O construtor de um estadual (o mesmo molde de 544 bytes nos doze, conferido instrução a instrução: só mudam a
/// vtable, as fases, as três rotinas próprias, o fonte e as linhas). League::League (0x682200, thiscall, ret 0); +4 = o
/// registro da competição; a vtable; o ano (+0x40, word) = `ano`; a regra de disciplina (+0x50) = 7. Se o estado
/// salvo carrega (League::load_from_temp_file 0x687b10, thiscall(this, 1), ret 4; devolve ≠ 0), devolve this. Senão
/// (a carreira nova): +0x51 = 0xf (a importância), +0x44 = 0xff, +0x30 = −1, +0x2c = as fases e o vetor delas (malloc
/// 0x944e46, sem conferir); então, cada um com a sua caixa (e devolvendo this) se devolver 0: as regras, a tabela, os
/// jogos (League::create_fixtures 0x6835c0, thiscall, ret 0) e a ordem (League::sort_table 0x6827d0, thiscall(this,
/// 0), ret 4); as estatísticas (operator new 0x944cf1 de 0x5ce e Comp_Stats::Comp_Stats 0x49ee70, thiscall(novo,
/// this), ret 4; o quadro SEH da original cobre só esse new) em +8 (sem elas: a caixa); por fim as posições previstas.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn cqt3_construir(this: usize, ano: u32, registro: usize, e: &Cqt3Estadual, disciplina: u8, previstas_sim: bool, ramo: &'static str) -> usize {
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
    core::ptr::write_unaligned((this + 0x40) as *mut u16, ano as u16);
    *((this + 0x50) as *mut u8) = disciplina;
    if carregar(this, 1) != 0 {
        return this;
    }
    crate::comparacao::ramo(ramo);
    *((this + 0x51) as *mut u8) = 0xf;
    *((this + 0x44) as *mut u8) = 0xff;
    core::ptr::write_unaligned((this + 0x30) as *mut i32, -1);
    core::ptr::write_unaligned((this + 0x2c) as *mut u32, e.fases);
    let v = alocar(e.fases * 4);
    core::ptr::write_unaligned((this + 0xc) as *mut usize, v);
    if regras(this) == 0 {
        aviso_em(e.arquivo, e.linhas[0]);
        return this;
    }
    if tabela(this) == 0 {
        aviso_em(e.arquivo, e.linhas[1]);
        return this;
    }
    if jogos(this) == 0 {
        aviso_em(e.arquivo, e.linhas[2]);
        return this;
    }
    if ordenar(this, 0) == 0 {
        aviso_em(e.arquivo, e.linhas[3]);
        return this;
    }
    let n = novo(0x5ce);
    let s = if n != 0 { estatisticas(n, this) } else { 0 };
    core::ptr::write_unaligned((this + 8) as *mut usize, s);
    if s == 0 {
        aviso_em(e.arquivo, e.linhas[4]);
        return this;
    }
    if previstas_sim {
        previstas(this);
    }
    this
}

/// O comparador dos três: alocam e sorteiam os jogos: só a original, conferindo o que ela grava: a devolução, a
/// vtable, o ano, o registro e a disciplina; na liga nova (+0x44 = 0xff, com as estatísticas), +0x2c, o vetor das
/// fases e a importância +0x51 = 0xf.
pub(crate) unsafe fn cqt3_comparar(nome: &str, this: usize, ano: u32, registro: usize, e: &Cqt3Estadual, original: impl FnOnce() -> usize) -> usize {
    let r = original();
    let mut ok = r == this && read_usize(this) == e.vtabela && core::ptr::read_unaligned((this + 0x40) as *const u16) == ano as u16
        && read_usize(this + 4) == registro && *((this + 0x50) as *const u8) == 7;
    if *((this + 0x44) as *const u8) == 0xff && read_usize(this + 8) != 0 {
        ok = ok && read_usize(this + 0x2c) == e.fases as usize && read_usize(this + 0xc) != 0 && *((this + 0x51) as *const u8) == 0xf;
    }
    PLACAR.registrar(ok, || format!("{}({:#x}): devolveu {:#x}, vtable {:#x}, +0x2c = {}, +0xc = {:#x}, +8 = {:#x}", nome, this, r, read_usize(this), read_usize(this + 0x2c), read_usize(this + 0xc), read_usize(this + 8)));
    r
}

pub(crate) const CQT3_NORDESTE: Cqt3Estadual = Cqt3Estadual {
    vtabela: 0x9683ec,
    fases: 1,
    regras: 0x4322d0,
    tabela: 0x436d90,
    previstas: 0x437290,
    arquivo: 0x98af54,
    linhas: [0x4d, 0x54, 0x5b, 0x64, 0x6b],
};

pub(crate) const CQT3_PARANAENSE: Cqt3Estadual = Cqt3Estadual {
    vtabela: 0x9684a0,
    fases: 3,
    regras: 0x437fc0,
    tabela: 0x437ec0,
    previstas: 0x438590,
    arquivo: 0x98af98,
    linhas: [0x4e, 0x55, 0x5b, 0x62, 0x69],
};

pub(crate) const CQT3_PERNAMBUCANO: Cqt3Estadual = Cqt3Estadual {
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
pub(crate) unsafe extern "thiscall" fn cqt3_construir_nordeste(this: usize, ano: u32, registro: usize) -> usize {
    cqt3_construir(this, ano, registro, &CQT3_NORDESTE, 7, true, "cqt_3 nordeste novo") // cqt3: a disciplina e as posições previstas do Nordeste
}



pub(crate) unsafe extern "thiscall" fn comparar_cqt3_construir_nordeste(this: usize, ano: u32, registro: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(2052);
    cqt3_comparar("cqt3_construir_nordeste", this, ano, registro, &CQT3_NORDESTE, || o(this, ano, registro))
}



// ---------- 277a leva: a tabela dos estaduais: Nordeste, Paranaense e Pernambucano (bra_reg_northeast.cpp, bra_reg_parana.cpp, bra_reg_pern.cpp) ----------



/// O clube é do estadual do global `g`: a divisão reserva do clube (+0x60, ReserveDivision, que no Brasil guarda o
/// estadual) não é nula e o id do registro (+0) é o do global.
pub(crate) unsafe fn cqm7_do_estadual(clube: usize, g: usize) -> bool {
    let comp = read_usize(clube + 0x60);
    comp != 0 && read_usize(comp) == read_usize(g)
}

/// O corpo comum dos estaduais (o mesmo molde nos doze, só mudam as constantes): +0x3e = `n` clubes; a tabela
/// (malloc 0x944e46 de n × 0x3b; sem memória: a caixa do fonte `arquivo` na linha `linha` e devolve 0); os clubes de
/// [0xae23b4] (0x245 bytes, [0xae235c] deles) do estadual do global `g`, na ordem do banco, cada um na linha seguinte
/// (League::init_table_row 0x687430: thiscall(liga, linha, clube, 0, 0), ret 0x10), até encher. Devolve 1 se encheu.
/// `girar` é 0 (o original: a linha k recebe o k-ésimo clube); existe só para as mutações de cada estadual.
pub(crate) unsafe fn cqm7_tabela(this: usize, n: i16, arquivo: usize, linha: u32, g: usize, girar: u8, ramo: &'static str) -> u32 {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, n);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let t = alocar((n as i32 as u32).wrapping_mul(0x3b));
    core::ptr::write_unaligned((this + 0xb1) as *mut usize, t);
    if t == 0 {
        aviso_em(arquivo, linha);
        return 0;
    }
    let iniciar: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(FN_LEAGUE_INIT_TABLE_ROW);
    let mut k: u8 = 0;
    let mut i: i16 = 0;
    let mut clube = read_usize(0xae23b4);
    while (i as i32) < read_usize(0xae235c) as i32 {
        if cqm7_do_estadual(clube, g) {
            // ponytail: a original empurra a linha no eax com os bits de cima do id do estadual; a 0x687430 só lê o
            // byte. Com girar = 0, (k + 0) % n = k.
            let l = ((k as u32 + girar as u32) % n as u32) as u8;
            iniciar(this, l as u32, clube, 0, 0);
            k = k.wrapping_add(1);
            if k as i8 as i16 == read_i16(this + 0x3e) {
                break;
            }
        }
        i = i.wrapping_add(1);
        clube = clube.wrapping_add(0x245);
    }
    if k as i8 as i16 != read_i16(this + 0x3e) {
        return 0;
    }
    crate::comparacao::ramo(ramo);
    1
}

/// the Northeast league has 16 clubs: those with it as their reserve division in the database (the field that holds the
/// state or regional league), in database order; the table only counts when full; no TV money nor prize fund
/// Original: `BraRegNortheast::create_table` (0x436d90, bra_reg_northeast.cpp, thiscall, ret 0).
///
/// [pt] { 0x436d90 } (thiscall, ret 0, bra_reg_northeast.cpp): `BraRegNortheast::create_table()`: o Nordeste tem 16 clubes (divisão reserva +0x60 = [0x9cf9a4]).
/// Fonte 0x98af54, linha 0x159.
pub(crate) unsafe extern "thiscall" fn cqm7_tabela_do_nordeste(this: usize) -> u32 {
    cqm7_tabela(this, 0x10, 0x98af54, 0x159, KEY_CLUB_COMP_BRAZILIAN_NORTHEAST_STATE_CHAMPIONSHIP, 0, "cqm_7 nordeste cheio")
}



/// O comparador dos três: a original aloca a tabela (não roda duas vezes): só ela. O plano, feito antes: os clubes do
/// estadual na ordem do banco (até `n`). Depois: +0x3e = `n`, a devolução = 1 se o plano encheu (senão 0) e as linhas
/// da tabela (+0 o clube, 0x3b bytes) com os clubes do plano, na ordem.
pub(crate) unsafe fn cqm7_comparar(nome: &str, this: usize, n: i16, g: usize, original: impl FnOnce() -> u32) -> u32 {
    let mut plano: Vec<usize> = Vec::new();
    let mut i: i32 = 0;
    while i < read_usize(0xae235c) as i32 && (plano.len() as i16) < n {
        let clube = read_usize(0xae23b4) + (i as usize) * 0x245;
        if cqm7_do_estadual(clube, g) {
            plano.push(clube);
        }
        i += 1;
    }
    let r = original();
    let t = read_usize(this + 0xb1);
    let esperado = (plano.len() as i16 == n) as u32;
    let mut igual = r == esperado && read_i16(this + 0x3e) == n && t != 0;
    let mut achou: Vec<usize> = Vec::new();
    if igual {
        achou = (0..plano.len()).map(|k| read_usize(t + k * 0x3b)).collect();
        igual = achou == plano;
    }
    PLACAR.registrar(igual, || format!("{}({:#x}) original={} plano={:x?} tabela={:x?}", nome, this, r, plano, achou));
    r
}

pub(crate) unsafe extern "thiscall" fn comparar_cqm7_tabela_do_nordeste(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1834);
    cqm7_comparar("cqm7_tabela_do_nordeste", this, 0x10, KEY_CLUB_COMP_BRAZILIAN_NORTHEAST_STATE_CHAMPIONSHIP, || o(this))
}

/// Northeast: after the round robin, the top 4 go to the semi-finals: 1st v 4th, 2nd v 3rd (the better placed is the
/// seed; the ties follow the seeds [inference: two legs, the decider at the better placed club])
/// Original: `BraRegNortheast::create_semi_final_stage` (0x4370d0, bra_reg_northeast.cpp, thiscall, ret 0).
///
/// [pt] { 0x4370d0 } (thiscall, ret 0, bra_reg_northeast.cpp): `BraRegNortheast::create_semi_final_stage()`, da fase seguinte (advance_stage, quando a fase passa de −1 a
/// 0). As semifinais: 1º × 4º, 2º × 3º (linhas 0, 3, 1, 2). Fonte 0x98af54, linhas 0x20d (sem lista) e 0x221 (sem fase).
pub(crate) unsafe extern "thiscall" fn cqn7_mata_mata_do_nordeste(this: usize) {
    cqn7_mata_mata(this, &[0, 3, 1, 2], &[0, 3, 1, 2], 0x98af54, 0x20d, 0x221, "cqn_7 nordeste"); // cqn7: 1º × 4º, 2º × 3º Nordeste
}

pub(crate) unsafe extern "thiscall" fn comparar_cqn7_mata_mata_do_nordeste(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1845);
    cqn7_comparar("cqn7_mata_mata_do_nordeste", this, &[0, 3, 1, 2], || o(this))
}

/// The Northeast turnover: archives the season in the statistics, clears the previous table and stages, moves to the
/// next year and rebuilds the rules, the table with the state clubs, the fixtures (round robin) and the table order;
/// then the predicted positions and the renewal of the state referees
/// Original: `BraRegNortheast::start_new_season` (0x436e90, bra_reg_northeast.cpp, thiscall, ret 0).
///
/// [pt] { 0x436e90 } (thiscall, ret 0, bra_reg_northeast.cpp): `BraRegNortheast::start_new_season()`, a virada da temporada do Nordeste (a tabela:
/// 0x436d90; caixas nas linhas 0x1bc, 0x1c3, 0x1c9 e 0x1d0).
pub(crate) unsafe extern "thiscall" fn cqp8_virada_do_nordeste(this: usize) -> u32 {
    cqp8_virada(this, 0x436d90, 0x98af54, [0x1bc, 0x1c3, 0x1c9, 0x1d0], 1, "cqp_8 virada do nordeste") // cqp8: o ano seguinte do Nordeste
}

pub(crate) unsafe extern "thiscall" fn comparar_cqp8_virada_do_nordeste(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1936);
    cqp8_comparar_virada("cqp8_virada_do_nordeste", this, || o(this))
}

/// The Northeast next stage: the round robin is stage −1; when it ends, the stage becomes 0 and the top 4 go to the
/// semi-finals (1st v 4th, 2nd v 3rd); past the last stage it is an error (the box)
/// Original: `BraRegNortheast::advance_stage` (0x437040, bra_reg_northeast.cpp, thiscall, ret 0).
///
/// [pt] { 0x437040 } (thiscall, ret 0, bra_reg_northeast.cpp): `BraRegNortheast::advance_stage()`, a fase seguinte do Nordeste: no fim dos pontos corridos,
/// o mata-mata de 4 (0x4370d0); a caixa da linha 0x1ea.
pub(crate) unsafe extern "thiscall" fn cqp8_fase_seguinte_do_nordeste(this: usize) {
    cqp8_fase_seguinte(this, 0x98af54, 0x1ea, 0x4370d0);
}

pub(crate) unsafe extern "thiscall" fn comparar_cqp8_fase_seguinte_do_nordeste(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1937);
    cqp8_comparar_fase_seguinte("cqp8_fase_seguinte_do_nordeste", this, || o(this))
}
