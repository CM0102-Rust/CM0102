use super::*;





/// Postponement outcome: random(n) != 0 -> 0 (no change), 0 -> 6 (abandoned).
///
/// [pt] O fim do adiamento: sorteio(n) diferente de zero não adia (0), zero abandona (6).
pub(crate) unsafe fn abandon_if_draw_zero(n: i32) -> u8 {
    if friendly_random(n) != 0 {
        0
    } else {
        6
    }
}

/// Postponement outcome: random(n) == 0 -> 0, otherwise 5 (postponed).
///
/// [pt] O fim do adiamento: sorteio(n) zero não adia (0), diferente de zero adia (5).
pub(crate) unsafe fn postpone_if_draw_nonzero(n: i32) -> u8 {
    if friendly_random(n) == 0 {
        0
    } else {
        5
    }
}

/// league matches of the home club only, at an uncovered stadium (+0x4c) with a city and places and outside the key
/// nation season end; in the evening slot, random(1000/2500/5000 by the finance band) zero returns 0x12; by the weather
/// (0x923010): rain 5 at 1 to 13 degrees in a stadium of up to 20000 places draws the postponement (9); rain 8 and 9
/// (ice), random chains returning 0, 5 or 6
/// Original: `Weather::check_postponement` (0x923630, weather.cpp, thiscall, ret 4).
///
/// [pt] { 0x923630 } (thiscall, ret 4, weather.cpp): `Weather::check_postponement(jogo)`, pelo dia dos jogos (0x6b3c10) e
/// pela preparação da partida (0x6b7f20). Sem estádio (+0x18) com cidade (+0x38) e lugares (+0x3c), sem competição
/// (+0x14), com o mandante (+0x1c) fora de liga (+0x57) ou o jogo fora da liga dele, ou estádio coberto (+0x4c): 0. Na
/// nação-chave do mandante (0x669360, tabela 0xb63d60 de 0x48 bytes), perto do fim da temporada do ano (+0x39/+0x3b): 0.
/// Na fatia da noite (+0x3f = 2), pelas finanças do mandante (0x5a0590, faixa 0x5986b0): sorteio(1000/2500/5000) zero
/// devolve 0x12. Depois o clima do jogo (0x923010, gravado na vaga do argumento como na original): chuva 5 com 1 a 13
/// graus, estádio descoberto (+0x4d) de até 20000 lugares: sorteio por lugares e mandante (+0x64), zero = 9; chuva 8
/// (até 15000 lugares) e 9 (pelo vento 4 e por 10000 lugares): uma cadeia de sorteios que devolve 0, 5 ou 6.
pub(crate) unsafe extern "thiscall" fn check_postponement(this: usize, jogo: usize) -> u8 {
    let chave: unsafe extern "C" fn(usize) -> i8 = core::mem::transmute(FN_KEY_NATION_INDEX_OF_CLUB);
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FINANCETABLE_GET_CLUB_FINANCE);
    let band: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(FN_FINANCE_GET_WEALTH_BAND);
    let clima: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_WEATHER_GET_MATCH_WEATHER);
    let est = read_usize(jogo + 0x18);
    if est == 0 || read_usize(est + 0x38) == 0 || read_usize(est + 0x3c) == 0 || read_usize(jogo + 0x14) == 0 {
        return 0;
    }
    let casa = read_usize(jogo + 0x1c);
    if read_usize(casa + 0x57) == 0 {
        return 0;
    }
    let k: i8 = if casa != 0 { chave(casa) } else { -1 };
    if read_usize(jogo + 0x14) != read_usize(casa + 0x57) || read_u8(est + 0x4c) != 0 {
        return 0;
    }
    if (0..0x23).contains(&k) {
        let e = k as usize * 0x48;
        let fim = core::ptr::read_unaligned((G_LEAGUE_WORLD_CUP_END_DATE + e) as *const i16);
        let ano = core::ptr::read_unaligned((0xb63d9b + e) as *const u16);
        let dia = core::ptr::read_unaligned(G_CURRENT_DATE as *const i16);
        let ano_atual = core::ptr::read_unaligned(G_CURRENT_DATE_YEAR as *const u16);
        if fim > 0x3c {
            if ano == ano_atual && (fim as i32 - 0x3c) < dia as i32 {
                return 0;
            }
        } else if ano == ano_atual && dia > 0x154 {
            return 0;
        }
    }
    if read_u8(jogo + 0x3f) == 2 { // cpu5: a fatia da noite
        let fin = financas(read_usize(G_FINANCE_TABLE), casa);
        if fin != 0 {
            let f = band(fin) as i8;
            let n = if f == -2 {
                1000
            } else if f == -1 && (read_u8(fin + 0x166) as i8) > 0xf {
                0x9c4
            } else {
                5000
            };
            crate::comparacao::ramo("cpu_5 rolagem das financas");
            if friendly_random(n) == 0 { // cpu5: a rolagem das finanças
                return 0x12;
            }
        }
    }
    // ponytail: a original grava o clima na vaga do próprio argumento; se o clima não grava (sem regiões), ficam os
    // bytes do ponteiro do jogo, como aqui.
    let mut cl = jogo as u32;
    clima(this, &mut cl as *mut u32 as usize, jogo);
    let [vento, chuva, temp, _] = cl.to_le_bytes();
    let temp = temp as i8;
    let est = read_usize(jogo + 0x18);
    let casa = read_usize(jogo + 0x1c);
    match chuva {
        5 => {
            if !(1..0xe).contains(&temp) || read_u8(est + 0x4d) != 0 {
                return 0;
            }
            let cap = read_i32(est + 0x3c);
            if cap > 0x4e20 {
                return 0;
            }
            let mandante = read_u8(casa + 0x64) == 1;
            let n = if cap > 0x30d4 {
                0xfa
            } else if cap > 0x1388 {
                if mandante { 0x96 } else { 0x1e }
            } else if mandante {
                0x32
            } else {
                0xa
            };
            crate::comparacao::ramo("cpu_5 chuva forte");
            if friendly_random(n) != 0 { // cpu5: o sorteio da chuva forte
                0
            } else {
                9
            }
        }
        8 => {
            if read_u8(est + 0x4d) != 0 {
                return 0;
            }
            let cap = read_i32(est + 0x3c);
            if cap > 0x3a98 {
                return 0;
            }
            let mandante = read_u8(casa + 0x64) == 1;
            if cap > 0x1d4c && mandante {
                if friendly_random(4) != 0 {
                    return 5;
                }
                return abandon_if_draw_zero(4);
            }
            if cap <= 0x1d4c && !mandante {
                if friendly_random(0x19) != 0 {
                    return 5;
                }
                return if friendly_random(3) == 0 { 0 } else { 6 };
            }
            if friendly_random(0x1e) == 0 {
                return 6;
            }
            postpone_if_draw_nonzero(0xf)
        }
        9 => {
            let (d1, d2, d3) = if vento == 4 { (10, 0x14, 10) } else { (0x14, 10, 4) };
            if read_u8(est + 0x4d) != 0 && friendly_random(d1) != 0 {
                return 0;
            }
            if read_i32(est + 0x3c) > 0x2710 {
                if vento == 4 {
                    if friendly_random(d2) != 0 {
                        return 5;
                    }
                    return abandon_if_draw_zero(d3);
                }
                if friendly_random(d2) == 0 {
                    return abandon_if_draw_zero(d3);
                }
                return 5;
            }
            if friendly_random(0x32) == 0 {
                return 6;
            }
            postpone_if_draw_nonzero(if vento == 4 { 8 } else { 4 })
        }
        _ => 0,
    }
}

