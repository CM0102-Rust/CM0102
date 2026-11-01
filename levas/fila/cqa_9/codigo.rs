// ---------- %%LEVA%%a leva: o valor do jogador, a comparação da moral de dois jogadores e as marcas de elenco da tela do clube (contract_manager.cpp) ----------

unsafe fn cqa9_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqa9_b(a: usize) -> i8 {
    *(a as *const i8)
}

unsafe fn cqa9_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O clube de id `id` ([0xae23b4], 0x245 bytes).
unsafe fn cqa9_clube(id: usize) -> usize {
    cqa9_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// O contrato principal da pessoa (índice +0 do registro de 79 bytes em [0xae2d0c], registros de 0x50 bytes em
/// [[0xae1050]]), ou 0: sem pessoa, fora das pessoas ([0xae2364]) ou sem índice.
unsafe fn cqa9_contrato_principal(pessoa: usize) -> usize {
    if pessoa == 0 || cqa9_d(pessoa) as i32 >= cqa9_d(0xae2364) as i32 {
        return 0;
    }
    let i = cqa9_d(cqa9_d(0xae2d0c).wrapping_add(cqa9_d(pessoa).wrapping_mul(0x4f))) as i32;
    if i < 0 { 0 } else { (i as usize).wrapping_mul(0x50).wrapping_add(cqa9_d(cqa9_d(0xae1050))) }
}

/// O mínimo com a cláusula de rescisão (+0x21 do contrato, se 0x867b50 a diz ativa) e o truncamento (_ftol).
unsafe fn cqa9_fim(contrato: usize, w: f64) -> i32 {
    let clausula: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x867b50usize);
    if contrato != 0 && clausula(contrato) != 0 {
        let cl = core::ptr::read_unaligned((contrato + 0x21) as *const i32) as f64;
        return ftol(if !(w >= cl) { w } else { cl });
    }
    ftol(w)
}

