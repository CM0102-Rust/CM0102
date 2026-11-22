// ---------- batch %%LEVA%%: the tactics screen object (tactics_screens.cpp, 3): building, copying, applying and freeing it ----------

/// More fields of the Tactics_Screen object (tel14_1's `tactics_screen`), named from its methods (tel15).
#[allow(dead_code)]
pub(crate) mod tactics_screen_state {
    /// u32: the screen shows the reserve team's tactics (the `team_index` of the Tactics calls) [inference].
    pub(crate) const RESERVE: usize = 0x8;
    /// i32: the match record index (-1: none).
    pub(crate) const RECORD_INDEX: usize = 0x10;
    /// The match record copy (a match_record of 0xd7a bytes): its player rows at `match_record::ROW_STAFF_ID` + side x
    /// `match_record::SIDE_SIZE` + row x `match_record_row::SIZE`.
    pub(crate) const RECORD: usize = 0x18;
    /// The fixture copy (0x4f bytes): the match's, the record's or the club's next fixture (`tactics_screen::COMP` is
    /// its competition, `STADIUM` its stadium).
    pub(crate) const FIXTURE_COPY: usize = 0xd92;
    /// Club*: the opponent (null outside a match and a match record).
    pub(crate) const OPPONENT: usize = 0xde5;
    /// i8: the opponent's side in the match.
    pub(crate) const OPPONENT_SIDE: usize = 0xdea;
    /// Formation*: the club's active tactic.
    pub(crate) const ACTIVE_TACTIC: usize = 0xdec;
    /// Staff*[20]: the opponent's likely line-up (`predict_opponent_lineup`).
    pub(crate) const OPPONENT_LINEUP: usize = 0xdf0;
    /// u8: the previous view (the human's `get_tactic_prev_view_mode`); 1 makes `get_slot_coordinate` use the view's
    /// own phase.
    pub(crate) const HOVER_VIEW: usize = 0x2728;
    /// The size of a team data block (the working one at `tactics_screen::FORMATION`, the undo copy and the clipboard).
    pub(crate) const TEAM_DATA_SIZE: usize = 0x3c1f;
    /// The size of the Formation at the start of a team data block.
    pub(crate) const FORMATION_SIZE: usize = 0x18e3;
    /// i32[2]: the first and second choice playmakers (staff ids, 0 none).
    pub(crate) const PLAYMAKERS: usize = 0x4a24;
    /// i32[5][2]: the set piece takers, first and second choice of each kind (staff ids, 0 none).
    pub(crate) const SET_PIECE_TAKERS: usize = 0x4a2c;
    /// u32: the team data changed since the screen opened (the last field of the working team data) [inference].
    pub(crate) const TEAM_DATA_CHANGED: usize = 0x4a5f;
    /// The coordinates copied from one zone (Copy) and from a whole phase.
    pub(crate) const COPIED_ZONE: usize = 0xc2a1;
    pub(crate) const COPIED_PHASE: usize = 0xc2cd;
}

/// A thiscall of the original exe with up to four dword arguments (the Formation, Tactics, human manager and
/// Tactics_Screen methods the screen calls by address); returns eax.
#[allow(dead_code)]
mod tactics_screen_calls {
    pub(crate) unsafe fn call0(f: usize, this: usize) -> u32 {
        let g: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(f);
        g(this)
    }
    pub(crate) unsafe fn call1(f: usize, this: usize, a: u32) -> u32 {
        let g: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(f);
        g(this, a)
    }
    pub(crate) unsafe fn call2(f: usize, this: usize, a: u32, b: u32) -> u32 {
        let g: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(f);
        g(this, a, b)
    }
    pub(crate) unsafe fn call3(f: usize, this: usize, a: u32, b: u32, c: u32) -> u32 {
        let g: unsafe extern "thiscall" fn(usize, u32, u32, u32) -> u32 = core::mem::transmute(f);
        g(this, a, b, c)
    }
    pub(crate) unsafe fn call4(f: usize, this: usize, a: u32, b: u32, c: u32, d: u32) -> u32 {
        let g: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(f);
        g(this, a, b, c, d)
    }
}

/// The routines of the original the Tactics_Screen methods call by address.
#[allow(dead_code)]
mod tactics_screen_fns {
    pub(crate) const FORMATION_GET_SLOT_COORDINATE: usize = 0x5b3c70;
    pub(crate) const FORMATION_SET_SLOT_COORDINATE: usize = 0x5b3560;
    pub(crate) const FORMATION_SET_ZONE_COORDINATES: usize = 0x5b3de0;
    pub(crate) const FORMATION_SET_PHASE_COORDINATES: usize = 0x5b4120;
    pub(crate) const FORMATION_SWAP_SLOT_COORDINATES: usize = 0x5b45b0;
    pub(crate) const FORMATION_IS_SLOT_CUSTOMISED: usize = 0x5b4a60;
    pub(crate) const FORMATION_IS_SLOT_COORDINATE_CUSTOMISED: usize = 0x5b4b70;
    pub(crate) const FORMATION_SET_PLAYER_INSTRUCTION: usize = 0x5b5290;
    pub(crate) const FORMATION_SET_MARKING_TYPE: usize = 0x5b57d0;
    pub(crate) const FORMATION_SET_MARKING_TARGET: usize = 0x5b5910;
    pub(crate) const FORMATION_LOAD_FROM_DISK: usize = 0x5b5d10;
    pub(crate) const FORMATION_SAVE_TO_DISK: usize = 0x5b6900;
    pub(crate) const FORMATION_GET_SLOT_POSITION: usize = 0x5b2ae0;
    pub(crate) const HUMAN_MANAGER_ADD_RECENT_FILE: usize = 0x5fcb60;
    pub(crate) const HUMAN_MANAGER_GET_PLAYER_SQUAD_NUMBER: usize = 0x601880;
    pub(crate) const HUMAN_MANAGER_GET_PASSWORD: usize = 0x5fde40;
    pub(crate) const TACTICS_SET_SHOOTOUT_TAKER_SLOT: usize = 0x898150;
    /// The Tactics_Screen getters the methods call on each other.
    pub(crate) const GET_SLOT_NUMBER_TEXT: usize = 0x8aa200;
    pub(crate) const GET_SLOT_PLAYER_NAME_TEXT: usize = 0x8aa550;
    pub(crate) const GET_SLOT_PITCH_POSITION: usize = 0x8aa9d0;
    pub(crate) const GET_SLOT_PLAYER: usize = 0x8ab480;
    pub(crate) const FIND_SLOT_AT_POSITION: usize = 0x8ab670;
    pub(crate) const GET_SLOT_MARKING_TARGET_ROW: usize = 0x8aba70;
}

