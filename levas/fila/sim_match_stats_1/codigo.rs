// ---------- %%LEVA%%a leva: as estatísticas da partida: a estatística de liga do jogador, a entrada do jogo e o balde do registro (player_stats.cpp, match_stats.cpp) ----------

/// { 0x7c47b0 } (thiscall, ret 8, player_stats.cpp): a estatística de liga do jogador no clube, como
/// float (fild): campo 1 as partidas (palavra +0x20) e campo 2 os gols (palavra +0x22) do registro de
/// 39 bytes em [this+0x30] + id × 39. Jogador nulo ou outro campo: a original (caixa de erro).
unsafe extern "thiscall" fn get_club_league_stat(estat: usize, jogador: usize, campo: i8) -> f32 {
    if jogador == 0 || !(1..=2).contains(&campo) {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, i8) -> f32>(%%I0%%)(estat, jogador, campo);
    }
    let registro = (*((estat + 0x30) as *const usize) as isize + *(jogador as *const i32) as isize * 39) as usize;
    let deslocamento = if campo == 1 { 0x20 } else { 0x22 };
    core::ptr::read_unaligned((registro + deslocamento) as *const i16) as f32
}

/// { 0x729500 } (thiscall, ret 0x14, match_stats.cpp): a entrada `k` (12 bytes em [this]) tem os dois
/// clubes em qualquer ordem (+0/+4) e as datas (+8/+0xa, palavras) dadas? Índice −1: não; abaixo de −1
/// ou além de [this+0x1c]: a original (caixa de erro).
unsafe extern "thiscall" fn fixture_entry_matches(estat: usize, k: i32, a: i32, b: i32, data_a: i16, data_b: i16) -> i32 {
    if k < -1 || k > (*((estat + 0x1c) as *const i32)).wrapping_sub(1) {
        return original_fn::<unsafe extern "thiscall" fn(usize, i32, i32, i32, i16, i16) -> i32>(%%I1%%)(estat, k, a, b, data_a, data_b);
    }
    if k == -1 {
        return 0;
    }
    let e = (*(estat as *const usize) as isize + k as isize * 12) as usize;
    let d = |o: usize| *((e + o) as *const i32);
    let w = |o: usize| *((e + o) as *const i16);
    let clubes = (d(0) == a && d(4) == b) || (d(0) == b && d(4) == a);
    (clubes && w(8) == data_a && w(0xa) == data_b) as i32
}

/// { 0x729bd0 } (stdcall, ret 8, match_stats.cpp): o balde (0..3) do registro de estatística do jogo:
/// 0 se um dos clubes (+0x1c/+0x20) tem a ficha ([+0xcf]) entre as 16 últimas da base com +0x5f; 3 se o
/// registro tem +0x4c e [0xa5a390]; sem +0x40, 3 se o tipo (+0x3a) é 1, senão 2; com +0x40 e clube de
/// seleção (0x53b900): 0 se a palavra +0x80 do clube passa de 4999 ou de 0x157b, senão 2; senão pelo tipo
/// (tabela de saltos em 0x729d38) e as tabelas de bytes do exe em 0x729d64 e 0x729d94.
unsafe extern "stdcall" fn match_stats_get_record_bucket(jogo: usize, registro: usize) -> u8 {
    let limit = *(0xae2364 as *const i32);
    for o in [0x1c, 0x20] {
        let p = core::ptr::read_unaligned((*((jogo + o) as *const usize) + 0xcf) as *const usize);
        if p != 0 {
            let id = *(p as *const i32);
            if id >= limit.wrapping_sub(0x10) && id < limit && *((p + 0x5f) as *const u8) != 0 {
                return 0;
            }
        }
    }
    if *((registro + 0x4c) as *const u8) != 0 && *(0xa5a390 as *const u8) != 0 {
        return 3;
    }
    if *((jogo + 0x40) as *const u8) == 0 {
        return (*((jogo + 0x3a) as *const u8) == 1) as u8 + 2;
    }
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    if selecao(*((jogo + 0x1c) as *const usize)) != 0 {
        if *((jogo + 0x40) as *const u8) == 0 {
            return 2;
        }
        let v = core::ptr::read_unaligned((*((jogo + 0x1c) as *const usize) + 0x80) as *const i16);
        return if v as i32 * 2 >= 10000 || v >= 0x157c { 0 } else { 2 };
    }
    let w = |o: usize| core::ptr::read_unaligned((jogo + o) as *const i16) as i32;
    // ponytail: as duas tabelas de bytes dos saltos são lidas do próprio exe (o desvio só troca o começo da rotina)
    let tabela = |end: usize, i: i32| *((end + i as usize) as *const u8);
    match (*((jogo + 0x3a) as *const i8) as i32).wrapping_sub(1) {
        0 => 3,
        4 | 8 => 1,
        5 => {
            let c = w(0x30);
            if c as u32 > 0x9b {
                return 1;
            }
            match tabela(0x729d94, c) {
                0 => 0,
                1 => (core::ptr::read_unaligned((*((jogo + 0x14) as *const usize) + 0x69) as *const i16) <= 0xc) as u8,
                _ => 1,
            }
        }
        7 => {
            if (0x41f..=0x426).contains(&w(0x32)) {
                return 0;
            }
            let c = w(0x30) - 0x78;
            if c as u32 > 0x23 { 1 } else { (tabela(0x729d64, c) != 0) as u8 }
        }
        _ => {
            let n = *((*((jogo + 0x14) as *const usize) + 0x69) as *const i8);
            if n > 0xd { 1 } else { (n <= 6) as u8 + 2 }
        }
    }
}

unsafe extern "thiscall" fn comparar_get_club_league_stat(estat: usize, jogador: usize, campo: i8) -> f32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, i8) -> f32>(%%I0%%)(estat, jogador, campo);
    if jogador == 0 || !(1..=2).contains(&campo) {
        return r;
    }
    let n = get_club_league_stat(estat, jogador, campo);
    PLACAR.record(r.to_bits() == n.to_bits(), || format!("get_club_league_stat({}) original={} rust={}", campo, r, n));
    r
}

unsafe extern "thiscall" fn comparar_fixture_entry_matches(estat: usize, k: i32, a: i32, b: i32, data_a: i16, data_b: i16) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32, i32, i32, i16, i16) -> i32>(%%I1%%);
    if k < -1 || k > (*((estat + 0x1c) as *const i32)).wrapping_sub(1) {
        return o(estat, k, a, b, data_a, data_b);
    }
    let r = o(estat, k, a, b, data_a, data_b);
    let n = fixture_entry_matches(estat, k, a, b, data_a, data_b);
    PLACAR.record(r == n, || format!("fixture_entry_matches({}, {}, {}, {}, {}) original={} rust={}", k, a, b, data_a, data_b, r, n));
    r
}

unsafe extern "stdcall" fn comparar_match_stats_get_record_bucket(jogo: usize, registro: usize) -> u8 {
    let r = original_fn::<unsafe extern "stdcall" fn(usize, usize) -> u8>(%%I2%%)(jogo, registro);
    let n = match_stats_get_record_bucket(jogo, registro);
    PLACAR.record(r == n, || format!("match_stats_get_record_bucket original={} rust={}", r, n));
    r
}
