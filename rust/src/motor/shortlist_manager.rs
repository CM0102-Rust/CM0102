use super::*;

/// Returns the entry of person `pessoa`'s shortlist (40 entries of 37 bytes at [this+0xdc] + id x 9) whose +0xf (or,
/// with `tambem_13`, +0x13) is the id of `alvo`; with `nao_11` only entries of type (+0) other than 11 match, without
/// it only type 11. None: 0.
///
/// [pt] { 0x849ce0 } (ret 0x10, shortlist_manager.cpp): a entrada da lista da pessoa (40 entradas de
/// 37 bytes no ponteiro em [this+0xdc] + id x 9) cujo +0xf (ou, com `tambem_13`, o +0x13) e o id
/// de `alvo`; com `nao_11` so serve a de tipo (+0) diferente de 11, sem ele so a de tipo 11. Nenhuma: 0.
pub(crate) unsafe extern "thiscall" fn shortlist_entry(this: usize, person: usize, target: usize, not_11: u32, tambem_13: u32) -> usize {
    let list = (*((this + 0xdc) as *const usize)).wrapping_add((*(person as *const i32) as usize).wrapping_mul(9));
    let base = *(list as *const usize);
    let id = *(target as *const u32);
    for k in 0..40usize {
        let e = base.wrapping_add(k * 37);
        let home = *((e + 0xf) as *const u32) == id || (tambem_13 != 0 && *((e + 0x13) as *const u32) == id);
        if home && (not_11 != 0) == (*(e as *const u8) != 11) {
            return e;
        }
    }
    0
}









/// CONTRACT_MANAGER get_contract (0x4cf5a0) for a person, also stored in G_CONTRACT_LOOKUP_RESULT.
///
/// [pt] O contrato da pessoa (0x4cf5a0: push pessoa; mov ecx, [0xae1050] -> thiscall, ret 4), guardado em [0xae1080].
pub(crate) unsafe fn contract_of_staff_cached(person: usize) -> usize {
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = contrato(read_u32_as_usize(G_CONTRACT_MANAGER), person);
    write_u32_from_usize(G_CONTRACT_LOOKUP_RESULT, c);
    c
}

/// Club of the person's contract (via the cached lookup), or 0 without contract.
///
/// [pt] O clube do contrato da pessoa (o tipo 5 conta; sem contrato, 0), com o contrato em [0xae1080].
pub(crate) unsafe fn club_of_staff_contract(person: usize) -> usize {
    let c = contract_of_staff_cached(person);
    if c == 0 { 0 } else { transfer_manager_club_by_id(read_u32_as_usize(c + 4)) }
}

/// Byte +0xb of the person's 79-byte staff state record (G_STAFF_STATE_TABLE).
///
/// [pt] A pessoa tem a pessoa no registro de 79 bytes em [0xae2d0c]: o byte +0xb.
pub(crate) unsafe fn staff_state_byte_0b(person: usize) -> u8 {
    read_u8(read_u32_as_usize(G_STAFF_STATE_TABLE).wrapping_add(read_u32_as_usize(person).wrapping_mul(0x4f)) + 0xb)
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
    let more: unsafe extern "thiscall" fn(usize, *mut [u32; 2], u32) -> usize = core::mem::transmute(FN_DATE_ADD_DAYS);
    let new_contract: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_STAFF_CONTRACT_STAFF_CONTRACT);
    let is_national_team: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let human: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let manager: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_CLUB_MANAGER);
    let responde: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_ON_HOLIDAY);
    let rule: unsafe extern "thiscall" fn(usize, u32, usize) -> u8 = core::mem::transmute(FN_HUMAN_MANAGER_GET_HOLIDAY_OPTION);
    let finances: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FINANCETABLE_GET_CLUB_FINANCE);
    let slots_with_play: unsafe extern "thiscall" fn(usize, usize, u32) -> u8 = core::mem::transmute(FN_SHORTLIST_MANAGER_COUNT_SLOTS_WITH_BIDS);
    let count_squad: unsafe extern "C" fn(usize, u32, u32) -> u8 = core::mem::transmute(FN_DATABASE_COUNT_CLUB_SQUAD);
    let future: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_FUTURE_TRANSFER);
    let acertado: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_AGREED_CONTRACT_CLUB);
    let chegar: unsafe extern "thiscall" fn(usize, usize, usize, *mut u16, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_CAN_CLUB_BID_FOR_PLAYER);
    let has_offer: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_CLUB_HAS_OFFER_FOR_PLAYER);
    let swaps: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_EXCHANGE_OFFERS_FOR_PLAYER);
    let pode_contratar: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, u32, *mut u16) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_CAN_SIGN_PLAYER);
    let lesao: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_INJURY_GET_PLAYER_INJURY);
    let sob_offer: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_IS_PLAYER_UNDER_OFFER);
    let desejado: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32, u32) -> i32 = core::mem::transmute(FN_STAFF_CONTRACT_SET_TO_DESIRED_CONTRACT);
    let ceiling: unsafe extern "thiscall" fn(usize, usize, u32) -> i32 = core::mem::transmute(FN_FINANCE_GET_MAX_WAGE);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let new_offer: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_TRANSFER_OFFER_TRANSFER_OFFER);
    let destroy_offer: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_TRANSFER_OFFER_DESTRUCTOR_TRANSFER_OFFER);
    let fill: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, *mut usize) -> u32 = core::mem::transmute(FN_TRANSFER_OFFER_BUILD_BID);
    let fazer_lance: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_ADD_TRANSFER_OFFER);
    let contract_offer: unsafe extern "thiscall" fn(usize, usize, i32, u32, u32, u32, u32, u32) = core::mem::transmute(FN_CONTRACT_MANAGER_ADD_CONTRACT_OFFER);
    let has_offer_of_contract: unsafe extern "thiscall" fn(usize, i32, i32) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_FIND_CONTRACT_OFFER);
    let contract_with: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let entry: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(FN_SHORTLIST_MANAGER_FIND_CLUB_SHORTLIST_ENTRY);
    let renovar: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32, u32) = core::mem::transmute(FN_CONTRACT_MANAGER_OFFER_CONTRACT);
    let free: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_OPERATOR_DELETE);
    let status: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u8 = core::mem::transmute(FN_SHORTLIST_MANAGER_UPDATE_STAFF_STATUS);
    let transfer_manager_can_loan_player: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_CAN_LOAN_PLAYER);
    let pedir_emprestimo: unsafe extern "thiscall" fn(usize, usize, usize) -> i32 = core::mem::transmute(FN_SHORTLIST_MANAGER_TRY_LOAN_OFFER_FOR_PLAYER);
    let mut week = [0u32; 2];
    more(G_CURRENT_DATE, &mut week, 7);
    let pdata = week.as_mut_ptr() as usize;
    let mut contrato0 = [0u8; 0x50];
    new_contract(contrato0.as_mut_ptr() as usize);
    if is_national_team(clube) != 0 || human(clube, 0, 1, 0, 0) != 0 {
        return;
    }
    if human(clube, 1, 1, 0, 0) != 0 && responde(read_u32_as_usize(G_HUMAN_MANAGER), manager(clube)) != 0
        && rule(read_u32_as_usize(G_HUMAN_MANAGER), 4, manager(clube)) == 1
    {
        return;
    }
    let reg = read_u32_as_usize(this + shortlist_manager_fields::CLUB_SHORTLISTS).wrapping_add(read_u32_as_usize(clube).wrapping_mul(9));
    if read_u8(reg + 4) != 1 || read_u8(reg + 8) >= 100 || finances(read_u32_as_usize(G_FINANCE_TABLE), clube) == 0 {
        return;
    }
    let mut count = slots_with_play(this, clube, 0xb) as i8;
    count = count.wrapping_add(count_squad(clube, 0, 0) as i8);
    // simplification: the reason (a local word) is not initialized by the original; 0x8ce990 and 0x8ced40 write it before it is read
    let mut reason: u16 = 0;
    let gm = read_u32_as_usize(G_TRANSFER_MANAGER);
    for s in 0..0x28usize {
        let off = s * 0x25;
        let slot = |k: usize| read_u32_as_usize(reg).wrapping_add(off) + k;
        let player = read_u8(slot(0)) == 0xb;
        if count >= 0x32 && player {
            continue;
        }
        let id = read_u32_as_usize(slot(0xf)) as i32;
        if id <= -1 || read_u32_as_usize(slot(0x17)) as i32 != -1 {
            continue;
        }
        let p = transfer_manager_staff_by_id(id as usize);
        if club_of_staff_contract(p) == clube {
            continue;
        }
        if !player {
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
            match read_u8(slot(0)) {
                8 if !livre(0x19f, 5) => continue,
                9 if !livre(0x1b3, 7) => continue,
                10 if !livre(0x1cf, 3) => continue,
                6 | 7 if read_u32_as_usize(clube + club::ASSISTANT_MANAGER) != 0 => continue,
                _ => {}
            }
        }
        if staff_state_byte_0b(p) & 0x30 != 0 && player {
            continue;
        }
        if future(gm, p, 0) != 0 || acertado(gm, p, 0) != 0 {
            continue;
        }
        if read_u32_as_usize(p + 0x24) != 0 && read_u8(p + 0x28) == 5 {
            continue;
        }
        if read_u8(slot(0x24)) & 1 == 0 {
            // the purchase slot
            if read_i16(slot(1)) <= -1 && player {
                continue;
            }
            if chegar(gm, p, clube, &mut reason, 0) == 0 && reason != 9 {
                continue;
            }
            if has_offer(gm, p, clube) != 0 || swaps(gm, p, 0) != 0 {
                continue;
            }
            let mut v = read_u32_as_usize(slot(0xb)) as i32;
            if read_u8(slot(0x24)) & 0x20 != 0 {
                v = v.wrapping_shl(1);
                if v <= 0x4e20 {
                    v = 0x4e20;
                }
            }
            let league = read_u32_as_usize(clube + club::DIVISION);
            if league != 0 {
                let k = read_u32_as_usize(league) as i32;
                if k > -1 && k < read_u32_as_usize(G_CLUB_COMP_COUNT) as i32 && read_u32_as_usize(read_u32_as_usize(G_COMP_OBJECT_TABLE).wrapping_add(k as usize * 4)) != 0 {
                    if read_i16(clube + club::REPUTATION) > 0x128e {
                        if count < 0x19 {
                            v = v.wrapping_mul(3);
                        } else if count < 0x23 {
                            v = v.wrapping_mul(2);
                        }
                    } else if count < 0x19 {
                        crate::comparacao::ramo("cqe_3 dobro pela liga");
                        v = v.wrapping_mul(2); // cqe3: o dobro pela liga ativa
                    }
                }
            }
            if read_u32_as_usize(slot(0x13)) as i32 > -1 && read_u8(slot(0)) == 0xb {
                let current = club_of_staff_contract(p);
                if pode_contratar(gm, clube, pdata, p, current, 1, &mut reason) == 0 && reason != 9 {
                    let x = read_u32_as_usize(slot(0xb)) as i32;
                    v = v.wrapping_add(if x > 0x2710 { x } else { 0x2710 });
                }
                let sub = transfer_manager_staff_by_id(read_u32_as_usize(slot(0x13)));
                let les = read_u32_as_usize(G_INJURY);
                if lesao(les, sub) != 0 {
                    let a = read_i16(lesao(les, sub) + 0x2c) as i32;
                    let b = read_i16(lesao(les, sub) + 0x2c) as i32;
                    v = v.wrapping_add(a.wrapping_mul(b).wrapping_mul(read_i16(clube + club::REPUTATION) as i32 / 500));
                }
                let f = read_u32_as_usize(sub + 0x61);
                if f != 0 {
                    let x = read_u8(f + 0x45) as i8 as f64;
                    v = v.wrapping_add(match_pl_ftol((read_f64_at(0x968940) - x * read_f64_at(0x969020)) * v as f64)); // 0x968940 = 1.5, 0x969020 = 0.03333333333
                }
                if sob_offer(gm, sub, 0) != 0 {
                    let x = read_u32_as_usize(slot(0xb)) as i32 as f64 * read_f64_at(0x968940); // 0x968940 = 1.5
                    v = v.wrapping_add(match_pl_ftol(if x > read_f64_at(0x968910) { x } else { read_f64_at(0x968910) })); // 0x968910 = 10000
                }
            }
            let c = contract_of_staff_cached(p);
            if c != 0 && transfer_manager_club_by_id(read_u32_as_usize(c + 4)) != 0 && player {
                let seller = club_of_staff_contract(p);
                if human(seller, 0, 1, 0, 0) != 0 || read_u32_as_usize(p + 0x39) == clube {
                    let mut contrato = [0u8; 0x50];
                    let pc = contrato.as_mut_ptr() as usize;
                    new_contract(pc);
                    let e = slot(0);
                    // simplification: the status goes with the high bytes of the slot pointer (as in the original); the role, with those of the
                    // edx left by 0x600a50 (garbage): 0x85fc50 receives a char, only the byte counts
                    let st = (e as u32 & 0xffff_ff00) | ((read_u8(e + 0x24) >> 1) & 0xf) as u32;
                    let r = desejado(pc, p, clube, u32::MAX, st, read_u8(e) as u32, 0, 0);
                    if r as f64 == read_f64_at(0x9689a0) { // 0x9689a0 = 0
                        continue;
                    }
                    // simplification: the role goes with the high bytes of the result and of fnstsw; 0x5968d0 reads only the byte
                    let t = ceiling(finances(read_u32_as_usize(G_FINANCE_TABLE), clube), p, ((read_u8(slot(0x24)) >> 1) & 0xf) as u32);
                    if (t as f64 * read_f64_at(0x968de8)) < read_u32_as_usize(pc + 0xc) as i32 as f64 { // 0x968de8 = 1.3
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
            write_u8(slot(0x24), read_u8(slot(0x24)) & 0xdf);
            let mut offer = [0u8; 0x44];
            let po = offer.as_mut_ptr() as usize;
            new_offer(po);
            let mut terms: usize = 0;
            if fill(po, p, clube, slot(0), read_u32_as_usize(slot(7)) as u32, 0, &mut terms) == 0 {
                status(this, p, clube, 1, (read_u8(slot(0)) != 0xb) as u32);
            } else {
                fazer_lance(gm, po, 0);
                if terms != 0 {
                    // simplification: the job goes with the high bytes of the slot vector pointer (as in the original)
                    let job = (read_u32_as_usize(reg) as u32 & 0xffff_ff00) | read_u8(slot(0)) as u32;
                    contract_offer(read_u32_as_usize(G_CONTRACT_MANAGER), terms, read_u32_as_usize(po + 0x20) as i32, read_u32_as_usize(po + 0x24) as u32, job, 0, 1, 0);
                    let c = contract_of_staff_cached(p);
                    let seller = if c == 0 { 0 } else { transfer_manager_club_by_id(read_u32_as_usize(c + 4)) };
                    if seller != 0 && human(seller, 0, 1, 0, 0) == 0
                        && has_offer_of_contract(read_u32_as_usize(G_CONTRACT_MANAGER), read_u32_as_usize(p) as i32, read_u32_as_usize(seller) as i32) == 0
                    {
                        let cv = contract_with(read_u32_as_usize(G_CONTRACT_MANAGER), p, seller);
                        if cv != 0 {
                            let e = (read_u8(cv + 0x4f) >> 4) as i8;
                            let factor = match e {
                                1 | 2 if entry(this, seller, p, 0, 0) != 0 => {
                                    let rep = read_i16(clube + club::REPUTATION);
                                    let div = if rep < 1 { 1 } else { rep as i32 };
                                    let f = (staff_state_byte_0b(p) & 0xf) as f64 * read_f64_at(0x967880) + rep as i32 as f64 / div as f64; // 0x967880 = 0.1
                                    Some(if read_f64_at(0x967890) <= f { if read_f64_at(0x968dc0) < f { 1.4 } else { f } } else { 1.0 }) // 0x967890 = 1, 0x968dc0 = 1.4
                                }
                                3 | 4 | 5 if entry(this, seller, p, 0, 0) != 0 => {
                                    let rep = read_i16(clube + club::REPUTATION) as i32;
                                    let div = if rep.wrapping_mul(2) >= 1 { rep.wrapping_mul(2) } else { 1 };
                                    let f = (staff_state_byte_0b(p) & 0xf) as f64 * read_f64_at(0x967880) + rep as f64 / div as f64 + read_f64_at(0x968968); // 0x967880 = 0.1, 0x968968 = 0.5
                                    Some(if read_f64_at(0x967890) <= f { if read_f64_at(0x968978) < f { 1.2 } else { f } } else { 1.0 }) // 0x967890 = 1, 0x968978 = 1.2
                                }
                                _ => None,
                            };
                            if let Some(f) = factor {
                                let b = f.to_bits();
                                renovar(read_u32_as_usize(G_CONTRACT_MANAGER), p, 0, 0, u32::MAX, b as u32, (b >> 32) as u32, 2);
                            }
                        }
                    }
                    free(terms);
                }
                write_u32_from_usize(slot(0x17), read_u32_as_usize(po + 0x20));
                if player {
                    count = count.wrapping_add(1);
                }
            }
            destroy_offer(po);
        } else {
            // the loan slot
            let n = read_u32_as_usize(clube + club::NATION);
            if n != 0 && read_u32_as_usize(n) == read_u32_as_usize(KEY_NATION_FRANCE) && read_u8(clube + club::PROFESSIONAL_STATUS) == 3 {
                continue;
            }
            if transfer_manager_can_loan_player(gm, clube, p, 0) == 0 {
                continue;
            }
            let c = contract_of_staff_cached(p);
            if c != 0 && transfer_manager_club_by_id(read_u32_as_usize(c + 4)) != 0 && human(club_of_staff_contract(p), 0, 1, 0, 0) != 0 {
                let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
                if read_u8(contrato(read_u32_as_usize(G_CONTRACT_MANAGER), p) + 0x4e) & 2 == 0 {
                    continue;
                }
            }
            if has_offer(gm, p, clube) != 0 || swaps(gm, p, 0) != 0 {
                continue;
            }
            crate::comparacao::ramo("cqe_3 sorteio do empréstimo");
            let r1 = sorteio(read_i16(clube + club::REPUTATION) as i32 / 10); // cqe3: o sorteio do empréstimo
            let r2 = sorteio(read_u32_as_usize(slot(0xb)) as i32);
            if r2 <= r1 {
                let r1 = sorteio((read_i16(clube + club::REPUTATION) as i32).wrapping_shl(1));
                let r2 = sorteio(read_u32_as_usize(slot(0xb)) as i32);
                if r2 <= r1 {
                    continue;
                }
            }
            let k = pedir_emprestimo(this, p, clube);
            write_u32_from_usize(slot(0x17), k as u32 as usize);
            if read_u32_as_usize(slot(0x17)) as i32 > -1 {
                count = count.wrapping_add(1);
            } else {
                // simplification: the setne goes with the high bytes of the slot pointer (as in the original); 0x843e10 reads the byte
                let e = slot(0);
                status(this, p, clube, 1, (e as u32 & 0xffff_ff00) | (read_u8(e) != 0xb) as u32);
            }
        }
    }
}









/// Days from today to a date given by year and day pointers, as the original: (year - this year) x 365 - today's day +
/// day.
///
/// [pt] A conta dos dias entre o ano e o dia de hoje ([0xae2c92], [0xae2c90]) e os de uma data (ano em `ano`, dia em
/// `dia`), como a original: (ano − ano de hoje) × 365 − dia de hoje + dia.
pub(crate) unsafe fn days_until_date(year: usize, day: usize) -> i32 {
    ((read_u16(year) as i32 - read_u16(G_CURRENT_DATE_YEAR) as i32) * 365 - read_i16(G_CURRENT_DATE) as i32) + read_i16(day) as i32
}

/// True if today minus 150 days is after the date stored at manager +0x3e (day) / +0x40 (year).
///
/// [pt] Hoje menos 150 dias (0x54c8e0: push 0x96; push saída; mov ecx, 0xae2c90 -> thiscall, ret 8) já passou da data
/// guardada em +0x3e (dia) / +0x40 (ano) do técnico `t`: ano maior (sem sinal), ou o mesmo ano e o dia maior (com
/// sinal).
pub(crate) unsafe fn is_150_days_since(t: usize) -> bool {
    let minus: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) = core::mem::transmute(FN_DATE_SUB_DAYS);
    let mut d = [0u8; 8];
    minus(G_CURRENT_DATE, &mut d, 0x96);
    let day = i16::from_le_bytes([d[0], d[1]]);
    let year = u16::from_le_bytes([d[2], d[3]]);
    let year_t = read_u16(t + 0x40);
    year > year_t || (year == year_t && day > read_i16(t + 0x3e))
}

/// Distance between two cities (f64 coordinates +0x23, +0x2b) x 55.
///
/// [pt] A distância entre as cidades `a` e `b` (coordenadas f64 em +0x23 e +0x2b), × 55.
pub(crate) unsafe fn shortlist_manager_city_distance(a: usize, b: usize) -> f64 {
    let dx = read_f64(a + 0x2b) - read_f64(b + 0x2b);
    let dy = read_f64(a + 0x23) - read_f64(b + 0x23);
    (dy * dy + dx * dx).sqrt() * read_f64(0x96d1b8)
}

/// The original's max: x if x > y, else y.
///
/// [pt] O máximo da original nas contas de reputação: `x` se ele passa `y`, senão `y`.
pub(crate) fn max_f64(x: f64, y: f64) -> f64 {
    if x > y { x } else { y }
}

/// Truncated average of the three reputations of a player record (+9, +0xb, +0xd).
///
/// [pt] A média das três reputações da ficha do jogador (+9, +0xb, +0xd), truncada.
pub(crate) unsafe fn average_reputation3(f: usize) -> i32 {
    (read_i16(f + 0xd) as i32 + read_i16(f + 0xb) as i32 + read_i16(f + 9) as i32) / 3
}

/// Club reputation adjusted by stadium city attraction.
///
/// [pt] A reputação do clube pelo estádio: sem estádio ou sem cidade (+0x69, +0x38), trunc(rep × 2/7); com a cidade de
/// importância 0 (+0x33), rep / 2; senão o maior entre rep × 2/7 e rep × importância / 15, truncado.
pub(crate) unsafe fn reputation_by_stadium(clube: usize) -> i32 {
    let rep = read_i16(clube + club::REPUTATION) as i32;
    let e = read_usize(clube + club::STADIUM);
    let cidade = if e != 0 { read_usize(e + 0x38) } else { 0 };
    if cidade == 0 {
        return match_pl_ftol(rep as f64 * read_f64(0x96d5e0));
    }
    let imp = read_i8(cidade + city::CITY_ATTRACTION);
    if imp == 0 {
        return rep / 2;
    }
    let q = (imp as i32).wrapping_mul(rep) / 15;
    let x = rep as f64 * read_f64(0x96d5e0);
    match_pl_ftol(max_f64(x, q as f64))
}

/// Club reputation adjusted by its division's reputation and attendance.
///
/// [pt] A reputação do clube pela divisão (+0x57, o +0x69 dela): sem divisão, a própria (o resto do registrador vem do
/// ponteiro; só a palavra conta); com reputação de 7500 e público acima de 35000 (+0x73), o maior entre a reputação e
/// 500 × a da divisão; senão o maior entre rep/2 + 250 × a da divisão e 500 × a da divisão.
pub(crate) unsafe fn reputation_by_division(clube: usize) -> i16 {
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

/// Reputation from the club's continental competition and its position there; else min(0.8 x division value, 7000);
/// returns (value, record).
///
/// [pt] A reputação pela competição do clube (+0x1db; a da lista [0xae2d14] + id × 6 quando ela é negativa ou é a
/// [0x9cf6f8] com reputação de 5000): o registro de [0xae23d0] (0x6b bytes; 15000 para a [0x9cf6e8], senão 600 × +0x69
/// em 16 bits) e, com o clube na tabela dela (0x4a4850) em 1º a 16º sem a marca +4, (1,32 − 0,02 × posição) × isso.
/// Sem registro: o mínimo entre 0,8 × `pela_divisao` e 7000. Devolve (valor, registro).
pub(crate) unsafe fn reputation_by_competition(clube: usize, by_division: i16) -> (i32, usize) {
    let rep = read_i16(clube + club::REPUTATION);
    let mut idx = read_usize(clube + club::EURO_FLAG) as i32;
    let mut has = true;
    if idx <= -1 || (idx == read_usize(KEY_CLUB_COMP_INTER_TOTO_CUP) as i32 && rep >= 0x1388) {
        idx = read_usize(read_usize(G_CLUB_STATE_TABLE).wrapping_add(read_usize(clube).wrapping_mul(6))) as i32;
        if idx <= -1 {
            has = false;
        }
    }
    let mut reg = 0usize;
    if has {
        reg = read_usize(G_CLUB_COMP_TABLE).wrapping_add((idx as usize).wrapping_mul(0x6b));
        if reg != 0 {
            let id = read_usize(reg);
            let s: i16 = if id == read_usize(KEY_CLUB_COMP_UEFA_EUROPEAN_CHAMPIONS_LEAGUE) { 15000 } else { read_i16(reg + 0x69).wrapping_mul(600) };
            // 0x4a4850: push clube; mov ecx, [[0xadadfc] + 4 × id] + 8 -> thiscall, ret 4
            let find: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_COMP_STATS_FIND_CLUB_ENTRY);
            let r = find(read_usize(read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(id.wrapping_mul(4))) + 8), clube);
            if r != 0 && read_u8(r + 4) == 0 && (1..=0x10).contains(&read_u8(r + 0x72)) {
                let p = read_u8(r + 0x72) as f64;
                return (match_pl_ftol((read_f64(0x96af38) - p * read_f64(0x968a98)) * s as f64), reg);
            }
            return (s as i32, reg);
        }
    }
    let mut x = by_division as f64 * read_f64(0x9689b0);
    if !(x < read_f64(0x96d470)) {
        x = read_f64(0x96d470);
    }
    (match_pl_ftol(x), reg)
}

