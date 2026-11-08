use super::*;

// The comparers of the routines in ../transfer_manager.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../transfer_manager.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `add_transfer_offer` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x8be300: só a original (aloca a oferta e as listas, tira a oferta antiga, agenda, entrega a
/// notícia). Confere o plano feito antes: a vaga (`find_offer_slot_and_rival`, com o vetor que cresce quando está cheio) e o número
/// de série gravados na oferta de entrada e a série que sobe.
pub(crate) unsafe extern "thiscall" fn comparar_add_transfer_offer(this: usize, offer: usize, terms: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_ADD_TRANSFER_OFFER);
    let (alvo, comprador) = (read_u32_as_usize(offer + offer::TARGET_STAFF_ID), read_u32_as_usize(offer + 8));
    let first = read_u8(read_u32_as_usize(this + 0x86e).wrapping_add(alvo.wrapping_mul(8)) + 1) == 0;
    let (lidas, conta) = (read_u32_as_usize(this + 4) as i32, read_u32_as_usize(this + 8) as i32);
    let cap = if conta == lidas { lidas.wrapping_add(0x32) } else { lidas };
    let (vaga, _) = find_offer_slot_and_rival(read_u32_as_usize(this), lidas, cap, conta, alvo, comprador, read_u32_as_usize(this + 0x85e) as i32, first);
    let serie = read_u32_as_usize(this + 0x85a);
    o(this, offer, terms); // simplification: original only
    if vaga == -1 {
        return;
    }
    let (v, s, s2) = (read_u32_as_usize(offer + offer::SLOT_INDEX) as i32, read_u32_as_usize(offer + offer::SERIAL), read_u32_as_usize(this + 0x85a));
    // the serial goes up at least once: a nested offer made inside the call (news, scheduling) raises it again
    // (8-league audit of 07/11/2026, ENG SCO IRL ITA ESP, 2002-050: 4625 against 4624)
    // [pt] a série sobe pelo menos uma vez: uma oferta aninhada feita dentro da chamada (notícia, agenda) a sobe de novo
    PLACAR.record(v == vaga && s == serie && s2 > serie, || format!(
        "add_transfer_offer(alvo {}, comprador {}): vaga original {} plano {}, série {} plano {}, próxima {} plano {}",
        alvo, comprador, v, vaga, s, serie, s2, serie.wrapping_add(1)));
}

/// Comparer for `accept_loan_offer` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x8c0660: só a original (entrega notícias, agenda a oferta, mexe nas listas de observação).
pub(crate) unsafe extern "thiscall" fn comparar_accept_loan_offer(this: usize, slot: *mut usize, mark: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, *mut usize, u32)>(IDX_ACCEPT_LOAN_OFFER)(this, slot, mark) // simplification: original only
}

/// Comparer for `handle_player_accepted_terms` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x8c2180: só a original (entrega notícias, conclui ou recusa o empréstimo, assina contrato, registra a
/// transferência, mexe nas listas).
pub(crate) unsafe extern "thiscall" fn comparar_handle_player_accepted_terms(this: usize, person: usize, slot: i32, contrato: usize, kind: u32, noticia: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, usize, u32, u32)>(IDX_HANDLE_PLAYER_ACCEPTED_TERMS)(this, person, slot, contrato, kind, noticia) // simplification: original only
}

/// Comparer for `ai_respond_to_bid` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x8c8090: só a original (aceita, recusa ou contrapropõe, reagenda, mexe nas listas de observação e
/// aloca e solta as opções).
pub(crate) unsafe extern "thiscall" fn comparar_ai_respond_to_bid(this: usize, slot: usize, ensaio: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_AI_RESPOND_TO_BID)(this, slot, ensaio) // simplification: original only
}

/// Comparer for `add_loan_negotiation` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x8bee50: só a original (a lista de contratos cresce, a negociação nova é alocada).
pub(crate) unsafe extern "thiscall" fn comparar_add_loan_negotiation(this: usize, o: usize, a2: u32, a3: u32, a4: u32, a5: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) -> u32>(IDX_ADD_LOAN_NEGOTIATION)(this, o, a2, a3, a4, a5) // simplification: original only
}

/// Comparer for `make_exchange_option` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x8c8860: lado a lado a partir do mesmo estado de [0xae1080], *saida, *atracao e do sorteador (a
/// 0x8e78f0 sorteia); a cópia roda sem contar sorteios. Confere a devolução, a atração, [0xae1080] e a foto da oferta
/// nova de cada lado; a da cópia é destruída (0x8e5710) e liberada (0x944c94) e fica tudo como a original deixou.
pub(crate) unsafe extern "thiscall" fn comparar_make_exchange_option(this: usize, buyer: usize, target: usize, offer: usize, out: *mut usize, atracao: *mut i8, value: i32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut usize, *mut i8, i32) -> u32>(IDX_MAKE_EXCHANGE_OPTION);
    let destroy: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_TRANSFER_OFFER_DESTRUCTOR_TRANSFER_OFFER);
    let free: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_OPERATOR_DELETE);
    let est = crate::rng::STATE_REGIONS;
    let regions = [(G_CONTRACT_LOOKUP_RESULT, 4usize), (out as usize, 4), (atracao as usize, 1), est[0], est[1]];
    let before = crate::comparison::MemorySnapshot::read(&regions);
    let r = o(this, buyer, target, offer, out, atracao, value);
    let after = crate::comparison::MemorySnapshot::read(&regions);
    let (ao, fo) = (*atracao, if r != 0 && *out != 0 { option_offer_snapshot(*out) } else { Vec::new() });
    let e_o = read_u32_as_usize(G_CONTRACT_LOOKUP_RESULT);
    before.write_back();
    let n = crate::rng::pause_trace(|| crate::rng::without_counting(|| make_exchange_option(this, buyer, target, offer, out, atracao, value)));
    let (an, fn_) = (*atracao, if n != 0 && *out != 0 { option_offer_snapshot(*out) } else { Vec::new() });
    let e_n = read_u32_as_usize(G_CONTRACT_LOOKUP_RESULT);
    if n != 0 && *out != 0 {
        let s = *out;
        destroy(s);
        free(s);
    }
    after.write_back();
    PLACAR.record(r == n && ao == an && e_o == e_n && fo == fn_, || format!(
        "make_exchange_option(comprador {:#x}, alvo {:#x}, valor {}): original {} rust {}, atração {} x {}, [G_CONTRACT_LOOKUP_RESULT] {:#x} x {:#x}, oferta {:x?} x {:x?}",
        buyer, target, value, r, n, ao, an, e_o, e_n, fo, fn_));
    r
}

/// Comparer for `accept_transfer_bid` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x8c0080: só a original (muda o status, reagenda, solta a troca, aloca notícias e chama a resposta da
/// IA).
pub(crate) unsafe extern "thiscall" fn comparar_accept_transfer_bid(this: usize, slot: usize, mark: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_ACCEPT_TRANSFER_BID)(this, slot, mark) // simplification: original only
}

/// Comparer for `post_offer_made_news` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x8d3090: só a original (a notícia vai aos técnicos e não tem outro efeito).
pub(crate) unsafe extern "thiscall" fn comparar_post_offer_made_news(this: usize, o: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_POST_OFFER_MADE_NEWS)(this, o) // simplification: only the news item
}

/// Comparer for `transfer_manager_can_loan_player` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x8d0040: lado a lado, com [0xae1080], o texto em [0xde1f64] e a mensagem (0x100 bytes), e o byte do
/// registro de 8 bytes do jogador em [[0xde1f54]+0x86e] (o único que ela pode apagar); a devolução também. As regras,
/// o período e as datas não sorteiam nem gravam fora das locais.
pub(crate) unsafe extern "thiscall" fn comparar_transfer_manager_can_loan_player(this: usize, clube: usize, player: usize, msg: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32>(IDX_TRANSFER_MANAGER_CAN_LOAN_PLAYER);
    let mut regions = [(G_CONTRACT_LOOKUP_RESULT, 4usize), (0xde1f64, 0x100), (0, 0), (0, 0)];
    let mut k = 2;
    if msg != 0 {
        regions[k] = (msg, 0x100);
        k += 1;
    }
    let gm = read_u32_as_usize(G_TRANSFER_MANAGER);
    if gm != 0 && read_u32_as_usize(gm + 0x86e) != 0 {
        regions[k] = (read_u32_as_usize(gm + 0x86e).wrapping_add(read_u32_as_usize(player).wrapping_mul(8)), 1);
        k += 1;
    }
    let (r, n, igual, _, _) = crate::comparison::side_by_side(&regions[..k], || o(this, clube, player, msg),
        || crate::rng::without_counting(|| transfer_manager_can_loan_player(this, clube, player, msg)));
    PLACAR.record(igual && r == n, || format!("transfer_manager_can_loan_player(clube {:#x}, jogador {:#x}): original {} rust {}, memória {}",
        clube, player, r, n, if igual { "igual" } else { "diferente" }));
    r
}

/// Comparer for `remove_exchange_player` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x8c41c0: só a original (os nós da troca são liberados, a notícia vai aos técnicos e a oferta segue
/// pela 0x8c2f50, que pode fechar a transferência e tirar outros jogadores das trocas, ou ser respondida pelo vendedor
/// do computador, 0x8c8090, que pode mudar a oferta da vaga).
pub(crate) unsafe extern "thiscall" fn comparar_remove_exchange_player(this: usize, person: usize, offer: usize, so_esta: u32, reason: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32)>(IDX_REMOVE_EXCHANGE_PLAYER)(this, person, offer, so_esta, reason) // simplification: o efeito segue por outras rotinas
}

/// Comparer for `check_offer_work_permits` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x8c7690: só a original (as notícias alocam, a oferta é reagendada e segue pela 0x8c2f50). O plano,
/// feito antes pelas consultas sem efeito (0x8c7560, 0x4cc9e0, 0x8c7950 sem sorteio): o alvo recusado devolve 0 com a
/// oferta no status 0x10; senão 1.
pub(crate) unsafe extern "thiscall" fn comparar_check_offer_work_permits(this: usize, offer: usize, quieto: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_CHECK_OFFER_WORK_PERMITS);
    let (precisa, proposta, concedida) = check_work_permit(this, transfer_manager_staff_by_id(read_u32_as_usize(offer + offer::TARGET_STAFF_ID)), transfer_manager_club_by_id(read_u32_as_usize(offer + 8)));
    let recusada = precisa && proposta != 0 && !concedida;
    let r = o(this, offer, quieto);
    let ok = if recusada { r == 0 && read_u8(offer + offer::STATUS) == 0x10 } else { r == 1 };
    PLACAR.record(ok, || format!("check_offer_work_permits(oferta {:x}) recusada prevista {} devolveu {}", offer, recusada, r));
    r
}

/// Comparer for `transfer_manager_daily_update` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x8c5880: só a original (ela processa as ofertas, que mudam elencos, finanças e notícias). Confere o
/// que ela garante, contra o estado de antes: rodando o dia (hoje igual à base e a fatia igual a +0x855), na fatia 2 a
/// base vira amanhã, +0x855 volta a 0 e +0x84c sobe (de 49 a 0); nas outras +0x855 sobe 1; e no dia 100 da fatia 2
/// cada pessoa que cumpre a condição (o laço é o último passo, então o estado de depois é o dele) fica com +0x1e (a
/// segunda nacionalidade) = a nação do clube: a naturalização do dia 100.
pub(crate) unsafe extern "thiscall" fn comparar_transfer_manager_daily_update(this: usize, today: usize, slice: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_TRANSFER_MANAGER_DAILY_UPDATE);
    let f = slice as u8;
    let sem_base = read_i16(this + 0x84f) == 0;
    let roda = if sem_base {
        true
    } else {
        read_i16(today + 2) == read_i16(this + 0x84f) && read_i16(today) == read_i16(this + 0x84d) && read_u8(this + 0x855) == f
    };
    let (f855, j84c) = (if sem_base { f } else { read_u8(this + 0x855) }, read_u8(this + 0x84c));
    let mut amanha = [0u8; 8];
    {
        let more: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_DATE_ADD_DAYS);
        let r = more(today, amanha.as_mut_ptr() as usize, 1);
        amanha = core::ptr::read_unaligned(r as *const [u8; 8]);
    }
    o(this, today, slice);
    if !roda {
        return;
    }
    let mut ok;
    if f == 2 {
        let j = j84c.wrapping_add(1);
        let expected = if (j as i8) >= 0x32 { 0 } else { j };
        let base = core::ptr::read_unaligned((this + 0x84d) as *const [u8; 4]);
        ok = read_u8(this + 0x855) == 0 && read_u8(this + 0x84c) == expected && base[..] == amanha[..4];
    } else {
        ok = read_u8(this + 0x855) == f855.wrapping_add(1);
    }
    let mut erradas = 0u32;
    if f == 2 && read_i16(today) == 100 {
        let new: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE);
        let comunitaria: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_STAFF_EU_NATIONAL);
        let year = read_i16(today + 2) as u16;
        let (mut d5, mut d2, mut d10) = ([0u8; 8], [0u8; 8], [0u8; 8]);
        let p5 = new(d5.as_mut_ptr() as usize, 100, year.wrapping_sub(5) as u32);
        let p2 = new(d2.as_mut_ptr() as usize, 100, year.wrapping_sub(2) as u32);
        let p10 = new(d10.as_mut_ptr() as usize, 100, year.wrapping_sub(10) as u32);
        let mut s = read_u32_as_usize(G_STAFF_TABLE);
        for _ in 0..(read_u32_as_usize(G_STAFF_COUNT) as i32).max(0) {
            if meets_residency_naturalisation(s, p2, p5, p10, comunitaria) && read_u32_as_usize(s + staff::SECOND_NATION) != read_u32_as_usize(read_u32_as_usize(s + staff::CLUB_JOB) + 0x53) {
                erradas += 1;
            }
            s += 0x6e;
        }
        ok = ok && erradas == 0;
    }
    PLACAR.record(ok, || format!("transfer_manager_daily_update(dia {}, fatia {}): +0x855 {} -> {}, +0x84c {} -> {}, pessoas sem a nação do clube em +0x1e {}",
        read_i16(today), f, f855, read_u8(this + 0x855), j84c, read_u8(this + 0x84c), erradas));
}

