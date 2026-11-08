use super::*;

// The comparers of the routines in ../contract_manager.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../contract_manager.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `sign_contract` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x4cf720: só a original (inclui e tira a pessoa dos clubes, realoca o vetor dos contratos, entrega
/// notícias, sorteia a segunda nacionalidade, recalcula a folha).
pub(crate) unsafe extern "thiscall" fn comparar_sign_contract(this: usize, contract: usize, job: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_SIGN_CONTRACT)(this, contract, job) // simplification: original only
}

/// Comparer for `contract_manager_offer_contract` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x4cda70: com `saida`, lado a lado (a saída, [0xae1080] e o estado do sorteador; a cópia sem contar os
/// sorteios); sem `saida` (pode propor o contrato, que aloca), só a original.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn comparar_contract_manager_offer_contract(this: usize, person: usize, club: usize, out: usize, slot: i32, factor: f64, mode: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, i32, f64, u32) -> u32>(IDX_CONTRACT_MANAGER_OFFER_CONTRACT);
    if out == 0 {
        return o(this, person, club, out, slot, factor, mode); // simplification: original only
    }
    let est = crate::rng::STATE_REGIONS;
    let regions = [(out, 0x50usize), (G_CONTRACT_LOOKUP_RESULT, 4usize), est[0], est[1]];
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&regions,
        || o(this, person, club, out, slot, factor, mode),
        || crate::rng::without_counting(|| contract_manager_offer_contract(this, person, club, out, slot, factor, mode)));
    // simplification: the original’s local STAFF_CONTRACT has no value at +0x38..+0x39 (of dword +0x36), +0x44 and in the nibble
    // low part of +0x4f (neither 0x85e410 nor 0x85fc50 writes them: stack leftover, copied to the output); the copy leaves 0
    // (built in zeroed memory). The comparison ignores these bits.
    let sem_sobra = |m: &crate::comparison::MemorySnapshot| {
        let mut v = m.0[0].1.clone();
        v[0x38] = 0;
        v[0x39] = 0;
        v[0x44] = 0;
        v[0x4f] &= 0xf0;
        v
    };
    let same = same || (sem_sobra(&mo) == sem_sobra(&mr) && mo.0[1..] == mr.0[1..]);
    SCOREBOARD.record(same && r == n, || format!(
        "contract_manager_offer_contract(pessoa {:#x}, clube {:#x}, vaga {}, fator {}, modo {}): original {} rust {}, memória {:x?} x {:x?}",
        person, club, slot, factor, mode, r, n, mo, mr));
    r
}

/// Comparer for `update_after_match` (comparison mode: see the header of motor.rs).
///
/// Morale (`0x4cb0a0`, `0x4cb250`: complaints, transfer request), development (`0x6d26b0`) and the squad change
/// (`0x4cbd20`) make random draws and allocate (they cannot run twice), so only the original routine runs. It checks
/// what the routine guarantees before any random draw: the bonuses each club pays (`+0x138` of the finances adds up to
/// the total predicted by the same rules as the copy, computed beforehand). The copy is proven by the branches and by
/// the mutations in the career.
///
/// [pt] Comparador: a moral (0x4cb0a0, 0x4cb250: queixas, pedido de transferência), a evolução (0x6d26b0) e a troca de
/// elenco (0x4cbd20) sorteiam e alocam (não podem rodar duas vezes): só a original roda. Confere o que a rotina garante
/// antes de qualquer sorteio: os bônus que cada clube paga (+0x138 das finanças soma o total previsto pelas mesmas
/// regras da cópia, calculado antes). A cópia é provada pelos ramos e pelas mutações na carreira.
pub(crate) unsafe extern "thiscall" fn comparar_update_after_match(this: usize, game: usize, summary: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_UPDATE_AFTER_MATCH);
    if read_i8(game + 0x40) == 0 || read_i8(game + 0x41) == 0 {
        return o(this, game, summary);
    }
    let partner: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
    let national_team: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let with_club: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let finances: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FINANCETABLE_GET_CLUB_FINANCE);
    let mut plan: [(usize, i32, i32); 2] = [(0, 0, 0); 2];
    let mut eh_a = 0u32;
    for (lado, pl) in plan.iter_mut().enumerate() {
        let club = match_clubs_by_side(game, lado).0;
        if (partner(club, &mut eh_a, 1) != 0 && eh_a == 0) || skip_side_by_reputation(game, lado) || national_team(club) != 0 {
            continue;
        }
        let fin = finances(read_usize(G_FINANCE_TABLE), club);
        let base = summary + lado * 0x625;
        let mut total = 0i32;
        for k in 0..20usize {
            let slot = base + 0x190 + k * 0x49;
            let Some(pessoa) = staff_of_summary_slot(slot) else { continue };
            let c = with_club(this, pessoa, club);
            if c != 0 {
                for v in match_bonuses_for_slot(game, c, slot, read_i16(base + 0x154 + 2 * k)).into_iter().flatten() {
                    total = total.wrapping_add(v);
                }
            }
        }
        if fin != 0 {
            *pl = (fin, read_i32(fin + 0x138), total);
        }
    }
    o(this, game, summary);
    for &(fin, antes, total) in &plan {
        if fin != 0 {
            let pago = read_i32(fin + 0x138).wrapping_sub(antes);
            SCOREBOARD.record(pago == total, || format!("update_after_match(jogo {:x}) bônus de fin {:x}: original {} previsto {}", game, fin, pago, total));
        }
    }
}

