// ---------- batch %%LEVA%%: the tactics screen object (tactics_screens.cpp, 4): the takers, the penalty order and the formation edits ----------

/// Sets a set piece taker: the first (`choice` 0) or second (1) choice of a kind (0..4) becomes the staff id
/// `player`.
///
/// A kind or choice out of range shows the tactics_screens.cpp error box of line 0x1630 and returns 0; a read-only
/// screen returns 0. Otherwise the undo copy is saved, the taker written and, when the first choice is cleared
/// (`player` 0), the second choice moves up into it and is cleared. The team data is marked changed; returns 1.
/// Original: `Tactics_Screen::screen_set_piece_taker` (0x8a8ed0, tactics_screens.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn screen_set_piece_taker(ts: usize, kind: u32, choice: u32, player: u32) -> u32 {
    let (kind, choice) = (kind as u8 as i8, choice as u8 as i8);
    if !(0..=4).contains(&kind) || !(0..=1).contains(&choice) {
        tel26_error_box(TACTICS_SCREENS_CPP, 0x1630);
        return 0;
    }
    if read_u32(ts + tactics_screen::READ_ONLY) != 0 {
        return 0;
    }
    covered!("tel15_2 set a set piece taker");
    tactics_save_undo(ts);
    let takers = ts + tactics_screen_state::SET_PIECE_TAKERS + kind as usize * 8;
    write_u32(takers + choice as usize * 4, player);
    if player == 0 && choice < 1 {
        let mut k = choice + 1;
        while k < 2 {
            let next = read_u32(takers + k as usize * 4);
            if next != 0 {
                write_u32(takers + choice as usize * 4, next);
                write_u32(takers + k as usize * 4, 0);
                break;
            }
            k += 1;
        }
    }
    write_u32(ts + tactics_screen_state::TEAM_DATA_CHANGED, 1);
    1
}

/// Sets the first (`choice` 0) or second (1) choice playmaker to the staff id `player`.
///
/// A choice out of range shows the error box of line 0x1674 and returns 0; a read-only screen returns 0. Otherwise
/// the undo copy is saved and the playmaker written; clearing the first choice moves the second up into it (and
/// clears the second). The team data is marked changed; returns 1.
/// Original: `Tactics_Screen::set_playmaker` (0x8a9020, tactics_screens.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn set_playmaker(ts: usize, choice: u32, player: u32) -> u32 {
    let choice = choice as u8 as i8;
    if !(0..=1).contains(&choice) {
        tel26_error_box(TACTICS_SCREENS_CPP, 0x1674);
        return 0;
    }
    if read_u32(ts + tactics_screen::READ_ONLY) != 0 {
        return 0;
    }
    covered!("tel15_2 set a playmaker");
    tactics_save_undo(ts);
    let p = ts + tactics_screen_state::PLAYMAKERS;
    write_u32(p + choice as usize * 4, player);
    if player == 0 && choice == 0 && read_u32(p + 4) != 0 {
        write_u32(p, read_u32(p + 4));
        write_u32(p + 4, 0);
    }
    write_u32(ts + tactics_screen_state::TEAM_DATA_CHANGED, 1);
    1
}

/// Puts a squad row (0..19) at one place (0..10) of the penalty takers order.
///
/// A place out of range shows the error box of line 0x16d0 and a row out of range the box of line 0x16d7 (both
/// return 0); a read-only screen returns 0. When the row is not already at that place, the undo copy is saved, the
/// row's other places (if any) get the place's previous row (a swap), the row takes the place and the team data is
/// marked changed. Returns 1.
/// Original: `Tactics_Screen::set_penalty_taker` (0x8a9120, tactics_screens.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn set_penalty_taker(ts: usize, order: u32, row: u32) -> u32 {
    let (order, row) = (order as u8 as i8, row as u8 as i8);
    if !(0..=10).contains(&order) {
        tel26_error_box(TACTICS_SCREENS_CPP, 0x16d0);
        return 0;
    }
    if !(0..=0x13).contains(&row) {
        tel26_error_box(TACTICS_SCREENS_CPP, 0x16d7);
        return 0;
    }
    if read_u32(ts + tactics_screen::READ_ONLY) != 0 {
        return 0;
    }
    let place = ts + tactics_screen::PENALTY_ORDER + order as usize;
    let previous = read_i8(place);
    if row != previous {
        covered!("tel15_2 set a penalty taker");
        tactics_save_undo(ts);
        for k in 0..11 {
            let p = ts + tactics_screen::PENALTY_ORDER + k;
            if read_i8(p) == row {
                write_i8(p, read_i8(place));
            }
        }
        write_i8(place, row);
        write_u32(ts + tactics_screen_state::TEAM_DATA_CHANGED, 1);
    }
    1
}

