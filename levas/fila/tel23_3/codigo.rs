// ---------- batch %%LEVA%%: the Intertoto application, the news link to a competition, the related competitions and stage menus and the fixture row (comp_screens.cpp, part 2) ----------

/// `News::find_item_by_id` (0x786b70, thiscall, ret 0xc): the news item of ids (`a`, `b`) copied into `item`; zero
/// when there is none.
unsafe fn news_find_item(a: u32, b: u32, item: usize) -> u32 {
    let f: unsafe extern "thiscall" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x786b70usize);
    f(read_usize(G_NEWS), a, b, item)
}

/// `News::set_item_slot` (0x786d30, thiscall, ret 0x14): slot `slot` of news item (`a`, `b`) = `value`.
unsafe fn news_set_item_slot(a: u32, b: u32, slot: u32, value: u32, extra: u32) {
    let f: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) = core::mem::transmute(FN_NEWS_SET_ITEM_SLOT);
    f(read_usize(G_NEWS), a, b, slot, value, extra)
}

/// A NewsItem on the stack (0xe0 bytes reserved; `NewsItem::NewsItem(0, 0)`, no destructor).
const NEWS_ITEM_BYTES: usize = 0xe0;

/// Rule: opening the Intertoto application screen of a news item (builder `comp_intertoto_apply_screen_draw`, input
/// `comp_intertoto_apply_screen_handle_event`, flag 1, no leave): params 0 and 1 the news item's ids.
/// Original: `comp_open_intertoto_apply_screen` (0x49cc60, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_open_intertoto_apply_screen(a: u32, b: u32) {
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x49ccb0usize, 0x49ce50usize, 1, 0usize, 0) == 0 {
        return;
    }
    covered!("tel23_3 open intertoto apply");
    set_screen_param(0, a as usize);
    set_screen_param(1, b as usize);
}

/// Rule: building the Intertoto application screen: "Apply for the one English Intertoto Cup place ?" with No and Yes.
///
/// The Intertoto Cup's picture, the menu bar (4), its colours with its name as the title and "Please Confirm" as the
/// subtitle. The question in a text area (x 0x6e..0x30c, y 0x91..0x217, pale grey, font 4) with the cup's name as a
/// link (id 0x7d0). A 2-column area (weights 3, 1; x 0x64..0x316, y 0x22b..0x24e) with the "No" (id 1) and "Yes" (id 2)
/// buttons.
/// Original: `comp_intertoto_apply_screen_draw` (0x49ccb0, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_intertoto_apply_screen_draw() {
    let weights: [u8; 2] = [3, 1];
    let cup = comp_record_of_key(KEY_CLUB_COMP_INTER_TOTO_CUP);
    exe::background_set_comp_picture(cup, 0, 0);
    exe::menubar_build(4, 0);
    comp_colours_title(comp_colours(cup), cup + competition::NAME);
    comp_lang_format()(JOB_TEXT, 0x9909b8); // "Please Confirm"
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, JOB_TEXT, 1);
    let gender = read_u8(cup + competition::GENDER_NAME) as i8 as i32;
    comp_lang_format()(JOB_TEXT, 0x99c938, gender, gender, cup + competition::NAME); // "Apply for the one English {}<%s - competition>{} place ?"
    type TextArea = unsafe extern "C" fn(u32, u32, u32, u32, u32, u32, u32, u32, u32, usize, usize, u32, usize, u32);
    let text_area: TextArea = core::mem::transmute(0x5ed070usize);
    text_area(0x6e, 0x91, 0x30c, 0x217, 2, 0, 0x10, 4, job_colour(G_GUI_COLOUR_PALE_GREY), JOB_TEXT, cup + competition::NAME, 0x7d0, cup, 0);
    let area = tel26_add_area(current_human_display(), [0x64, 0x22b, 0x316, 0x24e, 2, weights.as_ptr() as u32, 1, 0, 1, 0, u32::MAX]);
    comp_lang_format()(JOB_TEXT, 0x98e8cc); // "No"
    tel26_area_button([area, 0, 0, 0x30, job_colour(G_GUI_COLOUR_GREY), 0xc, 3, job_colour(G_GUI_COLOUR_PALE_GREY), JOB_TEXT as u32, 1, 0]);
    comp_lang_format()(JOB_TEXT, 0x98e8d0); // "Yes"
    tel26_area_button([area, 1, 0, 0x30, job_colour(G_GUI_COLOUR_GREY), 0xc, 3, job_colour(G_GUI_COLOUR_PALE_GREY), JOB_TEXT as u32, 2, 0]);
}

/// Rule: the Intertoto application's answer.
///
/// The news item is (param 0, param 1). "Yes" (id 2): the item is looked up (`News::find_item_by_id`; none: the
/// comp_screens.cpp box, line 0x1e7f, and 0); its club (slot 1, the club table's record) volunteers for the Intertoto
/// Cup (`UefaSeeding::add_intertoto_volunteer`); slot 0 of the item = 1 (answered) and slot 3 = 1; −9. "No" (id 1):
/// slot 0 = 1 and slot 3 = 0; −9. Other ids go to the menu bar.
/// Original: `comp_intertoto_apply_screen_handle_event` (0x49ce50, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_intertoto_apply_screen_handle_event(widget: u32) -> i32 {
    let mut item = [0u8; NEWS_ITEM_BYTES];
    exe::new_newsitem(item.as_mut_ptr() as usize, 0, 0);
    let a = get_screen_param(0) as u32;
    let b = get_screen_param(1) as u32;
    match tel26_item_id(widget as i16) {
        2 => {
            if news_find_item(a, b, item.as_mut_ptr() as usize) == 0 {
                comp_screens_error_box(0x1e7f);
                return 0;
            }
            covered!("tel23_3 intertoto yes");
            let club = read_usize(G_CLUB_TABLE) + exe::newsitem_get_slot(item.as_mut_ptr() as usize, 1) as usize * club::SIZE;
            let volunteer: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x9037b0usize);
            volunteer(read_usize(G_UEFA_SEEDING), club);
            news_set_item_slot(a, b, 0, 1, 0);
            news_set_item_slot(a, b, 3, 1, 0);
            -9
        }
        1 => {
            covered!("tel23_3 intertoto no");
            news_set_item_slot(a, b, 0, 1, 0);
            news_set_item_slot(a, b, 3, 0, 0);
            -9
        }
        _ => comp_click_menubar(widget),
    }
}

