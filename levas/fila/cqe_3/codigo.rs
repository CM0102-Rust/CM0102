// ---------- %%LEVA%%a leva: os lances e os pedidos de empréstimo da IA pelas 40 vagas da lista de observação (shortlist_manager.cpp) ----------

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
unsafe fn contract_of_staff_cached(pessoa: usize) -> usize {
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contrato(cqe3_d(0xae1050), pessoa);
    cqe3_pd(0xae1080, c);
    c
}

/// O clube do contrato da pessoa (o tipo 5 conta; sem contrato, 0), com o contrato em [0xae1080].
unsafe fn club_of_staff_contract(pessoa: usize) -> usize {
    let c = contract_of_staff_cached(pessoa);
    if c == 0 { 0 } else { cqe3_clube(cqe3_d(c + 4)) }
}

/// A pessoa tem a pessoa no registro de 79 bytes em [0xae2d0c]: o byte +0xb.
unsafe fn staff_state_byte_0b(pessoa: usize) -> u8 {
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
unsafe extern "thiscall" fn make_club_bids(this: usize, clube: usize) {
    let mais: unsafe extern "thiscall" fn(usize, *mut [u32; 2], u32) -> usize = core::mem::transmute(0x54c5b0usize);
    let novo_contrato: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x85e410usize);
    let is_national_team: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
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
    let contract_offer: unsafe extern "thiscall" fn(usize, usize, i32, u32, u32, u32, u32, u32) = core::mem::transmute(0x4cc490usize);
    let tem_oferta_de_contrato: unsafe extern "thiscall" fn(usize, i32, i32) -> usize = core::mem::transmute(0x4cc9e0usize);
    let contrato_com: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let entrada: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(0x849ce0usize);
    let renovar: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32, u32) = core::mem::transmute(0x4cda70usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
    let situacao: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u8 = core::mem::transmute(0x843e10usize);
    let transfer_manager_can_loan_player: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32 = core::mem::transmute(0x8d0040usize);
    let pedir_emprestimo: unsafe extern "thiscall" fn(usize, usize, usize) -> i32 = core::mem::transmute(0x842740usize);
    let mut semana = [0u32; 2];
    mais(0xae2c90, &mut semana, 7);
    let pdata = semana.as_mut_ptr() as usize;
    let mut contrato0 = [0u8; 0x50];
    novo_contrato(contrato0.as_mut_ptr() as usize);
    if is_national_team(clube) != 0 || humano(clube, 0, 1, 0, 0) != 0 {
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
        if club_of_staff_contract(p) == clube {
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
        if staff_state_byte_0b(p) & 0x30 != 0 && jogador {
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
                        crate::comparison::branch("cqe_3 dobro pela liga");
                        v = v.wrapping_mul(2); // cqe3: o dobro pela liga ativa
                    }
                }
            }
            if cqe3_d(vaga(0x13)) as i32 > -1 && cqe3_b(vaga(0)) == 0xb {
                let atual = club_of_staff_contract(p);
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
                    v = v.wrapping_add(match_pl_ftol((read_f64_at(0x968940) - x * read_f64_at(0x969020)) * v as f64));
                }
                if sob_oferta(gm, sub, 0) != 0 {
                    let x = cqe3_d(vaga(0xb)) as i32 as f64 * read_f64_at(0x968940);
                    v = v.wrapping_add(match_pl_ftol(if x > read_f64_at(0x968910) { x } else { read_f64_at(0x968910) }));
                }
            }
            let c = contract_of_staff_cached(p);
            if c != 0 && cqe3_clube(cqe3_d(c + 4)) != 0 && jogador {
                let vendedor = club_of_staff_contract(p);
                if humano(vendedor, 0, 1, 0, 0) != 0 || cqe3_d(p + 0x39) == clube {
                    let mut contrato = [0u8; 0x50];
                    let pc = contrato.as_mut_ptr() as usize;
                    novo_contrato(pc);
                    let e = vaga(0);
                    // ponytail: o estatuto vai com os bytes altos do ponteiro da vaga (como na original); o cargo, com os do
                    // edx que a 0x600a50 deixou (lixo): a 0x85fc50 recebe um char, só o byte conta
                    let st = (e as u32 & 0xffff_ff00) | ((cqe3_b(e + 0x24) >> 1) & 0xf) as u32;
                    let r = desejado(pc, p, clube, u32::MAX, st, cqe3_b(e) as u32, 0, 0);
                    if r as f64 == read_f64_at(0x9689a0) {
                        continue;
                    }
                    // ponytail: o cargo vai com os bytes altos do resultado e do fnstsw; a 0x5968d0 lê só o byte
                    let t = teto(financas(cqe3_d(0xae2a38), clube), p, ((cqe3_b(vaga(0x24)) >> 1) & 0xf) as u32);
                    if (t as f64 * read_f64_at(0x968de8)) < cqe3_d(pc + 0xc) as i32 as f64 {
                        continue;
                    }
                }
            }
            crate::comparison::branch("cqe_3 sorteio do lance");
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
                    contract_offer(cqe3_d(0xae1050), termos, cqe3_d(po + 0x20) as i32, cqe3_d(po + 0x24) as u32, cargo, 0, 1, 0);
                    let c = contract_of_staff_cached(p);
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
                                    let f = (staff_state_byte_0b(p) & 0xf) as f64 * read_f64_at(0x967880) + rep as i32 as f64 / div as f64;
                                    Some(if read_f64_at(0x967890) <= f { if read_f64_at(0x968dc0) < f { 1.4 } else { f } } else { 1.0 })
                                }
                                3 | 4 | 5 if entrada(this, vendedor, p, 0, 0) != 0 => {
                                    let rep = cqe3_w(clube + 0x80) as i32;
                                    let div = if rep.wrapping_mul(2) >= 1 { rep.wrapping_mul(2) } else { 1 };
                                    let f = (staff_state_byte_0b(p) & 0xf) as f64 * read_f64_at(0x967880) + rep as f64 / div as f64 + read_f64_at(0x968968);
                                    Some(if read_f64_at(0x967890) <= f { if read_f64_at(0x968978) < f { 1.2 } else { f } } else { 1.0 })
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
            if transfer_manager_can_loan_player(gm, clube, p, 0) == 0 {
                continue;
            }
            let c = contract_of_staff_cached(p);
            if c != 0 && cqe3_clube(cqe3_d(c + 4)) != 0 && humano(club_of_staff_contract(p), 0, 1, 0, 0) != 0 {
                let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
                if cqe3_b(contrato(cqe3_d(0xae1050), p) + 0x4e) & 2 == 0 {
                    continue;
                }
            }
            if tem_oferta(gm, p, clube) != 0 || trocas(gm, p, 0) != 0 {
                continue;
            }
            crate::comparison::branch("cqe_3 sorteio do empréstimo");
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
unsafe extern "thiscall" fn comparar_make_club_bids(this: usize, clube: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%)(this, clube) // ponytail: só a original
}
