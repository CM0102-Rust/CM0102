// ---------- centesima quadragesima terceira leva: o movimento do jogador (0x6f09f0) ----------

/// Unidades da fita do movimento: a 0x68d0d0 recebe o evento 0x82 montado na pilha (confere o conteudo).
static GRAVAR_MOVIMENTO: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

fn gravando_movimento() -> bool {
    GRAVAR_MOVIMENTO.load(std::sync::atomic::Ordering::Relaxed) && gravando()
}

/// A area atacada pelo lado (colunas 2 a 6; linhas 10-11 para o lado 0, 0-1 para o lado 1).
fn na_area_atacada(lado: i8, x: i8, y: i8) -> bool {
    (2..=6).contains(&x) && if lado == 0 { (10..=11).contains(&y) } else { (0..=1).contains(&y) }
}

/// Os jogadores marcados por `p` (+0x101 deles em +0x1ae) que ainda podem ser escolhidos (0x6bce20)
/// na casa dele saem do lugar (0x714d40) se +0x107 x +0x104 nao passar de sorteio(900) + 50.
unsafe fn soltar_marcados(m: usize, p: usize, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let mut i = 0i8;
    while i < byte(p, 0x101) {
        let q = *((p + 0x1ae + i as usize * 4) as *const usize);
        if pode_escolher(m, q, byte(p, 0x102), byte(p, 0x103)) as u8 != 0 {
            let a = byte(p, 0x107) as i32;
            let b = byte(p, 0x104) as i32;
            if a * b <= sortear(900) + 0x32 {
                chamar(0x714d40, q, [0; 5]);
            }
        }
        i = i.wrapping_add(1);
    }
}

/// O passo de uma casa em direcao ao alvo.
fn passo(de: i8, para: i8) -> i8 {
    if de < para {
        de + 1
    } else if de > para {
        de - 1
    } else {
        de
    }
}

