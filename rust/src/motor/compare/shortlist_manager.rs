use super::*;

// The comparers of the routines in ../shortlist_manager.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../shortlist_manager.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `make_club_bids` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x84a020: só a original (sorteia, faz lances e pedidos de empréstimo, aloca ofertas e termos).
pub(crate) unsafe extern "thiscall" fn comparar_make_club_bids(this: usize, clube: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_MAKE_CLUB_BIDS)(this, clube) // simplification: só a original
}

/// Comparer for `staff_interest_in_joining_club` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado na nota (os bits do f64), nos motivos e em [0xae1080] (que a original e o salário padrão
/// 0x864550 gravam); o resto das chamadas só lê, e o STAFF_CONTRACT do salário é local.
pub(crate) unsafe extern "thiscall" fn comparar_staff_interest_in_joining_club(this: usize, p1: usize, p2: usize, p3: u32, p4: u32, p5: u32, p6: *mut u32) -> f64 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, *mut u32) -> f64>(IDX_STAFF_INTEREST_IN_JOINING_CLUB);
    let mut regioes = vec![(G_CONTRACT_LOOKUP_RESULT, 4usize)];
    if !p6.is_null() {
        regioes.push((p6 as usize, 4));
    }
    let (ro, rr, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, p1, p2, p3, p4, p5, p6), || staff_interest_in_joining_club(this, p1, p2, p3, p4, p5, p6));
    PLACAR.registrar(igual && ro.to_bits() == rr.to_bits(), || format!("staff_interest_in_joining_club(pessoa {:#x}, clube {:#x}, {}, {}, {}) original={} {:x?} rust={} {:x?}", p1, p2, p3, p4 as u8, p5 as u8, ro, mo, rr, mr));
    ro
}

/// Comparer for `shortlist_manager_is_compensation_deal` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado (rotina pura: só a taxa e a devolução), pelas cópias leves.
pub(crate) unsafe extern "thiscall" fn comparar_shortlist_manager_is_compensation_deal(this: usize, vaga: usize, contrato: usize, clube: usize, taxa: *mut i32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut i32) -> u32>(IDX_SHORTLIST_MANAGER_IS_COMPENSATION_DEAL);
    let reg = [(taxa as usize, 4)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, vaga, contrato, clube, taxa),
        || shortlist_manager_is_compensation_deal(this, vaga, contrato, clube, taxa));
    PLACAR.registrar(igual && r == n, || format!("shortlist_manager_is_compensation_deal(vaga {:#x}, contrato {:#x}, clube {:#x}) original={} rust={} {:x?} x {:x?}",
        vaga, contrato, clube, r, n, mo, mr));
    r
}

/// Comparer for `shortlist_compare_slots_by_index` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a devolução (pura).
pub(crate) unsafe extern "C" fn comparar_shortlist_compare_slots_by_index(a: usize, b: usize) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_SHORTLIST_COMPARE_SLOTS_BY_INDEX);
    let r = o(a, b);
    let n = shortlist_compare_slots_by_index(a, b);
    PLACAR.registrar(r == n, || format!("shortlist_compare_slots_by_index({:#x}, {:#x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `mark_player_under_offer` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelas listas de 40 vagas dos clubes interessados (a cópia e a original montam e soltam a
/// mesma lista local) e pelo cursor da lista `troca`.
pub(crate) unsafe extern "thiscall" fn comparar_mark_player_under_offer(this: usize, jogador: usize, comprador: usize, troca: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(IDX_MARK_PLAYER_UNDER_OFFER);
    let mut reg: Vec<(usize, usize)> = interested_clubs(this, jogador).into_iter()
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

/// Comparer for `value_player_for_club` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x840d80: lado a lado na nota de saída e na devolução (o resto ela só lê). O técnico do clube sem a
/// ficha +0x69 (a original sorteia uma): só a original.
pub(crate) unsafe extern "thiscall" fn comparar_value_player_for_club(this: usize, clube: usize, p: usize, vaga: usize, saida: *mut i32, minimo: i32, teto: u32) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut i32, i32, u32) -> i32>(IDX_VALUE_PLAYER_FOR_CLUB);
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

/// Comparer for `count_slots_with_bids` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelas 40 vagas do clube e pelas datas estáticas, e a devolução.
pub(crate) unsafe extern "thiscall" fn comparar_count_slots_with_bids(this: usize, clube: usize, posicao: u32) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u8>(IDX_COUNT_SLOTS_WITH_BIDS);
    let mut reg = SHORTLIST_MANAGER_DATE_STATICS.to_vec();
    reg.push((read_usize(shortlist_club_entry(this, clube)), 40 * 0x25));
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube, posicao), || count_slots_with_bids(this, clube, posicao));
    PLACAR.registrar(igual && r == n, || format!("count_slots_with_bids(clube {:x}, posicao {}) original={} rust={} {:x?} x {:x?}", clube, posicao as i8, r, n, mo, mr));
    r
}

