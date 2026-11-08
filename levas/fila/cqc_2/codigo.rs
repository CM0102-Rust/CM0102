// ---------- %%LEVA%%a leva: o teto salarial: o salário máximo que o clube oferece e as notícias das obras e das obras recusadas (fin_prm.cpp) ----------

unsafe fn cqc2_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqc2_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cqc2_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqc2_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn read_i64(a: usize) -> i64 {
    core::ptr::read_unaligned(a as *const i64)
}

/// `v` × a constante f64 em `k`, truncado (_ftol).
unsafe fn mul_exe_f64(v: i32, k: usize) -> i32 {
    match_pl_ftol(v as f64 * read_f64_at(k))
}

/// O clube humano (0x600a50: 5 pushes; add esp, 0x14 -> cdecl(clube, a, 1, 0, 0)).
unsafe fn is_human_club_with(clube: usize, a: u32) -> u32 {
    let f: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    f(clube, a, 1, 0, 0)
}

/// A faixa de riqueza (0x5986b0: mov ecx, finanças -> thiscall).
unsafe fn cqc2_faixa(fin: usize) -> i8 {
    let f: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x5986b0usize);
    f(fin) as i8
}

/// A seleção (0x53b900: push; add esp, 4 -> cdecl).
unsafe fn fin_prm_is_national_team(clube: usize) -> bool {
    let f: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    f(clube) != 0
}

/// O valor do contrato da pessoa (0x4d0c20: mov ecx, [0xae1050]; push pessoa -> thiscall, ret 4), com sinal.
unsafe fn contract_wage(pessoa: usize) -> i32 {
    let f: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x4d0c20usize);
    f(cqc2_d(0xae1050), pessoa) as i32
}

/// O bônus de quem paga mais (o trecho 0x596c0d..0x597238): clube da Inglaterra, Itália ou Espanha, reputação acima
/// de 4750, faixa 0 a 2, pelo fator salarial `w` (a palavra, com sinal) e pela marca +0x82.
unsafe fn big_spender_bonus(clube: usize, band: i8, w: i32) -> i32 {
    let nac = cqc2_d(clube + 0x53);
    if nac == 0 {
        return 0;
    }
    let n = cqc2_i(nac);
    if n != cqc2_i(0x9cf2e4) && n != cqc2_i(0x9cf360) && n != cqc2_i(0x9cf488) {
        return 0;
    }
    if cqc2_w(clube + 0x80) <= 0x128e || !(band == 2 || band == 1 || band == 0) {
        return 0;
    }
    let w16 = w as i16;
    let x = w16 as i32;
    let (base, oito) = if cqc2_b(clube + 0x82) != 0 { (0x4e2, 0x2710) } else { (0x2ee, 0x1770) };
    if w16 < 0x1676 || band == 0 {
        x.wrapping_sub(base).wrapping_mul(6)
    } else if band == 1 {
        x.wrapping_mul(8).wrapping_sub(oito)
    } else {
        x.wrapping_sub(base).wrapping_mul(10)
    }
}

