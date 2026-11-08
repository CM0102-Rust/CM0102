// ---------- %%LEVA%%a leva: os construtores das quatro divisões inglesas (a Premier, a First, a Second e a Third Division: as regras, a tabela, os jogos, as estatísticas e as posições previstas na criação) (eng_prm.cpp, eng_first.cpp, eng_second.cpp, eng_third.cpp) ----------

unsafe fn cql6_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cql6_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

/// O que muda de uma divisão inglesa para outra no construtor.
struct EnglishDivision {
    /// a vtable da classe derivada
    vtabela: usize,
    /// a capacidade mínima do estádio (+0xe2) e os lugares sentados mínimos (+0xe4), quando a divisão os grava
    estadio: Option<(u16, u16)>,
    /// quantas fases além da liga (+0x2c): 1 = o play-off de acesso (o vetor [+0xc] de 1 fase, malloc de 4)
    fases: u32,
    /// set_up_rules e create_table da divisão (mov ecx, this -> thiscall, ret 0; devolvem 0 na falha)
    regras: usize,
    tabela: usize,
    /// o fonte e as linhas das cinco caixas (regras, tabela, jogos, ordem, estatísticas)
    arquivo: usize,
    linhas: [u32; 5],
}

/// O corpo dos quatro construtores (thiscall(this, ano, registro), ret 8, quadro SEH só para o operator new): a base
/// (League::League 0x682200: mov ecx -> thiscall, ret 0), o ano (+0x40, palavra), o registro da competição (+4), a vtable,
/// a regra de disciplina 9 (+0x50) e o estádio mínimo; tenta carregar o estado do arquivo temporário
/// (League::load_from_temp_file 0x687b10: push 1; mov ecx -> thiscall, ret 4). Carregou: pronto. Senão, a liga nova:
/// fase 0xff (+0x44), fase atual −1 (+0x30), +0x2c fases (e o vetor [+0xc]), as regras, a tabela, os jogos
/// (League::create_fixtures 0x6835c0: mov ecx -> thiscall, ret 0), a ordem da tabela (League::sort_table 0x6827d0:
/// push 0; mov ecx -> thiscall, ret 4), as estatísticas (operator new 0x944cf1 de 0x5ce: push; add esp, 4 -> cdecl;
/// Comp_Stats::Comp_Stats 0x49ee70: push liga; mov ecx, novo -> thiscall, ret 4) em +8 e as posições previstas
/// (League::set_predicted_positions 0x68a850: mov ecx -> thiscall, ret 0). Cada passo que falha mostra a caixa da
/// sua linha e encerra. Devolve this.
unsafe fn construct_english_division(this: usize, ano: u32, registro: usize, p: &EnglishDivision) -> usize {
    let base: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x682200usize);
    let carregar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x687b10usize);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let regras: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(p.regras);
    let tabela: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(p.tabela);
    let jogos: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x6835c0usize);
    let ordenar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x6827d0usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let estatisticas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x49ee70usize);
    let previstas: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68a850usize);
    base(this);
    cql6_pd(this + 4, registro);
    cql6_pd(this, p.vtabela);
    core::ptr::write_unaligned((this + 0x40) as *mut u16, ano as u16);
    *((this + 0x50) as *mut u8) = 9; // cql6: a regra de disciplina das ligas inglesas
    if let Some((capacidade, sentados)) = p.estadio {
        core::ptr::write_unaligned((this + 0xe2) as *mut u16, capacidade);
        core::ptr::write_unaligned((this + 0xe4) as *mut u16, sentados);
    }
    if carregar(this, 1) != 0 {
        return this;
    }
    crate::comparison::branch("cql_6 divisao inglesa nova");
    *((this + 0x44) as *mut u8) = 0xff;
    cql6_pd(this + 0x30, 0xffff_ffff);
    cql6_pd(this + 0x2c, p.fases as usize);
    if p.fases != 0 {
        let v = alocar(4);
        cql6_pd(this + 0xc, v);
    } else {
        cql6_pd(this + 0xc, 0);
    }
    if regras(this) == 0 {
        show_error_at(p.arquivo, p.linhas[0]);
        return this;
    }
    if tabela(this) == 0 {
        show_error_at(p.arquivo, p.linhas[1]);
        return this;
    }
    if jogos(this) == 0 {
        show_error_at(p.arquivo, p.linhas[2]);
        return this;
    }
    if ordenar(this, 0) == 0 {
        show_error_at(p.arquivo, p.linhas[3]);
        return this;
    }
    let e = novo(0x5ce);
    let s = if e != 0 { estatisticas(e, this) } else { 0 };
    cql6_pd(this + 8, s);
    if s == 0 {
        show_error_at(p.arquivo, p.linhas[4]);
        return this;
    }
    previstas(this);
    this
}