/// Comparer for `shortlist_manager_remove_from_transfer_list` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelo byte +0x4e do contrato, e a devolução.
pub(crate) unsafe extern "stdcall" fn comparar_shortlist_manager_remove_from_transfer_list(pessoa: usize) -> u32 {
    let o = original_fn::<unsafe extern "stdcall" fn(usize) -> u32>(IDX_SHORTLIST_MANAGER_REMOVE_FROM_TRANSFER_LIST);
    let c = contract_of_staff(pessoa);
    let reg = if c != 0 { vec![(c + 0x4e, 1)] } else { Vec::new() };
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(pessoa), || shortlist_manager_remove_from_transfer_list(pessoa));
    PLACAR.registrar(igual && r == n, || format!("shortlist_manager_remove_from_transfer_list(pessoa {:x}) original={:#x} rust={:#x} {:x?} x {:x?}", pessoa, r, n, mo, mr));
    r
}

/// Comparer for `shortlist_manager_remove_from_loan_list` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelo byte +0x4e do contrato, e a devolução.
pub(crate) unsafe extern "stdcall" fn comparar_shortlist_manager_remove_from_loan_list(pessoa: usize) -> u32 {
    let o = original_fn::<unsafe extern "stdcall" fn(usize) -> u32>(IDX_SHORTLIST_MANAGER_REMOVE_FROM_LOAN_LIST);
    let c = contract_of_staff(pessoa);
    let reg = if c != 0 { vec![(c + 0x4e, 1)] } else { Vec::new() };
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(pessoa), || shortlist_manager_remove_from_loan_list(pessoa));
    PLACAR.registrar(igual && r == n, || format!("shortlist_manager_remove_from_loan_list(pessoa {:x}) original={:#x} rust={:#x} {:x?} x {:x?}", pessoa, r, n, mo, mr));
    r
}

/// Comparer for `update_staff_status` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelos dois vetores de pares e suas contas, pelo byte da conta de listas da pessoa, pelas 40
/// vagas e o +8 da entrada do clube, pelo byte +0x4f do contrato e pelas datas estáticas, e a devolução. Na recursão do
/// evento 4 e no evento 1 que preenche a vaga, só a original (a cópia leva os ramos).
pub(crate) unsafe extern "thiscall" fn comparar_update_staff_status(this: usize, pessoa: usize, clube: usize, evento: u32, tipo: u32) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u8>(IDX_UPDATE_STAFF_STATUS);
    if is_original_only_case(this, pessoa, clube, evento, tipo) {
        return o(this, pessoa, clube, evento, tipo);
    }
    let mut reg = STAFF_STATUS_DATE_STATICS.to_vec();
    for base in [this, this + 4] {
        reg.push((base + 0x10, 4));
        if read_usize(base) != 0 {
            reg.push((read_usize(base), read_usize(base + 0x10).wrapping_mul(8)));
        }
    }
    reg.push((read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(pessoa).wrapping_mul(0x4f)) + 0xb, 1));
    if clube != 0 {
        let entrada = shortlist_club_entry(this, clube);
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

/// Comparer for `shortlist_manager_add_to_loan_list` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelo byte +0x4e do contrato, e a devolução.
pub(crate) unsafe extern "stdcall" fn comparar_shortlist_manager_add_to_loan_list(pessoa: usize) -> usize {
    let o = original_fn::<unsafe extern "stdcall" fn(usize) -> usize>(IDX_SHORTLIST_MANAGER_ADD_TO_LOAN_LIST);
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = f(read_usize(G_CONTRACT_MANAGER), pessoa);
    let reg = if c != 0 { vec![(c + 0x4e, 1)] } else { Vec::new() };
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(pessoa), || shortlist_manager_add_to_loan_list(pessoa));
    PLACAR.registrar(igual && r == n, || format!("shortlist_manager_add_to_loan_list(pessoa {:x}) original={:#x} rust={:#x} {:x?} x {:x?}", pessoa, r, n, mo, mr));
    r
}

