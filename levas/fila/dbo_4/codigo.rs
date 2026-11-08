// ---------- %%LEVA%%a leva: o olheiro pergunta se o clube tem atacante rápido e dois meias criativos, e a faixa de força do elenco (Database.cpp) ----------

/// Quantos dos 50 do elenco (+0xd7) são pessoa com jogador (+0x61) da `posicao`, sem lesão grave (INJURY 0x62c400
/// em [0xb63d28] ≠ 2), id (+0) < [0xae2364] (com sinal), habilidade atual (+5, curto com sinal) acima de
/// reputação/75 + 25 (de 40 a 135) e o `extra` com o limite de atributo reputação/500 + 5 (no máximo `teto`).
/// Reputação: +0x80 do clube, curto com sinal. Na ordem da original. Contador de 8 bits (no máximo 50).
unsafe fn count_squad_by_rep_thresholds(clube: usize, teto: i32, posicao: impl Fn(usize) -> bool, extra: impl Fn(usize, i32) -> bool) -> u8 {
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
        if j == 0 || !posicao(j) || lesao(rd(0xb63d28), p) == 2 {
            continue;
        }
        if core::ptr::read_unaligned(p as *const i32) >= *(0xae2364 as *const i32) {
            continue;
        }
        if core::ptr::read_unaligned((j + 5) as *const i16) as i32 > limit && extra(j, a) {
            n += 1;
        }
    }
    n
}

/// { 0x549770 } (cdecl, Database.cpp): `club_has_fast_striker(clube)`: clube nulo: 0; senão 1 se algum atacante
/// (posição +0x15 = 20) passa dos limites com o atributo +0x36 (a velocidade, byte com sinal) acima de
/// reputação/500 + 5 (no máximo 17). Só `al`. Só pelo olheiro (PLAYER_SEARCH::run_scout_day 0x7b3e20, relatório de
/// um clube).
unsafe extern "C" fn club_has_fast_striker(clube: usize) -> u8 {
    if clube == 0 {
        return 0;
    }
    let n = count_squad_by_rep_thresholds(clube, 17, |j| *((j + 0x15) as *const u8) == 20, |j, a| *((j + 0x36) as *const i8) as i32 > a);
    (n > 0) as u8 // atacante rapido (dbo)
}

unsafe extern "C" fn compare_club_has_fast_striker(clube: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u8>(%%I0%%)(clube);
    let n = club_has_fast_striker(clube);
    SCOREBOARD.record(r == n, || format!("club_has_fast_striker({:x}) original={} rust={}", clube, r, n));
    r
}

/// { 0x549940 } (cdecl, Database.cpp): `club_has_two_creative_midfielders(clube)`: clube nulo: 0; senão 1 se mais de
/// 1 meia (posição +0x14, +0x12 ou +0x13 = 20) passa dos limites com os atributos +0x43 e +0x37 (bytes com sinal)
/// acima de reputação/500 + 5 (no máximo 14). Só `al`. Só pelo olheiro.
unsafe extern "C" fn club_has_two_creative_midfielders(clube: usize) -> u8 {
    if clube == 0 {
        return 0;
    }
    let meia = |j: usize| *((j + 0x14) as *const u8) == 20 || *((j + 0x12) as *const u8) == 20 || *((j + 0x13) as *const u8) == 20;
    let n = count_squad_by_rep_thresholds(clube, 14, meia, |j, a| *((j + 0x43) as *const i8) as i32 > a && *((j + 0x37) as *const i8) as i32 > a);
    (n > 1) as u8 // meias criativos (dbo)
}

unsafe extern "C" fn compare_club_has_two_creative_midfielders(clube: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u8>(%%I1%%)(clube);
    let n = club_has_two_creative_midfielders(clube);
    SCOREBOARD.record(r == n, || format!("club_has_two_creative_midfielders({:x}) original={} rust={}", clube, r, n));
    r
}

