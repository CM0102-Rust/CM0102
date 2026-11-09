use super::*;

// The comparers of the routines in ../tactics.rs.
// [pt] Os comparadores das rotinas de ../tactics.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `stored_taker`.
pub(crate) unsafe extern "thiscall" fn compare_stored_taker(this: usize, club: usize, kind: u32, side: u32, human: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) -> usize>(IDX_STORED_TAKER);
    let (t, l) = (kind as u8 as i8, side as u8 as i8);
    if is_recording_top_level() || club == 0 || !(0..=4).contains(&t) || !(0..=1).contains(&l) {
        return o(this, club, kind, side, human);
    }
    let row: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(FN_TACTICS_GET_CLUB_TACTICS_INDEX);
    let k = row(club, human);
    let mut reg = vec![(G_MOUSE_LEFT_BUTTON_DOWN, 4)];
    if k != -1 {
        reg.push(((*((this + tactics_fields::ROWS) as *const usize)).wrapping_add(k as usize * 0x91 + 0x5e + (l as i32 + t as i32 * 2) as usize * 4), 4));
    }
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, club, kind, side, human), || stored_taker(this, club, kind, side, human));
    SCOREBOARD.record(igual && r == n, || format!("stored_taker(tipo {} lado {}) original={:#x} rust={:#x} {:x?} x {:x?}", t, l, r, n, mo, mr));
    r
}

/// Comparer for `store_instructions`.
pub(crate) unsafe extern "thiscall" fn compare_store_instructions(this: usize, club: usize, person: u32, usar: u32, volta: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) -> u32>(IDX_STORE_INSTRUCTIONS);
    if is_recording_top_level() || club == 0 {
        return o(this, club, person, usar, volta);
    }
    let row: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(FN_TACTICS_GET_CLUB_TACTICS_INDEX);
    let k = row(club, person);
    // the 16 spaces and the counter, and the club’s instructions row
    let mut reg = vec![(this + tactics_fields::UNDO_SLOTS, 0x5c * 16), (this + tactics_fields::UNDO_COUNT, 1)];
    if k != -1 {
        reg.push(((*((this + tactics_fields::ROWS) as *const usize)).wrapping_add(k as usize * 0x91 + 2), 80));
    }
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, club, person, usar, volta), || store_instructions(this, club, person, usar, volta));
    SCOREBOARD.record(igual && r == n, || format!("store_instructions(usar {} volta {}) original={} rust={} {:x?} x {:x?}", usar, volta, r, n, mo, mr));
    r
}

/// Comparer for `read_tactics_row`.
pub(crate) unsafe extern "thiscall" fn compare_read_tactics_row(this: usize, club: usize, target: *mut u8, human: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut u8, u32) -> u32>(IDX_READ_TACTICS_ROW);
    if is_recording_top_level() || club == 0 {
        return o(this, club, target, human);
    }
    let reg = tactics_row_regions(this, club, target as usize, human);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, club, target, human), || read_tactics_row(this, club, target, human));
    SCOREBOARD.record(igual && r == n, || format!("read_tactics_row(humano {}) original={} rust={} {:x?} x {:x?}", human, r, n, mo, mr));
    r
}

/// Comparer for `write_tactics_row`.
pub(crate) unsafe extern "thiscall" fn compare_write_tactics_row(this: usize, club: usize, source: *const u8, human: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *const u8, u32) -> u32>(IDX_WRITE_TACTICS_ROW);
    if is_recording_top_level() || club == 0 {
        return o(this, club, source, human);
    }
    let reg = tactics_row_regions(this, club, source as usize, human);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, club, source, human), || write_tactics_row(this, club, source, human));
    SCOREBOARD.record(igual && r == n, || format!("write_tactics_row(humano {}) original={} rust={} {:x?} x {:x?}", human, r, n, mo, mr));
    r
}

/// Comparer for `record_tactic_result`.
///
/// [pt] Comparador da 0x896290: lado a lado nas linhas das táticas dos dois clubes (0x28 bytes cada) e na devolução; o
/// registro da partida é local. Jogo ou clube nulos: a caixa, só a original.
pub(crate) unsafe extern "thiscall" fn compare_record_tactic_result(this: usize, game: usize) -> u32 {
    let game = Fixture(game);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_RECORD_TACTIC_RESULT);
    if game.is_null() || game.home_club() == 0 || game.away_club() == 0 {
        return o(this, game.addr());
    }
    let reg = [(tactic_stats_row(this, game.home_club()), 0x28), (tactic_stats_row(this, game.away_club()), 0x28)];
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, game.addr()), || record_tactic_result(this, game.addr()));
    SCOREBOARD.record(igual && r == n, || format!("record_tactic_result(jogo {:x}) original={} rust={} {:x?} x {:x?}", game.addr(), r, n, mo, mr));
    r
}

