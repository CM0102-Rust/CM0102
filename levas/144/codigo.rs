// ---------- centesima quadragesima quarta leva: o jogador com a bola (0x7109f0) ----------

/// Unidades da fita da conducao: a copia de 0x68d0d0 vai para a lista na pilha (endereco diferente
/// na copia Rust: grava sem o `this`) e a 0x702050 (o chute) entra como unidade.
static GRAVAR_CONDUCAO: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Na metade de ataque do lado (lado 1: linhas abaixo de 6; lado 0: de 6 em diante; outro lado, sim).
fn na_metade_de_ataque(lado: i8, y: i8) -> bool {
    !((lado == 1 && y >= 6) || (lado == 0 && y < 6))
}

/// Na metade de defesa do lado (o contrario da de ataque, so para os lados 0 e 1).
fn na_metade_de_defesa(lado: i8, y: i8) -> bool {
    (lado == 1 && y >= 6) || (lado == 0 && y < 6)
}

/// Na propria area (a que o lado defende; lado 1: linhas 10-11, os outros 0-1).
fn na_area_defendida(lado: i8, x: i8, y: i8) -> bool {
    na_area_atacada((lado != 1) as i8, x, y)
}

/// { 0x7109f0 } (match_pl.cpp, ~13 KB) com as chamadas que mexem fora do motor injetadas (0x6fa250
/// o passe, 0x6fa490 o lancamento, 0x70ec80 a dividida, 0x6f09f0 o movimento, 0x702050 o chute,
/// 0x6d6f70 narra, 0x68d0d0 a copia na lista da pressao): a decisao do jogador `p` com a bola.
/// Bola parada (+0x8eb2): 2 e 3 (escanteios, 0x36/0x34), 4 (lateral, 0x5a), 6 e 7 (faltas, 0x37) passam
/// para o companheiro escolhido (0x6c6a30) ou para uma casa sorteada; 5 (penalti, 0x33) e 6 perto do
/// gol (0x35) chutam (0x700bd0 + 0x702050). Em jogo, soma as notas do chute (+0x10), do passe
/// (+0x14) e da conducao (+0x44) pela distancia ao gol, a pressao (0x6bcbd0), o placar, o goleiro,
/// as marcas da posicao e o relogio; pode lancar (0x6fa490), dividir (0x70ec80), chutar (0x702050),
/// passar (0x6fa250) ou conduzir (0x6f09f0) para a casa escolhida (0x6bce80). Modo 4: o goleiro na
/// disputa de penaltis. Devolve 1 so quando conduz.
#[allow(clippy::cognitive_complexity)]
unsafe fn conduzir_com(p: usize, modo: u8, chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) -> u8 {
    let u = |q: usize, o: usize| *((q + o) as *const usize);
    let b = |q: usize, o: usize| (q + o) as *mut u8;
    let w = |q: usize, o: usize| (q + o) as *mut i16;
    let ub = |q: usize, o: usize| *((q + o) as *const u8) as u32;
    let fl = |o: usize| f32_em(p + o) as f64;
    let mut volta = 0u8;
    let m = u(p, 0x19e);
    let lado = byte(p, 0x27);
    let x0 = || byte(p, 0x102);
    let y0 = || byte(p, 0x103);
    let f: f32 = if lado == 0 { tabela_12x9(y0() as i32 - 0xb, x0() as i32 - 4) } else { distancia_ao_ponto(p, 4, 0) };
    let ff = f as f64;
    let (mut l14, mut l44) = (0i32, 0i32);
    let mut l10: i32;
    let mut l90 = 0f32;
    let mut l70 = 0i16;
    let mut l88: i8 = -1;
    let mut l64 = 0u16;
    let (mut l5f, mut l37) = (false, false);
    let mut la4 = 0i32;
    // a lista da pressao (0x68d020: contador e cabeca/cauda); so a copia do modo Rust a enche
    let mut lista = [0u8; 14];
    let la = lista.as_mut_ptr() as usize;
    let passar = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, tipo: u32, r: usize, moveu: u8, x: i8, y: i8| {
        *w(p, 0x198) = tipo as i16;
        chamar(0x6fa250, p, [tipo, r as u32, moveu as u32, x as u8 as u32, y as u8 as u32]);
    };
    let chutar = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32, tipo: u32, v: i32, som: u16, a4: u32, com_lista: bool| {
        chamar(0x702050, p, [tipo, v as u32, som as u32, a4, if com_lista { la as u32 } else { 0 }]);
    };
    'fim: {
        let c = byte(m, 0x8eb2);
        let (mut tipo, mut moveu, mut x, mut y, mut melhor) = (0i8, 0u8, 0i8, 0i8, 0i32);
        let escolher = |pedido: i8, tipo: &mut i8, moveu: &mut u8, x: &mut i8, y: &mut i8, melhor: &mut i32| {
            escolher_companheiro(m, byte(m, 0x8ea7), byte(m, 0x8ea8), tipo, moveu, x, y, melhor, pedido)
        };
        match c {
            2 => {
                *b(m, 0x8eb2) = 0;
                let r = escolher(0x36, &mut tipo, &mut moveu, &mut x, &mut y, &mut melhor);
                if r == 0 {
                    crate::comparacao::ramo("conducao escanteio sem companheiro");
                    let d: i8 = if lado != 1 { 1 } else { -1 };
                    y = byte(m, 0x8ea8).wrapping_add(d);
                    if y < 0 {
                        y = 0;
                    } else if y > 0xb {
                        y = 0xb;
                    }
                    x = if byte(m, 0x8ea7) == 0 { sortear(4) as i8 } else { 8 - sortear(4) as i8 };
                }
                passar(chamar, 0x36, r, moveu, x, y);
                break 'fim;
            }
            3 => {
                *b(m, 0x8eb2) = 0;
                let r = escolher(0x34, &mut tipo, &mut moveu, &mut x, &mut y, &mut melhor);
                if r == 0 {
                    x = 5 - sortear(3) as i8;
                    if lado == 1 {
                        y = 0;
                    } else if lado == 0 {
                        y = 0xb;
                    }
                }
                passar(chamar, 0x34, r, moveu, x, y);
                break 'fim;
            }
            4 => {
                *b(m, 0x8eb2) = 0;
                let mut r = escolher(0x5a, &mut tipo, &mut moveu, &mut x, &mut y, &mut melhor);
                if r == 0 || melhor < 2000 {
                    crate::comparacao::ramo("conducao lateral sorteada");
                    x = 7 - sortear(7) as i8;
                    y = if lado == 0 { sortear(3) as i8 + 6 } else { 5 - sortear(3) as i8 };
                    r = 0;
                }
                passar(chamar, 0x5a, r, moveu, x, y);
                break 'fim;
            }
            5 => {
                *b(m, 0x8eb2) = 0;
                crate::comparacao::ramo("conducao penalti");
                let mut t: i8 = 0x33;
                let v = valor_do_chute(p, &mut t, &mut l64) as i16 as i32;
                let a4 = if sortear(100) == 0 { 5 } else { 1 };
                chutar(chamar, 0x33, v, l64, a4, false);
                break 'fim;
            }
            6 | 7 => {
                *b(m, 0x8eb2) = 0;
                if c == 6 && (2..=6).contains(&x0()) && ff < f64_em(0x96d3e8) {
                    let bl = byte(p, 0x109) as i32;
                    let n = (sortear(ftol(fl(0xa1) + f32_em(0x96afb0) as f64)) + bl) / 3;
                    if (n as f64) > ff {
                        crate::comparacao::ramo("conducao falta direta");
                        *b(m, 0x8eb1) = 6;
                        let mut t: i8 = 0x35;
                        let v = valor_do_chute(p, &mut t, &mut l64) as i16 as i32;
                        let a4 = if sortear(100) == 0 { 6 } else { 1 };
                        chutar(chamar, 0x35, v, l64, a4, false);
                        break 'fim;
                    }
                }
                let mut r = escolher(0x37, &mut tipo, &mut moveu, &mut x, &mut y, &mut melhor);
                if r == 0 || melhor < 2000 {
                    crate::comparacao::ramo("conducao falta na area");
                    x = 4;
                    y = if lado == 0 { byte(m, 0x8eaf).wrapping_sub(2) } else { byte(m, 0x8eb0).wrapping_add(2) };
                    r = 0;
                }
                passar(chamar, 0x37, r, moveu, x, y);
                break 'fim;
            }
            8 => *b(m, 0x8eb2) = 0,
            _ => {}
        }
        // em jogo: o alvo (casa de 0x6c2be0) e as notas
        let (l74, l36): (i8, i8);
        if modo == 4 {
            l74 = 4;
            l36 = if lado != 0 { 0 } else { 0xb };
        } else {
            let (mut sx, mut sy) = (0i8, 0i8);
            posicionar(m, p, &mut sx, &mut sy, 1);
            l74 = nota_ajustada(p, sx) as i8;
            l36 = sy;
            if (lado == 0 && y0() > l36) || (lado == 1 && y0() < l36) {
                l90 = tabela_12x9(y0() as i32 - l36 as i32, x0() as i32 - l74 as i32);
            }
        }
        let l53 = alem_da_linha(m, y0(), lado, 0);
        let mut l27: i8 = 0;
        let l68 = mais_perto_do_gol(m, p);
        let mut l60 = menor_distancia(m, (lado == 0) as i8, x0(), y0(), 1);
        let l2f = pressao_no_ponto(m, (lado == 0) as i8, x0(), y0(), 1);
        let mut l49: i8 = 0;
        let mut l4c = false;
        let g = manter_posicao_um(m, (lado == 0) as i8);
        let (mut casa, mut fora, mut cl, mut ida, mut f3) = (0i8, 0i8, 0u8, 0u8, 0u8);
        placar_agregado(m, lado, &mut casa, &mut fora, &mut cl, &mut ida, &mut f3, 0);
        let mut l30 = 0i32;
        let mut l58 = 0i32;
        let (mut l8c, mut l54, mut l94, mut l98) = (0i8, 0u8, 0i8, 0i8);
        if modo == 4 {
            *w(p, 0x29) = 0x15e;
        } else {
            if byte(m, 0x8ea9) == -1 {
                *w(p, 0x29) = 200;
            } else {
                l49 = pressao_com(m, p, &mut l70, la, &mut |l, q| {
                    chamar(0x68d0d0, l, [q as u32, 0x1be, 0, 0, 0]);
                }) as i8;
                if l49 == 0 {
                    if l60 as f64 > f64_em(0x968e38) {
                        l60 = 3.5;
                    }
                    *w(p, 0x29) = ftol(l60 as f64 * f32_em(0x96d7b0) as f64 + f32_em(0x96d844) as f64) as i16;
                } else {
                    if l70 < 0 {
                        l70 = 0;
                    }
                    let s = l70 as i32;
                    let k = (l49 as i32).wrapping_mul(s).wrapping_mul(s);
                    l58 = ftol(-(k as f64 * f64_em(0x969040)));
                    if l58 < -500 {
                        l58 = sortear((l58 + 500) / 3) - 500;
                        if l58 < -2000 {
                            l58 = -2000;
                        }
                    } else if l58 > 32000 {
                        l58 = 32000;
                    }
                    *w(p, 0x29) = ftol(l58 as f64 * f64_em(0x969040)) as i16;
                }
            }
            if na_metade_de_ataque(lado, y0()) {
                l27 = jogadores_no_trajeto(m, x0(), y0(), 4, if lado != 0 { 0 } else { 0xb }) as i8;
            }
            let soma = if na_area_atacada(lado, x0(), y0()) { 2 } else { 4 };
            let mut l28 = 0i32;
            l58 = escolher_companheiro(m, x0(), y0(), &mut l8c, &mut l54, &mut l94, &mut l98, &mut l28, -1) as i32;
            l30 = (l49 as i32 + l27 as i32 + soma).wrapping_mul(l28) / 5;
        }
        *w(p, 0x29) = (*w(p, 0x29)).wrapping_add(0x46);
        let flags = *((m + 0x9766 + (lado == 0) as usize * 0x18e3) as *const u32);
        if flags & 0x400 != 0 && byte(p, 0x106) as i32 > sortear(5) && !na_area_atacada(lado, x0(), y0()) && !na_area_defendida(lado, x0(), y0()) && na_metade_de_ataque(lado, y0()) {
            crate::comparacao::ramo("conducao instrucao de avancar");
            l14 = 4000 - l30;
            l44 = 2000 - l30 / 2;
        }
        // a nota do chute
        if byte(m, 0x8ea9) == 4 {
            if ff < f64_em(0x967890) || ((y0() == 0 || y0() == 0xb) && (x0() as i32 - 4).abs() == 1) {
                l10 = ftol(fl(0xa9) * f32_em(0x96d750) as f64 + f32_em(0x968dac) as f64 - (400 * l27 as i32) as f64);
            } else {
                l10 = -5000;
            }
        } else if ff <= f64_em(0x968940) {
            l10 = ftol(fl(0x99) * f32_em(0x968960) as f64 + f32_em(0x96d408) as f64 - (400 * l27 as i32) as f64 + *w(p, 0x29) as f64 * f64_em(0x968978));
        } else {
            let bl = byte(p, 0x109) as i32;
            let n = bl / 7 + sortear(3) + 3;
            if (n as f64) <= ff || (modo == 4 && bl <= sortear(0x32)) {
                l10 = -500;
            } else {
                crate::comparacao::ramo("conducao chute de longe");
                let v = valor_do_chute(p, &mut l88, &mut l64) as i16 as i32;
                let k = byte(p, 0x106) as i32 * 4;
                let r1 = sortear(ftol(ff * f32_em(0x96d750) as f64 - k as f64));
                let t2 = ftol(k as f64 - ff * f32_em(0x96d750) as f64);
                la4 = sortear(-(r1.wrapping_add(t2))).wrapping_add(v);
                l37 = true;
                if v > 0 {
                    let div = if byte(p, 0x109) < 1 { 1 } else { byte(p, 0x109) as i32 };
                    l10 = ftol((v * v) as f64 * f64_em(0x968a80) - ff * ff * f32_em(0x96d40c) as f64 / div as f64 - (700 * l27 as i32) as f64);
                } else {
                    l10 = ((byte(p, 0x10b) as i32 + byte(p, 0x106) as i32) * 4 + 2) * v;
                }
            }
        }
        // o placar
        let dif = fora as i32 - casa as i32;
        if byte(m, 0x1ab + lado as usize) == 5 {
            l10 = l10.wrapping_add(dif * 3000);
            l30 = l30.wrapping_mul(2);
            if !(m != 0 && tabela_a(m, p) == 1) && sortear((dif * 3 + byte(u(p, 0x69), 0x5b) as i32) * 2) < 1 {
                crate::comparacao::ramo("conducao mata-mata perde folego");
                *b(p, 0x100) = (*b(p, 0x100)).wrapping_sub(1);
            }
        } else if (fora as i32) < casa as i32 - 2 {
            l10 = l10.wrapping_add((dif + 2) * 3000);
            l30 = l30.wrapping_mul(2);
        }
        if byte(m, 0x8ea9) < 4 && ff > f32_em(0x968970) as f64 {
            if l68 == 2 {
                let pw = pow_do_jogo(ftol(ff) as f64, f64_em(0x968930));
                let q = fl(0x95);
                let r = sortear(4) + 1;
                let k = ftol((q * q + f64_em(0x967890)) * f64_em(0x968968)) as i16 as i32;
                l14 = l14.wrapping_add(ftol((r * k) as f64 * pw));
            }
            if tabela_12x9(y0() as i32 - byte(g, 0x103) as i32, x0() as i32 - byte(g, 0x102) as i32) as f64 == f64_em(0x9689a0)
                && byte(m, 0x21b6 + (y0() as isize + x0() as isize * 12) as usize * 0x5a + (lado == 0) as usize) == 1
            {
                crate::comparacao::ramo("conducao cara a cara");
                let r1 = sortear(ftol(fl(0x99) * f32_em(0x968a78) as f64));
                let r2 = sortear(ftol(fl(0x95) * f32_em(0x968a78) as f64));
                l14 = l14.wrapping_add(ftol((r2 - r1) as f64 * (ff * f32_em(0x969560) as f64 + f32_em(0x96d840) as f64)));
                l4c = true;
            }
        }
        // o fim de jogo: a dividida para ganhar tempo
        if l68 == 0 && l30 < 3000 && modo != 4 && (*w(m, 0x8ed0) as i32 / 11) > sortear(0x14) + 0x46 && !na_area_atacada(lado, x0(), y0()) && byte(p, 0x106) != 0 && byte(m, 0x8ea9) < 3 {
            let e = sortear(byte(u(p, 0x69), 0x5c) as i32);
            if e < sortear(10) && lado_sob_controle(m, lado) != 0 {
                let r = placar_agregado(m, lado, &mut casa, &mut fora, &mut cl, &mut ida, &mut f3, 1) as i8;
                if r > 0 && r < 3 {
                    if !(2..=6).contains(&x0()) && ((lado == 1 && y0() < 3) || (lado == 0 && y0() > 8)) {
                        crate::comparacao::ramo("conducao segura a bola");
                        chamar(0x70ec80, p, [1, 0, 0, 0, 0]);
                        *w(p, 0x198) = 0x10;
                        // ver o fim da 0x7139cb
                        break 'fim;
                    }
                    if na_metade_de_ataque(lado, y0()) && *w(p, 0x29) > -0x14 && l10 < 0 {
                        crate::comparacao::ramo("conducao segura o resultado");
                        l5f = true;
                        l14 += 4000;
                    }
                }
            }
        }
        if ff < f64_em(0x968930) && l10 > 0 {
            l44 = l44.wrapping_add(l10 / -5);
        }
        if m != 0 && tabela_a(m, p) == 1 {
            if byte(p, 0x10a) != 0 {
                l14 = l14.wrapping_add(ftol(fl(0x9d) * f32_em(0x96d210) as f64 - f32_em(0x968db0) as f64));
            }
            if na_metade_de_defesa(lado, y0()) {
                l10 = l10.wrapping_sub(25000);
            }
        }
        // o goleiro (ou quem guarda a posicao 1)
        if g != 0 && na_metade_de_ataque(lado, y0()) {
            let t = if u(g, 0x19e) != 0 { tabela_a(u(g, 0x19e), g) } else { 0 };
            if t & 1 == 0 {
                l10 = l10.wrapping_add(5000);
            } else if byte(g, 0x2b) != 0 && byte(p, 0x10b) as i32 + 5 > sortear(ftol(ff * f32_em(0x96afb0) as f64)) {
                l10 = l10.wrapping_add(1500);
            }
            let (gx, gy) = (byte(g, 0x102), byte(g, 0x103));
            if !na_area_atacada(lado, gx, gy) {
                if byte(p, 0x10b) as i32 + 5 > sortear(ftol(ff)) {
                    l10 = l10.wrapping_sub(ftol(fl(0x95) * f32_em(0x968960) as f64 - f32_em(0x968dac) as f64));
                }
            } else if gx == 4 && gy == if byte(g, 0x27) == 1 { 0xb } else { 0 } {
                if alcanca_o_ponto(g, x0(), y0()) == 0 && byte(p, 0x10b) as i32 > sortear(ftol(ff * f32_em(0x968a78) as f64)) {
                    crate::comparacao::ramo("conducao goleiro no gol");
                    l10 = l10.wrapping_sub(ftol(fl(0x95) * f32_em(0x96d664) as f64 - f32_em(0x96d2dc) as f64));
                }
            } else {
                let e17 = byte(p, 0x17) as i32;
                let r = sortear(ftol(ff * ff * f32_em(0x969584) as f64));
                if byte(p, 0x10b) as i32 + e17 - 10 > r {
                    crate::comparacao::ramo("conducao goleiro adiantado");
                    l10 = l10.wrapping_sub(ftol(fl(0x95) * f32_em(0x96d664) as f64 - f32_em(0x96d3e4) as f64));
                }
                if x0() == gx && y0() == gy {
                    l14 = l14.wrapping_add(ftol((e17 as f64 + fl(0x95)) * f32_em(0x968960) as f64 - f32_em(0x96d408) as f64));
                }
            }
        }
        if byte(m, 0x8ea9) == 4 && ff > f64_em(0x967890) {
            l10 = l10.wrapping_add(ftol(ff * ff * f64_em(0x96d838)));
        } else if (4..=6).contains(&x0()) && ((lado == 1 && y0() == 0) || (lado == 0 && y0() == 0xb)) {
            l10 = l10.wrapping_add(5000 - (x0() as i32 - 4).abs() * 2000);
        }
        if l58 == 0 && l53 == 0 {
            l30 -= 2000;
        }
        // lancar
        if limite((lado == 0) as u8, y0()) != 0 && byte(m, 0x8ea9) != -1 && byte(p, 0x106) != 0 && l30 < 4000 - *w(m, 0x8ed0) as i32 && lado_sob_controle(m, lado) != 0 {
            crate::comparacao::ramo("conducao lancamento");
            chamar(0x6fa490, p, [byte(m, 0x8ea9) as u8 as u32, 1, 0, 0, 0]);
            break 'fim;
        }
        let perto = (na_area_defendida(lado, x0(), y0()) && l60 as f64 <= f64_em(0x968940)) || (limite((lado == 0) as u8, y0()) != 0 && (l60 as f64) < f64_em(0x967890));
        if perto {
            let r1 = sortear(ftol(f32_em(0x96d408) as f64 - l60 as f64 * f32_em(0x96d2dc) as f64));
            if r1 > sortear(l30 + 2000) && byte(m, 0x8ea9) != -1 && modo != 4 {
                crate::comparacao::ramo("conducao alivio");
                chamar(0x6fa490, p, [byte(m, 0x8ea9) as u8 as u32, 0, 0, 0, 0]);
                break 'fim;
            }
        }
        // a meta (o meio do gol atacado) e a boca do gol
        let ty: i8 = if lado == 0 { 0xb } else { 0 };
        let na_boca = || y0() == ty && (3..=5).contains(&x0());
        let consegue = || consegue_jogar(p, 4, ty, 0, 0.0, 0.0, core::ptr::null_mut()) != 0;
        if (diferenca_lado(m, lado as u8, y0() as u8) as i8) < 0 && (caminho_livre(m, p, 0) == 0 || l53 != 0 || tabela_a(m, p) == 1) {
            if na_area_atacada(lado, x0(), y0()) {
                l44 -= 1000;
            }
            if na_area_defendida(lado, x0(), y0()) {
                l44 -= 1000;
                l14 -= 1200;
            } else if na_metade_de_defesa(lado, y0()) && !na_boca() && !consegue() {
                l14 = l14.wrapping_add((-5 - l70 as i32) * 100);
            }
        } else {
            l44 -= 5000;
            l14 = l14.wrapping_add(ftol(ff * f32_em(0x96d3e4) as f64 - f32_em(0x96d3e4) as f64));
            if modo == 4 {
                l14 = l14.wrapping_add(ftol(ff * f32_em(0x96d404) as f64 - f32_em(0x96d3e4) as f64));
            }
            if na_metade_de_defesa(lado, y0()) {
                l10 -= 3000;
            }
            if na_area_atacada(lado, x0(), y0()) {
                l44 -= 1000;
                l10 += 1000;
            }
        }
        if tabela_a(m, p) == 1 || na_area_defendida(lado, x0(), y0()) {
            l44 = l44.wrapping_add(ftol(l60 as f64 * f32_em(0x96d1a8) as f64 - f32_em(0x96a588) as f64));
        } else {
            l14 = l14.wrapping_add(ftol((fl(0x95) - f32_em(0x96afb0) as f64 + l60 as f64 * f64_em(0x968eb0)) * f64_em(0x96afe8)));
        }
        if !na_boca() && !consegue() && na_metade_de_ataque(lado, y0()) {
            l14 = l14.wrapping_add(2000 - l30);
        }
        // a casa para onde conduzir (0x6bce80)
        let (mut sx, mut sy) = (l74, l36);
        let mut l43 = escolher_casa(m, p, &mut sx, &mut sy, (l68 != 0) as u32 + 2) as i8;
        let fundo = (lado == 1 && y0() < 3) || (lado == 0 && y0() > 8);
        let ta = |q: usize| if u(q, 0x19e) != 0 { tabela_a(u(q, 0x19e), q) } else { 0 };
        let tb = |q: usize| if u(q, 0x19e) != 0 { tabela_b(u(q, 0x19e), q) } else { 0 };
        if l43 != 0 {
            if modo == 4 {
                sx = sx.clamp(3, 5);
            }
            l44 -= 3000;
            if !(2..=6).contains(&x0()) && !fundo && !na_metade_de_defesa(lado, y0()) {
                l14 += 1250;
                l10 -= 500;
            } else if fundo && !na_area_atacada(lado, x0(), y0()) {
                l14 += 1000;
                l10 = l10.wrapping_add((byte(p, 0x109) as i32 - 20) * 100);
            }
        } else if (2..=6).contains(&x0()) || fundo {
            l14 -= 3000;
        } else {
            let k = (byte(p, 0x109) as f64 + fl(0x95)) * f32_em(0x968a78) as f64;
            if tabela_a(m, p) & 0x38 != 0 && ta(p) & 0x880 != 0 && tb(p) & 0x880 != 0 && !((lado == 0 && y0() < 3) || (lado == 1 && y0() > 8)) {
                crate::comparacao::ramo("conducao ponta de lanca");
                l14 = l14.wrapping_add(ftol(k));
            } else if tabela_a(m, p) & 0x38 != 0 && tabela_a(m, p) & 0x880 != 0 {
                l14 = l14.wrapping_add(ftol(k - f32_em(0x96d210) as f64));
            } else {
                l14 -= 3000;
            }
            l10 -= 650;
        }
        // o chute de fora da area
        let na_area = na_area_atacada(lado, x0(), y0());
        if (2..=6).contains(&x0()) && !na_area && byte(m, 0x8ea9) != 4 {
            let t = tabela_a(m, p);
            let bl = byte(p, 0x109) as i32;
            let r = sortear(ftol((l27 as i32 + 1) as f64 * (ff + f32_em(0x968e70) as f64) * ff * ff * ff * f32_em(0x96d834) as f64));
            l10 = l10.wrapping_add(if t & 0x40 != 0 { bl * 500 } else { bl * 325 } - r);
        } else if na_area {
            l10 = l10.wrapping_add(ftol((byte(p, 0x109) as f64 + fl(0x99) - fl(0xe1)) * (f32_em(0x96afb0) as f64 - ff * f32_em(0x96d228) as f64)));
        } else if na_metade_de_ataque(lado, y0()) {
            l10 = l10.wrapping_sub(ftol(ff * f32_em(0x96d20c) as f64 - (byte(p, 0x109) as i32 * 75) as f64));
        }
        let marcas = |q: usize| marcas_da_posicao(m + 0x91e2 + byte(q, 0x27) as isize as usize * 0x18e3, byte(q, 0x19));
        'nota: {
            if sortear(ftol(fl(0xe1))) == 0 && !(tabela_a(m, p) & 0x40 == 0 && sortear(4) != 0) {
                crate::comparacao::ramo("conducao folego");
                if !(na_metade_de_defesa(lado, y0()) && (byte(p, 0x106) != 0 || byte(p, 0x10b) != 0)) {
                    let bl = na_area_defendida(lado, x0(), y0()) as i32;
                    let pw = pow_do_jogo(fl(0x95), f64_em(0x968930));
                    l14 = l14.wrapping_add(ftol(pw + (1000 - bl * 1000) as f64));
                }
                if ff >= f64_em(0x968950) && ff < f64_em(0x96afa0) && (2..=6).contains(&x0()) {
                    if byte(p, 0x109) != 0 {
                        let r = sortear(ftol(fl(0xb1)));
                        l10 = l10.wrapping_add(*w(p, 0x29) as i32 * 2 + r * 400 + 2500);
                    }
                } else if na_area {
                    l10 = l10.wrapping_add(ftol((f32_em(0x96afb0) as f64 - ff * f32_em(0x96d228) as f64) * (byte(p, 0x109) as f64 + fl(0x99))));
                }
                break 'nota;
            }
            if modo == 4 {
                break 'nota;
            }
            if marcas(p) & 0x2000000 != 0 && !na_area_defendida(lado, x0(), y0()) && l49 < 2 && ff >= f64_em(0x968950) {
                let segue = if l70 != 0 && na_metade_de_defesa(lado, y0()) { (sortear(byte(p, 0x106) as i32 * 2 + 20) as f64) < ff } else { true };
                if segue {
                    crate::comparacao::ramo("conducao armador");
                    let k = (byte(p, 0x11) as u8 as i32 * 30 + *w(p, 0x29) as i32 + 400) * 10 - l30;
                    l14 = l14.wrapping_add(ftol(k as f64 + byte(p, 0x109) as f64 * fl(0x95) * fl(0xe1)));
                    if !(2..=6).contains(&x0()) && ta(p) & 0x880 != 0 && tb(p) & 0x880 != 0 && !fundo && marcas(p) & 0x4_0000_0000 != 0 {
                        l14 += 500;
                        l30 -= 250;
                    }
                }
            }
            if marcas(p) & 0x8000000 != 0 && na_metade_de_ataque(lado, y0()) && !na_boca() && !consegue() {
                l44 += 3500;
            }
            if marcas(p) & 0x20000000 != 0 && ff >= f64_em(0x968950) && ff < f64_em(0x96afa0) {
                let r = sortear(ftol(fl(0xb1)));
                l10 = l10.wrapping_add((*w(p, 0x29) as i32 + r * 200) * 2);
            }
        }
        // o lance de efeito e a pressao
        let c = byte(m, 0x8ea9);
        if c < 2 || (ff > f64_em(0x96d7a8) && c < 4) {
            let mut saida = 0u16;
            let r = classificar_lance(p, 0x16, x0(), y0(), 4, if lado == 1 { 0 } else { 0xb }, (ff + f64_em(0x967890)) as f32, &mut saida);
            if r != 2 && r != byte(p, 0x2c) as u8 && l68 == 0 {
                l10 = l10.wrapping_sub(sortear(byte(p, 0x2d) as i32 * 300));
            }
        }
        if ff < f64_em(0x9688a0) {
            let s = l70 as i32;
            l10 = l10.wrapping_sub(ftol(s.wrapping_mul(s).wrapping_mul(s) as f64 / (f64_em(0x9688b0) - ff)));
        }
        if byte(m, 0x8ea9) == 3 {
            l10 -= 2000;
        }
        let e = (l43 as i32 - l2f as i8 as i32) * 100;
        let t = ftol((f32_em(0x96d830) as f64 - fl(0x9d) * f32_em(0x96b020) as f64) * l90 as f64 * l90 as f64);
        l44 = l44.wrapping_add(e);
        l14 = l14.wrapping_add(e - t);
        let bl = byte(p, 0x109) as i32;
        if !(ff < f64_em(0x967890)) && bl * bl * bl * bl > sortear(ftol(ff * f32_em(0x96d1b0) as f64)) {
            if !na_area_defendida(lado, x0(), y0()) && ff >= f64_em(0x968950) && l49 < 2 && !(na_metade_de_defesa(lado, y0()) && (byte(p, 0x106) != 0 || byte(p, 0x10b) != 0)) {
                let pw = pow_do_jogo(fl(0x95), f64_em(0x968930));
                l14 = l14.wrapping_add(sortear(ftol(pw + (*w(p, 0x29) as i32 * 3) as f64 + (10000 - l30) as f64)));
            }
            if byte(m, 0x8ea9) < 4 && l27 < 2 && (ff < f64_em(0x9688a0) || (ff < f64_em(0x968988) && sortear(500) == 0)) {
                crate::comparacao::ramo("conducao chute colocado");
                let mut r = sortear(ftol(ff * f32_em(0x968a78) as f64)) as i8;
                if r < 10 {
                    r = 10;
                }
                l10 = l10.wrapping_add(sortear(400000 / r as i32));
            }
            l44 -= 10000;
        } else if !na_area_defendida(lado, x0(), y0()) {
            let e = sortear(ftol(byte(p, 0x106) as f64 * f64_em(0x968dd8)));
            if e > sortear(200) {
                // os adversarios na mesma casa
                let casa = m + 0x215e + (y0() as isize + x0() as isize * 12) as usize * 90;
                let outro = (lado == 0) as usize;
                let mut i = 0i8;
                while i < *((casa + outro + 0x58) as *const i8) {
                    let q = *((casa + outro * 44 + i as usize * 4) as *const usize);
                    if byte(q, 0x24) != 0 {
                        l14 = l14.wrapping_add(sortear(ftol((byte(p, 0x17) as i32 * 100) as f64 + fl(0x95) * f32_em(0x96d7c0) as f64)));
                    }
                    i += 1;
                }
            }
        }
        // o desconto da conducao (ebx do original)
        let k = ftol(pow_do_jogo(byte(m, 0x8ea9) as f64, f64_em(0x968930)) * f64_em(0x96d828));
        let mut nc = (-(*w(p, 0x29) as i32) - *w(p, 0x19c) as i32 * 10) * 10;
        nc = nc.wrapping_sub(ftol((f32_em(0x96d1b0) as f64 - fl(0x4d)) * f32_em(0x96d820) as f64)).wrapping_sub(k);
        if l58 != 0 && l30 > 0 && matches!(l8c as u8, 0x5e | 8 | 7 | 6) {
            nc = nc.wrapping_add(l30);
        } else if marcas(p) & 0x4_0000_0000 != 0 && ta(p) & 0x880 != 0 && tb(p) & 0x880 != 0 {
            if y0() == 0 || y0() == 0xb {
                nc += 500;
            } else if (2..=6).contains(&x0()) {
                // nada
            } else if fundo {
                l14 += 2000;
            } else if ff < f64_em(0x9688a0) {
                l14 += 6000;
            }
        }
        if !l4c && l10 > 2000 && *w(p, 0x29) > -0x32 {
            nc = nc.wrapping_add(sortear(l10));
        }
        if byte(p, 0x106) == 0 {
            nc = sortear(nc);
        }
        if na_metade_de_defesa(lado, y0()) && nc < 0 {
            nc = 0;
        }
        l14 = l14.wrapping_sub(nc);
        // na linha de fundo, fora do meio: cruza (ou guarda a jogada ensaiada)
        if fora_do_meio(x0()) != 0 && !(m != 0 && byte(m, 0x8ea9) >= 3) && ((lado == 0 && y0() == 0xb) || (lado == 1 && y0() == 0)) {
            if l58 != 0 && matches!(l8c, 8 | 7 | 6) {
                l30 += 1500;
                if marcas(p) & 0x4_0000_0000 != 0 {
                    l30 += 1500;
                }
            } else if sortear(10) != 0 {
                crate::comparacao::ramo("conducao cruza da linha de fundo");
                chamar(0x6fa490, p, [byte(m, 0x8ea9) as u8 as u32, 0, 0, 0, 0]);
                break 'fim;
            }
        }
        let k = (20 - byte(p, 0x106) as i32) * 50;
        l30 = l30.wrapping_add(sortear(k));
        l14 = l14.wrapping_add(sortear(k));
        let l44s = l44.wrapping_add(sortear(k));
        let mut nota = l10.wrapping_add(sortear(k));
        // a acao: 1 passa, 2 divide, 3 chuta, 4 conduz
        let mut acao = 3u8;
        if nota < l14 && byte(m, 0x8ea9) != 4 && byte(m, 0x8ea9) != -1 && !(x0() == 4 && ((lado == 0 && y0() == 0xb) || (lado == 1 && y0() == 0))) {
            nota = l14;
            acao = 4;
        }
        if nota < l44s && byte(m, 0x8ea9) != 4 && byte(m, 0x8ea9) != -1 && caminho_livre(m, p, 1) == 0 && !(*w(p, 0x198) == 0x10 && byte(m, 0x8d46 + lado as usize) >= 0xc) {
            nota = l44s;
            acao = 2;
        }
        'acao: {
            if *w(p, 0x198) != 0x10 && byte(m, 0x8ea9) == -1 {
                acao = 2;
                break 'acao;
            }
            if nota < l30 && modo != 4 {
                if l58 != 0 {
                    acao = 1;
                    break 'acao;
                }
                if (l53 != 0 && !fundo) || byte(m, 0x8ea9) == 4 {
                    crate::comparacao::ramo("conducao alivia sem companheiro");
                    chamar(0x6fa490, p, [byte(m, 0x8ea9) as u8 as u32, 0, 0, 0, 0]);
                    break 'fim;
                }
                acao = if sortear(if *w(p, 0x198) != 0x10 { 2 } else { 10 }) != 0 { 4 } else { 2 };
            }
            if acao == 3 {
                if !na_boca() && !consegue() {
                    match byte(m, 0x8ea9) {
                        3 => {
                            if !(byte(p, 0x109) as i32 > sortear(0xf) && ff < f64_em(0x96d700)) {
                                acao = 2;
                                break 'acao;
                            }
                        }
                        4 => {
                            acao = 1;
                            break 'acao;
                        }
                        c => {
                            acao = if c == -1 { 1 } else { 4 };
                            break 'acao;
                        }
                    }
                }
                let c = byte(m, 0x8ea9);
                if (c == 4 && !(ff < f64_em(0x968950))) || c == -1 {
                    acao = 1;
                    break 'acao;
                }
                if !na_metade_de_defesa(lado, y0()) && ((2..=6).contains(&x0()) || byte(p, 0x109) as i32 >= sortear(0x78)) {
                    break 'acao;
                }
                if modo == 4 {
                    break 'acao;
                }
                if l58 != 0 {
                    acao = 1;
                    break 'acao;
                }
                if (m != 0 && byte(m, 0x8ea9) >= 3) || l60 as f64 <= f64_em(0x968940) {
                    crate::comparacao::ramo("conducao alivia de longe");
                    chamar(0x6fa490, p, [byte(m, 0x8ea9) as u8 as u32, 0, 0, 0, 0]);
                    break 'fim;
                }
                if *w(p, 0x198) == 2 {
                    acao = 4;
                } else {
                    let bl = byte(p, 0x109) as i32;
                    let e = sortear(ftol(fl(0x95)) as i16 as i32) + bl;
                    acao = if e < sortear(0x32) { 2 } else { 4 };
                }
            } else if acao == 4 && (byte(m, 0x8ea9) == 4 || byte(m, 0x8ea9) == -1) {
                acao = 1;
            }
        }
        if *w(p, 0x198) == 0x3b {
            acao = 3;
        }
        if modo == 4 && (acao == 1 || acao == 2 || l43 == 0) {
            acao = 3;
        }
        let narrar = |chamar: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32| {
            chamar(0x6d6f70, u(m, 0x8f7a), [0x1f44, m as u32, ub(p, 0x2e), lado as u8 as u32, u32::MAX]);
            *w(p, 0x35) = (*w(p, 0x35)).wrapping_add(0x19);
        };
        match acao {
            1 => {
                if l58 == 0 {
                    crate::comparacao::ramo("conducao lanca sem companheiro");
                    *w(p, 0x198) = 9;
                    chamar(0x6fa490, p, [byte(m, 0x8ea9) as u8 as u32, 0, 0, 0, 0]);
                } else {
                    crate::comparacao::ramo("conducao passa");
                    *w(p, 0x198) = 0x6c;
                    chamar(0x6fa250, p, [l8c as u8 as u32, l58 as u32, l54 as u32, l94 as u8 as u32, l98 as u8 as u32]);
                }
            }
            2 => {
                crate::comparacao::ramo("conducao divide");
                chamar(0x70ec80, p, [0; 5]);
                *w(p, 0x198) = 0x10;
            }
            3 => {
                if modo == 4 {
                    crate::comparacao::ramo("conducao goleiro fica");
                    volta = 1;
                    break 'fim;
                }
                crate::comparacao::ramo("conducao chuta");
                if byte(m, 0x1a7) != 0 && *w(p, 0x198) != 0x3b && byte(m, 0x8ea9) == 0 {
                    narrar(chamar);
                }
                let v = if l37 {
                    let v = valor_do_chute(p, &mut l88, &mut l64) as i16 as i32;
                    v / 2 + la4 / 2
                } else {
                    valor_do_chute(p, &mut l88, &mut l64) as i16 as i32
                };
                let r = conferir_lance(m);
                chutar(chamar, l88 as u8 as u32, v, l64, r as u32, true);
            }
            _ => {
                // conduzir
                let (mut cx, mut cy) = (sx, sy);
                if l5f {
                    crate::comparacao::ramo("conducao segura no canto");
                    cx = if l74 > 4 || (l74 == 4 && sortear(2) == 0) { 8 } else { 0 };
                    cy = ty;
                    l43 = 1;
                }
                if !na_boca() && !consegue() && (l43 == 0 || modo == 4) {
                    crate::comparacao::ramo("conducao vira para o gol");
                    let d = if lado == 0 {
                        if y0() < 9 { y0() + 3 } else { 0xb }
                    } else if y0() > 3 {
                        y0() - 3
                    } else {
                        0
                    };
                    apontar(p, x0(), y0(), 4, d);
                    chamar(0x6f09f0, p, [x0() as u8 as u32, y0() as u8 as u32, 0xf, 0, 0]);
                    *w(p, 0x198) = 0xf;
                    break 'fim;
                }
                if l43 == 0 && modo != 4 && byte(p, 0x109) == 0 {
                    if sortear(10) == 0 || alcanca(p, 4, ty) != 0 {
                        crate::comparacao::ramo("conducao sem saida lanca");
                        *w(p, 0x198) = 9;
                        chamar(0x6fa490, p, [byte(m, 0x8ea9) as u8 as u32, 0, 0, 0, 0]);
                    } else {
                        crate::comparacao::ramo("conducao sem saida divide");
                        chamar(0x70ec80, p, [0; 5]);
                        *w(p, 0x198) = 0x10;
                    }
                    break 'fim;
                }
                let nx = cx.clamp(0, 8);
                let ny = cy.clamp(0, 0xb);
                if (lado == 0 && nx >= x0()) || (lado == 1 && nx <= x0()) {
                    apontar(p, x0(), y0(), ((nx as i32 + 4) / 2) as i8, ((ny as i32 / 2) as i8).wrapping_add(if lado != 0 { 0 } else { 6 }));
                } else {
                    apontar(p, x0(), y0(), nx, ny);
                }
                if byte(m, 0x1a7) != 0 {
                    narrar(chamar);
                }
                let a = *w(p, 0x198);
                if a == 0x12 || a == 0x11 {
                    chamar(0x6f09f0, p, [nx as u8 as u32, ny as u8 as u32, 0x12, l68 as u32, modo as u32]);
                    *w(p, 0x198) = 0x12;
                } else {
                    let q = u(m, 0xf5a6);
                    if l68 != 0 && p == u(m, 0xf5aa) && q != 0 && *w(p, 0x29) > -0x64 && *w(q, 0x198) != 0x34 && *w(q, 0x198) != 0x37 {
                        crate::comparacao::ramo("conducao tabela devolvida");
                        *b(q, 3) = (*b(q, 3)).wrapping_add(1);
                        *w(q, 0x35) = (*w(q, 0x35)).wrapping_add((*w(p, 0x29) as i32 / 3 + 0x113) as i16);
                        let t = (m + 0x4782) as *mut i32;
                        *t = (*t).wrapping_add(0x113);
                    }
                    chamar(0x6f09f0, p, [nx as u8 as u32, ny as u8 as u32, 0x11, l68 as u32, modo as u32]);
                    *w(p, 0x198) = 0x11;
                }
            }
        }
    }
    // o destrutor da lista (0x68d040): na copia Rust da comparacao ela fica vazia
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    destruir(la);
    volta
}

