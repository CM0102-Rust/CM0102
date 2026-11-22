// ---------- batch %%LEVA%%: the tactics screen object (tactics_screens.cpp, 6): the row values, the marking and the substitutions ----------

/// The condition (percent) of a row's player.
///
/// A row out of 0..19 shows the error box of line 0x1cf0 and returns 0; so does the opponent's row without an
/// opponent. In a live match: the match player's stamina x 0.01 (truncated; 0 for an empty row). Outside a match and
/// without a match record: the row's selected player (the opponent's predicted line-up, or
/// `Tactics::get_selected_player` with the reserve flag): -1 (0xff) while injured (`Injury::get_player_injury_status`
/// 2), else `Injury::get_condition` in percent; 0 with a match record or no player. The zero returns only clear al.
/// Original: `Tactics_Screen::get_slot_condition` (0x8aafd0, tactics_screens.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn get_slot_condition(ts: usize, row: u32, opponent: u32) -> u32 {
    let r = row as u8 as i8;
    if !(0..=0x13).contains(&r) {
        tel26_error_box(TACTICS_SCREENS_CPP, 0x1cf0);
        return 0;
    }
    let opp = opponent != 0;
    if opp && read_usize(ts + tactics_screen_state::OPPONENT) == 0 {
        return 0;
    }
    if read_usize(ts + tactics_screen::MATCH) != 0 {
        let mp = tactics_row_match_player(ts, r, opp);
        if read_u8(mp + match_player::POSITION) == 0xfb {
            return 0;
        }
        covered!("tel15_4 condition in a live match");
        return ftol(read_f32(mp + match_player::STAMINA) as f64 * read_f64(rdata::F64_0_01)) as u32;
    }
    if read_i32(ts + tactics_screen_state::RECORD_INDEX) != -1 {
        return 0;
    }
    let staff = if opp {
        read_usize(ts + tactics_screen_state::OPPONENT_LINEUP + r as usize * 4)
    } else {
        exe::tactics_get_selected_player(read_usize(G_TACTICS), read_usize(ts + tactics_screen::CLUB), row, read_u32(ts + tactics_screen_state::RESERVE) as i32)
    };
    if staff == 0 {
        return 0;
    }
    if exe::injury_get_player_injury_status(read_usize(G_INJURY), staff) == 2 {
        covered!("tel15_4 condition of an injured player");
        return 0xff;
    }
    covered!("tel15_4 condition outside a match");
    exe::injury_get_condition(read_usize(G_INJURY), staff, 1) as u16 as u32
}

/// The match rating of a row's player in the live match (`match_player::MATCH_GRADE`).
///
/// A row out of 0..19 shows the error box of line 0x1d38 and returns 0; outside a live match, for an empty row or an
/// unused substitute (-1), 0.
/// Original: `Tactics_Screen::get_slot_match_rating` (0x8ab150, tactics_screens.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn get_slot_match_rating(ts: usize, row: u32, opponent: u32) -> u8 {
    let r = row as u8 as i8;
    if !(0..=0x13).contains(&r) {
        tel26_error_box(TACTICS_SCREENS_CPP, 0x1d38);
        return 0;
    }
    match tactics_live_row(ts, r, opponent) {
        Some(mp) if !matches!(read_u8(mp + match_player::POSITION), 0xfb | 0xff) => read_u8(mp + match_player::MATCH_GRADE),
        _ => 0,
    }
}

/// The goals of a row's player in the live match.
///
/// A row out of 0..19 shows the error box of line 0x1d6f and returns 0; outside a live match or for an empty row, 0.
/// Original: `Tactics_Screen::get_slot_goals` (0x8ab260, tactics_screens.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn get_slot_goals(ts: usize, row: u32, opponent: u32) -> u8 {
    let r = row as u8 as i8;
    if !(0..=0x13).contains(&r) {
        tel26_error_box(TACTICS_SCREENS_CPP, 0x1d6f);
        return 0;
    }
    match tactics_live_row(ts, r, opponent) {
        Some(mp) if read_u8(mp + match_player::POSITION) != 0xfb => read_u8(mp + match_player::GOALS),
        _ => 0,
    }
}

