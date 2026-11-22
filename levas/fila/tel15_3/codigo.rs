// ---------- batch %%LEVA%%: the tactics screen object (tactics_screens.cpp, 5): the slot edits and the row texts ----------

/// Sets a formation slot's (0..10) pitch coordinate in the zone shown (`Formation::set_slot_coordinate`).
///
/// A slot out of range shows the error box of line 0x19e7 and returns 0; a read-only screen and the overview
/// (view 1) return 0. Otherwise the undo copy is saved, the coordinate set with the view's phase and the zone shown
/// (whatever the Formation answers) and the team data marked changed; returns 1.
/// Original: `Tactics_Screen::set_formation_slot_coordinate` (0x8a9e90, tactics_screens.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn set_formation_slot_coordinate(ts: usize, slot: u32, coordinate: u32) -> u32 {
    if !(0..=10).contains(&(slot as u8 as i8)) {
        tel26_error_box(TACTICS_SCREENS_CPP, 0x19e7);
        return 0;
    }
    if read_u32(ts + tactics_screen::READ_ONLY) != 0 || read_u8(ts + tactics_screen::VIEW) == 1 {
        return 0;
    }
    covered!("tel15_3 set a slot's coordinate");
    tactics_save_undo(ts);
    tactics_screen_calls::call4(tactics_screen_fns::FORMATION_SET_SLOT_COORDINATE, ts + tactics_screen::FORMATION, slot, tactics_view_phase(ts),
                                read_u32(ts + tactics_screen::ZONE), coordinate);
    write_u32(ts + tactics_screen_state::TEAM_DATA_CHANGED, 1);
    1
}

/// Whether a formation slot (0..10) has a custom position: in the overview, any custom coordinate
/// (`Formation::is_slot_customised`); in the other views, a custom coordinate in the zone shown with the view's phase
/// (`Formation::is_slot_coordinate_customised`).
///
/// A slot out of range shows the error box of line 0x1a0d and returns 0.
/// Original: `Tactics_Screen::is_formation_slot_customised` (0x8a9fa0, tactics_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn is_formation_slot_customised(ts: usize, slot: u32) -> u32 {
    if !(0..=10).contains(&(slot as u8 as i8)) {
        tel26_error_box(TACTICS_SCREENS_CPP, 0x1a0d);
        return 0;
    }
    let fm = ts + tactics_screen::FORMATION;
    if read_u8(ts + tactics_screen::VIEW) == 1 {
        return tactics_screen_calls::call1(tactics_screen_fns::FORMATION_IS_SLOT_CUSTOMISED, fm, slot);
    }
    tactics_screen_calls::call3(tactics_screen_fns::FORMATION_IS_SLOT_COORDINATE_CUSTOMISED, fm, slot, tactics_view_phase(ts), read_u32(ts + tactics_screen::ZONE))
}

/// Sets a formation slot's (0..10) marking type (`Formation::set_marking_type`).
///
/// A slot out of range shows the error box of line 0x1a25 and returns 0; a read-only screen returns 0. Otherwise the
/// undo copy is saved, the type set and the team data marked changed; returns 1.
/// Original: `Tactics_Screen::set_slot_marking_type` (0x8aa070, tactics_screens.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn set_slot_marking_type(ts: usize, slot: u32, marking: u32) -> u32 {
    if !(0..=10).contains(&(slot as u8 as i8)) {
        tel26_error_box(TACTICS_SCREENS_CPP, 0x1a25);
        return 0;
    }
    if read_u32(ts + tactics_screen::READ_ONLY) != 0 {
        return 0;
    }
    covered!("tel15_3 set a slot's marking type");
    tactics_save_undo(ts);
    tactics_screen_calls::call2(tactics_screen_fns::FORMATION_SET_MARKING_TYPE, ts + tactics_screen::FORMATION, slot, marking);
    write_u32(ts + tactics_screen_state::TEAM_DATA_CHANGED, 1);
    1
}