/// Swaps two places (0..10) of the penalty takers order.
///
/// A place out of range shows the error box of line 0x1704 and returns 0; a read-only screen returns 0. Two
/// different places: the undo copy is saved, the rows swapped and the team data marked changed. Returns 1.
/// Original: `Tactics_Screen::swap_penalty_takers` (0x8a9270, tactics_screens.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn swap_penalty_takers(ts: usize, a: u32, b: u32) -> u32 {
    let (a, b) = (a as u8 as i8, b as u8 as i8);
    if !(0..=10).contains(&a) || !(0..=10).contains(&b) {
        tel26_error_box(TACTICS_SCREENS_CPP, 0x1704);
        return 0;
    }
    if read_u32(ts + tactics_screen::READ_ONLY) != 0 {
        return 0;
    }
    if a != b {
        covered!("tel15_2 swap two penalty takers");
        tactics_save_undo(ts);
        let (pa, pb) = (ts + tactics_screen::PENALTY_ORDER + a as usize, ts + tactics_screen::PENALTY_ORDER + b as usize);
        let (va, vb) = (read_i8(pa), read_i8(pb));
        write_i8(pa, vb);
        write_i8(pb, va);
        write_u32(ts + tactics_screen_state::TEAM_DATA_CHANGED, 1);
    }
    1
}

/// The squad number the human manager gives a player for the penalty order, 0x33 when he has none (0xff).
unsafe fn tactics_penalty_squad_number(player: usize) -> i8 {
    let n = tactics_screen_calls::call2(tactics_screen_fns::HUMAN_MANAGER_GET_PLAYER_SQUAD_NUMBER, read_usize(G_HUMAN_MANAGER), player as u32, 0) as u8;
    if n == 0xff { 0x33 } else { n as i8 }
}

/// Fills the club's eleven penalty taker places (in the Tactics table) with the players on the pitch.
///
/// First the pitch position of each of the 20 rows is asked for and dropped (`get_slot_pitch_position` with no
/// output). Then, for each place 0..10, the best row not yet used whose player has player data and is on the pitch:
/// the lowest squad number (the human manager's; none counts as 0x33), the higher penalties attribute
/// (`player_sheet::PENALTIES`) on a tie, the first row on a full tie. The place gets that row in the club's Tactics
/// row (`Tactics::set_shootout_taker_slot`, with the reserve flag; -1 when no row is left) and the row is used.
/// Returns 1.
/// Original: `Tactics_Screen::set_default_penalty_takers` (0x8a9370, tactics_screens.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn set_default_penalty_takers(ts: usize) -> u32 {
    covered!("tel15_2 default penalty takers");
    for row in 0..0x14u32 {
        tactics_screen_calls::call3(tactics_screen_fns::GET_SLOT_PITCH_POSITION, ts, row, 0, 0);
    }
    let mut used = [0u32; 20];
    for order in 0..0xbu32 {
        let mut best: i8 = -1;
        let mut best_player = 0usize;
        for row in 0..0x14usize {
            let player = tactics_screen_calls::call2(tactics_screen_fns::GET_SLOT_PLAYER, ts, row as u32, 0) as usize;
            if player == 0 || read_usize(player + staff::PLAYER) == 0 {
                continue;
            }
            if tactics_screen_calls::call3(tactics_screen_fns::GET_SLOT_PITCH_POSITION, ts, row as u32, 0, 0) == 0 || used[row] != 0 {
                continue;
            }
            let number = tactics_penalty_squad_number(player);
            let best_number = if best_player != 0 { tactics_penalty_squad_number(best_player) } else { 0x33 };
            if best == -1 || number < best_number {
                best = row as i8;
                best_player = player;
            } else if number == best_number {
                let mine = read_i8(read_usize(player + staff::PLAYER) + player_sheet::PENALTIES);
                let theirs = read_i8(read_usize(best_player + staff::PLAYER) + player_sheet::PENALTIES);
                if mine > theirs {
                    best = row as i8;
                    best_player = player;
                }
            }
        }
        tactics_screen_calls::call4(tactics_screen_fns::TACTICS_SET_SHOOTOUT_TAKER_SLOT, read_usize(G_TACTICS), read_u32(ts + tactics_screen::CLUB), order,
                                    best as u8 as u32, read_u32(ts + tactics_screen_state::RESERVE));
        if best > -1 {
            used[best as usize] = 1;
        }
    }
    1
}

