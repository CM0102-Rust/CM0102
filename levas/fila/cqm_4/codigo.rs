// ---------- %%LEVA%%a leva: os construtores da Série A, da Série B e dos dois grupos da Série C1 italianas (as regras, a tabela, o calendário real de 2001/02, os jogos, as estatísticas e as posições previstas na criação) (ita_ser_a.cpp, ita_ser_b.cpp, ita_ser_c1a.cpp, ita_ser_c1b.cpp) ----------

unsafe fn cqm4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqm4_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

/// O que muda de uma divisão italiana para outra no construtor.
struct ItalianDivision {
    vtabela: usize,
    /// a Série A e a Série B gravam +0xee = 0 (o campo próprio da classe [inferência: o desempate em curso]) e +0x4c = 1
    serie: bool,
    /// fases além da liga (+0x2c): 0 na A e na B; 2 na C1 (o play-off e o play-out; o vetor [+0xc] = malloc de 8)
    fases: u32,
    regras: usize,
    tabela: usize,
    /// o calendário real de 2001/02 pelo nome dos clubes (set_up_fixed_fixtures; mov ecx, this -> thiscall, ret 0)
    fixos: usize,
    arquivo: usize,
    /// as caixas: regras, tabela, jogos, ordem, estatísticas
    linhas: [u32; 5],
}

/// O corpo dos quatro construtores (thiscall(this, ano, registro), ret 8, quadro SEH só para o operator new): a base
/// (League::League 0x682200: mov ecx -> thiscall, ret 0), o registro (+4), a vtable, o ano (+0x40) e a regra de
/// disciplina 0x11 (+0x50, a das ligas italianas); tenta carregar o estado (League::load_from_temp_file 0x687b10: push
/// 1; mov ecx -> thiscall, ret 4). Carregou: pronto. Senão, a liga nova: na A e na B, +0xee = 0 e +0x4c = 1; fase 0xff
/// (+0x44), fase atual −1 (+0x30), +0x2c fases e o vetor [+0xc] (malloc 0x944e46 de 4 × fases: push; add esp, 4 ->
/// cdecl; 0 sem fases); as regras e a tabela (caixa e fim se falharem); o calendário real de 2001/02 (sem conferir);
/// os jogos (League::create_fixtures 0x6835c0), a ordem (League::sort_table 0x6827d0: push 0 -> thiscall, ret 4), as
/// estatísticas (operator new 0x944cf1 de 0x5ce: push; add esp, 4 -> cdecl; Comp_Stats::Comp_Stats 0x49ee70: push
/// liga; mov ecx, novo -> thiscall, ret 4) em +8 e as posições previstas (0x68a850: mov ecx -> thiscall, ret 0).
/// Devolve this.
unsafe fn construct_italian_division(this: usize, ano: u32, registro: usize, p: &ItalianDivision) -> usize {
    let base: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x682200usize);
    let carregar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x687b10usize);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let regras: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(p.regras);
    let tabela: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(p.tabela);
    let fixos: unsafe extern "thiscall" fn(usize) = core::mem::transmute(p.fixos);
    let jogos: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x6835c0usize);
    let ordenar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x6827d0usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let estatisticas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x49ee70usize);
    let previstas: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68a850usize);
    base(this);
    cqm4_pd(this + 4, registro);
    cqm4_pd(this, p.vtabela);
    core::ptr::write_unaligned((this + 0x40) as *mut u16, ano as u16);
    *((this + 0x50) as *mut u8) = 0x11; // cqm4: a regra de disciplina das ligas italianas
    if carregar(this, 1) != 0 {
        return this;
    }
    crate::comparison::branch("cqm_4 divisao italiana nova");
    if p.serie {
        *((this + 0xee) as *mut u8) = 0;
        cqm4_pd(this + 0x4c, 1); // cqm4: a marca do histórico da A e da B
    }
    *((this + 0x44) as *mut u8) = 0xff;
    cqm4_pd(this + 0x30, 0xffff_ffff);
    cqm4_pd(this + 0x2c, p.fases as usize);
    if p.fases != 0 {
        let v = alocar(p.fases * 4);
        cqm4_pd(this + 0xc, v);
    } else {
        cqm4_pd(this + 0xc, 0);
    }
    if regras(this) == 0 {
        show_error_at(p.arquivo, p.linhas[0]);
        return this;
    }
    if tabela(this) == 0 {
        show_error_at(p.arquivo, p.linhas[1]);
        return this;
    }
    fixos(this); // cqm4: o calendário real de 2001/02
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
    cqm4_pd(this + 8, s);
    if s == 0 {
        show_error_at(p.arquivo, p.linhas[4]);
        return this;
    }
    previstas(this); // cqm4: as posições previstas
    this
}

