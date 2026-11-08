use super::*;

// The comparers of the routines in ../shortlist_manager.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../shortlist_manager.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `make_club_bids` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x84a020: só a original (sorteia, faz lances e pedidos de empréstimo, aloca ofertas e termos).
pub(crate) unsafe extern "thiscall" fn comparar_make_club_bids(this: usize, club: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_MAKE_CLUB_BIDS)(this, club) // simplification: original only
}

/// Comparer for `staff_interest_in_joining_club` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado na nota (os bits do f64), nos motivos e em [0xae1080] (que a original e o salário padrão
/// 0x864550 gravam); o resto das chamadas só lê, e o STAFF_CONTRACT do salário é local.
pub(crate) unsafe extern "thiscall" fn comparar_staff_interest_in_joining_club(this: usize, p1: usize, p2: usize, p3: u32, p4: u32, p5: u32, p6: *mut u32) -> f64 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, *mut u32) -> f64>(IDX_STAFF_INTEREST_IN_JOINING_CLUB);
    let mut regions = vec![(G_CONTRACT_LOOKUP_RESULT, 4usize)];
    if !p6.is_null() {
        regions.push((p6 as usize, 4));
    }
    let (ro, rr, igual, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, p1, p2, p3, p4, p5, p6), || staff_interest_in_joining_club(this, p1, p2, p3, p4, p5, p6));
    SCOREBOARD.record(igual && ro.to_bits() == rr.to_bits(), || format!("staff_interest_in_joining_club(pessoa {:#x}, clube {:#x}, {}, {}, {}) original={} {:x?} rust={} {:x?}", p1, p2, p3, p4 as u8, p5 as u8, ro, mo, rr, mr));
    ro
}

/// Comparer for `shortlist_manager_is_compensation_deal` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado (rotina pura: só a taxa e a devolução), pelas cópias leves.
pub(crate) unsafe extern "thiscall" fn comparar_shortlist_manager_is_compensation_deal(this: usize, slot: usize, contract: usize, club: usize, taxa: *mut i32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut i32) -> u32>(IDX_SHORTLIST_MANAGER_IS_COMPENSATION_DEAL);
    let reg = [(taxa as usize, 4)];
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(this, slot, contract, club, taxa),
        || shortlist_manager_is_compensation_deal(this, slot, contract, club, taxa));
    SCOREBOARD.record(igual && r == n, || format!("shortlist_manager_is_compensation_deal(vaga {:#x}, contrato {:#x}, clube {:#x}) original={} rust={} {:x?} x {:x?}",
        slot, contract, club, r, n, mo, mr));
    r
}