/// A lista da pressao na pilha da conducao entra na fita sem o endereco (o da original e o da
/// copia Rust diferem); o resto da copia (o bloco do jogador, 0x1be bytes) e conferido. Vale nas
/// duas fases da comparacao (na da copia Rust a fita ja foi tirada).
fn lista_gravada(l: usize, n: u32) -> usize {
    if GRAVAR_CONDUCAO.load(std::sync::atomic::Ordering::Relaxed) && n == 0x1be {
        0
    } else {
        l
    }
}

/// Modo Rust: as chamadas da conducao vao para o jogo (passe, lancamento, dividida e movimento pelos
/// proprios desvios; o chute com a lista: a cabeca e o endereco).
unsafe fn despachar_conducao(rotina: usize, this: usize, a: [u32; 5]) -> u32 {
    match rotina {
        0x6fa250 | 0x6f09f0 => {
            let f: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> u32 = core::mem::transmute(rotina);
            f(this, a[0], a[1], a[2], a[3], a[4])
        }
        0x6fa490 => {
            let f: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(rotina);
            f(this, a[0], a[1]);
            0
        }
        0x70ec80 => {
            let f: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(rotina);
            f(this, a[0]);
            0
        }
        0x702050 => {
            let f: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32) = core::mem::transmute(rotina);
            let l = a[4] as usize;
            f(this, a[0], a[1], a[2], a[3], if l != 0 { *(l as *const u32) } else { 0 }, l as u32);
            0
        }
        0x68d0d0 => {
            let f: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(rotina);
            f(this, a[0] as usize, a[1]);
            0
        }
        _ => despachar_dividida(rotina, this, a),
    }
}

