// ---------- %%LEVA%%a leva: os construtores da Série C, da J2 e da MLS (as regras, a tabela, os jogos, as estatísticas, os grupos e as posições previstas na criação) (bra_nat_third.cpp, jap_j2.cpp, usa_mls.cpp) ----------

unsafe fn cqu2_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// Uma liga do molde dos construtores (o dos estaduais e das ligas nacionais), com as variações destas três: a
/// importância (+0x51; `Some((valor, antes))`: gravada antes de carregar o estado salvo, como na MLS, ou só na liga
/// nova; `None`: não grava), as fases além da liga (+0x2c; 0: nem vetor, +0xc = 0, como na J2; senão o vetor [+0xc] =
/// malloc de 4 × fases), as regras gravadas em linha (deslocamento, tamanho 1/2/4, valor; a MLS), a rotina das regras
/// (thiscall, ret 0; `None` na MLS), a tabela e os jogos (thiscall, ret 0), as posições previstas (thiscall, ret 0), a
/// Série C (`serie_c`: +0xee zerado no começo e, no fim, os sete grupos, as posições previstas e a lista dos 56
/// liberada), o fonte e as linhas das caixas (as regras, se houver, a tabela, os jogos, a ordem, as estatísticas).
struct Cqu2Liga {
    vtabela: usize,
    importancia: Option<(u8, bool)>,
    fases: u32,
    escritas: &'static [(usize, u8, u32)],
    regras: Option<usize>,
    tabela: usize,
    jogos: usize,
    previstas: usize,
    serie_c: bool,
    arquivo: usize,
    linhas: &'static [u32],
}

/// O construtor (o molde de 544 bytes dos estaduais, com as variações de `Cqu2Liga`). League::League (0x682200,
/// thiscall, ret 0); +4 = o registro; a vtable; na Série C, +0xee = 0; o ano (+0x40, word); a disciplina (+0x50); na
/// MLS, a importância. Se o estado salvo carrega (0x687b10, thiscall(this, 1), ret 4; ≠ 0), devolve this. Senão: a
/// importância (se só na liga nova), +0x44 = 0xff, +0x30 = −1, +0x2c = as fases e o vetor delas (malloc 0x944e46, sem
/// conferir; sem fases: +0xc = 0), as regras em linha; então, cada um com a sua caixa (e devolvendo this) se devolver
/// 0: as regras, a tabela, os jogos e a ordem (League::sort_table 0x6827d0, thiscall(this, 0), ret 4); as estatísticas
/// (operator new 0x944cf1 de 0x5ce e Comp_Stats::Comp_Stats 0x49ee70, thiscall(novo, this), ret 4) em +8 (sem elas: a
/// caixa). Na Série C: os sete grupos da primeira fase (0x42ee80(g), thiscall, ret 4, g de 0 a 6; a original empurra o
/// dword com g no byte de baixo e, em cima, os bytes altos do ponteiro do new; a rotina só usa o g: a cópia passa g
/// limpo), as posições previstas e a lista dos 56 (+0xee) liberada (free 0x9452ca, se houver) e zerada. Nas outras, as
/// posições previstas.
unsafe fn cqu2_construir(this: usize, ano: u32, registro: usize, e: &Cqu2Liga, disciplina: u8, ramo: &'static str) -> usize {
    let base: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x682200usize);
    let carregar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x687b10usize);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let tabela: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(e.tabela);
    let jogos: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(e.jogos);
    let ordenar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x6827d0usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let estatisticas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x49ee70usize);
    let previstas: unsafe extern "thiscall" fn(usize) = core::mem::transmute(e.previstas);
    base(this);
    core::ptr::write_unaligned((this + 4) as *mut usize, registro);
    core::ptr::write_unaligned(this as *mut usize, e.vtabela);
    if e.serie_c {
        core::ptr::write_unaligned((this + 0xee) as *mut usize, 0);
    }
    core::ptr::write_unaligned((this + 0x40) as *mut u16, ano as u16);
    *((this + 0x50) as *mut u8) = disciplina;
    if let Some((v, true)) = e.importancia {
        *((this + 0x51) as *mut u8) = v;
    }
    if carregar(this, 1) != 0 {
        return this;
    }
    crate::comparison::branch(ramo);
    if let Some((v, false)) = e.importancia {
        *((this + 0x51) as *mut u8) = v;
    }
    *((this + 0x44) as *mut u8) = 0xff;
    core::ptr::write_unaligned((this + 0x30) as *mut i32, -1);
    core::ptr::write_unaligned((this + 0x2c) as *mut u32, e.fases);
    let v = if e.fases != 0 { alocar(e.fases * 4) } else { 0 };
    core::ptr::write_unaligned((this + 0xc) as *mut usize, v);
    for &(o, n, x) in e.escritas {
        match n {
            1 => *((this + o) as *mut u8) = x as u8,
            2 => core::ptr::write_unaligned((this + o) as *mut u16, x as u16),
            _ => core::ptr::write_unaligned((this + o) as *mut u32, x),
        }
    }
    let mut l = e.linhas.iter();
    if let Some(r) = e.regras {
        let regras: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(r);
        let linha = *l.next().unwrap();
        if regras(this) == 0 {
            show_error_at(e.arquivo, linha);
            return this;
        }
    }
    let (lt, lj, lo, ls) = (*l.next().unwrap(), *l.next().unwrap(), *l.next().unwrap(), *l.next().unwrap());
    if tabela(this) == 0 {
        show_error_at(e.arquivo, lt);
        return this;
    }
    if jogos(this) == 0 {
        show_error_at(e.arquivo, lj);
        return this;
    }
    if ordenar(this, 0) == 0 {
        show_error_at(e.arquivo, lo);
        return this;
    }
    let n = novo(0x5ce);
    let s = if n != 0 { estatisticas(n, this) } else { 0 };
    core::ptr::write_unaligned((this + 8) as *mut usize, s);
    if s == 0 {
        show_error_at(e.arquivo, ls);
        return this;
    }
    if e.serie_c {
        let grupo: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x42ee80usize);
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        for g in 0..7u32 {
            grupo(this, g); // cqu2: os sete grupos da Série C
        }
        previstas(this);
        let lista = cqu2_d(this + 0xee);
        if lista != 0 {
            soltar(lista);
        }
        core::ptr::write_unaligned((this + 0xee) as *mut usize, 0);
        return this;
    }
    previstas(this);
    this
}