/// Whether a row's player is the one the live match marks as its man of the match: his staff id is the engine's
/// `ADJUSTED_PLAYER_ID`.
///
/// A row out of 0..19 shows the error box of line 0x1da3 and returns 0; outside a live match, for an empty row or a
/// row without a staff, 0.
/// Original: `Tactics_Screen::is_slot_man_of_the_match` (0x8ab360, tactics_screens.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn is_slot_man_of_the_match(ts: usize, row: u32, opponent: u32) -> u32 {
    let r = row as u8 as i8;
    if !(0..=0x13).contains(&r) {
        tel26_error_box(TACTICS_SCREENS_CPP, 0x1da3);
        return 0;
    }
    let Some(mp) = tactics_live_row(ts, r, opponent) else { return 0 };
    if read_u8(mp + match_player::POSITION) == 0xfb {
        return 0;
    }
    let staff = read_usize(mp + match_player::STAFF);
    (staff != 0 && read_u32(staff) == read_u32(read_usize(ts + tactics_screen::MATCH) + engine::ADJUSTED_PLAYER_ID)) as u32
}

/// The staff record of a row's player, or 0.
///
/// A row out of 0..19 shows the error box of line 0x1dde and returns 0; so does the opponent's row without an
/// opponent. With a match record: the record row's staff (by its id, below the staff count) when it is filled and
/// the staff still has the row's names and date of birth (`database_staff_matches_names`). In a live match: the row's
/// match player's staff (0 for an empty row). Otherwise the opponent's predicted line-up or the club's selected
/// player (`Tactics::get_selected_player` with the reserve flag).
/// Original: `Tactics_Screen::get_slot_player` (0x8ab480, tactics_screens.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn get_slot_player(ts: usize, row: u32, opponent: u32) -> usize {
    let r = row as u8 as i8;
    if !(0..=0x13).contains(&r) {
        tel26_error_box(TACTICS_SCREENS_CPP, 0x1dde);
        return 0;
    }
    let opp = opponent != 0;
    if opp && read_usize(ts + tactics_screen_state::OPPONENT) == 0 {
        return 0;
    }
    if read_i32(ts + tactics_screen_state::RECORD_INDEX) > -1 {
        let rr = tactics_record_row(ts, r, opp);
        if read_u8(rr + match_record_row::POSITION) == 0xfb {
            return 0;
        }
        let id = read_i32(rr + match_record_row::STAFF_ID);
        if id < 0 || id >= read_i32(G_STAFF_COUNT) {
            return 0;
        }
        let staff = read_usize(G_STAFF_TABLE) + id as usize * staff::SIZE;
        let matches: unsafe extern "C" fn(usize, u32, u32, u32, usize) -> u32 = core::mem::transmute(0x53e990usize);
        if matches(staff, read_u32(rr + match_record_row::FIRST_NAME_ID), read_u32(rr + match_record_row::SECOND_NAME_ID),
                   read_u32(rr + match_record_row::COMMON_NAME_ID), rr + match_record_row::DATE_OF_BIRTH) == 0 {
            return 0;
        }
        covered!("tel15_4 player of a match record row");
        return staff;
    }
    if read_usize(ts + tactics_screen::MATCH) != 0 {
        let mp = tactics_row_match_player(ts, r, opp);
        if read_u8(mp + match_player::POSITION) == 0xfb {
            return 0;
        }
        return read_usize(mp + match_player::STAFF);
    }
    if opp {
        return read_usize(ts + tactics_screen_state::OPPONENT_LINEUP + r as usize * 4);
    }
    exe::tactics_get_selected_player(read_usize(G_TACTICS), read_usize(ts + tactics_screen::CLUB), row, read_u32(ts + tactics_screen_state::RESERVE) as i32)
}

