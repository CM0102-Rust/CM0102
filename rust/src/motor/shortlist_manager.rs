// ---------- septuagesima sexta leva: lista de observacao, bandeiras do contrato ----------

/// { 0x849ce0 } (ret 0x10, shortlist_manager.cpp): a entrada da lista da pessoa (40 entradas de
/// 37 bytes no ponteiro em [this+0xdc] + id x 9) cujo +0xf (ou, com `tambem_13`, o +0x13) e o id
/// de `alvo`; com `nao_11` so serve a de tipo (+0) diferente de 11, sem ele so a de tipo 11. Nenhuma: 0.
unsafe extern "thiscall" fn entrada_da_lista(this: usize, pessoa: usize, alvo: usize, nao_11: u32, tambem_13: u32) -> usize {
    let lista = (*((this + 0xdc) as *const usize)).wrapping_add((*(pessoa as *const i32) as usize).wrapping_mul(9));
    let base = *(lista as *const usize);
    let id = *(alvo as *const u32);
    for k in 0..40usize {
        let e = base.wrapping_add(k * 37);
        let casa = *((e + 0xf) as *const u32) == id || (tambem_13 != 0 && *((e + 0x13) as *const u32) == id);
        if casa && (nao_11 != 0) == (*(e as *const u8) != 11) {
            return e;
        }
    }
    0
}

/// { 0x8676f0 } (staff_contracts.cpp): as bandeiras do contrato (+0x40).
unsafe extern "thiscall" fn bandeiras_do_contrato(this: usize) -> u32 {
    *((this + 0x40) as *const u32)
}

// ---------- 268a leva: os lances e os pedidos de empréstimo da IA pelas 40 vagas da lista de observação (shortlist_manager.cpp) ----------

unsafe fn cqe3_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqe3_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqe3_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqe3_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

unsafe fn cqe3_pb(a: usize, v: u8) {
    *(a as *mut u8) = v
}

unsafe fn cqe3_pessoa(id: usize) -> usize {
    cqe3_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

unsafe fn cqe3_clube(id: usize) -> usize {
    cqe3_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// O contrato da pessoa (0x4cf5a0: push pessoa; mov ecx, [0xae1050] -> thiscall, ret 4), guardado em [0xae1080].
unsafe fn cqe3_contrato(pessoa: usize) -> usize {
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contrato(cqe3_d(0xae1050), pessoa);
    cqe3_pd(0xae1080, c);
    c
}

/// O clube do contrato da pessoa (o tipo 5 conta; sem contrato, 0), com o contrato em [0xae1080].
unsafe fn cqe3_clube_do_contrato(pessoa: usize) -> usize {
    let c = cqe3_contrato(pessoa);
    if c == 0 { 0 } else { cqe3_clube(cqe3_d(c + 4)) }
}

/// A pessoa tem a pessoa no registro de 79 bytes em [0xae2d0c]: o byte +0xb.
unsafe fn cqe3_registro_0b(pessoa: usize) -> u8 {
    cqe3_b(cqe3_d(0xae2d0c).wrapping_add(cqe3_d(pessoa).wrapping_mul(0x4f)) + 0xb)
}

/// { 0x84a020 } (thiscall, ret 4, shortlist_manager.cpp, quadro SEH: STAFF_CONTRACT e oferta locais, só a oferta com
/// destrutor): `Shortlist_Manager::make_club_bids(clube)`, os lances e pedidos de empréstimo da IA, pelo laço do dia
/// (0x5cccd0). Hoje + 7 (0x54c5b0) e um STAFF_CONTRACT local (0x85e410). Seleção (0x53b900), clube humano (0x600a50 com 0,
/// 1), ou humano de férias (com 1, 1) cujo técnico responde (0x5fd210) com a regra 4 (0x5fea50) igual a 1: nada. A lista
/// do clube (o registro de 9 bytes em [this+0xdc]): ativa (+4 = 1), com menos de 100 dias (+8) e finanças (0x5a0590).
/// A conta = vagas com lance de jogador (0x844540 com 0xb) + o elenco (0x53c590 com 0, 0). Cada uma das 40 vagas de
/// 0x25 bytes com alvo (+0xf) e sem oferta (+0x17 = −1), fora as de jogador com a conta em 50: o alvo fora do clube; nas
/// vagas de outro cargo, o cargo 8/9/10 com lugar livre nas listas +0x19f (5), +0x1b3 (7), +0x1cf (3) e o 6/7 sem
/// técnico (+0xd3); sem os bits 0x30 de +0xb do registro de 79 bytes (vaga de jogador), sem transferência futura
/// (0x8cca10), contrato acertado (0x8c5450) nem empréstimo de saída (+0x24 com +0x28 = 5).
/// - Vaga de compra (+0x24 bit 0 apagado): posição (+1) não negativa na de jogador; o clube chega a ele (0x8ce990, ou o
///   motivo 9); sem oferta do clube por ele (0x8cdc90) nem troca (0x8c5e40). O valor = +0xb (dobrado com o bit 0x20, pelo
///   menos 20000); com a liga (+0x57) ativa em [0xadadfc], × 3 ou × 2 pela reputação (+0x80 acima de 0x128e) e a conta
///   abaixo de 25/35, ou × 2 abaixo de 25. Vaga de jogador com substituto (+0x13): sem poder contratar (0x8ced40 com
///   hoje + 7, 1; fora o motivo 9) soma o +0xb (pelo menos 10000); a lesão do substituto (0x62ed70 em [0xb63d28], três
///   vezes) soma dias² × (reputação / 500); com ficha, (1,5 − a moral do jogador, o byte +0x45 da ficha, / 30) × valor; sob oferta (0x8cde70) soma
///   max(+0xb × 1,5, 10000). Vaga de jogador com vendedor humano (ou alvo já no clube): o contrato desejado (0x85fc50) 0
///   pula; salário (+0xc) acima de 1,3 × o teto (0x5968d0) pula. Os sorteios (0x90cfc0): sorteio(valor) acima de
///   sorteio(2 × reputação + 1000) faz o lance: +0x24 bit 5 apagado, a oferta local (0x8e5680) preenchida (0x8e5d60);
///   sem oferta, a situação na lista (0x843e10 com 1); com ela, o lance (0x8be300) e, com termos, a oferta de contrato
///   (0x4cc490) e, vendedor da IA sem oferta de contrato (0x4cc9e0) e com contrato do alvo (0x4cf5f0), a negociação de
///   renovação (0x4cda70 com o fator pelo estatuto 1..5 e a entrada do vendedor 0x849ce0) e os termos liberados; +0x17 =
///   o índice da oferta e a conta sobe na de jogador. A oferta local é destruída (0x8e5710).
/// - Vaga de empréstimo (bit 0): fora da Inglaterra ([0x9cf300]) de +0x64 = 3; o empréstimo possível (0x8d0040); vendedor
///   humano sem o bit 2 de +0x4e do contrato pula; sem oferta (0x8cdc90) nem troca; sorteio(+0xb) acima de
///   sorteio(reputação / 10), ou de sorteio(2 × reputação) no segundo par: o pedido (0x842740) em +0x17; negativo, a
///   situação na lista; senão a conta sobe.
unsafe extern "thiscall" fn lances_do_clube(this: usize, clube: usize) {
    let mais: unsafe extern "thiscall" fn(usize, *mut [u32; 2], u32) -> usize = core::mem::transmute(0x54c5b0usize);
    let novo_contrato: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x85e410usize);
    let e_selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let tecnico: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53ca90usize);
    let responde: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5fd210usize);
    let regra: unsafe extern "thiscall" fn(usize, u32, usize) -> u8 = core::mem::transmute(0x5fea50usize);
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    let vagas_com_lance: unsafe extern "thiscall" fn(usize, usize, u32) -> u8 = core::mem::transmute(0x844540usize);
    let contar_elenco: unsafe extern "C" fn(usize, u32, u32) -> u8 = core::mem::transmute(0x53c590usize);
    let futura: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x8cca10usize);
    let acertado: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x8c5450usize);
    let chegar: unsafe extern "thiscall" fn(usize, usize, usize, *mut u16, u32) -> u32 = core::mem::transmute(0x8ce990usize);
    let tem_oferta: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x8cdc90usize);
    let trocas: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x8c5e40usize);
    let pode_contratar: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, u32, *mut u16) -> u32 = core::mem::transmute(0x8ced40usize);
    let lesao: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x62ed70usize);
    let sob_oferta: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x8cde70usize);
    let desejado: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32, u32) -> i32 = core::mem::transmute(0x85fc50usize);
    let teto: unsafe extern "thiscall" fn(usize, usize, u32) -> i32 = core::mem::transmute(0x5968d0usize);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let nova_oferta: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x8e5680usize);
    let destruir_oferta: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x8e5710usize);
    let preencher: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, *mut usize) -> u32 = core::mem::transmute(0x8e5d60usize);
    let fazer_lance: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x8be300usize);
    let oferta_de_contrato: unsafe extern "thiscall" fn(usize, usize, i32, u32, u32, u32, u32, u32) = core::mem::transmute(0x4cc490usize);
    let tem_oferta_de_contrato: unsafe extern "thiscall" fn(usize, i32, i32) -> usize = core::mem::transmute(0x4cc9e0usize);
    let contrato_com: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let entrada: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(0x849ce0usize);
    let renovar: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32, u32) = core::mem::transmute(0x4cda70usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
    let situacao: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u8 = core::mem::transmute(0x843e10usize);
    let pode_emprestar: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32 = core::mem::transmute(0x8d0040usize);
    let pedir_emprestimo: unsafe extern "thiscall" fn(usize, usize, usize) -> i32 = core::mem::transmute(0x842740usize);
    let mut semana = [0u32; 2];
    mais(0xae2c90, &mut semana, 7);
    let pdata = semana.as_mut_ptr() as usize;
    let mut contrato0 = [0u8; 0x50];
    novo_contrato(contrato0.as_mut_ptr() as usize);
    if e_selecao(clube) != 0 || humano(clube, 0, 1, 0, 0) != 0 {
        return;
    }
    if humano(clube, 1, 1, 0, 0) != 0 && responde(cqe3_d(0xb63c98), tecnico(clube)) != 0
        && regra(cqe3_d(0xb63c98), 4, tecnico(clube)) == 1
    {
        return;
    }
    let reg = cqe3_d(this + 0xdc).wrapping_add(cqe3_d(clube).wrapping_mul(9));
    if cqe3_b(reg + 4) != 1 || cqe3_b(reg + 8) >= 100 || financas(cqe3_d(0xae2a38), clube) == 0 {
        return;
    }
    let mut conta = vagas_com_lance(this, clube, 0xb) as i8;
    conta = conta.wrapping_add(contar_elenco(clube, 0, 0) as i8);
    // ponytail: o motivo (palavra local) a original não inicializa; a 0x8ce990 e a 0x8ced40 o gravam antes da leitura
    let mut motivo: u16 = 0;
    let gm = cqe3_d(0xde1f54);
    for s in 0..0x28usize {
        let off = s * 0x25;
        let vaga = |k: usize| cqe3_d(reg).wrapping_add(off) + k;
        let jogador = cqe3_b(vaga(0)) == 0xb;
        if conta >= 0x32 && jogador {
            continue;
        }
        let id = cqe3_d(vaga(0xf)) as i32;
        if id <= -1 || cqe3_d(vaga(0x17)) as i32 != -1 {
            continue;
        }
        let p = cqe3_pessoa(id as usize);
        if cqe3_clube_do_contrato(p) == clube {
            continue;
        }
        if !jogador {
            let livre = |base: usize, n: i8| {
                let mut k: i8 = 0;
                while k < n {
                    if cqe3_d(clube + base + 4 * k as usize) == 0 {
                        break;
                    }
                    k += 1;
                }
                k != n
            };
            match cqe3_b(vaga(0)) {
                8 if !livre(0x19f, 5) => continue,
                9 if !livre(0x1b3, 7) => continue,
                10 if !livre(0x1cf, 3) => continue,
                6 | 7 if cqe3_d(clube + 0xd3) != 0 => continue,
                _ => {}
            }
        }
        if cqe3_registro_0b(p) & 0x30 != 0 && jogador {
            continue;
        }
        if futura(gm, p, 0) != 0 || acertado(gm, p, 0) != 0 {
            continue;
        }
        if cqe3_d(p + 0x24) != 0 && cqe3_b(p + 0x28) == 5 {
            continue;
        }
        if cqe3_b(vaga(0x24)) & 1 == 0 {
            // a vaga de compra
            if cqe3_w(vaga(1)) <= -1 && jogador {
                continue;
            }
            if chegar(gm, p, clube, &mut motivo, 0) == 0 && motivo != 9 {
                continue;
            }
            if tem_oferta(gm, p, clube) != 0 || trocas(gm, p, 0) != 0 {
                continue;
            }
            let mut v = cqe3_d(vaga(0xb)) as i32;
            if cqe3_b(vaga(0x24)) & 0x20 != 0 {
                v = v.wrapping_shl(1);
                if v <= 0x4e20 {
                    v = 0x4e20;
                }
            }
            let liga = cqe3_d(clube + 0x57);
            if liga != 0 {
                let k = cqe3_d(liga) as i32;
                if k > -1 && k < cqe3_d(0xae2378) as i32 && cqe3_d(cqe3_d(0xadadfc).wrapping_add(k as usize * 4)) != 0 {
                    if cqe3_w(clube + 0x80) > 0x128e {
                        if conta < 0x19 {
                            v = v.wrapping_mul(3);
                        } else if conta < 0x23 {
                            v = v.wrapping_mul(2);
                        }
                    } else if conta < 0x19 {
                        crate::comparacao::ramo("cqe_3 dobro pela liga");
                        v = v.wrapping_mul(2); // cqe3: o dobro pela liga ativa
                    }
                }
            }
            if cqe3_d(vaga(0x13)) as i32 > -1 && cqe3_b(vaga(0)) == 0xb {
                let atual = cqe3_clube_do_contrato(p);
                if pode_contratar(gm, clube, pdata, p, atual, 1, &mut motivo) == 0 && motivo != 9 {
                    let x = cqe3_d(vaga(0xb)) as i32;
                    v = v.wrapping_add(if x > 0x2710 { x } else { 0x2710 });
                }
                let sub = cqe3_pessoa(cqe3_d(vaga(0x13)));
                let les = cqe3_d(0xb63d28);
                if lesao(les, sub) != 0 {
                    let a = cqe3_w(lesao(les, sub) + 0x2c) as i32;
                    let b = cqe3_w(lesao(les, sub) + 0x2c) as i32;
                    v = v.wrapping_add(a.wrapping_mul(b).wrapping_mul(cqe3_w(clube + 0x80) as i32 / 500));
                }
                let f = cqe3_d(sub + 0x61);
                if f != 0 {
                    let x = cqe3_b(f + 0x45) as i8 as f64;
                    v = v.wrapping_add(ftol((f64_em(0x968940) - x * f64_em(0x969020)) * v as f64));
                }
                if sob_oferta(gm, sub, 0) != 0 {
                    let x = cqe3_d(vaga(0xb)) as i32 as f64 * f64_em(0x968940);
                    v = v.wrapping_add(ftol(if x > f64_em(0x968910) { x } else { f64_em(0x968910) }));
                }
            }
            let c = cqe3_contrato(p);
            if c != 0 && cqe3_clube(cqe3_d(c + 4)) != 0 && jogador {
                let vendedor = cqe3_clube_do_contrato(p);
                if humano(vendedor, 0, 1, 0, 0) != 0 || cqe3_d(p + 0x39) == clube {
                    let mut contrato = [0u8; 0x50];
                    let pc = contrato.as_mut_ptr() as usize;
                    novo_contrato(pc);
                    let e = vaga(0);
                    // ponytail: o estatuto vai com os bytes altos do ponteiro da vaga (como na original); o cargo, com os do
                    // edx que a 0x600a50 deixou (lixo): a 0x85fc50 recebe um char, só o byte conta
                    let st = (e as u32 & 0xffff_ff00) | ((cqe3_b(e + 0x24) >> 1) & 0xf) as u32;
                    let r = desejado(pc, p, clube, u32::MAX, st, cqe3_b(e) as u32, 0, 0);
                    if r as f64 == f64_em(0x9689a0) {
                        continue;
                    }
                    // ponytail: o cargo vai com os bytes altos do resultado e do fnstsw; a 0x5968d0 lê só o byte
                    let t = teto(financas(cqe3_d(0xae2a38), clube), p, ((cqe3_b(vaga(0x24)) >> 1) & 0xf) as u32);
                    if (t as f64 * f64_em(0x968de8)) < cqe3_d(pc + 0xc) as i32 as f64 {
                        continue;
                    }
                }
            }
            crate::comparacao::ramo("cqe_3 sorteio do lance");
            let r1 = sorteio((cqe3_w(clube + 0x80) as i32).wrapping_mul(2).wrapping_add(1000)); // cqe3: o sorteio do lance
            let r2 = sorteio(v);
            if r2 <= r1 {
                continue;
            }
            cqe3_pb(vaga(0x24), cqe3_b(vaga(0x24)) & 0xdf);
            let mut oferta = [0u8; 0x44];
            let po = oferta.as_mut_ptr() as usize;
            nova_oferta(po);
            let mut termos: usize = 0;
            if preencher(po, p, clube, vaga(0), cqe3_d(vaga(7)) as u32, 0, &mut termos) == 0 {
                situacao(this, p, clube, 1, (cqe3_b(vaga(0)) != 0xb) as u32);
            } else {
                fazer_lance(gm, po, 0);
                if termos != 0 {
                    // ponytail: o cargo vai com os bytes altos do ponteiro do vetor das vagas (como na original)
                    let cargo = (cqe3_d(reg) as u32 & 0xffff_ff00) | cqe3_b(vaga(0)) as u32;
                    oferta_de_contrato(cqe3_d(0xae1050), termos, cqe3_d(po + 0x20) as i32, cqe3_d(po + 0x24) as u32, cargo, 0, 1, 0);
                    let c = cqe3_contrato(p);
                    let vendedor = if c == 0 { 0 } else { cqe3_clube(cqe3_d(c + 4)) };
                    if vendedor != 0 && humano(vendedor, 0, 1, 0, 0) == 0
                        && tem_oferta_de_contrato(cqe3_d(0xae1050), cqe3_d(p) as i32, cqe3_d(vendedor) as i32) == 0
                    {
                        let cv = contrato_com(cqe3_d(0xae1050), p, vendedor);
                        if cv != 0 {
                            let e = (cqe3_b(cv + 0x4f) >> 4) as i8;
                            let fator = match e {
                                1 | 2 if entrada(this, vendedor, p, 0, 0) != 0 => {
                                    let rep = cqe3_w(clube + 0x80);
                                    let div = if rep < 1 { 1 } else { rep as i32 };
                                    let f = (cqe3_registro_0b(p) & 0xf) as f64 * f64_em(0x967880) + rep as i32 as f64 / div as f64;
                                    Some(if f64_em(0x967890) <= f { if f64_em(0x968dc0) < f { 1.4 } else { f } } else { 1.0 })
                                }
                                3 | 4 | 5 if entrada(this, vendedor, p, 0, 0) != 0 => {
                                    let rep = cqe3_w(clube + 0x80) as i32;
                                    let div = if rep.wrapping_mul(2) >= 1 { rep.wrapping_mul(2) } else { 1 };
                                    let f = (cqe3_registro_0b(p) & 0xf) as f64 * f64_em(0x967880) + rep as f64 / div as f64 + f64_em(0x968968);
                                    Some(if f64_em(0x967890) <= f { if f64_em(0x968978) < f { 1.2 } else { f } } else { 1.0 })
                                }
                                _ => None,
                            };
                            if let Some(f) = fator {
                                let b = f.to_bits();
                                renovar(cqe3_d(0xae1050), p, 0, 0, u32::MAX, b as u32, (b >> 32) as u32, 2);
                            }
                        }
                    }
                    liberar(termos);
                }
                cqe3_pd(vaga(0x17), cqe3_d(po + 0x20));
                if jogador {
                    conta = conta.wrapping_add(1);
                }
            }
            destruir_oferta(po);
        } else {
            // a vaga de empréstimo
            let n = cqe3_d(clube + 0x53);
            if n != 0 && cqe3_d(n) == cqe3_d(0x9cf300) && cqe3_b(clube + 0x64) == 3 {
                continue;
            }
            if pode_emprestar(gm, clube, p, 0) == 0 {
                continue;
            }
            let c = cqe3_contrato(p);
            if c != 0 && cqe3_clube(cqe3_d(c + 4)) != 0 && humano(cqe3_clube_do_contrato(p), 0, 1, 0, 0) != 0 {
                let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
                if cqe3_b(contrato(cqe3_d(0xae1050), p) + 0x4e) & 2 == 0 {
                    continue;
                }
            }
            if tem_oferta(gm, p, clube) != 0 || trocas(gm, p, 0) != 0 {
                continue;
            }
            crate::comparacao::ramo("cqe_3 sorteio do empréstimo");
            let r1 = sorteio(cqe3_w(clube + 0x80) as i32 / 10); // cqe3: o sorteio do empréstimo
            let r2 = sorteio(cqe3_d(vaga(0xb)) as i32);
            if r2 <= r1 {
                let r1 = sorteio((cqe3_w(clube + 0x80) as i32).wrapping_shl(1));
                let r2 = sorteio(cqe3_d(vaga(0xb)) as i32);
                if r2 <= r1 {
                    continue;
                }
            }
            let k = pedir_emprestimo(this, p, clube);
            cqe3_pd(vaga(0x17), k as u32 as usize);
            if cqe3_d(vaga(0x17)) as i32 > -1 {
                conta = conta.wrapping_add(1);
            } else {
                // ponytail: o setne vai com os bytes altos do ponteiro da vaga (como na original); a 0x843e10 lê o byte
                let e = vaga(0);
                situacao(this, p, clube, 1, (e as u32 & 0xffff_ff00) | (cqe3_b(e) != 0xb) as u32);
            }
        }
    }
}

/// Comparador da 0x84a020: só a original (sorteia, faz lances e pedidos de empréstimo, aloca ofertas e termos).
unsafe extern "thiscall" fn comparar_lances_do_clube(this: usize, clube: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize)>(1583)(this, clube) // ponytail: só a original
}

// ---------- 268a leva: a vontade de uma pessoa de ir para um clube (ou de ficar) e o construtor do contrato (shortlist_manager.cpp, staff_contracts.cpp) ----------

unsafe fn cqe4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqe4_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqe4_c(a: usize) -> i8 {
    *(a as *const i8)
}

