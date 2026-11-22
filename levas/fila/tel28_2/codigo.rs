// ---------- batch %%LEVA%%: the national squad screens (national_teams_screens.cpp): moving a player between squads, confirming a squad selection and a call-up ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\national_teams_screens.cpp", the path the national_teams_screens.cpp error boxes
/// print.
pub(crate) const NATIONAL_TEAMS_SCREENS_CPP: usize = 0xa645e4;
/// The No/Yes area's column weights (3 and 1).
const NATIONAL_SQUAD_YES_NO_WEIGHTS: [u8; 2] = [3, 1];
/// The national teams' per-nation record (110 bytes) in the vector at +4 of the national teams object.
const NATIONAL_SQUAD_RECORD_SIZE: usize = 0x6e;
/// The return value of the national squad dialogs once answered (back to the previous screen) [inference].
const NATIONAL_SQUAD_DONE: i32 = -9;

/// The national team the current screen slot's human manages (`staff::NATIONAL_JOB` of `tel26_slot_manager`): 0 with
/// no manager or no national job, after the national_teams_screens.cpp box of `line_manager` or `line_club`.
unsafe fn national_squad_human_team(line_manager: u32, line_club: u32) -> (Staff, usize) {
    let manager = tel26_slot_manager();
    if manager.is_null() {
        tel26_error_box(NATIONAL_TEAMS_SCREENS_CPP, line_manager);
        return (manager, 0);
    }
    let club = read_usize(manager.addr() + staff::NATIONAL_JOB);
    if club == 0 {
        tel26_error_box(NATIONAL_TEAMS_SCREENS_CPP, line_club);
    }
    (manager, club)
}

/// Whether the national squad screens' human team is missing (no manager or no national job): those runs show an
/// error box and run the original only.
unsafe fn national_squad_has_box() -> bool {
    let manager = tel26_slot_manager();
    manager.is_null() || read_usize(manager.addr() + staff::NATIONAL_JOB) == 0
}

/// The No (id 1) and Yes (id 2) buttons of a national squad dialog, in a two-column area of the current Display.
unsafe fn national_squad_yes_no() {
    let weights = NATIONAL_SQUAD_YES_NO_WEIGHTS;
    let area = tel26_add_area(current_human_display(), [0x64, 0x22b, 0x316, 0x24e, 2, weights.as_ptr() as u32, 1, 0, 1, 0, u32::MAX]);
    tel28_lang()(JOB_TEXT, 0x98e8cc); // "No"
    tel26_area_button([area, 0, 0, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 3, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 1, 0]);
    tel28_lang()(JOB_TEXT, 0x98e8d0); // "Yes"
    tel26_area_button([area, 1, 0, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 3, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 2, 0]); // tel28_2 Yes
}

/// The message of a national squad dialog: the text in [`JOB_TEXT`] as a pale grey text area (font 4).
unsafe fn national_squad_message() {
    tel26_text_area([0x6e, 0x91, 0x30c, 0x217, 2, 0, 0xf, 4, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 0]);
}

/// The "Ok" button of a national squad dialog that cannot go on (id 1).
unsafe fn national_squad_ok() {
    tel28_lang()(JOB_TEXT, 0x990f7c); // "Ok"
    tel26_add_button([0x6e, 0x22b, 0x30c, 0x24e, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 3, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 1, 0]);
}

/// The title of a national squad dialog: the club's picture, the menu bar (4) and `name` in the club's home kit
/// colours (`database_get_club_kit_colours`, kit 1, the defaults pale grey and 0xaebdf0) after `fill_name` wrote it.
unsafe fn national_squad_title(club: usize, name: usize, fill_name: impl FnOnce()) {
    let mut bottom = job_colour(JOB_COLOUR_3184);
    let mut front = read_u16(0xaebdf0) as u32;
    exe::background_set_club_picture(club, 0, 0);
    exe::menubar_build(4, 0);
    fill_name();
    exe::database_get_club_kit_colours(club, 1, &mut front, &mut bottom);
    exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, bottom, front, name);
}

/// `NationalTeams::format_squad_label` (0x775610, thiscall, ret 0x14) by address: the squad name of `kind` (the
/// under-23 form when the nation is marked for the Olympic Games) into `out` (100 bytes), lower case with `lower`.
unsafe fn national_squad_label(nation: usize, kind: u32, out: usize, lower: u32) {
    let marked = (read_u8(read_usize(read_usize(G_NATIONAL_TEAMS) + national_teams_fields::OLYMPIC_NATION_MARKS) + read_u32(nation) as usize) != 0) as u32;
    let f: unsafe extern "thiscall" fn(usize, u32, u32, usize, i32, u32) -> u32 = core::mem::transmute(0x775610usize);
    f(read_usize(G_NATIONAL_TEAMS), kind, marked, out, 0x64, lower);
}