/// Sets a formation slot's (0..10) marking target (`Formation::set_marking_target`), without an undo copy and
/// without marking the team data changed.
///
/// A slot out of range shows the error box of line 0x1a49 and returns 0; a read-only screen returns 0; else 1.
/// Original: `Tactics_Screen::set_slot_marking_target` (0x8aa150, tactics_screens.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn set_slot_marking_target(ts: usize, slot: u32, target: u32) -> u32 {
    if !(0..=10).contains(&(slot as u8 as i8)) {
        tel26_error_box(TACTICS_SCREENS_CPP, 0x1a49);
        return 0;
    }
    if read_u32(ts + tactics_screen::READ_ONLY) != 0 {
        return 0;
    }
    covered!("tel15_3 set a slot's marking target");
    tactics_screen_calls::call2(tactics_screen_fns::FORMATION_SET_MARKING_TARGET, ts + tactics_screen::FORMATION, slot, target);
    1
}

/// `sprintf(out, "%d", n)` (the CRT's).
unsafe fn tactics_sprintf_number(out: usize, n: i32) {
    let f: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    f(out, 0x98806c, n);
}

/// The squad number of a staff at a club (`SQUAD_MANAGER::get_squad_number`).
unsafe fn tactics_squad_number(staff: usize, club: usize) -> i8 {
    exe::squad_manager_get_squad_number(read_usize(G_SQUAD_MANAGER), staff, club)
}

/// Writes a row's number text into `out` (at least 10 bytes): the shirt number, or the row number.
///
/// A row out of 0..19, a null `out` or a size under 10 shows the error box of line 0x1a74 and returns 0. The text is
/// cleared first; the opponent's row without an opponent returns 0. With a match record (`RECORD_INDEX` > -1): the
/// record row's shirt number when the row is filled and has one, else the row number (row + 1). The reserve screen
/// and a club without squad numbers (`SQUAD_MANAGER::club_uses_squad_numbers`): the row number. In a live match: the
/// squad number (made positive) of the row's match player at the club, when it has a staff and a number (else
/// nothing). Outside a match: the squad number of the row's selected player (the opponent's predicted line-up, or
/// `Tactics::get_selected_player` with the reserve flag), when there is one; with no text and the fallback allowed
/// (`no_fallback` 0), the position abbreviation of the formation slot for rows 0..10
/// (`formation_get_position_abbreviation`), "SB<n>" (n = row - 10, through the language table) for the bench.
/// Returns 1.
/// Original: `Tactics_Screen::get_slot_number_text` (0x8aa200, tactics_screens.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn get_slot_number_text(ts: usize, row: u32, out: u32, size: u32, no_fallback: u32, opponent: u32) -> u32 {
    let r = row as u8 as i8;
    let out = out as usize;
    if !(0..=0x13).contains(&r) || out == 0 || (size as i32) < 0xa {
        tel26_error_box(TACTICS_SCREENS_CPP, 0x1a74);
        return 0;
    }
    write_u8(out, 0);
    let opp = opponent != 0;
    if opp && read_usize(ts + tactics_screen_state::OPPONENT) == 0 {
        return 0;
    }
    if read_i32(ts + tactics_screen_state::RECORD_INDEX) > -1 {
        covered!("tel15_3 number text of a match record row");
        let rr = tactics_record_row(ts, r, opp);
        let shirt = read_i8(rr + match_record_row::SHIRT_NUMBER);
        if read_u8(rr + match_record_row::POSITION) != 0xfb && shirt > 0 {
            tactics_sprintf_number(out, shirt as i32);
        } else {
            tactics_sprintf_number(out, r as i32 + 1);
        }
        return 1;
    }
    let club = tactics_row_club(ts, opp);
    if read_u32(ts + tactics_screen_state::RESERVE) != 0 || exe::squad_manager_club_uses_squad_numbers(read_usize(G_SQUAD_MANAGER), club, 0) == 0 {
        tactics_sprintf_number(out, r as i32 + 1);
        return 1;
    }
    if read_usize(ts + tactics_screen::MATCH) != 0 {
        covered!("tel15_3 number text in a live match");
        let mp = tactics_row_match_player(ts, r, opp);
        if read_u8(mp + match_player::POSITION) == 0xfb {
            return 1;
        }
        let staff = read_usize(mp + match_player::STAFF);
        if staff == 0 || tactics_squad_number(staff, club) == 0 {
            return 1;
        }
        tactics_sprintf_number(out, (tactics_squad_number(staff, club) as i32).abs());
        return 1;
    }
    let staff = if opp {
        read_usize(ts + tactics_screen_state::OPPONENT_LINEUP + r as usize * 4)
    } else {
        exe::tactics_get_selected_player(read_usize(G_TACTICS), read_usize(ts + tactics_screen::CLUB), row, read_u32(ts + tactics_screen_state::RESERVE) as i32)
    };
    if staff != 0 && tactics_squad_number(staff, club) != 0 {
        tactics_sprintf_number(out, (tactics_squad_number(staff, club) as i32).abs());
    }
    if read_u8(out) == 0 && no_fallback == 0 {
        if r < 0xb {
            covered!("tel15_3 number text falls back to the position");
            let position = tactics_screen_calls::call2(tactics_screen_fns::FORMATION_GET_SLOT_POSITION, ts + tactics_screen::FORMATION, row, 0);
            let f: unsafe extern "C" fn(u32, usize, u32, u32) -> u32 = core::mem::transmute(0x5b7810usize);
            f(position, out, size, 1);
            return 1;
        }
        covered!("tel15_3 number text falls back to the bench number");
        let f: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
        f(out, 0x98ea6c, r as i32 - 0xa);
        return 1;
    }
    1
}

