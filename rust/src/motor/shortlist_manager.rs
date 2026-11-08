use super::*;

// ---------- septuagesima sexta leva: lista de observacao, bandeiras do contrato ----------

/// { 0x849ce0 } (ret 0x10, shortlist_manager.cpp): a entrada da lista da pessoa (40 entradas de
/// 37 bytes no ponteiro em [this+0xdc] + id x 9) cujo +0xf (ou, com `tambem_13`, o +0x13) e o id
/// de `alvo`; com `nao_11` so serve a de tipo (+0) diferente de 11, sem ele so a de tipo 11. Nenhuma: 0.
pub(crate) unsafe extern "thiscall" fn shortlist_entry(this: usize, pessoa: usize, alvo: usize, nao_11: u32, tambem_13: u32) -> usize {
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


// ---------- 268a leva: os lances e os pedidos de empréstimo da IA pelas 40 vagas da lista de observação (shortlist_manager.cpp) ----------








/// O contrato da pessoa (0x4cf5a0: push pessoa; mov ecx, [0xae1050] -> thiscall, ret 4), guardado em [0xae1080].
pub(crate) unsafe fn cqe3_contrato(pessoa: usize) -> usize {
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = contrato(read_u32_as_usize(G_CONTRACT_MANAGER), pessoa);
    write_u32_from_usize(G_CONTRACT_LOOKUP_RESULT, c);
    c
}

/// O clube do contrato da pessoa (o tipo 5 conta; sem contrato, 0), com o contrato em [0xae1080].
pub(crate) unsafe fn cqe3_clube_do_contrato(pessoa: usize) -> usize {
    let c = cqe3_contrato(pessoa);
    if c == 0 { 0 } else { cqc7_clube(read_u32_as_usize(c + 4)) }
}

/// A pessoa tem a pessoa no registro de 79 bytes em [0xae2d0c]: o byte +0xb.
pub(crate) unsafe fn cqe3_registro_0b(pessoa: usize) -> u8 {
    read_u8(read_u32_as_usize(G_STAFF_STATE_TABLE).wrapping_add(read_u32_as_usize(pessoa).wrapping_mul(0x4f)) + 0xb)
}

/// computer club with an active shortlist: in each slot with a target and no offer, the role, contract, reach and offer
/// rules; in a purchase slot, the slot value (doubled by the 0x20 mark and the league, the injured player to replace,
/// the record and the offer for him) and a bid if random(value) beats random(2 × reputation + 1000), with the contract
/// offer and the renewal at the seller; in a loan slot, whether the loan is possible and the request (0x842740) if
/// random(+0xb) beats random(reputation / 10) or random(2 × reputation)
/// Original: `Shortlist_Manager::make_club_bids` (0x84a020, shortlist_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x84a020 } (thiscall, ret 4, shortlist_manager.cpp, quadro SEH: STAFF_CONTRACT e oferta locais, só a oferta com
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
pub(crate) unsafe extern "thiscall" fn make_club_bids(this: usize, clube: usize) {
    let mais: unsafe extern "thiscall" fn(usize, *mut [u32; 2], u32) -> usize = core::mem::transmute(FN_DATE_ADD_DAYS);
    let novo_contrato: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_STAFF_CONTRACT_STAFF_CONTRACT);
    let is_national_team: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let tecnico: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_CLUB_MANAGER);
    let responde: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_ON_HOLIDAY);
    let regra: unsafe extern "thiscall" fn(usize, u32, usize) -> u8 = core::mem::transmute(FN_HUMAN_MANAGER_GET_HOLIDAY_OPTION);
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FINANCETABLE_GET_CLUB_FINANCE);
    let vagas_com_lance: unsafe extern "thiscall" fn(usize, usize, u32) -> u8 = core::mem::transmute(FN_SHORTLIST_MANAGER_COUNT_SLOTS_WITH_BIDS);
    let contar_elenco: unsafe extern "C" fn(usize, u32, u32) -> u8 = core::mem::transmute(FN_DATABASE_COUNT_CLUB_SQUAD);
    let futura: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_FUTURE_TRANSFER);
    let acertado: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_AGREED_CONTRACT_CLUB);
    let chegar: unsafe extern "thiscall" fn(usize, usize, usize, *mut u16, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_CAN_CLUB_BID_FOR_PLAYER);
    let tem_oferta: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_CLUB_HAS_OFFER_FOR_PLAYER);
    let trocas: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_EXCHANGE_OFFERS_FOR_PLAYER);
    let pode_contratar: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, u32, *mut u16) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_CAN_SIGN_PLAYER);
    let lesao: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_INJURY_GET_PLAYER_INJURY);
    let sob_oferta: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_IS_PLAYER_UNDER_OFFER);
    let desejado: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32, u32) -> i32 = core::mem::transmute(FN_STAFF_CONTRACT_SET_TO_DESIRED_CONTRACT);
    let teto: unsafe extern "thiscall" fn(usize, usize, u32) -> i32 = core::mem::transmute(FN_FINANCE_GET_MAX_WAGE);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let nova_oferta: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_TRANSFER_OFFER_TRANSFER_OFFER);
    let destruir_oferta: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_TRANSFER_OFFER_DESTRUCTOR_TRANSFER_OFFER);
    let preencher: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, *mut usize) -> u32 = core::mem::transmute(FN_TRANSFER_OFFER_BUILD_BID);
    let fazer_lance: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_ADD_TRANSFER_OFFER);
    let contract_offer: unsafe extern "thiscall" fn(usize, usize, i32, u32, u32, u32, u32, u32) = core::mem::transmute(FN_CONTRACT_MANAGER_ADD_CONTRACT_OFFER);
    let tem_oferta_de_contrato: unsafe extern "thiscall" fn(usize, i32, i32) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_FIND_CONTRACT_OFFER);
    let contrato_com: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let entrada: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(FN_SHORTLIST_MANAGER_FIND_CLUB_SHORTLIST_ENTRY);
    let renovar: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32, u32) = core::mem::transmute(FN_CONTRACT_MANAGER_OFFER_CONTRACT);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_OPERATOR_DELETE);
    let situacao: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u8 = core::mem::transmute(FN_SHORTLIST_MANAGER_UPDATE_STAFF_STATUS);
    let transfer_manager_can_loan_player: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_CAN_LOAN_PLAYER);
    let pedir_emprestimo: unsafe extern "thiscall" fn(usize, usize, usize) -> i32 = core::mem::transmute(FN_SHORTLIST_MANAGER_TRY_LOAN_OFFER_FOR_PLAYER);
    let mut semana = [0u32; 2];
    mais(0xae2c90, &mut semana, 7);
    let pdata = semana.as_mut_ptr() as usize;
    let mut contrato0 = [0u8; 0x50];
    novo_contrato(contrato0.as_mut_ptr() as usize);
    if is_national_team(clube) != 0 || humano(clube, 0, 1, 0, 0) != 0 {
        return;
    }
    if humano(clube, 1, 1, 0, 0) != 0 && responde(read_u32_as_usize(G_HUMAN_MANAGER), tecnico(clube)) != 0
        && regra(read_u32_as_usize(G_HUMAN_MANAGER), 4, tecnico(clube)) == 1
    {
        return;
    }
    let reg = read_u32_as_usize(this + 0xdc).wrapping_add(read_u32_as_usize(clube).wrapping_mul(9));
    if read_u8(reg + 4) != 1 || read_u8(reg + 8) >= 100 || financas(read_u32_as_usize(G_FINANCE_TABLE), clube) == 0 {
        return;
    }
    let mut conta = vagas_com_lance(this, clube, 0xb) as i8;
    conta = conta.wrapping_add(contar_elenco(clube, 0, 0) as i8);
    // ponytail: o motivo (palavra local) a original não inicializa; a 0x8ce990 e a 0x8ced40 o gravam antes da leitura
    let mut motivo: u16 = 0;
    let gm = read_u32_as_usize(G_TRANSFER_MANAGER);
    for s in 0..0x28usize {
        let off = s * 0x25;
        let vaga = |k: usize| read_u32_as_usize(reg).wrapping_add(off) + k;
        let jogador = read_u8(vaga(0)) == 0xb;
        if conta >= 0x32 && jogador {
            continue;
        }
        let id = read_u32_as_usize(vaga(0xf)) as i32;
        if id <= -1 || read_u32_as_usize(vaga(0x17)) as i32 != -1 {
            continue;
        }
        let p = cqc7_pessoa(id as usize);
        if cqe3_clube_do_contrato(p) == clube {
            continue;
        }
        if !jogador {
            let livre = |base: usize, n: i8| {
                let mut k: i8 = 0;
                while k < n {
                    if read_u32_as_usize(clube + base + 4 * k as usize) == 0 {
                        break;
                    }
                    k += 1;
                }
                k != n
            };
            match read_u8(vaga(0)) {
                8 if !livre(0x19f, 5) => continue,
                9 if !livre(0x1b3, 7) => continue,
                10 if !livre(0x1cf, 3) => continue,
                6 | 7 if read_u32_as_usize(clube + club::ASSISTANT_MANAGER) != 0 => continue,
                _ => {}
            }
        }
        if cqe3_registro_0b(p) & 0x30 != 0 && jogador {
            continue;
        }
        if futura(gm, p, 0) != 0 || acertado(gm, p, 0) != 0 {
            continue;
        }
        if read_u32_as_usize(p + 0x24) != 0 && read_u8(p + 0x28) == 5 {
            continue;
        }
        if read_u8(vaga(0x24)) & 1 == 0 {
            // a vaga de compra
            if read_i16(vaga(1)) <= -1 && jogador {
                continue;
            }
            if chegar(gm, p, clube, &mut motivo, 0) == 0 && motivo != 9 {
                continue;
            }
            if tem_oferta(gm, p, clube) != 0 || trocas(gm, p, 0) != 0 {
                continue;
            }
            let mut v = read_u32_as_usize(vaga(0xb)) as i32;
            if read_u8(vaga(0x24)) & 0x20 != 0 {
                v = v.wrapping_shl(1);
                if v <= 0x4e20 {
                    v = 0x4e20;
                }
            }
            let liga = read_u32_as_usize(clube + club::DIVISION);
            if liga != 0 {
                let k = read_u32_as_usize(liga) as i32;
                if k > -1 && k < read_u32_as_usize(G_CLUB_COMP_COUNT) as i32 && read_u32_as_usize(read_u32_as_usize(G_COMP_OBJECT_TABLE).wrapping_add(k as usize * 4)) != 0 {
                    if read_i16(clube + club::REPUTATION) > 0x128e {
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
            if read_u32_as_usize(vaga(0x13)) as i32 > -1 && read_u8(vaga(0)) == 0xb {
                let atual = cqe3_clube_do_contrato(p);
                if pode_contratar(gm, clube, pdata, p, atual, 1, &mut motivo) == 0 && motivo != 9 {
                    let x = read_u32_as_usize(vaga(0xb)) as i32;
                    v = v.wrapping_add(if x > 0x2710 { x } else { 0x2710 });
                }
                let sub = cqc7_pessoa(read_u32_as_usize(vaga(0x13)));
                let les = read_u32_as_usize(G_INJURY);
                if lesao(les, sub) != 0 {
                    let a = read_i16(lesao(les, sub) + 0x2c) as i32;
                    let b = read_i16(lesao(les, sub) + 0x2c) as i32;
                    v = v.wrapping_add(a.wrapping_mul(b).wrapping_mul(read_i16(clube + club::REPUTATION) as i32 / 500));
                }
                let f = read_u32_as_usize(sub + 0x61);
                if f != 0 {
                    let x = read_u8(f + 0x45) as i8 as f64;
                    v = v.wrapping_add(ftol((f64_em(0x968940) - x * f64_em(0x969020)) * v as f64));
                }
                if sob_oferta(gm, sub, 0) != 0 {
                    let x = read_u32_as_usize(vaga(0xb)) as i32 as f64 * f64_em(0x968940);
                    v = v.wrapping_add(ftol(if x > f64_em(0x968910) { x } else { f64_em(0x968910) }));
                }
            }
            let c = cqe3_contrato(p);
            if c != 0 && cqc7_clube(read_u32_as_usize(c + 4)) != 0 && jogador {
                let vendedor = cqe3_clube_do_contrato(p);
                if humano(vendedor, 0, 1, 0, 0) != 0 || read_u32_as_usize(p + 0x39) == clube {
                    let mut contrato = [0u8; 0x50];
                    let pc = contrato.as_mut_ptr() as usize;
                    novo_contrato(pc);
                    let e = vaga(0);
                    // ponytail: o estatuto vai com os bytes altos do ponteiro da vaga (como na original); o cargo, com os do
                    // edx que a 0x600a50 deixou (lixo): a 0x85fc50 recebe um char, só o byte conta
                    let st = (e as u32 & 0xffff_ff00) | ((read_u8(e + 0x24) >> 1) & 0xf) as u32;
                    let r = desejado(pc, p, clube, u32::MAX, st, read_u8(e) as u32, 0, 0);
                    if r as f64 == f64_em(0x9689a0) {
                        continue;
                    }
                    // ponytail: o cargo vai com os bytes altos do resultado e do fnstsw; a 0x5968d0 lê só o byte
                    let t = teto(financas(read_u32_as_usize(G_FINANCE_TABLE), clube), p, ((read_u8(vaga(0x24)) >> 1) & 0xf) as u32);
                    if (t as f64 * f64_em(0x968de8)) < read_u32_as_usize(pc + 0xc) as i32 as f64 {
                        continue;
                    }
                }
            }
            crate::comparacao::ramo("cqe_3 sorteio do lance");
            let r1 = sorteio((read_i16(clube + club::REPUTATION) as i32).wrapping_mul(2).wrapping_add(1000)); // cqe3: o sorteio do lance
            let r2 = sorteio(v);
            if r2 <= r1 {
                continue;
            }
            write_u8(vaga(0x24), read_u8(vaga(0x24)) & 0xdf);
            let mut oferta = [0u8; 0x44];
            let po = oferta.as_mut_ptr() as usize;
            nova_oferta(po);
            let mut termos: usize = 0;
            if preencher(po, p, clube, vaga(0), read_u32_as_usize(vaga(7)) as u32, 0, &mut termos) == 0 {
                situacao(this, p, clube, 1, (read_u8(vaga(0)) != 0xb) as u32);
            } else {
                fazer_lance(gm, po, 0);
                if termos != 0 {
                    // ponytail: o cargo vai com os bytes altos do ponteiro do vetor das vagas (como na original)
                    let cargo = (read_u32_as_usize(reg) as u32 & 0xffff_ff00) | read_u8(vaga(0)) as u32;
                    contract_offer(read_u32_as_usize(G_CONTRACT_MANAGER), termos, read_u32_as_usize(po + 0x20) as i32, read_u32_as_usize(po + 0x24) as u32, cargo, 0, 1, 0);
                    let c = cqe3_contrato(p);
                    let vendedor = if c == 0 { 0 } else { cqc7_clube(read_u32_as_usize(c + 4)) };
                    if vendedor != 0 && humano(vendedor, 0, 1, 0, 0) == 0
                        && tem_oferta_de_contrato(read_u32_as_usize(G_CONTRACT_MANAGER), read_u32_as_usize(p) as i32, read_u32_as_usize(vendedor) as i32) == 0
                    {
                        let cv = contrato_com(read_u32_as_usize(G_CONTRACT_MANAGER), p, vendedor);
                        if cv != 0 {
                            let e = (read_u8(cv + 0x4f) >> 4) as i8;
                            let fator = match e {
                                1 | 2 if entrada(this, vendedor, p, 0, 0) != 0 => {
                                    let rep = read_i16(clube + club::REPUTATION);
                                    let div = if rep < 1 { 1 } else { rep as i32 };
                                    let f = (cqe3_registro_0b(p) & 0xf) as f64 * f64_em(0x967880) + rep as i32 as f64 / div as f64;
                                    Some(if f64_em(0x967890) <= f { if f64_em(0x968dc0) < f { 1.4 } else { f } } else { 1.0 })
                                }
                                3 | 4 | 5 if entrada(this, vendedor, p, 0, 0) != 0 => {
                                    let rep = read_i16(clube + club::REPUTATION) as i32;
                                    let div = if rep.wrapping_mul(2) >= 1 { rep.wrapping_mul(2) } else { 1 };
                                    let f = (cqe3_registro_0b(p) & 0xf) as f64 * f64_em(0x967880) + rep as f64 / div as f64 + f64_em(0x968968);
                                    Some(if f64_em(0x967890) <= f { if f64_em(0x968978) < f { 1.2 } else { f } } else { 1.0 })
                                }
                                _ => None,
                            };
                            if let Some(f) = fator {
                                let b = f.to_bits();
                                renovar(read_u32_as_usize(G_CONTRACT_MANAGER), p, 0, 0, u32::MAX, b as u32, (b >> 32) as u32, 2);
                            }
                        }
                    }
                    liberar(termos);
                }
                write_u32_from_usize(vaga(0x17), read_u32_as_usize(po + 0x20));
                if jogador {
                    conta = conta.wrapping_add(1);
                }
            }
            destruir_oferta(po);
        } else {
            // a vaga de empréstimo
            let n = read_u32_as_usize(clube + club::NATION);
            if n != 0 && read_u32_as_usize(n) == read_u32_as_usize(KEY_NATION_FRANCE) && read_u8(clube + club::PROFESSIONAL_STATUS) == 3 {
                continue;
            }
            if transfer_manager_can_loan_player(gm, clube, p, 0) == 0 {
                continue;
            }
            let c = cqe3_contrato(p);
            if c != 0 && cqc7_clube(read_u32_as_usize(c + 4)) != 0 && humano(cqe3_clube_do_contrato(p), 0, 1, 0, 0) != 0 {
                let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
                if read_u8(contrato(read_u32_as_usize(G_CONTRACT_MANAGER), p) + 0x4e) & 2 == 0 {
                    continue;
                }
            }
            if tem_oferta(gm, p, clube) != 0 || trocas(gm, p, 0) != 0 {
                continue;
            }
            crate::comparacao::ramo("cqe_3 sorteio do empréstimo");
            let r1 = sorteio(read_i16(clube + club::REPUTATION) as i32 / 10); // cqe3: o sorteio do empréstimo
            let r2 = sorteio(read_u32_as_usize(vaga(0xb)) as i32);
            if r2 <= r1 {
                let r1 = sorteio((read_i16(clube + club::REPUTATION) as i32).wrapping_shl(1));
                let r2 = sorteio(read_u32_as_usize(vaga(0xb)) as i32);
                if r2 <= r1 {
                    continue;
                }
            }
            let k = pedir_emprestimo(this, p, clube);
            write_u32_from_usize(vaga(0x17), k as u32 as usize);
            if read_u32_as_usize(vaga(0x17)) as i32 > -1 {
                conta = conta.wrapping_add(1);
            } else {
                // ponytail: o setne vai com os bytes altos do ponteiro da vaga (como na original); a 0x843e10 lê o byte
                let e = vaga(0);
                situacao(this, p, clube, 1, (e as u32 & 0xffff_ff00) | (read_u8(e) != 0xb) as u32);
            }
        }
    }
}

/// Comparador da 0x84a020: só a original (sorteia, faz lances e pedidos de empréstimo, aloca ofertas e termos).
pub(crate) unsafe extern "thiscall" fn comparar_make_club_bids(this: usize, clube: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize)>(IDX_MAKE_CLUB_BIDS)(this, clube) // ponytail: só a original
}

// ---------- 268a leva: a vontade de uma pessoa de ir para um clube (ou de ficar) e o construtor do contrato (shortlist_manager.cpp, staff_contracts.cpp) ----------








/// A conta dos dias entre o ano e o dia de hoje ([0xae2c92], [0xae2c90]) e os de uma data (ano em `ano`, dia em
/// `dia`), como a original: (ano − ano de hoje) × 365 − dia de hoje + dia.
pub(crate) unsafe fn cqe4_dias_para(ano: usize, dia: usize) -> i32 {
    ((read_u16(ano) as i32 - read_u16(G_CURRENT_DATE_YEAR) as i32) * 365 - read_i16(0xae2c90) as i32) + read_i16(dia) as i32
}

/// Hoje menos 150 dias (0x54c8e0: push 0x96; push saída; mov ecx, 0xae2c90 -> thiscall, ret 8) já passou da data
/// guardada em +0x3e (dia) / +0x40 (ano) do técnico `t`: ano maior (sem sinal), ou o mesmo ano e o dia maior (com
/// sinal).
pub(crate) unsafe fn cqe4_depois_de(t: usize) -> bool {
    let menos: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) = core::mem::transmute(FN_DATE_SUB_DAYS);
    let mut d = [0u8; 8];
    menos(0xae2c90, &mut d, 0x96);
    let dia = i16::from_le_bytes([d[0], d[1]]);
    let ano = u16::from_le_bytes([d[2], d[3]]);
    let ano_t = read_u16(t + 0x40);
    ano > ano_t || (ano == ano_t && dia > read_i16(t + 0x3e))
}

/// A distância entre as cidades `a` e `b` (coordenadas f64 em +0x23 e +0x2b), × 55.
pub(crate) unsafe fn cqe4_distancia(a: usize, b: usize) -> f64 {
    let dx = read_f64(a + 0x2b) - read_f64(b + 0x2b);
    let dy = read_f64(a + 0x23) - read_f64(b + 0x23);
    (dy * dy + dx * dx).sqrt() * read_f64(0x96d1b8)
}

/// O máximo da original nas contas de reputação: `x` se ele passa `y`, senão `y`.
pub(crate) fn cqe4_max(x: f64, y: f64) -> f64 {
    if x > y { x } else { y }
}

/// A média das três reputações da ficha do jogador (+9, +0xb, +0xd), truncada.
pub(crate) unsafe fn cqe4_media3(f: usize) -> i32 {
    (read_i16(f + 0xd) as i32 + read_i16(f + 0xb) as i32 + read_i16(f + 9) as i32) / 3
}

/// A reputação do clube pelo estádio: sem estádio ou sem cidade (+0x69, +0x38), trunc(rep × 2/7); com a cidade de
/// importância 0 (+0x33), rep / 2; senão o maior entre rep × 2/7 e rep × importância / 15, truncado.
pub(crate) unsafe fn cqe4_pelo_estadio(clube: usize) -> i32 {
    let rep = read_i16(clube + club::REPUTATION) as i32;
    let e = read_usize(clube + club::STADIUM);
    let cidade = if e != 0 { read_usize(e + 0x38) } else { 0 };
    if cidade == 0 {
        return ftol(rep as f64 * read_f64(0x96d5e0));
    }
    let imp = read_i8(cidade + city::CITY_ATTRACTION);
    if imp == 0 {
        return rep / 2;
    }
    let q = (imp as i32).wrapping_mul(rep) / 15;
    let x = rep as f64 * read_f64(0x96d5e0);
    ftol(cqe4_max(x, q as f64))
}

/// A reputação do clube pela divisão (+0x57, o +0x69 dela): sem divisão, a própria (o resto do registrador vem do
/// ponteiro; só a palavra conta); com reputação de 7500 e público acima de 35000 (+0x73), o maior entre a reputação e
/// 500 × a da divisão; senão o maior entre rep/2 + 250 × a da divisão e 500 × a da divisão.
pub(crate) unsafe fn cqe4_pela_divisao(clube: usize) -> i16 {
    let rep = read_i16(clube + club::REPUTATION) as i32;
    let comp = read_usize(clube + club::DIVISION);
    if comp == 0 {
        return rep as i16;
    }
    let cv = read_i16(comp + competition::CLUB_COMP_REPUTATION) as i32;
    let t = cv.wrapping_mul(500);
    if rep >= 0x1d4c && read_usize(clube + club::ATTENDANCE) as i32 > 0x88b8 {
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
pub(crate) unsafe fn cqe4_pela_competicao(clube: usize, pela_divisao: i16) -> (i32, usize) {
    let rep = read_i16(clube + club::REPUTATION);
    let mut idx = read_usize(clube + club::EURO_FLAG) as i32;
    let mut tem = true;
    if idx <= -1 || (idx == read_usize(KEY_CLUB_COMP_INTER_TOTO_CUP) as i32 && rep >= 0x1388) {
        idx = read_usize(read_usize(G_CLUB_STATE_TABLE).wrapping_add(read_usize(clube).wrapping_mul(6))) as i32;
        if idx <= -1 {
            tem = false;
        }
    }
    let mut reg = 0usize;
    if tem {
        reg = read_usize(G_CLUB_COMP_TABLE).wrapping_add((idx as usize).wrapping_mul(0x6b));
        if reg != 0 {
            let id = read_usize(reg);
            let s: i16 = if id == read_usize(KEY_CLUB_COMP_UEFA_EUROPEAN_CHAMPIONS_LEAGUE) { 15000 } else { read_i16(reg + 0x69).wrapping_mul(600) };
            // 0x4a4850: push clube; mov ecx, [[0xadadfc] + 4 × id] + 8 -> thiscall, ret 4
            let achar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_COMP_STATS_FIND_CLUB_ENTRY);
            let r = achar(read_usize(read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(id.wrapping_mul(4))) + 8), clube);
            if r != 0 && read_u8(r + 4) == 0 && (1..=0x10).contains(&read_u8(r + 0x72)) {
                let p = read_u8(r + 0x72) as f64;
                return (ftol((read_f64(0x96af38) - p * read_f64(0x968a98)) * s as f64), reg);
            }
            return (s as i32, reg);
        }
    }
    let mut x = pela_divisao as f64 * read_f64(0x9689b0);
    if !(x < read_f64(0x96d470)) {
        x = read_f64(0x96d470);
    }
    (ftol(x), reg)
}

/// A reputação vista pelo técnico ([0xb67a44] + id × 0x49: (+0xa + +8) × 0,25 + +6 × 0,5): abaixo de 5000, o maior
/// entre r / (5 − 0,0008 v) e r − 1000 × (5 − 0,0008 v); senão o menor entre r × (0,5 + 0,0001 v) e r + 1000 × isso.
pub(crate) unsafe fn cqe4_pelo_tecnico(clube: usize, r: i16) -> i32 {
    let t = read_usize(read_usize(G_MANAGER_MANAGER)).wrapping_add(read_usize(clube).wrapping_mul(0x49));
    let v = ftol((read_i16(t + 0xa) as f64 + read_i16(t + 8) as f64) * read_f64(0x9689e0) + read_i16(t + 6) as f64 * read_f64(0x968968)) as i16;
    let dd = r as f64;
    if v < 0x1388 {
        let t = read_f64(0x9688a0) - v as f64 * read_f64(0x96a618);
        let q1 = dd / t;
        let m = dd - t * read_f64(0x968908);
        ftol(if q1 > m { q1 } else { m })
    } else {
        let t = v as f64 * read_f64(0x96a5c0) + read_f64(0x968968);
        let q1 = t * dd;
        let m = t * read_f64(0x968908) + dd;
        ftol(if q1 < m { q1 } else { m })
    }
}

/// O teto da reputação pelas finanças (0x5a0590 e 0x5986b0): faixa 1, o maior entre 1,25 r e r + 750; faixa 2, o maior
/// entre 1,5 r e r + 1500.
pub(crate) fn cqe4_pela_faixa(faixa: i8, r: i16) -> i16 {
    match faixa {
        1 => ftol(cqe4_max(r as f64 * 1.25, (r as i32 + 0x2ee) as f64)) as i16,
        2 => ftol(cqe4_max(r as f64 * 1.5, (r as i32 + 0x5dc) as f64)) as i16,
        _ => r,
    }
}