/// Saves the undo copy: the working team data copied over the undo block, and Undo offered.
unsafe fn tactics_save_undo(ts: usize) {
    core::ptr::copy((ts + tactics_screen::FORMATION) as *const u8, (ts + tactics_screen::UNDO_DATA) as *mut u8, tactics_screen_state::TEAM_DATA_SIZE);
    write_u32(ts + tactics_screen::CAN_UNDO, 1);
}

/// Puts the undo copy back over the working team data (an edit the Formation refused) and withdraws Undo.
unsafe fn tactics_restore_undo(ts: usize) {
    core::ptr::copy((ts + tactics_screen::UNDO_DATA) as *const u8, (ts + tactics_screen::FORMATION) as *mut u8, tactics_screen_state::TEAM_DATA_SIZE);
    write_u32(ts + tactics_screen::CAN_UNDO, 0);
}

/// The phase flag the Formation methods take: 1 in the Without Ball view (view 2 is With Ball: the flag is
/// `view == 2`).
unsafe fn tactics_view_phase(ts: usize) -> u32 {
    (read_u8(ts + tactics_screen::VIEW) == 2) as u32
}

/// The match player of a row: the screen's own (`MATCH_PLAYERS`, 0x1be bytes each) or, for the opponent, the live
/// match's (`engine::PLAYERS`, 20 per side).
unsafe fn tactics_row_match_player(ts: usize, row: i8, opponent: bool) -> usize {
    if opponent {
        let k = read_i8(ts + tactics_screen_state::OPPONENT_SIDE) as isize * 20 + row as isize;
        (read_usize(ts + tactics_screen::MATCH) as isize + engine::PLAYERS as isize + k * 0x1be) as usize
    } else {
        (ts as isize + tactics_screen::MATCH_PLAYERS as isize + row as isize * 0x1be) as usize
    }
}

/// The row of the match record copy: the club's side (`SIDE`) or the opponent's (`OPPONENT_SIDE`).
unsafe fn tactics_record_row(ts: usize, row: i8, opponent: bool) -> usize {
    let side = read_i8(ts + if opponent { tactics_screen_state::OPPONENT_SIDE } else { tactics_screen::SIDE }) as isize;
    (ts as isize + (tactics_screen_state::RECORD + match_record::ROW_STAFF_ID) as isize + side * match_record::SIDE_SIZE as isize
        + row as isize * match_record_row::SIZE as isize) as usize
}

/// The club of a row: the opponent or the screen's club.
unsafe fn tactics_row_club(ts: usize, opponent: bool) -> usize {
    read_usize(ts + if opponent { tactics_screen_state::OPPONENT } else { tactics_screen::CLUB })
}

/// Runs a Tactics_Screen method's original and copy side by side over `regions` (the screen object and what the
/// method writes outside it) and records whether the memory and the results match.
unsafe fn compare_tactics_method<R: PartialEq + core::fmt::Debug>(name: &str, regions: &[(usize, usize)], original: impl FnOnce() -> R, rust: impl FnOnce() -> R) -> R {
    let (a, b, same, mo, mr) = crate::comparison::side_by_side(regions, original, rust);
    SCOREBOARD.record(same && a == b, || format!("tactics screen {}: {:?}/{:?} differences at {:?}", name, a, b, first_differences(&mo, &mr)));
    a
}

/// The Tactics_Screen object as a comparison region.
fn tactics_screen_region(ts: usize) -> (usize, usize) {
    (ts, tactics_screen::SIZE as usize)
}

/// The element constructors and destructors of the Tactics_Screen's sub-objects: the match team blocks (0x625
/// bytes), the Formation and the match players (0x1be).
const MATCH_TEAM_CONSTRUCT_FN: usize = 0x453c30;
const FORMATION_CONSTRUCT_FN: usize = 0x5b27c0;
const MATCH_PLAYER_CONSTRUCT_FN: usize = 0x6eada0;

/// Builds the sub-objects of a new Tactics_Screen (the C++ member constructors): the two match team blocks of the
/// record copy, and for each team data block (working, undo, clipboard) its Formation and its 20 match players.
unsafe fn tactics_screen_construct_members(ts: usize) {
    exe::crt_vector_constructor_iterator(ts + tactics_screen::MATCH_TEAMS, 0x625, 2, MATCH_TEAM_CONSTRUCT_FN, 0x453c10usize);
    for block in [tactics_screen::FORMATION, tactics_screen::UNDO_DATA, tactics_screen::CLIPBOARD_DATA] {
        tactics_screen_calls::call0(FORMATION_CONSTRUCT_FN, ts + block);
        exe::crt_vector_constructor_iterator(ts + block + tactics_screen::TEAM_PLAYERS, 0x1be, 0x14, MATCH_PLAYER_CONSTRUCT_FN, 0x6ebd60usize);
    }
}

