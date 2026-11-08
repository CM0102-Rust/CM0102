// ---------- %%LEVA%%a leva: a ordem das linhas da tabela, a grade de turnos da liga e a margem do último dia (league.cpp) ----------

/// { 0x683210 } (cdecl, league.cpp): comparação do qsort das linhas da tabela (`a` e `b`: [+0] clube).
/// Liga corrente em [0xb679fc], a coluna da tabela em +0xad (14 bytes por coluna nas linhas). Linha
/// sem clube: 0. Com a comparação particular em [0xb67a00] e resposta dela não nula: ela de novo. Senão
/// pontos (+0x19) decrescentes; sem jogos (+0x11) dos dois, pelo id do clube. Depois os quatro
/// critérios da liga (+0xc5): 1 saldo, 2 gols pró, 3 byte +0x12, 4 byte +4, 5 palavra +0x31 (na
/// coluna 0, critério 6 = byte +4 e fim); na coluna 0 com os dois no número de jogos da liga (+0xa7), o
/// id em +0x69 dos clubes; por fim o id do clube.
unsafe extern "C" fn league_compare_table_rows(a: usize, b: usize) -> i32 {
    let liga = *(0xb679fc as *const usize);
    let coluna = core::ptr::read_unaligned((liga + 0xad) as *const i32);
    if *(a as *const usize) == 0 || *(b as *const usize) == 0 {
        return 0;
    }
    let propria = *(0xb67a00 as *const usize);
    if propria != 0 {
        let f: unsafe extern "C" fn(usize, usize) -> i32 = core::mem::transmute(propria);
        if f(a, b) != 0 {
            let f: unsafe extern "C" fn(usize, usize) -> i32 = core::mem::transmute(*(0xb67a00 as *const usize));
            return f(a, b);
        }
    }
    let c = (coluna as usize).wrapping_mul(14);
    let w = |r: usize, o: usize| core::ptr::read_unaligned(r.wrapping_add(c).wrapping_add(o) as *const i16) as i32;
    let y = |r: usize, o: usize| *(r.wrapping_add(c).wrapping_add(o) as *const i8) as i32;
    let (pa, pb) = (w(a, 0x19), w(b, 0x19));
    if pa != pb {
        return pb - pa; // pontos
    }
    let id = |r: usize| *(*(r as *const usize) as *const i32);
    if y(a, 0x11) == 0 && y(b, 0x11) == 0 {
        return id(a).wrapping_sub(id(b));
    }
    for k in 0..4usize {
        let criterio = *((liga + 0xc5 + k) as *const i8);
        if coluna == 0 && criterio == 6 {
            return *((a + 4) as *const i8) as i32 - *((b + 4) as *const i8) as i32;
        }
        if criterio == 0 {
            break;
        }
        let (va, vb) = match criterio {
            1 => (w(a, 0x15) - w(a, 0x17), w(b, 0x15) - w(b, 0x17)),
            2 => (w(a, 0x15), w(b, 0x15)),
            3 => (y(a, 0x12), y(b, 0x12)),
            4 => (*((a + 4) as *const i8) as i32, *((b + 4) as *const i8) as i32),
            5 => (core::ptr::read_unaligned((a + 0x31) as *const i16) as i32, core::ptr::read_unaligned((b + 0x31) as *const i16) as i32),
            _ => continue,
        };
        if va != vb {
            return vb - va;
        }
    }
    if coluna == 0 {
        let jogos = core::ptr::read_unaligned((liga + 0xa7) as *const i16);
        if *((a + 0x11) as *const i8) as i16 == jogos && *((b + 0x11) as *const i8) as i16 == jogos {
            let xb = core::ptr::read_unaligned((*(b as *const usize) + 0x69) as *const usize);
            let xa = core::ptr::read_unaligned((*(a as *const usize) + 0x69) as *const usize);
            if xb != 0 && xa != 0 {
                let d = (*(xb as *const i32)).wrapping_sub(*(xa as *const i32));
                if d != 0 {
                    return d;
                }
            }
        }
    }
    id(a).wrapping_sub(id(b))
}