/// { 0x6f09f0 } (ret 0x14, match_pl.cpp) com as chamadas que mexem fora do motor injetadas (0x6d6f70
/// narra, 0x714d40 os marcados saem, 0x6f0630 dominio, 0x716e10/0x716c90, 0x6cd770 bola parada,
/// 0x6cd490 bola fora, 0x68d0d0 o evento; e as da lesao, da falta e da saida de campo): o jogador `p`
/// anda uma casa (duas, se for rapido) para (x, y) com a acao `codigo` (0xf, 0x11, 0x12...). Ja no
/// alvo (fora de 0xf e 0x100) so solta os marcados. Sem a bola: lesao na dividida (0x6cfe10) e, sem
/// estar virado para o alvo (0x6fa140), so vira (0x6f4920, +0x198 = 0x6f). Com a bola: o ataque pelo
/// folego, +0x19c e a acao, a narracao da aproximacao (0x2156..0x2162), o desafiante (0x6c87e0) com a
/// falta (0x6cfc60/0x6cfcd0, como na dividida, com o evento 0x82) ou a disputa limpa; vencendo o
/// desafiante, o lance dele (0x70d430 no 1) e o fim como na dividida; passando, a posse, +0x19c,
/// as contas do lance e as narracoes da acao (0x1f48/0x1f49; 0x2009, 0x20d1..0x20d3, 0x1f4a.. no 0x11
/// e 0x12); perdendo o controle (+0x1d ou sorteio), 0x1f42. Depois anda na grade (+0x215e, contas e
/// vagas por lado), o mais adiantado (+0xf596/+0xf59a), solta os marcados, desgasta (0x6f56c0) pela
/// distancia, pelo papel e pelas marcas da posicao (0x5b5270), e a bola: com ela acompanha
/// (+0x8ea7/+0x8ea8, fora 0x6cd490); sem ela, na bola livre a domina (0x1f46/0x2113/0x1f60). Por
/// fim vira para um terco do caminho ate a bola.
#[allow(clippy::cognitive_complexity)]
unsafe fn mover_com(p: usize, x: i8, y: i8, codigo: i16, conduz_in: u8, modo: u8, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let m = *((p + 0x19e) as *const usize);
    let u = |q: usize, o: usize| *((q + o) as *const usize);
    let b = |q: usize, o: usize| (q + o) as *mut u8;
    let w = |q: usize, o: usize| (q + o) as *mut i16;
    let ub = |q: usize, o: usize| *((q + o) as *const u8) as u32;
    let lado = byte(p, 0x27);
    let outro = (lado == 0) as u8;
    let par = |q: usize| ub(q, 0x2e) | ub(q, 0x27) << 8;
    let narrar = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, a3: u32, a4: u32, a5: u32, a7: i8| {
        chamar(0x6d6f70, u(m, 0x8f7a), [codigo | (a7 as u8 as u32) << 16, m as u32, a3, a4, a5]);
    };
    let tabela1 = |q: usize| u(q, 0x19e) != 0 && tabela_do_jogador(q) == 1;
    let somar = |q: usize, d: i16| *w(q, 0x35) = (*w(q, 0x35)).wrapping_add(d);
    let fixar_19c = |q: usize, v: i16| *w(q, 0x19c) = v.clamp(-200, 200);
    let mut conduz = conduz_in;
    let (ox, oy) = (byte(p, 0x102), byte(p, 0x103));
    *b(p, 0x2b) = (*b(p, 0x2b)).wrapping_add(1);
    let mut perdeu = false;
    if x == ox && y == oy && codigo != 0xf && codigo != 0x100 {
        soltar_marcados(m, p, chamar);
        return;
    }
    let com_bola = || p == u(m, 0xf582);
    if !com_bola() {
        lesao_na_dividida_com(m, p, codigo, 0, -1, 1, chamar);
    }
    if !com_bola() && virado_para(p, x, y) == 0 && byte(p, 0x10c) == 0 && byte(p, 0x10a) == 0 && byte(p, 0x105) == 0 {
        crate::comparacao::ramo("movimento so vira");
        apontar(p, byte(p, 0x102), byte(p, 0x103), x, y);
        soltar_marcados(m, p, chamar);
        *w(p, 0x198) = 0x6f;
        return;
    }
    let mut nx = passo(ox, x);
    let mut ny = passo(oy, y);
    if !com_bola() {
        let c = byte(p, 0x104) as i32;
        if c * c > sortear(1000) {
            nx = passo(nx, x);
            ny = passo(ny, y);
        }
    }
    'movimento: {
        if !com_bola() {
            break 'movimento;
        }
        crate::comparacao::ramo("movimento com a bola");
        // o ataque
        let d = 10000 - ftol(f32_em(p + 0x4d) as f64) as i16 as i32;
        let folego = sortear(d.wrapping_mul(d) / 50000) as i16;
        if conduz != 0 {
            let v = ftol(f32_em(p + 0x99) as f64 + f64_em(0x968ea8)) as i16;
            fixar_19c(p, (*w(p, 0x19c)).wrapping_add(v));
        }
        let base = *w(p, match codigo {
            0xf => 0x138,
            0x12 => 0x13e,
            _ => 0x13c,
        }) as i32;
        let mut at = sortear(base - folego as i32 + *w(p, 0x19c) as i32);
        match byte(m, 1) {
            1 | 2 => at += -1 - sortear(3),
            8 => at += -2 - sortear(5),
            0x20 => at += -3 - sortear(8),
            _ => {}
        }
        let c = byte(m, 0x8ea9) as i32;
        at -= sortear(c * c * c * 50);
        at += sortear(*w(p, 0x29) as i32 / 10 - 0x1e);
        let (mut a, mut b2, mut nota, mut lance, mut arriscado) = (0u8, 0u8, 0i16, 0i8, 0i8);
        let e = desafio_adversario(m, p, lado, codigo as i8, &mut a, &mut b2, &mut nota, &mut lance, &mut arriscado, 0);
        if e != 0 {
            lesao_na_dividida_com(m, p, codigo, e, lance as i16, 0, chamar);
        }
        // a narracao da aproximacao
        let acao = *w(p, 0x198);
        if !(0xf..=0x12).contains(&acao) && !tabela1(p) && sortear(0x14) == 0 {
            let valor = || {
                let t = if lado == 0 {
                    let i = ((byte(p, 0x103) as i32 - 0xb).abs() as i8 as i32) * 9 + (byte(p, 0x102) as i32 - 4).abs() as i8 as i32;
                    f32_em(0xa15e44 + i as usize * 4)
                } else {
                    distancia_ao_ponto(p, 4, 0)
                };
                ftol(t as f64 * f32_em(0x96d754) as f64 + f64_em(0x968e18)) as i8
            };
            let v = valor();
            let n = if (v as i32 / 5) * 5 < 1 { 1i8 } else { (((valor() as i32 / 5) as u8).wrapping_mul(5)) as i8 };
            let (px, py) = (byte(p, 0x102), byte(p, 0x103));
            let canto_proprio = (lado == 0 && px < 2) || (lado == 1 && px > 6);
            let canto_outro = (lado == 0 && px > 6) || (lado == 1 && px < 2);
            let fundo = (lado == 0 && py > 7) || (lado == 1 && py < 4);
            let meio = (lado == 0 && (4..=7).contains(&py)) || (lado == 1 && (4..=7).contains(&py));
            let codigo_n: Option<u32> = if fundo {
                if canto_proprio {
                    Some(0x2160)
                } else if canto_outro {
                    Some(0x2162)
                } else if na_area_atacada(lado, px, py) {
                    None
                } else {
                    Some(0x2161)
                }
            } else if !meio && (lado == 0 || lado == 1) {
                // atras (lado 0: < 4; lado 1: > 7)
                if canto_proprio {
                    Some(0x215a)
                } else if canto_outro {
                    Some(0x215c)
                } else {
                    Some(0x215b)
                }
            } else if (lado == 0 && px < 2) || (lado == 1 && px > 6) {
                Some(0x215d)
            } else if canto_outro {
                Some(0x215f)
            } else if (px as i32 - 4).abs() < 2 && (py == 5 || py == 6) && sortear(2) != 0 {
                Some(0x215e)
            } else {
                None
            };
            if let Some(cn) = codigo_n {
                crate::comparacao::ramo("movimento aproximacao narrada");
                narrar(chamar, cn, ub(p, 0x2e), ub(p, 0x27), u32::MAX, n);
            }
        }
        let at16 = at as i16;
        let mut penalti = false;
        let mut vantagem = 0u8;
        if e != 0 {
            if !tabela1(e) {
                narrar(chamar, 0x1f4d, ub(p, 0x2e), ub(p, 0x27), par(e), 0);
            }
            let mut tipo = 0u8;
            if !(nota < at16 && sortear(5) != 0) {
                let (mut cartao, mut grav) = (0u8, 0i8);
                tipo = falta_no_motor(m, e, p, lance, -1, &mut cartao, &mut grav, &mut vantagem);
                if byte(m, 0x8ebb) != 0
                    && na_area_atacada(lado, byte(e, 0x102), byte(e, 0x103))
                    && (codigo_especial(lance) != 0 || vantagem != 0 || sortear(*w(e, 0x31) as i32) < -10)
                    && sortear(3) != 0
                {
                    narrar(chamar, 0x2168, ub(e, 0x2e), ub(e, 0x27), u32::MAX, 0);
                }
                if tipo != 0 {
                    if sortear(0x1e) != 0 || arriscado != 0 {
                        crate::comparacao::ramo("movimento falta punida");
                        punir_no_movimento_com(m, e, p, tipo, cartao, grav, arriscado, vantagem, lance, chamar);
                        return;
                    }
                    crate::comparacao::ramo("movimento falta so registra");
                    registrar_lance(m, lance as i16 as u16, e as u32, p as u32, 0, tipo, *b(p, 0x102), *b(p, 0x103), cartao, grav as u8);
                }
            }
            if tipo == 0 && vantagem != 0 {
                penalti = na_area_atacada(lado, byte(p, 0x102), byte(p, 0x103)) || arriscado != 0;
            }
            // a disputa limpa
            *w(e, 0x198) = lance as i16;
            *b(e, 4) = (*b(e, 4)).wrapping_add(1);
            let t = tabela_do_jogador(e);
            somar(e, if t & 0x30 != 0 && tabela_do_jogador(e) & 0x880 != 0 { -0x3b } else { -0x46 });
        }
        if byte(p, 0x1d) != 0 {
            perdeu = true;
        } else if modo != 4 && at16 < 1 {
            let r = sortear(ftol(f32_em(p + 0xe5) as f64 * f32_em(0x968990) as f64 + f32_em(0x96d750) as f64));
            if r == 0 && (e == 0 || nota < at16) {
                perdeu = true;
            }
        }
        if perdeu {
            crate::comparacao::ramo("movimento perde a bola");
            *((m + 0x4786) as *mut u32) = 0;
            *b(m, 0x8eae) = 0xff;
            *((m + 0xf582) as *mut u32) = 0;
            *b(m, 0x1a7) = 0;
            *w(p, 0x29) = 0;
            *w(p, 0x19c) = 0;
            let r = sortear(2);
            somar(p, -0x96);
            *b(p, 0x2b) = (r as u8).wrapping_add(1);
            narrar(chamar, 0x1f42, ub(p, 0x2e), ub(p, 0x27), u32::MAX, 0);
            if sortear((byte(u(p, 0x69), 0x5a) as i32 + 1) * 0x14) == 0 {
                let r = sortear(3);
                if ajustar_17(p, (r as u8).wrapping_neg() as i8) != 0 {
                    desempenho_na_partida(p, 0);
                }
            }
            break 'movimento;
        }
        if e != 0 && at16 <= nota {
            crate::comparacao::ramo("movimento desafiante vence");
            desafiante_vence_com(m, p, e, lance, nota, at16, vantagem, penalti, a, conduz, chamar);
            return;
        }
        // `p` passa
        crate::comparacao::ramo("movimento passa");
        if !na_area_atacada(lado, byte(p, 0x102), byte(p, 0x103)) && sortear(*w(p, 0x31) as i32) < -10 {
            crate::comparacao::ramo("movimento cresce");
            let v = f32_em(p + 0x1a2) as f64 * f32_em(0x96d3b8) as f64;
            *((p + 0x1a2) as *mut f32) = v as f32;
            let minimo = ((25 - byte(u(p, 0x69), 0x5d) as i32) * 2) as f32;
            let v = if v > minimo as f64 { v } else { minimo as f64 };
            *((p + 0x1a2) as *mut f32) = v as f32;
            if v > f64_em(0x9688a8) {
                *((p + 0x1a2) as *mut f32) = 100.0;
            }
            narrar(chamar, 0x211d, u32::MAX, outro as u32, u32::MAX, 0);
        }
        *b(m, 0x8eab) = lado as u8;
        if lado != -1 {
            let q = (m as isize + lado as isize + 0x8d46) as *mut u8;
            *q = (*q).wrapping_add(1);
            let o2 = b(m, 0x8d46 + outro as usize);
            if (*o2 as i8) > 0 {
                *o2 = (*o2).wrapping_sub(10);
            }
        }
        let q = (m as isize + lado as isize + 0x8d46) as *mut u8;
        *q = (*q).wrapping_add(5);
        let limite_nota = (nota as i32 + 0x32).max(0xfa);
        let v = (at16 as i32 - limite_nota) as f64 + (f32_em(p + 0xe5) as f64 * f32_em(p + 0x95) as f64);
        let delta = if (f32_em(0x968970) as f64) < v { 0 } else { ftol(v) as i16 };
        fixar_19c(p, (*w(p, 0x19c)).wrapping_add(delta));
        let casa = (m as isize + outro as isize + 0x21b6 + ((byte(p, 0x103) as isize) + byte(p, 0x102) as isize * 0xc) * 0x5a) as *const u8;
        if *casa == 0 {
            fixar_19c(p, (*w(p, 0x19c)).wrapping_add(0x32));
        }
        let soma = |k: u32| {
            *((m + 0x4782) as *mut u32) = (*((m + 0x4782) as *const u32)).wrapping_add(k);
            *((m + 0x4786) as *mut u32) = (*((m + 0x4786) as *const u32)).wrapping_add(k);
        };
        if e == 0 {
            somar(p, 9);
            soma(0x24);
        } else if codigo == 0xf {
            *b(e, 0x2b) = (*b(e, 0x2b)).wrapping_add(b2);
            somar(p, 0x4b);
            *b(p, 0x11) = (*b(p, 0x11)).wrapping_add(1);
            soma(300);
        } else {
            let k = if (b2 as i8) > 2 { b2 } else { 2 };
            *b(e, 0x2b) = (*b(e, 0x2b)).wrapping_add(k);
            if tabela1(e) {
                *b(m, 0x1a4) = 1;
                soma(200);
            } else {
                soma(400);
            }
            somar(p, 100);
            *b(p, 0x11) = (*b(p, 0x11)).wrapping_add(1);
            if sortear((byte(p, 0x17) as i32 + 1) * 0x14) == 0 {
                let r = sortear(5);
                if ajustar_17(p, r as i8) != 0 {
                    desempenho_na_partida(p, 0);
                }
            }
        }
        *b(m, 0x8ea9) = 0;
        narrar_a_acao_com(m, p, e, codigo, nx, ny, conduz, modo, chamar);
        let _ = &mut conduz;
    }
    andar_com(m, p, ox, oy, nx, ny, perdeu, chamar);
}