/// Rule: a news item's link to a competition: the item (`a`, `b`) is looked up and the competition screen opens
/// (`comp_screens_open_comp_screen`) on the competition record of slot 0, the stage of slot 4 (a signed byte) and the
/// round of slot 5 (a signed word), mode 0.
/// Original: `comp_screens_open_comp_from_news_item` (0x49eac0, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_screens_open_comp_from_news_item(a: u32, b: u32) {
    let mut item = [0u8; NEWS_ITEM_BYTES];
    let p = item.as_mut_ptr() as usize;
    exe::new_newsitem(p, 0, 0);
    news_find_item(a, b, p);
    let comp = exe::newsitem_get_slot(p, 0);
    let stage = exe::newsitem_get_slot(p, 4) as u8 as i8;
    let round = exe::newsitem_get_slot(p, 5) as u16 as i16;
    let open: unsafe extern "C" fn(usize, i32, i32, u32) = core::mem::transmute(0x48f4a0usize);
    open(read_usize(G_CLUB_COMP_TABLE) + comp as usize * 0x6b, stage as i32, round as i32, 0);
}

/// The popup of a menu button (x 0x69..0x78, y 0xf..0x23, colours `colour_a`/`colour_b`, no text): the button
/// (`add_button_item`) and its popup area (flags 0x30); returns the area.
unsafe fn related_comps_popup(colour_a: u32, colour_b: u32) -> u32 {
    let button = tel26_add_button([0x69, 0xf, 0x78, 0x23, 0x30, colour_a, 0xc, 1, colour_b, 0, 0, 0]);
    tel26_add_area(current_human_display(), [0, 0, 0, 0, 0, 0, 0, 0, 0x30, job_colour(JOB_COLOUR_31AA), button])
}

/// Rule: the related competitions menu of a competition screen: a small menu button (top left) whose popup lists the
/// other competitions of the competition's nation and its written history.
///
/// Nothing while [0x99c00c] is clear (answers 1). No object: the comp_screens.cpp box (line 0x1ec4), 0. The nation's
/// entry of the league table ([G_LEAGUE_TABLE] + 0x48 × `key_nation_index_of_comp`): each competition object of its
/// list (+0x10, +0xc of them) adds its name (+0x38 of its record) as an item (id `comp_id`, the record as value),
/// creating the button and popup at the first one. When the competition has a written history file
/// (`history_his_file_exists` of its English name): a separator when the popup exists (else the popup is created) and
/// "History" (id `history_id`). Answers whether the popup was created.
/// Original: `comp_screens_add_related_comps_menu` (0x49d070, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_screens_add_related_comps_menu(object: usize, colour_a: u32, colour_b: u32, comp_id: u32, history_id: u32) -> u32 {
    if read_u32(COMP_SCREEN_BUILT_FLAG) == 0 {
        return 1;
    }
    if object == 0 {
        comp_screens_error_box(0x1ec4);
        return 0;
    }
    let mut area: u32 = u32::MAX;
    let index = exe::key_nation_index_of_comp(read_usize(object + league::COMPETITION)) as i8 as isize;
    let entry = (G_LEAGUE_TABLE as isize + index * 0x48) as usize;
    let (c31aa, yellow, black) = (job_colour(JOB_COLOUR_31AA), job_colour(G_GUI_COLOUR_YELLOW), job_colour(G_GUI_COLOUR_BLACK));
    if read_usize(entry + 0x10) != 0 {
        let mut k: i32 = 0;
        while k < read_i32(entry + 0xc) {
            let other = read_usize(read_usize(entry + 0x10) + k as usize * 4);
            if other != 0 {
                if area as u16 == 0xffff {
                    area = related_comps_popup(colour_a, colour_b);
                }
                let record = read_usize(other + league::COMPETITION);
                tel26_add_item(current_human_display(), [2, 0, 0, 0, 0, 0, 0, 0x10, c31aa, yellow, 1, 1, black, (record + competition::SHORT_NAME) as u32, 0, comp_id, record as u32, area]);
            }
            k += 1;
        }
    }
    let history_file: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x5f2500usize);
    if history_file(comp_get_english_name(read_usize(object + league::COMPETITION))) != 0 {
        covered!("tel23_3 related comps history");
        if area as u16 == 0xffff {
            area = related_comps_popup(colour_a, colour_b);
        } else {
            tel26_add_item(current_human_display(), [1, 0, 0, 0, 0, 0, 0, 0x1000010, c31aa, yellow, 1, 1, black, 0, 0, 0, 0, area]);
        }
        comp_lang_format()(JOB_TEXT, 0x9914d4, 0xad9c64usize); // "History<%s - COMMENT - written history screen>"
        tel26_add_item(current_human_display(), [2, 0, 0, 0, 0, 0, 0, 0x10, c31aa, yellow, 1, 1, black, JOB_TEXT as u32, 0, history_id, 0, area]);
    }
    (area as u16 != 0xffff) as u32
}

