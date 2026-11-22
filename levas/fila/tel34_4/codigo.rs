// ---------- batch %%LEVA%%: the FA Cup fifth round entry confirm screen (eng_fa_cup.cpp) ----------

/// The size of a competition record of `G_CLUB_COMP_TABLE` (107 bytes).
const CLUB_COMP_RECORD_SIZE: usize = 0x6b;

/// `gui_add_text_area` (0x5ed070, cdecl) with one link triple: x1, y1, x2, y2, two colours, the height, the font, the
/// text colour, the text, then the link (its text, its length, its object) and the closing 0.
unsafe fn tel34_text_area_with_link(f: [u32; 10], link_text: usize, link_len: u32, object: usize) {
    type TextAreaLink = unsafe extern "C" fn(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize, u32, usize, u32);
    let g: TextAreaLink = core::mem::transmute(FN_GUI_ADD_TEXT_AREA);
    g(f[0], f[1], f[2], f[3], f[4], f[5], f[6], f[7], f[8], f[9], link_text, link_len, object, 0);
}

/// Rule: the news item asking whether the club enters the FA Cup at the fifth round opens the confirm screen and keeps
/// the news item and its slot.
///
/// `open_screen` (builder 0x570940, handler 0x570ae0, single instance, no leave/exit); when it opens, parameter 0 =
/// the first argument (the manager the news belongs to) and parameter 1 = the second (the news id).
/// Original: `fa_cup_open_fifth_round_entry_screen` (0x5708f0, eng_fa_cup.cpp, cdecl).
pub(crate) unsafe extern "C" fn fa_cup_open_fifth_round_entry_screen(manager: usize, news_id: usize) {
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x570940usize, 0x570ae0usize, 1, 0usize, 0) != 0 {
        covered!("tel34_4 open fa cup fifth round");
        set_screen_param(0, manager);
        set_screen_param(1, news_id);
    }
}

/// Rule: the "Enter the FA Cup at fifth round ?" confirm screen, with No and Yes.
///
/// The competition = the FA Cup's record (`KEY_CLUB_COMP_ENGLISH_FA_CUP` in `G_CLUB_COMP_TABLE`, 0x6b bytes each):
/// its background (`background_set_comp_picture`, 0, 0); the menu bar (4, 0); the title: its name in its colours
/// (`database_get_comp_colours`); the subtitle "Please Confirm"; the question (translated, with the gender char +0x37
/// twice and the name) in a text area (0x6e,0x91..0x30c,0x217) where the name links to the competition (2000 chars);
/// a two-column area (weights 3/1) with "No" (id 2) on the left and "Yes" (id 1) on the right.
/// Original: `fa_cup_draw_fifth_round_entry_screen` (0x570940, eng_fa_cup.cpp, cdecl).
pub(crate) unsafe extern "C" fn fa_cup_draw_fifth_round_entry_screen() {
    let lang: Tel34FormatFn = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let weights: [u8; 2] = [3, 1];
    covered!("tel34_4 fa cup fifth round");
    let comp = read_usize(G_CLUB_COMP_TABLE).wrapping_add((read_i32(KEY_CLUB_COMP_ENGLISH_FA_CUP) as usize).wrapping_mul(CLUB_COMP_RECORD_SIZE));
    exe::background_set_comp_picture(comp, 0, 0);
    exe::menubar_build(4, 0);
    let (mut first, mut second) = (0u32, 0u32);
    exe::database_get_comp_colours(comp, &mut first, &mut second);
    let name = comp + 4;
    exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, second, first, name);
    tel34_lang_text(0x9909b8); // "Please Confirm"
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, JOB_TEXT, 1);
    let gender = read_u8(comp + competition::GENDER_NAME) as i8 as i32;
    lang(JOB_TEXT, 0x9c4304, gender, gender, name); // "Enter the {}<%s - competition>{} at fifth round ?"
    tel34_text_area_with_link([0x6e, 0x91, 0x30c, 0x217, 2, 0, 0x10, 4, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32], name, 0x7d0, comp);
    let area = tel26_add_area(current_human_display(), [0x64, 0x22b, 0x316, 0x24e, 2, weights.as_ptr() as u32, 1, 0, 1, 0, u32::MAX]);
    tel34_lang_text(0x98e8cc); // "No"
    tel26_area_button([area, 0, 0, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 3, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 2, 0]);
    tel34_lang_text(0x98e8d0); // "Yes"
    tel26_area_button([area, 1, 0, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 3, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 1, 0]);
}

/// Rule: "Yes" (id 1) enters the club at the fifth round (the FA Cup object's `eng_fa_cup::FIFTH_ROUND_QUESTION_ASKED`
/// = 2) and answers the news item with 2; "No" (id 2) answers it with 3; both close (-9); other ids go to the menu bar.
///
/// Parameters 0 (the manager) and 1 (the news id) are read first; the answer = `News::set_item_slot` (manager, news
/// id, slot 0, the value, 0). The FA Cup object = the competition object table [`G_COMP_OBJECT_TABLE`] at
/// `KEY_CLUB_COMP_ENGLISH_FA_CUP`.
/// Original: `fa_cup_handle_fifth_round_entry_screen` (0x570ae0, eng_fa_cup.cpp, cdecl).
pub(crate) unsafe extern "C" fn fa_cup_handle_fifth_round_entry_screen(raw: u32) -> i32 {
    let manager = get_screen_param(0);
    let news_id = get_screen_param(1);
    let set_slot: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(FN_NEWS_SET_ITEM_SLOT);
    match tel26_item_id(raw as i16) {
        1 => {
            covered!("tel34_4 fa cup fifth round yes");
            let cup = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add((read_i32(KEY_CLUB_COMP_ENGLISH_FA_CUP) as usize).wrapping_mul(4)));
            write_u8(cup + eng_fa_cup::FIFTH_ROUND_QUESTION_ASKED, 2);
            set_slot(read_usize(0xb7431c), manager, news_id, 0, 2, 0);
            -9
        }
        2 => {
            set_slot(read_usize(0xb7431c), manager, news_id, 0, 3, 0);
            -9
        }
        _ => {
            let r = exe::menubar_handle_command(raw);
            if r != 0 { r } else { exe::menubar_handle_object_link(raw) }
        }
    }
}

/// Comparer for `fa_cup_open_fifth_round_entry_screen`.
pub(crate) unsafe extern "C" fn compare_fa_cup_open_fifth_round_entry_screen(manager: usize, news_id: usize) {
    // simplification: opening a screen does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(usize, usize)>(%%I0%%)(manager, news_id)
}

/// Comparer for `fa_cup_draw_fifth_round_entry_screen`: the confirm screen side by side over the display list.
pub(crate) unsafe extern "C" fn compare_fa_cup_draw_fifth_round_entry_screen() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I1%%);
    compare_by_display_list_with("FA Cup fifth round entry screen (fa_cup_draw_fifth_round_entry_screen)", &[], || o(),
        || fa_cup_draw_fifth_round_entry_screen());
}

/// Comparer for `fa_cup_handle_fifth_round_entry_screen`.
pub(crate) unsafe extern "C" fn compare_fa_cup_handle_fifth_round_entry_screen(raw: u32) -> i32 {
    // simplification: the answer to the news item and the menu bar commands do not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I2%%)(raw)
}
