// ---------- 257a leva: o adiamento pelo clima: a decisão de adiar ou abandonar o jogo, as finanças do clube e a ordem das reservas do estádio (weather.cpp, finance.cpp, stadium.cpp) ----------


unsafe fn cpu5_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}


unsafe fn cpu5_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    f(n)
}

/// O fim do adiamento: sorteio(n) diferente de zero não adia (0), zero abandona (6).
unsafe fn cpu5_abandonar_se_zero(n: i32) -> u8 {
    if cpu5_sortear(n) != 0 {
        0
    } else {
        6
    }
}

/// O fim do adiamento: sorteio(n) zero não adia (0), diferente de zero adia (5).
unsafe fn cpu5_adiar_se_nao_zero(n: i32) -> u8 {
    if cpu5_sortear(n) == 0 {
        0
    } else {
        5
    }
}

/// { 0x923630 } (thiscall, ret 4, weather.cpp): `Weather::check_postponement(jogo)`, pelo dia dos jogos (0x6b3c10) e
/// pela preparação da partida (0x6b7f20). Sem estádio (+0x18) com cidade (+0x38) e lugares (+0x3c), sem competição
/// (+0x14), com o mandante (+0x1c) fora de liga (+0x57) ou o jogo fora da liga dele, ou estádio coberto (+0x4c): 0. Na
/// nação-chave do mandante (0x669360, tabela 0xb63d60 de 0x48 bytes), perto do fim da temporada do ano (+0x39/+0x3b): 0.
/// Na fatia da noite (+0x3f = 2), pelas finanças do mandante (0x5a0590, faixa 0x5986b0): sorteio(1000/2500/5000) zero
/// devolve 0x12. Depois o clima do jogo (0x923010, gravado na vaga do argumento como na original): chuva 5 com 1 a 13
/// graus, estádio descoberto (+0x4d) de até 20000 lugares: sorteio por lugares e mandante (+0x64), zero = 9; chuva 8
/// (até 15000 lugares) e 9 (pelo vento 4 e por 10000 lugares): uma cadeia de sorteios que devolve 0, 5 ou 6.
unsafe extern "thiscall" fn adiar_pelo_clima(this: usize, jogo: usize) -> u8 {
    let chave: unsafe extern "C" fn(usize) -> i8 = core::mem::transmute(FN_KEY_NATION_INDEX_OF_CLUB);
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FINANCETABLE_GET_CLUB_FINANCE);
    let faixa: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(FN_FINANCE_GET_WEALTH_BAND);
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
        let fim = core::ptr::read_unaligned((0xb63d99 + e) as *const i16);
        let ano = core::ptr::read_unaligned((0xb63d9b + e) as *const u16);
        let dia = core::ptr::read_unaligned(0xae2c90 as *const i16);
        let ano_atual = core::ptr::read_unaligned(0xae2c92 as *const u16);
        if fim > 0x3c {
            if ano == ano_atual && (fim as i32 - 0x3c) < dia as i32 {
                return 0;
            }
        } else if ano == ano_atual && dia > 0x154 {
            return 0;
        }
    }
    if read_u8(jogo + 0x3f) == 2 { // cpu5: a fatia da noite
        let fin = financas(read_usize(0xae2a38), casa);
        if fin != 0 {
            let f = faixa(fin) as i8;
            let n = if f == -2 {
                1000
            } else if f == -1 && (read_u8(fin + 0x166) as i8) > 0xf {
                0x9c4
            } else {
                5000
            };
            crate::comparacao::ramo("cpu_5 rolagem das financas");
            if cpu5_sortear(n) == 0 { // cpu5: a rolagem das finanças
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
            let cap = cpu5_i(est + 0x3c);
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
            if cpu5_sortear(n) != 0 { // cpu5: o sorteio da chuva forte
                0
            } else {
                9
            }
        }
        8 => {
            if read_u8(est + 0x4d) != 0 {
                return 0;
            }
            let cap = cpu5_i(est + 0x3c);
            if cap > 0x3a98 {
                return 0;
            }
            let mandante = read_u8(casa + 0x64) == 1;
            if cap > 0x1d4c && mandante {
                if cpu5_sortear(4) != 0 {
                    return 5;
                }
                return cpu5_abandonar_se_zero(4);
            }
            if cap <= 0x1d4c && !mandante {
                if cpu5_sortear(0x19) != 0 {
                    return 5;
                }
                return if cpu5_sortear(3) == 0 { 0 } else { 6 };
            }
            if cpu5_sortear(0x1e) == 0 {
                return 6;
            }
            cpu5_adiar_se_nao_zero(0xf)
        }
        9 => {
            let (d1, d2, d3) = if vento == 4 { (10, 0x14, 10) } else { (0x14, 10, 4) };
            if read_u8(est + 0x4d) != 0 && cpu5_sortear(d1) != 0 {
                return 0;
            }
            if cpu5_i(est + 0x3c) > 0x2710 {
                if vento == 4 {
                    if cpu5_sortear(d2) != 0 {
                        return 5;
                    }
                    return cpu5_abandonar_se_zero(d3);
                }
                if cpu5_sortear(d2) == 0 {
                    return cpu5_abandonar_se_zero(d3);
                }
                return 5;
            }
            if cpu5_sortear(0x32) == 0 {
                return 6;
            }
            cpu5_adiar_se_nao_zero(if vento == 4 { 8 } else { 4 })
        }
        _ => 0,
    }
}