/// { 0x549a30 } (cdecl, Database.cpp): `club_squad_strength_band(clube, outro)`: a faixa de força do elenco, 0..6.
/// Clube nulo: 3. Junta os do elenco (+0xd7) com jogador (+0x61) e id < [0xae2364] (lido uma vez) num vetor local de
/// 50, ordena pelo qsort da CRT (0x945333) com a comparação 0x549c30 (pelo endereço do jogo) e soma a habilidade atual
/// (+5 do jogador) dos primeiros min(n − 1, 16). Sem nenhum somado: pela reputação (+0x80; > 7250: 6, > 5750: 5,
/// > 4750: 4, > 3250: 3, > 2250: 2, senão 1); senão pela média (> 155: 6, > 135: 5, > 110: 4, > 85: 3, > 60: 2,
/// > 40: 1, senão 0). Com `outro` (o clube do olheiro): reputação dele > 7250 tira 2 se a do clube < 4250 e a faixa
/// > 2, e mais 1 se a do clube < 7250 e a faixa > 0; senão, reputação dele < 4250 soma 1 se a faixa < 5. Na mesma
/// competição (+0x57 igual), a diferença de reputação (16 bits) > 1250 tira 1 (faixa > 0, byte +4 da tabela
/// [0xae2d14] de 6 bytes por id: do outro ≥ 0 e do clube ≤ 0); < 1250 soma 1 (faixa < 5, do clube ≥ 0 e do outro
/// ≤ 0). No fim, entre 0 e 6. Só `al`. Só pelo olheiro.
unsafe extern "C" fn club_squad_strength_band(clube: usize, outro: usize) -> u8 {
    if clube == 0 {
        return 3;
    }
    let rd = |a: usize| core::ptr::read_unaligned(a as *const usize);
    let rd16 = |a: usize| core::ptr::read_unaligned(a as *const i16);
    let id = |a: usize| core::ptr::read_unaligned(a as *const i32);
    let total = *(0xae2364 as *const i32);
    let mut v = [0usize; 50];
    let mut n = 0usize;
    for k in 0..50 {
        let p = rd(clube + 0xd7 + 4 * k);
        if p != 0 && rd(p + 0x61) != 0 && id(p) < total {
            v[n] = p;
            n += 1;
        }
    }
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    ordenar(v.as_mut_ptr() as usize, n as u32, 4, 0x549c30); // os melhores primeiro (dbo)
    let m = (n as i32 - 1).clamp(0, 16) as usize;
    let mut soma = 0i32;
    for &p in &v[..m] {
        soma = soma.wrapping_add(rd16(rd(p + 0x61) + 5) as i32);
    }
    let rep = rd16(clube + 0x80);
    let mut f: i8 = if m == 0 {
        match rep {
            r if r > 0x1c52 => 6,
            r if r > 0x1676 => 5,
            r if r > 0x128e => 4,
            r if r > 0xcb2 => 3,
            r => (r > 0x8ca) as i8 + 1,
        }
    } else {
        match soma / m as i32 {
            x if x > 0x9b => 6,
            x if x > 0x87 => 5,
            x if x > 0x6e => 4,
            x if x > 0x55 => 3,
            x if x > 0x3c => 2,
            x => (x > 0x28) as i8,
        }
    };
    if outro != 0 {
        let ro = rd16(outro + 0x80);
        if ro > 0x1c52 {
            if rep < 0x109a && f > 2 {
                f -= 2;
            }
            if rep < 0x1c52 && f > 0 {
                f -= 1;
            }
        } else if ro < 0x109a && f < 5 {
            f += 1; // o olheiro de clube pequeno ve o clube mais forte (dbo)
        }
        if rd(outro + 0x57) == rd(clube + 0x57) {
            let d = ro.wrapping_sub(rep);
            let tab = rd(0xae2d14);
            let marca = |c: usize| *(tab.wrapping_add((id(c).wrapping_mul(6) as usize).wrapping_add(4)) as *const i8);
            if d > 0x4e2 && f > 0 && marca(outro) >= 0 && marca(clube) <= 0 {
                f -= 1;
            } else if d < 0x4e2 && f < 5 && marca(clube) >= 0 && marca(outro) <= 0 {
                f += 1;
            }
        }
    }
    f.clamp(0, 6) as u8
}

unsafe extern "C" fn compare_club_squad_strength_band(clube: usize, outro: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> u8>(%%I2%%)(clube, outro);
    let n = club_squad_strength_band(clube, outro);
    SCOREBOARD.record(r == n, || format!("club_squad_strength_band({:x}, {:x}) original={} rust={}", clube, outro, r, n));
    r
}
