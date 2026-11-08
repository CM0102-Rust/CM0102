use super::*;

// The comparers of the routines in ../tactics.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../tactics.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `stored_taker` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_stored_taker(this: usize, clube: usize, kind: u32, side: u32, human: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) -> usize>(IDX_STORED_TAKER);
    let (t, l) = (kind as u8 as i8, side as u8 as i8);
    if is_recording_top_level() || clube == 0 || !(0..=4).contains(&t) || !(0..=1).contains(&l) {
        return o(this, clube, kind, side, human);
    }
    let row: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(FN_TACTICS_GET_CLUB_TACTICS_INDEX);
    let k = row(clube, human);
    let mut reg = vec![(0xb67a34, 4)];
    if k != -1 {
        reg.push(((*((this + 0x601) as *const usize)).wrapping_add(k as usize * 0x91 + 0x5e + (l as i32 + t as i32 * 2) as usize * 4), 4));
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube, kind, side, human), || stored_taker(this, clube, kind, side, human));
    PLACAR.registrar(igual && r == n, || format!("stored_taker(tipo {} lado {}) original={:#x} rust={:#x} {:x?} x {:x?}", t, l, r, n, mo, mr));
    r
}

/// Comparer for `store_instructions` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_store_instructions(this: usize, clube: usize, person: u32, usar: u32, volta: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) -> u32>(IDX_STORE_INSTRUCTIONS);
    if is_recording_top_level() || clube == 0 {
        return o(this, clube, person, usar, volta);
    }
    let row: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(FN_TACTICS_GET_CLUB_TACTICS_INDEX);
    let k = row(clube, person);
    // os 16 espacos e o contador, e a linha de instrucoes do clube
    let mut reg = vec![(this + 0x30, 0x5c * 16), (this + 0x5f0, 1)];
    if k != -1 {
        reg.push(((*((this + 0x601) as *const usize)).wrapping_add(k as usize * 0x91 + 2), 80));
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube, person, usar, volta), || store_instructions(this, clube, person, usar, volta));
    PLACAR.registrar(igual && r == n, || format!("store_instructions(usar {} volta {}) original={} rust={} {:x?} x {:x?}", usar, volta, r, n, mo, mr));
    r
}

/// Comparer for `read_tactics_row` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_read_tactics_row(this: usize, clube: usize, target: *mut u8, human: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut u8, u32) -> u32>(IDX_READ_TACTICS_ROW);
    if is_recording_top_level() || clube == 0 {
        return o(this, clube, target, human);
    }
    let reg = tactics_row_regions(this, clube, target as usize, human);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube, target, human), || read_tactics_row(this, clube, target, human));
    PLACAR.registrar(igual && r == n, || format!("read_tactics_row(humano {}) original={} rust={} {:x?} x {:x?}", human, r, n, mo, mr));
    r
}

/// Comparer for `write_tactics_row` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_write_tactics_row(this: usize, clube: usize, source: *const u8, human: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *const u8, u32) -> u32>(IDX_WRITE_TACTICS_ROW);
    if is_recording_top_level() || clube == 0 {
        return o(this, clube, source, human);
    }
    let reg = tactics_row_regions(this, clube, source as usize, human);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube, source, human), || write_tactics_row(this, clube, source, human));
    PLACAR.registrar(igual && r == n, || format!("write_tactics_row(humano {}) original={} rust={} {:x?} x {:x?}", human, r, n, mo, mr));
    r
}

/// Comparer for `record_tactic_result` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x896290: lado a lado nas linhas das táticas dos dois clubes (0x28 bytes cada) e na devolução; o
/// registro da partida é local. Jogo ou clube nulos: a caixa, só a original.
pub(crate) unsafe extern "thiscall" fn comparar_record_tactic_result(this: usize, jogo: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_RECORD_TACTIC_RESULT);
    if jogo == 0 || read_usize(jogo + 0x1c) == 0 || read_usize(jogo + 0x20) == 0 {
        return o(this, jogo);
    }
    let reg = [(tactic_stats_row(this, read_usize(jogo + 0x1c)), 0x28), (tactic_stats_row(this, read_usize(jogo + 0x20)), 0x28)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, jogo), || record_tactic_result(this, jogo));
    PLACAR.registrar(igual && r == n, || format!("record_tactic_result(jogo {:x}) original={} rust={} {:x?} x {:x?}", jogo, r, n, mo, mr));
    r
}