/// Reputation as seen through the manager-manager confidence row of the club.
///
/// [pt] A reputação vista pelo técnico ([0xb67a44] + id × 0x49: (+0xa + +8) × 0,25 + +6 × 0,5): abaixo de 5000, o maior
/// entre r / (5 − 0,0008 v) e r − 1000 × (5 − 0,0008 v); senão o menor entre r × (0,5 + 0,0001 v) e r + 1000 × isso.
pub(crate) unsafe fn reputation_by_manager(clube: usize, r: i16) -> i32 {
    let t = read_usize(read_usize(G_MANAGER_MANAGER)).wrapping_add(read_usize(clube).wrapping_mul(0x49));
    let v = match_pl_ftol((read_i16(t + 0xa) as f64 + read_i16(t + 8) as f64) * read_f64(0x9689e0) + read_i16(t + 6) as f64 * read_f64(0x968968)) as i16;
    let dd = r as f64;
    if v < 0x1388 {
        let t = read_f64(0x9688a0) - v as f64 * read_f64(0x96a618);
        let q1 = dd / t;
        let m = dd - t * read_f64(0x968908);
        match_pl_ftol(if q1 > m { q1 } else { m })
    } else {
        let t = v as f64 * read_f64(0x96a5c0) + read_f64(0x968968);
        let q1 = t * dd;
        let m = t * read_f64(0x968908) + dd;
        match_pl_ftol(if q1 < m { q1 } else { m })
    }
}

/// Reputation ceiling by financial band: band 1 max(1.25r, r+750), band 2 max(1.5r, r+1500), else r.
///
/// [pt] O teto da reputação pelas finanças (0x5a0590 e 0x5986b0): faixa 1, o maior entre 1,25 r e r + 750; faixa 2, o maior
/// entre 1,5 r e r + 1500.
pub(crate) fn reputation_cap_by_finances(band: i8, r: i16) -> i16 {
    match band {
        1 => match_pl_ftol(max_f64(r as f64 * 1.25, (r as i32 + 0x2ee) as f64)) as i16,
        2 => match_pl_ftol(max_f64(r as f64 * 1.5, (r as i32 + 0x5dc) as f64)) as i16,
        _ => r,
    }
}

/// min(h / 2, h - 2000) on the word.
///
/// [pt] min(h / 2, h − 2000) sobre a palavra, a correção pelo +0x64 do clube (diferente de 1; e mais uma vez com 3).
pub(crate) fn half_or_minus_2000(h: i16) -> i16 {
    let a = h as i32 / 2;
    let e = h as i32 - 2000;
    (if a < e { a } else { e }) as i16
}

