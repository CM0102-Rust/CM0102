// ---------- %%LEVA%%a leva: os construtores dos estaduais: Nordeste, Paranaense e Pernambucano (as regras, a tabela, os jogos, as estatísticas e as posições previstas na criação) (bra_reg_northeast.cpp, bra_reg_parana.cpp, bra_reg_pern.cpp) ----------

unsafe fn cqt3_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// Um estadual: a vtable, as fases além da liga (+0x2c: 1 nos de pontos corridos com mata-mata, 3 nos de dois turnos e
/// final; o vetor [+0xc] = malloc de 4 × fases), as regras (thiscall, ret 0), a tabela (thiscall, ret 0), as posições
/// previstas (thiscall, ret 0), o fonte e as linhas das caixas (regras, tabela, jogos, ordem, estatísticas).
struct StateLeague {
    vtabela: usize,
    fases: u32,
    regras: usize,
    tabela: usize,
    previstas: usize,
    arquivo: usize,
    linhas: [u32; 5],
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
unsafe fn bra_reg_northeast_construct_state_league(this: usize, ano: u32, registro: usize, e: &StateLeague, disciplina: u8, previstas_sim: bool, ramo: &'static str) -> usize {
    let base: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x682200usize);
    let carregar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x687b10usize);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let regras: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(e.regras);
    let tabela: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(e.tabela);
    let jogos: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x6835c0usize);
    let ordenar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x6827d0usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let estatisticas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x49ee70usize);
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

/// O comparador dos três: alocam e sorteiam os jogos: só a original, conferindo o que ela grava: a devolução, a
/// vtable, o ano, o registro e a disciplina; na liga nova (+0x44 = 0xff, com as estatísticas), +0x2c, o vetor das
/// fases e a importância +0x51 = 0xf.
unsafe fn bra_reg_northeast_compare_state_league_ctor(nome: &str, this: usize, ano: u32, registro: usize, e: &StateLeague, original: impl FnOnce() -> usize) -> usize {
    let r = original();
    let mut ok = r == this && cqt3_d(this) == e.vtabela && core::ptr::read_unaligned((this + 0x40) as *const u16) == ano as u16
        && cqt3_d(this + 4) == registro && *((this + 0x50) as *const u8) == 7;
    if *((this + 0x44) as *const u8) == 0xff && cqt3_d(this + 8) != 0 {
        ok = ok && cqt3_d(this + 0x2c) == e.fases as usize && cqt3_d(this + 0xc) != 0 && *((this + 0x51) as *const u8) == 0xf;
    }
    PLACAR.registrar(ok, || format!("{}({:#x}): devolveu {:#x}, vtable {:#x}, +0x2c = {}, +0xc = {:#x}, +8 = {:#x}", nome, this, r, cqt3_d(this), cqt3_d(this + 0x2c), cqt3_d(this + 0xc), cqt3_d(this + 8)));
    r
}

const NORDESTE_STATE_LEAGUE: StateLeague = StateLeague {
    vtabela: 0x9683ec,
    fases: 1,
    regras: 0x4322d0,
    tabela: 0x436d90,
    previstas: 0x437290,
    arquivo: 0x98af54,
    linhas: [0x4d, 0x54, 0x5b, 0x64, 0x6b],
};

const PARANA_CHAMPIONSHIP: StateLeague = StateLeague {
    vtabela: 0x9684a0,
    fases: 3,
    regras: 0x437fc0,
    tabela: 0x437ec0,
    previstas: 0x438590,
    arquivo: 0x98af98,
    linhas: [0x4e, 0x55, 0x5b, 0x62, 0x69],
};

const PERNAMBUCANO_STATE_CHAMPIONSHIP: StateLeague = StateLeague {
    vtabela: 0x968554,
    fases: 1,
    regras: 0x439430,
    tabela: 0x439330,
    previstas: 0x4398d0,
    arquivo: 0x98afd8,
    linhas: [0x4d, 0x54, 0x5b, 0x64, 0x6b],
};

/// { 0x4363f0 } (thiscall, ret 8, bra_reg_northeast.cpp): `BraRegNortheast::BraRegNortheast(ano, registro)`: o Nordeste (uma fase além da liga).
unsafe extern "thiscall" fn construct_bra_reg_northeast(this: usize, ano: u32, registro: usize) -> usize {
    bra_reg_northeast_construct_state_league(this, ano, registro, &NORDESTE_STATE_LEAGUE, 7, true, "cqt_3 nordeste novo") // cqt3: a disciplina e as posições previstas do Nordeste
}

/// { 0x437400 } (thiscall, ret 8, bra_reg_parana.cpp): `BraRegParana::BraRegParana(ano, registro)`: o Paranaense (três fases além da liga).
unsafe extern "thiscall" fn construct_bra_reg_parana(this: usize, ano: u32, registro: usize) -> usize {
    bra_reg_northeast_construct_state_league(this, ano, registro, &PARANA_CHAMPIONSHIP, 7, true, "cqt_3 paranaense novo") // cqt3: a disciplina e as posições previstas do Paranaense
}

/// { 0x4388f0 } (thiscall, ret 8, bra_reg_pern.cpp): `BraRegPern::BraRegPern(ano, registro)`: o Pernambucano (uma fase além da liga).
unsafe extern "thiscall" fn construct_bra_reg_pern(this: usize, ano: u32, registro: usize) -> usize {
    bra_reg_northeast_construct_state_league(this, ano, registro, &PERNAMBUCANO_STATE_CHAMPIONSHIP, 7, true, "cqt_3 pernambucano novo") // cqt3: a disciplina e as posições previstas do Pernambucano
}

unsafe extern "thiscall" fn comparar_construct_bra_reg_northeast(this: usize, ano: u32, registro: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
    bra_reg_northeast_compare_state_league_ctor("construct_bra_reg_northeast", this, ano, registro, &NORDESTE_STATE_LEAGUE, || o(this, ano, registro))
}

unsafe extern "thiscall" fn comparar_construct_bra_reg_parana(this: usize, ano: u32, registro: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I1%%);
    bra_reg_northeast_compare_state_league_ctor("construct_bra_reg_parana", this, ano, registro, &PARANA_CHAMPIONSHIP, || o(this, ano, registro))
}

unsafe extern "thiscall" fn comparar_construct_bra_reg_pern(this: usize, ano: u32, registro: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I2%%);
    bra_reg_northeast_compare_state_league_ctor("construct_bra_reg_pern", this, ano, registro, &PERNAMBUCANO_STATE_CHAMPIONSHIP, || o(this, ano, registro))
}