/// Comparer for `shortlist_compare_slots_by_index` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a devolução (pura).
pub(crate) unsafe extern "C" fn comparar_shortlist_compare_slots_by_index(a: usize, b: usize) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_SHORTLIST_COMPARE_SLOTS_BY_INDEX);
    let r = o(a, b);
    let n = shortlist_compare_slots_by_index(a, b);
    SCOREBOARD.record(r == n, || format!("shortlist_compare_slots_by_index({:#x}, {:#x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `mark_player_under_offer` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelas listas de 40 vagas dos clubes interessados (a cópia e a original montam e soltam a
/// mesma lista local) e pelo cursor da lista `troca`.
pub(crate) unsafe extern "thiscall" fn comparar_mark_player_under_offer(this: usize, player: usize, buyer: usize, swap: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(IDX_MARK_PLAYER_UNDER_OFFER);
    let mut reg: Vec<(usize, usize)> = interested_clubs(this, player).into_iter()
        .map(|k| read_usize(read_usize(this + 0xdc).wrapping_add(k.wrapping_mul(9))))
        .filter(|&t| t != 0)
        .map(|t| (t, 40 * 0x25))
        .collect();
    if swap != 0 {
        reg.push((swap + 0xa, 4));
    }
    let ((), (), igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, player, buyer, swap),
        || mark_player_under_offer(this, player, buyer, swap));
    SCOREBOARD.record(igual, || format!("mark_player_under_offer(jogador {:#x}, comprador {:#x}, troca {:#x}) {:x?} x {:x?}",
        player, buyer, swap, mo, mr));
}

/// Comparer for `value_player_for_club` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x840d80: lado a lado na nota de saída e na devolução (o resto ela só lê). O técnico do clube sem a
/// ficha +0x69 (a original sorteia uma): só a original.
pub(crate) unsafe extern "thiscall" fn comparar_value_player_for_club(this: usize, club: usize, p: usize, slot: usize, out: *mut i32, minimum: i32, ceiling: u32) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut i32, i32, u32) -> i32>(IDX_VALUE_PLAYER_FOR_CLUB);
    let obs = read_usize(club + club::MANAGER);
    if obs != 0 && read_usize(obs + 0x69) == 0 {
        return o(this, club, p, slot, out, minimum, ceiling);
    }
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&[(out as usize, 4)], || o(this, club, p, slot, out, minimum, ceiling), || value_player_for_club(this, club, p, slot, out, minimum, ceiling));
    SCOREBOARD.record(igual && r == n, || {
        format!("value_player_for_club(clube {:x}, pessoa {:x}, vaga {:x}, mínimo {}, teto {}) original={} rust={} {:x?} x {:x?}", club, p, slot, minimum, ceiling, r, n, mo, mr)
    });
    r
}

/// Comparer for `count_slots_with_bids` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelas 40 vagas do clube e pelas datas estáticas, e a devolução.
pub(crate) unsafe extern "thiscall" fn comparar_count_slots_with_bids(this: usize, club: usize, position: u32) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u8>(IDX_COUNT_SLOTS_WITH_BIDS);
    let mut reg = SHORTLIST_MANAGER_DATE_STATICS.to_vec();
    reg.push((read_usize(shortlist_club_entry(this, club)), 40 * 0x25));
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, club, position), || count_slots_with_bids(this, club, position));
    SCOREBOARD.record(igual && r == n, || format!("count_slots_with_bids(clube {:x}, posicao {}) original={} rust={} {:x?} x {:x?}", club, position as i8, r, n, mo, mr));
    r
}

/// Comparer for `shortlist_manager_remove_from_transfer_list` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelo byte +0x4e do contrato, e a devolução.
pub(crate) unsafe extern "stdcall" fn comparar_shortlist_manager_remove_from_transfer_list(person: usize) -> u32 {
    let o = original_fn::<unsafe extern "stdcall" fn(usize) -> u32>(IDX_SHORTLIST_MANAGER_REMOVE_FROM_TRANSFER_LIST);
    let c = contract_of_staff(person);
    let reg = if c != 0 { vec![(c + 0x4e, 1)] } else { Vec::new() };
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(person), || shortlist_manager_remove_from_transfer_list(person));
    SCOREBOARD.record(igual && r == n, || format!("shortlist_manager_remove_from_transfer_list(pessoa {:x}) original={:#x} rust={:#x} {:x?} x {:x?}", person, r, n, mo, mr));
    r
}

/// Comparer for `shortlist_manager_remove_from_loan_list` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelo byte +0x4e do contrato, e a devolução.
pub(crate) unsafe extern "stdcall" fn comparar_shortlist_manager_remove_from_loan_list(person: usize) -> u32 {
    let o = original_fn::<unsafe extern "stdcall" fn(usize) -> u32>(IDX_SHORTLIST_MANAGER_REMOVE_FROM_LOAN_LIST);
    let c = contract_of_staff(person);
    let reg = if c != 0 { vec![(c + 0x4e, 1)] } else { Vec::new() };
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(person), || shortlist_manager_remove_from_loan_list(person));
    SCOREBOARD.record(igual && r == n, || format!("shortlist_manager_remove_from_loan_list(pessoa {:x}) original={:#x} rust={:#x} {:x?} x {:x?}", person, r, n, mo, mr));
    r
}