/// O teto do técnico humano pelas reputações (+0xb, +0xd) e pela experiência (+5) do não-jogador `np`, pela idade
/// (+0x18) e pela marca +0x22 da pessoa e pelo cargo (o trecho 0x597405..0x597566).
unsafe fn human_manager_wage_cap(pessoa: usize, np: usize, cargo: i8) -> i32 {
    let idade = cqc2_b(pessoa + 0x18) as i8;
    let marca = cqc2_b(pessoa + 0x22) != 0;
    let cx = cqc2_w(np + 0xb);
    let tecnico = cargo < 4 && cargo != 0 && cargo != 7;
    if cx < 0xea6 {
        let ax = cqc2_w(np + 5);
        if ax < 0x3c {
            if idade <= 0x17 { 0x1388 } else { 0x1388 + 0x9c4 }
        } else if ax < 0x64 {
            if (cargo >= 4 || cargo == 0) && idade < 0x18 { 0x2710 } else { 0x3a98 }
        } else {
            0x61a8
        }
    } else if cx < 0x1482 {
        if cqc2_w(np + 5) >= 0x64 { 0x7530 } else { 0x7530 - 5000 }
    } else if cx < 0x1c52 {
        if idade > 0x22 && cqc2_w(np + 5) < 0x78 {
            if tecnico { 0x927c } else { 0x7ef4 }
        } else {
            let cx2 = cqc2_w(np + 5);
            if cx2 < 0x8c && cqc2_w(np + 0xd) < 0xea6 {
                0x9c40 + if marca { 0x1388 } else { 0 }
            } else if tecnico {
                if marca { 0xe09c } else if cx2 >= 0x8c { 0x13880 } else { 0x13880 - 15000 }
            } else {
                0xafc8
            }
        }
    } else if cqc2_w(np + 0xd) < 0x1a5e {
        0x186a0 + if marca { 0x61a8 } else { 0 }
    } else {
        0x2ab98
    }
}