unsafe fn cqe4_h(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqe4_w(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

/// Uma constante f64 do exe.
unsafe fn cqe4_k(a: usize) -> f64 {
    core::ptr::read_unaligned(a as *const f64)
}

/// Uma constante f32 do exe.
unsafe fn cqe4_kf(a: usize) -> f32 {
    core::ptr::read_unaligned(a as *const f32)
}

/// A conta dos dias entre o ano e o dia de hoje ([0xae2c92], [0xae2c90]) e os de uma data (ano em `ano`, dia em
/// `dia`), como a original: (ano − ano de hoje) × 365 − dia de hoje + dia.
unsafe fn cqe4_dias_para(ano: usize, dia: usize) -> i32 {
    ((cqe4_w(ano) as i32 - cqe4_w(0xae2c92) as i32) * 365 - cqe4_h(0xae2c90) as i32) + cqe4_h(dia) as i32
}

/// Hoje menos 150 dias (0x54c8e0: push 0x96; push saída; mov ecx, 0xae2c90 -> thiscall, ret 8) já passou da data
/// guardada em +0x3e (dia) / +0x40 (ano) do técnico `t`: ano maior (sem sinal), ou o mesmo ano e o dia maior (com
/// sinal).
unsafe fn cqe4_depois_de(t: usize) -> bool {
    let menos: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) = core::mem::transmute(0x54c8e0usize);
    let mut d = [0u8; 8];
    menos(0xae2c90, &mut d, 0x96);
    let dia = i16::from_le_bytes([d[0], d[1]]);
    let ano = u16::from_le_bytes([d[2], d[3]]);
    let ano_t = cqe4_w(t + 0x40);
    ano > ano_t || (ano == ano_t && dia > cqe4_h(t + 0x3e))
}

/// A distância entre as cidades `a` e `b` (coordenadas f64 em +0x23 e +0x2b), × 55.
unsafe fn cqe4_distancia(a: usize, b: usize) -> f64 {
    let dx = cqe4_k(a + 0x2b) - cqe4_k(b + 0x2b);
    let dy = cqe4_k(a + 0x23) - cqe4_k(b + 0x23);
    (dy * dy + dx * dx).sqrt() * cqe4_k(0x96d1b8)
}

/// O máximo da original nas contas de reputação: `x` se ele passa `y`, senão `y`.
fn cqe4_max(x: f64, y: f64) -> f64 {
    if x > y { x } else { y }
}

/// A média das três reputações da ficha do jogador (+9, +0xb, +0xd), truncada.
unsafe fn cqe4_media3(f: usize) -> i32 {
    (cqe4_h(f + 0xd) as i32 + cqe4_h(f + 0xb) as i32 + cqe4_h(f + 9) as i32) / 3
}

/// A reputação do clube pelo estádio: sem estádio ou sem cidade (+0x69, +0x38), trunc(rep × 2/7); com a cidade de
/// importância 0 (+0x33), rep / 2; senão o maior entre rep × 2/7 e rep × importância / 15, truncado.
unsafe fn cqe4_pelo_estadio(clube: usize) -> i32 {
    let rep = cqe4_h(clube + 0x80) as i32;
    let e = cqe4_d(clube + 0x69);
    let cidade = if e != 0 { cqe4_d(e + 0x38) } else { 0 };
    if cidade == 0 {
        return ftol(rep as f64 * cqe4_k(0x96d5e0));
    }
    let imp = cqe4_c(cidade + 0x33);
    if imp == 0 {
        return rep / 2;
    }
    let q = (imp as i32).wrapping_mul(rep) / 15;
    let x = rep as f64 * cqe4_k(0x96d5e0);
    ftol(cqe4_max(x, q as f64))
}

/// A reputação do clube pela divisão (+0x57, o +0x69 dela): sem divisão, a própria (o resto do registrador vem do
/// ponteiro; só a palavra conta); com reputação de 7500 e público acima de 35000 (+0x73), o maior entre a reputação e
/// 500 × a da divisão; senão o maior entre rep/2 + 250 × a da divisão e 500 × a da divisão.
unsafe fn cqe4_pela_divisao(clube: usize) -> i16 {
    let rep = cqe4_h(clube + 0x80) as i32;
    let comp = cqe4_d(clube + 0x57);
    if comp == 0 {
        return rep as i16;
    }
    let cv = cqe4_h(comp + 0x69) as i32;
    let t = cv.wrapping_mul(500);
    if rep >= 0x1d4c && cqe4_d(clube + 0x73) as i32 > 0x88b8 {
        (if t > rep { t } else { rep }) as i16
    } else {
        let h = rep / 2 + cv.wrapping_mul(250);
        (if t > h { t } else { h }) as i16
    }
}

/// A reputação pela competição do clube (+0x1db; a da lista [0xae2d14] + id × 6 quando ela é negativa ou é a
/// [0x9cf6f8] com reputação de 5000): o registro de [0xae23d0] (0x6b bytes; 15000 para a [0x9cf6e8], senão 600 × +0x69
/// em 16 bits) e, com o clube na tabela dela (0x4a4850) em 1º a 16º sem a marca +4, (1,32 − 0,02 × posição) × isso.
/// Sem registro: o mínimo entre 0,8 × `pela_divisao` e 7000. Devolve (valor, registro).
unsafe fn cqe4_pela_competicao(clube: usize, pela_divisao: i16) -> (i32, usize) {
    let rep = cqe4_h(clube + 0x80);
    let mut idx = cqe4_d(clube + 0x1db) as i32;
    let mut tem = true;
    if idx <= -1 || (idx == cqe4_d(0x9cf6f8) as i32 && rep >= 0x1388) {
        idx = cqe4_d(cqe4_d(0xae2d14).wrapping_add(cqe4_d(clube).wrapping_mul(6))) as i32;
        if idx <= -1 {
            tem = false;
        }
    }
    let mut reg = 0usize;
    if tem {
        reg = cqe4_d(0xae23d0).wrapping_add((idx as usize).wrapping_mul(0x6b));
        if reg != 0 {
            let id = cqe4_d(reg);
            let s: i16 = if id == cqe4_d(0x9cf6e8) { 15000 } else { cqe4_h(reg + 0x69).wrapping_mul(600) };
            // 0x4a4850: push clube; mov ecx, [[0xadadfc] + 4 × id] + 8 -> thiscall, ret 4
            let achar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
            let r = achar(cqe4_d(cqe4_d(cqe4_d(0xadadfc).wrapping_add(id.wrapping_mul(4))) + 8), clube);
            if r != 0 && cqe4_b(r + 4) == 0 && (1..=0x10).contains(&cqe4_b(r + 0x72)) {
                let p = cqe4_b(r + 0x72) as f64;
                return (ftol((cqe4_k(0x96af38) - p * cqe4_k(0x968a98)) * s as f64), reg);
            }
            return (s as i32, reg);
        }
    }
    let mut x = pela_divisao as f64 * cqe4_k(0x9689b0);
    if !(x < cqe4_k(0x96d470)) {
        x = cqe4_k(0x96d470);
    }
    (ftol(x), reg)
}

/// A reputação vista pelo técnico ([0xb67a44] + id × 0x49: (+0xa + +8) × 0,25 + +6 × 0,5): abaixo de 5000, o maior
/// entre r / (5 − 0,0008 v) e r − 1000 × (5 − 0,0008 v); senão o menor entre r × (0,5 + 0,0001 v) e r + 1000 × isso.
unsafe fn cqe4_pelo_tecnico(clube: usize, r: i16) -> i32 {
    let t = cqe4_d(cqe4_d(0xb67a44)).wrapping_add(cqe4_d(clube).wrapping_mul(0x49));
    let v = ftol((cqe4_h(t + 0xa) as f64 + cqe4_h(t + 8) as f64) * cqe4_k(0x9689e0) + cqe4_h(t + 6) as f64 * cqe4_k(0x968968)) as i16;
    let dd = r as f64;
    if v < 0x1388 {
        let t = cqe4_k(0x9688a0) - v as f64 * cqe4_k(0x96a618);
        let q1 = dd / t;
        let m = dd - t * cqe4_k(0x968908);
        ftol(if q1 > m { q1 } else { m })
    } else {
        let t = v as f64 * cqe4_k(0x96a5c0) + cqe4_k(0x968968);
        let q1 = t * dd;
        let m = t * cqe4_k(0x968908) + dd;
        ftol(if q1 < m { q1 } else { m })
    }
}

/// O teto da reputação pelas finanças (0x5a0590 e 0x5986b0): faixa 1, o maior entre 1,25 r e r + 750; faixa 2, o maior
/// entre 1,5 r e r + 1500.
fn cqe4_pela_faixa(faixa: i8, r: i16) -> i16 {
    match faixa {
        1 => ftol(cqe4_max(r as f64 * 1.25, (r as i32 + 0x2ee) as f64)) as i16,
        2 => ftol(cqe4_max(r as f64 * 1.5, (r as i32 + 0x5dc) as f64)) as i16,
        _ => r,
    }
}

/// min(h / 2, h − 2000) sobre a palavra, a correção pelo +0x64 do clube (diferente de 1; e mais uma vez com 3).
fn cqe4_metade(h: i16) -> i16 {
    let a = h as i32 / 2;
    let e = h as i32 - 2000;
    (if a < e { a } else { e }) as i16
}

/// A fluência: max(F × 0,1, L / (12000 − 200 F)) + 0,2, com L = (s + 5000)² × 1/15000 e F a fluência da pessoa no
/// idioma da nação do clube (0x7c6830: push 1; push idioma; push clube; push pessoa; mov ecx, [0xb74350] -> thiscall,
/// ret 0x10; o idioma vai num registrador com lixo acima, a cópia passa só o byte). Devolve (fator, F).
unsafe fn cqe4_fluencia(p1: usize, clube: usize, s: i16) -> (f64, f64) {
    let q = (s as i32 + 0x1388) as f64;
    let l = q * cqe4_k(0x968d78) * q;
    let fluencia: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u8 = core::mem::transmute(0x7c6830usize);
    let f = fluencia(cqe4_d(0xb74350), p1, clube, cqe4_b(cqe4_d(clube + 0x53) + 0x77) as u32, 1) as i8 as i32 as f64;
    let a = f * cqe4_k(0x967880);
    let b = l / (cqe4_k(0x96f938) - f * cqe4_k(0x96a648));
    (cqe4_max(a, b) + cqe4_k(0x968918), f)
}

/// { 0x846920 } (thiscall, ret 0x18, shortlist_manager.cpp): `Shortlist_Manager::staff_interest_in_joining_club(pessoa,
/// clube, salário, empréstimo, situação, motivos)`: a nota de vontade da pessoa de ir para o clube (ou de ficar, quando
/// o clube é o dela), x87 transcrito em f64 na ordem da original. Os motivos (bits) só são marcados com o ponteiro. A
/// nota parte de 2,75 (3,5 para o próprio clube; com contrato acabando, 1,1 − dias × 0,002; sem clube, a idade fora do
/// clube), e passa pelos fatores: reputação dos clubes no empréstimo, gostar ou não do clube e do técnico, rivais, as
/// nações e a distância, a felicidade no contrato (+0x44), a situação no elenco (+0x4f) e a pedida, a reputação comparada (a do clube
/// novo e a do atual, pela divisão, pela competição, pelo técnico e pelas finanças), a fluência no idioma, os jogos
/// de seleção e o tempo sem clube; e, pedido o salário (salário ≠ 0, sem empréstimo e nota de 1,75), o salário padrão
/// (0x864550 num STAFF_CONTRACT local) contra o teto do clube (0x5968d0). O resto das contas está nos comentários.
unsafe extern "thiscall" fn cqe4_interesse(this: usize, p1: usize, p2: usize, p3: u32, p4: u32, p5: u32, p6: *mut u32) -> f64 {
    use crate::comparacao::ramo;
    let k = |a: usize| cqe4_k(a);
    let zero = k(0x9689a0);
    // as chamadas por endereço, com as convenções conferidas nos pontos de chamada da original
    // 0x54a110: lea ecx, data -> thiscall, ret 0
    let data_padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    // 0x8cca10: push 0; push pessoa; mov ecx, [0xde1f54] -> thiscall, ret 8
    let futura: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x8cca10usize);
    // 0x4cf5a0: push pessoa; mov ecx, [0xae1050] -> thiscall, ret 4
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    // 0x4cf5f0: push clube; push pessoa; mov ecx, [0xae1050] -> thiscall, ret 8
    let contrato_com: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    // 0x54cdb0: push [0xae2c94]; push [0xae2c90]; lea ecx, data -> thiscall, ret 8
    let dias_ate: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x54cdb0usize);
    // 0x53d980: push 1; push nação; push pessoa; add esp, 0xc -> cdecl
    let estrangeira: unsafe extern "C" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x53d980usize);
    // 0x544590 / 0x544520 / 0x5444a0: push clube; push pessoa; add esp, 8 -> cdecl
    let na_lista_emp: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x544590usize);
    let na_lista_transf: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x544520usize);
    let na_lista: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x5444a0usize);
    // 0x547d30 / 0x547d60: push clube; push pessoa; add esp, 8 -> cdecl
    let nao_gosta_do_clube: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547d30usize);
    let gosta_do_clube: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547d60usize);
    // 0x547810 / 0x547840: push pessoa; push técnico; add esp, 8 -> cdecl
    let gosta_de: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547810usize);
    let nao_gosta_de: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547840usize);
    // 0x4b02e0: push atual; push clube; add esp, 8 -> cdecl
    let rival: unsafe extern "C" fn(usize, usize) -> u8 = core::mem::transmute(0x4b02e0usize);
    // 0x4ca470: push contrato; push pessoa; mov ecx, [0xae1050] -> thiscall, ret 8
    let categoria: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(0x4ca470usize);
    // 0x600a50: push 0; push 0; push 1 (ou 1); push 0 (ou 1); push clube; add esp, 0x14 -> cdecl
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    // 0x5407e0: push 1; push pessoa; push técnico; add esp, 0xc -> cdecl (só a palavra conta)
    let vista: unsafe extern "C" fn(usize, usize, u32) -> i16 = core::mem::transmute(0x5407e0usize);
    // 0x5a0590: push clube; mov ecx, [0xae2a38] -> thiscall, ret 4; 0x5986b0: mov ecx, finanças -> thiscall, ret 0
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    let faixa: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x5986b0usize);
    // 0x4cbb60: push clube; push pessoa; mov ecx, [0xae1050] -> thiscall, ret 8
    let lugar: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(0x4cbb60usize);
    // 0x669bd0: push &data b; push &data a; push clube; add esp, 0xc -> cdecl
    let na_temporada: unsafe extern "C" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x669bd0usize);

    let mut data_a = [0u8; 8];
    let mut data_b = [0u8; 8];
    data_padrao(data_a.as_mut_ptr() as usize);
    data_padrao(data_b.as_mut_ptr() as usize);
    let p4b = p4 as u8 != 0;
    let p5b = p5 as u8 as i8;
    let mut l80 = 1.0f64; // [base+0x84]
    let mut l8c = 0usize; // [base+0x78]: o registro da competição do clube atual
    let mut l84 = 0usize; // [base+0x80]: o registro da competição do clube novo
    let mut ad: i8 = 0; // [base+0x57]
    let fl = move |b: u32| {
        if !p6.is_null() {
            *p6 |= b;
        }
    };
    if !p6.is_null() {
        *p6 = 0;
    }
    if cqe4_d(p1 + 0x61) == 0 || cqe4_d(p2 + 0x53) == 0 || cqe4_d(p1 + 0x1a) == 0 {
        return zero;
    }
    if futura(cqe4_d(0xde1f54), p1, 0) != 0 {
        return zero;
    }
    if cqe4_b(cqe4_d(0xae2d0c).wrapping_add(cqe4_d(p1).wrapping_mul(0x4f)) + 0xb) & 0x30 != 0 {
        return zero;
    }
    let a57 = cqe4_c(p1 + 0x57) as i32 as f64;
    let idade = cqe4_c(p1 + 0x18);
    let aa8: f32 = ((k(0x968dc0) - a57 * k(0x967878)) - cqe4_c(p1 + 0x5b) as i32 as f64 * k(0x967878)) as f32;
    let mut a88: f32 = (a57 * k(0x968a98) + k(0x9689b0)) as f32;
    let mut b4: f32 = (a57 * k(0x967888) + k(0x968968)) as f32;
    let c0 = contrato_de(cqe4_d(0xae1050), p1);
    core::ptr::write_unaligned(0xae1080 as *mut usize, c0);
    let mut d4 = 0usize;
    if c0 != 0 {
        d4 = cqe4_d(0xae23b4).wrapping_add(cqe4_d(c0 + 4).wrapping_mul(0x245));
        if d4 != 0 {
            if cqe4_d(d4 + 0x53) == 0 {
                return k(0x968e38);
            }
            if cqe4_b(contrato_de(cqe4_d(0xae1050), p1) + 0x35) & 0x3f == 5 {
                d4 = 0;
            }
        }
    }
    let c = contrato_com(cqe4_d(0xae1050), p1, d4);
    let a0: u32 = if c != 0 { cqe4_d(c + 0x45) as u32 } else { 0 };
    let r61 = cqe4_d(p1 + 0x61);
    // o próprio clube com contrato: fica (3,5) quando o contrato não acaba logo e a reputação dele não passa a do clube
    if p2 != 0 && p2 == d4 && c != 0 {
        let dias = dias_ate(c + 0x2d, cqe4_d(0xae2c90) as u32, cqe4_d(0xae2c94) as u32);
        let r1 = cqe4_h(r61 + 0xb) as i32;
        let mut s = ftol((cqe4_h(p2 + 0x80) as i32 - r1) as f64 * k(0x968968)) as i16;
        if a0 & 0x10 != 0 {
            s = ftol(s as f64 * k(0x968968)) as i16;
        }
        let lim = cqe4_c(p1 + 0x59) as i32 * 10 + 0x465;
        let s: i32 = if s < 0 { 0 } else if s as i32 > lim { lim as i16 as i32 } else { s as i32 };
        if a0 & 0x161a8 == 0 {
            let id = cqe4_d(p1) as i32;
            if dias < id % 0x23 + 0x14
                && cqe4_b(cqe4_d(0xae2d0c).wrapping_add((id as usize).wrapping_mul(0x4f)) + 0xb) & 0xf == 0
                && r1 + cqe4_c(p1 + 0x57) as i32 * 20 < (cqe4_c(p1 + 0x59) as i32 + 0xaa) * 25 + s
            {
                return k(0x968e38);
            }
        }
    }
    let mut f8: f64;
    if p2 == d4 {
        if c != 0 && a0 != 0 && a0 != 2 {
            return zero;
        }
        f8 = 3.5;
    } else {
        if c != 0 && d4 != 0 && cqe4_d(d4 + 0x53) != 0 && cqe4_d(p2 + 0x53) != 0
            && cqe4_b(cqe4_d(p2 + 0x53) + 0x77) == cqe4_b(cqe4_d(d4 + 0x53) + 0x77) && a0 & 0x20 != 0
        {
            return zero;
        }
        if idade < 0x10 && estrangeira(p1, cqe4_d(p2 + 0x53), 1) != 0 {
            return zero;
        }
        ramo("cqe_4 base de outro clube");
        f8 = 2.75; // cqe4: a base para outro clube
        if c != 0 && cqe4_b(c + 0x49) != 0 {
            let dd = cqe4_dias_para(c + 0x2f, c + 0x2d);
            let v = k(0x9689c0) - dd as f64 * k(0x969048);
            if !p6.is_null() && v < cqe4_c(p1 + 0x5a) as i32 as f64 * k(0x968a98) + k(0x968e48) {
                *p6 |= 1;
            }
            f8 = v * k(0x9690a8);
        } else if d4 == 0 {
            let dd =((cqe4_w(0xae2c92) as i32 - cqe4_w(p1 + 0x40) as i32) * 365 - cqe4_h(p1 + 0x3e) as i32) + cqe4_h(0xae2c90) as i32;
            let mut x = dd.wrapping_mul(dd) as f64 * k(0x96f9b8);
            if k(0x96d6a0) < x {
                x = k(0x96d6a0);
            }
            f8 = x + k(0x9690a8);
        }
    }
    // a reputação dos dois clubes num pedido de empréstimo
    if p4b && r61 != 0 && cqe4_h(r61 + 0xd) < 0x128e && p2 != 0 && d4 != 0 && p2 != d4 && a0 & 0x10 != 0 {
        let ax = cqe4_h(p2 + 0x80);
        let mesmo = cqe4_d(p2 + 0x53) == cqe4_d(d4 + 0x53);
        let rb = cqe4_h(r61 + 0xb);
        if ax > cqe4_h(d4 + 0x80) && (mesmo || ax > 0x1676) {
            if rb <= 0x1c52 && na_lista_emp(p1, 0) != 0 {
                f8 *= k(0x968940);
            } else {
                f8 *= k(0x968928);
            }
        } else if ax > 0xabe && (mesmo || ax > 0x1676) && ax > rb {
            if rb <= 0x1c52 && na_lista_emp(p1, 0) != 0 {
                f8 *= k(0x969588);
            } else {
                f8 *= k(0x968980);
            }
        } else if ax > 0x109a && mesmo {
            if rb > 0x186a && ax < 0x1482 {
                f8 *= k(0x968fa0);
            } else if rb <= 0x1c52 && na_lista_emp(p1, 0) != 0 {
                f8 *= k(0x968928);
            } else {
                f8 *= k(0x9689c0);
            }
        }
    }
    if nao_gosta_do_clube(p1, p2) != 0 {
        f8 *= k(0x968f78);
        fl(2);
    }
    if nao_gosta_do_clube(p1, d4) != 0 {
        let s = cqe4_b(p1 + 0x5a).wrapping_add(cqe4_b(p1 + 0x59)).wrapping_add(cqe4_b(p1 + 0x5b)) as i8;
        if s > 0x2d {
            f8 *= k(0x968928);
        } else if s > 0x19 {
            f8 *= k(0x968940);
        } else {
            f8 = f8 + f8;
        }
        fl(4);
    }
    if gosta_do_clube(p1, d4) != 0 && na_lista_transf(p1, d4) == 0 && cqe4_b(c + 0x49) == 0 {
        f8 *= k(0x968968);
        fl(8);
    }
    if gosta_do_clube(p1, p2) != 0 {
        fl(0x10);
        f8 *= k(0x968930);
    }
    if idade > 0x22 && d4 != p2 && cqe4_d(p2 + 0x53) != cqe4_d(p1 + 0x1a) {
        f8 *= k(0x969040);
    }
    let max1 = |n: i8| if n < 1 { 1 } else { n as i32 };
    if d4 != 0 && d4 != p2 {
        let (e1, e2) = (cqe4_d(0x9d0118), cqe4_d(0x9d011c));
        let i2 = cqe4_d(p2);
        let i4 = cqe4_d(d4);
        if (i2 == e1 || i2 == e2) && (i4 == e1 || i4 == e2) {
            f8 = (k(0x967878) / max1(cqe4_c(p1 + 0x59)) as f64) * f8;
            fl(0x20);
        } else {
            let n2 = cqe4_d(cqe4_d(p2 + 0x53));
            let (na, nb) = (cqe4_d(0x9cf2ac), cqe4_d(0x9cf51c));
            if n2 == nb && cqe4_d(cqe4_d(d4 + 0x53)) == na && cqe4_d(cqe4_d(p1 + 0x1a)) == na {
                f8 *= k(0x967888);
                fl(2);
            } else if n2 == na && cqe4_d(cqe4_d(d4 + 0x53)) == nb && cqe4_d(cqe4_d(p1 + 0x1a)) == nb {
                f8 *= k(0x967880);
                fl(2);
            } else {
                if p2 == cqe4_d(d4 + 0xb3) || p2 == cqe4_d(d4 + 0xb7) || p2 == cqe4_d(d4 + 0xbb) {
                    let m = if n2 == cqe4_d(0x9cf360) {
                        k(0x967890) - cqe4_c(p1 + 0x59) as i32 as f64 * k(0x968a98)
                    } else {
                        k(0x968918) / max1(cqe4_c(p1 + 0x59)) as f64
                    };
                    f8 = m * f8;
                    if !p6.is_null() && m < k(0x968e40) {
                        *p6 |= 0x20;
                    }
                }
                if rival(p2, d4) != 0 {
                    let m = if cqe4_d(cqe4_d(p2 + 0x53)) == cqe4_d(0x9cf360) {
                        k(0x967890) - cqe4_c(p1 + 0x59) as i32 as f64 * k(0x968a98)
                    } else {
                        k(0x969040) / max1(cqe4_c(p1 + 0x59)) as f64
                    };
                    f8 = m * f8;
                    if !p6.is_null() && m < k(0x968e40) {
                        *p6 |= 0x20;
                    }
                }
            }
        }
    }
    let t2 = cqe4_d(p2 + 0xcf);
    if gosta_de(t2, p1) != 0 {
        f8 = f8 + f8;
        fl(0x40);
    }
    if nao_gosta_de(t2, p1) != 0 {
        f8 *= k(0x968f78);
        fl(0x80);
    }
    if d4 != 0 {
        let t4 = cqe4_d(d4 + 0xcf);
        if nao_gosta_de(t4, p1) != 0 {
            f8 = f8 + f8;
            fl(0x100);
        }
        if gosta_de(t4, p1) != 0 {
            f8 *= k(0x968968);
            if !p6.is_null() && cqe4_b(c + 0x4e) & 8 == 0 {
                *p6 |= 0x200;
            }
        }
    }
    // a felicidade no contrato (+0x44), a situação no elenco (ea) e a pedida do contrato (c1 = +0x3d)
    let mut ea: i8;
    let c1: i8;
    if c == 0 {
        if p2 == d4 {
            c1 = 0;
            ea = p5b;
        } else {
            ea = 7;
            c1 = -100;
        }
    } else {
        let e44 = cqe4_c(c + 0x44) as i32;
        if p2 == d4 {
            f8 = (e44 as f64 * k(0x969028) + k(0x967890)) * f8;
            ea = p5b;
            if cqe4_d(p2 + 0xcf) != 0 && cqe4_depois_de(cqe4_d(p2 + 0xcf)) {
                ea = categoria(cqe4_d(0xae1050), p1, c) as i8;
            }
        } else {
            let v = k(0x967890) - e44 as f64 * k(0x969028);
            f8 = v * f8;
            if v > k(0x96afa8) {
                f8 = v * f8;
            }
            if a0 & 0x10 != 0 {
                a88 = (a88 as f64 * k(0x968940)) as f32;
            }
            if a0 & 0x100 != 0 {
                a88 = (a88 as f64 * k(0x968940)) as f32;
            }
            if a0 & 1 != 0 {
                b4 = (b4 as f64 * k(0x968940)) as f32;
            }
            if !p6.is_null() {
                if v > k(0x968940) && cqe4_b(c + 0x4e) & 8 == 0 {
                    *p6 |= 0x400;
                } else if v < k(0x968968) {
                    *p6 |= 0x800;
                }
            }
            if d4 != 0 && cqe4_d(d4 + 0xcf) != 0 && cqe4_depois_de(cqe4_d(d4 + 0xcf)) {
                ea = categoria(cqe4_d(0xae1050), p1, c) as i8;
            } else {
                ea = (cqe4_b(c + 0x4f) >> 4) as i8;
            }
            if cqe4_b(c + 0x4e) & 8 != 0 {
                ea = if ea == 1 { 3 } else if ea < 1 || ea > 3 { 7 } else { 4 };
            }
        }
        c1 = cqe4_c(c + 0x3d);
    }
    let pos20 = || cqe4_b(cqe4_d(p1 + 0x61) + 0xf) == 0x14;
    let jovem = idade < 0x20 && cqe4_c(p1 + 0x22) != 0;
    let mut sb: i16; // ebx: a reputação do clube atual (ou a da pessoa sem clube) na escala da comparação
    let mut e8: i32 = 0; // [base+0x1c]: a do clube novo
    let mut h68: i16 = 0;
    let mut h6c4: i32 = 0;
    let mut ae: i8 = 0;
    let mut den: f32 = 1.0; // [base+0x58]
    if p2 != d4 {
        if humano(p2, 0, 1, 0, 0) != 0 {
            let v = cqe4_c(cqe4_d(p2 + 0xcf) + 0x59) as i32 as f64 * k(0x967878) + k(0x968968);
            f8 = v * f8;
            if !p6.is_null() && v < k(0x96a698) {
                *p6 |= 0x20000000;
            }
        }
        match ea {
            1 => {
                f8 *= k(0x96a5d0);
                if !p6.is_null() && cqe4_d(c + 0x45) == 0 {
                    *p6 |= 0x1000;
                }
            }
            2 => {
                f8 *= k(0x969040);
                if !p6.is_null() && cqe4_d(c + 0x45) == 0 {
                    *p6 |= 0x1000;
                }
            }
            3 => f8 *= k(0x9689d8),
            4 => {
                f8 *= k(0x96d640);
                fl(0x2000);
                if idade < 0x20 {
                    if idade > 0x17 {
                        f8 = f8 + f8;
                    }
                    if cqe4_c(p1 + 0x22) != 0 {
                        f8 = f8 + f8;
                    }
                }
            }
            5 => {
                if !p4b {
                    f8 *= k(0x969040);
                }
                if jovem {
                    f8 = f8 + f8;
                }
            }
            6 => {
                f8 *= k(0x9689c0);
                if jovem {
                    f8 *= k(0x968938);
                }
            }
            7 => {
                f8 *= k(0x968988);
                fl(0x2000);
            }
            _ => {}
        }
        let cf10 = c != 0 && cqe4_b(c + 0x45) & 0x10 != 0;
        match p5b {
            1 => {
                if cf10 {
                    f8 = f8 + f8;
                    fl(0x4000);
                } else {
                    f8 *= k(0x96f9b0);
                }
                b4 *= cqe4_kf(0x96a33c);
            }
            2 => {
                if cf10 {
                    f8 *= k(0x96f9a8);
                    fl(0x4000);
                } else {
                    f8 *= k(0x96f9a0);
                }
                b4 *= cqe4_kf(0x96d3b8);
            }
            3 => {
                if !cf10 {
                    f8 *= k(0x96f998);
                }
                if pos20() {
                    f8 *= k(0x9689b0);
                }
            }
            4 => {
                if a0 & 0x10 != 0 {
                    f8 *= k(0x96f990);
                    if cqe4_d(p1 + 0x69) != 0 {
                        f8 *= k(0x96f988);
                    } else {
                        fl(0x8000);
                    }
                } else {
                    f8 *= if pos20() { k(0x968f20) } else { k(0x968fb0) };
                }
                if idade < 0x20 {
                    if idade > 0x17 {
                        f8 *= k(0x968968);
                    }
                    if cqe4_c(p1 + 0x22) != 0 {
                        f8 *= k(0x968968);
                    }
                }
                b4 *= cqe4_kf(0x96d38c);
            }
            5 => {
                if a0 & 0x10 != 0 {
                    f8 *= k(0x969020);
                    fl(0x8000);
                } else {
                    f8 *= if pos20() { k(0x968918) } else { k(0x968e00) };
                }
                if jovem {
                    f8 *= k(0x968968);
                }
                b4 *= cqe4_kf(0x96f980);
            }
            6 => {
                if a0 & 0x10 != 0 {
                    f8 *= k(0x968a98);
                    fl(0x8000);
                } else {
                    f8 *= if pos20() { k(0x967880) } else { k(0x968f20) };
                }
                if jovem {
                    f8 *= k(0x9689e0);
                }
                b4 *= cqe4_kf(0x968f18);
            }
            7 => f8 *= k(0x969048),
            _ => {}
        }
        // a reputação do clube novo, em várias escalas, contra a da pessoa
        let mut adv = cqe4_c(cqe4_d(p2 + 0x53) + 0x7e);
        if adv < 1 || adv > 3 {
            adv = 3;
        }
        ad = adv.wrapping_mul(adv);
        let h34 = cqe4_pelo_estadio(p2);
        h68 = cqe4_pela_divisao(p2);
        let (h6c, reg) = cqe4_pela_competicao(p2, h68);
        l84 = reg;
        let t = cqe4_d(p2 + 0xcf);
        let mut rv: i16 = 0;
        if t != 0 && cqe4_d(t + 0x69) != 0 {
            rv = vista(t, p1, 1);
            if cqe4_d(0xb67a44) != 0 {
                rv = cqe4_pelo_tecnico(p2, rv) as i16;
            }
        }
        let mut si: i16 = cqe4_h(p2 + 0x80);
        let fin = financas(cqe4_d(0xae2a38), p2);
        if fin != 0 && ad == 1 {
            si = cqe4_pela_faixa(faixa(fin) as i8, si);
        }
        let ax = cqe4_h(cqe4_d(this + 0xdc).wrapping_add(cqe4_d(p2).wrapping_mul(9)) + 5);
        let m = if si > ax { si } else { ax };
        let cx = ftol(si as f64 * k(0x96a5d0) + m as f64 * k(0x969510)) as i16;
        let den0 = ((b4 as f64 + a88 as f64) + aa8 as f64) + cqe4_kf(0x9689a8) as f64;
        den = if den0 > zero { den0 as f32 } else { 0.5 };
        let s = (((h6c as i16 as f64 * b4 as f64 + h68 as f64 * a88 as f64) + h34 as i16 as f64 * aa8 as f64) + cx as f64) + rv as f64;
        let mut w = ftol(s / den as f64) as i16;
        let cl = cqe4_b(p2 + 0x64);
        if cl != 1 {
            if !(cf10 && cqe4_b(p1 + 0x22) == 0) {
                w = cqe4_metade(w);
            }
            if cl == 3 {
                w = cqe4_metade(w);
            }
        }
        e8 = ftol(w as f64 * k(0x968940) - k(0x9688e8));
        if cqe4_d(p2 + 0x53) != 0 {
            let (m, f) = cqe4_fluencia(p1, p2, e8 as i16);
            if idade < 0x12 && f < k(0x9688a0) && cqe4_d(cqe4_d(p1 + 0x1a) + 0x71) != cqe4_d(cqe4_d(p2 + 0x53) + 0x71) {
                return zero;
            }
            f8 = m * f8;
            if !p6.is_null() && m < k(0x968e48) {
                *p6 |= 0x10000;
            }
        }
        // as nações, a distância e o que a pessoa quer (situação pedida)
        #[derive(PartialEq)]
        enum Segue {
            Padrao,
            Fatores,
        }
        let fav = cqe4_d(p1 + 0x39);
        let segue = 'n: {
            let cid2 = if cqe4_d(p2 + 0x69) != 0 { cqe4_d(cqe4_d(p2 + 0x69) + 0x38) } else { 0 };
            let cid4 = if d4 != 0 && cqe4_d(d4 + 0x69) != 0 { cqe4_d(cqe4_d(d4 + 0x69) + 0x38) } else { 0 };
            if fav != 0 && cqe4_d(fav + 0x53) == cqe4_d(p2 + 0x53) {
                if !p4b {
                    break 'n Segue::Padrao;
                }
                let mut g = f8;
                if na_lista_emp(p1, 0) != 0 && idade < 0x18 && r61 != 0 && cqe4_h(r61 + 0xb) < 0x109a && cqe4_c(p1 + 0x5b) > 0xf
                    && cqe4_c(p1 + 0x5a) > 0xa && c != 0 && !matches!(ea, 1 | 2 | 3 | 4)
                {
                    g *= k(0x968e18);
                }
                if d4 != 0 && cqe4_d(p2 + 0x53) != 0 && cqe4_d(cqe4_d(p2 + 0x53)) == cqe4_d(0x9cf360) && idade < 0x18
                    && cqe4_d(d4 + 0x57) != 0 && cqe4_d(cqe4_d(d4 + 0x57)) == cqe4_d(0x9cf570)
                {
                    g *= k(0x968930);
                }
                if cid2 != 0 && cid4 != 0 {
                    let dd = cqe4_distancia(cid2, cid4);
                    if dd < k(0x968e30) {
                        f8 = g * k(0x96f978);
                        fl(0x20000);
                    } else if dd < k(0x96d6e0) {
                        let m = k(0x96afa0) - dd * k(0x968a90);
                        f8 = m * g;
                        if !p6.is_null() && m > k(0x969018) {
                            *p6 |= 0x20000;
                        }
                    } else {
                        f8 = g * k(0x968de8);
                    }
                } else {
                    f8 = g * k(0x968de8);
                }
                break 'n Segue::Fatores;
            }
            if cqe4_d(p2 + 0x53) != 0 && estrangeira(p1, cqe4_d(p2 + 0x53), 1) == 0 {
                if !p4b {
                    break 'n Segue::Padrao;
                }
                if d4 == 0 {
                    break 'n Segue::Fatores;
                }
                if cid2 != 0 && cid4 != 0 {
                    let dd = cqe4_distancia(cid2, cid4);
                    if dd < k(0x968e30) {
                        f8 *= k(0x96f978);
                        fl(0x20000);
                    } else if dd < k(0x96d6e0) {
                        let m = k(0x96afa0) - dd * k(0x968a90);
                        f8 = m * f8;
                        if !p6.is_null() && m > k(0x969018) {
                            *p6 |= 0x20000;
                        }
                    } else {
                        f8 *= k(0x968de8);
                    }
                } else {
                    f8 *= k(0x968de8);
                }
                break 'n Segue::Fatores;
            }
            let t = cqe4_d(p2 + 0xcf);
            let mesmo_tecnico = t != 0 && cqe4_d(t + 0x1a) != 0 && estrangeira(p1, cqe4_d(t + 0x1a), 1) == 0;
            if cqe4_d(p2 + 0x53) != 0 && cqe4_d(cqe4_d(p2 + 0x53)) == cqe4_d(0x9cf314) {
                f8 *= k(0x969040);
            }
            if p4b {
                f8 *= if mesmo_tecnico { k(0x96a6a8) } else { k(0x968920) };
                fl(0x40000);
                break 'n Segue::Fatores;
            }
            let c56 = cqe4_c(p1 + 0x56) as i32;
            let c57 = cqe4_c(p1 + 0x57) as i32;
            if mesmo_tecnico {
                match p5b {
                    2 => f8 = ((c56 as f64 * k(0x968ef0) + c57 as f64 * k(0x968a98)) + k(0x9689b0)) * f8,
                    3 => f8 = ((c56 as f64 * k(0x968ef0) + c57 as f64 * k(0x968a98)) + k(0x968e40)) * f8,
                    4 => {
                        f8 *= k(0x968968);
                        fl(0x80000);
                    }
                    5 => {
                        f8 *= k(0x968e00);
                        fl(0x80000);
                    }
                    6 => {
                        f8 *= k(0x968918);
                        fl(0x80000);
                    }
                    _ => {}
                }
            } else {
                match p5b {
                    2 => f8 = ((c56 + c57) as f64 * k(0x967898) + k(0x968e40)) * f8,
                    3 => f8 = ((c56 + c57) as f64 * k(0x967898) + k(0x968e48)) * f8,
                    4 => {
                        f8 *= k(0x9689e0);
                        fl(0x80000);
                    }
                    5 => {
                        f8 = ((c56 + c57) as f64 * k(0x968d60) + k(0x9689e0)) * f8;
                        fl(0x80000);
                    }
                    6 => {
                        f8 *= k(0x967878);
                        fl(0x80000);
                    }
                    _ => {}
                }
            }
            Segue::Padrao
        };
        if segue == Segue::Padrao {
            if (c != 0 && cqe4_b(c + 0x49) != 0) || na_lista(p1, 0) != 0 {
                f8 *= k(0x968e18);
                if f8 < k(0x968950) && (cqe4_h(p2 + 0x80) > 0x1a5e || cqe4_h(p2 + 0x80) > cqe4_h(d4 + 0x80)) {
                    let t = f8 + k(0x967890);
                    f8 = if k(0x968950) < t { 2.0 } else { t };
                }
            }
        } else {
            if na_lista_emp(p1, 0) != 0 {
                f8 = (k(0x968940) - c1 as f64 * k(0x967898)) * f8;
            }
            let mut m = k(0x967890) - c1 as f64 * k(0x96d370);
            if m < k(0x967880) {
                m = k(0x967880);
            }
            f8 = m * f8;
            if !p6.is_null() && m < k(0x968e48) {
                *p6 |= 0x40000;
            }
            let a = if pos20() {
                (0x21 - idade as i32) as f64
            } else {
                let a = (0x1c - idade as i32) as f64;
                if a < zero { a * k(0x968e00) } else { a }
            };
            let a = a.abs() * k(0x968f00) + k(0x9689e0);
            f8 = a * f8;
            if !p6.is_null() && a < k(0x968e48) {
                *p6 |= 0x40000;
            }
        }
        if d4 == 0 {
            f8 *= k(0x968f98);
        } else if na_lista_transf(p1, 0) == 0 && na_lista_emp(p1, 0) == 0 && (c == 0 || cqe4_b(c + 0x49) == 0) {
            let dd = ((cqe4_w(0xae2c92) as i32 - cqe4_w(p1 + 0x40) as i32) * 365 - cqe4_h(p1 + 0x3e) as i32) + cqe4_h(0xae2c90) as i32;
            let e: i32 = if p4b {
                (20 - c1 as i32 / 5).wrapping_mul(dd)
            } else {
                if cqe4_d(c + 0x45) == 0 && dd < (cqe4_c(p1 + 0x59) as i32 + 8) * 15 {
                    return zero;
                }
                let dv = cqe4_c(p1 + 0x59) as i32 + 10;
                dd.wrapping_mul(20) / if dv < 1 { 1 } else { dv }
            };
            if e < 0 {
                return zero;
            }
            if e > 0x190 {
                l80 = 400.0 * k(0x969028);
            } else {
                if e == 0 {
                    return zero;
                }
                l80 = e as f64 * k(0x969028);
            }
        }
    } else {
        // o próprio clube: a situação no elenco pela pedida, com o técnico há mais de 150 dias
        if humano(p2, 0, 1, 0, 0) != 0 && cqe4_depois_de(cqe4_d(p2 + 0xcf)) {
            let lim = 0xceu8.wrapping_sub(cqe4_b(p1 + 0x59)) as i8;
            if c1 < lim && ea != 5 && ea != 6 {
                ea = 7;
            } else if c1 < 0 {
                if ea < 4 {
                    ea = 4;
                }
            } else if c1 < 0x32 {
                if ea < 3 {
                    ea = 3;
                }
            } else if c1 < 0x4b {
                if ea < 2 {
                    ea = 2;
                }
            } else if ea < 1 {
                ea = 1;
            }
        }
        match ea {
            1 => f8 *= k(0x968978),
            3 => f8 *= k(0x9689d8),
            4 => {
                f8 *= k(0x969520);
                if idade < 0x20 {
                    if idade > 0x17 {
                        f8 *= k(0x968968);
                    }
                    if cqe4_c(p1 + 0x22) != 0 {
                        f8 *= k(0x968968);
                    }
                }
            }
            5 => {
                f8 *= k(0x969040);
                if jovem {
                    f8 *= k(0x968968);
                }
            }
            6 => {
                f8 *= k(0x968e40);
                if jovem {
                    f8 *= k(0x9689e0);
                }
            }
            7 => f8 *= k(0x967898),
            _ => {}
        }
    }
    // a reputação do clube atual (ou da pessoa, sem clube) na mesma escala
    if d4 == 0 {
        let mut aev = cqe4_c(cqe4_d(p1 + 0x1a) + 0x7e);
        if aev < 1 || aev > 3 {
            aev = 3;
        }
        ae = aev.wrapping_mul(aev);
        sb = ftol(cqe4_media3(cqe4_d(p1 + 0x61)) as i16 as f64 * k(0x968940) - k(0x9688e8)) as i16;
    } else {
        let n4 = cqe4_d(d4 + 0x53);
        let mut aev = if n4 != 0 { cqe4_c(n4 + 0x7e) } else { 3 };
        if aev < 1 || aev > 3 {
            aev = 3;
        }
        ae = aev.wrapping_mul(aev);
        let h44 = cqe4_pelo_estadio(d4);
        h6c4 = cqe4_pela_divisao(d4) as i32;
        let (bx, reg) = cqe4_pela_competicao(d4, h6c4 as i16);
        l8c = reg;
        let t4 = cqe4_d(d4 + 0xcf);
        let si: i16 = if t4 == 0 || cqe4_d(t4 + 0x69) == 0 {
            ftol(h6c4 as i16 as f64 * k(0x969040)) as i16
        } else if cqe4_d(0xb67a44) == 0 {
            cqe4_h(cqe4_d(t4 + 0x69) + 0xa)
        } else {
            cqe4_pelo_tecnico(d4, cqe4_h(cqe4_d(t4 + 0x69) + 0xa)) as i16
        };
        let mut di: i16 = cqe4_h(d4 + 0x80);
        let fin = financas(cqe4_d(0xae2a38), d4);
        if fin != 0 && ae == 1 && c != 0 && cqe4_d(c + 0x45) == 0 {
            let m = faixa(fin);
            if m == 0xfe {
                fl(0x40000000);
            } else {
                di = cqe4_pela_faixa(m as i8, di);
            }
        }
        let ax = cqe4_h(cqe4_d(this + 0xdc).wrapping_add(cqe4_d(d4).wrapping_mul(9)) + 5);
        let m = if di > ax { di } else { ax };
        let axv = ftol(di as f64 * k(0x96a5d0) + m as f64 * k(0x969510)) as i16;
        // ponytail: no próprio clube (p2 == d4) a original divide pelo float [base+0x58] que ninguém gravou (sobra de
        // pilha); o resultado não é usado depois (o caminho vai direto ao fim), e a cópia divide por 1.
        let s = (((bx as i16 as f64 * b4 as f64 + h6c4 as i16 as f64 * a88 as f64) + h44 as i16 as f64 * aa8 as f64) + axv as f64) + si as f64;
        let mut di = ftol(s / den as f64) as i16;
        if !p4b && na_lista(p1, 0) != 0 {
            di = ftol(di as f64 * k(0x9689d8)) as i16;
        }
        let cl = cqe4_b(d4 + 0x64);
        if cl != 1 {
            if !(c != 0 && cqe4_b(c + 0x45) & 0x10 != 0 && cqe4_b(p1 + 0x22) == 0) {
                di = cqe4_metade(di);
            }
            if cl == 3 {
                di = cqe4_metade(di);
            }
        }
        if lugar(cqe4_d(0xae1050), p1, d4) == 3 {
            di = ftol(di as f64 * k(0x968e40)) as i16;
        }
        sb = ftol(di as f64 * k(0x968940) - k(0x9688e8)) as i16;
        if n4 != 0 && p2 != d4 {
            let (m, _) = cqe4_fluencia(p1, d4, sb);
            f8 = f8 / m;
        }
    }
    if p2 != d4 {
        let comp2 = cqe4_d(p2 + 0x57);
        if comp2 == 0 || cqe4_d(comp2) == cqe4_d(0x9cf764) {
            f8 *= k(0x967880);
        } else if cqe4_d(p1 + 0x39) == p2 && r61 != 0 && cqe4_c(r61 + 0x45) > 0xa {
            f8 = cqe4_c(r61 + 0x45) as i32 as f64 * f8 * k(0x967880);
        }
        let mut a8 = (cqe4_h(p2 + 0x80) as i32 - cqe4_media3(r61)) as f64 * k(0x96a6b8) + k(0x967890);
        let e8s = e8 as i16 as i32;
        let sbs = sb as i32;
        let cf10 = c != 0 && cqe4_b(c + 0x45) & 0x10 != 0;
        let mut longo = false;
        if p4b && (cf10 || na_lista_emp(p1, 0) != 0) {
            let f = cqe4_d(p1 + 0x61);
            longo = (cqe4_h(f + 5) as i32 * 50) < cqe4_h(d4 + 0x80) as i32
                && cqe4_c(p1 + 0x22) == 0
                && !(idade >= 0x18 && idade <= 0x20)
                && cqe4_h(p2 + 0x80) > 0xdac
                && cqe4_h(f + 0xb) < 0x157c
                && cqe4_d(p2 + 0x53) != 0
                && cqe4_d(p2 + 0x57) != 0
                && cqe4_d(d4 + 0x57) != 0
                && !(cqe4_h(cqe4_d(p2 + 0x57) + 0x69) <= 6 && cqe4_h(cqe4_d(p2 + 0x57) + 0x69) as i32 <= cqe4_h(cqe4_d(d4 + 0x57) + 0x69) as i32 - 3)
                && cqe4_d(d4 + 0x53) != 0
                && !(cqe4_c(cqe4_d(p2 + 0x53) + 0x85) < 0xa && cqe4_c(cqe4_d(p2 + 0x53) + 0x85) < cqe4_c(cqe4_d(d4 + 0x53) + 0x85));
        }
        let dv: f64 = if longo {
            (e8s - sbs) as f64 * k(0x968968)
        } else if cf10 && e8s < sbs && sbs - e8s < 0x1388 {
            let d = (sbs - e8s) as f64;
            d * k(0x96f970) * d
        } else if e8s > sbs {
            if c != 0 && cqe4_b(c + 0x45) & 1 != 0 {
                (e8s - sbs) as f64 * k(0x968e18)
            } else {
                (cqe4_c(p1 + 0x57) as i32 as f64 * k(0x96f930) + k(0x969040)) * (e8s - sbs) as f64
            }
        } else if p5b > 3 {
            (e8s - sbs) as f64 * k(0x968940)
        } else if !(f8 > k(0x9688b0)) {
            (e8s - sbs) as f64
        } else {
            let v = if sbs > 0x1b58 && e8s < 0x7d0 && p4b {
                let z = cqe4_kf(0x9689a8) as f64 - (sbs - e8s) as f64 * cqe4_kf(0x96f968) as f64;
                let mn = if f8 < k(0x968ff8) { f8 } else { k(0x968ff8) };
                let zm = z * mn;
                if k(0x9688b0) > zm {
                    k(0x9688b0) * k(0x967898)
                } else {
                    zm * k(0x967898)
                }
            } else {
                f8 * k(0x967898)
            };
            let mut w = k(0x9689c0) - v;
            if w < k(0x967880) {
                w = k(0x967880);
            } else if !(w <= k(0x967890)) {
                w = k(0x967890);
            }
            w * (e8s - sbs) as f64
        };
        if dv > zero && l80 < k(0x967890) {
            let mut t = dv * k(0x968a98);
            if k(0x9688a8) < t {
                t = k(0x9688a8);
            }
            let r = ((k(0x9688a8) - t) * l80 + t) * k(0x967898);
            f8 = f8 * r;
            if !p6.is_null() && r < k(0x968e48) {
                *p6 |= 0x100000;
            }
        }
        ramo("cqe_4 fator da reputacao");
        let mut e0 = dv * k(0x96f960) + k(0x967890); // cqe4: o fator da reputação comparada
        if e0 < k(0x96a350) {
            e0 = f64::from_bits(0x3f50624dd2f1a9fc);
        }
        if c != 0 && cqe4_b(c + 0x46) & 1 != 0 {
            e0 *= k(0x968940);
        }
        let mut x = e0 * f8;
        let sem_pedida = |c: usize| c == 0 || cqe4_b(c + 0x49) == 0;
        if !p6.is_null() {
            if e0 < k(0x968e48) {
                let ok = b4 as f64 > k(0x968978) && l8c != 0 && l84 == 0 && x > k(0x968928) && ea < 4 && ea > 0 && sem_pedida(c);
                *p6 |= if ok { 0x400000 } else { 0x800000 };
            } else if e0 > k(0x968940) && b4 as f64 > k(0x968978) && l8c == 0 && l84 != 0
                && !(d4 != 0 && cqe4_h(d4 + 0x80) >= 0x1d4c) && p5b < 4 && p5b > 0 && sem_pedida(c)
            {
                *p6 |= 0x200000;
            }
        }
        if !p4b {
            x *= e0;
        }
        if c != 0 && cqe4_b(c + 0x45) & 1 != 0 {
            x *= e0;
            if !p6.is_null() {
                if e0 < k(0x9689b0) {
                    let ok = b4 as f64 > k(0x968978) && l8c != 0 && l84 == 0 && x > k(0x968928) && ea < 4 && ea > 0 && cqe4_b(c + 0x49) == 0;
                    *p6 |= if ok { 0x400000 } else { 0x800000 };
                } else if e0 > k(0x968950) && b4 as f64 > k(0x968978) && l8c == 0 && l84 != 0
                    && !(d4 != 0 && cqe4_h(d4 + 0x80) >= 0x1d4c) && p5b < 4 && p5b > 0 && cqe4_b(c + 0x49) == 0
                {
                    *p6 |= 0x200000;
                }
            }
        }
        if p4b {
            if cqe4_h(r61 + 0xb) > sb && sbs > e8s {
                a8 = ((e8s - sbs) as f64 * k(0x96a600) + k(0x967890)) * a8;
                if a8 < k(0x967880) {
                    a8 = f64::from_bits(0x3fb999999999999a);
                }
            }
            let r = ae as i32 as f64 / ad as i32 as f64;
            x = x * r;
            if !p6.is_null() && r < k(0x968e48) {
                *p6 |= 0x1000000;
            }
        } else if d4 != 0 {
            if ae == ad {
                let t = cqe4_d(cqe4_d(p1 + 0x1a) + 0x71);
                if t != 0 && cqe4_d(t) == cqe4_d(0x9cfa10) {
                    let u = cqe4_d(cqe4_d(p2 + 0x53) + 0x71);
                    if u != 0 && cqe4_d(u) != cqe4_d(0x9cfa10) && u != cqe4_d(cqe4_d(d4 + 0x53) + 0x71) {
                        x *= k(0x968e00);
                        fl(0x2000000);
                    }
                }
            } else {
                let v6c = h6c4 as i16;
                let wv = if v6c > 0x1f4 { v6c as i32 } else { 500 };
                let mut q = (ae as i32 as f64 * h68 as f64) / (wv as f64 * ad as i32 as f64);
                if !(k(0x967898) <= q) {
                    q = k(0x967898);
                } else if k(0x9688b0) < q {
                    q = k(0x9688b0);
                }
                x = x * q;
                if cqe4_d(cqe4_d(p2 + 0x53) + 0x71) != cqe4_d(cqe4_d(d4 + 0x53) + 0x71) && q < k(0x967890) {
                    x *= k(0x968918);
                    fl(0x2000000);
                }
            }
        }
        f8 = a8 * x;
        if !p6.is_null() && a8 < k(0x968e48) {
            *p6 |= 0x800000;
        }
    }
    // jovem com o fim do contrato antes do fim da temporada do clube atual: não sai
    if d4 != 0 && d4 != p2 {
        let s7 = cqe4_h(cqe4_d(p1 + 0x61) + 7);
        if s7 > 0x82 && s7 as i32 > cqe4_h(d4 + 0x80) as i32 / 60 && idade < 0x1a
            && na_temporada(d4, data_a.as_mut_ptr() as usize, data_b.as_mut_ptr() as usize) != 0 && c1 > 0 && c != 0
        {
            let ano = u16::from_le_bytes([data_b[2], data_b[3]]);
            let dia = i16::from_le_bytes([data_b[0], data_b[1]]);
            let ano_c = cqe4_w(c + 0x2f);
            let antes = ano < ano_c || (ano == ano_c && dia < cqe4_h(c + 0x2d));
            if antes && cqe4_d(c + 0x45) == 0 && cqe4_b(c + 0x49) == 0
                && !(humano(p2, 1, 1, 0, 0) != 0 && cqe4_c(p1 + 0x5a) <= 0xf)
                && dias_ate(data_b.as_mut_ptr() as usize, cqe4_d(0xae2c90) as u32, cqe4_d(0xae2c94) as u32) < 0x96
            {
                fl(0x80000000);
                return zero;
            }
        }
    }
    if p3 == 0 || p4b || f8 < k(0x968f98) {
        return f8;
    }
    // o salário: o padrão (num STAFF_CONTRACT local) contra o teto do clube
    ramo("cqe_4 salario");
    let mut contrato_local = [0u32; 0x14];
    // 0x85e410: lea ecx, [base+0xa0] -> thiscall, ret 0
    let construir: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x85e410usize);
    construir(contrato_local.as_mut_ptr() as usize);
    let fin = financas(cqe4_d(0xae2a38), p2);
    if fin == 0 {
        return zero;
    }
    // 0x864550: push 0; push 0xb; push situação; push 0; push clube; push pessoa; lea ecx, contrato -> thiscall, ret 0x18
    let padrao: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32) -> f64 = core::mem::transmute(0x864550usize);
    let w0 = padrao(contrato_local.as_mut_ptr() as usize, p1, p2, 0, p5, 0xb, 0);
    // 0x5968d0: push situação; push pessoa; mov ecx, finanças -> thiscall, ret 8
    let teto: unsafe extern "thiscall" fn(usize, usize, u32) -> i32 = core::mem::transmute(0x5968d0usize);
    let a8w = teto(fin, p1, p5) as f64;
    let mut e8w = 0.0f64;
    let base = if !(k(0x967880) <= f8) { k(0x967880) } else { f8 };
    let mut q = k(0x968e38) / base; // cqe4: a pedida pelo interesse
    if q < k(0x9689d8) {
        q = f64::from_bits(0x3feccccccccccccd);
    }
    let mut wv = q * w0;
    if c != 0 {
        if a0 & 0x100 != 0 {
            e8w = 10.0;
        }
        if na_lista_transf(p1, d4) != 0 {
            e8w = e8w + k(0x9688b0);
        }
        if cqe4_c(c + 0x3d) < 0 {
            e8w = e8w - cqe4_c(c + 0x3d) as i32 as f64 * k(0x968918);
        }
        if a0 != 0 {
            e8w = e8w - cqe4_c(c + 0x44) as i32 as f64 * k(0x968918);
        }
        if !(zero <= e8w) {
            e8w = 0.0;
        } else if k(0x968ea8) < e8w {
            e8w = 20.0;
        }
    }
    let mut f = f8;
    if !(wv * k(0x96a5d0) <= a8w) {
        return zero;
    }
    if !(wv <= a8w) {
        let mut t = wv * k(0x968968);
        if !(a8w <= t) {
            t = a8w;
        }
        wv = (wv * (k(0x968ea8) - e8w) + t * e8w) * k(0x967888);
    }
    if !(wv <= a8w) && !(wv <= zero) {
        let r = a8w / wv;
        if r < k(0x968e40) {
            return k(0x967880);
        }
        f = f * r;
        wv = a8w;
    }
    if c == 0 {
        return f;
    }
    let mut g = cqe4_d(c + 0xc) as i32 as f64;
    if g < k(0x9688b0) {
        g = k(0x9688b0);
    }
    let mut h = if d4 == p2 { g / wv } else { wv / g };
    if !(k(0x967880) <= h) {
        h = k(0x967880);
    } else if k(0x9688b0) < h {
        return f * k(0x9688b0);
    } else if !(h < k(0x967890)) {
        return f * h;
    }
    f * ((((k(0x968ea8) - e8w) * h + e8w) * k(0x967888)) / q)
}