/// The row playing a formation position (0..10), or -1; `pending` set when that row left the pitch injured (-2) and
/// the position is its previous one.
///
/// A position out of range shows the error box of line 0x1e2b and returns -1; so does the opponent's row without an
/// opponent. `pending` is cleared first. Outside a live match the row is the position itself; in a live match the
/// first of the 20 rows whose match player holds the position.
/// Original: `Tactics_Screen::find_slot_at_position` (0x8ab670, tactics_screens.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn find_slot_at_position(ts: usize, position: u32, pending: u32, opponent: u32) -> u8 {
    let p = position as u8 as i8;
    if !(0..=10).contains(&p) {
        tel26_error_box(TACTICS_SCREENS_CPP, 0x1e2b);
        return 0xff;
    }
    let opp = opponent != 0;
    if opp && read_usize(ts + tactics_screen_state::OPPONENT) == 0 {
        return 0xff;
    }
    write_u32(pending as usize, 0);
    if read_usize(ts + tactics_screen::MATCH) == 0 {
        return p as u8;
    }
    for row in 0..0x14i8 {
        let mp = tactics_row_match_player(ts, row, opp);
        let at = read_i8(mp + match_player::POSITION);
        if at == p {
            return row as u8;
        }
        if at as u8 == 0xfe && read_i8(mp + match_player::PREVIOUS_POSITION) == p {
            covered!("tel15_4 position of a player off injured");
            write_u32(pending as usize, 1);
            return row as u8;
        }
    }
    0xff
}

/// A pitch position counts for a swap: 0..10, or -2 (off injured: its previous position).
fn tactics_swappable(position: i8) -> bool {
    position >= 0 || position as u8 == 0xfe
}

/// Swaps two rows (0..19) on the tactics screen: a substitution or a change of formation slots.
///
/// A row out of range shows the error box of line 0x1e79 and returns 0; so does the opponent's row without an
/// opponent. In a live match: an unused substitute (-1) and a player on the pitch (or off injured) queue a
/// substitution (`queue_substitution`, the substitute first) and return 1; two players on the pitch (or off injured)
/// swap their formation slots (`Formation::swap_slots`, the previous position for one off injured) after the undo
/// copy, the team data marked changed and 1 returned; anything else returns 0. Outside a live match, two rows of the
/// eleven swap their slots the same way; any other pair returns 0.
/// Original: `Tactics_Screen::swap_slots` (0x8ab7e0, tactics_screens.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn swap_slots(ts: usize, a: u32, b: u32, opponent: u32) -> u32 {
    let (ra, rb) = (a as u8 as i8, b as u8 as i8);
    if !(0..=0x13).contains(&ra) || !(0..=0x13).contains(&rb) {
        tel26_error_box(TACTICS_SCREENS_CPP, 0x1e79);
        return 0;
    }
    let opp = opponent != 0;
    if opp && read_usize(ts + tactics_screen_state::OPPONENT) == 0 {
        return 0;
    }
    let fm = ts + tactics_screen::FORMATION;
    if read_usize(ts + tactics_screen::MATCH) == 0 {
        if ra >= 0xb || rb >= 0xb {
            return 0;
        }
        covered!("tel15_4 swap two slots outside a match");
        tactics_save_undo(ts);
        tactics_screen_calls::call2(0x5b2f30, fm, a, b);
        write_u32(ts + tactics_screen_state::TEAM_DATA_CHANGED, 1);
        return 1;
    }
    let (pa, pb) = (tactics_row_match_player(ts, ra, opp), tactics_row_match_player(ts, rb, opp));
    let (ca, cb) = (read_i8(pa + match_player::POSITION), read_i8(pb + match_player::POSITION));
    if ca == -1 && tactics_swappable(cb) {
        covered!("tel15_4 swap queues a substitution");
        tactics_screen_calls::call2(0x8ac100, ts, ra as u8 as u32, rb as u8 as u32);
        return 1;
    }
    if tactics_swappable(ca) && cb == -1 {
        covered!("tel15_4 swap queues a substitution (reversed)");
        tactics_screen_calls::call2(0x8ac100, ts, rb as u8 as u32, ra as u8 as u32);
        return 1;
    }
    if !tactics_swappable(ca) || !tactics_swappable(cb) {
        return 0;
    }
    covered!("tel15_4 swap two slots in a match");
    tactics_save_undo(ts);
    let slot = |mp: usize, c: i8| if c as u8 == 0xfe { read_i8(mp + match_player::PREVIOUS_POSITION) } else { c };
    tactics_screen_calls::call2(0x5b2f30, fm, slot(pa, ca) as u8 as u32, slot(pb, cb) as u8 as u32);
    write_u32(ts + tactics_screen_state::TEAM_DATA_CHANGED, 1);
    1
}