/// { 0x5968d0 } (thiscall, ret 8, fin_prm.cpp; os asserts citam finance.CPP; 0x14ac bytes com x87):
/// `Finance::get_max_wage(pessoa, cargo)`, pelas propostas de contrato e de transferência, pelas telas e pela folha
/// salarial (0x598370, sem pessoa e cargo −1). Clube de +0x64 = 3: 0. O fator salarial pela reputação (/ 50, mais pela
/// faixa de riqueza 0x5986b0 e pela tabela de salários da nação: 0x9c8498 nações pequenas, 0x9c8548 médias, 0x9c85f8
/// França e clubes de +0x82, 0x9c86a8 as outras) × 47,6 (teto 10000). Com pessoa: o valor do contrato no clube
/// (0x4d0c20, até 2 × o teto +0x14 das finanças, no mínimo ele; jogador limitado pelo tipo +0x3d), o bônus de quem
/// paga mais, a reputação² × 0,25 × 0,0001 + o piso +0x10, as regras dos cargos 1 a 3, o teto do humano, a nação
/// (clube basco 0x53d7f0, o mercado pela tabela do gerente de transferências, o contrato perto do fim 0x54cdb0), o
/// presidente (+0xbf: o gosto 0x547810 e o favorito 0x547790, o perfil +0xf, o técnico +0xcf) e os cargos 1 a 6. Sem
/// pessoa: o fator da nação (+0x85 / 20 × o tipo +0x7e, pelas nações do tipo 1), a liga (+0x57 e +0x5b, o tamanho
/// +0x69 ao cubo), a tabela por (fator / 10) menos (+0xc − (fator³ × nação / divisor + saldo × 1,3) / 52), com o piso
/// 0x9c83e8 ou 0x9c8338, a faixa e a nação. Depois o clube de +0x64 1 e 2, o mínimo da liga + 100 e o menor dos dois.
unsafe extern "thiscall" fn get_max_wage(fin: usize, pessoa: usize, cargo_arg: u32) -> i32 {
    let mut cargo = cargo_arg as u8;
    let clube = cqc2_d(0xae23b4).wrapping_add((cqc2_i(fin + 8) as u32).wrapping_mul(0x245) as usize);
    let verba = match_pl_ftol(read_i64(fin) as f64 * read_f64_at(0x968de8));
    let mut liga: i32 = 100;
    let mut fator: f64 = 0.25; // [esp + 0x3c]: 0x3fd00000_00000000
    let mut contrato: usize = 0;
    if cqc2_b(clube + 0x64) == 3 {
        return 0;
    }
    let rep = cqc2_w(clube + 0x80);
    let mut di: i32 = rep as i32 / 50;
    let band: i8 = if fin_prm_is_national_team(clube) { 0 } else { cqc2_faixa(fin) };
    let nac = cqc2_d(clube + 0x53);
    let e = |g: usize| cqc2_i(nac) == cqc2_i(g);
    let tabela: usize = if nac == 0 || e(0x9cf510) || e(0x9cf2dc) || e(0x9cf3e4) || e(0x9cf224) {
        0x9c8498
    } else if [0x9cf36c, 0x9cf4a8, 0x9cf2ac, 0x9cf430, 0x9cf408, 0x9cf4f8, 0x9cf434, 0x9cf2c0, 0x9cf2fc, 0x9cf45c, 0x9cf314].iter().any(|&g| e(g)) {
        0x9c8548
    } else if e(0x9cf300) || (cqc2_b(clube + 0x82) != 0 && band != 2 && band != 1) {
        0x9c85f8
    } else {
        0x9c86a8
    };
    let padrao = tabela == 0x9c86a8;
    match band {
        0 => {
            if rep > 0x1676 {
                di = if padrao { (rep as i32 - 0x186a) / 25 + 0x73 } else { di + 5 };
            }
        }
        1 => {
            di = if rep <= 0x1482 { di + 5 } else if padrao { (rep as i32 - 0x1482) / 25 + 0x69 } else { di + 10 };
        }
        2 => {
            di = if rep <= 0x128e { di + 10 } else if padrao { (rep as i32 - 0x128e) / 25 + 0x5f } else { di + 0xf };
        }
        _ => {}
    }
    let mut s = match_pl_ftol((di as i16) as f64 * read_f64_at(0x96a5e0));
    if (s as i16) > 0x2710 {
        s = 0x2710;
    }
    if (di as i16) > 0xd2 {
        di = 0xd2;
    }
    let humano = is_human_club_with(clube, 0);
    if humano != 0 && cargo == 0xff {
        let com_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
        contrato = com_clube(cqc2_d(0xae1050), pessoa, clube);
        if contrato != 0 {
            cargo = cqc2_b(contrato + 0x4f) >> 4;
        }
    }
    let mut r: i32; // [esp + 0x18]: o teto que sai
    let b20: i32;
    if pessoa != 0 {
        r = 0;
        if cqc2_d(pessoa + 0x39) == clube {
            r = contract_wage(pessoa);
            let t = cqc2_i(fin + 0x14).wrapping_mul(2);
            if r > t {
                r = t;
            }
        }
        if cqc2_i(fin + 0x14) > r {
            r = cqc2_i(fin + 0x14);
        }
        if cqc2_d(pessoa + 0x69) != 0 && cqc2_d(pessoa + 0x61) == 0 {
            let cap = match (cqc2_b(pessoa + 0x3d) as i8 as i32).wrapping_sub(5) {
                0 | 1 | 2 => 0x88b8,
                3 => 0x4e20,
                4 => 0x5dc,
                5 => 0x3e8,
                _ => 0x2ee,
            };
            if r > cap {
                r = cap;
            }
        }
        b20 = big_spender_bonus(clube, band, s);
    } else {
        let mut f = fator;
        if nac != 0 {
            let mut dl = cqc2_b(nac + 0x7e) as i8;
            if dl < 1 {
                dl = 1;
            } else if dl > 3 {
                dl = 3;
            }
            let bl = cqc2_b(nac + 0x85) as i8;
            let q = bl as i32 as f64 / (dl as i32 * 20) as f64;
            f = q;
            if dl == 1 {
                let m = if e(0x9cf2e4) || e(0x9cf360) || e(0x9cf488) || e(0x9cf36c) {
                    read_f64_at(0x967890)
                } else if e(0x9cf4f8) {
                    read_f64_at(0x969040)
                } else {
                    let (k, tres) = if e(0x9cf314) || e(0x9cf300) || e(0x9cf45c) || e(0x9cf338) {
                        (0x96a5d0, false)
                    } else if e(0x9cf260) || e(0x9cf218) || e(0x9cf4a8) || e(0x9cf244) || e(0x9cf434) {
                        (0x96a5d8, false)
                    } else if e(0x9cf2c0) || e(0x9cf408) || e(0x9cf2fc) {
                        (0x969510, true)
                    } else {
                        (0x968e80, false)
                    };
                    let d16 = di as i16 as i32;
                    if d16 <= (bl as i32).wrapping_mul(10) {
                        read_f64_at(k)
                    } else {
                        let div = if bl < 1 { 1 } else { bl as i32 };
                        let mut m = d16 as f64 / div as f64 * read_f64_at(0x967880) * read_f64_at(k);
                        if tres && !(m <= read_f64_at(0x969040)) {
                            m = read_f64_at(0x969040);
                        } else if !(m <= read_f64_at(0x967890)) {
                            m = read_f64_at(0x967890);
                        }
                        m
                    }
                };
                f = q * m;
            }
            fator = f;
            let lc = cqc2_d(clube + 0x57);
            if lc != 0 {
                let d2 = cqc2_d(clube + 0x5b);
                let c16 = cqc2_w(lc + 0x69);
                let cubo = |x: i32| x.wrapping_mul(x).wrapping_mul(x);
                let v = if d2 != 0 && cqc2_w(d2 + 0x69) > c16 {
                    let t1 = match_pl_ftol(cubo(c16 as i32) as f64 * f * read_f64_at(0x96a5c8));
                    let t2 = match_pl_ftol(cubo(cqc2_w(d2 + 0x69) as i32) as f64 * f * read_f64_at(0x968928));
                    t2.wrapping_sub(t1)
                } else {
                    match_pl_ftol(cubo(c16 as i32) as f64 * f * read_f64_at(0x968e18))
                };
                liga = if v < 0xc8 { 0xc8 } else { v };
            }
        }
        let d16 = di as i16 as i32;
        let mut q10 = (200 - d16) / 10;
        if q10 < 1 {
            q10 = 1;
        }
        let a = d16 as f64;
        let t = match_pl_ftol(a * a * a * f / q10 as f64);
        crate::comparison::branch("cqc_2 teto pela tabela");
        let menos = cqc2_i(fin + 0xc).wrapping_sub(t.wrapping_add(verba) / 52); // cqc2: a folha e a verba por semana
        let idx = ((d16 / 10) * 8) as isize as usize;
        let vv = read_i64(tabela.wrapping_add(idx)).wrapping_sub(menos as i64) as f64 * f;
        let piso = read_i64((if padrao { 0x9c83e8usize } else { 0x9c8338 }).wrapping_add(idx)) as f64 * f;
        r = if vv <= piso { match_pl_ftol(piso) } else { match_pl_ftol(vv) };
        if !fin_prm_is_national_team(clube) {
            match cqc2_faixa(fin) {
                1 => r = mul_exe_f64(r, 0x9689b8),
                2 => r = mul_exe_f64(r, 0x9689c0),
                _ => {}
            }
            if nac != 0 {
                let ita_esp = e(0x9cf360) || e(0x9cf488);
                if ita_esp {
                    if rep > 0x1e46 {
                        r = mul_exe_f64(r, 0x9689b8);
                    }
                } else if e(0x9cf2e4) && rep > 0x1e46 {
                    r = mul_exe_f64(r, 0x968fa0);
                }
                if cqc2_b(clube + 0x82) == 0 && rep > 0x1e46 && ita_esp {
                    r = mul_exe_f64(r, 0x9689b8);
                }
            }
        }
        b20 = big_spender_bonus(clube, band, s);
    }
    if pessoa != 0 {
        let repf = rep as i32 as f64;
        let t = repf * repf * fator * read_f64_at(0x96a5c0) + cqc2_i(fin + 0x10) as f64;
        let v = contract_wage(pessoa);
        s = if (v as f64) < t {
            crate::comparison::branch("cqc_2 teto pela injury_word_0c");
            match_pl_ftol(repf * repf * fator * read_f64_at(0x96a5c0) + cqc2_i(fin + 0x10) as f64) // cqc2: a reputação ao quadrado
        } else {
            contract_wage(pessoa)
        };
        let np = cqc2_d(pessoa + 0x61);
        let c = cargo as i8;
        if np != 0 && cqc2_w(np + 0xb) > 0x1482 && (c == 3 || c == 2 || c == 1) && (cqc2_b(pessoa + 0x18) as i8) < 0x20 {
            let np2 = || cqc2_d(pessoa + 0x61);
            if c == 3 {
                let bf = b20 as f64;
                let lim = bf * read_f64_at(0x9689b0);
                if (r as f64) < lim {
                    r = if is_human_club_with(clube, 0) == 0 && np2() != 0 && cqc2_w(np2() + 0xb) > 0x1c52 { match_pl_ftol(bf * read_f64_at(0x9689d8)) } else { match_pl_ftol(lim) };
                }
            } else if c == 2 {
                let lim = b20 as f64 * read_f64_at(0x9689d8);
                if (r as f64) < lim {
                    r = if is_human_club_with(clube, 0) == 0 && np2() != 0 && cqc2_w(np2() + 0xb) > 0x1e46 { b20 } else { match_pl_ftol(lim) };
                }
            } else if r < b20 {
                r = b20;
            }
        }
        if is_human_club_with(clube, 0) != 0 {
            let np = cqc2_d(pessoa + 0x61);
            if np != 0 {
                let teto = human_manager_wage_cap(pessoa, np, cargo as i8);
                if teto < r {
                    r = teto.wrapping_sub(match_pl_ftol(r.wrapping_sub(teto) as f64 * read_f64_at(0x969518)));
                }
            }
        }
        let np = cqc2_d(pessoa + 0x61);
        if np != 0 {
            if nac != 0 && (e(0x9cf360) || e(0x9cf2e4) || e(0x9cf314) || e(0x9cf300) || e(0x9cf488)) {
                let basco: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53d7f0usize);
                if basco(clube) != 0 {
                    s = mul_exe_f64(s, 0x9689b8);
                }
                let tm = cqc2_d(0xde1f54);
                if tm != 0 {
                    let rr = cqc2_w(clube + 0x80);
                    let div: i32 = if rr < 0x3e8 { 0x3e8 } else { rr as i32 };
                    let tab = cqc2_d(cqc2_d(tm + 0x856) + 0xdc);
                    let x = cqc2_w(tab.wrapping_add((cqc2_i(clube) as usize).wrapping_mul(9)) + 5) as i32;
                    let mut m = x as f64 / div as f64;
                    if !(read_f64_at(0x967890) < m || read_f64_at(0x967890) == m) {
                        m = read_f64_at(0x967890);
                    } else if read_f64_at(0x968978) < m {
                        m = read_f64_at(0x968978);
                    }
                    s = match_pl_ftol(s as f64 * m);
                }
                // o contrato perto do fim (0x54cdb0: lea ecx, contrato + 0x2d; push [0xae2c94]; push [0xae2c90] ->
                // thiscall, ret 8), menos de 200 dias
                let perto = || {
                    let dias: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x54cdb0usize);
                    contrato != 0 && cqc2_d(pessoa + 0x39) == clube && dias(contrato + 0x2d, cqc2_d(0xae2c90) as u32, cqc2_d(0xae2c94) as u32) < 0xc8
                };
                let rr = cqc2_w(clube + 0x80);
                let k: Option<usize> = if rr > 0x186a {
                    let np = cqc2_d(pessoa + 0x61);
                    if perto() {
                        let np = cqc2_d(pessoa + 0x61);
                        if cqc2_w(np + 0xd) > 0x1482 {
                            Some(0x968978)
                        } else {
                            let ax = cqc2_w(np + 0xb);
                            let cl = cqc2_b(contrato + 0x4f) & 0xf0;
                            if ax > 0x1c52 {
                                Some(0x9689c0)
                            } else if (cl == 0x30 && ax > 0x1482) || cl == 0x10 || cl == 0x20 {
                                Some(0x9689b8)
                            } else {
                                None
                            }
                        }
                    } else if cqc2_w(np + 0xd) > 0x1482 {
                        Some(0x9689c0)
                    } else if cqc2_w(np + 0xb) > 0x1a5e {
                        Some(0x9689b8)
                    } else {
                        None
                    }
                } else if rr > 0x109a {
                    if perto() {
                        let cx = cqc2_w(cqc2_d(pessoa + 0x61) + 0xb);
                        let al = cqc2_b(contrato + 0x4f) & 0xf0;
                        if cx > 0x1a5e {
                            Some(0x9689b8)
                        } else if (al == 0x30 && cx > 0x1482) || al == 0x10 || al == 0x20 {
                            Some(0x968f88)
                        } else {
                            None
                        }
                    } else if cqc2_w(cqc2_d(pessoa + 0x61) + 0xb) > 0x1e46 {
                        Some(0x968f88)
                    } else {
                        None
                    }
                } else {
                    None
                };
                if let Some(k) = k {
                    s = mul_exe_f64(s, k);
                }
            }
            let pres = cqc2_d(clube + 0xbf);
            if pres != 0 && cqc2_d(pres + 0x69) != 0 && (cqc2_w(clube + 0x80) as i32) < cqc2_w(cqc2_d(pessoa + 0x61) + 0xb) as i32 + 0x4e2 {
                crate::comparison::branch("cqc_2 presidente");
                s = match_pl_ftol(((cqc2_b(pres + 0x57) as i8 as i32) as f64 * read_f64_at(0x967898) + read_f64_at(0x9689d8)) * s as f64); // cqc2: o presidente
                // 0x547810 (push pres; push pessoa; add esp, 8 -> cdecl), 0x547790 (push clube; push pessoa; add esp, 8 -> cdecl)
                let gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547810usize);
                let favorito: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547790usize);
                if gosta(pessoa, pres) != 0 || favorito(pessoa, clube) != 0 {
                    s = match_pl_ftol(((cqc2_b(cqc2_d(clube + 0xbf) + 0x57) as i8 as i32) as f64 * read_f64_at(0x967898) + read_f64_at(0x967890)) * s as f64);
                }
                let pres = cqc2_d(clube + 0xbf);
                let pc = cqc2_d(pres + 0x69);
                let perfil = cqc2_b(pc + 0xf) as i8;
                if perfil < 6 {
                    let t = cqc2_d(clube + 0xcf);
                    let k = if t != 0 && (cqc2_b(t + 0x57) as i8) > 0xf && cqc2_d(t + 0x69) != 0 && cqc2_w(cqc2_d(t + 0x69) + 0xa) > 0x1c52 { 0x9689c0 } else { 0x9689b8 };
                    s = mul_exe_f64(s, k);
                } else if perfil > 0xf {
                    s = mul_exe_f64(s, 0x9689d0);
                }
                if cqc2_b(clube + 0x82) == 0 && (cqc2_b(pc + 0x20) as i8) > 0xf && (cqc2_b(pres + 0x57) as i8) > 0xf {
                    let rr = cqc2_w(clube + 0x80);
                    let k = if rr >= 0xabe && cqc2_w(cqc2_d(pessoa + 0x61) + 0xb) >= 0x1c52 {
                        if rr < 0x1676 { 0x9689c0 } else { 0x96a5a8 }
                    } else {
                        0x9689b8
                    };
                    s = mul_exe_f64(s, k);
                }
            }
            let c = cargo as i8;
            let np = cqc2_d(pessoa + 0x61);
            match c {
                1 => s = mul_exe_f64(s, 0x968928),
                2 => {
                    if is_human_club_with(clube, 1) == 0 && cqc2_w(np + 0xb) > 0x1a5e {
                        s = mul_exe_f64(s, 0x968928);
                    } else {
                        s = match_pl_ftol(s as f64);
                        let ok = is_human_club_with(clube, 1) != 0 || {
                            let np = cqc2_d(pessoa + 0x61);
                            np != 0 && cqc2_w(np + 0xb) < 0x1e46
                        };
                        if ok && s > 0x14c08 {
                            s = match_pl_ftol(s.wrapping_sub(0x14c08) as f64 * read_f64_at(0x968918) + read_f64_at(0x96a5a0));
                        }
                    }
                }
                3 => {
                    if is_human_club_with(clube, 1) == 0 && cqc2_w(np + 0xb) > 0x1a5e {
                        s = mul_exe_f64(s, 0x968928);
                    } else {
                        s = mul_exe_f64(s, 0x9689d0);
                        if s > 0xd6d8 {
                            s = 0xd6d8;
                        }
                    }
                }
                4 => {
                    let k = if is_human_club_with(clube, 1) == 0 && cqc2_w(np + 0xb) > 0x1a5e { 0x9689d0 } else { 0x969040 };
                    s = mul_exe_f64(s, k);
                    if s > 0x927c {
                        s = 0x927c;
                    }
                }
                5 => {
                    s = if s > 0xc80 { match_pl_ftol(s.wrapping_sub(0xc80) as f64 * read_f64_at(0x968e80) + read_f64_at(0x96a598)) } else { mul_exe_f64(s, 0x968e48) };
                    if s > 0x30d4 {
                        s = 0x30d4;
                    }
                }
                6 => {
                    s = if s > 0xc80 { match_pl_ftol(s.wrapping_sub(0xc80) as f64 * read_f64_at(0x968f20) + read_f64_at(0x96a590)) } else { mul_exe_f64(s, 0x968968) };
                    if s > 0x203a {
                        s = 0x203a;
                    }
                }
                _ => {}
            }
        }
    } else {
        let x = (s as i16) as i32 as f64;
        s = match_pl_ftol(x * x * fator * read_f64_at(0x96a5c0) + cqc2_i(fin + 0x10) as f64);
        if s < b20 {
            s = match_pl_ftol(s as f64 * read_f64_at(0x969040) + b20 as f64 * read_f64_at(0x9689e0));
        }
        if !fin_prm_is_national_team(clube) && nac != 0 {
            if e(0x9cf314) || e(0x9cf300) {
                match cqc2_faixa(fin) {
                    1 => s = mul_exe_f64(s, 0x9689b8),
                    2 => s = mul_exe_f64(s, 0x9689c0),
                    _ => {}
                }
            } else if e(0x9cf360) || e(0x9cf2e4) || e(0x9cf488) {
                match cqc2_faixa(fin) {
                    0 => {
                        if cqc2_w(clube + 0x80) > 0x109a {
                            s = mul_exe_f64(s, 0x968fa0);
                        }
                    }
                    1 => s = mul_exe_f64(s, 0x9689b8),
                    2 => s = mul_exe_f64(s, 0x9689c0),
                    _ => {}
                }
            }
        }
    }
    let tipo = cqc2_b(clube + 0x64);
    if tipo == 1 {
        if s < 0xfa {
            s = 0xfa;
        }
        if cqc2_w(clube + 0x80) < 0x8ca && s > 0x834 {
            s = match_pl_ftol((s as f64 - read_f32_at(0x96a5b8) as f64) * read_f64_at(0x968918) + read_f64_at(0x96a5b0));
        }
    } else if tipo == 2 {
        s = mul_exe_f64(s, 0x969040);
        if cqc2_w(clube + 0x80) < 0xabe && s > 0x4b0 {
            s = match_pl_ftol((s as f64 - read_f32_at(0x96a588) as f64) * read_f64_at(0x968918) + read_f64_at(0x96a580));
        }
        if s < 0x96 {
            s = 0x96;
        }
    }
    if s < liga.wrapping_add(100) {
        s = liga.wrapping_add(100);
    }
    if liga > r {
        return liga;
    }
    if s >= r { r } else { s }
}