/// Comparador: lado a lado na nota (os bits do f64), nos motivos e em [0xae1080] (que a original e o salário padrão
/// 0x864550 gravam); o resto das chamadas só lê, e o STAFF_CONTRACT do salário é local.
unsafe extern "thiscall" fn comparar_cqe4_interesse(this: usize, p1: usize, p2: usize, p3: u32, p4: u32, p5: u32, p6: *mut u32) -> f64 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, *mut u32) -> f64>(1584);
    let mut regioes = vec![(0xae1080usize, 4usize)];
    if !p6.is_null() {
        regioes.push((p6 as usize, 4));
    }
    let (ro, rr, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, p1, p2, p3, p4, p5, p6), || cqe4_interesse(this, p1, p2, p3, p4, p5, p6));
    PLACAR.registrar(igual && ro.to_bits() == rr.to_bits(), || format!("cqe4_interesse(pessoa {:#x}, clube {:#x}, {}, {}, {}) original={} {:x?} rust={} {:x?}", p1, p2, p3, p4 as u8, p5 as u8, ro, mo, rr, mr));
    ro
}

/// { 0x85e410 } (thiscall, ret 0, staff_contracts.cpp): `STAFF_CONTRACT::STAFF_CONTRACT()`: as duas datas (+0x25,
/// +0x2d) padrão (0x54a110), +4 e +0x4a = −1, +0x49, +0x3b, +0x4e, +0x40, +0x45 e +8 = 0. Devolve o próprio contrato.
unsafe extern "thiscall" fn cqe4_construir_contrato(this: usize) -> usize {
    let data_padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    data_padrao(this + 0x25);
    data_padrao(this + 0x2d);
    core::ptr::write_unaligned((this + 4) as *mut u32, u32::MAX);
    core::ptr::write_unaligned((this + 0x4a) as *mut u32, u32::MAX);
    *((this + 0x49) as *mut u8) = 0;
    core::ptr::write_unaligned((this + 0x3b) as *mut u16, 0);
    *((this + 0x4e) as *mut u8) = 0;
    core::ptr::write_unaligned((this + 0x40) as *mut u32, 0);
    core::ptr::write_unaligned((this + 0x45) as *mut u32, 0);
    core::ptr::write_unaligned((this + 8) as *mut u32, 0);
    this
}

/// Comparador: lado a lado nos 0x4f bytes do contrato e na devolução.
unsafe extern "thiscall" fn comparar_cqe4_construir_contrato(this: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> usize>(1585);
    let (ro, rr, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this, 0x4f)], || o(this), || cqe4_construir_contrato(this));
    PLACAR.registrar(igual && ro == rr, || format!("cqe4_construir_contrato({:#x}) original={:x?} rust={:x?}", this, mo, mr));
    ro
}

// ---------- 267a leva: a troca com compensação, a ordem das vagas da lista de compras e o jogador sob oferta (shortlist_manager.cpp) ----------

unsafe fn cqd12_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqd12_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

/// Contador de ramo para a rotina quente (~27 milhões de chamadas até 2002-06-01): o `ramo` só a cada 4096 passagens
/// (a primeira inclusive), para não pesar na carreira.
fn cqd12_contar(n: &core::sync::atomic::AtomicU32, nome: &'static str) {
    // ponytail: rotina quente; o contador do ramo conta de 4096 em 4096
    if n.fetch_add(1, core::sync::atomic::Ordering::Relaxed) & 0xfff == 0 {
        crate::comparacao::ramo(nome);
    }
}

static CQD12_BOSMAN: core::sync::atomic::AtomicU32 = core::sync::atomic::AtomicU32::new(0);
static CQD12_FAIXA: core::sync::atomic::AtomicU32 = core::sync::atomic::AtomicU32::new(0);
static CQD12_LONGE: core::sync::atomic::AtomicU32 = core::sync::atomic::AtomicU32::new(0);

/// { 0x84b6f0 } (thiscall, ret 0x10, shortlist_manager.cpp; o this não é lido): `shortlist_manager_is_compensation_deal(
/// vaga, contrato, clube, &taxa)`, pela vaga da lista de compras (0x842e10, 0x840000), pelo relatório do olheiro
/// (0x7b62a0), pelo lance (0x8e5d60) e pela tela. Zera a taxa; sem contrato, sem vaga ou com o byte +0x49 do contrato:
/// 1. A pessoa do contrato ([0xae23bc] + 110 × [+0]). Fora do período protegido da FIFA (0x868210 zero): a taxa de
/// compensação (0x8d0780 com 0) e 1 se ela é menor que o valor (+0x52) da pessoa. Senão: sem lei Bosman no clube do
/// jogador (0x8d0a70), 0; a taxa (0x8d0780 com 1) acima do valor, 0; pela faixa do vencimento (0x867d30): 0 → 0; 1-2,
/// 3-4, 5-6, 7-8 → 1 se o +3 da vaga × 0,5 / 0,65 / 0,8 / 0,9 passa do preço (+0xb); 9-14 → 1; acima, 0.
unsafe extern "thiscall" fn troca_com_compensacao(this: usize, vaga: usize, contrato: usize, clube: usize, taxa: *mut i32) -> u32 {
    let _ = this;
    *taxa = 0;
    if contrato == 0 || vaga == 0 || *((contrato + 0x49) as *const u8) != 0 {
        return 1;
    }
    let pessoa = |c: usize| cqd12_d(0xae23bc).wrapping_add(cqd12_d(c).wrapping_mul(0x6e));
    let protegido: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32 = core::mem::transmute(0x868210usize);
    let compensacao: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> i32 = core::mem::transmute(0x8d0780usize);
    let bosman: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x8d0a70usize);
    let faixa: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x867d30usize);
    // 0x868210: push 0; push 0; push clube; mov ecx, contrato -> thiscall; 0x8d0780 e 0x8d0a70: mov ecx, [0xde1f54]
    // antes dos pushes (a cópia passa o ecx como a original; as duas só leem a pilha, ret 0xc e ret 4); 0x867d30: mov
    // ecx, contrato -> thiscall
    let gerente = cqd12_d(0xde1f54);
    if protegido(contrato, clube, 0, 0) == 0 {
        let t = compensacao(gerente, pessoa(contrato), clube, 0);
        *taxa = t;
        return (t < cqd12_i(pessoa(contrato) + 0x52)) as u32;
    }
    if bosman(gerente, pessoa(contrato)) == 0 { // cqd12: sem a lei Bosman
        return 0;
    }
    cqd12_contar(&CQD12_BOSMAN, "cqd_12 com lei Bosman");
    let t = compensacao(gerente, pessoa(contrato), clube, 1);
    *taxa = t;
    if t > cqd12_i(pessoa(contrato) + 0x52) {
        return 0;
    }
    let k = faixa(contrato) as i8 as i32;
    if k as u32 > 0xe {
        return 0;
    }
    let fator = match k {
        0 => return 0,
        1 | 2 => 0.5, // cqd12: a faixa 1-2
        3 | 4 => 0.65,
        5 | 6 => 0.8,
        7 | 8 => 0.9,
        _ => {
            cqd12_contar(&CQD12_LONGE, "cqd_12 faixa longe");
            return 1; // cqd12: as faixas 9-14
        }
    };
    cqd12_contar(&CQD12_FAIXA, "cqd_12 faixa pelo fator");
    // fild [vaga+0xb]; fild [vaga+3]; fmul fator; fcompp: 1 se o +3 × fator passa do +0xb (precisão dupla)
    (cqd12_i(vaga + 3) as f64 * fator > cqd12_i(vaga + 0xb) as f64) as u32
}