/// Writes the name of a row's player into `out` (at least 100 bytes) in a name format.
///
/// A row out of 0..19, a null `out` or a size under 100 shows the error box of line 0x1af7 and returns 0. The text
/// is cleared; the opponent's row without an opponent returns 0. With a match record: the record row's name (from its
/// name ids, `database_format_name_from_ids`); an empty record row returns 1 with `no_fallback`, else the position
/// name of the row's pitch place (`formation_get_position_name`, whose result is returned) or 1 off the pitch.
/// Without a record: the row's player (`get_slot_player`) named by `database_format_staff_name`; no player falls back
/// to the position name the same way. In name format 5 (the short name), when another player of the club's squad
/// (50 places, then those of its linked B team) gets the same short name, the name is written again in format 2
/// (a recursive call, whose result is returned). Returns 1.
/// Original: `Tactics_Screen::get_slot_player_name_text` (0x8aa550, tactics_screens.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn get_slot_player_name_text(ts: usize, row: u32, out: u32, size: u32, no_fallback: u32, format: u32, opponent: u32) -> u32 {
    let r = row as u8 as i8;
    let outp = out as usize;
    if !(0..=0x13).contains(&r) || outp == 0 || (size as i32) < 0x64 {
        tel26_error_box(TACTICS_SCREENS_CPP, 0x1af7);
        return 0;
    }
    write_u8(outp, 0);
    let opp = opponent != 0;
    if opp && read_usize(ts + tactics_screen_state::OPPONENT) == 0 {
        return 0;
    }
    let player = tactics_screen_calls::call2(tactics_screen_fns::GET_SLOT_PLAYER, ts, row, opponent) as usize;
    let position_name = || -> u32 {
        if no_fallback != 0 {
            return 1;
        }
        let mut place = 0u8;
        if tactics_screen_calls::call3(tactics_screen_fns::GET_SLOT_PITCH_POSITION, ts, row, &mut place as *mut u8 as u32, opponent) == 0 {
            return 1;
        }
        covered!("tel15_3 name text falls back to the position");
        let position = tactics_screen_calls::call2(tactics_screen_fns::FORMATION_GET_SLOT_POSITION, ts + tactics_screen::FORMATION, place as u32, 0);
        let f: unsafe extern "C" fn(u32, usize, u32, u32) -> u32 = core::mem::transmute(0x5b7580usize);
        f(position, outp, size, 1)
    };
    if read_i32(ts + tactics_screen_state::RECORD_INDEX) > -1 {
        let rr = tactics_record_row(ts, r, opp);
        if read_u8(rr + match_record_row::POSITION) == 0xfb {
            return position_name();
        }
        covered!("tel15_3 name text of a match record row");
        exe::database_format_name_from_ids(read_u32(rr + match_record_row::FIRST_NAME_ID), read_u32(rr + match_record_row::SECOND_NAME_ID),
                                           read_u32(rr + match_record_row::COMMON_NAME_ID), outp as *mut u8, size, format, read_u32(rr + match_record_row::STAFF_ID));
    } else {
        if player == 0 {
            return position_name();
        }
        exe::database_format_staff_name(player, outp, size, format);
    }
    if format as u8 != 5 {
        return 1;
    }
    let mut club = tactics_row_club(ts, opp);
    let mut b_team = false;
    loop {
        for k in 0..0x32usize {
            let other = read_usize(club + club::SQUAD + k * 4);
            if other == 0 || other == player {
                continue;
            }
            let mut name = [0u8; 0xc8];
            exe::database_format_staff_name(other, name.as_mut_ptr() as usize, 0xc8, 5);
            if exe::utils_string_compare(name.as_ptr(), outp as *const u8, 1, 0) == 0 {
                covered!("tel15_3 short name shared: the long name");
                let again: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32) -> u32 = core::mem::transmute(tactics_screen_fns::GET_SLOT_PLAYER_NAME_TEXT);
                return again(ts, row, out, size, no_fallback, 2, opponent);
            }
        }
        club = exe::club_get_linked_b_team(club, core::ptr::null_mut(), 1);
        if club == 0 || b_team {
            return 1;
        }
        b_team = true;
    }
}