// ponytail: sorteia: só a original; a cópia leva os ramos.
/// Comparer for `check_postponement` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_check_postponement(this: usize, jogo: usize) -> u8 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_CHECK_POSTPONEMENT)(this, jogo)
}

/// the finance record (0x167 bytes) by the club id; a B club with an A partner, the A one (0x540a50); null club, error
/// box
/// Original: `FinanceTable::get_club_finance` (0x5a0590, finance.cpp, thiscall, ret 4).
///
/// [pt] { 0x5a0590 } (thiscall, ret 4, finance.cpp): `FinanceTable::get_club_finance(clube)`: o registro de finanças
/// (0x167 bytes em [this]) do clube pelo id (+0); clube B com parceiro A (0x540a50 com o byte devolvido 0): o do A.
/// Clube nulo: a caixa "Error" de finance.CPP (a original).
pub(crate) unsafe extern "thiscall" fn get_club_finance(this: usize, clube: usize) -> usize {
    if clube == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_GET_CLUB_FINANCE)(this, clube);
    }
    let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
    let mut eh_a: u32 = 0;
    let mut c = clube;
    let p = parceiro(clube, &mut eh_a, 1);
    if p != 0 && eh_a == 0 {
        c = p;
    }
    read_usize(this).wrapping_add((read_usize(c) as u32).wrapping_mul(0x167) as usize)
}