/// Comparador: lado a lado (rotina pura: só a taxa e a devolução), pelas cópias leves.
unsafe extern "thiscall" fn comparar_troca_com_compensacao(this: usize, vaga: usize, contrato: usize, clube: usize, taxa: *mut i32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut i32) -> u32>(1573);
    let reg = [(taxa as usize, 4)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, vaga, contrato, clube, taxa),
        || troca_com_compensacao(this, vaga, contrato, clube, taxa));
    PLACAR.registrar(igual && r == n, || format!("troca_com_compensacao(vaga {:#x}, contrato {:#x}, clube {:#x}) original={} rust={} {:x?} x {:x?}",
        vaga, contrato, clube, r, n, mo, mr));
    r
}

/// { 0x83b6c0 } (cdecl, shortlist_manager.cpp): `shortlist_compare_slots_by_index(a, b)`, o critério do qsort das 40
/// vagas (0x25 bytes) da lista de compras na avaliação do elenco (0x83cb40): a diferença dos índices (+0x23, byte com
/// sinal); iguais, 1 (nunca empata).
unsafe extern "C" fn ordem_das_vagas(a: usize, b: usize) -> i32 {
    let d = (*((a + 0x23) as *const i8) as i32) - (*((b + 0x23) as *const i8) as i32);
    if d != 0 { d } else { 1 }
}

/// Comparador: a devolução (pura).
unsafe extern "C" fn comparar_ordem_das_vagas(a: usize, b: usize) -> i32 {
    let o = orig::<unsafe extern "C" fn(usize, usize) -> i32>(1574);
    let r = o(a, b);
    let n = ordem_das_vagas(a, b);
    PLACAR.registrar(r == n, || format!("ordem_das_vagas({:#x}, {:#x}) original={} rust={}", a, b, r, n));
    r
}

/// Os clubes interessados no jogador, como a 0x84b580 os pede (0x842180 com modo 1 numa lista local; os itens de 8
/// bytes: id do jogador, id do clube).
unsafe fn cqd12_interessados(this: usize, jogador: usize) -> Vec<usize> {
    let nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let juntar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) -> u32 = core::mem::transmute(0x842180usize);
    let mut l = [0u8; 0x10];
    let lista = l.as_mut_ptr() as usize;
    nova(lista);
    juntar(this, jogador, lista, 0, 1, 0);
    let mut v = Vec::new();
    let mut no = cqd12_d(lista + 2);
    while no != 0 && v.len() < 0x10000 {
        v.push(cqd12_d(cqd12_d(no) + 4));
        no = cqd12_d(no + 6);
    }
    soltar(lista);
    v
}

/// { 0x84b580 } (thiscall, ret 0xc, shortlist_manager.cpp, quadro SEH da lista local 0x68d020/0x68d040):
/// `Shortlist_Manager::mark_player_under_offer(jogador, comprador, troca)`, pela negociação acrescentada (0x8be9d0) e
/// pelo lance aceito (0x8c0080). Os clubes interessados no jogador (0x842180 com modo 1, numa lista local), menos os de
/// humano que decide (0x600a50 com 0, 1): com a lista `troca` (cursor na cabeça), um clube de alguma oferta dela
/// (0x8c61e0 pelo id do item; o clube +8 da oferta) fica de fora; senão a vaga do jogador (0x849ce0, só de tipo 11 se o
/// byte +0x11 da última oferta achada é 11) ganha o bit 0x20 (+0x24); sem a lista, fora o comprador, a vaga de tipo 11
/// ou, sem ela, a de outro tipo. Clube sem vaga encerra.
unsafe extern "thiscall" fn marcar_sob_oferta(this: usize, jogador: usize, comprador: usize, troca: usize) {
    let nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let juntar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) -> u32 = core::mem::transmute(0x842180usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let oferta: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x8c61e0usize);
    let vaga_de: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(0x849ce0usize);
    // 0x68d020/0x68d040/0x68d1b0: lea ecx, lista -> thiscall; 0x842180: 5 pushes; mov ecx, ebx (ret 0x14); 0x600a50:
    // 5 pushes; add esp, 0x14 -> cdecl; 0x8c61e0: push id; mov ecx, [0xde1f54] (ret 4); 0x849ce0: 4 pushes; mov ecx,
    // ebx (ret 0x10)
    let mut l = [0u8; 0x10];
    let lista = l.as_mut_ptr() as usize;
    nova(lista);
    juntar(this, jogador, lista, 0, 1, 0);
    core::ptr::write_unaligned((lista + 0xa) as *mut usize, cqd12_d(lista + 2));
    let mut ultima = jogador; // o ebp da original: o jogador, depois a última oferta achada (vale entre os clubes)
    let mut item = proximo(lista);
    'clubes: while item != 0 {
        let k = cqd12_d(item + 4);
        let clube = cqd12_d(0xae23b4).wrapping_add(k.wrapping_mul(0x245));
        if humano(clube, 0, 1, 0, 0) == 0 {
            let v = if troca != 0 {
                core::ptr::write_unaligned((troca + 0xa) as *mut usize, cqd12_d(troca + 2));
                let mut q = proximo(troca);
                while q != 0 {
                    ultima = oferta(cqd12_d(0xde1f54), cqd12_d(q));
                    if ultima != 0 && cqd12_d(cqd12_d(ultima) + 8) == k {
                        item = proximo(lista);
                        continue 'clubes;
                    }
                    q = proximo(troca);
                }
                let tipo = *((cqd12_d(ultima) + 0x11) as *const u8);
                vaga_de(this, clube, jogador, (tipo != 0xb) as u32, 0)
            } else {
                if clube == comprador {
                    item = proximo(lista);
                    continue 'clubes;
                }
                let v = vaga_de(this, clube, jogador, 0, 0);
                if v != 0 { v } else { vaga_de(this, clube, jogador, 1, 0) }
            };
            if v == 0 {
                break;
            }
            *((v + 0x24) as *mut u8) |= 0x20;
        }
        item = proximo(lista);
    }
    soltar(lista);
}

/// Comparador: lado a lado pelas listas de 40 vagas dos clubes interessados (a cópia e a original montam e soltam a
/// mesma lista local) e pelo cursor da lista `troca`.
unsafe extern "thiscall" fn comparar_marcar_sob_oferta(this: usize, jogador: usize, comprador: usize, troca: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(1575);
    let mut reg: Vec<(usize, usize)> = cqd12_interessados(this, jogador).into_iter()
        .map(|k| cqd12_d(cqd12_d(this + 0xdc).wrapping_add(k.wrapping_mul(9))))
        .filter(|&t| t != 0)
        .map(|t| (t, 40 * 0x25))
        .collect();
    if troca != 0 {
        reg.push((troca + 0xa, 4));
    }
    let ((), (), igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, jogador, comprador, troca),
        || marcar_sob_oferta(this, jogador, comprador, troca));
    PLACAR.registrar(igual, || format!("marcar_sob_oferta(jogador {:#x}, comprador {:#x}, troca {:#x}) {:x?} x {:x?}",
        jogador, comprador, troca, mo, mr));
}

// ---------- 263a leva: o valor do jogador para o clube (a nota pela vaga da lista de observação e o preço que o clube pagaria), a entrada do jogador na busca do clube observado e o prêmio por jogos na seleção achado (shortlist_manager.cpp, scout_manager.cpp, transfer_manager.cpp) ----------

unsafe fn cqa7_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqa7_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqa7_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// A constante double do exe em `a` (lida do próprio exe para ser a mesma, bit a bit, que o x87 da original carrega).
unsafe fn cqa7_k(a: usize) -> f64 {
    core::ptr::read_unaligned(a as *const f64)
}

/// O fim sem valor da 0x840d80 (0x841f62): fora da lista de empréstimos da vaga (bit 0 de +0x24), 0; com ela e a pessoa
/// listada para empréstimo (0x544590 com 0), a nota vira max(nota + 5000, nota × 1,2) (_ftol) e 0.
unsafe fn cqa7_sem_valor(p: usize, vaga: usize, saida: *mut i32) -> i32 {
    if cqa7_b(vaga + 0x24) & 1 == 0 {
        return 0;
    }
    let emprestimo: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x544590usize);
    if emprestimo(p, 0) != 0 {
        let r = *saida;
        let a = r.wrapping_add(5000) as f64;
        let q = r as f64 * cqa7_k(0x968978);
        *saida = ftol(if a > q { a } else { q });
    }
    0
}

/// O ajuste pela habilidade vista dos jogadores jovens (as vagas de tipo 1, 2 e 3 em +0x24 >> 1): com menos de 27 anos
/// (+0x18 da pessoa) e nota acima de 20000, a habilidade atual vista pelo técnico do clube (0x6e86d0 com 1 e nota/4)
/// × 220 / nota, limitada a `frac` + 0,6 e puxada para 1 aos 24, 25 e 26 anos; com 1,05 × mínimo abaixo da nota ou a
/// vaga de nota alta (`alta`), acima de 1 ela ainda vai pela reputação do clube na tabela [this+0xdc] (9 bytes por
/// clube, palavra +5) sobre a reputação do clube (+0x80, pelo menos 1000), entre 1 e 1,2. Acima de 1: `frac` × ela, e o
/// valor base e a nota × (ela + 1) / 2.
unsafe fn cqa7_jovem(this: usize, clube: usize, p: usize, saida: *mut i32, minimo: i32, alta: bool, idade: i8, r: i32, frac: &mut f64, e: &mut u32) {
    if idade >= 0x1b || r <= 20000 {
        return;
    }
    let vista: unsafe extern "C" fn(usize, usize, usize, u32, u32) -> i16 = core::mem::transmute(0x6e86d0usize);
    let sv = vista(cqa7_d(clube + 0xcf), p, clube, 1, (r / 4) as u32) as i32;
    let o = *saida as f64;
    let mut d1 = sv as f64 * cqa7_k(0x968f48) / o;
    let teto = *frac + cqa7_k(0x968e48);
    if !(d1 < teto) {
        d1 = teto;
    }
    match idade {
        0x18 => d1 = d1 * cqa7_k(0x968e40) + cqa7_k(0x968e80),
        0x19 => d1 = (d1 + cqa7_k(0x967890)) * cqa7_k(0x968968),
        0x1a => d1 = d1 * cqa7_k(0x968e80) + cqa7_k(0x968e40),
        _ => {}
    }
    if minimo as f64 * cqa7_k(0x9689b8) < o || alta {
        if !(d1 > cqa7_k(0x967890)) {
            return;
        }
        let a = cqa7_w(clube + 0x80);
        let ra = if a < 1000 { 1000 } else { a as i32 };
        let nr = cqa7_w(cqa7_d(this + 0xdc).wrapping_add(cqa7_d(clube).wrapping_mul(9)) + 5) as i32;
        let mut x = nr as f64 / ra as f64;
        if !(cqa7_k(0x967890) <= x) {
            x = cqa7_k(0x967890);
        } else if cqa7_k(0x968978) < x {
            x = cqa7_k(0x968978);
        }
        d1 = d1 * x;
    }
    if d1 > cqa7_k(0x967890) {
        crate::comparacao::ramo("cqa_7 jovem valorizado");
        *frac = d1 * *frac;
        let h = (d1 + cqa7_k(0x967890)) * cqa7_k(0x968968);
        *e = ftol(*e as f64 * h) as u32;
        *saida = ftol(o * h);
    }
}

/// { 0x840d80 } (thiscall, ret 0x18, shortlist_manager.cpp, 4,6 KB de x87; o mapa dá 1332 bytes):
/// `Shortlist_Manager::value_player_for_club(clube, pessoa, vaga, saida, mínimo, teto)`, das listas de observação e das
/// ofertas do computador (0x83f3a0, 0x842e10, 0x843800, 0x8c0810, 0x8c8090, 0x8c8860, 0x8e5d60). O observador é o
/// técnico do clube (+0xcf; sem a ficha +0x69, uma sorteada, 0x90cfc0 sobre [0xae2368], em [0xae23c0] + 0x44 × i). As
/// marcas da nota pelo tipo da vaga (+0x24 & 0x1e: 4 e 6 com 0x5882, ou 0x815882 para o próprio jogador do clube
/// humano, e o escopo da reputação 0x5408c0; 8 com 0x5982 e o escopo; senão 0x5902); a vaga da lista de empréstimos (bit
/// 0) muda o divisor do mínimo para 16 ou 6 (0x544590). Clube humano (0x600a50 com 1, 1): o técnico dele (0x53ca90) e,
/// com o jogador na lista do olheiro (0x7f7490), a nota (0x6e6340 com −1 e o substituto 0x600e90) vira max(nota +
/// 15000, nota × 1,5); senão a nota com o mínimo² × 0,000025 / divisor. Computador: com a marca 0x100 (ou 0x80) o melhor
/// observador entre o técnico e os 7 olheiros (+0x1b3) pelo conhecimento (0x544410 com 6 ou 5) menos 5 ou 10 pelo
/// escopo; a nota com o mínimo e esse observador, × 1,05 para o listado para transferência de outro clube (0x544520).
/// Nota negativa ou 10000 abaixo do mínimo, ou vaga da lista de empréstimos: o fim sem valor (`cqa7_sem_valor`). A névoa
/// (+0xb do registro de 79 bytes em [0xae2d0c], 4 bits) limita a nota a nota + 5000 por (1 + n²/100) × nota. Pela
/// posição (0x83bd00) o teto do preço e a fração; o valor base (alto com nota ≥ 35000, +0xb da vaga ≥ nota/8 e +1 ≥
/// 0x80); o ajuste pelo tipo da vaga (1, 2, 3: os jovens, `cqa7_jovem`; 3, 4, 5, 6: frações); a idade acima do divisor
/// (28, 32 sem posição, 19 na vaga 5) e o estrangeiro (0x53d980); o preço pela consistência (+0x2d da ficha +0x61) e o
/// teto pelo tipo da vaga, pela reputação, pela ficha (+0xd) e pela idade; acima de 1,5 milhão, a habilidade potencial
/// vista (0x6e86d0 com 2) abaixo de 145 puxa para 1,5 milhão; o limite da divisão do clube (+0x57, +0x69 < 16); a nação
/// do clube (+0x53) [0x9cf434] ou [0x9cf300]; com `teto`, no máximo +7 da vaga (pelo menos 1). Devolve o preço (_ftol).
unsafe extern "thiscall" fn valor_do_jogador_para_o_clube(this: usize, clube: usize, p: usize, vaga: usize, saida: *mut i32, minimo: i32, teto: u32) -> i32 {
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let escopo: unsafe extern "C" fn(usize, usize) -> u8 = core::mem::transmute(0x5408c0usize);
    let conhecer: unsafe extern "C" fn(usize, i8) -> i32 = core::mem::transmute(0x544410usize);
    let emprestimo: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x544590usize);
    let nota: unsafe extern "C" fn(usize, u32, u32, usize, i32, usize) -> i32 = core::mem::transmute(0x6e6340usize);
    let mut obs = cqa7_d(clube + 0xcf);
    let t7 = cqa7_d(vaga + 7) as i32;
    let teto_i = if t7 > 1 { t7 } else { 1 };
    let mut div: i8 = 4;
    if obs != 0 && cqa7_d(obs + 0x69) == 0 {
        let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
        let i = sorteio(cqa7_d(0xae2368) as i32);
        core::ptr::write_unaligned((obs + 0x69) as *mut usize, cqa7_d(0xae23c0).wrapping_add((i as usize).wrapping_mul(0x44)));
    }
    // ponytail: a original empurra a palavra da posição com a palavra alta de ecx (lixo); quem recebe lê só a palavra
    let pos = cqa7_w(vaga + 1) as u16 as u32;
    let tipo = cqa7_b(vaga + 0x24) & 0x1e;
    let flags: u32 = if tipo == 4 || tipo == 6 {
        let mut f = 0x5882;
        if cqa7_d(vaga + 0xf) == cqa7_d(p) && cqa7_d(p + 0x39) == clube && humano(clube, 1, 1, 0, 0) != 0 {
            f = 0x815882;
        }
        match escopo(p, cqa7_d(clube + 0xcf)) as i8 {
            0 => f | 0x40,
            1 => f | 0x50,
            2 => f | 0x10,
            3 => f | 0x30,
            4 => f | 0x20,
            _ => f,
        }
    } else if tipo == 8 {
        match escopo(p, cqa7_d(clube + 0xcf)) as i8 {
            0 => 0x59c2,
            1 => 0x59d2,
            2 => 0x5992,
            3 => 0x59b2,
            4 => 0x59a2,
            _ => 0x5982,
        }
    } else {
        0x5902
    };
    if cqa7_b(vaga + 0x24) & 1 != 0 {
        div = if emprestimo(p, 0) != 0 { 0x10 } else { 6 };
    }
    // o mínimo² × 0,000025 / divisor (_ftol)
    let corte = |m: i32, div: i8| {
        let q = m as f64;
        ftol(q * cqa7_k(0x96d240) * q / div as f64)
    };
    if humano(clube, 1, 1, 0, 0) != 0 {
        let tecnico: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53ca90usize);
        let na_lista: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7f7490usize);
        let substituto: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x600e90usize);
        let m = tecnico(clube);
        if na_lista(cqa7_d(0xb74448), p, m) != 0 {
            let s = substituto(clube);
            let r = nota(p, pos, flags, clube, -1, s);
            *saida = r;
            let a = r.wrapping_add(15000) as f64;
            let q = r as f64 * cqa7_k(0x968940);
            *saida = ftol(if a > q { a } else { q });
        } else {
            let s = substituto(clube);
            let v = corte(minimo, div);
            *saida = nota(p, pos, flags, clube, v, s);
        }
    } else {
        if flags & 0x100 != 0 || flags & 0x80 != 0 {
            let nivel: i8 = if flags & 0x100 != 0 { 6 } else { 5 };
            let mut melhor: i8 = 0;
            if flags & 0x100 != 0 {
                if obs != 0 && cqa7_d(obs + 0x69) != 0 {
                    melhor = conhecer(cqa7_d(obs + 0x69), 6) as i8;
                    match escopo(p, obs) as i8 {
                        3 => melhor = melhor.wrapping_sub(5),
                        4 => melhor = melhor.wrapping_sub(10),
                        _ => {}
                    }
                }
            } else if obs != 0 {
                // a marca 0x80: o técnico entra com +5 (−5 e +0 pelo escopo 4 e 3)
                melhor = (conhecer(cqa7_d(obs + 0x69), 5) as i8).wrapping_add(5);
                match escopo(p, obs) as i8 {
                    3 => melhor = melhor.wrapping_sub(5),
                    4 => melhor = melhor.wrapping_sub(10),
                    _ => {}
                }
            }
            for k in 0..7 {
                let s = cqa7_d(clube + 0x1b3 + 4 * k);
                if s == 0 || cqa7_d(s + 0x69) == 0 {
                    continue;
                }
                let mut v = conhecer(cqa7_d(s + 0x69), nivel) as i8;
                match escopo(p, s) as i8 {
                    3 => {
                        if nivel == 6 && cqa7_w(clube + 0x80) < 0xdac {
                            continue;
                        }
                        v = v.wrapping_sub(5);
                    }
                    4 => {
                        if nivel == 6 && cqa7_w(clube + 0x80) < 0xdac {
                            continue;
                        }
                        v = v.wrapping_sub(10);
                    }
                    _ => {}
                }
                if v > melhor {
                    melhor = v;
                    obs = s;
                }
            }
        }
        let v = if minimo > -1 { corte(minimo, div) } else { -1 };
        *saida = nota(p, pos, flags, clube, v, obs);
        let transferencia: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x544520usize);
        if clube != cqa7_d(p + 0x39) && transferencia(p, 0) != 0 {
            crate::comparacao::ramo("cqa_7 listado para transferencia");
            *saida = ftol(*saida as f64 * cqa7_k(0x9689b8)); // cqa7: o listado de outro clube × 1,05
        }
    }
    let r = *saida;
    if r < 0 || r < minimo.wrapping_sub(10000) || cqa7_b(vaga + 0x24) & 1 != 0 {
        return cqa7_sem_valor(p, vaga, saida);
    }
    let nevoa = cqa7_b(cqa7_d(0xae2d0c).wrapping_add(cqa7_d(p).wrapping_mul(0x4f)) + 0xb) & 0xf;
    if nevoa != 0 {
        let a = r.wrapping_add(5000) as f64;
        let w = (pow_do_jogo(nevoa as f64, cqa7_k(0x968950)) * cqa7_k(0x967898) + cqa7_k(0x967890)) * r as f64;
        *saida = ftol(if a < w { a } else { w });
    }
    div = 0x1c;
    let posicao: unsafe extern "thiscall" fn(usize, u32) -> u8 = core::mem::transmute(0x83bd00usize);
    let c = posicao(this, pos) as i8 as i32;
    let (teto_pos, mut frac) = match c {
        1..=4 | 17 => (8000000.0, 0.6),
        5..=7 | 19 => (9000000.0, 0.65),
        8..=10 | 18 => (10000000.0, 0.75),
        11..=13 | 21 => (11000000.0, 0.8),
        14 | 20 => (13000000.0, 1.0),
        _ => {
            div = 0x20;
            (6000000.0, 0.575)
        }
    };
    let r = *saida;
    let bv = cqa7_d(vaga + 0xb) as i32;
    let alta = r >= 35000 && bv >= r / 8 && cqa7_w(vaga + 1) >= 0x80;
    let mut e = if alta {
        let v = r as f64;
        ftol(((v * cqa7_k(0x96d238) - cqa7_k(0x967890)) * v + bv as f64 * cqa7_k(0x9689e0)) * cqa7_k(0x96f918)) as u32
    } else {
        crate::comparacao::ramo("cqa_7 valor base");
        ftol((bv as f64 * cqa7_k(0x9689e0) + r as f64 * cqa7_k(0x969040)) * cqa7_k(0x96f918)) as u32 // cqa7: o valor base
    };
    if e < 1 {
        e = 1;
    }
    let idade = cqa7_b(p + 0x18) as i8;
    match (cqa7_b(vaga + 0x24) >> 1) & 0xf {
        1 | 2 => cqa7_jovem(this, clube, p, saida, minimo, alta, idade, r, &mut frac, &mut e),
        3 => {
            let k = if cqa7_w(vaga + 1) < 0x80 { cqa7_k(0x96a670) } else { cqa7_k(0x96d718) };
            e = ftol(e as f64 * k) as u32;
            cqa7_jovem(this, clube, p, saida, minimo, alta, idade, r, &mut frac, &mut e);
        }
        4 => e = ftol(e as f64 * cqa7_k(0x968968)) as u32,
        5 => {
            e = ftol(e as f64 * cqa7_k(0x968a80)) as u32;
            div = div.wrapping_sub(9);
        }
        6 => e = ftol(e as f64 * cqa7_k(0x968918)) as u32,
        _ => {}
    }
    let tipo = cqa7_b(vaga + 0x24) & 0x1e;
    if idade > div {
        let d = idade as i32 - div as i32;
        let dd = d as f64;
        let ficha = cqa7_d(p + 0x61);
        let f = cqa7_k(0x967890) / (dd * cqa7_k(0x968e80) + cqa7_k(0x9689b0)); // cqa7: a idade acima do divisor
        let ts = cqa7_w(ficha + 0xd);
        let g = if ts > 0x1770 { ts as i32 as f64 * cqa7_k(0x96a680) - cqa7_k(0x968940) } else { 0.0 };
        let estrangeira: unsafe extern "C" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x53d980usize);
        let y = if estrangeira(p, cqa7_d(clube + 0x53), 1) != 0 {
            f / (dd * cqa7_k(0x968f20) + cqa7_k(0x9689d8))
        } else if idade as i32 > div as i32 + 2 && (tipo == 8 || tipo == 6) && cqa7_w(ficha + 0xb) < cqa7_w(clube + 0x80) {
            (cqa7_k(0x967890) - (d - 2) as f64 * cqa7_k(0x96d438)) * f
        } else {
            f
        };
        crate::comparacao::ramo("cqa_7 idade");
        let um = cqa7_k(0x967890);
        if tipo == 0xa {
            *saida = ftol(((um - g) * y + g) * *saida as f64);
        }
        e = ftol(((um - g) * y + g) * e as f64) as u32;
    }
    let ficha = cqa7_d(p + 0x61);
    let cons = cqa7_b(ficha + 0x2d) as i8 as f64;
    let c = ftol((cqa7_k(0x967890) - pow_do_jogo(cons, cqa7_k(0x968950)) * cqa7_k(0x968f40)) * e as f64);
    let mut kk = (c.wrapping_mul(c) as u32) as f64 * teto_pos * cqa7_k(0x968eb8);
    let ra = cqa7_w(clube + 0x80);
    if tipo == 0xc {
        if ra > 5000 {
            let lim = (ra as i32 - 5000) as f64 * cqa7_k(0x968ea8) + cqa7_k(0x968880);
            if !(kk <= lim) {
                kk = (ra as i32 - 100000) as f64 + cqa7_k(0x968880);
            }
        } else {
            let lim = (ra as i32 * 10) as f64;
            if !(kk <= lim) {
                kk = ra as f64 * cqa7_k(0x9688b0);
            }
        }
    } else if tipo == 0xa {
        let lim = if ra > 5000 {
            let z = (ra as i32 - 5000) as f64 * cqa7_k(0x968918) + cqa7_k(0x968878);
            z * z
        } else {
            let z = ra as f64;
            z * z * cqa7_k(0x96f910)
        };
        if !(kk <= lim) {
            kk = lim;
        }
    }
    let ts = cqa7_w(ficha + 0xd);
    if ts <= 5000 {
        let lim = frac * cqa7_k(0x96f908);
        if lim < kk {
            kk = lim;
        }
    } else if ts <= 8000 {
        let lim = ts as f64 * frac * cqa7_k(0x96f900) + cqa7_k(0x96f8f8);
        if lim < kk {
            kk = lim;
        }
    } else if ts < 9000 {
        let lim = (ts as f64 * cqa7_k(0x9690b8) - cqa7_k(0x96f8f0)) * frac;
        if lim < kk {
            kk = lim;
        }
        frac = frac * cqa7_k(0x968940);
    } else {
        kk = cqa7_k(0x96d568);
        frac = frac * cqa7_k(0x968e18);
    }
    let pela_idade = match idade {
        0x10 => Some(0x96f8d8),
        0x11 => Some(0x968e90),
        0x12 | 0x1f => Some(0x96d850),
        0x13 | 0x1d => Some(0x96f8e0),
        0x1c => Some(0x96a630),
        0x1e => Some(0x96f908),
        0x20 => Some(0x9688c8),
        0x21 => Some(0x96a650),
        0x22 => Some(0x9688d8),
        0x23 => Some(0x96f8e8),
        0x24..=0x28 => Some(0x968870),
        _ => None,
    };
    if let Some(a) = pela_idade {
        let t = frac * cqa7_k(a);
        if t < kk {
            kk = t;
        }
    }
    let base = cqa7_k(0x96f8d0);
    let mut x = kk;
    if kk > base {
        let vista: unsafe extern "C" fn(usize, usize, usize, u32, u32) -> i16 = core::mem::transmute(0x6e86d0usize);
        let n2 = vista(cqa7_d(clube + 0xcf), p, clube, 2, (*saida / 4) as u32) as i32 as f64;
        if n2 < cqa7_k(0x96f8c8) {
            let mut a = n2 * cqa7_k(0x967878) - cqa7_k(0x96f8c0);
            if !(cqa7_k(0x9689a0) <= a) {
                a = cqa7_k(0x9689a0);
            }
            x = (kk - base) * a + base;
        }
    }
    let bc = cqa7_d(clube + 0x57);
    if bc != 0 && cqa7_w(bc + 0x69) < 0x10 {
        let b69 = cqa7_w(bc + 0x69) as i32;
        let ri = cqa7_w(clube + 0x80) as i32 / 500;
        let m = if ri > b69 { ri } else { b69 };
        let mut lim = m.wrapping_mul(670000);
        if lim <= 1000000 {
            lim = 1000000;
        }
        let ts = cqa7_w(ficha + 0xd) as i32;
        if ts > 8000 {
            lim = lim.wrapping_add((ts - 8000) * 10000);
        }
        let l = lim as f64;
        if !(x <= l) {
            let m2 = if cqa7_k(0x967890) < frac { cqa7_k(0x967890) } else { frac };
            x = ftol(l * m2) as f64;
        }
    }
    // a nação do clube (+0x53 do clube: a original usa edi, que aqui já é o clube)
    let nacao = cqa7_d(cqa7_d(clube + 0x53));
    if nacao == cqa7_d(0x9cf434) {
        if x < cqa7_k(0x968e90) {
            x = x * (x * cqa7_k(0x968e88) * cqa7_k(0x968e48) + cqa7_k(0x968e80));
        } else {
            x = x * cqa7_k(0x9689d8);
        }
    } else if nacao == cqa7_d(0x9cf300) {
        x = ftol(x * cqa7_k(0x9689b0)) as f64;
    }
    if teto != 0 {
        let c = teto_i as f64;
        if c < x {
            x = c;
        }
        x = ftol(x) as f64;
    }
    ftol(x)
}

