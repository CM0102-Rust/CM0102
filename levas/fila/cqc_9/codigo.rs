// ---------- %%LEVA%%a leva: as expectativas da diretoria para a temporada, a vaga de técnico e a notícia da oferta de emprego (manager_manager.cpp) ----------

unsafe fn cqc9_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqc9_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cqc9_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqc9_b(a: usize) -> i8 {
    *(a as *const i8)
}

unsafe fn cqc9_u(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqc9_pd(a: usize, v: u32) {
    core::ptr::write_unaligned(a as *mut u32, v);
}

unsafe fn cqc9_pw(a: usize, v: i16) {
    core::ptr::write_unaligned(a as *mut i16, v);
}

unsafe fn cqc9_pb(a: usize, v: u8) {
    *(a as *mut u8) = v;
}

unsafe fn cqc9_f(a: usize) -> f64 {
    core::ptr::read_unaligned(a as *const f64)
}

/// _ftol (0x945640): truncado (a original usa só eax ou ax).
fn cqc9_ftol(x: f64) -> i32 {
    x as i64 as i32
}

unsafe fn cqc9_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// A linha da diretoria do clube: [this] + id × 0x49.
unsafe fn cqc9_linha(this: usize, clube: usize) -> usize {
    cqc9_d(this).wrapping_add((cqc9_i(clube) as usize).wrapping_mul(0x49))
}

/// O objeto da competição da divisão do clube: [[0xadadfc] + [+0x57] × 4].
unsafe fn club_division_comp(clube: usize) -> usize {
    cqc9_d(cqc9_d(0xadadfc).wrapping_add((cqc9_i(cqc9_d(clube + 0x57)) as usize).wrapping_mul(4)))
}

/// Método virtual sem argumentos da competição (+0x98 ou +0x9c da tabela), thiscall.
unsafe fn call_comp_virtual(obj: usize, desloc: usize) -> u32 {
    let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(cqc9_d(cqc9_d(obj) + desloc));
    f(obj)
}

unsafe fn cqc9_selecao(c: usize) -> u32 {
    let f: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    f(c)
}

/// { 0x6a3770 } (thiscall, ret 8, manager_manager.cpp): `MANAGER_MANAGER::set_season_expectations(clube, notícia)`.
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
unsafe extern "thiscall" fn set_season_expectations(this: usize, clube: usize, noticia: u32) {
    let ln = || cqc9_linha(this, clube);
    cqc9_pw(ln() + 0xe, 0);
    cqc9_pd(ln() + 0x2c, 0);
    let rep = cqc9_w(clube + 0x80) as i32;
    core::ptr::write_unaligned((ln() + 0x30) as *mut f64, rep.wrapping_mul(rep) as f64 * cqc9_f(0x9689e0));
    let l38 = ln() + 0x38;
    core::ptr::write_unaligned(l38 as *mut f64, cqc9_f(l38) * cqc9_f(0x9689e0));
    cqc9_pb(ln(), 0);
    let mut dias: i32 = 0;
    if cqc9_u(ln() + 2) & 1 == 0 && cqc9_d(clube + 0xcf) != 0 && cqc9_selecao(clube) == 0 {
        let tec = cqc9_d(clube + 0xcf);
        let ate: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x54cdb0usize);
        dias = ate(0xae2c90, cqc9_d(tec + 0x3e) as u32, cqc9_d(tec + 0x42) as u32);
        let pres = || {
            let bf = cqc9_d(clube + 0xbf);
            if bf != 0 && cqc9_d(bf + 0x69) != 0 { Some(cqc9_b(cqc9_d(bf + 0x69) + 0x1d) as i32) } else { None }
        };
        let sair = if dias > 0xe42 && cqc9_w(ln() + 6) < 0xdac {
            match pres() {
                Some(_) => {
                    let r = cqc9_sortear(0x1e);
                    r > pres().unwrap_or(0)
                }
                None => cqc9_sortear(0x1e) > 10,
            }
        } else if dias > 0x721 && cqc9_w(ln() + 6) < 0xbb8 {
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
            crate::comparison::branch("cqc_9 saida pelo tempo");
            // o item de 6 bytes {1, 6, id do clube} copiado na lista [this+4]
            let mut item = [0u8; 8];
            item[0] = 1;
            item[1] = 6;
            item[2..6].copy_from_slice(&(cqc9_d(clube) as u32).to_le_bytes());
            let acrescentar: unsafe extern "thiscall" fn(usize, *mut u8, u32) = core::mem::transmute(0x68d0d0usize);
            acrescentar(cqc9_d(this + 4), item.as_mut_ptr(), 6);
        }
    }
    if cqc9_selecao(clube) != 0 {
        let federacao: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(0x6a44a0usize);
        let segunda: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x5409b0usize);
        let mut feita = false;
        if segunda(clube) == 0 {
            let achar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x7751e0usize);
            let ent = achar(cqc9_d(0xb74318), cqc9_d(clube + 0x53));
            let co = if ent != 0 { cqc9_d(cqc9_d(0xadadfc).wrapping_add((cqc9_i(ent) as usize).wrapping_mul(4))) } else { 0 };
            if co != 0 {
                let exp: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(cqc9_d(cqc9_d(co) + 0x70));
                let w = exp(co, clube) as u16;
                cqc9_pw(ln() + 0xe, w as i16);
                cqc9_pd(ln() + 0x2c, cqc9_d(ent) as u32);
                if noticia as u8 != 0 {
                    // ponytail: a original empurra a palavra com a metade alta do registro (lixo); a 0x6a44a0 lê a palavra
                    federacao(this, clube, cqc9_w(ln() + 0xe) as u16 as u32, cqc9_d(ln() + 0x2c) as u32);
                }
                feita = true;
            } else {
                cqc9_pw(ln() + 0xe, 0);
                cqc9_pd(ln() + 0x2c, 0xffff_ffff);
            }
        }
        if !feita && noticia as u8 != 0 {
            federacao(this, clube, 0, 0xffff_ffff);
        }
        finish_board_update(this, clube);
        return;
    }
    // as vagas da UEFA da nação: campeão (o byte do 2º argumento) e as outras (o 3º)
    let mut vagas: u32 = clube as u32; // a original grava no lugar do argumento `clube` (só o byte é lido)
    let mut outras: u8 = 0;
    let uefa: unsafe extern "thiscall" fn(usize, usize, *mut u32, *mut u8) = core::mem::transmute(0x905c60usize);
    uefa(cqc9_d(0xde1f58), cqc9_d(clube + 0x53), &mut vagas, &mut outras);
    let vagas = vagas as u8 as i8 as i32;
    let outras = outras as i8 as i32;
    if cqc9_d(clube + 0xcf) == 0 {
        finish_board_update(this, clube);
        return;
    }
    let div = cqc9_d(clube + 0x57);
    let nacao = cqc9_d(clube + 0x53);
    if div == 0 || nacao == 0 || cqc9_u(nacao + 0x11c) == 0 || club_division_comp(clube) == 0 {
        cqc9_pw(ln() + 0xe, 0x10);
        post_expectations_news(this, clube, noticia);
        finish_board_update(this, clube);
        return;
    }
    let league_row: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
    let lg = league_row(cqc9_d(club_division_comp(clube) + 8), clube);
    let mut copa: usize = 0;
    let id = cqc9_i(clube + 0x1db);
    if id > -1 && id != cqc9_i(0x9cf6f8) {
        let cid = cqc9_i(cqc9_d(0xae23d0).wrapping_add((id as usize).wrapping_mul(0x6b)));
        let co = cqc9_d(cqc9_d(0xadadfc).wrapping_add((cid as usize).wrapping_mul(4)));
        copa = league_row(cqc9_d(co + 8), clube);
    }
    if lg == 0 || cqc9_u(lg + 0x72) < 1 {
        cqc9_pd(ln() + 0x2c, 8);
        cqc9_pb(ln() + 0xe, cqc9_u(ln() + 0xe) | 0x40);
        let diretoria: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x6a4a20usize);
        // ponytail: a original empurra a palavra com a metade alta do registro (lixo); a 0x6a4a20 lê a palavra
        diretoria(this, clube, cqc9_w(ln() + 0xe) as u16 as u32);
        return;
    }
    let meta = || cqc9_i(ln() + 0x2c);
    let por = |v: i32| cqc9_pd(ln() + 0x2c, v as u32);
    por(cqc9_u(lg + 0x72) as i32); // cqc9: a meta pela posição de perigo
    let n = cqc9_w(club_division_comp(clube) + 0x3e) as i32;
    if meta() == n {
        por(n - 1);
    }
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    let band: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x5986b0usize);
    let fin = financas(cqc9_d(0xae2a38), clube);
    crate::comparison::branch("cqc_9 meta");
    match band(fin) as i8 {
        2 => por(if meta() > 5 { meta() - 3 } else { meta() - 2 }),
        1 => por(if meta() > 5 { meta() - 2 } else { meta() - 1 }),
        0 => {
            let verba: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> i64 = core::mem::transmute(0x598800usize);
            let v = verba(fin, 0, 0, 0, 1, 0);
            if v > (cqc9_w(clube + 0x80) as i32 * 250) as i64 && meta() > 5 {
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
    let (d57, d5b) = (cqc9_d(clube + 0x57), cqc9_d(clube + 0x5b));
    if d57 != 0 && d5b != 0 && d57 != d5b {
        if cqc9_w(d57 + 0x69) > cqc9_w(d5b + 0x69) {
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
        let lim = cqc9_w(club_division_comp(clube) + 0x3e) as i32 - 1;
        if lim < x { lim } else { x }
    };
    por(x as i8 as i32);
    if cqc9_u(ln() + 2) & 1 != 0 && cqc9_u(lg + 0x74) >= 1 && cqc9_u(lg + 0x75) >= 1 {
        let v = (cqc9_u(lg + 0x73) as i32).wrapping_add(meta());
        por(cqc9_ftol(v as f64 * cqc9_f(0x968968)) as i8 as i32);
        let (lo, hi) = (cqc9_u(lg + 0x74) as i32, cqc9_u(lg + 0x75) as i32);
        let x = meta();
        let x = if lo > x { lo } else if hi < x { hi } else { x };
        por(x as i8 as i32);
    }
    // as marcas pela meta
    let co = club_division_comp(clube);
    let ou = |m: u8| cqc9_pb(ln() + 0xe, cqc9_u(ln() + 0xe) | m);
    'marcas: {
        if call_comp_virtual(co, 0x98) == 0 && meta() <= vagas {
            ou(2);
            break 'marcas;
        }
        let na = cqc9_d(clube + 0x53);
        if na != 0 && cqc9_i(na) != cqc9_i(0x9cf224) && call_comp_virtual(club_division_comp(clube), 0x98) == 0 && meta() <= vagas + outras {
            ou(1);
            break 'marcas;
        }
        let co = club_division_comp(clube);
        if call_comp_virtual(co, 0x98) != 0 && meta() <= cqc9_b(club_division_comp(clube) + 0xbe) as i32 {
            ou(4);
            break 'marcas;
        }
        let co = club_division_comp(clube);
        let bf = cqc9_b(co + 0xbf);
        if bf != 0 && meta() <= bf as i32 {
            ou(8);
            break 'marcas;
        }
        if meta() <= cqc9_w(co + 0x3e) as i32 / 2 {
            cqc9_pw(ln() + 0xe, 0x40);
            break 'marcas;
        }
        if call_comp_virtual(co, 0x9c) != 0 {
            let co = club_division_comp(clube);
            if meta() >= cqc9_w(co + 0x3e) as i32 - cqc9_b(co + 0xc1) as i32 {
                ou(0x20);
                break 'marcas;
            }
        }
        let r = call_comp_virtual(club_division_comp(clube), 0x9c);
        cqc9_pw(ln() + 0xe, if r != 0 { 0x10 } else { 0 });
    }
    // os sorteios do presidente e a liga da copa
    let bf = cqc9_d(clube + 0xbf);
    if cqc9_u(ln() + 2) & 1 == 0 && call_comp_virtual(club_division_comp(clube), 0x98) == 0 && bf != 0 && cqc9_d(bf + 0x69) != 0 {
        let p1d = || cqc9_b(cqc9_d(cqc9_d(clube + 0xbf) + 0x69) + 0x1d) as i32;
        crate::comparison::branch("cqc_9 sorteio do presidente");
        let r = cqc9_sortear(0x32); // cqc9: o sorteio do presidente
        if r > p1d() + meta() * 5 {
            ou(0x80);
        } else {
            let r = cqc9_sortear(0x4b);
            if r > p1d() + meta() * 5 {
                cqc9_pb(ln() + 0xf, cqc9_u(ln() + 0xf) | 1);
            } else {
                let r = cqc9_sortear(100);
                if r > p1d() + meta() * 5 {
                    cqc9_pb(ln() + 0xf, cqc9_u(ln() + 0xf) | 2);
                }
            }
        }
        if copa != 0 && cqc9_u(copa + 0x74) <= 1 {
            let t = (0x1d - p1d()) / 5;
            let p = cqc9_u(copa + 0x72) as i32;
            if p <= t {
                cqc9_pb(ln() + 0xf, cqc9_u(ln() + 0xf) | 4);
            } else if p <= t + 4 {
                cqc9_pw(ln() + 0xe, cqc9_w(ln() + 0xe) ^ 0x80);
                cqc9_pw(ln() + 0xe, cqc9_w(ln() + 0xe) ^ 0x200);
                cqc9_pb(ln() + 0xf, cqc9_u(ln() + 0xf) | 1);
            }
        }
    }
    post_expectations_news(this, clube, noticia);
    finish_board_update(this, clube);
}

/// A notícia das expectativas (0x6a4a20 com as marcas +0xe) quando pedida.
unsafe fn post_expectations_news(this: usize, clube: usize, noticia: u32) {
    if noticia as u8 != 0 {
        let diretoria: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x6a4a20usize);
        // ponytail: a original empurra a palavra com a metade alta do registro (lixo); a 0x6a4a20 lê a palavra
        diretoria(this, clube, cqc9_w(cqc9_linha(this, clube) + 0xe) as u16 as u32);
    }
}

/// O fim comum da 0x6a3770 (a partir de 0x6a415f): as confianças, a riqueza, a ficha do técnico, as marcas zeradas e as
/// multas.
unsafe fn finish_board_update(this: usize, clube: usize) {
    let ln = || cqc9_linha(this, clube);
    if cqc9_w(ln() + 6) < 0x1964 {
        crate::comparison::branch("cqc_9 confianca de volta");
        for (o, k) in [(6usize, 0x96d288usize), (8, 0x9688d0), (0xa, 0x9688d0), (0xc, 0x9688d0)] {
            let x = cqc9_w(ln() + o) as i32;
            let v = (x as f64 * cqc9_f(0x9688a0) + cqc9_f(k)) * cqc9_f(0x967880); // cqc9: a confiança de volta
            cqc9_pw(ln() + o, cqc9_ftol(v) as i16);
        }
    }
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    let band: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x5986b0usize);
    let r = if cqc9_selecao(clube) == 0 {
        let fin = financas(cqc9_d(0xae2a38), clube);
        if fin != 0 { band(fin) } else { 0 }
    } else {
        0
    };
    cqc9_pb(ln() + 1, r);
    let rep = || cqc9_w(clube + 0x80);
    let tec = cqc9_d(clube + 0xcf);
    if tec != 0 && cqc9_d(tec + 0x69) != 0 {
        let fx = cqc9_d(tec + 0x69);
        let a = cqc9_w(fx + 0xa);
        if a < rep() {
            let t = cqc9_ftol((rep() as i32 - a as i32) as f64 * cqc9_f(0x969518));
            cqc9_pw(fx + 0xa, a.wrapping_sub(t as i16));
            let fx = cqc9_d(cqc9_d(clube + 0xcf) + 0x69);
            if cqc9_w(fx + 0xa) > 0x2710 {
                cqc9_pw(fx + 0xa, 0x2710);
            }
            let fx = cqc9_d(cqc9_d(clube + 0xcf) + 0x69);
            let c = cqc9_w(fx + 0xc);
            let t = cqc9_ftol((rep() as i32 - c as i32) as f64 * cqc9_f(0x96d280));
            cqc9_pw(fx + 0xc, c.wrapping_sub(t as i16));
            let fx = cqc9_d(cqc9_d(clube + 0xcf) + 0x69);
            if cqc9_w(fx + 0xc) > 0x2710 {
                cqc9_pw(fx + 0xc, 0x2710);
            }
        }
    }
    let tec = cqc9_d(clube + 0xcf);
    if tec != 0 && cqc9_d(tec + 0x69) != 0 {
        let fx = cqc9_d(tec + 0x69);
        let a = cqc9_w(fx + 0xa);
        if rep() > a {
            let t = cqc9_ftol((rep() as i32 - a as i32) as f64 * cqc9_f(0x969518));
            cqc9_pw(fx + 0xa, a.wrapping_sub(t as i16));
        }
    }
    let tec = cqc9_d(clube + 0xcf);
    if tec != 0 && cqc9_i(tec) >= cqc9_i(0xae2364).wrapping_sub(0x10) && cqc9_b(tec + 0x59) < 0x14 {
        cqc9_pb(tec + 0x59, cqc9_u(tec + 0x59).wrapping_add(1));
    }
    cqc9_pb(ln() + 2, cqc9_u(ln() + 2) & 0xfe);
    cqc9_pb(ln() + 4, 0);
    cqc9_pb(ln() + 3, cqc9_u(ln() + 3) & 0xe1);
    cqc9_pb(ln() + 2, cqc9_u(ln() + 2) & 0xdf);
    cqc9_pd(ln() + 0x28, 0);
    cqc9_pb(ln() + 5, 0);
    cqc9_pb(ln() + 3, cqc9_u(ln() + 3) & 0xfe);
    cqc9_pb(ln() + 0x40, cqc9_u(ln() + 0x40) & 7);
    cqc9_pw(ln() + 0x12, 0);
    cqc9_pw(ln() + 0x14, 0);
    cqc9_pw(ln() + 0x16, 0);
    let multas = cqc9_d(0xae2a3c);
    if multas != 0 {
        let f: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x5a5930usize);
        f(multas, clube);
    }
}