/// Reads a formation slot's (0..10) pitch coordinate into `out` for the zone under the mouse
/// (`Formation::get_slot_coordinate`).
///
/// A slot out of range shows the error box of line 0x1784 and returns 0. The phase is that of the zone under the mouse
/// (`HOVER_VIEW` 2: without the ball), or the view's own when `HOVER_VIEW` is 1; the zone is `HOVER_ZONE`. Returns 1.
/// Original: `Tactics_Screen::get_slot_coordinate` (0x8a94d0, tactics_screens.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn get_slot_coordinate(ts: usize, slot: u32, out: u32) -> u32 {
    let s = slot as u8 as i8;
    if !(0..=10).contains(&s) {
        tel26_error_box(TACTICS_SCREENS_CPP, 0x1784);
        return 0;
    }
    let hover = read_u8(ts + tactics_screen_state::HOVER_VIEW);
    let phase = if hover == 1 { tactics_view_phase(ts) } else { (hover == 2) as u32 };
    tactics_screen_calls::call4(tactics_screen_fns::FORMATION_GET_SLOT_COORDINATE, ts + tactics_screen::FORMATION, slot, phase,
                                read_u32(ts + tactics_screen::HOVER_ZONE), out);
    1
}

/// Loads a formation file into the working tactics (`Formation::load_from_disk`, silent, `preset` as the
/// obfuscated flag).
///
/// A null path shows the error box of line 0x17b9 and returns 0; a read-only screen returns 0. The undo copy is saved
/// first; a failed load puts it back (Undo withdrawn) and returns 0. Otherwise the team data is marked changed and,
/// for a file of the human's own (not a preset), the path enters his recent files (kind 1). Returns 1.
/// Original: `Tactics_Screen::load_formation` (0x8a95d0, tactics_screens.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn load_formation(ts: usize, path: u32, preset: u32) -> u32 {
    if path == 0 {
        tel26_error_box(TACTICS_SCREENS_CPP, 0x17b9);
        return 0;
    }
    if read_u32(ts + tactics_screen::READ_ONLY) != 0 {
        return 0;
    }
    tactics_save_undo(ts);
    if tactics_screen_calls::call3(tactics_screen_fns::FORMATION_LOAD_FROM_DISK, ts + tactics_screen::FORMATION, path, 1, preset) == 0 {
        covered!("tel15_2 load a formation that fails");
        tactics_restore_undo(ts);
        return 0;
    }
    covered!("tel15_2 load a formation");
    write_u32(ts + tactics_screen_state::TEAM_DATA_CHANGED, 1);
    if preset == 0 {
        tactics_screen_calls::call3(tactics_screen_fns::HUMAN_MANAGER_ADD_RECENT_FILE, read_usize(G_HUMAN_MANAGER), path, 0, 1);
    }
    1
}

/// Copies a C string (the `repne scasb` / `rep movs` copies of the original).
unsafe fn tactics_copy_string(dst: usize, src: usize) {
    let n = (0..).take_while(|&k| read_u8(src + k) != 0).count() + 1;
    core::ptr::copy(src as *const u8, dst as *mut u8, n);
}

/// The C string compare of the original (inline `strcmp`): equal or not.
unsafe fn tactics_strings_equal(a: usize, b: usize) -> bool {
    let mut k = 0;
    loop {
        let (x, y) = (read_u8(a + k), read_u8(b + k));
        if x != y {
            return false;
        }
        if x == 0 {
            return true;
        }
        k += 1;
    }
}