/// Copies one Tactics data block of the club into the screen object (captain, set piece takers, penalty order,
/// playmakers) from the Tactics table, with the reserve flag; `default_penalties` fills the club's penalty order
/// first (`set_default_penalty_takers`).
unsafe fn tactics_screen_read_club_tactics(ts: usize, default_penalties: bool) {
    let (tactics, club, reserve) = (read_usize(G_TACTICS), read_u32(ts + tactics_screen::CLUB), read_u32(ts + tactics_screen_state::RESERVE));
    write_u32(ts + tactics_screen::CAPTAIN, tactics_screen_calls::call2(0x8976a0, tactics, club, reserve));
    for kind in 0..5u32 {
        for choice in 0..2u32 {
            let taker = tactics_screen_calls::call4(0x897990, tactics, club, kind, choice, reserve);
            write_u32(ts + tactics_screen_state::SET_PIECE_TAKERS + (kind * 8 + choice * 4) as usize, taker);
        }
    }
    if default_penalties {
        tactics_screen_calls::call0(0x8a9370, ts);
    }
    for k in 0..0xbu32 {
        write_u8(ts + tactics_screen::PENALTY_ORDER + k as usize, tactics_screen_calls::call3(0x898040, tactics, club, k, reserve) as u8);
    }
    write_u32(ts + tactics_screen_state::PLAYMAKERS, tactics_screen_calls::call3(0x897dc0, tactics, club, 0, reserve));
    write_u32(ts + tactics_screen_state::PLAYMAKERS + 4, tactics_screen_calls::call3(0x897dc0, tactics, club, 1, reserve));
}

/// The view settings of a screen that does not take the human's: the overview, zone 0, no runs, no substitutions.
unsafe fn tactics_screen_default_view(ts: usize) {
    write_u8(ts + tactics_screen::VIEW, 1);
    write_u8(ts + tactics_screen_state::HOVER_VIEW, 1);
    write_u32(ts + tactics_screen::ZONE, 0);
    write_u32(ts + tactics_screen::HOVER_ZONE, 0);
    write_u32(ts + tactics_screen::SHOW_RUNS, 0);
    write_u8(ts + tactics_screen::SUBS_MADE, 0);
}