/// Comparer for `update_staff_status` (comparison mode: see the header of motor.rs): side by side.
///
/// Compares the two pair vectors and their counts, the byte of the person's list count, the 40 slots and `+8` of the
/// club's entry, byte `+0x4f` of the contract and the static dates, and the return value. In the recursion of event 4
/// and in event 1 that fills the slot, only the original routine (the copy covers those branches).
///
/// [pt] Comparador: lado a lado pelos dois vetores de pares e suas contas, pelo byte da conta de listas da pessoa, pelas 40
/// vagas e o +8 da entrada do clube, pelo byte +0x4f do contrato e pelas datas estáticas, e a devolução. Na recursão do
/// evento 4 e no evento 1 que preenche a vaga, só a original (a cópia leva os ramos).
pub(crate) unsafe extern "thiscall" fn comparar_update_staff_status(this: usize, person: usize, club: usize, event: u32, kind: u32) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u8>(IDX_UPDATE_STAFF_STATUS);
    if is_original_only_case(this, person, club, event, kind) {
        return o(this, person, club, event, kind);
    }
    let mut reg = STAFF_STATUS_DATE_STATICS.to_vec();
    for base in [this, this + 4] {
        reg.push((base + 0x10, 4));
        if read_usize(base) != 0 {
            reg.push((read_usize(base), read_usize(base + 0x10).wrapping_mul(8)));
        }
    }
    reg.push((read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(person).wrapping_mul(0x4f)) + 0xb, 1));
    if club != 0 {
        let entry = shortlist_club_entry(this, club);
        reg.push((read_usize(entry), 40 * 0x25));
        reg.push((entry + 8, 1));
        let contract_of: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
        let c = contract_of(read_usize(G_CONTRACT_MANAGER), person, club);
        if c != 0 {
            reg.push((c + 0x4f, 1));
        }
    }
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, person, club, event, kind),
        || update_staff_status(this, person, club, event, kind));
    SCOREBOARD.record(igual && r == n, || format!("update_staff_status(pessoa {:x}, clube {:x}, evento {}, tipo {}) original={} rust={} {:x?} x {:x?}",
        person, club, event as u8, kind as u8, r, n, mo, mr));
    r
}

/// Comparer for `shortlist_manager_add_to_loan_list` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelo byte +0x4e do contrato, e a devolução.
pub(crate) unsafe extern "stdcall" fn comparar_shortlist_manager_add_to_loan_list(person: usize) -> usize {
    let o = original_fn::<unsafe extern "stdcall" fn(usize) -> usize>(IDX_SHORTLIST_MANAGER_ADD_TO_LOAN_LIST);
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = f(read_usize(G_CONTRACT_MANAGER), person);
    let reg = if c != 0 { vec![(c + 0x4e, 1)] } else { Vec::new() };
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(person), || shortlist_manager_add_to_loan_list(person));
    SCOREBOARD.record(igual && r == n, || format!("shortlist_manager_add_to_loan_list(pessoa {:x}) original={:#x} rust={:#x} {:x?} x {:x?}", person, r, n, mo, mr));
    r
}

/// Comparer for `shortlist_manager_max_fee_for_player_status` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a devolução (rotina pura).
pub(crate) unsafe extern "thiscall" fn comparar_shortlist_manager_max_fee_for_player_status(this: usize, club: usize, person: usize, status: u32) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32) -> i32>(IDX_SHORTLIST_MANAGER_MAX_FEE_FOR_PLAYER_STATUS);
    let r = o(this, club, person, status);
    let n = shortlist_manager_max_fee_for_player_status(this, club, person, status);
    SCOREBOARD.record(r == n, || format!("shortlist_manager_max_fee_for_player_status(clube {:x}, pessoa {:x}, estatuto {}) original={} rust={}", club, person, status as i8, r, n));
    r
}

