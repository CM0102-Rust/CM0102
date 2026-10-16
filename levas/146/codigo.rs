// ---------- centesima quadragesima sexta leva: a execucao do passe (0x6fad40) ----------

/// A nota base do passe pelos marcadores da posicao (casos 6..=8 e 0x5e de 0x6fad40): na linha de
/// fundo, com as marcas 0x880 nas duas tabelas (e 0x38 + a marca 4 da posicao), +0x4b/+0x3e/+0x2d
/// conforme o clube humano (0x600a50); senao +0x19 no fundo ou o desconto de um terco do sorteio.
unsafe fn nota_do_passe_pela_posicao(p: usize, base: i16, l20: i32, l14: i32) -> i16 {
    let m = *((p + 0x19e) as *const usize);
    let ta = || if m != 0 { tabela_a(m, p) } else { 0 };
    let tb = || if m != 0 { tabela_b(m, p) } else { 0 };
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let clube = *((p + 0xf9) as *const usize);
    let dirigido = || clube != 0 && humano(clube, 0, 1, 0, 0) != 0;
    let y = byte(p, 0x103);
    let marcas = || marcas_da_posicao(m + 0x91e2 + byte(p, 0x27) as isize as usize * 0x18e3, byte(p, 0x19));
    let fundo = y == 0 || y == 0xb;
    let w = base.wrapping_sub(l20 as i16);
    if fundo && ta() & 0x880 != 0 && tb() & 0x880 != 0 && ta() & 0x38 != 0 && marcas() & 0x4_0000_0000 != 0 {
        return w.wrapping_add(if dirigido() { 0x4b } else { 0x3e });
    }
    if fundo && ta() & 0x880 != 0 && tb() & 0x880 != 0 {
        return w.wrapping_add(if dirigido() { 0x3e } else { 0x2d });
    }
    let lado = byte(p, 0x27);
    if fundo || (((lado == 1 && y < 3) || (lado == 0 && y > 8)) && ta() & 0x880 != 0) {
        return w.wrapping_add(0x19);
    }
    base.wrapping_sub((l14 as i16 as i32 / 3) as i16).wrapping_sub(l20 as i16)
}