/// Comparer for `process_due_offer` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x8c63a0: só a original (cada ramo muda o status, reagenda, aloca notícias, conclui ou tira a oferta).
pub(crate) unsafe extern "thiscall" fn comparar_process_due_offer(this: usize, slot: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_PROCESS_DUE_OFFER)(this, slot) // simplification: original only
}

/// Comparer for `check_offer_restrictions` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x8cfc90: só a original (a vaga no elenco, 0x8cd450, pode liberar o jogador mais barato, e a lista de
/// observação aloca nós).
pub(crate) unsafe extern "thiscall" fn comparar_check_offer_restrictions(this: usize, o: usize, decide: usize, reason: *mut u16, without_squad: u32) -> u8 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, *mut u16, u32) -> u8>(IDX_CHECK_OFFER_RESTRICTIONS)(this, o, decide, reason, without_squad) // simplification: original only
}

/// Comparer for `pay_former_club_fee_cuts` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x8e4f20: a cópia em ensaio (sem a notícia) e a original a partir do mesmo estado, lado a lado nas
/// finanças do comprador (0x167 bytes) e em [0xae1080]; com entrega da notícia, só as finanças.
pub(crate) unsafe extern "thiscall" fn comparar_pay_former_club_fee_cuts(this: usize, o: usize) {
    let original = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_PAY_FORMER_CLUB_FEE_CUTS);
    let fin = european_cup_club_finance(friendly_club_by_id(read_usize(o + 8)));
    FORMER_CLUB_FEE_DELIVERIES = 0;
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&[(fin, 0x167), (G_CONTRACT_LOOKUP_RESULT, 4)], || original(this, o), || {
        FORMER_CLUB_FEE_DRY_RUN = true;
        crate::rng::without_counting(|| pay_former_club_fee_cuts(this, o));
        FORMER_CLUB_FEE_DRY_RUN = false;
    });
    let entregas = FORMER_CLUB_FEE_DELIVERIES;
    let same = same || (entregas != 0 && mo.0[0] == mr.0[0]);
    PLACAR.record(same, || format!("pay_former_club_fee_cuts(oferta {:#x}) entregas={} original={:x?} rust={:x?}", o, entregas, mo, mr));
}

/// Comparer for `pay_next_sale_percentage` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x8cb0f0: a cópia em ensaio (sem a notícia e sem os totais do clube) e a original a partir do mesmo
/// estado, lado a lado nas finanças dos dois clubes, em [0xae1080] e no sorteio (a 0x4d0c60 pode sortear); pago (a
/// notícia e os totais da original podem regravar [0xae1080]), só as finanças e a devolução.
pub(crate) unsafe extern "thiscall" fn comparar_pay_next_sale_percentage(this: usize, p: usize, o: usize) -> u32 {
    let original = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_PAY_NEXT_SALE_PERCENTAGE);
    let mut regions = vec![(G_CONTRACT_LOOKUP_RESULT, 4usize)];
    regions.extend(crate::rng::STATE_REGIONS);
    if let Some((a, b)) = clause_finance_regions(this, p) {
        regions.push((a, 0x167));
        regions.push((b, 0x167));
    }
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&regions, || original(this, p, o), || {
        FORMER_CLUB_FEE_DRY_RUN = true;
        let n = crate::rng::without_counting(|| pay_next_sale_percentage(this, p, o));
        FORMER_CLUB_FEE_DRY_RUN = false;
        n
    });
    let same = r == n && (same || (r != 0 && mo.0[3..] == mr.0[3..]));
    PLACAR.record(same, || format!("pay_next_sale_percentage(pessoa {:#x}, oferta {:#x}) original={} {:x?} rust={} {:x?}", p, o, r, mo, n, mr));
    r
}

/// Comparer for `post_sale_percentage_news` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x8db3a0: só a original (a notícia é entregue e não pode sair duas vezes; a cópia leva o ramo no
/// modo Rust). simplification: sem plano a conferir além do que a original entrega.
pub(crate) unsafe extern "thiscall" fn comparar_post_sale_percentage_news(this: usize, e: usize, parte: i32, o: usize, of_offer: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, usize, u32)>(IDX_POST_SALE_PERCENTAGE_NEWS)(this, e, parte, o, of_offer)
}

/// Comparer for `is_work_permit_granted` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x8c7950: lado a lado nas marcas da pessoa (8 bytes), no sorteio e na devolução (a cópia em
/// sem_contar).
pub(crate) unsafe extern "thiscall" fn comparar_is_work_permit_granted(this: usize, p: usize, nacao: usize, offer: usize, record: usize, sorteio: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, u32) -> u32>(IDX_IS_WORK_PERMIT_GRANTED);
    let mut regions = vec![crate::rng::STATE_REGIONS[0], crate::rng::STATE_REGIONS[1]];
    if p != 0 && (read_usize(p) as i32) < read_usize(G_STAFF_COUNT) as i32 {
        regions.push((staff_flags_record(this, p), 8));
    }
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&regions, || o(this, p, nacao, offer, record, sorteio),
        || crate::rng::without_counting(|| is_work_permit_granted(this, p, nacao, offer, record, sorteio)));
    PLACAR.record(igual && r == n, || format!("is_work_permit_granted(pessoa {:#x}, oferta {:#x}, registro {:#x}, sorteio {}) original={} {:x?} rust={} {:x?}", p, offer, record, sorteio, r, mo, n, mr));
    r
}

/// Comparador da 0x8c5800: lado a lado em [0xae1080] e no al devolvido.
pub(crate) unsafe extern "thiscall" fn comparar_transfer_manager_get_squad_status_for_reason(this: usize, p: usize, reason: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_TRANSFER_MANAGER_GET_SQUAD_STATUS_FOR_REASON);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(G_CONTRACT_LOOKUP_RESULT, 4)], || o(this, p, reason), || transfer_manager_get_squad_status_for_reason(this, p, reason));
    PLACAR.record(igual && r as u8 == n as u8, || format!("transfer_manager_get_squad_status_for_reason(pessoa {:#x}, motivo {:#x}) original={} {:x?} rust={} {:x?}", p, reason, r as u8, mo, n as u8, mr));
    r
}

/// Comparer for `get_negotiation_slot` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x8c6200: pelo valor (só lê).
pub(crate) unsafe extern "thiscall" fn comparar_get_negotiation_slot(this: usize, i: i32) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, i32) -> usize>(IDX_GET_NEGOTIATION_SLOT)(this, i);
    let n = get_negotiation_slot(this, i);
    PLACAR.record(r == n, || format!("get_negotiation_slot({}) original={:#x} rust={:#x}", i, r, n));
    r
}

/// Comparer for `reject_loan_offer` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x8bfa30: o da recusa e da retirada (`compare_reject_withdraw`: a cópia em ensaio e a original do mesmo estado;
/// iguais o estado, +0x10, a marca +4 dos jogadores da troca e a marca de oferta da pessoa).
pub(crate) unsafe extern "thiscall" fn comparar_reject_loan_offer(this: usize, slot: *mut usize, mark: u32, reason: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *mut usize, u32, u32)>(IDX_REJECT_LOAN_OFFER);
    compare_reject_withdraw("reject_loan_offer", this, slot, || o(this, slot, mark, reason), || reject_loan_offer(this, slot, mark, reason))
}

/// Comparer for `check_squad_limits` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x8cd450: a cópia em ensaio (sem liberar) e depois a original, do mesmo estado ([0xae1080], o motivo,
/// os cursores da fila livre e da troca). Sem liberação: iguais a devolução e essas regiões. Com liberação (a cópia
/// guardou o escolhido): iguais a devolução e o escolhido ter saído do clube (o contrato dele, pela 0x4cf5a0 que só
/// lê, sem o clube); o resto a original refaz ao liberar. Clube nulo: a caixa, só a original.
pub(crate) unsafe extern "thiscall" fn comparar_check_squad_limits(this: usize, clube: usize, o: usize, count_queue: u32, reason: *mut u16) -> u32 {
    let f = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, *mut u16) -> u32>(IDX_CHECK_SQUAD_LIMITS);
    if clube == 0 {
        return f(this, clube, o, count_queue, reason);
    }
    let mut regions = vec![(G_CONTRACT_LOOKUP_RESULT, 4usize), (read_usize(G_TRANSFER_MANAGER) + 0x936, 4)];
    if !reason.is_null() {
        regions.push((reason as usize, 2));
    }
    if read_usize(o + 4) != 0 {
        regions.push((read_usize(o + 4) + 0xa, 4));
    }
    let before = crate::comparison::MemorySnapshot::read(&regions);
    SQUAD_LIMITS_DRY_RUN = true;
    SQUAD_LIMITS_RELEASED_PLAYER = 0;
    let n = crate::rng::pause_trace(|| check_squad_limits(this, clube, o, count_queue, reason));
    SQUAD_LIMITS_DRY_RUN = false;
    let liberado = SQUAD_LIMITS_RELEASED_PLAYER;
    let mr = crate::comparison::MemorySnapshot::read(&regions);
    before.write_back();
    let r = f(this, clube, o, count_queue, reason);
    let mo = crate::comparison::MemorySnapshot::read(&regions);
    let same = if liberado == 0 {
        mo.0 == mr.0
    } else {
        let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
        let c = contrato(read_usize(G_CONTRACT_MANAGER), liberado);
        c == 0 || friendly_club_by_id(read_usize(c + 4)) != clube
    };
    PLACAR.record(same && r == n, || {
        format!("check_squad_limits(clube {:x}, oferta {:x}, fila {}) original={} rust={} liberado={:x} {:x?} x {:x?}", clube, o, count_queue, r, n, liberado, mo, mr)
    });
    r
}

/// Comparer for `find_next_sale_percentage` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x8d0dd0: lado a lado na marca da pessoa, em [0xae1080] e na devolução (só lê o resto).
pub(crate) unsafe extern "thiscall" fn comparar_find_next_sale_percentage(this: usize, p: usize) -> usize {
    let f = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_FIND_NEXT_SALE_PERCENTAGE);
    let m = read_usize(this + 0x86e).wrapping_add(read_usize(p).wrapping_mul(8));
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&[(m, 1), (G_CONTRACT_LOOKUP_RESULT, 4)], || f(this, p), || find_next_sale_percentage(this, p));
    PLACAR.record(igual && r == n, || format!("find_next_sale_percentage(pessoa {:x}) original={:x} rust={:x} {:x?} x {:x?}", p, r, n, mo, mr));
    r
}

