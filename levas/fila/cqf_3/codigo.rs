// ---------- %%LEVA%%a leva: o contrato que o clube oferece a uma pessoa (contract_manager.cpp) ----------

unsafe fn cqf3_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqf3_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cqf3_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqf3_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqf3_pd(a: usize, v: u32) {
    core::ptr::write_unaligned(a as *mut u32, v)
}

unsafe fn cqf3_pw(a: usize, v: u16) {
    core::ptr::write_unaligned(a as *mut u16, v)
}

unsafe fn cqf3_pb(a: usize, v: u8) {
    *(a as *mut u8) = v
}

/// Os três bônus do contrato (+0x10, +0x14, +0x18): os que não são −1 (> −1) × `f`, truncados.
unsafe fn scale_contract_bonuses(c: usize, f: f64) {
    for k in 0..3 {
        let a = c + 0x10 + 4 * k;
        let v = cqf3_i(a);
        if v > -1 {
            cqf3_pd(a, match_pl_ftol(v as f64 * f) as u32);
        }
    }
}

/// { 0x4cda70 } (thiscall, ret 0x1c, contract_manager.cpp, quadro SEH do STAFF_CONTRACT local sem destrutor; x87):
/// `contract_manager_offer_contract(pessoa, clube, saida, vaga, fator, modo)`: o contrato que o clube oferece. Sem
/// clube, o do contrato da pessoa (o registro de 79 bytes em [0xae2d0c] → o contrato de 0x50 bytes em [[0xae1050]],
/// guardado em [0xae1080]); nenhum: 0. Seleção (0x53b900), regen marcado (bits 0x30 do +0xb do registro), o próprio clube
/// fora dos tipos 11..15 (0x53b620 de +0x3d) com a lista do clube em 2 ou sem comissão e dirigível (0x544820), clube
/// humano (0x600a50) ou transferência futura (0x8cca10): 0. O contrato desejado (0x85fc50 no local, com o estatuto
/// 0x849f00, o 3 de habilidade (+0xb da ficha) acima de 5250 vira 2); a devolução igual a [0x9689a0]: 0. O salário (+8)
/// e o valor (+0xc) × fator. Com luvas (+0x1d): o mínimo da multa (+0x21) pelo valor da oferta da vaga (0x8e7b50: em
/// faixas de 50 mil, 1 milhão e 5 milhões) ou 50 mil sem oferta (100 mil abaixo da 1ª faixa); sem luvas, com +0x1f ou
/// +0x1c, 50 mil. O teto: o salário máximo do clube (0x5968d0, com o cargo do nibble alto de +0x4f) × (fator + 1) × 0,5
/// no estatuto 11, senão o do pessoal (0x597db0). O contrato atual com o clube (0x4cf5f0). Jogador (+0x61): a idade em
/// dias (0x54cdb0) × k menos o peso da ficha (o tipo 0x14 fixo, senão +0x3c, +0x34 e o +0x5b da pessoa): acima de
/// [0x968df8] (ou contrato atual com o nibble 6) o fim no ano que vem (valor × [0x9689c0]; bônus × [0x968928]); acima de
/// [0x968df0], daqui a 2 anos (bônus × [0x968de8]); acima de [0x968de0] com a reputação do clube acima da da ficha (+0xd)
/// × [0x968978], o ano pela idade (bônus × [0x968940]). Valor acima de 10 mil e idade acima de [0x968dd8]: o total
/// (semanas até o fim × valor) contra a habilidade e a idade (pow 0x945ff0): reduz o valor a cada 52 semanas a menos ou
/// baixa o teto para total / semanas. Contrato atual que acaba depois: 0; lesão (0x62ed70) acima de 70 dias com o
/// [0xb63d28]: 0. Com [0xae233c] = 1, no máximo 5 anos. Valor acima do teto × [0x968dc0]: 0; acima do teto: o salário e
/// os bônus × valor / max(teto, 1) e o valor = teto. As queixas limpas (0x867520). Com `saida`: copia o contrato e 1;
/// senão, no modo 2, ou com contrato atual no modo 1 que acaba antes de hoje + 150 dias, a proposta (0x4cc490); 1.
#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn contract_manager_offer_contract(this: usize, pessoa: usize, clube_arg: usize, saida: usize, vaga: i32, fator: f64, modo: u32) -> u32 {
    let construir: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x85e410usize);
    let is_national_team: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let tipo_11_15: unsafe extern "C" fn(i8) -> u32 = core::mem::transmute(0x53b620usize);
    let dirigivel: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(0x544820usize);
    let humano: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let futura: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x8cca10usize);
    let vaga_da: unsafe extern "thiscall" fn(usize, i32) -> usize = core::mem::transmute(0x8c61e0usize);
    let estatuto_de: unsafe extern "thiscall" fn(usize, usize, usize, *mut u8) -> u8 = core::mem::transmute(0x849f00usize);
    let desejado: unsafe extern "thiscall" fn(usize, usize, usize, i32, u32, u32, u32, u32) -> i32 = core::mem::transmute(0x85fc50usize);
    let valor_oferta: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> i32 = core::mem::transmute(0x8e7b50usize);
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    let teto_max: unsafe extern "thiscall" fn(usize, usize, u32) -> i32 = core::mem::transmute(0x5968d0usize);
    let teto_pessoal: unsafe extern "thiscall" fn(usize, i8) -> i32 = core::mem::transmute(0x597db0usize);
    let contrato_com: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let dias_ate: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x54cdb0usize);
    let lesao: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x62ed70usize);
    let limpar: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x867520usize);
    let copiar_data: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(0x417c10usize);
    let mais: unsafe extern "thiscall" fn(usize, *mut u8, u32) -> usize = core::mem::transmute(0x54c5b0usize);
    let propor: unsafe extern "thiscall" fn(usize, usize, i32, u32, u32, u32, u32, u32) = core::mem::transmute(0x4cc490usize);

    // o STAFF_CONTRACT local (0x50 bytes; a original o tem em [esp+0x38])
    let mut local = [0u32; 0x14];
    let c = local.as_mut_ptr() as usize;
    construir(c);
    let mut clube = clube_arg;
    if clube == 0 {
        let k = if pessoa == 0 {
            0
        } else if (cqf3_d(pessoa) as i32) < cqf3_i(0xae2364) {
            let idx = cqf3_i(cqf3_d(0xae2d0c).wrapping_add(cqf3_d(pessoa).wrapping_mul(0x4f)));
            if idx < 0 { 0 } else { (idx as usize).wrapping_mul(0x50).wrapping_add(cqf3_d(cqf3_d(0xae1050))) }
        } else {
            0
        };
        cqf3_pd(0xae1080, k as u32);
        if k == 0 {
            return 0;
        }
        clube = cqf3_d(0xae23b4).wrapping_add(cqf3_d(k + 4).wrapping_mul(0x245));
        if clube == 0 {
            return 0;
        }
    }
    if is_national_team(clube) != 0 {
        return 0;
    }
    if cqf3_b(cqf3_d(0xae2d0c).wrapping_add(cqf3_d(pessoa).wrapping_mul(0x4f)) + 0xb) & 0x30 != 0 {
        return 0;
    }
    if tipo_11_15(cqf3_b(pessoa + 0x3d) as i8) == 0 && clube == cqf3_d(pessoa + 0x39) {
        let lista = cqf3_d(cqf3_d(cqf3_d(0xde1f54) + 0x856) + 0xdc);
        if cqf3_b(lista.wrapping_add(cqf3_d(clube).wrapping_mul(9)) + 4) == 2 || (cqf3_d(clube + 0xcf) == 0 && dirigivel(clube) != 0) {
            return 0;
        }
    }
    if humano(clube, 0, 1, 0, 0) != 0 {
        return 0;
    }
    if futura(cqf3_d(0xde1f54), pessoa, 0) != 0 {
        return 0;
    }
    cqf3_pd(c + 4, cqf3_d(clube) as u32);
    let mut oferta = 0usize;
    if vaga != -1 {
        let p = vaga_da(cqf3_d(0xde1f54), vaga);
        if p != 0 {
            oferta = cqf3_d(p);
        }
    }
    let mut cargo: u8 = 0;
    let mut est = estatuto_de(cqf3_d(cqf3_d(0xde1f54) + 0x856), pessoa, clube, &mut cargo);
    if est == 3 && cqf3_w(cqf3_d(pessoa + 0x61) + 0xb) > 0x1482 {
        est = 2;
    }
    // ponytail: o estatuto e o cargo vão em dwords com os bytes altos de sobra da pilha na original; a 0x85fc50 e a
    // 0x597db0 leem o byte
    let r = desejado(c, pessoa, clube, vaga, est as u32, cargo as u32, 0, 0);
    if r as f64 == read_f64_at(0x9689a0) {
        return 0;
    }
    cqf3_pd(c + 8, match_pl_ftol(cqf3_i(c + 8) as f64 * fator) as u32);
    cqf3_pd(c + 0xc, match_pl_ftol(cqf3_i(c + 0xc) as f64 * fator) as u32);
    if cqf3_b(c + 0x1d) != 0 {
        let mut piso: i32 = 0;
        if oferta != 0 {
            let v = valor_oferta(oferta, 0, 0, 0, 0, 1);
            if v > 0x4c4b40 {
                piso = match_pl_ftol((v - 0x4c4b40) as f64 * read_f64_at(0x968928) + read_f64_at(0x968e28));
            } else if v > 0xf4240 {
                piso = match_pl_ftol((v - 0xf4240) as f64 * read_f64_at(0x968940) + read_f64_at(0x968e20));
            } else if v > 0xc350 {
                piso = match_pl_ftol((v - 0xc350) as f64 * read_f64_at(0x968e18) + read_f64_at(0x9688f0));
            } else if cqf3_i(c + 0x21) < 0x186a0 {
                piso = 0x186a0;
            }
        } else if cqf3_i(c + 0x21) < 0xc350 {
            piso = 0xc350;
        }
        if cqf3_i(c + 0x21) < piso {
            cqf3_pd(c + 0x21, piso as u32);
        }
    } else if (cqf3_b(c + 0x1f) != 0 || cqf3_b(c + 0x1c) != 0) && cqf3_i(c + 0x21) < 0xc350 {
        cqf3_pd(c + 0x21, 0xc350);
    }
    let mut teto: f64 = if cargo == 0xb {
        let nibble = (cqf3_b(c + 0x4f) >> 4) as u32;
        let f = financas(cqf3_d(0xae2a38), clube);
        let m = teto_max(f, pessoa, nibble);
        (fator + read_f64_at(0x967890)) * read_f64_at(0x968968) * m as f64 // cqf3: o teto do estatuto 11
    } else {
        let f = financas(cqf3_d(0xae2a38), clube);
        teto_pessoal(f, cargo as i8) as f64
    };
    let atual = contrato_com(this, pessoa, clube);
    let mut ano = cqf3_w(0xae2c92) as u16;
    let ficha = cqf3_d(pessoa + 0x61);
    if ficha != 0 {
        let dias = dias_ate(0xae2c90, cqf3_d(pessoa + 0x10) as u32, cqf3_d(pessoa + 0x14) as u32);
        let mut x = dias as f64 * read_f64_at(0x968e10);
        if cqf3_b(ficha + 0xf) == 0x14 {
            x -= read_f64_at(0x968950);
        } else {
            let s = cqf3_b(ficha + 0x3c) as i8 as i32 + cqf3_b(ficha + 0x34) as i8 as i32 + cqf3_b(pessoa + 0x5b) as i8 as i32;
            x -= (s as f64 * read_f64_at(0x968a80) - read_f64_at(0x968e08)) * read_f64_at(0x968e00);
        }
        let fim = |c: usize| core::ptr::read_unaligned((c + 0x2f) as *const u16) as u32;
        if x > read_f64_at(0x968df8) || (atual != 0 && cqf3_b(atual + 0x4f) & 0xf0 == 0x60) {
            if fim(c) > ano as u32 + 1 {
                crate::comparison::branch("cqf_3 ends next year");
                cqf3_pw(c + 0x2f, ano.wrapping_add(1)); // cqf3: o fim no ano que vem
                cqf3_pd(c + 0xc, match_pl_ftol(cqf3_i(c + 0xc) as f64 * read_f64_at(0x9689c0)) as u32);
            }
            scale_contract_bonuses(c, read_f64_at(0x968928));
        } else if x > read_f64_at(0x968df0) {
            if fim(c) > ano as u32 + 2 {
                cqf3_pw(c + 0x2f, ano.wrapping_add(2));
                cqf3_pd(c + 0xc, match_pl_ftol(cqf3_i(c + 0xc) as f64 * read_f64_at(0x9689c0)) as u32);
            }
            scale_contract_bonuses(c, read_f64_at(0x968de8));
        } else if x > read_f64_at(0x968de0) && cqf3_w(clube + 0x80) as f64 > cqf3_w(ficha + 0xd) as f64 * read_f64_at(0x968978) {
            let k = match_pl_ftol(x + read_f64_at(0x968968)) as i8;
            if fim(c) as i32 > (ano as i32 - k as i32) + 0x22 {
                cqf3_pw(c + 0x2f, ano.wrapping_sub(k as i16 as u16).wrapping_add(0x22));
                cqf3_pd(c + 0xc, match_pl_ftol(cqf3_i(c + 0xc) as f64 * read_f64_at(0x9689c0)) as u32);
            }
            scale_contract_bonuses(c, read_f64_at(0x968940));
        }
        if cqf3_i(c + 0xc) > 0x2710 && x > read_f64_at(0x968dd8) {
            let dias_fim = dias_ate(c + 0x2d, cqf3_d(0xae2c90) as u32, cqf3_d(0xae2c94) as u32);
            let mut semanas = dias_fim / 7;
            let mut total = semanas.wrapping_mul(cqf3_i(c + 0xc));
            let mut a = game_pow(cqf3_w(ficha + 0xb) as f64, read_f64_at(0x968950)) * read_f64_at(0x968dd0);
            if !(a < read_f64_at(0x968dc8)) {
                a = 8000.0;
            }
            let mut b = game_pow(x - read_f64_at(0x968dd8), read_f64_at(0x968950));
            if !(b > read_f64_at(0x967890)) {
                b = 1.0;
            }
            let limit = match_pl_ftol(game_pow(a, read_f64_at(0x968950)) / b);
            let reduz = (limit as f64 * read_f64_at(0x968940)) < total as f64;
            let mut feito = false;
            if reduz {
                while semanas > 0x68 {
                    semanas -= 0x34;
                    cqf3_pd(c + 0xc, match_pl_ftol(cqf3_i(c + 0xc) as f64 * read_f64_at(0x9689c0)) as u32);
                    total = semanas.wrapping_mul(cqf3_i(c + 0xc));
                    if limit >= total {
                        feito = true;
                        break;
                    }
                }
            }
            if !feito && total > limit {
                let q = limit / semanas;
                if (q as f64) < teto {
                    crate::comparison::branch("cqf_3 cap by total");
                    teto = q as f64; // cqf3: o teto pelo total
                }
            }
        }
        ano = cqf3_w(0xae2c92) as u16;
    }
    if atual != 0 {
        if core::ptr::read_unaligned((atual + 0x2f) as *const u16) >= core::ptr::read_unaligned((c + 0x2f) as *const u16) {
            return 0;
        }
        if cqf3_d(0xb63d28) != 0 {
            let l = lesao(cqf3_d(0xb63d28), pessoa);
            if l != 0 && cqf3_w(lesao(cqf3_d(0xb63d28), pessoa) + 0x2c) > 0x46 {
                return 0;
            }
            ano = cqf3_w(0xae2c92) as u16;
        }
    }
    if cqf3_b(0xae233c) == 1 && core::ptr::read_unaligned((c + 0x2f) as *const u16) as u32 > ano as u32 + 5 {
        cqf3_pw(c + 0x2f, ano.wrapping_add(5));
    }
    let w = cqf3_i(c + 0xc) as f64;
    if w > teto * read_f64_at(0x968dc0) {
        return 0;
    }
    if w > teto {
        crate::comparison::branch("cqf_3 above the cap");
        let den = if read_f64_at(0x967890) > teto { read_f64_at(0x967890) } else { teto };
        let razao = w / den;
        cqf3_pd(c + 8, match_pl_ftol(cqf3_i(c + 8) as f64 * razao) as u32);
        scale_contract_bonuses(c, razao);
        cqf3_pd(c + 0xc, match_pl_ftol(teto) as u32); // cqf3: o valor no teto
    }
    limpar(c, 0);
    if saida != 0 {
        for k in (0..0x20).step_by(4) {
            cqf3_pd(saida + k, cqf3_d(c + k) as u32);
        }
        cqf3_pb(saida + 0x20, cqf3_b(c + 0x20));
        cqf3_pd(saida + 0x21, cqf3_d(c + 0x21) as u32);
        let mut tmp = [0u8; 8];
        copiar_data(saida + 0x25, tmp.as_mut_ptr(), c + 0x25);
        copiar_data(saida + 0x2d, tmp.as_mut_ptr(), c + 0x2d);
        // o +0x35 pelos campos de bits da original: 6 bits, 1 bit e 1 bit
        let cl = cqf3_b(c + 0x35);
        let dl = cqf3_b(saida + 0x35);
        let al = ((((cl << 2) as i8 >> 2) as u8 ^ dl) & 0x3f) ^ dl;
        let d2 = (((((cl << 1) as i8 >> 1) as u8 ^ al) & 0xc0) ^ cl ^ al) & 0x7f;
        cqf3_pd(saida + 0x36, cqf3_d(c + 0x36) as u32);
        cqf3_pb(saida + 0x35, d2 ^ cl);
        cqf3_pb(saida + 0x3a, cqf3_b(c + 0x3a));
        cqf3_pw(saida + 0x3b, cqf3_w(c + 0x3b) as u16);
        for k in [0x3dusize, 0x3e, 0x3f] {
            cqf3_pb(saida + k, cqf3_b(c + k));
        }
        cqf3_pd(saida + 0x40, cqf3_d(c + 0x40) as u32);
        cqf3_pb(saida + 0x44, cqf3_b(c + 0x44));
        cqf3_pd(saida + 0x45, cqf3_d(c + 0x45) as u32);
        cqf3_pb(saida + 0x49, cqf3_b(c + 0x49));
        cqf3_pd(saida + 0x4a, cqf3_d(c + 0x4a) as u32);
        cqf3_pb(saida + 0x4e, cqf3_b(c + 0x4e));
        cqf3_pb(saida + 0x4f, cqf3_b(c + 0x4f));
        return 1;
    }
    let modo = modo as u8;
    let enviar = if modo == 2 {
        true
    } else if atual == 0 || modo != 1 {
        false
    } else {
        let mut d = [0u8; 8];
        let p = mais(0xae1068, d.as_mut_ptr(), 0x96);
        let (dia, ano_d) = (cqf3_w(p), core::ptr::read_unaligned((p + 2) as *const u16));
        let ano_a = core::ptr::read_unaligned((atual + 0x2f) as *const u16);
        ano_d < ano_a || (ano_d == ano_a && dia < cqf3_w(atual + 0x2d))
    };
    if enviar {
        // ponytail: o cargo (+0x3d) vai num dword com os bytes altos de sobra do edx na original; a 0x4cc490 lê o byte
        propor(this, c, vaga, u32::MAX, cqf3_b(pessoa + 0x3d) as u32, 0, 1, 0);
    }
    1
}