/// Rule: opening the dialog that moves a national team player into, out of or between squads.
///
/// `national_teams_screens_open_squad_move_screen(player, kind, joining)`: `open_screen` (builder
/// `national_squad_move_screen_build`, input `national_squad_move_screen_handle_input`, flag 1, no leave, no exit);
/// if it opens, parameter 0 = the player, 1 = the squad kind (its byte, sign-extended), 2 = joining (1) or leaving (0)
/// and 3 = −1 (the squad he is in, set by the build).
/// Original: `national_teams_screens_open_squad_move_screen` (0x778f60, national_teams_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn national_teams_screens_open_squad_move_screen(player: usize, kind: u32, joining: u32) {
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x778fd0usize, 0x779610usize, 1, 0usize, 0) == 0 {
        return;
    }
    covered!("tel28_2 open the squad move dialog");
    for (k, v) in national_squad_move_plan(player, kind, joining).iter().enumerate() {
        set_screen_param(k as u32, *v);
    }
}

/// The four parameters `national_teams_screens_open_squad_move_screen` leaves.
fn national_squad_move_plan(player: usize, kind: u32, joining: u32) -> [usize; 4] {
    [player, kind as u8 as i8 as isize as usize, joining as usize, usize::MAX]
}

/// Comparer for `national_teams_screens_open_squad_move_screen`: only the original runs (it opens a screen); the
/// parameters checked against the plan afterwards.
pub(crate) unsafe extern "C" fn compare_national_teams_screens_open_squad_move_screen(player: usize, kind: u32, joining: u32) {
    let plan = national_squad_move_plan(player, kind, joining);
    original_fn::<unsafe extern "C" fn(usize, u32, u32)>(%%I0%%)(player, kind, joining);
    if player != 0 && get_screen_param(0) == player {
        let got = [get_screen_param(0), get_screen_param(1), get_screen_param(2), get_screen_param(3)];
        SCOREBOARD.record(got == plan, || format!("national squad move dialog (open): params {:x?} x plan {:x?}", got, plan));
    }
}