/// Builds a Tactics_Screen for a club: from a match record, from the live match, or from the club's next fixture.
///
/// First the sub-objects (`tactics_screen_construct_members`) and a local fixture list. A null club shows the error
/// box of line 0x12b6 and leaves the object half built. The human's tactic slot is selected for the reserve flag
/// and the arguments stored (+0 club, +4 match, +8 reserve, +0xc `arg4`, +0x10 record index, +0x14 penalty mode).
/// - A match record (`record` > -1): read into `RECORD` (box of line 0x12d6 when it fails) with its fixture
///   (box 0x12de); the opponent is the record's other club, the side the club's, the kit the record's for that
///   side, no active tactic.
/// - A live match: its fixture copied, the opponent the fixture's other club, the kit the engine's for the club's
///   side, the active tactic the engine's tactics of that side, and the side's 20 match players copied in.
/// - Otherwise the first unplayed fixture of the club in the next 8 weeks (24 for a national team,
///   `FIXTURE_MANAGER::find_fixtures`; the reserve flag 0 asks for the first team), whose other club is the opponent
///   (none without a fixture); the kit 1; the active tactic the human's (`HUMAN_MANAGER::get_active_tactic` of the
///   club's manager) for a human club, else the club's Tactics; and the opponent's line-up predicted. The sides
///   are left as they were.
///
/// Then the team data: with a match record, the formation the record holds for the club's side, the overview, the
/// record's captain, no takers, no penalty order, no playmakers, read-only. A club the current human controls takes
/// the active tactic, his tactics view settings, and the club's captain, takers, default penalty order and
/// playmakers from the Tactics table (editable); any other club the same without the view settings and the default
/// penalty order, read-only. The team data starts unchanged, nothing to paste or undo, and is copied into the
/// clipboard block. Returns the object.
/// Original: `Tactics_Screen::Tactics_Screen` (0x8a7440, tactics_screens.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn tactics_screen_constructor(ts: usize, club: usize, m: usize, reserve: u32, arg4: u32, record: i32, penalty: u32) -> usize {
    use tactics_screen as t;
    use tactics_screen_state as s;
    tactics_screen_construct_members(ts);
    let mut fixtures = [0u8; 14];
    let list = fixtures.as_mut_ptr() as usize;
    exe::new_llist(list);
    if club == 0 {
        tel26_error_box(TACTICS_SCREENS_CPP, 0x12b6);
        exe::drop_llist(list);
        return ts;
    }
    exe::human_manager_select_tactic_slot(read_usize(G_HUMAN_MANAGER), reserve as i32, club, 0usize);
    write_usize(ts + t::CLUB, club);
    write_usize(ts + t::MATCH, m);
    write_u32(ts + s::RESERVE, reserve);
    write_u32(ts + t::FIXTURE, arg4);
    write_i32(ts + s::RECORD_INDEX, record);
    write_u32(ts + t::PENALTY_MODE, penalty);
    let fixture = ts + s::FIXTURE_COPY;
    if record > -1 {
        covered!("tel15_1 tactics screen of a match record");
        let read: unsafe extern "thiscall" fn(usize, i32, usize) -> u32 = core::mem::transmute(0x729600usize);
        if read(read_usize(G_MATCH_STATS), record, ts + s::RECORD) == 0 {
            tel26_error_box(TACTICS_SCREENS_CPP, 0x12d6);
            exe::drop_llist(list);
            return ts;
        }
        let find: unsafe extern "thiscall" fn(usize, i32, usize) -> u32 = core::mem::transmute(0x729e30usize);
        if find(read_usize(G_MATCH_STATS), read_i32(ts + s::RECORD_INDEX), fixture) == 0 {
            tel26_error_box(TACTICS_SCREENS_CPP, 0x12de);
            exe::drop_llist(list);
            return ts;
        }
        let home = read_u32(ts + s::RECORD + match_record::HOME_CLUB_ID);
        write_u32(ts + t::HAS_MATCH_RECORD, 1);
        let at_home = read_u32(club) == home;
        let other = if at_home { read_u32(ts + s::RECORD + match_record::AWAY_CLUB_ID) } else { home };
        write_usize(ts + s::OPPONENT, read_usize(G_CLUB_TABLE) + other as usize * club::SIZE);
        write_i8(ts + t::SIDE, if at_home { 0 } else { 1 });
        write_i8(ts + s::OPPONENT_SIDE, if at_home { 1 } else { 0 });
        let side = read_i8(ts + t::SIDE) as usize;
        write_u8(ts + t::KIT, read_u8(ts + s::RECORD + match_record::HOME_KIT + side * match_record::SIDE_SIZE));
        write_usize(ts + s::ACTIVE_TACTIC, 0);
    } else if m != 0 {
        covered!("tel15_1 tactics screen of a live match");
        let fx = read_usize(m + engine::MATCH);
        core::ptr::copy(fx as *const u8, fixture as *mut u8, 0x4f);
        write_u32(ts + t::HAS_MATCH_RECORD, 1);
        let home = if fx != 0 { read_usize(fx + fixture::HOME_CLUB) } else { 0 };
        let at_home = club == home;
        let other = if fx == 0 { 0 } else if at_home { read_usize(fx + fixture::AWAY_CLUB) } else { read_usize(fx + fixture::HOME_CLUB) };
        write_usize(ts + s::OPPONENT, other);
        write_i8(ts + t::SIDE, if at_home { 0 } else { 1 });
        write_i8(ts + s::OPPONENT_SIDE, if at_home { 1 } else { 0 });
        let side = read_i8(ts + t::SIDE);
        write_u8(ts + t::KIT, read_u8(m + engine::SIDE_KIT + if side == 0 { 0 } else { 1 }));
        write_usize(ts + s::ACTIVE_TACTIC, (m as isize + engine::TACTICS as isize + side as isize * s::FORMATION_SIZE as isize) as usize);
        for k in 0..20 {
            core::ptr::copy((m as isize + engine::PLAYERS as isize + (side as isize * 20 + k) * 0x1be) as *const u8,
                            (ts + t::MATCH_PLAYERS + k as usize * 0x1be) as *mut u8, 0x1be);
        }
    } else {
        covered!("tel15_1 tactics screen outside a match");
        write_u32(ts + t::HAS_MATCH_RECORD, 0);
        let weeks = if exe::database_is_national_team(club) != 0 { 0x18 } else { 8 };
        let (mut a, mut b) = ([0u8; 8], [0u8; 8]);
        let end_year = read_u16(exe::date_add_weeks(G_CURRENT_DATE, &mut a, weeks) + 2);
        let end = exe::date_add_weeks(G_CURRENT_DATE, &mut b, weeks);
        let end_day = read_u16(end);
        // the original pushes whole registers: their top halves are those of the club pointer and of the date's address
        let (high_club, high_stack) = (club as u32 & 0xffff_0000, end as u32 & 0xffff_0000);
        exe::fixture_manager_find_fixtures(read_usize(G_FIXTURE_MANAGER), high_club | read_u16(G_CURRENT_DATE) as u32, high_stack | read_u16(G_CURRENT_DATE + 2) as u32,
                                           high_stack | end_day as u32, high_club | end_year as u32, read_u32(club), list, u32::MAX, 0xffff_fffe, (reserve == 0) as u32);
        if read_u16(list + llist::COUNT) != 0 {
            write_u32(list + llist::CURSOR, read_u32(list + llist::HEAD));
            let mut f = exe::llist_get_next(list);
            while f != 0 {
                if read_u8(f + fixture::HOME_GOALS) == 0xff {
                    core::ptr::copy(f as *const u8, fixture as *mut u8, 0x4f);
                    write_u32(ts + t::HAS_MATCH_RECORD, 1);
                    break;
                }
                f = exe::llist_get_next(list);
            }
        }
        if read_u32(ts + t::HAS_MATCH_RECORD) != 0 {
            let home = read_usize(fixture + fixture::HOME_CLUB);
            write_usize(ts + s::OPPONENT, if club == home { read_usize(fixture + fixture::AWAY_CLUB) } else { home });
        } else {
            write_usize(ts + s::OPPONENT, 0);
        }
        write_u8(ts + t::KIT, 1);
        let active = if exe::human_manager_is_human_club(club, 0, 1, 0, 1) != 0 {
            exe::human_manager_get_active_tactic(read_usize(G_HUMAN_MANAGER), exe::database_get_club_manager(club))
        } else {
            tactics_screen_calls::call2(0x896660, read_usize(G_TACTICS), club as u32, 0) as usize
        };
        write_usize(ts + s::ACTIVE_TACTIC, active);
        tactics_screen_calls::call0(0x8abe30, ts);
    }
    let fm = ts + t::FORMATION;
    let read_only;
    if read_i32(ts + s::RECORD_INDEX) > -1 {
        let mut kept = vec![0u8; 0x18e4];
        let built = tactics_screen_calls::call2(0x5b2930, kept.as_mut_ptr() as usize, (ts + s::RECORD) as u32, read_u8(ts + t::SIDE) as u32) as usize;
        core::ptr::copy(built as *const u8, fm as *mut u8, s::FORMATION_SIZE);
        tactics_screen_default_view(ts);
        let side = read_i8(ts + t::SIDE) as isize;
        let team = ts as isize + s::RECORD as isize + side * match_record::SIDE_SIZE as isize;
        let captain = read_i8((team + match_record::HOME_CAPTAIN as isize) as usize) as isize;
        write_u32(ts + t::CAPTAIN, read_u32((team + match_record::ROW_STAFF_ID as isize + captain * match_record_row::SIZE as isize) as usize));
        core::ptr::write_bytes((ts + s::SET_PIECE_TAKERS) as *mut u8, 0, 0x28);
        core::ptr::write_bytes((ts + t::PENALTY_ORDER) as *mut u8, 0xff, 0xb);
        write_u32(ts + s::PLAYMAKERS + 4, 0);
        write_u32(ts + s::PLAYMAKERS, 0);
        read_only = 1;
    } else if exe::human_manager_current_controls_club(club, 0, 1) != 0 {
        covered!("tel15_1 tactics screen of the human's club");
        let active = read_usize(ts + s::ACTIVE_TACTIC);
        if active != 0 {
            core::ptr::copy(active as *const u8, fm as *mut u8, s::FORMATION_SIZE);
        }
        let hm = read_usize(G_HUMAN_MANAGER);
        write_u8(ts + t::VIEW, read_u8(tactics_screen_calls::call1(0x5fc250, hm, 0) as usize));
        write_u8(ts + s::HOVER_VIEW, read_u8(tactics_screen_calls::call1(0x5fc370, hm, 0) as usize));
        write_u32(ts + t::ZONE, read_u32(tactics_screen_calls::call1(0x5fc010, hm, 0) as usize));
        write_u32(ts + t::HOVER_ZONE, read_u32(tactics_screen_calls::call1(0x5fc130, hm, 0) as usize));
        write_u32(ts + t::SHOW_RUNS, read_u32(tactics_screen_calls::call1(0x5fc490, hm, 0) as usize));
        write_u8(ts + t::SUBS_MADE, 0);
        tactics_screen_read_club_tactics(ts, true);
        read_only = 0;
    } else {
        let active = read_usize(ts + s::ACTIVE_TACTIC);
        if active != 0 {
            core::ptr::copy(active as *const u8, fm as *mut u8, s::FORMATION_SIZE);
        }
        tactics_screen_default_view(ts);
        tactics_screen_read_club_tactics(ts, false);
        read_only = 1;
    }
    write_u32(ts + s::TEAM_DATA_CHANGED, 0);
    write_u8(ts + t::CAN_PASTE, 0);
    write_u32(ts + t::CAN_UNDO, 0);
    write_u32(ts + t::READ_ONLY, read_only);
    core::ptr::copy(fm as *const u8, (ts + t::CLIPBOARD_DATA) as *mut u8, s::TEAM_DATA_SIZE);
    exe::drop_llist(list);
    ts
}