/// Comparer for `update_appearance_clauses_after_match` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a cópia em ensaio (sem pagar os prêmios) e a original a partir do mesmo estado do vetor das permissões
/// de trabalho; os jogos (+4) e as faltas (+6) têm de bater. Seleção em casa: o elenco local da original começa com o
/// lixo da pilha, só a original roda. Os prêmios se provam pela própria 0x8cb340 (comparada) e pela carreira.
pub(crate) unsafe extern "thiscall" fn comparar_update_appearance_clauses_after_match(this: usize, jogo: usize, partida: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_UPDATE_APPEARANCE_CLAUSES_AFTER_MATCH);
    let is_national_team: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let v = read_usize(this + 0x93a);
    let n = read_usize(this + 0x93e) as i32;
    if is_national_team(read_usize(jogo + 0x1c)) != 0 || v == 0 || n <= 0 {
        return o(this, jogo, partida); // simplification: original only (stack garbage or no permissions)
    }
    let regions = [(v, n as usize * 0xe)];
    let before = crate::comparison::MemorySnapshot::read(&regions);
    APPEARANCE_CLAUSES_DRY_RUN = true;
    crate::rng::pause_trace(|| crate::rng::without_counting(|| update_appearance_clauses_after_match(this, jogo, partida)));
    APPEARANCE_CLAUSES_DRY_RUN = false;
    let mr = crate::comparison::MemorySnapshot::read(&regions);
    before.write_back();
    o(this, jogo, partida);
    // the original does not reallocate the vector (the awards touch other vectors)
    let mo = crate::comparison::MemorySnapshot::read(&regions);
    PLACAR.record(mo == mr, || {
        let (a, b) = (&mo.0[0].1, &mr.0[0].1);
        let dif: Vec<usize> = (0..a.len()).filter(|&i| a[i] != b[i]).take(16).collect();
        format!("update_appearance_clauses_after_match(jogo {:x}) permissões diferentes nos bytes {:x?}", jogo, dif)
    });
}

/// Comparer for `add_international_team_appearance_money` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a original acrescenta (pode realocar) e não pode rodar duas vezes: o plano antes (a conta e os 13
/// bytes), a original roda; confere a conta e que o registro está no vetor (reordenado).
pub(crate) unsafe extern "thiscall" fn comparar_add_international_team_appearance_money(this: usize, record: usize) {
    let n = read_usize(this + 0x8a0);
    let target = core::slice::from_raw_parts(record as *const u8, 0xd).to_vec();
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_ADD_INTERNATIONAL_TEAM_APPEARANCE_MONEY)(this, record);
    if read_usize(this + 0x898) == 0 {
        return; // out of memory: the box has already shown
    }
    let count = read_usize(this + 0x8a0);
    let ok = count == n.wrapping_add(1) && vector_contains_record(read_usize(this + 0x898), count, 0xd, &target);
    PLACAR.record(ok, || format!("add_international_team_appearance_money: conta {} -> {}, registro {:x?}", n, count, target));
}

/// Comparer for `post_transfer_finalised_news` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_post_transfer_finalised_news(this: usize, o: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_POST_TRANSFER_FINALISED_NEWS)(this, o)
}

/// Comparer for `free_transfer_staff` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_free_transfer_staff(this: usize, p: usize, clube: usize, reason: u32, compensacao: i32, so_record: u32) {
    let sheet: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_LOAN_CONTRACT);
    let emprestada = sheet(read_usize(G_CONTRACT_MANAGER), p) != 0;
    let n = read_usize(this + 0x86a);
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, i32, u32)>(IDX_FREE_TRANSFER_STAFF)(this, p, clube, reason, compensacao, so_record);
    if emprestada {
        PLACAR.record(read_usize(this + 0x86a) == n, || format!("free_transfer_staff(pessoa {}) emprestada: conta {} -> {}", read_usize(p), n, read_usize(this + 0x86a)));
        return;
    }
    if read_usize(this + 0x862) == 0 {
        return; // out of memory: the box has already shown
    }
    let r = read_usize(this + 0x862).wrapping_add(n.wrapping_mul(0x25));
    let mut ok = read_usize(this + 0x86a) == n.wrapping_add(1)
        && read_usize(r) == read_usize(p)
        && read_usize(r + 0x10) as u32 == u32::MAX
        && read_usize(r + 0x14) as u32 == if clube != 0 { read_usize(clube) as u32 } else { u32::MAX }
        && read_u8(r + 0x1c) == if so_record != 0 { 5 } else { 3 };
    if so_record == 0 {
        ok = ok && get_contract_unstored(p) == 0 && read_usize(p + staff::VALUE) == 0 && read_u8(p + staff::SQUAD_SELECTED_FOR) == 0;
    }
    PLACAR.record(ok, || format!("free_transfer_staff(pessoa {}, clube {:x}, só registro {}): conta {} -> {}, registro {:x?}", read_usize(p), clube, so_record,
        n, read_usize(this + 0x86a), core::slice::from_raw_parts(r as *const u8, 0x1d)));
}

/// Comparer for `post_contract_talks_news` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_post_contract_talks_news(this: usize, o: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_POST_CONTRACT_TALKS_NEWS)(this, o)
}

/// Comparer for `reject_transfer_offer` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_reject_transfer_offer(this: usize, slot: *mut usize, mark: u32, reason: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *mut usize, u32, u32)>(IDX_REJECT_TRANSFER_OFFER);
    compare_reject_withdraw("reject_transfer_offer", this, slot, || o(this, slot, mark, reason), || reject_transfer_offer(this, slot, mark, reason))
}

/// Comparer for `withdraw_transfer_offer` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_withdraw_transfer_offer(this: usize, slot: *mut usize, mark: u32, reason: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *mut usize, u32, u32)>(IDX_WITHDRAW_TRANSFER_OFFER);
    compare_reject_withdraw("withdraw_transfer_offer", this, slot, || o(this, slot, mark, reason), || withdraw_transfer_offer(this, slot, mark, reason))
}

/// Comparer for `post_offer_rejected_news` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_post_offer_rejected_news(this: usize, o: usize, reason: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_POST_OFFER_REJECTED_NEWS)(this, o, reason)
}

/// Comparer for `unsettle_player_after_bid` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador: a cópia em ensaio e a original a partir do mesmo estado (a oferta, [0xae1080] e o sorteio). Sem chegar
/// à queixa (quase sempre), tudo igual; chegando, só a original (a queixa, a fuga e as antipatias não podem rodar duas
/// vezes e a original sorteia mais).
pub(crate) unsafe extern "thiscall" fn comparar_unsettle_player_after_bid(this: usize, o: usize) {
    let orig_f = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_UNSETTLE_PLAYER_AFTER_BID);
    let mut regions = vec![(o, 0x30usize), (G_CONTRACT_LOOKUP_RESULT, 4)];
    regions.extend(crate::rng::STATE_REGIONS);
    let before = crate::comparison::MemorySnapshot::read(&regions);
    UNSETTLE_PLAYER_DRY_RUN = true;
    UNSETTLE_PLAYER_REACHED = false;
    crate::rng::pause_trace(|| crate::rng::without_counting(|| unsettle_player_after_bid(this, o)));
    UNSETTLE_PLAYER_DRY_RUN = false;
    let mr = crate::comparison::MemorySnapshot::read(&regions);
    before.write_back();
    orig_f(this, o);
    if UNSETTLE_PLAYER_REACHED {
        return;
    }
    let mo = crate::comparison::MemorySnapshot::read(&regions);
    PLACAR.record(mo == mr, || format!("unsettle_player_after_bid(oferta {:#x}) original={:x?} rust={:x?}", o, mo, mr));
}

/// Comparer for `post_offer_collapsed_news` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_post_offer_collapsed_news(this: usize, o: usize, clube: usize, reason: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(IDX_POST_OFFER_COLLAPSED_NEWS)(this, o, clube, reason)
}

/// Comparer for `post_appearance_money_news` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_post_appearance_money_news(this: usize, clausula: usize, value: u32, kind: u32, calado: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32)>(IDX_POST_APPEARANCE_MONEY_NEWS)(this, clausula, value, kind, calado)
}

/// Comparer for `finalise_transfer` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_finalise_transfer(this: usize, o: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_FINALISE_TRANSFER)(this, o);
    let e = read_u8(o + 0x2c);
    PLACAR.record(e == 0x14, || format!("finalise_transfer(oferta {:#x}) estado={:#x} (plano 0x14)", o, e));
}

/// Comparer for `post_transfer_completed_news` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_post_transfer_completed_news(this: usize, o: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_POST_TRANSFER_COMPLETED_NEWS)(this, o)
}

/// Comparer for `post_transfer_agreed_news` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_post_transfer_agreed_news(this: usize, o: usize, data: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_POST_TRANSFER_AGREED_NEWS)(this, o, data)
}

/// Comparer for `add_config_loan` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_add_config_loan(this: usize, p: usize, clube: usize, start: usize, end: usize) {
    let emprestado: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_LOAN_CONTRACT);
    let tinha = emprestado(read_usize(G_CONTRACT_MANAGER), p);
    let passa = config_loan_allowed(p, clube, start, end).is_some();
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize)>(IDX_ADD_CONFIG_LOAN)(this, p, clube, start, end);
    let has = emprestado(read_usize(G_CONTRACT_MANAGER), p);
    let same = if passa { has != 0 } else { has == tinha };
    PLACAR.record(same, || format!("add_config_loan(pessoa {:#x}) passa={} antes={:#x} depois={:#x}", read_usize(p), passa, tinha, has));
}

/// Comparer for `post_transfer_delay_request_news` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_post_transfer_delay_request_news(this: usize, o: usize, reason: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_POST_TRANSFER_DELAY_REQUEST_NEWS)(this, o, reason)
}

/// Comparer for `confirm_offer_by_club` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: sem chegar ao acordo dos dois (estado final diferente de 0xe), só o estado muda: lado a lado sobre a
/// oferta. Com o acordo, a oferta é agendada ou vira transferência futura (mexe nas outras ofertas e aloca): só a
/// original.
pub(crate) unsafe extern "thiscall" fn comparar_confirm_offer_by_club(this: usize, offer: usize, clube: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_CONFIRM_OFFER_BY_CLUB);
    if state_after_approval(offer, clube) == 0xe {
        return o(this, offer, clube);
    }
    let regions = [(offer, 0x31usize)];
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, offer, clube), || confirm_offer_by_club(this, offer, clube));
    PLACAR.record(igual, || format!("confirm_offer_by_club(oferta {:#x}, clube {:#x}) original={:x?} rust={:x?}", offer, clube, mo, mr));
}

/// Comparer for `try_complete_offer` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_try_complete_offer(this: usize, offer: usize, no_prazo: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_TRY_COMPLETE_OFFER)(this, offer, no_prazo)
}

/// Comparer for `settle_previous_transfer_clauses` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: sem marca de cláusula (bits 2, 4 e 8) no jogador nem nas pessoas da troca, só o cursor da lista muda:
/// lado a lado sobre a lista e as marcas. Com alguma, paga (finanças, notícias que alocam): só a original.
pub(crate) unsafe extern "thiscall" fn comparar_settle_previous_transfer_clauses(this: usize, offer: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_SETTLE_PREVIOUS_TRANSFER_CLAUSES);
    let marks = read_usize(this + 0x86e);
    let has = |id: usize| read_u8(marks.wrapping_add(id.wrapping_mul(8))) & 0xe != 0;
    let l = read_usize(offer + 4);
    let mut regions = vec![(marks.wrapping_add(read_usize(offer + offer::TARGET_STAFF_ID).wrapping_mul(8)), 8usize)];
    let mut alguma = has(read_usize(offer + offer::TARGET_STAFF_ID));
    if l != 0 {
        regions.push((l, 0xe));
        // the traversal without moving the list cursor: the nodes (+0 the item, +6 the next) starting from the head +2
        let mut no = read_usize(l + 2);
        let mut n = 0;
        while no != 0 && n < 64 {
            let item = read_usize(no);
            if item != 0 && has(read_usize(item)) {
                alguma = true;
            }
            no = read_usize(no + 6);
            n += 1;
        }
        if n >= 64 {
            alguma = true;
        }
    }
    if alguma {
        return o(this, offer);
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, offer), || settle_previous_transfer_clauses(this, offer));
    PLACAR.record(igual, || format!("settle_previous_transfer_clauses(oferta {:#x}) original={:x?} rust={:x?}", offer, mo, mr));
}

/// Comparer for `advance_offer_to_club_approval` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_advance_offer_to_club_approval(this: usize, offer: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_ADVANCE_OFFER_TO_CLUB_APPROVAL)(this, offer)
}

/// Comparer for `post_offer_accepted_news` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_post_offer_accepted_news(this: usize, offer: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_POST_OFFER_ACCEPTED_NEWS)(this, offer)
}

/// Comparer for `post_loan_offer_news` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_post_loan_offer_news(this: usize, offer: usize, terms: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_POST_LOAN_OFFER_NEWS)(this, offer, terms)
}

/// Comparer for `add_transfer_negotiation` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_add_transfer_negotiation(this: usize, offer: usize, terms: usize, mark: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(IDX_ADD_TRANSFER_NEGOTIATION)(this, offer, terms, mark)
}

