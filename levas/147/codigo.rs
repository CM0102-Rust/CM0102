// ---------- centesima quadragesima setima leva: o chute (0x702050, 0x6d5950, 0x6d5a70) ----------

/// { 0x6d5950 } (ret 4, match_eng.cpp): o lado `lado` esta atras no agregado (0x6ce310 sem bonus): perde
/// no placar somado ou, no jogo de ida sem terceira fase nem classificado, empata.
unsafe extern "thiscall" fn atras_no_agregado(m: usize, lado: i8) -> u8 {
    let (mut casa, mut fora, mut classificado, mut ida, mut fase3) = (0i8, 0i8, 0u8, 0u8, 0u8);
    placar_agregado(m, lado, &mut casa, &mut fora, &mut classificado, &mut ida, &mut fase3, 0);
    if casa < fora {
        return 1;
    }
    (ida != 0 && fase3 == 0 && classificado == 0 && casa <= fora) as u8
}

/// { 0x6d5a70 } (ret 4, match_eng.cpp): o primeiro companheiro de `p` (os 20 do lado dele, ativos) no
/// ataque fundo (0x70df40), no meio (0x70df20) e sem ninguem pressionando (0x6bcbd0 sem lista), ou 0.
unsafe extern "thiscall" fn companheiro_livre(m: usize, p: usize) -> usize {
    let l = lado(p);
    for k in 0..20 {
        let j = jogador_do_bloco(m, l, k);
        if byte(j, 0x19) < 0 || limite_jogador(j) == 0 || x_fora_do_meio(j) != 0 {
            continue;
        }
        let mut soma = 0i16;
        if pressao_com(m, j, &mut soma, 0, &mut |_, _| {}) == 0 {
            return j;
        }
    }
    0
}

/// Os locais do chute (0x702050) que o desfecho (gol, defesa, rebote) le: pelo deslocamento na pilha.
struct Chute {
    p: usize,
    m: usize,
    g: usize,
    lado: i8,
    tipo: u8,
    a1: u32,
    v: i32,
    v0: u32,
    som: u32,
    modo: u8,
    conta: i8,
    lista: usize,
    d: f32,
    l10: u8,
    l11: u8,
    l14: u8,
    l16: i8,
    l17: u8,
    l18: bool,
    l19: bool,
    l1a: bool,
    l1b: i8,
    l20: u8,
    l25: u8,
    l28: i8,
    l38_antes: i16,
    l40: i16,
    l50: i8,
    l54: i8,
    l58: i16,
    l5c: i32,
    l60: i32,
    l64: i32,
    l68: u32,
    e: usize,
    l8c: i16,
    l12: i8,
    l13: i8,
    l15: u8,
    l26: u8,
    l4e: u8,
    l4f: u8,
}

/// Narracao com sete argumentos pela fita (codigo | a7 << 16, a5 | a6 << 8).
fn narrar_finalizacao(chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, m: usize, codigo: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: i8) {
    unsafe {
        let narr = *((m + 0x8f7a) as *const usize);
        chamar(0x6d6f70, narr, [(codigo & 0xffff) | (a7 as u8 as u32) << 16, m as u32, a3, a4, (a5 & 0xff) | (a6 & 0xff) << 8]);
    }
}