/// { 0x6844b0 } (stdcall, ret 8, league.cpp): a grade de turnos da liga de `n` clubes: `grade[1..=n]`
/// são linhas de inteiros (colunas 1..n−1 = rodadas) com o adversário de cada clube na rodada (positivo
/// em casa, negativo fora). Zera tudo; o clube `n` enfrenta 1..n−1 numa rodada que anda de 2 em 2
/// (casa para a primeira metade); depois cada clube k de n−1 até 2 preenche as vagas vazias com os
/// clubes 1..n, alternando o mando a partir da paridade de n−1−k. Devolve 1.
unsafe extern "stdcall" fn league_build_round_robin_grid(grade: *const *mut i32, n: i32) -> u32 {
    let celula = |i: i32, j: i32| (*grade.offset(i as isize)).offset(j as isize);
    for i in 1..=n {
        for j in 1..n {
            *celula(i, j) = 0;
        }
    }
    let metade = n / 2;
    let mut col = n - 1;
    for i in 1..n {
        if i <= metade {
            *celula(i, col) = n;
            *celula(n, col) = -i;
        } else {
            *celula(i, col) = -n;
            *celula(n, col) = i;
        }
        col = if col == n - 2 { 1 } else if col == n - 1 { 2 } else { col + 2 };
    }
    let mut k = n - 1;
    while k >= 2 {
        let menos_k = -k;
        let mut fora = (n - 1 + menos_k) % 2;
        let mut col = k - 1;
        let mut contra = -1;
        for j in 1..=n {
            let c = celula(j, col);
            if *c == 0 {
                if fora != 0 {
                    *c = k; // vaga preenchida
                    *celula(k, col) = contra;
                } else {
                    *c = menos_k;
                    *celula(k, col) = j;
                }
                fora = 1 - fora;
            }
            col = if col == n - 1 { 1 } else { col + 1 };
            contra -= 1;
        }
        k -= 1;
    }
    1
}

/// A linha `i` (59 bytes) da tabela [+0xb1] da liga.
unsafe fn league_row(liga: usize, i: i32) -> usize {
    (*((liga + 0xb1) as *const usize)).wrapping_add((i as isize * 59) as usize)
}

/// A margem de gols (+1, mínimo 1, senão 0xff) que falta para alcançar a linha `alvo` pelo critério da
/// liga (+0xc5): 1 saldo (+0x15 − +0x17), 2 gols pró (+0x15).
unsafe fn goal_margin_to_reach(liga: usize, linha: usize, alvo: usize) -> u8 {
    let b = |r: usize, o: usize| *((r + o) as *const u8);
    let c = if *((liga + 0xc5) as *const u8) == 1 {
        b(alvo, 0x15).wrapping_sub(b(alvo, 0x17)).wrapping_sub(b(linha, 0x15)).wrapping_add(b(linha, 0x17))
    } else {
        b(alvo, 0x15).wrapping_sub(b(linha, 0x15))
    }
    .wrapping_add(1);
    if (c as i8) < 1 { 0xff } else { c }
}