/// Rule: the squad move dialog asks to confirm adding the player to a squad, removing him from one or moving him
/// between two, or says why he cannot be moved.
///
/// The player is parameter 0, the target squad kind parameter 1's byte, joining parameter 2. The current slot's human
/// and his national team (`staff::NATIONAL_JOB`) are needed (none: the national_teams_screens.cpp box of line 0x67 or
/// 0x6f and nothing else). The title: the team's national name (`club_get_national_team_name` with the under-21 form
/// for kinds other than 0 and 2) in its home colours. The squad the player is in (parameter 3): none (−1) without a
/// national job; 2 when that job is a national team (`club_is_national_team`); else 1 unless he is in the national
/// squad (`playing_squad::NATIONAL_SQUAD`), then 0. In a squad, he must be able to leave it
/// (`national_teams_can_change_squad`, leaving); joining, he must be able to join the target. Otherwise the reason
/// (written by those checks), "Could Not Add Player To Squad" or "Could Not Remove Player From Squad" as the subtitle
/// and "Ok". Otherwise, in a squad and joining: "Move Player To <target> Squad" and "Move <player> from <squad> to
/// <target> ?"; in no squad and joining: "Add Player To <target> Squad" and "Add <player> to <target> squad ?";
/// leaving: "Remove Player From <target> Squad" and "Remove <player> from <target> squad ?" (the squad names by
/// `format_squad_label`, the player's by `database_format_staff_name`); the question in a text area and No/Yes.
/// Original: `national_squad_move_screen_build` (0x778fd0, national_teams_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn national_squad_move_screen_build() {
    let player = get_screen_param(0);
    let target = get_screen_param(1) as u8 as u32;
    let joining = get_screen_param(2);
    let (_, club) = national_squad_human_team(0x67, 0x6f);
    if club == 0 {
        return;
    }
    covered!("tel28_2 build the squad move dialog");
    let under = (target as u8 != 0 && target as u8 != 2) as usize;
    national_squad_title(club, JOB_TEXT, || {
        exe::club_get_national_team_name(club, under, JOB_TEXT, 0x7d0, 3, 1);
    });
    let mut current: u8 = 0xff;
    let job = read_usize(player + staff::NATIONAL_JOB);
    if job != 0 {
        current = if exe::club_is_national_team(job) != 0 {
            2
        } else {
            ((!read_u8(player + staff::PLAYING_SQUAD)) >> 2) & 1
        };
    }
    set_screen_param(3, current as i8 as isize as usize);
    let nation = read_usize(club + club::NATION);
    let mut cannot = false;
    if current != 0xff && exe::national_teams_can_change_squad(read_usize(G_NATIONAL_TEAMS), nation, current as u32, player, 0, JOB_TEXT as u32, 0x7d0) == 0 {
        cannot = true;
    }
    if (joining != 0 && exe::national_teams_can_change_squad(read_usize(G_NATIONAL_TEAMS), nation, target, player, 1, JOB_TEXT as u32, 0x7d0) == 0) || cannot {
        covered!("tel28_2 squad move refused");
        national_squad_message();
        tel28_lang()(JOB_TEXT, if joining != 0 { 0xa645c4 } else { 0xa645a0 }); // "Could Not Add Player To Squad" / "Could Not Remove Player From Squad"
        exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, JOB_TEXT, 1);
        national_squad_ok();
        return;
    }
    let (mut to, mut name, mut from) = ([0u8; 0x64], [0u8; 0x12c], [0u8; 0x64]);
    let g = read_u8(club + club::SHORT_GENDER_NAME) as i8 as i32;
    let staff_name = |name: &mut [u8; 0x12c]| {
        exe::database_format_staff_name(player, name.as_mut_ptr() as usize, 0x12c, 1);
    };
    if current != 0xff && joining != 0 {
        national_squad_label(nation, target, to.as_mut_ptr() as usize, 0);
        tel28_lang()(JOB_TEXT, 0xa64564, g, g, to.as_ptr()); // "Move Player To {}<%s>{} Squad"
        exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, JOB_TEXT, 1);
        staff_name(&mut name);
        national_squad_label(nation, current as u32, from.as_mut_ptr() as usize, 1);
        national_squad_label(nation, target, to.as_mut_ptr() as usize, 1);
        tel28_lang()(JOB_TEXT, 0xa644e8, name.as_ptr(), from.as_ptr(), to.as_ptr());
    } else if joining != 0 {
        national_squad_label(nation, target, to.as_mut_ptr() as usize, 0);
        tel28_lang()(JOB_TEXT, 0xa644b4, to.as_ptr()); // "Add Player To <%s> Squad"
        exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, JOB_TEXT, 1);
        staff_name(&mut name);
        national_squad_label(nation, target, to.as_mut_ptr() as usize, 1);
        tel28_lang()(JOB_TEXT, 0xa64460, name.as_ptr(), to.as_ptr()); // "Add <%s> to <%s> squad ?"
    } else {
        national_squad_label(nation, target, to.as_mut_ptr() as usize, 0);
        tel28_lang()(JOB_TEXT, 0xa64424, to.as_ptr()); // "Remove Player From <%s> Squad"
        exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, JOB_TEXT, 1);
        staff_name(&mut name);
        national_squad_label(nation, target, to.as_mut_ptr() as usize, 1);
        tel28_lang()(JOB_TEXT, 0xa643cc, name.as_ptr(), to.as_ptr()); // "Remove <%s> from <%s> squad ?"
    }
    national_squad_message();
    national_squad_yes_no();
}

/// Comparer for `national_squad_move_screen_build`: side by side over the display list; with no human team (the
/// error box) only the original.
pub(crate) unsafe extern "C" fn compare_national_squad_move_screen_build() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I1%%);
    if national_squad_has_box() {
        return o();
    }
    compare_match_screen_build("national squad move dialog", &[], || o(), || national_squad_move_screen_build());
}

/// The widget's id, or the default command id, as the national squad inputs read it inline.
unsafe fn national_squad_widget_id(widget: u32) -> u16 {
    tel26_item_id(widget as i16)
}

/// The end of a national squad dialog's input: No (id 1) closes it (−9); otherwise the menu bar
/// (`menubar_handle_command`) and, when it does nothing, the object link (`menubar_handle_object_link`).
unsafe fn national_squad_other_click(widget: u32) -> i32 {
    if national_squad_widget_id(widget) == 1 {
        return NATIONAL_SQUAD_DONE;
    }
    let r = exe::menubar_handle_command(widget);
    if r != 0 {
        return r;
    }
    exe::menubar_handle_object_link(widget)
}