/// Copy-constructs a Tactics_Screen from `src` (the screen's team instructions dialogs edit a copy).
///
/// The sub-objects are built first; a null source shows the error box of line 0x13b9. Otherwise the copy takes the
/// arguments (+0..+0x14), the match record (`match_stats_record_assign`), the fixture copy, the record flag, the
/// opponent, the sides, the kit, the active tactic, the opponent's line-up, the read-only flag, the three team data
/// blocks, the copied zone and phase, and the paste and undo flags. Returns the object.
/// Original: `Tactics_Screen::Tactics_Screen(const Tactics_Screen&)` (0x8a7e00, tactics_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn tactics_screen_copy_constructor(ts: usize, src: usize) -> usize {
    use tactics_screen as t;
    use tactics_screen_state as s;
    tactics_screen_construct_members(ts);
    if src == 0 {
        tel26_error_box(TACTICS_SCREENS_CPP, 0x13b9);
        return ts;
    }
    covered!("tel15_1 copy a tactics screen");
    let copy = |at: usize, n: usize| core::ptr::copy((src + at) as *const u8, (ts + at) as *mut u8, n);
    copy(0, 0x18);
    tactics_screen_calls::call1(0x8a5010, ts + s::RECORD, (src + s::RECORD) as u32);
    copy(s::FIXTURE_COPY, 0x4f);
    copy(t::HAS_MATCH_RECORD, 4);
    copy(s::OPPONENT, 4);
    copy(t::SIDE, 1);
    copy(s::OPPONENT_SIDE, 1);
    copy(t::KIT, 1);
    copy(s::ACTIVE_TACTIC, 4);
    copy(s::OPPONENT_LINEUP, 0x50);
    copy(t::READ_ONLY, 4);
    copy(t::FORMATION, s::TEAM_DATA_SIZE);
    copy(t::UNDO_DATA, s::TEAM_DATA_SIZE);
    copy(t::CLIPBOARD_DATA, s::TEAM_DATA_SIZE);
    copy(s::COPIED_ZONE, 0x2c);
    copy(s::COPIED_PHASE, 0x210);
    copy(t::CAN_PASTE, 1);
    copy(t::CAN_UNDO, 4);
    ts
}

/// Destroys a Tactics_Screen's sub-objects: the match players of the clipboard, undo and working team data, then
/// the two match team blocks of the record copy.
/// Original: `Tactics_Screen::~Tactics_Screen` (0x8a2ca0, tactics_screens.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn tactics_screen_destructor(ts: usize) {
    covered!("tel15_1 destroy a tactics screen");
    for block in [tactics_screen::CLIPBOARD_DATA, tactics_screen::UNDO_DATA, tactics_screen::FORMATION] {
        exe::crt_vector_destructor_iterator(ts + block + tactics_screen::TEAM_PLAYERS, 0x1be, 0x14, MATCH_PLAYER_DESTRUCT_FN);
    }
    exe::crt_vector_destructor_iterator(ts + tactics_screen::MATCH_TEAMS, 0x625, 2, MATCH_TEAM_DESTRUCT_FN);
}

/// Leaves the tactics screen writing the edits back: the Tactics_Screen of parameter 0, when there is one, applies
/// its changes (`apply_changes`), is destroyed and freed, and parameter 0 is cleared.
/// Original: `tactics_screen_close_and_apply` (0x8a2d70, tactics_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn tactics_screen_close_and_apply() {
    let ts = get_screen_param(0);
    if ts == 0 {
        return;
    }
    covered!("tel15_1 close the tactics screen applying");
    tactics_screen_calls::call0(0x8a80d0, ts);
    display_method(TACTICS_DESTRUCT_FN, ts);
    exe::crt_operator_delete(ts);
    set_screen_param(0, 0);
}