/// Saves the working formation to a file (`Formation::save_to_disk`, silent, not obfuscated), signed by the
/// manager of the current network slot (his name formatted into the formation's author field).
///
/// A null path shows the error box of line 0x17f1 and returns 0. With `locking_manager`, the formation is saved under
/// that manager's password (the password field swapped for his and restored after). Without it, a saved formation
/// of a live match whose club the current human controls also replaces the human's active tactic of the club when
/// it has the same name (after selecting the club's tactic slot). A failed save returns 0; otherwise Undo is
/// withdrawn, the path enters the human's recent files (kind 1) and it returns 1.
/// Original: `Tactics_Screen::save_formation` (0x8a9700, tactics_screens.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn save_formation(ts: usize, path: u32, locking_manager: u32) -> u32 {
    if path == 0 {
        tel26_error_box(TACTICS_SCREENS_CPP, 0x17f1);
        return 0;
    }
    let fm = ts + tactics_screen::FORMATION;
    let slot = read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize;
    let author = read_usize((G_SCREEN_RECORDS_MANAGER as isize + slot * 0x300) as usize);
    exe::database_format_staff_name(author, fm + 0x65, 0xc8, 1);
    let saved;
    if locking_manager != 0 {
        covered!("tel15_2 save a formation locked");
        let mut kept = [0u8; 0x100];
        tactics_copy_string(kept.as_mut_ptr() as usize, ts + tactics_screen::LOCKED);
        let password = tactics_screen_calls::call1(tactics_screen_fns::HUMAN_MANAGER_GET_PASSWORD, read_usize(G_HUMAN_MANAGER), locking_manager) as usize;
        tactics_copy_string(ts + tactics_screen::LOCKED, password);
        saved = tactics_screen_calls::call3(tactics_screen_fns::FORMATION_SAVE_TO_DISK, fm, path, 1, 0);
        tactics_copy_string(ts + tactics_screen::LOCKED, kept.as_ptr() as usize);
    } else {
        saved = tactics_screen_calls::call3(tactics_screen_fns::FORMATION_SAVE_TO_DISK, fm, path, 1, 0);
        if saved == 0 {
            return 0;
        }
        let club = read_usize(ts + tactics_screen::CLUB);
        if read_usize(ts + tactics_screen::MATCH) != 0 && exe::human_manager_current_controls_club(club, 0, 1) != 0 {
            covered!("tel15_2 save the formation of a live match");
            exe::human_manager_select_tactic_slot(read_usize(G_HUMAN_MANAGER), read_u32(ts + tactics_screen_state::RESERVE) as i32, club, 0usize);
            let active = exe::human_manager_get_active_tactic(read_usize(G_HUMAN_MANAGER), read_usize(club + club::MANAGER));
            if tactics_strings_equal(fm, active) {
                core::ptr::copy(fm as *const u8, active as *mut u8, tactics_screen_state::FORMATION_SIZE);
            }
        }
    }
    if saved == 0 {
        return 0;
    }
    write_u32(ts + tactics_screen::CAN_UNDO, 0);
    tactics_screen_calls::call3(tactics_screen_fns::HUMAN_MANAGER_ADD_RECENT_FILE, read_usize(G_HUMAN_MANAGER), path, 0, 1);
    1
}

/// Copies one of the Tactics presets into the working formation.
///
/// An index below 0 or past the presets (`G_TACTICS` +0x5fd) shows the error box of line 0x1845 and returns 0; a
/// read-only screen returns 0. The undo copy is saved and the preset's formation block (`Tactics::get_tactic_block`)
/// copied over the formation; a preset whose name is one of the eight standard formations ("4-4-2 Defensive" ...
/// "Sweeper", compared without case) gets its name rewritten through the language table, any other its own name
/// copied again. The team data is marked changed; returns 1.
/// Original: `Tactics_Screen::set_preset_formation` (0x8a9950, tactics_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn set_preset_formation(ts: usize, preset: u32) -> u32 {
    let k = preset as u16 as i16;
    if k < 0 || k as i32 >= read_i32(read_usize(G_TACTICS) + 0x5fd) {
        tel26_error_box(TACTICS_SCREENS_CPP, 0x1845);
        return 0;
    }
    if read_u32(ts + tactics_screen::READ_ONLY) != 0 {
        return 0;
    }
    covered!("tel15_2 set a preset formation");
    tactics_save_undo(ts);
    let fm = ts + tactics_screen::FORMATION;
    let block = || exe::tactics_get_tactic_block(read_usize(G_TACTICS), preset, 1);
    core::ptr::copy(block() as *const u8, fm as *mut u8, tactics_screen_state::FORMATION_SIZE);
    let mut named = false;
    for &name in TACTICS_PRESET_NAMES.iter() {
        if exe::crt_stricmp(block(), name) == 0 {
            let f: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
            f(fm, name);
            named = true;
            break;
        }
    }
    if !named {
        tactics_copy_string(fm, block());
    }
    write_u32(ts + tactics_screen_state::TEAM_DATA_CHANGED, 1);
    1
}

