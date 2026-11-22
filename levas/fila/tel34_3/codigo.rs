// ---------- batch %%LEVA%%: the ban appeal confirm screen (discipline.cpp) ----------

/// Rule: the "Appeal" button of a ban news item opens the appeal confirm screen and copies the ban out of the news
/// item into its parameters.
///
/// A local `NewsItem` (`NewsItem::NewsItem`, 0, 0) and a staff name reference with its `Date` (`Date::Date` at +0x10)
/// are set up; the screen opens (`open_screen`: builder 0x55ea10, handler 0x55ed00, single instance, no leave/exit);
/// when it does not, nothing else. Otherwise the item `news_id` of `manager`'s inbox is fetched (`News::find_item_by_id`
/// on [0xb7431c], its answer not checked), its slots 0..14 read in order (`NewsItem::get_slot`), the slot index moved on
/// by two more, and the banned person read from the next slots (`read_staff_name_ref_from_news`, his staff id at +0xc).
/// Parameters: 0 slot 0 (signed byte), 1..3 the name ids (+0x33) of the person's first, second and common name records,
/// 4 his staff id, 5 slot 2 (signed byte), 6 slot 3 (signed word), 8 and 9 the low words of slots 4 and 5, 10..15
/// slots 6, 7, 8, 9, 10 and 11 (signed bytes), 0x10 slot 12, 0x11 the news id, 0x12 and 0x13 slots 13 and 14 (signed
/// bytes), 0x14 slot 1; parameter 7 is not written.
/// Original: `discipline_appeal_news_action` (0x55e620, discipline.cpp, cdecl).
pub(crate) unsafe extern "C" fn discipline_appeal_news_action(manager: usize, news_id: u32) {
    let find: unsafe extern "thiscall" fn(usize, usize, u32, *mut u8) -> u32 = core::mem::transmute(0x786b70usize);
    let mut item = [0u8; 0xe0];
    let mut person = [0u8; 0x18];
    let mut slot: u8 = 0;
    exe::new_newsitem(item.as_mut_ptr() as usize, 0, 0);
    exe::date_date_54a110(person.as_mut_ptr() as usize + 0x10);
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x55ea10usize, 0x55ed00usize, 1, 0usize, 0) == 0 {
        return;
    }
    covered!("tel34_3 open appeal");
    find(read_usize(0xb7431c), manager, news_id, item.as_mut_ptr());
    let mut s = [0u32; 15];
    for v in s.iter_mut() {
        let k = slot;
        slot = slot.wrapping_add(1);
        *v = exe::newsitem_get_slot(item.as_ptr() as usize, k as u32);
    }
    slot = slot.wrapping_add(2);
    exe::read_staff_name_ref_from_news(item.as_ptr() as usize, &mut slot, person.as_mut_ptr());
    let staff = staff_at(read_u32(person.as_ptr() as usize + 0xc) as i32 as isize);
    let byte = |v: u32| v as u8 as i8 as isize as usize;
    set_screen_param(0, byte(s[0]));
    set_screen_param(1, read_usize(staff.first_name() + names::ID));
    set_screen_param(2, read_usize(staff.second_name() + names::ID));
    set_screen_param(3, read_usize(staff.common_name() + names::ID));
    set_screen_param(4, read_usize(staff.addr()));
    set_screen_param(5, byte(s[2]));
    set_screen_param(6, s[3] as u16 as i16 as isize as usize);
    set_screen_param(8, (s[4] & 0xffff) as usize);
    set_screen_param(9, (s[5] & 0xffff) as usize);
    for (k, v) in [(0xau32, s[6]), (0xb, s[7]), (0xc, s[8]), (0xd, s[9]), (0xe, s[10]), (0xf, s[11])] {
        set_screen_param(k, byte(v));
    }
    set_screen_param(0x10, s[12] as usize);
    set_screen_param(0x11, news_id as usize);
    set_screen_param(0x12, byte(s[13]));
    set_screen_param(0x13, byte(s[14]));
    set_screen_param(0x14, s[1] as usize);
}