/// Writes a row's number and name into `out` (at least 100 bytes): "<number> <name>" when both exist, else whichever
/// does.
///
/// A row out of 0..19, a null `out` or a size under 100 shows the error box of line 0x1b6c and returns 0. The number
/// comes from `get_slot_number_text` without the fallback, the name from `get_slot_player_name_text` with it (both
/// into 200-byte buffers, for the opponent's row when `opponent`). Returns 1.
/// Original: `Tactics_Screen::get_slot_number_and_name_text` (0x8aa830, tactics_screens.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn get_slot_number_and_name_text(ts: usize, row: u32, out: u32, size: u32, format: u32, opponent: u32) -> u32 {
    if !(0..=0x13).contains(&(row as u8 as i8)) || out == 0 || (size as i32) < 0x64 {
        tel26_error_box(TACTICS_SCREENS_CPP, 0x1b6c);
        return 0;
    }
    covered!("tel15_3 number and name text");
    let (mut number, mut name) = ([0u8; 0xc8], [0u8; 0xc8]);
    let number_text: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> u32 = core::mem::transmute(tactics_screen_fns::GET_SLOT_NUMBER_TEXT);
    number_text(ts, row, number.as_mut_ptr() as u32, 0xc8, 1, opponent);
    let name_text: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32) -> u32 = core::mem::transmute(tactics_screen_fns::GET_SLOT_PLAYER_NAME_TEXT);
    name_text(ts, row, name.as_mut_ptr() as u32, 0xc8, 0, format, opponent);
    if number[0] != 0 {
        if name[0] != 0 {
            let f: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
            f(out as usize, 0x98f0b8, number.as_ptr(), name.as_ptr());
        } else {
            tactics_copy_string(out as usize, number.as_ptr() as usize);
        }
    } else {
        tactics_copy_string(out as usize, name.as_ptr() as usize);
    }
    1
}

/// Whether a row is on the pitch, its formation position (0..10) into `out` when given.
///
/// A row out of 0..19 returns 0 (no box); so does the opponent's row without an opponent. In a live match, the row's
/// match player: a position below 0 counts only when it is -2 (off injured, its previous position given) and no
/// penalty shoot-out is under way. Outside a match, rows 0..10 are on the pitch at their own index.
/// Original: `Tactics_Screen::get_slot_pitch_position` (0x8aa9d0, tactics_screens.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn get_slot_pitch_position(ts: usize, row: u32, out: u32, opponent: u32) -> u32 {
    let r = row as u8 as i8;
    if !(0..=0x13).contains(&r) {
        return 0;
    }
    let opp = opponent != 0;
    if opp && read_usize(ts + tactics_screen_state::OPPONENT) == 0 {
        return 0;
    }
    let m = read_usize(ts + tactics_screen::MATCH);
    if m == 0 {
        if r >= 0xb {
            return 0;
        }
        if out != 0 {
            write_i8(out as usize, r);
        }
        return 1;
    }
    let mp = tactics_row_match_player(ts, r, opp);
    let position = read_i8(mp + match_player::POSITION);
    if position < 0 && (read_u8(m + engine::SHOOTOUT_STATE) != 0 || position as u8 != 0xfe) {
        return 0;
    }
    if out != 0 {
        write_i8(out as usize, position);
        if position as u8 == 0xfe {
            covered!("tel15_3 pitch position of a player off injured");
            write_i8(out as usize, read_i8(mp + match_player::PREVIOUS_POSITION));
        }
    }
    1
}