/// Comparer for `contract_manager_get_player_morale` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "stdcall" fn comparar_contract_manager_get_player_morale(person: usize) -> u32 {
    let r = original_fn::<unsafe extern "stdcall" fn(usize) -> u32>(IDX_CONTRACT_MANAGER_GET_PLAYER_MORALE)(person);
    let n = contract_manager_get_player_morale(person);
    SCOREBOARD.record(r == n, || format!("contract_manager_get_player_morale({:x}) original={:#x} rust={:#x}", person, r, n));
    r
}

/// Comparer for `contract_manager_write_morale_text` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a original escreve a palavra; a cópia escreve de novo (o texto se sobrescreve) e as duas têm de bater;
/// fica o texto da original.
pub(crate) unsafe extern "stdcall" fn comparar_contract_manager_write_morale_text(person: usize, out: usize, x: u32) {
    original_fn::<unsafe extern "stdcall" fn(usize, usize, u32)>(IDX_CONTRACT_MANAGER_WRITE_MORALE_TEXT)(person, out, x);
    let r = c_string_bytes(out);
    contract_manager_write_morale_text(person, out, x);
    let n = c_string_bytes(out);
    core::ptr::copy_nonoverlapping(r.as_ptr(), out as *mut u8, r.len());
    *((out + r.len()) as *mut u8) = 0;
    SCOREBOARD.record(r == n, || format!("contract_manager_write_morale_text({:x}) original={:?} rust={:?}", person, String::from_utf8_lossy(&r), String::from_utf8_lossy(&n)));
}

/// Comparer for `calc_player_value` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado (sem alocar) com o estado do sorteador (sem clube, sorteio(3)), o [0xae1080] e as estáticas
/// das datas da compensação; a cópia sem contar os sorteios. Quente (5,6 milhões de chamadas): sem contador.
pub(crate) unsafe extern "thiscall" fn comparar_calc_player_value(this: usize, person: usize, club: usize, mode: u32, ability: u32, x: u32) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) -> i32>(IDX_CALC_PLAYER_VALUE);
    let regions = player_value_regions();
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&regions, || o(this, person, club, mode, ability, x),
        || crate::rng::without_counting(|| calc_player_value(this, person, club, mode, ability, x)));
    SCOREBOARD.record(igual && r == n, || format!("calc_player_value(pessoa {:x}, clube {:x}, modo {}, hab {}) original={} rust={} {:x?} x {:x?}", person, club, mode as u8, ability as i16, r, n, mo, mr));
    r
}

/// Comparer for `contract_manager_write_morale_comparison_text` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a original escreve; a cópia escreve de novo (sobrescreve) e o texto e a diferença têm de bater; fica o
/// texto da original.
pub(crate) unsafe extern "stdcall" fn comparar_contract_manager_write_morale_comparison_text(a: usize, b: usize, out: usize, x: u32) -> u8 {
    let r = original_fn::<unsafe extern "stdcall" fn(usize, usize, usize, u32) -> u8>(IDX_CONTRACT_MANAGER_WRITE_MORALE_COMPARISON_TEXT)(a, b, out, x);
    let tr = c_string_bytes(out);
    let n = contract_manager_write_morale_comparison_text(a, b, out, x);
    let tn = c_string_bytes(out);
    core::ptr::copy_nonoverlapping(tr.as_ptr(), out as *mut u8, tr.len());
    *((out + tr.len()) as *mut u8) = 0;
    SCOREBOARD.record(r == n && tr == tn, || format!("contract_manager_write_morale_comparison_text original={} {:?} rust={} {:?}", r as i8, String::from_utf8_lossy(&tr), n as i8, String::from_utf8_lossy(&tn)));
    r
}

/// Comparer for `get_squad_flags_for_club` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_get_squad_flags_for_club(this: usize, person: usize, club: usize) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u8>(IDX_GET_SQUAD_FLAGS_FOR_CLUB);
    if club == 0 || person == 0 {
        return o(this, person, club);
    }
    let r = o(this, person, club);
    let n = get_squad_flags_for_club(this, person, club);
    SCOREBOARD.record(r == n, || format!("get_squad_flags_for_club({:x}, {:x}) original={} rust={}", person, club, r, n));
    r
}

/// Comparer for `post_wants_new_contract_news` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_post_wants_new_contract_news(this: usize, contract: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_POST_WANTS_NEW_CONTRACT_NEWS)(this, contract)
}

/// Comparer for `post_transfer_request_withdrawn_news` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_post_transfer_request_withdrawn_news(this: usize, contract: usize, reason: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_POST_TRANSFER_REQUEST_WITHDRAWN_NEWS)(this, contract, reason)
}

/// Comparer for `post_contract_expiry_news` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_post_contract_expiry_news(this: usize, contract: usize, period: u32, bosman: u32, warn: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32)>(IDX_POST_CONTRACT_EXPIRY_NEWS)(this, contract, period, bosman, warn)
}

/// Comparer for `post_bosman_departure_news` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_post_bosman_departure_news(this: usize, contract: usize, target: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_POST_BOSMAN_DEPARTURE_NEWS)(this, contract, target)
}