/// Rule: the ban appeal confirm screen: the banned person's picture, the human manager's name in the club's colours,
/// "Please Confirm", the question (a game ban, a day ban or a plain ban) and the No and Yes buttons.
///
/// Parameters 1..3 are read and not used; the person = the staff record of parameter 4, the games = parameter 5
/// (signed byte), the days = parameter 6 (signed word); the club = his club (`staff::CLUB_JOB`) or, with none, the
/// first club of the table. The person's background (`background_set_staff_picture` 0x41c790, 0, 0); the menu bar
/// (4, 0); the title: the slot's human manager's name (2000 bytes) in the club's kit colours; the subtitle "Please
/// Confirm"; the person's name (500 bytes). Games > 0: the number as "%d" and, in Swedish (language 10) with one game,
/// the Swedish text (not translated, 0x9c36c0, with the name), otherwise "Appeal against the <n> game ban that has been
/// placed on <name>"; else days > 0: "... the <n> day ban ..."; else "Appeal against the ban that has been placed on
/// <name>" (the translated texts with their 100-char limits). The text in a text area (0x6e,0x91..0x30c,0x217); a
/// two-column area (weights 3/1) with "No" (id 1) and "Yes" (id 2).
/// Original: `discipline_appeal_dialog_build` (0x55ea10, discipline.cpp, cdecl).
pub(crate) unsafe extern "C" fn discipline_appeal_dialog_build() {
    let crt: Tel34FormatFn = core::mem::transmute(FN_CRT_SPRINTF);
    let weights: [u8; 2] = [3, 1];
    let mut name = [0u8; 500];
    let mut number = [0u8; 500];
    covered!("tel34_3 appeal dialog");
    get_screen_param(1);
    get_screen_param(2);
    get_screen_param(3);
    let person = get_screen_param(4);
    let games = get_screen_param(5) as u8 as i8;
    let days = get_screen_param(6) as u16 as i16;
    let staff = staff_at(person as i32 as isize);
    let club = if read_usize(staff.addr() + staff::CLUB_JOB) != 0 { read_usize(staff.addr() + staff::CLUB_JOB) } else { read_usize(G_CLUB_TABLE) };
    let staff_picture: unsafe extern "C" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x41c790usize);
    staff_picture(staff.addr(), 0, 0);
    exe::menubar_build(4, 0);
    let (mut fore, mut back) = (0u32, 0u32);
    exe::database_get_club_kit_colours(club, 1, &mut fore, &mut back);
    exe::database_format_staff_name(tel26_slot_manager(), JOB_TEXT, 0x7d0, 1);
    exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, back, fore, JOB_TEXT);
    tel34_lang_text(0x9909b8); // "Please Confirm"
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, JOB_TEXT, 1);
    exe::database_format_staff_name(staff.addr(), name.as_mut_ptr() as usize, 0x1f4, 1);
    let lang: Tel34FormatFn = core::mem::transmute(FN_LANGLIB_SPRINTF);
    if games > 0 {
        crt(number.as_mut_ptr() as usize, 0x98806c, games as i32); // "%d"
        if exe::langlib_get_language() == 10 && games == 1 {
            covered!("tel34_3 appeal swedish one game");
            crt(JOB_TEXT, 0x9c36c0, name.as_ptr());
        } else {
            lang(JOB_TEXT, 0x9c364c, 100, 100, number.as_ptr(), name.as_ptr());
        }
    } else if days > 0 {
        crt(number.as_mut_ptr() as usize, 0x98806c, days as i32); // "%d"
        lang(JOB_TEXT, 0x9c35d8, 100, 100, number.as_ptr(), name.as_ptr());
    } else {
        lang(JOB_TEXT, 0x9c3584, 100, name.as_ptr());
    }
    tel26_text_area([0x6e, 0x91, 0x30c, 0x217, 2, 0, 0x10, 4, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 0]);
    let area = tel26_add_area(current_human_display(), [0x64, 0x22b, 0x316, 0x24e, 2, weights.as_ptr() as u32, 1, 0, 1, 0, u32::MAX]);
    tel34_lang_text(0x98e8cc); // "No"
    tel26_area_button([area, 0, 0, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 3, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 1, 0]);
    tel34_lang_text(0x98e8d0); // "Yes"
    tel26_area_button([area, 1, 0, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 3, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 2, 0]);
}