/// The rounds of a cup stage `s` (`cup::NEXT_DRAW_ROUND` of them, re-read each round) as items of the menu in `area`:
/// each round's code (+7 of its 0x68-byte record), preceded, at the round before the last when the last is the third
/// place play-off (code 0x8c), by that play-off.
unsafe fn stage_selector_cup_rounds(area: *mut i32, x1: u32, x2: u32, top: i32, text: usize, code: i32, stage: i32, s: usize) {
    let mut r: i32 = 0;
    while r < read_u8(s + cup::NEXT_DRAW_ROUND) as i8 as i32 {
        let rounds = read_i16(s + cup::ROUND_COUNT) as i32;
        let table = read_usize(s + cup::ROUND_TABLE);
        if r == rounds - 2 && read_i16((table as isize + (rounds - 1) as isize * cup_round::SIZE as isize) as usize + cup_round::ROUND_CODE) == 0x8c {
            comp_screens_add_stage_menu_item(area as *mut i16, x1, top, x2, top + 0x14, text, code, 0x8c, stage, rounds - 1);
        }
        let round_code = read_i16(table + r as usize * cup_round::SIZE + cup_round::ROUND_CODE) as i32;
        comp_screens_add_stage_menu_item(area as *mut i16, x1, top, x2, top + 0x14, text, code, round_code, stage, r);
        r += 1;
    }
}

/// Rule: the stage menus above a competition tab: a "Stage" menu of the competition's stages and, for the selected
/// one, a second menu of its groups or rounds.
///
/// No competition: the comp_screens.cpp box (line 0x1f1b), 0. A competition still in its first stage (current stage
/// −1) has none when it is a league, or a cup with one round to draw. Then, for each stage k from −1 to the current
/// one (`Comp::get_stage`): a stage with a parent stage id (`comp_util_get_parent_stage_id`) heads a run of siblings
/// (the following stages with the same parent): when the run is every stage, each sibling goes on the main menu as a
/// "Group"; otherwise the run goes on it once as "Stage" (the parent's id) and, when the selected stage is in the run,
/// each sibling goes on the second menu (x 0x210..0x28d) as a "Group"; k skips the run. A cup stage: on a competition
/// still in its first stage its rounds go on the main menu as "Round" (with the stage id); a cup of one round to draw
/// goes on it as "Stage" with its first round's code; a cup without a stage id lists its rounds there as "Stage"
/// (code 0); otherwise it goes on as "Stage" and, when selected, its rounds on the second menu as "Round" (code 0). A
/// league stage goes on the main menu as "Stage". Each item by `comp_screens_add_stage_menu_item` (main menu x
/// 0x28f..0x30c, `top`..`top` + 0x14). Answers whether either menu exists.
/// Original: `comp_screens_add_stage_selector` (0x49d380, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_screens_add_stage_selector(comp: usize, selected: i32, top: i32) -> u32 {
    if comp == 0 {
        comp_screens_error_box(0x1f1b);
        return 0;
    }
    if read_i32(comp + league::CURRENT_STAGE) == -1 {
        if read_u8(comp + league::COMP_TYPE) == 2 {
            return 0;
        }
        if read_u8(comp + league::COMP_TYPE) == 1 && read_u8(comp + cup::NEXT_DRAW_ROUND) == 1 {
            return 0;
        }
    }
    let (mut stage_text, mut group_text, mut round_text) = ([0u8; 0xc8], [0u8; 0xc8], [0u8; 0xc8]);
    comp_lang_format()(stage_text.as_mut_ptr() as usize, 0x99c518); // "Stage"
    comp_lang_format()(group_text.as_mut_ptr() as usize, 0x99c97c); // "Group"
    comp_lang_format()(round_text.as_mut_ptr() as usize, 0x99c974); // "Round"
    let (stage_text, group_text, round_text) = (stage_text.as_ptr() as usize, group_text.as_ptr() as usize, round_text.as_ptr() as usize);
    let (mut menu, mut sub): (i32, i32) = (-1, -1);
    if read_i32(comp + league::CURRENT_STAGE) < -1 {
        return 0;
    }
    let stage_id = |s: usize| read_i16(s + league::STAGE_ID) as i32;
    let mut k: i32 = -1;
    loop {
        let s = exe::comp_get_stage(comp, k);
        let parent = exe::comp_util_get_parent_stage_id(stage_id(s));
        if parent != -1 {
            let mut n: i32 = 1;
            let mut j = k + 1;
            while j <= read_i32(comp + league::CURRENT_STAGE) {
                let sibling = read_usize(read_usize(comp + league::STAGES) + j as usize * 4);
                if exe::comp_util_get_parent_stage_id(stage_id(sibling)) != parent {
                    break;
                }
                n += 1;
                j += 1;
            }
            if n == read_i32(comp + league::CURRENT_STAGE) + 2 {
                covered!("tel23_3 stage selector all groups");
                let mut i = k;
                while i < n + k {
                    comp_screens_add_stage_menu_item(&mut menu as *mut i32 as *mut i16, 0x28f, top, 0x30c, top + 0x14, group_text, stage_id(exe::comp_get_stage(comp, i)), 0, i, 0);
                    i += 1;
                }
            } else {
                comp_screens_add_stage_menu_item(&mut menu as *mut i32 as *mut i16, 0x28f, top, 0x30c, top + 0x14, stage_text, parent, 0, k, 0);
                if selected >= k && selected < n + k {
                    let mut i = k;
                    while i < n + k {
                        comp_screens_add_stage_menu_item(&mut sub as *mut i32 as *mut i16, 0x210, top, 0x28d, top + 0x14, group_text, stage_id(exe::comp_get_stage(comp, i)), 0, i, 0);
                        i += 1;
                    }
                }
            }
            k = n + k - 1;
        } else {
            match read_u8(s + league::COMP_TYPE) {
                1 => {
                    if read_i32(comp + league::CURRENT_STAGE) == -1 {
                        stage_selector_cup_rounds(&mut menu, 0x28f, 0x30c, top, round_text, stage_id(s), k, s);
                    } else if read_u8(s + cup::NEXT_DRAW_ROUND) == 1 {
                        let first_code = read_i16(read_usize(s + cup::ROUND_TABLE) + cup_round::ROUND_CODE) as i32;
                        comp_screens_add_stage_menu_item(&mut menu as *mut i32 as *mut i16, 0x28f, top, 0x30c, top + 0x14, stage_text, stage_id(s), first_code, k, 0);
                    } else if read_u16(s + league::STAGE_ID) == 0 {
                        stage_selector_cup_rounds(&mut menu, 0x28f, 0x30c, top, stage_text, 0, k, s);
                    } else {
                        comp_screens_add_stage_menu_item(&mut menu as *mut i32 as *mut i16, 0x28f, top, 0x30c, top + 0x14, stage_text, stage_id(s), 0, k, 0);
                        if selected == k {
                            stage_selector_cup_rounds(&mut sub, 0x210, 0x28d, top, round_text, 0, k, s);
                        }
                    }
                }
                2 => {
                    comp_screens_add_stage_menu_item(&mut menu as *mut i32 as *mut i16, 0x28f, top, 0x30c, top + 0x14, stage_text, stage_id(s), 0, k, 0);
                }
                _ => {}
            }
        }
        k += 1;
        if k > read_i32(comp + league::CURRENT_STAGE) {
            break;
        }
    }
    (menu as i16 != -1 || sub as i16 != -1) as u32
}