/// { 0x702050 } (ret 0x18, match_pl.cpp, ~35 KB) com as chamadas que mexem fora do motor injetadas:
/// o chute de `p` (tipo a1, valor a2, narracao a3, modo a4: 1 jogo, 2 sem desafiante, 3 penaltis, 4
/// cobranca, 5/6 repetir; a5/a6 a lista dos companheiros da pressao).
#[allow(clippy::too_many_arguments, clippy::cognitive_complexity)]
unsafe fn chutar_com(p: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: usize, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let b = |q: usize, o: usize| (q + o) as *mut u8;
    let w = |q: usize, o: usize| (q + o) as *mut i16;
    let ub = |q: usize, o: usize| *((q + o) as *const u8) as u32;
    let u = |q: usize, o: usize| *((q + o) as *const usize);
    let dw = |q: usize, o: usize| (q + o) as *mut i32;
    let m = u(p, 0x19e);
    let lado = byte(p, 0x27);
    let tipo = a1 as u8;
    let mut v = a2 as i32;
    let mut som = a3;
    let mut modo = a4 as u8;
    let n7 = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: i8| narrar_finalizacao(chamar, m, codigo, a3, a4, a5, a6, a7);
    // 0x7002f0: dois campos -1; 0x70a860: tres
    let na = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, a3: u32, a4: u32, a7: i8| narrar_finalizacao(chamar, m, codigo, a3, a4, 0xff, 0xff, a7);
    let l60 = *dw(m, 0x4786);
    let l5c = *dw(m, 0x4782);
    let alvo_y = if lado == 0 { 0xb } else { 0 };
    let d = distancia_ao_ponto(p, 4, alvo_y);
    let (l58, l50, l54) = (*w(p, 0x39), byte(p, 0x102), byte(p, 0x103));
    let l8c = *w(p, 0x198);
    let mut l64: i32 = 0xf0;
    let v0 = a2;
    let mut l28: i8 = 6;
    let l25 = quantos_marcam(m, p);
    let minuto = (*w(m, 0x8ed0) as i32 / 11) as i16;
    let l16 = minuto as i8;
    *b(m, 0x8d4e) = lado as u8;
    *w(m, 0x8d4c) = minuto;
    let l18 = !((*dw(m, 0x4786) as f64) < f64_em(0x96d810));
    let l1b = ((ftol(d as f64 * f32_em(0x96d754) as f64 + f64_em(0x968e18)) as i8 as i32 / 5) * 5) as i8;
    if tipo == 0x39 || tipo == 3 {
        l28 = 5;
    }
    if tipo == 0x33 {
        l64 = 0x78;
    }
    let d2 = distancia_ao_ponto(p, 4, alvo_y);
    let mut l7c = 0u16;
    let mut l10 = classificar_lance(p, tipo as i8 as i16, l50, l54, 4, if lado == 1 { 0 } else { 0xb }, (d2 as f64 + f64_em(0x967890)) as f32, &mut l7c);
    *w(p, 0x39) = (*w(p, 0x39)).wrapping_add(-(l7c as i16 as i32 / 2) as i16);
    let r = sortear(l7c as i16 as i32);
    v = v.wrapping_sub(r);
    // quem deu o passe ganha a assistencia de chute
    let mut l68 = 0u32;
    let q = u(m, 0xf5a6);
    if p == u(m, 0xf5aa) && q != 0 && !matches!(*w(p, 0x198), 0x12 | 0x11) && (*w(p, 0x29) > -100 || v as i16 > 100) && modo == 1 {
        if !matches!(*w(q, 0x198), 0x34 | 0x37) {
            crate::comparacao::ramo("chute assistencia");
            *b(q, 3) = (*b(q, 3)).wrapping_add(1);
            *w(q, 0x35) = (*w(q, 0x35)).wrapping_add((*w(p, 0x29) as i32 / 3 + 0xe1) as i16);
            *dw(m, 0x4782) = (*dw(m, 0x4782)).wrapping_add(0x113);
        }
        l68 = 1;
    }
    let g = manter_posicao_um(m, (lado == 0) as i8);
    if g == 0 {
        // ponytail: o original para o jogo (match_pl.cpp linha 0x2e0b); sem goleiro nao ha chute
        linha("motor: chutar: sem goleiro");
        std::process::abort();
    }
    let l1c: u8 = match tipo {
        0x16 | 0x39 => 0,
        0x17 | 0x1a | 0x1b => 1,
        0x18 | 0x19 | 0x1c => (sortear(3) != 0) as u8,
        0x3a => (sortear(4) == 0) as u8,
        _ => sortear(2) as u8,
    };
    let (mut l20, mut l14) = (0u8, 0u8);
    let mut l38 = defesa_do_goleiro(g, l50, l54, l1c, tipo as i8, &mut l20, &mut l14) as i16 as i32;
    let (mut l19, mut l1a) = (false, false);
    if l14 == 0 {
        if l38 as i16 > 0x177 {
            l1a = true;
        } else if l38 as i16 > 0x104 {
            l19 = true;
        }
    }
    // o desafiante
    let (mut l4c, mut l4d, mut l70, mut l30, mut l84) = (0u8, 0u8, 0i16, 0i8, 0i8);
    let e = if byte(m, 0x8ea9) != 4 && modo != 4 && tipo != 0x33 && tipo != 0x35 {
        desafio_adversario(m, p, lado, 0x6d, &mut l4c, &mut l4d, &mut l70, &mut l30, &mut l84, l20)
    } else {
        0
    };
    lesao_na_dividida_com(m, p, 0x6d, e, l30 as i16, 0, chamar);
    if e != 0 {
        let mut penalti = false;
        let mut l44 = 0u8;
        'disputa: {
            if l70 < *w(p, 0x19c) {
                // sem falta: a disputa limpa
            } else {
                let (mut l88, mut l74) = (0u8, 0i8);
                let mut l2c = falta_no_motor(m, e, p, l30, -1, &mut l88, &mut l74, &mut l44);
                if byte(m, 0x8ebb) != 0
                    && na_area_atacada(lado, byte(e, 0x102), byte(e, 0x103))
                    && (codigo_especial(l30) != 0 || l44 != 0 || sortear(*w(e, 0x31) as i32) < -10)
                    && sortear(3) != 0
                {
                    n7(chamar, 0x2168, ub(e, 0x2e), ub(e, 0x27), 0xff, 0xff, 0);
                }
                if l2c != 0 {
                    if sortear(0x32) == 0 && modo != 4 && l84 == 0 {
                        crate::comparacao::ramo("chute falta so registra");
                        registrar_lance(m, l30 as i16 as u16, e as u32, p as u32, 0, l2c, l50 as u8, l54 as u8, l88, l74 as u8);
                        modo = 2;
                        break 'disputa;
                    }
                    crate::comparacao::ramo("chute falta punida");
                    let mut peso = 0f32;
                    motor_punir_lance(m, e, p, &mut l2c, l88 as i8, l74, l84 as u8, &mut peso, l44);
                    n7(chamar, narracao_da_falta(l30), ub(e, 0x2e), ub(e, 0x27), ub(p, 0x2e), ub(p, 0x27), 0);
                    if modo == 4 {
                        // na cobranca, a falta vira tiro livre: o chute de novo (tipo 0x33)
                        crate::comparacao::ramo("chute falta na cobranca");
                        n7(chamar, 0x1f7a, 0xff, lado as u8 as u32, 0xff, 0xff, 0);
                        let mut t = 0x33i8;
                        let mut s = som as u16;
                        let val = valor_do_chute(p, &mut t, &mut s);
                        let novo_som = (som & 0xffff_0000) | s as u32;
                        chamar(0x702050, p, [(a1 & 0xffff_ff00) | t as u8 as u32, val as i16 as i32 as u32, novo_som, 4, 0]);
                        return;
                    }
                    falta_no_chute_com(m, e, p, l2c, l74, l44, l30, peso, chamar);
                    return;
                }
            }
            penalti = l44 != 0 && (faixa_jogador(p, (lado == 0) as u8) as u8 != 0 || l84 != 0);
        }
        // a disputa limpa (0x702675)
        *w(e, 0x198) = l30 as i16;
        *b(e, 4) = (*b(e, 4)).wrapping_add(1);
        let t = tabela_a(m, e);
        *w(e, 0x35) = (*w(e, 0x35)).wrapping_add(if t & 0x30 != 0 && tabela_a(m, e) & 0x880 != 0 { -0x3b } else { -0x46 });
        if v as i16 > l70 {
            crate::comparacao::ramo("chute passa pelo desafiante");
            *b(e, 0x2b) = (*b(e, 0x2b)).wrapping_add(l4d);
        } else {
            crate::comparacao::ramo("chute desarmado");
            chute_desarmado_com(m, p, e, l30, l70, v, l4c, l44, penalti, chamar);
            return;
        }
    } else if !matches!(tipo, 0x33 | 0x35 | 3 | 0x39) && modo != 4 {
        // sem desafiante: a narracao da aproximacao
        if faixa_jogador(p, (lado == 0) as u8) as u8 != 0 && a5 as u8 == 0 && codigo_de_bola_parada(*w(p, 0x198)) != 0 {
            na(chamar, 0x1f43, ub(p, 0x2e), lado as u8 as u32, 0);
        } else if sortear(10) == 0 && (0x14..=0x23).contains(&l1b) && faixa_jogador(p, (lado == 0) as u8) as u8 == 0 && byte(p, 0x102) < 6 && byte(p, 0x102) > 2 {
            na(chamar, 0x2161, ub(p, 0x2e), lado as u8 as u32, l1b);
        }
    }
    // 0x703634: a narracao do chute
    if l10 == byte(p, 0x2c) as u8 && sortear(10) != 0 {
        l10 = 0xff;
    }
    let tg = || if u(g, 0x19e) != 0 { tabela_a(u(g, 0x19e), g) } else { 0 };
    if tipo == 3 {
        if a5 as u8 == 0 && e == 0 {
            na(chamar, som, ub(p, 0x2e), lado as u8 as u32, 7);
        } else {
            som = pela_classe(som, l10);
            na(chamar, som, ub(p, 0x2e), lado as u8 as u32, 0);
        }
    } else if tipo == 0x1b || tipo == 0x1a {
        if tg() != 1 {
            na(chamar, som, ub(p, 0x2e), lado as u8 as u32, 0);
        } else if l20 == 4 && distancia_ao_ponto(p, byte(g, 0x102), byte(g, 0x103)) as f64 > f64_em(0x968940) {
            crate::comparacao::ramo("chute goleiro adiantado");
            n7(chamar, 0x1fe0, ub(p, 0x2e), lado as u8 as u32, ub(g, 0x2e), (lado == 0) as u32, 0);
            n7(chamar, som, ub(p, 0x2e), lado as u8 as u32, ub(g, 0x2e), ub(g, 0x27), 0);
            l20 = 0;
        } else {
            let novo = match som as u16 as i16 as i32 {
                0x1f89 => Some(0x1f8a),
                0x1f8b => Some(0x1f8c),
                0x1fa3 => Some(0x1fa4),
                0x1fa5 => Some(0x1fa6),
                0x1fb1 => Some(0x1fb2),
                0x1fb3 => Some(0x1fb4),
                _ => None,
            };
            if let Some(c) = novo {
                som = c;
            }
            l20 = 0;
            n7(chamar, som, ub(p, 0x2e), lado as u8 as u32, ub(g, 0x2e), ub(g, 0x27), 0);
        }
    } else {
        som = pela_classe(som, l10);
        na(chamar, som, ub(p, 0x2e), lado as u8 as u32, 0);
    }
    // 0x703883: o impedimento
    *w(p, 0x198) = 0x6d;
    if tipo != 0x33 && tipo != 0x35 && modo == 1 {
        let o = motor_procurar_impedimento(m, lado, byte(p, 0x103), if lado == 0 { 0xb } else { 0 }, p);
        if o != 0 {
            if sortear(500) != 0 {
                crate::comparacao::ramo("chute impedimento registrado");
                registrar_lance(m, 0xffff, o as u32, 0, 0, 8, *b(o, 0x102), *b(o, 0x103), 0xff, 0xff);
                modo = 2;
            } else {
                crate::comparacao::ramo("chute impedimento");
                na(chamar, 0x1f78, ub(o, 0x2e), lado as u8 as u32, 0);
                let k = m + 0xf5ca + (lado as isize * 0x36) as usize;
                *b(k, 0) = (*b(k, 0)).wrapping_add(1);
                *b(o, 0x13) = (*b(o, 0x13)).wrapping_add(1);
                *b(m, 0x8ea7) = *b(o, 0x102);
                *b(m, 0x8ea8) = *b(o, 0x103);
                *b(m, 0x8eb2) = 7;
                *b(m, 0x8eae) = (lado == 0) as u8;
                *((m + 0xf582) as *mut usize) = 0;
                return;
            }
        }
    }
    *b(m, 0x8eae) = 0xff;
    *b(m, 0x1a7) = 0;
    let l40 = *w(p, 0x29);
    *w(p, 0x29) = 0;
    *w(p, 0x19c) = 0;
    let mut l11 = 0u8;
    if tipo != 0x33 && modo != 4 {
        let forca = if (v as i16) < 0 { 0 } else { v as i16 as i32 };
        l11 = chamar(0x6cc540, m, [l50 as u8 as u32 | (l54 as u8 as u32) << 8, forca as u32, (lado == 0) as u32, l28 as u8 as u32, (tipo == 0x35) as u32]) as u8;
        if byte(m, 0x8eb2) != 0 {
            return;
        }
    }
    if u(m, 0xf582) == p {
        *((m + 0xf582) as *mut usize) = 0;
    }
    if l11 == 2 {
        // o chute bateu em alguem no caminho
        crate::comparacao::ramo("chute bloqueado");
        let q = u(m, 0xf5ae);
        if l28 == 5 {
            na(chamar, 0x1fd4, ub(q, 0x2e), ub(q, 0x27), 5);
        } else {
            na(chamar, 0x1fd3, ub(q, 0x2e), ub(q, 0x27), l28);
        }
        let s1 = sortear(2000);
        let s2 = sortear(1000);
        v = v.wrapping_add(s2.wrapping_sub(s1));
        *w(p, 0x39) = 0;
    }
    // a classe do desfecho
    let tem = l1c != 0;
    l10 = if v as i16 > l64 as i16 {
        if sortear(v as i16 as i32) < 10 { 0xe + tem as u8 } else { 1 }
    } else if (v as i16) < 10 {
        if tem { if sortear(4) != 0 { 3 } else { 4 } } else { 2 }
    } else {
        let dif = l64 as i16 as i32 - v as i16 as i32;
        if dif > 0x3c {
            if tem { if sortear(4) != 0 { 6 } else { 7 } } else { 5 }
        } else if dif > 0x19 {
            8 + tem as u8
        } else if dif > 0xe {
            0xa + tem as u8
        } else {
            0xc + tem as u8
        }
    };
    let no_alvo = matches!(l10, 1 | 0xe | 0xf);
    match modo {
        3 | 4 => {
            let k = m + 0x1a9 + lado as isize as usize;
            *b(k, 0) = (*b(k, 0)).wrapping_add(1);
        }
        1 => {
            if tipo == 0x33 {
                *b(p, 0xd) = (*b(p, 0xd)).wrapping_add(1);
            }
            let q = u(m, 0xf5ae);
            if tipo == 0x33 || q == 0 || (byte(q, 0x102) == 4 && (byte(q, 0x103) == 0 || byte(q, 0x103) == 0xb)) {
                *b(p, 0xa) = (*b(p, 0xa)).wrapping_add(1);
                *w(p, 0x35) = (*w(p, 0x35)).wrapping_add(if tipo != 0x33 { -150 } else { -1000 });
                if no_alvo {
                    *b(p, 0xb) = (*b(p, 0xb)).wrapping_add(1);
                    let k: i32 = if tipo != 0x33 { 0xdc } else { 400 };
                    if l10 == 0xe || l10 == 0xf {
                        *w(p, 0x35) = (*w(p, 0x35)).wrapping_add(ftol(k as f64 * f64_em(0x968968)) as i16);
                    } else {
                        *w(p, 0x35) = (*w(p, 0x35)).wrapping_add(k as i16);
                    }
                }
            }
            if !no_alvo {
                let di = ((*w(p, 0x39)).wrapping_sub(v as i16).wrapping_add(0xf0)).clamp(0, 200);
                *w(p, 0x35) = (*w(p, 0x35)).wrapping_sub(di);
                if sortear(byte(u(p, 0x69), 0x5a) as i32) == 0 && ajustar_17(p, (-(di as i32 / 50)) as i8) != 0 {
                    desempenho_na_partida(p, 0);
                }
            }
        }
        _ => {}
    }
    if l11 == 1 {
        // a bola parou no caminho
        let (bx, by) = (byte(m, 0x8ea7), byte(m, 0x8ea8));
        if bx < 0 || by >= 0xc || (bx >= 9 && by < 0) {
            chamar(0x6cd490, m, [(lado == 0) as u32, 0, 0, 0, 0]);
        }
        *b(m, 0x1cd) = 1;
        return;
    }
    if lado == 1 {
        *b(m, 0x8ea7) = 4;
        *b(m, 0x8ea8) = 0;
    } else if lado == 0 {
        *b(m, 0x8ea7) = 4;
        *b(m, 0x8ea8) = 0xb;
    }
    l64 = l1b as i32;
    if l1b as i32 > sortear(0xf) + 0x19 {
        // chute de longe: todos decidem de novo com a bola parada 5
        crate::comparacao::ramo("chute de longe");
        let guardado = *b(m, 0x8ea9);
        *b(m, 0x8ea9) = 5;
        for l in 0..2i8 {
            for k in 0..20 {
                let j = jogador_do_bloco(m, l, k);
                if (0..=10).contains(&byte(j, 0x19)) && byte(j, 0x2b) == 0 && j != p {
                    chamar(0x7103e0, j, [0; 5]);
                }
            }
        }
        *b(m, 0x8ea9) = guardado;
    }
    if l20 != 0 {
        if modo == 1 && tg() == 1 {
            *w(g, 0x35) = (*w(g, 0x35)).wrapping_sub(800);
            if sortear(byte(u(g, 0x69), 0x5a) as i32 * 20 + 10) == 0 {
                let r = sortear(3);
                ajustar_17(g, (r as u8).wrapping_neg() as i8);
            }
            desempenho_na_partida(g, 0);
        }
        if (lado == 0 && byte(p, 0x102) > byte(g, 0x102)) || (lado == 1 && byte(p, 0x102) < byte(g, 0x102)) {
            v = sortear(400).wrapping_add(v).wrapping_add(0xbe);
        } else {
            v = v.wrapping_add(0xbe);
        }
    }
    if u(m, 0xf5ae) == g {
        l38 = 0;
    }
    if tipo == 0x39 {
        *b(p, 0x2b) = (*b(p, 0x2b)).wrapping_add(2);
    }
    let (mut l12, mut l13, mut l26, mut l4e, mut l4f) = (0i8, 0i8, 0u8, 0u8, 0u8);
    placar_agregado(m, lado, &mut l12, &mut l13, &mut l26, &mut l4e, &mut l4f, 1);
    let l38_antes = l38 as i16;
    l38 = (l38 as i16 as i32) - if l14 != 0 { 100 } else { 0 } - *w(p, 0x39) as i32 + v as i16 as i32 - 0x64;
    let fator = match tipo {
        5 => f32_em(0x96907c),
        0x17 | 0x1c => f32_em(0x96b004),
        0x18 => f32_em(0x96b008),
        0x19 => f32_em(0x96b00c),
        0x1a | 0x39 => f32_em(0x96d420),
        0x1b => f32_em(0x9689a8),
        0x35 => f32_em(0x969584),
        0x3a => f32_em(0x968f18),
        _ => f32_em(0x968e70),
    } as f64;
    let ganho = if l38 < 0 { l38 as f64 / fator } else { l38 as f64 * fator };
    *dw(m, 0x4786) = (*dw(m, 0x4786)).wrapping_add(ftol(ganho));
    let c = Chute {
        p, m, g, lado, tipo, a1, v, v0, som, modo, conta: a5 as u8 as i8, lista: a6, d, l10, l11, l14, l16,
        l17: 0, l18, l19, l1a, l1b, l20, l25, l28, l38_antes, l40, l50, l54, l58, l5c, l60, l64, l68,
        e, l8c, l12, l13, l15: 0, l26, l4e, l4f,
    };
    if no_alvo {
        chute_no_alvo_com(c, chamar);
        return;
    }
    chute_fora_com(&c, chamar);
}

/// A narracao do chute pela classe (0x7037ee): os codigos 0x1f86, 0x1f8e, 0x1f93, 0x1f96 e 0x1f99
/// passam ao seguinte (+1 com a classe diferente de zero); a classe 0xff (a do lance repetido) nao muda.
fn pela_classe(som: u32, classe: u8) -> u32 {
    if classe == 0xff {
        return som;
    }
    let k = (classe != 0) as u32;
    match som as u16 as i16 as i32 {
        0x1f86 => 0x1f87 + k,
        0x1f8e => 0x1f8f + k,
        0x1f93 => 0x1f94 + k,
        0x1f96 => 0x1f97 + k,
        0x1f99 => 0x1f9a + k,
        _ => som,
    }
}