/// The opposing row a row marks in the live match (`CM3_MATCH_ENGINE::find_marking_target` with the row's side), or
/// -1.
///
/// A row out of 0..19 shows the error box of line 0x1ee5 and returns -1; outside a live match or for the opponent's
/// row without an opponent, -1.
/// Original: `Tactics_Screen::get_slot_marking_target_row` (0x8aba70, tactics_screens.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn get_slot_marking_target_row(ts: usize, row: u32, opponent: u32) -> u8 {
    if !(0..=0x13).contains(&(row as u8 as i8)) {
        tel26_error_box(TACTICS_SCREENS_CPP, 0x1ee5);
        return 0xff;
    }
    if opponent != 0 && read_usize(ts + tactics_screen_state::OPPONENT) == 0 {
        return 0xff;
    }
    let m = read_usize(ts + tactics_screen::MATCH);
    if m == 0 {
        return 0xff;
    }
    let side = read_u8(ts + if opponent != 0 { tactics_screen_state::OPPONENT_SIDE } else { tactics_screen::SIDE });
    tactics_screen_calls::call2(0x6d4330, m, side as u32, row) as u8
}

/// The formation of a side for the marking getters: the screen's own, or the opponent's in the live match.
unsafe fn tactics_side_formation(ts: usize, opponent: u32) -> usize {
    if opponent == 0 {
        return ts + tactics_screen::FORMATION;
    }
    let side = read_i8(ts + tactics_screen_state::OPPONENT_SIDE) as isize;
    (read_usize(ts + tactics_screen::MATCH) as isize + engine::TACTICS as isize + side * tactics_screen_state::FORMATION_SIZE as isize) as usize
}

/// The opposing player a row's player man-marks, or 0.
///
/// A row out of 0..19 shows the error box of line 0x1f11 and returns 0. Nothing (0) without an opponent, when the
/// row is off the pitch or has no player, or when its slot does not man-mark (marking type bit 4,
/// `Formation::get_marking_type` of its side's formation). Bit 8: the slot's set target
/// (`Formation::get_marking_target`); bit 0x10: the player of the row the match assigns
/// (`get_slot_marking_target_row`, on the other side).
/// Original: `Tactics_Screen::get_slot_marked_player` (0x8abb50, tactics_screens.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn get_slot_marked_player(ts: usize, row: u32, opponent: u32) -> usize {
    if !(0..=0x13).contains(&(row as u8 as i8)) {
        tel26_error_box(TACTICS_SCREENS_CPP, 0x1f11);
        return 0;
    }
    if read_usize(ts + tactics_screen_state::OPPONENT) == 0 {
        return 0;
    }
    let mut place = 0u8;
    if tactics_screen_calls::call3(tactics_screen_fns::GET_SLOT_PITCH_POSITION, ts, row, &mut place as *mut u8 as u32, opponent) == 0 {
        return 0;
    }
    if tactics_screen_calls::call2(tactics_screen_fns::GET_SLOT_PLAYER, ts, row, opponent) == 0 {
        return 0;
    }
    let fm = tactics_side_formation(ts, opponent);
    let marking = tactics_screen_calls::call1(0x5b57c0, fm, place as u32) as u8;
    if marking & 4 == 0 {
        return 0;
    }
    if marking & 8 != 0 {
        covered!("tel15_4 marked player: the set target");
        return tactics_screen_calls::call1(0x5b58e0, fm, place as u32) as usize;
    }
    if marking & 0x10 == 0 {
        return 0;
    }
    let target = tactics_screen_calls::call2(tactics_screen_fns::GET_SLOT_MARKING_TARGET_ROW, ts, row, opponent) as u8 as i8;
    if target <= -1 {
        return 0;
    }
    covered!("tel15_4 marked player: the match's target");
    tactics_screen_calls::call2(tactics_screen_fns::GET_SLOT_PLAYER, ts, target as u8 as u32, (opponent == 0) as u32) as usize
}