/// Comparador da 0x840d80: lado a lado na nota de saída e na devolução (o resto ela só lê). O técnico do clube sem a
/// ficha +0x69 (a original sorteia uma): só a original.
unsafe extern "thiscall" fn comparar_valor_do_jogador_para_o_clube(this: usize, clube: usize, p: usize, vaga: usize, saida: *mut i32, minimo: i32, teto: u32) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut i32, i32, u32) -> i32>(1437);
    let obs = cqa7_d(clube + 0xcf);
    if obs != 0 && cqa7_d(obs + 0x69) == 0 {
        return o(this, clube, p, vaga, saida, minimo, teto);
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(saida as usize, 4)], || o(this, clube, p, vaga, saida, minimo, teto), || valor_do_jogador_para_o_clube(this, clube, p, vaga, saida, minimo, teto));
    PLACAR.registrar(igual && r == n, || {
        format!("valor_do_jogador_para_o_clube(clube {:x}, pessoa {:x}, vaga {:x}, mínimo {}, teto {}) original={} rust={} {:x?} x {:x?}", clube, p, vaga, minimo, teto, r, n, mo, mr)
    });
    r
}

/// { 0x7f8610 } (thiscall, ret 4, scout_manager.cpp): `SCOUT_MANAGER::find_club_search_entry(pessoa)`, das telas da
/// pessoa e da busca (0x808540, 0x813210, 0x870650, 0x87a510): sem o humano da vez ou sem o clube da pessoa (+0x39), 0.
/// A vaga do humano (byte +0 − byte de [0xae2364] + 16, 0..15; fora, a caixa da linha 0x4b5 e 0). A primeira das 10
/// buscas (0x7f6cd0) com o clube observado (+0x190) igual ao da pessoa e o registro dele (+0x170) dá a vaga da pessoa
/// nele (0x7bd700 sem reservar); nenhuma, 0.
unsafe extern "thiscall" fn entrada_do_jogador_na_busca_do_clube(this: usize, p: usize) -> usize {
    let m = cps8_gerente_da_vez();
    if m == 0 || cqa7_d(p + 0x39) == 0 {
        return 0;
    }
    let vaga = cqa7_b(m).wrapping_sub(cqa7_b(0xae2364)).wrapping_add(0x10) as i8;
    if !(0..=15).contains(&vaga) {
        aviso_em(0xa7b094, 0x4b5);
        return 0;
    }
    let busca: unsafe extern "thiscall" fn(usize, u32, usize) -> usize = core::mem::transmute(0x7f6cd0usize);
    for i in 0..10u32 {
        let l = busca(this, i, m);
        // ponytail: a original lê +0x190 sem conferir a busca (cairia com 0); aqui a busca nula não conta
        if l == 0 || cqa7_d(l + 0x190) != cqa7_d(cqa7_d(p + 0x39)) || cqa7_d(busca(this, i, m) + 0x170) == 0 {
            continue;
        }
        let vaga_do_jogador: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x7bd700usize);
        return vaga_do_jogador(cqa7_d(busca(this, i, m) + 0x170), p, 0);
    }
    0
}

/// Comparador da 0x7f8610: pela devolução (só lê).
unsafe extern "thiscall" fn comparar_entrada_do_jogador_na_busca_do_clube(this: usize, p: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(1438);
    let r = o(this, p);
    let n = entrada_do_jogador_na_busca_do_clube(this, p);
    PLACAR.registrar(r == n, || format!("entrada_do_jogador_na_busca_do_clube(pessoa {:x}) original={:x} rust={:x}", p, r, n));
    r
}

/// { 0x8d0f00 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::find_international_appearance_money(pessoa)`,
/// das telas da pessoa (0x884ea2): sem a marca 4 da pessoa ([this+0x86e] + id × 8), 0. A busca binária (0x946464:
/// cdecl(chave, vetor [this+0x898], conta [this+0x8a0], 0xd, 0x8b4e00)) pela chave com o id da pessoa em +0, o único
/// campo que a comparação lê; achada, o registro; senão a marca 4 sai e 0.
unsafe extern "thiscall" fn premio_de_jogos_na_selecao_achado(this: usize, p: usize) -> usize {
    let m = (cqa7_d(this + 0x86e).wrapping_add(cqa7_d(p).wrapping_mul(8))) as *mut u8;
    if *m & 4 == 0 {
        return 0;
    }
    let chave = [cqa7_d(p) as u32, 0, 0, 0];
    let bsearch: unsafe extern "C" fn(*const u32, usize, usize, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    let r = bsearch(chave.as_ptr(), cqa7_d(this + 0x898), cqa7_d(this + 0x8a0), 0xd, 0x8b4e00);
    if r != 0 {
        return r;
    }
    *m = (*m).wrapping_sub(4);
    0
}

/// Comparador da 0x8d0f00: lado a lado na marca da pessoa e na devolução.
unsafe extern "thiscall" fn comparar_premio_de_jogos_na_selecao_achado(this: usize, p: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(1439);
    let m = cqa7_d(this + 0x86e).wrapping_add(cqa7_d(p).wrapping_mul(8));
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(m, 1)], || o(this, p), || premio_de_jogos_na_selecao_achado(this, p));
    PLACAR.registrar(igual && r == n, || format!("premio_de_jogos_na_selecao_achado(pessoa {:x}) original={:x} rust={:x} {:x?} x {:x?}", p, r, n, mo, mr));
    r
}

// ---------- 261a leva: as vagas da lista de observação com lance, e as marcas de lista de transferências e de empréstimos do contrato (shortlist_manager.cpp) ----------

unsafe fn cpy1_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpy1_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpy1_w(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

/// O contrato principal da pessoa (CONTRACT_MANAGER::get_contract 0x4cf5a0: thiscall([0xae1050], pessoa), ret 4).
/// As três marcas não guardam o contrato em [0xae1080].
unsafe fn cpy1_contrato(pessoa: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    f(cpy1_d(0xae1050), pessoa)
}

/// As datas estáticas da 0x54c5b0, da 0x54c8e0 e da 0x54cdb0 (valor e guarda).
const CPY1_ESTATICAS: [(usize, usize); 6] = [(0xae2420, 8), (0xae2469, 1), (0xae24c0, 8), (0xae249e, 1), (0xae2470, 8), (0xae2418, 1)];

/// A entrada do clube no Shortlist_Manager (+0xdc, 9 bytes por clube: +0 as 40 vagas de 0x25 bytes, +4 o estado).
unsafe fn cpy1_entrada(this: usize, clube: usize) -> usize {
    cpy1_d(this + 0xdc).wrapping_add(cpy1_d(clube).wrapping_mul(9))
}

/// { 0x844540 } (thiscall, ret 8, shortlist_manager.cpp): `Shortlist_Manager::count_slots_with_bids(clube, posicao)`,
/// pela avaliação das listas (0x84a020, com 0xb), pela avaliação do elenco (0x83cb40, com −1) e pelo laço do dia
/// (0x5cccd0, com −1). Das 40 vagas da lista do clube, conta as que têm alvo (+0xf) e lance (+0x17), as duas acima de
/// −1, e posição (+0) igual a `posicao` (0xff: qualquer). Com a lista no estado 2 (+4 da entrada) e o clube do humano
/// (0x600a50 com 1, 1): se hoje − 20 dias (0x54c8e0) passa da data de chegada do técnico (+0x3e da ficha em +0xcf do
/// clube) e o alvo não tem ofertas (0x8c5cf0), a vaga perde o alvo e o lance (−1) e não conta. A conta é um byte.
unsafe extern "thiscall" fn contar_vagas_com_lance(this: usize, clube: usize, posicao: u32) -> u8 {
    // 0x600a50: cdecl(clube, 1, 1, 0, 0) (add esp, 0x14 depois); 0x54c8e0: thiscall(0xae2c90, saída, 0x14); 0x8c5cf0:
    // thiscall([0xde1f54], pessoa, 0, 0)
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let menos_dias: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(0x54c8e0usize);
    let ofertas: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x8c5cf0usize);
    let entrada = cpy1_entrada(this, clube);
    let mut n: u8 = 0;
    for i in 0..40usize {
        let v = cpy1_d(entrada).wrapping_add(i * 0x25);
        if (cpy1_d(v + 0xf) as i32) <= -1 {
            continue;
        }
        if (cpy1_d(v + 0x17) as i32) <= -1 {
            crate::comparacao::ramo("cpy_1 alvo sem lance");
            continue; // cpy1: alvo sem lance não conta
        }
        if posicao as u8 != 0xff && cpy1_b(v) != posicao as u8 {
            crate::comparacao::ramo("cpy_1 outra posicao");
            continue; // cpy1: lance de outra posição não conta
        }
        if cpy1_b(entrada + 4) == 2 && humano(clube, 1, 1, 0, 0) != 0 {
            let mut buf = [0u8; 8];
            let r = menos_dias(0xae2c90, &mut buf, 0x14);
            let dia = core::ptr::read_unaligned(r as *const i16);
            let ano = cpy1_w(r + 2);
            let chegada = cpy1_d(clube + 0xcf).wrapping_add(0x3e);
            let ano_c = cpy1_w(chegada + 2);
            let velho = ano > ano_c || (ano == ano_c && dia > core::ptr::read_unaligned(chegada as *const i16));
            if velho {
                let alvo = cpy1_d(0xae23bc).wrapping_add(cpy1_d(v + 0xf).wrapping_mul(0x6e));
                if ofertas(cpy1_d(0xde1f54), alvo, 0, 0) == 0 {
                    crate::comparacao::ramo("cpy_1 lance vencido");
                    *((v + 0xf) as *mut i32) = -1; // cpy1: a vaga perde o alvo
                    *((v + 0x17) as *mut i32) = -1;
                    continue;
                }
            }
        }
        crate::comparacao::ramo("cpy_1 vaga com lance");
        n = n.wrapping_add(1); // cpy1: conta a vaga com lance
    }
    n
}

/// Comparador: lado a lado pelas 40 vagas do clube e pelas datas estáticas, e a devolução.
unsafe extern "thiscall" fn comparar_contar_vagas_com_lance(this: usize, clube: usize, posicao: u32) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u8>(1344);
    let mut reg = CPY1_ESTATICAS.to_vec();
    reg.push((cpy1_d(cpy1_entrada(this, clube)), 40 * 0x25));
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube, posicao), || contar_vagas_com_lance(this, clube, posicao));
    PLACAR.registrar(igual && r == n, || format!("contar_vagas_com_lance(clube {:x}, posicao {}) original={} rust={} {:x?} x {:x?}", clube, posicao as i8, r, n, mo, mr));
    r
}

/// { 0x844280 } (stdcall, ret 4, shortlist_manager.cpp): `shortlist_manager_remove_from_transfer_list(pessoa)`, pelas
/// telas de transferência (0x8f6770, 0x8f8120). Sem contrato (0x4cf5a0): 0. Com ele, o bit 0 de +0x4e (na lista de
/// transferências) sai e devolve 1 no byte baixo (os de cima são os do ponteiro do contrato, como a original).
unsafe extern "stdcall" fn tirar_da_lista_de_transferencias(pessoa: usize) -> u32 {
    let c = cpy1_contrato(pessoa);
    if c == 0 {
        return 0;
    }
    *((c + 0x4e) as *mut u8) &= 0xfe;
    (c as u32 & !0xff) | 1
}

/// Comparador: lado a lado pelo byte +0x4e do contrato, e a devolução.
unsafe extern "stdcall" fn comparar_tirar_da_lista_de_transferencias(pessoa: usize) -> u32 {
    let o = orig::<unsafe extern "stdcall" fn(usize) -> u32>(1345);
    let c = cpy1_contrato(pessoa);
    let reg = if c != 0 { vec![(c + 0x4e, 1)] } else { Vec::new() };
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(pessoa), || tirar_da_lista_de_transferencias(pessoa));
    PLACAR.registrar(igual && r == n, || format!("tirar_da_lista_de_transferencias(pessoa {:x}) original={:#x} rust={:#x} {:x?} x {:x?}", pessoa, r, n, mo, mr));
    r
}

/// { 0x8442d0 } (stdcall, ret 4, shortlist_manager.cpp): `shortlist_manager_remove_from_loan_list(pessoa)`, pela
/// renovação do contrato (0x4dc3a0) e pelas telas de transferência. Sem contrato: 0; com ele, o bit 1 de +0x4e (na
/// lista de empréstimos) sai e devolve 1 no byte baixo (os de cima, os do ponteiro do contrato).
unsafe extern "stdcall" fn tirar_da_lista_de_emprestimos(pessoa: usize) -> u32 {
    let c = cpy1_contrato(pessoa);
    if c == 0 {
        return 0;
    }
    *((c + 0x4e) as *mut u8) &= 0xfd;
    (c as u32 & !0xff) | 1
}

/// Comparador: lado a lado pelo byte +0x4e do contrato, e a devolução.
unsafe extern "stdcall" fn comparar_tirar_da_lista_de_emprestimos(pessoa: usize) -> u32 {
    let o = orig::<unsafe extern "stdcall" fn(usize) -> u32>(1346);
    let c = cpy1_contrato(pessoa);
    let reg = if c != 0 { vec![(c + 0x4e, 1)] } else { Vec::new() };
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(pessoa), || tirar_da_lista_de_emprestimos(pessoa));
    PLACAR.registrar(igual && r == n, || format!("tirar_da_lista_de_emprestimos(pessoa {:x}) original={:#x} rust={:#x} {:x?} x {:x?}", pessoa, r, n, mo, mr));
    r
}

// ---------- 261a leva: a situação da pessoa nas listas de observação dos clubes, a marca de lista de empréstimos e o teto de preço pelo estatuto (shortlist_manager.cpp) ----------

unsafe fn cpy2_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpy2_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpy2_pd(a: usize, v: u32) {
    core::ptr::write_unaligned(a as *mut u32, v);
}

/// O registro de clube de id `id` ([0xae23b4], 0x245 bytes).
unsafe fn cpy2_clube(id: usize) -> usize {
    cpy2_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// A entrada do clube no Shortlist_Manager (+0xdc, 9 bytes por clube: +0 as 40 vagas de 0x25 bytes, +4 o estado, +8).
unsafe fn cpy2_entrada(this: usize, clube: usize) -> usize {
    cpy2_d(this + 0xdc).wrapping_add(cpy2_d(clube).wrapping_mul(9))
}

/// As datas estáticas da 0x54c5b0, da 0x54c8e0 e da 0x54cdb0 (valor e guarda).
const CPY2_ESTATICAS: [(usize, usize); 6] = [(0xae2420, 8), (0xae2469, 1), (0xae24c0, 8), (0xae249e, 1), (0xae2470, 8), (0xae2418, 1)];

/// Tira o par (pessoa, clube) de um dos vetores ordenados de pares (8 bytes) do Shortlist_Manager (`base` = +0 ou +4,
/// conta em `base` + 0x10) pela busca binária (0x946464: cdecl(chave, vetor, conta, 8, comparação), add esp, 0x14):
/// os de depois descem uma casa, o último fica zerado e a conta cai. Devolve se achou.
unsafe fn cpy2_tirar_par(base: usize, chave: &[u32; 2], comparar: usize) -> bool {
    let bsearch: unsafe extern "C" fn(*const u32, usize, u32, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    let p = bsearch(chave.as_ptr(), cpy2_d(base), cpy2_d(base + 0x10) as u32, 8, comparar);
    if p == 0 {
        return false;
    }
    let mut i = (p.wrapping_sub(cpy2_d(base)) >> 3) as i32;
    while i < cpy2_d(base + 0x10) as i32 - 1 {
        let v = cpy2_d(base);
        i += 1;
        let k = v.wrapping_add(i as usize * 8);
        cpy2_pd(k - 8, cpy2_d(k) as u32);
        cpy2_pd(k - 4, cpy2_d(k + 4) as u32);
    }
    let fim = cpy2_d(base).wrapping_add(cpy2_d(base + 0x10).wrapping_mul(8));
    cpy2_pd(fim - 4, 0);
    cpy2_pd(fim - 8, 0);
    cpy2_pd(base + 0x10, (cpy2_d(base + 0x10) as u32).wrapping_sub(1));
    true
}

/// A data de hoje mais (0x54c5b0) ou menos (0x54c8e0) `dias` (thiscall(0xae2c90, saída, dias), ret 8) copiada para a
/// data da vaga em +0x1b (0x417c10: thiscall(destino, saída, origem), ret 8).
unsafe fn cpy2_data_da_vaga(v: usize, rotina: usize, dias: u32) {
    let deslocar: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(rotina);
    let copiar: unsafe extern "thiscall" fn(usize, *mut [u8; 8], usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut a = [0u8; 8];
    let mut b = [0u8; 8];
    let r = deslocar(0xae2c90, &mut a, dias);
    copiar(v + 0x1b, &mut b, r);
}

/// O par (pessoa, clube) do evento 2 ou 5: a vaga guarda a pessoa em +0x13 (bloqueada até a data de +0x1b), sem lance
/// (+0x17 = −1) e sem valores (+0xb, +7); o estatuto da vaga (bits 1..4 de +0x24) vai ao nibble alto de +0x4f do
/// contrato da pessoa no clube; a entrada fica com 100 em +8.
unsafe fn cpy2_bloquear(v: usize, entrada: usize, pessoa: usize, contrato: usize, rotina: usize, dias: u32) {
    cpy2_data_da_vaga(v, rotina, dias);
    cpy2_pd(v + 0x13, cpy2_d(pessoa) as u32);
    cpy2_pd(v + 0x17, 0xffff_ffff);
    cpy2_pd(v + 0xb, 0);
    cpy2_pd(v + 7, 0);
    if contrato != 0 {
        let c = (contrato + 0x4f) as *mut u8;
        *c = (*c & 0xf) | (((cpy2_b(v + 0x24) >> 1) & 0xf) << 4);
    }
    *((entrada + 8) as *mut u8) = 0x64;
}

/// { 0x843e10 } (thiscall, ret 0x10, shortlist_manager.cpp, quadro SEH da LLIST local): `Shortlist_Manager::
/// update_staff_status(pessoa, clube, evento, tipo)`, pelas ofertas, contratos, técnicos e regens (0x4d07e0, 0x690730,
/// 0x7a59e0, 0x8bf680, 0x8c44e0...) e por ela mesma. Com o evento 4 e o clube nulo ou o da pessoa (+0x39): cada clube
/// interessado (0x842180 com 0, 3, 0) recebe o evento 1 com o tipo 2 (recursão). Clube nulo: 1. Com o evento 5, ou 1..3
/// de outro clube: o par (pessoa, clube) sai dos dois vetores ordenados (+0/+0x10 pela 0x757ea0, +4/+0x14 pela
/// 0x844740; vetor nulo: 0); achado nos dois e o clube não humano (0x600a50 com 0, 1), o nibble baixo de +0xb do
/// registro de 79 bytes da pessoa ([0xae2d0c]) cai um. Depois, nas 40 vagas do clube (tipo 1: fora as de jogador 0xb;
/// tipo 0: só elas) com a pessoa no alvo (+0xf; no evento 4, também em +0x13): 1 = a vaga se esvazia e é preenchida de
/// novo (0x83f3a0); 2 e 5 = bloqueio de 120 e 14 dias (0x54c5b0); 3 = esvazia, data de hoje − 120 (0x54c8e0); 4 =
/// solta +0x13 e o alvo, data de hoje − 120. Devolve 1.
unsafe extern "thiscall" fn situacao_na_lista_de_observacao(this: usize, pessoa: usize, clube: usize, evento: u32, tipo: u32) -> u8 {
    // 0x68d020/0x68d040/0x68d1b0: thiscall(lista); 0x842180: thiscall(this, pessoa, lista, 0, 3, 0) (5 push); a
    // recursão: thiscall(this, pessoa, clube, 1, 2) (4 push); 0x600a50: cdecl(clube, 0, 1, 0, 0) (add esp, 0x14);
    // 0x4cf5f0: thiscall([0xae1050], pessoa, clube) (2 push); 0x83f3a0: thiscall(this, vaga, clube, pessoa) (3 push)
    let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let interessados: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) -> u32 = core::mem::transmute(0x842180usize);
    let situacao: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u8 = core::mem::transmute(0x843e10usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let contrato_de: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let preencher: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(0x83f3a0usize);
    let mut lista = [0u8; 0x10]; // a LLIST local (0xe bytes)
    let l = lista.as_mut_ptr() as usize;
    lista_nova(l);
    let ev = evento as u8;
    let id = cpy2_d(pessoa);
    if (clube == 0 || clube == cpy2_d(pessoa + 0x39)) && ev == 4 {
        interessados(this, pessoa, l, 0, 3, 0);
        core::ptr::write_unaligned((l + 0xa) as *mut usize, cpy2_d(l + 2));
        let mut e = proximo(l);
        while e != 0 {
            situacao(this, pessoa, cpy2_clube(cpy2_d(e + 4)), 1, 2);
            e = proximo(l);
        }
    }
    if clube == 0 {
        destruir(l);
        return 1;
    }
    if ev == 5 || (ev != 4 && clube != cpy2_d(pessoa + 0x39)) {
        if cpy2_d(this) == 0 || cpy2_d(this + 4) == 0 {
            destruir(l);
            return 0;
        }
        let chave = [id as u32, cpy2_d(clube) as u32];
        let mut faltou = !cpy2_tirar_par(this, &chave, 0x757ea0);
        faltou |= !cpy2_tirar_par(this + 4, &chave, 0x844740);
        if humano(clube, 0, 1, 0, 0) == 0 && !faltou {
            crate::comparacao::ramo("cpy_2 contagem da lista");
            let r = (cpy2_d(0xae2d0c).wrapping_add(id.wrapping_mul(0x4f)) + 0xb) as *mut u8;
            *r = (*r & 0xf0) | ((*r).wrapping_sub(1) & 0xf); // cpy2: uma lista a menos na conta da pessoa
        }
    }
    let entrada = cpy2_entrada(this, clube);
    let contrato = contrato_de(cpy2_d(0xae1050), pessoa, clube);
    for i in 0..40usize {
        let v = cpy2_d(entrada).wrapping_add(i * 0x25);
        let t = tipo as u8;
        if t == 1 {
            if cpy2_b(v) == 0xb {
                continue;
            }
        } else if t == 0 && cpy2_b(v) != 0xb {
            continue;
        }
        if cpy2_d(v + 0xf) != id && (ev != 4 || cpy2_d(v + 0x13) != id) {
            continue;
        }
        match ev {
            1 => {
                cpy2_pd(v + 0xf, 0xffff_ffff);
                cpy2_pd(v + 0x17, 0xffff_ffff);
                if cpy2_d(v + 0x13) == id {
                    cpy2_pd(v + 0x13, 0xffff_ffff);
                }
                *((v + 0x24) as *mut u8) &= 0xfe;
                crate::comparacao::ramo("cpy_2 vaga refeita");
                preencher(this, v, clube, pessoa); // cpy2: a vaga é preenchida de novo
            }
            2 => cpy2_bloquear(v, entrada, pessoa, contrato, 0x54c5b0, 0x78),
            5 => {
                crate::comparacao::ramo("cpy_2 bloqueio de 14 dias");
                cpy2_bloquear(v, entrada, pessoa, contrato, 0x54c5b0, 0xe); // cpy2: o bloqueio de 14 dias
            }
            3 => {
                cpy2_pd(v + 0xf, 0xffff_ffff);
                cpy2_pd(v + 0x17, 0xffff_ffff);
                cpy2_data_da_vaga(v, 0x54c8e0, 0x78);
                if cpy2_d(v + 0x13) == id {
                    cpy2_pd(v + 0x13, 0xffff_ffff);
                }
            }
            4 => {
                if cpy2_d(v + 0x13) == id {
                    cpy2_pd(v + 0x13, 0xffff_ffff);
                }
                if cpy2_d(v + 0xf) == id {
                    cpy2_pd(v + 0xf, 0xffff_ffff);
                    cpy2_pd(v + 0x17, 0xffff_ffff);
                    cpy2_data_da_vaga(v, 0x54c8e0, 0x78);
                }
            }
            _ => {}
        }
    }
    destruir(l);
    1
}

/// O caso que não pode rodar duas vezes: a recursão do evento 4 pelos clubes interessados, ou o evento 1 com a pessoa no
/// alvo de alguma vaga do clube (a 0x83f3a0 preenche a vaga: sorteia, acrescenta pares com realocação e dá notícias).
unsafe fn cpy2_so_a_original(this: usize, pessoa: usize, clube: usize, evento: u32, tipo: u32) -> bool {
    let ev = evento as u8;
    if ev == 4 && (clube == 0 || clube == cpy2_d(pessoa + 0x39)) {
        return true;
    }
    if ev != 1 || clube == 0 {
        return false;
    }
    // de outro clube, o evento 1 passa pelos vetores; um deles nulo devolve 0 antes das vagas
    if clube != cpy2_d(pessoa + 0x39) && (cpy2_d(this) == 0 || cpy2_d(this + 4) == 0) {
        return false;
    }
    let entrada = cpy2_entrada(this, clube);
    (0..40usize).any(|i| {
        let v = cpy2_d(entrada).wrapping_add(i * 0x25);
        let ok = match tipo as u8 {
            1 => cpy2_b(v) != 0xb,
            0 => cpy2_b(v) == 0xb,
            _ => true,
        };
        ok && cpy2_d(v + 0xf) == cpy2_d(pessoa)
    })
}

/// Comparador: lado a lado pelos dois vetores de pares e suas contas, pelo byte da conta de listas da pessoa, pelas 40
/// vagas e o +8 da entrada do clube, pelo byte +0x4f do contrato e pelas datas estáticas, e a devolução. Na recursão do
/// evento 4 e no evento 1 que preenche a vaga, só a original (a cópia leva os ramos).
unsafe extern "thiscall" fn comparar_situacao_na_lista_de_observacao(this: usize, pessoa: usize, clube: usize, evento: u32, tipo: u32) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u8>(1347);
    if cpy2_so_a_original(this, pessoa, clube, evento, tipo) {
        return o(this, pessoa, clube, evento, tipo);
    }
    let mut reg = CPY2_ESTATICAS.to_vec();
    for base in [this, this + 4] {
        reg.push((base + 0x10, 4));
        if cpy2_d(base) != 0 {
            reg.push((cpy2_d(base), cpy2_d(base + 0x10).wrapping_mul(8)));
        }
    }
    reg.push((cpy2_d(0xae2d0c).wrapping_add(cpy2_d(pessoa).wrapping_mul(0x4f)) + 0xb, 1));
    if clube != 0 {
        let entrada = cpy2_entrada(this, clube);
        reg.push((cpy2_d(entrada), 40 * 0x25));
        reg.push((entrada + 8, 1));
        let contrato_de: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
        let c = contrato_de(cpy2_d(0xae1050), pessoa, clube);
        if c != 0 {
            reg.push((c + 0x4f, 1));
        }
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, pessoa, clube, evento, tipo),
        || situacao_na_lista_de_observacao(this, pessoa, clube, evento, tipo));
    PLACAR.registrar(igual && r == n, || format!("situacao_na_lista_de_observacao(pessoa {:x}, clube {:x}, evento {}, tipo {}) original={} rust={} {:x?} x {:x?}",
        pessoa, clube, evento as u8, tipo as u8, r, n, mo, mr));
    r
}

/// { 0x8442b0 } (stdcall, ret 4, shortlist_manager.cpp): `shortlist_manager_add_to_loan_list(pessoa)`, pelas telas de
/// transferência (0x8f6770, 0x8f8120). Com contrato (0x4cf5a0, sem guardar em [0xae1080]), o bit 1 de +0x4e (na lista
/// de empréstimos) liga. Devolve o eax que a original deixa: o contrato.
unsafe extern "stdcall" fn por_na_lista_de_emprestimos(pessoa: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = f(cpy2_d(0xae1050), pessoa);
    if c != 0 {
        *((c + 0x4e) as *mut u8) |= 2;
    }
    c
}

/// Comparador: lado a lado pelo byte +0x4e do contrato, e a devolução.
unsafe extern "stdcall" fn comparar_por_na_lista_de_emprestimos(pessoa: usize) -> usize {
    let o = orig::<unsafe extern "stdcall" fn(usize) -> usize>(1348);
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = f(cpy2_d(0xae1050), pessoa);
    let reg = if c != 0 { vec![(c + 0x4e, 1)] } else { Vec::new() };
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(pessoa), || por_na_lista_de_emprestimos(pessoa));
    PLACAR.registrar(igual && r == n, || format!("por_na_lista_de_emprestimos(pessoa {:x}) original={:#x} rust={:#x} {:x?} x {:x?}", pessoa, r, n, mo, mr));
    r
}

/// { 0x84b1f0 } (thiscall, ret 0xc, shortlist_manager.cpp): `shortlist_manager_max_fee_for_player_status(clube, pessoa,
/// estatuto)`, pela tela de contrato (0x4dd0f0, ecx = o Shortlist_Manager em +0x856 da gerência de transferências).
/// Sem pessoa ou sem ficha de jogador (+0x61): −1. Senão o teto da vaga (0x84b230) com as posições fortes do jogador
/// (0x5b7d20) e o estatuto.
unsafe extern "thiscall" fn teto_de_preco_pelo_estatuto(this: usize, clube: usize, pessoa: usize, estatuto: u32) -> i32 {
    // 0x5b7d20: cdecl(ficha) (add esp, 4); 0x84b230: chamada com mov ecx, esi (o this) e 3 push (clube, posições,
    // estatuto), ret 0xc: thiscall daqui (a cópia stdcall do motor não lê o ecx)
    if pessoa == 0 || cpy2_d(pessoa + 0x61) == 0 {
        return -1;
    }
    let posicoes: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x5b7d20usize);
    let teto: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> i32 = core::mem::transmute(0x84b230usize);
    let m = posicoes(cpy2_d(pessoa + 0x61));
    teto(this, clube, m, estatuto)
}

