// ---------- %%LEVA%%a leva: a diretoria depois da partida: a confiança pelo resultado, a notícia da reação e a avaliação da federação (manager_manager.cpp) ----------

unsafe fn cqc8_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqc8_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cqc8_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqc8_b(a: usize) -> i8 {
    *(a as *const i8)
}

unsafe fn cqc8_u(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqc8_pw(a: usize, v: i16) {
    core::ptr::write_unaligned(a as *mut i16, v);
}

/// Uma constante de ponto flutuante do exe (lida no lugar, como a original faz).
unsafe fn cqc8_f(a: usize) -> f64 {
    core::ptr::read_unaligned(a as *const f64)
}

/// _ftol (0x945640): truncado para inteiro (a original usa só eax).
fn ftol_f64(x: f64) -> i32 {
    x as i64 as i32
}

unsafe fn cqc8_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// A linha da diretoria do clube no gerenciador dos técnicos: [this] + id × 0x49.
unsafe fn board_row(this: usize, clube: usize) -> usize {
    cqc8_d(this).wrapping_add((cqc8_i(clube) as usize).wrapping_mul(0x49))
}

/// 0x53b900 (cdecl, 1): o clube é seleção.
unsafe fn manager_manager_is_national_team(c: usize) -> u32 {
    let f: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    f(c)
}

/// O peso do jogo (o double em [esp+0x2c]) e o fator (o double em [esp+0x34], 0,5 de início), pela competição da
/// partida: sem competição 0,01; fora da competição do clube (0x48f150) 0,2; amistoso (0x5c6710) 0,67 se o mandante
/// é seleção, senão 0,05; não jogada (+0x40) 0,1; na liga do mandante 1 com o fator 0,06 × (+0x69 da liga) + 0,27;
/// senão 1 − |500 × (+0x69) − reputação| / 15000, × 1,25 na copa (+0x3a = 8) e pela fase (+0x30: 0x78 × 1,25, 0x82
/// × 1,5, 0x8c × 0,75, 0x96 e 0xa0 × 2).
unsafe fn match_weight(clube: usize, partida: usize, fator: &mut f64) -> f64 {
    let comp = cqc8_d(partida + 0x14);
    if comp == 0 {
        return cqc8_f(0x967898);
    }
    let na_comp: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x48f150usize);
    let obj = cqc8_d(cqc8_d(0xadadfc).wrapping_add((cqc8_i(partida) as usize).wrapping_mul(4)));
    if na_comp(obj, clube) == 0 {
        return cqc8_f(0x968918);
    }
    let amistoso: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5c6710usize);
    if amistoso(cqc8_d(0xae2c74), comp) != 0 {
        return if manager_manager_is_national_team(cqc8_d(partida + 0x1c)) != 0 { cqc8_f(0x968f80) } else { cqc8_f(0x967888) };
    }
    if cqc8_u(partida + 0x40) == 0 {
        return cqc8_f(0x967880);
    }
    if comp == cqc8_d(cqc8_d(partida + 0x1c) + 0x57) {
        *fator = cqc8_w(comp + 0x69) as i32 as f64 * cqc8_f(0x96d1e0) + cqc8_f(0x96d1d8);
        return cqc8_f(0x967890);
    }
    let x = (cqc8_w(comp + 0x69) as i32).wrapping_mul(500).wrapping_sub(cqc8_w(clube + 0x80) as i32);
    let mut p = cqc8_f(0x967890) - (x as f64).abs() * cqc8_f(0x968d78);
    if cqc8_u(partida + 0x3a) == 8 {
        p *= cqc8_f(0x968928);
    }
    match cqc8_w(partida + 0x30) as u16 {
        0x78 => p *= cqc8_f(0x968928),
        0x82 => p *= cqc8_f(0x968940),
        0x8c => p *= cqc8_f(0x969040),
        0x96 | 0xa0 => p = p + p,
        _ => {}
    }
    p
}

/// A nota de uma linha da escalação (0x49 bytes; `e` aponta o byte da nota em +0x1c0/+0x1c4): o byte que entra nas
/// médias (0 = fora). Com +0x4c do jogo, os 11 titulares valem 90; reservas (k ≥ 11) só com o byte de entrada
/// (e − 2 ≠ 0xff) e menos ele. `nota`, `troca` são os deslocamentos dos dois bytes tentados em ordem (o primeiro ≥ 0
/// vale; senão o +0x4d do jogo).
unsafe fn lineup_rating(jogo: usize, e: usize, k: i32, nota: usize, troca: usize, entrada: usize) -> Option<i8> {
    if cqc8_u(jogo + 0x4c) != 0 && k < 11 {
        return Some(0x5a);
    }
    let escolher = || {
        let v = cqc8_b(e + nota);
        if v > -1 {
            return v;
        }
        let v = cqc8_b(e + troca);
        if v > -1 { v } else { cqc8_b(jogo + 0x4d) }
    };
    if k < 11 {
        return Some(escolher());
    }
    let dl = cqc8_b(e + entrada);
    if dl == -1 {
        return None;
    }
    Some(escolher().wrapping_sub(dl))
}

