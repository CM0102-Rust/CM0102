use super::*;

// The comparers of the routines in ../transfer_manager.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../transfer_manager.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `add_transfer_offer` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x8be300: só a original (aloca a oferta e as listas, tira a oferta antiga, agenda, entrega a
/// notícia). Confere o plano feito antes: a vaga (`find_offer_slot_and_rival`, com o vetor que cresce quando está cheio) e o número
/// de série gravados na oferta de entrada e a série que sobe.
pub(crate) unsafe extern "thiscall" fn comparar_add_transfer_offer(this: usize, oferta: usize, termos: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_ADD_TRANSFER_OFFER);
    let (alvo, comprador) = (read_u32_as_usize(oferta + 0xc), read_u32_as_usize(oferta + 8));
    let primeira = read_u8(read_u32_as_usize(this + 0x86e).wrapping_add(alvo.wrapping_mul(8)) + 1) == 0;
    let (lidas, conta) = (read_u32_as_usize(this + 4) as i32, read_u32_as_usize(this + 8) as i32);
    let cap = if conta == lidas { lidas.wrapping_add(0x32) } else { lidas };
    let (vaga, _) = find_offer_slot_and_rival(read_u32_as_usize(this), lidas, cap, conta, alvo, comprador, read_u32_as_usize(this + 0x85e) as i32, primeira);
    let serie = read_u32_as_usize(this + 0x85a);
    o(this, oferta, termos); // simplification: só a original
    if vaga == -1 {
        return;
    }
    let (v, s, s2) = (read_u32_as_usize(oferta + 0x20) as i32, read_u32_as_usize(oferta + 0x24), read_u32_as_usize(this + 0x85a));
    // the serial goes up at least once: a nested offer made inside the call (news, scheduling) raises it again
    // (8-league audit of 07/11/2026, ENG SCO IRL ITA ESP, 2002-050: 4625 against 4624)
    // [pt] a série sobe pelo menos uma vez: uma oferta aninhada feita dentro da chamada (notícia, agenda) a sobe de novo
    PLACAR.registrar(v == vaga && s == serie && s2 > serie, || format!(
        "add_transfer_offer(alvo {}, comprador {}): vaga original {} plano {}, série {} plano {}, próxima {} plano {}",
        alvo, comprador, v, vaga, s, serie, s2, serie.wrapping_add(1)));
}

/// Comparer for `accept_loan_offer` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x8c0660: só a original (entrega notícias, agenda a oferta, mexe nas listas de observação).
pub(crate) unsafe extern "thiscall" fn comparar_accept_loan_offer(this: usize, vaga: *mut usize, marcar: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, *mut usize, u32)>(IDX_ACCEPT_LOAN_OFFER)(this, vaga, marcar) // simplification: só a original
}

/// Comparer for `handle_player_accepted_terms` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x8c2180: só a original (entrega notícias, conclui ou recusa o empréstimo, assina contrato, registra a
/// transferência, mexe nas listas).
pub(crate) unsafe extern "thiscall" fn comparar_handle_player_accepted_terms(this: usize, pessoa: usize, vaga: i32, contrato: usize, tipo: u32, noticia: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, usize, u32, u32)>(IDX_HANDLE_PLAYER_ACCEPTED_TERMS)(this, pessoa, vaga, contrato, tipo, noticia) // simplification: só a original
}

/// Comparer for `ai_respond_to_bid` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x8c8090: só a original (aceita, recusa ou contrapropõe, reagenda, mexe nas listas de observação e
/// aloca e solta as opções).
pub(crate) unsafe extern "thiscall" fn comparar_ai_respond_to_bid(this: usize, vaga: usize, ensaio: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_AI_RESPOND_TO_BID)(this, vaga, ensaio) // simplification: só a original
}

/// Comparer for `add_loan_negotiation` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x8bee50: só a original (a lista de contratos cresce, a negociação nova é alocada).
pub(crate) unsafe extern "thiscall" fn comparar_add_loan_negotiation(this: usize, o: usize, a2: u32, a3: u32, a4: u32, a5: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) -> u32>(IDX_ADD_LOAN_NEGOTIATION)(this, o, a2, a3, a4, a5) // simplification: só a original
}

/// Comparer for `make_exchange_option` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x8c8860: lado a lado a partir do mesmo estado de [0xae1080], *saida, *atracao e do sorteador (a
/// 0x8e78f0 sorteia); a cópia roda sem contar sorteios. Confere a devolução, a atração, [0xae1080] e a foto da oferta
/// nova de cada lado; a da cópia é destruída (0x8e5710) e liberada (0x944c94) e fica tudo como a original deixou.
pub(crate) unsafe extern "thiscall" fn comparar_make_exchange_option(this: usize, comprador: usize, alvo: usize, oferta: usize, saida: *mut usize, atracao: *mut i8, valor: i32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut usize, *mut i8, i32) -> u32>(IDX_MAKE_EXCHANGE_OPTION);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_TRANSFER_OFFER_DESTRUCTOR_TRANSFER_OFFER);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_OPERATOR_DELETE);
    let est = crate::aleatorio::REGIOES_DO_ESTADO;
    let regioes = [(G_CONTRACT_LOOKUP_RESULT, 4usize), (saida as usize, 4), (atracao as usize, 1), est[0], est[1]];
    let antes = crate::comparacao::Memoria::ler(&regioes);
    let r = o(this, comprador, alvo, oferta, saida, atracao, valor);
    let depois = crate::comparacao::Memoria::ler(&regioes);
    let (ao, fo) = (*atracao, if r != 0 && *saida != 0 { option_offer_snapshot(*saida) } else { Vec::new() });
    let e_o = read_u32_as_usize(G_CONTRACT_LOOKUP_RESULT);
    antes.escrever();
    let n = crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| make_exchange_option(this, comprador, alvo, oferta, saida, atracao, valor)));
    let (an, fn_) = (*atracao, if n != 0 && *saida != 0 { option_offer_snapshot(*saida) } else { Vec::new() });
    let e_n = read_u32_as_usize(G_CONTRACT_LOOKUP_RESULT);
    if n != 0 && *saida != 0 {
        let s = *saida;
        destruir(s);
        liberar(s);
    }
    depois.escrever();
    PLACAR.registrar(r == n && ao == an && e_o == e_n && fo == fn_, || format!(
        "make_exchange_option(comprador {:#x}, alvo {:#x}, valor {}): original {} rust {}, atração {} x {}, [G_CONTRACT_LOOKUP_RESULT] {:#x} x {:#x}, oferta {:x?} x {:x?}",
        comprador, alvo, valor, r, n, ao, an, e_o, e_n, fo, fn_));
    r
}

/// Comparer for `accept_transfer_bid` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x8c0080: só a original (muda o status, reagenda, solta a troca, aloca notícias e chama a resposta da
/// IA).
pub(crate) unsafe extern "thiscall" fn comparar_accept_transfer_bid(this: usize, vaga: usize, marcar: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_ACCEPT_TRANSFER_BID)(this, vaga, marcar) // simplification: só a original
}

/// Comparer for `post_offer_made_news` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x8d3090: só a original (a notícia vai aos técnicos e não tem outro efeito).
pub(crate) unsafe extern "thiscall" fn comparar_post_offer_made_news(this: usize, o: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_POST_OFFER_MADE_NEWS)(this, o) // simplification: só a notícia
}

/// Comparer for `transfer_manager_can_loan_player` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x8d0040: lado a lado, com [0xae1080], o texto em [0xde1f64] e a mensagem (0x100 bytes), e o byte do
/// registro de 8 bytes do jogador em [[0xde1f54]+0x86e] (o único que ela pode apagar); a devolução também. As regras,
/// o período e as datas não sorteiam nem gravam fora das locais.
pub(crate) unsafe extern "thiscall" fn comparar_transfer_manager_can_loan_player(this: usize, clube: usize, jogador: usize, msg: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32>(IDX_TRANSFER_MANAGER_CAN_LOAN_PLAYER);
    let mut regioes = [(G_CONTRACT_LOOKUP_RESULT, 4usize), (0xde1f64, 0x100), (0, 0), (0, 0)];
    let mut k = 2;
    if msg != 0 {
        regioes[k] = (msg, 0x100);
        k += 1;
    }
    let gm = read_u32_as_usize(G_TRANSFER_MANAGER);
    if gm != 0 && read_u32_as_usize(gm + 0x86e) != 0 {
        regioes[k] = (read_u32_as_usize(gm + 0x86e).wrapping_add(read_u32_as_usize(jogador).wrapping_mul(8)), 1);
        k += 1;
    }
    let (r, n, igual, _, _) = crate::comparacao::lado_a_lado(&regioes[..k], || o(this, clube, jogador, msg),
        || crate::aleatorio::sem_contar(|| transfer_manager_can_loan_player(this, clube, jogador, msg)));
    PLACAR.registrar(igual && r == n, || format!("transfer_manager_can_loan_player(clube {:#x}, jogador {:#x}): original {} rust {}, memória {}",
        clube, jogador, r, n, if igual { "igual" } else { "diferente" }));
    r
}

/// Comparer for `remove_exchange_player` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x8c41c0: só a original (os nós da troca são liberados, a notícia vai aos técnicos e a oferta segue
/// pela 0x8c2f50, que pode fechar a transferência e tirar outros jogadores das trocas, ou ser respondida pelo vendedor
/// do computador, 0x8c8090, que pode mudar a oferta da vaga).
pub(crate) unsafe extern "thiscall" fn comparar_remove_exchange_player(this: usize, pessoa: usize, oferta: usize, so_esta: u32, motivo: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32)>(IDX_REMOVE_EXCHANGE_PLAYER)(this, pessoa, oferta, so_esta, motivo) // simplification: o efeito segue por outras rotinas
}

/// Comparer for `check_offer_work_permits` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x8c7690: só a original (as notícias alocam, a oferta é reagendada e segue pela 0x8c2f50). O plano,
/// feito antes pelas consultas sem efeito (0x8c7560, 0x4cc9e0, 0x8c7950 sem sorteio): o alvo recusado devolve 0 com a
/// oferta no status 0x10; senão 1.
pub(crate) unsafe extern "thiscall" fn comparar_check_offer_work_permits(this: usize, oferta: usize, quieto: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_CHECK_OFFER_WORK_PERMITS);
    let (precisa, proposta, concedida) = check_work_permit(this, transfer_manager_staff_by_id(read_u32_as_usize(oferta + 0xc)), transfer_manager_club_by_id(read_u32_as_usize(oferta + 8)));
    let recusada = precisa && proposta != 0 && !concedida;
    let r = o(this, oferta, quieto);
    let ok = if recusada { r == 0 && read_u8(oferta + 0x2c) == 0x10 } else { r == 1 };
    PLACAR.registrar(ok, || format!("check_offer_work_permits(oferta {:x}) recusada prevista {} devolveu {}", oferta, recusada, r));
    r
}

