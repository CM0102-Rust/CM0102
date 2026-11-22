// ---------- batch %%LEVA%%: the club screens (2): opening the club screen and building it, with the transfers tab (club_screens.cpp) ----------

/// club_screens.cpp's source path (0x98e598 "E:\dev\CM3\cm3 00-01\cm3\code\club_screens.cpp"), for the error boxes.
const CLUB_SCREENS_CPP_PATH: usize = 0x98e598;

/// The "Error" box of club_screens.cpp at line `row` ("v%s %s.%s %d": the version, the file's name and extension
/// split by `exe::crt_strtok`, which is the C runtime's _splitpath, and the line), then the mouse button released
/// (`G_MOUSE_LEFT_BUTTON_DOWN` = 0).
unsafe fn club_screens_error_box(row: u32) {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let version: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
    let (mut name, mut ext) = ([0u8; 256], [0u8; 256]);
    exe::crt_strtok(CLUB_SCREENS_CPP_PATH, 0, 0, name.as_mut_ptr(), ext.as_mut_ptr());
    let v = version(name.as_mut_ptr(), ext.as_mut_ptr(), row);
    format(0xae24d0, 0x9870e8, v, name.as_ptr(), ext.as_ptr(), row);
    exe::gui_show_message_box(0x9870e0, 0xae24d0, 0);
    write_u32(G_MOUSE_LEFT_BUTTON_DOWN, 0);
}

/// `HUMAN_MANAGER::get_squad_view_settings(human)` (0x5fe950, thiscall, ret 4): the 15-byte block where the human
/// keeps the squad screen's view (0 = the current human).
unsafe fn club_screens_squad_view_settings() -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5fe950usize);
    f(read_usize(0xb63c98), 0)
}

/// The club screen's parameters (0..0x22, 0 = not written) that `open_manager_club_screen` gives the new screen: the
/// plan, worked out before the screen opens (it reads the current screen's parameters to restore them).
unsafe fn open_manager_club_screen_plan(manager: Staff, squad_param: u32, restore_params: u32, open_tab_5: u32) -> [u32; 0x23] {
    let rec = exe::get_human_manager_record(manager.addr());
    let club = if rec != 0 && read_u32(rec + human_manager_sheet::LIVE_MATCH) != 0 {
        read_usize(rec + human_manager_sheet::CLUB)
    } else if read_usize(manager.addr() + staff::CLUB_JOB) != 0 {
        read_usize(manager.addr() + staff::CLUB_JOB)
    } else {
        read_usize(manager.addr() + staff::NATIONAL_JOB)
    };
    let mut saved = [0u32; 60];
    if restore_params != 0 {
        for (k, s) in saved.iter_mut().enumerate() {
            *s = get_screen_param(k as u32) as u32;
        }
    }
    // the default view when the human does not run the club (the original's local block)
    let mut default_view = [0u8; 16];
    default_view[0..4].copy_from_slice(&0x41u32.to_le_bytes());
    default_view[4..8].copy_from_slice(&8u32.to_le_bytes());
    default_view[8..12].copy_from_slice(&0x154a21u32.to_le_bytes());
    default_view[12] = 6;
    default_view[13] = 0xff;
    default_view[14] = 0xff;
    let view = if club != 0 && exe::human_manager_current_controls_club(club, 0, 1) != 0 {
        club_screens_squad_view_settings()
    } else {
        default_view.as_ptr() as usize
    };
    let year = read_u16(G_CURRENT_DATE + 2) as u32;
    let mut p = [0u32; 0x23];
    p[0] = club as u32;
    p[1] = squad_param;
    p[4] = if open_tab_5 != 0 { 5 } else { 1 };
    p[7] = read_u32(view);
    p[8] = read_u32(view + 4);
    p[9] = read_i8(view + 0xc) as i32 as u32;
    p[0xc] = year;
    p[0xf] = 1;
    p[0x12] = 1;
    p[0x17] = year;
    p[0x18] = read_u32(view + 8);
    p[0x19] = read_i8(view + 0xd) as i32 as u32;
    p[0x1a] = read_i8(view + 0xe) as i32 as u32;
    p[0x1b] = if open_tab_5 != 0 { 2 } else { 1 };
    if restore_params != 0 {
        for k in [4usize, 7, 8, 9, 0x10, 0x11, 0x12, 0x18, 0x19, 0x1a, 0x1b] {
            p[k] = saved[k];
        }
    }
    // the three players of the squad with the best reputation (world for a national team, otherwise current), by
    // strict comparison of the signed words, starting from -10000
    let (mut best, mut second, mut third) = (0usize, 0usize, 0usize);
    let (mut s1, mut s2, mut s3) = (-10000i16, -10000i16, -10000i16);
    let mut count = 0u8;
    if club != 0 {
        for i in 0..50 {
            let pl = read_usize(club + club::SQUAD + i * 4);
            if pl == 0 || read_usize(pl + staff::PLAYER) == 0 {
                continue;
            }
            let sheet = read_usize(pl + staff::PLAYER);
            let score = if exe::database_is_national_team(club) != 0 {
                read_i16(sheet + player_sheet::WORLD_REPUTATION)
            } else {
                read_i16(sheet + player_sheet::CURRENT_REPUTATION)
            };
            if count < 3 {
                count += 1;
            }
            if score > s1 {
                third = second;
                second = best;
                s3 = s2;
                s2 = s1;
                s1 = score;
                best = pl;
            } else if score > s2 {
                s3 = s2;
                third = second;
                s2 = score;
                second = pl;
            } else if score > s3 {
                s3 = score;
                third = pl;
            }
        }
    }
    p[0x20] = if count > 0 { read_u32(best) } else { u32::MAX };
    p[0x21] = if count > 1 { read_u32(second) } else { u32::MAX };
    p[0x22] = if count > 2 { read_u32(third) } else { u32::MAX };
    p
}