/// Fluency factor max(F x 0.1, L / (12000 - 200F)) + 0.2 from the person's fluency in the club nation's language;
/// returns (factor, F).
///
/// [pt] A fluência: max(F × 0,1, L / (12000 − 200 F)) + 0,2, com L = (s + 5000)² × 1/15000 e F a fluência da pessoa no
/// idioma da nação do clube (0x7c6830: push 1; push idioma; push clube; push pessoa; mov ecx, [0xb74350] -> thiscall,
/// ret 0x10; o idioma vai num registrador com lixo acima, a cópia passa só o byte). Devolve (fator, F).
pub(crate) unsafe fn language_fluency_factor(p1: usize, clube: usize, s: i16) -> (f64, f64) {
    let q = (s as i32 + 0x1388) as f64;
    let l = q * read_f64(0x968d78) * q;
    let fluencia: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u8 = core::mem::transmute(FN_PLAYERSTATS_GET_LANGUAGE_FLUENCY);
    let f = fluencia(read_usize(G_PLAYER_STATS), p1, clube, read_u8(read_usize(clube + club::NATION) + 0x77) as u32, 1) as i8 as i32 as f64;
    let a = f * read_f64(0x967880);
    let b = l / (read_f64(0x96f938) - f * read_f64(0x96a648));
    (max_f64(a, b) + read_f64(0x968918), f)
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
    // the calls by address, with the conventions checked at the original’s call sites
    // 0x54a110: lea ecx, data -> thiscall, ret 0
    let data_default: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    // 0x8cca10: push 0; push pessoa; mov ecx, [0xde1f54] -> thiscall, ret 8
    let future: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_FUTURE_TRANSFER);
    // 0x4cf5a0: push pessoa; mov ecx, [0xae1050] -> thiscall, ret 4
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    // 0x4cf5f0: push clube; push pessoa; mov ecx, [0xae1050] -> thiscall, ret 8
    let contract_with: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    // 0x54cdb0: push [0xae2c94]; push [0xae2c90]; lea ecx, data -> thiscall, ret 8
    let days_until: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
    // 0x53d980: push 1; push nation; push person; add esp, 0xc -> cdecl
    let estrangeira: unsafe extern "C" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_DATABASE_IS_STAFF_FOREIGN);
    // 0x544590 / 0x544520 / 0x5444a0: push clube; push pessoa; add esp, 8 -> cdecl
    let in_list_emp: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_LOAN_LISTED);
    let na_lista_transf: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_TRANSFER_LISTED_OR_REQUESTED);
    let in_list: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_TRANSFER_LISTED);
    // 0x547d30 / 0x547d60: push clube; push pessoa; add esp, 8 -> cdecl
    let dislikes_club: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_DISLIKES_CLUB);
    let likes_club: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_FAVOURS_CLUB);
    // 0x547810 / 0x547840: push person; push manager; add esp, 8 -> cdecl
    let likes: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_FAVOURED_BY);
    let dislikes: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_DISLIKED_BY);
    // 0x4b02e0: push atual; push clube; add esp, 8 -> cdecl
    let rival: unsafe extern "C" fn(usize, usize) -> u8 = core::mem::transmute(FN_COMP_UTIL_GET_LEAGUE_CLASH_TYPE);
    // 0x4ca470: push contrato; push pessoa; mov ecx, [0xae1050] -> thiscall, ret 8
    let category: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(FN_CONTRACT_MANAGER_GET_DESERVED_SQUAD_STATUS);
    // 0x600a50: push 0; push 0; push 1 (ou 1); push 0 (ou 1); push clube; add esp, 0x14 -> cdecl
    let human: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    // 0x5407e0: push 1; push person; push manager; add esp, 0xc -> cdecl (only the word counts)
    let vista: unsafe extern "C" fn(usize, usize, u32) -> i16 = core::mem::transmute(FN_DATABASE_GET_STAFF_REPUTATION_FOR_VIEWER);
    // 0x5a0590: push club; mov ecx, [0xae2a38] -> thiscall, ret 4; 0x5986b0: mov ecx, finances -> thiscall, ret 0
    let finances: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FINANCETABLE_GET_CLUB_FINANCE);
    let band: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(FN_FINANCE_GET_WEALTH_BAND);
    // 0x4cbb60: push clube; push pessoa; mov ecx, [0xae1050] -> thiscall, ret 8
    let place: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(FN_CONTRACT_MANAGER_GET_SQUAD_PLACEMENT);
    // 0x669bd0: push &data b; push &data a; push clube; add esp, 0xc -> cdecl
    let in_season: unsafe extern "C" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_KEY_NATION_CLUB_IN_SEASON);

    let mut data_a = [0u8; 8];
    let mut data_b = [0u8; 8];
    data_default(data_a.as_mut_ptr() as usize);
    data_default(data_b.as_mut_ptr() as usize);
    let p4b = p4 as u8 != 0;
    let p5b = p5 as u8 as i8;
    let mut l80 = 1.0f64; // [base+0x84]
    let mut l8c = 0usize; // [base+0x78]: the current club’s competition record
    let mut l84 = 0usize; // [base+0x80]: the new club’s competition record
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
    if future(read_usize(G_TRANSFER_MANAGER), p1, 0) != 0 {
        return zero;
    }
    if read_u8(read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(p1).wrapping_mul(0x4f)) + 0xb) & 0x30 != 0 {
        return zero;
    }
    let a57 = read_i8(p1 + 0x57) as i32 as f64;
    let age = read_i8(p1 + 0x18);
    let aa8: f32 = ((k(0x968dc0) - a57 * k(0x967878)) - read_i8(p1 + 0x5b) as i32 as f64 * k(0x967878)) as f32;
    let mut a88: f32 = (a57 * k(0x968a98) + k(0x9689b0)) as f32;
    let mut b4: f32 = (a57 * k(0x967888) + k(0x968968)) as f32;
    let c0 = contract_of(read_usize(G_CONTRACT_MANAGER), p1);
    core::ptr::write_unaligned(G_CONTRACT_LOOKUP_RESULT as *mut usize, c0);
    let mut d4 = 0usize;
    if c0 != 0 {
        d4 = read_usize(G_CLUB_TABLE).wrapping_add(read_usize(c0 + 4).wrapping_mul(club::SIZE));
        if d4 != 0 {
            if read_usize(d4 + 0x53) == 0 {
                return k(0x968e38);
            }
            if read_u8(contract_of(read_usize(G_CONTRACT_MANAGER), p1) + 0x35) & 0x3f == 5 {
                d4 = 0;
            }
        }
    }
    let c = contract_with(read_usize(G_CONTRACT_MANAGER), p1, d4);
    let a0: u32 = if c != 0 { read_usize(c + 0x45) as u32 } else { 0 };
    let r61 = read_usize(p1 + 0x61);
    // the club itself with a contract: stays (3,5) when the contract does not end soon and his reputation does not exceed the club’s
    if p2 != 0 && p2 == d4 && c != 0 {
        let days = days_until(c + 0x2d, read_usize(G_CURRENT_DATE) as u32, read_usize(G_CURRENT_DATE_LEAP) as u32);
        let r1 = read_i16(r61 + 0xb) as i32;
        let mut s = match_pl_ftol((read_i16(p2 + 0x80) as i32 - r1) as f64 * k(0x968968)) as i16;
        if a0 & 0x10 != 0 {
            s = match_pl_ftol(s as f64 * k(0x968968)) as i16;
        }
        let lim = read_i8(p1 + 0x59) as i32 * 10 + 0x465;
        let s: i32 = if s < 0 { 0 } else if s as i32 > lim { lim as i16 as i32 } else { s as i32 };
        if a0 & 0x161a8 == 0 {
            let id = read_usize(p1) as i32;
            if days < id % 0x23 + 0x14
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
        if age < 0x10 && estrangeira(p1, read_usize(p2 + 0x53), 1) != 0 {
            return zero;
        }
        ramo("cqe_4 base de outro clube");
        f8 = 2.75; // cqe4: a base para outro clube
        if c != 0 && read_u8(c + 0x49) != 0 {
            let dd = days_until_date(c + 0x2f, c + 0x2d);
            let v = k(0x9689c0) - dd as f64 * k(0x969048);
            if !p6.is_null() && v < read_i8(p1 + 0x5a) as i32 as f64 * k(0x968a98) + k(0x968e48) {
                *p6 |= 1;
            }
            f8 = v * k(0x9690a8);
        } else if d4 == 0 {
            let dd =((read_u16(G_CURRENT_DATE_YEAR) as i32 - read_u16(p1 + 0x40) as i32) * 365 - read_i16(p1 + 0x3e) as i32) + read_i16(G_CURRENT_DATE) as i32;
            let mut x = dd.wrapping_mul(dd) as f64 * k(0x96f9b8);
            if k(0x96d6a0) < x {
                x = k(0x96d6a0);
            }
            f8 = x + k(0x9690a8);
        }
    }
    // the reputation of both clubs in a loan request
    if p4b && r61 != 0 && read_i16(r61 + 0xd) < 0x128e && p2 != 0 && d4 != 0 && p2 != d4 && a0 & 0x10 != 0 {
        let ax = read_i16(p2 + 0x80);
        let same = read_usize(p2 + 0x53) == read_usize(d4 + 0x53);
        let rb = read_i16(r61 + 0xb);
        if ax > read_i16(d4 + 0x80) && (same || ax > 0x1676) {
            if rb <= 0x1c52 && in_list_emp(p1, 0) != 0 {
                f8 *= k(0x968940);
            } else {
                f8 *= k(0x968928);
            }
        } else if ax > 0xabe && (same || ax > 0x1676) && ax > rb {
            if rb <= 0x1c52 && in_list_emp(p1, 0) != 0 {
                f8 *= k(0x969588);
            } else {
                f8 *= k(0x968980);
            }
        } else if ax > 0x109a && same {
            if rb > 0x186a && ax < 0x1482 {
                f8 *= k(0x968fa0);
            } else if rb <= 0x1c52 && in_list_emp(p1, 0) != 0 {
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
    if age > 0x22 && d4 != p2 && read_usize(p2 + 0x53) != read_usize(p1 + 0x1a) {
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
    // the happiness in the contract (+0x44), the squad status (ea) and the contract demand (c1 = +0x3d)
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
            if read_usize(p2 + 0xcf) != 0 && is_150_days_since(read_usize(p2 + 0xcf)) {
                ea = category(read_usize(G_CONTRACT_MANAGER), p1, c) as i8;
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
            if d4 != 0 && read_usize(d4 + 0xcf) != 0 && is_150_days_since(read_usize(d4 + 0xcf)) {
                ea = category(read_usize(G_CONTRACT_MANAGER), p1, c) as i8;
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
    let jovem = age < 0x20 && read_i8(p1 + 0x22) != 0;
    let mut signed_byte_at: i16; // ebx: the reputation of the current club (or the person’s, without a club) on the comparison scale
    let mut e8: i32 = 0; // [base+0x1c]: the new club’s
    let mut h68: i16 = 0;
    let mut h6c4: i32 = 0;
    let mut ae: i8 = 0;
    let mut den: f32 = 1.0; // [base+0x58]
    if p2 != d4 {
        if human(p2, 0, 1, 0, 0) != 0 {
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
                if age < 0x20 {
                    if age > 0x17 {
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
                if age < 0x20 {
                    if age > 0x17 {
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
        // the new club’s reputation, on several scales, against the person’s
        let mut adv = read_i8(read_usize(p2 + 0x53) + 0x7e);
        if adv < 1 || adv > 3 {
            adv = 3;
        }
        ad = adv.wrapping_mul(adv);
        let h34 = reputation_by_stadium(p2);
        h68 = reputation_by_division(p2);
        let (h6c, reg) = reputation_by_competition(p2, h68);
        l84 = reg;
        let t = read_usize(p2 + 0xcf);
        let mut rv: i16 = 0;
        if t != 0 && read_usize(t + 0x69) != 0 {
            rv = vista(t, p1, 1);
            if read_usize(G_MANAGER_MANAGER) != 0 {
                rv = reputation_by_manager(p2, rv) as i16;
            }
        }
        let mut si: i16 = read_i16(p2 + 0x80);
        let fin = finances(read_usize(G_FINANCE_TABLE), p2);
        if fin != 0 && ad == 1 {
            si = reputation_cap_by_finances(band(fin) as i8, si);
        }
        let ax = read_i16(read_usize(this + shortlist_manager_fields::CLUB_SHORTLISTS).wrapping_add(read_usize(p2).wrapping_mul(9)) + 5);
        let m = if si > ax { si } else { ax };
        let cx = match_pl_ftol(si as f64 * k(0x96a5d0) + m as f64 * k(0x969510)) as i16;
        let den0 = ((b4 as f64 + a88 as f64) + aa8 as f64) + read_f32(0x9689a8) as f64;
        den = if den0 > zero { den0 as f32 } else { 0.5 };
        let s = (((h6c as i16 as f64 * b4 as f64 + h68 as f64 * a88 as f64) + h34 as i16 as f64 * aa8 as f64) + cx as f64) + rv as f64;
        let mut w = match_pl_ftol(s / den as f64) as i16;
        let cl = read_u8(p2 + 0x64);
        if cl != 1 {
            if !(cf10 && read_u8(p1 + 0x22) == 0) {
                w = half_or_minus_2000(w);
            }
            if cl == 3 {
                w = half_or_minus_2000(w);
            }
        }
        e8 = match_pl_ftol(w as f64 * k(0x968940) - k(0x9688e8));
        if read_usize(p2 + 0x53) != 0 {
            let (m, f) = language_fluency_factor(p1, p2, e8 as i16);
            if age < 0x12 && f < k(0x9688a0) && read_usize(read_usize(p1 + 0x1a) + 0x71) != read_usize(read_usize(p2 + 0x53) + 0x71) {
                return zero;
            }
            f8 = m * f8;
            if !p6.is_null() && m < k(0x968e48) {
                *p6 |= 0x10000;
            }
        }
        // the nations, the distance and what the person wants (requested status)
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
                if in_list_emp(p1, 0) != 0 && age < 0x18 && r61 != 0 && read_i16(r61 + 0xb) < 0x109a && read_i8(p1 + 0x5b) > 0xf
                    && read_i8(p1 + 0x5a) > 0xa && c != 0 && !matches!(ea, 1 | 2 | 3 | 4)
                {
                    g *= k(0x968e18);
                }
                if d4 != 0 && read_usize(p2 + 0x53) != 0 && read_usize(read_usize(p2 + 0x53)) == read_usize(KEY_NATION_ITALY) && age < 0x18
                    && read_usize(d4 + 0x57) != 0 && read_usize(read_usize(d4 + 0x57)) == read_usize(KEY_CLUB_COMP_ITALIAN_SERIE_A)
                {
                    g *= k(0x968930);
                }
                if cid2 != 0 && cid4 != 0 {
                    let dd = shortlist_manager_city_distance(cid2, cid4);
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
                    let dd = shortlist_manager_city_distance(cid2, cid4);
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
            let same_manager = t != 0 && read_usize(t + 0x1a) != 0 && estrangeira(p1, read_usize(t + 0x1a), 1) == 0;
            if read_usize(p2 + 0x53) != 0 && read_usize(read_usize(p2 + 0x53)) == read_usize(KEY_NATION_GERMANY) {
                f8 *= k(0x969040);
            }
            if p4b {
                f8 *= if same_manager { k(0x96a6a8) } else { k(0x968920) };
                fl(0x40000);
                break 'n Segue::Fatores;
            }
            let c56 = read_i8(p1 + 0x56) as i32;
            let c57 = read_i8(p1 + 0x57) as i32;
            if same_manager {
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
            if in_list_emp(p1, 0) != 0 {
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
                (0x21 - age as i32) as f64
            } else {
                let a = (0x1c - age as i32) as f64;
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
        } else if na_lista_transf(p1, 0) == 0 && in_list_emp(p1, 0) == 0 && (c == 0 || read_u8(c + 0x49) == 0) {
            let dd = ((read_u16(G_CURRENT_DATE_YEAR) as i32 - read_u16(p1 + 0x40) as i32) * 365 - read_i16(p1 + 0x3e) as i32) + read_i16(G_CURRENT_DATE) as i32;
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
        // the club itself: the squad status by the demand, with the manager there for more than 150 days
        if human(p2, 0, 1, 0, 0) != 0 && is_150_days_since(read_usize(p2 + 0xcf)) {
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
                if age < 0x20 {
                    if age > 0x17 {
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
    // the reputation of the current club (or of the person, without a club) on the same scale
    if d4 == 0 {
        let mut aev = read_i8(read_usize(p1 + 0x1a) + 0x7e);
        if aev < 1 || aev > 3 {
            aev = 3;
        }
        ae = aev.wrapping_mul(aev);
        signed_byte_at = match_pl_ftol(average_reputation3(read_usize(p1 + 0x61)) as i16 as f64 * k(0x968940) - k(0x9688e8)) as i16;
    } else {
        let n4 = read_usize(d4 + 0x53);
        let mut aev = if n4 != 0 { read_i8(n4 + 0x7e) } else { 3 };
        if aev < 1 || aev > 3 {
            aev = 3;
        }
        ae = aev.wrapping_mul(aev);
        let h44 = reputation_by_stadium(d4);
        h6c4 = reputation_by_division(d4) as i32;
        let (bx, reg) = reputation_by_competition(d4, h6c4 as i16);
        l8c = reg;
        let t4 = read_usize(d4 + 0xcf);
        let si: i16 = if t4 == 0 || read_usize(t4 + 0x69) == 0 {
            match_pl_ftol(h6c4 as i16 as f64 * k(0x969040)) as i16
        } else if read_usize(G_MANAGER_MANAGER) == 0 {
            read_i16(read_usize(t4 + 0x69) + 0xa)
        } else {
            reputation_by_manager(d4, read_i16(read_usize(t4 + 0x69) + 0xa)) as i16
        };
        let mut di: i16 = read_i16(d4 + 0x80);
        let fin = finances(read_usize(G_FINANCE_TABLE), d4);
        if fin != 0 && ae == 1 && c != 0 && read_usize(c + 0x45) == 0 {
            let m = band(fin);
            if m == 0xfe {
                fl(0x40000000);
            } else {
                di = reputation_cap_by_finances(m as i8, di);
            }
        }
        let ax = read_i16(read_usize(this + shortlist_manager_fields::CLUB_SHORTLISTS).wrapping_add(read_usize(d4).wrapping_mul(9)) + 5);
        let m = if di > ax { di } else { ax };
        let axv = match_pl_ftol(di as f64 * k(0x96a5d0) + m as f64 * k(0x969510)) as i16;
        // simplification: at the club itself (p2 == d4) the original divides by the float [base+0x58] that nobody wrote (leftover from
        // stack); the result is not used afterwards (the path goes straight to the end), and the copy divides by 1.
        let s = (((bx as i16 as f64 * b4 as f64 + h6c4 as i16 as f64 * a88 as f64) + h44 as i16 as f64 * aa8 as f64) + axv as f64) + si as f64;
        let mut di = match_pl_ftol(s / den as f64) as i16;
        if !p4b && in_list(p1, 0) != 0 {
            di = match_pl_ftol(di as f64 * k(0x9689d8)) as i16;
        }
        let cl = read_u8(d4 + 0x64);
        if cl != 1 {
            if !(c != 0 && read_u8(c + 0x45) & 0x10 != 0 && read_u8(p1 + 0x22) == 0) {
                di = half_or_minus_2000(di);
            }
            if cl == 3 {
                di = half_or_minus_2000(di);
            }
        }
        if place(read_usize(G_CONTRACT_MANAGER), p1, d4) == 3 {
            di = match_pl_ftol(di as f64 * k(0x968e40)) as i16;
        }
        signed_byte_at = match_pl_ftol(di as f64 * k(0x968940) - k(0x9688e8)) as i16;
        if n4 != 0 && p2 != d4 {
            let (m, _) = language_fluency_factor(p1, d4, signed_byte_at);
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
        let mut a8 = (read_i16(p2 + 0x80) as i32 - average_reputation3(r61)) as f64 * k(0x96a6b8) + k(0x967890);
        let e8s = e8 as i16 as i32;
        let sbs = signed_byte_at as i32;
        let cf10 = c != 0 && read_u8(c + 0x45) & 0x10 != 0;
        let mut longo = false;
        if p4b && (cf10 || in_list_emp(p1, 0) != 0) {
            let f = read_usize(p1 + 0x61);
            longo = (read_i16(f + 5) as i32 * 50) < read_i16(d4 + 0x80) as i32
                && read_i8(p1 + 0x22) == 0
                && !(age >= 0x18 && age <= 0x20)
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
        ramo("cqe_4 fator da injury_word_0c");
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
            if read_i16(r61 + 0xb) > signed_byte_at && sbs > e8s {
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
    // young player whose contract ends before the end of the current club’s season: does not leave
    if d4 != 0 && d4 != p2 {
        let s7 = read_i16(read_usize(p1 + 0x61) + 7);
        if s7 > 0x82 && s7 as i32 > read_i16(d4 + 0x80) as i32 / 60 && age < 0x1a
            && in_season(d4, data_a.as_mut_ptr() as usize, data_b.as_mut_ptr() as usize) != 0 && c1 > 0 && c != 0
        {
            let year = u16::from_le_bytes([data_b[2], data_b[3]]);
            let day = i16::from_le_bytes([data_b[0], data_b[1]]);
            let year_c = read_u16(c + 0x2f);
            let before = year < year_c || (year == year_c && day < read_i16(c + 0x2d));
            if before && read_usize(c + 0x45) == 0 && read_u8(c + 0x49) == 0
                && !(human(p2, 1, 1, 0, 0) != 0 && read_i8(p1 + 0x5a) <= 0xf)
                && days_until(data_b.as_mut_ptr() as usize, read_usize(G_CURRENT_DATE) as u32, read_usize(G_CURRENT_DATE_LEAP) as u32) < 0x96
            {
                fl(0x80000000);
                return zero;
            }
        }
    }
    if p3 == 0 || p4b || f8 < k(0x968f98) {
        return f8;
    }
    // the salary: the default (in a local STAFF_CONTRACT) against the club’s ceiling
    ramo("cqe_4 salario");
    let mut contract_local = [0u32; 0x14];
    // 0x85e410: lea ecx, [base+0xa0] -> thiscall, ret 0
    let construct: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_STAFF_CONTRACT_STAFF_CONTRACT);
    construct(contract_local.as_mut_ptr() as usize);
    let fin = finances(read_usize(G_FINANCE_TABLE), p2);
    if fin == 0 {
        return zero;
    }
    // 0x864550: push 0; push 0xb; push status; push 0; push club; push person; lea ecx, contract -> thiscall, ret 0x18
    let default: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32) -> f64 = core::mem::transmute(FN_STAFF_CONTRACT_DEFAULT_STAFF_CONTRACT);
    let w0 = default(contract_local.as_mut_ptr() as usize, p1, p2, 0, p5, 0xb, 0);
    // 0x5968d0: push situation; push person; mov ecx, finances -> thiscall, ret 8
    let ceiling: unsafe extern "thiscall" fn(usize, usize, u32) -> i32 = core::mem::transmute(FN_FINANCE_GET_MAX_WAGE);
    let a8w = ceiling(fin, p1, p5) as f64;
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






/// Branch counter for a hot routine: calls ramo only every 4096 passes.
///
/// [pt] Contador de ramo para a rotina quente (~27 milhões de chamadas até 2002-06-01): o `ramo` só a cada 4096 passagens
/// (a primeira inclusive), para não pesar na carreira.
pub(crate) fn count_hot_branch(n: &core::sync::atomic::AtomicU32, name: &'static str) {
    // simplification: hot routine; the branch counter counts in steps of 4096
    if n.fetch_add(1, core::sync::atomic::Ordering::Relaxed) & 0xfff == 0 {
        crate::comparacao::ramo(name);
    }
}

pub(crate) static BRANCH_BOSMAN: core::sync::atomic::AtomicU32 = core::sync::atomic::AtomicU32::new(0);
pub(crate) static HOT_BRANCH_BAND: core::sync::atomic::AtomicU32 = core::sync::atomic::AtomicU32::new(0);
pub(crate) static FAR_BAND_HITS: core::sync::atomic::AtomicU32 = core::sync::atomic::AtomicU32::new(0);

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
pub(crate) unsafe extern "thiscall" fn shortlist_manager_is_compensation_deal(this: usize, slot: usize, contrato: usize, clube: usize, taxa: *mut i32) -> u32 {
    let _ = this;
    *taxa = 0;
    if contrato == 0 || slot == 0 || *((contrato + contract::LEAVING_ON_BOSMAN) as *const u8) != 0 {
        return 1;
    }
    let person = |c: usize| read_usize(G_STAFF_TABLE).wrapping_add(read_usize(c).wrapping_mul(staff::SIZE));
    let protegido: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32 = core::mem::transmute(FN_STAFF_CONTRACT_CHECK_FIFA_PROTECTED_PERIOD);
    let compensacao: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> i32 = core::mem::transmute(FN_TRANSFER_MANAGER_CALC_COMPENSATION_FEE);
    let bosman: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_PLAYER_CLUB_USES_BOSMAN);
    let band: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(FN_STAFF_CONTRACT_GET_EXPIRY_BAND);
    // 0x868210: push 0; push 0; push clube; mov ecx, contrato -> thiscall; 0x8d0780 e 0x8d0a70: mov ecx, [0xde1f54]
    // before the pushes (the copy passes ecx like the original; both only read the stack, ret 0xc and ret 4); 0x867d30: mov
    // ecx, contrato -> thiscall
    let manager = read_usize(G_TRANSFER_MANAGER);
    if protegido(contrato, clube, 0, 0) == 0 {
        let t = compensacao(manager, person(contrato), clube, 0);
        *taxa = t;
        return (t < read_i32(person(contrato) + 0x52)) as u32;
    }
    if bosman(manager, person(contrato)) == 0 { // cqd12: sem a lei Bosman
        return 0;
    }
    count_hot_branch(&BRANCH_BOSMAN, "cqd_12 com lei Bosman");
    let t = compensacao(manager, person(contrato), clube, 1);
    *taxa = t;
    if t > read_i32(person(contrato) + 0x52) {
        return 0;
    }
    let k = band(contrato) as i8 as i32;
    if k as u32 > 0xe {
        return 0;
    }
    let factor = match k {
        0 => return 0,
        1 | 2 => 0.5, // cqd12: a faixa 1-2
        3 | 4 => 0.65,
        5 | 6 => 0.8,
        7 | 8 => 0.9,
        _ => {
            count_hot_branch(&FAR_BAND_HITS, "cqd_12 band longe");
            return 1; // cqd12: as faixas 9-14
        }
    };
    count_hot_branch(&HOT_BRANCH_BAND, "cqd_12 band pelo fator");
    // fild [vaga+0xb]; fild [vaga+3]; fmul fator; fcompp: 1 if +3 × fator exceeds +0xb (double precision)
    (read_i32(slot + 3) as f64 * factor > read_i32(slot + 0xb) as f64) as u32
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


/// Clubs interested in a player, collected by Shortlist_Manager collect_interested_clubs (0x842180, mode 1) into a
/// temporary list.
///
/// [pt] Os clubes interessados no jogador, como a 0x84b580 os pede (0x842180 com modo 1 numa lista local; os itens de 8
/// bytes: id do jogador, id do clube).
pub(crate) unsafe fn interested_clubs(this: usize, player: usize) -> Vec<usize> {
    let nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let join: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) -> u32 = core::mem::transmute(FN_SHORTLIST_MANAGER_COLLECT_INTERESTED_CLUBS);
    let mut l = [0u8; 0x10];
    let list = l.as_mut_ptr() as usize;
    nova(list);
    join(this, player, list, 0, 1, 0);
    let mut v = Vec::new();
    let mut no = read_usize(list + 2);
    while no != 0 && v.len() < 0x10000 {
        v.push(read_usize(read_usize(no) + 4));
        no = read_usize(no + 6);
    }
    soltar(list);
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
pub(crate) unsafe extern "thiscall" fn mark_player_under_offer(this: usize, player: usize, buyer: usize, swap: usize) {
    let new: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let join: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) -> u32 = core::mem::transmute(FN_SHORTLIST_MANAGER_COLLECT_INTERESTED_CLUBS);
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let human: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let offer: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_TRANSFER_MANAGER_GET_OFFER_SLOT);
    let slot_of: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(FN_SHORTLIST_MANAGER_FIND_CLUB_SHORTLIST_ENTRY);
    // 0x68d020/0x68d040/0x68d1b0: lea ecx, lista -> thiscall; 0x842180: 5 pushes; mov ecx, ebx (ret 0x14); 0x600a50:
    // 5 pushes; add esp, 0x14 -> cdecl; 0x8c61e0: push id; mov ecx, [0xde1f54] (ret 4); 0x849ce0: 4 pushes; mov ecx,
    // ebx (ret 0x10)
    let mut l = [0u8; 0x10];
    let list = l.as_mut_ptr() as usize;
    new(list);
    join(this, player, list, 0, 1, 0);
    core::ptr::write_unaligned((list + 0xa) as *mut usize, read_usize(list + 2));
    let mut last = player; // the original’s ebp: the player, then the last offer found (holds across clubs)
    let mut item = next(list);
    'clubes: while item != 0 {
        let k = read_usize(item + 4);
        let clube = read_usize(G_CLUB_TABLE).wrapping_add(k.wrapping_mul(club::SIZE));
        if human(clube, 0, 1, 0, 0) == 0 {
            let v = if swap != 0 {
                core::ptr::write_unaligned((swap + 0xa) as *mut usize, read_usize(swap + 2));
                let mut q = next(swap);
                while q != 0 {
                    last = offer(read_usize(G_TRANSFER_MANAGER), read_usize(q));
                    if last != 0 && read_usize(read_usize(last) + 8) == k {
                        item = next(list);
                        continue 'clubes;
                    }
                    q = next(swap);
                }
                let kind = *((read_usize(last) + 0x11) as *const u8);
                slot_of(this, clube, player, (kind != 0xb) as u32, 0)
            } else {
                if clube == buyer {
                    item = next(list);
                    continue 'clubes;
                }
                let v = slot_of(this, clube, player, 0, 0);
                if v != 0 { v } else { slot_of(this, clube, player, 1, 0) }
            };
            if v == 0 {
                break;
            }
            *((v + 0x24) as *mut u8) |= 0x20;
        }
        item = next(list);
    }
    soltar(list);
}






/// The valueless end of 0x840d80: for a loan slot (bit 0 of +0x24) and a loan-listed person, raises the rating to max(r
/// + 5000, r x 1.2); returns 0.
///
/// [pt] O fim sem valor da 0x840d80 (0x841f62): fora da lista de empréstimos da vaga (bit 0 de +0x24), 0; com ela e a pessoa
/// listada para empréstimo (0x544590 com 0), a nota vira max(nota + 5000, nota × 1,2) (_ftol) e 0.
pub(crate) unsafe fn loan_listed_value_end(p: usize, slot: usize, out: *mut i32) -> i32 {
    if read_u8(slot + 0x24) & 1 == 0 {
        return 0;
    }
    let loan: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_LOAN_LISTED);
    if loan(p, 0) != 0 {
        let r = *out;
        let a = r.wrapping_add(5000) as f64;
        let q = r as f64 * read_f64(0x968978);
        *out = match_pl_ftol(if a > q { a } else { q });
    }
    0
}

/// Adjusts a young player's rating by the ability seen by the club's manager, age and club reputation (slot types
/// 1..3).
///
/// [pt] O ajuste pela habilidade vista dos jogadores jovens (as vagas de tipo 1, 2 e 3 em +0x24 >> 1): com menos de 27 anos
/// (+0x18 da pessoa) e nota acima de 20000, a habilidade atual vista pelo técnico do clube (0x6e86d0 com 1 e nota/4)
/// × 220 / nota, limitada a `frac` + 0,6 e puxada para 1 aos 24, 25 e 26 anos; com 1,05 × mínimo abaixo da nota ou a
/// vaga de nota alta (`alta`), acima de 1 ela ainda vai pela reputação do clube na tabela [this+0xdc] (9 bytes por
/// clube, palavra +5) sobre a reputação do clube (+0x80, pelo menos 1000), entre 1 e 1,2. Acima de 1: `frac` × ela, e o
/// valor base e a nota × (ela + 1) / 2.
pub(crate) unsafe fn young_player_adjustment(this: usize, clube: usize, p: usize, out: *mut i32, minimum: i32, alta: bool, age: i8, r: i32, frac: &mut f64, e: &mut u32) {
    if age >= 0x1b || r <= 20000 {
        return;
    }
    let vista: unsafe extern "C" fn(usize, usize, usize, u32, u32) -> i16 = core::mem::transmute(FN_MATCH_MAN_JUDGE_PLAYER_ABILITY);
    let sv = vista(read_usize(clube + club::MANAGER), p, clube, 1, (r / 4) as u32) as i32;
    let o = *out as f64;
    let mut d1 = sv as f64 * read_f64(0x968f48) / o;
    let ceiling = *frac + read_f64(0x968e48);
    if !(d1 < ceiling) {
        d1 = ceiling;
    }
    match age {
        0x18 => d1 = d1 * read_f64(0x968e40) + read_f64(0x968e80),
        0x19 => d1 = (d1 + read_f64(0x967890)) * read_f64(0x968968),
        0x1a => d1 = d1 * read_f64(0x968e80) + read_f64(0x968e40),
        _ => {}
    }
    if minimum as f64 * read_f64(0x9689b8) < o || alta {
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
        *e = match_pl_ftol(*e as f64 * h) as u32;
        *out = match_pl_ftol(o * h);
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
/// Nota negativa ou 10000 abaixo do mínimo, ou vaga da lista de empréstimos: o fim sem valor (`loan_listed_value_end`). A névoa
/// (+0xb do registro de 79 bytes em [0xae2d0c], 4 bits) limita a nota a nota + 5000 por (1 + n²/100) × nota. Pela
/// posição (0x83bd00) o teto do preço e a fração; o valor base (alto com nota ≥ 35000, +0xb da vaga ≥ nota/8 e +1 ≥
/// 0x80); o ajuste pelo tipo da vaga (1, 2, 3: os jovens, `young_player_adjustment`; 3, 4, 5, 6: frações); a idade acima do divisor
/// (28, 32 sem posição, 19 na vaga 5) e o estrangeiro (0x53d980); o preço pela consistência (+0x2d da ficha +0x61) e o
/// teto pelo tipo da vaga, pela reputação, pela ficha (+0xd) e pela idade; acima de 1,5 milhão, a habilidade potencial
/// vista (0x6e86d0 com 2) abaixo de 145 puxa para 1,5 milhão; o limite da divisão do clube (+0x57, +0x69 < 16); a nação
/// do clube (+0x53) [0x9cf434] ou [0x9cf300]; com `teto`, no máximo +7 da vaga (pelo menos 1). Devolve o preço (_ftol).
pub(crate) unsafe extern "thiscall" fn value_player_for_club(this: usize, clube: usize, p: usize, slot: usize, out: *mut i32, minimum: i32, ceiling: u32) -> i32 {
    let human: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let escopo: unsafe extern "C" fn(usize, usize) -> u8 = core::mem::transmute(FN_DATABASE_GET_REPUTATION_SCOPE);
    let conhecer: unsafe extern "C" fn(usize, i8) -> i32 = core::mem::transmute(FN_NON_PLAYER_ATTRIBUTE_RATING);
    let loan: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_LOAN_LISTED);
    let rating: unsafe extern "C" fn(usize, u32, u32, usize, i32, usize) -> i32 = core::mem::transmute(FN_MATCH_MAN_SCORE_PLAYER_FOR_SELECTION);
    let mut obs = read_usize(clube + club::MANAGER);
    let t7 = read_usize(slot + 7) as i32;
    let ceiling_i = if t7 > 1 { t7 } else { 1 };
    let mut div: i8 = 4;
    if obs != 0 && read_usize(obs + 0x69) == 0 {
        let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
        let i = sorteio(read_usize(G_NON_PLAYER_COUNT) as i32);
        core::ptr::write_unaligned((obs + 0x69) as *mut usize, read_usize(G_NON_PLAYER_TABLE).wrapping_add((i as usize).wrapping_mul(0x44)));
    }
    // simplification: the original pushes the position word with the high word of ecx (garbage); the receiver reads only the word
    let pos = read_i16(slot + 1) as u16 as u32;
    let kind = read_u8(slot + 0x24) & 0x1e;
    let flags: u32 = if kind == 4 || kind == 6 {
        let mut f = 0x5882;
        if read_usize(slot + 0xf) == read_usize(p) && read_usize(p + 0x39) == clube && human(clube, 1, 1, 0, 0) != 0 {
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
    } else if kind == 8 {
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
    if read_u8(slot + 0x24) & 1 != 0 {
        div = if loan(p, 0) != 0 { 0x10 } else { 6 };
    }
    // the minimum² × 0.000025 / divisor (_ftol)
    let corte = |m: i32, div: i8| {
        let q = m as f64;
        match_pl_ftol(q * read_f64(0x96d240) * q / div as f64)
    };
    if human(clube, 1, 1, 0, 0) != 0 {
        let manager: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_CLUB_MANAGER);
        let in_list: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_SCOUT_MANAGER_IS_PLAYER_SHORTLISTED);
        let substituto: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_HUMAN_MANAGER_GET_STAND_IN_MANAGER);
        let m = manager(clube);
        if in_list(read_usize(G_SCOUT_MANAGER), p, m) != 0 {
            let s = substituto(clube);
            let r = rating(p, pos, flags, clube, -1, s);
            *out = r;
            let a = r.wrapping_add(15000) as f64;
            let q = r as f64 * read_f64(0x968940);
            *out = match_pl_ftol(if a > q { a } else { q });
        } else {
            let s = substituto(clube);
            let v = corte(minimum, div);
            *out = rating(p, pos, flags, clube, v, s);
        }
    } else {
        if flags & 0x100 != 0 || flags & 0x80 != 0 {
            let level: i8 = if flags & 0x100 != 0 { 6 } else { 5 };
            let mut best: i8 = 0;
            if flags & 0x100 != 0 {
                if obs != 0 && read_usize(obs + 0x69) != 0 {
                    best = conhecer(read_usize(obs + 0x69), 6) as i8;
                    match escopo(p, obs) as i8 {
                        3 => best = best.wrapping_sub(5),
                        4 => best = best.wrapping_sub(10),
                        _ => {}
                    }
                }
            } else if obs != 0 {
                // mark 0x80: the manager comes in with +5 (−5 and +0 by scope 4 and 3)
                best = (conhecer(read_usize(obs + 0x69), 5) as i8).wrapping_add(5);
                match escopo(p, obs) as i8 {
                    3 => best = best.wrapping_sub(5),
                    4 => best = best.wrapping_sub(10),
                    _ => {}
                }
            }
            for k in 0..7 {
                let s = read_usize(clube + club::SCOUTS + 4 * k);
                if s == 0 || read_usize(s + 0x69) == 0 {
                    continue;
                }
                let mut v = conhecer(read_usize(s + 0x69), level) as i8;
                match escopo(p, s) as i8 {
                    3 => {
                        if level == 6 && read_i16(clube + club::REPUTATION) < 0xdac {
                            continue;
                        }
                        v = v.wrapping_sub(5);
                    }
                    4 => {
                        if level == 6 && read_i16(clube + club::REPUTATION) < 0xdac {
                            continue;
                        }
                        v = v.wrapping_sub(10);
                    }
                    _ => {}
                }
                if v > best {
                    best = v;
                    obs = s;
                }
            }
        }
        let v = if minimum > -1 { corte(minimum, div) } else { -1 };
        *out = rating(p, pos, flags, clube, v, obs);
        let transferencia: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_TRANSFER_LISTED_OR_REQUESTED);
        if clube != read_usize(p + 0x39) && transferencia(p, 0) != 0 {
            crate::comparacao::ramo("cqa_7 listado para transferencia");
            *out = match_pl_ftol(*out as f64 * read_f64(0x9689b8)); // cqa7: o listado de outro clube × 1,05
        }
    }
    let r = *out;
    if r < 0 || r < minimum.wrapping_sub(10000) || read_u8(slot + 0x24) & 1 != 0 {
        return loan_listed_value_end(p, slot, out);
    }
    let nevoa = read_u8(read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(p).wrapping_mul(0x4f)) + 0xb) & 0xf;
    if nevoa != 0 {
        let a = r.wrapping_add(5000) as f64;
        let w = (game_pow(nevoa as f64, read_f64(0x968950)) * read_f64(0x967898) + read_f64(0x967890)) * r as f64;
        *out = match_pl_ftol(if a < w { a } else { w });
    }
    div = 0x1c;
    let position: unsafe extern "thiscall" fn(usize, u32) -> u8 = core::mem::transmute(FN_SHORTLIST_POSITION_MASK_TO_INDEX);
    let c = position(this, pos) as i8 as i32;
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
    let r = *out;
    let bv = read_usize(slot + 0xb) as i32;
    let alta = r >= 35000 && bv >= r / 8 && read_i16(slot + 1) >= 0x80;
    let mut e = if alta {
        let v = r as f64;
        match_pl_ftol(((v * read_f64(0x96d238) - read_f64(0x967890)) * v + bv as f64 * read_f64(0x9689e0)) * read_f64(0x96f918)) as u32
    } else {
        crate::comparacao::ramo("cqa_7 valor base");
        match_pl_ftol((bv as f64 * read_f64(0x9689e0) + r as f64 * read_f64(0x969040)) * read_f64(0x96f918)) as u32 // cqa7: o valor base
    };
    if e < 1 {
        e = 1;
    }
    let age = read_u8(p + 0x18) as i8;
    match (read_u8(slot + 0x24) >> 1) & 0xf {
        1 | 2 => young_player_adjustment(this, clube, p, out, minimum, alta, age, r, &mut frac, &mut e),
        3 => {
            let k = if read_i16(slot + 1) < 0x80 { read_f64(0x96a670) } else { read_f64(0x96d718) };
            e = match_pl_ftol(e as f64 * k) as u32;
            young_player_adjustment(this, clube, p, out, minimum, alta, age, r, &mut frac, &mut e);
        }
        4 => e = match_pl_ftol(e as f64 * read_f64(0x968968)) as u32,
        5 => {
            e = match_pl_ftol(e as f64 * read_f64(0x968a80)) as u32;
            div = div.wrapping_sub(9);
        }
        6 => e = match_pl_ftol(e as f64 * read_f64(0x968918)) as u32,
        _ => {}
    }
    let kind = read_u8(slot + 0x24) & 0x1e;
    if age > div {
        let d = age as i32 - div as i32;
        let dd = d as f64;
        let sheet = read_usize(p + 0x61);
        let f = read_f64(0x967890) / (dd * read_f64(0x968e80) + read_f64(0x9689b0)); // cqa7: a idade acima do divisor
        let ts = read_i16(sheet + 0xd);
        let g = if ts > 0x1770 { ts as i32 as f64 * read_f64(0x96a680) - read_f64(0x968940) } else { 0.0 };
        let estrangeira: unsafe extern "C" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_DATABASE_IS_STAFF_FOREIGN);
        let y = if estrangeira(p, read_usize(clube + club::NATION), 1) != 0 {
            f / (dd * read_f64(0x968f20) + read_f64(0x9689d8))
        } else if age as i32 > div as i32 + 2 && (kind == 8 || kind == 6) && read_i16(sheet + 0xb) < read_i16(clube + club::REPUTATION) {
            (read_f64(0x967890) - (d - 2) as f64 * read_f64(0x96d438)) * f
        } else {
            f
        };
        crate::comparacao::ramo("cqa_7 idade");
        let um = read_f64(0x967890);
        if kind == 0xa {
            *out = match_pl_ftol(((um - g) * y + g) * *out as f64);
        }
        e = match_pl_ftol(((um - g) * y + g) * e as f64) as u32;
    }
    let sheet = read_usize(p + 0x61);
    let cons = read_u8(sheet + 0x2d) as i8 as f64;
    let c = match_pl_ftol((read_f64(0x967890) - game_pow(cons, read_f64(0x968950)) * read_f64(0x968f40)) * e as f64);
    let mut kk = (c.wrapping_mul(c) as u32) as f64 * teto_pos * read_f64(0x968eb8);
    let ra = read_i16(clube + club::REPUTATION);
    if kind == 0xc {
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
    } else if kind == 0xa {
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
    let ts = read_i16(sheet + 0xd);
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
    let by_age = match age {
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
    if let Some(a) = by_age {
        let t = frac * read_f64(a);
        if t < kk {
            kk = t;
        }
    }
    let base = read_f64(0x96f8d0);
    let mut x = kk;
    if kk > base {
        let vista: unsafe extern "C" fn(usize, usize, usize, u32, u32) -> i16 = core::mem::transmute(FN_MATCH_MAN_JUDGE_PLAYER_ABILITY);
        let n2 = vista(read_usize(clube + club::MANAGER), p, clube, 2, (*out / 4) as u32) as i32 as f64;
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
        let ts = read_i16(sheet + 0xd) as i32;
        if ts > 8000 {
            lim = lim.wrapping_add((ts - 8000) * 10000);
        }
        let l = lim as f64;
        if !(x <= l) {
            let m2 = if read_f64(0x967890) < frac { read_f64(0x967890) } else { frac };
            x = match_pl_ftol(l * m2) as f64;
        }
    }
    // the club’s nation (+0x53 of the club: the original uses edi, which here is already the club)
    let nacao = read_usize(read_usize(clube + club::NATION));
    if nacao == read_usize(KEY_NATION_PORTUGAL) {
        if x < read_f64(0x968e90) {
            x = x * (x * read_f64(0x968e88) * read_f64(0x968e48) + read_f64(0x968e80));
        } else {
            x = x * read_f64(0x9689d8);
        }
    } else if nacao == read_usize(KEY_NATION_FRANCE) {
        x = match_pl_ftol(x * read_f64(0x9689b0)) as f64;
    }
    if ceiling != 0 {
        let c = ceiling_i as f64;
        if c < x {
            x = c;
        }
        x = match_pl_ftol(x) as f64;
    }
    match_pl_ftol(x)
}









/// CONTRACT_MANAGER get_contract (0x4cf5a0) for a person, without storing it.
///
/// [pt] O contrato principal da pessoa (CONTRACT_MANAGER::get_contract 0x4cf5a0: thiscall([0xae1050], pessoa), ret 4).
/// As três marcas não guardam o contrato em [0xae1080].
pub(crate) unsafe fn contract_of_staff(person: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    f(read_usize(G_CONTRACT_MANAGER), person)
}

/// As datas estáticas da 0x54c5b0, da 0x54c8e0 e da 0x54cdb0 (valor e guarda).
pub(crate) const SHORTLIST_MANAGER_DATE_STATICS: [(usize, usize); 6] = [(G_ADD_DAYS_RESULT_DATE, 8), (0xae2469, 1), (G_SUB_DAYS_RESULT_DATE, 8), (0xae249e, 1), (G_DAYS_SINCE_SCRATCH_DATE, 8), (0xae2418, 1)];


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
pub(crate) unsafe extern "thiscall" fn count_slots_with_bids(this: usize, clube: usize, position: u32) -> u8 {
    // 0x600a50: cdecl(club, 1, 1, 0, 0) (add esp, 0x14 afterwards); 0x54c8e0: thiscall(0xae2c90, output, 0x14); 0x8c5cf0:
    // thiscall([0xde1f54], pessoa, 0, 0)
    let human: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let minus_days: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(FN_DATE_SUB_DAYS);
    let offers: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_OFFERS_FOR_PLAYER);
    let entry = shortlist_club_entry(this, clube);
    let mut n: u8 = 0;
    for i in 0..40usize {
        let v = read_usize(entry).wrapping_add(i * 0x25);
        if (read_usize(v + 0xf) as i32) <= -1 {
            continue;
        }
        if (read_usize(v + 0x17) as i32) <= -1 {
            crate::comparacao::ramo("cpy_1 alvo sem lance");
            continue; // cpy1: alvo sem lance não conta
        }
        if position as u8 != 0xff && read_u8(v) != position as u8 {
            crate::comparacao::ramo("cpy_1 outra posicao");
            continue; // cpy1: lance de outra posição não conta
        }
        if read_u8(entry + 4) == 2 && human(clube, 1, 1, 0, 0) != 0 {
            let mut buf = [0u8; 8];
            let r = minus_days(G_CURRENT_DATE, &mut buf, 0x14);
            let day = core::ptr::read_unaligned(r as *const i16);
            let year = read_u16(r + 2);
            let arrival = read_usize(clube + club::MANAGER).wrapping_add(0x3e);
            let year_c = read_u16(arrival + 2);
            let velho = year > year_c || (year == year_c && day > core::ptr::read_unaligned(arrival as *const i16));
            if velho {
                let target = read_usize(G_STAFF_TABLE).wrapping_add(read_usize(v + 0xf).wrapping_mul(staff::SIZE));
                if offers(read_usize(G_TRANSFER_MANAGER), target, 0, 0) == 0 {
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


/// clears bit 0 (transfer listed) of the contract's +0x4e; 1 with a contract, 0 without
/// Original: `shortlist_manager_remove_from_transfer_list` (0x844280, shortlist_manager.cpp, stdcall).
///
/// [pt] { 0x844280 } (stdcall, ret 4, shortlist_manager.cpp): `shortlist_manager_remove_from_transfer_list(pessoa)`, pelas
/// telas de transferência (0x8f6770, 0x8f8120). Sem contrato (0x4cf5a0): 0. Com ele, o bit 0 de +0x4e (na lista de
/// transferências) sai e devolve 1 no byte baixo (os de cima são os do ponteiro do contrato, como a original).
pub(crate) unsafe extern "stdcall" fn shortlist_manager_remove_from_transfer_list(person: usize) -> u32 {
    let c = contract_of_staff(person);
    if c == 0 {
        return 0;
    }
    *((c + 0x4e) as *mut u8) &= 0xfe;
    (c as u32 & !0xff) | 1
}


/// clears bit 1 (loan listed) of the contract's +0x4e; 1 with a contract, 0 without
/// Original: `shortlist_manager_remove_from_loan_list` (0x8442d0, shortlist_manager.cpp, stdcall).
///
/// [pt] { 0x8442d0 } (stdcall, ret 4, shortlist_manager.cpp): `shortlist_manager_remove_from_loan_list(pessoa)`, pela
/// renovação do contrato (0x4dc3a0) e pelas telas de transferência. Sem contrato: 0; com ele, o bit 1 de +0x4e (na
/// lista de empréstimos) sai e devolve 1 no byte baixo (os de cima, os do ponteiro do contrato).
pub(crate) unsafe extern "stdcall" fn shortlist_manager_remove_from_loan_list(person: usize) -> u32 {
    let c = contract_of_staff(person);
    if c == 0 {
        return 0;
    }
    *((c + 0x4e) as *mut u8) &= 0xfd;
    (c as u32 & !0xff) | 1
}







/// As datas estáticas da 0x54c5b0, da 0x54c8e0 e da 0x54cdb0 (valor e guarda).
pub(crate) const STAFF_STATUS_DATE_STATICS: [(usize, usize); 6] = [(G_ADD_DAYS_RESULT_DATE, 8), (0xae2469, 1), (G_SUB_DAYS_RESULT_DATE, 8), (0xae249e, 1), (G_DAYS_SINCE_SCRATCH_DATE, 8), (0xae2418, 1)];

/// Removes a (person, club) pair from a sorted 8-byte pair array of the Shortlist_Manager via bsearch, shifting the
/// rest; returns whether found.
///
/// [pt] Tira o par (pessoa, clube) de um dos vetores ordenados de pares (8 bytes) do Shortlist_Manager (`base` = +0 ou +4,
/// conta em `base` + 0x10) pela busca binária (0x946464: cdecl(chave, vetor, conta, 8, comparação), add esp, 0x14):
/// os de depois descem uma casa, o último fica zerado e a conta cai. Devolve se achou.
pub(crate) unsafe fn remove_sorted_pair(base: usize, key: &[u32; 2], comparar: usize) -> bool {
    let bsearch: unsafe extern "C" fn(*const u32, usize, u32, u32, usize) -> usize = core::mem::transmute(FN_CRT_BSEARCH);
    let p = bsearch(key.as_ptr(), read_usize(base), read_usize(base + 0x10) as u32, 8, comparar);
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
    let end = read_usize(base).wrapping_add(read_usize(base + 0x10).wrapping_mul(8));
    write_u32(end - 4, 0);
    write_u32(end - 8, 0);
    write_u32(base + 0x10, (read_usize(base + 0x10) as u32).wrapping_sub(1));
    true
}

/// Sets the slot date (+0x1b) to today plus or minus `dias` (the given Date routine).
///
/// [pt] A data de hoje mais (0x54c5b0) ou menos (0x54c8e0) `dias` (thiscall(0xae2c90, saída, dias), ret 8) copiada para a
/// data da vaga em +0x1b (0x417c10: thiscall(destino, saída, origem), ret 8).
pub(crate) unsafe fn set_slot_date(v: usize, routine: usize, days: u32) {
    let deslocar: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(routine);
    let copy: unsafe extern "thiscall" fn(usize, *mut [u8; 8], usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let mut a = [0u8; 8];
    let mut b = [0u8; 8];
    let r = deslocar(G_CURRENT_DATE, &mut a, days);
    copy(v + 0x1b, &mut b, r);
}

/// Blocks a slot for a person until the slot date: stores the person, clears bid and values, writes the slot status
/// into the contract's squad status, entry +8 = 100.
///
/// [pt] O par (pessoa, clube) do evento 2 ou 5: a vaga guarda a pessoa em +0x13 (bloqueada até a data de +0x1b), sem lance
/// (+0x17 = −1) e sem valores (+0xb, +7); o estatuto da vaga (bits 1..4 de +0x24) vai ao nibble alto de +0x4f do
/// contrato da pessoa no clube; a entrada fica com 100 em +8.
pub(crate) unsafe fn block_slot_for_person(v: usize, entry: usize, person: usize, contrato: usize, routine: usize, days: u32) {
    set_slot_date(v, routine, days);
    write_u32(v + 0x13, read_usize(person) as u32);
    write_u32(v + 0x17, 0xffff_ffff);
    write_u32(v + 0xb, 0);
    write_u32(v + 7, 0);
    if contrato != 0 {
        let c = (contrato + contract::SQUAD_STATUS) as *mut u8;
        *c = (*c & 0xf) | (((read_u8(v + 0x24) >> 1) & 0xf) << 4);
    }
    *((entry + 8) as *mut u8) = 0x64;
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
pub(crate) unsafe extern "thiscall" fn update_staff_status(this: usize, person: usize, clube: usize, event: u32, kind: u32) -> u8 {
    // 0x68d020/0x68d040/0x68d1b0: thiscall(lista); 0x842180: thiscall(this, pessoa, lista, 0, 3, 0) (5 push); a
    // recursion: thiscall(this, person, club, 1, 2) (4 pushes); 0x600a50: cdecl(club, 0, 1, 0, 0) (add esp, 0x14);
    // 0x4cf5f0: thiscall([0xae1050], pessoa, clube) (2 push); 0x83f3a0: thiscall(this, vaga, clube, pessoa) (3 push)
    let list_new: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let destroy: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let interested: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) -> u32 = core::mem::transmute(FN_SHORTLIST_MANAGER_COLLECT_INTERESTED_CLUBS);
    let status: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u8 = core::mem::transmute(FN_SHORTLIST_MANAGER_UPDATE_STAFF_STATUS);
    let human: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let contract_of: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let fill: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(FN_SHORTLIST_MANAGER_FILL_SHORTLIST_SLOT);
    let mut list = [0u8; 0x10]; // a LLIST local (0xe bytes)
    let l = list.as_mut_ptr() as usize;
    list_new(l);
    let ev = event as u8;
    let id = read_usize(person);
    if (clube == 0 || clube == read_usize(person + staff::CLUB_JOB)) && ev == 4 {
        interested(this, person, l, 0, 3, 0);
        core::ptr::write_unaligned((l + 0xa) as *mut usize, read_usize(l + 2));
        let mut e = next(l);
        while e != 0 {
            status(this, person, friendly_club_by_id(read_usize(e + 4)), 1, 2);
            e = next(l);
        }
    }
    if clube == 0 {
        destroy(l);
        return 1;
    }
    if ev == 5 || (ev != 4 && clube != read_usize(person + staff::CLUB_JOB)) {
        if read_usize(this) == 0 || read_usize(this + 4) == 0 {
            destroy(l);
            return 0;
        }
        let key = [id as u32, read_usize(clube) as u32];
        let mut faltou = !remove_sorted_pair(this, &key, 0x757ea0);
        faltou |= !remove_sorted_pair(this + 4, &key, 0x844740);
        if human(clube, 0, 1, 0, 0) == 0 && !faltou {
            crate::comparacao::ramo("cpy_2 contagem da lista");
            let r = (read_usize(G_STAFF_STATE_TABLE).wrapping_add(id.wrapping_mul(0x4f)) + 0xb) as *mut u8;
            *r = (*r & 0xf0) | ((*r).wrapping_sub(1) & 0xf); // cpy2: uma lista a menos na conta da pessoa
        }
    }
    let entry = shortlist_club_entry(this, clube);
    let contrato = contract_of(read_usize(G_CONTRACT_MANAGER), person, clube);
    for i in 0..40usize {
        let v = read_usize(entry).wrapping_add(i * 0x25);
        let t = kind as u8;
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
                fill(this, v, clube, person); // cpy2: a vaga é preenchida de novo
            }
            2 => block_slot_for_person(v, entry, person, contrato, 0x54c5b0, 0x78),
            5 => {
                crate::comparacao::ramo("cpy_2 bloqueio de 14 dias");
                block_slot_for_person(v, entry, person, contrato, 0x54c5b0, 0xe); // cpy2: o bloqueio de 14 dias
            }
            3 => {
                write_u32(v + 0xf, 0xffff_ffff);
                write_u32(v + 0x17, 0xffff_ffff);
                set_slot_date(v, 0x54c8e0, 0x78);
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
                    set_slot_date(v, 0x54c8e0, 0x78);
                }
            }
            _ => {}
        }
    }
    destroy(l);
    1
}

/// True for the case that cannot run twice (event 4 recursion, or event 1 with the person targeted by a club slot).
///
/// [pt] O caso que não pode rodar duas vezes: a recursão do evento 4 pelos clubes interessados, ou o evento 1 com a pessoa no
/// alvo de alguma vaga do clube (a 0x83f3a0 preenche a vaga: sorteia, acrescenta pares com realocação e dá notícias).
pub(crate) unsafe fn is_original_only_case(this: usize, person: usize, clube: usize, event: u32, kind: u32) -> bool {
    let ev = event as u8;
    if ev == 4 && (clube == 0 || clube == read_usize(person + staff::CLUB_JOB)) {
        return true;
    }
    if ev != 1 || clube == 0 {
        return false;
    }
    // from another club, event 1 goes through the vectors; either one being null returns 0 before the slots
    if clube != read_usize(person + staff::CLUB_JOB) && (read_usize(this) == 0 || read_usize(this + 4) == 0) {
        return false;
    }
    let entry = shortlist_club_entry(this, clube);
    (0..40usize).any(|i| {
        let v = read_usize(entry).wrapping_add(i * 0x25);
        let ok = match kind as u8 {
            1 => read_u8(v) != 0xb,
            0 => read_u8(v) == 0xb,
            _ => true,
        };
        ok && read_usize(v + 0xf) == read_usize(person)
    })
}


/// sets bit 1 (loan listed) of the person's contract +0x4e
/// Original: `shortlist_manager_add_to_loan_list` (0x8442b0, shortlist_manager.cpp, stdcall).
///
/// [pt] { 0x8442b0 } (stdcall, ret 4, shortlist_manager.cpp): `shortlist_manager_add_to_loan_list(pessoa)`, pelas telas de
/// transferência (0x8f6770, 0x8f8120). Com contrato (0x4cf5a0, sem guardar em [0xae1080]), o bit 1 de +0x4e (na lista
/// de empréstimos) liga. Devolve o eax que a original deixa: o contrato.
pub(crate) unsafe extern "stdcall" fn shortlist_manager_add_to_loan_list(person: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = f(read_usize(G_CONTRACT_MANAGER), person);
    if c != 0 {
        *((c + 0x4e) as *mut u8) |= 2;
    }
    c
}


/// the club's fee limit for the player at the squad status (0x84b230 with his strong positions); −1 without a person or
/// player record
/// Original: `shortlist_manager_max_fee_for_player_status` (0x84b1f0, shortlist_manager.cpp, thiscall).
///
/// [pt] { 0x84b1f0 } (thiscall, ret 0xc, shortlist_manager.cpp): `shortlist_manager_max_fee_for_player_status(clube, pessoa,
/// estatuto)`, pela tela de contrato (0x4dd0f0, ecx = o Shortlist_Manager em +0x856 da gerência de transferências).
/// Sem pessoa ou sem ficha de jogador (+0x61): −1. Senão o teto da vaga (0x84b230) com as posições fortes do jogador
/// (0x5b7d20) e o estatuto.
pub(crate) unsafe extern "thiscall" fn shortlist_manager_max_fee_for_player_status(this: usize, clube: usize, person: usize, status: u32) -> i32 {
    // 0x5b7d20: cdecl(record) (add esp, 4); 0x84b230: call with mov ecx, esi (the this) and 3 push (club, positions,
    // status), ret 0xc: thiscall from here (the engine’s stdcall copy does not read ecx)
    if person == 0 || read_usize(person + staff::PLAYER) == 0 {
        return -1;
    }
    let positions: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_FORMATION_GET_POSITION_MASK);
    let ceiling: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> i32 = core::mem::transmute(FN_SHORTLIST_MANAGER_MAX_FEE_FOR_SQUAD_STATUS);
    let m = positions(read_usize(person + staff::PLAYER));
    ceiling(this, clube, m, status)
}







/// Staff record of index i (0x6e bytes, 32-bit math including negative).
///
/// [pt] A pessoa de índice `i` ([0xae23bc], 0x6e bytes; a conta da original em 32 bits, com índice negativo inclusive).
pub(crate) unsafe fn shortlist_manager_staff_by_index(i: i32) -> usize {
    read_usize(G_STAFF_TABLE).wrapping_add((i as usize).wrapping_mul(staff::SIZE))
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
    // thiscall([0xae2a38], club); 0x598800: thiscall(finances, 0, 0, 0, 1, 0) (5 push); 0x945333: cdecl(vector, 0x28,
    // 0x25, 0x83b690) (add esp, 0x10); 0x8c61e0: thiscall([0xde1f54], lance) (1 push); 0x83f3a0: thiscall(this, vaga,
    // clube, 0) (mov ecx, [this local]; 3 push); 0x4cf5a0: thiscall([0xae1050], pessoa); 0x544590: cdecl(pessoa, 0)
    // (add esp, 8); 0x4cf5f0: thiscall([0xae1050], pessoa, clube) (2 push); 0x846920: thiscall(this, pessoa, clube, 1, 0,
    // status, 0) (6 pushes, returns in st0); 0x90cfc0: cdecl(n) (add esp, 4); 0x85fa00: thiscall(contract) (no push,
    // movsx ax); 0x8c5cf0: thiscall([0xde1f54], person, 0, 0) (3 push); 0x54c8e0: thiscall(0xae2c90, output, days)
    let human: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let national_team: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let finances: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FINANCETABLE_GET_CLUB_FINANCE);
    let budget_of: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_FINANCE_GET_TRANSFER_BUDGET);
    let sort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    let play_of: unsafe extern "thiscall" fn(usize, i32) -> usize = core::mem::transmute(FN_TRANSFER_MANAGER_GET_OFFER_SLOT);
    let fill: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(FN_SHORTLIST_MANAGER_FILL_SHORTLIST_SLOT);
    if human(clube, 0, 1, 0, 0) != 0 || national_team(clube) != 0 {
        return;
    }
    let fin = finances(read_usize(G_FINANCE_TABLE), clube);
    if fin == 0 {
        return;
    }
    let mut budget = budget_of(fin, 0, 0, 0, 1, 0) as i32;
    if budget < 0 {
        budget = 0;
    }
    let entry = read_usize(this + shortlist_manager_fields::CLUB_SHORTLISTS).wrapping_add(read_usize(clube).wrapping_mul(9));
    if read_u8(entry + 4) != 1 {
        return;
    }
    sort(read_usize(entry), 0x28, 0x25, 0x83b690);
    let slot = |i: i32| read_usize(entry).wrapping_add((i as usize).wrapping_mul(0x25));
    // the open slots among the first 8, or up to the fourth open player slot
    let mut limit: i8 = 8;
    let mut abertas: i8 = 0;
    let mut players: u8 = 0;
    let mut sum: i32 = 0;
    let mut i: i8 = 0;
    loop {
        let v = slot(i as i32);
        if read_i32(v + 0xf) == -1 {
            if read_u8(v) == 0xb {
                players = players.wrapping_add(1);
                if players == 4 {
                    limit = i.wrapping_add(1);
                }
            }
            sum = sum.wrapping_add(read_i32(v + 0xb));
            abertas = abertas.wrapping_add(1);
        }
        i = i.wrapping_add(1);
        if i >= limit {
            break;
        }
    }
    let a = abertas as i32;
    let mut repartido: i32 = 0;
    if limit > 0 {
        for k in 0..limit as i32 {
            let v = slot(k);
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
            let x = if dv == 0 { 0 } else { budget.wrapping_div(dv) };
            write_i32(v + 7, x);
            repartido = repartido.wrapping_add(x);
        }
    }
    let mut rest = budget.wrapping_sub(repartido);
    let mut baixos: i8 = 0;
    if limit > 0 {
        for k in 0..limit as i32 {
            let v = slot(k);
            let t = read_u8(v);
            if t != 0xb && read_i32(v + 0x13) > -1 {
                continue;
            }
            let target = read_i32(v + 0xf);
            if target > -1 || t == 7 {
                continue;
            }
            let lance = read_i32(v + 0x17);
            if lance > -1 {
                if target == -1 {
                    write_i32(v + 0x17, -1);
                } else if t == 0xb {
                    let l = read_usize(play_of(read_usize(G_TRANSFER_MANAGER), lance));
                    if l != 0 {
                        rest = rest.wrapping_sub(read_i32(l + 0x1b));
                    }
                } else {
                    rest = rest.wrapping_sub(read_i32(shortlist_manager_staff_by_index(target) + 0x52));
                }
                continue;
            }
            if read_u8(clube + club::PROFESSIONAL_STATUS) != 1 && t != 0xb {
                write_i32(v + 7, 0);
            } else {
                let base = if repartido >= 1000 { repartido } else { 1000 };
                let e = read_i32(v + 7);
                let x = match_pl_ftol((e as f64) / (base as f64) * (rest as f64) * read_f64(0x969540));
                crate::comparacao::ramo("cpy_3 reducao pela sobra");
                write_i32(v + 7, e.wrapping_sub(x)); // cpy3: a redução da vaga pela sobra
            }
            let mark = read_u8(v + 0x24) & 0x1e;
            if mark <= 6 {
                if baixos < 3 {
                    baixos = baixos.wrapping_add(1);
                    let s = if sum >= 1 { sum } else { 1 };
                    let b = read_i32(v + 0xb);
                    let b = if b >= 1 { b } else { 1 };
                    let x = match_pl_ftol((rest as f64) / (s as f64) * (b as f64));
                    write_i32(v + 7, read_i32(v + 7).wrapping_add(x));
                }
            } else {
                let s = if sum >= 1 { sum } else { 1 };
                let b = read_i32(v + 0xb);
                let b = if b >= 1 { b } else { 1 };
                let x = match_pl_ftol((rest as f64) / (s as f64) * (b as f64) * read_f64(0x968968));
                write_i32(v + 7, read_i32(v + 7).wrapping_add(x));
            }
            *((v + 0x24) as *mut u8) &= 0xfe;
            crate::comparacao::ramo("cpy_3 vaga preenchida");
            fill(this, v, clube, 0); // cpy3: a vaga é preenchida
            let target = read_i32(v + 0xf);
            if target > -1 && read_u8(v + 0x24) & 1 == 0 {
                rest = rest.wrapping_add(match_pl_ftol(read_i32(shortlist_manager_staff_by_index(target) + 0x52) as f64 * read_f64(0x96f878)));
            }
            sum = sum.wrapping_sub(read_i32(v + 0xb));
        }
    }
    // the remainder goes to the first player slot of the club itself that the manager would agree to sell
    if limit >= 0x28 {
        return;
    }
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let emprestado: unsafe extern "C" fn(usize, u32) -> u32 = core::mem::transmute(FN_STAFF_IS_LOAN_LISTED);
    let contract_in: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let interesse: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32) -> f64 = core::mem::transmute(FN_SHORTLIST_MANAGER_STAFF_INTEREST_IN_JOINING_CLUB);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let end: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_STAFF_CONTRACT_DAYS_TO_END_DATE);
    let offers: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_OFFERS_FOR_PLAYER);
    let minus_days: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(FN_DATE_SUB_DAYS);
    let mut k = limit;
    while rest > 0 {
        let v = slot(k as i32);
        let target = read_i32(v + 0xf);
        let candidata = 'c: {
            if read_u8(v) != 0xb || target <= -1 || read_i32(v + 0x17) != -1 {
                break 'c false;
            }
            let p = shortlist_manager_staff_by_index(target);
            if read_usize(p + 0x39) != clube {
                break 'c false;
            }
            let c = contract_of(read_usize(G_CONTRACT_MANAGER), p);
            *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
            let of_contract = if c != 0 { read_usize(G_CLUB_TABLE).wrapping_add(read_usize(c + 4).wrapping_mul(club::SIZE)) } else { 0 };
            if of_contract == clube || emprestado(p, 0) != 0 {
                break 'c false;
            }
            let c2 = contract_in(read_usize(G_CONTRACT_MANAGER), p, clube);
            let status = if c2 != 0 { (read_u8(c2 + 0x4f) >> 4) as u32 } else { 4 };
            let f = interesse(this, p, clube, 1, 0, status, 0);
            if !(f >= read_f64(0x968e58)) {
                break 'c false;
            }
            if c2 != 0 {
                let lim = 20 - sorteio(10);
                if (end(c2) as i16 as i32) < lim {
                    break 'c true;
                }
            }
            if offers(read_usize(G_TRANSFER_MANAGER), p, 0, 0) != 0 {
                break 'c true;
            }
            if c2 == 0 {
                break 'c false;
            }
            let mut buf = [0u8; 8];
            let r = minus_days(G_CURRENT_DATE, &mut buf, (sorteio(100) + 0x1e) as u32);
            let day = core::ptr::read_unaligned(r as *const i16);
            let year = core::ptr::read_unaligned((r + 2) as *const u16);
            let year_c = core::ptr::read_unaligned((c2 + 0x27) as *const u16);
            year > year_c || (year == year_c && day > core::ptr::read_unaligned((c2 + 0x25) as *const i16))
        };
        if candidata {
            crate::comparacao::ramo("cpy_3 sobra ao jogador do clube");
            write_i32(v + 0xb, read_i32(v + 3));
            write_i32(v + 7, rest);
            return;
        }
        k = k.wrapping_add(1);
        if k >= 0x28 {
            return;
        }
    }
}

// simplification: 0x83f3a0 (called for each slot still without a target) rolls, appends pairs to the vectors with reallocation and gives
// news: cannot run twice. Original only; the copy takes the branches and the mutations.

/// statuses 5 and 6 (youth) only below age 24; the others, always
/// Original: `shortlist_manager_squad_status_allowed` (0x84b4d0, shortlist_manager.cpp, stdcall).
///
/// [pt] { 0x84b4d0 } (stdcall, ret 8, shortlist_manager.cpp; chamada com ecx = o Shortlist_Manager, que ela não lê):
/// `shortlist_manager_squad_status_allowed(pessoa, estatuto)`, pela tela de contrato (0x4dd0f0) e pela de transferência
/// (0x8f6770). Os estatutos 5 e 6 (jovens) só abaixo de 24 anos (+0x18, com sinal); os outros, 1. Sem ficha de jogador
/// (+0x61): a original (caixa "Error", 0).
pub(crate) unsafe extern "stdcall" fn shortlist_manager_squad_status_allowed(person: usize, status: u32) -> u32 {
    if read_usize(person + staff::PLAYER) == 0 {
        return original_fn::<unsafe extern "stdcall" fn(usize, u32) -> u32>(IDX_SHORTLIST_MANAGER_SQUAD_STATUS_ALLOWED)(person, status); // message box: original only
    }
    match status as u8 {
        5 | 6 => ((read_u8(person + staff::YEAR_OF_BIRTH) as i8) < 0x18) as u32,
        _ => 1,
    }
}






/// The club's 9-byte Shortlist_Manager entry (+0xdc; +0 its 40 slots of 0x25 bytes, +8 state).
///
/// [pt] A entrada de 9 bytes do clube na Shortlist_Manager (+0xdc; +0 as 40 vagas de 0x25 bytes, +8 o estado).
pub(crate) unsafe fn shortlist_club_entry(this: usize, clube: usize) -> usize {
    read_usize(this + 0xdc).wrapping_add(read_usize(clube).wrapping_mul(9))
}

/// First difference between two memory snapshots: (address, original, Rust).
///
/// [pt] A primeira diferença entre duas leituras de memória (endereço, original, Rust), para a mensagem.
pub(crate) fn first_memory_diff(a: &crate::comparacao::Memoria, b: &crate::comparacao::Memoria) -> Option<(usize, u8, u8)> {
    for ((ea, va), (_, vb)) in a.0.iter().zip(b.0.iter()) {
        if let Some(k) = (0..va.len()).find(|&k| va[k] != vb[k]) {
            return Some((ea + k, va[k], vb[k]));
        }
    }
    None
}

/// As máscaras de posição (+1, 16 bits) das vagas 1..24 de jogador, por formação (+7 da entrada); a vaga 0 é sempre
/// o goleiro (1). A linha 0 vale para formação fora de 1..9 (o `default` do switch).
pub(crate) static FORMATION_POSITION_MASKS: [[u16; 24]; 10] = [
    [0x804, 0x84, 0x404, 0x104, 0x810, 0x90, 0x410, 0x110, 0x440, 0x140, 0x4, 0x8, 0x10, 0x10, 0x20, 0x40, 0x4, 0x10, 0x1, 0x40, 0x4, 0x10, 0x10, 0x1], // outside 1..9
    [0x808, 0x88, 0x404, 0x2, 0x104, 0x410, 0x220, 0x110, 0x440, 0x140, 0x8, 0x210, 0x90, 0x810, 0x240, 0x220, 0x2, 0x210, 0x1, 0x40, 0x4, 0x8, 0x10, 0x1], // formation 1
    [0x808, 0x88, 0x404, 0x204, 0x104, 0x410, 0x220, 0x110, 0x440, 0x140, 0x8, 0x210, 0x90, 0x810, 0x240, 0x220, 0x204, 0x210, 0x1, 0x40, 0x4, 0x8, 0x10, 0x1], // formation 2
    [0x804, 0x84, 0x404, 0x104, 0x810, 0x90, 0x410, 0x110, 0x440, 0x140, 0x4, 0x4, 0x10, 0x10, 0x220, 0x240, 0x204, 0x210, 0x1, 0x40, 0x4, 0x8, 0x10, 0x1], // formation 3
    [0x804, 0x84, 0x404, 0x104, 0x210, 0x90, 0x810, 0x140, 0x440, 0x240, 0x4, 0x4, 0x10, 0x10, 0x140, 0x440, 0x4, 0x10, 0x1, 0x40, 0x4, 0x10, 0x40, 0x1], // formation 4
    [0x804, 0x84, 0x404, 0x104, 0x820, 0xa0, 0x410, 0x110, 0x440, 0x140, 0x4, 0x4, 0x10, 0x20, 0x440, 0x140, 0x204, 0x10, 0x1, 0x40, 0x4, 0x20, 0x10, 0x1], // formation 5
    [0x810, 0x90, 0x404, 0x204, 0x104, 0x410, 0x208, 0x110, 0x440, 0x140, 0x8, 0x210, 0x90, 0x810, 0x240, 0x220, 0x204, 0x8, 0x1, 0x40, 0x4, 0x10, 0x10, 0x1], // formation 6
    [0x804, 0x84, 0x404, 0x104, 0x208, 0x220, 0x410, 0x110, 0x140, 0x440, 0x4, 0x4, 0x10, 0x10, 0x220, 0x240, 0x204, 0x10, 0x1, 0x40, 0x4, 0x10, 0x10, 0x1], // formation 7
    [0x804, 0x84, 0x404, 0x104, 0x810, 0x90, 0x410, 0x110, 0x210, 0x240, 0x4, 0x4, 0x10, 0x10, 0x220, 0x240, 0x204, 0x10, 0x1, 0x40, 0x4, 0x10, 0x10, 0x1], // formation 8
    [0x204, 0x404, 0x104, 0x810, 0x90, 0x410, 0x110, 0x210, 0x140, 0x440, 0x4, 0x4, 0x10, 0x10, 0x220, 0x240, 0x204, 0x10, 0x1, 0x40, 0x4, 0x10, 0x10, 0x1], // formation 9
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
pub(crate) unsafe extern "stdcall" fn shortlist_set_up_formation_slots(entry: usize) {
    for k in 0..40usize {
        let s = read_usize(entry) + k * 0x25;
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
    let f = *((entry + 7) as *const i8) as i32;
    let row = if (1..=9).contains(&f) { f as usize } else { 0 };
    crate::comparacao::ramo("cpp_5 vagas da formacao");
    let v = read_usize(entry);
    write_u16(v + 1, 1); // o goleiro
    for k in 1..25usize {
        write_u16(v + 1 + k * 0x25, FORMATION_POSITION_MASKS[row][k - 1]);
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
    let budget: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_SHORTLIST_MANAGER_ALLOCATE_TRANSFER_BUDGET);
    soltar(this, clube, shortlist_club_entry(this, clube));
    avaliar(this, shortlist_club_entry(this, clube), clube, 1, 0);
    crate::comparacao::ramo("cpp_5 refazer lista");
    budget(this, clube); // the transfer budget
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
pub(crate) unsafe extern "thiscall" fn suspend_clubs_shortlisting_player(this: usize, player: usize, clear: u8) {
    let new: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let list_new: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let interested: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(FN_SHORTLIST_MANAGER_COLLECT_INTERESTED_CLUBS);
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let destroy: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let free: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_OPERATOR_DELETE);
    let p = new(0xe);
    let list = if p != 0 { list_new(p) } else { 0 };
    interested(this, player, list, 1, 3, 0);
    let n = *(list as *const i16);
    if n != 0 {
        core::ptr::write_unaligned((list + 0xa) as *mut usize, read_usize(list + 2));
    }
    if n > 0 {
        let mut i = 0i32;
        while i < *(list as *const i16) as i32 {
            let item = next(list);
            if item != 0 {
                let e = read_usize(this + shortlist_manager_fields::CLUB_SHORTLISTS).wrapping_add(read_usize(item + 4).wrapping_mul(9));
                crate::comparacao::ramo("cpp_5 suspender clube");
                *((e + 8) as *mut u8) = 0x64; // o clube suspende o jogador
                if clear != 0 {
                    for k in 0..0x28usize {
                        let s = read_usize(e) + k * 0x25;
                        if read_usize(s + 0xf) == read_usize(player) && core::ptr::read_unaligned((s + 0x17) as *const i32) > -1 {
                            core::ptr::write_unaligned((s + 0x17) as *mut i32, -1);
                        }
                    }
                }
            }
            i += 1;
        }
    }
    if list != 0 {
        destroy(list);
        free(list);
    }
}


/// Comparador: a 0x83cb40 reavalia o elenco (sorteios, ofertas, vagas de muitos clubes): roda só a original.
// simplification: original only; the copy is proven by the branch and by the mutation in the career.





/// CONTRACT_MANAGER get_contract (0x4cf5a0) for a person.
///
/// [pt] O contrato da pessoa (0x4cf5a0 no CONTRACT_MANAGER [0xae1050]).
pub(crate) unsafe fn get_contract(person: usize) -> usize {
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    contract_of(read_usize(G_CONTRACT_MANAGER), person)
}

/// get_contract for a person, stored in G_CONTRACT_LOOKUP_RESULT as the original does.
///
/// [pt] O contrato da pessoa, guardado em [0xae1080] como a original faz depois de cada chamada.
pub(crate) unsafe fn shortlist_manager_get_contract_and_store(person: usize) -> usize {
    let c = get_contract(person);
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
pub(crate) unsafe extern "thiscall" fn add_player_to_club_shortlists(this: usize, player: usize, force: u32) {
    let fichas = read_usize(player + 0x61);
    if fichas == 0 {
        return;
    }
    if force == 0 && core::ptr::read_unaligned((fichas + 0xb) as *const i16) < 0x157c {
        return;
    }
    let record = |j: usize| read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(j).wrapping_mul(0x4f));
    if read_u8(record(player) + 0xb) & 0x30 == 0x10 {
        return;
    }
    if read_u8(player + 0x3d) != 0xb {
        return;
    }
    if read_usize(player + 0x24) != 0 && read_u8(player + 0x28) == 5 {
        return;
    }
    let nacao = read_usize(player + 0x1a);
    if nacao != 0 && read_usize(nacao) == read_usize(KEY_NATION_SAUDI_ARABIA) {
        return;
    }
    let away: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_FUTURE_TRANSFER);
    if away(read_usize(G_TRANSFER_MANAGER), player, 0) != 0 {
        return;
    }
    if read_usize(player + 0x39) != 0 {
        let minus: unsafe extern "thiscall" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(FN_DATE_SUB_DAYS);
        let mut t = [0u32; 2];
        let p = minus(G_CURRENT_DATE, t.as_mut_ptr(), 200);
        let lo = read_usize(p) as u32;
        let (dia, ano) = (lo as u16 as i16, (lo >> 16) as u16);
        let year_j = core::ptr::read_unaligned((player + 0x40) as *const u16);
        let day_j = core::ptr::read_unaligned((player + 0x3e) as *const i16);
        let recente = if ano != year_j { ano < year_j } else { dia < day_j };
        if recente {
            let c = get_contract(player);
            if c != 0 && read_u8(c + 0x4f) & 0xf0 != 0x70 && read_usize(c + 0x45) == 0 {
                return;
            }
        }
    }
    let c = shortlist_manager_get_contract_and_store(player);
    let transfer_manager_contract_club = if c != 0 { friendly_club_by_id(read_usize(c + 4)) } else { 0 };
    let mut count = (read_u8(record(player) + 0xb) & 0xf) as i8;
    let mask: unsafe extern "C" fn(usize) -> u16 = core::mem::transmute(FN_FORMATION_GET_POSITION_MASK);
    // simplification: 0x5b7d20 returns only ax and 0x842e10 uses 16 bits of the mask: the upper 16 go zeroed
    let mut m = mask(read_usize(player + 0x61)) as u32;
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
    let national_team: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let append: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_SHORTLIST_MANAGER_TRY_ADD_PLAYER_TO_CLUB_SHORTLIST);
    let clubs = read_usize(G_CLUB_REPUTATION_ORDER);
    let mut i = 0i32;
    loop {
        let clube = friendly_club_by_id(read_usize(clubs + i as usize * 4));
        if count >= 5 {
            return;
        }
        if national_team(clube) == 0 && transfer_manager_contract_club != clube && read_usize(player + 0x39) != clube && append(this, player, clube, m) != 0 {
            crate::comparacao::ramo("cpp_6 clube acrescentou");
            count = count.wrapping_add(1); // one more club with him in the list
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
pub(crate) unsafe extern "thiscall" fn transfer_list_player(this: usize, player: usize, noticia: u32) {
    let listada: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_TRANSFER_LISTED);
    if listada(player, 0) != 0 {
        return;
    }
    let c = get_contract(player);
    if c == 0 {
        return;
    }
    let a = read_u8(c + 0x4f);
    let b = read_u8(c + 0x4e);
    *((c + 0x4e) as *mut u8) = b | 1;
    crate::comparacao::ramo("cpp_6 por a venda");
    *((c + 0x4f) as *mut u8) = (a & 0xf) | 0x70; // for sale
    let away: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_FUTURE_TRANSFER);
    if away(read_usize(G_TRANSFER_MANAGER), player, 0) != 0 {
        return;
    }
    let gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_FAVOURS_CLUB);
    let desgostar = if gosta(player, read_usize(player + 0x39)) != 0 {
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
        antipatia(player, read_usize(read_usize(player + 0x39) + 0xcf), 1);
    }
    if noticia != 0 {
        let in_lists: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_SHORTLIST_MANAGER_ADD_PLAYER_TO_CLUB_SHORTLISTS);
        let dar_news: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_CONTRACT_MANAGER_POST_TRANSFER_LISTED_NEWS);
        in_lists(this, player, 1);
        dar_news(read_usize(G_CONTRACT_MANAGER), player, read_usize(player + 0x39));
    }
}

/// bsearch for the person's appearance-bonus record (0xd bytes) in the TRANSFER_MANAGER's national-team or club array.
///
/// [pt] O registro de prêmio por jogos (0xd bytes: +0 id da pessoa, +4 valor, +8 jogos, +9 id do clube antigo) da pessoa,
/// pela busca binária (0x946464 com 0x8b4e00) no vetor das seleções (tipo 5: +0x898, conta +0x8a0) ou do clube
/// (+0x88c, conta +0x894) do TRANSFER_MANAGER.
pub(crate) unsafe fn appearance_bonus_record(this: usize, player: usize, kind: u32) -> usize {
    let bsearch: unsafe extern "C" fn(*const u32, usize, usize, u32, usize) -> usize = core::mem::transmute(FN_CRT_BSEARCH);
    let key = [read_usize(player) as u32];
    let (vetor, conta) = if kind as u8 == 5 { (read_usize(this + 0x898), read_usize(this + 0x8a0)) } else { (read_usize(this + 0x88c), read_usize(this + 0x894)) };
    bsearch(key.as_ptr(), vetor, conta, 0xd, 0x8b4e00)
}


/// Comparador: a 0x842e10 mexe nas vagas de muitos clubes (e as listas não cabem numa cópia barata): roda só a
/// original.
// simplification: original only; the copy is proven by the branch and by the mutation in the career.









/// The person's main contract (stored), its club, and the contract with that club (0x4cf5f0).
///
/// [pt] O contrato principal da pessoa (0x4cf5a0, guardado em [0xae1080]), o clube dele (id em +4) e o contrato com esse
/// clube (0x4cf5f0).
pub(crate) unsafe fn contract_with_own_club(p: usize) -> usize {
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let with_club: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let c = contract_of(read_usize(G_CONTRACT_MANAGER), p);
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
    let clube = if c != 0 { friendly_club_by_id(read_usize(c + 4)) } else { 0 };
    with_club(read_usize(G_CONTRACT_MANAGER), p, clube)
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
pub(crate) unsafe extern "thiscall" fn build_position_lists(this: usize, not_used: u32) {
    let livrar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let alloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    // simplification: the original pushes the slot in a dword with 3 bytes of stack garbage above; 0x844300 only reads the byte
    let serve: unsafe extern "stdcall" fn(usize, u32) -> u8 = core::mem::transmute(FN_SHORTLIST_STAFF_FITS_SLOT_POSITION);
    crate::comparacao::ramo("cpo_8 montar listas");
    for k in 0..32usize {
        let v = this + shortlist_manager_fields::POSITION_LISTS_BY_PERSON + k * 4;
        if read_usize(v) != 0 {
            livrar(read_usize(v));
            write_u32(v, 0);
        }
    }
    write_u32(this + shortlist_manager_fields::POSITION_LIST_CAPACITY, 0);
    for vaga in 0..16u32 {
        let base = read_usize(G_STAFF_TABLE);
        let mut n = 0i32;
        let mut i = 0usize;
        while (i as i32) < read_i32(G_STAFF_COUNT) {
            if serve(base.wrapping_add(i * 0x6e), vaga) != 0 {
                n += 1;
            }
            i += 1;
        }
        if n > read_i32(this + shortlist_manager_fields::POSITION_LIST_CAPACITY) {
            write_u32(this + shortlist_manager_fields::POSITION_LIST_CAPACITY, n as u32);
        }
    }
    for k in 0..32usize {
        let v = alloc(read_usize(this + shortlist_manager_fields::POSITION_LIST_CAPACITY).wrapping_mul(12) as u32);
        write_u32(this + shortlist_manager_fields::POSITION_LISTS_BY_PERSON + k * 4, v as u32);
        if v == 0 {
            fatal_error_at(0xa825a4, 0x10c);
        }
    }
    for k in 0..16usize {
        write_u32(this + shortlist_manager_fields::POSITION_LIST_COUNTS + k * 4, 0);
    }
    let base = read_usize(G_STAFF_TABLE);
    let mut i = 0usize;
    while (i as i32) < read_i32(G_STAFF_COUNT).wrapping_sub(0x10) {
        let p = base.wrapping_add(i * 0x6e);
        for vaga in 0..16u32 {
            if serve(p, vaga) == 0 {
                continue;
            }
            let pv = vaga as usize;
            let count = read_i32(this + shortlist_manager_fields::POSITION_LIST_COUNTS + pv * 4);
            if count >= read_i32(this + shortlist_manager_fields::POSITION_LIST_CAPACITY) {
                fatal_error_at(0xa825a4, 0x121);
            }
            let o = (count as usize).wrapping_mul(12);
            let (l1, l2) = (read_usize(this + shortlist_manager_fields::POSITION_LISTS_BY_PERSON + pv * 4).wrapping_add(o), read_usize(this + shortlist_manager_fields::POSITION_LISTS_BY_KEY + pv * 4).wrapping_add(o));
            write_u32(l1, i as u32);
            write_u32(l2, i as u32);
            let mut nacao = read_usize(p + 0x1a);
            if nacao == 0 {
                let sortear: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
                nacao = read_usize(G_NATION_TABLE).wrapping_add((sortear(read_i32(G_NATION_COUNT)) as usize).wrapping_mul(nation::SIZE));
                write_u32(p + 0x1a, nacao as u32);
            }
            write_u32(l1 + 4, read_usize(nacao) as u32);
            write_u32(l2 + 4, read_usize(read_usize(p + 0x1a)) as u32);
            write_u32(l1 + 8, read_usize(p + 0x52) as u32);
            let k = contract_with_own_club(p);
            write_u32(l2 + 8, if k != 0 { read_u8(k + 0x3d) as i8 as i32 as u32 } else { 100 });
            write_u32(this + shortlist_manager_fields::POSITION_LIST_COUNTS + pv * 4, (count + 1) as u32);
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
    let band: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(FN_STAFF_CONTRACT_GET_EXPIRY_BAND);
    let human: unsafe extern "C" fn(usize, u32, u32, usize, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let listado: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_TRANSFER_LISTED);
    let sort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    for vaga in 0..16usize {
        let mut j = 0i32;
        let mut o = 0usize;
        while j < read_i32(this + shortlist_manager_fields::POSITION_LIST_COUNTS + vaga * 4) {
            let p = staff_records_staff_by_id(read_usize(read_usize(this + shortlist_manager_fields::POSITION_LISTS_BY_PERSON + vaga * 4) + o));
            let k = contract_with_own_club(p);
            if vaga == 15 {
                write_u32(read_usize(this + 0x54) + o + 8, read_usize(p + 0x52) as u32);
                j += 1;
                o += 12;
                continue;
            }
            if k != 0 && band(k) == 0 {
                write_u32(read_usize(this + shortlist_manager_fields::POSITION_LISTS_BY_PERSON + vaga * 4) + o + 8, read_usize(p + 0x52) as u32);
                let clube = read_usize(p + 0x39);
                if clube != 0 && human(clube, 0, 1, 0, 0) != 0 && listado(p, 0) != 0 {
                    let e = read_usize(this + shortlist_manager_fields::POSITION_LISTS_BY_PERSON + vaga * 4) + o + 8;
                    write_u32(e, read_i32(e).wrapping_add(-0x16e360) as u32);
                    let terco = read_i32(p + 0x52) / 3;
                    if read_i32(e) < terco {
                        write_u32(e, terco as u32);
                    }
                }
            } else {
                if k != 0 {
                    crate::comparacao::ramo("cpo_8 band do contrato");
                }
                write_u32(read_usize(this + shortlist_manager_fields::POSITION_LISTS_BY_PERSON + vaga * 4) + o + 8, 0);
            }
            // a segunda lista
            let p2 = staff_records_staff_by_id(read_usize(read_usize(this + shortlist_manager_fields::POSITION_LISTS_BY_KEY + vaga * 4) + o));
            let k2 = contract_with_own_club(p2);
            let clube = read_usize(p2 + 0x39);
            let key: i32 = if read_usize(p2 + 0x61) == 0 || clube == 0 || read_usize(clube + club::DIVISION) == 0 {
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
            write_u32(read_usize(this + shortlist_manager_fields::POSITION_LISTS_BY_KEY + vaga * 4) + o + 8, key as u32);
            j += 1;
            o += 12;
        }
        let n = read_usize(this + shortlist_manager_fields::POSITION_LIST_COUNTS + vaga * 4) as u32;
        sort(read_usize(this + shortlist_manager_fields::POSITION_LISTS_BY_PERSON + vaga * 4), n, 12, 0x83b670);
        let n = read_usize(this + shortlist_manager_fields::POSITION_LIST_COUNTS + vaga * 4) as u32;
        sort(read_usize(this + shortlist_manager_fields::POSITION_LISTS_BY_KEY + vaga * 4), n, 12, 0x83b670);
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
pub(crate) unsafe extern "thiscall" fn collect_interested_clubs(this: usize, player: usize, out: usize, ignorar: u32, mode: u32, so_humanos: u32) {
    let append: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_LLIST_ADD_TAIL);
    if out != 0 {
        let clear: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_CLEAR);
        clear(out);
    }
    if mode as u8 != 3 {
        let controla: unsafe extern "C" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_CURRENT_CONTROLS_CLUB);
        let in_list: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_SCOUT_MANAGER_IS_PLAYER_SHORTLISTED);
        for k in 0..16i32 {
            let h = staff_records_staff_by_id(read_i32(G_STAFF_COUNT).wrapping_sub(k).wrapping_sub(1) as usize);
            let clube = read_usize(h + 0x39);
            if read_u8(h + 0x5f) == 0 || clube == 0 {
                continue;
            }
            if controla(clube, 0, 1) != 0 && mode as u8 == 2 {
                continue;
            }
            if in_list(read_usize(G_SCOUT_MANAGER), player, h) != 0 {
                let par = [read_usize(player) as u32, read_usize(clube) as u32];
                append(out, par.as_ptr() as usize, 8);
            }
        }
    }
    if so_humanos != 0 {
        return;
    }
    let bsearch: unsafe extern "C" fn(*const u32, usize, i32, u32, usize) -> usize = core::mem::transmute(FN_CRT_BSEARCH);
    let key = [read_usize(player) as u32, 0xffff_ffff];
    let mut e = bsearch(key.as_ptr(), read_usize(this), read_i32(this + shortlist_manager_fields::PAIRS_BY_PERSON_COUNT), 8, 0x757ea0);
    if e == 0 {
        return;
    }
    let base = read_usize(this);
    let last = base.wrapping_add((read_i32(this + shortlist_manager_fields::PAIRS_BY_PERSON_COUNT) as usize).wrapping_mul(8)).wrapping_sub(8);
    let id = read_usize(player);
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
        let club_id = read_usize(e + 4);
        let entry = read_usize(this + shortlist_manager_fields::CLUB_SHORTLISTS).wrapping_add(club_id.wrapping_mul(9));
        let slots = read_usize(entry);
        let mut k = 0usize;
        while k < 0x28 && read_usize(slots + 0xf + k * 0x25) != read_usize(player) {
            k += 1;
        }
        let value = if k < 0x28 { read_i32(slots + k * 0x25 + 7) } else { read_i32(slots + 7) };
        if !(ignorar == 0 && read_u8(entry + 8) >= 100) && value > read_i32(player + 0x52) {
            let c = read_usize(player + 0x39);
            if !(c != 0 && read_usize(c) == club_id) {
                crate::comparacao::ramo("cpo_8 clube interessado");
                append(out, e, 8);
            }
        }
        if e == last {
            return;
        }
        let next = read_usize(e + 8);
        e += 8;
        if next != read_usize(player) {
            return;
        }
    }
}

/// The 8-byte items (two u32) of an LLIST.
///
/// [pt] Os itens (8 bytes) de uma LLIST (+2 o primeiro nó; nó: +0 o dado, +6 o próximo).
pub(crate) unsafe fn llist_pairs(list: usize) -> Vec<(u32, u32)> {
    let mut v = Vec::new();
    let mut no = read_usize(list + 2);
    while no != 0 {
        let x = read_usize(no);
        v.push((read_usize(x) as u32, read_usize(x + 4) as u32));
        no = read_usize(no + 6);
    }
    v
}

/// Comparador: só a original.
// simplification: the original frees and allocates the 32 arrays (and may roll the nation of whoever has none); does not run twice. Checks
// the result: the two lists of each slot with the same ids, each entry fit for the slot, the count at the limit.








/// Appends a (person id, club id) pair to the array [this + vet], bumps the count and qsorts it.
///
/// [pt] Acrescenta o par (id da pessoa, id do clube) no fim do vetor `[this + vet]` (conta em `this + conta`) e o ordena
/// (0x945333 com `cmp`).
pub(crate) unsafe fn insert_sorted_pair(this: usize, vet: usize, count: usize, person: usize, clube: usize, cmp: usize) {
    let sort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    let n = read_i32(this + count);
    let e = read_usize(this + vet).wrapping_add((n as usize).wrapping_mul(8));
    write_u32(e, read_usize(person) as u32);
    write_u32(e + 4, read_usize(clube) as u32);
    write_u32(this + count, (n + 1) as u32);
    sort(read_usize(this + vet), (n + 1) as u32, 8, cmp);
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
pub(crate) unsafe extern "thiscall" fn add_shortlist_entry(this: usize, person: usize, clube: usize) {
    let alloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let realloc: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_CRT_REALLOC);
    if read_usize(this) == 0 {
        let v = alloc(0x320);
        write_u32(this, v as u32);
        if v == 0 {
            fatal_error_at(0xa825a4, 0x1594);
        }
        write_u32(this + 8, 100);
    }
    if read_usize(this + 4) == 0 {
        let v = alloc(0x320);
        write_u32(this + 4, v as u32);
        if v == 0 {
            fatal_error_at(0xa825a4, 0x159d);
        }
        write_u32(this + shortlist_manager_fields::PAIRS_BY_CLUB_CAPACITY, 100);
    }
    if read_usize(this) != 0 {
        let bsearch: unsafe extern "C" fn(*const u32, usize, i32, u32, usize) -> usize = core::mem::transmute(FN_CRT_BSEARCH);
        let key = [read_usize(person) as u32, if clube != 0 { read_usize(clube) as u32 } else { 0xffff_ffff }];
        if bsearch(key.as_ptr(), read_usize(this), read_i32(this + shortlist_manager_fields::PAIRS_BY_PERSON_COUNT), 8, 0x757ea0) != 0 {
            return;
        }
    }
    crate::comparacao::ramo("cpo_9 nova entrada");
    if read_i32(this + 8) == read_i32(this + shortlist_manager_fields::PAIRS_BY_PERSON_COUNT) {
        let v = realloc(read_usize(this), (read_i32(this + 8) as u32).wrapping_mul(8).wrapping_add(0x320));
        write_u32(this, v as u32);
        if v == 0 {
            fatal_error_at(0xa825a4, 0x15b5);
        }
        write_u32(this + 8, read_i32(this + 8).wrapping_add(100) as u32);
    }
    if read_i32(this + shortlist_manager_fields::PAIRS_BY_CLUB_CAPACITY) == read_i32(this + shortlist_manager_fields::PAIRS_BY_CLUB_COUNT) {
        let v = realloc(read_usize(this + 4), (read_i32(this + shortlist_manager_fields::PAIRS_BY_CLUB_CAPACITY) as u32).wrapping_mul(8).wrapping_add(0x320));
        write_u32(this + 4, v as u32);
        if v == 0 {
            fatal_error_at(0xa825a4, 0x15be);
        }
        write_u32(this + shortlist_manager_fields::PAIRS_BY_CLUB_CAPACITY, read_i32(this + shortlist_manager_fields::PAIRS_BY_CLUB_CAPACITY).wrapping_add(100) as u32);
    }
    insert_sorted_pair(this, 0, 0x10, person, clube, 0x757ea0); // the array by person
    insert_sorted_pair(this, 4, 0x14, person, clube, 0x844740); // the vector by club
    let human: unsafe extern "C" fn(usize, u32, u32, usize, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    if human(clube, 0, 1, 0, 0) == 0 {
        let r = (read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(person).wrapping_mul(0x4f)) + 0xb) as *mut u8;
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
pub(crate) unsafe extern "stdcall" fn shortlist_manager_predict_selling_club_response(seller: usize, player: usize, offer: i64, buyer: usize, request: i64, loan: u32) -> u8 {
    let with_club: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let human: unsafe extern "C" fn(usize, u32, u32, usize, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let k = with_club(read_usize(G_CONTRACT_MANAGER), player, seller);
    if seller == 0 || (k != 0 && read_u8(k + 0x49) != 0) {
        return 0;
    }
    if read_usize(seller + 0xcf) == player {
        return 2;
    }
    if k == 0 || read_u8(k + 0x35) & 0x3f != 2 {
        return if loan as u8 != 0 { 2 } else { 0 };
    }
    if loan as u8 != 0 {
        if human(seller, 0, 1, 0, 0) != 0 {
            return 0;
        }
        let listado: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_LOAN_LISTED);
        let rep = read_i16(buyer + 0x80) as f64;
        if listado(player, 0) != 0 && rep * read_f64_at(0x968928) > read_i16(read_usize(player + 0x61) + 0xb) as f64 { // 0x968928 = 1.25
            return 0;
        }
        let st = read_u8(k + 0x4f) & 0xf0;
        if st == 0x60 || st == 0x70 || read_u8(read_usize(player + 0x61) + 0xf) == 0x14 {
            let v = read_i16(read_usize(player + 0x61) + 0xb) as f64 * read_f64_at(0x969040); // 0x969040 = 0.75
            if rep > v && (read_u8(k + 0x3d) as i8) < -50 && read_usize(seller + 0x1db) != read_usize(KEY_CLUB_COMP_UEFA_EUROPEAN_CHAMPIONS_LEAGUE) {
                return 0;
            }
        }
        return 2;
    }
    let price: i32 = if human(seller, 0, 1, 0, 0) != 0 {
        let request_human: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> i32 = core::mem::transmute(FN_CONTRACT_MANAGER_GET_HUMAN_ASKING_PRICE);
        let mut v = match_pl_ftol(request_human(read_usize(G_CONTRACT_MANAGER), seller, buyer, player) as f64 * read_f64_at(0x9689b0)); // 0x9689b0 = 0.8
        let clausula: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_STAFF_CONTRACT_HAS_ACTIVE_RELEASE_CLAUSE);
        if clausula(k) == 0 {
            let manager: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_CLUB_MANAGER);
            let holiday: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_ON_HOLIDAY);
            if holiday(read_usize(G_HUMAN_MANAGER), manager(seller)) != 0 {
                let opcao: unsafe extern "thiscall" fn(usize, u32, usize) -> u8 = core::mem::transmute(FN_HUMAN_MANAGER_GET_HOLIDAY_OPTION);
                match opcao(read_usize(G_HUMAN_MANAGER), 0, manager(seller)) as i8 {
                    1 => return 2,
                    2 => v = read_i32(player + 0x52),
                    _ => {}
                }
            }
        }
        v
    } else if request >= 0 {
        request as i32
    } else if read_usize(player + 0x61) == 0 {
        read_i32(player + 0x52)
    } else {
        let calcular: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, u32) -> i32 = core::mem::transmute(FN_CONTRACT_MANAGER_CALC_ASKING_PRICE);
        calcular(read_usize(G_CONTRACT_MANAGER), seller, buyer, player, 0, 1, 1)
    };
    if offer >= price as i64 {
        return 0;
    }
    let listado: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_TRANSFER_LISTED);
    if human(seller, 0, 1, 0, 0) != 0 && (price.wrapping_sub(0x16e360) as i64) < offer && listado(player, 0) != 0 {
        return 1;
    }
    if price as f64 * read_f64_at(0x96f9c0) > offer as f64 { // 0x96f9c0 = 0.7142857143
        return 2;
    }
    crate::comparacao::ramo("cpo_9 negocia");
    1
}

/// Year of the default date (0x54a110: day 40 of 1972).
///
/// [pt] O ano da data padrão (0x54a110: dia 40 de 1972).
pub(crate) unsafe fn default_date_year() -> u16 {
    let default: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    let mut data = [0u8; 8];
    let p = default(data.as_mut_ptr() as usize);
    read_i16(p + 2) as u16
}

/// True if the date (day, year) is after contract c's end (+0x2d day, +0x2f year).
///
/// [pt] A data (dia do ano +0, ano +2) passa do fim do contrato `c` (+0x2d dia, +0x2f ano)?
pub(crate) unsafe fn is_after_contract_end(day: i16, year: u16, c: usize) -> bool {
    let end = read_i16(c + 0x2f) as u16;
    year > end || (year == end && day > read_i16(c + 0x2d))
}


/// Comparador: só a original, conferindo o plano (feito antes): o par já estava (nada muda) ou entra nos dois vetores
/// (contas + 1, o par achado depois) e o nibble da pessoa soma 1 fora de clube humano.
// simplification: the original reallocates the vectors (0x945667) and sorts them; does not run twice.





/// Club record of contract c (id at +4, G_CLUB_TABLE), or 0.
///
/// [pt] O clube do contrato `c` (id em +4; o registro de clube tem 0x245 bytes em [0xae23b4]), ou 0 sem contrato.
pub(crate) unsafe fn shortlist_manager_club_of_contract(c: usize) -> usize {
    if c == 0 {
        return 0;
    }
    read_usize(G_CLUB_TABLE).wrapping_add(read_usize(c + 4).wrapping_mul(club::SIZE))
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
pub(crate) unsafe extern "thiscall" fn get_squad_status_for_club(this: usize, person: usize, clube: usize, job: *mut u8) -> u8 {
    if clube == 0 {
        return 7;
    }
    let entry: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(FN_SHORTLIST_MANAGER_FIND_CLUB_SHORTLIST_ENTRY);
    let mut e = entry(this, clube, person, (read_usize(person + staff::PLAYER) == 0) as u32, 0);
    if e == 0 && read_usize(person + staff::NON_PLAYER) != 0 {
        e = entry(this, clube, person, 1, 0);
    }
    if e != 0 {
        if !job.is_null() {
            *job = read_u8(e);
        }
        crate::comparacao::ramo("cpn_6 estatuto pela lista");
        return (read_u8(e + 0x24) >> 1) & 0xf; // bits 1..4 of the entry
    }
    if !job.is_null() {
        *job = read_u8(person + staff::JOB_FOR_CLUB);
    }
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = contract_of(read_usize(G_CONTRACT_MANAGER), person);
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
    let by_contract = if shortlist_manager_club_of_contract(c) == clube {
        true
    } else {
        // simplification: the 3rd argument is pushed as dword 1; the job goes in a byte (0x53b620 only reads the byte)
        let human: unsafe extern "C" fn(usize, u32, u32, usize, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
        let type_11_to_15: unsafe extern "C" fn(i8) -> u32 = core::mem::transmute(FN_DATABASE_IS_PLAYER_JOB);
        if human(clube, 1, 1, 0, 0) != 0 && type_11_to_15(read_u8(person + staff::JOB_FOR_CLUB) as i8) != 0 {
            let estimar: unsafe extern "stdcall" fn(usize, usize) -> u32 = core::mem::transmute(FN_SHORTLIST_MANAGER_ESTIMATE_SQUAD_STATUS);
            return estimar(person, clube) as u8;
        }
        read_usize(person + staff::CLUB_JOB) == clube || {
            let partner: unsafe extern "C" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
            read_usize(person + staff::CLUB_JOB) == partner(clube, 0, 1)
        }
    };
    if !by_contract {
        return 7;
    }
    let with_club: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let k = with_club(read_usize(G_CONTRACT_MANAGER), person, clube);
    if k == 0 {
        return 7;
    }
    crate::comparacao::ramo("cpn_6 estatuto pelo contrato");
    read_u8(k + 0x4f) >> 4 // the high nibble of the contract
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
pub(crate) unsafe extern "stdcall" fn shortlist_manager_estimate_squad_status(person: usize, clube: usize) -> u32 {
    let manager: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_CLUB_MANAGER);
    let injury_word_0c: unsafe extern "C" fn(usize, usize, u32) -> u16 = core::mem::transmute(FN_DATABASE_GET_STAFF_REPUTATION_FOR_VIEWER);
    let mut rep = if injury_word_0c(person, manager(clube), 0) as i16 > 1 { injury_word_0c(person, manager(clube), 0) as i16 } else { 1 };
    let mut rc = core::ptr::read_unaligned((clube + club::REPUTATION) as *const i16);
    if rc <= 1 {
        rc = 1;
    }
    let t = manager(clube);
    let in_list: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_SCOUT_MANAGER_IS_PLAYER_SHORTLISTED);
    if in_list(read_usize(G_SCOUT_MANAGER), person, t) != 0 {
        let a = rep as f64 * read_f64_at(0x968940); // 0x968940 = 1.5
        let b = rc as f64 * read_f64_at(0x969040); // 0x969040 = 0.75
        rep = match_pl_ftol(if a > b { a } else { b }) as i16;
    }
    let mut q = rep as f64 / rc as f64;
    if q < read_f64_at(0x967880) { // 0x967880 = 0.1
        q = read_f64_at(0x967880); // 0x967880 = 0.1
    } else if !(q <= read_f64_at(0x968950)) { // 0x968950 = 2
        q = read_f64_at(0x968950); // 0x968950 = 2
    }
    let f = q as f32 as f64; // the original keeps the ratio in a float and compares the rest by it
    if q > read_f64_at(0x968940) { // 0x968940 = 1.5
        return 1;
    }
    if f > read_f64_at(0x968978) && rc < 0x2134 { // 0x968978 = 1.2
        return 2;
    }
    let age = read_u8(person + staff::YEAR_OF_BIRTH) as i8;
    if !(f > read_f64_at(0x968978)) && rep <= 0x1964 { // 0x968978 = 1.2
        if f > read_f64_at(0x968e48) && age < 0x17 { // 0x968e48 = 0.6
            return 5;
        }
        return if age >= 0x18 { 4 } else { 6 };
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
pub(crate) unsafe extern "thiscall" fn shortlist_manager_release_club_slot_targets(this: usize, clube: usize, entry: usize) {
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let atualizar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_SHORTLIST_MANAGER_UPDATE_STAFF_STATUS);
    for k in 0..40usize {
        let o = k * 0x25;
        let v = read_usize(entry).wrapping_add(o + 0xf);
        if read_usize(v) as i32 > (*(G_STAFF_COUNT as *const i32)).wrapping_sub(1) {
            core::ptr::write_unaligned(v as *mut i32, -1);
        }
        let id = read_usize(read_usize(entry).wrapping_add(o + 0xf)) as i32;
        if id <= -1 {
            continue;
        }
        let p = read_usize(G_STAFF_TABLE).wrapping_add((id as usize).wrapping_mul(staff::SIZE));
        let c = contract_of(read_usize(G_CONTRACT_MANAGER), p);
        *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
        let of_contract = if c != 0 && read_u8(c + 0x35) & 0x3f != 5 { shortlist_manager_club_of_contract(c) } else { 0 };
        if of_contract == clube || read_usize(p + 0x39) == clube {
            continue;
        }
        let s = read_usize(entry).wrapping_add(o);
        if read_usize(s + 0x17) as i32 != -1 {
            continue;
        }
        // setne al on the slot address: the high 24 bits are its own
        let kind = ((s & !0xff) | (read_u8(s) != 0xb) as usize) as u32;
        crate::comparacao::ramo("cpn_6 alvo fora do clube");
        atualizar(this, p, clube, 3, kind);
    }
}



/// Comparador: só a original.
// simplification: 0x843e10 changes the club’s list (and others) on each event and cannot run twice; the copy has the branch
// and the proof is by the career (Rust mode against the original).

/// without rule 0xe in the buyer nation, same nation, second nation or EU: yes; no national side: no; else the permit
/// (0x8c7950)
/// Original: `passes_work_permit_check` (0x84b180, shortlist_manager.cpp, stdcall, ret 0xc).
///
/// [pt] { 0x84b180 } (stdcall, ret 0xc, shortlist_manager.cpp): `passes_work_permit_check(jogador, _, comprador)`: o
/// jogador pode ir para o clube comprador? Sem nação (+0x1a): não. Nação do comprador (+0x53) sem a regra 0xe (+0x76),
/// a mesma nação, a 2ª nação (+0x1e) ou comunitário (0x53d740): sim. Sem seleção (+0x24) nem jogos por ela (+0x22):
/// não. Senão a permissão de trabalho (0x8c7950 sobre [0xde1f54], sem oferta, registro nem sorteio). O 2º argumento
/// não é usado.
pub(crate) unsafe extern "stdcall" fn passes_work_permit_check(player: usize, current: usize, buyer: usize) -> u8 {
    let nacao = read_usize(player + 0x1a);
    if nacao == 0 {
        return 0;
    }
    let target = read_usize(buyer + 0x53);
    if *((target + 0x76) as *const u8) != 0xe || nacao == target || read_usize(player + 0x1e) == target {
        return 1;
    }
    let comunitaria: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_STAFF_EU_NATIONAL);
    if comunitaria(player, target) != 0 {
        return 1; // community
    }
    if read_usize(player + 0x24) == 0 && *((player + 0x22) as *const u8) == 0 {
        return 0;
    }
    let permissao: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, u32) -> u8 = core::mem::transmute(FN_TRANSFER_MANAGER_IS_WORK_PERMIT_GRANTED);
    permissao(read_usize(G_TRANSFER_MANAGER), player, target, 0, 0, 0)
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
pub(crate) unsafe extern "thiscall" fn try_loan_offer_for_player(this: usize, player: usize, clube: usize) -> i32 {
    let shortlist_entry: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(FN_SHORTLIST_MANAGER_FIND_CLUB_SHORTLIST_ENTRY);
    let entry = shortlist_entry(this, clube, player, 0, 0);
    let c = main_contract_via_manager(player);
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
    let owner = if c != 0 { friendly_club_by_id(read_usize(c + 4)) } else { 0 };
    if owner == 0 {
        if entry != 0 {
            if LOAN_OFFER_DRY_RUN {
                return -3;
            }
            let state: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_SHORTLIST_MANAGER_UPDATE_STAFF_STATUS);
            state(this, player, clube, 1, 0);
        }
        return -1;
    }
    // simplification: the original pushes the slot’s situation in a register whose remainder is the owning club’s address (base +
    // id); the copy reproduces the dword.
    let status = (read_usize(G_CLUB_TABLE).wrapping_add(read_usize(c + 4)) as u32 & !0xff) | ((read_u8(entry + 0x24) >> 1) & 0xf) as u32;
    let f1 = staff_interest_in_club(read_usize(read_usize(G_TRANSFER_MANAGER) + 0x856), player, clube, 0, 1, status);
    crate::comparacao::ramo("cpw_3 interesse no emprestimo");
    if !(f1 >= read_f64_at(0x968e38)) { // cpw3: o interesse mínimo; 0x968e38 = 3.5
        return -1;
    }
    let month_of: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_DATE_GET_MONTH);
    let m = month_of(G_CURRENT_DATE) as u8;
    let c = main_contract_via_manager(player);
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
    let a2 = if c != 0 { friendly_club_by_id(read_usize(c + 4)) } else { 0 };
    let period: unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut i8, *mut u16, *mut i8, *mut u16) = core::mem::transmute(FN_TRANSFER_MANAGER_GET_LOAN_PERIOD);
    // simplification: the end month goes to the slot of the club argument (the low byte), the rest stays; the others are locals
    let (mut ini_mes, mut ini_ano, mut mes, mut ano) = (0i8, 0u16, clube as u8 as i8, 0u16);
    period(read_usize(G_TRANSFER_MANAGER), clube, a2, player, &mut ini_mes, &mut ini_ano, &mut mes, &mut ano);
    let today_year = read_i16(G_CURRENT_DATE_YEAR) as u16;
    let m16 = m as i8 as i16;
    let until_end = (ano.wrapping_sub(today_year) as i16).wrapping_mul(12).wrapping_sub(m16).wrapping_add(mes as i16);
    let ate_ini = (ini_ano.wrapping_sub(today_year) as i16).wrapping_mul(12).wrapping_sub(m16).wrapping_add(ini_mes as i16);
    let start = if ate_ini >= 1 { ate_ini } else { 1 };
    if start > until_end {
        return -1;
    }
    let nacao = read_usize(clube + club::NATION);
    if nacao != 0 && read_usize(nacao) == read_usize(KEY_NATION_NORWAY) && (m as i8 >= 0xa || m as i8 <= 2) {
        return -1;
    }
    let f2 = staff_interest_in_club(this, player, clube, 0, 0, 1);
    let mut meses = until_end as u8;
    crate::comparacao::ramo("cpw_3 conta do inicio");
    if f2 >= read_f64_at(0x9690a8) { // cpw3: o interesse para contar do início; 0x9690a8 = 2.75
        let n = read_usize(read_usize(clube + club::NATION));
        if n != read_usize(KEY_NATION_SPAIN) && n != read_usize(KEY_NATION_PORTUGAL) && n != read_usize(KEY_NATION_ITALY) {
            meses = start as u8;
        }
    }
    let mut month_return = meses.wrapping_add(m);
    // simplification: the year goes in the slot of the player argument (the low word; the rest is the player’s address), and the
    // percentage afterwards in its low byte; the copy reproduces the dword.
    let mut slot = (player as u32 & 0xffff_0000) | today_year as u32;
    if month_return as i8 > 0xb {
        let q = (month_return as i8 as i32 as u32) / 12;
        month_return = month_return.wrapping_add((q as u8).wrapping_mul(0xf4));
        slot = slot.wrapping_add(q);
    }
    let day_of_month: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_DATE_GET_DAY_OF_MONTH);
    let day = day_of_month(G_CURRENT_DATE) as i16;
    // the original rereads the day (second call) when it is not above 28
    let day = if day > 0x1c { 0x1c } else { day_of_month(G_CURRENT_DATE) as i16 as i32 };
    let data_dma: unsafe extern "thiscall" fn(*mut [u16; 4], i32, u32, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE_549F70);
    let mut volta = [0u16; 4];
    data_dma(&mut volta, day, month_return as u32, slot, 0xffff_ffff);
    let v = volta.as_ptr() as usize;
    let encerrar: unsafe extern "thiscall" fn(usize, usize, usize, u32, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_CAN_END_LOAN);
    if encerrar(read_usize(G_TRANSFER_MANAGER), clube, player, 0, v, 1) == 0 {
        return -1;
    }
    let c = main_contract_via_manager(player);
    if c == 0 || contract_manager_date_is_before(c + 0x2d, v) {
        return -1;
    }
    let key: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_KEY_NATION_INDEX_OF_CLUB);
    let k = key(clube) as u8 as i8 as i32;
    let window = (0xb63d8a + k * 72) as usize;
    let minus_weeks: unsafe extern "thiscall" fn(usize, *mut [u16; 4], u32) -> usize = core::mem::transmute(FN_DATE_SUB_WEEKS);
    let more_weeks: unsafe extern "thiscall" fn(usize, *mut [u16; 4], u32) -> usize = core::mem::transmute(FN_DATE_ADD_WEEKS);
    let mut t = [0u16; 4];
    let r = minus_weeks(v, &mut t, 4);
    if contract_manager_date_is_before(r, window) {
        return -1;
    }
    crate::comparacao::ramo("cpw_3 tres semanas");
    let r = more_weeks(G_CURRENT_DATE, &mut t, 3); // cpw3: as 3 semanas a partir de hoje
    if contract_manager_date_is_before(v, r) {
        return -1;
    }
    let dif = read_i16(clube + club::REPUTATION) as i32 - read_i16(owner + 0x80) as i32;
    let pct = if dif > 0x9c4 { 100 } else if dif <= -0x5dc { 0 } else { 50 };
    let slot = (slot & !0xff) | pct;
    if LOAN_OFFER_DRY_RUN {
        return -2;
    }
    let offer: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_MAKE_LOAN_OFFER);
    offer(read_usize(G_TRANSFER_MANAGER), player, clube, v, slot, 1, 1) as i32
}


/// clears the bid marker of the club shortlist slots that target the player
/// Original: `Shortlist_Manager::clear_bid_markers_for_player` (0x84aeb0, shortlist_manager.cpp, thiscall).
///
/// [pt] { 0x84aeb0 } (thiscall, ret 8, shortlist_manager.cpp): com a lista do clube ativa (marca 1), tira a
/// marca de lance (+0x17 = −1) das 40 vagas (0x25 bytes cada) cujo alvo (+0xf) é o jogador `*jogador`.
pub(crate) unsafe extern "thiscall" fn clear_bid_markers_for_player(manager: usize, clube: *const i32, player: *const i32) {
    let l = club_shortlist(manager, clube);
    if *((l + 4) as *const u8) != 1 {
        return;
    }
    for i in 0..40 {
        let v = core::ptr::read_unaligned(l as *const usize) + i * 0x25;
        if core::ptr::read_unaligned((v + 0xf) as *const i32) == *player {
            core::ptr::write_unaligned((v + 0x17) as *mut i32, -1);
        }
    }
}


/// adds to the list every shortlist entry (8 bytes) of the club, found by binary search
/// Original: `Shortlist_Manager::collect_club_shortlist_entries` (0x8420e0, shortlist_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x8420e0 } (thiscall, ret 8, shortlist_manager.cpp): `Shortlist_Manager::collect_club_shortlist_entries(clube,
/// lista)`: acrescenta (0x68d0d0, 8 bytes) à `lista` cada entrada da shortlist do clube (`*clube`), na ordem.
pub(crate) unsafe extern "thiscall" fn collect_club_shortlist_entries(this: usize, clube: *const u32, list: usize) {
    let append: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_LLIST_ADD_TAIL);
    for e in club_shortlist_entries(this, *clube) {
        append(list, e, 8);
    }
}


/// the shortlist slot fee ceiling: club reputation times a factor (or -1)
/// Original: `shortlist_manager_max_fee_for_squad_status` (0x84b230, shortlist_manager.cpp, stdcall).
///
/// [pt] { 0x84b230 } (stdcall, ret 0xc, shortlist_manager.cpp; `posicoes` não é lido): o teto de preço
/// para a vaga de tipo `tipo` na lista de observados, pela reputação do clube (palavra +0x80): tipo 4
/// = x550 em Itália, Inglaterra e Espanha ([0x9cf360], [0x9cf2e4], [0x9cf488], nação +0x53), x475 em
/// Alemanha e França ([0x9cf314], [0x9cf300]), x330 nas outras; tipo 5 = x550; tipo 6 = x110; os
/// outros, -1.
pub(crate) unsafe extern "stdcall" fn shortlist_manager_max_fee_for_squad_status(clube: usize, positions: u32, kind: u32) -> i32 {
    let rep = core::ptr::read_unaligned((clube + club::REPUTATION) as *const i16) as i32;
    match kind as i8 {
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


/// is the (player, club or −1) pair in the shortlist pairs? bsearch with 0x757ea0
/// Original: `Shortlist_Manager::is_on_shortlist` (0x842080, shortlist_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x842080 } (thiscall, ret 8, shortlist_manager.cpp): `Shortlist_Manager::is_on_shortlist(jogador, clube)`: sem
/// vetor de pares ([this]) devolve 0; senão procura (bsearch 0x946464, [this+0x10] pares de 8 bytes, comparação
/// 0x757ea0) a chave (id do jogador, id do clube ou −1 sem clube). O al é 1 se achou; os 3 bytes altos do eax são os do
/// ponteiro achado (setne al sobre o retorno do bsearch), e a cópia os reproduz.
pub(crate) unsafe extern "thiscall" fn is_on_shortlist(this: usize, player: usize, clube: usize) -> u32 {
    let v = read_usize(this);
    if v == 0 {
        return 0;
    }
    let key: [u32; 2] = [read_usize(player) as u32, if clube != 0 { read_usize(clube) as u32 } else { u32::MAX }]; // sem clube: −1
    let bsearch: unsafe extern "C" fn(*const [u32; 2], usize, usize, usize, usize) -> usize = core::mem::transmute(FN_CRT_BSEARCH);
    let p = bsearch(&key, v, read_usize(this + shortlist_manager_fields::PAIRS_BY_PERSON_COUNT), 8, 0x757ea0);
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
    let human: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let r = shopping_list_entry(this, clube);
    let target = (r + 5) as *mut i16;
    let u = |p: usize| core::ptr::read_unaligned(p as *const usize);
    let w = || core::ptr::read_unaligned(target) as i32;
    let por = |dif: i32, k: usize, v: i32| match_pl_ftol((dif as f64 * read_f64_at(k) + read_f64_at(0x967890)) * v as f64); // 0x967890 = 1
    core::ptr::write_unaligned(target, (core::ptr::read_unaligned((clube + club::ATTENDANCE) as *const i32) / 10 + 0x1194) as i16);
    if human(clube, 1, 1, 0, 0) == 0 {
        let manager = u(clube + club::MANAGER);
        if manager != 0 {
            let dif = byte(manager, 0x57) as i32 - byte(u(manager + 0x69), 0x1d) as i32;
            core::ptr::write_unaligned(target, por(dif, 0x96a740, w()) as i16);
        }
    }
    let p = u(clube + club::CHAIRMAN);
    if p != 0 && u(p + 0x69) != 0 {
        let dif = byte(p, 0x57) as i32 - byte(u(p + 0x69), 0x1d) as i32;
        let v = por(dif, 0x968958, w()) as i16;
        core::ptr::write_unaligned(target, v);
        let sheet = u(u(clube + club::CHAIRMAN) + 0x69);
        core::ptr::write_unaligned(target, por(byte(sheet, 0x20) as i32 - 10, 0x967878, v as i32) as i16);
    }
    let floor = core::ptr::read_unaligned((clube + club::REPUTATION) as *const i16);
    let v = core::ptr::read_unaligned(target);
    let v = if floor > v { floor } else if v > 0x2710 { 0x2710 } else { v };
    core::ptr::write_unaligned(target, v);
    v as i32
}


/// the player fits shortlist slot 0..15 (rating 15 in sector and side; slot 15 needs a contract, an id and 0x600a50 at
/// zero); returns only `al`
/// Original: `shortlist_slot_fits` (0x844300, shortlist_manager.cpp, stdcall).
///
/// [pt] { 0x844300 } (stdcall, ret 8, shortlist_manager.cpp): o jogador `j` (ficha em [j+0x61]) serve
/// para a vaga `vaga` 0..15 da lista de observação: 0..14 pedem nota 15 nas posições da vaga
/// (setor e lado: +0xf..+0x16 e +0x17..+0x19); 15 pede ficha de contrato ([j+0x69]), id abaixo de
/// [0xae2364]−16 e, com treinador em [j+0x39] de função 11, 13 ou 15 em +0x3d, a 0x600a50 em zero.
/// Sem ficha (fora a vaga 15): 0. Vaga fora de 0..15: a original (caixa de erro). Devolve só `al`.
pub(crate) unsafe extern "stdcall" fn shortlist_slot_fits(j: usize, slot: i8) -> u8 {
    let sheet = core::ptr::read_unaligned((j + 0x61) as *const usize);
    if sheet == 0 && slot != 15 {
        return 0;
    }
    if !(0..=15).contains(&slot) {
        return original_fn::<unsafe extern "stdcall" fn(usize, i8) -> u8>(IDX_SHORTLIST_SLOT_FITS)(j, slot);
    }
    let n = |o: usize| *((sheet + o) as *const i8) >= 15;
    // (setor, lado): 0 = so o setor
    let (setor, lado): (usize, usize) = match slot {
        0 => (0xf, 0),
        1 => (0x10, 0x19),
        2 => (0x11, 0x18),
        3 => (0x11, 0x17),
        4 => (0x11, 0x19),
        5 => (0x16, 0x18),
        6 => (0x12, 0x19),
        7 => (0x16, 0x17),
        8 => (0x13, 0x18),
        9 => (0x13, 0x19),
        10 => (0x13, 0x17),
        11 => (0x14, 0x18),
        12 => (0x14, 0x19),
        13 => (0x14, 0x17),
        14 => (0x15, 0),
        _ => {
            let manager = core::ptr::read_unaligned((j + 0x39) as *const usize);
            if manager != 0 && matches!(*((j + 0x3d) as *const u8), 0xb | 0xf | 0xd) {
                let f: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
                if f(manager, 0, 1, 0, 0) != 0 {
                    return 0;
                }
            }
            let contrato = core::ptr::read_unaligned((j + 0x69) as *const usize);
            return (contrato != 0 && *(j as *const i32) < (*(G_STAFF_COUNT as *const i32)).wrapping_sub(0x10)) as u8;
        }
    };
    (n(setor) && (lado == 0 || n(lado))) as u8
}

/// the position code from the sector and side mask (28 combinations; the rest 0xe); returns only `al`
/// Original: `position_code` (0x83bd00, shortlist_manager.cpp, stdcall).
///
/// [pt] { 0x83bd00 } (stdcall, ret 4, shortlist_manager.cpp): o código da posição (0..13, 0x11..0x15)
/// da máscara `m` de um bit de setor com um de lado (0x80, 0x100, 0x200, 0x400, 0x800); fora das
/// combinações conhecidas, 0xe. Devolve só `al`.
pub(crate) unsafe extern "stdcall" fn position_code(m: i16) -> u8 {
    match m {
        1 => 0,
        2 => 1,
        4 => 0x11,
        8 => 0x13,
        0x10 => 0x12,
        0x20 => 0x15,
        0x40 => 0x14,
        0x84 => 2,
        0x88 => 5,
        0x90 => 8,
        0xa0 => 0xb,
        0x104 | 0x204 | 0x404 => 4,
        0x108 | 0x208 | 0x408 => 6,
        0x110 | 0x210 | 0x410 => 9,
        0x120 | 0x220 | 0x420 => 0xc,
        0x804 => 3,
        0x808 => 7,
        0x810 => 0xa,
        0x820 => 0xd,
        _ => 0xe,
    }
}