/// Comparer for `transfer_manager_daily_update` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x8c5880: só a original (ela processa as ofertas, que mudam elencos, finanças e notícias). Confere o
/// que ela garante, contra o estado de antes: rodando o dia (hoje igual à base e a fatia igual a +0x855), na fatia 2 a
/// base vira amanhã, +0x855 volta a 0 e +0x84c sobe (de 49 a 0); nas outras +0x855 sobe 1; e no dia 100 da fatia 2
/// cada pessoa que cumpre a condição (o laço é o último passo, então o estado de depois é o dele) fica com +0x1e (a
/// segunda nacionalidade) = a nação do clube: a naturalização do dia 100.
pub(crate) unsafe extern "thiscall" fn comparar_transfer_manager_daily_update(this: usize, hoje: usize, fatia: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_TRANSFER_MANAGER_DAILY_UPDATE);
    let f = fatia as u8;
    let sem_base = read_i16(this + 0x84f) == 0;
    let roda = if sem_base {
        true
    } else {
        read_i16(hoje + 2) == read_i16(this + 0x84f) && read_i16(hoje) == read_i16(this + 0x84d) && read_u8(this + 0x855) == f
    };
    let (f855, j84c) = (if sem_base { f } else { read_u8(this + 0x855) }, read_u8(this + 0x84c));
    let mut amanha = [0u8; 8];
    {
        let mais: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_DATE_ADD_DAYS);
        let r = mais(hoje, amanha.as_mut_ptr() as usize, 1);
        amanha = core::ptr::read_unaligned(r as *const [u8; 8]);
    }
    o(this, hoje, fatia);
    if !roda {
        return;
    }
    let mut ok;
    if f == 2 {
        let j = j84c.wrapping_add(1);
        let esperado = if (j as i8) >= 0x32 { 0 } else { j };
        let base = core::ptr::read_unaligned((this + 0x84d) as *const [u8; 4]);
        ok = read_u8(this + 0x855) == 0 && read_u8(this + 0x84c) == esperado && base[..] == amanha[..4];
    } else {
        ok = read_u8(this + 0x855) == f855.wrapping_add(1);
    }
    let mut erradas = 0u32;
    if f == 2 && read_i16(hoje) == 100 {
        let nova: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE);
        let comunitaria: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_STAFF_EU_NATIONAL);
        let ano = read_i16(hoje + 2) as u16;
        let (mut d5, mut d2, mut d10) = ([0u8; 8], [0u8; 8], [0u8; 8]);
        let p5 = nova(d5.as_mut_ptr() as usize, 100, ano.wrapping_sub(5) as u32);
        let p2 = nova(d2.as_mut_ptr() as usize, 100, ano.wrapping_sub(2) as u32);
        let p10 = nova(d10.as_mut_ptr() as usize, 100, ano.wrapping_sub(10) as u32);
        let mut s = read_u32_as_usize(G_STAFF_TABLE);
        for _ in 0..(read_u32_as_usize(G_STAFF_COUNT) as i32).max(0) {
            if meets_residency_naturalisation(s, p2, p5, p10, comunitaria) && read_u32_as_usize(s + 0x1e) != read_u32_as_usize(read_u32_as_usize(s + 0x39) + 0x53) {
                erradas += 1;
            }
            s += 0x6e;
        }
        ok = ok && erradas == 0;
    }
    PLACAR.registrar(ok, || format!("transfer_manager_daily_update(dia {}, fatia {}): +0x855 {} -> {}, +0x84c {} -> {}, pessoas sem a nação do clube em +0x1e {}",
        read_i16(hoje), f, f855, read_u8(this + 0x855), j84c, read_u8(this + 0x84c), erradas));
}

/// Comparer for `process_due_offer` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x8c63a0: só a original (cada ramo muda o status, reagenda, aloca notícias, conclui ou tira a oferta).
pub(crate) unsafe extern "thiscall" fn comparar_process_due_offer(this: usize, vaga: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_PROCESS_DUE_OFFER)(this, vaga) // simplification: só a original
}

/// Comparer for `check_offer_restrictions` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x8cfc90: só a original (a vaga no elenco, 0x8cd450, pode liberar o jogador mais barato, e a lista de
/// observação aloca nós).
pub(crate) unsafe extern "thiscall" fn comparar_check_offer_restrictions(this: usize, o: usize, decide: usize, motivo: *mut u16, sem_elenco: u32) -> u8 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, *mut u16, u32) -> u8>(IDX_CHECK_OFFER_RESTRICTIONS)(this, o, decide, motivo, sem_elenco) // simplification: só a original
}

/// Comparer for `pay_former_club_fee_cuts` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x8e4f20: a cópia em ensaio (sem a notícia) e a original a partir do mesmo estado, lado a lado nas
/// finanças do comprador (0x167 bytes) e em [0xae1080]; com entrega da notícia, só as finanças.
pub(crate) unsafe extern "thiscall" fn comparar_pay_former_club_fee_cuts(this: usize, o: usize) {
    let original = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_PAY_FORMER_CLUB_FEE_CUTS);
    let fin = european_cup_club_finance(friendly_club_by_id(read_usize(o + 8)));
    FORMER_CLUB_FEE_DELIVERIES = 0;
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(fin, 0x167), (G_CONTRACT_LOOKUP_RESULT, 4)], || original(this, o), || {
        FORMER_CLUB_FEE_DRY_RUN = true;
        crate::aleatorio::sem_contar(|| pay_former_club_fee_cuts(this, o));
        FORMER_CLUB_FEE_DRY_RUN = false;
    });
    let entregas = FORMER_CLUB_FEE_DELIVERIES;
    let igual = igual || (entregas != 0 && mo.0[0] == mr.0[0]);
    PLACAR.registrar(igual, || format!("pay_former_club_fee_cuts(oferta {:#x}) entregas={} original={:x?} rust={:x?}", o, entregas, mo, mr));
}

/// Comparer for `pay_next_sale_percentage` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x8cb0f0: a cópia em ensaio (sem a notícia e sem os totais do clube) e a original a partir do mesmo
/// estado, lado a lado nas finanças dos dois clubes, em [0xae1080] e no sorteio (a 0x4d0c60 pode sortear); pago (a
/// notícia e os totais da original podem regravar [0xae1080]), só as finanças e a devolução.
pub(crate) unsafe extern "thiscall" fn comparar_pay_next_sale_percentage(this: usize, p: usize, o: usize) -> u32 {
    let original = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_PAY_NEXT_SALE_PERCENTAGE);
    let mut regioes = vec![(G_CONTRACT_LOOKUP_RESULT, 4usize)];
    regioes.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    if let Some((a, b)) = clause_finance_regions(this, p) {
        regioes.push((a, 0x167));
        regioes.push((b, 0x167));
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || original(this, p, o), || {
        FORMER_CLUB_FEE_DRY_RUN = true;
        let n = crate::aleatorio::sem_contar(|| pay_next_sale_percentage(this, p, o));
        FORMER_CLUB_FEE_DRY_RUN = false;
        n
    });
    let igual = r == n && (igual || (r != 0 && mo.0[3..] == mr.0[3..]));
    PLACAR.registrar(igual, || format!("pay_next_sale_percentage(pessoa {:#x}, oferta {:#x}) original={} {:x?} rust={} {:x?}", p, o, r, mo, n, mr));
    r
}

/// Comparer for `post_sale_percentage_news` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x8db3a0: só a original (a notícia é entregue e não pode sair duas vezes; a cópia leva o ramo no
/// modo Rust). simplification: sem plano a conferir além do que a original entrega.
pub(crate) unsafe extern "thiscall" fn comparar_post_sale_percentage_news(this: usize, e: usize, parte: i32, o: usize, da_oferta: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, usize, u32)>(IDX_POST_SALE_PERCENTAGE_NEWS)(this, e, parte, o, da_oferta)
}

/// Comparer for `is_work_permit_granted` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x8c7950: lado a lado nas marcas da pessoa (8 bytes), no sorteio e na devolução (a cópia em
/// sem_contar).
pub(crate) unsafe extern "thiscall" fn comparar_is_work_permit_granted(this: usize, p: usize, nacao: usize, oferta: usize, registro: usize, sorteio: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, u32) -> u32>(IDX_IS_WORK_PERMIT_GRANTED);
    let mut regioes = vec![crate::aleatorio::REGIOES_DO_ESTADO[0], crate::aleatorio::REGIOES_DO_ESTADO[1]];
    if p != 0 && (read_usize(p) as i32) < read_usize(G_STAFF_COUNT) as i32 {
        regioes.push((staff_flags_record(this, p), 8));
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&regioes, || o(this, p, nacao, oferta, registro, sorteio),
        || crate::aleatorio::sem_contar(|| is_work_permit_granted(this, p, nacao, oferta, registro, sorteio)));
    PLACAR.registrar(igual && r == n, || format!("is_work_permit_granted(pessoa {:#x}, oferta {:#x}, registro {:#x}, sorteio {}) original={} {:x?} rust={} {:x?}", p, oferta, registro, sorteio, r, mo, n, mr));
    r
}

/// Comparador da 0x8c5800: lado a lado em [0xae1080] e no al devolvido.
pub(crate) unsafe extern "thiscall" fn comparar_transfer_manager_get_squad_status_for_reason(this: usize, p: usize, motivo: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_TRANSFER_MANAGER_GET_SQUAD_STATUS_FOR_REASON);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(G_CONTRACT_LOOKUP_RESULT, 4)], || o(this, p, motivo), || transfer_manager_get_squad_status_for_reason(this, p, motivo));
    PLACAR.registrar(igual && r as u8 == n as u8, || format!("transfer_manager_get_squad_status_for_reason(pessoa {:#x}, motivo {:#x}) original={} {:x?} rust={} {:x?}", p, motivo, r as u8, mo, n as u8, mr));
    r
}

/// Comparer for `get_negotiation_slot` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x8c6200: pelo valor (só lê).
pub(crate) unsafe extern "thiscall" fn comparar_get_negotiation_slot(this: usize, i: i32) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, i32) -> usize>(IDX_GET_NEGOTIATION_SLOT)(this, i);
    let n = get_negotiation_slot(this, i);
    PLACAR.registrar(r == n, || format!("get_negotiation_slot({}) original={:#x} rust={:#x}", i, r, n));
    r
}

/// Comparer for `reject_loan_offer` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x8bfa30: o da recusa e da retirada (`compare_reject_withdraw`: a cópia em ensaio e a original do mesmo estado;
/// iguais o estado, +0x10, a marca +4 dos jogadores da troca e a marca de oferta da pessoa).
pub(crate) unsafe extern "thiscall" fn comparar_reject_loan_offer(this: usize, vaga: *mut usize, marcar: u32, motivo: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *mut usize, u32, u32)>(IDX_REJECT_LOAN_OFFER);
    compare_reject_withdraw("reject_loan_offer", this, vaga, || o(this, vaga, marcar, motivo), || reject_loan_offer(this, vaga, marcar, motivo))
}

/// Comparer for `check_squad_limits` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x8cd450: a cópia em ensaio (sem liberar) e depois a original, do mesmo estado ([0xae1080], o motivo,
/// os cursores da fila livre e da troca). Sem liberação: iguais a devolução e essas regiões. Com liberação (a cópia
/// guardou o escolhido): iguais a devolução e o escolhido ter saído do clube (o contrato dele, pela 0x4cf5a0 que só
/// lê, sem o clube); o resto a original refaz ao liberar. Clube nulo: a caixa, só a original.
pub(crate) unsafe extern "thiscall" fn comparar_check_squad_limits(this: usize, clube: usize, o: usize, contar_fila: u32, motivo: *mut u16) -> u32 {
    let f = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, *mut u16) -> u32>(IDX_CHECK_SQUAD_LIMITS);
    if clube == 0 {
        return f(this, clube, o, contar_fila, motivo);
    }
    let mut regioes = vec![(G_CONTRACT_LOOKUP_RESULT, 4usize), (read_usize(G_TRANSFER_MANAGER) + 0x936, 4)];
    if !motivo.is_null() {
        regioes.push((motivo as usize, 2));
    }
    if read_usize(o + 4) != 0 {
        regioes.push((read_usize(o + 4) + 0xa, 4));
    }
    let antes = crate::comparacao::Memoria::ler(&regioes);
    SQUAD_LIMITS_DRY_RUN = true;
    SQUAD_LIMITS_RELEASED_PLAYER = 0;
    let n = crate::aleatorio::pausar_rastro(|| check_squad_limits(this, clube, o, contar_fila, motivo));
    SQUAD_LIMITS_DRY_RUN = false;
    let liberado = SQUAD_LIMITS_RELEASED_PLAYER;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    antes.escrever();
    let r = f(this, clube, o, contar_fila, motivo);
    let mo = crate::comparacao::Memoria::ler(&regioes);
    let igual = if liberado == 0 {
        mo.0 == mr.0
    } else {
        let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
        let c = contrato(read_usize(G_CONTRACT_MANAGER), liberado);
        c == 0 || friendly_club_by_id(read_usize(c + 4)) != clube
    };
    PLACAR.registrar(igual && r == n, || {
        format!("check_squad_limits(clube {:x}, oferta {:x}, fila {}) original={} rust={} liberado={:x} {:x?} x {:x?}", clube, o, contar_fila, r, n, liberado, mo, mr)
    });
    r
}

/// Comparer for `find_next_sale_percentage` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x8d0dd0: lado a lado na marca da pessoa, em [0xae1080] e na devolução (só lê o resto).
pub(crate) unsafe extern "thiscall" fn comparar_find_next_sale_percentage(this: usize, p: usize) -> usize {
    let f = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_FIND_NEXT_SALE_PERCENTAGE);
    let m = read_usize(this + 0x86e).wrapping_add(read_usize(p).wrapping_mul(8));
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(m, 1), (G_CONTRACT_LOOKUP_RESULT, 4)], || f(this, p), || find_next_sale_percentage(this, p));
    PLACAR.registrar(igual && r == n, || format!("find_next_sale_percentage(pessoa {:x}) original={:x} rust={:x} {:x?} x {:x?}", p, r, n, mo, mr));
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
        return o(this, jogo, partida); // simplification: só a original (lixo da pilha ou sem permissões)
    }
    let regioes = [(v, n as usize * 0xe)];
    let antes = crate::comparacao::Memoria::ler(&regioes);
    APPEARANCE_CLAUSES_DRY_RUN = true;
    crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| update_appearance_clauses_after_match(this, jogo, partida)));
    APPEARANCE_CLAUSES_DRY_RUN = false;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    antes.escrever();
    o(this, jogo, partida);
    // a original não realoca o vetor (os prêmios mexem em outros vetores)
    let mo = crate::comparacao::Memoria::ler(&regioes);
    PLACAR.registrar(mo == mr, || {
        let (a, b) = (&mo.0[0].1, &mr.0[0].1);
        let dif: Vec<usize> = (0..a.len()).filter(|&i| a[i] != b[i]).take(16).collect();
        format!("update_appearance_clauses_after_match(jogo {:x}) permissões diferentes nos bytes {:x?}", jogo, dif)
    });
}

