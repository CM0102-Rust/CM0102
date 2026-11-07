use super::*;

// ---------- 266a leva: a diretoria depois da partida: a confiança pelo resultado, a notícia da reação e a avaliação da federação (manager_manager.cpp) ----------


pub(crate) unsafe fn cqc8_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}



pub(crate) unsafe fn cqc8_u(a: usize) -> u8 {
    *(a as *const u8)
}



/// _ftol (0x945640): truncado para inteiro (a original usa só eax).
pub(crate) fn cqc8_ftol(x: f64) -> i32 {
    x as i64 as i32
}

pub(crate) unsafe fn cqc8_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    f(n)
}

/// A linha da diretoria do clube no gerenciador dos técnicos: [this] + id × 0x49.
pub(crate) unsafe fn cqc8_linha(this: usize, clube: usize) -> usize {
    read_usize(this).wrapping_add((cqc8_i(clube) as usize).wrapping_mul(0x49))
}

/// 0x53b900 (cdecl, 1): o clube é seleção.
pub(crate) unsafe fn cqc8_selecao(c: usize) -> u32 {
    let f: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    f(c)
}

/// O peso do jogo (o double em [esp+0x2c]) e o fator (o double em [esp+0x34], 0,5 de início), pela competição da
/// partida: sem competição 0,01; fora da competição do clube (0x48f150) 0,2; amistoso (0x5c6710) 0,67 se o mandante
/// é seleção, senão 0,05; não jogada (+0x40) 0,1; na liga do mandante 1 com o fator 0,06 × (+0x69 da liga) + 0,27;
/// senão 1 − |500 × (+0x69) − reputação| / 15000, × 1,25 na copa (+0x3a = 8) e pela fase (+0x30: 0x78 × 1,25, 0x82
/// × 1,5, 0x8c × 0,75, 0x96 e 0xa0 × 2).
pub(crate) unsafe fn cqc8_peso(clube: usize, partida: usize, fator: &mut f64) -> f64 {
    let comp = read_usize(partida + 0x14);
    if comp == 0 {
        return read_f64(0x967898);
    }
    let na_comp: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_COMP_IS_CLUB_STILL_IN_CONTENTION);
    let obj = read_usize(read_usize(0xadadfc).wrapping_add((cqc8_i(partida) as usize).wrapping_mul(4)));
    if na_comp(obj, clube) == 0 {
        return read_f64(0x968918);
    }
    let amistoso: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_FRIENDLY_IS_FRIENDLY_COMPETITION);
    if amistoso(read_usize(0xae2c74), comp) != 0 {
        return if cqc8_selecao(read_usize(partida + 0x1c)) != 0 { read_f64(0x968f80) } else { read_f64(0x967888) };
    }
    if cqc8_u(partida + 0x40) == 0 {
        return read_f64(0x967880);
    }
    if comp == read_usize(read_usize(partida + 0x1c) + 0x57) {
        *fator = read_i16(comp + 0x69) as i32 as f64 * read_f64(0x96d1e0) + read_f64(0x96d1d8);
        return read_f64(0x967890);
    }
    let x = (read_i16(comp + 0x69) as i32).wrapping_mul(500).wrapping_sub(read_i16(clube + 0x80) as i32);
    let mut p = read_f64(0x967890) - (x as f64).abs() * read_f64(0x968d78);
    if cqc8_u(partida + 0x3a) == 8 {
        p *= read_f64(0x968928);
    }
    match read_i16(partida + 0x30) as u16 {
        0x78 => p *= read_f64(0x968928),
        0x82 => p *= read_f64(0x968940),
        0x8c => p *= read_f64(0x969040),
        0x96 | 0xa0 => p = p + p,
        _ => {}
    }
    p
}

/// A nota de uma linha da escalação (0x49 bytes; `e` aponta o byte da nota em +0x1c0/+0x1c4): o byte que entra nas
/// médias (0 = fora). Com +0x4c do jogo, os 11 titulares valem 90; reservas (k ≥ 11) só com o byte de entrada
/// (e − 2 ≠ 0xff) e menos ele. `nota`, `troca` são os deslocamentos dos dois bytes tentados em ordem (o primeiro ≥ 0
/// vale; senão o +0x4d do jogo).
pub(crate) unsafe fn cqc8_nota(jogo: usize, e: usize, k: i32, nota: usize, troca: usize, entrada: usize) -> Option<i8> {
    if cqc8_u(jogo + 0x4c) != 0 && k < 11 {
        return Some(0x5a);
    }
    let escolher = || {
        let v = read_i8(e + nota);
        if v > -1 {
            return v;
        }
        let v = read_i8(e + troca);
        if v > -1 { v } else { read_i8(jogo + 0x4d) }
    };
    if k < 11 {
        return Some(escolher());
    }
    let dl = read_i8(e + entrada);
    if dl == -1 {
        return None;
    }
    Some(escolher().wrapping_sub(dl))
}

/// the board confidence adjustment after the match: the rivalry, the reputation gap or the team and opponent ratings in
/// the match, the score, the aggregate, the derby and rival, the competition weight; writes the four confidence values
/// (1 to 10000, with floors by stage), raises the media stories and the fan protest, and decides whether to consider
/// sacking the manager
/// Original: `MANAGER_MANAGER::update_board_confidence_after_match` (0x695240, manager_manager.cpp, thiscall).
///
/// [pt] { 0x695240 } (thiscall, ret 0x10, manager_manager.cpp): `MANAGER_MANAGER::update_board_confidence_after_match(
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
pub(crate) unsafe extern "thiscall" fn confianca_depois_da_partida(this: usize, clube: usize, partida: usize, jogo: usize, mandante: u32) {
    let outro = if mandante != 0 { read_usize(partida + 0x20) } else { read_usize(partida + 0x1c) };
    let vis = mandante == 0;
    let mut fator: f64 = 0.5;
    let mut peso = cqc8_peso(clube, partida, &mut fator);
    // a disputa (pela seleção da partida +0x18)
    let ev = read_usize(partida + 0x18);
    let cidade = |p: usize| p != 0 && read_usize(p + 0x38) != 0;
    let mut aj: i32 = 0;
    if cqc8_selecao(clube) != 0 {
        if cidade(ev) && read_usize(read_usize(ev + 0x38) + 0x1f) == read_usize(clube + 0x53) {
            aj = -20;
        } else if read_usize(clube + 0x69) == ev {
            aj = -20;
        } else if read_usize(outro + 0x69) == ev {
            aj = 20;
        } else if cidade(ev) && read_usize(read_usize(ev + 0x38) + 0x1f) == read_usize(outro + 0x53) {
            aj = 20;
        }
    } else if read_usize(clube + 0x69) == ev {
        aj = -20;
    } else if read_usize(outro + 0x69) == ev {
        aj = if read_usize(outro + 0x53) != read_usize(clube + 0x53) { 30 } else { 20 };
    }
    let limitar = |q: i32| {
        let d = q as i16;
        if d < -100 { -100 } else if d > 100 { 100 } else { q }
    };
    let c: i32;
    if jogo == 0 {
        let x = (read_i16(clube + 0x80) as i32).wrapping_sub(read_i16(outro + 0x80) as i32);
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
            let quatro_d = read_i16(jogo + 0x4d);
            let al = if cqc8_u(jogo + 0x4c) != 0 && k < 11 {
                0x5a
            } else if k < 11 {
                let v = read_i8(e);
                if v > -1 {
                    di = di.wrapping_add(quatro_d.wrapping_sub(v as i16) as u16);
                    v
                } else {
                    let v = read_i8(e - 4);
                    if v > -1 { v } else { read_i8(jogo + 0x4d) }
                }
            } else {
                let dl = read_i8(e - 2);
                if dl == -1 {
                    continue;
                }
                let v = read_i8(e);
                if v > -1 {
                    di = di.wrapping_add(quatro_d.wrapping_sub(v as i16) as u16);
                    v.wrapping_sub(dl)
                } else {
                    let v = read_i8(e - 4);
                    (if v > -1 { v } else { read_i8(jogo + 0x4d) }).wrapping_sub(dl)
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
        crate::comparacao::ramo("cqc_8 nota do jogo");
        let mut v = (b127 - 10) as f64 * read_f64(0x969020) + media;
        v += (b128 - 10) as f64 * read_f64(0x969020);
        v += (b127 - o127) as f64 * read_f64(0x968a98);
        v += (di as i16) as f64 * read_f64(0x96d1d0);
        v += read_f64(0x967880);
        v = v * read_f64(0x96d1c8) - read_f64(0x96d1c0); // cqc8: a nota do jogo (× 67 − 470)
        v *= fator;
        aj = aj.wrapping_add(cqc8_ftol(v));
        // a reputação média dos adversários escalados (pela nação do clube)
        let estrangeira: unsafe extern "C" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_DATABASE_IS_STAFF_FOREIGN);
        let (mut s1, mut s2) = (0i32, 0i32);
        for k in 0..20i32 {
            let e = ob + 0x1c0 + k as usize * 0x49;
            let id = cqc8_i(e - 0x30);
            if id == -1 {
                continue;
            }
            let p = read_usize(0xae23bc).wrapping_add((id as usize).wrapping_mul(0x6e));
            let nacao = read_usize(clube + 0x53);
            let ficha = read_usize(p + 0x61);
            let rep = if read_usize(outro + 0x53) == nacao {
                read_i16(ficha + 0xb)
            } else if read_usize(p + 0x1a) == nacao {
                read_i16(ficha + 9)
            } else if estrangeira(p, nacao, 1) == 0 {
                read_i16(read_usize(p + 0x61) + 9)
            } else {
                read_i16(read_usize(p + 0x61) + 0xd)
            };
            let al = match cqc8_nota(jogo, e, k, 4, 0, 2) {
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
        let x = (read_i16(clube + 0x80) as i32).wrapping_sub((read_i16(outro + 0x80) as i32).wrapping_add(m) / 2);
        c = limitar(x / -40);
        aj = aj.wrapping_add(c);
    }
    // o placar: A e B (as notas de quem é o clube), C e D (o agregado), como a original guarda
    let p8 = |o: usize| read_i8(partida + o) as i32;
    let (a, b, cc, dd) = if read_i8(partida + 0x47) > -1 {
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
        crate::comparacao::ramo("cqc_8 vitoria");
        aj = aj.wrapping_add((c16 / 10 + 20).wrapping_mul(a - b)); // cqc8: a vitória
    } else if a < b {
        aj = aj.wrapping_add((20 - c16 / 10).wrapping_mul(a - b));
        fl = 0x1000;
        let comp = read_usize(partida + 0x14);
        let amistoso: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_FRIENDLY_IS_FRIENDLY_COMPETITION);
        let mut fim = false;
        if comp != 0 && amistoso(read_usize(0xae2c74), comp) == 0 {
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
            let tec = read_usize(clube + 0xcf);
            let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
            if tec != 0 && read_usize(tec + 0x69) != 0 && humano(clube, 0, 1, 0, 0) == 0 {
                let tec = read_usize(clube + 0xcf);
                let r = cqc8_sortear(6);
                if read_i8(tec + 0x5d) < (r as u8).wrapping_add(6) as i8
                    && read_i8(read_usize(0xae2d14).wrapping_add((cqc8_i(clube) as usize).wrapping_mul(6)) + 4) < 0
                    && cqc8_sortear(read_i8(read_usize(tec + 0x69) + 0x1d) as i32 + 1) == 0
                {
                    let pedido: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_MANAGER_MANAGER_HANDLE_FA_REQUEST);
                    pedido(this, clube, 1);
                }
            }
        }
    } else {
        fl = 0x400;
        if cqc8_u(partida + 0x3a) == 8 {
            let f = read_i16(partida + 0x32) as i32;
            if (0x41f..=0x426).contains(&f) && vis {
                aj = aj.wrapping_add(0xf);
            }
            let s = (read_i16(partida + 0x30) as i32).wrapping_sub(0x7b);
            if (0..=0x1f).contains(&s) && matches!(s, 0 | 1 | 10 | 11 | 30 | 31) {
                aj = aj.wrapping_add(if vis { a.wrapping_mul(10) } else { b.wrapping_mul(-5) });
            }
        }
    }
    // o agregado
    let g4b = read_i8(partida + 0x4b);
    if g4b != -1 {
        let g4c = read_i8(partida + 0x4c);
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
            let (g49, g4a) = (read_i8(partida + 0x49), read_i8(partida + 0x4a));
            if g49 > g4a {
                if mandante != 0 { mais(&mut fl, &mut aj, 10) } else { menos(&mut fl, &mut aj, 10) }
            } else if g49 < g4a {
                if mandante == 0 { mais(&mut fl, &mut aj, 10) } else { menos(&mut fl, &mut aj, 10) }
            } else if cqc8_u(partida + 0x38) & 4 == 0 {
                let x = g4b as i32 - read_i8(partida + 0x43) as i32;
                let fora = x > read_i8(partida + 0x44) as i32;
                if fora == (mandante != 0) { menos(&mut fl, &mut aj, 15) } else { mais(&mut fl, &mut aj, 15) }
            }
        }
    }
    // o clássico e o rival
    let rival = read_usize(clube + 0xb3) == outro || read_usize(clube + 0xb7) == outro || read_usize(clube + 0xbb) == outro;
    let f1 = read_usize(clube + 0x69);
    let mesma = f1 != 0 && read_usize(outro + 0x69) != 0 && read_usize(f1 + 0x38) == read_usize(read_usize(outro + 0x69) + 0x38);
    if mesma {
        fl |= if rival { 0x40 } else { 0x80 };
    } else if rival {
        fl |= 0x80;
        if f1 != 0 {
            let e1 = read_usize(f1 + 0x38);
            let f2 = read_usize(outro + 0x69);
            if e1 != 0 && f2 != 0 {
                let e2 = read_usize(f2 + 0x38);
                if e2 != 0 {
                    let x = read_f64(e1 + 0x2b) - read_f64(e2 + 0x2b);
                    let y = read_f64(e1 + 0x23) - read_f64(e2 + 0x23);
                    if (y * y + x * x).sqrt() * read_f64(0x96d1b8) <= read_f64(0x968890) {
                        fl |= 0x40;
                    }
                }
            }
        }
    } else {
        let confronto: unsafe extern "C" fn(usize, usize) -> u8 = core::mem::transmute(FN_COMP_UTIL_GET_LEAGUE_CLASH_TYPE);
        if confronto(clube, outro) != 0 {
            fl |= 0x80;
        }
    }
    if fl & 0x40 != 0 {
        peso *= read_f64(0x968ec8);
        if fl & 0x800 != 0 {
            aj = aj.wrapping_add(0xc);
        } else if fl & 0x1000 != 0 {
            aj = aj.wrapping_sub(0xc);
        }
    } else if fl & 0x80 != 0 {
        peso *= read_f64(0x968940);
        if fl & 0x800 != 0 {
            aj = aj.wrapping_add(5);
        } else if fl & 0x1000 != 0 {
            aj = aj.wrapping_sub(5);
        }
    }
    let segunda: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_CLUB_IS_NATIONAL_TEAM);
    let mut mult = if segunda(clube) != 0 {
        peso * read_f64(0x968968)
    } else {
        let s = cqc8_selecao(clube);
        if s != 0 { peso + peso } else { peso }
    };
    if cqc8_u(partida + 0x3a) == 8 {
        if cqc8_u(cqc8_linha(this, clube) + 0xf) & 4 != 0 { // a palavra +0xe & 0x400
            mult = mult + mult;
            if (aj as i16) < 0 {
                aj = cqc8_ftol((aj as i16) as f64 * read_f64(0x968940));
            }
        } else {
            mult *= read_f64(0x968940);
        }
    }
    let mut dx = cqc8_ftol((aj as i16) as f64 * mult);
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
    let conf6 = || read_i16(cqc8_linha(this, clube) + 6);
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
    let marcas = read_i16(partida + 0x38) as u16;
    if marcas & 0x2810 == 0 {
        let texto: unsafe extern "thiscall" fn(usize, usize, usize, usize, i32, usize, i32, i32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(FN_MANAGER_MANAGER_BUILD_BOARD_MATCH_REACTION_TEXT);
        let humano: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_NEWS_CLUB_HAS_HUMAN_MANAGER);
        if texto(this, clube, outro, read_usize(partida + 0x14), fl as i16 as i32, 0xad9c64, -1, -1, 0, 0, 0, 0, 0, 0, 0, 0) != 0
            && humano(read_usize(0xb7431c), clube) != 0
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
            let lim = ((vis as u16).wrapping_mul(0x4e2)).wrapping_add(read_i16(outro + 0x80) as u16).wrapping_add(0x1f4) as i16;
            if fl & 7 == 0 || read_i16(clube + 0x80) >= lim || read_i8(partida + 0x43) != read_i8(partida + 0x44) {
                let reacao: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, i32, i32, i32, i32) = core::mem::transmute(FN_MANAGER_MANAGER_POST_BOARD_MATCH_REACTION_NEWS);
                reacao(this, clube, outro, partida, fl, a, b, cc, dd);
            }
        }
    }
    // a história 0x3d da mídia
    let marcas = read_i16(partida + 0x38) as u16;
    let dirigivel: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(FN_CLUB_CAN_BE_MANAGED);
    let historia: unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, u32, i32) = core::mem::transmute(FN_MEDIA_POST_TEAM_MEDIA_STORY);
    if fl & 0xa00 == 0 && marcas & 0x2810 == 0 && dirigivel(clube) != 0 {
        let s = aj as i16 as i32;
        let oito = || read_i16(cqc8_linha(this, clube) + 8) as i32;
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
            historia(read_usize(0xb6b7c8), clube, 0x3d, 0, partida, 0, -1);
            let l = cqc8_linha(this, clube) + 0x16;
            write_i16(l, read_i16(l).wrapping_add(-0xfa));
        }
    }
    // a história 0x5c: o clube em perigo na liga
    let id = cqc8_i(partida);
    if id > -1 && id == cqc8_i(clube + 0x1db) && id != cqc8_i(KEY_CLUB_COMP_INTER_TOTO_CUP) && cqc8_u(partida + 0x3a) == 8
        && (read_i16(partida + 0x30) as i32 / 10) * 10 != 0x96
    {
        let co = read_usize(read_usize(0xadadfc).wrapping_add((id as usize).wrapping_mul(4)));
        if co != 0 && read_usize(co + 8) != 0 {
            let linha_da_liga: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_COMP_STATS_FIND_CLUB_ENTRY);
            let ln = linha_da_liga(read_usize(co + 8), clube);
            if fl & 0x100 == 0 {
                let n = cqc8_u(ln + 0x72);
                if n < 9 && n != 0 && cqc8_u(ln + 0x74) == 1 && (aj as i16 as i32) > (n as i32 * n as i32) / 2 + 0x32 {
                    historia(read_usize(0xb6b7c8), clube, 0x5c, 0, partida, 0, -1);
                }
            }
        }
    }
    let mut aj = aj;
    if (aj as i16) < 0 {
        let tec = read_usize(clube + 0xcf);
        let marca: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_PLAYER_REGEN_GET_RETIREMENT_STATUS);
        if tec != 0 && marca(read_usize(0xb7434c), tec) != 0 {
            aj = cqc8_ftol((aj as i16) as f64 * read_f64(0x968968));
        }
    }
    // a confiança
    let v = (aj as i16) as f64;
    crate::comparacao::ramo("cqc_8 confianca");
    for (o, k) in [(6usize, 0x968e40usize), (0xa, 0x968968), (8, 0x968980), (0xc, 0x968918)] {
        let l = cqc8_linha(this, clube) + o;
        let t = cqc8_ftol(v * read_f64(k)); // cqc8: a confiança pelo ajuste
        write_i16(l, read_i16(l).wrapping_add(t as i16));
    }
    for o in [6usize, 0xa, 8, 0xc] {
        let l = cqc8_linha(this, clube) + o;
        let x = read_i16(l);
        if x > 0x2710 {
            write_i16(l, 0x2710);
        } else if x < 1 {
            write_i16(l, 1);
        }
    }
    let piso = |o: usize, v: i16| {
        let l = cqc8_linha(this, clube) + o;
        if read_i16(l) < v {
            write_i16(l, v);
        }
    };
    let comp = read_usize(partida + 0x14);
    if (aj as i16) > 0 && read_usize(clube + 0xcf) != 0 {
        if comp != 0 {
            let n = read_i16(comp + 0x69);
            if n > 5 {
                let rep = read_i16(clube + 0x80);
                let fase = read_i16(partida + 0x30) as u16;
                if (n as i32) * 500 > rep as i32 {
                    if fase == 0x82 || fase == 0x78 {
                        piso(6, 0x1964);
                        piso(8, 0x1964);
                    } else if fase == 0x96 {
                        piso(6, 0x2134);
                        piso(8, 0x2134);
                    } else if comp != read_usize(clube + 0x57) && (rep < 0xea6 || cqc8_u(partida + 0x3a) == 8) {
                        piso(6, 0x10b3);
                        piso(8, 0x1964);
                    }
                } else if fase == 0x82 || fase == 0x96 {
                    piso(6, 0x1964);
                    piso(8, 0x1964);
                } else if comp != read_usize(clube + 0x57) && (fase == 0x78 || cqc8_u(partida + 0x3a) == 8) {
                    piso(6, 0x10b3);
                    piso(8, 0x10b3);
                }
            }
        }
    } else if (aj as i16) < 0 && comp != 0 && read_i16(partida + 0x30) == 0x96 && cqc8_u(partida + 0x3a) == 5 {
        let l = cqc8_linha(this, clube);
        if cqc8_u(l + 0xe) & 4 != 0 {
            if read_i16(l + 6) > 0x10b3 {
                write_i16(l + 6, 0x10b3);
            } else if read_i16(l + 8) > 0xdac {
                write_i16(l + 6, 0xdac);
            }
            let x = read_i16(l + 8);
            if x > 0x10b3 {
                write_i16(l + 8, 0x10b3);
            } else if x > 0xdac {
                write_i16(l + 8, 0xdac);
            }
        }
    }
    if read_usize(clube + 0xcf) != 0 && (aj as i16) < -0x4b && read_i16(cqc8_linha(this, clube) + 8) < 0xbb8 {
        let protesto: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_MANAGER_MANAGER_POST_SUPPORTER_PROTEST_NEWS);
        protesto(this, clube);
    }
    if read_usize(partida + 0x14) == read_usize(clube + 0x57) || (aj as i16) < 0 {
        if read_i16(partida + 0x30) != 0xa0 {
            let demitir: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_MANAGER_MANAGER_CONSIDER_SACKING_MANAGER);
            demitir(this, clube, (a < b) as u32);
        }
        return;
    }
    let lista = read_usize(this + 0x10);
    if read_i16(lista) == 0 {
        return;
    }
    core::ptr::write_unaligned((lista + 0xa) as *mut u32, read_usize(lista + 2) as u32);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let tirar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_LLIST_REMOVE);
    let mut i = 0i32;
    while i < read_i16(read_usize(this + 0x10)) as i32 {
        let e = proximo(read_usize(this + 0x10));
        if e != 0 && cqc8_i(clube) == cqc8_i(e) {
            tirar(read_usize(this + 0x10), e);
            return;
        }
        i += 1;
    }
}

// ponytail: chama o pedido à federação (0x6a9e60), a notícia da reação (0x6a5fa0), as histórias da mídia (0x74b060), o
// protesto, a demissão (0x69aba0) e tira item da lista dos ultimatos (0x68d1f0): alocam, liberam e sorteiam conforme o
// caminho; não roda duas vezes. Só a original; a cópia leva os ramos e se prova pela carreira e pelas mutações.
pub(crate) unsafe extern "thiscall" fn comparar_confianca_depois_da_partida(this: usize, clube: usize, partida: usize, jogo: usize, mandante: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32)>(1527)(this, clube, partida, jogo, mandante)
}

/// news item 0x138c with the board reaction to the match for the club manager: the flags, the clubs, the scores, the
/// named players and the match record
/// Original: `MANAGER_MANAGER::post_board_match_reaction_news` (0x6a5fa0, manager_manager.cpp, thiscall).
///
/// [pt] { 0x6a5fa0 } (thiscall, ret 0x20, manager_manager.cpp, quadro SEH sem destrutor no caminho normal):
/// `MANAGER_MANAGER::post_board_match_reaction_news(clube, adversário, partida, marcas, a, b, c, d)`: com clube,
/// adversário, partida e competição (+0x14), a notícia 0x138c (0x77bbe0) com as marcas, os dois ids, os placares a e
/// b, o agregado c e d e +8 da partida; os até 3 jogadores que a diretoria cita (0x6aaee0, com a derrota a > b), um
/// sorteio(4), a derrota e as 5 marcas deles; o registro da partida pelo clube (0x7e4750) gravado na notícia
/// (0x7e4de0); entregue ao técnico do clube (0x7861e0).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn noticia_da_reacao_da_diretoria(this: usize, clube: usize, outro: usize, partida: usize, marcas: u32, a: i32, b: i32, c: i32, d: i32) {
    if clube == 0 || outro == 0 || partida == 0 || read_usize(partida + 0x14) == 0 {
        return;
    }
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let data_padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let citados: unsafe extern "thiscall" fn(usize, usize, usize, *mut i32, usize, *mut u8, *mut u8, *mut u8, *mut u8, *mut u8, *mut u8) -> u8 =
        core::mem::transmute(FN_MANAGER_MANAGER_COLLECT_BOARD_MATCH_REACTION_PLAYERS);
    let registro: unsafe extern "C" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_FILL_MATCH_RECORD_FROM_FIXTURE);
    let no_registro: unsafe extern "C" fn(usize, usize, *mut u8) -> u32 = core::mem::transmute(FN_WRITE_MATCH_RECORD_TO_NEWS);
    let ao_tecnico: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    crate::comparacao::ramo("cqc_8 noticia da reacao");
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
    for v in [marcas as i16 as i32 as u32, read_usize(clube) as u32, read_usize(outro) as u32, a as u32, b as u32, c as u32, d as u32, read_usize(partida + 8) as u32] {
        gravar(n, k as i8, v);
        k = k.wrapping_add(1);
    }
    let mut derrota = (a > b) as u8;
    citados(this, partida, clube, ids.as_mut_ptr(), read_usize(partida + 8), &mut derrota, &mut m1, &mut m2, &mut m3, &mut m4, &mut m5);
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
    ao_tecnico(read_usize(0xb7431c), n, clube);
}

// ponytail: a notícia vai ao técnico (aloca): só a original; a cópia leva o ramo.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn comparar_noticia_da_reacao_da_diretoria(this: usize, clube: usize, outro: usize, partida: usize, marcas: u32, a: i32, b: i32, c: i32, d: i32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, i32, i32, i32, i32)>(1528)(this, clube, outro, partida, marcas, a, b, c, d)
}

/// news item 0x1781 with the FA assessment of the national team manager in a competition, from the game start
/// Original: `MANAGER_MANAGER::post_fa_performance_news` (0x6a5a60, manager_manager.cpp, thiscall).
///
/// [pt] { 0x6a5a60 } (thiscall, ret 0xc, manager_manager.cpp, quadro SEH sem destrutor no caminho normal):
/// `MANAGER_MANAGER::post_fa_performance_news(competição, seleção, nota)`: a partir da data do começo do jogo
/// ([0xdd6ec8]), a notícia 0x1781 (0x77bbe0) com o id da seleção, o id da competição (*competição) e a nota da
/// federação, pelo alcance (0x786090 com a seleção e 0, 0, 0, 0).
pub(crate) unsafe extern "thiscall" fn noticia_da_avaliacao_da_federacao(_this: usize, comp: usize, selecao: usize, nota: u32) {
    let dia = read_i16(0xdd6ec8);
    let ano = read_i16(0xdd6eca) as u16;
    let hoje = read_i16(0xae2c92) as u16;
    if ano > hoje || (ano == hoje && dia > read_i16(0xae2c90)) {
        return;
    }
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let por_alcance: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_NEWS_SEND_BY_SCOPE);
    crate::comparacao::ramo("cqc_8 avaliacao da federacao");
    let mut local = [0u32; 0x3a]; // a notícia local (0xe8 bytes)
    let n = local.as_mut_ptr() as usize;
    construir(n, 0x1781, 0);
    gravar(n, 0, read_usize(selecao) as u32);
    gravar(n, 1, read_usize(comp) as u32);
    gravar(n, 2, nota as i8 as i32 as u32);
    por_alcance(read_usize(0xb7431c), n, selecao, 0, 0, 0, 0);
}

// ponytail: a notícia vai pelo alcance (aloca): só a original; a cópia leva o ramo.
pub(crate) unsafe extern "thiscall" fn comparar_noticia_da_avaliacao_da_federacao(this: usize, comp: usize, selecao: usize, nota: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(1529)(this, comp, selecao, nota)
}

// ---------- 266a leva: as expectativas da diretoria para a temporada, a vaga de técnico e a notícia da oferta de emprego (manager_manager.cpp) ----------


pub(crate) unsafe fn cqc9_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}



pub(crate) unsafe fn cqc9_u(a: usize) -> u8 {
    *(a as *const u8)
}





/// _ftol (0x945640): truncado (a original usa só eax ou ax).
pub(crate) fn cqc9_ftol(x: f64) -> i32 {
    x as i64 as i32
}

pub(crate) unsafe fn cqc9_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    f(n)
}

/// A linha da diretoria do clube: [this] + id × 0x49.
pub(crate) unsafe fn cqc9_linha(this: usize, clube: usize) -> usize {
    read_usize(this).wrapping_add((cqc9_i(clube) as usize).wrapping_mul(0x49))
}

/// O objeto da competição da divisão do clube: [[0xadadfc] + [+0x57] × 4].
pub(crate) unsafe fn cqc9_comp(clube: usize) -> usize {
    read_usize(read_usize(0xadadfc).wrapping_add((cqc9_i(read_usize(clube + 0x57)) as usize).wrapping_mul(4)))
}

/// Método virtual sem argumentos da competição (+0x98 ou +0x9c da tabela), thiscall.
pub(crate) unsafe fn cqc9_virtual(obj: usize, desloc: usize) -> u32 {
    let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(obj) + desloc));
    f(obj)
}

pub(crate) unsafe fn cqc9_selecao(c: usize) -> u32 {
    let f: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    f(c)
}

/// the board target in the league (danger position, wealth, previous division, manager tenure) and the expectation
/// flags (title, European places, top half, avoid relegation), the chairman draws, the departure by tenure, the
/// confidence values pulled back to 6500 and the manager record; a national team takes those of its competition
/// Original: `MANAGER_MANAGER::set_season_expectations` (0x6a3770, manager_manager.cpp, thiscall).
///
/// [pt] { 0x6a3770 } (thiscall, ret 8, manager_manager.cpp): `MANAGER_MANAGER::set_season_expectations(clube, notícia)`.
/// Zera as marcas (+0xe) e a meta (+0x2c), +0x30 = reputação² / 4, +0x38 ÷ 4, +0 = 0. Sem a marca 1 de +2, com técnico
/// e fora das seleções: pelos dias do técnico no cargo (0x54cdb0), acima de 3650 com confiança +6 < 3500 (sorteio(30)
/// acima do +0x1d do presidente, ou de 10) ou acima de 1825 com +6 < 3000 (sorteio(20)), o item {1, 6, id} na lista
/// [this+4] (0x68d0d0). Seleção: a B nada; a principal pega as expectativas da competição dela (0x7751e0 e o virtual
/// +0x70) e a notícia da federação (0x6a44a0). Clube: as vagas da UEFA da nação (0x905c60 em [0xde1f58]); sem técnico
/// nada; sem divisão com competição: 0x10. A meta = a posição de perigo da linha da liga (+0x72; sem ela, 8 com 0x40 e a
/// notícia, e sai), menos 1 se for a última; pela riqueza (0x5986b0: 2 → −3/−2, 1 → −2/−1, 0 com verba (0x598800) acima de
/// 250 × reputação → −1, −1 → +1 abaixo de 5, −2 → +1); pela divisão anterior (+0x5b: mais forte +4/+2, mais fraca −7/−3/−2)
/// ou pelos dias do técnico (abaixo de 365 +1; acima de 1095 −1 acima de 10); entre 1 e o número de clubes − 1; com a
/// marca 1, a média com +0x73 da linha entre +0x74 e +0x75. As marcas: 2 (vaga de campeão), 1 (vagas da UEFA, fora da
/// Austrália), 4 (+0xbe), 8 (+0xbf), 0x40 (metade de cima), 0x20 (fuga do rebaixamento, +0xc1) ou 0x10/0; sorteios do
/// presidente (0x80, 0x100, 0x200 pela meta) e a liga da copa ([0xae23d0], 0x400 ou a troca 0x80/0x200). A notícia
/// (0x6a4a20). Confianças abaixo de 6500 sobem a (5x + 27500 ou 25000) / 10; a riqueza em +1; a ficha do técnico (+0xa,
/// +0xc) sobe para a reputação do clube; o técnico novo (dos 16 últimos) ganha 1 em +0x59 até 20; zera +2, +3, +4, +5,
/// +0x12..+0x16, +0x28 e +0x40 & 7; as multas (0x5a5930 em [0xae2a3c]).
pub(crate) unsafe extern "thiscall" fn expectativas_da_temporada(this: usize, clube: usize, noticia: u32) {
    let ln = || cqc9_linha(this, clube);
    write_i16(ln() + 0xe, 0);
    write_u32(ln() + 0x2c, 0);
    let rep = read_i16(clube + 0x80) as i32;
    core::ptr::write_unaligned((ln() + 0x30) as *mut f64, rep.wrapping_mul(rep) as f64 * read_f64(0x9689e0));
    let l38 = ln() + 0x38;
    core::ptr::write_unaligned(l38 as *mut f64, read_f64(l38) * read_f64(0x9689e0));
    write_u8(ln(), 0);
    let mut dias: i32 = 0;
    if cqc9_u(ln() + 2) & 1 == 0 && read_usize(clube + 0xcf) != 0 && cqc9_selecao(clube) == 0 {
        let tec = read_usize(clube + 0xcf);
        let ate: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
        dias = ate(0xae2c90, read_usize(tec + 0x3e) as u32, read_usize(tec + 0x42) as u32);
        let pres = || {
            let bf = read_usize(clube + 0xbf);
            if bf != 0 && read_usize(bf + 0x69) != 0 { Some(read_i8(read_usize(bf + 0x69) + 0x1d) as i32) } else { None }
        };
        let sair = if dias > 0xe42 && read_i16(ln() + 6) < 0xdac {
            match pres() {
                Some(_) => {
                    let r = cqc9_sortear(0x1e);
                    r > pres().unwrap_or(0)
                }
                None => cqc9_sortear(0x1e) > 10,
            }
        } else if dias > 0x721 && read_i16(ln() + 6) < 0xbb8 {
            match pres() {
                Some(_) => {
                    let r = cqc9_sortear(0x14);
                    r > pres().unwrap_or(0)
                }
                None => cqc9_sortear(0x14) > 10,
            }
        } else {
            false
        };
        if sair {
            crate::comparacao::ramo("cqc_9 saida pelo tempo");
            // o item de 6 bytes {1, 6, id do clube} copiado na lista [this+4]
            let mut item = [0u8; 8];
            item[0] = 1;
            item[1] = 6;
            item[2..6].copy_from_slice(&(read_usize(clube) as u32).to_le_bytes());
            let acrescentar: unsafe extern "thiscall" fn(usize, *mut u8, u32) = core::mem::transmute(FN_LLIST_ADD_TAIL);
            acrescentar(read_usize(this + 4), item.as_mut_ptr(), 6);
        }
    }
    if cqc9_selecao(clube) != 0 {
        let federacao: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(FN_MANAGER_MANAGER_POST_FA_EXPECTATIONS_NEWS);
        let segunda: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_CLUB_IS_NATIONAL_TEAM);
        let mut feita = false;
        if segunda(clube) == 0 {
            let achar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_NATIONALTEAMS_GET_CURRENT_COMPETITION);
            let ent = achar(read_usize(0xb74318), read_usize(clube + 0x53));
            let co = if ent != 0 { read_usize(read_usize(0xadadfc).wrapping_add((cqc9_i(ent) as usize).wrapping_mul(4))) } else { 0 };
            if co != 0 {
                let exp: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(read_usize(read_usize(co) + 0x70));
                let w = exp(co, clube) as u16;
                write_i16(ln() + 0xe, w as i16);
                write_u32(ln() + 0x2c, read_usize(ent) as u32);
                if noticia as u8 != 0 {
                    // ponytail: a original empurra a palavra com a metade alta do registro (lixo); a 0x6a44a0 lê a palavra
                    federacao(this, clube, read_i16(ln() + 0xe) as u16 as u32, read_usize(ln() + 0x2c) as u32);
                }
                feita = true;
            } else {
                write_i16(ln() + 0xe, 0);
                write_u32(ln() + 0x2c, 0xffff_ffff);
            }
        }
        if !feita && noticia as u8 != 0 {
            federacao(this, clube, 0, 0xffff_ffff);
        }
        cqc9_fim(this, clube);
        return;
    }
    // as vagas da UEFA da nação: campeão (o byte do 2º argumento) e as outras (o 3º)
    let mut vagas: u32 = clube as u32; // a original grava no lugar do argumento `clube` (só o byte é lido)
    let mut outras: u8 = 0;
    let uefa: unsafe extern "thiscall" fn(usize, usize, *mut u32, *mut u8) = core::mem::transmute(FN_UEFASEEDING_GET_NATION_PLACES);
    uefa(read_usize(0xde1f58), read_usize(clube + 0x53), &mut vagas, &mut outras);
    let vagas = vagas as u8 as i8 as i32;
    let outras = outras as i8 as i32;
    if read_usize(clube + 0xcf) == 0 {
        cqc9_fim(this, clube);
        return;
    }
    let div = read_usize(clube + 0x57);
    let nacao = read_usize(clube + 0x53);
    if div == 0 || nacao == 0 || cqc9_u(nacao + 0x11c) == 0 || cqc9_comp(clube) == 0 {
        write_i16(ln() + 0xe, 0x10);
        cqc9_noticia(this, clube, noticia);
        cqc9_fim(this, clube);
        return;
    }
    let linha_da_liga: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_COMP_STATS_FIND_CLUB_ENTRY);
    let lg = linha_da_liga(read_usize(cqc9_comp(clube) + 8), clube);
    let mut copa: usize = 0;
    let id = cqc9_i(clube + 0x1db);
    if id > -1 && id != cqc9_i(KEY_CLUB_COMP_INTER_TOTO_CUP) {
        let cid = cqc9_i(read_usize(0xae23d0).wrapping_add((id as usize).wrapping_mul(0x6b)));
        let co = read_usize(read_usize(0xadadfc).wrapping_add((cid as usize).wrapping_mul(4)));
        copa = linha_da_liga(read_usize(co + 8), clube);
    }
    if lg == 0 || cqc9_u(lg + 0x72) < 1 {
        write_u32(ln() + 0x2c, 8);
        write_u8(ln() + 0xe, cqc9_u(ln() + 0xe) | 0x40);
        let diretoria: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_MANAGER_MANAGER_POST_BOARD_EXPECTATIONS_NEWS);
        // ponytail: a original empurra a palavra com a metade alta do registro (lixo); a 0x6a4a20 lê a palavra
        diretoria(this, clube, read_i16(ln() + 0xe) as u16 as u32);
        return;
    }
    let meta = || cqc9_i(ln() + 0x2c);
    let por = |v: i32| write_u32(ln() + 0x2c, v as u32);
    por(cqc9_u(lg + 0x72) as i32); // cqc9: a meta pela posição de perigo
    let n = read_i16(cqc9_comp(clube) + 0x3e) as i32;
    if meta() == n {
        por(n - 1);
    }
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FINANCETABLE_GET_CLUB_FINANCE);
    let faixa: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(FN_FINANCE_GET_WEALTH_BAND);
    let fin = financas(read_usize(0xae2a38), clube);
    crate::comparacao::ramo("cqc_9 meta");
    match faixa(fin) as i8 {
        2 => por(if meta() > 5 { meta() - 3 } else { meta() - 2 }),
        1 => por(if meta() > 5 { meta() - 2 } else { meta() - 1 }),
        0 => {
            let verba: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> i64 = core::mem::transmute(FN_FINANCE_GET_TRANSFER_BUDGET);
            let v = verba(fin, 0, 0, 0, 1, 0);
            if v > (read_i16(clube + 0x80) as i32 * 250) as i64 && meta() > 5 {
                por(meta() - 1);
            }
        }
        -1 => {
            if meta() < 5 {
                por(meta() + 1);
            }
        }
        -2 => por(meta() + 1),
        _ => {}
    }
    let (d57, d5b) = (read_usize(clube + 0x57), read_usize(clube + 0x5b));
    if d57 != 0 && d5b != 0 && d57 != d5b {
        if read_i16(d57 + 0x69) > read_i16(d5b + 0x69) {
            if meta() > 10 {
                por(meta() + 1);
            }
            por(if meta() > 5 { meta() + 2 } else { meta() + 4 });
        } else {
            let x = meta();
            if x > 10 {
                por(x - 7);
            } else if x > 5 {
                por(x - 3);
            } else if x > 2 {
                por(x - 2);
            }
        }
    } else if dias < 0x16d {
        por(meta() + 1);
    } else if dias > 0x447 {
        if meta() > 10 {
            por(meta() - 1);
        }
    } else if dias > 0x721 {
        // (inalcançável: a original testa depois de dias ≤ 1095)
        if meta() > 10 {
            por(meta() - 2);
        }
        if meta() > 5 {
            por(meta() - 1);
        }
    }
    let x = meta();
    let x = if x < 1 {
        1
    } else {
        let lim = read_i16(cqc9_comp(clube) + 0x3e) as i32 - 1;
        if lim < x { lim } else { x }
    };
    por(x as i8 as i32);
    if cqc9_u(ln() + 2) & 1 != 0 && cqc9_u(lg + 0x74) >= 1 && cqc9_u(lg + 0x75) >= 1 {
        let v = (cqc9_u(lg + 0x73) as i32).wrapping_add(meta());
        por(cqc9_ftol(v as f64 * read_f64(0x968968)) as i8 as i32);
        let (lo, hi) = (cqc9_u(lg + 0x74) as i32, cqc9_u(lg + 0x75) as i32);
        let x = meta();
        let x = if lo > x { lo } else if hi < x { hi } else { x };
        por(x as i8 as i32);
    }
    // as marcas pela meta
    let co = cqc9_comp(clube);
    let ou = |m: u8| write_u8(ln() + 0xe, cqc9_u(ln() + 0xe) | m);
    'marcas: {
        if cqc9_virtual(co, 0x98) == 0 && meta() <= vagas {
            ou(2);
            break 'marcas;
        }
        let na = read_usize(clube + 0x53);
        if na != 0 && cqc9_i(na) != cqc9_i(KEY_NATION_AUSTRALIA) && cqc9_virtual(cqc9_comp(clube), 0x98) == 0 && meta() <= vagas + outras {
            ou(1);
            break 'marcas;
        }
        let co = cqc9_comp(clube);
        if cqc9_virtual(co, 0x98) != 0 && meta() <= read_i8(cqc9_comp(clube) + 0xbe) as i32 {
            ou(4);
            break 'marcas;
        }
        let co = cqc9_comp(clube);
        let bf = read_i8(co + 0xbf);
        if bf != 0 && meta() <= bf as i32 {
            ou(8);
            break 'marcas;
        }
        if meta() <= read_i16(co + 0x3e) as i32 / 2 {
            write_i16(ln() + 0xe, 0x40);
            break 'marcas;
        }
        if cqc9_virtual(co, 0x9c) != 0 {
            let co = cqc9_comp(clube);
            if meta() >= read_i16(co + 0x3e) as i32 - read_i8(co + 0xc1) as i32 {
                ou(0x20);
                break 'marcas;
            }
        }
        let r = cqc9_virtual(cqc9_comp(clube), 0x9c);
        write_i16(ln() + 0xe, if r != 0 { 0x10 } else { 0 });
    }
    // os sorteios do presidente e a liga da copa
    let bf = read_usize(clube + 0xbf);
    if cqc9_u(ln() + 2) & 1 == 0 && cqc9_virtual(cqc9_comp(clube), 0x98) == 0 && bf != 0 && read_usize(bf + 0x69) != 0 {
        let p1d = || read_i8(read_usize(read_usize(clube + 0xbf) + 0x69) + 0x1d) as i32;
        crate::comparacao::ramo("cqc_9 sorteio do presidente");
        let r = cqc9_sortear(0x32); // cqc9: o sorteio do presidente
        if r > p1d() + meta() * 5 {
            ou(0x80);
        } else {
            let r = cqc9_sortear(0x4b);
            if r > p1d() + meta() * 5 {
                write_u8(ln() + 0xf, cqc9_u(ln() + 0xf) | 1);
            } else {
                let r = cqc9_sortear(100);
                if r > p1d() + meta() * 5 {
                    write_u8(ln() + 0xf, cqc9_u(ln() + 0xf) | 2);
                }
            }
        }
        if copa != 0 && cqc9_u(copa + 0x74) <= 1 {
            let t = (0x1d - p1d()) / 5;
            let p = cqc9_u(copa + 0x72) as i32;
            if p <= t {
                write_u8(ln() + 0xf, cqc9_u(ln() + 0xf) | 4);
            } else if p <= t + 4 {
                write_i16(ln() + 0xe, read_i16(ln() + 0xe) ^ 0x80);
                write_i16(ln() + 0xe, read_i16(ln() + 0xe) ^ 0x200);
                write_u8(ln() + 0xf, cqc9_u(ln() + 0xf) | 1);
            }
        }
    }
    cqc9_noticia(this, clube, noticia);
    cqc9_fim(this, clube);
}

/// A notícia das expectativas (0x6a4a20 com as marcas +0xe) quando pedida.
pub(crate) unsafe fn cqc9_noticia(this: usize, clube: usize, noticia: u32) {
    if noticia as u8 != 0 {
        let diretoria: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_MANAGER_MANAGER_POST_BOARD_EXPECTATIONS_NEWS);
        // ponytail: a original empurra a palavra com a metade alta do registro (lixo); a 0x6a4a20 lê a palavra
        diretoria(this, clube, read_i16(cqc9_linha(this, clube) + 0xe) as u16 as u32);
    }
}

/// O fim comum da 0x6a3770 (a partir de 0x6a415f): as confianças, a riqueza, a ficha do técnico, as marcas zeradas e as
/// multas.
pub(crate) unsafe fn cqc9_fim(this: usize, clube: usize) {
    let ln = || cqc9_linha(this, clube);
    if read_i16(ln() + 6) < 0x1964 {
        crate::comparacao::ramo("cqc_9 confianca de volta");
        for (o, k) in [(6usize, 0x96d288usize), (8, 0x9688d0), (0xa, 0x9688d0), (0xc, 0x9688d0)] {
            let x = read_i16(ln() + o) as i32;
            let v = (x as f64 * read_f64(0x9688a0) + read_f64(k)) * read_f64(0x967880); // cqc9: a confiança de volta
            write_i16(ln() + o, cqc9_ftol(v) as i16);
        }
    }
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FINANCETABLE_GET_CLUB_FINANCE);
    let faixa: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(FN_FINANCE_GET_WEALTH_BAND);
    let r = if cqc9_selecao(clube) == 0 {
        let fin = financas(read_usize(0xae2a38), clube);
        if fin != 0 { faixa(fin) } else { 0 }
    } else {
        0
    };
    write_u8(ln() + 1, r);
    let rep = || read_i16(clube + 0x80);
    let tec = read_usize(clube + 0xcf);
    if tec != 0 && read_usize(tec + 0x69) != 0 {
        let fx = read_usize(tec + 0x69);
        let a = read_i16(fx + 0xa);
        if a < rep() {
            let t = cqc9_ftol((rep() as i32 - a as i32) as f64 * read_f64(0x969518));
            write_i16(fx + 0xa, a.wrapping_sub(t as i16));
            let fx = read_usize(read_usize(clube + 0xcf) + 0x69);
            if read_i16(fx + 0xa) > 0x2710 {
                write_i16(fx + 0xa, 0x2710);
            }
            let fx = read_usize(read_usize(clube + 0xcf) + 0x69);
            let c = read_i16(fx + 0xc);
            let t = cqc9_ftol((rep() as i32 - c as i32) as f64 * read_f64(0x96d280));
            write_i16(fx + 0xc, c.wrapping_sub(t as i16));
            let fx = read_usize(read_usize(clube + 0xcf) + 0x69);
            if read_i16(fx + 0xc) > 0x2710 {
                write_i16(fx + 0xc, 0x2710);
            }
        }
    }
    let tec = read_usize(clube + 0xcf);
    if tec != 0 && read_usize(tec + 0x69) != 0 {
        let fx = read_usize(tec + 0x69);
        let a = read_i16(fx + 0xa);
        if rep() > a {
            let t = cqc9_ftol((rep() as i32 - a as i32) as f64 * read_f64(0x969518));
            write_i16(fx + 0xa, a.wrapping_sub(t as i16));
        }
    }
    let tec = read_usize(clube + 0xcf);
    if tec != 0 && cqc9_i(tec) >= cqc9_i(0xae2364).wrapping_sub(0x10) && read_i8(tec + 0x59) < 0x14 {
        write_u8(tec + 0x59, cqc9_u(tec + 0x59).wrapping_add(1));
    }
    write_u8(ln() + 2, cqc9_u(ln() + 2) & 0xfe);
    write_u8(ln() + 4, 0);
    write_u8(ln() + 3, cqc9_u(ln() + 3) & 0xe1);
    write_u8(ln() + 2, cqc9_u(ln() + 2) & 0xdf);
    write_u32(ln() + 0x28, 0);
    write_u8(ln() + 5, 0);
    write_u8(ln() + 3, cqc9_u(ln() + 3) & 0xfe);
    write_u8(ln() + 0x40, cqc9_u(ln() + 0x40) & 7);
    write_i16(ln() + 0x12, 0);
    write_i16(ln() + 0x14, 0);
    write_i16(ln() + 0x16, 0);
    let multas = read_usize(0xae2a3c);
    if multas != 0 {
        let f: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_FINE_HALVE_CLUB_FINE_COUNT);
        f(multas, clube);
    }
}

// ponytail: acrescenta na lista das saídas (0x68d0d0, aloca), posta as notícias (0x6a44a0, 0x6a4a20) e sorteia: não roda
// duas vezes. Só a original; a cópia leva os ramos e se prova pela carreira e pelas mutações.
pub(crate) unsafe extern "thiscall" fn comparar_expectativas_da_temporada(this: usize, clube: usize, noticia: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(1530)(this, clube, noticia)
}

/// A pessoa de índice `k` ([0xae23bc] + k × 0x6e).
pub(crate) unsafe fn cqc9_pessoa(k: i32) -> usize {
    read_usize(0xae23bc).wrapping_add((k as usize).wrapping_mul(0x6e))
}

/// O byte +0xb do estado da pessoa de índice `k` ([0xae2d0c] + k × 0x4f).
pub(crate) unsafe fn cqc9_estado(k: i32) -> u8 {
    cqc9_u(read_usize(0xae2d0c).wrapping_add((k as usize).wrapping_mul(0x4f)) + 0xb)
}

/// A nota do candidato (0x69d1b0: thiscall, 4: pessoa, clube, 0, 5; original).
pub(crate) unsafe fn cqc9_nota(this: usize, p: usize, clube: usize) -> i32 {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> i32 = core::mem::transmute(FN_SCORE_STAFF_FOR_JOB);
    f(this, p, clube, 0, 5)
}

/// 0x69f810 (thiscall, 4: pessoa, clube e a nota mínima em double, em dois dwords).
pub(crate) unsafe fn cqc9_serve(this: usize, p: usize, clube: usize, minimo: f64) -> u8 {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, f64) -> u8 = core::mem::transmute(FN_MANAGER_MANAGER_IS_STAFF_SUITABLE_FOR_JOB);
    f(this, p, clube, minimo)
}

/// 0x4cf6d0 (thiscall em [0xae1050], 1): a pessoa tem ficha de contrato.
pub(crate) unsafe fn cqc9_com_contrato(p: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_LOAN_CONTRACT);
    f(read_usize(0xae1050), p)
}

/// 0x69fc60 (thiscall, 1): a pessoa já está numa lista de vagas.
pub(crate) unsafe fn cqc9_na_lista(this: usize, p: usize) -> u8 {
    let f: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_MANAGER_MANAGER_IS_VACANCY_CANDIDATE);
    f(this, p)
}

/// O item de 0x26 bytes da vaga (o que a original monta em [esp+0x1c]): +0 achou um livre, +1 0, +2 o id dele, +6 o
/// índice do escolhido, +0xa o id do técnico que saiu, +0xe o id do clube, +0x12 a nota anterior, +0x16 a data da
/// chegada e +0x1e a data do anúncio (8 bytes cada, 0x54a110).
pub(crate) struct Cqc9Vaga(pub(crate) [u8; 0x28]);

impl Cqc9Vaga {
    fn b(&mut self, o: usize, v: u8) {
        self.0[o] = v;
    }
    fn d(&mut self, o: usize, v: i32) {
        self.0[o..o + 4].copy_from_slice(&v.to_le_bytes());
    }
    fn rd(&self, o: usize) -> i32 {
        i32::from_le_bytes([self.0[o], self.0[o + 1], self.0[o + 2], self.0[o + 3]])
    }
    fn w(&mut self, o: usize, v: u16) {
        self.0[o..o + 2].copy_from_slice(&v.to_le_bytes());
    }
    fn rw(&self, o: usize) -> u16 {
        u16::from_le_bytes([self.0[o], self.0[o + 1]])
    }
}

/// resets the club board and picks the manager for the vacancy: the best score among the free people who qualify (or
/// the assistant, or the first free one counting backwards), sometimes a better drawn one; the announcement and arrival
/// dates and the item in the vacancy list; the human managers apply
/// Original: `MANAGER_MANAGER::add_job_vacancy` (0x69ba50, manager_manager.cpp, thiscall).
///
/// [pt] { 0x69ba50 } (thiscall, ret 8, manager_manager.cpp, quadro SEH sem destrutor no caminho normal):
/// `MANAGER_MANAGER::add_job_vacancy(clube, técnico que saiu)`. Clube sem divisão e reputação abaixo de 5000: nada;
/// seleção B com ranking (+0x88 da nação) até 1000 ou principal até 100: nada. A diretoria recomeça (as quatro
/// confianças 5000, +0xe = 0x10, as marcas zeradas). O escolhido: de todas as pessoas com ficha de técnico (+0x69; os 16
/// últimos, os humanos, só com +0x5f), fora o que saiu, sem estado 0x30, sem contrato (0x4cf6d0), fora das listas
/// (0x69fc60), da mesma nação ou com +0xa ≥ 3750, do mesmo continente (+0x75) ou com +0xc ≥ 4250 e o clube com
/// reputação ≥ 4750, que serve para o cargo (0x69f810 com 3,5), fora de jogadores com contrato em vigor nos cargos
/// 11/12/13/15, só técnico de seleção (+0x3d = 5) numa seleção: a maior nota (0x69d1b0). Sem ninguém: o auxiliar (+0xd3),
/// o primeiro de +0x19f ou, de trás para a frente a partir do 17º dos últimos, a primeira pessoa livre nos cargos 0, 5, 6,
/// 7, 8, 11, 12, 15 (sem dar a volta: a caixa "Can't find a manager" e nada). Seleção ou clube grande (reputação acima
/// de 4500 + sorteio(2000)): sorteia tantas pessoas quanto o número delas e fica com a melhor livre (com 150 dias no
/// cargo, 3,15). O humano rejeitado pelo clube (0x547e70). As datas: antes do começo do jogo, de 1 a 7 dias depois dele (o
/// anúncio) e a chegada de 1 a 7 dias depois; senão, o anúncio de 1 a 3 dias e a chegada de 4 a 8 dias (7 a 11 se quem
/// saiu é humano; 10 a 14 com a notícia da oferta 0x6a0540 ao clube humano do escolhido), às vezes mais 0 a 19; a
/// virada do ano. O item vai para a lista [this+8] (0x68d0d0, o cursor guardado); os humanos (0x5fd210, 0x5fea50) de
/// clube menor se candidatam (0x69ca00).
pub(crate) unsafe extern "thiscall" fn vaga_de_tecnico(this: usize, clube: usize, saiu: usize) {
    // ponytail: a nota anterior (+0x12) fica sem valor na original quando o escolhido é o auxiliar ou o de +0x19f (sobra de
    // pilha); aqui 0, um valor que o jogo também produz
    let mut v = Cqc9Vaga([0; 0x28]);
    let data_padrao: unsafe extern "thiscall" fn(*mut u8) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    data_padrao(v.0.as_mut_ptr().add(0x16));
    data_padrao(v.0.as_mut_ptr().add(0x1e));
    let mut melhor: i32 = -1_000_000;
    let segunda: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_CLUB_IS_NATIONAL_TEAM);
    if cqc9_selecao(clube) == 0 && read_usize(clube + 0x57) == 0 && read_i16(clube + 0x80) < 0x1388 {
        return;
    }
    if cqc9_selecao(clube) != 0 {
        if segunda(clube) != 0 && cqc9_i(read_usize(clube + 0x53) + 0x88) <= 0x3e8 {
            return;
        }
        if segunda(clube) == 0 && cqc9_i(read_usize(clube + 0x53) + 0x88) <= 0x64 {
            return;
        }
    }
    v.d(0xe, cqc9_i(clube));
    v.d(6, -1);
    let ln = || cqc9_linha(this, clube);
    for o in [6usize, 8, 0xa, 0xc] {
        write_i16(ln() + o, 0x1388);
    }
    write_u8(ln(), 0);
    write_i16(ln() + 0xe, 0x10);
    for o in [0x10usize, 0x12, 0x14, 0x16] {
        write_i16(ln() + o, 0);
    }
    for m in [0xfeu8, 0xfd, 0xfb, 0xf7, 0xef] {
        write_u8(ln() + 2, cqc9_u(ln() + 2) & m);
    }
    write_u8(ln() + 4, 0);
    write_u8(ln() + 3, cqc9_u(ln() + 3) & 0xe1);
    for m in [0xdfu8, 0xbf, 0x7f] {
        write_u8(ln() + 2, cqc9_u(ln() + 2) & m);
    }
    write_u32(ln() + 0x28, 0);
    write_u8(ln() + 5, 0);
    write_u8(ln() + 3, cqc9_u(ln() + 3) & 0xfe);
    write_u8(ln() + 0x40, cqc9_u(ln() + 0x40) & 7);
    v.d(0xa, if saiu != 0 { cqc9_i(saiu) } else { -1 });
    // o melhor de todos
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let dirigivel: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(FN_CLUB_CAN_BE_MANAGED);
    let mut k = 0i32;
    while k < cqc9_i(0xae2364) {
        let p = cqc9_pessoa(k);
        let ultimos = cqc9_i(0xae2364) - 0x10;
        let ok = 'c: {
            if read_usize(p + 0x69) == 0 || (k >= ultimos && cqc9_u(p + 0x5f) == 0) || saiu == p {
                break 'c false;
            }
            if k >= ultimos && cqc9_selecao(clube) != 0 && read_usize(p + 0x1a) != read_usize(clube + 0x53)
                && read_i16(read_usize(p + 0x69) + 0xc) < 0xfa0
            {
                break 'c false;
            }
            if k >= cqc9_i(0xae2364) - 0x10 && dirigivel(clube) == 0 {
                break 'c false;
            }
            if cqc9_estado(k) & 0x30 != 0 || cqc9_com_contrato(p) != 0 || cqc9_na_lista(this, p) != 0 {
                break 'c false;
            }
            let (n1, n2) = (read_usize(clube + 0x53), read_usize(p + 0x1a));
            if n1 == 0 || n2 == 0 || (n1 != n2 && read_i16(read_usize(p + 0x69) + 0xa) < 0xea6) {
                break 'c false;
            }
            if cqc9_u(n1 + 0x75) != cqc9_u(n2 + 0x75)
                && (read_i16(read_usize(p + 0x69) + 0xc) < 0x109a || read_i16(clube + 0x80) < 0x128e)
            {
                break 'c false;
            }
            if cqc9_serve(this, p, clube, 3.5) == 0 {
                break 'c false;
            }
            if read_usize(p + 0x61) != 0 && read_usize(p + 0x39) != 0 && matches!(cqc9_u(p + 0x3d), 0xb | 0xc | 0xd | 0xf) {
                let c = contrato(read_usize(0xae1050), p);
                if c != 0 && (cqc9_u(c + 0x49) != 0 || cqc9_u(c + 0x20) == 0) {
                    break 'c false;
                }
            }
            if cqc9_selecao(clube) != 0 && cqc9_u(p + 0x3d) != 5 {
                break 'c false;
            }
            true
        };
        if ok {
            let s = cqc9_nota(this, p, clube);
            if s > melhor {
                crate::comparacao::ramo("cqc_9 candidato melhor");
                v.d(6, k); // cqc9: o candidato escolhido
                v.d(0x12, melhor);
                melhor = s;
            }
        }
        k += 1;
    }
    if v.rd(6) == -1 {
        let ajudante = read_usize(clube + 0xd3);
        let lista = read_usize(clube + 0x19f);
        if ajudante != 0 {
            v.d(6, cqc9_i(ajudante));
        } else if lista != 0 {
            v.d(6, cqc9_i(lista));
        } else {
            // de trás para a frente, a partir do 17º dos últimos (a original parte do índice do ponteiro de quem saiu)
            let mut primeira = true;
            let mut i = saiu as i32;
            let mut inicio = 0i32;
            loop {
                if primeira {
                    primeira = false;
                    i = cqc9_i(0xae2364) - 0x11;
                    inicio = i;
                } else {
                    i -= 1;
                    if i < 0 {
                        i = cqc9_i(0xae2364) - 0x10;
                    }
                    if i == inicio {
                        let fmt: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
                        fmt(0xde1f64, if segunda(clube) != 0 { 0xa04d8c } else { 0xa04d28 }, clube + 4);
                        return;
                    }
                }
                v.d(6, i);
                if cqc9_estado(i) & 0x30 != 0 || i >= cqc9_i(0xae2364) - 0x10 {
                    v.d(6, -1);
                    continue;
                }
                let p = cqc9_pessoa(i);
                if read_usize(p + 0x69) == 0 || !matches!(cqc9_u(p + 0x3d), 0 | 0xb | 0xc | 5 | 8 | 0xf | 7 | 6) {
                    v.d(6, -1);
                    continue;
                }
                if cqc9_com_contrato(p) != 0 || cqc9_na_lista(this, cqc9_pessoa(v.rd(6))) != 0 {
                    v.d(6, -1);
                    continue;
                }
                let q = cqc9_pessoa(v.rd(6));
                if read_usize(q + 0x24) != 0 && cqc9_u(q + 0x28) == 5 {
                    v.d(6, -1);
                    continue;
                }
                if cqc9_selecao(clube) == 0 {
                    let q = cqc9_pessoa(v.rd(6));
                    let c = read_usize(q + 0x39);
                    if c != 0 {
                        let na = read_usize(c + 0x53);
                        let d57 = read_usize(c + 0x57);
                        let forte = (na != 0 && cqc9_u(na + 0x11c) != 0 && d57 != 0
                            && read_usize(read_usize(0xadadfc).wrapping_add((cqc9_i(d57) as usize).wrapping_mul(4))) != 0)
                            || read_i16(c + 0x80) > 0x1964;
                        if forte && cqc9_u(q + 0x3d) == 5 {
                            v.d(6, -1);
                            continue;
                        }
                    }
                }
                v.d(0x12, 100);
                if v.rd(6) != -1 {
                    break;
                }
            }
        }
    }
    v.b(1, 0);
    v.b(0, 0);
    v.d(2, -1);
    let grande = cqc9_selecao(clube) != 0 || (read_i16(clube + 0x80) > 0x1194 && {
        let r = cqc9_sortear(0x7d0);
        read_i16(clube + 0x80) as i32 > r + 0x1194
    });
    if grande {
        let mut melhor: i32 = -1_000_000;
        let mut k = 0i32;
        while k < cqc9_i(0xae2364) {
            let r = cqc9_sortear(cqc9_i(0xae2364));
            let p = cqc9_pessoa(r);
            // ponytail: o estado lido é o do contador `k`, não o da pessoa sorteada, como na original
            if read_usize(p + 0x69) != 0 && saiu != p && cqc9_estado(k) & 0x30 == 0
                && cqc9_serve(this, p, clube, f64::from_bits(0x4009_3333_3333_3333)) != 0
                && (cqc9_i(p) < cqc9_i(0xae2364) - 0x10 || cqc9_u(p + 0x5f) != 0)
            {
                let ate: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
                if ate(0xae2c90, read_usize(p + 0x3e) as u32, read_usize(p + 0x42) as u32) >= 0x96 && cqc9_na_lista(this, p) == 0 {
                    let s = cqc9_nota(this, p, clube);
                    if s > melhor {
                        v.b(0, 1);
                        v.d(2, cqc9_i(p));
                        melhor = s;
                    }
                }
            }
            k += 1;
        }
    }
    let ultimos = cqc9_i(0xae2364) - 0x10;
    if cqc9_i(cqc9_pessoa(v.rd(6))) >= ultimos && saiu != 0 && cqc9_i(saiu) < ultimos {
        let rejeitar: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(FN_STAFF_ADD_DISLIKED_CLUB);
        rejeitar(saiu, clube, 1);
    }
    // as datas: +0x16 a chegada (dia +0x16, ano +0x18), +0x1e o anúncio (dia +0x1e, ano +0x20)
    let (dia0, ano0) = (read_i16(0xdd6ec8), read_i16(0xdd6eca) as u16);
    let (dia, ano) = (read_i16(0xae2c90), read_i16(0xae2c92) as u16);
    let antes_do_comeco = ano0 > ano || (ano0 == ano && dia0 > dia);
    if antes_do_comeco && segunda(clube) == 0 {
        v.w(0x20, ano0);
        let r = cqc9_sortear(7);
        v.w(0x1e, (r as u16).wrapping_add(dia0 as u16).wrapping_add(1));
        if v.rw(0x1e) as i16 > 0x16c {
            let r = cqc9_sortear(5);
            v.w(0x20, v.rw(0x20).wrapping_add(1));
            v.w(0x1e, r as u16);
        }
        v.w(0x18, v.rw(0x20));
        let r = cqc9_sortear(7);
        v.w(0x16, (r as u16).wrapping_add(v.rw(0x1e)).wrapping_add(1));
        if cqc9_sortear(200) == 0 {
            let r = cqc9_sortear(0x14);
            v.w(0x16, v.rw(0x16).wrapping_add(r as u16));
        }
    } else {
        v.w(0x20, ano);
        let r = cqc9_sortear(3);
        v.w(0x1e, (r as u16).wrapping_add(dia as u16).wrapping_add(1));
        if v.rw(0x1e) as i16 > 0x16c {
            let r = cqc9_sortear(5);
            v.w(0x20, v.rw(0x20).wrapping_add(1));
            v.w(0x1e, r as u16);
        }
        v.w(0x18, v.rw(0x20));
        let q = cqc9_pessoa(v.rd(6));
        let c = read_usize(q + 0x39);
        let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
        if c != 0 && humano(c, 0, 1, 0, 0) != 0 && cqc9_u(cqc9_pessoa(v.rd(6)) + 0x3d) != 5 {
            let oferta: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_MANAGER_MANAGER_POST_JOB_OFFER_NEWS);
            oferta(this, cqc9_pessoa(v.rd(6)), clube);
            let r = cqc9_sortear(5);
            v.w(0x16, (r as u16).wrapping_add(v.rw(0x1e)).wrapping_add(10));
        }
        let r = cqc9_sortear(5);
        let mais = if saiu != 0 && cqc9_i(saiu) >= cqc9_i(0xae2364) - 0x10 { 7 } else { 4 };
        v.w(0x16, (r as u16).wrapping_add(v.rw(0x1e)).wrapping_add(mais)); // cqc9: a chegada do técnico
        if cqc9_sortear(500) == 0 {
            let r = cqc9_sortear(0x14);
            v.w(0x16, v.rw(0x16).wrapping_add(r as u16));
        }
    }
    if v.rw(0x16) as i16 > 0x16c {
        let r = cqc9_sortear(5);
        v.w(0x18, v.rw(0x18).wrapping_add(1));
        v.w(0x16, (r as u16).wrapping_add(4));
    }
    let lista = read_usize(this + 8);
    let cursor = read_usize(lista + 0xa);
    let acrescentar: unsafe extern "thiscall" fn(usize, *mut u8, u32) = core::mem::transmute(FN_LLIST_ADD_TAIL);
    acrescentar(lista, v.0.as_mut_ptr(), 0x26);
    write_u32(read_usize(this + 8) + 0xa, cursor as u32);
    let tecnico: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_CLUB_MANAGER);
    if tecnico(clube) == 0 && dirigivel(clube) != 0 && cqc9_selecao(clube) == 0 {
        let ativo: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_ON_HOLIDAY);
        let quer: unsafe extern "thiscall" fn(usize, u32, usize) -> u8 = core::mem::transmute(FN_HUMAN_MANAGER_GET_HOLIDAY_OPTION);
        let candidatar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_MANAGER_MANAGER_APPLY_FOR_JOB);
        let mut i = cqc9_i(0xae2364) - 0x10;
        while i < cqc9_i(0xae2364) {
            let p = cqc9_pessoa(i);
            if cqc9_u(p + 0x5f) != 0 && ativo(read_usize(0xb63c98), p) != 0 && quer(read_usize(0xb63c98), 1, p) != 0 {
                let c = read_usize(p + 0x39);
                if c == 0 || read_i16(c + 0x80) < read_i16(clube + 0x80) {
                    candidatar(this, clube, p);
                }
            }
            i += 1;
        }
    }
}

// ponytail: acrescenta a vaga na lista [this+8] (0x68d0d0, aloca), sorteia, posta a notícia da oferta e chama as
// candidaturas (0x69ca00): não roda duas vezes. Só a original; a cópia leva os ramos e se prova pela carreira e pelas
// mutações.
pub(crate) unsafe extern "thiscall" fn comparar_vaga_de_tecnico(this: usize, clube: usize, saiu: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(1531)(this, clube, saiu)
}

/// news item 0x1775 telling the club manager that another club offered one of his staff the manager job
/// Original: `MANAGER_MANAGER::post_job_offer_news` (0x6a0540, manager_manager.cpp, thiscall).
///
/// [pt] { 0x6a0540 } (thiscall, ret 8, manager_manager.cpp, quadro SEH sem destrutor no caminho normal):
/// `MANAGER_MANAGER::post_job_offer_news(pessoa, clube)`: com a pessoa empregada (+0x39) e o clube, a partir do começo
/// do jogo ([0xdd6ec8]), a notícia 0x1775 (0x77bbe0) com o id da pessoa, os três nomes (+0x33 de +4, +8, +0xc), o id do
/// clube onde ela está, o do clube que oferece e o cargo (+0x3d), ao técnico do clube dela (0x7861e0).
pub(crate) unsafe extern "thiscall" fn noticia_da_oferta_de_emprego(_this: usize, pessoa: usize, clube: usize) {
    if pessoa == 0 || read_usize(pessoa + 0x39) == 0 || clube == 0 {
        return;
    }
    let dia = read_i16(0xdd6ec8);
    let ano = read_i16(0xdd6eca) as u16;
    let hoje = read_i16(0xae2c92) as u16;
    if ano > hoje || (ano == hoje && dia > read_i16(0xae2c90)) {
        return;
    }
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let ao_tecnico: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    crate::comparacao::ramo("cqc_9 noticia da oferta");
    let mut local = [0u32; 0x3a]; // a notícia local (0xe8 bytes)
    let n = local.as_mut_ptr() as usize;
    construir(n, 0x1775, 0);
    gravar(n, 0, read_usize(pessoa) as u32);
    gravar(n, 1, read_usize(read_usize(pessoa + 4) + 0x33) as u32);
    gravar(n, 2, read_usize(read_usize(pessoa + 8) + 0x33) as u32);
    gravar(n, 3, read_usize(read_usize(pessoa + 0xc) + 0x33) as u32);
    gravar(n, 4, read_usize(read_usize(pessoa + 0x39)) as u32);
    gravar(n, 5, read_usize(clube) as u32);
    gravar(n, 6, read_i8(pessoa + 0x3d) as i32 as u32);
    ao_tecnico(read_usize(0xb7431c), n, read_usize(pessoa + 0x39));
}

// ponytail: a notícia vai ao técnico (aloca): só a original; a cópia leva o ramo.
pub(crate) unsafe extern "thiscall" fn comparar_noticia_da_oferta_de_emprego(this: usize, pessoa: usize, clube: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(1532)(this, pessoa, clube)
}

// ---------- 263a leva: a diretoria e os jogadores depois do jogo: os jogadores que ela cita (com a confiança), o veredito da nota na partida e o texto dela (manager_manager.cpp) ----------




/// A pessoa de id `id` ([0xae23bc], 0x6e bytes).
pub(crate) unsafe fn cqa10_pessoa(id: i32) -> usize {
    read_usize(0xae23bc).wrapping_add((id as usize).wrapping_mul(0x6e))
}

/// O registro da diretoria do clube na gerência dos técnicos ([this] + id × 0x49).
pub(crate) unsafe fn cqa10_diretoria(this: usize, clube: usize) -> usize {
    read_usize(this).wrapping_add(read_usize(clube).wrapping_mul(0x49))
}

/// after the match, by a draw, the league position for the news and the manager's favoured or rejected players who
/// played badly, with the board confidence rising or falling by them
/// Original: `board_named_players_after_match` (0x6aaee0, manager_manager.cpp, thiscall).
///
/// [pt] { 0x6aaee0 } (thiscall, ret 0x28, manager_manager.cpp): pela notícia da diretoria depois do jogo (0x6a5fa0):
/// `(jogo, clube, ids, partida, elogio, a6..a10)`. Zera os 3 ids (−1). Clube fora das seleções com técnico (+0xbf) de
/// ficha (+0x69): sorteio(+0x16 da ficha) contra sorteio(10); passando, zera as 5 saídas e, no jogo da divisão do clube,
/// a10 = 1 e, pela tabela da liga (0x4a4850, com mais de 8 jogos), a9/a8 (+0x74/+0x73 da linha), a7 (+0x2c da diretoria)
/// e a6 (2 ou 0xfe pelos +0x19/+0x2d da linha). Os jogadores (cargo 11, com ficha) que o técnico rejeita (0x5480f0, se
/// `elogio` = 0) ou prefere (0x548090), do clube pelo contrato (0x4cf5a0, em [0xae1080]), abaixo da reputação do clube
/// (rejeitados) ou até 35 anos e acima dela − 750 (preferidos), com nota abaixo de 7 na partida (0x6ab670 = −1): os ids
/// vão em `ids` e a confiança da diretoria (+6) sobe (preferidos) ou cai (rejeitados) 10 × o +0x57 do técnico. Devolve
/// a soma das duas contas (o chamador não lê).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn jogadores_que_a_diretoria_cita(this: usize, jogo: usize, clube: usize, ids: *mut i32, partida: u32, elogio: *const u8,
                                                         a6: *mut u8, a7: *mut u8, a8: *mut u8, a9: *mut u8, a10: *mut u8) -> u8 {
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    if !ids.is_null() {
        for k in 0..3 {
            *ids.add(k) = -1;
        }
    }
    if clube == 0 {
        return 0;
    }
    let t = read_usize(clube + 0xbf);
    if t == 0 || read_usize(t + 0x69) == 0 || selecao(clube) != 0 {
        return 0;
    }
    crate::comparacao::ramo("cqa_10 a diretoria sorteia");
    let r1 = sorteio(read_i8(read_usize(t + 0x69) + 0x16) as i32); // cqa10: o sorteio pela ficha do técnico
    let r2 = sorteio(10); // cqa10: o sorteio da diretoria
    if r1 < r2 {
        return 0;
    }
    *a10 = 0;
    *a6 = 0;
    *a7 = 0;
    *a8 = 0;
    *a9 = 0;
    let comp = read_usize(jogo + 0x14);
    if comp != 0 && read_usize(clube + 0x57) == comp {
        *a10 = 1;
        let div = read_usize(clube + 0x57);
        let c = read_usize(read_usize(0xadadfc).wrapping_add(read_usize(div).wrapping_mul(4)));
        if c != 0 && read_usize(c + 8) != 0 {
            let busca: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_COMP_STATS_FIND_CLUB_ENTRY);
            let r = busca(read_usize(c + 8), clube);
            if r != 0 && *((r + 4) as *const u8) > 8 {
                *a9 = *((r + 0x74) as *const u8);
                *a8 = *((r + 0x73) as *const u8);
                *a7 = *((cqa10_diretoria(this, clube) + 0x2c) as *const u8);
                if read_usize(r + 0x19) == 2 {
                    *a6 = 2;
                } else if read_usize(r + 0x2d) == 2 {
                    *a6 = 0xfe;
                }
            }
        }
    }
    // a original guarda os escolhidos em 3 dwords logo antes da lista: m[0..3] são os escolhidos, m[3..] a lista
    let mut m = [0usize; 53];
    let mut n: i8 = 0;
    let (mut da, mut d9) = (0i8, 0i8);
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let veredito: unsafe extern "thiscall" fn(usize, usize, u32) -> u8 = core::mem::transmute(FN_GET_PLAYER_MATCH_RATING_VERDICT);
    let preferidos = *elogio != 0;
    if preferidos {
        let f: unsafe extern "C" fn(usize, usize, *mut i8) -> usize = core::mem::transmute(FN_STAFF_GET_FAVOURED_STAFF);
        f(t, m.as_mut_ptr().add(3) as usize, &mut n);
    } else {
        let f: unsafe extern "C" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_STAFF_GET_DISLIKED_STAFF);
        f(t, m.as_mut_ptr().add(3) as usize, &mut n as *mut i8 as usize);
    }
    let mut k = 0i8;
    while k < n {
        let p = m[3 + k as usize];
        k += 1;
        if p == 0 || read_usize(p + 0x61) == 0 || read_i8(p + 0x3d) != 0xb {
            continue;
        }
        let c = contrato_de(read_usize(0xae1050), p);
        *(0xae1080 as *mut usize) = c;
        if c == 0 {
            continue;
        }
        let cl = read_usize(0xae23b4).wrapping_add(read_usize(c + 4).wrapping_mul(0x245));
        if cl == 0 || cl != clube {
            continue;
        }
        let nota = read_i16(read_usize(p + 0x61) + 0xb);
        let rep = read_i16(clube + 0x80);
        let passa = if preferidos { read_i8(p + 0x18) < 0x24 && nota as i32 > rep as i32 - 0x2ee } else { nota < rep };
        if passa && veredito(this, p, partida) == 0xff {
            let i = if preferidos { &mut da } else { &mut d9 };
            m[*i as usize] = p;
            *i += 1;
        }
    }
    if !ids.is_null() {
        for i in 0..da.max(0) as usize {
            *ids.add(i) = read_usize(m[i]) as i32;
        }
        let base = da.max(0) as usize;
        for j in 0..d9.max(0) as usize {
            *ids.add(base + j) = read_usize(m[j]) as i32;
        }
    }
    let x = read_i8(read_usize(clube + 0xbf) + 0x57) as i16;
    let conf = (cqa10_diretoria(this, clube) + 6) as *mut i16;
    if da != 0 {
        crate::comparacao::ramo("cqa_10 diretoria elogia");
        core::ptr::write_unaligned(conf, core::ptr::read_unaligned(conf).wrapping_add(x.wrapping_mul(10))); // cqa10: a confiança pelos preferidos
    } else if d9 != 0 {
        crate::comparacao::ramo("cqa_10 diretoria culpa");
        core::ptr::write_unaligned(conf, core::ptr::read_unaligned(conf).wrapping_sub(x.wrapping_mul(10))); // cqa10: a confiança pelos rejeitados
    }
    (d9 as u8).wrapping_add(da as u8)
}

/// Comparador: lado a lado a partir do mesmo estado do sorteador, do [0xae1080], do registro da diretoria do clube, dos
/// 3 ids e das 5 saídas; a devolução também (a lista do técnico, a tabela da liga e o registro da partida só leem). A
/// cópia sem contar os sorteios.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn comparar_jogadores_que_a_diretoria_cita(this: usize, jogo: usize, clube: usize, ids: *mut i32, partida: u32, elogio: *const u8,
                                                                  a6: *mut u8, a7: *mut u8, a8: *mut u8, a9: *mut u8, a10: *mut u8) -> u8 {
    type F = unsafe extern "thiscall" fn(usize, usize, usize, *mut i32, u32, *const u8, *mut u8, *mut u8, *mut u8, *mut u8, *mut u8) -> u8;
    let o = orig::<F>(1446);
    let mut regioes = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    regioes.push((0xae1080, 4));
    if clube != 0 {
        regioes.push((cqa10_diretoria(this, clube), 0x49));
    }
    if !ids.is_null() {
        regioes.push((ids as usize, 12));
    }
    for s in [a6, a7, a8, a9, a10] {
        regioes.push((s as usize, 1));
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, jogo, clube, ids, partida, elogio, a6, a7, a8, a9, a10),
        || crate::aleatorio::sem_contar(|| jogadores_que_a_diretoria_cita(this, jogo, clube, ids, partida, elogio, a6, a7, a8, a9, a10)));
    PLACAR.registrar(igual && r == n, || format!("jogadores_que_a_diretoria_cita(clube {:x}, elogio {}) original={} rust={} {:x?} x {:x?}", clube, *elogio, r, n, mo, mr));
    r
}

/// finds the player among the starters of the match record: −1 for a rating under 7, 1 above, else 0
/// Original: `get_player_match_rating_verdict` (0x6ab670, manager_manager.cpp, thiscall).
///
/// [pt] { 0x6ab670 } (thiscall, ret 8, manager_manager.cpp, quadro SEH do vetor local): `get_player_match_rating_verdict(
/// pessoa, partida)`: lê o registro da partida (0x729600 em [0xb6b7c0], com os dois lados de 0x625 bytes construídos e
/// destruídos pelos iteradores 0x9463af/0x9462bb) e acha a pessoa entre os 11 primeiros de cada lado (ids em +0x71, 0x49
/// bytes): a nota (+0x35) abaixo de 7 dá −1, acima de 7 dá 1, 7 dá 0; não achada (ou sem pessoa), 0. O ecx não conta.
pub(crate) unsafe extern "thiscall" fn veredito_da_nota_na_partida(_this: usize, pessoa: usize, partida: u32) -> u8 {
    let construir: unsafe extern "stdcall" fn(usize, usize, u32, usize, usize) = core::mem::transmute(FN_CRT_VECTOR_CONSTRUCTOR_ITERATOR);
    let destruir: unsafe extern "stdcall" fn(usize, usize, u32, usize) = core::mem::transmute(FN_CRT_VECTOR_DESTRUCTOR_ITERATOR);
    let ler: unsafe extern "thiscall" fn(usize, u32, usize) -> u32 = core::mem::transmute(FN_MATCH_STATS_READ_MATCH_RECORD);
    let mut registro = [0u32; 0x360]; // 0xd7a bytes do registro da partida
    let r = registro.as_mut_ptr() as usize;
    let lados = r + 0x11f;
    construir(lados, 0x625, 2, 0x453c30, 0x453c10);
    let mut v = 0u8;
    if pessoa != 0 && ler(read_usize(0xb6b7c0), partida, r) != 0 {
        'busca: for lado in 0..2usize {
            for k in 0..20usize {
                let s = lados + lado * 0x625 + 0x71 + k * 0x49;
                let id = read_usize(s) as i32;
                if id != -1 && pessoa == cqa10_pessoa(id) && k < 11 {
                    let nota = *((s + 0x35) as *const u8);
                    v = if nota < 7 { 0xff } else if nota > 7 { 1 } else { 0 };
                    break 'busca;
                }
            }
        }
    }
    destruir(lados, 0x625, 2, 0x453c10);
    v
}

/// Comparador: pelo valor (o registro é lido do arquivo temporário e o vetor local se constrói e destrói nos dois).
pub(crate) unsafe extern "thiscall" fn comparar_veredito_da_nota_na_partida(this: usize, pessoa: usize, partida: u32) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u8>(1447)(this, pessoa, partida);
    let n = veredito_da_nota_na_partida(this, pessoa, partida);
    PLACAR.registrar(r == n, || format!("veredito_da_nota_na_partida({:x}, partida {}) original={} rust={}", pessoa, partida as i32, r as i8, n as i8));
    r
}

/// the board sentence praising or blaming the named player, by variant
/// Original: `build_board_player_performance_text` (0x6ab2d0, manager_manager.cpp, stdcall).
///
/// [pt] { 0x6ab2d0 } (stdcall, ret 0x10, manager_manager.cpp): `build_board_player_performance_text(ids, vitória, saída,
/// variante)`, pelo texto da diretoria (0x6a69c0). A original guarda só o primeiro id que não é −1 (no lugar dele, 0, 1
/// ou 2), conta 1 e os textos leem sempre o lugar 0. Vitória: com o primeiro, o nome (0x53dba0, 200, estilo 1) num dos
/// três elogios pela variante (0, 1, outra); depois, como nenhum jogador conta na derrota, a saída fica vazia (a original
/// zera o primeiro byte). Derrota: a saída vazia e, com o primeiro, uma das quatro culpas pela variante (0, 1, 2,
/// outra), com 100 e o nome. Sem ids, nada.
pub(crate) unsafe extern "stdcall" fn texto_da_diretoria_sobre_os_jogadores(ids: *const i32, vitoria: u32, saida: usize, variante: u32) {
    if ids.is_null() {
        return;
    }
    let (i0, i1, i2) = (*ids, *ids.add(1), *ids.add(2));
    if i0 == -1 && (i1 != -1 || i2 != -1) {
        // ponytail: a original lê o lugar 0 sem valor (a pilha): esse caso fica com ela
        return orig::<unsafe extern "stdcall" fn(*const i32, u32, usize, u32)>(1448)(ids, vitoria, saida, variante);
    }
    let nome: unsafe extern "C" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_DATABASE_FORMAT_STAFF_NAME);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let mut buf = [0u8; 0xc8];
    let b = buf.as_mut_ptr() as usize;
    crate::comparacao::ramo("cqa_10 texto da diretoria");
    if vitoria as u8 != 0 {
        if i0 != -1 {
            nome(cqa10_pessoa(i0), b, 0xc8, 1);
            let f = match variante {
                0 => 0xa12558usize,
                1 => 0xa125ac,
                _ => 0xa125f8,
            };
            formatar(saida, f, b);
        }
        *(saida as *mut u8) = 0;
    } else {
        *(saida as *mut u8) = 0;
        if i0 != -1 {
            nome(cqa10_pessoa(i0), b, 0xc8, 1);
            let f = match variante {
                0 => 0xa12294usize,
                1 => 0xa12310,
                2 => 0xa12384,
                _ => 0xa123ec,
            };
            formatar(saida, f, 0x64u32, b);
        }
    }
}

/// Comparador: lado a lado nos primeiros 0x200 bytes da saída (o único chamador, 0x6a69c0, passa um texto de 1500).
pub(crate) unsafe extern "stdcall" fn comparar_texto_da_diretoria_sobre_os_jogadores(ids: *const i32, vitoria: u32, saida: usize, variante: u32) {
    let o = orig::<unsafe extern "stdcall" fn(*const i32, u32, usize, u32)>(1448);
    if ids.is_null() || (*ids == -1 && (*ids.add(1) != -1 || *ids.add(2) != -1)) {
        return o(ids, vitoria, saida, variante);
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(saida, 0x200)], || o(ids, vitoria, saida, variante),
        || texto_da_diretoria_sobre_os_jogadores(ids, vitoria, saida, variante));
    PLACAR.registrar(igual, || format!("texto_da_diretoria_sobre_os_jogadores(vitória {}, variante {}) {:?} x {:?}", vitoria as u8, variante,
        String::from_utf8_lossy(&mo.0[0].1[..0x40]), String::from_utf8_lossy(&mr.0[0].1[..0x40])));
}

// ---------- 261a leva: o veto dos fisioterapeutas e da diretoria à transferência, a notícia do ultimato da diretoria e o texto dela (manager_manager.cpp) ----------




/// O registro de clube de id `id` ([0xae23b4], 0x245 bytes).
pub(crate) unsafe fn cpy12_clube(id: usize) -> usize {
    read_usize(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// O registro da diretoria do clube na gerência dos técnicos ([this] + id × 0x49).
pub(crate) unsafe fn cpy12_diretoria(this: usize, clube: usize) -> usize {
    read_usize(this).wrapping_add(read_usize(clube).wrapping_mul(0x49))
}

/// Soma `delta` à palavra `o` (+6, +0x14, +0x16: as confianças) da diretoria do clube.
pub(crate) unsafe fn cpy12_confianca(this: usize, clube: usize, o: usize, delta: i16) {
    let a = (cpy12_diretoria(this, clube) + o) as *mut i16;
    core::ptr::write_unaligned(a, core::ptr::read_unaligned(a).wrapping_add(delta));
}

/// O piso da confiança +6 em 1 e o veto 0x800.
pub(crate) unsafe fn cpy12_piso(this: usize, clube: usize) -> u32 {
    let a = (cpy12_diretoria(this, clube) + 6) as *mut i16;
    if core::ptr::read_unaligned(a) < 1 {
        core::ptr::write_unaligned(a, 1);
    }
    0x800
}

/// O valor do jogador (CONTRACT_MANAGER::calc_player_value 0x4d0c60 em [0xae1050] com 0, −1 e `modo`).
pub(crate) unsafe fn cpy12_valor(j: usize, clube: usize, modo: u32) -> i32 {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, i32, u32) -> i32 = core::mem::transmute(FN_CONTRACT_MANAGER_CALC_PLAYER_VALUE);
    f(read_usize(0xae1050), j, clube, 0, -1, modo)
}

/// O clube do contrato do jogador (0x4cf5a0, guardado em [0xae1080]) ou 0.
pub(crate) unsafe fn cpy12_clube_do_contrato(j: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = f(read_usize(0xae1050), j);
    *(0xae1080 as *mut usize) = c;
    if c == 0 { 0 } else { cpy12_clube(read_usize(c + 4)) }
}

/// Se a cópia semeou o rand da CRT pelo relógio (os fisioterapeutas avaliaram): o comparador não confere o estado do
/// rand (o relógio anda entre a original e a cópia no jogo real).
pub(crate) static mut CPY12_SEMEOU: bool = false;

/// when buying an injured player, the physios draw the veto; at a human club, the board vetoes on budget, asking price
/// or fee and loses confidence
/// Original: `check_board_transfer_veto` (0x696b10, manager_manager.cpp, thiscall).
///
/// [pt] { 0x696b10 } (thiscall, ret 0x10, manager_manager.cpp): `MANAGER_MANAGER::check_board_transfer_veto(jogador, clube,
/// valor, comprando)`, pela 0x8cdb80 (oferta sem troca). Comprando um jogador com lesão (0x62ed70) de gravidade +0x28 > 20,
/// cada um dos três fisioterapeutas do clube (+0x1cf, com ficha +0x69) semeia o rand da CRT pelos dois ids e sorteia:
/// aposentadoria anunciada pela lesão (0x7a7ed0) contra a nota dele (0x544410 do byte +0x1e), a gravidade contra
/// sorteio(200) + 25 e (41 − nota) × 5, a recaída (0x62ece0), a nota³ + 100 (pow 0x945ff0) e a nota contra a do jogador
/// (+0x61 +0x2d): qualquer um veta (0x2000); no fim o rand volta ao relógio (0x946ebb). Depois, só com contrato fora dos
/// tipos 1 e 3, clube atual (+0x39) e clube humano (0x600a50): o limite (reputação +0x80 × 0,2)² entre 150 mil e 2
/// milhões. Vendendo: liga com divisão e elenco abaixo de 15, a diretoria perde 1250/1250/500 (+6/+0x14/+0x16, piso 1 em
/// +6) e veta 0x200; senão o pedido (0x4d1600, no máximo o valor pelo clube do contrato) acima do limite e o valor pelo
/// clube atual não abaixo do `valor`: pedido > 2 × valor com +0x3d do contrato > 25, ou pedido > 5 milhões com +0x3d > 0 e
/// > 2,5 × valor, perdem 1250/1250/500; > 6 × valor, 500/500/250; veto 0x800. Comprando: valor acima de 1,2 × a verba
/// (0x598800) veta 0x800; acima do limite, o menor valor (pelo clube e pelo do contrato) + 0,65 × limite + a faixa de
/// riqueza (0x5986b0: 1,25 milhão, 650 mil ou 65 mil) truncado abaixo de 0,275 × valor perde 250/500/500 e veta 0x800.
/// Senão 0.
pub(crate) unsafe extern "thiscall" fn veto_dos_fisios_e_da_diretoria(this: usize, j: usize, clube: usize, valor: i32, comprando: u32) -> u32 {
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let semear: unsafe extern "C" fn(u32) = core::mem::transmute(FN_CRT_SRAND);
    let relogio: unsafe extern "C" fn(*mut i32) -> i32 = core::mem::transmute(FN_946EBB);
    let nota: unsafe extern "C" fn(usize, u32) -> i8 = core::mem::transmute(FN_NON_PLAYER_ATTRIBUTE_RATING);
    let aposentadoria: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_PLAYER_REGEN_FIND_RETIREMENT_ENTRY);
    let lesao: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_INJURY_GET_PLAYER_INJURY);
    let recaida: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_INJURY_GET_RECURRING_INJURY_LEVEL);
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let humano: unsafe extern "C" fn(usize, u32, u32, usize, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    CPY12_SEMEOU = false;
    if comprando as u8 != 0 {
        let l = lesao(read_usize(0xb63d28), j);
        if l != 0 && read_i16(l + 0x28) > 0x14 {
            let mut semeou = false;
            for k in 0..3usize {
                let f = read_usize(clube + 0x1cf + k * 4);
                if f == 0 || read_usize(f + 0x69) == 0 {
                    continue;
                }
                semear((read_usize(j) as u32).wrapping_add(1).wrapping_mul((read_usize(f) as u32).wrapping_add(1)));
                semeou = true;
                CPY12_SEMEOU = true;
                crate::comparacao::ramo("cpy_12 fisioterapeuta avalia");
                let np = read_usize(f + 0x69);
                // a original empurra o registrador inteiro com o byte +0x1e no baixo; a 0x544410 só lê o byte
                let n = || nota(np, read_u8(np + 0x1e) as u32) as i32;
                if aposentadoria(read_usize(0xb7434c), j, 1) != 0 {
                    if sorteio(n()) != 0 {
                        return 0x2000;
                    }
                    if aposentadoria(read_usize(0xb7434c), j, 0) != 0 {
                        return 0x2000;
                    }
                }
                let r = sorteio(200); // cpy12: o sorteio da gravidade
                if read_i16(l + 0x28) as i32 > r.wrapping_add(0x19) {
                    let a = sorteio((0x29 - n()) * 5);
                    let b = sorteio(read_i16(l + 0x28) as i32);
                    if a < b {
                        return 0x2000;
                    }
                }
                if recaida(read_usize(0xb63d28), j) != 0 {
                    let c = n();
                    let rr = recaida(read_usize(0xb63d28), j) as i8 as i32;
                    let a = sorteio(c + rr);
                    let b = sorteio(10);
                    if a > b {
                        return 0x2000;
                    }
                }
                let c = n();
                if sorteio(ftol(pow_do_jogo(c as f64, f64_em(0x968930)) + f64_em(0x9688a8))) == 0 {
                    return 0x2000;
                }
                let a = sorteio(n());
                let b = sorteio(200);
                if a > b {
                    let a = sorteio(read_u8(read_usize(j + 0x61) + 0x2d) as i8 as i32);
                    let b = sorteio(200);
                    if a > b {
                        return 0x2000;
                    }
                }
            }
            if semeou {
                let mut t = 0i32;
                semear(relogio(&mut t) as u32);
            }
        }
    }
    let c = contrato(read_usize(0xae1050), j);
    if c == 0 {
        return 0;
    }
    let tipo = read_u8(c + 0x35) & 0x3f;
    if tipo == 1 || tipo == 3 || read_usize(j + 0x39) == 0 || humano(clube, 1, 1, 0, 0) == 0 {
        return 0;
    }
    let x = read_i16(clube + 0x80) as f64 * f64_em(0x968918);
    let mut limite = ftol(x * x);
    if limite < 0x249f0 {
        limite = 0x249f0;
    } else if limite > 0x1e8480 {
        limite = 0x1e8480;
    }
    if comprando as u8 == 0 {
        let contar: unsafe extern "C" fn(usize, u32, u32) -> u8 = core::mem::transmute(FN_DATABASE_COUNT_CLUB_SQUAD);
        let nacao = read_usize(clube + 0x53);
        let divisao = read_usize(clube + 0x57);
        if nacao != 0 && read_u8(nacao + 0x11c) != 0 && divisao != 0 && read_usize(divisao) != read_usize(KEY_CLUB_COMP_A_LOWER_DIVISION) && (contar(clube, 0, 0) as i8) < 0xf {
            cpy12_confianca(this, clube, 6, -0x4e2);
            cpy12_confianca(this, clube, 0x14, -0x4e2);
            cpy12_confianca(this, clube, 0x16, -500);
            cpy12_piso(this, clube);
            return 0x200;
        }
        let preco: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, u32) -> i32 = core::mem::transmute(FN_CONTRACT_MANAGER_CALC_ASKING_PRICE);
        let mut pedido = preco(read_usize(0xae1050), read_usize(j + 0x39), clube, j, 0, 0, 1);
        let pelo_contrato = cpy12_valor(j, cpy12_clube_do_contrato(j), 1);
        if pedido > pelo_contrato {
            pedido = pelo_contrato;
        }
        let pelo_atual = cpy12_valor(j, read_usize(j + 0x39), 0);
        if pedido <= limite || pelo_atual < valor {
            return 0;
        }
        let (p, v) = (pedido as f64, valor as f64);
        let mais = read_u8(c + 0x3d) as i8;
        crate::comparacao::ramo("cpy_12 diretoria vendendo");
        if p > v + v && mais > 0x19 {
            cpy12_confianca(this, clube, 6, -0x4e2);
            cpy12_confianca(this, clube, 0x14, -0x4e2);
        } else if pedido > 0x4c4b40 && mais > 0 && p > v * f64_em(0x968e18) {
            cpy12_confianca(this, clube, 6, -0x4e2);
            cpy12_confianca(this, clube, 0x14, -0x4e2);
        } else if p > v * f64_em(0x969000) {
            cpy12_confianca(this, clube, 6, -500);
            cpy12_confianca(this, clube, 0x14, -500);
            cpy12_confianca(this, clube, 0x16, -0xfa);
            return cpy12_piso(this, clube);
        } else {
            return 0;
        }
        cpy12_confianca(this, clube, 0x16, -500);
        return cpy12_piso(this, clube);
    }
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FINANCETABLE_GET_CLUB_FINANCE);
    let verba: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) -> i64 = core::mem::transmute(FN_FINANCE_GET_TRANSFER_BUDGET);
    let orcamento = verba(financas(read_usize(0xae2a38), clube), j, 0, 0, 1, 0);
    if valor as i64 > orcamento.wrapping_mul(12) / 10 { // cpy12: a verba da diretoria
        crate::comparacao::ramo("cpy_12 diretoria sem verba");
        return 0x800;
    }
    if valor <= limite {
        return 0;
    }
    let pelo_clube = cpy12_valor(j, clube, 0);
    let pelo_contrato = cpy12_valor(j, cpy12_clube_do_contrato(j), 1);
    let menor = if pelo_clube > pelo_contrato { pelo_contrato } else { pelo_clube };
    let fin = financas(read_usize(0xae2a38), clube);
    if fin == 0 {
        return 0;
    }
    let faixa: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(FN_FINANCE_GET_WEALTH_BAND);
    let k = match faixa(fin) {
        2 => f64_em(0x96d1f8),
        1 => f64_em(0x96d1f0),
        _ => f64_em(0x968ea0),
    };
    let teto = ftol(limite as f64 * f64_em(0x96a5d0) + menor as f64 + k);
    if !(valor as f64 * f64_em(0x96d1e8) > teto as f64) {
        return 0;
    }
    crate::comparacao::ramo("cpy_12 diretoria acha caro");
    cpy12_confianca(this, clube, 6, -0xfa);
    cpy12_confianca(this, clube, 0x14, -500);
    cpy12_confianca(this, clube, 0x16, -500);
    cpy12_piso(this, clube)
}

/// Comparador: lado a lado a partir do mesmo estado do registro da diretoria do clube, do contrato guardado em
/// [0xae1080] e do sorteio; a devolução e tudo isso têm de bater (o rand da CRT, só sem a semente do relógio).
/// Comprando, a primeira coisa das duas é a lesão (0x62ed70), que antes põe em dia a lesão do jogador (0x62c680: uma
/// vez por dia, pode sortear e grava a data no registro); a original, rodando primeiro, deixava-a em dia e a cópia não
/// sorteava (um sorteio a menos). O comparador faz antes essa mesma atualização (a do jogo, no mesmo ponto: nada antes
/// dela sorteia), e as duas partem da lesão já em dia.
pub(crate) unsafe extern "thiscall" fn comparar_veto_dos_fisios_e_da_diretoria(this: usize, j: usize, clube: usize, valor: i32, comprando: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, i32, u32) -> u32>(1374);
    if comprando as u8 != 0 && j != 0 {
        // ecx = [0xb63d28]; push pessoa -> thiscall, ret 4 (como em 0x62edfc); confere id e ficha por dentro
        let em_dia: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_INJURY_UPDATE_PLAYER_FITNESS);
        em_dia(read_usize(0xb63d28), j);
    }
    let regioes = [(cpy12_diretoria(this, clube), 0x49), (0xae1080, 4), (0xde1f5c, 8), (0xad6c30, 4)];
    let (r, n, _, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, j, clube, valor, comprando),
        || crate::aleatorio::sem_contar(|| veto_dos_fisios_e_da_diretoria(this, j, clube, valor, comprando)));
    let partes = if CPY12_SEMEOU { 3 } else { 4 };
    let igual = (0..partes).all(|i| mo.0[i] == mr.0[i]);
    PLACAR.registrar(igual && r == n, || format!("veto_dos_fisios_e_da_diretoria(clube {:x}, valor {}, comprando {}) original={:#x} rust={:#x} {:x?} x {:x?}", clube, valor, comprando as u8, r, n, mo, mr));
    r
}

/// the 0x178d news of the ultimatum before the match, to the club manager or, by a draw, to those of the division or
/// country
/// Original: `post_board_ultimatum_news` (0x6ac700, manager_manager.cpp, thiscall).
///
/// [pt] { 0x6ac700 } (thiscall, ret 0xc, manager_manager.cpp, quadro SEH da notícia local): `MANAGER_MANAGER::
/// post_board_ultimatum_news(clube, adversário, competição)`, pelo ultimato da diretoria (0x6ac560). Clube com divisão
/// (+0x57), técnico (+0xcf) e a competição: a notícia 0x178d com o técnico (o id e as três chaves), o clube, o adversário,
/// a competição e um sorteio(5); clube de reputação (+0x80) abaixo de 5750, ao técnico do clube (0x7861e0); senão, por
/// sorteio(3), aos técnicos da divisão (0x7862d0) ou aos do país (+0x53, 0x7863f0).
pub(crate) unsafe extern "thiscall" fn noticia_do_ultimato(_this: usize, clube: usize, adversario: usize, comp: usize) {
    if clube == 0 || read_usize(clube + 0x57) == 0 || read_usize(clube + 0xcf) == 0 || comp == 0 {
        return;
    }
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let ao_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    let a_divisao: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_DIVISION_MANAGERS);
    let ao_pais: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_NATION_MANAGERS);
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    construir(nt, 0x178d, 0);
    let t = read_usize(clube + 0xcf);
    gravar(nt, 0, read_usize(t) as u32);
    gravar(nt, 1, read_usize(read_usize(t + 4) + 0x33) as u32);
    gravar(nt, 2, read_usize(read_usize(t + 8) + 0x33) as u32);
    gravar(nt, 3, read_usize(read_usize(t + 0xc) + 0x33) as u32);
    gravar(nt, 4, read_usize(clube) as u32);
    gravar(nt, 5, read_usize(adversario) as u32);
    gravar(nt, 6, read_usize(comp) as u32);
    crate::comparacao::ramo("cpy_12 noticia do ultimato");
    gravar(nt, 7, sorteio(5) as u32); // cpy12: o sorteio do ultimato
    if read_i16(clube + 0x80) < 0x1676 {
        ao_clube(read_usize(0xb7431c), nt, clube);
    } else if sorteio(3) != 0 {
        a_divisao(read_usize(0xb7431c), nt, read_usize(clube + 0x57));
    } else {
        ao_pais(read_usize(0xb7431c), nt, read_usize(clube + 0x53));
    }
}

// ponytail: a notícia vai às caixas dos técnicos (aloca) e sorteia: só a original; a cópia leva o ramo.
pub(crate) unsafe extern "thiscall" fn comparar_noticia_do_ultimato(this: usize, clube: usize, adversario: usize, comp: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(1375)(this, clube, adversario, comp)
}

/// the headline or body of the ultimatum news, with the names and links of the manager and the clubs
/// Original: `news_board_ultimatum_text` (0x6ac880, manager_manager.cpp, stdcall).
///
/// [pt] { 0x6ac880 } (stdcall, ret 0xc, manager_manager.cpp): `news_board_ultimatum_text(notícia, corpo, saída)`: lê os
/// campos 0 a 7 da notícia (0x785830: o técnico, as três chaves, o clube, o adversário, a competição); o nome do técnico
/// (0x53dba0, 500, estilo 1). Corpo: o texto 0xa12dac em 0xde1f64 (0x66f4e0) com os gêneros (+0x37 dos clubes, +0x52 da
/// competição), os nomes (+4 dos clubes, +0x38 da competição) e o do técnico, na `saída` (0x4ae660), e as ligações
/// (0x4ae8a0): o técnico (0x7da, se os nomes conferem pela 0x53e990), o clube e o adversário (0x7d5). Título: 0xa12d64
/// com o gênero (+0x52) e o nome curto (+0x38) do clube.
pub(crate) unsafe extern "stdcall" fn texto_do_ultimato(noticia: usize, corpo: u32, saida: usize) {
    let campo: unsafe extern "thiscall" fn(usize, i8) -> u32 = core::mem::transmute(FN_NEWSITEM_GET_SLOT);
    let nome_da_pessoa: unsafe extern "C" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_DATABASE_FORMAT_STAFF_NAME);
    let confere: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_DATABASE_STAFF_MATCHES_NAMES);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let por_texto: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TEXT_SET_STRING);
    let ligar: unsafe extern "thiscall" fn(usize, usize, u32, usize) = core::mem::transmute(FN_TEXT_ADD_LINK);
    crate::comparacao::ramo("cpy_12 texto do ultimato");
    let v: [u32; 7] = core::array::from_fn(|k| campo(noticia, k as i8));
    campo(noticia, 7);
    let pessoa = read_usize(0xae23bc).wrapping_add((v[0] as usize).wrapping_mul(0x6e));
    let clube = cpy12_clube(v[4] as usize);
    let adversario = cpy12_clube(v[5] as usize);
    let comp = read_usize(0xae23d0).wrapping_add((v[6] as usize).wrapping_mul(0x6b));
    let mut nome = [0u8; 0x200];
    let buf = nome.as_mut_ptr() as usize;
    nome_da_pessoa(pessoa, buf, 0x1f4, 1);
    if corpo != 0 {
        let gc = read_u8(comp + 0x52) as i8 as i32;
        let ga = read_u8(adversario + 0x37) as i8 as i32;
        let gl = read_u8(clube + 0x37) as i8 as i32;
        formatar(0xde1f64, 0xa12dac, gl, gl, ga, ga, gc, gc, clube + 4, buf, adversario + 4, comp + 0x38);
        por_texto(saida, 0xde1f64);
        if confere(pessoa, v[1], v[2], v[3], 0) != 0 {
            ligar(saida, buf, 0x7da, pessoa);
        }
        ligar(saida, clube + 4, 0x7d5, clube);
        ligar(saida, adversario + 4, 0x7d5, adversario);
    } else {
        let g = read_u8(clube + 0x52) as i8 as i32;
        formatar(0xde1f64, 0xa12d64, g, g, clube + 0x38); // "... manager given ultimatum by board"
        por_texto(saida, 0xde1f64);
    }
}

/// Comparador: o texto vai para o objeto de saída (0x4ae660/0x4ae8a0 acrescentam e alocam): roda só a original.
pub(crate) unsafe extern "stdcall" fn comparar_texto_do_ultimato(noticia: usize, corpo: u32, saida: usize) {
    // ponytail: o objeto de texto não pode receber o texto duas vezes; só a original roda aqui
    orig::<unsafe extern "stdcall" fn(usize, u32, usize)>(1376)(noticia, corpo, saida);
}

// ---------- 260a leva: as notícias do técnico: a diretoria pensa no cargo do técnico, a saída do técnico e o técnico novo (manager_manager.cpp) ----------


pub(crate) unsafe fn cpx10_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}



pub(crate) unsafe fn cpx10_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    f(n)
}

/// A data das notícias de técnico ([0xdd6ec8]: dia com sinal em +0, ano sem sinal em +2) ainda não chegou (é depois de
/// hoje, [0xae2c90])? Então nenhuma notícia sai.
pub(crate) unsafe fn cpx10_antes_da_data() -> bool {
    let (dia, ano) = (read_i16(0xdd6ec8), read_i16(0xdd6eca) as u16);
    let hoje = read_i16(0xae2c92) as u16;
    ano > hoje || (ano == hoje && dia > read_i16(0xae2c90))
}

/// A linha do clube na tabela da diretoria (0x49 bytes por id, a partir de [this]).
pub(crate) unsafe fn cpx10_linha(this: usize, clube: usize) -> usize {
    read_usize(this).wrapping_add(cpx10_i(clube).wrapping_mul(0x49) as usize)
}

pub(crate) unsafe fn cpx10_gravar(n: usize, k: u32, v: u32) {
    let f: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    f(n, k, v);
}

/// O id e as três chaves (+0x33 de +4, +8 e +0xc) da pessoa `p` nos dados `k`..`k`+3 da notícia.
pub(crate) unsafe fn cpx10_gravar_pessoa(n: usize, k: u32, p: usize) {
    cpx10_gravar(n, k, read_usize(p) as u32);
    cpx10_gravar(n, k + 1, read_usize(read_usize(p + 4) + 0x33) as u32);
    cpx10_gravar(n, k + 2, read_usize(read_usize(p + 8) + 0x33) as u32);
    cpx10_gravar(n, k + 3, read_usize(read_usize(p + 0xc) + 0x33) as u32);
}

/// O técnico humano (uma das 16 últimas pessoas: id ≥ [0xae2364] − 16)?
pub(crate) unsafe fn cpx10_humano(p: usize) -> bool {
    cpx10_i(p) >= cpx10_i(0xae2364).wrapping_sub(0x10)
}

/// Os dois nomes do técnico humano (os textos de +4 e +8, 0x7858c0) a partir do dado `primeiro`; o próximo dado livre
/// depois do primeiro nome vai no dado `k_proximo`.
pub(crate) unsafe fn cpx10_nomes(n: usize, p: usize, primeiro: u8, k_proximo: u32) {
    let texto: unsafe extern "thiscall" fn(usize, u32, usize, *mut u8) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT_STRING);
    // ponytail: a original guarda o próximo dado no byte baixo de uma local que tinha a data [0xdd6ec8] e empurra o
    // dword inteiro como campo; a 0x7858c0 lê só o byte baixo, e a cópia passa o valor limpo
    let mut proximo = primeiro;
    texto(n, primeiro as u32, read_usize(p + 4), &mut proximo);
    cpx10_gravar(n, k_proximo, proximo as i8 as i32 as u32);
    texto(n, proximo as u32, read_usize(p + 8), &mut proximo);
}

/// O ensaio do comparador: a cópia roda sem entregar a notícia (as entregas copiam a notícia nas caixas dos técnicos
/// humanos, alocam e podem dar a resposta automática) e guarda o alcance que daria (+0xde: 4 todos, 2 os do país, 1 os
/// da divisão; 0 nenhuma) e a notícia montada.
pub(crate) static mut CPX10_ENSAIO: bool = false;
pub(crate) static mut CPX10_ALCANCE: u8 = 0;
pub(crate) static mut CPX10_NOTICIA: [u8; 0xe0] = [0; 0xe0];

/// A entrega da notícia `n` pelo `alcance` (4: 0x786780; 2: 0x7863f0 com a nação; 1: 0x7862d0 com a divisão) pelas
/// notícias [0xb7431c]; no ensaio, só o plano.
pub(crate) unsafe fn cpx10_entregar(n: usize, alcance: u8, alvo: usize) {
    if CPX10_ENSAIO {
        CPX10_ALCANCE = alcance;
        core::ptr::copy_nonoverlapping(n as *const u8, core::ptr::addr_of_mut!(CPX10_NOTICIA) as *mut u8, 0xe0);
        return;
    }
    let noticias = read_usize(0xb7431c);
    match alcance {
        4 => {
            let f: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_ALL_MANAGERS);
            f(noticias, n);
        }
        2 => {
            let f: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_NATION_MANAGERS);
            f(noticias, n, alvo);
        }
        _ => {
            let f: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_DIVISION_MANAGERS);
            f(noticias, n, alvo);
        }
    }
}

/// A entrega das notícias de saída e de técnico novo de um técnico da máquina: clube que pode ser dirigido (0x544820);
/// seleção (0x53b900) ou reputação (+0x80) acima de 7250: a todos; senão aos do país (+0x53), se houver.
pub(crate) unsafe fn cpx10_entregar_pelo_clube(n: usize, clube: usize) {
    let pode: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(FN_CLUB_CAN_BE_MANAGED);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    if pode(clube) == 0 {
        return;
    }
    if selecao(clube) != 0 || read_i16(clube + 0x80) > 0x1c52 {
        cpx10_entregar(n, 4, 0);
    } else {
        let nacao = read_usize(clube + 0x53);
        if nacao != 0 {
            cpx10_entregar(n, 2, nacao);
        }
    }
}

/// O comparador das três: a cópia em ensaio (sem entregar) e a original a partir do mesmo estado (as `regioes` e o
/// sorteio). Iguais: as `regioes`; os sorteios da cópia são o começo dos da original (a resposta automática da entrega
/// pode sortear depois); e cada técnico humano cuja caixa (News +8: 12 bytes por humano, início do anel em +0, próxima
/// vaga em +4) andou recebeu a notícia da cópia: tipo (+0), dados e data (+5..+0xd6) e o alcance (+0xde); sem entrega
/// planejada, nenhuma caixa anda.
pub(crate) unsafe fn cpx10_comparar(nome: &str, regioes: Vec<(usize, usize)>, original: impl FnOnce(), copia: impl FnOnce()) {
    let noticias = read_usize(0xb7431c);
    let caixas = if noticias != 0 { read_usize(noticias + 8) } else { 0 };
    let antes_caixas: Vec<(i32, i32)> = if caixas != 0 { (0..0x10).map(|k| (cpx10_i(caixas + 12 * k), cpx10_i(caixas + 12 * k + 4))).collect() } else { Vec::new() };
    let mut todas = regioes.clone();
    todas.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let antes = crate::comparacao::Memoria::ler(&todas);
    CPX10_ENSAIO = true;
    CPX10_ALCANCE = 0;
    let (_, rr) = crate::aleatorio::rastrear(|| crate::aleatorio::sem_contar(copia));
    CPX10_ENSAIO = false;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    let (alcance, noticia) = (CPX10_ALCANCE, *core::ptr::addr_of!(CPX10_NOTICIA));
    antes.escrever();
    let (_, ro) = crate::aleatorio::rastrear(original);
    let mo = crate::comparacao::Memoria::ler(&regioes);
    let mut entregues = 0;
    let mut noticia_igual = true;
    let caixas = if noticias != 0 { read_usize(noticias + 8) } else { 0 };
    if caixas != 0 && antes_caixas.len() == 0x10 {
        for (k, &(inicio, vaga)) in antes_caixas.iter().enumerate() {
            let e = caixas + 12 * k;
            if cpx10_i(e + 4) == vaga && cpx10_i(e) == inicio {
                continue;
            }
            entregues += 1;
            let v = if inicio == -1 { cpx10_i(e) } else { vaga }; // a primeira notícia abre o anel no início
            let s = read_usize(noticias).wrapping_add((v as usize).wrapping_mul(0xdf));
            let g = core::slice::from_raw_parts(s as *const u8, 0xdf);
            if g[0..4] != noticia[0..4] || g[5..0xd6] != noticia[5..0xd6] || g[0xde] != alcance {
                noticia_igual = false;
            }
        }
    }
    let igual = mo == mr && ro.len() >= rr.len() && ro[..rr.len()] == rr[..] && noticia_igual && (alcance != 0 || entregues == 0);
    PLACAR.registrar(igual, || format!("{} original {:x?} sorteios {:?} rust {:x?} sorteios {:?}; alcance {} entregues {} noticia igual {}", nome, mo, ro, mr, rr, alcance, entregues, noticia_igual));
}

/// the 0x1777 news with the manager, the club and the chairman; the reason by the board ratings below confidence − 500
/// (draws of 3 and 5) and the rumour (draw of 10, or 1 when the manager knows the club); delivered to all, to the
/// nation or to the division managers; today in +0x18 of the board row
/// Original: `MANAGER_MANAGER::post_manager_under_pressure_news` (0x6a0e70, manager_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x6a0e70 } (thiscall, ret 8, manager_manager.cpp): `MANAGER_MANAGER::post_manager_under_pressure_news(clube, tipo)`,
/// por pensar em demitir o técnico (0x69aba0: tipo 1 na troca pelo melhor candidato, 2 depois de 14 dias). Clube sem
/// técnico (+0xcf) ou antes da data [0xdd6ec8]: nada. A notícia 0x1777: o técnico (0..3), o clube (4), o presidente
/// (+0xbf: 5..7 e o id em 10; sem ele, −1). O motivo (8) pela linha da diretoria (+6 a confiança): avaliação +8, +0xa ou
/// +0xc abaixo da confiança − 500 com sorteio(3) zero dá 2, 1 ou 3 (+1 com sorteio(5) não zero); senão 0. O tipo (9) e
/// o boato (11): sorteio(10), ou 1 se o técnico conhece bem o clube (0x544410 do não jogador +0x69 com +0x1b acima de
/// 10). Entrega: técnico humano, a todos; clube de reputação acima de 5000, aos do país; senão aos da divisão (+0x57)
/// que não seja a [0x9cf764]. Por fim a data de hoje em +0x18 da linha (0x417c10).
pub(crate) unsafe extern "thiscall" fn noticia_da_pressao(this: usize, clube: usize, tipo: u32) {
    if clube == 0 {
        return;
    }
    let tec = read_usize(clube + 0xcf);
    if tec == 0 || cpx10_antes_da_data() {
        return;
    }
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    construir(n, 0x1777, 0);
    cpx10_gravar_pessoa(n, 0, tec);
    cpx10_gravar(n, 4, read_usize(clube) as u32);
    let pres = read_usize(clube + 0xbf);
    if pres != 0 {
        cpx10_gravar(n, 5, read_usize(read_usize(pres + 4) + 0x33) as u32);
        cpx10_gravar(n, 6, read_usize(read_usize(pres + 8) + 0x33) as u32);
        cpx10_gravar(n, 7, read_usize(read_usize(pres + 0xc) + 0x33) as u32);
        cpx10_gravar(n, 0xa, read_usize(pres) as u32);
    } else {
        for k in [5, 6, 7, 0xa] {
            cpx10_gravar(n, k, u32::MAX);
        }
    }
    let l = cpx10_linha(this, clube);
    let motivo: i8 = if (read_i16(l + 8) as i32) < read_i16(l + 6) as i32 - 500 /* cpx10: o limite pela avaliação +8 */ && {
        crate::comparacao::ramo("cpx_10 motivo pela avaliacao");
        cpx10_sortear(3) == 0
    } {
        2
    } else if (read_i16(l + 0xa) as i32) < read_i16(l + 6) as i32 - 500 && cpx10_sortear(3) == 0 {
        1
    } else if (read_i16(l + 0xc) as i32) < read_i16(l + 6) as i32 - 500 && cpx10_sortear(3) == 0 {
        3 + (cpx10_sortear(5) != 0) as i8
    } else {
        0
    };
    cpx10_gravar(n, 8, motivo as i32 as u32);
    cpx10_gravar(n, 9, tipo as u8 as i8 as i32 as u32);
    crate::comparacao::ramo("cpx_10 sorteio do boato");
    let mut boato = cpx10_sortear(10); // cpx10: o sorteio do boato
    let nj = read_usize(tec + 0x69);
    if nj != 0 {
        let conhecer: unsafe extern "C" fn(usize, i8) -> i32 = core::mem::transmute(FN_NON_PLAYER_ATTRIBUTE_RATING);
        if conhecer(nj, read_u8(nj + 0x1b) as i8) as u8 as i8 > 10 {
            boato = 1;
        }
    }
    cpx10_gravar(n, 0xb, boato as u32);
    if cpx10_humano(tec) {
        cpx10_entregar(n, 4, 0);
    } else if read_i16(clube + 0x80) > 0x1388 {
        cpx10_entregar(n, 2, read_usize(clube + 0x53));
    } else {
        let div = read_usize(clube + 0x57);
        if div != 0 && cpx10_i(div) != cpx10_i(KEY_CLUB_COMP_A_LOWER_DIVISION) {
            cpx10_entregar(n, 1, div);
        }
    }
    let copiar: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let mut data = [0u32; 2];
    crate::comparacao::ramo("cpx_10 data da pressao");
    copiar(cpx10_linha(this, clube) + 0x18, data.as_mut_ptr() as usize, 0xae2c90); // cpx10: a data da notícia na linha
}

pub(crate) unsafe extern "thiscall" fn comparar_noticia_da_pressao(this: usize, clube: usize, tipo: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(1332);
    let regioes = if clube != 0 { vec![(cpx10_linha(this, clube), 0x49)] } else { Vec::new() };
    cpx10_comparar("noticia_da_pressao", regioes, || o(this, clube, tipo), || noticia_da_pressao(this, clube, tipo));
}

/// the 6000 news with the manager, the type, the club, the job, the chairman and the reason; a human manager with the
/// names and to all; otherwise by the club (national team or reputation above 7250 to all, otherwise to the nation)
/// Original: `MANAGER_MANAGER::post_manager_departure_news` (0x6a1b00, manager_manager.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x6a1b00 } (thiscall, ret 0x14, manager_manager.cpp): `MANAGER_MANAGER::post_manager_departure_news(tecnico, tipo,
/// motivo, clube, cargo)`, por tirar o técnico do cargo (0x69b790). Técnico ou clube nulos, antes da data [0xdd6ec8] ou
/// tipo 5: nada. A notícia 6000: o técnico (0..3), o tipo (4), o clube (5), o cargo (7), as chaves do presidente (+0xbf:
/// 8..10; sem ele, −1) e o motivo (11). Técnico humano: os nomes dele a partir do dado 13 (o próximo livre no 12) e a
/// todos; senão pelo clube (0x544820; seleção ou reputação acima de 7250 a todos, senão aos do país).
pub(crate) unsafe extern "thiscall" fn noticia_da_saida_do_tecnico(this: usize, tec: usize, tipo: u32, motivo: u32, clube: usize, cargo: u32) {
    let _ = this;
    if tec == 0 || clube == 0 || cpx10_antes_da_data() {
        return;
    }
    let t = tipo as u8 as i8;
    if t == 5 {
        return;
    }
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let mut local = [0u32; 0x38];
    let n = local.as_mut_ptr() as usize;
    construir(n, 0x1770, 0);
    cpx10_gravar_pessoa(n, 0, tec);
    cpx10_gravar(n, 4, t as i32 as u32);
    cpx10_gravar(n, 5, read_usize(clube) as u32);
    cpx10_gravar(n, 7, cargo as u8 as i8 as i32 as u32);
    let pres = read_usize(clube + 0xbf);
    for (k, d) in [(8, 4), (9, 8), (0xa, 0xc)] {
        cpx10_gravar(n, k, if pres != 0 { read_usize(read_usize(pres + d) + 0x33) as u32 } else { u32::MAX });
    }
    cpx10_gravar(n, 0xb, motivo as u8 as i8 as i32 as u32);
    crate::comparacao::ramo("cpx_10 noticia da saida");
    if cpx10_humano(tec) {
        cpx10_nomes(n, tec, 0xd, 0xc);
        cpx10_entregar(n, 4, 0);
    } else {
        cpx10_entregar_pelo_clube(n, clube);
    }
}

pub(crate) unsafe extern "thiscall" fn comparar_noticia_da_saida_do_tecnico(this: usize, tec: usize, tipo: u32, motivo: u32, clube: usize, cargo: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, u32)>(1333);
    cpx10_comparar("noticia_da_saida_do_tecnico", Vec::new(), || o(this, tec, tipo, motivo, clube, cargo), || noticia_da_saida_do_tecnico(this, tec, tipo, motivo, clube, cargo));
}

/// only a club of an active nation with an active division, or a national team of an active nation: the 0x1787 news
/// with the manager, the previous club, the club, the job and the previous role; delivered like the departure one
/// Original: `MANAGER_MANAGER::post_manager_appointment_news` (0x6a2e00, manager_manager.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x6a2e00 } (thiscall, ret 0x10, manager_manager.cpp): `MANAGER_MANAGER::post_manager_appointment_news(tecnico,
/// anterior, clube, cargo)`, pelo dia do mercado de técnicos (0x68f9c0) e pela nomeação (0x69ebc0). Técnico ou clube
/// nulos, antes da data [0xdd6ec8] ou cargo 5: nada. Só clube de nação ativa (+0x11c da nação) com a divisão no vetor
/// [0xadadfc], ou seleção (0x53b900) de nação ativa. A notícia 0x1787: o técnico (0..3), o clube anterior (4; −1), o
/// clube (5), o cargo (6) e a função anterior (7: +0x28 numa seleção, +0x3d num clube; sem anterior, −1). Técnico
/// humano: os nomes a partir do dado 8 (o próximo livre no 7) e a todos; senão pelo clube, como a da saída.
pub(crate) unsafe extern "thiscall" fn noticia_do_tecnico_novo(this: usize, tec: usize, anterior: usize, clube: usize, cargo: u32) {
    let _ = this;
    if tec == 0 || clube == 0 || cpx10_antes_da_data() {
        return;
    }
    let c = cargo as u8 as i8;
    if c == 5 {
        return;
    }
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let nacao = read_usize(clube + 0x53);
    let ativa = nacao != 0 && read_u8(nacao + 0x11c) != 0 && {
        let div = read_usize(clube + 0x57);
        div != 0 && read_usize(read_usize(0xadadfc).wrapping_add(cpx10_i(div).wrapping_mul(4) as usize)) != 0
    };
    // a original lê +0x11c da nação da seleção sem conferir a nação (toda seleção tem uma)
    if !ativa && !(selecao(clube) != 0 && read_u8(read_usize(clube + 0x53) + 0x11c) != 0) {
        return;
    }
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let mut local = [0u32; 0x38];
    let n = local.as_mut_ptr() as usize;
    construir(n, 0x1787, 0);
    cpx10_gravar_pessoa(n, 0, tec);
    cpx10_gravar(n, 4, if anterior != 0 { read_usize(anterior) as u32 } else { u32::MAX });
    cpx10_gravar(n, 5, read_usize(clube) as u32);
    cpx10_gravar(n, 6, c as i32 as u32);
    let funcao = if anterior == 0 {
        u32::MAX
    } else if selecao(anterior) != 0 {
        read_u8(tec + 0x28) as i8 as i32 as u32
    } else {
        read_u8(tec + 0x3d) as i8 as i32 as u32
    };
    cpx10_gravar(n, 7, funcao);
    crate::comparacao::ramo("cpx_10 noticia do tecnico novo");
    if cpx10_humano(tec) {
        cpx10_nomes(n, tec, 8, 7);
        cpx10_entregar(n, 4, 0);
    } else {
        cpx10_entregar_pelo_clube(n, clube);
    }
}

pub(crate) unsafe extern "thiscall" fn comparar_noticia_do_tecnico_novo(this: usize, tec: usize, anterior: usize, clube: usize, cargo: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32)>(1334);
    cpx10_comparar("noticia_do_tecnico_novo", Vec::new(), || o(this, tec, anterior, clube, cargo), || noticia_do_tecnico_novo(this, tec, anterior, clube, cargo));
}

// ---------- 259a leva: o físico do dia do jogador (a forma e a condição de jogo até hoje), a faixa de confiança da diretoria na liga e as notas da diretoria (injury.cpp, manager_manager.cpp) ----------


pub(crate) unsafe fn cpw8_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}


pub(crate) unsafe fn cpw8_uw(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}


pub(crate) unsafe fn cpw8_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    f(n)
}

/// A data `destino` recebe `origem` (0x417c10, com a devolução numa local).
pub(crate) unsafe fn cpw8_copiar_data(destino: usize, origem: usize) {
    let copiar: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let mut t = [0u8; 8];
    copiar(destino, t.as_mut_ptr(), origem);
}

/// A diferença entre hoje ([0xae2c90]) e a data `d` (0x54cc30): os dias (+0, com sinal) mais 365 × os anos (+2).
pub(crate) unsafe fn cpw8_dias_desde(d: usize) -> i32 {
    let diferenca: unsafe extern "thiscall" fn(usize, *mut u8, u32, u32) -> usize = core::mem::transmute(FN_DATE_GET_DIFFERENCE);
    let mut r = [0u8; 8];
    diferenca(0xae2c90, r.as_mut_ptr(), read_usize(d) as u32, read_usize(d + 4) as u32);
    let p = r.as_ptr() as usize;
    (cpw8_uw(p + 2) as i32).wrapping_mul(365).wrapping_add(read_i16(p) as i32)
}

/// O ensaio do comparador: a cópia roda sem o fim da lesão (0x62e700: histórico em arquivo e notícia), que não pode
/// rodar duas vezes; chegando lá, marca `CPW8_TERMINOU` e o comparador fica com a original sozinha.
pub(crate) static mut CPW8_ENSAIO: bool = false;
pub(crate) static mut CPW8_TERMINOU: bool = false;

pub(crate) unsafe fn cpw8_terminar_lesao(this: usize, pessoa: usize) {
    if CPW8_ENSAIO {
        CPW8_TERMINOU = true;
        return;
    }
    let terminar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_INJURY_END_PLAYER_INJURY);
    terminar(this, pessoa, 1);
}

/// A estática local da guarda [0xb63d10] (bit `bit`): na primeira vez, Data() (0x54a110) e o destrutor vazio no atexit.
pub(crate) unsafe fn cpw8_estatica(bit: u8, data: usize, destrutor: usize) {
    let g = 0xb63d10 as *mut u8;
    if *g & bit == 0 {
        *g |= bit;
        let padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
        padrao(data);
        let atexit: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(FN_CRT_ATEXIT);
        atexit(destrutor);
    }
}

/// O trecho da forma acima de `base`: (x − base) × a constante `k` (f64), truncado a 16 bits com sinal, mais `base`.
pub(crate) unsafe fn cpw8_trecho(x: i32, base: i32, k: usize) -> i32 {
    (ftol(x.wrapping_sub(base) as f64 * f64_em(k)) as i16 as i32).wrapping_add(base)
}

/// once a day: the days since the last update become free days, recovery days, stopped days (injury, ending it through
/// 0x62e700) or holiday days (club out of season, 0x669bd0); fitness (+8) recovers 1000 − 12 × (+0x1e − natural) per
/// free day, loses 66 per stopped day and 55 per holiday day, capped by natural fitness and with shrunk bands above
/// 7000; match condition (+0xa) rises with free and recovery days; stores today's date
/// Original: `Injury::update_player_fitness` (0x62c680, injury.cpp, thiscall, ret 4).
///
/// [pt] { 0x62c680 } (thiscall, ret 4, injury.cpp, quadro SEH só das estáticas locais): `Injury::update_player_fitness(
/// pessoa)`: uma vez por dia (a data +0 do registro de 31 bytes da pessoa em [this] já é hoje: nada), pela condição de
/// jogo (0x62c510), pelo estado da lesão (0x62c400), pelo dia das lesões (0x62b7b0) e pelas notícias da volta (0x62e010):
/// toda pessoa que joga, todo dia em que alguém pergunta. T = dias desde a última atualização (0x54cc30). Lesionada
/// (+0xe na tabela de 0x30 bytes em [this+4]): pelos dias desde o começo (+0x20) contra +0x2a e +0x28, os dias parados
/// (perda), em recuperação ou livres, com o fim da lesão (0x62e700) quando passa de +0x28. Sem lesão: dias livres, salvo
/// na seleção com data futura (0x76d780/0x76e040) ou no clube fora da temporada (0x669bd0, guardado em 0xb63d2c/0xb63d30
/// com as datas 0xb63d18/0xb63d20): férias (o byte +0xa do registro de 79 bytes sobe com sorteio(+0x5b / 3 + 2) zero, até
/// 50) e a volta da pré-temporada (15 dias, ou o meio de um sorteio(2) com forma × condição abaixo de 80%). A forma (+8):
/// recuperação de 1000 − 12 × (+0x1e − natural (+0x34 da ficha), 0..100) por dia livre, perda de 66 por dia parado, teto
/// pela forma natural, trechos acima de 7000/9000/9850 encolhidos (0x96af30, 0x96bad0, 0x968968, 0x968918, 0x967880) e −55
/// por dia de férias; a condição de jogo (+0xa) sobe 500 × dias livres e 250 × em recuperação pelo fator do cansaço
/// (+0xc).
/// Grava as duas e a data de hoje.
pub(crate) unsafe extern "thiscall" fn fisico_do_dia(this: usize, pessoa: usize) {
    cpw8_estatica(1, 0xb63d08, 0x62ce20);
    cpw8_estatica(2, 0xb63d18, 0x62ce10);
    cpw8_estatica(4, 0xb63d20, 0x62ce00);
    if pessoa == 0 {
        return;
    }
    let id = cpw8_i(pessoa);
    if id >= cpw8_i(0xae2364) {
        return;
    }
    let ficha = read_usize(pessoa + 0x61);
    if ficha == 0 {
        return;
    }
    let rec = read_usize(this).wrapping_add((id as u32).wrapping_mul(31) as usize);
    if cpw8_uw(rec + 2) == cpw8_uw(0xae2c92) && read_i16(rec) == read_i16(0xae2c90) {
        return; // já atualizado hoje
    }
    let t = cpw8_dias_desde(rec);
    let atual = cpw8_i(rec + 0xe);
    if atual > cpw8_i(this + 8) || atual < -1 {
        core::ptr::write_unaligned((rec + 0xe) as *mut i32, -1);
    }
    // os dias livres (l38), em recuperação (l3c), parados (l34) e de férias (l30)
    let (mut l3c, mut l38, mut l34, mut l30) = (0i32, 0i32, 0i32, 0i32);
    let i = cpw8_i(rec + 0xe);
    if i != -1 {
        let inj = read_usize(this + 4).wrapping_add((i as u32).wrapping_mul(0x30) as usize);
        let e = cpw8_dias_desde(inj + 0x20);
        let s = read_i16(inj + 0x2a) as i32;
        if e < s {
            l34 = t;
        } else {
            let f = read_i16(inj + 0x28) as i32;
            if e < f {
                if t < s.wrapping_add(e) {
                    l3c = e - s;
                    l34 = t - (e - s);
                } else {
                    l3c = t;
                }
            } else if t > e - s {
                l38 = t - f;
                l3c = (e - s) - l38;
                l34 = f - l3c;
                cpw8_terminar_lesao(this, pessoa);
            } else {
                l38 = t;
                cpw8_terminar_lesao(this, pessoa);
            }
        }
    } else {
        l38 = t;
        let selecao = read_usize(pessoa + 0x24);
        let mut ocupado = false;
        if selecao != 0 {
            let classe: unsafe extern "stdcall" fn(usize, u32) -> u8 = core::mem::transmute(FN_NATIONAL_TEAMS_GET_SQUAD_SLOT);
            let k = classe(selecao, (*((pessoa + 0x5e) as *const u8) & 8) as u32);
            let campo: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_NATIONALTEAMS_GET_SQUAD_SLOT_RECORD);
            // ponytail: a original empurra o argumento com o al por cima (o resto é o ponteiro da pessoa); só o byte conta
            let d = campo(read_usize(0xb74318), read_usize(selecao + 0x53), (pessoa as u32 & 0xffff_ff00) | k as u32);
            let (ano, hoje) = (cpw8_uw(d + 2), cpw8_uw(0xae2c92));
            ocupado = if ano > hoje {
                true
            } else if ano < hoje {
                false
            } else {
                read_i16(d) > read_i16(0xae2c90)
            };
        }
        let clube = read_usize(pessoa + 0x39);
        if clube != 0 && !ocupado {
            if clube != read_usize(0xb63d2c) || cpw8_uw(0xb63d0a) != cpw8_uw(0xae2c92) || read_i16(0xb63d08) != read_i16(0xae2c90) {
                let na_temporada: unsafe extern "C" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_KEY_NATION_CLUB_IN_SEASON);
                let r = na_temporada(clube, 0xb63d18, 0xb63d20);
                *(0xb63d30 as *mut u32) = (r == 0) as u32;
                *(0xb63d2c as *mut usize) = read_usize(pessoa + 0x39);
                cpw8_copiar_data(0xb63d08, 0xae2c90);
            }
            if cpw8_i(0xb63d30) != 0 {
                l38 = 0;
                let dias_ate: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
                let u6 = dias_ate(0xae2c90, read_usize(0xb63d20) as u32, read_usize(0xb63d24) as u32) as u16;
                let u7 = dias_ate(0xb63d18, read_usize(0xae2c90) as u32, read_usize(0xae2c94) as u32) as u16;
                if u6 < u7 {
                    let lim = if (u6 as i32) < t { u6 as i32 } else { t };
                    l30 = lim;
                    l3c = t - lim;
                    for _ in 0..lim {
                        let g = read_usize(0xae2d0c).wrapping_add((cpw8_i(pessoa) as u32).wrapping_mul(0x4f) as usize) + 0xa;
                        if *(g as *const u8) >= 0x32 {
                            break;
                        }
                        if cpw8_sortear(read_i8(pessoa + 0x5b) as i32 / 3 + 2) == 0 {
                            *(g as *mut u8) = (*(g as *const u8)).wrapping_add(1);
                        }
                    }
                } else {
                    let mut x = u6 as i32 - (u7 as i32 + u6 as i32) / 2;
                    if x < 0 {
                        x = 0;
                    }
                    if t < x {
                        x = t;
                    }
                    l3c = x;
                    l34 = t - x;
                }
                if u7 < 0xf {
                    l38 = 0xf - u7 as i32;
                    l3c -= l38;
                    if l3c < 0 {
                        l3c = 0;
                    }
                } else {
                    let (f, mc) = (read_i16(rec + 8) as i32, read_i16(rec + 0xa) as i32);
                    if mc.wrapping_mul(f) / 10000 < 8000 { // cpw8: a forma abaixo de 80% na pré-temporada
                        crate::comparacao::ramo("cpw_8 sorteio da forma");
                        let r = cpw8_sortear(2);
                        l38 = (r + l3c) / 2;
                        l3c -= l38;
                    }
                }
            }
        }
    }
    // a forma
    let f = read_i16(rec + 8) as i32;
    let natural = read_i8(ficha + 0x34);
    let mut v = read_i8(rec + 0x1e).wrapping_sub(natural);
    if v < 0 {
        v = 0;
    } else if v > 100 {
        v = 100;
    }
    let s = (1000 - 12 * v as i32) as i16 as i32; // cpw8: a recuperação por dia livre
    let metade = s / 2;
    let n = natural as i32;
    let piso = ((n * 25 / 20 + 15) * 100) as i16 as i32;
    if l34 != 0 {
        crate::comparacao::ramo("cpw_8 perda");
    }
    let mut x = f.wrapping_sub(66i32.wrapping_mul(l34)); // cpw8: a perda por dia parado
    if x < piso {
        x = 4000;
    }
    let m = n * 16 / 20 + 0x54;
    let mut teto = m.wrapping_mul(100);
    if (teto as i16 as i32) < x {
        teto = x;
    }
    x = x.wrapping_add((metade as i16 as i32).wrapping_mul(l3c));
    let teto16 = teto as i16 as i32;
    if x > teto16 {
        x = teto16;
    }
    if l38 != 0 {
        crate::comparacao::ramo("cpw_8 recuperacao");
    }
    x = x.wrapping_add(s.wrapping_mul(l38));
    if f > 0x267a {
        x = cpw8_trecho(x, f, 0x967880);
    } else if f > 0x2328 {
        x = cpw8_trecho(x, f, 0x968918);
        if x > 0x267a {
            x = cpw8_trecho(x, 0x267a, 0x968968);
        }
    } else if x > 0x1b58 {
        x = cpw8_trecho(x, 0x1b58, 0x96af30);
        if x > 0x2328 {
            x = cpw8_trecho(x, 0x2328, 0x96bad0);
        }
        if x > 0x267a {
            x = cpw8_trecho(x, 0x267a, 0x968968);
        }
    }
    if x > 10000 {
        x = 10000;
    }
    let mut forma = x.wrapping_sub(55i32.wrapping_mul(l30));
    if forma < piso {
        forma = 5000;
    }
    let mut e2 = m;
    if (m as i16 as i32) < forma {
        e2 = forma;
    }
    let e16 = e2 as i16 as i32;
    if forma > e16 {
        forma = e16;
    }
    // a condição de jogo, pelo fator do cansaço (+0xc)
    let cansaco = read_i16(rec + 0xc);
    let mc = read_i16(rec + 0xa) as i32;
    let teto_k = f64_em(0x9689d0);
    let k = if cansaco > 1000 {
        let kv = f64_em(0x968dc0) - cansaco as f64 * f64_em(0x96a608);
        if !(kv > teto_k) {
            kv
        } else if l3c > 0 {
            teto_k
        } else {
            kv
        }
    } else if l3c > 0 {
        teto_k
    } else {
        f64_em(0x967890)
    };
    let r1 = ftol(l38.wrapping_mul(500) as f64 * k);
    let r2 = ftol(l3c.wrapping_mul(250) as f64 * k);
    let mut cond = r2.wrapping_add(mc).wrapping_add(r1);
    if cond > 10000 {
        cond = 10000;
    }
    core::ptr::write_unaligned((rec + 8) as *mut i16, forma as i16);
    core::ptr::write_unaligned((rec + 0xa) as *mut i16, cond as i16);
    cpw8_copiar_data(rec, 0xae2c90);
}

/// Comparador lado a lado: o registro de 31 bytes da pessoa, o byte +0xa do registro de 79 bytes, as estáticas e o
/// guardado do clube (0xb63d08..0xb63d34) e o sorteio. A cópia roda em ensaio, sem o fim da lesão (0x62e700): quando
/// ela chega lá, fica a original sozinha (já rodou primeiro). Na primeira vez (estáticas por criar, com atexit) e sem
/// registro (pessoa nula, id fora da tabela, sem ficha): só a original.
pub(crate) unsafe extern "thiscall" fn comparar_fisico_do_dia(this: usize, pessoa: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(1290);
    if *(0xb63d10 as *const u8) & 7 != 7 || pessoa == 0 || cpw8_i(pessoa) < 0 || cpw8_i(pessoa) >= cpw8_i(0xae2364) || read_usize(pessoa + 0x61) == 0 {
        return o(this, pessoa); // ponytail: as estáticas e o atexit da primeira vez não podem rodar duas vezes
    }
    let id = cpw8_i(pessoa) as usize;
    let rec = read_usize(this).wrapping_add(id.wrapping_mul(31));
    let geral = read_usize(0xae2d0c).wrapping_add(id.wrapping_mul(0x4f));
    let e = crate::aleatorio::REGIOES_DO_ESTADO;
    let reg = [(rec, 31), (geral + 0xa, 1), (0xb63d08, 0x2c), e[0], e[1]];
    CPW8_TERMINOU = false;
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, pessoa), || {
        CPW8_ENSAIO = true;
        crate::aleatorio::sem_contar(|| fisico_do_dia(this, pessoa));
        CPW8_ENSAIO = false;
    });
    if CPW8_TERMINOU {
        crate::comparacao::ramo("cpw_8 fim da lesao so na original");
        return;
    }
    PLACAR.registrar(igual, || format!("fisico_do_dia(pessoa {:x}) {:x?} x {:x?}", pessoa, mo, mr));
}

/// the board's league confidence (+0x12 of the club's 0x49-byte record) in bands 0 to 6 (−2000, −1000, −500, 500, 1000,
/// 2000)
/// Original: `MANAGER_MANAGER::get_league_confidence_band` (0x690bd0, manager_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x690bd0 } (thiscall, ret 4, manager_manager.cpp): `MANAGER_MANAGER::get_league_confidence_band(clube)`: a
/// confiança da diretoria na liga (palavra com sinal +0x12 do registro de 0x49 bytes do clube em [this]) em faixas:
/// abaixo de −2000: 0; −1000: 1; −500: 2; 500: 3; 1000: 4; 2000: 5; senão 6. Pelo texto da confiança (0x690d80).
/// Só o al conta (o resto do eax é o que sobrou da conta do endereço).
pub(crate) unsafe extern "thiscall" fn faixa_da_confianca_na_liga(this: usize, clube: usize) -> u8 {
    let v = read_i16(read_usize(this).wrapping_add((cpw8_i(clube) as u32).wrapping_mul(0x49) as usize) + 0x12);
    if v < -2000 {
        0
    } else if v < -1000 {
        1
    } else if v < -500 {
        2
    } else if v < 500 {
        3
    } else if v < 1000 {
        4
    } else if v < 2000 {
        5
    } else {
        6
    }
}

pub(crate) unsafe extern "thiscall" fn comparar_faixa_da_confianca_na_liga(this: usize, clube: usize) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize) -> u8>(1291)(this, clube);
    let n = faixa_da_confianca_na_liga(this, clube);
    PLACAR.registrar(r == n, || format!("faixa_da_confianca_na_liga(clube {:x}) original={} rust={}", clube, r, n));
    r
}

/// the four board ratings (+6, +8, +0xc, +0xa of the club's 0x49-byte record) get the value
/// Original: `MANAGER_MANAGER::set_board_ratings` (0x69f7c0, manager_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x69f7c0 } (thiscall, ret 8, manager_manager.cpp): `MANAGER_MANAGER::set_board_ratings(clube, valor)`: as quatro
/// notas da diretoria (+6, +8, +0xc e +0xa do registro de 0x49 bytes do clube em [this], palavras, nessa ordem) recebem
/// o valor (16 bits). Pela configuração inicial dos jogadores (0x7a46a6).
pub(crate) unsafe extern "thiscall" fn gravar_notas_da_diretoria(this: usize, clube: usize, valor: u32) {
    for off in [6usize, 8, 0xc, 0xa] {
        let r = read_usize(this).wrapping_add((cpw8_i(clube) as u32).wrapping_mul(0x49) as usize);
        core::ptr::write_unaligned((r + off) as *mut u16, valor as u16);
    }
}

pub(crate) unsafe extern "thiscall" fn comparar_gravar_notas_da_diretoria(this: usize, clube: usize, valor: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(1292);
    let r = read_usize(this).wrapping_add((cpw8_i(clube) as u32).wrapping_mul(0x49) as usize);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(r, 0x49)], || o(this, clube, valor), || gravar_notas_da_diretoria(this, clube, valor));
    PLACAR.registrar(igual, || format!("gravar_notas_da_diretoria(clube {:x}, {:x}) {:x?} x {:x?}", clube, valor, mo, mr));
}

// ---------- 258a leva: a recusa dos termos pelo jogador, a notícia da confiança da diretoria e a das expectativas da diretoria (transfer_manager.cpp, manager_manager.cpp) ----------





/// O registro de clube de id `id` ([0xae23b4], 0x245 bytes).
pub(crate) unsafe fn cpv2_clube(id: usize) -> usize {
    read_usize(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// A pessoa de id `id` ([0xae23bc], 0x6e bytes).
pub(crate) unsafe fn cpv2_pessoa(id: usize) -> usize {
    read_usize(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

/// O contrato principal da pessoa (0x4cf5a0), guardado em [0xae1080] como a original faz depois de cada chamada.
pub(crate) unsafe fn cpv2_contrato(p: usize) -> usize {
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = contrato_de(read_usize(0xae1050), p);
    *(0xae1080 as *mut usize) = c;
    c
}

/// O clube do contrato (id em +4); com `filtro`, nenhum para o contrato de tipo 5 (+0x35 & 0x3f).
pub(crate) unsafe fn cpv2_clube_do_contrato(c: usize, filtro: bool) -> usize {
    if c != 0 && (!filtro || read_u8(c + 0x35) & 0x3f != 5) {
        cpv2_clube(read_usize(c + 4))
    } else {
        0
    }
}

/// Volta a lista ao começo (o cursor +0xa recebe a cabeça +2).
pub(crate) unsafe fn cpv2_inicio(l: usize) {
    core::ptr::write_unaligned((l + 0xa) as *mut usize, read_usize(l + 2));
}

/// O próximo item da lista (0x68d1b0), ou 0.
pub(crate) unsafe fn cpv2_proximo(l: usize) -> usize {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    proximo(l)
}

/// A oferta já resolvida pelo status (+0x2c): 2, 3, 4, 0x10 ou 0x14, ou 0xf com +0x2e ≥ 3.
pub(crate) unsafe fn cpv2_resolvida(o: usize) -> bool {
    match read_u8(o + 0x2c) {
        2 | 3 | 4 | 0x10 | 0x14 => true,
        0xf => read_u8(o + 0x2e) >= 3,
        _ => false,
    }
}

/// the player refuses the terms: the offer returns in 10 days, the bid markers go, his reputation rises if the buyer is
/// more famous; or he leaves the exchange and his value moves to the fee
/// Original: `handle_player_rejected_terms` (0x8c28c0, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8c28c0 } (thiscall, ret 0xc, transfer_manager.cpp): `TRANSFER_MANAGER::handle_player_rejected_terms(pessoa,
/// índice, silencioso)`: índice fora do vetor [this] (conta +4): a caixa "Error" (0xf49, 0xf4e). Vaga vazia: nada.
/// Se a pessoa é o jogador da oferta (+0xc): termos recusados (+0x10 = 4), status 0xf; +0x2e sobe com comprador humano
/// (0x600a50 sem as férias) e +0x1f ≠ 1, senão 3; saem as marcas de lance (0x84aeb0) do comprador e do dono de cada
/// jogador da troca; o gerente de listas anota a recusa (0x843e10) se o comprador é do computador; reagendada em 10
/// dias (0x8c6220); resolvida com todas as rivais resolvidas, o bit 0 do registro de 8 bytes do jogador ([this+0x86e])
/// sai; saem as propostas de contrato (0x4cca70) dos jogadores da troca com o vendedor. Sem `silencioso`: com comprador
/// mais famoso que o vendedor (+0x80), a reputação do jogador (+0xb da ficha +0x61) sobe a diferença × 0,075 (acima de
/// 7750 com ficha abaixo de 4750), × 0,05 (acima de 4250) ou × 0,025, até 10000; a notícia da resposta (0x8d8e90).
/// Se não, a pessoa é um jogador da troca (+4; lista vazia: a caixa 0x101f): sai da lista (0x68d1f0; a lista vazia é
/// liberada), o valor dela (0x4d0c60) passa ao preço (+0x1b), o registro de 8 bytes dela perde uma oferta (+2), a
/// notícia (sem `silencioso`), e a reação do vendedor do computador (0x8c8090) e do comprador (0x8c0810), parando se a
/// oferta ficou resolvida. No fim, a oferta segue (0x8c2f50).
pub(crate) unsafe extern "thiscall" fn recusa_dos_termos(this: usize, pessoa: usize, indice: i32, silencioso: u32) {
    let clube_humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let limpar_marcas: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_SHORTLIST_MANAGER_CLEAR_BID_MARKERS_FOR_PLAYER);
    let anotar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(FN_SHORTLIST_MANAGER_UPDATE_STAFF_STATUS);
    let agendar: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_SCHEDULE_OFFER);
    let tirar_proposta: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_CONTRACT_MANAGER_REMOVE_CONTRACT_OFFER);
    let noticia: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, usize, u32, u32, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_POST_CONTRACT_RESPONSE_NEWS);
    let valor: unsafe extern "thiscall" fn(usize, usize, usize, u32, i32, u32) -> i32 = core::mem::transmute(FN_CONTRACT_MANAGER_CALC_PLAYER_VALUE);
    let tirar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_LLIST_REMOVE);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_OPERATOR_DELETE);
    let resposta_do_vendedor: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_AI_RESPOND_TO_BID);
    let resposta_do_comprador: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_REVIEW_AI_CLUB_BID);
    let seguir: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_ADVANCE_OFFER_TO_CLUB_APPROVAL);
    if indice < 0 {
        return aviso_em(0xa8d514, 0xf49);
    }
    if indice >= read_usize(this + 4) as i32 {
        return aviso_em(0xa8d514, 0xf4e);
    }
    let vaga = || read_usize(this).wrapping_add(indice as usize * 4);
    let o = read_usize(vaga());
    if o == 0 {
        return;
    }
    let preco = read_usize(o + 0x1b);
    if read_usize(pessoa) == read_usize(o + 0xc) {
        write_u8(o + 0x10, 4);
        let comprador = cpv2_clube(read_usize(o + 8));
        let vendedor = cpv2_clube_do_contrato(cpv2_contrato(pessoa), true);
        write_u8(o + 0x2c, 0xf);
        if silencioso == 0 && clube_humano(comprador, 1, 1, 0, 0) != 0 && read_u8(o + 0x1f) != 1 {
            write_u8(o + 0x2e, read_u8(o + 0x2e).wrapping_add(1));
        } else {
            write_u8(o + 0x2e, 3);
        }
        let p = cpv2_pessoa(read_usize(o + 0xc));
        let dono = cpv2_clube_do_contrato(cpv2_contrato(p), false);
        limpar_marcas(read_usize(this + 0x856), cpv2_clube(read_usize(o + 8)), p);
        let l = read_usize(o + 4);
        if l != 0 {
            cpv2_inicio(l);
            loop {
                let y = cpv2_proximo(l);
                if y == 0 {
                    break;
                }
                limpar_marcas(read_usize(this + 0x856), dono, cpv2_pessoa(read_usize(y)));
            }
        }
        if clube_humano(comprador, 0, 1, 0, 0) == 0 {
            crate::comparacao::ramo("cpv_2 recusa anotada na lista");
            // a original testa o byte com o eax zerado pela pergunta de antes (em cima, zero)
            anotar(read_usize(this + 0x856), pessoa, comprador, 1, (read_u8(o + 0x11) != 0xb) as u32); // cpv2: a recusa na lista
        }
        crate::comparacao::ramo("cpv_2 recusa do jogador");
        agendar(this, o, 0xa, 0); // cpv2: de volta em 10 dias
        if cpv2_resolvida(o) {
            let rivais = read_usize(o);
            let mut todas = true;
            if rivais != 0 {
                cpv2_inicio(rivais);
                loop {
                    let x = cpv2_proximo(rivais);
                    if x == 0 {
                        break;
                    }
                    if !cpv2_resolvida(read_usize(read_usize(this) + read_usize(x) * 4)) {
                        todas = false;
                        break;
                    }
                }
            }
            if todas {
                let m = read_usize(this + 0x86e).wrapping_add(read_usize(o + 0xc) * 8);
                if read_u8(m) & 1 != 0 {
                    write_u8(m, read_u8(m) & 0xfe);
                }
            }
        }
        let l = read_usize(o + 4);
        if l != 0 {
            cpv2_inicio(l);
            loop {
                let y = cpv2_proximo(l);
                if y == 0 {
                    break;
                }
                tirar_proposta(read_usize(0xae1050), cpv2_pessoa(read_usize(y)), vendedor);
            }
        }
        if silencioso == 0 {
            let ficha = read_usize(pessoa + 0x61);
            if comprador != 0 && vendedor != 0 && ficha != 0 {
                let rc = read_i16(comprador + 0x80);
                let rv = read_i16(vendedor + 0x80);
                if rc > rv {
                    let dif = rc.wrapping_sub(rv) as f64;
                    let k = if rc > 0x1e46 && read_i16(ficha + 0xb) < 0x128e {
                        core::ptr::read_unaligned(0x96a730 as *const f64)
                    } else if rc > 0x109a {
                        core::ptr::read_unaligned(0x967888 as *const f64)
                    } else {
                        core::ptr::read_unaligned(0x967878 as *const f64)
                    };
                    crate::comparacao::ramo("cpv_2 reputacao do jogador");
                    let soma = ftol(dif * k) as i16; // cpv2: a reputação sobe
                    let r = (ficha + 0xb) as *mut i16;
                    core::ptr::write_unaligned(r, core::ptr::read_unaligned(r).wrapping_add(soma));
                    let f2 = read_usize(pessoa + 0x61);
                    if read_i16(f2 + 0xb) > 0x2710 {
                        core::ptr::write_unaligned((f2 + 0xb) as *mut i16, 0x2710);
                    }
                }
            }
            noticia(this, pessoa, comprador, vendedor, 4, o, 0, 0, preco as u32);
        }
        return seguir(this, o);
    }
    let troca = read_usize(o + 4);
    if troca == 0 {
        aviso_em(0xa8d514, 0x101f);
        return seguir(this, o);
    }
    cpv2_inicio(troca);
    let mut x = cpv2_proximo(troca);
    while x != 0 && read_usize(x) != read_usize(pessoa) {
        x = cpv2_proximo(troca);
    }
    if x == 0 {
        return seguir(this, o);
    }
    crate::comparacao::ramo("cpv_2 jogador saiu da troca");
    let comprador = cpv2_clube(read_usize(o + 8));
    let novo_preco = valor(read_usize(0xae1050), pessoa, comprador, 0, -1, 0).wrapping_add(read_usize(o + 0x1b) as i32);
    let vendedor = cpv2_clube_do_contrato(cpv2_contrato(cpv2_pessoa(read_usize(o + 0xc))), false);
    let comprador = cpv2_clube(read_usize(o + 8));
    tirar(read_usize(o + 4), x);
    let l = read_usize(o + 4);
    if core::ptr::read_unaligned(l as *const u16) == 0 {
        if l != 0 {
            destruir(l);
            liberar(l);
        }
        core::ptr::write_unaligned((o + 4) as *mut usize, 0);
    }
    let m = read_usize(this + 0x86e).wrapping_add(read_usize(pessoa) * 8 + 2);
    write_u8(m, read_u8(m).wrapping_sub(1));
    if silencioso == 0 {
        noticia(this, pessoa, vendedor, comprador, 4, o, 0, 0, novo_preco as u32);
    }
    core::ptr::write_unaligned((o + 0x1b) as *mut i32, novo_preco);
    if vendedor != 0 && clube_humano(vendedor, 0, 1, 0, 0) == 0 {
        resposta_do_vendedor(this, vaga(), 1);
        if cpv2_resolvida(o) {
            return;
        }
    }
    if clube_humano(comprador, 0, 1, 0, 0) != 0 {
        return seguir(this, o);
    }
    resposta_do_comprador(read_usize(0xde1f54), vaga(), 1);
    if cpv2_resolvida(o) {
        return;
    }
    seguir(this, o);
}

/// Antes da data das notícias ([0xdd6ec8]: dia +0 com sinal, ano +2) em relação a hoje ([0xae2c90])?
pub(crate) unsafe fn cpv2_antes_das_noticias() -> bool {
    let dia = *(0xdd6ec8 as *const i16);
    let ano = *(0xdd6eca as *const u16);
    let hoje_ano = *(0xae2c92 as *const u16);
    ano > hoje_ano || (ano == hoje_ano && dia > *(0xae2c90 as *const i16))
}

/// the 0x1772 board confidence news with the manager's days in the job
/// Original: `post_board_confidence_update_news` (0x6a4c40, manager_manager.cpp, thiscall).
///
/// [pt] { 0x6a4c40 } (thiscall, ret 4, manager_manager.cpp, quadro SEH sem destrutor): `MANAGER_MANAGER::
/// post_board_confidence_update_news(clube)`: antes da data das notícias, nada. A notícia 0x1772 (local): 0 = o id do
/// clube, 1 = os dias do técnico (+0xcf) no cargo (desde a data +0x3e, 365 dias por ano) ou 100 sem técnico; pelo
/// alcance (0x786090) com o clube.
pub(crate) unsafe extern "thiscall" fn noticia_da_confianca_da_diretoria(_this: usize, clube: usize) {
    if cpv2_antes_das_noticias() {
        return;
    }
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let por_alcance: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_NEWS_SEND_BY_SCOPE);
    let mut local = [0u32; 0x3a]; // a notícia local (0xe8 bytes)
    let n = local.as_mut_ptr() as usize;
    construir(n, 0x1772, 0);
    gravar(n, 0, read_usize(clube) as u32);
    let tecnico = read_usize(clube + 0xcf);
    let dias = if tecnico != 0 {
        let anos = (*(0xae2c92 as *const u16) as i32).wrapping_sub(core::ptr::read_unaligned((tecnico + 0x40) as *const u16) as i32);
        anos.wrapping_mul(365).wrapping_sub(read_i16(tecnico + 0x3e) as i32).wrapping_add(*(0xae2c90 as *const i16) as i32)
    } else {
        100
    };
    gravar(n, 1, dias as u32);
    por_alcance(read_usize(0xb7431c), n, clube, 0, 0, 0, 0);
}

/// the 0x1771 board expectations news to the club manager
/// Original: `post_board_expectations_news` (0x6a4a20, manager_manager.cpp, thiscall).
///
/// [pt] { 0x6a4a20 } (thiscall, ret 8, manager_manager.cpp, quadro SEH sem destrutor): `MANAGER_MANAGER::
/// post_board_expectations_news(clube, expectativas)`: antes da data das notícias, nada. A notícia 0x1771 (local): 0 =
/// o id do clube, 1 = as expectativas (palavra com sinal); ao técnico do clube (0x7861e0).
pub(crate) unsafe extern "thiscall" fn noticia_das_expectativas_da_diretoria(_this: usize, clube: usize, expectativas: u32) {
    if cpv2_antes_das_noticias() {
        return;
    }
    crate::comparacao::ramo("cpv_2 noticia das expectativas");
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    let mut local = [0u32; 0x3a]; // a notícia local (0xe8 bytes)
    let n = local.as_mut_ptr() as usize;
    construir(n, 0x1771, 0);
    gravar(n, 0, read_usize(clube) as u32);
    gravar(n, 1, expectativas as i16 as i32 as u32);
    entregar(read_usize(0xb7431c), n, clube);
}

// ponytail: reagenda, mexe nas listas e na reputação, pode liberar a lista da troca e chama a resposta dos clubes e a
// oferta seguinte (0x8c2f50): não roda duas vezes. Só a original; a cópia leva os ramos e se prova pela carreira e
// pelas mutações.
pub(crate) unsafe extern "thiscall" fn comparar_recusa_dos_termos(this: usize, pessoa: usize, indice: i32, silencioso: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, i32, u32)>(1236)(this, pessoa, indice, silencioso)
}

// ponytail: a notícia vai pelo alcance (aloca): só a original. Quente (40 mil chamadas): sem contador.
pub(crate) unsafe extern "thiscall" fn comparar_noticia_da_confianca_da_diretoria(this: usize, clube: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize)>(1237)(this, clube)
}

// ponytail: a notícia vai à caixa do técnico (aloca): só a original; a cópia leva o ramo.
pub(crate) unsafe extern "thiscall" fn comparar_noticia_das_expectativas_da_diretoria(this: usize, clube: usize, expectativas: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(1238)(this, clube, expectativas)
}

// ---------- 258a leva: as ofertas em espera no fechamento da janela, a notícia das expectativas da federação e a do protesto da torcida (transfer_manager.cpp, manager_manager.cpp) ----------




/// O registro de clube de id `id` ([0xae23b4], 0x245 bytes).
pub(crate) unsafe fn cpv3_clube(id: usize) -> usize {
    read_usize(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// A pessoa de id `id` ([0xae23bc], 0x6e bytes).
pub(crate) unsafe fn cpv3_pessoa(id: usize) -> usize {
    read_usize(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

/// O contrato principal da pessoa (0x4cf5a0), guardado em [0xae1080] como a original faz depois de cada chamada.
pub(crate) unsafe fn cpv3_contrato(p: usize) -> usize {
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = contrato_de(read_usize(0xae1050), p);
    *(0xae1080 as *mut usize) = c;
    c
}

/// O clube do contrato (id em +4); com `filtro`, nenhum para o contrato de tipo 5 (+0x35 & 0x3f).
pub(crate) unsafe fn cpv3_clube_do_contrato(c: usize, filtro: bool) -> usize {
    if c != 0 && (!filtro || read_u8(c + 0x35) & 0x3f != 5) {
        cpv3_clube(read_usize(c + 4))
    } else {
        0
    }
}

/// A oferta já resolvida pelo status (+0x2c): 2, 3, 4, 0x10 ou 0x14, ou 0xf com +0x2e ≥ 3.
pub(crate) unsafe fn cpv3_resolvida(o: usize) -> bool {
    match read_u8(o + 0x2c) {
        2 | 3 | 4 | 0x10 | 0x14 => true,
        0xf => read_u8(o + 0x2e) >= 3,
        _ => false,
    }
}

/// Põe a oferta em espera: status 1, a marca +0x30 (bits 1..3 limpos, bit 0 ligado), sai a proposta de contrato do
/// jogador com o comprador (0x4cca70), termos zerados (+0x10) e, para cada jogador da troca (+4), sai a proposta pela
/// vaga +0x20 (0x4ccb30) e o estado dele (+4) zera.
pub(crate) unsafe fn cpv3_em_espera(o: usize) {
    let tirar_proposta: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_CONTRACT_MANAGER_REMOVE_CONTRACT_OFFER);
    let tirar_da_troca: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_CONTRACT_MANAGER_REMOVE_CONTRACT_OFFER_FOR_TRANSFER);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let marca = (read_u8(o + 0x30) & 0xf1) | 1;
    write_u8(o + 0x2c, 1);
    write_u8(o + 0x30, marca);
    tirar_proposta(read_usize(0xae1050), cpv3_pessoa(read_usize(o + 0xc)), cpv3_clube(read_usize(o + 8)));
    write_u8(o + 0x10, 0);
    let l = read_usize(o + 4);
    if l != 0 {
        core::ptr::write_unaligned((l + 0xa) as *mut usize, read_usize(l + 2));
        loop {
            let y = proximo(read_usize(o + 4));
            if y == 0 {
                break;
            }
            tirar_da_troca(read_usize(0xae1050), cpv3_pessoa(read_usize(y)), read_usize(o + 0x20));
            write_u8(y + 4, 0);
        }
    }
}

/// when the window closes, completes the approved offers and puts the nation's offers on hold: news if a human is
/// involved, otherwise withdrawn or rejected; for the computer, status 3 and back the next day
/// Original: `hold_offers_at_window_close` (0x8c4e10, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8c4e10 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::hold_offers_at_window_close(nação)`: as
/// regras da nação (das 28 em [this+0x8ac], a de +0xa igual à nação +0x53 da seleção 0x53b3d0; senão nenhuma). Para
/// cada oferta do vetor [this] (conta +4): status 0xe completa (0x8c34e0 no prazo). Comprador da nação: não resolvida,
/// jogador com ficha (+0x61), sem a marca +0x30 & 0xf, fora do status 0x13 e aceita pelas regras (virtual +0xc com o
/// comprador e hoje; resposta 0 = segura): em espera; a notícia da espera (0x8e1150) se o comprador ou o vendedor é
/// humano, senão retirada (0x8bfcb0); comprador do computador (0x600a50): status 3 e reagendada em 1 dia. Senão, com
/// jogadores na troca e vendedor da nação, pelos mesmos testes com o vendedor nas regras: em espera; a notícia, ou
/// recusada (0x8bf680); e o mesmo reagendamento.
pub(crate) unsafe extern "thiscall" fn ofertas_em_espera_na_janela(this: usize, nacao: usize) {
    let selecao: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_TEAM);
    let completar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_TRY_COMPLETE_OFFER);
    let humano: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_NEWS_CLUB_HAS_HUMAN_MANAGER);
    let noticia_da_espera: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_POST_OFFER_ON_HOLD_NEWS);
    let retirar: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_WITHDRAW_TRANSFER_OFFER);
    let recusar: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_REJECT_TRANSFER_OFFER);
    let clube_humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let agendar: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_SCHEDULE_OFFER);
    let da_nacao = read_usize(selecao(nacao) + 0x53);
    let mut regras = 0usize;
    for k in 0..0x1c {
        let r = read_usize(this + 0x8ac + 4 * k);
        if read_usize(r + 0xa) == da_nacao {
            regras = r;
            break;
        }
    }
    let aceita = |o: usize, clube: usize| -> bool {
        if regras == 0 {
            return true;
        }
        let segurar: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) -> u32 = core::mem::transmute(read_usize(read_usize(regras) + 0xc));
        segurar(regras, o, clube, 0xae2c90, 0) == 0
    };
    let mut i = 0usize;
    while (i as i32) < read_usize(this + 4) as i32 {
        let vaga = read_usize(this).wrapping_add(i * 4);
        let o = read_usize(vaga);
        i += 1;
        if o == 0 {
            continue;
        }
        let st = read_u8(o + 0x2c);
        if st == 0xe {
            completar(this, o, 1);
            continue;
        }
        let comprador = cpv3_clube(read_usize(o + 8));
        if read_usize(comprador + 0x53) == nacao
            && !cpv3_resolvida(o)
            && read_usize(cpv3_pessoa(read_usize(o + 0xc)) + 0x61) != 0
            && read_u8(o + 0x30) & 0xf == 0
            && st != 0x13
            && aceita(o, comprador)
        {
            cpv3_em_espera(o);
            let humano_vende = || {
                let c = cpv3_contrato(cpv3_pessoa(read_usize(o + 0xc)));
                if c == 0 || read_u8(c + 0x35) & 0x3f == 5 || cpv3_clube(read_usize(c + 4)) == 0 {
                    return false;
                }
                let v = cpv3_clube_do_contrato(cpv3_contrato(cpv3_pessoa(read_usize(o + 0xc))), true);
                humano(read_usize(0xb7431c), v) != 0
            };
            if humano(read_usize(0xb7431c), cpv3_clube(read_usize(o + 8))) != 0 || humano_vende() {
                noticia_da_espera(this, o, nacao);
            } else {
                crate::comparacao::ramo("cpv_3 retirada na janela");
                // a vaga relida de [this] como na original
                retirar(this, read_usize(this).wrapping_add((i - 1) * 4), 0, 0); // cpv3: o comprador retira
            }
            if clube_humano(cpv3_clube(read_usize(o + 8)), 0, 1, 0, 0) == 0 {
                crate::comparacao::ramo("cpv_3 reagendada na janela");
                write_u8(o + 0x2c, 3); // cpv3: status 3
                agendar(this, o, 1, 0); // cpv3: de volta amanhã
            }
            continue;
        }
        // os outros: oferta com troca cujo vendedor é da nação
        if read_usize(o + 4) == 0 {
            continue;
        }
        let c = cpv3_contrato(cpv3_pessoa(read_usize(o + 0xc)));
        if c == 0 || cpv3_clube(read_usize(c + 4)) == 0 {
            continue;
        }
        let vendedor = cpv3_clube_do_contrato(cpv3_contrato(cpv3_pessoa(read_usize(o + 0xc))), false);
        if read_usize(vendedor + 0x53) != nacao || cpv3_resolvida(o) || read_u8(o + 0x2c) == 0x13 {
            continue;
        }
        if regras != 0 {
            let dono = cpv3_clube_do_contrato(cpv3_contrato(cpv3_pessoa(read_usize(o + 0xc))), false);
            if !aceita(o, dono) {
                continue;
            }
        }
        cpv3_em_espera(o);
        let humano_vende = || {
            let c = cpv3_contrato(cpv3_pessoa(read_usize(o + 0xc)));
            if c == 0 || cpv3_clube(read_usize(c + 4)) == 0 {
                return false;
            }
            let v = cpv3_clube_do_contrato(cpv3_contrato(cpv3_pessoa(read_usize(o + 0xc))), false);
            humano(read_usize(0xb7431c), v) != 0
        };
        if humano(read_usize(0xb7431c), cpv3_clube(read_usize(o + 8))) != 0 || humano_vende() {
            noticia_da_espera(this, o, nacao);
        } else {
            crate::comparacao::ramo("cpv_3 recusada na janela");
            recusar(this, read_usize(this).wrapping_add((i - 1) * 4), 0, 0); // cpv3: o vendedor recusa
        }
        if clube_humano(cpv3_clube(read_usize(o + 8)), 0, 1, 0, 0) == 0 {
            write_u8(o + 0x2c, 3);
            agendar(this, o, 1, 0);
        }
    }
}

/// the 0x177e news of the FA's expectation for the national team in a competition
/// Original: `post_fa_expectations_news` (0x6a44a0, manager_manager.cpp, thiscall).
///
/// [pt] { 0x6a44a0 } (thiscall, ret 0xc, manager_manager.cpp, quadro SEH sem destrutor): `MANAGER_MANAGER::
/// post_fa_expectations_news(seleção, expectativa, competição)`: a notícia 0x177e (local): 0 = o id da seleção, 1 = a
/// expectativa (palavra com sinal), 2 = a competição; pelo alcance (0x786090) com a seleção.
pub(crate) unsafe extern "thiscall" fn noticia_das_expectativas_da_federacao(_this: usize, selecao: usize, expectativa: u32, competicao: u32) {
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let por_alcance: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_NEWS_SEND_BY_SCOPE);
    crate::comparacao::ramo("cpv_3 noticia da federacao");
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    construir(n, 0x177e, 0);
    gravar(n, 0, read_usize(selecao) as u32);
    gravar(n, 1, expectativa as i16 as i32 as u32);
    gravar(n, 2, competicao);
    por_alcance(read_usize(0xb7431c), n, selecao, 0, 0, 0, 0);
}

/// the 0x1782 news of the supporters protesting against the manager
/// Original: `post_supporter_protest_news` (0x6a5d10, manager_manager.cpp, thiscall).
///
/// [pt] { 0x6a5d10 } (thiscall, ret 4, manager_manager.cpp, quadro SEH sem destrutor): `MANAGER_MANAGER::
/// post_supporter_protest_news(clube)`: antes da data das notícias ([0xdd6ec8]), nada. A notícia 0x1782 (local): 0 = o
/// id do clube, 1..4 o técnico (+0xcf: o id e as três chaves +0x33 de +4, +8, +0xc); pelo alcance (0x786090).
pub(crate) unsafe extern "thiscall" fn noticia_do_protesto_da_torcida(_this: usize, clube: usize) {
    let dia = *(0xdd6ec8 as *const i16);
    let ano = *(0xdd6eca as *const u16);
    let hoje_ano = *(0xae2c92 as *const u16);
    if ano > hoje_ano || (ano == hoje_ano && dia > *(0xae2c90 as *const i16)) {
        return;
    }
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let por_alcance: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_NEWS_SEND_BY_SCOPE);
    crate::comparacao::ramo("cpv_3 noticia do protesto");
    let mut local = [0u32; 0x3a]; // a notícia local (0xe8 bytes)
    let n = local.as_mut_ptr() as usize;
    construir(n, 0x1782, 0);
    gravar(n, 0, read_usize(clube) as u32);
    gravar(n, 1, read_usize(read_usize(clube + 0xcf)) as u32);
    gravar(n, 2, read_usize(read_usize(read_usize(clube + 0xcf) + 4) + 0x33) as u32);
    gravar(n, 3, read_usize(read_usize(read_usize(clube + 0xcf) + 8) + 0x33) as u32);
    gravar(n, 4, read_usize(read_usize(read_usize(clube + 0xcf) + 0xc) + 0x33) as u32);
    por_alcance(read_usize(0xb7431c), n, clube, 0, 0, 0, 0);
}

// ponytail: completa, retira, recusa e reagenda ofertas (as chamadas liberam e alocam): não roda duas vezes. Só a
// original; a cópia leva os ramos e se prova pela carreira e pelas mutações.
pub(crate) unsafe extern "thiscall" fn comparar_ofertas_em_espera_na_janela(this: usize, nacao: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize)>(1239)(this, nacao)
}

// ponytail: a notícia vai pelo alcance (aloca): só a original; a cópia leva o ramo.
pub(crate) unsafe extern "thiscall" fn comparar_noticia_das_expectativas_da_federacao(this: usize, selecao: usize, expectativa: u32, competicao: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32)>(1240)(this, selecao, expectativa, competicao)
}

// ponytail: a notícia vai pelo alcance (aloca): só a original; a cópia leva o ramo.
pub(crate) unsafe extern "thiscall" fn comparar_noticia_do_protesto_da_torcida(this: usize, clube: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize)>(1241)(this, clube)
}

// ---------- 254a leva: as diretorias no início da carreira, as apelações de suspensão e a partida que o humano pode ver (manager_manager.cpp, human_manager.cpp) ----------







pub(crate) unsafe fn cpr5_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    f(n)
}

/// O técnico (ou o selecionador) do clube (0x53ca90).
pub(crate) unsafe fn cpr5_tecnico(clube: usize) -> usize {
    let f: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_CLUB_MANAGER);
    f(clube)
}

pub(crate) unsafe fn cpr5_humano(clube: usize, qualquer: u8, pessoa: usize) -> u32 {
    let f: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    f(clube, 0, qualquer, pessoa, 0)
}

/// Uma das confianças da linha (+6..+0xc): sorteio(`n1`) + `h` guardado em float, menos sorteio(500), truncado.
pub(crate) unsafe fn cpr5_confianca(c: usize, n1: i32, h: f32) {
    let r1 = cpr5_sortear(n1);
    let s = (r1 as f64 + h as f64) as f32;
    let r2 = cpr5_sortear(0x1f4);
    write_i16(c, ftol(s as f64 - r2 as f64) as i16);
}

/// Abaixo de 5500: 5500 + sorteio(250); acima de 10000: 10000.
pub(crate) unsafe fn cpr5_prender_5500(c: usize) {
    if read_i16(c) < 0x157c {
        write_i16(c, (cpr5_sortear(0xfa) + 0x157c) as i16);
    } else if read_i16(c) > 0x2710 {
        write_i16(c, 0x2710);
    }
}

/// Os campos da linha que as duas saídas zeram (+0x10..+0x16, os bits de +2, +3, +4, +5, +0x28 e +0x40).
pub(crate) unsafe fn cpr5_zerar_marcas(l: usize) {
    for k in [0x10usize, 0x12, 0x14, 0x16] {
        write_i16(l + k, 0);
    }
    write_u8(l + 2, read_u8(l + 2) & 0xe0);
    write_u8(l + 4, 0);
    write_u8(l + 3, read_u8(l + 3) & 0xe1);
    write_u8(l + 2, read_u8(l + 2) & 0x1f);
    write_u32(l + 0x28, 0);
    write_u8(l + 5, 0);
    write_u8(l + 3, read_u8(l + 3) & 0xfe);
    write_u8(l + 0x40, read_u8(l + 0x40) & 7);
    write_u8(l + 0x40, read_u8(l + 0x40) & 0xf8);
}

/// fixes the jobs of staff with a club (manager at +0xcf) and builds each club board row: dates, reputation squared,
/// the four confidences drawn from the expectation (board rating or reputation) and the reputation, held to
/// 5750/5500..10000, or 5000 without a manager; the season expectations (0x6a3770) and, without a manager, the vacancy
/// (0x69ba50)
/// Original: `MANAGER_MANAGER::init_club_board_records` (0x68f130, manager_manager.cpp, thiscall, ret 0).
///
/// [pt] { 0x68f130 } (thiscall, ret 0, manager_manager.cpp): `MANAGER_MANAGER::init_club_board_records()`: no começo da
/// carreira (pelo construtor 0x68dbf0). (1) Cada pessoa com clube (+0x24): função (+0x28) 5 ou 12 vira o técnico do
/// clube (+0xcf); 11 fica; 13 e 15 viram 11 (com ficha de jogador +0x61) ou 8; as outras perdem o clube; sem contrato
/// (+0x39) e sem função de origem (+0x3d), a função vai para +0x3d; fora de 11 a função zera. (2) Cada clube (as
/// seleções só com nação e +0x71): a segunda seleção tem a reputação (+0x80) presa a 200 × a da nação (+0x8e) e as
/// vagas zeradas; a linha dele (73 bytes em [this]) ganha as datas (23/07/1971 em +0x41, hoje em +0x18), a reputação² ×
/// 0,25 (double em +0x30) e zeros. Com técnico: a data 23/08/1971 em +0x20 (pela linha do id), a expectativa `f` (a
/// reputação, a nota da diretoria +0xa ou 4500, que cai a 0,75 × reputação), `h` = f / (reputação × 0,5 + 2500) ×
/// 7500 e as quatro confianças (+6, +8, +0xa, +0xc) por sorteios, presas embaixo (5750 ou 5500 com sorteio) e em
/// cima (10000; +6 acima de 9500 vira 10000 − sorteio(500)); zera as marcas e põe as expectativas da temporada
/// (0x6a3770 com 1). Sem técnico (fora do clube parceiro): confianças 5000, +0xe = 16, marcas zeradas; clube
/// dirigível (ou com competição e nação com +0x11c & 3) recebe as expectativas e a vaga (0x69ba50).
pub(crate) unsafe extern "thiscall" fn iniciar_diretorias_dos_clubes(this: usize) {
    // (1) as pessoas
    let mut k = 0i32;
    while k < read_usize(0xae2364) as i32 {
        let p = read_usize(0xae23bc).wrapping_add(k as usize * 0x6e);
        k += 1;
        let clube = read_usize(p + 0x24);
        if clube == 0 {
            continue;
        }
        let f = read_u8(p + 0x28);
        match f {
            5 | 12 => {
                write_u32(clube + 0xcf, p as u32);
                continue;
            }
            11 => continue,
            _ => {}
        }
        let mut tira = true;
        if f == 0xf || f == 0xd {
            if read_usize(p + 0x61) != 0 {
                write_u8(p + 0x28, 0xb);
                tira = false;
            } else {
                write_u8(p + 0x28, 8);
            }
        }
        if tira {
            write_u32(p + 0x24, 0);
        }
        if read_usize(p + 0x39) == 0 && read_u8(p + 0x3d) == 0 {
            write_u8(p + 0x3d, read_u8(p + 0x28));
        }
        if read_u8(p + 0x28) != 0xb {
            write_u8(p + 0x28, 0);
        }
    }
    // (2) os clubes
    let e_selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let segunda: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_CLUB_IS_NATIONAL_TEAM);
    let data_dma: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE_549F70);
    let copia: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let expectativas: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_MANAGER_MANAGER_SET_SEASON_EXPECTATIONS);
    let mut parceiro = 0u32; // a original guarda numa variável só, de clube em clube
    let mut i = 0i32;
    let mut ofs = 0usize;
    while i < read_usize(0xae235c) as i32 {
        let clube = read_usize(0xae23b4).wrapping_add(i as usize * 0x245);
        let l = read_usize(this).wrapping_add(ofs);
        i += 1;
        ofs += 0x49;
        if e_selecao(clube) != 0 {
            let n = read_usize(clube + 0x53);
            if n == 0 || read_usize(n + 0x71) == 0 {
                continue;
            }
        }
        if segunda(clube) != 0 {
            let n = read_usize(clube + 0x53);
            if n != 0 && read_i16(clube + 0x80) as i32 > read_i16(n + 0x8e) as i32 * 200 {
                write_i16(clube + 0x80, read_i16(n + 0x8e).wrapping_mul(200));
            }
            write_u32(clube + 0xcf, 0);
            write_u32(clube + 0xbf, 0);
            write_u32(clube + 0xd3, 0);
            core::ptr::write_bytes((clube + 0xc3) as *mut u8, 0, 12);
            core::ptr::write_bytes((clube + 0x19f) as *mut u8, 0, 0x14 + 0x1c + 12); // +0x19f..+0x1db
        }
        let (mut d, mut t) = ([0u8; 8], [0u8; 8]);
        let r = data_dma(d.as_mut_ptr() as usize, 0x17, 6, 0x7b3, 0xffff_ffff);
        copia(l + 0x41, t.as_mut_ptr() as usize, r);
        write_i16(l + 0xe, 0);
        write_u32(l + 0x2c, 0);
        copia(l + 0x18, t.as_mut_ptr() as usize, 0xae2c90);
        write_u8(l, 0);
        let rep = read_i16(clube + 0x80) as i32;
        core::ptr::write_unaligned((l + 0x30) as *mut f64, rep.wrapping_mul(rep) as f64 * 0.25);
        write_u32(l + 0x38, 0);
        write_u32(l + 0x3c, 0);
        if cpr5_tecnico(clube) == 0 {
            // sem técnico
            let parceiro_de: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
            if parceiro_de(clube, &mut parceiro, 1) != 0 && parceiro != 0 {
                continue;
            }
            crate::comparacao::ramo("cpr_5 diretoria sem tecnico");
            for c in [6usize, 8, 0xa, 0xc] {
                write_i16(l + c, 0x1388);
            }
            write_i16(l + 0xe, 0x10);
            cpr5_zerar_marcas(l);
            let dirigivel: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(FN_CLUB_CAN_BE_MANAGED);
            if dirigivel(clube) == 0 {
                if e_selecao(clube) != 0 {
                    continue;
                }
                let objs = read_usize(0xadadfc);
                let comp = read_usize(clube + 0x57);
                let n = read_usize(clube + 0x53);
                if objs == 0 || comp == 0 || n == 0 || read_usize(objs + read_usize(comp) * 4) == 0 || read_u8(n + 0x11c) & 3 == 0 {
                    continue;
                }
            }
            expectativas(this, clube, 1);
            let vaga: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_MANAGER_MANAGER_ADD_JOB_VACANCY);
            vaga(this, clube, 0);
            continue;
        }
        // com técnico
        let r = data_dma(d.as_mut_ptr() as usize, 0x17, 7, 0x7b3, 0xffff_ffff);
        copia(read_usize(this).wrapping_add(read_usize(clube).wrapping_mul(0x49)) + 0x20, t.as_mut_ptr() as usize, r);
        let mut f: f32;
        let mut direto = false; // a 4500 salta o teste do zero e do 500
        if read_usize(cpr5_tecnico(clube) + 0x69) == 0 && cpr5_humano(clube, 1, 0) == 0 {
            f = rep as f32;
        } else if cpr5_humano(clube, 1, 0) != 0 {
            if read_usize(clube + 0xcf) != 0 && read_usize(cpr5_tecnico(clube) + 0x69) != 0 {
                f = read_i16(read_usize(cpr5_tecnico(clube) + 0x69) + 0xa) as f32;
            } else {
                f = 4500.0;
                direto = true;
            }
        } else {
            f = read_i16(read_usize(cpr5_tecnico(clube) + 0x69) + 0xa) as f32;
            let teto = rep as f64 * 0.75;
            if (read_i16(read_usize(cpr5_tecnico(clube) + 0x69) + 0xa) as f64) < teto {
                let v = ftol(teto) as i16;
                write_i16(read_usize(cpr5_tecnico(clube) + 0x69) + 0xa, v); // a nota da diretoria cai a 0,75 × reputação
            }
        }
        if !direto {
            if f == 0.0 {
                let r80 = read_i16(clube + 0x80);
                f = if r80 != 0 { (r80 as f64 * 0.75) as f32 } else { 5000.0 };
            } else if f < 500.0 {
                f = (cpr5_sortear(0x3e8) + 0x1f4) as f32;
            }
        }
        let mut g = rep as f64 * 0.5 + 2500.0;
        if g == 0.0 {
            g = f as f64 * 1.5;
        } else if g < 500.0 {
            g = (cpr5_sortear(0x2ee) + 0xfa) as f64;
        }
        crate::comparacao::ramo("cpr_5 diretoria com tecnico");
        let h = ((f as f64 / g) * 7500.0) as f32;
        cpr5_confianca(l + 6, 0x1f4, h);
        let c = read_i16(l + 6);
        if c < 0x1676 {
            crate::comparacao::ramo("cpr_5 confianca baixa");
            write_i16(l + 6, (cpr5_sortear(0x2ee) + 0x1676) as i16);
        } else if c > 0x251c {
            write_i16(l + 6, (0x2710 - cpr5_sortear(0x1f4)) as i16);
        }
        cpr5_confianca(l + 8, 0x1f4, h);
        if read_i16(l + 8) < 0x1676 {
            write_i16(l + 8, (cpr5_sortear(0x1f4) + 0x1676) as i16);
        } else if read_i16(l + 8) > 0x2710 {
            write_i16(l + 8, 0x2710);
        }
        cpr5_confianca(l + 0xa, 0x6d6, h);
        cpr5_prender_5500(l + 0xa);
        cpr5_confianca(l + 0xc, 0x9c4, h);
        cpr5_prender_5500(l + 0xc);
        cpr5_zerar_marcas(l);
        expectativas(this, clube, 1);
    }
}

// ponytail: sorteia, põe as expectativas (0x6a3770) e abre vagas (0x69ba50, aloca): só a original; a cópia leva os
// ramos (roda uma vez, no começo da carreira).
pub(crate) unsafe extern "thiscall" fn comparar_iniciar_diretorias_dos_clubes(this: usize) {
    orig::<unsafe extern "thiscall" fn(usize)>(1095)(this)
}

/// for each appeal (+0x14) whose date has come: the result on up to 4 discipline rule sets (0x557000), the news
/// (0x6ac060) and the appeal leaves the list
/// Original: `MANAGER_MANAGER::process_ban_appeals` (0x6abef0, manager_manager.cpp, thiscall, ret 0).
///
/// [pt] { 0x6abef0 } (thiscall, ret 0, manager_manager.cpp): `MANAGER_MANAGER::process_ban_appeals()`: percorre a lista das
/// apelações (+0x14, a conta em palavra com sinal relida a cada volta) e, para cada uma cuja data (+5) já não é depois
/// de hoje (0x456740), com resultado (+4): as até 4 regras (+0xd..+0x10, ≥ 0) recebem o resultado na disciplina
/// (0x557000 com os campos +0x11..+0x18); depois a notícia do resultado (0x6ac060) e a apelação sai (0x68d1f0). Item
/// nulo: a caixa "Error" (0x57f5) e para. Pelo dia a dia da diretoria (0x68f9c0); só as apelações do humano entram na
/// lista.
pub(crate) unsafe extern "thiscall" fn apelacoes_de_suspensao(this: usize) {
    let lista = || read_usize(this + 0x14);
    write_u32(lista() + 0xa, read_usize(lista() + 2) as u32);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let depois: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_DATE_OPERATOR_3E);
    let aplicar: unsafe extern "thiscall" fn(usize, u32, i8, u8, u8, i16, u8, i16, u8) = core::mem::transmute(FN_DISCIPLINE_APPLY_APPEAL_RESULT);
    let noticia: unsafe extern "thiscall" fn(usize, usize, u8, u8, i16, u8) = core::mem::transmute(FN_MANAGER_MANAGER_POST_BAN_APPEAL_RESULT_NEWS);
    let remover: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_LLIST_REMOVE);
    let mut k = 0i32;
    while k < read_i16(lista()) as i32 {
        let item = proximo(lista());
        if item == 0 {
            return aviso_em(0x9f18a4, 0x57f5);
        }
        if depois(item + 5, read_usize(0xae2c90) as u32, read_usize(0xae2c94) as u32) == 0 {
            crate::comparacao::ramo("cpr_5 apelacao decidida");
            let p = read_usize(0xae23bc).wrapping_add(read_usize(item).wrapping_mul(0x6e));
            if read_u8(item + 4) != 0 {
                for j in 0..4usize {
                    let regra = read_u8(item + 0xd + j) as i8;
                    if regra > -1 {
                        // ponytail: a original empurra os campos em registradores com lixo em cima; a 0x557000 lê só
                        // o byte ou a palavra de cada um.
                        aplicar(read_usize(0xae28f0), read_usize(p) as u32, regra, read_u8(item + 0x11), read_u8(item + 0x12),
                                read_i16(item + 0x13), read_u8(item + 0x15), read_i16(item + 0x16), read_u8(item + 0x18));
                    }
                }
            }
            noticia(this, p, read_u8(item + 4), read_u8(item + 0x15), read_i16(item + 0x16), read_u8(item + 0x18));
            remover(lista(), item);
        }
        k += 1;
    }
}

// ponytail: aplica a disciplina, solta a notícia e libera a apelação: só a original; a cópia leva o ramo.
pub(crate) unsafe extern "thiscall" fn comparar_apelacoes_de_suspensao(this: usize) {
    orig::<unsafe extern "thiscall" fn(usize)>(1096)(this)
}

/// 1 when the fixture has a negative +0x43 and neither it nor the fixtures of that day (each key nation group) have a
/// club of the human; else 0
/// Original: `HUMAN_MANAGER::can_watch_fixture` (0x5fe700, human_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x5fe700 } (thiscall, ret 8, human_manager.cpp): `HUMAN_MANAGER::can_watch_fixture(partida, humano)`: o humano
/// (ou, nulo, o da tela atual [0xb74c26 + 0x300 × [0xb77c7a]]) pode ver a partida se ela tem +0x43 negativo, nenhum dos
/// dois clubes (+0x1c, +0x20, com as marcas +0x40, +0x41) é dele (0x600a50) e, em cada grupo das nações-chave
/// ([0xb64738]), nenhuma partida da lista do dia dela (data +0x28/+0x2a, fatia +0x3f; 0x5ae6a0) tem um clube dele: 1.
/// Senão 0. Partida nula: a caixa "Error" (0x707) e 0. Pela limpeza das partidas assistidas (0x5fe8d0) e pela escolha
/// da partida (0x5fe500).
pub(crate) unsafe extern "thiscall" fn pode_ver_a_partida(this: usize, partida: usize, humano: usize) -> u32 {
    if partida == 0 {
        aviso_em(0x9cf11c, 0x707);
        return 0;
    }
    let mut humano = humano;
    if humano == 0 {
        humano = read_usize(0xb74c26usize.wrapping_add((read_i16(0xb77c7a) as isize as usize).wrapping_mul(0x300)));
        if humano == 0 {
            return 0;
        }
    }
    if read_u8(partida + 0x43) as i8 > -1 {
        return 0;
    }
    let dele = |p: usize| cpr5_humano(read_usize(p + 0x1c), read_u8(p + 0x40), humano) != 0
        || cpr5_humano(read_usize(p + 0x20), read_u8(p + 0x41), humano) != 0;
    if dele(partida) {
        return 0;
    }
    let data_nova: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE);
    let lista_de: unsafe extern "stdcall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_FIX_MAN_GET_FIXTURE_LIST_FOR_DATE);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let mut grupo = 0u8;
    while (grupo as i8) < read_u8(0xb64738) as i8 {
        let mut d = [0u8; 8];
        let data = data_nova(d.as_mut_ptr() as usize, read_i16(partida + 0x2a) as u16 as u32, read_i16(partida + 0x28) as u16 as u32);
        // ponytail: a original empurra o grupo como palavra inteira (os bytes de cima são sobra de pilha); a 0x5ae6a0
        // repassa o tipo (char) e a 0x5abf10 lê só o byte.
        let lista = lista_de(data, read_u8(partida + 0x3f) as u32, grupo as u32);
        if lista != 0 {
            write_u32(lista + 0xa, read_usize(lista + 2) as u32);
            let mut p = proximo(lista);
            while p != 0 {
                if dele(p) {
                    return 0;
                }
                p = proximo(lista);
            }
        }
        grupo = grupo.wrapping_add(1);
    }
    1
}

/// Comparador: a original e a cópia (as duas só leem; a volta da lista recomeça o cursor), pelo valor.
pub(crate) unsafe extern "thiscall" fn comparar_pode_ver_a_partida(this: usize, partida: usize, humano: usize) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(1097)(this, partida, humano);
    if partida == 0 {
        return r;
    }
    let n = pode_ver_a_partida(this, partida, humano);
    PLACAR.registrar(r == n, || format!("pode_ver_a_partida(partida {:x}, humano {:x}) original={} rust={}", partida, humano, r, n));
    r
}

// ---------- 253a leva: a diretoria e o técnico: pensar em demitir o técnico, dar o ultimato e tirar o técnico do cargo (manager_manager.cpp) ----------


pub(crate) unsafe fn cpq1_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}




/// A constante double do exe em `a`.
pub(crate) unsafe fn cpq1_k(a: usize) -> f64 {
    core::ptr::read_unaligned(a as *const f64)
}

/// A linha da diretoria do clube (73 bytes em [this], pelo id do clube).
pub(crate) unsafe fn cpq1_linha(this: usize, clube: usize) -> usize {
    read_usize(this).wrapping_add(read_usize(clube).wrapping_mul(0x49))
}

/// O registro de não jogador do presidente do clube (+0xbf, +0x69), ou 0.
pub(crate) unsafe fn cpq1_presidente(clube: usize) -> usize {
    let p = read_usize(clube + 0xbf);
    if p == 0 {
        0
    } else {
        read_usize(p + 0x69)
    }
}

pub(crate) unsafe fn cpq1_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    f(n)
}

/// Os dias da data `(d0, d1)` (por valor) até a data do jogo (0x54cdb0).
pub(crate) unsafe fn cpq1_dias(d0: usize, d1: usize) -> i32 {
    let f: unsafe extern "thiscall" fn(usize, usize, usize) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
    f(0xae2c90, d0, d1)
}

pub(crate) unsafe fn cpq1_acrescentar(lista: usize, item: usize, n: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_LLIST_ADD_TAIL);
    f(lista, item, n);
}

/// A saída do técnico na lista das saídas [this+4] (6 bytes: `tipo`, o motivo 0x69b350 e o id do clube) e o bit 2 em
/// +2 da linha da diretoria.
pub(crate) unsafe fn cpq1_saida(this: usize, clube: usize, tipo: u8) {
    let id = read_usize(clube) as u32;
    let motivo: unsafe extern "thiscall" fn(usize, u32) -> u8 = core::mem::transmute(FN_MANAGER_MANAGER_GET_DEPARTURE_REASON);
    let mut reg = [0u8; 8];
    reg[0] = tipo;
    reg[2..6].copy_from_slice(&id.to_le_bytes());
    reg[1] = motivo(this, id);
    cpq1_acrescentar(read_usize(this + 4), reg.as_mut_ptr() as usize, 6);
    *((cpq1_linha(this, clube) + 2) as *mut u8) |= 2;
}

/// A nota do técnico atual (0x69d1b0 com 0 e 5; 0 sem técnico ou sem registro de não jogador) contra a melhor nota de
/// candidato (0x69c540): passando de 450 a mais, a confiança (+6 da linha) cai a 90% (mínimo 1) e, com mais de 30 dias
/// desde +0x18 da linha, sai a notícia da pressão (0x6a0e70 com 1). Devolve se passou.
pub(crate) unsafe fn cpq1_trocar_pelo_melhor(this: usize, clube: usize) -> bool {
    let nota_de: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> i32 = core::mem::transmute(FN_SCORE_STAFF_FOR_JOB);
    let melhor: unsafe extern "thiscall" fn(usize, usize, usize) -> i32 = core::mem::transmute(FN_MANAGER_MANAGER_BEST_MANAGER_CANDIDATE_SCORE);
    let tec = read_usize(clube + 0xcf);
    let nota = if tec != 0 && read_usize(tec + 0x69) != 0 { nota_de(this, tec, clube, 0, 5) } else { 0 };
    let m = melhor(this, clube, read_usize(clube + 0xcf));
    if nota <= m.wrapping_add(0x1c2) {
        return false;
    }
    let c = cpq1_linha(this, clube) + 6;
    write_i16(c, ftol(read_i16(c) as i32 as f64 * cpq1_k(0x9689d8)) as i16); // 0,9
    if read_i16(c) < 1 {
        write_i16(c, 1);
    }
    let l = cpq1_linha(this, clube);
    if cpq1_dias(read_usize(l + 0x18), read_usize(l + 0x1c)) > 0x1e {
        let pressao: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_MANAGER_MANAGER_POST_MANAGER_UNDER_PRESSURE_NEWS);
        pressao(this, clube, 1);
    }
    true
}

/// with low board confidence and the manager in the job for 150 days: the limit from the division table, the runs and
/// the manager mark; chairman draws hold back, issue the ultimatum, let the manager leave or sack him (the departure
/// queued, reason 0x69b350), or look for a better candidate with the pressure story
/// Original: `MANAGER_MANAGER::consider_sacking_manager` (0x69aba0, manager_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x69aba0 } (thiscall, ret 8, manager_manager.cpp): `MANAGER_MANAGER::consider_sacking_manager(clube, conferir)`:
/// antes da data [0xdd6ec8] (0x456770), com o bit 2 em +2 da linha da diretoria, com o ultimato resolvido (0x6ac470,
/// só com `conferir`), com a confiança (+6) acima de 4275 ou com o técnico no cargo há menos de 150 dias: nada. Técnico
/// bem visto (+0xa do não jogador acima de 5000 e acima da reputação do clube +0x80 − 750) há menos de 400 dias: com
/// sorteio(paciência do presidente +0x1d, ou 10) ≠ 0 a confiança vai a 4275. O limite (10 acima de 3500, 500 até 3000,
/// 200 no meio) muda pela linha do clube na tabela da divisão (0x4a4850): posição (+4) acima de 30 − paciência/2 nas
/// vagas de cima (+0xbf, ou +0xbe pela virtual +0x98 da competição) segura; 5 últimos resultados (+0x14..+0x18) com
/// média acima de 2 seguram; as séries +0x19, +0x41, +0x55 e +0x2d mexem no limite (×1,5 em +0x55). Com o técnico
/// marcado (0x7a7eb0), o limite ×0,25. Sorteio(paciência × 50, ou 500) acima do limite segura; sorteio(paciência + 1,
/// ou 11) = 0 dá o ultimato (0x6ac560; dado, nada mais). Técnico da máquina (0x600a50), sem a marca, com sorteio(+0x5a)
/// e sorteio(4) nulos sai (saída 2). Com presidente: no clube de liga ou seleção, sorteio(paciência) < 5 e sorteio(20)
/// ≠ 0 tenta trocar pelo melhor candidato; senão a notícia da pressão (0x6a0e70 com 2) depois de 14 dias, e só. Sem
/// presidente: sorteio(3) = 0 tenta a troca. Sem troca, a confiança do não jogador cai 250 (mínimo 1500 com presidente,
/// 1 sem) e o técnico é demitido (saída 1).
pub(crate) unsafe extern "thiscall" fn pensar_em_demitir_o_tecnico(this: usize, clube: usize, conferir: u32) {
    let antes: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_DATE_OPERATOR_3C);
    if antes(0xae2c90, read_usize(0xdd6ec8) as u32, read_usize(0xdd6ecc) as u32) != 0 {
        return;
    }
    if read_u8(cpq1_linha(this, clube) + 2) & 2 != 0 {
        return;
    }
    if conferir as u8 != 0 {
        let resolver: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_MANAGER_MANAGER_RESOLVE_BOARD_ULTIMATUM);
        if resolver(this, clube) != 0 {
            return;
        }
    }
    if read_i16(cpq1_linha(this, clube) + 6) > 0x10b3 {
        return;
    }
    let tec = read_usize(clube + 0xcf);
    if cpq1_dias(read_usize(tec + 0x3e), read_usize(tec + 0x42)) < 0x96 {
        return;
    }
    let tec = read_usize(clube + 0xcf);
    if tec != 0 && read_usize(tec + 0x69) != 0 {
        let conf = read_i16(read_usize(tec + 0x69) + 0xa);
        if conf > 0x1388 && read_i16(clube + 0x80).wrapping_sub(0x2ee) < conf
            && cpq1_dias(read_usize(tec + 0x3e), read_usize(tec + 0x42)) < 0x190 {
            let pres = cpq1_presidente(clube);
            let n = if pres != 0 { read_u8(pres + 0x1d) as i8 as i32 } else { 10 };
            if cpq1_sortear(n) != 0 {
                write_i16(cpq1_linha(this, clube) + 6, 0x10b3);
                return;
            }
        }
    }
    let v = read_i16(cpq1_linha(this, clube) + 6);
    let mut limite: i32 = if v > 0xdac { 10 } else if v <= 0xbb8 { 500 } else { 200 };
    let div = read_usize(clube + 0x57);
    let comp = if div != 0 { read_usize(read_usize(0xadadfc).wrapping_add(read_usize(div).wrapping_mul(4))) } else { 0 };
    if comp != 0 && read_usize(comp + 8) != 0 {
        let busca: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_COMP_STATS_FIND_CLUB_ENTRY);
        let ln = busca(read_usize(comp + 8), clube);
        if ln != 0 {
            let pres = cpq1_presidente(clube);
            let paciencia: i8 = if pres != 0 { (read_u8(pres + 0x1d) as i8) >> 1 } else { 5 };
            let teto = 0x1e - paciencia as i32;
            let comp = read_usize(read_usize(0xadadfc).wrapping_add(read_usize(read_usize(clube + 0x57)).wrapping_mul(4)));
            let vagas = read_u8(comp + 0xbf) as i8;
            if vagas != 0 && read_u8(ln + 0x74) as i32 <= vagas as i32 && read_u8(ln + 4) as i32 > teto {
                return;
            }
            let virtual_98: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(comp) + 0x98));
            if virtual_98(comp) != 0 {
                let comp = read_usize(read_usize(0xadadfc).wrapping_add(read_usize(read_usize(clube + 0x57)).wrapping_mul(4)));
                if read_u8(ln + 0x74) as i32 <= read_u8(comp + 0xbe) as i8 as i32 && read_u8(ln + 4) as i32 > teto {
                    return;
                }
            }
            let (mut pontos, mut jogos) = (0i32, 0i32);
            for k in (0x14..=0x18usize).rev() {
                match read_u8(ln + k) {
                    3 => {
                        pontos += 3;
                        jogos += 1;
                    }
                    1 | 2 => {
                        pontos += 1;
                        jogos += 1;
                    }
                    0xfd => jogos += 1,
                    _ => {}
                }
            }
            if jogos as i16 > 0 && ((pontos as i16 as i32) / (jogos as i16 as i32)) as f64 > cpq1_k(0x968950) {
                return; // média acima de 2
            }
            let s19 = cpq1_i(ln + 0x19);
            if s19 > 1 {
                if limite as i16 == 10 || s19 > 2 {
                    return;
                }
                if limite as i16 == 200 {
                    limite = 0x32;
                }
            } else {
                let s41 = cpq1_i(ln + 0x41);
                if s41 > 1 {
                    if limite as i16 == 10 || s41 > 3 {
                        return;
                    }
                    if limite as i16 == 200 {
                        limite = 100;
                    }
                } else if cpq1_i(ln + 0x55) > 1 {
                    limite = ftol(limite as i16 as i32 as f64 * cpq1_k(0x968940)); // 1,5
                } else if cpq1_i(ln + 0x2d) > 1 {
                    if limite as i16 == 10 {
                        limite = 200;
                    } else if limite as i16 == 200 {
                        limite = 300;
                    }
                }
            }
        }
    }
    let marca: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_PLAYER_REGEN_GET_RETIREMENT_STATUS);
    let teto = if marca(read_usize(0xb7434c), read_usize(clube + 0xcf)) != 0 {
        ftol(limite as i16 as i32 as f64 * cpq1_k(0x9689e0)) // 0,25
    } else {
        limite
    };
    let pres = cpq1_presidente(clube);
    let r = if pres != 0 {
        cpq1_sortear(((read_u8(pres + 0x1d) as i8 as i32) * 50) as i16 as i32)
    } else {
        cpq1_sortear(500)
    };
    if r > teto as i16 as i32 {
        return;
    }
    let pres = cpq1_presidente(clube);
    let ultimato = if pres != 0 { cpq1_sortear(read_u8(pres + 0x1d) as i8 as i32 + 1) == 0 } else { cpq1_sortear(0xb) == 0 };
    if ultimato {
        let dar: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_MANAGER_MANAGER_ISSUE_BOARD_ULTIMATUM);
        if dar(this, clube) != 0 {
            return;
        }
    }
    if read_usize(clube + 0xcf) != 0 {
        let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
        if humano(clube, 0, 1, 0, 0) == 0 && marca(read_usize(0xb7434c), read_usize(clube + 0xcf)) == 0
            && cpq1_sortear(read_u8(read_usize(clube + 0xcf) + 0x5a) as i8 as i32) == 0 && cpq1_sortear(4) == 0 {
            crate::comparacao::ramo("cpq_1 o tecnico sai");
            cpq1_saida(this, clube, 2);
            return;
        }
    }
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let tem_presidente = cpq1_presidente(clube) != 0;
    if tem_presidente {
        if (selecao(clube) != 0 || read_usize(clube + 0x57) != 0)
            && cpq1_sortear(read_u8(cpq1_presidente(clube) + 0x1d) as i8 as i32) < 5 && cpq1_sortear(0x14) != 0 {
            if cpq1_trocar_pelo_melhor(this, clube) {
                return;
            }
        } else {
            let l = cpq1_linha(this, clube);
            if cpq1_dias(read_usize(l + 0x18), read_usize(l + 0x1c)) > 0xe {
                let pressao: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_MANAGER_MANAGER_POST_MANAGER_UNDER_PRESSURE_NEWS);
                pressao(this, clube, 2);
            }
            return;
        }
    } else if (selecao(clube) != 0 || read_usize(clube + 0x57) != 0) && cpq1_sortear(3) == 0 && cpq1_trocar_pelo_melhor(this, clube) {
        return;
    }
    let tec = read_usize(clube + 0xcf);
    if tec != 0 && read_usize(tec + 0x69) != 0 {
        let nj = read_usize(tec + 0x69);
        write_i16(nj + 0xa, read_i16(nj + 0xa).wrapping_sub(250));
        let piso: i16 = if tem_presidente { 0x5dc } else { 1 };
        if read_i16(nj + 0xa) < piso {
            write_i16(nj + 0xa, piso);
        }
    }
    crate::comparacao::ramo("cpq_1 demitido");
    cpq1_saida(this, clube, 1);
}

// ponytail: sorteia e, conforme o sorteio, aloca (a saída na lista, o ultimato, a notícia): o caminho só se sabe
// rodando. Só a original; a cópia leva os ramos e a carreira a prova (as demissões mudam tudo).
pub(crate) unsafe extern "thiscall" fn comparar_pensar_em_demitir_o_tecnico(this: usize, clube: usize, conferir: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(1045)(this, clube, conferir)
}

/// O ultimato com a lista local (já construída): o primeiro jogo do clube (0x5acbe0 do gerenciador [0xae2a58]: de hoje
/// a 8 semanas, 0x54c770, qualquer competição, tipo −2, lado 1) que não é hoje, com +0x40 e +0x43 = 0xff, com
/// competição (+0x14): a notícia (0x6ac700 com o adversário, +0x1c ou +0x20) e o id do clube na lista dos ultimatos
/// [this+0x10]; devolve 1.
pub(crate) unsafe fn cpq1_ultimato(this: usize, clube: usize, l: usize) -> u8 {
    if clube == 0 || read_usize(clube + 0xcf) == 0 || read_usize(clube + 0x57) == 0 {
        return 0;
    }
    let nacao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_KEY_NATION_INDEX_OF_CLUB);
    nacao(clube); // a original descarta o valor
    let semanas: unsafe extern "thiscall" fn(usize, *mut u8, u32) -> usize = core::mem::transmute(FN_DATE_ADD_WEEKS);
    let (mut d1, mut d2) = ([0u8; 8], [0u8; 8]);
    let p1 = semanas(0xae2c90, d1.as_mut_ptr(), 8);
    let ano_fim = read_i16(p1 + 2) as u16 as u32;
    let p2 = semanas(0xae2c90, d2.as_mut_ptr(), 8);
    let dia_fim = read_i16(p2) as u16 as u32;
    // ponytail: a original empurra as palavras com a metade alta do registro: no dia de hoje e no dia do fim a do
    // ponteiro da segunda data, no ano de hoje a do ponteiro da lista (reproduzidas pelos daqui); no ano do fim a do
    // edi de quem chamou, que a 0x5acbe0 descarta (lê só as palavras do fim).
    let alto = |p: usize| (p as u32) & 0xffff_0000;
    let achar: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, usize, i32, i32, i32) -> u32 =
        core::mem::transmute(FN_FIXTURE_MANAGER_FIND_FIXTURES);
    achar(read_usize(0xae2a58), alto(p2) | read_i16(0xae2c90) as u16 as u32, alto(l) | read_i16(0xae2c92) as u16 as u32,
          alto(p2) | dia_fim, ano_fim, read_usize(clube) as u32, l, -1, -2, 1);
    if read_i16(l) == 0 {
        return 0;
    }
    core::ptr::write_unaligned((l + 0xa) as *mut u32, read_usize(l + 2) as u32);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let mut e = proximo(l);
    while e != 0 {
        let hoje = read_i16(e + 0x2a) as u16 as i32 == read_i16(0xae2c90) as i32 && read_i16(e + 0x28) == read_i16(0xae2c92);
        if !hoje && read_u8(e + 0x40) != 0 && read_u8(e + 0x43) == 0xff {
            break;
        }
        e = proximo(l);
    }
    if e == 0 || read_usize(e + 0x14) == 0 {
        return 0;
    }
    let adversario = if read_usize(e + 0x1c) == clube { read_usize(e + 0x20) } else { read_usize(e + 0x1c) };
    let noticia: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(FN_MANAGER_MANAGER_POST_BOARD_ULTIMATUM_NEWS);
    noticia(this, clube, adversario, read_usize(e + 0x14));
    crate::comparacao::ramo("cpq_1 ultimato dado");
    let id = read_usize(clube) as u32;
    cpq1_acrescentar(read_usize(this + 0x10), &id as *const u32 as usize, 4);
    1
}

/// the club's first match in the next 8 weeks (not today, still to play): the ultimatum story with the opponent and the
/// club in the ultimatum list; returns 1, or 0 without a match
/// Original: `MANAGER_MANAGER::issue_board_ultimatum` (0x6ac560, manager_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x6ac560 } (thiscall, ret 4, manager_manager.cpp, SEH da lista local): `MANAGER_MANAGER::issue_board_ultimatum(clube)`:
/// clube com técnico (+0xcf) e divisão (+0x57): a lista local (LLIST 0x68d020, solta pela 0x68d040) recebe os jogos
/// das próximas 8 semanas; o primeiro que vale dá a notícia do ultimato e põe o clube na lista dos ultimatos
/// [this+0x10]: devolve 1. Senão 0.
pub(crate) unsafe extern "thiscall" fn dar_o_ultimato_da_diretoria(this: usize, clube: usize) -> u8 {
    let mut lista = [0u8; 16];
    let l = lista.as_mut_ptr() as usize;
    let nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    nova(l);
    let r = cpq1_ultimato(this, clube, l);
    soltar(l);
    r
}

// ponytail: aloca a lista dos jogos, a notícia e o item da lista dos ultimatos: só a original, conferida pelo plano:
// a lista dos ultimatos [this+0x10] cresce de 1 quando devolve 1 e fica igual com 0.
pub(crate) unsafe extern "thiscall" fn comparar_dar_o_ultimato_da_diretoria(this: usize, clube: usize) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u8>(1046);
    if read_usize(this + 0x10) == 0 {
        return o(this, clube);
    }
    let antes = read_i16(read_usize(this + 0x10)) as i32;
    let r = o(this, clube);
    let depois = read_i16(read_usize(this + 0x10)) as i32;
    PLACAR.registrar(depois == antes + (r != 0) as i32, || {
        format!("dar_o_ultimato_da_diretoria(clube {:x}) devolveu {}, ultimatos {} -> {}", clube, r, antes, depois)
    });
    r
}

/// As marcas do contrato que a saída do técnico apaga: lidas pela 0x8676f0 (+0x40; a 0x10000000 é apagada pela
/// 0x8674e0, que mexe no +0x45) e lidas no +0x45 (apagadas pela 0x8674e0, que também atualiza a pessoa).
pub(crate) const CPQ1_PELA_LEITURA: [(u32, usize); 3] = [(0x1000_0000, 0x8674e0), (0x80, 0x8676d0), (0x100, 0x8676d0)];
pub(crate) const CPQ1_NO_45: [u32; 8] = [0x800_0000, 8, 0x200, 0x400, 0x2000, 0x1_0000, 0x8_0000, 0x80_0000];

/// O técnico do clube no cargo (pelo +0x39 do clube contratado, cargo +0x3d; ou pelo +0x24 da seleção, cargo +0x28):
/// (técnico, cargo).
pub(crate) unsafe fn cpq1_tecnico_no_cargo(clube: usize) -> Option<(usize, u8)> {
    let tec = read_usize(clube + 0xcf);
    if tec == 0 {
        return None;
    }
    if read_usize(tec + 0x39) == clube {
        Some((tec, read_u8(tec + 0x3d)))
    } else if read_usize(tec + 0x24) == clube {
        Some((tec, read_u8(tec + 0x28)))
    } else {
        None
    }
}

/// the manager leaves: the departure date, the squad and the staff, the contract (outside national teams), the story
/// (outside type 3), confidence at 3500 (unless kept), the job change record, the fines and the squad contracts'
/// grievances and flags
/// Original: `MANAGER_MANAGER::remove_club_manager` (0x69b790, manager_manager.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x69b790 } (thiscall, ret 0x10, manager_manager.cpp): `MANAGER_MANAGER::remove_club_manager(clube, tipo, motivo,
/// manter)`: com o técnico no cargo: no clube humano (0x600a50) as respostas automáticas (0x7874b0); a data da saída
/// (+0x29 na seleção, +0x3e no clube) vira a de hoje; com cargo de técnico (12, 13 ou 15) sai do elenco (0x690730);
/// sai da comissão (0x690890); fora da seleção perde o contrato (0x4d0b50); fora do tipo 3 a notícia da saída (0x6a1b00
/// com 2 para os cargos de técnico, 3 para os outros); sem `manter` a confiança (+6 da linha) vai a 3500. O registro
/// da troca (0x869aa0, com 0 no tipo 2 e 2 nos outros), as multas do clube (0x5a5960) e, fora da seleção, as queixas e
/// marcas de cada jogador do elenco com contrato (0x4cf5a0) saem.
pub(crate) unsafe extern "thiscall" fn tirar_o_tecnico_do_cargo(this: usize, clube: usize, tipo: u32, motivo: u32, manter: u32) {
    let (tec, cargo) = match cpq1_tecnico_no_cargo(clube) {
        Some(t) => t,
        None => return,
    };
    crate::comparacao::ramo("cpq_1 tirar o tecnico");
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    if humano(clube, 1, 1, 0, 0) != 0 {
        let responder: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_NEWS_AUTO_RESPOND_ALL);
        responder(read_usize(0xb7431c), tec, clube);
    }
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let copia: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let mut t = [0u8; 8];
    copia(if selecao(clube) != 0 { tec + 0x29 } else { tec + 0x3e }, t.as_mut_ptr(), 0xae2c90);
    let c = cargo as i8;
    let de_tecnico = c >= 0xc && (c <= 0xd || c == 0xf);
    let sair: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_MANAGER_MANAGER_REMOVE_STAFF_FROM_CLUB);
    if de_tecnico {
        let do_elenco: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_MANAGER_MANAGER_REMOVE_PLAYER_FROM_SQUAD);
        do_elenco(this, tec, clube);
    }
    sair(this, tec, clube);
    if selecao(clube) == 0 {
        let sem_contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_REMOVE_CONTRACT);
        sem_contrato(read_usize(0xae1050), tec);
    }
    if tipo as u8 != 3 {
        let noticia: unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, u32) = core::mem::transmute(FN_MANAGER_MANAGER_POST_MANAGER_DEPARTURE_NEWS);
        noticia(this, tec, tipo, motivo, clube, if de_tecnico { 2 } else { 3 });
    }
    if manter == 0 {
        write_i16(cpq1_linha(this, clube) + 6, 0xdac);
    }
    let registrar: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(FN_STAFF_RECORDS_RECORD_MANAGER_JOB_CHANGE);
    registrar(read_usize(0xdd7f00), tec, clube, if tipo as u8 == 2 { 0 } else { 2 });
    if read_usize(0xae2a3c) != 0 {
        let multas: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_FINE_CLEAR_CLUB_FINE_COUNT);
        multas(read_usize(0xae2a3c), clube);
    }
    if selecao(clube) != 0 {
        return;
    }
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let marcas: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_STAFF_CONTRACT_GET_FLAGS);
    let queixa: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_STAFF_CONTRACT_CLEAR_GRIEVANCES);
    for k in 0..50usize {
        let p = read_usize(clube + 0xd7 + 4 * k);
        if p == 0 || read_usize(p + 0x61) == 0 {
            continue;
        }
        let ct = contrato(read_usize(0xae1050), p);
        if ct == 0 {
            continue;
        }
        for (m, f) in CPQ1_PELA_LEITURA {
            if marcas(ct) & m != 0 {
                let apagar: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(f);
                apagar(ct, m);
            }
        }
        for m in CPQ1_NO_45 {
            if read_usize(ct + 0x45) as u32 & m != 0 {
                queixa(ct, m);
            }
        }
    }
}

// ponytail: tira o contrato, solta a notícia e grava o histórico em arquivo (não roda duas vezes): só a original,
// conferida depois: sem `manter` a confiança em 3500 e, fora da seleção, nenhum contrato do elenco com as marcas.
pub(crate) unsafe extern "thiscall" fn comparar_tirar_o_tecnico_do_cargo(this: usize, clube: usize, tipo: u32, motivo: u32, manter: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32)>(1047);
    let tira = cpq1_tecnico_no_cargo(clube).is_some();
    o(this, clube, tipo, motivo, manter);
    if !tira {
        return;
    }
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let marcas: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_STAFF_CONTRACT_GET_FLAGS);
    let mut sobra = 0u32;
    if selecao(clube) == 0 {
        for k in 0..50usize {
            let p = read_usize(clube + 0xd7 + 4 * k);
            if p == 0 || read_usize(p + 0x61) == 0 {
                continue;
            }
            let ct = contrato(read_usize(0xae1050), p);
            if ct != 0 {
                // a 0x10000000 é lida em +0x40 (0x8676f0) e apagada em +0x45 (0x8674e0): não dá para conferir
                sobra |= (marcas(ct) & 0x180) | (read_usize(ct + 0x45) as u32 & 0x889_2608);
            }
        }
    }
    let conf = read_i16(cpq1_linha(this, clube) + 6);
    PLACAR.registrar(sobra == 0 && (manter != 0 || conf == 0xdac), || {
        format!("tirar_o_tecnico_do_cargo(clube {:x}, tipo {}, manter {}) marcas que sobraram {:#x}, confiança {}", clube, tipo as u8, manter, sobra, conf)
    });
}

// ---------- 253a leva: a federação e a diretoria: avaliar a seleção na competição, o clube em apuro financeiro e a faixa da confiança da diretoria (manager_manager.cpp) ----------




/// A linha da diretoria (ou da federação) do clube: 73 bytes em [this], pelo id.
pub(crate) unsafe fn cpq2_linha(this: usize, clube: usize) -> usize {
    read_usize(this).wrapping_add(read_usize(clube).wrapping_mul(0x49))
}

pub(crate) unsafe fn cpq2_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    f(n)
}

/// A confiança `c` (+6 da linha) somada de `v` (16 bits), presa a `[piso, teto]` (primeiro o teto).
pub(crate) unsafe fn cpq2_somar(c: usize, v: i16, piso: i16, teto: i16) {
    write_i16(c, read_i16(c).wrapping_add(v));
    if read_i16(c) > teto {
        write_i16(c, teto);
    } else if read_i16(c) < piso {
        write_i16(c, piso);
    }
}

/// A saída do técnico da seleção na lista das saídas [this+4] (6 bytes: 1, `motivo`, o id da seleção).
pub(crate) unsafe fn cpq2_saida(this: usize, selecao: usize, motivo: u8) {
    let mut reg = [0u8; 8];
    reg[0] = 1;
    reg[1] = motivo;
    reg[2..6].copy_from_slice(&(read_usize(selecao) as u32).to_le_bytes());
    let acrescentar: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_LLIST_ADD_TAIL);
    acrescentar(read_usize(this + 4), reg.as_mut_ptr() as usize, 6);
}

/// A federação pensa em demitir (0x69aba0 sem conferir o ultimato).
pub(crate) unsafe fn cpq2_pensar(this: usize, selecao: usize) {
    crate::comparacao::ramo("cpq_2 a federacao pensa");
    let pensar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_MANAGER_MANAGER_CONSIDER_SACKING_MANAGER);
    pensar(this, selecao, 0);
}

/// the FA confidence from the manager grade in the competition (−3..3, virtual +0x74), the assessment story and, by
/// time in the job, reputation and draws, the manager departure or the FA considering the sacking (0x69aba0)
/// Original: `MANAGER_MANAGER::review_national_team_performance` (0x6a5610, manager_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x6a5610 } (thiscall, ret 8, manager_manager.cpp): `MANAGER_MANAGER::review_national_team_performance(comp,
/// selecao)`: seleção com técnico (+0xcf) e competição em [0xadadfc]: com o técnico há menos de 365 dias, a confiança
/// da federação (+6 da linha) sobe a 6500. A nota do técnico na competição (virtual +0x74 dela) muda a confiança: 3:
/// +1250 (teto 10000, piso 8500); 2: +750 (teto 10000, piso 6500), e a seleção da máquina com o técnico
/// há mais de 3650 dias sai se sorteio(15000) + 3750 < reputação (+0x80); 0: −250 e, só com +0xe da linha = −3, 3500 e a
/// saída (mais de 3650 dias) ou, com reputação acima de 6750 e mais de 1460 dias, sorteio(15000) contra a reputação
/// (abaixo: saída; senão a federação pensa, 0x69aba0); −1: −750 (1..3500), saída depois de 1460 dias, sorteio(20000)
/// depois de 730; −2: −1250 (1..3000), saída depois de 730 dias, senão sorteio(15000) + 1250; −3: −1750 (1..3000) e a
/// saída. A avaliação vai para a notícia (0x6a5a60) logo depois da mudança. As saídas vão com motivo 6 (2, 0 e −1 com
/// mais tempo) ou 0.
pub(crate) unsafe extern "thiscall" fn avaliar_a_selecao_na_competicao(this: usize, comp: usize, selecao: usize) {
    let tec = read_usize(selecao + 0xcf);
    let objeto = || read_usize(read_usize(0xadadfc).wrapping_add(read_usize(comp).wrapping_mul(4)));
    if tec == 0 || objeto() == 0 {
        return;
    }
    let dias_de: unsafe extern "thiscall" fn(usize, usize, usize) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
    let dias = dias_de(0xae2c90, read_usize(tec + 0x3e), read_usize(tec + 0x42));
    if dias < 0x16d && read_i16(cpq2_linha(this, selecao) + 6) < 0x1964 {
        write_i16(cpq2_linha(this, selecao) + 6, 0x1964); // técnico novo
    }
    let o = objeto();
    let nota_de: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(read_usize(read_usize(o) + 0x74));
    let nota = nota_de(o, selecao) as u8 as i8;
    crate::comparacao::ramo("cpq_2 selecao avaliada");
    let avaliar: unsafe extern "thiscall" fn(usize, usize, usize, i32) = core::mem::transmute(FN_MANAGER_MANAGER_POST_FA_PERFORMANCE_NEWS);
    let c = cpq2_linha(this, selecao) + 6;
    let reputacao = || read_i16(selecao + 0x80);
    match nota {
        3 => {
            cpq2_somar(c, 0x4e2, 0x2134, 0x2710);
            avaliar(this, comp, selecao, 3);
        }
        2 => {
            cpq2_somar(c, 0x2ee, 0x1964, 0x2710);
            avaliar(this, comp, selecao, 2);
            let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
            if humano(selecao, 0, 1, 0, 0) != 0 || dias <= 0xe42 {
                return;
            }
            if (cpq2_sortear(0x3a98) + 0xea6) as i16 >= reputacao() {
                return;
            }
            cpq2_saida(this, selecao, 6);
        }
        0 => {
            write_i16(c, read_i16(c).wrapping_sub(250));
            if read_i16(cpq2_linha(this, selecao) + 0xe) != -3 {
                return;
            }
            write_i16(c, 0xdac);
            avaliar(this, comp, selecao, 0);
            if dias > 0xe42 {
                return cpq2_saida(this, selecao, 6);
            }
            if reputacao() <= 0x1a5e || dias <= 0x5b4 {
                return;
            }
            if cpq2_sortear(0x3a98) >= reputacao() as i32 {
                return cpq2_pensar(this, selecao);
            }
            cpq2_saida(this, selecao, 6);
        }
        -1 => {
            cpq2_somar(c, -750, 1, 0xdac);
            avaliar(this, comp, selecao, -1);
            if dias > 0x5b4 {
                return cpq2_saida(this, selecao, 6);
            }
            if dias <= 0x2da {
                return;
            }
            if cpq2_sortear(0x4e20) >= reputacao() as i32 {
                return cpq2_pensar(this, selecao);
            }
            cpq2_saida(this, selecao, 0);
        }
        -2 => {
            cpq2_somar(c, -1250, 1, 0xbb8);
            avaliar(this, comp, selecao, -2);
            if dias <= 0x2da && cpq2_sortear(0x3a98) + 0x4e2 >= reputacao() as i32 {
                return cpq2_pensar(this, selecao);
            }
            cpq2_saida(this, selecao, 0);
        }
        -3 => {
            cpq2_somar(c, -1750, 1, 0xbb8);
            avaliar(this, comp, selecao, -3);
            cpq2_saida(this, selecao, 0);
        }
        _ => {}
    }
}

// ponytail: solta a notícia da avaliação (0x6a5a60), sorteia e aloca a saída (ou chama a 0x69aba0): só a original;
// a cópia leva os ramos.
pub(crate) unsafe extern "thiscall" fn comparar_avaliar_a_selecao_na_competicao(this: usize, comp: usize, selecao: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(1048)(this, comp, selecao)
}

/// manageable club, not a national team, with board confidence at row +8 under 3000 and a negative balance
/// Original: `MANAGER_MANAGER::is_club_in_financial_trouble` (0x69eb50, manager_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x69eb50 } (thiscall, ret 4, manager_manager.cpp): `MANAGER_MANAGER::is_club_in_financial_trouble(clube)`: clube
/// dirigível (0x544820), fora da seleção (0x53b900), com a confiança da diretoria em +8 da linha abaixo de 3000 e o saldo
/// (64 bits no registro de finanças, 0x5a0590 de [0xae2a38]) negativo: 1. Senão 0. Pelo motor (cantos da torcida).
pub(crate) unsafe extern "thiscall" fn clube_em_apuro_financeiro(this: usize, clube: usize) -> u8 {
    if clube == 0 {
        return 0;
    }
    let dirigivel: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_CLUB_CAN_BE_MANAGED);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    if dirigivel(clube) as u8 == 0 || selecao(clube) != 0 {
        return 0;
    }
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FINANCETABLE_GET_CLUB_FINANCE);
    let f = financas(read_usize(0xae2a38), clube);
    if read_i16(cpq2_linha(this, clube) + 8) >= 0xbb8 || f == 0 {
        return 0;
    }
    (core::ptr::read_unaligned(f as *const i64) < 0) as u8
}

pub(crate) unsafe extern "thiscall" fn comparar_clube_em_apuro_financeiro(this: usize, clube: usize) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize) -> u8>(1049)(this, clube);
    let n = clube_em_apuro_financeiro(this, clube);
    PLACAR.registrar(r == n, || format!("clube_em_apuro_financeiro(clube {:x}) original={} rust={}", clube, r, n));
    r
}

/// the 0..6 band of the board confidence (limits 3000, 3500, 4275, 6500, 8500, 9500)
/// Original: `MANAGER_MANAGER::get_board_confidence_band` (0x690b70, manager_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x690b70 } (thiscall, ret 4, manager_manager.cpp): `MANAGER_MANAGER::get_board_confidence_band(clube)`: a faixa
/// (0..6) da confiança da diretoria (+6 da linha) pelos limites 3000, 3500, 4275, 6500, 8500 e 9500. Pelo texto do
/// relatório da diretoria (0x690d80).
pub(crate) unsafe extern "thiscall" fn faixa_da_confianca_da_diretoria(this: usize, clube: usize) -> u8 {
    let v = read_i16(cpq2_linha(this, clube) + 6);
    [0xbb8i16, 0xdac, 0x10b3, 0x1964, 0x2134, 0x251c].iter().take_while(|&&l| v >= l).count() as u8
}

pub(crate) unsafe extern "thiscall" fn comparar_faixa_da_confianca_da_diretoria(this: usize, clube: usize) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize) -> u8>(1050)(this, clube);
    let n = faixa_da_confianca_da_diretoria(this, clube);
    PLACAR.registrar(r == n, || format!("faixa_da_confianca_da_diretoria(clube {:x}) original={} rust={}", clube, r, n));
    r
}

// ---------- 252a leva: a diretoria e o técnico: a diretoria do técnico novo, o motivo da saída do técnico e o registro do técnico (manager_manager.cpp, staff_records.cpp) ----------




pub(crate) unsafe fn cpp3_por(a: usize, v: u32) {
    core::ptr::write_unaligned(a as *mut u32, v);
}


/// A linha do clube de id `id` na tabela da diretoria do MANAGER_MANAGER ([this] + 0x49 × id; relida a cada uso, como
/// na original): +1 a faixa de riqueza, +2/+3 marcas, +6 confiança, +8/+0xa/+0xc os humores, +0x10..+0x16 as mudanças,
/// +0x28, +0x30/+0x38 doubles, +0x40 marcas.
pub(crate) unsafe fn cpp3_linha(this: usize, id: u32) -> usize {
    (read_u32(this) as usize).wrapping_add((id as usize).wrapping_mul(0x49))
}

/// O humor novo (`mov word`): ftol do float `f32(sorteio1 + q)` menos `sorteio2` (os dois de 0..499), em dupla precisão.
pub(crate) unsafe fn cpp3_humor(q: f32) -> i16 {
    let s1 = sortear(0x1f4);
    let base = (s1 as f64 + q as f64) as f32; // fstp dword
    let s2 = sortear(0x1f4);
    ftol(base as f64 - s2 as f64) as i16
}

/// recalculates the expectations, moves the manager's reputation towards the club's and draws the board confidence and
/// moods from the reputation; clears the changes and flags, records the manager and stores the wealth band; in
/// DESLIGADAS since 2026-10-30: with the detour the career in Rust mode left the reference be54a138 (bisection of the
/// 39 routines of batch 252): the cause (stack residue or a difference in the copy) is not isolated yet; the copy stays
/// in the code and does not run
/// Original: `MANAGER_MANAGER::reset_board_for_new_manager` (0x694c80, manager_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x694c80 } (thiscall, ret 4, manager_manager.cpp): `MANAGER_MANAGER::reset_board_for_new_manager(clube)`: marca o
/// bit 1 de +2 da linha da diretoria, recalcula as expectativas da temporada (0x6a3770 com a notícia) e marca de novo;
/// a lista dos clubes do Shortlist_Manager (+0xdc, 9 bytes por clube, +4) fica com 2. A reputação do técnico (+0xcf,
/// ficha +0x69): na mesma nação (+0x53 = +0x1a do técnico) a atual (+0xa) e a doméstica (+8) andam 10% para a do clube
/// (+0x80); senão a atual anda 10% a partir da de outra nação (+0xc). A base: a reputação atual (zero: a metade da do
/// clube, ou 5000 sem; abaixo de 500: 500 + sorteio de 1000) sobre a do clube (zero: ela mesma; abaixo de 500: 250 +
/// sorteio de 750) × 7500, em float; acima do double de 0x96d1b0 (lido como double: um denormal positivo) vira 10000,
/// abaixo de 6500, 6500. A confiança (+6) e os humores +8/+0xa/+0xc saem da base com dois sorteios de 500 cada (limites
/// 6250..9500, 6250..10000, 5250..10000, 5750..10000, com sorteios nos de baixo); zera as mudanças +0x10..+0x16, os
/// doubles +0x38, +0x28, +4, +5 e marcas (+2 &~0x2e, +3 &0xe0, +0x40 &7). Registra o técnico novo (0x869aa0 com 1). Com o
/// técnico humano (id ≥ [0xae2364] − 16): cancela as ofertas do clube (0x8c4950), refaz a lista do clube (0x84abe0) e
/// as táticas das 50 vagas (0x896c60). +1 = a faixa de riqueza das finanças (0x5986b0), 0 nas seleções ou sem finanças.
pub(crate) unsafe extern "thiscall" fn diretoria_do_tecnico_novo(this: usize, clube: usize) {
    let id = read_u32(clube);
    *(((cpp3_linha(this, id)) + 2) as *mut u8) |= 1;
    let expectativas: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_MANAGER_MANAGER_SET_SEASON_EXPECTATIONS);
    expectativas(this, clube, 1);
    *(((cpp3_linha(this, id)) + 2) as *mut u8) |= 1;
    let lista = read_u32(read_u32(read_u32(0xde1f54) as usize + 0x856) as usize + 0xdc) as usize;
    *(lista.wrapping_add((id as usize).wrapping_mul(9)) as *mut u8).add(4) = 2;
    let k = f64_em(0x967880); // 0,1
    let rep = read_i16(clube + 0x80) as i32;
    let tecnico = read_u32(clube + 0xcf) as usize;
    let ficha = read_u32(tecnico + 0x69) as usize;
    if read_u32(clube + 0x53) == read_u32(tecnico + 0x1a) {
        let a = read_i16(ficha + 0xa) as i32;
        write_i16(ficha + 0xa, ftol((rep - a) as f64 * k + a as f64) as i16);
        let ficha = read_u32(read_u32(clube + 0xcf) as usize + 0x69) as usize;
        let b = read_i16(ficha + 8) as i32;
        write_i16(ficha + 8, ftol((rep - b) as f64 * k + b as f64) as i16);
    } else {
        let c = read_i16(ficha + 0xc) as i32;
        let a = read_i16(ficha + 0xa) as i32;
        write_i16(ficha + 0xa, ftol((rep - c) as f64 * k + a as f64) as i16);
    }
    let ficha = read_u32(read_u32(clube + 0xcf) as usize + 0x69) as usize;
    let v = read_i16(ficha + 0xa) as i32;
    let mut atual = v as f32; // fst dword
    let zero = f32_em(0x968970) as f64;
    if v as f64 == zero {
        let r = read_i16(clube + 0x80) as i32;
        atual = if r != 0 { (r / 2) as f32 } else { 5000.0 };
    } else if (atual as f64) < f32_em(0x96d1a8) as f64 {
        atual = (sortear(0x3e8) + 0x1f4) as f32;
    }
    let d = read_i16(clube + 0x80) as i32;
    let divisor = if d as f64 == zero {
        atual as f64
    } else if (d as f64) < f32_em(0x96d1a8) as f64 {
        (sortear(0x2ee) + 0xfa) as f64
    } else {
        d as f64
    };
    let mut q = ((atual as f64 / divisor) * f32_em(0x96d1a4) as f64) as f32; // fstp dword
    if q as f64 > f64_em(0x96d1b0) {
        crate::comparacao::ramo("cpp_3 base no teto");
        q = 10000.0;
    } else if !(q as f64 >= f32_em(0x96d1ac) as f64) {
        q = 6500.0;
    }
    // a confiança e os humores, cada um com os seus limites
    let h = cpp3_humor(q);
    write_i16(cpp3_linha(this, id) + 6, h);
    let c = read_i16(cpp3_linha(this, id) + 6);
    if c < 0x186a {
        write_i16(cpp3_linha(this, id) + 6, (sortear(0x4e2) + 0x186a) as i16);
    } else if c > 0x251c {
        write_i16(cpp3_linha(this, id) + 6, (0x2710 - sortear(0x1f4)) as i16);
    }
    let s1 = sortear(0x1f4);
    let parcial = read_i16(cpp3_linha(this, id) + 6).wrapping_sub(s1 as i16);
    let s2 = sortear(0x1f4);
    write_i16(cpp3_linha(this, id) + 8, parcial.wrapping_add(s2 as i16));
    let c = read_i16(cpp3_linha(this, id) + 8);
    if c < 0x186a {
        write_i16(cpp3_linha(this, id) + 8, (sortear(0x4e2) + 0x186a) as i16);
    } else if c > 0x2710 {
        write_i16(cpp3_linha(this, id) + 8, 0x2710);
    }
    let h = cpp3_humor(q);
    write_i16(cpp3_linha(this, id) + 0xa, h);
    let c = read_i16(cpp3_linha(this, id) + 0xa);
    if c < 0x1482 {
        write_i16(cpp3_linha(this, id) + 0xa, (sortear(0xfa) + 0x1482) as i16);
    } else if c > 0x2710 {
        write_i16(cpp3_linha(this, id) + 0xa, 0x2710);
    }
    let h = cpp3_humor(q);
    write_i16(cpp3_linha(this, id) + 0xc, h);
    let c = read_i16(cpp3_linha(this, id) + 0xc);
    if c < 0x1676 {
        write_i16(cpp3_linha(this, id) + 0xc, (sortear(0xfa) + 0x1676) as i16);
    } else if c > 0x2710 {
        write_i16(cpp3_linha(this, id) + 0xc, 0x2710);
    }
    let e = cpp3_linha(this, id);
    for o in [0x10, 0x12, 0x14, 0x16] {
        write_i16(e + o, 0); // as mudanças
    }
    let m = |o: usize, mascara: u8| *((cpp3_linha(this, id) + o) as *mut u8) &= mascara;
    m(2, 0xf7);
    m(2, 0xfb);
    m(2, 0xfd);
    cpp3_por(e + 0x38, 0);
    cpp3_por(e + 0x3c, 0);
    *((e + 4) as *mut u8) = 0;
    m(3, 0xe1);
    m(2, 0xdf);
    cpp3_por(e + 0x28, 0);
    *((e + 5) as *mut u8) = 0;
    m(3, 0xfe);
    m(0x40, 7);
    let registrar: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(FN_STAFF_RECORDS_RECORD_MANAGER_JOB_CHANGE);
    registrar(read_u32(0xdd7f00) as usize, read_u32(clube + 0xcf) as usize, clube, 1);
    if read_u32(read_u32(clube + 0xcf) as usize) as i32 >= (read_u32(0xae2364) as i32).wrapping_sub(0x10) {
        let cancelar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_CANCEL_CLUB_OFFERS);
        cancelar(read_u32(0xde1f54) as usize, clube);
        let refazer: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_SHORTLIST_MANAGER_REBUILD_CLUB_SHORTLIST);
        refazer(read_u32(read_u32(0xde1f54) as usize + 0x856) as usize, clube);
        let tatica: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(FN_TACTICS_REMOVE_PLAYER_FROM_SELECTION);
        for k in 0..0x32 {
            let p = read_u32(clube + 0xd7 + 4 * k) as usize;
            if p != 0 {
                tatica(read_u32(0xde1d08) as usize, clube, p, 1, 1);
            }
        }
    }
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let mut faixa = 0u8;
    if selecao(clube) == 0 {
        let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FINANCETABLE_GET_CLUB_FINANCE);
        let fin = financas(read_u32(0xae2a38) as usize, clube);
        if fin != 0 {
            let riqueza: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(FN_FINANCE_GET_WEALTH_BAND);
            faixa = riqueza(fin);
        }
    }
    *((cpp3_linha(this, read_u32(clube)) + 1) as *mut u8) = faixa;
}

pub(crate) unsafe extern "thiscall" fn comparar_diretoria_do_tecnico_novo(this: usize, clube: usize) {
    // ponytail: as expectativas (0x6a3770) postam notícia e o registro (0x869aa0) grava o histórico em arquivo: não
    // rodam duas vezes; só a original, a cópia prova pela carreira no modo Rust (o ramo)
    orig::<unsafe extern "thiscall" fn(usize, usize)>(1012)(this, clube)
}

/// the reason (0 to 11) for the manager's departure: unsettled players, league position, debt, board changes and moods
/// (with draws) and time in the job
/// Original: `MANAGER_MANAGER::get_departure_reason` (0x69b350, manager_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x69b350 } (thiscall, ret 4, manager_manager.cpp): `MANAGER_MANAGER::get_departure_reason(id)`: o motivo (0..11) da
/// saída do técnico do clube `id`. Seleção (0x53b900): 0. Mais de 2 jogadores do elenco com a queixa 0x10000000 no
/// contrato (+0x45): 9. Na temporada (0x547770): na liga com a nação marcada (+0x11c), linha da tabela com mais de 25
/// (+4) e mais de 1 jogo (+0x55), pela posição 0x9c da competição: progresso (+0x73) dentro e a posição projetada
/// (0x4ae300) fora das (+0x3e − +0xc1) vagas: 11; dívida (double +0x30 < +0x38) com a mudança +0x12 abaixo de −300: 10.
/// Depois: +0x12 abaixo de −750 fora de uma sequência do tipo 1 (0x690c30) com sorteio de 3 = 0: 2; na temporada com o
/// bit 8 de +2: 7; +0x10 < −350: 3; +0x14 < −750: 5; humor +0xa abaixo da confiança +6 e sorteio(+0xa) < sorteio(+6):
/// 8; +0x16 < −750 com sorteio de 3 ≠ 0: 4; o técnico há mais dias (0x54cdb0 desde +0x3e) que 1000 + sorteio de 2000:
/// 6. Senão 0.
pub(crate) unsafe extern "thiscall" fn motivo_da_saida_do_tecnico(this: usize, id: u32) -> u8 {
    let clube = (read_u32(0xae23b4) as usize).wrapping_add((id as usize).wrapping_mul(0x245));
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    if selecao(clube) != 0 {
        return 0;
    }
    let em_temporada: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_CLUB_IS_IN_SEASON);
    crate::comparacao::ramo("cpp_3 motivo da saida");
    let temporada = em_temporada(clube, 0xae2c90);
    let (mut tipo, mut tamanho) = (0u8, 0u8);
    let sequencia: unsafe extern "thiscall" fn(usize, usize, *mut u8, *mut u8) = core::mem::transmute(FN_MANAGER_MANAGER_GET_CURRENT_RUN);
    sequencia(this, clube, &mut tipo, &mut tamanho);
    let contrato: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let mut queixosos = 0u8;
    for k in 0..0x32 {
        let p = read_u32(clube + 0xd7 + 4 * k) as usize;
        if p != 0 && read_u32(p + 0x61) != 0 {
            let c = contrato(read_u32(0xae1050) as usize, p, clube);
            if c != 0 && read_u32(c + 0x45) & 0x1000_0000 != 0 {
                queixosos = queixosos.wrapping_add(1);
            }
        }
    }
    if queixosos as i8 > 2 {
        return 9;
    }
    if temporada != 0 {
        let liga = read_u32(clube + 0x57) as usize;
        let nacao = read_u32(clube + 0x53) as usize;
        if liga != 0 && nacao != 0 && read_u8(nacao + 0x11c) != 0 {
            let comp = read_u32((read_u32(0xadadfc) as usize).wrapping_add((read_u32(liga) as usize).wrapping_mul(4))) as usize;
            if comp != 0 {
                let busca: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_COMP_STATS_FIND_CLUB_ENTRY);
                let linha = busca(read_u32(comp + 8) as usize, clube);
                let (mut pa, mut pb, mut pc) = (0u8, 0u8, 0u8); // a 0x4ae300 grava as três (com a linha achada)
                let projetar: unsafe extern "thiscall" fn(usize, usize, *mut u8, *mut u8, *mut u8) -> u32 = core::mem::transmute(FN_COMP_STATS_GET_CLUB_PROJECTED_POSITIONS);
                let comp = read_u32((read_u32(0xadadfc) as usize).wrapping_add((read_u32(read_u32(clube + 0x57) as usize) as usize).wrapping_mul(4))) as usize;
                projetar(read_u32(comp + 8) as usize, clube, &mut pa, &mut pb, &mut pc);
                if read_u8(linha + 4) > 0x19 && read_u32(linha + 0x55) as i32 > 1 {
                    let comp = read_u32((read_u32(0xadadfc) as usize).wrapping_add((read_u32(read_u32(clube + 0x57) as usize) as usize).wrapping_mul(4))) as usize;
                    let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_u32(read_u32(comp) as usize + 0x9c) as usize);
                    if f(comp) != 0 {
                        let comp = read_u32((read_u32(0xadadfc) as usize).wrapping_add((read_u32(read_u32(clube + 0x57) as usize) as usize).wrapping_mul(4))) as usize;
                        let vagas = read_i16(comp + 0x3e) as i32 - read_u8(comp + 0xc1) as i8 as i32;
                        if read_u8(linha + 0x73) as i32 >= vagas && (pa as i32) < vagas {
                            return 0xb;
                        }
                    }
                }
            }
        }
        let e = cpp3_linha(this, id);
        let (gasto, teto) = (core::ptr::read_unaligned((e + 0x30) as *const f64), core::ptr::read_unaligned((e + 0x38) as *const f64));
        if !(gasto >= teto) && read_i16(e + 0x12) < -300 {
            return 0xa;
        }
    }
    let e = cpp3_linha(this, id);
    if read_i16(e + 0x12) < -750 && tipo != 1 && sortear(3) == 0 {
        crate::comparacao::ramo("cpp_3 motivo 2");
        return 2;
    }
    if temporada != 0 && read_u8(e + 2) & 8 != 0 {
        return 7;
    }
    if read_i16(e + 0x10) < -350 {
        return 3;
    }
    if read_i16(e + 0x14) < -750 {
        return 5;
    }
    let humor = read_i16(e + 0xa);
    if humor < read_i16(e + 6) {
        let a = sortear(read_i16(cpp3_linha(this, id) + 6) as i32);
        let b = sortear(humor as i32);
        if b < a {
            return 8;
        }
    }
    if read_i16(cpp3_linha(this, id) + 0x16) < -750 && sortear(3) != 0 {
        return 4;
    }
    let tecnico = read_u32(clube + 0xcf) as usize;
    if tecnico != 0 {
        let dias: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
        let d = dias(0xae2c90, read_u32(tecnico + 0x3e), read_u32(tecnico + 0x42));
        if d > sortear(0x7d0) + 0x3e8 {
            return 6;
        }
    }
    0
}

/// Comparador: lado a lado a partir do mesmo estado do sorteador (a rotina sorteia em vários caminhos), pela devolução;
/// a cópia sem contar os sorteios.
pub(crate) unsafe extern "thiscall" fn comparar_motivo_da_saida_do_tecnico(this: usize, id: u32) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32) -> u8>(1013);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&crate::aleatorio::REGIOES_DO_ESTADO, || o(this, id),
        || crate::aleatorio::sem_contar(|| motivo_da_saida_do_tecnico(this, id)));
    PLACAR.registrar(igual && r == n, || format!("motivo_da_saida_do_tecnico(clube {}) original={} rust={} {:x?} x {:x?}", id, r, n, mo, mr));
    r
}

/// O registro do técnico (210 bytes) que a 0x86f5b0 olha: humano (id em [0xae2364] − 16 .. [0xae2364]) pela vaga dele
/// depois dos [0xae2368] registros; senão pelo id da ficha de não jogador (+0x69). None: sem ficha, ou humano sem
/// situação (+0x5f 0 ou 5).
pub(crate) unsafe fn cpp3_registro(this: usize, pessoa: usize) -> Option<usize> {
    let n = read_u32(0xae2364) as i32;
    let id = read_u32(pessoa) as i32;
    let k = if id >= n.wrapping_sub(0x10) && id < n {
        let s = read_u8(pessoa + 0x5f);
        if s == 0 || s == 5 {
            return None;
        }
        id.wrapping_sub(n).wrapping_add(read_u32(0xae2368) as i32).wrapping_add(0x10)
    } else {
        let ficha = read_u32(pessoa + 0x69) as usize;
        if ficha == 0 {
            return None;
        }
        read_u32(ficha) as i32
    };
    Some((read_u32(this + 8) as usize).wrapping_add((k as usize).wrapping_mul(0xd2)))
}

pub(crate) unsafe fn cpp3_data(destino: usize, origem: usize) {
    let copiar: unsafe extern "thiscall" fn(usize, *mut [u8; 8], usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let mut ret = [0u8; 8];
    copiar(destino, &mut ret, origem);
}

/// finds (or creates, or takes over) the manager's 210-byte record and updates the nations, the current spell and the
/// longest and shortest spells
/// Original: `Staff_Records::get_manager_record` (0x86f5b0, staff_records.cpp, thiscall, ret 8).
///
/// [pt] { 0x86f5b0 } (thiscall, ret 8, staff_records.cpp): `Staff_Records::get_manager_record(pessoa, criar)`: o registro de
/// 210 bytes do técnico (vetor +8). Pessoa nula: caixa da linha 0xf6e e 0. Vaga livre (+0x86 = −1): sem `criar`, 0;
/// com ele, a vaga fica com o id. Vaga de outro: zera o registro da ficha dele (−1 em +0x86, +0xc2, +0xc6, +0x4a, +0x5e,
/// +0x72) e olha de novo. Atualiza as nações do clube (+0x39) e da seleção (+0x24) em +0xc2/+0xc6, zerando os pontos
/// (+0xba/+0xbe) quando mudam. Com a data de [0xdd6ec8] até hoje: sem passagem (+0x4a = −1), abre a do clube atual
/// (desde +0x3e da pessoa; +0x56 = a data 13/11/1975 da 0x549f70); a mais longa (+0x5e, +0x62..+0x6a) e a mais curta
/// (+0x72, +0x76..+0x7e) pela 0x54cdb0, a de hoje em +0x6a / +0x7e. Devolve o registro.
pub(crate) unsafe extern "thiscall" fn registro_do_tecnico(this: usize, pessoa: usize, criar: u32) -> usize {
    if pessoa == 0 {
        let separar: unsafe extern "C" fn(usize, usize, usize, *mut u8, *mut u8) = core::mem::transmute(FN_CRT_STRTOK);
        let versao: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
        let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
        let caixa: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(FN_GUI_SHOW_MESSAGE_BOX);
        let (mut nome, mut ext) = ([0u8; 256], [0u8; 256]);
        separar(0xa83b78, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
        let v = versao(nome.as_mut_ptr(), ext.as_mut_ptr(), 0xf6e);
        formatar(0xae24d0, 0x9870e8, v, nome.as_ptr(), ext.as_ptr(), 0xf6eu32);
        caixa(0x9870e0, 0xae24d0, 0);
        cpp3_por(0xb67a34, 0);
        return 0;
    }
    let r = loop {
        let Some(r) = cpp3_registro(this, pessoa) else {
            return 0;
        };
        let dono = read_u32(r + 0x86) as i32;
        if dono == -1 {
            if criar == 0 {
                return 0;
            }
            crate::comparacao::ramo("cpp_3 criar registro");
            cpp3_por(r + 0x86, read_u32(pessoa)); // a vaga fica com ele
            break r;
        }
        if dono == read_u32(pessoa) as i32 {
            break r;
        }
        // a vaga é de outro: zera o registro da ficha dele e olha de novo
        let z = (read_u32(this + 8) as usize).wrapping_add((read_u32(read_u32(pessoa + 0x69) as usize) as usize).wrapping_mul(0xd2));
        core::ptr::write_bytes(z as *mut u8, 0, 0xd2);
        for o in [0x86, 0xc2, 0xc6, 0x4a, 0x5e, 0x72] {
            cpp3_por(z + o, u32::MAX);
        }
    };
    let clube = read_u32(pessoa + 0x39) as usize;
    if clube != 0 {
        let nacao = read_u32(clube + 0x53) as usize;
        if nacao != 0 && read_u32(nacao) != read_u32(r + 0xc2) {
            cpp3_por(r + 0xc2, read_u32(nacao));
            cpp3_por(r + 0xba, 0);
        }
    }
    let selecao = read_u32(pessoa + 0x24) as usize;
    if selecao != 0 {
        let nacao = read_u32(selecao + 0x53) as usize;
        if nacao != 0 && read_u32(nacao) != read_u32(r + 0xc6) {
            cpp3_por(r + 0xc6, read_u32(nacao));
            cpp3_por(r + 0xbe, 0);
        }
    }
    let (dia, ano) = (read_i16(0xdd6ec8), core::ptr::read_unaligned(0xdd6eca as *const u16));
    let hoje_ano = core::ptr::read_unaligned(0xae2c92 as *const u16);
    if ano > hoje_ano || (ano == hoje_ano && dia > read_i16(0xae2c90)) {
        return r; // a data de [0xdd6ec8] ainda não chegou
    }
    if read_u32(r + 0x4a) as i32 == -1 {
        let clube = read_u32(pessoa + 0x39) as usize;
        if clube != 0 {
            crate::comparacao::ramo("cpp_3 abrir passagem");
            cpp3_por(r + 0x4a, read_u32(clube));
            cpp3_data(r + 0x4e, pessoa + 0x3e);
            let mut local = [0u8; 8];
            let construir: unsafe extern "thiscall" fn(*mut [u8; 8], u32, u32, u32, i32) -> usize = core::mem::transmute(FN_DATE_DATE_549F70);
            let p = construir(&mut local, 0xd, 0xb, 0x7b7, -1);
            cpp3_data(r + 0x56, p);
        }
    }
    let dias: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
    if read_u32(r + 0x5e) as i32 == -1 {
        cpp3_por(r + 0x5e, read_u32(r + 0x4a));
        cpp3_data(r + 0x62, r + 0x4e);
        cpp3_data(r + 0x6a, 0xae2c90);
    } else if read_u32(r + 0x4a) as i32 != -1 {
        let a = dias(r + 0x6a, read_u32(r + 0x62), read_u32(r + 0x66));
        let b = dias(0xae2c90, read_u32(r + 0x4e), read_u32(r + 0x52));
        if b > a {
            cpp3_por(r + 0x5e, read_u32(r + 0x4a)); // a mais longa
            cpp3_data(r + 0x62, r + 0x4e);
            cpp3_data(r + 0x6a, 0xae2c90);
        }
    }
    if read_u32(r + 0x72) as i32 == -1 {
        cpp3_por(r + 0x72, read_u32(r + 0x4a));
        cpp3_data(r + 0x76, r + 0x4e);
        cpp3_data(r + 0x7e, 0xae2c90);
        return r;
    }
    let atual = read_u32(r + 0x4a);
    if read_u32(r + 0x72) == atual && read_i16(r + 0x50) == read_i16(r + 0x78) && read_i16(r + 0x4e) == read_i16(r + 0x76) {
        cpp3_data(r + 0x7e, 0xae2c90); // a mais curta é a atual: vai até hoje
        return r;
    }
    if atual as i32 == -1 {
        return r;
    }
    let a = dias(r + 0x7e, read_u32(r + 0x76), read_u32(r + 0x7a));
    let b = dias(0xae2c90, read_u32(r + 0x4e), read_u32(r + 0x52));
    if b < a {
        cpp3_por(r + 0x72, read_u32(r + 0x4a)); // a mais curta
        cpp3_data(r + 0x76, r + 0x4e);
        cpp3_data(r + 0x7e, 0xae2c90);
    }
    r
}

/// Comparador: lado a lado pelo registro do técnico (e, quando a vaga é de outro, pelo registro que a original zera),
/// pela devolução. Pessoa nula (a caixa) e o humano sem situação: só a original.
pub(crate) unsafe extern "thiscall" fn comparar_registro_do_tecnico(this: usize, pessoa: usize, criar: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(1014);
    if pessoa == 0 {
        return o(this, pessoa, criar);
    }
    let Some(r) = cpp3_registro(this, pessoa) else {
        return o(this, pessoa, criar);
    };
    let mut reg = vec![(r, 0xd2)];
    let dono = read_u32(r + 0x86) as i32;
    if dono != -1 && dono != read_u32(pessoa) as i32 {
        let z = (read_u32(this + 8) as usize).wrapping_add((read_u32(read_u32(pessoa + 0x69) as usize) as usize).wrapping_mul(0xd2));
        if z != r {
            reg.push((z, 0xd2));
        }
    }
    let (a, b, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, pessoa, criar), || registro_do_tecnico(this, pessoa, criar));
    PLACAR.registrar(igual && a == b, || format!("registro_do_tecnico(pessoa {:x}, criar {}) original={:x} rust={:x} {:x?} x {:x?}", pessoa, criar, a, b, mo, mr));
    a
}

// ---------- 251a leva: a confiança da diretoria: o negócio da transferência, a confiança depois da transferência e o ultimato (manager_manager.cpp) ----------


pub(crate) unsafe fn cpo1_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}


pub(crate) unsafe fn cpo1_por(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v);
}

pub(crate) unsafe fn cpo1_ou(a: usize, v: u8) {
    *(a as *mut u8) |= v;
}

/// Soma `v` à palavra de 16 bits (com a volta do `add word`).
pub(crate) unsafe fn cpo1_somar(a: usize, v: i16) {
    core::ptr::write_unaligned(a as *mut i16, read_i16(a).wrapping_add(v));
}


pub(crate) unsafe fn cpo1_f_por(a: usize, v: f64) {
    core::ptr::write_unaligned(a as *mut f64, v);
}

/// A constante `double` do próprio exe.
pub(crate) unsafe fn cpo1_k(a: usize) -> f64 {
    read_f64(a)
}

/// A linha do clube na tabela da diretoria do MANAGER_MANAGER ([this] + 0x49 × id): +2 marcas, +6 confiança (1..10000),
/// +0x14 e +0x16 os humores da diretoria, +0x38 o gasto líquido (double).
pub(crate) unsafe fn cpo1_linha(this: usize, id: usize) -> usize {
    read_usize(this).wrapping_add(id.wrapping_mul(0x49))
}

/// O valor do jogador para o clube (CONTRACT_MANAGER 0x4d0c60 em [0xae1050], pode sortear) como a 0x696570 o pede:
/// com o clube do jogador (+0x39) quando ele tem; senão o `clube`, com o último argumento 1 se o `clube` é de humano
/// (0x600a50).
pub(crate) unsafe fn cpo1_valor(pessoa: usize, clube: usize) -> i32 {
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> i32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let valor: unsafe extern "thiscall" fn(usize, usize, usize, u32, i32, u32) -> i32 = core::mem::transmute(FN_CONTRACT_MANAGER_CALC_PLAYER_VALUE);
    let h = humano(clube, 0, 1, 0, 0);
    let atual = read_usize(pessoa + 0x39);
    let (alvo, ultimo) = if h != 0 {
        (if atual != 0 { atual } else { clube }, 1)
    } else if atual != 0 {
        (atual, 1)
    } else {
        (clube, 0)
    };
    valor(read_usize(0xae1050), pessoa, alvo, 0, -1, ultimo)
}

/// O limite da taxa pela reputação (+0x80) do clube: (reputação × 0,2)² truncado (_ftol 0x945640).
pub(crate) unsafe fn cpo1_limite(clube: usize) -> i32 {
    let x = read_i16(clube + 0x80) as i32 as f64 * cpo1_k(0x968918);
    ftol(x * x)
}

/// flags the deal for buyer and seller by the fee/player value ratio (bands for values above or up to 250,000, a fee up
/// to (reputation × 0.2)² is fair), adds and subtracts the value from each one's net spend and flags the club's and
/// chairman's favoured or disliked player
/// Original: `MANAGER_MANAGER::assess_transfer_deal` (0x696570, manager_manager.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x696570 } (thiscall, ret 0xc, manager_manager.cpp): `MANAGER_MANAGER::assess_transfer_deal(transf, f1, f2)`:
/// com o jogador (+0xc) que tem ficha (+0x61), zera as duas palavras de marcas e: pelo clube comprador (+8, registro de
/// 0x245 bytes) soma o valor do jogador (0x4d0c60, 1 se 0) ao gasto líquido (+0x38) e marca `f1` pela razão taxa
/// (0x8e7a70) / valor: taxa até o limite (reputação × 0,2)² = 4; valor acima de 250.000: < 0,2 = 0x10, < 0,5 = 8, < 2 =
/// 4, < 6 = 2, senão 1; até 250.000: < 0,125 = 8, < 8 = 4, senão 2; o jogador favorito ou rejeitado do clube ou do
/// presidente (+0xbf; 0x547790/0x5477d0/0x547810/0x547840) = 0x40 ou 0x100. Pelo clube vendedor (+0x39 do jogador)
/// tira o valor do gasto e marca `f2` com as faixas espelhadas (0,25/0,5/2/4; 0,125/8) e 0x20 ou 0x80. Transferência
/// já avaliada (+0x1f = 1): nada (as marcas ficam como estão).
pub(crate) unsafe extern "thiscall" fn avaliar_o_negocio_da_transferencia(this: usize, transf: usize, f1: usize, f2: usize) {
    if *((transf + 0x1f) as *const u8) == 1 {
        return;
    }
    core::ptr::write_unaligned(f2 as *mut u16, 0);
    core::ptr::write_unaligned(f1 as *mut u16, 0);
    let pessoa = read_usize(0xae23bc).wrapping_add(read_usize(transf + 0xc).wrapping_mul(0x6e));
    if read_usize(pessoa + 0x61) == 0 {
        return;
    }
    let comprador = read_usize(0xae23b4).wrapping_add(read_usize(transf + 8).wrapping_mul(0x245));
    let mut valor = cpo1_valor(pessoa, comprador);
    if valor == 0 {
        valor = 1;
    }
    let oferta: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> i32 = core::mem::transmute(FN_TRANSFER_OFFER_GET_TOTAL_VALUE);
    let taxa = oferta(transf, 0, 0, 0, 1);
    let favorito: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_CLUB_FAVOURS_STAFF);
    let rejeitado: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_CLUB_DISLIKES_STAFF);
    let favorito_do_presidente: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_FAVOURED_BY);
    let rejeitado_do_presidente: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_DISLIKED_BY);
    if comprador != 0 {
        let g = cpo1_linha(this, read_usize(comprador)) + 0x38;
        cpo1_f_por(g, valor as f64 + read_f64(g));
        let limite = cpo1_limite(comprador);
        let razao = if valor == 0 { *(0x968f18 as *const f32) as f64 } else { taxa as f64 / valor as f64 };
        if taxa <= limite {
            crate::comparacao::ramo("cpo_1 comprador no limite");
            cpo1_ou(f1, 4); // o comprador: taxa dentro do limite
        } else if valor > 0x3d090 {
            let m = if razao < cpo1_k(0x968918) {
                0x10
            } else if razao < cpo1_k(0x968968) {
                8
            } else if razao < cpo1_k(0x968950) {
                4
            } else if razao < cpo1_k(0x969000) {
                2
            } else {
                1
            };
            cpo1_ou(f1, m);
        } else {
            let m = if razao < cpo1_k(0x968fd0) { 8 } else if razao < cpo1_k(0x968f10) { 4 } else { 2 };
            cpo1_ou(f1, m);
        }
        if favorito(pessoa, comprador) != 0 {
            cpo1_ou(f1, 0x40);
        } else if rejeitado(pessoa, comprador) != 0 {
            cpo1_ou(f1 + 1, 1);
        }
        if favorito_do_presidente(pessoa, read_usize(comprador + 0xbf)) != 0 {
            cpo1_ou(f1, 0x40);
        } else if rejeitado_do_presidente(pessoa, read_usize(comprador + 0xbf)) != 0 {
            cpo1_ou(f1 + 1, 1);
        }
    }
    let vendedor = read_usize(pessoa + 0x39);
    if vendedor == 0 {
        return;
    }
    let v2 = cpo1_valor(pessoa, vendedor); // aqui sem o mínimo 1
    let g = cpo1_linha(this, read_usize(vendedor)) + 0x38;
    cpo1_f_por(g, read_f64(g) - v2 as f64);
    let limite = cpo1_limite(vendedor);
    let razao = if v2 == 0 { *(0x968f18 as *const f32) as f64 } else { taxa as f64 / v2 as f64 };
    if taxa <= limite {
        crate::comparacao::ramo("cpo_1 vendedor no limite");
        cpo1_ou(f2, 4); // o vendedor: taxa dentro do limite
    } else if v2 > 0x3d090 {
        let m = if razao < cpo1_k(0x9689e0) {
            1
        } else if razao < cpo1_k(0x968968) {
            2
        } else if razao < cpo1_k(0x968950) {
            4
        } else if razao < cpo1_k(0x968938) {
            8
        } else {
            0x10
        };
        cpo1_ou(f2, m);
    } else {
        let m = if razao < cpo1_k(0x968fd0) { 2 } else if razao < cpo1_k(0x968f10) { 4 } else { 8 };
        cpo1_ou(f2, m);
    }
    if favorito(pessoa, vendedor) != 0 {
        cpo1_ou(f2, 0x20);
        return;
    }
    if rejeitado(pessoa, vendedor) != 0 {
        cpo1_ou(f2, 0x80);
        return;
    }
    if favorito_do_presidente(pessoa, read_usize(vendedor + 0xbf)) != 0 {
        cpo1_ou(f2, 0x20);
        return;
    }
    if rejeitado_do_presidente(pessoa, read_usize(vendedor + 0xbf)) != 0 {
        cpo1_ou(f2, 0x80);
    }
}

/// As faixas de confiança (+6, 1..10000) e o limite de baixo do humor +0x16 (−10000).
pub(crate) unsafe fn cpo1_limitar(e: usize) {
    if read_i16(e + 6) < 1 {
        core::ptr::write_unaligned((e + 6) as *mut i16, 1);
    }
    if read_i16(e + 6) > 10000 {
        core::ptr::write_unaligned((e + 6) as *mut i16, 10000);
    }
    if read_i16(e + 0x16) <= -10000 {
        core::ptr::write_unaligned((e + 0x16) as *mut i16, -10000);
    }
}

/// A parte da razão nas marcas (bits 1..0x10, o primeiro que vale): a confiança (+6) e o humor +0x14.
pub(crate) unsafe fn cpo1_pela_razao(e: usize, f: u32) {
    if f & 1 != 0 {
        cpo1_somar(e + 6, -0x7d);
        cpo1_somar(e + 0x14, -500);
    } else if f & 2 != 0 {
        cpo1_somar(e + 6, -0x19);
        cpo1_somar(e + 0x14, -250);
    } else if f & 4 != 0 {
        // o negócio justo: nada
    } else if f & 8 != 0 {
        cpo1_somar(e + 6, 0x19);
        cpo1_somar(e + 0x14, 0x19);
    } else if f & 0x10 != 0 {
        cpo1_somar(e + 6, 0x4b);
        cpo1_somar(e + 0x14, 0x4b);
    }
}

/// changes the board confidence (+6, 1..10000) and moods (+0x14, +0x16) of the buyer and the seller from the deal flags
/// and removes the player from the seller chairman's favoured people
/// Original: `MANAGER_MANAGER::apply_transfer_confidence_changes` (0x6978f0, manager_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x6978f0 } (thiscall, ret 4, manager_manager.cpp): `MANAGER_MANAGER::apply_transfer_confidence_changes(transf)`:
/// avalia o negócio (0x696570) com as marcas do comprador na vaga do próprio argumento e as do vendedor na vaga do
/// `ecx` empurrado (avaliado antes, +0x1f = 1: ficam os bits do ponteiro, como na original) e muda a linha da diretoria
/// do comprador (+8) e, se o vendedor (+0x39 do jogador) não é seleção (0x53b900) nem tem as finanças com +0x165 = 1
/// (0x5a0590), a dele: razão 1: −125/−500, 2: −25/−250, 8: +25/+25, 0x10: +75/+75 (confiança +6 e humor +0x14);
/// favorito e rejeitado no humor +0x16 (e +0x14). Limita a confiança a 1..10000 e +0x16 a −10000 ou mais; no fim o
/// jogador sai das pessoas favoritas do presidente do vendedor (0x547c10).
pub(crate) unsafe extern "thiscall" fn confianca_da_diretoria_na_transferencia(this: usize, transf: usize) {
    let pessoa = read_usize(0xae23bc).wrapping_add(read_usize(transf + 0xc).wrapping_mul(0x6e));
    let mut fc: usize = transf; // a vaga do argumento
    let mut fv: usize = this; // o ecx empurrado no prólogo
    let avaliar: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(FN_MANAGER_MANAGER_ASSESS_TRANSFER_DEAL);
    avaliar(this, transf, &mut fc as *mut usize as usize, &mut fv as *mut usize as usize);
    let f = fc as u32;
    let e = cpo1_linha(this, read_usize(transf + 8));
    cpo1_pela_razao(e, f);
    if f & 0x20 != 0 {
        cpo1_somar(e + 0x14, -100);
        cpo1_somar(e + 0x16, -100);
    } else if f & 0x40 != 0 {
        cpo1_somar(e + 0x16, 0x32);
    } else if f & 0x80 != 0 {
        cpo1_somar(e + 0x16, 0xa);
    } else if f & 0x100 != 0 {
        cpo1_somar(e + 0x16, -100);
        cpo1_somar(e + 0x14, -0x32);
    }
    cpo1_limitar(e);
    let vendedor = read_usize(pessoa + 0x39);
    if vendedor == 0 {
        return;
    }
    let id = read_usize(vendedor);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    if selecao(vendedor) != 0 {
        return;
    }
    let financas_do_clube: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FINANCETABLE_GET_CLUB_FINANCE);
    let fin = financas_do_clube(read_usize(0xae2a38), read_usize(pessoa + 0x39));
    if fin != 0 && *((fin + 0x165) as *const u8) == 1 {
        return;
    }
    let f = fv as u32;
    let e = cpo1_linha(this, id);
    cpo1_pela_razao(e, f);
    if f & 0x20 != 0 {
        cpo1_somar(e + 0x16, -100);
        cpo1_somar(e + 0x14, -0x19);
    } else if f & 0x40 != 0 {
        cpo1_somar(e + 0x16, 0x32);
        cpo1_somar(e + 0x14, 0x19);
    } else if f & 0x80 != 0 {
        cpo1_somar(e + 0x16, 0xa);
    } else if f & 0x100 != 0 {
        cpo1_somar(e + 0x16, -100);
    }
    cpo1_limitar(e);
    let tirar: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(FN_STAFF_REMOVE_FAVOURED_STAFF);
    crate::comparacao::ramo("cpo_1 tirar favorito");
    tirar(read_usize(read_usize(pessoa + 0x39) + 0xbf), pessoa); // o presidente do vendedor
}

/// removes the club from the ultimatum list and, when the manager has been in the job over 100 days, queues his
/// departure (reason 2, or 7 without games in the table) and returns 1
/// Original: `MANAGER_MANAGER::resolve_board_ultimatum` (0x6ac470, manager_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x6ac470 } (thiscall, ret 4, manager_manager.cpp): `MANAGER_MANAGER::resolve_board_ultimatum(clube)`: percorre a
/// lista dos ultimatos (+0x10, até a conta, que cai a cada retirada) e tira o item do clube (0x68d1f0); se o técnico
/// (+0xcf) está no cargo há mais de 100 dias (0x54cdb0 desde +0x3e), põe a saída na lista (+4, 0x68d0d0, 6 bytes: 1, o
/// motivo 2, ou 7 se a linha do clube na tabela da divisão (0x4a4850) não tem jogos (+0x55), e o id do clube), marca
/// o bit 2 em +2 da linha da diretoria e devolve 1. Senão 0.
pub(crate) unsafe extern "thiscall" fn resolver_o_ultimato_da_diretoria(this: usize, clube: usize) -> u8 {
    let lista = read_usize(this + 0x10);
    if read_i16(lista) == 0 {
        return 0;
    }
    cpo1_por(lista + 0xa, read_usize(lista + 2));
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let remover: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_LLIST_REMOVE);
    let dias: unsafe extern "thiscall" fn(usize, usize, usize) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
    let mut k = 0i32;
    while k < read_i16(read_usize(this + 0x10)) as i32 {
        let item = proximo(read_usize(this + 0x10));
        if item != 0 && read_usize(clube) == read_usize(item) {
            remover(read_usize(this + 0x10), item);
            let tecnico = read_usize(clube + 0xcf);
            if tecnico != 0 && dias(0xae2c90, read_usize(tecnico + 0x3e), read_usize(tecnico + 0x42)) > 0x64 {
                crate::comparacao::ramo("cpo_1 ultimato cumprido");
                let mut reg = [0u8; 8];
                reg[0] = 1;
                reg[2..6].copy_from_slice(&(read_usize(clube) as u32).to_le_bytes());
                let mut motivo = 2u8;
                let divisao = read_usize(clube + 0x57);
                if divisao != 0 {
                    let comp = read_usize(read_usize(0xadadfc).wrapping_add(read_usize(divisao).wrapping_mul(4)));
                    if comp != 0 && read_usize(comp + 8) != 0 {
                        let busca: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_COMP_STATS_FIND_CLUB_ENTRY);
                        let linha = busca(read_usize(comp + 8), clube);
                        if linha == 0 || cpo1_i(linha + 0x55) <= 0 {
                            motivo = 7;
                        }
                    }
                }
                reg[1] = motivo;
                let acrescentar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_LLIST_ADD_TAIL);
                acrescentar(read_usize(this + 4), reg.as_mut_ptr() as usize, 6);
                cpo1_ou(cpo1_linha(this, read_usize(clube)) + 2, 2);
                return 1;
            }
        }
        k += 1;
    }
    0
}

/// Comparador: a avaliação pede o valor do jogador (0x4d0c60, que sorteia): lado a lado com o estado do sorteador, as
/// duas palavras de marcas e os gastos líquidos do comprador e do vendedor; a cópia sem contar os sorteios.
pub(crate) unsafe extern "thiscall" fn comparar_avaliar_o_negocio_da_transferencia(this: usize, transf: usize, f1: usize, f2: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(967);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(f1, 2), (f2, 2)]);
    let pessoa = read_usize(0xae23bc).wrapping_add(read_usize(transf + 0xc).wrapping_mul(0x6e));
    let comprador = read_usize(0xae23b4).wrapping_add(read_usize(transf + 8).wrapping_mul(0x245));
    reg.push((cpo1_linha(this, read_usize(comprador)) + 0x38, 8));
    let vendedor = read_usize(pessoa + 0x39);
    if vendedor != 0 {
        reg.push((cpo1_linha(this, read_usize(vendedor)) + 0x38, 8));
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, transf, f1, f2),
        || crate::aleatorio::sem_contar(|| avaliar_o_negocio_da_transferencia(this, transf, f1, f2)));
    PLACAR.registrar(igual, || format!("avaliar_o_negocio_da_transferencia({:x}) {:x?} x {:x?}", transf, mo, mr));
}

/// Comparador: lado a lado com o estado do sorteador (a avaliação 0x696570 sorteia), as linhas da diretoria do
/// comprador e do vendedor e as três pessoas favoritas do presidente do vendedor (0x547c10); a cópia sem contar.
pub(crate) unsafe extern "thiscall" fn comparar_confianca_da_diretoria_na_transferencia(this: usize, transf: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(968);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    let pessoa = read_usize(0xae23bc).wrapping_add(read_usize(transf + 0xc).wrapping_mul(0x6e));
    reg.push((cpo1_linha(this, read_usize(transf + 8)), 0x49));
    let vendedor = read_usize(pessoa + 0x39);
    if vendedor != 0 {
        reg.push((cpo1_linha(this, read_usize(vendedor)), 0x49));
        let pref = dbp_preferencias(read_usize(vendedor + 0xbf));
        if pref != 0 {
            reg.push((pref + 0x1c, 12));
        }
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, transf),
        || crate::aleatorio::sem_contar(|| confianca_da_diretoria_na_transferencia(this, transf)));
    PLACAR.registrar(igual, || format!("confianca_da_diretoria_na_transferencia({:x}) {:x?} x {:x?}", transf, mo, mr));
}

/// Comparador: a original tira o item da lista (e o libera) e acrescenta a saída: roda só ela e confere o plano: a
/// lista das saídas (+4) cresce de 1 quando devolve 1 (e a linha da diretoria fica com o bit 2) e fica igual com 0.
pub(crate) unsafe extern "thiscall" fn comparar_resolver_o_ultimato_da_diretoria(this: usize, clube: usize) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u8>(969);
    let saidas = read_usize(this + 4);
    if saidas == 0 || read_usize(this + 0x10) == 0 {
        return o(this, clube);
    }
    let antes = read_i16(saidas) as i32;
    let r = o(this, clube);
    let depois = read_i16(read_usize(this + 4)) as i32;
    let ok = depois == antes + (r != 0) as i32 && (r == 0 || *((cpo1_linha(this, read_usize(clube)) + 2) as *const u8) & 2 != 0);
    PLACAR.registrar(ok, || format!("resolver_o_ultimato_da_diretoria(clube {:x}) devolveu {}, saídas {} -> {}", clube, r, antes, depois));
    r
}

// ---------- 251a leva: as saídas dos clubes: as saídas pendentes dos técnicos, tirar o jogador do elenco e tirar a pessoa do cargo (manager_manager.cpp) ----------


pub(crate) unsafe fn cpo4_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}



pub(crate) unsafe fn cpo4_por(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v);
}

/// O registro do clube pelo id (0x245 bytes em [0xae23b4]).
pub(crate) unsafe fn cpo4_clube(id: usize) -> usize {
    read_usize(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// walks the pending departure list and, at each club with a manager, carries out his departure and, when the club
/// opens a vacancy, the vacancy; removes each item from the list
/// Original: `MANAGER_MANAGER::process_pending_departures` (0x69b650, manager_manager.cpp, thiscall, ret 0).
///
/// [pt] { 0x69b650 } (thiscall, ret 0, manager_manager.cpp): `MANAGER_MANAGER::process_pending_departures()`: percorre a lista
/// das saídas pendentes (+4, itens de 6 bytes: dois bytes e o id do clube em +2) até a conta (que cai a cada retirada:
/// a original passa só por parte dos itens) e, para cada item, com técnico no clube (+0xcf) chama a saída (0x69b790
/// com os dois bytes empurrados nos registradores inteiros: os bits de cima são os do id e de 145 × id) e, se o clube
/// abre vaga (0x544820), a vaga (0x69ba50 com o técnico que saiu); tira o item (0x68d1f0). Item nulo: a caixa "Error".
pub(crate) unsafe extern "thiscall" fn saidas_pendentes_dos_tecnicos(this: usize) {
    let lista = read_usize(this + 4);
    if lista == 0 {
        return;
    }
    cpo4_por(lista + 0xa, read_usize(lista + 2));
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let remover: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_LLIST_REMOVE);
    let sair: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) = core::mem::transmute(FN_MANAGER_MANAGER_REMOVE_CLUB_MANAGER);
    let abre_vaga: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_CLUB_CAN_BE_MANAGED);
    let vaga: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_MANAGER_MANAGER_ADD_JOB_VACANCY);
    let mut k = 0i32;
    while k < read_i16(read_usize(this + 4)) as i32 {
        let item = proximo(read_usize(this + 4));
        if item == 0 {
            aviso_em(0x9f18a4, 0x2b70);
        } else {
            let id = read_usize(item + 2) as u32;
            let clube = cpo4_clube(id as usize);
            let tecnico = read_usize(clube + 0xcf);
            if tecnico != 0 {
                let a = (id & 0xffff_ff00) | read_u8(item) as u32;
                let b = (id.wrapping_mul(145) & 0xffff_ff00) | read_u8(item + 1) as u32;
                sair(this, clube, a, b, 0);
                if abre_vaga(clube) as u8 != 0 {
                    crate::comparacao::ramo("cpo_4 abrir a vaga");
                    vaga(this, clube, tecnico); // a vaga do clube
                }
            }
            remover(read_usize(this + 4), item);
        }
        k += 1;
    }
}

/// A função sem vaga no elenco depois de sair (12 → 5, 13 → 6, 15 → 8: a pessoa fica no clube) ou 0xb (sai).
pub(crate) fn cpo4_funcao_depois(f: u8) -> (u8, bool) {
    match f {
        0xc => (5, true),
        0xd => (6, true),
        0xf => (8, true),
        _ => (0xb, false),
    }
}

/// Tira a pessoa da primeira das 50 vagas do elenco do clube (+0xd7) em que ela está.
pub(crate) unsafe fn cpo4_tirar_das_50(clube: usize, pessoa: usize) {
    for k in 0..0x32usize {
        let v = clube + 0xd7 + 4 * k;
        if read_usize(v) == pessoa {
            crate::comparacao::ramo("cpo_4 tirar do elenco");
            cpo4_por(v, 0); // a vaga do elenco
            return;
        }
    }
}

/// takes the player out of the club (or the national team): a player-manager job goes back to the staff job (else 0xb
/// and he leaves), out of the lists, the media, the tactics and the 50 squad slots; the loan gets state 4
/// Original: `MANAGER_MANAGER::remove_player_from_squad` (0x690730, manager_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x690730 } (thiscall, ret 8, manager_manager.cpp): `MANAGER_MANAGER::remove_player_from_squad(pessoa, clube)`: no
/// clube da pessoa (+0x39): a função (+0x3d) de jogador-técnico volta à de técnico (12 → 5, 13 → 6, 15 → 8) ou vira
/// 0xb e ela sai do clube; tira das listas (0x7c5fe0), da mídia (0x843e10 com 4, 2), das duas táticas do clube (0x896c60)
/// e das 50 vagas do elenco (+0xd7). Na seleção (+0x24, função +0x28) o mesmo sem a 0x7c5fe0 e a 0x843e10. No fim o
/// contrato do empréstimo (0x4cf5a0) fica com o estado +0x4e = 4. Devolve o da 0x4cf5a0.
pub(crate) unsafe extern "thiscall" fn tirar_jogador_do_elenco_do_clube(_this: usize, pessoa: usize, clube: usize) -> usize {
    let tatica: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(FN_TACTICS_REMOVE_PLAYER_FROM_SELECTION);
    if read_usize(pessoa + 0x39) == clube {
        let (f, fica) = cpo4_funcao_depois(read_u8(pessoa + 0x3d));
        *((pessoa + 0x3d) as *mut u8) = f;
        let listas: unsafe extern "thiscall" fn(usize, usize, i32, u32, u32, i32, u32) = core::mem::transmute(FN_PLAYERSTATS_ARCHIVE_PLAYER_STATS);
        listas(read_usize(0xb74350), pessoa, -1, 0, 0, -1, 0);
        let midia: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(FN_SHORTLIST_MANAGER_UPDATE_STAFF_STATUS);
        midia(read_usize(read_usize(0xde1f54) + 0x856), pessoa, clube, 4, 2);
        tatica(read_usize(0xde1d08), clube, pessoa, 0, 1);
        tatica(read_usize(0xde1d08), clube, pessoa, 1, 1);
        cpo4_tirar_das_50(clube, pessoa);
        if !fica {
            cpo4_por(pessoa + 0x39, 0);
        }
    }
    if read_usize(pessoa + 0x24) == clube {
        let (f, fica) = cpo4_funcao_depois(read_u8(pessoa + 0x28));
        *((pessoa + 0x28) as *mut u8) = f;
        tatica(read_usize(0xde1d08), clube, pessoa, 0, 1);
        tatica(read_usize(0xde1d08), clube, pessoa, 1, 1);
        cpo4_tirar_das_50(clube, pessoa);
        if !fica {
            cpo4_por(pessoa + 0x24, 0);
        }
    }
    let emprestimo: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let r = emprestimo(read_usize(0xae1050), pessoa);
    if r != 0 {
        *((r + 0x4e) as *mut u8) = 4;
    }
    r
}

/// Zera as `n` vagas de `c + base` que guardam a pessoa (todas).
pub(crate) unsafe fn cpo4_zerar_vagas(c: usize, base: usize, n: usize, pessoa: usize) {
    for k in 0..n {
        if read_usize(c + base + 4 * k) == pessoa {
            cpo4_por(c + base + 4 * k, 0);
        }
    }
}

/// O corpo da 0x690890 sem as chamadas: a pessoa `pessoa` (campos em `s`) sai do cargo no clube `clube` (campos em `c`).
/// `s` e `c` são os próprios registros na cópia e as cópias deles no plano do comparador. Pela função no clube (+0x3d,
/// tabela 0x690af8): 1 presidente (+0xbf), 2 diretores (+0xc3, 3), 5 técnico (+0xcf, se for ele), 6 auxiliar (+0xd3),
/// 8 treinadores (+0x19f, 5), 9 olheiros (+0x1b3, 7), 10 fisioterapeutas (+0x1cf, 3): a vaga zerada e +0x39 = 0;
/// 12, 13, 15 (jogador-técnico, jogador-auxiliar, jogador-treinador): a vaga zerada e a função 0xb (fica no clube);
/// outras: +0x39 = 0 só se o byte baixo do ponteiro da pessoa é 0 (o `[esp+0x10]` da original). Na seleção (+0x24,
/// função +0x28, tabela 0x690b34) o mesmo com +0x24, sem a conferência do técnico, e as outras funções saem.
pub(crate) unsafe fn cpo4_desligar(pessoa: usize, s: usize, clube: usize, c: usize) {
    if read_usize(s + 0x39) == clube {
        match (read_u8(s + 0x3d) as i8 as i32).wrapping_sub(1) as u32 {
            0 => {
                cpo4_por(c + 0xbf, 0);
                cpo4_por(s + 0x39, 0);
            }
            1 => {
                cpo4_zerar_vagas(c, 0xc3, 3, pessoa);
                cpo4_por(s + 0x39, 0);
            }
            4 => {
                if read_usize(c + 0xcf) == pessoa {
                    crate::comparacao::ramo("cpo_4 tirar o tecnico");
                    cpo4_por(c + 0xcf, 0); // o técnico do clube
                }
                cpo4_por(s + 0x39, 0);
            }
            5 => {
                cpo4_por(c + 0xd3, 0);
                cpo4_por(s + 0x39, 0);
            }
            7 => {
                cpo4_zerar_vagas(c, 0x19f, 5, pessoa);
                cpo4_por(s + 0x39, 0);
            }
            8 => {
                cpo4_zerar_vagas(c, 0x1b3, 7, pessoa);
                cpo4_por(s + 0x39, 0);
            }
            9 => {
                cpo4_zerar_vagas(c, 0x1cf, 3, pessoa);
                cpo4_por(s + 0x39, 0);
            }
            11 => {
                *((s + 0x3d) as *mut u8) = 0xb;
                if read_usize(c + 0xcf) == pessoa {
                    cpo4_por(c + 0xcf, 0);
                }
            }
            12 => {
                *((s + 0x3d) as *mut u8) = 0xb;
                cpo4_por(c + 0xd3, 0);
            }
            14 => {
                *((s + 0x3d) as *mut u8) = 0xb;
                cpo4_zerar_vagas(c, 0x19f, 5, pessoa);
            }
            _ => {
                if pessoa as u8 == 0 {
                    cpo4_por(s + 0x39, 0);
                }
            }
        }
    } else if read_usize(s + 0x24) == clube {
        match (read_u8(s + 0x28) as i8 as i32).wrapping_sub(1) as u32 {
            0 => cpo4_por(c + 0xbf, 0),
            1 => cpo4_zerar_vagas(c, 0xc3, 3, pessoa),
            4 => cpo4_por(c + 0xcf, 0),
            5 => cpo4_por(c + 0xd3, 0),
            7 => cpo4_zerar_vagas(c, 0x19f, 5, pessoa),
            8 => cpo4_zerar_vagas(c, 0x1b3, 7, pessoa),
            9 => cpo4_zerar_vagas(c, 0x1cf, 3, pessoa),
            11 => {
                cpo4_por(c + 0xcf, 0);
                *((s + 0x28) as *mut u8) = 0xb;
                return;
            }
            12 => {
                *((s + 0x28) as *mut u8) = 0xb;
                cpo4_por(c + 0xd3, 0);
                return;
            }
            14 => {
                *((s + 0x28) as *mut u8) = 0xb;
                cpo4_zerar_vagas(c, 0x19f, 5, pessoa);
                return;
            }
            _ => {}
        }
        cpo4_por(s + 0x24, 0);
    }
}

/// removes the contract and the person from the job slot at the club or national team (chairman, director, manager,
/// assistant, coach, scout, physio; the playing jobs become 0xb), from the media and, if human, from the human's lists
/// Original: `MANAGER_MANAGER::remove_staff_from_club` (0x690890, manager_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x690890 } (thiscall, ret 8, manager_manager.cpp): `MANAGER_MANAGER::remove_staff_from_club(pessoa, clube)`: tira o
/// contrato (0x4d0b50 em [0xae1050]), tira a pessoa do cargo no clube ou na seleção (`cpo4_desligar`), da mídia
/// (0x843e10 com 4, 2) e, se ela é humana (entre as 16 últimas pessoas), das listas do humano (0x7f7270 em
/// [0xb74448]) e do clube (0x8b2470 em [0xde1f44] com 0). Devolve o eax da última chamada.
pub(crate) unsafe extern "thiscall" fn tirar_pessoa_do_cargo_no_clube(_this: usize, pessoa: usize, clube: usize) -> usize {
    let contrato: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_CONTRACT_MANAGER_REMOVE_CONTRACT);
    contrato(read_usize(0xae1050), pessoa);
    cpo4_desligar(pessoa, pessoa, clube, clube);
    let midia: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(FN_SHORTLIST_MANAGER_UPDATE_STAFF_STATUS);
    let mut r = midia(read_usize(read_usize(0xde1f54) + 0x856), pessoa, clube, 4, 2);
    if cpo4_i(pessoa) >= cpo4_i(0xae2364).wrapping_sub(0x10) {
        let humano: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_SCOUT_MANAGER_SYNC_SCOUT_LISTS);
        humano(read_usize(0xb74448), pessoa);
        let lista: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> usize = core::mem::transmute(FN_TRAINING_MANAGER_REASSIGN_SQUAD_SCHEDULES);
        r = lista(read_usize(0xde1f44), pessoa, clube, 0);
    }
    r
}

/// Comparador: a saída e a vaga (0x69b790, 0x69ba50) demitem e contratam: roda só a original.
// ponytail: sem plano (a 0x69ba50 pode acrescentar saídas na mesma lista); a prova é a carreira e o modo Rust.
pub(crate) unsafe extern "thiscall" fn comparar_saidas_pendentes_dos_tecnicos(this: usize) {
    orig::<unsafe extern "thiscall" fn(usize)>(976)(this)
}

/// Os campos da pessoa e do clube que a 0x690730/0x690890 gravam: (pessoa +0x24..+0x29, +0x39..+0x3e) e as vagas do
/// clube (+0xbf..+0x1db).
pub(crate) unsafe fn cpo4_campos(pessoa: usize, clube: usize) -> ([u8; 6], [u8; 5], [u8; 0x11c]) {
    let mut a = [0u8; 6];
    let mut b = [0u8; 5];
    let mut c = [0u8; 0x11c];
    core::ptr::copy_nonoverlapping((pessoa + 0x24) as *const u8, a.as_mut_ptr(), 6);
    core::ptr::copy_nonoverlapping((pessoa + 0x39) as *const u8, b.as_mut_ptr(), 5);
    core::ptr::copy_nonoverlapping((clube + 0xbf) as *const u8, c.as_mut_ptr(), 0x11c);
    (a, b, c)
}

/// Comparador: a original tira a pessoa das listas, da mídia e das táticas (não pode rodar duas vezes): roda só ela e
/// confere o plano: a função e o clube (e a seleção) da pessoa e ela fora das 50 vagas do elenco.
pub(crate) unsafe extern "thiscall" fn comparar_tirar_jogador_do_elenco_do_clube(this: usize, pessoa: usize, clube: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> usize>(977);
    let no_clube = read_usize(pessoa + 0x39) == clube;
    let na_selecao = read_usize(pessoa + 0x24) == clube;
    let (mut f39, mut c39) = (read_u8(pessoa + 0x3d), read_usize(pessoa + 0x39));
    let (mut f28, mut c24) = (read_u8(pessoa + 0x28), read_usize(pessoa + 0x24));
    if no_clube {
        let (f, fica) = cpo4_funcao_depois(f39);
        f39 = f;
        if !fica {
            c39 = 0;
        }
    }
    if c24 == clube {
        let (f, fica) = cpo4_funcao_depois(f28);
        f28 = f;
        if !fica {
            c24 = 0;
        }
    }
    let r = o(this, pessoa, clube);
    let fora = !(no_clube || na_selecao) || (0..0x32usize).all(|k| read_usize(clube + 0xd7 + 4 * k) != pessoa);
    let ok = fora && read_u8(pessoa + 0x3d) == f39 && read_usize(pessoa + 0x39) == c39 && read_u8(pessoa + 0x28) == f28 && read_usize(pessoa + 0x24) == c24;
    PLACAR.registrar(ok, || format!("tirar_jogador_do_elenco_do_clube(pessoa {:x}, clube {:x}): função {:x}/{:x} clube {:x}/{:x} (esperava {:x}/{:x} {:x}/{:x}), fora das vagas {}",
        read_usize(pessoa), read_usize(clube), read_u8(pessoa + 0x3d), read_u8(pessoa + 0x28), read_usize(pessoa + 0x39), read_usize(pessoa + 0x24), f39, f28, c39, c24, fora));
    r
}

/// Comparador: a original tira o contrato e tira a pessoa da mídia e das listas do humano (não pode rodar duas vezes):
/// roda só ela e confere o plano feito antes com o corpo (`cpo4_desligar`) sobre cópias dos campos da pessoa e das vagas
/// do clube. Pessoa humana: só a original (as listas do humano podem mexer no clube).
pub(crate) unsafe extern "thiscall" fn comparar_tirar_pessoa_do_cargo_no_clube(this: usize, pessoa: usize, clube: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> usize>(978);
    if pessoa == 0 || clube == 0 || cpo4_i(pessoa) >= cpo4_i(0xae2364).wrapping_sub(0x10) {
        return o(this, pessoa, clube);
    }
    // o plano: os campos em cópias com os mesmos deslocamentos
    let mut s = [0u8; 0x40];
    let mut c = [0u8; 0x1e0];
    core::ptr::copy_nonoverlapping(pessoa as *const u8, s.as_mut_ptr(), 0x40);
    core::ptr::copy_nonoverlapping(clube as *const u8, c.as_mut_ptr(), 0x1e0);
    cpo4_desligar(pessoa, s.as_mut_ptr() as usize, clube, c.as_mut_ptr() as usize);
    let esperado = cpo4_campos(s.as_ptr() as usize, c.as_ptr() as usize);
    let r = o(this, pessoa, clube);
    let depois = cpo4_campos(pessoa, clube);
    PLACAR.registrar(depois == esperado, || format!("tirar_pessoa_do_cargo_no_clube(pessoa {:x}, função {}, clube {:x}): pessoa {:x?}{:x?} esperava {:x?}{:x?}",
        read_usize(pessoa), read_u8(pessoa + 0x3d) as i8, read_usize(clube), depois.0, depois.1, esperado.0, esperado.1));
    r
}

// ---------- 250a leva: as vagas de técnico e o agendamento das ofertas: abrir as vagas que faltam, a melhor nota de candidato a técnico e agendar a oferta (manager_manager.cpp, transfer_manager.cpp) ----------




/// O `clube` já tem vaga na lista [this+8] (+0 conta, cursor +0xa recomeçado de +2, iterador 0x68d1b0; +0xe da vaga o
/// id do clube)? `None`: a lista acabou antes da conta (a caixa da original).
pub(crate) unsafe fn cpn5_tem_vaga(this: usize, clube: usize) -> Option<bool> {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let l = read_usize(this + 8);
    core::ptr::write_unaligned((l + 0xa) as *mut usize, read_usize(l + 2));
    if read_i16(read_usize(this + 8)) <= 0 {
        return Some(false);
    }
    let mut j = 0i32;
    loop {
        let v = proximo(read_usize(this + 8));
        if v == 0 {
            return None;
        }
        if read_usize(v + 0xe) == read_usize(clube) {
            crate::comparacao::ramo("cpn_5 clube com vaga aberta");
            return Some(true);
        }
        j += 1;
        if j >= read_i16(read_usize(this + 8)) as i32 {
            return Some(false);
        }
    }
}

/// O `clube` está sem técnico (0x53ca90), pode ser dirigido (0x544820), não é o time B de um par (0x540a50 com 1
/// achando o parceiro com 0 em `eh_a`) e não tem vaga aberta? `None`: a caixa da lista.
pub(crate) unsafe fn cpn5_falta_vaga(this: usize, clube: usize, eh_a: &mut u32) -> Option<bool> {
    let tecnico: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_CLUB_MANAGER);
    let dirigivel: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(FN_CLUB_CAN_BE_MANAGED);
    let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
    if tecnico(clube) != 0 || dirigivel(clube) == 0 {
        return Some(false);
    }
    if parceiro(clube, eh_a, 1) != 0 && *eh_a == 0 {
        return Some(false);
    }
    cpn5_tem_vaga(this, clube).map(|tem| !tem)
}

/// each club without a manager (0x53ca90), manageable (0x544820), not a B team and without an entry in the [this+8]
/// list gets the "Job not found" note and a job vacancy (0x69ba50)
/// Original: `MANAGER_MANAGER::fix_missing_jobs` (0x69d060, manager_manager.cpp, thiscall, ret 0).
///
/// [pt] { 0x69d060 } (thiscall, ret 0, manager_manager.cpp): `MANAGER_MANAGER::fix_missing_jobs()`: cada clube
/// ([0xae23b4], 0x245 bytes, [0xae235c] relido a cada volta) sem técnico, dirigível, que não é time B e sem vaga
/// aberta (`cpn5_falta_vaga`) ganha o aviso "Job not found for %s" (nome +4) em [0xde1f64] e a vaga (0x69ba50 sem
/// técnico anterior). A lista que acaba antes da conta: a caixa "Error" (linha 0x311e) e a rotina acaba. Sem SEH, sem
/// sorteio; a local do parceiro começa em 0 uma vez só e só é lida quando a 0x540a50 o acha (e a grava).
pub(crate) unsafe extern "thiscall" fn abrir_vagas_que_faltam(this: usize) {
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let abrir: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_MANAGER_MANAGER_ADD_JOB_VACANCY);
    let mut eh_a = 0u32;
    let mut clube = read_usize(0xae23b4);
    let mut i = 0i32;
    while i < read_usize(0xae235c) as i32 {
        match cpn5_falta_vaga(this, clube, &mut eh_a) {
            None => {
                aviso_em(0x9f18a4, 0x311e);
                return;
            }
            Some(true) => {
                formatar(0xde1f64, 0xa05298, clube + 4);
                abrir(this, clube, 0);
            }
            Some(false) => {}
        }
        i += 1;
        clube += 0x245;
    }
}

/// the highest manager score (0x69d1b0, job 5) among the available staff accepted for the club (nation, continent,
/// reputation, contract, 0x69f810), ignoring the current manager; nobody: −10000
/// Original: `MANAGER_MANAGER::best_manager_candidate_score` (0x69c540, manager_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x69c540 } (thiscall, ret 8, manager_manager.cpp): `MANAGER_MANAGER::best_manager_candidate_score(clube,
/// atual)`: a maior nota de técnico (0x69d1b0 com 0 e o cargo 5) entre as pessoas ([0xae23bc], 0x6e bytes,
/// [0xae2364] relido a cada volta) com ficha da comissão (+0x69), das 16 últimas só as humanas (+0x5f), que não são o
/// `atual`, sem as marcas 0x30 no byte +0xb do registro de 79 bytes ([0xae2d0c]), sem contrato de empréstimo (0x4cf6d0),
/// que não são candidatas de outra vaga (0x69fc60), com nação (+0x1a) e o clube com nação (+0x53): de outra nação só
/// com reputação (+0x69 → +0xa) a partir de 3750; de outro continente (+0x75) com +0x69 → +0xc a partir de 4250 e o
/// clube com reputação (+0x80) a partir de 4750; jogador (+0x61) com clube (+0x39) e cargo 11, 12, 13 ou 15 não pode ter
/// contrato com o clube (0x4cf5f0) marcado em +0x49; e aceita para o cargo (0x69f810 com 3,5). Ninguém: −10000.
pub(crate) unsafe extern "thiscall" fn melhor_nota_de_candidato_a_tecnico(this: usize, clube: usize, atual: usize) -> i32 {
    let emprestimo: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_LOAN_CONTRACT);
    let candidato: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_MANAGER_MANAGER_IS_VACANCY_CANDIDATE);
    let contrato: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let serve: unsafe extern "thiscall" fn(usize, usize, usize, f64) -> u8 = core::mem::transmute(FN_MANAGER_MANAGER_IS_STAFF_SUITABLE_FOR_JOB);
    let nota_de: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> i32 = core::mem::transmute(FN_SCORE_STAFF_FOR_JOB);
    let mut melhor = -10000i32;
    let mut p = read_usize(0xae23bc);
    let mut i = 0i32;
    while i < read_usize(0xae2364) as i32 {
        let reg79 = read_usize(0xae2d0c).wrapping_add((i as usize).wrapping_mul(0x4f));
        let pula = 'f: {
            if read_usize(p + 0x69) == 0 {
                break 'f true;
            }
            if i >= (read_usize(0xae2364) as i32).wrapping_sub(0x10) && read_u8(p + 0x5f) == 0 {
                break 'f true;
            }
            if atual == p {
                crate::comparacao::ramo("cpn_5 o tecnico atual fica de fora");
                break 'f true; // o atual fica de fora
            }
            if read_u8(reg79 + 0xb) & 0x30 != 0 || emprestimo(read_usize(0xae1050), p) != 0 || candidato(this, p) != 0 {
                break 'f true;
            }
            let (nc, np) = (read_usize(clube + 0x53), read_usize(p + 0x1a));
            if nc != np && read_i16(read_usize(p + 0x69) + 0xa) < 0xea6 {
                break 'f true;
            }
            if nc == 0 || np == 0 {
                break 'f true;
            }
            if read_u8(nc + 0x75) != read_u8(np + 0x75) && (read_i16(read_usize(p + 0x69) + 0xc) < 0x109a || read_i16(clube + 0x80) < 0x128e) {
                break 'f true;
            }
            if read_usize(p + 0x61) != 0 && read_usize(p + 0x39) != 0 && matches!(read_u8(p + 0x3d), 0xb | 0xc | 0xd | 0xf) {
                let k = contrato(read_usize(0xae1050), p, clube);
                if k != 0 && read_u8(k + 0x49) != 0 {
                    break 'f true;
                }
            }
            serve(this, p, clube, 3.5) == 0
        };
        if !pula {
            let s = nota_de(this, p, clube, 0, 5);
            if s > melhor {
                melhor = s;
            }
        }
        i += 1;
        p += 0x6e;
    }
    melhor
}

/// O plano do agendamento: a data da oferta (hoje [0xae2c90] + `dias`; com `dias` ≥ 50 a original mostra a caixa e
/// usa 10) com a fatia [0xae2c88] + `fatias` trazida para 0..2 (cada 3 fatias, mais um dia), e o balde da lista: os
/// dias desde a data base (+0x84d; ainda sem ano em +0x84f, hoje) mais o começo (+0x84c), módulo 50 de 50 a 127, × 3 +
/// fatia, nas listas de 14 bytes a partir de +0x18. Só lê (0x54c5b0 e 0x54cdb0 sobre cópias das datas).
pub(crate) unsafe fn cpn5_plano(this: usize, dias: u8, fatias: u8) -> ([u8; 8], i8, usize) {
    let somar: unsafe extern "thiscall" fn(usize, *mut u8, u32) -> usize = core::mem::transmute(FN_DATE_ADD_DAYS);
    let desde: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
    let base = if read_i16(this + 0x84f) == 0 { 0xae2c90 } else { this + 0x84d };
    let (b0, b1) = (read_usize(base) as u32, read_usize(base + 4) as u32);
    let dias = if dias as i8 >= 0x32 { 10 } else { dias as i8 };
    let (mut data, mut tmp) = ([0u8; 8], [0u8; 8]);
    // a 0x54c5b0 lê os dias em 16 bits (a original empurra o registro com lixo nos bits altos)
    let r = somar(0xae2c90, tmp.as_mut_ptr(), dias as i16 as i32 as u32);
    core::ptr::copy_nonoverlapping(r as *const u8, data.as_mut_ptr(), 8);
    let mut fatia = read_u8(0xae2c88).wrapping_add(fatias) as i8;
    while fatia > 2 {
        fatia -= 3;
        let r = somar(data.as_ptr() as usize, tmp.as_mut_ptr(), 1);
        core::ptr::copy_nonoverlapping(r as *const u8, data.as_mut_ptr(), 8);
    }
    let mut c = (desde(data.as_ptr() as usize, b0, b1) as u8).wrapping_add(read_u8(this + 0x84c));
    if c as i8 >= 0x32 {
        let q = (c as i8 as i32 as u32) / 50;
        c = c.wrapping_add((q as u8).wrapping_mul(0xce)); // − 50 × q
    }
    let balde = (c as i8 as i32).wrapping_mul(3).wrapping_add(fatia as i32);
    (data, fatia, this.wrapping_add(0x18).wrapping_add((balde as usize).wrapping_mul(14)))
}

/// gives the offer today's date + days and the day slice (0 to 2, every 3 slices one more day) and puts it in the
/// bucket (days since the base date + start, modulo 50, × 3 + slice) of the +0x18 lists
/// Original: `TRANSFER_MANAGER::schedule_offer` (0x8c6220, transfer_manager.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x8c6220 } (thiscall, ret 0xc, transfer_manager.cpp): `TRANSFER_MANAGER::schedule_offer(oferta, dias, fatias)`:
/// sem data base (ano +0x84f zerado), ela vira hoje (0x417c10 de [0xae2c90] em +0x84d) e a fatia +0x855 a de hoje
/// ([0xae2c88]); `dias` ≥ 50: a caixa "Error" (linha 0x18b9) e 10 dias. A oferta ganha a data (+0x12) e a fatia (+0x1a)
/// do plano (`cpn5_plano`) e o +0x20 dela (4 bytes) entra no balde (0x68d0d0, que aloca); devolve o que a 0x68d0d0
/// devolve. Sem SEH, sem sorteio.
pub(crate) unsafe extern "thiscall" fn agendar_oferta(this: usize, oferta: usize, dias: u32, fatias: u32) -> u32 {
    crate::comparacao::ramo("cpn_5 agendar oferta");
    if read_i16(this + 0x84f) == 0 {
        let copiar: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
        let mut tmp = [0u8; 8];
        copiar(this + 0x84d, tmp.as_mut_ptr(), 0xae2c90);
        *((this + 0x855) as *mut u8) = read_u8(0xae2c88);
    }
    if dias as u8 as i8 >= 0x32 {
        aviso_em(0xa8d514, 0x18b9);
    }
    // ponytail: as datas são feitas no plano sobre cópias (0x54c5b0, 0x54cdb0 e 0x417c10 só leem e copiam) e gravadas
    // de uma vez; a original grava a data e a fatia a cada volta da fatia, com o mesmo fim
    let (data, fatia, balde) = cpn5_plano(this, dias as u8, fatias as u8);
    core::ptr::copy_nonoverlapping(data.as_ptr(), (oferta + 0x12) as *mut u8, 8);
    *((oferta + 0x1a) as *mut i8) = fatia;
    let acrescentar: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_LLIST_ADD_TAIL);
    acrescentar(balde, oferta + 0x20, 4)
}

/// Comparador: sem vaga a abrir (o plano pelo mesmo teste, com o cursor da lista devolvido), lado a lado pelo cursor
/// da lista; com vaga a abrir (ou a caixa), só a original.
pub(crate) unsafe extern "thiscall" fn comparar_abrir_vagas_que_faltam(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(958);
    let l = read_usize(this + 8);
    let cursor = read_usize(l + 0xa);
    let mut eh_a = 0u32;
    let mut clube = read_usize(0xae23b4);
    let mut vazio = true;
    for _ in 0..(read_usize(0xae235c) as i32).max(0) {
        if cpn5_falta_vaga(this, clube, &mut eh_a) != Some(false) {
            vazio = false;
            break;
        }
        clube += 0x245;
    }
    core::ptr::write_unaligned((l + 0xa) as *mut usize, cursor);
    if !vazio {
        // ponytail: a 0x69ba50 abre a vaga (aloca, sorteia o candidato): não roda duas vezes; o modo Rust e a carreira provam
        return o(this);
    }
    let reg = [(l + 0xa, 4)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this), || abrir_vagas_que_faltam(this));
    PLACAR.registrar(igual, || format!("abrir_vagas_que_faltam (nenhuma a abrir) {:x?} x {:x?}", mo, mr));
}

/// Comparador: lado a lado a partir do mesmo estado do sorteio (a 0x69d1b0 e a 0x69f810 podem sortear), pela nota e
/// pelo cursor da lista de vagas (a 0x69fc60 o devolve).
pub(crate) unsafe extern "thiscall" fn comparar_melhor_nota_de_candidato_a_tecnico(this: usize, clube: usize, atual: usize) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> i32>(959);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.push((read_usize(this + 8) + 0xa, 4));
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube, atual), || crate::aleatorio::sem_contar(|| melhor_nota_de_candidato_a_tecnico(this, clube, atual)));
    PLACAR.registrar(igual && r == n, || format!("melhor_nota_de_candidato_a_tecnico(clube {}, atual {:x}) original={} rust={} {:x?} x {:x?}", read_usize(clube), atual, r, n, mo, mr));
    r
}

/// Comparador: a original roda (a 0x68d0d0 aloca o nó) e o resultado é conferido com o plano feito antes: a data
/// (+0x12) e a fatia (+0x1a) da oferta, a conta do balde mais 1 e a data base (+0x84d, 8 bytes) igual a hoje quando
/// não havia.
pub(crate) unsafe extern "thiscall" fn comparar_agendar_oferta(this: usize, oferta: usize, dias: u32, fatias: u32) -> u32 {
    let sem_base = read_i16(this + 0x84f) == 0;
    let (data, fatia, balde) = cpn5_plano(this, dias as u8, fatias as u8);
    let conta = read_i16(balde);
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32>(960)(this, oferta, dias, fatias);
    let gravada = core::slice::from_raw_parts((oferta + 0x12) as *const u8, 8);
    let base_ok = !sem_base || core::slice::from_raw_parts((this + 0x84d) as *const u8, 8) == core::slice::from_raw_parts(0xae2c90 as *const u8, 8);
    let ok = gravada == &data[..] && *((oferta + 0x1a) as *const i8) == fatia && read_i16(balde) == conta.wrapping_add(1) && base_ok;
    PLACAR.registrar(ok, || format!("agendar_oferta(dias {}, fatias {}): data {:x?} (plano {:x?}), fatia {} (plano {}), balde {:x} conta {} -> {}, base {}",
        dias as u8 as i8, fatias as u8 as i8, gravada, data, *((oferta + 0x1a) as *const i8), fatia, balde, conta, read_i16(balde), base_ok));
    r
}

// ---------- 246a leva: depois da partida, a diretoria e a disciplina; as entradas do clube na shortlist e a maior oferta pelo jogador ----------


/// played match: the board confidence of each side with a manager changes (0x695240, home with 1)
/// Original: `MANAGER_MANAGER::update_board_after_fixture` (0x6951e0, manager_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x6951e0 } (thiscall, ret 8, manager_manager.cpp): `MANAGER_MANAGER::update_board_after_fixture(partida,
/// jogo)`: partida jogada (+0 ≠ −2, +0x40 ≠ 0): a confiança da diretoria de cada lado com técnico (+0xcf) muda pela
/// 0x695240 (mandante +0x1c com 1, visitante +0x20 com 0).
pub(crate) unsafe extern "thiscall" fn diretoria_depois_da_partida(this: usize, partida: usize, jogo: usize) {
    if read_usize(partida) as i32 == -2 || *((partida + 0x40) as *const u8) == 0 {
        return;
    }
    let atualizar: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) = core::mem::transmute(FN_MANAGER_MANAGER_UPDATE_BOARD_AFTER_MATCH);
    for (lado, mandante) in [(0x1cusize, 1u32), (0x20, 0)] {
        let clube = read_usize(partida + lado);
        if read_usize(clube + 0xcf) != 0 { // o lado tem técnico
            atualizar(this, clube, partida, jogo, mandante);
        }
    }
}

/// O tribunal (vetor [this], pelo byte +0x50 da competição de id `id` em [0xadadfc]), se houver.
pub(crate) unsafe fn cpj6_tribunal(this: usize, id: i32) -> Option<usize> {
    let comp = read_usize(read_usize(0xadadfc).wrapping_add((id as usize).wrapping_mul(4)));
    if comp == 0 {
        return None;
    }
    let t = *((comp + 0x50) as *const i8);
    if t == -1 {
        return None;
    }
    let b = read_usize(read_usize(this).wrapping_add((t as isize as usize).wrapping_mul(4)));
    (b != 0).then_some(b)
}

/// the competition ruling body judges the match (0x7e7ff0); without it, in friendlies, the league ruling bodies of both
/// clubs
/// Original: `Discipline::process_match` (0x55ca00, discipline.cpp, thiscall, ret 8).
///
/// [pt] { 0x55ca00 } (thiscall, ret 8, discipline.cpp): `Discipline::process_match(partida, resultado)`: o tribunal da
/// competição da partida (+0, id > −1) julga (0x7e7ff0 com 1, 1, 1). Sem ele, num amistoso (0x5c6710 sobre [0xae2c74]
/// com +0x14) ou na competição [0x9cf684]: o tribunal da liga (+0x57) do mandante (+0x1c, com 1, 0, 0) e o do
/// visitante (+0x20, com 0, 1, 0). Devolve o que o primeiro tribunal devolve (os outros caminhos, 0; ninguém lê).
pub(crate) unsafe extern "thiscall" fn disciplina_da_partida(this: usize, partida: usize, resultado: u32) -> u32 {
    let julgar: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_RULINGBODY_PROCESS_MATCH);
    let id = read_usize(partida) as i32;
    if id > -1 {
        if let Some(b) = cpj6_tribunal(this, id) {
            return julgar(b, partida, resultado, 1, 1, 1);
        }
    }
    let amistoso: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_FRIENDLY_IS_FRIENDLY_COMPETITION);
    if amistoso(read_usize(0xae2c74), read_usize(partida + 0x14)) == 0 && read_usize(partida) != read_usize(KEY_CLUB_COMP_RESERVE) {
        return 0;
    }
    for (lado, a, b) in [(0x1cusize, 1u32, 0u32), (0x20, 0, 1)] {
        let liga = read_usize(read_usize(partida + lado) + 0x57);
        if liga == 0 {
            continue;
        }
        if let Some(t) = cpj6_tribunal(this, read_usize(liga) as i32) { // o tribunal da liga do clube
            julgar(t, partida, resultado, a, b, 0);
        }
    }
    0
}

/// As entradas (8 bytes: pessoa, clube) da shortlist (+4, [+0x14] delas, ordenada pelo clube) do clube `id`: a busca
/// binária (bsearch 0x946464 com a chave {−1, id} e a comparação 0x844740), recuando até a primeira do clube.
pub(crate) unsafe fn cpj6_entradas(this: usize, id: u32) -> Vec<usize> {
    let chave = [u32::MAX, id];
    let bsearch: unsafe extern "C" fn(*const u32, usize, usize, usize, usize) -> usize = core::mem::transmute(FN_CRT_BSEARCH);
    let (base, n) = (read_usize(this + 4), read_usize(this + 0x14));
    let mut e = bsearch(chave.as_ptr(), base, n, 8, 0x844740);
    if e == 0 {
        return Vec::new();
    }
    let ultima = base.wrapping_add(n.wrapping_mul(8)).wrapping_sub(8);
    while e > base && read_usize(e - 4) as u32 == id {
        e -= 8; // a anterior também é do clube
    }
    let mut v = Vec::new();
    if read_usize(e + 4) as u32 != id {
        return v;
    }
    loop {
        v.push(e);
        if e == ultima || read_usize(e + 0xc) as u32 != id {
            return v;
        }
        e += 8;
    }
}

/// adds to the list every shortlist entry (8 bytes) of the club, found by binary search
/// Original: `Shortlist_Manager::collect_club_shortlist_entries` (0x8420e0, shortlist_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x8420e0 } (thiscall, ret 8, shortlist_manager.cpp): `Shortlist_Manager::collect_club_shortlist_entries(clube,
/// lista)`: acrescenta (0x68d0d0, 8 bytes) à `lista` cada entrada da shortlist do clube (`*clube`), na ordem.
pub(crate) unsafe extern "thiscall" fn entradas_do_clube_na_shortlist(this: usize, clube: *const u32, lista: usize) {
    let acrescentar: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_LLIST_ADD_TAIL);
    for e in cpj6_entradas(this, *clube) {
        acrescentar(lista, e, 8);
    }
}

/// A oferta `o` serve: do `clube` (+8) ou marcada (+0x2d = 1), e no estado 2 (+0x2c)?
pub(crate) unsafe fn cpj6_serve(o: usize, clube: u32) -> bool {
    (read_usize(o + 8) as u32 == clube || *((o + 0x2d) as *const u8) == 1) && *((o + 0x2c) as *const u8) == 2
}

/// of the offers for the player from the club (or flagged) in state 2, the highest (0x8e7a70); 0 when none
/// Original: `TRANSFER_MANAGER::find_highest_offer_for_player` (0x8cc930, transfer_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x8cc930 } (thiscall, ret 8, transfer_manager.cpp): `TRANSFER_MANAGER::find_highest_offer_for_player(jogador,
/// clube)`: com a marca de ofertas do jogador (byte +1 dos 8 de [+0x86e]), o primeiro registro do vetor [this] ([+4]
/// deles) do jogador (+0xc): sem lista (+0), ele mesmo se serve; com lista, das ofertas (índices no vetor) que servem,
/// a de maior valor (0x8e7a70 com 0, 0, 0, 1; a primeira no empate). Senão 0.
pub(crate) unsafe extern "thiscall" fn maior_oferta_pelo_jogador(this: usize, jogador: *const u32, clube: *const u32) -> usize {
    let id = *jogador;
    if *((read_usize(this + 0x86e).wrapping_add((id as usize).wrapping_mul(8)) + 1) as *const u8) == 0 {
        return 0;
    }
    let mut t = 0;
    let mut k = 0i32;
    while k < read_usize(this + 4) as i32 {
        let x = read_usize(read_usize(this).wrapping_add(k as usize * 4));
        if x != 0 && read_usize(x + 0xc) as u32 == id {
            t = x;
            break;
        }
        k += 1;
    }
    if t == 0 {
        return 0;
    }
    let l = read_usize(t);
    if l == 0 {
        return if cpj6_serve(t, *clube) { t } else { 0 };
    }
    core::ptr::write_unaligned((l + 0xa) as *mut usize, read_usize(l + 2));
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let valor: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> i32 = core::mem::transmute(FN_TRANSFER_OFFER_GET_TOTAL_VALUE);
    let (mut melhor, mut maior) = (0usize, -1i32);
    let mut e = proximo(l);
    while e != 0 {
        let o = read_usize(read_usize(this).wrapping_add(read_usize(e).wrapping_mul(4)));
        if cpj6_serve(o, *clube) {
            let v = valor(o, 0, 0, 0, 1);
            if v > maior { // a primeira no empate
                melhor = o;
                maior = v;
            }
        }
        e = proximo(l);
    }
    melhor
}

/// Comparador: a 0x695240 mexe na diretoria (e pode dar notícia): só a original (a prova é a carreira e o modo Rust).
pub(crate) unsafe extern "thiscall" fn comparar_diretoria_depois_da_partida(this: usize, partida: usize, jogo: usize) {
    crate::comparacao::ramo("diretoria_depois_da_partida");
    orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(879)(this, partida, jogo)
}

/// Comparador: o tribunal (0x7e7ff0) dá as suspensões: só a original.
pub(crate) unsafe extern "thiscall" fn comparar_disciplina_da_partida(this: usize, partida: usize, resultado: u32) -> u32 {
    crate::comparacao::ramo("disciplina_da_partida");
    orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(880)(this, partida, resultado)
}

/// Comparador: as entradas da cópia antes; a original acrescenta; confere os últimos itens da lista (8 bytes cada).
pub(crate) unsafe extern "thiscall" fn comparar_entradas_do_clube_na_shortlist(this: usize, clube: *const u32, lista: usize) {
    let esperado: Vec<[u8; 8]> = cpj6_entradas(this, *clube).iter().map(|&e| core::ptr::read_unaligned(e as *const [u8; 8])).collect();
    let antes = cpd1_itens(lista).len();
    orig::<unsafe extern "thiscall" fn(usize, *const u32, usize)>(881)(this, clube, lista);
    let novos: Vec<[u8; 8]> = cpd1_itens(lista).iter().skip(antes).map(|&i| core::ptr::read_unaligned(i as *const [u8; 8])).collect();
    PLACAR.registrar(novos == esperado, || format!("entradas_do_clube_na_shortlist({}) original {} itens, rust {}", *clube, novos.len(), esperado.len()));
}

/// Comparador: pelo valor (a 0x8e7a70 só calcula; as duas voltam o cursor da lista ao começo).
pub(crate) unsafe extern "thiscall" fn comparar_maior_oferta_pelo_jogador(this: usize, jogador: *const u32, clube: *const u32) -> usize {
    let r = orig::<unsafe extern "thiscall" fn(usize, *const u32, *const u32) -> usize>(882)(this, jogador, clube);
    let n = maior_oferta_pelo_jogador(this, jogador, clube);
    PLACAR.registrar(r == n, || format!("maior_oferta_pelo_jogador({}, {}) original={:#x} rust={:#x}", *jogador, *clube, r, n));
    r
}

// ---------- 174a leva: a pessoa na lista do tecnico, a ficha de contrato e o desgaste diario ----------

/// is the person in the list (iterator 0x68d1b0, restored at the end)?
/// Original: `person_in_list` (0x69fc60, manager_manager.cpp, thiscall).
///
/// [pt] { 0x69fc60 } (thiscall, ret 4, manager_manager.cpp): a pessoa `*p` está na lista [this+8]
/// (palavra da quantidade em +0, iterador 0x68d1b0 com posição em +0xa, recomeçada de +2 e
/// devolvida no fim)? Lista que acaba antes da quantidade: a original (caixa de erro). Devolve só `al`.
pub(crate) unsafe extern "thiscall" fn pessoa_na_lista(t: usize, p: *const i32) -> u8 {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let lista = *((t + 8) as *const usize);
    let guardado = core::ptr::read_unaligned((lista + 0xa) as *const u32);
    core::ptr::write_unaligned((lista + 0xa) as *mut u32, core::ptr::read_unaligned((lista + 2) as *const u32));
    let mut i = 0i32;
    while i < *(lista as *const i16) as i32 {
        let e = proximo(lista);
        if e == 0 {
            core::ptr::write_unaligned((lista + 0xa) as *mut u32, guardado);
            return orig::<unsafe extern "thiscall" fn(usize, *const i32) -> u8>(394)(t, p);
        }
        if core::ptr::read_unaligned((e + 6) as *const i32) == *p {
            core::ptr::write_unaligned((lista + 0xa) as *mut u32, guardado);
            return 1;
        }
        i += 1;
    }
    core::ptr::write_unaligned((lista + 0xa) as *mut u32, guardado);
    0
}

/// the person's contract record (0x50 bytes)
/// Original: `contract_record` (0x4cf6d0, contract_manager.cpp, thiscall).
///
/// [pt] { 0x4cf6d0 } (thiscall, ret 4, contract_manager.cpp): a ficha de contrato (0x50 bytes, a partir de
/// [this]) da pessoa `*p`, pelo índice em +4 do registro de 79 bytes em [0xae2d0c]; 0 sem pessoa, com
/// id fora de [0xae2364] ou índice negativo.
pub(crate) unsafe extern "thiscall" fn ficha_de_contrato(t: usize, p: *const i32) -> usize {
    if p.is_null() {
        return 0;
    }
    let id = *p;
    if id >= *(0xae2364 as *const i32) {
        return 0;
    }
    let k = core::ptr::read_unaligned((*(0xae2d0c as *const usize)).wrapping_add((id as usize).wrapping_mul(79)).wrapping_add(4) as *const i32);
    if k < 0 {
        return 0;
    }
    (*(t as *const usize)).wrapping_add(k as usize * 0x50)
}

/// the day's wear on word +0xc (age, draws of 25 and 13, goalkeeper cap)
/// Original: `daily_wear` (0x62c2f0, injury.cpp, thiscall).
///
/// [pt] { 0x62c2f0 } (thiscall, ret 4, injury.cpp): o desgaste do dia na palavra +0xc do registro de 31
/// bytes da pessoa `id` (a partir de [this]): abaixo de −1000 desce 1; abaixo de 19 anos (+0x18 da
/// pessoa em [0xae23bc]) e negativo, zera; senão, de −500 para cima e com idade ≥ sorteio(25), desce
/// 13, e nos outros casos desce sorteio(13); goleiro (posição +0xf da ficha = 20) fica até 1050.
/// Id fora de [0xae2364]: nada.
pub(crate) unsafe extern "thiscall" fn desgaste_diario(t: usize, id: i32) {
    if id >= *(0xae2364 as *const i32) {
        return;
    }
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let e = (*(t as *const usize)).wrapping_add((id as usize).wrapping_mul(31)).wrapping_add(0xc) as *mut i16;
    let v = core::ptr::read_unaligned(e);
    if v < -1000 {
        core::ptr::write_unaligned(e, v.wrapping_sub(1));
    }
    let p = (*(0xae23bc as *const usize)).wrapping_add((id as usize).wrapping_mul(0x6e));
    let idade = *((p + 0x18) as *const i8);
    if idade < 0x13 && core::ptr::read_unaligned(e) < 0 {
        core::ptr::write_unaligned(e, 0);
    } else if core::ptr::read_unaligned(e) >= -500 && idade as i32 >= sorteio(0x19) {
        core::ptr::write_unaligned(e, core::ptr::read_unaligned(e).wrapping_sub(0xd));
    } else {
        let r = sorteio(0xd) as i16;
        core::ptr::write_unaligned(e, core::ptr::read_unaligned(e).wrapping_sub(r));
    }
    let ficha = core::ptr::read_unaligned((p + 0x61) as *const usize);
    if ficha != 0 && *((ficha + 0xf) as *const u8) == 0x14 && core::ptr::read_unaligned(e) >= 0x41a {
        core::ptr::write_unaligned(e, 0x41a);
    }
}

comparar_valor!(comparar_pessoa_na_lista, pessoa_na_lista, 394, (p: *const i32) -> u8);
comparar_valor!(comparar_ficha_de_contrato, ficha_de_contrato, 395, (p: *const i32) -> usize);

pub(crate) unsafe extern "thiscall" fn comparar_desgaste_diario(t: usize, id: i32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32)>(396);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    if id < *(0xae2364 as *const i32) {
        reg.push(((*(t as *const usize)).wrapping_add((id as usize).wrapping_mul(31)).wrapping_add(0xc), 2));
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(t, id), || crate::aleatorio::sem_contar(|| desgaste_diario(t, id)));
    PLACAR.registrar(igual, || format!("desgaste_diario({:#x}, {}) {:x?} x {:x?}", t, id, mo, mr));
}