/// O comparador do teto salarial: pela devolução (a rotina só lê; as chamadas de dentro também).
unsafe extern "thiscall" fn compare_get_max_wage(fin: usize, pessoa: usize, cargo: u32) -> i32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> i32>(%%I0%%)(fin, pessoa, cargo);
    let n = get_max_wage(fin, pessoa, cargo);
    SCOREBOARD.record(r == n, || format!("get_max_wage(fin {:x}, pessoa {:x}, cargo {}) original={} rust={}", fin, pessoa, cargo as u8 as i8, r, n));
    r
}

/// { 0x59ffb0 } (thiscall, ret 0xc, fin_prm.cpp; quadro SEH só da notícia local, sem destrutor no caminho normal):
/// `FinanceTable::news_ground_works(clube, valor, tipo)`, pelo fim de temporada do clube (0x59cfa0: compra do estádio,
/// aquecimento, expansão e lugares, centro de treino) e pelos eventos da semana (0x59ed70). A notícia 0x177d (0x77bbe0:
/// lea ecx, local; push 0; push 0x177d -> thiscall, ret 8) com o id do clube, o valor e o tipo (byte com sinal; 0x785790:
/// lea ecx, local; 2 pushes -> thiscall, ret 8) entregue ao técnico do clube (0x7861e0: ecx = [0xb7431c]; push clube;
/// push &local -> thiscall, ret 8).
unsafe extern "thiscall" fn news_ground_works(_this: usize, clube: usize, valor: u32, tipo: u32) {
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let mut n = [0u8; 0xe0];
    let p = n.as_mut_ptr() as usize;
    construir(p, 0x177d, 0);
    gravar(p, 0, cqc2_d(clube) as u32);
    gravar(p, 1, valor);
    gravar(p, 2, tipo as u8 as i8 as i32 as u32);
    entregar(cqc2_d(0xb7431c), p, clube);
}

