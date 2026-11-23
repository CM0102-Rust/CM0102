use super::*;

/// Opens the tactics screen of a club: the Tactics_Screen object, the screen and its parameters.
///
/// A null club shows the tactics_screens.cpp error box of line 0x17c, and `arg4` and `penalty` both set the box of
/// line 0x183; nothing opens. Otherwise a Tactics_Screen (0xc4e2 bytes) is built from the club, the match, the
/// reserve flag, `arg4`, the match record index and the penalty mode (a failed allocation leaves it null). The
/// screen opens with the reserve builder (thunk 0x89f700) when `reserve` is set, else `tactics_screen_build`, and
/// with `tactics_screen_handle_event`; a read-only screen (`READ_ONLY`) as a repeatable screen with
/// `tactics_screen_close_without_apply` as its leave function, otherwise as a single-instance screen with
/// `tactics_screen_close_and_apply` as its exit function. When the screen does not open, the object is destroyed and
/// freed. Otherwise the parameters: 0 the object, 1 the club, 2 the match, 3 the reserve flag, 4 `arg4`, 5 and 6
/// zero, 7 the record index, 8 the penalty mode, 9 -1 and 10 the first column view (6, the match rating, in a match;
/// else 2, the condition).
/// Original: `tactics_screen_open` (0x899a50, tactics_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn tactics_screen_open(club: usize, reserve: u32, m: usize, arg4: u32, record: u32, penalty: u32) {
    use tactics_screen as t;
    if club == 0 {
        tel26_error_box(TACTICS_SCREENS_CPP, 0x17c);
        return;
    }
    if arg4 != 0 && penalty != 0 {
        tel26_error_box(TACTICS_SCREENS_CPP, 0x183);
        return;
    }
    covered!("tel14_1 open the tactics screen");
    let p = exe::crt_operator_new(t::SIZE);
    let ts = if p != 0 { tactics_screen_construct(p, club, m, reserve, arg4, record, penalty) } else { 0 };
    let build = if reserve != 0 { TACTICS_BUILD_RESERVE_FN } else { TACTICS_BUILD_FN };
    let opened = if read_u32(ts + t::READ_ONLY) != 0 {
        exe::screenmanager_open_screen(G_SCREEN_MANAGER, build, TACTICS_HANDLE_EVENT_FN, 0, TACTICS_CLOSE_WITHOUT_APPLY_FN, 0)
    } else {
        exe::screenmanager_open_screen(G_SCREEN_MANAGER, build, TACTICS_HANDLE_EVENT_FN, 1, 0usize, TACTICS_CLOSE_AND_APPLY_FN as u32)
    };
    if opened == 0 {
        if ts != 0 {
            tactics_screen_free_inline(ts);
        }
        return;
    }
    let first_view: u32 = if read_u32(ts + t::MATCH) != 0 { 6 } else { 2 };
    set_screen_param(0, ts);
    set_screen_param(1, club);
    set_screen_param(2, m);
    set_screen_param(3, reserve as usize);
    set_screen_param(4, arg4 as usize);
    set_screen_param(5, 0);
    set_screen_param(6, 0);
    set_screen_param(7, record as usize);
    set_screen_param(8, penalty as usize);
    set_screen_param(9, usize::MAX);
    set_screen_param(10, first_view as usize);
}

/// Destroys a tactics team data block: the element destructor over its 20 match player records (0x1be bytes each,
/// at `TEAM_PLAYERS`).
/// Original: `Tactics_Team_Data::~Tactics_Team_Data` (0x899e50, tactics_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn tactics_team_data_destructor(data: usize) {
    covered!("tel14_1 destroy a team data block");
    exe::crt_vector_destructor_iterator(data + tactics_screen::TEAM_PLAYERS, 0x1be, 0x14, MATCH_PLAYER_DESTRUCT_FN);
}

/// Leaves the tactics screen without writing the edits back: the Tactics_Screen of parameter 0, when there is one,
/// is destroyed (`Tactics_Screen::~Tactics_Screen`) and freed, and parameter 0 cleared.
/// Original: `tactics_screen_close_without_apply` (0x8a2c60, tactics_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn tactics_screen_close_without_apply() {
    let ts = get_screen_param(0);
    if ts == 0 {
        return;
    }
    covered!("tel14_1 close the tactics screen without applying");
    display_method(TACTICS_DESTRUCT_FN, ts);
    exe::crt_operator_delete(ts);
    set_screen_param(0, 0);
}

/// Comparer for `tactics_screen_open`.
pub(crate) unsafe extern "C" fn compare_tactics_screen_open(club: usize, reserve: u32, m: usize, arg4: u32, record: u32, penalty: u32) {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, usize, u32, u32, u32)>(IDX_TACTICS_SCREEN_OPEN);
    let before = get_screen_param(0);
    o(club, reserve, m, arg4, record, penalty);
    let ts = get_screen_param(0);
    if club == 0 || (arg4 != 0 && penalty != 0) || ts == 0 || ts == before {
        return;
    }
    let view = if read_u32(ts + tactics_screen::MATCH) != 0 { 6 } else { 2 };
    let plan = [club, m, reserve as usize, arg4 as usize, 0, 0, record as usize, penalty as usize, usize::MAX, view];
    let got: Vec<usize> = (1..=10).map(|k| get_screen_param(k)).collect();
    SCOREBOARD.record(got[..] == plan[..], || format!("tactics_screen_open(club {:#x}): params 1..10 {:x?}, plan {:x?}", club, got, plan));
}

/// Comparer for `tactics_team_data_destructor`.
pub(crate) unsafe extern "thiscall" fn compare_tactics_team_data_destructor(data: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_TACTICS_TEAM_DATA_DESTRUCTOR)(data)
}

/// Comparer for `tactics_screen_close_without_apply`.
pub(crate) unsafe extern "C" fn compare_tactics_screen_close_without_apply() {
    let had = get_screen_param(0) != 0;
    original_fn::<unsafe extern "C" fn()>(IDX_TACTICS_SCREEN_CLOSE_WITHOUT_APPLY)();
    SCOREBOARD.record(!had || get_screen_param(0) == 0, || "tactics_screen_close_without_apply: parameter 0 not cleared".to_string());
}

/// Builds the tactics screen.
///
/// The first build sets the static colours (`tactics_build_statics`). The Tactics_Screen of parameter 0 is built
/// when missing (from parameters 1, 2, 3, 4, 7 and 8) and stored back; the view column is parameter 0xa. Background:
/// the stadium picture of the match record (else its competition's), or the club's. The title (`format_title`, its
/// two colours) and the menu bar (mode 4). A read-only screen has only the Team Instructions button; an editable one,
/// the Tactics, View, Last Match and Edit menus (`tactics_build_menus`). Then the formation name (with the unsaved
/// mark when not saved), LOCKED, the view tabs and the list's column widths (`tactics_build_header`); the list with
/// the heading of the instruction column (views 7 and up: one row lower) and its 20 rows (`tactics_build_row`); the
/// pitch (`draw_pitch` 0x8ac600, with the keeper's colours dark on a white kit); the overview (`tactics_build_overview`)
/// or the zone views (`tactics_build_zones`); and Cancel / Ok (editable) or the Back and Next buttons (read-only).
/// Original: `tactics_screen_build` (0x899ea0, tactics_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn tactics_screen_build() {
    use tactics_screen as t;
    covered!("tel14_2 build the tactics screen");
    let c = tactics_colour;
    tactics_build_statics_init();
    let mut widths: [u8; 9] = [0xe, 1, 5, 1, 0x3c, 0xf, 0x10, 7, 7];
    let mut list_widths: [u8; 2] = [0xe, 0x70];
    let mut button_widths: [u8; 2] = [3, 1];
    let mut ts = get_screen_param(0);
    let view = get_screen_param(0xa) as u8;
    if ts == 0 {
        let p = exe::crt_operator_new(t::SIZE);
        ts = 0;
        if p != 0 {
            let penalty = get_screen_param(8) as u32;
            let record = get_screen_param(7) as u32;
            let arg4 = get_screen_param(4) as u32;
            let reserve = get_screen_param(3) as u32;
            let m = get_screen_param(2);
            let club = get_screen_param(1);
            ts = tactics_screen_construct(p, club, m, reserve, arg4, record, penalty);
        }
        set_screen_param(0, ts);
    }
    if read_u32(ts + t::HAS_MATCH_RECORD) != 0 {
        let stadium = read_usize(ts + t::STADIUM);
        if stadium == 0 || exe::background_set_stadium_picture(stadium, 0, 1) == 0 {
            exe::background_set_comp_picture(read_usize(ts + t::COMP), 0, 0);
        }
    } else {
        exe::background_set_club_picture(read_usize(ts + t::CLUB), 0, 0);
    }
    let (mut title_text, mut title_back) = (0u32, 0u32);
    tactics_calls::format_title(ts, &mut title_text, &mut title_back);
    exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, title_back, title_text, JOB_TEXT);
    exe::menubar_build(4, 0);
    let mut heading = [0u8; 0xc8];
    if read_u32(ts + t::READ_ONLY) != 0 {
        tactics_text(0xa8bfa4);
        tel26_add_button([0x6e, 0x50, 0xeb, 0x64, 0x30, c(JOB_COLOUR_317E), 0xc, 1, c(JOB_COLOUR_3184), JOB_TEXT as u32, 1, 0]);
    } else {
        tactics_build_menus(ts, view, &mut heading);
    }
    tactics_build_header(ts, view, &mut widths);
    // the player list
    tactics_area([0x6e, 0x91, 0x1b8, 0x221, 1, 0, 1, 0, 2, 0, u32::MAX]);
    let (list, handles, mut line) = if heading[0] != 0 {
        let list = tactics_area([0x6e, 0x91, 0x1b8, 0x221, 9, widths.as_mut_ptr() as u32, 0x15, 0, 1, 0, u32::MAX]);
        let handles = tactics_area([0x6e, 0x91, 0x1b8, 0x221, 2, list_widths.as_mut_ptr() as u32, 0x15, 0, 1, 0, u32::MAX]);
        tel26_area_text([list, 5, 0, 0x10, c(JOB_COLOUR_BDEA), 0xc, 1, c(JOB_COLOUR_3184), heading.as_ptr() as u32]);
        (list, handles, 1u32)
    } else {
        let list = tactics_area([0x6e, 0x99, 0x1b8, 0x219, 9, widths.as_mut_ptr() as u32, 0x14, 0, 1, 0, u32::MAX]);
        let handles = tactics_area([0x6e, 0x99, 0x1b8, 0x219, 2, list_widths.as_mut_ptr() as u32, 0x14, 0, 1, 0, u32::MAX]);
        (list, handles, 0u32)
    };
    for k in 0..0x14i8 {
        tactics_build_row(ts, view, k, list, handles, line);
        line += 1;
    }
    // the pitch
    let mut r = [0i32; 4];
    let draw: unsafe extern "C" fn(i32, i32, i32, i32, *mut i32, *mut i32, *mut i32, *mut i32, u32, i32, u32) -> u32 = core::mem::transmute(FN_DRAW_PITCH);
    let (left, top, right, bottom) = (&mut r[0] as *mut i32, &mut r[1] as *mut i32, &mut r[2] as *mut i32, &mut r[3] as *mut i32);
    draw(0x1c2, 0x91, 0x30c, 0x221, left, top, right, bottom, 0, -1, 0x40800000);
    let club = read_usize(ts + t::CLUB);
    let white = |kit: u8, colour_at: usize| read_u8(ts + t::KIT) == kit && read_usize(club + colour_at) != 0
        && exe::database_get_colour_family(read_usize(club + colour_at)) == 5;
    let keeper_line = if white(1, 0x87) || white(2, 0x8f) || white(3, 0x97) { c(JOB_COLOUR_BDEC) } else { read_u32(tactics_build_statics::GREEN_90) };
    let pitch = TacticsPitch { r, keeper_back: c(JOB_COLOUR_BDEA), keeper_line, field_back: title_text, field_line: title_back };
    if read_u8(ts + t::VIEW) == 1 {
        tactics_build_overview(ts, &pitch);
    } else {
        tactics_build_zones(ts, &pitch);
    }
    if read_u32(ts + t::READ_ONLY) == 0 {
        let buttons = tactics_area([0x64, 0x22b, 0x316, 0x24e, 2, button_widths.as_mut_ptr() as u32, 1, 0, 1, 0, u32::MAX]);
        tactics_text(0x98a1a0);
        tel26_area_button([buttons, 0, 0, 0x30, c(JOB_COLOUR_317E), 0xc, 3, c(JOB_COLOUR_3184), JOB_TEXT as u32, 0x20, 0]);
        tactics_text(0x990f7c);
        tel26_area_button([buttons, 1, 0, 0x30, c(JOB_COLOUR_317E), 0xc, 3, c(JOB_COLOUR_3184), JOB_TEXT as u32, 0x21, 0]);
    } else {
        tel26_back_next();
    }
}

/// Comparer for `tactics_screen_build`.
///
/// Side by side over the built display list (`compare_match_screen_build`: the screen manager, the current
/// Display, the parameter block, the text buffer and the generator), with the Tactics_Screen object and the static
/// colours. Only the original when the object does not exist yet (it is allocated here).
pub(crate) unsafe extern "C" fn compare_tactics_screen_build() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_TACTICS_SCREEN_BUILD);
    let ts = get_screen_param(0);
    if ts == 0 {
        // simplification: the allocation of the object does not run twice; proven in Rust mode
        return o();
    }
    let extra = [(ts, tactics_screen::SIZE as usize), (tactics_build_statics::GUARD, 0xa), (tactics_build_statics::GUARD2, 1),
        (tactics_build_statics::GREEN_90, 2), (tactics_build_statics::ZONE, 4), (tactics_build_statics::HOVER_ZONE, 2),
        (tactics_build_statics::RED, 4)];
    compare_match_screen_build("tactics_screen_build", &extra, || o(), || tactics_screen_build());
}

/// The events of the tactics screen.
///
/// The Tactics_Screen of parameter 0 is built when missing (from parameters 1, 2, 3, 4, 7 and 8) and the club's
/// kit colours taken for the boxes. With a dialog answer to dispatch (`[0xb77c84]`), the answers first
/// (`tactics_event_dialog_answer`). Then the item's id (0 or item -1: the default command id): the Tactics menu
/// (`tactics_event_menu`), Last Match (`tactics_event_last_match`), Edit and the tabs (`tactics_event_edit`), the
/// player list (`tactics_event_list`), the pitch (`tactics_event_pitch`) and Cancel / Ok (`tactics_event_leave`).
/// Anything else goes to the menu bar (`menubar_handle_command`, then `menubar_handle_object_link`), else 0.
/// Results: -4 done, -9 the screen closed, -0xb redraw. A local Text holds the leave-the-match confirmation.
/// Original: `tactics_screen_handle_event` (0x89f710, tactics_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn tactics_screen_handle_event(item: u32) -> i32 {
    let mut text = [0u32; 4];
    let tx = text.as_mut_ptr() as usize;
    display_method1(FN_TEXT_TEXT, tx, 0);
    let r = tactics_screen_handle_event_body(item, tx);
    display_method(FN_TEXT_DESTRUCTOR_TEXT, tx);
    r
}

/// Comparer for `tactics_screen_handle_event`.
pub(crate) unsafe extern "C" fn compare_tactics_screen_handle_event(item: u32) -> i32 {
    use tactics_screen as t;
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_TACTICS_SCREEN_HANDLE_EVENT);
    let ts = get_screen_param(0);
    let pending = read_u32(TACTICS_PENDING_DISPATCH) != 0 && matches!(read_u32(G_PENDING_COMMAND_CODE), 0x2c..=0x36);
    let id = tel26_item_id(item as i16);
    if ts != 0 && !pending && tactics_event_view_only(id, item as i16) {
        let mut reg = screen_param_regions();
        reg.push((ts, t::SIZE as usize));
        reg.push((G_PENDING_COMMAND_CODE, 8));
        let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&reg, || o(item), || tactics_screen_handle_event(item));
        SCOREBOARD.record(equal && ro == rr, || {
            let dif: Vec<String> = mo.0.iter().zip(mr.0.iter()).filter_map(|((a, x), (_, y))| {
                let k = x.iter().zip(y.iter()).position(|(p, q)| p != q)?;
                Some(format!("{:#x}+{:#x}", a, k))
            }).collect();
            format!("tactics screen event (id {:#x}): original={} rust={} differences at {:?}", id, ro, rr, dif)
        });
        return ro;
    }
    let undo = ts != 0 && !pending && id == 0xf && read_u32(ts + t::CAN_UNDO) != 0;
    let cancel = ts != 0 && !pending && id == 0x20 && read_u32(ts + tactics_event_fields::CHANGES_MADE) == 0;
    let own = ts != 0 && !pending && (1..=0x22).contains(&id) && id != 0x1c;
    let r = o(item);
    if undo {
        let same = core::slice::from_raw_parts((ts + t::FORMATION) as *const u8, tactics_event_fields::TEAM_DATA_SIZE)
            == core::slice::from_raw_parts((ts + t::UNDO_DATA) as *const u8, tactics_event_fields::TEAM_DATA_SIZE);
        SCOREBOARD.record(same && read_u32(ts + t::CAN_UNDO) == 0 && r == TACTICS_EVENT_REDRAW, || format!("tactics screen undo: result {}, working date restored {}, can undo {}", r, same, read_u32(ts + t::CAN_UNDO)));
    } else if cancel {
        SCOREBOARD.record(r == TACTICS_EVENT_CLOSED, || format!("tactics screen cancel without changes: result {}, plan -9", r));
    } else if own && matches!(id, 0xf..=0x15 | 0x1a | 0x1b | 0x1f) {
        SCOREBOARD.record(r == TACTICS_EVENT_REDRAW, || format!("tactics screen event (id {:#x}): result {}, plan -0xb", id, r));
    }
    r
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
    let fixture = Fixture(ts + s::FIXTURE_COPY);
    if record > -1 {
        covered!("tel15_1 tactics screen of a match record");
        if exe::match_stats_read_match_record(read_usize(G_MATCH_STATS), (record) as i32 as u32, ts + s::RECORD) == 0 {
            tel26_error_box(TACTICS_SCREENS_CPP, 0x12d6);
            exe::drop_llist(list);
            return ts;
        }
        if exe::match_stats_find_record_fixture(read_usize(G_MATCH_STATS), (read_i32(ts + s::RECORD_INDEX)) as i32 as usize, fixture) == 0 {
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
        let fx = Fixture(read_usize(m + engine::MATCH));
        core::ptr::copy(fx.addr() as *const u8, fixture.addr() as *mut u8, 0x4f);
        write_u32(ts + t::HAS_MATCH_RECORD, 1);
        let home = if !fx.is_null() { fx.home_club().addr() } else { 0 };
        let at_home = club == home;
        let other = if fx.is_null() { 0 } else if at_home { fx.away_club().addr() } else { fx.home_club().addr() };
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
                    core::ptr::copy(f as *const u8, fixture.addr() as *mut u8, 0x4f);
                    write_u32(ts + t::HAS_MATCH_RECORD, 1);
                    break;
                }
                f = exe::llist_get_next(list);
            }
        }
        if read_u32(ts + t::HAS_MATCH_RECORD) != 0 {
            let home = fixture.home_club().addr();
            write_usize(ts + s::OPPONENT, if club == home { fixture.away_club().addr() } else { home });
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
            exe::tactics_set_set_piece_taker(tactics, (club) as u32 as usize, kind, choice, (taker) as u32 as usize, reserve);
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
        exe::match_events_add_event(read_usize(m + engine::COMMENTATOR), event, (m) as usize as u32, u32::MAX, side, u32::MAX, u32::MAX, 0);
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
    exe::database_get_club_kit_colours(club, read_u8(ts + tactics_screen::KIT) as u32, (colour_a) as u32 as *mut u32, (colour_b) as u32 as *mut u32);
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
    let score: unsafe extern "C" fn(usize, usize, usize) -> i16 = core::mem::transmute(FN_MATCH_MAN_GET_CAPTAIN_SCORE);
    let (mut best, mut top) = (-1i8, 0i16);
    for row in 0..0x14i8 {
        let mp = MatchPlayer(tactics_row_match_player(ts, row, false));
        if mp.position() < 0 {
            continue;
        }
        let v = score(0, mp.addr(), read_usize(ts + tactics_screen::CLUB));
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
    let opponent = Club(read_usize(ts + s::OPPONENT));
    if opponent.is_null() || read_u32(ts + tactics_screen::HAS_MATCH_RECORD) == 0 {
        return 0;
    }
    let fixture = Fixture(ts + s::FIXTURE_COPY);
    let mut candidates = [0usize; 50];
    let mut count = 0u8;
    for k in 0..0x32usize {
        let p = Staff(opponent.squad(k));
        if p.is_null() || p.player().addr() == 0 {
            continue;
        }
        if exe::injury_get_player_injury_status(read_usize(G_INJURY), p) == 2 {
            continue;
        }
        if tactics_screen_calls::call4(0x55d270, read_usize(G_DISCIPLINE), p.addr() as u32, read_u32(ts + tactics_screen::COMP), 1, 0) != 0 {
            continue;
        }
        let mut date = [0u8; 8];
        exe::new_date(date.as_mut_ptr() as usize, fixture.day() as u32, fixture.year() as u32);
        if exe::nationalteams_get_international_duty_club(read_usize(G_NATIONAL_TEAMS), p, date.as_mut_ptr() as usize, opponent, 0, 0) == 0 {
            candidates[count as usize] = p.addr();
            count += 1;
        }
    }
    if count == 0 {
        return 0;
    }
    covered!("tel15_1 predict the opponent's line-up");
    let tactic = tactics_screen_calls::call2(0x896660, read_usize(G_TACTICS), opponent.addr() as u32, 0) as usize;
    if tactic != 0 {
        for k in 0..0xbu32 {
            positions[k as usize] = tactics_screen_calls::call2(tactics_screen_fns::FORMATION_GET_SLOT_POSITION, tactic, k, 0) as u16;
        }
    }
    let flags: u32 = if exe::database_is_national_team(opponent) != 0 { 0x2014 } else { 0x2042 };
    exe::match_man_assign_players_to_positions(candidates.as_ptr() as usize, count as u32, positions.as_mut_ptr() as usize, 0xb, (read_u8(ts + tactics_screen::BENCH_SIZE) & 0xf) as u32, ts + s::OPPONENT_LINEUP, flags, opponent, 0, 0, 0, 0, 0, 0, 0);
    1
}

/// Comparer for `tactics_screen_constructor`.
pub(crate) unsafe extern "thiscall" fn compare_tactics_screen_constructor(ts: usize, club: usize, m: usize, reserve: u32, arg4: u32, record: i32, penalty: u32) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, i32, u32) -> usize>(IDX_TACTICS_SCREEN_CONSTRUCTOR)(ts, club, m, reserve, arg4, record, penalty);
    if club != 0 && read_u32(ts + tactics_screen::READ_ONLY) <= 1 {
        let args = [read_u32(ts), read_u32(ts + 4), read_u32(ts + 8), read_u32(ts + 0xc), read_u32(ts + 0x10), read_u32(ts + 0x14)];
        let plan = [club as u32, m as u32, reserve, arg4, record as u32, penalty];
        let n = tactics_screen_state::TEAM_DATA_SIZE;
        let same = core::slice::from_raw_parts((ts + tactics_screen::FORMATION) as *const u8, n) == core::slice::from_raw_parts((ts + tactics_screen::CLIPBOARD_DATA) as *const u8, n);
        SCOREBOARD.record(args == plan && same, || format!("tactics screen constructor: arguments {:x?} plan {:x?}, clipboard copy {}", args, plan, same));
    }
    r
}

/// Comparer for `tactics_screen_copy_constructor`.
pub(crate) unsafe extern "thiscall" fn compare_tactics_screen_copy_constructor(ts: usize, src: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_TACTICS_SCREEN_COPY_CONSTRUCTOR)(ts, src);
    if src != 0 {
        let n = tactics_screen_state::TEAM_DATA_SIZE;
        let same = core::slice::from_raw_parts((ts + tactics_screen::FORMATION) as *const u8, n) == core::slice::from_raw_parts((src + tactics_screen::FORMATION) as *const u8, n);
        SCOREBOARD.record(same, || "tactics screen copy constructor: working team data not copied".to_string());
    }
    r
}

/// Comparer for `tactics_screen_destructor`.
pub(crate) unsafe extern "thiscall" fn compare_tactics_screen_destructor(ts: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_TACTICS_SCREEN_DESTRUCTOR)(ts)
}

/// Comparer for `tactics_screen_close_and_apply`.
pub(crate) unsafe extern "C" fn compare_tactics_screen_close_and_apply() {
    let had = get_screen_param(0) != 0;
    original_fn::<unsafe extern "C" fn()>(IDX_TACTICS_SCREEN_CLOSE_AND_APPLY)();
    SCOREBOARD.record(!had || get_screen_param(0) == 0, || "tactics_screen_close_and_apply: parameter 0 not cleared".to_string());
}

/// Comparer for `match_stats_record_assign`: side by side over the destination record.
pub(crate) unsafe extern "thiscall" fn compare_match_stats_record_assign(this: usize, src: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_MATCH_STATS_RECORD_ASSIGN);
    compare_tactics_method("match_stats_record_assign", &[(this, 0xd7a)], || o(this, src), || match_stats_record_assign(this, src))
}

/// Comparer for `apply_changes`.
pub(crate) unsafe extern "thiscall" fn compare_apply_changes(ts: usize) -> u32 {
    let read_only = read_u32(ts + tactics_screen::READ_ONLY) != 0;
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_APPLY_CHANGES)(ts);
    SCOREBOARD.record((r == 0) == read_only, || format!("tactics screen apply_changes: {} on a screen read-only {}", r, read_only));
    r
}

/// Comparer for `format_title`: over the title and the two colours.
pub(crate) unsafe extern "thiscall" fn compare_format_title(ts: usize, out: u32, size: u32, colour_a: u32, colour_b: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u32>(IDX_FORMAT_TITLE);
    if out == 0 || (size as i32) < 0x64 || colour_a == 0 || colour_b == 0 {
        return o(ts, out, size, colour_a, colour_b); // simplification: the box, original only
    }
    compare_tactics_method("format_title", &[(out as usize, size.min(0x100) as usize), (colour_a as usize, 2), (colour_b as usize, 2)],
                           || o(ts, out, size, colour_a, colour_b), || format_title(ts, out, size, colour_a, colour_b))
}

/// Comparer for `auto_select_captain`: side by side over the screen object.
pub(crate) unsafe extern "thiscall" fn compare_auto_select_captain(ts: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_AUTO_SELECT_CAPTAIN);
    compare_tactics_method("auto_select_captain", &[tactics_screen_region(ts)], || o(ts), || auto_select_captain(ts))
}

/// Comparer for `predict_opponent_lineup`: side by side over the screen object (and the random generator).
pub(crate) unsafe extern "thiscall" fn compare_predict_opponent_lineup(ts: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_PREDICT_OPPONENT_LINEUP);
    let mut reg = vec![tactics_screen_region(ts)];
    reg.extend_from_slice(&crate::rng::STATE_REGIONS);
    compare_tactics_method("predict_opponent_lineup", &reg, || o(ts), || predict_opponent_lineup(ts))
}

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
            let player = Staff(tactics_screen_calls::call2(tactics_screen_fns::GET_SLOT_PLAYER, ts, row as u32, 0) as usize);
            if player.is_null() || player.player().addr() == 0 {
                continue;
            }
            if tactics_screen_calls::call3(tactics_screen_fns::GET_SLOT_PITCH_POSITION, ts, row as u32, 0, 0) == 0 || used[row] != 0 {
                continue;
            }
            let number = tactics_penalty_squad_number(player.addr());
            let best_number = if best_player != 0 { tactics_penalty_squad_number(best_player) } else { 0x33 };
            if best == -1 || number < best_number {
                best = row as i8;
                best_player = player.addr();
            } else if number == best_number {
                let mine = read_i8(player.player().addr() + player_sheet::PENALTIES);
                let theirs = read_i8(read_usize(best_player + staff::PLAYER) + player_sheet::PENALTIES);
                if mine > theirs {
                    best = row as i8;
                    best_player = player.addr();
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
        let club = Club(read_usize(ts + tactics_screen::CLUB));
        if read_usize(ts + tactics_screen::MATCH) != 0 && exe::human_manager_current_controls_club(club, 0, 1) != 0 {
            covered!("tel15_2 save the formation of a live match");
            exe::human_manager_select_tactic_slot(read_usize(G_HUMAN_MANAGER), read_u32(ts + tactics_screen_state::RESERVE) as i32, club, 0usize);
            let active = exe::human_manager_get_active_tactic(read_usize(G_HUMAN_MANAGER), club.manager().addr());
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32) -> u32>(IDX_SCREEN_SET_PIECE_TAKER);
    if !(0..=4).contains(&(kind as u8 as i8)) || !(0..=1).contains(&(choice as u8 as i8)) {
        return o(ts, kind, choice, player); // simplification: the box, original only
    }
    compare_tactics_method("screen_set_piece_taker", &[tactics_screen_region(ts)], || o(ts, kind, choice, player), || screen_set_piece_taker(ts, kind, choice, player))
}

/// Comparer for `set_playmaker`.
pub(crate) unsafe extern "thiscall" fn compare_set_playmaker(ts: usize, choice: u32, player: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u32>(IDX_SET_PLAYMAKER);
    if !(0..=1).contains(&(choice as u8 as i8)) {
        return o(ts, choice, player); // simplification: the box, original only
    }
    compare_tactics_method("set_playmaker", &[tactics_screen_region(ts)], || o(ts, choice, player), || set_playmaker(ts, choice, player))
}

/// Comparer for `set_penalty_taker`.
pub(crate) unsafe extern "thiscall" fn compare_set_penalty_taker(ts: usize, order: u32, row: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u32>(IDX_SET_PENALTY_TAKER);
    if !(0..=10).contains(&(order as u8 as i8)) || !(0..=0x13).contains(&(row as u8 as i8)) {
        return o(ts, order, row); // simplification: the box, original only
    }
    compare_tactics_method("set_penalty_taker", &[tactics_screen_region(ts)], || o(ts, order, row), || set_penalty_taker(ts, order, row))
}

/// Comparer for `swap_penalty_takers`.
pub(crate) unsafe extern "thiscall" fn compare_swap_penalty_takers(ts: usize, a: u32, b: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u32>(IDX_SWAP_PENALTY_TAKERS);
    if !(0..=10).contains(&(a as u8 as i8)) || !(0..=10).contains(&(b as u8 as i8)) {
        return o(ts, a, b); // simplification: the box, original only
    }
    compare_tactics_method("swap_penalty_takers", &[tactics_screen_region(ts)], || o(ts, a, b), || swap_penalty_takers(ts, a, b))
}

/// Comparer for `set_default_penalty_takers`: side by side over the screen object and the club's Tactics row.
pub(crate) unsafe extern "thiscall" fn compare_set_default_penalty_takers(ts: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SET_DEFAULT_PENALTY_TAKERS);
    let mut reg = vec![tactics_screen_region(ts)];
    if let Some(row) = tactics_row_of_club(read_usize(G_TACTICS), read_usize(ts + tactics_screen::CLUB), read_u32(ts + tactics_screen_state::RESERVE)) {
        reg.push((row, 0x91));
    }
    compare_tactics_method("set_default_penalty_takers", &reg, || o(ts), || set_default_penalty_takers(ts))
}