/// Comparer for `add_contract_offer` (comparison mode: see the header of motor.rs).
///
/// Offer already in the list (found beforehand, without moving the cursor): the copy as a dry run and the original
/// routine from the same state, comparing the offer (0x63 bytes), the counter and the list head (the cursor).
///
/// New offer (appended, which allocates): the copy as a dry run builds the record; the original appends it, and the new
/// node (the player's last one for the club) must have the same first 0x63 bytes, with the counter + 1.
///
/// [pt] Comparador. Oferta já na lista (achada antes, sem mexer no cursor): a cópia em ensaio e a original a partir do mesmo
/// estado, comparando a oferta (0x63 bytes), o contador e a cabeça da lista (o cursor). Oferta nova (acrescentada, aloca):
/// a cópia em ensaio monta o registro; a original acrescenta, e o nó novo (o último do jogador pelo clube) tem de ter os
/// mesmos 0x63 primeiros bytes, com o contador + 1.
pub(crate) unsafe extern "thiscall" fn comparar_add_contract_offer(this: usize, terms: usize, slot: i32, value: u32, mark: u32, a5: u32, decide: u32, acertado: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32, u32, u32, u32, u32)>(IDX_ADD_CONTRACT_OFFER);
    let id = read_usize(terms);
    let club = read_usize(terms + 4);
    let count = contract_offer_counter(terms);
    let l = read_usize(this + contract_manager_fields::CONTRACT_OFFERS);
    let item = if read_u8(count) != 0 && l != 0 { find_contract_offer(l, id, club) } else { 0 };
    let mut all = vec![(count, 1usize)];
    if item != 0 {
        all.push((item, 0x63));
        all.push((l, 0xe));
    }
    let n_cmp = all.len();
    all.extend(TRANSFER_MANAGER_DATE_STATICS);
    all.extend(crate::rng::STATE_REGIONS);
    let before = crate::comparison::MemorySnapshot::read(&all);
    CONTRACT_OFFER_DRY_RUN = true;
    crate::rng::pause_trace(|| crate::rng::without_counting(|| add_contract_offer(this, terms, slot, value, mark, a5, decide, acertado)));
    CONTRACT_OFFER_DRY_RUN = false;
    let mr = crate::comparison::MemorySnapshot::read(&all[..n_cmp]);
    let record = DRY_RUN_CONTRACT_OFFER_RECORD;
    before.write_back();
    o(this, terms, slot, value, mark, a5, decide, acertado);
    let mo = crate::comparison::MemorySnapshot::read(&all[..n_cmp]);
    let same = if item != 0 {
        mo == mr
    } else {
        // the new node: the player’s last one for the club in the list (created by the original if missing)
        let l = read_usize(this + contract_manager_fields::CONTRACT_OFFERS);
        let mut new = 0;
        if l != 0 {
            let mut no = read_usize(l + 2);
            let mut n = 0;
            while no != 0 && n < 100000 {
                let x = read_usize(no);
                if read_usize(x) == id && read_usize(x + 4) == club {
                    new = x;
                }
                no = read_usize(no + 6);
                n += 1;
            }
        }
        mo == mr && new != 0 && core::slice::from_raw_parts(new as *const u8, 0x63) == &record[..0x63]
    };
    SCOREBOARD.record(same, || format!("add_contract_offer(pessoa {:#x}, clube {:#x}, vaga {}, atualizada {}) original={:x?} rust={:x?}", id, club, slot, item != 0, mo, mr));
}

/// Comparer for `remove_contract_offer_for_transfer` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a original tira o item da lista (libera): só ela, conferindo o plano feito antes: com a oferta da pessoa
/// pela vaga na lista, o contador desce 1; sem ela, fica.
pub(crate) unsafe extern "thiscall" fn comparar_remove_contract_offer_for_transfer(this: usize, person: usize, slot: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32)>(IDX_REMOVE_CONTRACT_OFFER_FOR_TRANSFER);
    let l = read_usize(this + contract_manager_fields::CONTRACT_OFFERS);
    let mut found = false;
    if l != 0 {
        let mut no = read_usize(l + 2);
        let mut n = 0;
        while no != 0 && n < 100000 {
            let x = read_usize(no);
            if read_usize(x) == read_usize(person) && read_usize(x + 0x59) as i32 == slot {
                found = true;
                break;
            }
            no = read_usize(no + 6);
            n += 1;
        }
    }
    let before = read_u8(contract_offer_counter(person));
    o(this, person, slot);
    let after = read_u8(contract_offer_counter(person));
    let expected = if found { before.wrapping_sub(1) } else { before };
    SCOREBOARD.record(after == expected, || format!("remove_contract_offer_for_transfer(pessoa {:#x}, vaga {}) achou {} contador {} -> {}", read_usize(person), slot, found, before, after));
}

/// Comparer for `contract_manager_lowest_wage_demand` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a original e a cópia a partir do mesmo estado (o sorteio e o último contrato pedido [0xae1080]); a
/// devolução e a memória.
pub(crate) unsafe extern "stdcall" fn comparar_contract_manager_lowest_wage_demand(person: usize, club: usize) -> i32 {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, usize) -> i32>(IDX_CONTRACT_MANAGER_LOWEST_WAGE_DEMAND);
    let mut regions = vec![(G_CONTRACT_LOOKUP_RESULT, 4usize)];
    regions.extend(crate::rng::STATE_REGIONS);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&regions, || o(person, club), || crate::rng::without_counting(|| contract_manager_lowest_wage_demand(person, club)));
    SCOREBOARD.record(igual && r == n, || format!("contract_manager_lowest_wage_demand(pessoa {:#x}) original={} {:x?} rust={} {:x?}", read_usize(person), r, mo, n, mr));
    r
}

/// Comparer for `handle_transfer_request` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_handle_transfer_request(this: usize, contract: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_HANDLE_TRANSFER_REQUEST)(this, contract)
}

/// Comparer for `is_contract_staff_job` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor.
pub(crate) unsafe extern "stdcall" fn comparar_is_contract_staff_job(job: u32) -> u32 {
    let r = original_fn::<unsafe extern "stdcall" fn(u32) -> u32>(IDX_IS_CONTRACT_STAFF_JOB)(job);
    let n = is_contract_staff_job(job);
    SCOREBOARD.record(r == n, || format!("is_contract_staff_job({}) original={} rust={}", job as u8, r, n));
    r
}