/// Rule: opening a human manager's club screen shows the club he is managing in a live match, otherwise his club, otherwise
/// his national team, on the squad tab (or the tactics tab), with the view he left the squad screen in, and marks his
/// three best known players.
///
/// `open_manager_club_screen(manager, squad_param, restore_params, open_tab_5, set_flag_0x800)`. No manager: the error
/// box (line 0x32b) and nothing else. The club: the manager's record (`exe::get_human_manager_record`) with a live
/// match (`human_manager_sheet::LIVE_MATCH`) gives `human_manager_sheet::CLUB`; otherwise `staff::CLUB_JOB`, otherwise
/// `staff::NATIONAL_JOB`. With `restore_params`, the 60 parameters of the current screen are read first. The view: the
/// human's squad view block (`HUMAN_MANAGER::get_squad_view_settings`, 0x5fe950) when he runs the club
/// (`exe::human_manager_current_controls_club`(club, 0, 1)), otherwise the default block (0x41, 8, 0x154a21, 6, -1,
/// -1). The screen opens for the manager (`exe::screenmanager_open_screen_for_each_record` with the build 0x454900,
/// the input 0x4671f0 and the leave 0x469da0; when it does not open, nothing else) and gets, for the manager
/// (`exe::screenmanager_set_screen_param_for_each_record`), its parameters 0..0x1f: 0 the club, 1 `squad_param`, 4 the
/// tab (5 with `open_tab_5`, otherwise 1), 7 and 8 the view's first two dwords, 9 its byte +0xc, 0xc and 0x17 the
/// current year, 0xf and 0x12 one, 0x18 the view's third dword, 0x19 and 0x1a its bytes +0xd and +0xe (sign-extended),
/// 0x1b 2 with `open_tab_5` (otherwise 1), all others 0; with `restore_params`, 4, 7, 8, 9, 0x10..0x12 and 0x18..0x1b
/// come back from the old screen. With `set_flag_0x800`, the current screen's parameter 7 gets the view's first dword
/// with bit 0x800 (`exe::screenmanager_set_screen_param`). Last, parameters 0x20..0x22: the ids of the three players
/// of the squad (`club::SQUAD`, those with a player record) with the best reputation, by strict comparison
/// (`player_sheet::WORLD_REPUTATION` for a national team, `exe::database_is_national_team`, otherwise
/// `player_sheet::CURRENT_REPUTATION`; signed words from -10000); -1 for the places not filled.
/// Original: `open_manager_club_screen` (0x4542f0, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn open_manager_club_screen(manager: Staff, squad_param: u32, restore_params: u32, open_tab_5: u32, set_flag_0x800: u32) {
    if manager.is_null() {
        club_screens_error_box(0x32b);
        return;
    }
    covered!("tel01_2 open the club screen");
    let p = open_manager_club_screen_plan(manager, squad_param, restore_params, open_tab_5);
    let sm = G_SCREEN_MANAGER;
    // the view block is read again for the 0x800 flag (the plan has already read it)
    let club = p[0] as usize;
    let view_first = if club != 0 && exe::human_manager_current_controls_club(club, 0, 1) != 0 {
        read_u32(club_screens_squad_view_settings())
    } else {
        0x41
    };
    if exe::screenmanager_open_screen_for_each_record(sm, manager.addr() as u32, 0x454900, 0x4671f0, 0, 0x469da0, 0) == 0 {
        return;
    }
    let set = |k: u32| exe::screenmanager_set_screen_param_for_each_record(sm, manager.addr() as u32, k, p[k as usize] as usize, 0);
    for k in 0..0x20 {
        set(k);
    }
    if restore_params != 0 {
        for k in [4u32, 7, 8, 9, 0x10, 0x11, 0x12, 0x18, 0x19, 0x1a, 0x1b] {
            set(k);
        }
    }
    if set_flag_0x800 != 0 {
        exe::screenmanager_set_screen_param(sm, 7, (view_first | 0x800) as usize, 0);
    }
    for k in 0x20..0x23 {
        set(k);
    }
}

/// Comparer for `open_manager_club_screen`.
///
/// Opening cannot run twice: only the original runs, and then the parameters of the manager's new screen (his channel,
/// `exe::screenmanager_find_manager_channel`, parameter n at +0x14 + 8n of the block `screen_manager::PARAM_BLOCK`) are
/// checked against the plan worked out before (`open_manager_club_screen_plan`). Without a manager, or when the screen
/// did not open (parameter 0 is not the planned club), nothing is checked.
pub(crate) unsafe extern "C" fn compare_open_manager_club_screen(manager: Staff, squad_param: u32, restore_params: u32, open_tab_5: u32, set_flag_0x800: u32) {
    let o = original_fn::<unsafe extern "C" fn(Staff, u32, u32, u32, u32)>(%%I0%%);
    if manager.is_null() {
        return o(manager, squad_param, restore_params, open_tab_5, set_flag_0x800);
    }
    let plan = crate::rng::without_counting(|| open_manager_club_screen_plan(manager, squad_param, restore_params, open_tab_5));
    o(manager, squad_param, restore_params, open_tab_5, set_flag_0x800);
    let slot = exe::screenmanager_find_manager_channel(G_SCREEN_MANAGER, manager.addr()) as i16;
    if !(0..64).contains(&slot) {
        return;
    }
    let block = read_usize(G_SCREEN_MANAGER + screen_manager::PARAM_BLOCK + slot as usize * 0x300);
    if block == 0 || read_u32(block + 0x14) != plan[0] {
        return;
    }
    let differ: Vec<usize> = (0..0x23).filter(|&k| k != 7 || set_flag_0x800 == 0).filter(|&k| read_u32(block + 0x14 + 8 * k) != plan[k]).collect();
    SCOREBOARD.record(differ.is_empty(), || format!("open_manager_club_screen (club screen): parameters {:x?} differ from the plan", differ));
}

/// `Date::Date(this, day, year)` (0x549ef0, thiscall, ret 8): a date from the day of the year and the year.
unsafe fn club_screens_date(out: &mut [u8; 8], day: u32, year: u32) -> usize {
    let f: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE);
    f(out.as_mut_ptr() as usize, day, year)
}

/// `LLIST::LLIST` (0x68d020) and `LLIST::~LLIST` (0x68d040), thiscall: the screen's local fixture list.
unsafe fn club_screens_llist(l: &mut [u8; 0x20], construct: bool) {
    let f: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(if construct { FN_LLIST_LLIST } else { FN_LLIST_DESTRUCTOR_LLIST });
    f(l.as_mut_ptr() as usize);
}

/// A copy (operator new, `fixture::SIZE` bytes) of the fixture `f`, for the screen parameters 2 and 3.
unsafe fn club_screens_copy_fixture(f: usize) -> usize {
    let c = exe::crt_operator_new(fixture::SIZE as u32);
    core::ptr::copy_nonoverlapping(f as *const u8, c as *mut u8, fixture::SIZE);
    c
}

/// A tab of the club screen's tab bar (`gui_add_tab_bar`, 0xd4 bytes each: +2 the colour, +4 the text, +0xcc the kind,
/// +0xd0 the link).
const CLUB_TAB_SIZE: usize = 0xd4;