/// Comparer for `get_slot_coordinate`: side by side over the output coordinate.
pub(crate) unsafe extern "thiscall" fn compare_get_slot_coordinate(ts: usize, slot: u32, out: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u32>(IDX_GET_SLOT_COORDINATE);
    if !(0..=10).contains(&(slot as u8 as i8)) || out == 0 {
        return o(ts, slot, out); // simplification: the box, original only
    }
    compare_tactics_method("get_slot_coordinate", &[(out as usize, 4)], || o(ts, slot, out), || get_slot_coordinate(ts, slot, out))
}

/// Comparer for `load_formation`.
pub(crate) unsafe extern "thiscall" fn compare_load_formation(ts: usize, path: u32, preset: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u32>(IDX_LOAD_FORMATION);
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
    SCOREBOARD.record(ok, || format!("tactics screen load_formation({:#x}) {}: team date or undo off the plan", path, r));
    r
}

/// Comparer for `save_formation`.
pub(crate) unsafe extern "thiscall" fn compare_save_formation(ts: usize, path: u32, locking_manager: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u32>(IDX_SAVE_FORMATION);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(IDX_SET_PRESET_FORMATION);
    let k = preset as u16 as i16;
    if k < 0 || k as i32 >= read_i32(read_usize(G_TACTICS) + 0x5fd) {
        return o(ts, preset); // simplification: the box, original only
    }
    compare_tactics_method("set_preset_formation", &[tactics_screen_region(ts)], || o(ts, preset), || set_preset_formation(ts, preset))
}

/// Comparer for `set_player_instruction`.
pub(crate) unsafe extern "thiscall" fn compare_set_player_instruction(ts: usize, slot: u32, flag_lo: u32, flag_hi: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32) -> u32>(IDX_SET_PLAYER_INSTRUCTION);
    compare_tactics_method("set_player_instruction", &[tactics_screen_region(ts)], || o(ts, slot, flag_lo, flag_hi), || set_player_instruction(ts, slot, flag_lo, flag_hi))
}

/// Comparer for `paste_positions`.
pub(crate) unsafe extern "thiscall" fn compare_paste_positions(ts: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_PASTE_POSITIONS);
    compare_tactics_method("paste_positions", &[tactics_screen_region(ts)], || o(ts), || paste_positions(ts))
}

/// Comparer for `swap_slot_coordinates`.
pub(crate) unsafe extern "thiscall" fn compare_swap_slot_coordinates(ts: usize, a: u32, b: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u32>(IDX_SWAP_SLOT_COORDINATES);
    if !(0..=10).contains(&(a as u8 as i8)) || !(0..=10).contains(&(b as u8 as i8)) {
        return o(ts, a, b); // simplification: the box, original only
    }
    compare_tactics_method("swap_slot_coordinates", &[tactics_screen_region(ts)], || o(ts, a, b), || swap_slot_coordinates(ts, a, b))
}

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
        let mp = MatchPlayer(tactics_row_match_player(ts, r, opp));
        if (mp.position() as u8) == 0xfb {
            return 1;
        }
        let staff = mp.staff().addr();
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
            let f: unsafe extern "C" fn(u32, usize, u32, u32) -> u32 = core::mem::transmute(FN_FORMATION_GET_POSITION_ABBREVIATION);
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
        exe::formation_get_position_name(position, outp, size, 1)
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
    let mp = MatchPlayer(tactics_row_match_player(ts, r, opp));
    let position = mp.position();
    if position < 0 && (read_u8(m + engine::SHOOTOUT_STATE) != 0 || position as u8 != 0xfe) {
        return 0;
    }
    if out != 0 {
        write_i8(out as usize, position);
        if position as u8 == 0xfe {
            covered!("tel15_3 pitch position of a player off injured");
            write_i8(out as usize, mp.previous_position());
        }
    }
    1
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

/// Comparer for `set_formation_slot_coordinate`.
pub(crate) unsafe extern "thiscall" fn compare_set_formation_slot_coordinate(ts: usize, slot: u32, coordinate: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u32>(IDX_SET_FORMATION_SLOT_COORDINATE);
    if !(0..=10).contains(&(slot as u8 as i8)) {
        return o(ts, slot, coordinate); // simplification: the box, original only
    }
    compare_tactics_method("set_formation_slot_coordinate", &[tactics_screen_region(ts)], || o(ts, slot, coordinate), || set_formation_slot_coordinate(ts, slot, coordinate))
}

/// Comparer for `is_formation_slot_customised`.
pub(crate) unsafe extern "thiscall" fn compare_is_formation_slot_customised(ts: usize, slot: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(IDX_IS_FORMATION_SLOT_CUSTOMISED);
    if !(0..=10).contains(&(slot as u8 as i8)) {
        return o(ts, slot); // simplification: the box, original only
    }
    compare_tactics_method("is_formation_slot_customised", &[], || o(ts, slot), || is_formation_slot_customised(ts, slot))
}

/// Comparer for `set_slot_marking_type`.
pub(crate) unsafe extern "thiscall" fn compare_set_slot_marking_type(ts: usize, slot: u32, marking: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u32>(IDX_SET_SLOT_MARKING_TYPE);
    if !(0..=10).contains(&(slot as u8 as i8)) {
        return o(ts, slot, marking); // simplification: the box, original only
    }
    compare_tactics_method("set_slot_marking_type", &[tactics_screen_region(ts)], || o(ts, slot, marking), || set_slot_marking_type(ts, slot, marking))
}

/// Comparer for `set_slot_marking_target`.
pub(crate) unsafe extern "thiscall" fn compare_set_slot_marking_target(ts: usize, slot: u32, target: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u32>(IDX_SET_SLOT_MARKING_TARGET);
    if !(0..=10).contains(&(slot as u8 as i8)) {
        return o(ts, slot, target); // simplification: the box, original only
    }
    compare_tactics_method("set_slot_marking_target", &[tactics_screen_region(ts)], || o(ts, slot, target), || set_slot_marking_target(ts, slot, target))
}

/// Comparer for `get_slot_number_text`.
pub(crate) unsafe extern "thiscall" fn compare_get_slot_number_text(ts: usize, row: u32, out: u32, size: u32, no_fallback: u32, opponent: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> u32>(IDX_GET_SLOT_NUMBER_TEXT);
    if !(0..=0x13).contains(&(row as u8 as i8)) || out == 0 || (size as i32) < 0xa {
        return o(ts, row, out, size, no_fallback, opponent); // simplification: the box, original only
    }
    compare_tactics_method("get_slot_number_text", &[(out as usize, size.min(0x100) as usize)], || o(ts, row, out, size, no_fallback, opponent),
                           || get_slot_number_text(ts, row, out, size, no_fallback, opponent))
}

/// Comparer for `get_slot_player_name_text`.
pub(crate) unsafe extern "thiscall" fn compare_get_slot_player_name_text(ts: usize, row: u32, out: u32, size: u32, no_fallback: u32, format: u32, opponent: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32) -> u32>(IDX_GET_SLOT_PLAYER_NAME_TEXT);
    if !(0..=0x13).contains(&(row as u8 as i8)) || out == 0 || (size as i32) < 0x64 {
        return o(ts, row, out, size, no_fallback, format, opponent); // simplification: the box, original only
    }
    compare_tactics_method("get_slot_player_name_text", &[(out as usize, size.min(0x100) as usize)], || o(ts, row, out, size, no_fallback, format, opponent),
                           || get_slot_player_name_text(ts, row, out, size, no_fallback, format, opponent))
}

/// Comparer for `get_slot_number_and_name_text`.
pub(crate) unsafe extern "thiscall" fn compare_get_slot_number_and_name_text(ts: usize, row: u32, out: u32, size: u32, format: u32, opponent: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> u32>(IDX_GET_SLOT_NUMBER_AND_NAME_TEXT);
    if !(0..=0x13).contains(&(row as u8 as i8)) || out == 0 || (size as i32) < 0x64 {
        return o(ts, row, out, size, format, opponent); // simplification: the box, original only
    }
    compare_tactics_method("get_slot_number_and_name_text", &[(out as usize, size.min(0x100) as usize)], || o(ts, row, out, size, format, opponent),
                           || get_slot_number_and_name_text(ts, row, out, size, format, opponent))
}

/// Comparer for `get_slot_pitch_position`.
pub(crate) unsafe extern "thiscall" fn compare_get_slot_pitch_position(ts: usize, row: u32, out: u32, opponent: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32) -> u32>(IDX_GET_SLOT_PITCH_POSITION);
    let reg: &[(usize, usize)] = if out != 0 { &[(out as usize, 1)] } else { &[] };
    compare_tactics_method("get_slot_pitch_position", reg, || o(ts, row, out, opponent), || get_slot_pitch_position(ts, row, out, opponent))
}

/// Comparer for `is_slot_on_bench`.
pub(crate) unsafe extern "thiscall" fn compare_is_slot_on_bench(ts: usize, row: u32, opponent: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u32>(IDX_IS_SLOT_ON_BENCH);
    if !(0..=0x13).contains(&(row as u8 as i8)) {
        return o(ts, row, opponent); // simplification: the box, original only
    }
    compare_tactics_method("is_slot_on_bench", &[], || o(ts, row, opponent), || is_slot_on_bench(ts, row, opponent))
}

/// Comparer for `get_slot_yellow_cards`.
pub(crate) unsafe extern "thiscall" fn compare_get_slot_yellow_cards(ts: usize, row: u32, opponent: u32) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u8>(IDX_GET_SLOT_YELLOW_CARDS);
    if !(0..=0x13).contains(&(row as u8 as i8)) {
        return o(ts, row, opponent); // simplification: the box, original only
    }
    compare_tactics_method("get_slot_yellow_cards", &[], || o(ts, row, opponent), || get_slot_yellow_cards(ts, row, opponent))
}

/// Comparer for `get_slot_red_cards`.
pub(crate) unsafe extern "thiscall" fn compare_get_slot_red_cards(ts: usize, row: u32, opponent: u32) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u8>(IDX_GET_SLOT_RED_CARDS);
    if !(0..=0x13).contains(&(row as u8 as i8)) {
        return o(ts, row, opponent); // simplification: the box, original only
    }
    compare_tactics_method("get_slot_red_cards", &[], || o(ts, row, opponent), || get_slot_red_cards(ts, row, opponent))
}

/// Comparer for `get_slot_match_status_text`: over `out`, the colour and the shared text.
pub(crate) unsafe extern "thiscall" fn compare_get_slot_match_status_text(ts: usize, row: u32, out: u32, size: u32, colour: u32, opponent: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> u32>(IDX_GET_SLOT_MATCH_STATUS_TEXT);
    if !(0..=0x13).contains(&(row as u8 as i8)) || out == 0 || (size as i32) < 0x14 || colour == 0 {
        return o(ts, row, out, size, colour, opponent); // simplification: the box, original only
    }
    compare_tactics_method("get_slot_match_status_text", &[(out as usize, 1), (colour as usize, 2), (JOB_TEXT, 0x40)], || o(ts, row, out, size, colour, opponent),
                           || get_slot_match_status_text(ts, row, out, size, colour, opponent))
}

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
        let mp = MatchPlayer(tactics_row_match_player(ts, r, opp));
        if (mp.position() as u8) == 0xfb {
            return 0;
        }
        covered!("tel15_4 condition in a live match");
        return ftol(mp.stamina() as f64 * read_f64(rdata::F64_0_01)) as u32;
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
        if exe::database_staff_matches_names(staff, read_u32(rr + match_record_row::FIRST_NAME_ID), read_u32(rr + match_record_row::SECOND_NAME_ID), read_u32(rr + match_record_row::COMMON_NAME_ID), rr + match_record_row::DATE_OF_BIRTH) == 0 {
            return 0;
        }
        covered!("tel15_4 player of a match record row");
        return staff;
    }
    if read_usize(ts + tactics_screen::MATCH) != 0 {
        let mp = MatchPlayer(tactics_row_match_player(ts, r, opp));
        if (mp.position() as u8) == 0xfb {
            return 0;
        }
        return mp.staff().addr();
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
        let mp = MatchPlayer(tactics_row_match_player(ts, row, opp));
        let at = mp.position();
        if at == p {
            return row as u8;
        }
        if at as u8 == 0xfe && mp.previous_position() == p {
            covered!("tel15_4 position of a player off injured");
            write_u32(pending as usize, 1);
            return row as u8;
        }
    }
    0xff
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
    exe::match_events_add_event(read_usize(m + engine::COMMENTATOR), event, (m) as usize as u32, on, side as u32, off, side as u32, 0);
    1
}

/// Comparer for `get_slot_condition` (al only when the original clears just al).
pub(crate) unsafe extern "thiscall" fn compare_get_slot_condition(ts: usize, row: u32, opponent: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u32>(IDX_GET_SLOT_CONDITION);
    if !(0..=0x13).contains(&(row as u8 as i8)) {
        return o(ts, row, opponent); // simplification: the box, original only
    }
    let mut r = 0;
    compare_tactics_method("get_slot_condition", &[], || { r = o(ts, row, opponent); r as u8 }, || get_slot_condition(ts, row, opponent) as u8);
    r
}

/// Comparer for `get_slot_match_rating`.
pub(crate) unsafe extern "thiscall" fn compare_get_slot_match_rating(ts: usize, row: u32, opponent: u32) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u8>(IDX_GET_SLOT_MATCH_RATING);
    if !(0..=0x13).contains(&(row as u8 as i8)) {
        return o(ts, row, opponent); // simplification: the box, original only
    }
    compare_tactics_method("get_slot_match_rating", &[], || o(ts, row, opponent), || get_slot_match_rating(ts, row, opponent))
}

/// Comparer for `get_slot_goals`.
pub(crate) unsafe extern "thiscall" fn compare_get_slot_goals(ts: usize, row: u32, opponent: u32) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u8>(IDX_GET_SLOT_GOALS);
    if !(0..=0x13).contains(&(row as u8 as i8)) {
        return o(ts, row, opponent); // simplification: the box, original only
    }
    compare_tactics_method("get_slot_goals", &[], || o(ts, row, opponent), || get_slot_goals(ts, row, opponent))
}

/// Comparer for `is_slot_man_of_the_match`.
pub(crate) unsafe extern "thiscall" fn compare_is_slot_man_of_the_match(ts: usize, row: u32, opponent: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u32>(IDX_IS_SLOT_MAN_OF_THE_MATCH);
    if !(0..=0x13).contains(&(row as u8 as i8)) {
        return o(ts, row, opponent); // simplification: the box, original only
    }
    compare_tactics_method("is_slot_man_of_the_match", &[], || o(ts, row, opponent), || is_slot_man_of_the_match(ts, row, opponent))
}

/// Comparer for `get_slot_player`.
pub(crate) unsafe extern "thiscall" fn compare_get_slot_player(ts: usize, row: u32, opponent: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> usize>(IDX_GET_SLOT_PLAYER);
    if !(0..=0x13).contains(&(row as u8 as i8)) {
        return o(ts, row, opponent); // simplification: the box, original only
    }
    compare_tactics_method("get_slot_player", &[], || o(ts, row, opponent), || get_slot_player(ts, row, opponent))
}

/// Comparer for `find_slot_at_position`.
pub(crate) unsafe extern "thiscall" fn compare_find_slot_at_position(ts: usize, position: u32, pending: u32, opponent: u32) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32) -> u8>(IDX_FIND_SLOT_AT_POSITION);
    if !(0..=10).contains(&(position as u8 as i8)) || pending == 0 {
        return o(ts, position, pending, opponent); // simplification: the box, original only
    }
    compare_tactics_method("find_slot_at_position", &[(pending as usize, 4)], || o(ts, position, pending, opponent), || find_slot_at_position(ts, position, pending, opponent))
}

/// Comparer for `swap_slots`.
pub(crate) unsafe extern "thiscall" fn compare_swap_slots(ts: usize, a: u32, b: u32, opponent: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32) -> u32>(IDX_SWAP_SLOTS);
    if !(0..=0x13).contains(&(a as u8 as i8)) || !(0..=0x13).contains(&(b as u8 as i8)) {
        return o(ts, a, b, opponent); // simplification: the box, original only
    }
    compare_tactics_method("swap_slots", &[tactics_screen_region(ts)], || o(ts, a, b, opponent), || swap_slots(ts, a, b, opponent))
}

/// Comparer for `get_slot_marking_target_row`.
pub(crate) unsafe extern "thiscall" fn compare_get_slot_marking_target_row(ts: usize, row: u32, opponent: u32) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u8>(IDX_GET_SLOT_MARKING_TARGET_ROW);
    if !(0..=0x13).contains(&(row as u8 as i8)) {
        return o(ts, row, opponent); // simplification: the box, original only
    }
    compare_tactics_method("get_slot_marking_target_row", &[], || o(ts, row, opponent), || get_slot_marking_target_row(ts, row, opponent))
}

/// Comparer for `get_slot_marked_player`.
pub(crate) unsafe extern "thiscall" fn compare_get_slot_marked_player(ts: usize, row: u32, opponent: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> usize>(IDX_GET_SLOT_MARKED_PLAYER);
    if !(0..=0x13).contains(&(row as u8 as i8)) {
        return o(ts, row, opponent); // simplification: the box, original only
    }
    compare_tactics_method("get_slot_marked_player", &[], || o(ts, row, opponent), || get_slot_marked_player(ts, row, opponent))
}

/// Comparer for `get_slot_marker`.
pub(crate) unsafe extern "thiscall" fn compare_get_slot_marker(ts: usize, row: u32, opponent: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> usize>(IDX_GET_SLOT_MARKER);
    if !(0..=0x13).contains(&(row as u8 as i8)) {
        return o(ts, row, opponent); // simplification: the box, original only
    }
    compare_tactics_method("get_slot_marker", &[], || o(ts, row, opponent), || get_slot_marker(ts, row, opponent))
}

/// Comparer for `queue_substitution`.
pub(crate) unsafe extern "thiscall" fn compare_queue_substitution(ts: usize, on: u32, off: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u32>(IDX_QUEUE_SUBSTITUTION);
    if !(0..=0x13).contains(&(on as u8 as i8)) || !(0..=0x13).contains(&(off as u8 as i8)) {
        return o(ts, on, off); // simplification: the box, original only
    }
    compare_tactics_method("queue_substitution", &[tactics_screen_region(ts)], || o(ts, on, off), || queue_substitution(ts, on, off))
}

/// Comparer for `apply_substitution`.
pub(crate) unsafe extern "thiscall" fn compare_apply_substitution(ts: usize, on: u32, off: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u32>(IDX_APPLY_SUBSTITUTION);
    let r = o(ts, on, off);
    if r != 0 {
        let (pon, poff) = (tactics_row_live_player(ts, on as u8 as i8), tactics_row_live_player(ts, off as u8 as i8));
        let ok = read_i8(poff + match_player::POSITION) == -4 && read_u8(poff + match_player::SUBSTITUTED_OFF) == 1
            && read_i8(poff + match_player::MINUTE_OFF) == read_i8(pon + match_player::MINUTE_ON);
        SCOREBOARD.record(ok, || format!("tactics screen apply_substitution({}, {}): the player going off is not substituted off", on as u8 as i8, off as u8 as i8));
    }
    r
}

/// Builds the team instructions dialog over the tactics screen (the Tactics_Screen copy of parameter 0).
///
/// The first build sets the choice green (`tactics_dialog_statics`). No background picture; the whole-screen area,
/// then the dialog box (0x145 high read-only, else 0x1ea, centred in 0x258; dark green rgb 0, 0x38, 0), the title
/// "Team Instructions", the buttons row at its bottom: Cancel (event 1) and Ok (2), or only Ok across it read-only.
/// A pitch (`draw_pitch`, scale 2) in the buttons area. The instruction area (5 columns, weights 2, 1, 1, 1, 1; 12
/// rows, 7 read-only) with the seven rows of `TEAM_INSTRUCTION_ROWS`: the heading and each choice, ticked when its bit
/// is set in the formation's team instruction flags, else a button with event 3 and the bit (read-only: the text).
/// Editable, the set piece area (weights 8, 7, 1, 7, 1) with the headings Free Kicks (L), (R), Corners (L), (R) and
/// Playmaker on rows 7..11, and for each taker choice of `TEAM_TAKER_CHOICES` (a second choice only once the first
/// is set) a popup next to it listing the players on the pitch ("<number> <name>", event the choice's id, value the
/// player) but the one chosen for the other choice, plus an empty row; under it the chosen taker
/// (`tactics_dialog_chosen`, greyed when not on the pitch). The two playmaker choices the same with events 0xe and
/// 0xf. Asserts on the taker and playmaker indices that cannot fail (lines 0x1618 and 0x165f) are left out.
/// Original: `tactics_build_team_instructions_dialog` (0x8a2db0, tactics_screens.cpp, cdecl; 0x8a4830 jumps here).
pub(crate) unsafe extern "C" fn tactics_build_team_instructions_dialog() {
    use tactics_dialog_statics as s;
    use tactics_screen as t;
    covered!("tel15_5 build the team instructions");
    let c = tactics_colour;
    tactics_dialog_choice_colour(s::TEAM_GUARD, s::TEAM_CHOICE);
    let widths: [u8; 7] = [3, 1, 2, 1, 1, 1, 1];
    let taker_widths: [u8; 5] = [8, 7, 1, 7, 1];
    let ts = get_screen_param(0);
    exe::background_set_picture(0usize);
    tactics_area([0, 0, 0x31f, 0x257, 1, 0, 1, 0, 2, 0, u32::MAX]);
    let read_only = || read_u32(ts + t::READ_ONLY) != 0;
    let height: i32 = if read_only() { 0x145 } else { 0x1ea };
    let mut top = (0x258 - height) / 2;
    let mut bottom = top + height - 1;
    let back = exe::graphics_rgb_to_pixel(0, 0x38, 0, 0);
    tactics_area([0xaa, top as u32, 0x2cf, bottom as u32, 1, 0, 1, 0, 0x30, back, u32::MAX]);
    tactics_text(0xa8bfa4);
    top += 10;
    bottom -= 10;
    tactics_dialog_text_item([0xb4, top as u32, 0x2c5, (top + 0x2d) as u32, 0x30, c(JOB_COLOUR_BDEA), 0xc, 4, c(JOB_COLOUR_3184), JOB_TEXT as u32, 1]);
    let buttons_top = bottom - 0x1e;
    let buttons = tactics_area([0xb4, buttons_top as u32, 0x2c5, bottom as u32, 2, widths.as_ptr() as u32, 1, 0, 1, 0, u32::MAX]);
    top += 0x37;
    if !read_only() {
        tactics_text(0x98a1a0);
        tel26_area_button([buttons, 0, 0, 0x30, c(JOB_COLOUR_317E), 0xc, 3, c(JOB_COLOUR_3184), JOB_TEXT as u32, 1, 0]);
        tactics_text(0x990f7c);
        tel26_area_button([buttons, 1, 0, 0x30, c(JOB_COLOUR_317E), 0xc, 3, c(JOB_COLOUR_3184), JOB_TEXT as u32, 2, 0]);
    } else {
        tactics_text(0x990f7c);
        tel26_add_button([0xb4, buttons_top as u32, 0x2c5, bottom as u32, 0x30, c(JOB_COLOUR_317E), 0xc, 3, c(JOB_COLOUR_3184), JOB_TEXT as u32, 2, 0]);
    }
    bottom -= 0x32;
    top += 10;
    tactics_dialog_pitch(0xbe, top, 0x2bb, bottom, buttons, 2.0);
    top += 5;
    bottom -= 5;
    let area = tactics_area([0xc3, top as u32, 0x2b6, bottom as u32, 5, widths[2..].as_ptr() as u32, if read_only() { 7 } else { 0xc }, 0, 1, 0, u32::MAX]);
    let flags = read_u32(ts + t::FORMATION + formation_fields::FLAGS);
    for (row, &(heading, choices)) in TEAM_INSTRUCTION_ROWS.iter().enumerate() {
        tactics_text(heading);
        tactics_dialog_heading(area, row as u32);
        for (k, &(format, takes_empty, bit)) in choices.iter().enumerate() {
            tactics_dialog_choice_text(format, takes_empty);
            tactics_dialog_choice(area, k as u32 + 1, row as u32, flags & bit != 0, !read_only(), s::TEAM_CHOICE, 3, bit);
        }
    }
    if read_only() {
        return;
    }
    covered!("tel15_5 the set piece takers and playmakers");
    let takers = tactics_area([0xc3, top as u32, 0x2b6, bottom as u32, 5, taker_widths.as_ptr() as u32, 0xc, 0, 1, 0, u32::MAX]);
    for (k, heading) in [0xa8c40c, 0xa8c3fc, 0xa8c3ec, 0xa8c3dc, 0xa8c3d0].into_iter().enumerate() {
        tactics_text(heading);
        tactics_dialog_heading(takers, 7 + k as u32);
    }
    let (mut number, mut name) = ([0u8; 0xc8], [0u8; 0xc8]);
    let (mut col, mut row) = (1u32, 7u32);
    let taker = |kind: u32, choice: u32| read_u32(ts + tactics_screen_state::SET_PIECE_TAKERS + ((kind * 2 + choice) * 4) as usize);
    for &(kind, choice, id) in TEAM_TAKER_CHOICES.iter() {
        let mut on_pitch = false;
        let mut popup = u32::MAX;
        if choice == 0 || taker(kind, 0) != 0 {
            for r in 0..0x14i8 {
                if tactics_calls::slot_pitch_position(ts, r, core::ptr::null_mut()) == 0 {
                    continue;
                }
                let p = tactics_calls::slot_player(ts, r);
                if p == 0 {
                    continue;
                }
                tactics_dialog_number_text(ts, r, number.as_mut_ptr());
                if p as u32 == taker(kind, choice) {
                    on_pitch = true;
                }
                if p as u32 == taker(kind, 1 - choice) {
                    continue;
                }
                if popup as u16 == 0xffff {
                    popup = tactics_dialog_popup(takers, col + 1, row);
                }
                tactics_dialog_popup_text(p, number.as_ptr(), name.as_mut_ptr());
                tactics_dialog_popup_row(JOB_TEXT as u32, id, p as u32, popup);
            }
            if popup as u16 != 0xffff {
                tactics_dialog_popup_row(0, id, 0, popup);
            }
        }
        let chosen = taker(kind, choice);
        if chosen != 0 {
            tactics_dialog_chosen(ts, takers, col, row, choice > 0, chosen, on_pitch, name.as_mut_ptr());
        }
        col += 2;
        if col > 3 {
            col = 1;
            row += 1;
        }
    }
    let playmaker = |k: u32| read_u32(ts + tactics_screen_state::PLAYMAKERS + (k * 4) as usize);
    let mut popup_col = col + 1;
    for choice in 0..2u32 {
        let mut on_pitch = false;
        let mut popup = u32::MAX;
        let id = 0xe + (choice != 0) as u32;
        if choice == 0 || playmaker(0) != 0 {
            for r in 0..0x14i8 {
                if tactics_calls::slot_pitch_position(ts, r, core::ptr::null_mut()) == 0 {
                    continue;
                }
                let p = tactics_calls::slot_player(ts, r);
                if p == 0 {
                    continue;
                }
                tactics_dialog_number_text(ts, r, number.as_mut_ptr());
                if p as u32 == playmaker(choice) {
                    on_pitch = true;
                }
                if p as u32 == playmaker((choice == 0) as u32) {
                    continue;
                }
                if popup as u16 == 0xffff {
                    popup = tactics_dialog_popup(takers, popup_col, row);
                }
                tactics_dialog_popup_text(p, number.as_ptr(), name.as_mut_ptr());
                tactics_dialog_popup_row(JOB_TEXT as u32, id, p as u32, popup);
            }
            if popup as u16 != 0xffff {
                tactics_dialog_popup_row(0, id, 0, popup);
            }
        }
        let chosen = playmaker(choice);
        if chosen != 0 {
            tactics_dialog_chosen(ts, takers, col, row, choice != 0, chosen, on_pitch, name.as_mut_ptr());
        }
        col += 2;
        popup_col += 2;
    }
}