/// { 0x695240 } (thiscall, ret 0x10, manager_manager.cpp): `MANAGER_MANAGER::update_board_confidence_after_match(
/// clube, partida, jogo, mandante)`, pela diretoria depois da partida (0x6951e0, um lado por vez). O ajuste começa
/// pela disputa (±20/30 contra o adversário ou pela seleção da partida +0x18), soma a diferença de reputação / 35 (sem
/// jogo) ou, com o jogo, a nota do time (média das notas por idade +0x1c4, as notas +0x127/+0x128, a diferença com o
/// adversário, a soma de +0x4d − nota; x87: (... + 0,1) × 67 − 470, × fator) mais a reputação média dos adversários
/// (pela nação, 0x53d980) contra a do clube / 40, até ±100. Pelo placar (+0x47/+0x48 ou +0x43/+0x44): vitória 0x800,
/// derrota 0x1000 (fora de amistoso, × 2 ou × 5 com goleada; abaixo de −50 com técnico da IA, o pedido à federação
/// 0x6a9e60 por sorteio), empate 0x400 (na copa, +15 e as fases 0x7b/0x7c/0x85/0x86/0x99/0x9a). O agregado (+0x4b/+0x4c,
/// +0x49/+0x4a, +0x43/+0x44) dá 0x100/0x200 e ±10/15/20; o clássico (0x40, mesma cidade ou rival a até 25 km) ou o
/// rival/confronto (0x80) multiplicam o peso (2,25 ou 1,5) e dão ±12/±5. Peso × 0,5 na seleção B, × 2 na principal,
/// × 2 ou 1,5 na copa (marca 0x400 da linha); o ajuste × peso, ÷ 3 ou ÷ 2 pelas marcas. As marcas 1/2/4/8/0x10/0x20
/// (pelo placar, pelo ajuste e pela confiança +6) vão ao texto (0x6a69c0) e à notícia da reação (0x6a5fa0, só técnico
/// humano). A história da mídia 0x3d (0x74b060, −250 em +0x16) por sorteio no clube dirigível; a 0x5c na liga com o
/// clube em perigo. A confiança: +6 += 0,7 ×, +0xa += 0,5 ×, +8 += 1,15 ×, +0xc += 0,2 × (metade com a marca
/// 0x7a7eb0 no negativo), entre 1 e 10000; pisos pela fase e pela força da competição; o protesto da torcida (0x6a5d10);
/// na liga ou no negativo, pensar em demitir (0x69aba0, 1 na derrota); senão sai da lista dos ultimatos (0x68d1f0).
unsafe extern "thiscall" fn update_board_confidence_after_match(this: usize, clube: usize, partida: usize, jogo: usize, mandante: u32) {
    let outro = if mandante != 0 { cqc8_d(partida + 0x20) } else { cqc8_d(partida + 0x1c) };
    let vis = mandante == 0;
    let mut fator: f64 = 0.5;
    let mut peso = match_weight(clube, partida, &mut fator);
    // a disputa (pela seleção da partida +0x18)
    let ev = cqc8_d(partida + 0x18);
    let cidade = |p: usize| p != 0 && cqc8_d(p + 0x38) != 0;
    let mut aj: i32 = 0;
    if manager_manager_is_national_team(clube) != 0 {
        if cidade(ev) && cqc8_d(cqc8_d(ev + 0x38) + 0x1f) == cqc8_d(clube + 0x53) {
            aj = -20;
        } else if cqc8_d(clube + 0x69) == ev {
            aj = -20;
        } else if cqc8_d(outro + 0x69) == ev {
            aj = 20;
        } else if cidade(ev) && cqc8_d(cqc8_d(ev + 0x38) + 0x1f) == cqc8_d(outro + 0x53) {
            aj = 20;
        }
    } else if cqc8_d(clube + 0x69) == ev {
        aj = -20;
    } else if cqc8_d(outro + 0x69) == ev {
        aj = if cqc8_d(outro + 0x53) != cqc8_d(clube + 0x53) { 30 } else { 20 };
    }
    let limitar = |q: i32| {
        let d = q as i16;
        if d < -100 { -100 } else if d > 100 { 100 } else { q }
    };
    let c: i32;
    if jogo == 0 {
        let x = (cqc8_w(clube + 0x80) as i32).wrapping_sub(cqc8_w(outro + 0x80) as i32);
        c = limitar(x / -35);
        aj = aj.wrapping_add(c);
    } else {
        let base = jogo + 0x625 * vis as usize;
        let (mut s1, mut s2, mut di) = (0i32, 0i32, 0u16);
        for k in 0..20i32 {
            let e = base + 0x1c4 + k as usize * 0x49;
            if cqc8_i(e - 0x34) == -1 {
                continue;
            }
            // a soma de +0x4d − nota, só quando a nota (e) vale
            let quatro_d = cqc8_w(jogo + 0x4d);
            let al = if cqc8_u(jogo + 0x4c) != 0 && k < 11 {
                0x5a
            } else if k < 11 {
                let v = cqc8_b(e);
                if v > -1 {
                    di = di.wrapping_add(quatro_d.wrapping_sub(v as i16) as u16);
                    v
                } else {
                    let v = cqc8_b(e - 4);
                    if v > -1 { v } else { cqc8_b(jogo + 0x4d) }
                }
            } else {
                let dl = cqc8_b(e - 2);
                if dl == -1 {
                    continue;
                }
                let v = cqc8_b(e);
                if v > -1 {
                    di = di.wrapping_add(quatro_d.wrapping_sub(v as i16) as u16);
                    v.wrapping_sub(dl)
                } else {
                    let v = cqc8_b(e - 4);
                    (if v > -1 { v } else { cqc8_b(jogo + 0x4d) }).wrapping_sub(dl)
                }
            };
            if al == 0 {
                continue;
            }
            s1 = s1.wrapping_add(al as i32);
            s2 = s2.wrapping_add((cqc8_u(e + 1) as i32).wrapping_mul(al as i32));
        }
        let media = if s1 != 0 { s2 as f64 / s1 as f64 } else { fator };
        let ob = jogo + 0x625 * (!vis) as usize;
        let b127 = cqc8_u(base + 0x127) as i32;
        let b128 = cqc8_u(base + 0x128) as i32;
        let o127 = cqc8_u(ob + 0x127) as i32;
        crate::comparison::branch("cqc_8 nota do jogo");
        let mut v = (b127 - 10) as f64 * cqc8_f(0x969020) + media;
        v += (b128 - 10) as f64 * cqc8_f(0x969020);
        v += (b127 - o127) as f64 * cqc8_f(0x968a98);
        v += (di as i16) as f64 * cqc8_f(0x96d1d0);
        v += cqc8_f(0x967880);
        v = v * cqc8_f(0x96d1c8) - cqc8_f(0x96d1c0); // cqc8: a nota do jogo (× 67 − 470)
        v *= fator;
        aj = aj.wrapping_add(ftol_f64(v));
        // a reputação média dos adversários escalados (pela nação do clube)
        let estrangeira: unsafe extern "C" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x53d980usize);
        let (mut s1, mut s2) = (0i32, 0i32);
        for k in 0..20i32 {
            let e = ob + 0x1c0 + k as usize * 0x49;
            let id = cqc8_i(e - 0x30);
            if id == -1 {
                continue;
            }
            let p = cqc8_d(0xae23bc).wrapping_add((id as usize).wrapping_mul(0x6e));
            let nacao = cqc8_d(clube + 0x53);
            let ficha = cqc8_d(p + 0x61);
            let rep = if cqc8_d(outro + 0x53) == nacao {
                cqc8_w(ficha + 0xb)
            } else if cqc8_d(p + 0x1a) == nacao {
                cqc8_w(ficha + 9)
            } else if estrangeira(p, nacao, 1) == 0 {
                cqc8_w(cqc8_d(p + 0x61) + 9)
            } else {
                cqc8_w(cqc8_d(p + 0x61) + 0xd)
            };
            let al = match lineup_rating(jogo, e, k, 4, 0, 2) {
                Some(a) => a,
                None => continue,
            };
            if al == 0 {
                continue;
            }
            s1 = s1.wrapping_add(al as i32);
            s2 = s2.wrapping_add((rep as i32).wrapping_mul(al as i32));
        }
        // ponytail: sem ninguém escalado a original soma o ponteiro da partida (o ecx que sobrou): reproduzido
        let m = if s1 != 0 { s2.wrapping_div(s1) } else { partida as i32 };
        let x = (cqc8_w(clube + 0x80) as i32).wrapping_sub((cqc8_w(outro + 0x80) as i32).wrapping_add(m) / 2);
        c = limitar(x / -40);
        aj = aj.wrapping_add(c);
    }
    // o placar: A e B (as notas de quem é o clube), C e D (o agregado), como a original guarda
    let p8 = |o: usize| cqc8_b(partida + o) as i32;
    let (a, b, cc, dd) = if cqc8_b(partida + 0x47) > -1 {
        if mandante != 0 { (p8(0x47), p8(0x48), p8(0x4b), p8(0x4c)) } else { (p8(0x48), p8(0x47), p8(0x4c), p8(0x4b)) }
    } else if mandante != 0 {
        (p8(0x43), p8(0x44), p8(0x4b), p8(0x4c))
    } else {
        (p8(0x44), p8(0x43), p8(0x4c), p8(0x4b))
    };
    let c16 = c as i16 as i32;
    let mut fl: u32;
    if a > b {
        fl = 0x800;
        crate::comparison::branch("cqc_8 vitoria");
        aj = aj.wrapping_add((c16 / 10 + 20).wrapping_mul(a - b)); // cqc8: a vitória
    } else if a < b {
        aj = aj.wrapping_add((20 - c16 / 10).wrapping_mul(a - b));
        fl = 0x1000;
        let comp = cqc8_d(partida + 0x14);
        let amistoso: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5c6710usize);
        let mut fim = false;
        if comp != 0 && amistoso(cqc8_d(0xae2c74), comp) == 0 {
            let dif = b - a;
            if dif > 10 {
                aj = aj.wrapping_mul(5);
                if (aj as i16) < 0x4e2 {
                    aj = 0x4e2;
                    fim = true;
                }
            } else if dif > 5 {
                aj = aj.wrapping_add(aj);
            }
        }
        if !fim && (aj as i16) < -0x32 && clube != 0 {
            let tec = cqc8_d(clube + 0xcf);
            let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
            if tec != 0 && cqc8_d(tec + 0x69) != 0 && humano(clube, 0, 1, 0, 0) == 0 {
                let tec = cqc8_d(clube + 0xcf);
                let r = cqc8_sortear(6);
                if cqc8_b(tec + 0x5d) < (r as u8).wrapping_add(6) as i8
                    && cqc8_b(cqc8_d(0xae2d14).wrapping_add((cqc8_i(clube) as usize).wrapping_mul(6)) + 4) < 0
                    && cqc8_sortear(cqc8_b(cqc8_d(tec + 0x69) + 0x1d) as i32 + 1) == 0
                {
                    let pedido: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x6a9e60usize);
                    pedido(this, clube, 1);
                }
            }
        }
    } else {
        fl = 0x400;
        if cqc8_u(partida + 0x3a) == 8 {
            let f = cqc8_w(partida + 0x32) as i32;
            if (0x41f..=0x426).contains(&f) && vis {
                aj = aj.wrapping_add(0xf);
            }
            let s = (cqc8_w(partida + 0x30) as i32).wrapping_sub(0x7b);
            if (0..=0x1f).contains(&s) && matches!(s, 0 | 1 | 10 | 11 | 30 | 31) {
                aj = aj.wrapping_add(if vis { a.wrapping_mul(10) } else { b.wrapping_mul(-5) });
            }
        }
    }
    // o agregado
    let g4b = cqc8_b(partida + 0x4b);
    if g4b != -1 {
        let g4c = cqc8_b(partida + 0x4c);
        // (marca, ajuste) pelo lado
        let mais = |fl: &mut u32, aj: &mut i32, n: i32| {
            *fl |= 0x200;
            *aj = aj.wrapping_add(n);
        };
        let menos = |fl: &mut u32, aj: &mut i32, n: i32| {
            *fl |= 0x100;
            *aj = aj.wrapping_sub(n);
        };
        if g4b > g4c {
            if mandante != 0 { mais(&mut fl, &mut aj, 20) } else { menos(&mut fl, &mut aj, 20) }
        } else if g4b < g4c {
            if mandante == 0 { mais(&mut fl, &mut aj, 20) } else { menos(&mut fl, &mut aj, 20) }
        } else {
            let (g49, g4a) = (cqc8_b(partida + 0x49), cqc8_b(partida + 0x4a));
            if g49 > g4a {
                if mandante != 0 { mais(&mut fl, &mut aj, 10) } else { menos(&mut fl, &mut aj, 10) }
            } else if g49 < g4a {
                if mandante == 0 { mais(&mut fl, &mut aj, 10) } else { menos(&mut fl, &mut aj, 10) }
            } else if cqc8_u(partida + 0x38) & 4 == 0 {
                let x = g4b as i32 - cqc8_b(partida + 0x43) as i32;
                let fora = x > cqc8_b(partida + 0x44) as i32;
                if fora == (mandante != 0) { menos(&mut fl, &mut aj, 15) } else { mais(&mut fl, &mut aj, 15) }
            }
        }
    }
    // o clássico e o rival
    let rival = cqc8_d(clube + 0xb3) == outro || cqc8_d(clube + 0xb7) == outro || cqc8_d(clube + 0xbb) == outro;
    let f1 = cqc8_d(clube + 0x69);
    let mesma = f1 != 0 && cqc8_d(outro + 0x69) != 0 && cqc8_d(f1 + 0x38) == cqc8_d(cqc8_d(outro + 0x69) + 0x38);
    if mesma {
        fl |= if rival { 0x40 } else { 0x80 };
    } else if rival {
        fl |= 0x80;
        if f1 != 0 {
            let e1 = cqc8_d(f1 + 0x38);
            let f2 = cqc8_d(outro + 0x69);
            if e1 != 0 && f2 != 0 {
                let e2 = cqc8_d(f2 + 0x38);
                if e2 != 0 {
                    let x = cqc8_f(e1 + 0x2b) - cqc8_f(e2 + 0x2b);
                    let y = cqc8_f(e1 + 0x23) - cqc8_f(e2 + 0x23);
                    if (y * y + x * x).sqrt() * cqc8_f(0x96d1b8) <= cqc8_f(0x968890) {
                        fl |= 0x40;
                    }
                }
            }
        }
    } else {
        let confronto: unsafe extern "C" fn(usize, usize) -> u8 = core::mem::transmute(0x4b02e0usize);
        if confronto(clube, outro) != 0 {
            fl |= 0x80;
        }
    }
    if fl & 0x40 != 0 {
        peso *= cqc8_f(0x968ec8);
        if fl & 0x800 != 0 {
            aj = aj.wrapping_add(0xc);
        } else if fl & 0x1000 != 0 {
            aj = aj.wrapping_sub(0xc);
        }
    } else if fl & 0x80 != 0 {
        peso *= cqc8_f(0x968940);
        if fl & 0x800 != 0 {
            aj = aj.wrapping_add(5);
        } else if fl & 0x1000 != 0 {
            aj = aj.wrapping_sub(5);
        }
    }
    let segunda: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x5409b0usize);
    let mut mult = if segunda(clube) != 0 {
        peso * cqc8_f(0x968968)
    } else {
        let s = manager_manager_is_national_team(clube);
        if s != 0 { peso + peso } else { peso }
    };
    if cqc8_u(partida + 0x3a) == 8 {
        if cqc8_u(board_row(this, clube) + 0xf) & 4 != 0 { // a palavra +0xe & 0x400
            mult = mult + mult;
            if (aj as i16) < 0 {
                aj = ftol_f64((aj as i16) as f64 * cqc8_f(0x968940));
            }
        } else {
            mult *= cqc8_f(0x968940);
        }
    }
    let mut dx = ftol_f64((aj as i16) as f64 * mult);
    if fl & 0x800 != 0 && (dx as i16) < 0 {
        dx = (dx as i16 as i32) / 3;
    }
    if fl & 0x1000 != 0 && (dx as i16) > 0 {
        dx = (dx as i16 as i32) / 3;
    }
    if fl & 0x200 != 0 && (dx as i16) < 0 {
        dx = (dx as i16 as i32) / 2;
    }
    if fl & 0x100 != 0 && (dx as i16) > 0 {
        dx = (dx as i16 as i32) / 2;
    }
    let aj = dx;
    let d16 = aj as i16;
    let conf6 = || cqc8_w(board_row(this, clube) + 6);
    if a + 5 < b {
        fl |= if clube == 0 || conf6() < 0x10b3 { 1 } else { 2 };
    } else if a - 5 > b {
        fl |= 8;
    } else if d16 < -0x78 && (clube == 0 || conf6() < 0x10b3) {
        fl |= 1;
    } else if d16 < -0x4b && (clube == 0 || conf6() < 0x1964) {
        fl |= 2;
    } else if d16 < -0x1e {
        fl |= 4;
    } else if d16 > 0x78 && clube != 0 && conf6() > 0x1964 {
        fl |= 8;
    } else if d16 > 0x4b {
        fl |= 0x20;
    } else if d16 > 0x1e {
        fl |= 0x10;
    }
    // o texto e a notícia da reação
    let marcas = cqc8_w(partida + 0x38) as u16;
    if marcas & 0x2810 == 0 {
        let texto: unsafe extern "thiscall" fn(usize, usize, usize, usize, i32, usize, i32, i32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(0x6a69c0usize);
        let humano: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7875e0usize);
        if texto(this, clube, outro, cqc8_d(partida + 0x14), fl as i16 as i32, 0xad9c64, -1, -1, 0, 0, 0, 0, 0, 0, 0, 0) != 0
            && humano(cqc8_d(0xb7431c), clube) != 0
        {
            let c6 = conf6();
            if c6 > 0x1a5e {
                if fl & 1 != 0 {
                    fl ^= 5;
                } else if fl & 2 != 0 {
                    fl ^= 6;
                }
            } else if c6 > 0x1482 {
                if fl & 1 != 0 {
                    fl ^= 3;
                }
            } else if c6 < 0x1482 && fl & 8 != 0 {
                fl ^= 0x28;
            }
            let lim = ((vis as u16).wrapping_mul(0x4e2)).wrapping_add(cqc8_w(outro + 0x80) as u16).wrapping_add(0x1f4) as i16;
            if fl & 7 == 0 || cqc8_w(clube + 0x80) >= lim || cqc8_b(partida + 0x43) != cqc8_b(partida + 0x44) {
                let reacao: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, i32, i32, i32, i32) = core::mem::transmute(0x6a5fa0usize);
                reacao(this, clube, outro, partida, fl, a, b, cc, dd);
            }
        }
    }
    // a história 0x3d da mídia
    let marcas = cqc8_w(partida + 0x38) as u16;
    let dirigivel: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(0x544820usize);
    let historia: unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, u32, i32) = core::mem::transmute(0x74b060usize);
    if fl & 0xa00 == 0 && marcas & 0x2810 == 0 && dirigivel(clube) != 0 {
        let s = aj as i16 as i32;
        let oito = || cqc8_w(board_row(this, clube) + 8) as i32;
        let r = cqc8_sortear(0x5a);
        let mut sai = false;
        if s < -0x5a - r {
            let r2 = cqc8_sortear(0x7d0);
            sai = oito().wrapping_add(s.wrapping_mul(10)) < 0xfa0 - r2;
        }
        if !sai {
            let r3 = cqc8_sortear(0x78);
            sai = s < -0x1e - r3 && oito().wrapping_add(s.wrapping_mul(10)) < 0x898;
        }
        if sai {
            historia(cqc8_d(0xb6b7c8), clube, 0x3d, 0, partida, 0, -1);
            let l = board_row(this, clube) + 0x16;
            cqc8_pw(l, cqc8_w(l).wrapping_add(-0xfa));
        }
    }
    // a história 0x5c: o clube em perigo na liga
    let id = cqc8_i(partida);
    if id > -1 && id == cqc8_i(clube + 0x1db) && id != cqc8_i(0x9cf6f8) && cqc8_u(partida + 0x3a) == 8
        && (cqc8_w(partida + 0x30) as i32 / 10) * 10 != 0x96
    {
        let co = cqc8_d(cqc8_d(0xadadfc).wrapping_add((id as usize).wrapping_mul(4)));
        if co != 0 && cqc8_d(co + 8) != 0 {
            let league_row: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
            let ln = league_row(cqc8_d(co + 8), clube);
            if fl & 0x100 == 0 {
                let n = cqc8_u(ln + 0x72);
                if n < 9 && n != 0 && cqc8_u(ln + 0x74) == 1 && (aj as i16 as i32) > (n as i32 * n as i32) / 2 + 0x32 {
                    historia(cqc8_d(0xb6b7c8), clube, 0x5c, 0, partida, 0, -1);
                }
            }
        }
    }
    let mut aj = aj;
    if (aj as i16) < 0 {
        let tec = cqc8_d(clube + 0xcf);
        let marca: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x7a7eb0usize);
        if tec != 0 && marca(cqc8_d(0xb7434c), tec) != 0 {
            aj = ftol_f64((aj as i16) as f64 * cqc8_f(0x968968));
        }
    }
    // a confiança
    let v = (aj as i16) as f64;
    crate::comparison::branch("cqc_8 confianca");
    for (o, k) in [(6usize, 0x968e40usize), (0xa, 0x968968), (8, 0x968980), (0xc, 0x968918)] {
        let l = board_row(this, clube) + o;
        let t = ftol_f64(v * cqc8_f(k)); // cqc8: a confiança pelo ajuste
        cqc8_pw(l, cqc8_w(l).wrapping_add(t as i16));
    }
    for o in [6usize, 0xa, 8, 0xc] {
        let l = board_row(this, clube) + o;
        let x = cqc8_w(l);
        if x > 0x2710 {
            cqc8_pw(l, 0x2710);
        } else if x < 1 {
            cqc8_pw(l, 1);
        }
    }
    let piso = |o: usize, v: i16| {
        let l = board_row(this, clube) + o;
        if cqc8_w(l) < v {
            cqc8_pw(l, v);
        }
    };
    let comp = cqc8_d(partida + 0x14);
    if (aj as i16) > 0 && cqc8_d(clube + 0xcf) != 0 {
        if comp != 0 {
            let n = cqc8_w(comp + 0x69);
            if n > 5 {
                let rep = cqc8_w(clube + 0x80);
                let fase = cqc8_w(partida + 0x30) as u16;
                if (n as i32) * 500 > rep as i32 {
                    if fase == 0x82 || fase == 0x78 {
                        piso(6, 0x1964);
                        piso(8, 0x1964);
                    } else if fase == 0x96 {
                        piso(6, 0x2134);
                        piso(8, 0x2134);
                    } else if comp != cqc8_d(clube + 0x57) && (rep < 0xea6 || cqc8_u(partida + 0x3a) == 8) {
                        piso(6, 0x10b3);
                        piso(8, 0x1964);
                    }
                } else if fase == 0x82 || fase == 0x96 {
                    piso(6, 0x1964);
                    piso(8, 0x1964);
                } else if comp != cqc8_d(clube + 0x57) && (fase == 0x78 || cqc8_u(partida + 0x3a) == 8) {
                    piso(6, 0x10b3);
                    piso(8, 0x10b3);
                }
            }
        }
    } else if (aj as i16) < 0 && comp != 0 && cqc8_w(partida + 0x30) == 0x96 && cqc8_u(partida + 0x3a) == 5 {
        let l = board_row(this, clube);
        if cqc8_u(l + 0xe) & 4 != 0 {
            if cqc8_w(l + 6) > 0x10b3 {
                cqc8_pw(l + 6, 0x10b3);
            } else if cqc8_w(l + 8) > 0xdac {
                cqc8_pw(l + 6, 0xdac);
            }
            let x = cqc8_w(l + 8);
            if x > 0x10b3 {
                cqc8_pw(l + 8, 0x10b3);
            } else if x > 0xdac {
                cqc8_pw(l + 8, 0xdac);
            }
        }
    }
    if cqc8_d(clube + 0xcf) != 0 && (aj as i16) < -0x4b && cqc8_w(board_row(this, clube) + 8) < 0xbb8 {
        let protesto: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x6a5d10usize);
        protesto(this, clube);
    }
    if cqc8_d(partida + 0x14) == cqc8_d(clube + 0x57) || (aj as i16) < 0 {
        if cqc8_w(partida + 0x30) != 0xa0 {
            let demitir: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x69aba0usize);
            demitir(this, clube, (a < b) as u32);
        }
        return;
    }
    let lista = cqc8_d(this + 0x10);
    if cqc8_w(lista) == 0 {
        return;
    }
    core::ptr::write_unaligned((lista + 0xa) as *mut u32, cqc8_d(lista + 2) as u32);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let tirar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x68d1f0usize);
    let mut i = 0i32;
    while i < cqc8_w(cqc8_d(this + 0x10)) as i32 {
        let e = proximo(cqc8_d(this + 0x10));
        if e != 0 && cqc8_i(clube) == cqc8_i(e) {
            tirar(cqc8_d(this + 0x10), e);
            return;
        }
        i += 1;
    }
}

