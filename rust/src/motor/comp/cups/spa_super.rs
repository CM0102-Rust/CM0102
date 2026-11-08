use super::*;

// ---------- 276a leva: as supercopas e o Jogo das Estrelas: os clubes da Supercopa da Espanha e do Japão e o calendário do Jogo das Estrelas da MLS (spa_super.cpp, jap_super.cpp, usa_mls_all_stars.cpp) ----------



/// Common start: +0x3e = 2 and allocates the two-entry list (0xc bytes) at +0xa7; None with the error box on out of
/// memory.
///
/// [pt] O começo comum: +0x3e = 2 e a lista de dois (0x944e46 de 0xc em +0xa7); sem memória, a caixa `linha` e None.
pub(crate) unsafe fn alloc_two_team_list(this: usize, arquivo: usize, linha: u32) -> Option<usize> {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 2);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let l = alocar(0xc);
    core::ptr::write_unaligned((this + 0xa7) as *mut u32, l as u32);
    if l == 0 {
        show_error_at(arquivo, linha);
        return None;
    }
    Some(l)
}

/// Common end: class and status 0 for both entries; returns (1, 2).
///
/// [pt] O fim comum: classe e situação 0 nos dois; devolve 1.
pub(crate) unsafe fn finish_two_team_list(l: usize) -> (i32, i32) {
    *((l + 4) as *mut u8) = 0;
    *((l + 5) as *mut u8) = 0;
    *((l + 0xa) as *mut u8) = 0;
    *((l + 0xb) as *mut u8) = 0;
    (1, 2)
}

/// Writes club c into slot v of the list; with no club shows the error box and returns false.
///
/// [pt] Grava o clube `c` na vaga `v` da lista; sem clube, a caixa `linha` (fonte `arquivo`) e falso.
pub(crate) unsafe fn set_team_slot(l: usize, v: usize, c: usize, arquivo: usize, linha: u32) -> bool {
    core::ptr::write_unaligned((l + 6 * v) as *mut u32, c as u32);
    if c == 0 {
        show_error_at(arquivo, linha);
        return false;
    }
    true
}

/// SpaSuper::select_teams (0x858aa0): Copa del Rey winner and league champion (cup runner-up if the same club).
///
/// [pt] { 0x858aa0 } (thiscall, ret 0, spa_super.cpp): `SpaSuper::select_teams()`: a lista (sem memória, a caixa da linha
/// 0xc2 e −1). O campeão da Copa do Rei ([0x9cf740]; sem ele, a caixa 0xcb e 0) e o da Liga ([0x9cf624]; sem ele, 0xd4
/// e 0); se é o mesmo clube, o primeiro vira o vice da Copa do Rei (sem ele, 0xe0 e 0). Devolve 1.
pub(crate) unsafe fn select_spanish_super_cup_teams(this: usize) -> (i32, i32) {
    const F: usize = 0xa82a58;
    let l = match alloc_two_team_list(this, F, 0xc2) {
        Some(l) => l,
        None => return (-1, 0),
    };
    let a = latest_placed_club(KEY_CLUB_COMP_SPANISH_CUP, 0); // cqm12: o campeão da Copa do Rei
    if !set_team_slot(l, 0, a, F, 0xcb) {
        return (0, 0);
    }
    let b = latest_placed_club(KEY_CLUB_COMP_SPANISH_FIRST_DIVISION, 0);
    if !set_team_slot(l, 1, b, F, 0xd4) {
        return (0, 0);
    }
    if a == b {
        crate::comparacao::ramo("cqm_12 dobradinha espanhola");
        if !set_team_slot(l, 0, latest_placed_club(KEY_CLUB_COMP_SPANISH_CUP, 1), F, 0xe0) {
            return (0, 0);
        }
    }
    crate::comparacao::ramo("cqm_12 clubes da supercopa da espanha");
    finish_two_team_list(l)
}

/// rule: the Spanish Super Cup is between the Copa del Rey winner (home in the first leg [inference]) and the League
/// champion; if the same club won both, the Copa del Rey runner-up takes its place; if one is missing, the error box
/// Original: `SpaSuper::select_teams` (0x858aa0, spa_super.cpp, thiscall, ret 0).
///
/// [pt] regra: a Supercopa da Espanha é entre o campeão da Copa do Rei (o mandante da ida [inferência]) e o campeão da Liga;
/// se o mesmo clube ganhou as duas, entra o vice da Copa do Rei; sem algum deles, a caixa de erro
pub(crate) unsafe extern "thiscall" fn spa_super_select_teams(this: usize) -> i32 {
    select_spanish_super_cup_teams(this).0
}

/// Side-by-side comparer for routines that allocate the list +0xa7: compares +0x3e, [0xb67a34] and the 12 list bytes
/// (when returning 1); frees the port's list.
///
/// [pt] Lado a lado para as rotinas que alocam a lista +0xa7 (como `eng_cc_cup_compare_entrant_list`): o +0x3e e o [0xb67a34] a partir do
/// mesmo estado; a lista da cópia é comparada nos 12 bytes (só com a devolução 1) e liberada.
pub(crate) unsafe fn compare_team_list(nome: &str, this: usize, o: impl FnOnce() -> i32, c: impl FnOnce() -> (i32, i32)) -> i32 {
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

pub(crate) unsafe extern "thiscall" fn comparar_spa_super_select_teams(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_SPA_SUPER_SELECT_TEAMS);
    compare_team_list("spa_super_select_teams", this, || o(this), || select_spanish_super_cup_teams(this))
}