/// Rule: answering Yes in the squad move dialog takes the player out of the squad he is in and puts him in the target
/// squad, calling up a replacement when needed.
///
/// `national_squad_move_screen_handle_input(widget)`: Yes (id 2): the human team is needed (none: the box of line
/// 0x136 or 0x13e and 0). The nation's 110-byte record (the national teams' vector +4). With a national job the
/// player leaves the squad of parameter 3 (`nationalteams_remove_player_from_squad`); when that squad is not the main
/// one (0), its call-up is active (0x76e010) and the human does not control all national teams
/// (`human_manager_controls_all_national_teams`), a replacement is due: `national_team_level` of the player (with the
/// record, 1 for squad 2) replaces the record dword's low byte. Joining: `nationalteams_add_player_to_squad` into the
/// target (parameter 1). The replacement: `nationalteams_call_up_replacement` (the squad, the dword, the player).
/// Returns −9. Other widgets: `national_squad_other_click`.
/// Original: `national_squad_move_screen_handle_input` (0x779610, national_teams_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn national_squad_move_screen_handle_input(widget: u32) -> i32 {
    if national_squad_widget_id(widget) != 2 {
        return national_squad_other_click(widget);
    }
    let player = get_screen_param(0);
    let target = get_screen_param(1) as u8 as u32;
    let joining = get_screen_param(2);
    let (manager, club) = national_squad_human_team(0x136, 0x13e);
    if club == 0 {
        return 0;
    }
    covered!("tel28_2 squad move confirmed");
    let nt = read_usize(G_NATIONAL_TEAMS);
    let nation = read_usize(club + club::NATION);
    let mut record = read_usize(nt + 4) + read_u32(nation) as usize * NATIONAL_SQUAD_RECORD_SIZE;
    let mut replace = false;
    let mut squad = 0u32;
    if read_usize(player + staff::NATIONAL_JOB) != 0 {
        squad = get_screen_param(3) as u8 as u32;
        exe::nationalteams_remove_player_from_squad(nt, nation, squad, player);
        let active: unsafe extern "thiscall" fn(usize, usize, u32) -> i32 = core::mem::transmute(0x76e010usize);
        if active(read_usize(G_NATIONAL_TEAMS), nation, squad) != 0
            && squad != 0
            && exe::human_manager_controls_all_national_teams(read_usize(G_HUMAN_MANAGER), manager, core::ptr::null_mut()) == 0
        {
            covered!("tel28_2 squad move replacement");
            replace = true;
            let level = exe::national_teams_get_position_group(player, record, (squad == 2) as u32);
            record = (record & !0xff) | level as usize;
        }
    }
    if joining != 0 {
        exe::nationalteams_add_player_to_squad(read_usize(G_NATIONAL_TEAMS), nation, target, player);
    }
    if replace {
        exe::nationalteams_call_up_replacement(read_usize(G_NATIONAL_TEAMS), nation, squad, record as u32, player);
    }
    NATIONAL_SQUAD_DONE
}

/// Comparer for `national_squad_move_screen_handle_input`: No side by side (nothing changes); Yes (it moves the
/// player), the menu bar and the object links only the original.
pub(crate) unsafe extern "C" fn compare_national_squad_move_screen_handle_input(widget: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I2%%);
    let pure = national_squad_widget_id(widget) == 1;
    tel28_click_compare("national squad move dialog input", widget, pure, || o(widget), || national_squad_move_screen_handle_input(widget))
}

/// The national squad news item behind a dialog opened from the inbox: `NewsItem::NewsItem` on `item` and
/// `News::find_item_by_id` (0x786b70) of the manager record and the id.
unsafe fn national_squad_news_item(item: &mut [u8; 0xe0], manager: usize, id: u32) {
    exe::new_newsitem(item.as_mut_ptr() as usize, 0, 0);
    let find: unsafe extern "thiscall" fn(usize, usize, u32, usize) -> u32 = core::mem::transmute(0x786b70usize);
    find(read_usize(G_NEWS), manager, id, item.as_mut_ptr() as usize);
}

/// The club of a news item slot (an index into the club table).
unsafe fn national_squad_slot_club(item: &[u8; 0xe0], slot: u32) -> usize {
    read_usize(G_CLUB_TABLE).wrapping_add((exe::newsitem_get_slot(item.as_ptr() as usize, slot) as usize).wrapping_mul(club::SIZE))
}

