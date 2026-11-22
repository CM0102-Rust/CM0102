// ---------- batch %%LEVA%%: opening the staff profile and the compare players screens (staff_screens.cpp, part 1) ----------

/// Opens the staff profile screen of `staff` on tab `tab` and writes its 22 params.
///
/// With no staff: the error box (staff_screens.cpp line 0x149). The screen opens when the person is viewable: someone
/// who is not a manager (`staff::JOB_FOR_CLUB` 2) and is outside the 16 last staff records (id below
/// `G_STAFF_COUNT` − 16 or not below the count); a player sheet (`staff::PLAYER`) with an id below the count; or,
/// with any id below the count, someone who is neither 1 nor 2 in `staff::JOB_FOR_CLUB`. Otherwise nothing.
///
/// With `keep_view`, params 0..59 of the current screen are kept first. `exe::screenmanager_open_screen` with the
/// builder 0x870700, the handler 0x888260 and the close handler 0x88b8a0; then the params: 0 the person, 1 the tab (5
/// for one of the 16 last records), 2 `slot2` (both sign-extended bytes), 3 zero, 4 and 5: 1 and 1 for a non-player
/// sheet (`staff::NON_PLAYER`) below the count that has no player sheet, or whose job is not 0xb, 0xc, 0xd or 0xf, or
/// with `attribute_mode` 1; otherwise `attribute_mode` and 2; 6..0xe zero, 0xf and 0x10 one, 0x11 the blue colour,
/// 0x12 the pale grey colour, 0x13 zero, 0x14 and 0x15 one. With `keep_view`, params 1, 2, 5 and 0x13 come back from
/// the old screen, and 4 becomes 2 when it was 2.
/// Original: `staff_screens_open_staff_profile` (0x8702a0, staff_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn staff_screens_open_staff_profile(staff: usize, tab: u32, slot2: u32, keep_view: u32, attribute_mode: u32) {
    if staff == 0 {
        covered!("tel07_1 staff profile without a person");
        return show_error_at(0xa84aac, 0x149);
    }
    let job = read_u8(staff + staff::JOB_FOR_CLUB);
    let n = read_i32(G_STAFF_COUNT);
    let id = read_i32(staff);
    let last_records = id >= n.wrapping_sub(16) && id < n; // tel07_1 the 16 last staff records
    let viewable = (job != 2 && !last_records) || (read_usize(staff + staff::PLAYER) != 0 && id < n) || (job != 1 && job != 2 && id < n);
    if !viewable {
        covered!("tel07_1 staff profile not viewable");
        return;
    }
    let mut kept = [0usize; 0x3c];
    if keep_view != 0 {
        for (k, v) in kept.iter_mut().enumerate() {
            *v = get_screen_param(k as u32);
        }
    }
    let tab = if last_records { 5 } else { tab as u8 };
    if exe::screenmanager_open_screen(SCREEN_MANAGER, 0x870700, 0x888260, 0, 0x88b8a0, 0) == 0 {
        return;
    }
    covered!("tel07_1 open the staff profile");
    set_screen_param(0, staff);
    set_screen_param(1, tab as i8 as isize as usize);
    set_screen_param(2, slot2 as u8 as i8 as isize as usize);
    set_screen_param(3, 0);
    let mode = attribute_mode as u8;
    let id = read_i32(staff);
    let job = read_u8(staff + staff::JOB_FOR_CLUB);
    let full_view = id < read_i32(G_STAFF_COUNT)
        && read_usize(staff + staff::NON_PLAYER) != 0
        && (read_usize(staff + staff::PLAYER) == 0 || !matches!(job, 0xb | 0xf | 0xd | 0xc) || mode == 1);
    if full_view {
        set_screen_param(4, 1);
        set_screen_param(5, 1);
    } else {
        set_screen_param(4, mode as i8 as isize as usize);
        set_screen_param(5, 2);
    }
    for k in 6..=0xe {
        set_screen_param(k, 0);
    }
    set_screen_param(0xf, 1);
    set_screen_param(0x10, 1); // tel07_1 param 0x10
    set_screen_param(0x11, colour_word(G_GUI_COLOUR_BLUE));
    set_screen_param(0x12, colour_word(G_GUI_COLOUR_PALE_GREY));
    set_screen_param(0x13, 0);
    set_screen_param(0x14, 1);
    set_screen_param(0x15, 1);
    if keep_view != 0 {
        covered!("tel07_1 staff profile keeps the view");
        for k in [1usize, 2, 5, 0x13] {
            set_screen_param(k as u32, kept[k]);
        }
        if kept[4] == 2 {
            set_screen_param(4, 2);
        }
    }
}