/// The events of the team instructions dialog (parameter 0 the Tactics_Screen copy it edits, 1 the tactics screen's
/// own object, 0 read-only).
///
/// Cancel (1) throws the copy away. Ok (2) on an editable copy clears its undo flag and copies it over the screen's
/// object (`tactics_screen_assign`), then throws it away; both close (-9). A choice (3) on an editable copy saves the
/// undo copy and sets the team instruction (`Formation::set_team_instruction` with the item's value): refused, the
/// undo copy goes back (Undo withdrawn), else the team data is marked changed. A taker choice sets the set piece
/// taker of the copy (`set_set_piece_taker`): ids 4..0xd are kinds 0..4 (free kicks left and right, corners left and
/// right, penalties), even ids the first choice and odd the second; 0xe and 0xf the first and second playmaker
/// (`set_playmaker`), the player the item's value. These redraw (-0xb). Anything else goes to the menu bar
/// (`menubar_handle_object_link`).
/// Original: `tactics_team_instructions_dialog_handle_event` (0x8a4840, tactics_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn tactics_team_instructions_dialog_handle_event(item: u32) -> i32 {
    use tactics_screen as t;
    let copy = get_screen_param(0);
    let owner = get_screen_param(1);
    let it = item as i16;
    let id = tel26_item_id(it);
    match id {
        1 => {
            covered!("tel15_5 team instructions cancel");
            tactics_dialog_close(copy)
        }
        2 => {
            covered!("tel15_5 team instructions ok");
            if read_u32(copy + t::READ_ONLY) == 0 {
                write_u32(copy + t::CAN_UNDO, 0);
                tactics_screen_assign(owner, copy);
            }
            tactics_dialog_close(copy)
        }
        3 => {
            let value = tel26_item_value(it);
            if read_u32(copy + t::READ_ONLY) != 0 {
                return TACTICS_DIALOG_REDRAW;
            }
            covered!("tel15_5 a team instruction");
            tactics_save_undo(copy);
            if tactics_screen_calls::call1(FN_FORMATION_SET_TEAM_INSTRUCTION, copy + t::FORMATION, value) == 0 {
                tactics_restore_undo(copy);
            } else {
                write_u32(copy + tactics_screen_state::TEAM_DATA_CHANGED, 1);
            }
            TACTICS_DIALOG_REDRAW
        }
        4..=0xd => {
            covered!("tel15_5 a set piece taker");
            let target = get_screen_param(0);
            let k = id as u32 - 4;
            tactics_screen_calls::call3(tactics_dialog_fns::SET_SET_PIECE_TAKER, target, k / 2, k % 2, tel26_item_value(it));
            TACTICS_DIALOG_REDRAW
        }
        0xe | 0xf => {
            covered!("tel15_5 a playmaker");
            let target = get_screen_param(0);
            tactics_screen_calls::call2(tactics_dialog_fns::SET_PLAYMAKER, target, id as u32 - 0xe, tel26_item_value(it));
            TACTICS_DIALOG_REDRAW
        }
        _ => exe::menubar_handle_object_link(item),
    }
}

/// Comparer for `tactics_build_team_instructions_dialog`: side by side over the built display list
/// (`compare_match_screen_build`), the dialog's and the pitch's static colours.
pub(crate) unsafe extern "C" fn compare_tactics_build_team_instructions_dialog() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_TACTICS_BUILD_TEAM_INSTRUCTIONS_DIALOG);
    let extra = [(tactics_dialog_statics::TEAM_GUARD, 1), (tactics_dialog_statics::TEAM_CHOICE, 2), (0xde1e20, 1), (0xde1f34, 2), (0xde1e24, 2)];
    compare_match_screen_build("tactics_build_team_instructions_dialog", &extra, || o(), || tactics_build_team_instructions_dialog());
}

/// Comparer for `tactics_team_instructions_dialog_handle_event`.
pub(crate) unsafe extern "C" fn compare_tactics_team_instructions_dialog_handle_event(item: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_TACTICS_TEAM_INSTRUCTIONS_DIALOG_HANDLE_EVENT);
    let id = tel26_item_id(item as i16);
    let (copy, owner) = (get_screen_param(0), get_screen_param(1));
    match id {
        3..=0xf if copy != 0 => compare_tactics_method("team instructions event", &[tactics_screen_region(copy)], || o(item),
                                                       || tactics_team_instructions_dialog_handle_event(item)),
        1 | 2 => {
            let n = tactics_screen_state::TEAM_DATA_SIZE;
            let applies = id == 2 && copy != 0 && owner != 0 && read_u32(copy + tactics_screen::READ_ONLY) == 0;
            let kept = if applies { core::slice::from_raw_parts((copy + tactics_screen::FORMATION) as *const u8, n).to_vec() } else { Vec::new() };
            let r = o(item);
            let same = !applies || core::slice::from_raw_parts((owner + tactics_screen::FORMATION) as *const u8, n) == &kept[..];
            SCOREBOARD.record(r == TACTICS_DIALOG_CLOSED && same, || format!("team instructions event {}: {} applied {}", id, r, same));
            r
        }
        _ => o(item),
    }
}

/// Builds the player instructions dialog of a row (parameter 0 the tactics screen's object, 1 the row).
///
/// The first build sets the choice green (`tactics_dialog_statics`). No background picture; the whole-screen area and
/// the dialog box (dark green rgb 0, 0x38, 0); the title "<number> <name>" of the row (format 1) and a popup button
/// listing the other rows on the pitch (event 1, the row as value) to switch player. Cancel (event 2) and Ok (3); a
/// pitch (`draw_pitch`, scale 4) in the buttons area. The instruction area (6 columns, weights 5, 2, 2, 2, 2, 2; 14
/// rows) with the 13 rows of `PLAYER_INSTRUCTION_ROWS` for the row's position slot (`Formation::get_player_
/// instructions`, 64 bits): each choice ticked when its bit is set, else a button (there is no read-only case). The
/// marking row (row 13, weights 0x28, 0x10, 0x10, 0x10, 0x1b, 5; `Formation::get_marking_type`): Zonal (event 5,
/// value 2), None (1) and Man (4), ticked by the marking type's bits. Man marking adds a popup of the opponents (the
/// match squad of a record, on the pitch in a live match; event 6, the player), the marked player (marking bit 8,
/// `Formation::get_marking_target`) shown as a linked button, the text "  Opposition" or "  Opp (<number and
/// initials>)" of the marked row (`get_slot_marking_target_row`), written in column 4 with marking bit 0x10, and the
/// popup's last row with that text without its two leading spaces (value 0). The opponent index assert (line 0x1b8e)
/// cannot fail and is left out.
/// Original: `tactics_build_player_instructions_dialog` (0x8a5330, tactics_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn tactics_build_player_instructions_dialog() {
    use tactics_dialog_statics as s;
    use tactics_screen as t;
    covered!("tel15_6 build the player instructions");
    let c = tactics_colour;
    tactics_dialog_choice_colour(s::PLAYER_GUARD, s::PLAYER_CHOICE);
    let button_widths: [u8; 2] = [3, 1];
    let instruction_widths: [u8; 6] = [5, 2, 2, 2, 2, 2];
    let marking_widths: [u8; 6] = [0x28, 0x10, 0x10, 0x10, 0x1b, 5];
    let ts = get_screen_param(0);
    let row = get_screen_param(1) as u8;
    exe::background_set_picture(0usize);
    tactics_area([0, 0, 0x31f, 0x257, 1, 0, 1, 0, 2, 0, u32::MAX]);
    let back = exe::graphics_rgb_to_pixel(0, 0x38, 0, 0);
    tactics_area([0x91, 0x37, 0x2e8, 0x220, 1, 0, 1, 0, 0x30, back, u32::MAX]);
    tactics_dialog_number_and_name(ts, row, JOB_TEXT as *mut u8, 0x7d0, 1, 0);
    tactics_dialog_text_item([0x9b, 0x41, 0x2de, 0x6e, 0x30, c(JOB_COLOUR_BDEA), 0xc, 4, c(JOB_COLOUR_3184), JOB_TEXT as u32, 1]);
    let mut popup = u32::MAX;
    for r in 0..0x14u8 {
        if r == row || tactics_calls::slot_pitch_position(ts, r as i8, core::ptr::null_mut()) == 0 {
            continue;
        }
        tactics_dialog_number_and_name(ts, r, JOB_TEXT as *mut u8, 0x7d0, 1, 0);
        if popup as u16 == 0xffff {
            let button = tel26_add_button([0xa0, 0x46, 0xac, 0x56, 0x30, c(JOB_COLOUR_BDEA), 0xc, 1, c(JOB_COLOUR_3184), 0, 0, 0]);
            popup = tactics_area([0, 0, 0, 0, 0, 0, 0, 0, 0x30, c(JOB_COLOUR_31AA), button]);
        }
        tactics_dialog_popup_row(JOB_TEXT as u32, 1, r as u32, popup);
    }
    let buttons = tactics_area([0x9b, 0x1f8, 0x2de, 0x216, 2, button_widths.as_ptr() as u32, 1, 0, 1, 0, u32::MAX]);
    tactics_text(0x98a1a0);
    tel26_area_button([buttons, 0, 0, 0x30, c(JOB_COLOUR_317E), 0xc, 3, c(JOB_COLOUR_3184), JOB_TEXT as u32, 2, 0]);
    tactics_text(0x990f7c);
    tel26_area_button([buttons, 1, 0, 0x30, c(JOB_COLOUR_317E), 0xc, 3, c(JOB_COLOUR_3184), JOB_TEXT as u32, 3, 0]);
    tactics_dialog_pitch(0xa5, 0x82, 0x2d4, 0x1e4, buttons, 4.0);
    let area = tactics_area([0xaa, 0x87, 0x2cf, 0x1df, 6, instruction_widths.as_ptr() as u32, 0xe, 0, 1, 0, u32::MAX]);
    let slot = tactics_dialog_row_slot(ts, row);
    let fm = ts + t::FORMATION;
    let instructions = exe::formation_get_player_instructions(fm, slot as i8);
    let (low, high) = (instructions as u32, (instructions >> 32) as u32);
    for (r, &(heading, choices)) in PLAYER_INSTRUCTION_ROWS.iter().enumerate() {
        tactics_text(heading);
        tactics_dialog_heading(area, r as u32);
        for (k, &(format, takes_empty, in_high, bit)) in choices.iter().enumerate() {
            tactics_dialog_choice_text(format, takes_empty);
            let checked = if in_high { high & bit != 0 } else { low & bit != 0 };
            tactics_dialog_choice(area, k as u32 + 1, r as u32, checked, true, s::PLAYER_CHOICE, if in_high { 7 } else { 4 }, bit);
        }
    }
    let marking_area = tactics_area([0xaa, 0x87, 0x2cf, 0x1df, 6, marking_widths.as_ptr() as u32, 0xe, 0, 1, 0, u32::MAX]);
    let marking = exe::formation_get_marking_type(fm, slot as i8);
    tactics_text1(0xa8be78, TACTICS_EMPTY);
    tactics_dialog_heading(marking_area, 0xd);
    tactics_text1(0xa8b9e8, TACTICS_EMPTY);
    tactics_dialog_choice(marking_area, 1, 0xd, marking & 2 != 0, true, s::PLAYER_CHOICE, 5, 2);
    tactics_text1(0xa8ba0c, TACTICS_EMPTY);
    tactics_dialog_choice(marking_area, 2, 0xd, marking & 1 != 0, true, s::PLAYER_CHOICE, 5, 1);
    tactics_text1(0xa8b984, TACTICS_EMPTY);
    tactics_dialog_choice(marking_area, 3, 0xd, marking & 4 != 0, true, s::PLAYER_CHOICE, 5, 4);
    if marking & 4 == 0 {
        return;
    }
    covered!("tel15_6 man marking: the opponents");
    let popup = tactics_dialog_popup(marking_area, 5, 0xd);
    let mut name = [0u8; 0xc8];
    for r in 0..0x14i8 {
        if read_u32(ts + t::HAS_MATCH_RECORD) != 0 && r >= ((read_u8(ts + t::BENCH_SIZE) & 0xf) + 0xb) as i8 {
            continue;
        }
        let slot_of = |r: i8| tactics_screen_calls::call3(tactics_screen_fns::GET_SLOT_PITCH_POSITION, ts, r as u8 as u32, 0, 1);
        if slot_of(r) == 0 && read_usize(ts + t::MATCH) != 0 {
            continue;
        }
        let p = tactics_screen_calls::call2(tactics_screen_fns::GET_SLOT_PLAYER, ts, r as u8 as u32, 1) as usize;
        if p == 0 {
            continue;
        }
        if marking & 8 != 0 && exe::formation_get_marking_target(fm, slot as i8) == p {
            exe::database_format_staff_name(p, name.as_mut_ptr() as usize, 0xc8, 2);
            let f: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
            f(JOB_TEXT, 0x987ff0, name.as_ptr());
            let link = if exe::database_is_national_team(read_usize(ts + t::CLUB)) != 0 { 0x7dc } else { 0x7d7 };
            tel26_area_button([marking_area, 4, 0xd, 1, 0, 1, 2, c(JOB_COLOUR_BDEC), JOB_TEXT as u32, link, p as u32]);
        }
        tactics_dialog_number_and_name(ts, r as u8, JOB_TEXT as *mut u8, 0x7d0, 1, 1);
        tactics_dialog_popup_row(JOB_TEXT as u32, 6, p as u32, popup);
    }
    tactics_text(0xa8c500);
    let get_row: unsafe extern "thiscall" fn(usize, u32, u32) -> u8 = core::mem::transmute(tactics_screen_fns::GET_SLOT_MARKING_TARGET_ROW);
    let target = get_row(ts, row as u32, 0) as i8;
    if target > -1 && tactics_screen_calls::call2(tactics_screen_fns::GET_SLOT_PLAYER, ts, target as u8 as u32, 1) != 0 {
        tactics_dialog_number_and_name(ts, target as u8, name.as_mut_ptr(), 0xc8, 6, 1);
        tactics_text1(0xa8c4d4, name.as_ptr() as usize);
    }
    if marking & 0x10 != 0 {
        tel26_area_text([marking_area, 4, 0xd, 1, 0, 1, 2, c(JOB_COLOUR_BDEC), JOB_TEXT as u32]);
    }
    tactics_dialog_popup_row(JOB_TEXT as u32 + 2, 6, 0, popup);
}

/// The events of the player instructions dialog (parameter 0 the tactics screen's object it edits, 1 the row, 2 a
/// Tactics_Screen copy taken when it opened).
///
/// Another player (1) puts the item's value in parameter 1 and redraws. Cancel (2) copies the copy back over the
/// screen's object (`tactics_screen_assign`); Ok (3) clears the object's undo flag; both throw the copy away and close
/// (-9). On the row's position slot: a low-dword instruction (4) on an editable object saves the undo copy and sets
/// it (`Formation::set_player_instruction(slot, value, 0)`, the slot argument keeping the upper bytes of the item
/// as the original's reused stack slot does): refused, the undo copy goes back, else the team data is marked changed;
/// a high-dword one (7) through `Tactics_Screen::set_player_instruction(slot, 0, value)`; a marking type (5)
/// through `set_slot_marking_type`; a marked opponent (6): the player with type 8 (`set_slot_marking_type`, then
/// `set_slot_marking_target`), none with type 0x10 and target 0. These redraw (-0xb). Anything else goes to the menu
/// bar (`menubar_handle_object_link`).
/// Original: `tactics_player_instructions_dialog_handle_event` (0x8a6f80, tactics_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn tactics_player_instructions_dialog_handle_event(item: u32) -> i32 {
    use tactics_dialog_fns as f;
    use tactics_screen as t;
    let ts = get_screen_param(0);
    let row = get_screen_param(1) as u8;
    let copy = get_screen_param(2);
    let it = item as i16;
    match tel26_item_id(it) {
        1 => {
            covered!("tel15_6 player instructions: another player");
            set_screen_param(1, tel26_item_value(it) as usize);
            TACTICS_DIALOG_REDRAW
        }
        2 => {
            covered!("tel15_6 player instructions cancel");
            tactics_screen_assign(ts, copy);
            tactics_dialog_close(copy)
        }
        3 => {
            covered!("tel15_6 player instructions ok");
            write_u32(ts + t::CAN_UNDO, 0);
            tactics_dialog_close(copy)
        }
        4 => {
            let slot = tactics_dialog_row_slot(ts, row);
            let value = tel26_item_value(it);
            if read_u32(ts + t::READ_ONLY) != 0 {
                return TACTICS_DIALOG_REDRAW;
            }
            covered!("tel15_6 a player instruction");
            tactics_save_undo(ts);
            let slot_arg = (item & 0xffff_ff00) | slot as u32;
            if tactics_screen_calls::call3(tactics_screen_fns::FORMATION_SET_PLAYER_INSTRUCTION, ts + t::FORMATION, slot_arg, value, 0) == 0 {
                tactics_restore_undo(ts);
            } else {
                write_u32(ts + tactics_screen_state::TEAM_DATA_CHANGED, 1);
            }
            TACTICS_DIALOG_REDRAW
        }
        7 => {
            covered!("tel15_6 a high player instruction");
            let slot = tactics_dialog_row_slot(ts, row);
            tactics_screen_calls::call3(f::SET_PLAYER_INSTRUCTION, ts, slot as u32, 0, tel26_item_value(it));
            TACTICS_DIALOG_REDRAW
        }
        5 => {
            covered!("tel15_6 a marking type");
            let slot = tactics_dialog_row_slot(ts, row);
            tactics_screen_calls::call2(f::SET_SLOT_MARKING_TYPE, ts, slot as u32, tel26_item_value(it));
            TACTICS_DIALOG_REDRAW
        }
        6 => {
            covered!("tel15_6 a marked opponent");
            let slot = tactics_dialog_row_slot(ts, row) as u32;
            let target = tel26_item_value(it);
            if target != 0 {
                tactics_screen_calls::call2(f::SET_SLOT_MARKING_TYPE, ts, slot, 8);
                tactics_screen_calls::call2(f::SET_SLOT_MARKING_TARGET, ts, slot, target);
            } else {
                tactics_screen_calls::call2(f::SET_SLOT_MARKING_TYPE, ts, slot, 0x10);
                tactics_screen_calls::call2(f::SET_SLOT_MARKING_TARGET, ts, slot, 0);
            }
            TACTICS_DIALOG_REDRAW
        }
        _ => exe::menubar_handle_object_link(item),
    }
}

/// Comparer for `tactics_build_player_instructions_dialog`: side by side over the built display list
/// (`compare_match_screen_build`), the dialog's and the pitch's static colours.
pub(crate) unsafe extern "C" fn compare_tactics_build_player_instructions_dialog() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_TACTICS_BUILD_PLAYER_INSTRUCTIONS_DIALOG);
    let extra = [(tactics_dialog_statics::PLAYER_GUARD, 1), (tactics_dialog_statics::PLAYER_CHOICE, 2), (0xde1e20, 1), (0xde1f34, 2), (0xde1e24, 2)];
    compare_match_screen_build("tactics_build_player_instructions_dialog", &extra, || o(), || tactics_build_player_instructions_dialog());
}

/// Comparer for `tactics_player_instructions_dialog_handle_event`.
pub(crate) unsafe extern "C" fn compare_tactics_player_instructions_dialog_handle_event(item: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_TACTICS_PLAYER_INSTRUCTIONS_DIALOG_HANDLE_EVENT);
    let it = item as i16;
    let id = tel26_item_id(it);
    let (ts, copy) = (get_screen_param(0), get_screen_param(2));
    match id {
        4..=7 if ts != 0 => compare_tactics_method("player instructions event", &[tactics_screen_region(ts)], || o(item),
                                                   || tactics_player_instructions_dialog_handle_event(item)),
        1 => {
            let value = tel26_item_value(it) as usize;
            let r = o(item);
            SCOREBOARD.record(r == TACTICS_DIALOG_REDRAW && get_screen_param(1) == value, || format!("player instructions event 1: {} parameter 1 {:x}", r, get_screen_param(1)));
            r
        }
        2 | 3 if ts != 0 => {
            let n = tactics_screen_state::TEAM_DATA_SIZE;
            let kept = if id == 2 && copy != 0 { core::slice::from_raw_parts((copy + tactics_screen::FORMATION) as *const u8, n).to_vec() } else { Vec::new() };
            let r = o(item);
            let same = if id == 2 {
                copy == 0 || core::slice::from_raw_parts((ts + tactics_screen::FORMATION) as *const u8, n) == &kept[..]
            } else {
                read_u32(ts + tactics_screen::CAN_UNDO) == 0
            };
            SCOREBOARD.record(r == TACTICS_DIALOG_CLOSED && same, || format!("player instructions event {}: {} plan kept {}", id, r, same));
            r
        }
        _ => o(item),
    }
}

/// Builds the confirmation text for leaving the tactics screen back to the live match, and says whether it is needed.
///
/// The Text is cleared; outside a live match nothing more (0). The message, paragraph by paragraph: more than 3
/// substitutions made gives "You have made <number in words> substitutions." (line 0xa8c6a8); 1 to 3 give one
/// "<shirt> <player> has been brought on to replace <shirt> <player>." each (the shirt is the squad number when the
/// first team's club uses them, else the row + 1; both players linked). When the captain (the row whose player has
/// the `CAPTAIN` id) is not on the pitch, the player on the pitch with the highest captain score
/// (`match_man_get_captain_score`; the first on a tie) is named: "You have not selected a captain. <player> will be
/// selected as captain." (linked). The players of the match squad (the rows within the bench size of a match record)
/// neither on the pitch nor on the bench who are among the 10 set piece takers (the taker dword compared with the
/// player record's address, as the original does) are listed once each ("A", "A and B", "A, B, and C"; linked) in
/// "<list> have been specified as set piece takers, but are no longer on the pitch." (one: "... has been specified as
/// a set piece taker ..."). A non-empty message on a screen with `FIXTURE` set gets "Return to the match ?" and is
/// set into the Text: 1. Otherwise 0 (the links stay in the Text).
/// Original: `Tactics_Screen::build_return_to_match_text` (0x8a8630, tactics_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn build_return_to_match_text(ts: usize, text: usize) -> u32 {
    use tactics_screen as t;
    use tactics_subs_fields as f;
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    tactics_text_clear(text);
    if read_usize(ts + t::MATCH) == 0 {
        return 0;
    }
    let club = read_usize(ts + t::CLUB);
    let mut message = vec![0u8; 0x2700];
    let msg = message.as_mut_ptr();
    let (mut name, mut name2) = ([0u8; 0xc8], [0u8; 0xc8]);
    let made = read_i8(ts + t::SUBS_MADE);
    if made > 3 {
        covered!("tel15_7 return text: the substitutions count");
        exe::utils_number_to_words(made as i32, name.as_mut_ptr(), 0xc8, 1);
        sprintf(JOB_TEXT, 0xa8c6a8, name.as_ptr());
        tactics_append_paragraph(msg);
    } else if made > 0 {
        covered!("tel15_7 return text: each substitution");
        for k in 0..made as usize {
            let on = read_i8(ts + f::SUBS_ON + k);
            let mp_on = MatchPlayer(tactics_row_match_player(ts, on, false));
            let shirt_on = tactics_substitution_shirt(ts, on, mp_on.addr());
            exe::database_format_staff_name(mp_on.staff().addr(), name.as_mut_ptr() as usize, 0xc8, 1);
            tactics_player_link(text, club, name.as_ptr(), mp_on.staff().addr());
            let off = read_i8(ts + f::SUBS_OFF + k);
            let mp_off = MatchPlayer(tactics_row_match_player(ts, off, false));
            let shirt_off = tactics_substitution_shirt(ts, off, mp_off.addr());
            exe::database_format_staff_name(mp_off.staff().addr(), name2.as_mut_ptr() as usize, 0xc8, 1);
            tactics_player_link(text, club, name2.as_ptr(), mp_off.staff().addr());
            sprintf(JOB_TEXT, 0xa8c64c, shirt_on as i32, name.as_ptr(), shirt_off as i32, name2.as_ptr());
            tactics_append_paragraph(msg);
        }
    }
    let captain = (0..0x14i8).find(|&r| {
        let p = tactics_calls::slot_player(ts, r);
        p != 0 && read_u32(p) == read_u32(ts + t::CAPTAIN)
    }).unwrap_or(-1);
    if tactics_calls::slot_pitch_position(ts, captain, core::ptr::null_mut()) == 0 && read_usize(ts + t::MATCH) != 0 {
        let (mut best, mut top) = (-1i8, 0i16);
        for k in 0..0x14i8 {
            let mp = tactics_row_match_player(ts, k, false);
            if read_i8(mp + match_player::POSITION) >= 0 {
                let v = exe::match_man_get_captain_score(0usize, mp, club);
                if best < 0 || v > top {
                    best = k;
                    top = v;
                }
            }
        }
        if best > -1 {
            covered!("tel15_7 return text: the captain picked");
            let mp = tactics_row_match_player(ts, best, false);
            exe::database_format_staff_name(read_usize(mp + match_player::STAFF), name.as_mut_ptr() as usize, 0xc8, 1);
            tactics_player_link(text, club, name.as_ptr(), read_usize(mp + match_player::STAFF));
            sprintf(JOB_TEXT, 0xa16fec, name.as_ptr());
            tactics_append_paragraph(msg);
        }
    }
    let (mut takers, mut count) = ([0usize; 10], 0usize);
    for k in 0..0x14i8 {
        if read_u32(ts + t::HAS_MATCH_RECORD) != 0 && k >= ((read_u8(ts + t::BENCH_SIZE) & 0xf) + 0xb) as i8 {
            continue;
        }
        if tactics_calls::slot_pitch_position(ts, k, core::ptr::null_mut()) != 0 || tactics_calls::slot_on_bench(ts, k) != 0 {
            continue;
        }
        let p = tactics_calls::slot_player(ts, k);
        if p == 0 {
            continue;
        }
        for s in 0..10 {
            if read_u32(ts + tactics_screen_state::SET_PIECE_TAKERS + s * 4) == p as u32 && !takers[..count].contains(&p) {
                takers[count] = p;
                count += 1;
            }
        }
    }
    if count > 0 {
        covered!("tel15_7 return text: set piece takers off the pitch");
        let (mut list, mut joined) = (vec![0u8; 0x7d0], vec![0u8; 0x7d0]);
        for (k, &staff) in takers[..count].iter().enumerate() {
            exe::database_format_staff_name(staff, name.as_mut_ptr() as usize, 0xc8, 1);
            if k == 0 {
                tel26_copy_text(joined.as_mut_ptr(), name.as_ptr());
            } else {
                let format = if k != count - 1 { 0xa17af4 } else if count == 2 { 0xa17b40 } else { 0xa17b18 };
                sprintf(joined.as_mut_ptr() as usize, format, list.as_ptr(), name.as_ptr());
            }
            tel26_copy_text(list.as_mut_ptr(), joined.as_ptr());
            tactics_player_link(text, club, name.as_ptr(), staff);
        }
        sprintf(JOB_TEXT, if count > 1 { 0xa8c5ec } else { 0xa8c594 }, list.as_ptr());
        tactics_append_paragraph(msg);
    }
    if message[0] == 0 || read_u32(ts + t::FIXTURE) == 0 {
        return 0;
    }
    covered!("tel15_7 return text: return to the match");
    sprintf(JOB_TEXT, 0xa8c57c);
    tactics_append_paragraph(msg);
    exe::text_set_string(text, msg);
    1
}

