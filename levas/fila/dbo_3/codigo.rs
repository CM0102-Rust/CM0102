// ---------- %%LEVA%%a leva: o olheiro pergunta se faltam ao clube atacantes e zagueiros bons no alto e zagueiros rápidos (Database.cpp) ----------

/// Quantos dos 50 do elenco (+0xd7) são pessoa com jogador (+0x61) na `posicao` (byte = 20), sem lesão grave
/// (INJURY 0x62c400 em [0xb63d28] ≠ 2), id (+0) < [0xae2364] (com sinal), habilidade atual (+5, curto com sinal)
/// acima de reputação/75 + 25 (de 40 a 135) e o `atributo` (byte com sinal) acima de reputação/500 + 5 (no máximo
/// `teto`). Reputação: +0x80 do clube, curto com sinal. Na ordem da original. Contador de 8 bits (no máximo 50).
unsafe fn count_natural_at_position(clube: usize, posicao: usize, atributo: usize, teto: i32) -> u8 {
    let rd = |a: usize| core::ptr::read_unaligned(a as *const usize);
    let lesao: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x62c400usize);
    let rep = core::ptr::read_unaligned((clube + 0x80) as *const i16) as i32;
    let limit = (rep / 75 + 25).clamp(40, 0x87);
    let a = (rep / 500 + 5).min(teto);
    let mut n = 0u8;
    for k in 0..50 {
        let p = rd(clube + 0xd7 + 4 * k);
        if p == 0 {
            continue;
        }
        let j = rd(p + 0x61);
        if j == 0 || *((j + posicao) as *const u8) != 20 || lesao(rd(0xb63d28), p) == 2 {
            continue;
        }
        if core::ptr::read_unaligned(p as *const i32) >= *(0xae2364 as *const i32) {
            continue;
        }
        if core::ptr::read_unaligned((j + 5) as *const i16) as i32 > limit && *((j + atributo) as *const i8) as i32 > a {
            n += 1;
        }
    }
    n
}

/// { 0x5494d0 } (cdecl, Database.cpp): `club_lacks_aerial_strikers(clube)`: clube nulo: 0; senão 1 se no máximo 1
/// atacante (posição +0x15) passa dos limites com o atributo +0x2e (o jogo pelo alto) acima de reputação/500 + 5 (no
/// máximo 13). Só `al`. Só pelo olheiro (PLAYER_SEARCH::run_scout_day 0x7b3e20, relatório de um clube).
unsafe extern "C" fn club_lacks_aerial_strikers(clube: usize) -> u8 {
    if clube == 0 {
        return 0;
    }
    (count_natural_at_position(clube, 0x15, 0x2e, 13) <= 1) as u8 // atacantes pelo alto (dbo)
}

unsafe extern "C" fn comparar_club_lacks_aerial_strikers(clube: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u8>(%%I0%%)(clube);
    let n = club_lacks_aerial_strikers(clube);
    PLACAR.registrar(r == n, || format!("club_lacks_aerial_strikers({:x}) original={} rust={}", clube, r, n));
    r
}

/// { 0x5495b0 } (cdecl, Database.cpp): `club_lacks_aerial_defenders(clube)`: clube nulo: 0; senão 1 se no máximo 1
/// zagueiro (posição +0x11) passa dos limites com o atributo +0x2e acima de reputação/500 + 5 (no máximo 13). Só
/// `al`. Só pelo olheiro.
unsafe extern "C" fn club_lacks_aerial_defenders(clube: usize) -> u8 {
    if clube == 0 {
        return 0;
    }
    (count_natural_at_position(clube, 0x11, 0x2e, 13) <= 1) as u8 // zagueiros pelo alto (dbo)
}

unsafe extern "C" fn comparar_club_lacks_aerial_defenders(clube: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u8>(%%I1%%)(clube);
    let n = club_lacks_aerial_defenders(clube);
    PLACAR.registrar(r == n, || format!("club_lacks_aerial_defenders({:x}) original={} rust={}", clube, r, n));
    r
}

/// { 0x549690 } (cdecl, Database.cpp): `club_lacks_fast_defenders(clube)`: clube nulo: 0; senão 1 se no máximo 1
/// zagueiro (posição +0x11) passa dos limites com o atributo +0x36 (a velocidade) acima de reputação/500 + 5 (no
/// máximo 10). Só `al`. Só pelo olheiro.
unsafe extern "C" fn club_lacks_fast_defenders(clube: usize) -> u8 {
    if clube == 0 {
        return 0;
    }
    (count_natural_at_position(clube, 0x11, 0x36, 10) <= 1) as u8 // zagueiros rapidos (dbo)
}

unsafe extern "C" fn comparar_club_lacks_fast_defenders(clube: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u8>(%%I2%%)(clube);
    let n = club_lacks_fast_defenders(clube);
    PLACAR.registrar(r == n, || format!("club_lacks_fast_defenders({:x}) original={} rust={}", clube, r, n));
    r
}