/// Comparer for `add_international_team_appearance_money` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a original acrescenta (pode realocar) e não pode rodar duas vezes: o plano antes (a conta e os 13
/// bytes), a original roda; confere a conta e que o registro está no vetor (reordenado).
pub(crate) unsafe extern "thiscall" fn comparar_add_international_team_appearance_money(this: usize, registro: usize) {
    let n = read_usize(this + 0x8a0);
    let alvo = core::slice::from_raw_parts(registro as *const u8, 0xd).to_vec();
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_ADD_INTERNATIONAL_TEAM_APPEARANCE_MONEY)(this, registro);
    if read_usize(this + 0x898) == 0 {
        return; // sem memória: a caixa já saiu
    }
    let conta = read_usize(this + 0x8a0);
    let ok = conta == n.wrapping_add(1) && vector_contains_record(read_usize(this + 0x898), conta, 0xd, &alvo);
    PLACAR.registrar(ok, || format!("add_international_team_appearance_money: conta {} -> {}, registro {:x?}", n, conta, alvo));
}

/// Comparer for `post_transfer_finalised_news` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_post_transfer_finalised_news(this: usize, o: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_POST_TRANSFER_FINALISED_NEWS)(this, o)
}

/// Comparer for `free_transfer_staff` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_free_transfer_staff(this: usize, p: usize, clube: usize, motivo: u32, compensacao: i32, so_registro: u32) {
    let ficha: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_LOAN_CONTRACT);
    let emprestada = ficha(read_usize(G_CONTRACT_MANAGER), p) != 0;
    let n = read_usize(this + 0x86a);
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, i32, u32)>(IDX_FREE_TRANSFER_STAFF)(this, p, clube, motivo, compensacao, so_registro);
    if emprestada {
        PLACAR.registrar(read_usize(this + 0x86a) == n, || format!("free_transfer_staff(pessoa {}) emprestada: conta {} -> {}", read_usize(p), n, read_usize(this + 0x86a)));
        return;
    }
    if read_usize(this + 0x862) == 0 {
        return; // sem memória: a caixa já saiu
    }
    let r = read_usize(this + 0x862).wrapping_add(n.wrapping_mul(0x25));
    let mut ok = read_usize(this + 0x86a) == n.wrapping_add(1)
        && read_usize(r) == read_usize(p)
        && read_usize(r + 0x10) as u32 == u32::MAX
        && read_usize(r + 0x14) as u32 == if clube != 0 { read_usize(clube) as u32 } else { u32::MAX }
        && read_u8(r + 0x1c) == if so_registro != 0 { 5 } else { 3 };
    if so_registro == 0 {
        ok = ok && get_contract_unstored(p) == 0 && read_usize(p + 0x52) == 0 && read_u8(p + 0x6d) == 0;
    }
    PLACAR.registrar(ok, || format!("free_transfer_staff(pessoa {}, clube {:x}, só registro {}): conta {} -> {}, registro {:x?}", read_usize(p), clube, so_registro,
        n, read_usize(this + 0x86a), core::slice::from_raw_parts(r as *const u8, 0x1d)));
}

/// Comparer for `post_contract_talks_news` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_post_contract_talks_news(this: usize, o: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_POST_CONTRACT_TALKS_NEWS)(this, o)
}

/// Comparer for `reject_transfer_offer` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_reject_transfer_offer(this: usize, vaga: *mut usize, marcar: u32, motivo: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *mut usize, u32, u32)>(IDX_REJECT_TRANSFER_OFFER);
    compare_reject_withdraw("reject_transfer_offer", this, vaga, || o(this, vaga, marcar, motivo), || reject_transfer_offer(this, vaga, marcar, motivo))
}

/// Comparer for `withdraw_transfer_offer` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_withdraw_transfer_offer(this: usize, vaga: *mut usize, marcar: u32, motivo: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *mut usize, u32, u32)>(IDX_WITHDRAW_TRANSFER_OFFER);
    compare_reject_withdraw("withdraw_transfer_offer", this, vaga, || o(this, vaga, marcar, motivo), || withdraw_transfer_offer(this, vaga, marcar, motivo))
}

/// Comparer for `post_offer_rejected_news` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_post_offer_rejected_news(this: usize, o: usize, motivo: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_POST_OFFER_REJECTED_NEWS)(this, o, motivo)
}

/// Comparer for `unsettle_player_after_bid` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador: a cópia em ensaio e a original a partir do mesmo estado (a oferta, [0xae1080] e o sorteio). Sem chegar
/// à queixa (quase sempre), tudo igual; chegando, só a original (a queixa, a fuga e as antipatias não podem rodar duas
/// vezes e a original sorteia mais).
pub(crate) unsafe extern "thiscall" fn comparar_unsettle_player_after_bid(this: usize, o: usize) {
    let orig_f = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_UNSETTLE_PLAYER_AFTER_BID);
    let mut regioes = vec![(o, 0x30usize), (G_CONTRACT_LOOKUP_RESULT, 4)];
    regioes.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let antes = crate::comparacao::Memoria::ler(&regioes);
    UNSETTLE_PLAYER_DRY_RUN = true;
    UNSETTLE_PLAYER_REACHED = false;
    crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| unsettle_player_after_bid(this, o)));
    UNSETTLE_PLAYER_DRY_RUN = false;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    antes.escrever();
    orig_f(this, o);
    if UNSETTLE_PLAYER_REACHED {
        return;
    }
    let mo = crate::comparacao::Memoria::ler(&regioes);
    PLACAR.registrar(mo == mr, || format!("unsettle_player_after_bid(oferta {:#x}) original={:x?} rust={:x?}", o, mo, mr));
}

/// Comparer for `post_offer_collapsed_news` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_post_offer_collapsed_news(this: usize, o: usize, clube: usize, motivo: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(IDX_POST_OFFER_COLLAPSED_NEWS)(this, o, clube, motivo)
}

/// Comparer for `post_appearance_money_news` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_post_appearance_money_news(this: usize, clausula: usize, valor: u32, tipo: u32, calado: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32)>(IDX_POST_APPEARANCE_MONEY_NEWS)(this, clausula, valor, tipo, calado)
}

/// Comparer for `finalise_transfer` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_finalise_transfer(this: usize, o: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_FINALISE_TRANSFER)(this, o);
    let e = read_u8(o + 0x2c);
    PLACAR.registrar(e == 0x14, || format!("finalise_transfer(oferta {:#x}) estado={:#x} (plano 0x14)", o, e));
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
pub(crate) unsafe extern "thiscall" fn comparar_add_config_loan(this: usize, p: usize, clube: usize, inicio: usize, fim: usize) {
    let emprestado: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_LOAN_CONTRACT);
    let tinha = emprestado(read_usize(G_CONTRACT_MANAGER), p);
    let passa = config_loan_allowed(p, clube, inicio, fim).is_some();
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize)>(IDX_ADD_CONFIG_LOAN)(this, p, clube, inicio, fim);
    let tem = emprestado(read_usize(G_CONTRACT_MANAGER), p);
    let igual = if passa { tem != 0 } else { tem == tinha };
    PLACAR.registrar(igual, || format!("add_config_loan(pessoa {:#x}) passa={} antes={:#x} depois={:#x}", read_usize(p), passa, tinha, tem));
}

/// Comparer for `post_transfer_delay_request_news` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_post_transfer_delay_request_news(this: usize, o: usize, motivo: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_POST_TRANSFER_DELAY_REQUEST_NEWS)(this, o, motivo)
}

/// Comparer for `confirm_offer_by_club` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: sem chegar ao acordo dos dois (estado final diferente de 0xe), só o estado muda: lado a lado sobre a
/// oferta. Com o acordo, a oferta é agendada ou vira transferência futura (mexe nas outras ofertas e aloca): só a
/// original.
pub(crate) unsafe extern "thiscall" fn comparar_confirm_offer_by_club(this: usize, oferta: usize, clube: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_CONFIRM_OFFER_BY_CLUB);
    if state_after_approval(oferta, clube) == 0xe {
        return o(this, oferta, clube);
    }
    let regioes = [(oferta, 0x31usize)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, oferta, clube), || confirm_offer_by_club(this, oferta, clube));
    PLACAR.registrar(igual, || format!("confirm_offer_by_club(oferta {:#x}, clube {:#x}) original={:x?} rust={:x?}", oferta, clube, mo, mr));
}

/// Comparer for `try_complete_offer` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_try_complete_offer(this: usize, oferta: usize, no_prazo: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_TRY_COMPLETE_OFFER)(this, oferta, no_prazo)
}

/// Comparer for `settle_previous_transfer_clauses` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: sem marca de cláusula (bits 2, 4 e 8) no jogador nem nas pessoas da troca, só o cursor da lista muda:
/// lado a lado sobre a lista e as marcas. Com alguma, paga (finanças, notícias que alocam): só a original.
pub(crate) unsafe extern "thiscall" fn comparar_settle_previous_transfer_clauses(this: usize, oferta: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_SETTLE_PREVIOUS_TRANSFER_CLAUSES);
    let marcas = read_usize(this + 0x86e);
    let tem = |id: usize| read_u8(marcas.wrapping_add(id.wrapping_mul(8))) & 0xe != 0;
    let l = read_usize(oferta + 4);
    let mut regioes = vec![(marcas.wrapping_add(read_usize(oferta + 0xc).wrapping_mul(8)), 8usize)];
    let mut alguma = tem(read_usize(oferta + 0xc));
    if l != 0 {
        regioes.push((l, 0xe));
        // o percurso sem mexer no cursor da lista: os nós (+0 o item, +6 o próximo) a partir da cabeça +2
        let mut no = read_usize(l + 2);
        let mut n = 0;
        while no != 0 && n < 64 {
            let item = read_usize(no);
            if item != 0 && tem(read_usize(item)) {
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
        return o(this, oferta);
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, oferta), || settle_previous_transfer_clauses(this, oferta));
    PLACAR.registrar(igual, || format!("settle_previous_transfer_clauses(oferta {:#x}) original={:x?} rust={:x?}", oferta, mo, mr));
}

/// Comparer for `advance_offer_to_club_approval` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_advance_offer_to_club_approval(this: usize, oferta: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_ADVANCE_OFFER_TO_CLUB_APPROVAL)(this, oferta)
}

/// Comparer for `post_offer_accepted_news` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_post_offer_accepted_news(this: usize, oferta: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_POST_OFFER_ACCEPTED_NEWS)(this, oferta)
}

/// Comparer for `post_loan_offer_news` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_post_loan_offer_news(this: usize, oferta: usize, termos: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_POST_LOAN_OFFER_NEWS)(this, oferta, termos)
}

/// Comparer for `add_transfer_negotiation` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_add_transfer_negotiation(this: usize, oferta: usize, termos: usize, marcar: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(IDX_ADD_TRANSFER_NEGOTIATION)(this, oferta, termos, marcar)
}

/// Comparer for `post_player_rejects_move_news` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_post_player_rejects_move_news(this: usize, oferta: usize, motivo: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_POST_PLAYER_REJECTS_MOVE_NEWS)(this, oferta, motivo)
}

/// Comparer for `init_work_permits` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_init_work_permits(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_INIT_WORK_PERMITS)(this)
}

/// Comparer for `get_clause_count` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor (só lê).
pub(crate) unsafe extern "thiscall" fn comparar_get_clause_count(this: usize, pessoa: usize, tipo: u32) -> u16 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u16>(IDX_GET_CLAUSE_COUNT)(this, pessoa, tipo);
    let n = get_clause_count(this, pessoa, tipo);
    PLACAR.registrar(r == n, || format!("get_clause_count(pessoa {}, tipo {}) original={} rust={}", read_usize(pessoa), tipo as u8, r, n));
    r
}