/// Rule: "No" (id 1) closes the appeal screen (-9); "Yes" (id 2) submits the appeal with the ban in the screen
/// parameters, marks the news item answered and closes (-9); other ids go to the menu bar.
///
/// The news id = parameter 0x11 (read first). Yes: the person = the staff record of parameter 4, the slot's human
/// manager; `MANAGER_MANAGER::submit_ban_appeal` (0x6ab800 on [0xb67a44], thiscall, 11 arguments) with the person,
/// parameter 0x12 (byte), then four dwords packed from the bytes of parameters 10..13 as the original's frame holds them
/// (10|11|12|13, 11|12|13, 12|13, 13; the bytes above them, stack leftovers in the original, zero here), parameter 0x13
/// (byte), parameter 5 (byte), parameter 6, the dword of the handler's own argument with its low byte replaced by
/// parameter 0xe, and parameter 0x14; then `News::set_item_slot` (manager, news id, slot 0, value 1, 0).
/// Original: `discipline_appeal_dialog_handle_click` (0x55ed00, discipline.cpp, cdecl).
pub(crate) unsafe extern "C" fn discipline_appeal_dialog_handle_click(raw: u32) -> i32 {
    type SubmitBanAppeal = unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32, u32, u32, usize, u32, usize) -> u8;
    let news_id = get_screen_param(0x11);
    match tel26_item_id(raw as i16) {
        1 => -9,
        2 => {
            covered!("tel34_3 appeal submitted");
            let person = staff_at(get_screen_param(4) as i32 as isize);
            let manager = tel26_slot_manager();
            let b = |k: u32| get_screen_param(k) as u8 as u32;
            let human = b(0x12);
            let (p10, p11, p12, p13) = (b(0xa), b(0xb), b(0xc), b(0xd));
            let p14 = b(0xe);
            let p19 = b(0x13);
            let p5 = b(5);
            let p6 = get_screen_param(6);
            let p20 = get_screen_param(0x14);
            let submit: SubmitBanAppeal = core::mem::transmute(0x6ab800usize);
            submit(read_usize(0xb67a44), person.addr(), human, p10 | p11 << 8 | p12 << 16 | p13 << 24, p11 | p12 << 8 | p13 << 16, p12 | p13 << 8, p13,
                p19, p5, p6, (raw & 0xffff_ff00) | p14, p20);
            let set_slot: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(FN_NEWS_SET_ITEM_SLOT);
            set_slot(read_usize(0xb7431c), manager.addr(), news_id, 0, 1, 0);
            -9
        }
        _ => {
            let r = exe::menubar_handle_command(raw);
            if r != 0 { r } else { exe::menubar_handle_object_link(raw) }
        }
    }
}

/// Comparer for `discipline_appeal_news_action`.
pub(crate) unsafe extern "C" fn compare_discipline_appeal_news_action(manager: usize, news_id: u32) {
    // simplification: opening a screen does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(usize, u32)>(%%I0%%)(manager, news_id)
}

/// Comparer for `discipline_appeal_dialog_build`: the appeal screen side by side over the display list.
pub(crate) unsafe extern "C" fn compare_discipline_appeal_dialog_build() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I1%%);
    compare_by_display_list_with("Ban appeal screen (discipline_appeal_dialog_build)", &[], || o(), || discipline_appeal_dialog_build());
}

/// Comparer for `discipline_appeal_dialog_handle_click`.
pub(crate) unsafe extern "C" fn compare_discipline_appeal_dialog_handle_click(raw: u32) -> i32 {
    // simplification: the appeal and the menu bar commands do not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I2%%)(raw)
}