/// Comparer for `post_player_rejects_move_news` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_post_player_rejects_move_news(this: usize, offer: usize, reason: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_POST_PLAYER_REJECTS_MOVE_NEWS)(this, offer, reason)
}

/// Comparer for `init_work_permits` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_init_work_permits(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_INIT_WORK_PERMITS)(this)
}

/// Comparer for `get_clause_count` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor (só lê).
pub(crate) unsafe extern "thiscall" fn comparar_get_clause_count(this: usize, person: usize, kind: u32) -> u16 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u16>(IDX_GET_CLAUSE_COUNT)(this, person, kind);
    let n = get_clause_count(this, person, kind);
    PLACAR.record(r == n, || format!("get_clause_count(pessoa {}, tipo {}) original={} rust={}", read_usize(person), kind as u8, r, n));
    r
}

/// Comparer for `find_first_team_appearance_money` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelo byte do registro da pessoa, e pela devolução.
pub(crate) unsafe extern "thiscall" fn comparar_find_first_team_appearance_money(this: usize, person: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_FIND_FIRST_TEAM_APPEARANCE_MONEY);
    let m = read_usize(this + 0x86e).wrapping_add(read_usize(person).wrapping_mul(8));
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&[(m, 1)], || o(this, person), || find_first_team_appearance_money(this, person));
    PLACAR.record(igual && r == n, || format!("find_first_team_appearance_money(pessoa {}) original={:x} rust={:x} {:x?} x {:x?}", read_usize(person), r, n, mo, mr));
    r
}

/// Comparer for `process_free_transfer_queue` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_process_free_transfer_queue(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_PROCESS_FREE_TRANSFER_QUEUE)(this)
}

/// Comparer for `find_next_window_open_date` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pela data de saída (a data estática da 0x54c5b0 é rascunho). Sem janela num ano, a caixa
/// sairia duas vezes (não acontece com as regras do jogo).
pub(crate) unsafe extern "thiscall" fn comparar_find_next_window_open_date(this: usize, clube: usize, out: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_FIND_NEXT_WINDOW_OPEN_DATE);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&[(out, 8)], || o(this, clube, out), || find_next_window_open_date(this, clube, out));
    PLACAR.record(igual, || format!("find_next_window_open_date(clube {:x}) {:x?} x {:x?}", clube, mo, mr));
}

/// Comparer for `find_negotiated_loan_contract` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor (as duas voltas deixam o cursor da lista no mesmo lugar).
pub(crate) unsafe extern "thiscall" fn comparar_find_negotiated_loan_contract(this: usize, person: usize, clube: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> usize>(IDX_FIND_NEGOTIATED_LOAN_CONTRACT)(this, person, clube);
    let n = find_negotiated_loan_contract(this, person, clube);
    PLACAR.record(r == n, || format!("find_negotiated_loan_contract(pessoa {}, clube {}) original={:x} rust={:x}", read_usize(person), read_usize(clube), r, n));
    r
}

/// Comparer for `build_clause_offer_option` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_build_clause_offer_option(this: usize, clube: usize, player: usize, out: *mut usize, apelo: *mut u8, value: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, *mut usize, *mut u8, u32) -> u32>(IDX_BUILD_CLAUSE_OFFER_OPTION)(this, clube, player, out, apelo, value)
}

/// Comparer for `transfer_manager_is_rejection_reason_valid` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelo contrato da vez ([0xae1080]), e pela devolução.
pub(crate) unsafe extern "stdcall" fn comparar_transfer_manager_is_rejection_reason_valid(reason: u32, offer: usize) -> u32 {
    let o = original_fn::<unsafe extern "stdcall" fn(u32, usize) -> u32>(IDX_TRANSFER_MANAGER_IS_REJECTION_REASON_VALID);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&[(G_CONTRACT_LOOKUP_RESULT, 4)], || o(reason, offer), || transfer_manager_is_rejection_reason_valid(reason, offer));
    PLACAR.record(igual && r == n, || format!("transfer_manager_is_rejection_reason_valid({:x}, pessoa {}) original={} rust={} {:x?} x {:x?}", reason as u16, read_usize(offer + offer::TARGET_STAFF_ID), r, n, mo, mr));
    r
}

/// Comparer for `transfer_manager_get_squad_status_reason` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelo contrato da vez ([0xae1080]), e pela devolução.
pub(crate) unsafe extern "stdcall" fn comparar_transfer_manager_get_squad_status_reason(offer: usize) -> u16 {
    let o = original_fn::<unsafe extern "stdcall" fn(usize) -> u16>(IDX_TRANSFER_MANAGER_GET_SQUAD_STATUS_REASON);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&[(G_CONTRACT_LOOKUP_RESULT, 4)], || o(offer), || transfer_manager_get_squad_status_reason(offer));
    PLACAR.record(igual && r == n, || format!("transfer_manager_get_squad_status_reason(pessoa {}) original={:x} rust={:x} {:x?} x {:x?}", read_usize(offer + offer::TARGET_STAFF_ID), r, n, mo, mr));
    r
}

/// Comparer for `fill_squad_restrictions` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelos 42 bytes da tabela (o alcance e a virtual das regras só escrevem nela).
pub(crate) unsafe extern "thiscall" fn comparar_fill_squad_restrictions(this: usize, partida: usize, t: *mut u8, so_main: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut u8, u32)>(IDX_FILL_SQUAD_RESTRICTIONS);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side_light(&[(t as usize, 42)], || o(this, partida, t, so_main), || fill_squad_restrictions(this, partida, t, so_main));
    PLACAR.record(igual, || format!("fill_squad_restrictions({:x}, {}) {:x?} x {:x?}", partida, so_main, mo, mr));
}

/// Comparer for `transfer_manager_check_matchday_squad_limit` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pela saída (6 bytes) e pelo aviso (0x100 bytes, com `msg`), e pela devolução. A 0x8968f0 pode
/// pôr −1 numa vaga sem pessoa válida: a original já deixou assim e a cópia acha a mesma vaga vazia.
pub(crate) unsafe extern "thiscall" fn comparar_transfer_manager_check_matchday_squad_limit(this: usize, clube: usize, partida: usize, msg: usize, players: usize, not_used: u32, out: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, u32, usize) -> u32>(IDX_TRANSFER_MANAGER_CHECK_MATCHDAY_SQUAD_LIMIT);
    let mut regions = [(0usize, 0usize); 2];
    let mut n = 0;
    if out != 0 {
        regions[n] = (out, 6);
        n += 1;
    }
    if msg != 0 {
        regions[n] = (msg, 0x100);
        n += 1;
    }
    let (r, c, igual, mo, mr) = crate::comparison::side_by_side(&regions[..n], || o(this, clube, partida, msg, players, not_used, out), || transfer_manager_check_matchday_squad_limit(this, clube, partida, msg, players, not_used, out));
    PLACAR.record(igual && r == c, || format!("transfer_manager_check_matchday_squad_limit(clube {:x}, partida {:x}) original={} rust={} {:x?} x {:x?}", clube, partida, r, c, mo, mr));
    r
}

/// Comparer for `player_needs_work_permit` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor (só lê). Jogador nulo com clube de nação: a caixa, só a original.
pub(crate) unsafe extern "thiscall" fn comparar_player_needs_work_permit(this: usize, player: usize, clube: usize, ignorar_a_dada: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32>(IDX_PLAYER_NEEDS_WORK_PERMIT);
    if player == 0 {
        return o(this, player, clube, ignorar_a_dada);
    }
    let r = o(this, player, clube, ignorar_a_dada);
    let n = player_needs_work_permit(this, player, clube, ignorar_a_dada);
    PLACAR.record(r == n, || format!("player_needs_work_permit(jogador {}, clube {:x}, {}) original={} rust={}", read_usize(player), clube, ignorar_a_dada, r, n));
    r
}

/// Comparer for `can_club_bid_for_player` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelo motivo, pela marca da pessoa (+0x86e) e pelo cursor da fila livre, e pela devolução.
/// As outras gravações (tirar a marca 0x40, marcar +0x2d da oferta em negociação) a cópia repete sobre o que a original
/// deixou e chega ao mesmo.
pub(crate) unsafe extern "thiscall" fn comparar_can_club_bid_for_player(this: usize, player: usize, clube: usize, reason: *mut u16, ignorar_abertas: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, *mut u16, u32) -> u32>(IDX_CAN_CLUB_BID_FOR_PLAYER);
    let mark = read_usize(this + 0x86e).wrapping_add(read_usize(player).wrapping_mul(8));
    let regions = [(mark, 1), (this + 0x936, 4), (reason as usize, 2)];
    let n = if reason.is_null() { 2 } else { 3 };
    let (r, c, igual, mo, mr) = crate::comparison::side_by_side_light(&regions[..n], || o(this, player, clube, reason, ignorar_abertas), || can_club_bid_for_player(this, player, clube, reason, ignorar_abertas));
    PLACAR.record(igual && r == c, || format!("can_club_bid_for_player(jogador {}, clube {:x}, {}) original={} rust={} {:x?} x {:x?}", read_usize(player), clube, ignorar_abertas, r, c, mo, mr));
    r
}

/// Comparer for `note_terms_offered_to_exchange_player` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelo estado da oferta, pelas marcas (+4) dos itens da troca e pelo cursor da lista. Fora da
/// conta: a caixa, só a original.
pub(crate) unsafe extern "thiscall" fn comparar_note_terms_offered_to_exchange_player(this: usize, k: i32, person: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32, u32)>(IDX_NOTE_TERMS_OFFERED_TO_EXCHANGE_PLAYER);
    if k < 0 || k >= read_usize(this + 4) as i32 || read_usize(read_usize(this).wrapping_add((k as usize).wrapping_mul(4))) == 0 {
        return o(this, k, person);
    }
    let of = read_usize(read_usize(this).wrapping_add((k as usize).wrapping_mul(4)));
    let l = read_usize(of + 4);
    let mut regions = vec![(of + 0x2c, 1), (l + 0xa, 4)];
    for e in llist_collect(l) {
        regions.push((e + 4, 1));
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, k, person), || note_terms_offered_to_exchange_player(this, k, person));
    PLACAR.record(igual, || format!("note_terms_offered_to_exchange_player({}, pessoa {}) {:x?} x {:x?}", k, person, mo, mr));
}

/// Comparer for `count_club_free_transfer_queue` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelo cursor da fila e pela devolução (só lê). Clube nulo: a caixa, só a original.
pub(crate) unsafe extern "thiscall" fn comparar_count_club_free_transfer_queue(this: usize, clube: usize) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_COUNT_CLUB_FREE_TRANSFER_QUEUE);
    if clube == 0 {
        return o(this, clube);
    }
    let (r, c, igual, mo, mr) = crate::comparison::side_by_side_light(&[(this + 0x936, 4)], || o(this, clube), || count_club_free_transfer_queue(this, clube));
    PLACAR.record(igual && r == c, || format!("count_club_free_transfer_queue({:x}) original={} rust={} {:x?} x {:x?}", clube, r, c, mo, mr));
    r
}

/// Comparer for `get_future_transfer` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pela marca do jogador e por `saida`, e pela devolução (tirar a marca é o único outro efeito;
/// a cópia, depois da original, acha o mesmo).
pub(crate) unsafe extern "thiscall" fn comparar_get_future_transfer(this: usize, player: usize, out: *mut usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut usize) -> u32>(IDX_GET_FUTURE_TRANSFER);
    let regions = [(staff_flags_byte(this, read_usize(player)) as usize, 1), (out as usize, 4)];
    let n = if out.is_null() { 1 } else { 2 };
    let (r, c, igual, mo, mr) = crate::comparison::side_by_side_light(&regions[..n], || o(this, player, out), || get_future_transfer(this, player, out));
    PLACAR.record(igual && r == c, || format!("get_future_transfer(jogador {}) original={} rust={} {:x?} x {:x?}", read_usize(player), r, c, mo, mr));
    r
}

/// Comparer for `remove_future_transfer` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pela conta e pelo vetor das futuras, pelas marcas do jogador e dos da troca e pelo cursor
/// da lista da troca (não aloca nem libera).
pub(crate) unsafe extern "thiscall" fn comparar_remove_future_transfer(this: usize, offer: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_REMOVE_FUTURE_TRANSFER);
    let mut regions = vec![(this + 0x928, 4), (staff_flags_byte(this, read_usize(offer + offer::TARGET_STAFF_ID)) as usize, 1)];
    let n = read_usize(this + 0x928) as i32;
    if n > 0 {
        regions.push((read_usize(this + 0x920), n as usize * 0xc));
    }
    let l = read_usize(offer + 4);
    if l != 0 {
        let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
        write_usize(l + 0xa, read_usize(l + 2));
        let mut e = next(l);
        while e != 0 {
            regions.push((staff_flags_byte(this, read_usize(e)) as usize, 1));
            e = next(l);
        }
        regions.push((l + 0xa, 4));
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, offer), || remove_future_transfer(this, offer));
    PLACAR.record(igual, || format!("remove_future_transfer(oferta {:x}) {:x?} x {:x?}", read_usize(offer + offer::SLOT_INDEX), mo, mr));
}