/// Comparador da 0x4cda70: com `saida`, lado a lado (a saída, [0xae1080] e o estado do sorteador; a cópia sem contar os
/// sorteios); sem `saida` (pode propor o contrato, que aloca), só a original.
#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn compare_contract_manager_offer_contract(this: usize, pessoa: usize, clube: usize, saida: usize, vaga: i32, fator: f64, modo: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, i32, f64, u32) -> u32>(%%I0%%);
    if saida == 0 {
        return o(this, pessoa, clube, saida, vaga, fator, modo); // ponytail: só a original
    }
    let est = crate::rng::STATE_REGIONS;
    let regioes = [(saida, 0x50usize), (0xae1080usize, 4usize), est[0], est[1]];
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&regioes,
        || o(this, pessoa, clube, saida, vaga, fator, modo),
        || crate::rng::without_counting(|| contract_manager_offer_contract(this, pessoa, clube, saida, vaga, fator, modo)));
    // ponytail: o STAFF_CONTRACT local da original não tem valor em +0x38..+0x39 (do dword +0x36), +0x44 e no nibble
    // baixo de +0x4f (nem a 0x85e410 nem a 0x85fc50 os gravam: sobra de pilha, copiada para a saída); a cópia deixa 0
    // (montada em memória zerada). A comparação ignora esses bits.
    let sem_sobra = |m: &crate::comparison::MemorySnapshot| {
        let mut v = m.0[0].1.clone();
        v[0x38] = 0;
        v[0x39] = 0;
        v[0x44] = 0;
        v[0x4f] &= 0xf0;
        v
    };
    let igual = igual || (sem_sobra(&mo) == sem_sobra(&mr) && mo.0[1..] == mr.0[1..]);
    SCOREBOARD.record(igual && r == n, || format!(
        "contract_manager_offer_contract(pessoa {:#x}, clube {:#x}, vaga {}, fator {}, modo {}): original {} rust {}, memória {:x?} x {:x?}",
        pessoa, clube, vaga, fator, modo, r, n, mo, mr));
    r
}