/// { 0x4d0c60 } (thiscall, ret 0x14, contract_manager.cpp): `CONTRACT_MANAGER::calc_player_value(pessoa, clube, modo,
/// habilidade, _)`. O contrato principal vai para [0xae1080] e o contrato com o clube dele (0x4cf5f0, no this de
/// entrada). Sem ficha (+0x61): a compensação (0x8d0780 com 1) pelo clube do contrato, ou 0. A base e a idade do auge
/// pelas posições da ficha (+0x15/+0x14/+0x13/+0x12 ≥ 15: 13, 11, 10, 9 milhões; goleiro +0xf = 20: 6 milhões, 33; senão
/// 8 milhões; auge 28 ou 29). O fator da idade (auge − idade, com a correção acima de [0x9688a0]) × as notas da ficha
/// (+0x28, +0x27, +0x43, +0x36, +0x2f, fora dos goleiros) e +0x2d²; a reputação (+9) limita o fator. A habilidade: sem
/// clube, a atual (+0xb); com clube, a vista pelo técnico (0x6e86d0 se `habilidade` = −1, com a reputação vista 0x5407e0)
/// × 50 + a reputação + até 5000 × fator, × [0x968f08], entre 1 e 12000; o potencial (+0xd) acima de 5000 (e 8000) antes
/// do auge a puxa. Valor = habilidade² × base × a tabela 0x9a7f30 (pela idade, com a experiência +0x22/+0x1a) × a nação
/// do clube (+0x53: +0x7e e +0x85 ou a divisão +0x69; sem clube, sorteio(3) + 1), com piso. `modo`: a nação [0x9cf434]
/// e a cláusula. Senão: os dias de contrato (com a lei Bosman 0x8d0a70) e as nações [0x9cf434], [0x9cf300], [0x9cf484];
/// no fim a cláusula de rescisão ativa (0x867b50) limita.
unsafe extern "thiscall" fn valor_do_jogador(this: usize, pessoa: usize, clube: usize, modo: u32, habilidade: u32, _x: u32) -> i32 {
    let c1080 = cqa9_contrato_principal(pessoa);
    *(0xae1080 as *mut usize) = c1080;
    let clube_do_contrato = if c1080 == 0 { 0 } else { cqa9_clube(cqa9_d(c1080 + 4)) };
    let com_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let contrato = com_clube(this, pessoa, clube_do_contrato);
    let ficha = cqa9_d(pessoa + 0x61);
    if ficha == 0 {
        if clube_do_contrato == 0 {
            return 0;
        }
        // a original põe ecx = [0xde1f54] (a cópia Rust é stdcall): thiscall com 3 na pilha serve aos dois
        let compensacao: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> i32 = core::mem::transmute(0x8d0780usize);
        return compensacao(cqa9_d(0xde1f54), pessoa, clube_do_contrato, 1);
    }
    let (base, auge): (f64, i8) = if cqa9_b(ficha + 0x15) >= 0xf {
        (13_000_000.0, 0x1c) // cqa9: a base de quem joga na frente
    } else if cqa9_b(ficha + 0x14) >= 0xf {
        (11_000_000.0, 0x1d)
    } else if cqa9_b(ficha + 0x13) >= 0xf {
        (10_000_000.0, 0x1d)
    } else if cqa9_b(ficha + 0x12) >= 0xf {
        (9_000_000.0, 0x1d)
    } else if cqa9_b(ficha + 0xf) == 0x14 {
        (6_000_000.0, 0x21)
    } else {
        (8_000_000.0, 0x1d)
    };
    let idade = cqa9_b(pessoa + 0x18);
    let dif = auge as i32 - idade as i32;
    let mut t = dif as f64;
    if t > f64_em(0x9688a0) {
        t = f64_em(0x968f10) - t * f32_em(0x968f18) as f64;
    }
    let a = ftol(t).wrapping_abs();
    let mut f = t * a as f64 * f32_em(0x968f08) as f64 * f64_em(0x968f00) + f64_em(0x967890);
    if cqa9_b(ficha + 0xf) < 0xf {
        let nota = |o: usize, c: usize| (cqa9_b(ficha + o) as i32 - 0xf) as f64 * f64_em(c) + f64_em(0x967890);
        let fa = nota(0x28, 0x968ef8);
        let fb = cqa9_b(ficha + 0x27) as f64 * f64_em(0x968ef0) + f64_em(0x967890);
        let fc = cqa9_b(ficha + 0x43) as f64 * f64_em(0x968e60) + f64_em(0x967890);
        let fd = nota(0x36, 0x968ef8);
        let fe = nota(0x2f, 0x968ef8);
        f = fa * (fb * f) * fc * fd * fe;
    }
    let q = cqa9_b(ficha + 0x2d) as i32;
    let fator = ((f64_em(0x968ee0) - (q * q) as f64 * f64_em(0x967888) * f64_em(0x968ee8)) * f) as f32;
    let mut r = cqa9_w(ficha + 9) as f64 * f64_em(0x968ed8) + f64_em(0x968ed0);
    if !(r >= f64_em(0x9689c0)) {
        r = f64_em(0x9689c0);
    }
    let x: f32 = if !(fator as f64 >= f64_em(0x967898)) {
        f64_em(0x967898) as f32
    } else if !(r >= fator as f64) {
        r as f32
    } else {
        fator
    };
    let mut v: u32 = if clube == 0 {
        cqa9_w(ficha + 0xb) as i32 as u32
    } else {
        let vista: unsafe extern "C" fn(usize, usize, u32) -> u16 = core::mem::transmute(0x5407e0usize);
        let tecnico = cqa9_d(clube + 0xcf);
        let rep = vista(pessoa, tecnico, 0) as i16 as i32;
        let h = if habilidade as u16 == 0xffff {
            let hab: unsafe extern "C" fn(usize, usize, usize, i8, i16) -> i16 = core::mem::transmute(0x6e86d0usize);
            hab(tecnico, pessoa, clube, 1, rep as i16) as u16
        } else {
            habilidade as u16
        };
        // só o ax de h × 50 conta (movsx depois); negativo vira 0
        let h50 = (h.wrapping_mul(50) as i16).max(0);
        let teto = if (rep as u32) < 0x1388 { rep as u32 } else { 0x1388 };
        let s = (h50 as i32).wrapping_add(rep) as u32;
        let y = ftol((s as f64 + teto as f64 * x as f64) * f32_em(0x968f08) as f64) as u32;
        if y <= 1 { 1 } else if y > 0x2ee0 { 0x2ee0 } else { y }
    };
    let pot = cqa9_w(ficha + 0xd);
    if pot > 0x1388 && (idade as i32) < auge as i32 + 4 {
        let c = (v >> 1).wrapping_add(pot as i32 as u32).wrapping_sub(0x9c4);
        if c > v {
            v = c;
        }
        if pot > 0x1f40 && idade <= auge {
            let z = pot as f64 * f64_em(0x968ec8) + (v >> 1) as f64 - f64_em(0x968ec0);
            let e = v as f64;
            v = ftol(if z > e { z } else { e }) as u32;
        }
    }
    let vv = v.wrapping_mul(v) as f64 * base * f64_em(0x968eb8);
    let mut idade_ef = idade;
    let exp = *((pessoa + 0x22) as *const u8);
    if exp != 0 && cqa9_d(pessoa + 0x1a) != 0 {
        let n = (cqa9_w(cqa9_d(pessoa + 0x1a) + 0x8e) as i32).wrapping_mul(exp as i32) / 18;
        let n = n as i16 as i32;
        let m = if n < dif { n } else { dif };
        let soma = if m < 0 { 0 } else if n >= dif { dif } else { n };
        idade_ef = idade_ef.wrapping_add(soma as i8);
    }
    let mut i = auge.wrapping_sub(idade_ef).wrapping_add(10); // cqa9: o índice da tabela pela idade
    if i < 0 {
        i = 0;
    } else if i >= 0x14 {
        i = 0x13;
    }
    let tab = *((0x9a7f30 + i as usize) as *const i8) as i32;
    let mut w = tab as f64 * vv * f64_em(0x967898);
    let c890 = f64_em(0x967890);
    let k = if clube == 0 {
        let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
        (sorteio(3) + 1) as f64
    } else {
        let div = cqa9_d(clube + 0x57);
        let nat = cqa9_d(clube + 0x53);
        if nat == 0 {
            c890
        } else {
            let mut t = cqa9_b(nat + 0x7e) as f64;
            if !(t >= c890) || t > f64_em(0x968930) {
                t = f64_em(0x968930);
            }
            if div != 0 {
                t = (t + c890) * f64_em(0x968968);
            }
            match cqa9_b(clube + 0x64) {
                2 => t = t + t,
                3 => t *= f64_em(0x968eb0),
                _ => {}
            }
            let n = if div != 0 { cqa9_w(div + 0x69) as i32 } else { cqa9_b(nat + 0x85) as i32 };
            let e = n.wrapping_mul(n).wrapping_mul(25).wrapping_add(cqa9_w(clube + 0x80) as i32);
            e as f64 / (t * f64_em(0x968908))
        }
    };
    let dd = f64_em(0x968ea8) - k;
    let mut l = c890 - dd * dd * f64_em(0x968918) * f64_em(0x968a98);
    if !(l >= f64_em(0x967880)) {
        l = f64_em(0x967880);
    } else if l > f64_em(0x9689c0) {
        l = f64_em(0x9689c0);
    }
    w = l * w;
    let k2 = k * k;
    if !(k2 * f64_em(0x968ea0) >= w) {
        w = w * f64_em(0x9689e0) + k2 * f64_em(0x968e98);
    }
    let piso = k * f64_em(0x9688e8);
    if !(w >= piso) {
        w = piso;
    }
    let nacao_e = |g: usize| -> bool {
        let nat = cqa9_d(clube + 0x53);
        clube != 0 && nat != 0 && cqa9_d(nat) == cqa9_d(g)
    };
    // a nação [0x9cf434]: abaixo de [0x968e90] a curva com piso, senão × [0x9689d8]
    let curva = |w: f64, piso: f64| -> f64 {
        if !(w >= f64_em(0x968e90)) {
            let v = (w * f64_em(0x968e88) * f64_em(0x968e48) + f64_em(0x968e80)) * w;
            if !(f64_em(0x9688e8) > v) { v } else { piso }
        } else {
            w * f64_em(0x9689d8)
        }
    };
    if modo as u8 != 0 {
        if nacao_e(0x9cf434) {
            w = curva(w, 5000.0);
        }
        return cqa9_fim(contrato, w);
    }
    let mut nacoes = true; // 0x4d1480 (as outras nações) ou 0x4d14ad (a [0x9cf434] antes)
    if contrato == 0 {
        w = f64_em(0x9689a0);
    } else {
        let bosman: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x8d0a70usize);
        if bosman(cqa9_d(0xde1f54), pessoa) != 0 {
            let ano = |a: usize| core::ptr::read_unaligned(a as *const u16) as i32;
            let mut dias = (ano(contrato + 0x2f) - ano(0xae2c92)).wrapping_mul(365) + cqa9_w(contrato + 0x2d) as i32 - cqa9_w(0xae2c90) as i32;
            if dias < 1 {
                return 0;
            }
            let mut w2 = w;
            if clube == clube_do_contrato && cqa9_b(contrato + 0x3d) > 0 {
                if *((contrato + 0x35) as *const u8) & 0x3f == 2 {
                    dias = dias + 7 * cqa9_b(contrato + 0x3d) as i32 + 0xc8;
                } else {
                    w2 = w * f64_em(0x968968);
                }
            }
            let x = dias as f64 * f64_em(0x968e78);
            if dias < 0x15e { // cqa9: o contrato curto
                w = w2 * x;
            } else {
                let mut dv = f64_em(0x968e18) - x;
                if !(dv >= c890) {
                    dv = f32_em(0x968e70) as f64;
                }
                if dv > f64_em(0x968950) {
                    dv = f32_em(0x9689a8) as f64;
                }
                w = w2 / dv;
            }
        }
        nacoes = false;
    }
    if !nacoes {
        // 0x4d14ad: valor zero (ou desordenado) vai às outras nações; sem clube, ao fim
        if !(w < f64_em(0x9689a0) || w > f64_em(0x9689a0)) {
            nacoes = true;
        } else if clube == 0 {
            return cqa9_fim(contrato, w);
        } else if nacao_e(0x9cf434) {
            w = curva(w, f64_em(0x9688e8));
            return cqa9_fim(contrato, w);
        } else {
            nacoes = true;
        }
    }
    if nacoes && clube != 0 {
        if nacao_e(0x9cf300) {
            w *= f64_em(0x9689b0);
        } else if nacao_e(0x9cf484) {
            w *= f64_em(0x9689e0);
            let c = f64_em(0x968e68);
            if w > c {
                w = ftol((w - c) * f64_em(0x968968) + c) as f64;
            }
            let c = f64_em(0x9688e0);
            if w > c {
                w = ftol((w - c) * f64_em(0x968968) + c) as f64;
            }
        }
    }
    cqa9_fim(contrato, w)
}

