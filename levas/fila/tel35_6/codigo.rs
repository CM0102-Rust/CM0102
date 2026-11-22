// ---------- batch %%LEVA%%: the menu bar's object links (Menubar.cpp) ----------

/// The size of a club competition record (`G_CLUB_COMP_TABLE`) and of a cup round of a stage (+0xa3, the round id
/// word at +7).
const LINK_CLUB_COMP_SIZE: u32 = 0x6b;
const LINK_ROUND_SIZE: isize = 0x68;

/// `comp_screens_open_comp_screen(comp, stage, round, 0)` (0x48f4a0, cdecl).
unsafe fn link_open_comp(comp: usize, stage: i32, round: i32) {
    let f: unsafe extern "C" fn(usize, i32, i32, u32) = core::mem::transmute(0x48f4a0usize);
    f(comp, stage, round, 0);
}

/// `staff_screens_open_staff_profile(person, tab, kind, 0, extra)` (0x8702a0, cdecl).
unsafe fn link_open_profile(person: u32, tab: u32, kind: u32, extra: u32) {
    let f: unsafe extern "C" fn(u32, u32, u32, u32, u32) = core::mem::transmute(0x8702a0usize);
    f(person, tab, kind, 0, extra);
}

/// `open_club_squad_screen(club, reserves, 0, 0, 0)` (0x453d60, cdecl).
unsafe fn link_open_squad(club: u32, reserves: u32) {
    let f: unsafe extern "C" fn(u32, u32, u32, u32, u32) = core::mem::transmute(0x453d60usize);
    f(club, reserves, 0, 0, 0);
}

/// A thiscall reader of the current display or the screen manager taking the raw item (`Display::get_widget_id`
/// 0x468b10, `Display::get_widget_data` 0x4739a0, `ScreenManager::get_widget_action_id` 0x468b40,
/// `get_widget_action_data` 0x468b90).
unsafe fn link_widget(f: usize, this: usize, raw: u32) -> u32 {
    let g: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(f);
    g(this, raw)
}

/// The staff profile links read through the display's widget readers: (id, profile tab, kind).
const LINK_PROFILES: [(u16, u32, u32); 8] = [
    (0x7da, 3, 1), (0x7d9, 2, 1), (0x7db, 4, 1), (0x7dc, 1, 2), (0x7dd, 2, 2), (0x7df, 3, 2), (0x7de, 2, 2), (0x7e0, 4, 2),
];