/// Comparer for `get_captain`.
///
/// [pt] Comparador da 0x8976a0: pela devolução (só lê; a linha nula ou fora da faixa mostra a caixa nas duas).
pub(crate) unsafe extern "thiscall" fn compare_get_captain(this: usize, club: usize, team: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_GET_CAPTAIN);
    if club == 0 {
        return o(this, club, team);
    }
    let r = o(this, club, team);
    let n = get_captain(this, club, team);
    SCOREBOARD.record(r == n, || format!("get_captain(clube {:x}, time {}) original={:x} rust={:x}", club, team, r, n));
    r
}

/// Comparer for `set_captain`.
///
/// [pt] Comparador da 0x897880: lado a lado no dword do capitão da linha (achada antes pela 0x8982b0, que só lê) e na
/// devolução. Clube ou ponteiro nulos, ou sem linha: só a original.
pub(crate) unsafe extern "thiscall" fn compare_set_captain(this: usize, club: usize, person: usize, team: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32>(IDX_SET_CAPTAIN);
    if club == 0 || person == 0 {
        return o(this, club, person, team);
    }
    let k = exe::tactics_get_club_tactics_index(this, club, team);
    if k == -1 {
        return o(this, club, person, team);
    }
    let p = read_usize(this + tactics_fields::ROWS).wrapping_add((k as isize * 0x91) as usize) + 0x52;
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&[(p, 4)], || o(this, club, person, team), || set_captain(this, club, person, team));
    SCOREBOARD.record(igual && r == n, || format!("set_captain(clube {:x}, time {}) original={} rust={} {:x?} x {:x?}", club, team, r, n, mo, mr));
    r
}

/// Comparer for `remove_player_from_selection`.
///
/// [pt] Comparador: lado a lado pela devolução, pela linha do clube na tabela tática (as vagas, as extras e os batedores) e
/// pelas cópias para desfazer com a conta. Clube ou pessoa nulos (caixa): só a original.
pub(crate) unsafe extern "thiscall" fn compare_remove_player_from_selection(this: usize, club: usize, person: usize, team: u32, papeis: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32>(IDX_REMOVE_PLAYER_FROM_SELECTION);
    if club == 0 || person == 0 {
        return o(this, club, person, team, papeis);
    }
    let reg = lineup_regions(this, club, team);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, club, person, team, papeis), || remove_player_from_selection(this, club, person, team, papeis));
    SCOREBOARD.record(igual && r == n, || format!("remove_player_from_selection(clube {:#x}, pessoa {:#x}, time {}, papeis {}) original={} {:x?} rust={} {:x?}", club, person, team, papeis, r, mo, n, mr));
    r
}

/// Comparer for `swap_selection_slots`.
///
/// [pt] Comparador: lado a lado pela devolução, pela linha tática do clube e pelas cópias para desfazer. Clube nulo ou vaga
/// fora (caixa): só a original.
pub(crate) unsafe extern "thiscall" fn compare_swap_selection_slots(this: usize, club: usize, a: u32, b: u32, team: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) -> u32>(IDX_SWAP_SELECTION_SLOTS);
    let (va, vb) = (a as u8 as i8, b as u8 as i8);
    if club == 0 || !(0..=0x13).contains(&va) || !(0..=0x13).contains(&vb) {
        return o(this, club, a, b, team);
    }
    let reg = lineup_regions(this, club, team);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, club, a, b, team), || swap_selection_slots(this, club, a, b, team));
    SCOREBOARD.record(igual && r == n, || format!("swap_selection_slots(clube {:#x}, {}, {}, time {}) original={} {:x?} rust={} {:x?}", club, va, vb, team, r, mo, n, mr));
    r
}

/// Comparer for `has_selection_undo`.
///
/// [pt] Comparador: a devolução (rotina pura).
pub(crate) unsafe extern "thiscall" fn compare_has_selection_undo(this: usize, club: usize, team: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_HAS_SELECTION_UNDO)(this, club, team);
    let n = has_selection_undo(this, club, team);
    SCOREBOARD.record(r == n, || format!("has_selection_undo(clube {:#x}, time {}) original={} rust={}", club, team, r, n));
    r
}