// ponytail: sorteia: só a original; a cópia leva os ramos.
unsafe extern "thiscall" fn comparar_adiar_pelo_clima(this: usize, jogo: usize) -> u8 {
    orig::<unsafe extern "thiscall" fn(usize, usize) -> u8>(1206)(this, jogo)
}

/// { 0x5a0590 } (thiscall, ret 4, finance.cpp): `FinanceTable::get_club_finance(clube)`: o registro de finanças
/// (0x167 bytes em [this]) do clube pelo id (+0); clube B com parceiro A (0x540a50 com o byte devolvido 0): o do A.
/// Clube nulo: a caixa "Error" de finance.CPP (a original).
unsafe extern "thiscall" fn financas_do_clube(this: usize, clube: usize) -> usize {
    if clube == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(1207)(this, clube);
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

/// Comparador: pelo valor (clube nulo: só a original, com a caixa de erro).
unsafe extern "thiscall" fn comparar_financas_do_clube(this: usize, clube: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(1207);
    if clube == 0 {
        return o(this, clube);
    }
    let r = o(this, clube);
    let n = financas_do_clube(this, clube);
    PLACAR.registrar(r == n, || format!("financas_do_clube(clube {:#x}) original={:#x} rust={:#x}", clube, r, n));
    r
}

/// { 0x85e380 } (cdecl, stadium.cpp): `stadium_compare_bookings(a, b)`, a ordem das reservas do estádio (qsort em
/// 0x85ba1b, 0x85c4ba, 0x85ddf3, 0x85df26): pela data (+4, Date::operator< 0x456770 e > 0x456740), depois pelo id do
/// estádio (+0).
unsafe extern "C" fn ordem_das_reservas_do_estadio(a: usize, b: usize) -> i32 {
    let antes: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_DATE_OPERATOR_3C);
    let depois: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_DATE_OPERATOR_3E);
    let (d0, d1) = (read_usize(b + 4) as u32, read_usize(b + 8) as u32);
    if antes(a + 4, d0, d1) != 0 {
        return -1;
    }
    if depois(a + 4, d0, d1) != 0 {
        return 1;
    }
    cpu5_i(a).wrapping_sub(cpu5_i(b))
}

/// Comparador: pelo valor.
unsafe extern "C" fn comparar_ordem_das_reservas_do_estadio(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(1208)(a, b);
    let n = ordem_das_reservas_do_estadio(a, b);
    PLACAR.registrar(r == n, || format!("ordem_das_reservas_do_estadio({:#x}, {:#x}) original={} rust={}", a, b, r, n));
    r
}

// ---------- 257a leva: o clima da partida: o vento, a chuva e a temperatura do jogo, a temperatura pela faixa e o estado do gramado (weather.cpp) ----------


unsafe fn cpu6_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpu6_s(a: usize) -> i8 {
    *(a as *const i8)
}