/// O comparador dos três: alocam e sorteiam os jogos: só a original, conferindo o que ela grava: a devolução, a
/// vtable, o ano, o registro, a disciplina e a importância gravada sempre (a MLS); na liga nova (+0x44 = 0xff, com as
/// estatísticas), as fases, o vetor delas (nulo sem fases), a importância, as regras em linha e, na Série C, os sete
/// grupos nas vagas 0 a 6 e a lista dos 56 zerada.
unsafe fn cqu2_comparar(nome: &str, this: usize, ano: u32, registro: usize, e: &Cqu2Liga, disciplina: u8, original: impl FnOnce() -> usize) -> usize {
    let r = original();
    let b = |o: usize| *((this + o) as *const u8);
    let mut ok = r == this && cqu2_d(this) == e.vtabela && core::ptr::read_unaligned((this + 0x40) as *const u16) == ano as u16
        && cqu2_d(this + 4) == registro && b(0x50) == disciplina;
    if let Some((v, true)) = e.importancia {
        ok = ok && b(0x51) == v;
    }
    if b(0x44) == 0xff && cqu2_d(this + 8) != 0 {
        ok = ok && cqu2_d(this + 0x2c) == e.fases as usize && (cqu2_d(this + 0xc) != 0) == (e.fases != 0);
        if let Some((v, _)) = e.importancia {
            ok = ok && b(0x51) == v;
        }
        // ponytail: só as regras dos pontos e dos critérios (+0xbe..+0xc8); a ordem (0x6827d0) reescreve +0xa7 e as
        // outras podem mudar com os jogos
        for &(o, n, x) in e.escritas.iter().filter(|w| (0xbe..=0xc8).contains(&w.0)) {
            let lido = match n {
                1 => b(o) as u32,
                2 => core::ptr::read_unaligned((this + o) as *const u16) as u32,
                _ => cqu2_d(this + o) as u32,
            };
            ok = ok && lido == x;
        }
        if e.serie_c {
            ok = ok && cqu2_d(this + 0xee) == 0 && (0..7).all(|g| cqu2_d(cqu2_d(this + 0xc) + 4 * g) != 0);
        }
    }
    SCOREBOARD.record(ok, || format!("{}({:#x}): devolveu {:#x}, vtable {:#x}, +0x2c = {}, +0xc = {:#x}, +8 = {:#x}, +0x51 = {}", nome, this, r, cqu2_d(this), cqu2_d(this + 0x2c), cqu2_d(this + 0xc), cqu2_d(this + 8), b(0x51)));
    r
}