/// Comparer for `staff_is_not_board_member` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor.
pub(crate) unsafe extern "stdcall" fn comparar_staff_is_not_board_member(person: usize) -> u32 {
    let r = original_fn::<unsafe extern "stdcall" fn(usize) -> u32>(IDX_STAFF_IS_NOT_BOARD_MEMBER)(person);
    let n = staff_is_not_board_member(person);
    SCOREBOARD.record(r == n, || format!("staff_is_not_board_member(pessoa {}, cargo {}) original={} rust={}", read_usize(person), read_u8(person + staff::JOB_FOR_CLUB), r, n));
    r
}

/// Comparer for `compare_decision_dates` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_compare_decision_dates(a: usize, b: usize) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_COMPARE_DECISION_DATES);
    let r = o(a, b);
    let n = compare_decision_dates(a, b);
    SCOREBOARD.record(r == n, || format!("compare_decision_dates original={} rust={}", r, n));
    r
}

/// Comparer for `process_due_decision_dates` (comparison mode: see the header of motor.rs).
///
/// The offer decision (0x4ce4e0) changes the contracts, the offers and the array itself: the plan (the dates read
/// beforehand, without running anything) tells whether any type-3 entry with a person falls due; if so, only the
/// original runs. If not (the everyday case), side by side over index +0x18 and the entries that fall due (they become
/// −1).
///
/// [pt] Comparador: a decisão das ofertas (0x4ce4e0) mexe nos contratos, nas ofertas e no próprio vetor: o plano (as datas
/// lidas antes, sem rodar nada) diz se alguma entrada do tipo 3 com pessoa vence; com ela, só a original. Sem ela (o
/// caso de todo dia), lado a lado pelo índice +0x18 e pelas entradas que vencem (ficam com −1).
pub(crate) unsafe extern "thiscall" fn comparar_process_due_decision_dates(this: usize, data: usize, slice: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_PROCESS_DUE_DECISION_DATES);
    let (i0, n) = (read_u32(this + contract_manager_fields::DECISION_NEXT_INDEX) as i32, read_u32(this + contract_manager_fields::DECISION_COUNT) as i32);
    let mut i = i0;
    let mut decides = false;
    while i < n {
        let e = (read_u32(this + contract_manager_fields::DECISION_DATES) as usize).wrapping_add((i as usize).wrapping_mul(14));
        if call_date_compare(0x456740, e + 4, data) != 0
            || (call_date_compare(0x4a7f20, e + 4, data) != 0 && read_u8(e + 0xc) as i8 > slice as u8 as i8) {
            break;
        }
        decides |= read_u32(e) as i32 != -1 && read_u8(e + 0xd) == 3;
        i += 1;
    }
    if decides {
        return o(this, data, slice);
    }
    let mut reg = vec![(this + contract_manager_fields::DECISION_NEXT_INDEX, 4)];
    if i > i0 {
        reg.push(((read_u32(this + contract_manager_fields::DECISION_DATES) as usize).wrapping_add((i0 as usize).wrapping_mul(14)), ((i - i0) as usize) * 14));
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, data, slice), || process_due_decision_dates(this, data, slice));
    SCOREBOARD.record(igual, || format!("process_due_decision_dates(fatia {}) {:x?} x {:x?}", slice as u8 as i8, mo, mr));
}

/// Comparer for `get_current_contract_offers` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a original aloca o vetor que quem chama libera: roda só ela e confere o plano feito antes (as ofertas
/// do clube lidas da lista, sem alocar): a conta e os 111 bytes de cada uma, na ordem.
pub(crate) unsafe extern "thiscall" fn comparar_get_current_contract_offers(this: usize, club: usize, out: usize) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u8>(IDX_GET_CURRENT_CONTRACT_OFFERS);
    let list = read_u32(this + contract_manager_fields::CONTRACT_OFFERS) as usize;
    if list == 0 || club == 0 || out == 0 {
        return o(this, club, out);
    }
    let mut plan: Vec<u8> = Vec::new();
    core::ptr::write_unaligned((list + 0xa) as *mut u32, read_u32(list + 2));
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let mut item = next(list);
    while item != 0 {
        let state = read_u8(item + 0x35) & 0x3f;
        if read_u32(item + 4) == read_u32(club) && state != 5 && state != 6 {
            plan.extend_from_slice(core::slice::from_raw_parts(item as *const u8, 111));
        }
        item = next(list);
    }
    let r = o(this, club, out);
    let array = read_u32(out) as usize;
    let k = plan.len() / 111;
    let ok = r as usize == k && (k == 0 || core::slice::from_raw_parts(array as *const u8, k * 111) == &plan[..]);
    SCOREBOARD.record(ok, || format!("get_current_contract_offers(clube {:x}) original {} ofertas, plano {}", club, r, k));
    r
}

/// Comparer for `sign_loan_contract` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_sign_loan_contract(this: usize, contract: usize) {
    // simplification: touches the squad, the tactics, the lists and reallocates the array: original only; the copy proves in Rust mode
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_SIGN_LOAN_CONTRACT)(this, contract)
}

/// Comparer for `end_loan` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_end_loan(this: usize, person: usize) {
    // simplification: touches the squad, the tactics, the lists and the disputes (0x556570 frees items): original only
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_END_LOAN)(this, person)
}