/// Comparer for `get_captain` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x8976a0: pela devolução (só lê; a linha nula ou fora da faixa mostra a caixa nas duas).
pub(crate) unsafe extern "thiscall" fn comparar_get_captain(this: usize, clube: usize, team: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_GET_CAPTAIN);
    if clube == 0 {
        return o(this, clube, team);
    }
    let r = o(this, clube, team);
    let n = get_captain(this, clube, team);
    PLACAR.registrar(r == n, || format!("get_captain(clube {:x}, time {}) original={:x} rust={:x}", clube, team, r, n));
    r
}

/// Comparer for `set_captain` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x897880: lado a lado no dword do capitão da linha (achada antes pela 0x8982b0, que só lê) e na
/// devolução. Clube ou ponteiro nulos, ou sem linha: só a original.
pub(crate) unsafe extern "thiscall" fn comparar_set_captain(this: usize, clube: usize, person: usize, team: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32>(IDX_SET_CAPTAIN);
    if clube == 0 || person == 0 {
        return o(this, clube, person, team);
    }
    let row: unsafe extern "thiscall" fn(usize, usize, u32) -> i32 = core::mem::transmute(FN_TACTICS_GET_CLUB_TACTICS_INDEX);
    let k = row(this, clube, team);
    if k == -1 {
        return o(this, clube, person, team);
    }
    let p = read_usize(this + 0x601).wrapping_add((k as isize * 0x91) as usize) + 0x52;
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(p, 4)], || o(this, clube, person, team), || set_captain(this, clube, person, team));
    PLACAR.registrar(igual && r == n, || format!("set_captain(clube {:x}, time {}) original={} rust={} {:x?} x {:x?}", clube, team, r, n, mo, mr));
    r
}

/// Comparer for `remove_player_from_selection` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pela devolução, pela linha do clube na tabela tática (as vagas, as extras e os batedores) e
/// pelas cópias para desfazer com a conta. Clube ou pessoa nulos (caixa): só a original.
pub(crate) unsafe extern "thiscall" fn comparar_remove_player_from_selection(this: usize, clube: usize, person: usize, team: u32, papeis: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32>(IDX_REMOVE_PLAYER_FROM_SELECTION);
    if clube == 0 || person == 0 {
        return o(this, clube, person, team, papeis);
    }
    let reg = lineup_regions(this, clube, team);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube, person, team, papeis), || remove_player_from_selection(this, clube, person, team, papeis));
    PLACAR.registrar(igual && r == n, || format!("remove_player_from_selection(clube {:#x}, pessoa {:#x}, time {}, papeis {}) original={} {:x?} rust={} {:x?}", clube, person, team, papeis, r, mo, n, mr));
    r
}

/// Comparer for `swap_selection_slots` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pela devolução, pela linha tática do clube e pelas cópias para desfazer. Clube nulo ou vaga
/// fora (caixa): só a original.
pub(crate) unsafe extern "thiscall" fn comparar_swap_selection_slots(this: usize, clube: usize, a: u32, b: u32, team: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) -> u32>(IDX_SWAP_SELECTION_SLOTS);
    let (va, vb) = (a as u8 as i8, b as u8 as i8);
    if clube == 0 || !(0..=0x13).contains(&va) || !(0..=0x13).contains(&vb) {
        return o(this, clube, a, b, team);
    }
    let reg = lineup_regions(this, clube, team);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube, a, b, team), || swap_selection_slots(this, clube, a, b, team));
    PLACAR.registrar(igual && r == n, || format!("swap_selection_slots(clube {:#x}, {}, {}, time {}) original={} {:x?} rust={} {:x?}", clube, va, vb, team, r, mo, n, mr));
    r
}

/// Comparer for `has_selection_undo` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a devolução (rotina pura).
pub(crate) unsafe extern "thiscall" fn comparar_has_selection_undo(this: usize, clube: usize, team: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_HAS_SELECTION_UNDO)(this, clube, team);
    let n = has_selection_undo(this, clube, team);
    PLACAR.registrar(r == n, || format!("has_selection_undo(clube {:#x}, time {}) original={} rust={}", clube, team, r, n));
    r
}

