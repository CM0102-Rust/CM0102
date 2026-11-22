// ---------- batch %%LEVA%%: the Game Credits screen (its own sections) and the Web Sites rows (contract_screens.cpp) ----------

/// The credits list state, shared by every section builder of the Game Credits screen (and, for the row and the
/// area, by the Web Sites screen): the line counter [0xae1868] (a word: the low word of a section's popup value),
/// the row counter [0xae186c], the list area [0xae1870] (a word; the rows per page in the byte [0xae1872]), the
/// "View" popup area [0xae1874], the right-column flag of the two-column sections [0xae1878], the page being built
/// [0xae187c] and the page shown [0xae187e] (both words; screen parameter 2).
const CREDITS_LINE: usize = 0xae1868;
const CREDITS_ROW: usize = 0xae186c;
const CREDITS_AREA: usize = 0xae1870;
const CREDITS_AREA_ROWS: usize = 0xae1872;
const CREDITS_POPUP_AREA: usize = 0xae1874;
const CREDITS_RIGHT_COLUMN: usize = 0xae1878;
const CREDITS_PAGE: usize = 0xae187c;
const CREDITS_SHOWN_PAGE: usize = 0xae187e;
/// The colour of a credit's role (the right column of the name/role rows) [0xaebdde].
const CREDITS_ROLE_COLOUR: usize = 0xaebdde;
/// The credits state, as a region for the comparers.
const CREDITS_STATE: (usize, usize) = (CREDITS_LINE, 0x18);

/// `langlib_sprintf` (0x66f4e0) and `crt_sprintf` (0x944c9f): destination, format, the arguments.
type Tel34FormatFn = unsafe extern "C" fn(usize, usize, ...) -> i32;

/// The text buffer of the screens formatted from `format` translated to the game's language.
unsafe fn tel34_lang_text(format: usize) {
    let f: Tel34FormatFn = core::mem::transmute(FN_LANGLIB_SPRINTF);
    f(JOB_TEXT, format);
}

/// The text buffer of the screens formatted from `format` as it is (not translated).
unsafe fn tel34_crt_text(format: usize) {
    let f: Tel34FormatFn = core::mem::transmute(FN_CRT_SPRINTF);
    f(JOB_TEXT, format);
}

/// Copies a C string into the screens' text buffer (the original's inline `strcpy`).
unsafe fn tel34_copy_to_text(src: usize) {
    tel26_copy_text(JOB_TEXT as *mut u8, src as *const u8);
}

/// The page being built is the page shown.
unsafe fn tel34_credits_on_page() -> bool {
    read_u16(CREDITS_PAGE) == read_u16(CREDITS_SHOWN_PAGE)
}

/// The line counter moves on by `n` (a word).
unsafe fn tel34_credits_line_add(n: u16) {
    write_u16(CREDITS_LINE, read_u16(CREDITS_LINE).wrapping_add(n));
}

/// The row counter moves on by one; returns the row before.
unsafe fn tel34_credits_next_row() -> u32 {
    let row = read_u32(CREDITS_ROW);
    write_u32(CREDITS_ROW, row.wrapping_add(1));
    row
}

/// A text row cell of the list: `Display::add_item` of kind 1 from x1 to x2 at `row`, flags 1, font 3, the colour
/// word at `colour` and the text, in the list area.
unsafe fn tel34_credits_cell(x1: u32, x2: u32, row: u32, colour: usize, text: usize) {
    tel26_add_item(current_human_display(), [1, x1, 0, x2, 0, 0, row, 1, 0, 0, 1, 3, job_colour(colour), text as u32, 0, 0, 0,
        read_u16(CREDITS_AREA) as u32]);
}

/// An empty row across the list (the row counter moves on).
unsafe fn tel34_credits_blank_row() {
    let row = tel34_credits_next_row();
    tel34_credits_cell(0x82, 0x2f8, row, JOB_COLOUR_3184, 0xad9c64);
}