/// Sets a player instruction of a formation slot (`Formation::set_player_instruction`, the 64-bit flag).
///
/// A read-only screen returns 0. The undo copy is saved; when the Formation refuses, it is put back (Undo withdrawn)
/// and 0 returned; otherwise the team data is marked changed and 1 returned.
/// Original: `Tactics_Screen::set_player_instruction` (0x8a9c20, tactics_screens.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn set_player_instruction(ts: usize, slot: u32, flag_lo: u32, flag_hi: u32) -> u32 {
    if read_u32(ts + tactics_screen::READ_ONLY) != 0 {
        return 0;
    }
    tactics_save_undo(ts);
    if tactics_screen_calls::call3(tactics_screen_fns::FORMATION_SET_PLAYER_INSTRUCTION, ts + tactics_screen::FORMATION, slot, flag_lo, flag_hi) == 0 {
        covered!("tel15_2 player instruction refused");
        tactics_restore_undo(ts);
        return 0;
    }
    covered!("tel15_2 set a player instruction");
    write_u32(ts + tactics_screen_state::TEAM_DATA_CHANGED, 1);
    1
}

/// Pastes the copied positions into the view shown.
///
/// Nothing (0) on a read-only screen, in the overview (view 1) or with nothing copied (`CAN_PASTE` 0). Otherwise the
/// undo copy is saved and, for a whole phase copied (`CAN_PASTE` 2), the phase's coordinates replaced
/// (`Formation::set_phase_coordinates` with `COPIED_PHASE`), else the zone shown (`ZONE`) gets the copied zone
/// (`Formation::set_zone_coordinates` with `COPIED_ZONE`); the phase is the view's. The team data is marked changed;
/// returns 1.
/// Original: `Tactics_Screen::paste_positions` (0x8a9cb0, tactics_screens.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn paste_positions(ts: usize) -> u32 {
    if read_u32(ts + tactics_screen::READ_ONLY) != 0 || read_u8(ts + tactics_screen::VIEW) == 1 || read_u8(ts + tactics_screen::CAN_PASTE) == 0 {
        return 0;
    }
    tactics_save_undo(ts);
    let fm = ts + tactics_screen::FORMATION;
    if read_u8(ts + tactics_screen::CAN_PASTE) == 2 {
        covered!("tel15_2 paste a phase");
        tactics_screen_calls::call2(tactics_screen_fns::FORMATION_SET_PHASE_COORDINATES, fm, tactics_view_phase(ts), (ts + tactics_screen_state::COPIED_PHASE) as u32);
    } else {
        covered!("tel15_2 paste a zone");
        tactics_screen_calls::call3(tactics_screen_fns::FORMATION_SET_ZONE_COORDINATES, fm, tactics_view_phase(ts), read_u32(ts + tactics_screen::ZONE),
                                    (ts + tactics_screen_state::COPIED_ZONE) as u32);
    }
    write_u32(ts + tactics_screen_state::TEAM_DATA_CHANGED, 1);
    1
}