/// A região do clima de uma cidade (+0x34) se for válida (0 ≤ r < [this + 8]): [this] + r × 0x6e.
unsafe fn cpu6_regiao(this: usize, cidade: usize) -> Option<usize> {
    let r = cpu6_i(cidade + 0x34);
    if r >= 0 && r < cpu6_i(this + 8) {
        Some(read_usize(this).wrapping_add(r as usize * 0x6e))
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
unsafe extern "thiscall" fn clima_da_partida(this: usize, s: *mut u8, jogo: usize) {
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
        if let Some(r) = cpu6_regiao(this, cid) {
            reg = r;
        } else {
            let n = read_usize(cid + 0x1f);
            if n != 0 {
                let capital = read_usize(n + 0x7a);
                if capital != 0 {
                    if let Some(r) = cpu6_regiao(this, capital) {
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
        let leitura = |k: usize| cpu6_s(cidades.wrapping_add(read_usize(cid).wrapping_mul(4)) + k);
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
unsafe extern "thiscall" fn comparar_clima_da_partida(this: usize, s: *mut u8, jogo: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, *mut u8, usize)>(1209);
    let regioes = [(s as usize, 4)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&regioes, || o(this, s, jogo), || clima_da_partida(this, s, jogo));
    PLACAR.registrar(igual, || format!("clima_da_partida(jogo {:#x}) {:x?} x {:x?}", jogo, mo.0, mr.0));
}

/// { 0x9232c0 } (thiscall, ret 8, weather.cpp): `Weather::get_temperature(faixa, jogo)`, pelo clima da partida. Com o
/// dia d (do jogo +0x2a, sem sinal; sem jogo o dia do jogo) e x = d × faixa: faixa 0: x % 6 + 1; 1: x % 6 − 5; 2:
/// x % 7 + 7; 3: x % 8 + 14; 4: x % 7 + 22; 5: x % 6 + 29; 6: x % 6 + 35; outra: −100. Jogo da fatia da noite (+0x3f = 2):
/// abaixo de 10, − 2; senão × 0,8 truncado. (Lê +0x3f do jogo mesmo sem jogo, como a original.)
unsafe extern "thiscall" fn temperatura_do_jogo(_this: usize, faixa: u32, jogo: usize) -> u8 {
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
        return ftol(t as f64 * f64_em(0x9689b0)) as u8;
    }
    t as u8
}

/// Comparador: pelo valor.
unsafe extern "thiscall" fn comparar_temperatura_do_jogo(this: usize, faixa: u32, jogo: usize) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize, u32, usize) -> u8>(1210)(this, faixa, jogo);
    let n = temperatura_do_jogo(this, faixa, jogo);
    PLACAR.registrar(r == n, || format!("temperatura_do_jogo(faixa {}, jogo {:#x}) original={} rust={}", faixa as u8 as i8, jogo, r as i8, n as i8));
    r
}

/// { 0x923970 } (thiscall, ret 8, weather.cpp): `Weather::get_pitch_condition(clima, jogo)`, na preparação da partida
/// (0x6b7f20, em +1 do motor: o gramado 2 e 0x20 pesam nos chutes). Sem jogo, sem estádio (+0x18) ou estádio coberto
/// (+0x4c): 0. h = (dia +0x2a + +0x10 + +0xc + +0) % 20. Pela chuva clima[1] e a temperatura clima[2] (1 a 28 graus),
/// com o estádio de marca +0x4d e os lugares +0x3c: chuva 5: 2/1 ou 8/1; chuva 1: 2 ou 0 por h < 6/4; chuva 2 e 3: por
/// h < 4/1/3; chuva 4: 2 ou 1/2; abaixo de 1 grau, chuva 1 e 8: 2, 0x10 ou 1; chuva 9: 2, 1 ou 0x20/1; sem chuva a 29
/// graus ou mais: 0x40 ou 4; senão 0.
unsafe extern "thiscall" fn gramado_do_jogo(_this: usize, clima: *const u8, jogo: usize) -> u8 {
    if jogo == 0 {
        return 0;
    }
    let est = read_usize(jogo + 0x18);
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
unsafe extern "thiscall" fn comparar_gramado_do_jogo(this: usize, clima: *const u8, jogo: usize) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize, *const u8, usize) -> u8>(1211)(this, clima, jogo);
    let n = gramado_do_jogo(this, clima, jogo);
    PLACAR.registrar(r == n, || format!("gramado_do_jogo(clima {:x?}, jogo {:#x}) original={:#x} rust={:#x}", core::slice::from_raw_parts(clima, 4), jogo, r, n));
    r
}

// ---------- 256a leva: o tempo das cidades: o tempo do dia, a região do weather.cfg e o período da região (weather.cpp) ----------


unsafe fn cpt5_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    f(n)
}

/// A linha seguinte do arquivo (0x90cba0: FILE*, buffer), 0 no fim.
unsafe fn cpt5_ler_linha(f: usize, buf: *mut u8) -> i32 {
    let ler: unsafe extern "C" fn(usize, *mut u8) -> i32 = core::mem::transmute(FN_UTILS_READ_CONFIG_LINE);
    ler(f, buf)
}

/// { 0x922ee0 } (thiscall, weather.cpp): `Weather::update_city_weather()`, todo dia pelo laço do dia do jogo (0x5cd9aa,
/// [0xde2f74]). Sem a tabela das cidades (+4, 4 bytes por cidade): nada. Cada cidade ([0xae2354], relida a cada volta):
/// as três medidas (+0, +1, +2) viram (medida + sorteio(100)) × 0,5 truncado, presas em 0..100; sorteio(10) zerado dá
/// o vento novo (+3) = sorteio(3).
unsafe extern "thiscall" fn tempo_das_cidades(this: usize) {
    if read_usize(this + 4) == 0 || read_usize(0xae2354) as i32 <= 0 {
        return;
    }
    crate::comparacao::ramo("cpt_5 tempo do dia");
    let mut i = 0usize;
    loop {
        for k in 0..3usize { // cpt5: as três medidas
            let p = read_usize(this + 4) + i * 4 + k;
            let r = cpt5_sortear(100);
            *(p as *mut u8) = ftol((r as i16 as i32 + *(p as *const i8) as i32) as f64 * f64_em(0x968968)) as u8;
            let p = read_usize(this + 4) + i * 4 + k;
            let v = *(p as *const i8);
            if v < 0 {
                *(p as *mut u8) = 0;
            } else if v > 0x64 {
                *(p as *mut u8) = 0x64;
            }
        }
        if cpt5_sortear(10) == 0 { // cpt5: um em dez muda o vento
            crate::comparacao::ramo("cpt_5 vento novo");
            let r = cpt5_sortear(3); // cpt5: o vento novo
            *((read_usize(this + 4) + i * 4 + 3) as *mut u8) = r as u8;
        }
        i += 1;
        if i as i32 >= read_usize(0xae2354) as i32 {
            break;
        }
    }
}

// ponytail: sorteia 3 ou 4 vezes por cidade: só a original; a cópia leva os ramos.
unsafe extern "thiscall" fn comparar_tempo_das_cidades(this: usize) {
    orig::<unsafe extern "thiscall" fn(usize)>(1167)(this)
}

/// { 0x922c40 } (thiscall, ret 8, weather.cpp): `Weather::read_region(f, regiao)`: uma linha do weather.cfg (0x90cba0,
/// buffer de 2000) com "%ld %s" (0xad4e24) no id (+0) e no nome (+4) da região (0x6e bytes); depois os quatro
/// períodos (0x922d20) em +0x1e, +0x31, +0x44 e +0x57. Devolve 1 se leu tudo (só o al, como a original).
unsafe extern "thiscall" fn ler_regiao_do_clima(this: usize, f: usize, regiao: usize) -> u8 {
    let ler: unsafe extern "C" fn(*const u8, usize, ...) -> i32 = core::mem::transmute(FN_946FD8);
    let periodo: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(FN_WEATHER_READ_REGION_PERIOD);
    crate::comparacao::ramo("cpt_5 regiao do clima");
    let mut linha = [0u8; 2000];
    if cpt5_ler_linha(f, linha.as_mut_ptr()) == 0 {
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
unsafe extern "thiscall" fn comparar_ler_regiao_do_clima(this: usize, f: usize, regiao: usize) -> u8 {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u8>(1168)(this, f, regiao)
}

/// { 0x922d20 } (thiscall, ret 8, weather.cpp): `Weather::read_region_period(f, periodo)`: arquivo nulo é a caixa
/// "Error" (weather.cpp, 0xea) e 0. Uma linha (0x90cba0) com o formato de 0xad4e2c (o nome e 18 inteiros: o dia,
/// cinco W, cinco P e sete H); com os 19 lidos, o período (0x13 bytes) recebe o dia (palavra em +0) e os 17 bytes
/// seguintes (+2..+0x12). Devolve 1 se leu (só o al).
unsafe extern "thiscall" fn ler_periodo_do_clima(_this: usize, f: usize, periodo: usize) -> u8 {
    let ler: unsafe extern "C" fn(*const u8, usize, ...) -> i32 = core::mem::transmute(FN_946FD8);
    if f == 0 {
        aviso_em(0xad4dec, 0xea);
        return 0;
    }
    crate::comparacao::ramo("cpt_5 periodo do clima");
    let mut linha = [0u8; 2000];
    if cpt5_ler_linha(f, linha.as_mut_ptr()) == 0 {
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
unsafe extern "thiscall" fn comparar_ler_periodo_do_clima(this: usize, f: usize, periodo: usize) -> u8 {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u8>(1169)(this, f, periodo)
}