/// min(h / 2, h − 2000) sobre a palavra, a correção pelo +0x64 do clube (diferente de 1; e mais uma vez com 3).
pub(crate) fn cqe4_metade(h: i16) -> i16 {
    let a = h as i32 / 2;
    let e = h as i32 - 2000;
    (if a < e { a } else { e }) as i16
}

/// A fluência: max(F × 0,1, L / (12000 − 200 F)) + 0,2, com L = (s + 5000)² × 1/15000 e F a fluência da pessoa no
/// idioma da nação do clube (0x7c6830: push 1; push idioma; push clube; push pessoa; mov ecx, [0xb74350] -> thiscall,
/// ret 0x10; o idioma vai num registrador com lixo acima, a cópia passa só o byte). Devolve (fator, F).
pub(crate) unsafe fn cqe4_fluencia(p1: usize, clube: usize, s: i16) -> (f64, f64) {
    let q = (s as i32 + 0x1388) as f64;
    let l = q * read_f64(0x968d78) * q;
    let fluencia: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u8 = core::mem::transmute(FN_PLAYERSTATS_GET_LANGUAGE_FLUENCY);
    let f = fluencia(read_usize(G_PLAYER_STATS), p1, clube, read_u8(read_usize(clube + club::NATION) + 0x77) as u32, 1) as i8 as i32 as f64;
    let a = f * read_f64(0x967880);
    let b = l / (read_f64(0x96f938) - f * read_f64(0x96a648));
    (cqe4_max(a, b) + read_f64(0x968918), f)
}