/// O comparador da notícia das obras: só a original (ponytail: a notícia vai para a caixa do técnico humano e não pode
/// ser entregue duas vezes; com técnico da máquina não deixa nada).
unsafe extern "thiscall" fn compare_news_ground_works(this: usize, clube: usize, valor: u32, tipo: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32)>(%%I1%%)(this, clube, valor, tipo)
}

/// { 0x59fae0 } (thiscall, ret 0xc, fin_prm.cpp; quadro SEH só da notícia local, sem destrutor no caminho normal):
/// `FinanceTable::news_request_refused(clube, assunto, motivo)`, pelo fim de temporada do clube (0x59cfa0: o pedido de
/// estádio ou de centro de treino recusado). Sem clube, nada. A notícia 0x1788 (0x77bbe0) com o assunto (palavra com
/// sinal) no dado 1 e no dado 2 o motivo 2 ou o sorteio(2) (0x90cfc0: push 2; add esp, 4 -> cdecl; a palavra com
/// sinal), entregue ao técnico do clube (0x7861e0).
unsafe extern "thiscall" fn news_request_refused(_this: usize, clube: usize, assunto: u32, motivo: u32) {
    if clube == 0 {
        return;
    }
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let sortear: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let mut n = [0u8; 0xe0];
    let p = n.as_mut_ptr() as usize;
    construir(p, 0x1788, 0);
    gravar(p, 1, assunto as u16 as i16 as i32 as u32);
    let v = if motivo as u16 == 2 { 2 } else { sortear(2) };
    gravar(p, 2, v as u16 as i16 as i32 as u32);
    entregar(cqc2_d(0xb7431c), p, clube);
}

/// O comparador da notícia da obra recusada: só a original (ponytail: entrega ao técnico humano e sorteia; não pode
/// rodar duas vezes).
unsafe extern "thiscall" fn compare_news_request_refused(this: usize, clube: usize, assunto: u32, motivo: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32)>(%%I2%%)(this, clube, assunto, motivo)
}