/// As estáticas das datas (CPQ6_ESTATICAS) e o contrato guardado em [0xae1080]: o que a original e as chamadas dela
/// gravam, com o estado do sorteador (na pilha: a rotina é quente).
fn cqa9_regioes() -> [(usize, usize); 11] {
    let (e, c) = (crate::aleatorio::REGIOES_DO_ESTADO, CPQ6_ESTATICAS);
    [e[0], e[1], (0xae1080, 4), c[0], c[1], c[2], c[3], c[4], c[5], c[6], c[7]]
}

/// Comparador: lado a lado (sem alocar) com o estado do sorteador (sem clube, sorteio(3)), o [0xae1080] e as estáticas
/// das datas da compensação; a cópia sem contar os sorteios. Quente (5,6 milhões de chamadas): sem contador.
unsafe extern "thiscall" fn comparar_valor_do_jogador(this: usize, pessoa: usize, clube: usize, modo: u32, habilidade: u32, x: u32) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) -> i32>(%%I0%%);
    let regioes = cqa9_regioes();
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&regioes, || o(this, pessoa, clube, modo, habilidade, x),
        || crate::aleatorio::sem_contar(|| valor_do_jogador(this, pessoa, clube, modo, habilidade, x)));
    PLACAR.registrar(igual && r == n, || format!("valor_do_jogador(pessoa {:x}, clube {:x}, modo {}, hab {}) original={} rust={} {:x?} x {:x?}", pessoa, clube, modo as u8, habilidade as i16, r, n, mo, mr));
    r
}