/// The live match player of a row for the per-row getters, or None (the opponent's row without an opponent, no live
/// match).
unsafe fn tactics_live_row(ts: usize, row: i8, opponent: u32) -> Option<usize> {
    if opponent != 0 && read_usize(ts + tactics_screen_state::OPPONENT) == 0 {
        return None;
    }
    if read_usize(ts + tactics_screen::MATCH) == 0 {
        return None;
    }
    Some(tactics_row_match_player(ts, row, opponent != 0))
}

/// Whether a row holds an unused substitute of the live match (position -1).
///
/// A row out of 0..19 shows the error box of line 0x1bfe and returns 0; outside a live match, 0.
/// Original: `Tactics_Screen::is_slot_on_bench` (0x8aaaa0, tactics_screens.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn is_slot_on_bench(ts: usize, row: u32, opponent: u32) -> u32 {
    let r = row as u8 as i8;
    if !(0..=0x13).contains(&r) {
        tel26_error_box(TACTICS_SCREENS_CPP, 0x1bfe);
        return 0;
    }
    match tactics_live_row(ts, r, opponent) {
        Some(mp) => (read_u8(mp + match_player::POSITION) == 0xff) as u32,
        None => 0,
    }
}

/// The yellow cards of a row's player in the live match.
///
/// A row out of 0..19 shows the error box of line 0x1c31 and returns 0; outside a live match or for an empty row, 0.
/// Original: `Tactics_Screen::get_slot_yellow_cards` (0x8aabb0, tactics_screens.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn get_slot_yellow_cards(ts: usize, row: u32, opponent: u32) -> u8 {
    let r = row as u8 as i8;
    if !(0..=0x13).contains(&r) {
        tel26_error_box(TACTICS_SCREENS_CPP, 0x1c31);
        return 0;
    }
    match tactics_live_row(ts, r, opponent) {
        Some(mp) if read_u8(mp + match_player::POSITION) != 0xfb => read_u8(mp + match_player::YELLOW_CARDS),
        _ => 0,
    }
}

/// The straight red cards of a row's player in the live match (`match_player::RED_CARDS`).
///
/// A row out of 0..19 shows the error box of line 0x1c64 and returns 0; outside a live match or for an empty row, 0.
/// Original: `Tactics_Screen::get_slot_red_cards` (0x8aacb0, tactics_screens.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn get_slot_red_cards(ts: usize, row: u32, opponent: u32) -> u8 {
    let r = row as u8 as i8;
    if !(0..=0x13).contains(&r) {
        tel26_error_box(TACTICS_SCREENS_CPP, 0x1c64);
        return 0;
    }
    match tactics_live_row(ts, r, opponent) {
        Some(mp) if read_u8(mp + match_player::POSITION) != 0xfb => read_u8(mp + match_player::RED_CARDS),
        _ => 0,
    }
}