/// Comparer for `clear_selection_slot`.
///
/// [pt] Comparador: só grava a vaga e os espaços guardados: lado a lado pela linha da escalação (20 dwords), pelos 16
/// espaços e pela conta (+0x5f0). Clube nulo ou vaga fora: só a original (a caixa).
pub(crate) unsafe extern "thiscall" fn compare_clear_selection_slot(this: usize, club: usize, slot: u32, team: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32>(IDX_CLEAR_SELECTION_SLOT);
    let v = slot as u8 as i8;
    if club == 0 || !(0..=0x13).contains(&v) {
        return o(this, club, slot, team);
    }
    let mut reg = vec![(this + tactics_fields::UNDO_SLOTS, 16 * 0x5c), (this + tactics_fields::UNDO_COUNT, 1)];
    let k = club_tactics_row(club, team);
    if k != -1 {
        reg.push((tactics_cell(this, k, 2), 20 * 4));
    }
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, club, slot, team), || clear_selection_slot(this, club, slot, team));
    SCOREBOARD.record(igual && r == n, || format!("clear_selection_slot(vaga {}, time {}) original={} rust={} {:x?} x {:x?}", v, team, r, n, mo, mr));
    r
}

/// Comparer for `set_set_piece_taker`.
///
/// [pt] Comparador: só grava a linha dos batedores: lado a lado pelos 10 dwords (+0x5e) da linha do clube. Clube nulo,
/// papel ou escolha fora: só a original (a caixa).
pub(crate) unsafe extern "thiscall" fn compare_set_set_piece_taker(this: usize, club: usize, role: u32, choice: u32, person: usize, team: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, u32) -> u32>(IDX_SET_SET_PIECE_TAKER);
    let (p, e) = (role as u8 as i8, choice as u8 as i8);
    if club == 0 || !(0..=4).contains(&p) || !(0..=1).contains(&e) {
        return o(this, club, role, choice, person, team);
    }
    let k = club_tactics_row(club, team);
    let reg = if k != -1 { vec![(tactics_cell(this, k, 0x5e), 10 * 4)] } else { Vec::new() };
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, club, role, choice, person, team),
        || set_set_piece_taker(this, club, role, choice, person, team));
    SCOREBOARD.record(igual && r == n, || format!("set_set_piece_taker(papel {}, escolha {}, pessoa {:x}) original={} rust={} {:x?} x {:x?}", p, e, person, r, n, mo, mr));
    r
}

/// Comparer for `extra_tactic_slot`.
pub(crate) unsafe extern "thiscall" fn compare_extra_tactic_slot(t: usize, club: usize, which: i8, person: *const u32, human: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i8, *const u32, u32) -> u32>(IDX_EXTRA_TACTIC_SLOT);
    let reg = table_regions(t, club, human);
    let (r, v, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(t, club, which, person, human), || extra_tactic_slot(t, club, which, person, human));
    SCOREBOARD.record(igual && r == v, || format!("extra_tactic_slot({:#x}, {:#x}, {}, {}) original={} {:x?} rust={} {:x?}", t, club, which, human, r, mo, v, mr));
    r
}

/// Comparer for `get_club_tactic`.
///
/// [pt] Comparador: lado a lado pela devolução e, havendo técnico humano no clube, pelo byte da vaga ativa dele (a 0x5fbe20
/// grava). Clube nulo (caixa): só a original.
pub(crate) unsafe extern "thiscall" fn compare_get_club_tactic(this: usize, club: usize, team: u32) -> usize {
    let club = Club(club);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(IDX_GET_CLUB_TACTIC);
    if club.is_null() {
        return o(this, club.addr(), team);
    }
    let manager = read_usize(G_HUMAN_MANAGER);
    let t = club.manager().addr();
    let mut reg = Vec::new();
    if manager != 0 && t != 0 {
        if let Some(k) = human_manager_human_manager_index(manager, t) {
            reg.push((human_manager_record_at(manager, k) + 0x9a69, 1));
        }
    }
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, club.addr(), team), || get_club_tactic(this, club.addr(), team));
    SCOREBOARD.record(igual && r == n, || format!("get_club_tactic(clube {:#x}, time {}) original={:#x} {:x?} rust={:#x} {:x?}", club.addr(), team, r, mo, n, mr));
    r
}