/// Comparador: a devolução (rotina pura).
unsafe extern "thiscall" fn comparar_teto_de_preco_pelo_estatuto(this: usize, clube: usize, pessoa: usize, estatuto: u32) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32) -> i32>(1349);
    let r = o(this, clube, pessoa, estatuto);
    let n = teto_de_preco_pelo_estatuto(this, clube, pessoa, estatuto);
    PLACAR.registrar(r == n, || format!("teto_de_preco_pelo_estatuto(clube {:x}, pessoa {:x}, estatuto {}) original={} rust={}", clube, pessoa, estatuto as i8, r, n));
    r
}

// ---------- 261a leva: a verba de transferências repartida pelas vagas da lista de observação, o estatuto permitido pela idade e o nome das listas do olheiro (shortlist_manager.cpp, scout_manager.cpp) ----------

unsafe fn cpy3_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpy3_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpy3_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpy3_pi(a: usize, v: i32) {
    core::ptr::write_unaligned(a as *mut i32, v);
}

unsafe fn cpy3_f64(a: usize) -> f64 {
    core::ptr::read_unaligned(a as *const f64)
}

/// A pessoa de índice `i` ([0xae23bc], 0x6e bytes; a conta da original em 32 bits, com índice negativo inclusive).
unsafe fn cpy3_pessoa(i: i32) -> usize {
    cpy3_d(0xae23bc).wrapping_add((i as usize).wrapping_mul(0x6e))
}

/// { 0x83b6e0 } (thiscall, ret 4, shortlist_manager.cpp): `Shortlist_Manager::allocate_transfer_budget(clube)`, pela
/// atualização da lista do clube (0x83a730, já em Rust), pelas listas iniciais (0x83a1b0) e pela lista refeita
/// (0x84abe0). Clube do humano (0x600a50 com 0, 1), seleção (0x53b900), sem finanças (0x5a0590) ou lista fora do estado
/// 1: nada. A verba (0x598800 com 0, 0, 0, 1, 0; negativa: 0) se reparte pelas vagas abertas (alvo −1) depois de
/// ordenar as 40 (qsort 0x83b690): entre as primeiras 8 (ou até a quarta vaga de jogador aberta), cada vaga aberta
/// recebe em +7 a verba / (abertas × fator do tipo e do estatuto); o resto da verba passa pelas vagas ainda sem alvo
/// (reduções pelos lances guardados, pelo valor de +0xb sobre a soma e pelos fatores de 0x969540, 0x968968), cada uma
/// preenchida pela 0x83f3a0; o que sobra vai à primeira vaga de jogador do próprio clube que o técnico aceitaria vender
/// (0x846920 acima de 0x968e58, sorteios de 10 e 100 dias, ofertas 0x8c5cf0).
unsafe extern "thiscall" fn distribuir_verba_de_transferencias(this: usize, clube: usize) {
    // 0x600a50: cdecl(clube, 0, 1, 0, 0) (add esp, 0x14); 0x53b900: cdecl(clube) (add esp, 4); 0x5a0590:
    // thiscall([0xae2a38], clube); 0x598800: thiscall(finanças, 0, 0, 0, 1, 0) (5 push); 0x945333: cdecl(vetor, 0x28,
    // 0x25, 0x83b690) (add esp, 0x10); 0x8c61e0: thiscall([0xde1f54], lance) (1 push); 0x83f3a0: thiscall(this, vaga,
    // clube, 0) (mov ecx, [this local]; 3 push); 0x4cf5a0: thiscall([0xae1050], pessoa); 0x544590: cdecl(pessoa, 0)
    // (add esp, 8); 0x4cf5f0: thiscall([0xae1050], pessoa, clube) (2 push); 0x846920: thiscall(this, pessoa, clube, 1, 0,
    // estatuto, 0) (6 push, devolve em st0); 0x90cfc0: cdecl(n) (add esp, 4); 0x85fa00: thiscall(contrato) (sem push,
    // movsx ax); 0x8c5cf0: thiscall([0xde1f54], pessoa, 0, 0) (3 push); 0x54c8e0: thiscall(0xae2c90, saída, dias)
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    let verba_de: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x598800usize);
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let lance_de: unsafe extern "thiscall" fn(usize, i32) -> usize = core::mem::transmute(0x8c61e0usize);
    let preencher: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x83f3a0usize);
    if humano(clube, 0, 1, 0, 0) != 0 || selecao(clube) != 0 {
        return;
    }
    let fin = financas(cpy3_d(0xae2a38), clube);
    if fin == 0 {
        return;
    }
    let mut verba = verba_de(fin, 0, 0, 0, 1, 0) as i32;
    if verba < 0 {
        verba = 0;
    }
    let entrada = cpy3_d(this + 0xdc).wrapping_add(cpy3_d(clube).wrapping_mul(9));
    if cpy3_b(entrada + 4) != 1 {
        return;
    }
    ordenar(cpy3_d(entrada), 0x28, 0x25, 0x83b690);
    let vaga = |i: i32| cpy3_d(entrada).wrapping_add((i as usize).wrapping_mul(0x25));
    // as vagas abertas entre as primeiras 8, ou até a quarta vaga de jogador aberta
    let mut limite: i8 = 8;
    let mut abertas: i8 = 0;
    let mut jogadores: u8 = 0;
    let mut soma: i32 = 0;
    let mut i: i8 = 0;
    loop {
        let v = vaga(i as i32);
        if cpy3_i(v + 0xf) == -1 {
            if cpy3_b(v) == 0xb {
                jogadores = jogadores.wrapping_add(1);
                if jogadores == 4 {
                    limite = i.wrapping_add(1);
                }
            }
            soma = soma.wrapping_add(cpy3_i(v + 0xb));
            abertas = abertas.wrapping_add(1);
        }
        i = i.wrapping_add(1);
        if i >= limite {
            break;
        }
    }
    let a = abertas as i32;
    let mut repartido: i32 = 0;
    if limite > 0 {
        for k in 0..limite as i32 {
            let v = vaga(k);
            if cpy3_i(v + 0xf) != -1 {
                continue;
            }
            let t = cpy3_b(v);
            let divisor = if t == 0xb {
                match ((cpy3_b(v + 0x24) >> 1) & 0xf).wrapping_sub(2) {
                    0 | 1 => {
                        crate::comparacao::ramo("cpy_3 verba da vaga de titular");
                        Some(a * 2) // cpy3: a verba da vaga de titular
                    }
                    2 => Some(a * 4),
                    3 => Some(a * 10),
                    _ => None,
                }
            } else {
                if cpy3_i(v + 0x13) != -1 {
                    continue;
                }
                let rico = cpy3_b(clube + 0x64) == 1;
                match (t as i8 as i32).wrapping_sub(6) {
                    0 | 2 => Some(if rico { a * 10 } else { 0 }),
                    1 => Some(if rico { a * 1000 } else { 0 }),
                    3 | 4 => Some(if rico { a * 20 } else { 0 }),
                    _ => None,
                }
            };
            let Some(dv) = divisor else { continue };
            let x = if dv == 0 { 0 } else { verba.wrapping_div(dv) };
            cpy3_pi(v + 7, x);
            repartido = repartido.wrapping_add(x);
        }
    }
    let mut resto = verba.wrapping_sub(repartido);
    let mut baixos: i8 = 0;
    if limite > 0 {
        for k in 0..limite as i32 {
            let v = vaga(k);
            let t = cpy3_b(v);
            if t != 0xb && cpy3_i(v + 0x13) > -1 {
                continue;
            }
            let alvo = cpy3_i(v + 0xf);
            if alvo > -1 || t == 7 {
                continue;
            }
            let lance = cpy3_i(v + 0x17);
            if lance > -1 {
                if alvo == -1 {
                    cpy3_pi(v + 0x17, -1);
                } else if t == 0xb {
                    let l = cpy3_d(lance_de(cpy3_d(0xde1f54), lance));
                    if l != 0 {
                        resto = resto.wrapping_sub(cpy3_i(l + 0x1b));
                    }
                } else {
                    resto = resto.wrapping_sub(cpy3_i(cpy3_pessoa(alvo) + 0x52));
                }
                continue;
            }
            if cpy3_b(clube + 0x64) != 1 && t != 0xb {
                cpy3_pi(v + 7, 0);
            } else {
                let base = if repartido >= 1000 { repartido } else { 1000 };
                let e = cpy3_i(v + 7);
                let x = ftol((e as f64) / (base as f64) * (resto as f64) * cpy3_f64(0x969540));
                crate::comparacao::ramo("cpy_3 reducao pela sobra");
                cpy3_pi(v + 7, e.wrapping_sub(x)); // cpy3: a redução da vaga pela sobra
            }
            let marca = cpy3_b(v + 0x24) & 0x1e;
            if marca <= 6 {
                if baixos < 3 {
                    baixos = baixos.wrapping_add(1);
                    let s = if soma >= 1 { soma } else { 1 };
                    let b = cpy3_i(v + 0xb);
                    let b = if b >= 1 { b } else { 1 };
                    let x = ftol((resto as f64) / (s as f64) * (b as f64));
                    cpy3_pi(v + 7, cpy3_i(v + 7).wrapping_add(x));
                }
            } else {
                let s = if soma >= 1 { soma } else { 1 };
                let b = cpy3_i(v + 0xb);
                let b = if b >= 1 { b } else { 1 };
                let x = ftol((resto as f64) / (s as f64) * (b as f64) * cpy3_f64(0x968968));
                cpy3_pi(v + 7, cpy3_i(v + 7).wrapping_add(x));
            }
            *((v + 0x24) as *mut u8) &= 0xfe;
            crate::comparacao::ramo("cpy_3 vaga preenchida");
            preencher(this, v, clube, 0); // cpy3: a vaga é preenchida
            let alvo = cpy3_i(v + 0xf);
            if alvo > -1 && cpy3_b(v + 0x24) & 1 == 0 {
                resto = resto.wrapping_add(ftol(cpy3_i(cpy3_pessoa(alvo) + 0x52) as f64 * cpy3_f64(0x96f878)));
            }
            soma = soma.wrapping_sub(cpy3_i(v + 0xb));
        }
    }
    // a sobra vai à primeira vaga de jogador do próprio clube que o técnico aceitaria vender
    if limite >= 0x28 {
        return;
    }
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let emprestado: unsafe extern "C" fn(usize, u32) -> u32 = core::mem::transmute(0x544590usize);
    let contrato_no: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let interesse: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32) -> f64 = core::mem::transmute(0x846920usize);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let fim: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x85fa00usize);
    let ofertas: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x8c5cf0usize);
    let menos_dias: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(0x54c8e0usize);
    let mut k = limite;
    while resto > 0 {
        let v = vaga(k as i32);
        let alvo = cpy3_i(v + 0xf);
        let candidata = 'c: {
            if cpy3_b(v) != 0xb || alvo <= -1 || cpy3_i(v + 0x17) != -1 {
                break 'c false;
            }
            let p = cpy3_pessoa(alvo);
            if cpy3_d(p + 0x39) != clube {
                break 'c false;
            }
            let c = contrato_de(cpy3_d(0xae1050), p);
            *(0xae1080 as *mut usize) = c;
            let do_contrato = if c != 0 { cpy3_d(0xae23b4).wrapping_add(cpy3_d(c + 4).wrapping_mul(0x245)) } else { 0 };
            if do_contrato == clube || emprestado(p, 0) != 0 {
                break 'c false;
            }
            let c2 = contrato_no(cpy3_d(0xae1050), p, clube);
            let estatuto = if c2 != 0 { (cpy3_b(c2 + 0x4f) >> 4) as u32 } else { 4 };
            let f = interesse(this, p, clube, 1, 0, estatuto, 0);
            if !(f >= cpy3_f64(0x968e58)) {
                break 'c false;
            }
            if c2 != 0 {
                let lim = 20 - sorteio(10);
                if (fim(c2) as i16 as i32) < lim {
                    break 'c true;
                }
            }
            if ofertas(cpy3_d(0xde1f54), p, 0, 0) != 0 {
                break 'c true;
            }
            if c2 == 0 {
                break 'c false;
            }
            let mut buf = [0u8; 8];
            let r = menos_dias(0xae2c90, &mut buf, (sorteio(100) + 0x1e) as u32);
            let dia = core::ptr::read_unaligned(r as *const i16);
            let ano = core::ptr::read_unaligned((r + 2) as *const u16);
            let ano_c = core::ptr::read_unaligned((c2 + 0x27) as *const u16);
            ano > ano_c || (ano == ano_c && dia > core::ptr::read_unaligned((c2 + 0x25) as *const i16))
        };
        if candidata {
            crate::comparacao::ramo("cpy_3 sobra ao jogador do clube");
            cpy3_pi(v + 0xb, cpy3_i(v + 3));
            cpy3_pi(v + 7, resto);
            return;
        }
        k = k.wrapping_add(1);
        if k >= 0x28 {
            return;
        }
    }
}

// ponytail: a 0x83f3a0 (chamada por cada vaga ainda sem alvo) sorteia, acrescenta pares aos vetores com realocação e dá
// notícias: não pode rodar duas vezes. Só a original; a cópia leva os ramos e as mutações.
unsafe extern "thiscall" fn comparar_distribuir_verba_de_transferencias(this: usize, clube: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize)>(1350)(this, clube)
}

/// { 0x84b4d0 } (stdcall, ret 8, shortlist_manager.cpp; chamada com ecx = o Shortlist_Manager, que ela não lê):
/// `shortlist_manager_squad_status_allowed(pessoa, estatuto)`, pela tela de contrato (0x4dd0f0) e pela de transferência
/// (0x8f6770). Os estatutos 5 e 6 (jovens) só abaixo de 24 anos (+0x18, com sinal); os outros, 1. Sem ficha de jogador
/// (+0x61): a original (caixa "Error", 0).
unsafe extern "stdcall" fn estatuto_permitido(pessoa: usize, estatuto: u32) -> u32 {
    if cpy3_d(pessoa + 0x61) == 0 {
        return orig::<unsafe extern "stdcall" fn(usize, u32) -> u32>(1351)(pessoa, estatuto); // caixa: só a original
    }
    match estatuto as u8 {
        5 | 6 => ((cpy3_b(pessoa + 0x18) as i8) < 0x18) as u32,
        _ => 1,
    }
}

/// Comparador: a devolução (rotina pura); sem ficha de jogador, só a original (caixa).
unsafe extern "stdcall" fn comparar_estatuto_permitido(pessoa: usize, estatuto: u32) -> u32 {
    let o = orig::<unsafe extern "stdcall" fn(usize, u32) -> u32>(1351);
    if cpy3_d(pessoa + 0x61) == 0 {
        return o(pessoa, estatuto);
    }
    let r = o(pessoa, estatuto);
    let n = estatuto_permitido(pessoa, estatuto);
    PLACAR.registrar(r == n, || format!("estatuto_permitido(pessoa {:x}, estatuto {}) original={} rust={}", pessoa, estatuto as i8, r, n));
    r
}

/// { 0x7fc150 } (cdecl, scout_manager.cpp): `scout_manager_write_list_name(indice, texto, tamanho)`, pelo construtor do
/// SCOUT_MANAGER (0x7f6760) e pelas telas de busca. O nome traduzido (0x66f4e0) da lista `indice`: 0 "Short List", 1
/// "Player Search", 2 "Staff Search", 3..9 "Scout #<%d - number>" com `indice` − 2. Devolve 1. Índice fora de 0..9,
/// texto nulo ou tamanho abaixo de 50: a original (caixa "Error", 0).
unsafe extern "C" fn nome_da_lista_do_olheiro(indice: u32, texto: usize, tamanho: i32) -> u32 {
    // 0x66f4e0: cdecl(texto, formato, ...) (add esp, 8 ou 0xc)
    let k = indice as u8 as i8;
    if !(0..=9).contains(&k) || texto == 0 || tamanho < 0x32 {
        return orig::<unsafe extern "C" fn(u32, usize, i32) -> u32>(1352)(indice, texto, tamanho); // caixa: só a original
    }
    let traduzir: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    match k {
        0 => traduzir(texto, 0xa7dd94),
        1 => traduzir(texto, 0xa7dd84),
        2 => traduzir(texto, 0xa7dd74),
        _ => traduzir(texto, 0xa7dd5c, k as i32 - 2),
    };
    1
}

/// Comparador: lado a lado pelo texto (até 0x100 bytes do tamanho dado), e a devolução; nos casos de erro, só a
/// original (caixa).
unsafe extern "C" fn comparar_nome_da_lista_do_olheiro(indice: u32, texto: usize, tamanho: i32) -> u32 {
    let o = orig::<unsafe extern "C" fn(u32, usize, i32) -> u32>(1352);
    let k = indice as u8 as i8;
    if !(0..=9).contains(&k) || texto == 0 || tamanho < 0x32 {
        return o(indice, texto, tamanho);
    }
    let reg = [(texto, (tamanho as usize).min(0x100))];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(indice, texto, tamanho), || nome_da_lista_do_olheiro(indice, texto, tamanho));
    PLACAR.registrar(igual && r == n, || format!("nome_da_lista_do_olheiro(indice {}) original={} rust={} {:x?} x {:x?}", k, r, n, mo, mr));
    r
}

// ---------- 252a leva: a lista de alvos dos clubes: as vagas da formação, refazer a lista do clube e suspender os clubes que miram o jogador (shortlist_manager.cpp) ----------

unsafe fn cpp5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpp5_w(a: usize, v: u16) {
    core::ptr::write_unaligned(a as *mut u16, v);
}

/// A entrada de 9 bytes do clube na Shortlist_Manager (+0xdc; +0 as 40 vagas de 0x25 bytes, +8 o estado).
unsafe fn cpp5_entrada(this: usize, clube: usize) -> usize {
    cpp5_d(this + 0xdc).wrapping_add(cpp5_d(clube).wrapping_mul(9))
}

/// A primeira diferença entre duas leituras de memória (endereço, original, Rust), para a mensagem.
fn cpp5_dif(a: &crate::comparacao::Memoria, b: &crate::comparacao::Memoria) -> Option<(usize, u8, u8)> {
    for ((ea, va), (_, vb)) in a.0.iter().zip(b.0.iter()) {
        if let Some(k) = (0..va.len()).find(|&k| va[k] != vb[k]) {
            return Some((ea + k, va[k], vb[k]));
        }
    }
    None
}

/// As máscaras de posição (+1, 16 bits) das vagas 1..24 de jogador, por formação (+7 da entrada); a vaga 0 é sempre
/// o goleiro (1). A linha 0 vale para formação fora de 1..9 (o `default` do switch).
static CPP5_MASCARAS: [[u16; 24]; 10] = [
    [0x804, 0x84, 0x404, 0x104, 0x810, 0x90, 0x410, 0x110, 0x440, 0x140, 0x4, 0x8, 0x10, 0x10, 0x20, 0x40, 0x4, 0x10, 0x1, 0x40, 0x4, 0x10, 0x10, 0x1], // fora de 1..9
    [0x808, 0x88, 0x404, 0x2, 0x104, 0x410, 0x220, 0x110, 0x440, 0x140, 0x8, 0x210, 0x90, 0x810, 0x240, 0x220, 0x2, 0x210, 0x1, 0x40, 0x4, 0x8, 0x10, 0x1], // formação 1
    [0x808, 0x88, 0x404, 0x204, 0x104, 0x410, 0x220, 0x110, 0x440, 0x140, 0x8, 0x210, 0x90, 0x810, 0x240, 0x220, 0x204, 0x210, 0x1, 0x40, 0x4, 0x8, 0x10, 0x1], // formação 2
    [0x804, 0x84, 0x404, 0x104, 0x810, 0x90, 0x410, 0x110, 0x440, 0x140, 0x4, 0x4, 0x10, 0x10, 0x220, 0x240, 0x204, 0x210, 0x1, 0x40, 0x4, 0x8, 0x10, 0x1], // formação 3
    [0x804, 0x84, 0x404, 0x104, 0x210, 0x90, 0x810, 0x140, 0x440, 0x240, 0x4, 0x4, 0x10, 0x10, 0x140, 0x440, 0x4, 0x10, 0x1, 0x40, 0x4, 0x10, 0x40, 0x1], // formação 4
    [0x804, 0x84, 0x404, 0x104, 0x820, 0xa0, 0x410, 0x110, 0x440, 0x140, 0x4, 0x4, 0x10, 0x20, 0x440, 0x140, 0x204, 0x10, 0x1, 0x40, 0x4, 0x20, 0x10, 0x1], // formação 5
    [0x810, 0x90, 0x404, 0x204, 0x104, 0x410, 0x208, 0x110, 0x440, 0x140, 0x8, 0x210, 0x90, 0x810, 0x240, 0x220, 0x204, 0x8, 0x1, 0x40, 0x4, 0x10, 0x10, 0x1], // formação 6
    [0x804, 0x84, 0x404, 0x104, 0x208, 0x220, 0x410, 0x110, 0x140, 0x440, 0x4, 0x4, 0x10, 0x10, 0x220, 0x240, 0x204, 0x10, 0x1, 0x40, 0x4, 0x10, 0x10, 0x1], // formação 7
    [0x804, 0x84, 0x404, 0x104, 0x810, 0x90, 0x410, 0x110, 0x210, 0x240, 0x4, 0x4, 0x10, 0x10, 0x220, 0x240, 0x204, 0x10, 0x1, 0x40, 0x4, 0x10, 0x10, 0x1], // formação 8
    [0x204, 0x404, 0x104, 0x810, 0x90, 0x410, 0x110, 0x210, 0x140, 0x440, 0x4, 0x4, 0x10, 0x10, 0x220, 0x240, 0x204, 0x10, 0x1, 0x40, 0x4, 0x10, 0x10, 0x1], // formação 9
];

/// { 0x83c090 } (stdcall, ret 4, shortlist_manager.cpp): `shortlist_set_up_formation_slots(entrada)`: nas 40 vagas de
/// 0x25 bytes da entrada (+0), as 25 primeiras viram vagas de jogador (+0 = 0xb) e as 15 seguintes recebem o papel
/// (6, 7, 8 ×5, 10, 9 ×7) com +0x24 = (+0x24 & 0xe3) | 2; depois grava a máscara de posição (+1, 16 bits) das 25 vagas
/// de jogador pela formação (+7, byte com sinal; fora de 1..9 a linha padrão). A tabela saiu da emulação do switch
/// (cada caso grava as 25 vagas uma vez só).
unsafe extern "stdcall" fn montar_vagas_da_formacao(entrada: usize) {
    for k in 0..40usize {
        let s = cpp5_d(entrada) + k * 0x25;
        if k < 25 {
            *(s as *mut u8) = 0xb;
        } else {
            let b = (s + 0x24) as *mut u8;
            *b = (*b & 0xe3) | 2;
            *(s as *mut u8) = match k - 25 {
                0 => 6,
                1 => 7,
                2..=6 => 8,
                7 => 10,
                _ => 9,
            };
        }
    }
    let f = *((entrada + 7) as *const i8) as i32;
    let linha = if (1..=9).contains(&f) { f as usize } else { 0 };
    crate::comparacao::ramo("cpp_5 vagas da formacao");
    let v = cpp5_d(entrada);
    cpp5_w(v + 1, 1); // o goleiro
    for k in 1..25usize {
        cpp5_w(v + 1 + k * 0x25, CPP5_MASCARAS[linha][k - 1]);
    }
}

/// { 0x83a730 } (thiscall, ret 4, shortlist_manager.cpp): `Shortlist_Manager::update_club_shortlist(clube)`: solta os
/// alvos velhos das vagas do clube (0x844660 com a entrada), reavalia o elenco (0x83cb40 com a entrada recalculada,
/// o clube, diário 1, refazer 0) e redistribui a verba de transferências (0x83b6e0).
unsafe extern "thiscall" fn refazer_lista_do_clube(this: usize, clube: usize) {
    let soltar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x844660usize);
    let avaliar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(0x83cb40usize);
    let verba: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x83b6e0usize);
    soltar(this, clube, cpp5_entrada(this, clube));
    avaliar(this, cpp5_entrada(this, clube), clube, 1, 0);
    crate::comparacao::ramo("cpp_5 refazer lista");
    verba(this, clube); // a verba de transferências
}

/// { 0x842b00 } (thiscall, ret 8, shortlist_manager.cpp, quadro SEH só para a lista nova):
/// `Shortlist_Manager::suspend_clubs_shortlisting_player(jogador, limpar)`: numa LLIST nova (0x944cf1(0xe) e 0x68d020)
/// junta os clubes que miram o jogador (0x842180 com ignorar 1, modo 3, todos); para cada item (0x68d1b0, depois de
/// recomeçar o cursor +0xa = +2) põe o estado da entrada do clube (+8) em 100 e, com `limpar`, tira a marca de lance
/// (+0x17 = −1 quando > −1) das vagas que têm o jogador (+0xf = id). Destrói e libera a lista (0x68d040, 0x944c94).
/// A contagem (+0, 16 bits com sinal) é relida a cada volta, como na original.
unsafe extern "thiscall" fn suspender_clubes_que_miram_o_jogador(this: usize, jogador: usize, limpar: u8) {
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let interessados: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(0x842180usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
    let p = novo(0xe);
    let lista = if p != 0 { lista_nova(p) } else { 0 };
    interessados(this, jogador, lista, 1, 3, 0);
    let n = *(lista as *const i16);
    if n != 0 {
        core::ptr::write_unaligned((lista + 0xa) as *mut usize, cpp5_d(lista + 2));
    }
    if n > 0 {
        let mut i = 0i32;
        while i < *(lista as *const i16) as i32 {
            let item = proximo(lista);
            if item != 0 {
                let e = cpp5_d(this + 0xdc).wrapping_add(cpp5_d(item + 4).wrapping_mul(9));
                crate::comparacao::ramo("cpp_5 suspender clube");
                *((e + 8) as *mut u8) = 0x64; // o clube suspende o jogador
                if limpar != 0 {
                    for k in 0..0x28usize {
                        let s = cpp5_d(e) + k * 0x25;
                        if cpp5_d(s + 0xf) == cpp5_d(jogador) && core::ptr::read_unaligned((s + 0x17) as *const i32) > -1 {
                            core::ptr::write_unaligned((s + 0x17) as *mut i32, -1);
                        }
                    }
                }
            }
            i += 1;
        }
    }
    if lista != 0 {
        destruir(lista);
        liberar(lista);
    }
}

/// Comparador: lado a lado pelas 40 vagas da entrada (sem chamadas).
unsafe extern "stdcall" fn comparar_montar_vagas_da_formacao(entrada: usize) {
    let o = orig::<unsafe extern "stdcall" fn(usize)>(1018);
    let reg = [(cpp5_d(entrada), 40 * 0x25)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(entrada), || montar_vagas_da_formacao(entrada));
    PLACAR.registrar(igual, || format!("montar_vagas_da_formacao(formacao {}) primeira diferença {:x?}", *((entrada + 7) as *const i8), cpp5_dif(&mo, &mr)));
}

/// Comparador: a 0x83cb40 reavalia o elenco (sorteios, ofertas, vagas de muitos clubes): roda só a original.
// ponytail: só a original; a cópia é provada pelo ramo e pela mutação na carreira.
unsafe extern "thiscall" fn comparar_refazer_lista_do_clube(this: usize, clube: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize)>(1019)(this, clube)
}

/// Comparador: lado a lado pelas entradas (9 bytes) e, com `limpar`, pelas vagas (40 × 0x25) dos clubes do plano: os
/// clubes que a 0x842180 (pura) junta antes, numa lista própria. Cada lado cria e libera a sua lista.
unsafe extern "thiscall" fn comparar_suspender_clubes_que_miram_o_jogador(this: usize, jogador: usize, limpar: u8) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u8)>(1020);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let interessados: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(0x842180usize);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
    let p = novo(0xe);
    if p == 0 {
        return o(this, jogador, limpar);
    }
    let lista = lista_nova(p);
    interessados(this, jogador, lista, 1, 3, 0);
    // os itens (8 bytes: id do jogador, id do clube) pelos nós (+2 o primeiro; nó: +0 o dado, +6 o próximo)
    let mut reg: Vec<(usize, usize)> = Vec::new();
    let mut no = cpp5_d(lista + 2);
    while no != 0 {
        let e = cpp5_d(this + 0xdc).wrapping_add(cpp5_d(cpp5_d(no) + 4).wrapping_mul(9));
        reg.push((e, 9));
        if limpar != 0 {
            reg.push((cpp5_d(e), 0x28 * 0x25));
        }
        no = cpp5_d(no + 6);
    }
    destruir(lista);
    liberar(lista);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, jogador, limpar), || suspender_clubes_que_miram_o_jogador(this, jogador, limpar));
    PLACAR.registrar(igual, || format!("suspender_clubes_que_miram_o_jogador(jogador {:x}, limpar {}, {} regiões) primeira diferença {:x?}", cpp5_d(jogador), limpar, reg.len(), cpp5_dif(&mo, &mr)));
}

// ---------- 252a leva: pôr o jogador à venda, pô-lo nas listas de alvos dos clubes e pagar o prêmio por jogos da transferência (shortlist_manager.cpp, transfer_manager.cpp) ----------