/// A falta do desafiante no chute (0x702050, de 0x702934 a 0x702f1a): como a do passe, sem o evento
/// 0x82 e sem as narracoes do lance 0x2c/0x2f; o perto do gol vem de 0x70df70.
#[allow(clippy::too_many_arguments)]
unsafe fn falta_no_chute_com(m: usize, e: usize, p: usize, tipo: u8, grav: i8, vantagem: u8, lance: i8, peso: f32, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let b = |q: usize, o: usize| (q + o) as *mut u8;
    let w = |q: usize, o: usize| (q + o) as *mut i16;
    let ub = |q: usize, o: usize| *((q + o) as *const u8) as u32;
    let u = |q: usize, o: usize| *((q + o) as *const usize);
    let lado = byte(p, 0x27);
    let n7 = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, a3: u32, a4: u32| narrar_finalizacao(chamar, m, codigo, a3, a4, 0xff, 0xff, 0);
    let na = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, a3: u32, a4: u32| narrar_finalizacao(chamar, m, codigo, a3, a4, 0xff, 0xff, 0);
    let nb = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, a4: u32| narrar_finalizacao(chamar, m, codigo, 0xff, a4, 0xff, 0xff, 0);
    let cor: i32 = match tipo {
        2 => 0x1ff9,
        3 => 0x1ff7,
        4 => 0x1ff8,
        5 => 0x1ff6,
        _ => {
            if grav as i32 > sortear(7) + 2 { 0x210a } else { -1 }
        }
    };
    let som: u32;
    if tipo == 7 {
        som = 0x1f79;
        *b(m, 0x8eb2) = 7;
    } else if na_area_atacada(lado, byte(p, 0x102), byte(p, 0x103)) {
        crate::comparacao::ramo("chute penalti");
        if cor as i16 == -1 {
            match sortear(4) {
                0 => n7(chamar, 0x20e9, 0xff, lado as u8 as u32),
                1 => n7(chamar, 0x20ec, 0xff, lado as u8 as u32),
                _ => {}
            }
        }
        som = 0x1f7a;
        *b(m, 0x8eb2) = 5;
        *((m + 0x478a) as *mut usize) = p;
        *w(e, 0x35) = (*w(e, 0x35)).wrapping_sub(500);
        if sortear(byte(u(e, 0x69), 0x5a) as i32) == 0 {
            let r = sortear(10);
            if ajustar_17(e, (r as u8).wrapping_neg() as i8) != 0 {
                desempenho_na_partida(e, 0);
            }
        }
    } else {
        som = 0x1f79;
        *b(m, 0x8eb2) = 6;
    }
    *b(m, 0x8ea7) = *b(p, 0x102);
    *b(m, 0x8ea8) = *b(p, 0x103);
    *b(m, 0x8eae) = *b(p, 0x27);
    *((m + 0xf582) as *mut u32) = 0;
    if som == 0x1f7a {
        n7(chamar, 0x1f7a, 0xff, lado as u8 as u32);
    }
    let expulso = matches!(tipo, 3..=6);
    if cor as i16 > -1 {
        if sortear(4) == 0 {
            let x = sortear(grav as i32);
            if x > sortear(0x14) {
                let t = u(m, 0x4792);
                let clube = u(t, if byte(e, 0x27) == 0 { 0x20 } else { 0x1c });
                let k = u(clube, 0x53);
                if k != 0 && byte(k, 0x77) == 8 {
                    n7(chamar, 0x20f5, 0, 0);
                }
            }
            n7(chamar, 0x2109, ub(e, 0x2e), ub(e, 0x27));
        }
        n7(chamar, cor as u32, ub(e, 0x2e), ub(e, 0x27));
        if expulso {
            crate::comparacao::ramo("chute expulsao");
            jogador_saiu_com(m, e, 0, chamar);
        }
    }
    if som != 0x1f7a {
        let c = escolher_cobrador(m, lado, byte(m, 0x8eb2), 0);
        if c == 0 {
            n7(chamar, som, 0xff, lado as u8 as u32);
        } else {
            if tipo == 7 {
                n7(chamar, 0x21e2, ub(c, 0x2e), lado as u8 as u32);
            } else {
                n7(chamar, som, 0xff, lado as u8 as u32);
            }
            if som == 0x1f79 && vantagem == 0 {
                if tipo == 7 {
                    if perto_do_gol(p) != 0 && faixa_jogador(p, (lado == 0) as u8) as u8 == 0 {
                        n7(chamar, 0x21e1, ub(c, 0x2e), lado as u8 as u32);
                    }
                } else if fora_do_meio(byte(p, 0x102)) != 0 && perto_do_gol(p) != 0 {
                    na(chamar, 0x21df, ub(c, 0x2e), lado as u8 as u32);
                } else if perto_do_gol(p) != 0 {
                    na(chamar, 0x21e0, ub(c, 0x2e), lado as u8 as u32);
                }
            }
        }
    }
    if codigo_especial(lance) == 0 {
        let extra = expulso || som == 0x1f7a;
        let zero = (lado == 0) as u32;
        if vantagem != 0 {
            na(chamar, 0x20dd, ub(p, 0x2e), lado as u8 as u32);
            if tipo == 4 || tipo == 5 {
                *b(e, 0x16) = 1;
            }
            if extra && sortear(2) == 0 {
                nb(chamar, 0x20d9, zero);
            }
        } else if sortear(9) == 0 {
            nb(chamar, 0x20df, zero);
            if sortear(3) == 0 {
                nb(chamar, 0x20e0, zero);
            }
            if extra && sortear(2) == 0 {
                nb(chamar, 0x20d9, zero);
            }
        }
        aplicar_ao_time(m, e, peso);
        chamar(0x716e10, e, [0; 5]);
    }
    *w(e, 0x35) = (*w(e, 0x35)).wrapping_sub(0x19);
}

/// O desafiante tirou a bola no chute (0x702050, de 0x702f35 a 0x70353b): a narracao (na area dele,
/// 1 em 2, a da falta e 0x20e8/0x20ea/0x20eb; senao pelo lance, com a defesa 0x70d430 no lance 1),
/// ele domina (0x6f0630, 0x70a890 com nota - chute - 200), a bola espirra abaixo do limite do lance
/// (0x6cd770) ou ele aponta para o lado do campo; os ganhos e o +0x2b de quem chutou.
#[allow(clippy::too_many_arguments)]
unsafe fn chute_desarmado_com(m: usize, p: usize, e: usize, lance: i8, nota: i16, v: i32, a: u8, vantagem: u8, penalti: bool, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let b = |q: usize, o: usize| (q + o) as *mut u8;
    let w = |q: usize, o: usize| (q + o) as *mut i16;
    let ub = |q: usize, o: usize| *((q + o) as *const u8) as u32;
    let u = |q: usize, o: usize| *((q + o) as *const usize);
    let lado = byte(p, 0x27);
    let n7 = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, a3: u32, a4: u32, a5: u32, a6: u32| narrar_finalizacao(chamar, m, codigo, a3, a4, a5, a6, 0);
    let nb = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, a4: u32| narrar_finalizacao(chamar, m, codigo, 0xff, a4, 0xff, 0xff, 0);
    let narrou = (codigo_especial(lance) != 0 || vantagem != 0 || sortear(*w(e, 0x31) as i32) < -10)
        && faixa_jogador(e, byte(e, 0x27) as u8) as u8 != 0
        && sortear(2) == 0;
    if narrou {
        crate::comparacao::ramo("chute desarme narrado na area");
        n7(chamar, narracao_da_falta(lance), ub(e, 0x2e), ub(e, 0x27), ub(p, 0x2e), ub(p, 0x27));
        let zero = (byte(e, 0x27) == 0) as u32;
        match sortear(0x32) {
            0 => nb(chamar, 0x20eb, zero),
            1 => nb(chamar, 0x20ea, zero),
            _ => nb(chamar, 0x20e8, zero),
        }
    } else {
        let codigo = match lance {
            1 => {
                crate::comparacao::ramo("chute desarme do goleiro");
                if defender_com(e, (nota as i32).wrapping_sub(v) as i16, chamar) == 0 {
                    return;
                }
                0
            }
            0x13 | 0x21 | 0x2d => 0x1f62,
            0x22 | 0x2c => 0x2150,
            0x29 => 0x1f61,
            0x2a | 0x2b => 0x1f64,
            0x15 | 0x2e => {
                let r = sortear(6);
                if (r as f64 + f32_em(e + 0xad) as f64 + f32_em(0x969584) as f64) < f32_em(p + 0xad) as f64 { 0x2164 } else { 0x1f5d }
            }
            _ => 0,
        };
        if codigo != 0 {
            n7(chamar, codigo, ub(e, 0x2e), ub(e, 0x27), ub(p, 0x2e), ub(p, 0x27));
        }
    }
    chamar(0x6f0630, e, [0; 5]);
    gravar_19c(e, (nota as i32).wrapping_sub(v).wrapping_sub(200) as i16);
    let mut dominou = false;
    let limite: i16 = match lance {
        0x13 => 400,
        0x21 | 0x2d => 0xfa,
        _ => 100,
    };
    if *w(e, 0x19c) < limite && byte(m, 0x8ea9) != 4 && byte(m, 0x8ea9) != -1 {
        crate::comparacao::ramo("chute bola espirra");
        chamar(0x6cd770, m, [ub(p, 0x102), ub(p, 0x103), 1 | ((lado == 0) as u32) << 8, 0, 0]);
        if byte(m, 0x8eb2) == 0 {
            let mut k = 3;
            let (c10d, c10c) = (byte(e, 0x10d), byte(e, 0x10c));
            k -= (c10d as i32 > sortear(10)) as i32;
            k -= (c10c as i32 > sortear(10)) as i32;
            *b(e, 0x2b) = sortear(k) as u8;
            dominou = true;
        } else if penalti {
            chamar(0x716c90, p, [0; 5]);
        }
    } else {
        if byte(m, 0x8ea9) < 4 && byte(m, 0x8ea9) > -1 {
            *b(m, 0x8ea9) = 0;
        }
        let (x, y) = (byte(p, 0x102), byte(p, 0x103));
        if byte(e, 0x27) == 0 && byte(m, 0x8ea9) != -1 {
            apontar(e, x, y, 4, if y < 9 { y + 3 } else { 11 });
        } else {
            apontar(e, x, y, 4, if y > 3 { y - 3 } else { 0 });
        }
    }
    *b(e, 5) = (*b(e, 5)).wrapping_add(1);
    let me = u(e, 0x19e);
    let t = || if me != 0 { tabela_a(me, e) } else { 0 };
    let ganho: i16 = if dominou { 0x78 } else { 0xc8 };
    let longe = t() & 4 == 0 && t() & 2 == 0;
    *w(e, 0x35) = (*w(e, 0x35)).wrapping_add(if longe { ganho << 1 } else { ganho });
    *b(p, 0x2b) = (*b(p, 0x2b)).wrapping_add(a);
    if faixa_jogador(p, (lado == 0) as u8) as u8 != 0 {
        *b(e, 6) = (*b(e, 6)).wrapping_add(1);
    }
    if dominou && sortear(7) == 0 {
        narrar_finalizacao(chamar, m, 0x1f41, ub(e, 0x2e), ub(e, 0x27), 0xff, 0xff, 0);
    }
}

/// O nome (+0x69 -> +8) da ficha e "Shearer" (0xa16110): a comemoracao dele.
unsafe fn e_o_shearer(p: usize) -> bool {
    let ficha = *((p + 0x69) as *const usize);
    let nome = *((ficha + 8) as *const *const u8);
    std::ffi::CStr::from_ptr(nome as *const std::ffi::c_char).to_bytes() == b"Shearer"
}

/// Os destaques do jogador (0x717470), na ordem: o primeiro que valer e narrado com (q.2e, a4, 0).
unsafe fn destaques(chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, m: usize, q: usize, a4: u32, codigos: &[u32]) -> bool {
    for &c in codigos {
        if destaque_do_jogador(q, c) != 0 {
            narrar_finalizacao(chamar, m, c, byte(q, 0x2e) as u8 as u32, a4, 0xff, 0xff, 0);
            return true;
        }
    }
    false
}

const DESTAQUES_P: [u32; 4] = [0x219e, 0x21b1, 0x21b2, 0x21b7];
const DESTAQUES_R: [u32; 6] = [0x219f, 0x21a0, 0x21b0, 0x21b3, 0x21b4, 0x21b5];
const DESTAQUES_GOL: [u32; 7] = [0x219f, 0x21a0, 0x21a1, 0x21b0, 0x21b3, 0x21b4, 0x21b5];