/// Comparer for `find_first_team_appearance_money` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelo byte do registro da pessoa, e pela devolução.
pub(crate) unsafe extern "thiscall" fn comparar_find_first_team_appearance_money(this: usize, pessoa: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_FIND_FIRST_TEAM_APPEARANCE_MONEY);
    let m = read_usize(this + 0x86e).wrapping_add(read_usize(pessoa).wrapping_mul(8));
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(m, 1)], || o(this, pessoa), || find_first_team_appearance_money(this, pessoa));
    PLACAR.registrar(igual && r == n, || format!("find_first_team_appearance_money(pessoa {}) original={:x} rust={:x} {:x?} x {:x?}", read_usize(pessoa), r, n, mo, mr));
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
pub(crate) unsafe extern "thiscall" fn comparar_find_next_window_open_date(this: usize, clube: usize, saida: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_FIND_NEXT_WINDOW_OPEN_DATE);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(saida, 8)], || o(this, clube, saida), || find_next_window_open_date(this, clube, saida));
    PLACAR.registrar(igual, || format!("find_next_window_open_date(clube {:x}) {:x?} x {:x?}", clube, mo, mr));
}

/// Comparer for `find_negotiated_loan_contract` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor (as duas voltas deixam o cursor da lista no mesmo lugar).
pub(crate) unsafe extern "thiscall" fn comparar_find_negotiated_loan_contract(this: usize, pessoa: usize, clube: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> usize>(IDX_FIND_NEGOTIATED_LOAN_CONTRACT)(this, pessoa, clube);
    let n = find_negotiated_loan_contract(this, pessoa, clube);
    PLACAR.registrar(r == n, || format!("find_negotiated_loan_contract(pessoa {}, clube {}) original={:x} rust={:x}", read_usize(pessoa), read_usize(clube), r, n));
    r
}

/// Comparer for `build_clause_offer_option` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_build_clause_offer_option(this: usize, clube: usize, jogador: usize, saida: *mut usize, apelo: *mut u8, valor: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, *mut usize, *mut u8, u32) -> u32>(IDX_BUILD_CLAUSE_OFFER_OPTION)(this, clube, jogador, saida, apelo, valor)
}

/// Comparer for `transfer_manager_is_rejection_reason_valid` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelo contrato da vez ([0xae1080]), e pela devolução.
pub(crate) unsafe extern "stdcall" fn comparar_transfer_manager_is_rejection_reason_valid(motivo: u32, oferta: usize) -> u32 {
    let o = original_fn::<unsafe extern "stdcall" fn(u32, usize) -> u32>(IDX_TRANSFER_MANAGER_IS_REJECTION_REASON_VALID);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(G_CONTRACT_LOOKUP_RESULT, 4)], || o(motivo, oferta), || transfer_manager_is_rejection_reason_valid(motivo, oferta));
    PLACAR.registrar(igual && r == n, || format!("transfer_manager_is_rejection_reason_valid({:x}, pessoa {}) original={} rust={} {:x?} x {:x?}", motivo as u16, read_usize(oferta + 0xc), r, n, mo, mr));
    r
}

/// Comparer for `transfer_manager_get_squad_status_reason` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelo contrato da vez ([0xae1080]), e pela devolução.
pub(crate) unsafe extern "stdcall" fn comparar_transfer_manager_get_squad_status_reason(oferta: usize) -> u16 {
    let o = original_fn::<unsafe extern "stdcall" fn(usize) -> u16>(IDX_TRANSFER_MANAGER_GET_SQUAD_STATUS_REASON);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(G_CONTRACT_LOOKUP_RESULT, 4)], || o(oferta), || transfer_manager_get_squad_status_reason(oferta));
    PLACAR.registrar(igual && r == n, || format!("transfer_manager_get_squad_status_reason(pessoa {}) original={:x} rust={:x} {:x?} x {:x?}", read_usize(oferta + 0xc), r, n, mo, mr));
    r
}

/// Comparer for `fill_squad_restrictions` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelos 42 bytes da tabela (o alcance e a virtual das regras só escrevem nela).
pub(crate) unsafe extern "thiscall" fn comparar_fill_squad_restrictions(this: usize, partida: usize, t: *mut u8, so_a_principal: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut u8, u32)>(IDX_FILL_SQUAD_RESTRICTIONS);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(t as usize, 42)], || o(this, partida, t, so_a_principal), || fill_squad_restrictions(this, partida, t, so_a_principal));
    PLACAR.registrar(igual, || format!("fill_squad_restrictions({:x}, {}) {:x?} x {:x?}", partida, so_a_principal, mo, mr));
}

/// Comparer for `transfer_manager_check_matchday_squad_limit` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pela saída (6 bytes) e pelo aviso (0x100 bytes, com `msg`), e pela devolução. A 0x8968f0 pode
/// pôr −1 numa vaga sem pessoa válida: a original já deixou assim e a cópia acha a mesma vaga vazia.
pub(crate) unsafe extern "thiscall" fn comparar_transfer_manager_check_matchday_squad_limit(this: usize, clube: usize, partida: usize, msg: usize, jogadores: usize, nao_usado: u32, saida: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, u32, usize) -> u32>(IDX_TRANSFER_MANAGER_CHECK_MATCHDAY_SQUAD_LIMIT);
    let mut regioes = [(0usize, 0usize); 2];
    let mut n = 0;
    if saida != 0 {
        regioes[n] = (saida, 6);
        n += 1;
    }
    if msg != 0 {
        regioes[n] = (msg, 0x100);
        n += 1;
    }
    let (r, c, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes[..n], || o(this, clube, partida, msg, jogadores, nao_usado, saida), || transfer_manager_check_matchday_squad_limit(this, clube, partida, msg, jogadores, nao_usado, saida));
    PLACAR.registrar(igual && r == c, || format!("transfer_manager_check_matchday_squad_limit(clube {:x}, partida {:x}) original={} rust={} {:x?} x {:x?}", clube, partida, r, c, mo, mr));
    r
}

/// Comparer for `player_needs_work_permit` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor (só lê). Jogador nulo com clube de nação: a caixa, só a original.
pub(crate) unsafe extern "thiscall" fn comparar_player_needs_work_permit(this: usize, jogador: usize, clube: usize, ignorar_a_dada: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32>(IDX_PLAYER_NEEDS_WORK_PERMIT);
    if jogador == 0 {
        return o(this, jogador, clube, ignorar_a_dada);
    }
    let r = o(this, jogador, clube, ignorar_a_dada);
    let n = player_needs_work_permit(this, jogador, clube, ignorar_a_dada);
    PLACAR.registrar(r == n, || format!("player_needs_work_permit(jogador {}, clube {:x}, {}) original={} rust={}", read_usize(jogador), clube, ignorar_a_dada, r, n));
    r
}

/// Comparer for `can_club_bid_for_player` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelo motivo, pela marca da pessoa (+0x86e) e pelo cursor da fila livre, e pela devolução.
/// As outras gravações (tirar a marca 0x40, marcar +0x2d da oferta em negociação) a cópia repete sobre o que a original
/// deixou e chega ao mesmo.
pub(crate) unsafe extern "thiscall" fn comparar_can_club_bid_for_player(this: usize, jogador: usize, clube: usize, motivo: *mut u16, ignorar_abertas: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, *mut u16, u32) -> u32>(IDX_CAN_CLUB_BID_FOR_PLAYER);
    let marca = read_usize(this + 0x86e).wrapping_add(read_usize(jogador).wrapping_mul(8));
    let regioes = [(marca, 1), (this + 0x936, 4), (motivo as usize, 2)];
    let n = if motivo.is_null() { 2 } else { 3 };
    let (r, c, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&regioes[..n], || o(this, jogador, clube, motivo, ignorar_abertas), || can_club_bid_for_player(this, jogador, clube, motivo, ignorar_abertas));
    PLACAR.registrar(igual && r == c, || format!("can_club_bid_for_player(jogador {}, clube {:x}, {}) original={} rust={} {:x?} x {:x?}", read_usize(jogador), clube, ignorar_abertas, r, c, mo, mr));
    r
}

/// Comparer for `note_terms_offered_to_exchange_player` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelo estado da oferta, pelas marcas (+4) dos itens da troca e pelo cursor da lista. Fora da
/// conta: a caixa, só a original.
pub(crate) unsafe extern "thiscall" fn comparar_note_terms_offered_to_exchange_player(this: usize, k: i32, pessoa: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32, u32)>(IDX_NOTE_TERMS_OFFERED_TO_EXCHANGE_PLAYER);
    if k < 0 || k >= read_usize(this + 4) as i32 || read_usize(read_usize(this).wrapping_add((k as usize).wrapping_mul(4))) == 0 {
        return o(this, k, pessoa);
    }
    let of = read_usize(read_usize(this).wrapping_add((k as usize).wrapping_mul(4)));
    let l = read_usize(of + 4);
    let mut regioes = vec![(of + 0x2c, 1), (l + 0xa, 4)];
    for e in llist_collect(l) {
        regioes.push((e + 4, 1));
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, k, pessoa), || note_terms_offered_to_exchange_player(this, k, pessoa));
    PLACAR.registrar(igual, || format!("note_terms_offered_to_exchange_player({}, pessoa {}) {:x?} x {:x?}", k, pessoa, mo, mr));
}

/// Comparer for `count_club_free_transfer_queue` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelo cursor da fila e pela devolução (só lê). Clube nulo: a caixa, só a original.
pub(crate) unsafe extern "thiscall" fn comparar_count_club_free_transfer_queue(this: usize, clube: usize) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_COUNT_CLUB_FREE_TRANSFER_QUEUE);
    if clube == 0 {
        return o(this, clube);
    }
    let (r, c, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this + 0x936, 4)], || o(this, clube), || count_club_free_transfer_queue(this, clube));
    PLACAR.registrar(igual && r == c, || format!("count_club_free_transfer_queue({:x}) original={} rust={} {:x?} x {:x?}", clube, r, c, mo, mr));
    r
}

/// Comparer for `get_future_transfer` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pela marca do jogador e por `saida`, e pela devolução (tirar a marca é o único outro efeito;
/// a cópia, depois da original, acha o mesmo).
pub(crate) unsafe extern "thiscall" fn comparar_get_future_transfer(this: usize, jogador: usize, saida: *mut usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut usize) -> u32>(IDX_GET_FUTURE_TRANSFER);
    let regioes = [(staff_flags_byte(this, read_usize(jogador)) as usize, 1), (saida as usize, 4)];
    let n = if saida.is_null() { 1 } else { 2 };
    let (r, c, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&regioes[..n], || o(this, jogador, saida), || get_future_transfer(this, jogador, saida));
    PLACAR.registrar(igual && r == c, || format!("get_future_transfer(jogador {}) original={} rust={} {:x?} x {:x?}", read_usize(jogador), r, c, mo, mr));
    r
}

/// Comparer for `remove_future_transfer` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pela conta e pelo vetor das futuras, pelas marcas do jogador e dos da troca e pelo cursor
/// da lista da troca (não aloca nem libera).
pub(crate) unsafe extern "thiscall" fn comparar_remove_future_transfer(this: usize, oferta: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_REMOVE_FUTURE_TRANSFER);
    let mut regioes = vec![(this + 0x928, 4), (staff_flags_byte(this, read_usize(oferta + 0xc)) as usize, 1)];
    let n = read_usize(this + 0x928) as i32;
    if n > 0 {
        regioes.push((read_usize(this + 0x920), n as usize * 0xc));
    }
    let l = read_usize(oferta + 4);
    if l != 0 {
        let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
        write_usize(l + 0xa, read_usize(l + 2));
        let mut e = proximo(l);
        while e != 0 {
            regioes.push((staff_flags_byte(this, read_usize(e)) as usize, 1));
            e = proximo(l);
        }
        regioes.push((l + 0xa, 4));
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, oferta), || remove_future_transfer(this, oferta));
    PLACAR.registrar(igual, || format!("remove_future_transfer(oferta {:x}) {:x?} x {:x?}", read_usize(oferta + 0x20), mo, mr));
}