/// Draws a football pitch in a rectangle of the current human's display (the display items of a pitch).
///
/// The first call sets the two static greens (`pitch_statics`). The line colour is rgb 0, 0x80, 0 without an area
/// (`area` -1 in its low word) and rgb 0, 0x50, 0 with one. The pitch: the rectangle in the dark green (style 0x30,
/// area -1; 0x70 in an area), and three stripes in the stripe green over the second, fourth and sixth of seven bands
/// (one pixel in from the sides). The touchlines sit `2 x scale` inside the rectangle (written to the four outputs:
/// left, top, right, bottom) and are drawn as an outline (0x220); then the halfway line through the centre, the centre
/// circle (radius 10 x scale), and at each end the penalty arc (from 2 to 22 x scale deep, 10 x scale each side of the
/// centre line) masked by a dark rectangle up to the penalty area (and the middle stripe drawn again over the mask),
/// the penalty spot (12 x scale), the penalty area (6 deep, 10 each side) and the goal area (18 deep, 22 each side),
/// all in pitch units of `scale`. `across` set lays the pitch with the goals left and right (stripes across, halfway
/// line upright), else with the goals top and bottom. The coordinates are computed on the x87 in double precision and
/// truncated like `_ftol`. Returns 1.
/// Original: `draw_pitch` (0x8ac600, tactics_screens.cpp, cdecl, 11 arguments: the float `scale` last).
pub(crate) unsafe extern "C" fn draw_pitch(left: i32, top: i32, right: i32, bottom: i32, out_left: *mut i32, out_top: *mut i32, out_right: *mut i32,
                                           out_bottom: *mut i32, across: u32, area: u32, scale: f32) -> u32 {
    use pitch_statics as s;
    covered!("tel15_7 draw a pitch");
    let rgb = |g: u32| exe::graphics_rgb_to_pixel(0, g, 0, 0);
    let guard = read_u8(s::GUARD);
    if guard & 1 == 0 {
        write_u8(s::GUARD, guard | 1);
        write_u16(s::GREEN_40, rgb(0x40) as u16);
    }
    let guard = read_u8(s::GUARD);
    if guard & 2 == 0 {
        write_u8(s::GUARD, guard | 2);
        write_u16(s::GREEN_38, rgb(0x38) as u16);
    }
    let no_area = area as u16 == 0xffff;
    let line = rgb(if no_area { 0x80 } else { 0x50 });
    let dark = || read_u16(s::GREEN_40) as u32;
    let stripe = || read_u16(s::GREEN_38) as u32;
    let (first_style, first_area) = if no_area { (0x30, u32::MAX) } else { (0x70, area) };
    pitch_item(left, top, right, bottom, first_style, dark(), 0, first_area);
    let scaled = |k: usize| scale as f64 * read_f64(k);
    let two = scale as f64 + scale as f64;
    if across != 0 {
        covered!("tel15_7 pitch with the goals left and right");
        let h = bottom - top;
        for k in [1, 3, 5] {
            pitch_item(left + 1, top + k * h / 7, right - 1, top + (k + 1) * h / 7, 0x10, stripe(), 0, area);
        }
        let (band_top, band_bottom) = (top + 3 * h / 7, top + 4 * h / 7);
        *out_left = pitch_ftol(left as f64 + two);
        *out_top = pitch_ftol(top as f64 + two);
        *out_right = pitch_ftol(right as f64 - two);
        let y1 = pitch_ftol(bottom as f64 - two);
        *out_bottom = y1;
        let (x0, y0, x1) = (*out_left, *out_top, *out_right);
        let cx = (x1 - x0 + 1) / 2 + *out_left;
        let cy = (y1 - y0 + 1) / 2 + y0;
        pitch_item(*out_left, y0, *out_right, y1, 0x220, line, 0, area);
        pitch_item(cx, *out_top, cx, *out_bottom, 0x400, line, 0, area);
        let r10 = scaled(rdata::F64_10);
        let cyf = cy as f64;
        let (upper, lower) = (pitch_ftol(cyf - r10), pitch_ftol(cyf + r10));
        pitch_item(pitch_ftol(cx as f64 - r10), upper, pitch_ftol(cx as f64 + r10), lower, 0x10020, line, line, area);
        let (r22, r18, r12, r6) = (scaled(PITCH_F64_22), scaled(rdata::F64_18), scaled(rdata::F64_12), scaled(rdata::F64_6));
        let one = read_f64(rdata::F64_1);
        let x0f = *out_left as f64;
        pitch_item(pitch_ftol(x0f + two), upper, pitch_ftol(x0f + r22), lower, 0x10020, line, line, area);
        pitch_item(pitch_ftol(x0f + two), upper, pitch_ftol(r18 + x0f - one), lower, 0x10, dark(), 0, area);
        let x0f = *out_left as f64;
        pitch_item(pitch_ftol(x0f + two), band_top, pitch_ftol(r18 + x0f - one), band_bottom, 0x10, stripe(), 0, area);
        let spot = pitch_ftol(*out_left as f64 + r12);
        pitch_item(spot, cy, spot, cy, 0x400, line, 0, area);
        let x0 = *out_left;
        pitch_item(x0, upper, pitch_ftol(x0 as f64 + r6), lower, 0x220, line, 0, area);
        let (goal_upper, goal_lower) = (pitch_ftol(cyf - r22), pitch_ftol(r22 + cyf));
        let x0 = *out_left;
        pitch_item(x0, goal_upper, pitch_ftol(x0 as f64 + r18), goal_lower, 0x220, line, 0, area);
        let x1f = *out_right as f64;
        pitch_item(pitch_ftol(x1f - r22), upper, pitch_ftol(x1f - two), lower, 0x10020, line, line, area);
        let x1f = *out_right as f64;
        pitch_item(pitch_ftol(x1f - r18 + one), upper, pitch_ftol(x1f - two), lower, 0x10, dark(), 0, area);
        let x1f = *out_right as f64;
        pitch_item(pitch_ftol(x1f - r18 + one), band_top, pitch_ftol(x1f - two), band_bottom, 0x10, stripe(), 0, area);
        let spot = pitch_ftol(*out_right as f64 - r12);
        pitch_item(spot, cy, spot, cy, 0x400, line, 0, area);
        let x1 = *out_right;
        pitch_item(pitch_ftol(x1 as f64 - r6), upper, x1, lower, 0x220, line, 0, area);
        let x1 = *out_right;
        pitch_item(pitch_ftol(x1 as f64 - r18), goal_upper, x1, goal_lower, 0x220, line, 0, area);
    } else {
        covered!("tel15_7 pitch with the goals top and bottom");
        let w = right - left;
        for k in [1, 3, 5] {
            pitch_item(left + k * w / 7, top + 1, left + (k + 1) * w / 7, bottom - 1, 0x10, stripe(), 0, area);
        }
        let (band_left, band_right) = (left + 3 * w / 7, left + 4 * w / 7);
        *out_left = pitch_ftol(left as f64 + two);
        *out_top = pitch_ftol(top as f64 + two);
        *out_right = pitch_ftol(right as f64 - two);
        let y1 = pitch_ftol(bottom as f64 - two);
        *out_bottom = y1;
        let (x0, x1, y0) = (*out_left, *out_right, *out_top);
        let cx = (x1 - x0 + 1) / 2 + x0;
        let cy = (y1 - y0 + 1) / 2 + y0;
        pitch_item(x0, y0, x1, y1, 0x220, line, 0, area);
        pitch_item(*out_left, cy, *out_right, cy, 0x400, line, 0, area);
        let r10 = scaled(rdata::F64_10);
        let cxf = cx as f64;
        let (side_left, side_right) = (pitch_ftol(cxf - r10), pitch_ftol(cxf + r10));
        pitch_item(side_left, pitch_ftol(cy as f64 - r10), side_right, pitch_ftol(cy as f64 + r10), 0x10020, line, line, area);
        let (r22, r18, r12, r6) = (scaled(PITCH_F64_22), scaled(rdata::F64_18), scaled(rdata::F64_12), scaled(rdata::F64_6));
        let one = read_f64(rdata::F64_1);
        let y0f = *out_top as f64;
        pitch_item(side_left, pitch_ftol(y0f + two), side_right, pitch_ftol(y0f + r22), 0x10020, line, line, area);
        let y0f = *out_top as f64;
        pitch_item(side_left, pitch_ftol(y0f + two), side_right, pitch_ftol(y0f + r18 - one), 0x10, dark(), 0, area);
        let y0f = *out_top as f64;
        pitch_item(band_left, pitch_ftol(y0f + two), band_right, pitch_ftol(y0f + r18 - one), 0x10, stripe(), 0, area);
        let spot = pitch_ftol(*out_top as f64 + r12);
        pitch_item(cx, spot, cx, spot, 0x400, line, 0, area);
        let y0 = *out_top;
        pitch_item(side_left, y0, side_right, pitch_ftol(y0 as f64 + r6), 0x220, line, 0, area);
        let y0 = *out_top;
        pitch_item(pitch_ftol(cxf - r22), y0, pitch_ftol(r22 + cxf), pitch_ftol(y0 as f64 + r18), 0x220, line, 0, area);
        let y1f = *out_bottom as f64;
        pitch_item(side_left, pitch_ftol(y1f - r22), side_right, pitch_ftol(y1f - two), 0x10020, line, line, area);
        let y1f = *out_bottom as f64;
        pitch_item(side_left, pitch_ftol(y1f - r18 + one), side_right, pitch_ftol(y1f - two), 0x10, dark(), 0, area);
        let y1f = *out_bottom as f64;
        pitch_item(band_left, pitch_ftol(y1f - r18 + one), band_right, pitch_ftol(y1f - two), 0x10, stripe(), 0, area);
        let spot = pitch_ftol(*out_bottom as f64 - r12);
        pitch_item(cx, spot, cx, spot, 0x400, line, 0, area);
        let y1 = *out_bottom;
        pitch_item(side_left, pitch_ftol(y1 as f64 - r6), side_right, y1, 0x220, line, 0, area);
        let y1 = *out_bottom;
        pitch_item(pitch_ftol(cxf - r22), pitch_ftol(y1 as f64 - r18), pitch_ftol(r22 + cxf), y1, 0x220, line, 0, area);
    }
    1
}

/// The point of a position on a pitch drawn in a rectangle, from its position flag (`Formation` position bits).
///
/// The goalkeeper (bit 1): the middle of the width, 10 above the bottom (no column needed). Otherwise the row from
/// bits 2..0x40 (bottom up: bottom - h/12 - 15, bottom - h/6 - 20, then 2/6, 3/6, 4/6 and 5/6 of the height up from
/// the bottom, the first one 10 higher) and the column from bits 0x80..0x800 (1/10, 3/10, 5/10, 7/10 and 9/10 of
/// the width with `wide`, else 1/8, 3/8, 1/2, 5/8 and 7/8), each division truncated like the compiler's. A flag
/// without a row shows the error box of line 0x21a8, without a column the box of line 0x21dd (after the row is
/// written): 0. Returns 1.
/// Original: `get_pitch_point_for_position_flags` (0x8ad780, tactics_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn get_pitch_point_for_position_flags(flags: u32, left: i32, top: i32, right: i32, bottom: i32, out_x: *mut i32, out_y: *mut i32,
                                                                   wide: u32) -> u32 {
    if flags & 1 != 0 {
        *out_x = (right - left) / 2 + left;
        *out_y = bottom - 10;
        return 1;
    }
    let (h, up) = (bottom - top, top - bottom);
    *out_y = if flags & 2 != 0 {
        bottom - h / 12 - 0xf
    } else if flags & 4 != 0 {
        bottom - h / 6 - 0x14
    } else if flags & 8 != 0 {
        2 * up / 6 + bottom - 10
    } else if flags & 0x10 != 0 {
        3 * up / 6 + bottom
    } else if flags & 0x20 != 0 {
        4 * up / 6 + bottom
    } else if flags & 0x40 != 0 {
        5 * up / 6 + bottom
    } else {
        tel26_error_box(TACTICS_SCREENS_CPP, 0x21a8);
        return 0;
    };
    let w = right - left;
    let wide = wide != 0;
    let x = if flags & 0x80 != 0 {
        if wide { w / 10 } else { w / 8 }
    } else if flags & 0x100 != 0 {
        if wide { 3 * w / 10 } else { 3 * w / 8 }
    } else if flags & 0x200 != 0 {
        5 * w / 10
    } else if flags & 0x400 != 0 {
        if wide { 7 * w / 10 } else { 5 * w / 8 }
    } else if flags & 0x800 != 0 {
        if wide { 9 * w / 10 } else { 7 * w / 8 }
    } else {
        tel26_error_box(TACTICS_SCREENS_CPP, 0x21dd);
        return 0;
    };
    *out_x = x + left;
    1
}

/// Comparer for `build_return_to_match_text`: the Text side by side (`compare_text_writer`: the strings and the
/// links) and the result.
pub(crate) unsafe extern "thiscall" fn compare_build_return_to_match_text(ts: usize, text: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_BUILD_RETURN_TO_MATCH_TEXT);
    let (ro, rr) = (core::cell::Cell::new(0u32), core::cell::Cell::new(0u32));
    compare_text_writer(text, || ro.set(o(ts, text)), |t| rr.set(build_return_to_match_text(ts, t)), || "tactics screen build_return_to_match_text".to_string());
    SCOREBOARD.record(ro.get() == rr.get(), || format!("tactics screen build_return_to_match_text: returned {}/{}", ro.get(), rr.get()));
    ro.get()
}

/// Comparer for `draw_pitch`: side by side over the built display list (`compare_match_screen_build`), the four
/// outputs and the static colours.
pub(crate) unsafe extern "C" fn compare_draw_pitch(left: i32, top: i32, right: i32, bottom: i32, out_left: *mut i32, out_top: *mut i32, out_right: *mut i32,
                                                   out_bottom: *mut i32, across: u32, area: u32, scale: f32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(i32, i32, i32, i32, *mut i32, *mut i32, *mut i32, *mut i32, u32, u32, f32) -> u32>(IDX_DRAW_PITCH);
    let r = core::cell::Cell::new(0u32);
    let extra = [(out_left as usize, 4), (out_top as usize, 4), (out_right as usize, 4), (out_bottom as usize, 4), (pitch_statics::GUARD, 1),
        (pitch_statics::GREEN_40, 2), (pitch_statics::GREEN_38, 2)];
    compare_match_screen_build("draw_pitch", &extra, || r.set(o(left, top, right, bottom, out_left, out_top, out_right, out_bottom, across, area, scale)),
                               || { draw_pitch(left, top, right, bottom, out_left, out_top, out_right, out_bottom, across, area, scale); });
    r.get()
}

/// Comparer for `get_pitch_point_for_position_flags`: side by side over the two outputs and the result.
pub(crate) unsafe extern "C" fn compare_get_pitch_point_for_position_flags(flags: u32, left: i32, top: i32, right: i32, bottom: i32, out_x: *mut i32,
                                                                           out_y: *mut i32, wide: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(u32, i32, i32, i32, i32, *mut i32, *mut i32, u32) -> u32>(IDX_GET_PITCH_POINT_FOR_POSITION_FLAGS);
    if flags & 1 == 0 && (flags & 0x7e == 0 || flags & 0xf80 == 0) {
        return o(flags, left, top, right, bottom, out_x, out_y, wide); // simplification: the box, original only
    }
    compare_tactics_method("get_pitch_point_for_position_flags", &[(out_x as usize, 4), (out_y as usize, 4)],
                           || o(flags, left, top, right, bottom, out_x, out_y, wide), || get_pitch_point_for_position_flags(flags, left, top, right, bottom, out_x, out_y, wide))
}

/// Rule: a tactics mask with a single bit among the low 48 gives its index (0..47); anything else -1.
///
/// `mask64_to_bit_index(low, high)`: the mask high:low compared against 1 << k for k = 0..47 in order.
/// Original: `mask64_to_bit_index` (0x8adbb0, tactics_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn mask64_to_bit_index(low: u32, high: u32) -> i8 {
    let mask = (high as u64) << 32 | low as u64;
    if mask.count_ones() == 1 && mask.trailing_zeros() < 0x30 {
        covered!("tel36_11 mask bit index");
        return mask.trailing_zeros() as i8;
    }
    -1
}

/// Rule: a tactics bit index gives its mask: 1 << k for 0..47, 0 otherwise.
///
/// `bit_index_to_mask64(k)`: the 64-bit mask in edx:eax for the char `k`.
/// Original: `bit_index_to_mask64` (0x8ade80, tactics_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn bit_index_to_mask64(k: u32) -> u64 {
    match k as u8 {
        b @ 0..=0x2f => 1u64 << b,
        _ => 0,
    }
}

/// Comparer for `mask64_to_bit_index`.
pub(crate) unsafe extern "C" fn compare_mask64_to_bit_index(low: u32, high: u32) -> i8 {
    let r = original_fn::<unsafe extern "C" fn(u32, u32) -> i8>(IDX_MASK64_TO_BIT_INDEX)(low, high);
    let n = mask64_to_bit_index(low, high);
    SCOREBOARD.record(r == n, || format!("mask64_to_bit_index({:#x}, {:#x}): original={} rust={}", low, high, r, n));
    r
}

/// Comparer for `bit_index_to_mask64`.
pub(crate) unsafe extern "C" fn compare_bit_index_to_mask64(k: u32) -> u64 {
    let r = original_fn::<unsafe extern "C" fn(u32) -> u64>(IDX_BIT_INDEX_TO_MASK64)(k);
    let n = bit_index_to_mask64(k);
    SCOREBOARD.record(r == n, || format!("bit_index_to_mask64({}): original={:#x} rust={:#x}", k as u8, r, n));
    r
}

/// Builds a Tactics_Screen in `p` (`Tactics_Screen::Tactics_Screen`, 0x8a7440).
pub(crate) unsafe fn tactics_screen_construct(p: usize, club: usize, m: usize, reserve: u32, arg4: u32, record: u32, penalty: u32) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32) -> usize = core::mem::transmute(TACTICS_CONSTRUCT_FN);
    f(p, club, m, reserve, arg4, record, penalty)
}

/// Destroys a Tactics_Screen the way `tactics_screen_open` does inline when the screen does not open: the match
/// players of the clipboard, undo and working team data (20 records of 0x1be bytes each), the two match team blocks,
/// and the memory.
pub(crate) unsafe fn tactics_screen_free_inline(ts: usize) {
    use tactics_screen as t;
    exe::crt_vector_destructor_iterator(ts + t::CLIPBOARD_DATA + t::TEAM_PLAYERS, 0x1be, 0x14, MATCH_PLAYER_DESTRUCT_FN);
    exe::crt_vector_destructor_iterator(ts + t::UNDO_DATA + t::TEAM_PLAYERS, 0x1be, 0x14, MATCH_PLAYER_DESTRUCT_FN);
    exe::crt_vector_destructor_iterator(ts + t::FORMATION + t::TEAM_PLAYERS, 0x1be, 0x14, MATCH_PLAYER_DESTRUCT_FN);
    exe::crt_vector_destructor_iterator(ts + t::MATCH_TEAMS, 0x625, 2, MATCH_TEAM_DESTRUCT_FN);
    exe::crt_operator_delete(ts);
}

/// `Display::add_area` on the current human's Display (the 11 fields); returns eax.
pub(crate) unsafe fn tactics_area(f: [u32; 11]) -> u32 {
    tel26_add_area(current_human_display(), f)
}

/// A colour word, zero-extended.
pub(crate) unsafe fn tactics_colour(a: usize) -> u32 {
    job_colour(a)
}

/// `langlib_sprintf(JOB_TEXT, fmt)` (the translated text into the screen text buffer).
pub(crate) unsafe fn tactics_text(fmt: usize) {
    let f: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    f(JOB_TEXT, fmt);
}

/// `langlib_sprintf(JOB_TEXT, fmt, a)`.
pub(crate) unsafe fn tactics_text1(fmt: usize, a: usize) {
    let f: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    f(JOB_TEXT, fmt, a);
}

/// Sets the static colours on the first build, each once (guard bits of `GUARD`, then bit 1 of `GUARD2`).
pub(crate) unsafe fn tactics_build_statics_init() {
    use tactics_build_statics as s;
    let rgb = |r: u32, g: u32, b: u32| exe::graphics_rgb_to_pixel(r, g, b, 0) as u16;
    let mut guard = read_u8(s::GUARD);
    if guard & 1 == 0 {
        write_u8(s::GUARD, guard | 1);
        write_u16(s::BENCH_NUMBER, rgb(0, 0, 0x40));
        guard = read_u8(s::GUARD);
    }
    if guard & 2 == 0 {
        write_u8(s::GUARD, guard | 2);
        write_u16(s::GREEN_90, rgb(0, 0x90, 0));
        guard = read_u8(s::GUARD);
    }
    if guard & 4 == 0 {
        guard |= 4;
        write_u8(s::GUARD, guard);
        write_u16(s::RED, read_u16(TACTICS_COLOUR_3154));
    }
    if guard & 8 == 0 {
        guard |= 8;
        write_u8(s::GUARD, guard);
        write_u16(s::RUN, read_u16(JOB_COLOUR_BDD4));
    }
    if guard & 0x10 == 0 {
        write_u8(s::GUARD, guard | 0x10);
        write_u16(s::GREEN_60, rgb(0, 0x60, 0));
        guard = read_u8(s::GUARD);
    }
    if guard & 0x20 == 0 {
        guard |= 0x20;
        write_u8(s::GUARD, guard);
        write_u16(s::ZONE, read_u16(JOB_COLOUR_BDEC));
    }
    if guard & 0x40 == 0 {
        guard |= 0x40;
        write_u8(s::GUARD, guard);
        write_u16(s::HOVER_ZONE, read_u16(JOB_COLOUR_317E));
    }
    if guard & 0x80 == 0 {
        write_u8(s::GUARD, guard | 0x80);
        write_u16(s::GREEN_80, rgb(0, 0x80, 0));
    }
    let guard2 = read_u8(s::GUARD2);
    if guard2 & 1 == 0 {
        write_u8(s::GUARD2, guard2 | 1);
        write_u16(s::GREEN_40, rgb(0, 0x40, 0));
    }
}

/// The menus of an editable tactics screen: Tactics (Team Instructions, Penalty Takers, Set To Default with the
/// presets, Load, Load Preset, Save, Save As, Save For, Delete, Lock or Unlock and the recent files), View (the
/// column views), Last Match and Edit (Undo, Copy, Copy All, Paste, Reflect, Show or Hide Runs).
pub(crate) unsafe fn tactics_build_menus(ts: usize, view: u8, heading: &mut [u8; 0xc8]) {
    use tactics_screen as t;
    let c = tactics_colour;
    // the Tactics menu
    let bar = tactics_area([0x6e, 0x50, 0x168, 0x64, 2, 0, 1, 0, 1, 0, u32::MAX]);
    tactics_text(0x98e840);
    let button = tel26_area_button([bar, 0, 0, 0x30, c(JOB_COLOUR_317E), 0xc, 1, c(JOB_COLOUR_3184), JOB_TEXT as u32, 0, 0]);
    let menu = tactics_popup(0x130, button);
    tactics_text(0xa8bfa4);
    tactics_menu_row(2, 1, 1, 0, menu);
    tactics_text(0xa8bf94);
    tactics_menu_row(2, 1, 0x22, 0, menu);
    tactics_text(0xa8bf84);
    let default_row = tactics_menu_row(2, 1, 0, 0, menu);
    let presets = tactics_popup(0x30, default_row);
    let mut k: u8 = 0;
    while (k as i8 as i32) < read_i32(read_usize(G_TACTICS) + 0x5fd) {
        let idx = k as i8 as i16 as u16 as u32;
        let found = TACTICS_PRESET_NAMES.iter().find(|&&name| exe::crt_stricmp(exe::tactics_get_tactic_block(read_usize(G_TACTICS), idx, 1), name) == 0);
        match found {
            Some(&name) => tactics_text(name),
            None => tel26_copy_text(JOB_TEXT as *mut u8, exe::tactics_get_tactic_block(read_usize(G_TACTICS), idx, 1) as *const u8),
        }
        tactics_menu_row(2, 1, 2, k as i8 as i32 as u32, presets);
        k = k.wrapping_add(1);
    }
    tactics_menu_separator(menu);
    tactics_text(0xa7e7f4);
    tactics_menu_row(2, 1, 3, 0, menu);
    if read_u8(TACTICS_PRESETS_ON) != 0 {
        tactics_text(0xa8bf78);
        tactics_menu_row(2, 1, 4, 0, menu);
    }
    tactics_text(0xa7e7dc);
    if read_u32(ts + t::SAVED) == 0 && read_u32(ts + t::HAS_FILE) != 0 {
        tactics_menu_row(2, 1, 5, 0, menu);
    } else {
        tactics_menu_row(2, 0x21, 0, 0, menu);
    }
    tactics_text(0xa7e7c4);
    tactics_menu_row(2, 1, 6, 0, menu);
    if read_u8(TACTICS_PRESETS_ON) != 0 {
        tactics_text(0xa8bf78);
        tactics_menu_row(2, 1, 4, 0, menu);
    }
    // Save For: one row per human manager (the others), greyed without a password
    if exe_count_human_managers(0) as i32 > 1 {
        tactics_text(0xa7e7a4);
        let save_for = tactics_menu_row(2, 1, 0, 0, menu);
        let sub = tactics_popup(0x30, save_for);
        for n in 0..0x10 {
            let staff = staff_at(read_i32(G_STAFF_COUNT) as isize - n - 1);
            if staff.classification() != 1 || staff == tel26_slot_manager() {
                continue;
            }
            exe::database_format_staff_name(staff, JOB_TEXT, 0x7d0, 1);
            if read_u8(tactics_calls::password(staff.addr())) != 0 {
                tactics_menu_row(2, 1, 8, staff.addr() as u32, sub);
            } else {
                tactics_menu_row(2, 0x21, 0, 0, sub);
            }
        }
    }
    tactics_text(0xa69fb4);
    tactics_menu_row(2, 1, 9, 0, menu);
    if read_u8(ts + t::LOCKED) != 0 {
        tactics_text(0xa7e778);
        tactics_menu_row(2, 1, 0xb, 0, menu);
    } else if read_u8(tactics_calls::password(0)) != 0 {
        tactics_text(0xa7e770);
        tactics_menu_row(2, 1, 0xb, 0, menu);
    }
    // the recent files (4), after a separator
    let mut separated = false;
    let mut file = [0u8; 0x104];
    for n in 0..4u32 {
        exe::human_manager_get_recent_file(read_usize(G_HUMAN_MANAGER), n, file.as_mut_ptr(), 0x104, 1, 0, 1);
        if file[0] == 0 {
            continue;
        }
        if !separated {
            tactics_menu_separator(menu);
            separated = true;
        }
        let f: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
        f(JOB_TEXT, 0xa7e728, n + 1, file.as_ptr());
        tactics_menu_row(2, 1, 0xc, n, menu);
    }
    // the View menu
    tactics_text(0x988004);
    let button = tel26_area_button([bar, 1, 0, 0x30, c(JOB_COLOUR_317E), 0xc, 1, c(JOB_COLOUR_3184), JOB_TEXT as u32, 0, 0]);
    let menu = tactics_popup(0x130, button);
    tactics_text(0x98edf4);
    tactics_view_row(view, 1, menu, None);
    tactics_text(0x98edc8);
    tactics_view_row(view, 2, menu, None);
    if read_u32(ts + t::MATCH) == 0 {
        tactics_text(0x98ed4c);
        tactics_view_row(view, 3, menu, None);
        tactics_text(0x98eddc);
        tactics_view_row(view, 4, menu, None);
        tactics_text(0xa8bf68);
        tactics_view_row(view, 5, menu, None);
    } else {
        tactics_text(0xa8bf58);
        tactics_view_row(view, 6, menu, None);
    }
    tactics_menu_separator(menu);
    for &(k, text) in TACTICS_VIEW_TEXTS.iter() {
        tactics_text(text);
        tactics_view_row(view, k, menu, Some(&mut *heading));
    }
    tactics_text1(0xa8be78, TACTICS_EMPTY);
    tactics_view_row(view, 0x14, menu, Some(&mut *heading));
    if read_u32(ts + t::MATCH) != 0 {
        tactics_menu_separator(menu);
        tactics_text(0xa8be70);
        tactics_view_row(view, 0x15, menu, Some(&mut *heading));
    }
    // Last Match and the Edit menu
    let bar = tactics_area([0x212, 0x50, 0x30c, 0x64, 2, 0, 1, 0, 1, 0, u32::MAX]);
    tactics_text(0x98e828);
    tel26_area_button([bar, 0, 0, 0x30, c(JOB_COLOUR_317E), 0xc, 1, c(JOB_COLOUR_3184), JOB_TEXT as u32, 0xe, 0]);
    tactics_text(0xa8be68);
    let button = tel26_area_button([bar, 1, 0, 0x30, c(JOB_COLOUR_317E), 0xc, 1, c(JOB_COLOUR_3184), JOB_TEXT as u32, 0, 0]);
    let menu = tactics_popup(0x130, button);
    let edit_row = |enabled: bool, id: u32| if enabled { tactics_menu_row(2, 1, id, 0, menu) } else { tactics_menu_row(2, 0x21, 0, 0, menu) };
    tactics_text(0xa8be60);
    edit_row(read_u32(ts + t::CAN_UNDO) != 0, 0xf);
    tactics_text(0xa8be58);
    edit_row(read_u8(ts + t::VIEW) != 1, 0x10);
    tactics_text(0xa8be4c);
    edit_row(read_u8(ts + t::VIEW) != 1, 0x11);
    tactics_text(0xa8be44);
    edit_row(read_u8(ts + t::VIEW) != 1 && read_u8(ts + t::CAN_PASTE) != 0, 0x12);
    tactics_text(0xa8be3c);
    edit_row(read_u8(ts + t::VIEW) != 1, 0x13);
    tactics_text(if read_u32(ts + t::SHOW_RUNS) != 0 { 0xa8be24 } else { 0xa8be30 });
    edit_row(read_u8(ts + t::VIEW) != 1, 0x14);
}

/// The formation name (with the unsaved mark), LOCKED and the Overview / With Ball / Without Ball tabs; then the
/// column widths of the player list for the view.
pub(crate) unsafe fn tactics_build_header(ts: usize, view: u8, widths: &mut [u8; 9]) {
    use tactics_screen as t;
    let c = tactics_colour;
    tel26_copy_text(JOB_TEXT as *mut u8, (ts + t::FORMATION) as *const u8);
    if read_u32(ts + t::SAVED) == 0 {
        let mut n = 0;
        while read_u8(JOB_TEXT + n) != 0 {
            n += 1;
        }
        tel26_copy_text((JOB_TEXT + n) as *mut u8, 0x989ad8 as *const u8);
    }
    exe::screenmanager_add_text_item(G_SCREEN_MANAGER, 0x6e, 0x69, 0x1b8, 0x8c, 2, 0, 0xc, 3, c(JOB_COLOUR_BDEC), (JOB_TEXT) as usize as u32, 1);
    if read_u8(ts + t::LOCKED) != 0 {
        tactics_text(0xa8be18);
        exe::screenmanager_add_text_item(G_SCREEN_MANAGER, 0x154, 0x69, 0x1b8, 0x7d, 1, 0, 0x4c, 1, c(JOB_COLOUR_BDEC), (JOB_TEXT) as usize as u32, 1);
    }
    let tabs = tactics_area([0x1c2, 0x69, 0x30c, 0x8c, 3, 0, 1, 0, 1, 0, u32::MAX]);
    // the font of the current tab: an immediate the 1080 exe patches (1 -> 3), read from the running exe
    tactics_text(0xa8be0c);
    if read_u8(ts + t::VIEW) == 1 {
        tel26_area_text([tabs, 0, 0, 0x830, c(TACTICS_COLOUR_BDD6), 0xc, core::ptr::read(0x89c626 as *const u8) as u32, c(JOB_COLOUR_3184), JOB_TEXT as u32]);
    } else {
        tel26_area_button([tabs, 0, 0, 0x30, c(TACTICS_COLOUR_BDD6), 0xc, 1, c(JOB_COLOUR_3184), JOB_TEXT as u32, 0x15, 1]);
    }
    let tab = |col: u32, view_k: u8, text: usize, current_font: u32| {
        tactics_text(text);
        if read_u8(ts + t::VIEW) == view_k {
            tel26_area_text([tabs, col, 0, 0x830, c(TACTICS_COLOUR_BDD6), 0xc, current_font, c(JOB_COLOUR_3184), JOB_TEXT as u32]);
        } else if read_u32(ts + t::READ_ONLY) == 0 {
            tel26_area_button([tabs, col, 0, 0x30, c(TACTICS_COLOUR_BDD6), 0xc, 1, c(JOB_COLOUR_3184), JOB_TEXT as u32, 0x15, view_k as u32]);
        } else {
            tel26_area_text([tabs, col, 0, 0x30, c(TACTICS_COLOUR_BDD6), 0x2c, 1, c(JOB_COLOUR_3184), JOB_TEXT as u32]);
        }
    };
    tab(1, 2, 0xa8be00, core::ptr::read(0x89c69e as *const u8) as u32);
    tab(2, 3, 0xa8bdf0, core::ptr::read(0x89c73c as *const u8) as u32);
    // the column widths: the extra columns merge into the one the view shows
    match view {
        1 | 3 | 4 | 5 => {
            widths[6] = widths[6].wrapping_add(widths[7]).wrapping_add(widths[8]);
            widths[7] = 0;
            widths[8] = 0;
        }
        2 => {
            widths[7] = widths[7].wrapping_add(widths[8]);
            widths[8] = 0;
        }
        6 => {}
        _ => {
            widths[5] = widths[5].wrapping_add(widths[6]).wrapping_add(widths[7]).wrapping_add(widths[8]);
            widths[6] = 0;
            widths[7] = 0;
            widths[8] = 0;
        }
    }
}