const SERIE_A: ItalianDivision = ItalianDivision {
    vtabela: 0x96c3cc,
    serie: true,
    fases: 0,
    regras: 0x641640,
    tabela: 0x641510,
    fixos: 0x642a70,
    arquivo: 0x9ea2a8,
    linhas: [0x53, 0x5a, 0x64, 0x6d, 0x74],
};
const SERIE_B_DIVISION: ItalianDivision = ItalianDivision {
    vtabela: 0x96c480,
    serie: true,
    fases: 0,
    regras: 0x6468a0,
    tabela: 0x646780,
    fixos: 0x647230,
    arquivo: 0x9ea384,
    linhas: [0x4b, 0x52, 0x5c, 0x65, 0x6c],
};
const SERIE_C1A_DIVISION: ItalianDivision = ItalianDivision {
    vtabela: 0x96c534,
    serie: false,
    fases: 2, // cqm4: o play-off e o play-out da C1/A
    regras: 0x64b770,
    tabela: 0x64b670,
    fixos: 0x64be90,
    arquivo: 0x9ea474,
    linhas: [0x49, 0x50, 0x5a, 0x63, 0x6a],
};
const SERIE_C1B_DIVISION: ItalianDivision = ItalianDivision {
    vtabela: 0x96c5e8,
    serie: false,
    fases: 2,
    regras: 0x64f920,
    tabela: 0x64f820,
    fixos: 0x650100,
    arquivo: 0x9ea558,
    linhas: [0x4a, 0x51, 0x5b, 0x64, 0x6b],
};

/// { 0x6406d0 } (thiscall, ret 8, ita_ser_a.cpp): `ItaSerA::ItaSerA(ano, registro)`: a Série A, só a liga (o jogo de
/// desempate nasce no fim, pela 0x642880).
unsafe extern "thiscall" fn construct_ita_ser_a(this: usize, ano: u32, registro: usize) -> usize {
    construct_italian_division(this, ano, registro, &SERIE_A)
}

/// { 0x645810 } (thiscall, ret 8, ita_ser_b.cpp): `ItaSerB::ItaSerB(ano, registro)`: a Série B, só a liga.
unsafe extern "thiscall" fn construct_ita_ser_b(this: usize, ano: u32, registro: usize) -> usize {
    construct_italian_division(this, ano, registro, &SERIE_B_DIVISION)
}

/// { 0x64aa70 } (thiscall, ret 8, ita_ser_c1a.cpp): `ItaSerC1a::ItaSerC1a(ano, registro)`: a C1/A, a liga e 2 fases
/// (o play-off de acesso e o play-out).
unsafe extern "thiscall" fn construct_ita_ser_c1a(this: usize, ano: u32, registro: usize) -> usize {
    construct_italian_division(this, ano, registro, &SERIE_C1A_DIVISION)
}

/// { 0x64ec20 } (thiscall, ret 8, ita_ser_c1b.cpp): `ItaSerC1b::ItaSerC1b(ano, registro)`: a C1/B, como a C1/A.
unsafe extern "thiscall" fn construct_ita_ser_c1b(this: usize, ano: u32, registro: usize) -> usize {
    construct_italian_division(this, ano, registro, &SERIE_C1B_DIVISION)
}

/// O comparador dos quatro: alocam: só a original, conferindo o que ela grava: a devolução, a vtable, o ano, o
/// registro e a disciplina; na liga nova (+0x44 = 0xff, com as estatísticas), +0x2c, o vetor das fases e, na A e na
/// B, +0x4c = 1 e +0xee = 0.
unsafe fn compare_italian_constructor(nome: &str, this: usize, ano: u32, registro: usize, p: &ItalianDivision, original: impl FnOnce() -> usize) -> usize {
    let r = original();
    let mut ok = r == this && cqm4_d(this) == p.vtabela && core::ptr::read_unaligned((this + 0x40) as *const u16) == ano as u16
        && cqm4_d(this + 4) == registro && *((this + 0x50) as *const u8) == 0x11;
    if *((this + 0x44) as *const u8) == 0xff && cqm4_d(this + 8) != 0 {
        ok = ok && cqm4_d(this + 0x2c) == p.fases as usize && (cqm4_d(this + 0xc) != 0) == (p.fases != 0);
        if p.serie {
            ok = ok && *((this + 0xee) as *const u8) == 0;
        }
    }
    SCOREBOARD.record(ok, || format!("{}({:#x}): devolveu {:#x}, vtable {:#x}, +0x2c = {}, +0xc = {:#x}, +8 = {:#x}", nome, this, r, cqm4_d(this), cqm4_d(this + 0x2c), cqm4_d(this + 0xc), cqm4_d(this + 8)));
    r
}

unsafe extern "thiscall" fn compare_construct_ita_ser_a(this: usize, ano: u32, registro: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
    compare_italian_constructor("construct_ita_ser_a", this, ano, registro, &SERIE_A, || o(this, ano, registro))
}

unsafe extern "thiscall" fn compare_construct_ita_ser_b(this: usize, ano: u32, registro: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I1%%);
    compare_italian_constructor("construct_ita_ser_b", this, ano, registro, &SERIE_B_DIVISION, || o(this, ano, registro))
}

unsafe extern "thiscall" fn compare_construct_ita_ser_c1a(this: usize, ano: u32, registro: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I2%%);
    compare_italian_constructor("construct_ita_ser_c1a", this, ano, registro, &SERIE_C1A_DIVISION, || o(this, ano, registro))
}

unsafe extern "thiscall" fn compare_construct_ita_ser_c1b(this: usize, ano: u32, registro: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I3%%);
    compare_italian_constructor("construct_ita_ser_c1b", this, ano, registro, &SERIE_C1B_DIVISION, || o(this, ano, registro))
}
