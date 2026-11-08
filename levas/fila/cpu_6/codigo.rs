// ---------- %%LEVA%%a leva: o clima da partida: o vento, a chuva e a temperatura do jogo, a temperatura pela faixa e o estado do gramado (weather.cpp) ----------

unsafe fn cpu6_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpu6_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpu6_s(a: usize) -> i8 {
    *(a as *const i8)
}

/// A região do clima de uma cidade (+0x34) se for válida (0 ≤ r < [this + 8]): [this] + r × 0x6e.
unsafe fn weather_region(this: usize, cidade: usize) -> Option<usize> {
    let r = cpu6_i(cidade + 0x34);
    if r >= 0 && r < cpu6_i(this + 8) {
        Some(cpu6_d(this).wrapping_add(r as usize * 0x6e))
    } else {
        None
    }
}

/// { 0x923010 } (thiscall, ret 8, weather.cpp): `Weather::get_match_weather(saida, jogo)`, na preparação da partida
/// (0x6b7f20, em +0x4756 do motor: o vento pesa nos chutes de longe), no adiamento (0x923630) e nas telas. Sem regiões
/// ([this], [this + 8]): nada. A cidade: a do estádio (+0x18 → +0x38), senão a do estádio do mandante (+0x1c → +0x69 →
/// +0x38), senão a capital da nação dele (+0x53 → +0x7a). Com cidade: saida[3] = o vento da cidade (tabela [this + 4], 4
/// bytes por cidade, byte 3) e a região dela (senão a da capital da nação +0x1f, senão a primeira). O período pelo dia do
/// jogo (+0x2a; sem jogo o dia do jogo) contra os limites +0x1e/+0x31/+0x44/+0x57 da região (3, 0, 1, 2, 3); no período
/// (19 bytes a partir de +0x20): saida[0] = vento pela leitura 2 da cidade (+0x20..+0x24: 0..4), saida[1] = chuva pela
/// leitura 1 (+0x25..+0x29: 0, 1, 2, 3, 5), e a faixa da temperatura pela leitura 2 (+0x2a..+0x30: 1, 0, 2..6, senão 3).
/// Sem cidade: saida[3], [0] e [1] = 0 e a faixa 2. saida[2] = a temperatura (0x9232c0); abaixo de zero, a chuva vira
/// neve/gelo: 1 → 6, 2 e 3 → 6/7/8 e 5 → 8/7/9 pelo vento da cidade 0/1/outro.
unsafe extern "thiscall" fn get_match_weather(this: usize, s: *mut u8, jogo: usize) {
    let temperatura: unsafe extern "thiscall" fn(usize, u32, usize) -> u8 = core::mem::transmute(0x9232c0usize);
    if cpu6_d(this) == 0 || cpu6_d(this + 8) == 0 {
        return;
    }
    let mut cid = 0usize;
    if jogo != 0 {
        let est = cpu6_d(jogo + 0x18);
        if est != 0 {
            cid = cpu6_d(est + 0x38);
        }
    }
    if cid == 0 {
        let casa = cpu6_d(jogo + 0x1c);
        if casa != 0 {
            let c = cpu6_d(casa + 0x69);
            if c != 0 && cpu6_d(c + 0x38) != 0 {
                cid = cpu6_d(c + 0x38);
            } else {
                let n = cpu6_d(casa + 0x53);
                if n != 0 && cpu6_d(n + 0x7a) != 0 {
                    cid = cpu6_d(n + 0x7a);
                }
            }
        }
    }
    let cidades = cpu6_d(this + 4);
    let mut reg = cpu6_d(this);
    if cid != 0 {
        let leituras = cidades.wrapping_add(cpu6_d(cid).wrapping_mul(4));
        *s.add(3) = *((leituras + 3) as *const u8);
        if let Some(r) = weather_region(this, cid) {
            reg = r;
        } else {
            let n = cpu6_d(cid + 0x1f);
            if n != 0 {
                let capital = cpu6_d(n + 0x7a);
                if capital != 0 {
                    if let Some(r) = weather_region(this, capital) {
                        reg = r;
                    }
                }
            }
        }
    } else {
        *s.add(3) = 0;
    }
    let dia = if jogo != 0 {
        core::ptr::read_unaligned((jogo + 0x2a) as *const i16)
    } else {
        core::ptr::read_unaligned(0xae2c90 as *const i16)
    };
    let limite = |o: usize| core::ptr::read_unaligned((reg + o) as *const i16);
    let p: usize = if dia < limite(0x1e) {
        3
    } else if dia < limite(0x31) {
        0
    } else if dia < limite(0x44) {
        1
    } else if dia < limite(0x57) {
        2
    } else {
        3
    };
    let faixa: u32 = if cid == 0 {
        *s = 0;
        *s.add(1) = 0;
        2
    } else {
        let per = reg + 19 * p;
        let lim = |o: usize| cpu6_s(per + o);
        let leitura = |k: usize| cpu6_s(cidades.wrapping_add(cpu6_d(cid).wrapping_mul(4)) + k);
        let a = leitura(2);
        *s = if a < lim(0x20) {
            0
        } else if a < lim(0x21) {
            crate::comparacao::ramo("cpu_6 vento fraco");
            1 // cpu6: o vento fraco
        } else if a < lim(0x22) {
            crate::comparacao::ramo("cpu_6 vento moderado");
            2 // cpu6: o vento moderado
        } else if a < lim(0x23) {
            crate::comparacao::ramo("cpu_6 vento forte");
            3 // cpu6: o vento forte
        } else if a >= lim(0x24) {
            0
        } else {
            4
        };
        let c = leitura(1);
        *s.add(1) = if c < lim(0x25) {
            0
        } else if c < lim(0x26) {
            1
        } else if c < lim(0x27) {
            2
        } else if c < lim(0x28) {
            3
        } else if c < lim(0x29) {
            5
        } else {
            0
        };
        let t = leitura(2);
        if t < lim(0x2a) {
            1
        } else if t < lim(0x2b) {
            0
        } else if t < lim(0x2c) {
            2
        } else if t < lim(0x2d) {
            3
        } else if t < lim(0x2e) {
            4
        } else if t < lim(0x2f) {
            5
        } else if t < lim(0x30) {
            6
        } else {
            3
        }
    };
    let t = temperatura(this, faixa, jogo) as i8;
    *s.add(2) = t as u8;
    if t >= 0 {
        return;
    }
    let vento = *s.add(3) as i8;
    match *s.add(1) {
        1 => *s.add(1) = 6,
        2 | 3 => *s.add(1) = match vento {
            0 => 6,
            1 => 7,
            _ => 8,
        },
        5 => *s.add(1) = match vento {
            0 => 8,
            1 => 7,
            _ => 9,
        },
        _ => {}
    }
}