/// The heading rows of a section (the text buffer): the text as wide as it is in font 3 (`font_text_width`) from
/// 0x82 in the heading colour [0xaebdec], and under it a line item (flags 0xc400) as wide; the row counter moves on
/// by two.
unsafe fn tel34_credits_title_rows() {
    let width = exe::font_text_width(3usize, JOB_TEXT as *const u8) as u32;
    let row = tel34_credits_next_row();
    tel34_credits_cell(0x82, width.wrapping_add(0x82), row, JOB_COLOUR_BDEC, JOB_TEXT);
    let width = exe::font_text_width(3usize, JOB_TEXT as *const u8) as u32;
    let row = tel34_credits_next_row();
    tel26_add_item(current_human_display(), [1, 0x82, 0, width.wrapping_add(0x82), 0, 0, row, 0xc400, job_colour(JOB_COLOUR_BDEC),
        0, 0, 0, 0, 0, 0, 0, 0, read_u16(CREDITS_AREA) as u32]);
}

/// The gap before a credits section: the line counter moves on and, on the page shown, an empty row.
unsafe fn tel34_credits_section_gap() {
    tel34_credits_line_add(1);
    if tel34_credits_on_page() {
        tel34_credits_blank_row();
    }
}

/// The section's entry in the "View" popup (the text buffer): ticked (0x2002, no id) when the list's selected row
/// (screen parameter 0) is the current row and the page is the one shown; otherwise an item of kind 2 with id 100
/// and the value page << 16 | line (what `credits_screen_handle_action` opens). Both in the popup colours.
unsafe fn tel34_credits_popup_entry() {
    let selected = get_screen_param(0) as u32;
    let ticked = read_u32(CREDITS_ROW) == selected && tel34_credits_on_page();
    let (kind, id, value) = if ticked {
        (0x2002, 0, 0)
    } else {
        (2, 0x64, (read_u16(CREDITS_PAGE) as u32) << 16 | read_u16(CREDITS_LINE) as u32)
    };
    tel26_add_item(current_human_display(), [kind, 0, 0, 0, 0, 0, 0, 0x10, job_colour(JOB_COLOUR_31AA), job_colour(JOB_COLOUR_BDEC), 1, 1,
        job_colour(JOB_COLOUR_BDEA), JOB_TEXT as u32, 0, id, value, read_u16(CREDITS_POPUP_AREA) as u32]);
}

/// The heading of a section (the text buffer): on the page shown the heading rows; the line counter moves on by two.
unsafe fn tel34_credits_heading() {
    if tel34_credits_on_page() {
        tel34_credits_title_rows();
    }
    tel34_credits_line_add(2);
}

/// A name of a two-column section: on the page shown it goes in the left column (0x82..0x1b8) or in the right one
/// (0x1c2..0x2f8, the row counter moving on), and the column flips. Then, for any name but the last, the line counter
/// moves on when the flag is back on the left column; after the last one an unpaired left name closes its row (the
/// row counter moves on) and the line counter moves on.
unsafe fn tel34_credits_column_name(name: usize, last: bool) {
    if tel34_credits_on_page() {
        if read_u32(CREDITS_RIGHT_COLUMN) == 0 {
            tel34_credits_cell(0x82, 0x1b8, read_u32(CREDITS_ROW), JOB_COLOUR_3184, name);
        } else {
            let row = tel34_credits_next_row();
            tel34_credits_cell(0x1c2, 0x2f8, row, JOB_COLOUR_3184, name);
        }
        write_u32(CREDITS_RIGHT_COLUMN, 1u32.wrapping_sub(read_u32(CREDITS_RIGHT_COLUMN)));
    }
    let right = read_u32(CREDITS_RIGHT_COLUMN) != 0;
    if !last {
        if !right {
            tel34_credits_line_add(1);
        }
        return;
    }
    if right {
        tel34_credits_next_row();
    }
    tel34_credits_line_add(1);
}

/// A name and its role (the role already in the text buffer): the line counter moves on and, on the page shown, the
/// name on the left (0x82..0x1b8) and the role on the right (0x1c2..0x2f8, in the role colour [0xaebdde]); the row
/// counter moves on.
unsafe fn tel34_credits_name_and_role(name: usize) {
    tel34_credits_line_add(1);
    if tel34_credits_on_page() {
        tel34_credits_cell(0x82, 0x1b8, read_u32(CREDITS_ROW), JOB_COLOUR_3184, name);
        let row = tel34_credits_next_row();
        tel34_credits_cell(0x1c2, 0x2f8, row, CREDITS_ROLE_COLOUR, JOB_TEXT);
    }
}