/// The eight parameters of the squad selection dialog as `national_squad_selection_open_from_news` leaves them: the
/// team (slot 0), the squad kind (slot 1), the manager record, the news id, the players not yet in that squad (the
/// byte), and slots 5, 6 and 7 (the match day and year of the squad date [inference]).
unsafe fn national_squad_selection_plan(manager: usize, id: u32) -> [usize; 8] {
    let mut item = [0u8; 0xe0];
    national_squad_news_item(&mut item, manager, id);
    let club = national_squad_slot_club(&item, 0);
    let kind = exe::newsitem_get_slot(item.as_ptr() as usize, 1);
    let flag = if kind != 0 { playing_squad::NATIONAL_SQUAD } else { playing_squad::NATIONAL_B_SQUAD };
    let mut count = 0u8;
    for k in 0..0x32 {
        let p = read_usize(club + club::SQUAD + k * 4);
        if p != 0 && read_u8(p + staff::PLAYING_SQUAD) & flag == 0 {
            count = count.wrapping_add(1);
        }
    }
    let slot = |k: u32| exe::newsitem_get_slot(item.as_ptr() as usize, k) as usize;
    [club, kind as usize, manager, id as usize, count as i8 as isize as usize, slot(5), slot(6), slot(7)]
}

/// Rule: answering a squad selection news item opens the dialog that confirms the selected squad.
///
/// `national_squad_selection_open_from_news(manager, id)`: `open_screen` (builder
/// `national_squad_selection_screen_build`, input `national_squad_selection_screen_handle_input`, flag 1); if it
/// opens, the news item (`News::find_item_by_id`) gives the team (slot 0, an index into the club table) and the squad
/// kind (slot 1); the players of the team's squad (50 slots) not yet flagged for that squad (the national squad flag
/// for a kind other than 0, the B squad flag for 0) are counted. Parameters 0 the team, 1 the kind, 2 the manager, 3
/// the id, 4 the count (its byte), 5 to 7 slots 5 to 7.
/// Original: `national_squad_selection_open_from_news` (0x7798c0, national_teams_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn national_squad_selection_open_from_news(manager: usize, id: u32) {
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x779a50usize, 0x779df0usize, 1, 0usize, 0) == 0 {
        return;
    }
    covered!("tel28_2 open the squad selection dialog");
    for (k, v) in national_squad_selection_plan(manager, id).iter().enumerate() {
        set_screen_param(k as u32, *v);
    }
}

/// Comparer for `national_squad_selection_open_from_news`: only the original runs (it opens a screen); the
/// parameters checked against the plan afterwards.
pub(crate) unsafe extern "C" fn compare_national_squad_selection_open_from_news(manager: usize, id: u32) {
    let plan = national_squad_selection_plan(manager, id);
    original_fn::<unsafe extern "C" fn(usize, u32)>(%%I3%%)(manager, id);
    if get_screen_param(2) == manager && get_screen_param(3) == id as usize {
        let mut got = [0usize; 8];
        for (k, v) in got.iter_mut().enumerate() {
            *v = get_screen_param(k as u32);
        }
        SCOREBOARD.record(got == plan, || format!("national squad selection dialog (open {}): params {:x?} x plan {:x?}", id, got, plan));
    }
}

/// The squad confirmation dialogs (the selection and the call-up): the title, then either the confirmation with
/// No/Yes or why the squad is invalid with "Ok".
///
/// The team is parameter 0, the squad kind parameter 1, the players counted by the opener parameter 4's byte. The
/// title: the team's national name (`club_get_national_team_name`, form 3) in its home colours (the call-up writes it
/// into the local name, the selection into [`JOB_TEXT`]). The squad size limits (`NationalTeams::get_max_squad_size`
/// 0x7754c0, `get_min_squad_size` 0x775520, signed bytes). Within them: "Confirm Squad Selection" as the subtitle and
/// "Confirm <team name> squad selection ?" (the name in form 2 for the selection, 3 for the call-up) with No/Yes.
/// Outside: "Squad Invalid" and the reason with the name in form 2: with equal limits "must have <max> player" (count
/// 1) or the limit and the count; else the two limits (and the count when it is not 1); then "Ok".
unsafe fn national_squad_confirm_build(call_up: bool) {
    let club = get_screen_param(0);
    let kind = get_screen_param(1);
    let count = get_screen_param(4) as u8 as i8;
    let mut name = [0u8; 0xc8];
    let title = if call_up { name.as_mut_ptr() as usize } else { JOB_TEXT };
    national_squad_title(club, title, || {
        if call_up {
            exe::club_get_national_team_name(club, kind, title, 0xc8, 3, 1);
        } else {
            exe::club_get_national_team_name(club, kind, title, 0x7d0, 3, 1);
        }
    });
    let limit = |address: usize| {
        let f: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(address);
        f(read_usize(G_NATIONAL_TEAMS), club, kind) as i8
    };
    let max = limit(0x7754c0);
    let min = limit(0x775520);
    let g = read_u8(club + club::SHORT_GENDER_NAME) as i8 as i32;
    let national_name = |name: &mut [u8; 0xc8], form: u32| {
        exe::club_get_national_team_name(club, kind, name.as_mut_ptr() as usize, 0xc8, form, 1);
    };
    if count <= max && count >= min {
        covered!("tel28_2 squad confirmation");
        if !call_up {
            national_name(&mut name, 2);
        }
        tel28_lang()(JOB_TEXT, 0xa64968); // "Confirm Squad Selection"
        exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, JOB_TEXT, 1);
        if call_up {
            national_name(&mut name, 3);
        }
        tel28_lang()(JOB_TEXT, 0xa64918, g, g, name.as_ptr()); // "Confirm {}<%s>{} squad selection ?"
        national_squad_message();
        national_squad_yes_no();
        return;
    }
    covered!("tel28_2 squad invalid");
    tel28_lang()(JOB_TEXT, 0xa64908); // "Squad Invalid"
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, JOB_TEXT, 1);
    national_name(&mut name, 2);
    let (min, max, count) = (min as i32, max as i32, count as i32);
    if min == max {
        if count == 1 {
            tel28_lang()(JOB_TEXT, 0xa6486c, g, g, name.as_ptr(), max);
        } else {
            tel28_lang()(JOB_TEXT, 0xa647b8, g, g, name.as_ptr(), max, count);
        }
    } else if count == 1 {
        tel28_lang()(JOB_TEXT, 0xa646f8, g, g, name.as_ptr(), min, max);
    } else {
        tel28_lang()(JOB_TEXT, 0xa64620, g, g, name.as_ptr(), min, max, count);
    }
    national_squad_message();
    national_squad_ok();
}

