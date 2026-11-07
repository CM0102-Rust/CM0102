// ---------- 276a leva: as supercopas e o Jogo das Estrelas: os clubes da Supercopa da Espanha e do Japão e o calendário do Jogo das Estrelas da MLS (spa_super.cpp, jap_super.cpp, usa_mls_all_stars.cpp) ----------


/// O clube que terminou em `lugar` (0 o campeão, 1 o vice) na última temporada da competição de id [id] (0x4afd80: push
/// lugar; push competição do banco [0xae23d0] + 0x6b × id; add esp, 8 -> cdecl).
unsafe fn cqm12_colocado(id: usize, lugar: u32) -> usize {
    let colocado: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x4afd80usize);
    colocado(read_u32_as_usize(0xae23d0).wrapping_add(read_u32_as_usize(id).wrapping_mul(0x6b)), lugar)
}

/// O começo comum: +0x3e = 2 e a lista de dois (0x944e46 de 0xc em +0xa7); sem memória, a caixa `linha` e None.
unsafe fn cqm12_lista(this: usize, arquivo: usize, linha: u32) -> Option<usize> {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 2);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let l = alocar(0xc);
    core::ptr::write_unaligned((this + 0xa7) as *mut u32, l as u32);
    if l == 0 {
        aviso_em(arquivo, linha);
        return None;
    }
    Some(l)
}

/// O fim comum: classe e situação 0 nos dois; devolve 1.
unsafe fn cqm12_fim(l: usize) -> (i32, i32) {
    *((l + 4) as *mut u8) = 0;
    *((l + 5) as *mut u8) = 0;
    *((l + 0xa) as *mut u8) = 0;
    *((l + 0xb) as *mut u8) = 0;
    (1, 2)
}

/// Grava o clube `c` na vaga `v` da lista; sem clube, a caixa `linha` (fonte `arquivo`) e falso.
unsafe fn cqm12_vaga(l: usize, v: usize, c: usize, arquivo: usize, linha: u32) -> bool {
    core::ptr::write_unaligned((l + 6 * v) as *mut u32, c as u32);
    if c == 0 {
        aviso_em(arquivo, linha);
        return false;
    }
    true
}

/// { 0x858aa0 } (thiscall, ret 0, spa_super.cpp): `SpaSuper::select_teams()`: a lista (sem memória, a caixa da linha
/// 0xc2 e −1). O campeão da Copa do Rei ([0x9cf740]; sem ele, a caixa 0xcb e 0) e o da Liga ([0x9cf624]; sem ele, 0xd4
/// e 0); se é o mesmo clube, o primeiro vira o vice da Copa do Rei (sem ele, 0xe0 e 0). Devolve 1.
unsafe fn cqm12_espanha(this: usize) -> (i32, i32) {
    const F: usize = 0xa82a58;
    let l = match cqm12_lista(this, F, 0xc2) {
        Some(l) => l,
        None => return (-1, 0),
    };
    let a = cqm12_colocado(0x9cf740, 0); // cqm12: o campeão da Copa do Rei
    if !cqm12_vaga(l, 0, a, F, 0xcb) {
        return (0, 0);
    }
    let b = cqm12_colocado(0x9cf624, 0);
    if !cqm12_vaga(l, 1, b, F, 0xd4) {
        return (0, 0);
    }
    if a == b {
        crate::comparacao::ramo("cqm_12 dobradinha espanhola");
        if !cqm12_vaga(l, 0, cqm12_colocado(0x9cf740, 1), F, 0xe0) {
            return (0, 0);
        }
    }
    crate::comparacao::ramo("cqm_12 clubes da supercopa da espanha");
    cqm12_fim(l)
}

unsafe extern "thiscall" fn cqm12_supercopa_da_espanha_clubes(this: usize) -> i32 {
    cqm12_espanha(this).0
}