/// the willingness score (returned in st0) and the reason bits: the 2.75 base (3.5 at his own club, the contract
/// running out, the age with no club), the club reputations on loans, liking or disliking the club and the manager,
/// rivals, nations and distance, morale, squad status and demands, the compared reputation (stadium, division,
/// competition, manager, finances), language fluency, time without a club and, when a wage is asked, the default wage
/// against the club wage cap
/// Original: `Shortlist_Manager::staff_interest_in_joining_club` (0x846920, shortlist_manager.cpp, thiscall, ret 0x18).
///
/// [pt] { 0x846920 } (thiscall, ret 0x18, shortlist_manager.cpp): `Shortlist_Manager::staff_interest_in_joining_club(pessoa,
/// clube, salário, empréstimo, situação, motivos)`: a nota de vontade da pessoa de ir para o clube (ou de ficar, quando
/// o clube é o dela), x87 transcrito em f64 na ordem da original. Os motivos (bits) só são marcados com o ponteiro. A
/// nota parte de 2,75 (3,5 para o próprio clube; com contrato acabando, 1,1 − dias × 0,002; sem clube, a idade fora do
/// clube), e passa pelos fatores: reputação dos clubes no empréstimo, gostar ou não do clube e do técnico, rivais, as
/// nações e a distância, a felicidade no contrato (+0x44), a situação no elenco (+0x4f) e a pedida, a reputação comparada (a do clube
/// novo e a do atual, pela divisão, pela competição, pelo técnico e pelas finanças), a fluência no idioma, os jogos
/// de seleção e o tempo sem clube; e, pedido o salário (salário ≠ 0, sem empréstimo e nota de 1,75), o salário padrão
/// (0x864550 num STAFF_CONTRACT local) contra o teto do clube (0x5968d0). O resto das contas está nos comentários.
pub(crate) unsafe extern "thiscall" fn staff_interest_in_joining_club(this: usize, p1: usize, p2: usize, p3: u32, p4: u32, p5: u32, p6: *mut u32) -> f64 {
    use crate::comparacao::ramo;
    let k = |a: usize| read_f64(a);
    let zero = k(0x9689a0);
    // as chamadas por endereço, com as convenções conferidas nos pontos de chamada da original
    // 0x54a110: lea ecx, data -> thiscall, ret 0
    let data_padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    // 0x8cca10: push 0; push pessoa; mov ecx, [0xde1f54] -> thiscall, ret 8
    let futura: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_FUTURE_TRANSFER);
    // 0x4cf5a0: push pessoa; mov ecx, [0xae1050] -> thiscall, ret 4
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    // 0x4cf5f0: push clube; push pessoa; mov ecx, [0xae1050] -> thiscall, ret 8
    let contrato_com: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    // 0x54cdb0: push [0xae2c94]; push [0xae2c90]; lea ecx, data -> thiscall, ret 8
    let dias_ate: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
    // 0x53d980: push 1; push nação; push pessoa; add esp, 0xc -> cdecl
    let estrangeira: unsafe extern "C" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_DATABASE_IS_STAFF_FOREIGN);
    // 0x544590 / 0x544520 / 0x5444a0: push clube; push pessoa; add esp, 8 -> cdecl
    let na_lista_emp: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_LOAN_LISTED);
    let na_lista_transf: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_TRANSFER_LISTED_OR_REQUESTED);
    let in_list: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_TRANSFER_LISTED);
    // 0x547d30 / 0x547d60: push clube; push pessoa; add esp, 8 -> cdecl
    let dislikes_club: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_DISLIKES_CLUB);
    let likes_club: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_FAVOURS_CLUB);
    // 0x547810 / 0x547840: push pessoa; push técnico; add esp, 8 -> cdecl
    let likes: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_FAVOURED_BY);
    let dislikes: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_DISLIKED_BY);
    // 0x4b02e0: push atual; push clube; add esp, 8 -> cdecl
    let rival: unsafe extern "C" fn(usize, usize) -> u8 = core::mem::transmute(FN_COMP_UTIL_GET_LEAGUE_CLASH_TYPE);
    // 0x4ca470: push contrato; push pessoa; mov ecx, [0xae1050] -> thiscall, ret 8
    let categoria: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(FN_CONTRACT_MANAGER_GET_DESERVED_SQUAD_STATUS);
    // 0x600a50: push 0; push 0; push 1 (ou 1); push 0 (ou 1); push clube; add esp, 0x14 -> cdecl
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    // 0x5407e0: push 1; push pessoa; push técnico; add esp, 0xc -> cdecl (só a palavra conta)
    let vista: unsafe extern "C" fn(usize, usize, u32) -> i16 = core::mem::transmute(FN_DATABASE_GET_STAFF_REPUTATION_FOR_VIEWER);
    // 0x5a0590: push clube; mov ecx, [0xae2a38] -> thiscall, ret 4; 0x5986b0: mov ecx, finanças -> thiscall, ret 0
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FINANCETABLE_GET_CLUB_FINANCE);
    let faixa: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(FN_FINANCE_GET_WEALTH_BAND);
    // 0x4cbb60: push clube; push pessoa; mov ecx, [0xae1050] -> thiscall, ret 8
    let lugar: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(FN_CONTRACT_MANAGER_GET_SQUAD_PLACEMENT);
    // 0x669bd0: push &data b; push &data a; push clube; add esp, 0xc -> cdecl
    let na_temporada: unsafe extern "C" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_KEY_NATION_CLUB_IN_SEASON);

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
    if read_usize(p1 + 0x61) == 0 || read_usize(p2 + 0x53) == 0 || read_usize(p1 + 0x1a) == 0 {
        return zero;
    }
    if futura(read_usize(G_TRANSFER_MANAGER), p1, 0) != 0 {
        return zero;
    }
    if read_u8(read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(p1).wrapping_mul(0x4f)) + 0xb) & 0x30 != 0 {
        return zero;
    }
    let a57 = read_i8(p1 + 0x57) as i32 as f64;
    let idade = read_i8(p1 + 0x18);
    let aa8: f32 = ((k(0x968dc0) - a57 * k(0x967878)) - read_i8(p1 + 0x5b) as i32 as f64 * k(0x967878)) as f32;
    let mut a88: f32 = (a57 * k(0x968a98) + k(0x9689b0)) as f32;
    let mut b4: f32 = (a57 * k(0x967888) + k(0x968968)) as f32;
    let c0 = contract_of(read_usize(G_CONTRACT_MANAGER), p1);
    core::ptr::write_unaligned(G_CONTRACT_LOOKUP_RESULT as *mut usize, c0);
    let mut d4 = 0usize;
    if c0 != 0 {
        d4 = read_usize(G_CLUB_TABLE).wrapping_add(read_usize(c0 + 4).wrapping_mul(0x245));
        if d4 != 0 {
            if read_usize(d4 + 0x53) == 0 {
                return k(0x968e38);
            }
            if read_u8(contract_of(read_usize(G_CONTRACT_MANAGER), p1) + 0x35) & 0x3f == 5 {
                d4 = 0;
            }
        }
    }
    let c = contrato_com(read_usize(G_CONTRACT_MANAGER), p1, d4);
    let a0: u32 = if c != 0 { read_usize(c + 0x45) as u32 } else { 0 };
    let r61 = read_usize(p1 + 0x61);
    // o próprio clube com contrato: fica (3,5) quando o contrato não acaba logo e a reputação dele não passa a do clube
    if p2 != 0 && p2 == d4 && c != 0 {
        let dias = dias_ate(c + 0x2d, read_usize(G_CURRENT_DATE) as u32, read_usize(G_CURRENT_DATE_LEAP) as u32);
        let r1 = read_i16(r61 + 0xb) as i32;
        let mut s = ftol((read_i16(p2 + 0x80) as i32 - r1) as f64 * k(0x968968)) as i16;
        if a0 & 0x10 != 0 {
            s = ftol(s as f64 * k(0x968968)) as i16;
        }
        let lim = read_i8(p1 + 0x59) as i32 * 10 + 0x465;
        let s: i32 = if s < 0 { 0 } else if s as i32 > lim { lim as i16 as i32 } else { s as i32 };
        if a0 & 0x161a8 == 0 {
            let id = read_usize(p1) as i32;
            if dias < id % 0x23 + 0x14
                && read_u8(read_usize(G_STAFF_STATE_TABLE).wrapping_add((id as usize).wrapping_mul(0x4f)) + 0xb) & 0xf == 0
                && r1 + read_i8(p1 + 0x57) as i32 * 20 < (read_i8(p1 + 0x59) as i32 + 0xaa) * 25 + s
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
        if c != 0 && d4 != 0 && read_usize(d4 + 0x53) != 0 && read_usize(p2 + 0x53) != 0
            && read_u8(read_usize(p2 + 0x53) + 0x77) == read_u8(read_usize(d4 + 0x53) + 0x77) && a0 & 0x20 != 0
        {
            return zero;
        }
        if idade < 0x10 && estrangeira(p1, read_usize(p2 + 0x53), 1) != 0 {
            return zero;
        }
        ramo("cqe_4 base de outro clube");
        f8 = 2.75; // cqe4: a base para outro clube
        if c != 0 && read_u8(c + 0x49) != 0 {
            let dd = cqe4_dias_para(c + 0x2f, c + 0x2d);
            let v = k(0x9689c0) - dd as f64 * k(0x969048);
            if !p6.is_null() && v < read_i8(p1 + 0x5a) as i32 as f64 * k(0x968a98) + k(0x968e48) {
                *p6 |= 1;
            }
            f8 = v * k(0x9690a8);
        } else if d4 == 0 {
            let dd =((read_u16(G_CURRENT_DATE_YEAR) as i32 - read_u16(p1 + 0x40) as i32) * 365 - read_i16(p1 + 0x3e) as i32) + read_i16(0xae2c90) as i32;
            let mut x = dd.wrapping_mul(dd) as f64 * k(0x96f9b8);
            if k(0x96d6a0) < x {
                x = k(0x96d6a0);
            }
            f8 = x + k(0x9690a8);
        }
    }
    // a reputação dos dois clubes num pedido de empréstimo
    if p4b && r61 != 0 && read_i16(r61 + 0xd) < 0x128e && p2 != 0 && d4 != 0 && p2 != d4 && a0 & 0x10 != 0 {
        let ax = read_i16(p2 + 0x80);
        let mesmo = read_usize(p2 + 0x53) == read_usize(d4 + 0x53);
        let rb = read_i16(r61 + 0xb);
        if ax > read_i16(d4 + 0x80) && (mesmo || ax > 0x1676) {
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
    if dislikes_club(p1, p2) != 0 {
        f8 *= k(0x968f78);
        fl(2);
    }
    if dislikes_club(p1, d4) != 0 {
        let s = read_u8(p1 + 0x5a).wrapping_add(read_u8(p1 + 0x59)).wrapping_add(read_u8(p1 + 0x5b)) as i8;
        if s > 0x2d {
            f8 *= k(0x968928);
        } else if s > 0x19 {
            f8 *= k(0x968940);
        } else {
            f8 = f8 + f8;
        }
        fl(4);
    }
    if likes_club(p1, d4) != 0 && na_lista_transf(p1, d4) == 0 && read_u8(c + 0x49) == 0 {
        f8 *= k(0x968968);
        fl(8);
    }
    if likes_club(p1, p2) != 0 {
        fl(0x10);
        f8 *= k(0x968930);
    }
    if idade > 0x22 && d4 != p2 && read_usize(p2 + 0x53) != read_usize(p1 + 0x1a) {
        f8 *= k(0x969040);
    }
    let max1 = |n: i8| if n < 1 { 1 } else { n as i32 };
    if d4 != 0 && d4 != p2 {
        let (e1, e2) = (read_usize(G_KEY_CLUB_GLASGOW_CELTIC), read_usize(G_KEY_CLUB_GLASGOW_RANGERS));
        let i2 = read_usize(p2);
        let i4 = read_usize(d4);
        if (i2 == e1 || i2 == e2) && (i4 == e1 || i4 == e2) {
            f8 = (k(0x967878) / max1(read_i8(p1 + 0x59)) as f64) * f8;
            fl(0x20);
        } else {
            let n2 = read_usize(read_usize(p2 + 0x53));
            let (na, nb) = (read_usize(KEY_NATION_CROATIA), read_usize(KEY_NATION_YUGOSLAVIA));
            if n2 == nb && read_usize(read_usize(d4 + 0x53)) == na && read_usize(read_usize(p1 + 0x1a)) == na {
                f8 *= k(0x967888);
                fl(2);
            } else if n2 == na && read_usize(read_usize(d4 + 0x53)) == nb && read_usize(read_usize(p1 + 0x1a)) == nb {
                f8 *= k(0x967880);
                fl(2);
            } else {
                if p2 == read_usize(d4 + 0xb3) || p2 == read_usize(d4 + 0xb7) || p2 == read_usize(d4 + 0xbb) {
                    let m = if n2 == read_usize(KEY_NATION_ITALY) {
                        k(0x967890) - read_i8(p1 + 0x59) as i32 as f64 * k(0x968a98)
                    } else {
                        k(0x968918) / max1(read_i8(p1 + 0x59)) as f64
                    };
                    f8 = m * f8;
                    if !p6.is_null() && m < k(0x968e40) {
                        *p6 |= 0x20;
                    }
                }
                if rival(p2, d4) != 0 {
                    let m = if read_usize(read_usize(p2 + 0x53)) == read_usize(KEY_NATION_ITALY) {
                        k(0x967890) - read_i8(p1 + 0x59) as i32 as f64 * k(0x968a98)
                    } else {
                        k(0x969040) / max1(read_i8(p1 + 0x59)) as f64
                    };
                    f8 = m * f8;
                    if !p6.is_null() && m < k(0x968e40) {
                        *p6 |= 0x20;
                    }
                }
            }
        }
    }
    let t2 = read_usize(p2 + 0xcf);
    if likes(t2, p1) != 0 {
        f8 = f8 + f8;
        fl(0x40);
    }
    if dislikes(t2, p1) != 0 {
        f8 *= k(0x968f78);
        fl(0x80);
    }
    if d4 != 0 {
        let t4 = read_usize(d4 + 0xcf);
        if dislikes(t4, p1) != 0 {
            f8 = f8 + f8;
            fl(0x100);
        }
        if likes(t4, p1) != 0 {
            f8 *= k(0x968968);
            if !p6.is_null() && read_u8(c + 0x4e) & 8 == 0 {
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
        let e44 = read_i8(c + 0x44) as i32;
        if p2 == d4 {
            f8 = (e44 as f64 * k(0x969028) + k(0x967890)) * f8;
            ea = p5b;
            if read_usize(p2 + 0xcf) != 0 && cqe4_depois_de(read_usize(p2 + 0xcf)) {
                ea = categoria(read_usize(G_CONTRACT_MANAGER), p1, c) as i8;
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
                if v > k(0x968940) && read_u8(c + 0x4e) & 8 == 0 {
                    *p6 |= 0x400;
                } else if v < k(0x968968) {
                    *p6 |= 0x800;
                }
            }
            if d4 != 0 && read_usize(d4 + 0xcf) != 0 && cqe4_depois_de(read_usize(d4 + 0xcf)) {
                ea = categoria(read_usize(G_CONTRACT_MANAGER), p1, c) as i8;
            } else {
                ea = (read_u8(c + 0x4f) >> 4) as i8;
            }
            if read_u8(c + 0x4e) & 8 != 0 {
                ea = if ea == 1 { 3 } else if ea < 1 || ea > 3 { 7 } else { 4 };
            }
        }
        c1 = read_i8(c + 0x3d);
    }
    let pos20 = || read_u8(read_usize(p1 + 0x61) + 0xf) == 0x14;
    let jovem = idade < 0x20 && read_i8(p1 + 0x22) != 0;
    let mut sb: i16; // ebx: a reputação do clube atual (ou a da pessoa sem clube) na escala da comparação
    let mut e8: i32 = 0; // [base+0x1c]: a do clube novo
    let mut h68: i16 = 0;
    let mut h6c4: i32 = 0;
    let mut ae: i8 = 0;
    let mut den: f32 = 1.0; // [base+0x58]
    if p2 != d4 {
        if humano(p2, 0, 1, 0, 0) != 0 {
            let v = read_i8(read_usize(p2 + 0xcf) + 0x59) as i32 as f64 * k(0x967878) + k(0x968968);
            f8 = v * f8;
            if !p6.is_null() && v < k(0x96a698) {
                *p6 |= 0x20000000;
            }
        }
        match ea {
            1 => {
                f8 *= k(0x96a5d0);
                if !p6.is_null() && read_usize(c + 0x45) == 0 {
                    *p6 |= 0x1000;
                }
            }
            2 => {
                f8 *= k(0x969040);
                if !p6.is_null() && read_usize(c + 0x45) == 0 {
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
                    if read_i8(p1 + 0x22) != 0 {
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
        let cf10 = c != 0 && read_u8(c + 0x45) & 0x10 != 0;
        match p5b {
            1 => {
                if cf10 {
                    f8 = f8 + f8;
                    fl(0x4000);
                } else {
                    f8 *= k(0x96f9b0);
                }
                b4 *= read_f32(0x96a33c);
            }
            2 => {
                if cf10 {
                    f8 *= k(0x96f9a8);
                    fl(0x4000);
                } else {
                    f8 *= k(0x96f9a0);
                }
                b4 *= read_f32(0x96d3b8);
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
                    if read_usize(p1 + 0x69) != 0 {
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
                    if read_i8(p1 + 0x22) != 0 {
                        f8 *= k(0x968968);
                    }
                }
                b4 *= read_f32(0x96d38c);
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
                b4 *= read_f32(0x96f980);
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
                b4 *= read_f32(0x968f18);
            }
            7 => f8 *= k(0x969048),
            _ => {}
        }
        // a reputação do clube novo, em várias escalas, contra a da pessoa
        let mut adv = read_i8(read_usize(p2 + 0x53) + 0x7e);
        if adv < 1 || adv > 3 {
            adv = 3;
        }
        ad = adv.wrapping_mul(adv);
        let h34 = cqe4_pelo_estadio(p2);
        h68 = cqe4_pela_divisao(p2);
        let (h6c, reg) = cqe4_pela_competicao(p2, h68);
        l84 = reg;
        let t = read_usize(p2 + 0xcf);
        let mut rv: i16 = 0;
        if t != 0 && read_usize(t + 0x69) != 0 {
            rv = vista(t, p1, 1);
            if read_usize(G_MANAGER_MANAGER) != 0 {
                rv = cqe4_pelo_tecnico(p2, rv) as i16;
            }
        }
        let mut si: i16 = read_i16(p2 + 0x80);
        let fin = financas(read_usize(G_FINANCE_TABLE), p2);
        if fin != 0 && ad == 1 {
            si = cqe4_pela_faixa(faixa(fin) as i8, si);
        }
        let ax = read_i16(read_usize(this + 0xdc).wrapping_add(read_usize(p2).wrapping_mul(9)) + 5);
        let m = if si > ax { si } else { ax };
        let cx = ftol(si as f64 * k(0x96a5d0) + m as f64 * k(0x969510)) as i16;
        let den0 = ((b4 as f64 + a88 as f64) + aa8 as f64) + read_f32(0x9689a8) as f64;
        den = if den0 > zero { den0 as f32 } else { 0.5 };
        let s = (((h6c as i16 as f64 * b4 as f64 + h68 as f64 * a88 as f64) + h34 as i16 as f64 * aa8 as f64) + cx as f64) + rv as f64;
        let mut w = ftol(s / den as f64) as i16;
        let cl = read_u8(p2 + 0x64);
        if cl != 1 {
            if !(cf10 && read_u8(p1 + 0x22) == 0) {
                w = cqe4_metade(w);
            }
            if cl == 3 {
                w = cqe4_metade(w);
            }
        }
        e8 = ftol(w as f64 * k(0x968940) - k(0x9688e8));
        if read_usize(p2 + 0x53) != 0 {
            let (m, f) = cqe4_fluencia(p1, p2, e8 as i16);
            if idade < 0x12 && f < k(0x9688a0) && read_usize(read_usize(p1 + 0x1a) + 0x71) != read_usize(read_usize(p2 + 0x53) + 0x71) {
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
        let fav = read_usize(p1 + 0x39);
        let segue = 'n: {
            let cid2 = if read_usize(p2 + 0x69) != 0 { read_usize(read_usize(p2 + 0x69) + 0x38) } else { 0 };
            let cid4 = if d4 != 0 && read_usize(d4 + 0x69) != 0 { read_usize(read_usize(d4 + 0x69) + 0x38) } else { 0 };
            if fav != 0 && read_usize(fav + 0x53) == read_usize(p2 + 0x53) {
                if !p4b {
                    break 'n Segue::Padrao;
                }
                let mut g = f8;
                if na_lista_emp(p1, 0) != 0 && idade < 0x18 && r61 != 0 && read_i16(r61 + 0xb) < 0x109a && read_i8(p1 + 0x5b) > 0xf
                    && read_i8(p1 + 0x5a) > 0xa && c != 0 && !matches!(ea, 1 | 2 | 3 | 4)
                {
                    g *= k(0x968e18);
                }
                if d4 != 0 && read_usize(p2 + 0x53) != 0 && read_usize(read_usize(p2 + 0x53)) == read_usize(KEY_NATION_ITALY) && idade < 0x18
                    && read_usize(d4 + 0x57) != 0 && read_usize(read_usize(d4 + 0x57)) == read_usize(KEY_CLUB_COMP_ITALIAN_SERIE_A)
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
            if read_usize(p2 + 0x53) != 0 && estrangeira(p1, read_usize(p2 + 0x53), 1) == 0 {
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
            let t = read_usize(p2 + 0xcf);
            let mesmo_tecnico = t != 0 && read_usize(t + 0x1a) != 0 && estrangeira(p1, read_usize(t + 0x1a), 1) == 0;
            if read_usize(p2 + 0x53) != 0 && read_usize(read_usize(p2 + 0x53)) == read_usize(KEY_NATION_GERMANY) {
                f8 *= k(0x969040);
            }
            if p4b {
                f8 *= if mesmo_tecnico { k(0x96a6a8) } else { k(0x968920) };
                fl(0x40000);
                break 'n Segue::Fatores;
            }
            let c56 = read_i8(p1 + 0x56) as i32;
            let c57 = read_i8(p1 + 0x57) as i32;
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
            if (c != 0 && read_u8(c + 0x49) != 0) || in_list(p1, 0) != 0 {
                f8 *= k(0x968e18);
                if f8 < k(0x968950) && (read_i16(p2 + 0x80) > 0x1a5e || read_i16(p2 + 0x80) > read_i16(d4 + 0x80)) {
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
        } else if na_lista_transf(p1, 0) == 0 && na_lista_emp(p1, 0) == 0 && (c == 0 || read_u8(c + 0x49) == 0) {
            let dd = ((read_u16(G_CURRENT_DATE_YEAR) as i32 - read_u16(p1 + 0x40) as i32) * 365 - read_i16(p1 + 0x3e) as i32) + read_i16(0xae2c90) as i32;
            let e: i32 = if p4b {
                (20 - c1 as i32 / 5).wrapping_mul(dd)
            } else {
                if read_usize(c + 0x45) == 0 && dd < (read_i8(p1 + 0x59) as i32 + 8) * 15 {
                    return zero;
                }
                let dv = read_i8(p1 + 0x59) as i32 + 10;
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
        if humano(p2, 0, 1, 0, 0) != 0 && cqe4_depois_de(read_usize(p2 + 0xcf)) {
            let lim = 0xceu8.wrapping_sub(read_u8(p1 + 0x59)) as i8;
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
                    if read_i8(p1 + 0x22) != 0 {
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
        let mut aev = read_i8(read_usize(p1 + 0x1a) + 0x7e);
        if aev < 1 || aev > 3 {
            aev = 3;
        }
        ae = aev.wrapping_mul(aev);
        sb = ftol(cqe4_media3(read_usize(p1 + 0x61)) as i16 as f64 * k(0x968940) - k(0x9688e8)) as i16;
    } else {
        let n4 = read_usize(d4 + 0x53);
        let mut aev = if n4 != 0 { read_i8(n4 + 0x7e) } else { 3 };
        if aev < 1 || aev > 3 {
            aev = 3;
        }
        ae = aev.wrapping_mul(aev);
        let h44 = cqe4_pelo_estadio(d4);
        h6c4 = cqe4_pela_divisao(d4) as i32;
        let (bx, reg) = cqe4_pela_competicao(d4, h6c4 as i16);
        l8c = reg;
        let t4 = read_usize(d4 + 0xcf);
        let si: i16 = if t4 == 0 || read_usize(t4 + 0x69) == 0 {
            ftol(h6c4 as i16 as f64 * k(0x969040)) as i16
        } else if read_usize(G_MANAGER_MANAGER) == 0 {
            read_i16(read_usize(t4 + 0x69) + 0xa)
        } else {
            cqe4_pelo_tecnico(d4, read_i16(read_usize(t4 + 0x69) + 0xa)) as i16
        };
        let mut di: i16 = read_i16(d4 + 0x80);
        let fin = financas(read_usize(G_FINANCE_TABLE), d4);
        if fin != 0 && ae == 1 && c != 0 && read_usize(c + 0x45) == 0 {
            let m = faixa(fin);
            if m == 0xfe {
                fl(0x40000000);
            } else {
                di = cqe4_pela_faixa(m as i8, di);
            }
        }
        let ax = read_i16(read_usize(this + 0xdc).wrapping_add(read_usize(d4).wrapping_mul(9)) + 5);
        let m = if di > ax { di } else { ax };
        let axv = ftol(di as f64 * k(0x96a5d0) + m as f64 * k(0x969510)) as i16;
        // ponytail: no próprio clube (p2 == d4) a original divide pelo float [base+0x58] que ninguém gravou (sobra de
        // pilha); o resultado não é usado depois (o caminho vai direto ao fim), e a cópia divide por 1.
        let s = (((bx as i16 as f64 * b4 as f64 + h6c4 as i16 as f64 * a88 as f64) + h44 as i16 as f64 * aa8 as f64) + axv as f64) + si as f64;
        let mut di = ftol(s / den as f64) as i16;
        if !p4b && in_list(p1, 0) != 0 {
            di = ftol(di as f64 * k(0x9689d8)) as i16;
        }
        let cl = read_u8(d4 + 0x64);
        if cl != 1 {
            if !(c != 0 && read_u8(c + 0x45) & 0x10 != 0 && read_u8(p1 + 0x22) == 0) {
                di = cqe4_metade(di);
            }
            if cl == 3 {
                di = cqe4_metade(di);
            }
        }
        if lugar(read_usize(G_CONTRACT_MANAGER), p1, d4) == 3 {
            di = ftol(di as f64 * k(0x968e40)) as i16;
        }
        sb = ftol(di as f64 * k(0x968940) - k(0x9688e8)) as i16;
        if n4 != 0 && p2 != d4 {
            let (m, _) = cqe4_fluencia(p1, d4, sb);
            f8 = f8 / m;
        }
    }
    if p2 != d4 {
        let comp2 = read_usize(p2 + 0x57);
        if comp2 == 0 || read_usize(comp2) == read_usize(KEY_CLUB_COMP_A_LOWER_DIVISION) {
            f8 *= k(0x967880);
        } else if read_usize(p1 + 0x39) == p2 && r61 != 0 && read_i8(r61 + 0x45) > 0xa {
            f8 = read_i8(r61 + 0x45) as i32 as f64 * f8 * k(0x967880);
        }
        let mut a8 = (read_i16(p2 + 0x80) as i32 - cqe4_media3(r61)) as f64 * k(0x96a6b8) + k(0x967890);
        let e8s = e8 as i16 as i32;
        let sbs = sb as i32;
        let cf10 = c != 0 && read_u8(c + 0x45) & 0x10 != 0;
        let mut longo = false;
        if p4b && (cf10 || na_lista_emp(p1, 0) != 0) {
            let f = read_usize(p1 + 0x61);
            longo = (read_i16(f + 5) as i32 * 50) < read_i16(d4 + 0x80) as i32
                && read_i8(p1 + 0x22) == 0
                && !(idade >= 0x18 && idade <= 0x20)
                && read_i16(p2 + 0x80) > 0xdac
                && read_i16(f + 0xb) < 0x157c
                && read_usize(p2 + 0x53) != 0
                && read_usize(p2 + 0x57) != 0
                && read_usize(d4 + 0x57) != 0
                && !(read_i16(read_usize(p2 + 0x57) + 0x69) <= 6 && read_i16(read_usize(p2 + 0x57) + 0x69) as i32 <= read_i16(read_usize(d4 + 0x57) + 0x69) as i32 - 3)
                && read_usize(d4 + 0x53) != 0
                && !(read_i8(read_usize(p2 + 0x53) + 0x85) < 0xa && read_i8(read_usize(p2 + 0x53) + 0x85) < read_i8(read_usize(d4 + 0x53) + 0x85));
        }
        let dv: f64 = if longo {
            (e8s - sbs) as f64 * k(0x968968)
        } else if cf10 && e8s < sbs && sbs - e8s < 0x1388 {
            let d = (sbs - e8s) as f64;
            d * k(0x96f970) * d
        } else if e8s > sbs {
            if c != 0 && read_u8(c + 0x45) & 1 != 0 {
                (e8s - sbs) as f64 * k(0x968e18)
            } else {
                (read_i8(p1 + 0x57) as i32 as f64 * k(0x96f930) + k(0x969040)) * (e8s - sbs) as f64
            }
        } else if p5b > 3 {
            (e8s - sbs) as f64 * k(0x968940)
        } else if !(f8 > k(0x9688b0)) {
            (e8s - sbs) as f64
        } else {
            let v = if sbs > 0x1b58 && e8s < 0x7d0 && p4b {
                let z = read_f32(0x9689a8) as f64 - (sbs - e8s) as f64 * read_f32(0x96f968) as f64;
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
        if c != 0 && read_u8(c + 0x46) & 1 != 0 {
            e0 *= k(0x968940);
        }
        let mut x = e0 * f8;
        let sem_pedida = |c: usize| c == 0 || read_u8(c + 0x49) == 0;
        if !p6.is_null() {
            if e0 < k(0x968e48) {
                let ok = b4 as f64 > k(0x968978) && l8c != 0 && l84 == 0 && x > k(0x968928) && ea < 4 && ea > 0 && sem_pedida(c);
                *p6 |= if ok { 0x400000 } else { 0x800000 };
            } else if e0 > k(0x968940) && b4 as f64 > k(0x968978) && l8c == 0 && l84 != 0
                && !(d4 != 0 && read_i16(d4 + 0x80) >= 0x1d4c) && p5b < 4 && p5b > 0 && sem_pedida(c)
            {
                *p6 |= 0x200000;
            }
        }
        if !p4b {
            x *= e0;
        }
        if c != 0 && read_u8(c + 0x45) & 1 != 0 {
            x *= e0;
            if !p6.is_null() {
                if e0 < k(0x9689b0) {
                    let ok = b4 as f64 > k(0x968978) && l8c != 0 && l84 == 0 && x > k(0x968928) && ea < 4 && ea > 0 && read_u8(c + 0x49) == 0;
                    *p6 |= if ok { 0x400000 } else { 0x800000 };
                } else if e0 > k(0x968950) && b4 as f64 > k(0x968978) && l8c == 0 && l84 != 0
                    && !(d4 != 0 && read_i16(d4 + 0x80) >= 0x1d4c) && p5b < 4 && p5b > 0 && read_u8(c + 0x49) == 0
                {
                    *p6 |= 0x200000;
                }
            }
        }
        if p4b {
            if read_i16(r61 + 0xb) > sb && sbs > e8s {
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
                let t = read_usize(read_usize(p1 + 0x1a) + 0x71);
                if t != 0 && read_usize(t) == read_usize(KEY_CONTINENT_EUROPE) {
                    let u = read_usize(read_usize(p2 + 0x53) + 0x71);
                    if u != 0 && read_usize(u) != read_usize(KEY_CONTINENT_EUROPE) && u != read_usize(read_usize(d4 + 0x53) + 0x71) {
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
                if read_usize(read_usize(p2 + 0x53) + 0x71) != read_usize(read_usize(d4 + 0x53) + 0x71) && q < k(0x967890) {
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
        let s7 = read_i16(read_usize(p1 + 0x61) + 7);
        if s7 > 0x82 && s7 as i32 > read_i16(d4 + 0x80) as i32 / 60 && idade < 0x1a
            && na_temporada(d4, data_a.as_mut_ptr() as usize, data_b.as_mut_ptr() as usize) != 0 && c1 > 0 && c != 0
        {
            let ano = u16::from_le_bytes([data_b[2], data_b[3]]);
            let dia = i16::from_le_bytes([data_b[0], data_b[1]]);
            let ano_c = read_u16(c + 0x2f);
            let antes = ano < ano_c || (ano == ano_c && dia < read_i16(c + 0x2d));
            if antes && read_usize(c + 0x45) == 0 && read_u8(c + 0x49) == 0
                && !(humano(p2, 1, 1, 0, 0) != 0 && read_i8(p1 + 0x5a) <= 0xf)
                && dias_ate(data_b.as_mut_ptr() as usize, read_usize(G_CURRENT_DATE) as u32, read_usize(G_CURRENT_DATE_LEAP) as u32) < 0x96
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
    let construir: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_STAFF_CONTRACT_STAFF_CONTRACT);
    construir(contrato_local.as_mut_ptr() as usize);
    let fin = financas(read_usize(G_FINANCE_TABLE), p2);
    if fin == 0 {
        return zero;
    }
    // 0x864550: push 0; push 0xb; push situação; push 0; push clube; push pessoa; lea ecx, contrato -> thiscall, ret 0x18
    let padrao: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32) -> f64 = core::mem::transmute(FN_STAFF_CONTRACT_DEFAULT_STAFF_CONTRACT);
    let w0 = padrao(contrato_local.as_mut_ptr() as usize, p1, p2, 0, p5, 0xb, 0);
    // 0x5968d0: push situação; push pessoa; mov ecx, finanças -> thiscall, ret 8
    let teto: unsafe extern "thiscall" fn(usize, usize, u32) -> i32 = core::mem::transmute(FN_FINANCE_GET_MAX_WAGE);
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
        if read_i8(c + 0x3d) < 0 {
            e8w = e8w - read_i8(c + 0x3d) as i32 as f64 * k(0x968918);
        }
        if a0 != 0 {
            e8w = e8w - read_i8(c + 0x44) as i32 as f64 * k(0x968918);
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
    let mut g = read_usize(c + 0xc) as i32 as f64;
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
pub(crate) unsafe extern "thiscall" fn comparar_staff_interest_in_joining_club(this: usize, p1: usize, p2: usize, p3: u32, p4: u32, p5: u32, p6: *mut u32) -> f64 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, *mut u32) -> f64>(IDX_STAFF_INTEREST_IN_JOINING_CLUB);
    let mut regioes = vec![(0xae1080usize, 4usize)];
    if !p6.is_null() {
        regioes.push((p6 as usize, 4));
    }
    let (ro, rr, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, p1, p2, p3, p4, p5, p6), || staff_interest_in_joining_club(this, p1, p2, p3, p4, p5, p6));
    PLACAR.registrar(igual && ro.to_bits() == rr.to_bits(), || format!("staff_interest_in_joining_club(pessoa {:#x}, clube {:#x}, {}, {}, {}) original={} {:x?} rust={} {:x?}", p1, p2, p3, p4 as u8, p5 as u8, ro, mo, rr, mr));
    ro
}



// ---------- 267a leva: a troca com compensação, a ordem das vagas da lista de compras e o jogador sob oferta (shortlist_manager.cpp) ----------



/// Contador de ramo para a rotina quente (~27 milhões de chamadas até 2002-06-01): o `ramo` só a cada 4096 passagens
/// (a primeira inclusive), para não pesar na carreira.
pub(crate) fn cqd12_contar(n: &core::sync::atomic::AtomicU32, nome: &'static str) {
    // ponytail: rotina quente; o contador do ramo conta de 4096 em 4096
    if n.fetch_add(1, core::sync::atomic::Ordering::Relaxed) & 0xfff == 0 {
        crate::comparacao::ramo(nome);
    }
}

pub(crate) static CQD12_BOSMAN: core::sync::atomic::AtomicU32 = core::sync::atomic::AtomicU32::new(0);
pub(crate) static CQD12_FAIXA: core::sync::atomic::AtomicU32 = core::sync::atomic::AtomicU32::new(0);
pub(crate) static CQD12_LONGE: core::sync::atomic::AtomicU32 = core::sync::atomic::AtomicU32::new(0);

/// the club takes the player paying the compensation: outside the FIFA protected period, if the fee (0x8d0780) is below
/// his value; inside it, only with the Bosman rule and the fee up to the value, by the contract expiry band (0 no; 1-8:
/// the slot's +3 × 0.5 to 0.9 above the price; 9-14 yes)
/// Original: `shortlist_manager_is_compensation_deal` (0x84b6f0, shortlist_manager.cpp, thiscall).
///
/// [pt] { 0x84b6f0 } (thiscall, ret 0x10, shortlist_manager.cpp; o this não é lido): `shortlist_manager_is_compensation_deal(
/// vaga, contrato, clube, &taxa)`, pela vaga da lista de compras (0x842e10, 0x840000), pelo relatório do olheiro
/// (0x7b62a0), pelo lance (0x8e5d60) e pela tela. Zera a taxa; sem contrato, sem vaga ou com o byte +0x49 do contrato:
/// 1. A pessoa do contrato ([0xae23bc] + 110 × [+0]). Fora do período protegido da FIFA (0x868210 zero): a taxa de
/// compensação (0x8d0780 com 0) e 1 se ela é menor que o valor (+0x52) da pessoa. Senão: sem lei Bosman no clube do
/// jogador (0x8d0a70), 0; a taxa (0x8d0780 com 1) acima do valor, 0; pela faixa do vencimento (0x867d30): 0 → 0; 1-2,
/// 3-4, 5-6, 7-8 → 1 se o +3 da vaga × 0,5 / 0,65 / 0,8 / 0,9 passa do preço (+0xb); 9-14 → 1; acima, 0.
pub(crate) unsafe extern "thiscall" fn shortlist_manager_is_compensation_deal(this: usize, vaga: usize, contrato: usize, clube: usize, taxa: *mut i32) -> u32 {
    let _ = this;
    *taxa = 0;
    if contrato == 0 || vaga == 0 || *((contrato + contract::LEAVING_ON_BOSMAN) as *const u8) != 0 {
        return 1;
    }
    let pessoa = |c: usize| read_usize(G_STAFF_TABLE).wrapping_add(read_usize(c).wrapping_mul(0x6e));
    let protegido: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32 = core::mem::transmute(FN_STAFF_CONTRACT_CHECK_FIFA_PROTECTED_PERIOD);
    let compensacao: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> i32 = core::mem::transmute(FN_TRANSFER_MANAGER_CALC_COMPENSATION_FEE);
    let bosman: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_PLAYER_CLUB_USES_BOSMAN);
    let faixa: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(FN_STAFF_CONTRACT_GET_EXPIRY_BAND);
    // 0x868210: push 0; push 0; push clube; mov ecx, contrato -> thiscall; 0x8d0780 e 0x8d0a70: mov ecx, [0xde1f54]
    // antes dos pushes (a cópia passa o ecx como a original; as duas só leem a pilha, ret 0xc e ret 4); 0x867d30: mov
    // ecx, contrato -> thiscall
    let gerente = read_usize(G_TRANSFER_MANAGER);
    if protegido(contrato, clube, 0, 0) == 0 {
        let t = compensacao(gerente, pessoa(contrato), clube, 0);
        *taxa = t;
        return (t < read_i32(pessoa(contrato) + 0x52)) as u32;
    }
    if bosman(gerente, pessoa(contrato)) == 0 { // cqd12: sem a lei Bosman
        return 0;
    }
    cqd12_contar(&CQD12_BOSMAN, "cqd_12 com lei Bosman");
    let t = compensacao(gerente, pessoa(contrato), clube, 1);
    *taxa = t;
    if t > read_i32(pessoa(contrato) + 0x52) {
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
    (read_i32(vaga + 3) as f64 * fator > read_i32(vaga + 0xb) as f64) as u32
}

/// Comparador: lado a lado (rotina pura: só a taxa e a devolução), pelas cópias leves.
pub(crate) unsafe extern "thiscall" fn comparar_shortlist_manager_is_compensation_deal(this: usize, vaga: usize, contrato: usize, clube: usize, taxa: *mut i32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut i32) -> u32>(IDX_SHORTLIST_MANAGER_IS_COMPENSATION_DEAL);
    let reg = [(taxa as usize, 4)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, vaga, contrato, clube, taxa),
        || shortlist_manager_is_compensation_deal(this, vaga, contrato, clube, taxa));
    PLACAR.registrar(igual && r == n, || format!("shortlist_manager_is_compensation_deal(vaga {:#x}, contrato {:#x}, clube {:#x}) original={} rust={} {:x?} x {:x?}",
        vaga, contrato, clube, r, n, mo, mr));
    r
}

/// the qsort criterion of the 40 shortlist slots: the difference of the indices (+0x23); equal, 1
/// Original: `shortlist_compare_slots_by_index` (0x83b6c0, shortlist_manager.cpp, cdecl).
///
/// [pt] { 0x83b6c0 } (cdecl, shortlist_manager.cpp): `shortlist_compare_slots_by_index(a, b)`, o critério do qsort das 40
/// vagas (0x25 bytes) da lista de compras na avaliação do elenco (0x83cb40): a diferença dos índices (+0x23, byte com
/// sinal); iguais, 1 (nunca empata).
pub(crate) unsafe extern "C" fn shortlist_compare_slots_by_index(a: usize, b: usize) -> i32 {
    let d = (*((a + 0x23) as *const i8) as i32) - (*((b + 0x23) as *const i8) as i32);
    if d != 0 { d } else { 1 }
}

/// Comparador: a devolução (pura).
pub(crate) unsafe extern "C" fn comparar_shortlist_compare_slots_by_index(a: usize, b: usize) -> i32 {
    let o = orig::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_SHORTLIST_COMPARE_SLOTS_BY_INDEX);
    let r = o(a, b);
    let n = shortlist_compare_slots_by_index(a, b);
    PLACAR.registrar(r == n, || format!("shortlist_compare_slots_by_index({:#x}, {:#x}) original={} rust={}", a, b, r, n));
    r
}

/// Os clubes interessados no jogador, como a 0x84b580 os pede (0x842180 com modo 1 numa lista local; os itens de 8
/// bytes: id do jogador, id do clube).
pub(crate) unsafe fn cqd12_interessados(this: usize, jogador: usize) -> Vec<usize> {
    let nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let juntar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) -> u32 = core::mem::transmute(FN_SHORTLIST_MANAGER_COLLECT_INTERESTED_CLUBS);
    let mut l = [0u8; 0x10];
    let lista = l.as_mut_ptr() as usize;
    nova(lista);
    juntar(this, jogador, lista, 0, 1, 0);
    let mut v = Vec::new();
    let mut no = read_usize(lista + 2);
    while no != 0 && v.len() < 0x10000 {
        v.push(read_usize(read_usize(no) + 4));
        no = read_usize(no + 6);
    }
    soltar(lista);
    v
}

/// the player under offer: his slot in the interested clubs' lists (except human clubs, the buyer or the clubs of the
/// exchange offers) gets bit 0x20, which doubles the AI bid price
/// Original: `Shortlist_Manager::mark_player_under_offer` (0x84b580, shortlist_manager.cpp, thiscall).
///
/// [pt] { 0x84b580 } (thiscall, ret 0xc, shortlist_manager.cpp, quadro SEH da lista local 0x68d020/0x68d040):
/// `Shortlist_Manager::mark_player_under_offer(jogador, comprador, troca)`, pela negociação acrescentada (0x8be9d0) e
/// pelo lance aceito (0x8c0080). Os clubes interessados no jogador (0x842180 com modo 1, numa lista local), menos os de
/// humano que decide (0x600a50 com 0, 1): com a lista `troca` (cursor na cabeça), um clube de alguma oferta dela
/// (0x8c61e0 pelo id do item; o clube +8 da oferta) fica de fora; senão a vaga do jogador (0x849ce0, só de tipo 11 se o
/// byte +0x11 da última oferta achada é 11) ganha o bit 0x20 (+0x24); sem a lista, fora o comprador, a vaga de tipo 11
/// ou, sem ela, a de outro tipo. Clube sem vaga encerra.
pub(crate) unsafe extern "thiscall" fn mark_player_under_offer(this: usize, jogador: usize, comprador: usize, troca: usize) {
    let nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let juntar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) -> u32 = core::mem::transmute(FN_SHORTLIST_MANAGER_COLLECT_INTERESTED_CLUBS);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let oferta: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_TRANSFER_MANAGER_GET_OFFER_SLOT);
    let vaga_de: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(FN_SHORTLIST_MANAGER_FIND_CLUB_SHORTLIST_ENTRY);
    // 0x68d020/0x68d040/0x68d1b0: lea ecx, lista -> thiscall; 0x842180: 5 pushes; mov ecx, ebx (ret 0x14); 0x600a50:
    // 5 pushes; add esp, 0x14 -> cdecl; 0x8c61e0: push id; mov ecx, [0xde1f54] (ret 4); 0x849ce0: 4 pushes; mov ecx,
    // ebx (ret 0x10)
    let mut l = [0u8; 0x10];
    let lista = l.as_mut_ptr() as usize;
    nova(lista);
    juntar(this, jogador, lista, 0, 1, 0);
    core::ptr::write_unaligned((lista + 0xa) as *mut usize, read_usize(lista + 2));
    let mut ultima = jogador; // o ebp da original: o jogador, depois a última oferta achada (vale entre os clubes)
    let mut item = proximo(lista);
    'clubes: while item != 0 {
        let k = read_usize(item + 4);
        let clube = read_usize(G_CLUB_TABLE).wrapping_add(k.wrapping_mul(0x245));
        if humano(clube, 0, 1, 0, 0) == 0 {
            let v = if troca != 0 {
                core::ptr::write_unaligned((troca + 0xa) as *mut usize, read_usize(troca + 2));
                let mut q = proximo(troca);
                while q != 0 {
                    ultima = oferta(read_usize(G_TRANSFER_MANAGER), read_usize(q));
                    if ultima != 0 && read_usize(read_usize(ultima) + 8) == k {
                        item = proximo(lista);
                        continue 'clubes;
                    }
                    q = proximo(troca);
                }
                let tipo = *((read_usize(ultima) + 0x11) as *const u8);
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
pub(crate) unsafe extern "thiscall" fn comparar_mark_player_under_offer(this: usize, jogador: usize, comprador: usize, troca: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(IDX_MARK_PLAYER_UNDER_OFFER);
    let mut reg: Vec<(usize, usize)> = cqd12_interessados(this, jogador).into_iter()
        .map(|k| read_usize(read_usize(this + 0xdc).wrapping_add(k.wrapping_mul(9))))
        .filter(|&t| t != 0)
        .map(|t| (t, 40 * 0x25))
        .collect();
    if troca != 0 {
        reg.push((troca + 0xa, 4));
    }
    let ((), (), igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, jogador, comprador, troca),
        || mark_player_under_offer(this, jogador, comprador, troca));
    PLACAR.registrar(igual, || format!("mark_player_under_offer(jogador {:#x}, comprador {:#x}, troca {:#x}) {:x?} x {:x?}",
        jogador, comprador, troca, mo, mr));
}

// ---------- 263a leva: o valor do jogador para o clube (a nota pela vaga da lista de observação e o preço que o clube pagaria), a entrada do jogador na busca do clube observado e o prêmio por jogos na seleção achado (shortlist_manager.cpp, scout_manager.cpp, transfer_manager.cpp) ----------





/// O fim sem valor da 0x840d80 (0x841f62): fora da lista de empréstimos da vaga (bit 0 de +0x24), 0; com ela e a pessoa
/// listada para empréstimo (0x544590 com 0), a nota vira max(nota + 5000, nota × 1,2) (_ftol) e 0.
pub(crate) unsafe fn cqa7_sem_valor(p: usize, vaga: usize, saida: *mut i32) -> i32 {
    if read_u8(vaga + 0x24) & 1 == 0 {
        return 0;
    }
    let emprestimo: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_LOAN_LISTED);
    if emprestimo(p, 0) != 0 {
        let r = *saida;
        let a = r.wrapping_add(5000) as f64;
        let q = r as f64 * read_f64(0x968978);
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
pub(crate) unsafe fn cqa7_jovem(this: usize, clube: usize, p: usize, saida: *mut i32, minimo: i32, alta: bool, idade: i8, r: i32, frac: &mut f64, e: &mut u32) {
    if idade >= 0x1b || r <= 20000 {
        return;
    }
    let vista: unsafe extern "C" fn(usize, usize, usize, u32, u32) -> i16 = core::mem::transmute(FN_MATCH_MAN_JUDGE_PLAYER_ABILITY);
    let sv = vista(read_usize(clube + club::MANAGER), p, clube, 1, (r / 4) as u32) as i32;
    let o = *saida as f64;
    let mut d1 = sv as f64 * read_f64(0x968f48) / o;
    let teto = *frac + read_f64(0x968e48);
    if !(d1 < teto) {
        d1 = teto;
    }
    match idade {
        0x18 => d1 = d1 * read_f64(0x968e40) + read_f64(0x968e80),
        0x19 => d1 = (d1 + read_f64(0x967890)) * read_f64(0x968968),
        0x1a => d1 = d1 * read_f64(0x968e80) + read_f64(0x968e40),
        _ => {}
    }
    if minimo as f64 * read_f64(0x9689b8) < o || alta {
        if !(d1 > read_f64(0x967890)) {
            return;
        }
        let a = read_i16(clube + club::REPUTATION);
        let ra = if a < 1000 { 1000 } else { a as i32 };
        let nr = read_i16(read_usize(this + 0xdc).wrapping_add(read_usize(clube).wrapping_mul(9)) + 5) as i32;
        let mut x = nr as f64 / ra as f64;
        if !(read_f64(0x967890) <= x) {
            x = read_f64(0x967890);
        } else if read_f64(0x968978) < x {
            x = read_f64(0x968978);
        }
        d1 = d1 * x;
    }
    if d1 > read_f64(0x967890) {
        crate::comparacao::ramo("cqa_7 jovem valorizado");
        *frac = d1 * *frac;
        let h = (d1 + read_f64(0x967890)) * read_f64(0x968968);
        *e = ftol(*e as f64 * h) as u32;
        *saida = ftol(o * h);
    }
}

/// the player's rating for the slot (through the club's best observer, the scout list at the human club, the fog of
/// war) and the fee the club would pay: the base value, age, the young players' seen ability, consistency, the caps by
/// position, reputation, record, age, division and the club's nation
/// Original: `Shortlist_Manager::value_player_for_club` (0x840d80, shortlist_manager.cpp, thiscall, ret 0x18).
///
/// [pt] { 0x840d80 } (thiscall, ret 0x18, shortlist_manager.cpp, 4,6 KB de x87; o mapa dá 1332 bytes):
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
pub(crate) unsafe extern "thiscall" fn value_player_for_club(this: usize, clube: usize, p: usize, vaga: usize, saida: *mut i32, minimo: i32, teto: u32) -> i32 {
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let escopo: unsafe extern "C" fn(usize, usize) -> u8 = core::mem::transmute(FN_DATABASE_GET_REPUTATION_SCOPE);
    let conhecer: unsafe extern "C" fn(usize, i8) -> i32 = core::mem::transmute(FN_NON_PLAYER_ATTRIBUTE_RATING);
    let emprestimo: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_LOAN_LISTED);
    let nota: unsafe extern "C" fn(usize, u32, u32, usize, i32, usize) -> i32 = core::mem::transmute(FN_MATCH_MAN_SCORE_PLAYER_FOR_SELECTION);
    let mut obs = read_usize(clube + club::MANAGER);
    let t7 = read_usize(vaga + 7) as i32;
    let teto_i = if t7 > 1 { t7 } else { 1 };
    let mut div: i8 = 4;
    if obs != 0 && read_usize(obs + 0x69) == 0 {
        let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
        let i = sorteio(read_usize(G_NON_PLAYER_COUNT) as i32);
        core::ptr::write_unaligned((obs + 0x69) as *mut usize, read_usize(G_NON_PLAYER_TABLE).wrapping_add((i as usize).wrapping_mul(0x44)));
    }
    // ponytail: a original empurra a palavra da posição com a palavra alta de ecx (lixo); quem recebe lê só a palavra
    let pos = read_i16(vaga + 1) as u16 as u32;
    let tipo = read_u8(vaga + 0x24) & 0x1e;
    let flags: u32 = if tipo == 4 || tipo == 6 {
        let mut f = 0x5882;
        if read_usize(vaga + 0xf) == read_usize(p) && read_usize(p + 0x39) == clube && humano(clube, 1, 1, 0, 0) != 0 {
            f = 0x815882;
        }
        match escopo(p, read_usize(clube + club::MANAGER)) as i8 {
            0 => f | 0x40,
            1 => f | 0x50,
            2 => f | 0x10,
            3 => f | 0x30,
            4 => f | 0x20,
            _ => f,
        }
    } else if tipo == 8 {
        match escopo(p, read_usize(clube + club::MANAGER)) as i8 {
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
    if read_u8(vaga + 0x24) & 1 != 0 {
        div = if emprestimo(p, 0) != 0 { 0x10 } else { 6 };
    }
    // o mínimo² × 0,000025 / divisor (_ftol)
    let corte = |m: i32, div: i8| {
        let q = m as f64;
        ftol(q * read_f64(0x96d240) * q / div as f64)
    };
    if humano(clube, 1, 1, 0, 0) != 0 {
        let tecnico: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_CLUB_MANAGER);
        let in_list: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_SCOUT_MANAGER_IS_PLAYER_SHORTLISTED);
        let substituto: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_HUMAN_MANAGER_GET_STAND_IN_MANAGER);
        let m = tecnico(clube);
        if in_list(read_usize(G_SCOUT_MANAGER), p, m) != 0 {
            let s = substituto(clube);
            let r = nota(p, pos, flags, clube, -1, s);
            *saida = r;
            let a = r.wrapping_add(15000) as f64;
            let q = r as f64 * read_f64(0x968940);
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
                if obs != 0 && read_usize(obs + 0x69) != 0 {
                    melhor = conhecer(read_usize(obs + 0x69), 6) as i8;
                    match escopo(p, obs) as i8 {
                        3 => melhor = melhor.wrapping_sub(5),
                        4 => melhor = melhor.wrapping_sub(10),
                        _ => {}
                    }
                }
            } else if obs != 0 {
                // a marca 0x80: o técnico entra com +5 (−5 e +0 pelo escopo 4 e 3)
                melhor = (conhecer(read_usize(obs + 0x69), 5) as i8).wrapping_add(5);
                match escopo(p, obs) as i8 {
                    3 => melhor = melhor.wrapping_sub(5),
                    4 => melhor = melhor.wrapping_sub(10),
                    _ => {}
                }
            }
            for k in 0..7 {
                let s = read_usize(clube + club::SCOUTS + 4 * k);
                if s == 0 || read_usize(s + 0x69) == 0 {
                    continue;
                }
                let mut v = conhecer(read_usize(s + 0x69), nivel) as i8;
                match escopo(p, s) as i8 {
                    3 => {
                        if nivel == 6 && read_i16(clube + club::REPUTATION) < 0xdac {
                            continue;
                        }
                        v = v.wrapping_sub(5);
                    }
                    4 => {
                        if nivel == 6 && read_i16(clube + club::REPUTATION) < 0xdac {
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
        let transferencia: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_TRANSFER_LISTED_OR_REQUESTED);
        if clube != read_usize(p + 0x39) && transferencia(p, 0) != 0 {
            crate::comparacao::ramo("cqa_7 listado para transferencia");
            *saida = ftol(*saida as f64 * read_f64(0x9689b8)); // cqa7: o listado de outro clube × 1,05
        }
    }
    let r = *saida;
    if r < 0 || r < minimo.wrapping_sub(10000) || read_u8(vaga + 0x24) & 1 != 0 {
        return cqa7_sem_valor(p, vaga, saida);
    }
    let nevoa = read_u8(read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(p).wrapping_mul(0x4f)) + 0xb) & 0xf;
    if nevoa != 0 {
        let a = r.wrapping_add(5000) as f64;
        let w = (pow_do_jogo(nevoa as f64, read_f64(0x968950)) * read_f64(0x967898) + read_f64(0x967890)) * r as f64;
        *saida = ftol(if a < w { a } else { w });
    }
    div = 0x1c;
    let posicao: unsafe extern "thiscall" fn(usize, u32) -> u8 = core::mem::transmute(FN_SHORTLIST_POSITION_MASK_TO_INDEX);
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
    let bv = read_usize(vaga + 0xb) as i32;
    let alta = r >= 35000 && bv >= r / 8 && read_i16(vaga + 1) >= 0x80;
    let mut e = if alta {
        let v = r as f64;
        ftol(((v * read_f64(0x96d238) - read_f64(0x967890)) * v + bv as f64 * read_f64(0x9689e0)) * read_f64(0x96f918)) as u32
    } else {
        crate::comparacao::ramo("cqa_7 valor base");
        ftol((bv as f64 * read_f64(0x9689e0) + r as f64 * read_f64(0x969040)) * read_f64(0x96f918)) as u32 // cqa7: o valor base
    };
    if e < 1 {
        e = 1;
    }
    let idade = read_u8(p + 0x18) as i8;
    match (read_u8(vaga + 0x24) >> 1) & 0xf {
        1 | 2 => cqa7_jovem(this, clube, p, saida, minimo, alta, idade, r, &mut frac, &mut e),
        3 => {
            let k = if read_i16(vaga + 1) < 0x80 { read_f64(0x96a670) } else { read_f64(0x96d718) };
            e = ftol(e as f64 * k) as u32;
            cqa7_jovem(this, clube, p, saida, minimo, alta, idade, r, &mut frac, &mut e);
        }
        4 => e = ftol(e as f64 * read_f64(0x968968)) as u32,
        5 => {
            e = ftol(e as f64 * read_f64(0x968a80)) as u32;
            div = div.wrapping_sub(9);
        }
        6 => e = ftol(e as f64 * read_f64(0x968918)) as u32,
        _ => {}
    }
    let tipo = read_u8(vaga + 0x24) & 0x1e;
    if idade > div {
        let d = idade as i32 - div as i32;
        let dd = d as f64;
        let ficha = read_usize(p + 0x61);
        let f = read_f64(0x967890) / (dd * read_f64(0x968e80) + read_f64(0x9689b0)); // cqa7: a idade acima do divisor
        let ts = read_i16(ficha + 0xd);
        let g = if ts > 0x1770 { ts as i32 as f64 * read_f64(0x96a680) - read_f64(0x968940) } else { 0.0 };
        let estrangeira: unsafe extern "C" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_DATABASE_IS_STAFF_FOREIGN);
        let y = if estrangeira(p, read_usize(clube + club::NATION), 1) != 0 {
            f / (dd * read_f64(0x968f20) + read_f64(0x9689d8))
        } else if idade as i32 > div as i32 + 2 && (tipo == 8 || tipo == 6) && read_i16(ficha + 0xb) < read_i16(clube + club::REPUTATION) {
            (read_f64(0x967890) - (d - 2) as f64 * read_f64(0x96d438)) * f
        } else {
            f
        };
        crate::comparacao::ramo("cqa_7 idade");
        let um = read_f64(0x967890);
        if tipo == 0xa {
            *saida = ftol(((um - g) * y + g) * *saida as f64);
        }
        e = ftol(((um - g) * y + g) * e as f64) as u32;
    }
    let ficha = read_usize(p + 0x61);
    let cons = read_u8(ficha + 0x2d) as i8 as f64;
    let c = ftol((read_f64(0x967890) - pow_do_jogo(cons, read_f64(0x968950)) * read_f64(0x968f40)) * e as f64);
    let mut kk = (c.wrapping_mul(c) as u32) as f64 * teto_pos * read_f64(0x968eb8);
    let ra = read_i16(clube + club::REPUTATION);
    if tipo == 0xc {
        if ra > 5000 {
            let lim = (ra as i32 - 5000) as f64 * read_f64(0x968ea8) + read_f64(0x968880);
            if !(kk <= lim) {
                kk = (ra as i32 - 100000) as f64 + read_f64(0x968880);
            }
        } else {
            let lim = (ra as i32 * 10) as f64;
            if !(kk <= lim) {
                kk = ra as f64 * read_f64(0x9688b0);
            }
        }
    } else if tipo == 0xa {
        let lim = if ra > 5000 {
            let z = (ra as i32 - 5000) as f64 * read_f64(0x968918) + read_f64(0x968878);
            z * z
        } else {
            let z = ra as f64;
            z * z * read_f64(0x96f910)
        };
        if !(kk <= lim) {
            kk = lim;
        }
    }
    let ts = read_i16(ficha + 0xd);
    if ts <= 5000 {
        let lim = frac * read_f64(0x96f908);
        if lim < kk {
            kk = lim;
        }
    } else if ts <= 8000 {
        let lim = ts as f64 * frac * read_f64(0x96f900) + read_f64(0x96f8f8);
        if lim < kk {
            kk = lim;
        }
    } else if ts < 9000 {
        let lim = (ts as f64 * read_f64(0x9690b8) - read_f64(0x96f8f0)) * frac;
        if lim < kk {
            kk = lim;
        }
        frac = frac * read_f64(0x968940);
    } else {
        kk = read_f64(0x96d568);
        frac = frac * read_f64(0x968e18);
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
        let t = frac * read_f64(a);
        if t < kk {
            kk = t;
        }
    }
    let base = read_f64(0x96f8d0);
    let mut x = kk;
    if kk > base {
        let vista: unsafe extern "C" fn(usize, usize, usize, u32, u32) -> i16 = core::mem::transmute(FN_MATCH_MAN_JUDGE_PLAYER_ABILITY);
        let n2 = vista(read_usize(clube + club::MANAGER), p, clube, 2, (*saida / 4) as u32) as i32 as f64;
        if n2 < read_f64(0x96f8c8) {
            let mut a = n2 * read_f64(0x967878) - read_f64(0x96f8c0);
            if !(read_f64(0x9689a0) <= a) {
                a = read_f64(0x9689a0);
            }
            x = (kk - base) * a + base;
        }
    }
    let bc = read_usize(clube + club::DIVISION);
    if bc != 0 && read_i16(bc + 0x69) < 0x10 {
        let b69 = read_i16(bc + 0x69) as i32;
        let ri = read_i16(clube + club::REPUTATION) as i32 / 500;
        let m = if ri > b69 { ri } else { b69 };
        let mut lim = m.wrapping_mul(670000);
        if lim <= 1000000 {
            lim = 1000000;
        }
        let ts = read_i16(ficha + 0xd) as i32;
        if ts > 8000 {
            lim = lim.wrapping_add((ts - 8000) * 10000);
        }
        let l = lim as f64;
        if !(x <= l) {
            let m2 = if read_f64(0x967890) < frac { read_f64(0x967890) } else { frac };
            x = ftol(l * m2) as f64;
        }
    }
    // a nação do clube (+0x53 do clube: a original usa edi, que aqui já é o clube)
    let nacao = read_usize(read_usize(clube + club::NATION));
    if nacao == read_usize(KEY_NATION_PORTUGAL) {
        if x < read_f64(0x968e90) {
            x = x * (x * read_f64(0x968e88) * read_f64(0x968e48) + read_f64(0x968e80));
        } else {
            x = x * read_f64(0x9689d8);
        }
    } else if nacao == read_usize(KEY_NATION_FRANCE) {
        x = ftol(x * read_f64(0x9689b0)) as f64;
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
pub(crate) unsafe extern "thiscall" fn comparar_value_player_for_club(this: usize, clube: usize, p: usize, vaga: usize, saida: *mut i32, minimo: i32, teto: u32) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut i32, i32, u32) -> i32>(IDX_VALUE_PLAYER_FOR_CLUB);
    let obs = read_usize(clube + club::MANAGER);
    if obs != 0 && read_usize(obs + 0x69) == 0 {
        return o(this, clube, p, vaga, saida, minimo, teto);
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(saida as usize, 4)], || o(this, clube, p, vaga, saida, minimo, teto), || value_player_for_club(this, clube, p, vaga, saida, minimo, teto));
    PLACAR.registrar(igual && r == n, || {
        format!("value_player_for_club(clube {:x}, pessoa {:x}, vaga {:x}, mínimo {}, teto {}) original={} rust={} {:x?} x {:x?}", clube, p, vaga, minimo, teto, r, n, mo, mr)
    });
    r
}





// ---------- 261a leva: as vagas da lista de observação com lance, e as marcas de lista de transferências e de empréstimos do contrato (shortlist_manager.cpp) ----------




/// O contrato principal da pessoa (CONTRACT_MANAGER::get_contract 0x4cf5a0: thiscall([0xae1050], pessoa), ret 4).
/// As três marcas não guardam o contrato em [0xae1080].
pub(crate) unsafe fn cpy1_contrato(pessoa: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    f(read_usize(G_CONTRACT_MANAGER), pessoa)
}

/// As datas estáticas da 0x54c5b0, da 0x54c8e0 e da 0x54cdb0 (valor e guarda).
pub(crate) const CPY1_ESTATICAS: [(usize, usize); 6] = [(0xae2420, 8), (0xae2469, 1), (0xae24c0, 8), (0xae249e, 1), (0xae2470, 8), (0xae2418, 1)];


/// counts the club's shortlist slots with a target and a bid at the given position (0xff: any); with the list in state
/// 2 for a human club, a slot whose manager arrived over 20 days ago and whose target has no offers loses its target
/// and bid
/// Original: `Shortlist_Manager::count_slots_with_bids` (0x844540, shortlist_manager.cpp, thiscall).
///
/// [pt] { 0x844540 } (thiscall, ret 8, shortlist_manager.cpp): `Shortlist_Manager::count_slots_with_bids(clube, posicao)`,
/// pela avaliação das listas (0x84a020, com 0xb), pela avaliação do elenco (0x83cb40, com −1) e pelo laço do dia
/// (0x5cccd0, com −1). Das 40 vagas da lista do clube, conta as que têm alvo (+0xf) e lance (+0x17), as duas acima de
/// −1, e posição (+0) igual a `posicao` (0xff: qualquer). Com a lista no estado 2 (+4 da entrada) e o clube do humano
/// (0x600a50 com 1, 1): se hoje − 20 dias (0x54c8e0) passa da data de chegada do técnico (+0x3e da ficha em +0xcf do
/// clube) e o alvo não tem ofertas (0x8c5cf0), a vaga perde o alvo e o lance (−1) e não conta. A conta é um byte.
pub(crate) unsafe extern "thiscall" fn count_slots_with_bids(this: usize, clube: usize, posicao: u32) -> u8 {
    // 0x600a50: cdecl(clube, 1, 1, 0, 0) (add esp, 0x14 depois); 0x54c8e0: thiscall(0xae2c90, saída, 0x14); 0x8c5cf0:
    // thiscall([0xde1f54], pessoa, 0, 0)
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let menos_dias: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(FN_DATE_SUB_DAYS);
    let ofertas: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_OFFERS_FOR_PLAYER);
    let entrada = cpp5_entrada(this, clube);
    let mut n: u8 = 0;
    for i in 0..40usize {
        let v = read_usize(entrada).wrapping_add(i * 0x25);
        if (read_usize(v + 0xf) as i32) <= -1 {
            continue;
        }
        if (read_usize(v + 0x17) as i32) <= -1 {
            crate::comparacao::ramo("cpy_1 alvo sem lance");
            continue; // cpy1: alvo sem lance não conta
        }
        if posicao as u8 != 0xff && read_u8(v) != posicao as u8 {
            crate::comparacao::ramo("cpy_1 outra posicao");
            continue; // cpy1: lance de outra posição não conta
        }
        if read_u8(entrada + 4) == 2 && humano(clube, 1, 1, 0, 0) != 0 {
            let mut buf = [0u8; 8];
            let r = menos_dias(0xae2c90, &mut buf, 0x14);
            let dia = core::ptr::read_unaligned(r as *const i16);
            let ano = read_u16(r + 2);
            let chegada = read_usize(clube + club::MANAGER).wrapping_add(0x3e);
            let ano_c = read_u16(chegada + 2);
            let velho = ano > ano_c || (ano == ano_c && dia > core::ptr::read_unaligned(chegada as *const i16));
            if velho {
                let alvo = read_usize(G_STAFF_TABLE).wrapping_add(read_usize(v + 0xf).wrapping_mul(0x6e));
                if ofertas(read_usize(G_TRANSFER_MANAGER), alvo, 0, 0) == 0 {
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
pub(crate) unsafe extern "thiscall" fn comparar_count_slots_with_bids(this: usize, clube: usize, posicao: u32) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u8>(IDX_COUNT_SLOTS_WITH_BIDS);
    let mut reg = CPY1_ESTATICAS.to_vec();
    reg.push((read_usize(cpp5_entrada(this, clube)), 40 * 0x25));
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube, posicao), || count_slots_with_bids(this, clube, posicao));
    PLACAR.registrar(igual && r == n, || format!("count_slots_with_bids(clube {:x}, posicao {}) original={} rust={} {:x?} x {:x?}", clube, posicao as i8, r, n, mo, mr));
    r
}

/// clears bit 0 (transfer listed) of the contract's +0x4e; 1 with a contract, 0 without
/// Original: `shortlist_manager_remove_from_transfer_list` (0x844280, shortlist_manager.cpp, stdcall).
///
/// [pt] { 0x844280 } (stdcall, ret 4, shortlist_manager.cpp): `shortlist_manager_remove_from_transfer_list(pessoa)`, pelas
/// telas de transferência (0x8f6770, 0x8f8120). Sem contrato (0x4cf5a0): 0. Com ele, o bit 0 de +0x4e (na lista de
/// transferências) sai e devolve 1 no byte baixo (os de cima são os do ponteiro do contrato, como a original).
pub(crate) unsafe extern "stdcall" fn shortlist_manager_remove_from_transfer_list(pessoa: usize) -> u32 {
    let c = cpy1_contrato(pessoa);
    if c == 0 {
        return 0;
    }
    *((c + 0x4e) as *mut u8) &= 0xfe;
    (c as u32 & !0xff) | 1
}

/// Comparador: lado a lado pelo byte +0x4e do contrato, e a devolução.
pub(crate) unsafe extern "stdcall" fn comparar_shortlist_manager_remove_from_transfer_list(pessoa: usize) -> u32 {
    let o = orig::<unsafe extern "stdcall" fn(usize) -> u32>(IDX_SHORTLIST_MANAGER_REMOVE_FROM_TRANSFER_LIST);
    let c = cpy1_contrato(pessoa);
    let reg = if c != 0 { vec![(c + 0x4e, 1)] } else { Vec::new() };
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(pessoa), || shortlist_manager_remove_from_transfer_list(pessoa));
    PLACAR.registrar(igual && r == n, || format!("shortlist_manager_remove_from_transfer_list(pessoa {:x}) original={:#x} rust={:#x} {:x?} x {:x?}", pessoa, r, n, mo, mr));
    r
}

/// clears bit 1 (loan listed) of the contract's +0x4e; 1 with a contract, 0 without
/// Original: `shortlist_manager_remove_from_loan_list` (0x8442d0, shortlist_manager.cpp, stdcall).
///
/// [pt] { 0x8442d0 } (stdcall, ret 4, shortlist_manager.cpp): `shortlist_manager_remove_from_loan_list(pessoa)`, pela
/// renovação do contrato (0x4dc3a0) e pelas telas de transferência. Sem contrato: 0; com ele, o bit 1 de +0x4e (na
/// lista de empréstimos) sai e devolve 1 no byte baixo (os de cima, os do ponteiro do contrato).
pub(crate) unsafe extern "stdcall" fn shortlist_manager_remove_from_loan_list(pessoa: usize) -> u32 {
    let c = cpy1_contrato(pessoa);
    if c == 0 {
        return 0;
    }
    *((c + 0x4e) as *mut u8) &= 0xfd;
    (c as u32 & !0xff) | 1
}

/// Comparador: lado a lado pelo byte +0x4e do contrato, e a devolução.
pub(crate) unsafe extern "stdcall" fn comparar_shortlist_manager_remove_from_loan_list(pessoa: usize) -> u32 {
    let o = orig::<unsafe extern "stdcall" fn(usize) -> u32>(IDX_SHORTLIST_MANAGER_REMOVE_FROM_LOAN_LIST);
    let c = cpy1_contrato(pessoa);
    let reg = if c != 0 { vec![(c + 0x4e, 1)] } else { Vec::new() };
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(pessoa), || shortlist_manager_remove_from_loan_list(pessoa));
    PLACAR.registrar(igual && r == n, || format!("shortlist_manager_remove_from_loan_list(pessoa {:x}) original={:#x} rust={:#x} {:x?} x {:x?}", pessoa, r, n, mo, mr));
    r
}

// ---------- 261a leva: a situação da pessoa nas listas de observação dos clubes, a marca de lista de empréstimos e o teto de preço pelo estatuto (shortlist_manager.cpp) ----------






/// As datas estáticas da 0x54c5b0, da 0x54c8e0 e da 0x54cdb0 (valor e guarda).
pub(crate) const CPY2_ESTATICAS: [(usize, usize); 6] = [(0xae2420, 8), (0xae2469, 1), (0xae24c0, 8), (0xae249e, 1), (0xae2470, 8), (0xae2418, 1)];

/// Tira o par (pessoa, clube) de um dos vetores ordenados de pares (8 bytes) do Shortlist_Manager (`base` = +0 ou +4,
/// conta em `base` + 0x10) pela busca binária (0x946464: cdecl(chave, vetor, conta, 8, comparação), add esp, 0x14):
/// os de depois descem uma casa, o último fica zerado e a conta cai. Devolve se achou.
pub(crate) unsafe fn cpy2_tirar_par(base: usize, chave: &[u32; 2], comparar: usize) -> bool {
    let bsearch: unsafe extern "C" fn(*const u32, usize, u32, u32, usize) -> usize = core::mem::transmute(FN_CRT_BSEARCH);
    let p = bsearch(chave.as_ptr(), read_usize(base), read_usize(base + 0x10) as u32, 8, comparar);
    if p == 0 {
        return false;
    }
    let mut i = (p.wrapping_sub(read_usize(base)) >> 3) as i32;
    while i < read_usize(base + 0x10) as i32 - 1 {
        let v = read_usize(base);
        i += 1;
        let k = v.wrapping_add(i as usize * 8);
        write_u32(k - 8, read_usize(k) as u32);
        write_u32(k - 4, read_usize(k + 4) as u32);
    }
    let fim = read_usize(base).wrapping_add(read_usize(base + 0x10).wrapping_mul(8));
    write_u32(fim - 4, 0);
    write_u32(fim - 8, 0);
    write_u32(base + 0x10, (read_usize(base + 0x10) as u32).wrapping_sub(1));
    true
}

/// A data de hoje mais (0x54c5b0) ou menos (0x54c8e0) `dias` (thiscall(0xae2c90, saída, dias), ret 8) copiada para a
/// data da vaga em +0x1b (0x417c10: thiscall(destino, saída, origem), ret 8).
pub(crate) unsafe fn cpy2_data_da_vaga(v: usize, rotina: usize, dias: u32) {
    let deslocar: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(rotina);
    let copiar: unsafe extern "thiscall" fn(usize, *mut [u8; 8], usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let mut a = [0u8; 8];
    let mut b = [0u8; 8];
    let r = deslocar(0xae2c90, &mut a, dias);
    copiar(v + 0x1b, &mut b, r);
}

/// O par (pessoa, clube) do evento 2 ou 5: a vaga guarda a pessoa em +0x13 (bloqueada até a data de +0x1b), sem lance
/// (+0x17 = −1) e sem valores (+0xb, +7); o estatuto da vaga (bits 1..4 de +0x24) vai ao nibble alto de +0x4f do
/// contrato da pessoa no clube; a entrada fica com 100 em +8.
pub(crate) unsafe fn cpy2_bloquear(v: usize, entrada: usize, pessoa: usize, contrato: usize, rotina: usize, dias: u32) {
    cpy2_data_da_vaga(v, rotina, dias);
    write_u32(v + 0x13, read_usize(pessoa) as u32);
    write_u32(v + 0x17, 0xffff_ffff);
    write_u32(v + 0xb, 0);
    write_u32(v + 7, 0);
    if contrato != 0 {
        let c = (contrato + contract::SQUAD_STATUS) as *mut u8;
        *c = (*c & 0xf) | (((read_u8(v + 0x24) >> 1) & 0xf) << 4);
    }
    *((entrada + 8) as *mut u8) = 0x64;
}

/// applies a transfer event for the person to the club's shortlist: for event 4 at his own club, event 1 to every
/// interested club; the (person, club) pair leaves the sorted arrays and the person's shortlist count drops; in the
/// slots holding the person, 1 refills the slot, 2 and 5 block for 120 and 14 days, 3 and 4 release with today − 120
/// Original: `Shortlist_Manager::update_staff_status` (0x843e10, shortlist_manager.cpp, thiscall).
///
/// [pt] { 0x843e10 } (thiscall, ret 0x10, shortlist_manager.cpp, quadro SEH da LLIST local): `Shortlist_Manager::
/// update_staff_status(pessoa, clube, evento, tipo)`, pelas ofertas, contratos, técnicos e regens (0x4d07e0, 0x690730,
/// 0x7a59e0, 0x8bf680, 0x8c44e0...) e por ela mesma. Com o evento 4 e o clube nulo ou o da pessoa (+0x39): cada clube
/// interessado (0x842180 com 0, 3, 0) recebe o evento 1 com o tipo 2 (recursão). Clube nulo: 1. Com o evento 5, ou 1..3
/// de outro clube: o par (pessoa, clube) sai dos dois vetores ordenados (+0/+0x10 pela 0x757ea0, +4/+0x14 pela
/// 0x844740; vetor nulo: 0); achado nos dois e o clube não humano (0x600a50 com 0, 1), o nibble baixo de +0xb do
/// registro de 79 bytes da pessoa ([0xae2d0c]) cai um. Depois, nas 40 vagas do clube (tipo 1: fora as de jogador 0xb;
/// tipo 0: só elas) com a pessoa no alvo (+0xf; no evento 4, também em +0x13): 1 = a vaga se esvazia e é preenchida de
/// novo (0x83f3a0); 2 e 5 = bloqueio de 120 e 14 dias (0x54c5b0); 3 = esvazia, data de hoje − 120 (0x54c8e0); 4 =
/// solta +0x13 e o alvo, data de hoje − 120. Devolve 1.
pub(crate) unsafe extern "thiscall" fn update_staff_status(this: usize, pessoa: usize, clube: usize, evento: u32, tipo: u32) -> u8 {
    // 0x68d020/0x68d040/0x68d1b0: thiscall(lista); 0x842180: thiscall(this, pessoa, lista, 0, 3, 0) (5 push); a
    // recursão: thiscall(this, pessoa, clube, 1, 2) (4 push); 0x600a50: cdecl(clube, 0, 1, 0, 0) (add esp, 0x14);
    // 0x4cf5f0: thiscall([0xae1050], pessoa, clube) (2 push); 0x83f3a0: thiscall(this, vaga, clube, pessoa) (3 push)
    let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let interessados: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) -> u32 = core::mem::transmute(FN_SHORTLIST_MANAGER_COLLECT_INTERESTED_CLUBS);
    let situacao: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u8 = core::mem::transmute(FN_SHORTLIST_MANAGER_UPDATE_STAFF_STATUS);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let contract_of: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let preencher: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(FN_SHORTLIST_MANAGER_FILL_SHORTLIST_SLOT);
    let mut lista = [0u8; 0x10]; // a LLIST local (0xe bytes)
    let l = lista.as_mut_ptr() as usize;
    lista_nova(l);
    let ev = evento as u8;
    let id = read_usize(pessoa);
    if (clube == 0 || clube == read_usize(pessoa + staff::CLUB_JOB)) && ev == 4 {
        interessados(this, pessoa, l, 0, 3, 0);
        core::ptr::write_unaligned((l + 0xa) as *mut usize, read_usize(l + 2));
        let mut e = proximo(l);
        while e != 0 {
            situacao(this, pessoa, cpe1_clube(read_usize(e + 4)), 1, 2);
            e = proximo(l);
        }
    }
    if clube == 0 {
        destruir(l);
        return 1;
    }
    if ev == 5 || (ev != 4 && clube != read_usize(pessoa + staff::CLUB_JOB)) {
        if read_usize(this) == 0 || read_usize(this + 4) == 0 {
            destruir(l);
            return 0;
        }
        let chave = [id as u32, read_usize(clube) as u32];
        let mut faltou = !cpy2_tirar_par(this, &chave, 0x757ea0);
        faltou |= !cpy2_tirar_par(this + 4, &chave, 0x844740);
        if humano(clube, 0, 1, 0, 0) == 0 && !faltou {
            crate::comparacao::ramo("cpy_2 contagem da lista");
            let r = (read_usize(G_STAFF_STATE_TABLE).wrapping_add(id.wrapping_mul(0x4f)) + 0xb) as *mut u8;
            *r = (*r & 0xf0) | ((*r).wrapping_sub(1) & 0xf); // cpy2: uma lista a menos na conta da pessoa
        }
    }
    let entrada = cpp5_entrada(this, clube);
    let contrato = contract_of(read_usize(G_CONTRACT_MANAGER), pessoa, clube);
    for i in 0..40usize {
        let v = read_usize(entrada).wrapping_add(i * 0x25);
        let t = tipo as u8;
        if t == 1 {
            if read_u8(v) == 0xb {
                continue;
            }
        } else if t == 0 && read_u8(v) != 0xb {
            continue;
        }
        if read_usize(v + 0xf) != id && (ev != 4 || read_usize(v + 0x13) != id) {
            continue;
        }
        match ev {
            1 => {
                write_u32(v + 0xf, 0xffff_ffff);
                write_u32(v + 0x17, 0xffff_ffff);
                if read_usize(v + 0x13) == id {
                    write_u32(v + 0x13, 0xffff_ffff);
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
                write_u32(v + 0xf, 0xffff_ffff);
                write_u32(v + 0x17, 0xffff_ffff);
                cpy2_data_da_vaga(v, 0x54c8e0, 0x78);
                if read_usize(v + 0x13) == id {
                    write_u32(v + 0x13, 0xffff_ffff);
                }
            }
            4 => {
                if read_usize(v + 0x13) == id {
                    write_u32(v + 0x13, 0xffff_ffff);
                }
                if read_usize(v + 0xf) == id {
                    write_u32(v + 0xf, 0xffff_ffff);
                    write_u32(v + 0x17, 0xffff_ffff);
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
pub(crate) unsafe fn cpy2_so_a_original(this: usize, pessoa: usize, clube: usize, evento: u32, tipo: u32) -> bool {
    let ev = evento as u8;
    if ev == 4 && (clube == 0 || clube == read_usize(pessoa + staff::CLUB_JOB)) {
        return true;
    }
    if ev != 1 || clube == 0 {
        return false;
    }
    // de outro clube, o evento 1 passa pelos vetores; um deles nulo devolve 0 antes das vagas
    if clube != read_usize(pessoa + staff::CLUB_JOB) && (read_usize(this) == 0 || read_usize(this + 4) == 0) {
        return false;
    }
    let entrada = cpp5_entrada(this, clube);
    (0..40usize).any(|i| {
        let v = read_usize(entrada).wrapping_add(i * 0x25);
        let ok = match tipo as u8 {
            1 => read_u8(v) != 0xb,
            0 => read_u8(v) == 0xb,
            _ => true,
        };
        ok && read_usize(v + 0xf) == read_usize(pessoa)
    })
}

/// Comparador: lado a lado pelos dois vetores de pares e suas contas, pelo byte da conta de listas da pessoa, pelas 40
/// vagas e o +8 da entrada do clube, pelo byte +0x4f do contrato e pelas datas estáticas, e a devolução. Na recursão do
/// evento 4 e no evento 1 que preenche a vaga, só a original (a cópia leva os ramos).
pub(crate) unsafe extern "thiscall" fn comparar_update_staff_status(this: usize, pessoa: usize, clube: usize, evento: u32, tipo: u32) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u8>(IDX_UPDATE_STAFF_STATUS);
    if cpy2_so_a_original(this, pessoa, clube, evento, tipo) {
        return o(this, pessoa, clube, evento, tipo);
    }
    let mut reg = CPY2_ESTATICAS.to_vec();
    for base in [this, this + 4] {
        reg.push((base + 0x10, 4));
        if read_usize(base) != 0 {
            reg.push((read_usize(base), read_usize(base + 0x10).wrapping_mul(8)));
        }
    }
    reg.push((read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(pessoa).wrapping_mul(0x4f)) + 0xb, 1));
    if clube != 0 {
        let entrada = cpp5_entrada(this, clube);
        reg.push((read_usize(entrada), 40 * 0x25));
        reg.push((entrada + 8, 1));
        let contract_of: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
        let c = contract_of(read_usize(G_CONTRACT_MANAGER), pessoa, clube);
        if c != 0 {
            reg.push((c + 0x4f, 1));
        }
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, pessoa, clube, evento, tipo),
        || update_staff_status(this, pessoa, clube, evento, tipo));
    PLACAR.registrar(igual && r == n, || format!("update_staff_status(pessoa {:x}, clube {:x}, evento {}, tipo {}) original={} rust={} {:x?} x {:x?}",
        pessoa, clube, evento as u8, tipo as u8, r, n, mo, mr));
    r
}

/// sets bit 1 (loan listed) of the person's contract +0x4e
/// Original: `shortlist_manager_add_to_loan_list` (0x8442b0, shortlist_manager.cpp, stdcall).
///
/// [pt] { 0x8442b0 } (stdcall, ret 4, shortlist_manager.cpp): `shortlist_manager_add_to_loan_list(pessoa)`, pelas telas de
/// transferência (0x8f6770, 0x8f8120). Com contrato (0x4cf5a0, sem guardar em [0xae1080]), o bit 1 de +0x4e (na lista
/// de empréstimos) liga. Devolve o eax que a original deixa: o contrato.
pub(crate) unsafe extern "stdcall" fn shortlist_manager_add_to_loan_list(pessoa: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = f(read_usize(G_CONTRACT_MANAGER), pessoa);
    if c != 0 {
        *((c + 0x4e) as *mut u8) |= 2;
    }
    c
}

/// Comparador: lado a lado pelo byte +0x4e do contrato, e a devolução.
pub(crate) unsafe extern "stdcall" fn comparar_shortlist_manager_add_to_loan_list(pessoa: usize) -> usize {
    let o = orig::<unsafe extern "stdcall" fn(usize) -> usize>(IDX_SHORTLIST_MANAGER_ADD_TO_LOAN_LIST);
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = f(read_usize(G_CONTRACT_MANAGER), pessoa);
    let reg = if c != 0 { vec![(c + 0x4e, 1)] } else { Vec::new() };
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(pessoa), || shortlist_manager_add_to_loan_list(pessoa));
    PLACAR.registrar(igual && r == n, || format!("shortlist_manager_add_to_loan_list(pessoa {:x}) original={:#x} rust={:#x} {:x?} x {:x?}", pessoa, r, n, mo, mr));
    r
}

/// the club's fee limit for the player at the squad status (0x84b230 with his strong positions); −1 without a person or
/// player record
/// Original: `shortlist_manager_max_fee_for_player_status` (0x84b1f0, shortlist_manager.cpp, thiscall).
///
/// [pt] { 0x84b1f0 } (thiscall, ret 0xc, shortlist_manager.cpp): `shortlist_manager_max_fee_for_player_status(clube, pessoa,
/// estatuto)`, pela tela de contrato (0x4dd0f0, ecx = o Shortlist_Manager em +0x856 da gerência de transferências).
/// Sem pessoa ou sem ficha de jogador (+0x61): −1. Senão o teto da vaga (0x84b230) com as posições fortes do jogador
/// (0x5b7d20) e o estatuto.
pub(crate) unsafe extern "thiscall" fn shortlist_manager_max_fee_for_player_status(this: usize, clube: usize, pessoa: usize, estatuto: u32) -> i32 {
    // 0x5b7d20: cdecl(ficha) (add esp, 4); 0x84b230: chamada com mov ecx, esi (o this) e 3 push (clube, posições,
    // estatuto), ret 0xc: thiscall daqui (a cópia stdcall do motor não lê o ecx)
    if pessoa == 0 || read_usize(pessoa + staff::PLAYER) == 0 {
        return -1;
    }
    let posicoes: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_FORMATION_GET_POSITION_MASK);
    let teto: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> i32 = core::mem::transmute(FN_SHORTLIST_MANAGER_MAX_FEE_FOR_SQUAD_STATUS);
    let m = posicoes(read_usize(pessoa + staff::PLAYER));
    teto(this, clube, m, estatuto)
}

/// Comparador: a devolução (rotina pura).
pub(crate) unsafe extern "thiscall" fn comparar_shortlist_manager_max_fee_for_player_status(this: usize, clube: usize, pessoa: usize, estatuto: u32) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32) -> i32>(IDX_SHORTLIST_MANAGER_MAX_FEE_FOR_PLAYER_STATUS);
    let r = o(this, clube, pessoa, estatuto);
    let n = shortlist_manager_max_fee_for_player_status(this, clube, pessoa, estatuto);
    PLACAR.registrar(r == n, || format!("shortlist_manager_max_fee_for_player_status(clube {:x}, pessoa {:x}, estatuto {}) original={} rust={}", clube, pessoa, estatuto as i8, r, n));
    r
}

// ---------- 261a leva: a verba de transferências repartida pelas vagas da lista de observação, o estatuto permitido pela idade e o nome das listas do olheiro (shortlist_manager.cpp, scout_manager.cpp) ----------






/// A pessoa de índice `i` ([0xae23bc], 0x6e bytes; a conta da original em 32 bits, com índice negativo inclusive).
pub(crate) unsafe fn cpy3_pessoa(i: i32) -> usize {
    read_usize(G_STAFF_TABLE).wrapping_add((i as usize).wrapping_mul(0x6e))
}

/// shares an AI club's transfer budget across the open shortlist slots (type and status factors), reduces it by held
/// bids and the remainder, fills each slot without a target and gives the remainder to the first own-club player slot
/// the manager would sell
/// Original: `Shortlist_Manager::allocate_transfer_budget` (0x83b6e0, shortlist_manager.cpp, thiscall).
///
/// [pt] { 0x83b6e0 } (thiscall, ret 4, shortlist_manager.cpp): `Shortlist_Manager::allocate_transfer_budget(clube)`, pela
/// atualização da lista do clube (0x83a730, já em Rust), pelas listas iniciais (0x83a1b0) e pela lista refeita
/// (0x84abe0). Clube do humano (0x600a50 com 0, 1), seleção (0x53b900), sem finanças (0x5a0590) ou lista fora do estado
/// 1: nada. A verba (0x598800 com 0, 0, 0, 1, 0; negativa: 0) se reparte pelas vagas abertas (alvo −1) depois de
/// ordenar as 40 (qsort 0x83b690): entre as primeiras 8 (ou até a quarta vaga de jogador aberta), cada vaga aberta
/// recebe em +7 a verba / (abertas × fator do tipo e do estatuto); o resto da verba passa pelas vagas ainda sem alvo
/// (reduções pelos lances guardados, pelo valor de +0xb sobre a soma e pelos fatores de 0x969540, 0x968968), cada uma
/// preenchida pela 0x83f3a0; o que sobra vai à primeira vaga de jogador do próprio clube que o técnico aceitaria vender
/// (0x846920 acima de 0x968e58, sorteios de 10 e 100 dias, ofertas 0x8c5cf0).
pub(crate) unsafe extern "thiscall" fn allocate_transfer_budget(this: usize, clube: usize) {
    // 0x600a50: cdecl(clube, 0, 1, 0, 0) (add esp, 0x14); 0x53b900: cdecl(clube) (add esp, 4); 0x5a0590:
    // thiscall([0xae2a38], clube); 0x598800: thiscall(finanças, 0, 0, 0, 1, 0) (5 push); 0x945333: cdecl(vetor, 0x28,
    // 0x25, 0x83b690) (add esp, 0x10); 0x8c61e0: thiscall([0xde1f54], lance) (1 push); 0x83f3a0: thiscall(this, vaga,
    // clube, 0) (mov ecx, [this local]; 3 push); 0x4cf5a0: thiscall([0xae1050], pessoa); 0x544590: cdecl(pessoa, 0)
    // (add esp, 8); 0x4cf5f0: thiscall([0xae1050], pessoa, clube) (2 push); 0x846920: thiscall(this, pessoa, clube, 1, 0,
    // estatuto, 0) (6 push, devolve em st0); 0x90cfc0: cdecl(n) (add esp, 4); 0x85fa00: thiscall(contrato) (sem push,
    // movsx ax); 0x8c5cf0: thiscall([0xde1f54], pessoa, 0, 0) (3 push); 0x54c8e0: thiscall(0xae2c90, saída, dias)
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FINANCETABLE_GET_CLUB_FINANCE);
    let verba_de: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_FINANCE_GET_TRANSFER_BUDGET);
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    let lance_de: unsafe extern "thiscall" fn(usize, i32) -> usize = core::mem::transmute(FN_TRANSFER_MANAGER_GET_OFFER_SLOT);
    let preencher: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(FN_SHORTLIST_MANAGER_FILL_SHORTLIST_SLOT);
    if humano(clube, 0, 1, 0, 0) != 0 || selecao(clube) != 0 {
        return;
    }
    let fin = financas(read_usize(G_FINANCE_TABLE), clube);
    if fin == 0 {
        return;
    }
    let mut verba = verba_de(fin, 0, 0, 0, 1, 0) as i32;
    if verba < 0 {
        verba = 0;
    }
    let entrada = read_usize(this + 0xdc).wrapping_add(read_usize(clube).wrapping_mul(9));
    if read_u8(entrada + 4) != 1 {
        return;
    }
    ordenar(read_usize(entrada), 0x28, 0x25, 0x83b690);
    let vaga = |i: i32| read_usize(entrada).wrapping_add((i as usize).wrapping_mul(0x25));
    // as vagas abertas entre as primeiras 8, ou até a quarta vaga de jogador aberta
    let mut limite: i8 = 8;
    let mut abertas: i8 = 0;
    let mut jogadores: u8 = 0;
    let mut soma: i32 = 0;
    let mut i: i8 = 0;
    loop {
        let v = vaga(i as i32);
        if read_i32(v + 0xf) == -1 {
            if read_u8(v) == 0xb {
                jogadores = jogadores.wrapping_add(1);
                if jogadores == 4 {
                    limite = i.wrapping_add(1);
                }
            }
            soma = soma.wrapping_add(read_i32(v + 0xb));
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
            if read_i32(v + 0xf) != -1 {
                continue;
            }
            let t = read_u8(v);
            let divisor = if t == 0xb {
                match ((read_u8(v + 0x24) >> 1) & 0xf).wrapping_sub(2) {
                    0 | 1 => {
                        crate::comparacao::ramo("cpy_3 verba da vaga de titular");
                        Some(a * 2) // cpy3: a verba da vaga de titular
                    }
                    2 => Some(a * 4),
                    3 => Some(a * 10),
                    _ => None,
                }
            } else {
                if read_i32(v + 0x13) != -1 {
                    continue;
                }
                let rico = read_u8(clube + club::PROFESSIONAL_STATUS) == 1;
                match (t as i8 as i32).wrapping_sub(6) {
                    0 | 2 => Some(if rico { a * 10 } else { 0 }),
                    1 => Some(if rico { a * 1000 } else { 0 }),
                    3 | 4 => Some(if rico { a * 20 } else { 0 }),
                    _ => None,
                }
            };
            let Some(dv) = divisor else { continue };
            let x = if dv == 0 { 0 } else { verba.wrapping_div(dv) };
            write_i32(v + 7, x);
            repartido = repartido.wrapping_add(x);
        }
    }
    let mut resto = verba.wrapping_sub(repartido);
    let mut baixos: i8 = 0;
    if limite > 0 {
        for k in 0..limite as i32 {
            let v = vaga(k);
            let t = read_u8(v);
            if t != 0xb && read_i32(v + 0x13) > -1 {
                continue;
            }
            let alvo = read_i32(v + 0xf);
            if alvo > -1 || t == 7 {
                continue;
            }
            let lance = read_i32(v + 0x17);
            if lance > -1 {
                if alvo == -1 {
                    write_i32(v + 0x17, -1);
                } else if t == 0xb {
                    let l = read_usize(lance_de(read_usize(G_TRANSFER_MANAGER), lance));
                    if l != 0 {
                        resto = resto.wrapping_sub(read_i32(l + 0x1b));
                    }
                } else {
                    resto = resto.wrapping_sub(read_i32(cpy3_pessoa(alvo) + 0x52));
                }
                continue;
            }
            if read_u8(clube + club::PROFESSIONAL_STATUS) != 1 && t != 0xb {
                write_i32(v + 7, 0);
            } else {
                let base = if repartido >= 1000 { repartido } else { 1000 };
                let e = read_i32(v + 7);
                let x = ftol((e as f64) / (base as f64) * (resto as f64) * read_f64(0x969540));
                crate::comparacao::ramo("cpy_3 reducao pela sobra");
                write_i32(v + 7, e.wrapping_sub(x)); // cpy3: a redução da vaga pela sobra
            }
            let marca = read_u8(v + 0x24) & 0x1e;
            if marca <= 6 {
                if baixos < 3 {
                    baixos = baixos.wrapping_add(1);
                    let s = if soma >= 1 { soma } else { 1 };
                    let b = read_i32(v + 0xb);
                    let b = if b >= 1 { b } else { 1 };
                    let x = ftol((resto as f64) / (s as f64) * (b as f64));
                    write_i32(v + 7, read_i32(v + 7).wrapping_add(x));
                }
            } else {
                let s = if soma >= 1 { soma } else { 1 };
                let b = read_i32(v + 0xb);
                let b = if b >= 1 { b } else { 1 };
                let x = ftol((resto as f64) / (s as f64) * (b as f64) * read_f64(0x968968));
                write_i32(v + 7, read_i32(v + 7).wrapping_add(x));
            }
            *((v + 0x24) as *mut u8) &= 0xfe;
            crate::comparacao::ramo("cpy_3 vaga preenchida");
            preencher(this, v, clube, 0); // cpy3: a vaga é preenchida
            let alvo = read_i32(v + 0xf);
            if alvo > -1 && read_u8(v + 0x24) & 1 == 0 {
                resto = resto.wrapping_add(ftol(read_i32(cpy3_pessoa(alvo) + 0x52) as f64 * read_f64(0x96f878)));
            }
            soma = soma.wrapping_sub(read_i32(v + 0xb));
        }
    }
    // a sobra vai à primeira vaga de jogador do próprio clube que o técnico aceitaria vender
    if limite >= 0x28 {
        return;
    }
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let emprestado: unsafe extern "C" fn(usize, u32) -> u32 = core::mem::transmute(FN_STAFF_IS_LOAN_LISTED);
    let contrato_no: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let interesse: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32) -> f64 = core::mem::transmute(FN_SHORTLIST_MANAGER_STAFF_INTEREST_IN_JOINING_CLUB);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let fim: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_STAFF_CONTRACT_DAYS_TO_END_DATE);
    let ofertas: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_OFFERS_FOR_PLAYER);
    let menos_dias: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(FN_DATE_SUB_DAYS);
    let mut k = limite;
    while resto > 0 {
        let v = vaga(k as i32);
        let alvo = read_i32(v + 0xf);
        let candidata = 'c: {
            if read_u8(v) != 0xb || alvo <= -1 || read_i32(v + 0x17) != -1 {
                break 'c false;
            }
            let p = cpy3_pessoa(alvo);
            if read_usize(p + 0x39) != clube {
                break 'c false;
            }
            let c = contract_of(read_usize(G_CONTRACT_MANAGER), p);
            *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
            let do_contrato = if c != 0 { read_usize(G_CLUB_TABLE).wrapping_add(read_usize(c + 4).wrapping_mul(0x245)) } else { 0 };
            if do_contrato == clube || emprestado(p, 0) != 0 {
                break 'c false;
            }
            let c2 = contrato_no(read_usize(G_CONTRACT_MANAGER), p, clube);
            let estatuto = if c2 != 0 { (read_u8(c2 + 0x4f) >> 4) as u32 } else { 4 };
            let f = interesse(this, p, clube, 1, 0, estatuto, 0);
            if !(f >= read_f64(0x968e58)) {
                break 'c false;
            }
            if c2 != 0 {
                let lim = 20 - sorteio(10);
                if (fim(c2) as i16 as i32) < lim {
                    break 'c true;
                }
            }
            if ofertas(read_usize(G_TRANSFER_MANAGER), p, 0, 0) != 0 {
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
            write_i32(v + 0xb, read_i32(v + 3));
            write_i32(v + 7, resto);
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
pub(crate) unsafe extern "thiscall" fn comparar_allocate_transfer_budget(this: usize, clube: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize)>(IDX_ALLOCATE_TRANSFER_BUDGET)(this, clube)
}

/// statuses 5 and 6 (youth) only below age 24; the others, always
/// Original: `shortlist_manager_squad_status_allowed` (0x84b4d0, shortlist_manager.cpp, stdcall).
///
/// [pt] { 0x84b4d0 } (stdcall, ret 8, shortlist_manager.cpp; chamada com ecx = o Shortlist_Manager, que ela não lê):
/// `shortlist_manager_squad_status_allowed(pessoa, estatuto)`, pela tela de contrato (0x4dd0f0) e pela de transferência
/// (0x8f6770). Os estatutos 5 e 6 (jovens) só abaixo de 24 anos (+0x18, com sinal); os outros, 1. Sem ficha de jogador
/// (+0x61): a original (caixa "Error", 0).
pub(crate) unsafe extern "stdcall" fn shortlist_manager_squad_status_allowed(pessoa: usize, estatuto: u32) -> u32 {
    if read_usize(pessoa + staff::PLAYER) == 0 {
        return orig::<unsafe extern "stdcall" fn(usize, u32) -> u32>(IDX_SHORTLIST_MANAGER_SQUAD_STATUS_ALLOWED)(pessoa, estatuto); // caixa: só a original
    }
    match estatuto as u8 {
        5 | 6 => ((read_u8(pessoa + staff::YEAR_OF_BIRTH) as i8) < 0x18) as u32,
        _ => 1,
    }
}

/// Comparador: a devolução (rotina pura); sem ficha de jogador, só a original (caixa).
pub(crate) unsafe extern "stdcall" fn comparar_shortlist_manager_squad_status_allowed(pessoa: usize, estatuto: u32) -> u32 {
    let o = orig::<unsafe extern "stdcall" fn(usize, u32) -> u32>(IDX_SHORTLIST_MANAGER_SQUAD_STATUS_ALLOWED);
    if read_usize(pessoa + staff::PLAYER) == 0 {
        return o(pessoa, estatuto);
    }
    let r = o(pessoa, estatuto);
    let n = shortlist_manager_squad_status_allowed(pessoa, estatuto);
    PLACAR.registrar(r == n, || format!("shortlist_manager_squad_status_allowed(pessoa {:x}, estatuto {}) original={} rust={}", pessoa, estatuto as i8, r, n));
    r
}



// ---------- 252a leva: a lista de alvos dos clubes: as vagas da formação, refazer a lista do clube e suspender os clubes que miram o jogador (shortlist_manager.cpp) ----------



/// A entrada de 9 bytes do clube na Shortlist_Manager (+0xdc; +0 as 40 vagas de 0x25 bytes, +8 o estado).
pub(crate) unsafe fn cpp5_entrada(this: usize, clube: usize) -> usize {
    read_usize(this + 0xdc).wrapping_add(read_usize(clube).wrapping_mul(9))
}

/// A primeira diferença entre duas leituras de memória (endereço, original, Rust), para a mensagem.
pub(crate) fn cpp5_dif(a: &crate::comparacao::Memoria, b: &crate::comparacao::Memoria) -> Option<(usize, u8, u8)> {
    for ((ea, va), (_, vb)) in a.0.iter().zip(b.0.iter()) {
        if let Some(k) = (0..va.len()).find(|&k| va[k] != vb[k]) {
            return Some((ea + k, va[k], vb[k]));
        }
    }
    None
}

/// As máscaras de posição (+1, 16 bits) das vagas 1..24 de jogador, por formação (+7 da entrada); a vaga 0 é sempre
/// o goleiro (1). A linha 0 vale para formação fora de 1..9 (o `default` do switch).
pub(crate) static CPP5_MASCARAS: [[u16; 24]; 10] = [
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

/// sets the role of the 40 slots of the club's entry (25 player slots, 15 staff slots) and writes the position mask of
/// the 25 player slots by formation (+7)
/// Original: `shortlist_set_up_formation_slots` (0x83c090, shortlist_manager.cpp, stdcall, ret 4).
///
/// [pt] { 0x83c090 } (stdcall, ret 4, shortlist_manager.cpp): `shortlist_set_up_formation_slots(entrada)`: nas 40 vagas de
/// 0x25 bytes da entrada (+0), as 25 primeiras viram vagas de jogador (+0 = 0xb) e as 15 seguintes recebem o papel
/// (6, 7, 8 ×5, 10, 9 ×7) com +0x24 = (+0x24 & 0xe3) | 2; depois grava a máscara de posição (+1, 16 bits) das 25 vagas
/// de jogador pela formação (+7, byte com sinal; fora de 1..9 a linha padrão). A tabela saiu da emulação do switch
/// (cada caso grava as 25 vagas uma vez só).
pub(crate) unsafe extern "stdcall" fn shortlist_set_up_formation_slots(entrada: usize) {
    for k in 0..40usize {
        let s = read_usize(entrada) + k * 0x25;
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
    let v = read_usize(entrada);
    write_u16(v + 1, 1); // o goleiro
    for k in 1..25usize {
        write_u16(v + 1 + k * 0x25, CPP5_MASCARAS[linha][k - 1]);
    }
}

/// releases the stale targets of the club's slots, reassesses the squad (0x83cb40) and shares out the transfer budget
/// again (0x83b6e0)
/// Original: `Shortlist_Manager::update_club_shortlist` (0x83a730, shortlist_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x83a730 } (thiscall, ret 4, shortlist_manager.cpp): `Shortlist_Manager::update_club_shortlist(clube)`: solta os
/// alvos velhos das vagas do clube (0x844660 com a entrada), reavalia o elenco (0x83cb40 com a entrada recalculada,
/// o clube, diário 1, refazer 0) e redistribui a verba de transferências (0x83b6e0).
pub(crate) unsafe extern "thiscall" fn update_club_shortlist(this: usize, clube: usize) {
    let soltar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_SHORTLIST_MANAGER_RELEASE_CLUB_SLOT_TARGETS);
    let avaliar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(FN_SHORTLIST_MANAGER_ASSESS_CLUB_SQUAD);
    let verba: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_SHORTLIST_MANAGER_ALLOCATE_TRANSFER_BUDGET);
    soltar(this, clube, cpp5_entrada(this, clube));
    avaliar(this, cpp5_entrada(this, clube), clube, 1, 0);
    crate::comparacao::ramo("cpp_5 refazer lista");
    verba(this, clube); // a verba de transferências
}

/// sets to 100 the entry state of every club that targets the player (0x842180) and, with clear, removes the bid marker
/// from the slots holding him
/// Original: `Shortlist_Manager::suspend_clubs_shortlisting_player` (0x842b00, shortlist_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x842b00 } (thiscall, ret 8, shortlist_manager.cpp, quadro SEH só para a lista nova):
/// `Shortlist_Manager::suspend_clubs_shortlisting_player(jogador, limpar)`: numa LLIST nova (0x944cf1(0xe) e 0x68d020)
/// junta os clubes que miram o jogador (0x842180 com ignorar 1, modo 3, todos); para cada item (0x68d1b0, depois de
/// recomeçar o cursor +0xa = +2) põe o estado da entrada do clube (+8) em 100 e, com `limpar`, tira a marca de lance
/// (+0x17 = −1 quando > −1) das vagas que têm o jogador (+0xf = id). Destrói e libera a lista (0x68d040, 0x944c94).
/// A contagem (+0, 16 bits com sinal) é relida a cada volta, como na original.
pub(crate) unsafe extern "thiscall" fn suspend_clubs_shortlisting_player(this: usize, jogador: usize, limpar: u8) {
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let interessados: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(FN_SHORTLIST_MANAGER_COLLECT_INTERESTED_CLUBS);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_OPERATOR_DELETE);
    let p = novo(0xe);
    let lista = if p != 0 { lista_nova(p) } else { 0 };
    interessados(this, jogador, lista, 1, 3, 0);
    let n = *(lista as *const i16);
    if n != 0 {
        core::ptr::write_unaligned((lista + 0xa) as *mut usize, read_usize(lista + 2));
    }
    if n > 0 {
        let mut i = 0i32;
        while i < *(lista as *const i16) as i32 {
            let item = proximo(lista);
            if item != 0 {
                let e = read_usize(this + 0xdc).wrapping_add(read_usize(item + 4).wrapping_mul(9));
                crate::comparacao::ramo("cpp_5 suspender clube");
                *((e + 8) as *mut u8) = 0x64; // o clube suspende o jogador
                if limpar != 0 {
                    for k in 0..0x28usize {
                        let s = read_usize(e) + k * 0x25;
                        if read_usize(s + 0xf) == read_usize(jogador) && core::ptr::read_unaligned((s + 0x17) as *const i32) > -1 {
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
pub(crate) unsafe extern "stdcall" fn comparar_shortlist_set_up_formation_slots(entrada: usize) {
    let o = orig::<unsafe extern "stdcall" fn(usize)>(IDX_SHORTLIST_SET_UP_FORMATION_SLOTS);
    let reg = [(read_usize(entrada), 40 * 0x25)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(entrada), || shortlist_set_up_formation_slots(entrada));
    PLACAR.registrar(igual, || format!("shortlist_set_up_formation_slots(formacao {}) primeira diferença {:x?}", *((entrada + 7) as *const i8), cpp5_dif(&mo, &mr)));
}

/// Comparador: a 0x83cb40 reavalia o elenco (sorteios, ofertas, vagas de muitos clubes): roda só a original.
// ponytail: só a original; a cópia é provada pelo ramo e pela mutação na carreira.
pub(crate) unsafe extern "thiscall" fn comparar_update_club_shortlist(this: usize, clube: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize)>(IDX_UPDATE_CLUB_SHORTLIST)(this, clube)
}

/// Comparador: lado a lado pelas entradas (9 bytes) e, com `limpar`, pelas vagas (40 × 0x25) dos clubes do plano: os
/// clubes que a 0x842180 (pura) junta antes, numa lista própria. Cada lado cria e libera a sua lista.
pub(crate) unsafe extern "thiscall" fn comparar_suspend_clubs_shortlisting_player(this: usize, jogador: usize, limpar: u8) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u8)>(IDX_SUSPEND_CLUBS_SHORTLISTING_PLAYER);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let interessados: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(FN_SHORTLIST_MANAGER_COLLECT_INTERESTED_CLUBS);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_OPERATOR_DELETE);
    let p = novo(0xe);
    if p == 0 {
        return o(this, jogador, limpar);
    }
    let lista = lista_nova(p);
    interessados(this, jogador, lista, 1, 3, 0);
    // os itens (8 bytes: id do jogador, id do clube) pelos nós (+2 o primeiro; nó: +0 o dado, +6 o próximo)
    let mut reg: Vec<(usize, usize)> = Vec::new();
    let mut no = read_usize(lista + 2);
    while no != 0 {
        let e = read_usize(this + 0xdc).wrapping_add(read_usize(read_usize(no) + 4).wrapping_mul(9));
        reg.push((e, 9));
        if limpar != 0 {
            reg.push((read_usize(e), 0x28 * 0x25));
        }
        no = read_usize(no + 6);
    }
    destruir(lista);
    liberar(lista);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, jogador, limpar), || suspend_clubs_shortlisting_player(this, jogador, limpar));
    PLACAR.registrar(igual, || format!("suspend_clubs_shortlisting_player(jogador {:x}, limpar {}, {} regiões) primeira diferença {:x?}", read_usize(jogador), limpar, reg.len(), cpp5_dif(&mo, &mr)));
}

// ---------- 252a leva: pôr o jogador à venda, pô-lo nas listas de alvos dos clubes e pagar o prêmio por jogos da transferência (shortlist_manager.cpp, transfer_manager.cpp) ----------




/// O contrato da pessoa (0x4cf5a0 no CONTRACT_MANAGER [0xae1050]).
pub(crate) unsafe fn cpp6_contrato(pessoa: usize) -> usize {
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    contract_of(read_usize(G_CONTRACT_MANAGER), pessoa)
}

/// O contrato da pessoa, guardado em [0xae1080] como a original faz depois de cada chamada.
pub(crate) unsafe fn cpp6_contrato_guardado(pessoa: usize) -> usize {
    let c = cpp6_contrato(pessoa);
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
    c
}

/// with the player's filters (record, marked regen, job, career end, nation, untouchable, arrived less than 200 days
/// ago without being listed), tries to put him on the list of each club in the array (0x842e10) up to 5 clubs, except
/// national teams, the contract's club and his own; in DESLIGADAS since 2026-10-30: the detour alone (without running
/// the copy) made the career in original mode leave the reference be54a138 (bisection of the 39 routines of batch 252):
/// the detour changes the stack residue a later original reads; the copy stays in the code and does not run
/// Original: `Shortlist_Manager::add_player_to_club_shortlists` (0x842c00, shortlist_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x842c00 } (thiscall, ret 8, shortlist_manager.cpp): `Shortlist_Manager::add_player_to_club_shortlists(jogador,
/// forçar)`: com ficha de posições (+0x61; sem forçar, nota +0xb ≥ 0x157c), fora da marca 0x10 dos bits 0x30 do registro
/// de 79 bytes (+0xb), jogador (+0x3d = 0xb), não em fim de carreira (+0x24 com +0x28 = 5), fora da nação [0x9cf458]
/// (+0x1a) e sem ser intocável (0x8cca10 = 0): se chegou ao clube há menos de 200 dias (hoje − 200, 0x54c8e0, antes da
/// data +0x3e/+0x40) e o contrato não está à venda (+0x4f nibble alto ≠ 7) nem tem +0x45, sai. Senão, com o clube do
/// contrato (guardado em [0xae1080]), o contador (nibble baixo do +0xb do registro de 79 bytes) e a máscara de
/// posições (0x5b7d20 com os vizinhos: 2→4, 8→0x10, 0x20→0x10, 0x80→0x100, 0x800→0x400), percorre os clubes do vetor
/// [0xae2d1c] ([0xae235c] relido a cada volta) até o contador chegar a 5: fora seleção (0x53b900), o clube do contrato e
/// o do jogador (+0x39), tenta a 0x842e10 e conta cada clube que o pôs na lista.
pub(crate) unsafe extern "thiscall" fn add_player_to_club_shortlists(this: usize, jogador: usize, forcar: u32) {
    let fichas = read_usize(jogador + 0x61);
    if fichas == 0 {
        return;
    }
    if forcar == 0 && core::ptr::read_unaligned((fichas + 0xb) as *const i16) < 0x157c {
        return;
    }
    let registro = |j: usize| read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(j).wrapping_mul(0x4f));
    if read_u8(registro(jogador) + 0xb) & 0x30 == 0x10 {
        return;
    }
    if read_u8(jogador + 0x3d) != 0xb {
        return;
    }
    if read_usize(jogador + 0x24) != 0 && read_u8(jogador + 0x28) == 5 {
        return;
    }
    let nacao = read_usize(jogador + 0x1a);
    if nacao != 0 && read_usize(nacao) == read_usize(KEY_NATION_SAUDI_ARABIA) {
        return;
    }
    let fora: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_FUTURE_TRANSFER);
    if fora(read_usize(G_TRANSFER_MANAGER), jogador, 0) != 0 {
        return;
    }
    if read_usize(jogador + 0x39) != 0 {
        let menos: unsafe extern "thiscall" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(FN_DATE_SUB_DAYS);
        let mut t = [0u32; 2];
        let p = menos(0xae2c90, t.as_mut_ptr(), 200);
        let lo = read_usize(p) as u32;
        let (dia, ano) = (lo as u16 as i16, (lo >> 16) as u16);
        let ano_j = core::ptr::read_unaligned((jogador + 0x40) as *const u16);
        let dia_j = core::ptr::read_unaligned((jogador + 0x3e) as *const i16);
        let recente = if ano != ano_j { ano < ano_j } else { dia < dia_j };
        if recente {
            let c = cpp6_contrato(jogador);
            if c != 0 && read_u8(c + 0x4f) & 0xf0 != 0x70 && read_usize(c + 0x45) == 0 {
                return;
            }
        }
    }
    let c = cpp6_contrato_guardado(jogador);
    let clube_do_contrato = if c != 0 { cpe1_clube(read_usize(c + 4)) } else { 0 };
    let mut conta = (read_u8(registro(jogador) + 0xb) & 0xf) as i8;
    let mascara: unsafe extern "C" fn(usize) -> u16 = core::mem::transmute(FN_FORMATION_GET_POSITION_MASK);
    // ponytail: a 0x5b7d20 devolve só ax e a 0x842e10 usa 16 bits da máscara: os 16 de cima vão zerados
    let mut m = mascara(read_usize(jogador + 0x61)) as u32;
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
    if *(G_CLUB_COUNT as *const i32) <= 0 {
        return;
    }
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let acrescentar: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_SHORTLIST_MANAGER_TRY_ADD_PLAYER_TO_CLUB_SHORTLIST);
    let clubes = read_usize(G_CLUB_REPUTATION_ORDER);
    let mut i = 0i32;
    loop {
        let clube = cpe1_clube(read_usize(clubes + i as usize * 4));
        if conta >= 5 {
            return;
        }
        if selecao(clube) == 0 && clube_do_contrato != clube && read_usize(jogador + 0x39) != clube && acrescentar(this, jogador, clube, m) != 0 {
            crate::comparacao::ramo("cpp_6 clube acrescentou");
            conta = conta.wrapping_add(1); // mais um clube com ele na lista
        }
        i += 1;
        if i >= *(G_CLUB_COUNT as *const i32) {
            return;
        }
    }
}

/// marks the contract as transfer listed; the player may come to dislike the manager (likes the club or loyalty against
/// two draws of 100); with news, puts him on the clubs' lists and posts the news
/// Original: `Shortlist_Manager::transfer_list_player` (0x842320, shortlist_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x842320 } (thiscall, ret 8, shortlist_manager.cpp): `Shortlist_Manager::transfer_list_player(jogador, notícia)`:
/// se o clube ainda não o listou (0x5444a0 com 0) e há contrato: marca o contrato à venda (+0x4e bit 1, +0x4f nibble
/// alto 7); se não é intocável (0x8cca10): com o jogador gostando do clube (0x547d60) ou com a lealdade (+0x44 do
/// contrato, com sinal) acima da diferença de dois sorteios(100), o jogador passa a desgostar do técnico do clube
/// (0x547af0 com o +0xcf do clube, 1); com `notícia`, põe o jogador nas listas dos clubes (0x842c00 com 1) e dá a
/// notícia (0x4d71b0 com o clube +0x39).
pub(crate) unsafe extern "thiscall" fn transfer_list_player(this: usize, jogador: usize, noticia: u32) {
    let listada: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_TRANSFER_LISTED);
    if listada(jogador, 0) != 0 {
        return;
    }
    let c = cpp6_contrato(jogador);
    if c == 0 {
        return;
    }
    let a = read_u8(c + 0x4f);
    let b = read_u8(c + 0x4e);
    *((c + 0x4e) as *mut u8) = b | 1;
    crate::comparacao::ramo("cpp_6 por a venda");
    *((c + 0x4f) as *mut u8) = (a & 0xf) | 0x70; // à venda
    let fora: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_FUTURE_TRANSFER);
    if fora(read_usize(G_TRANSFER_MANAGER), jogador, 0) != 0 {
        return;
    }
    let gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_FAVOURS_CLUB);
    let desgostar = if gosta(jogador, read_usize(jogador + 0x39)) != 0 {
        true
    } else {
        let lealdade = read_u8(c + 0x44) as i8 as i32;
        let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
        let s1 = sorteio(100);
        let s2 = sorteio(100);
        lealdade > s1.wrapping_sub(s2)
    };
    if desgostar {
        let antipatia: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(FN_STAFF_ADD_DISLIKED_STAFF);
        antipatia(jogador, read_usize(read_usize(jogador + 0x39) + 0xcf), 1);
    }
    if noticia != 0 {
        let nas_listas: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_SHORTLIST_MANAGER_ADD_PLAYER_TO_CLUB_SHORTLISTS);
        let dar_noticia: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_CONTRACT_MANAGER_POST_TRANSFER_LISTED_NEWS);
        nas_listas(this, jogador, 1);
        dar_noticia(read_usize(G_CONTRACT_MANAGER), jogador, read_usize(jogador + 0x39));
    }
}

/// O registro de prêmio por jogos (0xd bytes: +0 id da pessoa, +4 valor, +8 jogos, +9 id do clube antigo) da pessoa,
/// pela busca binária (0x946464 com 0x8b4e00) no vetor das seleções (tipo 5: +0x898, conta +0x8a0) ou do clube
/// (+0x88c, conta +0x894) do TRANSFER_MANAGER.
pub(crate) unsafe fn cpp6_registro_do_premio(this: usize, jogador: usize, tipo: u32) -> usize {
    let bsearch: unsafe extern "C" fn(*const u32, usize, usize, u32, usize) -> usize = core::mem::transmute(FN_CRT_BSEARCH);
    let chave = [read_usize(jogador) as u32];
    let (vetor, conta) = if tipo as u8 == 5 { (read_usize(this + 0x898), read_usize(this + 0x8a0)) } else { (read_usize(this + 0x88c), read_usize(this + 0x894)) };
    bsearch(chave.as_ptr(), vetor, conta, 0xd, 0x8b4e00)
}


/// Comparador: a 0x842e10 mexe nas vagas de muitos clubes (e as listas não cabem numa cópia barata): roda só a
/// original.
// ponytail: só a original; a cópia é provada pelo ramo e pela mutação na carreira.
pub(crate) unsafe extern "thiscall" fn comparar_add_player_to_club_shortlists(this: usize, jogador: usize, forcar: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_ADD_PLAYER_TO_CLUB_SHORTLISTS)(this, jogador, forcar)
}

/// Comparador: sorteia e dá notícia: roda só a original e confere o plano feito antes: sem listagem do clube e com
/// contrato, o contrato fica à venda (+0x4e bit 1, +0x4f nibble alto 7).
pub(crate) unsafe extern "thiscall" fn comparar_transfer_list_player(this: usize, jogador: usize, noticia: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_TRANSFER_LIST_PLAYER);
    let listada: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_TRANSFER_LISTED);
    let marca = listada(jogador, 0) == 0 && cpp6_contrato(jogador) != 0;
    o(this, jogador, noticia);
    if marca {
        let c = cpp6_contrato(jogador);
        let ok = c != 0 && read_u8(c + 0x4e) & 1 != 0 && read_u8(c + 0x4f) & 0xf0 == 0x70;
        PLACAR.registrar(ok, || format!("transfer_list_player(jogador {:x}) contrato {:x} sem a marca de à venda", read_usize(jogador), c));
    }
}


// ---------- 251a leva: as listas de posição da lista de observação: montar as listas, ordená-las a cada dia e os clubes interessados no jogador (shortlist_manager.cpp) ----------







/// O contrato principal da pessoa (0x4cf5a0, guardado em [0xae1080]), o clube dele (id em +4) e o contrato com esse
/// clube (0x4cf5f0).
pub(crate) unsafe fn cpo8_contrato_com_o_clube(p: usize) -> usize {
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let com_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let c = contract_of(read_usize(G_CONTRACT_MANAGER), p);
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
    let clube = if c != 0 { cpe1_clube(read_usize(c + 4)) } else { 0 };
    com_clube(read_usize(G_CONTRACT_MANAGER), p, clube)
}

/// rebuilds the 32 lists (16 slots × 2) at 12 bytes per entry: the size is the largest number of people fitting a slot
/// (0x844300); each non-human person enters every slot he fits with his id, the nation id (drawn when missing) and the
/// value or the contract +0x3d
/// Original: `Shortlist_Manager::build_position_lists` (0x839e60, shortlist_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x839e60 } (thiscall, ret 4, shortlist_manager.cpp): `Shortlist_Manager::build_position_lists(não usado)`: libera
/// os 32 vetores das listas de posição (+0x18: 16 pela pessoa, +0x58: 16 pela chave do contrato); o tamanho deles
/// (+0xd8) é o maior número de pessoas (todas, humanos inclusive) que servem para uma das 16 vagas (0x844300); aloca os
/// 32 com 12 bytes por entrada (sem memória: a caixa "Error" e o jogo para) e zera as contas (+0x98). Depois, cada
/// pessoa fora dos 16 humanos, em cada vaga que serve (conta no limite: a caixa e o jogo para): as duas entradas com o
/// id dela (+0), o id da nação (+4; sem nação em +0x1a, a 0x90cfc0 sorteia uma e a grava na pessoa), na primeira o valor
/// (+0x52) e na segunda o +0x3d do contrato com o clube do contrato (sem contrato: 100). Roda a cada 30 dias.
pub(crate) unsafe extern "thiscall" fn build_position_lists(this: usize, _nao_usado: u32) {
    let livrar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    // ponytail: a original empurra a vaga num dword com 3 bytes de lixo da pilha acima; a 0x844300 só lê o byte
    let serve: unsafe extern "stdcall" fn(usize, u32) -> u8 = core::mem::transmute(FN_SHORTLIST_STAFF_FITS_SLOT_POSITION);
    crate::comparacao::ramo("cpo_8 montar listas");
    for k in 0..32usize {
        let v = this + 0x18 + k * 4;
        if read_usize(v) != 0 {
            livrar(read_usize(v));
            write_u32(v, 0);
        }
    }
    write_u32(this + 0xd8, 0);
    for vaga in 0..16u32 {
        let base = read_usize(G_STAFF_TABLE);
        let mut n = 0i32;
        let mut i = 0usize;
        while (i as i32) < read_i32(0xae2364) {
            if serve(base.wrapping_add(i * 0x6e), vaga) != 0 {
                n += 1;
            }
            i += 1;
        }
        if n > read_i32(this + 0xd8) {
            write_u32(this + 0xd8, n as u32);
        }
    }
    for k in 0..32usize {
        let v = alocar(read_usize(this + 0xd8).wrapping_mul(12) as u32);
        write_u32(this + 0x18 + k * 4, v as u32);
        if v == 0 {
            erro_em(0xa825a4, 0x10c);
        }
    }
    for k in 0..16usize {
        write_u32(this + 0x98 + k * 4, 0);
    }
    let base = read_usize(G_STAFF_TABLE);
    let mut i = 0usize;
    while (i as i32) < read_i32(0xae2364).wrapping_sub(0x10) {
        let p = base.wrapping_add(i * 0x6e);
        for vaga in 0..16u32 {
            if serve(p, vaga) == 0 {
                continue;
            }
            let pv = vaga as usize;
            let conta = read_i32(this + 0x98 + pv * 4);
            if conta >= read_i32(this + 0xd8) {
                erro_em(0xa825a4, 0x121);
            }
            let o = (conta as usize).wrapping_mul(12);
            let (l1, l2) = (read_usize(this + 0x18 + pv * 4).wrapping_add(o), read_usize(this + 0x58 + pv * 4).wrapping_add(o));
            write_u32(l1, i as u32);
            write_u32(l2, i as u32);
            let mut nacao = read_usize(p + 0x1a);
            if nacao == 0 {
                let sortear: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
                nacao = read_usize(G_NATION_TABLE).wrapping_add((sortear(read_i32(0xae2350)) as usize).wrapping_mul(0x122));
                write_u32(p + 0x1a, nacao as u32);
            }
            write_u32(l1 + 4, read_usize(nacao) as u32);
            write_u32(l2 + 4, read_usize(read_usize(p + 0x1a)) as u32);
            write_u32(l1 + 8, read_usize(p + 0x52) as u32);
            let k = cpo8_contrato_com_o_clube(p);
            write_u32(l2 + 8, if k != 0 { read_u8(k + 0x3d) as i8 as i32 as u32 } else { 100 });
            write_u32(this + 0x98 + pv * 4, (conta + 1) as u32);
        }
        i += 1;
    }
}

/// recomputes the entry keys (the value when the contract band is 0, minus 1,500,000 for a listed player of a human
/// club; in the second list −1000, 2000 or the contract +0x3d) and sorts both arrays of each slot (0x945333 with
/// 0x83b670)
/// Original: `Shortlist_Manager::sort_position_lists` (0x83a780, shortlist_manager.cpp, thiscall, ret 0).
///
/// [pt] { 0x83a780 } (thiscall, ret 0, shortlist_manager.cpp): `Shortlist_Manager::sort_position_lists()`: a cada dia,
/// refaz a chave (+8) das entradas das 16 vagas e ordena os dois vetores de cada uma (0x945333 com 0x83b670: pela chave
/// e pelo +4). Na primeira lista: o valor da pessoa (+0x52) se o contrato com o clube existe e a faixa do vencimento
/// (0x867d30) é 0, senão 0; o jogador de clube humano (0x600a50) na lista de transferências (0x5444a0) perde 1.500.000,
/// no mínimo um terço do valor. Na vaga 15 só a primeira, com o valor. Na segunda: −1000 com o bit 2 do +0x4e do
/// contrato, 2000 com os bits 0 ou 3 ou sem jogador (+0x61), sem clube, sem divisão (+0x57) ou divisão sem marca em
/// [0xadadfc] com reputação abaixo de 6000, ou sem contrato; senão o +0x3d do contrato.
pub(crate) unsafe extern "thiscall" fn sort_position_lists(this: usize) {
    let faixa: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(FN_STAFF_CONTRACT_GET_EXPIRY_BAND);
    let humano: unsafe extern "C" fn(usize, u32, u32, usize, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let listado: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_TRANSFER_LISTED);
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    for vaga in 0..16usize {
        let mut j = 0i32;
        let mut o = 0usize;
        while j < read_i32(this + 0x98 + vaga * 4) {
            let p = cpo3_pessoa(read_usize(read_usize(this + 0x18 + vaga * 4) + o));
            let k = cpo8_contrato_com_o_clube(p);
            if vaga == 15 {
                write_u32(read_usize(this + 0x54) + o + 8, read_usize(p + 0x52) as u32);
                j += 1;
                o += 12;
                continue;
            }
            if k != 0 && faixa(k) == 0 {
                write_u32(read_usize(this + 0x18 + vaga * 4) + o + 8, read_usize(p + 0x52) as u32);
                let clube = read_usize(p + 0x39);
                if clube != 0 && humano(clube, 0, 1, 0, 0) != 0 && listado(p, 0) != 0 {
                    let e = read_usize(this + 0x18 + vaga * 4) + o + 8;
                    write_u32(e, read_i32(e).wrapping_add(-0x16e360) as u32);
                    let terco = read_i32(p + 0x52) / 3;
                    if read_i32(e) < terco {
                        write_u32(e, terco as u32);
                    }
                }
            } else {
                if k != 0 {
                    crate::comparacao::ramo("cpo_8 faixa do contrato");
                }
                write_u32(read_usize(this + 0x18 + vaga * 4) + o + 8, 0);
            }
            // a segunda lista
            let p2 = cpo3_pessoa(read_usize(read_usize(this + 0x58 + vaga * 4) + o));
            let k2 = cpo8_contrato_com_o_clube(p2);
            let clube = read_usize(p2 + 0x39);
            let chave: i32 = if read_usize(p2 + 0x61) == 0 || clube == 0 || read_usize(clube + club::DIVISION) == 0 {
                0x7d0
            } else if read_usize(read_usize(G_COMP_OBJECT_TABLE) + read_usize(read_usize(clube + club::DIVISION)).wrapping_mul(4)) == 0
                && core::ptr::read_unaligned((clube + club::REPUTATION) as *const i16) < 0x1770
            {
                0x7d0
            } else if k2 == 0 {
                0x7d0
            } else if read_u8(k2 + 0x4e) & 2 != 0 {
                -1000
            } else if read_u8(k2 + 0x4e) & 9 != 0 {
                0x7d0
            } else {
                read_u8(k2 + 0x3d) as i8 as i32
            };
            write_u32(read_usize(this + 0x58 + vaga * 4) + o + 8, chave as u32);
            j += 1;
            o += 12;
        }
        let n = read_usize(this + 0x98 + vaga * 4) as u32;
        ordenar(read_usize(this + 0x18 + vaga * 4), n, 12, 0x83b670);
        let n = read_usize(this + 0x98 + vaga * 4) as u32;
        ordenar(read_usize(this + 0x58 + vaga * 4), n, 12, 0x83b670);
    }
}

/// empties the output list and adds the (player, club) pairs: the humans who have the player on the scout shortlist and
/// the clubs in the by-person array that are not suspended, are not his and value him above his value
/// Original: `Shortlist_Manager::collect_interested_clubs` (0x842180, shortlist_manager.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x842180 } (thiscall, ret 0x14, shortlist_manager.cpp): `Shortlist_Manager::collect_interested_clubs(jogador,
/// saída, ignorar, modo, só_humanos)`: esvazia a lista `saída` (0x68d080) e acrescenta (0x68d0d0, 8 bytes: id do
/// jogador, id do clube) os clubes interessados: com modo ≠ 3, os 16 humanos (do último para o primeiro) com +0x5f e
/// clube que têm o jogador na lista do olheiro (0x7f7490; com modo 2, fora os clubes que o humano da vez controla,
/// 0x600c90); sem `só_humanos`, os pares (jogador, clube) do vetor pela pessoa (+0, conta +0x10; 0x946464 até o primeiro
/// do jogador) cujo clube não está suspenso (+8 da entrada de 9 bytes do clube em +0xdc abaixo de 100, salvo com
/// `ignorar`), não é o do jogador e põe nele (+7 da vaga de 0x25 bytes com o jogador em +0xf; sem vaga, a primeira)
/// mais que o valor (+0x52).
pub(crate) unsafe extern "thiscall" fn collect_interested_clubs(this: usize, jogador: usize, saida: usize, ignorar: u32, modo: u32, so_humanos: u32) {
    let acrescentar: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_LLIST_ADD_TAIL);
    if saida != 0 {
        let limpar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_CLEAR);
        limpar(saida);
    }
    if modo as u8 != 3 {
        let controla: unsafe extern "C" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_CURRENT_CONTROLS_CLUB);
        let in_list: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_SCOUT_MANAGER_IS_PLAYER_SHORTLISTED);
        for k in 0..16i32 {
            let h = cpo3_pessoa(read_i32(0xae2364).wrapping_sub(k).wrapping_sub(1) as usize);
            let clube = read_usize(h + 0x39);
            if read_u8(h + 0x5f) == 0 || clube == 0 {
                continue;
            }
            if controla(clube, 0, 1) != 0 && modo as u8 == 2 {
                continue;
            }
            if in_list(read_usize(G_SCOUT_MANAGER), jogador, h) != 0 {
                let par = [read_usize(jogador) as u32, read_usize(clube) as u32];
                acrescentar(saida, par.as_ptr() as usize, 8);
            }
        }
    }
    if so_humanos != 0 {
        return;
    }
    let bsearch: unsafe extern "C" fn(*const u32, usize, i32, u32, usize) -> usize = core::mem::transmute(FN_CRT_BSEARCH);
    let chave = [read_usize(jogador) as u32, 0xffff_ffff];
    let mut e = bsearch(chave.as_ptr(), read_usize(this), read_i32(this + 0x10), 8, 0x757ea0);
    if e == 0 {
        return;
    }
    let base = read_usize(this);
    let ultimo = base.wrapping_add((read_i32(this + 0x10) as usize).wrapping_mul(8)).wrapping_sub(8);
    let id = read_usize(jogador);
    if e > base {
        loop {
            let x = read_usize(e - 8);
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
    if read_usize(e) != id {
        return;
    }
    loop {
        let clube_id = read_usize(e + 4);
        let entrada = read_usize(this + 0xdc).wrapping_add(clube_id.wrapping_mul(9));
        let vagas = read_usize(entrada);
        let mut k = 0usize;
        while k < 0x28 && read_usize(vagas + 0xf + k * 0x25) != read_usize(jogador) {
            k += 1;
        }
        let valor = if k < 0x28 { read_i32(vagas + k * 0x25 + 7) } else { read_i32(vagas + 7) };
        if !(ignorar == 0 && read_u8(entrada + 8) >= 100) && valor > read_i32(jogador + 0x52) {
            let c = read_usize(jogador + 0x39);
            if !(c != 0 && read_usize(c) == clube_id) {
                crate::comparacao::ramo("cpo_8 clube interessado");
                acrescentar(saida, e, 8);
            }
        }
        if e == ultimo {
            return;
        }
        let prox = read_usize(e + 8);
        e += 8;
        if prox != read_usize(jogador) {
            return;
        }
    }
}

/// Os itens (8 bytes) de uma LLIST (+2 o primeiro nó; nó: +0 o dado, +6 o próximo).
pub(crate) unsafe fn cpo8_itens(lista: usize) -> Vec<(u32, u32)> {
    let mut v = Vec::new();
    let mut no = read_usize(lista + 2);
    while no != 0 {
        let x = read_usize(no);
        v.push((read_usize(x) as u32, read_usize(x + 4) as u32));
        no = read_usize(no + 6);
    }
    v
}

/// Comparador: só a original.
// ponytail: a original solta e aloca os 32 vetores (e pode sortear a nação de quem não tem); não roda duas vezes. Confere
// o resultado: as duas listas de cada vaga com os mesmos ids, cada entrada servindo para a vaga, a conta no limite.
pub(crate) unsafe extern "thiscall" fn comparar_build_position_lists(this: usize, nao_usado: u32) {
    orig::<unsafe extern "thiscall" fn(usize, u32)>(IDX_BUILD_POSITION_LISTS)(this, nao_usado);
    let serve: unsafe extern "stdcall" fn(usize, u32) -> u8 = core::mem::transmute(FN_SHORTLIST_STAFF_FITS_SLOT_POSITION);
    let mut ruim = None;
    'f: for vaga in 0..16usize {
        let n = read_i32(this + 0x98 + vaga * 4);
        if n > read_i32(this + 0xd8) {
            ruim = Some((vaga, -1));
            break;
        }
        for j in 0..n.max(0) as usize {
            let (a, b) = (read_usize(this + 0x18 + vaga * 4) + j * 12, read_usize(this + 0x58 + vaga * 4) + j * 12);
            if read_usize(a) != read_usize(b) || serve(cpo3_pessoa(read_usize(a)), vaga as u32) == 0 {
                ruim = Some((vaga, j as i32));
                break 'f;
            }
        }
    }
    PLACAR.registrar(ruim.is_none(), || format!("build_position_lists: vaga e entrada {:?}", ruim));
}