/// Comparer for `add_future_transfer` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a original acrescenta (pode realocar), reordena e suspende as listas de alvos: só ela, conferida pelo
/// plano: a conta sobe de 1, o registro (data e oferta) está no vetor, o estado 0x13 com +0x2d = 1 e a marca 0x10 do
/// jogador.
pub(crate) unsafe extern "thiscall" fn comparar_add_future_transfer(this: usize, offer: usize, data: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_ADD_FUTURE_TRANSFER);
    let n = read_usize(this + 0x928);
    let mut target = [0u8; 0xc];
    target[..8].copy_from_slice(core::slice::from_raw_parts(data as *const u8, 8));
    target[8..].copy_from_slice(&(read_usize(offer + offer::SLOT_INDEX) as u32).to_le_bytes());
    o(this, offer, data);
    if read_usize(this + 0x920) == 0 {
        return; // out of memory: the box has already shown
    }
    let count = read_usize(this + 0x928);
    let mark = *staff_flags_byte(this, read_usize(offer + offer::TARGET_STAFF_ID));
    let ok = count == n.wrapping_add(1) && vector_contains_record(read_usize(this + 0x920), count, 0xc, &target) && read_u8(offer + offer::STATUS) == 0x13 && read_u8(offer + 0x2d) == 1 && mark & 0x10 != 0;
    PLACAR.record(ok, || format!("add_future_transfer(oferta {:x}): conta {} -> {}, estado {:#x}, marca {:#x}", read_usize(offer + offer::SLOT_INDEX), n, count, read_u8(offer + offer::STATUS), mark));
}

/// Comparer for `get_open_offers_for_club_players` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: sem `lista`, lado a lado por [0xae1080] e pela devolução; com `lista` (cresce a lista: não pode rodar
/// duas vezes), a cópia conta antes sem lista e a original roda: confere a conta.
pub(crate) unsafe extern "thiscall" fn comparar_get_open_offers_for_club_players(this: usize, clube: usize, list: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_GET_OPEN_OFFERS_FOR_CLUB_PLAYERS);
    if list != 0 {
        let n = get_open_offers_for_club_players(this, clube, 0);
        let r = o(this, clube, list);
        PLACAR.record(r == n, || format!("get_open_offers_for_club_players({:x}, lista) original={} rust={}", clube, r, n));
        return r;
    }
    let (r, c, igual, mo, mr) = crate::comparison::side_by_side_light(&[(G_CONTRACT_LOOKUP_RESULT, 4)], || o(this, clube, 0), || get_open_offers_for_club_players(this, clube, 0));
    PLACAR.record(igual && r == c, || format!("get_open_offers_for_club_players({:x}) original={} rust={} {:x?} x {:x?}", clube, r, c, mo, mr));
    r
}

/// Comparer for `collapse_offers_for_player` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: agenda (aloca) e mexe nas listas de alvos: só a original. O plano antes: as ofertas pela pessoa (0x8c5cf0
/// numa lista própria, destruída depois); depois confere que todas ficaram no estado 0xf com +0x2e = 3.
pub(crate) unsafe extern "thiscall" fn comparar_collapse_offers_for_player(this: usize, person: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_COLLAPSE_OFFERS_FOR_PLAYER);
    if read_u8(read_usize(this + 0x86e).wrapping_add(read_usize(person).wrapping_mul(8)) + 1) == 0 {
        return o(this, person);
    }
    let nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let offers: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_OFFERS_FOR_PLAYER);
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let mut list = [0u8; 0x10];
    let l = list.as_mut_ptr() as usize;
    nova(l);
    offers(this, person, l, 0);
    let mut target = Vec::new();
    write_usize(l + 0xa, read_usize(l + 2));
    let mut e = next(l);
    while e != 0 {
        target.push(read_usize(read_usize(this).wrapping_add(read_usize(e).wrapping_mul(4))));
        e = next(l);
    }
    soltar(l);
    o(this, person);
    let ok = target.iter().all(|&of| read_u8(of + 0x2c) == 0xf && read_u8(of + 0x2e) == 3);
    PLACAR.record(ok, || format!("collapse_offers_for_player(pessoa {}): {} ofertas, estados {:x?}", read_usize(person), target.len(), target.iter().map(|&of| (read_u8(of + 0x2c), read_u8(of + 0x2e))).collect::<Vec<_>>()));
}

/// Comparer for `add_next_sale_percentage` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: o plano antes (a conta e os 13 bytes), a original roda; confere a conta e que o registro está no vetor
/// (reordenado).
pub(crate) unsafe extern "thiscall" fn comparar_add_next_sale_percentage(this: usize, record: usize) {
    let n = read_usize(this + 0x888);
    let target = core::slice::from_raw_parts(record as *const u8, 0xd).to_vec();
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_ADD_NEXT_SALE_PERCENTAGE)(this, record);
    if read_usize(this + 0x880) == 0 {
        return; // out of memory: the box has already shown
    }
    let count = read_usize(this + 0x888);
    let ok = count == n.wrapping_add(1) && vector_contains_record(read_usize(this + 0x880), count, 0xd, &target);
    PLACAR.record(ok, || format!("add_next_sale_percentage: conta {} -> {}, registro {:x?}", n, count, target));
}

/// Comparer for `transfer_manager_get_offer_completion_date` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pela saída, [0xae1080] e as estáticas das datas, e pelo motivo (as regras, a temporada e a
/// janela só calculam).
pub(crate) unsafe extern "thiscall" fn comparar_transfer_manager_get_offer_completion_date(this: usize, offer: usize, out: *mut [u32; 2]) -> u16 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut [u32; 2]) -> u16>(IDX_TRANSFER_MANAGER_GET_OFFER_COMPLETION_DATE);
    let reg = date_comparer_regions(&[out as usize]);
    let (ro, rr, igual, _, _) = crate::comparison::side_by_side(&reg, || o(this, offer, out), || transfer_manager_get_offer_completion_date(this, offer, out));
    PLACAR.record(igual && ro == rr, || format!("transfer_manager_get_offer_completion_date(tipo {}) motivo {} x {}, memória igual {}", read_u8(offer + offer::COMPLETION_TIMING) & 0xf, ro, rr, igual));
    ro
}

/// Comparer for `find_next_allowed_transfer_date` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pela saída, [0xae1080] e as estáticas das datas, e pelo motivo.
pub(crate) unsafe extern "thiscall" fn comparar_find_next_allowed_transfer_date(this: usize, offer: usize, out: *mut [u32; 2]) -> u16 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut [u32; 2]) -> u16>(IDX_FIND_NEXT_ALLOWED_TRANSFER_DATE);
    let reg = date_comparer_regions(&[out as usize]);
    let (ro, rr, igual, _, _) = crate::comparison::side_by_side(&reg, || o(this, offer, out), || find_next_allowed_transfer_date(this, offer, out));
    PLACAR.record(igual && ro == rr, || format!("find_next_allowed_transfer_date(tipo {}) motivo {} x {}, memória igual {}", read_u8(offer + offer::COMPLETION_TIMING) & 0xf, ro, rr, igual));
    ro
}

/// Comparer for `transfer_manager_get_next_window_dates` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelo início e o fim (só datas, sem estáticas).
pub(crate) unsafe extern "stdcall" fn comparar_transfer_manager_get_next_window_dates(start: *mut [u32; 2], end: *mut [u32; 2]) {
    let o = original_fn::<unsafe extern "stdcall" fn(*mut [u32; 2], *mut [u32; 2])>(IDX_TRANSFER_MANAGER_GET_NEXT_WINDOW_DATES);
    let reg = date_comparer_regions(&[start as usize, end as usize]);
    let (_, _, igual, _, _) = crate::comparison::side_by_side(&reg, || o(start, end), || transfer_manager_get_next_window_dates(start, end));
    PLACAR.record(igual, || format!("transfer_manager_get_next_window_dates(hoje {:x}, fim {}) diferente", *(TODAY as *const u32), !end.is_null()));
}

/// Comparer for `transfer_manager_calc_compensation_fee` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor (lado a lado pelas estáticas das datas; o valor do jogador 0x4d0c60 e a idade só calculam).
pub(crate) unsafe extern "stdcall" fn comparar_transfer_manager_calc_compensation_fee(person: usize, buyer: usize, sem_regras_novas: u32) -> i32 {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, usize, u32) -> i32>(IDX_TRANSFER_MANAGER_CALC_COMPENSATION_FEE);
    let (ro, rr, igual, _, _) = crate::comparison::side_by_side(&DATE_OP_STATICS, || o(person, buyer, sem_regras_novas), || transfer_manager_calc_compensation_fee(person, buyer, sem_regras_novas));
    PLACAR.record(igual && ro == rr, || format!("transfer_manager_calc_compensation_fee(pessoa {:x}) {} x {}, estáticas iguais {}", if person != 0 { read_usize(person) } else { 0 }, ro, rr, igual));
    ro
}

/// Comparer for `pay_fee_and_record_clauses` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_pay_fee_and_record_clauses(this: usize, offer: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_PAY_FEE_AND_RECORD_CLAUSES)(this, offer)
}

/// Comparer for `transfer_manager_selling_club_must_accept` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelo motivo e [0xae1080], e pelo valor devolvido (o preço e o valor ponderado só calculam).
pub(crate) unsafe extern "stdcall" fn comparar_transfer_manager_selling_club_must_accept(offer: usize, estrito: u32, reason: *mut u16) -> u32 {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, u32, *mut u16) -> u32>(IDX_TRANSFER_MANAGER_SELLING_CLUB_MUST_ACCEPT);
    let mut reg: Vec<(usize, usize)> = vec![(G_CONTRACT_LOOKUP_RESULT, 4)];
    if !reason.is_null() {
        reg.push((reason as usize, 2));
    }
    reg.extend_from_slice(&DATE_OP_STATICS);
    let (ro, rr, igual, _, _) = crate::comparison::side_by_side(&reg, || o(offer, estrito, reason), || transfer_manager_selling_club_must_accept(offer, estrito, reason));
    PLACAR.record(igual && ro == rr, || format!("transfer_manager_selling_club_must_accept(estrito {}) {} x {}, memória igual {}", estrito, ro, rr, igual));
    ro
}

/// Comparer for `update_work_permit_on_move` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_update_work_permit_on_move(this: usize, person: usize, clube: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_UPDATE_WORK_PERMIT_ON_MOVE)(this, person, clube)
}

/// Comparer for `complete_accepted_contract_offer` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_complete_accepted_contract_offer(this: usize, offer: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_COMPLETE_ACCEPTED_CONTRACT_OFFER)(this, offer)
}

/// Comparer for `offer_contract_to_offer_player` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_offer_contract_to_offer_player(this: usize, clube: usize, person: usize, offer: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32>(IDX_OFFER_CONTRACT_TO_OFFER_PLAYER)(this, clube, person, offer)
}

/// Comparer for `collapse_offer` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_collapse_offer(this: usize, offer: usize, clube: usize, reason: u32, remove_future: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32)>(IDX_COLLAPSE_OFFER)(this, offer, clube, reason, remove_future)
}

/// Comparer for `make_loan_offer` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_make_loan_offer(this: usize, player: usize, clube: usize, end: usize, write_percent_pair: u32, a5: u32, a6: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, u32) -> u32>(IDX_MAKE_LOAN_OFFER)(this, player, clube, end, write_percent_pair, a5, a6)
}

/// Comparer for `process_due_future_transfers` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_process_due_future_transfers(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_PROCESS_DUE_FUTURE_TRANSFERS)(this)
}

/// Comparer for `club_rejects_offer` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_club_rejects_offer(this: usize, offer: usize, clube: usize, reason: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(IDX_CLUB_REJECTS_OFFER)(this, offer, clube, reason)
}

/// Comparer for `detach_offer_players` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_detach_offer_players(this: usize, offer: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_DETACH_OFFER_PLAYERS)(this, offer)
}