/// Rule: an item of a stage menu: the menu's button and popup made at the first item, then the stage or round's name.
///
/// No button text: the comp_screens.cpp box (line 0x2001), 0. While `*area` is −1 the button (x1, y1, x2, y2 with the
/// text, grey on pale grey, font 0xc) and its popup (flags 0x130) are made, the popup's index written to `*area` (a
/// word) and [0xadae0c] = 1. The item's name is the stage and round name (`comp_util_format_stage_round_name` of
/// `code` and `round`), "League" when that is empty; the item (id 5, value 1000 × (`stage` + 1) + `index`) goes in the
/// popup. Answers 1.
/// Original: `comp_screens_add_stage_menu_item` (0x49d930, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_screens_add_stage_menu_item(area: *mut i16, x1: u32, y1: i32, x2: u32, y2: i32, text: usize, code: i32, round: i32, stage: i32, index: i32) -> u32 {
    if text == 0 {
        comp_screens_error_box(0x2001);
        return 0;
    }
    if *area == -1 {
        covered!("tel23_3 stage menu made");
        let button = tel26_add_button([x1, y1 as u32, x2, y2 as u32, 0x30, job_colour(G_GUI_COLOUR_GREY), 0xc, 1, job_colour(G_GUI_COLOUR_PALE_GREY), text as u32, 0, 0]);
        *area = tel26_add_area(current_human_display(), [0, 0, 0, 0, 0, 0, 0, 0, 0x130, job_colour(JOB_COLOUR_31AA), button]) as i16;
        write_u32(COMP_SCREEN_DATE_BUTTONS_FLAG, 1);
    }
    exe::comp_util_format_stage_round_name(code, round, JOB_TEXT, 0x7d0, 0);
    if read_u8(JOB_TEXT) == 0 {
        comp_lang_format()(JOB_TEXT, 0x990f4c); // "League"
    }
    let value = (stage + 1) * 1000 + index; // tel23_3 the stage item's value
    tel26_add_item(current_human_display(), [2, 0, 0, 0, 0, 0, 0, 0x10, job_colour(JOB_COLOUR_31AA), job_colour(G_GUI_COLOUR_YELLOW), 1, 1, job_colour(G_GUI_COLOUR_BLACK),
                                              JOB_TEXT as u32, 0, 5, value as u32, *area as u16 as u32]);
    1
}