/// Rule: the squad selection dialog confirms the selected national squad when its size is within the limits.
///
/// See `national_squad_confirm_build` (the selection form: the title in [`JOB_TEXT`], the question with the name in
/// form 2).
/// Original: `national_squad_selection_screen_build` (0x779a50, national_teams_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn national_squad_selection_screen_build() {
    national_squad_confirm_build(false);
}

/// Comparer for `national_squad_selection_screen_build`: side by side over the display list.
pub(crate) unsafe extern "C" fn compare_national_squad_selection_screen_build() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I4%%);
    compare_match_screen_build("national squad selection dialog", &[], || o(), || national_squad_selection_screen_build());
}

/// Rule: confirming a squad selection marks the selection as made, posts the call-up news and, when no match is
/// pending in the news, moves the team's club fixtures that clash with the squad date.
///
/// `national_squad_selection_screen_handle_input(widget)`: Yes (id 2): the news item of parameters 2 and 3 gives
/// slots 3 (a pending fixture [inference]), 4 (how many clubs, 1 to 3: slots 8 on are their indexes, kept in a local
/// list) and 5. The squad slot (`national_teams_get_squad_slot` of the team and parameter 1) over the kind's low byte
/// marks the nation (0x76c0f0); the item's slot 2 gets 1 (`News::set_item_slot`); `national_teams_post_call_up_news`
/// (the team, the kind, 0, slot 5, slot 3, the club list, slot 4). With slot 3 at 0: the squad date (parameters 6 and
/// 7) and the day before (`Date::sub_days`), the team's fixtures between them (`fixture_manager_find_fixtures`, every
/// competition, −2, −1); the first one records the clubs' call-ups (`nationalteams_record_club_call_ups`) and
/// reschedules the clashing club fixtures (`nationalteams_reschedule_clashing_club_fixtures` on the squad date).
/// Returns −9. Other widgets: `national_squad_other_click`.
/// Original: `national_squad_selection_screen_handle_input` (0x779df0, national_teams_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn national_squad_selection_screen_handle_input(widget: u32) -> i32 {
    if national_squad_widget_id(widget) != 2 {
        return national_squad_other_click(widget);
    }
    covered!("tel28_2 squad selection confirmed");
    let mut clubs = [0u8; 0xe];
    exe::new_llist(clubs.as_mut_ptr() as usize);
    let mut item = [0u8; 0xe0];
    exe::new_newsitem(item.as_mut_ptr() as usize, 0, 0);
    let club = get_screen_param(0);
    let kind = get_screen_param(1) as u32;
    let (manager, id) = (get_screen_param(2), get_screen_param(3) as u32);
    let find: unsafe extern "thiscall" fn(usize, usize, u32, usize) -> u32 = core::mem::transmute(0x786b70usize);
    find(read_usize(G_NEWS), manager, id, item.as_mut_ptr() as usize);
    let pending = exe::newsitem_get_slot(item.as_ptr() as usize, 3);
    let n = exe::newsitem_get_slot(item.as_ptr() as usize, 4) as u8 as i8;
    let slot5 = exe::newsitem_get_slot(item.as_ptr() as usize, 5);
    if n < 4 && n > 0 {
        let mut i: i8 = 0;
        while i < n {
            let mut c = national_squad_slot_club(&item, (i as u8).wrapping_add(8) as u32);
            exe::llist_add_tail(clubs.as_mut_ptr() as usize, &mut c as *mut usize as usize, 4);
            i += 1;
        }
    }
    let slot = exe::national_teams_get_squad_slot(club, kind);
    let mark: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x76c0f0usize);
    mark(read_usize(G_NATIONAL_TEAMS), read_usize(club + club::NATION), (kind & !0xff) | slot as u32);
    let set_slot: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) = core::mem::transmute(FN_NEWS_SET_ITEM_SLOT);
    set_slot(read_usize(G_NEWS), manager, id, 2, 1, 0);
    exe::national_teams_post_call_up_news(read_usize(G_NATIONAL_TEAMS), club, kind, 0usize, slot5, pending, clubs.as_ptr() as u32, n as u8 as u32);
    if pending == 0 {
        covered!("tel28_2 squad selection reschedules");
        let mut fixtures = [0u8; 0xe];
        exe::new_llist(fixtures.as_mut_ptr() as usize);
        let (mut date, mut before) = ([0u8; 8], [0u8; 8]);
        exe::new_date(date.as_mut_ptr() as usize, get_screen_param(6) as u32, get_screen_param(7) as u32);
        exe::date_sub_days(date.as_mut_ptr() as usize, before.as_mut_ptr() as usize, 1);
        let (b, d) = (before.as_ptr() as usize, date.as_ptr() as usize);
        exe::fixture_manager_find_fixtures(read_usize(G_FIXTURE_MANAGER), read_u32(b), read_u32(b + 2), read_u32(d), read_u32(d + 2), read_u32(club),
            fixtures.as_mut_ptr() as usize, u32::MAX, 0xffff_fffe, u32::MAX);
        let list = fixtures.as_mut_ptr() as usize;
        write_u32(list + llist::CURSOR, read_u32(list + llist::HEAD));
        let fixture = exe::llist_get_next(list);
        if fixture != 0 {
            exe::nationalteams_record_club_call_ups(read_usize(G_NATIONAL_TEAMS), club, kind, fixture);
            exe::nationalteams_reschedule_clashing_club_fixtures(read_usize(G_NATIONAL_TEAMS), date.as_mut_ptr() as usize);
        }
        exe::drop_llist(list);
    }
    exe::drop_llist(clubs.as_mut_ptr() as usize);
    NATIONAL_SQUAD_DONE
}