/// The opposing player who man-marks a row's player, or 0.
///
/// A row out of 0..19 shows the error box of line 0x1f56 and returns 0. Nothing without an opponent, when the row is
/// off the pitch or has no player. Otherwise the player of the first row of the other side on the pitch whose marked
/// player (`get_slot_marked_player`) is this row's.
/// Original: `Tactics_Screen::get_slot_marker` (0x8abd00, tactics_screens.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn get_slot_marker(ts: usize, row: u32, opponent: u32) -> usize {
    if !(0..=0x13).contains(&(row as u8 as i8)) {
        tel26_error_box(TACTICS_SCREENS_CPP, 0x1f56);
        return 0;
    }
    if read_usize(ts + tactics_screen_state::OPPONENT) == 0 {
        return 0;
    }
    if tactics_screen_calls::call3(tactics_screen_fns::GET_SLOT_PITCH_POSITION, ts, row, 0, opponent) == 0 {
        return 0;
    }
    let me = tactics_screen_calls::call2(tactics_screen_fns::GET_SLOT_PLAYER, ts, row, opponent) as usize;
    if me == 0 {
        return 0;
    }
    let other = (opponent == 0) as u32;
    for k in 0..0x14u32 {
        if tactics_screen_calls::call3(tactics_screen_fns::GET_SLOT_PITCH_POSITION, ts, k, 0, other) != 0
            && tactics_screen_calls::call2(0x8abb50, ts, k, other) as usize == me {
            covered!("tel15_4 the marker of a row");
            return tactics_screen_calls::call2(tactics_screen_fns::GET_SLOT_PLAYER, ts, k, other) as usize;
        }
    }
    0
}

/// The minute of the live match for a substitution: the clock / 11, capped at the end of the period (45, 90, 105 or
/// 120 when the period ends at tick 0x1ef, 0x3de, 0x483 or 0x528).
unsafe fn tactics_substitution_minute(m: usize) -> i8 {
    let minute = (read_i16(m + engine::CLOCK) as i32 / 11) as i8;
    let cap = match read_i16(m + engine::PERIOD_END) {
        0x1ef => 0x2d,
        0x3de => 0x5a,
        0x483 => 0x69,
        0x528 => 0x78,
        _ => return minute,
    };
    if minute < cap { minute } else { cap }
}

/// The coming-on and going-off marks of a substitution between two match players: the minute and position of the
/// one coming on (the other's position, its previous one when off injured), the other substituted off (-4) at that
/// minute.
unsafe fn tactics_mark_substitution(on: usize, off: usize, m: usize) {
    write_i8(on + match_player::MINUTE_ON, tactics_substitution_minute(m));
    let position = read_i8(off + match_player::POSITION);
    write_i8(on + match_player::POSITION, if position as u8 == 0xfe { read_i8(off + match_player::PREVIOUS_POSITION) } else { position });
}