/// JapSuper::select_teams (0x666160): J1 champion and Emperor's Cup winner (runners-up when the winner has no nation or
/// both are the same club).
///
/// [pt] { 0x666160 } (thiscall, ret 0, jap_super.cpp): `JapSuper::select_teams()`: a lista (sem memória, a caixa da linha
/// 0xc1 e −1). O campeão da J1 ([0x9cf65c]; sem ele, 0xca e 0) — sem nação (+0x53 = 0), o vice (sem ele, 0xd6 e 0); o da
/// Copa do Imperador ([0x9cf660]; sem ele, 0xe0 e 0) — sem nação, o vice (sem ele, 0xec e 0). Se é o mesmo clube, o
/// segundo vira o vice da Copa do Imperador (sem ele, 0xf9 e 0) e, se ainda é o mesmo, o primeiro vira o vice da J1
/// (sem ele, 0x105 e 0). Devolve 1.
pub(crate) unsafe fn select_japanese_super_cup_teams(this: usize) -> (i32, i32) {
    const F: usize = 0x9eaf4c;
    let l = match alloc_two_team_list(this, F, 0xc1) {
        Some(l) => l,
        None => return (-1, 0),
    };
    let a = latest_placed_club(KEY_CLUB_COMP_JAPANESE_J_LEAGUE_1, 0); // cqm12: o campeão da J1
    if !set_team_slot(l, 0, a, F, 0xca) {
        return (0, 0);
    }
    if read_u32_as_usize(a + 0x53) == 0 && !set_team_slot(l, 0, latest_placed_club(KEY_CLUB_COMP_JAPANESE_J_LEAGUE_1, 1), F, 0xd6) {
        return (0, 0);
    }
    let b = latest_placed_club(KEY_CLUB_COMP_JAPANESE_EMPEROR_S_CUP, 0);
    if !set_team_slot(l, 1, b, F, 0xe0) {
        return (0, 0);
    }
    if read_u32_as_usize(b + 0x53) == 0 && !set_team_slot(l, 1, latest_placed_club(KEY_CLUB_COMP_JAPANESE_EMPEROR_S_CUP, 1), F, 0xec) {
        return (0, 0);
    }
    if read_u32_as_usize(l) == read_u32_as_usize(l + 6) {
        crate::comparacao::ramo("cqm_12 dobradinha japonesa");
        if !set_team_slot(l, 1, latest_placed_club(KEY_CLUB_COMP_JAPANESE_EMPEROR_S_CUP, 1), F, 0xf9) {
            return (0, 0);
        }
        if read_u32_as_usize(l) == read_u32_as_usize(l + 6) && !set_team_slot(l, 0, latest_placed_club(KEY_CLUB_COMP_JAPANESE_J_LEAGUE_1, 1), F, 0x105) {
            return (0, 0);
        }
    }
    crate::comparacao::ramo("cqm_12 clubes da supercopa do japao");
    finish_two_team_list(l)
}

/// rule: the Japanese Super Cup is between the J1 champion and the Emperor's Cup winner; a winner with no nation (+0x53
/// = 0, a club that no longer exists [inference]) gives the place to the runner-up; if it is the same club, the
/// Emperor's Cup runner-up comes in and, if they still coincide, the J1 runner-up; if one is missing, the error box
/// Original: `JapSuper::select_teams` (0x666160, jap_super.cpp, thiscall, ret 0).
///
/// [pt] regra: a Supercopa do Japão é entre o campeão da J1 e o da Copa do Imperador; um campeão sem nação (+0x53 = 0, clube
/// que não existe mais [inferência]) dá a vaga ao vice; se é o mesmo clube, entra o vice da Copa do Imperador e, se
/// ainda coincidir, o vice da J1; sem algum deles, a caixa de erro
pub(crate) unsafe extern "thiscall" fn spa_super_select_teams_666160(this: usize) -> i32 {
    select_japanese_super_cup_teams(this).0
}

pub(crate) unsafe extern "thiscall" fn comparar_spa_super_select_teams_666160(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_SPA_SUPER_SELECT_TEAMS_666160);
    compare_team_list("spa_super_select_teams_666160", this, || o(this), || select_japanese_super_cup_teams(this))
}


/// Bytes the original writes in a round: +0..+0x10, +0x13..+0x22 and +0x5c..+0x67.
///
/// [pt] Os bytes que a original grava na rodada: +0..+0x10, +0x13..+0x22 e +0x5c..+0x67.
pub(crate) unsafe fn spa_super_written_round_bytes(t: usize) -> Vec<u8> {
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
pub(crate) unsafe extern "thiscall" fn spa_super_create_round_table(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    fra_super_create_single_stage_round_table(this, fase, rodadas, extra, saida, &CQL12_SUPERCOPA_DA_ESPANHA, 0xa82a58, 0x99, "cql_12 tabela da supercopa da espanha", |_| {})
}

pub(crate) unsafe extern "thiscall" fn comparar_spa_super_create_round_table(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_SPA_SUPER_CREATE_ROUND_TABLE);
    fra_super_compare_round_table("spa_super_create_round_table", fase, rodadas, extra, saida, || o(this, fase, rodadas, extra, saida),
        |r, e, s| spa_super_create_round_table(this, fase, r, e, s))
}