/// O chute no alvo (0x702050 de 0x706cba ao fim): a defesa do goleiro ou o gol.
#[allow(clippy::cognitive_complexity)]
unsafe fn chute_no_alvo_com(mut c: Chute, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let b = |q: usize, o: usize| (q + o) as *mut u8;
    let w = |q: usize, o: usize| (q + o) as *mut i16;
    let ub = |q: usize, o: usize| *((q + o) as *const u8) as u32;
    let u = |q: usize, o: usize| *((q + o) as *const usize);
    let dw = |q: usize, o: usize| (q + o) as *mut i32;
    let (p, m, g, lado) = (c.p, c.m, c.g, c.lado);
    let lp = lado as u8 as u32;
    let na = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, a3: u32, a4: u32, a7: i8| narrar_finalizacao(chamar, m, codigo, a3, a4, 0xff, 0xff, a7);
    let tg = || if u(g, 0x19e) != 0 { tabela_a(u(g, 0x19e), g) } else { 0 };
    let ajustar = |q: usize, delta: i32| {
        if ajustar_17(q, delta as i8) != 0 {
            desempenho_na_partida(q, 0);
        }
    };
    // o gol pela razao chute / (defesa + chute)
    let l6c = (c.l38_antes as i32).wrapping_add(c.v as i16 as i32);
    let mut razao: f32;
    let gol = if l6c != 0 {
        razao = ((c.v as i16 as i32).wrapping_mul(100) as f64 / l6c as f64) as f32;
        if m != 0 && !(razao as f64 > f32_em(0x96d80c) as f64) && matches!(byte(m, 1), 2 | 8) {
            razao = (razao as f64 + f32_em(0x9689a8) as f64) as f32;
        }
        razao as f64 > f32_em(0x96d808) as f64
    } else {
        razao = 0.0;
        false
    };
    if !gol {
        if c.l19 && tg() == 1 {
            *w(g, 0x35) = (*w(g, 0x35)).wrapping_add(0x15e);
            if sortear(byte(g, 0x17) as i32 * 20 + 0x32) == 0 {
                let r = sortear(4);
                ajustar(g, r);
            }
        } else if c.l1a && tg() == 1 {
            *w(g, 0x35) = (*w(g, 0x35)).wrapping_add(0x258);
            if sortear((byte(g, 0x17) as i32 + 1) * 20) == 0 {
                let r = sortear(7);
                ajustar(g, r);
            }
        }
        if !(razao as f64 > f32_em(0x96d808) as f64) {
            defesa_no_chute_com(c, razao, chamar);
            return;
        }
    }
    // o gol (0x706dfd)
    crate::comparacao::ramo("chute gol");
    let modo = c.modo;
    if modo == 1 {
        if lado_do_lance(m) as i8 > -1 {
            limpar_lance(m);
        }
        let k = u(m, 0x478a);
        if c.tipo == 0x33 && k != 0 && k != p {
            *b(k, 0x10) = (*b(k, 0x10)).wrapping_add(1);
        }
        let base = m + (lado as isize * 0x36) as usize;
        match byte(m, 0x8eb1) {
            -1 | 7 => *b(base, 0xf5c4) = (*b(base, 0xf5c4)).wrapping_add(1),
            3 => *b(base, 0xf5c3) = (*b(base, 0xf5c3)).wrapping_add(1),
            6 => *b(base, 0xf5c5) = (*b(base, 0xf5c5)).wrapping_add(1),
            _ => {}
        }
        if momento_marcado(m) as u8 != 0 {
            *b(m, 0x1a8) = 1;
        }
        if c.tipo == 0x33 {
            *b(p, 0xe) = (*b(p, 0xe)).wrapping_add(1);
        }
    } else if modo == 3 || modo == 4 {
        gol_nos_penaltis_com(&c, chamar);
        gol_validado_com(&mut c, chamar);
        return;
    }
    if c.l11 == 0 {
        for o in [0x3d, 0x3f, 0x41, 0x43, 0x45] {
            *w(p, o) = (*w(p, o)).wrapping_add(1);
        }
        *b(p, 0xc) = (*b(p, 0xc)).wrapping_add(1);
        c.l17 = 1;
    }
    if modo == 1 {
        if c.l11 == 0 {
            if c.tipo == 0x33 {
                *w(p, 0x35) = (*w(p, 0x35)).wrapping_add(0x2bc);
            } else {
                *w(p, 0x35) = (*w(p, 0x35)).wrapping_add(0x28a);
                if sortear((byte(p, 0x17) as i32 + 1) * 20) == 0 {
                    let r = sortear(8);
                    ajustar(p, r);
                }
            }
            let piso = (ub(p, 0xc) as i32 + 5) * 1000;
            if (*w(p, 0x35) as i32) <= piso {
                *w(p, 0x35) = piso as i16;
            }
        } else {
            *b(p, 0x10) = (*b(p, 0x10)).wrapping_add(1);
            *w(p, 0x35) = (*w(p, 0x35)).wrapping_add(0x113);
            if sortear(byte(p, 0x17) as i32 * 20 + 0x32) == 0 {
                let r = sortear(4);
                ajustar(p, r);
            }
            let piso = ftol((ub(p, 0x10) as f64 * f32_em(0x968f18) as f64 + f64_em(0x96afa0)) * f64_em(0x968908)) as i16;
            if *w(p, 0x35) <= piso {
                *w(p, 0x35) = piso;
            }
        }
        if tg() == 1 {
            // o goleiro sofreu o gol
            *b(g, 0xf) = (*b(g, 0xf)).wrapping_add(1);
            let ficha = u(g, 0x69);
            if sortear(byte(ficha, 0x58) as i32 + 1) != 0 && sortear(byte(ficha, 0x5a) as i32 + 1) != 0 {
                let r = sortear(ub(g, 0xf) as i32 * ub(g, 0xf) as i32) as i8;
                let soma = if (r as i32) < ub(p, 0xf) as i32 { *b(p, 0xf) } else { r as u8 };
                *b(g, 0x100) = (*b(g, 0x100)).wrapping_add(soma);
                desempenho_na_partida(g, 0);
            }
            if c.tipo != 0x33 {
                *w(g, 0x35) = (*w(g, 0x35)).wrapping_sub(500);
            }
            let di = *w(g, 0x35);
            if c.l14 != 0 {
                let mut v = (c.d as f64 + f64_em(0x9689e0)) * f64_em(0x96d800) * f64_em(0x968968);
                if !(v > f64_em(0x96d7f8)) {
                    v = f64_em(0x96d7f8);
                }
                *w(g, 0x35) = (ftol(v) as i16).wrapping_add(di);
            }
        }
        let q = u(m, 0xf5a6);
        if p == u(m, 0xf5aa) && q != 0 {
            *b(q, 0x10) = (*b(q, 0x10)).wrapping_add(1);
            *w(q, 0x35) = (*w(q, 0x35)).wrapping_add(0x113);
            if c.l68 == 0 && !matches!(*w(q, 0x198), 0x34 | 0x37) {
                *b(q, 3) = (*b(q, 3)).wrapping_add(1);
                *w(q, 0x35) = (*w(q, 0x35)).wrapping_add((*w(p, 0x29) as i32 / 3 + 0xe1) as i16);
                *dw(m, 0x4782) = (*dw(m, 0x4782)).wrapping_add(0x113);
            }
        }
    }
    // 0x70717c: as narracoes do gol
    if c.tipo != 0x33 && c.tipo != 0x3a && c.l14 == 0 && sortear(0x32) == 0 && c.l20 == 0 && tg() == 1 {
        na(chamar, if c.tipo == 0x1a { 0x21d2 } else { 0x20d6 }, ub(g, 0x2e), ub(g, 0x27), 0);
    }
    na(chamar, 0x1fff, ub(p, 0x2e), lp, 0);
    if c.tipo == 0x33 {
        na(chamar, if modo == 1 { 0x1fcf } else { 0x20f8 }, ub(p, 0x2e), lp, 0);
    } else {
        *dw(m, 0x4782) = (*dw(m, 0x4782)).wrapping_sub(*dw(m, 0x4786));
        narrar_o_gol_com(&mut c, chamar);
    }
    gol_validado_com(&mut c, chamar);
}

