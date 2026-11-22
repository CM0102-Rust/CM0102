// ---------- batch %%LEVA%%: the tactics screen (tactics_screens.cpp, 1): opening and freeing it ----------

/// The Tactics_Screen object (tactics_screens.cpp, 0xc4e2 bytes, built by `Tactics_Screen::Tactics_Screen`
/// 0x8a7440 from the club, the live match or a match record): the fields the screen routines use.
#[allow(dead_code)]
pub(crate) mod tactics_screen {
    pub(crate) const SIZE: u32 = 0xc4e2;
    /// Club*: the club whose tactics are shown.
    pub(crate) const CLUB: usize = 0x0;
    /// The live match engine (null outside a match).
    pub(crate) const MATCH: usize = 0x4;
    /// The match fixture [inference].
    pub(crate) const FIXTURE: usize = 0xc;
    /// u32: the screen edits the penalty takers (the penalty mode of `tactics_screen_open`).
    pub(crate) const PENALTY_MODE: usize = 0x14;
    /// The two match team blocks (2 x 0x625 bytes, destroyed with 0x453c10).
    pub(crate) const MATCH_TEAMS: usize = 0x137;
    /// Comp*: the competition of the match record (its background picture).
    pub(crate) const COMP: usize = 0xda6;
    /// Stadium*: the stadium of the match record (its background picture).
    pub(crate) const STADIUM: usize = 0xdaa;
    /// u8: the substitutes on the bench, low nibble (the rows past 11 + it are not in the match squad).
    pub(crate) const BENCH_SIZE: usize = 0xddf;
    /// u32: the screen shows a match record (bench size and pictures valid).
    pub(crate) const HAS_MATCH_RECORD: usize = 0xde1;
    /// i8: the side of the club in the match (0 home, 1 away).
    pub(crate) const SIDE: usize = 0xde9;
    /// u8: the kit the club plays in (1 home, 2 away, 3 third).
    pub(crate) const KIT: usize = 0xdeb;
    /// u32: read-only (a past match: no Tactics menu, no editing).
    pub(crate) const READ_ONLY: usize = 0xe40;
    /// The working team data: the formation (Formation object, name first) and its match players.
    pub(crate) const FORMATION: usize = 0xe44;
    /// u8: the formation is locked with a password.
    pub(crate) const LOCKED: usize = 0xe77;
    /// u32: the formation was saved since the last change.
    pub(crate) const SAVED: usize = 0x26c3;
    /// u32: the formation has a file to save to.
    pub(crate) const HAS_FILE: usize = 0x26cb;
    /// u8: the view (1 overview, 2 with the ball, 3 without the ball).
    pub(crate) const VIEW: usize = 0x2727;
    /// {i16 x 0..2, i16 y 0..3}: the ball zone the view shows.
    pub(crate) const ZONE: usize = 0x2729;
    /// {i16 x, i16 y}: the ball zone under the mouse [inference].
    pub(crate) const HOVER_ZONE: usize = 0x272d;
    /// u32: the runs are drawn.
    pub(crate) const SHOW_RUNS: usize = 0x2731;
    /// The match players of the working team data (`MATCH_PLAYERS` in a match).
    pub(crate) const MATCH_PLAYERS: usize = 0x2735;
    /// u8: the substitutions made.
    pub(crate) const SUBS_MADE: usize = 0x4a1f;
    /// i32: the captain's id.
    pub(crate) const CAPTAIN: usize = 0x4a20;
    /// i8[11]: the penalty takers, in order (row of each).
    pub(crate) const PENALTY_ORDER: usize = 0x4a54;
    /// The undo copy of the team data and the clipboard copy.
    pub(crate) const UNDO_DATA: usize = 0x4a63;
    pub(crate) const CLIPBOARD_DATA: usize = 0x8682;
    /// u8: positions were copied (Paste available).
    pub(crate) const CAN_PASTE: usize = 0xc4dd;
    /// u32: there is something to undo.
    pub(crate) const CAN_UNDO: usize = 0xc4de;
    /// Inside a team data block: its 20 match players (0x1be bytes each).
    pub(crate) const TEAM_PLAYERS: usize = 0x18f1;
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
const MATCH_PLAYER_DESTRUCT_FN: usize = 0x6ebd60;
const MATCH_TEAM_DESTRUCT_FN: usize = 0x453c10;

/// Builds a Tactics_Screen in `p` (`Tactics_Screen::Tactics_Screen`, 0x8a7440).
pub(crate) unsafe fn tactics_screen_construct(p: usize, club: usize, m: usize, reserve: u32, arg4: u32, record: u32, penalty: u32) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32) -> usize = core::mem::transmute(TACTICS_CONSTRUCT_FN);
    f(p, club, m, reserve, arg4, record, penalty)
}

/// Destroys a Tactics_Screen the way `tactics_screen_open` does inline when the screen does not open: the match
/// players of the clipboard, undo and working team data (20 records of 0x1be bytes each), the two match team blocks,
/// and the memory.
unsafe fn tactics_screen_free_inline(ts: usize) {
    use tactics_screen as t;
    exe::crt_vector_destructor_iterator(ts + t::CLIPBOARD_DATA + t::TEAM_PLAYERS, 0x1be, 0x14, MATCH_PLAYER_DESTRUCT_FN);
    exe::crt_vector_destructor_iterator(ts + t::UNDO_DATA + t::TEAM_PLAYERS, 0x1be, 0x14, MATCH_PLAYER_DESTRUCT_FN);
    exe::crt_vector_destructor_iterator(ts + t::FORMATION + t::TEAM_PLAYERS, 0x1be, 0x14, MATCH_PLAYER_DESTRUCT_FN);
    exe::crt_vector_destructor_iterator(ts + t::MATCH_TEAMS, 0x625, 2, MATCH_TEAM_DESTRUCT_FN);
    exe::crt_operator_delete(ts);
}

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

// simplification: allocates, opens a screen and writes its parameters: it does not run twice. Original only; the plan:
// when a new Tactics_Screen took parameter 0, parameters 1..10 are the arguments.
/// Comparer for `tactics_screen_open`.
pub(crate) unsafe extern "C" fn compare_tactics_screen_open(club: usize, reserve: u32, m: usize, arg4: u32, record: u32, penalty: u32) {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, usize, u32, u32, u32)>(%%I0%%);
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

// simplification: destroys records: it does not run twice. Original only.
/// Comparer for `tactics_team_data_destructor`.
pub(crate) unsafe extern "thiscall" fn compare_tactics_team_data_destructor(data: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(data)
}

// simplification: destroys and frees the object: it does not run twice. Original only; the plan: parameter 0 cleared.
/// Comparer for `tactics_screen_close_without_apply`.
pub(crate) unsafe extern "C" fn compare_tactics_screen_close_without_apply() {
    let had = get_screen_param(0) != 0;
    original_fn::<unsafe extern "C" fn()>(%%I2%%)();
    SCOREBOARD.record(!had || get_screen_param(0) == 0, || "tactics_screen_close_without_apply: parameter 0 not cleared".to_string());
}