/// Queues a substitution on the screen's copy of the match players: `on` (a substitute row) replaces `off`.
///
/// A row out of 0..19 shows the error box of line 0x1fde and returns 0; outside a live match, 0. The undo copy is
/// saved. When `off` came on in a substitution already queued on this screen, that one is undone (both rows'
/// match players copied back from the live match) and replaced: its substitute becomes `on`, its player going off
/// stays. Otherwise the pair is added to the queue (on rows at +0x4a0d, off rows at +0x4a16, the count in
/// `SUBS_MADE`). Then the marks: `on` comes on at the match minute in the position of the player going off, who is
/// substituted off (-4, `MINUTE_OFF`, `SUBSTITUTED_OFF`). The team data is marked changed; returns 1.
/// Original: `Tactics_Screen::queue_substitution` (0x8ac100, tactics_screens.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn queue_substitution(ts: usize, on: u32, off: u32) -> u32 {
    let (on, mut off) = (on as u8 as i8, off as u8 as i8);
    if !(0..=0x13).contains(&on) || !(0..=0x13).contains(&off) {
        tel26_error_box(TACTICS_SCREENS_CPP, 0x1fde);
        return 0;
    }
    let m = read_usize(ts + tactics_screen::MATCH);
    if m == 0 {
        return 0;
    }
    tactics_save_undo(ts);
    let (ons, offs) = (ts + 0x4a0d, ts + 0x4a16);
    let made = read_i8(ts + tactics_screen::SUBS_MADE);
    let queued = (0..made.max(0)).find(|&k| read_i8(ons + k as usize) == off);
    match queued {
        Some(k) => {
            covered!("tel15_4 a queued substitution replaced");
            for row in [read_i8(ons + k as usize), read_i8(offs + k as usize)] {
                core::ptr::copy(tactics_row_live_player(ts, row) as *const u8, tactics_row_match_player(ts, row, false) as *mut u8, 0x1be);
            }
            write_i8(ons + k as usize, on);
            off = read_i8(offs + k as usize);
        }
        None => {
            covered!("tel15_4 a substitution queued");
            write_i8(ons + made as usize, on);
            write_i8(offs + read_i8(ts + tactics_screen::SUBS_MADE) as usize, off);
            write_i8(ts + tactics_screen::SUBS_MADE, read_i8(ts + tactics_screen::SUBS_MADE).wrapping_add(1));
        }
    }
    let (pon, poff) = (tactics_row_match_player(ts, on, false), tactics_row_match_player(ts, off, false));
    tactics_mark_substitution(pon, poff, m);
    write_i8(poff + match_player::POSITION, -4);
    write_i8(poff + match_player::MINUTE_OFF, read_i8(pon + match_player::MINUTE_ON));
    write_u8(poff + match_player::SUBSTITUTED_OFF, 1);
    write_u32(ts + tactics_screen_state::TEAM_DATA_CHANGED, 1);
    1
}

/// The live match player of a row of the club's side (`SIDE`).
unsafe fn tactics_row_live_player(ts: usize, row: i8) -> usize {
    let k = read_i8(ts + tactics_screen::SIDE) as isize * 20 + row as isize;
    (read_usize(ts + tactics_screen::MATCH) as isize + engine::PLAYERS as isize + k * 0x1be) as usize
}

/// Makes a substitution in the live match: the club's row `on` replaces row `off`.
///
/// A row out of 0..19 shows the error box of line 0x2055 and returns 0; outside a live match, 0. The substitute is
/// placed at his entry cell (`CM3_MATCH_ENGINE::place_player_at_entry_cell`), comes on at the match minute in the
/// position of the player going off, who leaves the pitch (`remove_player_from_pitch`, reason -1) substituted off
/// (-4, `MINUTE_OFF`, `SUBSTITUTED_OFF`); the side's substitutions left drop by one and the commentary gets event
/// 0x20d7 (the player going off was injured) or 0x1f5b (`Match_Events::add_event` with both rows and the side).
/// Returns 1.
/// Original: `Tactics_Screen::apply_substitution` (0x8ac3d0, tactics_screens.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn apply_substitution(ts: usize, on: u32, off: u32) -> u32 {
    let (ron, roff) = (on as u8 as i8, off as u8 as i8);
    if !(0..=0x13).contains(&ron) || !(0..=0x13).contains(&roff) {
        tel26_error_box(TACTICS_SCREENS_CPP, 0x2055);
        return 0;
    }
    let m = read_usize(ts + tactics_screen::MATCH);
    if m == 0 {
        return 0;
    }
    covered!("tel15_4 a substitution in the live match");
    let (pon, poff) = (tactics_row_live_player(ts, ron), tactics_row_live_player(ts, roff));
    tactics_screen_calls::call1(0x6cdd00, m, pon as u32);
    tactics_mark_substitution(pon, poff, m);
    tactics_screen_calls::call2(0x6cdd50, m, poff as u32, 0xffff_ffff);
    write_i8(poff + match_player::POSITION, -4);
    write_i8(poff + match_player::MINUTE_OFF, read_i8(pon + match_player::MINUTE_ON));
    write_u8(poff + match_player::SUBSTITUTED_OFF, 1);
    let side = read_u8(ts + tactics_screen::SIDE);
    let manager = (m as isize + engine::MATCH_MANAGERS as isize + side as i8 as isize * 0xd0) as usize;
    write_u8(manager + match_manager::SUBSTITUTIONS_LEFT, read_u8(manager + match_manager::SUBSTITUTIONS_LEFT).wrapping_sub(1));
    let event: u32 = if read_i8(poff + match_player::INJURY_MINUTE) > -1 { 0x20d7 } else { 0x1f5b };
    let add: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32, u32, u32, u32) = core::mem::transmute(0x6d6f70usize);
    add(read_usize(m + engine::COMMENTATOR), event, m, on, side as u32, off, side as u32, 0);
    1
}