/// Comparer for `shortlist_manager_max_fee_for_player_status` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a devolução (rotina pura).
pub(crate) unsafe extern "thiscall" fn comparar_shortlist_manager_max_fee_for_player_status(this: usize, clube: usize, pessoa: usize, estatuto: u32) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32) -> i32>(IDX_SHORTLIST_MANAGER_MAX_FEE_FOR_PLAYER_STATUS);
    let r = o(this, clube, pessoa, estatuto);
    let n = shortlist_manager_max_fee_for_player_status(this, clube, pessoa, estatuto);
    PLACAR.registrar(r == n, || format!("shortlist_manager_max_fee_for_player_status(clube {:x}, pessoa {:x}, estatuto {}) original={} rust={}", clube, pessoa, estatuto as i8, r, n));
    r
}

/// Comparer for `allocate_transfer_budget` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_allocate_transfer_budget(this: usize, clube: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_ALLOCATE_TRANSFER_BUDGET)(this, clube)
}

/// Comparer for `shortlist_manager_squad_status_allowed` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a devolução (rotina pura); sem ficha de jogador, só a original (caixa).
pub(crate) unsafe extern "stdcall" fn comparar_shortlist_manager_squad_status_allowed(pessoa: usize, estatuto: u32) -> u32 {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, u32) -> u32>(IDX_SHORTLIST_MANAGER_SQUAD_STATUS_ALLOWED);
    if read_usize(pessoa + staff::PLAYER) == 0 {
        return o(pessoa, estatuto);
    }
    let r = o(pessoa, estatuto);
    let n = shortlist_manager_squad_status_allowed(pessoa, estatuto);
    PLACAR.registrar(r == n, || format!("shortlist_manager_squad_status_allowed(pessoa {:x}, estatuto {}) original={} rust={}", pessoa, estatuto as i8, r, n));
    r
}

/// Comparador: lado a lado pelas 40 vagas da entrada (sem chamadas).
pub(crate) unsafe extern "stdcall" fn comparar_shortlist_set_up_formation_slots(entrada: usize) {
    let o = original_fn::<unsafe extern "stdcall" fn(usize)>(IDX_SHORTLIST_SET_UP_FORMATION_SLOTS);
    let reg = [(read_usize(entrada), 40 * 0x25)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(entrada), || shortlist_set_up_formation_slots(entrada));
    PLACAR.registrar(igual, || format!("shortlist_set_up_formation_slots(formacao {}) primeira diferença {:x?}", *((entrada + 7) as *const i8), first_memory_diff(&mo, &mr)));
}

/// Comparer for `update_club_shortlist` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_update_club_shortlist(this: usize, clube: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_UPDATE_CLUB_SHORTLIST)(this, clube)
}

/// Comparer for `suspend_clubs_shortlisting_player` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelas entradas (9 bytes) e, com `limpar`, pelas vagas (40 × 0x25) dos clubes do plano: os
/// clubes que a 0x842180 (pura) junta antes, numa lista própria. Cada lado cria e libera a sua lista.
pub(crate) unsafe extern "thiscall" fn comparar_suspend_clubs_shortlisting_player(this: usize, jogador: usize, limpar: u8) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u8)>(IDX_SUSPEND_CLUBS_SHORTLISTING_PLAYER);
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
    PLACAR.registrar(igual, || format!("suspend_clubs_shortlisting_player(jogador {:x}, limpar {}, {} regiões) primeira diferença {:x?}", read_usize(jogador), limpar, reg.len(), first_memory_diff(&mo, &mr)));
}

