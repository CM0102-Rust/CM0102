use super::*;

// ---------- 276a leva: as supercopas e o Jogo das Estrelas: os clubes da Supercopa da Espanha e do Japão e o calendário do Jogo das Estrelas da MLS (spa_super.cpp, jap_super.cpp, usa_mls_all_stars.cpp) ----------


/// O clube que terminou em `lugar` (0 o campeão, 1 o vice) na última temporada da competição de id [id] (0x4afd80: push
/// lugar; push competição do banco [0xae23d0] + 0x6b × id; add esp, 8 -> cdecl).
pub(crate) unsafe fn cqm12_colocado(id: usize, lugar: u32) -> usize {
    let colocado: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_COMP_UTIL_GET_LATEST_PLACED_CLUB);
    colocado(read_u32_as_usize(G_CLUB_COMP_TABLE).wrapping_add(read_u32_as_usize(id).wrapping_mul(0x6b)), lugar)
}

/// O começo comum: +0x3e = 2 e a lista de dois (0x944e46 de 0xc em +0xa7); sem memória, a caixa `linha` e None.
pub(crate) unsafe fn cqm12_lista(this: usize, arquivo: usize, linha: u32) -> Option<usize> {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 2);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let l = alocar(0xc);
    core::ptr::write_unaligned((this + 0xa7) as *mut u32, l as u32);
    if l == 0 {
        aviso_em(arquivo, linha);
        return None;
    }
    Some(l)
}

/// O fim comum: classe e situação 0 nos dois; devolve 1.
pub(crate) unsafe fn cqm12_fim(l: usize) -> (i32, i32) {
    *((l + 4) as *mut u8) = 0;
    *((l + 5) as *mut u8) = 0;
    *((l + 0xa) as *mut u8) = 0;
    *((l + 0xb) as *mut u8) = 0;
    (1, 2)
}

/// Grava o clube `c` na vaga `v` da lista; sem clube, a caixa `linha` (fonte `arquivo`) e falso.
pub(crate) unsafe fn cqm12_vaga(l: usize, v: usize, c: usize, arquivo: usize, linha: u32) -> bool {
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
pub(crate) unsafe fn cqm12_espanha(this: usize) -> (i32, i32) {
    const F: usize = 0xa82a58;
    let l = match cqm12_lista(this, F, 0xc2) {
        Some(l) => l,
        None => return (-1, 0),
    };
    let a = cqm12_colocado(KEY_CLUB_COMP_SPANISH_CUP, 0); // cqm12: o campeão da Copa do Rei
    if !cqm12_vaga(l, 0, a, F, 0xcb) {
        return (0, 0);
    }
    let b = cqm12_colocado(KEY_CLUB_COMP_SPANISH_FIRST_DIVISION, 0);
    if !cqm12_vaga(l, 1, b, F, 0xd4) {
        return (0, 0);
    }
    if a == b {
        crate::comparacao::ramo("cqm_12 dobradinha espanhola");
        if !cqm12_vaga(l, 0, cqm12_colocado(KEY_CLUB_COMP_SPANISH_CUP, 1), F, 0xe0) {
            return (0, 0);
        }
    }
    crate::comparacao::ramo("cqm_12 clubes da supercopa da espanha");
    cqm12_fim(l)
}

/// rule: the Spanish Super Cup is between the Copa del Rey winner (home in the first leg [inference]) and the League
/// champion; if the same club won both, the Copa del Rey runner-up takes its place; if one is missing, the error box
/// Original: `SpaSuper::select_teams` (0x858aa0, spa_super.cpp, thiscall, ret 0).
///
/// [pt] regra: a Supercopa da Espanha é entre o campeão da Copa do Rei (o mandante da ida [inferência]) e o campeão da Liga;
/// se o mesmo clube ganhou as duas, entra o vice da Copa do Rei; sem algum deles, a caixa de erro
pub(crate) unsafe extern "thiscall" fn cqm12_supercopa_da_espanha_clubes(this: usize) -> i32 {
    cqm12_espanha(this).0
}

/// Lado a lado para as rotinas que alocam a lista +0xa7 (como `cqm10_comparar`): o +0x3e e o [0xb67a34] a partir do
/// mesmo estado; a lista da cópia é comparada nos 12 bytes (só com a devolução 1) e liberada.
pub(crate) unsafe fn cqm12_comparar(nome: &str, this: usize, o: impl FnOnce() -> i32, c: impl FnOnce() -> (i32, i32)) -> i32 {
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
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
        soltar(lr);
    }
    PLACAR.registrar(igual && a == b && lista, || format!("{}({:#x}) devolveu {} x {} memória {} lista {}", nome, this, a, b, igual, lista));
    a
}

pub(crate) unsafe extern "thiscall" fn comparar_cqm12_supercopa_da_espanha_clubes(this: usize) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> i32>(1787);
    cqm12_comparar("cqm12_supercopa_da_espanha_clubes", this, || o(this), || cqm12_espanha(this))
}