// The getters compare side by side over nothing but their result; the edits over the screen object; the box cases,
// original only.

/// Comparer for `get_slot_condition` (al only when the original clears just al).
pub(crate) unsafe extern "thiscall" fn compare_get_slot_condition(ts: usize, row: u32, opponent: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u32>(%%I0%%);
    if !(0..=0x13).contains(&(row as u8 as i8)) {
        return o(ts, row, opponent); // simplification: the box, original only
    }
    let mut r = 0;
    compare_tactics_method("get_slot_condition", &[], || { r = o(ts, row, opponent); r as u8 }, || get_slot_condition(ts, row, opponent) as u8);
    r
}

/// Comparer for `get_slot_match_rating`.
pub(crate) unsafe extern "thiscall" fn compare_get_slot_match_rating(ts: usize, row: u32, opponent: u32) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u8>(%%I1%%);
    if !(0..=0x13).contains(&(row as u8 as i8)) {
        return o(ts, row, opponent); // simplification: the box, original only
    }
    compare_tactics_method("get_slot_match_rating", &[], || o(ts, row, opponent), || get_slot_match_rating(ts, row, opponent))
}

/// Comparer for `get_slot_goals`.
pub(crate) unsafe extern "thiscall" fn compare_get_slot_goals(ts: usize, row: u32, opponent: u32) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u8>(%%I2%%);
    if !(0..=0x13).contains(&(row as u8 as i8)) {
        return o(ts, row, opponent); // simplification: the box, original only
    }
    compare_tactics_method("get_slot_goals", &[], || o(ts, row, opponent), || get_slot_goals(ts, row, opponent))
}

/// Comparer for `is_slot_man_of_the_match`.
pub(crate) unsafe extern "thiscall" fn compare_is_slot_man_of_the_match(ts: usize, row: u32, opponent: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u32>(%%I3%%);
    if !(0..=0x13).contains(&(row as u8 as i8)) {
        return o(ts, row, opponent); // simplification: the box, original only
    }
    compare_tactics_method("is_slot_man_of_the_match", &[], || o(ts, row, opponent), || is_slot_man_of_the_match(ts, row, opponent))
}

/// Comparer for `get_slot_player`.
pub(crate) unsafe extern "thiscall" fn compare_get_slot_player(ts: usize, row: u32, opponent: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> usize>(%%I4%%);
    if !(0..=0x13).contains(&(row as u8 as i8)) {
        return o(ts, row, opponent); // simplification: the box, original only
    }
    compare_tactics_method("get_slot_player", &[], || o(ts, row, opponent), || get_slot_player(ts, row, opponent))
}

