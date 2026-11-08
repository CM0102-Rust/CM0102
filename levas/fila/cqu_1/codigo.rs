// ---------- %%LEVA%%a leva: os construtores das ligas nacionais de fora da Europa: as Séries A e B, a J1 e a NSL (as regras, a tabela, os jogos, as estatísticas e as posições previstas na criação) (bra_nat_first.cpp, bra_nat_second.cpp, jap_j1.cpp, aus_nsl.cpp) ----------

unsafe fn cqu1_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// Uma liga nacional: a vtable, a regra de disciplina (+0x50), a importância (+0x51; `importancia_antes`: gravada antes
/// de carregar o estado salvo, em toda criação, como na NSL; senão só na liga nova), as fases além da liga (+0x2c; o
/// vetor [+0xc] = malloc de 4 × fases), as regras (thiscall, ret 0), a tabela (thiscall, ret 0), o fonte e as linhas
/// das caixas (regras, tabela, jogos, ordem, estatísticas). As posições previstas são as da League (0x68a850).
struct Cqu1Liga {
    vtabela: usize,
    importancia: u8,
    importancia_antes: bool,
    fases: u32,
    regras: usize,
    tabela: usize,
    arquivo: usize,
    linhas: [u32; 5],
}

/// O construtor de uma liga nacional (o mesmo molde de 544 bytes dos estaduais, conferido instrução a instrução: só
/// mudam a vtable, a disciplina, a importância e quando ela é gravada, as fases, as regras, a tabela, o fonte e as
/// linhas; as posições previstas são as da League). League::League (0x682200, thiscall, ret 0); +4 = o registro da
/// competição; a vtable; o ano (+0x40, word) = `ano`; a regra de disciplina (+0x50) = `disciplina` (e, na NSL, a
/// importância +0x51). Se o estado salvo carrega (League::load_from_temp_file 0x687b10, thiscall(this, 1), ret 4;
/// devolve ≠ 0), devolve this. Senão (a carreira nova): +0x51 = a importância (fora da NSL), +0x44 = 0xff, +0x30 = −1,
/// +0x2c = as fases e o vetor delas (malloc 0x944e46, sem conferir); então, cada um com a sua caixa (e devolvendo this)
/// se devolver 0: as regras, a tabela, os jogos (League::create_fixtures 0x6835c0, thiscall, ret 0) e a ordem
/// (League::sort_table 0x6827d0, thiscall(this, 0), ret 4); as estatísticas (operator new 0x944cf1 de 0x5ce e
/// Comp_Stats::Comp_Stats 0x49ee70, thiscall(novo, this), ret 4; o quadro SEH da original cobre só esse new) em +8 (sem
/// elas: a caixa); por fim as posições previstas (League::set_predicted_positions 0x68a850, thiscall, ret 0).
unsafe fn cqu1_construir(this: usize, ano: u32, registro: usize, e: &Cqu1Liga, disciplina: u8, ramo: &'static str) -> usize {
    let base: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x682200usize);
    let carregar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x687b10usize);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let regras: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(e.regras);
    let tabela: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(e.tabela);
    let jogos: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x6835c0usize);
    let ordenar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x6827d0usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let estatisticas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x49ee70usize);
    let previstas: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68a850usize);
    base(this);
    core::ptr::write_unaligned((this + 4) as *mut usize, registro);
    core::ptr::write_unaligned(this as *mut usize, e.vtabela);
    core::ptr::write_unaligned((this + 0x40) as *mut u16, ano as u16);
    *((this + 0x50) as *mut u8) = disciplina;
    if e.importancia_antes {
        *((this + 0x51) as *mut u8) = e.importancia;
    }
    if carregar(this, 1) != 0 {
        return this;
    }
    crate::comparison::branch(ramo);
    if !e.importancia_antes {
        *((this + 0x51) as *mut u8) = e.importancia;
    }
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
    previstas(this);
    this
}

/// O comparador dos quatro: alocam e sorteiam os jogos: só a original, conferindo o que ela grava: a devolução, a
/// vtable, o ano, o registro e a disciplina; a importância quando ela é gravada sempre (a NSL); na liga nova (+0x44 =
/// 0xff, com as estatísticas), +0x2c, o vetor das fases e a importância.
unsafe fn cqu1_comparar(nome: &str, this: usize, ano: u32, registro: usize, e: &Cqu1Liga, disciplina: u8, original: impl FnOnce() -> usize) -> usize {
    let r = original();
    let mut ok = r == this && cqu1_d(this) == e.vtabela && core::ptr::read_unaligned((this + 0x40) as *const u16) == ano as u16
        && cqu1_d(this + 4) == registro && *((this + 0x50) as *const u8) == disciplina;
    if e.importancia_antes {
        ok = ok && *((this + 0x51) as *const u8) == e.importancia;
    }
    if *((this + 0x44) as *const u8) == 0xff && cqu1_d(this + 8) != 0 {
        ok = ok && cqu1_d(this + 0x2c) == e.fases as usize && cqu1_d(this + 0xc) != 0 && *((this + 0x51) as *const u8) == e.importancia;
    }
    PLACAR.record(ok, || format!("{}({:#x}): devolveu {:#x}, vtable {:#x}, +0x2c = {}, +0xc = {:#x}, +8 = {:#x}, +0x51 = {}", nome, this, r, cqu1_d(this), cqu1_d(this + 0x2c), cqu1_d(this + 0xc), cqu1_d(this + 8), *((this + 0x51) as *const u8)));
    r
}

