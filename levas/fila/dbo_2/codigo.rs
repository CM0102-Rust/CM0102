// ---------- %%LEVA%%a leva: o olheiro pergunta se o clube tem esforçados, goleiro em condições e meias que roubam a bola (Database.cpp) ----------

/// Quantos dos 50 do elenco (+0xd7) contam: pessoa não nula com jogador (+0x61) da `posicao`, sem lesão grave
/// (INJURY 0x62c400 em [0xb63d28] ≠ 2), id (+0) < [0xae2364] (com sinal), habilidade atual (+5 do jogador, curto com
/// sinal) acima do `limite` e o `extra` (os atributos do jogador). Na ordem da original (a 0x62c400 só para os da
/// posição). Contador de 8 bits (no máximo 50).
unsafe fn count_squad_above_ability_with(clube: usize, limit: i32, posicao: impl Fn(usize) -> bool, extra: impl Fn(usize) -> bool) -> u8 {
    let rd = |a: usize| core::ptr::read_unaligned(a as *const usize);
    let lesao: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x62c400usize);
    let mut n = 0u8;
    for k in 0..50 {
        let p = rd(clube + 0xd7 + 4 * k);
        if p == 0 {
            continue;
        }
        let j = rd(p + 0x61);
        if j == 0 || !posicao(j) || lesao(rd(0xb63d28), p) == 2 {
            continue;
        }
        if core::ptr::read_unaligned(p as *const i32) >= *(0xae2364 as *const i32) {
            continue;
        }
        if core::ptr::read_unaligned((j + 5) as *const i16) as i32 > limit && extra(j) {
            n += 1;
        }
    }
    n
}

/// A reputação do clube (+0x80, curto com sinal).
unsafe fn club_reputation(clube: usize) -> i32 {
    core::ptr::read_unaligned((clube + 0x80) as *const i16) as i32
}

/// { 0x5493c0 } (cdecl, Database.cpp): `club_has_enough_hard_workers(clube)`: clube nulo: 0; senão 1 se mais de 4
/// jogadores (qualquer posição) passam do limite reputação/65 + 40 (de 40 a 140) com o atributo +0x44 (byte com
/// sinal; o mesmo que a 0x549850 usa) acima de 14. Só `al`. Só pelo olheiro (PLAYER_SEARCH::run_scout_day 0x7b3e20,
/// relatório de um clube).
unsafe extern "C" fn club_has_enough_hard_workers(clube: usize) -> u8 {
    if clube == 0 {
        return 0;
    }
    let limit = (club_reputation(clube) / 65 + 40).clamp(40, 0x8c);
    let n = count_squad_above_ability_with(clube, limit, |_| true, |j| *((j + 0x44) as *const i8) > 14);
    (n > 4) as u8 // esforcados (dbo)
}

unsafe extern "C" fn comparar_club_has_enough_hard_workers(clube: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u8>(%%I0%%)(clube);
    let n = club_has_enough_hard_workers(clube);
    PLACAR.registrar(r == n, || format!("club_has_enough_hard_workers({:x}) original={} rust={}", clube, r, n));
    r
}

/// { 0x549470 } (cdecl, Database.cpp): `club_has_fit_goalkeeper(clube)`: 1 assim que um dos 50 do elenco (+0xd7) é
/// pessoa com jogador (+0x61) goleiro (+0xf = 20), sem lesão grave (0x62c400 ≠ 2) e id < [0xae2364]; nenhum ou clube
/// nulo: 0. Sem limite de habilidade. Só `al`. Só pelo olheiro.
unsafe extern "C" fn club_has_fit_goalkeeper(clube: usize) -> u8 {
    if clube == 0 {
        return 0;
    }
    let rd = |a: usize| core::ptr::read_unaligned(a as *const usize);
    let lesao: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x62c400usize);
    for k in 0..50 {
        let p = rd(clube + 0xd7 + 4 * k);
        if p == 0 {
            continue;
        }
        let j = rd(p + 0x61);
        if j == 0 || *((j + 0xf) as *const u8) != 20 || lesao(rd(0xb63d28), p) == 2 {
            continue;
        }
        if core::ptr::read_unaligned(p as *const i32) < *(0xae2364 as *const i32) {
            return 1; // goleiro em condicoes (dbo)
        }
    }
    0
}

unsafe extern "C" fn comparar_club_has_fit_goalkeeper(clube: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u8>(%%I1%%)(clube);
    let n = club_has_fit_goalkeeper(clube);
    PLACAR.registrar(r == n, || format!("club_has_fit_goalkeeper({:x}) original={} rust={}", clube, r, n));
    r
}

/// { 0x549850 } (cdecl, Database.cpp): `club_has_ball_winning_midfielders(clube)`: clube nulo: 0; senão 1 se mais
/// de 1 meia (posição +0x14, +0x12 ou +0x13 = 20) sem lesão grave passa do limite reputação/75 + 25 (de 40 a 135)
/// com os atributos +0x44 e +0x3e (bytes com sinal) acima de reputação/500 + 5 (no máximo 14). Só `al`. Só pelo
/// olheiro.
unsafe extern "C" fn club_has_ball_winning_midfielders(clube: usize) -> u8 {
    if clube == 0 {
        return 0;
    }
    let rep = club_reputation(clube);
    let limit = (rep / 75 + 25).clamp(40, 0x87);
    let a = (rep / 500 + 5).min(14);
    let meia = |j: usize| *((j + 0x14) as *const u8) == 20 || *((j + 0x12) as *const u8) == 20 || *((j + 0x13) as *const u8) == 20;
    let n = count_squad_above_ability_with(clube, limit, meia, |j| *((j + 0x44) as *const i8) as i32 > a && *((j + 0x3e) as *const i8) as i32 > a);
    (n > 1) as u8 // meias que roubam (dbo)
}

unsafe extern "C" fn comparar_club_has_ball_winning_midfielders(clube: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u8>(%%I2%%)(clube);
    let n = club_has_ball_winning_midfielders(clube);
    PLACAR.registrar(r == n, || format!("club_has_ball_winning_midfielders({:x}) original={} rust={}", clube, r, n));
    r
}