/// { 0x68a4d0 } (thiscall, ret 8, league.cpp): no último dia (jogos +0x11 ≥ rodadas da liga +0xa9 − 1)
/// e linha sem marca (+0x37 = 0xff), a margem de gols que o clube precisa para o título, as vagas de
/// cima (+0xbe/+0xbf) ou fugir das de baixo (+0xc0/+0xc1); 0xff se nenhuma. Com pontos iguais a quem
/// está à frente sem margem possível, grava 0x14 em `saida` (decisivo) e devolve 0 ou 0xff. Só o al é
/// devolvido (quem chama usa al).
unsafe extern "thiscall" fn final_day_margin_needed(liga: usize, linha: usize, saida: *mut u8) -> u8 {
    let p = |r: usize| core::ptr::read_unaligned((r + 0x19) as *const i16) as i32;
    let extra = *((liga + 0xc2) as *const i8) as i32;
    let pos = *((linha + 4) as *const i8);
    let clubes = core::ptr::read_unaligned((liga + 0x3e) as *const i16);
    let tabela = *((liga + 0xb1) as *const usize);
    let criterio = *((liga + 0xc5) as *const u8);
    if (*((linha + 0x11) as *const i8) as i32) < core::ptr::read_unaligned((liga + 0xa9) as *const i16) as i32 - 1 || *((linha + 0x37) as *const u8) != 0xff {
        return 0xff; // fora do último dia
    }
    // a linha `alvo` à frente, sem marca: pontos + extra acima dela, 0xff; iguais (extra 0), a margem
    let ver = |alvo: usize| -> Option<u8> {
        if p(linha) + extra > p(alvo) {
            return Some(0xff);
        }
        if extra == 0 && (criterio == 1 || criterio == 2) {
            return Some(goal_margin_to_reach(liga, linha, alvo));
        }
        None
    };
    // a primeira linha sem marca de `de` até `ate` (exclusive), com pontos + extra contra os da `linha`
    let decidir = |de: i16, ate: i16| -> Option<u8> {
        let mut i = de;
        while i < ate {
            let r = league_row(liga, i as i32);
            if *((r + 0x37) as *const u8) == 0xff {
                let v = p(r) + extra;
                if v > p(linha) {
                    *saida = 0x14;
                    return Some(0xff);
                }
                if v != p(linha) {
                    return Some(0xff);
                }
                *saida = 0x14;
                return Some(0);
            }
            i += 1;
        }
        None
    };
    if *((liga + 0xd9) as *const u8) & 1 != 0 && *((tabela + 0x37) as *const u8) != 5 {
        if pos > 0 {
            let v = p(linha) + extra;
            if v > p(tabela) {
                return 0xff;
            }
            if v == p(tabela) && (criterio == 1 || criterio == 2) {
                return goal_margin_to_reach(liga, linha, tabela);
            }
        } else if clubes > 1 {
            let v = core::ptr::read_unaligned((tabela + 0x54) as *const i16) as i32 + extra;
            if v > p(tabela) {
                *saida = 0x14;
                return 0xff;
            }
            if v != p(tabela) {
                return 0xff;
            }
            *saida = 0x14;
            return 0;
        }
    }
    let (s1, s2) = (*((liga + 0xbe) as *const u8), *((liga + 0xbf) as *const u8));
    if s1 != 0 || s2 != 0 {
        let vagas = s2.wrapping_add(s1) as i8;
        if pos < vagas {
            if let Some(r) = decidir(vagas as i16, clubes) {
                return r;
            }
        } else {
            let mut i = 0i16;
            while i < vagas as i16 {
                let r = league_row(liga, i as i32);
                if *((r + 0x37) as *const u8) == 0xff {
                    if let Some(m) = ver(r) {
                        return m;
                    }
                }
                i += 1;
            }
        }
    }
    let (d1, d2) = (*((liga + 0xc1) as *const u8), *((liga + 0xc0) as *const u8));
    if d1 == 0 && d2 == 0 {
        return 0xff;
    }
    let descem = d2.wrapping_add(d1) as i8;
    let limit = clubes as i32 - descem as i32;
    if (pos as i32) < limit {
        return decidir(clubes.wrapping_sub(descem as i16), clubes).unwrap_or(0xff);
    }
    let mut i = 0i16;
    while (i as i32) < limit {
        let r = league_row(liga, i as i32);
        if *((r + 0x37) as *const u8) == 0xff {
            if let Some(m) = ver(r) {
                return m;
            }
        }
        i += 1;
    }
    0xff
}

unsafe extern "C" fn comparar_league_compare_table_rows(a: usize, b: usize) -> i32 {
    // a comparação particular de [0xb67a00] (se houver) roda nos dois lados: só lê as linhas
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(%%I0%%)(a, b);
    let n = league_compare_table_rows(a, b);
    SCOREBOARD.record(r == n, || format!("league_compare_table_rows original={} rust={}", r, n));
    r
}

unsafe extern "stdcall" fn comparar_league_build_round_robin_grid(grade: *const *mut i32, n: i32) -> u32 {
    let o = original_fn::<unsafe extern "stdcall" fn(*const *mut i32, i32) -> u32>(%%I1%%);
    let regioes: Vec<(usize, usize)> = (1..=n.max(0)).map(|i| (*grade.offset(i as isize) as usize + 4, (n.max(1) as usize - 1) * 4)).collect();
    let (r, s, igual, mo, mr) = crate::comparison::side_by_side(&regioes, || o(grade, n), || league_build_round_robin_grid(grade, n));
    SCOREBOARD.record(igual && r == s, || format!("league_build_round_robin_grid({}) {:x?} x {:x?}", n, mo, mr));
    r
}

unsafe extern "thiscall" fn comparar_final_day_margin_needed(liga: usize, linha: usize, saida: *mut u8) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut u8) -> u8>(%%I2%%);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(saida as usize, 1)], || o(liga, linha, saida),
        || final_day_margin_needed(liga, linha, saida));
    SCOREBOARD.record(igual && r == n, || format!("final_day_margin_needed original={:#x} rust={:#x} {:x?} x {:x?}", r, n, mo, mr));
    r
}
