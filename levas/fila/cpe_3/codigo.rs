// ---------- %%LEVA%%a leva: a ordem pelo confronto guardado, a ordem das datas e a linha do clube na tabela da liga (league.cpp) ----------

unsafe fn cpe3_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpe3_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// { 0x683410 } (cdecl, league.cpp): `league_compare_table_rows_h2h(a, b)` (callback do qsort da 0x6827d0 na regra
/// 4 e da 0x6471a0 da Série B italiana): linha vazia empata; a comparação própria [0xb67a00] quando não dá 0 (chamada
/// de novo); os pontos (+0x19 da coluna [[0xb679fc]+0xad], b − a); os dois sem jogos (+0x11): o id dos clubes (a − b);
/// a posição do confronto guardada em +4 (b − a); por fim o nome dos clubes (+4, strcmp: −1, 0 ou 1).
unsafe extern "C" fn ordem_pela_posicao_do_confronto(a: usize, b: usize) -> i32 {
    let col = cpe3_d(cpe3_d(0xb679fc) + 0xad);
    if cpe3_d(a) == 0 || cpe3_d(b) == 0 {
        return 0;
    }
    if cpe3_d(0xb67a00) != 0 {
        let f: unsafe extern "C" fn(usize, usize) -> i32 = core::mem::transmute(cpe3_d(0xb67a00));
        if f(a, b) != 0 {
            let f: unsafe extern "C" fn(usize, usize) -> i32 = core::mem::transmute(cpe3_d(0xb67a00));
            return f(a, b);
        }
    }
    let o = col.wrapping_mul(14);
    let (pa, pb) = (cpe3_w(a + o + 0x19), cpe3_w(b + o + 0x19));
    if pa != pb {
        return pb as i32 - pa as i32;
    }
    if *((a + o + 0x11) as *const u8) == 0 && *((b + o + 0x11) as *const u8) == 0 {
        return (cpe3_d(cpe3_d(a)) as i32).wrapping_sub(cpe3_d(cpe3_d(b)) as i32);
    }
    let dif = *((b + 4) as *const i8) as i32 - *((a + 4) as *const i8) as i32; // a posição do confronto
    if dif != 0 {
        return dif;
    }
    let (mut x, mut y) = (cpe3_d(a) + 4, cpe3_d(b) + 4);
    loop {
        let (cx, cy) = (*(x as *const u8), *(y as *const u8));
        if cx != cy {
            return if cx < cy { -1 } else { 1 };
        }
        if cx == 0 {
            return 0;
        }
        x += 1;
        y += 1;
    }
}

unsafe extern "C" fn comparar_ordem_pela_posicao_do_confronto(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(%%I0%%)(a, b);
    let n = ordem_pela_posicao_do_confronto(a, b);
    PLACAR.registrar(r == n, || format!("ordem_pela_posicao_do_confronto({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// { 0x685cf0 } (cdecl, league.cpp): `league_compare_dates(a, b)` (callback do bsearch da 0x685640 na tabela de
/// datas da liga): o ano (+2, sem sinal) e depois o dia (+0, com sinal): −1, 0 ou 1.
unsafe extern "C" fn ordem_das_datas_da_liga(a: usize, b: usize) -> i32 {
    let (ya, yb) = (cpe3_w(a + 2) as u16, cpe3_w(b + 2) as u16);
    if ya < yb { // o ano antes
        return -1;
    }
    if ya > yb {
        return 1;
    }
    let (da, db) = (cpe3_w(a), cpe3_w(b));
    if da < db {
        -1
    } else {
        (da > db) as i32
    }
}

unsafe extern "C" fn comparar_ordem_das_datas_da_liga(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(%%I1%%)(a, b);
    let n = ordem_das_datas_da_liga(a, b);
    PLACAR.registrar(r == n, || format!("ordem_das_datas_da_liga({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// { 0x68ab70 } (thiscall, ret 8, league.cpp): `League::find_club_row(clube, linhas)`: a linha (0x3b bytes) do
/// clube entre as +0x3e (curto) de `linhas` (nula: a tabela da liga, +0xb1), ou 0.
unsafe extern "thiscall" fn linha_do_clube(liga: usize, clube: usize, linhas: usize) -> usize {
    let t = if linhas == 0 { cpe3_d(liga + 0xb1) } else { linhas };
    let n = cpe3_w(liga + 0x3e);
    let mut i: i16 = 0;
    if n > 0 {
        loop {
            let r = t.wrapping_add(0x3b * i as i32 as usize);
            if cpe3_d(r) == clube { // a linha do clube
                return r;
            }
            i = i.wrapping_add(1);
            if i >= n {
                break;
            }
        }
    }
    0
}

comparar_valor!(comparar_linha_do_clube, linha_do_clube, %%I2%%, (clube: usize, linhas: usize) -> usize);

/// { 0x68abd0 } (thiscall, ret 4, league.cpp; virtual +0x80 das ligas): `League::contains_club(clube)`: 1 se o
/// clube tem linha na tabela da liga (+0xb1, +0x3e linhas; a linha no endereço 0 conta como não), senão 0.
unsafe extern "thiscall" fn liga_tem_o_clube(liga: usize, clube: usize) -> u32 {
    let t = cpe3_d(liga + 0xb1);
    let n = cpe3_w(liga + 0x3e);
    let mut i: i16 = 0;
    if n > 0 {
        loop {
            let r = t.wrapping_add(0x3b * i as i32 as usize);
            if cpe3_d(r) == clube {
                return (r != 0) as u32; // a liga tem o clube
            }
            i = i.wrapping_add(1);
            if i >= n {
                break;
            }
        }
    }
    0
}

comparar_valor!(comparar_liga_tem_o_clube, liga_tem_o_clube, %%I3%%, (clube: usize) -> u32);