/// Comparer for `toggle_squad_placement` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_toggle_squad_placement(this: usize, person: usize, club: usize, out: usize) -> u32 {
    // simplification: touches the squad, the tactics and the lists: original only; the copy is proven in Rust mode
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32>(IDX_TOGGLE_SQUAD_PLACEMENT)(this, person, club, out)
}

/// Comparer for `staff_can_take_job` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "stdcall" fn comparar_staff_can_take_job(job: u32, person: usize) -> u32 {
    let r = original_fn::<unsafe extern "stdcall" fn(u32, usize) -> u32>(IDX_STAFF_CAN_TAKE_JOB)(job, person);
    let n = staff_can_take_job(job, person);
    SCOREBOARD.record(r == n, || format!("staff_can_take_job(cargo {}, pessoa {:x}) original={} rust={}", job as u8 as i8, person, r, n));
    r
}

/// Comparer for `contract_manager_lower_match_morale` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: as queixas (0x867700) sorteiam e podem abrir pedido de transferência: só a original; o plano (a queda
/// `m`, calculada antes, sem efeito) diz a moral +0x44 que ela deixa (as queixas não a gravam) ou que ela não mexe.
pub(crate) unsafe extern "thiscall" fn comparar_contract_manager_lower_match_morale(this: usize, person: usize, club: usize, contract: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(IDX_CONTRACT_MANAGER_LOWER_MATCH_MORALE);
    if contract == 0 {
        return o(this, person, club, contract);
    }
    let before = read_u8_volatile(contract + staff_contract::MORALE) as i8;
    let plan = match_morale_drop(person, club, contract);
    // deleting a complaint (0x8674e0) recalculates the contract (0x867240), which may rewrite morale: with a complaint marked before
    // or a drop above 5 (the branches that always delete), original only
    if read_u8_volatile(contract + staff_contract::GRIEVANCES) & 0x14 != 0 || plan.is_some_and(|m| m > 5) {
        return o(this, person, club, contract);
    }
    o(this, person, club, contract);
    let after = read_u8_volatile(contract + staff_contract::MORALE) as i8;
    let esperada = plan.map_or(before, |m| lowered_morale(before, m));
    SCOREBOARD.record(after == esperada, || format!("contract_manager_lower_match_morale(queda {:?}) moral {} -> original {} plano {}", plan, before, after, esperada));
}

/// Comparer for `remove_contract` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pela vaga da pessoa e pela entrada do contrato (a folha 0x598370 refeita duas vezes dá o
/// mesmo, pelo contrato já tirado).
pub(crate) unsafe extern "thiscall" fn comparar_remove_contract(this: usize, person: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_REMOVE_CONTRACT);
    let slot = main_contract_slot(person);
    let index = read_i32(slot);
    if index == -1 {
        return o(this, person);
    }
    let e = read_usize(this).wrapping_add((index as usize).wrapping_mul(0x50));
    let reg = [(slot, 4), (e, 8)];
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(this, person), || remove_contract(this, person));
    SCOREBOARD.record(igual, || format!("remove_contract(pessoa {:x}, contrato {}) {:x?} x {:x?}", read_usize(person), index, mo, mr));
}

/// Comparer for `resubmit_held_offers` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a original reenvia (0x4cc490, ofertas e transferências) e tira as ofertas da lista (libera): roda só ela
/// e confere o plano feito antes: devolve quantas ofertas da pessoa havia na lista.
pub(crate) unsafe extern "thiscall" fn comparar_resubmit_held_offers(this: usize, id: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_RESUBMIT_HELD_OFFERS);
    let list = read_usize(this + contract_manager_fields::HELD_OFFERS);
    if list == 0 {
        return o(this, id);
    }
    // the plan: the original restarts the cursor (+0xa) before traversing
    write_usize(list + 0xa, read_usize(list + 2));
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let mut expected = 0u32;
    let mut item = next(list);
    while item != 0 {
        if read_usize(item) == id {
            expected += 1;
        }
        item = next(list);
    }
    let r = o(this, id);
    SCOREBOARD.record(r == expected, || format!("resubmit_held_offers(id {:x}) devolveu {}, esperava {}", id, r, expected));
    r
}

/// Comparer for `contract_manager_season_end_month` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor (datas e a 0x669bd0 só calculam).
pub(crate) unsafe extern "stdcall" fn comparar_contract_manager_season_end_month(club: usize) -> u32 {
    let r = original_fn::<unsafe extern "stdcall" fn(usize) -> u32>(IDX_CONTRACT_MANAGER_SEASON_END_MONTH)(club);
    let n = contract_manager_season_end_month(club);
    SCOREBOARD.record(r == n, || format!("contract_manager_season_end_month({:x}) original={} rust={}", club, r, n));
    r
}

/// Comparer for `get_free_staff_contract_space` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a vaga esperada (a primeira vazia, ou a do fim com a conta + 1 e a capacidade + 200 se cheio).
pub(crate) unsafe extern "thiscall" fn comparar_get_free_staff_contract_space(this: usize, so_in_end: u32) -> i32 {
    let (n, cap, p) = (read_usize(this + 8), read_usize(this + 4), read_usize(this));
    let mut free = -1i32;
    if so_in_end == 0 {
        for i in 0..(n as i32).max(0) {
            if read_usize(p.wrapping_add((i as usize).wrapping_mul(0x50))) as i32 == -1 {
                free = i;
                break;
            }
        }
    }
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> i32>(IDX_GET_FREE_STAFF_CONTRACT_SPACE)(this, so_in_end);
    if r == -1 && read_usize(this) == 0 {
        return r; // out of memory: the box has already shown
    }
    let (er, en, ecap) = if free >= 0 { (free, n, cap) } else { (n as i32, n.wrapping_add(1), if n == cap { cap.wrapping_add(200) } else { cap }) };
    SCOREBOARD.record(r == er && read_usize(this + 8) == en && read_usize(this + 4) == ecap, || {
        format!("get_free_staff_contract_space({}) devolveu {} (esperava {}), conta {} -> {} (esperava {}), capacidade {} -> {} (esperava {})",
            so_in_end, r, er, n, read_usize(this + 8), en, cap, read_usize(this + 4), ecap)
    });
    r
}