/// Comparer for `add_future_transfer` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a original acrescenta (pode realocar), reordena e suspende as listas de alvos: só ela, conferida pelo
/// plano: a conta sobe de 1, o registro (data e oferta) está no vetor, o estado 0x13 com +0x2d = 1 e a marca 0x10 do
/// jogador.
pub(crate) unsafe extern "thiscall" fn comparar_add_future_transfer(this: usize, oferta: usize, data: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_ADD_FUTURE_TRANSFER);
    let n = read_usize(this + 0x928);
    let mut alvo = [0u8; 0xc];
    alvo[..8].copy_from_slice(core::slice::from_raw_parts(data as *const u8, 8));
    alvo[8..].copy_from_slice(&(read_usize(oferta + 0x20) as u32).to_le_bytes());
    o(this, oferta, data);
    if read_usize(this + 0x920) == 0 {
        return; // sem memória: a caixa já saiu
    }
    let conta = read_usize(this + 0x928);
    let marca = *staff_flags_byte(this, read_usize(oferta + 0xc));
    let ok = conta == n.wrapping_add(1) && vector_contains_record(read_usize(this + 0x920), conta, 0xc, &alvo) && read_u8(oferta + 0x2c) == 0x13 && read_u8(oferta + 0x2d) == 1 && marca & 0x10 != 0;
    PLACAR.registrar(ok, || format!("add_future_transfer(oferta {:x}): conta {} -> {}, estado {:#x}, marca {:#x}", read_usize(oferta + 0x20), n, conta, read_u8(oferta + 0x2c), marca));
}

/// Comparer for `get_open_offers_for_club_players` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: sem `lista`, lado a lado por [0xae1080] e pela devolução; com `lista` (cresce a lista: não pode rodar
/// duas vezes), a cópia conta antes sem lista e a original roda: confere a conta.
pub(crate) unsafe extern "thiscall" fn comparar_get_open_offers_for_club_players(this: usize, clube: usize, lista: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_GET_OPEN_OFFERS_FOR_CLUB_PLAYERS);
    if lista != 0 {
        let n = get_open_offers_for_club_players(this, clube, 0);
        let r = o(this, clube, lista);
        PLACAR.registrar(r == n, || format!("get_open_offers_for_club_players({:x}, lista) original={} rust={}", clube, r, n));
        return r;
    }
    let (r, c, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(G_CONTRACT_LOOKUP_RESULT, 4)], || o(this, clube, 0), || get_open_offers_for_club_players(this, clube, 0));
    PLACAR.registrar(igual && r == c, || format!("get_open_offers_for_club_players({:x}) original={} rust={} {:x?} x {:x?}", clube, r, c, mo, mr));
    r
}

/// Comparer for `collapse_offers_for_player` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: agenda (aloca) e mexe nas listas de alvos: só a original. O plano antes: as ofertas pela pessoa (0x8c5cf0
/// numa lista própria, destruída depois); depois confere que todas ficaram no estado 0xf com +0x2e = 3.
pub(crate) unsafe extern "thiscall" fn comparar_collapse_offers_for_player(this: usize, pessoa: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_COLLAPSE_OFFERS_FOR_PLAYER);
    if read_u8(read_usize(this + 0x86e).wrapping_add(read_usize(pessoa).wrapping_mul(8)) + 1) == 0 {
        return o(this, pessoa);
    }
    let nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let ofertas: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_OFFERS_FOR_PLAYER);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let mut lista = [0u8; 0x10];
    let l = lista.as_mut_ptr() as usize;
    nova(l);
    ofertas(this, pessoa, l, 0);
    let mut alvo = Vec::new();
    write_usize(l + 0xa, read_usize(l + 2));
    let mut e = proximo(l);
    while e != 0 {
        alvo.push(read_usize(read_usize(this).wrapping_add(read_usize(e).wrapping_mul(4))));
        e = proximo(l);
    }
    soltar(l);
    o(this, pessoa);
    let ok = alvo.iter().all(|&of| read_u8(of + 0x2c) == 0xf && read_u8(of + 0x2e) == 3);
    PLACAR.registrar(ok, || format!("collapse_offers_for_player(pessoa {}): {} ofertas, estados {:x?}", read_usize(pessoa), alvo.len(), alvo.iter().map(|&of| (read_u8(of + 0x2c), read_u8(of + 0x2e))).collect::<Vec<_>>()));
}

/// Comparer for `add_next_sale_percentage` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: o plano antes (a conta e os 13 bytes), a original roda; confere a conta e que o registro está no vetor
/// (reordenado).
pub(crate) unsafe extern "thiscall" fn comparar_add_next_sale_percentage(this: usize, registro: usize) {
    let n = read_usize(this + 0x888);
    let alvo = core::slice::from_raw_parts(registro as *const u8, 0xd).to_vec();
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_ADD_NEXT_SALE_PERCENTAGE)(this, registro);
    if read_usize(this + 0x880) == 0 {
        return; // sem memória: a caixa já saiu
    }
    let conta = read_usize(this + 0x888);
    let ok = conta == n.wrapping_add(1) && vector_contains_record(read_usize(this + 0x880), conta, 0xd, &alvo);
    PLACAR.registrar(ok, || format!("add_next_sale_percentage: conta {} -> {}, registro {:x?}", n, conta, alvo));
}

/// Comparer for `transfer_manager_get_offer_completion_date` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pela saída, [0xae1080] e as estáticas das datas, e pelo motivo (as regras, a temporada e a
/// janela só calculam).
pub(crate) unsafe extern "thiscall" fn comparar_transfer_manager_get_offer_completion_date(this: usize, oferta: usize, saida: *mut [u32; 2]) -> u16 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut [u32; 2]) -> u16>(IDX_TRANSFER_MANAGER_GET_OFFER_COMPLETION_DATE);
    let reg = date_comparer_regions(&[saida as usize]);
    let (ro, rr, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || o(this, oferta, saida), || transfer_manager_get_offer_completion_date(this, oferta, saida));
    PLACAR.registrar(igual && ro == rr, || format!("transfer_manager_get_offer_completion_date(tipo {}) motivo {} x {}, memória igual {}", read_u8(oferta + 0x30) & 0xf, ro, rr, igual));
    ro
}

/// Comparer for `find_next_allowed_transfer_date` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pela saída, [0xae1080] e as estáticas das datas, e pelo motivo.
pub(crate) unsafe extern "thiscall" fn comparar_find_next_allowed_transfer_date(this: usize, oferta: usize, saida: *mut [u32; 2]) -> u16 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut [u32; 2]) -> u16>(IDX_FIND_NEXT_ALLOWED_TRANSFER_DATE);
    let reg = date_comparer_regions(&[saida as usize]);
    let (ro, rr, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || o(this, oferta, saida), || find_next_allowed_transfer_date(this, oferta, saida));
    PLACAR.registrar(igual && ro == rr, || format!("find_next_allowed_transfer_date(tipo {}) motivo {} x {}, memória igual {}", read_u8(oferta + 0x30) & 0xf, ro, rr, igual));
    ro
}

/// Comparer for `transfer_manager_get_next_window_dates` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelo início e o fim (só datas, sem estáticas).
pub(crate) unsafe extern "stdcall" fn comparar_transfer_manager_get_next_window_dates(inicio: *mut [u32; 2], fim: *mut [u32; 2]) {
    let o = original_fn::<unsafe extern "stdcall" fn(*mut [u32; 2], *mut [u32; 2])>(IDX_TRANSFER_MANAGER_GET_NEXT_WINDOW_DATES);
    let reg = date_comparer_regions(&[inicio as usize, fim as usize]);
    let (_, _, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || o(inicio, fim), || transfer_manager_get_next_window_dates(inicio, fim));
    PLACAR.registrar(igual, || format!("transfer_manager_get_next_window_dates(hoje {:x}, fim {}) diferente", *(TODAY as *const u32), !fim.is_null()));
}

/// Comparer for `transfer_manager_calc_compensation_fee` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor (lado a lado pelas estáticas das datas; o valor do jogador 0x4d0c60 e a idade só calculam).
pub(crate) unsafe extern "stdcall" fn comparar_transfer_manager_calc_compensation_fee(pessoa: usize, comprador: usize, sem_regras_novas: u32) -> i32 {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, usize, u32) -> i32>(IDX_TRANSFER_MANAGER_CALC_COMPENSATION_FEE);
    let (ro, rr, igual, _, _) = crate::comparacao::lado_a_lado(&DATE_OP_STATICS, || o(pessoa, comprador, sem_regras_novas), || transfer_manager_calc_compensation_fee(pessoa, comprador, sem_regras_novas));
    PLACAR.registrar(igual && ro == rr, || format!("transfer_manager_calc_compensation_fee(pessoa {:x}) {} x {}, estáticas iguais {}", if pessoa != 0 { read_usize(pessoa) } else { 0 }, ro, rr, igual));
    ro
}

/// Comparer for `pay_fee_and_record_clauses` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_pay_fee_and_record_clauses(this: usize, oferta: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_PAY_FEE_AND_RECORD_CLAUSES)(this, oferta)
}

/// Comparer for `transfer_manager_selling_club_must_accept` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelo motivo e [0xae1080], e pelo valor devolvido (o preço e o valor ponderado só calculam).
pub(crate) unsafe extern "stdcall" fn comparar_transfer_manager_selling_club_must_accept(oferta: usize, estrito: u32, motivo: *mut u16) -> u32 {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, u32, *mut u16) -> u32>(IDX_TRANSFER_MANAGER_SELLING_CLUB_MUST_ACCEPT);
    let mut reg: Vec<(usize, usize)> = vec![(G_CONTRACT_LOOKUP_RESULT, 4)];
    if !motivo.is_null() {
        reg.push((motivo as usize, 2));
    }
    reg.extend_from_slice(&DATE_OP_STATICS);
    let (ro, rr, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || o(oferta, estrito, motivo), || transfer_manager_selling_club_must_accept(oferta, estrito, motivo));
    PLACAR.registrar(igual && ro == rr, || format!("transfer_manager_selling_club_must_accept(estrito {}) {} x {}, memória igual {}", estrito, ro, rr, igual));
    ro
}

/// Comparer for `update_work_permit_on_move` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_update_work_permit_on_move(this: usize, pessoa: usize, clube: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_UPDATE_WORK_PERMIT_ON_MOVE)(this, pessoa, clube)
}

/// Comparer for `complete_accepted_contract_offer` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_complete_accepted_contract_offer(this: usize, oferta: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_COMPLETE_ACCEPTED_CONTRACT_OFFER)(this, oferta)
}

/// Comparer for `offer_contract_to_offer_player` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_offer_contract_to_offer_player(this: usize, clube: usize, pessoa: usize, oferta: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32>(IDX_OFFER_CONTRACT_TO_OFFER_PLAYER)(this, clube, pessoa, oferta)
}

/// Comparer for `collapse_offer` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_collapse_offer(this: usize, oferta: usize, clube: usize, motivo: u32, tirar_futura: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32)>(IDX_COLLAPSE_OFFER)(this, oferta, clube, motivo, tirar_futura)
}

/// Comparer for `make_loan_offer` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_make_loan_offer(this: usize, jogador: usize, clube: usize, fim: usize, write_percent_pair: u32, a5: u32, a6: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, u32) -> u32>(IDX_MAKE_LOAN_OFFER)(this, jogador, clube, fim, write_percent_pair, a5, a6)
}

/// Comparer for `process_due_future_transfers` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_process_due_future_transfers(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_PROCESS_DUE_FUTURE_TRANSFERS)(this)
}

/// Comparer for `club_rejects_offer` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_club_rejects_offer(this: usize, oferta: usize, clube: usize, motivo: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(IDX_CLUB_REJECTS_OFFER)(this, oferta, clube, motivo)
}

/// Comparer for `detach_offer_players` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_detach_offer_players(this: usize, oferta: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_DETACH_OFFER_PLAYERS)(this, oferta)
}

/// Comparer for `build_cash_offer_option` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: aloca a oferta e a 0x8e78f0 pode sortear: roda só a original e confere o resultado: devolve 1, a oferta
/// guardada em `saída` tem o valor (+0x1b) e a atração ficou em 50.
pub(crate) unsafe extern "thiscall" fn comparar_build_cash_offer_option(this: usize, clube: usize, jogador: usize, saida: *mut usize, atracao: *mut u8, valor: i32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, *mut usize, *mut u8, i32) -> u32>(IDX_BUILD_CASH_OFFER_OPTION)(this, clube, jogador, saida, atracao, valor);
    let oferta = *saida;
    let ok = r == 1 && oferta != 0 && core::ptr::read_unaligned((oferta + 0x1b) as *const i32) == valor && *atracao == 0x32;
    PLACAR.registrar(ok, || format!("build_cash_offer_option(jogador {:x}, valor {}) devolveu {}, oferta {:x}", read_usize(jogador), valor, r, oferta));
    r
}

