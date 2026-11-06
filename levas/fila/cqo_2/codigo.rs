// ---------- %%LEVA%%a leva: o público de um jogo (stadium.cpp) ----------

unsafe fn cqo2_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqo2_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cqo2_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqo2_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O sorteio do jogo (0x90cfc0, cdecl): 0..n-1.
unsafe fn cqo2_sortear(n: u32) -> i32 {
    let f: unsafe extern "C" fn(u32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// A cidade (registro de 0x38 bytes em [0xae23ac]) do estádio `est` (+0x38); sem estádio ou sem cidade, uma sorteada
/// entre as [0xae2354].
unsafe fn cqo2_cidade(est: usize) -> usize {
    if est != 0 && cqo2_d(est + 0x38) != 0 {
        return cqo2_d(est + 0x38);
    }
    let r = cqo2_sortear(cqo2_d(0xae2354) as u32);
    cqo2_d(0xae23ac).wrapping_add((r as usize).wrapping_mul(0x38))
}

/// O piso do fator de cada lado na competição de tipo `t` (+0x3a da partida), pelos limites +0x30/+0x32 da partida e
/// pela reputação (/500) `rep` do clube do lado (0x109a = 4250, 0x1676 = 5750).
unsafe fn cqo2_piso(jogo: usize, lado: u8, f: &mut [f64; 2]) {
    let t = cqo2_b(jogo + 0x3a);
    let rep = |l: u8| cqo2_w(cqo2_d(jogo + if l == 0 { 0x1c } else { 0x20 }) + 0x80);
    let k = lado as usize;
    let piso = |f: &mut f64, m: f64| {
        if *f < m {
            *f = m;
        }
    };
    let alto = |lim: i16| -> bool {
        // os lados além do 1 contam como alto (como a original)
        lado > 1 || rep(lado) >= lim
    };
    if t == 8 {
        let a = cqo2_w(jogo + 0x30) as u16;
        if a == 0x96 {
            if alto(0x1676) {
                piso(&mut f[k], f64_em(0x9690a8));
            } else {
                piso(&mut f[k], f64_em(0x968e38));
            }
        } else if a == 0x82 && f[k] < f64_em(0x968e18) {
            if alto(0x1676) {
                piso(&mut f[k], f64_em(0x968ec8));
            } else {
                piso(&mut f[k], f64_em(0x968930));
            }
        } else if lado > 1 || rep(lado) >= 0x1676 {
            piso(&mut f[k], f64_em(0x9689b8));
        } else if rep(lado) >= 0x109a {
            piso(&mut f[k], f64_em(0x968928));
        } else {
            piso(&mut f[k], f64_em(0x968f98));
        }
    }
    if t == 6 {
        let a = cqo2_w(jogo + 0x30) as u16;
        if a == 0x96 {
            if alto(0x109a) {
                piso(&mut f[k], f64_em(0x9690a8));
            } else {
                piso(&mut f[k], f64_em(0x968e38));
            }
        } else if a == 0x82 && f[k] < f64_em(0x968e18) {
            f[k] = f64_em(0x968e18);
        } else {
            piso(&mut f[k], f64_em(0x968928));
        }
    } else if cqo2_w(jogo + 0x30) as u16 == 0x96 && t == 5 {
        if alto(0x109a) {
            piso(&mut f[k], f64_em(0x9690a8));
        } else {
            piso(&mut f[k], f64_em(0x968e38));
        }
    } else if cqo2_w(jogo + 0x32) as u16 == 0xa0 && t == 5 {
        if lado == 0 {
            if rep(0) < 0x109a && f[0] < f64_em(0x96fe20) {
                f[0] = f64_em(0x96fe20);
            } else {
                piso(&mut f[0], f64_em(0x968e18));
            }
        } else {
            piso(&mut f[k], f64_em(0x968e18));
        }
    } else if t == 10 && f[k] < f64_em(0x968928) {
        f[k] = f64_em(0x968928);
    } else if t == 9 && f[k] < f64_em(0x9689b0) {
        f[k] = f64_em(0x9689b0);
    } else {
        piso(&mut f[k], f64_em(0x967880));
    }
}

/// { 0x85cba0 } (thiscall, ret 8, stadium.cpp, `Stadium::calculate_match_attendance(partida, saida)`; this =
/// [0xdd7ef4]; pelo motor 0x6b7f20): o público do jogo. Sem partida, [0xa835ac] (15 mil). R = reputação do clube /500
/// (no mínimo 1). Para cada lado, o fator começa no número de clubes da competição (+0x69, no mínimo 1; sem competição
/// 1), +2 nas competições de tipo 6, 8 e 10; nas ligas (tabela [0xadadfc], fora de amistoso 0x5c6710 e das nações de
/// 0x7e6f20, sem +0x43 = 2) mais a diferença (+0x34 da partida − +0x3c da liga, entre −6 e 5). O fator do mandante vezes
/// R do visitante / R dele (se ele tem R < 15 ou abaixo do outro), ×1,1 com o outro acima de 17, ×1,25 com o outro
/// acima de 12 e ele abaixo de 14; dividido pelo número de clubes da liga do clube (+0x57 → +0x69), sem ela ×0,2 (tipo
/// 10) ou ×0,05. O do visitante, o mesmo com os papéis trocados. Pisos por tipo de competição (cqo2_piso: copas 8, 6 e
/// 5 com os limites +0x30/+0x32 e a reputação; 10 1,25; 9 0,8; senão 0,1). Com estádio fora de campo neutro: as
/// cidades do estádio e dos dois clubes (sorteadas sem cidade), o fator da distância (0x85d8a0, no mínimo 0,15); o
/// mandante leva a média (+0x73) × fator × distância, o visitante (+0x73 + +0x77) × fator × distância × 0,5; na
/// liga do mandante, pela posição (0x4a4850 na tabela da liga): +50/100/150 por clube nos primeiros, +25/50 na
/// briga, −25/−10 nos de baixo, e pela sequência (+0x72/+0x73/+0x74); no estádio da cidade de um clube, o público dele
/// entre o mínimo (+0x77) e o máximo (+0x7b). Sem estádio ou neutro: média × fator. Clássico (+0xb3/+0xb7/+0xbb):
/// ×1,5 (mesma cidade ×1,35); visitante rival ×1,35; fora de campo neutro com +0x3b != 0xff ×1,05 e ×0,85. Na liga
/// com +0x6d: o mínimo pelo número de clubes da liga (e o máximo com até 4 clubes). Estádio com menos de 1.500
/// lugares passa a R × 1.000 (no mínimo 1.500). Limites pela lotação (o visitante até max(5%, min(lotação − média
/// do visitante × fator do mandante, 15%)) e o mandante o resto; neutro metade), sem estádio 40 mil e 25 mil; sem
/// público, 1 a 50 e 1 a 15. Total = mandante + visitante − sorteio(50) (negativo: 45 a 194). Grava em `saida` o
/// total, o mandante e o visitante e devolve o total.
unsafe extern "thiscall" fn publico_do_jogo(this: usize, jogo: usize, saida: *mut i32) -> i32 {
    if jogo == 0 {
        return cqo2_i(0xa835ac);
    }
    let casa = cqo2_d(jogo + 0x1c);
    let fora = cqo2_d(jogo + 0x20);
    let r500 = |c: usize| -> i8 { (cqo2_w(c + 0x80) as i32 / 500) as i8 };
    let mut rc = r500(casa);
    let mut rf = r500(fora);
    if rc < 1 {
        rc = 1;
    }
    if rf < 1 {
        rf = 1;
    }
    let mut f = [0f64; 2];
    let mut lado: u8 = 0;
    while lado < 2 {
        let k = lado as usize;
        let comp = cqo2_d(jogo + 0x14);
        if comp != 0 {
            f[k] = cqo2_w(comp + 0x69) as i32 as f64;
            if f[k] < f64_em(0x967890) {
                f[k] = 1.0;
            }
        } else {
            f[k] = 1.0;
        }
        let t = cqo2_b(jogo + 0x3a);
        if t == 8 || t == 10 || t == 6 {
            f[k] += f64_em(0x968950);
        }
        let amistoso: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5c6710usize);
        let mut ajuste = f64_em(0x9689a0);
        if amistoso(cqo2_d(0xae2c74), comp) == 0 && cqo2_d(0xadadfc) != 0 && cqo2_d(cqo2_d(0xadadfc) + cqo2_d(comp) * 4) != 0 && cqo2_d(0xb74434) != 0 {
            let grupo: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(0x7e6f20usize);
            if grupo(cqo2_d(jogo + 0x14)) == 0 {
                let liga = cqo2_d(cqo2_d(0xadadfc) + cqo2_d(cqo2_d(jogo + 0x14)) * 4);
                if cqo2_b(liga + 0x43) != 2 {
                    let x = (cqo2_b(jogo + 0x34).wrapping_sub(cqo2_b(liga + 0x3c)) as i8) as i32 as f64;
                    ajuste = if x < f64_em(0x96fe28) {
                        f64_em(0x96fe28)
                    } else if x <= f64_em(0x9688a0) {
                        x
                    } else {
                        f64_em(0x9688a0)
                    };
                }
            }
        }
        let v = ajuste + f[k];
        f[k] = v;
        if lado <= 1 {
            // o mandante (lado 0) contra o visitante; o visitante com os papéis trocados
            let (eu, outro, clube) = if lado == 0 { (rc, rf, casa) } else { (rf, rc, fora) };
            if eu < 0xf || eu < outro {
                f[k] = outro as i32 as f64 / eu as i32 as f64 * v;
            }
            if outro > 0x11 {
                f[k] *= f64_em(0x9689c0);
            }
            if outro > 0xc && eu < 0xe {
                f[k] *= f64_em(0x968928);
            }
            let _ = clube;
            let lg = cqo2_d(cqo2_d(jogo + if lado == 0 { 0x1c } else { 0x20 }) + 0x57);
            if lg != 0 && cqo2_w(lg + 0x69) != 0 {
                f[k] /= cqo2_w(lg + 0x69) as i32 as f64;
            } else if cqo2_b(jogo + 0x3a) == 0xa {
                f[k] *= f64_em(0x968918);
            } else {
                f[k] *= f64_em(0x967888); // cqo2: o fator sem a liga do clube
            }
        }
        cqo2_piso(jogo, lado, &mut f);
        lado += 1;
    }
    let mut pc: i32; // o público do mandante ([esp+0x14] / esi)
    let mut pf: i32; // o do visitante ([esp+0x18])
    let est = cqo2_d(jogo + 0x18);
    if est != 0 && cqo2_b(jogo + 0x3c) == 0 {
        let c_est = cqo2_cidade(est);
        let c_casa = cqo2_cidade(cqo2_d(casa + 0x69));
        let c_fora = cqo2_cidade(cqo2_d(fora + 0x69));
        let fator: unsafe extern "thiscall" fn(usize, usize, usize, usize, f64) -> f64 = core::mem::transmute(0x85d8a0usize);
        let mut d = fator(this, jogo, c_est, c_casa, f[0]);
        if d < f64_em(0x968f20) {
            d = f64_em(0x968f20);
        }
        pc = ftol(cqo2_i(casa + 0x73) as f64 * f[0] * d);
        let mut d = fator(this, jogo, c_est, c_fora, f[1]);
        if d < f64_em(0x968f20) {
            d = f64_em(0x968f20);
        }
        pf = ftol(cqo2_i(fora + 0x77).wrapping_add(cqo2_i(fora + 0x73)) as f64 * f[1] * d * f64_em(0x968968));
        let comp = cqo2_d(jogo + 0x14);
        if comp != 0 && casa != 0 && cqo2_d(casa + 0x57) == comp {
            let liga = cqo2_d(cqo2_d(0xadadfc) + cqo2_d(comp) * 4);
            if liga != 0 && cqo2_d(liga + 8) != 0 {
                let busca: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
                let p = busca(cqo2_d(liga + 8), casa);
                if p != 0 {
                    let n = cqo2_w(comp + 0x69) as i32;
                    let v = cqo2_i(p + 0x19);
                    if v > 1 {
                        crate::comparacao::ramo("cqo_2 posicao de cima");
                        pc = pc.wrapping_add(if v < 4 || n < 0xc {
                            n * 50
                        } else if v < 7 {
                            n * 100 // cqo2: os primeiros
                        } else {
                            n * 150
                        });
                    } else if cqo2_i(p + 0x41) > 2 {
                        pc = pc.wrapping_add(if cqo2_i(p + 0x41) >= 4 && n >= 0xc { n * 50 } else { n * 25 });
                    } else if cqo2_i(p + 0x2d) > 2 {
                        pc = pc.wrapping_sub(n * 25);
                    } else if cqo2_i(p + 0x55) > 3 {
                        pc = pc.wrapping_sub(n * 10);
                    }
                    let dl = cqo2_b(p + 0x74);
                    if dl != cqo2_b(p + 0x75) && cqo2_b(p + 4) > 4 {
                        let cl = cqo2_b(p + 0x73);
                        let bl = cqo2_b(p + 0x72);
                        let mut ax = (bl as i16).wrapping_sub(cl as i16);
                        if bl < 4 && ax <= 0 {
                            ax = if cl == 1 {
                                4
                            } else if cl < 4 {
                                2
                            } else {
                                ftol(ax as i32 as f64 * f64_em(0x9689e0)) as i16
                            };
                        }
                        let m = if bl < 4 && dl == 1 { 25 } else { 15 };
                        pc = pc.wrapping_add(n.wrapping_mul(ax as i32).wrapping_mul(m));
                    }
                }
            }
        }
        if c_est == c_casa {
            let (mx, mn) = (cqo2_i(casa + 0x7b), cqo2_i(casa + 0x77));
            if pc > mx {
                pc = mx;
            } else if pc < mn {
                pc = mn;
            }
        }
        if c_est == c_fora {
            let (mx, mn) = (cqo2_i(fora + 0x7b), cqo2_i(fora + 0x77));
            if pf > mx {
                pf = mx;
            } else if pf < mn {
                pf = mn;
            }
        }
    } else {
        pc = ftol(cqo2_i(casa + 0x73) as f64 * f[0]);
        pf = ftol(cqo2_i(fora + 0x73) as f64 * f[1]);
    }
    // os clássicos
    let (casa, fora) = (cqo2_d(jogo + 0x1c), cqo2_d(jogo + 0x20));
    let rival = |a: usize, b: usize| cqo2_d(a + 0xb3) == b || cqo2_d(a + 0xb7) == b || cqo2_d(a + 0xbb) == b;
    if rival(casa, fora) {
        crate::comparacao::ramo("cqo_2 classico");
        pc = ftol(pc as f64 * f64_em(0x968940)); // cqo2: o clássico
    } else if cqo2_d(casa + 0x69) != 0 && cqo2_d(fora + 0x69) != 0 && cqo2_d(cqo2_d(casa + 0x69) + 0x38) == cqo2_d(cqo2_d(fora + 0x69) + 0x38) {
        pc = ftol(pc as f64 * f64_em(0x969588));
    }
    if rival(fora, casa) {
        pf = ftol(pf as f64 * f64_em(0x969588));
    }
    if cqo2_b(jogo + 0x3c) == 0 && cqo2_b(jogo + 0x3b) != 0xff {
        pc = ftol(pc as f64 * f64_em(0x9689b8));
        pf = ftol(pf as f64 * f64_em(0x969520));
    }
    let comp = cqo2_d(jogo + 0x14);
    if comp != 0 && casa != 0 && cqo2_d(casa + 0x57) == comp && cqo2_b(casa + 0x6d) != 0 {
        let n = cqo2_w(comp + 0x69);
        if n > 0xc {
            let m = (n.wrapping_sub(0xc) as i32 + 3) * 2000;
            if pc < m {
                pc = m;
            }
        } else if n > 8 {
            let m = (n.wrapping_sub(8) as i32 + 4) * 750;
            if pc < m {
                pc = m;
            }
        } else if n > 4 {
            let m = (n.wrapping_sub(4) as i32 + 2) * 500;
            if pc < m {
                pc = m;
            }
        } else {
            let n = n as i32;
            if pc < n * 250 {
                pc = n * 250;
            } else if pc > n * 1500 {
                pc = n * 1500 - ftol((pc - n * 1500) as f64 * f64_em(0x968f30));
            }
        }
    }
    let est = cqo2_d(jogo + 0x18);
    if est != 0 && cqo2_i(est + 0x3c) < 0x5dc {
        core::ptr::write_unaligned((est + 0x3c) as *mut i32, rc as i32 * 1000);
        if cqo2_i(est + 0x3c) < 0x5dc {
            core::ptr::write_unaligned((est + 0x3c) as *mut i32, 0x5dc);
        }
    }
    if pf.wrapping_add(pc) != 0 {
        let est = cqo2_d(jogo + 0x18);
        if est != 0 {
            let lot = cqo2_i(est + 0x3c);
            let vis = if cqo2_b(jogo + 0x3c) == 0 {
                let a = lot as f64;
                let x = a - cqo2_i(cqo2_d(jogo + 0x20) + 0x73) as f64 * f[0];
                let y = a * f64_em(0x968f20);
                let z = if x < y { x } else { y };
                let w = a * f64_em(0x967888);
                ftol(if w > z { w } else { z })
            } else {
                lot / 2
            };
            let m = lot.wrapping_sub(vis);
            if pc > m {
                pc = m;
            }
            if pf > vis {
                pf = vis;
            }
        } else {
            if pc > 40000 {
                pc = 40000;
            }
            if pf > 25000 {
                pf = 25000;
            }
        }
    } else {
        pc = cqo2_sortear(0x32) + 1;
        pf = cqo2_sortear(0xf) + 1;
    }
    let r = cqo2_sortear(0x32); // cqo2: o sorteio do total
    let mut total = pf.wrapping_add(pc).wrapping_sub(r);
    if total < 0 {
        total = cqo2_sortear(0x96) + 0x2d;
    }
    if !saida.is_null() {
        *saida = total;
        *saida.add(1) = pc;
        *saida.add(2) = pf;
    }
    total
}

/// Lado a lado (a cópia sem contar os sorteios) pelo estado do sorteador, pela saída, pelos lugares do estádio (que
/// ela pode subir para 1.500) e pela devolução.
unsafe extern "thiscall" fn comparar_publico_do_jogo(this: usize, jogo: usize, saida: *mut i32) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, *mut i32) -> i32>(%%I0%%);
    if jogo == 0 {
        return o(this, jogo, saida);
    }
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    if !saida.is_null() {
        reg.push((saida as usize, 12));
    }
    if cqo2_d(jogo + 0x18) != 0 {
        reg.push((cqo2_d(jogo + 0x18) + 0x3c, 4));
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, jogo, saida), || crate::aleatorio::sem_contar(|| publico_do_jogo(this, jogo, saida)));
    PLACAR.registrar(igual && r == n, || format!("publico_do_jogo(comp {:x}, tipo {}): original {} rust {} {:x?} x {:x?}", cqo2_d(jogo), cqo2_b(jogo + 0x3a), r, n, mo, mr));
    r
}
