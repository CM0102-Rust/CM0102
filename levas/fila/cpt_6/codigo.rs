// ---------- %%LEVA%%a leva: os coeficientes das nações (o bônus de dezembro e a história de cinco anos), o dia das aposentadorias e a soma presa de dois bytes (game.cpp) ----------

unsafe fn cpt6_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpt6_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpt6_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpt6_f(a: usize) -> f64 {
    core::ptr::read_unaligned(a as *const f64)
}

unsafe fn cpt6_pf(a: usize, v: f64) {
    core::ptr::write_unaligned(a as *mut f64, v);
}

/// A história do coeficiente da nação: +0xb0, +0xb8, +0xc0, +0xc8, +0xd0 (cinco anos) e +0xd8 (o ano corrente).
unsafe fn nation_coefficient_history(na: usize) -> [f64; 6] {
    let mut h = [0f64; 6];
    for (k, x) in h.iter_mut().enumerate() {
        *x = cpt6_f(na + 0xb0 + k * 8);
    }
    h
}

/// O mês da data do jogo (0x54cfb0 sobre 0xae2c90, o al com sinal).
unsafe fn current_month() -> i32 {
    let mes: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x54cfb0usize);
    mes(0xae2c90) as u8 as i8 as i32
}

/// Dezembro na história `h` da nação `na` (`marca` = +0x121): com marca < 12, o ano corrente ganha
/// reputação (+0x8e) × 0,05 × (12 − marca) × 3; a história anda um ano, o ano corrente e a marca zeram.
unsafe fn roll_coefficient_year(na: usize, h: &mut [f64; 6], marca: &mut u8) {
    crate::comparison::branch("cpt_6 dezembro");
    if (*marca as i8) < 0xc {
        crate::comparison::branch("cpt_6 bonus de dezembro");
        h[5] = cpt6_w(na + 0x8e) as i32 as f64 * read_f64_at(0x967888) * (0xc - *marca as i8 as i32) as f64 * read_f64_at(0x968930) + h[5]; // cpt6: o bônus de dezembro
    }
    h[0] = h[1];
    h[1] = h[2];
    h[2] = h[3];
    h[3] = h[4];
    h[4] = h[5];
    h[5] = 0.0; // cpt6: o ano corrente zerado
    *marca = 0;
}

/// { 0x5d5f20 } (cdecl, game.cpp): `game_update_nation_coefficients()`, todo dia pelo laço do dia do jogo (0x5cda10).
/// Na primeira vez ([0x9cd218]), a história de cada nação (+0xb0..+0xd0) recebe o coeficiente de +0xa8. No dia do
/// mês da tabela 0x9cd200 (palavra pelo mês 0x54cfb0): um vetor (malloc de 12 bytes por nação: a nação e a soma
/// 0,6·c0 + 0,4·b8 + 0,2·b0 + 0,8·c8 + d8 + d0), ordenado (qsort 0x945333 com 0x4afba0) e solto; no dia de dezembro
/// ([0x9cd216]) cada nação passa também pelo bônus e pelo deslocamento da história (`roll_coefficient_year`).
unsafe extern "C" fn game_update_nation_coefficients() {
    let malloc: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x944e46usize);
    let qsort: unsafe extern "C" fn(usize, usize, usize, usize) = core::mem::transmute(0x945333usize);
    let free: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    if cpt6_b(0x9cd218) != 0 {
        if (cpt6_d(0xae2350) as i32) > 0 {
            let mut p = cpt6_d(0xae23a8) + 0xd0;
            let mut i = 0i16;
            loop {
                let v = cpt6_f(p - 0x28);
                for k in 0..5usize {
                    cpt6_pf(p - k * 8, v);
                }
                i = i.wrapping_add(1);
                p += 0x122;
                if i as i32 >= cpt6_d(0xae2350) as i32 {
                    break;
                }
            }
        }
        *(0x9cd218 as *mut u8) = 0;
    }
    let hoje = cpt6_d(0xae2c90) as i16;
    if cpt6_w(0x9cd200 + (current_month() * 2) as usize) != hoje {
        return;
    }
    let vetor = malloc(cpt6_d(0xae2350).wrapping_mul(0xc));
    if vetor == 0 {
        show_error_at(0x9cd2a4, 0x156d);
        return;
    }
    let dez = cpt6_w(0x9cd216) == cpt6_w(0xae2c90); // cpt6: o dia de dezembro
    let mut n = cpt6_d(0xae2350) as i32;
    if n > 0 {
        let mut na = cpt6_d(0xae23a8);
        let mut i = 0i16;
        loop {
            let e = vetor + i as i32 as usize * 0xc;
            *(e as *mut usize) = na;
            let s = cpt6_f(na + 0xc0) * read_f64_at(0x968e48) + cpt6_f(na + 0xb8) * read_f64_at(0x968e00) + cpt6_f(na + 0xb0) * read_f64_at(0x968918)
                + cpt6_f(na + 0xc8) * read_f64_at(0x9689b0) + cpt6_f(na + 0xd8) + cpt6_f(na + 0xd0);
            cpt6_pf(e + 4, s);
            if dez {
                let mut h = nation_coefficient_history(na);
                let mut marca = cpt6_b(na + 0x121);
                roll_coefficient_year(na, &mut h, &mut marca);
                for (k, x) in h.iter().enumerate() {
                    cpt6_pf(na + 0xb0 + k * 8, *x);
                }
                *((na + 0x121) as *mut u8) = marca;
            }
            i = i.wrapping_add(1);
            na += 0x122;
            n = cpt6_d(0xae2350) as i32;
            if i as i32 >= n {
                break;
            }
        }
    }
    qsort(vetor, n as usize, 0xc, 0x4afba0);
    free(vetor);
}