/// Comparador: lado a lado pelos 32 vetores (12 bytes por entrada) e por [0xae1080].
pub(crate) unsafe extern "thiscall" fn comparar_sort_position_lists(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(IDX_SORT_POSITION_LISTS);
    let mut reg = vec![(0xae1080usize, 4usize)];
    for vaga in 0..16usize {
        let n = read_i32(this + 0x98 + vaga * 4);
        if n > 0 {
            reg.push((read_usize(this + 0x18 + vaga * 4), n as usize * 12));
            reg.push((read_usize(this + 0x58 + vaga * 4), n as usize * 12));
        }
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this), || sort_position_lists(this));
    PLACAR.registrar(igual, || {
        let dif: Vec<usize> = mo.0.iter().zip(mr.0.iter()).filter(|(a, b)| a.1 != b.1).map(|(a, _)| a.0).take(8).collect();
        format!("sort_position_lists: vetores diferentes {:x?}", dif)
    });
}

/// Comparador: a original na lista de saída e a cópia numa lista nova (0x68d020, solta com 0x68d080); compara os pares.
pub(crate) unsafe extern "thiscall" fn comparar_collect_interested_clubs(this: usize, jogador: usize, saida: usize, ignorar: u32, modo: u32, so_humanos: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32)>(IDX_COLLECT_INTERESTED_CLUBS);
    o(this, jogador, saida, ignorar, modo, so_humanos);
    if saida == 0 {
        return;
    }
    let criar: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let limpar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_CLEAR);
    let mut nova = [0u8; 0x10];
    let l = nova.as_mut_ptr() as usize;
    criar(l);
    collect_interested_clubs(this, jogador, l, ignorar, modo, so_humanos);
    let (a, b) = (cpo8_itens(saida), cpo8_itens(l));
    limpar(l);
    PLACAR.registrar(a == b, || format!("collect_interested_clubs({:#x}) original={:x?} rust={:x?}", jogador, a, b));
}

