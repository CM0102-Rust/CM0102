use super::*;

/// Person -> club of their contract (get_contract, result stored in G_CONTRACT_LOOKUP_RESULT); 0 with no contract or a
/// type-5 contract (+0x35 & 0x3f).
///
/// [pt] O clube do contrato da pessoa (0x4cf5a0: push pessoa; mov ecx, [0xae1050] -> thiscall, ret 4; guardado em
/// [0xae1080]); sem contrato ou com o de tipo 5, 0.
pub(crate) unsafe fn contract_club_excluding_type5(person: usize) -> usize {
    let contract: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = contract(read_u32_as_usize(G_CONTRACT_MANAGER), person);
    write_u32_from_usize(G_CONTRACT_LOOKUP_RESULT, c);
    if c == 0 || read_u8(c + contract::CONTRACT_TYPE) & 0x3f == 5 { 0 } else { transfer_manager_club_by_id(read_u32_as_usize(c + contract::CLUB)) }
}

/// Finds the slot for an offer in pointer vector `v` (capacity `cap`; the slots from `lidas` on count as empty: the
/// vector that has just grown), and the rival offer; returns (slot, rival).
///
/// The slot of `inicial` (the lowest free slot stored at `+0x85e`), otherwise the first empty one; an offer from the
/// same buyer for the same target gives its slot; the first offer from another buyer for the target is the rival. It
/// stops at the empty slot once it has counted the `conta` offers or when it is the first offer for the target.
///
/// [pt] A vaga da oferta no vetor de ponteiros `v` (capacidade `cap`; as vagas de `lidas` em diante contam como vazias: o
/// vetor que acabou de crescer): a vaga de `inicial` (a menor livre guardada em +0x85e), senão a primeira vazia; a
/// oferta do mesmo comprador pelo mesmo alvo dá a vaga dela; a primeira oferta de outro comprador pelo alvo é a rival.
/// Para na vaga vazia quando já contou as `conta` ofertas ou quando é a primeira oferta pelo alvo. Devolve (vaga, rival).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn find_offer_slot_and_rival(v: usize, lidas: i32, cap: i32, count: i32, target: usize, buyer: usize, initial: i32, first: bool) -> (i32, i32) {
    let (mut vaga, mut rival, mut contadas) = (initial, -1i32, 0i32);
    let mut i = 0i32;
    while i < cap {
        let e = if i < lidas { read_u32_as_usize(v + 4 * i as usize) } else { 0 };
        if e != 0 {
            contadas += 1;
            if read_u32_as_usize(e + offer::TARGET_STAFF_ID) == target {
                if read_u32_as_usize(e + offer::BUYER_CLUB_ID) == buyer {
                    crate::comparison::branch("cqf_1 mesma vaga");
                    vaga = i; // cqf1: the offer slot of the same buyer
                } else if rival == -1 {
                    rival = i;
                }
            }
        } else {
            if vaga == -1 {
                vaga = i;
            }
            if contadas == count || first {
                break;
            }
        }
        i += 1;
    }
    (vaga, rival)
}

/// Shows transfer_manager.cpp's "Memory Allocation Error" box with version, file, line and text (sprintf into 0xae24d0,
/// translated title/body, GUI message box).
///
/// [pt] A caixa "Memory Allocation Error" de transfer_manager.cpp (0xa8d514): "v%s %s.%s %d (%s)" (0x90d130 com a linha e
/// o texto) em 0xae24d0, o título traduzido (0x66f4e0) em 0xae28bc, o corpo (0x9a7ff8) em 0xae26c4 e a caixa 0x5e8290.
pub(crate) unsafe fn show_out_of_memory_box(row: u32, text: usize) {
    let split: unsafe extern "C" fn(usize, usize, usize, *mut u8, *mut u8) = core::mem::transmute(FN_CRT_STRTOK);
    let version: unsafe extern "C" fn(*mut u8, *mut u8, u32, usize) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let translate: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let caixa: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(FN_GUI_SHOW_MESSAGE_BOX);
    let (mut nome, mut ext) = ([0u8; 256], [0u8; 256]);
    split(0xa8d514, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
    let s = version(nome.as_mut_ptr(), ext.as_mut_ptr(), row, text);
    format(0xae24d0, 0x9a8120, s, nome.as_ptr(), ext.as_ptr(), row, text);
    translate(0xae28bc, 0x9a8108);
    translate(0xae26c4, 0x9a7ff8, 0xae24d0usize);
    caixa(0xae28bc, 0xae26c4, 0);
}

/// Adds a transfer offer: puts the offer in an array slot (the stored one, the same buyer's for the same target or the
/// first empty one; grows by 50 when full), copies the fields, gives the serial number, links rival offers for the
/// player, schedules the answer (28 days with a human, otherwise the reply deadline) and posts the news.
///
/// Ret 8; SEH frame with no destructor on the normal path (only the `new` of the offer and of the lists):
/// `TRANSFER_MANAGER::add_transfer_offer(offer, terms)`. The lowest stored free slot (+0x85e) is read and set back to
/// −1; an offer with no fee (+0x1b = 0) loses +0x40. Vector full (+8 = +4): grows by 50 (calloc 0x945501 the first
/// time, otherwise realloc 0x945667 with the 50 new ones zeroed; out of memory, the box (line 0x4d2) and the game
/// ends). The slot (`find_offer_slot_and_rival`); none: the "Error" box (line 0x511) and nothing. The slot's old offer
/// is removed (0x8d0f60); a new one (0x944cf1 of 0x41 bytes, 0x8e5680) receives a copy of the fields (the date +0x12
/// through 0x417c10, the low nibble of +0x30 and of +0x31), the slot index (+0x20, also in the incoming offer), +0x28 =
/// −1, +0x2c and +0x10 zeroed, +0x2d 2 becomes 1, and the serial number (+0x24 = `[this+0x85a]`, which goes up; also in
/// the incoming one). With an exchange (+4), a new list (0x68d020) with a copy of the exchange (0x68d2f0). With a
/// rival, the slot joins the rival's list of offers for the player (+0, 0x68d0d0), or a new one with the rival and it.
/// The deadline: with +0x2f, status 7 and 28 days if the buyer is human (0x600a50 with 0, 1, 0, 0), otherwise nothing;
/// without +0x2f, 28 days if the seller (the club of the target's contract) is human, otherwise the reply deadline
/// (0x8cc5b0), scheduled (0x8c6220) if it gives days or slices. The news: loan offered (0x8e1780 with the terms) or
/// offer made (0x8d3090). The target's 8-byte record (`[this+0x86e]`): bit 0 of +0 and +1 goes up; +2 of each exchange
/// player goes up. A human buyer and a target with a player sheet (+0x61): 0x7f75f0 (`[0xb74448]`, target, 0).
/// Original: `TRANSFER_MANAGER::add_transfer_offer` (0x8be300, transfer_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x8be300 } (thiscall, ret 8, transfer_manager.cpp, quadro SEH sem destrutor no caminho normal: só os `new` da
/// oferta e das listas): `TRANSFER_MANAGER::add_transfer_offer(oferta, termos)`. A menor vaga livre guardada (+0x85e)
/// é lida e volta a −1; a oferta sem valor (+0x1b = 0) perde o +0x40. Vetor cheio (+8 = +4): cresce 50 (calloc
/// 0x945501 na primeira vez, senão realloc 0x945667 com as 50 novas zeradas; sem memória, a caixa (linha 0x4d2) e o
/// fim do jogo). A vaga (`find_offer_slot_and_rival`); nenhuma: a caixa "Error" (linha 0x511) e nada. A oferta antiga da vaga sai
/// (0x8d0f60); uma nova (0x944cf1 de 0x41 bytes, 0x8e5680) recebe a cópia dos campos (a data +0x12 pela 0x417c10, o
/// nibble baixo de +0x30 e de +0x31), o índice da vaga (+0x20, também na oferta de entrada), +0x28 = −1, +0x2c e +0x10
/// zerados, +0x2d 2 vira 1, e o número de série (+0x24 = [this+0x85a], que sobe; também na de entrada). Com troca
/// (+4), uma lista nova (0x68d020) com a cópia da troca (0x68d2f0). Com rival, a vaga entra na lista das ofertas pelo
/// jogador (+0, 0x68d0d0) da rival, ou numa nova com a rival e ela. O prazo: com +0x2f, o status 7 e 28 dias se o
/// comprador é humano (0x600a50 com 0, 1, 0, 0), senão nada; sem +0x2f, 28 dias se o vendedor (clube do contrato do
/// alvo) é humano, senão o prazo da resposta (0x8cc5b0), agendado (0x8c6220) se der dias ou fatias. A notícia: a de
/// empréstimo oferecido (0x8e1780 com os termos) ou a da oferta feita (0x8d3090). O registro de 8 bytes do alvo
/// ([this+0x86e]): bit 0 de +0 e +1 sobe; o +2 de cada jogador da troca sobe. Comprador humano e alvo com ficha de
/// jogador (+0x61): 0x7f75f0 ([0xb74448], alvo, 0).
pub(crate) unsafe extern "thiscall" fn add_transfer_offer(this: usize, offer: usize, terms: u32) {
    let calloc: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(FN_CRT_CALLOC);
    let realloc: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_CRT_REALLOC);
    let fechar1: unsafe extern "C" fn() = core::mem::transmute(FN_GAME_FATAL_CLEANUP);
    let fechar2: unsafe extern "C" fn() = core::mem::transmute(FN_SHARED_NOOP_RET0_5A8F60);
    let exit: unsafe extern "C" fn(i32) -> ! = core::mem::transmute(FN_CRT_EXIT);
    let remove: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_REMOVE_OFFER);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let new_offer: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_TRANSFER_OFFER_TRANSFER_OFFER);
    let copy_data: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let list_new: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let copy_list: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_LLIST_COPY_FROM);
    let append: unsafe extern "thiscall" fn(usize, *const i32, u32) -> u32 = core::mem::transmute(FN_LLIST_ADD_TAIL);
    let human: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let deadline: unsafe extern "thiscall" fn(usize, usize, *mut u8, *mut u8) = core::mem::transmute(FN_TRANSFER_MANAGER_GET_OFFER_RESPONSE_DELAY);
    let schedule: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_SCHEDULE_OFFER);
    let news_loan: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_POST_LOAN_OFFER_NEWS);
    let news_offer: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_POST_OFFER_MADE_NEWS);
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let observar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_SCOUT_MANAGER_ADD_TO_SHORTLIST);

    let target = read_u32_as_usize(offer + offer::TARGET_STAFF_ID);
    let buyer = read_u32_as_usize(offer + offer::BUYER_CLUB_ID);
    let initial = read_u32_as_usize(this + transfer_manager_fields::LOWEST_FREE_SLOT) as i32;
    write_u32_from_usize(this + transfer_manager_fields::LOWEST_FREE_SLOT, u32::MAX as usize);
    let first = read_u8(read_u32_as_usize(this + transfer_manager_fields::PERSON_FLAGS).wrapping_add(target.wrapping_mul(8)) + 1) == 0;
    if read_u32_as_usize(offer + offer::FEE) == 0 {
        write_u8(offer + offer::INSTALMENTS, 0);
    }
    if read_u32_as_usize(this + transfer_manager_fields::OFFER_COUNT) == read_u32_as_usize(this + transfer_manager_fields::OFFER_CAPACITY) {
        let cap = (read_u32_as_usize(this + transfer_manager_fields::OFFER_CAPACITY) as u32).wrapping_add(0x32);
        write_u32_from_usize(this + transfer_manager_fields::OFFER_CAPACITY, cap as usize);
        if read_u32_as_usize(this) == 0 {
            write_u32_from_usize(this, calloc(cap, 4));
        } else {
            let p = realloc(read_u32_as_usize(this), cap.wrapping_mul(4));
            write_u32_from_usize(this, p);
            if p == 0 {
                show_out_of_memory_box(0x4d2, 0xa8e940);
                *(G_MOUSE_LEFT_BUTTON_DOWN as *mut u32) = 0;
                fechar1();
                fechar2();
                exit(-1);
            }
            core::ptr::write_bytes((p + 4 * cap as usize - 200) as *mut u32, 0, 0x32);
        }
    }
    let cap = read_u32_as_usize(this + transfer_manager_fields::OFFER_CAPACITY) as i32;
    let (vaga, rival) = find_offer_slot_and_rival(read_u32_as_usize(this), cap, cap, read_u32_as_usize(this + transfer_manager_fields::OFFER_COUNT) as i32, target, buyer, initial, first);
    if vaga == -1 {
        show_error_at(0xa8d514, 0x511);
        return;
    }
    let slot = |k: i32| read_u32_as_usize(this).wrapping_add(4 * k as usize);
    let old = read_u32_as_usize(slot(vaga));
    if old != 0 {
        remove(this, old);
    }
    let mem = novo(0x41);
    let new = if mem != 0 { new_offer(mem) } else { 0 };
    write_u32_from_usize(slot(vaga), new);
    write_u32_from_usize(this + transfer_manager_fields::OFFER_COUNT, read_u32_as_usize(this + transfer_manager_fields::OFFER_COUNT).wrapping_add(1));
    let o = read_u32_as_usize(slot(vaga));
    for k in [0usize, 4, 8, 0xc] {
        write_u32_from_usize(o + k, read_u32_as_usize(offer + k));
    }
    write_u8(o + offer::TERMS_STATE, read_u8(offer + offer::TERMS_STATE));
    write_u8(o + offer::JOB_TYPE, read_u8(offer + offer::JOB_TYPE));
    let mut tmp = [0u8; 8];
    copy_data(o + offer::DUE_DATE, tmp.as_mut_ptr(), offer + offer::DUE_DATE);
    write_u8(o + offer::DUE_SLICE, read_u8(offer + offer::DUE_SLICE));
    write_u32_from_usize(o + offer::FEE, read_u32_as_usize(offer + offer::FEE));
    write_u8(o + offer::OFFER_TYPE, read_u8(offer + offer::OFFER_TYPE));
    for k in [0x20usize, 0x24, 0x28] {
        write_u32_from_usize(o + k, read_u32_as_usize(offer + k));
    }
    for k in [0x2cusize, 0x2d, 0x2e, 0x2f] {
        write_u8(o + k, read_u8(offer + k));
    }
    write_u8(o + offer::COMPLETION_TIMING, (read_u8(offer + offer::COMPLETION_TIMING) & 0xf) | (read_u8(o + offer::COMPLETION_TIMING) & 0xf0));
    write_u32_from_usize(o + offer::OFFER_KIND, (read_u32_as_usize(offer + offer::OFFER_KIND) & 0xf) | (read_u32_as_usize(o + offer::OFFER_KIND) & !0xf));
    write_u32_from_usize(o + offer::APPEARANCE_FEE, read_u32_as_usize(offer + offer::APPEARANCE_FEE));
    write_u8(o + offer::APPEARANCE_GAMES, read_u8(offer + offer::APPEARANCE_GAMES));
    write_u32_from_usize(o + offer::INTERNATIONAL_FEE, read_u32_as_usize(offer + offer::INTERNATIONAL_FEE));
    write_u8(o + offer::INTERNATIONAL_GAMES, read_u8(offer + offer::INTERNATIONAL_GAMES));
    write_u8(o + offer::SELL_ON_PERCENT, read_u8(offer + offer::SELL_ON_PERCENT));
    write_u8(o + offer::INSTALMENTS, read_u8(offer + offer::INSTALMENTS));
    write_u32_from_usize(read_u32_as_usize(slot(vaga)) + 0x20, vaga as u32 as usize);
    write_u32_from_usize(offer + offer::SLOT_INDEX, vaga as u32 as usize);
    write_u32_from_usize(read_u32_as_usize(slot(vaga)) + 0x28, u32::MAX as usize);
    write_u8(read_u32_as_usize(slot(vaga)) + 0x2c, 0);
    write_u8(read_u32_as_usize(slot(vaga)) + 0x10, 0);
    let o = read_u32_as_usize(slot(vaga));
    if read_u8(o + offer::PLAYER_AWARE) == 2 {
        write_u8(o + offer::PLAYER_AWARE, 1);
    }
    write_u32_from_usize(read_u32_as_usize(slot(vaga)) + 0x24, read_u32_as_usize(this + transfer_manager_fields::NEXT_SERIAL));
    write_u32_from_usize(offer + offer::SERIAL, read_u32_as_usize(read_u32_as_usize(slot(vaga)) + 0x24));
    write_u32_from_usize(this + transfer_manager_fields::NEXT_SERIAL, read_u32_as_usize(this + transfer_manager_fields::NEXT_SERIAL).wrapping_add(1));
    if read_u32_as_usize(offer + offer::EXCHANGE_PLAYERS) != 0 {
        let mem = novo(0xe);
        let l = if mem != 0 { list_new(mem) } else { 0 };
        write_u32_from_usize(read_u32_as_usize(slot(vaga)) + 4, l);
        copy_list(read_u32_as_usize(read_u32_as_usize(slot(vaga)) + 4), read_u32_as_usize(offer + offer::EXCHANGE_PLAYERS));
    }
    if rival != -1 { // cqf1: the rival links the offers
        // the lists copy the 4 bytes of the index (the original passes the address of the stack variables)
        let (vaga_l, rival_l) = (vaga, rival);
        crate::comparison::branch("cqf_1 ofertas rivais ligadas");
        let list = read_u32_as_usize(read_u32_as_usize(slot(rival)));
        if list != 0 {
            append(list, &vaga_l, 4); // cqf1: the slot in the list of the rival
            write_u32_from_usize(read_u32_as_usize(slot(vaga)), read_u32_as_usize(read_u32_as_usize(slot(rival))));
        } else {
            let mem = novo(0xe);
            let l = if mem != 0 { list_new(mem) } else { 0 };
            append(l, &rival_l, 4);
            append(l, &vaga_l, 4);
            write_u32_from_usize(read_u32_as_usize(slot(rival)), l);
            write_u32_from_usize(read_u32_as_usize(slot(vaga)), l);
        }
    }
    let seller = contract_club_excluding_type5(transfer_manager_staff_by_id(target));
    // simplification: the two dwords the original pushes to 0x8c6220 have only the low byte written (the rest is leftover from the
    // stack); 0x8c6220 reads the bytes (days as u8, slices as u8)
    let (mut dias, mut fatias) = (0u8, 0u8);
    let agenda = if read_u8(offer + offer::PRE_CONTRACT) != 0 {
        let h = human(transfer_manager_club_by_id(buyer), 0, 1, 0, 0);
        write_u8(read_u32_as_usize(slot(vaga)) + 0x2c, 7);
        if h != 0 {
            dias = 0x1c;
            true
        } else {
            false
        }
    } else if seller != 0 && human(seller, 0, 1, 0, 0) != 0 {
        dias = 0x1c;
        true
    } else {
        deadline(this, read_u32_as_usize(slot(vaga)), &mut dias, &mut fatias);
        dias != 0 || fatias != 0
    };
    if agenda {
        crate::comparison::branch("cqf_1 agenda pelo prazo");
        schedule(this, read_u32_as_usize(slot(vaga)), dias as u32, fatias as u32); // cqf1: the offer deadline
    }
    let o = read_u32_as_usize(slot(vaga));
    if read_u8(o + offer::OFFER_TYPE) == 1 {
        news_loan(this, o, terms);
    } else {
        news_offer(this, o);
    }
    let reg = read_u32_as_usize(this + transfer_manager_fields::PERSON_FLAGS).wrapping_add(read_u32_as_usize(offer + offer::TARGET_STAFF_ID).wrapping_mul(8));
    write_u8(reg, read_u8(reg) | 1);
    let reg = read_u32_as_usize(this + transfer_manager_fields::PERSON_FLAGS).wrapping_add(read_u32_as_usize(offer + offer::TARGET_STAFF_ID).wrapping_mul(8));
    write_u8(reg + person_transfer_record::OFFER_COUNT, read_u8(reg + person_transfer_record::OFFER_COUNT).wrapping_add(1));
    let swap = read_u32_as_usize(offer + offer::EXCHANGE_PLAYERS);
    if swap != 0 {
        write_u32_from_usize(swap + llist::CURSOR, read_u32_as_usize(swap + llist::HEAD));
        let mut n = next(read_u32_as_usize(offer + offer::EXCHANGE_PLAYERS));
        while n != 0 {
            let r = read_u32_as_usize(this + transfer_manager_fields::PERSON_FLAGS).wrapping_add(read_u32_as_usize(n).wrapping_mul(8)) + 2;
            write_u8(r, read_u8(r).wrapping_add(1));
            n = next(read_u32_as_usize(offer + offer::EXCHANGE_PLAYERS));
        }
    }
    if human(transfer_manager_club_by_id(read_u32_as_usize(offer + offer::BUYER_CLUB_ID)), 0, 1, 0, 0) != 0 {
        let p = transfer_manager_staff_by_id(read_u32_as_usize(offer + offer::TARGET_STAFF_ID));
        if read_u32_as_usize(p + 0x61) != 0 {
            observar(read_u32_as_usize(G_SCOUT_MANAGER), p, 0);
        }
    }
}

/// Offer status (+0x2c) is 2, 3, 4, 0x10, 0x14, or 0xf with +0x2e >= 3 (the test that lets the under-offer mark be
/// cleared).
///
/// [pt] O status da oferta deixa seguir (0x8c0660): 2, 3, 0x14, 0x10, 4, ou 0xf com +0x2e ≥ 3.
pub(crate) unsafe fn is_offer_settled(e: usize) -> bool {
    match read_u8(e + offer::STATUS) {
        2 | 3 | 0x14 | 0x10 | 4 => true,
        0xf => read_u8(e + offer::RESPONSE_ROUND) >= 3,
        _ => false,
    }
}

/// Screens only: the club accepts the loan request; the contract decision, the news, and without agreement status 0xf,
/// the shortlists, the 10-day scheduling and bit 0 of the player record.
///
/// Ret 8: `TRANSFER_MANAGER::accept_loan_offer(slot, mark)`, only from the transfer screen (0x8ef1e0) and the news
/// (0x784100). With `mark`, +0x2d = 1. The contract decision (0x4cc930 on `[0xae1050]`), status 8 and the
/// offer-accepted news (0x8d76d0); decision 0: nothing more. Otherwise the refusal-to-move news (0x8d8310 with 0),
/// status 0xf with +0x2e = 3, the player leaves the buyer's scouting lists (0x84aeb0) and each exchange player those of
/// the seller (the club of the player's contract), and the offer is scheduled for 10 days (0x8c6220). If it and all
/// offers for the player (+0) are settled (`is_offer_settled`), bit 0 of the player's 8-byte record (`[this+0x86e]`) is
/// cleared.
/// Original: `TRANSFER_MANAGER::accept_loan_offer` (0x8c0660, transfer_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x8c0660 } (thiscall, ret 8, transfer_manager.cpp): `TRANSFER_MANAGER::accept_loan_offer(vaga, marcar)`, só pela
/// tela de transferências (0x8ef1e0) e pela notícia (0x784100). Com `marcar`, +0x2d = 1. A decisão do contrato
/// (0x4cc930 em [0xae1050]), o status 8 e a notícia da oferta aceita (0x8d76d0); a decisão 0: nada mais. Senão a
/// notícia da recusa em se mudar (0x8d8310 com 0), o status 0xf com +0x2e = 3, o jogador sai das listas de observação
/// do comprador (0x84aeb0) e cada jogador da troca das do vendedor (o clube do contrato do jogador), e a oferta é
/// agendada para 10 dias (0x8c6220). Se ela e todas as ofertas pelo jogador (+0) seguem (`is_offer_settled`), o bit 0 do
/// registro de 8 bytes do jogador ([this+0x86e]) apaga.
pub(crate) unsafe extern "thiscall" fn accept_loan_offer(this: usize, slot: *mut usize, mark: u32) {
    let decide: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_CONTRACT_MANAGER_SCHEDULE_DECISION_FOR_TRANSFER);
    let news_accepted: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_POST_OFFER_ACCEPTED_NEWS);
    let news_refusal: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_POST_PLAYER_REJECTS_MOVE_NEWS);
    let contract: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let unmark: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_SHORTLIST_MANAGER_CLEAR_BID_MARKERS_FOR_PLAYER);
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let schedule: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_SCHEDULE_OFFER);
    crate::comparison::branch("cqf_1 aceitar empréstimo");
    if mark != 0 {
        write_u8(*slot + 0x2d, 1);
    }
    let d = decide(read_u32_as_usize(G_CONTRACT_MANAGER), *slot);
    write_u8(*slot + 0x2c, 8);
    news_accepted(this, *slot);
    if d == 0 {
        return;
    }
    news_refusal(this, *slot, 0);
    write_u8(*slot + 0x2c, 0xf);
    write_u8(*slot + 0x2e, 3);
    let o = *slot;
    let player = transfer_manager_staff_by_id(read_u32_as_usize(o + offer::TARGET_STAFF_ID));
    let c = contract(read_u32_as_usize(G_CONTRACT_MANAGER), player);
    write_u32_from_usize(G_CONTRACT_LOOKUP_RESULT, c);
    let seller = if c != 0 { transfer_manager_club_by_id(read_u32_as_usize(c + contract::CLUB)) } else { 0 };
    unmark(read_u32_as_usize(this + transfer_manager_fields::SHORTLIST_MANAGER), transfer_manager_club_by_id(read_u32_as_usize(o + offer::BUYER_CLUB_ID)), player);
    let swap = read_u32_as_usize(o + offer::EXCHANGE_PLAYERS);
    if swap != 0 {
        write_u32_from_usize(swap + llist::CURSOR, read_u32_as_usize(swap + llist::HEAD));
        let mut n = next(read_u32_as_usize(o + offer::EXCHANGE_PLAYERS));
        while n != 0 {
            unmark(read_u32_as_usize(this + transfer_manager_fields::SHORTLIST_MANAGER), seller, transfer_manager_staff_by_id(read_u32_as_usize(n)));
            n = next(read_u32_as_usize(o + offer::EXCHANGE_PLAYERS));
        }
    }
    schedule(this, *slot, 10, 0);
    let o = *slot;
    if !is_offer_settled(o) {
        return;
    }
    let list = read_u32_as_usize(o);
    if list != 0 {
        write_u32_from_usize(list + llist::CURSOR, read_u32_as_usize(list + llist::HEAD));
        let mut n = next(read_u32_as_usize(o));
        while n != 0 {
            if !is_offer_settled(read_u32_as_usize(read_u32_as_usize(this).wrapping_add(4 * read_u32_as_usize(n)))) {
                return;
            }
            n = next(read_u32_as_usize(o));
        }
    }
    let r = read_u32_as_usize(this + transfer_manager_fields::PERSON_FLAGS).wrapping_add(read_u32_as_usize(o + offer::TARGET_STAFF_ID).wrapping_mul(8));
    if read_u8(r) & 1 != 0 {
        write_u8(r, read_u8(r) & 0xfe);
    }
}

/// If the offer and every other offer for the same player (its list at +0) are settled, clears bit 0 of the player's
/// 8-byte flags record in [this+0x86e].
///
/// [pt] O bit 0 do registro de 8 bytes do jogador da oferta ([this+0x86e]) apaga quando ela e todas as ofertas pelo jogador
/// (a lista +0) seguem (`is_offer_settled`).
pub(crate) unsafe fn clear_under_offer_mark(this: usize, offer: usize) {
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    if !is_offer_settled(offer) {
        return;
    }
    let list = read_u32_as_usize(offer);
    if list != 0 {
        write_u32_from_usize(list + llist::CURSOR, read_u32_as_usize(list + llist::HEAD));
        let mut n = next(read_u32_as_usize(offer));
        while n != 0 {
            if !is_offer_settled(read_u32_as_usize(read_u32_as_usize(this).wrapping_add(4 * read_u32_as_usize(n)))) {
                return;
            }
            n = next(read_u32_as_usize(offer));
        }
    }
    let r = read_u32_as_usize(this + transfer_manager_fields::PERSON_FLAGS).wrapping_add(read_u32_as_usize(offer + offer::TARGET_STAFF_ID).wrapping_mul(8));
    if read_u8(r) & 1 != 0 {
        write_u8(r, read_u8(r) & 0xfe);
    }
}

/// Finds the nation's transfer rules among the 28 objects at [this+0x8ac] (matching +0xa); None if none.
///
/// [pt] A regra de transferência da nação (os 28 objetos em [this+0x8ac], o +0xa de cada um comparado com `nacao`; a
/// original lê o +0xa sem conferir o ponteiro); None sem nenhuma.
pub(crate) unsafe fn nation_transfer_rules_opt(this: usize, nation: usize) -> Option<usize> {
    for k in 0..0x1c {
        let p = this + transfer_manager_fields::TRANSFER_RULES + 4 * k;
        if read_u32_as_usize(read_u32_as_usize(p) + 0xa) == nation {
            return Some(read_u32_as_usize(p));
        }
    }
    None
}

/// Handles a player accepting terms: marks the agreement of the target (or the exchange player); outside loans, the
/// reply news and the offer follow-up; for loans, a squad above 50 or the nation rule refuse, otherwise completes at
/// once (contract, history, numbers, record, status 0x14 in 5 days).
///
/// Ret 0x14; SEH frame for the local date, without a destructor:
/// `TRANSFER_MANAGER::handle_player_accepted_terms(person, slot, contract, type, news)`, from the acceptance of a
/// contract (0x4cf413). Slot outside the vector: the "Error" box (0xe0c, 0xe11) and nothing; empty slot: nothing. +0x2e
/// is zeroed. The offer's target accepted: +0x10 = 2 and +0x11 = type; A = the buyer, B = the club of the target's
/// contract (type 5 does not count). Otherwise the person is an exchange player: his node in the exchange (+4) gets 2
/// and the type (+4, +6); A = the club of the target's contract, B = the buyer (no exchange: box 0xe4d; without the
/// node, nothing found: A and B the original leaves without a value, the copy uses 0). Outside a loan (+0x1f ≠ 1): the
/// reply news (0x8d8e90 with A, B, 2 and the offer) and the offer follow-up (0x8c2f50). On a loan: A's squad (0x53c590)
/// + 1 above 50, reason 4; otherwise the rule of A's nation (+0x53) answers (vtable +0x20 with the buyer and the
/// target) with the reason. With a reason: the refusal (0x8e23d0), status 0x10, the end (0x8c6db0), the target leaves
/// the buyer's lists and the exchange players those of the club of the target's contract (0x84aeb0), 5 days (0x8c6220)
/// and the mark (`clear_under_offer_mark`). Without a reason, the loan happens at once: the terms (0x8e1f10), the rule
/// of the buyer's nation (vtable +8, or +4 outside a loan), the signed contract (0x4d01e0), the person's +0x6d 2
/// becomes 0x10, the history (0x86da20), the scouting list (0x842080/0x843e10 with 5), 0x5563f0, the numbers (0x85a850,
/// 0x540980, 0x85add0/0x85a470), 0x4196a0, the record of the past transfer (0x8cb810: the person, the +0x33 of its +4,
/// +8 and +0xc, the ids of A and B, 0, 2 and today), status 0x14 with 5 days, the mark, the end (0x8c6db0) and the
/// person out of the lists (0x842b00 with 1).
/// Original: `TRANSFER_MANAGER::handle_player_accepted_terms` (0x8c2180, transfer_manager.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x8c2180 } (thiscall, ret 0x14, transfer_manager.cpp, quadro SEH da data local sem destrutor):
/// `TRANSFER_MANAGER::handle_player_accepted_terms(pessoa, vaga, contrato, tipo, noticia)`, pela aceitação de um
/// contrato (0x4cf413). Vaga fora do vetor: a caixa "Error" (0xe0c, 0xe11) e nada; vaga vazia: nada. O +0x2e zera. O
/// alvo da oferta aceitou: +0x10 = 2 e +0x11 = tipo; A = o comprador, B = o clube do contrato do alvo (tipo 5 não
/// conta). Senão a pessoa é um jogador da troca: o nó dela na troca (+4) ganha 2 e o tipo (+4, +6); A = o clube do
/// contrato do alvo, B = o comprador (sem troca: a caixa 0xe4d; sem o nó, nada achado: A e B a original deixa sem
/// valor, a cópia 0). Fora do empréstimo (+0x1f ≠ 1): a notícia da resposta (0x8d8e90 com A, B, 2 e a oferta) e o
/// seguimento da oferta (0x8c2f50). No empréstimo: elenco de A (0x53c590) + 1 acima de 50, o motivo 4; senão a regra
/// da nação de A (+0x53) responde (vtable +0x20 com o comprador e o alvo) com o motivo. Com motivo: a recusa
/// (0x8e23d0), o status 0x10, o fim (0x8c6db0), o alvo sai das listas do comprador e os da troca das do clube do
/// contrato do alvo (0x84aeb0), 5 dias (0x8c6220) e a marca (`clear_under_offer_mark`). Sem motivo, o empréstimo na hora:
/// os termos (0x8e1f10), a regra da nação do comprador (vtable +8, ou +4 fora do empréstimo), o contrato assinado
/// (0x4d01e0), o +0x6d 2 da pessoa vira 0x10, o histórico (0x86da20), a lista de observação (0x842080/0x843e10 com 5),
/// 0x5563f0, os números (0x85a850, 0x540980, 0x85add0/0x85a470), 0x4196a0, o registro da transferência passada
/// (0x8cb810: a pessoa, os +0x33 de +4, +8 e +0xc dela, os ids de A e B, 0, 2 e hoje), o status 0x14 com 5 dias, a
/// marca, o fim (0x8c6db0) e a pessoa fora das listas (0x842b00 com 1).
pub(crate) unsafe extern "thiscall" fn handle_player_accepted_terms(this: usize, person: usize, slot: i32, contract: usize, kind: u32, news: u32) {
    let news_reply: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, usize, u32, u32, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_POST_CONTRACT_RESPONSE_NEWS);
    let follow: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_ADVANCE_OFFER_TO_CLUB_APPROVAL);
    let squad: unsafe extern "C" fn(usize, u32, u32) -> i8 = core::mem::transmute(FN_DATABASE_COUNT_CLUB_SQUAD);
    let recusar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_POST_LOAN_CANCELLED_NEWS);
    let close: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_CLOSE_OFFER_CONTRACT_OFFERS);
    let unmark: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_SHORTLIST_MANAGER_CLEAR_BID_MARKERS_FOR_PLAYER);
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let schedule: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_SCHEDULE_OFFER);
    let terms: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_POST_LOAN_ACCEPTED_NEWS);
    let assinar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_CONTRACT_MANAGER_SIGN_LOAN_CONTRACT);
    let history: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(FN_STAFF_RECORDS_ADD_LOAN_RECORDS);
    let observado: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(FN_SHORTLIST_MANAGER_IS_ON_SHORTLIST);
    let status: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(FN_SHORTLIST_MANAGER_UPDATE_STAFF_STATUS);
    let store: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_DISCIPLINE_HOLD_BANS_FOR_LOAN);
    let number: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(FN_SQUAD_MANAGER_SET_SQUAD_NUMBER);
    let mark_squad: unsafe extern "C" fn(usize, u32) = core::mem::transmute(FN_STAFF_SET_SQUAD_FLAGS);
    let usa_numeros: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_SQUAD_MANAGER_CLUB_USES_SQUAD_NUMBERS);
    let give_number: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_SQUAD_MANAGER_ALLOCATE_SQUAD_NUMBER);
    let human: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let acabar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_AWOL_CLEAR_AWOL);
    let new_data: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    let copy_data: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let register: unsafe extern "thiscall" fn(usize, *mut u8) = core::mem::transmute(FN_TRANSFER_MANAGER_ADD_PAST_TRANSFER);
    let suspender: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_SHORTLIST_MANAGER_SUSPEND_CLUBS_SHORTLISTING_PLAYER);
    if slot < 0 {
        show_error_at(0xa8d514, 0xe0c);
        return;
    }
    if slot >= read_u32_as_usize(this + transfer_manager_fields::OFFER_CAPACITY) as i32 {
        show_error_at(0xa8d514, 0xe11);
        return;
    }
    let offer = read_u32_as_usize(read_u32_as_usize(this).wrapping_add(4 * slot as usize));
    if offer == 0 {
        return;
    }
    write_u8(offer + offer::RESPONSE_ROUND, 0);
    // a = edi of the original, b = [esp+0x1c]
    let (mut a, mut b) = (0usize, 0usize);
    if read_u32_as_usize(person) == read_u32_as_usize(offer + offer::TARGET_STAFF_ID) {
        write_u8(offer + offer::TERMS_STATE, 2);
        write_u8(offer + offer::JOB_TYPE, kind as u8);
        a = transfer_manager_club_by_id(read_u32_as_usize(offer + offer::BUYER_CLUB_ID));
        let c = contract_of_staff_cached(person);
        b = if c == 0 || read_u8(c + contract::CONTRACT_TYPE) & 0x3f == 5 { 0 } else { transfer_manager_club_by_id(read_u32_as_usize(c + contract::CLUB)) };
    } else if read_u32_as_usize(offer + offer::EXCHANGE_PLAYERS) == 0 {
        show_error_at(0xa8d514, 0xe4d);
    } else {
        let swap = read_u32_as_usize(offer + offer::EXCHANGE_PLAYERS);
        write_u32_from_usize(swap + llist::CURSOR, read_u32_as_usize(swap + llist::HEAD));
        let mut n = next(read_u32_as_usize(offer + offer::EXCHANGE_PLAYERS));
        while n != 0 {
            if read_u32_as_usize(n) == read_u32_as_usize(person) {
                write_u8(n + 4, 2);
                write_u8(n + 6, kind as u8);
                let c = contract_of_staff_cached(transfer_manager_staff_by_id(read_u32_as_usize(offer + offer::TARGET_STAFF_ID)));
                a = if c != 0 { transfer_manager_club_by_id(read_u32_as_usize(c + contract::CLUB)) } else { 0 };
                b = transfer_manager_club_by_id(read_u32_as_usize(offer + offer::BUYER_CLUB_ID));
                break;
            }
            n = next(read_u32_as_usize(offer + offer::EXCHANGE_PLAYERS));
        }
    }
    if read_u8(offer + offer::OFFER_TYPE) != 1 {
        // simplification: the reply (2) goes in a dword with leftover high bytes in the original; 0x8d8e90 reads the byte
        news_reply(this, person, a, b, 2, offer, 0, news, 0);
        crate::comparison::branch("cqf_2 seguir a oferta");
        follow(this, offer); // cqf2: a oferta segue
        return;
    }
    crate::comparison::branch("cqf_2 empréstimo");
    let mut reason: u32 = 0;
    if (squad(a, 0, 0) as i32) + 1 > 0x32 { // cqf2: o elenco cheio
        reason = 4;
    } else if let Some(r) = nation_transfer_rules_opt(this, read_u32_as_usize(a + club::NATION)) {
        if r != 0 {
            let can: unsafe extern "thiscall" fn(usize, usize, usize, u32, *mut u32) = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(r) + 0x20));
            can(r, transfer_manager_club_by_id(read_u32_as_usize(offer + offer::BUYER_CLUB_ID)), transfer_manager_staff_by_id(read_u32_as_usize(offer + offer::TARGET_STAFF_ID)), 0, &mut reason);
        }
    }
    if reason as u16 != 0 {
        crate::comparison::branch("cqf_2 empréstimo recusado");
        recusar(this, offer, reason);
        write_u8(offer + offer::STATUS, 0x10);
        close(this, offer);
        let target = transfer_manager_staff_by_id(read_u32_as_usize(offer + offer::TARGET_STAFF_ID));
        let c = contract_of_staff_cached(target);
        let owner = if c != 0 { transfer_manager_club_by_id(read_u32_as_usize(c + contract::CLUB)) } else { 0 };
        unmark(read_u32_as_usize(this + transfer_manager_fields::SHORTLIST_MANAGER), transfer_manager_club_by_id(read_u32_as_usize(offer + offer::BUYER_CLUB_ID)), target);
        let swap = read_u32_as_usize(offer + offer::EXCHANGE_PLAYERS);
        if swap != 0 {
            write_u32_from_usize(swap + llist::CURSOR, read_u32_as_usize(swap + llist::HEAD));
            let mut n = next(read_u32_as_usize(offer + offer::EXCHANGE_PLAYERS));
            while n != 0 {
                unmark(read_u32_as_usize(this + transfer_manager_fields::SHORTLIST_MANAGER), owner, transfer_manager_staff_by_id(read_u32_as_usize(n)));
                n = next(read_u32_as_usize(offer + offer::EXCHANGE_PLAYERS));
            }
        }
        schedule(this, offer, 5, 0);
        clear_under_offer_mark(this, offer);
        return;
    }
    crate::comparison::branch("cqf_2 empréstimo na hora");
    terms(this, offer, contract);
    if let Some(r) = nation_transfer_rules_opt(this, read_u32_as_usize(transfer_manager_club_by_id(read_u32_as_usize(offer + offer::BUYER_CLUB_ID)) + 0x53)) {
        if r != 0 {
            let k = if read_u8(offer + offer::OFFER_TYPE) == 0 { 4 } else { 8 };
            let f: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(r) + k));
            f(r, offer);
        }
    }
    assinar(read_u32_as_usize(G_CONTRACT_MANAGER), contract);
    if read_u8(person + staff::SQUAD_SELECTED_FOR) == 2 {
        write_u8(person + staff::SQUAD_SELECTED_FOR, 0x10);
    }
    history(read_u32_as_usize(G_STAFF_RECORDS), person, b, a);
    if observado(read_u32_as_usize(this + transfer_manager_fields::SHORTLIST_MANAGER), person, a) != 0 {
        status(read_u32_as_usize(this + transfer_manager_fields::SHORTLIST_MANAGER), person, a, 5, 0);
    }
    store(read_u32_as_usize(G_DISCIPLINE), transfer_manager_staff_by_id(read_u32_as_usize(offer + offer::TARGET_STAFF_ID)));
    number(read_u32_as_usize(G_SQUAD_MANAGER), transfer_manager_staff_by_id(read_u32_as_usize(offer + offer::TARGET_STAFF_ID)), a, 0);
    mark_squad(person, 1);
    if human(a, 0, 1, 0, 0) == 0 && usa_numeros(read_u32_as_usize(G_SQUAD_MANAGER), a, 0) != 0 {
        give_number(read_u32_as_usize(G_SQUAD_MANAGER), a, person);
    }
    acabar(read_u32_as_usize(G_AWOL), person);
    // the past transfer record, 0x25 bytes: +0 the person, +4/+8/+0xc the +0x33 of its pointers +4/+8/+0xc,
    // +0x10 and +0x14 the ids of A and B, +0x18 0, +0x1c 2, +0x1d today
    let mut reg = [0u8; 0x28];
    let r = reg.as_mut_ptr() as usize;
    new_data(r + past_transfer::DATE);
    write_u32_from_usize(r + past_transfer::TO_CLUB_ID, read_u32_as_usize(a));
    write_u32_from_usize(r + past_transfer::FROM_CLUB_ID, read_u32_as_usize(b));
    let mut tmp = [0u8; 8];
    copy_data(r + past_transfer::DATE, tmp.as_mut_ptr(), G_CURRENT_DATE);
    write_u32_from_usize(r, read_u32_as_usize(person));
    write_u32_from_usize(r + past_transfer::FIRST_NAME_ID, read_u32_as_usize(read_u32_as_usize(person + staff::FIRST_NAME) + 0x33));
    write_u32_from_usize(r + past_transfer::SECOND_NAME_ID, read_u32_as_usize(read_u32_as_usize(person + staff::SECOND_NAME) + 0x33));
    write_u32_from_usize(r + past_transfer::COMMON_NAME_ID, read_u32_as_usize(read_u32_as_usize(person + staff::COMMON_NAME) + 0x33));
    write_u8(r + past_transfer::MOVE_TYPE, 2);
    write_u32_from_usize(r + past_transfer::FEE, 0);
    register(this, reg.as_mut_ptr());
    write_u8(offer + offer::STATUS, 0x14);
    schedule(this, offer, 5, 0); // cqf2: the loan completed in 5 days
    clear_under_offer_mark(this, offer);
    close(this, offer);
    suspender(read_u32_as_usize(this + transfer_manager_fields::SHORTLIST_MANAGER), person, 1);
}

/// Shows the "Error" box "v%s %s.%s %d" for source file `arquivo` and line, then clears G_MOUSE_LEFT_BUTTON_DOWN.
///
/// [pt] A caixa "Error" com "v%s %s.%s %d" (versão 0x90d130, nome e extensão do .cpp `arquivo` pela 0x944cff, `linha`) em
/// 0xae24d0 (0x944c9f) e a caixa 0x5e8290; [0xb67a34] = 0. As três chamadas cdecl, como na original.
pub(crate) unsafe fn show_error_box(file: usize, row: u32) {
    let split: unsafe extern "C" fn(usize, usize, usize, *mut u8, *mut u8) = core::mem::transmute(FN_CRT_STRTOK);
    let version: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let caixa: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(FN_GUI_SHOW_MESSAGE_BOX);
    let (mut nome, mut ext) = ([0u8; 256], [0u8; 256]);
    split(file, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
    let s = version(nome.as_mut_ptr(), ext.as_mut_ptr(), row);
    format(0xae24d0, 0x9870e8, s, nome.as_ptr(), ext.as_ptr(), row);
    caixa(0x9870e0, 0xae24d0, 0);
    write_u32_from_usize(G_MOUSE_LEFT_BUTTON_DOWN, 0);
}

/// The computer seller answers a bid: loan bids are refused without looking at anything else; unless a rule makes it
/// accept, the floor fee and the other offers, the exchange value, the cash, exchange and clause options; an option
/// worth less than the bid / 0.85 accepts, the best one under the bid / 0.75 becomes a counter offer, otherwise
/// refuses.
///
/// Ret 8; SEH frame without a destructor (the slot's local date): `TRANSFER_MANAGER::ai_respond_to_bid(slot, dry_run)`,
/// the computer seller club's answer to a bid. The target is +0xc of the offer, the buyer +8 and the seller the club of
/// the target's contract (0x4cf5a0, in `[0xae1080]`; except type 5).
///
/// - Target without a player sheet (+0x61): status 0x10 and the offer in the next slice (0x8c6220: push 0; push 1; push
/// offer).
///
/// - Loan bid (+0x1f = 1): refused without looking at anything else (0x8bfa30: push 0; push 0; push slot), outside the
/// dry run.
///
/// - The seller must accept (0x8cd240, stdcall(offer, 0, 0)): accepted (0x8c0080: push 0; push slot), outside the dry
/// run.
///
/// - Human seller (0x600a50 with 1, 1) with a manager (0x53ca90) who answers (0x5fd210 on `[0xb63c98]`): his rule 0
/// (0x5fea50: push manager; push 0) with an exchange, or equal to 1: refusal (0x8bf680: push 0; push 0; push slot).
///
/// - The floor (0x4d1600: push 1; push 1; push &reason; push target; push buyer; push seller; mov ecx, `[0xae1050]`)
/// and, outside the dry run, the other offers of list +0 that are not active: their value (0x8e7b50 with 0, 0, 1, 0, 1)
/// above the floor becomes the floor and zeroes the reason. The total (0x8e7a70 with 1, 0, 0, 1), the asking price
/// (0x8e7b50 with 0, 0, 1, 1, 1) and the value k (0x8e7b50 with 0, 1, 1, 1, 1). Each exchange player (+4): the position
/// word (0x5b7d20: the first bit of 1, 2, 4, 0x10, 8; otherwise 0x20 or 0x40) in the local slot, his value for the
/// seller (0x840d80 with −1, 0) up to floor − k and, outside the dry run, if the seller does not accept him (0x843800
/// with 1) the exchange is refused. The price is the larger of floor and k.
///
/// - The options (cash 0x8c8740, exchange 0x8c8860, clause 0x8c8fd0): with none, refusal with the reason (0x8bf680 with
/// 1). The most attractive one is the best; the first whose value (0x8e7b50 with 0, 0, 1, 1, 1) × 0.85 is below the
/// asking price, with no refused exchange: accepted (outside the dry run). No best: the box (line 0x1eee) and the
/// refusal, without freeing the options.
///
/// - Otherwise: the best one's value (0x8e7b50 with 0, 0, 0, 0, 1) × 0.75 below the asking price, type (+0x31 & 0xf)
/// not 1 or 2 and outside the dry run: the counter offer (the low nibble of +0x30 of the offer into the best one and
/// 0x8be9d0: push 0; push best; push offer). Otherwise the refusal (0x8bf680 with 1), with reason 10 if the total was
/// below 0.55 of the target's value (+0x52) with a 2 million difference, or below 0.1 with 100 thousand (outside the
/// dry run). The options are destroyed (0x8e5710) and freed.
/// Original: `TRANSFER_MANAGER::ai_respond_to_bid` (0x8c8090, transfer_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x8c8090 } (thiscall, ret 8, transfer_manager.cpp, quadro SEH sem destrutor: a data local da vaga):
/// `TRANSFER_MANAGER::ai_respond_to_bid(vaga, ensaio)`, a resposta do clube vendedor do computador a um lance. O alvo é
/// o +0xc da oferta, o comprador o +8 e o vendedor o clube do contrato do alvo (0x4cf5a0, em [0xae1080]; fora o tipo 5).
/// - Alvo sem ficha de jogador (+0x61): status 0x10 e a oferta na fatia seguinte (0x8c6220: push 0; push 1; push oferta).
/// - Lance de empréstimo (+0x1f = 1): recusado sem olhar mais nada (0x8bfa30: push 0; push 0; push vaga), fora do ensaio.
/// - O vendedor deve aceitar (0x8cd240, stdcall(oferta, 0, 0)): aceito (0x8c0080: push 0; push vaga), fora do ensaio.
/// - Vendedor humano (0x600a50 com 1, 1) com o técnico (0x53ca90) que responde (0x5fd210 em [0xb63c98]): a regra 0 dele
///   (0x5fea50: push técnico; push 0) com troca, ou igual a 1: recusa (0x8bf680: push 0; push 0; push vaga).
/// - O piso (0x4d1600: push 1; push 1; push &motivo; push alvo; push comprador; push vendedor; mov ecx, [0xae1050]) e,
///   fora do ensaio, as outras ofertas da lista +0 que não estão ativas: o valor delas (0x8e7b50 com 0, 0, 1, 0, 1) acima
///   do piso vira o piso e zera o motivo. O total (0x8e7a70 com 1, 0, 0, 1), o pedido (0x8e7b50 com 0, 0, 1, 1, 1) e o
///   valor k (0x8e7b50 com 0, 1, 1, 1, 1). Cada jogador da troca (+4): a palavra da posição (0x5b7d20: o primeiro bit de
///   1, 2, 4, 0x10, 8; senão 0x20 ou 0x40) na vaga local, o valor dele para o vendedor (0x840d80 com −1, 0) até piso − k
///   e, fora do ensaio, se o vendedor não o aceita (0x843800 com 1) a troca fica recusada. O preço é o maior de piso e k.
/// - As opções (o dinheiro 0x8c8740, a troca 0x8c8860, a cláusula 0x8c8fd0): sem nenhuma, recusa com o motivo (0x8bf680
///   com 1). A de maior atração é a melhor; a primeira de valor (0x8e7b50 com 0, 0, 1, 1, 1) × 0,85 abaixo do pedido, sem
///   troca recusada: aceita (fora do ensaio). Sem melhor: a caixa (linha 0x1eee) e a recusa, sem soltar as opções.
/// - Senão: o valor da melhor (0x8e7b50 com 0, 0, 0, 0, 1) × 0,75 abaixo do pedido, tipo (+0x31 & 0xf) fora de 1 e 2 e
///   fora do ensaio: a contraproposta (o nibble baixo de +0x30 da oferta na melhor e 0x8be9d0: push 0; push melhor; push
///   oferta). Senão a recusa (0x8bf680 com 1), com o motivo 10 se o total ficou abaixo de 0,55 do valor do alvo (+0x52)
///   com 2 milhões de diferença, ou de 0,1 com 100 mil (fora do ensaio). As opções são destruídas (0x8e5710) e liberadas.
pub(crate) unsafe extern "thiscall" fn ai_respond_to_bid(this: usize, slot: usize, ensaio: u32) {
    let data_default: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let schedule: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_SCHEDULE_OFFER);
    let recusar_emprestimo: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_REJECT_LOAN_OFFER);
    let deve_aceitar: unsafe extern "stdcall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_SELLING_CLUB_MUST_ACCEPT);
    let aceitar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_ACCEPT_TRANSFER_BID);
    let human: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let manager: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_CLUB_MANAGER);
    let responde: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_ON_HOLIDAY);
    let rule: unsafe extern "thiscall" fn(usize, u32, usize) -> u8 = core::mem::transmute(FN_HUMAN_MANAGER_GET_HOLIDAY_OPTION);
    let recusar: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_REJECT_TRANSFER_OFFER);
    let floor_of: unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u32, u32, u32) -> i32 = core::mem::transmute(FN_CONTRACT_MANAGER_CALC_ASKING_PRICE);
    let value: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> i32 = core::mem::transmute(FN_TRANSFER_OFFER_GET_WEIGHTED_VALUE);
    let total_of: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> i32 = core::mem::transmute(FN_TRANSFER_OFFER_GET_TOTAL_VALUE);
    let mask: unsafe extern "C" fn(usize) -> u16 = core::mem::transmute(FN_FORMATION_GET_POSITION_MASK);
    let value_player: unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut i32, i32, u32) -> i32 = core::mem::transmute(FN_SHORTLIST_MANAGER_VALUE_PLAYER_FOR_CLUB);
    let accepted_swap: unsafe extern "thiscall" fn(usize, usize, usize, i32, u32) -> u32 = core::mem::transmute(FN_SHORTLIST_MANAGER_CLUB_WANTS_PLAYER);
    let dinheiro: unsafe extern "thiscall" fn(usize, usize, usize, *mut usize, *mut i8, i32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_BUILD_CASH_OFFER_OPTION);
    let swap: unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut usize, *mut i8, i32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_BUILD_EXCHANGE_OFFER_OPTION);
    let clausula: unsafe extern "thiscall" fn(usize, usize, usize, *mut usize, *mut i8, i32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_BUILD_CLAUSE_OFFER_OPTION);
    let negociar: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_ADD_TRANSFER_NEGOTIATION);
    let destroy: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_TRANSFER_OFFER_DESTRUCTOR_TRANSFER_OFFER);
    let free: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_OPERATOR_DELETE);
    // simplification: the local slot (esp+0x58) the original leaves with stack leftover outside the date (+0x1b), of the position word,
    // of +0xb and bits 2..4 and 0..1 of +0x24; 0x840d80 reads here only the position, +0x24 (type 4) and +0xf (compared with
    // the swap player, who is not the seller’s): the copy zeroes
    let mut vl = [0u8; 0x28];
    let pv = vl.as_mut_ptr() as usize;
    let mut melhor_atracao: i8 = -1;
    let mut troca_recusada: u32 = 0;
    data_default(pv + 0x1b);
    vl[0x24] = (vl[0x24] & 0xe4) | 4;
    write_u32_from_usize(pv + 0xb, 10000);
    let o = read_u32_as_usize(slot);
    let target = transfer_manager_staff_by_id(read_u32_as_usize(o + offer::TARGET_STAFF_ID));
    let buyer = transfer_manager_club_by_id(read_u32_as_usize(o + offer::BUYER_CLUB_ID));
    let c = contract_of(read_u32_as_usize(G_CONTRACT_MANAGER), target);
    write_u32_from_usize(G_CONTRACT_LOOKUP_RESULT, c);
    let seller = if c == 0 || read_u8(c + contract::CONTRACT_TYPE) & 0x3f == 5 { 0 } else { transfer_manager_club_by_id(read_u32_as_usize(c + contract::CLUB)) };
    if read_u32_as_usize(target + staff::PLAYER) == 0 {
        write_u8(o + offer::STATUS, 0x10);
        schedule(this, o, 1, 0);
        return;
    }
    let mut reason: u32 = 0;
    let mut n: i8 = 0;
    if read_u8(o + offer::OFFER_TYPE) == 1 {
        if ensaio == 0 {
            recusar_emprestimo(this, slot, 0, 0);
        }
        return;
    }
    if deve_aceitar(read_u32_as_usize(slot), 0, 0) != 0 {
        if ensaio == 0 {
            aceitar(this, slot, 0);
        }
        return;
    }
    if seller != 0 && human(seller, 1, 1, 0, 0) != 0 && responde(read_u32_as_usize(G_HUMAN_MANAGER), manager(seller)) != 0 {
        if rule(read_u32_as_usize(G_HUMAN_MANAGER), 0, manager(seller)) != 0 && read_u32_as_usize(o + offer::EXCHANGE_PLAYERS) != 0 {
            recusar(this, slot, 0, 0);
            return;
        }
        if rule(read_u32_as_usize(G_HUMAN_MANAGER), 0, manager(seller)) == 1 {
            recusar(this, slot, 0, 0);
            return;
        }
    }
    let mut floor = floor_of(read_u32_as_usize(G_CONTRACT_MANAGER), seller, buyer, target, &mut reason, 1, 1);
    let l = read_u32_as_usize(o);
    if l != 0 && ensaio == 0 {
        write_u32_from_usize(l + llist::CURSOR, read_u32_as_usize(l + llist::HEAD));
        loop {
            let p = transfer_manager_llist_next(read_u32_as_usize(o));
            if p == 0 {
                break;
            }
            let k = read_u32_as_usize(p);
            let other = read_u32_as_usize(read_u32_as_usize(this).wrapping_add(k.wrapping_mul(4)));
            if k != read_u32_as_usize(o + offer::SLOT_INDEX) && !is_offer_status_settled(other) {
                let v = value(other, 0, 0, 1, 0, 1);
                if v > floor {
                    floor = v;
                    reason = 0;
                }
            }
        }
    }
    let total = total_of(o, 1, 0, 0, 1);
    let request = value(o, 0, 0, 1, 1, 1);
    let k = value(o, 0, 1, 1, 1, 1);
    let t = read_u32_as_usize(o + offer::EXCHANGE_PLAYERS);
    if t != 0 {
        write_u32_from_usize(t + llist::CURSOR, read_u32_as_usize(t + llist::HEAD));
        let ceiling = floor.wrapping_sub(k);
        let mut p = transfer_manager_llist_next(read_u32_as_usize(o + offer::EXCHANGE_PLAYERS));
        while p != 0 {
            let j = transfer_manager_staff_by_id(read_u32_as_usize(p));
            let m = mask(read_u32_as_usize(j + staff::PLAYER));
            let w: u16 = if m & 1 != 0 {
                1
            } else if m & 2 != 0 {
                2
            } else if m & 4 != 0 {
                4
            } else if m & 0x10 != 0 {
                0x10
            } else if m & 8 != 0 {
                8
            } else if m & 0x20 != 0 {
                0x20
            } else {
                0x40
            };
            core::ptr::write_unaligned((pv + 1) as *mut u16, w);
            let mut out: i32 = 0;
            let mut v = value_player(read_u32_as_usize(read_u32_as_usize(G_TRANSFER_MANAGER) + 0x856), seller, j, pv, &mut out, -1, 0);
            if v > ceiling {
                v = ceiling;
            }
            if ensaio == 0 && accepted_swap(read_u32_as_usize(this + transfer_manager_fields::SHORTLIST_MANAGER), j, seller, v, 1) == 0 {
                troca_recusada = 1;
            }
            p = transfer_manager_llist_next(read_u32_as_usize(o + offer::EXCHANGE_PLAYERS));
        }
    }
    let price = if k > floor { // cqe1: the price by value k
        crate::comparison::branch("cqe_1 preço pelo k");
        k
    } else {
        floor
    };
    let mut opcoes = [0usize; 4];
    let mut atracao = [0i8; 4];
    if dinheiro(this, buyer, target, &mut opcoes[0], &mut atracao[0], price) != 0 {
        n = 1;
    }
    if swap(this, buyer, target, o, &mut opcoes[n as usize], &mut atracao[n as usize], price) != 0 {
        n += 1;
    }
    if clausula(this, buyer, target, &mut opcoes[n as usize], &mut atracao[n as usize], price) != 0 {
        n += 1;
    }
    let mut aceito = false;
    let mut best: usize = 0;
    if n > 0 {
        let request_f = request as f64;
        for i in 0..n as usize {
            if atracao[i] > melhor_atracao {
                best = opcoes[i];
                melhor_atracao = atracao[i];
            }
            let v = value(opcoes[i], 0, 0, 1, 1, 1);
            if v as f64 * read_f64_at(0x969520) < request_f && troca_recusada == 0 { // cqe1: the option × 0.85 below the asking price; 0x969520 = 0.85
                crate::comparison::branch("cqe_1 aceita");
                if ensaio == 0 {
                    aceitar(this, slot, 0);
                }
                aceito = true;
                break;
            }
        }
    }
    if n == 0 {
        recusar(this, slot, 1, reason);
        return;
    }
    if best == 0 {
        show_error_box(0xa8d514, 0x1eee);
        recusar(this, slot, 1, reason);
        return;
    }
    if !aceito {
        let request_f = request as f64;
        let v = value(best, 0, 0, 0, 0, 1);
        let kind = read_u32_as_usize(o + offer::OFFER_KIND) & 0xf;
        if v as f64 * read_f64_at(0x969040) < request_f && kind != 1 && kind != 2 && ensaio == 0 { // cqe1: a contraproposta; 0x969040 = 0.75
            crate::comparison::branch("cqe_1 contraproposta");
            write_u8(best + offer::COMPLETION_TIMING, (read_u8(best + offer::COMPLETION_TIMING) & 0xf0) | (read_u8(o + offer::COMPLETION_TIMING) & 0xf));
            negociar(this, o, best, 0);
        } else {
            let w = read_u32_as_usize(target + staff::VALUE) as i32;
            let (tf, wf) = (total as f64, w as f64);
            if (tf < wf * read_f64_at(0x96a5d8) && w.wrapping_sub(total) > 2000000) // 0x96a5d8 = 0.55
                || (tf < wf * read_f64_at(0x967880) && w.wrapping_sub(total) > 100000 && ensaio == 0) // 0x967880 = 0.1
            {
                reason = 10;
            }
            recusar(this, slot, 1, reason);
        }
    }
    for i in 0..n as usize {
        let s = opcoes[i];
        if s != 0 {
            destroy(s);
            free(s);
        }
    }
}

/// Screens only: the offer copy without its lists, with the contract built from the new loan terms, becomes a
/// negotiation; returns the copy's index.
///
/// Ret 0x14; SEH frame for the local offer: `TRANSFER_MANAGER::add_loan_negotiation(offer, a2, a3, a4, a5)`, only from
/// the transfer screen (0x8fb5e0). A local STAFF_CONTRACT (0x85e410) and a local offer (0x8e5680) with the offer's
/// fields (the date +0x12 through 0x417c10; the low nibble of +0x30 and of +0x31; lists +0 and +4 empty); the contract
/// from the current one (0x85e590: push a5; push a4; push a3; push a2; push club +8; push target +0xc; lea ecx,
/// contract) goes into the list at this+0x946 (created if missing: 0x944cf1 of 0xe bytes and 0x68d020; 0x68d0d0 with
/// 0x50 bytes) and the copy becomes a negotiation (0x8be9d0: push 0; push &copy; push offer). Returns +0x20 of the copy
/// (the index); the copy is destroyed (0x8e5710).
/// Original: `TRANSFER_MANAGER::add_loan_negotiation` (0x8bee50, transfer_manager.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x8bee50 } (thiscall, ret 0x14, transfer_manager.cpp, quadro SEH da oferta local): `TRANSFER_MANAGER::
/// add_loan_negotiation(oferta, a2, a3, a4, a5)`, só pela tela de transferências (0x8fb5e0). Um STAFF_CONTRACT local
/// (0x85e410) e uma oferta local (0x8e5680) com os campos da oferta (a data +0x12 pela 0x417c10; o nibble baixo de +0x30
/// e de +0x31; as listas +0 e +4 vazias); o contrato pelo atual (0x85e590: push a5; push a4; push a3; push a2; push
/// clube +8; push alvo +0xc; lea ecx, contrato) entra na lista em this+0x946 (criada se faltar: 0x944cf1 de 0xe bytes e
/// 0x68d020; 0x68d0d0 com 0x50 bytes) e a cópia vira negociação (0x8be9d0: push 0; push &cópia; push oferta). Devolve o
/// +0x20 da cópia (o índice); a cópia é destruída (0x8e5710).
pub(crate) unsafe extern "thiscall" fn add_loan_negotiation(this: usize, o: usize, a2: u32, a3: u32, a4: u32, a5: u32) -> u32 {
    let new_contract: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_STAFF_CONTRACT_STAFF_CONTRACT);
    let new_offer: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_TRANSFER_OFFER_TRANSFER_OFFER);
    let copy: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let by_current: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32) = core::mem::transmute(FN_STAFF_CONTRACT_INIT_FROM_CURRENT_CONTRACT);
    let alloc: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let list_new: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let append: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_LLIST_ADD_TAIL);
    let negociar: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_ADD_TRANSFER_NEGOTIATION);
    let destroy: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_TRANSFER_OFFER_DESTRUCTOR_TRANSFER_OFFER);
    let mut contract = [0u8; 0x50];
    let pc = contract.as_mut_ptr() as usize;
    // simplification: the high nibble of +0x30 and bits 4..31 of the copy’s +0x31 the original leaves with stack leftovers; here zero
    let mut copy_offer = [0u8; 0x44];
    let po = copy_offer.as_mut_ptr() as usize;
    new_contract(pc);
    new_offer(po);
    let target = transfer_manager_staff_by_id(read_u32_as_usize(o + offer::TARGET_STAFF_ID));
    let club = transfer_manager_club_by_id(read_u32_as_usize(o + offer::BUYER_CLUB_ID));
    write_u32_from_usize(po + offer::TARGET_STAFF_ID, read_u32_as_usize(o + offer::TARGET_STAFF_ID));
    write_u8(po + offer::JOB_TYPE, read_u8(o + offer::JOB_TYPE));
    write_u32_from_usize(po + offer::BUYER_CLUB_ID, read_u32_as_usize(o + offer::BUYER_CLUB_ID));
    write_u8(po + offer::TERMS_STATE, read_u8(o + offer::TERMS_STATE));
    write_u32_from_usize(po, read_u32_as_usize(o));
    write_u32_from_usize(po + offer::EXCHANGE_PLAYERS, read_u32_as_usize(o + offer::EXCHANGE_PLAYERS));
    let mut tmp = [0u8; 8];
    copy(po + offer::DUE_DATE, tmp.as_mut_ptr() as usize, o + offer::DUE_DATE);
    write_u32_from_usize(po + offer::FEE, read_u32_as_usize(o + offer::FEE));
    write_u8(po + offer::OFFER_TYPE, read_u8(o + offer::OFFER_TYPE));
    write_u8(po + offer::DUE_SLICE, read_u8(o + offer::DUE_SLICE));
    write_u32_from_usize(po + offer::SERIAL, read_u32_as_usize(o + offer::SERIAL));
    write_u8(po + offer::PLAYER_AWARE, read_u8(o + offer::PLAYER_AWARE));
    write_u32_from_usize(po + offer::NEGOTIATION_INDEX, read_u32_as_usize(o + offer::NEGOTIATION_INDEX));
    write_u8(po + offer::RESPONSE_ROUND, read_u8(o + offer::RESPONSE_ROUND));
    write_u32_from_usize(po + offer::SLOT_INDEX, read_u32_as_usize(o + offer::SLOT_INDEX));
    write_u8(po + offer::STATUS, read_u8(o + offer::STATUS));
    write_u8(po + offer::PRE_CONTRACT, read_u8(o + offer::PRE_CONTRACT));
    write_u8(po + offer::COMPLETION_TIMING, (read_u8(po + offer::COMPLETION_TIMING) & 0xf0) | (read_u8(o + offer::COMPLETION_TIMING) & 0xf));
    write_u32_from_usize(po + offer::OFFER_KIND, (read_u32_as_usize(po + offer::OFFER_KIND) & !0xf) | (read_u32_as_usize(o + offer::OFFER_KIND) & 0xf));
    write_u32_from_usize(po + offer::APPEARANCE_FEE, read_u32_as_usize(o + offer::APPEARANCE_FEE));
    write_u8(po + offer::APPEARANCE_GAMES, read_u8(o + offer::APPEARANCE_GAMES));
    write_u32_from_usize(po + offer::INTERNATIONAL_FEE, read_u32_as_usize(o + offer::INTERNATIONAL_FEE));
    write_u8(po + offer::INSTALMENTS, read_u8(o + offer::INSTALMENTS));
    write_u8(po + offer::INTERNATIONAL_GAMES, read_u8(o + offer::INTERNATIONAL_GAMES));
    write_u8(po + offer::SELL_ON_PERCENT, read_u8(o + offer::SELL_ON_PERCENT));
    write_u32_from_usize(po + offer::EXCHANGE_PLAYERS, 0);
    write_u32_from_usize(po, 0);
    by_current(pc, target, club, a2, a3, a4, a5);
    if read_u32_as_usize(this + transfer_manager_fields::LOAN_NEGOTIATIONS) == 0 {
        let m = alloc(0xe);
        let l = if m != 0 { list_new(m) } else { 0 };
        write_u32_from_usize(this + transfer_manager_fields::LOAN_NEGOTIATIONS, l);
    }
    append(read_u32_as_usize(this + transfer_manager_fields::LOAN_NEGOTIATIONS), pc, 0x50);
    negociar(this, o, po, 0);
    let r = read_u32_as_usize(po + offer::SLOT_INDEX) as u32;
    destroy(po);
    r
}

/// Person -> club of their contract (get_contract, stored in G_CONTRACT_LOOKUP_RESULT), type 5 included; 0 without
/// contract.
///
/// [pt] O contrato da pessoa (0x4cf5a0: push pessoa; mov ecx, [0xae1050] -> thiscall, ret 4), guardado em [0xae1080], e o
/// clube dele (sem contrato: 0; o tipo 5 conta).
pub(crate) unsafe fn contract_club_any_type(person: usize) -> usize {
    let contract: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = contract(read_u32_as_usize(G_CONTRACT_MANAGER), person);
    write_u32_from_usize(G_CONTRACT_LOOKUP_RESULT, c);
    if c == 0 { 0 } else { transfer_manager_club_by_id(read_u32_as_usize(c + contract::CLUB)) }
}

/// ORs into `m` the flag (0x40/0x50/0x10/0x30/0x20) for the reputation scope of the person vs the seller's manager
/// (+0xcf); other scopes return `antes` unchanged.
///
/// [pt] As marcas da nota pelo alcance da reputação (0x5408c0, cdecl(pessoa, técnico do vendedor +0xcf)): 0..4 dão 0x40,
/// 0x50, 0x10, 0x30, 0x20 sobre `m` (as duas tabelas de salto da original, 0x8c8fa4 e 0x8c8fb8); fora disso a original
/// deixa o ebp como estava (`antes`).
pub(crate) unsafe fn reputation_scope_flags(person: usize, seller: usize, m: u32, before: u32) -> u32 {
    let escopo: unsafe extern "C" fn(usize, usize) -> u8 = core::mem::transmute(FN_DATABASE_GET_REPUTATION_SCOPE);
    match escopo(person, read_u32_as_usize(seller + club::MANAGER)) as i8 {
        0 => m | 0x40,
        1 => m | 0x50,
        2 => m | 0x10,
        3 => m | 0x30,
        4 => m | 0x20,
        _ => before,
    }
}

/// Rates the person for a position with the given flags for the seller club (MATCH_MAN score_player_for_role 0x6e2f20
/// with the original's fixed arguments).
///
/// [pt] A nota da pessoa para o vendedor (0x6e2f20, cdecl de 15 dwords: pessoa, +0x61, 0, a posição, as marcas, o vendedor,
/// 0, 0, −1 e seis zeros).
pub(crate) unsafe fn score_player_for_seller(person: usize, position: u16, marks: u32, seller: usize) -> i32 {
    let rating: unsafe extern "C" fn(usize, usize, usize, u32, u32, usize, usize, usize, i32, u32, u32, u32, u32, u32, usize) -> i32 =
        core::mem::transmute(FN_MATCH_MAN_SCORE_PLAYER_FOR_ROLE);
    rating(person, read_u32_as_usize(person + staff::PLAYER), 0, position as u32, marks, seller, 0, 0, -1, 0, 0, 0, 0, 0, 0)
}

/// Builds the exchange option: a new offer at the price with attraction 50; for the target and the 25 slots of the
/// seller's shortlist, buyer players in the same position, free of offers and exchanges, with interest of at least 3.15
/// and rating above 0.9 of the candidate's join the exchange (up to 3): attraction rises 20 or 5 and their value leaves
/// the price.
///
/// Ret 0x18; SEH frame for the local LLIST: `TRANSFER_MANAGER::make_exchange_option(buyer, target, offer, out,
/// attraction, value)`, the exchange option of the AI's answer to a bid (0x8c8090). The seller is the club of the
/// target's contract (0x4cf5a0, in `[0xae1080]`); the local LLIST (0x68d020/0x68d040). The target's contract with the
/// seller (0x4cf5f0). Without the seller's scouting list (byte +4 of the 9-byte record in `[[this+0x856]+0xdc]`): 0.
/// Otherwise the buyer's squad (0x541f90 with 0: 50 slots), the new offer (0x944cf1 of 0x41 bytes and 0x8e5680) in
/// *out, prepared (0x8e78f0: push value; push buyer; push target; mov ecx, offer), attraction 50, +0x1b = value and the
/// ceiling = value × 1.1 (_ftol). For the target (slot −1, except a contract with status 7) and the 25 slots of the
/// seller's list (the +0xf, excluding the seller's own players), while the exchange does not have 3: the position
/// (0x5b7d20), the flags (0x4800, or 0x4900 outside status 2 and 3; outside 5, | 0x80 and the range) and the base
/// rating. Then the target himself and the 50 of the squad with the same position: a contract with the candidate's
/// current club (0x4cf5f0) of a type other than 5, at the buyer, without a sheet (0x4cf6d0), without bits 0x30 of +0xb
/// of the 79-byte record, floor (0x4d1600 with 0, 1, 1) up to the ceiling; with no offer (0x8c5cf0), exchange (0x8c5e40
/// in the local list) or exchange negotiation of the target (0x8c5fc0), or only this offer's exchange (+0x20); the
/// seller can reach him (0x8ce990 with 0, 1); interest (0x846920: push 0; push status; push 0; push 1; push seller;
/// push player; mov ecx, `[this+0x856]`) of at least 3.15; his rating / the base above 0.9: the attraction rises 20
/// (higher rating) or 5, between 0 and 100; not yet in the exchange (+4, created if missing), he joins it (0x68d0d0
/// with 7 bytes: id, 0, status), his value (0x840d80 with −1, 1) is taken off the ceiling and off +0x1b (at least 0),
/// and the slot ends. Returns 1.
/// Original: `TRANSFER_MANAGER::make_exchange_option` (0x8c8860, transfer_manager.cpp, thiscall, ret 0x18).
///
/// [pt] { 0x8c8860 } (thiscall, ret 0x18, transfer_manager.cpp, quadro SEH da LLIST local): `TRANSFER_MANAGER::
/// make_exchange_option(comprador, alvo, oferta, saida, atracao, valor)`, a opção de troca da resposta da IA ao lance
/// (0x8c8090). O vendedor é o clube do contrato do alvo (0x4cf5a0, em [0xae1080]); a LLIST local (0x68d020/0x68d040). O
/// contrato do alvo com o vendedor (0x4cf5f0). Sem a lista de observação do vendedor (o byte +4 do registro de 9 bytes em
/// [[this+0x856]+0xdc]): 0. Senão o elenco do comprador (0x541f90 com 0: 50 vagas), a oferta nova (0x944cf1 de 0x41
/// bytes e 0x8e5680) em *saida, preparada (0x8e78f0: push valor; push comprador; push alvo; mov ecx, oferta), atração 50,
/// +0x1b = valor e o teto = valor × 1,1 (_ftol). Para o alvo (vaga −1, fora do contrato de estatuto 7) e as 25 vagas da
/// lista do vendedor (o +0xf, fora dos jogadores do próprio vendedor), enquanto a troca não tem 3: a posição (0x5b7d20),
/// as marcas (0x4800, ou 0x4900 fora do estatuto 2 e 3; fora do 5, | 0x80 e o alcance) e a nota base. Depois o próprio e
/// os 50 do elenco com a mesma posição: contrato com o clube atual do candidato (0x4cf5f0) de tipo diferente de 5, no
/// comprador, sem ficha (0x4cf6d0), sem os bits 0x30 de +0xb do registro de 79 bytes, piso (0x4d1600 com 0, 1, 1) até o
/// teto; sem oferta (0x8c5cf0), troca (0x8c5e40 na lista local) nem negociação de troca do alvo (0x8c5fc0), ou só a troca
/// desta oferta (+0x20); o vendedor chega a ele (0x8ce990 com 0, 1); interesse (0x846920: push 0; push estatuto; push 0;
/// push 1; push vendedor; push jogador; mov ecx, [this+0x856]) de pelo menos 3,15; a nota dele / a base acima de 0,9: a
/// atração sobe 20 (nota maior) ou 5, entre 0 e 100; fora da troca (+4, criada se faltar), ele entra (0x68d0d0 com 7
/// bytes: id, 0, estatuto), o valor dele (0x840d80 com −1, 1) sai do teto e de +0x1b (no mínimo 0), e a vaga acaba.
/// Devolve 1.
pub(crate) unsafe extern "thiscall" fn make_exchange_option(this: usize, buyer: usize, target: usize, offer: usize, out: *mut usize, atracao: *mut i8, value: i32) -> u32 {
    let list_new: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let destroy_list: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let contract_with: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let reunir: unsafe extern "C" fn(usize, *mut usize, u32) -> u32 = core::mem::transmute(FN_DATABASE_COLLECT_CLUB_SQUAD);
    let alloc: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let new_offer: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_TRANSFER_OFFER_TRANSFER_OFFER);
    let prepare: unsafe extern "thiscall" fn(usize, usize, usize, i32) = core::mem::transmute(FN_TRANSFER_OFFER_INIT_TRANSFER_OFFER);
    let mask: unsafe extern "C" fn(usize) -> u16 = core::mem::transmute(FN_FORMATION_GET_POSITION_MASK);
    let sheet: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_LOAN_CONTRACT);
    let floor_of: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, u32, u32) -> i32 = core::mem::transmute(FN_CONTRACT_MANAGER_CALC_ASKING_PRICE);
    let esvaziar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_CLEAR);
    let offers: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_OFFERS_FOR_PLAYER);
    let swaps: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_EXCHANGE_OFFERS_FOR_PLAYER);
    let negociacoes: unsafe extern "thiscall" fn(usize, usize, usize) -> i32 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_EXCHANGE_NEGOTIATIONS_FOR_PLAYER);
    let chegar: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_CAN_CLUB_BID_FOR_PLAYER);
    let interest: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, usize) -> f64 = core::mem::transmute(FN_SHORTLIST_MANAGER_STAFF_INTEREST_IN_JOINING_CLUB);
    let append: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_LLIST_ADD_TAIL);
    let value_player: unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut i32, i32, u32) -> i32 = core::mem::transmute(FN_SHORTLIST_MANAGER_VALUE_PLAYER_FOR_CLUB);
    let seller = contract_club_any_type(target);
    let mut list = [0u8; 0x10]; // a LLIST local (0xe bytes)
    let pl = list.as_mut_ptr() as usize;
    list_new(pl);
    let cv = contract_with(read_u32_as_usize(G_CONTRACT_MANAGER), target, seller);
    let reg = read_u32_as_usize(read_u32_as_usize(this + transfer_manager_fields::SHORTLIST_MANAGER) + 0xdc).wrapping_add(read_u32_as_usize(seller).wrapping_mul(9));
    if read_u8(reg + club_shortlist::ACTIVE) == 0 { // cqe2: the seller without an active scouting list
        destroy_list(pl);
        return 0;
    }
    let mut squad = [0usize; 50];
    reunir(buyer, squad.as_mut_ptr(), 0);
    let m = alloc(0x41);
    let new = if m != 0 { new_offer(m) } else { 0 };
    *out = new;
    prepare(new, target, buyer, value);
    crate::comparison::branch("cqe_2 opção montada");
    *atracao = 0x32; // cqe2: the attraction of the option
    write_u32_from_usize(new + offer::FEE, value as u32 as usize); // cqe2: the option price
    let mut ceiling = match_pl_ftol(value as f64 * read_f64_at(0x9689c0)); // 0x9689c0 = 1.1
    // simplification: the position, the base rating, the marks and the status carry over from one slot to the next (the original leaves them on the
    // stack without value up to the first slot with record +0x61; a candidate without a record does not happen: they start at zero). The flags
    // outside 0..4 of the reach (which 0x5408c0 does not return) would be the previous ebp, initially the offer pointer.
    let mut position: u16 = 0;
    let mut base: i32 = 0;
    let mut marks: u32 = 0;
    let mut status: u8 = 0;
    let mut ebp: u32 = m as u32;
    let status_of_slot = |i: i8| (read_u8(read_u32_as_usize(reg).wrapping_add((i as i32 * 0x25) as usize) + 0x24) >> 1) & 0xf;
    let mut i: i8 = -1;
    loop {
        let t = read_u32_as_usize(new + offer::EXCHANGE_PLAYERS);
        if t != 0 && read_u16(t) == 3 {
            break;
        }
        'vaga: {
            let cand = if i == -1 {
                if read_u8(cv + contract::SQUAD_STATUS) & 0xf0 == 0x70 {
                    break 'vaga;
                }
                target
            } else {
                let id = read_u32_as_usize(read_u32_as_usize(reg).wrapping_add((i as i32 * 0x25) as usize) + 0xf) as i32;
                if id <= -1 { // cqe2: the slot without a target
                    break 'vaga;
                }
                let p = transfer_manager_staff_by_id(id as usize);
                if read_u32_as_usize(p + 0x39) == seller {
                    break 'vaga;
                }
                crate::comparison::branch("cqe_2 vaga com alvo");
                p
            };
            let cc = contract_with(read_u32_as_usize(G_CONTRACT_MANAGER), cand, contract_club_any_type(cand));
            let mut j: i8 = -1;
            loop {
                'jogador: {
                    let x = if j == -1 {
                        if read_u32_as_usize(cand + staff::PLAYER) == 0 {
                            break 'jogador;
                        }
                        position = mask(read_u32_as_usize(cand + staff::PLAYER));
                        marks = 0x4800;
                        status = if i == -1 { read_u8(cc + contract::SQUAD_STATUS) >> 4 } else { status_of_slot(i) };
                        if status != 2 && status != 3 {
                            marks = 0x4900;
                        }
                        if status == 5 {
                            ebp = marks;
                        } else {
                            marks |= 0x80;
                            ebp = reputation_scope_flags(cand, seller, marks, ebp);
                        }
                        base = score_player_for_seller(cand, position, ebp, seller);
                        cand
                    } else {
                        let x = squad[j as usize];
                        if x == 0 || read_u32_as_usize(x) as i32 >= read_u32_as_usize(G_STAFF_COUNT) as i32 || read_u32_as_usize(x + staff::PLAYER) == 0 {
                            break 'jogador;
                        }
                        if mask(read_u32_as_usize(x + staff::PLAYER)) != position {
                            break 'jogador;
                        }
                        x
                    };
                    let cx = contract_with(read_u32_as_usize(G_CONTRACT_MANAGER), x, contract_club_any_type(cand));
                    if cx == 0 || read_u32_as_usize(x + staff::CLUB_JOB) != buyer || sheet(read_u32_as_usize(G_CONTRACT_MANAGER), x) != 0 {
                        break 'jogador;
                    }
                    if read_u8(read_u32_as_usize(G_STAFF_STATE_TABLE).wrapping_add(read_u32_as_usize(x).wrapping_mul(0x4f)) + 0xb) & 0x30 != 0 || read_u8(cx + contract::CONTRACT_TYPE) & 0x3f == 5 {
                        break 'jogador;
                    }
                    if floor_of(read_u32_as_usize(G_CONTRACT_MANAGER), buyer, 0, x, 0, 1, 1) > ceiling {
                        break 'jogador;
                    }
                    esvaziar(pl);
                    if offers(this, x, 0, 0) != 0 || swaps(this, x, pl) != 0 || negociacoes(this, target, 0) != 0 {
                        if read_u16(pl) != 1 {
                            break 'jogador;
                        }
                        write_u32_from_usize(pl + llist::CURSOR, read_u32_as_usize(pl + llist::HEAD));
                        let p = transfer_manager_llist_next(pl);
                        if read_u32_as_usize(offer + offer::SLOT_INDEX) != read_u32_as_usize(p) {
                            break 'jogador;
                        }
                    }
                    if chegar(this, x, seller, 0, 1) == 0 {
                        break 'jogador;
                    }
                    // simplification: at the target the original pushes the status with the high bytes of ecx that 0x8ce990 left (garbage);
                    // in the slots, with those of 0x25 × slot; 0x846920 receives a char: only the byte counts
                    let st = if i == -1 {
                        (read_u8(cc + contract::SQUAD_STATUS) >> 4) as u32
                    } else {
                        ((i as i32 * 0x25) as u32 & 0xffff_ff00) | status_of_slot(i) as u32
                    };
                    let r = interest(read_u32_as_usize(this + transfer_manager_fields::SHORTLIST_MANAGER), x, seller, 1, 0, st, 0);
                    if !(r >= read_f64_at(0x968e58)) { // cqe2: the interest below 3.15; 0x968e58 = 3.15
                        break 'jogador;
                    }
                    crate::comparison::branch("cqe_2 interesse");
                    if status == 5 {
                        ebp = marks;
                    } else {
                        marks |= 0x80;
                        ebp = reputation_scope_flags(x, seller, marks, ebp);
                    }
                    let nx = score_player_for_seller(x, position, ebp, seller);
                    if nx == 0 || base == 0 {
                        break 'jogador;
                    }
                    if !(nx as f64 / base as f64 > read_f64_at(0x9689d8)) { // cqe2: the rating up to 0.9 of the base; 0x9689d8 = 0.9
                        break 'jogador;
                    }
                    crate::comparison::branch("cqe_2 nota da troca");
                    let a = if nx > base { (*atracao).wrapping_add(20) } else { (*atracao).wrapping_add(5) };
                    *atracao = if a < 0 { 0 } else if a > 100 { 100 } else { a };
                    if read_u32_as_usize(new + offer::EXCHANGE_PLAYERS) == 0 {
                        let m = alloc(0xe);
                        let l = if m != 0 { list_new(m) } else { 0 };
                        write_u32_from_usize(new + offer::EXCHANGE_PLAYERS, l);
                    }
                    let t = read_u32_as_usize(new + offer::EXCHANGE_PLAYERS);
                    write_u32_from_usize(t + llist::CURSOR, read_u32_as_usize(t + llist::HEAD));
                    loop {
                        let p = transfer_manager_llist_next(read_u32_as_usize(new + offer::EXCHANGE_PLAYERS));
                        if p == 0 {
                            break;
                        }
                        if read_u32_as_usize(p) == read_u32_as_usize(x) {
                            break 'jogador;
                        }
                    }
                    // simplification: the 7th byte of the item the original copies from the stack without value; here zero
                    let mut item = [0u8; 8];
                    write_u32_from_usize(item.as_mut_ptr() as usize, read_u32_as_usize(x));
                    item[5] = status;
                    append(read_u32_as_usize(new + offer::EXCHANGE_PLAYERS), item.as_mut_ptr() as usize, 7);
                    let entry = read_u32_as_usize(reg).wrapping_add((i as i32 * 0x25) as usize);
                    let mut away: i32 = 0;
                    let v = value_player(read_u32_as_usize(read_u32_as_usize(G_TRANSFER_MANAGER) + 0x856), seller, x, entry, &mut away, -1, 1);
                    crate::comparison::branch("cqe_2 jogador na troca");
                    ceiling = ceiling.wrapping_sub(v);
                    let r = (read_u32_as_usize(new + offer::FEE) as i32).wrapping_sub(v); // cqe2: his value comes out of the price
                    write_u32_from_usize(new + offer::FEE, if r < 0 { 0 } else { r as u32 as usize });
                    break 'vaga;
                }
                j += 1;
                if j >= 0x32 {
                    break;
                }
            }
        }
        i += 1;
        if i >= 0x19 {
            break;
        }
    }
    destroy_list(pl);
    1
}

/// The byte snapshot of an option offer for the comparer: the 0x41 bytes without the pointer +4, without +0x1a,
/// +0x20..+0x23 and +0x31..+0x34, and only the low nibble of +0x30 (what 0x8e5680 and 0x8e78f0 do not fill keeps
/// whatever was in the new memory), plus the exchange items (id, 0, status: the 7th byte comes from the original's
/// stack).
///
/// [pt] A foto da oferta da opção para o comparador: os 0x41 bytes sem o ponteiro +4, sem +0x1a, +0x20..+0x23 e +0x31..+0x34
/// e só o nibble baixo de +0x30 (o que a 0x8e5680 e a 0x8e78f0 não preenchem fica com o que havia na memória nova), e os
/// itens da troca (id, 0, estatuto: o 7º byte vem da pilha da original).
pub(crate) unsafe fn option_offer_snapshot(o: usize) -> Vec<u8> {
    let mut v: Vec<u8> = (0..0x41).filter(|&k| !(4..8).contains(&k) && k != 0x1a && !(0x20..0x24).contains(&k) && !(0x31..0x35).contains(&k))
        .map(|k| if k == 0x30 { read_u8(o + k) & 0xf } else { read_u8(o + k) }).collect();
    let l = read_u32_as_usize(o + offer::EXCHANGE_PLAYERS);
    if l != 0 {
        v.extend_from_slice(&read_u16(l).to_le_bytes());
        let mut no = read_u32_as_usize(l + llist::HEAD);
        while no != 0 {
            let item = read_u32_as_usize(no);
            v.extend((0..6).map(|k| read_u8(item + k)));
            no = read_u32_as_usize(no + llist_node::NEXT);
        }
    }
    v
}

/// Person -> contract record via the index at `campo` (+0 contract, +4 loan) of their 79-byte staff state record, into
/// the 0x50-byte vector at [vetor]; 0 if id out of range or index negative.
///
/// [pt] O contrato do registro de 79 bytes da pessoa em [0xae2d0c] pelo índice em `campo` (+0 o contrato, +4 o do
/// empréstimo) no vetor de 0x50 bytes em [`vetor`]: id fora do total ([0xae2364]) ou índice negativo, 0.
pub(crate) unsafe fn staff_contract_by_field(person: usize, field: usize, array: usize) -> usize {
    let id = read_u32_as_usize(person) as i32;
    if id >= read_u32_as_usize(G_STAFF_COUNT) as i32 {
        return 0;
    }
    let k = read_u32_as_usize(read_u32_as_usize(G_STAFF_STATE_TABLE).wrapping_add((id as usize).wrapping_mul(0x4f)) + field) as i32;
    if k < 0 { 0 } else { (k as usize).wrapping_mul(0x50).wrapping_add(read_u32_as_usize(array)) }
}

/// Offer status (+0x2c) is 2, 3, 4, 0x10, 0x14, or 0xf with +0x2e >= 3 (the test the original repeats).
///
/// [pt] A oferta segue ativa (status 2, 3, 4, 0x10, 0x14, ou 0xf com +0x2e ≥ 3), o teste que a original repete.
pub(crate) unsafe fn is_offer_status_settled(o: usize) -> bool {
    let s = read_u8(o + offer::STATUS);
    matches!(s, 2 | 3 | 4 | 0x10 | 0x14) || (s == 0xf && read_u8(o + offer::RESPONSE_ROUND) >= 3)
}

/// If the offer's exchange list (+4) has count 0, destroys and frees it and zeroes +4.
///
/// [pt] A lista de troca (+4) vazia (palavra +0 zerada) é destruída (0x68d040: mov ecx, lista -> thiscall, ret 0), liberada
/// (0x944c94: push; add esp, 4 -> cdecl) e +4 zera.
pub(crate) unsafe fn free_empty_exchange_list(o: usize) {
    let l = read_u32_as_usize(o + offer::EXCHANGE_PLAYERS);
    if read_i16(l) != 0 {
        return;
    }
    if l != 0 {
        let destroy: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
        let free: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_OPERATOR_DELETE);
        destroy(l);
        free(l);
    }
    write_u32_from_usize(o + offer::EXCHANGE_PLAYERS, 0);
}

/// The seller accepts the bid: status 5; on loans, target interest below 0.75 gives the refusal to move; exchange
/// players wanting more than 1.5 times the seller's wage cap leave, with their value added to the fee; the offer goes
/// to the next slice.
///
/// Ret 8; SEH frame with no destructor on the normal path (the local STAFF_CONTRACT of 0x85e410):
/// `TRANSFER_MANAGER::accept_transfer_bid(slot, mark)`, the selling club accepts the bid. The target is +0xc, the buyer
/// +8 and the seller the club of the target's contract (except type 5). +0x2e = 0; with `mark`, +0x2d = 1 and the
/// player marked as under offer in the scouting list (0x84b580: push +0 of the offer; push buyer; push target; mov ecx,
/// `[this+0x856]` -> thiscall, ret 0xc). +0x2e = 0, status 5; the count of human managers (0x7875e0, stdcall: ecx =
/// `[0xb7431c]` is not read) between buyer and seller; the acceptance news (0x8d76d0).
///
/// - Loan (+0x11 = 0xb): the status the buyer would give (1 if human; otherwise 0x849f00: push 0; push buyer; push
/// target, ret 0xc) goes in the low byte of the `mark` dword, and the target's interest (0x846920: push &reason; push
/// that dword; push 0; push 0; push buyer; push target; mov ecx, `[this+0x856]` -> thiscall, ret 0x18, returns ST0)
/// below 0.75 (or NaN): the refusal to move (0x8d8310: push reason; push offer, ret 8), status 0xf, +0x2e = 3, the bids
/// unmarked (0x84aeb0), 10 days (0x8c6220), with the offer active and closed together with those of list +0 (0x8d0d90,
/// stdcall) bit 0 of the target's 8-byte record in `[this+0x86e]` cleared, and the status in the scouting list
/// (0x849ce0: push 0; push type ≠ 0xb; push target; push buyer, ret 0x10; found, 0x843e10: push the dword with the low
/// byte = role ≠ 0xb and the high bytes from the entry pointer; push 1; push buyer; push target, ret 0x10). End.
///
/// - Otherwise, each exchange player (+4): human seller, +5 = 2; his desired contract (0x85fc50: push 0; push 0; push
/// 0xb; push +5 (only the byte is read); push −1; push seller; push person; lea ecx, contract -> thiscall, ret 0x1c)
/// and the seller's wage cap (0x5a0590: push seller; mov ecx, `[0xae2a38]`, ret 4; 0x5968d0: push +5; push person; mov
/// ecx, finances, ret 8): cap × 1.5 below the wage, he leaves the exchange: his value (0x4d0c60: push 0; push −1; push
/// 0; push buyer; push person; mov ecx, `[0xae1050]`, ret 0x14) is added to +0x1b, the news (0x8d8910: push value; push
/// person; push offer, ret 0xc), removal from the list (0x68d1f0: push node; mov ecx, list, ret 4), byte +2 of his
/// record in `[this+0x86e]` drops 1; with an AI seller, the AI's answer (0x8c8090: push 1; push slot, ret 8) and, if
/// the offer is active, the empty exchange is freed and end. At the end of the list, the empty exchange is freed.
///
/// - Human manager: the contract talks news (0x8d7cd0); the offer rescheduled for the next slice (0x8c6220: push 1;
/// push 0); with an exchange and an AI seller, each exchange player's entry in the seller's list (0x849ce0 with 0, 0)
/// receives the offer's index +0x20 in +0x17.
/// Original: `TRANSFER_MANAGER::accept_transfer_bid` (0x8c0080, transfer_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x8c0080 } (thiscall, ret 8, transfer_manager.cpp, quadro SEH sem destrutor no caminho normal: o STAFF_CONTRACT
/// local da 0x85e410): `TRANSFER_MANAGER::accept_transfer_bid(vaga, marcar)`, o clube vendedor aceita o lance. O alvo é
/// o +0xc, o comprador o +8 e o vendedor o clube do contrato do alvo (fora o tipo 5). +0x2e = 0; com `marcar`, +0x2d = 1
/// e o jogador sob oferta na lista de observação (0x84b580: push +0 da oferta; push comprador; push alvo; mov ecx,
/// [this+0x856] -> thiscall, ret 0xc). +0x2e = 0, status 5; a conta dos técnicos humanos (0x7875e0, stdcall: o ecx =
/// [0xb7431c] não é lido) entre comprador e vendedor; a notícia da aceitação (0x8d76d0).
/// - Empréstimo (+0x11 = 0xb): o estatuto que o comprador daria (1 se humano; senão 0x849f00: push 0; push comprador;
///   push alvo, ret 0xc) vai no byte baixo do dword de `marcar`, e o interesse do alvo (0x846920: push &motivo; push
///   esse dword; push 0; push 0; push comprador; push alvo; mov ecx, [this+0x856] -> thiscall, ret 0x18, devolve ST0)
///   abaixo de 0,75 (ou NaN): a recusa em se mudar (0x8d8310: push motivo; push oferta, ret 8), status 0xf, +0x2e = 3, os
///   lances desmarcados (0x84aeb0), 10 dias (0x8c6220), com a oferta ativa e fechada com as da lista +0 (0x8d0d90,
///   stdcall) o bit 0 do registro de 8 bytes do alvo em [this+0x86e] apagado, e a situação na lista de observação
///   (0x849ce0: push 0; push tipo ≠ 0xb; push alvo; push comprador, ret 0x10; achada, 0x843e10: push o dword com o byte
///   baixo = cargo ≠ 0xb e os altos do ponteiro da entrada; push 1; push comprador; push alvo, ret 0x10). Fim.
/// - Senão, cada jogador da troca (+4): vendedor humano, +5 = 2; o contrato desejado dele (0x85fc50: push 0; push 0;
///   push 0xb; push +5 (só o byte é lido); push −1; push vendedor; push pessoa; lea ecx, contrato ->
///   thiscall, ret 0x1c) e o teto salarial do vendedor (0x5a0590: push vendedor; mov ecx, [0xae2a38], ret 4; 0x5968d0:
///   push +5; push pessoa; mov ecx, finanças, ret 8): teto × 1,5 abaixo do salário, ele sai da troca: o valor dele
///   (0x4d0c60: push 0; push −1; push 0; push comprador; push pessoa; mov ecx, [0xae1050], ret 0x14) soma ao +0x1b, a
///   notícia (0x8d8910: push valor; push pessoa; push oferta, ret 0xc), a saída da lista (0x68d1f0: push nó; mov ecx,
///   lista, ret 4), o byte +2 do registro dele em [this+0x86e] cai 1; com vendedor da IA, a resposta da IA (0x8c8090:
///   push 1; push vaga, ret 8) e, ativa a oferta, a troca vazia é solta e fim. Ao fim da lista, a troca vazia é solta.
/// - Técnico humano: a notícia da conversa de contrato (0x8d7cd0); a oferta reagendada para a fatia seguinte (0x8c6220:
///   push 1; push 0); com troca e vendedor da IA, a entrada de cada jogador da troca na lista do vendedor (0x849ce0 com
///   0, 0) recebe o índice +0x20 da oferta em +0x17.
pub(crate) unsafe extern "thiscall" fn accept_transfer_bid(this: usize, slot: usize, mark: u32) {
    let new_contract: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_STAFF_CONTRACT_STAFF_CONTRACT);
    let sob_offer: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(FN_SHORTLIST_MANAGER_MARK_PLAYER_UNDER_OFFER);
    let human_manager_record_checked: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(FN_NEWS_CLUB_HAS_HUMAN_MANAGER);
    let accepted: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_POST_OFFER_ACCEPTED_NEWS);
    let status: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u8 = core::mem::transmute(FN_SHORTLIST_MANAGER_GET_SQUAD_STATUS_FOR_CLUB);
    let interest: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, *mut u32) -> f64 = core::mem::transmute(FN_SHORTLIST_MANAGER_STAFF_INTEREST_IN_JOINING_CLUB);
    let recusa_mudar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_POST_PLAYER_REJECTS_MOVE_NEWS);
    let unmark: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_SHORTLIST_MANAGER_CLEAR_BID_MARKERS_FOR_PLAYER);
    let schedule: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_SCHEDULE_OFFER);
    let fechada: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(FN_TRANSFER_OFFER_IS_CLOSED);
    let entry: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(FN_SHORTLIST_MANAGER_FIND_CLUB_SHORTLIST_ENTRY);
    let situacao: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u8 = core::mem::transmute(FN_SHORTLIST_MANAGER_UPDATE_STAFF_STATUS);
    let desejado: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32, u32) -> i32 = core::mem::transmute(FN_STAFF_CONTRACT_SET_TO_DESIRED_CONTRACT);
    let finances: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FINANCETABLE_GET_CLUB_FINANCE);
    let ceiling: unsafe extern "thiscall" fn(usize, usize, u32) -> i32 = core::mem::transmute(FN_FINANCE_GET_MAX_WAGE);
    let value: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) -> i32 = core::mem::transmute(FN_CONTRACT_MANAGER_CALC_PLAYER_VALUE);
    let news_swap: unsafe extern "thiscall" fn(usize, usize, usize, i32) = core::mem::transmute(FN_TRANSFER_MANAGER_POST_SWAP_PLAYER_REJECTS_NEWS);
    let remove: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_LLIST_REMOVE);
    let reply_ia: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_AI_RESPOND_TO_BID);
    let conversa: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_POST_CONTRACT_TALKS_NEWS);
    let mut contract = [0u8; 0x60];
    let pc = contract.as_mut_ptr() as usize;
    new_contract(pc);
    let o0 = read_u32_as_usize(slot);
    let target = transfer_manager_staff_by_id(read_u32_as_usize(o0 + offer::TARGET_STAFF_ID));
    let buyer = transfer_manager_club_by_id(read_u32_as_usize(o0 + offer::BUYER_CLUB_ID));
    let seller = contract_club_filtered(target, true);
    let o = read_u32_as_usize(slot);
    write_u8(o + offer::RESPONSE_ROUND, 0);
    if mark != 0 {
        write_u8(o + offer::PLAYER_AWARE, 1);
        sob_offer(read_u32_as_usize(this + transfer_manager_fields::SHORTLIST_MANAGER), target, buyer, read_u32_as_usize(read_u32_as_usize(slot)));
    }
    write_u8(o + offer::RESPONSE_ROUND, 0);
    write_u8(o + offer::STATUS, 5);
    let mut humanos: u8 = 0;
    if human_manager_record_checked(transfer_manager_club_by_id(read_u32_as_usize(o + offer::BUYER_CLUB_ID))) != 0 {
        humanos = 1;
    }
    if seller != 0 && human_manager_record_checked(seller) != 0 {
        humanos = humanos.wrapping_add(1);
    }
    accepted(this, o);
    if read_u8(o + offer::JOB_TYPE) == 0xb {
        let e = if transfer_manager_is_human_club(buyer) { 1 } else { status(read_u32_as_usize(this + transfer_manager_fields::SHORTLIST_MANAGER), target, buyer, 0) };
        let dw = (mark & 0xffff_ff00) | e as u32;
        let mut reason: u32 = 0;
        let r = interest(read_u32_as_usize(this + transfer_manager_fields::SHORTLIST_MANAGER), target, buyer, 0, 0, dw, &mut reason);
        crate::comparison::branch("cqd_2 empréstimo");
        if !(r >= 0.75) { // cqd2: the target does not want to move
            recusa_mudar(this, o, reason);
            write_u8(o + offer::STATUS, 0xf);
            write_u8(o + offer::RESPONSE_ROUND, 3);
            let a2 = transfer_manager_staff_by_id(read_u32_as_usize(o + offer::TARGET_STAFF_ID));
            let current = contract_club_filtered(a2, false);
            unmark(read_u32_as_usize(this + transfer_manager_fields::SHORTLIST_MANAGER), transfer_manager_club_by_id(read_u32_as_usize(o + offer::BUYER_CLUB_ID)), a2);
            let l = read_u32_as_usize(o + offer::EXCHANGE_PLAYERS);
            if l != 0 {
                write_u32_from_usize(l + llist::CURSOR, read_u32_as_usize(l + llist::HEAD));
                loop {
                    let p = transfer_manager_llist_next(read_u32_as_usize(o + offer::EXCHANGE_PLAYERS));
                    if p == 0 {
                        break;
                    }
                    unmark(read_u32_as_usize(this + transfer_manager_fields::SHORTLIST_MANAGER), current, transfer_manager_staff_by_id(read_u32_as_usize(p)));
                }
            }
            schedule(this, o, 0xa, 0);
            'fechar: {
                if !is_offer_status_settled(o) {
                    break 'fechar;
                }
                let l = read_u32_as_usize(o);
                if l != 0 {
                    write_u32_from_usize(l + llist::CURSOR, read_u32_as_usize(l + llist::HEAD));
                    loop {
                        let p = transfer_manager_llist_next(read_u32_as_usize(o));
                        if p == 0 {
                            break;
                        }
                        if fechada(read_u32_as_usize(read_u32_as_usize(this).wrapping_add(read_u32_as_usize(p).wrapping_mul(4)))) == 0 {
                            break 'fechar;
                        }
                    }
                }
                let m = read_u32_as_usize(this + transfer_manager_fields::PERSON_FLAGS).wrapping_add(read_u32_as_usize(o + offer::TARGET_STAFF_ID).wrapping_mul(8));
                if read_u8(m) & 1 != 0 {
                    write_u8(m, read_u8(m) & 0xfe);
                }
            }
            let ne = (read_u8(o + offer::JOB_TYPE) != 0xb) as u32;
            let x = entry(read_u32_as_usize(this + transfer_manager_fields::SHORTLIST_MANAGER), transfer_manager_club_by_id(read_u32_as_usize(o + offer::BUYER_CLUB_ID)), transfer_manager_staff_by_id(read_u32_as_usize(o + offer::TARGET_STAFF_ID)), ne, 0);
            if x != 0 {
                // simplification: the original pushes the setne with the high bytes of the entry pointer; 0x843e10 only reads the byte
                let t = (x as u32 & 0xffff_ff00) | (read_u8(x) != 0xb) as u32;
                situacao(read_u32_as_usize(this + transfer_manager_fields::SHORTLIST_MANAGER), transfer_manager_staff_by_id(read_u32_as_usize(o + offer::TARGET_STAFF_ID)), transfer_manager_club_by_id(read_u32_as_usize(o + offer::BUYER_CLUB_ID)), 1, t);
            }
            return;
        }
    }
    if read_u32_as_usize(o + offer::EXCHANGE_PLAYERS) != 0 {
        let l = read_u32_as_usize(o + offer::EXCHANGE_PLAYERS);
        write_u32_from_usize(l + llist::CURSOR, read_u32_as_usize(l + llist::HEAD));
        let mut p = transfer_manager_llist_next(read_u32_as_usize(o + offer::EXCHANGE_PLAYERS));
        while p != 0 {
            let person = transfer_manager_staff_by_id(read_u32_as_usize(p));
            if transfer_manager_is_human_club(seller) {
                write_u8(p + 5, 2);
            }
            let job = read_u8(p + 5);
            // simplification: the original pushes edx with byte +5 and the high bytes of what 0x600a50 left in it; 0x85fc50
            // reads only the byte (mov cl, [esp+0x284])
            let salary = desejado(pc, person, seller, u32::MAX, job as u32, 0xb, 0, 0);
            let t = ceiling(finances(read_u32_as_usize(G_FINANCE_TABLE), seller), person, job as u32);
            crate::comparison::branch("cqd_2 troca cara");
            if (t as f64 * 1.5) < salary as f64 { // cqd2: the exchange player too expensive
                let v = value(read_u32_as_usize(G_CONTRACT_MANAGER), person, buyer, 0, u32::MAX, 0).wrapping_add(read_u32_as_usize(o + offer::FEE) as i32);
                news_swap(this, o, person, v);
                remove(read_u32_as_usize(o + offer::EXCHANGE_PLAYERS), p);
                let m = read_u32_as_usize(this + transfer_manager_fields::PERSON_FLAGS).wrapping_add(read_u32_as_usize(person).wrapping_mul(8)) + 2;
                write_u8(m, read_u8(m).wrapping_sub(1));
                write_u32_from_usize(o + offer::FEE, v as u32 as usize);
                if seller != 0 && !transfer_manager_is_human_club(seller) {
                    reply_ia(this, slot, 1);
                    if is_offer_status_settled(o) {
                        free_empty_exchange_list(o);
                        return;
                    }
                }
            }
            p = transfer_manager_llist_next(read_u32_as_usize(o + offer::EXCHANGE_PLAYERS));
        }
        free_empty_exchange_list(o);
    }
    if humanos != 0 {
        conversa(this, o);
    }
    crate::comparison::branch("cqd_2 lance aceito");
    schedule(this, o, 0, 1); // cqd2: a próxima fatia
    if read_u32_as_usize(o + offer::EXCHANGE_PLAYERS) == 0 || transfer_manager_is_human_club(seller) {
        return;
    }
    let l = read_u32_as_usize(o + offer::EXCHANGE_PLAYERS);
    write_u32_from_usize(l + llist::CURSOR, read_u32_as_usize(l + llist::HEAD));
    loop {
        let p = transfer_manager_llist_next(read_u32_as_usize(o + offer::EXCHANGE_PLAYERS));
        if p == 0 {
            break;
        }
        let x = entry(read_u32_as_usize(this + transfer_manager_fields::SHORTLIST_MANAGER), seller, transfer_manager_staff_by_id(read_u32_as_usize(p)), 0, 0);
        if x != 0 {
            write_u32_from_usize(x + 0x17, read_u32_as_usize(o + offer::SLOT_INDEX));
        }
    }
}

/// Sends news 0xfa0 of an offer made (target, clubs, values, exchange, contract, window) to the interested human
/// managers.
///
/// Ret 4; SEH frame with no destructor on the normal path (the local news item and date):
/// `TRANSFER_MANAGER::post_offer_made_news(offer)`. The seller is the club of the target's contract (except type 5).
/// The recipients (0x8d1130: push 0x1e; push &vector; push 0x3e; push offer; mov ecx, this -> thiscall, ret 0x10);
/// none: nothing. News 0xfa0 (0x77bbe0: push 0; push 0xfa0; lea ecx, news, ret 8; the data through 0x785790: push v;
/// push k; lea ecx, news, ret 8): 0 the target's id, 1..3 his +0x33 of +4/+8/+0xc, 4 the buyer (+8), 5 the seller's id
/// (−1 if none), 6 +0x20, 7 +0x24, 8 +0x1b, 9 the status, 0xa the value (0x8e7a70: push 1; push 0; push 1; push 0; mov
/// ecx, offer -> thiscall(offer, 0, 1, 0, 1), ret 0x10), 0xb +0x2f, 0xe +0x31 & 0xf. With +0x2f, a seller and the
/// target's role 11..15 (+0x3d; 0x53b620, cdecl): through the target's contract (0x4cf5a0, without storing into
/// `[0xae1080]`) its date (+0x2d/+0x2f) if it has not arrived yet (otherwise 0, 0) in 0xc/0xd and the type (+0x35, 6
/// signed bits) in 0xf; without a contract, 0, 0, 0. Each exchange player (+4): the first 3 give the id and the 3 +0x33
/// values starting at 0x18; 0x17 the count. The current person (the last of the exchange, or the target) with a
/// national side (+0x24) that is a second team (0x5409b0, cdecl): 0x11 its id and flag 1; otherwise −1. The record
/// (0x4486c0: push 1; push offer; mov ecx, `[0xad9dac]` -> thiscall, ret 8) adds flag 2: 0x10 the flag; the next free
/// index receives +0x11. The news to each recipient (0x7861e0: push club; push &news; mov ecx, `[0xb7431c]` ->
/// thiscall, ret 8). With +0x2f: +0x30 & 0xf = 5, the fixed window (0x8d0bb0, stdcall(&date, 0): ecx is not read) gives
/// 0x12 = 0 if today is the date, otherwise 5, and the date in 0xc/0xd; otherwise 0x12 = 3.
/// Original: `TRANSFER_MANAGER::post_offer_made_news` (0x8d3090, transfer_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x8d3090 } (thiscall, ret 4, transfer_manager.cpp, quadro SEH sem destrutor no caminho normal: a notícia e a data
/// locais): `TRANSFER_MANAGER::post_offer_made_news(oferta)`. O vendedor é o clube do contrato do alvo (fora o tipo 5).
/// Os destinatários (0x8d1130: push 0x1e; push &vetor; push 0x3e; push oferta; mov ecx, this -> thiscall, ret 0x10);
/// nenhum: nada. A notícia 0xfa0 (0x77bbe0: push 0; push 0xfa0; lea ecx, notícia, ret 8; os dados pela 0x785790: push
/// v; push k; lea ecx, notícia, ret 8): 0 o id do alvo, 1..3 o +0x33 de +4/+8/+0xc dele, 4 o comprador (+8), 5 o id do
/// vendedor (−1 sem), 6 +0x20, 7 +0x24, 8 +0x1b, 9 o status, 0xa o valor (0x8e7a70: push 1; push 0; push 1; push 0; mov
/// ecx, oferta -> thiscall(oferta, 0, 1, 0, 1), ret 0x10), 0xb +0x2f, 0xe +0x31 & 0xf. Com +0x2f, vendedor e cargo 11..15
/// do alvo (+0x3d; 0x53b620, cdecl): pelo contrato do alvo (0x4cf5a0, sem guardar em [0xae1080]) a data dele (+0x2d/
/// +0x2f) se ainda não chegou (senão 0, 0) em 0xc/0xd e o tipo (+0x35, 6 bits com sinal) em 0xf; sem contrato, 0, 0, 0.
/// Cada jogador da troca (+4): os 3 primeiros dão o id e os 3 +0x33 a partir de 0x18; 0x17 a conta. A pessoa da vez (o
/// último da troca, ou o alvo) com seleção (+0x24) segunda seleção (0x5409b0, cdecl): 0x11 o id dela e a marca 1; senão
/// −1. O recorde (0x4486c0: push 1; push oferta; mov ecx, [0xad9dac] -> thiscall, ret 8) soma a marca 2: 0x10 a marca; o
/// próximo índice livre recebe o +0x11. A notícia a cada destinatário (0x7861e0: push clube; push &notícia; mov ecx,
/// [0xb7431c] -> thiscall, ret 8). Com +0x2f: +0x30 & 0xf = 5, a janela fixa (0x8d0bb0, stdcall(&data, 0): o ecx não é
/// lido) dá 0x12 = 0 se hoje é a data, senão 5, e a data em 0xc/0xd; senão 0x12 = 3.
pub(crate) unsafe extern "thiscall" fn post_offer_made_news(this: usize, o: usize) {
    let recipients: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32) -> u8 = core::mem::transmute(FN_TRANSFER_MANAGER_COLLECT_OFFER_NEWS_RECIPIENTS);
    let construct: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let write: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let total: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> i32 = core::mem::transmute(FN_TRANSFER_OFFER_GET_TOTAL_VALUE);
    let kind: unsafe extern "C" fn(i8) -> u32 = core::mem::transmute(FN_DATABASE_IS_PLAYER_JOB);
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let second: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_CLUB_IS_NATIONAL_TEAM);
    let recorde: unsafe extern "thiscall" fn(usize, usize, u32) -> u8 = core::mem::transmute(FN_CLUB_RECORDS_OFFER_BREAKS_FEE_RECORD);
    let deliver: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    let default: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    let window: unsafe extern "stdcall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_GET_NEXT_WINDOW_DATES);
    let mut target = transfer_manager_staff_by_id(read_u32_as_usize(o + offer::TARGET_STAFF_ID));
    let seller = contract_club_filtered(target, true);
    let mut clubs = [0usize; 0x1e];
    let n = recipients(this, o, 0x3e, clubs.as_mut_ptr() as usize, 0x1e);
    if n == 0 {
        return;
    }
    let mut news = [0u32; 0x38];
    let pn = news.as_mut_ptr() as usize;
    construct(pn, 0xfa0, 0);
    let g = |k: i8, v: u32| { write(pn, k, v); };
    g(0, read_u32_as_usize(target) as u32);
    g(1, read_u32_as_usize(read_u32_as_usize(target + staff::FIRST_NAME) + 0x33) as u32);
    g(2, read_u32_as_usize(read_u32_as_usize(target + staff::SECOND_NAME) + 0x33) as u32);
    g(3, read_u32_as_usize(read_u32_as_usize(target + staff::COMMON_NAME) + 0x33) as u32);
    g(4, read_u32_as_usize(o + offer::BUYER_CLUB_ID) as u32);
    g(5, if seller != 0 { read_u32_as_usize(seller) as u32 } else { u32::MAX });
    g(6, read_u32_as_usize(o + offer::SLOT_INDEX) as u32);
    g(7, read_u32_as_usize(o + offer::SERIAL) as u32);
    g(8, read_u32_as_usize(o + offer::FEE) as u32);
    g(9, read_u8(o + offer::STATUS) as i8 as i32 as u32);
    g(0xa, total(o, 0, 1, 0, 1) as u32);
    g(0xb, read_u8(o + offer::PRE_CONTRACT) as i8 as i32 as u32);
    g(0xe, (read_u32_as_usize(o + offer::OFFER_KIND) & 0xf) as u32);
    if read_u8(o + offer::PRE_CONTRACT) != 0 && seller != 0 && kind(read_u8(target + staff::JOB_FOR_CLUB) as i8) != 0 {
        let c = contract_of(read_u32_as_usize(G_CONTRACT_MANAGER), target);
        if c != 0 {
            let (ha, ca) = (read_i16(G_CURRENT_DATE_YEAR) as u16, read_i16(c + 0x2f) as u16);
            if ha < ca || (ha == ca && read_i16(G_CURRENT_DATE) < read_i16(c + contract::CONTRACT_EXPIRES)) {
                g(0xc, read_i16(c + contract::CONTRACT_EXPIRES) as i32 as u32);
                g(0xd, ca as u32);
            } else {
                g(0xc, 0);
                g(0xd, 0);
            }
            g(0xf, (((read_u8(c + contract::CONTRACT_TYPE) << 2) as i8) >> 2) as i32 as u32);
        } else {
            g(0xc, 0);
            g(0xd, 0);
            g(0xf, 0);
        }
    }
    let mut k: u8 = 0x18;
    let mut count: u8 = 0;
    let l = read_u32_as_usize(o + offer::EXCHANGE_PLAYERS);
    if l != 0 {
        write_u32_from_usize(l + llist::CURSOR, read_u32_as_usize(l + llist::HEAD));
        loop {
            let p = transfer_manager_llist_next(read_u32_as_usize(o + offer::EXCHANGE_PLAYERS));
            if p == 0 {
                break;
            }
            target = transfer_manager_staff_by_id(read_u32_as_usize(p));
            if (count as i8) < 3 {
                g(k as i8, read_u32_as_usize(target) as u32);
                g(k.wrapping_add(1) as i8, read_u32_as_usize(read_u32_as_usize(target + staff::FIRST_NAME) + 0x33) as u32);
                g(k.wrapping_add(2) as i8, read_u32_as_usize(read_u32_as_usize(target + staff::SECOND_NAME) + 0x33) as u32);
                g(k.wrapping_add(3) as i8, read_u32_as_usize(read_u32_as_usize(target + staff::COMMON_NAME) + 0x33) as u32);
                k = k.wrapping_add(4);
            }
            count = count.wrapping_add(1);
        }
    }
    g(0x17, count as i8 as i32 as u32);
    let mut mark: u8 = 0;
    let s = read_u32_as_usize(target + staff::NATIONAL_JOB);
    if s != 0 && second(s) != 0 {
        mark = 1;
        g(0x11, read_u32_as_usize(read_u32_as_usize(target + staff::NATIONAL_JOB)) as u32);
    } else {
        g(0x11, u32::MAX);
    }
    if recorde(read_u32_as_usize(G_CLUB_RECORDS), o, 1) != 0 {
        mark |= 2;
    }
    g(0x10, mark as i8 as i32 as u32);
    g(k as i8, read_u8(o + offer::JOB_TYPE) as i8 as i32 as u32);
    for i in 0..(n as i8).max(0) as usize {
        deliver(read_u32_as_usize(G_NEWS), pn, clubs[i]);
    }
    if read_u8(o + offer::PRE_CONTRACT) == 0 {
        return;
    }
    if read_u8(o + offer::COMPLETION_TIMING) & 0xf != 5 {
        g(0x12, 3);
        return;
    }
    let mut data = [0u8; 8];
    let pd = data.as_mut_ptr() as usize;
    default(pd);
    window(pd, 0);
    let today_same = read_i16(G_CURRENT_DATE_YEAR) == read_i16(pd + cm_date::YEAR) && read_i16(G_CURRENT_DATE) == read_i16(pd);
    g(0x12, if today_same { 0 } else { 5 });
    g(0xc, read_i16(pd) as i32 as u32);
    g(0xd, read_i16(pd + cm_date::YEAR) as u16 as u32);
}

/// Returns the virtual method at vtable +0x20 of a nation's transfer rules object (4 arguments).
///
/// [pt] O virtual +0x20 da regra de nação `r` (push 0; push mensagem; push jogador; push clube; mov ecx, regra: 4 dwords).
pub(crate) unsafe fn rules_vfunc_20(r: usize) -> unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) -> u32 {
    core::mem::transmute(read_u32_as_usize(read_u32_as_usize(r) + 0x20))
}

/// Returns the virtual method at vtable +0x24 of a nation's transfer rules object (5 arguments).
///
/// [pt] O virtual +0x24 da regra de nação `r` (push 1; push &data; push 0; push jogador; push clube; mov ecx, regra: 5 dwords).
pub(crate) unsafe fn rules_vfunc_24(r: usize) -> unsafe extern "thiscall" fn(usize, usize, usize, u32, usize, u32) -> u32 {
    core::mem::transmute(read_u32_as_usize(read_u32_as_usize(r) + 0x24))
}

/// Tells whether the club can take the player on loan: current club, contract record, playing role, whether the club
/// can reach him, the rule of the club's nation and of the current club month by month over the loan period; the reason
/// text in the message.
///
/// Ret 0xc; SEH frame without a destructor (the local date and texts): `TRANSFER_MANAGER::can_loan_player(club, player,
/// message)`, 1 can and 0 cannot. The current club is that of the player's contract (0x4cf5a0, in `[0xae1080]`; except
/// type 5); the default local date (0x54a110) and today's day of the month (0x54cf70: mov ecx, 0xae2c90 -> thiscall,
/// ret 0). No current club, or with a contract record (0x4cf6d0: push player; mov ecx, `[0xae1050]`, ret 4): 0. Role
/// (+0x3d) other than 0xb: 0, with the name text (0x53dba0: push 1; push 0x12c; push text; push player; add esp, 0x10)
/// in `[0xde1f64]` (0x66f4e0, cdecl) if there is a message. If the club cannot reach the player (0x8ce990: push 0; push
/// &reason; push club; push player; mov ecx, this, ret 0x10): 0 and, with a message, the text for the reason (0x21 the
/// club of the agreed offer, 0x4cca30, or the current one, clearing bit 0x40 of the 8-byte record in
/// `[[0xde1f54]+0x86e]` without an offer; 0x27, 7 and 1 in the message; 0x12/0x13; 0x19, 0x28 and 0x1a with the
/// nation's adjective, 0x6181d0: push 0xd; push 0x64; push text; push nation; add esp, 0x10). The rule of the club's
/// nation (of the 28 at this+0x8ac, the one whose +0xa equals `[club+0x53]`): virtual +0x20 (push 0; push message; push
/// player; push club; mov ecx, rule) equal to 0 gives 0. The rule of the current club's nation: without it, 1. The loan
/// period (0x8d0590: push &end_year; push &end_month; push &start_year; push &start_month; push player; push contract
/// club; push club; mov ecx, this -> thiscall, ret 0x1c): start equal to end gives 1. Otherwise the date (day of the
/// month, start; 0x54a130: push −1; push year; push month; push day; lea ecx, date -> thiscall, ret 0x10) and virtual
/// +0x24 (push 1; push &date; push 0; push player; push current club; mov ecx, rule) different from 0 gives 1; it
/// advances one month (0x549f70: push −1; push end_year; push end_month; push day; lea ecx, local -> thiscall, ret
/// 0x10) while the end date is after the date, again 0x54a130 and the virtual (different from 0 gives 1); once past: 0,
/// with "This loan deal cannot go ahead" in the message.
/// Original: `TRANSFER_MANAGER::can_loan_player` (0x8d0040, transfer_manager.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x8d0040 } (thiscall, ret 0xc, transfer_manager.cpp, quadro SEH sem destrutor: a data e os textos locais):
/// `TRANSFER_MANAGER::can_loan_player(clube, jogador, mensagem)`, 1 pode e 0 não pode. O clube atual é o do contrato
/// do jogador (0x4cf5a0, em [0xae1080]; fora o tipo 5); a data local padrão (0x54a110) e o dia do mês de hoje (0x54cf70:
/// mov ecx, 0xae2c90 -> thiscall, ret 0). Sem clube atual, ou com ficha de contrato (0x4cf6d0: push jogador; mov ecx,
/// [0xae1050], ret 4): 0. Cargo (+0x3d) diferente de 0xb: 0, com o texto do nome (0x53dba0: push 1; push 0x12c; push
/// texto; push jogador; add esp, 0x10) em [0xde1f64] (0x66f4e0, cdecl) se há mensagem. Se o clube não pode chegar ao
/// jogador (0x8ce990: push 0; push &motivo; push clube; push jogador; mov ecx, this, ret 0x10): 0 e, com mensagem, o
/// texto pelo motivo (0x21 o clube da oferta acertada, 0x4cca30, ou o atual, apagando o bit 0x40 do registro de 8
/// bytes em [[0xde1f54]+0x86e] sem oferta; 0x27, 7 e 1 na mensagem; 0x12/0x13; 0x19, 0x28 e 0x1a com o adjetivo da
/// nação, 0x6181d0: push 0xd; push 0x64; push texto; push nação; add esp, 0x10). A regra da nação do clube (das 28 em
/// this+0x8ac, a de +0xa igual a [clube+0x53]): o virtual +0x20 (push 0; push mensagem; push jogador; push clube; mov
/// ecx, regra) igual a 0 dá 0. A regra da nação do clube atual: sem ela, 1. O período do empréstimo (0x8d0590: push
/// &fim_ano; push &fim_mês; push &início_ano; push &início_mês; push jogador; push clube do contrato; push clube; mov
/// ecx, this -> thiscall, ret 0x1c): início igual ao fim dá 1. Senão a data (dia do mês, início; 0x54a130: push −1;
/// push ano; push mês; push dia; lea ecx, data -> thiscall, ret 0x10) e o virtual +0x24 (push 1; push &data; push 0;
/// push jogador; push clube atual; mov ecx, regra) diferente de 0 dá 1; avança um mês (0x549f70: push −1; push fim_ano;
/// push fim_mês; push dia; lea ecx, local -> thiscall, ret 0x10) enquanto a data do fim passar da data, de novo a 0x54a130
/// e o virtual (diferente de 0 dá 1); passou: 0, com "This loan deal cannot go ahead" na mensagem.
pub(crate) unsafe extern "thiscall" fn transfer_manager_can_loan_player(this: usize, club: usize, player: usize, msg: usize) -> u32 {
    let default: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    let day_of_month: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_DATE_GET_DAY_OF_MONTH);
    let sheet: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_LOAN_CONTRACT);
    let name: unsafe extern "C" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_DATABASE_FORMAT_STAFF_NAME);
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let chegar: unsafe extern "thiscall" fn(usize, usize, usize, *mut u16, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_CAN_CLUB_BID_FOR_PLAYER);
    let acertada: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_FIND_AGREED_CONTRACT_OFFER);
    let adjetivo: unsafe extern "C" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_NATION_GET_NATIONALITY_ADJECTIVE);
    let period: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_GET_LOAN_PERIOD);
    let mark: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_DATE_SET);
    let dma: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE_549F70);
    let current = contract_club_filtered(player, true);
    let mut data = [0u8; 8];
    let pd = data.as_mut_ptr() as usize;
    default(pd);
    let day = day_of_month(G_CURRENT_DATE);
    if current == 0 {
        return 0;
    }
    if sheet(read_u32_as_usize(G_CONTRACT_MANAGER), player) != 0 {
        return 0;
    }
    let mut buf_name = [0u8; 0x130];
    let pn = buf_name.as_mut_ptr() as usize;
    if read_u8(player + staff::JOB_FOR_CLUB) != 0xb {
        if msg != 0 {
            name(player, pn, 0x12c, 1);
            text(0xde1f64, 0xa8f4dc, pn);
        }
        return 0;
    }
    let mut reason: u16 = 0;
    crate::comparison::branch("cqd_2 clube chega");
    if chegar(this, player, club, &mut reason, 0) == 0 { // cqd2: the club cannot reach the player
        if msg == 0 {
            return 0;
        }
        name(player, pn, 0x12c, 1);
        let mut adj = [0u8; 0x68];
        let pa = adj.as_mut_ptr() as usize;
        let nation = match reason {
            0x21 => {
                let gm = read_u32_as_usize(G_TRANSFER_MANAGER);
                let id = read_u32_as_usize(player);
                let m = read_u32_as_usize(gm + transfer_manager_fields::PERSON_FLAGS).wrapping_add(id.wrapping_mul(8));
                let mut c = current;
                if read_u8(m) & 0x40 != 0 {
                    let a = acertada(read_u32_as_usize(G_CONTRACT_MANAGER), id);
                    if a != 0 {
                        c = transfer_manager_club_by_id(read_u32_as_usize(a + 4));
                    } else {
                        let m = read_u32_as_usize(gm + transfer_manager_fields::PERSON_FLAGS).wrapping_add(read_u32_as_usize(player).wrapping_mul(8));
                        write_u8(m, read_u8(m) & 0xbf);
                    }
                }
                let x = read_u8(c + club::SHORT_GENDER_NAME) as i8 as i32;
                text(0xde1f64, 0xa8f468, x, x, pn, c + club::SHORT_NAME);
                return 0;
            }
            0x27 => {
                text(msg, 0xa8f420);
                return 0;
            }
            7 => {
                text(msg, 0xa8f3c4, pn);
                return 0;
            }
            1 => {
                text(msg, 0xa8f378, pn);
                return 0;
            }
            0x12 | 0x13 => {
                text(0xde1f64, 0xa8f2bc, pn);
                return 0;
            }
            0x19 => read_u32_as_usize(G_NATION_TABLE).wrapping_add(read_u32_as_usize(KEY_NATION_PAYS_BASQUE).wrapping_mul(nation::SIZE)),
            0x28 => read_u32_as_usize(club + club::NATION),
            0x1a => read_u32_as_usize(G_NATION_TABLE).wrapping_add(read_u32_as_usize(KEY_NATION_SPAIN).wrapping_mul(nation::SIZE)),
            _ => return 0,
        };
        adjetivo(nation, pa, 0x64, 0xd);
        let x = read_u8(club + club::SHORT_GENDER_NAME) as i8 as i32;
        text(0xde1f64, 0xa8f30c, x, x, 0x64, 0x64, club + club::SHORT_NAME, pa);
        return 0;
    }
    for k in 0..0x1c {
        let r = read_u32_as_usize(this + transfer_manager_fields::TRANSFER_RULES + k * 4);
        if read_u32_as_usize(r + transfer_rules::NATION_ID) == read_u32_as_usize(club + club::NATION) {
            if r != 0 && rules_vfunc_20(r)(r, club, player, msg, 0) == 0 {
                return 0;
            }
            break;
        }
    }
    let mut rule = 0;
    for k in 0..0x1c {
        let r = read_u32_as_usize(this + transfer_manager_fields::TRANSFER_RULES + k * 4);
        if read_u32_as_usize(r + transfer_rules::NATION_ID) == read_u32_as_usize(current + club::NATION) {
            rule = r;
            break;
        }
    }
    if rule == 0 {
        return 1;
    }
    let c2 = contract_club_filtered(player, false);
    // simplification: the original reads all four as dwords with stack leftovers in the bytes that 0x8d0590 does not write (the month is a
    // byte, the year a word); 0x54a130 and 0x549f70 only read the month byte and the year word (and ecx, 0xffff)
    let (mut im, mut ia, mut fm, mut fa) = (0u32, 0u32, 0u32, 0u32);
    period(this, club, c2, player, &mut im as *mut u32 as usize, &mut ia as *mut u32 as usize,
        &mut fm as *mut u32 as usize, &mut fa as *mut u32 as usize);
    if im as u8 == fm as u8 && ia as u16 == fa as u16 {
        return 1;
    }
    let mut month = im as u8;
    let mut year = ia;
    mark(pd, day, month as u32, year, u32::MAX);
    let v24 = rules_vfunc_24(rule);
    if v24(rule, current, player, 0, pd, 1) != 0 {
        return 1;
    }
    loop {
        month = month.wrapping_add(1);
        if (month as i8) > 0xb {
            month = 0;
            year = year.wrapping_add(1);
        }
        let mut t = [0u8; 8];
        let p = dma(t.as_mut_ptr() as usize, day, fm, fa, u32::MAX);
        let (na, nd) = (read_i16(p + cm_date::YEAR) as u16, read_i16(p));
        let (da, dd) = (read_i16(pd + cm_date::YEAR) as u16, read_i16(pd));
        if !(na > da || (na == da && nd > dd)) {
            if msg != 0 {
                text(msg, 0xa8f288);
            }
            return 0;
        }
        mark(pd, day, month as u32, year, u32::MAX);
        if v24(rule, current, player, 0, pd, 1) != 0 {
            return 1;
        }
    }
}

/// Staff id -> person record (0x6e bytes each in G_STAFF_TABLE).
///
/// [pt] A pessoa (0x6e bytes em [0xae23bc]) e o clube (0x245 bytes em [0xae23b4]) pelo id.
pub(crate) unsafe fn transfer_manager_staff_by_id(id: usize) -> usize {
    read_u32_as_usize(G_STAFF_TABLE).wrapping_add(id.wrapping_mul(staff::SIZE))
}

/// Club id -> club record (0x245 bytes each in G_CLUB_TABLE).
pub(crate) unsafe fn transfer_manager_club_by_id(id: usize) -> usize {
    read_u32_as_usize(G_CLUB_TABLE).wrapping_add(id.wrapping_mul(club::SIZE))
}

/// Person -> main contract (CONTRACT_MANAGER::get_contract), also written to G_CONTRACT_LOOKUP_RESULT.
///
/// [pt] O contrato da pessoa (0x4cf5a0: push pessoa; mov ecx, [0xae1050] -> thiscall, ret 4), gravado em [0xae1080].
pub(crate) unsafe fn transfer_manager_get_contract_stored(person: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = f(read_u32_as_usize(G_CONTRACT_MANAGER), person);
    write_u32_from_usize(G_CONTRACT_LOOKUP_RESULT, c);
    c
}

/// If the offer's list at +4 has count 0, destroys and frees it and zeroes +4 (reads the count before the null check,
/// like the original).
///
/// [pt] Lista vazia (palavra +0 = 0) da oferta (+4): destruída (0x68d040), liberada (0x944c94) e +4 = 0, como a original
/// (que lê a palavra antes de testar o ponteiro).
pub(crate) unsafe fn free_empty_offer_list(offer: usize) {
    let l = read_u32_as_usize(offer + offer::EXCHANGE_PLAYERS);
    if read_i16(l) != 0 {
        return;
    }
    if l != 0 {
        let destroy: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
        let free: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_OPERATOR_DELETE);
        destroy(l);
        free(l);
    }
    write_u32_from_usize(offer + offer::EXCHANGE_PLAYERS, 0);
}

/// Cup + round index -> round entry (0x68 bytes each at [copa+0xa3]).
///
/// [pt] A entrada da rodada (0x68 bytes em [copa + 0xa3], o índice como o dword que a original multiplica).
pub(crate) unsafe fn cup_round_entry(cup: usize, round: u32) -> usize {
    read_u32_as_usize(cup + cup::ROUND_TABLE).wrapping_add((round as usize).wrapping_mul(0x68))
}

/// Collects into `lista` the cup's fixtures on the round's date and slice (FIXTURE_MANAGER
/// collect_comp_fixtures_on_day); returns the count.
///
/// [pt] Os jogos da copa na data e fatia da rodada, numa lista (0x5abd90: push lista; push liga +0x44; push rodada; push
/// id; push fatia +6; push dia; push ano; mov ecx, [0xae2a58] -> thiscall, ret 0x1c).
pub(crate) unsafe fn collect_round_fixtures(cup: usize, data: usize, round: u32, list: usize) -> u32 {
    let games: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, usize) -> u32 = core::mem::transmute(FN_FIXTURE_MANAGER_COLLECT_COMP_FIXTURES_ON_DAY);
    // simplification: the original pushes the day, the slice and the type in registers with leftovers in the high bytes; 0x5abd90 only
    // reads the words and the bytes
    games(read_u32_as_usize(G_FIXTURE_MANAGER), read_i16(data + cm_date::YEAR) as u16 as u32, read_i16(data) as u16 as u32, read_u8(cup_round_entry(cup, round) + 6) as u32,
        read_u32_as_usize(read_u32_as_usize(cup + 4)) as u32, round, read_u8(cup + cup::STAGE_INDEX) as u32, list) & 0xffff
}

/// Fixture `f` is on the round entry's scheduled date: day, year (+cup year +0x40), slice and the league type (+0x44)
/// match.
///
/// [pt] O jogo `f` está na data marcada da rodada: dia (+0x2a, sem sinal) = +3 da rodada, ano (+0x28) = +5 + o da copa
/// (+0x40), fatia (+0x3f) = +6 e o tipo (+0x42) da liga.
pub(crate) unsafe fn fixture_on_round_date(cup: usize, e: usize, f: usize) -> bool {
    read_i16(f + fixture::DAY) as u16 as i32 == read_i16(e + 3) as i32
        && read_i16(f + fixture::YEAR) as u16 as i32 == read_u8(e + 5) as i8 as i32 + read_i16(cup + cup::YEAR) as i32
        && read_u8(f + fixture::TIME_SLICE) as i32 == read_u8(e + 6) as i8 as i32
        && read_u8(f + fixture::STAGE_INDEX) == read_u8(cup + cup::STAGE_INDEX)
}

/// Removes the player from the offer's part-exchange list (or from his other offers), his value goes to the price, the
/// news, the computer seller's reply and the offer goes on.
///
/// Ret 0x10; SEH frame with no destructor on the normal path (the local list):
/// `TRANSFER_MANAGER::remove_exchange_player(person, offer, only_this, reason)`, from the contract offer to the player
/// (0x8c1e40), the confirmation by the club (0x8c3310), the release of the offer's players (0x8c6ae0) and the screen.
/// Only this one: without the exchange list (+4), nothing; with the person in it (+0 of each item = id), he leaves
/// (0x68d1f0), his 8-byte record (`[this+0x86e]` + id × 8, +2) loses an offer, the news (0x8de150 with the value =
/// +0x52 of the person + +0x1b of the offer and the reason) and, with the owner of the target's contract (0x4cf5a0,
/// except type 5) being the computer (0x600a50), its answer (0x8c8090 with &offer and 1): a resolved offer (status 2,
/// 3, 0x14, 0x10, 4, or 0xf with +0x2e ≥ 3) only frees the empty list; otherwise the price (+0x1b) becomes the value
/// and the offer goes on (0x8c2f50). At the end, the empty list is freed. Otherwise, with the person in some offer (+2
/// of the record): his offers by the club (0x8c5e40 into a local list), except this one; in each, he leaves the
/// exchange list (the empty list is freed) and, from the first one that had him, the price becomes +0x52 + the price,
/// the news (reason 0) and the offer goes on.
/// Original: `TRANSFER_MANAGER::remove_exchange_player` (0x8c41c0, transfer_manager.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x8c41c0 } (thiscall, ret 0x10, transfer_manager.cpp, quadro SEH sem destrutor no caminho normal: a lista local):
/// `TRANSFER_MANAGER::remove_exchange_player(pessoa, oferta, só_esta, motivo)`, da oferta de contrato ao jogador
/// (0x8c1e40), da confirmação pelo clube (0x8c3310), da soltura dos jogadores da oferta (0x8c6ae0) e da tela. Só esta:
/// sem a lista da troca (+4), nada; com a pessoa nela (+0 de cada item = id), sai (0x68d1f0), o registro de 8 bytes dela
/// ([this+0x86e] + id × 8, +2) perde uma oferta, a notícia (0x8de150 com o valor = +0x52 da pessoa + +0x1b da oferta e o
/// motivo) e, com o dono do contrato do alvo (0x4cf5a0, fora do tipo 5) do computador (0x600a50), a resposta dele
/// (0x8c8090 com &oferta e 1): oferta resolvida (status 2, 3, 0x14, 0x10, 4, ou 0xf com +0x2e ≥ 3) só solta a lista
/// vazia; senão o preço (+0x1b) vira o valor e a oferta segue (0x8c2f50). No fim, a lista vazia sai. Senão, com a pessoa
/// em alguma oferta (+2 do registro): as ofertas dela pelo clube (0x8c5e40 numa lista local), menos esta; em cada uma, sai
/// da lista da troca (a lista vazia sai) e, desde a primeira que a tinha, o preço vira +0x52 + o preço, a notícia (motivo
/// 0) e a oferta segue.
pub(crate) unsafe extern "thiscall" fn remove_exchange_player(this: usize, person: usize, offer: usize, so_esta: u32, reason: u32) {
    let create: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let destroy: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let remove: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_LLIST_REMOVE);
    let news: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_POST_EXCHANGE_PLAYER_REMOVED_NEWS);
    let follow: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_ADVANCE_OFFER_TO_CLUB_APPROVAL);
    let mut list = [0u8; 0x10];
    let l = list.as_mut_ptr() as usize;
    create(l);
    let minus_one = |p: usize| {
        let a = read_u32_as_usize(this + transfer_manager_fields::PERSON_FLAGS) + read_u32_as_usize(p) * 8 + 2;
        write_u8(a, read_u8(a).wrapping_sub(1));
    };
    let mut offer = offer;
    if so_esta != 0 {
        let c = transfer_manager_get_contract_stored(transfer_manager_staff_by_id(read_u32_as_usize(offer + offer::TARGET_STAFF_ID)));
        let owner = if c != 0 && read_u8(c + contract::CONTRACT_TYPE) & 0x3f != 5 { transfer_manager_club_by_id(read_u32_as_usize(c + contract::CLUB)) } else { 0 };
        let swap = read_u32_as_usize(offer + offer::EXCHANGE_PLAYERS);
        if swap == 0 {
            destroy(l);
            return;
        }
        write_u32_from_usize(swap + llist::CURSOR, read_u32_as_usize(swap + llist::HEAD));
        let mut it = next(read_u32_as_usize(offer + offer::EXCHANGE_PLAYERS));
        while it != 0 && read_u32_as_usize(person) != read_u32_as_usize(it) {
            it = next(read_u32_as_usize(offer + offer::EXCHANGE_PLAYERS));
        }
        if it != 0 {
            remove(read_u32_as_usize(offer + offer::EXCHANGE_PLAYERS), it);
            minus_one(person);
            let v = read_u32_as_usize(person + staff::VALUE).wrapping_add(read_u32_as_usize(offer + offer::FEE));
            news(this, offer, person, v, reason);
            let human_club: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
            if owner != 0 && human_club(owner, 0, 1, 0, 0) == 0 {
                let reply: unsafe extern "thiscall" fn(usize, *mut usize, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_AI_RESPOND_TO_BID);
                reply(this, &mut offer, 1);
                let s = read_u8(offer + offer::STATUS);
                if matches!(s, 2 | 3 | 0x14 | 0x10 | 4) || (s == 0xf && read_u8(offer + offer::RESPONSE_ROUND) >= 3) {
                    free_empty_offer_list(offer);
                    destroy(l);
                    return;
                }
            }
            crate::comparison::branch("cqc_7 sai da troca da oferta");
            write_u32_from_usize(offer + offer::FEE, v);
            follow(this, offer);
        }
        free_empty_offer_list(offer);
    } else {
        if read_u8(read_u32_as_usize(this + transfer_manager_fields::PERSON_FLAGS) + read_u32_as_usize(person) * 8 + 2) != 0 {
            let swaps: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_EXCHANGE_OFFERS_FOR_PLAYER);
            swaps(this, transfer_manager_staff_by_id(read_u32_as_usize(offer + offer::TARGET_STAFF_ID)), l);
            write_u32_from_usize(l + llist::CURSOR, read_u32_as_usize(l + llist::HEAD));
            let mut found = false;
            let mut it = next(l);
            while it != 0 {
                let o = read_u32_as_usize(read_u32_as_usize(this) + read_u32_as_usize(it) * 4);
                if o != offer {
                    if read_u32_as_usize(o + offer::EXCHANGE_PLAYERS) != 0 {
                        let t = read_u32_as_usize(o + offer::EXCHANGE_PLAYERS);
                        write_u32_from_usize(t + llist::CURSOR, read_u32_as_usize(t + llist::HEAD));
                        let mut x = next(read_u32_as_usize(o + offer::EXCHANGE_PLAYERS));
                        while x != 0 {
                            if read_u32_as_usize(person) == read_u32_as_usize(x) {
                                found = true;
                                remove(read_u32_as_usize(o + offer::EXCHANGE_PLAYERS), x);
                                minus_one(person);
                                break;
                            }
                            x = next(read_u32_as_usize(o + offer::EXCHANGE_PLAYERS));
                        }
                        free_empty_offer_list(o);
                    }
                    if found {
                        crate::comparison::branch("cqc_7 sai da troca das outras");
                        let v = read_u32_as_usize(person + staff::VALUE).wrapping_add(read_u32_as_usize(o + offer::FEE));
                        news(this, o, person, v, 0);
                        write_u32_from_usize(o + offer::FEE, v);
                        follow(this, o);
                    }
                }
                it = next(l);
            }
        }
    }
    destroy(l);
}

/// The person's work permit at the club; returns (needs, offer, granted).
///
/// Whether one is needed (0x8c7560: push 0; push club; push person; mov ecx, this -> thiscall, ret 0xc), the person's
/// contract offer to the club (0x4cc9e0: push club id; push person id; mov ecx, [0xae1050] -> thiscall, ret 8) and
/// whether it is granted (0x8c7950: push 0; push 0; push offer; push club nation +0x53; push person; mov ecx, this ->
/// thiscall, ret 0x14, no random draw).
///
/// [pt] A permissão de trabalho da pessoa no clube: precisa (0x8c7560: push 0; push clube; push pessoa; mov ecx, this ->
/// thiscall, ret 0xc), a proposta de contrato dela ao clube (0x4cc9e0: push id do clube; push id da pessoa; mov ecx,
/// [0xae1050] -> thiscall, ret 8) e a concessão (0x8c7950: push 0; push 0; push proposta; push nação do clube +0x53; push
/// pessoa; mov ecx, this -> thiscall, ret 0x14, sem sorteio). Devolve (precisa, proposta, concedida).
pub(crate) unsafe fn check_work_permit(this: usize, person: usize, club: usize) -> (bool, usize, bool) {
    let precisa: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_PLAYER_NEEDS_WORK_PERMIT);
    if precisa(this, person, club, 0) == 0 {
        return (false, 0, false);
    }
    let proposta: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_FIND_CONTRACT_OFFER);
    let r = proposta(read_u32_as_usize(G_CONTRACT_MANAGER), read_u32_as_usize(person) as u32, read_u32_as_usize(club) as u32);
    if r == 0 {
        return (true, 0, false);
    }
    let concedida: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_IS_WORK_PERMIT_GRANTED);
    (true, r, concedida(this, person, read_u32_as_usize(club + club::NATION), r, 0, 0) != 0)
}

/// Checks the offer's work permits: of the target at the buyer (refused: the offer collapses, status 0x10, rescheduled
/// in 10 days, returns 0) and of each part-exchange player at the seller (refused: he leaves the exchange and his value
/// goes to the price); the news and the offer goes on.
///
/// Ret 8: `TRANSFER_MANAGER::check_offer_work_permits(offer, quiet)`, from the processing of the offer (status 0xa,
/// 0x8c6880) and the completion (0x8cbc40). The owner of the target's contract (0x4cf5a0, `[0xae1080]`; any type).
/// Target (+0xc) with a permit to request at the buyer (+8) and a proposal, not granted (check_work_permit): the offer
/// collapses (0x8de790 with 0x13), status 0x10, the proposals closed (0x8c6db0), the bid marks removed (0x84aeb0 on the
/// list manager `[this+0x856]`: the buyer and the target, then the owner of the target's contract and each exchange
/// player), rescheduled in 10 days (0x8c6220) and returns 0. Granted (or no proposal) and without `quiet`: the grant
/// news (0x8e3130). Then, each exchange player (+4) at the owner of the target's contract: refused, the news (0x8e35a0
/// with value +0x52 + the price), he leaves the exchange (0x68d1f0), his 8-byte record loses an offer and the price
/// (+0x1b) becomes the value; granted without `quiet`, the news. The empty list is freed; without `quiet`, the offer
/// goes on (0x8c2f50). Returns 1.
/// Original: `TRANSFER_MANAGER::check_offer_work_permits` (0x8c7690, transfer_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x8c7690 } (thiscall, ret 8, transfer_manager.cpp): `TRANSFER_MANAGER::check_offer_work_permits(oferta, quieto)`,
/// do processamento da oferta (status 0xa, 0x8c6880) e da conclusão (0x8cbc40). O dono do contrato do alvo (0x4cf5a0,
/// [0xae1080]; qualquer tipo). Alvo (+0xc) com permissão a pedir no comprador (+8) e proposta, sem concessão
/// (check_work_permit): a oferta desfeita (0x8de790 com 0x13), status 0x10, as propostas fechadas (0x8c6db0), as marcas de
/// lance saem (0x84aeb0 no gerente de listas [this+0x856]: o comprador e o alvo, depois o dono do contrato do alvo e cada
/// jogador da troca), reagendada em 10 dias (0x8c6220) e devolve 0. Concedida (ou sem proposta) e sem `quieto`: a
/// notícia da concessão (0x8e3130). Depois, cada jogador da troca (+4) no dono do contrato do alvo: recusado, a notícia
/// (0x8e35a0 com o valor +0x52 + o preço), sai da troca (0x68d1f0), o registro de 8 bytes dele perde uma oferta e o
/// preço (+0x1b) vira o valor; concedido sem `quieto`, a notícia. A lista vazia sai; sem `quieto`, a oferta segue
/// (0x8c2f50). Devolve 1.
pub(crate) unsafe extern "thiscall" fn check_offer_work_permits(this: usize, offer: usize, quieto: u32) -> u32 {
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let concedida_noticia: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_POST_WORK_PERMIT_GRANTED_NEWS);
    let target = transfer_manager_staff_by_id(read_u32_as_usize(offer + offer::TARGET_STAFF_ID));
    let buyer = transfer_manager_club_by_id(read_u32_as_usize(offer + offer::BUYER_CLUB_ID));
    let c = transfer_manager_get_contract_stored(target);
    let owner = if c != 0 { transfer_manager_club_by_id(read_u32_as_usize(c + contract::CLUB)) } else { 0 };
    let (precisa, proposta, concedida) = check_work_permit(this, target, buyer);
    if precisa {
        if proposta != 0 && !concedida {
            crate::comparison::branch("cqc_7 permissao recusada");
            let desfeita: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_POST_OFFER_COLLAPSED_NEWS);
            let close: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_CLOSE_OFFER_CONTRACT_OFFERS);
            let unmark: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_SHORTLIST_MANAGER_CLEAR_BID_MARKERS_FOR_PLAYER);
            let schedule: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_SCHEDULE_OFFER);
            desfeita(this, offer, buyer, 0x13);
            write_u8(offer + offer::STATUS, 0x10); // cqc7: the offer falls through over the permit
            close(this, offer);
            let target = transfer_manager_staff_by_id(read_u32_as_usize(offer + offer::TARGET_STAFF_ID));
            let c = transfer_manager_get_contract_stored(target);
            let owner = if c != 0 { transfer_manager_club_by_id(read_u32_as_usize(c + contract::CLUB)) } else { 0 };
            unmark(read_u32_as_usize(this + transfer_manager_fields::SHORTLIST_MANAGER), transfer_manager_club_by_id(read_u32_as_usize(offer + offer::BUYER_CLUB_ID)), target);
            if read_u32_as_usize(offer + offer::EXCHANGE_PLAYERS) != 0 {
                let t = read_u32_as_usize(offer + offer::EXCHANGE_PLAYERS);
                write_u32_from_usize(t + llist::CURSOR, read_u32_as_usize(t + llist::HEAD));
                let mut it = next(read_u32_as_usize(offer + offer::EXCHANGE_PLAYERS));
                while it != 0 {
                    unmark(read_u32_as_usize(this + transfer_manager_fields::SHORTLIST_MANAGER), owner, transfer_manager_staff_by_id(read_u32_as_usize(it)));
                    it = next(read_u32_as_usize(offer + offer::EXCHANGE_PLAYERS));
                }
            }
            schedule(this, offer, 0xa, 0);
            return 0;
        }
        if quieto == 0 {
            concedida_noticia(this, target, buyer, offer);
        }
    }
    if read_u32_as_usize(offer + offer::EXCHANGE_PLAYERS) != 0 {
        let t = read_u32_as_usize(offer + offer::EXCHANGE_PLAYERS);
        write_u32_from_usize(t + llist::CURSOR, read_u32_as_usize(t + llist::HEAD));
        let mut it = next(read_u32_as_usize(offer + offer::EXCHANGE_PLAYERS));
        while it != 0 {
            let x = transfer_manager_staff_by_id(read_u32_as_usize(it));
            let (precisa, proposta, concedida) = check_work_permit(this, x, owner);
            if precisa {
                if proposta != 0 && !concedida {
                    crate::comparison::branch("cqc_7 troca recusada pela permissao");
                    let v = read_u32_as_usize(x + staff::VALUE).wrapping_add(read_u32_as_usize(offer + offer::FEE));
                    let recusada: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_POST_EXCHANGE_PLAYER_PERMIT_REFUSED_NEWS);
                    let remove: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_LLIST_REMOVE);
                    recusada(this, offer, x, v);
                    remove(read_u32_as_usize(offer + offer::EXCHANGE_PLAYERS), it);
                    let a = read_u32_as_usize(this + transfer_manager_fields::PERSON_FLAGS) + read_u32_as_usize(x) * 8 + 2;
                    write_u8(a, read_u8(a).wrapping_sub(1));
                    write_u32_from_usize(offer + offer::FEE, v);
                } else if quieto == 0 {
                    concedida_noticia(this, x, owner, offer);
                }
            }
            it = next(read_u32_as_usize(offer + offer::EXCHANGE_PLAYERS));
        }
        free_empty_offer_list(offer);
    }
    if quieto == 0 {
        let follow: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_ADVANCE_OFFER_TO_CLUB_APPROVAL);
        follow(this, offer);
    }
    1
}

/// Person -> club of their contract (stored in G_CONTRACT_LOOKUP_RESULT); 0 without contract, or for type 5 (+0x35)
/// when `sem_5`.
///
/// [pt] O contrato da pessoa (0x4cf5a0: push pessoa; mov ecx, [0xae1050] -> thiscall, ret 4), guardado em [0xae1080] como
/// a original faz, e o clube dele (+4); sem contrato, ou com o tipo 5 em +0x35 quando `sem_5`: 0.
pub(crate) unsafe fn contract_club_filtered(person: usize, sem_5: bool) -> usize {
    let contract: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = contract(read_u32_as_usize(G_CONTRACT_MANAGER), person);
    write_u32_from_usize(G_CONTRACT_LOOKUP_RESULT, c);
    if c == 0 || (sem_5 && read_u8(c + contract::CONTRACT_TYPE) & 0x3f == 5) {
        0
    } else {
        transfer_manager_club_by_id(read_u32_as_usize(c + contract::CLUB))
    }
}

/// Club is controlled by a human manager (HUMAN_MANAGER is_human_club with 0,1,0,0).
///
/// [pt] O clube é humano (0x600a50: push 0; push 0; push 1; push 0; push clube; add esp, 0x14 -> cdecl(clube, 0, 1, 0, 0)).
pub(crate) unsafe fn transfer_manager_is_human_club(club: usize) -> bool {
    let f: unsafe extern "C" fn(usize, u32, u32, usize, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    f(club, 0, 1, 0, 0) != 0
}

/// Reschedules the offer `dias` days ahead (TRANSFER_MANAGER::schedule_offer, 0 slices).
///
/// [pt] Reagenda a oferta (0x8c6220: push 0; push dias; push oferta; mov ecx, this -> thiscall, ret 0xc).
pub(crate) unsafe fn schedule_offer_days(this: usize, o: usize, days: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_SCHEDULE_OFFER);
    f(this, o, days, 0);
}

/// Next node of a game LLIST (0x68d1b0), advancing its cursor.
///
/// [pt] O próximo da lista (0x68d1b0: mov ecx, lista -> thiscall, ret 0).
pub(crate) unsafe fn transfer_manager_llist_next(l: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_LLIST_GET_NEXT);
    f(l) as usize
}

/// The block that `0x8c63a0` repeats: clears shortlist bid markers.
///
/// Takes the club of the target's contract (without looking at type 5); the buyer's (`+8`) bids for the target (`+0xc`)
/// are unmarked on the watch list (`0x84aeb0`: push player; push club; mov ecx, [this+0x856] -> thiscall, ret 8), and
/// so are those of the contract club for each player in the exchange (list at `+4`, cursor at the start).
///
/// [pt] O bloco que a 0x8c63a0 repete: o clube do contrato do alvo (sem olhar o tipo 5), os lances do comprador (+8) pelo
/// alvo (+0xc) desmarcados na lista de observação (0x84aeb0: push jogador; push clube; mov ecx, [this+0x856] ->
/// thiscall, ret 8) e os do clube do contrato por cada jogador da troca (lista em +4, cursor no começo).
pub(crate) unsafe fn clear_offer_bid_markers(this: usize, o: usize) {
    let unmark: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_SHORTLIST_MANAGER_CLEAR_BID_MARKERS_FOR_PLAYER);
    let target = transfer_manager_staff_by_id(read_u32_as_usize(o + offer::TARGET_STAFF_ID));
    let current = contract_club_filtered(target, false);
    unmark(read_u32_as_usize(this + transfer_manager_fields::SHORTLIST_MANAGER), transfer_manager_club_by_id(read_u32_as_usize(o + offer::BUYER_CLUB_ID)), target);
    let l = read_u32_as_usize(o + offer::EXCHANGE_PLAYERS);
    if l == 0 {
        return;
    }
    write_u32_from_usize(l + llist::CURSOR, read_u32_as_usize(l + llist::HEAD));
    loop {
        let p = transfer_manager_llist_next(read_u32_as_usize(o + offer::EXCHANGE_PLAYERS));
        if p == 0 {
            break;
        }
        unmark(read_u32_as_usize(this + transfer_manager_fields::SHORTLIST_MANAGER), current, transfer_manager_staff_by_id(read_u32_as_usize(p)));
    }
}

/// One slice of the transfer day: window notices, overdue offers rescheduled, the slice's bucket processed; in slice 2
/// the base date moves to tomorrow, instalments at month end, nation rules; on day 100 the +0x1e of non-EU persons at
/// the club for years.
///
/// Ret 8; SEH frame with no destructor on the normal path (the local dates): `TRANSFER_MANAGER::daily_update(today,
/// slice)`. Without a base date (year +0x84f zeroed), it becomes today (0x417c10: push today; push out; lea ecx,
/// `[this+0x84d]` -> thiscall, ret 8) and slice +0x855 today's. Only with today equal to the base and the slice equal
/// to +0x855: in slice 0 the window notices (0x8c6bc0: push today; mov ecx, this -> thiscall, ret 4); on a day that is
/// a multiple of 7, each overdue offer (date +0x12/+0x14 before today; status +0x2c not 0x13, 7, 8 or 0xa) is
/// rescheduled for tomorrow (0x8c6220 with 1 day); slice +0x855 goes up 1; the free transfer queue (0x8c18c0: mov ecx,
/// this -> thiscall, ret 0); the current bucket (0xe-byte lists at this+0x18 + 0xe·(3·`[+0x84c]` + slice), cursor at
/// the start): each valid index of vector `[this]` (count +4) with today's offer and of this slice (+0x1a) is processed
/// (0x8c63a0: push slot; mov ecx, this -> thiscall, ret 4); the bucket is emptied (0x68d080). In slice 2: the base
/// becomes tomorrow (0x54c5b0 push 1; push out; mov ecx, base -> thiscall, ret 8 and 0x417c10), +0x855 goes back to 0,
/// +0x84c goes up (from 49 back to 0); when the month changes from today to tomorrow (0x54cfb0: mov ecx, date ->
/// thiscall, ret 0), the instalments (0x8caee0); on day 0, the old transfers to the archives (0x8c3eb0) and the work
/// permits (0x8c7330); virtual +0 of each of the 28 nation rules (`[this+0x8ac]`, push today; mov ecx, rules); on a day
/// that is a multiple of 7, the permits again. In slice 0 (other than slice 2), the future transfers (0x8cbbe0). On day
/// 100 of slice 2: the day-100 dates of 5, 2 and 10 years ago (0x549ef0: push year; push 0x64; lea ecx, local ->
/// thiscall, ret 8) and, for each person (0x6e-byte records in `[0xae23bc]`, count `[0xae2364]`, reread) with date
/// +0x3e/+0x40 before the 2-year one, a club (`[[+0x39]+0x53]`) in a nation (`[+0x71]`) with id `[0x9cfa10]` and not
/// EU-eligible there (0x53d740: push club; push person; add esp, 8 -> cdecl): +0x1e (the second nationality) = the
/// club's nation (club +0x53), if it is `[0x9cf488]` or `[0x9cf434]`; if the date is before the 5-year one and it is
/// not `[0x9cf360]`, `[0x9cf2fc]` or `[0x9cf448]`; or if the date is before the 10-year one.
/// Original: `TRANSFER_MANAGER::daily_update` (0x8c5880, transfer_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x8c5880 } (thiscall, ret 8, transfer_manager.cpp, quadro SEH sem destrutor no caminho normal: as datas locais):
/// `TRANSFER_MANAGER::daily_update(hoje, fatia)`. Sem data base (ano +0x84f zerado), ela vira hoje (0x417c10: push
/// hoje; push saída; lea ecx, [this+0x84d] -> thiscall, ret 8) e a fatia +0x855 a de hoje. Só com hoje igual à base e a
/// fatia igual a +0x855: na fatia 0 os avisos das janelas (0x8c6bc0: push hoje; mov ecx, this -> thiscall, ret 4); no
/// dia múltiplo de 7, cada oferta vencida (data +0x12/+0x14 antes de hoje; status +0x2c fora de 0x13, 7, 8 e 0xa) é
/// reagendada para amanhã (0x8c6220 com 1 dia); a fatia +0x855 sobe 1; a fila da transferência livre (0x8c18c0: mov
/// ecx, this -> thiscall, ret 0); o balde da vez (listas de 0xe bytes em this+0x18 + 0xe·(3·[+0x84c] + fatia), cursor no
/// começo): cada índice válido do vetor [this] (conta +4) com a oferta de hoje e desta fatia (+0x1a) é processado
/// (0x8c63a0: push vaga; mov ecx, this -> thiscall, ret 4); o balde é esvaziado (0x68d080). Na fatia 2: a base passa a
/// amanhã (0x54c5b0 push 1; push saída; mov ecx, base -> thiscall, ret 8 e 0x417c10), +0x855 volta a 0, +0x84c sobe (de
/// 49 volta a 0); mudando o mês de hoje para amanhã (0x54cfb0: mov ecx, data -> thiscall, ret 0), as prestações
/// (0x8caee0); no dia 0, as transferências antigas aos arquivos (0x8c3eb0) e as permissões de trabalho (0x8c7330); o
/// virtual +0 de cada uma das 28 regras de nação ([this+0x8ac], push hoje; mov ecx, regras); no dia múltiplo de 7, as
/// permissões de novo. Na fatia 0 (fora a 2), as transferências futuras (0x8cbbe0). No dia 100 da fatia 2: as datas 100
/// de 5, 2 e 10 anos atrás (0x549ef0: push ano; push 0x64; lea ecx, local -> thiscall, ret 8) e, para cada pessoa
/// (registros de 0x6e bytes em [0xae23bc], conta [0xae2364], relida) com a data +0x3e/+0x40 antes da de 2 anos, clube
/// ([[+0x39]+0x53]) numa nação ([+0x71]) de id [0x9cfa10] e sem ser comunitária lá (0x53d740: push clube; push pessoa;
/// add esp, 8 -> cdecl): +0x1e (a segunda nacionalidade) = a nação do clube (clube +0x53), se ela é [0x9cf488] ou [0x9cf434]; se a data é
/// antes da de 5 anos e ela não é [0x9cf360], [0x9cf2fc] ou [0x9cf448]; ou se a data é antes da de 10 anos.
pub(crate) unsafe extern "thiscall" fn transfer_manager_daily_update(this: usize, today: usize, slice: u32) {
    let copy: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let more: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_DATE_ADD_DAYS);
    let month: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(FN_DATE_GET_MONTH);
    let new: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE);
    let call: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_TRANSFER_MANAGER_PROCESS_FREE_TRANSFER_QUEUE);
    let windows: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_UPDATE_TRANSFER_WINDOW_NEWS);
    let processar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_PROCESS_DUE_OFFER);
    let esvaziar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_CLEAR);
    let prestacoes: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_TRANSFER_MANAGER_PAY_TRANSFER_INSTALMENTS);
    let antigas: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_TRANSFER_MANAGER_CLEAR_OLD_PAST_TRANSFERS);
    let permissoes: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_TRANSFER_MANAGER_UPDATE_WORK_PERMITS);
    let futuras: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_TRANSFER_MANAGER_PROCESS_DUE_FUTURE_TRANSFERS);
    let community: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_STAFF_EU_NATIONAL);
    let f = slice as u8;
    let mut t20 = [0u8; 8];
    if read_i16(this + transfer_manager_fields::CURRENT_YEAR) == 0 {
        copy(this + transfer_manager_fields::CURRENT_DATE, t20.as_mut_ptr() as usize, today);
        write_u8(this + transfer_manager_fields::CURRENT_SLICE, f);
    }
    if read_i16(today + cm_date::YEAR) != read_i16(this + transfer_manager_fields::CURRENT_YEAR) || read_i16(today) != read_i16(this + transfer_manager_fields::CURRENT_DATE) || read_u8(this + transfer_manager_fields::CURRENT_SLICE) != f {
        return;
    }
    if f == 0 {
        windows(this, today);
    }
    if read_i16(today) as i32 % 7 == 0 {
        let mut i: i32 = 0;
        while i < read_u32_as_usize(this + transfer_manager_fields::OFFER_CAPACITY) as i32 {
            let o = read_u32_as_usize(read_u32_as_usize(this).wrapping_add((i as usize).wrapping_mul(4)));
            if o != 0 {
                let (ha, oa) = (read_i16(today + cm_date::YEAR) as u16, read_i16(o + offer::DUE_YEAR) as u16);
                let vencida = ha > oa || (ha == oa && read_i16(today) > read_i16(o + offer::DUE_DATE));
                let s = read_u8(o + offer::STATUS);
                if vencida && s != 0x13 && s != 7 && s != 8 && s != 0xa {
                    schedule_offer_days(this, o, 1);
                }
            }
            i += 1;
        }
    }
    write_u8(this + transfer_manager_fields::CURRENT_SLICE, read_u8(this + transfer_manager_fields::CURRENT_SLICE).wrapping_add(1));
    call(this);
    let balde = |this: usize| {
        let k = (read_u8(this + transfer_manager_fields::DAY_CURSOR) as i8 as i32) * 3 + f as i8 as i32;
        this.wrapping_add((k as isize * 0xe) as usize).wrapping_add(0x18)
    };
    let l = balde(this);
    write_u32_from_usize(l + llist::CURSOR, read_u32_as_usize(l + llist::HEAD));
    loop {
        let p = transfer_manager_llist_next(balde(this));
        if p == 0 {
            break;
        }
        let idx = read_u32_as_usize(p) as i32;
        if idx < read_u32_as_usize(this + transfer_manager_fields::OFFER_CAPACITY) as i32 && idx >= 0 {
            let slot = read_u32_as_usize(this).wrapping_add(idx as usize * 4);
            let o = read_u32_as_usize(slot);
            if o != 0 && read_i16(today + cm_date::YEAR) == read_i16(o + offer::DUE_YEAR) && read_i16(today) == read_i16(o + offer::DUE_DATE)
                && read_u8(o + offer::DUE_SLICE) == f { // cqd1: a oferta desta fatia
                crate::comparison::branch("cqd_1 oferta do dia");
                processar(this, slot);
            }
        }
    }
    esvaziar(balde(this));
    if f == 2 {
        let (mut t28, mut t30) = ([0u8; 8], [0u8; 8]);
        let r = more(this + transfer_manager_fields::CURRENT_DATE, t28.as_mut_ptr() as usize, 1);
        copy(this + transfer_manager_fields::CURRENT_DATE, t30.as_mut_ptr() as usize, r);
        write_u8(this + transfer_manager_fields::CURRENT_SLICE, 0);
        let j = read_u8(this + transfer_manager_fields::DAY_CURSOR).wrapping_add(1);
        write_u8(this + transfer_manager_fields::DAY_CURSOR, j);
        if (j as i8) >= 0x32 {
            write_u8(this + transfer_manager_fields::DAY_CURSOR, 0);
        }
        let amanha = more(today, t30.as_mut_ptr() as usize, 1);
        let m = month(today);
        if m != month(amanha) {
            crate::comparison::branch("cqd_1 prestações do mês");
            prestacoes(this); // cqd1: the instalments at the turn of the month
        }
        if read_i16(today) == 0 {
            antigas(this);
            permissoes(this);
        }
        for k in 0..0x1c {
            let r = read_u32_as_usize(this + transfer_manager_fields::TRANSFER_RULES + k * 4);
            if r != 0 {
                let v: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(r)));
                v(r, today);
            }
        }
        if read_i16(today) as i32 % 7 == 0 {
            permissoes(this);
        }
    } else if f == 0 {
        futuras(this);
    }
    if read_i16(today) != 100 || f != 2 {
        return;
    }
    // simplification: the original pushes the year in eax with leftover in the high 16 bits; 0x549ef0 only reads the words
    let year = read_i16(today + cm_date::YEAR) as u16;
    let (mut d5, mut d2, mut d10) = ([0u8; 8], [0u8; 8], [0u8; 8]);
    let p5 = new(d5.as_mut_ptr() as usize, 100, year.wrapping_sub(5) as u32);
    let p2 = new(d2.as_mut_ptr() as usize, 100, year.wrapping_sub(2) as u32);
    let p10 = new(d10.as_mut_ptr() as usize, 100, year.wrapping_sub(10) as u32);
    let mut s = read_u32_as_usize(G_STAFF_TABLE);
    let mut i: i32 = 0;
    while i < read_u32_as_usize(G_STAFF_COUNT) as i32 {
        if meets_residency_naturalisation(s, p2, p5, p10, community) {
            write_u32_from_usize(s + staff::SECOND_NATION, read_u32_as_usize(read_u32_as_usize(s + staff::CLUB_JOB) + 0x53));
        }
        i += 1;
        s += 0x6e;
    }
}

/// Condition for day 100 of 0x8c5880: the person's date (+0x3e/+0x40) is older than the 2/5/10-year threshold of the
/// club's European non-member nation (Spain/Portugal 2, most 5, Italy/Finland/Russia 10) [inference: residency
/// naturalisation].
///
/// [pt] A condição do dia 100 da 0x8c5880 (também para o comparador): a data +0x3e (dia com sinal)/+0x40 (ano sem sinal) da
/// pessoa antes da de 2 anos, o clube na nação [0x9cfa10] sem ser comunitária, e o caso pelo id do clube.
pub(crate) unsafe fn meets_residency_naturalisation(s: usize, p2: usize, p5: usize, p10: usize, community: unsafe extern "C" fn(usize, usize) -> u32) -> bool {
    let before = |d: usize| {
        let (da, sa) = (read_i16(d + cm_date::YEAR) as u16, read_i16(s + 0x40) as u16);
        da > sa || (da == sa && read_i16(d) > read_i16(s + staff::DATE_JOINED_CLUB))
    };
    if !before(p2) {
        return false;
    }
    let j = read_u32_as_usize(s + staff::CLUB_JOB);
    if j == 0 {
        return false;
    }
    let c = read_u32_as_usize(j + club::NATION);
    if c == 0 {
        return false;
    }
    let n = read_u32_as_usize(c + nation::CONTINENT);
    if n == 0 || read_u32_as_usize(n) != read_u32_as_usize(KEY_CONTINENT_EUROPE) || community(s, c) != 0 {
        return false;
    }
    let id = read_u32_as_usize(c);
    if id == read_u32_as_usize(KEY_NATION_SPAIN) || id == read_u32_as_usize(KEY_NATION_PORTUGAL) {
        return true;
    }
    if before(p5) && id != read_u32_as_usize(KEY_NATION_ITALY) && id != read_u32_as_usize(KEY_NATION_FINLAND) && id != read_u32_as_usize(KEY_NATION_RUSSIA) {
        return true;
    }
    before(p10)
}

/// Dispatches the offer due today by its status: seller response, acceptance, player contract, permits, completion,
/// removal or a new round of the offer.
///
/// Ret 4; SEH frame (the local lists, built by 0x68d020 and destroyed by 0x68d040 in the same order):
/// `TRANSFER_MANAGER::process_due_offer(slot)`, the offer that falls due today, by status +0x2c. The seller is the club
/// of the target's contract (+0xc; except type 5), the buyer +8.
///
/// - 0: without a seller, the bid acceptance (0x8c0080: push 0; push slot -> thiscall, ret 8). Human seller: status 4,
/// 14 days (0x8c6220) and the bids unmarked (clear_offer_bid_markers). Otherwise, without flag +0x1f = 1, the AI's
/// answer (0x8c8090: push 0; push slot, ret 8); with it, the seller's prediction (0x84af00, stdcall(seller, target, 0
/// i64, buyer, −1 i64, 1): ecx = `[this+0x856]` is not read): refusal, the loan refusal (0x8bfa30: push 0; push 0; push
/// slot -> thiscall, ret 0xc); accepted, the decision (0x4cc930: push offer; mov ecx, `[0xae1050]` -> thiscall, ret 4),
/// status 8 and the acceptance news (0x8d76d0: push offer, ret 4); with the decision, the refusal-to-move news
/// (0x8d8310: push 0; push offer, ret 8), status 0xf, +0x2e = 3, the bids unmarked, 10 days, and if the offer and those
/// of the list at +0 are closed (0x8d0d90, stdcall(offer)), bit 0 of the target's 8-byte record in `[this+0x86e]` is
/// cleared.
///
/// - 2, 3, 4, 0xf, 0x10, 0x14: with 0xf and +0x2e < 3, +0x2e = 3 and 10 days; otherwise the offer is removed (0x8d0f60:
/// push offer, ret 4).
///
/// - 1: human buyer: status 4, the bids unmarked and 10 days; otherwise 0x8c0f40 (push slot, ret 4) with +0x1f = 1 or
/// 0x8c0810 (push 0; push slot, ret 8).
///
/// - 5, 6: human buyer: status 9 and 10 days; otherwise the contract to the target (0x8c1e40: push offer; push target;
/// push buyer, ret 0xc).
///
/// - 7 with an exchange (+4): human seller: status 8 and 10 days; otherwise, on a copy of the list (0x68d2f0: push
/// list; lea ecx, local, ret 4), the contract to each exchange player until the offer becomes active.
///
/// - 0xa: the permits (0x8c7690: push 0; push offer, ret 8). 0xe: the completion (0x8c34e0: push 0; push offer, ret 8).
///
/// - 0x11, 0x12: status 0; with +0x10, the offer passes to the clubs (0x8c2f50: push offer, ret 4); otherwise, with a
/// human manager at the contract club (0x7875e0, stdcall(club): ecx = `[0xb7431c]` is not read), the offer-made news
/// (0x8d3090: push offer, ret 4) and 28 days; otherwise the offer value (0x8e7b50: mov ecx, offer -> thiscall(offer, 0,
/// 0, 0, 0, 1), ret 0x14) and the seller's prediction (seller, target, value, buyer, −1, 0): 0 accepts (0x8c0080), 1 or
/// 2 refuses (0x8bf680: push 0; push 0; push slot, ret 0xc).
/// Original: `TRANSFER_MANAGER::process_due_offer` (0x8c63a0, transfer_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x8c63a0 } (thiscall, ret 4, transfer_manager.cpp, quadro SEH: as listas locais, construídas pela 0x68d020 e
/// destruídas pela 0x68d040 na mesma ordem): `TRANSFER_MANAGER::process_due_offer(vaga)`, a oferta que vence hoje, pelo
/// status +0x2c. O vendedor é o clube do contrato do alvo (+0xc; fora o tipo 5), o comprador o +8.
/// - 0: sem vendedor, a aceitação do lance (0x8c0080: push 0; push vaga -> thiscall, ret 8). Vendedor humano: status 4,
///   14 dias (0x8c6220) e os lances desmarcados (clear_offer_bid_markers). Senão, sem a marca +0x1f = 1, a resposta da IA
///   (0x8c8090: push 0; push vaga, ret 8); com ela, a previsão do vendedor (0x84af00, stdcall(vendedor, alvo, 0 i64,
///   comprador, −1 i64, 1): o ecx = [this+0x856] não é lido): recusa, a do empréstimo (0x8bfa30: push 0; push 0; push
///   vaga -> thiscall, ret 0xc); aceita, a decisão (0x4cc930: push oferta; mov ecx, [0xae1050] -> thiscall, ret 4),
///   status 8 e a notícia da aceitação (0x8d76d0: push oferta, ret 4); com a decisão, a notícia da recusa em se mudar
///   (0x8d8310: push 0; push oferta, ret 8), status 0xf, +0x2e = 3, os lances desmarcados, 10 dias, e se a oferta e as
///   da lista em +0 estão fechadas (0x8d0d90, stdcall(oferta)), o bit 0 do registro de 8 bytes do alvo em [this+0x86e]
///   apagado.
/// - 2, 3, 4, 0xf, 0x10, 0x14: com 0xf e +0x2e < 3, +0x2e = 3 e 10 dias; senão a oferta sai (0x8d0f60: push oferta,
///   ret 4).
/// - 1: comprador humano: status 4, os lances desmarcados e 10 dias; senão 0x8c0f40 (push vaga, ret 4) com +0x1f = 1 ou
///   0x8c0810 (push 0; push vaga, ret 8).
/// - 5, 6: comprador humano: status 9 e 10 dias; senão o contrato ao alvo (0x8c1e40: push oferta; push alvo; push
///   comprador, ret 0xc).
/// - 7 com troca (+4): vendedor humano: status 8 e 10 dias; senão, numa cópia da lista (0x68d2f0: push lista; lea ecx,
///   local, ret 4), o contrato a cada jogador da troca até a oferta ficar ativa.
/// - 0xa: as permissões (0x8c7690: push 0; push oferta, ret 8). 0xe: a conclusão (0x8c34e0: push 0; push oferta, ret 8).
/// - 0x11, 0x12: status 0; com +0x10, a oferta passa aos clubes (0x8c2f50: push oferta, ret 4); senão, com técnico
///   humano no clube do contrato (0x7875e0, stdcall(clube): o ecx = [0xb7431c] não é lido), a notícia da oferta feita
///   (0x8d3090: push oferta, ret 4) e 28 dias; senão o valor da oferta (0x8e7b50: mov ecx, oferta -> thiscall(oferta, 0,
///   0, 0, 0, 1), ret 0x14) e a previsão do vendedor (vendedor, alvo, valor, comprador, −1, 0): 0 aceita (0x8c0080),
///   1 ou 2 recusa (0x8bf680: push 0; push 0; push vaga, ret 0xc).
pub(crate) unsafe extern "thiscall" fn process_due_offer(this: usize, slot: usize) {
    let list_new: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let destroy: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let mut l1 = [0u8; 0x10];
    list_new(l1.as_mut_ptr() as usize);
    process_due_offer_body(this, slot);
    destroy(l1.as_mut_ptr() as usize);
}

/// Body of TRANSFER_MANAGER::process_due_offer (0x8c63a0): advances the offer in slot `vaga` by its status (AI
/// response, rejection, contract, permits, completion, news, rescheduling).
pub(crate) unsafe fn process_due_offer_body(this: usize, slot: usize) {
    let aceitar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_ACCEPT_TRANSFER_BID);
    let respond: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_AI_RESPOND_TO_BID);
    let prever: unsafe extern "stdcall" fn(usize, usize, i64, usize, i64, u32) -> u8 = core::mem::transmute(FN_SHORTLIST_MANAGER_PREDICT_SELLING_CLUB_RESPONSE);
    let recusar_emprestimo: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_REJECT_LOAN_OFFER);
    let recusar: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_REJECT_TRANSFER_OFFER);
    let decide: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_CONTRACT_MANAGER_SCHEDULE_DECISION_FOR_TRANSFER);
    let accepted: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_POST_OFFER_ACCEPTED_NEWS);
    let recusa_mudar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_POST_PLAYER_REJECTS_MOVE_NEWS);
    let fechada: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(FN_TRANSFER_OFFER_IS_CLOSED);
    let remove: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_REMOVE_OFFER);
    let ia_1a: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_SUBMIT_LOAN_CONTRACT_OFFER);
    let ia_1b: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_REVIEW_AI_CLUB_BID);
    let contract: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_OFFER_CONTRACT_TO_OFFER_PLAYER);
    let permissoes: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_CHECK_OFFER_WORK_PERMITS);
    let concluir: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_TRY_COMPLETE_OFFER);
    let pass_ball: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_ADVANCE_OFFER_TO_CLUB_APPROVAL);
    let human_manager_record_checked: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(FN_NEWS_CLUB_HAS_HUMAN_MANAGER);
    let noticia_feita: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_POST_OFFER_MADE_NEWS);
    let value: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> i32 = core::mem::transmute(FN_TRANSFER_OFFER_GET_WEIGHTED_VALUE);
    let o = read_u32_as_usize(slot);
    let target = transfer_manager_staff_by_id(read_u32_as_usize(o + offer::TARGET_STAFF_ID));
    let seller = contract_club_filtered(target, true);
    let buyer = transfer_manager_club_by_id(read_u32_as_usize(o + offer::BUYER_CLUB_ID));
    let s = read_u8(o + offer::STATUS);
    if s == 0 {
        if seller == 0 {
            aceitar(this, slot, 0);
            return;
        }
        if transfer_manager_is_human_club(seller) {
            write_u8(o + offer::STATUS, 4);
            schedule_offer_days(this, o, 0xe);
            clear_offer_bid_markers(this, o);
            return;
        }
        if read_u8(read_u32_as_usize(slot) + 0x1f) != 1 {
            crate::comparison::branch("cqd_1 resposta da IA");
            respond(this, slot, 0); // cqd1: the AI response to the bid
            return;
        }
        if prever(seller, target, 0, buyer, -1, 1) != 0 {
            recusar_emprestimo(this, slot, 0, 0);
            return;
        }
        let r = decide(read_u32_as_usize(G_CONTRACT_MANAGER), read_u32_as_usize(slot));
        write_u8(read_u32_as_usize(slot) + 0x2c, 8);
        accepted(this, read_u32_as_usize(slot));
        if r == 0 {
            return;
        }
        recusa_mudar(this, read_u32_as_usize(slot), 0);
        write_u8(read_u32_as_usize(slot) + 0x2c, 0xf);
        write_u8(read_u32_as_usize(slot) + 0x2e, 3);
        clear_offer_bid_markers(this, read_u32_as_usize(slot));
        schedule_offer_days(this, read_u32_as_usize(slot), 0xa);
        let o3 = read_u32_as_usize(slot);
        if fechada(o3) == 0 {
            return;
        }
        let l = read_u32_as_usize(o3);
        if l != 0 {
            write_u32_from_usize(l + llist::CURSOR, read_u32_as_usize(l + llist::HEAD));
            loop {
                let p = transfer_manager_llist_next(read_u32_as_usize(o3));
                if p == 0 {
                    break;
                }
                if fechada(read_u32_as_usize(read_u32_as_usize(this).wrapping_add(read_u32_as_usize(p).wrapping_mul(4)))) == 0 {
                    return;
                }
            }
        }
        let m = read_u32_as_usize(this + transfer_manager_fields::PERSON_FLAGS).wrapping_add(read_u32_as_usize(o3 + offer::TARGET_STAFF_ID).wrapping_mul(8));
        if read_u8(m) & 1 != 0 {
            write_u8(m, read_u8(m) & 0xfe);
        }
        return;
    }
    if matches!(s, 2 | 3 | 4 | 0xf | 0x10 | 0x14) {
        if s == 0xf && read_u8(o + offer::RESPONSE_ROUND) < 3 {
            write_u8(o + offer::RESPONSE_ROUND, 3);
            schedule_offer_days(this, o, 0xa);
        } else {
            remove(this, o);
        }
        return;
    }
    if s == 1 {
        if transfer_manager_is_human_club(buyer) {
            write_u8(o + offer::STATUS, 4);
            clear_offer_bid_markers(this, o);
            schedule_offer_days(this, o, 0xa);
        } else if read_u8(read_u32_as_usize(slot) + 0x1f) == 1 {
            ia_1a(this, slot);
        } else {
            ia_1b(this, slot, 0);
        }
        return;
    }
    if s == 5 || s == 6 {
        if transfer_manager_is_human_club(buyer) {
            write_u8(o + offer::STATUS, 9);
            schedule_offer_days(this, o, 0xa);
        } else {
            crate::comparison::branch("cqd_1 contrato ao jogador");
            contract(this, buyer, transfer_manager_staff_by_id(read_u32_as_usize(o + offer::TARGET_STAFF_ID)), o); // cqd1: o contrato ao alvo
        }
        return;
    }
    if read_u32_as_usize(o + offer::EXCHANGE_PLAYERS) != 0 && s == 7 {
        if transfer_manager_is_human_club(seller) {
            write_u8(o + offer::STATUS, 8);
            schedule_offer_days(this, o, 0xa);
            return;
        }
        let list_new: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
        let copy: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_LLIST_COPY_FROM);
        let destroy: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
        let mut l2 = [0u8; 0x10];
        let l = l2.as_mut_ptr() as usize;
        list_new(l);
        copy(l, read_u32_as_usize(o + offer::EXCHANGE_PLAYERS));
        write_u32_from_usize(l + llist::CURSOR, read_u32_as_usize(l + llist::HEAD));
        loop {
            let p = transfer_manager_llist_next(l);
            if p == 0 {
                break;
            }
            contract(this, seller, transfer_manager_staff_by_id(read_u32_as_usize(p)), o);
            let s2 = read_u8(o + offer::STATUS);
            if matches!(s2, 2 | 3 | 4 | 0x10 | 0x14) || (s2 == 0xf && read_u8(o + offer::RESPONSE_ROUND) >= 3) {
                break;
            }
        }
        destroy(l);
        return;
    }
    if s == 0xa {
        permissoes(this, o, 0);
        return;
    }
    if s == 0xe {
        concluir(this, o, 0);
        return;
    }
    if s != 0x11 && s != 0x12 {
        return;
    }
    write_u8(o + offer::STATUS, 0);
    if read_u8(o + offer::TERMS_STATE) != 0 {
        pass_ball(this, o);
        return;
    }
    let club = contract_club_filtered(transfer_manager_staff_by_id(read_u32_as_usize(o + offer::TARGET_STAFF_ID)), false);
    if human_manager_record_checked(club) != 0 {
        noticia_feita(this, o);
        schedule_offer_days(this, o, 0x1c);
        return;
    }
    let v = value(o, 0, 0, 0, 0, 1);
    let r = prever(seller, transfer_manager_staff_by_id(read_u32_as_usize(o + offer::TARGET_STAFF_ID)), v as i64, buyer, -1, 0);
    if r == 0 {
        aceitar(this, slot, 0);
    } else if (r as i8) > 0 && (r as i8) <= 2 {
        recusar(this, slot, 0, 0);
    }
}

/// Checks an offer's restrictions: completion date, board veto, budget (1.1 times, plus the shortlist offers), squad
/// room and the nation rule; 0 allows, 1 or 2 block.
///
/// Ret 0x10; SEH frame (the local date and list, the list built by 0x68d020 and destroyed by 0x68d040):
/// `TRANSFER_MANAGER::check_offer_restrictions(offer, decider, reason, no_squad)`; 0 allows, 1 or 2 block. The buyer is
/// +8, the target +0xc and the seller the club of his contract (except type 5).
///
/// - With +0x30 & 0xf and status not 0x13: the completion date (0x8cee70: push &date; push offer, ret 8) goes into the
/// reason; a reason different from 0 gives 2; today before the date gives 0.
///
/// - Without `no_squad`, a target with a player sheet (+0x61) and type 11..15 in +0x11 (0x53b620, cdecl of the byte):
/// the decider is the seller with +0x2f = 0 and the seller not obliged to accept (0x8cd240, stdcall(offer, 0, 0)), or
/// the decider is the buyer: the veto (0x8cdb80: push reason; push decider; push target; push seller; push buyer; push
/// offer, ret 0x18) gives 2.
///
/// - The buyer's budget (0x5a0590: push buyer; mov ecx, `[0xae2a38]`, ret 4; 0x598800: push 0 ×4; push target; mov ecx,
/// finances, ret 0x14, returns i64; negative becomes 0) rounded to the next step (0x43f4e0: push 4; lea ecx, &money,
/// ret 4) and the offer value (0x8e7a70: mov ecx, offer -> thiscall(offer, 0, 1, 1, 0), ret 0x10). The decider is the
/// buyer: value > budget × 1.1: reason 3; a human buyer gives 1; otherwise the budget plus the +0x1b of the buyer's
/// offers (0x8cdd80: push 0; push buyer; push person, ret 0xc) for the players on his scouting list (0x8420e0: push
/// &list; push buyer; mov ecx, `[this+0x856]`, ret 8) exceeding the value gives 1, otherwise 2. Within budget, no room
/// in the squad (0x8cd450: push reason; push no_squad; push offer; push club, ret 0x10) gives 1.
///
/// - The decider is the seller: no room in his squad gives 1. Reason 0. With an exchange (+4) or type 11..15: the rule
/// of the decider's nation (`[this+0x8ac]`, the one whose +0xa equals `[decider+0x53]`): virtual +0xc (push reason;
/// push 0xae2c90; push decider; push offer; mov ecx, rule) different from 0 gives 0; reason 9 with +0x30 & 0xf = 5
/// gives 0; otherwise 2. No rule: 0.
/// Original: `TRANSFER_MANAGER::check_offer_restrictions` (0x8cfc90, transfer_manager.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x8cfc90 } (thiscall, ret 0x10, transfer_manager.cpp, quadro SEH: a data e a lista locais, a lista construída pela
/// 0x68d020 e destruída pela 0x68d040): `TRANSFER_MANAGER::check_offer_restrictions(oferta, decide, motivo, sem_elenco)`;
/// 0 libera, 1 ou 2 barram. O comprador é o +8, o alvo o +0xc e o vendedor o clube do contrato dele (fora o tipo 5).
/// - Com +0x30 & 0xf e status fora de 0x13: a data de conclusão (0x8cee70: push &data; push oferta, ret 8) vai ao
///   motivo; motivo diferente de 0 dá 2; hoje antes da data dá 0.
/// - Sem `sem_elenco`, alvo com jogador (+0x61) e tipo 11..15 em +0x11 (0x53b620, cdecl do byte): decide o vendedor com
///   +0x2f = 0 e o vendedor não obrigado a aceitar (0x8cd240, stdcall(oferta, 0, 0)), ou decide o comprador: o veto
///   (0x8cdb80: push motivo; push decide; push alvo; push vendedor; push comprador; push oferta, ret 0x18) dá 2.
/// - O orçamento do comprador (0x5a0590: push comprador; mov ecx, [0xae2a38], ret 4; 0x598800: push 0 ×4; push alvo;
///   mov ecx, finanças, ret 0x14, devolve i64; negativo vira 0) ao degrau seguinte (0x43f4e0: push 4; lea ecx, &dinheiro,
///   ret 4) e o valor da oferta (0x8e7a70: mov ecx, oferta -> thiscall(oferta, 0, 1, 1, 0), ret 0x10). Decide o
///   comprador: valor > orçamento × 1,1: motivo 3; comprador humano dá 1; senão o orçamento mais o +0x1b das ofertas do
///   comprador (0x8cdd80: push 0; push comprador; push pessoa, ret 0xc) pelos jogadores da lista de observação dele
///   (0x8420e0: push &lista; push comprador; mov ecx, [this+0x856], ret 8) passando do valor dá 1, senão 2. Dentro do
///   orçamento, sem vaga no elenco (0x8cd450: push motivo; push sem_elenco; push oferta; push clube, ret 0x10) dá 1.
/// - Decide o vendedor: sem vaga no elenco dele dá 1. Motivo 0. Com troca (+4) ou tipo 11..15: a regra da nação do
///   decide ([this+0x8ac], a de +0xa igual a [decide+0x53]): o virtual +0xc (push motivo; push 0xae2c90; push decide;
///   push oferta; mov ecx, regra) diferente de 0 dá 0; motivo 9 com +0x30 & 0xf = 5 dá 0; senão 2. Sem regra: 0.
pub(crate) unsafe extern "thiscall" fn check_offer_restrictions(this: usize, o: usize, decides: usize, reason: *mut u16, without_squad: u32) -> u8 {
    let default: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    let conclusao: unsafe extern "thiscall" fn(usize, usize, usize) -> u16 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_OFFER_COMPLETION_DATE);
    let kind: unsafe extern "C" fn(i8) -> u32 = core::mem::transmute(FN_DATABASE_IS_PLAYER_JOB);
    let deve_aceitar: unsafe extern "stdcall" fn(usize, u32, usize) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_SELLING_CLUB_MUST_ACCEPT);
    let veto: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, *mut u16) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_CHECK_BOARD_VETO);
    let finances: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FINANCETABLE_GET_CLUB_FINANCE);
    let orcamento: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) -> i64 = core::mem::transmute(FN_FINANCE_GET_TRANSFER_BUDGET);
    let apply_attribute_step: unsafe extern "thiscall" fn(usize, u32) -> i32 = core::mem::transmute(FN_CASH_STEP_UP);
    let total: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> i32 = core::mem::transmute(FN_TRANSFER_OFFER_GET_TOTAL_VALUE);
    let slot: unsafe extern "thiscall" fn(usize, usize, usize, u32, *mut u16) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_CHECK_SQUAD_LIMITS);
    let mut data = [0u8; 8];
    let pd = data.as_mut_ptr() as usize;
    default(pd);
    let buyer = transfer_manager_club_by_id(read_u32_as_usize(o + offer::BUYER_CLUB_ID));
    let target = transfer_manager_staff_by_id(read_u32_as_usize(o + offer::TARGET_STAFF_ID));
    let seller = contract_club_filtered(target, true);
    if read_u8(o + offer::COMPLETION_TIMING) & 0xf != 0 && read_u8(o + offer::STATUS) != 0x13 {
        let r = conclusao(this, o, pd);
        if !reason.is_null() {
            *reason = r;
        }
        if r != 0 {
            return 2;
        }
        let (ha, da) = (read_i16(G_CURRENT_DATE_YEAR) as u16, read_i16(pd + cm_date::YEAR) as u16);
        if ha < da || (ha == da && read_i16(G_CURRENT_DATE) < read_i16(pd)) {
            return 0;
        }
    }
    if without_squad == 0 && read_u32_as_usize(target + staff::PLAYER) != 0 && kind(read_u8(o + offer::JOB_TYPE) as i8) != 0 {
        let mut vetar = false;
        if decides == seller && read_u8(o + offer::PRE_CONTRACT) == 0 && deve_aceitar(o, 0, 0) == 0 {
            vetar = true;
        } else if decides == buyer {
            vetar = true;
        }
        if vetar && veto(this, o, buyer, seller, target, decides, reason) != 0 {
            return 2;
        }
    }
    let mut b = orcamento(finances(read_u32_as_usize(G_FINANCE_TABLE), buyer), target, 0, 0, 0, 0);
    if b < 0 {
        b = 0;
    }
    let mut dinheiro = b as u32;
    let v = apply_attribute_step(&mut dinheiro as *mut u32 as usize, 4) as i64;
    let t = total(o, 0, 1, 1, 0);
    if decides == buyer {
        crate::comparison::branch("cqd_1 orçamento");
        if t as f64 > v as f64 * 1.1 { // cqd1: acima do orçamento
            if !reason.is_null() {
                *reason = 3;
            }
            if transfer_manager_is_human_club(buyer) {
                return 1;
            }
            let list_new: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
            let destroy: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
            let entradas: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_SHORTLIST_MANAGER_COLLECT_CLUB_SHORTLIST_ENTRIES);
            let offer_of_club: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> usize = core::mem::transmute(FN_TRANSFER_MANAGER_FIND_CLUB_OFFER_FOR_PLAYER);
            let mut l = [0u8; 0x10];
            let pl = l.as_mut_ptr() as usize;
            list_new(pl);
            entradas(read_u32_as_usize(this + transfer_manager_fields::SHORTLIST_MANAGER), buyer, pl);
            write_u32_from_usize(pl + llist::CURSOR, read_u32_as_usize(pl + llist::HEAD));
            let mut sum = v;
            loop {
                let p = transfer_manager_llist_next(pl);
                if p == 0 {
                    break;
                }
                let x = offer_of_club(this, transfer_manager_staff_by_id(read_u32_as_usize(p)), buyer, 0);
                if x != 0 {
                    sum = sum.wrapping_add(read_u32_as_usize(x + offer::FEE) as i32 as i64);
                }
            }
            destroy(pl);
            return if sum > t as i64 { 1 } else { 2 };
        }
        if slot(this, buyer, o, without_squad, reason) == 0 {
            return 1;
        }
    }
    if seller == decides && slot(this, seller, o, without_squad, reason) == 0 {
        return 1;
    }
    if !reason.is_null() {
        *reason = 0;
    }
    if read_u32_as_usize(o + offer::EXCHANGE_PLAYERS) == 0 && kind(read_u8(o + offer::JOB_TYPE) as i8) == 0 {
        return 0;
    }
    for k in 0..0x1c {
        let r = read_u32_as_usize(this + transfer_manager_fields::TRANSFER_RULES + k * 4);
        if read_u32_as_usize(r + transfer_rules::NATION_ID) == read_u32_as_usize(decides + club::NATION) {
            if r == 0 {
                return 0;
            }
            let rule: unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u16) -> u32 =
                core::mem::transmute(read_u32_as_usize(read_u32_as_usize(r) + 0xc));
            if rule(r, o, decides, G_CURRENT_DATE, reason) != 0 {
                return 0;
            }
            if *reason == 9 && read_u8(o + offer::COMPLETION_TIMING) & 0xf == 5 {
                return 0;
            }
            return 2;
        }
    }
    0
}

/// Contract record -> club record (id at +4); 0 for no contract, type not checked.
///
/// [pt] O clube do contrato `c` (+4 = id; 0 sem contrato), sem olhar o tipo.
pub(crate) unsafe fn transfer_manager_club_of_contract(c: usize) -> usize {
    if c == 0 { 0 } else { friendly_club_by_id(read_usize(c + contract::CLUB)) }
}

/// Calls the finance routine `rotina` (sales 0x599800 or purchases 0x599890) on finances `fin` with the value
/// sign-extended to 64 bits.
///
/// [pt] O valor (i32 estendido para 64 bits, como o `cdq` da original) entra nas vendas (0x599800) ou sai pelas compras
/// (0x599890) das finanças `fin` (thiscall(fin, baixo, alto), ret 8).
pub(crate) unsafe fn post_finance_amount(routine: usize, fin: usize, v: i32) {
    let f: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(routine);
    f(fin, v as u32, (v >> 31) as u32);
}

/// Constructs a local news item of the given type (NEWSITEM::NEWSITEM).
///
/// [pt] A notícia local: construir (0x77bbe0: thiscall(notícia, tipo, 0), ret 8), gravar um dado (0x785790:
/// thiscall(notícia, k, v), ret 8) e entregar ao técnico do clube (0x7861e0: thiscall([0xb7431c], notícia, clube),
/// ret 8).
pub(crate) unsafe fn transfer_manager_construct_news_item(n: usize, kind: u32) {
    let f: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    f(n, kind, 0);
}

/// Sends a news item to the club's manager (G_NEWS send_to_club_manager).
pub(crate) unsafe fn send_news_to_club_manager(n: usize, club: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    f(read_usize(G_NEWS), n, club);
}

/// The comparers' rehearsal: the copy runs without the news items and without the club totals (0x448ad0), which
/// allocate or deliver and cannot run twice. FORMER_CLUB_FEE_DELIVERIES counts the deliveries the original routine
/// would make (they may change [0xae1080] through the automatic reply of a human on holiday).
///
/// [pt] O ensaio dos comparadores: a cópia roda sem as notícias e sem os totais do clube (0x448ad0), que alocam ou entregam
/// e não podem rodar duas vezes; FORMER_CLUB_FEE_DELIVERIES conta as entregas que a original faria (elas podem mexer em [0xae1080]
/// pela resposta automática do humano de férias).
pub(crate) static mut FORMER_CLUB_FEE_DRY_RUN: bool = false;
pub(crate) static mut FORMER_CLUB_FEE_DELIVERIES: u32 = 0;

/// Pays former clubs their fee cuts: for a player over 22, (offer fee / 20) / sum of weights × the weight of each of up
/// to five former clubs goes into the buyer's sales; news 0xfc3 goes to the former club when the cut exceeds 11 ×
/// reputation − 10000.
///
/// Ret 4; SEH frame without a destructor; this is not used: `TRANSFER_MANAGER::pay_former_club_fee_cuts(offer)`, from a
/// completed transfer with a fee (+0x1b) of a player without +0x69 (0x8c9390). A player (+0xc of the offer) over 22
/// years old (+0x18): news 0xfc3 (local; 0 = id, 1..3 = the +0x33 of +4/+8/+0xc, 4 = the buyer +8, 7 = the contract
/// club or −1); the sum (16 bits) of the weights (byte +0x12) of the five former-club slots of the 79-byte record
/// (`[0xae2d0c]`, step 5, club at +0xe, −1 = empty slot; a sum of 0 becomes 1). Each slot with a club: ((fee / 20) /
/// sum) × weight (a weight below 1 counts as 1) goes into the sales of the BUYER's finances (0x5a0590 with +8,
/// 0x599800; this is what the original does); above 11 × reputation (+0x80) − 10000 of the slot's club, the news with 5
/// = the amount and 6 = the club id goes to its manager.
/// Original: `TRANSFER_MANAGER::pay_former_club_fee_cuts` (0x8e4f20, transfer_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x8e4f20 } (thiscall, ret 4, transfer_manager.cpp, quadro SEH sem destrutor; o this não é usado):
/// `TRANSFER_MANAGER::pay_former_club_fee_cuts(oferta)`, pela transferência efetivada com valor (+0x1b) de jogador sem
/// +0x69 (0x8c9390). Jogador (+0xc da oferta) com mais de 22 anos (+0x18): a notícia 0xfc3 (local; 0 = id, 1..3 = os
/// +0x33 de +4/+8/+0xc, 4 = o comprador +8, 7 = o clube do contrato ou −1); a soma (16 bits) dos pesos (byte +0x12)
/// das cinco vagas de clube anterior do registro de 79 bytes ([0xae2d0c], passo 5, clube em +0xe, −1 = vaga vazia;
/// soma 0 vira 1). Cada vaga com clube: ((valor / 20) / soma) × peso (peso abaixo de 1 vale 1) entra nas vendas das
/// finanças do COMPRADOR (0x5a0590 com +8, 0x599800; é o que a original faz); acima de 11 × reputação (+0x80) − 10000
/// do clube da vaga, a notícia com 5 = o valor e 6 = o id do clube vai ao técnico dele.
pub(crate) unsafe extern "thiscall" fn pay_former_club_fee_cuts(_this: usize, o: usize) {
    let p = staff_records_staff_by_id(read_usize(o + offer::TARGET_STAFF_ID));
    if (read_u8(p + staff::AGE) as i8) < 0x17 { // cqb6: só acima de 22 anos
        return;
    }
    crate::comparison::branch("cqb_6 parte dos clubes anteriores");
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    if !FORMER_CLUB_FEE_DRY_RUN {
        transfer_manager_construct_news_item(n, 0xfc3);
    }
    let r = read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(p).wrapping_mul(0x4f));
    let mut sum: i16 = 0;
    for k in 0..5 {
        if read_usize(r + 0xe + 5 * k) as i32 != -1 {
            sum = sum.wrapping_add(read_u8(r + 0x12 + 5 * k) as i8 as i16);
        }
    }
    let vinte = (read_usize(o + offer::FEE) as i32) / 20;
    let divisor = if sum == 0 { 1 } else { sum as i32 };
    if !FORMER_CLUB_FEE_DRY_RUN {
        set_news_slot(n, 0, read_usize(p) as u32);
        set_news_slot(n, 1, read_usize(read_usize(p + staff::FIRST_NAME) + 0x33) as u32);
        set_news_slot(n, 2, read_usize(read_usize(p + staff::SECOND_NAME) + 0x33) as u32);
        set_news_slot(n, 3, read_usize(read_usize(p + staff::COMMON_NAME) + 0x33) as u32);
        set_news_slot(n, 4, read_usize(o + offer::BUYER_CLUB_ID) as u32);
    }
    let c = injury_main_contract_stored(p);
    let v7 = if c == 0 || read_u8(c + contract::CONTRACT_TYPE) & 0x3f == 5 || transfer_manager_club_of_contract(c) == 0 {
        u32::MAX
    } else {
        let c2 = injury_main_contract_stored(p);
        // simplification: the original reads [0] if the second contract were missing; unreachable (the same contract has just passed)
        if c2 != 0 && read_u8(c2 + contract::CONTRACT_TYPE) & 0x3f != 5 { read_usize(transfer_manager_club_of_contract(c2)) as u32 } else { u32::MAX }
    };
    if !FORMER_CLUB_FEE_DRY_RUN {
        set_news_slot(n, 7, v7);
    }
    for k in 0..5 {
        let r = read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(p).wrapping_mul(0x4f)) + 5 * k;
        let i = read_usize(r + 0xe);
        if i as i32 == -1 {
            continue;
        }
        let weight = read_u8(r + 0x12) as i8;
        let weight = if weight < 1 { 1 } else { weight as i32 };
        let club = friendly_club_by_id(i);
        let value = vinte.wrapping_div(divisor).wrapping_mul(weight); // cqb6: the slot share
        let fin = european_cup_club_finance(friendly_club_by_id(read_usize(o + offer::BUYER_CLUB_ID)));
        crate::comparison::branch("cqb_6 parte paga");
        post_finance_amount(0x599800, fin, value); // cqb6: enters the sales of the buyer
        if value > (read_i16(club + club::REPUTATION) as i32).wrapping_mul(11).wrapping_sub(10000) {
            FORMER_CLUB_FEE_DELIVERIES += 1;
            if !FORMER_CLUB_FEE_DRY_RUN {
                set_news_slot(n, 5, value as u32);
                set_news_slot(n, 6, read_usize(club) as u32);
                send_news_to_club_manager(n, club);
            }
        }
    }
}

/// Pays the next-sale percentage: binary-searches the person's clause with the contract club; the cut (percentage ×
/// offer fee plus exchange players, or the person's value) from 1000 up: news, transfer totals, paid into the clause
/// club and out of the contract club.
///
/// Ret 8: `TRANSFER_MANAGER::pay_next_sale_percentage(person, offer)`, from a completed transfer with the person's flag
/// 8 (0x8c91e0). With a contract (and a contract club), looks by binary search (0x946464, 0xd-byte entries in
/// `[this+0x880]`, count `[this+0x888]`, comparison 0x8e54d0) for the clause with +1 = contract club id and +9 = person
/// id. Found: the base is the offer fee (+0x1b) plus the value (+0x52) of each exchange player (+4, LLIST of ids,
/// cursor restarted) when the person is the offer's one (+0xc); otherwise the person's value for the contract club
/// (0x4d0c60 with 0, −1, 0). The cut = percentage (byte +0, signed) × base / 100; from 1000 up: the news (0x8db3a0,
/// with 1 if it was the offer's person), the club's transfer totals (0x448ad0 on `[0xad9dac]`: the cut, the contract
/// club, the clause club +5), the cut goes into the clause club's sales and out through the contract club's purchases.
/// The clause stays (0x8cb4f0 is what removes it).
/// Original: `TRANSFER_MANAGER::pay_next_sale_percentage` (0x8cb0f0, transfer_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x8cb0f0 } (thiscall, ret 8, transfer_manager.cpp): `TRANSFER_MANAGER::pay_next_sale_percentage(pessoa, oferta)`,
/// pela transferência efetivada com a marca 8 da pessoa (0x8c91e0). Com contrato (e clube do contrato), procura pela
/// busca binária (0x946464, entradas de 0xd bytes em [this+0x880], conta [this+0x888], comparação 0x8e54d0) a
/// cláusula com +1 = id do clube do contrato e +9 = id da pessoa. Achada: a base é o valor da oferta (+0x1b) mais o
/// valor (+0x52) de cada jogador da troca (+4, LLIST de ids, cursor recomeçado) quando a pessoa é a da oferta (+0xc);
/// senão o valor da pessoa para o clube do contrato (0x4d0c60 com 0, −1, 0). A parte = percentual (byte +0, com sinal)
/// × base / 100; de 1000 para cima: a notícia (0x8db3a0, com 1 se era a da oferta), os totais de transferência do
/// clube (0x448ad0 em [0xad9dac]: a parte, o clube do contrato, o clube da cláusula +5), a parte entra nas vendas do
/// clube da cláusula e sai pelas compras do clube do contrato. A cláusula fica (quem a tira é a 0x8cb4f0).
pub(crate) unsafe extern "thiscall" fn pay_next_sale_percentage(this: usize, p: usize, o: usize) -> u32 {
    let c = injury_main_contract_stored(p);
    if c == 0 || transfer_manager_club_of_contract(c) == 0 {
        return 0;
    }
    let mut key = [0u8; 0xd];
    core::ptr::write_unaligned(key.as_mut_ptr().add(9) as *mut usize, read_usize(p));
    let c2 = injury_main_contract_stored(p);
    // simplification: the original reads [0] if the second contract were missing; unreachable (the same contract has just passed)
    let id_club = if c2 != 0 { read_usize(transfer_manager_club_of_contract(c2)) } else { usize::MAX };
    core::ptr::write_unaligned(key.as_mut_ptr().add(1) as *mut usize, id_club);
    let search: unsafe extern "C" fn(*const u8, usize, usize, u32, usize) -> usize = core::mem::transmute(FN_CRT_BSEARCH);
    let e = search(key.as_ptr(), read_usize(this + transfer_manager_fields::NEXT_SALE_PERCENTAGES), read_usize(this + transfer_manager_fields::NEXT_SALE_PERCENTAGE_COUNT), 0xd, 0x8e54d0);
    if e == 0 {
        return 0;
    }
    let (base, da_oferta) = if read_usize(p) == read_usize(o + offer::TARGET_STAFF_ID) {
        let mut v = read_usize(o + offer::FEE) as i32;
        let l = read_usize(o + offer::EXCHANGE_PLAYERS);
        if l != 0 {
            let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
            core::ptr::write_unaligned((l + llist::CURSOR) as *mut usize, read_usize(l + llist::HEAD));
            let mut i = next(read_usize(o + offer::EXCHANGE_PLAYERS));
            while i != 0 {
                v = v.wrapping_add(read_usize(staff_records_staff_by_id(read_usize(i)) + 0x52) as i32);
                i = next(read_usize(o + offer::EXCHANGE_PLAYERS));
            }
        }
        (v, 1u32)
    } else {
        let club = transfer_manager_club_of_contract(injury_main_contract_stored(p));
        let value: unsafe extern "thiscall" fn(usize, usize, usize, u32, i32, u32) -> i32 = core::mem::transmute(FN_CONTRACT_MANAGER_CALC_PLAYER_VALUE);
        (value(read_usize(G_CONTRACT_MANAGER), p, club, 0, -1, 0), 0)
    };
    let part = (read_u8(e) as i8 as i32).wrapping_mul(base) / 100;
    if part < 1000 {
        return 0;
    }
    crate::comparison::branch("cqb_6 write_percent_pair pago");
    if !FORMER_CLUB_FEE_DRY_RUN {
        let news: unsafe extern "thiscall" fn(usize, usize, i32, usize, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_POST_SALE_PERCENTAGE_NEWS);
        news(this, e, part, o, da_oferta);
    }
    let da_clausula = friendly_club_by_id(read_usize(e + sell_on_clause::RECEIVING_CLUB_ID));
    let fin_recebe = european_cup_club_finance(da_clausula);
    let fin_paga = european_cup_club_finance(transfer_manager_club_of_contract(injury_main_contract_stored(p)));
    let of_contract = transfer_manager_club_of_contract(injury_main_contract_stored(p));
    if !FORMER_CLUB_FEE_DRY_RUN {
        let totais: unsafe extern "thiscall" fn(usize, i32, usize, usize) -> u32 = core::mem::transmute(FN_CLUB_RECORDS_ADD_TRANSFER_TOTALS);
        totais(read_usize(G_CLUB_RECORDS), part, of_contract, friendly_club_by_id(read_usize(e + sell_on_clause::RECEIVING_CLUB_ID)));
    }
    post_finance_amount(0x599800, fin_recebe, part);
    post_finance_amount(0x599890, fin_paga, part);
    1
}

/// For the comparer: finds the person's next-sale clause (bsearch in [this+0x880]) and returns the finances of the
/// clause club and of the contract club; None if no clause.
///
/// [pt] O que a 0x8cb0f0 vai mexer, para as regiões do comparador: as finanças do clube da cláusula e do clube do
/// contrato, se a cláusula existe (sem gravar nada além de [0xae1080], que a original regrava).
pub(crate) unsafe fn clause_finance_regions(this: usize, p: usize) -> Option<(usize, usize)> {
    let c = injury_main_contract_stored(p);
    if c == 0 || transfer_manager_club_of_contract(c) == 0 {
        return None;
    }
    let mut key = [0u8; 0xd];
    core::ptr::write_unaligned(key.as_mut_ptr().add(9) as *mut usize, read_usize(p));
    core::ptr::write_unaligned(key.as_mut_ptr().add(1) as *mut usize, read_usize(transfer_manager_club_of_contract(c)));
    let search: unsafe extern "C" fn(*const u8, usize, usize, u32, usize) -> usize = core::mem::transmute(FN_CRT_BSEARCH);
    let e = search(key.as_ptr(), read_usize(this + transfer_manager_fields::NEXT_SALE_PERCENTAGES), read_usize(this + transfer_manager_fields::NEXT_SALE_PERCENTAGE_COUNT), 0xd, 0x8e54d0);
    if e == 0 {
        return None;
    }
    Some((european_cup_club_finance(friendly_club_by_id(read_usize(e + sell_on_clause::RECEIVING_CLUB_ID))), european_cup_club_finance(transfer_manager_club_of_contract(c))))
}

/// Sends news 0xfa7: the person, the percentage, the two clause clubs, the cut and the buyer (or the contract club), to
/// the managers of the clause club and the seller.
///
/// Ret 0x10; SEH frame without a destructor; this is not used: `TRANSFER_MANAGER::post_sale_percentage_news(clause,
/// cut, offer, from_offer)`, from the percentage paid (0x8cb0f0). News 0xfa7 (local): 0..3 the clause's person (+9: id
/// and the +0x33 of +4/+8/+0xc), 4 = the percentage (byte +0), 5 = the clause club (+5), 6 = the selling club (+1), 7 =
/// the cut, 8 = `from_offer`, 9 = the buyer (+8 of the offer) with `from_offer`; otherwise the id of the contract club
/// (0x4cf5a0; without a contract or without a club: ends without delivering). Goes to the managers of the clause club
/// and of the seller (0x7861e0).
/// Original: `TRANSFER_MANAGER::post_sale_percentage_news` (0x8db3a0, transfer_manager.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x8db3a0 } (thiscall, ret 0x10, transfer_manager.cpp, quadro SEH sem destrutor; o this não é usado):
/// `TRANSFER_MANAGER::post_sale_percentage_news(cláusula, parte, oferta, da_oferta)`, pelo percentual pago (0x8cb0f0).
/// A notícia 0xfa7 (local): 0..3 a pessoa da cláusula (+9: id e os +0x33 de +4/+8/+0xc), 4 = o percentual (byte +0),
/// 5 = o clube da cláusula (+5), 6 = o clube vendedor (+1), 7 = a parte, 8 = `da_oferta`, 9 = o comprador (+8 da
/// oferta) com `da_oferta`; senão o id do clube do contrato (0x4cf5a0; sem contrato ou sem clube: acaba sem entregar).
/// Vai aos técnicos do clube da cláusula e do vendedor (0x7861e0).
pub(crate) unsafe extern "thiscall" fn post_sale_percentage_news(_this: usize, e: usize, part: i32, o: usize, of_offer: u32) {
    crate::comparison::branch("cqb_6 noticia do write_percent_pair");
    let p = staff_records_staff_by_id(read_usize(e + sell_on_clause::PERSON_ID));
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    transfer_manager_construct_news_item(n, 0xfa7);
    set_news_slot(n, 0, read_usize(p) as u32);
    set_news_slot(n, 1, read_usize(read_usize(p + staff::FIRST_NAME) + 0x33) as u32);
    set_news_slot(n, 2, read_usize(read_usize(p + staff::SECOND_NAME) + 0x33) as u32);
    set_news_slot(n, 3, read_usize(read_usize(p + staff::COMMON_NAME) + 0x33) as u32);
    set_news_slot(n, 4, read_u8(e) as i8 as i32 as u32);
    set_news_slot(n, 5, read_usize(e + sell_on_clause::RECEIVING_CLUB_ID) as u32);
    set_news_slot(n, 6, read_usize(e + sell_on_clause::PAYING_CLUB_ID) as u32);
    set_news_slot(n, 7, part as u32);
    set_news_slot(n, 8, of_offer);
    let v9 = if of_offer != 0 {
        read_usize(o + offer::BUYER_CLUB_ID)
    } else {
        let c = injury_main_contract_stored(p);
        if c == 0 || transfer_manager_club_of_contract(c) == 0 {
            return;
        }
        let c2 = injury_main_contract_stored(p);
        // simplification: the original reads [0] if the second contract were missing; unreachable (the same contract has just passed)
        if c2 != 0 { read_usize(transfer_manager_club_of_contract(c2)) } else { usize::MAX }
    };
    set_news_slot(n, 9, v9 as u32);
    send_news_to_club_manager(n, friendly_club_by_id(read_usize(e + sell_on_clause::RECEIVING_CLUB_ID)));
    send_news_to_club_manager(n, friendly_club_by_id(read_usize(e + sell_on_clause::PAYING_CLUB_ID)));
}

/// Date `a` is after date at `d` (Date operator> 0x456740, date passed by value).
///
/// [pt] A data `a` vem depois da data em `d` (0x456740: thiscall(a, d.dword0, d.dword1), ret 8; a data vai por valor).
pub(crate) unsafe fn transfer_manager_date_is_after(a: usize, d: usize) -> bool {
    let f: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_DATE_OPERATOR_3E);
    f(a, read_usize(d) as u32, read_usize(d + cm_date::LEAP_YEAR) as u32) != 0
}

/// Person's name contains `agulha` (strstr) and the offer's club name equals "Falkirk" (0xa8eb48), the hard-coded
/// exception of the original.
///
/// [pt] O nome (texto C em [pessoa+4]) contém `agulha` (0x945a60, strstr: cdecl(texto, agulha)) e o clube da oferta (+4 =
/// id; nome em +4 do clube) é o de 0xa8eb48 ("Falkirk"; a comparação de bytes que a original faz em linha).
pub(crate) unsafe fn is_falkirk_name_exception(p: usize, agulha: usize, offer: usize) -> bool {
    let name = read_usize(p + 4);
    if name == 0 {
        return false;
    }
    let strstr: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(FN_CRT_STRSTR);
    if strstr(name, agulha) == 0 || offer == 0 {
        return false;
    }
    let mut a = read_usize(G_CLUB_TABLE).wrapping_add(read_usize(offer + 4).wrapping_mul(club::SIZE)) + 4;
    let mut b = 0xa8eb48usize;
    loop {
        let (x, y) = (read_u8(a), read_u8(b));
        if x != y {
            return false;
        }
        if x == 0 {
            return true;
        }
        a += 1;
        b += 1;
    }
}

/// Decides the work permit: by the permit record (matches and refusals), a non-player by reputation, the 2001
/// exceptions, potential above 8500, wage above 34999, youth above 190, under 5 caps no, over 20 with a strong nation
/// yes, and the 16-match national side history (over 11 or 70%), which advances with new matches.
///
/// Ret 0x14: `TRANSFER_MANAGER::is_work_permit_granted(person, nation, offer, record, random)`, from the permit in the
/// scouting list (0x84b180, without offer, record or random), from the accepted contract offer (0x8c38b0, with the
/// offer) and from the offer's permits (0x8c7690). The person's contract (0x4cf5a0, without writing `[0xae1080]`): of
/// type 2 it is the "loan", otherwise the "contract". With the permit record (the given one, or found by binary search
/// 0x946464 in `[this+0x93a]`, count `[this+0x93e]`, 14 bytes, when flag 0x20 exists): without an offer, granted if
/// matches (+4) > 3 × refusals (+6) or refusals < 4; with an offer, the same rule if the loan ends after the offer
/// (+0x2d) or the contract club is not the offer's (+4), and granted if the loan does not end after. Non-player (+0x69
/// without sheet +0x61): reputation (+0xc) above 160. In 2001, the two cases of Collin and Devon at Falkirk. Potential
/// (+0xd of the sheet) above 8500 (with random: 8000 + random(1000)); offer wage (+0xc) above 34999; under 24 years
/// with ability (+7) above 190: granted. Fewer than 5 caps (+0x22): not granted; more than 20 with the nation's
/// reputation (+0x8e) above 3000 and under 32 years: granted. The history advances with the national side's new matches
/// (`[[0xb74318]+4]` + nation id × 0x6e + 2; each one pushes a zero bit) and the count stays. The window: 16 matches,
/// or those of the two-year period (0x58d710, up to 16) from 2003; with more than 11 matches marked (random: 11 −
/// random(3) + random(3)) or more than 70% (70 − random(10) + random(10)): granted.
/// Original: `TRANSFER_MANAGER::is_work_permit_granted` (0x8c7950, transfer_manager.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x8c7950 } (thiscall, ret 0x14, transfer_manager.cpp): `TRANSFER_MANAGER::is_work_permit_granted(pessoa, nação,
/// oferta, registro, sorteio)`, pela permissão na lista de observação (0x84b180, sem oferta, registro nem sorteio),
/// pela oferta de contrato aceita (0x8c38b0, com a oferta) e pelas permissões da oferta (0x8c7690). Contrato da pessoa
/// (0x4cf5a0, sem gravar [0xae1080]): de tipo 2 é o "empréstimo", outro é o "contrato". Com o registro de permissão
/// (o dado, ou achado pela busca binária 0x946464 em [this+0x93a], conta [this+0x93e], 14 bytes, quando a marca 0x20
/// existe): sem oferta, sai se jogos (+4) > 3 × recusas (+6) ou recusas < 4; com oferta, a mesma regra se o empréstimo
/// acaba depois da oferta (+0x2d) ou o clube do contrato não é o da oferta (+4), e sai se o empréstimo não acaba depois.
/// Não jogador (+0x69 sem ficha +0x61): reputação (+0xc) acima de 160. Em 2001, os dois casos do Collin e do Devon no
/// Falkirk. Potencial (+0xd da ficha) acima de 8500 (com sorteio: 8000 + sorteio(1000)); salário da oferta (+0xc) acima
/// de 34999; menos de 24 anos com habilidade (+7) acima de 190: sai. Menos de 5 jogos pela seleção (+0x22): não sai;
/// mais de 20 com reputação da nação (+0x8e) acima de 3000 e menos de 32 anos: sai. O histórico anda pelos jogos novos
/// da seleção ([[0xb74318]+4] + id da nação × 0x6e + 2; cada um empurra um bit zero) e a conta fica. A janela: 16
/// jogos, ou os do período de dois anos (0x58d710, até 16) a partir de 2003; com mais de 11 jogos marcados (sorteio:
/// 11 − sorteio(3) + sorteio(3)) ou mais de 70% (70 − sorteio(10) + sorteio(10)): sai.
pub(crate) unsafe extern "thiscall" fn is_work_permit_granted(this: usize, p: usize, nation: usize, offer: usize, record: usize, sorteio: u32) -> u32 {
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = contract_of(read_usize(G_CONTRACT_MANAGER), p);
    let (emprestimo, contrato) = if c != 0 && read_u8(c + contract::CONTRACT_TYPE) & 0x3f != 2 { (0, c) } else { (c, 0) };
    let mut reg = record;
    let has_record = if reg != 0 {
        true
    } else if (p == 0 || (read_usize(p) as i32) < read_usize(G_STAFF_COUNT) as i32) && read_u8(staff_flags_record(this, p)) & 0x20 != 0 {
        let key = read_usize(p) as u32;
        let search: unsafe extern "C" fn(*const u32, usize, usize, u32, usize) -> usize = core::mem::transmute(FN_CRT_BSEARCH);
        reg = search(&key, read_usize(this + transfer_manager_fields::WORK_PERMITS), read_usize(this + transfer_manager_fields::WORK_PERMIT_COUNT), 0xe, 0x8e55c0);
        reg != 0
    } else {
        false
    };
    if has_record {
        let games = core::ptr::read_unaligned((reg + work_permit::MATCHES_PLAYED) as *const u16) as u32;
        let recusas = core::ptr::read_unaligned((reg + work_permit::MATCHES_MISSED) as *const u16) as u32;
        let by_count = games > recusas * 3 || recusas < 4;
        if offer == 0 {
            if by_count {
                return 1;
            }
        } else {
            let away = (emprestimo != 0 && transfer_manager_date_is_after(offer + 0x2d, emprestimo + contract::CONTRACT_EXPIRES))
                || (contrato != 0 && read_usize(offer + 4) != read_usize(contrato + contract::CLUB));
            if away && by_count {
                return 1;
            }
            if emprestimo != 0 && !transfer_manager_date_is_after(offer + 0x2d, emprestimo + contract::CONTRACT_EXPIRES) {
                return 1;
            }
        }
    }
    let np = read_usize(p + staff::NON_PLAYER);
    if np != 0 && read_usize(p + staff::PLAYER) == 0 {
        return (read_i16(np + non_player::WORLD_REPUTATION) > 0xa0) as u32;
    }
    if read_i16(G_CURRENT_DATE_YEAR) == 0x7d1 && (is_falkirk_name_exception(p, 0x9b5ac0, offer) || read_i16(G_CURRENT_DATE_YEAR) == 0x7d1 && is_falkirk_name_exception(p, 0x9b5ab0, offer)) {
        crate::comparison::branch("cqb_7 excecao de 2001");
        return 1;
    }
    let sheet = read_usize(p + staff::PLAYER);
    if sorteio != 0 {
        if sheet != 0 && read_i16(sheet + player_sheet::WORLD_REPUTATION) as i32 > friendly_random(1000) + 8000 {
            return 1;
        }
    } else if sheet != 0 {
        crate::comparison::branch("cqb_7 confere o potencial");
        if read_i16(sheet + player_sheet::WORLD_REPUTATION) > 0x2134 { // cqb7: potential above 8500
            return 1;
        }
    }
    if offer != 0 && read_usize(offer + offer::TARGET_STAFF_ID) as i32 > 0x88b7 {
        return 1;
    }
    let age = read_u8(p + staff::AGE) as i8;
    if age < 0x18 && sheet != 0 {
        crate::comparison::branch("cqb_7 confere o jovem");
        if read_i16(sheet + player_sheet::POTENTIAL_ABILITY) > 0xbe { // cqb7: the young player above 190
            return 1;
        }
    }
    let games = read_u8(p + staff::INT_APPS);
    if games < 5 {
        return 0;
    }
    if games > 0x14 {
        let nacao = read_usize(p + staff::NATION);
        if nacao != 0 && read_i16(nacao + nation::REPUTATION) > 0xbb8 && age < 0x20 {
            return 1;
        }
    }
    // the history from the national team’s new matches
    let national_team = read_i16(read_usize(read_usize(G_NATIONAL_TEAMS) + 4).wrapping_add(read_usize(read_usize(p + staff::NATION)).wrapping_mul(0x6e)) + 2);
    let new = national_team.wrapping_sub(read_i16(staff_flags_record(this, p) + 5));
    if new > 0 {
        for _ in 0..new {
            let h = staff_flags_record(this, p) + 3;
            core::ptr::write_unaligned(h as *mut u16, core::ptr::read_unaligned(h as *const u16) << 1);
            *(h as *mut u8) &= 0xfe;
        }
    }
    core::ptr::write_unaligned((staff_flags_record(this, p) + 5) as *mut i16, national_team);
    let year = read_i16(G_CURRENT_DATE_YEAR) as u16;
    let window: i16 = if (year as i32) - 2 < 0x7d1 {
        0x10
    } else {
        let month: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_DATE_GET_MONTH);
        let ate = month(G_CURRENT_DATE);
        let de = month(G_CURRENT_DATE);
        let period: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) -> u16 = core::mem::transmute(FN_FIFA_RANKINGS_COUNT_RESULTS_IN_PERIOD);
        let n = period(read_usize(read_usize(G_NATIONAL_TEAMS) + 0x1c), read_usize(p + staff::NATION), de, (year as u32).wrapping_sub(2), ate, year as u32) as i16;
        if n > 0x10 { 0x10 } else { n }
    };
    let mut marcados: i16 = 0;
    let mut k: i16 = 0;
    while k < window && k < 0x10 {
        let mask = 1u16 << k; // ftol(pow(2.0, k)) (0x945ff0 and 0x945640): exact from 0 to 15
        if core::ptr::read_unaligned((staff_flags_record(this, p) + 3) as *const u16) & mask != 0 {
            marcados += 1;
        }
        k += 1;
    }
    if sorteio != 0 {
        let a = friendly_random(3);
        let limit = 0xb - a + friendly_random(3);
        if marcados as i32 > limit {
            return 1;
        }
        if window == 0 {
            return 0;
        }
        let b = friendly_random(10);
        let limit = 0x46 - b + friendly_random(10);
        return ((marcados as i32 * 100) / window as i32 > limit) as u32;
    }
    crate::comparison::branch("cqb_7 jogos pela selecao");
    if marcados > 0xb { // cqb7: mais de 11 jogos marcados
        return 1;
    }
    if window == 0 {
        return 0;
    }
    ((marcados as i32 * 100) / window as i32 > 0x46) as u32
}

/// Returns the squad status for the refusal reason (0x200: 1, 0x400: 2, 0x2000: 5) or that of the contract with the
/// club (+0x4f >> 4); no contract, 0.
///
/// Ret 8; this is not used: `transfer_manager_get_squad_status_for_reason(person, reason)`, from the offer screen. The
/// person's contract (0x4cf5a0, stored in `[0xae1080]`) gives the club; the contract with it (0x4cf5f0); without it, 0.
/// The reason (signed 16 bits) 0x200 gives 1, 0x400 gives 2, 0x2000 gives 5; the others, the contract status (+0x4f >>
/// 4). Returns in al.
/// Original: `transfer_manager_get_squad_status_for_reason` (0x8c5800, transfer_manager.cpp, thiscall (this sem uso), ret 8).
///
/// [pt] { 0x8c5800 } (thiscall, ret 8, transfer_manager.cpp; o this não é usado): `transfer_manager_get_squad_status_for_reason
/// (pessoa, motivo)`, pela tela da oferta. O contrato da pessoa (0x4cf5a0, guardado em [0xae1080]) dá o clube; o
/// contrato com ele (0x4cf5f0); sem ele, 0. O motivo (16 bits com sinal) 0x200 dá 1, 0x400 dá 2, 0x2000 dá 5; os
/// outros, o estatuto do contrato (+0x4f >> 4). Devolve no al.
pub(crate) unsafe extern "thiscall" fn transfer_manager_get_squad_status_for_reason(_this: usize, p: usize, reason: u32) -> u32 {
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = contract_of(read_usize(G_CONTRACT_MANAGER), p);
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
    let with_club: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let cc = with_club(read_usize(G_CONTRACT_MANAGER), p, injury_club_of_contract(c));
    if cc == 0 {
        return 0;
    }
    match reason as i16 as i32 {
        0x200 => 1,
        0x400 => 2,
        0x2000 => 5,
        _ => (read_u8(cc + contract::SQUAD_STATUS) >> 4) as u32,
    }
}

/// Returns the address of slot i of the negotiation array; out of range, the start of the array.
///
/// Ret 4: `TRANSFER_MANAGER::get_negotiation_slot(i)`, from the automatic reply to news items (0x782f70) and from the
/// screens: the address of slot `i` of the negotiation vector (`[this+0xc]`, count `[this+0x10]`, signed); outside the
/// count, the start of the vector.
/// Original: `TRANSFER_MANAGER::get_negotiation_slot` (0x8c6200, transfer_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x8c6200 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::get_negotiation_slot(i)`, pela resposta
/// automática às notícias (0x782f70) e pelas telas: o endereço da vaga `i` do vetor de negociações ([this+0xc], conta
/// [this+0x10], com sinal); fora da conta, o começo do vetor.
pub(crate) unsafe extern "thiscall" fn get_negotiation_slot(this: usize, i: i32) -> usize {
    if i < read_usize(this + transfer_manager_fields::NEGOTIATION_CAPACITY) as i32 {
        read_usize(this + transfer_manager_fields::NEGOTIATIONS).wrapping_add((i as usize).wrapping_mul(4))
    } else {
        read_usize(this + transfer_manager_fields::NEGOTIATIONS)
    }
}

/// The club rejects the loan offer: state 2, the bids unmarked, the news, the offer comes back in 10 days to be
/// removed, the person's offer mark cleared with no open rival, the contract offers closed and the buyer's shortlist
/// status.
///
/// Ret 0xc: `TRANSFER_MANAGER::reject_loan_offer(slot, mark, reason)`, from the computer seller that refuses every loan
/// bid (0x8c8090, offer with +0x1f = 1) and from the scheduled offer (0x8c63a0). With `mark`, +0x2d = 1; state 2
/// (refused) straight away; the person's contract goes into `[0xae1080]` and gives the contract club; the buyer's bids
/// for the player and those of the contract club for the exchange players are removed (0x84aeb0); the news (0x8d5170);
/// the offer comes back in 10 days (0x8c6220) to be removed; with no open rival, the person's offer mark is removed
/// (+0x86e); the contract offers (0x4cca70 and, through slot +0x20, 0x4ccb30 for each exchange player, whose mark +4 is
/// zeroed), with +0x10 = 0 between them; finally the status in the buyer's scouting list (0x849ce0; found, 0x843e10
/// with event 1 and type 0). It is the transfer refusal (0x8bf680) without the unsettling, without the future transfer,
/// without closing the linked contract offers and without the exchange players' part at the end: the `cpx1_*` helpers
/// (with the dry run OFFER_RESPONSE_DRY_RUN) serve the same way.
/// Original: `TRANSFER_MANAGER::reject_loan_offer` (0x8bfa30, transfer_manager.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x8bfa30 } (thiscall, ret 0xc, transfer_manager.cpp): `TRANSFER_MANAGER::reject_loan_offer(vaga, marcar, motivo)`,
/// pelo vendedor do computador que recusa todo lance de empréstimo (0x8c8090, oferta com +0x1f = 1) e pela oferta
/// agendada (0x8c63a0). Com `marcar`, +0x2d = 1; estado 2 (recusada) já de cara; o contrato da pessoa vai para
/// [0xae1080] e dá o clube do contrato; os lances do comprador pelo jogador e os do clube do contrato pelos da troca
/// saem (0x84aeb0); a notícia (0x8d5170); a oferta volta em 10 dias (0x8c6220) para sair; sem rival em aberto, a marca
/// de oferta da pessoa sai (+0x86e); as ofertas de contrato (0x4cca70 e, pela vaga +0x20, a 0x4ccb30 de cada jogador
/// da troca, cuja marca +4 zera), com +0x10 = 0 entre elas; por fim a situação na lista de observação do comprador
/// (0x849ce0; achada, 0x843e10 com evento 1 e tipo 0). É a recusa da transferência (0x8bf680) sem o desassossego, sem
/// a transferência futura, sem fechar as ofertas de contrato ligadas e sem a parte dos jogadores da troca no fim: os
/// ajudantes `cpx1_*` (com o ensaio OFFER_RESPONSE_DRY_RUN) servem igual.
pub(crate) unsafe extern "thiscall" fn reject_loan_offer(this: usize, slot: *mut usize, mark: u32, reason: u32) {
    let o = *slot;
    if mark != 0 {
        *((o + offer::PLAYER_AWARE) as *mut u8) = 1;
    }
    *((o + offer::STATUS) as *mut u8) = 2;
    crate::comparison::branch("cqa_5 emprestimo recusado");
    let seller = contract_club_stored(staff_records_staff_by_id(read_usize(o + offer::TARGET_STAFF_ID)));
    clear_offer_and_exchange_markers(this, o, seller);
    transfer_manager_post_offer_rejected_news(this, o, reason);
    schedule_offer_unless_dry_run(this, o, 0xa);
    clear_offer_mark_if_closed(this, o);
    // seller 0: without the players’ share of the exchange in the buyer’s list, which this routine does not have
    withdraw_offer_contract_offers(this, o, 0, 0);
    let entry: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(FN_SHORTLIST_MANAGER_FIND_CLUB_SHORTLIST_ENTRY);
    let buyer = friendly_club_by_id(read_usize(o + offer::BUYER_CLUB_ID));
    let p = staff_records_staff_by_id(read_usize(o + offer::TARGET_STAFF_ID));
    if entry(shortlist_manager_of(this), buyer, p, (read_u8(o + offer::JOB_TYPE) != 0xb) as u32, 0) != 0 {
        update_shortlist_status(this, p, buyer, 1, 0);
    }
}

/// O ensaio do comparador da 0x8cd450: a cópia roda sem liberar o jogador escolhido (0x86d800 e 0x8c1af0 mexem no
/// resto do jogo) e guarda quem seria em SQUAD_LIMITS_RELEASED_PLAYER.
pub(crate) static mut SQUAD_LIMITS_DRY_RUN: bool = false;
pub(crate) static mut SQUAD_LIMITS_RELEASED_PLAYER: usize = 0;

/// Job type byte is a player job (DATABASE is_player_job 0x53b620, types 11..15).
///
/// [pt] Os tipos 0x53b620 (11 a 15: jogador) e 0x53b610 (não 11), cdecl com o byte.
pub(crate) unsafe fn transfer_manager_is_player_job(t: u8) -> bool {
    let f: unsafe extern "C" fn(i8) -> u32 = core::mem::transmute(FN_DATABASE_IS_PLAYER_JOB);
    f(t as i8) != 0
}

/// Job type byte is not type 11 (DATABASE 0x53b610).
pub(crate) unsafe fn is_not_plain_player_job(t: u8) -> bool {
    let f: unsafe extern "C" fn(i8) -> u32 = core::mem::transmute(FN_DATABASE_IS_NOT_PLAIN_PLAYER_JOB);
    f(t as i8) != 0
}

/// Job type -> index of the club's staff slot counter (0 coaches 8/0xf, 1 scouts 9, 2 physios 0xa, 3 assistant 6/0xd);
/// None otherwise.
///
/// [pt] As quatro contas de vagas da comissão pelo tipo da pessoa: [0] treinadores (8 e 0xf, +0x19f), [1] olheiros (9,
/// +0x1b3), [2] fisioterapeutas (0xa, +0x1cf), [3] auxiliar (6 e 0xd, +0xd3).
pub(crate) unsafe fn staff_role_slot_group(t: u8) -> Option<usize> {
    match t {
        8 | 0xf => Some(0),
        9 => Some(1),
        0xa => Some(2),
        6 | 0xd => Some(3),
        _ => None,
    }
}

/// Person is non-null and its id is below G_STAFF_COUNT (the person still exists).
///
/// [pt] O id da pessoa no vetor (dword com sinal) abaixo de [0xae2364]: a pessoa ainda existe.
pub(crate) unsafe fn staff_id_in_range(p: usize) -> bool {
    p != 0 && (read_usize(p) as i32) < read_usize(G_STAFF_COUNT) as i32
}

/// Writes the failure reason (base + 1 if the club is not the offer's buyer) to `motivo` if given; returns 0.
///
/// [pt] A falha com o motivo: (clube ≠ comprador) + `base` na palavra do motivo, se houver; 0.
pub(crate) unsafe fn fail_with_reason(club: usize, o: usize, reason: *mut u16, base: u16) -> u32 {
    if !reason.is_null() {
        *reason = (read_usize(club) != read_usize(o + offer::BUYER_CLUB_ID)) as u16 + base;
    }
    0
}

/// Person's id is in the free-transfer queue LLIST (G_TRANSFER_MANAGER + 0x92c), rewinding the cursor first.
///
/// [pt] A pessoa está na fila da transferência livre ([0xde1f54] + 0x92c, LLIST de ids; o cursor +0xa volta a +2 e a busca
/// para no achado, como na original).
pub(crate) unsafe fn in_free_transfer_queue(p: usize) -> bool {
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let l = read_usize(G_TRANSFER_MANAGER) + 0x92c;
    core::ptr::write_unaligned((l + llist::CURSOR) as *mut usize, read_usize(l + llist::HEAD));
    let mut e = next(l);
    while e != 0 {
        if read_usize(p) == read_usize(e) {
            return true;
        }
        e = next(l);
    }
    false
}

/// Checks that the offer keeps the club (with its partner) within 50 players and the coach (5), physio (3), scout (7)
/// and assistant slots; a computer buyer without an exchange releases for free the cheapest player who can leave;
/// otherwise reason 4 or 0x22 (plus 1 for the seller) and 0.
///
/// Ret 0x10; the map gives it as stdcall, but the two call sites in 0x8cfc90 put the manager in ecx and it uses it for
/// 0x8c1780 and 0x8c1af0: `TRANSFER_MANAGER::check_squad_limits(club, offer, count_queue, reason)`, from the offer
/// restrictions (0x8cfc90), for the buyer and for the seller. Null club: the box at line 0x2c3b and 0. The club's
/// partner (0x540a50 with 0 and 1). For the buyer (+8 of the offer = club id): the offer's person already belongs to it
/// (contract of type 5, again through 0x4cf5a0, with the club; or the loan record 0x4cf6d0 with +4 = the club) counts
/// −1; a player by the offer's type +0x11 counts +1; a staff type by the same byte marks 1; each exchange player (+4,
/// LLIST of ids) who leaves is discounted (player −1, staff −1 by the person's +0x3d). For the seller: without an
/// exchange, 1; the player sold counts −1 and the staff type −1 (by +0x3d); each incoming exchange node adds by byte
/// +6. A positive count: adds the 50 of the squad (+0xd7, pointers with a live id) of the club and of the partner;
/// above 50, minus the club's free queue (0x8c1780) if `count_queue`; still above 50: other than a computer buyer
/// without an exchange, the reason (club ≠ buyer) + 4 and 0; otherwise the cheapest of the squad ((+0x3d of the
/// contract + 100) × value +0x52, with +0x3d ≤ 0, without a sheet, without a type 5 contract in `[0xae1080]`, without
/// offers 0x8c5cf0, exchanges 0x8c5e40 or future transfer 0x8cca10, outside the free queue and not of type 0xb, 0xf or
/// 0xd) leaves for free (0x86d800 on `[0xdd7f00]`, 0x8c1af0 with −1); none, the reason + 4 and 0. Then the marked staff
/// slots: 5 coaches (+0x19f), 3 physios (+0x1cf), 7 scouts (+0x1b3), those of the club counted by pointer and those of
/// the partner only with a live id, and the assistant (+0xd3 of the club or of the partner, occupied); over the limit,
/// the reason + 0x22 and 0. Returns 1.
/// Original: `TRANSFER_MANAGER::check_squad_limits` (0x8cd450, transfer_manager.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x8cd450 } (thiscall, ret 0x10, transfer_manager.cpp; o mapa a dá como stdcall, mas os dois pontos de chamada na
/// 0x8cfc90 põem a gerência em ecx e ela o usa para a 0x8c1780 e a 0x8c1af0): `TRANSFER_MANAGER::check_squad_limits
/// (clube, oferta, contar_fila, motivo)`, das restrições da oferta (0x8cfc90), pelo comprador e pelo vendedor. Clube
/// nulo: a caixa da linha 0x2c3b e 0. O parceiro do clube (0x540a50 com 0 e 1). Pelo comprador (+8 da oferta = id do
/// clube): a pessoa da oferta já é dele (contrato de tipo 5, de novo pela 0x4cf5a0, com o clube; ou a ficha de
/// empréstimo 0x4cf6d0 com +4 = o clube) conta −1; jogador pelo tipo +0x11 da oferta conta +1; o tipo da comissão
/// pelo mesmo byte marca 1; cada jogador da troca (+4, LLIST de ids) que sai desconta (jogador −1, comissão −1 pelo
/// +0x3d da pessoa). Pelo vendedor: sem troca, 1; o jogador vendido conta −1 e o tipo da comissão −1 (pelo +0x3d); cada
/// nó da troca que chega soma pelo byte +6. Conta positiva: soma os 50 do elenco (+0xd7, ponteiros com id vivo) do clube
/// e do parceiro; acima de 50, menos a fila livre do clube (0x8c1780) se `contar_fila`; ainda acima de 50: fora do
/// comprador do computador sem troca, o motivo (clube ≠ comprador) + 4 e 0; senão o mais barato do elenco ((+0x3d do
/// contrato + 100) × valor +0x52, com +0x3d ≤ 0, sem ficha, sem contrato de tipo 5 em [0xae1080], sem ofertas 0x8c5cf0,
/// trocas 0x8c5e40 ou transferência futura 0x8cca10, fora da fila livre e de tipo 0xb, 0xf ou 0xd) sai de graça
/// (0x86d800 em [0xdd7f00], 0x8c1af0 com −1); nenhum, o motivo + 4 e 0. Depois as vagas da comissão marcadas: 5
/// treinadores (+0x19f), 3 fisioterapeutas (+0x1cf), 7 olheiros (+0x1b3) — os do clube contam por ponteiro, os do
/// parceiro só com id vivo — e o auxiliar (+0xd3 do clube ou do parceiro ocupado); passou do limite, o motivo + 0x22 e 0.
/// Devolve 1.
pub(crate) unsafe extern "thiscall" fn check_squad_limits(this: usize, club: usize, o: usize, count_queue: u32, reason: *mut u16) -> u32 {
    if club == 0 {
        show_error_at(0xa8d514, 0x2c3b);
        return 0;
    }
    let partner_of: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
    let sheet: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_LOAN_CONTRACT);
    let partner = partner_of(club, core::ptr::null_mut(), 1);
    let mut n: i8 = 0;
    let mut com = [0i8; 4];
    if read_usize(club) == read_usize(o + offer::BUYER_CLUB_ID) {
        let p = staff_records_staff_by_id(read_usize(o + offer::TARGET_STAFF_ID));
        let c = transfer_manager_get_contract_and_store(p);
        let mut own = false;
        if c != 0 && read_u8(c + contract::CONTRACT_TYPE) & 0x3f == 5 {
            own = contract_club_stored(p) == club;
        }
        if !own && sheet(read_usize(G_CONTRACT_MANAGER), p) != 0 && read_usize(sheet(read_usize(G_CONTRACT_MANAGER), p) + 4) == read_usize(club) {
            own = true;
        }
        if own {
            n = -1;
        }
        let t = read_u8(o + offer::JOB_TYPE);
        if transfer_manager_is_player_job(t) {
            n = n.wrapping_add(1); // cqa5: the offer player counts
        }
        if is_not_plain_player_job(t) {
            if let Some(k) = staff_role_slot_group(t) {
                com[k] = 1;
            }
        }
        let l = read_usize(o + offer::EXCHANGE_PLAYERS);
        if l != 0 {
            for e in llist_items_from_start(l) {
                let t = read_u8(staff_records_staff_by_id(read_usize(e)) + 0x3d);
                if transfer_manager_is_player_job(t) {
                    n = n.wrapping_sub(1);
                }
                if is_not_plain_player_job(t) {
                    if let Some(k) = staff_role_slot_group(t) {
                        com[k] = com[k].wrapping_sub(1);
                    }
                }
            }
        }
    } else {
        if read_usize(o + offer::EXCHANGE_PLAYERS) == 0 {
            crate::comparison::branch("cqa_5 vendedor sem troca");
            return 1; // cqa5: the seller without an exchange has a slot
        }
        let t = read_u8(staff_records_staff_by_id(read_usize(o + offer::TARGET_STAFF_ID)) + 0x3d);
        if transfer_manager_is_player_job(t) {
            n = -1;
        }
        if is_not_plain_player_job(t) {
            if let Some(k) = staff_role_slot_group(t) {
                com[k] = -1;
            }
        }
        for e in llist_items_from_start(read_usize(o + offer::EXCHANGE_PLAYERS)) {
            let t = read_u8(e + 6);
            if transfer_manager_is_player_job(t) {
                n = n.wrapping_add(1);
            }
            if is_not_plain_player_job(t) {
                if let Some(k) = staff_role_slot_group(t) {
                    com[k] = com[k].wrapping_add(1);
                }
            }
        }
    }
    if n > 0 {
        let count = |base: usize, n: &mut i8| {
            for k in 0..0x32 {
                if staff_id_in_range(read_usize(base + 0xd7 + 4 * k)) {
                    *n = n.wrapping_add(1); // cqa5: cada um do elenco conta
                }
            }
        };
        crate::comparison::branch("cqa_5 conta o elenco");
        count(club, &mut n);
        if partner != 0 {
            count(partner, &mut n);
        }
        if n > 0x32 { // cqa5: the limit of 50 in the squad
            if count_queue != 0 {
                let queue: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_TRANSFER_MANAGER_COUNT_CLUB_FREE_TRANSFER_QUEUE);
                n = n.wrapping_sub(queue(this, club) as i8);
            }
            if n > 0x32 {
                let human: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
                if read_usize(club) != read_usize(o + offer::BUYER_CLUB_ID) || human(club, 1, 1, 0, 0) != 0 || read_usize(o + offer::EXCHANGE_PLAYERS) != 0 {
                    crate::comparison::branch("cqa_5 elenco cheio");
                    return fail_with_reason(club, o, reason, 4);
                }
                let contract: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
                let offers: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_OFFERS_FOR_PLAYER);
                let swaps: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_EXCHANGE_OFFERS_FOR_PLAYER);
                let future: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_FUTURE_TRANSFER);
                let mut best = 1000000.0f64;
                let mut chosen = 0usize;
                for k in 0..0x32 {
                    let p = read_usize(club + club::SQUAD + 4 * k);
                    if !staff_id_in_range(p) || sheet(read_usize(G_CONTRACT_MANAGER), p) != 0 {
                        continue;
                    }
                    // the first contract query does not go to [0xae1080]
                    let c = contract(read_usize(G_CONTRACT_MANAGER), p);
                    if c == 0 {
                        continue;
                    }
                    let s = read_u8(c + 0x3d) as i8;
                    let v = (s as f64 + 100.0) * (read_usize(p + staff::VALUE) as i32 as f64);
                    if !(v < best) || s > 0 {
                        continue;
                    }
                    let c = transfer_manager_get_contract_and_store(p);
                    if c != 0 && read_u8(c + contract::CONTRACT_TYPE) & 0x3f == 5 {
                        continue;
                    }
                    if sheet(read_usize(G_CONTRACT_MANAGER), p) != 0 {
                        continue;
                    }
                    let g = read_usize(G_TRANSFER_MANAGER);
                    if offers(g, p, 0, 0) != 0 || swaps(g, p, 0) != 0 || future(g, p, 0) != 0 {
                        continue;
                    }
                    if in_free_transfer_queue(p) {
                        continue;
                    }
                    if matches!(read_u8(p + staff::JOB_FOR_CLUB), 0xb | 0xf | 0xd) {
                        chosen = p;
                        best = v;
                    }
                }
                if chosen == 0 {
                    return fail_with_reason(club, o, reason, 4);
                }
                crate::comparison::branch("cqa_5 libera o mais barato");
                if SQUAD_LIMITS_DRY_RUN {
                    SQUAD_LIMITS_RELEASED_PLAYER = chosen;
                } else {
                    let register: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_STAFF_RECORDS_ADD_FREE_RELEASE_RECORDS);
                    let free: unsafe extern "thiscall" fn(usize, usize, usize, u32, i32, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_FREE_TRANSFER_STAFF);
                    register(read_usize(G_STAFF_RECORDS), chosen);
                    free(this, chosen, club, 0, -1, 0);
                }
            }
        }
    }
    // the staff slots: (count, offset, slots)
    for (k, base, vagas) in [(0usize, 0x19fusize, 5usize), (2, 0x1cf, 3), (1, 0x1b3, 7)] {
        let mut m = com[k];
        if m <= 0 {
            continue;
        }
        for j in 0..vagas {
            if read_usize(club + base + 4 * j) != 0 {
                m = m.wrapping_add(1);
            }
        }
        if partner != 0 {
            for j in 0..vagas {
                if staff_id_in_range(read_usize(partner + base + 4 * j)) {
                    m = m.wrapping_add(1);
                }
            }
        }
        if m > vagas as i8 {
            return fail_with_reason(club, o, reason, 0x22);
        }
    }
    if com[3] > 0 && (read_usize(club + club::ASSISTANT_MANAGER) != 0 || (partner != 0 && read_usize(partner + club::ASSISTANT_MANAGER) != 0)) {
        return fail_with_reason(club, o, reason, 0x22);
    }
    1
}

/// With the person's flag 8, binary-searches the person's next-sale percentage clause with the contract club; when not
/// found, clears the flag.
///
/// Ret 4: `TRANSFER_MANAGER::find_next_sale_percentage(person)`, from the person screens (0x884ea2): without the
/// person's flag 8 (`[this+0x86e]` + id × 8), 0. Her contract (0x4cf5a0, in `[0xae1080]`) and the club; without both,
/// 0. The binary search (0x946464: cdecl(key, vector `[this+0x880]`, count `[this+0x888]`, 0xd, 0x8e54d0)) by the key
/// with the club id (+1, again through the contract) and the person's (+9), the only two fields the comparison reads;
/// found, returns the record; otherwise flag 8 is cleared and 0.
/// Original: `TRANSFER_MANAGER::find_next_sale_percentage` (0x8d0dd0, transfer_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x8d0dd0 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::find_next_sale_percentage(pessoa)`, das
/// telas da pessoa (0x884ea2): sem a marca 8 da pessoa ([this+0x86e] + id × 8), 0. O contrato dela (0x4cf5a0, em
/// [0xae1080]) e o clube; sem os dois, 0. A busca binária (0x946464: cdecl(chave, vetor [this+0x880], conta
/// [this+0x888], 0xd, 0x8e54d0)) pela chave com o id do clube (+1, de novo pelo contrato) e o da pessoa (+9), os dois
/// únicos campos que a comparação lê; achada, devolve o registro; senão a marca 8 sai e 0.
pub(crate) unsafe extern "thiscall" fn find_next_sale_percentage(this: usize, p: usize) -> usize {
    let m = (read_usize(this + transfer_manager_fields::PERSON_FLAGS).wrapping_add(read_usize(p).wrapping_mul(8))) as *mut u8;
    if *m & 8 == 0 {
        return 0;
    }
    if contract_club_stored(p) == 0 {
        return 0;
    }
    let mut key = [0u8; 0x10];
    key[9..13].copy_from_slice(&(read_usize(p) as u32).to_le_bytes());
    // simplification: the original reads the club id without checking the contract (it has just found it); without it, here 0
    let c = contract_club_stored(p);
    let id = if c != 0 { read_usize(c) as u32 } else { 0 };
    key[1..5].copy_from_slice(&id.to_le_bytes());
    let bsearch: unsafe extern "C" fn(*const u8, usize, usize, u32, usize) -> usize = core::mem::transmute(FN_CRT_BSEARCH);
    let r = bsearch(key.as_ptr(), read_usize(this + transfer_manager_fields::NEXT_SALE_PERCENTAGES), read_usize(this + transfer_manager_fields::NEXT_SALE_PERCENTAGE_COUNT), 0xd, 0x8e54d0);
    if r != 0 {
        return r;
    }
    *m = (*m).wrapping_sub(8);
    0
}

/// Address of the person's 8-byte flags record in the transfer manager ([this+0x86e] + id x 8).
///
/// [pt] O byte das marcas da pessoa no registro de 8 bytes da gerência de transferências ([this+0x86e] + id × 8).
pub(crate) unsafe fn staff_flags_record(this: usize, p: usize) -> usize {
    read_usize(this + transfer_manager_fields::PERSON_FLAGS).wrapping_add(read_usize(p).wrapping_mul(8))
}

/// Person's work-permit record (0xe bytes, sorted at [this+0x93a], count +0x93e) by bsearch; 0 if id out of range or
/// flag 0x20 not set.
///
/// [pt] A permissão de trabalho da pessoa (0xe bytes, [this+0x93a], conta +0x93e), como a original procura em linha: id fora
/// dos [0xae2364] (com sinal) ou sem a marca 0x20, 0; senão a busca binária (bsearch 0x946464 com 0x8e55c0).
pub(crate) unsafe fn transfer_manager_find_work_permit(this: usize, p: usize) -> usize {
    if p != 0 && read_usize(p) as i32 >= read_usize(G_STAFF_COUNT) as i32 {
        return 0;
    }
    if read_u8(staff_flags_record(this, p)) & 0x20 == 0 {
        return 0;
    }
    let bsearch: unsafe extern "C" fn(*const u32, usize, usize, u32, usize) -> usize = core::mem::transmute(FN_CRT_BSEARCH);
    let key = [read_usize(p) as u32];
    bsearch(key.as_ptr(), read_usize(this + transfer_manager_fields::WORK_PERMITS), read_usize(this + transfer_manager_fields::WORK_PERMIT_COUNT), 0xe, 0x8e55c0)
}

/// Adds 1 to the u16 at offset `o` of a work-permit record (+4 games, +6 misses); nothing if the record is null.
///
/// [pt] Soma 1 à palavra `o` da permissão (+4 jogos, +6 faltas). A original soma sem conferir a busca (a marca garante a
/// permissão); sem ela, a original cairia: aqui nada.
pub(crate) unsafe fn increment_permit_counter(r: usize, o: usize) {
    if r != 0 {
        let a = (r + o) as *mut u16;
        core::ptr::write_unaligned(a, core::ptr::read_unaligned(a).wrapping_add(1));
    }
}

/// The comparer's dry run: the copy runs without paying the appearance bonuses (0x8cb340 sends a news item, allocates
/// and pays), as if none paid (without removing the bonus); the permission count does not depend on them.
///
/// [pt] O ensaio do comparador: a cópia roda sem pagar os prêmios por jogos (0x8cb340 dá notícia, aloca e paga), como se
/// nenhum pagasse (sem tirar o bônus); a contagem das permissões não depende deles.
pub(crate) static mut APPEARANCE_CLAUSES_DRY_RUN: bool = false;

/// TRANSFER_MANAGER::pay_appearance_money(person, games, type); returns 0 without calling during the comparer's dry
/// run.
///
/// [pt] TRANSFER_MANAGER::pay_appearance_money (0x8cb340: thiscall(this, pessoa, jogos, tipo, 0), ret 0x10); no ensaio, 0.
pub(crate) unsafe fn pay_appearance_money_unless_dry_run(this: usize, p: usize, games: i32, kind: u32) -> u32 {
    if APPEARANCE_CLAUSES_DRY_RUN {
        return 0;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, i32, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_PAY_APPEARANCE_MONEY);
    f(this, p, games, kind, 0)
}

/// After the match, on both sides: pays the appearance money at 1, 10, 20... 50 games (club by contract, national side
/// by caps since the deal) and removes the paid bonus; in a counting match, adds an appearance to the work permit of
/// those who played and an absence to those left out without injury, ban, call-up or loan.
///
/// Ret 8: `TRANSFER_MANAGER::update_appearance_clauses_after_match(match, fixture)`, from the end of each match
/// (0x6b5f00 and the simulated statistics 0x84be20). For both sides (+0x1c home, +0x20 away): the match counts for work
/// permits with a competition (+0x14) that is not a national side's (home side through 0x53b900) nor a friendly
/// (0x5c6710) and the side's byte (+0x40/+0x41) set, or with +0 = −2. A club that is not a national side: the squad
/// (0x541f90, 50 slots; national side: the vector stays as it was) and, when counting, its players with flag 0x20 (the
/// 8-byte record at +0x86e). In the 20 match slots (+0x190 the id, +0x1ad ≠ −5, substitutes with +0x1c2 ≠ −1; without a
/// player sheet +0x61, end of the slots): outside a loan (0x4cf6d0, only at the club) and with the clause flag (2 club,
/// 4 national side), the matches (national side: +0x22 minus byte +7 of the record; club: +0x3b of the contract
/// 0x4cf5a0) that are multiples of 10 up to 50, or 1, pay the bonus (0x8cb340 with type 5 or 2) and, once paid, the
/// bonus is removed (0x8cb630 or 0x8cb590). When counting, the marked player adds a match (+4 of the permit) and leaves
/// the list; those remaining, without injury (0x62c400), suspension (0x55d270), call-up (0x7757d0) or ineligibility
/// (0x8cf5e0), and not loaned by another club (0x4cf6d0 with the contract stored in `[0xae1080]`), add an absence (+6).
/// Original: `update_appearance_clauses_after_match` (0x8c39f0, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8c39f0 } (thiscall, ret 8, transfer_manager.cpp): `TRANSFER_MANAGER::update_appearance_clauses_after_match(jogo,
/// partida)`, pelo fim de cada partida (0x6b5f00 e as estatísticas simuladas 0x84be20). Para os dois lados (+0x1c casa,
/// +0x20 fora): o jogo conta para as permissões de trabalho com competição (+0x14) que não é de seleção (casa pela
/// 0x53b900) nem amistoso (0x5c6710) e o byte do lado (+0x40/+0x41) ligado, ou com +0 = −2. Clube que não é seleção: o
/// elenco (0x541f90, 50 vagas; seleção: o vetor fica como estava) e, contando, os jogadores dele com a marca 0x20 (o
/// registro de 8 bytes em +0x86e). Nas 20 vagas da partida (+0x190 o id, +0x1ad ≠ −5, reservas com +0x1c2 ≠ −1; sem
/// ficha de jogador +0x61, fim das vagas): fora do empréstimo (0x4cf6d0, só no clube) e com a marca da cláusula (2
/// clube, 4 seleção), os jogos (seleção: +0x22 menos o byte +7 do registro; clube: +0x3b do contrato 0x4cf5a0) múltiplos
/// de 10 até 50, ou 1, pagam o prêmio (0x8cb340 com o tipo 5 ou 2) e, pago, o bônus sai (0x8cb630 ou 0x8cb590). Contando,
/// o jogador marcado soma um jogo (+4 da permissão) e sai da lista; os que sobram, sem lesão (0x62c400), suspensão
/// (0x55d270), convocação (0x7757d0) nem inelegibilidade (0x8cf5e0), e não emprestados por outro clube (0x4cf6d0 com o
/// contrato guardado em [0xae1080]), somam uma falta (+6).
pub(crate) unsafe extern "thiscall" fn update_appearance_clauses_after_match(this: usize, game: usize, partida: usize) {
    let is_national_team: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let friendly: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_FRIENDLY_IS_FRIENDLY_COMPETITION);
    let reunir: unsafe extern "C" fn(usize, *mut usize, u32) -> u32 = core::mem::transmute(FN_DATABASE_COLLECT_CLUB_SQUAD);
    let sheet: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_LOAN_CONTRACT);
    let contract: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let remove_national_team: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_REMOVE_INTERNATIONAL_TEAM_APPEARANCE_MONEY);
    let remove_team: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_REMOVE_FIRST_TEAM_APPEARANCE_MONEY);
    let injury: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_INJURY_GET_PLAYER_INJURY_STATUS);
    let suspended: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_DISCIPLINE_IS_PLAYER_BANNED);
    let convocado: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize) -> usize = core::mem::transmute(FN_NATIONALTEAMS_GET_INTERNATIONAL_DUTY_CLUB);
    let inelegivel: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_INELIGIBILITY_CODE);
    // simplification: the original’s local squad (esp+0x110) is not filled on the national team side and keeps whatever the stack had
    // (the one from the other side or garbage); here it starts zeroed (empty slot, which the game also produces) and passes from one side to the other
    let mut squad = [0usize; 50];
    let mut side: i8 = 0;
    while side < 2 {
        let club = read_usize(game + if side == 0 { 0x1c } else { 0x20 });
        let comp = read_usize(game + fixture::COMPETITION);
        let away = comp == 0 || is_national_team(read_usize(game + fixture::HOME_CLUB)) != 0 || friendly(read_usize(G_FRIENDLY), comp) != 0;
        let contando = (!away && read_u8(game + fixture::HOME_SIDE_MARK + side as usize) != 0) || read_usize(game) as i32 == -2;
        let mask: u8 = if is_national_team(club) != 0 {
            4
        } else {
            reunir(club, squad.as_mut_ptr(), 0);
            2
        };
        let mut seguidos = [0usize; 50];
        let mut n: i8 = 0;
        if contando {
            for &p in squad.iter() {
                if p != 0 && (read_usize(p) as i32) < read_usize(G_STAFF_COUNT) as i32 && read_u8(staff_flags_record(this, p)) & 0x20 != 0 {
                    seguidos[n as usize] = p;
                    n += 1;
                }
            }
        }
        let base = partida + side as usize * 0x625;
        let mut k: i8 = 0;
        while k < 0x14 {
            let v = base + k as usize * 0x49;
            let id = read_usize(v + 0x190) as i32;
            if id != -1 && read_u8(v + 0x1ad) != 0xfb && (k < 0xb || read_u8(v + 0x1c2) != 0xff) {
                let p = staff_records_staff_by_id(id as usize);
                if read_usize(p + staff::PLAYER) == 0 {
                    break;
                }
                if (p == 0 || (read_usize(p) as i32) < read_usize(G_STAFF_COUNT) as i32) && !(mask == 2 && sheet(read_usize(G_CONTRACT_MANAGER), p) != 0) {
                    let reg = staff_flags_record(this, p);
                    if read_u8(reg) & mask != 0 {
                        let (jogos, tipo): (i16, u8) = if mask == 4 {
                            ((read_u8(p + staff::INT_APPS) as u16).wrapping_sub(read_u8(reg + person_transfer_record::INTERNATIONAL_APPS_AT_DEAL) as u16) as i16, 5)
                        } else {
                            let mut j = 0i16;
                            if contract(read_usize(G_CONTRACT_MANAGER), p) != 0 {
                                j = read_i16(contract(read_usize(G_CONTRACT_MANAGER), p) + 0x3b);
                            }
                            (j, 2)
                        };
                        // simplification: the original pushes the dword of the type local, with the 3 high bytes of stack garbage;
                        // 0x8cb340 and the news item only read the byte: passed clean
                        if (jogos as i32 % 10 == 0 && jogos <= 0x32) || jogos == 1 { // cpy11: the bonus matches
                            if pay_appearance_money_unless_dry_run(this, p, jogos as i32, tipo as u32) != 0 {
                                crate::comparison::branch("cpy_11 premio pago");
                                if tipo == 5 {
                                    remove_national_team(this, p);
                                } else {
                                    remove_team(this, p);
                                }
                            }
                        }
                    }
                    if contando && read_u8(staff_flags_record(this, p)) & 0x20 != 0 {
                        let r = transfer_manager_find_work_permit(this, p);
                        crate::comparison::branch("cpy_11 jogo contado");
                        increment_permit_counter(r, 4); // cpy11: the match counts toward the permit
                        let mut i: i8 = 0;
                        while i < n {
                            if seguidos[i as usize] == p {
                                seguidos[i as usize] = 0;
                                break;
                            }
                            i += 1;
                        }
                    }
                }
            }
            k += 1;
        }
        if contando && n > 0 {
            for i in 0..n as usize {
                let p = seguidos[i];
                if p == 0 || injury(read_usize(G_INJURY), p) != 0 {
                    continue;
                }
                let comp = read_usize(game + fixture::COMPETITION);
                if comp != 0 && suspended(read_usize(G_DISCIPLINE), p, comp, 1, 0) != 0 {
                    continue;
                }
                if convocado(read_usize(G_NATIONAL_TEAMS), p, G_CURRENT_DATE, club, 0, 0) != 0 {
                    continue;
                }
                if read_usize(game + fixture::COMPETITION) != 0 && inelegivel(this, p, game) != 0 {
                    continue;
                }
                if sheet(read_usize(G_CONTRACT_MANAGER), p) != 0 {
                    let c = contract(read_usize(G_CONTRACT_MANAGER), p);
                    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
                    let owner = if c == 0 { 0 } else { friendly_club_by_id(read_usize(c + contract::CLUB)) };
                    if owner == club {
                        continue;
                    }
                }
                let r = transfer_manager_find_work_permit(this, p);
                crate::comparison::branch("cpy_11 falta contada");
                increment_permit_counter(r, 6); // cpy11: the foul counts toward the permit
            }
        }
        side += 1;
    }
}

/// Appends the international appearance money (13 bytes) to the array, growing by 50, and sorts it by person.
///
/// Ret 4: `TRANSFER_MANAGER::add_international_team_appearance_money(record)`: appends the national side appearance
/// bonus (13 bytes copied from `record`) to vector `[this+0x898]` (capacity +0x89c, count +0x8a0); full, grows by 50 at
/// a time (out of memory: box 0x25af and nothing). Sorts by person (qsort 0x945333 with 0x8b4e00). The twin for the
/// senior team is 0x8cac40.
/// Original: `add_international_team_appearance_money` (0x8cad90, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8cad90 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::add_international_team_appearance_money(
/// registro)`: acrescenta o prêmio por jogos na seleção (13 bytes copiados de `registro`) ao vetor [this+0x898]
/// (capacidade +0x89c, conta +0x8a0); cheio, cresce de 50 em 50 (sem memória: caixa 0x25af e nada). Reordena pela pessoa
/// (qsort 0x945333 com 0x8b4e00). A gêmea do time principal é a 0x8cac40.
pub(crate) unsafe extern "thiscall" fn add_international_team_appearance_money(this: usize, record: usize) {
    crate::comparison::branch("cpy_11 premio na selecao guardado");
    if !grow_record_vector(this, 0x898, 4, 50, 0xd, 0x25af, 0xa8ebe0) {
        return;
    }
    let n = read_usize(this + transfer_manager_fields::INTERNATIONAL_APPEARANCE_MONEY_COUNT);
    core::ptr::copy_nonoverlapping(record as *const u8, read_usize(this + transfer_manager_fields::INTERNATIONAL_APPEARANCE_MONEY).wrapping_add(n.wrapping_mul(0xd)) as *mut u8, 0xd);
    core::ptr::write_unaligned((this + transfer_manager_fields::INTERNATIONAL_APPEARANCE_MONEY_COUNT) as *mut usize, n.wrapping_add(1));
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    qsort(read_usize(this + transfer_manager_fields::INTERNATIONAL_APPEARANCE_MONEY), read_usize(this + transfer_manager_fields::INTERNATIONAL_APPEARANCE_MONEY_COUNT) as u32, 0xd, 0x8b4e00);
}

/// Sends the 0xfa5 news of the agreed transfer, with the completion date, to the human seller and buyer.
///
/// Ret 4; SEH frame for the local date and news item: `TRANSFER_MANAGER::post_transfer_finalised_news(offer)`, from the
/// agreed transfer (0x8c2f50). The seller: the club of the person's contract (0x4cf5a0, stored in `[0xae1080]`) except
/// type 5 (requested again, without storing). News 0xfa5: the person (the id and the three keys), the buyer, the seller
/// (or −1), +0x20, +0x24, the fee +0x1b, the state +0x2c, the total (0x8e7a70 with 0, 1, 0, 1), +0x2f, +0x11; the
/// completion date (0x8cee70) after today goes in fields 0xe (day), 0xf (year) and 0xd (the low nibble of +0x30,
/// signed), otherwise 0, 0, 0; the exchange count (+4) in 0xb. To the human seller (0x7875e0), outside state 0xc and
/// without a pending exchange (+0x2f with nibble +0x30), and to the human buyer, each in field 0x10.
/// Original: `post_transfer_finalised_news` (0x8da5e0, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8da5e0 } (thiscall, ret 4, transfer_manager.cpp, quadro SEH da data e da notícia locais): `TRANSFER_MANAGER::
/// post_transfer_finalised_news(oferta)`, pela transferência acertada (0x8c2f50). O vendedor: o clube do contrato da
/// pessoa (0x4cf5a0, guardado em [0xae1080]) fora do tipo 5 (pedido de novo, sem guardar). A notícia 0xfa5: a pessoa (o
/// id e as três chaves), o comprador, o vendedor (ou −1), +0x20, +0x24, o valor +0x1b, o estado +0x2c, o total
/// (0x8e7a70 com 0, 1, 0, 1), +0x2f, +0x11; a data de conclusão (0x8cee70) depois de hoje vai nos campos 0xe (dia), 0xf
/// (ano) e 0xd (o nibble baixo de +0x30, com sinal), senão 0, 0, 0; a conta da troca (+4) no 0xb. Ao vendedor humano
/// (0x7875e0), fora do estado 0xc e sem troca pendente (+0x2f com nibble +0x30), e ao comprador humano, cada um no
/// campo 0x10.
pub(crate) unsafe extern "thiscall" fn post_transfer_finalised_news(this: usize, o: usize) {
    let contract: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let data_default: unsafe extern "thiscall" fn(*mut u8) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    let construct: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let write: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let total: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_OFFER_GET_TOTAL_VALUE);
    let conclusao: unsafe extern "thiscall" fn(usize, usize, *mut u8) -> u16 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_OFFER_COMPLETION_DATE);
    // the original puts the newspaper in ecx; 0x7875e0 is stdcall (ret 4) and does not read it
    let human: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_NEWS_CLUB_HAS_HUMAN_MANAGER);
    let deliver: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    let p = staff_records_staff_by_id(read_usize(o + offer::TARGET_STAFF_ID));
    let c = contract(read_usize(G_CONTRACT_MANAGER), p);
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
    let mut seller = if c == 0 || read_u8(c + contract::CONTRACT_TYPE) & 0x3f == 5 { 0 } else { friendly_club_by_id(read_usize(c + contract::CLUB)) };
    let buyer = friendly_club_by_id(read_usize(o + offer::BUYER_CLUB_ID));
    let mut data = [0u8; 8];
    data_default(data.as_mut_ptr());
    if seller != 0 && read_u8(contract(read_usize(G_CONTRACT_MANAGER), p) + 0x35) & 0x3f == 5 {
        seller = 0;
    }
    crate::comparison::branch("cpy_11 noticia da transferencia finalizada");
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    construct(nt, 0xfa5, 0);
    write(nt, 0, read_usize(p) as u32);
    write(nt, 1, read_usize(read_usize(p + staff::FIRST_NAME) + 0x33) as u32);
    write(nt, 2, read_usize(read_usize(p + staff::SECOND_NAME) + 0x33) as u32);
    write(nt, 3, read_usize(read_usize(p + staff::COMMON_NAME) + 0x33) as u32);
    write(nt, 4, read_usize(o + offer::BUYER_CLUB_ID) as u32);
    write(nt, 5, if seller != 0 { read_usize(seller) as u32 } else { u32::MAX });
    write(nt, 6, read_usize(o + offer::SLOT_INDEX) as u32);
    write(nt, 7, read_usize(o + offer::SERIAL) as u32);
    write(nt, 8, read_usize(o + offer::FEE) as u32);
    write(nt, 9, read_u8(o + offer::STATUS) as i8 as u32);
    write(nt, 0xa, total(o, 0, 1, 0, 1));
    write(nt, 0xc, read_u8(o + offer::PRE_CONTRACT) as i8 as u32);
    write(nt, 0x11, read_u8(o + offer::JOB_TYPE) as i8 as u32);
    conclusao(this, o, data.as_mut_ptr());
    let (dia, ano) = (i16::from_le_bytes([data[0], data[1]]), u16::from_le_bytes([data[2], data[3]]));
    let (hoje_dia, hoje_ano) = (read_i16(G_CURRENT_DATE), read_i16(G_CURRENT_DATE_YEAR) as u16);
    if hoje_ano < ano || (hoje_ano == ano && hoje_dia < dia) {
        write(nt, 0xe, dia as i32 as u32);
        write(nt, 0xf, ano as u32);
        write(nt, 0xd, (((read_u8(o + offer::COMPLETION_TIMING) << 4) as i8) >> 4) as i32 as u32);
    } else {
        write(nt, 0xe, 0);
        write(nt, 0xf, 0);
        write(nt, 0xd, 0);
    }
    let swap = read_usize(o + offer::EXCHANGE_PLAYERS);
    write(nt, 0xb, if swap != 0 { read_u8(swap) as i8 as u32 } else { 0 });
    let jornal = read_usize(G_NEWS);
    if seller != 0 && human(jornal, seller) != 0 && read_u8(o + offer::STATUS) != 0xc && (read_u8(o + offer::PRE_CONTRACT) == 0 || read_u8(o + offer::COMPLETION_TIMING) & 0xf == 0) {
        write(nt, 0x10, read_usize(seller) as u32);
        deliver(read_usize(G_NEWS), nt, seller);
    }
    if human(read_usize(G_NEWS), buyer) != 0 {
        write(nt, 0x10, read_usize(buyer) as u32);
        deliver(read_usize(G_NEWS), nt, buyer);
    }
}

// simplification: the news item goes to the managers’ inboxes (allocates): original only; the copy takes the branch.

/// Person -> main contract (get_contract), without writing G_CONTRACT_LOOKUP_RESULT.
///
/// [pt] O contrato principal da pessoa (0x4cf5a0 em [0xae1050]), sem guardar.
pub(crate) unsafe fn get_contract_unstored(p: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    f(read_usize(G_CONTRACT_MANAGER), p)
}

/// Copies today's date (0xae2c90) into `destino` (Date::operator=).
///
/// [pt] Date::operator= (0x417c10: thiscall(destino, resultado, origem), ret 8): a data de hoje ([0xae2c90]) no destino.
pub(crate) unsafe fn copy_today_to(target: usize) {
    let f: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let mut result = [0u8; 8];
    f(target, result.as_mut_ptr(), G_CURRENT_DATE);
}

/// Records the free transfer and, unless record only, pays the club's compensation, removes the person from shortlists,
/// selections, squad, offers and contract, clauses and work permit.
///
/// Ret 0x14: `TRANSFER_MANAGER::free_transfer_staff(person, club, reason, compensation, record_only)`, from the free
/// transfer queue (0x8c18c0), from the end of contracts (0x4c7ac0) and from 0x8cd450. A person on loan (0x4cf6d0):
/// nothing. The past transfer (0x25 bytes in vector `[this+0x862]`, capacity +0x866, count +0x86a; full, grows by 100
/// at a time; out of memory: box 0xce9 and nothing): +0x10 −1, +0x14 the club or −1, today's date in +0x1d, the person
/// (the id and the three keys), +0x1c 5 (record only) or 3, +0x18 0. Record only: done. Otherwise: with a player sheet
/// (+0x61), the history (0x7c5fe0 on `[0xb74350]` with the reason); with a club, the compensation (−1: with a type 2
/// contract, the days until the end (0x85fa00) × the wage (+0xc) / 7; otherwise 0) paid to the club (0x599920), the
/// status in the scouting lists (0x843e10, event 4, type 2) and, at the current club (+0x39), the two line-ups
/// (0x896c60 on `[0xde1d08]`) and the squad (0x53c190), with date +0x3e set to today; the contract offers of the
/// contract club (0x4cca70, contract stored in `[0xae1080]`), the main contract (0x4d0b50), +0x52 and +0x6d zeroed, the
/// three clauses (0x8cb4f0, 0x8cb590, 0x8cb630), the work permit (0x8c7e20 without a club) and 0x4196a0 on
/// `[0xad9c6c]`.
/// Original: `free_transfer_staff` (0x8c1af0, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8c1af0 } (thiscall, ret 0x14, transfer_manager.cpp): `TRANSFER_MANAGER::free_transfer_staff(pessoa, clube, motivo,
/// compensação, só_registro)`, pela fila das transferências livres (0x8c18c0), pelo fim dos contratos (0x4c7ac0) e pela
/// 0x8cd450. Pessoa emprestada (0x4cf6d0): nada. A transferência passada (0x25 bytes no vetor [this+0x862], capacidade
/// +0x866, conta +0x86a; cheio, cresce de 100 em 100; sem memória: a caixa 0xce9 e nada): +0x10 −1, +0x14 o clube ou −1,
/// a data de hoje em +0x1d, a pessoa (o id e as três chaves), +0x1c 5 (só registro) ou 3, +0x18 0. Só registro: acabou.
/// Senão: com ficha (+0x61), o histórico (0x7c5fe0 em [0xb74350] com o motivo); com clube, a compensação (−1: com
/// contrato do tipo 2, os dias até o fim (0x85fa00) × o salário (+0xc) / 7; senão 0) paga ao clube (0x599920), a situação
/// nas listas de observação (0x843e10, evento 4, tipo 2) e, no clube atual (+0x39), as duas escalações (0x896c60 em
/// [0xde1d08]) e o elenco (0x53c190), com a data +0x3e de hoje; as ofertas de contrato do clube do contrato (0x4cca70,
/// contrato guardado em [0xae1080]), o contrato principal (0x4d0b50), +0x52 e +0x6d zerados, as três cláusulas (0x8cb4f0,
/// 0x8cb590, 0x8cb630), a permissão de trabalho (0x8c7e20 sem clube) e 0x4196a0 em [0xad9c6c].
pub(crate) unsafe extern "thiscall" fn free_transfer_staff(this: usize, p: usize, club: usize, reason: u32, compensacao: i32, so_record: u32) {
    let sheet: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_LOAN_CONTRACT);
    if sheet(read_usize(G_CONTRACT_MANAGER), p) != 0 {
        return;
    }
    if !grow_record_vector(this, 0x862, 4, 100, 0x25, 0xce9, 0xa8e998) {
        return;
    }
    let n = read_usize(this + transfer_manager_fields::PAST_TRANSFER_COUNT);
    let r = read_usize(this + transfer_manager_fields::PAST_TRANSFERS).wrapping_add(n.wrapping_mul(0x25));
    core::ptr::write_unaligned((this + transfer_manager_fields::PAST_TRANSFER_COUNT) as *mut usize, n.wrapping_add(1));
    let por = |o: usize, v: u32| core::ptr::write_unaligned((r + o) as *mut u32, v);
    por(0x10, u32::MAX);
    por(0x14, if club != 0 { read_usize(club) as u32 } else { u32::MAX });
    copy_today_to(r + past_transfer::DATE);
    por(0, read_usize(p) as u32);
    por(4, read_usize(read_usize(p + staff::FIRST_NAME) + 0x33) as u32);
    por(8, read_usize(read_usize(p + staff::SECOND_NAME) + 0x33) as u32);
    por(0xc, read_usize(read_usize(p + staff::COMMON_NAME) + 0x33) as u32);
    *((r + past_transfer::MOVE_TYPE) as *mut u8) = if so_record != 0 { 5 } else { 3 };
    por(0x18, 0);
    if so_record != 0 {
        return;
    }
    crate::comparison::branch("cpy_13 liberado");
    if read_usize(p + staff::PLAYER) != 0 {
        let history: unsafe extern "thiscall" fn(usize, usize, u32, u32, i32, u32, u32) = core::mem::transmute(FN_PLAYERSTATS_ARCHIVE_PLAYER_STATS);
        history(read_usize(G_PLAYER_STATS), p, 0, 0, -1, reason, 0);
    }
    if club != 0 {
        let c = get_contract_unstored(p);
        let mut comp = compensacao;
        if comp == -1 {
            comp = if c != 0 && read_u8(c + contract::CONTRACT_TYPE) & 0x3f == 2 {
                let days: unsafe extern "thiscall" fn(usize) -> i16 = core::mem::transmute(FN_STAFF_CONTRACT_DAYS_TO_END_DATE);
                (days(c) as i32).wrapping_mul(read_usize(c + contract::WAGE) as i32) / 7
            } else {
                0
            };
        }
        let finances: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FINANCETABLE_GET_CLUB_FINANCE);
        let pay: unsafe extern "thiscall" fn(usize, i32) = core::mem::transmute(FN_FINANCE_PAY_WAGES);
        let fin = finances(read_usize(G_FINANCE_TABLE), club);
        if comp != 0 {
            crate::comparison::branch("cpy_13 compensacao paga");
        }
        pay(fin, comp); // cpy13: the compensation to the club
        let lists: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(FN_SHORTLIST_MANAGER_UPDATE_STAFF_STATUS);
        lists(read_usize(this + transfer_manager_fields::SHORTLIST_MANAGER), p, club, 4, 2); // cpy13: the status in the scouting lists
        if read_usize(p + staff::CLUB_JOB) != 0 {
            let escalacao: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_TACTICS_REMOVE_PLAYER_FROM_SELECTION);
            let squad: unsafe extern "C" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_DATABASE_REMOVE_STAFF_FROM_CLUB);
            crate::comparison::branch("cpy_13 sai do clube atual");
            escalacao(read_usize(G_TACTICS), read_usize(p + staff::CLUB_JOB), p, 1, 1); // cpy13: the line-ups
            escalacao(read_usize(G_TACTICS), read_usize(p + staff::CLUB_JOB), p, 0, 1);
            squad(read_usize(p + staff::CLUB_JOB), p, 0, 1, 0);
        }
        copy_today_to(p + staff::DATE_JOINED_CLUB);
    }
    let c = get_contract_unstored(p);
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
    if c != 0 && friendly_club_by_id(read_usize(c + contract::CLUB)) != 0 {
        let c = get_contract_unstored(p);
        *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
        let owner = if c == 0 { 0 } else { friendly_club_by_id(read_usize(c + contract::CLUB)) };
        let remove_offers: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_CONTRACT_MANAGER_REMOVE_CONTRACT_OFFER);
        remove_offers(read_usize(G_CONTRACT_MANAGER), p, owner);
    }
    let tirar_contrato: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_CONTRACT_MANAGER_REMOVE_CONTRACT);
    tirar_contrato(read_usize(G_CONTRACT_MANAGER), p);
    core::ptr::write_unaligned((p + staff::VALUE) as *mut u32, 0);
    *((p + staff::SQUAD_SELECTED_FOR) as *mut u8) = 0;
    let write_percent_pair: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_REMOVE_NEXT_SALE_PERCENTAGE);
    let bonus_team: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_REMOVE_FIRST_TEAM_APPEARANCE_MONEY);
    let bonus_national_team: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_REMOVE_INTERNATIONAL_TEAM_APPEARANCE_MONEY);
    let permissao: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_UPDATE_WORK_PERMIT_ON_MOVE);
    let end: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_AWOL_CLEAR_AWOL);
    write_percent_pair(this, p);
    bonus_team(this, p);
    bonus_national_team(this, p);
    permissao(this, p, 0);
    end(read_usize(G_AWOL), p);
}

// simplification: pays, removes from lists, line-ups, squad and contract (cannot run twice): original only, checked
// by the plan: on loan, the past transfers count stays; otherwise it goes up by 1 with the new record (the person, the
// club, −1 and type 5 or 3) and, outside registration only, without a main contract, with +0x52 and +0x6d zeroed.

// simplification: answers news and changes the day’s interest (cannot run twice): original only, checked by the
// plan: the holiday mark becomes `férias` and the count changes by ±1 only when the mark changes.

/// Sends the 0xfac news of permission to discuss a contract to the human buyer and, per exchange player, to the human
/// seller.
///
/// Ret 4; SEH frame for the local news items: `TRANSFER_MANAGER::post_contract_talks_news(offer)`, from the accepted
/// offer (0x8c0080). The seller: the club of the person's contract (0x4cf5a0, stored in `[0xae1080]`) except type 5.
/// Human buyer (0x7875e0): news 0xfac with the person (the id and the three keys), the seller (or −1), the buyer,
/// +0x20, +0x24, 0 and 0. Human seller with an exchange (+4): one 0xfac per exchange player, with him, the buyer, the
/// seller, +0x20, +0x24, 0 and 1, to the seller.
/// Original: `post_contract_talks_news` (0x8d7cd0, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8d7cd0 } (thiscall, ret 4, transfer_manager.cpp, quadro SEH das notícias locais): `TRANSFER_MANAGER::
/// post_contract_talks_news(oferta)`, pela oferta aceita (0x8c0080). O vendedor: o clube do contrato da pessoa (0x4cf5a0,
/// guardado em [0xae1080]) fora do tipo 5. Comprador humano (0x7875e0): a notícia 0xfac com a pessoa (o id e as três
/// chaves), o vendedor (ou −1), o comprador, +0x20, +0x24, 0 e 0. Vendedor humano com troca (+4): uma 0xfac por jogador
/// da troca, com ele, o comprador, o vendedor, +0x20, +0x24, 0 e 1, ao vendedor.
pub(crate) unsafe extern "thiscall" fn post_contract_talks_news(_this: usize, o: usize) {
    let construct: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let write: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let deliver: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    // the original puts the newspaper in ecx; 0x7875e0 is stdcall (ret 4) and does not read it
    let human: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_NEWS_CLUB_HAS_HUMAN_MANAGER);
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let p = staff_records_staff_by_id(read_usize(o + offer::TARGET_STAFF_ID));
    let c = get_contract_unstored(p);
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
    let seller = if c == 0 || read_u8(c + contract::CONTRACT_TYPE) & 0x3f == 5 { 0 } else { friendly_club_by_id(read_usize(c + contract::CLUB)) };
    let buyer = friendly_club_by_id(read_usize(o + offer::BUYER_CLUB_ID));
    let keys = |nt: usize, q: usize| {
        write(nt, 0, read_usize(q) as u32);
        write(nt, 1, read_usize(read_usize(q + staff::FIRST_NAME) + 0x33) as u32);
        write(nt, 2, read_usize(read_usize(q + staff::SECOND_NAME) + 0x33) as u32);
        write(nt, 3, read_usize(read_usize(q + staff::COMMON_NAME) + 0x33) as u32);
    };
    if human(read_usize(G_NEWS), buyer) != 0 {
        crate::comparison::branch("cpy_13 noticia ao comprador");
        let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
        let nt = local.as_mut_ptr() as usize;
        construct(nt, 0xfac, 0);
        keys(nt, p);
        write(nt, 4, if seller != 0 { read_usize(seller) as u32 } else { u32::MAX });
        write(nt, 5, read_usize(o + offer::BUYER_CLUB_ID) as u32);
        write(nt, 7, read_usize(o + offer::SLOT_INDEX) as u32);
        write(nt, 8, read_usize(o + offer::SERIAL) as u32);
        write(nt, 9, 0);
        write(nt, 0xa, 0);
        deliver(read_usize(G_NEWS), nt, buyer);
    }
    if seller == 0 || human(read_usize(G_NEWS), seller) == 0 || read_usize(o + offer::EXCHANGE_PLAYERS) == 0 {
        return;
    }
    let swap = read_usize(o + offer::EXCHANGE_PLAYERS);
    core::ptr::write_unaligned((swap + llist::CURSOR) as *mut usize, read_usize(swap + llist::HEAD));
    let mut e = next(read_usize(o + offer::EXCHANGE_PLAYERS));
    let mut local = [0u32; 0x38]; // the second local news item
    let nt = local.as_mut_ptr() as usize;
    while e != 0 {
        let q = staff_records_staff_by_id(read_usize(e));
        crate::comparison::branch("cpy_13 noticia ao vendedor");
        construct(nt, 0xfac, 0);
        keys(nt, q);
        write(nt, 4, read_usize(o + offer::BUYER_CLUB_ID) as u32);
        write(nt, 5, read_usize(seller) as u32);
        write(nt, 7, read_usize(o + offer::SLOT_INDEX) as u32);
        write(nt, 8, read_usize(o + offer::SERIAL) as u32);
        write(nt, 9, 0);
        write(nt, 0xa, 1);
        deliver(read_usize(G_NEWS), nt, seller);
        e = next(read_usize(o + offer::EXCHANGE_PLAYERS));
    }
}

// simplification: the news goes to the managers’ inboxes (allocates): original only; the copy takes the branches.

/// Person -> main contract (get_contract), written to G_CONTRACT_LOOKUP_RESULT as the original does.
///
/// [pt] O contrato principal da pessoa (CONTRACT_MANAGER::get_contract 0x4cf5a0 em [0xae1050]), guardado em [0xae1080] como
/// a original faz depois de cada chamada.
pub(crate) unsafe fn transfer_manager_get_contract_and_store(person: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = f(read_usize(G_CONTRACT_MANAGER), person);
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
    c
}

/// Rewinds a LLIST's cursor (+0xa = +2) and collects all node pointers.
///
/// [pt] Os itens de uma LLIST do começo (o cursor +0xa volta a +2), como os laços da original.
pub(crate) unsafe fn llist_items_from_start(l: usize) -> Vec<usize> {
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    core::ptr::write_unaligned((l + llist::CURSOR) as *mut usize, read_usize(l + llist::HEAD));
    let mut v = Vec::new();
    let mut e = next(l);
    while e != 0 {
        v.push(e);
        e = next(l);
    }
    v
}

/// The comparer's dry run: the copy runs without the calls that touch the rest of the game (the unrest with a random
/// draw, the news item, the scheduling, the scouting lists and the contract offers), which cannot run twice.
///
/// [pt] O ensaio do comparador: a cópia roda sem as chamadas que mexem no resto do jogo (o desassossego com sorteio, a
/// notícia, o agendamento, as listas de observação e as ofertas de contrato), que não podem rodar duas vezes.
pub(crate) static mut OFFER_RESPONSE_DRY_RUN: bool = false;

/// The offers that the dry run moved from status 0xf to 8 (`dry_run_restore_contract_offers`), returned to 0xf after
/// the copy.
///
/// [pt] As ofertas que o ensaio passou do estado 0xf para o 8 (`dry_run_restore_contract_offers`), devolvidas ao 0xf depois da cópia.
pub(crate) static mut DRY_RUN_OFFERS_TO_RESTORE: Vec<usize> = Vec::new();

/// In the rehearsal, what 0x4cf4a0 (the person's stored contract offers come back) changes in what the offer mark
/// reads.
///
/// In each stored contract offer of the person (list [[0xae1050]+0x38], +0 the id) linked to a transfer offer (+0x59
/// the slot, ≠ −1; 0x8c61e0 of the management [0xde1f54]) with the same series (+0x5d = its +0x24), an offer in status
/// 0xf goes back to 8 and no longer counts as closed (recorded for undo).
///
/// [pt] No ensaio, o que a 0x4cf4a0 (as ofertas de contrato guardadas da pessoa voltam) muda no que a marca de oferta lê: em
/// cada oferta de contrato guardada da pessoa (lista [[0xae1050]+0x38], +0 o id) ligada a uma oferta de transferência
/// (+0x59 a vaga, ≠ −1; 0x8c61e0 da gerência [0xde1f54]) com a mesma série (+0x5d = +0x24 dela), a oferta no estado
/// 0xf volta ao 8 e deixa de contar como encerrada.
pub(crate) unsafe fn dry_run_restore_contract_offers(id: usize) {
    let slot: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_TRANSFER_MANAGER_GET_OFFER_SLOT);
    for e in llist_items_keep_cursor(read_usize(read_usize(G_CONTRACT_MANAGER) + 0x38)) {
        if read_usize(e) != id || read_usize(e + contract_offer::TRANSFER_OFFER_SLOT) == usize::MAX {
            continue;
        }
        let v = slot(read_usize(G_TRANSFER_MANAGER), read_usize(e + contract_offer::TRANSFER_OFFER_SLOT));
        if v == 0 || read_usize(v) == 0 {
            continue;
        }
        let r = read_usize(v);
        if read_usize(r + offer::SERIAL) == read_usize(e + contract_offer::TRANSFER_OFFER_SERIAL) && read_u8(r + offer::STATUS) == 0xf {
            *((r + offer::STATUS) as *mut u8) = 8;
            (*core::ptr::addr_of_mut!(DRY_RUN_OFFERS_TO_RESTORE)).push(r);
        }
    }
}

/// Transfer manager -> its Shortlist_Manager (+0x856).
///
/// [pt] A gerência da lista de observação (Shortlist_Manager em +0x856 da gerência de transferências).
pub(crate) unsafe fn shortlist_manager_of(this: usize) -> usize {
    read_usize(this + transfer_manager_fields::SHORTLIST_MANAGER)
}

/// Shortlist_Manager::clear_bid_markers_for_player(club, person); nothing during the dry run.
///
/// [pt] Shortlist_Manager::clear_bid_markers_for_player (0x84aeb0: thiscall(gerência, clube, pessoa), ret 8); no ensaio,
/// nada.
pub(crate) unsafe fn clear_bid_markers(this: usize, club: usize, person: usize) {
    if OFFER_RESPONSE_DRY_RUN {
        return;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_SHORTLIST_MANAGER_CLEAR_BID_MARKERS_FOR_PLAYER);
    f(shortlist_manager_of(this), club, person);
}

/// Shortlist_Manager::update_staff_status(person, club, event, type); nothing during the dry run.
///
/// [pt] Shortlist_Manager::update_staff_status (0x843e10: thiscall(gerência, pessoa, clube, evento, tipo), ret 0x10); no
/// ensaio, nada.
pub(crate) unsafe fn update_shortlist_status(this: usize, person: usize, club: usize, event: u32, kind: u32) {
    if OFFER_RESPONSE_DRY_RUN {
        return;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(FN_SHORTLIST_MANAGER_UPDATE_STAFF_STATUS);
    f(shortlist_manager_of(this), person, club, event, kind);
}

/// A thiscall(transfer manager, offer) call with ret 4: `0x8c1290` (unsettle), `0x8cbf70` (remove the future transfer)
/// or `0x8c6db0` (close the offer's contract offers).
///
/// In the dry run, nothing, except bit 0x10 of the person's mark which `0x8cbf70` clears (the comparer reads that byte;
/// it neither reads nor restores the marks of the exchange players).
///
/// [pt] Uma chamada thiscall(gerência de transferências, oferta) com ret 4 (0x8c1290 desassossegar, 0x8cbf70 tirar a
/// transferência futura, 0x8c6db0 fechar as ofertas de contrato da oferta); no ensaio, nada, só o bit 0x10 da marca da
/// pessoa que a 0x8cbf70 apaga (o comparador lê o byte; as marcas dos jogadores da troca ele não lê nem devolve).
pub(crate) unsafe fn call_with_offer(f: usize, this: usize, o: usize) {
    if OFFER_RESPONSE_DRY_RUN {
        if f == 0x8cbf70 {
            *((read_usize(this + transfer_manager_fields::PERSON_FLAGS).wrapping_add(read_usize(o + offer::TARGET_STAFF_ID).wrapping_mul(8))) as *mut u8) &= 0xef;
        }
        return;
    }
    let f: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(f);
    f(this, o);
}

/// Offer status (+0x2c) is 2, 3, 4, 0x10, 0x14, or 0xf with +0x2e >= 3 (closed for the under-offer mark).
///
/// [pt] O fim comum da recusa e da retirada, a partir do teste da oferta encerrada: sem oferta rival (+0) em aberto, a marca
/// de oferta em andamento (bit 0 do registro de 8 bytes da pessoa em +0x86e) sai.
pub(crate) unsafe fn offer_is_closed(o: usize) -> bool {
    match read_u8(o + offer::STATUS) {
        2 | 3 | 0x14 | 0x10 | 4 => true,
        0xf => read_u8(o + offer::RESPONSE_ROUND) >= 3,
        _ => false,
    }
}

/// If the offer and all rival offers in its list (+0) are closed, clears bit 0 of the target's flags record (+0x86e).
pub(crate) unsafe fn clear_offer_mark_if_closed(this: usize, o: usize) {
    if !offer_is_closed(o) {
        return;
    }
    let rivais = read_usize(o);
    if rivais != 0 {
        let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
        core::ptr::write_unaligned((rivais + llist::CURSOR) as *mut usize, read_usize(rivais + llist::HEAD));
        let mut e = next(read_usize(o));
        while e != 0 {
            let r = read_usize(read_usize(this).wrapping_add(read_usize(e).wrapping_mul(4)));
            if !offer_is_closed(r) {
                return;
            }
            e = next(read_usize(o));
        }
    }
    let m = (read_usize(this + transfer_manager_fields::PERSON_FLAGS).wrapping_add(read_usize(o + offer::TARGET_STAFF_ID).wrapping_mul(8))) as *mut u8;
    if *m & 1 != 0 {
        *m &= 0xfe;
    }
}

/// The start of the buyer's shortlist (0x849ce0) and the contract offers (0x4cca70 and, via slot +0x20, 0x4ccb30 of
/// each player in the exchange, whose mark +4 is zeroed), with +0x10 zeroed between them.
///
/// And, with an exchange, a seller and a computer-controlled seller (0x600a50), the status in the buyer's shortlist of
/// each exchange player who was on it (0x842080, 0x843e10 event 1).
///
/// [pt] O começo da lista de observação do comprador (0x849ce0) e as ofertas de contrato (0x4cca70 e, pela vaga +0x20, a
/// 0x4ccb30 de cada jogador da troca, cuja marca +4 zera), com +0x10 zerado entre elas; e, com troca, vendedor e
/// vendedor do computador (0x600a50), a situação na lista do comprador de cada jogador da troca que estava nela
/// (0x842080, 0x843e10 evento 1).
pub(crate) unsafe fn update_buyer_shortlist_entry(this: usize, o: usize) {
    let entry: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(FN_SHORTLIST_MANAGER_FIND_CLUB_SHORTLIST_ENTRY);
    let e = entry(shortlist_manager_of(this), friendly_club_by_id(read_usize(o + offer::BUYER_CLUB_ID)), staff_records_staff_by_id(read_usize(o + offer::TARGET_STAFF_ID)), (read_u8(o + offer::JOB_TYPE) != 0xb) as u32, 0);
    if e != 0 {
        // simplification: the original pushes the entry pointer register with the setne in the low byte; 0x843e10 only reads
        // o byte
        let kind = (e as u32 & !0xff) | (read_u8(e) != 0xb) as u32;
        update_shortlist_status(this, staff_records_staff_by_id(read_usize(o + offer::TARGET_STAFF_ID)), friendly_club_by_id(read_usize(o + offer::BUYER_CLUB_ID)), 1, kind);
    }
}

/// Removes the contract offers of the target and of each exchange player (clearing +0x10 and each item's +4), and
/// updates the buyer's shortlist status of exchange players when the seller is AI.
pub(crate) unsafe fn withdraw_offer_contract_offers(this: usize, o: usize, seller: usize, buyer: usize) {
    if !OFFER_RESPONSE_DRY_RUN {
        let remove: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_CONTRACT_MANAGER_REMOVE_CONTRACT_OFFER);
        remove(read_usize(G_CONTRACT_MANAGER), staff_records_staff_by_id(read_usize(o + offer::TARGET_STAFF_ID)), friendly_club_by_id(read_usize(o + offer::BUYER_CLUB_ID)));
    }
    *((o + offer::TERMS_STATE) as *mut u8) = 0;
    let swap = read_usize(o + offer::EXCHANGE_PLAYERS);
    if swap != 0 {
        let remove_of_slot: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_CONTRACT_MANAGER_REMOVE_CONTRACT_OFFER_FOR_TRANSFER);
        for e in llist_items_from_start(swap) {
            if !OFFER_RESPONSE_DRY_RUN {
                remove_of_slot(read_usize(G_CONTRACT_MANAGER), staff_records_staff_by_id(read_usize(e)), read_usize(o + offer::SLOT_INDEX));
            }
            *((e + 4) as *mut u8) = 0;
        }
    }
    let human: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    if read_usize(o + offer::EXCHANGE_PLAYERS) != 0 && seller != 0 && human(seller, 0, 1, 0, 0) == 0 {
        let in_list: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(FN_SHORTLIST_MANAGER_IS_ON_SHORTLIST);
        for e in llist_items_from_start(read_usize(o + offer::EXCHANGE_PLAYERS)) {
            if in_list(shortlist_manager_of(this), staff_records_staff_by_id(read_usize(e)), buyer) != 0 {
                update_shortlist_status(this, staff_records_staff_by_id(read_usize(e)), buyer, 1, 0);
            }
        }
    }
}

/// Clears the buyer's bid markers for the target and the seller club's markers for each exchange player.
///
/// [pt] Desmarca os lances do comprador pelo jogador (0x84aeb0) e os do clube do contrato (`vendedor`, ou 0) por cada
/// jogador da troca.
pub(crate) unsafe fn clear_offer_and_exchange_markers(this: usize, o: usize, seller: usize) {
    let p = staff_records_staff_by_id(read_usize(o + offer::TARGET_STAFF_ID));
    clear_bid_markers(this, friendly_club_by_id(read_usize(o + offer::BUYER_CLUB_ID)), p); // cpx1: unmark the bids of the buyer
    let swap = read_usize(o + offer::EXCHANGE_PLAYERS);
    if swap != 0 {
        for e in llist_items_from_start(swap) {
            clear_bid_markers(this, seller, staff_records_staff_by_id(read_usize(e)));
        }
    }
}

/// Person -> club of their contract (or 0), with the contract stored in G_CONTRACT_LOOKUP_RESULT.
///
/// [pt] O clube do contrato da pessoa (ou 0), com o contrato guardado em [0xae1080].
pub(crate) unsafe fn contract_club_stored(person: usize) -> usize {
    let c = transfer_manager_get_contract_and_store(person);
    if c == 0 {
        0
    } else {
        friendly_club_by_id(read_usize(c + contract::CLUB))
    }
}

/// TRANSFER_MANAGER post_offer_rejected_news(offer, reason); nothing during the dry run.
///
/// [pt] A notícia da oferta recusada (0x8d5170: thiscall(gerência, oferta, motivo), ret 8); no ensaio, nada.
pub(crate) unsafe fn transfer_manager_post_offer_rejected_news(this: usize, o: usize, reason: u32) {
    if OFFER_RESPONSE_DRY_RUN {
        return;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_POST_OFFER_REJECTED_NEWS);
    f(this, o, reason);
}

/// TRANSFER_MANAGER::schedule_offer(offer, days, 0); nothing during the dry run.
///
/// [pt] TRANSFER_MANAGER::schedule_offer (0x8c6220: thiscall(gerência, oferta, dias, fatias), ret 0xc); no ensaio, nada.
pub(crate) unsafe fn schedule_offer_unless_dry_run(this: usize, o: usize, days: u32) {
    if OFFER_RESPONSE_DRY_RUN {
        return;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_SCHEDULE_OFFER);
    f(this, o, days, 0);
}

/// Rejects the offer: the player may become unsettled, state 2, bids unmarked, contract offers closed, the news, back
/// in 25 days for removal and the person's offer flag.
///
/// Ret 0xc: `TRANSFER_MANAGER::reject_transfer_offer(slot, mark, reason)`, from the selling club that refuses (0x8c36b0
/// and the bid evaluation). With `mark`, +0x2d of the offer = 1; the player may become unsettled (0x8c1290); a future
/// transfer (state 0x13): leaves the list (0x8cbf70). State 2 (refused); the buyer's bids for the player and those of
/// the contract club for the exchange players are removed (0x84aeb0); the linked contract offers are closed (0x8c6db0);
/// the news (0x8d5170); the offer comes back in 25 days (0x8c6220) to be removed. With no open rival, the person's
/// offer mark is removed (+0x86e). The buyer's scouting list (0x849ce0, 0x843e10), the contract offers (0x4cca70,
/// 0x4ccb30) and +0x10 = 0; with an exchange and a computer seller, the exchange players in the buyer's list (0x842080,
/// 0x843e10).
/// Original: `reject_transfer_offer` (0x8bf680, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8bf680 } (thiscall, ret 0xc, transfer_manager.cpp): `TRANSFER_MANAGER::reject_transfer_offer(vaga, marcar,
/// motivo)`, pelo clube vendedor que recusa (0x8c36b0 e a avaliação dos lances). Com `marcar`, +0x2d da oferta = 1; o
/// jogador pode se desassossegar (0x8c1290); transferência futura (estado 0x13): sai da lista (0x8cbf70). Estado 2
/// (recusada); os lances do comprador pelo jogador e os do clube do contrato pelos da troca saem (0x84aeb0); as ofertas
/// de contrato ligadas fecham (0x8c6db0); a notícia (0x8d5170); a oferta volta em 25 dias (0x8c6220) para sair. Sem
/// rival em aberto, a marca de oferta da pessoa sai (+0x86e). A lista de observação do comprador (0x849ce0, 0x843e10),
/// as ofertas de contrato (0x4cca70, 0x4ccb30) e +0x10 = 0; com troca e vendedor do computador, os jogadores da troca
/// na lista do comprador (0x842080, 0x843e10).
pub(crate) unsafe extern "thiscall" fn reject_transfer_offer(this: usize, slot: *mut usize, mark: u32, reason: u32) {
    let o = *slot;
    let seller = contract_club_stored(staff_records_staff_by_id(read_usize(o + offer::TARGET_STAFF_ID)));
    let buyer = friendly_club_by_id(read_usize(o + offer::BUYER_CLUB_ID));
    if mark != 0 {
        *((*slot + 0x2d) as *mut u8) = 1;
    }
    crate::comparison::branch("cpx_1 recusa desassossega");
    call_with_offer(0x8c1290, this, o); // cpx1: the unsettlement from the refusal
    let current = *slot;
    if read_u8(current + offer::STATUS) == 0x13 {
        call_with_offer(0x8cbf70, this, current);
    }
    *((o + offer::STATUS) as *mut u8) = 2;
    let vendedor2 = contract_club_stored(staff_records_staff_by_id(read_usize(o + offer::TARGET_STAFF_ID)));
    crate::comparison::branch("cpx_1 recusa desmarca");
    clear_offer_and_exchange_markers(this, o, vendedor2);
    call_with_offer(0x8c6db0, this, o);
    transfer_manager_post_offer_rejected_news(this, o, reason);
    schedule_offer_unless_dry_run(this, o, 0x19);
    clear_offer_mark_if_closed(this, o);
    update_buyer_shortlist_entry(this, o);
    withdraw_offer_contract_offers(this, o, seller, buyer);
}

/// Withdraws the offer: state 3, held contract offers resubmitted, bids unmarked, back in 10 days; for reason 2, the
/// unsettling; the news and the contract offers.
///
/// Ret 0xc: `TRANSFER_MANAGER::withdraw_transfer_offer(slot, mark, reason)`, from the buyer who gives up (the bid
/// review and the collapsed offers). With `mark`, +0x2d = 1; a future transfer: leaves the list (0x8cbf70); state 0xb
/// and above: the held contract offers come back (0x4cf4a0). State 3 (withdrawn); the linked contract offers are closed
/// (0x8c6db0); the bids are removed (0x84aeb0); +0x10 = 4; the offer comes back in 10 days (0x8c6220). With no open
/// rival, the person's offer mark is removed. Reason 2: the player may become unsettled (0x8c1290). The buyer's
/// scouting list, the news (0x8d5170), the contract offers and +0x10 = 0; with an exchange and a computer seller, the
/// exchange players in the buyer's list.
/// Original: `withdraw_transfer_offer` (0x8bfcb0, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8bfcb0 } (thiscall, ret 0xc, transfer_manager.cpp): `TRANSFER_MANAGER::withdraw_transfer_offer(vaga, marcar,
/// motivo)`, pelo comprador que desiste (a revisão dos lances e as ofertas desfeitas). Com `marcar`, +0x2d = 1;
/// transferência futura: sai da lista (0x8cbf70); estado de 0xb para cima: as ofertas de contrato guardadas voltam
/// (0x4cf4a0). Estado 3 (retirada); as ofertas de contrato ligadas fecham (0x8c6db0); os lances saem (0x84aeb0); +0x10 =
/// 4; a oferta volta em 10 dias (0x8c6220). Sem rival em aberto, a marca de oferta da pessoa sai. Motivo 2: o jogador
/// pode se desassossegar (0x8c1290). A lista de observação do comprador, a notícia (0x8d5170), as ofertas de contrato e
/// +0x10 = 0; com troca e vendedor do computador, os jogadores da troca na lista do comprador.
pub(crate) unsafe extern "thiscall" fn withdraw_transfer_offer(this: usize, slot: *mut usize, mark: u32, reason: u32) {
    let o = *slot;
    let seller = contract_club_stored(staff_records_staff_by_id(read_usize(o + offer::TARGET_STAFF_ID)));
    let buyer = friendly_club_by_id(read_usize(o + offer::BUYER_CLUB_ID));
    if mark != 0 {
        *((*slot + 0x2d) as *mut u8) = 1;
    }
    let current = *slot;
    if read_u8(current + offer::STATUS) == 0x13 {
        call_with_offer(0x8cbf70, this, current);
    }
    if read_u8(o + offer::STATUS) as i8 >= 0xb {
        if OFFER_RESPONSE_DRY_RUN {
            dry_run_restore_contract_offers(read_usize(o + offer::TARGET_STAFF_ID));
        } else {
            let reenviar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_CONTRACT_MANAGER_RESUBMIT_HELD_OFFERS);
            reenviar(read_usize(G_CONTRACT_MANAGER), read_usize(o + offer::TARGET_STAFF_ID));
        }
    }
    *((o + offer::STATUS) as *mut u8) = 3;
    call_with_offer(0x8c6db0, this, o);
    let vendedor2 = contract_club_stored(staff_records_staff_by_id(read_usize(o + offer::TARGET_STAFF_ID)));
    clear_offer_and_exchange_markers(this, o, vendedor2);
    *((o + offer::TERMS_STATE) as *mut u8) = 4;
    schedule_offer_unless_dry_run(this, o, 0xa);
    clear_offer_mark_if_closed(this, o);
    if reason as u16 == 2 { // cpx1: só o motivo 2 desassossega
        call_with_offer(0x8c1290, this, o);
    } else {
        crate::comparison::branch("cpx_1 retirada sem desassossego");
    }
    update_buyer_shortlist_entry(this, o);
    transfer_manager_post_offer_rejected_news(read_usize(G_TRANSFER_MANAGER), o, reason);
    withdraw_offer_contract_offers(this, o, seller, buyer);
}

/// All node pointers of a LLIST (empty if null), restoring its cursor afterwards.
///
/// [pt] As vagas de uma LLIST (ou nenhuma), sem mexer no cursor.
pub(crate) unsafe fn llist_items_keep_cursor(l: usize) -> Vec<usize> {
    if l == 0 {
        return Vec::new();
    }
    let cursor = read_usize(l + llist::CURSOR);
    let v = llist_items_from_start(l);
    core::ptr::write_unaligned((l + llist::CURSOR) as *mut usize, cursor);
    v
}

/// The comparer of offer rejection and withdrawal.
///
/// The copy as a dry run (without the calls that touch the rest of the game) and the original from the same state (the
/// offer, its two lists, the exchange slots, the person's record at +0x86e and [0xae1080]); equal: the status (+0x2c),
/// +0x10, the +4 mark of each player in the exchange and the person's under-offer mark, which the original writes
/// independently of the calls.
///
/// [pt] O comparador da recusa e da retirada: a cópia em ensaio (sem as chamadas que mexem no resto do jogo) e a original a
/// partir do mesmo estado (a oferta, as duas listas dela, as vagas da troca, o registro da pessoa em +0x86e e
/// [0xae1080]); iguais o estado (+0x2c), +0x10, a marca +4 de cada jogador da troca e a marca de oferta da pessoa, que
/// a original grava sem depender das chamadas.
pub(crate) unsafe fn compare_reject_withdraw(name: &str, this: usize, slot: *mut usize, original: impl FnOnce(), copy: impl FnOnce()) {
    let o = *slot;
    let mark = read_usize(this + transfer_manager_fields::PERSON_FLAGS).wrapping_add(read_usize(o + offer::TARGET_STAFF_ID).wrapping_mul(8));
    let nos = llist_items_keep_cursor(read_usize(o + offer::EXCHANGE_PLAYERS));
    let mut regions = vec![(o, 0x30usize), (mark, 8), (G_CONTRACT_LOOKUP_RESULT, 4)];
    for l in [read_usize(o), read_usize(o + offer::EXCHANGE_PLAYERS)] {
        if l != 0 {
            regions.push((l, 0xe));
        }
    }
    for &e in &nos {
        regions.push((e, 5));
    }
    let before = crate::comparison::MemorySnapshot::read(&regions);
    OFFER_RESPONSE_DRY_RUN = true;
    crate::rng::pause_trace(copy);
    OFFER_RESPONSE_DRY_RUN = false;
    let mr = crate::comparison::MemorySnapshot::read(&regions);
    for r in (*core::ptr::addr_of_mut!(DRY_RUN_OFFERS_TO_RESTORE)).drain(..) {
        *((r + offer::STATUS) as *mut u8) = 0xf;
    }
    before.write_back();
    original();
    let mo = crate::comparison::MemorySnapshot::read(&regions);
    let (a, b) = (&mo.0, &mr.0);
    let mut same = a[0].1[0x2c] == b[0].1[0x2c] && a[0].1[0x10] == b[0].1[0x10] && a[1].1[0] == b[1].1[0];
    for k in regions.len() - nos.len()..regions.len() {
        same &= a[k].1[4] == b[k].1[4];
    }
    SCOREBOARD.record(same, || format!("{}(oferta {:#x}) original={:x?} rust={:x?}", name, o, mo, mr));
}

/// Sends the 0xfa1 news of a rejected or withdrawn offer, with the reason, to the interested human clubs.
///
/// Ret 8; SEH frame for the local news item: `TRANSFER_MANAGER::post_offer_rejected_news(offer, reason)`, from the
/// refusal, the withdrawal and the loan refusal. The human clubs that receive it (0x8d1130, with 0x2d if the offer is
/// refused, otherwise 0x2e; up to 30); with none, nothing. News 0xfa1: the person (the id and the three keys), the
/// contract club (without a type 5 contract; none for reason 6 with +0x2f), the buyer, the exchange list (+4), the
/// state, the reason, +0x1f, the total value (0x8e7a70 with 0, 0, 0, 1) and the contract club's division (+0x57); to
/// each recipient, in slot 12, 1 if it is the buyer or the contract club.
/// Original: `post_offer_rejected_news` (0x8d5170, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8d5170 } (thiscall, ret 8, transfer_manager.cpp, quadro SEH da notícia local): `TRANSFER_MANAGER::
/// post_offer_rejected_news(oferta, motivo)`, pela recusa, pela retirada e pela recusa do empréstimo. Os clubes humanos
/// que recebem (0x8d1130, com 0x2d se a oferta estiver recusada, senão 0x2e; até 30); sem nenhum, nada. A notícia 0xfa1:
/// a pessoa (o id e as três chaves), o clube do contrato (sem contrato de tipo 5; nenhum no motivo 6 com +0x2f), o
/// comprador, a lista da troca (+4), o estado, o motivo, +0x1f, o valor total (0x8e7a70 com 0, 0, 0, 1) e a divisão do
/// clube do contrato (+0x57); a cada destinatário, na vaga 12, 1 se for o comprador ou o clube do contrato.
pub(crate) unsafe extern "thiscall" fn post_offer_rejected_news(this: usize, o: usize, reason: u32) {
    let recipients: unsafe extern "thiscall" fn(usize, usize, u32, *mut usize, u32) -> u8 = core::mem::transmute(FN_TRANSFER_MANAGER_COLLECT_OFFER_NEWS_RECIPIENTS);
    let construct: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let write: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let deliver: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    let total: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_OFFER_GET_TOTAL_VALUE);
    let p = staff_records_staff_by_id(read_usize(o + offer::TARGET_STAFF_ID));
    let buyer = friendly_club_by_id(read_usize(o + offer::BUYER_CLUB_ID));
    let tipo5 = |c: usize| read_u8(c + contract::CONTRACT_TYPE) & 0x3f == 5;
    let c = transfer_manager_get_contract_and_store(p);
    let of_contract = if c == 0 || tipo5(c) { 0 } else { friendly_club_by_id(read_usize(c + contract::CLUB)) };
    let mut list = [0usize; 0x1e];
    let n = recipients(this, o, (read_u8(o + offer::STATUS) != 2) as u32 + 0x2d, list.as_mut_ptr(), 0x1e) as i8;
    if n == 0 {
        return;
    }
    let c = transfer_manager_get_contract_and_store(p);
    let mut club = if c == 0 || tipo5(c) { 0 } else { friendly_club_by_id(read_usize(c + contract::CLUB)) };
    let c = transfer_manager_get_contract_and_store(p);
    if c != 0 && tipo5(c) {
        club = 0;
    }
    if reason as i16 == 6 && read_u8(o + offer::PRE_CONTRACT) != 0 {
        club = 0;
    }
    crate::comparison::branch("cpx_1 noticia da recusa");
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    construct(nt, 0xfa1, 0);
    write(nt, 0, read_usize(p) as u32);
    write(nt, 1, read_usize(read_usize(p + staff::FIRST_NAME) + 0x33) as u32);
    write(nt, 2, read_usize(read_usize(p + staff::SECOND_NAME) + 0x33) as u32);
    write(nt, 3, read_usize(read_usize(p + staff::COMMON_NAME) + 0x33) as u32);
    write(nt, 4, if club != 0 { read_usize(club) as u32 } else { u32::MAX });
    write(nt, 5, read_usize(o + offer::BUYER_CLUB_ID) as u32);
    write(nt, 6, read_usize(o + offer::EXCHANGE_PLAYERS) as u32);
    write(nt, 7, read_u8(o + offer::STATUS) as i8 as u32);
    write(nt, 8, reason as i16 as u32);
    write(nt, 9, read_u8(o + offer::OFFER_TYPE) as i8 as u32);
    write(nt, 10, total(o, 0, 0, 0, 1));
    let division = if club != 0 { read_usize(club + club::DIVISION) } else { 0 };
    write(nt, 11, if division != 0 { read_usize(division) as u32 } else { u32::MAX });
    for &d in list.iter().take(n.max(0) as usize) {
        write(nt, 12, (d == buyer || d == of_contract) as u32);
        deliver(read_usize(G_NEWS), nt, d);
    }
}

// simplification: the news item goes to the managers’ inboxes (allocates): original only; the copy takes the branch.

/// Person -> main contract, also written to G_CONTRACT_LOOKUP_RESULT.
///
/// [pt] O mesmo, guardado em [0xae1080] como a original faz.
pub(crate) unsafe fn get_contract_store_lookup(person: usize) -> usize {
    let c = contract_of_staff(person);
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
    c
}

/// The dry run of the comparer.
///
/// The copy stops before the grievance (`0x867700`), the escape (`0x419840`) and the dislikes (`0x547e70`, `0x547af0`),
/// which touch the rest of the game, and records that it got there.
///
/// [pt] O ensaio do comparador: a cópia para antes da queixa (0x867700), da fuga (0x419840) e das antipatias (0x547e70,
/// 0x547af0), que mexem no resto do jogo, e marca que chegou lá.
pub(crate) static mut UNSETTLE_PLAYER_DRY_RUN: bool = false;
pub(crate) static mut UNSETTLE_PLAYER_REACHED: bool = false;

/// A random draw against the buyer's reputation flags the player as unsettled; when flagged, a score from his
/// attributes and interest leads to the 0x80 complaint, going missing and grudges.
///
/// Ret 4: `TRANSFER_MANAGER::unsettle_player_after_bid(offer)`, from the refusal (0x8bf680), the withdrawal with reason
/// 2 (0x8bfcb0) and the club that refuses (0x8c36b0). Random(5750) + 4250 below the buyer's reputation (+0x80): +0x2d
/// of the offer = 1. With +0x2d = 1, a contract club, a buyer and a player sheet: the interest x (0x846920 by status
/// 0x849f00) and the score n = (8 × ambition² − 1600) − reputation / 40 − min(x⁴, 300 × x) − 2 × loyalty²...
/// (attributes +0x5d, +0x57, +0x5b, +0x59 through the game's pow); transfer-listed (+0x4e & 8): − 2000 − random(6000).
/// With x of 5.5, n < −random(700), (n < −4000 or +0x5b + +0x59 < 15), reputation above 3500 and the offer's weighted
/// value (0x8e7b50) above the lesser of 1.5 × value (0x4d0c60) and 0.8 × price (0x4d1600): the 0x80 complaint
/// (0x867700); a player of 5250, aged 19 to 31, n < −8000, with the criterion (0x866f60), without flag +1 of record
/// +0x86e and present (0x418f60): goes missing (0x419840) and, with +0x44 − random(200) + random(400) < −150, rejects
/// the club (0x547e70). x above 10, n < −5000 or the lesser below 0.75 × the weighted value: dislike of the manager
/// (0x547af0).
/// Original: `unsettle_player_after_bid` (0x8c1290, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8c1290 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::unsettle_player_after_bid(oferta)`, pela
/// recusa (0x8bf680), pela retirada no motivo 2 (0x8bfcb0) e pelo clube que recusa (0x8c36b0). Sorteio(5750) + 4250
/// abaixo da reputação do comprador (+0x80): +0x2d da oferta = 1. Com +0x2d = 1, clube do contrato, comprador e ficha: o
/// interesse x (0x846920 pelo estatuto 0x849f00) e a nota n = (8 × ambição² − 1600) − reputação / 40 − min(x⁴, 300 ×
/// x) − 2 × lealdade²... (os atributos +0x5d, +0x57, +0x5b, +0x59 pela pow do jogo); listado (+0x4e & 8): − 2000 −
/// sorteio(6000). Com x de 5,5, n < −sorteio(700), (n < −4000 ou +0x5b + +0x59 < 15), reputação acima de 3500 e o
/// valor ponderado da oferta (0x8e7b50) acima do menor entre 1,5 × valor (0x4d0c60) e 0,8 × preço (0x4d1600): a queixa
/// 0x80 (0x867700); jogador de 5250, idade 19 a 31, n < −8000, com o critério (0x866f60), sem a marca +1 do registro
/// +0x86e e presente (0x418f60): some (0x419840) e, com +0x44 − sorteio(200) + sorteio(400) < −150, rejeita o clube
/// (0x547e70). x acima de 10, n < −5000 ou o menor abaixo de 0,75 × o ponderado: antipatia ao técnico (0x547af0).
pub(crate) unsafe extern "thiscall" fn unsettle_player_after_bid(this: usize, o: usize) {
    let p = staff_records_staff_by_id(read_usize(o + offer::TARGET_STAFF_ID));
    let c = get_contract_store_lookup(p);
    let of_contract = if c == 0 { 0 } else { friendly_club_by_id(read_usize(c + contract::CLUB)) };
    let buyer = friendly_club_by_id(read_usize(o + offer::BUYER_CLUB_ID));
    let r = random(0x1676);
    crate::comparison::branch("cpx_2 sorteio do desassossego");
    if (read_i16(buyer + club::REPUTATION) as i32) > r.wrapping_add(0x109a) { // cpx2: the unsettlement random draw
        *((o + offer::PLAYER_AWARE) as *mut u8) = 1;
    }
    if read_u8(o + offer::PLAYER_AWARE) != 1 || of_contract == 0 || buyer == 0 || read_usize(p + staff::PLAYER) == 0 {
        return;
    }
    let c2 = contract_of_staff(p);
    let lists = read_usize(this + transfer_manager_fields::SHORTLIST_MANAGER);
    let status: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u8 = core::mem::transmute(FN_SHORTLIST_MANAGER_GET_SQUAD_STATUS_FOR_CLUB);
    let interest: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32) -> f64 = core::mem::transmute(FN_SHORTLIST_MANAGER_STAFF_INTEREST_IN_JOINING_CLUB);
    let e = status(lists, p, buyer, 0);
    // simplification: the original pushes the dword of the slot where it stored the byte (the rest is stack leftover); 0x846920 only reads the
    // byte
    let x = interest(lists, p, buyer, 1, 0, e as u32, 0);
    let pw = game_pow(x, read_f64_at(0x968938)); // 0x968938 = 4
    let a = x * read_f64_at(0x96a6b0); // 0x96a6b0 = 300
    let v = if !(pw >= a) { pw } else { a };
    let mut n = match_pl_ftol(game_pow(read_i8_as_i32(p + staff::TEMPERAMENT) as f64, read_f64_at(0x968950)) * read_f64_at(0x968f10) - read_f64_at(0x96a590)) as i16; // 0x968950 = 2, 0x968f10 = 8, 0x96a590 = 1600
    let sheet = read_usize(p + staff::PLAYER);
    n = n.wrapping_sub((read_i16(sheet + player_sheet::CURRENT_REPUTATION) as i32 / 40) as i16);
    n = n.wrapping_sub(match_pl_ftol(v) as i16);
    n = n.wrapping_sub(match_pl_ftol(game_pow(read_i8_as_i32(p + staff::AMBITION) as f64, read_f64_at(0x968950))) as i16); // 0x968950 = 2
    n = n.wrapping_add(match_pl_ftol(game_pow(read_i8_as_i32(p + staff::PROFESSIONALISM) as f64, read_f64_at(0x968930))) as i16); // 0x968930 = 3
    n = n.wrapping_add(match_pl_ftol(game_pow(read_i8_as_i32(p + staff::LOYALITY) as f64, read_f64_at(0x968930))) as i16); // 0x968930 = 3
    if c2 == 0 {
        return;
    }
    if read_u8(c2 + contract::TRANSFER_STATUS) & 8 != 0 {
        crate::comparison::branch("cpx_2 listado");
        n = n.wrapping_add((-2000i32).wrapping_sub(random(0x1770)) as i16); // cpx2: the discount of the listed player
    }
    crate::comparison::branch("cpx_2 interesse avaliado");
    if !(x >= read_f64_at(0x96d298)) { // cpx2: o interesse mínimo; 0x96d298 = 5.5
        return;
    }
    if n as i32 >= -random(0x2bc) {
        return;
    }
    if n >= -4000 && read_i8_as_i32(p + staff::PROFESSIONALISM) + read_i8_as_i32(p + staff::LOYALITY) >= 0xf {
        return;
    }
    if read_i16(read_usize(p + staff::PLAYER) + 0xb) <= 0xdac {
        return;
    }
    let value: unsafe extern "thiscall" fn(usize, usize, usize, u32, i32, u32) -> i32 = core::mem::transmute(FN_CONTRACT_MANAGER_CALC_PLAYER_VALUE);
    let price: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, u32) -> i32 = core::mem::transmute(FN_CONTRACT_MANAGER_CALC_ASKING_PRICE);
    let ponderado: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> i32 = core::mem::transmute(FN_TRANSFER_OFFER_GET_WEIGHTED_VALUE);
    let a1 = match_pl_ftol(value(read_usize(G_CONTRACT_MANAGER), p, of_contract, 0, -1, 0) as f64 * read_f64_at(0x968940)); // 0x968940 = 1.5
    let b1 = match_pl_ftol(price(read_usize(G_CONTRACT_MANAGER), of_contract, buyer, p, 0, 1, 1) as f64 * read_f64_at(0x9689b0)); // 0x9689b0 = 0.8
    let pond = ponderado(o, 0, 0, 0, 1, 1);
    let smaller = if a1 < b1 { a1 } else { b1 };
    if pond < smaller {
        return;
    }
    if UNSETTLE_PLAYER_DRY_RUN {
        UNSETTLE_PLAYER_REACHED = true;
        return;
    }
    crate::comparison::branch("cpx_2 desassossegado");
    let queixa: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(FN_STAFF_CONTRACT_APPLY_EVENT_FLAGS);
    queixa(c2, 0x80, 0);
    let age = read_i8_as_i32(p + staff::AGE);
    if read_i16(read_usize(p + staff::PLAYER) + 0xb) > 0x1482 && age > 0x12 && age < 0x20 && n < -8000 {
        let criterio: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_STAFF_CONTRACT_WANTS_TO_LEAVE);
        let ausente: unsafe extern "thiscall" fn(usize, usize, u32) -> u8 = core::mem::transmute(FN_AWOL_IS_PLAYER_AWOL);
        if criterio(c2) != 0
            && read_u8(read_usize(this + transfer_manager_fields::PERSON_FLAGS).wrapping_add(read_usize(p).wrapping_mul(8)) + 1) == 0
            && ausente(read_usize(G_AWOL), p, 0) == 0
        {
            let sumir: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(FN_AWOL_START_ABSENCE);
            sumir(read_usize(G_AWOL), p, buyer, 1);
            let strength = read_i8_as_i32(c2 + 0x44);
            let r1 = random(0xc8);
            let r2 = random(0x190);
            if strength.wrapping_sub(r1).wrapping_add(r2) < -150 {
                let rejeitar: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(FN_STAFF_ADD_DISLIKED_CLUB);
                rejeitar(p, of_contract, 1);
            }
        }
    }
    if x > read_f64_at(0x9688b0) || n < -5000 || (smaller as f64) < pond as f64 * read_f64_at(0x969040) { // 0x9688b0 = 10, 0x969040 = 0.75
        let desafeto: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(FN_STAFF_ADD_DISLIKED_STAFF);
        desafeto(p, read_usize(of_contract + club::MANAGER), 1);
    }
}

/// Sends the 0xfb0 news of a collapsed offer, with the club and the reason, to the interested human clubs.
///
/// Ret 0xc; SEH frame for the local news item: `TRANSFER_MANAGER::post_offer_collapsed_news(offer, club, reason)`, from
/// the accepted contract offer (0x8c38b0), the collapsed offer (0x8c44e0), the work permits (0x8c7690) and 0x8c4950.
/// The human clubs that receive it (0x8d1130 with 0x2f, up to 30); with none, nothing. News 0xfb0: the contract club
/// (without a type 5 contract), the buyer, the person (the id and the three keys), the club, the reason and +0x1f; to
/// each recipient, except the club itself for reason 0xb.
/// Original: `post_offer_collapsed_news` (0x8de790, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8de790 } (thiscall, ret 0xc, transfer_manager.cpp, quadro SEH da notícia local): `TRANSFER_MANAGER::
/// post_offer_collapsed_news(oferta, clube, motivo)`, pela oferta de contrato aceita (0x8c38b0), pela oferta desfeita
/// (0x8c44e0), pelas permissões de trabalho (0x8c7690) e por 0x8c4950. Os clubes humanos que recebem (0x8d1130 com 0x2f,
/// até 30); sem nenhum, nada. A notícia 0xfb0: o clube do contrato (sem contrato de tipo 5), o comprador, a pessoa (o id
/// e as três chaves), o clube, o motivo e +0x1f; a cada destinatário, menos o próprio clube no motivo 0xb.
pub(crate) unsafe extern "thiscall" fn post_offer_collapsed_news(this: usize, o: usize, club: usize, reason: u32) {
    let recipients: unsafe extern "thiscall" fn(usize, usize, u32, *mut usize, u32) -> u8 = core::mem::transmute(FN_TRANSFER_MANAGER_COLLECT_OFFER_NEWS_RECIPIENTS);
    let construct: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let write: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let deliver: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    let p = staff_records_staff_by_id(read_usize(o + offer::TARGET_STAFF_ID));
    let c = get_contract_store_lookup(p);
    let mut of_contract = if c == 0 || read_u8(c + contract::CONTRACT_TYPE) & 0x3f == 5 { 0 } else { friendly_club_by_id(read_usize(c + contract::CLUB)) };
    let buyer = friendly_club_by_id(read_usize(o + offer::BUYER_CLUB_ID));
    let mut list = [0usize; 0x1e];
    let n = recipients(this, o, 0x2f, list.as_mut_ptr(), 0x1e) as i8;
    if n == 0 {
        return;
    }
    if of_contract != 0 && read_u8(contract_of_staff(p) + 0x35) & 0x3f == 5 {
        of_contract = 0;
    }
    crate::comparison::branch("cpx_2 noticia da oferta desfeita");
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    construct(nt, 0xfb0, 0);
    write(nt, 0, if of_contract != 0 { read_usize(of_contract) as u32 } else { u32::MAX });
    write(nt, 1, read_usize(buyer) as u32);
    write(nt, 2, read_usize(p) as u32);
    write(nt, 3, read_usize(read_usize(p + staff::FIRST_NAME) + 0x33) as u32);
    write(nt, 4, read_usize(read_usize(p + staff::SECOND_NAME) + 0x33) as u32);
    write(nt, 5, read_usize(read_usize(p + staff::COMMON_NAME) + 0x33) as u32);
    write(nt, 6, if club != 0 { read_usize(club) as u32 } else { u32::MAX });
    write(nt, 7, reason as i16 as u32);
    write(nt, 8, read_u8(o + offer::OFFER_TYPE) as i8 as u32);
    for &d in list.iter().take(n.max(0) as usize) {
        if d != club || reason as u16 != 0xb {
            deliver(read_usize(G_NEWS), nt, d);
        }
    }
}

// simplification: the news item goes to the managers’ inboxes (allocates): original only; the copy takes the branch.

/// Sends the 0xfa6 news of appearance money paid, to the paying club and to the contract club.
///
/// Ret 0x10; SEH frame for the local news item: `TRANSFER_MANAGER::post_appearance_money_news(clause, amount, type,
/// silent)`, from the appearance bonus (0x8cb340). Silent, without a contract or without its club: nothing. Otherwise
/// news 0xfa6: the clause's person (+0: the id and the three keys), +4, the amount, the paying club (+9), the type and
/// the contract club; to the paying club and to the contract club.
/// Original: `post_appearance_money_news` (0x8db8a0, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8db8a0 } (thiscall, ret 0x10, transfer_manager.cpp, quadro SEH da notícia local): `TRANSFER_MANAGER::
/// post_appearance_money_news(cláusula, valor, tipo, calado)`, pelo prêmio por jogos (0x8cb340). Calado, sem contrato ou
/// sem o clube dele: nada. Senão a notícia 0xfa6: a pessoa da cláusula (+0: o id e as três chaves), +4, o valor, o
/// clube que paga (+9), o tipo e o clube do contrato; ao clube que paga e ao clube do contrato.
pub(crate) unsafe extern "thiscall" fn post_appearance_money_news(this: usize, clausula: usize, value: u32, kind: u32, calado: u32) {
    let _ = this;
    if calado != 0 {
        return;
    }
    let p = staff_records_staff_by_id(read_usize(clausula));
    let c = get_contract_store_lookup(p);
    if c == 0 || friendly_club_by_id(read_usize(c + contract::CLUB)) == 0 {
        return;
    }
    let construct: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let write: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let deliver: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    crate::comparison::branch("cpx_2 noticia do premio por jogos");
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    construct(nt, 0xfa6, 0);
    write(nt, 0, read_usize(p) as u32);
    write(nt, 1, read_usize(read_usize(p + staff::FIRST_NAME) + 0x33) as u32);
    write(nt, 2, read_usize(read_usize(p + staff::SECOND_NAME) + 0x33) as u32);
    write(nt, 3, read_usize(read_usize(p + staff::COMMON_NAME) + 0x33) as u32);
    write(nt, 4, read_usize(clausula + appearance_clause::AMOUNT) as u32);
    write(nt, 5, value);
    write(nt, 6, read_usize(clausula + appearance_clause::CLUB_ID) as u32);
    write(nt, 7, kind as i8 as u32);
    let c = get_contract_store_lookup(p);
    // the original reads the contract’s club id even without a contract (record 0): never happens here
    let club = if c == 0 { 0 } else { friendly_club_by_id(read_usize(c + contract::CLUB)) };
    write(nt, 8, read_usize(club) as u32);
    deliver(read_usize(G_NEWS), nt, friendly_club_by_id(read_usize(clausula + appearance_clause::CLUB_ID)));
    let c = get_contract_store_lookup(p);
    deliver(read_usize(G_NEWS), nt, if c == 0 { 0 } else { friendly_club_by_id(read_usize(c + contract::CLUB)) });
}

// simplification: the news item goes to the managers’ inboxes (allocates): original only; the copy takes the branch.

/// Person -> club of their main contract unless it is type 5 (or 0), contract stored in G_CONTRACT_LOOKUP_RESULT.
///
/// [pt] O clube do contrato principal que não é de tipo 5 (ou 0), com o contrato guardado em [0xae1080].
pub(crate) unsafe fn contract_club_not_type5(person: usize) -> usize {
    let c = get_contract_store_lookup(person);
    if c == 0 || read_u8(c + contract::CONTRACT_TYPE) & 0x3f == 5 {
        0
    } else {
        friendly_club_by_id(read_usize(c + contract::CLUB))
    }
}

/// For a non-human club that uses squad numbers (SQUAD_MANAGER), allocates the numbers of new arrivals.
///
/// [pt] O clube que recebe: sem técnico humano (0x600a50) e com números de camisa (SQUAD_MANAGER 0x85add0 em [0xdd7ef0]),
/// os números de quem chegou (0x85a470).
pub(crate) unsafe fn allocate_arrival_squad_numbers(club: usize) {
    let human: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    if human(club, 1, 1, 0, 0) != 0 {
        return;
    }
    let uses: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_SQUAD_MANAGER_CLUB_USES_SQUAD_NUMBERS);
    if uses(read_usize(G_SQUAD_MANAGER), club, 0) != 0 {
        let numerar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_SQUAD_MANAGER_ALLOCATE_SQUAD_NUMBER);
        numerar(read_usize(G_SQUAD_MANAGER), club, 0); // cpx3: the shirt numbers of the newcomers
    }
}

/// Carries out the transfer: board confidence, nation rules, clauses, payment, records, former clubs' cut, the player
/// and the exchange players moved, squad numbers, state 0x14 and back in 10 days.
///
/// Ret 4: `TRANSFER_MANAGER::finalise_transfer(offer)`, from the closing of the offer (0x8c34e0) and from the future
/// transfer. The boards' confidence (0x6978f0); the rules of the buyer's nation (the 28 at +0x8ac, by nation +0xa):
/// method 1 (without +0x1f) or 2 with the offer; the previous clauses (0x8c91e0); with a seller (contract not of type
/// 5), the payment (0x8ca8a0); without +0x1f = 1, the manager's records (0x86cd70); the club records (0x448760). With
/// an exchange, the first exchange player leaves his club's squad (+0xd7, 50 slots). With a fee (+0x1b) and without
/// +0x69 on the person, the former clubs' cut (0x8e4f20). The player goes to the buyer (0x8c9720) and the exchange
/// players to the seller, with the squad numbers; state 0x14, the offer comes back in 10 days (0x8c6220); with no open
/// rival, the offer mark (+0x86e) is removed; the bids (0x84aeb0) and the linked contract offers (0x8c6db0).
/// Original: `finalise_transfer` (0x8c9390, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8c9390 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::finalise_transfer(oferta)`, pelo
/// fechamento da oferta (0x8c34e0) e pela transferência futura. A confiança das diretorias (0x6978f0); as regras da
/// nação do comprador (as 28 de +0x8ac, pela nação +0xa): método 1 (sem +0x1f) ou 2 com a oferta; as cláusulas
/// anteriores (0x8c91e0); com vendedor (contrato sem tipo 5), o pagamento (0x8ca8a0); sem +0x1f = 1, os registros do
/// técnico (0x86cd70); os recordes do clube (0x448760). Com troca, o primeiro da troca sai do elenco do clube dele
/// (+0xd7, 50 vagas). Com valor (+0x1b) e sem +0x69 na pessoa, a parte dos clubes anteriores (0x8e4f20). O jogador vai
/// ao comprador (0x8c9720) e os da troca ao vendedor, com os números de camisa; estado 0x14, a oferta volta em 10 dias
/// (0x8c6220); sem rival em aberto, a marca de oferta (+0x86e) sai; os lances (0x84aeb0) e as ofertas de contrato
/// ligadas (0x8c6db0).
pub(crate) unsafe extern "thiscall" fn finalise_transfer(this: usize, o: usize) {
    let p = staff_records_staff_by_id(read_usize(o + offer::TARGET_STAFF_ID));
    let buyer = friendly_club_by_id(read_usize(o + offer::BUYER_CLUB_ID));
    let seller = contract_club_not_type5(p);
    let confianca: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_MANAGER_MANAGER_APPLY_TRANSFER_CONFIDENCE_CHANGES);
    crate::comparison::branch("cpx_3 confianca das diretorias");
    confianca(read_usize(G_MANAGER_MANAGER), o); // cpx3: the confidence of the boards
    let nation = read_usize(friendly_club_by_id(read_usize(o + offer::BUYER_CLUB_ID)) + 0x53);
    for k in 0..0x1c {
        let rules = read_usize(this + transfer_manager_fields::TRANSFER_RULES + k * 4);
        if read_usize(rules + transfer_rules::NATION_ID) == nation {
            if rules != 0 {
                let metodo = read_usize(read_usize(rules) + if read_u8(o + offer::OFFER_TYPE) == 0 { 4 } else { 8 });
                let f: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(metodo);
                f(rules, o);
            }
            break;
        }
    }
    let with_offer = |f: usize, t: usize| {
        let f: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(f);
        f(t, o);
    };
    with_offer(0x8c91e0, this);
    if seller != 0 {
        crate::comparison::branch("cpx_3 paga a transferencia");
        with_offer(0x8ca8a0, this); // cpx3: the transfer payment
    }
    if read_u8(o + offer::OFFER_TYPE) != 1 {
        with_offer(0x86cd70, read_usize(G_STAFF_RECORDS));
    }
    with_offer(0x448760, read_usize(G_CLUB_RECORDS));
    let swap = read_usize(o + offer::EXCHANGE_PLAYERS);
    if swap != 0 {
        let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
        core::ptr::write_unaligned((swap + llist::CURSOR) as *mut usize, read_usize(swap + llist::HEAD));
        let q = staff_records_staff_by_id(read_usize(next(read_usize(o + offer::EXCHANGE_PLAYERS))));
        let club = read_usize(q + staff::CLUB_JOB);
        for j in 0..0x32usize {
            let slot = club + club::SQUAD + j * 4;
            if read_usize(slot) == q {
                *(slot as *mut usize) = 0;
                break;
            }
        }
    }
    if read_usize(o + offer::FEE) != 0 && read_usize(p + staff::NON_PLAYER) == 0 {
        crate::comparison::branch("cpx_3 parte dos clubes anteriores");
        with_offer(0x8e4f20, this); // cpx3: the share of the previous clubs
    }
    let mudar: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_MOVE_PLAYER_TO_CLUB);
    mudar(this, p, buyer, o);
    allocate_arrival_squad_numbers(buyer);
    if read_usize(o + offer::EXCHANGE_PLAYERS) != 0 {
        for e in llist_items_from_start(read_usize(o + offer::EXCHANGE_PLAYERS)) {
            mudar(this, staff_records_staff_by_id(read_usize(e)), seller, o);
        }
        allocate_arrival_squad_numbers(seller);
    }
    *((o + offer::STATUS) as *mut u8) = 0x14;
    let schedule: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_SCHEDULE_OFFER);
    schedule(this, o, 0xa, 0);
    'marca: {
        if !offer_is_closed(o) {
            break 'marca;
        }
        let rivais = read_usize(o);
        if rivais != 0 {
            let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
            core::ptr::write_unaligned((rivais + llist::CURSOR) as *mut usize, read_usize(rivais + llist::HEAD));
            let mut e = next(read_usize(o));
            while e != 0 {
                if !offer_is_closed(read_usize(read_usize(this).wrapping_add(read_usize(e).wrapping_mul(4)))) {
                    break 'marca;
                }
                e = next(read_usize(o));
            }
        }
        let m = (read_usize(this + transfer_manager_fields::PERSON_FLAGS).wrapping_add(read_usize(o + offer::TARGET_STAFF_ID).wrapping_mul(8))) as *mut u8;
        if *m & 1 != 0 {
            *m &= 0xfe;
        }
    }
    let p = staff_records_staff_by_id(read_usize(o + offer::TARGET_STAFF_ID));
    let c = get_contract_store_lookup(p);
    let new = if c == 0 { 0 } else { friendly_club_by_id(read_usize(c + contract::CLUB)) };
    let unmark: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_SHORTLIST_MANAGER_CLEAR_BID_MARKERS_FOR_PLAYER);
    unmark(read_usize(this + transfer_manager_fields::SHORTLIST_MANAGER), friendly_club_by_id(read_usize(o + offer::BUYER_CLUB_ID)), p);
    if read_usize(o + offer::EXCHANGE_PLAYERS) != 0 {
        for e in llist_items_from_start(read_usize(o + offer::EXCHANGE_PLAYERS)) {
            unmark(read_usize(this + transfer_manager_fields::SHORTLIST_MANAGER), new, staff_records_staff_by_id(read_usize(e)));
        }
    }
    with_offer(0x8c6db0, this);
}

// simplification: moves players between clubs, pays and schedules (cannot run twice): original only, with the state plan
// final (0x14, which no later call changes); the copy takes the branches.

/// Writes the person's id and three keys (+0x33 of +4, +8, +0xc) into news slots k..k+3.
///
/// [pt] A notícia local com a pessoa (o id e as três chaves) nas vagas `k` a `k` + 3.
pub(crate) unsafe fn put_person_in_news(nt: usize, k: u8, p: usize) {
    let write: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    write(nt, k as i8, read_usize(p) as u32);
    write(nt, k.wrapping_add(1) as i8, read_usize(read_usize(p + staff::FIRST_NAME) + 0x33) as u32);
    write(nt, k.wrapping_add(2) as i8, read_usize(read_usize(p + staff::SECOND_NAME) + 0x33) as u32);
    write(nt, k.wrapping_add(3) as i8, read_usize(read_usize(p + staff::COMMON_NAME) + 0x33) as u32);
}

/// Id of the person's non-type-5 contract club, or -1 (value for news slot 5).
///
/// [pt] A vaga 5 das duas notícias: o id do clube do contrato (sem tipo 5) ou −1 (a original relê o contrato e, sem ele, lê
/// o id no endereço 0, o que não acontece porque acabou de achá-lo).
pub(crate) unsafe fn contract_club_id_or_minus1(p: usize) -> u32 {
    if contract_club_not_type5(p) == 0 {
        return u32::MAX;
    }
    read_usize(contract_club_not_type5(p)) as u32
}

/// Writes the first three exchange players of the offer into the news from slot k (4 slots each); returns (count, next
/// slot).
///
/// [pt] Os da troca (+4) na notícia a partir da vaga `k` (os três primeiros, quatro vagas cada); devolve (quantos, a vaga
/// seguinte).
pub(crate) unsafe fn put_exchange_players_in_news(nt: usize, o: usize, mut k: u8) -> (i8, u8) {
    let mut n = 0i8;
    let swap = read_usize(o + offer::EXCHANGE_PLAYERS);
    if swap != 0 {
        for e in llist_items_from_start(swap) {
            if n < 3 {
                put_person_in_news(nt, k, staff_records_staff_by_id(read_usize(e)));
                k = k.wrapping_add(4);
            }
            n = n.wrapping_add(1);
        }
    }
    (n, k)
}

/// Sends the news to all managers if the buyer has a human manager, otherwise to each of the first n recipients.
///
/// [pt] A entrega: com técnico humano no comprador (0x7875e0), a todos os técnicos (0x786780); senão a cada destinatário.
pub(crate) unsafe fn deliver_transfer_news(nt: usize, o: usize, list: &[usize], n: i8) {
    let human: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_NEWS_CLUB_HAS_HUMAN_MANAGER);
    if human(read_usize(G_NEWS), friendly_club_by_id(read_usize(o + offer::BUYER_CLUB_ID))) != 0 {
        let all: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_ALL_MANAGERS);
        all(read_usize(G_NEWS), nt);
        return;
    }
    let deliver: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    for &d in list.iter().take(n.max(0) as usize) {
        deliver(read_usize(G_NEWS), nt, d);
    }
}

/// Sends the 0xfa8 news of a completed transfer, with the fee, the exchange players, a random draw and the player's
/// strong points.
///
/// Ret 4; SEH frame for the local news item: `TRANSFER_MANAGER::post_transfer_completed_news(offer)`, from the closing
/// of the offer (0x8c34e0). The human clubs that receive it (0x8d1130 with 0x5f, up to 30); with none, nothing. News
/// 0xfa8: the person, the buyer, the contract club, the fee (+0x1b), 0, the total value (0x8e7a70 with 1, 1, 0, 1),
/// +0x2f, +0x11, the clauses (0x8e7ef0); with +0x2f, a club contract and +0x30 & 0xf = 3, the end of the contract if it
/// is still to come (slots 10 and 11). The exchange players (up to 3, from slot 15) and how many (14); a random(3) in
/// the next slot; with a player sheet, 1, the strong points (0x546ab0, random) and the positions (0x5b7d20), otherwise
/// 0.
/// Original: `post_transfer_completed_news` (0x8dbef0, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8dbef0 } (thiscall, ret 4, transfer_manager.cpp, quadro SEH da notícia local): `TRANSFER_MANAGER::
/// post_transfer_completed_news(oferta)`, pelo fechamento da oferta (0x8c34e0). Os clubes humanos que recebem (0x8d1130
/// com 0x5f, até 30); sem nenhum, nada. A notícia 0xfa8: a pessoa, o comprador, o clube do contrato, o valor (+0x1b), 0,
/// o valor total (0x8e7a70 com 1, 1, 0, 1), +0x2f, +0x11, as cláusulas (0x8e7ef0); com +0x2f, contrato de clube e +0x30
/// & 0xf = 3, o fim do contrato se ainda vier (vagas 10 e 11). Os da troca (até 3, da vaga 15) e quantos (14); um
/// sorteio(3) na vaga seguinte; com ficha, 1, os pontos fortes (0x546ab0, sorteia) e as posições (0x5b7d20), senão 0.
pub(crate) unsafe extern "thiscall" fn post_transfer_completed_news(this: usize, o: usize) {
    let recipients: unsafe extern "thiscall" fn(usize, usize, u32, *mut usize, u32) -> u8 = core::mem::transmute(FN_TRANSFER_MANAGER_COLLECT_OFFER_NEWS_RECIPIENTS);
    let construct: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let write: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let total: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_OFFER_GET_TOTAL_VALUE);
    let clausulas: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_TRANSFER_OFFER_GET_CLAUSE_TOTAL);
    let p = staff_records_staff_by_id(read_usize(o + offer::TARGET_STAFF_ID));
    let mut list = [0usize; 0x1e];
    let n = recipients(this, o, 0x5f, list.as_mut_ptr(), 0x1e) as i8;
    if n == 0 {
        return;
    }
    crate::comparison::branch("cpx_3 noticia da transferencia concluida");
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    construct(nt, 0xfa8, 0);
    put_person_in_news(nt, 0, p);
    write(nt, 4, read_usize(o + offer::BUYER_CLUB_ID) as u32);
    write(nt, 5, contract_club_id_or_minus1(p));
    write(nt, 6, read_usize(o + offer::FEE) as u32);
    write(nt, 7, 0);
    write(nt, 8, total(o, 1, 1, 0, 1));
    write(nt, 9, read_u8(o + offer::PRE_CONTRACT) as i8 as u32);
    write(nt, 0xc, read_u8(o + offer::JOB_TYPE) as i8 as u32);
    write(nt, 0xd, clausulas(o));
    if read_u8(o + offer::PRE_CONTRACT) != 0 && contract_club_not_type5(p) != 0 && read_u8(o + offer::COMPLETION_TIMING) & 0xf == 3 {
        let c = contract_of_staff(p);
        let year = read_i16(G_CURRENT_DATE_YEAR) as u16;
        let vem = c != 0
            && (year < read_i16(c + 0x2f) as u16 || (year == read_i16(c + 0x2f) as u16 && read_i16(G_CURRENT_DATE) < read_i16(c + contract::CONTRACT_EXPIRES)));
        if vem {
            write(nt, 10, read_i16(c + contract::CONTRACT_EXPIRES) as i32 as u32);
            write(nt, 11, read_i16(c + 0x2f) as u16 as u32);
        } else {
            write(nt, 10, 0);
            write(nt, 11, 0);
        }
    }
    let (m, k) = put_exchange_players_in_news(nt, o, 0xf);
    write(nt, 0xe, m as i32 as u32);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    write(nt, k as i8, sorteio(3) as u32);
    let sheet = read_usize(p + staff::PLAYER);
    if sheet != 0 {
        write(nt, k.wrapping_add(1) as i8, 1);
        let fortes: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_GET_PLAYER_STRENGTH_FLAGS);
        let positions: unsafe extern "C" fn(usize) -> i16 = core::mem::transmute(FN_FORMATION_GET_POSITION_MASK);
        let club = contract_club_not_type5(p);
        write(nt, k.wrapping_add(2) as i8, fortes(p, club));
        write(nt, k.wrapping_add(3) as i8, positions(read_usize(p + staff::PLAYER)) as i32 as u32);
    } else {
        write(nt, k.wrapping_add(1) as i8, 0);
    }
    deliver_transfer_news(nt, o, &list, n);
}

// simplification: the news item rolls and goes to the managers’ inboxes (allocates): original only; the copy takes the branch.

/// Sends the 0xfb3 news of the clubs' agreement, with the fee, the exchange players and the completion date.
///
/// Ret 8; SEH frame for the local news item: `TRANSFER_MANAGER::post_transfer_agreed_news(offer, date)`, from the
/// clubs' approval (0x8c3310). The human clubs that receive it (0x8d1130 with 0xf, up to 30); with none, nothing. News
/// 0xfb3: the person, the buyer, the contract club, the fee (+0x1b), +0x2f, the total value (0x8e7a70 with 0, 1, 0, 1),
/// the low nibble of +0x30, signed, the completion date (signed day, year), the exchange players (up to 3, from slot
/// 13) and how many (12).
/// Original: `post_transfer_agreed_news` (0x8e05d0, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8e05d0 } (thiscall, ret 8, transfer_manager.cpp, quadro SEH da notícia local): `TRANSFER_MANAGER::
/// post_transfer_agreed_news(oferta, data)`, pela aprovação dos clubes (0x8c3310). Os clubes humanos que recebem
/// (0x8d1130 com 0xf, até 30); sem nenhum, nada. A notícia 0xfb3: a pessoa, o comprador, o clube do contrato, o valor
/// (+0x1b), +0x2f, o valor total (0x8e7a70 com 0, 1, 0, 1), o nibble baixo de +0x30 com sinal, a data da conclusão (dia
/// com sinal, ano), os da troca (até 3, da vaga 13) e quantos (12).
pub(crate) unsafe extern "thiscall" fn post_transfer_agreed_news(this: usize, o: usize, data: usize) {
    let recipients: unsafe extern "thiscall" fn(usize, usize, u32, *mut usize, u32) -> u8 = core::mem::transmute(FN_TRANSFER_MANAGER_COLLECT_OFFER_NEWS_RECIPIENTS);
    let construct: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let write: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let total: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_OFFER_GET_TOTAL_VALUE);
    let p = staff_records_staff_by_id(read_usize(o + offer::TARGET_STAFF_ID));
    let mut list = [0usize; 0x1e];
    let n = recipients(this, o, 0xf, list.as_mut_ptr(), 0x1e) as i8;
    if n == 0 {
        return;
    }
    crate::comparison::branch("cpx_3 noticia da transferencia acertada");
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    construct(nt, 0xfb3, 0);
    put_person_in_news(nt, 0, p);
    write(nt, 4, read_usize(o + offer::BUYER_CLUB_ID) as u32);
    write(nt, 5, contract_club_id_or_minus1(p));
    write(nt, 6, read_usize(o + offer::FEE) as u32);
    write(nt, 7, read_u8(o + offer::PRE_CONTRACT) as i8 as u32);
    write(nt, 8, total(o, 0, 1, 0, 1));
    write(nt, 9, (((read_u8(o + offer::COMPLETION_TIMING) << 4) as i8) >> 4) as i32 as u32);
    write(nt, 0xa, read_i16(data) as i32 as u32);
    write(nt, 0xb, read_i16(data + cm_date::YEAR) as u16 as u32);
    let (m, _) = put_exchange_players_in_news(nt, o, 0xd);
    write(nt, 0xc, m as i32 as u32);
    deliver_transfer_news(nt, o, &list, n);
}

// simplification: the news item goes to the managers’ inboxes (allocates): original only; the copy takes the branch.

/// Copies date `origem` into `destino` (Date::operator= 0x417c10).
///
/// [pt] Date::operator= (0x417c10: thiscall(destino, resultado, origem), ret 8): copia a data nos dois.
pub(crate) unsafe fn transfer_manager_assign_date(target: usize, source: usize) {
    let f: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let mut result = [0u8; 8];
    f(target, result.as_mut_ptr(), source);
}

/// Date `a` (signed day +0, year +2) is before date `b`.
///
/// [pt] A data `a` (dia com sinal em +0, ano em +2) vem antes da `b`.
pub(crate) unsafe fn transfer_manager_date_before(a: usize, b: usize) -> bool {
    let (ya, yb) = (read_i16(a + cm_date::YEAR) as u16, read_i16(b + cm_date::YEAR) as u16);
    ya < yb || (ya == yb && read_i16(a) < read_i16(b))
}

/// The loan conditions from the configuration, in the original routine's order; returns the B team (or 0) when they
/// pass.
///
/// The B team linked to the club (0x540a50; it only counts if it is not the senior team), the club's squad below 50
/// (0x53c590), the person without a current loan (0x4cf6d0), the start date up to today, the end date from today on,
/// and a contract (0x4cf5a0).
///
/// [pt] As condições do empréstimo da configuração, na ordem da original: o time B ligado ao clube (0x540a50; vale só se não
/// for o principal), o elenco do clube abaixo de 50 (0x53c590), a pessoa sem empréstimo (0x4cf6d0), o começo até hoje,
/// o fim de hoje em diante e um contrato (0x4cf5a0). Devolve o time B (ou 0) quando passam.
pub(crate) unsafe fn config_loan_allowed(p: usize, club: usize, start: usize, end: usize) -> Option<usize> {
    let ligado: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
    let count: unsafe extern "C" fn(usize, u32, u32) -> u8 = core::mem::transmute(FN_DATABASE_COUNT_CLUB_SQUAD);
    let emprestado: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_LOAN_CONTRACT);
    let contract: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let mut main = 0u32;
    let mut b = ligado(club, &mut main, 1);
    if b != 0 && main != 0 {
        b = 0;
    }
    if count(club, 0, 0) as i8 >= 0x32 || emprestado(read_usize(G_CONTRACT_MANAGER), p) != 0 {
        return None;
    }
    if transfer_manager_date_before(G_CURRENT_DATE, start) || transfer_manager_date_before(end, G_CURRENT_DATE) {
        return None;
    }
    if contract(read_usize(G_CONTRACT_MANAGER), p) == 0 {
        return None;
    }
    Some(b)
}

/// Adds a loan from the configuration file: the checks, the contract extended to its end, the loan contract, the squad,
/// the squad numbers and the work permit.
///
/// Ret 0x10; SEH frame for the local contract: `TRANSFER_MANAGER::add_config_loan(person, club, start, end)`, from the
/// configuration file loans at the start of the game. Through the conditions (config_loan_allowed), the loan goes to
/// the linked B team or to the club: with the end after the person's date +0x46, the date and the contract end (+0x2d)
/// move to the end; the local contract (0x85e410, 0x85e590 with the end, 0, 1, 1; start in +0x25) is signed (0x4d01e0);
/// the suspensions are kept (0x5563f0); squad 1 (0x540980); +0x6d 2 becomes 0x10; the squad numbers (0x85add0,
/// 0x85a470); in the B team, the person goes to the reserves (0x4cc0f0). Needing a work permit at the club (0x8c7560),
/// the received end moves 7 days and the permit is given (0x8c7070 with the club's nation and the start year).
/// Original: `add_config_loan` (0x8bf400, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8bf400 } (thiscall, ret 0x10, transfer_manager.cpp, quadro SEH do contrato local): `TRANSFER_MANAGER::
/// add_config_loan(pessoa, clube, começo, fim)`, pelos empréstimos do arquivo de configuração no começo do jogo. Pelas
/// condições (config_loan_allowed), o empréstimo vai ao time B ligado ou ao clube: com o fim depois da data +0x46 da pessoa, a
/// data e o fim do contrato (+0x2d) passam ao fim; o contrato local (0x85e410, 0x85e590 com o fim, 0, 1, 1; começo em
/// +0x25) é assinado (0x4d01e0); as suspensões ficam guardadas (0x5563f0); o elenco 1 (0x540980); +0x6d 2 vira 0x10; os
/// números de camisa (0x85add0, 0x85a470); no time B, a pessoa vai às reservas (0x4cc0f0). Precisando de permissão de
/// trabalho no clube (0x8c7560), o fim recebido anda 7 dias e a permissão é dada (0x8c7070 com a nação do clube e o ano
/// do começo).
pub(crate) unsafe extern "thiscall" fn add_config_loan(this: usize, p: usize, club: usize, start: usize, end: usize) {
    let construct: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_STAFF_CONTRACT_STAFF_CONTRACT);
    let mut local = [0u8; 0x50]; // o STAFF_CONTRACT local (0x50 bytes)
    let l = local.as_mut_ptr() as usize;
    construct(l);
    let b = match config_loan_allowed(p, club, start, end) {
        Some(b) => b,
        None => return,
    };
    if transfer_manager_date_before(p + staff::DATE_EXPIRES_CLUB, end) {
        crate::comparison::branch("cpx_4 contrato estendido");
        let contract: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
        transfer_manager_assign_date(p + staff::DATE_EXPIRES_CLUB, end); // cpx4: the date of the person until the end
        transfer_manager_assign_date(contract(read_usize(G_CONTRACT_MANAGER), p) + 0x2d, end); // cpx4: the contract end until the loan end
    }
    let target = if b != 0 { b } else { club };
    let fill: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, u32) = core::mem::transmute(FN_STAFF_CONTRACT_INIT_FROM_CURRENT_CONTRACT);
    fill(l, p, target, end, 0, 1, 1);
    transfer_manager_assign_date(l + 0x25, start);
    crate::comparison::branch("cpx_4 emprestimo da configuracao");
    let assinar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_CONTRACT_MANAGER_SIGN_LOAN_CONTRACT);
    assinar(read_usize(G_CONTRACT_MANAGER), l); // cpx4: the loan contract
    let store: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_DISCIPLINE_HOLD_BANS_FOR_LOAN);
    store(read_usize(G_DISCIPLINE), p);
    let squad: unsafe extern "C" fn(usize, u32) = core::mem::transmute(FN_STAFF_SET_SQUAD_FLAGS);
    squad(p, 1);
    if read_u8(p + staff::SQUAD_SELECTED_FOR) == 2 {
        *((p + staff::SQUAD_SELECTED_FOR) as *mut u8) = 0x10;
    }
    let uses: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_SQUAD_MANAGER_CLUB_USES_SQUAD_NUMBERS);
    if uses(read_usize(G_SQUAD_MANAGER), target, 0) != 0 {
        let numerar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_SQUAD_MANAGER_ALLOCATE_SQUAD_NUMBER);
        numerar(read_usize(G_SQUAD_MANAGER), target, 0);
    }
    if b != 0 && b != club {
        let reservas: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_CONTRACT_MANAGER_MOVE_TO_RESERVES);
        reservas(read_usize(G_CONTRACT_MANAGER), p, b);
    }
    let precisa: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_PLAYER_NEEDS_WORK_PERMIT);
    if precisa(this, p, club, 0) != 0 {
        crate::comparison::branch("cpx_4 permissao de trabalho");
        let more_days: unsafe extern "thiscall" fn(usize, *mut u8, u32) -> usize = core::mem::transmute(FN_DATE_ADD_DAYS);
        let mut t = [0u8; 8];
        let r = more_days(end, t.as_mut_ptr(), 7);
        transfer_manager_assign_date(end, r);
        // the original pushes cx over the ecx that 0x417c10 zeroes: the clean year
        let permissao: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_ADD_WORK_PERMIT);
        permissao(this, p, read_usize(club + club::NATION), read_i16(start + cm_date::YEAR) as u16 as u32, 1, 1); // cpx4: the work permit
    }
}

// simplification: signs the loan, touches the squad and the permissions (cannot run twice): original only, with the
// plan by the same conditions (if it passes, the person leaves on loan; otherwise her loan stays as it was).

/// Person -> main contract computed inline as 0x4da590 does (staff state index into the contract vector), stored in
/// G_CONTRACT_LOOKUP_RESULT.
///
/// [pt] O contrato principal da pessoa como a 0x4da590 o acha (o mesmo da 0x4cf5a0, em linha): sem pessoa, id fora da tabela
/// ou índice negativo no registro de 79 bytes, nenhum; guardado em [0xae1080].
pub(crate) unsafe fn contract_inline_lookup(p: usize) -> usize {
    let c = if p == 0 || read_usize(p) as i32 >= read_usize(G_STAFF_COUNT) as i32 {
        0
    } else {
        let k = read_usize(read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(p).wrapping_mul(0x4f))) as i32;
        if k < 0 {
            0
        } else {
            read_usize(read_usize(G_CONTRACT_MANAGER)).wrapping_add((k as usize).wrapping_mul(0x50))
        }
    };
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
    c
}

/// Fills the local LLIST with clubs interested in the person (Shortlist_Manager 0x842180) and returns their ids.
///
/// [pt] Os clubes interessados na pessoa (Shortlist_Manager 0x842180 com 1, 1, 0) numa LLIST local já construída; devolve os
/// ids, do começo.
pub(crate) unsafe fn interested_club_ids(p: usize, l: usize) -> Vec<usize> {
    let interested: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(FN_SHORTLIST_MANAGER_COLLECT_INTERESTED_CLUBS);
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    interested(read_usize(read_usize(G_TRANSFER_MANAGER) + 0x856), p, l, 1, 1, 0);
    core::ptr::write_unaligned((l + llist::CURSOR) as *mut usize, read_usize(l + llist::HEAD));
    let mut v = Vec::new();
    let mut e = next(l);
    while e != 0 {
        v.push(read_usize(e + 4));
        e = next(l);
    }
    v
}

// simplification: the news item goes to the managers’ inboxes (allocates): original only; the copy takes the branch.

/// Sends the 0xfab news of a club asking to delay a transfer by a week, to the interested human clubs.
///
/// Ret 8; SEH frame for the local news item: `TRANSFER_MANAGER::post_transfer_delay_request_news(offer, reason)`, from
/// the club that asks to delay the transfer by a week. The human clubs that receive it (0x8d1130 with 1, or 2 outside
/// state 0x11; up to 30); with none, nothing. News 0xfab: the contract club (not type 5) or −1, the buyer, the person
/// (the id and the three keys), the state, 0, 0, slot +0x20 and the reason; to each recipient.
/// Original: `post_transfer_delay_request_news` (0x8dda90, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8dda90 } (thiscall, ret 8, transfer_manager.cpp, quadro SEH da notícia local): `TRANSFER_MANAGER::
/// post_transfer_delay_request_news(oferta, motivo)`, pelo clube que pede para adiar a transferência uma semana. Os
/// clubes humanos que recebem (0x8d1130 com 1, ou 2 fora do estado 0x11; até 30); sem nenhum, nada. A notícia 0xfab: o
/// clube do contrato (sem tipo 5) ou −1, o comprador, a pessoa (o id e as três chaves), o estado, 0, 0, a vaga +0x20 e o
/// motivo; a cada destinatário.
pub(crate) unsafe extern "thiscall" fn post_transfer_delay_request_news(this: usize, o: usize, reason: u32) {
    let recipients: unsafe extern "thiscall" fn(usize, usize, u32, *mut usize, u32) -> u8 = core::mem::transmute(FN_TRANSFER_MANAGER_COLLECT_OFFER_NEWS_RECIPIENTS);
    let construct: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let write: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let deliver: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    let contract: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let p = staff_records_staff_by_id(read_usize(o + offer::TARGET_STAFF_ID));
    let c = contract(read_usize(G_CONTRACT_MANAGER), p);
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
    let of_contract = if c == 0 || read_u8(c + contract::CONTRACT_TYPE) & 0x3f == 5 { 0 } else { friendly_club_by_id(read_usize(c + contract::CLUB)) };
    let buyer = friendly_club_by_id(read_usize(o + offer::BUYER_CLUB_ID));
    let mut list = [0usize; 0x1e];
    let n = recipients(this, o, (read_u8(o + offer::STATUS) != 0x11) as u32 + 1, list.as_mut_ptr(), 0x1e) as i8;
    if n == 0 {
        return;
    }
    crate::comparison::branch("cpx_4 noticia do adiamento");
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    construct(nt, 0xfab, 0);
    write(nt, 0, if of_contract != 0 { read_usize(of_contract) as u32 } else { u32::MAX });
    write(nt, 1, read_usize(buyer) as u32);
    write(nt, 2, read_usize(p) as u32);
    write(nt, 3, read_usize(read_usize(p + staff::FIRST_NAME) + 0x33) as u32);
    write(nt, 4, read_usize(read_usize(p + staff::SECOND_NAME) + 0x33) as u32);
    write(nt, 5, read_usize(read_usize(p + staff::COMMON_NAME) + 0x33) as u32);
    write(nt, 6, read_u8(o + offer::STATUS) as i8 as u32);
    write(nt, 7, 0);
    write(nt, 8, 0);
    write(nt, 9, read_usize(o + offer::SLOT_INDEX) as u32);
    write(nt, 10, reason as i16 as u32);
    for &d in list.iter().take(n.max(0) as usize) {
        deliver(read_usize(G_NEWS), nt, d);
    }
}

// simplification: the news item goes to the managers’ inboxes (allocates): original only; the copy takes the branch.

/// TRANSFER_MANAGER::remove_exchange_player(person, offer, 0, 0): removes the person from other offers' exchange lists.
///
/// [pt] Tira a pessoa das listas de troca das outras ofertas (TRANSFER_MANAGER::remove_exchange_player 0x8c41c0 com a
/// oferta, 0, 0).
pub(crate) unsafe fn remove_from_other_exchanges(this: usize, person: usize, offer: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_REMOVE_EXCHANGE_PLAYER);
    f(this, person, offer, 0, 0);
}

/// TRANSFER_MANAGER::schedule_offer(offer, days, slices).
///
/// [pt] Agenda a oferta (0x8c6220: thiscall(this, oferta, dias, fatias)).
pub(crate) unsafe fn schedule_offer_with_slices(this: usize, offer: usize, days: u32, fatias: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_SCHEDULE_OFFER);
    f(this, offer, days, fatias);
}

/// Offer status after `clube` approves: buyer moves 0xb->0xd and 0xc->0xe, the other side 0xb->0xc and 0xd->0xe; others
/// unchanged.
///
/// [pt] O estado (+0x2c) da oferta depois da aprovação do `clube`: o comprador (+8) leva 0xb a 0xd e 0xc a 0xe; o outro
/// lado (ou sem clube) leva 0xb a 0xc e 0xd a 0xe. Os outros estados ficam.
pub(crate) unsafe fn state_after_approval(offer: usize, club: usize) -> u8 {
    let s = read_u8(offer + offer::STATUS);
    if club != 0 && read_usize(club) == read_usize(offer + offer::BUYER_CLUB_ID) {
        match s {
            0xb => 0xd,
            0xc => 0xe,
            _ => s,
        }
    } else {
        match s {
            0xb => 0xc,
            0xd => 0xe,
            _ => s,
        }
    }
}

/// Records one club's approval (states 0xb to 0xe); once both agree, schedules completion for today or, with the
/// deadline still ahead, books a future transfer.
///
/// Ret 8; SEH frame for the local list: `TRANSFER_MANAGER::confirm_offer_by_club(offer, club)`, from the offer passed
/// to the clubs (0x8c2f50), the news and the screen: the club's approval changes the state (`state_after_approval`);
/// with both in agreement (0xe): without a deadline (+0x30 & 0xf), the offer is scheduled for today (0x8c6220 with 0,
/// 1); with a deadline, a completion date (0x8cee70) still to come makes it a future transfer: a local LLIST it does
/// not use (0x68d020/0x68d040), the player (+0xc) and the exchange players (list +4) leave the other offers (0x8c41c0,
/// and 0x8c47d0 for the exchange players), the offers held for the player are removed (0x4cf560), the news (0x8e05d0)
/// and the future transfer (0x8cb9a0) with the date; a date already reached: scheduled for today.
/// Original: `confirm_offer_by_club` (0x8c3310, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8c3310 } (thiscall, ret 8, transfer_manager.cpp, quadro SEH da lista local): `TRANSFER_MANAGER::
/// confirm_offer_by_club(oferta, clube)`, pela oferta passada aos clubes (0x8c2f50), pelas notícias e pela tela: a
/// aprovação do clube muda o estado (`state_after_approval`); com os dois de acordo (0xe): sem prazo (+0x30 & 0xf), a
/// oferta é agendada para hoje (0x8c6220 com 0, 1); com prazo, a data de conclusão (0x8cee70) ainda por vir faz a
/// transferência futura: uma LLIST local que não usa (0x68d020/0x68d040), o jogador (+0xc) e os da troca (lista +4)
/// saem das outras ofertas (0x8c41c0, e 0x8c47d0 para os da troca), as ofertas guardadas pelo jogador saem (0x4cf560),
/// a notícia (0x8e05d0) e a transferência futura (0x8cb9a0) com a data; data já chegada: agendada para hoje.
pub(crate) unsafe extern "thiscall" fn confirm_offer_by_club(this: usize, offer: usize, club: usize) {
    let data_default: unsafe extern "thiscall" fn(*mut [u32; 2]) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    let mut data = [0u32; 2];
    data_default(&mut data);
    let s = state_after_approval(offer, club);
    *((offer + offer::STATUS) as *mut u8) = s;
    if s != 0xe {
        return;
    }
    'future: {
        if read_u8(offer + offer::COMPLETION_TIMING) & 0xf == 0 {
            break 'future;
        }
        let conclusao: unsafe extern "thiscall" fn(usize, usize, *mut [u32; 2]) -> u16 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_OFFER_COMPLETION_DATE);
        conclusao(this, offer, &mut data);
        let today = core::ptr::read_unaligned(G_CURRENT_DATE as *const [u16; 4]);
        let d = [data[0] as u16, (data[0] >> 16) as u16];
        let before = today[1] < d[1] || (today[1] == d[1] && (today[0] as i16) < (d[0] as i16));
        if !before { // cpw2: the completion date still to come
            break 'future;
        }
        crate::comparison::branch("cpw_2 transferencia futura");
        let list_new: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
        let destroy: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
        let desfazer: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_COLLAPSE_OFFERS_FOR_PLAYER);
        let guardadas: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_CONTRACT_MANAGER_REMOVE_HELD_OFFERS);
        let news: unsafe extern "thiscall" fn(usize, usize, *mut [u32; 2]) = core::mem::transmute(FN_TRANSFER_MANAGER_POST_TRANSFER_AGREED_NEWS);
        let future: unsafe extern "thiscall" fn(usize, usize, *mut [u32; 2]) = core::mem::transmute(FN_TRANSFER_MANAGER_ADD_FUTURE_TRANSFER);
        let mut list = [0u8; 0xe];
        let l = list.as_mut_ptr() as usize;
        list_new(l);
        remove_from_other_exchanges(this, staff_records_staff_by_id(read_usize(offer + offer::TARGET_STAFF_ID)), offer);
        if read_usize(offer + offer::EXCHANGE_PLAYERS) != 0 {
            transfer_manager_llist_rewind(read_usize(offer + offer::EXCHANGE_PLAYERS));
            let mut x = list_next_item(read_usize(offer + offer::EXCHANGE_PLAYERS));
            while x != 0 {
                remove_from_other_exchanges(this, staff_records_staff_by_id(read_usize(x)), offer);
                desfazer(this, staff_records_staff_by_id(read_usize(x)));
                x = list_next_item(read_usize(offer + offer::EXCHANGE_PLAYERS));
            }
        }
        destroy(l);
        guardadas(read_usize(G_CONTRACT_MANAGER), read_usize(offer + offer::TARGET_STAFF_ID) as u32);
        news(this, offer, &mut data);
        future(this, offer, &mut data);
        return;
    }
    crate::comparison::branch("cpw_2 agendar conclusao");
    schedule_offer_with_slices(this, offer, 0, 1); // cpw2: the completion scheduled for today
}

/// Tries to complete an offer: rechecks buyer and seller; a refusal collapses or rejects the offer; with both agreeing,
/// completes the transfer or defers it to the end of the contract.
///
/// Ret 8: `TRANSFER_MANAGER::try_complete_offer(offer, at_deadline)`, from the scheduled offer (0x8c63a0) and from the
/// end of the window (0x8c4e10). The seller is the club of the player's contract (+0xc; 0x4cf5a0, stored in
/// `[0xae1080]`; type 5 does not count); the buyer (+8) reconfirms (0x8cfc90, which gives the reason): outside the
/// deadline with reason 9, it does not count; a computer buyer with answer 1: the refusal (0x8c36b0); answer 1 or 2:
/// the offer collapses (0x8c44e0 with the reason, 1). The same with the seller. Both in agreement: the player and the
/// exchange players leave the other offers (0x8c6ae0), the offers held for the player are removed (0x4cf560), the news
/// (0x8dbef0); with confirmation (+0x2f), a seller and the contract-end deadline (+0x30 & 0xf = 3), the future transfer
/// at the end of the contract (0x8cb9a0 with +0x2d of the contract); otherwise the transfer is completed (0x8c9390).
/// Original: `try_complete_offer` (0x8c34e0, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8c34e0 } (thiscall, ret 8, transfer_manager.cpp): `TRANSFER_MANAGER::try_complete_offer(oferta, no_prazo)`, pela
/// oferta agendada (0x8c63a0) e pelo fim da janela (0x8c4e10). O vendedor é o clube do contrato do jogador (+0xc;
/// 0x4cf5a0, guardado em [0xae1080]; tipo 5 não conta); o comprador (+8) reconfirma (0x8cfc90, que dá o motivo): fora do
/// prazo com motivo 9, não conta; comprador do computador com resposta 1: a recusa (0x8c36b0); resposta 1 ou 2: a oferta
/// se desfaz (0x8c44e0 com o motivo, 1). O mesmo com o vendedor. Os dois de acordo: o jogador e os da troca saem das
/// outras ofertas (0x8c6ae0), as ofertas guardadas pelo jogador saem (0x4cf560), a notícia (0x8dbef0); com confirmação
/// (+0x2f), vendedor e prazo do fim do contrato (+0x30 & 0xf = 3), a transferência futura no fim do contrato (0x8cb9a0
/// com +0x2d do contrato); senão a transferência se conclui (0x8c9390).
pub(crate) unsafe extern "thiscall" fn try_complete_offer(this: usize, offer: usize, in_deadline: u32) {
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let reconfirmar: unsafe extern "thiscall" fn(usize, usize, usize, *mut u32, u32) -> u8 = core::mem::transmute(FN_TRANSFER_MANAGER_CHECK_OFFER_RESTRICTIONS);
    let human: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let refusal: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_CLUB_REJECTS_OFFER);
    let desfazer: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_COLLAPSE_OFFER);
    let p = staff_records_staff_by_id(read_usize(offer + offer::TARGET_STAFF_ID));
    let c = contract_of(read_usize(G_CONTRACT_MANAGER), p);
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
    let seller = if c != 0 && read_u8(c + contract::CONTRACT_TYPE) & 0x3f != 5 { friendly_club_by_id(read_usize(c + contract::CLUB)) } else { 0 };
    let buyer = friendly_club_by_id(read_usize(offer + offer::BUYER_CLUB_ID));
    // simplification: 0x8cfc90 writes only the low word of the reason; the original pushes the dword (the high word is leftover from
    // stack). The copy starts from zero, a value the game also produces.
    let mut reason: u32 = 0;
    let r = reconfirmar(this, offer, buyer, &mut reason, 0);
    if in_deadline == 0 || reason as u16 != 9 {
        if human(buyer, 0, 1, 0, 0) == 0 && r == 1 {
            return refusal(this, offer, buyer, 0);
        }
        if r == 2 || r == 1 {
            return desfazer(this, offer, buyer, reason, 1);
        }
    }
    if seller != 0 {
        let r = reconfirmar(this, offer, seller, &mut reason, 0);
        if human(seller, 0, 1, 0, 0) == 0 && r == 1 {
            return refusal(this, offer, seller, 0);
        }
        if r == 2 || r == 1 {
            return desfazer(this, offer, seller, reason, 1);
        }
    }
    let soltar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_DETACH_OFFER_PLAYERS);
    let guardadas: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_CONTRACT_MANAGER_REMOVE_HELD_OFFERS);
    let news: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_POST_TRANSFER_COMPLETED_NEWS);
    soltar(this, offer);
    guardadas(read_usize(G_CONTRACT_MANAGER), read_usize(offer + offer::TARGET_STAFF_ID) as u32);
    news(this, offer);
    if read_u8(offer + offer::PRE_CONTRACT) != 0 && seller != 0 && read_u8(offer + offer::COMPLETION_TIMING) & 0xf == 3 {
        let c = contract_of(read_usize(G_CONTRACT_MANAGER), p);
        if c != 0 {
            crate::comparison::branch("cpw_2 futura no fim do contrato");
            let future: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_ADD_FUTURE_TRANSFER);
            return future(this, offer, c + contract::CONTRACT_EXPIRES);
        }
    }
    let concluir: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_FINALISE_TRANSFER);
    crate::comparison::branch("cpw_2 concluir agora");
    concluir(this, offer); // cpw2: the transfer completes
}

// simplification: completes transfers, undoes offers and gives news (allocates, does not run twice): original only; the copy
// takes the branches and is proved by the career and the mutations.

/// By the person's flags (+0x86e): pays and removes the next-sale percentage (bit 8), first-team appearance money (bit
/// 2) and international appearance money (bit 4).
///
/// [pt] As cláusulas antigas de uma pessoa pelas marcas do registro de 8 bytes em [this+0x86e]: o percentual da próxima
/// venda (bit 8: 0x8cb0f0 paga, 0x8cb4f0 tira, este da pessoa `tirar`), o prêmio por jogo no time (bit 2: 0x8cb340 com
/// −1, 2 e tira 0x8cb590) e o pela seleção (bit 4: 0x8cb340 com −1, 5 e tira 0x8cb630).
pub(crate) unsafe fn settle_old_clauses(this: usize, person: usize, remove: usize, offer: usize) {
    let mark = |p: usize| read_u8(read_usize(this + transfer_manager_fields::PERSON_FLAGS).wrapping_add(read_usize(p).wrapping_mul(8)));
    let pagar_venda: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_PAY_NEXT_SALE_PERCENTAGE);
    let tirar_venda: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_REMOVE_NEXT_SALE_PERCENTAGE);
    let pay_games: unsafe extern "thiscall" fn(usize, usize, i32, u32, usize) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_PAY_APPEARANCE_MONEY);
    let remove_games: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_REMOVE_FIRST_TEAM_APPEARANCE_MONEY);
    let remove_national_team: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_REMOVE_INTERNATIONAL_TEAM_APPEARANCE_MONEY);
    if mark(person) & 8 != 0 {
        crate::comparison::branch("cpw_2 write_percent_pair da venda anterior");
        pagar_venda(this, person, offer);
        tirar_venda(this, remove);
    }
    if mark(person) & 2 != 0 {
        crate::comparison::branch("cpw_2 premio por jogo anterior");
        pay_games(this, person, -1, 2, offer);
        remove_games(this, person);
    }
    if mark(person) & 4 != 0 {
        pay_games(this, person, -1, 5, offer);
        remove_national_team(this, person);
    }
}

/// Pays and clears the old clauses (next-sale percentage, club and international appearance money) of the player and
/// the exchange players.
///
/// Ret 4: `TRANSFER_MANAGER::settle_previous_transfer_clauses(offer)`, from the completed transfer (0x8c9390): the old
/// clauses of the player (+0xc) and of the exchange persons (list +4, cursor restarted; the original removes the
/// next-sale percentage of the offer's player, not that of the exchange person).
/// Original: `settle_previous_transfer_clauses` (0x8c91e0, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8c91e0 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::settle_previous_transfer_clauses(oferta)`,
/// pela transferência concluída (0x8c9390): as cláusulas antigas do jogador (+0xc) e das pessoas da troca (lista +4,
/// cursor recomeçado; a original tira o percentual da próxima venda do jogador da oferta, não o da pessoa da troca).
pub(crate) unsafe extern "thiscall" fn settle_previous_transfer_clauses(this: usize, offer: usize) {
    let p = staff_records_staff_by_id(read_usize(offer + offer::TARGET_STAFF_ID));
    settle_old_clauses(this, p, p, offer);
    let l = read_usize(offer + offer::EXCHANGE_PLAYERS);
    if l == 0 {
        return;
    }
    transfer_manager_llist_rewind(l);
    let mut x = list_next_item(read_usize(offer + offer::EXCHANGE_PLAYERS));
    while x != 0 {
        settle_old_clauses(this, staff_records_staff_by_id(read_usize(x)), p, offer);
        x = list_next_item(read_usize(offer + offer::EXCHANGE_PLAYERS));
    }
}

/// Person -> main contract (get_contract), stored in G_CONTRACT_LOOKUP_RESULT.
///
/// [pt] O contrato principal da pessoa (0x4cf5a0), guardado em [0xae1080] como a original faz depois de cada chamada.
pub(crate) unsafe fn transfer_manager_main_contract_stored(p: usize) -> usize {
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = contract_of(read_usize(G_CONTRACT_MANAGER), p);
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
    c
}

/// Rewinds a LLIST cursor (+0xa = head +2).
///
/// [pt] Volta a lista ao começo (o cursor +0xa recebe a cabeça +2).
pub(crate) unsafe fn transfer_manager_llist_rewind(l: usize) {
    core::ptr::write_unaligned((l + llist::CURSOR) as *mut usize, read_usize(l + llist::HEAD));
}

/// Writes the person's id and three keys (+0x33 of +4, +8, +0xc) into news slots k..k+3.
///
/// [pt] Os dados da pessoa numa notícia, a partir da vaga `k`: o id e as três chaves (+0x33 de +4, +8 e +0xc).
pub(crate) unsafe fn write_person_news_slots(n: usize, k: i8, p: usize) {
    let write: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    write(n, k, read_usize(p) as u32);
    write(n, k + 1, read_usize(read_usize(p + staff::FIRST_NAME) + 0x33) as u32);
    write(n, k + 2, read_usize(read_usize(p + staff::SECOND_NAME) + 0x33) as u32);
    write(n, k + 3, read_usize(read_usize(p + staff::COMMON_NAME) + 0x33) as u32);
}

/// Advances the offer to club approval: once every player agrees, cancels the rival offers (rescheduled in 20 days),
/// waits for the work permit (18 to 24 days) or passes the offer to the buyer's and the seller's decision.
///
/// Ret 4: `TRANSFER_MANAGER::advance_offer_to_club_approval(offer)`: buyer = club +8; seller = the club of the player's
/// contract (+0xc), none for a type 5 contract (0x4cf5a0 twice, `[0xae1080]`). Outside status 0xa (+0x2c): only with
/// +0x10 = 2 and all the exchange players (+4) with +4 = 2. The other offers for the same player (list +0, indices into
/// vector `[this]`; except +0x20) fall: status 0xf, +0x2e = 3, rescheduled in 0x14 days (0x8c6220), and the bid marks
/// (0x84aeb0 on the list manager `[this+0x856]`) of the buyer and of the owner of each exchange player are removed.
/// Without flag +0x30 & 0xf, with a work permit to request (0x8c7470) and +0x2e = 0: the waiting news (0x8e2a00),
/// status 0xa and rescheduled in 18 + random(7) days. Otherwise status 0xb, the transfer news (0x8da5e0) if one of the
/// clubs is human, and the decision of the computer buyer (0x600a50): with a restriction (0x8cfc90) it refuses
/// (0x8c36b0, +0x2e < 3) or withdraws (0x8bfcb0, which can change the offer); otherwise confirms (0x8c3310). Then the
/// seller: with +0x2f or without a seller it confirms directly; a computer seller refuses with a restriction or
/// confirms.
/// Original: `advance_offer_to_club_approval` (0x8c2f50, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8c2f50 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::advance_offer_to_club_approval(oferta)`:
/// comprador = o clube +8; vendedor = o clube do contrato do jogador (+0xc), nenhum no contrato de tipo 5 (0x4cf5a0
/// duas vezes, [0xae1080]). Fora do status 0xa (+0x2c): só com +0x10 = 2 e todos os jogadores da troca (+4) com +4 = 2.
/// As outras ofertas pelo mesmo jogador (lista +0, índices no vetor [this]; menos a +0x20) caem: status 0xf, +0x2e = 3,
/// reagendadas em 0x14 dias (0x8c6220), e as marcas de lance (0x84aeb0 no gerente de listas [this+0x856]) do comprador
/// e do dono de cada jogador da troca saem. Sem a marca +0x30 & 0xf, com permissão de trabalho a pedir (0x8c7470) e
/// +0x2e = 0: a notícia da espera (0x8e2a00), status 0xa e reagendada em 18 + sorteio(7) dias. Senão status 0xb, a
/// notícia da transferência (0x8da5e0) se um dos clubes é humano, e a decisão do comprador do computador (0x600a50):
/// com restrição (0x8cfc90) recusa (0x8c36b0, +0x2e < 3) ou retira (0x8bfcb0, que pode trocar a oferta); senão
/// confirma (0x8c3310). Depois o vendedor: com +0x2f ou sem vendedor confirma direto; do computador, recusa com
/// restrição ou confirma.
pub(crate) unsafe extern "thiscall" fn advance_offer_to_club_approval(this: usize, offer: usize) {
    let schedule: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_SCHEDULE_OFFER);
    let clear_marks: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_SHORTLIST_MANAGER_CLEAR_BID_MARKERS_FOR_PLAYER);
    // thiscall (ecx = this, the transfer manager; fixed on 31/10/2026: previously stdcall, this went with whatever ecx held and sometimes 0)
    let pede_permissao: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_OFFER_NEEDS_WORK_PERMIT);
    let noticia_da_espera: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_POST_WORK_PERMIT_DELAY_NEWS);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let human: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_NEWS_CLUB_HAS_HUMAN_MANAGER);
    let noticia_da_transferencia: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_POST_TRANSFER_FINALISED_NEWS);
    let human_club: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let restricoes: unsafe extern "thiscall" fn(usize, usize, usize, *mut u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_CHECK_OFFER_RESTRICTIONS);
    let recusar: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_CLUB_REJECTS_OFFER);
    let retirar: unsafe extern "thiscall" fn(usize, *mut usize, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_WITHDRAW_TRANSFER_OFFER);
    let confirmar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_CONFIRM_OFFER_BY_CLUB);
    let buyer = friendly_club_by_id(read_usize(offer + offer::BUYER_CLUB_ID));
    let player = staff_records_staff_by_id(read_usize(offer + offer::TARGET_STAFF_ID));
    let mut seller = offer_contract_club(transfer_manager_main_contract_stored(player), true);
    let c = transfer_manager_main_contract_stored(player);
    if c != 0 && read_u8(c + contract::CONTRACT_TYPE) & 0x3f == 5 {
        seller = 0;
    }
    if read_u8(offer + offer::STATUS) != 0xa {
        if read_u8(offer + offer::TERMS_STATE) != 2 {
            return;
        }
        let swap = read_usize(offer + offer::EXCHANGE_PLAYERS);
        if swap != 0 {
            transfer_manager_llist_rewind(swap);
            loop {
                let x = list_next_item(swap);
                if x == 0 {
                    break;
                }
                if read_u8(x + 4) != 2 {
                    return;
                }
            }
        }
        let rivais = read_usize(offer);
        if rivais != 0 {
            transfer_manager_llist_rewind(rivais);
            let mut x = list_next_item(rivais);
            while x != 0 {
                let other = read_usize(read_usize(this) + read_usize(x) * 4);
                if other != 0 && read_usize(x) != read_usize(offer + offer::SLOT_INDEX) {
                    crate::comparison::branch("cpv_1 ofertas rivais canceladas");
                    write_u8(other + offer::STATUS, 0xf);
                    write_u8(read_usize(read_usize(this) + read_usize(x) * 4) + 0x2e, 3);
                    schedule(this, read_usize(read_usize(this) + read_usize(x) * 4), 0x14, 0); // cpv1: the rivals in 20 days
                    let o = read_usize(read_usize(this) + read_usize(x) * 4);
                    let p = staff_records_staff_by_id(read_usize(o + offer::TARGET_STAFF_ID));
                    let owner = offer_contract_club(transfer_manager_main_contract_stored(p), false);
                    clear_marks(read_usize(this + transfer_manager_fields::SHORTLIST_MANAGER), friendly_club_by_id(read_usize(o + offer::BUYER_CLUB_ID)), p);
                    let l = read_usize(o + offer::EXCHANGE_PLAYERS);
                    if l != 0 {
                        transfer_manager_llist_rewind(l);
                        loop {
                            let y = list_next_item(l);
                            if y == 0 {
                                break;
                            }
                            clear_marks(read_usize(this + transfer_manager_fields::SHORTLIST_MANAGER), owner, staff_records_staff_by_id(read_usize(y)));
                        }
                    }
                }
                x = list_next_item(rivais);
            }
        }
        if read_u8(offer + offer::COMPLETION_TIMING) & 0xf == 0 && pede_permissao(this, offer) != 0 && read_u8(offer + offer::RESPONSE_ROUND) == 0 {
            noticia_da_espera(this, offer);
            write_u8(offer + offer::STATUS, 0xa);
            let r = sorteio(7);
            crate::comparison::branch("cpv_1 permissao de trabalho");
            // the original adds 0x12 only to the low byte of the random eax (0..6: above, zero)
            let days = (r as u32 & !0xff) | (r as u8).wrapping_add(0x12) as u32; // cpv1: the wait for the permit
            schedule(this, offer, days, 0);
            return;
        }
    }
    write_u8(offer + offer::STATUS, 0xb);
    if human(read_usize(G_NEWS), buyer) != 0 || (seller != 0 && human(read_usize(G_NEWS), seller) != 0) {
        noticia_da_transferencia(this, offer);
    }
    let mut slot = offer;
    // simplification: the original passes a short* to a meaningless local dword and pushes the whole dword; the callees read the
    // word. The copy starts at zero.
    let mut reason: u32 = 0;
    if human_club(buyer, 0, 1, 0, 0) == 0 {
        if restricoes(this, offer, buyer, &mut reason, 1) as u8 == 1 {
            if read_u8(offer + offer::RESPONSE_ROUND) < 3 {
                recusar(this, offer, buyer, reason);
            } else {
                retirar(this, &mut slot, 0, reason);
            }
        } else {
            confirmar(this, offer, buyer);
        }
    }
    // reread from the slot, like the original ([esp+0x20], which 0x8bfcb0 may change)
    let o = slot;
    if read_u8(o + offer::PRE_CONTRACT) != 0 || seller == 0 {
        return confirmar(this, o, seller);
    }
    if human_club(seller, 0, 1, 0, 0) != 0 {
        return;
    }
    if restricoes(this, slot, seller, &mut reason, 1) as u8 == 1 {
        return recusar(this, slot, seller, 0);
    }
    crate::comparison::branch("cpv_1 confirmada pelo vendedor");
    confirmar(this, slot, seller); // cpv1: o vendedor confirma
}

/// Sends the 0xfa3 news of an accepted offer to the human clubs involved.
///
/// Ret 4; SEH frame without a destructor: `TRANSFER_MANAGER::post_offer_accepted_news(offer)`: the player's owner
/// (+0xc; contract 0x4cf5a0, none for type 5), the recipients (0x8d1130 with 0x2d, up to 30), news 0xfa3 (local): 0..3
/// the player, 4 = the owner's id (or −1), 5 = +8 (buyer), 6 = +4, 7 = +0x20, 8 = +0x24, 9 = +0x1f; delivered to each
/// recipient (0x7861e0).
/// Original: `post_offer_accepted_news` (0x8d76d0, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8d76d0 } (thiscall, ret 4, transfer_manager.cpp, quadro SEH sem destrutor): `TRANSFER_MANAGER::
/// post_offer_accepted_news(oferta)`: o dono do jogador (+0xc; contrato 0x4cf5a0, nenhum no tipo 5), os destinatários
/// (0x8d1130 com 0x2d, até 30), a notícia 0xfa3 (local): 0..3 o jogador, 4 = o id do dono (ou −1), 5 = +8 (comprador),
/// 6 = +4, 7 = +0x20, 8 = +0x24, 9 = +0x1f; entregue a cada destinatário (0x7861e0).
pub(crate) unsafe extern "thiscall" fn post_offer_accepted_news(this: usize, offer: usize) {
    let recipients: unsafe extern "thiscall" fn(usize, usize, u32, *mut usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_COLLECT_OFFER_NEWS_RECIPIENTS);
    let construct: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let write: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let deliver: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    crate::comparison::branch("cpv_1 noticia da oferta aceita");
    let p = staff_records_staff_by_id(read_usize(offer + offer::TARGET_STAFF_ID));
    let owner = offer_contract_club(transfer_manager_main_contract_stored(p), true);
    let mut clubs = [0usize; 30];
    let q = recipients(this, offer, 0x2d, clubs.as_mut_ptr(), 0x1e) as i8;
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    construct(n, 0xfa3, 0);
    write_person_news_slots(n, 0, p);
    write(n, 4, if owner != 0 { read_usize(owner) as u32 } else { u32::MAX });
    write(n, 5, read_usize(offer + offer::BUYER_CLUB_ID) as u32);
    write(n, 6, read_usize(offer + offer::EXCHANGE_PLAYERS) as u32);
    write(n, 7, read_usize(offer + offer::SLOT_INDEX) as u32);
    write(n, 8, read_usize(offer + offer::SERIAL) as u32);
    write(n, 9, read_u8(offer + offer::OFFER_TYPE) as i8 as i32 as u32);
    for &c in clubs.iter().take(if q > 0 { q as usize } else { 0 }) {
        deliver(read_usize(G_NEWS), n, c);
    }
}

/// Sends the 0xfb5 news of a loan offer to the human clubs involved; without a contract, the error box.
///
/// Ret 8; SEH frame without a destructor: `TRANSFER_MANAGER::post_loan_offer_news(offer, terms)`: the recipients
/// (0x8d1130 with 0x2e, up to 30); with none, nothing. News 0xfb5 (local): 0..3 the player (+0xc), 4 = +8, 5 = the id
/// of the contract club (none for type 5: the original reads address 0), 6 = +0x20, 7 = +0x24, 8 = terms +0xc, 9 =
/// +0x2c, 10 = terms +0x2d (signed word), 11 = terms +0x2f (word); without a contract, the "Error" box (0x5e4c) and
/// nothing; otherwise 12 = contract +0xc and the delivery to each one.
/// Original: `post_loan_offer_news` (0x8e1780, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8e1780 } (thiscall, ret 8, transfer_manager.cpp, quadro SEH sem destrutor): `TRANSFER_MANAGER::
/// post_loan_offer_news(oferta, termos)`: os destinatários (0x8d1130 com 0x2e, até 30); sem nenhum, nada. A notícia
/// 0xfb5 (local): 0..3 o jogador (+0xc), 4 = +8, 5 = o id do clube do contrato (nenhum no tipo 5: a original lê o
/// endereço 0), 6 = +0x20, 7 = +0x24, 8 = termos +0xc, 9 = +0x2c, 10 = termos +0x2d (palavra com sinal), 11 = termos
/// +0x2f (palavra); sem contrato, a caixa "Error" (0x5e4c) e nada; senão 12 = contrato +0xc e a entrega a cada um.
pub(crate) unsafe extern "thiscall" fn post_loan_offer_news(this: usize, offer: usize, terms: usize) {
    let recipients: unsafe extern "thiscall" fn(usize, usize, u32, *mut usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_COLLECT_OFFER_NEWS_RECIPIENTS);
    let construct: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let write: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let deliver: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    let p = staff_records_staff_by_id(read_usize(offer + offer::TARGET_STAFF_ID));
    let mut clubs = [0usize; 30];
    let q = recipients(this, offer, 0x2e, clubs.as_mut_ptr(), 0x1e) as i8;
    if q == 0 {
        return;
    }
    crate::comparison::branch("cpv_1 noticia do emprestimo");
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    construct(n, 0xfb5, 0);
    write_person_news_slots(n, 0, p);
    write(n, 4, read_usize(offer + offer::BUYER_CLUB_ID) as u32);
    // simplification: without a club (type 5 contract) the original reads address 0 and crashes; the copy does the same.
    let club = offer_contract_club(transfer_manager_main_contract_stored(p), true);
    write(n, 5, read_usize(club) as u32);
    write(n, 6, read_usize(offer + offer::SLOT_INDEX) as u32);
    write(n, 7, read_usize(offer + offer::SERIAL) as u32);
    write(n, 8, read_usize(terms + staff_contract::WAGE) as u32);
    write(n, 9, read_u8(offer + offer::STATUS) as i8 as i32 as u32);
    write(n, 10, core::ptr::read_unaligned((terms + staff_contract::CONTRACT_EXPIRES) as *const i16) as i32 as u32);
    write(n, 11, core::ptr::read_unaligned((terms + staff_contract::CONTRACT_EXPIRES_YEAR) as *const u16) as u32);
    let c = transfer_manager_main_contract_stored(p);
    if c == 0 {
        return show_error_at(0xa8d514, 0x5e4c);
    }
    write(n, 12, read_usize(c + contract::WAGE) as u32);
    for &d in clubs.iter().take(if q > 0 { q as usize } else { 0 }) {
        deliver(read_usize(G_NEWS), n, d);
    }
}

// simplification: reschedules, cancels rivals, draws and decides for the clubs (the calls touch the offers and allocate): does not run
// twice. Original only; the copy carries the branches and is proven by the career and by the mutations.

// simplification: the news item goes to the managers’ inboxes (allocates): original only; the copy takes the branch.

// simplification: the news item goes to the managers’ inboxes (allocates): original only; the copy takes the branch.

/// Stores the offer terms in a new negotiation (the array grows by 50; the next id) and schedules the offer in 2 days,
/// or in 14 with news for a human buyer.
///
/// Ret 0xc; SEH frame only for the `new`s: `TRANSFER_MANAGER::add_transfer_negotiation(offer, terms, mark)`: with
/// `mark`, +0x2d = 1 in both and the player marked under offer (0x84b580, with list +0). Without a slot (+0x28 = −1):
/// the negotiation vector (+0xc, capacity +0x10, count +0x14) grows by 50 when full (0x945667 zeroing the new ones;
/// 0x945501 without a vector), the first empty slot goes to +0x28 and the offer to status 1 (with no empty slot: the
/// "Error" box 0x60f and nothing). The slot's old negotiation is destroyed (0x8e5710, 0x944c94), or the count goes up;
/// a new one (0x41 bytes, 0x944cf1 and 0x8e5680) receives the terms field by field (the date +0x12 through 0x417c10; of
/// +0x30 and +0x31 only the low 4 bits), +0x20 of the offer and the next id (+0x85a); list +4 of the terms is copied
/// into a new one (0xe bytes, 0x68d020, 0x68d2f0). A human buyer (terms +8): the negotiation news (0x8e4610 for a loan
/// +0x1f = 1, otherwise 0x8d6bf0) and the offer scheduled in 14 days; otherwise in 2 (0x8c6220).
/// Original: `add_transfer_negotiation` (0x8be9d0, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8be9d0 } (thiscall, ret 0xc, transfer_manager.cpp, quadro SEH só dos `new`): `TRANSFER_MANAGER::
/// add_transfer_negotiation(oferta, termos, marcar)`: com `marcar`, +0x2d = 1 nas duas e o jogador marcado sob oferta
/// (0x84b580, com a lista +0). Sem vaga (+0x28 = −1): o vetor das negociações (+0xc, capacidade +0x10, conta +0x14)
/// cresce de 50 quando cheio (0x945667 zerando as novas; 0x945501 sem vetor), a primeira vaga vazia vai para +0x28 e a
/// oferta para o status 1 (sem vaga vazia: a caixa "Error" 0x60f e nada). A negociação antiga da vaga é destruída
/// (0x8e5710, 0x944c94), ou a conta sobe; uma nova (0x41 bytes, 0x944cf1 e 0x8e5680) recebe os termos campo a campo
/// (a data +0x12 por 0x417c10; de +0x30 e +0x31 só os 4 bits baixos), +0x20 da oferta e o id seguinte (+0x85a); a
/// lista +4 dos termos é copiada numa nova (0xe bytes, 0x68d020, 0x68d2f0). Comprador (termos +8) humano: a notícia da
/// negociação (0x8e4610 no empréstimo +0x1f = 1, senão 0x8d6bf0) e a oferta agendada em 14 dias; senão em 2 (0x8c6220).
pub(crate) unsafe extern "thiscall" fn add_transfer_negotiation(this: usize, offer: usize, terms: usize, mark: u32) {
    let mark_player_under_offer: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(FN_SHORTLIST_MANAGER_MARK_PLAYER_UNDER_OFFER);
    let realloc: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(FN_CRT_REALLOC);
    let alocar_zerado: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(FN_CRT_CALLOC);
    let destroy: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_TRANSFER_OFFER_DESTRUCTOR_TRANSFER_OFFER);
    let free: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_OPERATOR_DELETE);
    let novo: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let construct: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_TRANSFER_OFFER_TRANSFER_OFFER);
    let atribuir_data: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let new_list: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let copy_list: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_LLIST_COPY_FROM);
    let human: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_NEWS_CLUB_HAS_HUMAN_MANAGER);
    let news_of_loan: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_POST_LOAN_NEGOTIATION_NEWS);
    let noticia_da_negociacao: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_POST_OFFER_NEGOTIATION_NEWS);
    let schedule: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_SCHEDULE_OFFER);
    if mark != 0 {
        write_u8(offer + offer::PLAYER_AWARE, 1);
        write_u8(terms + offer::PLAYER_AWARE, 1);
        mark_player_under_offer(read_usize(this + transfer_manager_fields::SHORTLIST_MANAGER), staff_records_staff_by_id(read_usize(offer + offer::TARGET_STAFF_ID)), friendly_club_by_id(read_usize(offer + offer::BUYER_CLUB_ID)), read_usize(offer));
    }
    let mut k = read_usize(offer + offer::NEGOTIATION_INDEX);
    if k == usize::MAX {
        if read_usize(this + transfer_manager_fields::NEGOTIATION_COUNT) == read_usize(this + transfer_manager_fields::NEGOTIATION_CAPACITY) {
            let v = read_usize(this + transfer_manager_fields::NEGOTIATIONS);
            let cap = read_usize(this + transfer_manager_fields::NEGOTIATION_CAPACITY).wrapping_add(0x32);
            write_usize(this + transfer_manager_fields::NEGOTIATION_CAPACITY, cap);
            if v != 0 {
                let nv = realloc(v, cap * 4);
                write_usize(this + transfer_manager_fields::NEGOTIATIONS, nv);
                core::ptr::write_bytes((nv + read_usize(this + transfer_manager_fields::NEGOTIATION_CAPACITY) * 4 - 0xc8) as *mut u32, 0, 0x32);
            } else {
                write_usize(this + transfer_manager_fields::NEGOTIATIONS, alocar_zerado(cap, 4));
            }
        }
        let cap = read_usize(this + transfer_manager_fields::NEGOTIATION_CAPACITY) as i32;
        let v = read_usize(this + transfer_manager_fields::NEGOTIATIONS);
        match (0..cap.max(0) as usize).find(|&j| read_usize(v + 4 * j) == 0) {
            Some(j) => k = j,
            None => return show_error_at(0xa8d514, 0x60f),
        }
        crate::comparison::branch("cpv_4 vaga nova");
        write_usize(offer + offer::NEGOTIATION_INDEX, k);
        write_u8(offer + offer::STATUS, 1); // cpv4: the offer under negotiation
    }
    let old = read_usize(read_usize(this + transfer_manager_fields::NEGOTIATIONS) + k * 4);
    if old != 0 {
        destroy(old);
        free(old);
    } else {
        write_usize(this + transfer_manager_fields::NEGOTIATION_COUNT, read_usize(this + transfer_manager_fields::NEGOTIATION_COUNT).wrapping_add(1));
    }
    let mut new = novo(0x41);
    new = if new != 0 { construct(new) } else { 0 };
    write_usize(read_usize(this + transfer_manager_fields::NEGOTIATIONS) + k * 4, new);
    let neg = read_usize(read_usize(this + transfer_manager_fields::NEGOTIATIONS) + k * 4);
    write_usize(neg, read_usize(terms));
    write_usize(neg + offer::EXCHANGE_PLAYERS, read_usize(terms + offer::EXCHANGE_PLAYERS));
    write_usize(neg + offer::BUYER_CLUB_ID, read_usize(terms + offer::BUYER_CLUB_ID));
    write_usize(neg + offer::TARGET_STAFF_ID, read_usize(terms + offer::TARGET_STAFF_ID));
    write_u8(neg + offer::TERMS_STATE, read_u8(terms + offer::TERMS_STATE));
    let b11 = read_u8(terms + offer::JOB_TYPE);
    let mut temporaria = [0u32; 2];
    write_u8(neg + offer::JOB_TYPE, b11);
    atribuir_data(neg + offer::DUE_DATE, temporaria.as_mut_ptr() as usize, terms + offer::DUE_DATE);
    write_u8(neg + offer::DUE_SLICE, read_u8(terms + offer::DUE_SLICE));
    write_usize(neg + offer::FEE, read_usize(terms + offer::FEE)); // cpv4: the price of the terms
    write_u8(neg + offer::OFFER_TYPE, read_u8(terms + offer::OFFER_TYPE));
    write_usize(neg + offer::SLOT_INDEX, read_usize(terms + offer::SLOT_INDEX));
    write_usize(neg + offer::SERIAL, read_usize(terms + offer::SERIAL));
    write_usize(neg + offer::NEGOTIATION_INDEX, read_usize(terms + offer::NEGOTIATION_INDEX));
    for o in 0x2c..0x30 {
        write_u8(neg + o, read_u8(terms + o));
    }
    write_u8(neg + offer::COMPLETION_TIMING, (read_u8(neg + offer::COMPLETION_TIMING) & 0xf0) | (read_u8(terms + offer::COMPLETION_TIMING) & 0xf));
    write_usize(neg + offer::OFFER_KIND, (read_usize(neg + offer::OFFER_KIND) & !0xf) | (read_usize(terms + offer::OFFER_KIND) & 0xf));
    write_usize(neg + offer::APPEARANCE_FEE, read_usize(terms + offer::APPEARANCE_FEE));
    write_u8(neg + offer::APPEARANCE_GAMES, read_u8(terms + offer::APPEARANCE_GAMES));
    write_usize(neg + offer::INTERNATIONAL_FEE, read_usize(terms + offer::INTERNATIONAL_FEE));
    write_u8(neg + offer::INTERNATIONAL_GAMES, read_u8(terms + offer::INTERNATIONAL_GAMES));
    write_u8(neg + offer::SELL_ON_PERCENT, read_u8(terms + offer::SELL_ON_PERCENT));
    write_u8(neg + offer::INSTALMENTS, read_u8(terms + offer::INSTALMENTS));
    let neg = || read_usize(read_usize(this + transfer_manager_fields::NEGOTIATIONS) + k * 4);
    write_usize(neg() + 0x20, read_usize(offer + offer::SLOT_INDEX));
    write_usize(neg() + 0x24, read_usize(this + transfer_manager_fields::NEXT_SERIAL));
    write_usize(this + transfer_manager_fields::NEXT_SERIAL, read_usize(this + transfer_manager_fields::NEXT_SERIAL).wrapping_add(1));
    if read_usize(terms + offer::EXCHANGE_PLAYERS) != 0 {
        let mut l = novo(0xe);
        l = if l != 0 { new_list(l) } else { 0 };
        write_usize(neg() + 4, l);
        copy_list(read_usize(neg() + 4), read_usize(terms + offer::EXCHANGE_PLAYERS));
    }
    if human(read_usize(G_NEWS), friendly_club_by_id(read_usize(terms + offer::BUYER_CLUB_ID))) != 0 {
        if read_u8(offer + offer::OFFER_TYPE) == 1 {
            news_of_loan(this, neg(), offer);
        } else {
            noticia_da_negociacao(this, neg(), offer);
        }
        schedule(this, offer, 0xe, 0);
    } else {
        crate::comparison::branch("cpv_4 negociacao agendada");
        schedule(this, offer, 2, 0); // cpv4: the offer in 2 days
    }
}

/// Sends the 0xfae news of the player refusing the move, with the reason and both clubs' managers.
///
/// Ret 8; SEH frame without a destructor: `TRANSFER_MANAGER::post_player_rejects_move_news(offer, reason)`: the
/// player's owner (+0xc; contract 0x4cf5a0, none for type 5), the recipients (0x8d1130 with 0x2f, up to 30), news 0xfae
/// (local): 0..3 the player, 4 = the owner's id (or −1), 5 = +8, 6 = +0x1f, 7 = the reason, 8 = the owner's manager, 9
/// = the buyer's manager (+8 > −1); to each recipient.
/// Original: `post_player_rejects_move_news` (0x8d8310, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8d8310 } (thiscall, ret 8, transfer_manager.cpp, quadro SEH sem destrutor): `TRANSFER_MANAGER::
/// post_player_rejects_move_news(oferta, motivo)`: o dono do jogador (+0xc; contrato 0x4cf5a0, nenhum no tipo 5), os
/// destinatários (0x8d1130 com 0x2f, até 30), a notícia 0xfae (local): 0..3 o jogador, 4 = o id do dono (ou −1), 5 =
/// +8, 6 = +0x1f, 7 = o motivo, 8 = o técnico do dono, 9 = o técnico do comprador (+8 > −1); a cada destinatário.
pub(crate) unsafe extern "thiscall" fn post_player_rejects_move_news(this: usize, offer: usize, reason: u32) {
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let recipients: unsafe extern "thiscall" fn(usize, usize, u32, *mut usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_COLLECT_OFFER_NEWS_RECIPIENTS);
    let construct: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let write: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let deliver: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    crate::comparison::branch("cpv_4 recusa em se mudar");
    let p = staff_records_staff_by_id(read_usize(offer + offer::TARGET_STAFF_ID));
    let c = contract_of(read_usize(G_CONTRACT_MANAGER), p);
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
    let owner = if c != 0 && read_u8(c + contract::CONTRACT_TYPE) & 0x3f != 5 { friendly_club_by_id(read_usize(c + contract::CLUB)) } else { 0 };
    let mut clubs = [0usize; 30];
    let q = recipients(this, offer, 0x2f, clubs.as_mut_ptr(), 0x1e) as i8;
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    construct(n, 0xfae, 0);
    write(n, 0, read_usize(p) as u32);
    write(n, 1, read_usize(read_usize(p + staff::FIRST_NAME) + 0x33) as u32);
    write(n, 2, read_usize(read_usize(p + staff::SECOND_NAME) + 0x33) as u32);
    write(n, 3, read_usize(read_usize(p + staff::COMMON_NAME) + 0x33) as u32);
    write(n, 4, if owner != 0 { read_usize(owner) as u32 } else { u32::MAX });
    write(n, 5, read_usize(offer + offer::BUYER_CLUB_ID) as u32);
    write(n, 6, read_u8(offer + offer::OFFER_TYPE) as i8 as i32 as u32);
    write(n, 7, reason);
    if owner != 0 {
        let t = read_usize(owner + club::MANAGER);
        if t != 0 {
            write(n, 8, read_usize(t) as u32);
        }
    }
    let buyer = read_usize(offer + offer::BUYER_CLUB_ID) as i32;
    if buyer > -1 {
        let t = read_usize(friendly_club_by_id(buyer as usize) + 0xcf);
        if t != 0 {
            write(n, 9, read_usize(t) as u32);
        }
    }
    for &d in clubs.iter().take(if q > 0 { q as usize } else { 0 }) {
        deliver(read_usize(G_NEWS), n, d);
    }
}

// simplification: allocates and frees negotiations and lists, schedules the offer and sends news: does not run twice. Original only;
// the copy carries the branches and is proven by the career and by the mutations.

// simplification: rolls random numbers and the news goes to the inboxes (allocates): original only; the copy takes the branch.

// simplification: the news item goes to the managers’ inboxes (allocates): original only; the copy takes the branch.

/// Initialises the work permits: marks 0x3fff/0x1fff by international caps; for a player needing a permit (0x8c7560),
/// by the earliest year in the club's nation (career 0x7c3e40): under 5 years, the permit (0x8c7070); otherwise the
/// second nationality (+0x1e); sorts the permits.
///
/// Ret 0; SEH frame only for the local date: `TRANSFER_MANAGER::init_work_permits()`, from creating the game
/// (0x8bba30). For each person: with more than 13 caps for the national side (+0x22), 0x3fff in word +3 of the 8-byte
/// record (`[this+0x86e]` + i × 8); with more than 5, 0x1fff. A player (+0x3d = 0xb) at a club (+0x39) that needs a
/// work permit (0x8c7560 without ignoring the given one): the earliest year in the club's nation from the career
/// (0x7c3e40, a group of 6 entries of 0x2b bytes per season; stops at the first club of another nation), starting from
/// the year of +0x40. Fewer than 5 years up to today's year (`[0xae2c92]`): the permit (0x8c7070, without sorting;
/// before that the original moves date +0x3e forward 52 weeks at a time up to date `[0xdd6ec8]`, without using the
/// result); otherwise the second nationality (+0x1e) becomes the club's nation. At the end sorts the permits (qsort
/// 0x945333 with 0x8e55c0).
/// Original: `TRANSFER_MANAGER::init_work_permits` (0x8c6e60, transfer_manager.cpp, thiscall, ret 0).
///
/// [pt] { 0x8c6e60 } (thiscall, ret 0, transfer_manager.cpp, quadro SEH só da data local): `TRANSFER_MANAGER::
/// init_work_permits()`: pela criação do jogo (0x8bba30). Para cada pessoa: com mais de 13 jogos pela seleção (+0x22),
/// 0x3fff na palavra +3 do registro de 8 bytes ([this+0x86e] + i × 8); com mais de 5, 0x1fff. Jogador (+0x3d = 0xb) de
/// clube (+0x39) que precisa de permissão de trabalho (0x8c7560 sem ignorar a dada): o ano mais antigo na nação do
/// clube pela carreira (0x7c3e40, um grupo de 6 entradas de 0x2b bytes por temporada; para no primeiro clube de outra
/// nação), começando pelo ano de +0x40. Menos de 5 anos até o ano de hoje ([0xae2c92]): a permissão (0x8c7070, sem
/// ordenar; antes a original leva a data +0x3e de 52 em 52 semanas até a data [0xdd6ec8], sem usar o resultado); senão a
/// segunda nacionalidade (+0x1e) vira a nação do clube. No fim ordena as permissões (qsort 0x945333 com 0x8e55c0).
pub(crate) unsafe extern "thiscall" fn init_work_permits(this: usize) {
    let data_default: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    let copy: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let more_weeks: unsafe extern "thiscall" fn(usize, usize, i32) -> usize = core::mem::transmute(FN_DATE_ADD_WEEKS);
    let precisa: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_PLAYER_NEEDS_WORK_PERMIT);
    let carreira: unsafe extern "thiscall" fn(usize, usize, *mut i32) -> usize = core::mem::transmute(FN_PLAYERSTATS_BUILD_CAREER_HISTORY);
    let free: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let give: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_ADD_WORK_PERMIT);
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    let mut d = [0u8; 8];
    data_default(d.as_mut_ptr() as usize);
    let mut i = 0usize;
    let mut p = read_usize(G_STAFF_TABLE);
    while (i as i32) < read_usize(G_STAFF_COUNT) as i32 {
        let games = read_u8(p + staff::INT_APPS);
        let mark = (read_usize(this + transfer_manager_fields::PERSON_FLAGS).wrapping_add(i.wrapping_mul(8)) + 3) as *mut u16;
        if games > 0xd {
            core::ptr::write_unaligned(mark, 0x3fff);
        } else if games > 5 {
            core::ptr::write_unaligned(mark, 0x1fff);
        }
        let club = read_usize(p + staff::CLUB_JOB);
        if club != 0 && read_u8(p + staff::JOB_FOR_CLUB) == 0xb && precisa(this, p, club, 0) != 0 {
            let mut year = read_u16(p + 0x40);
            let mut count = 0i32;
            let t = carreira(read_usize(G_PLAYER_STATS), p, &mut count);
            let (mut k, mut e) = (0i32, t + 2);
            while k < count {
                let id = read_usize(e) as i32;
                if id >= 0 && id < read_usize(G_CLUB_COUNT) as i32 {
                    let c = read_usize(G_CLUB_TABLE).wrapping_add((id as usize).wrapping_mul(club::SIZE));
                    if read_usize(c + club::NATION) != read_usize(read_usize(p + staff::CLUB_JOB) + 0x53) {
                        break; // club from another nation
                    }
                    let a = read_u16(e - 2);
                    if a < year { // cps2: o ano mais antigo
                        crate::comparison::branch("cps_2 ano mais antigo");
                        year = a;
                    }
                }
                k += 6;
                e += 0x102;
            }
            if t != 0 {
                free(t);
            }
            if (read_u16(G_CURRENT_DATE_YEAR) as i32) - (year as i32) < 5 { // cps2: less than five years in the nation
                // date +0x3e advanced 52 weeks at a time up to date [0xdd6ec8] (the original does not use the result)
                let (mut t1, mut t2, mut t3) = ([0u8; 8], [0u8; 8], [0u8; 8]);
                let dp = d.as_mut_ptr() as usize;
                copy(dp, t1.as_mut_ptr() as usize, p + staff::DATE_JOINED_CLUB);
                loop {
                    let (ah, dh) = (read_u16(G_GAME_START_DATE_YEAR), read_u16(dp + cm_date::YEAR));
                    if !(ah > dh || (ah == dh && (read_u16(G_GAME_START_DATE) as i16) > (read_u16(dp) as i16))) {
                        break;
                    }
                    let r = more_weeks(dp, t2.as_mut_ptr() as usize, 0x34);
                    copy(dp, t3.as_mut_ptr() as usize, r);
                }
                crate::comparison::branch("cps_2 permissao pela carreira");
                // simplification: the original pushes the year register with the high 16 bits of a pointer; 0x8c7070
                // writes only the low word
                give(this, p, read_usize(read_usize(p + staff::CLUB_JOB) + 0x53), year as u32, 1, 0);
            } else {
                crate::comparison::branch("cps_2 segunda nacionalidade");
                core::ptr::write_unaligned((p + staff::SECOND_NATION) as *mut usize, read_usize(read_usize(p + staff::CLUB_JOB) + 0x53)); // cps2: a segunda nacionalidade
            }
        }
        i += 1;
        p += 0x6e;
    }
    qsort(read_usize(this + transfer_manager_fields::WORK_PERMITS), read_usize(this + transfer_manager_fields::WORK_PERMIT_COUNT) as u32, 0xe, 0x8e55c0);
}

/// Returns the clause count: type 4, international caps since the deal (+0x22 minus byte +7 of the person record);
/// otherwise club appearances (+0x3b of the contract).
///
/// Ret 8: `TRANSFER_MANAGER::get_clause_count(person, type)`: in type 4, the caps for the national side (+0x22) minus
/// those at the time of the deal (byte +7 of the 8-byte record `[this+0x86e]` + id × 8); otherwise the club appearances
/// (+0x3b) of the contract (0x4cf5a0, requested twice as in the original), or 0 without a contract.
/// Original: `TRANSFER_MANAGER::get_clause_count` (0x8c3e50, transfer_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x8c3e50 } (thiscall, ret 8, transfer_manager.cpp): `TRANSFER_MANAGER::get_clause_count(pessoa, tipo)`: no tipo 4,
/// os jogos pela seleção (+0x22) menos os da época do acordo (byte +7 do registro de 8 bytes [this+0x86e] + id × 8);
/// senão os jogos pelo clube (+0x3b) do contrato (0x4cf5a0, pedido duas vezes como na original), ou 0 sem contrato.
pub(crate) unsafe extern "thiscall" fn get_clause_count(this: usize, person: usize, kind: u32) -> u16 {
    if kind as u8 == 4 {
        let r = read_usize(this + transfer_manager_fields::PERSON_FLAGS).wrapping_add(read_usize(person).wrapping_mul(8));
        return (read_u8(person + staff::INT_APPS) as u16).wrapping_sub(read_u8(r + person_transfer_record::INTERNATIONAL_APPS_AT_DEAL) as u16);
    }
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    if contract_of(read_usize(G_CONTRACT_MANAGER), person) == 0 {
        return 0;
    }
    read_u16(contract_of(read_usize(G_CONTRACT_MANAGER), person) + 0x3b)
}

/// With the person's bit 2, searches the appearance money array +0x88c; when not found, clears the bit.
///
/// Ret 4: `TRANSFER_MANAGER::find_first_team_appearance_money(person)`: with bit 2 of the 8-byte record (`[this+0x86e]`
/// + id × 8), searches for the person's appearance bonus in vector `[this+0x88c]` (count +0x894; binary search 0x946464
/// with 0x8b4e00, 13-byte records) and returns it; not found, clears the bit (adds 0xfe to the byte) and returns 0.
/// Original: `TRANSFER_MANAGER::find_first_team_appearance_money` (0x8d0ea0, transfer_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x8d0ea0 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::find_first_team_appearance_money(pessoa)`:
/// com o bit 2 do registro de 8 bytes ([this+0x86e] + id × 8), busca o prêmio por jogo da pessoa no vetor [this+0x88c]
/// (conta +0x894; busca binária 0x946464 com 0x8b4e00, registros de 13 bytes) e o devolve; sem achar, tira o bit (soma
/// 0xfe ao byte) e devolve 0.
pub(crate) unsafe extern "thiscall" fn find_first_team_appearance_money(this: usize, person: usize) -> usize {
    let id = read_usize(person);
    if read_u8(read_usize(this + transfer_manager_fields::PERSON_FLAGS).wrapping_add(id.wrapping_mul(8))) & 2 == 0 {
        return 0;
    }
    let bsearch: unsafe extern "C" fn(*const i32, usize, usize, u32, usize) -> usize = core::mem::transmute(FN_CRT_BSEARCH);
    let key = id as i32;
    let r = bsearch(&key, read_usize(this + transfer_manager_fields::APPEARANCE_MONEY), read_usize(this + transfer_manager_fields::APPEARANCE_MONEY_COUNT), 0xd, 0x8b4e00);
    if r != 0 {
        return r;
    }
    let m = read_usize(this + transfer_manager_fields::PERSON_FLAGS).wrapping_add(read_usize(person).wrapping_mul(8)) as *mut u8;
    *m = (*m).wrapping_add(0xfe);
    0
}

// simplification: grows the permissions array (0x8c7070) and writes to every person: original only; the copy takes the branches
// (runs once, when the game is created).

/// Processes the free transfer queue: for each queued player (+0x92c), at a human club the board may block it
/// (0x696940); a random draw adds the manager to the disliked staff (0x547af0); the records (0x86d800), the cancelled
/// offers (0x8df9b0) and the release with the compensation (0x8c1af0); clears the queue.
///
/// Ret 0; SEH frame for the two local LLISTs 0x68d020/0x68d040: `TRANSFER_MANAGER::process_free_transfer_queue()`, from
/// the daily transfer update (0x8c5880). Goes through a copy of the queue header (+0x92c, 14 bytes, with the cursor at
/// the start); for each item (+0 person id, +4 reason, +5 compensation) with a contract (0x4cf5a0, stored in
/// `[0xae1080]`) and a club: at a human club (0x7875e0) without a reason, the board may block it (0x696940; blocking,
/// 0x6a0990 and the next one); otherwise the club joins the rejected clubs (0x547e70) and the manager (+0xcf) the
/// disliked staff (0x547af0), and the board records it (0x697430). Without a reason, a random draw by +0x5b against
/// another of 20 (or liking the club, 0x547d60) puts the manager in the disliked staff. Then the release records
/// (0x86d800), the cancelled offers (0x8df9b0) and the release (0x8c1af0 with reason and compensation). At the end the
/// copy becomes the empty list, the queue is cleared (0x68d080) and the two locals are destroyed.
/// Original: `TRANSFER_MANAGER::process_free_transfer_queue` (0x8c18c0, transfer_manager.cpp, thiscall, ret 0).
///
/// [pt] { 0x8c18c0 } (thiscall, ret 0, transfer_manager.cpp, quadro SEH das duas LLIST locais 0x68d020/0x68d040):
/// `TRANSFER_MANAGER::process_free_transfer_queue()`: pelo dia a dia das transferências (0x8c5880). Percorre uma cópia
/// do cabeçalho da fila (+0x92c, 14 bytes, com o cursor no começo); para cada item (+0 id da pessoa, +4 motivo, +5
/// compensação) com contrato (0x4cf5a0, guardado em [0xae1080]) e clube: no clube humano (0x7875e0) sem motivo, a
/// diretoria pode barrar (0x696940; barrando, 0x6a0990 e o próximo); senão o clube entra nos rejeitados (0x547e70) e o
/// técnico (+0xcf) nos desafetos (0x547af0), e a diretoria registra (0x697430). Sem motivo, um sorteio pelo +0x5b contra
/// outro de 20 (ou gostar do clube, 0x547d60) põe o técnico nos desafetos. Depois os registros da liberação (0x86d800),
/// as ofertas canceladas (0x8df9b0) e a liberação (0x8c1af0 com motivo e compensação). No fim a cópia vira a lista
/// vazia, a fila é limpa (0x68d080) e as duas locais são destruídas.
pub(crate) unsafe extern "thiscall" fn process_free_transfer_queue(this: usize) {
    let new: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let clear: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_CLEAR);
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let human: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_NEWS_CLUB_HAS_HUMAN_MANAGER);
    let bar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_MANAGER_MANAGER_CHECK_BOARD_BLOCKS_RELEASE);
    let barrado: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(FN_MANAGER_MANAGER_POST_BOARD_RELEASE_REACTION_NEWS);
    let rejeitado: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(FN_STAFF_ADD_DISLIKED_CLUB);
    let desafeto: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(FN_STAFF_ADD_DISLIKED_STAFF);
    let board: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_MANAGER_MANAGER_APPLY_RELEASE_CONFIDENCE_PENALTY);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_FAVOURS_CLUB);
    let records: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_STAFF_RECORDS_ADD_FREE_RELEASE_RECORDS);
    let cancelar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_CANCEL_OFFERS_FOR_RELEASED_PLAYER);
    let free: unsafe extern "thiscall" fn(usize, usize, usize, i32, usize, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_FREE_TRANSFER_STAFF);
    let (mut l1, mut l2) = ([0u8; 14], [0u8; 14]);
    let (a, b) = (l1.as_mut_ptr() as usize, l2.as_mut_ptr() as usize);
    new(a);
    new(b);
    let queue = this + transfer_manager_fields::FREE_TRANSFER_QUEUE;
    // the header copy: the first 10 bytes and the cursor (+0xa) at the start (+2)
    core::ptr::copy_nonoverlapping(queue as *const u8, a as *mut u8, 0xa);
    write_usize(a + llist::CURSOR, read_usize(a + llist::HEAD));
    let mut e = next(a);
    while e != 0 {
        let p = read_usize(G_STAFF_TABLE).wrapping_add(read_usize(e).wrapping_mul(staff::SIZE));
        let c = contract_of(read_usize(G_CONTRACT_MANAGER), p);
        write_usize(G_CONTRACT_LOOKUP_RESULT, c);
        let club = if c != 0 { read_usize(G_CLUB_TABLE).wrapping_add(read_usize(c + contract::CLUB).wrapping_mul(club::SIZE)) } else { 0 };
        if club != 0 {
            let mut follow = true;
            if human(read_usize(G_NEWS), club) != 0 && read_u8(e + free_transfer_entry::REASON) == 0 {
                let r = bar(read_usize(G_MANAGER_MANAGER), p, club);
                if r as u16 != 0 {
                    barrado(read_usize(G_MANAGER_MANAGER), p, club, r);
                    follow = false;
                } else {
                    if read_u8(e + free_transfer_entry::REASON) == 0 {
                        rejeitado(p, club, 0);
                        if read_usize(club + club::MANAGER) != 0 {
                            desafeto(p, read_usize(club + club::MANAGER), 1);
                        }
                    }
                    board(read_usize(G_MANAGER_MANAGER), p, club);
                }
            }
            if follow {
                if read_u8(e + free_transfer_entry::REASON) == 0 {
                    let r = sorteio(read_u8(p + staff::PROFESSIONALISM) as i8 as i32);
                    if r < sorteio(0x14) || gosta(p, club) != 0 { // cps3: the grudge random draw
                        crate::comparison::branch("cps_3 desafeto do tecnico");
                        desafeto(p, read_usize(club + club::MANAGER), 1);
                    }
                }
                crate::comparison::branch("cps_3 liberado");
                records(read_usize(G_STAFF_RECORDS), p); // cps3: the release records
                cancelar(this, p);
                let comp = read_usize(e + free_transfer_entry::COMPENSATION); // cps3: the compensation to the club
                if comp != 0 {
                    crate::comparison::branch("cps_3 compensacao");
                }
                free(this, p, club, read_u8(e + free_transfer_entry::REASON) as i8 as i32, comp, 0);
            }
        }
        e = next(a);
    }
    // the copy becomes the empty list (the 14 bytes), the queue is cleared and the two locals leave
    core::ptr::copy_nonoverlapping(b as *const u8, a as *mut u8, 14);
    clear(queue);
    soltar(b);
    soltar(a);
}

/// Finds the first day, from today up to a year later, with the club's nation window open (0x8e8e50; without a rule,
/// today); when none, the "Error" box.
///
/// Ret 8; SEH frame only for the local dates: `TRANSFER_MANAGER::find_next_window_open_date(club, out)`: the rule of
/// the club's nation (+0x53) among the 28 at +0x8ac (+0xa); from today (`[0xae2c90]`) up to a year later (0x54c5b0 with
/// 365), the first day with the window open (0x8e8e50; without a rule, today) goes into `out`; not found, the "Error"
/// box 0x32c6. The desired signing (0x85fc50) overwrites the date with today right afterwards.
/// Original: `TRANSFER_MANAGER::find_next_window_open_date` (0x8cf470, transfer_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x8cf470 } (thiscall, ret 8, transfer_manager.cpp, quadro SEH só das datas locais): `TRANSFER_MANAGER::
/// find_next_window_open_date(clube, saida)`: a regra da nação do clube (+0x53) entre as 28 de +0x8ac (+0xa); de hoje
/// ([0xae2c90]) até um ano depois (0x54c5b0 com 365), o primeiro dia com a janela aberta (0x8e8e50; sem regra, hoje) vai
/// para `saida`; sem achar, a caixa "Error" 0x32c6. A contratação desejada (0x85fc50) regrava a data com hoje logo depois.
pub(crate) unsafe extern "thiscall" fn find_next_window_open_date(this: usize, club: usize, out: usize) {
    let data_default: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    let more: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_DATE_ADD_DAYS);
    let copy: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let aberta: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_TRANSFER_RULES_FIND_OPEN_WINDOW);
    let (mut d, mut lim, mut t1, mut t2) = ([0u8; 8], [0u8; 8], [0u8; 8], [0u8; 8]);
    let (dp, lp, p1, p2) = (d.as_mut_ptr() as usize, lim.as_mut_ptr() as usize, t1.as_mut_ptr() as usize, t2.as_mut_ptr() as usize);
    data_default(dp);
    more(G_CURRENT_DATE, lp, 0x16d);
    let nation = read_usize(club + club::NATION);
    let mut rule = 0usize;
    for k in 0..28 {
        let r = read_usize(this + transfer_manager_fields::TRANSFER_RULES + k * 4);
        if read_usize(r + transfer_rules::NATION_ID) == nation {
            rule = r;
            break;
        }
    }
    copy(dp, p1, G_CURRENT_DATE);
    loop {
        let (la, da) = (read_u16(lp + cm_date::YEAR), read_u16(dp + cm_date::YEAR));
        if !(la > da || (la == da && (read_u16(lp) as i16) > (read_u16(dp) as i16))) {
            return show_error_at(0xa8d514, 0x32c6);
        }
        if rule == 0 || aberta(rule, dp, 0) != 0 {
            copy(out, p2, dp);
            return;
        }
        let r = more(dp, p1, 1);
        copy(dp, p2, r);
    }
}

/// Returns the +0x946 list item with the person and the club, or 0.
///
/// Ret 8: `TRANSFER_MANAGER::find_negotiated_loan_contract(person, club)`: in list `[this+0x946]` (cursor +0xa goes
/// back to the start), the item with the person (+0) and the club (+4) by their ids; without a list or not found, 0.
/// Original: `TRANSFER_MANAGER::find_negotiated_loan_contract` (0x8c72d0, transfer_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x8c72d0 } (thiscall, ret 8, transfer_manager.cpp): `TRANSFER_MANAGER::find_negotiated_loan_contract(pessoa,
/// clube)`: na lista [this+0x946] (o cursor +0xa volta ao começo), o item com a pessoa (+0) e o clube (+4) pelos ids;
/// sem lista ou sem achar, 0.
pub(crate) unsafe extern "thiscall" fn find_negotiated_loan_contract(this: usize, person: usize, club: usize) -> usize {
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let l = read_usize(this + transfer_manager_fields::LOAN_NEGOTIATIONS);
    if l == 0 {
        return 0;
    }
    write_usize(l + llist::CURSOR, read_usize(l + llist::HEAD));
    let mut e = next(read_usize(this + transfer_manager_fields::LOAN_NEGOTIATIONS));
    while e != 0 {
        if read_usize(person) == read_usize(e) && read_usize(club) == read_usize(e + loan_negotiation::CLUB_ID) {
            return e;
        }
        e = next(read_usize(this + transfer_manager_fields::LOAN_NEGOTIATIONS));
    }
    0
}

// simplification: rolls (0x90cfc0) and releases the players (0x8c1af0, 0x8df9b0), clearing the queue: original only; the copy
// carries the branches.

/// Offer value (+0x1b) x f, truncated with ftol.
///
/// [pt] O valor da oferta (+0x1b) × `f`, truncado (_ftol 0x945640; o x87 em precisão dupla).
pub(crate) unsafe fn offer_value_times(o: usize, f: f64) -> usize {
    match_pl_ftol(read_usize(o + offer::FEE) as i32 as f64 * f) as usize
}

/// Person -> club of their contract (stored at 0xae1080), or 0.
///
/// [pt] O clube do contrato da pessoa (0x4cf5a0, guardado em [0xae1080]), ou 0.
pub(crate) unsafe fn current_contract_club(person: usize) -> usize {
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = contract_of(read_usize(G_CONTRACT_MANAGER), person);
    write_usize(G_CONTRACT_LOOKUP_RESULT, c);
    if c == 0 {
        0
    } else {
        read_usize(G_CLUB_TABLE).wrapping_add(read_usize(c + contract::CLUB).wrapping_mul(club::SIZE))
    }
}

/// Builds the clause offer option: with a much bigger bidding club and a high-potential player up to 30, a new offer
/// (0x8e5680, 0x8e78f0) with a 20% sell-on (appeal 80), international (70) or first-team (60) appearance money, and the
/// fee × 0.8; none, frees the offer and 0.
///
/// Thiscall with `this` unused, ret 0x14; SEH frame for the `new`: `TRANSFER_MANAGER::build_clause_offer_option(club,
/// player, out, appeal, value)`, from the computer club's answer to a bid (0x8c8090). With the bidding club more than 3
/// above the current club (0x4cf5a0) in reputation (+0x80), a player sheet (+0x61) with potential (+7) at least 25
/// above ability (+5) and up to 30 years old (+0x18): a new offer (0x944cf1 of 0x41 bytes, 0x8e5680) in `*out`,
/// initialised with the value (0x8e78f0), and appeal 50. Bidding club below 15 with the sheet's reputation (+0xb / 500)
/// more than 2 above: 20% of the next sale (+0x3f), the value × 0.8 and appeal 80. Otherwise, a player without caps
/// (+0x22) from a nation (+0x1a) above 5000 (+0x8e): the national side appearance bonus (+0x3a, value × 0.3, per 10
/// matches +0x3e), the value × 0.8 and appeal 70. Otherwise, a sheet reputation more than 3 below the club's: the team
/// appearance bonus (+0x35, value × 0.3, per 20 matches +0x39), the value × 0.8 and appeal 60. None: the offer is
/// destroyed (0x8e5710, 0x944c94; `*out` keeps the pointer, as in the original) and 0.
/// Original: `TRANSFER_MANAGER::build_clause_offer_option` (0x8c8fd0, transfer_manager.cpp, thiscall (this sem uso), ret 0x14).
///
/// [pt] { 0x8c8fd0 } (thiscall com o `this` sem uso, ret 0x14, transfer_manager.cpp, quadro SEH do `new`):
/// `TRANSFER_MANAGER::build_clause_offer_option(clube, jogador, saida, apelo, valor)`: pela resposta do clube do computador
/// ao lance (0x8c8090). Com o clube do lance mais de 3 acima na reputação (+0x80) do clube atual (0x4cf5a0), ficha de
/// jogador (+0x61) com potencial (+7) ao menos 25 acima da habilidade (+5) e até 30 anos (+0x18): uma oferta nova
/// (0x944cf1 de 0x41 bytes, 0x8e5680) em `*saida`, iniciada com o valor (0x8e78f0), e apelo 50. Clube do lance abaixo de
/// 15 com a reputação da ficha (+0xb / 500) mais de 2 acima: 20% da próxima venda (+0x3f), o valor × 0,8 e apelo 80.
/// Senão, jogador sem jogos pela seleção (+0x22) de nação (+0x1a) acima de 5000 (+0x8e): o prêmio por jogo na seleção
/// (+0x3a, valor × 0,3, por 10 jogos +0x3e), o valor × 0,8 e apelo 70. Senão, reputação da ficha mais de 3 abaixo da do
/// clube: o prêmio por jogo no time (+0x35, valor × 0,3, por 20 jogos +0x39), o valor × 0,8 e apelo 60. Nenhum: a
/// oferta é destruída (0x8e5710, 0x944c94; `*saida` fica com o ponteiro, como na original) e 0.
pub(crate) unsafe extern "thiscall" fn build_clause_offer_option(_this: usize, club: usize, player: usize, out: *mut usize, apelo: *mut u8, value: u32) -> u32 {
    let current = current_contract_club(player);
    if (read_u16(club + club::REPUTATION) as i16 as i32) - (read_u16(current + club::REPUTATION) as i16 as i32) <= 3 {
        return 0;
    }
    let sheet = read_usize(player + staff::PLAYER);
    if sheet == 0 || (read_u16(sheet + player_sheet::POTENTIAL_ABILITY) as i16 as i32) - (read_u16(sheet + player_sheet::CURRENT_ABILITY) as i16 as i32) < 0x19 || read_u8(player + staff::AGE) as i8 > 0x1e {
        return 0;
    }
    let new: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let construct: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_TRANSFER_OFFER_TRANSFER_OFFER);
    let iniciar: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(FN_TRANSFER_OFFER_INIT_TRANSFER_OFFER);
    let m = new(0x41);
    let o = if m != 0 { construct(m) } else { 0 };
    *out = o;
    iniciar(o, player, club, value);
    *apelo = 0x32;
    let rep = read_u16(club + club::REPUTATION) as i16;
    // the record’s reputation (+0xb) / 500, with the sign (the original’s magic multiplication)
    let fama = || (read_u16(read_usize(player + staff::PLAYER) + 0xb) as i16 as i32) / 500;
    if rep < 0xf && fama() - (rep as i32) > 2 {
        crate::comparison::branch("cps_4 write_percent_pair da venda");
        let o = *out;
        write_usize(o + offer::FEE, offer_value_times(o, 0.8)); // cps4: the percentage value
        *((*out + offer::SELL_ON_PERCENT) as *mut u8) = 0x14;
        *apelo = 0x50;
        return 1;
    }
    let nation = read_usize(player + staff::NATION);
    if nation != 0 && read_u16(nation + nation::REPUTATION) as i16 > 0x1388 && read_u8(player + staff::INT_APPS) == 0 {
        crate::comparison::branch("cps_4 premio na selecao");
        let o = *out;
        write_usize(o + offer::INTERNATIONAL_FEE, offer_value_times(o, 0.3));
        *((*out + offer::INTERNATIONAL_GAMES) as *mut u8) = 0xa;
        let o = *out;
        write_usize(o + offer::FEE, offer_value_times(o, 0.8)); // cps4: the bonus value in the national team
        *apelo = 0x46;
        return 1;
    }
    if fama() - (rep as i32) < -3 {
        crate::comparison::branch("cps_4 premio no time");
        let o = *out;
        write_usize(o + offer::APPEARANCE_FEE, offer_value_times(o, 0.3));
        *((*out + offer::APPEARANCE_GAMES) as *mut u8) = 0x14;
        let o = *out;
        write_usize(o + offer::FEE, offer_value_times(o, 0.8)); // cps4: the bonus value in the team
        *apelo = 0x3c;
        return 1;
    }
    let o = *out;
    if o != 0 {
        let destroy: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_TRANSFER_OFFER_DESTRUCTOR_TRANSFER_OFFER);
        let free: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_OPERATOR_DELETE);
        destroy(o);
        free(o);
    }
    0
}

/// Returns 0 when the rejection reason does not fit: hot prospect (0x2000) aged 24 or more, same division (0x1000),
/// rival (0x800) without a fixture or link; otherwise 1.
///
/// Stdcall, ret 8: `transfer_manager_is_rejection_reason_valid(reason, offer)`: 0 when the reason does not fit the
/// offer (person +0xc, bidding club +8): 0x2000 (prospect) with 24 years or more (+0x18); 0x1000 with the current club
/// (0x4cf5a0) and the bidding club in different divisions (+0x57); 0x800 (rival) with both clubs, not both human
/// (0x600a50), without a league fixture (0x4b02e0) and neither being a partner (+0xb3/+0xb7/+0xbb) of the other.
/// Otherwise 1.
/// Original: `transfer_manager_is_rejection_reason_valid` (0x8c5560, transfer_manager.cpp, stdcall, ret 8).
///
/// [pt] { 0x8c5560 } (stdcall, ret 8, transfer_manager.cpp): `transfer_manager_is_rejection_reason_valid(motivo, oferta)`: 0
/// quando o motivo não cabe na oferta (pessoa +0xc, clube do lance +8): 0x2000 (promessa) com 24 anos ou mais (+0x18);
/// 0x1000 com o clube atual (0x4cf5a0) e o do lance em divisões diferentes (+0x57); 0x800 (rival) com os dois clubes,
/// sem os dois humanos (0x600a50), sem confronto na liga (0x4b02e0) e sem um ser parceiro (+0xb3/+0xb7/+0xbb) do outro.
/// Senão 1.
pub(crate) unsafe extern "stdcall" fn transfer_manager_is_rejection_reason_valid(reason: u32, offer: usize) -> u32 {
    let person = read_usize(G_STAFF_TABLE).wrapping_add(read_usize(offer + offer::TARGET_STAFF_ID).wrapping_mul(staff::SIZE));
    let current = current_contract_club(person);
    let lance = read_usize(G_CLUB_TABLE).wrapping_add(read_usize(offer + offer::BUYER_CLUB_ID).wrapping_mul(club::SIZE));
    let r = match reason as u16 {
        0x2000 => (read_u8(person + staff::AGE) as i8) < 0x18,
        0x1000 => current == 0 || read_usize(current + club::DIVISION) == read_usize(lance + club::DIVISION),
        0x800 => {
            if lance == 0 || current == 0 {
                return 1;
            }
            let human: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
            let tie: unsafe extern "C" fn(usize, usize) -> u8 = core::mem::transmute(FN_COMP_UTIL_GET_LEAGUE_CLASH_TYPE);
            if human(current, 0, 1, 0, 0) != 0 && human(lance, 0, 1, 0, 0) != 0 {
                return 1;
            }
            tie(lance, current) != 0
                || [0xb3, 0xb7, 0xbb].iter().any(|&k| read_usize(lance + k) == current)
                || [0xb3, 0xb7, 0xbb].iter().any(|&k| read_usize(current + k) == lance)
        }
        _ => true,
    };
    r as u32
}

/// Returns the rejection reason by the squad status of the current club contract: 1 → 0x200, 2 and 3 → 0x400, 5 →
/// 0x2000; otherwise 0.
///
/// Stdcall, ret 4: `transfer_manager_get_squad_status_reason(offer)`: the refusal reason from the squad status (+0x4f
/// >> 4) of the contract of the person (+0xc) with the current club (0x4cf5f0): 1 → 0x200, 2 and 3 → 0x400, 5 → 0x2000;
/// otherwise 0.
/// Original: `transfer_manager_get_squad_status_reason` (0x8c54c0, transfer_manager.cpp, stdcall, ret 4).
///
/// [pt] { 0x8c54c0 } (stdcall, ret 4, transfer_manager.cpp): `transfer_manager_get_squad_status_reason(oferta)`: o motivo de
/// recusa pelo status no elenco (+0x4f >> 4) do contrato da pessoa (+0xc) com o clube atual (0x4cf5f0): 1 → 0x200, 2 e
/// 3 → 0x400, 5 → 0x2000; senão 0.
pub(crate) unsafe extern "stdcall" fn transfer_manager_get_squad_status_reason(offer: usize) -> u16 {
    let person = read_usize(G_STAFF_TABLE).wrapping_add(read_usize(offer + offer::TARGET_STAFF_ID).wrapping_mul(staff::SIZE));
    let current = current_contract_club(person);
    let with_club: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let c = with_club(read_usize(G_CONTRACT_MANAGER), person, current);
    if c == 0 {
        return 0;
    }
    match read_u8(c + contract::SQUAD_STATUS) >> 4 {
        1 => 0x200,
        2 | 3 => 0x400,
        5 => 0x2000,
        _ => 0,
    }
}

// simplification: allocates and frees the offer (0x944cf1, 0x944c94): original only; the copy takes the branches.

/// Fills the 42-byte restriction table with 0xff.
///
/// [pt] Os 42 bytes da tabela de restrições com 0xff (o `rep stosd` de 10 duplas e o `stosw` da original).
pub(crate) unsafe fn clear_restriction_table(t: *mut u8) {
    core::ptr::write_bytes(t, 0xff, 42);
}

/// Fills the 42-byte squad restrictions table with 0xff and, by the fixture's rules scope (0x8d2fa0), lets the nation
/// or continent rule fill it (virtual +0x1c); when asked, keeps only the first limit (+5/+3, +2/+0 or +0x16/+0x17).
///
/// Ret 0xc: `TRANSFER_MANAGER::fill_squad_restrictions(fixture, t, main_only)`: fills the 42 bytes of `t` with 0xff.
/// With a fixture: the rules' scope (0x8d2fa0); 0xff stops there. Outside type 4, the first of the 28 rules at +0x8ac
/// of the continent (+0xe, type 2) or of the nation (+0xa) with the same type (+0x12) fills the table (virtual +0x1c
/// with the fixture and `t`). With `main_only`, only the first limit that exists remains: +5 or +3 (written into +5),
/// +2 or +0 (written into +2), or +0x16 with +0x17; the rest goes back to 0xff.
/// Original: `TRANSFER_MANAGER::fill_squad_restrictions` (0x8cf6b0, transfer_manager.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x8cf6b0 } (thiscall, ret 0xc, transfer_manager.cpp): `TRANSFER_MANAGER::fill_squad_restrictions(partida, t,
/// so_a_principal)`: enche os 42 bytes de `t` com 0xff. Com partida: o alcance das regras (0x8d2fa0); 0xff para aí. Fora
/// do tipo 4, a primeira das 28 regras de +0x8ac do continente (+0xe, tipo 2) ou da nação (+0xa) com o mesmo tipo
/// (+0x12) enche a tabela (virtual +0x1c com a partida e `t`). Com `so_a_principal`, fica só o primeiro limite que
/// existe: +5 ou +3 (gravado em +5), +2 ou +0 (gravado em +2), ou +0x16 com +0x17; o resto volta a 0xff.
pub(crate) unsafe extern "thiscall" fn fill_squad_restrictions(this: usize, partida: usize, t: *mut u8, so_main: u32) {
    clear_restriction_table(t);
    if partida == 0 {
        return;
    }
    let (mut tipo, mut nacao, mut continente) = (0u8, 0usize, 0usize);
    let escopo: unsafe extern "thiscall" fn(usize, usize, *mut u8, *mut usize, *mut usize) = core::mem::transmute(FN_TRANSFER_MANAGER_GET_FIXTURE_RULES_SCOPE);
    escopo(this, partida, &mut tipo, &mut nacao, &mut continente);
    if tipo == 0xff {
        return;
    }
    if tipo != 4 {
        for k in 0..28 {
            let r = read_usize(this + transfer_manager_fields::TRANSFER_RULES + k * 4);
            let home = if tipo == 2 {
                read_usize(r + transfer_rules::CONTINENT_ID) == continente && read_u8(r + transfer_rules::SCOPE) == 2
            } else {
                read_usize(r + transfer_rules::NATION_ID) == nacao && read_u8(r + transfer_rules::SCOPE) == tipo
            };
            if home {
                let encher: unsafe extern "thiscall" fn(usize, usize, *mut u8) -> u32 = core::mem::transmute(read_usize(read_usize(r) + 0x1c));
                encher(r, partida, t);
                break;
            }
        }
    }
    if so_main == 0 {
        return;
    }
    let v = if *t.add(5) != 0xff { *t.add(5) } else { *t.add(3) };
    if v != 0xff {
        crate::comparison::branch("cpr_1 limit de nao comunitarios");
        clear_restriction_table(t);
        *t.add(5) = v; // the non-EU player limit
        return;
    }
    let v = if *t.add(2) != 0xff { *t.add(2) } else { *t };
    if v != 0xff {
        clear_restriction_table(t);
        *t.add(2) = v;
        return;
    }
    let (idade, n) = (*t.add(0x16), *t.add(0x17));
    clear_restriction_table(t);
    if idade != 0xff {
        *t.add(0x16) = idade;
        *t.add(0x17) = n;
    }
}

/// Counts, over the matchday squad slots (given players or the tactics 0x8968f0), the non-EU, foreign or over-age
/// players of the fixture limit (0x8cf7e0); when over, 1 and the message.
///
/// Thiscall through `this` in ecx, which the original passes straight to 0x8cf7e0; ret 0x18:
/// `transfer_manager_check_matchday_squad_limit(club, fixture, msg, players, _, out)`: the fixture's limit (0x8cf7e0
/// into 6 local bytes: +0 type, +1 age, +5 count); without a type, 0. The human side (+0x1c/+0x40 or +0x20/+0x41) goes
/// to 0x8968f0. Over the (+0x4d & 0xf) + 11 slots, the slot's person (`players[k]` or the tactic `[0xde1d08]` 0x8968f0)
/// counts if foreign in the club's nation (0x53d980 with 1; type 1, also outside the EU, 0x53d740) or over the age
/// (+0x18, type 3): +2 (starters) or +3 (the others) and +4 (all). The count goes into `out`; exceeding the limit, 1
/// and, with `msg`, the warning (0x66f4e0) by the type, the count and the limit.
/// Original: `transfer_manager_check_matchday_squad_limit` (0x8cf880, transfer_manager.cpp, thiscall (this em ecx), ret 0x18).
///
/// [pt] { 0x8cf880 } (thiscall pelo `this` em ecx, que a original passa direto à 0x8cf7e0; ret 0x18, transfer_manager.cpp):
/// `transfer_manager_check_matchday_squad_limit(clube, partida, msg, jogadores, _, saida)`: o limite da partida
/// (0x8cf7e0 em 6 bytes locais: +0 tipo, +1 idade, +5 quantidade); sem tipo, 0. O lado humano (+0x1c/+0x40 ou
/// +0x20/+0x41) vai para a 0x8968f0. Pelas (+0x4d & 0xf) + 11 vagas, a pessoa da vaga (`jogadores[k]` ou a tática
/// [0xde1d08] 0x8968f0) conta se é estrangeira na nação do clube (0x53d980 com 1; tipo 1, também fora da UE, 0x53d740)
/// ou passa da idade (+0x18, tipo 3): +2 (titulares) ou +3 (as outras) e +4 (todas). A conta vai a `saida`; passando da
/// quantidade, 1 e, com `msg`, o aviso (0x66f4e0) pelo tipo, pela conta e pela quantidade.
pub(crate) unsafe extern "thiscall" fn transfer_manager_check_matchday_squad_limit(this: usize, club: usize, partida: usize, msg: usize, players: usize, not_used: u32, out: usize) -> u32 {
    let limit: unsafe extern "thiscall" fn(usize, usize, *mut u8) = core::mem::transmute(FN_TRANSFER_MANAGER_GET_SELECTION_LIMIT);
    let of_tactic: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> usize = core::mem::transmute(FN_TACTICS_GET_SELECTED_PLAYER);
    let foreign: unsafe extern "C" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_DATABASE_IS_STAFF_FOREIGN);
    let community: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_STAFF_EU_NATIONAL);
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let mut l = [0u8; 6];
    limit(this, partida, l.as_mut_ptr());
    let kind = l[0];
    if kind == 0 {
        return 0;
    }
    let human = ((club == read_usize(partida + fixture::HOME_CLUB) && read_u8(partida + fixture::HOME_SIDE_MARK) == 0) || (club == read_usize(partida + fixture::AWAY_CLUB) && read_u8(partida + fixture::AWAY_SIDE_MARK) == 0)) as u32;
    if kind == 1 {
        crate::comparison::branch("cpr_1 conta nao comunitarios");
    }
    let mut k: i8 = 0;
    while k < ((read_u8(partida + fixture::SQUAD_RULES) & 0xf).wrapping_add(0xb)) as i8 {
        let p = if players == 0 {
            // simplification: the original pushes the match pair with the low byte replaced by the slot (0x8968f0 reads the byte)
            of_tactic(read_usize(G_TACTICS), club, (partida & !0xff) | k as u8 as usize, human)
        } else {
            read_usize(players + (k as isize * 4) as usize)
        };
        if p != 0 {
            let count = match kind {
                2 => foreign(p, read_usize(club + club::NATION), 1) != 0,
                1 => foreign(p, read_usize(club + club::NATION), 1) != 0 && community(p, read_usize(club + club::NATION)) == 0,
                3 => read_u8(p + staff::YEAR_OF_BIRTH) as i8 > l[1] as i8,
                _ => false,
            };
            if count {
                if k < 11 {
                    l[2] = l[2].wrapping_add(1);
                } else {
                    l[3] = l[3].wrapping_add(1);
                }
                l[4] = l[4].wrapping_add(1);
            }
        }
        k += 1;
    }
    if out != 0 {
        core::ptr::copy_nonoverlapping(l.as_ptr(), out as *mut u8, 6);
    }
    let (c, n) = (l[4] as i8, l[5] as i8);
    if !(1..=3).contains(&kind) || c <= n {
        return 0;
    }
    if msg == 0 {
        return 1;
    }
    if kind == 3 {
        if c > 1 {
            text(msg, 0xa8ee08, c as i32, l[1] as i8 as i32);
        } else {
            text(msg, 0xa8edbc, l[1] as i8 as i32);
        }
        let mut end = msg; // the end of the text (the original’s `repne scasb`)
        while read_u8(end) != 0 {
            end += 1;
        }
        if n > 1 {
            text(end, 0xa8ed8c, n as i32);
        } else if n == 1 {
            text(end, 0xa8ed70);
        } else {
            text(end, 0xa8ed48);
        }
        return 1;
    }
    // the type 1 texts (non-EU) and type 2 texts (foreigners)
    let f: [usize; 5] = if kind == 1 { [0xa8f208, 0xa8f19c, 0xa8f134, 0xa8f0cc, 0xa8f07c] } else { [0xa8effc, 0xa8ef90, 0xa8ef28, 0xa8eebc, 0xa8ee68] };
    if c > 1 {
        if n > 1 {
            text(msg, f[0], c as i32, n as i32);
        } else if n == 1 {
            text(msg, f[1], c as i32);
        } else {
            text(msg, f[2], c as i32);
        }
    } else if n == 1 {
        text(msg, f[3], c as i32);
    } else if n == 0 {
        text(msg, f[4]);
    }
    1
}

/// Returns 1 when the club's nation uses work permits (+0x76 = 0xe), the player is not EU and holds no granted permit
/// (type 2 contract, 0x20 flag, search in the +0x93a array).
///
/// Ret 0xc: `TRANSFER_MANAGER::player_needs_work_permit(player, club, ignore_given)`: 0 without a club or without its
/// nation (+0x53); null player: the "Error" box 0x1c23 and 0. Nation without flag 0xe (+0x76) or an EU player (0x53d740
/// with nation 0): 0. With `ignore_given`: 1. Otherwise 0 only if the contract (0x4cf5a0) is of type 2 (+0x35 & 0x3f),
/// the id fits in `[0xae2364]`, it has flag 0x20 (`[this+0x86e]` + id × 8) and the permit is in vector `[this+0x93a]`
/// (count +0x93e; binary search 0x946464 with 0x8e55c0); otherwise 1.
/// Original: `TRANSFER_MANAGER::player_needs_work_permit` (0x8c7560, transfer_manager.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x8c7560 } (thiscall, ret 0xc, transfer_manager.cpp): `TRANSFER_MANAGER::player_needs_work_permit(jogador, clube,
/// ignorar_a_dada)`: 0 sem clube ou sem a nação dele (+0x53); jogador nulo: a caixa "Error" 0x1c23 e 0. Nação sem a marca
/// 0xe (+0x76) ou jogador da UE (0x53d740 com nação 0): 0. Com `ignorar_a_dada`: 1. Senão 0 só se o contrato (0x4cf5a0)
/// é do tipo 2 (+0x35 & 0x3f), o id cabe em [0xae2364], tem a marca 0x20 ([this+0x86e] + id × 8) e a permissão está no
/// vetor [this+0x93a] (conta +0x93e; busca binária 0x946464 com 0x8e55c0); senão 1.
pub(crate) unsafe extern "thiscall" fn player_needs_work_permit(this: usize, player: usize, club: usize, ignorar_a_dada: u32) -> u32 {
    if club == 0 || read_usize(club + club::NATION) == 0 {
        return 0;
    }
    if player == 0 {
        show_error_at(0xa8d514, 0x1c23);
        return 0;
    }
    if read_u8(read_usize(club + club::NATION) + 0x76) != 0xe {
        return 0;
    }
    let community: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_STAFF_EU_NATIONAL);
    if community(player, 0) != 0 {
        return 0;
    }
    if ignorar_a_dada != 0 {
        return 1;
    }
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = contract_of(read_usize(G_CONTRACT_MANAGER), player);
    if c != 0 && read_u8(c + contract::CONTRACT_TYPE) & 0x3f == 2 {
        let id = read_usize(player) as i32;
        if id < read_usize(G_STAFF_COUNT) as i32 && read_u8(read_usize(this + transfer_manager_fields::PERSON_FLAGS).wrapping_add((id as usize).wrapping_mul(8))) & 0x20 != 0 {
            let bsearch: unsafe extern "C" fn(*const i32, usize, usize, u32, usize) -> usize = core::mem::transmute(FN_CRT_BSEARCH);
            if bsearch(&id, read_usize(this + transfer_manager_fields::WORK_PERMITS), read_usize(this + transfer_manager_fields::WORK_PERMIT_COUNT), 0xe, 0x8e55c0) != 0 {
                crate::comparison::branch("cpr_1 permissao achada");
                return 0; // has the permission
            }
        }
    }
    1
}

/// Rewinds a LLIST and collects all node pointers (cursor left at the end).
///
/// [pt] Os itens de uma LLIST, do começo (o cursor +0xa volta a +2 e fica no fim, como depois de uma volta da original).
pub(crate) unsafe fn llist_collect(l: usize) -> Vec<usize> {
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    write_usize(l + llist::CURSOR, read_usize(l + llist::HEAD));
    let mut v = Vec::new();
    let mut e = next(l);
    while e != 0 {
        v.push(e);
        e = next(l);
    }
    v
}

/// Body of TRANSFER_MANAGER::can_club_bid_for_player (0x8ce990) over a built local list: returns (reason, allowed).
///
/// [pt] O corpo da 0x8ce990 sobre a lista local `l` (já construída): devolve (motivo, pode).
pub(crate) unsafe fn can_club_bid_body(this: usize, player: usize, club: usize, ignorar_abertas: u32, l: usize) -> (u16, u32) {
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let acertada: unsafe extern "thiscall" fn(usize, i32) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_FIND_AGREED_CONTRACT_OFFER);
    let swaps: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_EXCHANGE_OFFERS_FOR_PLAYER);
    let negociacoes: unsafe extern "thiscall" fn(usize, usize, usize) -> i32 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_EXCHANGE_NEGOTIATIONS_FOR_PLAYER);
    let sob_offer: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_IS_PLAYER_UNDER_OFFER);
    let offers: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_OFFERS_FOR_PLAYER);
    let basco: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_BASQUE_CLUB);
    let foreign: unsafe extern "C" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_DATABASE_IS_STAFF_FOREIGN);
    let nat = read_usize(player + staff::NATION);
    if nat != 0 && read_usize(nat) == read_usize(KEY_NATION_SAUDI_ARABIA) {
        let cc = read_usize(player + staff::CLUB_JOB);
        if (cc == 0 || read_usize(cc + club::NATION) == nat) && club != 0 && read_usize(club + club::NATION) != nat {
            return (0x27, 0); // cannot leave the nation
        }
    }
    let mark = read_usize(this + transfer_manager_fields::PERSON_FLAGS).wrapping_add(read_usize(player).wrapping_mul(8));
    if read_u8(mark) & 0x40 != 0 {
        if acertada(read_usize(G_CONTRACT_MANAGER), read_usize(player) as i32) != 0 {
            crate::comparison::branch("cpr_2 contrato acertado");
            return (0x21, 0); // contrato acertado
        }
        let m = read_usize(this + transfer_manager_fields::PERSON_FLAGS).wrapping_add(read_usize(player).wrapping_mul(8)) as *mut u8;
        *m &= 0xbf;
    }
    if ignorar_abertas == 0 && (swaps(this, player, 0) != 0 || negociacoes(this, player, 0) != 0) {
        return (7, 0);
    }
    // the free transfer queue (+0x92c)
    let queue = this + transfer_manager_fields::FREE_TRANSFER_QUEUE;
    write_usize(queue + llist::CURSOR, read_usize(queue + llist::HEAD));
    let mut e = next(queue);
    while e != 0 {
        if read_usize(player) == read_usize(e) {
            return (1, 0);
        }
        e = next(queue);
    }
    if sob_offer(this, player, 0) != 0 {
        offers(this, player, l, 0);
        write_usize(l + llist::CURSOR, read_usize(l + llist::HEAD));
        let mut e = next(l);
        while e != 0 {
            let k = read_usize(e) as i32;
            let slot = if k < read_usize(this + transfer_manager_fields::OFFER_CAPACITY) as i32 { read_usize(this).wrapping_add((k as usize).wrapping_mul(4)) } else { read_usize(this) };
            let o = read_usize(slot);
            let st = read_u8(o + offer::STATUS);
            if (0xa..=0xe).contains(&st) {
                crate::comparison::branch("cpr_2 oferta em negociacao");
                *((o + offer::PLAYER_AWARE) as *mut u8) = 1;
                return (0x12, 0); // already in negotiation
            }
            if st == 0x13 {
                crate::comparison::branch("cpr_2 transferencia futura");
                return (0x20, 0); // future transfer agreed
            }
            e = next(l);
        }
    }
    if club == 0 || player == 0 || read_usize(player + staff::PLAYER) == 0 {
        return (0, 1);
    }
    if basco(club) != 0 {
        let nation = read_usize(G_NATION_TABLE).wrapping_add(read_usize(KEY_NATION_PAYS_BASQUE).wrapping_mul(nation::SIZE));
        if foreign(player, nation, 1) != 0 {
            return (0x19, 0);
        }
        return (0, 1);
    }
    let n = read_usize(club + club::NATION);
    if n != 0 && read_usize(n) == read_usize(KEY_NATION_TURKEY) {
        let c57 = read_usize(club + club::DIVISION);
        if c57 == 0 || read_usize(c57) != read_usize(KEY_CLUB_COMP_TURKISH_PREMIER_DIVISION) {
            if foreign(player, n, 1) != 0 {
                return (0x28, 0);
            }
            return (0, 1);
        }
    }
    let id = read_usize(club);
    if id == read_usize(G_KEY_CLUB_REAL_SOCIEDAD_C_F) || id == read_usize(G_KEY_CLUB_REAL_SOCIEDAD_C_F_B) {
        let n1 = read_usize(player + staff::NATION);
        if n1 == 0 || read_usize(n1) == read_usize(KEY_NATION_SPAIN) {
            let n2 = read_usize(player + staff::SECOND_NATION);
            if n2 == 0 || read_usize(n2) != read_usize(KEY_NATION_PAYS_BASQUE) {
                return (0x1a, 0);
            }
        }
    }
    (0, 1)
}

/// Tells whether the club can bid for the player. Cannot (0 and the reason): nation that keeps its players (0x27),
/// agreed contract (0x21), open exchanges (7), free queue (1), offer in negotiation (0x12, flag +0x2d) or future
/// transfer (0x20), the rules of the Basque club (0x19), of nation `[0x9cf4dc]` (0x28) and of clubs
/// `[0x9d0128]`/`[0x9d01a0]` (0x1a); otherwise 1.
///
/// Ret 0x10; SEH frame for the local LLIST 0x68d020/0x68d040: `TRANSFER_MANAGER::can_club_bid_for_player(player, club,
/// reason, ignore_open)`: cannot (0 and the reason) if the player is from nation `[0x9cf458]` without a club or with a
/// club of that nation and the club is from outside (0x27); with the agreed contract flag (0x40) and the agreement
/// (0x4cca30) (0x21; without an agreement, the flag is removed); without `ignore_open`, with exchanges offering him
/// (0x8c5e40) or exchange negotiations (0x8c5fc0) (7); in the free transfer queue (+0x92c) (1); under offer (0x8cde70),
/// through the offers for him (0x8c5cf0): one in negotiation (states 0xa to 0xe; flag +0x2d) (0x12) or a future
/// transfer (0x13) (0x20). By the club: Basque (0x53d7f0) with a player foreign in nation `[0x9cf424]` (0x19); of
/// nation `[0x9cf4dc]` outside `[0x9cf8d8]` (+0x57) with a foreign player (0x28); clubs `[0x9d0128]`/`[0x9d01a0]` with
/// a player without nation `[0x9cf424]` (0x1a). Otherwise it can (1, reason 0).
/// Original: `TRANSFER_MANAGER::can_club_bid_for_player` (0x8ce990, transfer_manager.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x8ce990 } (thiscall, ret 0x10, transfer_manager.cpp, quadro SEH da LLIST local 0x68d020/0x68d040):
/// `TRANSFER_MANAGER::can_club_bid_for_player(jogador, clube, motivo, ignorar_abertas)`: não pode (0 e o motivo) se o
/// jogador é da nação [0x9cf458] sem clube ou com clube dela e o clube é de fora (0x27); com a marca de contrato
/// acertado (0x40) e o acerto (0x4cca30) (0x21; sem acerto, a marca sai); sem `ignorar_abertas`, com trocas que o
/// oferecem (0x8c5e40) ou negociações de troca (0x8c5fc0) (7); na fila da transferência livre (+0x92c) (1); sob oferta
/// (0x8cde70), pelas ofertas por ele (0x8c5cf0): uma em negociação (estados 0xa a 0xe; marca +0x2d) (0x12) ou uma
/// transferência futura (0x13) (0x20). Pelo clube: basco (0x53d7f0) com jogador estrangeiro na nação [0x9cf424] (0x19);
/// da nação [0x9cf4dc] fora da [0x9cf8d8] (+0x57) com jogador estrangeiro (0x28); os clubes [0x9d0128]/[0x9d01a0] com
/// jogador sem a nação [0x9cf424] (0x1a). Senão pode (1, motivo 0).
pub(crate) unsafe extern "thiscall" fn can_club_bid_for_player(this: usize, player: usize, club: usize, reason: *mut u16, ignorar_abertas: u32) -> u32 {
    let new: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let mut list = [0u8; 0x10];
    let l = list.as_mut_ptr() as usize;
    new(l);
    let (m, r) = can_club_bid_body(this, player, club, ignorar_abertas, l);
    if !reason.is_null() {
        *reason = m;
    }
    soltar(l);
    r
}

/// Flags that the exchange player was sent terms; when none is flagged, the offer moves to state 8 (from 7) or 6.
///
/// Ret 8: `TRANSFER_MANAGER::note_terms_offered_to_exchange_player(k, person)`: `k` negative or beyond the count (+4):
/// the "Error" box (0xdc3 or 0xdc8). In offer `k` (empty: nothing), through the list of exchange players (+4), marks
/// (+4 of the item) that of `person`; no item marked: the state (+0x2c) becomes 8 (if it was 7) or 6.
/// Original: `TRANSFER_MANAGER::note_terms_offered_to_exchange_player` (0x8c2060, transfer_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x8c2060 } (thiscall, ret 8, transfer_manager.cpp): `TRANSFER_MANAGER::note_terms_offered_to_exchange_player(k,
/// pessoa)`: `k` negativo ou fora da conta (+4): a caixa "Error" (0xdc3 ou 0xdc8). Na oferta `k` (vazia: nada), pela
/// lista dos jogadores da troca (+4), marca (+4 do item) o da `pessoa`; nenhum item marcado: o estado (+0x2c) vira 8 (se
/// era 7) ou 6.
pub(crate) unsafe extern "thiscall" fn note_terms_offered_to_exchange_player(this: usize, k: i32, person: u32) {
    if k < 0 {
        show_error_at(0xa8d514, 0xdc3);
        return;
    }
    if k >= read_usize(this + transfer_manager_fields::OFFER_CAPACITY) as i32 {
        show_error_at(0xa8d514, 0xdc8);
        return;
    }
    let o = read_usize(read_usize(this).wrapping_add((k as usize).wrapping_mul(4)));
    if o == 0 {
        return;
    }
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let l = read_usize(o + offer::EXCHANGE_PLAYERS);
    write_usize(l + llist::CURSOR, read_usize(l + llist::HEAD));
    let mut nenhum = true;
    let mut e = next(read_usize(o + offer::EXCHANGE_PLAYERS));
    while e != 0 {
        if read_usize(e) as u32 == person {
            *((e + exchange_player::TERMS_STATE) as *mut u8) = 1; // termos mandados
        }
        if read_u8(e + exchange_player::TERMS_STATE) != 0 {
            nenhum = false;
        }
        e = next(read_usize(o + offer::EXCHANGE_PLAYERS));
    }
    if nenhum {
        *((o + offer::STATUS) as *mut u8) = if read_u8(o + offer::STATUS) == 7 { 8 } else { 6 };
    }
}

/// Counts the free transfer queue persons of the club or of its partner club (0x540a50).
///
/// Ret 4: `TRANSFER_MANAGER::count_club_free_transfer_queue(club)`: null club: the "Error" box 0xc04 and 0. Otherwise
/// counts the persons of the free transfer queue (+0x92c; the cursor goes back to the start) with the contract club
/// (+0x39) equal to the club or to its partner (0x540a50 with 0 and 1), non-null.
/// Original: `TRANSFER_MANAGER::count_club_free_transfer_queue` (0x8c1780, transfer_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x8c1780 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::count_club_free_transfer_queue(clube)`:
/// clube nulo: a caixa "Error" 0xc04 e 0. Senão conta as pessoas da fila da transferência livre (+0x92c; o cursor volta
/// ao começo) com o clube do contrato (+0x39) igual ao clube ou ao parceiro dele (0x540a50 com 0 e 1), não nulo.
pub(crate) unsafe extern "thiscall" fn count_club_free_transfer_queue(this: usize, club: usize) -> u8 {
    if club == 0 {
        show_error_at(0xa8d514, 0xc04);
        return 0;
    }
    let partner: unsafe extern "C" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let p = partner(club, 0, 1);
    write_usize(this + transfer_manager_fields::FREE_TRANSFER_QUEUE_CURSOR, read_usize(this + transfer_manager_fields::FREE_TRANSFER_QUEUE_HEAD));
    let mut n = 0u8;
    let mut e = next(this + transfer_manager_fields::FREE_TRANSFER_QUEUE);
    while e != 0 {
        let c = read_usize(read_usize(G_STAFF_TABLE).wrapping_add(read_usize(e).wrapping_mul(staff::SIZE)) + 0x39);
        if c == club || (c == p && p != 0) {
            n = n.wrapping_add(1);
        }
        e = next(this + transfer_manager_fields::FREE_TRANSFER_QUEUE);
    }
    n
}

/// Pointer to the flags byte of person id in the 8-byte records at [this+0x86e].
///
/// [pt] O byte de marcas da pessoa de id `id` no registro de 8 bytes em [this+0x86e].
pub(crate) unsafe fn staff_flags_byte(this: usize, id: usize) -> *mut u8 {
    read_usize(this + transfer_manager_fields::PERSON_FLAGS).wrapping_add(id.wrapping_mul(8)) as *mut u8
}

/// In a list of offer indices (rewound), index of the first offer with status 0x13 (future transfer), or None.
///
/// [pt] Na lista de índices de ofertas `l` (o cursor volta ao começo), o índice da primeira oferta de estado 0x13
/// (transferência futura), ou None.
pub(crate) unsafe fn find_future_offer_in_list(this: usize, l: usize) -> Option<usize> {
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    write_usize(l + llist::CURSOR, read_usize(l + llist::HEAD));
    let mut e = next(l);
    while e != 0 {
        if read_u8(read_usize(read_usize(this).wrapping_add(read_usize(e).wrapping_mul(4))) + 0x2c) == 0x13 {
            return Some(read_usize(e));
        }
        e = next(l);
    }
    None
}

/// Body of TRANSFER_MANAGER::get_future_transfer (0x8cca10) over a built local list: finds the future-transfer offer,
/// writes its record to `saida`, clears flag 0x10 if none.
///
/// [pt] O corpo da 0x8cca10 sobre a lista local `l` (já construída).
pub(crate) unsafe fn get_future_transfer_body(this: usize, player: usize, out: *mut usize, l: usize) -> u32 {
    let offers: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_OFFERS_FOR_PLAYER);
    let swaps: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_EXCHANGE_OFFERS_FOR_PLAYER);
    let clear: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_CLEAR);
    if *staff_flags_byte(this, read_usize(player)) & 0x10 == 0 {
        return 0;
    }
    let mut k = None;
    if offers(this, player, l, 0) != 0 {
        k = find_future_offer_in_list(this, l).filter(|&i| i != usize::MAX);
    }
    if k.is_none() {
        clear(l);
        if swaps(this, player, l) != 0 {
            k = find_future_offer_in_list(this, l).filter(|&i| i != usize::MAX);
        }
    }
    let Some(k) = k else {
        // simplification: the flag is reread by the original after the calls; the vector [this+0x86e] does not change between them
        *staff_flags_byte(this, read_usize(player)) &= 0xef; // no future transfer: the flag goes away
        return 0;
    };
    crate::comparison::branch("cpr_3 futura achada");
    if !out.is_null() {
        let n = read_usize(this + transfer_manager_fields::FUTURE_TRANSFER_COUNT) as i32;
        for i in 0..n.max(0) as usize {
            let r = read_usize(this + transfer_manager_fields::FUTURE_TRANSFERS).wrapping_add(i * 0xc);
            if read_usize(r + future_transfer::OFFER_SLOT) == k {
                *out = r;
                break;
            }
        }
    }
    1 // the future transfer exists
}

/// With the 0x10 flag, searches the offers for the player (0x8c5cf0) and the exchanges offering him (0x8c5e40) for a
/// state 0x13 one; found, the +0x920 array entry and 1; otherwise clears the flag and 0.
///
/// Ret 8; SEH frame for the local LLIST 0x68d020/0x68d040: `TRANSFER_MANAGER::get_future_transfer(player, out)`:
/// without flag 0x10 (`[this+0x86e]` + id × 8), 0. Through the offers for the player (0x8c5cf0) and, not found, through
/// the exchanges offering him (0x8c5e40, with the list emptied by 0x68d080), the first in state 0x13 with an index ≠
/// −1; with `out`, the 12-byte record of vector `[this+0x920]` (count +0x928) with the offer at +8; 1. None: clears
/// flag 0x10 and 0.
/// Original: `TRANSFER_MANAGER::get_future_transfer` (0x8cca10, transfer_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x8cca10 } (thiscall, ret 8, transfer_manager.cpp, quadro SEH da LLIST local 0x68d020/0x68d040):
/// `TRANSFER_MANAGER::get_future_transfer(jogador, saida)`: sem a marca 0x10 ([this+0x86e] + id × 8), 0. Pelas ofertas
/// pelo jogador (0x8c5cf0) e, sem achar, pelas trocas que o oferecem (0x8c5e40, com a lista esvaziada por 0x68d080), a
/// primeira de estado 0x13 com índice ≠ −1; com `saida`, o registro de 12 bytes do vetor [this+0x920] (conta +0x928)
/// com a oferta em +8; 1. Nenhuma: tira a marca 0x10 e 0.
pub(crate) unsafe extern "thiscall" fn get_future_transfer(this: usize, player: usize, out: *mut usize) -> u32 {
    let new: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let mut list = [0u8; 0x10];
    let l = list.as_mut_ptr() as usize;
    new(l);
    let r = get_future_transfer_body(this, player, out, l);
    soltar(l);
    r
}

/// Clears the 0x10 flag of the player and the exchange players and removes the offer's entry from the +0x920 array (the
/// next ones move down, 0x8cbf40).
///
/// Ret 4: `TRANSFER_MANAGER::remove_future_transfer(offer)`: clears flag 0x10 of the player (+0xc) and of the exchange
/// players (list +4, the cursor goes back to the start); in vector `[this+0x920]` (count +0x928), the first record with
/// the offer (+0x20 of the offer, +8 of the record) is removed: the following ones move down one (0x8cbf40) and the
/// count drops by 1.
/// Original: `TRANSFER_MANAGER::remove_future_transfer` (0x8cbf70, transfer_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x8cbf70 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::remove_future_transfer(oferta)`: tira a
/// marca 0x10 do jogador (+0xc) e dos jogadores da troca (lista +4, o cursor volta ao começo); no vetor [this+0x920]
/// (conta +0x928), o primeiro registro com a oferta (+0x20 da oferta, +8 do registro) sai: os seguintes descem um
/// (0x8cbf40) e a conta cai de 1.
pub(crate) unsafe extern "thiscall" fn remove_future_transfer(this: usize, offer: usize) {
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let copy: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FUTURE_TRANSFER_ASSIGN);
    *staff_flags_byte(this, read_usize(offer + offer::TARGET_STAFF_ID)) &= 0xef;
    let l = read_usize(offer + offer::EXCHANGE_PLAYERS);
    if l != 0 {
        write_usize(l + llist::CURSOR, read_usize(l + llist::HEAD));
        let mut e = next(read_usize(offer + offer::EXCHANGE_PLAYERS));
        while e != 0 {
            *staff_flags_byte(this, read_usize(e)) &= 0xef;
            e = next(read_usize(offer + offer::EXCHANGE_PLAYERS));
        }
    }
    let n = read_usize(this + transfer_manager_fields::FUTURE_TRANSFER_COUNT) as i32;
    let target = read_usize(offer + offer::SLOT_INDEX);
    for i in 0..n.max(0) {
        if read_usize(read_usize(this + transfer_manager_fields::FUTURE_TRANSFERS).wrapping_add(i as usize * 0xc) + 8) == target {
            crate::comparison::branch("cpr_3 tirar futura");
            let mut j = i;
            while j < read_usize(this + transfer_manager_fields::FUTURE_TRANSFER_COUNT) as i32 - 1 {
                let r = read_usize(this + transfer_manager_fields::FUTURE_TRANSFERS).wrapping_add(j as usize * 0xc);
                copy(r, r + 0xc);
                j += 1;
            }
            write_usize(this + transfer_manager_fields::FUTURE_TRANSFER_COUNT, read_usize(this + transfer_manager_fields::FUTURE_TRANSFER_COUNT).wrapping_sub(1));
            return;
        }
    }
}

/// Adds (date, offer) to the +0x920 array and sorts it; the offer moves to state 0x13; suspends the shortlists for the
/// player and the exchange players (0x842b00) and sets their 0x10 flag; a flagged contract keeps the offer.
///
/// Ret 8: `TRANSFER_MANAGER::add_future_transfer(offer, date)`: appends to vector `[this+0x920]` (capacity +0x924,
/// count +0x928; full, grows by 10 at a time, out of memory: box 0x27c3 and nothing) the 12-byte record with `date`
/// (0x417c10) and the offer (+0x20 at +8) and sorts it (qsort 0x945333 with 0x778e90). The offer gets +0x2d = 1 and
/// state 0x13. Suspends the clubs targeting the player (0x842b00 of `[this+0x856]`, with 0) and sets his flag 0x10;
/// with a contract (0x4cf5a0) flagged at +0x49, the offer goes to +0x4a of the contract. The same (suspend and flag)
/// for each exchange player (list +4).
/// Original: `TRANSFER_MANAGER::add_future_transfer` (0x8cb9a0, transfer_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x8cb9a0 } (thiscall, ret 8, transfer_manager.cpp): `TRANSFER_MANAGER::add_future_transfer(oferta, data)`:
/// acrescenta ao vetor [this+0x920] (capacidade +0x924, conta +0x928; cheio, cresce de 10 em 10, sem memória: a caixa
/// 0x27c3 e nada) o registro de 12 bytes com a `data` (0x417c10) e a oferta (+0x20 em +8) e reordena (qsort 0x945333
/// com 0x778e90). A oferta fica com +0x2d = 1 e o estado 0x13. Suspende os clubes que miram o jogador (0x842b00 de
/// [this+0x856], com 0) e liga a marca 0x10 dele; com contrato (0x4cf5a0) marcado em +0x49, a oferta vai a +0x4a do
/// contrato. O mesmo (suspender e marcar) para cada jogador da troca (lista +4).
pub(crate) unsafe extern "thiscall" fn add_future_transfer(this: usize, offer: usize, data: usize) {
    crate::comparison::branch("cpr_3 acrescentar futura");
    if !grow_record_vector(this, 0x920, 4, 10, 0xc, 0x27c3, 0xa8ec74) {
        return;
    }
    let copy: unsafe extern "thiscall" fn(usize, *mut [u16; 4], usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    let suspender: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_SHORTLIST_MANAGER_SUSPEND_CLUBS_SHORTLISTING_PLAYER);
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let mut temp = [0u16; 4];
    copy(read_usize(this + transfer_manager_fields::FUTURE_TRANSFERS).wrapping_add(read_usize(this + transfer_manager_fields::FUTURE_TRANSFER_COUNT).wrapping_mul(0xc)), &mut temp, data);
    write_usize(read_usize(this + transfer_manager_fields::FUTURE_TRANSFERS).wrapping_add(read_usize(this + transfer_manager_fields::FUTURE_TRANSFER_COUNT).wrapping_mul(0xc)) + 8, read_usize(offer + offer::SLOT_INDEX));
    let n = read_usize(this + transfer_manager_fields::FUTURE_TRANSFER_COUNT).wrapping_add(1);
    write_usize(this + transfer_manager_fields::FUTURE_TRANSFER_COUNT, n);
    qsort(read_usize(this + transfer_manager_fields::FUTURE_TRANSFERS), n as u32, 0xc, 0x778e90);
    *((offer + offer::PLAYER_AWARE) as *mut u8) = 1;
    *((offer + offer::STATUS) as *mut u8) = 0x13; // future transfer
    suspender(read_usize(this + transfer_manager_fields::SHORTLIST_MANAGER), staff_records_staff_by_id(read_usize(offer + offer::TARGET_STAFF_ID)), 0);
    *staff_flags_byte(this, read_usize(offer + offer::TARGET_STAFF_ID)) |= 0x10; // the future transfer mark
    let c = contract_of(read_usize(G_CONTRACT_MANAGER), staff_records_staff_by_id(read_usize(offer + offer::TARGET_STAFF_ID)));
    if c != 0 && read_u8(c + contract::LEAVING_ON_BOSMAN) != 0 {
        write_usize(c + contract::TRANSFER_ARRANGED_FOR, read_usize(offer + offer::SLOT_INDEX));
    }
    let l = read_usize(offer + offer::EXCHANGE_PLAYERS);
    if l != 0 {
        write_usize(l + llist::CURSOR, read_usize(l + llist::HEAD));
        let mut e = next(read_usize(offer + offer::EXCHANGE_PLAYERS));
        while e != 0 {
            suspender(read_usize(this + transfer_manager_fields::SHORTLIST_MANAGER), staff_records_staff_by_id(read_usize(e)), 0);
            *staff_flags_byte(this, read_usize(e)) |= 0x10;
            e = next(read_usize(offer + offer::EXCHANGE_PLAYERS));
        }
    }
}

/// Counts (and lists the indices of) the open offers for players contracted to the club, except states 2, 3, 4, 0x10,
/// 0x14, collapsed ones and loans.
///
/// Ret 8: `TRANSFER_MANAGER::get_open_offers_for_club_players(club, list)`: over the offers (vector `[this]`, count
/// +4), excluding the empty ones, those in state 2, 3, 4, 0x10 and 0x14 and those in state 0xf with +0x2e ≥ 3: is the
/// contract of the player (+0xc; 0x4cf5a0, stored in `[0xae1080]`), when it is not a loan (+0x35 & 0x3f ≠ 5), with
/// `club`? Counts and, with `list`, appends the index (4 bytes, 0x68d0d0). Returns the count.
/// Original: `TRANSFER_MANAGER::get_open_offers_for_club_players` (0x8c6100, transfer_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x8c6100 } (thiscall, ret 8, transfer_manager.cpp): `TRANSFER_MANAGER::get_open_offers_for_club_players(clube,
/// lista)`: pelas ofertas (vetor [this], conta +4), fora das vazias e das de estado 2, 3, 4, 0x10 e 0x14 e das de estado
/// 0xf com +0x2e ≥ 3: o contrato do jogador (+0xc; 0x4cf5a0, guardado em [0xae1080]) que não é de empréstimo (+0x35 &
/// 0x3f ≠ 5) é do `clube`? Conta e, com `lista`, acrescenta o índice (4 bytes, 0x68d0d0). Devolve a conta.
pub(crate) unsafe extern "thiscall" fn get_open_offers_for_club_players(this: usize, club: usize, list: usize) -> u32 {
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let append: unsafe extern "thiscall" fn(usize, *const i32, u32) -> u32 = core::mem::transmute(FN_LLIST_ADD_TAIL);
    let mut n = 0u32;
    let mut i: i32 = 0;
    let (mut desfeita, mut emprestado) = (false, false); // the counters, once per call
    while i < read_usize(this + transfer_manager_fields::OFFER_CAPACITY) as i32 {
        let o = read_usize(read_usize(this).wrapping_add((i as usize).wrapping_mul(4)));
        if o != 0 {
            let count = match read_u8(o + offer::STATUS) {
                2 | 3 | 4 | 0x10 | 0x14 => false,
                0xf => {
                    desfeita |= read_u8(o + offer::RESPONSE_ROUND) >= 3;
                    read_u8(o + offer::RESPONSE_ROUND) < 3 // undone (+0x2e = 3) does not count
                }
                _ => true,
            };
            if count {
                let c = contract_of(read_usize(G_CONTRACT_MANAGER), staff_records_staff_by_id(read_usize(o + offer::TARGET_STAFF_ID)));
                write_usize(G_CONTRACT_LOOKUP_RESULT, c);
                emprestado |= c != 0 && read_u8(c + contract::CONTRACT_TYPE) & 0x3f == 5 && friendly_club_by_id(read_usize(c + contract::CLUB)) == club;
                let owner = if c != 0 && read_u8(c + contract::CONTRACT_TYPE) & 0x3f != 5 { friendly_club_by_id(read_usize(c + contract::CLUB)) } else { 0 }; // the loan does not count
                if owner == club {
                    if list != 0 {
                        append(list, &i, 4);
                    }
                    n += 1;
                }
            }
        }
        i += 1;
    }
    if desfeita {
        crate::comparison::branch("cpr_4 oferta desfeita");
    }
    if emprestado {
        crate::comparison::branch("cpr_4 jogador emprestado");
    }
    n
}

/// Collapses the offers for a player: each offer for the player loses its future transfer (0x8cbf70), is collapsed
/// (0xf, +0x2e = 3), clears the buyer's and seller's bid markers (0x84aeb0) and is scheduled (0x8c6220, 10 days).
///
/// Ret 4; SEH frame for the local LLIST 0x68d020/0x68d040: `TRANSFER_MANAGER::collapse_offers_for_player(person)`: with
/// offers for the person (byte +1 of the 8-byte record in `[this+0x86e]`), through the offers for her (0x8c5cf0 into
/// the local list): one that is a future transfer (0x13) loses the future transfer (0x8cbf70); all go to state 0xf with
/// +0x2e = 3; the seller is the club of the offer player's contract (0x4cf5a0, stored in `[0xae1080]`); the buyer's
/// (+8) bids for the player and the seller's for the exchange players (list +4) are removed (0x84aeb0 of
/// `[this+0x856]`) and the offer is scheduled (0x8c6220 with 10 days).
/// Original: `TRANSFER_MANAGER::collapse_offers_for_player` (0x8c47d0, transfer_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x8c47d0 } (thiscall, ret 4, transfer_manager.cpp, quadro SEH da LLIST local 0x68d020/0x68d040):
/// `TRANSFER_MANAGER::collapse_offers_for_player(pessoa)`: com ofertas pela pessoa (byte +1 do registro de 8 bytes em
/// [this+0x86e]), pelas ofertas por ela (0x8c5cf0 na lista local): a de transferência futura (0x13) perde a futura
/// (0x8cbf70); todas ficam no estado 0xf com +0x2e = 3; o vendedor é o clube do contrato do jogador da oferta (0x4cf5a0,
/// guardado em [0xae1080]); os lances do comprador (+8) pelo jogador e os do vendedor pelos da troca (lista +4) saem
/// (0x84aeb0 de [this+0x856]) e a oferta é agendada (0x8c6220 com 10 dias).
pub(crate) unsafe extern "thiscall" fn collapse_offers_for_player(this: usize, person: usize) {
    let new: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let offers: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_OFFERS_FOR_PLAYER);
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let remove_future: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_REMOVE_FUTURE_TRANSFER);
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let unmark: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_SHORTLIST_MANAGER_CLEAR_BID_MARKERS_FOR_PLAYER);
    let schedule: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_SCHEDULE_OFFER);
    let mut list = [0u8; 0x10];
    let l = list.as_mut_ptr() as usize;
    new(l);
    if read_u8(read_usize(this + transfer_manager_fields::PERSON_FLAGS).wrapping_add(read_usize(person).wrapping_mul(8)) + 1) != 0 {
        offers(this, person, l, 0);
        write_usize(l + llist::CURSOR, read_usize(l + llist::HEAD));
        let mut e = next(l);
        while e != 0 {
            crate::comparison::branch("cpr_4 desfazer oferta");
            let o = read_usize(read_usize(this).wrapping_add(read_usize(e).wrapping_mul(4)));
            if read_u8(o + offer::STATUS) == 0x13 {
                remove_future(this, o);
            }
            *((o + offer::STATUS) as *mut u8) = 0xf; // desfeita
            *((o + offer::RESPONSE_ROUND) as *mut u8) = 3;
            let player = staff_records_staff_by_id(read_usize(o + offer::TARGET_STAFF_ID));
            let c = contract_of(read_usize(G_CONTRACT_MANAGER), player);
            write_usize(G_CONTRACT_LOOKUP_RESULT, c);
            let seller = if c != 0 { friendly_club_by_id(read_usize(c + contract::CLUB)) } else { 0 };
            unmark(read_usize(this + transfer_manager_fields::SHORTLIST_MANAGER), friendly_club_by_id(read_usize(o + offer::BUYER_CLUB_ID)), player);
            let t = read_usize(o + offer::EXCHANGE_PLAYERS);
            if t != 0 {
                write_usize(t + llist::CURSOR, read_usize(t + llist::HEAD));
                let mut x = next(read_usize(o + offer::EXCHANGE_PLAYERS));
                while x != 0 {
                    unmark(read_usize(this + transfer_manager_fields::SHORTLIST_MANAGER), seller, staff_records_staff_by_id(read_usize(x)));
                    x = next(read_usize(o + offer::EXCHANGE_PLAYERS));
                }
            }
            schedule(this, o, 0xa, 0); // the offer comes back in 10 days
            e = next(l);
        }
    }
    soltar(l);
}

/// Appends the 13-byte next-sale percentage to the +0x880 array (growing by 50) and sorts it (0x8e54d0).
///
/// Ret 4: `TRANSFER_MANAGER::add_next_sale_percentage(record)`: appends the next-sale percentage (13 bytes copied from
/// `record`) to vector `[this+0x880]` (capacity +0x884, count +0x888); full, grows by 50 at a time (out of memory: box
/// 0x2553 and nothing). Sorts it (qsort 0x945333 with 0x8e54d0, the order of search 0x8cb0f0).
/// Original: `TRANSFER_MANAGER::add_next_sale_percentage` (0x8caaf0, transfer_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x8caaf0 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::add_next_sale_percentage(registro)`:
/// acrescenta o percentual da próxima venda (13 bytes copiados de `registro`) ao vetor [this+0x880] (capacidade +0x884,
/// conta +0x888); cheio, cresce de 50 em 50 (sem memória: caixa 0x2553 e nada). Reordena (qsort 0x945333 com 0x8e54d0,
/// a ordem da busca 0x8cb0f0).
pub(crate) unsafe extern "thiscall" fn add_next_sale_percentage(this: usize, record: usize) {
    crate::comparison::branch("cpr_4 write_percent_pair da proxima venda");
    if !grow_record_vector(this, 0x880, 4, 50, 0xd, 0x2553, 0xa8eb50) {
        return;
    }
    let n = read_usize(this + transfer_manager_fields::NEXT_SALE_PERCENTAGE_COUNT);
    core::ptr::copy_nonoverlapping(record as *const u8, read_usize(this + transfer_manager_fields::NEXT_SALE_PERCENTAGES).wrapping_add(n.wrapping_mul(0xd)) as *mut u8, 0xd);
    write_usize(this + transfer_manager_fields::NEXT_SALE_PERCENTAGE_COUNT, n.wrapping_add(1)); // mais um percentual
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    qsort(read_usize(this + transfer_manager_fields::NEXT_SALE_PERCENTAGES), read_usize(this + transfer_manager_fields::NEXT_SALE_PERCENTAGE_COUNT) as u32, 0xd, 0x8e54d0);
}

/// O dia de hoje: Data de 8 bytes em 0xae2c90 (+0 dia do ano i16, +2 ano u16, +4 bissexto).
pub(crate) const TODAY: usize = G_CURRENT_DATE;

/// The statics of the date operations (0x54c5b0, 0x54c8e0, 0x54c770, 0x54cdb0: value and guard), for the comparers.
///
/// [pt] As estáticas das operações de data (0x54c5b0, 0x54c8e0, 0x54c770, 0x54cdb0: valor e guarda), para os comparadores.
pub(crate) const TRANSFER_MANAGER_DATE_STATICS: [(usize, usize); 8] = [(G_ADD_DAYS_RESULT_DATE, 8), (0xae2469, 1), (G_SUB_DAYS_RESULT_DATE, 8), (0xae249e, 1), (G_ADD_WEEKS_RESULT_DATE, 8), (0xae2430, 1), (G_DAYS_SINCE_SCRATCH_DATE, 8), (0xae2418, 1)];

/// Date `a` before `b` (unsigned year +2, then signed day +0), on 8-byte date values.
///
/// [pt] `a` antes de `b` na ordem dos originais: o ano (+2, 16 bits sem sinal) e, no mesmo ano, o dia (+0, 16 bits com sinal).
pub(crate) unsafe fn date_pair_before(a: *const [u32; 2], b: *const [u32; 2]) -> bool {
    let read = |p: *const [u32; 2]| (core::ptr::read_unaligned(p as *const i16), core::ptr::read_unaligned((p as usize + 2) as *const u16));
    let ((da, aa), (db, ab)) = (read(a), read(b));
    aa < ab || (aa == ab && da < db)
}

/// Offer -> club of the target's (+0xc) contract unless type 5, contract stored in G_CONTRACT_LOOKUP_RESULT.
///
/// [pt] O clube do contrato da pessoa da oferta (+0xc): 0x4cf5a0 com o gerente de contratos [0xae1050], guardado em
/// [0xae1080]; contrato de tipo 5 (+0x35 & 0x3f) não conta.
pub(crate) unsafe fn offer_seller_club(offer: usize) -> usize {
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = contract_of(read_usize(G_CONTRACT_MANAGER), staff_records_staff_by_id(read_usize(offer + offer::TARGET_STAFF_ID)));
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
    if c != 0 && read_u8(c + contract::CONTRACT_TYPE) & 0x3f != 5 {
        friendly_club_by_id(read_usize(c + contract::CLUB))
    } else {
        0
    }
}

/// Calls TRANSFER_RULES::can_make_offer (vtable +0xc)(offer, club, date, reason): 1 allows, 0 with reason.
///
/// [pt] O método +0xc das regras (`TRANSFER_RULES::can_make_offer(oferta, clube, data, motivo)`): 1 permite; 0 e o motivo.
pub(crate) unsafe fn rules_can_make_offer(rule: usize, offer: usize, club: usize, data: *const [u32; 2], reason: *mut u32) -> u32 {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, *const [u32; 2], *mut u32) -> u32 = core::mem::transmute(read_usize(read_usize(rule) + 0xc));
    f(rule, offer, club, data, reason)
}

/// Reasons that do not hold the date back: 0x14, 0x15, 0x1f, 0x11.
///
/// [pt] Os motivos que não seguram a data (o dia vale assim mesmo): 0x14, 0x15, 0x1f e 0x11.
pub(crate) fn reason_ignores_date(m: u32) -> bool {
    matches!(m as u16, 0x14 | 0x15 | 0x1f | 0x11)
}

/// Returns the offer's completion date by its timing type (+0x30): today, the first allowed day (0x8cf0e0), the
/// seller's season end plus 7 days, the day after the contract ends (never before today) or the start of the fixed
/// window; returns the reason.
///
/// Ret 8: `transfer_manager_get_offer_completion_date(offer, out)`: the date on which the offer can go through, by the
/// deadline type (+0x30 & 0xf): 0 today; 5 the start of the fixed window (0x8d0bb0); 1 and 4 the first day the rules
/// allow (0x8cf0e0, which gives the reason); 2 the season end of the selling club's nation plus 7 days (0x669bd0;
/// outside the season, today; without a seller, the "Error" box 0x31da); 3 the day after the contract end (+0x2d),
/// never before today (without a contract, today). Other types write nothing. Returns the reason (only ax: 0 outside
/// types 1 and 4). The seller (0x4cf5a0, stored in `[0xae1080]`) is always computed.
/// Original: `transfer_manager_get_offer_completion_date` (0x8cee70, transfer_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x8cee70 } (thiscall, ret 8, transfer_manager.cpp): `transfer_manager_get_offer_completion_date(oferta, saida)`:
/// a data em que a oferta pode se concretizar, pelo tipo de prazo (+0x30 & 0xf): 0 hoje; 5 o início da janela fixa
/// (0x8d0bb0); 1 e 4 o primeiro dia que as regras permitem (0x8cf0e0, que dá o motivo); 2 o fim da temporada da nação
/// do clube vendedor mais 7 dias (0x669bd0; fora da temporada, hoje; sem vendedor, a caixa "Error" 0x31da); 3 o dia
/// seguinte ao fim do contrato (+0x2d), nunca antes de hoje (sem contrato, hoje). Outros tipos não gravam. Devolve o
/// motivo (só o ax: 0 fora dos tipos 1 e 4). O vendedor (0x4cf5a0, guardado em [0xae1080]) é calculado sempre.
pub(crate) unsafe extern "thiscall" fn transfer_manager_get_offer_completion_date(this: usize, offer: usize, out: *mut [u32; 2]) -> u16 {
    let more: unsafe extern "thiscall" fn(*const [u32; 2], *mut [u32; 2], u32) -> *const [u32; 2] = core::mem::transmute(FN_DATE_ADD_DAYS);
    let copy: unsafe extern "thiscall" fn(*mut [u32; 2], *mut [u32; 2], *const [u32; 2]) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let today = TODAY as *const [u32; 2];
    let (mut t, mut t2) = ([0u32; 2], [0u32; 2]);
    let seller = offer_seller_club(offer);
    match read_u8(offer + offer::COMPLETION_TIMING) & 0xf {
        0 => {
            crate::comparison::branch("cpq_5 conclusao imediata");
            copy(out, &mut t, today); // tipo 0: hoje
        }
        5 => {
            let window: unsafe extern "stdcall" fn(*mut [u32; 2], usize) = core::mem::transmute(FN_TRANSFER_MANAGER_GET_NEXT_WINDOW_DATES);
            window(out, 0);
        }
        1 | 4 => {
            let first: unsafe extern "thiscall" fn(usize, usize, *mut [u32; 2]) -> u16 = core::mem::transmute(FN_TRANSFER_MANAGER_FIND_NEXT_ALLOWED_TRANSFER_DATE);
            return first(this, offer, out);
        }
        2 => {
            if seller == 0 {
                show_error_at(0xa8d514, 0x31da);
            } else {
                let season: unsafe extern "C" fn(usize, usize, *mut [u32; 2]) -> u32 = core::mem::transmute(FN_KEY_NATION_CLUB_IN_SEASON);
                if season(seller, 0, out) != 0 {
                    let r = more(out, &mut t, 7);
                    copy(out, &mut t2, r);
                } else {
                    copy(out, &mut t2, today);
                }
            }
        }
        3 => {
            let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
            let c = contract_of(read_usize(G_CONTRACT_MANAGER), staff_records_staff_by_id(read_usize(offer + offer::TARGET_STAFF_ID)));
            if c != 0 {
                let r = more((c + contract::CONTRACT_EXPIRES) as *const [u32; 2], &mut t2, 1);
                copy(out, &mut t, r);
                if date_pair_before(out, today) {
                    copy(out, &mut t2, today);
                }
            } else {
                copy(out, &mut t2, today);
            }
        }
        _ => {}
    }
    0
}

/// Steps day by day, from today (or the contract end, in type 4) up to a year, until the rules of the buyer's and
/// seller's nations allow it (method +0xc); with no day, returns the first reason.
///
/// Ret 8: `TRANSFER_MANAGER::find_next_allowed_transfer_date(offer, out)`: writes today into the output and starts at
/// the later of today and, for deadline type 4 (+0x30 & 0xf), the contract end (+0x2d). Type 5: the start of the fixed
/// window (0x8d0bb0) and method +0xc of the rule of the buyer's (+8) nation gives the reason (9 does not count). A
/// person without a player sheet (+0x61): today. Otherwise steps day by day (up to today + 365, not reaching it)
/// through the rules of the buyer's nation and, with +4 of the offer and a seller (0x4cf5a0, stored in `[0xae1080]`),
/// of the seller's: the first day both allow (or whose reason releases: 0x14, 0x15, 0x1f, 0x11) goes into the output
/// and returns 0; with no day, returns the first reason (9 does not count) and the output stays today. Only ax is
/// returned.
/// Original: `TRANSFER_MANAGER::find_next_allowed_transfer_date` (0x8cf0e0, transfer_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x8cf0e0 } (thiscall, ret 8, transfer_manager.cpp): `TRANSFER_MANAGER::find_next_allowed_transfer_date(oferta,
/// saida)`: grava hoje na saída e começa no maior entre hoje e, no tipo de prazo 4 (+0x30 & 0xf), o fim do contrato
/// (+0x2d). Tipo 5: o início da janela fixa (0x8d0bb0) e o método +0xc da regra da nação do comprador (+8) dá o motivo
/// (9 não conta). Pessoa sem ficha de jogador (+0x61): hoje. Senão anda dia a dia (até hoje + 365, sem chegar) pelas
/// regras da nação do comprador e, com o +4 da oferta e um vendedor (0x4cf5a0, guardado em [0xae1080]), da do vendedor:
/// o primeiro dia que as duas permitem (ou cujo motivo libera: 0x14, 0x15, 0x1f, 0x11) vai para a saída e devolve 0;
/// sem dia, devolve o primeiro motivo (9 não conta) e a saída fica hoje. Só o ax é devolvido.
pub(crate) unsafe extern "thiscall" fn find_next_allowed_transfer_date(this: usize, offer: usize, out: *mut [u32; 2]) -> u16 {
    let default: unsafe extern "thiscall" fn(*mut [u32; 2]) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    let more: unsafe extern "thiscall" fn(*const [u32; 2], *mut [u32; 2], u32) -> *const [u32; 2] = core::mem::transmute(FN_DATE_ADD_DAYS);
    let copy: unsafe extern "thiscall" fn(*mut [u32; 2], *mut [u32; 2], *const [u32; 2]) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let today = TODAY as *const [u32; 2];
    let (mut dia, mut limit, mut inicio, mut t, mut t2) = ([0u32; 2], [0u32; 2], [0u32; 2], [0u32; 2], [0u32; 2]);
    default(&mut dia);
    more(today, &mut limit, 0x16d);
    default(&mut inicio);
    let mut reason = 0u32;
    // simplification: the rule’s reason is a single variable, without a value before the first call (the original does not zero it; the
    // rule writes it whenever it returns 0)
    let mut m = 0u32;
    copy(out, &mut t, today);
    let person = staff_records_staff_by_id(read_usize(offer + offer::TARGET_STAFF_ID));
    if read_u8(offer + offer::COMPLETION_TIMING) & 0xf == 4 {
        let c = contract_of(read_usize(G_CONTRACT_MANAGER), person);
        if c != 0 {
            copy(&mut inicio, &mut t, (c + contract::CONTRACT_EXPIRES) as *const [u32; 2]);
            t = *today;
            if date_pair_before(&inicio, today) {
                copy(&mut inicio, &mut t, today);
            }
        } else {
            copy(&mut inicio, &mut t, today);
        }
    } else {
        copy(&mut inicio, &mut t, today);
    }
    let buyer = friendly_club_by_id(read_usize(offer + offer::BUYER_CLUB_ID));
    let seller = offer_seller_club(offer);
    if read_u8(offer + offer::COMPLETION_TIMING) & 0xf == 5 {
        let window: unsafe extern "stdcall" fn(*mut [u32; 2], usize) = core::mem::transmute(FN_TRANSFER_MANAGER_GET_NEXT_WINDOW_DATES);
        window(out, 0);
        let rule = nation_transfer_rules_or_zero(this, read_usize(buyer + club::NATION));
        if rule != 0 && rules_can_make_offer(rule, offer, buyer, out, &mut m) == 0 && m as u16 != 9 {
            reason = m;
        }
        return reason as u16;
    }
    if read_usize(person + staff::PLAYER) == 0 {
        copy(out, &mut t, today);
        return 0;
    }
    let rule_c = nation_transfer_rules_or_zero(this, read_usize(buyer + club::NATION));
    let rule_v = if seller != 0 && read_usize(offer + offer::EXCHANGE_PLAYERS) != 0 { nation_transfer_rules_or_zero(this, read_usize(seller + club::NATION)) } else { 0 };
    crate::comparison::branch("cpq_5 dia permitido");
    copy(&mut dia, &mut t, &inicio); // the first day tested is the start
    while date_pair_before(&dia, &limit) {
        let mut ve_seller = true;
        if rule_c != 0 && rules_can_make_offer(rule_c, offer, buyer, &dia, &mut m) == 0 {
            if m as u16 != 9 && reason as u16 == 0 {
                reason = m;
            }
            ve_seller = reason_ignores_date(reason);
        }
        if ve_seller {
            if rule_v == 0 || rules_can_make_offer(rule_v, offer, seller, &dia, &mut m) != 0 {
                copy(out, &mut t2, &dia);
                return 0;
            }
            if m as u16 != 9 && reason as u16 == 0 {
                reason = m;
            }
            if reason_ignores_date(reason) {
                copy(out, &mut t2, &dia);
                return 0;
            }
        }
        let r = more(&dia, &mut t, 1);
        copy(&mut dia, &mut t2, r);
    }
    reason as u16
}

/// Returns the start (today, when inside) and the end of the current or next window: 1 to 31 July and 15 December to 15
/// January.
///
/// Stdcall, ret 8: `transfer_manager_get_next_window_dates(start, end)`: the fixed windows of today's year are 1 to 31
/// July and 15 December to 15 January (0x549f70; compared only by the day of the year, signed). Inside a window, the
/// start is today and the end that of the window (15 January of the following year, if it is the December one); before
/// July, 1 to 31 July; between August and 14 December, 15 December to 15 January of the following year. The end only
/// with a non-null `end`. Does not use ecx (the callers pass the manager).
/// Original: `transfer_manager_get_next_window_dates` (0x8d0bb0, transfer_manager.cpp, stdcall, ret 8).
///
/// [pt] { 0x8d0bb0 } (stdcall, ret 8, transfer_manager.cpp): `transfer_manager_get_next_window_dates(inicio, fim)`: as
/// janelas fixas do ano de hoje são 1º a 31 de julho e 15 de dezembro a 15 de janeiro (0x549f70; comparadas só pelo
/// dia do ano, com sinal). Dentro de uma janela, o início é hoje e o fim o da janela (15 de janeiro do ano seguinte, se
/// for a de dezembro); antes de julho, 1º a 31 de julho; entre agosto e 14 de dezembro, 15 de dezembro a 15 de janeiro
/// do ano seguinte. O fim só com `fim` não nulo. Não usa o ecx (os chamadores passam o gerente).
pub(crate) unsafe extern "stdcall" fn transfer_manager_get_next_window_dates(start: *mut [u32; 2], end: *mut [u32; 2]) {
    let dma: unsafe extern "thiscall" fn(*mut [u32; 2], u32, u32, u32, u32) -> *mut [u32; 2] = core::mem::transmute(FN_DATE_DATE_549F70);
    let copy: unsafe extern "thiscall" fn(*mut [u32; 2], *mut [u32; 2], *const [u32; 2]) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let today = TODAY as *const [u32; 2];
    // simplification: the original pushes the year in a register with garbage above (and the year + 1 over the argument’s word); the
    // 0x549f70 only uses the low 16 bits
    let year = || *((TODAY + 2) as *const u16) as u32;
    let (mut a, mut b, mut c, mut d, mut e, mut t) = ([0u32; 2], [0u32; 2], [0u32; 2], [0u32; 2], [0u32; 2], [0u32; 2]);
    dma(&mut a, 1, 6, year(), 0xffff_ffff); // July 1
    dma(&mut b, 0x1f, 6, year(), 0xffff_ffff); // 31 July
    dma(&mut c, 0xf, 0xb, year(), 0xffff_ffff); // December 15
    dma(&mut d, 0xf, 0, year(), 0xffff_ffff); // 15 January
    let day = |x: &[u32; 2]| x[0] as u16 as i16;
    let h = *(TODAY as *const i16);
    if h >= day(&a) && h <= day(&b) {
        copy(start, &mut t, today);
        if !end.is_null() {
            copy(end, &mut t, &b);
        }
    } else if h >= day(&c) {
        copy(start, &mut t, today);
        if !end.is_null() {
            dma(&mut e, 0xf, 0, (year() as u16).wrapping_add(1) as u32, 0xffff_ffff);
            copy(end, &mut t, &e);
        }
    } else if h <= day(&d) {
        copy(start, &mut t, today);
        if !end.is_null() {
            copy(end, &mut t, &d);
        }
    } else if h < day(&a) {
        crate::comparison::branch("cpq_5 janela de julho");
        copy(start, &mut t, &a);
        if !end.is_null() {
            copy(end, &mut t, &b);
        }
    } else {
        copy(start, &mut t, &c);
        if !end.is_null() {
            dma(&mut e, 0xf, 0, (year() as u16).wrapping_add(1) as u32, 0xffff_ffff);
            copy(end, &mut t, &e);
        }
    }
}

/// Memory regions for the date comparers: the outputs (8 bytes each), G_CONTRACT_LOOKUP_RESULT and the dates' statics.
///
/// [pt] As regiões dos comparadores de datas: a saída, o contrato guardado [0xae1080] e as estáticas das datas.
pub(crate) fn date_comparer_regions(outs: &[usize]) -> Vec<(usize, usize)> {
    let mut r: Vec<(usize, usize)> = outs.iter().filter(|&&s| s != 0).map(|&s| (s, 8)).collect();
    r.push((G_CONTRACT_LOOKUP_RESULT, 4));
    r.extend_from_slice(&TRANSFER_MANAGER_DATE_STATICS);
    r
}

/// Person -> club of their contract unless type 5, contract stored in G_CONTRACT_LOOKUP_RESULT.
///
/// [pt] O clube do contrato da `pessoa` (0x4cf5a0 com o gerente de contratos [0xae1050], guardado em [0xae1080]); contrato
/// de tipo 5 (+0x35 & 0x3f) não conta.
pub(crate) unsafe fn seller_club_of_person(person: usize) -> usize {
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = contract_of(read_usize(G_CONTRACT_MANAGER), person);
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
    if c != 0 && read_u8(c + contract::CONTRACT_TYPE) & 0x3f != 5 {
        friendly_club_by_id(read_usize(c + contract::CLUB))
    } else {
        0
    }
}

/// Writes `v` as little-endian u32 at offset k of a 13-byte record buffer.
pub(crate) unsafe fn put_u32_le(r: &mut [u8; 13], k: usize, v: usize) {
    r[k..k + 4].copy_from_slice(&(v as u32).to_le_bytes());
}

/// The statics of the date operations (value and guard), for the comparers.
///
/// [pt] As estáticas das operações de data (valor e guarda), para os comparadores.
pub(crate) const DATE_OP_STATICS: [(usize, usize); 8] = [(G_ADD_DAYS_RESULT_DATE, 8), (0xae2469, 1), (G_SUB_DAYS_RESULT_DATE, 8), (0xae249e, 1), (G_ADD_WEEKS_RESULT_DATE, 8), (0xae2430, 1), (G_DAYS_SINCE_SCRATCH_DATE, 8), (0xae2418, 1)];

/// Calculates the compensation fee: staff on a type 2 contract, what is left of the contract (1000 to 1000000); new
/// rules: the average with the player's value; between clubs of nation `[0x9cf4a8]` or, under 24, between British
/// nations: half the value (0x4d0c60).
///
/// Stdcall, ret 0xc; SEH frame only for the local date: `transfer_manager_calc_compensation_fee(person, buyer,
/// no_new_rules)`: without person or buyer, the "Error" box 0x372e and 0. Staff (without a player sheet +0x61, with
/// +0x69) on a type 2 contract: what is left of the contract (days until the end +0x2d × the wage +0xc / 7) between
/// 1000 and 1000000. New rules (`[0xae233c]` = 1, without the argument) with a type 2 contract: the average of what is
/// left and the player's value at the seller (0x4d0c60). Otherwise (a contract not of type 5, both nations): between
/// clubs of nation `[0x9cf4a8]`, half the value; outside that, only under 24 years at the date (the contract end, for
/// type 2; otherwise today) and between British nations or Ireland (0x5446d0), half the value. The other cases, 0.
/// Original: `transfer_manager_calc_compensation_fee` (0x8d0780, transfer_manager.cpp, stdcall, ret 0xc).
///
/// [pt] { 0x8d0780 } (stdcall, ret 0xc, transfer_manager.cpp, quadro SEH só da data local):
/// `transfer_manager_calc_compensation_fee(pessoa, comprador, sem_regras_novas)`: sem pessoa ou comprador, a caixa
/// "Error" 0x372e e 0. Comissão técnica (sem ficha de jogador +0x61, com +0x69) de contrato tipo 2: o que falta do
/// contrato (dias até o fim +0x2d × o salário +0xc / 7) entre 1000 e 1000000. Regras novas ([0xae233c] = 1, sem o
/// argumento) com contrato tipo 2: a média entre o que falta e o valor do jogador no vendedor (0x4d0c60). Senão (contrato
/// que não é de tipo 5, as duas nações): entre clubes da nação [0x9cf4a8], metade do valor; fora disso, só abaixo de 24
/// anos na data (o fim do contrato, no tipo 2; senão hoje) e entre nações britânicas ou a Irlanda (0x5446d0), metade
/// do valor. Os outros casos, 0.
pub(crate) unsafe extern "stdcall" fn transfer_manager_calc_compensation_fee(person: usize, buyer: usize, sem_regras_novas: u32) -> i32 {
    let default: unsafe extern "thiscall" fn(*mut [u32; 2]) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    let copy: unsafe extern "thiscall" fn(*mut [u32; 2], *mut [u32; 2], usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let days_until: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
    let value: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) -> i32 = core::mem::transmute(FN_CONTRACT_MANAGER_CALC_PLAYER_VALUE);
    let age: unsafe extern "C" fn(usize, *const [u32; 2]) -> u16 = core::mem::transmute(FN_DATABASE_GET_STAFF_AGE);
    let britanica: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_NATION_IS_UK_OR_IRELAND);
    let manager = || read_usize(G_CONTRACT_MANAGER);
    // what is left of the contract: the days until the end × the weekly wage, divided by 7 (truncated division, like the original)
    let rest = |c: usize| days_until(c + contract::CONTRACT_EXPIRES, read_usize(G_CURRENT_DATE) as u32, read_usize(G_CURRENT_DATE_LEAP) as u32).wrapping_mul(read_usize(c + contract::WAGE) as i32) / 7;
    let mut data = [0u32; 2];
    let mut t = [0u32; 2];
    default(&mut data);
    if person == 0 || buyer == 0 {
        show_error_at(0xa8d514, 0x372e);
        return 0;
    }
    if read_usize(person + staff::PLAYER) == 0 && read_usize(person + staff::NON_PLAYER) != 0 {
        let c = contract_of(manager(), person);
        if c == 0 || read_u8(c + contract::CONTRACT_TYPE) & 0x3f != 2 {
            return 0;
        }
        return rest(c).clamp(1000, 1_000_000);
    }
    if read_u8(G_FIFA_TRANSFER_RULES_IN_FORCE) == 1 && sem_regras_novas == 0 {
        let c = contract_of(manager(), person);
        if c == 0 || read_u8(c + contract::CONTRACT_TYPE) & 0x3f != 2 {
            return 0;
        }
        let r = rest(c);
        let v = value(manager(), person, friendly_club_by_id(read_usize(c + contract::CLUB)), 1, 0xffff_ffff, 0);
        return v.wrapping_add(r) / 2;
    }
    let c = contract_of(manager(), person);
    if c == 0 {
        return 0;
    }
    let kind = read_u8(c + contract::CONTRACT_TYPE) & 0x3f;
    if kind == 5 {
        return 0;
    }
    let seller = friendly_club_by_id(read_usize(c + contract::CLUB));
    if read_usize(buyer + club::NATION) == 0 || read_usize(seller + club::NATION) == 0 {
        return 0;
    }
    if kind == 2 {
        copy(&mut data, &mut t, c + contract::CONTRACT_EXPIRES);
    } else {
        copy(&mut data, &mut t, G_CURRENT_DATE);
    }
    let n = read_usize(KEY_NATION_SWEDEN);
    if read_usize(read_usize(seller + club::NATION)) == n && read_usize(read_usize(buyer + club::NATION)) == n {
        return value(manager(), person, seller, 1, 0xffff_ffff, 0) / 2;
    }
    if age(person, &data) as u8 as i8 >= 0x18 {
        return 0;
    }
    if britanica(read_usize(buyer + club::NATION)) == 0 || britanica(read_usize(seller + club::NATION)) == 0 {
        return 0;
    }
    crate::comparison::branch("cpq_6 compensacao britanica");
    value(manager(), person, seller, 1, 0xffff_ffff, 0) / 2 // under 24 between British clubs
}

/// Adds the fee to the totals, the buyer pays and the seller receives the first instalment, stores the remaining
/// instalments (list +0x872) and the sell-on and appearance clauses, with the person's flags.
///
/// Ret 4: `TRANSFER_MANAGER::pay_fee_and_record_clauses(offer)`: adds the fee (+0x1b) to the transfer totals
/// (0x448ad0); the buyer (+8) pays and the seller (club of the contract of person +0xc, 0x4cf5a0) receives the first
/// instalment (the fee / instalments +0x40, or all of it), in 64 bits (0x599890, 0x599800). With instalments, stores
/// the remaining ones (13 bytes: the instalment, the ids of buyer and seller, instalments − 1) in list +0x872
/// (0x68d0d0) and adds to +0x1c of the finances the instalment × −5/21 (buyer) and × 5/21 (seller). Stores the clauses:
/// the next-sale percentage (+0x3f, 0x8caaf0, flag 8), the team appearance bonus (+0x35 with byte +0x39, 0x8cac40, flag
/// 2) and the national side appearance bonus (+0x3a with +0x3e, 0x8cad90, flag 4); the flags go in the person's 8-byte
/// record in `[this+0x86e]`.
/// Original: `TRANSFER_MANAGER::pay_fee_and_record_clauses` (0x8ca8a0, transfer_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x8ca8a0 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::pay_fee_and_record_clauses(oferta)`: soma o
/// valor (+0x1b) aos totais das transferências (0x448ad0); o comprador (+8) paga e o vendedor (clube do contrato da
/// pessoa +0xc, 0x4cf5a0) recebe a primeira parcela (o valor / parcelas +0x40, ou tudo), em 64 bits (0x599890,
/// 0x599800). Com parcelas, guarda as que faltam (13 bytes: a parcela, os ids do comprador e do vendedor, parcelas − 1)
/// na lista +0x872 (0x68d0d0) e soma ao +0x1c das finanças a parcela × −5/21 (comprador) e × 5/21 (vendedor). Guarda
/// as cláusulas: a porcentagem da próxima venda (+0x3f, 0x8caaf0, marca 8), o prêmio por jogo no time (+0x35 com o byte
/// +0x39, 0x8cac40, marca 2) e o prêmio por jogo pela seleção (+0x3a com +0x3e, 0x8cad90, marca 4); as marcas no
/// registro de 8 bytes da pessoa em [this+0x86e].
pub(crate) unsafe extern "thiscall" fn pay_fee_and_record_clauses(this: usize, offer: usize) {
    let finances: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FINANCETABLE_GET_CLUB_FINANCE);
    let totais: unsafe extern "thiscall" fn(usize, i32, usize, usize) -> u32 = core::mem::transmute(FN_CLUB_RECORDS_ADD_TRANSFER_TOTALS);
    let pay: unsafe extern "thiscall" fn(usize, i64) = core::mem::transmute(FN_FINANCE_PAY_PLAYERS_BOUGHT);
    let receber: unsafe extern "thiscall" fn(usize, i64) = core::mem::transmute(FN_FINANCE_ADD_PLAYERS_SOLD);
    let by_in_list: unsafe extern "thiscall" fn(usize, *const u8, u32) -> usize = core::mem::transmute(FN_LLIST_ADD_TAIL);
    let proxima_venda: unsafe extern "thiscall" fn(usize, *const u8) = core::mem::transmute(FN_TRANSFER_MANAGER_ADD_NEXT_SALE_PERCENTAGE);
    let prize_team: unsafe extern "thiscall" fn(usize, *const u8) = core::mem::transmute(FN_TRANSFER_MANAGER_ADD_FIRST_TEAM_APPEARANCE_MONEY);
    let prize_national_team: unsafe extern "thiscall" fn(usize, *const u8) = core::mem::transmute(FN_TRANSFER_MANAGER_ADD_INTERNATIONAL_TEAM_APPEARANCE_MONEY);
    let table = || read_usize(G_FINANCE_TABLE);
    let mark = |bit: u8| {
        let m = (read_usize(this + transfer_manager_fields::PERSON_FLAGS) + read_usize(offer + offer::TARGET_STAFF_ID) * 8) as *mut u8;
        *m |= bit;
    };
    let buyer = friendly_club_by_id(read_usize(offer + offer::BUYER_CLUB_ID));
    let seller = seller_club_of_person(staff_records_staff_by_id(read_usize(offer + offer::TARGET_STAFF_ID)));
    let fin_c = finances(table(), buyer);
    // simplification: without a seller, the original would read the finances from a place without a value (and would crash later, at [vendedor]); the
    // the copy pays no one
    let fin_v = if seller != 0 { finances(table(), seller) } else { 0 };
    let value = read_usize(offer + offer::FEE) as i32;
    let n = read_u8(offer + offer::INSTALMENTS) as i8;
    let first = if n > 0 { (value / n as i32) as i64 } else { value as i64 };
    totais(read_usize(G_CLUB_RECORDS), value, buyer, seller);
    crate::comparison::branch("cpq_6 pagar transferencia");
    pay(fin_c, first);
    if fin_v != 0 {
        receber(fin_v, first); // o vendedor recebe
    }
    let mut r = [0u8; 13];
    let n = read_u8(offer + offer::INSTALMENTS) as i8;
    if n > 0 {
        let parcela = read_usize(offer + offer::FEE) as i32 / n as i32;
        put_u32_le(&mut r, 0, parcela as usize);
        put_u32_le(&mut r, 4, read_usize(buyer));
        put_u32_le(&mut r, 8, read_usize(seller));
        r[12] = (n as u8).wrapping_sub(1);
        let fc = finances(table(), buyer);
        let fv = finances(table(), seller);
        if fc != 0 {
            let s = (fc + finance::INSTALMENT_BALANCE) as *mut i32;
            *s = (*s).wrapping_add(match_pl_ftol(parcela as f64 * read_f64_at(0x970420))); // 0x970420 = -0.2380952381
        }
        if fv != 0 {
            let s = (fv + finance::INSTALMENT_BALANCE) as *mut i32;
            *s = (*s).wrapping_add(match_pl_ftol(parcela as f64 * read_f64_at(0x970418))); // 0x970418 = 0.2380952381
        }
        by_in_list(this + transfer_manager_fields::INSTALMENTS, r.as_ptr(), 0xd);
    }
    let p = read_u8(offer + offer::SELL_ON_PERCENT);
    if (p as i8) > 0 {
        r[0] = p;
        put_u32_le(&mut r, 1, read_usize(buyer));
        put_u32_le(&mut r, 5, read_usize(seller));
        put_u32_le(&mut r, 9, read_usize(offer + offer::TARGET_STAFF_ID));
        proxima_venda(this, r.as_ptr());
        mark(8);
    }
    let v = read_usize(offer + offer::APPEARANCE_FEE) as i32;
    if v > 0 {
        put_u32_le(&mut r, 0, read_usize(offer + offer::TARGET_STAFF_ID));
        put_u32_le(&mut r, 4, v as usize);
        put_u32_le(&mut r, 9, read_usize(seller));
        r[8] = read_u8(offer + offer::APPEARANCE_GAMES);
        prize_team(this, r.as_ptr());
        mark(2);
    }
    let v = read_usize(offer + offer::INTERNATIONAL_FEE) as i32;
    if v > 0 {
        put_u32_le(&mut r, 0, read_usize(offer + offer::TARGET_STAFF_ID));
        put_u32_le(&mut r, 4, v as usize);
        put_u32_le(&mut r, 9, read_usize(seller));
        r[8] = read_u8(offer + offer::INTERNATIONAL_GAMES);
        prize_national_team(this, r.as_ptr());
        mark(4);
    }
}

/// Returns 1 when the fee covers an active release clause (reason 0x1b) or the seller in financial trouble gets far
/// more than the value (reason 0x1c); without a seller, 1.
///
/// Stdcall, ret 0xc; SEH frame only for the local values: `transfer_manager_selling_club_must_accept(offer, strict,
/// reason)`: without a seller (the club of the contract of person +0xc that is not of type 5, 0x4cf5a0), 1. With an
/// active release clause (0x867b50), without an exchange (+4) or instalments (+0x40) and the fee (+0x1b) not below the
/// clause amount (+0x21 of the contract; both through 0x43f460 with 4): reason 0x1b and 1. With the seller in crisis
/// (+0x165 of the finances = 1), the asking price (0x4d1600) not below 20 × the reputation (+0x80; at least 20000), the
/// offer's weighted value (0x8e7b50) above 50000 and double it above the price: reason 0x1c and 1 (with `strict`, only
/// if the price is below 1.3 × the weighted value). Otherwise 0.
/// Original: `transfer_manager_selling_club_must_accept` (0x8cd240, transfer_manager.cpp, stdcall, ret 0xc).
///
/// [pt] { 0x8cd240 } (stdcall, ret 0xc, transfer_manager.cpp, quadro SEH só dos valores locais):
/// `transfer_manager_selling_club_must_accept(oferta, estrito, motivo)`: sem vendedor (clube do contrato da pessoa
/// +0xc que não é de tipo 5, 0x4cf5a0), 1. Com cláusula de rescisão ativa (0x867b50), sem troca (+4) nem parcelas
/// (+0x40) e o valor (+0x1b) não abaixo da multa (+0x21 do contrato; os dois por 0x43f460 com 4): motivo 0x1b e 1. Com
/// o vendedor em crise (+0x165 das finanças = 1), o preço pedido (0x4d1600) não abaixo de 20 × a reputação (+0x80;
/// pelo menos 20000), o valor ponderado da oferta (0x8e7b50) acima de 50000 e o dobro dele acima do preço: motivo 0x1c
/// e 1 (com `estrito`, só se o preço ficar abaixo de 1,3 × o valor ponderado). Senão 0.
pub(crate) unsafe extern "stdcall" fn transfer_manager_selling_club_must_accept(offer: usize, estrito: u32, reason: *mut u16) -> u32 {
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let active: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_STAFF_CONTRACT_HAS_ACTIVE_RELEASE_CLAUSE);
    let exibir: unsafe extern "thiscall" fn(*const u32, u32) -> i64 = core::mem::transmute(FN_CASH_GET_DISPLAY_AMOUNT);
    let finances: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FINANCETABLE_GET_CLUB_FINANCE);
    let price: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, u32) -> i32 = core::mem::transmute(FN_CONTRACT_MANAGER_CALC_ASKING_PRICE);
    let ponderado: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> i32 = core::mem::transmute(FN_TRANSFER_OFFER_GET_WEIGHTED_VALUE);
    let buyer = friendly_club_by_id(read_usize(offer + offer::BUYER_CLUB_ID));
    let person = staff_records_staff_by_id(read_usize(offer + offer::TARGET_STAFF_ID));
    let seller = seller_club_of_person(person);
    if seller == 0 {
        return 1;
    }
    let c = contract_of(read_usize(G_CONTRACT_MANAGER), person);
    crate::comparison::branch("cpq_6 clube deve aceitar");
    if active(c) != 0 && read_usize(offer + offer::EXCHANGE_PLAYERS) == 0 && read_u8(offer + offer::INSTALMENTS) == 0 {
        let (multa, valor) = (read_usize(c + contract::RELEASE_FEE) as u32, read_usize(offer + offer::FEE) as u32);
        let m = exibir(&multa, 4);
        let v = exibir(&valor, 4);
        if v >= m {
            if !reason.is_null() {
                *reason = 0x1b;
            }
            return 1;
        }
    }
    if read_u8(finances(read_usize(G_FINANCE_TABLE), seller) + 0x165) != 1 {
        return 0;
    }
    let request = price(read_usize(G_CONTRACT_MANAGER), seller, buyer, person, 0, 1, 1);
    let w = ponderado(offer, 0, 0, 1, 0, 1);
    let floor = (*((seller + club::REPUTATION) as *const i16) as i32 * 20).max(20000);
    if request < floor || w.wrapping_mul(2) <= request || w <= 50000 {
        return 0;
    }
    if estrito != 0 {
        let w2 = ponderado(offer, 0, 0, 1, 0, 1);
        if !((request as f64) < w2 as f64 * read_f64_at(0x968de8)) { // 0x968de8 = 1.3
            return 0;
        }
    }
    if !reason.is_null() {
        *reason = 0x1c;
    }
    1
}

/// Comparador: paga, acrescenta nas listas (aloca) e marca: roda só a original.
// simplification: original only; the copy is proven by the branch and by the mutation in the career.

/// The work permit of `pessoa` (14 bytes, sorted by id: [this+0x93a], count +0x93e), looked up as the original does.
///
/// A person with an id outside [0xae2364] or without the flag (bit 0x20 of the 8-byte record in [this+0x86e]): 0;
/// otherwise the binary search (0x946464 with 0x8e55c0).
///
/// [pt] A permissão de trabalho da `pessoa` (14 bytes, ordenadas pelo id: [this+0x93a], conta +0x93e), como a original
/// procura: pessoa com id fora de [0xae2364] ou sem a marca (bit 0x20 do registro de 8 bytes em [this+0x86e]), 0; senão
/// a busca binária (0x946464 com 0x8e55c0).
pub(crate) unsafe fn find_work_permit_record(this: usize, person: usize) -> usize {
    if person != 0 && read_usize(person) as i32 >= read_usize(G_STAFF_COUNT) as i32 {
        return 0;
    }
    let id = read_usize(person) as u32;
    if read_u8(read_usize(this + transfer_manager_fields::PERSON_FLAGS) + id as usize * 8) & 0x20 == 0 {
        return 0;
    }
    let bsearch: unsafe extern "C" fn(*const u32, usize, usize, u32, usize) -> usize = core::mem::transmute(FN_CRT_BSEARCH);
    bsearch(&id, read_usize(this + transfer_manager_fields::WORK_PERMITS), read_usize(this + transfer_manager_fields::WORK_PERMIT_COUNT), 0xe, 0x8e55c0)
}

/// Updates the work permit on a move: when the new club needs a permit, updates the existing one (nation, year) or
/// grants a new one (0x8c7070); otherwise removes any permit and re-sorts the array.
///
/// Ret 8; SEH frame only for the local date: `TRANSFER_MANAGER::update_work_permit_on_move(person, club)`: if the
/// player needs a permit at the new club (0x8c7560 with 1) and already has one: replaces the permit's nation (+0xa)
/// with the club's (+0x53), zeroes +4 and +6 and, if the nation changed and neither has flag 0xe (+0x76), writes
/// today's year (+8); a club without a nation: the "Error" box 0x1dc2. Without a permit: the new one (0x8c7070 with
/// today's year, sorting), after today + 52 weeks (0x54c770, unused). If he does not need one (0x8c7560 is called
/// again) or without a club: removes whatever permit he has (flag 0x20, the last one in its place, count − 1 and
/// re-sorts with 0x945333).
/// Original: `TRANSFER_MANAGER::update_work_permit_on_move` (0x8c7e20, transfer_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x8c7e20 } (thiscall, ret 8, transfer_manager.cpp, quadro SEH só da data local):
/// `TRANSFER_MANAGER::update_work_permit_on_move(pessoa, clube)`: se o jogador precisa de permissão no clube novo
/// (0x8c7560 com 1) e já tem uma: troca a nação da permissão (+0xa) pela do clube (+0x53), zera +4 e +6 e, se mudou de
/// nação e as duas não são da marca 0xe (+0x76), grava o ano de hoje (+8); clube sem nação: a caixa "Error" 0x1dc2. Sem
/// permissão: a nova (0x8c7070 com o ano de hoje, ordenando), depois de hoje + 52 semanas (0x54c770, sem uso). Se não
/// precisa (a 0x8c7560 é chamada de novo) ou sem clube: tira a permissão que tiver (a marca 0x20, a última no lugar
/// dela, conta − 1 e reordena com 0x945333).
pub(crate) unsafe extern "thiscall" fn update_work_permit_on_move(this: usize, person: usize, club: usize) {
    let precisa: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_PLAYER_NEEDS_WORK_PERMIT);
    if club != 0 {
        if precisa(this, person, club, 1) != 0 {
            let r = find_work_permit_record(this, person);
            if r != 0 {
                let nation = read_usize(G_NATION_TABLE).wrapping_add(read_usize(r + work_permit::NATION_ID).wrapping_mul(nation::SIZE));
                let nc = read_usize(club + club::NATION);
                if nc == 0 {
                    show_error_at(0xa8d514, 0x1dc2);
                    return;
                }
                if nation != nc && !(read_u8(nation + nation::ACTUAL_REGION) == 0xe && read_u8(nc + nation::ACTUAL_REGION) == 0xe) {
                    core::ptr::write_unaligned((r + work_permit::YEAR) as *mut u16, *((0xae2c92) as *const u16));
                }
                core::ptr::write_unaligned((r + work_permit::MATCHES_PLAYED) as *mut u16, 0);
                core::ptr::write_unaligned((r + work_permit::MATCHES_MISSED) as *mut u16, 0);
                core::ptr::write_unaligned((r + work_permit::NATION_ID) as *mut u32, read_usize(nc) as u32);
                return;
            }
            let weeks: unsafe extern "thiscall" fn(usize, *mut [u32; 2], u32) -> usize = core::mem::transmute(FN_DATE_ADD_WEEKS);
            let give: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_ADD_WORK_PERMIT);
            let mut t = [0u32; 2];
            weeks(G_CURRENT_DATE, &mut t, 0x34);
            crate::comparison::branch("cpq_7 permissao nova");
            // simplification: the original pushes the year with garbage in the high 16 bits; 0x8c7070 only writes the word
            let year = *((0xae2c92) as *const u16) as u32;
            give(this, person, read_usize(club + club::NATION), year, 0, 1); // the new permission
            return;
        }
        if precisa(this, person, club, 1) != 0 {
            return;
        }
    }
    let r = find_work_permit_record(this, person);
    if r == 0 {
        return;
    }
    let mark = (read_usize(this + transfer_manager_fields::PERSON_FLAGS) + read_usize(person) * 8) as *mut u8;
    *mark &= 0xdf;
    let last = read_usize(this + transfer_manager_fields::WORK_PERMITS) + read_usize(this + transfer_manager_fields::WORK_PERMIT_COUNT) * 0xe - 0xe;
    core::ptr::copy(last as *const u8, r as *mut u8, 0xe);
    let n = read_usize(this + transfer_manager_fields::WORK_PERMIT_COUNT).wrapping_sub(1);
    core::ptr::write_unaligned((this + transfer_manager_fields::WORK_PERMIT_COUNT) as *mut usize, n);
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    qsort(read_usize(this + transfer_manager_fields::WORK_PERMITS), n as u32, 0xe, 0x8e55c0);
}

/// Completes an accepted contract offer: without the work permit, the news of the collapsed offer or the refused permit
/// and 0; otherwise signs (0x4cf720), the club pays the signing-on fee and 1.
///
/// Ret 4; SEH frame for the local offer: `TRANSFER_MANAGER::complete_accepted_contract_offer(offer)`: contract offer
/// (+0 the person, +4 the club, +8 the signing-on fee, +0x61 the role). If the player needs a permit at the club
/// (0x8c7560 with 1) and it is not granted (0x8c7950): with a type 5 contract (0x4cf5a0, stored in `[0xae1080]`), the
/// news of the collapsed offer (0x8de790 with a local TRANSFER_OFFER: 0x8e5680, ids at +8 and +0xc, +0x1f = 0, reason
/// 0x13; 0x8e5710); otherwise the refused permit news (0x8e4290); returns 0. Otherwise signs (0x4cf720) and, once
/// signed, the club pays the signing-on fee (0x5a0590, 0x5998e0) and returns 1.
/// Original: `TRANSFER_MANAGER::complete_accepted_contract_offer` (0x8c38b0, transfer_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x8c38b0 } (thiscall, ret 4, transfer_manager.cpp, quadro SEH da oferta local):
/// `TRANSFER_MANAGER::complete_accepted_contract_offer(oferta)`: oferta de contrato (+0 a pessoa, +4 o clube, +8 as
/// luvas, +0x61 o cargo). Se o jogador precisa de permissão no clube (0x8c7560 com 1) e ela não sai (0x8c7950): com
/// contrato de tipo 5 (0x4cf5a0, guardado em [0xae1080]), a notícia da oferta que caiu (0x8de790 com uma
/// TRANSFER_OFFER local: 0x8e5680, ids em +8 e +0xc, +0x1f = 0, motivo 0x13; 0x8e5710); senão a notícia da permissão
/// recusada (0x8e4290); devolve 0. Senão assina (0x4cf720) e, assinado, o clube paga as luvas (0x5a0590, 0x5998e0) e
/// devolve 1.
pub(crate) unsafe extern "thiscall" fn complete_accepted_contract_offer(this: usize, offer: usize) -> u32 {
    let precisa: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_PLAYER_NEEDS_WORK_PERMIT);
    let concedida: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_IS_WORK_PERMIT_GRANTED);
    let person = staff_records_staff_by_id(read_usize(offer));
    let club = friendly_club_by_id(read_usize(offer + contract_offer::CLUB_ID));
    if precisa(this, person, club, 1) != 0 && concedida(this, person, read_usize(club + club::NATION), offer, 0, 0) == 0 {
        let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
        let c = contract_of(read_usize(G_CONTRACT_MANAGER), person);
        *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
        if c != 0 && read_u8(c + contract::CONTRACT_TYPE) & 0x3f == 5 {
            let construct: unsafe extern "thiscall" fn(*mut u8) -> usize = core::mem::transmute(FN_TRANSFER_OFFER_TRANSFER_OFFER);
            let caiu: unsafe extern "thiscall" fn(usize, *mut u8, usize, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_POST_OFFER_COLLAPSED_NEWS);
            let destroy: unsafe extern "thiscall" fn(*mut u8) = core::mem::transmute(FN_TRANSFER_OFFER_DESTRUCTOR_TRANSFER_OFFER);
            let mut o = [0u8; 0x44];
            construct(o.as_mut_ptr());
            o[8..12].copy_from_slice(&(read_usize(club) as u32).to_le_bytes());
            o[0xc..0x10].copy_from_slice(&(read_usize(person) as u32).to_le_bytes());
            o[0x1f] = 0;
            caiu(this, o.as_mut_ptr(), club, 0x13);
            destroy(o.as_mut_ptr());
        } else {
            let recusada: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_POST_CONTRACT_PERMIT_REFUSED_NEWS);
            recusada(this, person, offer);
        }
        return 0;
    }
    let assinar: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_CONTRACT_MANAGER_SIGN_CONTRACT);
    // simplification: the original pushes the job in a register with garbage above; 0x4cf720 and 0x53bd10 only use the byte
    if assinar(read_usize(G_CONTRACT_MANAGER), offer, read_u8(offer + contract_offer::MARK) as u32) == 0 {
        return 0;
    }
    let finances: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FINANCETABLE_GET_CLUB_FINANCE);
    let luvas: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_FINANCE_PAY_SIGN_ON_FEE);
    let value = read_usize(offer + 8) as u32;
    let fin = finances(read_usize(G_FINANCE_TABLE), club);
    crate::comparison::branch("cpq_7 luvas");
    luvas(fin, value); // the signing-on fee
    1
}

/// Builds the club's contract (0x4cda70, factor 1) and adds the contract offer; without one, withdraws the offer (main
/// player) or drops the player from the exchange adding his value.
///
/// Ret 0xc; SEH frame for the local contract: `TRANSFER_MANAGER::offer_contract_to_offer_player(club, person, offer)`:
/// builds the contract (local STAFF_CONTRACT, 0x85e410) the club would offer (0x4cda70 with the offer's slot +0x20,
/// wage factor 1 and mode 2). Built: adds the contract offer (0x4cc490 with slot +0x20, the value +0x24, 0xb, 0, 1, 0).
/// Without a contract: if he is the main player (+0xc), updates the club's list (0x849ce0 and 0x843e10 with 1) and
/// withdraws the offer (0x8bfcb0, reason 0x16); if he is from the exchange, adds his value (+0x52) to the offer's
/// (+0x1b) and takes him out of the exchange (0x8c41c0 with 1 and 0x16). Returns what the last call returned.
/// Original: `TRANSFER_MANAGER::offer_contract_to_offer_player` (0x8c1e40, transfer_manager.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x8c1e40 } (thiscall, ret 0xc, transfer_manager.cpp, quadro SEH do contrato local):
/// `TRANSFER_MANAGER::offer_contract_to_offer_player(clube, pessoa, oferta)`: monta o contrato (STAFF_CONTRACT local,
/// 0x85e410) que o clube ofereceria (0x4cda70 com a vaga +0x20 da oferta, fator de salário 1 e modo 2). Montado:
/// acrescenta a oferta de contrato (0x4cc490 com a vaga +0x20, o valor +0x24, 0xb, 0, 1, 0). Sem contrato: sendo o
/// jogador principal (+0xc), atualiza a lista do clube (0x849ce0 e 0x843e10 com 1) e retira a oferta (0x8bfcb0, motivo
/// 0x16); sendo da troca, soma o valor dele (+0x52) ao da oferta (+0x1b) e o tira da troca (0x8c41c0 com 1 e 0x16).
/// Devolve o que a última chamada devolveu.
pub(crate) unsafe extern "thiscall" fn offer_contract_to_offer_player(this: usize, club: usize, person: usize, offer: usize) -> u32 {
    let construct: unsafe extern "thiscall" fn(*mut u8) -> usize = core::mem::transmute(FN_STAFF_CONTRACT_STAFF_CONTRACT);
    let oferecer: unsafe extern "thiscall" fn(usize, usize, usize, *mut u8, usize, f64, u32) -> u32 = core::mem::transmute(FN_CONTRACT_MANAGER_OFFER_CONTRACT);
    let mut c = [0u8; 0x50];
    construct(c.as_mut_ptr());
    crate::comparison::branch("cpq_7 contrato da oferta");
    if oferecer(read_usize(G_CONTRACT_MANAGER), person, club, c.as_mut_ptr(), read_usize(offer + offer::SLOT_INDEX), 1.0f64, 2) != 0 {
        let append: unsafe extern "thiscall" fn(usize, *mut u8, usize, usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_CONTRACT_MANAGER_ADD_CONTRACT_OFFER);
        return append(read_usize(G_CONTRACT_MANAGER), c.as_mut_ptr(), read_usize(offer + offer::SLOT_INDEX), read_usize(offer + offer::SERIAL), 0xb, 0, 1, 0);
    }
    if read_usize(person) == read_usize(offer + offer::TARGET_STAFF_ID) {
        let entry: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(FN_SHORTLIST_MANAGER_FIND_CLUB_SHORTLIST_ENTRY);
        let atualizar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(FN_SHORTLIST_MANAGER_UPDATE_STAFF_STATUS);
        let retirar: unsafe extern "thiscall" fn(usize, *mut usize, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_WITHDRAW_TRANSFER_OFFER);
        let e = entry(read_usize(this + transfer_manager_fields::SHORTLIST_MANAGER), club, person, (read_u8(offer + offer::JOB_TYPE) != 0xb) as u32, 0);
        if e != 0 {
            // simplification: the original pushes the type (setne) with the rest of the entry pointer above; 0x843e10 only reads the byte
            atualizar(read_usize(this + transfer_manager_fields::SHORTLIST_MANAGER), person, club, 1, (read_u8(e) != 0xb) as u32);
        }
        let mut o = offer;
        return retirar(this, &mut o, 0, 0x16);
    }
    let v = (offer + offer::FEE) as *mut i32;
    core::ptr::write_unaligned(v, core::ptr::read_unaligned(v).wrapping_add(read_usize(person + staff::VALUE) as i32));
    let remove_of_swap: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_REMOVE_EXCHANGE_PLAYER);
    remove_of_swap(read_usize(G_TRANSFER_MANAGER), person, offer, 1, 0x16)
}

/// Comparador: grava as permissões (pode crescer o vetor e reordena): roda só a original.
// simplification: original only; the copy is proven by the branch and by the mutation in the career.

/// Comparador: assina o contrato (aloca), paga e posta notícias: roda só a original.
// simplification: original only; the copy is proven by the branch and by the mutation in the career.

/// Comparador: acrescenta ofertas de contrato (aloca), mexe nas listas e retira ofertas: roda só a original.
// simplification: original only; the copy is proven by the branch and by the mutation in the career.

/// Offer -> contract of its target person (+0xc), stored in G_CONTRACT_LOOKUP_RESULT.
///
/// [pt] O contrato da pessoa da oferta (+0xc; 0x4cf5a0 com o gerente [0xae1050]), guardado em [0xae1080].
pub(crate) unsafe fn offer_target_contract(offer: usize) -> usize {
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = contract_of(read_usize(G_CONTRACT_MANAGER), staff_records_staff_by_id(read_usize(offer + offer::TARGET_STAFF_ID)));
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
    c
}

/// Offer -> club of the target's contract unless type 5.
///
/// [pt] O clube do contrato da pessoa da oferta; contrato de tipo 5 (+0x35 & 0x3f) não conta.
pub(crate) unsafe fn offer_target_seller_club(offer: usize) -> usize {
    let c = offer_target_contract(offer);
    if c != 0 && read_u8(c + contract::CONTRACT_TYPE) & 0x3f != 5 {
        friendly_club_by_id(read_usize(c + contract::CLUB))
    } else {
        0
    }
}

/// Rewinds a LLIST cursor (+0xa = +2, first node).
///
/// [pt] Recomeça o cursor da LLIST (+0xa = +2, o primeiro nó).
pub(crate) unsafe fn llist_restart(list: usize) {
    core::ptr::write_unaligned((list + llist::CURSOR) as *mut usize, read_usize(list + llist::HEAD));
}

/// Offer status that lets the person's mark 1 clear: 2, 3, 4, 0x10, 0x14, or 0xf with +0x2e >= 3.
///
/// [pt] O estado da oferta (+0x2c) que deixa a marca 1 da pessoa sair: 2, 3, 0x14, 0x10, 4 e 0xf com +0x2e ≥ 3.
pub(crate) unsafe fn is_offer_state_closed(offer: usize) -> bool {
    match read_u8(offer + offer::STATUS) {
        2 | 3 | 0x14 | 0x10 | 4 => true,
        0xf => read_u8(offer + offer::RESPONSE_ROUND) >= 3,
        _ => false,
    }
}

/// Collapses the offer: state 0x10, closes the proposals, releases the bids, posts the news, schedules the removal,
/// clears the flags, the contract offer and the exchange contracts.
///
/// Ret 0x10: `TRANSFER_MANAGER::collapse_offer(offer, club, reason, remove_future)`: in an offer in state 0x13 clears
/// flag 0x10 of the person (+0xc) and of the exchange players (list +4) and, with `remove_future`, removes the future
/// transfer (0x8cbf70). Sets state 0x10, closes the proposals (0x8c6db0), releases the buyer's (+8) bids for the person
/// and the seller's (contract club, stored in `[0xae1080]`) for the exchange players (0x84aeb0), posts the news
/// (0x8de790 with the club and the reason) and schedules the offer (0x8c6220 with 0x14). With the offer and all those
/// of list +0 closed (states of `is_offer_state_closed`), clears flag 1 of the person. Updates the buyer's list
/// (0x849ce0 and 0x843e10 with 1), removes the contract offer (0x4cca70), zeroes +0x10 and releases the exchange
/// contracts (0x4ccb30 with slot +0x20; +4 of the item = 0).
/// Original: `TRANSFER_MANAGER::collapse_offer` (0x8c44e0, transfer_manager.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x8c44e0 } (thiscall, ret 0x10, transfer_manager.cpp): `TRANSFER_MANAGER::collapse_offer(oferta, clube, motivo,
/// tirar_futura)`: na oferta de estado 0x13 tira a marca 0x10 da pessoa (+0xc) e dos jogadores da troca (lista +4) e,
/// com `tirar_futura`, tira a transferência futura (0x8cbf70). Põe o estado 0x10, fecha as propostas (0x8c6db0), solta
/// os lances do comprador (+8) pela pessoa e do vendedor (clube do contrato, guardado em [0xae1080]) pelos da troca
/// (0x84aeb0), posta a notícia (0x8de790 com o clube e o motivo) e agenda a oferta (0x8c6220 com 0x14). Encerrada a
/// oferta e todas as da lista +0 (estados de `is_offer_state_closed`), tira a marca 1 da pessoa. Atualiza a lista do
/// comprador (0x849ce0 e 0x843e10 com 1), tira a oferta de contrato (0x4cca70), zera +0x10 e solta os contratos da troca
/// (0x4ccb30 com a vaga +0x20; +4 do item = 0).
pub(crate) unsafe extern "thiscall" fn collapse_offer(this: usize, offer: usize, club: usize, reason: u32, remove_future: u32) {
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let unmark: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_SHORTLIST_MANAGER_CLEAR_BID_MARKERS_FOR_PLAYER);
    let mark = |id: usize| (read_usize(this + transfer_manager_fields::PERSON_FLAGS) + id * 8) as *mut u8;
    crate::comparison::branch("cpq_8 desfazer oferta");
    if read_u8(offer + offer::STATUS) == 0x13 {
        *mark(read_usize(offer + offer::TARGET_STAFF_ID)) &= 0xef;
        if read_usize(offer + offer::EXCHANGE_PLAYERS) != 0 {
            llist_restart(read_usize(offer + offer::EXCHANGE_PLAYERS));
            loop {
                let it = next(read_usize(offer + offer::EXCHANGE_PLAYERS));
                if it == 0 {
                    break;
                }
                *mark(read_usize(it)) &= 0xef;
            }
        }
        if remove_future != 0 {
            let remove: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_REMOVE_FUTURE_TRANSFER);
            remove(this, offer);
        }
    }
    *((offer + offer::STATUS) as *mut u8) = 0x10;
    let close: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_CLOSE_OFFER_CONTRACT_OFFERS);
    close(this, offer);
    let person = staff_records_staff_by_id(read_usize(offer + offer::TARGET_STAFF_ID));
    let c = offer_target_contract(offer);
    let seller = if c != 0 { friendly_club_by_id(read_usize(c + contract::CLUB)) } else { 0 };
    unmark(read_usize(this + transfer_manager_fields::SHORTLIST_MANAGER), friendly_club_by_id(read_usize(offer + offer::BUYER_CLUB_ID)), person);
    if read_usize(offer + offer::EXCHANGE_PLAYERS) != 0 {
        llist_restart(read_usize(offer + offer::EXCHANGE_PLAYERS));
        loop {
            let it = next(read_usize(offer + offer::EXCHANGE_PLAYERS));
            if it == 0 {
                break;
            }
            unmark(read_usize(this + transfer_manager_fields::SHORTLIST_MANAGER), seller, staff_records_staff_by_id(read_usize(it)));
        }
    }
    let news: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_POST_OFFER_COLLAPSED_NEWS);
    let schedule: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_SCHEDULE_OFFER);
    news(this, offer, club, reason);
    schedule(this, offer, 0x14, 0);
    if is_offer_state_closed(offer) {
        let mut all = true;
        if read_usize(offer) != 0 {
            llist_restart(read_usize(offer));
            loop {
                let it = next(read_usize(offer));
                if it == 0 {
                    break;
                }
                if !is_offer_state_closed(read_usize(read_usize(this) + read_usize(it) * 4)) {
                    all = false;
                    break;
                }
            }
        }
        if all {
            let m = mark(read_usize(offer + offer::TARGET_STAFF_ID));
            if *m & 1 != 0 {
                *m &= 0xfe;
            }
        }
    }
    let entry: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(FN_SHORTLIST_MANAGER_FIND_CLUB_SHORTLIST_ENTRY);
    let atualizar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(FN_SHORTLIST_MANAGER_UPDATE_STAFF_STATUS);
    let e = entry(read_usize(this + transfer_manager_fields::SHORTLIST_MANAGER), friendly_club_by_id(read_usize(offer + offer::BUYER_CLUB_ID)), staff_records_staff_by_id(read_usize(offer + offer::TARGET_STAFF_ID)), (read_u8(offer + offer::JOB_TYPE) != 0xb) as u32, 0);
    if e != 0 {
        // simplification: the original pushes the type (setne) with garbage in the high bytes; 0x843e10 only reads the byte
        atualizar(read_usize(this + transfer_manager_fields::SHORTLIST_MANAGER), staff_records_staff_by_id(read_usize(offer + offer::TARGET_STAFF_ID)), friendly_club_by_id(read_usize(offer + offer::BUYER_CLUB_ID)), 1, (read_u8(e) != 0xb) as u32);
    }
    let tirar_contrato: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_CONTRACT_MANAGER_REMOVE_CONTRACT_OFFER);
    tirar_contrato(read_usize(G_CONTRACT_MANAGER), staff_records_staff_by_id(read_usize(offer + offer::TARGET_STAFF_ID)), friendly_club_by_id(read_usize(offer + offer::BUYER_CLUB_ID))); // the contract offer goes out
    *((offer + offer::TERMS_STATE) as *mut u8) = 0;
    if read_usize(offer + offer::EXCHANGE_PLAYERS) != 0 {
        let soltar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_CONTRACT_MANAGER_REMOVE_CONTRACT_OFFER_FOR_TRANSFER);
        llist_restart(read_usize(offer + offer::EXCHANGE_PLAYERS));
        loop {
            let it = next(read_usize(offer + offer::EXCHANGE_PLAYERS));
            if it == 0 {
                break;
            }
            soltar(read_usize(G_CONTRACT_MANAGER), staff_records_staff_by_id(read_usize(it)), read_usize(offer + offer::SLOT_INDEX));
            *((it + exchange_player::TERMS_STATE) as *mut u8) = 0;
        }
    }
}

/// builds the loan offer and the contract from the current one (wage percentage), adds both and returns the offer slot;
/// turned off (DESLIGADAS) from 2026-10-30 to 2026-10-31, when the test criterion changed and the list was emptied: with the detour the career in Rust mode left the reference be54a138 (bisection of
/// the 38 routines of batch 253); the original builds the offer and the contract in locals the constructors (0x8e5680,
/// 0x85e410) do not fully fill, and the stack residue of those bytes goes into the stored offer, where the copy puts
/// zero; the copy stays in the code and does not run
/// Original: `TRANSFER_MANAGER::make_loan_offer` (0x8bed60, transfer_manager.cpp, thiscall, ret 0x18).
///
/// [pt] { 0x8bed60 } (thiscall, ret 0x18, transfer_manager.cpp, quadro SEH da oferta local):
/// `TRANSFER_MANAGER::make_loan_offer(jogador, clube, fim, percentual, a5, a6)`: numa TRANSFER_OFFER local (0x8e5680)
/// monta a oferta de empréstimo (0x8e7810) e, num STAFF_CONTRACT local (0x85e410), o contrato pelo atual (0x85e590 com o
/// fim, a porcentagem do salário e as duas cláusulas); acrescenta a oferta (0x8be300 com o contrato) e a oferta de
/// contrato (0x4cc490 com a vaga +0x20 e o valor +0x24 da oferta, 0xb, 0, 1, 0). Destrói a oferta local (0x8e5710) e
/// devolve a vaga (+0x20).
pub(crate) unsafe extern "thiscall" fn make_loan_offer(this: usize, player: usize, club: usize, end: usize, write_percent_pair: u32, a5: u32, a6: u32) -> u32 {
    let construct: unsafe extern "thiscall" fn(*mut u8) -> usize = core::mem::transmute(FN_TRANSFER_OFFER_TRANSFER_OFFER);
    let loan: unsafe extern "thiscall" fn(*mut u8, usize, usize) = core::mem::transmute(FN_TRANSFER_OFFER_INIT_LOAN_OFFER);
    let construct_contract: unsafe extern "thiscall" fn(*mut u8) -> usize = core::mem::transmute(FN_STAFF_CONTRACT_STAFF_CONTRACT);
    let by_current: unsafe extern "thiscall" fn(*mut u8, usize, usize, usize, u32, u32, u32) = core::mem::transmute(FN_STAFF_CONTRACT_INIT_FROM_CURRENT_CONTRACT);
    let append: unsafe extern "thiscall" fn(usize, *mut u8, *mut u8) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_ADD_TRANSFER_OFFER);
    let append_contract: unsafe extern "thiscall" fn(usize, *mut u8, u32, u32, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_CONTRACT_MANAGER_ADD_CONTRACT_OFFER);
    let destroy: unsafe extern "thiscall" fn(*mut u8) = core::mem::transmute(FN_TRANSFER_OFFER_DESTRUCTOR_TRANSFER_OFFER);
    let field = |o: &[u8; 0x44], k: usize| u32::from_le_bytes([o[k], o[k + 1], o[k + 2], o[k + 3]]);
    let mut o = [0u8; 0x44];
    construct(o.as_mut_ptr());
    loan(o.as_mut_ptr(), player, club);
    let mut c = [0u8; 0x50];
    construct_contract(c.as_mut_ptr());
    crate::comparison::branch("cpq_8 oferta de emprestimo");
    by_current(c.as_mut_ptr(), player, club, end, write_percent_pair, a5, a6); // the salary by percentage
    append(this, o.as_mut_ptr(), c.as_mut_ptr());
    append_contract(read_usize(G_CONTRACT_MANAGER), c.as_mut_ptr(), field(&o, 0x20), field(&o, 0x24), 0xb, 0, 1, 0);
    let slot = field(&o, 0x20);
    destroy(o.as_mut_ptr());
    slot
}

/// Processes the future transfers whose date has come: collapses those the restrictions block, completes the others
/// (0x8c9390) and removes them from the array.
///
/// No arguments: `TRANSFER_MANAGER::process_due_future_transfers()`: goes through the future transfers (12 bytes: +0
/// the date, +8 the offer index; `[this+0x920]`, count +0x928) up to the first after today (0x456740). Null offer: the
/// "Error" box 0x2846. With the permits (0x8c7690 with 1): if the restrictions (0x8cfc90) block it for the buyer or, if
/// there is a seller, for the seller (reason 9 does not count), collapses the offer (0x8c44e0 with the club and the
/// reason); otherwise the news (0x8dbef0), completes it (0x8c9390), state 0x14 and schedules (0x8c6220 with 0xa). Then
/// removes the processed ones from the start of the vector (0x8cbf40 one by one) and decreases the count.
/// Original: `TRANSFER_MANAGER::process_due_future_transfers` (0x8cbbe0, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8cbbe0 } (thiscall, sem argumentos, transfer_manager.cpp): `TRANSFER_MANAGER::process_due_future_transfers()`:
/// percorre as transferências futuras (12 bytes: +0 a data, +8 o índice da oferta; [this+0x920], conta +0x928) até a
/// primeira depois de hoje (0x456740). Oferta nula: a caixa "Error" 0x2846. Com as permissões (0x8c7690 com 1): se as
/// restrições (0x8cfc90) seguram pelo comprador ou, havendo vendedor, pelo vendedor (motivo 9 não conta), desfaz a
/// oferta (0x8c44e0 com o clube e o motivo); senão a notícia (0x8dbef0), conclui (0x8c9390), estado 0x14 e agenda
/// (0x8c6220 com 0xa). Depois tira as processadas do começo do vetor (0x8cbf40 uma a uma) e diminui a conta.
pub(crate) unsafe extern "thiscall" fn process_due_future_transfers(this: usize) {
    let after: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_DATE_OPERATOR_3E);
    let permissoes: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_CHECK_OFFER_WORK_PERMITS);
    let restricoes: unsafe extern "thiscall" fn(usize, usize, usize, *mut u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_CHECK_OFFER_RESTRICTIONS);
    let desfazer: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_COLLAPSE_OFFER);
    let news: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_POST_TRANSFER_COMPLETED_NEWS);
    let concluir: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_FINALISE_TRANSFER);
    let schedule: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_SCHEDULE_OFFER);
    // the reason for the restrictions: only the low word is written; the high 16 bits stay 0, as in the original
    let mut reason = 0u32;
    let mut feitas = 0i32;
    while feitas < read_usize(this + transfer_manager_fields::FUTURE_TRANSFER_COUNT) as i32 {
        let e = read_usize(this + transfer_manager_fields::FUTURE_TRANSFERS) + feitas as usize * 0xc;
        if after(e, read_usize(G_CURRENT_DATE) as u32, read_usize(G_CURRENT_DATE_LEAP) as u32) != 0 {
            break;
        }
        let offer = read_usize(read_usize(this) + read_usize(e + future_transfer::OFFER_SLOT) * 4);
        if offer == 0 {
            show_error_at(0xa8d514, 0x2846);
        } else if permissoes(this, offer, 1) != 0 {
            let buyer = friendly_club_by_id(read_usize(offer + offer::BUYER_CLUB_ID));
            let mut segura = false;
            if restricoes(this, offer, buyer, &mut reason, 0) as u8 != 0 && reason as u16 != 9 {
                desfazer(this, offer, buyer, reason, 0);
                segura = true;
            } else if offer_target_seller_club(offer) != 0 {
                let v = offer_target_seller_club(offer);
                if restricoes(this, offer, v, &mut reason, 0) as u8 != 0 && reason as u16 != 9 {
                    let v = offer_target_seller_club(offer);
                    desfazer(this, offer, v, reason, 0);
                    segura = true;
                }
            }
            if !segura {
                news(this, offer);
                crate::comparison::branch("cpq_8 transferencia futura");
                concluir(this, offer); // the transfer happens
                *((offer + offer::STATUS) as *mut u8) = 0x14;
                schedule(this, offer, 0xa, 0);
            }
        }
        feitas += 1;
    }
    if feitas < 0 {
        show_error_at(0xa8d514, 0x288a);
    }
    if feitas == 0 {
        return;
    }
    let mut k = feitas;
    if k < read_usize(this + transfer_manager_fields::FUTURE_TRANSFER_COUNT) as i32 {
        let copy: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FUTURE_TRANSFER_ASSIGN);
        while k < read_usize(this + transfer_manager_fields::FUTURE_TRANSFER_COUNT) as i32 {
            let v = read_usize(this + transfer_manager_fields::FUTURE_TRANSFERS);
            copy(v + (k - feitas) as usize * 0xc, v + k as usize * 0xc);
            k += 1;
        }
    }
    let n = (this + transfer_manager_fields::FUTURE_TRANSFER_COUNT) as *mut i32;
    core::ptr::write_unaligned(n, core::ptr::read_unaligned(n).wrapping_sub(feitas));
}

/// Comparador: muda o estado, as listas e as marcas, posta notícia e agenda: roda só a original.
// simplification: original only; the copy is proven by the branch and by the mutation in the career.

/// Comparador: acrescenta a oferta e a oferta de contrato (alocam): roda só a original.
// simplification: original only; the copy is proven by the branch and by the mutation in the career.

/// Comparador: conclui e desfaz transferências (elencos, dinheiro, notícias): roda só a original.
// simplification: original only; the copy is proven by the branch and by the mutation in the career.

/// A club rejects the offer: counts the offer's attempt; by the refusing side, a random(2 × attempts) rejects or
/// withdraws the offer, otherwise it is rescheduled with the delay news.
///
/// Ret 0xc: `TRANSFER_MANAGER::club_rejects_offer(offer, club, reason)`: outside states 0x11/0x12 (+0x2c), counts the
/// attempt (+0x2e). Refusal by the selling club (id = +8): state 0x12; reason 8 (16 bits) reschedules (0x8c6220 with 7,
/// 0) and exits; otherwise, with no human club in the player's contract (+0xc; contract in `[0xae1080]`, +0x35 & 0x3f ≠
/// 5, 0x600a50) and more than one attempt, a random(2 × attempts) different from 0 rejects the offer (0x8bf680 with the
/// offer's address, 0, 6) and exits. Refusal by the other side: state 0x11, unsettles the player (0x8c1290); with a
/// computer selling club and the random draw, withdraws the offer (0x8bfcb0); with reason 8 the rest reschedules and
/// exits (without the attempts test). At the end: reschedules and gives the delay news (0x8dda90 with the whole reason,
/// as the original pushes it).
/// Original: `TRANSFER_MANAGER::club_rejects_offer` (0x8c36b0, transfer_manager.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x8c36b0 } (thiscall, ret 0xc, transfer_manager.cpp): `TRANSFER_MANAGER::club_rejects_offer(oferta, clube,
/// motivo)`: fora dos estados 0x11/0x12 (+0x2c), conta a tentativa (+0x2e). Recusa do clube vendedor (id = +8): estado
/// 0x12; motivo 8 (16 bits) reagenda (0x8c6220 com 7, 0) e sai; senão, sem clube humano no contrato do jogador
/// (+0xc; contrato em [0xae1080], +0x35 & 0x3f ≠ 5, 0x600a50) e com mais de uma tentativa, um sorteio(2 × tentativas)
/// diferente de 0 rejeita a oferta (0x8bf680 com o endereço da oferta, 0, 6) e sai. Recusa do outro lado: estado
/// 0x11, desassossega o jogador (0x8c1290); com o clube vendedor do computador e o sorteio, retira a oferta (0x8bfcb0);
/// com motivo 8 o resto reagenda e sai (sem o teste de tentativas). No fim: reagenda e dá a notícia do adiamento
/// (0x8dda90 com o motivo inteiro, como a original o empurra).
pub(crate) unsafe extern "thiscall" fn club_rejects_offer(this: usize, offer: usize, club: usize, reason: u32) {
    let state = read_u8(offer + offer::STATUS);
    if state == 0x11 || state == 0x12 {
        return;
    }
    crate::comparison::branch("cpp_7 recusa");
    *((offer + offer::RESPONSE_ROUND) as *mut u8) = read_u8(offer + offer::RESPONSE_ROUND).wrapping_add(1); // one more attempt
    let schedule: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_SCHEDULE_OFFER);
    let human: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    // the original passes the address of its own argument
    let mut o = offer;
    if read_usize(club) == read_usize(offer + offer::BUYER_CLUB_ID) {
        *((offer + offer::STATUS) as *mut u8) = 0x12;
        if reason as u16 == 8 {
            schedule(this, offer, 7, 0);
            return;
        }
        let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
        let c = contract_of(read_usize(G_CONTRACT_MANAGER), staff_records_staff_by_id(read_usize(offer + offer::TARGET_STAFF_ID)));
        *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
        let mut in_end = false;
        if c != 0 && read_u8(c + contract::CONTRACT_TYPE) & 0x3f != 5 {
            let cl = friendly_club_by_id(read_usize(c + contract::CLUB));
            in_end = cl != 0 && human(cl, 0, 1, 0, 0) != 0;
        }
        if !in_end {
            let n = read_u8(offer + offer::RESPONSE_ROUND);
            if n > 1 && sorteio(n as i32 * 2) != 0 {
                let rejeitar: unsafe extern "thiscall" fn(usize, *mut usize, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_REJECT_TRANSFER_OFFER);
                rejeitar(this, &mut o, 0, 6);
                return;
            }
        }
    } else {
        *((offer + offer::STATUS) as *mut u8) = 0x11;
        let desassossegar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_UNSETTLE_PLAYER_AFTER_BID);
        desassossegar(this, offer);
        let retirar: unsafe extern "thiscall" fn(usize, *mut usize, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_WITHDRAW_TRANSFER_OFFER);
        let cl = friendly_club_by_id(read_usize(offer + offer::BUYER_CLUB_ID));
        if reason as u16 == 8 {
            if human(cl, 0, 1, 0, 0) == 0 && sorteio(read_u8(offer + offer::RESPONSE_ROUND) as i32 * 2) != 0 {
                retirar(this, &mut o, 0, 6);
                return;
            }
            schedule(this, offer, 7, 0);
            return;
        }
        if human(cl, 0, 1, 0, 0) == 0 {
            let n = read_u8(offer + offer::RESPONSE_ROUND);
            if n > 1 && sorteio(n as i32 * 2) != 0 {
                retirar(this, &mut o, 0, 6);
                return;
            }
        }
    }
    let news: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_POST_TRANSFER_DELAY_REQUEST_NEWS);
    schedule(this, offer, 7, 0);
    news(this, offer, reason);
}

/// Takes the offer's player and each exchange player out of other offers and collapses the offers for each exchange
/// player.
///
/// Ret 4; SEH frame for the local list: `TRANSFER_MANAGER::detach_offer_players(offer)`: builds a local LLIST it does
/// not use (0x68d020, destroyed at the end by 0x68d040), takes the offer's player (+0xc; 0x8c41c0 with the offer, 0, 0)
/// out of the other offers and, through the exchange player list (+4, reread on each call, cursor restarted), takes out
/// each one (0x8c41c0) and collapses the offers for him (0x8c47d0). The exchange player's id is reread from the item
/// after 0x8c41c0, as in the original.
/// Original: `TRANSFER_MANAGER::detach_offer_players` (0x8c6ae0, transfer_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x8c6ae0 } (thiscall, ret 4, transfer_manager.cpp, quadro SEH da lista local): `TRANSFER_MANAGER::
/// detach_offer_players(oferta)`: constrói uma LLIST local que não usa (0x68d020, destruída no fim por 0x68d040), tira
/// das outras ofertas o jogador da oferta (+0xc; 0x8c41c0 com a oferta, 0, 0) e, pela lista dos jogadores de troca
/// (+4, relida a cada chamada, cursor recomeçado), tira cada um (0x8c41c0) e desfaz as ofertas por ele (0x8c47d0). O
/// id do jogador de troca é relido do item depois da 0x8c41c0, como na original.
pub(crate) unsafe extern "thiscall" fn detach_offer_players(this: usize, offer: usize) {
    let list_new: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let destroy: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let remove: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_REMOVE_EXCHANGE_PLAYER);
    let desfazer: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_COLLAPSE_OFFERS_FOR_PLAYER);
    let mut local = [0u32; 4]; // the original’s LLIST (0xe bytes)
    list_new(local.as_mut_ptr() as usize);
    crate::comparison::branch("cpp_7 soltar jogadores");
    remove(this, staff_records_staff_by_id(read_usize(offer + offer::TARGET_STAFF_ID)), offer, 0, 0); // the offer’s player
    let swaps = read_usize(offer + offer::EXCHANGE_PLAYERS);
    if swaps != 0 {
        core::ptr::write_unaligned((swaps + llist::CURSOR) as *mut usize, read_usize(swaps + llist::HEAD));
        let mut item = next(read_usize(offer + offer::EXCHANGE_PLAYERS));
        while item != 0 {
            remove(this, staff_records_staff_by_id(read_usize(item)), offer, 0, 0);
            desfazer(this, staff_records_staff_by_id(read_usize(item)));
            item = next(read_usize(offer + offer::EXCHANGE_PLAYERS));
        }
    }
    destroy(local.as_mut_ptr() as usize);
}

/// Allocates and initialises a cash offer for the player at the given fee (0x8e78f0), stores it in the output, appeal
/// 50, and returns 1.
///
/// Ret 0x14; SEH frame only for the new offer: `TRANSFER_MANAGER::build_cash_offer_option(club, player, out,
/// attraction, value)`: allocates the offer (0x944cf1(0x41) and 0x8e5680), stores it in `out`, initialises it with the
/// player, the buying club and the value (0x8e78f0, which may draw random numbers) and sets the attraction to 50. A
/// club without finances (0x5a0590 = 0): the original routine ("Error" box).
/// Original: `TRANSFER_MANAGER::build_cash_offer_option` (0x8c8740, transfer_manager.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x8c8740 } (thiscall, ret 0x14, transfer_manager.cpp, quadro SEH só para a oferta nova):
/// `TRANSFER_MANAGER::build_cash_offer_option(clube, jogador, saída, atração, valor)`: aloca a oferta (0x944cf1(0x41) e
/// 0x8e5680), guarda em `saída`, inicia pelo jogador, o clube comprador e o valor (0x8e78f0, que pode sortear) e põe a
/// atração em 50. Clube sem finanças (0x5a0590 = 0): a original (caixa "Error").
pub(crate) unsafe extern "thiscall" fn build_cash_offer_option(this: usize, club: usize, player: usize, out: *mut usize, atracao: *mut u8, value: i32) -> u32 {
    let finances: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FINANCETABLE_GET_CLUB_FINANCE);
    // simplification: 0x5a0590 only searches; queried before allocating so that the error case goes entirely through the original
    if finances(read_usize(G_FINANCE_TABLE), club) == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, *mut usize, *mut u8, i32) -> u32>(IDX_BUILD_CASH_OFFER_OPTION)(this, club, player, out, atracao, value);
    }
    let new: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let construct: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_TRANSFER_OFFER_TRANSFER_OFFER);
    let iniciar: unsafe extern "thiscall" fn(usize, usize, usize, i32) = core::mem::transmute(FN_TRANSFER_OFFER_INIT_TRANSFER_OFFER);
    let p = new(0x41);
    let offer = if p != 0 { construct(p) } else { 0 };
    *out = offer;
    crate::comparison::branch("cpp_7 oferta em dinheiro");
    iniciar(offer, player, club, value); // the offer value
    *atracao = 0x32;
    1
}

/// Comparador: sorteia, reagenda, rejeita ou retira (pode liberar a oferta) e dá notícia: roda só a original.
// simplification: original only; the copy is proven by the branch and by the mutation in the career.

/// Comparador: tira jogadores de ofertas e desfaz ofertas (libera itens): roda só a original.
// simplification: original only; the copy is proven by the branch and by the mutation in the career.

/// Transfer rules of nation `nacao` among the 28 at this+0x8ac (matching +0xa), or 0.
///
/// [pt] A regra de transferência da nação `nacao` entre as 28 de `this + 0x8ac` (a de +0xa igual), ou 0.
pub(crate) unsafe fn nation_transfer_rules_or_zero(this: usize, nation: usize) -> usize {
    for k in 0..0x1cusize {
        let r = read_usize(this + transfer_manager_fields::TRANSFER_RULES + k * 4);
        if read_usize(r + transfer_rules::NATION_ID) == nation {
            return r;
        }
    }
    0
}

/// Returns the offer response delay: from the time until the window closes (the rules of the buyer's and the player
/// club's nations), 0 days, 1 or 2 days with random slices, or a value from the player's interest (0x846920) or
/// randomly drawn from the status; days = value / 3, slices = value % 3.
///
/// Ret 0xc: `TRANSFER_MANAGER::get_offer_response_delay(offer, days, slices)`: the deadline is the window closing
/// (0x8e8ed0) of the buyer's (+8) nation (without a rule: 365) and, with +4 of the offer, the lesser of that and the
/// one of the nation of the club of the player's (+0xc) contract (a type 5 contract does not count). Below 3: 0 days
/// and 1 slice; below 6 or 11: 1 or 2 days and 1..3 random slices. Otherwise the value by the status of the contract
/// with the club (+0x3d; without it −100): a player (0x53b620 of +0x11; without a sheet, the "Error" box) with ability
/// +7 of the sheet above 130 and above the club's reputation / 60, under 26 years old, in fewer than 3 lists and with a
/// positive status, with the interest (0x846920) above `[0x968e58]`: the deadline × `[0x968e18]` up to `[0x968dd8]`;
/// otherwise a random draw up to status/10 + 12 (the larger with 1..3). Days = value / 3, slices = value % 3.
/// Original: `TRANSFER_MANAGER::get_offer_response_delay` (0x8cc5b0, transfer_manager.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x8cc5b0 } (thiscall, ret 0xc, transfer_manager.cpp): `TRANSFER_MANAGER::get_offer_response_delay(oferta, dias,
/// fatias)`: o prazo é o de fechar a janela (0x8e8ed0) da nação do comprador (+8; sem regra: 365) e, com o +4 da oferta,
/// o menor com o da nação do clube do contrato do jogador (+0xc; contrato de tipo 5 não conta). Abaixo de 3: 0 dia e 1
/// fatia; abaixo de 6 ou de 11: 1 ou 2 dias e 1..3 fatias sorteadas. Senão o valor pelo estatuto do contrato com o clube
/// (+0x3d; sem ele −100): o jogador (0x53b620 do +0x11; sem ficha, a caixa "Error") de habilidade +7 da ficha acima de
/// 130 e da reputação do clube / 60, menos de 26 anos, em menos de 3 listas e estatuto positivo, com o interesse
/// (0x846920) acima de [0x968e58]: o prazo × [0x968e18] até [0x968dd8]; senão um sorteio até estatuto/10 + 12 (o maior
/// com 1..3). Dias = valor / 3, fatias = valor % 3.
pub(crate) unsafe extern "thiscall" fn get_offer_response_delay(this: usize, offer: usize, days: *mut u8, fatias: *mut u8) {
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let with_club: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let window: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_TRANSFER_RULES_GET_DAYS_UNTIL_WINDOW_CLOSES);
    let sortear: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let buyer = friendly_club_by_id(read_usize(offer + offer::BUYER_CLUB_ID));
    let p = staff_records_staff_by_id(read_usize(offer + offer::TARGET_STAFF_ID));
    let c = contract_of(read_usize(G_CONTRACT_MANAGER), p);
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
    let owner = if c != 0 && read_u8(c + contract::CONTRACT_TYPE) & 0x3f != 5 { friendly_club_by_id(read_usize(c + contract::CLUB)) } else { 0 };
    let k = with_club(read_usize(G_CONTRACT_MANAGER), p, owner);
    let mut deadline: u16 = 0x16d;
    let r = nation_transfer_rules_or_zero(this, read_usize(buyer + club::NATION));
    if r != 0 {
        deadline = window(r) as u16;
    }
    if read_usize(offer + offer::EXCHANGE_PLAYERS) != 0 {
        let r = nation_transfer_rules_or_zero(this, read_usize(owner + club::NATION));
        if r != 0 && deadline >= window(r) as u16 {
            deadline = window(r) as u16;
        }
    }
    if deadline < 3 {
        *days = 0;
        *fatias = 1;
        return;
    }
    if deadline < 0xb {
        *days = if deadline < 6 { 1 } else { 2 };
        *fatias = (sortear(3) as u8).wrapping_add(1);
        return;
    }
    let status: i8 = if k != 0 { read_u8(k + 0x3d) as i8 } else { -100 };
    // simplification: the original pushes the role in a register with garbage above; 0x53b620 only reads the byte
    let and_player: unsafe extern "C" fn(u32) -> u32 = core::mem::transmute(FN_DATABASE_IS_PLAYER_JOB);
    if and_player(read_u8(offer + offer::JOB_TYPE) as u32) != 0 && read_usize(p + staff::PLAYER) == 0 {
        show_error_at(0xa8d514, 0x29c3);
    }
    let mut value: Option<u8> = None;
    if owner != 0 && and_player(read_u8(offer + offer::JOB_TYPE) as u32) != 0 {
        let hab = read_i16(read_usize(p + staff::PLAYER) + 7);
        if hab > 0x82
            && hab as i32 > read_i16(owner + club::REPUTATION) as i32 / 60
            && (read_u8(p + staff::AGE) as i8) < 0x1a
            && read_u8(read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(p).wrapping_mul(0x4f)) + 0xb) & 0xf < 3
            && status > 0
        {
            let interest: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32) -> f64 = core::mem::transmute(FN_SHORTLIST_MANAGER_STAFF_INTEREST_IN_JOINING_CLUB);
            if interest(read_usize(this + transfer_manager_fields::SHORTLIST_MANAGER), p, buyer, 0, 0, 1, 0) > read_f64_at(0x968e58) { // 0x968e58 = 3.15
                let x = deadline as i32 as f64 * read_f64_at(0x968e18); // 0x968e18 = 2.5
                let ceiling = read_f64_at(0x968dd8); // 0x968dd8 = 30
                value = Some(match_pl_ftol(if ceiling < x || x.is_nan() { ceiling } else { x }) as u8);
            }
        }
    }
    let v = match value {
        Some(v) => v,
        None => {
            crate::comparison::branch("cpo_10 prazo sorteado");
            let s = sortear(status as i32 / 10 + 0xc) as i8;
            if sortear(3) + 1 > s as i32 {
                (sortear(3) as u8).wrapping_add(1)
            } else {
                s as u8
            }
        }
    } as i8 as i32;
    *days = (v / 3) as u8;
    *fatias = (v % 3) as u8;
}

/// Pays each transfer instalment (payer's and receiver's finances), adds the fractions to the instalment figure (+0x1c)
/// in the last 8, removes the finished ones and zeroes the figure of clubs without an instalment in its direction.
///
/// Ret 0: `TRANSFER_MANAGER::pay_transfer_instalments()`: at the end of the month, each instalment in the list (+0x872;
/// 13 bytes: +0 the amount, +4 the paying club, +8 the receiving one, +0xc the number remaining) is paid (0x599890 on
/// the payer's finances, 0x599800 on the receiver's, 0x5a0590) and counts one fewer; with fewer than 4 or 8 remaining,
/// the instalment figure (+0x1c of the finances) receives the fractions (`[0x968918]`/`[0x968f30]` or
/// `[0x967880]`/`[0x969518]`); finished, it leaves the list (0x68d1f0); otherwise both clubs are marked (0x945501).
/// Then, in each club with finances, the figure is zeroed: without a mark; positive only when paying; negative only
/// when receiving.
/// Original: `TRANSFER_MANAGER::pay_transfer_instalments` (0x8caee0, transfer_manager.cpp, thiscall, ret 0).
///
/// [pt] { 0x8caee0 } (thiscall, ret 0, transfer_manager.cpp): `TRANSFER_MANAGER::pay_transfer_instalments()`: no fim do mês,
/// cada prestação da lista (+0x872; 13 bytes: +0 o valor, +4 o clube que paga, +8 o que recebe, +0xc as que faltam) é
/// paga (0x599890 nas finanças do pagador, 0x599800 nas do recebedor, 0x5a0590) e conta uma a menos; faltando menos de
/// 4 ou de 8, a figura das prestações (+0x1c das finanças) recebe as frações ([0x968918]/[0x968f30] ou
/// [0x967880]/[0x969518]); acabada, sai da lista (0x68d1f0); senão os dois clubes ficam marcados (0x945501). Depois, em
/// cada clube com finanças, a figura zera: sem marca; positiva só pagando; negativa só recebendo.
pub(crate) unsafe extern "thiscall" fn pay_transfer_instalments(this: usize) {
    let calloc: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(FN_CRT_CALLOC);
    let livrar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let finances: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FINANCETABLE_GET_CLUB_FINANCE);
    let pay: unsafe extern "thiscall" fn(usize, i64) = core::mem::transmute(FN_FINANCE_PAY_PLAYERS_BOUGHT);
    let receber: unsafe extern "thiscall" fn(usize, i64) = core::mem::transmute(FN_FINANCE_ADD_PLAYERS_SOLD);
    let remove: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_LLIST_REMOVE);
    let recebe = calloc(read_usize(G_CLUB_COUNT) as u32, 1);
    let paga = calloc(read_usize(G_CLUB_COUNT) as u32, 1);
    let list = this + transfer_manager_fields::INSTALMENTS;
    write_u32(list + llist::CURSOR, read_usize(this + transfer_manager_fields::INSTALMENTS_HEAD) as u32);
    loop {
        let e = next(list);
        if e == 0 {
            break;
        }
        let fr = finances(read_usize(G_FINANCE_TABLE), friendly_club_by_id(read_usize(e + instalment::RECEIVING_CLUB_ID)));
        let fp = finances(read_usize(G_FINANCE_TABLE), friendly_club_by_id(read_usize(e + instalment::PAYING_CLUB_ID)));
        pay(fp, read_i32(e) as i64);
        receber(fr, read_i32(e) as i64);
        let n = (read_u8(e + instalment::REMAINING) as i8).wrapping_sub(1);
        *((e + instalment::REMAINING) as *mut i8) = n;
        let fracoes = if n < 4 { Some((0x968918, 0x968f30)) } else if n < 8 { Some((0x967880, 0x969518)) } else { None };
        if let Some((a, b)) = fracoes {
            write_u32(fp + finance::INSTALMENT_BALANCE, read_i32(fp + finance::INSTALMENT_BALANCE).wrapping_add(match_pl_ftol(read_i32(e) as f64 * read_f64_at(a))) as u32);
            write_u32(fr + finance::INSTALMENT_BALANCE, read_i32(fr + finance::INSTALMENT_BALANCE).wrapping_add(match_pl_ftol(read_i32(e) as f64 * read_f64_at(b))) as u32);
        }
        if read_u8(e + instalment::REMAINING) == 0 {
            crate::comparison::branch("cpo_10 prestacao acabada");
            remove(list, e);
        } else {
            *((recebe + read_usize(friendly_club_by_id(read_usize(e + instalment::RECEIVING_CLUB_ID)))) as *mut u8) = 1;
            *((paga + read_usize(friendly_club_by_id(read_usize(e + instalment::PAYING_CLUB_ID)))) as *mut u8) = 1;
        }
    }
    let mut i = 0usize;
    while (i as i32) < read_i32(G_CLUB_COUNT) {
        let f = finances(read_usize(G_FINANCE_TABLE), read_usize(G_CLUB_TABLE).wrapping_add(i * club::SIZE));
        if f != 0 {
            let (r, p) = (read_u8(recebe + i) != 0, read_u8(paga + i) != 0);
            let v = read_i32(f + finance::INSTALMENT_BALANCE);
            let clear = match (r, p) {
                (false, false) => true,
                (false, true) => v > 0,
                (true, false) => v < 0,
                (true, true) => false,
            };
            if clear {
                write_u32(f + finance::INSTALMENT_BALANCE, 0);
            }
        }
        i += 1;
    }
    if recebe != 0 {
        livrar(recebe);
    }
    if paga != 0 {
        livrar(paga);
    }
}

/// Removes the offer: destroys the linked offer, removes the pair from the loan list and the contract offers of the
/// exchange and of the player, lowers the player's (+0x86e) and the manager's offer counts, the lowest free slot
/// follows and the offer is destroyed.
///
/// Ret 4: `TRANSFER_MANAGER::remove_offer(offer)`: the linked offer (+0x28 ≠ −1, in +0xc) is destroyed (0x8e5710,
/// 0x944c94), the slot is zeroed and count +0x14 drops; with +0x1f at 1, the pair (club +8, player +0xc) leaves the
/// loan list (+0x946); for each exchange player (+4): the contract proposal linked to slot +0x20 is removed (0x4ccb30,
/// if the offer's player has a contract) and +2 of his 8-byte record (+0x86e) drops; the player's contract proposal
/// with the buyer is removed (0x4cca70), +1 of the player's record drops, the lowest free slot (+0x85e) follows, slot
/// +0x20 of the vector (+0) is zeroed, the offer is destroyed and count +8 drops.
/// Original: `TRANSFER_MANAGER::remove_offer` (0x8d0f60, transfer_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x8d0f60 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::remove_offer(oferta)`: a oferta ligada
/// (+0x28 ≠ −1, em +0xc) é destruída (0x8e5710, 0x944c94), a vaga zera e a conta +0x14 baixa; com o +0x1f em 1, sai da
/// lista de empréstimos (+0x946) o par (clube +8, jogador +0xc); para cada jogador da troca (+4): a proposta de contrato
/// ligada à vaga +0x20 sai (0x4ccb30, se o jogador da oferta tem contrato) e o +2 do registro de 8 bytes dele (+0x86e)
/// baixa; sai a proposta de contrato do jogador com o comprador (0x4cca70), o +1 do registro do jogador baixa, a menor
/// vaga livre (+0x85e) acompanha, a vaga +0x20 do vetor (+0) zera, a oferta é destruída e a conta +8 baixa.
pub(crate) unsafe extern "thiscall" fn remove_offer(this: usize, offer: usize) {
    let destroy: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_TRANSFER_OFFER_DESTRUCTOR_TRANSFER_OFFER);
    let erase: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_OPERATOR_DELETE);
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    crate::comparison::branch("cpo_10 tirar oferta");
    if read_i32(offer + offer::NEGOTIATION_INDEX) != -1 {
        let x = read_usize(read_usize(this + transfer_manager_fields::NEGOTIATIONS).wrapping_add((read_i32(offer + offer::NEGOTIATION_INDEX) as usize).wrapping_mul(4)));
        if x != 0 {
            destroy(x);
            erase(x);
        }
        write_u32(read_usize(this + transfer_manager_fields::NEGOTIATIONS).wrapping_add((read_i32(offer + offer::NEGOTIATION_INDEX) as usize).wrapping_mul(4)), 0);
        write_u32(this + transfer_manager_fields::NEGOTIATION_COUNT, read_i32(this + transfer_manager_fields::NEGOTIATION_COUNT).wrapping_sub(1) as u32);
    }
    if read_u8(offer + offer::OFFER_TYPE) == 1 && read_usize(this + transfer_manager_fields::LOAN_NEGOTIATIONS) != 0 {
        let l = read_usize(this + transfer_manager_fields::LOAN_NEGOTIATIONS);
        write_u32(l + llist::CURSOR, read_usize(l + llist::HEAD) as u32);
        loop {
            let e = next(read_usize(this + transfer_manager_fields::LOAN_NEGOTIATIONS));
            if e == 0 {
                break;
            }
            if read_usize(e + 4) == read_usize(offer + offer::BUYER_CLUB_ID) && read_usize(e) == read_usize(offer + offer::TARGET_STAFF_ID) {
                let remove: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_LLIST_REMOVE);
                remove(read_usize(this + transfer_manager_fields::LOAN_NEGOTIATIONS), e);
                break;
            }
        }
    }
    if read_usize(offer + offer::EXCHANGE_PLAYERS) != 0 {
        let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
        let tirar_proposta: unsafe extern "thiscall" fn(usize, usize, i32) = core::mem::transmute(FN_CONTRACT_MANAGER_REMOVE_CONTRACT_OFFER_FOR_TRANSFER);
        let l = read_usize(offer + offer::EXCHANGE_PLAYERS);
        write_u32(l + llist::CURSOR, read_usize(l + llist::HEAD) as u32);
        loop {
            let e = next(read_usize(offer + offer::EXCHANGE_PLAYERS));
            if e == 0 {
                break;
            }
            let c = contract_of(read_usize(G_CONTRACT_MANAGER), staff_records_staff_by_id(read_usize(offer + offer::TARGET_STAFF_ID)));
            *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
            if c != 0 && friendly_club_by_id(read_usize(c + contract::CLUB)) != 0 {
                tirar_proposta(read_usize(G_CONTRACT_MANAGER), staff_records_staff_by_id(read_usize(e)), read_i32(offer + offer::SLOT_INDEX));
            }
            let q = (read_usize(this + transfer_manager_fields::PERSON_FLAGS).wrapping_add(read_usize(e).wrapping_mul(8)) + 2) as *mut u8;
            *q = (*q).wrapping_sub(1);
        }
    }
    let remove_of_club: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_CONTRACT_MANAGER_REMOVE_CONTRACT_OFFER);
    remove_of_club(read_usize(G_CONTRACT_MANAGER), staff_records_staff_by_id(read_usize(offer + offer::TARGET_STAFF_ID)), friendly_club_by_id(read_usize(offer + offer::BUYER_CLUB_ID)));
    let q = (read_usize(this + transfer_manager_fields::PERSON_FLAGS).wrapping_add(read_usize(offer + offer::TARGET_STAFF_ID).wrapping_mul(8)) + 1) as *mut u8;
    *q = (*q).wrapping_sub(1); // the offers for the person
    if read_i32(offer + offer::SLOT_INDEX) < read_i32(this + transfer_manager_fields::LOWEST_FREE_SLOT) {
        write_u32(this + transfer_manager_fields::LOWEST_FREE_SLOT, read_usize(offer + offer::SLOT_INDEX) as u32);
    }
    write_u32(read_usize(this).wrapping_add((read_i32(offer + offer::SLOT_INDEX) as usize).wrapping_mul(4)), 0);
    if offer != 0 {
        destroy(offer);
        erase(offer);
    }
    write_u32(this + transfer_manager_fields::OFFER_COUNT, read_i32(this + transfer_manager_fields::OFFER_COUNT).wrapping_sub(1) as u32);
}

/// Comparador: só a original, conferindo o plano (feito antes): saem da lista as prestações que tinham 1 a pagar.
// simplification: the original pays (finances), removes items from the list (frees) and allocates the flags; it does not run twice. The copy
// has the branches and the proof is by the career.

/// Comparador: só a original, conferindo o plano (feito antes): a vaga zerada, as contas +8 e +0x14, a menor vaga livre
/// e o +1 do registro do jogador.
// simplification: the original destroys and frees the offer and touches the lists; it does not run twice.

/// Person id has the "under offer" flag (bit 0 of [this+0x86e] + id x 8).
///
/// [pt] A marca "sob oferta" da pessoa (bit 0 do byte [this+0x86e] + id × 8).
pub(crate) unsafe fn is_marked_under_offer(this: usize, id: usize) -> bool {
    *((read_usize(this + transfer_manager_fields::PERSON_FLAGS).wrapping_add(id.wrapping_mul(8))) as *const u8) & 1 != 0
}

/// Offer at index k in the pointer vector [this].
///
/// [pt] A oferta do índice `k` (vetor de ponteiros [this]).
pub(crate) unsafe fn offer_at(this: usize, k: usize) -> usize {
    read_usize(read_usize(this).wrapping_add(k.wrapping_mul(4)))
}

/// Builds the local list of indices of the offers for the person (`0x68d020`, which `0x8c5cf0` fills with `todas`),
/// runs `f` on it and destroys it.
///
/// The cursor (`+0xa`) is reset to the start (`+2`) as in the original routine; `f` walks it and the list is destroyed
/// (`0x68d040`) at the end. On the exits before `0x8c5cf0` the original routine builds and destroys the empty list,
/// with no effect (it does not allocate): here it is not created.
///
/// [pt] A lista local de índices das ofertas pela pessoa (0x68d020, que a 0x8c5cf0 enche com `todas`), com o cursor
/// (+0xa) reposto no começo (+2) como a original; `f` percorre e a lista é destruída (0x68d040) no fim. Nas saídas
/// antes da 0x8c5cf0 a original constrói e destrói a lista vazia, sem efeito (não aloca): aqui não nasce.
pub(crate) unsafe fn with_offers_for_player<R>(this: usize, person: usize, all: u32, f: impl FnOnce(usize) -> R) -> R {
    let list_new: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let destroy: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let offers: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_OFFERS_FOR_PLAYER);
    let mut l = [0u8; 0x10];
    let list = l.as_mut_ptr() as usize;
    list_new(list);
    offers(this, person, list, all);
    core::ptr::write_unaligned((list + llist::CURSOR) as *mut u32, core::ptr::read_unaligned((list + llist::HEAD) as *const u32));
    let r = f(list);
    destroy(list);
    r
}

/// Next node of the LLIST (0x68d1b0), or 0.
///
/// [pt] O próximo nó da lista (0x68d1b0), ou 0.
pub(crate) unsafe fn llist_next_node(list: usize) -> usize {
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    next(list)
}

/// Non-null person whose id is >= G_STAFF_COUNT.
///
/// [pt] A pessoa fora da tabela (id ≥ [0xae2364]); pessoa nula passa e é lida como na original.
pub(crate) unsafe fn staff_out_of_table(person: usize) -> bool {
    person != 0 && read_usize(person) as i32 >= read_usize(G_STAFF_COUNT) as i32
}

/// Returns 1 when the person flagged as under offer (bit 0 of `[+0x86e]` + id × 8) has, among the open offers
/// (0x8c5cf0), one from the club (offer +8); else 0.
///
/// Ret 8: `TRANSFER_MANAGER::club_has_offer_for_player(person, club)`: 1 if the person flagged as under offer (bit 0 of
/// `[this+0x86e]` + id × 8) has, among the open offers (0x8c5cf0 without `all`), one from the club (+8 of the offer =
/// club id); otherwise 0 (also for an id outside the table).
/// Original: `TRANSFER_MANAGER::club_has_offer_for_player` (0x8cdc90, transfer_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x8cdc90 } (thiscall, ret 8, transfer_manager.cpp): `TRANSFER_MANAGER::club_has_offer_for_player(pessoa,
/// clube)`: 1 se a pessoa marcada como sob oferta (bit 0 de [this+0x86e] + id × 8) tem, entre as ofertas em aberto
/// (0x8c5cf0 sem `todas`), uma do clube (+8 da oferta = id do clube); senão 0 (id fora da tabela também).
pub(crate) unsafe extern "thiscall" fn club_has_offer_for_player(this: usize, person: usize, club: usize) -> u32 {
    if staff_out_of_table(person) || !is_marked_under_offer(this, read_usize(person)) {
        return 0;
    }
    with_offers_for_player(this, person, 0, |lista| loop {
        let no = llist_next_node(lista);
        if no == 0 {
            return 0;
        }
        if read_usize(offer_at(this, read_usize(no)) + 8) == read_usize(club) { // the offer is from the club
            crate::comparison::branch("cpl_1 clube tem oferta");
            return 1;
        }
    })
}

/// Returns the club's offer for the person (with `all`, closed ones too and without requiring the flag), or 0.
///
/// Ret 0xc: `TRANSFER_MANAGER::find_club_offer_for_player(person, club, all)`: the club's offer for the person (the
/// first in the list from 0x8c5cf0, with `all` also the closed ones), or 0. Without `all`, the person must be flagged
/// as under offer; id outside the table: 0.
/// Original: `TRANSFER_MANAGER::find_club_offer_for_player` (0x8cdd80, transfer_manager.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x8cdd80 } (thiscall, ret 0xc, transfer_manager.cpp): `TRANSFER_MANAGER::find_club_offer_for_player(pessoa,
/// clube, todas)`: a oferta do clube pela pessoa (a primeira da lista da 0x8c5cf0, com `todas` também as encerradas),
/// ou 0. Sem `todas`, a pessoa tem de estar marcada como sob oferta; id fora da tabela: 0.
pub(crate) unsafe extern "thiscall" fn find_club_offer_for_player(this: usize, person: usize, club: usize, all: u32) -> usize {
    if staff_out_of_table(person) || (all == 0 && !is_marked_under_offer(this, read_usize(person))) {
        return 0;
    }
    with_offers_for_player(this, person, all, |lista| loop {
        let no = llist_next_node(lista);
        if no == 0 {
            return 0;
        }
        let o = offer_at(this, read_usize(no));
        if read_usize(o + offer::BUYER_CLUB_ID) == read_usize(club) { // the club’s offer
            crate::comparison::branch("cpl_1 oferta do clube");
            return o;
        }
    })
}

/// Returns 1 when the person is under offer (0x8cde70) and an open offer is in state (+0x2c) 0, 1, 5 or 6; else 0.
///
/// Ret 4: `TRANSFER_MANAGER::player_has_pending_offer(person)`: 1 if the person is under offer (0x8cde70 without the
/// human view) and one of the open offers (0x8c5cf0) is in state (+0x2c) 0, 1, 5 or 6; otherwise 0. An index beyond the
/// count (+4) reads offer 0, as the original.
/// Original: `TRANSFER_MANAGER::player_has_pending_offer` (0x8cd150, transfer_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x8cd150 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::player_has_pending_offer(pessoa)`: 1 se
/// a pessoa está sob oferta (0x8cde70 sem a visão do humano) e uma das ofertas em aberto (0x8c5cf0) está na situação
/// (+0x2c) 0, 1, 5 ou 6; senão 0. O índice além da conta (+4) lê a oferta 0, como a original.
pub(crate) unsafe extern "thiscall" fn player_has_pending_offer(this: usize, person: usize) -> u32 {
    let sob_offer: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_IS_PLAYER_UNDER_OFFER);
    // simplification: the original builds the list before asking; empty, 0x68d020/0x68d040 do not allocate (nothing to imitate)
    if sob_offer(this, person, 0) == 0 {
        return 0;
    }
    with_offers_for_player(this, person, 0, |lista| loop {
        let no = llist_next_node(lista);
        if no == 0 {
            return 0;
        }
        let k = read_usize(no) as i32;
        let o = if k < read_usize(this + transfer_manager_fields::OFFER_CAPACITY) as i32 { offer_at(this, k as usize) } else { read_usize(read_usize(this)) };
        if matches!(*((o + offer::STATUS) as *const u8), 0 | 1 | 5 | 6) { // the pending status
            crate::comparison::branch("cpl_1 oferta pendente");
            return 1;
        }
    })
}

/// Region touched by 0x8c5cf0: the cursor bytes of the person's offer list (or a dummy region if none).
///
/// [pt] As regiões que a 0x8c5cf0 mexe: o cursor da lista própria da oferta da pessoa (+0xa..+0xe; a 0x8c5cf0 o repõe).
pub(crate) unsafe fn offer_cursor_region(this: usize, person: usize) -> [(usize, usize); 1] {
    let e = offer_for_player(this, read_usize(person) as i32);
    let l = if e == 0 { 0 } else { read_usize(e) };
    if l == 0 { [(0xae2364, 4)] } else { [(l, 0xe)] }
}

// simplification: all three only read (the local list is born and dies in them); with a null person, original only (reads address 0)

/// Club id -> club record (0x245 bytes in G_CLUB_TABLE).
///
/// [pt] O clube do id `i` (registros de 0x245 bytes em [0xae23b4]).
pub(crate) unsafe fn club_record(i: usize) -> usize {
    read_usize(G_CLUB_TABLE).wrapping_add(i.wrapping_mul(club::SIZE))
}

/// Person -> contract (get_contract), stored in G_CONTRACT_LOOKUP_RESULT.
///
/// [pt] O contrato da pessoa (0x4cf5a0 em [0xae1050]), guardado em [0xae1080] como a original.
pub(crate) unsafe fn contract_of_person(person: usize) -> usize {
    let contract: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = contract(read_usize(G_CONTRACT_MANAGER), person);
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
    c
}

/// Contract is non-null and its type (+0x35 & 0x3f) is not 5.
///
/// [pt] O contrato vale (não nulo e a situação +0x35 & 0x3f ≠ 5).
pub(crate) unsafe fn is_contract_valid(c: usize) -> bool {
    c != 0 && *((c + contract::CONTRACT_TYPE) as *const u8) & 0x3f != 5
}

/// The current human manager ([0xb74c26 + 0x300 x current screen slot]).
///
/// [pt] O técnico humano da vez ([0xb74c26] + 0x300 × (i16)[0xb77c7a]).
pub(crate) unsafe fn transfer_manager_current_human_manager() -> usize {
    let i = *(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as isize;
    read_usize((0xb74c26isize + i * 0x300) as usize)
}

/// The current human manager controls the club (HUMAN_MANAGER current_controls_club with 0, 1).
///
/// [pt] O humano da vez manda no clube (0x600c90 com 0, 1).
pub(crate) unsafe fn current_human_controls_club(club: usize) -> bool {
    let manda: unsafe extern "C" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_CURRENT_CONTROLS_CLUB);
    manda(club, 0, 1) != 0
}

/// Body of TRANSFER_MANAGER::is_player_under_offer (0x8cde70) over a built local list: the under-offer flag, or as seen
/// by the current human (shortlist, offers involving clubs they control).
///
/// [pt] O corpo da 0x8cde70 sobre a lista local já construída.
pub(crate) unsafe fn is_under_offer_body(this: usize, person: usize, visao: u32, list: usize) -> u32 {
    if person != 0 && read_usize(person) as i32 >= read_usize(G_STAFF_COUNT) as i32 {
        return 0;
    }
    let mark = || (*((read_usize(this + transfer_manager_fields::PERSON_FLAGS).wrapping_add(read_usize(person).wrapping_mul(8))) as *const u8) & 1) as u32;
    if visao == 0 {
        return mark(); // a marca "sob oferta"
    }
    let m = transfer_manager_current_human_manager();
    if m == 0 {
        return mark();
    }
    let na_shortlist: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_SCOUT_MANAGER_IS_PLAYER_SHORTLISTED);
    let offers: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_OFFERS_FOR_PLAYER);
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let marked = na_shortlist(read_usize(G_SCOUT_MANAGER), person, m);
    offers(this, person, list, 0);
    core::ptr::write_unaligned((list + llist::CURSOR) as *mut u32, core::ptr::read_unaligned((list + llist::HEAD) as *const u32));
    loop {
        let no = next(list);
        if no == 0 {
            break;
        }
        let o = read_usize(read_usize(this).wrapping_add(read_usize(no).wrapping_mul(4)));
        if *((o + 0x2d) as *const u8) != 0 || current_human_controls_club(club_record(read_usize(o + 8))) {
            return 1;
        }
        let p = read_usize(G_STAFF_TABLE).wrapping_add(read_usize(o + 0xc).wrapping_mul(staff::SIZE));
        let c = contract_of_person(p);
        if is_contract_valid(c) && club_record(read_usize(c + contract::CLUB)) != 0 {
            // the player’s club: the original asks for the contract again
            let c = contract_of_person(p);
            let club = if is_contract_valid(c) { club_record(read_usize(c + contract::CLUB)) } else { 0 };
            if current_human_controls_club(club) {
                return 1;
            }
        }
        if marked != 0 {
            return 1;
        }
    }
    let x = read_usize(transfer_manager_current_human_manager() + 0x39);
    if x != 0 {
        let contract_offer: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_FIND_CONTRACT_OFFER);
        let o = contract_offer(read_usize(G_CONTRACT_MANAGER), read_usize(person), read_usize(x));
        if o != 0 && *((o + 0x35) as *const u8) & 0x3f == 5 {
            return 1;
        }
    }
    0
}

/// Tells whether the player is under offer: without the human view, the "under offer" flag (bit 0 of `[+0x86e]` + id ×
/// 8); with it, 1 when an offer is known (+0x2d), comes from a club of the current human or is for a player of his
/// club, when the person is on his shortlist or when he has an accepted contract offer for the person.
///
/// Ret 8: `TRANSFER_MANAGER::is_player_under_offer(person, view)`: id outside the table: 0. Without `view` (or without
/// a human whose turn it is), the "under offer" flag (bit 0 of `[this+0x86e]` + id × 8). With it: 1 if some open offer
/// for the person (0x8c5cf0, into a local list) is known (+0x2d), is from a club of the human whose turn it is
/// (0x600c90) or the current club of the offer's player (+0xc; contract 0x4cf5a0, which stays in `[0xae1080]`) is his,
/// or if the person is on his shortlist (0x7f7490; valid after the 1st offer); finally, 1 if the human (+0x39) has an
/// accepted contract offer (0x4cc9e0, +0x35 & 0x3f = 5) for the person. Otherwise 0.
/// Original: `TRANSFER_MANAGER::is_player_under_offer` (0x8cde70, transfer_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x8cde70 } (thiscall, ret 8, transfer_manager.cpp): `TRANSFER_MANAGER::is_player_under_offer(pessoa, visao)`: id
/// fora da tabela: 0. Sem `visao` (ou sem humano da vez), a marca "sob oferta" (bit 0 de [this+0x86e] + id × 8). Com
/// ela: 1 se alguma oferta em aberto pela pessoa (0x8c5cf0, numa lista local) é conhecida (+0x2d), é de clube do
/// humano da vez (0x600c90) ou o clube atual do jogador da oferta (+0xc; contrato 0x4cf5a0, que fica em [0xae1080]) é
/// dele, ou se a pessoa está na shortlist dele (0x7f7490; vale depois da 1ª oferta); por fim, 1 se o humano (+0x39)
/// tem oferta de contrato aceita (0x4cc9e0, +0x35 & 0x3f = 5) pela pessoa. Senão 0.
pub(crate) unsafe extern "thiscall" fn is_player_under_offer(this: usize, person: usize, visao: u32) -> u32 {
    let list_new: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let destroy: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let mut l = [0u8; 0x10];
    let list = l.as_mut_ptr() as usize;
    list_new(list);
    let r = is_under_offer_body(this, person, visao, list);
    destroy(list);
    r
}

/// Returns 1 when the nation of the club of the person's contract has free transfers at contract expiry (0x544670);
/// without a contract, club or nation: 1.
///
/// Stdcall, ret 4: `player_club_uses_bosman(person)`: 1 if the club of the person's contract (0x4cf5a0, stored in
/// `[0xae1080]`) is from a nation with free transfer at the end of the contract (0x544670); without a valid contract,
/// without a club or without a nation (+0x53): 1. Null person: the original routine (box 0x3798 and 0). The original's
/// local date (0x54a110) only serves the SEH frame. Hot (~31 million calls): no counter.
/// Original: `player_club_uses_bosman` (0x8d0a70, transfer_manager.cpp, stdcall, ret 4).
///
/// [pt] { 0x8d0a70 } (stdcall, ret 4, transfer_manager.cpp): `player_club_uses_bosman(pessoa)`: 1 se o clube do contrato
/// da pessoa (0x4cf5a0, guardado em [0xae1080]) é de nação com transferência livre no fim do contrato (0x544670);
/// sem contrato que valha, sem clube ou sem nação (+0x53): 1. Pessoa nula: a original (caixa 0x3798 e 0). A data
/// local da original (0x54a110) só serve ao quadro SEH. Quente (~31 milhões de chamadas): sem contador.
pub(crate) unsafe extern "stdcall" fn player_club_uses_bosman(person: usize) -> u32 {
    if person == 0 {
        return original_fn::<unsafe extern "stdcall" fn(usize) -> u32>(IDX_PLAYER_CLUB_USES_BOSMAN)(person);
    }
    let c = contract_of_person(person);
    if !is_contract_valid(c) {
        return 1;
    }
    let club = club_record(read_usize(c + contract::CLUB));
    if club == 0 || read_usize(club + club::NATION) == 0 {
        return 1;
    }
    let free: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_NATION_ALLOWS_FREE_TRANSFERS);
    free(read_usize(club + club::NATION)) // the club’s nation
}

/// Checks the board veto: with an empty exchange part, the deciding club's board answers (0x696b10, by the 0x8e7b50
/// value); 0x2000: reason 0x24 and the player loses 10% of his reputations (+0xb current, +9 home, +0xd world); 0x800:
/// 0x17; 0x200: 0x18; each returns 1.
///
/// Ret 0x18: `TRANSFER_MANAGER::check_board_veto(offer, buyer, seller, player, decider, reason)`: the offer value
/// (0x8e7b50 with 0, 0, 1, 0, 1); if the offer's exchange part (+4) is empty (null or word 0), the board of the
/// deciding club answers (0x696b10 on `[0xb67a44]` with the player, the club, the value and "is the buyer" in the low
/// byte of the buyer pointer, as the original pushes it). Bit 0x2000: reason 0x24 and the player (+0x61) loses 10% of
/// reputations +0xb (current), +9 (in the nation) and +0xd (world); 0x800: reason 0x17; 0x200: 0x18; each returns 1.
/// Otherwise 0.
/// Original: `TRANSFER_MANAGER::check_board_veto` (0x8cdb80, transfer_manager.cpp, thiscall, ret 0x18).
///
/// [pt] { 0x8cdb80 } (thiscall, ret 0x18, transfer_manager.cpp): `TRANSFER_MANAGER::check_board_veto(oferta, comprador,
/// vendedor, jogador, decide, motivo)`: o valor da oferta (0x8e7b50 com 0, 0, 1, 0, 1); se a oferta tem a parte de
/// troca (+4) vazia (nula ou palavra 0), a diretoria do clube que decide responde (0x696b10 em [0xb67a44] com o
/// jogador, o clube, o valor e "é o comprador" no byte baixo do ponteiro do comprador, como a original empurra). Bit
/// 0x2000: motivo 0x24 e o jogador (+0x61) perde 10% das reputações +0xb (atual), +9 (na nação) e +0xd (mundial); 0x800: motivo 0x17; 0x200: 0x18;
/// cada um devolve 1. Senão 0.
pub(crate) unsafe extern "thiscall" fn check_board_veto(_this: usize, offer: usize, buyer: usize, seller: usize, player: usize, decides: usize, reason: *mut u16) -> u32 {
    let value: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_OFFER_GET_WEIGHTED_VALUE);
    let v = value(offer, 0, 0, 1, 0, 1);
    // no offer, or with the exchange part (+4) filled in: the board is not consulted
    let reply = if offer == 0 || { let t = read_usize(offer + offer::EXCHANGE_PLAYERS); t != 0 && *(t as *const u16) != 0 } {
        0
    } else {
        let board: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_MANAGER_MANAGER_CHECK_BOARD_TRANSFER_VETO);
        let and_buyer = (buyer as u32 & 0xffff_ff00) | (decides == buyer) as u32;
        board(read_usize(G_MANAGER_MANAGER), player, decides, v, and_buyer)
    };
    let give = |m: u16| {
        if !reason.is_null() {
            *reason = m;
        }
    };
    if reply & 0x2000 != 0 {
        give(0x24);
        crate::comparison::branch("cpl_2 veto da diretoria valores");
        for k in [0xb, 9, 0xd] {
            let p = (read_usize(player + staff::PLAYER) + k) as *mut i16;
            core::ptr::write_unaligned(p, match_pl_ftol(core::ptr::read_unaligned(p) as f64 * read_f64_at(0x9689d8)) as i16); // 90%; 0x9689d8 = 0.9
        }
        return 1;
    }
    if reply & 0x800 != 0 {
        give(0x17);
        return 1;
    }
    if reply & 0x200 != 0 {
        give(0x18);
        return 1;
    }
    0
}

/// Regions touched by 0x8c5cf0 (the person's offer-list cursor) plus G_CONTRACT_LOOKUP_RESULT.
///
/// [pt] A região que a 0x8c5cf0 mexe (o cursor da lista própria da oferta da pessoa) e [0xae1080].
pub(crate) unsafe fn offer_cursor_and_lookup_regions(this: usize, person: usize) -> [(usize, usize); 2] {
    let e = offer_for_player(this, read_usize(person) as i32);
    let l = if e == 0 { 0 } else { read_usize(e) };
    [(G_CONTRACT_LOOKUP_RESULT, 4), if l == 0 { (0xae2364, 4) } else { (l, 0xe) }]
}

// simplification: only reads (the local list is born and dies in it); with a null person, original only (reads address 0)

/// Shows transfer_manager.cpp's (0xa8d514) "Memory Allocation Error" box; the game continues.
///
/// Text "v%s %s.%s %d (%s)" (version 0x90d130, name and extension, `linha`, `texto`) at 0xae24d0, the translated title
/// (0x66f4e0) at 0xae28bc, the body (0x9a7ff8 with 0xae24d0) at 0xae26c4, and the box 0x5e8290.
///
/// [pt] A caixa "Memory Allocation Error" de transfer_manager.cpp (0xa8d514): "v%s %s.%s %d (%s)" (versão 0x90d130, nome
/// e extensão, `linha`, `texto`) em 0xae24d0, o título traduzido (0x66f4e0) em 0xae28bc, o corpo (0x9a7ff8 com
/// 0xae24d0) em 0xae26c4 e a caixa 0x5e8290. O jogo segue.
pub(crate) unsafe fn show_transfer_memory_error(row: u32, text: usize) {
    let split: unsafe extern "C" fn(usize, usize, usize, *mut u8, *mut u8) = core::mem::transmute(FN_CRT_STRTOK);
    let version: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let translate: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let caixa: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(FN_GUI_SHOW_MESSAGE_BOX);
    let (mut nome, mut ext) = ([0u8; 256], [0u8; 256]);
    split(0xa8d514, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
    let s = version(nome.as_mut_ptr(), ext.as_mut_ptr(), row);
    format(0xae24d0, 0x9a8120, s, nome.as_ptr(), ext.as_ptr(), row, text);
    translate(0xae28bc, 0x9a8108);
    translate(0xae26c4, 0x9a7ff8, 0xae24d0usize);
    caixa(0xae28bc, 0xae26c4, 0);
}

/// Grows the vector at [this+v] by `passo` records of `tam` bytes when full (malloc/realloc), showing the memory box on
/// failure; returns whether a vector exists.
///
/// [pt] Cresce o vetor [this+v] (capacidade em +v+`c`, conta em +v+12−`c`: `c` = 4 nas passadas e nos prêmios, 8 nas
/// permissões) de `passo` registros de `tam` bytes quando cheio: malloc (0x944e46) sem vetor, senão realloc
/// (0x945667). Devolve se há vetor (sem memória: a caixa já saiu).
pub(crate) unsafe fn grow_record_vector(this: usize, v: usize, c: usize, step_towards: usize, size: usize, row: u32, text: usize) -> bool {
    if read_usize(this + v + 12 - c) != read_usize(this + v + c) {
        return true;
    }
    let cap = read_usize(this + v + c).wrapping_add(step_towards);
    write_usize(this + v + c, cap);
    let p = if read_usize(this + v) == 0 {
        let alloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
        alloc(cap.wrapping_mul(size) as u32)
    } else {
        let realloc: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_CRT_REALLOC);
        realloc(read_usize(this + v), cap.wrapping_mul(size) as u32)
    };
    write_usize(this + v, p);
    if p == 0 {
        show_transfer_memory_error(row, text);
        return false;
    }
    true
}

/// Appends the past transfer (0x25 bytes, 0x8cb940) to the +0x862 vector (capacity +0x866, count +0x86a), growing by
/// 100; returns the new record.
///
/// Ret 4: `TRANSFER_MANAGER::add_past_transfer(record)`: appends the past transfer (0x25 bytes, copy 0x8cb940) to
/// vector `[this+0x862]` (capacity +0x866, count +0x86a, which goes up before the copy); full, grows by 100 at a time
/// (out of memory: box 0x2794 and nothing). Returns the new record.
/// Original: `TRANSFER_MANAGER::add_past_transfer` (0x8cb810, transfer_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x8cb810 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::add_past_transfer(registro)`: acrescenta
/// a transferência passada (0x25 bytes, cópia 0x8cb940) ao vetor [this+0x862] (capacidade +0x866, conta +0x86a, que
/// sobe antes da cópia); cheio, cresce de 100 em 100 (sem memória: caixa 0x2794 e nada). Devolve o registro novo.
pub(crate) unsafe extern "thiscall" fn add_past_transfer(this: usize, record: usize) -> usize {
    crate::comparison::branch("cpl_3 transferencia passada");
    // simplification: out of memory the original returns whatever the message box left in eax; here 0 (the game does not read it)
    if !grow_record_vector(this, 0x862, 4, 100, 0x25, 0x2794, 0xa8ec34) {
        return 0;
    }
    let n = read_usize(this + transfer_manager_fields::PAST_TRANSFER_COUNT);
    let new = read_usize(this + transfer_manager_fields::PAST_TRANSFERS).wrapping_add(n.wrapping_mul(0x25));
    write_usize(this + transfer_manager_fields::PAST_TRANSFER_COUNT, n.wrapping_add(1)); // the count of past transfers
    let copy: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_PAST_TRANSFER_COPY_CONSTRUCT);
    copy(new, record)
}

/// Appends the work permit (14 bytes: person, year +8, nation +0xa) to the +0x93a vector (growing by 30), sets the
/// person's 0x20 flag at +0x86e and, when asked, sorts (0x8e55c0).
///
/// Ret 0x14: `TRANSFER_MANAGER::add_work_permit(person, nation, year, _, sort)`: appends the work permit (14 bytes: +0
/// person id, +4 and +6 zeroed, +8 year, +0xa nation id) to vector `[this+0x93a]` (capacity +0x942, count +0x93e);
/// full, grows by 30 at a time (out of memory: box 0x1b1b and nothing). Sets the permit flag (bit 0x20 of
/// `[this+0x86e]` + id × 8) and, with `sort`, sorts (qsort 0x945333 with 0x8e55c0, the order of search 0x8c7560).
/// Original: `TRANSFER_MANAGER::add_work_permit` (0x8c7070, transfer_manager.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x8c7070 } (thiscall, ret 0x14, transfer_manager.cpp): `TRANSFER_MANAGER::add_work_permit(pessoa, nacao, ano, _,
/// ordenar)`: acrescenta a permissão de trabalho (14 bytes: +0 id da pessoa, +4 e +6 zerados, +8 ano, +0xa id da
/// nação) ao vetor [this+0x93a] (capacidade +0x942, conta +0x93e); cheio, cresce de 30 em 30 (sem memória: caixa
/// 0x1b1b e nada). Liga a marca da permissão (bit 0x20 de [this+0x86e] + id × 8) e, com `ordenar`, reordena (qsort
/// 0x945333 com 0x8e55c0, a ordem da busca 0x8c7560).
pub(crate) unsafe extern "thiscall" fn add_work_permit(this: usize, person: usize, nation: usize, year: u32, not_used: u32, sort: u32) {
    crate::comparison::branch("cpl_3 permissao de trabalho");
    if !grow_record_vector(this, 0x93a, 8, 30, 0xe, 0x1b1b, 0xa8eb04) {
        return;
    }
    let n = read_usize(this + transfer_manager_fields::WORK_PERMIT_COUNT);
    let r = read_usize(this + transfer_manager_fields::WORK_PERMITS).wrapping_add(n.wrapping_mul(0xe));
    write_usize(r, read_usize(person));
    write_usize(r + work_permit::NATION_ID, read_usize(nation));
    core::ptr::write_unaligned((r + work_permit::YEAR) as *mut u16, year as u16);
    core::ptr::write_unaligned((r + work_permit::MATCHES_MISSED) as *mut u16, 0);
    core::ptr::write_unaligned((r + work_permit::MATCHES_PLAYED) as *mut u16, 0);
    write_usize(this + transfer_manager_fields::WORK_PERMIT_COUNT, n.wrapping_add(1));
    let mark = (read_usize(this + transfer_manager_fields::PERSON_FLAGS).wrapping_add(read_usize(person).wrapping_mul(8))) as *mut u8;
    *mark |= 0x20; // the permission flag
    if sort != 0 {
        let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
        qsort(read_usize(this + transfer_manager_fields::WORK_PERMITS), read_usize(this + transfer_manager_fields::WORK_PERMIT_COUNT) as u32, 0xe, 0x8e55c0);
    }
}

/// Appends the first-team appearance money (13 bytes) to the +0x88c vector (growing by 50) and sorts by person
/// (0x8b4e00).
///
/// Ret 4: `TRANSFER_MANAGER::add_first_team_appearance_money(record)`: appends the first-team appearance bonus (13
/// bytes copied from `record`) to vector `[this+0x88c]` (capacity +0x890, count +0x894); full, grows by 50 at a time
/// (out of memory: box 0x2581 and nothing). Sorts by person (qsort 0x945333 with 0x8b4e00).
/// Original: `TRANSFER_MANAGER::add_first_team_appearance_money` (0x8cac40, transfer_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x8cac40 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::add_first_team_appearance_money(registro)`:
/// acrescenta o prêmio por jogo no time principal (13 bytes copiados de `registro`) ao vetor [this+0x88c] (capacidade
/// +0x890, conta +0x894); cheio, cresce de 50 em 50 (sem memória: caixa 0x2581 e nada). Reordena pela pessoa (qsort
/// 0x945333 com 0x8b4e00).
pub(crate) unsafe extern "thiscall" fn add_first_team_appearance_money(this: usize, record: usize) {
    crate::comparison::branch("cpl_3 premio por jogo");
    if !grow_record_vector(this, 0x88c, 4, 50, 0xd, 0x2581, 0xa8eb94) {
        return;
    }
    let n = read_usize(this + transfer_manager_fields::APPEARANCE_MONEY_COUNT);
    core::ptr::copy_nonoverlapping(record as *const u8, read_usize(this + transfer_manager_fields::APPEARANCE_MONEY).wrapping_add(n.wrapping_mul(0xd)) as *mut u8, 0xd);
    write_usize(this + transfer_manager_fields::APPEARANCE_MONEY_COUNT, n.wrapping_add(1)); // one more prize
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    qsort(read_usize(this + transfer_manager_fields::APPEARANCE_MONEY), read_usize(this + transfer_manager_fields::APPEARANCE_MONEY_COUNT) as u32, 0xd, 0x8b4e00);
}

/// Some of the n records of `tam` bytes in vector v equals `alvo`.
///
/// [pt] Algum dos `n` registros de `tam` bytes do vetor `v` é igual a `alvo`.
pub(crate) unsafe fn vector_contains_record(v: usize, n: usize, size: usize, target: &[u8]) -> bool {
    (0..n).any(|i| core::slice::from_raw_parts((v + i * size) as *const u8, size) == target)
}

/// Returns the fixture's rules scope: reserves, the home nation and type 1; friendly, 0xff; otherwise the key nation of
/// the competition (0x669470), the continent (+0x59) and type 4 (0, 1), 2 (3 to 7) or 1.
///
/// Ret 0x10: `TRANSFER_MANAGER::get_fixture_rules_scope(fixture, type, nation, continent)`: the fixture's competition
/// (+0x14) is the reserves one (`[0x9cf684]`): the home side's nation (+0x1c, +0x53), its continent (+0x71, or 0) and
/// type 1. Friendly (0x5c6710 over `[0xae2c74]`): 0, 0 and 0xff. Otherwise, by the competition's key nation (0x669470,
/// signed): the nation from table 0xb63d60 (72 bytes each), the competition's continent (+0x59) and type 4 (keys 0 and
/// 1), 2 (3 to 7) or 1.
/// Original: `TRANSFER_MANAGER::get_fixture_rules_scope` (0x8d2fa0, transfer_manager.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x8d2fa0 } (thiscall, ret 0x10, transfer_manager.cpp): `TRANSFER_MANAGER::get_fixture_rules_scope(partida, tipo,
/// nação, continente)`: competição da partida (+0x14) de reservas ([0x9cf684]): a nação do mandante (+0x1c, +0x53), o
/// continente dela (+0x71, ou 0) e tipo 1. Amistoso (0x5c6710 sobre [0xae2c74]): 0, 0 e 0xff. Senão, pela nação-chave
/// da competição (0x669470, com sinal): a nação da tabela 0xb63d60 (72 bytes cada), o continente da competição (+0x59)
/// e tipo 4 (chaves 0 e 1), 2 (3 a 7) ou 1.
pub(crate) unsafe extern "thiscall" fn get_fixture_rules_scope(_this: usize, partida: usize, kind: *mut u8, nation: *mut usize, continent: *mut usize) {
    let comp = read_usize(partida + fixture::COMPETITION);
    if read_usize(comp) == read_usize(KEY_CLUB_COMP_RESERVE) {
        let n = read_usize(read_usize(partida + fixture::HOME_CLUB) + 0x53);
        *nation = n;
        *continent = if n != 0 { read_usize(n + nation::CONTINENT) } else { 0 };
        *kind = 1;
        return;
    }
    let friendly: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_FRIENDLY_IS_FRIENDLY_COMPETITION);
    if friendly(read_usize(G_FRIENDLY), comp) != 0 {
        *nation = 0;
        *continent = 0;
        *kind = 0xff;
        return;
    }
    let key: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(FN_KEY_NATION_INDEX_OF_COMP);
    let k = key(comp) as i8;
    *nation = read_usize((0xb63d60isize + k as isize * 72) as usize);
    *continent = read_usize(read_usize(partida + fixture::COMPETITION) + 0x59);
    *kind = match k {
        0 | 1 => 4,
        3..=7 => 2,
        _ => {
            crate::comparison::branch("cpk_4 escopo da nacao");
            1 // the competition’s nation
        }
    };
}

/// Returns the ineligibility code: 9 for an AWOL player (outside national sides); otherwise the first of the 28 rules
/// (+0x8ac) of the continent or nation with the same type answers (virtual +0x18); none: 0.
///
/// Ret 8: `TRANSFER_MANAGER::get_ineligibility_code(player, fixture)`: a home side (+0x1c) that is not a national side
/// (0x53b900) and a player absent without leave (AWOL: 0x418f60 over `[0xad9c6c]`, with 0, which clears the flag of
/// those without a club): 9. Otherwise, by the rules' scope (0x8d2fa0; 0xff and 4: 0): the first of the 28 rules at
/// +0x8ac of the continent (+0xe, type 2) or of the nation (+0xa) with the same type (+0x12) answers (virtual +0x18
/// with player and fixture). None: 0.
/// Original: `TRANSFER_MANAGER::get_ineligibility_code` (0x8cf5e0, transfer_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x8cf5e0 } (thiscall, ret 8, transfer_manager.cpp): `TRANSFER_MANAGER::get_ineligibility_code(jogador, partida)`:
/// mandante (+0x1c) que não é seleção (0x53b900) e jogador ausente sem licença (AWOL: 0x418f60 sobre [0xad9c6c], com 0,
/// que tira a marca de quem está sem clube): 9. Senão, pelo alcance das regras (0x8d2fa0; 0xff e 4: 0): a primeira das
/// 28 regras de +0x8ac do continente (+0xe, tipo 2) ou da nação (+0xa) com o mesmo tipo (+0x12) responde (virtual
/// +0x18 com jogador e partida). Nenhuma: 0.
pub(crate) unsafe extern "thiscall" fn get_ineligibility_code(this: usize, player: usize, partida: usize) -> u8 {
    if partida != 0 {
        let national_team: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
        if national_team(read_usize(partida + fixture::HOME_CLUB)) == 0 {
            let ausente: unsafe extern "thiscall" fn(usize, usize, u32) -> u8 = core::mem::transmute(FN_AWOL_IS_PLAYER_AWOL);
            if ausente(read_usize(G_AWOL), player, 0) != 0 {
                return 9;
            }
        }
    }
    let (mut tipo, mut nacao, mut continente) = (0u8, 0usize, 0usize);
    let escopo: unsafe extern "thiscall" fn(usize, usize, *mut u8, *mut usize, *mut usize) = core::mem::transmute(FN_TRANSFER_MANAGER_GET_FIXTURE_RULES_SCOPE);
    escopo(this, partida, &mut tipo, &mut nacao, &mut continente);
    if tipo == 0xff || tipo == 4 {
        return 0;
    }
    for k in 0..28 {
        let r = read_usize(this + transfer_manager_fields::TRANSFER_RULES + k * 4);
        let home = if tipo == 2 {
            read_usize(r + transfer_rules::CONTINENT_ID) == continente && *((r + transfer_rules::SCOPE) as *const u8) == 2
        } else {
            read_usize(r + transfer_rules::NATION_ID) == nacao && *((r + transfer_rules::SCOPE) as *const u8) == tipo // the nation’s rule
        };
        if home {
            crate::comparison::branch("cpk_4 regra da partida");
            let respond: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(read_usize(read_usize(r) + 0x18));
            return respond(r, player, partida);
        }
    }
    0
}

/// Transfer rules object i (pointers at this+0x8ac).
///
/// [pt] As regras de transferência (+0x8ac, 4 bytes cada) do índice `i` no gerenciador de transferências `this`.
pub(crate) unsafe fn transfer_rules_at(this: usize, i: usize) -> usize {
    read_usize(this + transfer_manager_fields::TRANSFER_RULES + i.wrapping_mul(4))
}

/// Writes the transfer deadline news text through the rules of the nation in news slot 0 (method +0x30).
///
/// Ret 0xc: `TRANSFER_MANAGER::write_deadline_news_text(news, full, text)`: the text of the transfer deadline news item
/// is left to the rules of the nation in slot 0 of the news item (0x785830): their method +0x30, with the same
/// arguments. Returns what it returns.
/// Original: `TRANSFER_MANAGER::write_deadline_news_text` (0x8e05a0, transfer_manager.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x8e05a0 } (thiscall, ret 0xc, transfer_manager.cpp): `TRANSFER_MANAGER::write_deadline_news_text(noticia,
/// completo, texto)`: o texto da notícia do fim do prazo de transferências fica com as regras da nação da vaga 0 da
/// notícia (0x785830): o método +0x30 delas, com os mesmos argumentos. Devolve o que ele devolve.
pub(crate) unsafe extern "thiscall" fn write_deadline_news_text(this: usize, news: usize, full: u32, text: usize) -> u32 {
    let slot: unsafe extern "thiscall" fn(usize, i8) -> u32 = core::mem::transmute(FN_NEWSITEM_GET_SLOT);
    let rules = transfer_rules_at(this, slot(news, 0) as usize); // the rules of slot 0
    let metodo: unsafe extern "thiscall" fn(usize, usize, u32, usize) -> u32 = core::mem::transmute(read_usize(read_usize(rules) + 0x30));
    metodo(rules, news, full, text)
}

/// Transfer rules of the club's nation if the player's owning club (+0x39) is in that nation; else None.
///
/// [pt] As regras (das 28 em +0x8ac) da nação do `clube` (+0x53), se o clube dono do `jogador` (+0x39) é da mesma nação;
/// senão nenhuma.
pub(crate) unsafe fn loan_transfer_rules(this: usize, club: usize, player: usize) -> Option<usize> {
    let owner = read_usize(player + staff::CLUB_JOB);
    if owner == 0 || read_usize(owner + club::NATION) != read_usize(club + club::NATION) {
        return None;
    }
    let nation = read_usize(club + club::NATION);
    (0..0x1c).map(|i| transfer_rules_at(this, i)).find(|&r| read_usize(r + transfer_rules::NATION_ID) == nation) // the nation of the rules
}

/// Tells whether a loan can end: with the player's parent club in the same nation as the club, that nation's rules
/// answer (method +0x24); else 1.
///
/// Ret 0x14: `TRANSFER_MANAGER::can_end_loan(club, player, a, b, c)`: with the club that owns the player (+0x39) in the
/// same nation (+0x53) as `club`, the rules of that nation answer (the first of the 28 at +0x8ac with an equal +0xa):
/// their method +0x24 with `club`, `player`, `a`, `b`, `c`. Otherwise 1.
/// Original: `TRANSFER_MANAGER::can_end_loan` (0x8d0530, transfer_manager.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x8d0530 } (thiscall, ret 0x14, transfer_manager.cpp): `TRANSFER_MANAGER::can_end_loan(clube, jogador, a, b,
/// c)`: com o clube dono do jogador (+0x39) na mesma nação (+0x53) do `clube`, quem responde são as regras dessa nação
/// (a primeira das 28 em +0x8ac com +0xa igual): o método +0x24 delas com `clube`, `jogador`, `a`, `b`, `c`. Senão 1.
pub(crate) unsafe extern "thiscall" fn can_end_loan(this: usize, club: usize, player: usize, a: u32, b: u32, c: u32) -> u32 {
    match loan_transfer_rules(this, club, player) {
        Some(r) => {
            let metodo: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) -> u32 = core::mem::transmute(read_usize(read_usize(r) + 0x24));
            metodo(r, club, player, a, b, c)
        }
        None => 1,
    }
}

/// Staff id -> person record (110 bytes each in G_STAFF_TABLE).
pub(crate) unsafe fn staff_record_by_id(i: u32) -> usize {
    (read_u32(G_STAFF_TABLE) as usize).wrapping_add((i as usize).wrapping_mul(110))
}

/// Club id -> club record (581 bytes each in G_CLUB_TABLE).
pub(crate) unsafe fn club_record_by_id(i: u32) -> usize {
    let i = i as usize;
    (read_u32(G_CLUB_TABLE) as usize).wrapping_add(i).wrapping_add(i.wrapping_mul(145).wrapping_mul(4))
}

/// Person needs a work permit to play at the club (0x8c7560 with 0).
///
/// [pt] A pessoa precisa de permissão para jogar no clube (0x8c7560, original, com 0 = sem ignorar as dadas)?
pub(crate) unsafe fn needs_work_permit(this: usize, person: usize, club: usize) -> bool {
    let precisa: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_PLAYER_NEEDS_WORK_PERMIT);
    precisa(this, person, club, 0) != 0
}

/// Returns 1 when the offer's player needs a permit at the offer club (0x8c7560) or one of the exchange players does at
/// his contract club; else 0.
///
/// Ret 4: `transfer_manager_offer_needs_work_permit(offer)`: 1 if the offer's player (+0xc) needs a permit at the
/// offer's club (+8), or if any of the exchange players (the list at +4, from the start: cursor +0xa goes back to +2
/// and advances through 0x68d1b0) needs one at the club of his contract (0x4cf5a0 on `[0xae1050]`, written into
/// `[0xae1080]`; without a contract, null club); otherwise 0.
/// Original: `transfer_manager_offer_needs_work_permit` (0x8c7470, transfer_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x8c7470 } (thiscall, ret 4, transfer_manager.cpp): `transfer_manager_offer_needs_work_permit(oferta)`: 1 se o
/// jogador da oferta (+0xc) precisa de permissão no clube da oferta (+8), ou se algum dos jogadores de troca (a lista
/// em +4, do começo: o cursor +0xa volta ao +2 e anda pela 0x68d1b0) precisa no clube do contrato dele (0x4cf5a0 em
/// [0xae1050], gravado em [0xae1080]; sem contrato, clube nulo); senão 0.
pub(crate) unsafe extern "thiscall" fn transfer_manager_offer_needs_work_permit(this: usize, offer: usize) -> u32 {
    if needs_work_permit(this, staff_record_by_id(read_u32(offer + offer::TARGET_STAFF_ID)), club_record_by_id(read_u32(offer + offer::BUYER_CLUB_ID))) {
        return 1; // the offer’s player
    }
    let list = read_u32(offer + offer::EXCHANGE_PLAYERS) as usize;
    if list == 0 {
        return 0;
    }
    core::ptr::write_unaligned((list + llist::CURSOR) as *mut u32, read_u32(list + llist::HEAD)); // the cursor at the start
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let mut item = next(list);
    while item != 0 {
        let c = contract_of(read_u32(G_CONTRACT_MANAGER) as usize, staff_record_by_id(read_u32(item)));
        *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
        let club = if c != 0 { club_record_by_id(read_u32(c + contract::CLUB)) } else { 0 };
        if needs_work_permit(this, staff_record_by_id(read_u32(item)), club) {
            return 1;
        }
        item = next(list);
    }
    0
}

/// Updates the work permits: clears the flag of whoever lacks bit 0x20; after more than 4 years grants the second
/// nationality and drops the permit; drops the permit of non-players; sorts and trims the dropped ones from the end.
///
/// `TRANSFER_MANAGER::update_work_permits`: in the permit vector (`[this+0x93a]`, `[this+0x93e]` records of 14 bytes:
/// +0 person, +8 year, +0xa nation), the person's flag (byte at `[this+0x86e]` + 8 × id) without bit 0x20 is zeroed
/// entirely; with more than 4 years (game year − +8) the person gains the second nationality (+0x1e = nation +0xa,
/// 0x122 bytes in `[0xae23a8]`), loses bit 0x20 and the permit is dropped (person −1); whoever is not a player (+0x61)
/// also loses it. If any are dropped: sorts (qsort 0x945333 with 0x8e55c0) and discounts those at the end with person
/// −1.
/// Original: `TRANSFER_MANAGER::update_work_permits` (0x8c7330, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8c7330 } (thiscall, transfer_manager.cpp): `TRANSFER_MANAGER::update_work_permits`: no vetor de permissões
/// ([this+0x93a], [this+0x93e] registros de 14 bytes: +0 pessoa, +8 ano, +0xa nação), a marca da pessoa (byte em
/// [this+0x86e] + 8 × id) sem o bit 0x20 é zerada inteira; com mais de 4 anos (ano do jogo − +8) a pessoa ganha a
/// segunda nacionalidade (+0x1e = nação +0xa, 0x122 bytes em [0xae23a8]), perde o bit 0x20 e a permissão sai (pessoa
/// −1); quem não é jogador (+0x61) também perde. Saindo alguma: reordena (qsort 0x945333 com 0x8e55c0) e desconta as
/// do fim com pessoa −1.
pub(crate) unsafe extern "thiscall" fn update_work_permits(this: usize) {
    let n = (this + transfer_manager_fields::WORK_PERMIT_COUNT) as *mut i32;
    let mut left = false;
    let mut i = 0i32;
    let mut off = 0usize;
    while i < *n {
        let r = (read_u32(this + transfer_manager_fields::WORK_PERMITS) as usize).wrapping_add(off);
        let person = staff_record_by_id(read_u32(r));
        let mark = |p: usize| ((read_u32(this + transfer_manager_fields::PERSON_FLAGS) as usize).wrapping_add((read_u32(p) as usize).wrapping_mul(8))) as *mut u8;
        if *mark(person) & 0x20 == 0 {
            *mark(person) = 0; // the mark without permission
        }
        let r = (read_u32(this + transfer_manager_fields::WORK_PERMITS) as usize).wrapping_add(off);
        let years = (*(G_CURRENT_DATE_YEAR as *const u16) as i32).wrapping_sub(*((r + work_permit::YEAR) as *const u16) as i32);
        if years > 4 {
            let nation = read_u32(r + work_permit::NATION_ID) as usize;
            core::ptr::write_unaligned((person + staff::SECOND_NATION) as *mut usize, (read_u32(G_NATION_TABLE) as usize).wrapping_add(nation.wrapping_mul(nation::SIZE)));
            *mark(person) &= 0xdf;
            core::ptr::write_unaligned(((read_u32(this + transfer_manager_fields::WORK_PERMITS) as usize).wrapping_add(off)) as *mut u32, u32::MAX);
            left = true;
        } else if read_u32(person + staff::PLAYER) == 0 {
            *mark(person) &= 0xdf;
            core::ptr::write_unaligned(((read_u32(this + transfer_manager_fields::WORK_PERMITS) as usize).wrapping_add(off)) as *mut u32, u32::MAX);
            left = true; // whoever is not a player
        }
        i += 1;
        off = off.wrapping_add(0xe);
    }
    if !left {
        return;
    }
    let qsort: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    qsort(read_u32(this + transfer_manager_fields::WORK_PERMITS) as usize, *n, 0xe, 0x8e55c0);
    let mut k = *n - 1;
    while k >= 0 {
        let r = (read_u32(this + transfer_manager_fields::WORK_PERMITS) as usize).wrapping_add(k as usize * 0xe);
        if read_u32(r) != u32::MAX {
            break;
        }
        *n -= 1;
        k -= 1;
    }
}

/// copies the past transfer (0x25 bytes): seven dwords, the byte +0x1c and the date (+0x1d)
/// Original: `past_transfer_copy_construct` (0x8cb940, transfer_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x8cb940 } (thiscall, ret 4, transfer_manager.cpp): `past_transfer_copy_construct`: copia a transferência passada
/// (0x25 bytes) de `origem`: sete duplas (+0..+0x18), o byte +0x1c e a data em +0x1d. Devolve `this`.
pub(crate) unsafe extern "thiscall" fn past_transfer_copy_construct(this: usize, source: usize) -> usize {
    for k in 0..7 {
        core::ptr::write_unaligned((this + past_transfer::FIRST_NAME_ID * k) as *mut u32, core::ptr::read_unaligned((source + past_transfer::FIRST_NAME_ID * k) as *const u32)); // the transfer pairs
    }
    *((this + past_transfer::MOVE_TYPE) as *mut u8) = *((source + past_transfer::MOVE_TYPE) as *const u8);
    copy_record_date(this + past_transfer::DATE, source + past_transfer::DATE);
    this
}

/// copies the future transfer (0xc bytes): the date (+0) and the offer index (+8)
/// Original: `future_transfer_assign` (0x8cbf40, transfer_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x8cbf40 } (thiscall, ret 4, transfer_manager.cpp): `future_transfer_assign`: copia a transferência futura (0xc
/// bytes) de `origem`: a data em +0 e o índice da oferta (+8). Devolve `this`.
pub(crate) unsafe extern "thiscall" fn future_transfer_assign(this: usize, source: usize) -> usize {
    copy_record_date(this, source);
    core::ptr::write_unaligned((this + future_transfer::OFFER_SLOT) as *mut u32, core::ptr::read_unaligned((source + future_transfer::OFFER_SLOT) as *const u32)); // the offer index
    this
}

/// Transfer rules of the nation among the 28 pointers at [manager+0x8ac] (matching +0xa), or 0.
///
/// [pt] As regras de transferência da nação: das 28 em [gerente+0x8ac] (ponteiros), a de nação (+0xa) igual;
/// 0 se nenhuma.
pub(crate) unsafe fn nation_transfer_rules(manager: usize, nation: usize) -> usize {
    (0..28)
        .map(|i| *((manager + transfer_manager_fields::TRANSFER_RULES + 4 * i) as *const usize))
        .find(|&r| core::ptr::read_unaligned((r + transfer_rules::NATION_ID) as *const usize) == nation)
        .unwrap_or(0)
}

/// Tells whether the club may sign the player under the nation rules, with the refusal reason and the human club's
/// second try on the foreign player limit.
///
/// Ret 0x18: whether `club` can sign `player`, by the rules of the club's nation (method +0x10 with club, player, a4,
/// a2 and the reason); without rules, 1. The reason (word) goes into `out` (zeroed first). Refused with `a5`, a human
/// club (0x600a50) and a foreign-player reason (0x11, 0x14, 0x15, 0x1f): asks again with the player's nation (+0x1a)
/// replaced by the club's (and restored afterwards); accepted that way, 1.
/// Original: `TRANSFER_MANAGER::can_sign_player` (0x8ced40, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8ced40 } (thiscall, ret 0x18, transfer_manager.cpp): se o `clube` pode contratar o `jogador`,
/// pelas regras da nação do clube (método +0x10 com clube, jogador, a4, a2 e o motivo); sem regras, 1. O
/// motivo (palavra) vai para `saida` (zerada antes). Recusado com `a5`, clube humano (0x600a50) e motivo
/// de estrangeiros (0x11, 0x14, 0x15, 0x1f): pergunta de novo com a nação do jogador (+0x1a) trocada pela
/// do clube (e devolvida depois); aceito assim, 1.
pub(crate) unsafe extern "thiscall" fn can_sign_player(manager: usize, club: usize, a2: u32, player: usize, a4: u32, a5: u32, out: *mut u16) -> u32 {
    let rules = nation_transfer_rules(manager, core::ptr::read_unaligned((club + club::NATION) as *const usize));
    if !out.is_null() {
        *out = 0;
    }
    if rules == 0 {
        return 1;
    }
    let consultar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, *mut u32) -> u32 =
        core::mem::transmute(*((*(rules as *const usize) + 0x10) as *const usize));
    // the reason is the slot of the `clube` argument on the original’s stack: with no answer from the rules the club stays
    let mut reason = club as u32;
    let r = consultar(rules, club, player, a4, a2, &mut reason);
    if !out.is_null() {
        *out = reason as u16;
    }
    if r != 0 {
        return r;
    }
    if a5 == 0 {
        return 0; // recusado, sem segunda pergunta
    }
    let human: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    if human(club, 0, 1, 0, 0) == 0 || !matches!(reason as u16, 0x11 | 0x14 | 0x15 | 0x1f) {
        if !out.is_null() {
            *out = reason as u16;
        }
        return 0;
    }
    let nation = (player + staff::NATION) as *mut u32;
    let stored = core::ptr::read_unaligned(nation);
    core::ptr::write_unaligned(nation, core::ptr::read_unaligned((club + club::NATION) as *const u32));
    let r = consultar(rules, club, player, a4, a2, &mut reason);
    core::ptr::write_unaligned(nation, stored);
    if r != 0 {
        return 1;
    }
    if !out.is_null() {
        *out = reason as u16;
    }
    0
}

/// Returns how many of the club's open offers include the player in the exchange, listing their indices when asked.
///
/// Ret 8: how many open offers of the club of `player`'s contract (0x4cf5a0) offer him in the exchange (list +4 of the
/// offer, a node with his id), noting the index (+0x20) in `out` (0x68d0d0) if there is one. Offers in state (+0x2c) 2,
/// 3, 4, 0x10, 0x14 or 0xf with +0x2e ≥ 3 do not count. Stops on reaching the number of exchanges recorded (`[+0x86e]`
/// + 8 × id + 2); recorded 0 or an id out of range, 0.
/// Original: `TRANSFER_MANAGER::get_exchange_offers_for_player` (0x8c5e40, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8c5e40 } (thiscall, ret 8, transfer_manager.cpp): quantas ofertas abertas do clube do contrato do
/// `jogador` (0x4cf5a0) o oferecem na troca (lista +4 da oferta, nó com o id dele), anotando o índice
/// (+0x20) em `saida` (0x68d0d0) se houver. Ofertas no estado (+0x2c) 2, 3, 4, 0x10, 0x14 ou 0xf com
/// +0x2e ≥ 3 não contam. Para ao chegar ao número de trocas marcado ([+0x86e] + 8 × id + 2); marcado 0
/// ou id fora da faixa, 0.
pub(crate) unsafe extern "thiscall" fn get_exchange_offers_for_player(manager: usize, player: usize, out: usize) -> u32 {
    if player != 0 && *(player as *const i32) >= *(G_STAFF_COUNT as *const i32) {
        return 0;
    }
    let limit = || *((*((manager + transfer_manager_fields::PERSON_FLAGS) as *const usize) as isize + *(player as *const i32) as isize * 8 + 2) as *const u8) as u32;
    if limit() == 0 {
        return 0; // sem trocas marcadas
    }
    if transfer_manager_contract_club(player) == 0 {
        return 0;
    }
    let club = *(transfer_manager_contract_club(player) as *const u32);
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let anexar: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_LLIST_ADD_TAIL);
    let mut count = 0u32;
    let mut i = 0;
    while i < *((manager + transfer_manager_fields::OFFER_CAPACITY) as *const i32) {
        let slot = (*(manager as *const usize) + 4 * i as usize) as *const usize;
        i += 1;
        let offer = *slot;
        if offer == 0 || *((offer + offer::BUYER_CLUB_ID) as *const u32) != club {
            continue;
        }
        let state = *((offer + offer::STATUS) as *const u8);
        if matches!(state, 2 | 3 | 4 | 0x10 | 0x14) || (state == 0xf && *((offer + offer::RESPONSE_ROUND) as *const u8) >= 3) {
            continue;
        }
        let l = *((offer + offer::EXCHANGE_PLAYERS) as *const usize);
        if l != 0 {
            core::ptr::write_unaligned((l + llist::CURSOR) as *mut u32, core::ptr::read_unaligned((l + llist::HEAD) as *const u32));
            loop {
                let no = next(*((*slot + 4) as *const usize));
                if no == 0 {
                    break;
                }
                if *(no as *const u32) == *(player as *const u32) {
                    if out != 0 {
                        anexar(out, *slot + 0x20, 4);
                    }
                    count += 1; // trocas
                    break;
                }
            }
        }
        if count == limit() {
            break;
        }
    }
    count
}

/// Walks a game LLIST from head (+2) and returns each node's data (+0), next at +6.
///
/// [pt] O nó de uma lista do jogo (0x68d0d0/0x68d1b0): contagem u16 em +0, cabeça +2, cauda +6, cursor +0xa;
/// cada nó tem o dado em +0 e o próximo em +6.
pub(crate) unsafe fn llist_data_values(l: usize) -> Vec<usize> {
    let mut v = Vec::new();
    let mut no = core::ptr::read_unaligned((l + llist::HEAD) as *const usize);
    while no != 0 {
        v.push(*(no as *const usize));
        no = core::ptr::read_unaligned((no + llist_node::NEXT) as *const usize);
    }
    v
}

/// Person -> club record of their contract (G_CLUB_TABLE + 581 x [contract+4]), contract stored in
/// G_CONTRACT_LOOKUP_RESULT; 0 without.
///
/// [pt] 0x4cf5a0 (o contrato do jogador `pessoa`, thiscall em [0xae1050]) gravado em [0xae1080]; devolve o
/// registro do clube do contrato ([0xae23b4] + 581 × [contrato+4]) ou 0 sem contrato.
pub(crate) unsafe fn transfer_manager_contract_club(person: usize) -> usize {
    let contract: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = contract(*(G_CONTRACT_MANAGER as *const usize), person);
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
    if c == 0 {
        return 0;
    }
    (*(G_CLUB_TABLE as *const usize)).wrapping_add((*((c + contract::CLUB) as *const usize)).wrapping_mul(581))
}

/// Closes (state 4) the contract offers linked to the transfer offer: the target player's and the exchange players'.
///
/// Stdcall, ret 4: closes (state +0x50 = 4) the contract proposals linked to the offer: that of the target player
/// (0x4cc9e0 with id +0xc and club +8) and that of each exchange player (list +4, with the club of the target's
/// contract, 0x4cf5a0).
/// Original: `transfer_manager_close_offer_contract_offers` (0x8c6db0, transfer_manager.cpp, stdcall).
///
/// [pt] { 0x8c6db0 } (stdcall, ret 4, transfer_manager.cpp): fecha (estado +0x50 = 4) as propostas de contrato
/// ligadas à oferta: a do jogador-alvo (0x4cc9e0 com o id +0xc e o clube +8) e a de cada jogador da troca
/// (lista +4, com o clube do contrato do alvo, 0x4cf5a0).
pub(crate) unsafe extern "stdcall" fn transfer_manager_close_offer_contract_offers(offer: usize) {
    let id = *((offer + offer::TARGET_STAFF_ID) as *const usize);
    let person = (*(G_STAFF_TABLE as *const usize)).wrapping_add(id.wrapping_mul(110));
    let club = transfer_manager_contract_club(person);
    let proposta: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_FIND_CONTRACT_OFFER);
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let r = proposta(*(G_CONTRACT_MANAGER as *const usize), id as u32, *((offer + offer::BUYER_CLUB_ID) as *const u32));
    if r != 0 {
        *((r + contract_offer::STATE) as *mut u8) = 4;
    }
    let l = *((offer + offer::EXCHANGE_PLAYERS) as *const usize);
    if l == 0 {
        return;
    }
    core::ptr::write_unaligned((l + llist::CURSOR) as *mut u32, core::ptr::read_unaligned((l + llist::HEAD) as *const u32));
    let mut e = next(l);
    while e != 0 {
        // without the target’s contract the original reads [0] (crashes); the copy too
        let t = proposta(*(G_CONTRACT_MANAGER as *const usize), *(e as *const u32), core::ptr::read_volatile(club as *const u32));
        if t != 0 {
            *((t + contract_offer::STATE) as *mut u8) = 4;
        }
        e = next(l);
    }
}

/// Port of 0x8c16b0: appends a 9-byte record (id, reason, compensation) to the free-transfer queue (+0x92c) via the
/// injected `anexar`; false if the person is on loan.
///
/// [pt] { 0x8c16b0 } (thiscall, ret 0xc, transfer_manager.cpp): põe o jogador na fila da transferência livre
/// (lista +0x92c, registro de 9 bytes: id, motivo, compensação). Emprestado (0x4cf6d0): a original
/// (caixa de erro).
pub(crate) unsafe fn queue_free_transfer_with(manager: usize, person: usize, reason: u8, compensacao: u32, anexar: &mut dyn FnMut(usize, &[u8; 9])) -> bool {
    let emprestado: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_LOAN_CONTRACT);
    if emprestado(*(G_CONTRACT_MANAGER as *const usize), person) != 0 {
        return false;
    }
    let mut r = [0u8; 9];
    r[..4].copy_from_slice(&(*(person as *const u32)).to_le_bytes());
    r[4] = reason;
    r[5..].copy_from_slice(&compensacao.to_le_bytes());
    anexar(manager + transfer_manager_fields::FREE_TRANSFER_QUEUE, &r);
    true
}

/// queues the player for a free transfer (id, reason and compensation), refusing players out on loan
/// Original: `TRANSFER_MANAGER::queue_free_transfer` (0x8c16b0, transfer_manager.cpp, thiscall).
///
/// [pt] põe o jogador na fila da transferência livre (id, motivo e compensação), recusando o emprestado
pub(crate) unsafe extern "thiscall" fn queue_free_transfer(manager: usize, person: usize, reason: u8, compensacao: u32) {
    let anexar_9: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_LLIST_ADD_TAIL);
    if !queue_free_transfer_with(manager, person, reason, compensacao, &mut |l, r| anexar_9(l, r.as_ptr() as usize, 9)) {
        original_fn::<unsafe extern "thiscall" fn(usize, usize, u8, u32)>(IDX_QUEUE_FREE_TRANSFER)(manager, person, reason, compensacao);
    }
}

/// Counts the open negotiations of the player's club that include him in a part exchange and, if asked, lists the offer
/// indices.
///
/// Ret 8: counts the negotiations (`[+0xc]`, `[+0x10]` of them) of the club of the player's contract (+8) whose offer
/// (`[manager]` + 4 × `[+0x20]`) is still open (states 2, 3, 4, 0x10 and 0x14 not; 0xf only with +0x2e < 3) and that
/// have the player in the exchange list (+4); with `out`, puts the offer index in it (0x68d0d0, 4 bytes). A player
/// outside the database or without a contract: 0.
/// Original: `TRANSFER_MANAGER::get_exchange_negotiations_for_player` (0x8c5fc0, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8c5fc0 } (thiscall, ret 8, transfer_manager.cpp): conta as negociações ([+0xc], [+0x10] delas) do
/// clube do contrato do jogador (+8) cuja oferta ([gerente] + 4 × [+0x20]) segue aberta (estados 2, 3, 4,
/// 0x10 e 0x14 não; 0xf só com +0x2e < 3) e que têm o jogador na lista da troca (+4); com `saida`, põe
/// nela o índice da oferta (0x68d0d0, 4 bytes). Jogador fora do banco ou sem contrato: 0.
pub(crate) unsafe extern "thiscall" fn get_exchange_negotiations_for_player(manager: usize, person: usize, out: usize) -> i32 {
    if person != 0 && *(person as *const i32) >= *(G_STAFF_COUNT as *const i32) {
        return 0;
    }
    // like `transfer_manager_contract_club`, kept separate so this routine’s mutation does not fall into 0x8c6db0
    let contract: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = contract(*(G_CONTRACT_MANAGER as *const usize), person);
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
    if c == 0 {
        return 0;
    }
    let p = (*(G_CLUB_TABLE as *const usize)).wrapping_add((*((c + contract::CLUB) as *const usize)).wrapping_mul(581));
    if p == 0 {
        return 0;
    }
    let club = *(p as *const u32);
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let anexar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_LLIST_ADD_TAIL);
    let negociacoes = *((manager + staff::COMMON_NAME) as *const usize);
    let mut total = 0;
    let mut i = 0;
    while i < *((manager + staff::DATE_OF_BIRTH) as *const i32) {
        let neg = *((negociacoes + i as usize * 4) as *const usize);
        i += 1;
        if neg == 0 || *((neg + offer::BUYER_CLUB_ID) as *const u32) != club {
            continue;
        }
        let k = *((neg + offer::SLOT_INDEX) as *const usize);
        let offer = *((*(manager as *const usize)).wrapping_add(k.wrapping_mul(4)) as *const usize);
        if offer == 0 {
            continue;
        }
        match *((offer + offer::STATUS) as *const u8) {
            2 | 3 | 4 | 0x10 | 0x14 => continue,
            0xf if *((offer + offer::RESPONSE_ROUND) as *const u8) >= 3 => continue,
            _ => {}
        }
        let l = *((neg + offer::EXCHANGE_PLAYERS) as *const usize);
        if l == 0 {
            continue;
        }
        core::ptr::write_unaligned((l + llist::CURSOR) as *mut u32, core::ptr::read_unaligned((l + llist::HEAD) as *const u32));
        let mut e = next(l);
        while e != 0 && *(e as *const u32) != *(person as *const u32) {
            e = next(l);
        }
        if e == 0 {
            continue;
        }
        if out != 0 {
            anexar(out, neg + offer::SLOT_INDEX, 4);
        }
        total += 1;
    }
    total
}

/// First offer in the manager's vector [this] (count [this+4]) whose target id (+0xc) is `id`, or 0.
///
/// [pt] A oferta do gerente de transferências para a pessoa `id`: a primeira das [this+4] em [this] com
/// +0xc = `id`, ou 0.
pub(crate) unsafe fn offer_for_player(this: usize, id: i32) -> usize {
    let offers = *(this as *const usize);
    let n = *((this + transfer_manager_fields::OFFER_CAPACITY) as *const i32);
    (0..n.max(0) as usize)
        .map(|i| *((offers + 4 * i) as *const usize))
        .find(|&e| e != 0 && *((e + offer::TARGET_STAFF_ID) as *const i32) == id)
        .unwrap_or(0)
}

/// Counts the transfer offers for a player (skipping closed ones unless asked) and may list their indices.
///
/// Ret 0xc: counts the offers for the person and, with `list`, appends to it (0x68d0d0, 4 bytes) the index of each one.
/// Id from 0xae2364 up or no offer: 0. The offer found without its own list (+0) counts 1 (appends +0x20); with a list,
/// goes through its indices (cursor +0xa restarted from +2) and counts those pointing to offers for the same person.
/// Without `all`, the closed ones are left out (0x8d0d90, inlined here as in the original).
/// Original: `TRANSFER_MANAGER::get_offers_for_player` (0x8c5cf0, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8c5cf0 } (thiscall, ret 0xc, transfer_manager.cpp): conta as ofertas pela pessoa e, com
/// `lista`, acrescenta nela (0x68d0d0, 4 bytes) o índice de cada uma. Id a partir de 0xae2364 ou sem
/// oferta: 0. A oferta achada sem lista própria (+0) conta 1 (acrescenta +0x20); com lista, percorre
/// os índices dela (cursor +0xa reiniciado de +2) e conta os que apontam para ofertas da mesma pessoa.
/// Sem `todas`, ficam de fora as encerradas (0x8d0d90, aqui embutida como na original).
pub(crate) unsafe extern "thiscall" fn get_offers_for_player(this: usize, person: *const i32, list: usize, all: u32) -> u32 {
    // the original reads the id before testing the pointer
    let id = core::ptr::read_volatile(person);
    if !person.is_null() && id >= *(G_STAFF_COUNT as *const i32) {
        return 0;
    }
    let e = offer_for_player(this, id);
    if e == 0 {
        return 0;
    }
    let append: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_LLIST_ADD_TAIL);
    let next: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_LLIST_GET_NEXT);
    let l = *(e as *const usize);
    if l == 0 {
        if all == 0 && transfer_offer_closed(e) != 0 {
            return 0;
        }
        if list != 0 {
            append(list, e + offer::SLOT_INDEX, 4);
        }
        return 1;
    }
    core::ptr::write_unaligned((l + llist::CURSOR) as *mut u32, core::ptr::read_unaligned((l + llist::HEAD) as *const u32));
    let mut count = 0u32;
    loop {
        let no = next(*(e as *const usize)) as usize;
        if no == 0 {
            break;
        }
        let k = *(no as *const usize);
        let o = *((*(this as *const usize)).wrapping_add(k.wrapping_mul(4)) as *const usize);
        if o == 0 || *((o + offer::TARGET_STAFF_ID) as *const i32) != id {
            continue;
        }
        if all == 0 && transfer_offer_closed(o) != 0 {
            continue;
        }
        if list != 0 {
            append(list, no, 4);
        }
        count += 1;
    }
    count
}

/// 9-byte entry of the club (id +0) in the shopping list [this+0xdc].
///
/// [pt] O registro de 9 bytes do clube (id em +0) na lista de compras [this+0xdc].
pub(crate) unsafe fn shopping_list_entry(this: usize, club: usize) -> usize {
    (*(club as *const i32) as usize).wrapping_mul(9).wrapping_add(*((this + shortlist_manager_fields::CLUB_SHORTLISTS) as *const usize))
}

/// Clears bit `bit` of the player's flags (+0x86e) and removes their 13-byte record from the payment list at
/// [manager+lista], shifting the rest down and decrementing the count.
///
/// [pt] O comum das três: desliga o bit `bit` das marcas do jogador `*id` (+0x86e, 8 bytes por pessoa) e
/// tira o registro dele (13 bytes, o id em `chave`) da lista [gerente+`lista`] (quantos em
/// [gerente+`quantos`]), puxando os seguintes uma vaga para trás.
pub(crate) unsafe fn remove_from_payment_list(manager: usize, id: *const i32, bit: u8, list: usize, how_many: usize, key: usize) {
    let marks = (*((manager + transfer_manager_fields::PERSON_FLAGS) as *const usize) as isize + *id as isize * 8) as *mut u8;
    if *marks & bit != 0 {
        *marks -= bit;
    }
    let n = *((manager + how_many) as *const i32);
    let base = *((manager + list) as *const usize);
    let Some(i) = (0..n.max(0) as usize).find(|&i| core::ptr::read_unaligned((base + i * 13 + key) as *const i32) == *id) else {
        return;
    };
    let mut j = i + 1;
    while (j as i32) < *((manager + how_many) as *const i32) {
        let source = *((manager + list) as *const usize) + j * 13;
        core::ptr::copy(source as *const u8, (base + (j - 1) * 13) as *mut u8, 13);
        j += 1;
    }
    *((manager + how_many) as *mut i32) -= 1;
}

/// removes the player's next sale percentage (flag and list record)
/// Original: `TRANSFER_MANAGER::remove_next_sale_percentage` (0x8cb4f0, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8cb4f0 } (thiscall, ret 4, transfer_manager.cpp): tira o percentual da próxima venda (bit 8;
/// lista +0x880, quantos +0x888, o id em +9).
pub(crate) unsafe extern "thiscall" fn remove_next_sale_percentage(manager: usize, id: *const i32) {
    remove_from_payment_list(manager, id, 8, 0x880, 0x888, 9);
}

/// removes the first-team appearance bonus (flag and list record)
/// Original: `TRANSFER_MANAGER::remove_first_team_appearance_money` (0x8cb590, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8cb590 } (thiscall, ret 4, transfer_manager.cpp): tira o bônus por jogo no time principal (bit 2;
/// lista +0x88c, quantos +0x894, o id em +0).
pub(crate) unsafe extern "thiscall" fn remove_first_team_appearance_money(manager: usize, id: *const i32) {
    remove_from_payment_list(manager, id, 2, 0x88c, 0x894, 0);
}

/// removes the international appearance bonus (flag and list record)
/// Original: `TRANSFER_MANAGER::remove_international_team_appearance_money` (0x8cb630, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8cb630 } (thiscall, ret 4, transfer_manager.cpp): tira o bônus por jogo pela seleção (bit 4;
/// lista +0x898, quantos +0x8a0, o id em +0).
pub(crate) unsafe extern "thiscall" fn remove_international_team_appearance_money(manager: usize, id: *const i32) {
    remove_from_payment_list(manager, id, 4, 0x898, 0x8a0, 0);
}

/// Comparer regions for the payment-list removals: the flags byte, the counter and the whole list.
///
/// [pt] As regiões das três: o byte das marcas, a lista inteira e o contador.
pub(crate) unsafe fn payment_list_regions(manager: usize, id: *const i32, list: usize, how_many: usize) -> Vec<(usize, usize)> {
    let marks = (*((manager + transfer_manager_fields::PERSON_FLAGS) as *const usize) as isize + *id as isize * 8) as usize;
    let n = (*((manager + how_many) as *const i32)).max(0) as usize;
    let mut r = vec![(marks, 1), (manager + how_many, 4)];
    if n > 0 {
        r.push((*((manager + list) as *const usize), n * 13));
    }
    r
}

/// Returns the club of the person's agreed contract (clears the flag when the contract is gone).
///
/// Ret 8: with the agreed contract flag (0x40) of person `*id`, looks for the contract (0x4cca30 on `[0xae1050]`):
/// found, writes into `club` (if not null) the contract's club (`[0xae23b4]`, 581 bytes per club, by the id at +4) and
/// returns 1; not found, clears the flag and returns 0.
/// Original: `TRANSFER_MANAGER::get_agreed_contract_club` (0x8c5450, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8c5450 } (thiscall, ret 8, transfer_manager.cpp): com a marca de contrato acertado (0x40) da
/// pessoa `*id`, procura o contrato (0x4cca30 em [0xae1050]): achou, grava em `clube` (se não nulo) o
/// clube do contrato ([0xae23b4], 581 bytes por clube, pelo id em +4) e devolve 1; não achou, desliga a
/// marca e devolve 0.
pub(crate) unsafe extern "thiscall" fn get_agreed_contract_club(manager: usize, id: *const i32, club: *mut usize) -> u32 {
    let marks = |g: usize| (*((g + transfer_manager_fields::PERSON_FLAGS) as *const usize) as isize + *id as isize * 8) as *mut u8;
    if *marks(manager) & 0x40 == 0 {
        return 0;
    }
    let procurar: unsafe extern "thiscall" fn(usize, i32) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_FIND_AGREED_CONTRACT_OFFER);
    let contract = procurar(*(G_CONTRACT_MANAGER as *const usize), *id);
    if contract == 0 {
        *marks(manager) &= 0xbf;
        return 0;
    }
    if !club.is_null() {
        *club = (*(G_CLUB_TABLE as *const usize) as isize + *((contract + contract::CLUB) as *const i32) as isize * 581) as usize;
    }
    1
}

/// Address of person *id's 8-byte record in the transfer manager (+0x86e).
///
/// [pt] O registro de 8 bytes da pessoa `*id` no gerente de transferências (+0x86e).
pub(crate) unsafe fn staff_flags_entry(manager: usize, id: *const i32) -> usize {
    (*((manager + transfer_manager_fields::PERSON_FLAGS) as *const usize) as isize + *id as isize * 8) as usize
}

/// records the player's national team match in the recent-matches bits
/// Original: `TRANSFER_MANAGER::record_international_appearance` (0x8ce920, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8ce920 } (thiscall, ret 8, transfer_manager.cpp): registra que o jogador jogou o jogo `numero`
/// da seleção: os bits dos últimos jogos (+3, 16 bits) andam um por jogo desde o último registrado
/// (+5), o mais novo é ligado e o número fica em +5.
pub(crate) unsafe extern "thiscall" fn record_international_appearance(manager: usize, id: *const i32, number: i16) {
    let r = staff_flags_entry(manager, id);
    let bits = (r + person_transfer_record::RECENT_INTERNATIONAL_BITS) as *mut u16;
    let last = (r + person_transfer_record::LAST_INTERNATIONAL_MATCH) as *mut i16;
    let d = number.wrapping_sub(core::ptr::read_unaligned(last));
    for _ in 0..d.max(0) {
        core::ptr::write_unaligned(bits, core::ptr::read_unaligned(bits) << 1);
        *(bits as *mut u8) &= 0xfe;
    }
    *(bits as *mut u8) |= 1;
    core::ptr::write_unaligned(last, number);
}

/// Pointer to person *id's flags byte in the transfer manager (+0x86e, 8 bytes per person).
///
/// [pt] O byte das marcas da pessoa `*id` no gerente de transferências (+0x86e, 8 bytes por pessoa).
pub(crate) unsafe fn staff_flags_ptr(manager: usize, id: *const i32) -> *mut u8 {
    (*((manager + transfer_manager_fields::PERSON_FLAGS) as *const usize) as isize + *id as isize * 8) as *mut u8
}

/// sets the person's agreed-contract flag (0x40) in the transfer manager
/// Original: `set_agreed_contract_flag` (0x8c5410, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8c5410 } (thiscall, ret 4, transfer_manager.cpp): liga a marca de contrato acertado (0x40).
pub(crate) unsafe extern "thiscall" fn set_agreed_contract_flag(manager: usize, id: *const i32) {
    *staff_flags_ptr(manager, id) |= 0x40;
}

/// clears the person's agreed-contract flag
/// Original: `clear_agreed_contract_flag` (0x8c5430, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8c5430 } (thiscall, ret 4, transfer_manager.cpp): desliga a marca de contrato acertado.
pub(crate) unsafe extern "thiscall" fn clear_agreed_contract_flag(manager: usize, id: *const i32) {
    *staff_flags_ptr(manager, id) &= 0xbf;
}

/// the address of an offer slot (out of range, the table start)
/// Original: `offer_slot` (0x8c61e0, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8c61e0 } (thiscall, ret 4, transfer_manager.cpp): o endereço da vaga `k` das ofertas (+0,
/// +4 quantas); fora da faixa, o próprio começo da tabela (o valor de +0, não o endereço).
pub(crate) unsafe extern "thiscall" fn offer_slot(table: usize, k: i32) -> usize {
    let base = *(table as *const usize);
    if k < *((table + transfer_manager_fields::OFFER_CAPACITY) as *const i32) { (base as isize + 4 * k as isize) as usize } else { base }
}

/// Returns the selection limit from the match restriction table (0x8cf6b0), the first one present: type at +0, count at
/// +5 (and the age at +1 for type 3).
///
/// Ret 8: `get_selection_limit(fixture, out)`: zeroes the 6 bytes of the output, asks for the fixture's 42-byte
/// restrictions table (0x8cf6b0 with 1; `this` is passed straight through) and keeps the first one that exists (≠
/// 0xff): +5 or +3 (type 1), +2 or +0 (type 2), +0x16 with the count in +0x17 (type 3, the age in +1). The type goes in
/// +0 and the count in +5.
/// Original: `get_selection_limit` (0x8cf7e0, transfer_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x8cf7e0 } (thiscall, ret 8, transfer_manager.cpp): `get_selection_limit(partida, saida)`: zera os 6 bytes da
/// saída, pede a tabela de 42 bytes das restrições da partida (0x8cf6b0 com 1; o `this` passa direto) e fica com a
/// primeira que existe (≠ 0xff): +5 ou +3 (tipo 1), +2 ou +0 (tipo 2), +0x16 com a quantidade em +0x17 (tipo 3, a
/// idade em +1). O tipo vai em +0 e a quantidade em +5.
pub(crate) unsafe extern "thiscall" fn get_selection_limit(this: usize, partida: usize, out: *mut u8) {
    core::ptr::write_unaligned(out as *mut u32, 0);
    core::ptr::write_unaligned(out.add(4) as *mut u16, 0);
    let mut t = [0u8; 0x2c];
    let restricoes: unsafe extern "thiscall" fn(usize, usize, *mut u8, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_FILL_SQUAD_RESTRICTIONS);
    restricoes(this, partida, t.as_mut_ptr(), 1);
    let (tipo, n) = if t[5] != 0xff {
        (1, t[5])
    } else if t[3] != 0xff {
        (1, t[3])
    } else if t[2] != 0xff {
        (2, t[2]) // o 3º limite
    } else if t[0] != 0xff {
        (2, t[0])
    } else if t[0x16] != 0xff {
        *out.add(1) = t[0x16];
        (3, t[0x17])
    } else {
        return;
    };
    *out = tipo;
    *out.add(5) = n;
}

/// Finds the person's work permit: with the 0x20 flag of the id in `[+0x86e]`, the binary search for the permit
/// (0xe-byte entries in `[+0x93a]`); else 0.
///
/// Ret 4: `TRANSFER_MANAGER::find_work_permit(person)`: the person's work permit: id outside the `[0xae2364]` (signed),
/// 0; without flag 0x20 in the id's byte (8 bytes each, in `[+0x86e]`), 0; otherwise the binary search (bsearch
/// 0x946464, 0xe-byte records in `[+0x93a]`, `[+0x93e]` of them, comparison 0x8e55c0) by the id. Null person: reads
/// address 0 as the original.
/// Original: `TRANSFER_MANAGER::find_work_permit` (0x8c7260, transfer_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x8c7260 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::find_work_permit(pessoa)`: a permissão
/// de trabalho da pessoa: id fora dos [0xae2364] (com sinal), 0; sem a marca 0x20 no byte do id (8 bytes cada, em
/// [+0x86e]), 0; senão a busca binária (bsearch 0x946464, registros de 0xe bytes em [+0x93a], [+0x93e] deles,
/// comparação 0x8e55c0) pelo id. Pessoa nula: lê o endereço 0 como o original.
pub(crate) unsafe extern "thiscall" fn find_work_permit(this: usize, person: *const i32) -> usize {
    let id = core::ptr::read_volatile(person);
    if !person.is_null() && id >= read_usize(G_STAFF_COUNT) as i32 {
        return 0;
    }
    if *((read_usize(this + transfer_manager_fields::PERSON_FLAGS).wrapping_add((id as usize).wrapping_mul(8))) as *const u8) & 0x20 == 0 {
        return 0; // without the permission flag
    }
    let key = id;
    let bsearch: unsafe extern "C" fn(*const i32, usize, usize, usize, usize) -> usize = core::mem::transmute(FN_CRT_BSEARCH);
    bsearch(&key, read_usize(this + transfer_manager_fields::WORK_PERMITS), read_usize(this + transfer_manager_fields::WORK_PERMIT_COUNT), 0xe, 0x8e55c0)
}

/// Fills a vacated staff job: the first unemployed person of the club's nation in the wanted job (6, 8, 9, 10; 13 → 6,
/// 15 → 8) joins the club (0x53bd10).
///
/// Stdcall, ret 8: `fill_vacated_staff_job(club, job)`: the staff vacancy that became free is filled by the first
/// unemployed person of the club's nation in the job sought (6, 8, 9 and 10; 13 looks for 6, 15 looks for 8): 0x53bd10
/// with the job (the argument's dword), today's date `[0xae2c90]`, write and without reducing. Null club: the original
/// routine ("Error" box).
/// Original: `fill_vacated_staff_job` (0x8cb6d0, transfer_manager.cpp, stdcall, ret 8).
///
/// [pt] { 0x8cb6d0 } (stdcall, ret 8, transfer_manager.cpp): `fill_vacated_staff_job(clube, cargo)`: a vaga da comissão
/// que ficou livre é preenchida pela primeira pessoa desempregada da nação do clube no cargo procurado (6, 8, 9 e 10;
/// 13 procura 6, 15 procura 8): 0x53bd10 com o cargo (o dword do argumento), a data de hoje [0xae2c90], gravar e sem
/// reduzir. Clube nulo: a original (caixa "Error").
pub(crate) unsafe extern "stdcall" fn fill_vacated_staff_job(club: usize, job: u32) {
    if club == 0 {
        return original_fn::<unsafe extern "stdcall" fn(usize, u32)>(IDX_FILL_VACATED_STAFF_JOB)(club, job); // message box: original only
    }
    let Some(c) = job_sought_by_slot(job) else { return };
    if let Some(p) = first_unemployed_staff(club, c as u8) {
        let incluir: unsafe extern "C" fn(usize, usize, u32, usize, u32, u32) -> u32 = core::mem::transmute(FN_DATABASE_ADD_STAFF_TO_CLUB);
        incluir(club, p, c, G_CURRENT_DATE, 1, 0);
    }
}

/// is the person in list +0x92c?
/// Original: `person_in_transfer_list` (0x8c1870, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8c1870 } (thiscall, ret 4, transfer_manager.cpp): a pessoa `*p` está na lista +0x92c (recomeça
/// o iterador +0xa de +2 e anda pela 0x68d1b0, comparando o primeiro dword de cada item)?
pub(crate) unsafe extern "thiscall" fn person_in_transfer_list(t: usize, p: *const i32) -> u32 {
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let list = t + transfer_manager_fields::FREE_TRANSFER_QUEUE;
    core::ptr::write_unaligned((list + llist::CURSOR) as *mut u32, core::ptr::read_unaligned((t + transfer_manager_fields::FREE_TRANSFER_QUEUE_HEAD) as *const u32));
    loop {
        let e = next(list);
        if e == 0 {
            return 0;
        }
        if *p == *(e as *const i32) {
            return 1;
        }
    }
}

/// The player refuses the terms: the offer returns in 10 days, the bid markers go, his reputation rises if the buyer is
/// more famous; or he leaves the exchange and his value moves to the fee.
///
/// Ret 0xc: `TRANSFER_MANAGER::handle_player_rejected_terms(person, index, silent)`: index outside vector `[this]`
/// (count +4): the "Error" box (0xf49, 0xf4e). Empty slot: nothing. If the person is the offer's player (+0xc): terms
/// refused (+0x10 = 4), status 0xf; +0x2e goes up with a human buyer (0x600a50 without holidays) and +0x1f ≠ 1,
/// otherwise 3; the bid marks (0x84aeb0) of the buyer and of the owner of each exchange player are removed; the list
/// manager notes the refusal (0x843e10) if the buyer is the computer; rescheduled in 10 days (0x8c6220); resolved with
/// all rivals resolved, bit 0 of the player's 8-byte record (`[this+0x86e]`) is cleared; the contract proposals
/// (0x4cca70) of the exchange players with the seller are removed. Without `silent`: with a buyer more famous than the
/// seller (+0x80), the player's reputation (+0xb of sheet +0x61) rises by the difference × 0.075 (above 7750 with the
/// sheet below 4750), × 0.05 (above 4250) or × 0.025, up to 10000; the reply news (0x8d8e90). Otherwise, the person is
/// an exchange player (+4; empty list: box 0x101f): leaves the list (0x68d1f0; the empty list is freed), her value
/// (0x4d0c60) moves to the price (+0x1b), her 8-byte record loses an offer (+2), the news (without `silent`), and the
/// reaction of the computer seller (0x8c8090) and of the buyer (0x8c0810), stopping if the offer became resolved. At
/// the end, the offer goes on (0x8c2f50).
/// Original: `handle_player_rejected_terms` (0x8c28c0, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8c28c0 } (thiscall, ret 0xc, transfer_manager.cpp): `TRANSFER_MANAGER::handle_player_rejected_terms(pessoa,
/// índice, silencioso)`: índice fora do vetor [this] (conta +4): a caixa "Error" (0xf49, 0xf4e). Vaga vazia: nada.
/// Se a pessoa é o jogador da oferta (+0xc): termos recusados (+0x10 = 4), status 0xf; +0x2e sobe com comprador humano
/// (0x600a50 sem as férias) e +0x1f ≠ 1, senão 3; saem as marcas de lance (0x84aeb0) do comprador e do dono de cada
/// jogador da troca; o gerente de listas anota a recusa (0x843e10) se o comprador é do computador; reagendada em 10
/// dias (0x8c6220); resolvida com todas as rivais resolvidas, o bit 0 do registro de 8 bytes do jogador ([this+0x86e])
/// sai; saem as propostas de contrato (0x4cca70) dos jogadores da troca com o vendedor. Sem `silencioso`: com comprador
/// mais famoso que o vendedor (+0x80), a reputação do jogador (+0xb da ficha +0x61) sobe a diferença × 0,075 (acima de
/// 7750 com ficha abaixo de 4750), × 0,05 (acima de 4250) ou × 0,025, até 10000; a notícia da resposta (0x8d8e90).
/// Se não, a pessoa é um jogador da troca (+4; lista vazia: a caixa 0x101f): sai da lista (0x68d1f0; a lista vazia é
/// liberada), o valor dela (0x4d0c60) passa ao preço (+0x1b), o registro de 8 bytes dela perde uma oferta (+2), a
/// notícia (sem `silencioso`), e a reação do vendedor do computador (0x8c8090) e do comprador (0x8c0810), parando se a
/// oferta ficou resolvida. No fim, a oferta segue (0x8c2f50).
pub(crate) unsafe extern "thiscall" fn handle_player_rejected_terms(this: usize, person: usize, index: i32, silencioso: u32) {
    let human_club: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let clear_marks: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_SHORTLIST_MANAGER_CLEAR_BID_MARKERS_FOR_PLAYER);
    let anotar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(FN_SHORTLIST_MANAGER_UPDATE_STAFF_STATUS);
    let schedule: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_SCHEDULE_OFFER);
    let tirar_proposta: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_CONTRACT_MANAGER_REMOVE_CONTRACT_OFFER);
    let news: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, usize, u32, u32, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_POST_CONTRACT_RESPONSE_NEWS);
    let value: unsafe extern "thiscall" fn(usize, usize, usize, u32, i32, u32) -> i32 = core::mem::transmute(FN_CONTRACT_MANAGER_CALC_PLAYER_VALUE);
    let remove: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_LLIST_REMOVE);
    let destroy: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let free: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_OPERATOR_DELETE);
    let reply_of_seller: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_AI_RESPOND_TO_BID);
    let reply_of_buyer: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_REVIEW_AI_CLUB_BID);
    let follow: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_ADVANCE_OFFER_TO_CLUB_APPROVAL);
    if index < 0 {
        return show_error_at(0xa8d514, 0xf49);
    }
    if index >= read_usize(this + transfer_manager_fields::OFFER_CAPACITY) as i32 {
        return show_error_at(0xa8d514, 0xf4e);
    }
    let slot = || read_usize(this).wrapping_add(index as usize * 4);
    let o = read_usize(slot());
    if o == 0 {
        return;
    }
    let price = read_usize(o + offer::FEE);
    if read_usize(person) == read_usize(o + offer::TARGET_STAFF_ID) {
        write_u8(o + offer::TERMS_STATE, 4);
        let buyer = friendly_club_by_id(read_usize(o + offer::BUYER_CLUB_ID));
        let seller = offer_contract_club(transfer_manager_main_contract_stored(person), true);
        write_u8(o + offer::STATUS, 0xf);
        if silencioso == 0 && human_club(buyer, 1, 1, 0, 0) != 0 && read_u8(o + offer::OFFER_TYPE) != 1 {
            write_u8(o + offer::RESPONSE_ROUND, read_u8(o + offer::RESPONSE_ROUND).wrapping_add(1));
        } else {
            write_u8(o + offer::RESPONSE_ROUND, 3);
        }
        let p = staff_records_staff_by_id(read_usize(o + offer::TARGET_STAFF_ID));
        let owner = offer_contract_club(transfer_manager_main_contract_stored(p), false);
        clear_marks(read_usize(this + transfer_manager_fields::SHORTLIST_MANAGER), friendly_club_by_id(read_usize(o + offer::BUYER_CLUB_ID)), p);
        let l = read_usize(o + offer::EXCHANGE_PLAYERS);
        if l != 0 {
            transfer_manager_llist_rewind(l);
            loop {
                let y = list_next_item(l);
                if y == 0 {
                    break;
                }
                clear_marks(read_usize(this + transfer_manager_fields::SHORTLIST_MANAGER), owner, staff_records_staff_by_id(read_usize(y)));
            }
        }
        if human_club(buyer, 0, 1, 0, 0) == 0 {
            crate::comparison::branch("cpv_2 recusa anotada na lista");
            // the original tests the byte with eax zeroed by the previous query (above, zero)
            anotar(read_usize(this + transfer_manager_fields::SHORTLIST_MANAGER), person, buyer, 1, (read_u8(o + offer::JOB_TYPE) != 0xb) as u32); // cpv2: the refusal in the list
        }
        crate::comparison::branch("cpv_2 recusa do jogador");
        schedule(this, o, 0xa, 0); // cpv2: back in 10 days
        if is_offer_resolved(o) {
            let rivais = read_usize(o);
            let mut all = true;
            if rivais != 0 {
                transfer_manager_llist_rewind(rivais);
                loop {
                    let x = list_next_item(rivais);
                    if x == 0 {
                        break;
                    }
                    if !is_offer_resolved(read_usize(read_usize(this) + read_usize(x) * 4)) {
                        all = false;
                        break;
                    }
                }
            }
            if all {
                let m = read_usize(this + transfer_manager_fields::PERSON_FLAGS).wrapping_add(read_usize(o + offer::TARGET_STAFF_ID) * 8);
                if read_u8(m) & 1 != 0 {
                    write_u8(m, read_u8(m) & 0xfe);
                }
            }
        }
        let l = read_usize(o + offer::EXCHANGE_PLAYERS);
        if l != 0 {
            transfer_manager_llist_rewind(l);
            loop {
                let y = list_next_item(l);
                if y == 0 {
                    break;
                }
                tirar_proposta(read_usize(G_CONTRACT_MANAGER), staff_records_staff_by_id(read_usize(y)), seller);
            }
        }
        if silencioso == 0 {
            let sheet = read_usize(person + staff::PLAYER);
            if buyer != 0 && seller != 0 && sheet != 0 {
                let rc = read_i16(buyer + club::REPUTATION);
                let rv = read_i16(seller + club::REPUTATION);
                if rc > rv {
                    let dif = rc.wrapping_sub(rv) as f64;
                    let k = if rc > 0x1e46 && read_i16(sheet + player_sheet::CURRENT_REPUTATION) < 0x128e {
                        core::ptr::read_unaligned(0x96a730 as *const f64)
                    } else if rc > 0x109a {
                        core::ptr::read_unaligned(0x967888 as *const f64)
                    } else {
                        core::ptr::read_unaligned(0x967878 as *const f64)
                    };
                    crate::comparison::branch("cpv_2 injury_word_0c do jogador");
                    let sum = match_pl_ftol(dif * k) as i16; // cpv2: the reputation rises
                    let r = (sheet + player_sheet::CURRENT_REPUTATION) as *mut i16;
                    core::ptr::write_unaligned(r, core::ptr::read_unaligned(r).wrapping_add(sum));
                    let f2 = read_usize(person + staff::PLAYER);
                    if read_i16(f2 + player_sheet::CURRENT_REPUTATION) > 0x2710 {
                        core::ptr::write_unaligned((f2 + player_sheet::CURRENT_REPUTATION) as *mut i16, 0x2710);
                    }
                }
            }
            news(this, person, buyer, seller, 4, o, 0, 0, price as u32);
        }
        return follow(this, o);
    }
    let swap = read_usize(o + offer::EXCHANGE_PLAYERS);
    if swap == 0 {
        show_error_at(0xa8d514, 0x101f);
        return follow(this, o);
    }
    transfer_manager_llist_rewind(swap);
    let mut x = list_next_item(swap);
    while x != 0 && read_usize(x) != read_usize(person) {
        x = list_next_item(swap);
    }
    if x == 0 {
        return follow(this, o);
    }
    crate::comparison::branch("cpv_2 jogador saiu da troca");
    let buyer = friendly_club_by_id(read_usize(o + offer::BUYER_CLUB_ID));
    let new_price = value(read_usize(G_CONTRACT_MANAGER), person, buyer, 0, -1, 0).wrapping_add(read_usize(o + offer::FEE) as i32);
    let seller = offer_contract_club(transfer_manager_main_contract_stored(staff_records_staff_by_id(read_usize(o + offer::TARGET_STAFF_ID))), false);
    let buyer = friendly_club_by_id(read_usize(o + offer::BUYER_CLUB_ID));
    remove(read_usize(o + offer::EXCHANGE_PLAYERS), x);
    let l = read_usize(o + offer::EXCHANGE_PLAYERS);
    if core::ptr::read_unaligned(l as *const u16) == 0 {
        if l != 0 {
            destroy(l);
            free(l);
        }
        core::ptr::write_unaligned((o + offer::EXCHANGE_PLAYERS) as *mut usize, 0);
    }
    let m = read_usize(this + transfer_manager_fields::PERSON_FLAGS).wrapping_add(read_usize(person) * 8 + 2);
    write_u8(m, read_u8(m).wrapping_sub(1));
    if silencioso == 0 {
        news(this, person, seller, buyer, 4, o, 0, 0, new_price as u32);
    }
    core::ptr::write_unaligned((o + offer::FEE) as *mut i32, new_price);
    if seller != 0 && human_club(seller, 0, 1, 0, 0) == 0 {
        reply_of_seller(this, slot(), 1);
        if is_offer_resolved(o) {
            return;
        }
    }
    if human_club(buyer, 0, 1, 0, 0) != 0 {
        return follow(this, o);
    }
    reply_of_buyer(read_usize(G_TRANSFER_MANAGER), slot(), 1);
    if is_offer_resolved(o) {
        return;
    }
    follow(this, o);
}

/// Holds offers at window close: completes the approved offers and puts the nation's offers on hold: news if a human is
/// involved, otherwise withdrawn or rejected; for the computer, status 3 and back the next day.
///
/// Ret 4: `TRANSFER_MANAGER::hold_offers_at_window_close(nation)`: the nation's rules (of the 28 in `[this+0x8ac]`, the
/// one whose +0xa equals nation +0x53 of the national side 0x53b3d0; otherwise none). For each offer in vector `[this]`
/// (count +4): status 0xe completes (0x8c34e0 at the deadline). Buyer from the nation: not resolved, a player with a
/// sheet (+0x61), without flag +0x30 & 0xf, outside status 0x13 and accepted by the rules (virtual +0xc with the buyer
/// and today; answer 0 = hold): on hold; the waiting news (0x8e1150) if the buyer or the seller is human, otherwise
/// withdrawn (0x8bfcb0); computer buyer (0x600a50): status 3 and rescheduled in 1 day. Otherwise, with players in the
/// exchange and a seller from the nation, by the same tests with the seller in the rules: on hold; the news, or refused
/// (0x8bf680); and the same rescheduling.
/// Original: `hold_offers_at_window_close` (0x8c4e10, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8c4e10 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::hold_offers_at_window_close(nação)`: as
/// regras da nação (das 28 em [this+0x8ac], a de +0xa igual à nação +0x53 da seleção 0x53b3d0; senão nenhuma). Para
/// cada oferta do vetor [this] (conta +4): status 0xe completa (0x8c34e0 no prazo). Comprador da nação: não resolvida,
/// jogador com ficha (+0x61), sem a marca +0x30 & 0xf, fora do status 0x13 e aceita pelas regras (virtual +0xc com o
/// comprador e hoje; resposta 0 = segura): em espera; a notícia da espera (0x8e1150) se o comprador ou o vendedor é
/// humano, senão retirada (0x8bfcb0); comprador do computador (0x600a50): status 3 e reagendada em 1 dia. Senão, com
/// jogadores na troca e vendedor da nação, pelos mesmos testes com o vendedor nas regras: em espera; a notícia, ou
/// recusada (0x8bf680); e o mesmo reagendamento.
pub(crate) unsafe extern "thiscall" fn hold_offers_at_window_close(this: usize, nation: usize) {
    let national_team: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_TEAM);
    let completar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_TRY_COMPLETE_OFFER);
    let human: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_NEWS_CLUB_HAS_HUMAN_MANAGER);
    let noticia_da_espera: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_POST_OFFER_ON_HOLD_NEWS);
    let retirar: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_WITHDRAW_TRANSFER_OFFER);
    let recusar: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_REJECT_TRANSFER_OFFER);
    let human_club: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let schedule: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_SCHEDULE_OFFER);
    let of_nation = read_usize(national_team(nation) + 0x53);
    let mut rules = 0usize;
    for k in 0..0x1c {
        let r = read_usize(this + transfer_manager_fields::TRANSFER_RULES + 4 * k);
        if read_usize(r + transfer_rules::NATION_ID) == of_nation {
            rules = r;
            break;
        }
    }
    let accepted = |o: usize, club: usize| -> bool {
        if rules == 0 {
            return true;
        }
        let segurar: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) -> u32 = core::mem::transmute(read_usize(read_usize(rules) + 0xc));
        segurar(rules, o, club, G_CURRENT_DATE, 0) == 0
    };
    let mut i = 0usize;
    while (i as i32) < read_usize(this + transfer_manager_fields::OFFER_CAPACITY) as i32 {
        let slot = read_usize(this).wrapping_add(i * 4);
        let o = read_usize(slot);
        i += 1;
        if o == 0 {
            continue;
        }
        let st = read_u8(o + offer::STATUS);
        if st == 0xe {
            completar(this, o, 1);
            continue;
        }
        let buyer = friendly_club_by_id(read_usize(o + offer::BUYER_CLUB_ID));
        if read_usize(buyer + club::NATION) == nation
            && !is_offer_resolved(o)
            && read_usize(staff_records_staff_by_id(read_usize(o + offer::TARGET_STAFF_ID)) + 0x61) != 0
            && read_u8(o + offer::COMPLETION_TIMING) & 0xf == 0
            && st != 0x13
            && accepted(o, buyer)
        {
            put_offer_on_hold(o);
            let humano_vende = || {
                let c = transfer_manager_main_contract_stored(staff_records_staff_by_id(read_usize(o + offer::TARGET_STAFF_ID)));
                if c == 0 || read_u8(c + contract::CONTRACT_TYPE) & 0x3f == 5 || friendly_club_by_id(read_usize(c + contract::CLUB)) == 0 {
                    return false;
                }
                let v = offer_contract_club(transfer_manager_main_contract_stored(staff_records_staff_by_id(read_usize(o + offer::TARGET_STAFF_ID))), true);
                human(read_usize(G_NEWS), v) != 0
            };
            if human(read_usize(G_NEWS), friendly_club_by_id(read_usize(o + offer::BUYER_CLUB_ID))) != 0 || humano_vende() {
                noticia_da_espera(this, o, nation);
            } else {
                crate::comparison::branch("cpv_3 retirada na janela");
                // the slot reread from [this] as in the original
                retirar(this, read_usize(this).wrapping_add((i - 1) * 4), 0, 0); // cpv3: o comprador retira
            }
            if human_club(friendly_club_by_id(read_usize(o + offer::BUYER_CLUB_ID)), 0, 1, 0, 0) == 0 {
                crate::comparison::branch("cpv_3 reagendada na janela");
                write_u8(o + offer::STATUS, 3); // cpv3: status 3
                schedule(this, o, 1, 0); // cpv3: de volta amanhã
            }
            continue;
        }
        // the others: swap offer whose seller is from the nation
        if read_usize(o + offer::EXCHANGE_PLAYERS) == 0 {
            continue;
        }
        let c = transfer_manager_main_contract_stored(staff_records_staff_by_id(read_usize(o + offer::TARGET_STAFF_ID)));
        if c == 0 || friendly_club_by_id(read_usize(c + contract::CLUB)) == 0 {
            continue;
        }
        let seller = offer_contract_club(transfer_manager_main_contract_stored(staff_records_staff_by_id(read_usize(o + offer::TARGET_STAFF_ID))), false);
        if read_usize(seller + club::NATION) != nation || is_offer_resolved(o) || read_u8(o + offer::STATUS) == 0x13 {
            continue;
        }
        if rules != 0 {
            let owner = offer_contract_club(transfer_manager_main_contract_stored(staff_records_staff_by_id(read_usize(o + offer::TARGET_STAFF_ID))), false);
            if !accepted(o, owner) {
                continue;
            }
        }
        put_offer_on_hold(o);
        let humano_vende = || {
            let c = transfer_manager_main_contract_stored(staff_records_staff_by_id(read_usize(o + offer::TARGET_STAFF_ID)));
            if c == 0 || friendly_club_by_id(read_usize(c + contract::CLUB)) == 0 {
                return false;
            }
            let v = offer_contract_club(transfer_manager_main_contract_stored(staff_records_staff_by_id(read_usize(o + offer::TARGET_STAFF_ID))), false);
            human(read_usize(G_NEWS), v) != 0
        };
        if human(read_usize(G_NEWS), friendly_club_by_id(read_usize(o + offer::BUYER_CLUB_ID))) != 0 || humano_vende() {
            noticia_da_espera(this, o, nation);
        } else {
            crate::comparison::branch("cpv_3 recusada na janela");
            recusar(this, read_usize(this).wrapping_add((i - 1) * 4), 0, 0); // cpv3: o vendedor recusa
        }
        if human_club(friendly_club_by_id(read_usize(o + offer::BUYER_CLUB_ID)), 0, 1, 0, 0) == 0 {
            write_u8(o + offer::STATUS, 3);
            schedule(this, o, 1, 0);
        }
    }
}

/// Schedules the offer: gives it today's date + days and the day slice (0 to 2, every 3 slices one more day) and puts
/// it in the bucket (days since the base date + start, modulo 50, × 3 + slice) of the +0x18 lists.
///
/// Ret 0xc: `TRANSFER_MANAGER::schedule_offer(offer, days, slices)`: without a base date (year +0x84f zeroed), it
/// becomes today (0x417c10 of `[0xae2c90]` into +0x84d) and slice +0x855 today's (`[0xae2c88]`); `days` ≥ 50: the
/// "Error" box (line 0x18b9) and 10 days. The offer gets the date (+0x12) and the slice (+0x1a) of the plan
/// (`vacancy_schedule_plan`) and its +0x20 (4 bytes) goes into the bucket (0x68d0d0, which allocates); returns what
/// 0x68d0d0 returns. No SEH, no random draw.
/// Original: `TRANSFER_MANAGER::schedule_offer` (0x8c6220, transfer_manager.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x8c6220 } (thiscall, ret 0xc, transfer_manager.cpp): `TRANSFER_MANAGER::schedule_offer(oferta, dias, fatias)`:
/// sem data base (ano +0x84f zerado), ela vira hoje (0x417c10 de [0xae2c90] em +0x84d) e a fatia +0x855 a de hoje
/// ([0xae2c88]); `dias` ≥ 50: a caixa "Error" (linha 0x18b9) e 10 dias. A oferta ganha a data (+0x12) e a fatia (+0x1a)
/// do plano (`vacancy_schedule_plan`) e o +0x20 dela (4 bytes) entra no balde (0x68d0d0, que aloca); devolve o que a 0x68d0d0
/// devolve. Sem SEH, sem sorteio.
pub(crate) unsafe extern "thiscall" fn schedule_offer(this: usize, offer: usize, days: u32, fatias: u32) -> u32 {
    crate::comparison::branch("cpn_5 agendar oferta");
    if read_i16(this + transfer_manager_fields::CURRENT_YEAR) == 0 {
        let copy: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
        let mut tmp = [0u8; 8];
        copy(this + transfer_manager_fields::CURRENT_DATE, tmp.as_mut_ptr(), G_CURRENT_DATE);
        *((this + transfer_manager_fields::CURRENT_SLICE) as *mut u8) = read_u8(G_CURRENT_DAY_SLICE);
    }
    if days as u8 as i8 >= 0x32 {
        show_error_at(0xa8d514, 0x18b9);
    }
    // simplification: the dates are built in the plan on copies (0x54c5b0, 0x54cdb0 and 0x417c10 only read and copy) and written
    // at once; the original writes the date and the slice on each pass of the slice, with the same end
    let (data, fatia, balde) = vacancy_schedule_plan(this, days as u8, fatias as u8);
    core::ptr::copy_nonoverlapping(data.as_ptr(), (offer + offer::DUE_DATE) as *mut u8, 8);
    *((offer + offer::DUE_SLICE) as *mut i8) = fatia;
    let append: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_LLIST_ADD_TAIL);
    append(balde, offer + offer::SLOT_INDEX, 4)
}

/// Returns, of the offers for the player from the club (or flagged) in state 2, the highest (0x8e7a70); 0 when none.
///
/// Ret 8: `TRANSFER_MANAGER::find_highest_offer_for_player(player, club)`: with the player's offers flag (byte +1 of
/// the 8 of `[+0x86e]`), the first record of vector `[this]` (`[+4]` of them) for the player (+0xc): without a list
/// (+0), itself if it qualifies; with a list, of the offers (indices into the vector) that qualify, the one with the
/// highest value (0x8e7a70 with 0, 0, 0, 1; the first on a tie). Otherwise 0.
/// Original: `TRANSFER_MANAGER::find_highest_offer_for_player` (0x8cc930, transfer_manager.cpp, thiscall, ret 8).
///
/// [pt] { 0x8cc930 } (thiscall, ret 8, transfer_manager.cpp): `TRANSFER_MANAGER::find_highest_offer_for_player(jogador,
/// clube)`: com a marca de ofertas do jogador (byte +1 dos 8 de [+0x86e]), o primeiro registro do vetor [this] ([+4]
/// deles) do jogador (+0xc): sem lista (+0), ele mesmo se serve; com lista, das ofertas (índices no vetor) que servem,
/// a de maior valor (0x8e7a70 com 0, 0, 0, 1; a primeira no empate). Senão 0.
pub(crate) unsafe extern "thiscall" fn find_highest_offer_for_player(this: usize, player: *const u32, club: *const u32) -> usize {
    let id = *player;
    if *((read_usize(this + transfer_manager_fields::PERSON_FLAGS).wrapping_add((id as usize).wrapping_mul(8)) + 1) as *const u8) == 0 {
        return 0;
    }
    let mut t = 0;
    let mut k = 0i32;
    while k < read_usize(this + transfer_manager_fields::OFFER_CAPACITY) as i32 {
        let x = read_usize(read_usize(this).wrapping_add(k as usize * 4));
        if x != 0 && read_usize(x + offer::TARGET_STAFF_ID) as u32 == id {
            t = x;
            break;
        }
        k += 1;
    }
    if t == 0 {
        return 0;
    }
    let l = read_usize(t);
    if l == 0 {
        return if is_offer_eligible(t, *club) { t } else { 0 };
    }
    core::ptr::write_unaligned((l + llist::CURSOR) as *mut usize, read_usize(l + llist::HEAD));
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let value: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> i32 = core::mem::transmute(FN_TRANSFER_OFFER_GET_TOTAL_VALUE);
    let (mut melhor, mut maior) = (0usize, -1i32);
    let mut e = next(l);
    while e != 0 {
        let o = read_usize(read_usize(this).wrapping_add(read_usize(e).wrapping_mul(4)));
        if is_offer_eligible(o, *club) {
            let v = value(o, 0, 0, 0, 1);
            if v > maior { // the first one on a tie
                melhor = o;
                maior = v;
            }
        }
        e = next(l);
    }
    melhor
}

/// With the person's flag 4, binary-searches the international appearance fee clause; when not found, clears the flag.
///
/// Ret 4: `TRANSFER_MANAGER::find_international_appearance_money(person)`, from the person screens (0x884ea2): without
/// the person's flag 4 (`[this+0x86e]` + id × 8), 0. The binary search (0x946464: cdecl(key, vector `[this+0x898]`,
/// count `[this+0x8a0]`, 0xd, 0x8b4e00)) by the key with the person's id at +0, the only field the comparison reads;
/// found, the record; otherwise flag 4 is cleared and 0.
/// Original: `TRANSFER_MANAGER::find_international_appearance_money` (0x8d0f00, transfer_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x8d0f00 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::find_international_appearance_money(pessoa)`,
/// das telas da pessoa (0x884ea2): sem a marca 4 da pessoa ([this+0x86e] + id × 8), 0. A busca binária (0x946464:
/// cdecl(chave, vetor [this+0x898], conta [this+0x8a0], 0xd, 0x8b4e00)) pela chave com o id da pessoa em +0, o único
/// campo que a comparação lê; achada, o registro; senão a marca 4 sai e 0.
pub(crate) unsafe extern "thiscall" fn find_international_appearance_money(this: usize, p: usize) -> usize {
    let m = (read_usize(this + transfer_manager_fields::PERSON_FLAGS).wrapping_add(read_usize(p).wrapping_mul(8))) as *mut u8;
    if *m & 4 == 0 {
        return 0;
    }
    let key = [read_usize(p) as u32, 0, 0, 0];
    let bsearch: unsafe extern "C" fn(*const u32, usize, usize, u32, usize) -> usize = core::mem::transmute(FN_CRT_BSEARCH);
    let r = bsearch(key.as_ptr(), read_usize(this + transfer_manager_fields::INTERNATIONAL_APPEARANCE_MONEY), read_usize(this + transfer_manager_fields::INTERNATIONAL_APPEARANCE_MONEY_COUNT), 0xd, 0x8b4e00);
    if r != 0 {
        return r;
    }
    *m = (*m).wrapping_sub(4);
    0
}

/// Pays appearance money: finds the appearance money record (club or national side) and, when the appearances match,
/// posts the news, adds to the totals and the former club receives the amount the current one pays; returns 1 when
/// paid.
///
/// Ret 0x10: `TRANSFER_MANAGER::pay_appearance_money(player, matches, type, arg)`: with a contract (stored in
/// `[0xae1080]`) and the person's bonus record (the whole `type` is passed, as the original pushes it) whose matches
/// (+8, signed) are `matches` (> 0): gives the news (0x8db8a0), adds the amount (+4) to the transfer totals (0x448ad0
/// on `[0xad9dac]`, with the current and the former club), the former club (+9) receives (0x599800) and the current one
/// pays (0x599890), in signed 64 bits. Returns 1 if it paid.
/// Original: `TRANSFER_MANAGER::pay_appearance_money` (0x8cb340, transfer_manager.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x8cb340 } (thiscall, ret 0x10, transfer_manager.cpp): `TRANSFER_MANAGER::pay_appearance_money(jogador, jogos,
/// tipo, arg)`: com contrato (guardado em [0xae1080]) e o registro do prêmio da pessoa (o `tipo` vai inteiro, como a
/// original o empurra) cujos jogos (+8, com sinal) são os `jogos` (> 0): dá a notícia (0x8db8a0), soma o valor (+4)
/// aos totais de transferência (0x448ad0 em [0xad9dac], com o clube atual e o antigo), o clube antigo (+9) recebe
/// (0x599800) e o atual paga (0x599890), em 64 bits com sinal. Devolve 1 se pagou.
pub(crate) unsafe extern "thiscall" fn pay_appearance_money(this: usize, player: usize, games: i32, kind: u32, arg: u32) -> u32 {
    let c = shortlist_manager_get_contract_and_store(player);
    if c == 0 || friendly_club_by_id(read_usize(c + contract::CLUB)) == 0 {
        return 0;
    }
    let r = appearance_bonus_record(this, player, kind);
    if r == 0 {
        return 0;
    }
    // the original: different matches only continue with −1, which falls into the next test (≤ 0)
    if read_u8(r + appearance_clause::GAMES) as i8 as i32 != games || games <= 0 {
        return 0;
    }
    let news: unsafe extern "thiscall" fn(usize, usize, i32, u32, u32) = core::mem::transmute(FN_TRANSFER_MANAGER_POST_APPEARANCE_MONEY_NEWS);
    let finances: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FINANCETABLE_GET_CLUB_FINANCE);
    let add: unsafe extern "thiscall" fn(usize, i32, usize, usize) -> u32 = core::mem::transmute(FN_CLUB_RECORDS_ADD_TRANSFER_TOTALS);
    let receber: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(FN_FINANCE_ADD_PLAYERS_SOLD);
    let pay: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(FN_FINANCE_PAY_PLAYERS_BOUGHT);
    news(this, r, games, kind, arg);
    let f_old = finances(read_usize(G_FINANCE_TABLE), friendly_club_by_id(read_usize(r + appearance_clause::CLUB_ID)));
    let c = shortlist_manager_get_contract_and_store(player);
    let f_current = finances(read_usize(G_FINANCE_TABLE), if c != 0 { friendly_club_by_id(read_usize(c + contract::CLUB)) } else { 0 });
    let c = shortlist_manager_get_contract_and_store(player);
    let current = if c != 0 { friendly_club_by_id(read_usize(c + contract::CLUB)) } else { 0 };
    let value = core::ptr::read_unaligned((r + appearance_clause::AMOUNT) as *const i32);
    add(read_usize(G_CLUB_RECORDS), value, current, friendly_club_by_id(read_usize(r + appearance_clause::CLUB_ID)));
    crate::comparison::branch("cpp_6 premio por jogos");
    receber(f_old, value as u32, (value >> 31) as u32); // o clube antigo recebe
    pay(f_current, value as u32, (value >> 31) as u32);
    1
}

/// the period from the club nation's rules and, with a type 2 contract, moves the end back month by month until it no
/// longer passes the contract end (the start follows); without a contract, expired, too far or before this year: the
/// default year 1972; turned off (DESLIGADAS) from 2026-10-29 to 2026-10-31, when the test criterion changed and the list was emptied: with the detour the career in Rust mode left the reference
/// be54a138 (bisection of batch 251's 39 routines; with 0x77bbe0, the news constructor, the game later crashed in
/// 0x7c6650 on a broken index); the cause (stack residue or a difference in the copy) is not isolated yet; the copy
/// stays in the code and does not run
/// Original: `TRANSFER_MANAGER::get_loan_period` (0x8d0590, transfer_manager.cpp, thiscall, ret 0x1c).
///
/// [pt] { 0x8d0590 } (thiscall, ret 0x1c, transfer_manager.cpp): `TRANSFER_MANAGER::get_loan_period(clube, a2, jogador,
/// início_mês, início_ano, fim_mês, fim_ano)`: as regras da nação do clube (+0x53; das 28 em +0x8ac, a primeira sem
/// achar) dão o período (método +0x28). Sem contrato de tipo 2 (0x4cf5a0), com o contrato já vencido hoje ou o fim mais
/// de 5 anos à frente: o fim vira o ano da data padrão (1972). Senão, enquanto a data (dia do mês de hoje, fim_mês,
/// fim_ano; 0x549f70) passar do fim do contrato, recua um mês (o início acompanha se ficar depois do fim); fim antes do
/// ano de hoje: o ano padrão.
pub(crate) unsafe extern "thiscall" fn transfer_manager_get_loan_period(this: usize, club: usize, a2: u32, player: usize, start_month: *mut i8, start_year: *mut u16, month: *mut i8, year: *mut u16) {
    let nation = read_usize(club + club::NATION);
    let mut rule = read_usize(this + transfer_manager_fields::TRANSFER_RULES);
    for k in 0..0x1cusize {
        let r = read_usize(this + transfer_manager_fields::TRANSFER_RULES + k * 4);
        if read_usize(r + transfer_rules::NATION_ID) == nation {
            if r != 0 {
                rule = r;
            }
            break;
        }
    }
    let period: unsafe extern "thiscall" fn(usize, usize, u32, *mut i8, *mut u16, *mut i8, *mut u16, usize) = core::mem::transmute(read_usize(read_usize(rule) + 0x28));
    period(rule, club, a2, start_month, start_year, month, year, player);
    let day_of_month: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(FN_DATE_GET_DAY_OF_MONTH);
    let day = day_of_month(G_CURRENT_DATE);
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = contract_of(read_usize(G_CONTRACT_MANAGER), player);
    if c == 0 || read_u8(c + contract::CONTRACT_TYPE) & 0x3f != 2 || is_after_contract_end(read_i16(G_CURRENT_DATE), read_i16(G_CURRENT_DATE_YEAR) as u16, c) {
        *year = default_date_year();
        return;
    }
    if *year as i32 > read_i16(G_CURRENT_DATE_YEAR) as u16 as i32 + 5 {
        *year = default_date_year();
        return;
    }
    // simplification: the original pushes the month and the year in registers with garbage above; 0x549f70 reads the byte and the word
    let data_de: unsafe extern "thiscall" fn(usize, i32, i32, u32, i32) -> usize = core::mem::transmute(FN_DATE_DATE_549F70);
    loop {
        let mut data = [0u8; 8];
        let d = data_de(data.as_mut_ptr() as usize, day, *month as i32, *year as u32, -1);
        if !is_after_contract_end(read_i16(d), read_i16(d + cm_date::YEAR) as u16, c) {
            return;
        }
        crate::comparison::branch("cpo_9 recua o mes");
        let m = (*month).wrapping_sub(1);
        *month = m;
        if m < 0 {
            *year = (*year).wrapping_sub(1);
            *month = 11;
        }
        if *start_year > *year || (*start_year == *year && *start_month > *month) {
            *start_month = *month;
            *start_year = *year;
        }
        if *year < read_i16(G_CURRENT_DATE_YEAR) as u16 {
            *year = default_date_year();
            return;
        }
    }
}

/// Updates the transfer window news: checks each transfer window in the previous, current and next months: on the day,
/// the opening or closing notice; the day after the opening, the notice and the nation's waiting offers; a week before
/// the opening, the notice.
///
/// Ret 4; SEH frame with no destructor on the normal path (the local dates):
/// `TRANSFER_MANAGER::update_transfer_window_news(today)`, from the transfers' day. The previous, current and next
/// months (0x54cfb0: mov ecx, today -> thiscall, ret 0; the month before January is December of year −1, the one after
/// December is January of year +1), today + 1 week (0x54c770: push 1; push out; mov ecx, today -> thiscall, ret 8) and
/// yesterday (0x54c8e0: likewise). For each window of vector `[this+0x8a4]` (count +0x8a8, reread): in the first of the
/// three months equal to its own (+4), its date in that year (0x40a7d0: push year; push out; mov ecx, window ->
/// thiscall, ret 8; copied through 0x417c10). Today: virtual +0x2c of the rules of its nation (`[this+0x8ac + 4·(+0)]`:
/// push the dword of the date's year; push code 1, or 3 if the window closes (+5), with the high bytes of the same
/// dword; push window -> thiscall, ret 0xc); yesterday and a window that opens: the virtual with 0 and the waiting
/// offers of the rules' nation (+0xa; 0x8c4e10: push nation; mov ecx, esi -> thiscall, ret 4); a week from now and a
/// window that opens: the virtual with 2.
/// Original: `TRANSFER_MANAGER::update_transfer_window_news` (0x8c6bc0, transfer_manager.cpp, thiscall, ret 4).
///
/// [pt] { 0x8c6bc0 } (thiscall, ret 4, transfer_manager.cpp, quadro SEH sem destrutor no caminho normal: as datas locais):
/// `TRANSFER_MANAGER::update_transfer_window_news(hoje)`, do dia das transferências. Os meses anterior, atual e
/// seguinte (0x54cfb0: mov ecx, hoje -> thiscall, ret 0; o anterior de janeiro é dezembro do ano −1, o seguinte de
/// dezembro é janeiro do ano +1), hoje + 1 semana (0x54c770: push 1; push saída; mov ecx, hoje -> thiscall, ret 8) e
/// ontem (0x54c8e0: idem). Para cada janela do vetor [this+0x8a4] (conta +0x8a8, relida): no primeiro dos três meses
/// igual ao dela (+4), a data dela naquele ano (0x40a7d0: push ano; push saída; mov ecx, janela -> thiscall, ret 8;
/// copiada pela 0x417c10). Hoje: o virtual +0x2c das regras da nação dela ([this+0x8ac + 4·(+0)]: push o dword do ano
/// da data; push o código 1, ou 3 se a janela fecha (+5), com os bytes altos do mesmo dword; push janela -> thiscall,
/// ret 0xc); ontem e janela que abre: o virtual com 0 e as ofertas em espera da nação das regras (+0xa; 0x8c4e10: push
/// nação; mov ecx, esi -> thiscall, ret 4); daqui a uma semana e janela que abre: o virtual com 2.
pub(crate) unsafe extern "thiscall" fn update_transfer_window_news(this: usize, today: usize) {
    let month: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(FN_DATE_GET_MONTH);
    let default: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    let more_weeks: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_DATE_ADD_WEEKS);
    let minus: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_DATE_SUB_DAYS);
    let data_of_window: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_TRANSFER_WINDOW_GET_DATE);
    let copy: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let espera: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_HOLD_OFFERS_AT_WINDOW_CLOSE);
    let mut years = [0i16; 3];
    let mut d = [0u8; 8];
    let pd = d.as_mut_ptr() as usize;
    default(pd);
    let mut meses = [0u8; 3];
    meses[0] = month(today).wrapping_sub(1);
    if (meses[0] as i8) < 0 {
        meses[0] = 0xb;
        years[0] = -1;
    }
    meses[1] = month(today);
    meses[2] = month(today).wrapping_add(1);
    if (meses[2] as i8) > 0xb {
        meses[2] = 0;
        years[2] = 1;
    }
    let (mut semana, mut ontem) = ([0u8; 8], [0u8; 8]);
    let (ps, po) = (semana.as_mut_ptr() as usize, ontem.as_mut_ptr() as usize);
    more_weeks(today, ps, 1);
    minus(today, po, 1);
    let mut v = read_u32_as_usize(this + transfer_manager_fields::TRANSFER_WINDOWS);
    let mut i: i32 = 0;
    while i < read_u32_as_usize(this + transfer_manager_fields::TRANSFER_WINDOW_COUNT) as i32 {
        for j in 0..3 {
            let w = read_u32_as_usize(v);
            if read_u8(w + transfer_window::MONTH) != meses[j] {
                continue;
            }
            let year = (years[j] as u16).wrapping_add(read_i16(today + cm_date::YEAR) as u16);
            let (mut t, mut c) = ([0u8; 8], [0u8; 8]);
            let r = data_of_window(w, t.as_mut_ptr() as usize, year as u32);
            copy(pd, c.as_mut_ptr() as usize, r);
            let (hd, ha) = (read_i16(today), read_i16(today + cm_date::YEAR));
            let (dd, da, dw) = (read_i16(pd), read_i16(pd + cm_date::YEAR), read_u32_as_usize(pd + cm_date::YEAR) as u32);
            let rules = |w: usize| read_u32_as_usize(this + transfer_manager_fields::TRANSFER_RULES + (read_u8(w) as i8 as isize * 4) as usize);
            let aviso = |w: usize, code: u32| {
                let rg = rules(w);
                let f: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(rg) + 0x2c));
                f(rg, w, code, dw);
            };
            if ha == da && hd == dd {
                let code = if read_u8(w + transfer_window::CLOSES) != 0 { 3 } else { 1 };
                aviso(w, (dw & 0xffff_ff00) | code); // cqc5: the notice of the window day
                break;
            }
            if read_i16(po + cm_date::YEAR) == da && read_i16(po) == dd && read_u8(w + transfer_window::CLOSES) == 0 {
                aviso(w, 0);
                let nation = read_u32_as_usize(rules(read_u32_as_usize(v)) + 0xa);
                if nation != 0 {
                    crate::comparison::branch("cqc_5 janela aberta");
                    espera(this, nation); // cqc5: the pending offers of the window that opened
                }
                break;
            }
            if read_i16(ps + cm_date::YEAR) == da && read_i16(ps) == dd && read_u8(w + transfer_window::CLOSES) == 0 {
                aviso(w, 2);
                break;
            }
        }
        i += 1;
        v += 4;
    }
}

/// Marks that the transfer offer had contract terms offered to the player.
///
/// Ret 4: marks that offer `k` (not a loan, +0x1f ≠ 1) had the terms offered to the player: state +0x2c becomes 8 if it
/// was 6, otherwise 7. Index outside the list (`[+4]` offers): the original routine (error box).
/// Original: `TRANSFER_MANAGER::note_terms_offered_to_main_player` (0x8c1f80, transfer_manager.cpp, thiscall).
///
/// [pt] { 0x8c1f80 } (thiscall, ret 4, transfer_manager.cpp): marca que a oferta `k` (fora de empréstimo,
/// +0x1f ≠ 1) teve os termos oferecidos ao jogador: o estado +0x2c vira 8 se era 6, senão 7. Índice
/// fora da lista ([+4] ofertas): a original (caixa de erro).
pub(crate) unsafe extern "thiscall" fn note_terms_offered_to_main_player(manager: usize, k: i32) {
    if k < 0 || k >= *((manager + transfer_manager_fields::OFFER_CAPACITY) as *const i32) {
        return original_fn::<unsafe extern "thiscall" fn(usize, i32)>(IDX_NOTE_TERMS_OFFERED_TO_MAIN_PLAYER)(manager, k);
    }
    let offer = *((*(manager as *const usize) as isize + k as isize * 4) as *const usize);
    if offer != 0 && *((offer + offer::OFFER_TYPE) as *const u8) != 1 {
        let state = (offer + offer::STATUS) as *mut u8;
        *state = (*state == 6) as u8 + 7;
    }
}