/// As narracoes do gol pelo momento (0x70724d a 0x707cc6): a virada no placar (+0x4782, +0x4786), o
/// gol de quem esta atras no agregado, a comemoracao do Shearer, quem marcava, a assistencia e o
/// goleiro.
#[allow(clippy::cognitive_complexity)]
unsafe fn narrar_o_gol_com(c: &mut Chute, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let w = |q: usize, o: usize| (q + o) as *mut i16;
    let ub = |q: usize, o: usize| *((q + o) as *const u8) as u32;
    let u = |q: usize, o: usize| *((q + o) as *const usize);
    let dw = |q: usize, o: usize| (q + o) as *mut i32;
    let (p, m, g, lado) = (c.p, c.m, c.g, c.lado);
    let lp = lado as u8 as u32;
    let na = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, a3: u32, a4: u32, a7: i8| narrar_finalizacao(chamar, m, codigo, a3, a4, 0xff, 0xff, a7);
    let n7 = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, a3: u32, a4: u32, a5: u32, a6: u32| narrar_finalizacao(chamar, m, codigo, a3, a4, a5, a6, 0);
    let tg = || if u(g, 0x19e) != 0 { tabela_a(u(g, 0x19e), g) } else { 0 };
    let ajustar = |q: usize, delta: i32| {
        if ajustar_17(q, delta as i8) != 0 {
            desempenho_na_partida(q, 0);
        }
    };
    let quem_bloqueou = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32| {
        let q = u(m, 0xf5ae);
        na(chamar, codigo, ub(q, 0x2e), ub(q, 0x27), 0);
    };
    let atras = || atras_no_agregado(m, lado) != 0 && {
        let r = sortear(10);
        byte(u(p, 0x69), 0x58) > (r as u8).wrapping_add(0xe) as i8
    };
    let l4786 = *dw(m, 0x4786);
    let l4782 = || *dw(m, 0x4782) as f64;
    if lado_do_lance(m) as i8 > -1 || c.modo == 6 || c.modo == 5 {
        // 0x707c86
        if c.l11 == 0 {
            na(chamar, 0x20f9, ub(p, 0x2e), lp, 0);
        } else {
            quem_bloqueou(chamar, 0x20fa);
        }
        return;
    }
    if c.l18 && c.l11 == 0 {
        na(chamar, 0x201a, ub(p, 0x2e), lp, 0);
        if atras() {
            na(chamar, 0x21be, ub(p, 0x2e), lp, 0);
        }
        return;
    }
    if l4786 > 0x578 {
        let codigo = if l4782() > f64_em(0x96d810) {
            0x2013
        } else if l4782() > f64_em(0x96d7f0) {
            0x2015
        } else {
            0x201b
        };
        na(chamar, codigo, ub(p, 0x2e), lp, 0);
        let r = sortear(8);
        ajustar(p, r);
        if atras() {
            na(chamar, 0x21be, ub(p, 0x2e), lp, 0);
        }
        return;
    }
    if c.l18 && c.l11 != 0 {
        na(chamar, 0x201c, ub(p, 0x2e), lp, 0);
        quem_bloqueou(chamar, if c.modo == 1 { 0x1fcd } else { 0x20fa });
        let r = sortear(6);
        ajustar(p, r);
        return;
    }
    if l4786 > 0x384 {
        na(chamar, if l4782() > f64_em(0x96d7f0) { 0x2016 } else { 0x2018 }, ub(p, 0x2e), lp, 0);
        let r = sortear(6);
        ajustar(p, r);
        return;
    }
    if l4782() > f64_em(0x96d810) {
        na(chamar, 0x2019, ub(p, 0x2e), lp, 0);
        let r = sortear(4);
        ajustar(p, r);
        return;
    }
    if c.modo != 1 {
        // 0x707bcb
        if c.l11 != 0 {
            quem_bloqueou(chamar, 0x20fa);
            return;
        }
        na(chamar, 0x20f9, ub(p, 0x2e), lp, 0);
        if byte(m, 0x8ebb) != 0 && sortear(8) == 0 && e_o_shearer(p) {
            na(chamar, 0x216c, ub(p, 0x2e), lp, 0);
        }
        return;
    }
    if c.l11 != 0 {
        quem_bloqueou(chamar, 0x1fcd);
        return;
    }
    na(chamar, 0x1fcc, ub(p, 0x2e), lp, 0);
    if byte(m, 0x8ebb) != 0 && sortear(3) != 0 && e_o_shearer(p) {
        crate::comparacao::ramo("chute gol do Shearer");
        na(chamar, 0x216c, ub(p, 0x2e), lp, 0);
    }
    // quem marcava o artilheiro
    let marcadores = quantos_marcam(m, p);
    c.l16 = marcadores as i8;
    if c.tipo != 0x35 && marcadores != 0 {
        let mk = quem_marca(m, p);
        if mk == 0 {
            return;
        }
        let minuto = (*w(m, 0x8ed0) as i32 / 11) as i8;
        if c.conta == 0 {
            if c.e != 0 {
                return;
            }
            if marcadores == 1 {
                if *w(mk, 0x198) != 0x6e && *w(mk, 0x35) < 0x1a5e && *w(p, 0x35) > 0x1a5e && minuto > 0xf {
                    n7(chamar, 0x21c2, ub(p, 0x2e), lp, ub(mk, 0x2e), ub(mk, 0x27));
                }
            } else if byte(p, 0x1b) as i32 > 0x1c52 {
                n7(chamar, 0x21c3, ub(p, 0x2e), lp, ub(mk, 0x2e), ub(mk, 0x27));
            }
            return;
        }
        let (pe, me) = (u(p, 0x6d), u(mk, 0x6d));
        if c.e == 0 || pe == 0 || me == 0 || *w(p, 0x35) <= 0x1a5e || *w(mk, 0x35) >= 0x1c52 || mk != c.e {
            return;
        }
        if (c.tipo == 3 || c.tipo == 0x39) && byte(pe, 0x2e) > 10 && byte(pe, 0x2e) as i32 - byte(me, 0x2e) as i32 > 3 {
            n7(chamar, 0x21c4, ub(p, 0x2e), lp, ub(mk, 0x2e), ub(mk, 0x27));
        } else if matches!(c.l8c, 0x70 | 0x6a | 0x69 | 0x12) && byte(pe, 0x36) > 10 && byte(pe, 0x36) as i32 - byte(me, 0x36) as i32 > 3 {
            n7(chamar, 0x21c5, ub(p, 0x2e), lp, ub(mk, 0x2e), ub(mk, 0x27));
        } else if minuto > 0x3c && estado_do_adversario(p) == 0 && estado_do_lado(p) == 1 {
            let k = m + (lado as isize * 0x36) as usize;
            if ub(k, 0xf5fe) > 4 && ub(k, 0xf5d2) > 0x41 && ub(k, 0xf5d1) > 0x37 {
                n7(chamar, 0x21d7, ub(p, 0x2e), lp, ub(p, 0x2e), lp);
            }
        }
        return;
    }
    // 0x707840: o proprio artilheiro, a assistencia ou o goleiro
    let tp = if u(p, 0x19e) != 0 { tabela_a(u(p, 0x19e), p) } else { 0 };
    if tp & 0x30 != 0 && papel_880(p) == 0 && *w(p, 0x35) > 0x1a5e && ub(p, 0xb) > 1 {
        na(chamar, 0x21c7, ub(p, 0x2e), lp, 0);
        return;
    }
    let q = u(m, 0xf5a6);
    if p == u(m, 0xf5aa) && q != 0 {
        let tq = || if u(q, 0x19e) != 0 { tabela_a(u(q, 0x19e), q) } else { 0 };
        let nq = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32| na(chamar, codigo, ub(q, 0x2e), ub(q, 0x27), 0);
        if papel_880(q) != 0 && tq() & 0x38 != 0 && matches!(*w(m, 0x8d4a), 0x5e | 6 | 7 | 8) {
            if *w(q, 0x35) <= 0x1a5e {
                return;
            }
            if ub(q, 3) > 1 && ub(q, 0x10) > 1 && ub(q, 1) > 0xa {
                nq(chamar, 0x21c8);
            } else if ub(q, 1) > 0xa && ub(q, 3) > 1 && byte(q, 0x10b) != 0 {
                nq(chamar, 0x21ca);
            }
            return;
        }
        if papel_880(q) == 0 && tq() & 0x30 != 0 && ub(q, 2) > 0xa && *w(q, 0x35) > 0x1a5e && ub(q, 3) > 1 && ub(q, 1) > 0xa && sortear(2) != 0 && byte(q, 0x10b) != 0 {
            nq(chamar, 0x21c9);
        }
        return;
    }
    if c.l14 != 0 {
        return;
    }
    let ng = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32| na(chamar, codigo, ub(g, 0x2e), ub(g, 0x27), 0);
    if c.l20 == 0 && tg() == 1 {
        let r = (sortear(10) as u8).wrapping_add(5) as i8;
        if byte(u(g, 0x6d), 0x1c) as i32 + ub(g, 0xf) as i32 > r as i32 {
            let f = ub(g, 0xf) as i32;
            let r2 = (sortear(10) as u8).wrapping_add(5) as i8;
            if byte(u(g, 0x69), 0x57) as i32 + f > r2 as i32 && sortear(f) != 0 {
                let r3 = (sortear(10) as u8).wrapping_add(1) as i8;
                if byte(p, 0x100) as i32 + ub(g, 0xf) as i32 > r3 as i32 {
                    ng(chamar, 0x21d3);
                    return;
                }
            }
        }
    }
    if c.l20 != 0 || tg() != 1 {
        return;
    }
    if byte(u(g, 0x6d), 0x1c) >= 0xa || byte(u(g, 0x69), 0x57) as i32 >= ub(g, 0xf) as i32 + 0xa {
        return;
    }
    let f = ub(g, 0xf) as i32;
    let r = sortear(5);
    if byte(u(g, 0x69), 0x5a) as i32 >= r + f * 2 + 4 || f <= 1 || sortear(f) == 0 {
        return;
    }
    let r2 = (sortear(5) as u8).wrapping_add(1) as i8;
    if byte(p, 0x100) >= r2 {
        return;
    }
    ng(chamar, 0x21d4);
}

/// Repetir o lance (modos 5 e 6): o motor volta aos +0x4786/+0x4782 de antes, narra 0x20f7, o
/// chutador volta a casa e ao +0x39 de antes, domina e chuta de novo no modo 1 (0x704a57 e iguais).
unsafe fn repetir_chute_com(c: &Chute, aplicar: bool, desfazer: bool, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let (p, m) = (c.p, c.m);
    *((m + 0x4786) as *mut i32) = c.l60;
    *((m + 0x4782) as *mut i32) = c.l5c;
    if aplicar {
        aplicar_ao_time(m, p, f32::from_bits(0x3fb9999a));
    }
    narrar_finalizacao(chamar, m, 0x20f7, byte(p, 0x2e) as u8 as u32, c.lado as u8 as u32, 0xff, 0xff, 0);
    *((p + 0x39) as *mut i16) = c.l58;
    if desfazer {
        desfazer_gol(p);
    }
    chamar(0x6f46b0, p, [c.l50 as u8 as u32, c.l54 as u8 as u32, 0, 0, 0]);
    chamar(0x6f0630, p, [0; 5]);
    chamar(0x702050, p, [c.a1, c.v0, c.som, 1, 0]);
}

/// Desfaz as contas do gol (+0x3d..+0x45 e +0xc) quando ele e anulado ou repetido.
unsafe fn desfazer_gol(p: usize) {
    for o in [0x3d, 0x3f, 0x41, 0x43, 0x45] {
        let w = (p + o) as *mut i16;
        *w = (*w).wrapping_sub(1);
    }
    let b = (p + 0xc) as *mut u8;
    *b = (*b).wrapping_sub(1);
}

/// Os companheiros na lista da pressao (a5 = quantos, a6 a lista): com sorteio(n x n + k), 1 narra
/// 0x2154 com o primeiro, 2 (1 em 2) 0x2155 com os dois, senao 0x2132; depois os destaques deles.
/// Com `assistencia`, a de quem marcava (0x21c6) entra como na 0x704d4b.
unsafe fn narrar_companheiros_com(c: &Chute, k: i32, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) -> bool {
    let (p, m, lado) = (c.p, c.m, c.lado);
    let ub = |q: usize, o: usize| *((q + o) as *const u8) as u32;
    let n = c.conta as i32;
    if sortear(n * n + k) == 0 {
        return false;
    }
    let zero = (lado == 0) as u32;
    let reiniciar = || core::ptr::write_unaligned((c.lista + 0xa) as *mut u32, core::ptr::read_unaligned((c.lista + 2) as *const u32));
    if n == 1 {
        reiniciar();
        let r1 = proximo_da_lista(c.lista) as usize;
        narrar_finalizacao(chamar, m, 0x2154, ub(p, 0x2e), lado as u8 as u32, ub(r1, 0x2e), zero, 0);
        destaques(chamar, m, r1, zero, &DESTAQUES_R);
    } else if n == 2 && sortear(2) != 0 {
        reiniciar();
        let r1 = proximo_da_lista(c.lista) as usize;
        let r2 = proximo_da_lista(c.lista) as usize;
        narrar_finalizacao(chamar, m, 0x2155, ub(r1, 0x2e), zero, ub(r2, 0x2e), zero, 0);
        if !destaques(chamar, m, r1, zero, &DESTAQUES_R) {
            destaques(chamar, m, r2, zero, &DESTAQUES_R);
        }
    } else {
        narrar_finalizacao(chamar, m, 0x2132, ub(p, 0x2e), lado as u8 as u32, 0xff, 0xff, 0);
    }
    true
}

/// A assistencia contra quem marcava (0x21c6): o marcador (0x6d5a10) e o companheiro da lista, no
/// jogo (modo 1, nao 0x33/0x35), sem gols nem chutes demais do artilheiro, nos minutos e folegos.
unsafe fn assistencia_ao_marcador(c: &Chute, r: usize) -> bool {
    let (p, m) = (c.p, c.m);
    let w = |q: usize, o: usize| *((q + o) as *const i16);
    let mk = quem_marca(m, p);
    mk != 0
        && c.tipo != 0x33
        && c.tipo != 0x35
        && c.modo == 1
        && byte(p, 0xc) == 0
        && byte(p, 0x10) == 0
        && byte(p, 3) == 0
        && (byte(p, 0xa) as u8) < 5
        && (byte(p, 0xb) as u8) < 2
        && c.l16 > (sortear(0x14) as u8).wrapping_add(0x41) as i8
        && w(p, 0x35) < 0x1c52
        && w(mk, 0x35) > 0x1a5e
        && (byte(p, 0x100) < 1 || sortear(byte(p, 0x100) as i32) == 0)
        && r == mk
}