// ---------- 251a leva: a lista de observação e o empréstimo: acrescentar o par na lista, prever a resposta do clube vendedor e o período do empréstimo (shortlist_manager.cpp, transfer_manager.cpp) ----------






/// Acrescenta o par (id da pessoa, id do clube) no fim do vetor `[this + vet]` (conta em `this + conta`) e o ordena
/// (0x945333 com `cmp`).
pub(crate) unsafe fn cpo9_inserir(this: usize, vet: usize, conta: usize, pessoa: usize, clube: usize, cmp: usize) {
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    let n = read_i32(this + conta);
    let e = read_usize(this + vet).wrapping_add((n as usize).wrapping_mul(8));
    write_u32(e, read_usize(pessoa) as u32);
    write_u32(e + 4, read_usize(clube) as u32);
    write_u32(this + conta, (n + 1) as u32);
    ordenar(read_usize(this + vet), (n + 1) as u32, 8, cmp);
}

/// creates both pair arrays (capacity 100) and, when the pair is new: grows by 100, adds and sorts (by person, by club)
/// and, outside a human club, adds 1 to the low nibble of +0xb of the person's 79-byte record
/// Original: `Shortlist_Manager::add_shortlist_entry` (0x8423f0, shortlist_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x8423f0 } (thiscall, ret 8, shortlist_manager.cpp): `Shortlist_Manager::add_shortlist_entry(pessoa, clube)`: cria
/// os dois vetores de pares na primeira vez (0x944e46 de 0x320 bytes, capacidade +8/+0xc = 100; sem memória: a caixa
/// "Error" e o jogo para); com o par (pessoa, clube; sem clube −1) já no vetor pela pessoa (0x946464 com 0x757ea0), nada.
/// Senão cresce de 100 em 100 o vetor cheio (0x945667), acrescenta o par nos dois e os ordena (pela pessoa com
/// 0x757ea0, pelo clube com 0x844740) e, fora de clube humano (0x600a50), soma 1 no nibble baixo do +0xb do registro de
/// 79 bytes da pessoa ([0xae2d0c]: quantas listas a têm).
pub(crate) unsafe extern "thiscall" fn add_shortlist_entry(this: usize, pessoa: usize, clube: usize) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_CRT_REALLOC);
    if read_usize(this) == 0 {
        let v = alocar(0x320);
        write_u32(this, v as u32);
        if v == 0 {
            erro_em(0xa825a4, 0x1594);
        }
        write_u32(this + 8, 100);
    }
    if read_usize(this + 4) == 0 {
        let v = alocar(0x320);
        write_u32(this + 4, v as u32);
        if v == 0 {
            erro_em(0xa825a4, 0x159d);
        }
        write_u32(this + 0xc, 100);
    }
    if read_usize(this) != 0 {
        let bsearch: unsafe extern "C" fn(*const u32, usize, i32, u32, usize) -> usize = core::mem::transmute(FN_CRT_BSEARCH);
        let chave = [read_usize(pessoa) as u32, if clube != 0 { read_usize(clube) as u32 } else { 0xffff_ffff }];
        if bsearch(chave.as_ptr(), read_usize(this), read_i32(this + 0x10), 8, 0x757ea0) != 0 {
            return;
        }
    }
    crate::comparacao::ramo("cpo_9 nova entrada");
    if read_i32(this + 8) == read_i32(this + 0x10) {
        let v = realocar(read_usize(this), (read_i32(this + 8) as u32).wrapping_mul(8).wrapping_add(0x320));
        write_u32(this, v as u32);
        if v == 0 {
            erro_em(0xa825a4, 0x15b5);
        }
        write_u32(this + 8, read_i32(this + 8).wrapping_add(100) as u32);
    }
    if read_i32(this + 0xc) == read_i32(this + 0x14) {
        let v = realocar(read_usize(this + 4), (read_i32(this + 0xc) as u32).wrapping_mul(8).wrapping_add(0x320));
        write_u32(this + 4, v as u32);
        if v == 0 {
            erro_em(0xa825a4, 0x15be);
        }
        write_u32(this + 0xc, read_i32(this + 0xc).wrapping_add(100) as u32);
    }
    cpo9_inserir(this, 0, 0x10, pessoa, clube, 0x757ea0); // o vetor pela pessoa
    cpo9_inserir(this, 4, 0x14, pessoa, clube, 0x844740); // o vetor pelo clube
    let humano: unsafe extern "C" fn(usize, u32, u32, usize, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    if humano(clube, 0, 1, 0, 0) == 0 {
        let r = (read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(pessoa).wrapping_mul(0x4f)) + 0xb) as *mut u8;
        let a = *r;
        *r = (a & 0xf0) | (a.wrapping_add(1) & 0xf);
    }
}