/// Comparer for `allocate_transfer_budget` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_allocate_transfer_budget(this: usize, club: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_ALLOCATE_TRANSFER_BUDGET)(this, club)
}

/// Comparer for `shortlist_manager_squad_status_allowed` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a devolução (rotina pura); sem ficha de jogador, só a original (caixa).
pub(crate) unsafe extern "stdcall" fn comparar_shortlist_manager_squad_status_allowed(person: usize, status: u32) -> u32 {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, u32) -> u32>(IDX_SHORTLIST_MANAGER_SQUAD_STATUS_ALLOWED);
    if read_usize(person + staff::PLAYER) == 0 {
        return o(person, status);
    }
    let r = o(person, status);
    let n = shortlist_manager_squad_status_allowed(person, status);
    SCOREBOARD.record(r == n, || format!("shortlist_manager_squad_status_allowed(pessoa {:x}, estatuto {}) original={} rust={}", person, status as i8, r, n));
    r
}

/// Comparador: lado a lado pelas 40 vagas da entrada (sem chamadas).
pub(crate) unsafe extern "stdcall" fn comparar_shortlist_set_up_formation_slots(entry: usize) {
    let o = original_fn::<unsafe extern "stdcall" fn(usize)>(IDX_SHORTLIST_SET_UP_FORMATION_SLOTS);
    let reg = [(read_usize(entry), 40 * 0x25)];
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(entry), || shortlist_set_up_formation_slots(entry));
    SCOREBOARD.record(igual, || format!("shortlist_set_up_formation_slots(formacao {}) primeira diferença {:x?}", *((entry + 7) as *const i8), first_memory_diff(&mo, &mr)));
}

/// Comparer for `update_club_shortlist` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_update_club_shortlist(this: usize, club: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_UPDATE_CLUB_SHORTLIST)(this, club)
}

/// Comparer for `suspend_clubs_shortlisting_player` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelas entradas (9 bytes) e, com `limpar`, pelas vagas (40 × 0x25) dos clubes do plano: os
/// clubes que a 0x842180 (pura) junta antes, numa lista própria. Cada lado cria e libera a sua lista.
pub(crate) unsafe extern "thiscall" fn comparar_suspend_clubs_shortlisting_player(this: usize, player: usize, clear: u8) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u8)>(IDX_SUSPEND_CLUBS_SHORTLISTING_PLAYER);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let list_new: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let interested: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(FN_SHORTLIST_MANAGER_COLLECT_INTERESTED_CLUBS);
    let destroy: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let free: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_OPERATOR_DELETE);
    let p = novo(0xe);
    if p == 0 {
        return o(this, player, clear);
    }
    let list = list_new(p);
    interested(this, player, list, 1, 3, 0);
    // the items (8 bytes: player id, club id) through the nodes (+2 the first; node: +0 the data, +6 the next)
    let mut reg: Vec<(usize, usize)> = Vec::new();
    let mut no = read_usize(list + 2);
    while no != 0 {
        let e = read_usize(this + 0xdc).wrapping_add(read_usize(read_usize(no) + 4).wrapping_mul(9));
        reg.push((e, 9));
        if clear != 0 {
            reg.push((read_usize(e), 0x28 * 0x25));
        }
        no = read_usize(no + 6);
    }
    destroy(list);
    free(list);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, player, clear), || suspend_clubs_shortlisting_player(this, player, clear));
    SCOREBOARD.record(igual, || format!("suspend_clubs_shortlisting_player(jogador {:x}, limpar {}, {} regiões) primeira diferença {:x?}", read_usize(player), clear, reg.len(), first_memory_diff(&mo, &mr)));
}

/// Comparer for `add_player_to_club_shortlists` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_add_player_to_club_shortlists(this: usize, player: usize, force: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_ADD_PLAYER_TO_CLUB_SHORTLISTS)(this, player, force)
}