/// Comparer for `national_squad_selection_screen_handle_input`: No side by side; Yes (news, call-ups, fixtures), the
/// menu bar and the object links only the original.
pub(crate) unsafe extern "C" fn compare_national_squad_selection_screen_handle_input(widget: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I5%%);
    let pure = national_squad_widget_id(widget) == 1;
    tel28_click_compare("national squad selection dialog input", widget, pure, || o(widget), || national_squad_selection_screen_handle_input(widget))
}

/// The five parameters of the call-up dialog as `national_squad_call_up_open_from_news` leaves them: the team (slot
/// 4), the squad kind (slot 5), the manager record, the news id and the squad count (`database_count_club_squad` of
/// the B squad flag 8 for a kind other than 0, else 4; its byte).
unsafe fn national_squad_call_up_plan(manager: usize, id: u32) -> [usize; 5] {
    let mut item = [0u8; 0xe0];
    national_squad_news_item(&mut item, manager, id);
    let club = national_squad_slot_club(&item, 4);
    let kind = exe::newsitem_get_slot(item.as_ptr() as usize, 5);
    let count = exe::database_count_club_squad(club, if kind != 0 { 8 } else { 4 }, 0);
    [club, kind as usize, manager, id as usize, count as i8 as isize as usize]
}

/// Rule: answering a call-up news item opens the dialog that confirms the called-up squad.
///
/// `national_squad_call_up_open_from_news(manager, id)`: `open_screen` (builder `national_squad_call_up_screen_build`,
/// input `national_squad_call_up_screen_handle_input`, flag 1); if it opens, the five parameters of
/// `national_squad_call_up_plan` (the team from slot 4, the kind from slot 5, the manager, the id, the squad count).
/// Original: `national_squad_call_up_open_from_news` (0x77a140, national_teams_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn national_squad_call_up_open_from_news(manager: usize, id: u32) {
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x77a270usize, 0x77a610usize, 1, 0usize, 0) == 0 {
        return;
    }
    covered!("tel28_2 open the call-up dialog");
    for (k, v) in national_squad_call_up_plan(manager, id).iter().enumerate() {
        set_screen_param(k as u32, *v);
    }
}