/// Comparer for `add_player_to_club_shortlists` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_add_player_to_club_shortlists(this: usize, jogador: usize, forcar: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_ADD_PLAYER_TO_CLUB_SHORTLISTS)(this, jogador, forcar)
}

/// Comparer for `transfer_list_player` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: sorteia e dá notícia: roda só a original e confere o plano feito antes: sem listagem do clube e com
/// contrato, o contrato fica à venda (+0x4e bit 1, +0x4f nibble alto 7).
pub(crate) unsafe extern "thiscall" fn comparar_transfer_list_player(this: usize, jogador: usize, noticia: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_TRANSFER_LIST_PLAYER);
    let listada: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_STAFF_IS_TRANSFER_LISTED);
    let marca = listada(jogador, 0) == 0 && get_contract(jogador) != 0;
    o(this, jogador, noticia);
    if marca {
        let c = get_contract(jogador);
        let ok = c != 0 && read_u8(c + 0x4e) & 1 != 0 && read_u8(c + 0x4f) & 0xf0 == 0x70;
        PLACAR.registrar(ok, || format!("transfer_list_player(jogador {:x}) contrato {:x} sem a marca de à venda", read_usize(jogador), c));
    }
}

/// Comparer for `build_position_lists` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_build_position_lists(this: usize, nao_usado: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_BUILD_POSITION_LISTS)(this, nao_usado);
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
    PLACAR.registrar(ruim.is_none(), || format!("build_position_lists: vaga e entrada {:?}", ruim));
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
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this), || sort_position_lists(this));
    PLACAR.registrar(igual, || {
        let dif: Vec<usize> = mo.0.iter().zip(mr.0.iter()).filter(|(a, b)| a.1 != b.1).map(|(a, _)| a.0).take(8).collect();
        format!("sort_position_lists: vetores diferentes {:x?}", dif)
    });
}

/// Comparer for `collect_interested_clubs` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a original na lista de saída e a cópia numa lista nova (0x68d020, solta com 0x68d080); compara os pares.
pub(crate) unsafe extern "thiscall" fn comparar_collect_interested_clubs(this: usize, jogador: usize, saida: usize, ignorar: u32, modo: u32, so_humanos: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32)>(IDX_COLLECT_INTERESTED_CLUBS);
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
    let (a, b) = (llist_pairs(saida), llist_pairs(l));
    limpar(l);
    PLACAR.registrar(a == b, || format!("collect_interested_clubs({:#x}) original={:x?} rust={:x?}", jogador, a, b));
}

/// Comparer for `add_shortlist_entry` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_add_shortlist_entry(this: usize, pessoa: usize, clube: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_ADD_SHORTLIST_ENTRY);
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

/// Comparer for `shortlist_manager_predict_selling_club_response` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor (o byte baixo) e lado a lado pelo sorteio e [0xae1080] (as chamadas de preço podem sortear).
pub(crate) unsafe extern "stdcall" fn comparar_shortlist_manager_predict_selling_club_response(vendedor: usize, jogador: usize, oferta: i64, comprador: usize, pedido: i64, emprestimo: u32) -> u8 {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, usize, i64, usize, i64, u32) -> u8>(IDX_SHORTLIST_MANAGER_PREDICT_SELLING_CLUB_RESPONSE);
    let mut reg = vec![(G_CONTRACT_LOOKUP_RESULT, 4usize)];
    reg.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(
        &reg,
        || o(vendedor, jogador, oferta, comprador, pedido, emprestimo),
        || crate::aleatorio::sem_contar(|| shortlist_manager_predict_selling_club_response(vendedor, jogador, oferta, comprador, pedido, emprestimo)),
    );
    PLACAR.registrar(igual && r == n, || format!("shortlist_manager_predict_selling_club_response({:#x}, {:#x}, {}, {:#x}, {}, {}) original={} rust={} {:x?} x {:x?}", vendedor, jogador, oferta, comprador, pedido, emprestimo as u8, r, n, mo, mr));
    r
}