/// Opens the compare players screen.
///
/// With no first compare player (`SCOUT_MANAGER::get_compare_player` 1 of `G_SCOUT_MANAGER`) and no `manager`: the
/// error box (staff_screens.cpp line 0x1ac). Otherwise `exe::screenmanager_open_screen` with the builder 0x876520, the
/// handler 0x889c30 and the close handler 0x88b8a0 (no params). The callers push five arguments; only the first is
/// read.
/// Original: `staff_screens_open_compare_players_screen` (0x870650, staff_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn staff_screens_open_compare_players_screen(manager: usize) {
    let first: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(0x7f7920usize);
    if first(read_usize(G_SCOUT_MANAGER), 1) == 0 && manager == 0 {
        covered!("tel07_1 compare players without players");
        return show_error_at(0xa84aac, 0x1ac);
    }
    covered!("tel07_1 open the compare players");
    exe::screenmanager_open_screen(SCREEN_MANAGER, 0x876520, 0x889c30, 0, 0x88b8a0, 0);
}

/// The params the staff profile screen of `staff` should get (read before it opens: with `keep_view` the current
/// screen's params come back), or `None` when the person is not viewable. Written apart from the routine, from the
/// same disassembly, to check the original afterwards.
unsafe fn staff_profile_planned_params(staff: usize, tab: u32, slot2: u32, keep_view: u32, attribute_mode: u32) -> Option<[usize; 0x16]> {
    let job = read_u8(staff + staff::JOB_FOR_CLUB);
    let n = read_i32(G_STAFF_COUNT);
    let id = read_i32(staff);
    let last = id >= n.wrapping_sub(16) && id < n;
    let player = read_usize(staff + staff::PLAYER) != 0;
    if !((job != 2 && !last) || (player && id < n) || (job != 1 && job != 2 && id < n)) {
        return None;
    }
    let mut p = [0usize; 0x16];
    p[0] = staff;
    p[1] = if last { 5 } else { tab as u8 as i8 as isize as usize };
    p[2] = slot2 as u8 as i8 as isize as usize;
    let mode = attribute_mode as u8;
    if id < n && read_usize(staff + staff::NON_PLAYER) != 0 && (!player || !matches!(job, 0xb | 0xc | 0xd | 0xf) || mode == 1) {
        p[4] = 1;
        p[5] = 1;
    } else {
        p[4] = mode as i8 as isize as usize;
        p[5] = 2;
    }
    p[0xf] = 1;
    p[0x10] = 1;
    p[0x11] = colour_word(G_GUI_COLOUR_BLUE);
    p[0x12] = colour_word(G_GUI_COLOUR_PALE_GREY);
    p[0x14] = 1;
    p[0x15] = 1;
    if keep_view != 0 {
        for k in [1u32, 2, 5, 0x13] {
            p[k as usize] = get_screen_param(k);
        }
        if get_screen_param(4) == 2 {
            p[4] = 2;
        }
    }
    Some(p)
}

/// Comparer for `staff_screens_open_staff_profile`: opening a screen does not run twice (the screen stack, the
/// network), so only the original runs and its 22 params are checked afterwards against the plan
/// (`staff_profile_planned_params`), when the new screen holds the person in param 0.
pub(crate) unsafe extern "C" fn compare_staff_screens_open_staff_profile(staff: usize, tab: u32, slot2: u32, keep_view: u32, attribute_mode: u32) {
    let plan = if staff == 0 { None } else { staff_profile_planned_params(staff, tab, slot2, keep_view, attribute_mode) };
    original_fn::<unsafe extern "C" fn(usize, u32, u32, u32, u32)>(%%I0%%)(staff, tab, slot2, keep_view, attribute_mode);
    if let Some(plan) = plan {
        if get_screen_param(0) == staff {
            let mut got = [0usize; 0x16];
            for (k, v) in got.iter_mut().enumerate() {
                *v = get_screen_param(k as u32);
            }
            SCOREBOARD.record(got == plan, || format!("staff profile screen (open {:#x}): params {:x?} x plan {:x?}", staff, got, plan));
        }
    }
}

/// Comparer for `staff_screens_open_compare_players_screen`: opening a screen does not run twice; only the original.
pub(crate) unsafe extern "C" fn compare_staff_screens_open_compare_players_screen(manager: usize) {
    original_fn::<unsafe extern "C" fn(usize)>(%%I1%%)(manager)
}