/// The match status of a row's player and its colour: injured, sent off, substituted or brought on, with the minute.
///
/// A row out of 0..19, a null `out` or `colour`, or a size under 20 shows the error box of line 0x1c9a and returns 0.
/// `out` is cleared and the colour set to 0xaebdea; the opponent's row without an opponent returns 0; outside a live
/// match or for an empty row, 1 with no text. Otherwise the first that applies: "inj <minute>" (`INJURY_MINUTE`),
/// "off <minute>" (`MINUTE_LEFT_PITCH`), "sub <minute>" (`MINUTE_OFF`), all in colour 0xae3154, or "on <minute>"
/// (`MINUTE_ON`) in colour 0xaebdd4 (minute + 1, through the language table). Returns 1.
/// Defect of the original: the text goes into the shared text buffer (`JOB_TEXT`), not into `out`, which stays
/// empty; the only caller passes `JOB_TEXT` as `out`.
/// Original: `Tactics_Screen::get_slot_match_status_text` (0x8aadb0, tactics_screens.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn get_slot_match_status_text(ts: usize, row: u32, out: u32, size: u32, colour: u32, opponent: u32) -> u32 {
    let r = row as u8 as i8;
    if !(0..=0x13).contains(&r) || out == 0 || (size as i32) < 0x14 || colour == 0 {
        tel26_error_box(TACTICS_SCREENS_CPP, 0x1c9a);
        return 0;
    }
    write_u8(out as usize, 0);
    write_u16(colour as usize, read_u16(JOB_COLOUR_BDEA));
    if opponent != 0 && read_usize(ts + tactics_screen_state::OPPONENT) == 0 {
        return 0;
    }
    let Some(mp) = tactics_live_row(ts, r, opponent) else { return 1 };
    if read_u8(mp + match_player::POSITION) == 0xfb {
        return 1;
    }
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let statuses = [(match_player::INJURY_MINUTE, 0xa16a68usize, TACTICS_COLOUR_3154), (match_player::MINUTE_LEFT_PITCH, 0xa16a54, TACTICS_COLOUR_3154),
                    (match_player::MINUTE_OFF, 0xa16a40, TACTICS_COLOUR_3154), (match_player::MINUTE_ON, 0xa16a2c, JOB_COLOUR_BDD4)];
    for (field, format, shade) in statuses {
        let minute = read_i8(mp + field);
        if minute > -1 {
            covered!("tel15_3 a row's match status");
            text(JOB_TEXT, format, minute as i32 + 1);
            write_u16(colour as usize, read_u16(shade));
            return 1;
        }
    }
    1
}

// The comparers of the edits run side by side over the screen object; the getters, over their outputs (the box
// cases, original only).

/// Comparer for `set_formation_slot_coordinate`.
pub(crate) unsafe extern "thiscall" fn compare_set_formation_slot_coordinate(ts: usize, slot: u32, coordinate: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u32>(%%I0%%);
    if !(0..=10).contains(&(slot as u8 as i8)) {
        return o(ts, slot, coordinate); // simplification: the box, original only
    }
    compare_tactics_method("set_formation_slot_coordinate", &[tactics_screen_region(ts)], || o(ts, slot, coordinate), || set_formation_slot_coordinate(ts, slot, coordinate))
}

/// Comparer for `is_formation_slot_customised`.
pub(crate) unsafe extern "thiscall" fn compare_is_formation_slot_customised(ts: usize, slot: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(%%I1%%);
    if !(0..=10).contains(&(slot as u8 as i8)) {
        return o(ts, slot); // simplification: the box, original only
    }
    compare_tactics_method("is_formation_slot_customised", &[], || o(ts, slot), || is_formation_slot_customised(ts, slot))
}

/// Comparer for `set_slot_marking_type`.
pub(crate) unsafe extern "thiscall" fn compare_set_slot_marking_type(ts: usize, slot: u32, marking: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u32>(%%I2%%);
    if !(0..=10).contains(&(slot as u8 as i8)) {
        return o(ts, slot, marking); // simplification: the box, original only
    }
    compare_tactics_method("set_slot_marking_type", &[tactics_screen_region(ts)], || o(ts, slot, marking), || set_slot_marking_type(ts, slot, marking))
}

/// Comparer for `set_slot_marking_target`.
pub(crate) unsafe extern "thiscall" fn compare_set_slot_marking_target(ts: usize, slot: u32, target: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u32>(%%I3%%);
    if !(0..=10).contains(&(slot as u8 as i8)) {
        return o(ts, slot, target); // simplification: the box, original only
    }
    compare_tactics_method("set_slot_marking_target", &[tactics_screen_region(ts)], || o(ts, slot, target), || set_slot_marking_target(ts, slot, target))
}

/// Comparer for `get_slot_number_text`.
pub(crate) unsafe extern "thiscall" fn compare_get_slot_number_text(ts: usize, row: u32, out: u32, size: u32, no_fallback: u32, opponent: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> u32>(%%I4%%);
    if !(0..=0x13).contains(&(row as u8 as i8)) || out == 0 || (size as i32) < 0xa {
        return o(ts, row, out, size, no_fallback, opponent); // simplification: the box, original only
    }
    compare_tactics_method("get_slot_number_text", &[(out as usize, size.min(0x100) as usize)], || o(ts, row, out, size, no_fallback, opponent),
                           || get_slot_number_text(ts, row, out, size, no_fallback, opponent))
}