/// Comparador: lado a lado nos 4 bytes da saída.
unsafe extern "thiscall" fn comparar_get_match_weather(this: usize, s: *mut u8, jogo: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *mut u8, usize)>(%%I0%%);
    let regioes = [(s as usize, 4)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&regioes, || o(this, s, jogo), || get_match_weather(this, s, jogo));
    PLACAR.registrar(igual, || format!("get_match_weather(jogo {:#x}) {:x?} x {:x?}", jogo, mo.0, mr.0));
}

/// { 0x9232c0 } (thiscall, ret 8, weather.cpp): `Weather::get_temperature(faixa, jogo)`, pelo clima da partida. Com o
/// dia d (do jogo +0x2a, sem sinal; sem jogo o dia do jogo) e x = d × faixa: faixa 0: x % 6 + 1; 1: x % 6 − 5; 2:
/// x % 7 + 7; 3: x % 8 + 14; 4: x % 7 + 22; 5: x % 6 + 29; 6: x % 6 + 35; outra: −100. Jogo da fatia da noite (+0x3f = 2):
/// abaixo de 10, − 2; senão × 0,8 truncado. (Lê +0x3f do jogo mesmo sem jogo, como a original.)
unsafe extern "thiscall" fn get_temperature(_this: usize, faixa: u32, jogo: usize) -> u8 {
    let f = faixa as u8 as i8 as i32;
    let d: i32 = if jogo != 0 {
        core::ptr::read_unaligned((jogo + 0x2a) as *const u16) as i32
    } else {
        core::ptr::read_unaligned(0xae2c90 as *const i16) as i32
    };
    let x = d.wrapping_mul(f);
    let t: i8 = match f {
        0 => (x % 6 + 1) as i8,
        1 => (x % 6 - 5) as i8,
        2 => (x % 7 + 7) as i8,
        3 => (x % 8 + 14) as i8,
        4 => (x % 7 + 0x16) as i8,
        5 => (x % 6 + 0x1d) as i8,
        6 => (x % 6 + 0x23) as i8,
        _ => -100,
    };
    if *((jogo + 0x3f) as *const u8) == 2 {
        if t < 10 {
            return (t as u8).wrapping_sub(2);
        }
        return match_pl_ftol(t as f64 * read_f64_at(0x9689b0)) as u8;
    }
    t as u8
}