/// Comparer for `get_offer_response_delay` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelas duas saídas, o sorteio, [0xae1080] e [0xb67a34] (a caixa da falta de ficha).
pub(crate) unsafe extern "thiscall" fn comparar_get_offer_response_delay(this: usize, oferta: usize, dias: *mut u8, fatias: *mut u8) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut u8, *mut u8)>(IDX_GET_OFFER_RESPONSE_DELAY);
    let mut reg = vec![(dias as usize, 1usize), (fatias as usize, 1), (G_CONTRACT_LOOKUP_RESULT, 4), (0xb67a34, 4)];
    reg.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(
        &reg,
        || o(this, oferta, dias, fatias),
        || crate::aleatorio::sem_contar(|| get_offer_response_delay(this, oferta, dias, fatias)),
    );
    PLACAR.registrar(igual, || format!("get_offer_response_delay({:#x}) {:x?} x {:x?}", oferta, mo, mr));
}

/// Comparer for `pay_transfer_instalments` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_pay_transfer_instalments(this: usize) {
    let lista = this + 0x872;
    let antes = national_teams_list_items(lista);
    let acabam = antes.iter().filter(|&&e| read_u8(e + 0xc) == 1).count();
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_PAY_TRANSFER_INSTALMENTS)(this);
    let depois = national_teams_list_items(lista).len();
    PLACAR.registrar(depois + acabam == antes.len(), || format!("pay_transfer_instalments: {} prestações, {} acabando, {} depois", antes.len(), acabam, depois));
}

/// Comparer for `remove_offer` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_remove_offer(this: usize, oferta: usize) {
    let vaga = read_i32(oferta + 0x20);
    let ligada = read_i32(oferta + 0x28) != -1;
    let (n8, n14, livre) = (read_i32(this + 8), read_i32(this + 0x14), read_i32(this + 0x85e));
    let q = read_usize(this + 0x86e).wrapping_add(read_usize(oferta + 0xc).wrapping_mul(8)) + 1;
    let ofertas = read_u8(q);
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_REMOVE_OFFER)(this, oferta);
    let ok = read_usize(read_usize(this).wrapping_add((vaga as usize).wrapping_mul(4))) == 0
        && read_i32(this + 8) == n8 - 1
        && read_i32(this + 0x14) == n14 - ligada as i32
        && read_i32(this + 0x85e) == livre.min(vaga)
        && read_u8(q) == ofertas.wrapping_sub(1);
    PLACAR.registrar(ok, || format!("remove_offer: vaga {} contas {}/{} -> {}/{} livre {} -> {} ofertas {} -> {}",
        vaga, n8, n14, read_i32(this + 8), read_i32(this + 0x14), livre, read_i32(this + 0x85e), ofertas, read_u8(q)));
}

/// Comparer for `club_has_offer_for_player` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_club_has_offer_for_player(this: usize, pessoa: usize, clube: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_CLUB_HAS_OFFER_FOR_PLAYER);
    if pessoa == 0 {
        return o(this, pessoa, clube);
    }
    let reg = offer_cursor_region(this, pessoa);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, pessoa, clube), || club_has_offer_for_player(this, pessoa, clube));
    PLACAR.registrar(igual && r == n, || format!("club_has_offer_for_player({}, clube {}) original={} rust={} {:x?} x {:x?}", read_usize(pessoa), read_usize(clube), r, n, mo, mr));
    r
}

/// Comparer for `find_club_offer_for_player` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_find_club_offer_for_player(this: usize, pessoa: usize, clube: usize, todas: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32) -> usize>(IDX_FIND_CLUB_OFFER_FOR_PLAYER);
    if pessoa == 0 {
        return o(this, pessoa, clube, todas);
    }
    let reg = offer_cursor_region(this, pessoa);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, pessoa, clube, todas), || find_club_offer_for_player(this, pessoa, clube, todas));
    PLACAR.registrar(igual && r == n, || format!("find_club_offer_for_player({}, clube {}, todas {}) original={:x} rust={:x} {:x?} x {:x?}", read_usize(pessoa), read_usize(clube), todas, r, n, mo, mr));
    r
}

/// Comparer for `player_has_pending_offer` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_player_has_pending_offer(this: usize, pessoa: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_PLAYER_HAS_PENDING_OFFER);
    if pessoa == 0 {
        return o(this, pessoa);
    }
    let reg = offer_cursor_region(this, pessoa);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, pessoa), || player_has_pending_offer(this, pessoa));
    PLACAR.registrar(igual && r == n, || format!("player_has_pending_offer({}) original={} rust={} {:x?} x {:x?}", read_usize(pessoa), r, n, mo, mr));
    r
}

/// Comparer for `is_player_under_offer` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_is_player_under_offer(this: usize, pessoa: usize, visao: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_IS_PLAYER_UNDER_OFFER);
    if pessoa == 0 {
        return o(this, pessoa, visao);
    }
    let reg = offer_cursor_and_lookup_regions(this, pessoa);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, pessoa, visao), || is_player_under_offer(this, pessoa, visao));
    PLACAR.registrar(igual && r == n, || format!("is_player_under_offer({}, visao {}) original={} rust={} {:x?} x {:x?}", read_usize(pessoa), visao, r, n, mo, mr));
    r
}

/// Comparer for `player_club_uses_bosman` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "stdcall" fn comparar_player_club_uses_bosman(pessoa: usize) -> u32 {
    let o = original_fn::<unsafe extern "stdcall" fn(usize) -> u32>(IDX_PLAYER_CLUB_USES_BOSMAN);
    if pessoa == 0 {
        return o(pessoa);
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(G_CONTRACT_LOOKUP_RESULT, 4)], || o(pessoa), || player_club_uses_bosman(pessoa));
    PLACAR.registrar(igual && r == n, || format!("player_club_uses_bosman({}) original={} rust={} {:x?} x {:x?}", read_usize(pessoa), r, n, mo, mr));
    r
}

/// Comparer for `check_board_veto` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a diretoria (0x696b10) mexe na confiança e não pode rodar duas vezes: o plano antes (as três palavras
/// do jogador), a original roda e confere: devolução 0 ou 1; com motivo, 0x24 (as palavras a 90%), 0x17 ou 0x18 quando
/// 1 e intocado quando 0; as palavras intocadas fora do motivo 0x24 (sem motivo, intocadas ou as três a 90%).
pub(crate) unsafe extern "thiscall" fn comparar_check_board_veto(this: usize, oferta: usize, comprador: usize, vendedor: usize, jogador: usize, decide: usize, motivo: *mut u16) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, *mut u16) -> u32>(IDX_CHECK_BOARD_VETO);
    let dados = if jogador == 0 { 0 } else { read_usize(jogador + 0x61) };
    if dados == 0 {
        return o(this, oferta, comprador, vendedor, jogador, decide, motivo);
    }
    let ler = || [0xb, 9, 0xd].map(|k| core::ptr::read_unaligned((dados + k) as *const i16));
    let antes = ler();
    let m0 = if motivo.is_null() { 0 } else { *motivo };
    let r = o(this, oferta, comprador, vendedor, jogador, decide, motivo);
    let depois = ler();
    let a_90 = antes.map(|v| match_pl_ftol(v as f64 * read_f64_at(0x9689d8)) as i16); // 0x9689d8 = 0.9
    let m = if motivo.is_null() { 0 } else { *motivo };
    let ok = match (r, motivo.is_null()) {
        (0, false) => m == m0 && depois == antes,
        (0, true) => depois == antes,
        (1, false) => (m == 0x24 && depois == a_90) || ((m == 0x17 || m == 0x18) && depois == antes),
        (1, true) => depois == antes || depois == a_90,
        _ => false,
    };
    PLACAR.registrar(ok, || format!("check_board_veto devolveu {}, motivo {:#x} (antes {:#x}), palavras {:?} -> {:?}", r, m, m0, antes, depois));
    r
}

/// Comparer for `add_past_transfer` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a original acrescenta (pode realocar) e não pode rodar duas vezes: o plano antes (a conta, o registro),
/// a original roda, confere a conta, o registro novo (as duplas e o byte +0x1c; a data é da 0x8cb940) e a devolução.
pub(crate) unsafe extern "thiscall" fn comparar_add_past_transfer(this: usize, registro: usize) -> usize {
    let n = read_usize(this + 0x86a);
    let fonte = core::slice::from_raw_parts(registro as *const u8, 0x1d).to_vec();
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_ADD_PAST_TRANSFER)(this, registro);
    if read_usize(this + 0x862) == 0 {
        return r; // sem memória: a caixa já saiu
    }
    let novo = read_usize(this + 0x862).wrapping_add(n.wrapping_mul(0x25));
    let gravado = core::slice::from_raw_parts(novo as *const u8, 0x1d);
    PLACAR.registrar(r == novo && read_usize(this + 0x86a) == n.wrapping_add(1) && gravado == &fonte[..], || {
        format!("add_past_transfer: conta {} -> {}, devolveu {:x} (esperava {:x}), registro {:x?} x {:x?}", n, read_usize(this + 0x86a), r, novo, fonte, gravado)
    });
    r
}

/// Comparer for `add_work_permit` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: o plano antes (a conta e o registro esperado), a original roda; confere a conta, a marca 0x20 e que o
/// registro está no vetor (com `ordenar`, em qualquer lugar).
pub(crate) unsafe extern "thiscall" fn comparar_add_work_permit(this: usize, pessoa: usize, nacao: usize, ano: u32, nao_usado: u32, ordenar: u32) {
    let n = read_usize(this + 0x93e);
    let mut alvo = [0u8; 0xe];
    alvo[0..4].copy_from_slice(&(read_usize(pessoa) as u32).to_le_bytes());
    alvo[8..10].copy_from_slice(&(ano as u16).to_le_bytes());
    alvo[0xa..0xe].copy_from_slice(&(read_usize(nacao) as u32).to_le_bytes());
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32)>(IDX_ADD_WORK_PERMIT)(this, pessoa, nacao, ano, nao_usado, ordenar);
    if read_usize(this + 0x93a) == 0 {
        return; // sem memória: a caixa já saiu
    }
    let conta = read_usize(this + 0x93e);
    let marca = *((read_usize(this + 0x86e).wrapping_add(read_usize(pessoa).wrapping_mul(8))) as *const u8);
    let ok = conta == n.wrapping_add(1) && marca & 0x20 != 0 && vector_contains_record(read_usize(this + 0x93a), conta, 0xe, &alvo);
    PLACAR.registrar(ok, || format!("add_work_permit(pessoa {}, nação {}, ano {}): conta {} -> {}, marca {:#x}", read_usize(pessoa), read_usize(nacao), ano as u16, n, conta, marca));
}

/// Comparer for `add_first_team_appearance_money` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: o plano antes (a conta e os 13 bytes), a original roda; confere a conta e que o registro está no vetor
/// (reordenado).
pub(crate) unsafe extern "thiscall" fn comparar_add_first_team_appearance_money(this: usize, registro: usize) {
    let n = read_usize(this + 0x894);
    let alvo = core::slice::from_raw_parts(registro as *const u8, 0xd).to_vec();
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_ADD_FIRST_TEAM_APPEARANCE_MONEY)(this, registro);
    if read_usize(this + 0x88c) == 0 {
        return; // sem memória: a caixa já saiu
    }
    let conta = read_usize(this + 0x894);
    let ok = conta == n.wrapping_add(1) && vector_contains_record(read_usize(this + 0x88c), conta, 0xd, &alvo);
    PLACAR.registrar(ok, || format!("add_first_team_appearance_money: conta {} -> {}, registro {:x?}", n, conta, alvo));
}

/// Comparer for `get_fixture_rules_scope` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelas três saídas (a 0x669470 e a 0x5c6710 só leem).
pub(crate) unsafe extern "thiscall" fn comparar_get_fixture_rules_scope(this: usize, partida: usize, tipo: *mut u8, nacao: *mut usize, continente: *mut usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut u8, *mut usize, *mut usize)>(IDX_GET_FIXTURE_RULES_SCOPE);
    let regioes = [(tipo as usize, 1), (nacao as usize, 4), (continente as usize, 4)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&regioes, || o(this, partida, tipo, nacao, continente), || get_fixture_rules_scope(this, partida, tipo, nacao, continente));
    PLACAR.registrar(igual, || format!("get_fixture_rules_scope({:x}) {:x?} x {:x?}", partida, mo, mr));
}

/// Comparer for `get_ineligibility_code` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor. Jogador com a marca de ausente (0x418f60 com 1, que só lê): só a original (com 0 ela tira
/// a marca de quem está sem clube e a segunda chamada veria outro estado).
pub(crate) unsafe extern "thiscall" fn comparar_get_ineligibility_code(this: usize, jogador: usize, partida: usize) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u8>(IDX_GET_INELIGIBILITY_CODE);
    let ausente: unsafe extern "thiscall" fn(usize, usize, u32) -> u8 = core::mem::transmute(FN_AWOL_IS_PLAYER_AWOL);
    if ausente(read_usize(G_AWOL), jogador, 1) != 0 {
        return o(this, jogador, partida);
    }
    let r = o(this, jogador, partida);
    let n = get_ineligibility_code(this, jogador, partida);
    PLACAR.registrar(r == n, || format!("get_ineligibility_code({:x}, {:x}) original={} rust={}", jogador, partida, r, n));
    r
}