/// Comparer for `add_staff_contract` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a vaga é a do fim (a conta de antes); confere a ligação da pessoa, o id no contrato e a conta.
pub(crate) unsafe extern "thiscall" fn comparar_add_staff_contract(this: usize, person: usize, value: u32) -> u32 {
    let n = read_usize(this + 8);
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_ADD_STAFF_CONTRACT)(this, person, value);
    if read_usize(this) == 0 {
        return r; // out of memory
    }
    let id = read_usize(person);
    let ligada = read_usize(read_usize(G_STAFF_STATE_TABLE).wrapping_add(id.wrapping_mul(0x4f)));
    let in_contract = read_usize(read_usize(this).wrapping_add(n.wrapping_mul(0x50)));
    SCOREBOARD.record(ligada == n && in_contract == id && read_usize(this + 8) == n.wrapping_add(1), || {
        format!("add_staff_contract(pessoa {}, valor {}) vaga {} (esperava {}), id no contrato {}, conta {} -> {}",
            id, value, ligada, n, in_contract, n, read_usize(this + 8))
    });
    r
}

/// Comparador: os lembretes e a entrada esperados antes da original; confere a conta e cada entrada nova.
pub(crate) unsafe extern "thiscall" fn comparar_add_decision_date(this: usize, person: usize, data: usize, slice: u32, kind: u32) {
    let n = read_usize(this + contract_manager_fields::DECISION_COUNT);
    let (lembretes, k) = decision_reminder_dates(this, person, data, kind as u8);
    let d = core::ptr::read_unaligned(data as *const [u8; 8]);
    let id = read_usize(person);
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32)>(IDX_ADD_DECISION_DATE)(this, person, data, slice, kind);
    if read_usize(this + contract_manager_fields::DECISION_DATES) == 0 {
        return; // out of memory: the box has already shown
    }
    let v = read_usize(this + contract_manager_fields::DECISION_DATES);
    let e = |j: usize| v.wrapping_add(n.wrapping_add(j).wrapping_mul(14));
    let data_de = |j: usize| core::ptr::read_unaligned((e(j) + 4) as *const [u8; 8]);
    let mut certo = read_usize(this + contract_manager_fields::DECISION_COUNT) == n.wrapping_add(k + 1);
    for (j, l) in lembretes[..k].iter().enumerate() {
        certo &= read_usize(e(j)) == id && data_de(j) == *l && *((e(j) + 0xd) as *const u8) == 2;
    }
    certo &= read_usize(e(k)) == id && data_de(k) == d && *((e(k) + 0xc) as *const u8) == slice as u8 && *((e(k) + 0xd) as *const u8) == kind as u8;
    SCOREBOARD.record(certo, || {
        format!("add_decision_date(pessoa {}, fatia {}, tipo {}) conta {} -> {} (esperava {}), lembretes {:?}, entradas {:?}",
            id, slice as u8, kind as u8, n, read_usize(this + contract_manager_fields::DECISION_COUNT), n + k + 1, &lembretes[..k],
            (0..=k).map(|j| core::ptr::read_unaligned(e(j) as *const [u8; 14])).collect::<Vec<_>>())
    });
}

/// Comparer for `trigger_relegation_clauses` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: os contratos que a cópia acionaria (antes); a original aciona e dá as notícias (não roda duas vezes);
/// confere que todos ficaram com 2.
pub(crate) unsafe extern "thiscall" fn comparar_trigger_relegation_clauses(this: usize, club: usize, league: usize) {
    let mut alvos = Vec::new();
    for c in club_and_b_team(club) {
        if c != 0 {
            alvos.extend((0..50).map(|k| contract_with_relegation_clause(this, c, read_usize(c + 0xd7 + k * 4))).filter(|&x| x != 0));
        }
    }
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_TRIGGER_RELEGATION_CLAUSES)(this, club, league);
    let away: Vec<usize> = alvos.iter().copied().filter(|&x| *((x + 0x1f) as *const u8) != 2).collect();
    SCOREBOARD.record(away.is_empty(), || format!("trigger_relegation_clauses({:x}) {} contratos, sem acionar {:x?}", club, alvos.len(), away));
}

/// Comparador: a lista esperada (sem a primeira oferta da pessoa ao clube) e as pendentes antes; a original tira (e
/// libera o item: nada dele é lido depois); confere os itens e o byte.
pub(crate) unsafe extern "thiscall" fn comparar_remove_contract_offer(this: usize, person: *const u32, club: *const u32) {
    let l = read_usize(this + contract_manager_fields::CONTRACT_OFFERS);
    if l == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, *const u32, *const u32)>(IDX_REMOVE_CONTRACT_OFFER)(this, person, club);
    }
    let n = (read_usize(G_STAFF_STATE_TABLE).wrapping_add((*person as usize).wrapping_mul(79)) + 8) as *const u8;
    let target = first_offer_for_staff_club(l, *person as usize, *club as usize);
    let expected: Vec<usize> = list_items_capped(l).into_iter().filter(|&e| Some(e) != target).collect();
    let pendentes = if target.is_some() { (*n).wrapping_sub(1) } else { *n };
    original_fn::<unsafe extern "thiscall" fn(usize, *const u32, *const u32)>(IDX_REMOVE_CONTRACT_OFFER)(this, person, club);
    let after = list_items_capped(l);
    SCOREBOARD.record(after == expected && *n == pendentes, || format!("remove_contract_offer({}, {}) itens {} (esperava {}), pendentes {} (esperava {})", *person, *club, after.len(), expected.len(), *n, pendentes));
}