/// { 0x666160 } (thiscall, ret 0, jap_super.cpp): `JapSuper::select_teams()`: a lista (sem memória, a caixa da linha
/// 0xc1 e −1). O campeão da J1 ([0x9cf65c]; sem ele, 0xca e 0) — sem nação (+0x53 = 0), o vice (sem ele, 0xd6 e 0); o da
/// Copa do Imperador ([0x9cf660]; sem ele, 0xe0 e 0) — sem nação, o vice (sem ele, 0xec e 0). Se é o mesmo clube, o
/// segundo vira o vice da Copa do Imperador (sem ele, 0xf9 e 0) e, se ainda é o mesmo, o primeiro vira o vice da J1
/// (sem ele, 0x105 e 0). Devolve 1.
pub(crate) unsafe fn cqm12_japao(this: usize) -> (i32, i32) {
    const F: usize = 0x9eaf4c;
    let l = match cqm12_lista(this, F, 0xc1) {
        Some(l) => l,
        None => return (-1, 0),
    };
    let a = cqm12_colocado(KEY_CLUB_COMP_JAPANESE_J_LEAGUE_1, 0); // cqm12: o campeão da J1
    if !cqm12_vaga(l, 0, a, F, 0xca) {
        return (0, 0);
    }
    if read_u32_as_usize(a + 0x53) == 0 && !cqm12_vaga(l, 0, cqm12_colocado(KEY_CLUB_COMP_JAPANESE_J_LEAGUE_1, 1), F, 0xd6) {
        return (0, 0);
    }
    let b = cqm12_colocado(KEY_CLUB_COMP_JAPANESE_EMPEROR_S_CUP, 0);
    if !cqm12_vaga(l, 1, b, F, 0xe0) {
        return (0, 0);
    }
    if read_u32_as_usize(b + 0x53) == 0 && !cqm12_vaga(l, 1, cqm12_colocado(KEY_CLUB_COMP_JAPANESE_EMPEROR_S_CUP, 1), F, 0xec) {
        return (0, 0);
    }
    if read_u32_as_usize(l) == read_u32_as_usize(l + 6) {
        crate::comparacao::ramo("cqm_12 dobradinha japonesa");
        if !cqm12_vaga(l, 1, cqm12_colocado(KEY_CLUB_COMP_JAPANESE_EMPEROR_S_CUP, 1), F, 0xf9) {
            return (0, 0);
        }
        if read_u32_as_usize(l) == read_u32_as_usize(l + 6) && !cqm12_vaga(l, 0, cqm12_colocado(KEY_CLUB_COMP_JAPANESE_J_LEAGUE_1, 1), F, 0x105) {
            return (0, 0);
        }
    }
    crate::comparacao::ramo("cqm_12 clubes da supercopa do japao");
    cqm12_fim(l)
}

/// rule: the Japanese Super Cup is between the J1 champion and the Emperor's Cup winner; a winner with no nation (+0x53
/// = 0, a club that no longer exists [inference]) gives the place to the runner-up; if it is the same club, the
/// Emperor's Cup runner-up comes in and, if they still coincide, the J1 runner-up; if one is missing, the error box
/// Original: `JapSuper::select_teams` (0x666160, jap_super.cpp, thiscall, ret 0).
///
/// [pt] regra: a Supercopa do Japão é entre o campeão da J1 e o da Copa do Imperador; um campeão sem nação (+0x53 = 0, clube
/// que não existe mais [inferência]) dá a vaga ao vice; se é o mesmo clube, entra o vice da Copa do Imperador e, se
/// ainda coincidir, o vice da J1; sem algum deles, a caixa de erro
pub(crate) unsafe extern "thiscall" fn cqm12_supercopa_do_japao_clubes(this: usize) -> i32 {
    cqm12_japao(this).0
}

pub(crate) unsafe extern "thiscall" fn comparar_cqm12_supercopa_do_japao_clubes(this: usize) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> i32>(1788);
    cqm12_comparar("cqm12_supercopa_do_japao_clubes", this, || o(this), || cqm12_japao(this))
}


/// Os bytes que a original grava na rodada: +0..+0x10, +0x13..+0x22 e +0x5c..+0x67.
pub(crate) unsafe fn cqm12_gravados(t: usize) -> Vec<u8> {
    let mut v = Vec::new();
    for &(a, b) in &[(0usize, 0x11usize), (0x13, 0x23), (0x5c, 0x68)] {
        v.extend_from_slice(core::slice::from_raw_parts((t + a) as *const u8, b - a));
    }
    v
}

/// rule: the Spanish Super Cup is played over two legs (7 days apart) between 2 clubs, without a draw (+0xd = 0x200:
/// the built order), drawn on 5/7 with the first leg on 20/8 (moved to weekday 6) at the home side [inference: venue
/// 0]; two-leg tiebreak 1 [inference: away goals and penalties]; no prize; category 6
/// Original: `SpaSuper::create_round_table` (0x858960, spa_super.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x858960 } (thiscall, ret 0x10, spa_super.cpp; a virtual +0x3c da Supercopa da Espanha): `SpaSuper::
/// create_round_table(fase, &rodadas, &extra, &saida)`: na fase −1, a rodada única; a caixa da linha 0x99 sem memória.
pub(crate) unsafe extern "thiscall" fn cql12_supercopa_da_espanha_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    cql12_tabela(this, fase, rodadas, extra, saida, &CQL12_SUPERCOPA_DA_ESPANHA, 0xa82a58, 0x99, "cql_12 tabela da supercopa da espanha", |_| {})
}

pub(crate) unsafe extern "thiscall" fn comparar_cql12_supercopa_da_espanha_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1779);
    cql12_comparar("cql12_supercopa_da_espanha_tabela", fase, rodadas, extra, saida, || o(this, fase, rodadas, extra, saida),
        |r, e, s| cql12_supercopa_da_espanha_tabela(this, fase, r, e, s))
}