/// Comparer for `transfer_list_player` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: sorteia e dá notícia: roda só a original e confere o plano feito antes: sem listagem do clube e com
/// contrato, o contrato fica à venda (+0x4e bit 1, +0x4f nibble alto 7).
pub(crate) unsafe extern "thiscall" fn comparar_transfer_list_player(this: usize, player: usize, news: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_TRANSFER_LIST_PLAYER);
    let listed: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_TRANSFER_LISTED);
    let mark = listed(player, 0) == 0 && get_contract(player) != 0;
    o(this, player, news);
    if mark {
        let c = get_contract(player);
        let ok = c != 0 && read_u8(c + 0x4e) & 1 != 0 && read_u8(c + 0x4f) & 0xf0 == 0x70;
        SCOREBOARD.record(ok, || format!("transfer_list_player(jogador {:x}) contrato {:x} sem a marca de à venda", read_usize(player), c));
    }
}

/// Comparer for `build_position_lists` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_build_position_lists(this: usize, not_used: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_BUILD_POSITION_LISTS)(this, not_used);
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
            if read_usize(a) != read_usize(b) || serve(staff_records_staff_by_id(read_usize(a)), vaga as u32) == 0 {
                ruim = Some((vaga, j as i32));
                break 'f;
            }
        }
    }
    SCOREBOARD.record(ruim.is_none(), || format!("build_position_lists: vaga e entrada {:?}", ruim));
}

/// Comparador: lado a lado pelos 32 vetores (12 bytes por entrada) e por [0xae1080].
pub(crate) unsafe extern "thiscall" fn comparar_sort_position_lists(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_SORT_POSITION_LISTS);
    let mut reg = vec![(G_CONTRACT_LOOKUP_RESULT, 4usize)];
    for vaga in 0..16usize {
        let n = read_i32(this + 0x98 + vaga * 4);
        if n > 0 {
            reg.push((read_usize(this + 0x18 + vaga * 4), n as usize * 12));
            reg.push((read_usize(this + 0x58 + vaga * 4), n as usize * 12));
        }
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this), || sort_position_lists(this));
    SCOREBOARD.record(igual, || {
        let dif: Vec<usize> = mo.0.iter().zip(mr.0.iter()).filter(|(a, b)| a.1 != b.1).map(|(a, _)| a.0).take(8).collect();
        format!("sort_position_lists: vetores diferentes {:x?}", dif)
    });
}

/// Comparer for `collect_interested_clubs` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a original na lista de saída e a cópia numa lista nova (0x68d020, solta com 0x68d080); compara os pares.
pub(crate) unsafe extern "thiscall" fn comparar_collect_interested_clubs(this: usize, player: usize, out: usize, ignore: u32, mode: u32, so_humanos: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32)>(IDX_COLLECT_INTERESTED_CLUBS);
    o(this, player, out, ignore, mode, so_humanos);
    if out == 0 {
        return;
    }
    let create: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let clear: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_CLEAR);
    let mut new = [0u8; 0x10];
    let l = new.as_mut_ptr() as usize;
    create(l);
    collect_interested_clubs(this, player, l, ignore, mode, so_humanos);
    let (a, b) = (llist_pairs(out), llist_pairs(l));
    clear(l);
    SCOREBOARD.record(a == b, || format!("collect_interested_clubs({:#x}) original={:x?} rust={:x?}", player, a, b));
}