/// Copies a match statistics record field by field (the compiler's assignment): the header (0x11f bytes), each
/// side's team block header (0x71 bytes) and its 20 player rows (`match_player_row_assign`), the event log
/// descriptor (13 bytes) and the weather. Returns `this`.
/// Original: `match_stats_record_assign` (0x8a5010, tactics_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn match_stats_record_assign(this: usize, src: usize) -> usize {
    let copy = |at: usize, n: usize| core::ptr::copy((src + at) as *const u8, (this + at) as *mut u8, n);
    copy(0, match_record::HOME_TEAM);
    for side in 0..2 {
        let team = match_record::HOME_TEAM + side * match_record::SIDE_SIZE;
        copy(team, 0x71);
        for row in 0..0x14 {
            let at = team + 0x71 + row * match_record_row::SIZE;
            tactics_screen_calls::call1(0x71df00, this + at, (src + at) as u32);
        }
    }
    copy(match_record::EVENT_LOG, 0xd);
    copy(match_record::WEATHER, 4);
    this
}

/// Writes the edits back to the game: the working formation (its position lookup rebuilt) over the active tactic,
/// the human's tactics view settings, and the captain, set piece takers, penalty order and playmakers into the
/// club's Tactics row (with the reserve flag). In a live match, the queued substitutions: the commentary event of
/// their count (0x1f5a one, 0x1f5c two, 0x1f59 more) and each one made (`apply_substitution`), then the engine
/// refreshes the side (goalkeeper cleared and found again, side score) and the marking targets. A read-only screen
/// does nothing and returns 0; otherwise 1.
/// Original: `Tactics_Screen::apply_changes` (0x8a80d0, tactics_screens.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn apply_changes(ts: usize) -> u32 {
    use tactics_screen as t;
    use tactics_screen_state as s;
    if read_u32(ts + t::READ_ONLY) != 0 {
        return 0;
    }
    covered!("tel15_1 apply the tactics");
    let fm = ts + t::FORMATION;
    tactics_screen_calls::call0(0x5b7160, fm);
    let active = read_usize(ts + s::ACTIVE_TACTIC);
    if active != 0 {
        core::ptr::copy(fm as *const u8, active as *mut u8, s::FORMATION_SIZE);
    }
    let hm = read_usize(G_HUMAN_MANAGER);
    write_u8(tactics_screen_calls::call1(0x5fc250, hm, 0) as usize, read_u8(ts + t::VIEW));
    write_u8(tactics_screen_calls::call1(0x5fc370, hm, 0) as usize, read_u8(ts + s::HOVER_VIEW));
    write_u32(tactics_screen_calls::call1(0x5fc010, hm, 0) as usize, read_u32(ts + t::ZONE));
    write_u32(tactics_screen_calls::call1(0x5fc130, hm, 0) as usize, read_u32(ts + t::HOVER_ZONE));
    write_u32(tactics_screen_calls::call1(0x5fc490, hm, 0) as usize, read_u32(ts + t::SHOW_RUNS));
    let (tactics, club, reserve) = (read_usize(G_TACTICS), read_u32(ts + t::CLUB), read_u32(ts + s::RESERVE));
    let captain = read_i32(ts + t::CAPTAIN);
    if captain != -1 {
        tactics_screen_calls::call3(0x897880, tactics, club, (read_usize(G_STAFF_TABLE) + captain as usize * staff::SIZE) as u32, reserve);
    }
    for kind in 0..5u32 {
        for choice in 0..2u32 {
            let taker = read_u32(ts + s::SET_PIECE_TAKERS + (kind * 8 + choice * 4) as usize);
            let set: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x897bb0usize);
            set(tactics, club, kind, choice, taker, reserve);
        }
    }
    for k in 0..0xbu32 {
        tactics_screen_calls::call4(tactics_screen_fns::TACTICS_SET_SHOOTOUT_TAKER_SLOT, tactics, club, k, read_u8(ts + t::PENALTY_ORDER + k as usize) as u32, reserve);
    }
    for choice in 0..2u32 {
        tactics_screen_calls::call4(0x897ee0, tactics, club, choice, read_u32(ts + s::PLAYMAKERS + choice as usize * 4), reserve);
    }
    let m = read_usize(ts + t::MATCH);
    if m == 0 {
        return 1;
    }
    let side = read_u8(ts + t::SIDE) as u32;
    let made = read_i8(ts + t::SUBS_MADE);
    if made != 0 {
        covered!("tel15_1 apply the queued substitutions");
        let event: u32 = if made > 2 { 0x1f59 } else if made == 2 { 0x1f5c } else { 0x1f5a };
        let add: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32, u32, u32, u32) = core::mem::transmute(0x6d6f70usize);
        add(read_usize(m + engine::COMMENTATOR), event, m, u32::MAX, side, u32::MAX, u32::MAX, 0);
        let mut k = 0i8;
        while k < read_i8(ts + t::SUBS_MADE) {
            tactics_screen_calls::call2(0x8ac3d0, ts, read_u8(ts + 0x4a0d + k as usize) as u32, read_u8(ts + 0x4a16 + k as usize) as u32);
            k += 1;
        }
    }
    tactics_screen_calls::call1(0x6ce0f0, m, side);
    tactics_screen_calls::call1(0x6ce110, m, side);
    tactics_screen_calls::call1(0x6bbb60, m, side);
    tactics_screen_calls::call0(0x6bbf60, m);
    1
}

/// Writes the screen's title into `out` (at least 100 bytes) and the club's kit colours for its kit into the two
/// colours: "<club> Penalty Takers" in the penalty mode, "<club> Reserve Tactics" for the reserves of a club that is
/// not a national team, else "<club> Tactics" (through the language table, with the club's gender byte).
///
/// A null `out`, a size under 100 or a null colour shows the error box of line 0x1452 and returns 0; else 1.
/// Original: `Tactics_Screen::format_title` (0x8a8330, tactics_screens.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn format_title(ts: usize, out: u32, size: u32, colour_a: u32, colour_b: u32) -> u32 {
    if out == 0 || (size as i32) < 0x64 || colour_a == 0 || colour_b == 0 {
        tel26_error_box(TACTICS_SCREENS_CPP, 0x1452);
        return 0;
    }
    let club = read_usize(ts + tactics_screen::CLUB);
    let colours: unsafe extern "C" fn(usize, u32, u32, u32) = core::mem::transmute(0x53b640usize);
    colours(club, read_u8(ts + tactics_screen::KIT) as u32, colour_a, colour_b);
    let format = if read_u32(ts + tactics_screen::PENALTY_MODE) != 0 {
        0xa8c558usize
    } else if read_u32(ts + tactics_screen_state::RESERVE) != 0 && exe::database_is_national_team(club) == 0 {
        0xa8c530
    } else {
        0xa8c510
    };
    let gender = read_i8(club + 0x37) as i32;
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    text(out as usize, format, gender, gender, club + 4);
    1
}