const PREMIER_DIVISION: EnglishDivision = EnglishDivision {
    vtabela: 0x969d1c,
    estadio: Some((0x2710, 0x2710)),
    fases: 0,
    regras: 0x574b70,
    tabela: 0x574a20,
    arquivo: 0x9c43b0,
    linhas: [0x4d, 0x54, 0x5b, 0x64, 0x6b],
};
const FIRST_DIVISION: EnglishDivision = EnglishDivision {
    vtabela: 0x969c68,
    estadio: None,
    fases: 1, // cql6: o play-off da First Division
    regras: 0x572d90,
    tabela: 0x572bf0,
    arquivo: 0x9c4374,
    linhas: [0x49, 0x50, 0x57, 0x60, 0x68],
};
const ENG_SECOND_DIVISION: EnglishDivision = EnglishDivision {
    vtabela: 0x969dd0,
    estadio: None,
    fases: 1, // cql6: o play-off da Second Division
    regras: 0x576780,
    tabela: 0x576630,
    arquivo: 0x9c43e8,
    linhas: [0x4a, 0x51, 0x58, 0x61, 0x68],
};
const THIRD_DIVISION: EnglishDivision = EnglishDivision {
    vtabela: 0x969e84,
    estadio: Some((0x1770, 0x3e8)),
    fases: 1,
    regras: 0x5780c0,
    tabela: 0x577f70,
    arquivo: 0x9c4424,
    linhas: [0x4e, 0x55, 0x5c, 0x65, 0x6d],
};

/// { 0x573380 } (thiscall, ret 8, eng_prm.cpp): `EngPrm::EngPrm(ano, registro)`. A Premier: sem fases além da liga
/// (+0x2c = 0, +0xc = 0) e o estádio mínimo de 10.000 lugares, todos sentados.
unsafe extern "thiscall" fn construct_eng_prm(this: usize, ano: u32, registro: usize) -> usize {
    construct_english_division(this, ano, registro, &PREMIER_DIVISION)
}

/// { 0x571680 } (thiscall, ret 8, eng_first.cpp): `EngFirst::EngFirst(ano, registro)`. A First Division: 1 fase além da
/// liga (o play-off de acesso), sem estádio mínimo.
unsafe extern "thiscall" fn construct_eng_first(this: usize, ano: u32, registro: usize) -> usize {
    construct_english_division(this, ano, registro, &FIRST_DIVISION)
}

/// { 0x5754a0 } (thiscall, ret 8, eng_second.cpp): `EngSecond::EngSecond(ano, registro)`. A Second Division: 1 fase além
/// da liga (o play-off), sem estádio mínimo.
unsafe extern "thiscall" fn construct_eng_second(this: usize, ano: u32, registro: usize) -> usize {
    construct_english_division(this, ano, registro, &ENG_SECOND_DIVISION)
}

/// { 0x576dd0 } (thiscall, ret 8, eng_third.cpp): `EngThird::EngThird(ano, registro)`. A Third Division: 1 fase além da
/// liga (o play-off) e o estádio mínimo de 6.000 lugares, 1.000 sentados.
unsafe extern "thiscall" fn construct_eng_third(this: usize, ano: u32, registro: usize) -> usize {
    construct_english_division(this, ano, registro, &THIRD_DIVISION)
}

/// O comparador dos quatro: alocam (as estatísticas, a tabela, os jogos): só a original, conferindo o que ela grava:
/// a devolução, a vtable, o ano, o registro e a regra de disciplina; na liga nova (+0x44 = 0xff), +0x2c, o vetor das
/// fases, a fase atual −1 e as estatísticas em +8.
unsafe fn compare_english_constructor(nome: &str, this: usize, ano: u32, registro: usize, p: &EnglishDivision, original: impl FnOnce() -> usize) -> usize {
    let r = original();
    let mut ok = r == this && cql6_d(this) == p.vtabela && core::ptr::read_unaligned((this + 0x40) as *const u16) == ano as u16
        && cql6_d(this + 4) == registro && *((this + 0x50) as *const u8) == 9;
    if let Some((c, s)) = p.estadio {
        ok = ok && core::ptr::read_unaligned((this + 0xe2) as *const u16) == c && core::ptr::read_unaligned((this + 0xe4) as *const u16) == s;
    }
    if *((this + 0x44) as *const u8) == 0xff && cql6_d(this + 8) != 0 {
        ok = ok && cql6_d(this + 0x2c) == p.fases as usize && (cql6_d(this + 0xc) != 0) == (p.fases != 0);
    }
    PLACAR.record(ok, || format!("{}({:#x}): devolveu {:#x}, vtable {:#x}, +0x2c = {}, +0xc = {:#x}, +8 = {:#x}", nome, this, r, cql6_d(this), cql6_d(this + 0x2c), cql6_d(this + 0xc), cql6_d(this + 8)));
    r
}

unsafe extern "thiscall" fn comparar_construct_eng_prm(this: usize, ano: u32, registro: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
    compare_english_constructor("construct_eng_prm", this, ano, registro, &PREMIER_DIVISION, || o(this, ano, registro))
}

unsafe extern "thiscall" fn comparar_construct_eng_first(this: usize, ano: u32, registro: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I1%%);
    compare_english_constructor("construct_eng_first", this, ano, registro, &FIRST_DIVISION, || o(this, ano, registro))
}

unsafe extern "thiscall" fn comparar_construct_eng_second(this: usize, ano: u32, registro: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I2%%);
    compare_english_constructor("construct_eng_second", this, ano, registro, &ENG_SECOND_DIVISION, || o(this, ano, registro))
}

unsafe extern "thiscall" fn comparar_construct_eng_third(this: usize, ano: u32, registro: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I3%%);
    compare_english_constructor("construct_eng_third", this, ano, registro, &THIRD_DIVISION, || o(this, ano, registro))
}