/// Comparer for `write_deadline_news_text` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: o método das regras pode escrever o texto e sortear: só a original; a escolha das regras da cópia é a
/// mesma conta (a prova é a carreira e o modo Rust).
pub(crate) unsafe extern "thiscall" fn comparar_write_deadline_news_text(this: usize, noticia: usize, completo: u32, texto: usize) -> u32 {
    crate::comparacao::ramo("write_deadline_news_text");
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize) -> u32>(IDX_WRITE_DEADLINE_NEWS_TEXT)(this, noticia, completo, texto)
}

/// Comparer for `can_end_loan` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: quando as regras da nação respondem, só a original (o método é delas); senão os dois devolvem 1.
pub(crate) unsafe extern "thiscall" fn comparar_can_end_loan(this: usize, clube: usize, jogador: usize, a: u32, b: u32, c: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) -> u32>(IDX_CAN_END_LOAN)(this, clube, jogador, a, b, c);
    if loan_transfer_rules(this, clube, jogador).is_some() {
        crate::comparacao::ramo("can_end_loan pelas regras");
        return r;
    }
    PLACAR.registrar(r == 1, || format!("can_end_loan({:x}, {:x}) original={} rust=1", clube, jogador, r));
    r
}

/// Comparer for `transfer_manager_offer_needs_work_permit` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pela ficha [0xae1080] e pelo cursor (+0xa) da lista de troca.
pub(crate) unsafe extern "thiscall" fn comparar_transfer_manager_offer_needs_work_permit(this: usize, oferta: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_TRANSFER_MANAGER_OFFER_NEEDS_WORK_PERMIT);
    let lista = read_u32(oferta + 4) as usize;
    let mut reg = vec![(G_CONTRACT_LOOKUP_RESULT, 4)];
    if lista != 0 {
        reg.push((lista + 0xa, 4));
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, oferta), || transfer_manager_offer_needs_work_permit(this, oferta));
    PLACAR.registrar(igual && r == n, || format!("transfer_manager_offer_needs_work_permit({:x}) original={} rust={} {:x?} x {:x?}", oferta, r, n, mo, mr));
    r
}

/// Comparer for `update_work_permits` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelo vetor, pela contagem e, de cada permissão, pela marca da pessoa e pela segunda
/// nacionalidade (+0x1e). Permissões com pessoa negativa não entram nas regiões (o original leria fora do vetor).
pub(crate) unsafe extern "thiscall" fn comparar_update_work_permits(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_UPDATE_WORK_PERMITS);
    let n = (*((this + 0x93e) as *const i32)).max(0) as usize;
    let vetor = read_u32(this + 0x93a) as usize;
    let mut reg = vec![(this + 0x93a, 8), (vetor, n * 0xe)];
    for k in 0..n {
        let id = read_u32(vetor + k * 0xe);
        if (id as i32) < 0 {
            continue;
        }
        let pessoa = staff_record_by_id(id);
        reg.push(((read_u32(this + 0x86e) as usize).wrapping_add((read_u32(pessoa) as usize).wrapping_mul(8)), 1));
        reg.push((pessoa + staff::SECOND_NATION, 4));
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this), || update_work_permits(this));
    PLACAR.registrar(igual, || format!("update_work_permits {:x?} x {:x?}", mo, mr));
}

/// Comparer for `past_transfer_copy_construct` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_past_transfer_copy_construct(this: usize, origem: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_PAST_TRANSFER_COPY_CONSTRUCT);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x25)], || o(this, origem), || past_transfer_copy_construct(this, origem));
    PLACAR.registrar(igual && r == n, || format!("past_transfer_copy_construct {:x?} x {:x?}", mo, mr));
    r
}

/// Comparer for `future_transfer_assign` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_future_transfer_assign(this: usize, origem: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_FUTURE_TRANSFER_ASSIGN);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0xc)], || o(this, origem), || future_transfer_assign(this, origem));
    PLACAR.registrar(igual && r == n, || format!("future_transfer_assign {:x?} x {:x?}", mo, mr));
    r
}

/// Comparer for `can_sign_player` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_can_sign_player(gerente: usize, clube: usize, a2: u32, jogador: usize, a4: u32, a5: u32, saida: *mut u16) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, u32, *mut u16) -> u32>(IDX_CAN_SIGN_PLAYER);
    let mut regioes = vec![];
    if !saida.is_null() {
        regioes.push((saida as usize, 2));
    }
    if jogador != 0 {
        regioes.push((jogador + 0x1a, 4));
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&regioes, || o(gerente, clube, a2, jogador, a4, a5, saida),
        || can_sign_player(gerente, clube, a2, jogador, a4, a5, saida));
    PLACAR.registrar(igual && r == n, || format!("can_sign_player(a5 {}) original={} rust={} {:x?} x {:x?}", a5, r, n, mo, mr));
    r
}

/// Comparer for `get_exchange_offers_for_player` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_get_exchange_offers_for_player(gerente: usize, jogador: usize, saida: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_GET_EXCHANGE_OFFERS_FOR_PLAYER);
    if saida != 0 {
        // simplification: com lista de saída a original anexa (aloca); só a original
        return o(gerente, jogador, saida);
    }
    let mut regioes = vec![(G_CONTRACT_LOOKUP_RESULT, 4usize)];
    if jogador != 0 && *(jogador as *const i32) < *(G_STAFF_COUNT as *const i32) {
        let guardado = *(G_CONTRACT_LOOKUP_RESULT as *const usize);
        let p = transfer_manager_contract_club(jogador);
        *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = guardado;
        if p != 0 {
            // os cursores das listas de troca das ofertas do clube (as que a rotina pode percorrer)
            let clube = *(p as *const u32);
            for i in 0..(*((gerente + 4) as *const i32)).max(0) as usize {
                let oferta = *((*(gerente as *const usize) + 4 * i) as *const usize);
                if oferta != 0 && *((oferta + 8) as *const u32) == clube && *((oferta + 4) as *const usize) != 0 {
                    regioes.push((*((oferta + 4) as *const usize) + 0xa, 4));
                }
            }
        }
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(gerente, jogador, saida),
        || get_exchange_offers_for_player(gerente, jogador, saida));
    PLACAR.registrar(igual && r == n, || format!("get_exchange_offers_for_player original={} rust={} {:x?} x {:x?}", r, n, mo, mr));
    r
}

/// Comparer for `transfer_manager_close_offer_contract_offers` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "stdcall" fn comparar_transfer_manager_close_offer_contract_offers(oferta: usize) {
    let o = original_fn::<unsafe extern "stdcall" fn(usize)>(IDX_TRANSFER_MANAGER_CLOSE_OFFER_CONTRACT_OFFERS);
    // as propostas que podem fechar: as do id do alvo e dos ids da troca (o clube não filtra aqui)
    let mut ids = vec![*((oferta + 0xc) as *const u32)];
    let l = *((oferta + 4) as *const usize);
    if l != 0 {
        ids.extend(llist_data_values(l).iter().map(|&e| *(e as *const u32)));
    }
    let propostas = *((*(G_CONTRACT_MANAGER as *const usize) + 0x34) as *const usize);
    let mut regioes = vec![(G_CONTRACT_LOOKUP_RESULT, 4usize)];
    if l != 0 {
        regioes.push((l + 0xa, 4));
    }
    if propostas != 0 {
        regioes.push((propostas + 0xa, 4));
        for r in llist_data_values(propostas) {
            if ids.contains(&*(r as *const u32)) {
                regioes.push((r + 0x50, 1));
            }
        }
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(oferta), || transfer_manager_close_offer_contract_offers(oferta));
    PLACAR.registrar(igual, || format!("transfer_manager_close_offer_contract_offers(id {}) {:x?} x {:x?}", ids[0], mo, mr));
}

/// Comparer for `queue_free_transfer` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_queue_free_transfer(gerente: usize, pessoa: usize, motivo: u8, compensacao: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u8, u32)>(IDX_QUEUE_FREE_TRANSFER);
    // simplification: a fila aloca um nó (não roda duas vezes): a cópia só monta o registro (0x4cf6d0 é
    // consulta pura), a original põe na fila e confere-se o nó novo da cauda
    let mut registro = None;
    if !queue_free_transfer_with(gerente, pessoa, motivo, compensacao, &mut |_, r| registro = Some(*r)) {
        return o(gerente, pessoa, motivo, compensacao);
    }
    let l = gerente + 0x92c;
    let antes = core::ptr::read_unaligned(l as *const u16);
    o(gerente, pessoa, motivo, compensacao);
    let depois = core::ptr::read_unaligned(l as *const u16);
    let cauda = core::ptr::read_unaligned((l + 6) as *const usize);
    let novo = if cauda == 0 { None } else { Some(*(*(cauda as *const usize) as *const [u8; 9])) };
    PLACAR.registrar(depois == antes.wrapping_add(1) && novo == registro,
        || format!("queue_free_transfer {} -> {} original={:x?} rust={:x?}", antes, depois, novo, registro));
}

/// Comparer for `get_exchange_negotiations_for_player` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_get_exchange_negotiations_for_player(gerente: usize, pessoa: usize, saida: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> i32>(IDX_GET_EXCHANGE_NEGOTIATIONS_FOR_PLAYER);
    if saida != 0 {
        // simplification: com lista de saída (só a tela 0x47ec79) a original anexa (aloca); só a original
        return o(gerente, pessoa, saida);
    }
    let mut regioes = vec![(G_CONTRACT_LOOKUP_RESULT, 4usize)];
    if pessoa != 0 && *(pessoa as *const i32) < *(G_STAFF_COUNT as *const i32) {
        let guardado = *(G_CONTRACT_LOOKUP_RESULT as *const usize);
        let p = transfer_manager_contract_club(pessoa);
        *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = guardado;
        if p != 0 {
            // os cursores das listas de troca das negociações do clube (as que a rotina pode percorrer)
            let clube = *(p as *const u32);
            let negociacoes = *((gerente + 0xc) as *const usize);
            for i in 0..(*((gerente + 0x10) as *const i32)).max(0) as usize {
                let neg = *((negociacoes + i * 4) as *const usize);
                if neg != 0 && *((neg + 8) as *const u32) == clube && *((neg + 4) as *const usize) != 0 {
                    regioes.push((*((neg + 4) as *const usize) + 0xa, 4));
                }
            }
        }
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&regioes, || o(gerente, pessoa, saida),
        || get_exchange_negotiations_for_player(gerente, pessoa, saida));
    PLACAR.registrar(igual && r == n, || format!("get_exchange_negotiations_for_player original={} rust={} {:x?} x {:x?}", r, n, mo, mr));
    r
}

/// Comparer for `get_offers_for_player` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_get_offers_for_player(this: usize, pessoa: *const i32, lista: usize, todas: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *const i32, usize, u32) -> u32>(IDX_GET_OFFERS_FOR_PLAYER);
    // simplification: com `lista` a original aloca nós (0x68d0d0) e não pode rodar duas vezes; só ela
    if lista != 0 || pessoa.is_null() {
        return o(this, pessoa, lista, todas);
    }
    // a original só mexe no cursor da lista da oferta achada
    let e = offer_for_player(this, *pessoa);
    let l = if e == 0 { 0 } else { *(e as *const usize) };
    let reg = if l == 0 { vec![] } else { vec![(l, 0xe)] };
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, pessoa, lista, todas), || get_offers_for_player(this, pessoa, lista, todas));
    PLACAR.registrar(igual && r == n, || format!("get_offers_for_player({}, todas {}) original={} rust={} {:x?} x {:x?}", *pessoa, todas, r, n, mo, mr));
    r
}

/// Comparer for `lista_de_pagamentos` (comparison mode: see the header of motor.rs).
pub(crate) unsafe fn comparar_lista_de_pagamentos(nome: &str, k: usize, rust: unsafe extern "thiscall" fn(usize, *const i32), gerente: usize, id: *const i32, lista: usize, quantos: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *const i32)>(k);
    let reg = payment_list_regions(gerente, id, lista, quantos);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(gerente, id), || rust(gerente, id));
    PLACAR.registrar(igual, || format!("{}({}) {:x?} x {:x?}", nome, *id, mo, mr));
}

