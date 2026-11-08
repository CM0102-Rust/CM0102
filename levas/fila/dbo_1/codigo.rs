// ---------- %%LEVA%%a leva: o olheiro pergunta se o clube tem defensores, meias e atacantes que bastem (Database.cpp) ----------

/// Quantos dos 50 do elenco (+0xd7) contam para as três consultas da força do elenco pelo limite /65: pessoa não
/// nula com jogador (+0x61) da `posicao`, sem lesão grave (INJURY 0x62c400 em [0xb63d28] ≠ 2), id (+0) < [0xae2364]
/// (com sinal) e habilidade atual (+5 do jogador, curto com sinal) acima do `limite`. A ordem dos testes é a da
/// original (a 0x62c400 só é chamada para os da posição). Contador de 8 bits (no máximo 50).
unsafe fn count_squad_above_ability(clube: usize, limit: i32, posicao: impl Fn(usize) -> bool) -> u8 {
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
        if core::ptr::read_unaligned((j + 5) as *const i16) as i32 > limit {
            n += 1;
        }
    }
    n
}

/// O limite de habilidade das três: reputação do clube (+0x80, curto com sinal) / 65 + 40, entre 40 e `teto`.
unsafe fn squad_ability_threshold(clube: usize, teto: i32) -> i32 {
    (core::ptr::read_unaligned((clube + 0x80) as *const i16) as i32 / 65 + 40).clamp(40, teto)
}

/// A média de gols por jogo do clube na competição (+0x57): a estatística do clube (COMP_STATS 0x4a4850, thiscall
/// no +8 da competição [[0xadadfc] + 4·id]); `None` sem competição, sem estatística ou sem jogos (+4); senão o byte
/// `campo` / jogos (inteiros sem sinal). A 0x4a4850 ordena a tabela da competição na primeira chamada (marca +0x18)
/// e guarda a competição em [0xadae10]: na comparação a original vem antes e a cópia acha a tabela já ordenada.
unsafe fn club_goals_per_game(clube: usize, campo: usize) -> Option<i32> {
    let rd = |a: usize| core::ptr::read_unaligned(a as *const usize);
    let comp = rd(clube + 0x57);
    if comp == 0 {
        return None;
    }
    let c = rd(rd(0xadadfc).wrapping_add(rd(comp).wrapping_mul(4)));
    if c == 0 || rd(c + 8) == 0 {
        return None;
    }
    let estatistica: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
    let e = estatistica(rd(c + 8), clube);
    if e == 0 {
        return None;
    }
    let jogos = *((e + 4) as *const u8) as i32;
    if jogos == 0 {
        return None;
    }
    Some(*((e + campo) as *const u8) as i32 / jogos)
}

/// { 0x549100 } (cdecl, Database.cpp): `club_has_enough_defenders(clube)`: clube nulo: 0. Se o clube toma mais de
/// 1,35 gol por jogo ([0x969588]; gols sofridos +0x6a da estatística / jogos): 0. Senão, 1 se mais de 2 zagueiros
/// (posição +0x11 = 20) passam do limite /65 (teto 145). Só `al`. Só pelo olheiro (PLAYER_SEARCH::run_scout_day
/// 0x7b3e20, relatório de um clube).
unsafe extern "C" fn club_has_enough_defenders(clube: usize) -> u8 {
    if clube == 0 {
        return 0;
    }
    if let Some(v) = club_goals_per_game(clube, 0x6a) {
        if v as f64 > *(0x969588 as *const f64) {
            return 0; // a defesa toma gols demais
        }
    }
    let limit = squad_ability_threshold(clube, 0x91);
    (count_squad_above_ability(clube, limit, |j| *((j + 0x11) as *const u8) == 20) > 2) as u8 // zagueiros (dbo)
}

unsafe extern "C" fn comparar_club_has_enough_defenders(clube: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u8>(%%I0%%)(clube);
    let n = club_has_enough_defenders(clube);
    SCOREBOARD.record(r == n, || format!("club_has_enough_defenders({:x}) original={} rust={}", clube, r, n));
    r
}

/// { 0x549200 } (cdecl, Database.cpp): `club_has_enough_midfielders(clube)`: clube nulo: 0; senão 1 se mais de 2
/// meias (posição +0x14, +0x12 ou +0x13 = 20, nessa ordem) passam do limite /65 (teto 145). Só `al`. Só pelo olheiro.
unsafe extern "C" fn club_has_enough_midfielders(clube: usize) -> u8 {
    if clube == 0 {
        return 0;
    }
    let limit = squad_ability_threshold(clube, 0x91);
    let meia = |j: usize| *((j + 0x14) as *const u8) == 20 || *((j + 0x12) as *const u8) == 20 || *((j + 0x13) as *const u8) == 20;
    (count_squad_above_ability(clube, limit, meia) > 2) as u8 // meias (dbo)
}

unsafe extern "C" fn comparar_club_has_enough_midfielders(clube: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u8>(%%I1%%)(clube);
    let n = club_has_enough_midfielders(clube);
    SCOREBOARD.record(r == n, || format!("club_has_enough_midfielders({:x}) original={} rust={}", clube, r, n));
    r
}

/// { 0x5492c0 } (cdecl, Database.cpp): `club_has_enough_strikers(clube)`: clube nulo: 0. Se o clube faz menos de
/// 1,5 gol por jogo ([0x968940]; gols marcados +0x69 da estatística / jogos): 0. Senão, 1 se mais de 1 atacante
/// (posição +0x15 = 20) passa do limite /65 (teto 150). Só `al`. Só pelo olheiro.
unsafe extern "C" fn club_has_enough_strikers(clube: usize) -> u8 {
    if clube == 0 {
        return 0;
    }
    if let Some(v) = club_goals_per_game(clube, 0x69) {
        if (v as f64) < *(0x968940 as *const f64) {
            return 0; // o ataque faz poucos gols
        }
    }
    let limit = squad_ability_threshold(clube, 0x96);
    (count_squad_above_ability(clube, limit, |j| *((j + 0x15) as *const u8) == 20) > 1) as u8 // atacantes (dbo)
}

unsafe extern "C" fn comparar_club_has_enough_strikers(clube: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u8>(%%I2%%)(clube);
    let n = club_has_enough_strikers(clube);
    SCOREBOARD.record(r == n, || format!("club_has_enough_strikers({:x}) original={} rust={}", clube, r, n));
    r
}