/// 0 accepts, 1 haggles, 2 rejects: from the contract with the seller, the loan case (listed, reputation, status), the
/// price (human, clause, holiday, asking or 0x4d1600) against the offer
/// Original: `shortlist_manager_predict_selling_club_response` (0x84af00, shortlist_manager.cpp, stdcall, ret 0x20).
///
/// [pt] { 0x84af00 } (stdcall, ret 0x20, shortlist_manager.cpp): `shortlist_manager_predict_selling_club_response(vendedor,
/// jogador, oferta, comprador, pedido, empréstimo)`: 0 aceita, 1 negocia, 2 recusa. Sem vendedor ou com o contrato com
/// ele (0x4cf5f0) marcado em +0x49: 0; o técnico do vendedor é o jogador: 2; sem contrato de tipo 2 (+0x35): 2 no
/// empréstimo, 0 senão. Empréstimo: clube humano (0x600a50): 0; listado para empréstimo (0x544590) com a reputação do
/// comprador × [0x968928] acima do +0xb da ficha: 0; estatuto 6/7 (+0x4f) ou +0xf da ficha 20, com a reputação acima de
/// +0xb × [0x969040], +0x3d do contrato abaixo de −50 e o +0x1db do vendedor diferente de [0x9cf6e8]: 0; senão 2.
/// Compra: o preço (humano: 0x4d42d0 × [0x9689b0], sem cláusula 0x867b50 e com o técnico de férias 0x5fd210 a opção 0
/// 0x5fea50: 1 recusa, 2 o valor +0x52; senão o `pedido` não negativo, o valor sem ficha ou 0x4d1600); oferta no preço:
/// 0; humano com a oferta acima do preço − 1.500.000 e listado (0x5444a0): 1; preço × [0x96f9c0] acima da oferta: 2; 1.
pub(crate) unsafe extern "stdcall" fn shortlist_manager_predict_selling_club_response(vendedor: usize, jogador: usize, oferta: i64, comprador: usize, pedido: i64, emprestimo: u32) -> u8 {
    let com_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let humano: unsafe extern "C" fn(usize, u32, u32, usize, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let k = com_clube(read_usize(G_CONTRACT_MANAGER), jogador, vendedor);
    if vendedor == 0 || (k != 0 && read_u8(k + 0x49) != 0) {
        return 0;
    }
    if read_usize(vendedor + 0xcf) == jogador {
        return 2;
    }
    if k == 0 || read_u8(k + 0x35) & 0x3f != 2 {
        return if emprestimo as u8 != 0 { 2 } else { 0 };
    }
    if emprestimo as u8 != 0 {
        if humano(vendedor, 0, 1, 0, 0) != 0 {
            return 0;
        }
        let listado: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_LOAN_LISTED);
        let rep = read_i16(comprador + 0x80) as f64;
        if listado(jogador, 0) != 0 && rep * f64_em(0x968928) > read_i16(read_usize(jogador + 0x61) + 0xb) as f64 {
            return 0;
        }
        let st = read_u8(k + 0x4f) & 0xf0;
        if st == 0x60 || st == 0x70 || read_u8(read_usize(jogador + 0x61) + 0xf) == 0x14 {
            let v = read_i16(read_usize(jogador + 0x61) + 0xb) as f64 * f64_em(0x969040);
            if rep > v && (read_u8(k + 0x3d) as i8) < -50 && read_usize(vendedor + 0x1db) != read_usize(KEY_CLUB_COMP_UEFA_EUROPEAN_CHAMPIONS_LEAGUE) {
                return 0;
            }
        }
        return 2;
    }
    let preco: i32 = if humano(vendedor, 0, 1, 0, 0) != 0 {
        let pedido_humano: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> i32 = core::mem::transmute(FN_CONTRACT_MANAGER_GET_HUMAN_ASKING_PRICE);
        let mut v = ftol(pedido_humano(read_usize(G_CONTRACT_MANAGER), vendedor, comprador, jogador) as f64 * f64_em(0x9689b0));
        let clausula: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_STAFF_CONTRACT_HAS_ACTIVE_RELEASE_CLAUSE);
        if clausula(k) == 0 {
            let tecnico: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_CLUB_MANAGER);
            let ferias: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_ON_HOLIDAY);
            if ferias(read_usize(G_HUMAN_MANAGER), tecnico(vendedor)) != 0 {
                let opcao: unsafe extern "thiscall" fn(usize, u32, usize) -> u8 = core::mem::transmute(FN_HUMAN_MANAGER_GET_HOLIDAY_OPTION);
                match opcao(read_usize(G_HUMAN_MANAGER), 0, tecnico(vendedor)) as i8 {
                    1 => return 2,
                    2 => v = read_i32(jogador + 0x52),
                    _ => {}
                }
            }
        }
        v
    } else if pedido >= 0 {
        pedido as i32
    } else if read_usize(jogador + 0x61) == 0 {
        read_i32(jogador + 0x52)
    } else {
        let calcular: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, u32) -> i32 = core::mem::transmute(FN_CONTRACT_MANAGER_CALC_ASKING_PRICE);
        calcular(read_usize(G_CONTRACT_MANAGER), vendedor, comprador, jogador, 0, 1, 1)
    };
    if oferta >= preco as i64 {
        return 0;
    }
    let listado: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_TRANSFER_LISTED);
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
pub(crate) unsafe fn cpo9_ano_padrao() -> u16 {
    let padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    let mut data = [0u8; 8];
    let p = padrao(data.as_mut_ptr() as usize);
    read_i16(p + 2) as u16
}