/// A text by the game's language (`langlib_get_language`): 2 French, 4 Italian, 5 Spanish, 6 Danish, 9 Portuguese,
/// 10 Swedish, any other English (the original's jump table over language − 2).
unsafe fn tel34_by_language(english: usize, french: usize, italian: usize, spanish: usize, danish: usize, portuguese: usize, swedish: usize) -> usize {
    match exe::langlib_get_language() {
        2 => french,
        4 => italian,
        5 => spanish,
        6 => danish,
        9 => portuguese,
        10 => swedish,
        _ => english,
    }
}

/// The external testers of the Additional Testing section, in the original's order.
const ADDITIONAL_TESTERS: [usize; 56] = [
    0x9b4e78, 0x9b4e64, 0x9b4e54, 0x9b4e3c, 0x9b4e2c, 0x9b4e18,
    0x9b4e08, 0x9b4804, 0x9b4dec, 0x9b4ddc, 0x9b4dcc, 0x9b4db8,
    0x9b4da4, 0x9b4d90, 0x9b4d78, 0x9b4d68, 0x9b34c4, 0x9b34b8,
    0x9b34a8, 0x9b3494, 0x9b3488, 0x9b347c, 0x9b345c, 0x9b4d50,
    0x9b4d38, 0x9b4d28, 0x9b4d18, 0x9b4d04, 0x9b4cf8, 0x9b4cec,
    0x9b4cd8, 0x9b4ccc, 0x9b4cbc, 0x9b4ca8, 0x9b4c94, 0x9b4c7c,
    0x9b4c64, 0x9b4c58, 0x9b4c48, 0x9b4c30, 0x9b4c1c, 0x9b4c0c,
    0x9b4bfc, 0x9b2c78, 0x9b4bec, 0x9b4bdc, 0x9b4bcc, 0x9b4bb4,
    0x9b4ba4, 0x9b4b90, 0x9b4b84, 0x9b4b74, 0x9b4b64, 0x9b4b58,
    0x9b4b44, 0x9b4b30,
];

/// Rule: the Additional Testing section of the credits: its heading and the 56 external testers in two columns.
///
/// The section gap, "Additional Testing" (translated) as the section's "View" popup entry and heading, the column flag
/// reset to the left, then each tester of `ADDITIONAL_TESTERS` as a two-column name (`tel34_credits_column_name`, the
/// last one closing the section). Off the page shown only the counters move.
/// Original: `credits_add_additional_testing_rows` (0x50e000, contract_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn credits_add_additional_testing_rows() {
    covered!("tel34_1 additional testing");
    tel34_credits_section_gap();
    tel34_lang_text(0x9b4e88); // "Additional Testing"
    tel34_credits_popup_entry();
    tel34_credits_heading();
    write_u32(CREDITS_RIGHT_COLUMN, 0);
    for (k, &name) in ADDITIONAL_TESTERS.iter().enumerate() {
        tel34_credits_column_name(name, k == ADDITIONAL_TESTERS.len() - 1);
    }
}

/// The name/role rows of the Special Thanks section shown in English only (language 0), the roles not translated.
const SPECIAL_THANKS_ENGLISH: [(usize, usize); 16] = [
    (0x9b5220, 0x9b5210), (0x9b5220, 0x9b5200), (0x9b51ec, 0x99aa30), (0x9b51cc, 0x9b51bc), (0x9b51ac, 0x9b51a0),
    (0x9b5190, 0x9b5184), (0x9b517c, 0x9b515c), (0x9b517c, 0x9b513c), (0x9b517c, 0x9b5120), (0x9b517c, 0x9b5110),
    (0x9b50f4, 0x9b50e4), (0x9b50c4, 0x9b2940), (0x9b50a8, 0x9b5098), (0x9b5084, 0x9b185c), (0x9b5068, 0x9b5050),
    (0x9b5068, 0x9b5030),
];