// ponytail: acrescenta na lista das saídas (0x68d0d0, aloca), posta as notícias (0x6a44a0, 0x6a4a20) e sorteia: não roda
// duas vezes. Só a original; a cópia leva os ramos e se prova pela carreira e pelas mutações.
unsafe extern "thiscall" fn comparar_set_season_expectations(this: usize, clube: usize, noticia: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(%%I0%%)(this, clube, noticia)
}

/// A pessoa de índice `k` ([0xae23bc] + k × 0x6e).
unsafe fn manager_manager_staff_by_index(k: i32) -> usize {
    cqc9_d(0xae23bc).wrapping_add((k as usize).wrapping_mul(0x6e))
}

/// O byte +0xb do estado da pessoa de índice `k` ([0xae2d0c] + k × 0x4f).
unsafe fn staff_state_byte_b(k: i32) -> u8 {
    cqc9_u(cqc9_d(0xae2d0c).wrapping_add((k as usize).wrapping_mul(0x4f)) + 0xb)
}

/// A nota do candidato (0x69d1b0: thiscall, 4: pessoa, clube, 0, 5; original).
unsafe fn score_candidate(this: usize, p: usize, clube: usize) -> i32 {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> i32 = core::mem::transmute(0x69d1b0usize);
    f(this, p, clube, 0, 5)
}

