use super::*;

/// Common start: +0x3e = 2 and allocates the two-entry list (0xc bytes) at +0xa7; None with the error box on out of
/// memory.
///
/// [pt] O começo comum: +0x3e = 2 e a lista de dois (0x944e46 de 0xc em +0xa7); sem memória, a caixa `linha` e None.
pub(crate) unsafe fn alloc_two_team_list(this: usize, file: usize, row: u32) -> Option<usize> {
    core::ptr::write_unaligned((this + cup::TEAM_COUNT) as *mut i16, 2);
    let l = exe::crt_malloc(0xc);
    core::ptr::write_unaligned((this + cup::TEAM_TABLE) as *mut u32, l as u32);
    if l == 0 {
        show_error_at(file, row);
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
pub(crate) unsafe fn set_team_slot(l: usize, v: usize, c: usize, file: usize, row: u32) -> bool {
    core::ptr::write_unaligned((l + 6 * v) as *mut u32, c as u32);
    if c == 0 {
        show_error_at(file, row);
        return false;
    }
    true
}

/// `SpaSuper::select_teams()` (`0x858aa0`, thiscall, ret 0, `spa_super.cpp`): picks the Copa del Rey winner and the
/// league champion for the Spanish Super Cup.
///
/// Allocates the list (out of memory, the box of line `0xc2` and −1). The Copa del Rey winner ([`0x9cf740`]; without
/// it, box `0xcb` and 0) and the league champion ([`0x9cf624`]; without it, `0xd4` and 0); if both are the same club,
/// the first becomes the Copa del Rey runner-up (without it, `0xe0` and 0). Returns 1.
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
    let a = latest_placed_club(KEY_CLUB_COMP_SPANISH_CUP, 0); // cqm12: the Copa del Rey champion
    if !set_team_slot(l, 0, a, F, 0xcb) {
        return (0, 0);
    }
    let b = latest_placed_club(KEY_CLUB_COMP_SPANISH_FIRST_DIVISION, 0);
    if !set_team_slot(l, 1, b, F, 0xd4) {
        return (0, 0);
    }
    if a == b {
        crate::comparison::branch("cqm_12 dobradinha espanhola");
        if !set_team_slot(l, 0, latest_placed_club(KEY_CLUB_COMP_SPANISH_CUP, 1), F, 0xe0) {
            return (0, 0);
        }
    }
    crate::comparison::branch("cqm_12 clubes da supercopa da espanha");
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
pub(crate) unsafe fn compare_team_list(name: &str, this: usize, o: impl FnOnce() -> i32, c: impl FnOnce() -> (i32, i32)) -> i32 {
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let reg = [(this + cup::TEAM_COUNT, 2usize), (G_MOUSE_LEFT_BUTTON_DOWN, 4)];
    let (a, b, igual, _, _) = crate::comparison::side_by_side(&reg, || {
        let r = o();
        lo.set(read_usize(this + cup::TEAM_TABLE));
        r
    }, || {
        let (r, _) = c();
        lr.set(read_usize(this + cup::TEAM_TABLE));
        core::ptr::write_unaligned((this + cup::TEAM_TABLE) as *mut u32, lo.get() as u32);
        r
    });
    let (lo, lr) = (lo.get(), lr.get());
    let k = if a == 1 { 12 } else { 0 };
    let list = if lo != 0 && lr != 0 {
        core::slice::from_raw_parts(lo as *const u8, k) == core::slice::from_raw_parts(lr as *const u8, k)
    } else {
        lo == 0 && lr == 0
    };
    if lr != 0 && lr != lo {
        exe::crt_free(lr);
    }
    SCOREBOARD.record(igual && a == b && list, || format!("{}({:#x}) devolveu {} x {} memória {} lista {}", name, this, a, b, igual, list));
    a
}

/// `JapSuper::select_teams()` (0x666160, thiscall, ret 0, jap_super.cpp): the J1 champion and the Emperor's Cup winner.
/// The list (out of memory, the box for line 0xc1 and −1).
///
/// The J1 champion ([0x9cf65c]; without it, 0xca and 0) — with no nation (+0x53 = 0), the runner-up (without it, 0xd6
/// and 0); the Emperor's Cup winner ([0x9cf660]; without it, 0xe0 and 0) — with no nation, the runner-up (without it,
/// 0xec and 0).
///
/// If they are the same club, the second becomes the Emperor's Cup runner-up (without it, 0xf9 and 0) and, if still the
/// same, the first becomes the J1 runner-up (without it, 0x105 and 0). Returns 1.
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
    let a = latest_placed_club(KEY_CLUB_COMP_JAPANESE_J_LEAGUE_1, 0); // cqm12: the J1 champion
    if !set_team_slot(l, 0, a, F, 0xca) {
        return (0, 0);
    }
    if read_usize(a + 0x53) == 0 && !set_team_slot(l, 0, latest_placed_club(KEY_CLUB_COMP_JAPANESE_J_LEAGUE_1, 1), F, 0xd6) {
        return (0, 0);
    }
    let b = latest_placed_club(KEY_CLUB_COMP_JAPANESE_EMPEROR_S_CUP, 0);
    if !set_team_slot(l, 1, b, F, 0xe0) {
        return (0, 0);
    }
    if read_usize(b + 0x53) == 0 && !set_team_slot(l, 1, latest_placed_club(KEY_CLUB_COMP_JAPANESE_EMPEROR_S_CUP, 1), F, 0xec) {
        return (0, 0);
    }
    if read_usize(l) == read_usize(l + 6) {
        crate::comparison::branch("cqm_12 dobradinha japonesa");
        if !set_team_slot(l, 1, latest_placed_club(KEY_CLUB_COMP_JAPANESE_EMPEROR_S_CUP, 1), F, 0xf9) {
            return (0, 0);
        }
        if read_usize(l) == read_usize(l + 6) && !set_team_slot(l, 0, latest_placed_club(KEY_CLUB_COMP_JAPANESE_J_LEAGUE_1, 1), F, 0x105) {
            return (0, 0);
        }
    }
    crate::comparison::branch("cqm_12 clubes da supercopa do japao");
    finish_two_team_list(l)
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
pub(crate) unsafe extern "thiscall" fn spa_super_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    fra_super_create_single_stage_round_table(this, stage, rounds, extra, out, &SPANISH_SUPER_CUP, 0xa82a58, 0x99, "cql_12 tabela da supercopa da espanha", |_| {})
}