/// Comparer for `remove_pending_offers_for_staff` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: o mesmo plano pelo byte de pendentes; a original tira (e desmarca); confere os itens e o byte.
pub(crate) unsafe extern "thiscall" fn comparar_remove_pending_offers_for_staff(this: usize, person: *const u32) {
    let l = read_usize(this + contract_manager_fields::CONTRACT_OFFERS);
    if l == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, *const u32)>(IDX_REMOVE_PENDING_OFFERS_FOR_STAFF)(this, person);
    }
    let p = pending_offers_byte(*person as usize);
    let before = list_items_capped(l);
    let mut resta = *p;
    let mut expected = Vec::new();
    for &e in &before {
        if resta != 0 && read_usize(e) as u32 == *person {
            resta = resta.wrapping_sub(1);
        } else {
            expected.push(e);
        }
    }
    original_fn::<unsafe extern "thiscall" fn(usize, *const u32)>(IDX_REMOVE_PENDING_OFFERS_FOR_STAFF)(this, person);
    let after = list_items_capped(l);
    SCOREBOARD.record(after == expected && *p == resta, || format!("remove_pending_offers_for_staff({}) itens {} -> {} (esperava {}), pendentes {} (esperava {})", *person, before.len(), after.len(), expected.len(), *p, resta));
}

/// Comparer for `schedule_decision_for_transfer` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a oferta que a cópia marcaria antes; a original marca e agenda (a 0x4cd230 pode acrescentar na lista
/// das decisões); confere a marca e o valor.
pub(crate) unsafe extern "thiscall" fn comparar_schedule_decision_for_transfer(this: usize, offer: usize) -> u32 {
    let target = offer_to_schedule(read_usize(this + contract_manager_fields::CONTRACT_OFFERS), offer);
    let n = ability_above_club_reputation(offer);
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_SCHEDULE_DECISION_FOR_TRANSFER)(this, offer);
    let marcada = target.map_or(true, |e| *((e + 0x62) as *const u8) == 1);
    SCOREBOARD.record(marcada && r == n, || format!("schedule_decision_for_transfer({:x}) alvo {:x?} marcada={} original={} rust={}", offer, target, marcada, r, n));
    r
}

/// Comparer for `remove_held_offers` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: o plano antes (a 0x68d1f0 libera o item tirado, que não pode mais ser lido); a original tira; confere
/// que ficaram os itens de antes menos os da pessoa, na mesma ordem.
pub(crate) unsafe extern "thiscall" fn comparar_remove_held_offers(this: usize, person: u32) {
    let l = read_usize(this + contract_manager_fields::HELD_OFFERS);
    let before: Vec<usize> = if l != 0 { list_items_capped(l) } else { Vec::new() };
    let expected: Vec<usize> = before.iter().copied().filter(|&e| read_usize(e) as u32 != person).collect();
    original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_REMOVE_HELD_OFFERS)(this, person);
    let after: Vec<usize> = if l != 0 { list_items_capped(l) } else { Vec::new() };
    SCOREBOARD.record(after == expected, || format!("remove_held_offers({:x}) {} itens, esperava {}, ficaram {}", person, before.len(), expected.len(), after.len()));
}

/// Comparer for `get_squad_placement` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pela ficha [0xae1080] (a vaga da saída é o argumento na pilha de cada uma).
pub(crate) unsafe extern "thiscall" fn comparar_get_squad_placement(this: usize, person: usize, club: usize) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u8>(IDX_GET_SQUAD_PLACEMENT);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(G_CONTRACT_LOOKUP_RESULT, 4)], || o(this, person, club), || get_squad_placement(this, person, club));
    SCOREBOARD.record(igual && r == n, || format!("get_squad_placement({:x}, {:x}) original={} rust={} {:x?} x {:x?}", person, club, r, n, mo, mr));
    r
}

/// Comparer for `move_to_first_team` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_move_to_first_team(this: usize, person: usize, club: usize) -> u32 {
    compare_placement_toggle(830, "move_to_first_team", this, person, club, [1, 3])
}

/// Comparer for `move_to_reserves` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_move_to_reserves(this: usize, person: usize, club: usize) -> u32 {
    compare_placement_toggle(831, "move_to_reserves", this, person, club, [2, 4])
}

/// Comparer for `contract_manager_operator` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_contract_manager_operator(this: usize, source: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_CONTRACT_MANAGER_OPERATOR);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(this, 0x50)], || o(this, source), || contract_manager_operator(this, source));
    SCOREBOARD.record(igual && r == n, || format!("contract_manager_operator {:x?} x {:x?}", mo, mr));
    r
}

/// Comparer for `contract_manager_raise_match_morale` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "stdcall" fn comparar_contract_manager_raise_match_morale(person: usize, club: usize, contract: usize) {
    original_fn::<unsafe extern "stdcall" fn(usize, usize, usize)>(IDX_CONTRACT_MANAGER_RAISE_MATCH_MORALE)(person, club, contract)
}