/// Comparer for `add_shortlist_entry` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_add_shortlist_entry(this: usize, person: usize, club: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_ADD_SHORTLIST_ENTRY);
    let bsearch: unsafe extern "C" fn(*const u32, usize, i32, u32, usize) -> usize = core::mem::transmute(FN_CRT_BSEARCH);
    let human: unsafe extern "C" fn(usize, u32, u32, usize, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let key = [read_usize(person) as u32, if club != 0 { read_usize(club) as u32 } else { 0xffff_ffff }];
    let tinha = read_usize(this) != 0 && bsearch(key.as_ptr(), read_usize(this), read_i32(this + 0x10), 8, 0x757ea0) != 0;
    let (n0, n1) = if read_usize(this) != 0 { (read_i32(this + 0x10), read_i32(this + 0x14)) } else { (0, 0) };
    let r = read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(person).wrapping_mul(0x4f)) + 0xb;
    let nib = read_u8(r);
    let sum = !tinha && human(club, 0, 1, 0, 0) == 0;
    o(this, person, club);
    let expected = if sum { (nib & 0xf0) | (nib.wrapping_add(1) & 0xf) } else { nib };
    let (d0, d1) = (if tinha { n0 } else { n0 + 1 }, if tinha { n1 } else { n1 + 1 });
    let found = bsearch(key.as_ptr(), read_usize(this), read_i32(this + 0x10), 8, 0x757ea0) != 0;
    let ok = read_i32(this + 0x10) == d0 && read_i32(this + 0x14) == d1 && read_u8(r) == expected && (found || club == 0);
    SCOREBOARD.record(ok, || {
        format!("add_shortlist_entry({:#x}, {:#x}) tinha={} contas {}/{} -> {}/{} nibble {:#x} -> {:#x} (esperado {:#x})",
            person, club, tinha, n0, n1, read_i32(this + 0x10), read_i32(this + 0x14), nib, read_u8(r), expected)
    });
}

/// Comparer for `shortlist_manager_predict_selling_club_response` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor (o byte baixo) e lado a lado pelo sorteio e [0xae1080] (as chamadas de preço podem sortear).
pub(crate) unsafe extern "stdcall" fn comparar_shortlist_manager_predict_selling_club_response(seller: usize, player: usize, offer: i64, buyer: usize, request: i64, loan: u32) -> u8 {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, usize, i64, usize, i64, u32) -> u8>(IDX_SHORTLIST_MANAGER_PREDICT_SELLING_CLUB_RESPONSE);
    let mut reg = vec![(G_CONTRACT_LOOKUP_RESULT, 4usize)];
    reg.extend(crate::rng::STATE_REGIONS);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(
        &reg,
        || o(seller, player, offer, buyer, request, loan),
        || crate::rng::without_counting(|| shortlist_manager_predict_selling_club_response(seller, player, offer, buyer, request, loan)),
    );
    SCOREBOARD.record(igual && r == n, || format!("shortlist_manager_predict_selling_club_response({:#x}, {:#x}, {}, {:#x}, {}, {}) original={} rust={} {:x?} x {:x?}", seller, player, offer, buyer, request, loan as u8, r, n, mo, mr));
    r
}

/// Comparer for `get_squad_status_for_club` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelo estatuto, pelo cargo de saída e por [0xae1080].
pub(crate) unsafe extern "thiscall" fn comparar_get_squad_status_for_club(this: usize, person: usize, club: usize, job: *mut u8) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, *mut u8) -> u8>(IDX_GET_SQUAD_STATUS_FOR_CLUB);
    let out = if job.is_null() { G_CONTRACT_LOOKUP_RESULT } else { job as usize };
    let reg = [(G_CONTRACT_LOOKUP_RESULT, 4usize), (out, 1)];
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(this, person, club, job), || get_squad_status_for_club(this, person, club, job));
    SCOREBOARD.record(igual && r == n, || format!("get_squad_status_for_club({:#x}, {:#x}) original={} rust={} {:x?} x {:x?}", person, club, r, n, mo, mr));
    r
}

/// Comparer for `shortlist_manager_estimate_squad_status` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor (o byte baixo; a original deixa a palavra de estado do x87 em ah).
pub(crate) unsafe extern "stdcall" fn comparar_shortlist_manager_estimate_squad_status(person: usize, club: usize) -> u32 {
    let r = original_fn::<unsafe extern "stdcall" fn(usize, usize) -> u32>(IDX_SHORTLIST_MANAGER_ESTIMATE_SQUAD_STATUS)(person, club);
    let n = shortlist_manager_estimate_squad_status(person, club);
    SCOREBOARD.record(r as u8 == n as u8, || format!("shortlist_manager_estimate_squad_status({:#x}, {:#x}) original={:#x} rust={}", person, club, r, n));
    r
}