/// Comparer for `find_slot_at_position`.
pub(crate) unsafe extern "thiscall" fn compare_find_slot_at_position(ts: usize, position: u32, pending: u32, opponent: u32) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32) -> u8>(%%I5%%);
    if !(0..=10).contains(&(position as u8 as i8)) || pending == 0 {
        return o(ts, position, pending, opponent); // simplification: the box, original only
    }
    compare_tactics_method("find_slot_at_position", &[(pending as usize, 4)], || o(ts, position, pending, opponent), || find_slot_at_position(ts, position, pending, opponent))
}

/// Comparer for `swap_slots`.
pub(crate) unsafe extern "thiscall" fn compare_swap_slots(ts: usize, a: u32, b: u32, opponent: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32) -> u32>(%%I6%%);
    if !(0..=0x13).contains(&(a as u8 as i8)) || !(0..=0x13).contains(&(b as u8 as i8)) {
        return o(ts, a, b, opponent); // simplification: the box, original only
    }
    compare_tactics_method("swap_slots", &[tactics_screen_region(ts)], || o(ts, a, b, opponent), || swap_slots(ts, a, b, opponent))
}

/// Comparer for `get_slot_marking_target_row`.
pub(crate) unsafe extern "thiscall" fn compare_get_slot_marking_target_row(ts: usize, row: u32, opponent: u32) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u8>(%%I7%%);
    if !(0..=0x13).contains(&(row as u8 as i8)) {
        return o(ts, row, opponent); // simplification: the box, original only
    }
    compare_tactics_method("get_slot_marking_target_row", &[], || o(ts, row, opponent), || get_slot_marking_target_row(ts, row, opponent))
}

/// Comparer for `get_slot_marked_player`.
pub(crate) unsafe extern "thiscall" fn compare_get_slot_marked_player(ts: usize, row: u32, opponent: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> usize>(%%I8%%);
    if !(0..=0x13).contains(&(row as u8 as i8)) {
        return o(ts, row, opponent); // simplification: the box, original only
    }
    compare_tactics_method("get_slot_marked_player", &[], || o(ts, row, opponent), || get_slot_marked_player(ts, row, opponent))
}

/// Comparer for `get_slot_marker`.
pub(crate) unsafe extern "thiscall" fn compare_get_slot_marker(ts: usize, row: u32, opponent: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> usize>(%%I9%%);
    if !(0..=0x13).contains(&(row as u8 as i8)) {
        return o(ts, row, opponent); // simplification: the box, original only
    }
    compare_tactics_method("get_slot_marker", &[], || o(ts, row, opponent), || get_slot_marker(ts, row, opponent))
}

/// Comparer for `queue_substitution`.
pub(crate) unsafe extern "thiscall" fn compare_queue_substitution(ts: usize, on: u32, off: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u32>(%%I10%%);
    if !(0..=0x13).contains(&(on as u8 as i8)) || !(0..=0x13).contains(&(off as u8 as i8)) {
        return o(ts, on, off); // simplification: the box, original only
    }
    compare_tactics_method("queue_substitution", &[tactics_screen_region(ts)], || o(ts, on, off), || queue_substitution(ts, on, off))
}

// simplification: moves players in the live match and adds commentary: only the original runs; the plan: the player
// going off is substituted off (-4) with the minute the substitute came on.
/// Comparer for `apply_substitution`.
pub(crate) unsafe extern "thiscall" fn compare_apply_substitution(ts: usize, on: u32, off: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u32>(%%I11%%);
    let r = o(ts, on, off);
    if r != 0 {
        let (pon, poff) = (tactics_row_live_player(ts, on as u8 as i8), tactics_row_live_player(ts, off as u8 as i8));
        let ok = read_i8(poff + match_player::POSITION) == -4 && read_u8(poff + match_player::SUBSTITUTED_OFF) == 1
            && read_i8(poff + match_player::MINUTE_OFF) == read_i8(pon + match_player::MINUTE_ON);
        SCOREBOARD.record(ok, || format!("tactics screen apply_substitution({}, {}): the player going off is not substituted off", on as u8 as i8, off as u8 as i8));
    }
    r
}