/// Makes the match player with the best captain score (`match_man_get_captain_score`) captain.
///
/// Nothing (0) on a read-only screen or outside a live match. Among the 20 match players on the pitch (position 0
/// and up) the first with the highest score; none returns 0, one without a staff shows the error box of line
/// 0x14e7 and returns 0. When he is already the captain's row it returns 1; otherwise the undo copy is saved, the
/// team data marked changed and his staff id made captain; returns 1.
/// Original: `Tactics_Screen::auto_select_captain` (0x8a84a0, tactics_screens.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn auto_select_captain(ts: usize) -> u32 {
    if read_u32(ts + tactics_screen::READ_ONLY) != 0 || read_usize(ts + tactics_screen::MATCH) == 0 {
        return 0;
    }
    let score: unsafe extern "C" fn(usize, usize, usize) -> i16 = core::mem::transmute(0x6e1cf0usize);
    let (mut best, mut top) = (-1i8, 0i16);
    for row in 0..0x14i8 {
        let mp = tactics_row_match_player(ts, row, false);
        if read_i8(mp + match_player::POSITION) < 0 {
            continue;
        }
        let v = score(0, mp, read_usize(ts + tactics_screen::CLUB));
        if best == -1 || v > top {
            best = row;
            top = v;
        }
    }
    if best <= -1 {
        return 0;
    }
    let staff = read_usize(tactics_row_match_player(ts, best, false) + match_player::STAFF);
    if staff == 0 {
        tel26_error_box(TACTICS_SCREENS_CPP, 0x14e7);
        return 0;
    }
    let captain_row = (0..0x14u32).find(|&k| {
        let p = tactics_screen_calls::call2(tactics_screen_fns::GET_SLOT_PLAYER, ts, k, 0) as usize;
        p != 0 && read_u32(p) == read_u32(ts + tactics_screen::CAPTAIN)
    }).map_or(-1, |k| k as i8);
    if captain_row == best {
        return 1;
    }
    covered!("tel15_1 a captain chosen");
    tactics_save_undo(ts);
    write_u32(ts + tactics_screen_state::TEAM_DATA_CHANGED, 1);
    write_u32(ts + tactics_screen::CAPTAIN, read_u32(staff));
    1
}

/// The default formation positions `predict_opponent_lineup` starts from (the 11 slots and the words after them, as
/// the original lays them on its stack).
const TACTICS_DEFAULT_LINEUP: [u16; 28] = [1, 0x84, 0x104, 0x404, 0x804, 0x90, 0x110, 0x410, 0x810, 0x140, 0x440,
                                            1, 4, 0x10, 0x40, 0x40, 4, 0x10, 0x40, 0x40, 4, 0x10, 0x40, 0x40, 4, 0x10, 0x40, 0x40];

/// Predicts the opponent's starting line-up into `OPPONENT_LINEUP` (20 staff, cleared first).
///
/// Nothing (0) without an opponent or a fixture. The candidates are the opponent's squad players (50 places) with
/// player data who are not injured (`Injury::get_player_injury_status` 2), not banned for the fixture's competition
/// (`Discipline::is_player_banned`) and not away on international duty on the fixture date
/// (`NationalTeams::get_international_duty_club`); none returns 0. The positions are the opponent's club tactic's
/// (`Tactics::get_club_tactic`), else a default 4-4-2; `match_man_assign_players_to_positions` picks the eleven and
/// the bench (the fixture's bench size, flags 0x2014 for a national team, else 0x2042). Returns 1.
/// Original: `Tactics_Screen::predict_opponent_lineup` (0x8abe30, tactics_screens.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn predict_opponent_lineup(ts: usize) -> u32 {
    use tactics_screen_state as s;
    let mut positions = TACTICS_DEFAULT_LINEUP;
    core::ptr::write_bytes((ts + s::OPPONENT_LINEUP) as *mut u8, 0, 0x50);
    let opponent = read_usize(ts + s::OPPONENT);
    if opponent == 0 || read_u32(ts + tactics_screen::HAS_MATCH_RECORD) == 0 {
        return 0;
    }
    let fixture = ts + s::FIXTURE_COPY;
    let mut candidates = [0usize; 50];
    let mut count = 0u8;
    for k in 0..0x32usize {
        let p = read_usize(opponent + club::SQUAD + k * 4);
        if p == 0 || read_usize(p + staff::PLAYER) == 0 {
            continue;
        }
        if exe::injury_get_player_injury_status(read_usize(G_INJURY), p) == 2 {
            continue;
        }
        if tactics_screen_calls::call4(0x55d270, read_usize(G_DISCIPLINE), p as u32, read_u32(ts + tactics_screen::COMP), 1, 0) != 0 {
            continue;
        }
        let mut date = [0u8; 8];
        exe::new_date(date.as_mut_ptr() as usize, read_u16(fixture + fixture::DAY) as u32, read_u16(fixture + fixture::YEAR) as u32);
        let duty: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(0x7757d0usize);
        if duty(read_usize(G_NATIONAL_TEAMS), p, date.as_mut_ptr() as usize, opponent, 0, 0) == 0 {
            candidates[count as usize] = p;
            count += 1;
        }
    }
    if count == 0 {
        return 0;
    }
    covered!("tel15_1 predict the opponent's line-up");
    let tactic = tactics_screen_calls::call2(0x896660, read_usize(G_TACTICS), opponent as u32, 0) as usize;
    if tactic != 0 {
        for k in 0..0xbu32 {
            positions[k as usize] = tactics_screen_calls::call2(tactics_screen_fns::FORMATION_GET_SLOT_POSITION, tactic, k, 0) as u16;
        }
    }
    let flags: u32 = if exe::database_is_national_team(opponent) != 0 { 0x2014 } else { 0x2042 };
    let assign: unsafe extern "C" fn(usize, u32, usize, u32, u32, usize, u32, usize, u32, u32, u32, u32, u32, u32, u32) = core::mem::transmute(0x6e2000usize);
    assign(candidates.as_ptr() as usize, count as u32, positions.as_mut_ptr() as usize, 0xb, (read_u8(ts + tactics_screen::BENCH_SIZE) & 0xf) as u32,
           ts + s::OPPONENT_LINEUP, flags, opponent, 0, 0, 0, 0, 0, 0, 0);
    1
}