// ponytail: chama o pedido à federação (0x6a9e60), a notícia da reação (0x6a5fa0), as histórias da mídia (0x74b060), o
// protesto, a demissão (0x69aba0) e tira item da lista dos ultimatos (0x68d1f0): alocam, liberam e sorteiam conforme o
// caminho; não roda duas vezes. Só a original; a cópia leva os ramos e se prova pela carreira e pelas mutações.
unsafe extern "thiscall" fn compare_update_board_confidence_after_match(this: usize, clube: usize, partida: usize, jogo: usize, mandante: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32)>(%%I0%%)(this, clube, partida, jogo, mandante)
}

/// { 0x6a5fa0 } (thiscall, ret 0x20, manager_manager.cpp, quadro SEH sem destrutor no caminho normal):
/// `MANAGER_MANAGER::post_board_match_reaction_news(clube, adversário, partida, marcas, a, b, c, d)`: com clube,
/// adversário, partida e competição (+0x14), a notícia 0x138c (0x77bbe0) com as marcas, os dois ids, os placares a e
/// b, o agregado c e d e +8 da partida; os até 3 jogadores que a diretoria cita (0x6aaee0, com a derrota a > b), um
/// sorteio(4), a derrota e as 5 marcas deles; o registro da partida pelo clube (0x7e4750) gravado na notícia
/// (0x7e4de0); entregue ao técnico do clube (0x7861e0).
#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn post_board_match_reaction_news(this: usize, clube: usize, outro: usize, partida: usize, marcas: u32, a: i32, b: i32, c: i32, d: i32) {
    if clube == 0 || outro == 0 || partida == 0 || cqc8_d(partida + 0x14) == 0 {
        return;
    }
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let data_padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let citados: unsafe extern "thiscall" fn(usize, usize, usize, *mut i32, usize, *mut u8, *mut u8, *mut u8, *mut u8, *mut u8, *mut u8) -> u8 =
        core::mem::transmute(0x6aaee0usize);
    let registro: unsafe extern "C" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7e4750usize);
    let no_registro: unsafe extern "C" fn(usize, usize, *mut u8) -> u32 = core::mem::transmute(0x7e4de0usize);
    let ao_tecnico: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    crate::comparison::branch("cqc_8 noticia da reacao");
    let mut noticia = [0u32; 0x3a]; // a notícia local (0xe0 bytes na original)
    let n = noticia.as_mut_ptr() as usize;
    let mut reg = [0u8; 0x20]; // o registro da partida
    let r = reg.as_mut_ptr() as usize;
    // a derrota e as 5 marcas dos citados (os bytes +0xe, +0x11, +0x13, +0x12, +0xf e +0x10 da original)
    let (mut m1, mut m2, mut m3, mut m4, mut m5) = (0u8, 0u8, 0u8, 0u8, 0u8);
    construir(n, 0x138c, 0);
    let mut k: u8 = 0;
    data_padrao(r);
    let mut ids = [-1i32; 3];
    for v in [marcas as i16 as i32 as u32, cqc8_d(clube) as u32, cqc8_d(outro) as u32, a as u32, b as u32, c as u32, d as u32, cqc8_d(partida + 8) as u32] {
        gravar(n, k as i8, v);
        k = k.wrapping_add(1);
    }
    let mut derrota = (a > b) as u8;
    citados(this, partida, clube, ids.as_mut_ptr(), cqc8_d(partida + 8), &mut derrota, &mut m1, &mut m2, &mut m3, &mut m4, &mut m5);
    for v in ids {
        gravar(n, k as i8, v as u32);
        k = k.wrapping_add(1);
    }
    let s = cqc8_sortear(4);
    gravar(n, k as i8, s as u32);
    k = k.wrapping_add(1);
    for v in [derrota, m1, m2, m3, m4, m5] {
        gravar(n, k as i8, v as i8 as i32 as u32);
        k = k.wrapping_add(1);
    }
    registro(partida, clube, r);
    no_registro(r, n, &mut k);
    ao_tecnico(cqc8_d(0xb7431c), n, clube);
}