/// Rule: the Special Thanks section of the credits: its heading and the name/role rows, some only in English, two
/// with the name in the game's language.
///
/// The section gap, "Special Thanks" (translated) as the section's popup entry and heading; then the rows
/// (`tel34_credits_name_and_role`, the role translated unless said): Webmaster @ SIgames.com, CMzine Editor, CMzine
/// proof reader; in English only (`langlib_get_language` 0) the 16 rows of `SPECIAL_THANKS_ENGLISH` (roles not
/// translated: PR Guru's ×2, For being 'Safe', Top articles and office help, Man City photos, Manchester City, Umbro ×4,
/// Research and quiz questions, Scottish non league research, Teaboy and player histories, Player histories,
/// Stevengage Borough photos ×2); Written Player Histories; in English only For extra curricular stuff (not translated);
/// Manual Design; Maintaining Sanity with "The Sponge Ball" in the game's language; For all your support with "Every CM
/// Fan" in the game's language.
/// Original: `credits_add_special_thanks_rows` (0x511320, contract_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn credits_add_special_thanks_rows() {
    covered!("tel34_1 special thanks");
    tel34_credits_section_gap();
    tel34_lang_text(0x9b5294); // "Special Thanks"
    tel34_credits_popup_entry();
    tel34_credits_heading();
    for (role, name) in [(0x9b527cusize, 0x9b526cusize), (0x9b525c, 0x9b5248), (0x9b5234, 0x9b522c)] {
        tel34_lang_text(role);
        tel34_credits_name_and_role(name);
    }
    if exe::langlib_get_language() == 0 {
        covered!("tel34_1 special thanks in english");
        for (role, name) in SPECIAL_THANKS_ENGLISH {
            tel34_crt_text(role);
            tel34_credits_name_and_role(name);
        }
    }
    tel34_lang_text(0x9b5014); // "Written Player Histories"
    tel34_credits_name_and_role(0x9b5008);
    if exe::langlib_get_language() == 0 {
        tel34_crt_text(0x9b4fec); // "For extra curricular stuff"
        tel34_credits_name_and_role(0x9b4fdc);
    }
    tel34_lang_text(0x9b4fcc); // "Manual Design"
    tel34_credits_name_and_role(0x9b4fc4);
    tel34_lang_text(0x9b4fb0); // "Maintaining Sanity"
    tel34_credits_name_and_role(tel34_by_language(0x9b4f38, 0x9b4f58, 0x9b4f9c, 0x9b4f88, 0x9b4f48, 0x9b4f78, 0x9b4f6c));
    tel34_lang_text(0x9b4f20); // "For all your support"
    tel34_credits_name_and_role(tel34_by_language(0x9b4e9c, 0x9b4ebc, 0x9b4f0c, 0x9b4ef4, 0x9b4eac, 0x9b4ee0, 0x9b4ed0));
}

/// Rule: the Best Wishes To section of the credits: its heading (no popup entry) and three couples "On getting married".
///
/// The section gap, "Best Wishes To" (translated) as the heading, then "On getting married" (translated) as the role of
/// Woodg and Carol, Honorino and Lorena, and Alby and Seve.
/// Original: `credits_add_best_wishes_rows` (0x512d40, contract_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn credits_add_best_wishes_rows() {
    covered!("tel34_1 best wishes");
    tel34_credits_section_gap();
    tel34_lang_text(0x9b52ec); // "Best Wishes To"
    tel34_credits_heading();
    for name in [0x9b52c8usize, 0x9b52b4, 0x9b52a4] {
        tel34_lang_text(0x9b52d8); // "On getting married"
        tel34_credits_name_and_role(name);
    }
}

/// The scrolling list of a credits-like screen: the rows per page = `height` / font 3's height (signed), kept in
/// [0xae1872], and the list area (x1 0x6e, y1 `y1`, to 0x30c/0x217, one column, `rows`, 2, no owner) in [0xae1870].
unsafe fn tel34_credits_list_area(height: i32, y1: u32) {
    let rows = height.wrapping_div(exe::font_get_height(3)) as u8;
    write_u8(CREDITS_AREA_ROWS, rows);
    let area = tel26_add_area(current_human_display(), [0x6e, y1, 0x30c, 0x217, 1, 0, rows as u32, 0, 2, 0, u32::MAX]);
    write_u16(CREDITS_AREA, area as u16);
}