/// A defesa do goleiro (0x708e84 ao fim): pela razao, ele segura (ate 51), espalma para escanteio
/// (ate 61) ou rebate; as narracoes, os ganhos dele e a bola.
#[allow(clippy::cognitive_complexity)]
unsafe fn defesa_no_chute_com(mut c: Chute, razao: f32, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let b = |q: usize, o: usize| (q + o) as *mut u8;
    let w = |q: usize, o: usize| (q + o) as *mut i16;
    let ub = |q: usize, o: usize| *((q + o) as *const u8) as u32;
    let (p, m, g, lado) = (c.p, c.m, c.g, c.lado);
    let lp = lado as u8 as u32;
    let na = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, a3: u32, a4: u32, a7: i8| narrar_finalizacao(chamar, m, codigo, a3, a4, 0xff, 0xff, a7);
    let ng = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32| narrar_finalizacao(chamar, m, codigo, ub(g, 0x2e), ub(g, 0x27), 0xff, 0xff, 0);
    let modo = c.modo;
    let classe = if razao as f64 > f32_em(0x96d7e8) as f64 {
        1
    } else if razao as f64 > f32_em(0x96d7e4) as f64 {
        2
    } else {
        3
    };
    crate::comparacao::ramo(match classe {
        1 => "chute defesa rebatida",
        2 => "chute defesa escanteio",
        _ => "chute defesa segura",
    });
    if classe == 3 {
        *b(g, 0x2b) = (*b(g, 0x2b)).wrapping_add(sortear(3) as u8);
        *w(g, 0x35) = (*w(g, 0x35)).wrapping_add(if c.tipo != 0x33 { 200 } else { 1300 });
    }
    // o comeco das narracoes: cobranca, penaltis ou tiro livre
    let mut a_2122 = false;
    if modo == 4 || modo == 3 {
        na(chamar, 0x2008, ub(p, 0x2e), lp, 0);
        if modo == 3 {
            a_2122 = true;
        } else {
            na(chamar, 0x2174, ub(p, 0x2e), lp, 0);
        }
    } else if c.tipo == 0x33 {
        if modo == 1 {
            na(chamar, 0x1fce, ub(p, 0x2e), lp, 0);
        }
        a_2122 = true;
    }
    if a_2122 {
        c.l15 = 1;
        ng(chamar, 0x2122);
    } else {
        let tipo_1a = c.tipo == 0x1a && (classe != 3 || c.l20 == 0);
        if c.l1a {
            ng(chamar, if classe == 3 { 0x2112 } else { 0x2111 });
            destaques(chamar, m, g, ub(g, 0x27), &DESTAQUES_R);
        } else if c.l19 {
            ng(chamar, if classe == 3 && tipo_1a { 0x21d0 } else if classe == 3 { 0x2110 } else { 0x210f });
        } else {
            match classe {
                1 => ng(chamar, 0x1fe6),
                2 => ng(chamar, if c.tipo == 0x1a { 0x21d0 } else { 0x1fe4 }),
                _ => {
                    ng(chamar, if tipo_1a { 0x21d0 } else { 0x1fe5 });
                    destaques(chamar, m, p, lp, &DESTAQUES_P);
                }
            }
        }
    }
    let alto = (*((m + 0x4782) as *const i32) as f64) > f64_em(0x96d810);
    if classe == 1 {
        // rebatida (0x70909d)
        if modo == 5 || modo == 6 {
            repetir_chute_com(&c, false, false, chamar);
            return;
        }
        if modo != 2 {
            *w(g, 0x35) = (*w(g, 0x35)).wrapping_add(if c.tipo != 0x33 { 140 } else { 1000 });
        }
        if modo != 1 && modo != 2 {
            return;
        }
        *b(g, 0x2b) = (*b(g, 0x2b)).wrapping_add((sortear(3) + 1) as u8);
        chamar(0x6cd770, m, [ub(m, 0x8ea7), ub(m, 0x8ea8), ((lado == 0) as u32) << 8, ((c.tipo == 0x33) as u32) << 8, 0]);
        *b(m, 0x1cd) = 1;
        if c.l15 != 0 || byte(m, 0x8eb2) == 0 || modo == 4 {
            return;
        }
        if alto || (c.l18 && c.l11 == 0) {
            na(chamar, 0x2020, ub(p, 0x2e), lp, 0);
        } else if !c.l18 && c.l11 == 0 && c.l1b >= 0x19 && modo == 1 {
            na(chamar, 0x1f9e, ub(p, 0x2e), lp, c.l64 as i8);
        }
        return;
    }
    for codigo in [0x21a1, 0x21a2] {
        if destaque_do_jogador(p, codigo) != 0 {
            na(chamar, codigo, ub(p, 0x2e), lp, 0);
        }
    }
    if classe == 2 {
        // para escanteio (0x70950f)
        if c.l15 == 0 {
            if alto || (c.l18 && c.l11 == 0) {
                na(chamar, 0x2020, ub(p, 0x2e), lp, 0);
            }
            if !matches!(modo, 5 | 6 | 2) {
                *w(g, 0x35) = (*w(g, 0x35)).wrapping_add(if c.tipo != 0x33 { 120 } else { 1000 });
            }
        }
        if modo == 4 || modo == 3 {
            return;
        }
        if modo == 5 || modo == 6 {
            repetir_chute_com(&c, false, false, chamar);
            return;
        }
        escanteio_do_chute(&c, chamar);
        return;
    }
    // segura (0x709afb)
    if c.l15 == 0 {
        if alto || (c.l18 && c.l11 == 0) {
            na(chamar, 0x2020, ub(p, 0x2e), lp, 0);
        } else {
            let limite = 0x12c + if c.l1a { 0x190 } else { 0 } + if c.l19 { 0xc8 } else { 0 };
            if *w(p, 0x39) as i32 > limite || c.tipo == 0x3a {
                na(chamar, 0x1fdd, ub(p, 0x2e), lp, byte(p, 0xc));
            }
            if c.l40 < -0x87 && c.tipo != 0x3a {
                narrar_companheiros_com(&c, 3, chamar);
            }
        }
    }
    if modo == 5 || modo == 6 {
        repetir_chute_com(&c, false, false, chamar);
        return;
    }
    chamar(0x6f0630, g, [0; 5]);
}

/// Escanteio para o adversario (0x70961e): a bola na linha de fundo do lado de quem chutou, a
/// reposicao 3 com a bola do lado dele e a narracao 0x1f75.
unsafe fn escanteio_do_chute(c: &Chute, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let (p, m, lado) = (c.p, c.m, c.lado);
    let b = |o: usize| (m + o) as *mut u8;
    match lado {
        1 => *b(0x8ea8) = 0xff,
        0 => *b(0x8ea8) = 0xc,
        _ => {
            // ponytail: o original para o jogo (match_pl.cpp); o lado e sempre 0 ou 1
            linha("motor: chutar: lado invalido no escanteio");
            std::process::abort();
        }
    }
    *b(0x8eae) = *((p + 0x27) as *const u8);
    *((m + 0xf582) as *mut u32) = 0;
    *b(0x8eb2) = 3;
    narrar_finalizacao(chamar, m, 0x1f75, 0xff, lado as u8 as u32, 0xff, 0xff, 0);
}

/// O gol nos penaltis e na cobranca (0x707db2): +1 na conta do lado (+0xf5bc), 0x2000 e, nos
/// penaltis, 0x212e.
unsafe fn gol_nos_penaltis_com(c: &Chute, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let (p, m, lado) = (c.p, c.m, c.lado);
    let k = (m + 0xf5bc + (lado as isize * 0x36) as usize) as *mut u8;
    *k = (*k).wrapping_add(1);
    narrar_finalizacao(chamar, m, 0x2000, byte(p, 0x2e) as u8 as u32, lado as u8 as u32, 0xff, 0xff, 0);
    if c.modo == 3 {
        narrar_finalizacao(chamar, m, 0x212e, byte(p, 0x2e) as u8 as u32, lado as u8 as u32, 0xff, 0xff, 0);
    }
}

/// O gol anulado (0x20ed): o lance marcado antes (0x6bba50) e aplicado (0x6bac40). Contra o proprio
/// lado, 0x20df, talvez 0x20d9 e fator 3; senao, empatado ou um a frente, 1 em 40 0x20d9, fator
/// 1,45. Quem chutou leva o fator (0x6bb8d0) e cansa (0x716e10); as contas do gol sao desfeitas.
unsafe fn gol_anulado_com(c: &Chute, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let (p, m, lado) = (c.p, c.m, c.lado);
    let nb = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32| narrar_finalizacao(chamar, m, codigo, 0xff, lado as u8 as u32, 0xff, 0xff, 0);
    crate::comparacao::ramo("chute gol anulado");
    nb(chamar, 0x20ed);
    let l = lado_do_lance(m) as i8;
    aplicar_falta_com(m, chamar);
    let fator: u32;
    if l == lado {
        nb(chamar, 0x20df);
        if sortear(c.l13 as i32 - c.l12 as i32 + 3) > 0 {
            nb(chamar, 0x20d9);
        }
        fator = 0x40400000;
    } else {
        if (c.l12 == c.l13 || c.l12 as i32 - 1 == c.l13 as i32) && sortear(0x28) == 0 {
            nb(chamar, 0x20d9);
        }
        fator = 0x3fb9999a;
    }
    aplicar_ao_time(m, p, f32::from_bits(fator));
    chamar(0x716e10, p, [0; 5]);
    if c.l17 != 0 {
        desfazer_gol(p);
    }
}