/// Comparer for `build_cash_offer_option` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: aloca a oferta e a 0x8e78f0 pode sortear: roda só a original e confere o resultado: devolve 1, a oferta
/// guardada em `saída` tem o valor (+0x1b) e a atração ficou em 50.
pub(crate) unsafe extern "thiscall" fn comparar_build_cash_offer_option(this: usize, clube: usize, player: usize, out: *mut usize, atracao: *mut u8, value: i32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, *mut usize, *mut u8, i32) -> u32>(IDX_BUILD_CASH_OFFER_OPTION)(this, clube, player, out, atracao, value);
    let offer = *out;
    let ok = r == 1 && offer != 0 && core::ptr::read_unaligned((offer + offer::FEE) as *const i32) == value && *atracao == 0x32;
    PLACAR.record(ok, || format!("build_cash_offer_option(jogador {:x}, valor {}) devolveu {}, oferta {:x}", read_usize(player), value, r, offer));
    r
}

/// Comparer for `get_offer_response_delay` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelas duas saídas, o sorteio, [0xae1080] e [0xb67a34] (a caixa da falta de ficha).
pub(crate) unsafe extern "thiscall" fn comparar_get_offer_response_delay(this: usize, offer: usize, days: *mut u8, fatias: *mut u8) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut u8, *mut u8)>(IDX_GET_OFFER_RESPONSE_DELAY);
    let mut reg = vec![(days as usize, 1usize), (fatias as usize, 1), (G_CONTRACT_LOOKUP_RESULT, 4), (0xb67a34, 4)];
    reg.extend(crate::rng::STATE_REGIONS);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side_light(
        &reg,
        || o(this, offer, days, fatias),
        || crate::rng::without_counting(|| get_offer_response_delay(this, offer, days, fatias)),
    );
    PLACAR.record(igual, || format!("get_offer_response_delay({:#x}) {:x?} x {:x?}", offer, mo, mr));
}

/// Comparer for `pay_transfer_instalments` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_pay_transfer_instalments(this: usize) {
    let list = this + 0x872;
    let before = national_teams_list_items(list);
    let acabam = before.iter().filter(|&&e| read_u8(e + 0xc) == 1).count();
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_PAY_TRANSFER_INSTALMENTS)(this);
    let after = national_teams_list_items(list).len();
    PLACAR.record(after + acabam == before.len(), || format!("pay_transfer_instalments: {} prestações, {} acabando, {} depois", before.len(), acabam, after));
}

/// Comparer for `remove_offer` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_remove_offer(this: usize, offer: usize) {
    let slot = read_i32(offer + offer::SLOT_INDEX);
    let ligada = read_i32(offer + offer::NEGOTIATION_INDEX) != -1;
    let (n8, n14, livre) = (read_i32(this + 8), read_i32(this + 0x14), read_i32(this + 0x85e));
    let q = read_usize(this + 0x86e).wrapping_add(read_usize(offer + offer::TARGET_STAFF_ID).wrapping_mul(8)) + 1;
    let offers = read_u8(q);
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_REMOVE_OFFER)(this, offer);
    let ok = read_usize(read_usize(this).wrapping_add((slot as usize).wrapping_mul(4))) == 0
        && read_i32(this + 8) == n8 - 1
        && read_i32(this + 0x14) == n14 - ligada as i32
        && read_i32(this + 0x85e) == livre.min(slot)
        && read_u8(q) == offers.wrapping_sub(1);
    PLACAR.record(ok, || format!("remove_offer: vaga {} contas {}/{} -> {}/{} livre {} -> {} ofertas {} -> {}",
        slot, n8, n14, read_i32(this + 8), read_i32(this + 0x14), livre, read_i32(this + 0x85e), offers, read_u8(q)));
}

/// Comparer for `club_has_offer_for_player` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_club_has_offer_for_player(this: usize, person: usize, clube: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_CLUB_HAS_OFFER_FOR_PLAYER);
    if person == 0 {
        return o(this, person, clube);
    }
    let reg = offer_cursor_region(this, person);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(this, person, clube), || club_has_offer_for_player(this, person, clube));
    PLACAR.record(igual && r == n, || format!("club_has_offer_for_player({}, clube {}) original={} rust={} {:x?} x {:x?}", read_usize(person), read_usize(clube), r, n, mo, mr));
    r
}

/// Comparer for `find_club_offer_for_player` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_find_club_offer_for_player(this: usize, person: usize, clube: usize, all: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32) -> usize>(IDX_FIND_CLUB_OFFER_FOR_PLAYER);
    if person == 0 {
        return o(this, person, clube, all);
    }
    let reg = offer_cursor_region(this, person);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(this, person, clube, all), || find_club_offer_for_player(this, person, clube, all));
    PLACAR.record(igual && r == n, || format!("find_club_offer_for_player({}, clube {}, todas {}) original={:x} rust={:x} {:x?} x {:x?}", read_usize(person), read_usize(clube), all, r, n, mo, mr));
    r
}

/// Comparer for `player_has_pending_offer` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_player_has_pending_offer(this: usize, person: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_PLAYER_HAS_PENDING_OFFER);
    if person == 0 {
        return o(this, person);
    }
    let reg = offer_cursor_region(this, person);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(this, person), || player_has_pending_offer(this, person));
    PLACAR.record(igual && r == n, || format!("player_has_pending_offer({}) original={} rust={} {:x?} x {:x?}", read_usize(person), r, n, mo, mr));
    r
}

/// Comparer for `is_player_under_offer` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_is_player_under_offer(this: usize, person: usize, visao: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_IS_PLAYER_UNDER_OFFER);
    if person == 0 {
        return o(this, person, visao);
    }
    let reg = offer_cursor_and_lookup_regions(this, person);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(this, person, visao), || is_player_under_offer(this, person, visao));
    PLACAR.record(igual && r == n, || format!("is_player_under_offer({}, visao {}) original={} rust={} {:x?} x {:x?}", read_usize(person), visao, r, n, mo, mr));
    r
}

/// Comparer for `player_club_uses_bosman` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "stdcall" fn comparar_player_club_uses_bosman(person: usize) -> u32 {
    let o = original_fn::<unsafe extern "stdcall" fn(usize) -> u32>(IDX_PLAYER_CLUB_USES_BOSMAN);
    if person == 0 {
        return o(person);
    }
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(G_CONTRACT_LOOKUP_RESULT, 4)], || o(person), || player_club_uses_bosman(person));
    PLACAR.record(igual && r == n, || format!("player_club_uses_bosman({}) original={} rust={} {:x?} x {:x?}", read_usize(person), r, n, mo, mr));
    r
}

/// Comparer for `check_board_veto` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a diretoria (0x696b10) mexe na confiança e não pode rodar duas vezes: o plano antes (as três palavras
/// do jogador), a original roda e confere: devolução 0 ou 1; com motivo, 0x24 (as palavras a 90%), 0x17 ou 0x18 quando
/// 1 e intocado quando 0; as palavras intocadas fora do motivo 0x24 (sem motivo, intocadas ou as três a 90%).
pub(crate) unsafe extern "thiscall" fn comparar_check_board_veto(this: usize, offer: usize, buyer: usize, seller: usize, player: usize, decide: usize, reason: *mut u16) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, *mut u16) -> u32>(IDX_CHECK_BOARD_VETO);
    let dados = if player == 0 { 0 } else { read_usize(player + staff::PLAYER) };
    if dados == 0 {
        return o(this, offer, buyer, seller, player, decide, reason);
    }
    let read = || [0xb, 9, 0xd].map(|k| core::ptr::read_unaligned((dados + k) as *const i16));
    let before = read();
    let m0 = if reason.is_null() { 0 } else { *reason };
    let r = o(this, offer, buyer, seller, player, decide, reason);
    let after = read();
    let a_90 = before.map(|v| match_pl_ftol(v as f64 * read_f64_at(0x9689d8)) as i16); // 0x9689d8 = 0.9
    let m = if reason.is_null() { 0 } else { *reason };
    let ok = match (r, reason.is_null()) {
        (0, false) => m == m0 && after == before,
        (0, true) => after == before,
        (1, false) => (m == 0x24 && after == a_90) || ((m == 0x17 || m == 0x18) && after == before),
        (1, true) => after == before || after == a_90,
        _ => false,
    };
    PLACAR.record(ok, || format!("check_board_veto devolveu {}, motivo {:#x} (antes {:#x}), palavras {:?} -> {:?}", r, m, m0, before, after));
    r
}

/// Comparer for `add_past_transfer` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a original acrescenta (pode realocar) e não pode rodar duas vezes: o plano antes (a conta, o registro),
/// a original roda, confere a conta, o registro novo (as duplas e o byte +0x1c; a data é da 0x8cb940) e a devolução.
pub(crate) unsafe extern "thiscall" fn comparar_add_past_transfer(this: usize, record: usize) -> usize {
    let n = read_usize(this + 0x86a);
    let font = core::slice::from_raw_parts(record as *const u8, 0x1d).to_vec();
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_ADD_PAST_TRANSFER)(this, record);
    if read_usize(this + 0x862) == 0 {
        return r; // out of memory: the box has already shown
    }
    let new = read_usize(this + 0x862).wrapping_add(n.wrapping_mul(0x25));
    let gravado = core::slice::from_raw_parts(new as *const u8, 0x1d);
    PLACAR.record(r == new && read_usize(this + 0x86a) == n.wrapping_add(1) && gravado == &font[..], || {
        format!("add_past_transfer: conta {} -> {}, devolveu {:x} (esperava {:x}), registro {:x?} x {:x?}", n, read_usize(this + 0x86a), r, new, font, gravado)
    });
    r
}

/// Comparer for `add_work_permit` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: o plano antes (a conta e o registro esperado), a original roda; confere a conta, a marca 0x20 e que o
/// registro está no vetor (com `ordenar`, em qualquer lugar).
pub(crate) unsafe extern "thiscall" fn comparar_add_work_permit(this: usize, person: usize, nacao: usize, year: u32, not_used: u32, sort: u32) {
    let n = read_usize(this + 0x93e);
    let mut target = [0u8; 0xe];
    target[0..4].copy_from_slice(&(read_usize(person) as u32).to_le_bytes());
    target[8..10].copy_from_slice(&(year as u16).to_le_bytes());
    target[0xa..0xe].copy_from_slice(&(read_usize(nacao) as u32).to_le_bytes());
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32)>(IDX_ADD_WORK_PERMIT)(this, person, nacao, year, not_used, sort);
    if read_usize(this + 0x93a) == 0 {
        return; // out of memory: the box has already shown
    }
    let count = read_usize(this + 0x93e);
    let mark = *((read_usize(this + 0x86e).wrapping_add(read_usize(person).wrapping_mul(8))) as *const u8);
    let ok = count == n.wrapping_add(1) && mark & 0x20 != 0 && vector_contains_record(read_usize(this + 0x93a), count, 0xe, &target);
    PLACAR.record(ok, || format!("add_work_permit(pessoa {}, nação {}, ano {}): conta {} -> {}, marca {:#x}", read_usize(person), read_usize(nacao), year as u16, n, count, mark));
}

/// Comparer for `add_first_team_appearance_money` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: o plano antes (a conta e os 13 bytes), a original roda; confere a conta e que o registro está no vetor
/// (reordenado).
pub(crate) unsafe extern "thiscall" fn comparar_add_first_team_appearance_money(this: usize, record: usize) {
    let n = read_usize(this + 0x894);
    let target = core::slice::from_raw_parts(record as *const u8, 0xd).to_vec();
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_ADD_FIRST_TEAM_APPEARANCE_MONEY)(this, record);
    if read_usize(this + 0x88c) == 0 {
        return; // out of memory: the box has already shown
    }
    let count = read_usize(this + 0x894);
    let ok = count == n.wrapping_add(1) && vector_contains_record(read_usize(this + 0x88c), count, 0xd, &target);
    PLACAR.record(ok, || format!("add_first_team_appearance_money: conta {} -> {}, registro {:x?}", n, count, target));
}

/// Comparer for `get_fixture_rules_scope` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelas três saídas (a 0x669470 e a 0x5c6710 só leem).
pub(crate) unsafe extern "thiscall" fn comparar_get_fixture_rules_scope(this: usize, partida: usize, kind: *mut u8, nacao: *mut usize, continent: *mut usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut u8, *mut usize, *mut usize)>(IDX_GET_FIXTURE_RULES_SCOPE);
    let regions = [(kind as usize, 1), (nacao as usize, 4), (continent as usize, 4)];
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side_light(&regions, || o(this, partida, kind, nacao, continent), || get_fixture_rules_scope(this, partida, kind, nacao, continent));
    PLACAR.record(igual, || format!("get_fixture_rules_scope({:x}) {:x?} x {:x?}", partida, mo, mr));
}