/// { 0x7109f0 } (ret 4, match_pl.cpp): ver `conduzir_com`.
unsafe extern "thiscall" fn conduzir(p: usize, modo: u32) -> u32 {
    conduzir_com(p, modo as u8, &mut |r, t, a| despachar_conducao(r, t, a)) as u32
}

unsafe extern "thiscall" fn comparar_conduzir(p: usize, modo: u32) -> u32 {
    if gravando() {
        // unidade da fita de fora (penaltis, decisao do jogador); por dentro, a propria comparacao
        return chamar_gravando(0x7109f0, p, [modo, 0, 0, 0, 0], || fita_da_conducao(p, modo));
    }
    fita_da_conducao(p, modo)
}

unsafe fn fita_da_conducao(p: usize, modo: u32) -> u32 {
    use std::sync::atomic::Ordering::Relaxed;
    let o = orig::<unsafe extern "thiscall" fn(usize, u32) -> u32>(319);
    let m = *((p + 0x19e) as *const usize);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(m, MOTOR), (0xb67a34, 4), (0xae1080, 4)]);
    let t = *((m + 0x4792) as *const usize);
    for clube in [*((t + 0x1c) as *const usize), *((t + 0x20) as *const usize)] {
        reg.extend(linhas_do_clube(clube));
    }
    let gravar = GRAVAR_716.swap(true, Relaxed);
    let a7 = GRAVAR_A7.swap(true, Relaxed);
    let chute = GRAVAR_702050.swap(true, Relaxed);
    let cond = GRAVAR_CONDUCAO.swap(true, Relaxed);
    let (r, rr, igual, sequencia, k) = comparar_com_fita(
        reg,
        || {
            zerar_pilha();
            o(p, modo) & 0xff
        },
        |chamar| {
            conduzir_com(p, modo as u8, &mut |rotina, this, args| match rotina {
                // passe, lancamento e dividida: as unidades deles entram na fita da conducao
                0x6fa250 => {
                    passar_com(this, args[0] as u8, args[1] as usize, args[3], args[4], &mut |r, t, a| {
                        let mascara = match r {
                            0x6fad40 => [0xff, 0xff, 0xff, u32::MAX, u32::MAX],
                            0x6d6f70 => [0xffff, u32::MAX, 0xff, 0xff, 0xff],
                            0x7103e0 => [0; 5],
                            _ => [u32::MAX; 5],
                        };
                        chamar(r, t, a, mascara)
                    });
                    0
                }
                0x6fa490 => {
                    lancar_com(this, args[0] as i8, args[1] as u8, &mut |r, t, a| {
                        let mascara = match r {
                            0x6fad40 => [0xff, 0xff, u32::MAX, u32::MAX, 0xff],
                            0x6d6f70 => [0xffff, u32::MAX, 0xff, 0xff, 0xff],
                            0x6cd490 => [0xff, 0, 0, 0, 0],
                            _ => [u32::MAX; 5],
                        };
                        // a porta da 0x6fad40 grava (x, y, tipo, receptor, coluna); a linha do alvo nao entra
                        let a = if r == 0x6fad40 { [a[0], a[1], a[2], 0, a[3]] } else { a };
                        chamar(r, t, a, mascara)
                    });
                    0
                }
                0x70ec80 => {
                    dividida_com(this, args[0] as u8, &mut |r, t, a| {
                        let mascara = match r {
                            0x6d6f70 => [0xffff, u32::MAX, 0xff, 0xff, 0xff],
                            0x6cd770 => [0xff, 0xff, 0xffff, 0xff, u32::MAX],
                            0x62ed70 => [u32::MAX, 0, 0, 0, 0],
                            0x6d0980 => [0xff, 0xff, u32::MAX, u32::MAX, 0],
                            0x897760 => [u32::MAX, 0xff, u32::MAX, 0, 0],
                            0x6e1aa0 | 0x716c90 | 0x6f0630 => [0, 0, 0, 0, 0],
                            _ => [0xff, 0, 0, 0, 0],
                        };
                        chamar(r, t, a, mascara)
                    });
                    0
                }
                0x68d0d0 => chamar(rotina, lista_gravada(this, args[1]), args, [u32::MAX, u32::MAX, 0, 0, 0]),
                _ => {
                    let mascara = match rotina {
                        0x6d6f70 => [0xff_ffff, u32::MAX, 0xff, 0xff, 0xff],
                        0x6f09f0 => [0xff, 0xff, 0xffff, 0xff, 0xff],
                        // a cabeca da lista da pressao: so a copia do modo Rust a enche
                        0x702050 => [0xff, u32::MAX, 0xffff, 0xff, 0],
                        _ => [u32::MAX; 5],
                    };
                    chamar(rotina, this, args, mascara)
                }
            }) as u32
        },
    );
    GRAVAR_CONDUCAO.store(cond, Relaxed);
    GRAVAR_702050.store(chute, Relaxed);
    GRAVAR_A7.store(a7, Relaxed);
    GRAVAR_716.store(gravar, Relaxed);
    PLACAR.registrar(igual && sequencia && r == rr, || format!("conduzir(modo {}) original={} rust={} {} chamadas: {}", modo as u8, r, rr, k, detalhe_fita()));
    r
}