/// The screen title of the credits and web sites screens: the game title (`get_game_title` 0x839570) in the heading
/// colours 0 and 1 (`get_heading_colour`).
unsafe fn tel34_game_title() {
    let title: unsafe extern "C" fn() -> usize = core::mem::transmute(0x839570usize);
    let text = title();
    let second = exe::get_heading_colour(1) as u32;
    let first = exe::get_heading_colour(0) as u32;
    exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, first, second, text);
}

/// After the list is built: the list scrolls to the selected row (screen parameter 0, `Area::set_scroll_row`) when
/// there is an area, and parameter 1 keeps the area index.
unsafe fn tel34_credits_list_done() {
    let selected = get_screen_param(0);
    let area = read_u16(CREDITS_AREA) as i16;
    if area != -1 {
        exe::area_set_scroll_row(display_area_at(current_human_display(), area), selected);
    }
    set_screen_param(1, area as isize as usize);
}

/// Rule: building the Game Credits screen: the title, the buttons, the "View" popup and the scrolling list with every
/// credits section, page 0 then page 1, showing the page of parameter 2.
///
/// The line counter and both pages zeroed, the page shown = parameter 2; the background of picture section 10
/// (`background_set_section_picture` 0x41c8e0, 10, 0); the game title; the menu bar (4, 0); the Back/Next buttons;
/// the subtitle "Game Credits"; the "View" button (0x6e,0x91..0xeb,0xa5) and its popup area (flags 0x130, in
/// [0xae1874]); the list area (0x15d / the font height rows, from y 0xaa); the row counter zeroed. Page 0: Sports
/// Interactive, JASK/Xbox, Research, 0x4e92f0, 0x4eb480, 0x4eb110 and 0x4eccd0 (by address); then page 1 with the line
/// counter zeroed: Additional Research (0x4eda90), Additional Testing, Special Thanks and Best Wishes To; the list
/// scrolled to parameter 0 and parameter 1 = the area.
/// Original: `credits_screen_build` (0x513180, contract_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn credits_screen_build() {
    covered!("tel34_1 credits screen");
    write_u16(CREDITS_SHOWN_PAGE, 0);
    write_u16(CREDITS_PAGE, 0);
    write_u16(CREDITS_LINE, 0);
    write_u16(CREDITS_SHOWN_PAGE, get_screen_param(2) as u16);
    let section_picture: unsafe extern "C" fn(u32, u32) -> u32 = core::mem::transmute(0x41c8e0usize);
    section_picture(10, 0);
    tel34_game_title();
    exe::menubar_build(4, 0);
    tel26_back_next();
    tel34_lang_text(0x9b52fc); // "Game Credits"
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, JOB_TEXT, 0);
    tel34_lang_text(0x988004); // "View"
    let button = tel26_add_button([0x6e, 0x91, 0xeb, 0xa5, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 1, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 0, 0]);
    let popup = tel26_add_area(current_human_display(), [0, 0, 0, 0, 0, 0, 0, 0, 0x130, job_colour(JOB_COLOUR_31AA), button]);
    write_u16(CREDITS_POPUP_AREA, popup as u16);
    tel34_credits_list_area(0x15d, 0xaa);
    write_u32(CREDITS_ROW, 0);
    let section = |address: usize| {
        let f: unsafe extern "C" fn() = core::mem::transmute(address);
        f();
    };
    for address in [0x4e5970usize, 0x4e6a70, 0x4e6dc0, 0x4e92f0, 0x4eb480, 0x4eb110, 0x4eccd0] {
        section(address);
    }
    write_u16(CREDITS_PAGE, 1);
    write_u16(CREDITS_LINE, 0);
    section(0x4eda90);
    credits_add_additional_testing_rows();
    credits_add_special_thanks_rows();
    credits_add_best_wishes_rows();
    tel34_credits_list_done();
}