/// Rule: the club screen shows the club's name (or its penalty takers), its tabs (Squad, Transfers, Next Match,
/// Fixtures, Finances & Info or General Info, Tactics, Training, Last Match, the division and History; the transfers
/// and training of a club the human does not run are greyed out), the chosen tab, the squad menu, the action button
/// or the print menu, and the back and next buttons; it remembers the last and the next fixture of the club.
///
/// `club_screen_build()`, from the screen parameters 0..7 (`get_screen_param`); [0x98ded8] = 1 first. No club (param 0):
/// the current human's (`screen_manager::DISPLAY` slot's human at +0x7d6: `staff::CLUB_JOB`, otherwise
/// `staff::NATIONAL_JOB`) written to param 0; still none: the error box (line 0x416) and nothing else. Param 1 (the
/// reserves) set when the human does not run the club and it is not a national team goes back to 0. Param 2 empty (the
/// last match): the club's fixtures from 8 weeks back (24 for a national team; `exe::date_sub_weeks`, the year of a first
/// result and the day of a second, as the original) to today (`exe::fixture_manager_find_fixtures` with the club's id,
/// the local list, -1, -2 and "no reserves") and, from the end of the list (`exe::llist_get_prev`), the first one with
/// the byte +0x43 ≥ 0 (played), copied (`fixture::SIZE` bytes) into param 2. Param 3 empty (the next match): the same
/// with the fixtures from today to 8 (24) weeks ahead (`exe::date_add_weeks`), appended to the same list, and from its
/// start (`exe::llist_get_next`) the first with +0x43 = -1. Params 5 and 6 both empty: the club's kit colours
/// (`exe::database_get_club_kit_colours`, kit 1) written there. The club's background picture
/// (`exe::background_set_club_picture`). Bit 0x800 of param 7 (the penalty takers) dropped and written back when the
/// human does not run the club. The title: the club's name (`exe::club_get_national_team_name`, style 3) or, with bit
/// 0x800 on the squad tab of a club he runs, "{}<%s>{} Penalty Takers" with the short name (style 1), in the colours of
/// params 6 and 5 (`exe::screenmanager_add_screen_title`). The menu bar (`exe::menubar_build` 4). The ten tabs
/// (text, kind, link): "Squad" 2/1; "Transfers" 2/2 (for a national team greyed 0/0, and the transfers tab becomes the
/// squad tab, param 4 = 1); "Next Match" 2/3; "Fixtures" 2/4; "Finances & Info" (a club he runs that is not a national
/// team) or "General Info" 2/5; "Tactics" 3/0; "Training" 4/0 (greyed 0/0 unless he runs the club and it is not a
/// national team); "Last Match" 5/0; the division: with the club's current league record
/// (`club_history_get_current_league_record`, 0x444190) "<pos> in {}<division>{}" (the ordinal of position + 1,
/// `exe::utils_format_ordinal`) when the record's byte +0xb is positive, otherwise the division's name (its short name
/// when the name has more than 20 characters), kind 0x7e5 with the link (division id << 11 | stage +0x3a & 0x7ff) << 11
/// when there is a stage, otherwise 0x7d0/0; without a record "Division" greyed 0/0; "History" 6/0. The tab bar
/// (`exe::gui_add_tab_bar`, 10 tabs, from 0x50 to 0x221, split), then the tab of param 4: 1 the squad (0x456900), 2 the
/// transfers (`club_screen_draw_transfers_tab`), 3 the next match (0x45ccf0), 4 the fixtures (0x45f380), 5 the info
/// (0x460590); another: the error box (line 0x505). Then the squad menu (`build_club_squad_navigation_menu`, 0x481ef0)
/// and the action button (`add_club_screen_action_button`, 0x482a40); when there is no action button, the "Print"
/// button (its colour 0xae3184 when param 6 is the colour 0xaebdea) with its popup of "Squad" 1, "Stats" 2,
/// "Transfers" 4 (a disabled item for a national team), "Fixtures" 8 and "All" 0xf. Last the back and next buttons
/// (`exe::gui_add_back_next_buttons`).
/// Original: `club_screen_build` (0x454900, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn club_screen_build() {
    let sm = G_SCREEN_MANAGER;
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let mut list = [0u8; 0x20];
    club_screens_llist(&mut list, true);
    let mut tabs = [0u8; CLUB_TAB_SIZE * 0x19];
    for k in 0..0x19 {
        tabs[k * CLUB_TAB_SIZE..k * CLUB_TAB_SIZE + 2].copy_from_slice(&0xffffu16.to_le_bytes());
        tabs[k * CLUB_TAB_SIZE + 2..k * CLUB_TAB_SIZE + 4].copy_from_slice(&0u16.to_le_bytes());
    }
    let tab_at = |tabs: &mut [u8; CLUB_TAB_SIZE * 0x19], k: usize| tabs.as_mut_ptr() as usize + k * CLUB_TAB_SIZE;
    let mut stage = 0usize;
    write_u32(0x98ded8, 1);
    let mut club = get_screen_param(0);
    let mut reserves = get_screen_param(1);
    let last_match = get_screen_param(2);
    let next_match = get_screen_param(3);
    let mut tab = get_screen_param(4) as u8;
    let mut colour_text = get_screen_param(5) as u32;
    let mut colour = get_screen_param(6) as u32;
    let mut view = get_screen_param(7) as u32;
    if club == 0 {
        let human = read_usize(sm + 0x7d6 + read_i16(sm + screen_manager::CURRENT_SLOT) as isize as usize * 0x300);
        if human != 0 {
            club = read_usize(human + staff::CLUB_JOB);
            if club == 0 {
                club = read_usize(human + staff::NATIONAL_JOB);
            }
            set_screen_param(0, club);
        }
        if club == 0 {
            club_screens_error_box(0x416);
            club_screens_llist(&mut list, false);
            return;
        }
    }
    covered!("tel01_2 club screen");
    if reserves != 0 && exe::human_manager_current_controls_club(club, 0, 1) == 0 && exe::database_is_national_team(club) == 0 {
        set_screen_param(1, 0);
        reserves = 0;
    }
    let (mut d1, mut d2) = ([0u8; 8], [0u8; 8]);
    // simplification: the high halves of the date words pushed to find_fixtures are register garbage in the original
    // (find_fixtures keeps the words); here zero
    if last_match == 0 {
        let weeks = if exe::database_is_national_team(club) != 0 { 0x18 } else { 8 };
        let a = exe::date_sub_weeks(G_CURRENT_DATE, &mut d1, weeks);
        let year = read_u16(a + 2) as u32;
        let b = exe::date_sub_weeks(G_CURRENT_DATE, &mut d2, weeks);
        let day = read_u16(b) as u32;
        exe::fixture_manager_find_fixtures(read_usize(0xae2a58), day, year, read_u16(G_CURRENT_DATE) as u32, read_u16(G_CURRENT_DATE + 2) as u32,
            read_u32(club), list.as_mut_ptr() as usize, u32::MAX, (-2i32) as u32, (reserves == 0) as u32);
        let l = list.as_mut_ptr() as usize;
        if read_u16(l) != 0 {
            write_u32(l + 0xa, read_u32(l + 6));
            let mut f = exe::llist_get_prev(l);
            while f != 0 {
                if read_i8(f + 0x43) > -1 {
                    covered!("tel01_2 last match");
                    set_screen_param(2, club_screens_copy_fixture(f));
                    break;
                }
                f = exe::llist_get_prev(l);
            }
        }
    }
    if next_match == 0 {
        let weeks = if exe::database_is_national_team(club) != 0 { 0x18 } else { 8 };
        let a = exe::date_add_weeks(G_CURRENT_DATE, &mut d1, weeks);
        let year = read_u16(a + 2) as u32;
        let b = exe::date_add_weeks(G_CURRENT_DATE, &mut d2, weeks);
        let day = read_u16(b) as u32;
        exe::fixture_manager_find_fixtures(read_usize(0xae2a58), read_u16(G_CURRENT_DATE) as u32, read_u16(G_CURRENT_DATE + 2) as u32, day, year,
            read_u32(club), list.as_mut_ptr() as usize, u32::MAX, (-2i32) as u32, (reserves == 0) as u32);
        let l = list.as_mut_ptr() as usize;
        if read_u16(l) != 0 {
            write_u32(l + 0xa, read_u32(l + 2));
            let mut f = exe::llist_get_next(l);
            while f != 0 {
                if read_i8(f + 0x43) == -1 {
                    covered!("tel01_2 next match");
                    set_screen_param(3, club_screens_copy_fixture(f));
                    break;
                }
                f = exe::llist_get_next(l);
            }
        }
    }
    if colour_text as u16 == 0 && colour as u16 == 0 {
        exe::database_get_club_kit_colours(club, 1, &mut colour_text, &mut colour);
        set_screen_param(5, (colour_text & 0xffff) as usize);
        set_screen_param(6, (colour & 0xffff) as usize);
    }
    exe::background_set_club_picture(club, 0, 0);
    if view & 0x800 != 0 && exe::human_manager_current_controls_club(club, 0, 1) == 0 {
        view -= 0x800;
        set_screen_param(7, view as usize);
    }
    let (mut name, mut short_name) = ([0u8; 0x12c], [0u8; 0x12c]);
    exe::club_get_national_team_name(club, reserves, name.as_mut_ptr() as usize, 0x12c, 3, 1);
    exe::club_get_national_team_name(club, reserves, short_name.as_mut_ptr() as usize, 0x12c, 1, 1);
    if view & 0x800 != 0 && tab == 1 && exe::human_manager_current_controls_club(club, 0, 1) != 0 {
        let g = read_i8(club + club::SHORT_GENDER_NAME) as i32;
        text(0xde1f64, 0x98e894, g, g, short_name.as_ptr());
    } else {
        copy_text(0xde1f64 as *mut u8, name.as_ptr());
    }
    exe::screenmanager_add_screen_title(sm, colour, colour_text, 0xde1f64usize);
    exe::menubar_build(4, 0);
    let set_tab = |tabs: &mut [u8; CLUB_TAB_SIZE * 0x19], k: usize, kind: u32, link: u32| {
        write_u32(tab_at(tabs, k) + 0xcc, kind);
        write_u32(tab_at(tabs, k) + 0xd0, link);
    };
    let grey = |tabs: &mut [u8; CLUB_TAB_SIZE * 0x19], k: usize| write_u16(tab_at(tabs, k) + 2, read_u16(0xae317e));
    text(tab_at(&mut tabs, 0) + 4, 0x98e88c); // "Squad"
    set_tab(&mut tabs, 0, 2, 1);
    text(tab_at(&mut tabs, 1) + 4, 0x98e880); // "Transfers"
    if exe::database_is_national_team(club) == 0 {
        set_tab(&mut tabs, 1, 2, 2);
    } else {
        if tab == 2 {
            set_screen_param(4, 1);
            tab = 1;
        }
        grey(&mut tabs, 1);
        set_tab(&mut tabs, 1, 0, 0);
    }
    text(tab_at(&mut tabs, 2) + 4, 0x98e874); // "Next Match"
    set_tab(&mut tabs, 2, 2, 3);
    text(tab_at(&mut tabs, 3) + 4, 0x98e868); // "Fixtures"
    set_tab(&mut tabs, 3, 2, 4);
    if exe::human_manager_current_controls_club(club, 0, 1) != 0 && exe::database_is_national_team(club) == 0 {
        text(tab_at(&mut tabs, 4) + 4, 0x98e858); // "Finances & Info"
    } else {
        text(tab_at(&mut tabs, 4) + 4, 0x98e848); // "General Info"
    }
    set_tab(&mut tabs, 4, 2, 5);
    text(tab_at(&mut tabs, 5) + 4, 0x98e840); // "Tactics"
    set_tab(&mut tabs, 5, 3, 0);
    text(tab_at(&mut tabs, 6) + 4, 0x98e834); // "Training"
    if exe::human_manager_current_controls_club(club, 0, 1) != 0 && exe::database_is_national_team(club) == 0 {
        write_u32(tab_at(&mut tabs, 6) + 0xcc, 4);
    } else {
        grey(&mut tabs, 6);
        write_u32(tab_at(&mut tabs, 6) + 0xcc, 0);
    }
    write_u32(tab_at(&mut tabs, 6) + 0xd0, 0);
    text(tab_at(&mut tabs, 7) + 4, 0x98e828); // "Last Match"
    set_tab(&mut tabs, 7, 5, 0);
    let mut record = [0u8; 0x20];
    let mut complete = 0i32;
    let league_record: unsafe extern "C" fn(usize, *mut u8, *mut i32, *mut usize, u32) -> u32 = core::mem::transmute(0x444190usize);
    if league_record(club, record.as_mut_ptr(), &mut complete, &mut stage, 1) != 0 {
        let comp = read_usize(G_CLUB_COMP_TABLE) + read_u32(record.as_ptr() as usize) as usize * 0x6b;
        let division = tab_at(&mut tabs, 8) + 4;
        if (record[0xb] as i8) > 0 {
            let g = read_i8(comp + 0x52) as i32;
            let ordinal = exe::utils_format_ordinal(read_i8(record.as_ptr() as usize + 6) as i32 + 1, -1);
            text(division, 0x98e800, g, g, ordinal, comp + 0x53);
        } else {
            copy_text(division as *mut u8, (comp + 0x38) as *const u8);
            if club_screens_text_length(division as *const u8) > 0x14 {
                copy_text(division as *mut u8, (comp + 0x53) as *const u8);
            }
        }
        if stage != 0 {
            let link = ((read_u32(comp) << 0xb) | (read_u16(stage + 0x3a) as u32 & 0x7ff)) << 0xb;
            set_tab(&mut tabs, 8, 0x7e5, link);
        } else {
            set_tab(&mut tabs, 8, 0x7d0, 0);
        }
    } else {
        text(tab_at(&mut tabs, 8) + 4, 0x98e7f4); // "Division"
        grey(&mut tabs, 8);
        set_tab(&mut tabs, 8, 0, 0);
    }
    text(tab_at(&mut tabs, 9) + 4, 0x98e7c8, 0xad9c64usize); // "History<..>"
    set_tab(&mut tabs, 9, 6, 0);
    let (mut top, mut bottom) = (0x50i32, 0x221i32);
    exe::gui_add_tab_bar(tabs.as_mut_ptr() as usize, 0xa, tab as i8 as i32, &mut top, &mut bottom, 1);
    let draw = |f: usize, top: i32, bottom: i32| {
        let g: unsafe extern "C" fn(i32, i32) = core::mem::transmute(f);
        g(top, bottom)
    };
    match tab as i8 {
        1 => draw(0x456900, top, bottom),
        2 => club_screen_draw_transfers_tab(top, bottom),
        3 => draw(0x45ccf0, top, bottom),
        4 => draw(0x45f380, top, bottom),
        5 => draw(0x460590, top, bottom),
        _ => club_screens_error_box(0x505),
    }
    let menu: unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x481ef0usize);
    menu(club, colour, colour_text, 1, 0x21, 0x1f, 1, reserves as u32);
    let action: unsafe extern "C" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x482a40usize);
    if action(club, colour, colour_text) == 0 {
        covered!("tel01_2 print menu");
        let button: ManagerBoxFn = core::mem::transmute(FN_SCREENMANAGER_ADD_BUTTON_ITEM);
        let add_area: DisplayAddAreaFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
        let add_item: DisplayAddItemFn = core::mem::transmute(FN_DISPLAY_ADD_ITEM);
        text(0xde1f64, 0x98e5f4); // "Print"
        let mut c = read_u16(0xaebdea);
        if colour as u16 == c {
            c = read_u16(0xae3184);
        }
        let b = button(sm, 0x2c6, 0xf, 0x311, 0x23, 0x30, colour as usize, 0xc, 1, c as usize, 0xde1f64, 0, 0);
        let area = add_area(screens_current_display(sm), 0, 0, 0, 0, 0, 0, 0, 0, 0x130, read_u16(0xae31aa) as u32, b as u32);
        let item = |label: usize, data: u32| {
            text(0xde1f64, label);
            add_item(screens_current_display(sm), 2, 0, 0, 0, 0, 0, 0, 0x10, read_u16(0xae31aa) as u32, read_u16(0xaebdec) as u32, 1, 1,
                read_u16(0xaebdea) as u32, 0xde1f64, 0, 0x22, data, area);
        };
        item(0x98e88c, 1); // "Squad"
        item(0x98e5ec, 2); // "Stats"
        if exe::database_is_national_team(club) == 0 {
            item(0x98e880, 4); // "Transfers"
        } else {
            text(0xde1f64, 0x98e880);
            add_item(screens_current_display(sm), 2, 0, 0, 0, 0, 0, 0, 0x10, read_u16(0xae31aa) as u32, read_u16(0xaebdec) as u32, 0x21, 1,
                read_u16(0xaebdea) as u32, 0xde1f64, 0, 0, 0, area);
        }
        item(0x98e868, 8); // "Fixtures"
        text(0xde1f64, 0x98e5c8, 0xad9c64usize); // "All<..>"
        add_item(screens_current_display(sm), 2, 0, 0, 0, 0, 0, 0, 0x10, read_u16(0xae31aa) as u32, read_u16(0xaebdec) as u32, 1, 1,
            read_u16(0xaebdea) as u32, 0xde1f64, 0, 0x22, 0xf, area);
    }
    let forward = (exe::screenmanager_is_forward_unavailable(sm) == 0) as u32;
    let back = (exe::screenmanager_is_back_unavailable(sm, 0) == 0) as u32;
    exe::gui_add_back_next_buttons(back, forward);
    club_screens_llist(&mut list, false);
}