/// { 0x4cc3a0 } (stdcall, ret 0x10, contract_manager.cpp): `contract_manager_write_morale_comparison_text(a, b, saída,
/// _)`: a diferença das morais (+0x45 das fichas, a − b, em byte) vira "Superior" (até −14 e desde 13), "Better" (−13 a
/// −8 e 7 a 12), "Marginal" (−7 a −4 e 3 a 6) ou "Same" (−3 a 2) na saída (0x66f4e0 com 0xad9c64); devolve a diferença
/// (o jogo lê o al), pela tela de comparação.
unsafe extern "stdcall" fn diferenca_de_moral_em_texto(a: usize, b: usize, saida: usize, _x: u32) -> u8 {
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let dif = cqa9_b(cqa9_d(a + 0x61) + 0x45).wrapping_sub(cqa9_b(cqa9_d(b + 0x61) + 0x45));
    let f = if dif < -13 {
        0x9a8348usize
    } else if dif < -7 {
        0x9a83bc
    } else if dif < -3 {
        0x9a8394
    } else if dif < 3 {
        0x9a8370
    } else if dif < 7 {
        0x9a8394
    } else if dif < 13 {
        0x9a83bc
    } else {
        0x9a8348
    };
    formatar(saida, f, 0xad9c64usize);
    dif as u8
}