/// Comparador: a original aloca e solta o vetor (e o ordena), então roda só ela, conferida contra o plano feito antes
/// pelas mesmas contas da cópia: a história (+0xb0..+0xd8) e a marca (+0x121) de cada nação e [0x9cd218] zerado.
unsafe extern "C" fn comparar_game_update_nation_coefficients() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I0%%);
    let iniciar = cpt6_b(0x9cd218) != 0;
    let hoje = cpt6_d(0xae2c90) as i16;
    let dez = cpt6_w(0x9cd200 + (current_month() * 2) as usize) == hoje && cpt6_w(0x9cd216) == hoje;
    let n = cpt6_d(0xae2350) as i32;
    let base = cpt6_d(0xae23a8);
    let mut plano = Vec::new();
    for k in 0..n.max(0) as usize {
        let na = base + k * 0x122;
        let mut h = nation_coefficient_history(na);
        if iniciar {
            let v = cpt6_f(na + 0xa8);
            for x in h.iter_mut().take(5) {
                *x = v;
            }
        }
        let mut marca = cpt6_b(na + 0x121);
        if dez {
            roll_coefficient_year(na, &mut h, &mut marca);
        }
        plano.push((h, marca));
    }
    o();
    let mut erro = None;
    for (k, (h, marca)) in plano.iter().enumerate() {
        let na = base + k * 0x122;
        let g = nation_coefficient_history(na);
        if (0..6).any(|j| g[j].to_bits() != h[j].to_bits()) || cpt6_b(na + 0x121) != *marca {
            erro = Some(format!("nação {} ({:x}): {:?}/{} x plano {:?}/{}", k, na, g, cpt6_b(na + 0x121), h, marca));
            break;
        }
    }
    if cpt6_b(0x9cd218) != 0 {
        erro = Some(format!("[0x9cd218] = {}", cpt6_b(0x9cd218)));
    }
    PLACAR.record(erro.is_none(), || format!("game_update_nation_coefficients(dezembro {}): {}", dez, erro.clone().unwrap_or_default()));
}

/// { 0x5d6250 } (cdecl, game.cpp): `is_player_regen_day(dia)`: 1 nos dias 0, 80, 173 e 250 do ano (a busca das
/// aposentadorias), senão 0. Pelo laço do dia do jogo (0x5cda97, com a data inteira: o dia é a palavra baixa).
/// A original só devolve o al (o ah fica com o byte alto do dia).
unsafe extern "C" fn is_player_regen_day(dia: u32) -> u32 {
    let d = dia as u16;
    (d == 0 || d == 0x50 || d == 0xad || d == 0xfa) as u32
}

/// Comparador: pelo al (o único byte que os chamadores leem).
unsafe extern "C" fn comparar_is_player_regen_day(dia: u32) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(u32) -> u32>(%%I1%%)(dia);
    let n = is_player_regen_day(dia);
    PLACAR.record(r as u8 == n as u8, || format!("is_player_regen_day({}) {} x {}", dia as u16, r as u8, n as u8));
    r
}

/// { 0x5d61d0 } (cdecl, game.cpp): `clamp_char_sum(a, b, min, max)`: a soma dos dois bytes com sinal (16 bits),
/// presa entre os outros dois. Devolve o ax da original: `min` ou `max` estendidos, ou o byte alto do `max`
/// estendido com a soma no al (os 26 chamadores, no diário dos jogadores, só gravam o al).
unsafe extern "C" fn clamp_char_sum(a: u32, b: u32, min: u32, max: u32) -> u32 {
    let s = (a as u8 as i8 as i16).wrapping_add(b as u8 as i8 as i16);
    let lo = min as u8 as i8 as i16;
    let hi = max as u8 as i8 as i16;
    if lo > s {
        lo as u16 as u32
    } else if hi < s {
        hi as u16 as u32
    } else {
        ((hi as u16 & 0xff00) | (s as u16 & 0xff)) as u32
    }
}

/// Comparador: pelo ax (os 16 bits altos do eax da original são os do chamador).
unsafe extern "C" fn comparar_clamp_char_sum(a: u32, b: u32, min: u32, max: u32) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(u32, u32, u32, u32) -> u32>(%%I2%%)(a, b, min, max);
    let n = clamp_char_sum(a, b, min, max);
    PLACAR.record(r as u16 == n as u16, || format!("clamp_char_sum({}, {}, {}, {}) {:x} x {:x}", a as u8 as i8, b as u8 as i8, min as u8 as i8, max as u8 as i8, r as u16, n as u16));
    r
}