const CQU1_SERIE_A: Cqu1Liga = Cqu1Liga {
    vtabela: 0x967d98,
    importancia: 0xa,
    importancia_antes: false,
    fases: 1,
    regras: 0x42a5a0,
    tabela: 0x42a480,
    arquivo: 0x98aa30,
    linhas: [0x56, 0x5d, 0x64, 0x6d, 0x74],
};

const CQU1_SERIE_B: Cqu1Liga = Cqu1Liga {
    vtabela: 0x967e4c,
    importancia: 0xa,
    importancia_antes: false,
    fases: 1,
    regras: 0x42cb70,
    tabela: 0x42ca70,
    arquivo: 0x98ab64,
    linhas: [0x50, 0x57, 0x5e, 0x67, 0x6e],
};

const CQU1_J1: Cqu1Liga = Cqu1Liga {
    vtabela: 0x96ca40,
    importancia: 0xf,
    importancia_antes: false,
    fases: 3,
    regras: 0x663620,
    tabela: 0x6634e0,
    arquivo: 0x9eabc8,
    linhas: [0x4b, 0x52, 0x59, 0x62, 0x69],
};

const CQU1_NSL: Cqu1Liga = Cqu1Liga {
    vtabela: 0x967768,
    importancia: 0xa,
    importancia_antes: true,
    fases: 2,
    regras: 0x411ce0,
    tabela: 0x411b70,
    arquivo: 0x987c20,
    linhas: [0x46, 0x4d, 0x53, 0x5a, 0x61],
};

/// { 0x429850 } (thiscall, ret 8, bra_nat_first.cpp): `BraNatFirst::BraNatFirst(ano, registro)`: a Série A (uma fase além da liga).
unsafe extern "thiscall" fn cqu1_construir_serie_a(this: usize, ano: u32, registro: usize) -> usize {
    cqu1_construir(this, ano, registro, &CQU1_SERIE_A, 6, "cqu_1 serie_a nova") // cqu1: a Série A
}

/// { 0x42be40 } (thiscall, ret 8, bra_nat_second.cpp): `BraNatSecond::BraNatSecond(ano, registro)`: a Série B (uma fase além da liga).
unsafe extern "thiscall" fn cqu1_construir_serie_b(this: usize, ano: u32, registro: usize) -> usize {
    cqu1_construir(this, ano, registro, &CQU1_SERIE_B, 6, "cqu_1 serie_b nova") // cqu1: a disciplina da Série B
}

/// { 0x662b30 } (thiscall, ret 8, jap_j1.cpp): `JapJ1::JapJ1(ano, registro)`: a J1 (três fases além da liga: os dois turnos e a final).
unsafe extern "thiscall" fn cqu1_construir_j1(this: usize, ano: u32, registro: usize) -> usize {
    cqu1_construir(this, ano, registro, &CQU1_J1, 0x13, "cqu_1 j1 nova") // cqu1: a J1
}

/// { 0x410e90 } (thiscall, ret 8, aus_nsl.cpp): `AusNsl::AusNsl(ano, registro)`: a NSL (duas fases além da liga: a série final e a grande final).
unsafe extern "thiscall" fn cqu1_construir_nsl(this: usize, ano: u32, registro: usize) -> usize {
    cqu1_construir(this, ano, registro, &CQU1_NSL, 0x22, "cqu_1 nsl nova") // cqu1: a disciplina da NSL
}

unsafe extern "thiscall" fn comparar_cqu1_construir_serie_a(this: usize, ano: u32, registro: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
    cqu1_comparar("cqu1_construir_serie_a", this, ano, registro, &CQU1_SERIE_A, 6, || o(this, ano, registro))
}

unsafe extern "thiscall" fn comparar_cqu1_construir_serie_b(this: usize, ano: u32, registro: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I1%%);
    cqu1_comparar("cqu1_construir_serie_b", this, ano, registro, &CQU1_SERIE_B, 6, || o(this, ano, registro))
}

unsafe extern "thiscall" fn comparar_cqu1_construir_j1(this: usize, ano: u32, registro: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I2%%);
    cqu1_comparar("cqu1_construir_j1", this, ano, registro, &CQU1_J1, 0x13, || o(this, ano, registro))
}

unsafe extern "thiscall" fn comparar_cqu1_construir_nsl(this: usize, ano: u32, registro: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I3%%);
    cqu1_comparar("cqu1_construir_nsl", this, ano, registro, &CQU1_NSL, 0x22, || o(this, ano, registro))
}