/// Lado a lado para as rotinas que alocam a lista +0xa7 (como `cqm10_comparar`): o +0x3e e o [0xb67a34] a partir do
/// mesmo estado; a lista da cópia é comparada nos 12 bytes (só com a devolução 1) e liberada.
unsafe fn cqm12_comparar(nome: &str, this: usize, o: impl FnOnce() -> i32, c: impl FnOnce() -> (i32, i32)) -> i32 {
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let reg = [(this + 0x3e, 2usize), (0xb67a34, 4)];
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || {
        let r = o();
        lo.set(read_u32_as_usize(this + 0xa7));
        r
    }, || {
        let (r, _) = c();
        lr.set(read_u32_as_usize(this + 0xa7));
        core::ptr::write_unaligned((this + 0xa7) as *mut u32, lo.get() as u32);
        r
    });
    let (lo, lr) = (lo.get(), lr.get());
    let k = if a == 1 { 12 } else { 0 };
    let lista = if lo != 0 && lr != 0 {
        core::slice::from_raw_parts(lo as *const u8, k) == core::slice::from_raw_parts(lr as *const u8, k)
    } else {
        lo == 0 && lr == 0
    };
    if lr != 0 && lr != lo {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        soltar(lr);
    }
    PLACAR.registrar(igual && a == b && lista, || format!("{}({:#x}) devolveu {} x {} memória {} lista {}", nome, this, a, b, igual, lista));
    a
}

unsafe extern "thiscall" fn comparar_cqm12_supercopa_da_espanha_clubes(this: usize) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> i32>(1787);
    cqm12_comparar("cqm12_supercopa_da_espanha_clubes", this, || o(this), || cqm12_espanha(this))
}

/// { 0x666160 } (thiscall, ret 0, jap_super.cpp): `JapSuper::select_teams()`: a lista (sem memória, a caixa da linha
/// 0xc1 e −1). O campeão da J1 ([0x9cf65c]; sem ele, 0xca e 0) — sem nação (+0x53 = 0), o vice (sem ele, 0xd6 e 0); o da
/// Copa do Imperador ([0x9cf660]; sem ele, 0xe0 e 0) — sem nação, o vice (sem ele, 0xec e 0). Se é o mesmo clube, o
/// segundo vira o vice da Copa do Imperador (sem ele, 0xf9 e 0) e, se ainda é o mesmo, o primeiro vira o vice da J1
/// (sem ele, 0x105 e 0). Devolve 1.
unsafe fn cqm12_japao(this: usize) -> (i32, i32) {
    const F: usize = 0x9eaf4c;
    let l = match cqm12_lista(this, F, 0xc1) {
        Some(l) => l,
        None => return (-1, 0),
    };
    let a = cqm12_colocado(0x9cf65c, 0); // cqm12: o campeão da J1
    if !cqm12_vaga(l, 0, a, F, 0xca) {
        return (0, 0);
    }
    if read_u32_as_usize(a + 0x53) == 0 && !cqm12_vaga(l, 0, cqm12_colocado(0x9cf65c, 1), F, 0xd6) {
        return (0, 0);
    }
    let b = cqm12_colocado(0x9cf660, 0);
    if !cqm12_vaga(l, 1, b, F, 0xe0) {
        return (0, 0);
    }
    if read_u32_as_usize(b + 0x53) == 0 && !cqm12_vaga(l, 1, cqm12_colocado(0x9cf660, 1), F, 0xec) {
        return (0, 0);
    }
    if read_u32_as_usize(l) == read_u32_as_usize(l + 6) {
        crate::comparacao::ramo("cqm_12 dobradinha japonesa");
        if !cqm12_vaga(l, 1, cqm12_colocado(0x9cf660, 1), F, 0xf9) {
            return (0, 0);
        }
        if read_u32_as_usize(l) == read_u32_as_usize(l + 6) && !cqm12_vaga(l, 0, cqm12_colocado(0x9cf65c, 1), F, 0x105) {
            return (0, 0);
        }
    }
    crate::comparacao::ramo("cqm_12 clubes da supercopa do japao");
    cqm12_fim(l)
}

unsafe extern "thiscall" fn cqm12_supercopa_do_japao_clubes(this: usize) -> i32 {
    cqm12_japao(this).0
}

unsafe extern "thiscall" fn comparar_cqm12_supercopa_do_japao_clubes(this: usize) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> i32>(1788);
    cqm12_comparar("cqm12_supercopa_do_japao_clubes", this, || o(this), || cqm12_japao(this))
}