/// The length of a zero-ended text (strlen).
unsafe fn club_screens_text_length(p: *const u8) -> usize {
    core::ffi::CStr::from_ptr(p as *const core::ffi::c_char).to_bytes().len()
}

/// The club record of id `id` in the club table ([0xae23b4] + `club::SIZE` × id).
unsafe fn club_screens_club(id: i32) -> usize {
    read_usize(0xae23b4).wrapping_add((id as usize).wrapping_mul(club::SIZE))
}

/// The person record of id `id` in the person table ([0xae23bc] + `staff::SIZE` × id).
unsafe fn club_screens_staff(id: i32) -> usize {
    read_usize(0xae23bc).wrapping_add((id as usize).wrapping_mul(staff::SIZE))
}

/// `Cash::format_display_amount(this, out, size, flags)` (0x450aa0, thiscall, ret 0xc) on a local amount.
unsafe fn club_screens_format_cash(amount: i32, out: usize, size: u32, flags: u32) {
    let f: unsafe extern "thiscall" fn(*const i32, usize, u32, u32) -> u32 = core::mem::transmute(0x450aa0usize);
    f(&amount, out, size, flags);
}

/// `Date::get_short_date_string(this, out, size, time_of_day)` (0x54a650, thiscall, ret 0xc).
unsafe fn club_screens_short_date(date: usize, out: usize, size: u32, time_of_day: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x54a650usize);
    f(date, out, size, time_of_day);
}