/// Comparer for `remove_talks_records` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_remove_talks_records(this: usize, person: usize, manager: usize, club: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(IDX_REMOVE_TALKS_RECORDS);
    let n = (*((this + contract_manager_fields::TALKS_COUNT) as *const i32)).max(0) as usize;
    let reg = [(this + contract_manager_fields::TALKS_RECORDS, 8), (read_u32(this + contract_manager_fields::TALKS_RECORDS) as usize, n * 21)];
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, person, manager, club), || remove_talks_records(this, person, manager, club));
    SCOREBOARD.record(igual, || format!("remove_talks_records({:x}, {:x}, {:x}) {:x?} x {:x?}", person, manager, club, mo, mr));
}

/// Comparer for `remove_loan_contract` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_remove_loan_contract(manager: usize, id: *const i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *const i32)>(IDX_REMOVE_LOAN_CONTRACT);
    let e = loan_state_entry(id);
    let mut reg = vec![(e + 4, 4)];
    let slot = core::ptr::read_unaligned((e + 4) as *const i32);
    if slot != -1 {
        reg.push(((*(manager as *const usize) as isize + slot as isize * 80) as usize, 4));
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(manager, id), || remove_loan_contract(manager, id));
    SCOREBOARD.record(igual, || format!("remove_loan_contract({}) {:x?} x {:x?}", *id, mo, mr));
}

/// Comparer for `contract_offer` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_contract_offer(manager: usize, person: i32, club: i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32, i32) -> usize>(IDX_CONTRACT_OFFER);
    let reg = offer_list_regions(manager);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(manager, person, club), || contract_offer(manager, person, club));
    SCOREBOARD.record(igual && r == n, || format!("contract_offer({}, {}) original={:#x} rust={:#x} {:x?} x {:x?}", person, club, r, n, mo, mr));
    r
}

/// Comparer for `agreed_contract_offer` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_agreed_contract_offer(manager: usize, person: i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32) -> usize>(IDX_AGREED_CONTRACT_OFFER);
    let reg = offer_list_regions(manager);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(manager, person), || agreed_contract_offer(manager, person));
    SCOREBOARD.record(igual && r == n, || format!("agreed_contract_offer({}) original={:#x} rust={:#x} {:x?} x {:x?}", person, r, n, mo, mr));
    r
}

/// Comparer for `schedule_offer_decision` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: sem a entrada (a 0x4cd6d0 acrescenta e pode realocar) só a original; com ela, lado a lado pelo sorteio,
/// as datas estáticas, o vetor das decisões e +0x1c (a cópia sem contar).
pub(crate) unsafe extern "thiscall" fn comparar_schedule_offer_decision(this: usize, id: i32, slot: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32, i32)>(IDX_SCHEDULE_OFFER_DECISION);
    let mut limit = [0u8; 8];
    injury_date_add_days(G_CURRENT_DATE, 0xe, &mut limit);
    if find_type3_decision(this, id, &limit, false) == 0 {
        return o(this, id, slot);
    }
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.extend_from_slice(&INJURY_DATE_STATICS);
    reg.push((this + contract_manager_fields::DECISIONS_ADDED, 4));
    reg.push((read_u32_as_usize(this + contract_manager_fields::DECISION_DATES), read_u32_as_usize(this + contract_manager_fields::DECISION_COUNT).wrapping_mul(14)));
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, id, slot),
        || crate::rng::without_counting(|| schedule_offer_decision(this, id, slot)));
    SCOREBOARD.record(igual, || format!("schedule_offer_decision(id {}, vaga {}) {:x?} x {:x?}", id, slot, mo, mr));
}

/// Comparer for `contract_category` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "stdcall" fn comparar_contract_category(p: usize, c: usize) -> u8 {
    let r = original_fn::<unsafe extern "stdcall" fn(usize, usize) -> u8>(IDX_CONTRACT_CATEGORY)(p, c);
    let n = contract_category(p, c);
    SCOREBOARD.record(r == n, || format!("contract_category({:#x}, {:#x}) original={} rust={}", p, c, r, n));
    r
}

/// Comparer for `post_transfer_listed_news` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x4d71b0: só a original (sorteia, entrega a notícia, que aloca).
pub(crate) unsafe extern "thiscall" fn comparar_post_transfer_listed_news(this: usize, player: usize, club: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_POST_TRANSFER_LISTED_NEWS)(this, player, club) // simplification: original only
}

/// Comparer for `schedule_loan_end` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x4ccca0: só a original (a volta do empréstimo entrega notícias e mexe no elenco).
pub(crate) unsafe extern "thiscall" fn comparar_schedule_loan_end(this: usize, person: usize, club: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_SCHEDULE_LOAN_END)(this, person, club) // simplification: original only
}

/// Comparer for `post_contract_end_news` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_post_contract_end_news(this: usize, p: usize, kind: u32, amplo: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32)>(IDX_POST_CONTRACT_END_NEWS)(this, p, kind, amplo)
}

/// Comparer for `contract_with_club` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_contract_with_club(this: usize, person: usize, club: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> usize>(IDX_CONTRACT_WITH_CLUB);
    if is_recording_post_match() {
        return call_recording(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB, this, [person as u32, club as u32, 0, 0, 0], || o(this, person, club) as u32) as usize;
    }
    let r = o(this, person, club);
    let n = contract_with_club(this, person, club);
    SCOREBOARD.record(r == n, || format!("contract_with_club({:#x}, {:?}, {:?}) original={:#x} rust={:#x}", this, person, club, r, n));
    r
}

/// Comparer for `contract_level` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_contract_level(person: usize, club: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(IDX_CONTRACT_LEVEL)(person, club);
    let v = contract_level(person, club);
    SCOREBOARD.record(r == v, || format!("contract_level original={:#x} rust={:#x}", r, v));
    r
}