// ponytail: a notícia vai ao técnico (aloca): só a original; a cópia leva o ramo.
#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn compare_post_board_match_reaction_news(this: usize, clube: usize, outro: usize, partida: usize, marcas: u32, a: i32, b: i32, c: i32, d: i32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, i32, i32, i32, i32)>(%%I1%%)(this, clube, outro, partida, marcas, a, b, c, d)
}

/// { 0x6a5a60 } (thiscall, ret 0xc, manager_manager.cpp, quadro SEH sem destrutor no caminho normal):
/// `MANAGER_MANAGER::post_fa_performance_news(competição, seleção, nota)`: a partir da data do começo do jogo
/// ([0xdd6ec8]), a notícia 0x1781 (0x77bbe0) com o id da seleção, o id da competição (*competição) e a nota da
/// federação, pelo alcance (0x786090 com a seleção e 0, 0, 0, 0).
unsafe extern "thiscall" fn post_fa_performance_news(_this: usize, comp: usize, selecao: usize, nota: u32) {
    let dia = cqc8_w(0xdd6ec8);
    let ano = cqc8_w(0xdd6eca) as u16;
    let hoje = cqc8_w(0xae2c92) as u16;
    if ano > hoje || (ano == hoje && dia > cqc8_w(0xae2c90)) {
        return;
    }
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let por_alcance: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x786090usize);
    crate::comparison::branch("cqc_8 avaliacao da federacao");
    let mut local = [0u32; 0x3a]; // a notícia local (0xe8 bytes)
    let n = local.as_mut_ptr() as usize;
    construir(n, 0x1781, 0);
    gravar(n, 0, cqc8_d(selecao) as u32);
    gravar(n, 1, cqc8_d(comp) as u32);
    gravar(n, 2, nota as i8 as i32 as u32);
    por_alcance(cqc8_d(0xb7431c), n, selecao, 0, 0, 0, 0);
}

// ponytail: a notícia vai pelo alcance (aloca): só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn compare_post_fa_performance_news(this: usize, comp: usize, selecao: usize, nota: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(%%I2%%)(this, comp, selecao, nota)
}