/// Row `k` of the player list (number, name, marks and the view's column), in the areas `list` (the 9 columns) and
/// `handles` (the drag column); `line` is the display row.
pub(crate) unsafe fn tactics_build_row(ts: usize, view: u8, k: i8, list: u32, handles: u32, line: u32) {
    use tactics_screen as t;
    use tactics_calls as x;
    let c = tactics_colour;
    let read_only = read_u32(ts + t::READ_ONLY) != 0;
    let player = x::slot_player(ts, k);
    if !read_only {
        tactics_item([0x102, 0, 0, 0, 0, 1, line, 1, 0, 0, 0xc, 1, 0, 0, 0, 0x16, k as i32 as u32, handles]);
    }
    // the penalty order or the captain mark (condition and match rating views)
    if view == 2 || view == 6 {
        if read_u32(ts + t::PENALTY_MODE) != 0 {
            let order = (0..0xbi8).find(|&n| read_i8(ts + t::PENALTY_ORDER + n as usize) == k);
            if let Some(n) = order {
                tactics_text1(0xa8bddc, (n as i32 + 1) as usize);
                let colour = if n < 5 { JOB_COLOUR_BDEC } else { JOB_COLOUR_317E };
                let flags = if read_only { 1 } else { 0x42 };
                tactics_item([flags, 0, 0, 0, 0, 5, line, 1, 0, 0, 0xc, 1, c(colour), JOB_TEXT as u32, 0, 0x18, n as i32 as u32, list]);
            }
        } else {
            let captain_row = || (0..0x14i8).find(|&r| {
                let p = x::slot_player(ts, r);
                p != 0 && read_u32(p) == read_u32(ts + t::CAPTAIN)
            });
            let mark = match captain_row() {
                Some(r) if r == k => true,
                _ => captain_row().is_none() && k == 0,
            };
            if mark {
                covered!("tel14_2 captain mark");
                tactics_text(0xa8bdd4);
                if read_only {
                    tactics_item([1, 0, 0, 0, 0, 5, line, 1, 0, 0, 0xc, 1, c(JOB_COLOUR_BDEC), JOB_TEXT as u32, 0, 0x17, 0, list]);
                } else {
                    let colour = if captain_row().is_some() { JOB_COLOUR_BDEC } else { TACTICS_COLOUR_3154 };
                    tactics_item([0x42, 0, 0, 0, 0, 5, line, 1, 0, 0, 0xc, 1, c(colour), JOB_TEXT as u32, 0, 0x17, 0, list]);
                }
            }
        }
    }
    // the number: clickable on the pitch or for a substitute who can still come on
    let number_colour = if k < 0xb { c(JOB_COLOUR_BE04) } else { c(tactics_build_statics::BENCH_NUMBER) };
    x::slot_number_text(ts, k);
    let outside_squad = read_u32(ts + t::HAS_MATCH_RECORD) != 0 && k >= ((read_u8(ts + t::BENCH_SIZE) & 0xf) as i8).wrapping_add(0xb);
    if outside_squad {
        tel26_area_text([list, 0, line, 0x30, number_colour, 0x2c, 1, c(JOB_COLOUR_3184), JOB_TEXT as u32]);
    } else {
        let mut clickable = x::slot_pitch_position(ts, k, core::ptr::null_mut()) != 0;
        if !clickable && x::slot_on_bench(ts, k) != 0 {
            let m = read_usize(ts + t::MATCH);
            if m != 0 && read_u32(ts + t::FIXTURE) != 0 {
                let side = read_i8(ts + t::SIDE) as isize;
                let left = read_u8((m as isize + side * 0xd0 + 0x9049) as usize).wrapping_sub(read_u8(ts + t::SUBS_MADE)) as i8;
                clickable = left > 0;
            }
        }
        if clickable {
            let flags = if read_only { 1 } else { 0x42 };
            tactics_item([flags, 0, 0, 0, 0, 0, line, 0x30, number_colour, number_colour, 0xc, 1, c(JOB_COLOUR_3184), JOB_TEXT as u32, 0,
                0x19, k as i32 as u32, list]);
        } else {
            tel26_area_text([list, 0, line, 0x30, number_colour, 0xc, 1, c(JOB_COLOUR_3184), JOB_TEXT as u32]);
        }
    }
    if !read_only {
        tactics_item([0x302, 0, 0, 0, 0, 0, line, 1, 0, 0, 0xc, 1, 0, 0, 0, 0x16, k as i32 as u32, list]);
    }
    // the cards
    let card = |colour: usize, style: u32| tel26_area_text([list, 2, line, style, c(colour), 0xc, 1, c(JOB_COLOUR_BDEA), 0]);
    if x::slot_byte(0x8aabb0, ts, k) != 0 {
        if x::slot_byte(0x8aacb0, ts, k) != 0 {
            card(JOB_COLOUR_BDEC, 0x20030);
            card(TACTICS_COLOUR_31A8, 0x40030);
        } else {
            card(JOB_COLOUR_BDEC, 0x20030);
        }
    } else if x::slot_byte(0x8aacb0, ts, k) != 0 {
        card(TACTICS_COLOUR_31A8, 0x20030);
    }
    tactics_build_row_column(ts, view, k, player, list, line, number_colour);
    // the name
    x::slot_name_text(ts, k, 1);
    if read_u8(JOB_TEXT) == 0 {
        return;
    }
    if player != 0 {
        let colour = if x::slot_motm(ts, k) != 0 {
            JOB_COLOUR_BDE8
        } else if x::slot_pitch_position(ts, k, core::ptr::null_mut()) != 0 {
            JOB_COLOUR_3184
        } else {
            JOB_COLOUR_317E
        };
        let id = if exe::database_is_national_team(read_usize(ts + t::CLUB)) != 0 { 0x7dc } else { 0x7d7 };
        tactics_item([0x202, 0, 0, 0, 0, 4, line, 1, 0, 0, 1, 2, c(colour), JOB_TEXT as u32, 0, id, player as u32, list]);
    } else {
        let colour = if x::slot_motm(ts, k) != 0 {
            JOB_COLOUR_BDE8
        } else if x::slot_pitch_position(ts, k, core::ptr::null_mut()) != 0 {
            JOB_COLOUR_3184
        } else {
            JOB_COLOUR_317E
        };
        tactics_item([1, 0, 0, 0, 0, 4, line, 1, 0, 0, 1, 2, c(colour), JOB_TEXT as u32, 0, 0, 0, list]);
    }
}

/// The overview: the run arrows, then each slot's circle at its defending position, with the name under it.
pub(crate) unsafe fn tactics_build_overview(ts: usize, pitch: &TacticsPitch) {
    use tactics_screen as t;
    use tactics_calls as x;
    let flags = if read_u32(ts + t::READ_ONLY) != 0 { 1 } else { 6 };
    let r = pitch.r;
    tactics_item([flags, r[0] as u32, r[1] as u32, r[2] as u32, r[3] as u32, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0x1c, 0, u32::MAX]);
    tactics_build_overview_runs(ts, pitch);
    let fm = ts + t::FORMATION;
    for slot in 0..0xbi8 {
        let ord = slot as u32;
        let mut pending: u32 = 0;
        let row = x::find_slot_at_position(ts, slot, &mut pending);
        if row <= -1 {
            continue;
        }
        let mut later: u32 = 0;
        if x::duplicate_position(fm, slot, &mut later) == 0 {
            continue;
        }
        let position = exe::formation_get_slot_position(fm, slot as u8 as u32, 0);
        let players = if read_u32(ts + t::MATCH) != 0 { ts + t::MATCH_PLAYERS } else { 0 };
        let wide = x::central_in_line(fm, position, players);
        let (mut cx, mut cy) = (0i32, 0i32);
        x::pitch_point(position, &pitch.r, &mut cx, &mut cy, wide);
        let (back, line, name_colour) = if pending == 0 {
            let keeper = position & 1 != 0;
            (if keeper { pitch.keeper_back } else { pitch.field_back }, if keeper { pitch.keeper_line } else { pitch.field_line },
                tactics_colour(JOB_COLOUR_BDEC))
        } else {
            let g = tactics_colour(tactics_build_statics::GREEN_80);
            (g, tactics_colour(tactics_build_statics::GREEN_40), g)
        };
        x::slot_number_text(ts, row);
        tactics_slot_circle(ts, cx, cy, ord, back & 0xffff, line & 0xffff, later, true);
        if x::slot_customised(ts, slot) != 0 {
            tactics_slot_customised_mark(cx, cy - 0x13, cy + 9, back & 0xffff);
        }
        x::slot_name_text(ts, row, 5);
        tactics_item([1, (cx - 0x2d) as u32, (cy + 6) as u32, (cx + 0x2d) as u32, (cy + 0x10) as u32, 0, 0, 1, 0, 0, 0xc, 1, name_colour,
            JOB_TEXT as u32, 0, 0, 0, u32::MAX]);
    }
}

/// The with / without the ball views: the 12 zones (clickable when editable, id 0x1f, value y + 100 x), the runs
/// to the zone shown, the zone under the mouse and the zone shown, then the circles at the coordinates of the zone.
pub(crate) unsafe fn tactics_build_zones(ts: usize, pitch: &TacticsPitch) {
    use tactics_screen as t;
    use tactics_calls as x;
    let r = pitch.r;
    let read_only = read_u32(ts + t::READ_ONLY) != 0;
    let green = tactics_colour(tactics_build_statics::GREEN_60);
    for zx in 0..3 {
        for zy in 0..4 {
            let q = tactics_zone_rect(&r, zx, zy, 0);
            let flags = if read_only { 1 } else { 0x306 };
            tactics_item([flags, q[0], q[1], q[2], q[3], 0, 0, 0x2220, green, green, 0, 0, 0, 0, 0, 0x1f, (zy + 100 * zx) as u32, u32::MAX]);
        }
    }
    let zone = read_u32(ts + t::ZONE);
    let fm = ts + t::FORMATION;
    if read_u32(ts + t::SHOW_RUNS) != 0 {
        for slot in 0..0xbi8 {
            let mut pending: u32 = 0;
            if x::find_slot_at_position(ts, slot, &mut pending) <= -1 {
                continue;
            }
            let mut later: u32 = 0;
            if x::duplicate_coordinate(fm, slot, (read_u8(ts + t::VIEW) == 2) as u8, read_u32(ts + t::ZONE), &mut later) == 0 {
                continue;
            }
            let mut c = [0i16; 2];
            x::slot_coordinate(ts, slot, &mut c);
            let (ax1, ay1, ax2, ay2) = tactics_coordinate_square(&r, c);
            exe::formation_get_slot_coordinate(fm, slot, (read_u8(ts + t::VIEW) == 2) as i8, read_u32(ts + t::ZONE), c.as_mut_ptr() as *mut u32);
            let (bx1, by1, bx2, by2) = tactics_coordinate_square(&r, c);
            let colour = if pending == 0 { read_u32(tactics_build_statics::RUN) } else { tactics_colour(tactics_build_statics::GREEN_80) };
            tactics_item([1, ((ax2 - ax1) / 2 + ax1) as u32, ((ay1 - ay2) / 2 + ay2) as u32, ((bx2 - bx1) / 2 + bx1) as u32,
                ((by1 - by2) / 2 + by2) as u32, 0, 0, 0x2400, colour & 0xffff, 0, 0, 0, 0, 0, 0, 0, 0, u32::MAX]);
        }
        let hover = read_u32(ts + t::HOVER_ZONE);
        if hover as u16 != zone as u16 || (hover >> 16) as u16 != (zone >> 16) as u16 {
            let q = tactics_zone_rect(&r, hover as u16 as i16 as i32, (hover >> 16) as u16 as i16 as i32, 1);
            let colour = tactics_colour(tactics_build_statics::HOVER_ZONE);
            tactics_item([1, q[0], q[1], q[2], q[3], 0, 0, 0x2220, colour, colour, 0, 0, 0, 0, 0, 0, 0, u32::MAX]);
        }
    }
    let q = tactics_zone_rect(&r, zone as u16 as i16 as i32, (zone >> 16) as u16 as i16 as i32, 1);
    let colour = tactics_colour(tactics_build_statics::ZONE);
    tactics_item([1, q[0], q[1], q[2], q[3], 0, 0, 0x2220, colour, colour, 0, 0, 0, 0, 0, 0, 0, u32::MAX]);
    let flags = if read_only { 1 } else { 6 };
    tactics_item([flags, r[0] as u32, r[1] as u32, r[2] as u32, r[3] as u32, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0x1c, 0, u32::MAX]);
    for slot in 0..0xbi8 {
        let ord = slot as u32;
        let mut pending: u32 = 0;
        let row = x::find_slot_at_position(ts, slot, &mut pending);
        if row <= -1 {
            continue;
        }
        let mut later: u32 = 0;
        if x::duplicate_coordinate(fm, slot, (read_u8(ts + t::VIEW) == 2) as u8, read_u32(ts + t::ZONE), &mut later) == 0 {
            continue;
        }
        let mut c = [0i16; 2];
        exe::formation_get_slot_coordinate(fm, slot, (read_u8(ts + t::VIEW) == 2) as i8, read_u32(ts + t::ZONE), c.as_mut_ptr() as *mut u32);
        let (x1, y1, x2, y2) = tactics_coordinate_square(&r, c);
        let position = exe::formation_get_slot_position(fm, slot as u8 as u32, 0);
        let (back, line) = if pending == 0 {
            let keeper = position & 1 != 0;
            (if keeper { pitch.keeper_back } else { pitch.field_back }, if keeper { pitch.keeper_line } else { pitch.field_line })
        } else {
            (tactics_colour(tactics_build_statics::GREEN_80), tactics_colour(tactics_build_statics::GREEN_40))
        };
        x::slot_number_text(ts, row);
        let (cx, cy) = ((x2 - x1) / 2 + x1, (y1 - y2) / 2 + y2);
        tactics_slot_circle(ts, cx, cy, ord, back & 0xffff, line & 0xffff, later, false);
        if x::slot_customised(ts, slot) != 0 {
            tactics_slot_customised_mark(cx, cy - 0xe, cy + 0xe, back & 0xffff);
        }
    }
}

/// The answer of a pending dialog of the screen (`G_PENDING_COMMAND_CODE` and `_ARG`, 2 = yes), each code cleared
/// when seen: 0x34 set to default, 0x2c load (the file dialog), 0x2d the file to load (refused when another
/// manager's password locks it), 0x36 the preset to load, 0x2e the file to save, 0x35 the preset to save, 0x2f the
/// file to delete (the file name is freed after each), 0x30 a recent file, 0x31 Cancel (frees the object: closed),
/// 0x32 Ok (the captain picked when the current one is not on the pitch, the changes applied, the object freed:
/// closed), 0x33 save before leaving (the file dialog). A failure is a confirm box. None when the event goes on to
/// the item dispatch.
pub(crate) unsafe fn tactics_event_dialog_answer(ts: usize, c: (u32, u32)) -> Option<i32> {
    use tactics_event_texts as x;
    use tactics_screen as t;
    let s = tactics_sprintf();
    let code = || read_u32(G_PENDING_COMMAND_CODE);
    let arg = || read_u32(G_PENDING_COMMAND_ARG);
    let (mut password, mut creator) = ([0u8; 0x34], [0u8; 200]);
    if code() == 0x34 {
        write_u32(G_PENDING_COMMAND_CODE, 0);
        if arg() == 2 {
            write_u32(G_PENDING_COMMAND_ARG, 0);
            tactics_set_preset(ts, get_screen_param(9));
        }
        write_u32(G_PENDING_COMMAND_ARG, 0);
    }
    if code() == 0x2c {
        write_u32(G_PENDING_COMMAND_CODE, 0);
        if arg() == 2 {
            covered!("tel14_3 answer: load");
            write_u32(G_PENDING_COMMAND_ARG, 0);
            let folder = tactics_folder();
            tactics_file_dialog(c, x::LOAD_FORMATION, x::SELECT_FILE, 0, folder, TACTICS_EXT_TCT, 0, TACTICS_ON_FILE, 0x2d);
            return Some(TACTICS_EVENT_DONE);
        }
        write_u32(G_PENDING_COMMAND_ARG, 0);
    }
    if code() == 0x2d {
        write_u32(G_PENDING_COMMAND_CODE, 0);
        let file = arg() as usize;
        if file != 0 {
            covered!("tel14_3 answer: the file to load");
            write_u32(G_PENDING_COMMAND_ARG, 0);
            if tactics_locked_by_other(file, &mut password, &mut creator) {
                let text = s(JOB_TEXT, x::LOCKED_BY, file, creator.as_ptr(), 0, c.0, c.1, 0);
                tactics_confirm(c, x::LOAD_FORMATION, x::UNABLE_TO_LOAD, text, 0, 0);
                exe::crt_operator_delete(file);
                return Some(TACTICS_EVENT_DONE);
            }
            if tactics_load(ts, file, 0) == 0 {
                let text = s(JOB_TEXT, x::NOT_LOADED, file, 0, c.0, c.1, 0);
                tactics_confirm(c, x::LOAD_FORMATION, x::UNABLE_TO_LOAD, text, 0, 0);
                exe::crt_operator_delete(file);
                return Some(TACTICS_EVENT_DONE);
            }
            exe::crt_operator_delete(file);
        }
    }
    if code() == 0x36 {
        write_u32(G_PENDING_COMMAND_CODE, 0);
        let file = arg() as usize;
        if file != 0 {
            covered!("tel14_3 answer: the preset to load");
            write_u32(G_PENDING_COMMAND_ARG, 0);
            if tactics_load(ts, file, 1) == 0 {
                let text = s(JOB_TEXT, x::PRESET_NOT_LOADED, file, 0, c.0, c.1, 0);
                tactics_confirm(c, x::LOAD_PRESET_FORMATION, x::UNABLE_TO_LOAD, text, 0, 0);
                exe::crt_operator_delete(file);
                return Some(TACTICS_EVENT_DONE);
            }
            exe::crt_operator_delete(file);
        }
    }
    if code() == 0x2e {
        write_u32(G_PENDING_COMMAND_CODE, 0);
        let file = arg() as usize;
        if file != 0 {
            covered!("tel14_3 answer: the file to save");
            write_u32(G_PENDING_COMMAND_ARG, 0);
            if tactics_locked_by_other(file, &mut password, &mut creator) {
                let text = s(JOB_TEXT, x::LOCKED_BY_SAVE, file, creator.as_ptr(), 0, c.0, c.1, 0);
                tactics_confirm(c, x::SAVE_FORMATION, x::UNABLE_TO_SAVE, text, 0, 0);
                exe::crt_operator_delete(file);
                return Some(TACTICS_EVENT_DONE);
            }
            if tactics_save(ts, file, get_screen_param(5)) == 0 {
                let text = s(JOB_TEXT, x::NOT_SAVED_FILE, file, 0, c.0, c.1, 0);
                tactics_confirm(c, x::SAVE_FORMATION, x::UNABLE_TO_SAVE, text, 0, 0);
                exe::crt_operator_delete(file);
                return Some(TACTICS_EVENT_DONE);
            }
            exe::crt_operator_delete(file);
        }
    }
    if code() == 0x35 {
        write_u32(G_PENDING_COMMAND_CODE, 0);
        let file = arg() as usize;
        if file != 0 {
            covered!("tel14_3 answer: the preset to save");
            write_u32(G_PENDING_COMMAND_ARG, 0);
            // Formation::save_to_disk(path, silent, obfuscated)
            if tactics_calls::thiscall3(0x5b6900, ts + t::FORMATION, file as u32, 1, 1) == 0 {
                let text = s(JOB_TEXT, x::NOT_SAVED_AS_PRESET, file, 0, c.0, c.1, 0);
                tactics_confirm(c, x::SAVE_AS_PRESET, x::UNABLE_TO_SAVE, text, 0, 0);
                exe::crt_operator_delete(file);
                return Some(TACTICS_EVENT_DONE);
            }
            exe::crt_operator_delete(file);
        }
    }
    if code() == 0x2f {
        write_u32(G_PENDING_COMMAND_CODE, 0);
        let file = arg() as usize;
        if file != 0 {
            covered!("tel14_3 answer: the file to delete");
            write_u32(G_PENDING_COMMAND_ARG, 0);
            if tactics_locked_by_other(file, &mut password, &mut creator) {
                let text = s(JOB_TEXT, x::LOCKED_BY, file, creator.as_ptr(), 0, c.0, c.1, 0);
                tactics_confirm(c, x::DELETE_FORMATION, x::UNABLE_TO_DELETE, text, 0, 0);
                exe::crt_operator_delete(file);
                return Some(TACTICS_EVENT_DONE);
            }
            if exe::crt_remove(file as *const u8) != 0 {
                let text = s(JOB_TEXT, x::NOT_DELETED, file, 0, c.0, c.1, 0);
                tactics_confirm(c, x::DELETE_FORMATION, x::UNABLE_TO_DELETE, text, 0, 0);
                exe::crt_operator_delete(file);
                return Some(TACTICS_EVENT_DONE);
            }
            exe::crt_operator_delete(file);
        }
    }
    if code() == 0x30 {
        write_u32(G_PENDING_COMMAND_CODE, 0);
        if arg() == 2 {
            covered!("tel14_3 answer: a recent file");
            write_u32(G_PENDING_COMMAND_ARG, 0);
            let mut path = [0u8; 0x104];
            if let Some(r) = tactics_load_recent(ts, c, &mut path, &mut password, &mut creator) {
                return Some(r);
            }
        }
        write_u32(G_PENDING_COMMAND_ARG, 0);
    }
    if code() == 0x31 {
        write_u32(G_PENDING_COMMAND_CODE, 0);
        if arg() == 2 {
            covered!("tel14_3 answer: cancel");
            write_u32(G_PENDING_COMMAND_ARG, 0);
            if ts != 0 {
                tactics_event_free(ts);
            }
            return Some(TACTICS_EVENT_CLOSED);
        }
        write_u32(G_PENDING_COMMAND_ARG, 0);
    }
    if code() == 0x32 {
        write_u32(G_PENDING_COMMAND_CODE, 0);
        if arg() == 2 {
            covered!("tel14_3 answer: ok");
            write_u32(G_PENDING_COMMAND_ARG, 0);
            let row = tactics_captain_row(ts);
            if tactics_calls::slot_pitch_position(ts, row, core::ptr::null_mut()) == 0 {
                display_method(0x8a84a0, ts); // Tactics_Screen::auto_select_captain
            }
            display_method(0x8a80d0, ts); // Tactics_Screen::apply_changes
            if ts != 0 {
                tactics_event_free(ts);
            }
            return Some(TACTICS_EVENT_CLOSED);
        }
        write_u32(G_PENDING_COMMAND_ARG, 0);
    }
    if code() == 0x33 {
        let yes = arg() == 2;
        write_u32(G_PENDING_COMMAND_CODE, 0);
        write_u32(G_PENDING_COMMAND_ARG, 0);
        if yes {
            covered!("tel14_3 answer: save before leaving");
            let name = if read_u32(ts + t::HAS_FILE) != 0 { ts + t::FORMATION } else { TACTICS_NO_TEXT };
            let folder = tactics_folder();
            tactics_file_dialog(c, x::SAVE_FORMATION, x::ENTER_FILE_NAME, 1, folder, TACTICS_EXT_TCT, name, TACTICS_ON_FILE, 0x2e);
            return Some(TACTICS_EVENT_DONE);
        }
    }
    None
}

/// The Tactics menu and its neighbours (ids 1 to 0xd and 0x22): 1 Team Instructions (a copy of the object on the
/// Team Instructions screen, the read-only builder without the original), 0x22 Penalty Takers (the club squad
/// screen), 2 Set To Default, 3 Load, 4 Load Preset, 5 Save, 6 Save As, 7 Save As Preset, 8 Save For, 9 Delete,
/// 0xa Unlock, 0xb Lock, 0xc a recent file and 0xd a View column. An unsaved formation asks first (2, 3, 0xc);
/// Lock and Unlock ask to save. None for the other ids.
pub(crate) unsafe fn tactics_event_menu(ts: usize, id: u16, item: i16, c: (u32, u32)) -> Option<i32> {
    use tactics_event_fields as f;
    use tactics_event_texts as x;
    use tactics_screen as t;
    let s = tactics_sprintf();
    let value = || tel26_item_value(item);
    match id {
        1 => {
            covered!("tel14_3 team instructions");
            let copy = tactics_screen_copy(ts);
            let mut owner = ts;
            if read_u32(copy + t::READ_ONLY) == 0 {
                if exe::screenmanager_open_screen(G_SCREEN_MANAGER, TACTICS_TEAM_INSTRUCTIONS_FN, TACTICS_TEAM_INSTRUCTIONS_EVENT_FN, 1, 0usize, 0) == 0 {
                    return Some(TACTICS_EVENT_DONE);
                }
            } else {
                if exe::screenmanager_open_screen(G_SCREEN_MANAGER, TACTICS_TEAM_INSTRUCTIONS_RO_FN, TACTICS_TEAM_INSTRUCTIONS_EVENT_FN, 1, 0usize, 0) == 0 {
                    return Some(TACTICS_EVENT_DONE);
                }
                owner = 0;
            }
            set_screen_param(0, copy);
            set_screen_param(1, owner);
            Some(TACTICS_EVENT_DONE)
        }
        0x22 => {
            covered!("tel14_3 penalty takers");
            let club = get_screen_param(1);
            let reserve = get_screen_param(3);
            let f: unsafe extern "C" fn(usize, usize, u32, u32, u32) = core::mem::transmute(FN_OPEN_CLUB_SQUAD_SCREEN);
            f(club, reserve, 0, 0, 1);
            Some(TACTICS_EVENT_DONE)
        }
        2 => {
            covered!("tel14_3 set to default");
            set_screen_param(9, value() as usize);
            if read_u32(ts + t::SAVED) != 0 {
                tactics_set_preset(ts, get_screen_param(9));
                return Some(TACTICS_EVENT_REDRAW);
            }
            let text = s(JOB_TEXT, x::ASK_DEFAULT, ts + t::FORMATION, 1, c.0, c.1, 0x34);
            tactics_confirm(c, x::SET_TO_DEFAULT, x::NOT_SAVED, text, 1, 0x34);
            Some(TACTICS_EVENT_DONE)
        }
        3 => {
            covered!("tel14_3 load");
            if read_u32(ts + t::SAVED) != 0 {
                let folder = tactics_folder();
                tactics_file_dialog(c, x::LOAD_FORMATION, x::SELECT_FILE, 0, folder, TACTICS_EXT_TCT, 0, TACTICS_ON_FILE, 0x2d);
                return Some(TACTICS_EVENT_DONE);
            }
            let text = s(JOB_TEXT, x::ASK_LOAD, ts + t::FORMATION, 1, c.0, c.1, 0x2c);
            tactics_confirm(c, x::LOAD_FORMATION, x::NOT_SAVED, text, 1, 0x2c);
            Some(TACTICS_EVENT_DONE)
        }
        4 => {
            covered!("tel14_3 load preset");
            let folder = tactics_preset_folder(TACTICS_PRESET_LOAD_PATH);
            tactics_file_dialog(c, x::LOAD_PRESET_FORMATION, x::SELECT_FILE, 0, folder, TACTICS_EXT_PCT, TACTICS_NO_TEXT, TACTICS_ON_PRESET, 0x36);
            Some(TACTICS_EVENT_DONE)
        }
        5 => {
            covered!("tel14_3 save");
            let mut path = [0u8; 0x104];
            let (mut password, mut creator) = ([0u8; 0x34], [0u8; 200]);
            let folder = tactics_folder();
            exe::crt_makepath(path.as_mut_ptr(), TACTICS_NO_TEXT, folder, ts + t::FORMATION, TACTICS_EXT_TCT);
            let p = path.as_ptr() as usize;
            let text = if tactics_locked_by_other(p, &mut password, &mut creator) {
                s(JOB_TEXT, x::LOCKED_BY_SAVE, p, creator.as_ptr(), 0, c.0, c.1, 0)
            } else {
                if tactics_save(ts, p, get_screen_param(5)) != 0 {
                    return Some(TACTICS_EVENT_REDRAW);
                }
                s(JOB_TEXT, x::NOT_SAVED_FILE, p, 0, c.0, c.1, 0)
            };
            tactics_confirm(c, x::SAVE_FORMATION, x::UNABLE_TO_SAVE, text, 0, 0);
            Some(TACTICS_EVENT_DONE)
        }
        6 | 8 => {
            covered!("tel14_3 save as / save for");
            set_screen_param(5, if id == 6 { 0 } else { value() as usize });
            let name = if read_u32(ts + t::HAS_FILE) != 0 { ts + t::FORMATION } else { TACTICS_NO_TEXT };
            let folder = tactics_folder();
            tactics_file_dialog(c, x::SAVE_FORMATION, x::ENTER_FILE_NAME, 1, folder, TACTICS_EXT_TCT, name, TACTICS_ON_FILE, 0x2e);
            Some(TACTICS_EVENT_DONE)
        }
        7 => {
            covered!("tel14_3 save as preset");
            set_screen_param(5, 0);
            let folder = tactics_preset_folder(TACTICS_PRESET_SAVE_PATH);
            tactics_file_dialog(c, x::SAVE_AS_PRESET, x::ENTER_FILE_NAME, 1, folder, TACTICS_EXT_PCT, TACTICS_NO_TEXT, TACTICS_ON_PRESET, 0x35);
            Some(TACTICS_EVENT_DONE)
        }
        9 => {
            covered!("tel14_3 delete");
            let folder = tactics_folder();
            tactics_file_dialog(c, x::DELETE_FORMATION, x::SELECT_FILE, 2, folder, TACTICS_EXT_TCT, 0, TACTICS_ON_FILE, 0x2f);
            Some(TACTICS_EVENT_DONE)
        }
        0xa => {
            covered!("tel14_3 unlock");
            if read_u32(ts + t::READ_ONLY) == 0 && read_u8(ts + t::LOCKED) != 0 {
                tactics_event_save_undo(ts);
                write_u8(ts + t::LOCKED, 0);
                write_u32(ts + f::CHANGES_MADE, 1);
            }
            if read_u32(ts + f::LOCK_SAVED) == 0 {
                return Some(TACTICS_EVENT_REDRAW);
            }
            let text = s(JOB_TEXT, x::ASK_SAVE_UNLOCK, 1, c.0, c.1, 0x33);
            tactics_confirm(c, x::UNLOCK_FORMATION, x::NOT_SAVED, text, 1, 0x33);
            Some(TACTICS_EVENT_DONE)
        }
        0xb => {
            covered!("tel14_3 lock");
            if read_u32(ts + t::READ_ONLY) == 0 && read_u8(ts + t::LOCKED) == 0 && read_u8(tactics_calls::password(0)) != 0 {
                tactics_event_save_undo(ts);
                tel26_copy_text((ts + t::LOCKED) as *mut u8, tactics_calls::password(0) as *const u8);
                write_u32(ts + f::CHANGES_MADE, 1);
            }
            if read_u32(ts + f::LOCK_SAVED) != 0 {
                return Some(TACTICS_EVENT_REDRAW);
            }
            let text = s(JOB_TEXT, x::ASK_SAVE_LOCK, 1, c.0, c.1, 0x33);
            tactics_confirm(c, x::LOCK_FORMATION, x::NOT_SAVED, text, 1, 0x33);
            Some(TACTICS_EVENT_DONE)
        }
        0xc => {
            covered!("tel14_3 recent file");
            set_screen_param(6, value() as usize);
            if read_u32(ts + t::SAVED) != 0 {
                let mut path = [0u8; 0x104];
                let (mut password, mut creator) = ([0u8; 0x34], [0u8; 200]);
                return Some(tactics_load_recent(ts, c, &mut path, &mut password, &mut creator).unwrap_or(TACTICS_EVENT_REDRAW));
            }
            let text = s(JOB_TEXT, x::ASK_LOAD, ts + t::FORMATION, 1, c.0, c.1, 0x30);
            tactics_confirm(c, x::LOAD_FORMATION, x::NOT_SAVED, text, 1, 0x30);
            Some(TACTICS_EVENT_DONE)
        }
        0xd => {
            covered!("tel14_3 view column");
            set_screen_param(10, value() as usize);
            Some(TACTICS_EVENT_REDRAW)
        }
        _ => None,
    }
}