/// Comparer for `get_squad_status_for_club` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelo estatuto, pelo cargo de saída e por [0xae1080].
pub(crate) unsafe extern "thiscall" fn comparar_get_squad_status_for_club(this: usize, pessoa: usize, clube: usize, cargo: *mut u8) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, *mut u8) -> u8>(IDX_GET_SQUAD_STATUS_FOR_CLUB);
    let saida = if cargo.is_null() { G_CONTRACT_LOOKUP_RESULT } else { cargo as usize };
    let reg = [(G_CONTRACT_LOOKUP_RESULT, 4usize), (saida, 1)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, pessoa, clube, cargo), || get_squad_status_for_club(this, pessoa, clube, cargo));
    PLACAR.registrar(igual && r == n, || format!("get_squad_status_for_club({:#x}, {:#x}) original={} rust={} {:x?} x {:x?}", pessoa, clube, r, n, mo, mr));
    r
}

/// Comparer for `shortlist_manager_estimate_squad_status` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor (o byte baixo; a original deixa a palavra de estado do x87 em ah).
pub(crate) unsafe extern "stdcall" fn comparar_shortlist_manager_estimate_squad_status(pessoa: usize, clube: usize) -> u32 {
    let r = original_fn::<unsafe extern "stdcall" fn(usize, usize) -> u32>(IDX_SHORTLIST_MANAGER_ESTIMATE_SQUAD_STATUS)(pessoa, clube);
    let n = shortlist_manager_estimate_squad_status(pessoa, clube);
    PLACAR.registrar(r as u8 == n as u8, || format!("shortlist_manager_estimate_squad_status({:#x}, {:#x}) original={:#x} rust={}", pessoa, clube, r, n));
    r
}

/// Comparer for `shortlist_manager_release_club_slot_targets` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_shortlist_manager_release_club_slot_targets(this: usize, clube: usize, entrada: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_SHORTLIST_MANAGER_RELEASE_CLUB_SLOT_TARGETS)(this, clube, entrada)
}

/// Comparer for `passes_work_permit_check` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a 0x8c7950 (sem sorteio pedido) pode mexer no sorteio: lado a lado pelo estado dele, a cópia em
/// sem_contar.
pub(crate) unsafe extern "stdcall" fn comparar_passes_work_permit_check(jogador: usize, atual: usize, comprador: usize) -> u8 {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, usize, usize) -> u8>(IDX_PASSES_WORK_PERMIT_CHECK);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&crate::aleatorio::REGIOES_DO_ESTADO, || o(jogador, atual, comprador),
        || crate::aleatorio::sem_contar(|| passes_work_permit_check(jogador, atual, comprador)));
    PLACAR.registrar(igual && r == n, || format!("passes_work_permit_check({:x}, {:x}) original={} rust={} {:x?} x {:x?}", jogador, comprador, r, n, mo, mr));
    r
}

/// Comparer for `try_loan_offer_for_player` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador: a cópia em ensaio e a original a partir do mesmo estado ([0xae1080], o sorteio e as estáticas das
/// operações de data). A cópia recusou (−1): a original também, com a mesma memória. A cópia atualizaria a lista (−3):
/// a original devolve −1. A cópia faria a oferta (−2): a original não devolve −1.
pub(crate) unsafe extern "thiscall" fn comparar_try_loan_offer_for_player(this: usize, jogador: usize, clube: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> i32>(IDX_TRY_LOAN_OFFER_FOR_PLAYER);
    let mut regioes = vec![(G_CONTRACT_LOOKUP_RESULT, 4usize)];
    regioes.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    regioes.extend(TRANSFER_MANAGER_DATE_STATICS);
    let antes = crate::comparacao::Memoria::ler(&regioes);
    LOAN_OFFER_DRY_RUN = true;
    let n = crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| try_loan_offer_for_player(this, jogador, clube)));
    LOAN_OFFER_DRY_RUN = false;
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

/// Comparer for `clear_bid_markers_for_player` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_clear_bid_markers_for_player(gerente: usize, clube: *const i32, jogador: *const i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *const i32, *const i32)>(IDX_CLEAR_BID_MARKERS_FOR_PLAYER);
    let l = club_shortlist(gerente, clube);
    let reg = if *((l + 4) as *const u8) == 1 { vec![(core::ptr::read_unaligned(l as *const usize), 40 * 0x25)] } else { vec![(l, 9)] };
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(gerente, clube, jogador), || clear_bid_markers_for_player(gerente, clube, jogador));
    PLACAR.registrar(igual, || format!("clear_bid_markers_for_player({}, {}) {:x?} x {:x?}", *clube, *jogador, mo, mr));
}