/// A data (dia do ano +0, ano +2) passa do fim do contrato `c` (+0x2d dia, +0x2f ano)?
pub(crate) unsafe fn cpo9_depois_do_contrato(dia: i16, ano: u16, c: usize) -> bool {
    let fim = read_i16(c + 0x2f) as u16;
    ano > fim || (ano == fim && dia > read_i16(c + 0x2d))
}


/// Comparador: só a original, conferindo o plano (feito antes): o par já estava (nada muda) ou entra nos dois vetores
/// (contas + 1, o par achado depois) e o nibble da pessoa soma 1 fora de clube humano.
// ponytail: a original realoca os vetores (0x945667) e os ordena; não roda duas vezes.
pub(crate) unsafe extern "thiscall" fn comparar_add_shortlist_entry(this: usize, pessoa: usize, clube: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_ADD_SHORTLIST_ENTRY);
    let bsearch: unsafe extern "C" fn(*const u32, usize, i32, u32, usize) -> usize = core::mem::transmute(FN_CRT_BSEARCH);
    let humano: unsafe extern "C" fn(usize, u32, u32, usize, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let chave = [read_usize(pessoa) as u32, if clube != 0 { read_usize(clube) as u32 } else { 0xffff_ffff }];
    let tinha = read_usize(this) != 0 && bsearch(chave.as_ptr(), read_usize(this), read_i32(this + 0x10), 8, 0x757ea0) != 0;
    let (n0, n1) = if read_usize(this) != 0 { (read_i32(this + 0x10), read_i32(this + 0x14)) } else { (0, 0) };
    let r = read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(pessoa).wrapping_mul(0x4f)) + 0xb;
    let nib = read_u8(r);
    let soma = !tinha && humano(clube, 0, 1, 0, 0) == 0;
    o(this, pessoa, clube);
    let esperado = if soma { (nib & 0xf0) | (nib.wrapping_add(1) & 0xf) } else { nib };
    let (d0, d1) = (if tinha { n0 } else { n0 + 1 }, if tinha { n1 } else { n1 + 1 });
    let achado = bsearch(chave.as_ptr(), read_usize(this), read_i32(this + 0x10), 8, 0x757ea0) != 0;
    let ok = read_i32(this + 0x10) == d0 && read_i32(this + 0x14) == d1 && read_u8(r) == esperado && (achado || clube == 0);
    PLACAR.registrar(ok, || {
        format!("add_shortlist_entry({:#x}, {:#x}) tinha={} contas {}/{} -> {}/{} nibble {:#x} -> {:#x} (esperado {:#x})",
            pessoa, clube, tinha, n0, n1, read_i32(this + 0x10), read_i32(this + 0x14), nib, read_u8(r), esperado)
    });
}

/// Comparador: pelo valor (o byte baixo) e lado a lado pelo sorteio e [0xae1080] (as chamadas de preço podem sortear).
pub(crate) unsafe extern "stdcall" fn comparar_shortlist_manager_predict_selling_club_response(vendedor: usize, jogador: usize, oferta: i64, comprador: usize, pedido: i64, emprestimo: u32) -> u8 {
    let o = orig::<unsafe extern "stdcall" fn(usize, usize, i64, usize, i64, u32) -> u8>(IDX_SHORTLIST_MANAGER_PREDICT_SELLING_CLUB_RESPONSE);
    let mut reg = vec![(0xae1080usize, 4usize)];
    reg.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(
        &reg,
        || o(vendedor, jogador, oferta, comprador, pedido, emprestimo),
        || crate::aleatorio::sem_contar(|| shortlist_manager_predict_selling_club_response(vendedor, jogador, oferta, comprador, pedido, emprestimo)),
    );
    PLACAR.registrar(igual && r == n, || format!("shortlist_manager_predict_selling_club_response({:#x}, {:#x}, {}, {:#x}, {}, {}) original={} rust={} {:x?} x {:x?}", vendedor, jogador, oferta, comprador, pedido, emprestimo as u8, r, n, mo, mr));
    r
}


// ---------- 250a leva: a lista de observação do clube: o estatuto no elenco que o clube dá à pessoa, a estimativa do clube humano e soltar os alvos que não são do clube (shortlist_manager.cpp) ----------



/// O clube do contrato `c` (id em +4; o registro de clube tem 0x245 bytes em [0xae23b4]), ou 0 sem contrato.
pub(crate) unsafe fn cpn6_clube_do_contrato(c: usize) -> usize {
    if c == 0 {
        return 0;
    }
    read_usize(G_CLUB_TABLE).wrapping_add(read_usize(c + 4).wrapping_mul(0x245))
}

/// the squad status the club gives the person: from her entry in the club shortlist (bits 1..4 of +0x24, job from +0),
/// from the contract with the club (high nibble of +0x4f), from the human club estimate, or 7
/// Original: `Shortlist_Manager::get_squad_status_for_club` (0x849f00, shortlist_manager.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x849f00 } (thiscall, ret 0xc, shortlist_manager.cpp): `Shortlist_Manager::get_squad_status_for_club(pessoa,
/// clube, cargo)`: o estatuto no elenco que o `clube` dá à `pessoa`. Clube nulo: 7. A entrada da pessoa na lista do
/// clube (0x849ce0: a de tipo ≠ 11 para jogador, +0x61; tendo +0x69, a de tipo 11 também): o cargo (+0 da entrada) e
/// os bits 1..4 do +0x24. Senão o cargo é o +0x3d da pessoa e vale o contrato (0x4cf5a0, guardado em [0xae1080]): do
/// próprio clube, o nibble alto do +0x4f do contrato com o clube (0x4cf5f0; sem ele, 7); clube humano (0x600a50) com
/// cargo 11 a 15 (0x53b620): a estimativa 0x849d60; senão, o clube da pessoa (+0x39) sendo o clube ou o parceiro dele
/// (0x540a50) leva ao contrato com o clube; nenhum: 7.
pub(crate) unsafe extern "thiscall" fn get_squad_status_for_club(this: usize, pessoa: usize, clube: usize, cargo: *mut u8) -> u8 {
    if clube == 0 {
        return 7;
    }
    let entrada: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(FN_SHORTLIST_MANAGER_FIND_CLUB_SHORTLIST_ENTRY);
    let mut e = entrada(this, clube, pessoa, (read_usize(pessoa + staff::PLAYER) == 0) as u32, 0);
    if e == 0 && read_usize(pessoa + staff::NON_PLAYER) != 0 {
        e = entrada(this, clube, pessoa, 1, 0);
    }
    if e != 0 {
        if !cargo.is_null() {
            *cargo = read_u8(e);
        }
        crate::comparacao::ramo("cpn_6 estatuto pela lista");
        return (read_u8(e + 0x24) >> 1) & 0xf; // os bits 1..4 da entrada
    }
    if !cargo.is_null() {
        *cargo = read_u8(pessoa + staff::JOB_FOR_CLUB);
    }
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = contract_of(read_usize(G_CONTRACT_MANAGER), pessoa);
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
    let pelo_contrato = if cpn6_clube_do_contrato(c) == clube {
        true
    } else {
        // ponytail: o 3º argumento é empurrado como dword 1; o cargo vai num byte (a 0x53b620 só lê o byte)
        let humano: unsafe extern "C" fn(usize, u32, u32, usize, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
        let type_11_to_15: unsafe extern "C" fn(i8) -> u32 = core::mem::transmute(FN_DATABASE_IS_PLAYER_JOB);
        if humano(clube, 1, 1, 0, 0) != 0 && type_11_to_15(read_u8(pessoa + staff::JOB_FOR_CLUB) as i8) != 0 {
            let estimar: unsafe extern "stdcall" fn(usize, usize) -> u32 = core::mem::transmute(FN_SHORTLIST_MANAGER_ESTIMATE_SQUAD_STATUS);
            return estimar(pessoa, clube) as u8;
        }
        read_usize(pessoa + staff::CLUB_JOB) == clube || {
            let parceiro: unsafe extern "C" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
            read_usize(pessoa + staff::CLUB_JOB) == parceiro(clube, 0, 1)
        }
    };
    if !pelo_contrato {
        return 7;
    }
    let com_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let k = com_clube(read_usize(G_CONTRACT_MANAGER), pessoa, clube);
    if k == 0 {
        return 7;
    }
    crate::comparacao::ramo("cpn_6 estatuto pelo contrato");
    read_u8(k + 0x4f) >> 4 // o nibble alto do contrato
}

/// the estimate of the status the human club gives: the ratio between the person's reputation seen by the manager (1.5
/// × when on the human shortlist) and the club's, and the age
/// Original: `shortlist_manager_estimate_squad_status` (0x849d60, shortlist_manager.cpp, stdcall, ret 8).
///
/// [pt] { 0x849d60 } (stdcall, ret 8, shortlist_manager.cpp): `shortlist_manager_estimate_squad_status(pessoa, clube)`: o
/// estatuto que o clube humano daria. A reputação da pessoa vista pelo técnico do clube (0x53ca90, 0x5407e0; até 1:
/// 1, senão as duas chamadas de novo) e a do clube (+0x80, no mínimo 1); pessoa na lista do humano (0x7f7490 do
/// [0xb74448]): a reputação passa a ser o maior de 1,5 × ela e 0,75 × a do clube (truncado, 16 bits). A razão (limitada
/// a 0,1..2): acima de 1,5: 1; o float dela acima de 1,2 com o clube abaixo de 8500: 2; até 1,2 com a reputação até
/// 6500: 5 (acima de 0,6 e idade +0x18 abaixo de 23), 4 (24 ou mais) ou 6; senão 3. Só roda para o clube humano.
pub(crate) unsafe extern "stdcall" fn shortlist_manager_estimate_squad_status(pessoa: usize, clube: usize) -> u32 {
    let tecnico: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_CLUB_MANAGER);
    let reputacao: unsafe extern "C" fn(usize, usize, u32) -> u16 = core::mem::transmute(FN_DATABASE_GET_STAFF_REPUTATION_FOR_VIEWER);
    let mut rep = if reputacao(pessoa, tecnico(clube), 0) as i16 > 1 { reputacao(pessoa, tecnico(clube), 0) as i16 } else { 1 };
    let mut rc = core::ptr::read_unaligned((clube + club::REPUTATION) as *const i16);
    if rc <= 1 {
        rc = 1;
    }
    let t = tecnico(clube);
    let in_list: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_SCOUT_MANAGER_IS_PLAYER_SHORTLISTED);
    if in_list(read_usize(G_SCOUT_MANAGER), pessoa, t) != 0 {
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
    let idade = read_u8(pessoa + staff::YEAR_OF_BIRTH) as i8;
    if !(f > f64_em(0x968978)) && rep <= 0x1964 {
        if f > f64_em(0x968e48) && idade < 0x17 {
            return 5;
        }
        return if idade >= 0x18 { 4 } else { 6 };
    }
    3
}