/// Comparer for `get_slot_player_name_text`.
pub(crate) unsafe extern "thiscall" fn compare_get_slot_player_name_text(ts: usize, row: u32, out: u32, size: u32, no_fallback: u32, format: u32, opponent: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32) -> u32>(%%I5%%);
    if !(0..=0x13).contains(&(row as u8 as i8)) || out == 0 || (size as i32) < 0x64 {
        return o(ts, row, out, size, no_fallback, format, opponent); // simplification: the box, original only
    }
    compare_tactics_method("get_slot_player_name_text", &[(out as usize, size.min(0x100) as usize)], || o(ts, row, out, size, no_fallback, format, opponent),
                           || get_slot_player_name_text(ts, row, out, size, no_fallback, format, opponent))
}

/// Comparer for `get_slot_number_and_name_text`.
pub(crate) unsafe extern "thiscall" fn compare_get_slot_number_and_name_text(ts: usize, row: u32, out: u32, size: u32, format: u32, opponent: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> u32>(%%I6%%);
    if !(0..=0x13).contains(&(row as u8 as i8)) || out == 0 || (size as i32) < 0x64 {
        return o(ts, row, out, size, format, opponent); // simplification: the box, original only
    }
    compare_tactics_method("get_slot_number_and_name_text", &[(out as usize, size.min(0x100) as usize)], || o(ts, row, out, size, format, opponent),
                           || get_slot_number_and_name_text(ts, row, out, size, format, opponent))
}

/// Comparer for `get_slot_pitch_position`.
pub(crate) unsafe extern "thiscall" fn compare_get_slot_pitch_position(ts: usize, row: u32, out: u32, opponent: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32) -> u32>(%%I7%%);
    let reg: &[(usize, usize)] = if out != 0 { &[(out as usize, 1)] } else { &[] };
    compare_tactics_method("get_slot_pitch_position", reg, || o(ts, row, out, opponent), || get_slot_pitch_position(ts, row, out, opponent))
}

/// Comparer for `is_slot_on_bench`.
pub(crate) unsafe extern "thiscall" fn compare_is_slot_on_bench(ts: usize, row: u32, opponent: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u32>(%%I8%%);
    if !(0..=0x13).contains(&(row as u8 as i8)) {
        return o(ts, row, opponent); // simplification: the box, original only
    }
    compare_tactics_method("is_slot_on_bench", &[], || o(ts, row, opponent), || is_slot_on_bench(ts, row, opponent))
}

/// Comparer for `get_slot_yellow_cards`.
pub(crate) unsafe extern "thiscall" fn compare_get_slot_yellow_cards(ts: usize, row: u32, opponent: u32) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u8>(%%I9%%);
    if !(0..=0x13).contains(&(row as u8 as i8)) {
        return o(ts, row, opponent); // simplification: the box, original only
    }
    compare_tactics_method("get_slot_yellow_cards", &[], || o(ts, row, opponent), || get_slot_yellow_cards(ts, row, opponent))
}

/// Comparer for `get_slot_red_cards`.
pub(crate) unsafe extern "thiscall" fn compare_get_slot_red_cards(ts: usize, row: u32, opponent: u32) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u8>(%%I10%%);
    if !(0..=0x13).contains(&(row as u8 as i8)) {
        return o(ts, row, opponent); // simplification: the box, original only
    }
    compare_tactics_method("get_slot_red_cards", &[], || o(ts, row, opponent), || get_slot_red_cards(ts, row, opponent))
}

/// Comparer for `get_slot_match_status_text`: over `out`, the colour and the shared text.
pub(crate) unsafe extern "thiscall" fn compare_get_slot_match_status_text(ts: usize, row: u32, out: u32, size: u32, colour: u32, opponent: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> u32>(%%I11%%);
    if !(0..=0x13).contains(&(row as u8 as i8)) || out == 0 || (size as i32) < 0x14 || colour == 0 {
        return o(ts, row, out, size, colour, opponent); // simplification: the box, original only
    }
    compare_tactics_method("get_slot_match_status_text", &[(out as usize, 1), (colour as usize, 2), (JOB_TEXT, 0x40)], || o(ts, row, out, size, colour, opponent),
                           || get_slot_match_status_text(ts, row, out, size, colour, opponent))
}