/// Comparer for `national_squad_call_up_open_from_news`: only the original runs (it opens a screen); the parameters
/// checked against the plan afterwards.
pub(crate) unsafe extern "C" fn compare_national_squad_call_up_open_from_news(manager: usize, id: u32) {
    let plan = national_squad_call_up_plan(manager, id);
    original_fn::<unsafe extern "C" fn(usize, u32)>(%%I6%%)(manager, id);
    if get_screen_param(2) == manager && get_screen_param(3) == id as usize {
        let got = [get_screen_param(0), get_screen_param(1), get_screen_param(2), get_screen_param(3), get_screen_param(4)];
        SCOREBOARD.record(got == plan, || format!("national squad call-up dialog (open {}): params {:x?} x plan {:x?}", id, got, plan));
    }
}

/// Rule: the call-up dialog confirms the called-up national squad when its size is within the limits.
///
/// See `national_squad_confirm_build` (the call-up form: the title in the local name, the question with the name in
/// form 3).
/// Original: `national_squad_call_up_screen_build` (0x77a270, national_teams_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn national_squad_call_up_screen_build() {
    national_squad_confirm_build(true);
}

/// Comparer for `national_squad_call_up_screen_build`: side by side over the display list.
pub(crate) unsafe extern "C" fn compare_national_squad_call_up_screen_build() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I7%%);
    compare_match_screen_build("national squad call-up dialog", &[], || o(), || national_squad_call_up_screen_build());
}

/// Rule: confirming a call-up posts the news of each squad player called up today and clears the nation's call-up
/// mark.
///
/// `national_squad_call_up_screen_handle_input(widget)`: Yes (id 2): the team is parameter 0, the kind parameter 1;
/// for each of the team's 50 squad slots, a valid player (id below `G_STAFF_COUNT`) flagged for the squad (the B
/// squad flag 8 for a kind other than 0, else 4) who joined the nation today (`staff::DATE_JOINED_NATION` ==
/// `G_CURRENT_DATE`) gets the called-up news (`national_teams_post_player_called_up_news` with the squad slot of
/// `national_teams_get_squad_slot`); the news item of parameters 2 and 3 gets 1 in slot 7 (`News::set_item_slot`) and
/// the nation's mark of the squad slot is cleared (0x76d7e0). Returns −9. Other widgets: `national_squad_other_click`.
/// Original: `national_squad_call_up_screen_handle_input` (0x77a610, national_teams_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn national_squad_call_up_screen_handle_input(widget: u32) -> i32 {
    if national_squad_widget_id(widget) != 2 {
        return national_squad_other_click(widget);
    }
    covered!("tel28_2 call-up confirmed");
    let club = get_screen_param(0);
    let kind = get_screen_param(1);
    let (manager, id) = (get_screen_param(2), get_screen_param(3) as u32);
    let flag = if kind != 0 { playing_squad::NATIONAL_B_SQUAD } else { playing_squad::NATIONAL_SQUAD };
    for k in 0..0x32 {
        let p = read_usize(club + club::SQUAD + k * 4);
        if p != 0
            && read_i32(p) < read_i32(G_STAFF_COUNT)
            && read_u8(p + staff::PLAYING_SQUAD) & flag != 0
            && exe::date_operator_3d_3d(p + staff::DATE_JOINED_NATION, read_u32(G_CURRENT_DATE), read_u32(G_CURRENT_DATE_LEAP)) != 0
        {
            let slot = exe::national_teams_get_squad_slot(club, kind as u32);
            exe::national_teams_post_player_called_up_news(read_usize(G_NATIONAL_TEAMS), p, read_usize(club + club::NATION), slot as u32);
        }
    }
    let set_slot: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) = core::mem::transmute(FN_NEWS_SET_ITEM_SLOT);
    set_slot(read_usize(G_NEWS), manager, id, 7, 1, 0);
    let slot = exe::national_teams_get_squad_slot(club, kind as u32);
    let clear: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x76d7e0usize);
    clear(read_usize(G_NATIONAL_TEAMS), read_usize(club + club::NATION), slot as u32);
    NATIONAL_SQUAD_DONE
}

/// Comparer for `national_squad_call_up_screen_handle_input`: No side by side; Yes (news), the menu bar and the
/// object links only the original.
pub(crate) unsafe extern "C" fn compare_national_squad_call_up_screen_handle_input(widget: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I8%%);
    let pure = national_squad_widget_id(widget) == 1;
    tel28_click_compare("national squad call-up dialog input", widget, pure, || o(widget), || national_squad_call_up_screen_handle_input(widget))
}