/// Comparer for `collect_club_shortlist_entries` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: as entradas da cópia antes; a original acrescenta; confere os últimos itens da lista (8 bytes cada).
pub(crate) unsafe extern "thiscall" fn comparar_collect_club_shortlist_entries(this: usize, clube: *const u32, lista: usize) {
    let esperado: Vec<[u8; 8]> = club_shortlist_entries(this, *clube).iter().map(|&e| core::ptr::read_unaligned(e as *const [u8; 8])).collect();
    let antes = list_items_capped(lista).len();
    original_fn::<unsafe extern "thiscall" fn(usize, *const u32, usize)>(IDX_COLLECT_CLUB_SHORTLIST_ENTRIES)(this, clube, lista);
    let novos: Vec<[u8; 8]> = list_items_capped(lista).iter().skip(antes).map(|&i| core::ptr::read_unaligned(i as *const [u8; 8])).collect();
    PLACAR.registrar(novos == esperado, || format!("collect_club_shortlist_entries({}) original {} itens, rust {}", *clube, novos.len(), esperado.len()));
}

/// Comparer for `shortlist_manager_max_fee_for_squad_status` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "stdcall" fn comparar_shortlist_manager_max_fee_for_squad_status(clube: usize, posicoes: u32, tipo: u32) -> i32 {
    let r = original_fn::<unsafe extern "stdcall" fn(usize, u32, u32) -> i32>(IDX_SHORTLIST_MANAGER_MAX_FEE_FOR_SQUAD_STATUS)(clube, posicoes, tipo);
    let n = shortlist_manager_max_fee_for_squad_status(clube, posicoes, tipo);
    PLACAR.registrar(r == n, || format!("shortlist_manager_max_fee_for_squad_status(tipo {}) original={} rust={}", tipo as i8, r, n));
    r
}

/// Comparer for `update_club_target_reputation` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_update_club_target_reputation(this: usize, clube: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> i32>(IDX_UPDATE_CLUB_TARGET_REPUTATION);
    let reg = [(shopping_list_entry(this, clube) + 5, 2)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube), || update_club_target_reputation(this, clube));
    PLACAR.registrar(igual && r == n, || format!("update_club_target_reputation({}) original={} rust={} {:x?} x {:x?}", *(clube as *const i32), r, n, mo, mr));
    r
}

/// Comparer for `shortlist_slot_fits` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "stdcall" fn comparar_shortlist_slot_fits(j: usize, vaga: i8) -> u8 {
    let r = original_fn::<unsafe extern "stdcall" fn(usize, i8) -> u8>(IDX_SHORTLIST_SLOT_FITS)(j, vaga);
    let n = shortlist_slot_fits(j, vaga);
    PLACAR.registrar(r == n, || format!("shortlist_slot_fits({:#x}, {}) original={} rust={}", j, vaga, r, n));
    r
}

/// Comparer for `position_code` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "stdcall" fn comparar_position_code(m: i16) -> u8 {
    let r = original_fn::<unsafe extern "stdcall" fn(i16) -> u8>(IDX_POSITION_CODE)(m);
    let n = position_code(m);
    PLACAR.registrar(r == n, || format!("position_code({:#x}) original={:#x} rust={:#x}", m, r, n));
    r
}

/// Comparer for `shortlist_entry` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_shortlist_entry(this: usize, pessoa: usize, alvo: usize, nao_11: u32, tambem_13: u32) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize>(IDX_SHORTLIST_ENTRY)(this, pessoa, alvo, nao_11, tambem_13);
    let v = shortlist_entry(this, pessoa, alvo, nao_11, tambem_13);
    PLACAR.registrar(r == v, || format!("shortlist_entry({}, {}) original={:#x} rust={:#x}", nao_11, tambem_13, r, v));
    r
}