/// O desafiante tirou a bola na execucao do passe (0x6fad40, de 0x6fc0e5 a 0x6fc988): a disputa
/// limpa conta para ele (+0x198 = lance, +4, -0x3b/-0x46 em +0x35), a narracao (na area dele, 1 em
/// 2: a da falta e 0x20e8/0x20ea/0x20eb; senao pelo lance, com a defesa do goleiro 0x70d430 no lance
/// 1), ele domina (0x6f0630, +0x19c = nota - 400 em -200..=200), a bola espirra (0x6cd770) abaixo do
/// limite do lance ou a media de acertos e narrada; os ganhos dele e o +0x2b do passador.
#[allow(clippy::too_many_arguments)]
unsafe fn passe_desarmado_com(m: usize, p: usize, e: usize, lance: i8, nota: i16, a: u8, vantagem: u8, penalti: bool, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let b = |q: usize, o: usize| (q + o) as *mut u8;
    let w = |q: usize, o: usize| (q + o) as *mut i16;
    let ub = |q: usize, o: usize| *((q + o) as *const u8) as u32;
    let u = |q: usize, o: usize| *((q + o) as *const usize);
    let narrar = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, a3: u32, a4: u32, a5: u32| {
        chamar(0x6d6f70, u(m, 0x8f7a), [codigo, m as u32, a3, a4, a5]);
    };
    let par = |q: usize| ub(q, 0x2e) | ub(q, 0x27) << 8;
    let zero = |q: usize| (byte(q, 0x27) == 0) as u32;
    // a disputa limpa
    *w(e, 0x198) = lance as i16;
    *b(e, 4) = (*b(e, 4)).wrapping_add(1);
    let t = tabela_a(m, e);
    *w(e, 0x35) = (*w(e, 0x35)).wrapping_add(if t & 0x30 != 0 && tabela_a(m, e) & 0x880 != 0 { -0x3b } else { -0x46 });
    let narrou = (codigo_especial(lance) != 0 || vantagem != 0 || sortear(*w(e, 0x31) as i32) < -10)
        && na_area_atacada((byte(e, 0x27) != 1) as i8, byte(e, 0x102), byte(e, 0x103))
        && sortear(2) == 0;
    if narrou {
        crate::comparacao::ramo("passe desarme narrado na area");
        narrar(chamar, narracao_da_falta(lance), ub(e, 0x2e), ub(e, 0x27), par(p));
        let codigo = match sortear(0x32) {
            1 => 0x20ea,
            0 | 2..=5 => 0x20eb,
            _ => 0x20e8,
        };
        narrar(chamar, codigo, u32::MAX, zero(e), u32::MAX);
    } else {
        let codigo = match lance {
            1 => {
                crate::comparacao::ramo("passe desarme do goleiro");
                if defender_com(e, (nota as i32 - 200) as i16, &mut |rotina, this, x| {
                    if rotina == 0x6d6f70 {
                        chamar(rotina, this, [x[0], x[1], x[2], x[3], (x[4] & 0xff) | 0xff00])
                    } else {
                        chamar(rotina, this, x)
                    }
                }) == 0
                {
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
            narrar(chamar, codigo, ub(e, 0x2e), ub(e, 0x27), par(p));
        }
    }
    chamar(0x6f0630, e, [0; 5]);
    *w(e, 0x19c) = ((nota as i32 - 400) as i16).clamp(-200, 200);
    let mut dominou = false;
    let limite = match lance {
        0x13 => 400,
        0x21 | 0x2d => 0xfa,
        _ => 100,
    };
    if *w(e, 0x19c) < limite && byte(m, 0x8ea9) != 4 && byte(m, 0x8ea9) != -1 {
        crate::comparacao::ramo("passe bola espirra");
        chamar(0x6cd770, m, [ub(p, 0x102), ub(p, 0x103), 1 | zero(p) << 8, 0, 0]);
        if byte(m, 0x8eb2) == 0 {
            let mut k = 3;
            k -= (byte(e, 0x10d) as i32 > sortear(10)) as i32;
            k -= (byte(e, 0x10c) as i32 > sortear(10)) as i32;
            *b(e, 0x2b) = sortear(k) as u8;
            dominou = true;
        } else if penalti {
            chamar(0x716c90, p, [0; 5]);
        }
    } else if byte(m, 0x8ea9) < 4 {
        if byte(m, 0x8ea9) > -1 {
            *b(m, 0x8ea9) = 0;
        }
        let (x, y) = (byte(p, 0x102), byte(p, 0x103));
        if byte(e, 0x27) == 0 && byte(m, 0x8ea9) != -1 {
            apontar(e, x, y, 4, if y < 9 { y + 3 } else { 11 });
        } else {
            apontar(e, x, y, 4, if y > 3 { y - 3 } else { 0 });
        }
        let minuto = (*w(m, 0x8ed0) as i32 / 11) as i8;
        let (contas, sorteio) = if lance == 0x15 { ((7, 8), 0x37) } else { ((4, 5), 0x41) };
        let media = if byte(e, contas.0) != 0 { (ub(e, contas.1) as f64 / ub(e, contas.0) as f64) as f32 } else { 0.0 };
        if minuto > (sortear(0x14) as u8).wrapping_add(sorteio) as i8 {
            let me = u(e, 0x19e);
            let gols = byte(me, if byte(e, 0x27) != 0 { 0xf5bd } else { 0xf5f3 });
            let tentativas = if lance == 0x15 { ub(e, 8) > 3 && byte(e, 9) != 0 } else { ub(e, 6) > 1 };
            if gols == 0 && media as f64 > f64_em(0x96a6a0) && tentativas && *w(e, 0x35) > 0x1a5e && byte(e, 0xfd) == 0 {
                crate::comparacao::ramo("passe media de acertos");
                narrar(chamar, if lance == 0x15 { 0x21ce } else { 0x21cd }, ub(e, 0x2e), ub(e, 0x27), par(p));
                *b(e, 0xfd) = 1;
            }
        }
    }
    *b(e, 5) = (*b(e, 5)).wrapping_add(1);
    let me = u(e, 0x19e);
    let t = || if me != 0 { tabela_a(me, e) } else { 0 };
    let ganho: i16 = if dominou { 0x78 } else { 0xc8 };
    let longe = t() & 4 == 0 && t() & 2 == 0;
    *w(e, 0x35) = (*w(e, 0x35)).wrapping_add(if longe { ganho << 1 } else { ganho });
    *b(p, 0x2b) = (*b(p, 0x2b)).wrapping_add(a);
    let alvo = if byte(e, 0x27) != 0 { 0xb } else { 0 };
    let perto = !(tabela_12x9(byte(e, 0x103) as i32 - alvo, byte(e, 0x102) as i32 - 4) as f64 > f64_em(0x967890));
    if (perto || atras_do_extremo(me, e) != 0) && *w(p, 0x29) > 0 {
        *b(e, 6) = (*b(e, 6)).wrapping_add(1);
    }
    if dominou && sortear(7) == 0 {
        narrar(chamar, 0x1f41, ub(e, 0x2e), ub(e, 0x27), u32::MAX);
    }
}

/// A falta do desafiante na execucao do passe (0x6fad40, de 0x6fb678 a 0x6fc08f): como
/// `punir_no_movimento_com`, mas a bola parada fica com o passador antes das narracoes, o penalti
/// sorteia pela pessoa (+0x5a, sem o +6) e 10, e o evento 0x82 nao depende do cartao.
#[allow(clippy::too_many_arguments)]
unsafe fn falta_no_passe_com(m: usize, e: usize, p: usize, mut tipo: u8, cartao: u8, grav: i8, arriscado: i8, vantagem: u8, lance: i8, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let b = |q: usize, o: usize| (q + o) as *mut u8;
    let w = |q: usize, o: usize| (q + o) as *mut i16;
    let ub = |q: usize, o: usize| *((q + o) as *const u8) as u32;
    let u = |q: usize, o: usize| *((q + o) as *const usize);
    let narrar = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, a3: u32, a4: u32, a5: u32| {
        chamar(0x6d6f70, u(m, 0x8f7a), [codigo, m as u32, a3, a4, a5]);
    };
    let par = |q: usize| ub(q, 0x2e) | ub(q, 0x27) << 8;
    let zero = |q: usize| (byte(q, 0x27) == 0) as u32;
    let lado = byte(p, 0x27);
    let mut peso = 0f32;
    motor_punir_lance(m, e, p, &mut tipo, cartao as i8, grav, arriscado as u8, &mut peso, vantagem);
    narrar(chamar, narracao_da_falta(lance), ub(e, 0x2e), ub(e, 0x27), par(p));
    let cor: i32 = match tipo {
        2 => 0x1ff9,
        3 => 0x1ff7,
        4 => 0x1ff8,
        5 => 0x1ff6,
        _ => {
            if grav as i32 > sortear(7) + 2 { 0x210a } else { -1 }
        }
    };
    // o evento 0x82 (cartao 4/5 pelos motivos 5, 10, 11 e o historico do jogador)
    let media: unsafe extern "thiscall" fn(usize, usize, u32, u32, f32) -> f64 = core::mem::transmute(0x7c1e30usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let v = ftol(media(u(0xb74350, 0), u(e, 0x6d), 0xb, 2, 6.5)) as i8;
    let base = *(0xae2364 as *const i32);
    let lista = if m != 0 { u(m, 0x478e) } else { 0 };
    if lista != 0
        && (tipo == 5 || tipo == 4)
        && (cartao == 0xa || cartao == 5 || cartao == 0xb)
        && u(e, 0x69) != 0
        && *(u(e, 0x69) as *const i32) < base
        && (u(p, 0x69) == 0 || *(u(p, 0x69) as *const i32) < base)
        && u(e, 0xf9) != 0
        && humano(u(e, 0xf9), 0, 1, 0, 0) == 0
        && (byte(u(e, 0x69), 0x5b) as i32) < v as i32 + 10
    {
        let pessoa = u(e, 0x6d);
        let k = 0xfu8.wrapping_sub(v as u8);
        if byte(pessoa, 0x1c) > (sortear(5) as u8).wrapping_add(k) as i8
            && byte(pessoa, 0x25) > (sortear(5) as u8).wrapping_add(k) as i8
            && *((pessoa + 0xb) as *const i16) > (sortear(0xdac) + 3000) as i16
            && v as i32 > sortear(3) + 1
        {
            crate::comparacao::ramo("passe evento 0x82");
            let mut ev = [0u8; 0x19];
            ev[0..4].copy_from_slice(&*(u(e, 0x69) as *const [u8; 4]));
            ev[4..8].copy_from_slice(&*(u(e, 0xf9) as *const [u8; 4]));
            ev[0xd] = lado as u8;
            ev[0xf..0x13].copy_from_slice(&u32::MAX.to_le_bytes());
            ev[0x13..0x15].copy_from_slice(&0x82u16.to_le_bytes());
            ev[0x15..0x19].copy_from_slice(&*(u(p, 0x69) as *const [u8; 4]));
            pedir_vetores(&[(ev.as_ptr() as usize, 0x19)], &[]);
            chamar(0x68d0d0, lista, [0, 0x19, 0, 0, 0]);
        }
    }
    let som: u32;
    if tipo == 7 {
        som = 0x1f79;
        *b(m, 0x8eb2) = 7;
    } else if na_area_atacada(lado, byte(p, 0x102), byte(p, 0x103)) {
        crate::comparacao::ramo("passe penalti");
        if cor as i16 == -1 {
            match sortear(4) {
                0 => narrar(chamar, 0x20e9, u32::MAX, ub(p, 0x27), u32::MAX),
                1 => narrar(chamar, 0x20ec, u32::MAX, ub(p, 0x27), u32::MAX),
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
        narrar(chamar, 0x1f7a, u32::MAX, ub(p, 0x27), u32::MAX);
    }
    let expulso = matches!(tipo, 3..=6);
    if cor as i16 > -1 {
        if (lance == 0x2c || lance == 0x2f) && vantagem != 0 {
            narrar(chamar, if lance == 0x2f { 0x21da } else { 0x21d9 }, ub(p, 0x2e), ub(p, 0x27), par(e));
            if expulso || som == 0x1f7a {
                *b(e, 0x16) = 1;
            }
        }
        if sortear(4) == 0 {
            let x = sortear(grav as i32);
            if x > sortear(0x14) {
                let t = u(m, 0x4792);
                let clube = u(t, if byte(e, 0x27) == 0 { 0x20 } else { 0x1c });
                let k = u(clube, 0x53);
                if k != 0 && byte(k, 0x77) == 8 {
                    narrar(chamar, 0x20f5, 0, 0, u32::MAX);
                }
            }
            narrar(chamar, 0x2109, ub(e, 0x2e), ub(e, 0x27), u32::MAX);
        }
        narrar(chamar, cor as u32, ub(e, 0x2e), ub(e, 0x27), u32::MAX);
        if expulso {
            crate::comparacao::ramo("passe expulsao");
            jogador_saiu_com(m, e, 0, chamar);
        }
    }
    if som != 0x1f7a {
        let c = escolher_cobrador(m, lado, byte(m, 0x8eb2), 0);
        if c == 0 {
            narrar(chamar, som, 0xff, ub(p, 0x27), u32::MAX);
        } else {
            if tipo == 7 {
                narrar(chamar, 0x21e2, ub(c, 0x2e), ub(p, 0x27), u32::MAX);
            } else {
                narrar(chamar, som, 0xff, ub(p, 0x27), u32::MAX);
            }
            let longe = (lado == 1 && byte(p, 0x103) < 4) || (lado == 0 && byte(p, 0x103) > 7);
            if som == 0x1f79 && vantagem == 0 && longe {
                if tipo == 7 {
                    if !na_area_atacada(lado, byte(p, 0x102), byte(p, 0x103)) {
                        narrar(chamar, 0x21e1, ub(c, 0x2e), ub(p, 0x27), u32::MAX);
                    }
                } else if !(2..=6).contains(&byte(p, 0x102)) {
                    narrar(chamar, 0x21df, ub(c, 0x2e), ub(p, 0x27), u32::MAX);
                } else {
                    narrar(chamar, 0x21e0, ub(c, 0x2e), ub(p, 0x27), u32::MAX);
                }
            }
        }
    }
    if codigo_especial(lance) == 0 {
        let extra = expulso || som == 0x1f7a;
        if vantagem != 0 {
            narrar(chamar, 0x20dd, ub(p, 0x2e), ub(p, 0x27), u32::MAX);
            if tipo == 4 || tipo == 5 {
                *b(e, 0x16) = 1;
            }
            if extra && sortear(2) == 0 {
                narrar(chamar, 0x20d9, u32::MAX, zero(p), u32::MAX);
            }
        } else if sortear(9) == 0 {
            narrar(chamar, 0x20df, u32::MAX, zero(p), u32::MAX);
            if sortear(3) == 0 {
                narrar(chamar, 0x20e0, u32::MAX, zero(p), u32::MAX);
            }
            if extra && sortear(2) == 0 {
                narrar(chamar, 0x20d9, u32::MAX, zero(p), u32::MAX);
            }
        }
        aplicar_ao_time(m, e, peso);
        chamar(0x716e10, e, [0; 5]);
    }
    *w(e, 0x35) = (*w(e, 0x35)).wrapping_sub(0x19);
}

/// Os oito codigos de narracao pela direcao do passe (`bl` de 0x6fad40: 0x10 pela faixa, 0x20/0x40
/// para a ponta, 2/4 de lado, 1 para frente, 8 para tras), na ordem 0x10, 0x20, 0x40, 2, 4, 1, 8.
fn pela_direcao(bl: u8, c: [u32; 7], padrao: u32) -> u32 {
    for (k, f) in [0x10u8, 0x20, 0x40, 2, 4, 1, 8].iter().enumerate() {
        if bl & f != 0 {
            return c[k];
        }
    }
    padrao
}

/// { 0x6fad40 } (ret 0x1c, match_pl.cpp, ~22 KB) com as chamadas que mexem fora do motor injetadas:
/// a execucao do passe de `p` (tipo 6..=0x5e) de (x, y) para (tx, ty), com `receptor` (ou 0) e a
/// saida (1 quando o passe nao chega ao receptor). A nota do passe pelo tipo e pela distancia; o
/// desafiante (0x6c87e0) com a falta (como na dividida) ou o desarme; sem ele, o folego, o passe
/// impreciso (o alvo sorteado pelo fator de +0x4756), o custo em +0x35, a narracao da aproximacao, a
/// narracao do tipo de passe (pela direcao, pela saida e por +0x1a7 do motor), a bola parada seguinte
/// (+0x8ea9, +0x8eaa) e o tracado (0x6ccb60): pela tabela, chute (0x70c040/0x70a910) ou a bola em
/// transito (+0x1c5..+0x1c9, +0x477e) e bola fora (0x6cd490). Devolve 1 se a bola segue em jogo.
#[allow(clippy::too_many_arguments, clippy::cognitive_complexity)]
unsafe fn executar_passe_com(p: usize, x: i8, y: i8, tipo_in: i8, saida: *mut u8, receptor: usize, tx_in: i8, ty_in: i8, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) -> u8 {
    let b = |q: usize, o: usize| (q + o) as *mut u8;
    let w = |q: usize, o: usize| (q + o) as *mut i16;
    let ub = |q: usize, o: usize| *((q + o) as *const u8) as u32;
    let u = |q: usize, o: usize| *((q + o) as *const usize);
    let m = u(p, 0x19e);
    let lado = byte(p, 0x27);
    let (mut tipo, mut tx, mut ty) = (tipo_in, tx_in, ty_in);
    // narracao com sete argumentos (a5 | a6 << 8 e a7 << 16 no codigo)
    let narrar7 = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: i8| {
        chamar(0x6d6f70, u(m, 0x8f7a), [codigo | (a7 as u8 as u32) << 16, m as u32, a3, a4, (a5 & 0xff) | (a6 & 0xff) << 8]);
    };
    // o folego e a distancia
    let r0 = sortear({
        let v = 10000 - ftol(f32_em(p + 0x4d) as f64) as i16 as i32;
        v * v / 50000
    }) as i16 as i32;
    let mut d = distancia_entre(x, y, tx, ty);
    let k = if (f32_em(p + 0xc5) as f64) < f32_em(0x968e70) as f64 { f32_em(0x968e70) as f64 } else { f32_em(p + 0xc5) as f64 };
    let mut l14 = sortear(ftol(d as f64 * d as f64 * d as f64 * d as f64 * f32_em(0x96d228) as f64 / k));
    if receptor == 0 {
        l14 = sortear(l14 as i16 as i32);
    }
    if (d as f64) < f64_em(0x968950) && tipo != 0x3d && tipo != 0x5c {
        l14 = l14.wrapping_add(ftol(f32_em(0x96d210) as f64 - d as f64 * f32_em(0x96d7c0) as f64));
    }
    let cinco = |v: f64| ((ftol(v * f32_em(0x96d754) as f64 + f64_em(0x968e18)) as i8 as i32 / 5) * 5) as i8;
    let mut l12: i8 = 1;
    let c5 = cinco(d as f64);
    if c5 as i32 >= 1 {
        l12 = c5;
    }
    if tipo != 0x36 && tipo != 0x5c && tipo != 0x3d {
        *b(m, 0x1a5 + lado as isize as usize) = 1;
    }
    let mut saida_lance = 0u16;
    classificar_lance(p, tipo as i16, x, y, tx, ty, (d as f64 + f64_em(0x967890)) as f32, &mut saida_lance);
    let l20 = sortear(saida_lance as i16 as i32);
    // a nota do passe pelo tipo
    let mut l13 = false;
    let wp = |o: usize| *w(p, o);
    let mut bp: i16 = match tipo {
        6..=8 => nota_do_passe_pela_posicao(p, wp(0x11a + tipo as usize * 2), l20, l14),
        0x5e => nota_do_passe_pela_posicao(p, wp(0x12a), l20, l14),
        0x34 | 0x37 => {
            l13 = true;
            wp(0x182).wrapping_sub((l14 as i16 as i32 / 5) as i16).wrapping_add(300)
        }
        0x36 => {
            l13 = true;
            wp(0x186).wrapping_sub((l14 as i16).wrapping_add(l14 as i16))
        }
        0x5a => {
            l13 = true;
            wp(0x156).wrapping_sub(l14 as i16).wrapping_add(300)
        }
        0x5b | 0x5c => wp(0x194).wrapping_sub(l14 as i16),
        0x5d => wp(0x130).wrapping_sub(l20 as i16).wrapping_sub(l14 as i16),
        _ => wp(0x11a + (tipo as isize * 2) as usize).wrapping_sub(l20 as i16).wrapping_sub(l14 as i16),
    };
    if byte(m, 0x8ea9) == 5 && d as f64 > f64_em(0x96d700) {
        let (n, c) = match byte(m, 0x4756) {
            1 => (7, 0x9689d0),
            2 => (5, 0x96af30),
            3 => (3, 0x9689d8),
            4 => (3, 0x96a678),
            _ => (0, 0),
        };
        if n != 0 && sortear(n) == 0 {
            crate::comparacao::ramo("passe disputa de penaltis");
            bp = bp.wrapping_mul(ftol(bp as f64 * f64_em(c)) as i16);
        }
    }
    let r1 = sortear(r0 as i16 as i32);
    // a nota do ataque: so os 16 bits baixos sao lidos depois
    let l44 = wp(0x19c).wrapping_sub(r1 as i16).wrapping_add(sortear(bp as i32) as i16).wrapping_add(wp(0x29));
    // o desafiante
    let (mut a, mut bb, mut nota, mut lance, mut arriscado) = (0u8, 0u8, 0i16, 0i8, 0i8);
    let e = if byte(m, 0x8ea9) == 4 || l13 { 0 } else { desafio_adversario(m, p, lado, 0x6c, &mut a, &mut bb, &mut nota, &mut lance, &mut arriscado, 0) };
    lesao_na_dividida_com(m, p, 0x6c, e, lance as i16, 0, chamar);
    if e != 0 && nota as i32 >= wp(0x19c) as i32 + 200 {
        let (mut cartao, mut grav, mut vantagem) = (0u8, 0i8, 0u8);
        let tipo_falta = falta_no_motor(m, e, p, lance, 0x6c, &mut cartao, &mut grav, &mut vantagem);
        if byte(m, 0x8ebb) != 0
            && na_area_atacada(lado, byte(e, 0x102), byte(e, 0x103))
            && (codigo_especial(lance) != 0 || vantagem != 0 || sortear(*w(e, 0x31) as i32) < -10)
            && sortear(3) != 0
        {
            narrar7(chamar, 0x2168, ub(e, 0x2e), ub(e, 0x27), 0xff, 0xff, 0);
        }
        if tipo_falta != 0 {
            if sortear(10) == 0 && arriscado == 0 {
                crate::comparacao::ramo("passe falta so registra");
                registrar_lance(m, lance as i16 as u16, e as u32, p as u32, 0, tipo_falta, *b(p, 0x102), *b(p, 0x103), cartao, grav as u8);
            } else {
                crate::comparacao::ramo("passe falta punida");
                falta_no_passe_com(m, e, p, tipo_falta, cartao, grav, arriscado, vantagem, lance, chamar);
                return 0;
            }
        }
        // o registro do lance pula a conta do penalti (0x6fb667 vai direto a 0x6fc0e5)
        let penalti = tipo_falta == 0 && vantagem != 0 && (na_area_atacada(lado, byte(p, 0x102), byte(p, 0x103)) || arriscado != 0);
        crate::comparacao::ramo("passe desarmado");
        passe_desarmado_com(m, p, e, lance, nota, a, vantagem, penalti, chamar);
        return 0;
    }
    // sem desafiante: o folego de quem passa
    if !na_area_atacada(lado, byte(p, 0x102), byte(p, 0x103)) && sortear(*w(p, 0x31) as i32) < -10 {
        crate::comparacao::ramo("passe sem folego");
        let pessoa = u(p, 0x69);
        let mut v = f32_em(p + 0x1a2) as f64 * f32_em(0x96d3b8) as f64;
        *((p + 0x1a2) as *mut f32) = v as f32;
        let lim = ((25 - byte(pessoa, 0x5d) as i32) * 2) as f32;
        if !(v > lim as f64) {
            v = lim as f64;
        }
        *((p + 0x1a2) as *mut f32) = v as f32;
        if v > f64_em(0x9688a8) {
            *((p + 0x1a2) as *mut f32) = 100.0;
        }
        narrar7(chamar, 0x211d, u32::MAX, (lado == 0) as u32, 0xff, 0xff, 0);
        if sortear(byte(pessoa, 0x5a) as i32 * 20 + 10) == 0 {
            let r = sortear(3);
            if ajustar_17(p, (r as u8).wrapping_neg() as i8) != 0 {
                desempenho_na_partida(p, 0);
            }
        }
    }
    *((m + 0xf576) as *mut usize) = p;
    *w(m, 0x8d4a) = tipo as i16;
    *b(p, 1) = (*b(p, 1)).wrapping_add(1);
    if sortear(byte(u(p, 0x69), 0x5a) as i32 * 20 + 10) == 0 {
        let r = sortear(3);
        if ajustar_17(p, (r as u8).wrapping_neg() as i8) != 0 {
            desempenho_na_partida(p, 0);
        }
    }
    let mut l24 = 0i32;
    if u(m, 0xf582) != 0 {
        l24 = byte(m, 0x1a7) as i32;
        *((m + 0x4786) as *mut usize) = 0;
        *b(m, 0x8eae) = 0xff;
        *((m + 0xf582) as *mut usize) = 0;
        *b(m, 0x1a7) = 0;
        *w(p, 0x29) = 0;
        *w(p, 0x19c) = 0;
    }
    *w(m, 0x8ed6) = l44.wrapping_sub(nota);
    *b(m, 0x8eb9) = x as u8;
    *b(m, 0x8ea6) = y as u8;
    // a precisao do passe
    let mut fator = (byte(m, 0x8ea9) as f64 * f64_em(0x968e80) + f64_em(0x967890)) as f32;
    let c = match byte(m, 0x4756) {
        1 => 0x96d7bc,
        2 => 0x96d3d4,
        3 => 0x96d7b8,
        4 => 0x96d7b4,
        _ => 0,
    };
    if c != 0 {
        fator = (fator as f64 * f32_em(c) as f64) as f32;
    }
    let r = sortear(ftol(fator as f64 * fator as f64 * f32_em(0x968a78) as f64));
    if (l44 as i32 - nota as i32) < r {
        crate::comparacao::ramo("passe impreciso");
        let e8 = ftol((fator as f64 * d as f64 + f64_em(0x967890)) * f64_em(0x968968)) as i8 as i32;
        let s1 = sortear(e8);
        let s2 = sortear(e8);
        let nx = (tx as u8).wrapping_sub(s1 as u8).wrapping_add(s2 as u8) as i8;
        let ny = if m != 0 && tabela_a(m, p) == 1 && sortear((wp(0x29) as i32 + 400) * 50) != 0 {
            ty
        } else {
            let s1 = sortear(e8);
            let s2 = sortear(e8);
            (ty as u8).wrapping_sub(s1 as u8).wrapping_add(s2 as u8) as i8
        };
        if nx != tx || ny != ty {
            if receptor != 0 {
                if m != 0 && tabela_a(m, p) == 1 {
                    *w(p, 0x35) = wp(0x35).wrapping_sub(0x32);
                } else if na_metade_de_defesa(lado, y) && na_metade_de_defesa(lado, ny) {
                    *w(p, 0x35) = wp(0x35).wrapping_sub(0x6e);
                }
                *w(p, 0x35) = wp(0x35).wrapping_sub(if matches!(tipo, 8 | 7 | 0x5e | 6) { 0xf } else { 0x64 });
            }
            ty = ny;
            tx = nx;
            d = distancia_entre(x, y, nx, ny);
        }
    }
    // o custo do passe
    if receptor != 0 {
        if *saida != 0 && limite(lado as u8, ty) != 0 {
            *w(p, 0x35) = wp(0x35).wrapping_sub(if m != 0 && tabela_a(m, p) == 1 { 0xd } else { 0x14 });
        } else {
            if m != 0 && tabela_a(m, p) == 1 {
                *w(p, 0x35) = wp(0x35).wrapping_sub(0x42);
            } else if na_metade_de_defesa(lado, y) && na_metade_de_defesa(lado, ty) {
                *w(p, 0x35) = wp(0x35).wrapping_sub(0x64);
            }
            *w(p, 0x35) = wp(0x35).wrapping_sub(if matches!(tipo, 8 | 7 | 6) { 0x14 } else { 0x48 });
        }
    } else {
        *w(p, 0x35) = wp(0x35).wrapping_sub(0x24);
    }
    // a narracao da aproximacao (para onde ele leva a jogada)
    let acao = wp(0x198);
    let setpiece = matches!(tipo, 6 | 7 | 8 | 0x5e);
    if !matches!(acao, 0x12 | 0x11 | 0x10 | 0xf) && tipo != 8 && !setpiece && !l13 {
        let r = sortear(0x14);
        if l12 as i32 >= 30 - r && byte(m, 0x8ea9) < 3 && !(m != 0 && tabela_a(m, p) == 1) && sortear(3) == 0 {
            let t = if lado == 0 { tabela_12x9(byte(p, 0x103) as i32 - 0xb, byte(p, 0x102) as i32 - 4) } else { distancia_ao_ponto(p, 4, 0) };
            let c5 = cinco(t as f64);
            let n5: i8 = if (c5 as i32) < 1 { 1 } else { c5 };
            let (px, py) = (byte(p, 0x102), byte(p, 0x103));
            let fundo = (lado == 0 && py > 7) || (lado == 1 && py < 4);
            let recuado = (lado == 0 && py < 4) || (lado == 1 && py > 7);
            let canto_proprio = (lado == 0 && px < 2) || (lado == 1 && px > 6);
            let canto_outro = (lado == 0 && px > 6) || (lado == 1 && px < 2);
            let codigo = if fundo {
                if canto_proprio {
                    Some(0x2160)
                } else if canto_outro {
                    Some(0x2162)
                } else if na_area_atacada(lado, px, py) {
                    None
                } else {
                    Some(0x2161)
                }
            } else if recuado {
                Some(if canto_proprio { 0x215a } else if canto_outro { 0x215c } else { 0x215b })
            } else if (lado == 0 && px < 2) || (lado == 1 && px > 6) {
                Some(0x215d)
            } else if canto_outro {
                Some(0x215f)
            } else if (px as i32 - 4).abs() < 2 && (py == 5 || py == 6) && sortear(2) != 0 {
                Some(0x215e)
            } else {
                None
            };
            if let Some(cn) = codigo {
                crate::comparacao::ramo("passe aproximacao narrada");
                narrar7(chamar, cn, ub(p, 0x2e), lado as u8 as u32, 0xff, 0xff, n5);
            }
        }
    }
    // a direcao do passe e a narracao pelo tipo
    let mut l54: i32 = -1;
    let rp = |o: usize| if receptor != 0 { ub(receptor, o) } else { 0 };
    let (bl, al): (u8, u8);
    if setpiece {
        let mut l13b = 0u8;
        if (lado == 0 && x < 3) || (lado == 1 && x > 5) {
            l13b = 1;
        } else if (lado == 1 && x < 3) || (lado == 0 && x > 5) {
            l13b = 2;
        }
        let dif = ((x as i32 - 4).abs() as u8).wrapping_sub((x as i32 - tx as i32).abs() as u8) as i8;
        bl = l13b;
        al = if dif > 0 { l13b | 8 } else if dif < 0 { l13b | 4 } else { l13b };
    } else {
        let mut f = 0u8;
        if receptor != 0 && u(receptor, 0x19e) != 0 && tabela_a(u(receptor, 0x19e), receptor) == 1 {
            f = 0x80;
        }
        if (lado == 0 && ty > y) || (lado == 1 && ty < y) {
            f |= 1;
        } else if (lado == 0 && ty < y) || (lado == 1 && ty > y) {
            f |= 8;
        }
        let dy = (ty as i32 - y as i32).abs();
        if dy < 2 {
            let dx = (tx as i32 - x as i32).abs();
            if ((lado == 0 && tx < x) || (lado == 1 && tx > x)) && dy <= dx {
                f |= 2;
            } else if ((lado == 0 && tx > x) || (lado == 1 && tx < x)) && dy <= dx {
                f |= 4;
            }
        }
        if !na_area_atacada(lado, byte(p, 0x102), byte(p, 0x103)) && faixa((lado == 0) as u8, tx, ty) != 0 {
            f |= 0x10;
        }
        let longo = (lado == 1 && byte(p, 0x103) < 3) || (lado == 0 && byte(p, 0x103) > 8) || (lado == 0 && ty as i32 > y as i32 + 3) || (lado == 1 && (ty as i32) < y as i32 - 3);
        if longo {
            if fora_do_meio(tx) != 0 && ((lado == 0 && tx < byte(p, 0x102) && tx < 2) || (lado == 1 && tx > byte(p, 0x102) && tx > 6)) {
                f |= 0x20;
            } else if fora_do_meio(tx) != 0 && ((lado == 0 && tx > byte(p, 0x102) && tx > 6) || (lado == 1 && tx < byte(p, 0x102) && tx < 2)) {
                f |= 0x40;
            }
        }
        if lado == 0 && tx == 4 && ty == 0xc && (d as f64) < f64_em(0x968950) {
            tipo = if tipo != 0x3d { 0x5d } else { 0x5c };
        } else if lado == 1 && tx == 4 && ty == -1 && (d as f64) < f64_em(0x968950) {
            tipo = if tipo != 0x3d { 0x5d } else { 0x5c };
        } else if byte(m, 0x8ea9) < 4 && tipo != 0x5d && tipo != 0x5c && tx == 4 && (ty == -1 || ty == 0xc) {
            l54 = 0x211f;
        }
        bl = f;
        al = l13 as u8;
    }
    let n = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32| narrar7(chamar, codigo, ub(p, 0x2e), lado as u8 as u32, 0xff, 0xff, l12);
    let nr = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, a7: i8| narrar7(chamar, codigo, ub(p, 0x2e), lado as u8 as u32, rp(0x2e), rp(0x27), a7);
    let grupo = |saida_v: u8, l24: i32, c: [[u32; 7]; 4]| -> [u32; 7] {
        match (saida_v != 0, l24 != 0) {
            (false, false) => c[0],
            (false, true) => c[1],
            (true, false) => c[2],
            (true, true) => c[3],
        }
    };
    let de_lado = |al: u8, c: [u32; 3]| if al & 8 != 0 { c[0] } else if al & 4 != 0 { c[1] } else { c[2] };
    match tipo {
        9 => {
            if (ty >= 0xc && lado == 1) || (ty < 0 && lado == 0) {
                n(chamar, if tx != 4 && sortear(4) != 0 { 0x210e } else { 0x211a });
            } else {
                let f: unsafe extern "thiscall" fn(usize, u8) -> u32 = core::mem::transmute(0x6f5b20usize);
                n(chamar, if f(p, lado as u8) as u8 == 0 { 0x2157 } else { 0x1f70 });
            }
        }
        0x3e => n(chamar, 0x2134),
        0x5b => {
            if (0..0xc).contains(&ty) {
                n(chamar, 0x1f6f);
            } else {
                n(chamar, if tx != 4 && sortear(4) != 0 { 0x210d } else { 0x2119 });
            }
        }
        0x5d => n(chamar, pela_direcao(bl, [0x20b2, 0x20b3, 0x20b4, 0x20af, 0x20b0, 0x20ae, 0x20b1], 0x1f72)),
        6 | 8 | 0x5e => {
            let codigo = if al & 1 != 0 {
                if l24 != 0 { de_lado(al, [0x20c3, 0x20c1, 0x20bf]) } else { de_lado(al, [0x20bd, 0x20bb, 0x20b9]) }
            } else if al & 2 != 0 {
                if l24 != 0 { de_lado(al, [0x20c4, 0x20c2, 0x20c0]) } else { de_lado(al, [0x20be, 0x20bc, 0x20ba]) }
            } else {
                0x1f73
            };
            narrar7(chamar, codigo, ub(p, 0x2e), lado as u8 as u32, 0xff, 0xff, 0);
        }
        7 => {
            let codigo = if al & 1 != 0 {
                if l24 != 0 { de_lado(al, [0x20cf, 0x20cd, 0x20cb]) } else { de_lado(al, [0x20c9, 0x20c7, 0x20c5]) }
            } else if al & 2 != 0 {
                if l24 != 0 { de_lado(al, [0x20d0, 0x20ce, 0x20cc]) } else { de_lado(al, [0x20ca, 0x20c8, 0x20c6]) }
            } else {
                0x21e4
            };
            narrar7(chamar, codigo, ub(p, 0x2e), lado as u8 as u32, 0xff, 0xff, 0);
        }
        0x5c => n(chamar, 0x1f71),
        0x3d => {
            let rr = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32| narrar7(chamar, codigo, ub(p, 0x2e), lado as u8 as u32, rp(0x2e), rp(0x27), 0);
            if bl & 0x80 != 0 && bl & 1 == 0 {
                rr(chamar, 0x2148);
            } else if faixa((lado == 0) as u8, tx, ty) != 0 {
                rr(chamar, 0x21d5);
            } else if bl & 2 != 0 {
                rr(chamar, 0x20b6);
            } else if bl & 4 != 0 {
                rr(chamar, 0x20b7);
            } else if bl & 1 != 0 {
                rr(chamar, 0x20b5);
            } else {
                rr(chamar, if bl & 8 != 0 { 0x20b8 } else { 0x1f6d });
            }
        }
        0x34 => {
            if (d as f64) < f64_em(0x968930) {
                n(chamar, 0x2130);
            } else {
                n(chamar, if faixa((lado == 0) as u8, tx, ty) == 0 { 0x2131 } else { 0x1f80 });
            }
        }
        0x5a => n(chamar, 0x1f7e),
        0x37 => n(chamar, if fora_do_meio(tx) != 0 { 0x21bc } else { 0x1f7d }),
        0x36 => {
            if receptor != 0 {
                nr(chamar, 0x1f7b, l12);
            } else {
                n(chamar, 0x1f7c);
            }
        }
        0x1f => {
            if bl & 0x80 != 0 && bl & 1 == 0 {
                nr(chamar, 0x214a, 0);
            } else {
                let c = if byte(m, 0x8ea9) == 0 {
                    grupo(*saida, l24, [
                        [0x2026, 0x2027, 0x2028, 0x2023, 0x2024, 0x2022, 0x2025],
                        [0x202d, 0x202e, 0x202f, 0x202a, 0x202b, 0x2029, 0x202c],
                        [0x2034, 0x2035, 0x2036, 0x2031, 0x2032, 0x2030, 0x2033],
                        [0x203b, 0x203c, 0x203d, 0x2038, 0x2039, 0x2037, 0x203a],
                    ])
                } else {
                    grupo(*saida, l24, [
                        [0x2042, 0x2043, 0x2044, 0x203f, 0x2040, 0x203e, 0x2041],
                        [0x2049, 0x204a, 0x204b, 0x2046, 0x2047, 0x2045, 0x2048],
                        [0x2050, 0x2051, 0x2052, 0x204d, 0x204e, 0x204c, 0x204f],
                        [0x2057, 0x2058, 0x2059, 0x2054, 0x2055, 0x2053, 0x2056],
                    ])
                };
                nr(chamar, pela_direcao(bl, c, 0x1f67), l12);
            }
        }
        0xb => {
            let area: unsafe extern "thiscall" fn(usize, u8) -> u32 = core::mem::transmute(0x6f5b20usize);
            if bl & 0x80 != 0 && bl & 1 == 0 {
                nr(chamar, 0x214a, 0);
            } else if area(receptor, (lado == 0) as u8) as u8 != 0 && area(p, (lado == 0) as u8) as u8 != 0 && (d as f64) < f64_em(0x968950) {
                nr(chamar, 0x2021, l12);
            } else {
                let c = grupo(*saida, l24, [
                    [0x205e, 0x205f, 0x2060, 0x205b, 0x205c, 0x205a, 0x205d],
                    [0x2065, 0x2066, 0x2067, 0x2062, 0x2063, 0x2061, 0x2064],
                    [0x206c, 0x206d, 0x206e, 0x2069, 0x206a, 0x2068, 0x206b],
                    [0x2073, 0x2074, 0x2075, 0x2070, 0x2071, 0x206f, 0x2072],
                ]);
                nr(chamar, pela_direcao(bl, c, 0x1f69), l12);
            }
        }
        0x1e => {
            if bl & 0x80 != 0 && bl & 1 == 0 {
                nr(chamar, 0x214c, 0);
            } else {
                let c = grupo(*saida, l24, [
                    [0x2096, 0x2097, 0x2098, 0x2093, 0x2094, 0x2092, 0x2095],
                    [0x209d, 0x209e, 0x209f, 0x209a, 0x209b, 0x2099, 0x209c],
                    [0x20a4, 0x20a5, 0x20a6, 0x20a1, 0x20a2, 0x20a0, 0x20a3],
                    [0x20ab, 0x20ac, 0x20ad, 0x20a8, 0x20a9, 0x20a7, 0x20aa],
                ]);
                nr(chamar, pela_direcao(bl, c, 0x1f6a), l12);
            }
        }
        0x3c => {
            if bl & 0x80 != 0 && bl & 1 == 0 {
                nr(chamar, 0x214c, 0);
            } else {
                let c = grupo(*saida, l24, [
                    [0x207a, 0x207b, 0x207c, 0x2077, 0x2078, 0x2076, 0x2079],
                    [0x2081, 0x2082, 0x2083, 0x207e, 0x207f, 0x207d, 0x2080],
                    [0x2088, 0x2089, 0x208a, 0x2085, 0x2086, 0x2084, 0x2087],
                    [0x208f, 0x2090, 0x2091, 0x208c, 0x208d, 0x208b, 0x208e],
                ]);
                nr(chamar, pela_direcao(bl, c, 0x1f6b), l12);
            }
        }
        _ => {
            // ponytail: o original para o jogo ("MATCH_PLAYER::pass() - unknown pass"); os chamadores so usam os 16 tipos acima
            linha(&format!("motor: executar_passe: tipo desconhecido {:#x}", tipo));
            std::process::abort();
        }
    }
    if l54 as i16 > -1 {
        n(chamar, l54 as u32);
    }
    // a bola parada seguinte (+0x8ea9) e o tempo dela (+0x8eaa)
    let (v, t): (i8, i8) = match tipo {
        0x34 | 0x37 => {
            let v = if (d as f64) < f64_em(0x968930) && faixa((lado == 0) as u8, tx, ty) == 0 {
                0
            } else if sortear(0x28) != 0 {
                4
            } else {
                sortear(3) as i8
            };
            (v, sortear(3) as i8 + 1)
        }
        0x1e | 8 => (4, sortear(3) as i8 + 1),
        0x3c | 0x1f | 0x20 => {
            let r = if sortear(3) != 0 { sortear(4) } else { 0 };
            (4 - r as i8, sortear(2) as i8 + 1)
        }
        0xb | 7 => ((sortear(7) == 0) as i8, sortear(2) as i8),
        9 | 0x3e | 0x5a => {
            let r = if sortear(0x23) != 0 { 0 } else { sortear(2) };
            (4 - r as i8, sortear(2) as i8 + 2)
        }
        _ => {
            let v = if sortear(3) != 0 { 2 - sortear(2) as i8 } else { 4 - sortear(4) as i8 };
            (v, sortear(2) as i8)
        }
    };
    *b(m, 0x8ea9) = v as u8;
    *b(m, 0x8eaa) = t as u8;
    let mut bl2 = byte(m, 0x8ea9);
    let cai = (*saida != 0 && bl2 > 0 && sortear(3) == 0) || (bl2 < 4 && wp(0x19c) as i32 > sortear(200));
    if cai {
        bl2 = bl2.wrapping_sub(sortear(2) as i8);
        *b(m, 0x8ea9) = bl2 as u8;
    }
    let cy = ty.clamp(0, 0xb);
    let cx = tx.clamp(0, 8);
    *w(m, 0x8ed8) = por_diferenca(m, x, y, cx, cy) as i16;
    // 0x7003b0: o lado do passe e a conta de posse
    *b(m, 0x8eab) = lado as u8;
    if lado != -1 {
        *b(m, 0x8d46 + lado as isize as usize) = (*b(m, 0x8d46 + lado as isize as usize)).wrapping_add(1);
        let o = (m + 0x8d46 + (lado == 0) as usize) as *mut i8;
        if *o > 0 {
            *o = (*o).wrapping_sub(10);
        }
    }
    if receptor != 0 && (tx != byte(receptor, 0x102) || ty != byte(receptor, 0x103)) {
        *saida = 1;
    }
    let forca = if l44 > -100 { l44 as i32 } else { -100 };
    let r = chamar(0x6ccb60, m, [x as u8 as u32 | (y as u8 as u32) << 8 | (tx as u8 as u32) << 16 | (ty as u8 as u32) << 24, tipo as u8 as u32, forca as u32, 1, 0]) as u8;
    let fora = |m: usize| !((0..9).contains(&byte(m, 0x8ea7)) && (0..0xc).contains(&byte(m, 0x8ea8)));
    if r != 0 {
        if byte(m, 0x8eb2) != 0 {
            return 0;
        }
        if fora(m) {
            crate::comparacao::ramo("passe tracado sai");
            chamar(0x6cd490, m, [(lado == 0) as u32, 0, 0, 0, 0]);
        }
        return 0;
    }
    if tx == 4 && (ty == 0xc || ty == -1) {
        crate::comparacao::ramo("passe vira chute");
        let contra = (ty == -1) == (lado == 0);
        chamar(if contra { 0x70c040 } else { 0x70a910 }, p, [d.to_bits(), 0, 0, 0, 0]);
        if byte(m, 0x8eb2) != 0 {
            return 0;
        }
    }
    let v = byte(m, 0x8eaa) as i32;
    let k = if (v as f64 + f64_em(0x968918)) < f64_em(0x967890) { f64_em(0x967890) } else { byte(m, 0x8eaa) as f64 + f64_em(0x968918) };
    *b(m, 0x1c5) = ftol((d as f64 + d as f64) / k + f64_em(0x968968)) as u8;
    *b(m, 0x1c6) = tx as u8;
    *b(m, 0x1c7) = ty as u8;
    *b(m, 0x1c8) = *b(p, 0x102);
    *b(m, 0x1c9) = *b(p, 0x103);
    *((m + 0x477e) as *mut usize) = receptor;
    *b(m, 0x8ea7) = tx as u8;
    *b(m, 0x8ea8) = ty as u8;
    if fora(m) {
        crate::comparacao::ramo("passe em transito sai");
        chamar(0x6cd490, m, [lado as u8 as u32, 0, 0, 0, 0]);
    }
    (byte(m, 0x8eb2) == 0) as u8
}

/// Unidades da fita do passe: o evento 0x82 vai para a lista montado na pilha (confere o conteudo).
static GRAVAR_PASSE: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

fn gravando_passe() -> bool {
    GRAVAR_PASSE.load(std::sync::atomic::Ordering::Relaxed) && gravando()
}

/// Modo Rust: as chamadas da execucao do passe vao para o jogo (o tracado com as coordenadas num
/// dword, os chutes com a distancia em bits; o resto como na dividida e no movimento).
unsafe fn despachar_passe(rotina: usize, this: usize, a: [u32; 5]) -> u32 {
    match rotina {
        0x6ccb60 => {
            let f: unsafe extern "thiscall" fn(usize, i8, i8, i8, i8, i8, u32, u32) -> u8 = core::mem::transmute(rotina);
            f(this, a[0] as u8 as i8, (a[0] >> 8) as u8 as i8, (a[0] >> 16) as u8 as i8, (a[0] >> 24) as u8 as i8, a[1] as u8 as i8, a[2], a[3]) as u32
        }
        0x70c040 | 0x70a910 => {
            let f: unsafe extern "thiscall" fn(usize, f32) = core::mem::transmute(rotina);
            f(this, f32::from_bits(a[0]));
            0
        }
        _ => despachar_movimento(rotina, this, a),
    }
}

/// { 0x6fad40 } (ret 0x1c, match_pl.cpp): ver `executar_passe_com`.
#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn executar_passe(p: usize, x: u32, y: u32, tipo: u32, saida: *mut u8, receptor: usize, tx: u32, ty: u32) -> u8 {
    executar_passe_com(p, x as u8 as i8, y as u8 as i8, tipo as u8 as i8, saida, receptor, tx as u8 as i8, ty as u8 as i8, &mut |r, t, a| despachar_passe(r, t, a))
}

#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn comparar_executar_passe(p: usize, x: u32, y: u32, tipo: u32, saida: *mut u8, receptor: usize, tx: u32, ty: u32) -> u8 {
    if gravando() {
        // unidade da fita de fora (passe, lancamento): a resposta com o byte da saida; por dentro, a propria comparacao
        let mut r = 0u8;
        chamar_gravando(0x6fad40, p, [x, y, tipo, receptor as u32, tx], || {
            r = fita_do_passe(p, x, y, tipo, saida, receptor, tx, ty);
            r as u32 | ((*saida as u32) << 8)
        });
        return r;
    }
    fita_do_passe(p, x, y, tipo, saida, receptor, tx, ty)
}

#[allow(clippy::too_many_arguments)]
unsafe fn fita_do_passe(p: usize, x: u32, y: u32, tipo: u32, saida: *mut u8, receptor: usize, tx: u32, ty: u32) -> u8 {
    use std::sync::atomic::Ordering::Relaxed;
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32, u32, *mut u8, usize, u32, u32) -> u8>(321);
    let m = *((p + 0x19e) as *const usize);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(m, MOTOR), (0xb67a34, 4), (0xae1080, 4), (saida as usize, 1)]);
    let t = *((m + 0x4792) as *const usize);
    for clube in [*((t + 0x1c) as *const usize), *((t + 0x20) as *const usize)] {
        reg.extend(linhas_do_clube(clube));
    }
    let gravar = GRAVAR_716.swap(true, Relaxed);
    let a7 = GRAVAR_A7.swap(true, Relaxed);
    let passe = GRAVAR_PASSE.swap(true, Relaxed);
    let (r, rr, igual, sequencia, k) = comparar_com_fita(
        reg,
        || o(p, x, y, tipo, saida, receptor, tx, ty) as u32,
        |chamar| {
            executar_passe_com(p, x as u8 as i8, y as u8 as i8, tipo as u8 as i8, saida, receptor, tx as u8 as i8, ty as u8 as i8, &mut |rotina, this, args| {
                let mascara = match rotina {
                    0x6d6f70 => [0xff_ffff, u32::MAX, 0xff, 0xff, 0xff],
                    0x6cd770 => [0xff, 0xff, 0xffff, 0xff, u32::MAX],
                    0x6cd490 => [0xff, 0, 0, 0, 0],
                    0x62ed70 => [u32::MAX, 0, 0, 0, 0],
                    0x6d0980 => [0xff, 0xff, u32::MAX, u32::MAX, 0],
                    0x897760 => [u32::MAX, 0xff, u32::MAX, 0, 0],
                    0x68d0d0 => [0, u32::MAX, 0, 0, 0],
                    0x6ccb60 => [u32::MAX, 0xff, u32::MAX, u32::MAX, 0],
                    0x70c040 | 0x70a910 => [u32::MAX, 0, 0, 0, 0],
                    0x6e1aa0 | 0x716c90 | 0x6f0630 | 0x714d40 => [0, 0, 0, 0, 0],
                    _ => [0xff, 0, 0, 0, 0],
                };
                chamar(rotina, this, args, mascara)
            }) as u32
        },
    );
    GRAVAR_PASSE.store(passe, Relaxed);
    GRAVAR_A7.store(a7, Relaxed);
    GRAVAR_716.store(gravar, Relaxed);
    PLACAR.registrar(igual && sequencia && (r as u8) == (rr as u8), || format!("executar_passe(tipo {:#x}) original={} rust={} {} chamadas: {}", tipo as u8, r as u8, rr as u8, k, detalhe_fita()));
    r as u8
}