/// Last Match (id 0xe): the club's fixtures of the last 8 weeks (24 for a national team) from the fixture manager;
/// the latest played one (an entry whose byte +0x43 is not negative) opens its tactics read-only
/// (`tactics_screen_open` with the match record) when the match statistics still have it; otherwise a box: no
/// tactics for the last match, or no recent matches (the reserve and 'B' squad texts with the club name as title).
pub(crate) unsafe fn tactics_event_last_match(c: (u32, u32)) -> i32 {
    use tactics_event_texts as x;
    let s = tactics_sprintf();
    covered!("tel14_3 last match");
    let mut list = [0u8; 0x20];
    let l = list.as_mut_ptr() as usize;
    exe::new_llist(l);
    let club = get_screen_param(1);
    let reserve = get_screen_param(3);
    let weeks = if exe::database_is_national_team(club) != 0 { 0x18 } else { 8 };
    let today_year = read_u16(G_CURRENT_DATE_YEAR) as u32;
    let (mut d1, mut d2) = ([0u8; 8], [0u8; 8]);
    let year = read_u16(exe::date_sub_weeks(G_CURRENT_DATE, &mut d1, weeks) + 2) as u32;
    let day = read_u16(exe::date_sub_weeks(G_CURRENT_DATE, &mut d2, weeks)) as u32;
    // ponytail: the callee reads only the low words of the four date arguments (the original leaves stack and
    // register leftovers in the high halves)
    exe::fixture_manager_find_fixtures(read_usize(G_FIXTURE_MANAGER), day, year, read_u16(G_CURRENT_DATE) as u32, today_year, read_u32(club), l,
        u32::MAX, 0xffff_fffe, (reserve == 0) as u32);
    let mut entry = 0usize;
    if read_u16(l) != 0 {
        write_u32(l + 0xa, read_u32(l + 6));
        loop {
            let e = exe::llist_get_prev(l);
            if e == 0 {
                break;
            }
            if read_i8(e + 0x43) >= 0 {
                entry = e;
                break;
            }
        }
    }
    let (mut hb, mut tb) = ([0u8; 200], [0u8; 200]);
    let h = hb.as_mut_ptr() as usize;
    let tbuf = tb.as_mut_ptr() as usize;
    let (heading, text_fmt, title_fmt) = if entry != 0 {
        let found = exe::match_stats_fixture_entry_matches(read_usize(G_MATCH_STATS), read_i32(entry + 8), read_i32(entry + 0xc), read_i32(entry + 0x10),
            read_u16(entry + 0x2a) as i32, read_u16(entry + 0x28) as i32);
        if found != 0 {
            tactics_screen_open(club, reserve as u32, 0, 0, read_u32(entry + 8), 0);
            exe::drop_llist(l);
            return TACTICS_EVENT_DONE;
        }
        if reserve == 0 {
            (x::NO_TACTICS, x::NO_TACTICS_MATCH, 0)
        } else if exe::database_is_national_team(club) != 0 {
            (x::NO_TACTICS, x::NO_TACTICS_B, x::CLUB_B_SQUAD)
        } else {
            (x::NO_TACTICS, x::NO_TACTICS_RESERVE, x::CLUB_RESERVES)
        }
    } else if reserve == 0 {
        (x::NO_RECENT, x::NO_RECENT_MATCHES, 0)
    } else if exe::database_is_national_team(club) != 0 {
        (x::NO_RECENT, x::NO_RECENT_B, x::CLUB_B_SQUAD)
    } else {
        (x::NO_RECENT, x::NO_RECENT_RESERVE, x::CLUB_RESERVES)
    };
    let text = s(JOB_TEXT, text_fmt, 0, c.0, c.1, 0);
    let hd = s(h, heading, text, 0, c.0, c.1, 0);
    let title = if title_fmt == 0 {
        club + 4
    } else {
        let g = read_i8(club + 0x52) as i32;
        s(tbuf, title_fmt, g, g, club + 0x38, hd, text, 0, c.0, c.1, 0)
    };
    exe::confirm_push(title, hd, text, 0, c.0, c.1, 0);
    exe::drop_llist(l);
    TACTICS_EVENT_DONE
}

/// The Edit menu and the tabs (ids 0xf to 0x15): 0xf Undo, 0x10 Copy (the zone shown), 0x11 Copy All (the phase),
/// 0x12 Paste, 0x13 Reflect (the zone to the other flank, then shown), 0x14 Show or Hide Runs and 0x15 a tab
/// (Overview / With Ball / Without Ball; another value is the error box of line 0x147a). Copy and Reflect do nothing
/// in the overview. Redraw. None for the other ids.
pub(crate) unsafe fn tactics_event_edit(ts: usize, id: u16, item: i16) -> Option<i32> {
    use tactics_event_fields as f;
    use tactics_screen as t;
    let fm = ts + t::FORMATION;
    let view = read_u8(ts + t::VIEW);
    match id {
        0xf => {
            covered!("tel14_3 undo");
            if read_u32(ts + t::CAN_UNDO) != 0 {
                tactics_event_restore_undo(ts);
                write_u32(ts + t::CAN_UNDO, 0);
            }
        }
        0x10 => {
            covered!("tel14_3 copy");
            if view != 1 {
                // Formation::get_zone_coordinates(phase, zone, out)
                tactics_calls::thiscall3(0x5b3f90, fm, (view == 2) as u32, read_u32(ts + t::ZONE), (ts + f::COPIED_ZONE) as u32);
                write_u8(ts + t::CAN_PASTE, 1);
            }
        }
        0x11 => {
            covered!("tel14_3 copy all");
            if view != 1 {
                // Formation::get_phase_coordinates(phase, out)
                tactics_calls::thiscall2(0x5b42a0, fm, (view == 2) as u32, (ts + f::COPIED_PHASE) as u32);
                write_u8(ts + t::CAN_PASTE, 2);
            }
        }
        0x12 => {
            covered!("tel14_3 paste");
            display_method(0x8a9cb0, ts); // Tactics_Screen::paste_positions
        }
        0x13 => {
            covered!("tel14_3 reflect");
            if read_u32(ts + t::READ_ONLY) == 0 && view != 1 {
                tactics_event_save_undo(ts);
                // Formation::mirror_zone_coordinates(phase, zone)
                tactics_calls::thiscall2(0x5b47a0, fm, (view == 2) as u32, read_u32(ts + t::ZONE));
                write_u32(ts + f::CHANGES_MADE, 1);
                write_i16(ts + t::ZONE, 2i16.wrapping_sub(read_i16(ts + t::ZONE)));
            }
        }
        0x14 => {
            covered!("tel14_3 show runs");
            write_u32(ts + t::SHOW_RUNS, (read_u32(ts + t::SHOW_RUNS) == 0) as u32);
        }
        0x15 => {
            covered!("tel14_3 tab");
            let tab = tel26_item_value(item) as u8;
            if item != -1 && (tab == 1 || tab == 2 || tab == 3) {
                write_u8(ts + f::PREVIOUS_VIEW, view);
                write_u8(ts + t::VIEW, tab);
                write_u32(ts + t::HOVER_ZONE, read_u32(ts + t::ZONE));
            } else {
                tel26_error_box(TACTICS_SCREENS_CPP, 0x147a);
            }
        }
        _ => return None,
    }
    Some(TACTICS_EVENT_REDRAW)
}

/// The player list (ids 0x16 to 0x1b): 0x16 a row number clicked (a row on the pitch opens the player instructions
/// screen with a copy of the object), 0x17 a row dropped on the captain mark (the new captain), 0x18 a row dropped
/// on a penalty place (set the taker; another place dropped there swaps the two), 0x19 two rows swapped, 0x1a a
/// player instruction (bit index and slot in the value) and 0x1b a marking type. Read-only screens change nothing;
/// a change saves the undo copy and marks the changes. None for the other ids and the drops that do not apply.
pub(crate) unsafe fn tactics_event_list(ts: usize, id: u16, item: i16) -> Option<i32> {
    use tactics_event_fields as f;
    use tactics_screen as t;
    let fm = ts + t::FORMATION;
    let value = tel26_item_value(item);
    let partner = tactics_partner(item);
    let partner_id = if partner != -1 { tel26_item_id(partner) } else { 0 };
    match id {
        0x16 => {
            let row = value as u8 as i8;
            if tactics_calls::slot_pitch_position(ts, row, core::ptr::null_mut()) != 0 {
                covered!("tel14_3 player instructions");
                let copy = tactics_screen_copy(ts);
                if exe::screenmanager_open_screen(G_SCREEN_MANAGER, TACTICS_PLAYER_INSTRUCTIONS_FN, TACTICS_PLAYER_INSTRUCTIONS_EVENT_FN, 1, 0usize, 0) != 0 {
                    set_screen_param(0, ts);
                    set_screen_param(1, row as i32 as usize);
                    set_screen_param(2, copy);
                }
                return Some(TACTICS_EVENT_DONE);
            }
            None
        }
        0x17 if partner != -1 && partner_id == 0x16 => {
            let dragged = tel26_item_value(partner) as u8 as i8;
            if dragged == tactics_captain_row(ts) {
                return None;
            }
            covered!("tel14_3 new captain");
            if tactics_calls::slot_player(ts, dragged) == 0 {
                return Some(0);
            }
            let p = tactics_calls::slot_player(ts, dragged);
            if p == 0 {
                tel26_error_box(TACTICS_SCREENS_CPP, 0x14b5);
            } else if read_u32(ts + t::READ_ONLY) == 0 && read_u32(p) != read_u32(ts + t::CAPTAIN) {
                tactics_event_save_undo(ts);
                write_u32(ts + t::CAPTAIN, read_u32(p));
                write_u32(ts + f::CHANGES_MADE, 1);
            }
            Some(TACTICS_EVENT_REDRAW)
        }
        0x18 if partner != -1 && (partner_id == 0x16 || partner_id == 0x18) => {
            let a = value as u8 as i8;
            let b = tel26_item_value(partner) as u8 as i8;
            if partner_id == 0x16 {
                let taker = if (0..=10).contains(&a) {
                    read_i8(ts + t::PENALTY_ORDER + a as usize)
                } else {
                    tel26_error_box(TACTICS_SCREENS_CPP, 0x16b9);
                    -1
                };
                if b != taker && tactics_calls::slot_pitch_position(ts, b, core::ptr::null_mut()) != 0 {
                    covered!("tel14_3 penalty taker");
                    // Tactics_Screen::set_penalty_taker(order, row)
                    tactics_calls::thiscall2(0x8a9120, ts, a as u8 as u32, b as u8 as u32);
                    return Some(TACTICS_EVENT_REDRAW);
                }
                None
            } else if a != b {
                covered!("tel14_3 swap penalty takers");
                // Tactics_Screen::swap_penalty_takers(a, b)
                tactics_calls::thiscall2(0x8a9270, ts, a as u8 as u32, b as u8 as u32);
                Some(TACTICS_EVENT_REDRAW)
            } else {
                None
            }
        }
        0x19 if partner != -1 && partner_id == 0x19 => {
            let other = tel26_item_value(partner);
            if value as u8 == other as u8 {
                return None;
            }
            covered!("tel14_3 swap rows");
            // Tactics_Screen::swap_slots(row a, row b, opponent)
            tactics_calls::thiscall3(0x8ab7e0, ts, value, other, 0);
            Some(TACTICS_EVENT_REDRAW)
        }
        0x1a | 0x1b => {
            let slot = ((value as i32 >> 6) as u8) & 0x1f;
            let m = exe::bit_index_to_mask64(value & 0xffff_ff3f);
            let (lo, hi) = (m as u32, (m >> 32) as u32);
            if id == 0x1a {
                covered!("tel14_3 player instruction");
                if read_u32(ts + t::READ_ONLY) == 0 {
                    tactics_event_save_undo(ts);
                    // Formation::set_player_instruction(slot, flag)
                    if tactics_calls::thiscall3(0x5b5290, fm, slot as u32, lo, hi) == 0 {
                        tactics_event_restore_undo(ts);
                        write_u32(ts + t::CAN_UNDO, 0);
                    } else {
                        write_u32(ts + f::CHANGES_MADE, 1);
                    }
                }
            } else if slot <= 10 {
                covered!("tel14_3 marking");
                if read_u32(ts + t::READ_ONLY) == 0 {
                    tactics_event_save_undo(ts);
                    // Formation::set_marking_type(slot, flag)
                    tactics_calls::thiscall2(0x5b57d0, fm, slot as u32, lo);
                    write_u32(ts + f::CHANGES_MADE, 1);
                }
            } else {
                tel26_error_box(TACTICS_SCREENS_CPP, 0x1a25);
            }
            Some(TACTICS_EVENT_REDRAW)
        }
        _ => None,
    }
}

/// The pitch (ids 0x1d to 0x1f): 0x1d a circle dropped on the pitch: in the overview at the nearest position (the
/// two slots swapped when another slot has it, else the slot moved there), in the zone views on the cell of the
/// 27 x 36 grid it overlaps most (the coordinates swapped with the slot there, else the slot's coordinate set);
/// 0x1e a circle's drop target: the slot's extra position set to the nearest; 0x1f a zone clicked (x = value / 100,
/// y = value % 100; out of range is the error box of line 0x15fa). None for the other ids and the drops that do
/// not apply.
pub(crate) unsafe fn tactics_event_pitch(ts: usize, id: u16, item: i16) -> Option<i32> {
    use tactics_event_fields as f;
    use tactics_screen as t;
    let fm = ts + t::FORMATION;
    let partner = tactics_partner(item);
    match id {
        0x1d | 0x1e if partner != -1 && tel26_item_id(partner) == 0x1c => {
            let slot = tel26_item_value(item) as u8 as i8;
            let d = tactics_item_rect(item, tactics_event_item::DROP_RECT);
            let p = tactics_item_rect(partner, tactics_event_item::RECT);
            if id == 0x1e {
                let (best, pos) = tactics_nearest_position(ts, &p, &d);
                if best >= 800 {
                    return None;
                }
                covered!("tel14_3 extra position");
                if !(0..=10).contains(&slot) {
                    tel26_error_box(TACTICS_SCREENS_CPP, 0x199c);
                } else if read_u32(ts + t::READ_ONLY) == 0 {
                    tactics_event_save_undo(ts);
                    // Formation::set_slot_position(slot, position, extra)
                    tactics_calls::thiscall3(0x5b2ba0, fm, slot as u8 as u32, pos as u32, 1);
                    write_u32(ts + f::CHANGES_MADE, 1);
                }
                return Some(TACTICS_EVENT_REDRAW);
            }
            if read_u8(ts + t::VIEW) == 1 {
                let (best, pos) = tactics_nearest_position(ts, &p, &d);
                if best >= 800 {
                    return None;
                }
                let there = tactics_calls::slot_by_position(fm, pos);
                if there > -1 && there != slot {
                    covered!("tel14_3 swap slots on the pitch");
                    if !((0..=10).contains(&slot) && (0..=10).contains(&there)) {
                        tel26_error_box(TACTICS_SCREENS_CPP, 0x1977);
                    } else if read_u32(ts + t::READ_ONLY) == 0 {
                        tactics_event_save_undo(ts);
                        // Formation::swap_slots(a, b)
                        tactics_calls::thiscall2(0x5b2f30, fm, slot as u8 as u32, there as u8 as u32);
                        write_u32(ts + f::CHANGES_MADE, 1);
                    }
                    return Some(TACTICS_EVENT_REDRAW);
                }
                if there != -1 {
                    return None;
                }
                covered!("tel14_3 move a slot on the pitch");
                if !(0..=10).contains(&slot) {
                    tel26_error_box(TACTICS_SCREENS_CPP, 0x199c);
                } else if read_u32(ts + t::READ_ONLY) == 0 {
                    tactics_event_save_undo(ts);
                    tactics_calls::thiscall3(0x5b2ba0, fm, slot as u8 as u32, pos as u32, 0);
                    write_u32(ts + f::CHANGES_MADE, 1);
                }
                return Some(TACTICS_EVENT_REDRAW);
            }
            let (w, h) = (p[2] - p[0], p[3] - p[1]);
            let (mut best, mut cell) = (0i32, 0u32);
            let mut acc_i = 0i32;
            for i in 0..0x1bu32 {
                let x2 = p[0] + (w + acc_i) / 0x1b;
                let x1 = p[0] + acc_i / 0x1b;
                let mut acc_j = 0i32;
                for j in 0..0x24u32 {
                    let area = exe::rect_overlap_area(d[0], d[1], d[2], d[3], x1, p[3] - (h + acc_j) / 0x24, x2, p[3] - acc_j / 0x24);
                    if area > best {
                        cell = i | (j << 16);
                        best = area;
                    }
                    acc_j += h;
                }
                acc_i += w;
            }
            if best == 0 {
                return None;
            }
            let view = read_u8(ts + t::VIEW);
            // Formation::find_slot_at_coordinate(phase, zone, coordinate)
            let there = tactics_calls::thiscall3(0x5b4410, fm, (view == 2) as u32, read_u32(ts + t::ZONE), cell) as u8 as i8;
            if there > -1 {
                if there == slot {
                    return None;
                }
                covered!("tel14_3 swap coordinates");
                // Tactics_Screen::swap_slot_coordinates(a, b)
                tactics_calls::thiscall2(0x8a9d70, ts, slot as u8 as u32, there as u8 as u32);
                return Some(TACTICS_EVENT_REDRAW);
            }
            if there != -1 {
                return None;
            }
            covered!("tel14_3 set a coordinate");
            // Tactics_Screen::set_formation_slot_coordinate(slot, coordinate)
            tactics_calls::thiscall2(0x8a9e90, ts, slot as u8 as u32, cell);
            Some(TACTICS_EVENT_REDRAW)
        }
        0x1f => {
            covered!("tel14_3 zone");
            let v = tel26_item_value(item) as i32;
            let (zx, zy) = ((v / 100) as i16, (v % 100) as i16);
            if (0..=2).contains(&zx) && (0..=3).contains(&zy) {
                let old = read_u32(ts + t::ZONE);
                write_u32(ts + t::ZONE, zx as u16 as u32 | (zy as u16 as u32) << 16);
                write_u32(ts + t::HOVER_ZONE, old);
                write_u8(ts + f::PREVIOUS_VIEW, read_u8(ts + t::VIEW));
            } else {
                tel26_error_box(TACTICS_SCREENS_CPP, 0x15fa);
            }
            Some(TACTICS_EVENT_REDRAW)
        }
        _ => None,
    }
}

/// Cancel and Ok (ids 0x20, 0x21): Cancel with changes made asks first, else frees the object (closed); Ok asks
/// when leaving the match needs a confirmation (`build_return_to_match_text`, the box with the text object), else
/// applies the changes and frees the object (closed). None for the other ids.
pub(crate) unsafe fn tactics_event_leave(ts: usize, id: u16, text: usize, c: (u32, u32)) -> Option<i32> {
    use tactics_event_texts as x;
    let s = tactics_sprintf();
    match id {
        0x20 => {
            covered!("tel14_3 cancel");
            if read_u32(ts + tactics_event_fields::CHANGES_MADE) != 0 {
                let t = s(JOB_TEXT, x::ASK_CANCEL, 1, c.0, c.1, 0x31);
                tactics_confirm(c, x::TACTICS, x::CHANGES_MADE, t, 1, 0x31);
                return Some(TACTICS_EVENT_DONE);
            }
        }
        0x21 => {
            covered!("tel14_3 ok");
            if display_method_ret(0x8a8630, ts, text as u32) != 0 {
                let (mut hb, mut tb) = ([0u8; 200], [0u8; 200]);
                let h = s(hb.as_mut_ptr() as usize, x::PLEASE_CONFIRM, text, 1, c.0, c.1, 0x32);
                let t = s(tb.as_mut_ptr() as usize, x::TACTICS, h, text, 1, c.0, c.1, 0x32);
                exe::confirm_push_text_object(t, h, text, 1, c.0, c.1, 0x32);
                return Some(TACTICS_EVENT_DONE);
            }
            display_method(0x8a80d0, ts); // Tactics_Screen::apply_changes
        }
        _ => return None,
    }
    if ts != 0 {
        tactics_screen_free_inline(ts);
    }
    Some(TACTICS_EVENT_CLOSED)
}

pub(crate) unsafe fn tactics_screen_handle_event_body(cell: u32, text: usize) -> i32 {
    let item = cell as i16;
    let mut ts = get_screen_param(0);
    if ts == 0 {
        let p = exe::crt_operator_new(tactics_screen::SIZE);
        if p != 0 {
            let penalty = get_screen_param(8) as u32;
            let record = get_screen_param(7) as u32;
            let arg4 = get_screen_param(4) as u32;
            let reserve = get_screen_param(3) as u32;
            let m = get_screen_param(2);
            let club = get_screen_param(1);
            ts = tactics_screen_construct(p, club, m, reserve, arg4, record, penalty);
        }
        set_screen_param(0, ts);
    }
    let (mut a, mut b) = (0u32, 0u32);
    exe::database_get_club_kit_colours(read_usize(ts), read_u8(ts + tactics_screen::KIT) as u32, &mut a, &mut b);
    let c = (a, b);
    if read_u32(TACTICS_PENDING_DISPATCH) != 0 {
        if let Some(r) = tactics_event_dialog_answer(ts, c) {
            return r;
        }
    }
    let id = tel26_item_id(item);
    if id == 0xe {
        return tactics_event_last_match(c);
    }
    if let Some(r) = tactics_event_menu(ts, id, item, c) {
        return r;
    }
    if let Some(r) = tactics_event_edit(ts, id, item) {
        return r;
    }
    if let Some(r) = tactics_event_list(ts, id, item) {
        return r;
    }
    if let Some(r) = tactics_event_pitch(ts, id, item) {
        return r;
    }
    if let Some(r) = tactics_event_leave(ts, id, text, c) {
        return r;
    }
    let r = exe::menubar_handle_command(cell);
    if r != 0 {
        return r;
    }
    exe::menubar_handle_object_link(cell)
}

/// The events that only write the view state of the screen object (side by side): a View column, Copy, Copy All,
/// Show Runs, a valid tab and a valid zone.
pub(crate) unsafe fn tactics_event_view_only(id: u16, item: i16) -> bool {
    let v = tel26_item_value(item);
    match id {
        0xd | 0x10 | 0x11 | 0x14 => true,
        0x15 => item != -1 && matches!(v as u8, 1..=3),
        0x1f => (0..=2).contains(&(v as i32 / 100)) && (0..=3).contains(&(v as i32 % 100)),
        _ => false,
    }
}

/// Saves the undo copy: the working team data copied over the undo block, and Undo offered.
pub(crate) unsafe fn tactics_save_undo(ts: usize) {
    core::ptr::copy((ts + tactics_screen::FORMATION) as *const u8, (ts + tactics_screen::UNDO_DATA) as *mut u8, tactics_screen_state::TEAM_DATA_SIZE);
    write_u32(ts + tactics_screen::CAN_UNDO, 1);
}

/// Puts the undo copy back over the working team data (an edit the Formation refused) and withdraws Undo.
pub(crate) unsafe fn tactics_restore_undo(ts: usize) {
    core::ptr::copy((ts + tactics_screen::UNDO_DATA) as *const u8, (ts + tactics_screen::FORMATION) as *mut u8, tactics_screen_state::TEAM_DATA_SIZE);
    write_u32(ts + tactics_screen::CAN_UNDO, 0);
}

/// The phase flag the Formation methods take: 1 in the Without Ball view (view 2 is With Ball: the flag is
/// `view == 2`).
pub(crate) unsafe fn tactics_view_phase(ts: usize) -> u32 {
    (read_u8(ts + tactics_screen::VIEW) == 2) as u32
}

/// The match player of a row: the screen's own (`MATCH_PLAYERS`, 0x1be bytes each) or, for the opponent, the live
/// match's (`engine::PLAYERS`, 20 per side).
pub(crate) unsafe fn tactics_row_match_player(ts: usize, row: i8, opponent: bool) -> usize {
    if opponent {
        let k = read_i8(ts + tactics_screen_state::OPPONENT_SIDE) as isize * 20 + row as isize;
        (read_usize(ts + tactics_screen::MATCH) as isize + engine::PLAYERS as isize + k * 0x1be) as usize
    } else {
        (ts as isize + tactics_screen::MATCH_PLAYERS as isize + row as isize * 0x1be) as usize
    }
}

/// The row of the match record copy: the club's side (`SIDE`) or the opponent's (`OPPONENT_SIDE`).
pub(crate) unsafe fn tactics_record_row(ts: usize, row: i8, opponent: bool) -> usize {
    let side = read_i8(ts + if opponent { tactics_screen_state::OPPONENT_SIDE } else { tactics_screen::SIDE }) as isize;
    (ts as isize + (tactics_screen_state::RECORD + match_record::ROW_STAFF_ID) as isize + side * match_record::SIDE_SIZE as isize
        + row as isize * match_record_row::SIZE as isize) as usize
}

/// The club of a row: the opponent or the screen's club.
pub(crate) unsafe fn tactics_row_club(ts: usize, opponent: bool) -> usize {
    read_usize(ts + if opponent { tactics_screen_state::OPPONENT } else { tactics_screen::CLUB })
}

/// Runs a Tactics_Screen method's original and copy side by side over `regions` (the screen object and what the
/// method writes outside it) and records whether the memory and the results match.
pub(crate) unsafe fn compare_tactics_method<R: PartialEq + core::fmt::Debug>(name: &str, regions: &[(usize, usize)], original: impl FnOnce() -> R, rust: impl FnOnce() -> R) -> R {
    let (a, b, same, mo, mr) = crate::comparison::side_by_side(regions, original, rust);
    SCOREBOARD.record(same && a == b, || format!("tactics screen {}: {:?}/{:?} differences at {:?}", name, a, b, first_differences(&mo, &mr)));
    a
}

/// The Tactics_Screen object as a comparison region.
pub(crate) fn tactics_screen_region(ts: usize) -> (usize, usize) {
    (ts, tactics_screen::SIZE as usize)
}

/// Builds the sub-objects of a new Tactics_Screen (the C++ member constructors): the two match team blocks of the
/// record copy, and for each team data block (working, undo, clipboard) its Formation and its 20 match players.
pub(crate) unsafe fn tactics_screen_construct_members(ts: usize) {
    exe::crt_vector_constructor_iterator(ts + tactics_screen::MATCH_TEAMS, 0x625, 2, MATCH_TEAM_CONSTRUCT_FN, 0x453c10usize);
    for block in [tactics_screen::FORMATION, tactics_screen::UNDO_DATA, tactics_screen::CLIPBOARD_DATA] {
        tactics_screen_calls::call0(FORMATION_CONSTRUCT_FN, ts + block);
        exe::crt_vector_constructor_iterator(ts + block + tactics_screen::TEAM_PLAYERS, 0x1be, 0x14, MATCH_PLAYER_CONSTRUCT_FN, 0x6ebd60usize);
    }
}