/// Comparador: pelo valor.
unsafe extern "thiscall" fn comparar_get_temperature(this: usize, faixa: u32, jogo: usize) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> u8>(%%I1%%)(this, faixa, jogo);
    let n = get_temperature(this, faixa, jogo);
    PLACAR.registrar(r == n, || format!("get_temperature(faixa {}, jogo {:#x}) original={} rust={}", faixa as u8 as i8, jogo, r as i8, n as i8));
    r
}

/// { 0x923970 } (thiscall, ret 8, weather.cpp): `Weather::get_pitch_condition(clima, jogo)`, na preparação da partida
/// (0x6b7f20, em +1 do motor: o gramado 2 e 0x20 pesam nos chutes). Sem jogo, sem estádio (+0x18) ou estádio coberto
/// (+0x4c): 0. h = (dia +0x2a + +0x10 + +0xc + +0) % 20. Pela chuva clima[1] e a temperatura clima[2] (1 a 28 graus),
/// com o estádio de marca +0x4d e os lugares +0x3c: chuva 5: 2/1 ou 8/1; chuva 1: 2 ou 0 por h < 6/4; chuva 2 e 3: por
/// h < 4/1/3; chuva 4: 2 ou 1/2; abaixo de 1 grau, chuva 1 e 8: 2, 0x10 ou 1; chuva 9: 2, 1 ou 0x20/1; sem chuva a 29
/// graus ou mais: 0x40 ou 4; senão 0.
unsafe extern "thiscall" fn get_pitch_condition(_this: usize, clima: *const u8, jogo: usize) -> u8 {
    if jogo == 0 {
        return 0;
    }
    let est = cpu6_d(jogo + 0x18);
    if est == 0 || *((est + 0x4c) as *const u8) != 0 {
        return 0;
    }
    let soma = (core::ptr::read_unaligned((jogo + 0x2a) as *const u16) as i32)
        .wrapping_add(cpu6_i(jogo + 0x10))
        .wrapping_add(cpu6_i(jogo + 0xc))
        .wrapping_add(cpu6_i(jogo));
    let h = soma % 20;
    let chuva = *clima.add(1);
    let t = *clima.add(2) as i8;
    let marca = *((est + 0x4d) as *const u8) != 0;
    let lugares = cpu6_i(est + 0x3c);
    let morno = (1..0x1d).contains(&t);
    if chuva == 5 && morno {
        if marca {
            return if lugares > 0x6b6c { 2 } else { 1 };
        }
        return if lugares <= 0x445c { 8 } else { 1 };
    }
    if chuva == 1 && morno {
        if !marca {
            return 2;
        }
        let corte = if lugares > 0x6b6c { 4 } else { 6 };
        return if h >= corte { 0 } else { 2 };
    }
    if (chuva == 2 || chuva == 3) && morno {
        if marca {
            if lugares <= 0x6b6c {
                return if h >= 4 { 0 } else { 2 };
            }
            return if h != 0 { 0 } else { 2 };
        }
        return if h >= 3 { 0 } else { 2 };
    }
    if chuva == 4 && morno {
        if marca {
            if lugares <= 0x6b6c {
                return 2;
            }
            return if h >= 3 { 0 } else { 2 };
        }
        return if lugares > 0x445c { 2 } else { 1 };
    }
    if t < 1 && (chuva == 1 || chuva == 8) {
        if marca || lugares > 0xafc8 {
            return 2;
        }
        return if lugares <= 0x30d4 { 0x10 } else { 1 };
    }
    if chuva == 9 {
        if marca {
            return if lugares > 0x7530 { 2 } else { 1 };
        }
        return if lugares <= 0x4e20 { 0x20 } else { 1 };
    }
    if chuva == 0 && t >= 0x1d {
        return if lugares <= 0x88b8 { 0x40 } else { 4 };
    }
    0
}

/// Comparador: pelo valor.
unsafe extern "thiscall" fn comparar_get_pitch_condition(this: usize, clima: *const u8, jogo: usize) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, *const u8, usize) -> u8>(%%I2%%)(this, clima, jogo);
    let n = get_pitch_condition(this, clima, jogo);
    PLACAR.registrar(r == n, || format!("get_pitch_condition(clima {:x?}, jogo {:#x}) original={:#x} rust={:#x}", core::slice::from_raw_parts(clima, 4), jogo, r, n));
    r
}