/// O texto terminado em zero em `a` (até 0x400 bytes).
unsafe fn cqa9_texto(a: usize) -> Vec<u8> {
    let s = core::slice::from_raw_parts(a as *const u8, 0x400);
    s[..s.iter().position(|&c| c == 0).unwrap_or(0x400)].to_vec()
}

/// Comparador: a original escreve; a cópia escreve de novo (sobrescreve) e o texto e a diferença têm de bater; fica o
/// texto da original.
unsafe extern "stdcall" fn comparar_diferenca_de_moral_em_texto(a: usize, b: usize, saida: usize, x: u32) -> u8 {
    let r = orig::<unsafe extern "stdcall" fn(usize, usize, usize, u32) -> u8>(%%I1%%)(a, b, saida, x);
    let tr = cqa9_texto(saida);
    let n = diferenca_de_moral_em_texto(a, b, saida, x);
    let tn = cqa9_texto(saida);
    core::ptr::copy_nonoverlapping(tr.as_ptr(), saida as *mut u8, tr.len());
    *((saida + tr.len()) as *mut u8) = 0;
    PLACAR.registrar(r == n && tr == tn, || format!("diferenca_de_moral_em_texto original={} {:?} rust={} {:?}", r as i8, String::from_utf8_lossy(&tr), n as i8, String::from_utf8_lossy(&tn)));
    r
}

/// { 0x4cc130 } (thiscall, ret 8, contract_manager.cpp): `CONTRACT_MANAGER::get_squad_flags_for_club(pessoa, clube)`,
/// pelas telas do clube: as marcas de elenco da pessoa (+0x5e sem os bits de seleção, & 0xf3), a não ser que ela esteja
/// emprestada (segundo contrato, +4 do registro de 79 bytes) a outro clube que não o `clube` nem o parceiro dele
/// (0x540a50): com parceiro, 1; senão 2 com o bit 0x40 de +0x35 do contrato principal, 1 sem. Sem clube, a caixa
/// "Error" (a original). O jogo lê o al.
unsafe extern "thiscall" fn marcas_de_elenco_para_a_tela(this: usize, pessoa: usize, clube: usize) -> u8 {
    if clube == 0 || pessoa == 0 {
        // ponytail: sem clube a caixa de erro; sem pessoa a original lê [0x5e] e cai: as duas ficam com a original
        return orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u8>(%%I2%%)(this, pessoa, clube);
    }
    let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(0x540a50usize);
    let geral = || *((pessoa + 0x5e) as *const u8) & 0xf3;
    if cqa9_d(pessoa) as i32 >= cqa9_d(0xae2364) as i32 {
        return geral();
    }
    let reg = cqa9_d(0xae2d0c).wrapping_add(cqa9_d(pessoa).wrapping_mul(0x4f));
    let i = cqa9_d(reg + 4) as i32;
    if i < 0 {
        return geral();
    }
    let c = (i as usize).wrapping_mul(0x50).wrapping_add(cqa9_d(this));
    if c == 0 || cqa9_d(c + 4) == cqa9_d(clube) {
        return geral();
    }
    let p = parceiro(clube, core::ptr::null_mut(), 1);
    if p != 0 && cqa9_d(c + 4) == cqa9_d(p) {
        return geral();
    }
    if parceiro(clube, core::ptr::null_mut(), 1) != 0 {
        return 1;
    }
    let i0 = cqa9_d(reg) as i32;
    if i0 < 0 {
        // a original lê [0x35] do contrato nulo e cai
        return orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u8>(%%I2%%)(this, pessoa, clube);
    }
    let c0 = (i0 as usize).wrapping_mul(0x50).wrapping_add(cqa9_d(this));
    (*((c0 + 0x35) as *const u8) & 0x40 != 0) as u8 + 1
}

unsafe extern "thiscall" fn comparar_marcas_de_elenco_para_a_tela(this: usize, pessoa: usize, clube: usize) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u8>(%%I2%%);
    if clube == 0 || pessoa == 0 {
        return o(this, pessoa, clube);
    }
    let r = o(this, pessoa, clube);
    let n = marcas_de_elenco_para_a_tela(this, pessoa, clube);
    PLACAR.registrar(r == n, || format!("marcas_de_elenco_para_a_tela({:x}, {:x}) original={} rust={}", pessoa, clube, r, n));
    r
}