/// Comparer for `find_selection_slot`.
pub(crate) unsafe extern "thiscall" fn compare_find_selection_slot(tactic: usize, club: usize, person: *const i32, team: u32) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, *const i32, u32) -> u8>(IDX_FIND_SELECTION_SLOT)(tactic, club, person, team);
    if club == 0 || person.is_null() {
        return r;
    }
    let n = find_selection_slot(tactic, club, person, team);
    SCOREBOARD.record(r == n, || format!("find_selection_slot(time {}) original={} rust={}", team, r, n));
    r
}

/// Comparer for `set_tactic_slot`.
pub(crate) unsafe extern "thiscall" fn compare_set_tactic_slot(t: usize, club: usize, slot: i8, person: *const u32, human: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i8, *const u32, u32) -> u32>(IDX_SET_TACTIC_SLOT);
    let reg = table_regions(t, club, human);
    let (r, v, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(t, club, slot, person, human), || set_tactic_slot(t, club, slot, person, human));
    SCOREBOARD.record(igual && r == v, || format!("set_tactic_slot({:#x}, {:#x}, {}, {}) original={} {:x?} rust={} {:x?}", t, club, slot, human, r, mo, v, mr));
    r
}

/// Comparer for `tactic_slot_person`.
pub(crate) unsafe extern "thiscall" fn compare_tactic_slot_person(t: usize, club: usize, slot: i8, human: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i8, u32) -> usize>(IDX_TACTIC_SLOT_PERSON);
    let reg = table_regions(t, club, human);
    let (r, v, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(t, club, slot, human), || tactic_slot_person(t, club, slot, human));
    SCOREBOARD.record(igual && r == v, || format!("tactic_slot_person({:#x}, {:#x}, {}, {}) original={:#x} {:x?} rust={:#x} {:x?}", t, club, slot, human, r, mo, v, mr));
    r
}

/// Comparer for `set_tactic_formation`.
pub(crate) unsafe extern "thiscall" fn compare_set_tactic_formation(t: usize, club: usize, formation: i16, human: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i16, u32) -> u32>(IDX_SET_TACTIC_FORMATION);
    let reg = table_regions(t, club, human);
    let (r, v, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(t, club, formation, human), || set_tactic_formation(t, club, formation, human));
    SCOREBOARD.record(igual && r == v, || format!("set_tactic_formation({:#x}, {:#x}, {}, {}) original={} {:x?} rust={} {:x?}", t, club, formation, human, r, mo, v, mr));
    r
}

/// Comparer for `marked_slot`.
pub(crate) unsafe extern "thiscall" fn compare_marked_slot(this: usize, club: usize, human: u32, minus_one: u32) -> i8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32) -> i8>(IDX_MARKED_SLOT)(this, club, human, minus_one);
    let v = marked_slot(this, club, human, minus_one);
    SCOREBOARD.record(r == v, || format!("marked_slot original={} rust={}", r, v));
    r
}

/// Comparer for `mark_slot`.
pub(crate) unsafe extern "thiscall" fn compare_mark_slot(this: usize, club: usize, slot: i8, human: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i8, u32) -> u32>(IDX_MARK_SLOT);
    if is_recording_top_level() {
        return call_recording(FN_TACTICS_SET_CAPTAIN_FROM_SLOT, this, [club as u32, slot as u8 as u32, human, 0, 0], || o(this, club, slot, human));
    }
    let table = *((this + tactics_fields::ROWS) as *const usize);
    // the row can be any in the table: compares 0x52..0x56 of every possible row
    let row: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(FN_TACTICS_GET_CLUB_TACTICS_INDEX);
    let l = row(club, human);
    let reg: Vec<(usize, usize)> = if l == -1 || club == 0 { vec![] } else { vec![(table.wrapping_add((l as usize).wrapping_mul(0x91)) + 0x52, 4)] };
    let (r, v, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, club, slot, human), || mark_slot(this, club, slot, human));
    SCOREBOARD.record(igual && r == v, || format!("mark_slot original={} {:x?} rust={} {:x?}", r, mo, v, mr));
    r
}

/// Comparer for `tactics_row`.
pub(crate) unsafe extern "stdcall" fn compare_tactics_row(club: usize, human: u32) -> i32 {
    let r = original_fn::<unsafe extern "stdcall" fn(usize, u32) -> i32>(IDX_TACTICS_ROW)(club, human);
    let v = tactics_row(club, human);
    SCOREBOARD.record(r == v, || format!("tactics_row({:#x}, {}) original={} rust={}", club, human, r, v));
    r
}