/// { 0x90a400 } (thiscall, ret 0x10, usa_mls_all_stars.cpp; a virtual +0x3c do Jogo das Estrelas): `UsaMlsAllStars::
/// create_round_table(fase, &rodadas, &extra, &saida)`: fora da fase −1 (o byte), 0. Na −1: saida (se pedida) = 0,
/// rodadas = 1, extra = 0; a tabela (0x944e46 de 0x68; sem memória, a caixa da linha 0x9a e 0) com a rodada única: o
/// sorteio em 15 do mês 5 (junho) levado ao dia da semana 5, o jogo em 2 do mês 7 (agosto) levado ao dia 6, turno 1,
/// mando 0 (0x521e60 e 0x521eb0, cdecl, com o ano +0x40); nome 0, +0xd = 0x20, categoria 8, 2 times, 1 jogo, 2 novos,
/// 1 jogo por confronto, sem prêmio.
unsafe extern "thiscall" fn cqm12_jogo_das_estrelas_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    if fase as u8 != 0xff {
        return 0;
    }
    if !saida.is_null() {
        *saida = 0;
    }
    *rodadas = 1;
    *extra = 0;
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let t = alocar(0x68);
    if t == 0 {
        aviso_em(0xaa20a0, 0x9a);
        return 0;
    }
    crate::comparacao::ramo("cqm_12 tabela do jogo das estrelas");
    let ano = core::ptr::read_unaligned((this + 0x40) as *const i16) as i32 as u32;
    let sorteio: unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(0x521e60usize);
    let jogo: unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(0x521eb0usize);
    sorteio(t, 0, 0xf, 5, 0, 5, ano);
    jogo(t, 0, 2, 7, 0, 6, 1, ano, 0); // cqm12: o Jogo das Estrelas em 2 de agosto
    let w = |o: usize, v: u16| core::ptr::write_unaligned((t + o) as *mut u16, v);
    let d = |o: usize, v: u32| core::ptr::write_unaligned((t + o) as *mut u32, v);
    w(0x18, 2);
    w(0x1c, 2);
    w(7, 0);
    w(9, 0);
    w(0xb, 0);
    w(0xd, 0x20);
    *((t + 0x17) as *mut u8) = 8;
    w(0x1a, 1);
    w(0xf, 0);
    w(0x1e, 0);
    *((t + 0x20) as *mut u8) = 0;
    *((t + 0x21) as *mut u8) = 1;
    *((t + 0x22) as *mut u8) = 0;
    d(0x5c, 0);
    d(0x60, 0);
    d(0x64, 0);
    t
}

/// Os bytes que a original grava na rodada: +0..+0x10, +0x13..+0x22 e +0x5c..+0x67.
unsafe fn cqm12_gravados(t: usize) -> Vec<u8> {
    let mut v = Vec::new();
    for &(a, b) in &[(0usize, 0x11usize), (0x13, 0x23), (0x5c, 0x68)] {
        v.extend_from_slice(core::slice::from_raw_parts((t + a) as *const u8, b - a));
    }
    v
}

/// Lado a lado: a original com as saídas de quem chama, a cópia com saídas próprias que começam iguais, a partir do
/// mesmo [0xb67a34]; as saídas e os bytes gravados das duas tabelas têm de ser iguais (a da cópia é liberada). Fora da
/// fase −1, só a original (nenhuma grava nada).
unsafe extern "thiscall" fn comparar_cqm12_jogo_das_estrelas_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1789);
    if fase as u8 != 0xff {
        let a = o(this, fase, rodadas, extra, saida);
        PLACAR.registrar(a == 0, || format!("cqm12_jogo_das_estrelas_tabela(fase {}) devolveu {:#x}", fase as u8 as i8, a));
        return a;
    }
    let (mut r2, mut e2) = (*rodadas, *extra);
    let mut s2 = if saida.is_null() { 0 } else { *saida };
    let reg = [(0xb67a34usize, 4usize)];
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || o(this, fase, rodadas, extra, saida),
        || cqm12_jogo_das_estrelas_tabela(this, fase, &mut r2, &mut e2, if saida.is_null() { core::ptr::null_mut() } else { &mut s2 }));
    let saidas = *rodadas == r2 && *extra == e2 && (saida.is_null() || *saida == s2);
    let tabela = if a != 0 && b != 0 { cqm12_gravados(a) == cqm12_gravados(b) } else { a == 0 && b == 0 };
    if b != 0 {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        soltar(b);
    }
    PLACAR.registrar(igual && saidas && tabela, || format!("cqm12_jogo_das_estrelas_tabela: saídas {} tabela {}", saidas, tabela));
    a
}