/// Rule: a linked word on any screen (a competition, a stage, a club, a match, an award, a person, a fixture to watch, a
/// search) opens its screen; "Print" prints the screen first.
///
/// `menubar_handle_object_link(item)`, by the widget's id (`tel26_item_id`) and data (`tel26_item_value`): 0x426 prints
/// the screen (`ScreenManager::print_screen`) and goes on. 0x7d0 (a competition record): when the competition object
/// exists (`G_COMP_OBJECT_TABLE` of its id) and it is neither a reserve or minor competition nor a friendly's own
/// record, its screen opens (stage and round -2). 0x7e5 (a stage link: competition << 22 | stage << 11 | round): a club
/// competition with an object, not reserve or minor and not a friendly's own: the stage (`Comp::get_stage` from -1 to
/// +0x30) whose id (+0x3a) is the link's; none: the competition's screen (-2, -2); a cup stage (+0x43 = 1) with rounds
/// (+0xab) whose round id (+7 of each 0x68-byte round at +0xa3) is the link round's base (`comp_util_get_base_round_id`):
/// that stage and round; otherwise the stage (-2). 0x7e6: the club history (club, 0, 1). 0x7d1: the tournament
/// invitations (club, 0). 0x7d3: a match (-1 gives 0) found in the match stats (`Match_Stats::find_record_fixture`,
/// none gives 0) opens its details. 0x7d5: the club's squad, except a national team with a division whose nation
/// (+0x71) is not set, or a club with no division. 0x7d6: the reserves' squad. 0x7d4: the award screen. 0x7d7, 0x7e7,
/// 0x7d8: the person's profile (1, 1, 0), (1, 1, extra 2), (2, 1, 0). Through the display's readers, the profiles of
/// `LINK_PROFILES`. Through the screen manager's: 0x7d2 switches the watched fixture (`HUMAN_MANAGER::is_watching_fixture`
/// / `set_watched_fixture` with the fixture or 0) and gives -11 when it took; 0x7e4 opens the player search on the
/// data. Each screen opened gives -4; nothing else, 0.
/// Original: `menubar_handle_object_link` (0x7643d0, Menubar.cpp, cdecl).
pub(crate) unsafe extern "C" fn menubar_handle_object_link(raw: u32) -> i32 {
    let item = raw as i16;
    if tel26_item_id(item) == 0x426 {
        let print: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x803bf0usize);
        print(G_SCREEN_MANAGER);
    }
    if tel26_item_id(item) == 0x7d0 {
        let record = tel26_item_value(item) as usize;
        if read_usize(read_usize(G_COMP_OBJECT_TABLE) + read_u32(record) as usize * 4) != 0
            && exe::comp_is_reserve_or_minor_comp(read_usize(G_RESERVE_COMP), record) == 0
            && exe::friendly_is_own_comp_record(read_usize(G_FRIENDLY), record) == 0 {
            covered!("tel35_6 link competition");
            link_open_comp(record, -2, -2);
            return -4;
        }
    }
    if tel26_item_id(item) == 0x7e5 {
        let link = tel26_item_value(item);
        let comp = (link >> 0x16) as i32;
        let stage_id = ((link as i32 >> 0xb) & 0x7ff) as u16;
        let round = link & 0x7ff;
        if comp >= 0 && comp < read_i32(G_CLUB_COMP_COUNT) {
            let object = read_usize(read_usize(G_COMP_OBJECT_TABLE) + comp as usize * 4);
            if object != 0 && exe::comp_is_reserve_or_minor_comp(read_usize(G_RESERVE_COMP), read_usize(object + 4)) == 0 {
                let offset = (comp as u32).wrapping_mul(LINK_CLUB_COMP_SIZE) as usize;
                if exe::friendly_is_own_comp_record(read_usize(G_FRIENDLY), read_usize(G_CLUB_COMP_TABLE) + offset) == 0 {
                    covered!("tel35_6 link stage");
                    let mut s = -1i32;
                    while s <= read_i32(object + 0x30) {
                        let stage = exe::comp_get_stage(object, s);
                        if read_u16(stage + 0x3a) == stage_id {
                            if read_u8(stage + 0x43) == 1 {
                                let mut j = 0i32;
                                while j < read_i8(stage + 0xab) as i32 {
                                    let base = exe::comp_util_get_base_round_id(round) as u16;
                                    if read_u16((read_usize(stage + 0xa3) as isize + 7 + j as isize * LINK_ROUND_SIZE) as usize) == base {
                                        link_open_comp(read_usize(G_CLUB_COMP_TABLE) + offset, s, j);
                                        return -4;
                                    }
                                    j += 1;
                                }
                            }
                            link_open_comp(read_usize(G_CLUB_COMP_TABLE) + offset, s, -2);
                            return -4;
                        }
                        s += 1;
                    }
                    link_open_comp(read_usize(G_CLUB_COMP_TABLE) + offset, -2, -2);
                    return -4;
                }
            }
        }
    }
    match tel26_item_id(item) {
        0x7e6 => {
            let f: unsafe extern "C" fn(u32, u32, u32) = core::mem::transmute(0x469e70usize);
            f(tel26_item_value(item), 0, 1);
            return -4;
        }
        0x7d1 => {
            let f: unsafe extern "C" fn(u32, u32) = core::mem::transmute(0x4739d0usize);
            f(tel26_item_value(item), 0);
            return -4;
        }
        0x7d3 => {
            let fixture = tel26_item_value(item);
            if item != -1 && fixture == u32::MAX {
                return 0;
            }
            let mut record = [0u8; 0x50];
            if exe::match_stats_find_record_fixture(read_usize(G_MATCH_STATS), fixture as usize, record.as_mut_ptr() as usize) == 0 {
                return 0;
            }
            let f: unsafe extern "C" fn(u32) = core::mem::transmute(0x71b4a0usize);
            f(fixture);
            return -4;
        }
        _ => {}
    }
    if tel26_item_id(item) == 0x7d5 {
        let club = tel26_item_value(item) as usize;
        let skip = (exe::database_is_national_team(club) != 0 && read_usize(club + 0x53) != 0 && read_usize(read_usize(club + 0x53) + 0x71) == 0)
            || (exe::database_is_national_team(club) == 0 && read_usize(club + 0x53) == 0);
        if !skip {
            link_open_squad(club as u32, 0);
            return -4;
        }
    }
    match tel26_item_id(item) {
        0x7d6 => {
            link_open_squad(tel26_item_value(item), 1);
            return -4;
        }
        0x7d4 => {
            tel35_cdecl1(0x414c20, tel26_item_value(item) as usize);
            return -4;
        }
        0x7d7 => {
            link_open_profile(tel26_item_value(item), 1, 1, 0);
            return -4;
        }
        0x7e7 => {
            link_open_profile(tel26_item_value(item), 1, 1, 2);
            return -4;
        }
        0x7d8 => {
            link_open_profile(tel26_item_value(item), 2, 1, 0);
            return -4;
        }
        _ => {}
    }
    for (id, tab, kind) in LINK_PROFILES {
        let mut w = link_widget(0x468b10, current_human_display(), raw) as u16;
        if w == 0 {
            w = read_u16(G_CURRENT_COMMAND_ID);
        }
        if w == id {
            covered!("tel35_6 link profile tab");
            link_open_profile(link_widget(0x4739a0, current_human_display(), raw), tab, kind, 0);
            return -4;
        }
    }
    if link_widget(0x468b40, G_SCREEN_MANAGER, raw) == 0x7d2 {
        let fixture = link_widget(0x468b90, G_SCREEN_MANAGER, raw) as usize;
        let watching = exe::human_manager_is_watching_fixture(read_usize(G_HUMAN_MANAGER), fixture, 0usize);
        let set: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x5fe500usize);
        if set(read_usize(G_HUMAN_MANAGER), if watching != 0 { 0 } else { fixture }, 0) != 0 {
            covered!("tel35_6 link watch fixture");
            return -11;
        }
    }
    if link_widget(0x468b40, G_SCREEN_MANAGER, raw) == 0x7e4 {
        tel35_cdecl1(0x808540, link_widget(0x468b90, G_SCREEN_MANAGER, raw) as usize);
        return -4;
    }
    0
}

/// Comparer for `menubar_handle_object_link`.
pub(crate) unsafe extern "C" fn compare_menubar_handle_object_link(item: u32) -> i32 {
    // simplification: opening the linked screen (and printing) does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I0%%)(item)
}