/// Comparer for `get_ineligibility_code` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor. Jogador com a marca de ausente (0x418f60 com 1, que só lê): só a original (com 0 ela tira
/// a marca de quem está sem clube e a segunda chamada veria outro estado).
pub(crate) unsafe extern "thiscall" fn comparar_get_ineligibility_code(this: usize, player: usize, partida: usize) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u8>(IDX_GET_INELIGIBILITY_CODE);
    let ausente: unsafe extern "thiscall" fn(usize, usize, u32) -> u8 = core::mem::transmute(FN_AWOL_IS_PLAYER_AWOL);
    if ausente(read_usize(G_AWOL), player, 1) != 0 {
        return o(this, player, partida);
    }
    let r = o(this, player, partida);
    let n = get_ineligibility_code(this, player, partida);
    PLACAR.record(r == n, || format!("get_ineligibility_code({:x}, {:x}) original={} rust={}", player, partida, r, n));
    r
}

/// Comparer for `write_deadline_news_text` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: o método das regras pode escrever o texto e sortear: só a original; a escolha das regras da cópia é a
/// mesma conta (a prova é a carreira e o modo Rust).
pub(crate) unsafe extern "thiscall" fn comparar_write_deadline_news_text(this: usize, noticia: usize, full: u32, text: usize) -> u32 {
    crate::comparison::branch("write_deadline_news_text");
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize) -> u32>(IDX_WRITE_DEADLINE_NEWS_TEXT)(this, noticia, full, text)
}

/// Comparer for `can_end_loan` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: quando as regras da nação respondem, só a original (o método é delas); senão os dois devolvem 1.
pub(crate) unsafe extern "thiscall" fn comparar_can_end_loan(this: usize, clube: usize, player: usize, a: u32, b: u32, c: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) -> u32>(IDX_CAN_END_LOAN)(this, clube, player, a, b, c);
    if loan_transfer_rules(this, clube, player).is_some() {
        crate::comparison::branch("can_end_loan pelas regras");
        return r;
    }
    PLACAR.record(r == 1, || format!("can_end_loan({:x}, {:x}) original={} rust=1", clube, player, r));
    r
}

/// Comparer for `transfer_manager_offer_needs_work_permit` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pela ficha [0xae1080] e pelo cursor (+0xa) da lista de troca.
pub(crate) unsafe extern "thiscall" fn comparar_transfer_manager_offer_needs_work_permit(this: usize, offer: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_TRANSFER_MANAGER_OFFER_NEEDS_WORK_PERMIT);
    let list = read_u32(offer + 4) as usize;
    let mut reg = vec![(G_CONTRACT_LOOKUP_RESULT, 4)];
    if list != 0 {
        reg.push((list + 0xa, 4));
    }
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(this, offer), || transfer_manager_offer_needs_work_permit(this, offer));
    PLACAR.record(igual && r == n, || format!("transfer_manager_offer_needs_work_permit({:x}) original={} rust={} {:x?} x {:x?}", offer, r, n, mo, mr));
    r
}

/// Comparer for `update_work_permits` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelo vetor, pela contagem e, de cada permissão, pela marca da pessoa e pela segunda
/// nacionalidade (+0x1e). Permissões com pessoa negativa não entram nas regiões (o original leria fora do vetor).
pub(crate) unsafe extern "thiscall" fn comparar_update_work_permits(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_UPDATE_WORK_PERMITS);
    let n = (*((this + 0x93e) as *const i32)).max(0) as usize;
    let array = read_u32(this + 0x93a) as usize;
    let mut reg = vec![(this + 0x93a, 8), (array, n * 0xe)];
    for k in 0..n {
        let id = read_u32(array + k * 0xe);
        if (id as i32) < 0 {
            continue;
        }
        let person = staff_record_by_id(id);
        reg.push(((read_u32(this + 0x86e) as usize).wrapping_add((read_u32(person) as usize).wrapping_mul(8)), 1));
        reg.push((person + staff::SECOND_NATION, 4));
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this), || update_work_permits(this));
    PLACAR.record(igual, || format!("update_work_permits {:x?} x {:x?}", mo, mr));
}

/// Comparer for `past_transfer_copy_construct` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_past_transfer_copy_construct(this: usize, source: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_PAST_TRANSFER_COPY_CONSTRUCT);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(this, 0x25)], || o(this, source), || past_transfer_copy_construct(this, source));
    PLACAR.record(igual && r == n, || format!("past_transfer_copy_construct {:x?} x {:x?}", mo, mr));
    r
}

/// Comparer for `future_transfer_assign` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_future_transfer_assign(this: usize, source: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_FUTURE_TRANSFER_ASSIGN);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(this, 0xc)], || o(this, source), || future_transfer_assign(this, source));
    PLACAR.record(igual && r == n, || format!("future_transfer_assign {:x?} x {:x?}", mo, mr));
    r
}

/// Comparer for `can_sign_player` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_can_sign_player(manager: usize, clube: usize, a2: u32, player: usize, a4: u32, a5: u32, out: *mut u16) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, u32, *mut u16) -> u32>(IDX_CAN_SIGN_PLAYER);
    let mut regions = vec![];
    if !out.is_null() {
        regions.push((out as usize, 2));
    }
    if player != 0 {
        regions.push((player + staff::NATION, 4));
    }
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&regions, || o(manager, clube, a2, player, a4, a5, out),
        || can_sign_player(manager, clube, a2, player, a4, a5, out));
    PLACAR.record(igual && r == n, || format!("can_sign_player(a5 {}) original={} rust={} {:x?} x {:x?}", a5, r, n, mo, mr));
    r
}

/// Comparer for `get_exchange_offers_for_player` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_get_exchange_offers_for_player(manager: usize, player: usize, out: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_GET_EXCHANGE_OFFERS_FOR_PLAYER);
    if out != 0 {
        // simplification: with an output list the original appends (allocates); original only
        return o(manager, player, out);
    }
    let mut regions = vec![(G_CONTRACT_LOOKUP_RESULT, 4usize)];
    if player != 0 && *(player as *const i32) < *(G_STAFF_COUNT as *const i32) {
        let stored = *(G_CONTRACT_LOOKUP_RESULT as *const usize);
        let p = transfer_manager_contract_club(player);
        *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = stored;
        if p != 0 {
            // the cursors of the swap lists of the club’s offers (those the routine may walk through)
            let clube = *(p as *const u32);
            for i in 0..(*((manager + 4) as *const i32)).max(0) as usize {
                let offer = *((*(manager as *const usize) + 4 * i) as *const usize);
                if offer != 0 && *((offer + 8) as *const u32) == clube && *((offer + 4) as *const usize) != 0 {
                    regions.push((*((offer + 4) as *const usize) + 0xa, 4));
                }
            }
        }
    }
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&regions, || o(manager, player, out),
        || get_exchange_offers_for_player(manager, player, out));
    PLACAR.record(igual && r == n, || format!("get_exchange_offers_for_player original={} rust={} {:x?} x {:x?}", r, n, mo, mr));
    r
}

/// Comparer for `transfer_manager_close_offer_contract_offers` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "stdcall" fn comparar_transfer_manager_close_offer_contract_offers(offer: usize) {
    let o = original_fn::<unsafe extern "stdcall" fn(usize)>(IDX_TRANSFER_MANAGER_CLOSE_OFFER_CONTRACT_OFFERS);
    // the proposals that can close: those of the target’s id and of the swap ids (the club does not filter here)
    let mut ids = vec![*((offer + offer::TARGET_STAFF_ID) as *const u32)];
    let l = *((offer + 4) as *const usize);
    if l != 0 {
        ids.extend(llist_data_values(l).iter().map(|&e| *(e as *const u32)));
    }
    let propostas = *((*(G_CONTRACT_MANAGER as *const usize) + 0x34) as *const usize);
    let mut regions = vec![(G_CONTRACT_LOOKUP_RESULT, 4usize)];
    if l != 0 {
        regions.push((l + 0xa, 4));
    }
    if propostas != 0 {
        regions.push((propostas + 0xa, 4));
        for r in llist_data_values(propostas) {
            if ids.contains(&*(r as *const u32)) {
                regions.push((r + 0x50, 1));
            }
        }
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&regions, || o(offer), || transfer_manager_close_offer_contract_offers(offer));
    PLACAR.record(igual, || format!("transfer_manager_close_offer_contract_offers(id {}) {:x?} x {:x?}", ids[0], mo, mr));
}

/// Comparer for `queue_free_transfer` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_queue_free_transfer(manager: usize, person: usize, reason: u8, compensacao: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u8, u32)>(IDX_QUEUE_FREE_TRANSFER);
    // simplification: the queue allocates a node (does not run twice): the copy only builds the record (0x4cf6d0 is
    // pure query), the original queues it and the new tail node is checked
    let mut record = None;
    if !queue_free_transfer_with(manager, person, reason, compensacao, &mut |_, r| record = Some(*r)) {
        return o(manager, person, reason, compensacao);
    }
    let l = manager + 0x92c;
    let before = core::ptr::read_unaligned(l as *const u16);
    o(manager, person, reason, compensacao);
    let after = core::ptr::read_unaligned(l as *const u16);
    let cauda = core::ptr::read_unaligned((l + 6) as *const usize);
    let new = if cauda == 0 { None } else { Some(*(*(cauda as *const usize) as *const [u8; 9])) };
    PLACAR.record(after == before.wrapping_add(1) && new == record,
        || format!("queue_free_transfer {} -> {} original={:x?} rust={:x?}", before, after, new, record));
}

/// Comparer for `get_exchange_negotiations_for_player` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_get_exchange_negotiations_for_player(manager: usize, person: usize, out: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> i32>(IDX_GET_EXCHANGE_NEGOTIATIONS_FOR_PLAYER);
    if out != 0 {
        // simplification: with an output list (only screen 0x47ec79) the original appends (allocates); original only
        return o(manager, person, out);
    }
    let mut regions = vec![(G_CONTRACT_LOOKUP_RESULT, 4usize)];
    if person != 0 && *(person as *const i32) < *(G_STAFF_COUNT as *const i32) {
        let stored = *(G_CONTRACT_LOOKUP_RESULT as *const usize);
        let p = transfer_manager_contract_club(person);
        *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = stored;
        if p != 0 {
            // the cursors of the swap lists of the club’s negotiations (the ones the routine may traverse)
            let clube = *(p as *const u32);
            let negociacoes = *((manager + staff::COMMON_NAME) as *const usize);
            for i in 0..(*((manager + staff::DATE_OF_BIRTH) as *const i32)).max(0) as usize {
                let neg = *((negociacoes + i * 4) as *const usize);
                if neg != 0 && *((neg + 8) as *const u32) == clube && *((neg + 4) as *const usize) != 0 {
                    regions.push((*((neg + 4) as *const usize) + 0xa, 4));
                }
            }
        }
    }
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&regions, || o(manager, person, out),
        || get_exchange_negotiations_for_player(manager, person, out));
    PLACAR.record(igual && r == n, || format!("get_exchange_negotiations_for_player original={} rust={} {:x?} x {:x?}", r, n, mo, mr));
    r
}

/// Comparer for `get_offers_for_player` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_get_offers_for_player(this: usize, person: *const i32, list: usize, all: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *const i32, usize, u32) -> u32>(IDX_GET_OFFERS_FOR_PLAYER);
    // simplification: with `lista` the original allocates nodes (0x68d0d0) and cannot run twice; original only
    if list != 0 || person.is_null() {
        return o(this, person, list, all);
    }
    // the original only touches the list cursor of the offer found
    let e = offer_for_player(this, *person);
    let l = if e == 0 { 0 } else { *(e as *const usize) };
    let reg = if l == 0 { vec![] } else { vec![(l, 0xe)] };
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, person, list, all), || get_offers_for_player(this, person, list, all));
    PLACAR.record(igual && r == n, || format!("get_offers_for_player({}, todas {}) original={} rust={} {:x?} x {:x?}", *person, all, r, n, mo, mr));
    r
}

/// Comparer for `lista_de_pagamentos` (comparison mode: see the header of motor.rs).
pub(crate) unsafe fn comparar_lista_de_pagamentos(name: &str, k: usize, rust: unsafe extern "thiscall" fn(usize, *const i32), manager: usize, id: *const i32, list: usize, how_many: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *const i32)>(k);
    let reg = payment_list_regions(manager, id, list, how_many);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(manager, id), || rust(manager, id));
    PLACAR.record(igual, || format!("{}({}) {:x?} x {:x?}", name, *id, mo, mr));
}

/// Comparer for `remove_next_sale_percentage` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_remove_next_sale_percentage(manager: usize, id: *const i32) {
    comparar_lista_de_pagamentos("remove_next_sale_percentage", 451, remove_next_sale_percentage, manager, id, 0x880, 0x888);
}