/// Depois das narracoes do gol (0x707cc7): o gol pode ser anulado (1 em 3 a cada passo, com lance
/// marcado), as narracoes de marca e recorde, o placar, a virada e o fim (+0x8eb2 = 8).
#[allow(clippy::cognitive_complexity)]
unsafe fn gol_validado_com(c: &mut Chute, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let b = |q: usize, o: usize| (q + o) as *mut u8;
    let w = |q: usize, o: usize| *((q + o) as *const i16);
    let ub = |q: usize, o: usize| *((q + o) as *const u8) as u32;
    let u = |q: usize, o: usize| *((q + o) as *const usize);
    let (p, m, g, lado) = (c.p, c.m, c.g, c.lado);
    let lp = lado as u8 as u32;
    let modo = c.modo;
    let na = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, a3: u32, a4: u32, a7: i8| narrar_finalizacao(chamar, m, codigo, a3, a4, 0xff, 0xff, a7);
    let nb = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, a7: i8| narrar_finalizacao(chamar, m, codigo, 0xff, lp, 0xff, 0xff, a7);
    let tg = || if u(g, 0x19e) != 0 { tabela_a(u(g, 0x19e), g) } else { 0 };
    let anular = |c: &Chute, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32| -> bool {
        if sortear(3) != 0 && lado_do_lance(m) as i8 > -1 {
            gol_anulado_com(c, chamar);
            return true;
        }
        false
    };
    if modo != 3 && modo != 4 {
        match c.l10 {
            0xe => na(chamar, 0x2175, ub(p, 0x2e), lp, 0),
            0xf => na(chamar, 0x2176, ub(p, 0x2e), lp, 0),
            _ => {}
        }
    }
    if anular(c, chamar) {
        return;
    }
    let minuto = *((m + 0x8ed0) as *const i16) as i32 / 11;
    'final_: {
        if modo == 3 || modo == 4 {
            aplicar_ao_lado(m, (lado == 0) as i8, f32::from_bits(0x3fa00000));
            break 'final_;
        }
        if c.l11 == 0 && byte(p, 0xc) == 3 {
            crate::comparacao::ramo("chute tres gols");
            na(chamar, if byte(p, 0x30) != 0 { 0x2011 } else { 0x1fd5 }, ub(p, 0x2e), lp, 0);
        }
        if modo == 5 || modo == 6 {
            repetir_chute_com(c, true, c.l17 != 0, chamar);
            return;
        }
        if anular(c, chamar) {
            return;
        }
        if c.l11 == 0 && c.l1b >= 0x14 {
            let codigo = if c.l1b > 0x28 { 0x1fa1 } else if c.l1b > 0x19 { 0x1fa0 } else { 0x1f9f };
            na(chamar, codigo, ub(p, 0x2e), lp, c.l64 as i8);
        }
        if anular(c, chamar) {
            return;
        }
        if c.l11 == 0 {
            // as marcas do artilheiro
            if byte(p, 0x30) != 0 && byte(p, 0xc) == 1 {
                let codigo = if destaque_do_jogador(p, 0x21b6) != 0 { 0x21b6 } else { 0x2010 };
                na(chamar, codigo, ub(p, 0x2e), lp, w(p, 0x3d) as i8);
            } else {
                let (g3d, g3f, g41, g43, g45) = (w(p, 0x3d), w(p, 0x3f), w(p, 0x41), w(p, 0x43), w(p, 0x45));
                let redondo = |v: i16| v == 0x32 || (v >= 100 && v % 100 == 0);
                let marca = if g3d == 1 {
                    Some((0x200f, 1))
                } else if g3f == 1 {
                    Some((0x200b, 1))
                } else if g43 == 1 {
                    Some((0x2012, 1))
                } else if g3f >= 10 && g3f % 10 == 0 {
                    Some((0x200b, g3f))
                } else if redondo(g3d) {
                    Some((0x200a, g3d))
                } else if g41 != g3d && redondo(g41) {
                    Some((0x200c, g41))
                } else if g43 != g41 && g43 >= 100 && g43 % 100 == 0 {
                    Some((0x200d, g43))
                } else if g45 == 1 || (g45 >= 5 && g45 % 5 == 0) {
                    Some((0x200e, g45))
                } else {
                    None
                };
                if let Some((codigo, v)) = marca {
                    crate::comparacao::ramo("chute marca do artilheiro");
                    na(chamar, codigo, ub(p, 0x2e), lp, v as i8);
                }
            }
            destaques(chamar, m, p, lp, &DESTAQUES_GOL);
        }
        if anular(c, chamar) {
            return;
        }
        if byte(m, 0x1a4) == 0 && tg() == 1 && c.l20 != 0 {
            na(chamar, 0x1fe1, ub(g, 0x2e), ub(g, 0x27), c.l20 as i8);
        }
        if anular(c, chamar) {
            return;
        }
        // o gol vale (0x70847e): a virada e os momentos
        c.l12 = c.l12.wrapping_add(1);
        let tarde = (0x55..0x5a).contains(&(minuto as i16)) || minuto as i16 >= 0x73;
        let campeonato = w(m, 0x8ed4) == 0x3de;
        let dif = c.l12 as i32 - c.l13 as i32;
        let ao_lado = |fator: u32| aplicar_ao_lado(m, (lado == 0) as i8, f32::from_bits(fator));
        if byte(m, 0x1a8) != 0 {
            nb(chamar, 0x1fd2, 0);
            break 'final_;
        }
        if c.l4f == 0 {
            if dif == 2 && ((minuto >= 0x46 && campeonato) || minuto >= 0x6a) {
                nb(chamar, 0x1fd0, 0);
                ao_lado(0x3fc66666);
                break 'final_;
            }
            if (dif == 1 || c.l26 != 0) && ((minuto >= 0x50 && campeonato) || minuto >= 0x6e) {
                nb(chamar, 0x1fd1, 0);
                ao_lado(0x3fb9999a);
                break 'final_;
            }
            if c.l12 == c.l13 && c.l26 == 0 && minuto >= 0x46 && c.l4e == 0 {
                crate::comparacao::ramo("chute empate no fim");
                let a0 = contar_ativos(m, (lado == 0) as i8) as i8;
                let a1 = contar_ativos(m, lado) as i8;
                let bl: i8 = if a1 < a0 { byte(m, 4 + lado as isize as usize) } else { -1 };
                let (codigo, fator) = match (bl == -1, tarde) {
                    (true, true) => (0x1fd8, 0x3fb9999a),
                    (true, false) => (0x1fd6, 0x3faccccd),
                    (false, true) => (0x1fd9, 0x3fd33333),
                    (false, false) => (0x1fd7, 0x3fb33333),
                };
                ao_lado(fator);
                nb(chamar, codigo, bl);
                break 'final_;
            }
            if dif < 0 && dif > -3 && (minuto >= 0x50 || (minuto >= 0x46 && c.l12 == 1) || (minuto >= 0x3c && c.l12 == 1 && 1 - c.l13 as i32 == -2)) {
                nb(chamar, if tarde { 0x1fdb } else { 0x1fda }, 0);
                break 'final_;
            }
        }
        if c.l26 != 0 && minuto < 0x50 {
            nb(chamar, 0x1fdc, 0);
            ao_lado(0x3fb9999a);
        }
    }
    // 0x708844: o lance marcado ainda anula
    if lado_do_lance(m) as i8 > -1 {
        gol_anulado_com(c, chamar);
        return;
    }
    if c.l14 != 0 && c.l20 == 0 && tg() == 1 {
        na(chamar, 0x20d5, ub(g, 0x2e), ub(g, 0x27), 0);
    }
    if modo == 1 {
        crate::comparacao::ramo("chute gol valido");
        aplicar_ao_time(m, p, f32::from_bits(0x3ca3d70a));
        somar_disposicao(m, lado, 0x96);
        somar_disposicao(m, (lado == 0) as i8, -0x87);
        let k = m + 0xf5bd + (lado as isize * 0x36) as usize;
        *b(k, 0) = (*b(k, 0)).wrapping_add(1);
        // a narracao do placar (0x6d4b70 fica na original)
        let consulta: unsafe extern "thiscall" fn(usize, u32, u32, *mut u32) -> u32 = core::mem::transmute(0x6d4b70usize);
        let zero = (lado == 0) as u32;
        let mut l3c = 0u32;
        for (codigo, lado_c, com) in [
            (0x21a3, lp, false),
            (0x21a4, lp, false),
            (0x21a5, lp, false),
            (0x21a6, zero, false),
            (0x21a7, zero, false),
            (0x21a8, zero, false),
            (0x21a9, zero, false),
            (0x21aa, lp, false),
            (0x21ab, lp, true),
            (0x21ac, zero, true),
            (0x21ad, lp, true),
            (0x21ae, zero, true),
            (0x21af, zero, true),
        ] {
            if consulta(m, lado_c, codigo, if com { &mut l3c } else { core::ptr::null_mut() }) != 0 {
                narrar_finalizacao(chamar, m, codigo, 0xff, lado_c, 0xff, 0xff, if com { l3c as i8 } else { 0 });
                break;
            }
        }
        let clube_8 = || {
            let t = u(m, 0x4792);
            let clube = u(t, if lado != 0 { 0x20 } else { 0x1c });
            let k = u(clube, 0x53);
            k != 0 && byte(k, 0x77) == 8
        };
        let r = sortear(5);
        let mut feito = false;
        if c.l12 as i32 - c.l13 as i32 > r + 2 && sortear(2) == 0 && clube_8() {
            nb(chamar, 0x2117, 0);
            feito = true;
        }
        if !feito && (c.l12 == c.l13 || c.l12 as i32 - 1 == c.l13 as i32 || c.l26 != 0) {
            let x = sortear(minuto);
            if x > sortear(0x32) && clube_8() {
                nb(chamar, 0x2116, 0);
            }
        }
    }
    *b(m, 0x8eae) = (lado == 0) as u8;
    *((m + 0xf582) as *mut u32) = 0;
    *b(m, 0x8eb2) = 8;
    *b(m, 0x1ca) = 0;
}

/// O chute que nao foi no alvo (0x7040cb a 0x706cb7): bloqueado (escanteio), na trave (a bola
/// espirra) ou para fora (tiro de meta); as narracoes pela classe e pelo momento.
#[allow(clippy::cognitive_complexity)]
unsafe fn chute_fora_com(c: &Chute, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let b = |q: usize, o: usize| (q + o) as *mut u8;
    let w = |q: usize, o: usize| *((q + o) as *const i16);
    let ub = |q: usize, o: usize| *((q + o) as *const u8) as u32;
    let u = |q: usize, o: usize| *((q + o) as *const usize);
    let (p, m, g, lado) = (c.p, c.m, c.g, c.lado);
    let lp = lado as u8 as u32;
    let modo = c.modo;
    let na = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, a7: i8| narrar_finalizacao(chamar, m, codigo, ub(p, 0x2e), lp, 0xff, 0xff, a7);
    let tg = || if u(g, 0x19e) != 0 { tabela_a(u(g, 0x19e), g) } else { 0 };
    if c.l11 == 2 {
        // o chute bateu em alguem e saiu (0x7040d6): escanteio
        crate::comparacao::ramo("chute bloqueado para escanteio");
        if c.l28 == 5 {
            na(chamar, 0x1f83, 5);
        } else {
            na(chamar, 0x1f82, c.l28);
        }
        if modo == 4 || modo == 3 {
            na(chamar, 0x2008, 0);
        } else if c.tipo == 0x33 && modo == 1 {
            na(chamar, 0x1fce, 0);
        }
        if modo == 5 || modo == 6 {
            match lado {
                1 => *b(m, 0x8ea8) = 0xff,
                0 => *b(m, 0x8ea8) = 0xc,
                _ => {}
            }
            repetir_chute_com(c, false, false, chamar);
            return;
        }
        escanteio_do_chute(c, chamar);
        return;
    }
    if modo == 4 || modo == 3 {
        na(chamar, 0x2008, 0);
        if modo == 4 {
            na(chamar, 0x2174, 0);
        }
    } else if c.tipo == 0x33 {
        na(chamar, 0x1fce, 0);
    }
    let goleiro = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32| {
        if c.tipo != 0x33 && c.tipo != 0x3a && c.l14 == 0 && sortear(0x32) == 0 && c.l20 == 0 && tg() == 1 {
            narrar_finalizacao(chamar, m, if c.tipo == 0x1a { 0x21d2 } else { 0x20d6 }, ub(g, 0x2e), ub(g, 0x27), 0xff, 0xff, 0);
        }
    };
    if c.tipo == 0x33 {
        match c.l10 {
            2 | 5 | 8 | 3 | 4 | 6 | 7 | 9 => {
                na(chamar, if matches!(c.l10, 2 | 5 | 8) { 0x2126 } else { 0x2125 }, c.l28);
                destaques(chamar, m, p, lp, &DESTAQUES_P);
            }
            0xa | 0xc => na(chamar, 0x2123, c.l28),
            0xb | 0xd => na(chamar, 0x2124, c.l28),
            _ => {}
        }
    } else {
        match c.l10 {
            2 => {
                na(chamar, 0x1fbf, 0);
                destaques(chamar, m, p, lp, &DESTAQUES_P);
            }
            3 => {
                if c.l28 == 5 {
                    na(chamar, 0x1fc1, 5);
                } else {
                    na(chamar, 0x1fc0, c.l28);
                }
                destaques(chamar, m, p, lp, &DESTAQUES_P);
            }
            4 => {
                na(chamar, 0x1fc2, 0);
                destaques(chamar, m, p, lp, &DESTAQUES_P);
            }
            5 => na(chamar, 0x1fc3, 0),
            6 => na(chamar, 0x1fc4, 0),
            7 => na(chamar, 0x1fc5, 0),
            8..=0xd => {
                goleiro(chamar);
                na(chamar, 0x1fc6 + (c.l10 as u32 - 8), 0);
            }
            _ => {}
        }
    }
    if modo != 1 && modo != 2 {
        if modo == 5 || modo == 6 {
            repetir_chute_com(c, false, false, chamar);
        }
        return;
    }
    match c.l10 {
        0xc | 0xd => {
            // na trave (0x704b27): a bola espirra
            crate::comparacao::ramo("chute na trave");
            if c.l18 {
                if c.l11 == 0 {
                    na(chamar, 0x2020, 0);
                }
            } else if c.l11 == 0 && c.l1b >= 0x19 && w(p, 0x39) < 0x32 {
                na(chamar, 0x1f9e, c.l64 as i8);
            }
            if c.l14 != 0 && c.l20 == 0 && tg() == 1 {
                narrar_finalizacao(chamar, m, 0x20d5, ub(g, 0x2e), ub(g, 0x27), 0xff, 0xff, 0);
            }
            let l = if c.l11 != 0 { (lado == 0) as u32 } else { lp };
            chamar(0x6cd770, m, [ub(m, 0x8ea7), ub(m, 0x8ea8), 2 | l << 8, 1 | ((c.tipo == 0x33) as u32) << 8, 0]);
            if tg() == 1 {
                *b(g, 0x2b) = (*b(g, 0x2b)).wrapping_add(sortear(3) as u8);
            }
            *b(m, 0x1cd) = 1;
        }
        2..=4 => {
            crate::comparacao::ramo("chute fraco para fora");
            if c.tipo != 0x33 {
                fora_fraco_com(c, chamar);
            }
            for codigo in [0x21a1, 0x21a2] {
                if destaque_do_jogador(p, codigo) != 0 {
                    na(chamar, codigo, 0);
                }
            }
            tiro_de_meta(c, chamar);
        }
        _ => {
            crate::comparacao::ramo("chute para fora");
            if c.tipo != 0x33 {
                fora_com(c, chamar);
            }
            tiro_de_meta(c, chamar);
        }
    }
}

/// A bola saiu pela linha de fundo (0x705d15 e 0x706c5d): a linha do outro lado e a bola fora.
unsafe fn tiro_de_meta(c: &Chute, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let m = c.m;
    match c.lado {
        1 => *((m + 0x8ea8) as *mut u8) = 0xff,
        0 => *((m + 0x8ea8) as *mut u8) = 0xc,
        _ => {}
    }
    chamar(0x6cd490, m, [c.lado as u8 as u32, 0, 0, 0, 0]);
}