/// As narracoes da acao de quem passa com a bola (0xf: 0x1f48/0x1f49; 0x11 e 0x12: pela area, pelo
/// meio, pelas pontas, pelo marcado e pelo goleiro).
#[allow(clippy::too_many_arguments)]
unsafe fn narrar_a_acao_com(m: usize, p: usize, e: usize, codigo: i16, x: i8, y: i8, conduz_in: u8, modo: u8, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let u = |q: usize, o: usize| *((q + o) as *const usize);
    let ub = |q: usize, o: usize| *((q + o) as *const u8) as u32;
    let lado = byte(p, 0x27);
    let outro = (lado == 0) as u8;
    let par = |q: usize| ub(q, 0x2e) | ub(q, 0x27) << 8;
    let narrar = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, a5: u32| {
        chamar(0x6d6f70, u(m, 0x8f7a), [codigo, m as u32, ub(p, 0x2e), lado as u8 as u32, a5]);
    };
    let tabela1 = |q: usize| u(q, 0x19e) != 0 && tabela_do_jogador(q) == 1;
    let (px, py) = (byte(p, 0x102), byte(p, 0x103));
    let mut conduz = conduz_in;
    let defesa_400 = || *((m + 0x9766 + outro as usize * 0x18e3) as *const u32) & 0x400 != 0;
    // a narracao com o desafiante (0x2173 pelo +0x71, 0x1f4a; com o goleiro 0x1f55/0x2158)
    let com_desafiante = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32| {
        let cod = if !tabela1(e) {
            let v = f32_em(p + 0x71);
            let forte = v > f32_em(0x96b020) || (v as f64 - f32_em(e + 0x71) as f64) > f32_em(0x968a78) as f64;
            if forte && sortear(ftol(f32_em(p + 0x95) as f64)) < sortear(0x1e) { 0x2173 } else { 0x1f4a }
        } else if na_area_atacada(lado, px, py) {
            0x1f55
        } else {
            0x2158
        };
        narrar(chamar, cod, par(e));
    };
    // a arvore sem desafiante, a partir do alvo
    let sem_desafiante = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32| {
        if na_area_atacada(lado, x, y) && !na_area_atacada(lado, px, py) {
            narrar(chamar, 0x1f50, u32::MAX);
            return;
        }
        if fora_do_meio(x) != 0 && ((lado == 0 && y == 0xb && py < 0xb) || (lado == 1 && y == 0 && py > 0)) {
            narrar(chamar, 0x2156, u32::MAX);
            return;
        }
        if (lado == 0 && y == 6 && py < 6) || (lado == 1 && y == 5 && py > 5) {
            narrar(chamar, 0x1f51, u32::MAX);
            return;
        }
        if (lado == 0 && py < 6 && py > y) || (lado == 1 && py >= 6 && y > py) {
            narrar(chamar, 0x1f54, u32::MAX);
            return;
        }
        if ((lado == 1 && py < 3) || (lado == 0 && py > 8)) && (px < 2 || px > 6) && fora_do_meio(x) == 0 {
            narrar(chamar, 0x1f4e, u32::MAX);
            return;
        }
        if fora_do_meio(x) != 0 && ((lado == 0 && x < px && x < 2) || (lado == 1 && px < x && x > 6)) {
            narrar(chamar, 0x1f52, u32::MAX);
            return;
        }
        if fora_do_meio(x) != 0 && ((lado == 0 && px < x && x > 6) || (lado == 1 && x < px && x < 2)) {
            narrar(chamar, 0x1f53, u32::MAX);
            return;
        }
        let q = u(p, 0x1ae);
        if q != 0 && byte(q, 0x102) == px && byte(q, 0x103) == py {
            let c = byte(q, 0x10a) as i32;
            if sortear(3) < c && byte(q, 0x2b) < 2 {
                crate::comparacao::ramo("movimento narra o marcado");
                narrar(chamar, 0x2172, ub(q, 0x2e) | (outro as u32) << 8);
                return;
            }
        }
        let acao = *((p + 0x198) as *const i16);
        if acao == 0x12 || acao == 0x11 {
            narrar(chamar, 0x1f4c, u32::MAX);
        } else if na_area_atacada(lado, px, py) {
            narrar(chamar, 0x20d1, u32::MAX);
        } else if limite(outro, py) != 0 {
            narrar(chamar, 0x20d3, u32::MAX);
        } else {
            narrar(chamar, 0x20d2, u32::MAX);
        }
    };
    match codigo {
        0xf => {
            if e == 0 {
                narrar(chamar, 0x1f48, u32::MAX);
            } else {
                narrar(chamar, 0x1f49, par(e));
            }
        }
        0x11 => {
            if na_area_atacada(lado, px, py) || (lado == 1 && py > 5) || (lado == 0 && py < 6) {
                conduz = 0;
            }
            if modo == 4 {
                crate::comparacao::ramo("movimento 0x11 modo 4");
                narrar(chamar, if *((p + 0x198) as *const i16) != 0xf { 0x2009 } else { 0x20d2 }, u32::MAX);
            } else if e == 0 {
                if conduz == 0 || !(menor_distancia(m, outro as i8, px, py, 1) as f64 > f64_em(0x968940)) {
                    sem_desafiante(chamar);
                } else {
                    narrar(chamar, if defesa_400() { 0x20de } else { 0x1f45 }, u32::MAX);
                }
            } else {
                com_desafiante(chamar);
                if conduz != 0 && menor_distancia(m, outro as i8, px, py, 1) as f64 > f64_em(0x968940) {
                    narrar(chamar, if defesa_400() { 0x20de } else { 0x1f45 }, u32::MAX);
                }
            }
        }
        0x12 => {
            if na_area_atacada(lado, px, py) || modo == 4 {
                conduz = 0;
            }
            if e != 0 {
                com_desafiante(chamar);
                if conduz != 0
                    && ((lado == 1 && py < 3) || (lado == 0 && py > 8))
                    && menor_distancia(m, outro as i8, px, py, 1) as f64 > f64_em(0x968940)
                {
                    if defesa_400() && sortear(2) == 0 {
                        narrar(chamar, 0x20de, u32::MAX);
                    } else {
                        narrar(chamar, 0x1f45, u32::MAX);
                    }
                }
            } else {
                sem_desafiante(chamar);
            }
        }
        _ => {
            // a original mostra a caixa de erro ("unknown action") e zera 0xb67a34
            *(0xb67a34 as *mut u32) = 0;
        }
    }
}