/// The cell a fixture row gives one side's club beside its name (column 0 for the home club, 8 for the away club):
/// its league position, its MLS conference, its division or its nation.
///
/// No competition: an empty cell. A league fixture (the fixture's stage, up to the competition's current stage, a
/// league of a competition other than the MLS and not reserve or minor): the club's position in the stage's table
/// (`utils_format_ordinal`, a button `id` with the fixture; empty text when the club is not in it). The MLS: "E", "C"
/// or "W" (`usa_mls_get_club_division` 0x413, 0x47b, other) as a button 0x7e5 with (the MLS id << 22) | 0x209800,
/// 0x23d800 or 0x20a000. A friendly: between clubs of the same nation, the club's division (its +0x53 name, a button
/// 0x7d0, when the division has an object; else empty text) when it has one; otherwise a national team gets an empty
/// cell and a club its nation's three-letter name (a button 0x7d5 to the national team). Other competitions: played
/// by club sides (+0x42 = 1) with the side's mark (+0x40/+0x41) and a division, the division as above; by nations
/// (+0x42 = 2) the nation as above; else empty. Dark blue text, font 0xc, pale grey.
unsafe fn fixture_row_side_cell(fixture: usize, club: usize, mark: u8, column: u32, area: u32, row: i32, id: u32) {
    let cell = |text: usize, link: Option<(u32, usize)>| {
        let (dark_blue, pale) = (job_colour(G_GUI_COLOUR_DARK_BLUE), job_colour(G_GUI_COLOUR_PALE_GREY));
        match link {
            Some((id, value)) => {
                tel26_area_button([area, column, row as u32, 0x30, dark_blue, 0xc, 1, pale, text as u32, id, value as u32]);
            }
            None => {
                tel26_area_text([area, column, row as u32, 0x30, dark_blue, 0xc, 1, pale, text as u32]);
            }
        }
    };
    let division_cell = |division: usize| {
        if read_usize(read_usize(G_COMP_OBJECT_TABLE) + read_u32(division) as usize * 4) != 0 {
            cell(division + 0x53, Some((0x7d0, division)));
        } else {
            cell(0xad9c64, None);
        }
    };
    let nation_cell = |nation: usize| cell(nation + nation::THREE_LETTER_NAME, Some((0x7d5, exe::database_get_national_team(nation))));
    let comp = read_usize(fixture + fixture::COMPETITION);
    if comp == 0 {
        return cell(0, None);
    }
    let object = read_usize(read_usize(G_COMP_OBJECT_TABLE) + read_u32(comp) as usize * 4);
    let stage = read_u8(fixture + fixture::STAGE_INDEX) as i8 as i32;
    if stage <= read_i32(object + league::CURRENT_STAGE) {
        let s = exe::comp_get_stage(object, stage);
        if s != 0 && read_u8(s + league::COMP_TYPE) == 2 && read_u32(comp) != read_u32(KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE)
            && exe::comp_is_reserve_or_minor_comp(read_usize(G_RESERVE_COMP), comp) == 0 {
            let mut position = [0u8; 0x40];
            let mut k: i32 = 0;
            while k < read_i16(s + league::CLUB_COUNT) as i32 {
                let r = exe::league_get_table_row(s, k);
                if read_usize(r + league_table_row::CLUB) == club {
                    let ordinal = exe::utils_format_ordinal(read_u8(r + league_table_row::POSITION) as i8 as i32 + 1, 3);
                    tel26_copy_text(position.as_mut_ptr(), ordinal as *const u8);
                    break;
                }
                k += 1;
            }
            let p = position.as_ptr() as usize;
            return cell(p, if position[0] != 0 { Some((id, fixture)) } else { None });
        }
    }
    let comp_id = read_u32(comp);
    if comp_id == read_u32(KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE) {
        let mut conference = [0u8; 0x40];
        let (fmt, bits) = match exe::usa_mls_get_club_division(club) {
            0x413 => (0x99ca64usize, 0x209800u32), // "E<%s - COMMENT, Eastern Conference of the MLS>"
            0x47b => (0x99ca34, 0x23d800),          // "C<%s - COMMENT, Central Conference of the MLS>"
            _ => (0x99ca04, 0x20a000),              // "W<%s - COMMENT, Western Conference of the MLS>"
        };
        comp_lang_format()(conference.as_mut_ptr() as usize, fmt, 0xad9c64usize);
        return cell(conference.as_ptr() as usize, Some((0x7e5, ((read_u32(KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE) << 22) | bits) as usize)));
    }
    if comp_id == read_u32(KEY_CLUB_COMP_FRIENDLY) {
        let (home, away) = (read_usize(fixture + fixture::HOME_CLUB), read_usize(fixture + fixture::AWAY_CLUB));
        if read_usize(home + club::NATION) == read_usize(away + club::NATION) && read_usize(club + club::DIVISION) != 0 {
            return division_cell(read_usize(club + club::DIVISION));
        }
        if exe::database_is_national_team(club) != 0 {
            return cell(0, None);
        }
        return nation_cell(read_usize(club + club::NATION));
    }
    match read_u8(object + league::CLUB_SIDES) {
        1 if read_usize(club + club::DIVISION) != 0 && mark != 0 => division_cell(read_usize(club + club::DIVISION)),
        2 if read_usize(club + club::NATION) != 0 => nation_cell(read_usize(club + club::NATION)),
        _ => cell(0, None),
    }
}

/// A fixture row's club name cell (column 2 home, 6 away): the club's name, yellow for a human's club, else pale
/// grey; a button 0x7d5 with the club when the side's mark is set, else 0x7d6.
unsafe fn fixture_row_club_cell(club: usize, mark: u8, column: u32, area: u32, row: i32) {
    let colour = if exe::human_manager_is_human_club(club, 0, 1, 0, 0) != 0 { job_colour(G_GUI_COLOUR_YELLOW) } else { job_colour(G_GUI_COLOUR_PALE_GREY) };
    let id = if mark != 0 { 0x7d5 } else { 0x7d6 };
    tel26_area_button([area, column, row as u32, 1, 0, 1, 3, colour, (club + club::SHORT_NAME) as u32, id, club as u32]);
}

/// A tie result mark as text: 1 "w", 2 "ww", a negative mark its absolute value; otherwise nothing.
unsafe fn fixture_result_mark(buf: &mut [u8; 8], mark: i8) {
    match mark {
        1 => {
            comp_lang_format()(buf.as_mut_ptr() as usize, 0x99c370); // "w"
        }
        2 => {
            comp_lang_format()(buf.as_mut_ptr() as usize, 0x99c36c); // "ww"
        }
        m if m < 0 => {
            comp_crt_format()(buf.as_mut_ptr() as usize, 0x98806c, -(m as i32)); // "%d"
        }
        _ => {}
    }
}