/// Swaps the pitch coordinates of two formation slots (0..10) in the zone shown (`Formation::swap_slot_coordinates`).
///
/// A slot out of range shows the error box of line 0x19c0; a read-only screen and the overview do nothing. Otherwise
/// the undo copy is saved, the coordinates swapped with the view's phase and the zone shown, and the team data
/// marked changed. Returns 0 in every case (the callers redraw on their own).
/// Original: `Tactics_Screen::swap_slot_coordinates` (0x8a9d70, tactics_screens.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn swap_slot_coordinates(ts: usize, a: u32, b: u32) -> u32 {
    let (sa, sb) = (a as u8 as i8, b as u8 as i8);
    if !(0..=10).contains(&sa) || !(0..=10).contains(&sb) {
        tel26_error_box(TACTICS_SCREENS_CPP, 0x19c0);
        return 0;
    }
    if read_u32(ts + tactics_screen::READ_ONLY) != 0 || read_u8(ts + tactics_screen::VIEW) == 1 {
        return 0;
    }
    covered!("tel15_2 swap two slots' coordinates");
    tactics_save_undo(ts);
    tactics_screen_calls::call4(tactics_screen_fns::FORMATION_SWAP_SLOT_COORDINATES, ts + tactics_screen::FORMATION, a, b, tactics_view_phase(ts),
                                read_u32(ts + tactics_screen::ZONE));
    write_u32(ts + tactics_screen_state::TEAM_DATA_CHANGED, 1);
    0
}

/// Comparer for `screen_set_piece_taker`: side by side over the screen object (the box cases, original only).
pub(crate) unsafe extern "thiscall" fn compare_screen_set_piece_taker(ts: usize, kind: u32, choice: u32, player: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32) -> u32>(%%I0%%);
    if !(0..=4).contains(&(kind as u8 as i8)) || !(0..=1).contains(&(choice as u8 as i8)) {
        return o(ts, kind, choice, player); // simplification: the box, original only
    }
    compare_tactics_method("screen_set_piece_taker", &[tactics_screen_region(ts)], || o(ts, kind, choice, player), || screen_set_piece_taker(ts, kind, choice, player))
}

/// Comparer for `set_playmaker`.
pub(crate) unsafe extern "thiscall" fn compare_set_playmaker(ts: usize, choice: u32, player: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u32>(%%I1%%);
    if !(0..=1).contains(&(choice as u8 as i8)) {
        return o(ts, choice, player); // simplification: the box, original only
    }
    compare_tactics_method("set_playmaker", &[tactics_screen_region(ts)], || o(ts, choice, player), || set_playmaker(ts, choice, player))
}

/// Comparer for `set_penalty_taker`.
pub(crate) unsafe extern "thiscall" fn compare_set_penalty_taker(ts: usize, order: u32, row: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u32>(%%I2%%);
    if !(0..=10).contains(&(order as u8 as i8)) || !(0..=0x13).contains(&(row as u8 as i8)) {
        return o(ts, order, row); // simplification: the box, original only
    }
    compare_tactics_method("set_penalty_taker", &[tactics_screen_region(ts)], || o(ts, order, row), || set_penalty_taker(ts, order, row))
}

/// Comparer for `swap_penalty_takers`.
pub(crate) unsafe extern "thiscall" fn compare_swap_penalty_takers(ts: usize, a: u32, b: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u32>(%%I3%%);
    if !(0..=10).contains(&(a as u8 as i8)) || !(0..=10).contains(&(b as u8 as i8)) {
        return o(ts, a, b); // simplification: the box, original only
    }
    compare_tactics_method("swap_penalty_takers", &[tactics_screen_region(ts)], || o(ts, a, b), || swap_penalty_takers(ts, a, b))
}

/// Comparer for `set_default_penalty_takers`: side by side over the screen object and the club's Tactics row.
pub(crate) unsafe extern "thiscall" fn compare_set_default_penalty_takers(ts: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I4%%);
    let mut reg = vec![tactics_screen_region(ts)];
    if let Some(row) = tactics_row_of_club(read_usize(G_TACTICS), read_usize(ts + tactics_screen::CLUB), read_u32(ts + tactics_screen_state::RESERVE)) {
        reg.push((row, 0x91));
    }
    compare_tactics_method("set_default_penalty_takers", &reg, || o(ts), || set_default_penalty_takers(ts))
}

/// Comparer for `get_slot_coordinate`: side by side over the output coordinate.
pub(crate) unsafe extern "thiscall" fn compare_get_slot_coordinate(ts: usize, slot: u32, out: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u32>(%%I5%%);
    if !(0..=10).contains(&(slot as u8 as i8)) || out == 0 {
        return o(ts, slot, out); // simplification: the box, original only
    }
    compare_tactics_method("get_slot_coordinate", &[(out as usize, 4)], || o(ts, slot, out), || get_slot_coordinate(ts, slot, out))
}