/// Comparer for `get_club_finance` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor (clube nulo: só a original, com a caixa de erro).
pub(crate) unsafe extern "thiscall" fn comparar_get_club_finance(this: usize, clube: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_GET_CLUB_FINANCE);
    if clube == 0 {
        return o(this, clube);
    }
    let r = o(this, clube);
    let n = get_club_finance(this, clube);
    PLACAR.registrar(r == n, || format!("get_club_finance(clube {:#x}) original={:#x} rust={:#x}", clube, r, n));
    r
}






/// City's weather region record ([this] + r x 0x6e) if 0 <= r < [this+8].
///
/// [pt] A região do clima de uma cidade (+0x34) se for válida (0 ≤ r < [this + 8]): [this] + r × 0x6e.
pub(crate) unsafe fn weather_region(this: usize, cidade: usize) -> Option<usize> {
    let r = read_i32(cidade + city::CITY_WEATHER);
    if r >= 0 && r < read_i32(this + 8) {
        Some(read_usize(this).wrapping_add(r as usize * 0x6e))
    } else {
        None
    }
}

/// the stadium city (else the home club one, else the capital); the city wind, the region and the period by the day;
/// wind (0..4), rain (0, 1, 2, 3, 5) and temperature band by the period limits; below zero, the rain becomes snow/ice
/// (6..9) by the city wind
/// Original: `Weather::get_match_weather` (0x923010, weather.cpp, thiscall, ret 8).
///
/// [pt] { 0x923010 } (thiscall, ret 8, weather.cpp): `Weather::get_match_weather(saida, jogo)`, na preparação da partida
/// (0x6b7f20, em +0x4756 do motor: o vento pesa nos chutes de longe), no adiamento (0x923630) e nas telas. Sem regiões
/// ([this], [this + 8]): nada. A cidade: a do estádio (+0x18 → +0x38), senão a do estádio do mandante (+0x1c → +0x69 →
/// +0x38), senão a capital da nação dele (+0x53 → +0x7a). Com cidade: saida[3] = o vento da cidade (tabela [this + 4], 4
/// bytes por cidade, byte 3) e a região dela (senão a da capital da nação +0x1f, senão a primeira). O período pelo dia do
/// jogo (+0x2a; sem jogo o dia do jogo) contra os limites +0x1e/+0x31/+0x44/+0x57 da região (3, 0, 1, 2, 3); no período
/// (19 bytes a partir de +0x20): saida[0] = vento pela leitura 2 da cidade (+0x20..+0x24: 0..4), saida[1] = chuva pela
/// leitura 1 (+0x25..+0x29: 0, 1, 2, 3, 5), e a faixa da temperatura pela leitura 2 (+0x2a..+0x30: 1, 0, 2..6, senão 3).
/// Sem cidade: saida[3], [0] e [1] = 0 e a faixa 2. saida[2] = a temperatura (0x9232c0); abaixo de zero, a chuva vira
/// neve/gelo: 1 → 6, 2 e 3 → 6/7/8 e 5 → 8/7/9 pelo vento da cidade 0/1/outro.
pub(crate) unsafe extern "thiscall" fn get_match_weather(this: usize, s: *mut u8, jogo: usize) {
    let temperatura: unsafe extern "thiscall" fn(usize, u32, usize) -> u8 = core::mem::transmute(FN_WEATHER_GET_TEMPERATURE);
    if read_usize(this) == 0 || read_usize(this + 8) == 0 {
        return;
    }
    let mut cid = 0usize;
    if jogo != 0 {
        let est = read_usize(jogo + 0x18);
        if est != 0 {
            cid = read_usize(est + 0x38);
        }
    }
    if cid == 0 {
        let casa = read_usize(jogo + 0x1c);
        if casa != 0 {
            let c = read_usize(casa + 0x69);
            if c != 0 && read_usize(c + 0x38) != 0 {
                cid = read_usize(c + 0x38);
            } else {
                let n = read_usize(casa + 0x53);
                if n != 0 && read_usize(n + 0x7a) != 0 {
                    cid = read_usize(n + 0x7a);
                }
            }
        }
    }
    let cidades = read_usize(this + 4);
    let mut reg = read_usize(this);
    if cid != 0 {
        let leituras = cidades.wrapping_add(read_usize(cid).wrapping_mul(4));
        *s.add(3) = *((leituras + 3) as *const u8);
        if let Some(r) = weather_region(this, cid) {
            reg = r;
        } else {
            let n = read_usize(cid + 0x1f);
            if n != 0 {
                let capital = read_usize(n + 0x7a);
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
        core::ptr::read_unaligned(G_CURRENT_DATE as *const i16)
    };
    let limit = |o: usize| core::ptr::read_unaligned((reg + o) as *const i16);
    let p: usize = if dia < limit(0x1e) {
        3
    } else if dia < limit(0x31) {
        0
    } else if dia < limit(0x44) {
        1
    } else if dia < limit(0x57) {
        2
    } else {
        3
    };
    let band: u32 = if cid == 0 {
        *s = 0;
        *s.add(1) = 0;
        2
    } else {
        let per = reg + 19 * p;
        let lim = |o: usize| read_i8(per + o);
        let leitura = |k: usize| read_i8(cidades.wrapping_add(read_usize(cid).wrapping_mul(4)) + k);
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
    let t = temperatura(this, band, jogo) as i8;
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

/// Comparer for `get_match_weather` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado nos 4 bytes da saída.
pub(crate) unsafe extern "thiscall" fn comparar_get_match_weather(this: usize, s: *mut u8, jogo: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *mut u8, usize)>(IDX_GET_MATCH_WEATHER);
    let regioes = [(s as usize, 4)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&regioes, || o(this, s, jogo), || get_match_weather(this, s, jogo));
    PLACAR.registrar(igual, || format!("get_match_weather(jogo {:#x}) {:x?} x {:x?}", jogo, mo.0, mr.0));
}

/// (day × band) modulo 6/7/8 plus the band base (−5 to 35; other band −100); in the evening slot, − 2 below 10,
/// otherwise × 0.8
/// Original: `Weather::get_temperature` (0x9232c0, weather.cpp, thiscall, ret 8).
///
/// [pt] { 0x9232c0 } (thiscall, ret 8, weather.cpp): `Weather::get_temperature(faixa, jogo)`, pelo clima da partida. Com o
/// dia d (do jogo +0x2a, sem sinal; sem jogo o dia do jogo) e x = d × faixa: faixa 0: x % 6 + 1; 1: x % 6 − 5; 2:
/// x % 7 + 7; 3: x % 8 + 14; 4: x % 7 + 22; 5: x % 6 + 29; 6: x % 6 + 35; outra: −100. Jogo da fatia da noite (+0x3f = 2):
/// abaixo de 10, − 2; senão × 0,8 truncado. (Lê +0x3f do jogo mesmo sem jogo, como a original.)
pub(crate) unsafe extern "thiscall" fn get_temperature(_this: usize, band: u32, jogo: usize) -> u8 {
    let f = band as u8 as i8 as i32;
    let d: i32 = if jogo != 0 {
        core::ptr::read_unaligned((jogo + 0x2a) as *const u16) as i32
    } else {
        core::ptr::read_unaligned(G_CURRENT_DATE as *const i16) as i32
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

/// Comparer for `get_temperature` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor.
pub(crate) unsafe extern "thiscall" fn comparar_get_temperature(this: usize, band: u32, jogo: usize) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> u8>(IDX_GET_TEMPERATURE)(this, band, jogo);
    let n = get_temperature(this, band, jogo);
    PLACAR.registrar(r == n, || format!("get_temperature(band {}, jogo {:#x}) original={} rust={}", band as u8 as i8, jogo, r as i8, n as i8));
    r
}

/// by the rain, the temperature, the stadium flag and places and a fixture remainder modulo 20: 0, 1, 2, 4, 8, 0x10,
/// 0x20 or 0x40; covered stadium, 0
/// Original: `Weather::get_pitch_condition` (0x923970, weather.cpp, thiscall, ret 8).
///
/// [pt] { 0x923970 } (thiscall, ret 8, weather.cpp): `Weather::get_pitch_condition(clima, jogo)`, na preparação da partida
/// (0x6b7f20, em +1 do motor: o gramado 2 e 0x20 pesam nos chutes). Sem jogo, sem estádio (+0x18) ou estádio coberto
/// (+0x4c): 0. h = (dia +0x2a + +0x10 + +0xc + +0) % 20. Pela chuva clima[1] e a temperatura clima[2] (1 a 28 graus),
/// com o estádio de marca +0x4d e os lugares +0x3c: chuva 5: 2/1 ou 8/1; chuva 1: 2 ou 0 por h < 6/4; chuva 2 e 3: por
/// h < 4/1/3; chuva 4: 2 ou 1/2; abaixo de 1 grau, chuva 1 e 8: 2, 0x10 ou 1; chuva 9: 2, 1 ou 0x20/1; sem chuva a 29
/// graus ou mais: 0x40 ou 4; senão 0.
pub(crate) unsafe extern "thiscall" fn get_pitch_condition(_this: usize, clima: *const u8, jogo: usize) -> u8 {
    if jogo == 0 {
        return 0;
    }
    let est = read_usize(jogo + 0x18);
    if est == 0 || *((est + 0x4c) as *const u8) != 0 {
        return 0;
    }
    let soma = (core::ptr::read_unaligned((jogo + 0x2a) as *const u16) as i32)
        .wrapping_add(read_i32(jogo + 0x10))
        .wrapping_add(read_i32(jogo + 0xc))
        .wrapping_add(read_i32(jogo));
    let h = soma % 20;
    let chuva = *clima.add(1);
    let t = *clima.add(2) as i8;
    let marca = *((est + 0x4d) as *const u8) != 0;
    let lugares = read_i32(est + 0x3c);
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

/// Comparer for `get_pitch_condition` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor.
pub(crate) unsafe extern "thiscall" fn comparar_get_pitch_condition(this: usize, clima: *const u8, jogo: usize) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, *const u8, usize) -> u8>(IDX_GET_PITCH_CONDITION)(this, clima, jogo);
    let n = get_pitch_condition(this, clima, jogo);
    PLACAR.registrar(r == n, || format!("get_pitch_condition(clima {:x?}, jogo {:#x}) original={:#x} rust={:#x}", core::slice::from_raw_parts(clima, 4), jogo, r, n));
    r
}



/// Next line of the file into buf (utils read_config_line 0x90cba0); 0 at end.
///
/// [pt] A linha seguinte do arquivo (0x90cba0: FILE*, buffer), 0 no fim.
pub(crate) unsafe fn read_config_line(f: usize, buf: *mut u8) -> i32 {
    let ler: unsafe extern "C" fn(usize, *mut u8) -> i32 = core::mem::transmute(FN_UTILS_READ_CONFIG_LINE);
    ler(f, buf)
}

/// each city: the three readings become (reading + random(100)) × 0.5, clamped to 0..100; one time in ten (random(10)
/// zero), the new wind = random(3)
/// Original: `Weather::update_city_weather` (0x922ee0, weather.cpp, thiscall).
///
/// [pt] { 0x922ee0 } (thiscall, weather.cpp): `Weather::update_city_weather()`, todo dia pelo laço do dia do jogo (0x5cd9aa,
/// [0xde2f74]). Sem a tabela das cidades (+4, 4 bytes por cidade): nada. Cada cidade ([0xae2354], relida a cada volta):
/// as três medidas (+0, +1, +2) viram (medida + sorteio(100)) × 0,5 truncado, presas em 0..100; sorteio(10) zerado dá
/// o vento novo (+3) = sorteio(3).
pub(crate) unsafe extern "thiscall" fn update_city_weather(this: usize) {
    if read_usize(this + 4) == 0 || read_usize(G_CITY_COUNT) as i32 <= 0 {
        return;
    }
    crate::comparacao::ramo("cpt_5 tempo do dia");
    let mut i = 0usize;
    loop {
        for k in 0..3usize { // cpt5: as três medidas
            let p = read_usize(this + 4) + i * 4 + k;
            let r = friendly_random(100);
            *(p as *mut u8) = match_pl_ftol((r as i16 as i32 + *(p as *const i8) as i32) as f64 * read_f64_at(0x968968)) as u8;
            let p = read_usize(this + 4) + i * 4 + k;
            let v = *(p as *const i8);
            if v < 0 {
                *(p as *mut u8) = 0;
            } else if v > 0x64 {
                *(p as *mut u8) = 0x64;
            }
        }
        if friendly_random(10) == 0 { // cpt5: um em dez muda o vento
            crate::comparacao::ramo("cpt_5 vento novo");
            let r = friendly_random(3); // cpt5: o vento novo
            *((read_usize(this + 4) + i * 4 + 3) as *mut u8) = r as u8;
        }
        i += 1;
        if i as i32 >= read_usize(G_CITY_COUNT) as i32 {
            break;
        }
    }
}

// ponytail: sorteia 3 ou 4 vezes por cidade: só a original; a cópia leva os ramos.
/// Comparer for `update_city_weather` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_update_city_weather(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_UPDATE_CITY_WEATHER)(this)
}

/// one weather.cfg line with the region id and name ("%ld %s") and its four periods (0x922d20) at +0x1e, +0x31, +0x44
/// and +0x57
/// Original: `Weather::read_region` (0x922c40, weather.cpp, thiscall, ret 8).
///
/// [pt] { 0x922c40 } (thiscall, ret 8, weather.cpp): `Weather::read_region(f, regiao)`: uma linha do weather.cfg (0x90cba0,
/// buffer de 2000) com "%ld %s" (0xad4e24) no id (+0) e no nome (+4) da região (0x6e bytes); depois os quatro
/// períodos (0x922d20) em +0x1e, +0x31, +0x44 e +0x57. Devolve 1 se leu tudo (só o al, como a original).
pub(crate) unsafe extern "thiscall" fn read_region(this: usize, f: usize, regiao: usize) -> u8 {
    let ler: unsafe extern "C" fn(*const u8, usize, ...) -> i32 = core::mem::transmute(FN_946FD8);
    let periodo: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(FN_WEATHER_READ_REGION_PERIOD);
    crate::comparacao::ramo("cpt_5 regiao do clima");
    let mut linha = [0u8; 2000];
    if read_config_line(f, linha.as_mut_ptr()) == 0 {
        return 0;
    }
    if ler(linha.as_ptr(), 0xad4e24, regiao, regiao + 4) != 2 {
        return 0;
    }
    for k in [0x1e, 0x31, 0x44, 0x57] {
        if periodo(this, f, regiao + k) == 0 {
            return 0;
        }
    }
    1
}

// ponytail: lê do arquivo (não dá para ler duas vezes): só a original; a cópia leva o ramo.
/// Comparer for `read_region` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_read_region(this: usize, f: usize, regiao: usize) -> u8 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u8>(IDX_READ_REGION)(this, f, regiao)
}

/// one line with the name and 18 integers (the day, five W, five P, seven H): the period gets the day (word) and the 17
/// bytes; a null file is the "Error" box
/// Original: `Weather::read_region_period` (0x922d20, weather.cpp, thiscall, ret 8).
///
/// [pt] { 0x922d20 } (thiscall, ret 8, weather.cpp): `Weather::read_region_period(f, periodo)`: arquivo nulo é a caixa
/// "Error" (weather.cpp, 0xea) e 0. Uma linha (0x90cba0) com o formato de 0xad4e2c (o nome e 18 inteiros: o dia,
/// cinco W, cinco P e sete H); com os 19 lidos, o período (0x13 bytes) recebe o dia (palavra em +0) e os 17 bytes
/// seguintes (+2..+0x12). Devolve 1 se leu (só o al).
pub(crate) unsafe extern "thiscall" fn read_region_period(_this: usize, f: usize, periodo: usize) -> u8 {
    let ler: unsafe extern "C" fn(*const u8, usize, ...) -> i32 = core::mem::transmute(FN_946FD8);
    if f == 0 {
        show_error_at(0xad4dec, 0xea);
        return 0;
    }
    crate::comparacao::ramo("cpt_5 periodo do clima");
    let mut linha = [0u8; 2000];
    if read_config_line(f, linha.as_mut_ptr()) == 0 {
        return 0;
    }
    // ponytail: o nome tem 200 bytes na original; aqui 2000 para o %s não estourar (com os nomes do arquivo, igual).
    let mut nome = [0u8; 2000];
    let mut v = [0i32; 18];
    let p = v.as_mut_ptr();
    let n = ler(linha.as_ptr(), 0xad4e2c, nome.as_mut_ptr(), p, p.add(1), p.add(2), p.add(3), p.add(4), p.add(5),
                p.add(6), p.add(7), p.add(8), p.add(9), p.add(10), p.add(11), p.add(12), p.add(13), p.add(14),
                p.add(15), p.add(16), p.add(17));
    if n != 0x13 {
        return 0;
    }
    core::ptr::write_unaligned(periodo as *mut u16, v[0] as u16);
    for k in 1..18usize {
        *((periodo + 1 + k) as *mut u8) = v[k] as u8;
    }
    1
}

// ponytail: lê do arquivo: só a original; a cópia leva o ramo.
/// Comparer for `read_region_period` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_read_region_period(this: usize, f: usize, periodo: usize) -> u8 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u8>(IDX_READ_REGION_PERIOD)(this, f, periodo)
}