/// Rule: a click on the Game Credits screen keeps the list's scroll row and, on a "View" popup entry (id 100), opens
/// that section: its page and line become the shown page and the selected row (-11, rebuild).
///
/// The command id 0x42b is cleared first. For an item (not -1) parameter 0 = the list's scroll row
/// (`ScreenManager::get_area_scroll_row` of the area in parameter 1); the item's id (or the command id). Id 100: the
/// item's value (0 for -1): parameter 0 = its low word, parameter 2 = its high word, -11. Otherwise the menu bar
/// (`menubar_handle_command`, then `menubar_handle_object_link` when it gives 0).
/// Original: `credits_screen_handle_action` (0x5133c0, contract_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn credits_screen_handle_action(raw: u32) -> i32 {
    if read_u16(G_CURRENT_COMMAND_ID) == 0x42b {
        write_u16(G_CURRENT_COMMAND_ID, 0);
    }
    let item = raw as i16;
    if item != -1 {
        let row = exe::screenmanager_get_area_scroll_row(G_SCREEN_MANAGER, get_screen_param(1));
        set_screen_param(0, row);
    }
    if tel26_item_id(item) == 100 {
        covered!("tel34_1 credits view");
        let value = tel26_item_value(item);
        set_screen_param(0, (value & 0xffff) as usize);
        set_screen_param(2, (value >> 16) as usize);
        return -11;
    }
    let r = exe::menubar_handle_command(raw);
    if r != 0 { r } else { exe::menubar_handle_object_link(raw) }
}

/// Rule: opening the Web Sites screen (builder 0x513a20, handler 0x49cc10, flag 0); when it opens, parameters 0 and
/// 1 are cleared.
/// Original: `web_sites_screen_open` (0x5134c0, contract_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn web_sites_screen_open() {
    covered!("tel34_1 open web sites");
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x513a20usize, 0x49cc10usize, 0, 0usize, 0) != 0 {
        set_screen_param(0, 0);
        set_screen_param(1, 0);
    }
}

/// A web site row (its label already in the text buffer): the address across the list (0x82..0x2f8) and the label
/// on the right (0x1c2..0x2f8, in the role colour); the row counter moves on.
unsafe fn tel34_web_site_row(address: usize) {
    tel34_credits_cell(0x82, 0x2f8, read_u32(CREDITS_ROW), JOB_COLOUR_3184, address);
    let row = tel34_credits_next_row();
    tel34_credits_cell(0x1c2, 0x2f8, row, CREDITS_ROLE_COLOUR, JOB_TEXT);
}

/// Rule: the publisher rows of the Web Sites list: the heading "Eidos Interactive" (not translated), "The publishers"
/// with www.eidos.co.uk, and an empty row.
/// Original: `web_sites_add_publisher_rows` (0x513500, contract_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn web_sites_add_publisher_rows() {
    covered!("tel34_1 publisher rows");
    tel34_copy_to_text(0x9b1b80); // "Eidos Interactive"
    tel34_credits_title_rows();
    tel34_lang_text(0x9b530c); // "The publishers"
    tel34_web_site_row(0x9b531c); // "www.eidos.co.uk"
    tel34_credits_blank_row();
}