/// 0x69f810 (thiscall, 4: pessoa, clube e a nota mínima em double, em dois dwords).
unsafe fn is_staff_suitable(this: usize, p: usize, clube: usize, minimo: f64) -> u8 {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, f64) -> u8 = core::mem::transmute(0x69f810usize);
    f(this, p, clube, minimo)
}

/// 0x4cf6d0 (thiscall em [0xae1050], 1): a pessoa tem ficha de contrato.
unsafe fn loan_contract_of(p: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf6d0usize);
    f(cqc9_d(0xae1050), p)
}

/// 0x69fc60 (thiscall, 1): a pessoa já está numa lista de vagas.
unsafe fn is_vacancy_candidate(this: usize, p: usize) -> u8 {
    let f: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x69fc60usize);
    f(this, p)
}

/// O item de 0x26 bytes da vaga (o que a original monta em [esp+0x1c]): +0 achou um livre, +1 0, +2 o id dele, +6 o
/// índice do escolhido, +0xa o id do técnico que saiu, +0xe o id do clube, +0x12 a nota anterior, +0x16 a data da
/// chegada e +0x1e a data do anúncio (8 bytes cada, 0x54a110).
struct ManagerVacancy([u8; 0x28]);

impl ManagerVacancy {
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

/// { 0x69ba50 } (thiscall, ret 8, manager_manager.cpp, quadro SEH sem destrutor no caminho normal):
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
unsafe extern "thiscall" fn add_job_vacancy(this: usize, clube: usize, saiu: usize) {
    // ponytail: a nota anterior (+0x12) fica sem valor na original quando o escolhido é o auxiliar ou o de +0x19f (sobra de
    // pilha); aqui 0, um valor que o jogo também produz
    let mut v = ManagerVacancy([0; 0x28]);
    let data_padrao: unsafe extern "thiscall" fn(*mut u8) -> usize = core::mem::transmute(0x54a110usize);
    data_padrao(v.0.as_mut_ptr().add(0x16));
    data_padrao(v.0.as_mut_ptr().add(0x1e));
    let mut melhor: i32 = -1_000_000;
    let segunda: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x5409b0usize);
    if cqc9_selecao(clube) == 0 && cqc9_d(clube + 0x57) == 0 && cqc9_w(clube + 0x80) < 0x1388 {
        return;
    }
    if cqc9_selecao(clube) != 0 {
        if segunda(clube) != 0 && cqc9_i(cqc9_d(clube + 0x53) + 0x88) <= 0x3e8 {
            return;
        }
        if segunda(clube) == 0 && cqc9_i(cqc9_d(clube + 0x53) + 0x88) <= 0x64 {
            return;
        }
    }
    v.d(0xe, cqc9_i(clube));
    v.d(6, -1);
    let ln = || cqc9_linha(this, clube);
    for o in [6usize, 8, 0xa, 0xc] {
        cqc9_pw(ln() + o, 0x1388);
    }
    cqc9_pb(ln(), 0);
    cqc9_pw(ln() + 0xe, 0x10);
    for o in [0x10usize, 0x12, 0x14, 0x16] {
        cqc9_pw(ln() + o, 0);
    }
    for m in [0xfeu8, 0xfd, 0xfb, 0xf7, 0xef] {
        cqc9_pb(ln() + 2, cqc9_u(ln() + 2) & m);
    }
    cqc9_pb(ln() + 4, 0);
    cqc9_pb(ln() + 3, cqc9_u(ln() + 3) & 0xe1);
    for m in [0xdfu8, 0xbf, 0x7f] {
        cqc9_pb(ln() + 2, cqc9_u(ln() + 2) & m);
    }
    cqc9_pd(ln() + 0x28, 0);
    cqc9_pb(ln() + 5, 0);
    cqc9_pb(ln() + 3, cqc9_u(ln() + 3) & 0xfe);
    cqc9_pb(ln() + 0x40, cqc9_u(ln() + 0x40) & 7);
    v.d(0xa, if saiu != 0 { cqc9_i(saiu) } else { -1 });
    // o melhor de todos
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let dirigivel: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(0x544820usize);
    let mut k = 0i32;
    while k < cqc9_i(0xae2364) {
        let p = manager_manager_staff_by_index(k);
        let ultimos = cqc9_i(0xae2364) - 0x10;
        let ok = 'c: {
            if cqc9_d(p + 0x69) == 0 || (k >= ultimos && cqc9_u(p + 0x5f) == 0) || saiu == p {
                break 'c false;
            }
            if k >= ultimos && cqc9_selecao(clube) != 0 && cqc9_d(p + 0x1a) != cqc9_d(clube + 0x53)
                && cqc9_w(cqc9_d(p + 0x69) + 0xc) < 0xfa0
            {
                break 'c false;
            }
            if k >= cqc9_i(0xae2364) - 0x10 && dirigivel(clube) == 0 {
                break 'c false;
            }
            if staff_state_byte_b(k) & 0x30 != 0 || loan_contract_of(p) != 0 || is_vacancy_candidate(this, p) != 0 {
                break 'c false;
            }
            let (n1, n2) = (cqc9_d(clube + 0x53), cqc9_d(p + 0x1a));
            if n1 == 0 || n2 == 0 || (n1 != n2 && cqc9_w(cqc9_d(p + 0x69) + 0xa) < 0xea6) {
                break 'c false;
            }
            if cqc9_u(n1 + 0x75) != cqc9_u(n2 + 0x75)
                && (cqc9_w(cqc9_d(p + 0x69) + 0xc) < 0x109a || cqc9_w(clube + 0x80) < 0x128e)
            {
                break 'c false;
            }
            if is_staff_suitable(this, p, clube, 3.5) == 0 {
                break 'c false;
            }
            if cqc9_d(p + 0x61) != 0 && cqc9_d(p + 0x39) != 0 && matches!(cqc9_u(p + 0x3d), 0xb | 0xc | 0xd | 0xf) {
                let c = contrato(cqc9_d(0xae1050), p);
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
            let s = score_candidate(this, p, clube);
            if s > melhor {
                crate::comparison::branch("cqc_9 candidato melhor");
                v.d(6, k); // cqc9: o candidato escolhido
                v.d(0x12, melhor);
                melhor = s;
            }
        }
        k += 1;
    }
    if v.rd(6) == -1 {
        let ajudante = cqc9_d(clube + 0xd3);
        let lista = cqc9_d(clube + 0x19f);
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
                        let fmt: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
                        fmt(0xde1f64, if segunda(clube) != 0 { 0xa04d8c } else { 0xa04d28 }, clube + 4);
                        return;
                    }
                }
                v.d(6, i);
                if staff_state_byte_b(i) & 0x30 != 0 || i >= cqc9_i(0xae2364) - 0x10 {
                    v.d(6, -1);
                    continue;
                }
                let p = manager_manager_staff_by_index(i);
                if cqc9_d(p + 0x69) == 0 || !matches!(cqc9_u(p + 0x3d), 0 | 0xb | 0xc | 5 | 8 | 0xf | 7 | 6) {
                    v.d(6, -1);
                    continue;
                }
                if loan_contract_of(p) != 0 || is_vacancy_candidate(this, manager_manager_staff_by_index(v.rd(6))) != 0 {
                    v.d(6, -1);
                    continue;
                }
                let q = manager_manager_staff_by_index(v.rd(6));
                if cqc9_d(q + 0x24) != 0 && cqc9_u(q + 0x28) == 5 {
                    v.d(6, -1);
                    continue;
                }
                if cqc9_selecao(clube) == 0 {
                    let q = manager_manager_staff_by_index(v.rd(6));
                    let c = cqc9_d(q + 0x39);
                    if c != 0 {
                        let na = cqc9_d(c + 0x53);
                        let d57 = cqc9_d(c + 0x57);
                        let forte = (na != 0 && cqc9_u(na + 0x11c) != 0 && d57 != 0
                            && cqc9_d(cqc9_d(0xadadfc).wrapping_add((cqc9_i(d57) as usize).wrapping_mul(4))) != 0)
                            || cqc9_w(c + 0x80) > 0x1964;
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
    let grande = cqc9_selecao(clube) != 0 || (cqc9_w(clube + 0x80) > 0x1194 && {
        let r = cqc9_sortear(0x7d0);
        cqc9_w(clube + 0x80) as i32 > r + 0x1194
    });
    if grande {
        let mut melhor: i32 = -1_000_000;
        let mut k = 0i32;
        while k < cqc9_i(0xae2364) {
            let r = cqc9_sortear(cqc9_i(0xae2364));
            let p = manager_manager_staff_by_index(r);
            // ponytail: o estado lido é o do contador `k`, não o da pessoa sorteada, como na original
            if cqc9_d(p + 0x69) != 0 && saiu != p && staff_state_byte_b(k) & 0x30 == 0
                && is_staff_suitable(this, p, clube, f64::from_bits(0x4009_3333_3333_3333)) != 0
                && (cqc9_i(p) < cqc9_i(0xae2364) - 0x10 || cqc9_u(p + 0x5f) != 0)
            {
                let ate: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x54cdb0usize);
                if ate(0xae2c90, cqc9_d(p + 0x3e) as u32, cqc9_d(p + 0x42) as u32) >= 0x96 && is_vacancy_candidate(this, p) == 0 {
                    let s = score_candidate(this, p, clube);
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
    if cqc9_i(manager_manager_staff_by_index(v.rd(6))) >= ultimos && saiu != 0 && cqc9_i(saiu) < ultimos {
        let rejeitar: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x547e70usize);
        rejeitar(saiu, clube, 1);
    }
    // as datas: +0x16 a chegada (dia +0x16, ano +0x18), +0x1e o anúncio (dia +0x1e, ano +0x20)
    let (dia0, ano0) = (cqc9_w(0xdd6ec8), cqc9_w(0xdd6eca) as u16);
    let (dia, ano) = (cqc9_w(0xae2c90), cqc9_w(0xae2c92) as u16);
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
        let q = manager_manager_staff_by_index(v.rd(6));
        let c = cqc9_d(q + 0x39);
        let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
        if c != 0 && humano(c, 0, 1, 0, 0) != 0 && cqc9_u(manager_manager_staff_by_index(v.rd(6)) + 0x3d) != 5 {
            let oferta: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x6a0540usize);
            oferta(this, manager_manager_staff_by_index(v.rd(6)), clube);
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
    let lista = cqc9_d(this + 8);
    let cursor = cqc9_d(lista + 0xa);
    let acrescentar: unsafe extern "thiscall" fn(usize, *mut u8, u32) = core::mem::transmute(0x68d0d0usize);
    acrescentar(lista, v.0.as_mut_ptr(), 0x26);
    cqc9_pd(cqc9_d(this + 8) + 0xa, cursor as u32);
    let tecnico: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53ca90usize);
    if tecnico(clube) == 0 && dirigivel(clube) != 0 && cqc9_selecao(clube) == 0 {
        let ativo: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5fd210usize);
        let quer: unsafe extern "thiscall" fn(usize, u32, usize) -> u8 = core::mem::transmute(0x5fea50usize);
        let candidatar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x69ca00usize);
        let mut i = cqc9_i(0xae2364) - 0x10;
        while i < cqc9_i(0xae2364) {
            let p = manager_manager_staff_by_index(i);
            if cqc9_u(p + 0x5f) != 0 && ativo(cqc9_d(0xb63c98), p) != 0 && quer(cqc9_d(0xb63c98), 1, p) != 0 {
                let c = cqc9_d(p + 0x39);
                if c == 0 || cqc9_w(c + 0x80) < cqc9_w(clube + 0x80) {
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
unsafe extern "thiscall" fn comparar_add_job_vacancy(this: usize, clube: usize, saiu: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I1%%)(this, clube, saiu)
}

/// { 0x6a0540 } (thiscall, ret 8, manager_manager.cpp, quadro SEH sem destrutor no caminho normal):
/// `MANAGER_MANAGER::post_job_offer_news(pessoa, clube)`: com a pessoa empregada (+0x39) e o clube, a partir do começo
/// do jogo ([0xdd6ec8]), a notícia 0x1775 (0x77bbe0) com o id da pessoa, os três nomes (+0x33 de +4, +8, +0xc), o id do
/// clube onde ela está, o do clube que oferece e o cargo (+0x3d), ao técnico do clube dela (0x7861e0).
unsafe extern "thiscall" fn post_job_offer_news(_this: usize, pessoa: usize, clube: usize) {
    if pessoa == 0 || cqc9_d(pessoa + 0x39) == 0 || clube == 0 {
        return;
    }
    let dia = cqc9_w(0xdd6ec8);
    let ano = cqc9_w(0xdd6eca) as u16;
    let hoje = cqc9_w(0xae2c92) as u16;
    if ano > hoje || (ano == hoje && dia > cqc9_w(0xae2c90)) {
        return;
    }
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let ao_tecnico: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    crate::comparison::branch("cqc_9 noticia da oferta");
    let mut local = [0u32; 0x3a]; // a notícia local (0xe8 bytes)
    let n = local.as_mut_ptr() as usize;
    construir(n, 0x1775, 0);
    gravar(n, 0, cqc9_d(pessoa) as u32);
    gravar(n, 1, cqc9_d(cqc9_d(pessoa + 4) + 0x33) as u32);
    gravar(n, 2, cqc9_d(cqc9_d(pessoa + 8) + 0x33) as u32);
    gravar(n, 3, cqc9_d(cqc9_d(pessoa + 0xc) + 0x33) as u32);
    gravar(n, 4, cqc9_d(cqc9_d(pessoa + 0x39)) as u32);
    gravar(n, 5, cqc9_d(clube) as u32);
    gravar(n, 6, cqc9_b(pessoa + 0x3d) as i32 as u32);
    ao_tecnico(cqc9_d(0xb7431c), n, cqc9_d(pessoa + 0x39));
}

// ponytail: a notícia vai ao técnico (aloca): só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_post_job_offer_news(this: usize, pessoa: usize, clube: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I2%%)(this, pessoa, clube)
}