/// As narracoes do chute fraco para fora (classes 2 a 4, 0x704c6d a 0x705cb2).
#[allow(clippy::cognitive_complexity)]
unsafe fn fora_fraco_com(c: &Chute, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let ub = |q: usize, o: usize| *((q + o) as *const u8) as u32;
    let u = |q: usize, o: usize| *((q + o) as *const usize);
    let (p, m, lado) = (c.p, c.m, c.lado);
    let lp = lado as u8 as u32;
    let zero = (lado == 0) as u32;
    let na = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, a7: i8| narrar_finalizacao(chamar, m, codigo, ub(p, 0x2e), lp, 0xff, 0xff, a7);
    let n_21c6 = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, r: usize| narrar_finalizacao(chamar, m, 0x21c6, ub(p, 0x2e), lp, ub(r, 0x2e), ub(r, 0x27), 0);
    let reiniciar = || core::ptr::write_unaligned((c.lista + 0xa) as *mut u32, core::ptr::read_unaligned((c.lista + 2) as *const u32));
    if c.l18 && c.l11 == 0 {
        let q = companheiro_livre(m, p);
        if q != 0 && byte(u(p, 0x6d), 0x3f) < 10 && {
            let r = sortear(6);
            byte(u(p, 0x69), 0x5b) < (r as u8).wrapping_add(10) as i8
        } && byte(u(q, 0x69), 0x5b) < 10
            && sortear(byte(u(q, 0x69), 0x5b) as i32) == 0
        {
            crate::comparacao::ramo("chute companheiro livre");
            narrar_finalizacao(chamar, m, 0x21dd, ub(p, 0x2e), lp, ub(q, 0x2e), ub(q, 0x27), 0);
        } else {
            na(chamar, 0x201e, 0);
        }
        return;
    }
    if (*((m + 0x4782) as *const i32) as f64) > f64_em(0x96d810) {
        if c.l40 >= -0x87 {
            if c.tipo == 0x3a && c.conta == 0 {
                na(chamar, 0x219a, 0);
            }
        } else if c.tipo != 0x3a {
            let n = c.conta as i32;
            if sortear(n * n + 4) != 0 {
                if n == 1 {
                    reiniciar();
                    let r1 = proximo_da_lista(c.lista) as usize;
                    if c.l25 != 0 {
                        if assistencia_ao_marcador(c, r1) {
                            n_21c6(chamar, r1);
                        }
                    } else {
                        narrar_finalizacao(chamar, m, 0x2154, ub(p, 0x2e), lp, ub(r1, 0x2e), zero, 0);
                    }
                    destaques(chamar, m, r1, zero, &DESTAQUES_R);
                } else if n == 2 {
                    if sortear(2) == 0 {
                        na(chamar, 0x2132, 0);
                    } else {
                        reiniciar();
                        let r1 = proximo_da_lista(c.lista) as usize;
                        let r2 = proximo_da_lista(c.lista) as usize;
                        if c.l25 != 0 {
                            if r2 != 0 && assistencia_ao_marcador(c, r2) {
                                n_21c6(chamar, r2);
                            }
                        } else {
                            narrar_finalizacao(chamar, m, 0x2155, ub(r1, 0x2e), zero, ub(r2, 0x2e), zero, 0);
                        }
                        if !destaques(chamar, m, r1, zero, &DESTAQUES_R) {
                            destaques(chamar, m, r2, zero, &DESTAQUES_R);
                        }
                    }
                } else {
                    na(chamar, 0x2132, 0);
                }
            }
        }
        na(chamar, 0x201f, 0);
        return;
    }
    let x39 = *((p + 0x39) as *const i16);
    if x39 > 0x15e {
        if byte(p, 0xc) == 2 && (*((m + 0x8ed0) as *const i16) as i32 / 11) > 0x32 {
            na(chamar, 0x1fdf, 2);
        } else {
            na(chamar, 0x1fde, byte(p, 0xc));
            destaques(chamar, m, p, lp, &DESTAQUES_P);
        }
        return;
    }
    if x39 > 0xfa {
        na(chamar, 0x1fde, byte(p, 0xc));
        if !destaques(chamar, m, p, lp, &DESTAQUES_P) && c.l25 != 0 {
            reiniciar();
            let r1 = proximo_da_lista(c.lista) as usize;
            if assistencia_ao_marcador(c, r1) {
                n_21c6(chamar, r1);
            }
        }
        return;
    }
    if c.l40 >= -0x87 || c.tipo == 0x3a {
        return;
    }
    let n = c.conta as i32;
    if sortear(n * n + 4) == 0 {
        return;
    }
    if n == 1 {
        reiniciar();
        let r1 = proximo_da_lista(c.lista) as usize;
        narrar_finalizacao(chamar, m, 0x2154, ub(p, 0x2e), lp, ub(r1, 0x2e), zero, 0);
        if !destaques(chamar, m, r1, zero, &DESTAQUES_R) && c.l25 != 0 && assistencia_ao_marcador(c, r1) {
            n_21c6(chamar, r1);
        }
    } else if n == 2 && sortear(2) != 0 {
        reiniciar();
        let r1 = proximo_da_lista(c.lista) as usize;
        let r2 = proximo_da_lista(c.lista) as usize;
        narrar_finalizacao(chamar, m, 0x2155, ub(r1, 0x2e), zero, ub(r2, 0x2e), zero, 0);
        if !destaques(chamar, m, r1, zero, &DESTAQUES_R) {
            destaques(chamar, m, r2, zero, &DESTAQUES_R);
        }
    } else {
        na(chamar, 0x2132, 0);
    }
}

/// As narracoes do chute para fora (classes 5 a 0xb, 0x705d52 a 0x706c5c).
unsafe fn fora_com(c: &Chute, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let ub = |q: usize, o: usize| *((q + o) as *const u8) as u32;
    let u = |q: usize, o: usize| *((q + o) as *const usize);
    let (p, m, g, lado) = (c.p, c.m, c.g, c.lado);
    let lp = lado as u8 as u32;
    let na = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, a7: i8| narrar_finalizacao(chamar, m, codigo, ub(p, 0x2e), lp, 0xff, 0xff, a7);
    let tg = || if u(g, 0x19e) != 0 { tabela_a(u(g, 0x19e), g) } else { 0 };
    let companheiros = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32| {
        if c.l40 < -0x87 && c.tipo != 0x3a {
            narrar_companheiros_com(c, 4, chamar);
        }
    };
    if c.l18 && c.l11 == 0 {
        na(chamar, 0x201e, 0);
        companheiros(chamar);
        return;
    }
    if (*((m + 0x4782) as *const i32) as f64) > f64_em(0x96d810) {
        companheiros(chamar);
        na(chamar, 0x201f, 0);
        return;
    }
    let x39 = *((p + 0x39) as *const i16);
    let k3a = c.tipo == 0x3a;
    if (x39 > 0x15e && !k3a) || (x39 > 0xaf && k3a) {
        na(chamar, if byte(p, 0xc) != 2 { 0x1fdd } else { 0x1fdf }, byte(p, 0xc));
        destaques(chamar, m, p, lp, &DESTAQUES_P);
        return;
    }
    if x39 > 0xfa || k3a {
        na(chamar, 0x1fdd, byte(p, 0xc));
        destaques(chamar, m, p, lp, &DESTAQUES_P);
        return;
    }
    if c.l14 != 0 && c.l20 == 0 && tg() == 1 && !matches!(c.l10, 5 | 6 | 7) {
        narrar_finalizacao(chamar, m, 0x20d5, ub(g, 0x2e), ub(g, 0x27), 0xff, 0xff, 0);
    }
    companheiros(chamar);
}

/// Modo Rust: as chamadas do chute vao para o jogo (a interceptacao com x | y << 8; a bola desviada
/// com dentro | sem cansar << 8; o chute de novo sem a lista).
unsafe fn despachar_finalizacao(rotina: usize, this: usize, a: [u32; 5]) -> u32 {
    match rotina {
        0x6cc540 => {
            let f: unsafe extern "thiscall" fn(usize, i8, i8, u32, i8, u32, u32) -> u8 = core::mem::transmute(rotina);
            f(this, a[0] as u8 as i8, (a[0] >> 8) as u8 as i8, a[1], a[2] as u8 as i8, a[3], a[4]) as u32
        }
        0x6cd770 => {
            let f: unsafe extern "thiscall" fn(usize, i8, i8, i8, i8, u8, u8, usize) = core::mem::transmute(rotina);
            f(this, a[0] as i8, a[1] as i8, a[2] as u8 as i8, (a[2] >> 8) as u8 as i8, a[3] as u8, ((a[3] >> 8) as u8 == 0) as u8, a[4] as usize);
            0
        }
        0x702050 => {
            let f: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32) = core::mem::transmute(rotina);
            f(this, a[0], a[1], a[2], a[3], 0, 0);
            0
        }
        0x6f46b0 => {
            let f: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(rotina);
            f(this, a[0], a[1]);
            0
        }
        _ => despachar_movimento(rotina, this, a),
    }
}

/// { 0x702050 } (ret 0x18, match_pl.cpp): ver `chutar_com`.
#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn chutar(p: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32) {
    chutar_com(p, a1, a2, a3, a4, a5, a6 as usize, &mut |r, t, a| despachar_finalizacao(r, t, a))
}

#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn comparar_chutar(p: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32) {
    if gravando() {
        // unidade da fita de fora (conducao, penaltis, o proprio chute de novo); por dentro, a propria comparacao
        chamar_gravando(0x702050, p, [a1, a2, a3, a4, a6], || {
            fita_do_chute(p, a1, a2, a3, a4, a5, a6);
            0
        });
        return;
    }
    fita_do_chute(p, a1, a2, a3, a4, a5, a6)
}

#[allow(clippy::too_many_arguments)]
unsafe fn fita_do_chute(p: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32) {
    use std::sync::atomic::Ordering::Relaxed;
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32)>(324);
    let m = *((p + 0x19e) as *const usize);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(m, MOTOR), (0xb67a34, 4), (0xae1080, 4)]);
    if a6 != 0 {
        // a lista dos companheiros: o cursor (+0xa) anda
        reg.push((a6 as usize, 14));
    }
    let t = *((m + 0x4792) as *const usize);
    for clube in [*((t + 0x1c) as *const usize), *((t + 0x20) as *const usize)] {
        reg.extend(linhas_do_clube(clube));
    }
    let gravar = GRAVAR_716.swap(true, Relaxed);
    let a7 = GRAVAR_A7.swap(true, Relaxed);
    let (_, _, igual, sequencia, k) = comparar_com_fita(
        reg,
        || {
            o(p, a1, a2, a3, a4, a5, a6);
            0
        },
        |chamar| {
            chutar_com(p, a1, a2, a3, a4, a5, a6 as usize, &mut |rotina, this, args| match rotina {
                // a interceptacao roda por dentro (o comparador dela nao grava unidade): as jogadas sao as unidades
                0x6cc540 => {
                    let mut jogada = |q: usize, a: [u32; 4]| chamar(0x70dfd0, q, [a[0], a[1], a[2], a[3], 0], [u32::MAX, u32::MAX, 0xff, u32::MAX, 0]) as u8;
                    interceptar_com(this, args[0] as u8 as i8, (args[0] >> 8) as u8 as i8, args[1], args[2] as u8 as i8, args[3], args[4], &mut |lado, grupo, v, a3, a5, a6| {
                        procurar_jogada_com(this, lado, grupo, v, a3, 1, a5, a6, &mut jogada)
                    }) as u32
                }
                _ => {
                    let mascara = match rotina {
                        0x6d6f70 => [0xff_ffff, u32::MAX, 0xff, 0xff, 0xff],
                        0x6cd770 => [0xff, 0xff, 0xffff, 0xff, u32::MAX],
                        0x702050 => [0xff, u32::MAX, 0xffff, 0xff, 0],
                        0x6f46b0 => [0xff, 0xff, 0, 0, 0],
                        0x6cd490 => [0xff, 0, 0, 0, 0],
                        0x62ed70 => [u32::MAX, 0, 0, 0, 0],
                        0x6d0980 => [0xff, 0xff, u32::MAX, u32::MAX, 0],
                        0x897760 => [u32::MAX, 0xff, u32::MAX, 0, 0],
                        0x68d0d0 => [0, u32::MAX, 0, 0, 0],
                        0x6e1aa0 | 0x716c90 | 0x6f0630 | 0x714d40 | 0x7103e0 => [0, 0, 0, 0, 0],
                        _ => [0xff, 0, 0, 0, 0],
                    };
                    chamar(rotina, this, args, mascara)
                }
            });
            0
        },
    );
    GRAVAR_A7.store(a7, Relaxed);
    GRAVAR_716.store(gravar, Relaxed);
    PLACAR.registrar(igual && sequencia, || format!("chutar(tipo {:#x}, modo {}) {} chamadas: {}", a1 as u8, a4 as u8, k, detalhe_fita()));
}