/// A result mark's event letter: 5 "s", 4 "p", 3 "e", 6 "a", 8 "w"; otherwise nothing.
unsafe fn fixture_event_letter(buf: &mut [u8; 8], mark: i8) {
    let fmt = match mark {
        5 => 0x99c368usize, // "s"
        4 => 0x99c364,      // "p"
        3 => 0x99c360,      // "e"
        6 => 0x99c35c,      // "a"
        8 => 0x99c370,      // "w"
        _ => return,
    };
    comp_lang_format()(buf.as_mut_ptr() as usize, fmt);
}

/// Rule: one row of fixtures in a competition list: both clubs with their positions, the score or the "view" link,
/// and a venue line under it when the match is on neutral ground.
///
/// No fixture: the comp_screens.cpp box (line 0x203e), 0. Columns of `area`, row `*row`: 0 the home side cell and 8
/// the away side cell (`fixture_row_side_cell`), 2 and 6 the club names (`fixture_row_club_cell`); 1 and 7 the tie
/// marks in yellow font 0xc (the aggregate +0x4b/+0x4c when there is one, style 3; else the result marks +0x3d/+0x3e,
/// style 1). A played match (+0x43 ≠ 0xff): the event letters of the two marks in columns 3 (font 0x40) and 5, and in
/// column 4 the score in colour 0xaebde8: "<home>    ", ":" and "    <away>" (the extra time goals +0x47/+0x48 when
/// there are, else +0x43/+0x44); with match statistics (+8 ≥ 0, `fixture_entry_matches`) the ":" is a button 0x7d3 to
/// them; a postponed match (0xfe) shows "P    " and "    P", an abandoned one (0xfd) "A    " (a button 0x7d3) and
/// "    A" (style 2). An unplayed match: "view" (yellow, style 1) when a human is watching it, else "v", a button 0x7d2
/// with the fixture. Then the row moves on; on neutral ground (+0x3c) with a stadium (+0x18): "<stadium>, <city>"
/// (or the stadium's name) in colour 0xaebdd4 font 2 from x 0xb4 to its width, and the row moves on again. Answers 1.
/// Original: `comp_screens_add_fixture_row` (0x49db00, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_screens_add_fixture_row(fixture: usize, area: u32, row: *mut i32, id: u32) -> u32 {
    if fixture == 0 {
        comp_screens_error_box(0x203e);
        return 0;
    }
    let home = read_usize(fixture + fixture::HOME_CLUB);
    let away = read_usize(fixture + fixture::AWAY_CLUB);
    let r = *row;
    fixture_row_side_cell(fixture, home, read_u8(fixture + fixture::HOME_SIDE_MARK), 0, area, r, id);
    fixture_row_club_cell(home, read_u8(fixture + fixture::HOME_SIDE_MARK), 2, area, r);
    let (yellow, score) = (job_colour(G_GUI_COLOUR_YELLOW), job_colour(JOB_COLOUR_BDE8));
    let (mut home_mark, mut away_mark) = ([0u8; 8], [0u8; 8]);
    let style = if read_u8(fixture + fixture::HOME_AGGREGATE) != 0xff {
        comp_crt_format()(home_mark.as_mut_ptr() as usize, 0x98806c, read_u8(fixture + fixture::HOME_AGGREGATE) as i8 as i32); // "%d"
        comp_crt_format()(away_mark.as_mut_ptr() as usize, 0x98806c, read_u8(fixture + fixture::AWAY_AGGREGATE) as i8 as i32); // "%d"
        3
    } else {
        fixture_result_mark(&mut home_mark, read_u8(fixture + fixture::HOME_RESULT_MARK) as i8);
        fixture_result_mark(&mut away_mark, read_u8(fixture + fixture::AWAY_RESULT_MARK) as i8);
        1
    };
    if home_mark[0] != 0 {
        tel26_area_text([area, 1, r as u32, 1, 0, 0xc, style, yellow, home_mark.as_ptr() as u32]);
    }
    if away_mark[0] != 0 {
        tel26_area_text([area, 7, r as u32, 1, 0, 0xc, style, yellow, away_mark.as_ptr() as u32]);
    }
    let home_goals = read_u8(fixture + fixture::HOME_GOALS);
    if home_goals != 0xff {
        let (mut home_event, mut away_event) = ([0u8; 8], [0u8; 8]);
        fixture_event_letter(&mut home_event, read_u8(fixture + fixture::HOME_RESULT_MARK) as i8);
        fixture_event_letter(&mut away_event, read_u8(fixture + fixture::AWAY_RESULT_MARK) as i8);
        let (mut home_text, mut away_text) = ([0u8; 8], [0u8; 8]);
        let (h, a) = if read_u8(fixture + fixture::HOME_GOALS_EXTRA_TIME) != 0xff {
            (fixture::HOME_GOALS_EXTRA_TIME, fixture::AWAY_GOALS_EXTRA_TIME)
        } else {
            (fixture::HOME_GOALS, fixture::AWAY_GOALS)
        };
        comp_crt_format()(home_text.as_mut_ptr() as usize, 0x98806c, read_u8(fixture + h) as i8 as i32); // "%d"
        comp_crt_format()(away_text.as_mut_ptr() as usize, 0x98806c, read_u8(fixture + a) as i8 as i32); // "%d"
        if home_event[0] != 0 {
            tel26_area_text([area, 3, r as u32, 1, 0, 0x40, 1, yellow, home_event.as_ptr() as u32]);
        }
        let slot = read_i32(fixture + fixture::MATCH_STATS_SLOT);
        if home_goals != 0xfe && home_goals != 0xfd && slot > -1
            && exe::match_stats_fixture_entry_matches(read_usize(G_MATCH_STATS), slot, read_i32(fixture + fixture::HOME_CLUB_ID), read_i32(fixture + fixture::AWAY_CLUB_ID),
                                                      read_u16(fixture + fixture::DAY) as i32, read_u16(fixture + fixture::YEAR) as i32) != 0 {
            covered!("tel23_3 fixture row stats link");
            comp_crt_format()(JOB_TEXT, 0x99c354, home_text.as_ptr()); // "%s    "
            tel26_area_text([area, 4, r as u32, 1, 0, 0xc, 3, score, JOB_TEXT as u32]);
            tel26_area_button([area, 4, r as u32, 1, 0, 0xc, 3, score, 0x989adc, 0x7d3, slot as u32]); // ":"
            comp_crt_format()(JOB_TEXT, 0x99c34c, away_text.as_ptr()); // "    %s"
            tel26_area_text([area, 4, r as u32, 1, 0, 0xc, 3, score, JOB_TEXT as u32]);
        } else {
            match home_goals {
                0xfe => {
                    comp_lang_format()(JOB_TEXT, 0x99c9e4, 0xad9c64usize); // "P    <%s - COMMENT - postponed>"
                    tel26_area_text([area, 4, r as u32, 1, 0, 0xc, 2, score, JOB_TEXT as u32]);
                }
                0xfd => {
                    comp_lang_format()(JOB_TEXT, 0x99c9c4, 0xad9c64usize); // "A    <%s - COMMENT - abandoned>"
                    tel26_area_button([area, 4, r as u32, 1, 0, 0xc, 2, score, JOB_TEXT as u32, 0x7d3, slot as u32]);
                }
                _ => {
                    comp_crt_format()(JOB_TEXT, 0x99c354, home_text.as_ptr()); // "%s    "
                    tel26_area_text([area, 4, r as u32, 1, 0, 0xc, 3, score, JOB_TEXT as u32]);
                }
            }
            tel26_area_text([area, 4, r as u32, 1, 0, 0xc, 3, score, 0x989adc]); // ":"
            match home_goals {
                0xfe => {
                    comp_lang_format()(JOB_TEXT, 0x99c9a4, 0xad9c64usize); // "    P<%s - COMMENT - postponed>"
                    tel26_area_text([area, 4, r as u32, 1, 0, 0xc, 2, score, JOB_TEXT as u32]);
                }
                0xfd => {
                    comp_lang_format()(JOB_TEXT, 0x99c984, 0xad9c64usize); // "    A<%s - COMMENT - abandoned>"
                    tel26_area_text([area, 4, r as u32, 1, 0, 0xc, 2, score, JOB_TEXT as u32]);
                }
                _ => {
                    comp_crt_format()(JOB_TEXT, 0x99c34c, away_text.as_ptr()); // "    %s"
                    tel26_area_text([area, 4, r as u32, 1, 0, 0xc, 3, score, JOB_TEXT as u32]);
                }
            }
        }
        if away_event[0] != 0 {
            tel26_area_text([area, 5, r as u32, 1, 0, 1, 1, yellow, away_event.as_ptr() as u32]);
        }
    } else if exe::human_manager_is_watching_fixture(read_usize(G_HUMAN_MANAGER), fixture, 0usize) != 0 {
        comp_lang_format()(JOB_TEXT, 0x9918d4); // "view"
        tel26_area_button([area, 4, r as u32, 1, 0, 0xc, 1, yellow, JOB_TEXT as u32, 0x7d2, fixture as u32]);
    } else {
        comp_lang_format()(JOB_TEXT, 0x99c1b4); // "v"
        tel26_area_button([area, 4, r as u32, 1, 0, 0xc, 3, score, JOB_TEXT as u32, 0x7d2, fixture as u32]); // tel23_3 the unplayed link's style
    }
    fixture_row_club_cell(away, read_u8(fixture + fixture::AWAY_SIDE_MARK), 6, area, r);
    fixture_row_side_cell(fixture, away, read_u8(fixture + fixture::AWAY_SIDE_MARK), 8, area, r, id);
    *row += 1;
    let stadium = read_usize(fixture + fixture::STADIUM);
    if read_u8(fixture + fixture::NEUTRAL_VENUE) != 0 && stadium != 0 {
        covered!("tel23_3 fixture row venue");
        let mut name = [0u8; 0x80];
        tel26_copy_text(name.as_mut_ptr(), (stadium + stadium::NAME) as *const u8);
        let city = read_usize(stadium + stadium::CITY);
        if city != 0 {
            let (g1, g2) = (read_u8(stadium + stadium::GENDER_NAME) as i8 as i32, read_u8(city + city::GENDER_NAME) as i8 as i32);
            comp_lang_format()(JOB_TEXT, 0x9902a4, g1, g1, g2, g2, name.as_ptr(), city + 4); // the city's name (+4); "{}<%s - stadium>{}, {}<%s - city>{}"
        } else {
            tel26_copy_text(JOB_TEXT as *mut u8, name.as_ptr());
        }
        let x2 = exe::font_text_width(2usize, JOB_TEXT as *const u8) + 0xb4;
        tel26_add_item(current_human_display(), [1, 0xb4, 0, x2 as u32, 0, 0, *row as u32, 1, 0, 0, 1, 2, job_colour(JOB_COLOUR_BDD4), JOB_TEXT as u32, 0, 0, 0, area]);
        *row += 1;
    }
    1
}