/// Comparer for `remove_next_sale_percentage` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_remove_next_sale_percentage(gerente: usize, id: *const i32) {
    comparar_lista_de_pagamentos("remove_next_sale_percentage", 451, remove_next_sale_percentage, gerente, id, 0x880, 0x888);
}

/// Comparer for `remove_first_team_appearance_money` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_remove_first_team_appearance_money(gerente: usize, id: *const i32) {
    comparar_lista_de_pagamentos("remove_first_team_appearance_money", 452, remove_first_team_appearance_money, gerente, id, 0x88c, 0x894);
}

/// Comparer for `remove_international_team_appearance_money` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_remove_international_team_appearance_money(gerente: usize, id: *const i32) {
    comparar_lista_de_pagamentos("remove_international_team_appearance_money", 453, remove_international_team_appearance_money, gerente, id, 0x898, 0x8a0);
}

/// Comparer for `get_agreed_contract_club` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_get_agreed_contract_club(gerente: usize, id: *const i32, clube: *mut usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *const i32, *mut usize) -> u32>(IDX_GET_AGREED_CONTRACT_CLUB);
    let mut reg = vec![(staff_flags_entry(gerente, id), 1)];
    if !clube.is_null() {
        reg.push((clube as usize, 4));
    }
    let (ro, rr, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(gerente, id, clube), || get_agreed_contract_club(gerente, id, clube));
    PLACAR.registrar(igual && ro == rr, || format!("get_agreed_contract_club({}) original={} rust={} {:x?} x {:x?}", *id, ro, rr, mo, mr));
    ro
}

/// Comparer for `record_international_appearance` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_record_international_appearance(gerente: usize, id: *const i32, numero: i16) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *const i32, i16)>(IDX_RECORD_INTERNATIONAL_APPEARANCE);
    let reg = [(staff_flags_entry(gerente, id) + 3, 4)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(gerente, id, numero), || record_international_appearance(gerente, id, numero));
    PLACAR.registrar(igual, || format!("record_international_appearance({}, {}) {:x?} x {:x?}", *id, numero, mo, mr));
}

/// Comparer for `set_agreed_contract_flag` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_set_agreed_contract_flag(gerente: usize, id: *const i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *const i32)>(IDX_SET_AGREED_CONTRACT_FLAG);
    let a = staff_flags_ptr(gerente, id) as usize;
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(a, 1)], || o(gerente, id), || set_agreed_contract_flag(gerente, id));
    PLACAR.registrar(igual, || format!("set_agreed_contract_flag({}) {:x?} x {:x?}", *id, mo, mr));
}

/// Comparer for `clear_agreed_contract_flag` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_clear_agreed_contract_flag(gerente: usize, id: *const i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *const i32)>(IDX_CLEAR_AGREED_CONTRACT_FLAG);
    let a = staff_flags_ptr(gerente, id) as usize;
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(a, 1)], || o(gerente, id), || clear_agreed_contract_flag(gerente, id));
    PLACAR.registrar(igual, || format!("clear_agreed_contract_flag({}) {:x?} x {:x?}", *id, mo, mr));
}

/// Comparer for `offer_slot` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_offer_slot(tabela: usize, k: i32) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, i32) -> usize>(IDX_OFFER_SLOT)(tabela, k);
    let n = offer_slot(tabela, k);
    PLACAR.registrar(r == n, || format!("offer_slot({}) original={:#x} rust={:#x}", k, r, n));
    r
}

/// Comparer for `get_selection_limit` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelos 6 bytes da saída (a 0x8cf6b0 só escreve na tabela local).
pub(crate) unsafe extern "thiscall" fn comparar_get_selection_limit(this: usize, partida: usize, saida: *mut u8) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut u8)>(IDX_GET_SELECTION_LIMIT);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(saida as usize, 6)], || o(this, partida, saida), || get_selection_limit(this, partida, saida));
    PLACAR.registrar(igual, || format!("get_selection_limit({:x}) {:x?} x {:x?}", partida, mo, mr));
}

/// Comparer for `find_work_permit` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor (a busca só lê).
pub(crate) unsafe extern "thiscall" fn comparar_find_work_permit(this: usize, pessoa: *const i32) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, *const i32) -> usize>(IDX_FIND_WORK_PERMIT)(this, pessoa);
    let n = find_work_permit(this, pessoa);
    PLACAR.registrar(r == n, || format!("find_work_permit({:x}) original={:#x} rust={:#x}", pessoa as usize, r, n));
    r
}

/// Comparer for `fill_vacated_staff_job` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a escolha da cópia antes; sem ninguém, só a original. Com a pessoa, original e cópia lado a lado no
/// clube (0x245 bytes) e na pessoa (0x6e), que é tudo o que a 0x53bd10 grava: ela pode recusar (casa ocupada, faixa
/// dos humanos; com mais ligas acontece) ou gravar na seleção (+0x24), e a pessoa não fica em +0x39.
pub(crate) unsafe extern "stdcall" fn comparar_fill_vacated_staff_job(clube: usize, cargo: u32) {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, u32)>(IDX_FILL_VACATED_STAFF_JOB);
    let escolhida = if clube != 0 { job_sought_by_slot(cargo).and_then(|c| first_unemployed_staff(clube, c as u8)) } else { None };
    let Some(p) = escolhida else {
        crate::comparacao::ramo("fill_vacated_staff_job sem ninguem");
        return o(clube, cargo);
    };
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(clube, 0x245), (p, 0x6e)], || o(clube, cargo),
        || fill_vacated_staff_job(clube, cargo));
    if read_usize(p + 0x39) != clube {
        crate::comparacao::ramo("fill_vacated_staff_job sem entrar no clube");
    }
    PLACAR.registrar(igual, || format!("fill_vacated_staff_job({:x}, {}) pessoa {:x} {:x?} x {:x?}", clube, cargo as u8, p, mo, mr));
}

/// Comparer for `person_in_transfer_list` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_person_in_transfer_list(t: usize, p: *const i32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *const i32) -> u32>(IDX_PERSON_IN_TRANSFER_LIST);
    let reg = [(t + 0x92c + 0xa, 4)];
    let (r, v, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(t, p), || person_in_transfer_list(t, p));
    PLACAR.registrar(igual && r == v, || format!("person_in_transfer_list({:#x}, {:p}) original={} {:x?} rust={} {:x?}", t, p, r, mo, v, mr));
    r
}

/// Comparer for `handle_player_rejected_terms` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_handle_player_rejected_terms(this: usize, pessoa: usize, indice: i32, silencioso: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32)>(IDX_HANDLE_PLAYER_REJECTED_TERMS)(this, pessoa, indice, silencioso)
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
pub(crate) unsafe extern "thiscall" fn comparar_schedule_offer(this: usize, oferta: usize, dias: u32, fatias: u32) -> u32 {
    let sem_base = read_i16(this + 0x84f) == 0;
    let (data, fatia, balde) = vacancy_schedule_plan(this, dias as u8, fatias as u8);
    let conta = read_i16(balde);
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32>(IDX_SCHEDULE_OFFER)(this, oferta, dias, fatias);
    let gravada = core::slice::from_raw_parts((oferta + 0x12) as *const u8, 8);
    let base_ok = !sem_base || core::slice::from_raw_parts((this + 0x84d) as *const u8, 8) == core::slice::from_raw_parts(G_CURRENT_DATE as *const u8, 8);
    let ok = gravada == &data[..] && *((oferta + 0x1a) as *const i8) == fatia && read_i16(balde) == conta.wrapping_add(1) && base_ok;
    PLACAR.registrar(ok, || format!("schedule_offer(dias {}, fatias {}): data {:x?} (plano {:x?}), fatia {} (plano {}), balde {:x} conta {} -> {}, base {}",
        dias as u8 as i8, fatias as u8 as i8, gravada, data, *((oferta + 0x1a) as *const i8), fatia, balde, conta, read_i16(balde), base_ok));
    r
}

/// Comparer for `find_highest_offer_for_player` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor (a 0x8e7a70 só calcula; as duas voltam o cursor da lista ao começo).
pub(crate) unsafe extern "thiscall" fn comparar_find_highest_offer_for_player(this: usize, jogador: *const u32, clube: *const u32) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, *const u32, *const u32) -> usize>(IDX_FIND_HIGHEST_OFFER_FOR_PLAYER)(this, jogador, clube);
    let n = find_highest_offer_for_player(this, jogador, clube);
    PLACAR.registrar(r == n, || format!("find_highest_offer_for_player({}, {}) original={:#x} rust={:#x}", *jogador, *clube, r, n));
    r
}

/// Comparer for `find_international_appearance_money` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x8d0f00: lado a lado na marca da pessoa e na devolução.
pub(crate) unsafe extern "thiscall" fn comparar_find_international_appearance_money(this: usize, p: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_FIND_INTERNATIONAL_APPEARANCE_MONEY);
    let m = read_usize(this + 0x86e).wrapping_add(read_usize(p).wrapping_mul(8));
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(m, 1)], || o(this, p), || find_international_appearance_money(this, p));
    PLACAR.registrar(igual && r == n, || format!("find_international_appearance_money(pessoa {:x}) original={:x} rust={:x} {:x?} x {:x?}", p, r, n, mo, mr));
    r
}

/// Comparer for `pay_appearance_money` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: dá notícia e mexe nas finanças: roda só a original e confere a devolução com o plano (contrato, clube,
/// registro do prêmio e jogos, sem efeitos).
pub(crate) unsafe extern "thiscall" fn comparar_pay_appearance_money(this: usize, jogador: usize, jogos: i32, tipo: u32, arg: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32, u32) -> u32>(IDX_PAY_APPEARANCE_MONEY);
    let c = get_contract(jogador);
    let esperado = if c == 0 || friendly_club_by_id(read_usize(c + 4)) == 0 {
        0
    } else {
        let r = appearance_bonus_record(this, jogador, tipo);
        (r != 0 && read_u8(r + 8) as i8 as i32 == jogos && jogos > 0) as u32
    };
    let r = o(this, jogador, jogos, tipo, arg);
    PLACAR.registrar(r == esperado, || format!("pay_appearance_money(jogador {:x}, jogos {}, tipo {}) devolveu {}, esperava {}", read_usize(jogador), jogos, tipo as u8, r, esperado));
    r
}

/// Comparer for `transfer_manager_get_loan_period` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelas quatro saídas, o sorteio e [0xae1080].
pub(crate) unsafe extern "thiscall" fn comparar_transfer_manager_get_loan_period(this: usize, clube: usize, a2: u32, jogador: usize, ini_mes: *mut i8, ini_ano: *mut u16, mes: *mut i8, ano: *mut u16) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize, *mut i8, *mut u16, *mut i8, *mut u16)>(IDX_TRANSFER_MANAGER_GET_LOAN_PERIOD);
    let mut reg = vec![(ini_mes as usize, 1usize), (ini_ano as usize, 2), (mes as usize, 1), (ano as usize, 2), (G_CONTRACT_LOOKUP_RESULT, 4)];
    reg.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(
        &reg,
        || o(this, clube, a2, jogador, ini_mes, ini_ano, mes, ano),
        || crate::aleatorio::sem_contar(|| transfer_manager_get_loan_period(this, clube, a2, jogador, ini_mes, ini_ano, mes, ano)),
    );
    PLACAR.registrar(igual, || format!("transfer_manager_get_loan_period({:#x}, {:#x}) {:x?} x {:x?}", clube, jogador, mo, mr));
}

/// Comparer for `update_transfer_window_news` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x8c6bc0: só a original (ela só grava nas locais; os avisos das regras alocam notícias e as ofertas
/// em espera mudam de status e são reagendadas).
pub(crate) unsafe extern "thiscall" fn comparar_update_transfer_window_news(this: usize, hoje: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_UPDATE_TRANSFER_WINDOW_NEWS)(this, hoje) // simplification: nada próprio a conferir
}

/// Comparer for `note_terms_offered_to_main_player` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_note_terms_offered_to_main_player(gerente: usize, k: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32)>(IDX_NOTE_TERMS_OFFERED_TO_MAIN_PLAYER);
    if k < 0 || k >= *((gerente + 4) as *const i32) {
        return o(gerente, k);
    }
    let oferta = *((*(gerente as *const usize) as isize + k as isize * 4) as *const usize);
    if oferta == 0 {
        return o(gerente, k);
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(oferta + 0x2c, 1)], || o(gerente, k), || note_terms_offered_to_main_player(gerente, k));
    PLACAR.registrar(igual, || format!("note_terms_offered_to_main_player({}) {:x?} x {:x?}", k, mo, mr));
}