/// O desafiante `e` vence a disputa (como na dividida, com outras narracoes e o desgaste de `p`).
#[allow(clippy::too_many_arguments)]
unsafe fn desafiante_vence_com(m: usize, p: usize, e: usize, lance: i8, nota: i16, at: i16, vantagem: u8, penalti: bool, a: u8, conduz: u8, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
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
    let narrou = (codigo_especial(lance) != 0 || vantagem != 0 || sortear(*w(e, 0x31) as i32) < -10)
        && na_area_atacada((byte(e, 0x27) == 0) as i8, byte(e, 0x102), byte(e, 0x103))
        && sortear(2) == 0;
    if narrou {
        crate::comparacao::ramo("movimento narra falta na area");
        narrar(chamar, narracao_da_falta(lance), ub(e, 0x2e), ub(e, 0x27), par(p));
        let codigo = match sortear(0x32) {
            1 => 0x20ea,
            0 | 2 | 4 | 6 | 8 | 10 => 0x20eb,
            _ => 0x20e8,
        };
        narrar(chamar, codigo, u32::MAX, zero(e), u32::MAX);
    } else {
        let codigo = match lance {
            1 => {
                crate::comparacao::ramo("movimento defesa");
                if defender_com(e, nota.wrapping_sub(at), &mut |rotina, this, x| {
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
            _ => 0,
        };
        if codigo != 0 {
            narrar(chamar, codigo, ub(e, 0x2e), ub(e, 0x27), par(p));
        }
    }
    chamar(0x6f0630, e, [0; 5]);
    // `p` perde: mais na propria area
    if na_area_atacada((lado == 0) as i8, byte(p, 0x102), byte(p, 0x103)) {
        crate::comparacao::ramo("movimento perde na area");
        *w(p, 0x35) = (*w(p, 0x35)).wrapping_sub(0xfa);
        if sortear(byte(u(p, 0x69), 0x5a) as i32 * 0x1e + 0x14) == 0 {
            let r = sortear(3);
            if ajustar_17(p, (r as u8).wrapping_neg() as i8) != 0 {
                desempenho_na_partida(p, 0);
            }
        }
    } else if (lado == 0 && byte(p, 0x103) < 3) || (lado == 1 && byte(p, 0x103) > 8) {
        *w(p, 0x35) = (*w(p, 0x35)).wrapping_sub(100);
    } else {
        *w(p, 0x35) = (*w(p, 0x35)).wrapping_sub(0x14);
    }
    *w(e, 0x19c) = nota.wrapping_sub(at).wrapping_sub(200).clamp(-200, 200);
    let mut dominou = false;
    let limite = match lance {
        0x13 => 400,
        0x21 | 0x2d => 0xfa,
        _ => 100,
    };
    if *w(e, 0x19c) < limite && byte(m, 0x8ea9) != 4 && byte(m, 0x8ea9) != -1 {
        crate::comparacao::ramo("movimento bola espirra");
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
    } else {
        if (0..4).contains(&byte(m, 0x8ea9)) {
            *b(m, 0x8ea9) = 0;
        }
        let (x, y) = (byte(p, 0x102), byte(p, 0x103));
        if byte(e, 0x27) == 0 && byte(m, 0x8ea9) != -1 {
            apontar(e, x, y, 4, if y < 9 { y + 3 } else { 11 });
        } else {
            apontar(e, x, y, 4, if y < 4 { 0 } else { y - 3 });
        }
        let minuto = (*w(m, 0x8ed0) as i32 / 11) as i8;
        let (contas, sorteio) = if lance == 0x15 { ((7, 8), 0x37) } else { ((4, 5), 0x41) };
        let media = if byte(e, contas.0) != 0 { (ub(e, contas.1) as f64 / ub(e, contas.0) as f64) as f32 } else { 0.0 };
        if minuto > (sortear(0x14) + sorteio) as u8 as i8 {
            let gols = byte(m, if byte(e, 0x27) != 0 { 0xf5bd } else { 0xf5f3 });
            let tentativas = if lance == 0x15 { ub(e, 8) > 3 && byte(e, 9) != 0 } else { ub(e, 6) > 1 };
            if gols == 0 && media as f64 > f64_em(0x96a6a0) && tentativas && *w(e, 0x35) > 0x1a5e && byte(e, 0xfd) == 0 {
                crate::comparacao::ramo("movimento media de acertos");
                narrar(chamar, if lance == 0x15 { 0x21ce } else { 0x21cd }, ub(e, 0x2e), ub(e, 0x27), par(p));
                *b(e, 0xfd) = 1;
            }
        }
    }
    *b(e, 5) = (*b(e, 5)).wrapping_add(1);
    let mm = u(e, 0x19e);
    let t = |_: ()| if mm != 0 { tabela_do_jogador(e) } else { 0 };
    let longe = t(()) & 4 == 0 && t(()) & 2 == 0;
    let mut ganho: i16 = if dominou { 0x78 } else { 0xc8 };
    if longe {
        ganho <<= 1;
    }
    let py = byte(p, 0x103);
    let na_linha = match lado {
        0 | 1 => py < 3 || py > 8,
        _ => false,
    };
    if !na_linha {
        ganho = ftol(ganho as f64 * f64_em(0x969520)) as i16;
    }
    *w(e, 0x35) = (*w(e, 0x35)).wrapping_add(ganho);
    let alvo = if byte(e, 0x27) != 0 { 0xb } else { 0 };
    let i = ((byte(e, 0x103) as i32 - alvo).abs() as i8 as i32) * 9 + (byte(e, 0x102) as i32 - 4).abs() as i8 as i32;
    let perto = !(f32_em(0xa15e44 + i as usize * 4) as f64 > f64_em(0x967890));
    if conduz != 0 || perto || atras_do_extremo(mm, e) != 0 {
        crate::comparacao::ramo("movimento desafiante bonus");
        *w(e, 0x35) = (*w(e, 0x35)).wrapping_add(300);
        *b(e, 6) = (*b(e, 6)).wrapping_add(1);
        if sortear(byte(e, 0x17) as i32 * 0x1e + 0x32) == 0 {
            let r = sortear(5);
            if ajustar_17(e, r as i8) != 0 {
                desempenho_na_partida(e, 0);
            }
        }
    }
    *b(p, 0x2b) = (*b(p, 0x2b)).wrapping_add(a);
    if dominou && sortear(7) == 0 {
        narrar(chamar, 0x1f41, ub(e, 0x2e), ub(e, 0x27), u32::MAX);
    }
}

/// A falta de `e` sobre `p` punida no movimento (como `punir_desafiante_com`, com o evento 0x82 e a
/// marca da bola parada no fim).
#[allow(clippy::too_many_arguments)]
unsafe fn punir_no_movimento_com(m: usize, e: usize, p: usize, mut tipo: u8, cartao: u8, grav: i8, arriscado: i8, vantagem: u8, lance: i8, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
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
    let som: u32;
    if tipo == 7 {
        som = 0x1f79;
        *b(m, 0x8eb2) = 7;
    } else if na_area_atacada(lado, byte(p, 0x102), byte(p, 0x103)) {
        crate::comparacao::ramo("movimento penalti");
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
        if sortear(byte(u(e, 0x69), 0x5a) as i32 + 6) == 0 {
            let r = sortear(8);
            if ajustar_17(e, (r as u8).wrapping_neg() as i8) != 0 {
                desempenho_na_partida(e, 0);
            }
        }
        narrar(chamar, 0x1f7a, u32::MAX, ub(p, 0x27), u32::MAX);
    } else {
        som = 0x1f79;
        *b(m, 0x8eb2) = 6;
    }
    let expulso = matches!(tipo, 3..=6);
    if cor as i16 > -1 {
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
                crate::comparacao::ramo("movimento evento 0x82");
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
            crate::comparacao::ramo("movimento expulsao");
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
                } else if (2..=6).contains(&byte(p, 0x102)) {
                    narrar(chamar, 0x21e0, ub(c, 0x2e), ub(p, 0x27), u32::MAX);
                } else {
                    narrar(chamar, 0x21df, ub(c, 0x2e), ub(p, 0x27), u32::MAX);
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
    *b(m, 0x8ea7) = *b(p, 0x102);
    *b(m, 0x8ea8) = *b(p, 0x103);
    *b(m, 0x8eae) = *b(p, 0x27);
    *((m + 0xf582) as *mut u32) = 0;
    *w(e, 0x35) = (*w(e, 0x35)).wrapping_sub(0x19);
}

/// O passo na grade e o que vem depois: vagas da casa (+0x215e, 0x5a bytes por casa, 11 vagas por
/// lado e a conta em +0x58), o mais adiantado, os marcados, o desgaste, a bola e a direcao.
#[allow(clippy::too_many_arguments)]
unsafe fn andar_com(m: usize, p: usize, ox: i8, oy: i8, nx: i8, ny: i8, perdeu: bool, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let b = |q: usize, o: usize| (q + o) as *mut u8;
    let w = |q: usize, o: usize| (q + o) as *mut i16;
    let ub = |q: usize, o: usize| *((q + o) as *const u8) as u32;
    let u = |q: usize, o: usize| *((q + o) as *const usize);
    let narrar = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, codigo: u32, a3: u32, a4: u32| {
        chamar(0x6d6f70, u(m, 0x8f7a), [codigo, m as u32, a3, a4, u32::MAX]);
    };
    let lado = byte(p, 0x27);
    let casa = |x: i8, y: i8| (m as isize + 0x215e + (y as isize + x as isize * 0xc) * 0x5a) as usize;
    let vaga = |c: usize, k: i8| (c as isize + (lado as isize * 0xb + k as isize) * 4) as *mut usize;
    let conta = |c: usize| (c as isize + lado as isize + 0x58) as *mut i8;
    if byte(p, 0x102) != nx || byte(p, 0x103) != ny {
        let nova = casa(nx, ny);
        if byte(p, 0x28) < 0 {
            *b(p, 0x28) = *conta(nova) as u8;
            *conta(nova) = (*conta(nova)).wrapping_add(1);
            *vaga(nova, byte(p, 0x28)) = p;
        } else {
            let velha = casa(byte(p, 0x102), byte(p, 0x103));
            *vaga(velha, byte(p, 0x28)) = *vaga(velha, (*conta(velha)).wrapping_sub(1));
            *conta(velha) = (*conta(velha)).wrapping_sub(1);
            let q = *vaga(velha, byte(p, 0x28));
            if q != 0 {
                *b(q, 0x28) = *b(p, 0x28);
            }
            *b(p, 0x28) = *conta(nova) as u8;
            *conta(nova) = (*conta(nova)).wrapping_add(1);
            *vaga(nova, byte(p, 0x28)) = p;
        }
    }
    *b(p, 0x102) = nx as u8;
    *b(p, 0x103) = ny as u8;
    if lado == 0 {
        let q = u(m, 0xf596);
        let qy = if q == 0 { -1 } else { byte(q, 0x103) };
        if ny > qy {
            *((m + 0xf596) as *mut usize) = p;
        }
    } else if lado == 1 {
        let q = u(m, 0xf59a);
        let qy = if q == 0 { 9 } else { byte(q, 0x103) };
        if ny < qy {
            *((m + 0xf59a) as *mut usize) = p;
        }
    }
    soltar_marcados(m, p, chamar);
    if byte(p, 0x19) >= 0 {
        let mut k: f32 = match byte(m, 1) {
            1 => 1.4,
            2 => 1.2,
            4 | 0x10 => 1.05,
            8 => 1.45,
            0x20 => 1.15,
            0x40 => 1.1,
            _ => 1.0,
        };
        match byte(m, 0x4756) {
            3 => k = (k as f64 * f32_em(0x96d3b8) as f64) as f32,
            4 => k = (k as f64 * f32_em(0x96a33c) as f64) as f32,
            _ => {}
        }
        let dy = (byte(p, 0x103) as i32 - oy as i32).abs() as i8 as i32;
        let dx = (byte(p, 0x102) as i32 - ox as i32).abs() as i8 as i32;
        let t = f32_em(0xa15e44 + (dy * 9 + dx) as usize * 4) as f64;
        let d5 = f32_em(p + 0xd5) as f64;
        let f = (f64_em(0x9688a0) - (d5 * f64_em(0x968968) + f64_em(0x968e08)) * f64_em(0x96a7a8)) + f64_em(0x968950);
        k = (t * f * k as f64) as f32;
        let papel_b = if m != 0 { tabela_b(m, p) } else { 0 };
        let papel = if m != 0 { tabela_do_jogador(p) } else { 0 };
        if papel_b != papel {
            let classe = |t: u16| -> i8 {
                if t & 1 != 0 {
                    0
                } else if t & 4 != 0 {
                    1
                } else if t & 8 != 0 {
                    2
                } else if t & 0x10 != 0 {
                    3
                } else if t & 0x20 != 0 {
                    4
                } else if t & 0x40 != 0 {
                    5
                } else {
                    3
                }
            };
            let a = classe(if m != 0 { tabela_do_jogador(p) } else { 0 });
            let c = classe(if m != 0 { tabela_b(m, p) } else { 0 });
            let kk = if (a as i32 - c as i32).abs() as u8 as i8 > 1 {
                0x968928
            } else if (if m != 0 { tabela_do_jogador(p) } else { 0 }) & 4 == 0 && (if m != 0 { tabela_b(m, p) } else { 0 }) & 4 == 0 {
                0x968978
            } else {
                0x968980
            };
            k = ((f64_em(kk) - d5 * f64_em(0x968ef0)) * k as f64) as f32;
        }
        let tatica = m + 0x91e2 + lado as usize * 0x18e3;
        let marcas = marcas_da_posicao(tatica, byte(p, 0x19));
        if marcas & 0x8000 != 0 || (marcas_da_posicao(tatica, byte(p, 0x19)) & 0x2000 != 0 && *((m + lado as usize * 0x18e3 + 0x9766) as *const u32) & 0x100 != 0) {
            k = ((f64_em(0x96d748) - d5 * f64_em(0x967878)) * k as f64) as f32;
        }
        desgastar(p, k, -1);
    }
    if p == u(m, 0xf582) || perdeu {
        *b(m, 0x8ea7) = *b(p, 0x102);
        *b(m, 0x8ea8) = *b(p, 0x103);
        let (bx, by) = (byte(m, 0x8ea7), byte(m, 0x8ea8));
        if !(bx >= 0 && by >= 0 && by < 0xc && bx < 9) {
            crate::comparacao::ramo("movimento bola fora");
            chamar(0x6cd490, m, [lado as u8 as u32, 0, 0, 0, 0]);
        }
    } else if byte(m, 0x8ea9) < 5
        && byte(m, 0x1c5) == 0
        && byte(m, 0x8eae) == -1
        && byte(m, 0x8ea7) == byte(p, 0x102)
        && byte(m, 0x8ea8) == byte(p, 0x103)
        && p != u(m, 0xf57a)
        && p != u(m, 0xf576)
    {
        let q = u(m, 0xf57a);
        if q != 0 {
            if byte(q, 0x27) != lado {
                crate::comparacao::ramo("movimento rouba a bola livre");
                narrar(chamar, 0x1f60, ub(p, 0x2e), lado as u8 as u32);
                chamar(0x6f0630, p, [0; 5]);
                *w(p, 0x35) = (*w(p, 0x35)).wrapping_add(0x1c);
            }
        } else {
            crate::comparacao::ramo("movimento domina a bola livre");
            apontar(p, byte(p, 0x102), byte(p, 0x103), 4, if lado != 0 { 0 } else { 0xb });
            let (px, py) = (byte(p, 0x102), byte(p, 0x103));
            let gol = byte(m, 0x1cd) != 0 && (3..=5).contains(&px) && ((lado == 0 && py >= 10) || (lado == 1 && py <= 1));
            narrar(chamar, if gol { 0x2113 } else { 0x1f46 }, ub(p, 0x2e), lado as u8 as u32);
            *b(m, 0x8ea9) = 0;
            chamar(0x6f0630, p, [0; 5]);
            *w(p, 0x35) = (*w(p, 0x35)).wrapping_add(4);
        }
    }
    // vira para um terco do caminho ate a bola
    if p == u(m, 0xf582) {
        return;
    }
    let bola_y = byte(m, 0x8ea8) as i32;
    let q = u(m, 0xf57a);
    let dono = byte(m, 0x8eae);
    let perto: bool = if lado == 0 {
        if dono == 1 {
            true
        } else if p == q {
            false
        } else if dono != 0 && (q == 0 || byte(q, 0x27) != 0) {
            byte(p, 0x103) < 6
        } else {
            false
        }
    } else if dono == 0 {
        false
    } else {
        !(p != q && dono != lado && (q == 0 || byte(q, 0x27) != lado) && lado == 1 && byte(p, 0x103) >= 6)
    };
    let dy = if perto { bola_y + ny as i32 } else { bola_y + ny as i32 + 0xb };
    let dx = byte(m, 0x8ea7) as i32 + 4 + nx as i32;
    apontar(p, byte(p, 0x102), byte(p, 0x103), (dx / 3) as i8, (dy / 3) as i8);
}

/// Despacho real das chamadas injetadas do movimento: as da dividida, o marcado que sai (0x714d40)
/// e o evento 0x82 (0x68d0d0, o bloco pedido por ultimo).
unsafe fn despachar_movimento(rotina: usize, this: usize, a: [u32; 5]) -> u32 {
    match rotina {
        0x714d40 => {
            let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x714d40usize);
            f(this);
            0
        }
        0x68d0d0 => despachar_lesao(rotina, this, a),
        _ => despachar_dividida(rotina, this, a),
    }
}

/// { 0x6f09f0 }: ver `mover_com`.
unsafe extern "thiscall" fn mover_jogador(p: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> u32 {
    mover_com(p, a1 as u8 as i8, a2 as u8 as i8, a3 as u16 as i16, a4 as u8, a5 as u8, &mut |r, t, a| despachar_movimento(r, t, a));
    0
}

unsafe extern "thiscall" fn comparar_mover(p: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> u32 {
    if gravando() {
        // unidade da fita de fora; por dentro, a propria comparacao (fita aninhada)
        return chamar_gravando(0x6f09f0, p, [a1, a2, a3, a4, a5], || fita_do_movimento(p, a1, a2, a3, a4, a5));
    }
    fita_do_movimento(p, a1, a2, a3, a4, a5)
}

unsafe fn fita_do_movimento(p: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> u32 {
    use std::sync::atomic::Ordering::Relaxed;
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> u32>(318);
    let m = *((p + 0x19e) as *const usize);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(m, MOTOR), (0xb67a34, 4), (0xae1080, 4)]);
    let t = *((m + 0x4792) as *const usize);
    for clube in [*((t + 0x1c) as *const usize), *((t + 0x20) as *const usize)] {
        reg.extend(linhas_do_clube(clube));
    }
    let gravar = GRAVAR_716.swap(true, Relaxed);
    let a7 = GRAVAR_A7.swap(true, Relaxed);
    let mov = GRAVAR_MOVIMENTO.swap(true, Relaxed);
    let (r, _, igual, sequencia, k) = comparar_com_fita(reg, || o(p, a1, a2, a3, a4, a5), |chamar| {
        mover_com(p, a1 as u8 as i8, a2 as u8 as i8, a3 as u16 as i16, a4 as u8, a5 as u8, &mut |rotina, this, args| {
            let mascara = match rotina {
                0x6d6f70 => [0xff_ffff, u32::MAX, 0xff, 0xff, 0xff],
                0x6cd770 => [0xff, 0xff, 0xffff, 0xff, u32::MAX],
                0x6cd490 => [0xff, 0, 0, 0, 0],
                0x62ed70 => [u32::MAX, 0, 0, 0, 0],
                0x6d0980 => [0xff, 0xff, u32::MAX, u32::MAX, 0],
                0x897760 => [u32::MAX, 0xff, u32::MAX, 0, 0],
                0x68d0d0 => [0, u32::MAX, 0, 0, 0],
                0x6e1aa0 | 0x716c90 | 0x6f0630 | 0x714d40 => [0, 0, 0, 0, 0],
                _ => [0xff, 0, 0, 0, 0],
            };
            chamar(rotina, this, args, mascara)
        });
        0
    });
    GRAVAR_MOVIMENTO.store(mov, Relaxed);
    GRAVAR_A7.store(a7, Relaxed);
    GRAVAR_716.store(gravar, Relaxed);
    PLACAR.registrar(igual && sequencia, || format!("mover(acao {:#x}) {} chamadas: {}", a3 as u16, k, detalhe_fita()));
    r
}

unsafe fn fita_do_chute_a_gol(p: usize, f: f32) {
    use std::sync::atomic::Ordering::Relaxed;
    let o = orig::<unsafe extern "thiscall" fn(usize, f32)>(317);
    let m = *((p + 0x19e) as *const usize);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(m, MOTOR), (0xb67a34, 4), (0xae1080, 4)]);
    let t = *((m + 0x4792) as *const usize);
    for clube in [*((t + 0x1c) as *const usize), *((t + 0x20) as *const usize)] {
        reg.extend(linhas_do_clube(clube));
    }
    let gravar = GRAVAR_716.swap(true, Relaxed);
    let a7 = GRAVAR_A7.swap(true, Relaxed);
    let (_, _, igual, sequencia, k) = comparar_com_fita(reg, || o(p, f), |chamar| {
        chute_a_gol_com(p, f, &mut |rotina, this, args| {
            let mascara = match rotina {
                0x6d6f70 => [0xff_ffff, u32::MAX, 0xff, 0xff, 0xff],
                0x6cd770 => [0xff, 0xff, 0xffff, 0xff, u32::MAX],
                0x62ed70 => [u32::MAX, 0, 0, 0, 0],
                0x6d0980 => [0xff, 0xff, u32::MAX, u32::MAX, 0],
                0x897760 => [u32::MAX, 0xff, u32::MAX, 0, 0],
                0x6e1aa0 | 0x716c90 | 0x6f0630 => [0, 0, 0, 0, 0],
                _ => [0xff, 0, 0, 0, 0],
            };
            chamar(rotina, this, args, mascara)
        })
    });
    GRAVAR_A7.store(a7, Relaxed);
    GRAVAR_716.store(gravar, Relaxed);
    PLACAR.registrar(igual && sequencia, || format!("chute_a_gol {} chamadas: {}", k, detalhe_fita()));
}