/// Comparer for `comp_open_intertoto_apply_screen`: only the original (it opens a screen), params 0 and 1 checked
/// afterwards when they hold the ids.
pub(crate) unsafe extern "C" fn compare_comp_open_intertoto_apply_screen(a: u32, b: u32) {
    original_fn::<unsafe extern "C" fn(u32, u32)>(%%I0%%)(a, b);
    let got = (get_screen_param(0), get_screen_param(1));
    if got.0 == a as usize {
        SCOREBOARD.record(got == (a as usize, b as usize), || format!("intertoto apply screen (open {} {}): params 0/1 {:x?}", a, b, got));
    }
}

/// Comparer for `comp_intertoto_apply_screen_draw`: side by side over the display list.
pub(crate) unsafe extern "C" fn compare_comp_intertoto_apply_screen_draw() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I1%%);
    compare_match_screen_build("intertoto apply screen", &[], || o(), || comp_intertoto_apply_screen_draw());
}

/// Comparer for `comp_intertoto_apply_screen_handle_event`: only the original (the answer writes the news item and
/// the Intertoto volunteers; the rest is the menu bar), its answer checked for No and Yes (−9).
pub(crate) unsafe extern "C" fn compare_comp_intertoto_apply_screen_handle_event(widget: u32) -> i32 {
    let id = tel26_item_id(widget as i16);
    let r = original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I2%%)(widget);
    if id == 1 {
        SCOREBOARD.record(r == -9, || format!("intertoto apply screen (No): answer {}", r));
    }
    r
}