/// Copies one Tactics data block of the club into the screen object (captain, set piece takers, penalty order,
/// playmakers) from the Tactics table, with the reserve flag; `default_penalties` fills the club's penalty order
/// first (`set_default_penalty_takers`).
pub(crate) unsafe fn tactics_screen_read_club_tactics(ts: usize, default_penalties: bool) {
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
pub(crate) unsafe fn tactics_screen_default_view(ts: usize) {
    write_u8(ts + tactics_screen::VIEW, 1);
    write_u8(ts + tactics_screen_state::HOVER_VIEW, 1);
    write_u32(ts + tactics_screen::ZONE, 0);
    write_u32(ts + tactics_screen::HOVER_ZONE, 0);
    write_u32(ts + tactics_screen::SHOW_RUNS, 0);
    write_u8(ts + tactics_screen::SUBS_MADE, 0);
}

/// The squad number the human manager gives a player for the penalty order, 0x33 when he has none (0xff).
pub(crate) unsafe fn tactics_penalty_squad_number(player: usize) -> i8 {
    let n = tactics_screen_calls::call2(tactics_screen_fns::HUMAN_MANAGER_GET_PLAYER_SQUAD_NUMBER, read_usize(G_HUMAN_MANAGER), player as u32, 0) as u8;
    if n == 0xff { 0x33 } else { n as i8 }
}

/// Copies a C string (the `repne scasb` / `rep movs` copies of the original).
pub(crate) unsafe fn tactics_copy_string(dst: usize, src: usize) {
    let n = (0..).take_while(|&k| read_u8(src + k) != 0).count() + 1;
    core::ptr::copy(src as *const u8, dst as *mut u8, n);
}

/// The C string compare of the original (inline `strcmp`): equal or not.
pub(crate) unsafe fn tactics_strings_equal(a: usize, b: usize) -> bool {
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

/// `sprintf(out, "%d", n)` (the CRT's).
pub(crate) unsafe fn tactics_sprintf_number(out: usize, n: i32) {
    let f: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    f(out, 0x98806c, n);
}

/// The squad number of a staff at a club (`SQUAD_MANAGER::get_squad_number`).
pub(crate) unsafe fn tactics_squad_number(staff: usize, club: usize) -> i8 {
    exe::squad_manager_get_squad_number(read_usize(G_SQUAD_MANAGER), staff, club)
}

/// The live match player of a row for the per-row getters, or None (the opponent's row without an opponent, no live
/// match).
pub(crate) unsafe fn tactics_live_row(ts: usize, row: i8, opponent: u32) -> Option<usize> {
    if opponent != 0 && read_usize(ts + tactics_screen_state::OPPONENT) == 0 {
        return None;
    }
    if read_usize(ts + tactics_screen::MATCH) == 0 {
        return None;
    }
    Some(tactics_row_match_player(ts, row, opponent != 0))
}

/// A pitch position counts for a swap: 0..10, or -2 (off injured: its previous position).
pub(crate) fn tactics_swappable(position: i8) -> bool {
    position >= 0 || position as u8 == 0xfe
}

/// The formation of a side for the marking getters: the screen's own, or the opponent's in the live match.
pub(crate) unsafe fn tactics_side_formation(ts: usize, opponent: u32) -> usize {
    if opponent == 0 {
        return ts + tactics_screen::FORMATION;
    }
    let side = read_i8(ts + tactics_screen_state::OPPONENT_SIDE) as isize;
    (read_usize(ts + tactics_screen::MATCH) as isize + engine::TACTICS as isize + side * tactics_screen_state::FORMATION_SIZE as isize) as usize
}

/// The coming-on and going-off marks of a substitution between two match players: the minute and position of the
/// one coming on (the other's position, its previous one when off injured), the other substituted off (-4) at that
/// minute.
pub(crate) unsafe fn tactics_mark_substitution(on: usize, off: usize, m: usize) {
    let on = MatchPlayer(on);
    let off = MatchPlayer(off);
    on.set_minute_on(tactics_substitution_minute(m));
    let position = off.position();
    on.set_position(if position as u8 == 0xfe { off.previous_position() } else { position });
}

/// The live match player of a row of the club's side (`SIDE`).
pub(crate) unsafe fn tactics_row_live_player(ts: usize, row: i8) -> usize {
    let k = read_i8(ts + tactics_screen::SIDE) as isize * 20 + row as isize;
    (read_usize(ts + tactics_screen::MATCH) as isize + engine::PLAYERS as isize + k * 0x1be) as usize
}

/// Sets a dialog's choice colour the first time (bit 0 of `guard`, set before the colour is made).
pub(crate) unsafe fn tactics_dialog_choice_colour(guard: usize, colour: usize) {
    let g = read_u8(guard);
    if g & 1 == 0 {
        write_u8(guard, g | 1);
        write_u16(colour, exe::graphics_rgb_to_pixel(0, 0xc0, 0, 0) as u16);
    }
}

/// `ScreenManager::add_text_item` (0x803fd0): x1, y1, x2, y2, five style fields, colour, text, alignment.
pub(crate) unsafe fn tactics_dialog_text_item(f: [u32; 11]) -> u32 {
    type AddText = unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32;
    let g: AddText = core::mem::transmute(FN_SCREENMANAGER_ADD_TEXT_ITEM);
    g(G_SCREEN_MANAGER, f[0], f[1], f[2], f[3], f[4], f[5], f[6], f[7], f[8], f[9], f[10])
}

/// The heading of an instruction row (column 0, in the heading red).
pub(crate) unsafe fn tactics_dialog_heading(area: u32, row: u32) {
    tel26_area_text([area, 0, row, 1, 0, 1, 3, tactics_colour(TACTICS_COLOUR_3154), JOB_TEXT as u32]);
}

/// One choice of an instruction row, its text already in `JOB_TEXT`: the ticked one in the tick colours (style
/// 0x221), else a button with the event `id` and its `value` (editable) or the plain text (read-only), both in the
/// dialog's choice colour.
pub(crate) unsafe fn tactics_dialog_choice(area: u32, col: u32, row: u32, checked: bool, editable: bool, colour: usize, id: u32, value: u32) {
    let tick = tactics_colour(JOB_COLOUR_BDEC);
    if checked {
        tel26_area_text([area, col, row, 0x221, tick, 0xc, 2, tick, JOB_TEXT as u32]);
    } else if editable {
        tel26_area_button([area, col, row, 1, 0, 0xc, 2, tactics_colour(colour), JOB_TEXT as u32, id, value]);
    } else {
        tel26_area_text([area, col, row, 1, 0, 0xc, 2, tactics_colour(colour), JOB_TEXT as u32]);
    }
}

/// The choice's text: the translated format, with the empty string as its argument when it takes one.
pub(crate) unsafe fn tactics_dialog_choice_text(format: usize, takes_empty: bool) {
    if takes_empty {
        tactics_text1(format, TACTICS_EMPTY);
    } else {
        tactics_text(format);
    }
}

/// `draw_pitch` (0x8ac600) by address: the pitch under a dialog, with the goals left and right; the four outputs
/// are not used.
pub(crate) unsafe fn tactics_dialog_pitch(left: i32, top: i32, right: i32, bottom: i32, area: u32, scale: f32) {
    let draw: unsafe extern "C" fn(i32, i32, i32, i32, *mut i32, *mut i32, *mut i32, *mut i32, u32, u32, f32) -> u32 = core::mem::transmute(FN_DRAW_PITCH);
    let mut r = [0i32; 4];
    let p = r.as_mut_ptr();
    draw(left, top, right, bottom, p, p.add(1), p.add(2), p.add(3), 1, area, scale);
}

/// `Tactics_Screen::get_slot_number_text(row, out, 0xc8, no fallback 0, opponent 0)` (0x8aa200) into `out`.
pub(crate) unsafe fn tactics_dialog_number_text(ts: usize, row: i8, out: *mut u8) {
    let g: unsafe extern "thiscall" fn(usize, u32, *mut u8, u32, u32, u32) -> u32 = core::mem::transmute(tactics_screen_fns::GET_SLOT_NUMBER_TEXT);
    g(ts, row as u8 as u32, out, 0xc8, 0, 0);
}

/// "<number> <name>" of a player (`sprintf(JOB_TEXT, "%s %s")`, the CRT's) for a popup row; the name in format 2.
pub(crate) unsafe fn tactics_dialog_popup_text(player: usize, number: *const u8, name: *mut u8) {
    exe::database_format_staff_name(player, name as usize, 0xc8, 2);
    let f: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    f(JOB_TEXT, 0x98f0b8, number, name as *const u8);
}

/// A popup row of the current human's display: kind 2 with its text (0: the empty row), the event `id` and `value`.
pub(crate) unsafe fn tactics_dialog_popup_row(text: u32, id: u32, value: u32, popup: u32) {
    tactics_item([2, 0, 0, 0, 0, 0, 0, 0x10, tactics_colour(JOB_COLOUR_31AA), tactics_colour(JOB_COLOUR_BDEC), 1, 1, tactics_colour(JOB_COLOUR_BDEA), text, 0, id,
        value, popup]);
}

/// The popup of a choice list: a button in `area` at (`col`, `row`) and the popup area it opens.
pub(crate) unsafe fn tactics_dialog_popup(area: u32, col: u32, row: u32) -> u32 {
    let button = tel26_area_button([area, col, row, 0x30, tactics_colour(JOB_COLOUR_BDEA), 0xc, 1, tactics_colour(JOB_COLOUR_3184), 0, 0, 0]);
    tactics_area([0, 0, 0, 0, 0, 0, 0, 0, 0x500030, tactics_colour(JOB_COLOUR_31AA), button])
}

/// The chosen player of a taker or playmaker choice under its popup: "  <name>" (first choice, the CRT's sprintf) or
/// "  or <name>" (second), linked to the player (national team or club kind), greyed (style 0x21) when he is not on
/// the pitch.
pub(crate) unsafe fn tactics_dialog_chosen(ts: usize, area: u32, col: u32, row: u32, second: bool, chosen: u32, on_pitch: bool, name: *mut u8) {
    exe::database_format_staff_name(chosen as usize, name as usize, 0xc8, 2);
    if second {
        tactics_text1(0xa8c3b8, name as usize);
    } else {
        let f: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
        f(JOB_TEXT, 0x987ff0, name as *const u8);
    }
    let link = if exe::database_is_national_team(read_usize(ts + tactics_screen::CLUB)) != 0 { 0x7dc } else { 0x7d7 };
    tel26_area_button([area, col, row, 1, 0, if on_pitch { 1 } else { 0x21 }, 2, tactics_colour(JOB_COLOUR_BDEC), JOB_TEXT as u32, link, chosen]);
}

/// Copies one Tactics_Screen over another field by field (the compiler's assignment, inline in the dialogs'
/// handlers): the arguments, the match record (`match_stats_record_assign`), the fixture copy, the record flag, the
/// opponent, the sides, the kit, the active tactic, the opponent's line-up, the read-only flag, the three team data
/// blocks, the copied zone and phase and the paste and undo flags.
pub(crate) unsafe fn tactics_screen_assign(dst: usize, src: usize) {
    use tactics_screen as t;
    use tactics_screen_state as s;
    let copy = |at: usize, n: usize| core::ptr::copy((src + at) as *const u8, (dst + at) as *mut u8, n);
    copy(0, 0x18);
    tactics_screen_calls::call1(tactics_dialog_fns::MATCH_STATS_RECORD_ASSIGN, dst + s::RECORD, (src + s::RECORD) as u32);
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
}

/// Closes a dialog: the Tactics_Screen copy, when there is one, destroyed (`~Tactics_Screen` by address) and freed;
/// the screen closed (-9). The parameter is left as it is.
pub(crate) unsafe fn tactics_dialog_close(copy: usize) -> i32 {
    if copy != 0 {
        display_method(TACTICS_DESTRUCT_FN, copy);
        exe::crt_operator_delete(copy);
    }
    TACTICS_DIALOG_CLOSED
}

/// `Tactics_Screen::get_slot_number_and_name_text(row, out, size, name format, opponent)` (0x8aa830).
pub(crate) unsafe fn tactics_dialog_number_and_name(ts: usize, row: u8, out: *mut u8, size: u32, format: u32, opponent: u32) {
    let g: unsafe extern "thiscall" fn(usize, u32, *mut u8, u32, u32, u32) -> u32 = core::mem::transmute(tactics_dialog_fns::GET_SLOT_NUMBER_AND_NAME_TEXT);
    g(ts, row as u32, out, size, format, opponent);
}

/// The position slot of a row (`get_slot_pitch_position(row, out, opponent 0)`; 0 when the row is not on the pitch:
/// the original leaves its local unset there).
pub(crate) unsafe fn tactics_dialog_row_slot(ts: usize, row: u8) -> u8 {
    let mut slot = 0u8;
    tactics_calls::slot_pitch_position(ts, row as i8, &mut slot);
    slot
}

/// `Text::clear` (0x4afa40, thiscall).
pub(crate) unsafe fn tactics_text_clear(text: usize) {
    exe::text_clear((text) as usize as *mut usize);
}

/// Appends the screen text (`JOB_TEXT`) to `message` as a paragraph: "\n\n" first when the message is not empty
/// (the inline `strcat`s of the original).
pub(crate) unsafe fn tactics_append_paragraph(message: *mut u8) {
    let append = |src: *const u8| {
        let mut end = 0;
        while *message.add(end) != 0 {
            end += 1;
        }
        tel26_copy_text(message.add(end), src);
    };
    if *message != 0 {
        append(TACTICS_PARAGRAPH as *const u8);
    }
    append(JOB_TEXT as *const u8);
}

/// The link of a player in a screen text: the national team kind (0x7dc) when the screen's club is a national team,
/// else the club kind (0x7d7).
pub(crate) unsafe fn tactics_player_link(text: usize, club: usize, name: *const u8, staff: usize) {
    let kind = if exe::database_is_national_team(club) != 0 { 0x7dc } else { 0x7d7 };
    exe::text_add_link(text, name, kind, staff);
}

/// The shirt number shown for a substitution row: the squad number (its absolute value) when the first team's
/// club uses squad numbers, else the row + 1.
pub(crate) unsafe fn tactics_substitution_shirt(ts: usize, row: i8, mp: usize) -> i8 {
    let mp = MatchPlayer(mp);
    let club = read_usize(ts + tactics_screen::CLUB);
    let squad = read_usize(G_SQUAD_MANAGER);
    if read_u32(ts + tactics_screen_state::RESERVE) == 0 && exe::squad_manager_club_uses_squad_numbers(squad, club, 0) != 0 {
        let n = exe::squad_manager_get_squad_number(squad, mp.staff().addr(), club);
        return (n as i32).wrapping_abs() as u8 as i8;
    }
    (row as u8).wrapping_add(1) as i8
}

/// One shape of the pitch drawing: `Display::add_item` kind 1 (the rectangle, the style, the colour, the outline
/// colour of a circle and the area).
pub(crate) unsafe fn pitch_item(x1: i32, y1: i32, x2: i32, y2: i32, style: u32, colour: u32, outline: u32, area: u32) {
    tactics_item([1, x1 as u32, y1 as u32, x2 as u32, y2 as u32, 0, 0, style, colour, 0, 0, 0, outline, 0, 0, 0, 0, area]);
}

/// The game's `_ftol` (truncation toward zero).
pub(crate) fn pitch_ftol(v: f64) -> i32 {
    v as i32
}

/// The "Error" box of a source file at a line ("v<version> <file>.<ext> <line>") and the left button let go.
pub(crate) unsafe fn tel26_error_box(file: usize, line: u32) {
    let version: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let (mut name, mut ext) = ([0u8; 256], [0u8; 256]);
    exe::crt_strtok(file, 0usize, 0usize, name.as_mut_ptr(), ext.as_mut_ptr());
    let v = version(name.as_mut_ptr(), ext.as_mut_ptr(), line);
    // the original leaves the three arguments of the version call on the stack: they are the rest of the format
    format(0xae24d0, 0x9870e8, v, name.as_ptr(), ext.as_ptr(), line);
    exe::gui_show_message_box(0x9870e0, 0xae24d0, 0);
    write_u32(G_MOUSE_LEFT_BUTTON_DOWN, 0);
}

/// `Display::add_item` on the current human's Display (the 18 fields); returns eax.
pub(crate) unsafe fn tactics_item(f: [u32; 18]) -> u32 {
    tel26_add_item(current_human_display(), f)
}

/// `count_human_managers(connected only)` (0x838d10) by address (hooked when ported).
pub(crate) unsafe fn exe_count_human_managers(connected: u32) -> u32 {
    exe::count_human_managers(connected) as u32
}

/// A menu row of a popup area: style `kind` (2 a row, 0x2002 a ticked row), `flags` (1 enabled, 0x21 greyed), the
/// screen text, the id and value the event handler gets.
pub(crate) unsafe fn tactics_menu_row(kind: u32, flags: u32, id: u32, value: u32, area: u32) -> u32 {
    tactics_item([kind, 0, 0, 0, 0, 0, 0, 0x10, tactics_colour(JOB_COLOUR_31AA), tactics_colour(JOB_COLOUR_BDEC), flags, 1,
        tactics_colour(JOB_COLOUR_BDEA), JOB_TEXT as u32, 0, id, value, area])
}

/// A separator line of a popup area.
pub(crate) unsafe fn tactics_menu_separator(area: u32) -> u32 {
    tactics_item([1, 0, 0, 0, 0, 0, 0, 0x1000010, tactics_colour(JOB_COLOUR_31AA), tactics_colour(JOB_COLOUR_BDEC), 1, 1,
        tactics_colour(JOB_COLOUR_BDEA), 0, 0, 0, 0, area])
}

/// A popup area owned by item `owner` (flags 0x130 for a menu opened by a button, 0x30 for a submenu).
pub(crate) unsafe fn tactics_popup(flags: u32, owner: u32) -> u32 {
    tactics_area([0, 0, 0, 0, 0, 0, 0, 0, flags, tactics_colour(JOB_COLOUR_31AA), owner])
}

/// One View menu row: ticked (0x2002, id 0) when it is the current view `view`, else id 0xd with the view as value.
/// A ticked row of view 7 or more copies its text into `heading`.
pub(crate) unsafe fn tactics_view_row(view: u8, k: u8, area: u32, heading: Option<&mut [u8; 0xc8]>) {
    if view == k {
        tactics_menu_row(0x2002, 1, 0, 0, area);
        if let Some(h) = heading {
            tel26_copy_text(h.as_mut_ptr(), JOB_TEXT as *const u8);
        }
    } else {
        tactics_menu_row(2, 1, 0xd, k as u32, area);
    }
}

/// The view's column of a row: positions (1), goals (3), form (4) and average rating (5) of the player; the match
/// status with the condition (2) or the match rating and goals (6); a player instruction (7 to 0x13), the marking
/// (0x14) or the marker (0x15).
pub(crate) unsafe fn tactics_build_row_column(ts: usize, view: u8, k: i8, player: usize, list: u32, line: u32, number_colour: u32) {
    let player = Staff(player);
    use tactics_screen as t;
    use tactics_calls as x;
    let c = tactics_colour;
    let on_pitch_colour = || if x::slot_pitch_position(ts, k, core::ptr::null_mut()) != 0 { JOB_COLOUR_BDEC } else { JOB_COLOUR_317E };
    let player_data = || if !player.is_null() { player.player().addr() } else { 0 };
    match view {
        1 => {
            if player.is_null() || player_data() == 0 {
                return;
            }
            let positions: unsafe extern "C" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(FN_DATABASE_FORMAT_PLAYER_POSITIONS);
            positions(player_data(), JOB_TEXT, 0x7d0, 1);
            let colour = on_pitch_colour();
            tel26_area_text([list, 6, line, 1, 0, 1, 1, c(colour), JOB_TEXT as u32]);
        }
        3 | 4 | 5 => {
            if player.is_null() || player_data() == 0 {
                return;
            }
            if view == 4 {
                let form: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(FN_PLAYERSTATS_FORMAT_RECENT_RATINGS);
                form(read_usize(G_PLAYER_STATS), player_data(), JOB_TEXT, 0x7d0);
            } else {
                let category = if exe::database_is_national_team(read_usize(ts + t::CLUB)) != 0 { 5 } else { 6 };
                let field = if view == 3 { 2 } else { 0x11 };
                exe::playerstats_format_player_stat(read_usize(G_PLAYER_STATS), player_data(), category, field, JOB_TEXT, 0x7d0, 1, 0, 1);
            }
            let colour = on_pitch_colour();
            tel26_area_text([list, 6, line, 1, 0, 0xc, 1, c(colour), JOB_TEXT as u32]);
        }
        2 | 6 => {
            // the status colour goes over the number colour's local (a word): without status text it is not used
            let mut status_colour: u32 = number_colour;
            x::slot_status_text(ts, k, &mut status_colour);
            if read_u8(JOB_TEXT) != 0 {
                tel26_area_text([list, 6, line, 1, 0, 1, 1, status_colour & 0xffff, JOB_TEXT as u32]);
            }
            if view == 2 {
                let condition = x::slot_byte(0x8aafd0, ts, k);
                if condition == 0 {
                    return;
                }
                if condition > 0 {
                    tactics_sprintf1(JOB_TEXT, 0x98e8d4, condition as i32);
                } else {
                    tactics_text(0x98e98c);
                }
                let colour = on_pitch_colour();
                tel26_area_text([list, 7, line, 1, 0, 0xc, 1, c(colour), JOB_TEXT as u32]);
            } else {
                let rating = x::slot_byte(0x8ab150, ts, k);
                if rating <= 0 {
                    return;
                }
                tactics_sprintf1(JOB_TEXT, 0x98806c, rating as i32);
                tel26_area_text([list, 7, line, 1, 0, 0xc, 2, c(JOB_COLOUR_BDE8), JOB_TEXT as u32]);
                let goals = x::slot_byte(0x8ab260, ts, k);
                if goals <= 0 {
                    return;
                }
                tactics_sprintf1(JOB_TEXT, 0x98806c, goals as i32);
                tel26_area_text([list, 8, line, 1, 0, 0xc, 1, c(JOB_COLOUR_BDEC), JOB_TEXT as u32]);
            }
        }
        _ => {
            let mut slot: u8 = 0;
            if x::slot_pitch_position(ts, k, &mut slot) == 0 {
                return;
            }
            let fm = ts + t::FORMATION;
            let instructions = exe::formation_get_player_instructions(fm, slot as i8);
            let (lo, hi) = (instructions as u32, (instructions >> 32) as u32);
            if view == 0x14 {
                let marking = exe::formation_get_marking_type(fm, slot as i8);
                if marking & 1 != 0 {
                    tactics_text1(0xa8ba0c, TACTICS_EMPTY);
                    let v = tactics_option_value(2, 0, slot);
                    tactics_option_button(list, line, JOB_COLOUR_317E, 0x1b, v);
                }
                if marking & 2 != 0 {
                    tactics_text1(0xa8b9e8, TACTICS_EMPTY);
                    let v = tactics_option_value(4, 0, slot);
                    tactics_option_button(list, line, JOB_COLOUR_BDEC, 0x1b, v);
                }
                if marking & 4 != 0 {
                    let marked = x::slot_staff(0x8abb50, ts, k);
                    if marked != 0 {
                        let mut initials = [0u8; 0xc8];
                        exe::database_format_staff_name(marked, initials.as_mut_ptr() as usize, 0xc8, 6);
                        tactics_text2(0xa8b9ac, initials.as_ptr() as usize, TACTICS_EMPTY);
                    } else {
                        tactics_text1(0xa8b984, TACTICS_EMPTY);
                    }
                    let v = tactics_option_value(1, 0, slot);
                    tactics_option_button(list, line, JOB_COLOUR_BDEC, 0x1b, v);
                }
            } else if view == 0x15 {
                let marker = x::slot_staff(0x8abd00, ts, k);
                if marker != 0 {
                    exe::database_format_staff_name(marker, JOB_TEXT, 0x7d0, 6);
                    let id = if exe::database_is_national_team(read_usize(ts + t::CLUB)) != 0 { 0x7dc } else { 0x7d7 };
                    tactics_option_button(list, line, JOB_COLOUR_BDEC, id, marker as u32);
                } else {
                    tel26_copy_text(JOB_TEXT as *mut u8, 0xa8b980 as *const u8);
                    tel26_area_text([list, 5, line, 1, 0, 0xc, 1, c(JOB_COLOUR_BDEC), JOB_TEXT as u32]);
                }
            } else {
                for &(mlo, mhi, text, with_empty, colour, nlo, nhi) in tactics_instruction_options(view).iter() {
                    if (lo & mlo) | (hi & mhi) == 0 {
                        continue;
                    }
                    if with_empty {
                        tactics_text1(text, TACTICS_EMPTY);
                    } else {
                        tactics_text(text);
                    }
                    let v = tactics_option_value(nlo, nhi, slot);
                    tactics_option_button(list, line, colour, 0x1a, v);
                }
            }
        }
    }
}

/// The run arrows of the overview: for each slot whose attacking position differs from its defending one, a line
/// from one to the other, nudged off the circles, and its two heads.
pub(crate) unsafe fn tactics_build_overview_runs(ts: usize, pitch: &TacticsPitch) {
    use tactics_screen as t;
    use tactics_calls as x;
    let fm = ts + t::FORMATION;
    for slot in 0..0xbi8 {
        let mut pending: u32 = 0;
        if x::find_slot_at_position(ts, slot, &mut pending) <= -1 {
            continue;
        }
        let mut later: u32 = 0;
        if x::duplicate_position(fm, slot, &mut later) == 0 {
            continue;
        }
        let from = exe::formation_get_slot_position(fm, slot as u8 as u32, 0);
        let to = exe::formation_get_slot_position(fm, slot as u8 as u32, 1);
        if from == to {
            continue;
        }
        let players = || if read_u32(ts + t::MATCH) != 0 { ts + t::MATCH_PLAYERS } else { 0 };
        let wide = x::central_in_line(fm, from, players());
        let (mut x1, mut y1) = (0i32, 0i32);
        x::pitch_point(from, &pitch.r, &mut x1, &mut y1, wide);
        let (mut line_from, mut side_from, mut line_to, mut side_to) = (0u16, 0u16, 0u16, 0u16);
        exe::formation_split_position_code(from as u32, &mut line_from, &mut side_from);
        exe::formation_split_position_code(to as u32, &mut line_to, &mut side_to);
        let wide = if side_from != side_to { x::central_in_line(fm, to, players()) } else { x::central_in_line(fm, from, players()) };
        let (mut x2, mut y2) = (0i32, 0i32);
        x::pitch_point(to, &pitch.r, &mut x2, &mut y2, wide);
        if x::slot_by_position(fm, to) > -1 {
            let (dx, dy) = ((x2 - x1).wrapping_abs(), (y2 - y1).wrapping_abs());
            if y2 < y1 && dy > dx {
                y2 += 0x10;
            } else if y2 > y1 && dy > dx {
                y2 -= 0x12;
            } else if x2 < x1 {
                x2 += 0xd;
            } else if x2 > x1 {
                x2 -= 0xd;
            }
        }
        let colour = if pending == 0 { read_u32(tactics_build_statics::RED) } else { tactics_colour(tactics_build_statics::GREEN_80) };
        let colour = colour & 0xffff;
        tactics_item([1, x1 as u32, y1 as u32, x2 as u32, y2 as u32, 0, 0, 0x2400, colour, 0, 0, 0, 0, 0, 0, 0, 0, u32::MAX]);
        let h = tactics_arrow_heads(x1, y1, x2, y2);
        tactics_item([1, h[0] as u32, h[1] as u32, x2 as u32, y2 as u32, 0, 0, 0x400, colour, 0, 0, 0, 0, 0, 0, 0, 0, u32::MAX]);
        tactics_item([1, h[2] as u32, h[3] as u32, x2 as u32, y2 as u32, 0, 0, 0x400, colour, 0, 0, 0, 0, 0, 0, 0, 0, u32::MAX]);
    }
}

/// The circles of the players: shadow (a later duplicate), the number (clickable and draggable, id 0x1d, plus the
/// drop target 0x1e when editable), the customised mark and, in the overview, the name under the circle.
pub(crate) unsafe fn tactics_slot_circle(ts: usize, cx: i32, cy: i32, ord: u32, back: u32, line: u32, later: u32, overview: bool) {
    use tactics_screen as t;
    let read_only = read_u32(ts + t::READ_ONLY) != 0;
    let (top, bottom) = if overview { (cy - 0x12, cy + 8) } else { (cy - 0xd, cy + 0xd) };
    if later != 0 {
        if overview {
            tactics_item([1, (cx - 0x11) as u32, (cy - 0x16) as u32, (cx + 9) as u32, (cy + 4) as u32, 0, 0, 0x10030, line, line, 0, 1, back,
                TACTICS_EMPTY as u32, 0, 0, 0, u32::MAX]);
        } else {
            tactics_item([1, (cx - 0x11) as u32, (cy - 0x11) as u32, (cx + 9) as u32, (cy + 9) as u32, 0, 0, 0x10030, line, line, 0, 1, back,
                TACTICS_EMPTY as u32, 0, 0, 0, u32::MAX]);
        }
    }
    let flags = if read_only { 1 } else { 0x246 };
    tactics_item([flags, (cx - 0xd) as u32, top as u32, (cx + 0xd) as u32, bottom as u32, 0, 0, 0x10030, line, line, 0, 1, back,
        JOB_TEXT as u32, 0, 0x1d, ord, u32::MAX]);
    if overview && !read_only {
        tactics_item([0x346, (cx - 0xd) as u32, top as u32, (cx + 0xd) as u32, bottom as u32, 0, 0, 0x10020, line, line, 0, 1, back,
            0, 0, 0x1e, ord, u32::MAX]);
    }
}

/// The customised mark around a circle.
pub(crate) unsafe fn tactics_slot_customised_mark(cx: i32, top: i32, bottom: i32, back: u32) {
    tactics_item([1, (cx - 0xe) as u32, top as u32, (cx + 0xe) as u32, bottom as u32, 0, 0, 0x10020, 0, 0, 0, 1, back, 0, 0, 0, 0, u32::MAX]);
}

/// A ball zone square of the 3 x 4 grid (x 0..2 across, y 0..3 up from the bottom), grown by `grow` pixels.
pub(crate) fn tactics_zone_rect(r: &[i32; 4], zx: i32, zy: i32, grow: i32) -> [u32; 4] {
    let (w, h) = (r[2] - r[0], r[3] - r[1]);
    [((zx * w) / 3 + r[0] - grow) as u32, (r[3] - ((zy + 1) * h) / 4 - grow) as u32, (((zx + 1) * w) / 3 + r[0] + grow) as u32,
        (r[3] - (zy * h) / 4 + grow) as u32]
}

/// A formation coordinate (27 x 36 grid) as the pitch square around it: (left, bottom, right, top) like the original's
/// registers: x = c.x * w / 27 + left, y = bottom - c.y * h / 36, the square w / 27 wide.
pub(crate) fn tactics_coordinate_square(r: &[i32; 4], c: [i16; 2]) -> (i32, i32, i32, i32) {
    let (w, h) = (r[2] - r[0], r[3] - r[1]);
    let x = (c[0] as i32).wrapping_mul(w) / 27 + r[0];
    let y = r[3] - (c[1] as i32).wrapping_mul(h) / 36;
    let d = w / 27;
    (x, y, x + d, y - d)
}

/// `langlib_sprintf` (variadic: the original pushes the arguments of the next call with each text, and they go in
/// here too).
pub(crate) unsafe fn tactics_sprintf() -> unsafe extern "C" fn(usize, usize, ...) -> usize {
    core::mem::transmute(FN_LANGLIB_SPRINTF)
}

/// `confirm_push(title, heading, text, buttons, colour a, colour b, code)` (0x591840) with the translated heading and
/// title (each sprintf gets the arguments still on the stack, as in the original). `text` is already formatted.
pub(crate) unsafe fn tactics_confirm(c: (u32, u32), title: usize, heading: usize, text: usize, buttons: u32, code: u32) {
    let s = tactics_sprintf();
    let (mut hb, mut tb) = ([0u8; 200], [0u8; 200]);
    let h = s(hb.as_mut_ptr() as usize, heading, text, buttons, c.0, c.1, code);
    let t = s(tb.as_mut_ptr() as usize, title, h, text, buttons, c.0, c.1, code);
    exe::confirm_push(t, h, text, buttons, c.0, c.1, code);
}

/// `open_file_dialog(title, heading, mode, folder, extension, default name, on ok, colour a, colour b, code, 0, 0,
/// 0, 0)` (0x58f500) with the translated heading and title.
pub(crate) unsafe fn tactics_file_dialog(c: (u32, u32), title: usize, heading: usize, mode: u32, folder: usize, ext: usize, name: usize, on_ok: usize, code: u32) {
    let s = tactics_sprintf();
    let (mut hb, mut tb) = ([0u8; 200], [0u8; 200]);
    let h = s(hb.as_mut_ptr() as usize, heading, mode, folder, ext, name, on_ok, c.0, c.1, code, 0, 0, 0, 0);
    let t = s(tb.as_mut_ptr() as usize, title, h, mode, folder, ext, name, on_ok, c.0, c.1, code, 0, 0, 0, 0);
    exe::open_file_dialog(t, h, mode, folder, ext, name, on_ok, (c.0) as u32 as usize, (c.1) as u32 as usize, code, 0, 0, 0, 0);
}

/// `HUMAN_MANAGER::get_folder_path(current manager, 1)` (0x5fc5b0): the formations folder.
pub(crate) unsafe fn tactics_folder() -> usize {
    exe::human_manager_get_folder_path(read_usize(G_HUMAN_MANAGER), 0, 1)
}

/// `utils_build_file_path("CM3_TACTICS", "")` copied into a static path buffer (the inline strcpy).
pub(crate) unsafe fn tactics_preset_folder(dst: usize) -> usize {
    tel26_copy_text(dst as *mut u8, exe::utils_build_file_path(TACTICS_FOLDER_KEY, TACTICS_NO_TEXT) as *const u8);
    dst
}

/// `formation_read_file_password(path, password, creator, 0)` (0x5b8600); true when the file has a password that is
/// not the current manager's (the inline strcmp against `HUMAN_MANAGER::get_password`).
pub(crate) unsafe fn tactics_locked_by_other(path: usize, password: &mut [u8; 0x34], creator: &mut [u8; 200]) -> bool {
    exe::formation_read_file_password(path, password.as_mut_ptr(), creator.as_mut_ptr(), 0);
    if password[0] == 0 {
        return false;
    }
    let mine = tactics_calls::password(0) as *const u8;
    let mut k = 0;
    loop {
        let b = *mine.add(k);
        if b != password[k] {
            return true;
        }
        if b == 0 {
            return false;
        }
        k += 1;
    }
}

/// `Tactics_Screen::load_formation(path, preset)` (0x8a95d0) and `save_formation(path, locking manager)` (0x8a9700).
pub(crate) unsafe fn tactics_load(ts: usize, path: usize, preset: u32) -> u32 {
    tactics_calls::thiscall2(0x8a95d0, ts, path as u32, preset)
}

pub(crate) unsafe fn tactics_save(ts: usize, path: usize, manager: usize) -> u32 {
    tactics_calls::thiscall2(0x8a9700, ts, path as u32, manager as u32)
}

/// `Tactics_Screen::set_preset_formation(preset)` (0x8a9950).
pub(crate) unsafe fn tactics_set_preset(ts: usize, preset: usize) {
    display_method1(0x8a9950, ts, preset as u32);
}

/// The undo copy of the working team data (`UNDO_DATA`), with `CAN_UNDO` set.
pub(crate) unsafe fn tactics_event_save_undo(ts: usize) {
    use tactics_screen as t;
    core::ptr::copy_nonoverlapping((ts + t::FORMATION) as *const u8, (ts + t::UNDO_DATA) as *mut u8, tactics_event_fields::TEAM_DATA_SIZE);
    write_u32(ts + t::CAN_UNDO, 1);
}

/// The working team data back from the undo copy.
pub(crate) unsafe fn tactics_event_restore_undo(ts: usize) {
    use tactics_screen as t;
    core::ptr::copy_nonoverlapping((ts + t::UNDO_DATA) as *const u8, (ts + t::FORMATION) as *mut u8, tactics_event_fields::TEAM_DATA_SIZE);
}

/// The row of the captain (the row whose player has the id `CAPTAIN`), or -1.
pub(crate) unsafe fn tactics_captain_row(ts: usize) -> i8 {
    let mut r: i8 = 0;
    loop {
        let p = tactics_calls::slot_player(ts, r);
        if p != 0 && read_u32(p) == read_u32(ts + tactics_screen::CAPTAIN) {
            return r;
        }
        r += 1;
        if r >= 0x14 {
            return -1;
        }
    }
}

/// Destroys and frees the Tactics_Screen the way the Cancel and Ok answers do: the three team data blocks by their
/// destructor (clipboard, undo, working), the two match team blocks and the memory.
pub(crate) unsafe fn tactics_event_free(ts: usize) {
    use tactics_screen as t;
    display_method(TACTICS_TEAM_DATA_DESTRUCT_FN, ts + t::CLIPBOARD_DATA);
    display_method(TACTICS_TEAM_DATA_DESTRUCT_FN, ts + t::UNDO_DATA);
    display_method(TACTICS_TEAM_DATA_DESTRUCT_FN, ts + t::FORMATION);
    exe::crt_vector_destructor_iterator(ts + t::MATCH_TEAMS, 0x625, 2, MATCH_TEAM_DESTRUCT_FN);
    exe::crt_operator_delete(ts);
}

/// A copy of the Tactics_Screen (`Tactics_Screen::Tactics_Screen(const Tactics_Screen*)`, 0x8a7e00), or 0 when the
/// allocation fails.
pub(crate) unsafe fn tactics_screen_copy(ts: usize) -> usize {
    let p = exe::crt_operator_new(tactics_screen::SIZE);
    if p == 0 {
        return 0;
    }
    exe::new_tactics_screen(p, ts)
}

/// Loads the recent file of parameter 6 (`HUMAN_MANAGER::get_recent_file(index, path, 0x104, 0, 0, 1)`): a file
/// locked by another manager's password or that does not load is a confirm box (done). None when it loaded.
pub(crate) unsafe fn tactics_load_recent(ts: usize, c: (u32, u32), path: &mut [u8; 0x104], password: &mut [u8; 0x34], creator: &mut [u8; 200]) -> Option<i32> {
    use tactics_event_texts as x;
    let s = tactics_sprintf();
    exe::human_manager_get_recent_file(read_usize(G_HUMAN_MANAGER), get_screen_param(6) as u32, path.as_mut_ptr(), 0x104, 0, (0) as u32 as usize, 1);
    let p = path.as_ptr() as usize;
    if tactics_locked_by_other(p, password, creator) {
        let text = s(JOB_TEXT, x::LOCKED_BY, p, creator.as_ptr(), 0, c.0, c.1, 0);
        tactics_confirm(c, x::LOAD_FORMATION, x::UNABLE_TO_LOAD, text, 0, 0);
        return Some(TACTICS_EVENT_DONE);
    }
    if tactics_load(ts, p, 0) == 0 {
        let text = s(JOB_TEXT, x::NOT_LOADED, p, 0, c.0, c.1, 0);
        tactics_confirm(c, x::LOAD_FORMATION, x::UNABLE_TO_LOAD, text, 0, 0);
        return Some(TACTICS_EVENT_DONE);
    }
    None
}

/// The item of the drag and drop pair of `item` (-1 for none).
pub(crate) unsafe fn tactics_partner(item: i16) -> i16 {
    if item == -1 {
        return -1;
    }
    read_i16((current_human_display() as isize + item as isize * 0x18c + tactics_event_item::PARTNER as isize) as usize)
}

/// A display item's rectangle at `field` (x1, y1, x2, y2).
pub(crate) unsafe fn tactics_item_rect(item: i16, field: usize) -> [i32; 4] {
    let a = (current_human_display() as isize + item as isize * 0x18c + field as isize) as usize;
    [read_i32(a), read_i32(a + 4), read_i32(a + 8), read_i32(a + 12)]
}

/// The pitch position of the overview list nearest to the dropped circle `d` in the pitch rectangle `p`: for each
/// position its pitch point (`get_pitch_point_for_position_flags`, wide when the line has a central slot), the
/// distance from the circle's centre (truncated square root). Returns (distance, position), 800 when none is nearer.
pub(crate) unsafe fn tactics_nearest_position(ts: usize, p: &[i32; 4], d: &[i32; 4]) -> (i32, u16) {
    use tactics_screen as t;
    let (mut best, mut pos) = (800i32, 0u16);
    let half_h = (d[3] - d[1]) / 2;
    let half_w = (d[2] - d[0]) / 2;
    let mut k = TACTICS_PITCH_POSITIONS;
    while read_i16(k) != -1 {
        let position = read_u16(k);
        let players = if read_u32(ts + t::MATCH) != 0 { ts + t::MATCH_PLAYERS } else { 0 };
        let wide = tactics_calls::central_in_line(ts + t::FORMATION, position, players);
        let (mut x, mut y) = (0i32, 0i32);
        tactics_calls::pitch_point(position, p, &mut x, &mut y, wide);
        let dy = d[1].wrapping_sub(y).wrapping_add(half_h);
        let dx = d[0].wrapping_sub(x).wrapping_add(half_w);
        let dist = tactics_sqrt(dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy)));
        if dist < best {
            best = dist;
            pos = position;
        }
        k += 2;
    }
    (best, pos)
}