/// reads CM3_DATA\weather.cfg: the region count, the regions (0x922c40) and the city table, with the three readings
/// ((random(100) + random(100)) × 0.5) and the wind (random(3)) drawn; a file or memory error stops the game
/// Original: `Weather::load_config` (0x922810, weather.cpp, thiscall).
///
/// [pt] { 0x922810 } (thiscall, weather.cpp): `Weather::load_config()`, na criação do jogo (0x9226c0, Setup.cpp). Abre o
/// CM3_DATA\weather.cfg (0x90bde0, fopen 0x945b00 "r"); a primeira linha (0x90cba0, buffer de 1000) dá quantas regiões
/// ("%ld" em +8); as regiões (malloc de 0x6e bytes cada em +0) e a tabela das cidades (malloc de 4 bytes por cidade
/// [0xae2354] em +4); cada região pela 0x922c40 (a primeira que falha para o jogo; as outras só encerram a leitura);
/// cada cidade sorteia as três medidas ((sorteio(100) + sorteio(100)) × 0,5) e o vento (sorteio(3)). Fecha o arquivo
/// (0x945a09). Os erros (arquivo, linha, contagem, memória) são a caixa "Error" de weather.cpp e o jogo para.
pub(crate) unsafe extern "thiscall" fn load_config(this: usize) {
    let pasta: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(FN_UTILS_BUILD_FILE_PATH);
    let abrir: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(FN_CRT_FOPEN);
    let ler_linha: unsafe extern "C" fn(usize, *mut u8) -> i32 = core::mem::transmute(FN_UTILS_READ_CONFIG_LINE);
    let ler: unsafe extern "C" fn(*const u8, usize, ...) -> i32 = core::mem::transmute(FN_946FD8);
    let malloc: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let regiao: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(FN_WEATHER_READ_REGION);
    let fechar: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(FN_CRT_FCLOSE);
    let f = abrir(pasta(0x9b5348, 0xad4e18), 0x9899fc);
    if f == 0 {
        fatal_error_at(0xad4dec, 0x68);
    }
    let mut linha = [0u8; 1000];
    if ler_linha(f, linha.as_mut_ptr()) == 0 {
        fatal_error_at(0xad4dec, 0x6f);
    }
    if ler(linha.as_ptr(), 0x98c11c, this + 8) != 1 {
        fatal_error_at(0xad4dec, 0x75);
    }
    let r = malloc(read_usize(this + 8).wrapping_mul(0x6e));
    *(this as *mut usize) = r;
    if r == 0 {
        fatal_error_at(0xad4dec, 0x7e);
    }
    let c = malloc(read_usize(G_CITY_COUNT).wrapping_mul(4));
    *((this + 4) as *mut usize) = c;
    if c == 0 {
        fatal_error_at(0xad4dec, 0x86);
    }
    let mut i = 0i32;
    while i < read_i32(this + 8) {
        if regiao(this, f, read_usize(this) + i as usize * 0x6e) == 0 {
            if i == 0 {
                fatal_error_at(0xad4dec, 0x92);
            }
            break;
        }
        i += 1;
    }
    let mut k = 0usize;
    while (k as i32) < read_i32(0xae2354) {
        crate::comparacao::ramo("cpt_7 clima inicial");
        for j in 0..3usize {
            let a = friendly_random(100);
            let b = friendly_random(100); // cpt7: o segundo sorteio da medida
            *((read_usize(this + 4) + k * 4 + j) as *mut u8) = match_pl_ftol(a.wrapping_add(b) as f64 * read_f64_at(0x968968)) as u8;
        }
        let v = friendly_random(3); // cpt7: o vento inicial
        *((read_usize(this + 4) + k * 4 + 3) as *mut u8) = v as u8;
        k += 1;
    }
    fechar(f);
}

/// Comparer for `load_config` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_load_config(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_LOAD_CONFIG)(this)
}