/// A transfer line's kind word when it has no fee: "Exch" (1), "Loan" (2, only `with_loan`), "Bos" (4), else "Free".
unsafe fn club_screens_free_kind(kind: u8, with_loan: bool) -> usize {
    match kind {
        1 => 0x98e620,
        2 if with_loan => 0x98e5fc,
        4 => 0x98e61c,
        _ => 0x98e614,
    }
}

/// Rule: the club's transfers tab lists, for the season shown, the players in and out, the future transfers, the loans
/// or the staff moves (as the View menu picks), each with its date, the person, the other club and the fee, and the
/// balance of the fees at the bottom.
///
/// The transfers tab of `club_screen_build` (inline in the original, 0x455273..0x45633d), between `top` and `bottom`.
/// The parameters: 0 the club, 0x12 the view flags (1 players in, 2 players out, 4 future transfers, 0x10 loans in,
/// 0x20 loans out, 0x40 staff in, 0x80 staff out), 0x15 and 0x16 the past transfers array and its count (kept between
/// builds), 0x17 the season's year, 0x13 the scroll row. With any view but the future transfers (flags & 0xf3), the season
/// arrows ("<<" button 0x1b when the year is above [0xdd6eca], ">>" button 0x1c when below the current year; the current
/// year one less when the club's key nation, `exe::key_nation_index_of_club`, has its season start +0x15 ahead of today
/// and +0x17 set; otherwise plain texts). The "View" button with its popup (`screenmanager_add_popup_area`) of the eight
/// views, the chosen ones ticked (`screenmanager_add_ticked_popup_item`, the text of the last ticked one kept as the
/// title) and the others plain (`ScreenManager::add_menu_item`, 0x415640). The season: from the key nation's start day
/// (+0x15, or day 0 without +0x17) of the year to the day before it a year later; in 2001 from day 1. The title "<view>
/// - Season <season>" (`key_nation_format_club_season`, 0x669630) when a view is chosen, else the view text; the rows that
/// fit in font 3. Future transfers (`exe::transfer_manager_get_club_future_transfer_list`, 0x1b bytes each, freed at the
/// end): those of a club the human runs or with the other club run by a human (or stage +0x19 = 6, or +0x1a set); the
/// stage (Offer, Negotiation, Contract, Work Permit, Confirmation, Delayed, else the date), the player, "to"/"from" the
/// other club (or "from Free Transfer"), the fee (with an exchange "<cash> p/x"; without fee "Exch", "Bos" or "Free"), the
/// balance (an outgoing fee is added once per player, an incoming one subtracted; a negative balance with "-"). Past
/// transfers (`TRANSFER_MANAGER::get_past_transfers`, 0x8cc030, by the months and years of the season's ends, into params 0x15
/// and 0x16): those of the season's dates
/// passing the view's filters (0x51: the club is +0x10; 0xa2: +0x14; 0x33: kind +0x1c not 5; 0xc0: kind 5; 0x30: kind 2;
/// 3: kind not 2), sorted (qsort with 0x8e5540), each with its date, the name (a button when the person still matches the
/// names, `exe::database_staff_matches_names`), the other club (+0x14 with 0x51, otherwise +0x10; "Free Transfer"
/// without), the fee or the kind word ("Loan" too), and the sum of the fees. The list area's scroll row from param 0x13
/// (`screenmanager_set_area_scroll_row`) and the area written to param 0x14.
unsafe fn club_screen_draw_transfers_tab(top: i32, bottom: i32) {
    let sm = G_SCREEN_MANAGER;
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let area_button: ListCellFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_BUTTON);
    let area_text: ListCellTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
    let button: ManagerBoxFn = core::mem::transmute(FN_SCREENMANAGER_ADD_BUTTON_ITEM);
    let manager_text: ManagerTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_TEXT_ITEM);
    let add_area: DisplayAddAreaFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    let w = |a: usize| read_u16(a) as usize;
    let weights = [5u8, 0xd, 0xa, 5];
    let (mut start, mut end) = ([0u8; 8], [0u8; 8]);
    let date_default: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    date_default(start.as_mut_ptr() as usize);
    date_default(end.as_mut_ptr() as usize);
    let mut future: usize = 0;
    let mut total: i32 = 0;
    let club = get_screen_param(0);
    let flags = get_screen_param(0x12) as u32;
    let mut past = get_screen_param(0x15);
    let mut past_count = get_screen_param(0x16) as i32;
    let mut season_year = get_screen_param(0x17) as u32;
    let knr = 0xb63d60 + exe::key_nation_index_of_club(club) as i8 as isize as usize * 0x48;
    let mut title = [0u8; 0xc8];
    let mut current_year: u32;
    let mut ebp = top;
    if flags & 0xf3 != 0 {
        covered!("tel01_2 transfers season arrows");
        let arrows = add_area(screens_current_display(sm), 0x6e, top as u32, 0x168, (top + 0x14) as u32, 2, 0, 1, 0, 1, 0, u32::MAX);
        current_year = read_u16(G_CURRENT_DATE + 2) as u32;
        if read_u32(knr + 0x17) != 0 && (read_i16(G_CURRENT_DATE) as i32) < read_u16(knr + 0x15) as i32 {
            season_year = season_year.wrapping_add(0xffff);
            current_year = current_year.wrapping_add(0xffff);
        }
        text(0xde1f64, 0x98e7ac, 0x98e7c4usize); // "<<"
        if season_year as u16 > read_u16(0xdd6eca) {
            area_button(sm, arrows as usize, 0, 0, 0x30, w(0xae317e), 0xc, 1, w(0xae3184), 0xde1f64, 0x1b, 0);
        } else {
            area_text(sm, arrows as usize, 0, 0, 0x30, w(0xae317e), 0x2c, 1, w(0xae3184), 0xde1f64);
        }
        text(0xde1f64, 0x98e790, 0x98e7a8usize); // ">>"
        if (season_year as u16) < current_year as u16 {
            area_button(sm, arrows as usize, 1, 0, 0x30, w(0xae317e), 0xc, 1, w(0xae3184), 0xde1f64, 0x1c, 0);
        } else {
            area_text(sm, arrows as usize, 1, 0, 0x30, w(0xae317e), 0x2c, 1, w(0xae3184), 0xde1f64);
        }
    }
    text(0xde1f64, 0x988004); // "View"
    let b = button(sm, 0x28f, top as usize, 0x30c, (top + 0x14) as usize, 0x30, w(0xae317e), 0xc, 1, w(0xae3184), 0xde1f64, 0, 0);
    let popup: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x456850usize);
    let pa = popup(sm, b as u32);
    let ticked: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x456890usize);
    let plain: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x415640usize);
    for (label, bit) in [(0x98e784usize, 1u32), (0x98e774, 2), (0x98e760, 4), (0x98e754, 0x10), (0x98e748, 0x20), (0x98e73c, 0x40), (0x98e730, 0x80)] {
        text(0xde1f64, label);
        if flags & bit != 0 {
            ticked(sm, pa, 0xde1f64, 0x1a, bit);
            copy_text(title.as_mut_ptr(), 0xde1f64 as *const u8);
        } else {
            plain(sm, pa, 0xde1f64, 0x1a, bit);
        }
    }
    let mut tmp = [0u8; 8];
    if read_u32(knr + 0x17) != 0 {
        club_screens_date(&mut tmp, read_u16(knr + 0x15) as u32, season_year);
    } else {
        club_screens_date(&mut tmp, 0, season_year);
    }
    start = tmp;
    let (lo, hi) = (u32::from_le_bytes([start[0], start[1], start[2], start[3]]), u32::from_le_bytes([start[2], start[3], start[4], start[5]]));
    let a = club_screens_date(&mut tmp, lo, hi.wrapping_add(1));
    exe::date_sub_days(a, end.as_mut_ptr() as usize, 1);
    // the dword that later carries the row count holds the first dword of the season's last date (in 2001, of day 1)
    let mut rows_word = u32::from_le_bytes([end[0], end[1], end[2], end[3]]);
    if season_year as u16 == 0x7d1 {
        club_screens_date(&mut tmp, 1, 0x7d1);
        start = tmp;
        rows_word = u32::from_le_bytes([start[0], start[1], start[2], start[3]]);
    }
    let mut season = [0u8; 0xc8];
    let season_label: unsafe extern "C" fn(u32, usize, *mut u8, u32) -> i32 = core::mem::transmute(0x669630usize);
    season_label(season_year, club, season.as_mut_ptr(), 0xc8);
    if flags & 0xf3 != 0 {
        text(0xde1f64, 0x98e6f0, title.as_ptr(), season.as_ptr());
    } else {
        copy_text(0xde1f64 as *mut u8, title.as_ptr());
    }
    ebp += 0x19;
    manager_text(sm, 0x6e, ebp as usize, 0x30c, (ebp + 0x23) as usize, 2, 0, 0xc, 3, w(0xaebdec), 0xde1f64, 1);
    ebp += 0x28;
    // the row count is a byte written over the low byte of that dword, as the original
    let rows = (rows_word & 0xffff_ff00) | ((bottom - ebp - 0x10) / exe::font_get_height(3)) as u8 as u32;
    let area;
    if flags & 4 != 0 {
        covered!("tel01_2 future transfers");
        let list: unsafe extern "thiscall" fn(usize, usize, *mut usize) -> i32 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_CLUB_FUTURE_TRANSFER_LIST);
        let count = list(read_usize(G_TRANSFER_MANAGER), club, &mut future);
        let add: DisplayAddAreaFn = core::mem::transmute(0x4567f0usize);
        area = add(sm, 0x6e, ebp as u32, 0x30c, bottom as u32, 4, weights.as_ptr() as u32, rows, 0, 2, 0, u32::MAX);
        let mut row = 0usize;
        let mut k = 0i32;
        while k < count {
            let rec = future + k as usize * 0x1b;
            let mut shown = read_u8(rec + 0x1a) != 0 || read_u8(rec + 0x19) == 6 || exe::human_manager_current_controls_club(club, 0, 1) != 0;
            if !shown {
                let other = read_i32(rec + 0xc);
                shown = other != -1 && exe::human_manager_current_controls_club(club_screens_club(other), 0, 1) != 0;
            }
            if !shown {
                k += 1;
                continue;
            }
            match read_u8(rec + 0x19) {
                0 => { text(0xde1f64, 0x98e6e8); } // "Offer"
                1 => { text(0xde1f64, 0x98e6dc); } // "Negotiation"
                2 => { text(0xde1f64, 0x98e6d0); } // "Contract"
                3 => { text(0xde1f64, 0x98e6c4); } // "Work Permit"
                4 => { text(0xde1f64, 0x98e6b4); } // "Confirmation"
                5 => { text(0xde1f64, 0x98e6ac); } // "Delayed"
                _ => club_screens_short_date(rec, 0xde1f64, 0x7d0, 0xff),
            }
            area_text(sm, area as usize, 0, row, 0x30, w(0xaebe04), 0xc, 1, w(0xae3184), 0xde1f64);
            let person = club_screens_staff(read_i32(rec + 8));
            exe::database_format_staff_name(person, title.as_mut_ptr() as usize, 0xc8, 1);
            format(0xde1f64, 0x987ff0, title.as_ptr());
            area_button(sm, area as usize, 1, row, 1, 0, 1, 3, w(0xae3184), 0xde1f64, 0x7db, person);
            let other = read_i32(rec + 0xc);
            if read_u32(rec + 0x10) != 0 {
                let c = club_screens_club(other);
                let g = read_i8(c + club::SHORT_GENDER_NAME) as i32;
                text(0xde1f64, 0x98e684, g, g, c + club::SHORT_NAME);
                // the fee counts once per player (an earlier line with the same person does not add it again)
                let seen = (0..k).any(|j| read_i32(future + j as usize * 0x1b + 8) == read_i32(rec + 8));
                if !seen {
                    total = total.wrapping_add(read_i32(rec + 0x14));
                }
            } else {
                if other == -1 {
                    text(0xde1f64, 0x98e670); // "from Free Transfer"
                } else {
                    let c = club_screens_club(other);
                    let g = read_i8(c + club::SHORT_GENDER_NAME) as i32;
                    text(0xde1f64, 0x98e644, g, g, c + club::SHORT_NAME);
                }
                total = total.wrapping_sub(read_i32(rec + 0x14));
            }
            if other == -1 {
                area_text(sm, area as usize, 2, row, 1, 0, 1, 2, w(0xaebdec), 0xde1f64);
            } else {
                area_button(sm, area as usize, 2, row, 1, 0, 1, 2, w(0xaebdec), 0xde1f64, 0x7d5, club_screens_club(other));
            }
            let fee = read_i32(rec + 0x14);
            if fee != 0 && read_u8(rec + 0x18) == 1 {
                let amount = fee;
                let v = exe::cash_get_display_amount(&amount as *const i32 as *const u32, 5);
                exe::cash_format_money(title.as_mut_ptr() as usize, 0xc8, v, 5);
                text(0xde1f64, 0x98e628, title.as_ptr());
            } else if fee != 0 {
                club_screens_format_cash(fee, 0xde1f64, 0x7d0, 5);
            } else {
                text(0xde1f64, club_screens_free_kind(read_u8(rec + 0x18), false));
            }
            area_text(sm, area as usize, 3, row, 0x30, w(0xae3192), 0xc, 1, w(0xae3184), 0xde1f64);
            row += 1;
            k += 1;
        }
        if total < 0 {
            format(0xde1f64, 0x98ba0c); // "-"
            club_screens_format_cash(total.wrapping_neg(), 0xde1f65, 0x7d0, 5);
        } else {
            club_screens_format_cash(total, 0xde1f64, 0x7d0, 5);
        }
        area_text(sm, area as usize, 3, row, 0x30, w(0xae3192), 0xc, 1, w(0xae3152), 0xde1f64);
        if future != 0 {
            exe::crt_free(future);
        }
    } else {
        covered!("tel01_2 past transfers");
        let get_past: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, *mut usize, *mut i32) -> u32 = core::mem::transmute(0x8cc030usize);
        let end_year = u32::from_le_bytes([end[2], end[3], end[4], end[5]]);
        let start_year = u32::from_le_bytes([start[2], start[3], start[4], start[5]]);
        // the month index (0..11, or -1 when out of the table) of each end of the season, sign-extended as the original's eax
        let e = exe::date_get_month(end.as_ptr() as usize) as i8 as i32 as u32;
        let s = exe::date_get_month(start.as_ptr() as usize) as i8 as i32 as u32;
        get_past(read_usize(G_TRANSFER_MANAGER), s, start_year, e, end_year, &mut past, &mut past_count);
        set_screen_param(0x15, past);
        set_screen_param(0x16, past_count as u32 as usize);
        let add: DisplayAddAreaFn = core::mem::transmute(0x4567f0usize);
        area = add(sm, 0x6e, ebp as u32, 0x30c, bottom as u32, 4, weights.as_ptr() as u32, rows, 0, 2, 0, u32::MAX);
        let mut picked: usize = 0;
        if past_count != 0 {
            picked = exe::crt_malloc((past_count as u32).wrapping_mul(4));
        }
        let mut n = 0u32;
        let id = read_u32(club);
        let start_lo = u32::from_le_bytes([start[0], start[1], start[2], start[3]]);
        let start_hi = u32::from_le_bytes([start[4], start[5], start[6], start[7]]);
        let end_lo = u32::from_le_bytes([end[0], end[1], end[2], end[3]]);
        let end_hi = u32::from_le_bytes([end[4], end[5], end[6], end[7]]);
        let mut i = 0i32;
        while i < past_count {
            let rec = past + i as usize * 0x25;
            i += 1;
            let kind = read_u8(rec + 0x1c);
            if flags & 0x51 != 0 && read_u32(rec + 0x10) != id {
                continue;
            }
            if flags & 0xa2 != 0 && read_u32(rec + 0x14) != id {
                continue;
            }
            if (flags & 0x33 != 0 && kind == 5) || (flags & 0xc0 != 0 && kind != 5) || (flags & 0x30 != 0 && kind != 2) || (flags & 3 != 0 && kind == 2) {
                continue;
            }
            if exe::date_operator_3c(rec + 0x1d, start_lo, start_hi) != 0 || exe::date_operator_3e(rec + 0x1d, end_lo, end_hi) != 0 {
                continue;
            }
            write_usize(picked + n as usize * 4, rec);
            n += 1;
        }
        exe::crt_qsort(picked, n, 4, 0x8e5540usize);
        let mut row = 0usize;
        for k in 0..n as usize {
            let rec = read_usize(picked + k * 4);
            club_screens_short_date(rec + 0x1d, 0xde1f64, 0x7d0, 0xff);
            area_text(sm, area as usize, 0, row, 0x30, w(0xaebe04), 0xc, 1, w(0xae3184), 0xde1f64);
            exe::database_format_name_from_ids(read_u32(rec + 4), read_u32(rec + 8), read_u32(rec + 0xc), title.as_mut_ptr(), 0xc8, 1, read_u32(rec));
            format(0xde1f64, 0x987ff0, title.as_ptr());
            let person = club_screens_staff(read_i32(rec));
            if exe::database_staff_matches_names(person, read_u32(rec + 4), read_u32(rec + 8), read_u32(rec + 0xc), 0usize) != 0 {
                area_button(sm, area as usize, 1, row, 1, 0, 1, 3, w(0xae3184), 0xde1f64, 0x7d7, person);
            } else {
                area_text(sm, area as usize, 1, row, 1, 0, 1, 3, w(0xae3184), 0xde1f64);
            }
            let other = if flags & 0x51 != 0 { read_i32(rec + 0x14) } else { read_i32(rec + 0x10) };
            total = total.wrapping_add(read_i32(rec + 0x18));
            if other > -1 {
                let c = club_screens_club(other);
                copy_text(0xde1f64 as *mut u8, (c + club::SHORT_NAME) as *const u8);
                area_button(sm, area as usize, 2, row, 1, 0, 1, 2, w(0xaebdec), 0xde1f64, 0x7d5, c);
            } else {
                text(0xde1f64, 0x98e604); // "Free Transfer"
                area_text(sm, area as usize, 2, row, 1, 0, 1, 2, w(0xaebdec), 0xde1f64);
            }
            let fee = read_i32(rec + 0x18);
            if fee != 0 && read_u8(rec + 0x1c) == 1 {
                club_screens_format_cash(fee, title.as_mut_ptr() as usize, 0xc8, 5);
                text(0xde1f64, 0x98e628, title.as_ptr());
            } else if fee != 0 {
                club_screens_format_cash(fee, 0xde1f64, 0x7d0, 5);
            } else {
                text(0xde1f64, club_screens_free_kind(read_u8(rec + 0x1c), true));
            }
            area_text(sm, area as usize, 3, row, 0x30, w(0xae3192), 0xc, 1, w(0xae3184), 0xde1f64);
            row += 1;
        }
        club_screens_format_cash(total, 0xde1f64, 0x7d0, 5);
        area_text(sm, area as usize, 3, row, 0x30, w(0xae3192), 0xc, 1, w(0xae3152), 0xde1f64);
        if picked != 0 {
            exe::crt_free(picked);
        }
    }
    let scroll: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(0x4567a0usize);
    scroll(sm, area, get_screen_param(0x13) as u32);
    set_screen_param(0x14, area as i16 as i32 as u32 as usize);
}

/// Comparer for `club_screen_build`.
///
/// Through the display list (`compare_by_display_list_with`: the screen manager, the current `Display`, the parameter
/// block, 0xde1f64, the network position, the generator), plus [0x98ded8] and the mouse flag. Only the original runs
/// when the build allocates or reallocates what it keeps: no club yet (param 0), no last or next match copy yet (params
/// 2 and 3), the fixtures tab (4, which allocates), the past transfers (tab 2 without the future view: the array of
/// params 0x15 and 0x16 grows) and an unknown tab (the error box).
pub(crate) unsafe extern "C" fn compare_club_screen_build() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I1%%);
    let tab = get_screen_param(4) as u8;
    if get_screen_param(0) == 0 || get_screen_param(2) == 0 || get_screen_param(3) == 0 || !matches!(tab, 1 | 2 | 3 | 5)
        || (tab == 2 && get_screen_param(0x12) & 4 == 0) {
        // simplification: allocating and reallocating do not run twice; these cases are proven in Rust mode
        return o();
    }
    compare_by_display_list_with("club_screen_build (club screen)", &[(0x98ded8, 4), (G_MOUSE_LEFT_BUTTON_DOWN, 4)], || o(), || club_screen_build());
}