/// A thiscall with one argument that returns eax.
pub(crate) unsafe fn display_method_ret(f: usize, this: usize, a: u32) -> u32 {
    let g: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(f);
    g(this, a)
}

/// The minute of the live match for a substitution: the clock / 11, capped at the end of the period (45, 90, 105 or
/// 120 when the period ends at tick 0x1ef, 0x3de, 0x483 or 0x528).
pub(crate) unsafe fn tactics_substitution_minute(m: usize) -> i8 {
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

/// `langlib_sprintf(JOB_TEXT, fmt, a, b)`.
pub(crate) unsafe fn tactics_text2(fmt: usize, a: usize, b: usize) {
    let f: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    f(JOB_TEXT, fmt, a, b);
}

/// `sprintf(out, fmt, a)` (the CRT's).
pub(crate) unsafe fn tactics_sprintf1(out: usize, fmt: usize, a: i32) {
    let f: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    f(out, fmt, a);
}

/// The player instruction options of the column views 7 to 0x13: (low mask, high mask, text, the text takes the
/// empty-string argument, colour, the next option's low and high masks). Each option the player has shows a button
/// whose value is the next option's bit index with the formation slot (6 bits each).
pub(crate) unsafe fn tactics_instruction_options(view: u8) -> &'static [(u32, u32, usize, bool, usize, u32, u32)] {
    const T: usize = JOB_COLOUR_317E;
    const N: usize = JOB_COLOUR_BDEC;
    const NO: usize = 0x98e8cc;
    const YES: usize = 0x98e8d0;
    match view {
        7 => &[(1, 0, 0xa8bda8, true, T, 2, 0), (2, 0, 0xa8bd7c, true, N, 4, 0), (4, 0, 0xa8bd50, true, N, 8, 0),
            (8, 0, 0xa8bd24, true, N, 0x10, 0), (0x10, 0, 0xa8bcf8, true, N, 1, 0)],
        8 => &[(0x200, 0, 0xa8bcd8, true, T, 0x400, 0), (0x400, 0, 0xa8bcb8, true, N, 0x800, 0), (0x800, 0, 0xa8bc98, true, N, 0x1000, 0),
            (0x1000, 0, 0xa8bc78, true, N, 0x200, 0)],
        9 => &[(0x2000, 0, 0xa8bc58, true, T, 0x4000, 0), (0x4000, 0, 0xa8bc3c, true, N, 0x8000, 0), (0x8000, 0, 0xa8bc1c, true, N, 0x2000, 0)],
        0xa => &[(0x20, 0, 0xa8bbfc, true, T, 0x40, 0), (0x40, 0, 0xa8bbdc, true, N, 0x80, 0), (0x80, 0, 0xa8bbbc, true, N, 0x100, 0),
            (0x100, 0, 0xa8bb9c, true, N, 0x20, 0)],
        0xb => &[(0x10000, 0, 0xa8bb5c, true, T, 0x40000000, 0), (0x40000000, 0, 0xa8bb24, true, N, 0x20000, 0), (0x20000, 0, 0xa8bae4, true, N, 0x10000, 0)],
        0xc => &[(0x40000, 0, 0xa8baa4, true, T, 0x80000, 0), (0x80000, 0, 0xa8ba6c, true, N, 0x80000000, 0), (0x80000000, 0, 0xa8ba2c, true, N, 0x40000, 0)],
        0xd => &[(0x100000, 0, NO, false, T, 0x200000, 0), (0x200000, 0, YES, false, N, 0x100000, 0)],
        0xe => &[(0x400000, 0, NO, false, T, 0x800000, 0), (0x800000, 0, YES, false, N, 0x400000, 0)],
        0xf => &[(0x1000000, 0, NO, false, T, 0x2000000, 0), (0x2000000, 0, YES, false, N, 0x1000000, 0)],
        0x10 => &[(0x4000000, 0, NO, false, T, 0x8000000, 0), (0x8000000, 0, YES, false, N, 0x4000000, 0)],
        0x11 => &[(0x10000000, 0, NO, false, T, 0x20000000, 0), (0x20000000, 0, YES, false, N, 0x10000000, 0)],
        0x12 => &[(0, 2, NO, false, T, 0, 1), (0, 1, YES, false, N, 0, 2)],
        0x13 => &[(0, 8, NO, false, T, 0, 4), (0, 4, YES, false, N, 0, 8)],
        _ => &[],
    }
}

/// The value of an option button: the next option's bit index (6 bits) and the formation slot above it.
pub(crate) unsafe fn tactics_option_value(next_lo: u32, next_hi: u32, slot: u8) -> u32 {
    (tactics_calls::mask_index(next_lo, next_hi) & 0x3f) | ((slot as u32 & 0x3f) << 6)
}

/// An option button in column 5 of row `row`: id 0x1a (instructions), 0x1b (marking) or a person's id.
pub(crate) unsafe fn tactics_option_button(area: u32, row: u32, colour: usize, id: u32, value: u32) {
    tel26_area_button([area, 5, row, 1, 0, 0xc, 1, tactics_colour(colour), JOB_TEXT as u32, id, value]);
}

/// The two arrowheads of a run from (x1, y1) to (x2, y2), computed on the x87 like the original (fpatan, fsin,
/// fcos, the game's `_ftol` 0x945640): the angle a = atan2(y1 - y2, x2 - x1); the first head at
/// (x2 - 8 sin t, y2 + 8 cos t) with t = pi - a - pi/2 - c, the second at (x2 - 8 cos u, y2 + 8 sin u) with u = a - c
/// (c = 0.4488, the constants read from the exe). Returns (x, y) of each head.
pub(crate) unsafe fn tactics_arrow_heads(x1: i32, y1: i32, x2: i32, y2: i32) -> [i32; 4] {
    // [0] dy, [1] dx, [2] x2, [3] y2, [4..8] the results, [8..10] u (f64), then the constants
    let mut m = [0u32; 19];
    m[0] = y1.wrapping_sub(y2) as u32;
    m[1] = x2.wrapping_sub(x1) as u32;
    m[2] = x2 as u32;
    m[3] = y2 as u32;
    for (k, a) in [0x9703e8usize, 0x9703e0, 0x9703d8, 0x968f10].iter().enumerate() {
        m[10 + 2 * k] = read_u32(*a);
        m[11 + 2 * k] = read_u32(*a + 4);
    }
    m[18] = 0x945640;
    core::arch::asm!(
        "fild dword ptr [edi]",
        "fild dword ptr [edi + 4]",
        "fpatan",
        "fld qword ptr [edi + 40]",
        // fsub st(0), st(1)
        ".byte 0xd8, 0xe1",
        "fsub qword ptr [edi + 48]",
        "fsub qword ptr [edi + 56]",
        "fild dword ptr [edi + 8]",
        "fld st(1)",
        "fsin",
        "fmul qword ptr [edi + 64]",
        // fsubp st(1), st(0)
        ".byte 0xde, 0xe9",
        "call dword ptr [edi + 72]",
        "mov dword ptr [edi + 16], eax",
        "fcos",
        "fmul qword ptr [edi + 64]",
        "fiadd dword ptr [edi + 12]",
        "call dword ptr [edi + 72]",
        "mov dword ptr [edi + 20], eax",
        "fsub qword ptr [edi + 56]",
        "fstp qword ptr [edi + 32]",
        "fld qword ptr [edi + 32]",
        "fsin",
        "fmul qword ptr [edi + 64]",
        "fiadd dword ptr [edi + 12]",
        "call dword ptr [edi + 72]",
        "mov dword ptr [edi + 24], eax",
        "fild dword ptr [edi + 8]",
        "fld qword ptr [edi + 32]",
        "fcos",
        "fmul qword ptr [edi + 64]",
        ".byte 0xde, 0xe9",
        "call dword ptr [edi + 72]",
        "mov dword ptr [edi + 28], eax",
        in("edi") m.as_mut_ptr(),
        clobber_abi("C"),
    );
    [m[4] as i32, m[5] as i32, m[7] as i32, m[6] as i32]
}

/// `(int) sqrt(n)` on the x87 like the original (fild, fsqrt, the game's `_ftol` 0x945640).
pub(crate) unsafe fn tactics_sqrt(n: i32) -> i32 {
    let mut m = [n as u32, 0x945640, 0];
    core::arch::asm!(
        "fild dword ptr [edi]",
        "fsqrt",
        "call dword ptr [edi + 4]",
        "mov dword ptr [edi + 8], eax",
        in("edi") m.as_mut_ptr(),
        clobber_abi("C"),
    );
    m[2] as i32
}

/// tactics_screens.cpp, the source file of the screen's error boxes.
pub(crate) const TACTICS_SCREENS_CPP: usize = 0xa8b94c;

/// The screen's builders and handlers: `tactics_screen_build`, its thunk (the reserve tactics screen), the event
/// handler and the two close callbacks.
pub(crate) const TACTICS_BUILD_FN: usize = 0x899ea0;

pub(crate) const TACTICS_BUILD_RESERVE_FN: usize = 0x89f700;

pub(crate) const TACTICS_HANDLE_EVENT_FN: usize = 0x89f710;

pub(crate) const TACTICS_CLOSE_WITHOUT_APPLY_FN: usize = 0x8a2c60;

pub(crate) const TACTICS_CLOSE_AND_APPLY_FN: usize = 0x8a2d70;

/// `Tactics_Screen::Tactics_Screen(club, match, reserve, arg4, record index, penalty mode)` (thiscall, ret 0x18) and
/// `Tactics_Screen::~Tactics_Screen` (thiscall).
pub(crate) const TACTICS_CONSTRUCT_FN: usize = 0x8a7440;

pub(crate) const TACTICS_DESTRUCT_FN: usize = 0x8a2ca0;

/// The element destructors of the match player records (0x1be bytes) and of the match team blocks (0x625).
pub(crate) const MATCH_PLAYER_DESTRUCT_FN: usize = 0x6ebd60;

pub(crate) const MATCH_TEAM_DESTRUCT_FN: usize = 0x453c10;

/// Colours of the screen items not named in tel26_1 (words, zero-extended when pushed).
pub(crate) const TACTICS_COLOUR_BDD6: usize = 0xaebdd6;

pub(crate) const TACTICS_COLOUR_3154: usize = 0xae3154;

pub(crate) const TACTICS_COLOUR_31A8: usize = 0xae31a8;

/// The flag byte that offers the preset formations ("Load Preset") [inference].
pub(crate) const TACTICS_PRESETS_ON: usize = 0xb7434a;

/// The empty string.
pub(crate) const TACTICS_EMPTY: usize = 0xad9c64;

/// The preset formation names that have a translated text (the name of the tactic block, then the text).
pub(crate) const TACTICS_PRESET_NAMES: [usize; 8] = [0xa8b758, 0xa8b748, 0xa8b720, 0xa8b6e4, 0xa1593c, 0xa8b658, 0xa8b62c, 0x9b75dc];

/// The column views of the View menu: (view, text). Views 1..6 only tick; 7.. also copy their text as the column
/// heading.
pub(crate) const TACTICS_VIEW_TEXTS: [(u8, usize); 13] = [(7, 0xa8bf4c), (8, 0xa8bf40), (9, 0xa8bf34), (0xa, 0xa8bf28), (0xb, 0xa8bf18),
    (0xc, 0xa8bf08), (0xd, 0xa8befc), (0xe, 0xa8beec), (0xf, 0xa8bedc), (0x10, 0xa8becc), (0x11, 0xa8bec0), (0x12, 0xa8beac),
    (0x13, 0xa8bea0)];

/// The pitch rectangle (left, top, right, bottom) and the colours the pitch views use.
pub(crate) struct TacticsPitch {
    pub(crate) r: [i32; 4],
    /// The goalkeeper's colours (shirt, outline): dark on a white-ish kit, else the screen's light green.
    pub(crate) keeper_back: u32,
    pub(crate) keeper_line: u32,
    /// The outfield colours: the screen title's colours.
    pub(crate) field_back: u32,
    pub(crate) field_line: u32,
}

/// The results of the handler: done (-4), the screen closed (-9), redraw (-0xb).
pub(crate) const TACTICS_EVENT_DONE: i32 = -4;

pub(crate) const TACTICS_EVENT_CLOSED: i32 = -9;

pub(crate) const TACTICS_EVENT_REDRAW: i32 = -0xb;

/// The flag that dispatches the answer of a pending dialog (`G_PENDING_COMMAND_CODE` / `_ARG`).
pub(crate) const TACTICS_PENDING_DISPATCH: usize = 0xb77c84;

/// The pitch positions offered in the overview (words, -1 at the end).
pub(crate) const TACTICS_PITCH_POSITIONS: usize = 0x9cc368;

/// The static paths of the preset folder (Load Preset, Save As Preset).
pub(crate) const TACTICS_PRESET_LOAD_PATH: usize = 0xde1d14;

pub(crate) const TACTICS_PRESET_SAVE_PATH: usize = 0xde1e2c;

/// "CM3_TACTICS", the empty string and the file extensions.
pub(crate) const TACTICS_FOLDER_KEY: usize = 0x9ccae4;

pub(crate) const TACTICS_NO_TEXT: usize = 0xad9c64;

pub(crate) const TACTICS_EXT_TCT: usize = 0xa8c3b0;

pub(crate) const TACTICS_EXT_PCT: usize = 0xa8c0c8;

/// The file dialog callbacks: a formation file and a preset file chosen.
pub(crate) const TACTICS_ON_FILE: usize = 0x5b87e0;

pub(crate) const TACTICS_ON_PRESET: usize = 0x5b88a0;

/// The screens the handler opens: Team Instructions (editable and read-only builders, its handler) and the player
/// instructions of a row (builder, handler).
pub(crate) const TACTICS_TEAM_INSTRUCTIONS_FN: usize = 0x8a2db0;

pub(crate) const TACTICS_TEAM_INSTRUCTIONS_RO_FN: usize = 0x8a4830;

pub(crate) const TACTICS_TEAM_INSTRUCTIONS_EVENT_FN: usize = 0x8a4840;

pub(crate) const TACTICS_PLAYER_INSTRUCTIONS_FN: usize = 0x8a5330;

pub(crate) const TACTICS_PLAYER_INSTRUCTIONS_EVENT_FN: usize = 0x8a6f80;

/// `Tactics_Team_Data::~Tactics_Team_Data` (0x899e50, tel14_1), called by address.
pub(crate) const TACTICS_TEAM_DATA_DESTRUCT_FN: usize = 0x899e50;

/// The element constructors and destructors of the Tactics_Screen's sub-objects: the match team blocks (0x625
/// bytes), the Formation and the match players (0x1be).
pub(crate) const MATCH_TEAM_CONSTRUCT_FN: usize = 0x453c30;

pub(crate) const FORMATION_CONSTRUCT_FN: usize = 0x5b27c0;

pub(crate) const MATCH_PLAYER_CONSTRUCT_FN: usize = 0x6eada0;

/// The default formation positions `predict_opponent_lineup` starts from (the 11 slots and the words after them, as
/// the original lays them on its stack).
pub(crate) const TACTICS_DEFAULT_LINEUP: [u16; 28] = [1, 0x84, 0x104, 0x404, 0x804, 0x90, 0x110, 0x410, 0x810, 0x140, 0x440,
                                            1, 4, 0x10, 0x40, 0x40, 4, 0x10, 0x40, 0x40, 4, 0x10, 0x40, 0x40, 4, 0x10, 0x40, 0x40];

/// The results of the dialogs' event handlers: the dialog closed (-9) and redraw (-0xb).
pub(crate) const TACTICS_DIALOG_CLOSED: i32 = -9;

pub(crate) const TACTICS_DIALOG_REDRAW: i32 = -0xb;

/// The team instruction rows: (heading, [(choice text, takes the empty argument, flag bit of the formation's team
/// instructions)]): Mentality, Passing, Tackling, Pressing, Offside Trap, Counter Attack and Men Behind Ball.
pub(crate) const TEAM_INSTRUCTION_ROWS: [(usize, &[(usize, bool, u32)]); 7] = [
    (0xa8c4c8, &[(0xa8c4a0, true, 0x20000), (0xa8c478, true, 0x8000), (0xa8c450, true, 0x10000)]),
    (0xa8bf4c, &[(0xa8bd7c, true, 1), (0xa8bd50, true, 2), (0xa8bd24, true, 4), (0xa8bcf8, true, 8)]),
    (0xa8bf40, &[(0xa8bcb8, true, 0x10), (0xa8bc98, true, 0x20), (0xa8bc78, true, 0x40)]),
    (0xa8bf34, &[(0x98e8cc, false, 0x80), (0x98e8d0, false, 0x100)]),
    (0xa8c440, &[(0x98e8cc, false, 0x200), (0x98e8d0, false, 0x400)]),
    (0xa8c430, &[(0x98e8cc, false, 0x800), (0x98e8d0, false, 0x1000)]),
    (0xa8c41c, &[(0x98e8cc, false, 0x2000), (0x98e8d0, false, 0x4000)]),
];

/// The set piece taker choices (`g_team_instr_row_*` at 0x97039c): (kind, choice, event id): free kicks left and
/// right, corners left and right, each first choice and "or" second choice.
pub(crate) const TEAM_TAKER_CHOICES: [(u32, u32, u32); 8] = [(0, 0, 4), (0, 1, 5), (1, 0, 6), (1, 1, 7), (2, 0, 8), (2, 1, 9), (3, 0, 0xa), (3, 1, 0xb)];

/// The player instruction rows: (heading, [(choice text, takes the empty argument, the bit is in the high dword of
/// the 64-bit player instructions, flag bit)]): Passing, Tackling, Pressing, Pass To, Set Pieces (A), Set Pieces (D),
/// Free Role, Forward Runs, Run With Ball, Hold Up Ball, Long Shots, Try Through Balls and Cross Ball. A low-dword
/// choice is event 4, a high-dword one event 7, the bit the value.
pub(crate) const PLAYER_INSTRUCTION_ROWS: [(usize, &[(usize, bool, bool, u32)]); 13] = [
    (0xa8bf4c, &[(0xa8bda8, true, false, 1), (0xa8bd7c, true, false, 2), (0xa8bd50, true, false, 4), (0xa8bd24, true, false, 8), (0xa8bcf8, true, false, 0x10)]),
    (0xa8bf40, &[(0xa8bcd8, true, false, 0x200), (0xa8bcb8, true, false, 0x400), (0xa8bc98, true, false, 0x800), (0xa8bc78, true, false, 0x1000)]),
    (0xa8bf34, &[(0xa8bc58, true, false, 0x2000), (0xa8bc3c, true, false, 0x4000), (0xa8bc1c, true, false, 0x8000)]),
    (0xa8bf28, &[(0xa8bbfc, true, false, 0x20), (0xa8bbdc, true, false, 0x40), (0xa8bbbc, true, false, 0x80), (0xa8bb9c, true, false, 0x100)]),
    (0xa8bf18, &[(0xa8bb5c, true, false, 0x10000), (0xa8bb24, true, false, 0x4000_0000), (0xa8bae4, true, false, 0x20000)]),
    (0xa8bf08, &[(0xa8baa4, true, false, 0x40000), (0xa8ba6c, true, false, 0x80000), (0xa8ba2c, true, false, 0x8000_0000)]),
    (0xa8befc, &[(0x98e8cc, false, false, 0x10_0000), (0x98e8d0, false, false, 0x20_0000)]),
    (0xa8beec, &[(0x98e8cc, false, false, 0x40_0000), (0x98e8d0, false, false, 0x80_0000)]),
    (0xa8bedc, &[(0x98e8cc, false, false, 0x100_0000), (0x98e8d0, false, false, 0x200_0000)]),
    (0xa8becc, &[(0x98e8cc, false, false, 0x400_0000), (0x98e8d0, false, false, 0x800_0000)]),
    (0xa8bec0, &[(0x98e8cc, false, false, 0x1000_0000), (0x98e8d0, false, false, 0x2000_0000)]),
    (0xa8beac, &[(0x98e8cc, false, true, 2), (0x98e8d0, false, true, 1)]),
    (0xa8bea0, &[(0x98e8cc, false, true, 8), (0x98e8d0, false, true, 4)]),
];

/// "\n\n", the paragraph break of the screen messages.
pub(crate) const TACTICS_PARAGRAPH: usize = 0x9ed918;

/// 22.0, the half width of the penalty area in pitch units (`g_f64_22_0`).
pub(crate) const PITCH_F64_22: usize = 0x96af40;