/// Rule: the fan club rows of the Web Sites list, read from CM3_DATA\fancluburls.txt: an empty row, the heading
/// "Unofficial Sites" and one row per "http://" link of the file.
///
/// The file opened "r" (`utils_build_file_path` + `fopen`); none: nothing. Each character read (`fgetc` 0x9469e0)
/// goes into a 256-byte buffer (at most 255 kept, always closed); once 7 or more are in, the last seven compared to
/// "http://" without case (`stricmp`): on a match the buffer restarts with the link, read up to EOF or a character up
/// to 0x20 (that character lost), then the rest of the line up to '\n' or EOF is read into a second buffer that is
/// never used (characters below 0x20 skipped), and the link becomes a row across the list (the row counter moves on);
/// the scan goes on writing the first buffer from the length of that unused rest (one index register in the
/// original). EOF ends; the file is closed.
/// Original: `web_sites_add_fan_club_links` (0x513750, contract_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn web_sites_add_fan_club_links() {
    let path = exe::utils_build_file_path(0x9b5348usize, 0x9b5354usize); // "CM3_DATA", "fancluburls.txt"
    let file = exe::crt_fopen(path, 0x9899fcusize); // "r"
    if file == 0 {
        return;
    }
    covered!("tel34_1 fan club links");
    tel34_credits_blank_row();
    tel34_lang_text(0x9b5334); // "Unofficial Sites"
    tel34_credits_title_rows();
    let mut link = [0u8; 0x100];
    let mut rest = [0u8; 0x100];
    let mut n = 0usize;
    let push = |buf: &mut [u8; 0x100], n: &mut usize, c: u8| {
        buf[*n] = c;
        *n = (*n + 1).min(0xff);
        buf[*n] = 0;
    };
    link[0] = 0;
    let mut c = exe::at_9469e0(file);
    while c != -1 {
        push(&mut link, &mut n, c as u8);
        if n >= 7 && exe::crt_stricmp(link.as_ptr().wrapping_add(n - 7) as usize, 0x9b532cusize) == 0 {
            covered!("tel34_1 fan club link");
            n = 0;
            link[0] = 0;
            let mut c = exe::at_9469e0(file);
            while c != -1 && c as u8 > 0x20 {
                push(&mut link, &mut n, c as u8);
                c = exe::at_9469e0(file);
            }
            let mut m = 0usize;
            let mut c = exe::at_9469e0(file);
            while c != -1 && c as u8 != b'\n' {
                if c as u8 >= 0x20 {
                    push(&mut rest, &mut m, c as u8);
                }
                c = exe::at_9469e0(file);
            }
            let row = tel34_credits_next_row();
            tel34_credits_cell(0x82, 0x2f8, row, JOB_COLOUR_3184, link.as_ptr() as usize);
            // the original keeps one index register: the link buffer goes on from the length of the unused rest
            n = m;
        }
        c = exe::at_9469e0(file);
    }
    exe::crt_fclose(file);
}

/// Comparer for `credits_add_additional_testing_rows`: side by side over the display list and the credits state.
pub(crate) unsafe extern "C" fn compare_credits_add_additional_testing_rows() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I0%%);
    compare_by_display_list_with("Game Credits screen (credits_add_additional_testing_rows)", &[CREDITS_STATE], || o(),
        || credits_add_additional_testing_rows());
}

/// Comparer for `credits_add_special_thanks_rows`: side by side over the display list and the credits state.
pub(crate) unsafe extern "C" fn compare_credits_add_special_thanks_rows() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I1%%);
    compare_by_display_list_with("Game Credits screen (credits_add_special_thanks_rows)", &[CREDITS_STATE], || o(),
        || credits_add_special_thanks_rows());
}

/// Comparer for `credits_add_best_wishes_rows`: side by side over the display list and the credits state.
pub(crate) unsafe extern "C" fn compare_credits_add_best_wishes_rows() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I2%%);
    compare_by_display_list_with("Game Credits screen (credits_add_best_wishes_rows)", &[CREDITS_STATE], || o(),
        || credits_add_best_wishes_rows());
}

/// Comparer for `credits_screen_build`: the Game Credits screen side by side over the display list and the credits
/// state.
pub(crate) unsafe extern "C" fn compare_credits_screen_build() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I3%%);
    compare_by_display_list_with("Game Credits screen (credits_screen_build)", &[CREDITS_STATE], || o(), || credits_screen_build());
}

/// Comparer for `credits_screen_handle_action`.
pub(crate) unsafe extern "C" fn compare_credits_screen_handle_action(raw: u32) -> i32 {
    // simplification: the menu bar commands (they open screens) do not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I4%%)(raw)
}

/// Comparer for `web_sites_screen_open`.
pub(crate) unsafe extern "C" fn compare_web_sites_screen_open() {
    // simplification: opening a screen does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn()>(%%I5%%)()
}

/// Comparer for `web_sites_add_publisher_rows`: side by side over the display list and the row state.
pub(crate) unsafe extern "C" fn compare_web_sites_add_publisher_rows() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I6%%);
    compare_by_display_list_with("Web Sites screen (web_sites_add_publisher_rows)", &[CREDITS_STATE], || o(), || web_sites_add_publisher_rows());
}

/// Comparer for `web_sites_add_fan_club_links`: side by side over the display list and the row state (the file is
/// only read, once by each side).
pub(crate) unsafe extern "C" fn compare_web_sites_add_fan_club_links() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I7%%);
    compare_by_display_list_with("Web Sites screen (web_sites_add_fan_club_links)", &[CREDITS_STATE], || o(), || web_sites_add_fan_club_links());
}