// The comparers: what builds, copies, frees or writes outside the screen object runs only in the original, with its
// plan checked afterwards; the rest side by side.

// simplification: builds sub-objects and reads the game: only the original runs; the plan: the arguments stored and
// the clipboard a copy of the working team data.
/// Comparer for `tactics_screen_constructor`.
pub(crate) unsafe extern "thiscall" fn compare_tactics_screen_constructor(ts: usize, club: usize, m: usize, reserve: u32, arg4: u32, record: i32, penalty: u32) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, i32, u32) -> usize>(%%I0%%)(ts, club, m, reserve, arg4, record, penalty);
    if club != 0 && read_u32(ts + tactics_screen::READ_ONLY) <= 1 {
        let args = [read_u32(ts), read_u32(ts + 4), read_u32(ts + 8), read_u32(ts + 0xc), read_u32(ts + 0x10), read_u32(ts + 0x14)];
        let plan = [club as u32, m as u32, reserve, arg4, record as u32, penalty];
        let n = tactics_screen_state::TEAM_DATA_SIZE;
        let same = core::slice::from_raw_parts((ts + tactics_screen::FORMATION) as *const u8, n) == core::slice::from_raw_parts((ts + tactics_screen::CLIPBOARD_DATA) as *const u8, n);
        SCOREBOARD.record(args == plan && same, || format!("tactics screen constructor: arguments {:x?} plan {:x?}, clipboard copy {}", args, plan, same));
    }
    r
}

// simplification: builds sub-objects: only the original runs; the plan: the team data blocks copied.
/// Comparer for `tactics_screen_copy_constructor`.
pub(crate) unsafe extern "thiscall" fn compare_tactics_screen_copy_constructor(ts: usize, src: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I1%%)(ts, src);
    if src != 0 {
        let n = tactics_screen_state::TEAM_DATA_SIZE;
        let same = core::slice::from_raw_parts((ts + tactics_screen::FORMATION) as *const u8, n) == core::slice::from_raw_parts((src + tactics_screen::FORMATION) as *const u8, n);
        SCOREBOARD.record(same, || "tactics screen copy constructor: working team data not copied".to_string());
    }
    r
}

// simplification: destroys sub-objects: only the original runs.
/// Comparer for `tactics_screen_destructor`.
pub(crate) unsafe extern "thiscall" fn compare_tactics_screen_destructor(ts: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(ts)
}

// simplification: applies, destroys and frees: only the original runs; the plan: parameter 0 cleared.
/// Comparer for `tactics_screen_close_and_apply`.
pub(crate) unsafe extern "C" fn compare_tactics_screen_close_and_apply() {
    let had = get_screen_param(0) != 0;
    original_fn::<unsafe extern "C" fn()>(%%I3%%)();
    SCOREBOARD.record(!had || get_screen_param(0) == 0, || "tactics_screen_close_and_apply: parameter 0 not cleared".to_string());
}

/// Comparer for `match_stats_record_assign`: side by side over the destination record.
pub(crate) unsafe extern "thiscall" fn compare_match_stats_record_assign(this: usize, src: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I4%%);
    compare_tactics_method("match_stats_record_assign", &[(this, 0xd7a)], || o(this, src), || match_stats_record_assign(this, src))
}

// simplification: writes the Tactics table, the human's settings and the live match: only the original runs; the
// plan: 0 exactly on a read-only screen.
/// Comparer for `apply_changes`.
pub(crate) unsafe extern "thiscall" fn compare_apply_changes(ts: usize) -> u32 {
    let read_only = read_u32(ts + tactics_screen::READ_ONLY) != 0;
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I5%%)(ts);
    SCOREBOARD.record((r == 0) == read_only, || format!("tactics screen apply_changes: {} on a screen read-only {}", r, read_only));
    r
}

/// Comparer for `format_title`: over the title and the two colours.
pub(crate) unsafe extern "thiscall" fn compare_format_title(ts: usize, out: u32, size: u32, colour_a: u32, colour_b: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u32>(%%I6%%);
    if out == 0 || (size as i32) < 0x64 || colour_a == 0 || colour_b == 0 {
        return o(ts, out, size, colour_a, colour_b); // simplification: the box, original only
    }
    compare_tactics_method("format_title", &[(out as usize, size.min(0x100) as usize), (colour_a as usize, 2), (colour_b as usize, 2)],
                           || o(ts, out, size, colour_a, colour_b), || format_title(ts, out, size, colour_a, colour_b))
}

/// Comparer for `auto_select_captain`: side by side over the screen object.
pub(crate) unsafe extern "thiscall" fn compare_auto_select_captain(ts: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I7%%);
    compare_tactics_method("auto_select_captain", &[tactics_screen_region(ts)], || o(ts), || auto_select_captain(ts))
}

/// Comparer for `predict_opponent_lineup`: side by side over the screen object (and the random generator).
pub(crate) unsafe extern "thiscall" fn compare_predict_opponent_lineup(ts: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I8%%);
    let mut reg = vec![tactics_screen_region(ts)];
    reg.extend_from_slice(&crate::rng::STATE_REGIONS);
    compare_tactics_method("predict_opponent_lineup", &reg, || o(ts), || predict_opponent_lineup(ts))
}