// simplification: reads a file and adds a recent file: only the original runs; the plan: on success the team data is
// marked changed, else the working team data is the one before.
/// Comparer for `load_formation`.
pub(crate) unsafe extern "thiscall" fn compare_load_formation(ts: usize, path: u32, preset: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u32>(%%I6%%);
    if path == 0 || read_u32(ts + tactics_screen::READ_ONLY) != 0 {
        return o(ts, path, preset);
    }
    let before: Vec<u8> = core::slice::from_raw_parts((ts + tactics_screen::FORMATION) as *const u8, tactics_screen_state::TEAM_DATA_SIZE).to_vec();
    let r = o(ts, path, preset);
    let now = core::slice::from_raw_parts((ts + tactics_screen::FORMATION) as *const u8, tactics_screen_state::TEAM_DATA_SIZE);
    let undo = core::slice::from_raw_parts((ts + tactics_screen::UNDO_DATA) as *const u8, tactics_screen_state::TEAM_DATA_SIZE);
    let ok = if r != 0 {
        read_u32(ts + tactics_screen_state::TEAM_DATA_CHANGED) == 1 && read_u32(ts + tactics_screen::CAN_UNDO) == 1 && undo == &before[..]
    } else {
        now == &before[..] && read_u32(ts + tactics_screen::CAN_UNDO) == 0
    };
    SCOREBOARD.record(ok, || format!("tactics screen load_formation({:#x}) {}: team data or undo off the plan", path, r));
    r
}

// simplification: writes a file and adds a recent file: only the original runs; the plan: on success Undo is withdrawn
// and the password field is the one before.
/// Comparer for `save_formation`.
pub(crate) unsafe extern "thiscall" fn compare_save_formation(ts: usize, path: u32, locking_manager: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u32>(%%I7%%);
    if path == 0 {
        return o(ts, path, locking_manager);
    }
    let password: Vec<u8> = core::slice::from_raw_parts((ts + tactics_screen::LOCKED) as *const u8, 0x30).to_vec();
    let r = o(ts, path, locking_manager);
    let same = core::slice::from_raw_parts((ts + tactics_screen::LOCKED) as *const u8, 0x30) == &password[..];
    SCOREBOARD.record(same && (r == 0 || read_u32(ts + tactics_screen::CAN_UNDO) == 0), || format!("tactics screen save_formation({:#x}) {}: off the plan", path, r));
    r
}

/// Comparer for `set_preset_formation`.
pub(crate) unsafe extern "thiscall" fn compare_set_preset_formation(ts: usize, preset: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(%%I8%%);
    let k = preset as u16 as i16;
    if k < 0 || k as i32 >= read_i32(read_usize(G_TACTICS) + 0x5fd) {
        return o(ts, preset); // simplification: the box, original only
    }
    compare_tactics_method("set_preset_formation", &[tactics_screen_region(ts)], || o(ts, preset), || set_preset_formation(ts, preset))
}

/// Comparer for `set_player_instruction`.
pub(crate) unsafe extern "thiscall" fn compare_set_player_instruction(ts: usize, slot: u32, flag_lo: u32, flag_hi: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32) -> u32>(%%I9%%);
    compare_tactics_method("set_player_instruction", &[tactics_screen_region(ts)], || o(ts, slot, flag_lo, flag_hi), || set_player_instruction(ts, slot, flag_lo, flag_hi))
}

/// Comparer for `paste_positions`.
pub(crate) unsafe extern "thiscall" fn compare_paste_positions(ts: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I10%%);
    compare_tactics_method("paste_positions", &[tactics_screen_region(ts)], || o(ts), || paste_positions(ts))
}

/// Comparer for `swap_slot_coordinates`.
pub(crate) unsafe extern "thiscall" fn compare_swap_slot_coordinates(ts: usize, a: u32, b: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u32>(%%I11%%);
    if !(0..=10).contains(&(a as u8 as i8)) || !(0..=10).contains(&(b as u8 as i8)) {
        return o(ts, a, b); // simplification: the box, original only
    }
    compare_tactics_method("swap_slot_coordinates", &[tactics_screen_region(ts)], || o(ts, a, b), || swap_slot_coordinates(ts, a, b))
}