unsafe fn cpp6_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpp6_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O clube de id `id` (registros de 0x245 bytes em [0xae23b4]).
unsafe fn cpp6_clube(id: usize) -> usize {
    cpp6_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// O contrato da pessoa (0x4cf5a0 no CONTRACT_MANAGER [0xae1050]).
unsafe fn cpp6_contrato(pessoa: usize) -> usize {
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    contrato_de(cpp6_d(0xae1050), pessoa)
}

/// O contrato da pessoa, guardado em [0xae1080] como a original faz depois de cada chamada.
unsafe fn cpp6_contrato_guardado(pessoa: usize) -> usize {
    let c = cpp6_contrato(pessoa);
    *(0xae1080 as *mut usize) = c;
    c
}

/// { 0x842c00 } (thiscall, ret 8, shortlist_manager.cpp): `Shortlist_Manager::add_player_to_club_shortlists(jogador,
/// forçar)`: com ficha de posições (+0x61; sem forçar, nota +0xb ≥ 0x157c), fora da marca 0x10 dos bits 0x30 do registro
/// de 79 bytes (+0xb), jogador (+0x3d = 0xb), não em fim de carreira (+0x24 com +0x28 = 5), fora da nação [0x9cf458]
/// (+0x1a) e sem ser intocável (0x8cca10 = 0): se chegou ao clube há menos de 200 dias (hoje − 200, 0x54c8e0, antes da
/// data +0x3e/+0x40) e o contrato não está à venda (+0x4f nibble alto ≠ 7) nem tem +0x45, sai. Senão, com o clube do
/// contrato (guardado em [0xae1080]), o contador (nibble baixo do +0xb do registro de 79 bytes) e a máscara de
/// posições (0x5b7d20 com os vizinhos: 2→4, 8→0x10, 0x20→0x10, 0x80→0x100, 0x800→0x400), percorre os clubes do vetor
/// [0xae2d1c] ([0xae235c] relido a cada volta) até o contador chegar a 5: fora seleção (0x53b900), o clube do contrato e
/// o do jogador (+0x39), tenta a 0x842e10 e conta cada clube que o pôs na lista.
unsafe extern "thiscall" fn por_jogador_nas_listas_dos_clubes(this: usize, jogador: usize, forcar: u32) {
    let fichas = cpp6_d(jogador + 0x61);
    if fichas == 0 {
        return;
    }
    if forcar == 0 && core::ptr::read_unaligned((fichas + 0xb) as *const i16) < 0x157c {
        return;
    }
    let registro = |j: usize| cpp6_d(0xae2d0c).wrapping_add(cpp6_d(j).wrapping_mul(0x4f));
    if cpp6_b(registro(jogador) + 0xb) & 0x30 == 0x10 {
        return;
    }
    if cpp6_b(jogador + 0x3d) != 0xb {
        return;
    }
    if cpp6_d(jogador + 0x24) != 0 && cpp6_b(jogador + 0x28) == 5 {
        return;
    }
    let nacao = cpp6_d(jogador + 0x1a);
    if nacao != 0 && cpp6_d(nacao) == cpp6_d(0x9cf458) {
        return;
    }
    let fora: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x8cca10usize);
    if fora(cpp6_d(0xde1f54), jogador, 0) != 0 {
        return;
    }
    if cpp6_d(jogador + 0x39) != 0 {
        let menos: unsafe extern "thiscall" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(0x54c8e0usize);
        let mut t = [0u32; 2];
        let p = menos(0xae2c90, t.as_mut_ptr(), 200);
        let lo = cpp6_d(p) as u32;
        let (dia, ano) = (lo as u16 as i16, (lo >> 16) as u16);
        let ano_j = core::ptr::read_unaligned((jogador + 0x40) as *const u16);
        let dia_j = core::ptr::read_unaligned((jogador + 0x3e) as *const i16);
        let recente = if ano != ano_j { ano < ano_j } else { dia < dia_j };
        if recente {
            let c = cpp6_contrato(jogador);
            if c != 0 && cpp6_b(c + 0x4f) & 0xf0 != 0x70 && cpp6_d(c + 0x45) == 0 {
                return;
            }
        }
    }
    let c = cpp6_contrato_guardado(jogador);
    let clube_do_contrato = if c != 0 { cpp6_clube(cpp6_d(c + 4)) } else { 0 };
    let mut conta = (cpp6_b(registro(jogador) + 0xb) & 0xf) as i8;
    let mascara: unsafe extern "C" fn(usize) -> u16 = core::mem::transmute(0x5b7d20usize);
    // ponytail: a 0x5b7d20 devolve só ax e a 0x842e10 usa 16 bits da máscara: os 16 de cima vão zerados
    let mut m = mascara(cpp6_d(jogador + 0x61)) as u32;
    if m & 2 != 0 {
        m |= 4;
    }
    if m & 8 != 0 {
        m |= 0x10;
    }
    if m & 0x20 != 0 {
        m |= 0x10;
    }
    if m & 0x80 != 0 {
        m |= 0x100;
    }
    if m & 0x800 != 0 {
        m |= 0x400;
    }
    if *(0xae235c as *const i32) <= 0 {
        return;
    }
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let acrescentar: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x842e10usize);
    let clubes = cpp6_d(0xae2d1c);
    let mut i = 0i32;
    loop {
        let clube = cpp6_clube(cpp6_d(clubes + i as usize * 4));
        if conta >= 5 {
            return;
        }
        if selecao(clube) == 0 && clube_do_contrato != clube && cpp6_d(jogador + 0x39) != clube && acrescentar(this, jogador, clube, m) != 0 {
            crate::comparacao::ramo("cpp_6 clube acrescentou");
            conta = conta.wrapping_add(1); // mais um clube com ele na lista
        }
        i += 1;
        if i >= *(0xae235c as *const i32) {
            return;
        }
    }
}

/// { 0x842320 } (thiscall, ret 8, shortlist_manager.cpp): `Shortlist_Manager::transfer_list_player(jogador, notícia)`:
/// se o clube ainda não o listou (0x5444a0 com 0) e há contrato: marca o contrato à venda (+0x4e bit 1, +0x4f nibble
/// alto 7); se não é intocável (0x8cca10): com o jogador gostando do clube (0x547d60) ou com a lealdade (+0x44 do
/// contrato, com sinal) acima da diferença de dois sorteios(100), o jogador passa a desgostar do técnico do clube
/// (0x547af0 com o +0xcf do clube, 1); com `notícia`, põe o jogador nas listas dos clubes (0x842c00 com 1) e dá a
/// notícia (0x4d71b0 com o clube +0x39).
unsafe extern "thiscall" fn por_jogador_a_venda(this: usize, jogador: usize, noticia: u32) {
    let listada: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x5444a0usize);
    if listada(jogador, 0) != 0 {
        return;
    }
    let c = cpp6_contrato(jogador);
    if c == 0 {
        return;
    }
    let a = cpp6_b(c + 0x4f);
    let b = cpp6_b(c + 0x4e);
    *((c + 0x4e) as *mut u8) = b | 1;
    crate::comparacao::ramo("cpp_6 por a venda");
    *((c + 0x4f) as *mut u8) = (a & 0xf) | 0x70; // à venda
    let fora: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x8cca10usize);
    if fora(cpp6_d(0xde1f54), jogador, 0) != 0 {
        return;
    }
    let gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547d60usize);
    let desgostar = if gosta(jogador, cpp6_d(jogador + 0x39)) != 0 {
        true
    } else {
        let lealdade = cpp6_b(c + 0x44) as i8 as i32;
        let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
        let s1 = sorteio(100);
        let s2 = sorteio(100);
        lealdade > s1.wrapping_sub(s2)
    };
    if desgostar {
        let antipatia: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x547af0usize);
        antipatia(jogador, cpp6_d(cpp6_d(jogador + 0x39) + 0xcf), 1);
    }
    if noticia != 0 {
        let nas_listas: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x842c00usize);
        let dar_noticia: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x4d71b0usize);
        nas_listas(this, jogador, 1);
        dar_noticia(cpp6_d(0xae1050), jogador, cpp6_d(jogador + 0x39));
    }
}

/// O registro de prêmio por jogos (0xd bytes: +0 id da pessoa, +4 valor, +8 jogos, +9 id do clube antigo) da pessoa,
/// pela busca binária (0x946464 com 0x8b4e00) no vetor das seleções (tipo 5: +0x898, conta +0x8a0) ou do clube
/// (+0x88c, conta +0x894) do TRANSFER_MANAGER.
unsafe fn cpp6_registro_do_premio(this: usize, jogador: usize, tipo: u32) -> usize {
    let bsearch: unsafe extern "C" fn(*const u32, usize, usize, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    let chave = [cpp6_d(jogador) as u32];
    let (vetor, conta) = if tipo as u8 == 5 { (cpp6_d(this + 0x898), cpp6_d(this + 0x8a0)) } else { (cpp6_d(this + 0x88c), cpp6_d(this + 0x894)) };
    bsearch(chave.as_ptr(), vetor, conta, 0xd, 0x8b4e00)
}

/// { 0x8cb340 } (thiscall, ret 0x10, transfer_manager.cpp): `TRANSFER_MANAGER::pay_appearance_money(jogador, jogos,
/// tipo, arg)`: com contrato (guardado em [0xae1080]) e o registro do prêmio da pessoa (o `tipo` vai inteiro, como a
/// original o empurra) cujos jogos (+8, com sinal) são os `jogos` (> 0): dá a notícia (0x8db8a0), soma o valor (+4)
/// aos totais de transferência (0x448ad0 em [0xad9dac], com o clube atual e o antigo), o clube antigo (+9) recebe
/// (0x599800) e o atual paga (0x599890), em 64 bits com sinal. Devolve 1 se pagou.
unsafe extern "thiscall" fn pagar_premio_por_jogos(this: usize, jogador: usize, jogos: i32, tipo: u32, arg: u32) -> u32 {
    let c = cpp6_contrato_guardado(jogador);
    if c == 0 || cpp6_clube(cpp6_d(c + 4)) == 0 {
        return 0;
    }
    let r = cpp6_registro_do_premio(this, jogador, tipo);
    if r == 0 {
        return 0;
    }
    // a original: jogos diferentes só seguem com −1, que cai no teste seguinte (≤ 0)
    if cpp6_b(r + 8) as i8 as i32 != jogos || jogos <= 0 {
        return 0;
    }
    let noticia: unsafe extern "thiscall" fn(usize, usize, i32, u32, u32) = core::mem::transmute(0x8db8a0usize);
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    let somar: unsafe extern "thiscall" fn(usize, i32, usize, usize) -> u32 = core::mem::transmute(0x448ad0usize);
    let receber: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(0x599800usize);
    let pagar: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(0x599890usize);
    noticia(this, r, jogos, tipo, arg);
    let f_antigo = financas(cpp6_d(0xae2a38), cpp6_clube(cpp6_d(r + 9)));
    let c = cpp6_contrato_guardado(jogador);
    let f_atual = financas(cpp6_d(0xae2a38), if c != 0 { cpp6_clube(cpp6_d(c + 4)) } else { 0 });
    let c = cpp6_contrato_guardado(jogador);
    let atual = if c != 0 { cpp6_clube(cpp6_d(c + 4)) } else { 0 };
    let valor = core::ptr::read_unaligned((r + 4) as *const i32);
    somar(cpp6_d(0xad9dac), valor, atual, cpp6_clube(cpp6_d(r + 9)));
    crate::comparacao::ramo("cpp_6 premio por jogos");
    receber(f_antigo, valor as u32, (valor >> 31) as u32); // o clube antigo recebe
    pagar(f_atual, valor as u32, (valor >> 31) as u32);
    1
}

/// Comparador: a 0x842e10 mexe nas vagas de muitos clubes (e as listas não cabem numa cópia barata): roda só a
/// original.
// ponytail: só a original; a cópia é provada pelo ramo e pela mutação na carreira.
unsafe extern "thiscall" fn comparar_por_jogador_nas_listas_dos_clubes(this: usize, jogador: usize, forcar: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(1021)(this, jogador, forcar)
}

/// Comparador: sorteia e dá notícia: roda só a original e confere o plano feito antes: sem listagem do clube e com
/// contrato, o contrato fica à venda (+0x4e bit 1, +0x4f nibble alto 7).
unsafe extern "thiscall" fn comparar_por_jogador_a_venda(this: usize, jogador: usize, noticia: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(1022);
    let listada: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x5444a0usize);
    let marca = listada(jogador, 0) == 0 && cpp6_contrato(jogador) != 0;
    o(this, jogador, noticia);
    if marca {
        let c = cpp6_contrato(jogador);
        let ok = c != 0 && cpp6_b(c + 0x4e) & 1 != 0 && cpp6_b(c + 0x4f) & 0xf0 == 0x70;
        PLACAR.registrar(ok, || format!("por_jogador_a_venda(jogador {:x}) contrato {:x} sem a marca de à venda", cpp6_d(jogador), c));
    }
}

/// Comparador: dá notícia e mexe nas finanças: roda só a original e confere a devolução com o plano (contrato, clube,
/// registro do prêmio e jogos, sem efeitos).
unsafe extern "thiscall" fn comparar_pagar_premio_por_jogos(this: usize, jogador: usize, jogos: i32, tipo: u32, arg: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, i32, u32, u32) -> u32>(1023);
    let c = cpp6_contrato(jogador);
    let esperado = if c == 0 || cpp6_clube(cpp6_d(c + 4)) == 0 {
        0
    } else {
        let r = cpp6_registro_do_premio(this, jogador, tipo);
        (r != 0 && cpp6_b(r + 8) as i8 as i32 == jogos && jogos > 0) as u32
    };
    let r = o(this, jogador, jogos, tipo, arg);
    PLACAR.registrar(r == esperado, || format!("pagar_premio_por_jogos(jogador {:x}, jogos {}, tipo {}) devolveu {}, esperava {}", cpp6_d(jogador), jogos, tipo as u8, r, esperado));
    r
}

// ---------- 251a leva: as listas de posição da lista de observação: montar as listas, ordená-las a cada dia e os clubes interessados no jogador (shortlist_manager.cpp) ----------

unsafe fn cpo8_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpo8_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpo8_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpo8_w(a: usize, v: u32) {
    core::ptr::write_unaligned(a as *mut u32, v)
}

/// O registro de clube de id `id` ([0xae23b4], 0x245 bytes).
unsafe fn cpo8_clube(id: usize) -> usize {
    cpo8_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// A pessoa de id `id` ([0xae23bc], 0x6e bytes).
unsafe fn cpo8_pessoa(id: usize) -> usize {
    cpo8_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

/// O contrato principal da pessoa (0x4cf5a0, guardado em [0xae1080]), o clube dele (id em +4) e o contrato com esse
/// clube (0x4cf5f0).
unsafe fn cpo8_contrato_com_o_clube(p: usize) -> usize {
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let com_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let c = contrato_de(cpo8_d(0xae1050), p);
    *(0xae1080 as *mut usize) = c;
    let clube = if c != 0 { cpo8_clube(cpo8_d(c + 4)) } else { 0 };
    com_clube(cpo8_d(0xae1050), p, clube)
}

/// { 0x839e60 } (thiscall, ret 4, shortlist_manager.cpp): `Shortlist_Manager::build_position_lists(não usado)`: libera
/// os 32 vetores das listas de posição (+0x18: 16 pela pessoa, +0x58: 16 pela chave do contrato); o tamanho deles
/// (+0xd8) é o maior número de pessoas (todas, humanos inclusive) que servem para uma das 16 vagas (0x844300); aloca os
/// 32 com 12 bytes por entrada (sem memória: a caixa "Error" e o jogo para) e zera as contas (+0x98). Depois, cada
/// pessoa fora dos 16 humanos, em cada vaga que serve (conta no limite: a caixa e o jogo para): as duas entradas com o
/// id dela (+0), o id da nação (+4; sem nação em +0x1a, a 0x90cfc0 sorteia uma e a grava na pessoa), na primeira o valor
/// (+0x52) e na segunda o +0x3d do contrato com o clube do contrato (sem contrato: 100). Roda a cada 30 dias.
unsafe extern "thiscall" fn montar_listas_de_posicao(this: usize, _nao_usado: u32) {
    let livrar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    // ponytail: a original empurra a vaga num dword com 3 bytes de lixo da pilha acima; a 0x844300 só lê o byte
    let serve: unsafe extern "stdcall" fn(usize, u32) -> u8 = core::mem::transmute(0x844300usize);
    crate::comparacao::ramo("cpo_8 montar listas");
    for k in 0..32usize {
        let v = this + 0x18 + k * 4;
        if cpo8_d(v) != 0 {
            livrar(cpo8_d(v));
            cpo8_w(v, 0);
        }
    }
    cpo8_w(this + 0xd8, 0);
    for vaga in 0..16u32 {
        let base = cpo8_d(0xae23bc);
        let mut n = 0i32;
        let mut i = 0usize;
        while (i as i32) < cpo8_i(0xae2364) {
            if serve(base.wrapping_add(i * 0x6e), vaga) != 0 {
                n += 1;
            }
            i += 1;
        }
        if n > cpo8_i(this + 0xd8) {
            cpo8_w(this + 0xd8, n as u32);
        }
    }
    for k in 0..32usize {
        let v = alocar(cpo8_d(this + 0xd8).wrapping_mul(12) as u32);
        cpo8_w(this + 0x18 + k * 4, v as u32);
        if v == 0 {
            erro_em(0xa825a4, 0x10c);
        }
    }
    for k in 0..16usize {
        cpo8_w(this + 0x98 + k * 4, 0);
    }
    let base = cpo8_d(0xae23bc);
    let mut i = 0usize;
    while (i as i32) < cpo8_i(0xae2364).wrapping_sub(0x10) {
        let p = base.wrapping_add(i * 0x6e);
        for vaga in 0..16u32 {
            if serve(p, vaga) == 0 {
                continue;
            }
            let pv = vaga as usize;
            let conta = cpo8_i(this + 0x98 + pv * 4);
            if conta >= cpo8_i(this + 0xd8) {
                erro_em(0xa825a4, 0x121);
            }
            let o = (conta as usize).wrapping_mul(12);
            let (l1, l2) = (cpo8_d(this + 0x18 + pv * 4).wrapping_add(o), cpo8_d(this + 0x58 + pv * 4).wrapping_add(o));
            cpo8_w(l1, i as u32);
            cpo8_w(l2, i as u32);
            let mut nacao = cpo8_d(p + 0x1a);
            if nacao == 0 {
                let sortear: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
                nacao = cpo8_d(0xae23a8).wrapping_add((sortear(cpo8_i(0xae2350)) as usize).wrapping_mul(0x122));
                cpo8_w(p + 0x1a, nacao as u32);
            }
            cpo8_w(l1 + 4, cpo8_d(nacao) as u32);
            cpo8_w(l2 + 4, cpo8_d(cpo8_d(p + 0x1a)) as u32);
            cpo8_w(l1 + 8, cpo8_d(p + 0x52) as u32);
            let k = cpo8_contrato_com_o_clube(p);
            cpo8_w(l2 + 8, if k != 0 { cpo8_b(k + 0x3d) as i8 as i32 as u32 } else { 100 });
            cpo8_w(this + 0x98 + pv * 4, (conta + 1) as u32);
        }
        i += 1;
    }
}

/// { 0x83a780 } (thiscall, ret 0, shortlist_manager.cpp): `Shortlist_Manager::sort_position_lists()`: a cada dia,
/// refaz a chave (+8) das entradas das 16 vagas e ordena os dois vetores de cada uma (0x945333 com 0x83b670: pela chave
/// e pelo +4). Na primeira lista: o valor da pessoa (+0x52) se o contrato com o clube existe e a faixa do vencimento
/// (0x867d30) é 0, senão 0; o jogador de clube humano (0x600a50) na lista de transferências (0x5444a0) perde 1.500.000,
/// no mínimo um terço do valor. Na vaga 15 só a primeira, com o valor. Na segunda: −1000 com o bit 2 do +0x4e do
/// contrato, 2000 com os bits 0 ou 3 ou sem jogador (+0x61), sem clube, sem divisão (+0x57) ou divisão sem marca em
/// [0xadadfc] com reputação abaixo de 6000, ou sem contrato; senão o +0x3d do contrato.
unsafe extern "thiscall" fn ordenar_listas_de_posicao(this: usize) {
    let faixa: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x867d30usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, usize, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let listado: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x5444a0usize);
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    for vaga in 0..16usize {
        let mut j = 0i32;
        let mut o = 0usize;
        while j < cpo8_i(this + 0x98 + vaga * 4) {
            let p = cpo8_pessoa(cpo8_d(cpo8_d(this + 0x18 + vaga * 4) + o));
            let k = cpo8_contrato_com_o_clube(p);
            if vaga == 15 {
                cpo8_w(cpo8_d(this + 0x54) + o + 8, cpo8_d(p + 0x52) as u32);
                j += 1;
                o += 12;
                continue;
            }
            if k != 0 && faixa(k) == 0 {
                cpo8_w(cpo8_d(this + 0x18 + vaga * 4) + o + 8, cpo8_d(p + 0x52) as u32);
                let clube = cpo8_d(p + 0x39);
                if clube != 0 && humano(clube, 0, 1, 0, 0) != 0 && listado(p, 0) != 0 {
                    let e = cpo8_d(this + 0x18 + vaga * 4) + o + 8;
                    cpo8_w(e, cpo8_i(e).wrapping_add(-0x16e360) as u32);
                    let terco = cpo8_i(p + 0x52) / 3;
                    if cpo8_i(e) < terco {
                        cpo8_w(e, terco as u32);
                    }
                }
            } else {
                if k != 0 {
                    crate::comparacao::ramo("cpo_8 faixa do contrato");
                }
                cpo8_w(cpo8_d(this + 0x18 + vaga * 4) + o + 8, 0);
            }
            // a segunda lista
            let p2 = cpo8_pessoa(cpo8_d(cpo8_d(this + 0x58 + vaga * 4) + o));
            let k2 = cpo8_contrato_com_o_clube(p2);
            let clube = cpo8_d(p2 + 0x39);
            let chave: i32 = if cpo8_d(p2 + 0x61) == 0 || clube == 0 || cpo8_d(clube + 0x57) == 0 {
                0x7d0
            } else if cpo8_d(cpo8_d(0xadadfc) + cpo8_d(cpo8_d(clube + 0x57)).wrapping_mul(4)) == 0
                && core::ptr::read_unaligned((clube + 0x80) as *const i16) < 0x1770
            {
                0x7d0
            } else if k2 == 0 {
                0x7d0
            } else if cpo8_b(k2 + 0x4e) & 2 != 0 {
                -1000
            } else if cpo8_b(k2 + 0x4e) & 9 != 0 {
                0x7d0
            } else {
                cpo8_b(k2 + 0x3d) as i8 as i32
            };
            cpo8_w(cpo8_d(this + 0x58 + vaga * 4) + o + 8, chave as u32);
            j += 1;
            o += 12;
        }
        let n = cpo8_d(this + 0x98 + vaga * 4) as u32;
        ordenar(cpo8_d(this + 0x18 + vaga * 4), n, 12, 0x83b670);
        let n = cpo8_d(this + 0x98 + vaga * 4) as u32;
        ordenar(cpo8_d(this + 0x58 + vaga * 4), n, 12, 0x83b670);
    }
}

/// { 0x842180 } (thiscall, ret 0x14, shortlist_manager.cpp): `Shortlist_Manager::collect_interested_clubs(jogador,
/// saída, ignorar, modo, só_humanos)`: esvazia a lista `saída` (0x68d080) e acrescenta (0x68d0d0, 8 bytes: id do
/// jogador, id do clube) os clubes interessados: com modo ≠ 3, os 16 humanos (do último para o primeiro) com +0x5f e
/// clube que têm o jogador na lista do olheiro (0x7f7490; com modo 2, fora os clubes que o humano da vez controla,
/// 0x600c90); sem `só_humanos`, os pares (jogador, clube) do vetor pela pessoa (+0, conta +0x10; 0x946464 até o primeiro
/// do jogador) cujo clube não está suspenso (+8 da entrada de 9 bytes do clube em +0xdc abaixo de 100, salvo com
/// `ignorar`), não é o do jogador e põe nele (+7 da vaga de 0x25 bytes com o jogador em +0xf; sem vaga, a primeira)
/// mais que o valor (+0x52).
unsafe extern "thiscall" fn clubes_interessados_no_jogador(this: usize, jogador: usize, saida: usize, ignorar: u32, modo: u32, so_humanos: u32) {
    let acrescentar: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x68d0d0usize);
    if saida != 0 {
        let limpar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d080usize);
        limpar(saida);
    }
    if modo as u8 != 3 {
        let controla: unsafe extern "C" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x600c90usize);
        let na_lista: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7f7490usize);
        for k in 0..16i32 {
            let h = cpo8_pessoa(cpo8_i(0xae2364).wrapping_sub(k).wrapping_sub(1) as usize);
            let clube = cpo8_d(h + 0x39);
            if cpo8_b(h + 0x5f) == 0 || clube == 0 {
                continue;
            }
            if controla(clube, 0, 1) != 0 && modo as u8 == 2 {
                continue;
            }
            if na_lista(cpo8_d(0xb74448), jogador, h) != 0 {
                let par = [cpo8_d(jogador) as u32, cpo8_d(clube) as u32];
                acrescentar(saida, par.as_ptr() as usize, 8);
            }
        }
    }
    if so_humanos != 0 {
        return;
    }
    let bsearch: unsafe extern "C" fn(*const u32, usize, i32, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    let chave = [cpo8_d(jogador) as u32, 0xffff_ffff];
    let mut e = bsearch(chave.as_ptr(), cpo8_d(this), cpo8_i(this + 0x10), 8, 0x757ea0);
    if e == 0 {
        return;
    }
    let base = cpo8_d(this);
    let ultimo = base.wrapping_add((cpo8_i(this + 0x10) as usize).wrapping_mul(8)).wrapping_sub(8);
    let id = cpo8_d(jogador);
    if e > base {
        loop {
            let x = cpo8_d(e - 8);
            e -= 8;
            if x != id {
                e += 8;
                break;
            }
            if e <= base {
                break;
            }
        }
    }
    if cpo8_d(e) != id {
        return;
    }
    loop {
        let clube_id = cpo8_d(e + 4);
        let entrada = cpo8_d(this + 0xdc).wrapping_add(clube_id.wrapping_mul(9));
        let vagas = cpo8_d(entrada);
        let mut k = 0usize;
        while k < 0x28 && cpo8_d(vagas + 0xf + k * 0x25) != cpo8_d(jogador) {
            k += 1;
        }
        let valor = if k < 0x28 { cpo8_i(vagas + k * 0x25 + 7) } else { cpo8_i(vagas + 7) };
        if !(ignorar == 0 && cpo8_b(entrada + 8) >= 100) && valor > cpo8_i(jogador + 0x52) {
            let c = cpo8_d(jogador + 0x39);
            if !(c != 0 && cpo8_d(c) == clube_id) {
                crate::comparacao::ramo("cpo_8 clube interessado");
                acrescentar(saida, e, 8);
            }
        }
        if e == ultimo {
            return;
        }
        let prox = cpo8_d(e + 8);
        e += 8;
        if prox != cpo8_d(jogador) {
            return;
        }
    }
}

/// Os itens (8 bytes) de uma LLIST (+2 o primeiro nó; nó: +0 o dado, +6 o próximo).
unsafe fn cpo8_itens(lista: usize) -> Vec<(u32, u32)> {
    let mut v = Vec::new();
    let mut no = cpo8_d(lista + 2);
    while no != 0 {
        let x = cpo8_d(no);
        v.push((cpo8_d(x) as u32, cpo8_d(x + 4) as u32));
        no = cpo8_d(no + 6);
    }
    v
}

/// Comparador: só a original.
// ponytail: a original solta e aloca os 32 vetores (e pode sortear a nação de quem não tem); não roda duas vezes. Confere
// o resultado: as duas listas de cada vaga com os mesmos ids, cada entrada servindo para a vaga, a conta no limite.
unsafe extern "thiscall" fn comparar_montar_listas_de_posicao(this: usize, nao_usado: u32) {
    orig::<unsafe extern "thiscall" fn(usize, u32)>(988)(this, nao_usado);
    let serve: unsafe extern "stdcall" fn(usize, u32) -> u8 = core::mem::transmute(0x844300usize);
    let mut ruim = None;
    'f: for vaga in 0..16usize {
        let n = cpo8_i(this + 0x98 + vaga * 4);
        if n > cpo8_i(this + 0xd8) {
            ruim = Some((vaga, -1));
            break;
        }
        for j in 0..n.max(0) as usize {
            let (a, b) = (cpo8_d(this + 0x18 + vaga * 4) + j * 12, cpo8_d(this + 0x58 + vaga * 4) + j * 12);
            if cpo8_d(a) != cpo8_d(b) || serve(cpo8_pessoa(cpo8_d(a)), vaga as u32) == 0 {
                ruim = Some((vaga, j as i32));
                break 'f;
            }
        }
    }
    PLACAR.registrar(ruim.is_none(), || format!("montar_listas_de_posicao: vaga e entrada {:?}", ruim));
}

/// Comparador: lado a lado pelos 32 vetores (12 bytes por entrada) e por [0xae1080].
unsafe extern "thiscall" fn comparar_ordenar_listas_de_posicao(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(989);
    let mut reg = vec![(0xae1080usize, 4usize)];
    for vaga in 0..16usize {
        let n = cpo8_i(this + 0x98 + vaga * 4);
        if n > 0 {
            reg.push((cpo8_d(this + 0x18 + vaga * 4), n as usize * 12));
            reg.push((cpo8_d(this + 0x58 + vaga * 4), n as usize * 12));
        }
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this), || ordenar_listas_de_posicao(this));
    PLACAR.registrar(igual, || {
        let dif: Vec<usize> = mo.0.iter().zip(mr.0.iter()).filter(|(a, b)| a.1 != b.1).map(|(a, _)| a.0).take(8).collect();
        format!("ordenar_listas_de_posicao: vetores diferentes {:x?}", dif)
    });
}