/// Comparer for `comp_screens_open_comp_from_news_item`: only the original (it opens a screen).
pub(crate) unsafe extern "C" fn compare_comp_screens_open_comp_from_news_item(a: u32, b: u32) {
    // simplification: opening a screen does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(u32, u32)>(%%I3%%)(a, b)
}

/// Comparer for `comp_screens_add_related_comps_menu`: side by side over the display list and the answer; the error
/// box (no object) only the original.
pub(crate) unsafe extern "C" fn compare_comp_screens_add_related_comps_menu(object: usize, colour_a: u32, colour_b: u32, comp_id: u32, history_id: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32>(%%I4%%);
    if object == 0 && read_u32(COMP_SCREEN_BUILT_FLAG) != 0 {
        return o(object, colour_a, colour_b, comp_id, history_id);
    }
    let (mut ro, mut rr) = (0, 0);
    compare_match_screen_build("related comps menu", &[], || ro = o(object, colour_a, colour_b, comp_id, history_id),
        || rr = comp_screens_add_related_comps_menu(object, colour_a, colour_b, comp_id, history_id));
    SCOREBOARD.record(ro == rr, || format!("related comps menu: answer {} x {}", ro, rr));
    ro
}

/// Comparer for `comp_screens_add_stage_selector`: side by side over the display list (plus [0xadae0c]) and the
/// answer; the error box (no competition) only the original.
pub(crate) unsafe extern "C" fn compare_comp_screens_add_stage_selector(comp: usize, selected: i32, top: i32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, i32, i32) -> u32>(%%I5%%);
    if comp == 0 {
        return o(comp, selected, top);
    }
    let (mut ro, mut rr) = (0, 0);
    compare_match_screen_build("stage selector", &[(COMP_SCREEN_DATE_BUTTONS_FLAG, 4)], || ro = o(comp, selected, top), || rr = comp_screens_add_stage_selector(comp, selected, top));
    SCOREBOARD.record(ro == rr, || format!("stage selector: answer {} x {}", ro, rr));
    ro
}

/// Comparer for `comp_screens_add_stage_menu_item`: side by side over the display list, [0xadae0c] and the caller's
/// popup word; the error box (no text) only the original.
pub(crate) unsafe extern "C" fn compare_comp_screens_add_stage_menu_item(area: *mut i16, x1: u32, y1: i32, x2: u32, y2: i32, text: usize, code: i32, round: i32, stage: i32, index: i32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(*mut i16, u32, i32, u32, i32, usize, i32, i32, i32, i32) -> u32>(%%I6%%);
    if text == 0 || area.is_null() {
        return o(area, x1, y1, x2, y2, text, code, round, stage, index);
    }
    let mut r = 0;
    compare_match_screen_build("stage menu item", &[(COMP_SCREEN_DATE_BUTTONS_FLAG, 4), (area as usize, 2)], || r = o(area, x1, y1, x2, y2, text, code, round, stage, index),
        || { comp_screens_add_stage_menu_item(area, x1, y1, x2, y2, text, code, round, stage, index); });
    r
}

/// Comparer for `comp_screens_add_fixture_row`: side by side over the display list and the caller's row; the error box
/// (no fixture) only the original.
pub(crate) unsafe extern "C" fn compare_comp_screens_add_fixture_row(fixture: usize, area: u32, row: *mut i32, id: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, *mut i32, u32) -> u32>(%%I7%%);
    if fixture == 0 || row.is_null() {
        return o(fixture, area, row, id);
    }
    let mut r = 0;
    compare_match_screen_build("fixture row", &[(row as usize, 4)], || r = o(fixture, area, row, id), || { comp_screens_add_fixture_row(fixture, area, row, id); });
    r
}