/// Comparer for `shortlist_manager_release_club_slot_targets` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_shortlist_manager_release_club_slot_targets(this: usize, club: usize, entry: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_SHORTLIST_MANAGER_RELEASE_CLUB_SLOT_TARGETS)(this, club, entry)
}

/// Comparer for `passes_work_permit_check` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a 0x8c7950 (sem sorteio pedido) pode mexer no sorteio: lado a lado pelo estado dele, a cópia em
/// sem_contar.
pub(crate) unsafe extern "stdcall" fn comparar_passes_work_permit_check(player: usize, current: usize, buyer: usize) -> u8 {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, usize, usize) -> u8>(IDX_PASSES_WORK_PERMIT_CHECK);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&crate::rng::STATE_REGIONS, || o(player, current, buyer),
        || crate::rng::without_counting(|| passes_work_permit_check(player, current, buyer)));
    SCOREBOARD.record(igual && r == n, || format!("passes_work_permit_check({:x}, {:x}) original={} rust={} {:x?} x {:x?}", player, buyer, r, n, mo, mr));
    r
}

/// Comparer for `try_loan_offer_for_player` (comparison mode: see the header of motor.rs).
///
/// The copy as a dry run and the original routine from the same state ([0xae1080], the random generator and the statics
/// of the date operations).
///
/// The copy refused (−1): the original too, with the same memory. The copy would update the list (−3): the original
/// returns −1. The copy would make the offer (−2): the original does not return −1.
///
/// [pt] O comparador: a cópia em ensaio e a original a partir do mesmo estado ([0xae1080], o sorteio e as estáticas das
/// operações de data). A cópia recusou (−1): a original também, com a mesma memória. A cópia atualizaria a lista (−3):
/// a original devolve −1. A cópia faria a oferta (−2): a original não devolve −1.
pub(crate) unsafe extern "thiscall" fn comparar_try_loan_offer_for_player(this: usize, player: usize, club: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> i32>(IDX_TRY_LOAN_OFFER_FOR_PLAYER);
    let mut regions = vec![(G_CONTRACT_LOOKUP_RESULT, 4usize)];
    regions.extend(crate::rng::STATE_REGIONS);
    regions.extend(TRANSFER_MANAGER_DATE_STATICS);
    let before = crate::comparison::MemorySnapshot::read(&regions);
    LOAN_OFFER_DRY_RUN = true;
    let n = crate::rng::pause_trace(|| crate::rng::without_counting(|| try_loan_offer_for_player(this, player, club)));
    LOAN_OFFER_DRY_RUN = false;
    let mr = crate::comparison::MemorySnapshot::read(&regions);
    before.write_back();
    let r = o(this, player, club);
    let mo = crate::comparison::MemorySnapshot::read(&regions);
    let same = match n {
        -1 => r == -1 && mo == mr,
        -3 => r == -1,
        _ => r != -1,
    };
    SCOREBOARD.record(same, || format!("try_loan_offer_for_player(jogador {:#x}, clube {:#x}) original={} {:x?} rust={} {:x?}", read_usize(player), read_usize(club), r, mo, n, mr));
    r
}

/// Comparer for `clear_bid_markers_for_player` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_clear_bid_markers_for_player(manager: usize, club: *const i32, player: *const i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *const i32, *const i32)>(IDX_CLEAR_BID_MARKERS_FOR_PLAYER);
    let l = club_shortlist(manager, club);
    let reg = if *((l + 4) as *const u8) == 1 { vec![(core::ptr::read_unaligned(l as *const usize), 40 * 0x25)] } else { vec![(l, 9)] };
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(manager, club, player), || clear_bid_markers_for_player(manager, club, player));
    SCOREBOARD.record(igual, || format!("clear_bid_markers_for_player({}, {}) {:x?} x {:x?}", *club, *player, mo, mr));
}