/// Comparer for `clear_selection_slot` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: só grava a vaga e os espaços guardados: lado a lado pela linha da escalação (20 dwords), pelos 16
/// espaços e pela conta (+0x5f0). Clube nulo ou vaga fora: só a original (a caixa).
pub(crate) unsafe extern "thiscall" fn comparar_clear_selection_slot(this: usize, clube: usize, slot: u32, team: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32>(IDX_CLEAR_SELECTION_SLOT);
    let v = slot as u8 as i8;
    if clube == 0 || !(0..=0x13).contains(&v) {
        return o(this, clube, slot, team);
    }
    let mut reg = vec![(this + 0x30, 16 * 0x5c), (this + 0x5f0, 1)];
    let k = club_tactics_row(clube, team);
    if k != -1 {
        reg.push((tactics_cell(this, k, 2), 20 * 4));
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube, slot, team), || clear_selection_slot(this, clube, slot, team));
    PLACAR.registrar(igual && r == n, || format!("clear_selection_slot(vaga {}, time {}) original={} rust={} {:x?} x {:x?}", v, team, r, n, mo, mr));
    r
}

/// Comparer for `set_set_piece_taker` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: só grava a linha dos batedores: lado a lado pelos 10 dwords (+0x5e) da linha do clube. Clube nulo,
/// papel ou escolha fora: só a original (a caixa).
pub(crate) unsafe extern "thiscall" fn comparar_set_set_piece_taker(this: usize, clube: usize, role: u32, choice: u32, person: usize, team: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, u32) -> u32>(IDX_SET_SET_PIECE_TAKER);
    let (p, e) = (role as u8 as i8, choice as u8 as i8);
    if clube == 0 || !(0..=4).contains(&p) || !(0..=1).contains(&e) {
        return o(this, clube, role, choice, person, team);
    }
    let k = club_tactics_row(clube, team);
    let reg = if k != -1 { vec![(tactics_cell(this, k, 0x5e), 10 * 4)] } else { Vec::new() };
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube, role, choice, person, team),
        || set_set_piece_taker(this, clube, role, choice, person, team));
    PLACAR.registrar(igual && r == n, || format!("set_set_piece_taker(papel {}, escolha {}, pessoa {:x}) original={} rust={} {:x?} x {:x?}", p, e, person, r, n, mo, mr));
    r
}

/// Comparer for `extra_tactic_slot` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_extra_tactic_slot(t: usize, clube: usize, qual: i8, person: *const u32, human: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i8, *const u32, u32) -> u32>(IDX_EXTRA_TACTIC_SLOT);
    let reg = regioes_da_tabela(t, clube, human);
    let (r, v, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(t, clube, qual, person, human), || extra_tactic_slot(t, clube, qual, person, human));
    PLACAR.registrar(igual && r == v, || format!("extra_tactic_slot({:#x}, {:#x}, {}, {}) original={} {:x?} rust={} {:x?}", t, clube, qual, human, r, mo, v, mr));
    r
}

/// Comparer for `get_club_tactic` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pela devolução e, havendo técnico humano no clube, pelo byte da vaga ativa dele (a 0x5fbe20
/// grava). Clube nulo (caixa): só a original.
pub(crate) unsafe extern "thiscall" fn comparar_get_club_tactic(this: usize, clube: usize, team: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(IDX_GET_CLUB_TACTIC);
    if clube == 0 {
        return o(this, clube, team);
    }
    let manager = read_usize(G_HUMAN_MANAGER);
    let t = read_usize(clube + club::MANAGER);
    let mut reg = Vec::new();
    if manager != 0 && t != 0 {
        if let Some(k) = human_manager_human_manager_index(manager, t) {
            reg.push((human_manager_record_at(manager, k) + 0x9a69, 1));
        }
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube, team), || get_club_tactic(this, clube, team));
    PLACAR.registrar(igual && r == n, || format!("get_club_tactic(clube {:#x}, time {}) original={:#x} {:x?} rust={:#x} {:x?}", clube, team, r, mo, n, mr));
    r
}