/// in the club's 40 slots, a target with an invalid id becomes −1 and a target not at the club (contract nor +0x39) and
/// without +0x17 gets shortlist event 3 (0x843e10)
/// Original: `shortlist_manager_release_club_slot_targets` (0x844660, shortlist_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x844660 } (thiscall, ret 8, shortlist_manager.cpp): `shortlist_manager_release_club_slot_targets(clube,
/// entrada)`: nas 40 vagas de 0x25 bytes de [entrada], o alvo (+0xf) acima do último id de pessoa ([0xae2364] − 1)
/// vira −1; o alvo válido cujo contrato (0x4cf5a0, guardado em [0xae1080]; o de tipo 5 em +0x35 não conta) não é do
/// clube, nem o clube dele (+0x39) é o clube, e com o +0x17 da vaga em −1: o evento 3 da lista (0x843e10) com o tipo
/// "não é 11" do +0 da vaga (no byte baixo do endereço da vaga, como a original).
pub(crate) unsafe extern "thiscall" fn shortlist_manager_release_club_slot_targets(this: usize, clube: usize, entrada: usize) {
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let atualizar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_SHORTLIST_MANAGER_UPDATE_STAFF_STATUS);
    for k in 0..40usize {
        let o = k * 0x25;
        let v = read_usize(entrada).wrapping_add(o + 0xf);
        if read_usize(v) as i32 > (*(G_STAFF_COUNT as *const i32)).wrapping_sub(1) {
            core::ptr::write_unaligned(v as *mut i32, -1);
        }
        let id = read_usize(read_usize(entrada).wrapping_add(o + 0xf)) as i32;
        if id <= -1 {
            continue;
        }
        let p = read_usize(G_STAFF_TABLE).wrapping_add((id as usize).wrapping_mul(0x6e));
        let c = contract_of(read_usize(G_CONTRACT_MANAGER), p);
        *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
        let do_contrato = if c != 0 && read_u8(c + 0x35) & 0x3f != 5 { cpn6_clube_do_contrato(c) } else { 0 };
        if do_contrato == clube || read_usize(p + 0x39) == clube {
            continue;
        }
        let s = read_usize(entrada).wrapping_add(o);
        if read_usize(s + 0x17) as i32 != -1 {
            continue;
        }
        // setne al sobre o endereço da vaga: os 24 bits altos são os dele
        let tipo = ((s & !0xff) | (read_u8(s) != 0xb) as usize) as u32;
        crate::comparacao::ramo("cpn_6 alvo fora do clube");
        atualizar(this, p, clube, 3, tipo);
    }
}

/// Comparador: lado a lado pelo estatuto, pelo cargo de saída e por [0xae1080].
pub(crate) unsafe extern "thiscall" fn comparar_get_squad_status_for_club(this: usize, pessoa: usize, clube: usize, cargo: *mut u8) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, *mut u8) -> u8>(IDX_GET_SQUAD_STATUS_FOR_CLUB);
    let saida = if cargo.is_null() { 0xae1080 } else { cargo as usize };
    let reg = [(0xae1080usize, 4usize), (saida, 1)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, pessoa, clube, cargo), || get_squad_status_for_club(this, pessoa, clube, cargo));
    PLACAR.registrar(igual && r == n, || format!("get_squad_status_for_club({:#x}, {:#x}) original={} rust={} {:x?} x {:x?}", pessoa, clube, r, n, mo, mr));
    r
}

/// Comparador: pelo valor (o byte baixo; a original deixa a palavra de estado do x87 em ah).
pub(crate) unsafe extern "stdcall" fn comparar_shortlist_manager_estimate_squad_status(pessoa: usize, clube: usize) -> u32 {
    let r = orig::<unsafe extern "stdcall" fn(usize, usize) -> u32>(IDX_SHORTLIST_MANAGER_ESTIMATE_SQUAD_STATUS)(pessoa, clube);
    let n = shortlist_manager_estimate_squad_status(pessoa, clube);
    PLACAR.registrar(r as u8 == n as u8, || format!("shortlist_manager_estimate_squad_status({:#x}, {:#x}) original={:#x} rust={}", pessoa, clube, r, n));
    r
}

/// Comparador: só a original.
// ponytail: a 0x843e10 muda a lista do clube (e outras) a cada evento e não pode rodar duas vezes; a cópia tem o ramo
// e a prova é pela carreira (modo Rust contra a original).
pub(crate) unsafe extern "thiscall" fn comparar_shortlist_manager_release_club_slot_targets(this: usize, clube: usize, entrada: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_SHORTLIST_MANAGER_RELEASE_CLUB_SLOT_TARGETS)(this, clube, entrada)
}

/// without rule 0xe in the buyer nation, same nation, second nation or EU: yes; no national side: no; else the permit
/// (0x8c7950)
/// Original: `passes_work_permit_check` (0x84b180, shortlist_manager.cpp, stdcall, ret 0xc).
///
/// [pt] { 0x84b180 } (stdcall, ret 0xc, shortlist_manager.cpp): `passes_work_permit_check(jogador, _, comprador)`: o
/// jogador pode ir para o clube comprador? Sem nação (+0x1a): não. Nação do comprador (+0x53) sem a regra 0xe (+0x76),
/// a mesma nação, a 2ª nação (+0x1e) ou comunitário (0x53d740): sim. Sem seleção (+0x24) nem jogos por ela (+0x22):
/// não. Senão a permissão de trabalho (0x8c7950 sobre [0xde1f54], sem oferta, registro nem sorteio). O 2º argumento
/// não é usado.
pub(crate) unsafe extern "stdcall" fn passes_work_permit_check(jogador: usize, _atual: usize, comprador: usize) -> u8 {
    let nacao = read_usize(jogador + 0x1a);
    if nacao == 0 {
        return 0;
    }
    let destino = read_usize(comprador + 0x53);
    if *((destino + 0x76) as *const u8) != 0xe || nacao == destino || read_usize(jogador + 0x1e) == destino {
        return 1;
    }
    let comunitaria: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_STAFF_EU_NATIONAL);
    if comunitaria(jogador, destino) != 0 {
        return 1; // comunitário
    }
    if read_usize(jogador + 0x24) == 0 && *((jogador + 0x22) as *const u8) == 0 {
        return 0;
    }
    let permissao: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, u32) -> u8 = core::mem::transmute(FN_TRANSFER_MANAGER_IS_WORK_PERMIT_GRANTED);
    permissao(read_usize(G_TRANSFER_MANAGER), jogador, destino, 0, 0, 0)
}

/// Comparador: a 0x8c7950 (sem sorteio pedido) pode mexer no sorteio: lado a lado pelo estado dele, a cópia em
/// sem_contar.
pub(crate) unsafe extern "stdcall" fn comparar_passes_work_permit_check(jogador: usize, atual: usize, comprador: usize) -> u8 {
    let o = orig::<unsafe extern "stdcall" fn(usize, usize, usize) -> u8>(IDX_PASSES_WORK_PERMIT_CHECK);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&crate::aleatorio::REGIOES_DO_ESTADO, || o(jogador, atual, comprador),
        || crate::aleatorio::sem_contar(|| passes_work_permit_check(jogador, atual, comprador)));
    PLACAR.registrar(igual && r == n, || format!("passes_work_permit_check({:x}, {:x}) original={} rust={} {:x?} x {:x?}", jogador, comprador, r, n, mo, mr));
    r
}

/// with an interest of 3.5, the loan period and a return date within the contract, the window and 3 weeks from today,
/// makes the offer with 0, 50 or 100% of the wage by the reputation gap; otherwise −1
/// Original: `try_loan_offer_for_player` (0x842740, shortlist_manager.cpp, thiscall).
///
/// [pt] { 0x842740 } (thiscall, ret 8, shortlist_manager.cpp, quadro SEH sem destrutor): `Shortlist_Manager::
/// try_loan_offer_for_player(jogador, clube)`, pela avaliação das listas dos clubes (0x84a020): a vaga da oferta de
/// empréstimo, ou −1. Sem dono (contrato 0x4cf5a0, guardado em [0xae1080]): a vaga da lista (0x849ce0) volta ao estado 1
/// (0x843e10) e −1. O interesse (0x846920, com a situação da vaga) abaixo de 3,5: −1. O período do empréstimo (0x8d0590):
/// os meses até o fim e até o início (no mínimo 1); início depois do fim: −1; clube da nação 0x9cf408 fora de março a
/// setembro: −1. O interesse sem salário de 2,75 (fora das nações 0x9cf488, 0x9cf434, 0x9cf360) conta do início; senão do
/// fim. A data de volta (dia de hoje, no máximo 28; 0x549f70); sem poder encerrar o empréstimo (0x8d0530), depois do fim
/// do contrato, antes de 4 semanas do começo da janela da nação-chave (0x669360, [0xb63d8a]) ou a menos de 3 semanas de
/// hoje: −1. A porcentagem do salário (100 com reputação 2500 acima do dono, 0 com 1500 abaixo, senão 50) e a oferta
/// (0x8bed60).
pub(crate) unsafe extern "thiscall" fn try_loan_offer_for_player(this: usize, jogador: usize, clube: usize) -> i32 {
    let shortlist_entry: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(FN_SHORTLIST_MANAGER_FIND_CLUB_SHORTLIST_ENTRY);
    let entrada = shortlist_entry(this, clube, jogador, 0, 0);
    let c = cpw3_contrato(jogador);
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
    let dono = if c != 0 { cpe1_clube(read_usize(c + 4)) } else { 0 };
    if dono == 0 {
        if entrada != 0 {
            if CPW3_ENSAIO {
                return -3;
            }
            let estado: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_SHORTLIST_MANAGER_UPDATE_STAFF_STATUS);
            estado(this, jogador, clube, 1, 0);
        }
        return -1;
    }
    // ponytail: a original empurra a situação da vaga num registrador cujo resto é o endereço do clube dono (base +
    // id); a cópia reproduz o dword.
    let situacao = (read_usize(G_CLUB_TABLE).wrapping_add(read_usize(c + 4)) as u32 & !0xff) | ((read_u8(entrada + 0x24) >> 1) & 0xf) as u32;
    let f1 = cpw3_interesse(read_usize(read_usize(G_TRANSFER_MANAGER) + 0x856), jogador, clube, 0, 1, situacao);
    crate::comparacao::ramo("cpw_3 interesse no emprestimo");
    if !(f1 >= f64_em(0x968e38)) { // cpw3: o interesse mínimo
        return -1;
    }
    let mes_de: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_DATE_GET_MONTH);
    let m = mes_de(0xae2c90) as u8;
    let c = cpw3_contrato(jogador);
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
    let a2 = if c != 0 { cpe1_clube(read_usize(c + 4)) } else { 0 };
    let periodo: unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut i8, *mut u16, *mut i8, *mut u16) = core::mem::transmute(FN_TRANSFER_MANAGER_GET_LOAN_PERIOD);
    // ponytail: o mês do fim vai para a vaga do argumento do clube (o byte baixo), o resto fica; os outros são locais
    let (mut ini_mes, mut ini_ano, mut mes, mut ano) = (0i8, 0u16, clube as u8 as i8, 0u16);
    periodo(read_usize(G_TRANSFER_MANAGER), clube, a2, jogador, &mut ini_mes, &mut ini_ano, &mut mes, &mut ano);
    let hoje_ano = read_i16(G_CURRENT_DATE_YEAR) as u16;
    let m16 = m as i8 as i16;
    let ate_fim = (ano.wrapping_sub(hoje_ano) as i16).wrapping_mul(12).wrapping_sub(m16).wrapping_add(mes as i16);
    let ate_ini = (ini_ano.wrapping_sub(hoje_ano) as i16).wrapping_mul(12).wrapping_sub(m16).wrapping_add(ini_mes as i16);
    let inicio = if ate_ini >= 1 { ate_ini } else { 1 };
    if inicio > ate_fim {
        return -1;
    }
    let nacao = read_usize(clube + club::NATION);
    if nacao != 0 && read_usize(nacao) == read_usize(KEY_NATION_NORWAY) && (m as i8 >= 0xa || m as i8 <= 2) {
        return -1;
    }
    let f2 = cpw3_interesse(this, jogador, clube, 0, 0, 1);
    let mut meses = ate_fim as u8;
    crate::comparacao::ramo("cpw_3 conta do inicio");
    if f2 >= f64_em(0x9690a8) { // cpw3: o interesse para contar do início
        let n = read_usize(read_usize(clube + club::NATION));
        if n != read_usize(KEY_NATION_SPAIN) && n != read_usize(KEY_NATION_PORTUGAL) && n != read_usize(KEY_NATION_ITALY) {
            meses = inicio as u8;
        }
    }
    let mut mes_volta = meses.wrapping_add(m);
    // ponytail: o ano vai na vaga do argumento do jogador (a palavra baixa; o resto é o endereço do jogador), e a
    // porcentagem depois no byte baixo dela; a cópia reproduz o dword.
    let mut vaga = (jogador as u32 & 0xffff_0000) | hoje_ano as u32;
    if mes_volta as i8 > 0xb {
        let q = (mes_volta as i8 as i32 as u32) / 12;
        mes_volta = mes_volta.wrapping_add((q as u8).wrapping_mul(0xf4));
        vaga = vaga.wrapping_add(q);
    }
    let dia_do_mes: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_DATE_GET_DAY_OF_MONTH);
    let dia = dia_do_mes(0xae2c90) as i16;
    // a original relê o dia (segunda chamada) quando ele não passa de 28
    let dia = if dia > 0x1c { 0x1c } else { dia_do_mes(0xae2c90) as i16 as i32 };
    let data_dma: unsafe extern "thiscall" fn(*mut [u16; 4], i32, u32, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE_549F70);
    let mut volta = [0u16; 4];
    data_dma(&mut volta, dia, mes_volta as u32, vaga, 0xffff_ffff);
    let v = volta.as_ptr() as usize;
    let encerrar: unsafe extern "thiscall" fn(usize, usize, usize, u32, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_CAN_END_LOAN);
    if encerrar(read_usize(G_TRANSFER_MANAGER), clube, jogador, 0, v, 1) == 0 {
        return -1;
    }
    let c = cpw3_contrato(jogador);
    if c == 0 || cpw3_antes(c + 0x2d, v) {
        return -1;
    }
    let chave: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_KEY_NATION_INDEX_OF_CLUB);
    let k = chave(clube) as u8 as i8 as i32;
    let janela = (0xb63d8a + k * 72) as usize;
    let menos_semanas: unsafe extern "thiscall" fn(usize, *mut [u16; 4], u32) -> usize = core::mem::transmute(FN_DATE_SUB_WEEKS);
    let mais_semanas: unsafe extern "thiscall" fn(usize, *mut [u16; 4], u32) -> usize = core::mem::transmute(FN_DATE_ADD_WEEKS);
    let mut t = [0u16; 4];
    let r = menos_semanas(v, &mut t, 4);
    if cpw3_antes(r, janela) {
        return -1;
    }
    crate::comparacao::ramo("cpw_3 tres semanas");
    let r = mais_semanas(0xae2c90, &mut t, 3); // cpw3: as 3 semanas a partir de hoje
    if cpw3_antes(v, r) {
        return -1;
    }
    let dif = read_i16(clube + club::REPUTATION) as i32 - read_i16(dono + 0x80) as i32;
    let pct = if dif > 0x9c4 { 100 } else if dif <= -0x5dc { 0 } else { 50 };
    let vaga = (vaga & !0xff) | pct;
    if CPW3_ENSAIO {
        return -2;
    }
    let oferta: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_MAKE_LOAN_OFFER);
    oferta(read_usize(G_TRANSFER_MANAGER), jogador, clube, v, vaga, 1, 1) as i32
}

/// O comparador: a cópia em ensaio e a original a partir do mesmo estado ([0xae1080], o sorteio e as estáticas das
/// operações de data). A cópia recusou (−1): a original também, com a mesma memória. A cópia atualizaria a lista (−3):
/// a original devolve −1. A cópia faria a oferta (−2): a original não devolve −1.
pub(crate) unsafe extern "thiscall" fn comparar_try_loan_offer_for_player(this: usize, jogador: usize, clube: usize) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> i32>(IDX_TRY_LOAN_OFFER_FOR_PLAYER);
    let mut regioes = vec![(0xae1080usize, 4usize)];
    regioes.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    regioes.extend(CPQ5_ESTATICAS);
    let antes = crate::comparacao::Memoria::ler(&regioes);
    CPW3_ENSAIO = true;
    let n = crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| try_loan_offer_for_player(this, jogador, clube)));
    CPW3_ENSAIO = false;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    antes.escrever();
    let r = o(this, jogador, clube);
    let mo = crate::comparacao::Memoria::ler(&regioes);
    let igual = match n {
        -1 => r == -1 && mo == mr,
        -3 => r == -1,
        _ => r != -1,
    };
    PLACAR.registrar(igual, || format!("try_loan_offer_for_player(jogador {:#x}, clube {:#x}) original={} {:x?} rust={} {:x?}", read_usize(jogador), read_usize(clube), r, mo, n, mr));
    r
}

/// clears the bid marker of the club shortlist slots that target the player
/// Original: `Shortlist_Manager::clear_bid_markers_for_player` (0x84aeb0, shortlist_manager.cpp, thiscall).
///
/// [pt] { 0x84aeb0 } (thiscall, ret 8, shortlist_manager.cpp): com a lista do clube ativa (marca 1), tira a
/// marca de lance (+0x17 = −1) das 40 vagas (0x25 bytes cada) cujo alvo (+0xf) é o jogador `*jogador`.
pub(crate) unsafe extern "thiscall" fn clear_bid_markers_for_player(gerente: usize, clube: *const i32, jogador: *const i32) {
    let l = lista_do_clube(gerente, clube);
    if *((l + 4) as *const u8) != 1 {
        return;
    }
    for i in 0..40 {
        let v = core::ptr::read_unaligned(l as *const usize) + i * 0x25;
        if core::ptr::read_unaligned((v + 0xf) as *const i32) == *jogador {
            core::ptr::write_unaligned((v + 0x17) as *mut i32, -1);
        }
    }
}

pub(crate) unsafe extern "thiscall" fn comparar_clear_bid_markers_for_player(gerente: usize, clube: *const i32, jogador: *const i32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, *const i32, *const i32)>(IDX_CLEAR_BID_MARKERS_FOR_PLAYER);
    let l = lista_do_clube(gerente, clube);
    let reg = if *((l + 4) as *const u8) == 1 { vec![(core::ptr::read_unaligned(l as *const usize), 40 * 0x25)] } else { vec![(l, 9)] };
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(gerente, clube, jogador), || clear_bid_markers_for_player(gerente, clube, jogador));
    PLACAR.registrar(igual, || format!("clear_bid_markers_for_player({}, {}) {:x?} x {:x?}", *clube, *jogador, mo, mr));
}

/// adds to the list every shortlist entry (8 bytes) of the club, found by binary search
/// Original: `Shortlist_Manager::collect_club_shortlist_entries` (0x8420e0, shortlist_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x8420e0 } (thiscall, ret 8, shortlist_manager.cpp): `Shortlist_Manager::collect_club_shortlist_entries(clube,
/// lista)`: acrescenta (0x68d0d0, 8 bytes) à `lista` cada entrada da shortlist do clube (`*clube`), na ordem.
pub(crate) unsafe extern "thiscall" fn collect_club_shortlist_entries(this: usize, clube: *const u32, lista: usize) {
    let acrescentar: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_LLIST_ADD_TAIL);
    for e in cpj6_entradas(this, *clube) {
        acrescentar(lista, e, 8);
    }
}

/// Comparador: as entradas da cópia antes; a original acrescenta; confere os últimos itens da lista (8 bytes cada).
pub(crate) unsafe extern "thiscall" fn comparar_collect_club_shortlist_entries(this: usize, clube: *const u32, lista: usize) {
    let esperado: Vec<[u8; 8]> = cpj6_entradas(this, *clube).iter().map(|&e| core::ptr::read_unaligned(e as *const [u8; 8])).collect();
    let antes = cpd1_itens(lista).len();
    orig::<unsafe extern "thiscall" fn(usize, *const u32, usize)>(IDX_COLLECT_CLUB_SHORTLIST_ENTRIES)(this, clube, lista);
    let novos: Vec<[u8; 8]> = cpd1_itens(lista).iter().skip(antes).map(|&i| core::ptr::read_unaligned(i as *const [u8; 8])).collect();
    PLACAR.registrar(novos == esperado, || format!("collect_club_shortlist_entries({}) original {} itens, rust {}", *clube, novos.len(), esperado.len()));
}

/// the shortlist slot fee ceiling: club reputation times a factor (or -1)
/// Original: `shortlist_manager_max_fee_for_squad_status` (0x84b230, shortlist_manager.cpp, stdcall).
///
/// [pt] { 0x84b230 } (stdcall, ret 0xc, shortlist_manager.cpp; `posicoes` não é lido): o teto de preço
/// para a vaga de tipo `tipo` na lista de observados, pela reputação do clube (palavra +0x80): tipo 4
/// = x550 em Itália, Inglaterra e Espanha ([0x9cf360], [0x9cf2e4], [0x9cf488], nação +0x53), x475 em
/// Alemanha e França ([0x9cf314], [0x9cf300]), x330 nas outras; tipo 5 = x550; tipo 6 = x110; os
/// outros, -1.
pub(crate) unsafe extern "stdcall" fn shortlist_manager_max_fee_for_squad_status(clube: usize, _posicoes: u32, tipo: u32) -> i32 {
    let rep = core::ptr::read_unaligned((clube + club::REPUTATION) as *const i16) as i32;
    match tipo as i8 {
        4 => {
            let nacao = core::ptr::read_unaligned((clube + club::NATION) as *const usize);
            let eh = |gs: &[usize]| nacao != 0 && gs.iter().any(|&g| *(nacao as *const u32) == *(g as *const u32));
            if eh(&[KEY_NATION_ITALY, KEY_NATION_ENGLAND, KEY_NATION_SPAIN]) {
                rep.wrapping_mul(550)
            } else if eh(&[KEY_NATION_GERMANY, KEY_NATION_FRANCE]) {
                rep.wrapping_mul(475)
            } else {
                rep.wrapping_mul(330)
            }
        }
        5 => rep.wrapping_mul(550),
        6 => rep.wrapping_mul(110),
        _ => -1,
    }
}

pub(crate) unsafe extern "stdcall" fn comparar_shortlist_manager_max_fee_for_squad_status(clube: usize, posicoes: u32, tipo: u32) -> i32 {
    let r = orig::<unsafe extern "stdcall" fn(usize, u32, u32) -> i32>(IDX_SHORTLIST_MANAGER_MAX_FEE_FOR_SQUAD_STATUS)(clube, posicoes, tipo);
    let n = shortlist_manager_max_fee_for_squad_status(clube, posicoes, tipo);
    PLACAR.registrar(r == n, || format!("shortlist_manager_max_fee_for_squad_status(tipo {}) original={} rust={}", tipo as i8, r, n));
    r
}

/// is the (player, club or −1) pair in the shortlist pairs? bsearch with 0x757ea0
/// Original: `Shortlist_Manager::is_on_shortlist` (0x842080, shortlist_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x842080 } (thiscall, ret 8, shortlist_manager.cpp): `Shortlist_Manager::is_on_shortlist(jogador, clube)`: sem
/// vetor de pares ([this]) devolve 0; senão procura (bsearch 0x946464, [this+0x10] pares de 8 bytes, comparação
/// 0x757ea0) a chave (id do jogador, id do clube ou −1 sem clube). O al é 1 se achou; os 3 bytes altos do eax são os do
/// ponteiro achado (setne al sobre o retorno do bsearch), e a cópia os reproduz.
pub(crate) unsafe extern "thiscall" fn is_on_shortlist(this: usize, jogador: usize, clube: usize) -> u32 {
    let v = read_usize(this);
    if v == 0 {
        return 0;
    }
    let chave: [u32; 2] = [read_usize(jogador) as u32, if clube != 0 { read_usize(clube) as u32 } else { u32::MAX }]; // sem clube: −1
    let bsearch: unsafe extern "C" fn(*const [u32; 2], usize, usize, usize, usize) -> usize = core::mem::transmute(FN_CRT_BSEARCH);
    let p = bsearch(&chave, v, read_usize(this + 0x10), 8, 0x757ea0);
    (p as u32 & !0xff) | (p != 0) as u32
}

/// the reputation a club aims for in signings, adjusted by manager and chairman, between the club floor and 10000
/// Original: `Shortlist_Manager::update_club_target_reputation` (0x83bf40, shortlist_manager.cpp, thiscall).
///
/// [pt] { 0x83bf40 } (thiscall, ret 4, shortlist_manager.cpp): a reputação que o clube procura nas
/// contratações, gravada na palavra +5 do registro dele: +0x73 / 10 + 4500; se não for de humano
/// (0x600a50 com 1, 1), o técnico (+0xcf) ajusta por (+0x57 − ficha +0x1d) × 0,0125; o presidente
/// (+0xbf, com ficha) por (+0x57 − ficha +0x1d) / 60 e por (ficha +0x20 − 10) × 0,025 (cada um
/// multiplica 1 + fator, truncando). Nunca abaixo de +0x80 nem acima de 10000. Devolve o valor.
pub(crate) unsafe extern "thiscall" fn update_club_target_reputation(this: usize, clube: usize) -> i32 {
    let humano: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let r = registro_da_lista_de_compras(this, clube);
    let alvo = (r + 5) as *mut i16;
    let u = |p: usize| core::ptr::read_unaligned(p as *const usize);
    let w = || core::ptr::read_unaligned(alvo) as i32;
    let por = |dif: i32, k: usize, v: i32| ftol((dif as f64 * f64_em(k) + f64_em(0x967890)) * v as f64);
    core::ptr::write_unaligned(alvo, (core::ptr::read_unaligned((clube + club::ATTENDANCE) as *const i32) / 10 + 0x1194) as i16);
    if humano(clube, 1, 1, 0, 0) == 0 {
        let tecnico = u(clube + club::MANAGER);
        if tecnico != 0 {
            let dif = byte(tecnico, 0x57) as i32 - byte(u(tecnico + 0x69), 0x1d) as i32;
            core::ptr::write_unaligned(alvo, por(dif, 0x96a740, w()) as i16);
        }
    }
    let p = u(clube + club::CHAIRMAN);
    if p != 0 && u(p + 0x69) != 0 {
        let dif = byte(p, 0x57) as i32 - byte(u(p + 0x69), 0x1d) as i32;
        let v = por(dif, 0x968958, w()) as i16;
        core::ptr::write_unaligned(alvo, v);
        let ficha = u(u(clube + club::CHAIRMAN) + 0x69);
        core::ptr::write_unaligned(alvo, por(byte(ficha, 0x20) as i32 - 10, 0x967878, v as i32) as i16);
    }
    let piso = core::ptr::read_unaligned((clube + club::REPUTATION) as *const i16);
    let v = core::ptr::read_unaligned(alvo);
    let v = if piso > v { piso } else if v > 0x2710 { 0x2710 } else { v };
    core::ptr::write_unaligned(alvo, v);
    v as i32
}

pub(crate) unsafe extern "thiscall" fn comparar_update_club_target_reputation(this: usize, clube: usize) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> i32>(IDX_UPDATE_CLUB_TARGET_REPUTATION);
    let reg = [(registro_da_lista_de_compras(this, clube) + 5, 2)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube), || update_club_target_reputation(this, clube));
    PLACAR.registrar(igual && r == n, || format!("update_club_target_reputation({}) original={} rust={} {:x?} x {:x?}", *(clube as *const i32), r, n, mo, mr));
    r
}