/// Comparer for `collect_club_shortlist_entries` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: as entradas da cópia antes; a original acrescenta; confere os últimos itens da lista (8 bytes cada).
pub(crate) unsafe extern "thiscall" fn comparar_collect_club_shortlist_entries(this: usize, club: *const u32, list: usize) {
    let expected: Vec<[u8; 8]> = club_shortlist_entries(this, *club).iter().map(|&e| core::ptr::read_unaligned(e as *const [u8; 8])).collect();
    let before = list_items_capped(list).len();
    original_fn::<unsafe extern "thiscall" fn(usize, *const u32, usize)>(IDX_COLLECT_CLUB_SHORTLIST_ENTRIES)(this, club, list);
    let new: Vec<[u8; 8]> = list_items_capped(list).iter().skip(before).map(|&i| core::ptr::read_unaligned(i as *const [u8; 8])).collect();
    SCOREBOARD.record(new == expected, || format!("collect_club_shortlist_entries({}) original {} itens, rust {}", *club, new.len(), expected.len()));
}

/// Comparer for `shortlist_manager_max_fee_for_squad_status` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "stdcall" fn comparar_shortlist_manager_max_fee_for_squad_status(club: usize, positions: u32, kind: u32) -> i32 {
    let r = original_fn::<unsafe extern "stdcall" fn(usize, u32, u32) -> i32>(IDX_SHORTLIST_MANAGER_MAX_FEE_FOR_SQUAD_STATUS)(club, positions, kind);
    let n = shortlist_manager_max_fee_for_squad_status(club, positions, kind);
    SCOREBOARD.record(r == n, || format!("shortlist_manager_max_fee_for_squad_status(tipo {}) original={} rust={}", kind as i8, r, n));
    r
}

/// Comparer for `update_club_target_reputation` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_update_club_target_reputation(this: usize, club: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> i32>(IDX_UPDATE_CLUB_TARGET_REPUTATION);
    let reg = [(shopping_list_entry(this, club) + 5, 2)];
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, club), || update_club_target_reputation(this, club));
    SCOREBOARD.record(igual && r == n, || format!("update_club_target_reputation({}) original={} rust={} {:x?} x {:x?}", *(club as *const i32), r, n, mo, mr));
    r
}

/// Comparer for `shortlist_slot_fits` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "stdcall" fn comparar_shortlist_slot_fits(j: usize, slot: i8) -> u8 {
    let r = original_fn::<unsafe extern "stdcall" fn(usize, i8) -> u8>(IDX_SHORTLIST_SLOT_FITS)(j, slot);
    let n = shortlist_slot_fits(j, slot);
    SCOREBOARD.record(r == n, || format!("shortlist_slot_fits({:#x}, {}) original={} rust={}", j, slot, r, n));
    r
}

/// Comparer for `position_code` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "stdcall" fn comparar_position_code(m: i16) -> u8 {
    let r = original_fn::<unsafe extern "stdcall" fn(i16) -> u8>(IDX_POSITION_CODE)(m);
    let n = position_code(m);
    SCOREBOARD.record(r == n, || format!("position_code({:#x}) original={:#x} rust={:#x}", m, r, n));
    r
}

/// Comparer for `shortlist_entry` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_shortlist_entry(this: usize, person: usize, target: usize, not_11: u32, tambem_13: u32) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize>(IDX_SHORTLIST_ENTRY)(this, person, target, not_11, tambem_13);
    let v = shortlist_entry(this, person, target, not_11, tambem_13);
    SCOREBOARD.record(r == v, || format!("shortlist_entry({}, {}) original={:#x} rust={:#x}", not_11, tambem_13, r, v));
    r
}