/// Comparer for `find_selection_slot` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_find_selection_slot(tactic: usize, clube: usize, person: *const i32, team: u32) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, *const i32, u32) -> u8>(IDX_FIND_SELECTION_SLOT)(tactic, clube, person, team);
    if clube == 0 || person.is_null() {
        return r;
    }
    let n = find_selection_slot(tactic, clube, person, team);
    PLACAR.registrar(r == n, || format!("find_selection_slot(time {}) original={} rust={}", team, r, n));
    r
}

/// Comparer for `set_tactic_slot` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_set_tactic_slot(t: usize, clube: usize, slot: i8, person: *const u32, human: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i8, *const u32, u32) -> u32>(IDX_SET_TACTIC_SLOT);
    let reg = regioes_da_tabela(t, clube, human);
    let (r, v, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(t, clube, slot, person, human), || set_tactic_slot(t, clube, slot, person, human));
    PLACAR.registrar(igual && r == v, || format!("set_tactic_slot({:#x}, {:#x}, {}, {}) original={} {:x?} rust={} {:x?}", t, clube, slot, human, r, mo, v, mr));
    r
}

/// Comparer for `tactic_slot_person` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_tactic_slot_person(t: usize, clube: usize, slot: i8, human: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i8, u32) -> usize>(IDX_TACTIC_SLOT_PERSON);
    let reg = regioes_da_tabela(t, clube, human);
    let (r, v, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(t, clube, slot, human), || tactic_slot_person(t, clube, slot, human));
    PLACAR.registrar(igual && r == v, || format!("tactic_slot_person({:#x}, {:#x}, {}, {}) original={:#x} {:x?} rust={:#x} {:x?}", t, clube, slot, human, r, mo, v, mr));
    r
}

/// Comparer for `set_tactic_formation` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_set_tactic_formation(t: usize, clube: usize, formacao: i16, human: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i16, u32) -> u32>(IDX_SET_TACTIC_FORMATION);
    let reg = regioes_da_tabela(t, clube, human);
    let (r, v, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(t, clube, formacao, human), || set_tactic_formation(t, clube, formacao, human));
    PLACAR.registrar(igual && r == v, || format!("set_tactic_formation({:#x}, {:#x}, {}, {}) original={} {:x?} rust={} {:x?}", t, clube, formacao, human, r, mo, v, mr));
    r
}

/// Comparer for `marked_slot` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_marked_slot(this: usize, clube: usize, human: u32, minus_one: u32) -> i8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32) -> i8>(IDX_MARKED_SLOT)(this, clube, human, minus_one);
    let v = marked_slot(this, clube, human, minus_one);
    PLACAR.registrar(r == v, || format!("marked_slot original={} rust={}", r, v));
    r
}

/// Comparer for `mark_slot` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_mark_slot(this: usize, clube: usize, slot: i8, human: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i8, u32) -> u32>(IDX_MARK_SLOT);
    if is_recording_top_level() {
        return call_recording(FN_TACTICS_SET_CAPTAIN_FROM_SLOT, this, [clube as u32, slot as u8 as u32, human, 0, 0], || o(this, clube, slot, human));
    }
    let table = *((this + 0x601) as *const usize);
    // a linha pode ser qualquer uma da tabela: compara as 0x52..0x56 de todas as linhas possiveis
    let row: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(FN_TACTICS_GET_CLUB_TACTICS_INDEX);
    let l = row(clube, human);
    let reg: Vec<(usize, usize)> = if l == -1 || clube == 0 { vec![] } else { vec![(table.wrapping_add((l as usize).wrapping_mul(0x91)) + 0x52, 4)] };
    let (r, v, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube, slot, human), || mark_slot(this, clube, slot, human));
    PLACAR.registrar(igual && r == v, || format!("mark_slot original={} {:x?} rust={} {:x?}", r, mo, v, mr));
    r
}

/// Comparer for `tactics_row` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "stdcall" fn comparar_tactics_row(clube: usize, human: u32) -> i32 {
    let r = original_fn::<unsafe extern "stdcall" fn(usize, u32) -> i32>(IDX_TACTICS_ROW)(clube, human);
    let v = tactics_row(clube, human);
    PLACAR.registrar(r == v, || format!("tactics_row({:#x}, {}) original={} rust={}", clube, human, r, v));
    r
}