/// Comparer for `remove_first_team_appearance_money` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_remove_first_team_appearance_money(manager: usize, id: *const i32) {
    comparar_lista_de_pagamentos("remove_first_team_appearance_money", 452, remove_first_team_appearance_money, manager, id, 0x88c, 0x894);
}

/// Comparer for `remove_international_team_appearance_money` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_remove_international_team_appearance_money(manager: usize, id: *const i32) {
    comparar_lista_de_pagamentos("remove_international_team_appearance_money", 453, remove_international_team_appearance_money, manager, id, 0x898, 0x8a0);
}

/// Comparer for `get_agreed_contract_club` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_get_agreed_contract_club(manager: usize, id: *const i32, clube: *mut usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *const i32, *mut usize) -> u32>(IDX_GET_AGREED_CONTRACT_CLUB);
    let mut reg = vec![(staff_flags_entry(manager, id), 1)];
    if !clube.is_null() {
        reg.push((clube as usize, 4));
    }
    let (ro, rr, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(manager, id, clube), || get_agreed_contract_club(manager, id, clube));
    PLACAR.record(igual && ro == rr, || format!("get_agreed_contract_club({}) original={} rust={} {:x?} x {:x?}", *id, ro, rr, mo, mr));
    ro
}

/// Comparer for `record_international_appearance` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_record_international_appearance(manager: usize, id: *const i32, number: i16) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *const i32, i16)>(IDX_RECORD_INTERNATIONAL_APPEARANCE);
    let reg = [(staff_flags_entry(manager, id) + 3, 4)];
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(manager, id, number), || record_international_appearance(manager, id, number));
    PLACAR.record(igual, || format!("record_international_appearance({}, {}) {:x?} x {:x?}", *id, number, mo, mr));
}

/// Comparer for `set_agreed_contract_flag` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_set_agreed_contract_flag(manager: usize, id: *const i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *const i32)>(IDX_SET_AGREED_CONTRACT_FLAG);
    let a = staff_flags_ptr(manager, id) as usize;
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&[(a, 1)], || o(manager, id), || set_agreed_contract_flag(manager, id));
    PLACAR.record(igual, || format!("set_agreed_contract_flag({}) {:x?} x {:x?}", *id, mo, mr));
}

/// Comparer for `clear_agreed_contract_flag` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_clear_agreed_contract_flag(manager: usize, id: *const i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *const i32)>(IDX_CLEAR_AGREED_CONTRACT_FLAG);
    let a = staff_flags_ptr(manager, id) as usize;
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&[(a, 1)], || o(manager, id), || clear_agreed_contract_flag(manager, id));
    PLACAR.record(igual, || format!("clear_agreed_contract_flag({}) {:x?} x {:x?}", *id, mo, mr));
}

/// Comparer for `offer_slot` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_offer_slot(table: usize, k: i32) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, i32) -> usize>(IDX_OFFER_SLOT)(table, k);
    let n = offer_slot(table, k);
    PLACAR.record(r == n, || format!("offer_slot({}) original={:#x} rust={:#x}", k, r, n));
    r
}

/// Comparer for `get_selection_limit` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelos 6 bytes da saída (a 0x8cf6b0 só escreve na tabela local).
pub(crate) unsafe extern "thiscall" fn comparar_get_selection_limit(this: usize, partida: usize, out: *mut u8) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut u8)>(IDX_GET_SELECTION_LIMIT);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side_light(&[(out as usize, 6)], || o(this, partida, out), || get_selection_limit(this, partida, out));
    PLACAR.record(igual, || format!("get_selection_limit({:x}) {:x?} x {:x?}", partida, mo, mr));
}

/// Comparer for `find_work_permit` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor (a busca só lê).
pub(crate) unsafe extern "thiscall" fn comparar_find_work_permit(this: usize, person: *const i32) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, *const i32) -> usize>(IDX_FIND_WORK_PERMIT)(this, person);
    let n = find_work_permit(this, person);
    PLACAR.record(r == n, || format!("find_work_permit({:x}) original={:#x} rust={:#x}", person as usize, r, n));
    r
}

/// Comparer for `fill_vacated_staff_job` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a escolha da cópia antes; sem ninguém, só a original. Com a pessoa, original e cópia lado a lado no
/// clube (0x245 bytes) e na pessoa (0x6e), que é tudo o que a 0x53bd10 grava: ela pode recusar (casa ocupada, faixa
/// dos humanos; com mais ligas acontece) ou gravar na seleção (+0x24), e a pessoa não fica em +0x39.
pub(crate) unsafe extern "stdcall" fn comparar_fill_vacated_staff_job(clube: usize, job: u32) {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, u32)>(IDX_FILL_VACATED_STAFF_JOB);
    let escolhida = if clube != 0 { job_sought_by_slot(job).and_then(|c| first_unemployed_staff(clube, c as u8)) } else { None };
    let Some(p) = escolhida else {
        crate::comparison::branch("fill_vacated_staff_job sem ninguem");
        return o(clube, job);
    };
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&[(clube, 0x245), (p, 0x6e)], || o(clube, job),
        || fill_vacated_staff_job(clube, job));
    if read_usize(p + staff::CLUB_JOB) != clube {
        crate::comparison::branch("fill_vacated_staff_job sem entrar no clube");
    }
    PLACAR.record(igual, || format!("fill_vacated_staff_job({:x}, {}) pessoa {:x} {:x?} x {:x?}", clube, job as u8, p, mo, mr));
}

/// Comparer for `person_in_transfer_list` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_person_in_transfer_list(t: usize, p: *const i32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *const i32) -> u32>(IDX_PERSON_IN_TRANSFER_LIST);
    let reg = [(t + 0x92c + 0xa, 4)];
    let (r, v, igual, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(t, p), || person_in_transfer_list(t, p));
    PLACAR.record(igual && r == v, || format!("person_in_transfer_list({:#x}, {:p}) original={} {:x?} rust={} {:x?}", t, p, r, mo, v, mr));
    r
}

/// Comparer for `handle_player_rejected_terms` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_handle_player_rejected_terms(this: usize, person: usize, indice: i32, silencioso: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32)>(IDX_HANDLE_PLAYER_REJECTED_TERMS)(this, person, indice, silencioso)
}

/// Comparer for `hold_offers_at_window_close` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_hold_offers_at_window_close(this: usize, nacao: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_HOLD_OFFERS_AT_WINDOW_CLOSE)(this, nacao)
}

/// Comparer for `schedule_offer` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a original roda (a 0x68d0d0 aloca o nó) e o resultado é conferido com o plano feito antes: a data
/// (+0x12) e a fatia (+0x1a) da oferta, a conta do balde mais 1 e a data base (+0x84d, 8 bytes) igual a hoje quando
/// não havia.
pub(crate) unsafe extern "thiscall" fn comparar_schedule_offer(this: usize, offer: usize, days: u32, fatias: u32) -> u32 {
    let sem_base = read_i16(this + 0x84f) == 0;
    let (data, fatia, balde) = vacancy_schedule_plan(this, days as u8, fatias as u8);
    let count = read_i16(balde);
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32>(IDX_SCHEDULE_OFFER)(this, offer, days, fatias);
    let gravada = core::slice::from_raw_parts((offer + offer::DUE_DATE) as *const u8, 8);
    let base_ok = !sem_base || core::slice::from_raw_parts((this + 0x84d) as *const u8, 8) == core::slice::from_raw_parts(G_CURRENT_DATE as *const u8, 8);
    let ok = gravada == &data[..] && *((offer + offer::DUE_SLICE) as *const i8) == fatia && read_i16(balde) == count.wrapping_add(1) && base_ok;
    PLACAR.record(ok, || format!("schedule_offer(dias {}, fatias {}): data {:x?} (plano {:x?}), fatia {} (plano {}), balde {:x} conta {} -> {}, base {}",
        days as u8 as i8, fatias as u8 as i8, gravada, data, *((offer + offer::DUE_SLICE) as *const i8), fatia, balde, count, read_i16(balde), base_ok));
    r
}

/// Comparer for `find_highest_offer_for_player` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor (a 0x8e7a70 só calcula; as duas voltam o cursor da lista ao começo).
pub(crate) unsafe extern "thiscall" fn comparar_find_highest_offer_for_player(this: usize, player: *const u32, clube: *const u32) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, *const u32, *const u32) -> usize>(IDX_FIND_HIGHEST_OFFER_FOR_PLAYER)(this, player, clube);
    let n = find_highest_offer_for_player(this, player, clube);
    PLACAR.record(r == n, || format!("find_highest_offer_for_player({}, {}) original={:#x} rust={:#x}", *player, *clube, r, n));
    r
}

/// Comparer for `find_international_appearance_money` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x8d0f00: lado a lado na marca da pessoa e na devolução.
pub(crate) unsafe extern "thiscall" fn comparar_find_international_appearance_money(this: usize, p: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_FIND_INTERNATIONAL_APPEARANCE_MONEY);
    let m = read_usize(this + 0x86e).wrapping_add(read_usize(p).wrapping_mul(8));
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&[(m, 1)], || o(this, p), || find_international_appearance_money(this, p));
    PLACAR.record(igual && r == n, || format!("find_international_appearance_money(pessoa {:x}) original={:x} rust={:x} {:x?} x {:x?}", p, r, n, mo, mr));
    r
}

/// Comparer for `pay_appearance_money` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: dá notícia e mexe nas finanças: roda só a original e confere a devolução com o plano (contrato, clube,
/// registro do prêmio e jogos, sem efeitos).
pub(crate) unsafe extern "thiscall" fn comparar_pay_appearance_money(this: usize, player: usize, games: i32, kind: u32, arg: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32, u32) -> u32>(IDX_PAY_APPEARANCE_MONEY);
    let c = get_contract(player);
    let expected = if c == 0 || friendly_club_by_id(read_usize(c + 4)) == 0 {
        0
    } else {
        let r = appearance_bonus_record(this, player, kind);
        (r != 0 && read_u8(r + 8) as i8 as i32 == games && games > 0) as u32
    };
    let r = o(this, player, games, kind, arg);
    PLACAR.record(r == expected, || format!("pay_appearance_money(jogador {:x}, jogos {}, tipo {}) devolveu {}, esperava {}", read_usize(player), games, kind as u8, r, expected));
    r
}

/// Comparer for `transfer_manager_get_loan_period` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelas quatro saídas, o sorteio e [0xae1080].
pub(crate) unsafe extern "thiscall" fn comparar_transfer_manager_get_loan_period(this: usize, clube: usize, a2: u32, player: usize, ini_month: *mut i8, ini_year: *mut u16, month: *mut i8, year: *mut u16) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize, *mut i8, *mut u16, *mut i8, *mut u16)>(IDX_TRANSFER_MANAGER_GET_LOAN_PERIOD);
    let mut reg = vec![(ini_month as usize, 1usize), (ini_year as usize, 2), (month as usize, 1), (year as usize, 2), (G_CONTRACT_LOOKUP_RESULT, 4)];
    reg.extend(crate::rng::STATE_REGIONS);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side_light(
        &reg,
        || o(this, clube, a2, player, ini_month, ini_year, month, year),
        || crate::rng::without_counting(|| transfer_manager_get_loan_period(this, clube, a2, player, ini_month, ini_year, month, year)),
    );
    PLACAR.record(igual, || format!("transfer_manager_get_loan_period({:#x}, {:#x}) {:x?} x {:x?}", clube, player, mo, mr));
}

/// Comparer for `update_transfer_window_news` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x8c6bc0: só a original (ela só grava nas locais; os avisos das regras alocam notícias e as ofertas
/// em espera mudam de status e são reagendadas).
pub(crate) unsafe extern "thiscall" fn comparar_update_transfer_window_news(this: usize, today: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_UPDATE_TRANSFER_WINDOW_NEWS)(this, today) // simplification: nothing of its own to check
}

/// Comparer for `note_terms_offered_to_main_player` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_note_terms_offered_to_main_player(manager: usize, k: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32)>(IDX_NOTE_TERMS_OFFERED_TO_MAIN_PLAYER);
    if k < 0 || k >= *((manager + 4) as *const i32) {
        return o(manager, k);
    }
    let offer = *((*(manager as *const usize) as isize + k as isize * 4) as *const usize);
    if offer == 0 {
        return o(manager, k);
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&[(offer + offer::STATUS, 1)], || o(manager, k), || note_terms_offered_to_main_player(manager, k));
    PLACAR.record(igual, || format!("note_terms_offered_to_main_player({}) {:x?} x {:x?}", k, mo, mr));
}