const CQU2_SERIE_C: Cqu2Liga = Cqu2Liga {
    vtabela: 0x967f00,
    importancia: None,
    fases: 0xb,
    escritas: &[],
    regras: Some(0x42de50),
    tabela: 0x42dc90,
    jogos: 0x6835c0,
    previstas: 0x42f0a0,
    serie_c: true,
    arquivo: 0x98aba4,
    linhas: &[0x4a, 0x51, 0x58, 0x61, 0x68],
};

const CQU2_J2: Cqu2Liga = Cqu2Liga {
    vtabela: 0x96caf4,
    importancia: None,
    fases: 0,
    escritas: &[],
    regras: Some(0x665140),
    tabela: 0x665030,
    jogos: 0x6835c0,
    previstas: 0x68a850,
    serie_c: false,
    arquivo: 0x9eaedc,
    linhas: &[0x46, 0x4d, 0x53, 0x5a, 0x61],
};

/// As regras da MLS, gravadas em linha na ordem da original (deslocamento, tamanho, valor).
const CQU2_REGRAS_MLS: [(usize, u8, u32); 18] = [
    (0xc2, 1, 3), // cqu2: os pontos por vitória na MLS
    (0xc7, 1, 3),
    (0xc3, 1, 1),
    (0xab, 2, 0xc2),
    (0xc4, 1, 2),
    (0x42, 1, 1),
    (0xc5, 1, 1),
    (0xc6, 1, 2),
    (0xc8, 1, 0),
    (0xbe, 1, 1),
    (0xbf, 1, 0),
    (0xc0, 1, 0),
    (0xc1, 1, 0),
    (0x1c, 4, 0xffff_ffff),
    (0x20, 4, 0xffff_ffff),
    (0xd9, 2, 0x48),
    (0x3c, 2, 4),
    (0xa7, 2, 0x20),
];

const CQU2_MLS: Cqu2Liga = Cqu2Liga {
    vtabela: 0x970844,
    importancia: Some((0xa, true)),
    fases: 4,
    escritas: &CQU2_REGRAS_MLS,
    regras: None,
    tabela: 0x908560,
    jogos: 0x9086e0,
    previstas: 0x909bb0,
    serie_c: false,
    arquivo: 0xaa1fb4,
    linhas: &[0x59, 0x5f, 0x66, 0x6d],
};

/// { 0x42d2c0 } (thiscall, ret 8, bra_nat_third.cpp): `BraNatThird::BraNatThird(ano, registro)`: a Série C (onze fases além da liga: os sete grupos e as fases seguintes).
unsafe extern "thiscall" fn cqu2_construir_serie_c(this: usize, ano: u32, registro: usize) -> usize {
    cqu2_construir(this, ano, registro, &CQU2_SERIE_C, 7, "cqu_2 serie_c nova") // cqu2: a Série C
}

/// { 0x6644c0 } (thiscall, ret 8, jap_j2.cpp): `JapJ2::JapJ2(ano, registro)`: a J2 (só pontos corridos, sem fases além da liga).
unsafe extern "thiscall" fn cqu2_construir_j2(this: usize, ano: u32, registro: usize) -> usize {
    cqu2_construir(this, ano, registro, &CQU2_J2, 0x13, "cqu_2 j2 nova") // cqu2: a disciplina da J2
}

/// { 0x906f80 } (thiscall, ret 8, usa_mls.cpp): `UsaMls::UsaMls(ano, registro)`: a MLS (quatro fases além da liga: as conferências e o mata-mata).
unsafe extern "thiscall" fn cqu2_construir_mls(this: usize, ano: u32, registro: usize) -> usize {
    cqu2_construir(this, ano, registro, &CQU2_MLS, 0x1e, "cqu_2 mls nova") // cqu2: a MLS
}

unsafe extern "thiscall" fn compare_cqu2_construir_serie_c(this: usize, ano: u32, registro: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
    cqu2_comparar("cqu2_construir_serie_c", this, ano, registro, &CQU2_SERIE_C, 7, || o(this, ano, registro))
}

unsafe extern "thiscall" fn compare_cqu2_construir_j2(this: usize, ano: u32, registro: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I1%%);
    cqu2_comparar("cqu2_construir_j2", this, ano, registro, &CQU2_J2, 0x13, || o(this, ano, registro))
}

unsafe extern "thiscall" fn compare_cqu2_construir_mls(this: usize, ano: u32, registro: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I2%%);
    cqu2_comparar("cqu2_construir_mls", this, ano, registro, &CQU2_MLS, 0x1e, || o(this, ano, registro))
}