/// Comparador: a original na lista de saída e a cópia numa lista nova (0x68d020, solta com 0x68d080); compara os pares.
unsafe extern "thiscall" fn comparar_clubes_interessados_no_jogador(this: usize, jogador: usize, saida: usize, ignorar: u32, modo: u32, so_humanos: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32)>(990);
    o(this, jogador, saida, ignorar, modo, so_humanos);
    if saida == 0 {
        return;
    }
    let criar: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let limpar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d080usize);
    let mut nova = [0u8; 0x10];
    let l = nova.as_mut_ptr() as usize;
    criar(l);
    clubes_interessados_no_jogador(this, jogador, l, ignorar, modo, so_humanos);
    let (a, b) = (cpo8_itens(saida), cpo8_itens(l));
    limpar(l);
    PLACAR.registrar(a == b, || format!("clubes_interessados_no_jogador({:#x}) original={:x?} rust={:x?}", jogador, a, b));
}

// ---------- 251a leva: a lista de observação e o empréstimo: acrescentar o par na lista, prever a resposta do clube vendedor e o período do empréstimo (shortlist_manager.cpp, transfer_manager.cpp) ----------

unsafe fn cpo9_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpo9_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpo9_h(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpo9_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpo9_w(a: usize, v: u32) {
    core::ptr::write_unaligned(a as *mut u32, v)
}

/// Acrescenta o par (id da pessoa, id do clube) no fim do vetor `[this + vet]` (conta em `this + conta`) e o ordena
/// (0x945333 com `cmp`).
unsafe fn cpo9_inserir(this: usize, vet: usize, conta: usize, pessoa: usize, clube: usize, cmp: usize) {
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let n = cpo9_i(this + conta);
    let e = cpo9_d(this + vet).wrapping_add((n as usize).wrapping_mul(8));
    cpo9_w(e, cpo9_d(pessoa) as u32);
    cpo9_w(e + 4, cpo9_d(clube) as u32);
    cpo9_w(this + conta, (n + 1) as u32);
    ordenar(cpo9_d(this + vet), (n + 1) as u32, 8, cmp);
}

/// { 0x8423f0 } (thiscall, ret 8, shortlist_manager.cpp): `Shortlist_Manager::add_shortlist_entry(pessoa, clube)`: cria
/// os dois vetores de pares na primeira vez (0x944e46 de 0x320 bytes, capacidade +8/+0xc = 100; sem memória: a caixa
/// "Error" e o jogo para); com o par (pessoa, clube; sem clube −1) já no vetor pela pessoa (0x946464 com 0x757ea0), nada.
/// Senão cresce de 100 em 100 o vetor cheio (0x945667), acrescenta o par nos dois e os ordena (pela pessoa com
/// 0x757ea0, pelo clube com 0x844740) e, fora de clube humano (0x600a50), soma 1 no nibble baixo do +0xb do registro de
/// 79 bytes da pessoa ([0xae2d0c]: quantas listas a têm).
unsafe extern "thiscall" fn acrescentar_na_lista_de_observacao(this: usize, pessoa: usize, clube: usize) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x945667usize);
    if cpo9_d(this) == 0 {
        let v = alocar(0x320);
        cpo9_w(this, v as u32);
        if v == 0 {
            erro_em(0xa825a4, 0x1594);
        }
        cpo9_w(this + 8, 100);
    }
    if cpo9_d(this + 4) == 0 {
        let v = alocar(0x320);
        cpo9_w(this + 4, v as u32);
        if v == 0 {
            erro_em(0xa825a4, 0x159d);
        }
        cpo9_w(this + 0xc, 100);
    }
    if cpo9_d(this) != 0 {
        let bsearch: unsafe extern "C" fn(*const u32, usize, i32, u32, usize) -> usize = core::mem::transmute(0x946464usize);
        let chave = [cpo9_d(pessoa) as u32, if clube != 0 { cpo9_d(clube) as u32 } else { 0xffff_ffff }];
        if bsearch(chave.as_ptr(), cpo9_d(this), cpo9_i(this + 0x10), 8, 0x757ea0) != 0 {
            return;
        }
    }
    crate::comparacao::ramo("cpo_9 nova entrada");
    if cpo9_i(this + 8) == cpo9_i(this + 0x10) {
        let v = realocar(cpo9_d(this), (cpo9_i(this + 8) as u32).wrapping_mul(8).wrapping_add(0x320));
        cpo9_w(this, v as u32);
        if v == 0 {
            erro_em(0xa825a4, 0x15b5);
        }
        cpo9_w(this + 8, cpo9_i(this + 8).wrapping_add(100) as u32);
    }
    if cpo9_i(this + 0xc) == cpo9_i(this + 0x14) {
        let v = realocar(cpo9_d(this + 4), (cpo9_i(this + 0xc) as u32).wrapping_mul(8).wrapping_add(0x320));
        cpo9_w(this + 4, v as u32);
        if v == 0 {
            erro_em(0xa825a4, 0x15be);
        }
        cpo9_w(this + 0xc, cpo9_i(this + 0xc).wrapping_add(100) as u32);
    }
    cpo9_inserir(this, 0, 0x10, pessoa, clube, 0x757ea0); // o vetor pela pessoa
    cpo9_inserir(this, 4, 0x14, pessoa, clube, 0x844740); // o vetor pelo clube
    let humano: unsafe extern "C" fn(usize, u32, u32, usize, u32) -> u32 = core::mem::transmute(0x600a50usize);
    if humano(clube, 0, 1, 0, 0) == 0 {
        let r = (cpo9_d(0xae2d0c).wrapping_add(cpo9_d(pessoa).wrapping_mul(0x4f)) + 0xb) as *mut u8;
        let a = *r;
        *r = (a & 0xf0) | (a.wrapping_add(1) & 0xf);
    }
}

/// { 0x84af00 } (stdcall, ret 0x20, shortlist_manager.cpp): `shortlist_manager_predict_selling_club_response(vendedor,
/// jogador, oferta, comprador, pedido, empréstimo)`: 0 aceita, 1 negocia, 2 recusa. Sem vendedor ou com o contrato com
/// ele (0x4cf5f0) marcado em +0x49: 0; o técnico do vendedor é o jogador: 2; sem contrato de tipo 2 (+0x35): 2 no
/// empréstimo, 0 senão. Empréstimo: clube humano (0x600a50): 0; listado para empréstimo (0x544590) com a reputação do
/// comprador × [0x968928] acima do +0xb da ficha: 0; estatuto 6/7 (+0x4f) ou +0xf da ficha 20, com a reputação acima de
/// +0xb × [0x969040], +0x3d do contrato abaixo de −50 e o +0x1db do vendedor diferente de [0x9cf6e8]: 0; senão 2.
/// Compra: o preço (humano: 0x4d42d0 × [0x9689b0], sem cláusula 0x867b50 e com o técnico de férias 0x5fd210 a opção 0
/// 0x5fea50: 1 recusa, 2 o valor +0x52; senão o `pedido` não negativo, o valor sem ficha ou 0x4d1600); oferta no preço:
/// 0; humano com a oferta acima do preço − 1.500.000 e listado (0x5444a0): 1; preço × [0x96f9c0] acima da oferta: 2; 1.
unsafe extern "stdcall" fn prever_resposta_do_vendedor(vendedor: usize, jogador: usize, oferta: i64, comprador: usize, pedido: i64, emprestimo: u32) -> u8 {
    let com_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, usize, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let k = com_clube(cpo9_d(0xae1050), jogador, vendedor);
    if vendedor == 0 || (k != 0 && cpo9_b(k + 0x49) != 0) {
        return 0;
    }
    if cpo9_d(vendedor + 0xcf) == jogador {
        return 2;
    }
    if k == 0 || cpo9_b(k + 0x35) & 0x3f != 2 {
        return if emprestimo as u8 != 0 { 2 } else { 0 };
    }
    if emprestimo as u8 != 0 {
        if humano(vendedor, 0, 1, 0, 0) != 0 {
            return 0;
        }
        let listado: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x544590usize);
        let rep = cpo9_h(comprador + 0x80) as f64;
        if listado(jogador, 0) != 0 && rep * f64_em(0x968928) > cpo9_h(cpo9_d(jogador + 0x61) + 0xb) as f64 {
            return 0;
        }
        let st = cpo9_b(k + 0x4f) & 0xf0;
        if st == 0x60 || st == 0x70 || cpo9_b(cpo9_d(jogador + 0x61) + 0xf) == 0x14 {
            let v = cpo9_h(cpo9_d(jogador + 0x61) + 0xb) as f64 * f64_em(0x969040);
            if rep > v && (cpo9_b(k + 0x3d) as i8) < -50 && cpo9_d(vendedor + 0x1db) != cpo9_d(0x9cf6e8) {
                return 0;
            }
        }
        return 2;
    }
    let preco: i32 = if humano(vendedor, 0, 1, 0, 0) != 0 {
        let pedido_humano: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> i32 = core::mem::transmute(0x4d42d0usize);
        let mut v = ftol(pedido_humano(cpo9_d(0xae1050), vendedor, comprador, jogador) as f64 * f64_em(0x9689b0));
        let clausula: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x867b50usize);
        if clausula(k) == 0 {
            let tecnico: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53ca90usize);
            let ferias: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5fd210usize);
            if ferias(cpo9_d(0xb63c98), tecnico(vendedor)) != 0 {
                let opcao: unsafe extern "thiscall" fn(usize, u32, usize) -> u8 = core::mem::transmute(0x5fea50usize);
                match opcao(cpo9_d(0xb63c98), 0, tecnico(vendedor)) as i8 {
                    1 => return 2,
                    2 => v = cpo9_i(jogador + 0x52),
                    _ => {}
                }
            }
        }
        v
    } else if pedido >= 0 {
        pedido as i32
    } else if cpo9_d(jogador + 0x61) == 0 {
        cpo9_i(jogador + 0x52)
    } else {
        let calcular: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, u32) -> i32 = core::mem::transmute(0x4d1600usize);
        calcular(cpo9_d(0xae1050), vendedor, comprador, jogador, 0, 1, 1)
    };
    if oferta >= preco as i64 {
        return 0;
    }
    let listado: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x5444a0usize);
    if humano(vendedor, 0, 1, 0, 0) != 0 && (preco.wrapping_sub(0x16e360) as i64) < oferta && listado(jogador, 0) != 0 {
        return 1;
    }
    if preco as f64 * f64_em(0x96f9c0) > oferta as f64 {
        return 2;
    }
    crate::comparacao::ramo("cpo_9 negocia");
    1
}

/// O ano da data padrão (0x54a110: dia 40 de 1972).
unsafe fn cpo9_ano_padrao() -> u16 {
    let padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    let mut data = [0u8; 8];
    let p = padrao(data.as_mut_ptr() as usize);
    cpo9_h(p + 2) as u16
}

/// A data (dia do ano +0, ano +2) passa do fim do contrato `c` (+0x2d dia, +0x2f ano)?
unsafe fn cpo9_depois_do_contrato(dia: i16, ano: u16, c: usize) -> bool {
    let fim = cpo9_h(c + 0x2f) as u16;
    ano > fim || (ano == fim && dia > cpo9_h(c + 0x2d))
}

/// { 0x8d0590 } (thiscall, ret 0x1c, transfer_manager.cpp): `TRANSFER_MANAGER::get_loan_period(clube, a2, jogador,
/// início_mês, início_ano, fim_mês, fim_ano)`: as regras da nação do clube (+0x53; das 28 em +0x8ac, a primeira sem
/// achar) dão o período (método +0x28). Sem contrato de tipo 2 (0x4cf5a0), com o contrato já vencido hoje ou o fim mais
/// de 5 anos à frente: o fim vira o ano da data padrão (1972). Senão, enquanto a data (dia do mês de hoje, fim_mês,
/// fim_ano; 0x549f70) passar do fim do contrato, recua um mês (o início acompanha se ficar depois do fim); fim antes do
/// ano de hoje: o ano padrão.
unsafe extern "thiscall" fn periodo_do_emprestimo(this: usize, clube: usize, a2: u32, jogador: usize, ini_mes: *mut i8, ini_ano: *mut u16, mes: *mut i8, ano: *mut u16) {
    let nacao = cpo9_d(clube + 0x53);
    let mut regra = cpo9_d(this + 0x8ac);
    for k in 0..0x1cusize {
        let r = cpo9_d(this + 0x8ac + k * 4);
        if cpo9_d(r + 0xa) == nacao {
            if r != 0 {
                regra = r;
            }
            break;
        }
    }
    let periodo: unsafe extern "thiscall" fn(usize, usize, u32, *mut i8, *mut u16, *mut i8, *mut u16, usize) = core::mem::transmute(cpo9_d(cpo9_d(regra) + 0x28));
    periodo(regra, clube, a2, ini_mes, ini_ano, mes, ano, jogador);
    let dia_do_mes: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x54cf70usize);
    let dia = dia_do_mes(0xae2c90);
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contrato_de(cpo9_d(0xae1050), jogador);
    if c == 0 || cpo9_b(c + 0x35) & 0x3f != 2 || cpo9_depois_do_contrato(cpo9_h(0xae2c90), cpo9_h(0xae2c92) as u16, c) {
        *ano = cpo9_ano_padrao();
        return;
    }
    if *ano as i32 > cpo9_h(0xae2c92) as u16 as i32 + 5 {
        *ano = cpo9_ano_padrao();
        return;
    }
    // ponytail: a original empurra o mês e o ano em registradores com lixo acima; a 0x549f70 lê o byte e a palavra
    let data_de: unsafe extern "thiscall" fn(usize, i32, i32, u32, i32) -> usize = core::mem::transmute(0x549f70usize);
    loop {
        let mut data = [0u8; 8];
        let d = data_de(data.as_mut_ptr() as usize, dia, *mes as i32, *ano as u32, -1);
        if !cpo9_depois_do_contrato(cpo9_h(d), cpo9_h(d + 2) as u16, c) {
            return;
        }
        crate::comparacao::ramo("cpo_9 recua o mes");
        let m = (*mes).wrapping_sub(1);
        *mes = m;
        if m < 0 {
            *ano = (*ano).wrapping_sub(1);
            *mes = 11;
        }
        if *ini_ano > *ano || (*ini_ano == *ano && *ini_mes > *mes) {
            *ini_mes = *mes;
            *ini_ano = *ano;
        }
        if *ano < cpo9_h(0xae2c92) as u16 {
            *ano = cpo9_ano_padrao();
            return;
        }
    }
}

/// Comparador: só a original, conferindo o plano (feito antes): o par já estava (nada muda) ou entra nos dois vetores
/// (contas + 1, o par achado depois) e o nibble da pessoa soma 1 fora de clube humano.
// ponytail: a original realoca os vetores (0x945667) e os ordena; não roda duas vezes.
unsafe extern "thiscall" fn comparar_acrescentar_na_lista_de_observacao(this: usize, pessoa: usize, clube: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(991);
    let bsearch: unsafe extern "C" fn(*const u32, usize, i32, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, usize, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let chave = [cpo9_d(pessoa) as u32, if clube != 0 { cpo9_d(clube) as u32 } else { 0xffff_ffff }];
    let tinha = cpo9_d(this) != 0 && bsearch(chave.as_ptr(), cpo9_d(this), cpo9_i(this + 0x10), 8, 0x757ea0) != 0;
    let (n0, n1) = if cpo9_d(this) != 0 { (cpo9_i(this + 0x10), cpo9_i(this + 0x14)) } else { (0, 0) };
    let r = cpo9_d(0xae2d0c).wrapping_add(cpo9_d(pessoa).wrapping_mul(0x4f)) + 0xb;
    let nib = cpo9_b(r);
    let soma = !tinha && humano(clube, 0, 1, 0, 0) == 0;
    o(this, pessoa, clube);
    let esperado = if soma { (nib & 0xf0) | (nib.wrapping_add(1) & 0xf) } else { nib };
    let (d0, d1) = (if tinha { n0 } else { n0 + 1 }, if tinha { n1 } else { n1 + 1 });
    let achado = bsearch(chave.as_ptr(), cpo9_d(this), cpo9_i(this + 0x10), 8, 0x757ea0) != 0;
    let ok = cpo9_i(this + 0x10) == d0 && cpo9_i(this + 0x14) == d1 && cpo9_b(r) == esperado && (achado || clube == 0);
    PLACAR.registrar(ok, || {
        format!("acrescentar_na_lista_de_observacao({:#x}, {:#x}) tinha={} contas {}/{} -> {}/{} nibble {:#x} -> {:#x} (esperado {:#x})",
            pessoa, clube, tinha, n0, n1, cpo9_i(this + 0x10), cpo9_i(this + 0x14), nib, cpo9_b(r), esperado)
    });
}

/// Comparador: pelo valor (o byte baixo) e lado a lado pelo sorteio e [0xae1080] (as chamadas de preço podem sortear).
unsafe extern "stdcall" fn comparar_prever_resposta_do_vendedor(vendedor: usize, jogador: usize, oferta: i64, comprador: usize, pedido: i64, emprestimo: u32) -> u8 {
    let o = orig::<unsafe extern "stdcall" fn(usize, usize, i64, usize, i64, u32) -> u8>(992);
    let mut reg = vec![(0xae1080usize, 4usize)];
    reg.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(
        &reg,
        || o(vendedor, jogador, oferta, comprador, pedido, emprestimo),
        || crate::aleatorio::sem_contar(|| prever_resposta_do_vendedor(vendedor, jogador, oferta, comprador, pedido, emprestimo)),
    );
    PLACAR.registrar(igual && r == n, || format!("prever_resposta_do_vendedor({:#x}, {:#x}, {}, {:#x}, {}, {}) original={} rust={} {:x?} x {:x?}", vendedor, jogador, oferta, comprador, pedido, emprestimo as u8, r, n, mo, mr));
    r
}

/// Comparador: lado a lado pelas quatro saídas, o sorteio e [0xae1080].
unsafe extern "thiscall" fn comparar_periodo_do_emprestimo(this: usize, clube: usize, a2: u32, jogador: usize, ini_mes: *mut i8, ini_ano: *mut u16, mes: *mut i8, ano: *mut u16) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, usize, *mut i8, *mut u16, *mut i8, *mut u16)>(993);
    let mut reg = vec![(ini_mes as usize, 1usize), (ini_ano as usize, 2), (mes as usize, 1), (ano as usize, 2), (0xae1080, 4)];
    reg.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(
        &reg,
        || o(this, clube, a2, jogador, ini_mes, ini_ano, mes, ano),
        || crate::aleatorio::sem_contar(|| periodo_do_emprestimo(this, clube, a2, jogador, ini_mes, ini_ano, mes, ano)),
    );
    PLACAR.registrar(igual, || format!("periodo_do_emprestimo({:#x}, {:#x}) {:x?} x {:x?}", clube, jogador, mo, mr));
}

// ---------- 250a leva: a lista de observação do clube: o estatuto no elenco que o clube dá à pessoa, a estimativa do clube humano e soltar os alvos que não são do clube (shortlist_manager.cpp) ----------

unsafe fn cpn6_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpn6_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O clube do contrato `c` (id em +4; o registro de clube tem 0x245 bytes em [0xae23b4]), ou 0 sem contrato.
unsafe fn cpn6_clube_do_contrato(c: usize) -> usize {
    if c == 0 {
        return 0;
    }
    cpn6_d(0xae23b4).wrapping_add(cpn6_d(c + 4).wrapping_mul(0x245))
}

/// { 0x849f00 } (thiscall, ret 0xc, shortlist_manager.cpp): `Shortlist_Manager::get_squad_status_for_club(pessoa,
/// clube, cargo)`: o estatuto no elenco que o `clube` dá à `pessoa`. Clube nulo: 7. A entrada da pessoa na lista do
/// clube (0x849ce0: a de tipo ≠ 11 para jogador, +0x61; tendo +0x69, a de tipo 11 também): o cargo (+0 da entrada) e
/// os bits 1..4 do +0x24. Senão o cargo é o +0x3d da pessoa e vale o contrato (0x4cf5a0, guardado em [0xae1080]): do
/// próprio clube, o nibble alto do +0x4f do contrato com o clube (0x4cf5f0; sem ele, 7); clube humano (0x600a50) com
/// cargo 11 a 15 (0x53b620): a estimativa 0x849d60; senão, o clube da pessoa (+0x39) sendo o clube ou o parceiro dele
/// (0x540a50) leva ao contrato com o clube; nenhum: 7.
unsafe extern "thiscall" fn estatuto_no_elenco_pelo_clube(this: usize, pessoa: usize, clube: usize, cargo: *mut u8) -> u8 {
    if clube == 0 {
        return 7;
    }
    let entrada: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(0x849ce0usize);
    let mut e = entrada(this, clube, pessoa, (cpn6_d(pessoa + 0x61) == 0) as u32, 0);
    if e == 0 && cpn6_d(pessoa + 0x69) != 0 {
        e = entrada(this, clube, pessoa, 1, 0);
    }
    if e != 0 {
        if !cargo.is_null() {
            *cargo = cpn6_b(e);
        }
        crate::comparacao::ramo("cpn_6 estatuto pela lista");
        return (cpn6_b(e + 0x24) >> 1) & 0xf; // os bits 1..4 da entrada
    }
    if !cargo.is_null() {
        *cargo = cpn6_b(pessoa + 0x3d);
    }
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contrato_de(cpn6_d(0xae1050), pessoa);
    *(0xae1080 as *mut usize) = c;
    let pelo_contrato = if cpn6_clube_do_contrato(c) == clube {
        true
    } else {
        // ponytail: o 3º argumento é empurrado como dword 1; o cargo vai num byte (a 0x53b620 só lê o byte)
        let humano: unsafe extern "C" fn(usize, u32, u32, usize, u32) -> u32 = core::mem::transmute(0x600a50usize);
        let tipo_11_a_15: unsafe extern "C" fn(i8) -> u32 = core::mem::transmute(0x53b620usize);
        if humano(clube, 1, 1, 0, 0) != 0 && tipo_11_a_15(cpn6_b(pessoa + 0x3d) as i8) != 0 {
            let estimar: unsafe extern "stdcall" fn(usize, usize) -> u32 = core::mem::transmute(0x849d60usize);
            return estimar(pessoa, clube) as u8;
        }
        cpn6_d(pessoa + 0x39) == clube || {
            let parceiro: unsafe extern "C" fn(usize, usize, u32) -> usize = core::mem::transmute(0x540a50usize);
            cpn6_d(pessoa + 0x39) == parceiro(clube, 0, 1)
        }
    };
    if !pelo_contrato {
        return 7;
    }
    let com_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let k = com_clube(cpn6_d(0xae1050), pessoa, clube);
    if k == 0 {
        return 7;
    }
    crate::comparacao::ramo("cpn_6 estatuto pelo contrato");
    cpn6_b(k + 0x4f) >> 4 // o nibble alto do contrato
}

/// { 0x849d60 } (stdcall, ret 8, shortlist_manager.cpp): `shortlist_manager_estimate_squad_status(pessoa, clube)`: o
/// estatuto que o clube humano daria. A reputação da pessoa vista pelo técnico do clube (0x53ca90, 0x5407e0; até 1:
/// 1, senão as duas chamadas de novo) e a do clube (+0x80, no mínimo 1); pessoa na lista do humano (0x7f7490 do
/// [0xb74448]): a reputação passa a ser o maior de 1,5 × ela e 0,75 × a do clube (truncado, 16 bits). A razão (limitada
/// a 0,1..2): acima de 1,5: 1; o float dela acima de 1,2 com o clube abaixo de 8500: 2; até 1,2 com a reputação até
/// 6500: 5 (acima de 0,6 e idade +0x18 abaixo de 23), 4 (24 ou mais) ou 6; senão 3. Só roda para o clube humano.
unsafe extern "stdcall" fn estimar_estatuto_no_elenco(pessoa: usize, clube: usize) -> u32 {
    let tecnico: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53ca90usize);
    let reputacao: unsafe extern "C" fn(usize, usize, u32) -> u16 = core::mem::transmute(0x5407e0usize);
    let mut rep = if reputacao(pessoa, tecnico(clube), 0) as i16 > 1 { reputacao(pessoa, tecnico(clube), 0) as i16 } else { 1 };
    let mut rc = core::ptr::read_unaligned((clube + 0x80) as *const i16);
    if rc <= 1 {
        rc = 1;
    }
    let t = tecnico(clube);
    let na_lista: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7f7490usize);
    if na_lista(cpn6_d(0xb74448), pessoa, t) != 0 {
        let a = rep as f64 * f64_em(0x968940);
        let b = rc as f64 * f64_em(0x969040);
        rep = ftol(if a > b { a } else { b }) as i16;
    }
    let mut q = rep as f64 / rc as f64;
    if q < f64_em(0x967880) {
        q = f64_em(0x967880);
    } else if !(q <= f64_em(0x968950)) {
        q = f64_em(0x968950);
    }
    let f = q as f32 as f64; // a original guarda a razão num float e compara o resto por ele
    if q > f64_em(0x968940) {
        return 1;
    }
    if f > f64_em(0x968978) && rc < 0x2134 {
        return 2;
    }
    let idade = cpn6_b(pessoa + 0x18) as i8;
    if !(f > f64_em(0x968978)) && rep <= 0x1964 {
        if f > f64_em(0x968e48) && idade < 0x17 {
            return 5;
        }
        return if idade >= 0x18 { 4 } else { 6 };
    }
    3
}

/// { 0x844660 } (thiscall, ret 8, shortlist_manager.cpp): `shortlist_manager_release_club_slot_targets(clube,
/// entrada)`: nas 40 vagas de 0x25 bytes de [entrada], o alvo (+0xf) acima do último id de pessoa ([0xae2364] − 1)
/// vira −1; o alvo válido cujo contrato (0x4cf5a0, guardado em [0xae1080]; o de tipo 5 em +0x35 não conta) não é do
/// clube, nem o clube dele (+0x39) é o clube, e com o +0x17 da vaga em −1: o evento 3 da lista (0x843e10) com o tipo
/// "não é 11" do +0 da vaga (no byte baixo do endereço da vaga, como a original).
unsafe extern "thiscall" fn liberar_alvos_da_lista_do_clube(this: usize, clube: usize, entrada: usize) {
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let atualizar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(0x843e10usize);
    for k in 0..40usize {
        let o = k * 0x25;
        let v = cpn6_d(entrada).wrapping_add(o + 0xf);
        if cpn6_d(v) as i32 > (*(0xae2364 as *const i32)).wrapping_sub(1) {
            core::ptr::write_unaligned(v as *mut i32, -1);
        }
        let id = cpn6_d(cpn6_d(entrada).wrapping_add(o + 0xf)) as i32;
        if id <= -1 {
            continue;
        }
        let p = cpn6_d(0xae23bc).wrapping_add((id as usize).wrapping_mul(0x6e));
        let c = contrato_de(cpn6_d(0xae1050), p);
        *(0xae1080 as *mut usize) = c;
        let do_contrato = if c != 0 && cpn6_b(c + 0x35) & 0x3f != 5 { cpn6_clube_do_contrato(c) } else { 0 };
        if do_contrato == clube || cpn6_d(p + 0x39) == clube {
            continue;
        }
        let s = cpn6_d(entrada).wrapping_add(o);
        if cpn6_d(s + 0x17) as i32 != -1 {
            continue;
        }
        // setne al sobre o endereço da vaga: os 24 bits altos são os dele
        let tipo = ((s & !0xff) | (cpn6_b(s) != 0xb) as usize) as u32;
        crate::comparacao::ramo("cpn_6 alvo fora do clube");
        atualizar(this, p, clube, 3, tipo);
    }
}

/// Comparador: lado a lado pelo estatuto, pelo cargo de saída e por [0xae1080].
unsafe extern "thiscall" fn comparar_estatuto_no_elenco_pelo_clube(this: usize, pessoa: usize, clube: usize, cargo: *mut u8) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, *mut u8) -> u8>(961);
    let saida = if cargo.is_null() { 0xae1080 } else { cargo as usize };
    let reg = [(0xae1080usize, 4usize), (saida, 1)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, pessoa, clube, cargo), || estatuto_no_elenco_pelo_clube(this, pessoa, clube, cargo));
    PLACAR.registrar(igual && r == n, || format!("estatuto_no_elenco_pelo_clube({:#x}, {:#x}) original={} rust={} {:x?} x {:x?}", pessoa, clube, r, n, mo, mr));
    r
}

/// Comparador: pelo valor (o byte baixo; a original deixa a palavra de estado do x87 em ah).
unsafe extern "stdcall" fn comparar_estimar_estatuto_no_elenco(pessoa: usize, clube: usize) -> u32 {
    let r = orig::<unsafe extern "stdcall" fn(usize, usize) -> u32>(962)(pessoa, clube);
    let n = estimar_estatuto_no_elenco(pessoa, clube);
    PLACAR.registrar(r as u8 == n as u8, || format!("estimar_estatuto_no_elenco({:#x}, {:#x}) original={:#x} rust={}", pessoa, clube, r, n));
    r
}

/// Comparador: só a original.
// ponytail: a 0x843e10 muda a lista do clube (e outras) a cada evento e não pode rodar duas vezes; a cópia tem o ramo
// e a prova é pela carreira (modo Rust contra a original).
unsafe extern "thiscall" fn comparar_liberar_alvos_da_lista_do_clube(this: usize, clube: usize, entrada: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(963)(this, clube, entrada)
}

