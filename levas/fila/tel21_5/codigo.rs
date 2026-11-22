// ---------- batch %%LEVA%%: the search screens (5): building the search Filters screen (search_screens.cpp) ----------

/// The function-static colour of the Filters screen: bit 0 of the guard byte, then the pixel word
/// (`graphics_rgb_to_pixel(0, 0xc0, 0, 0)`), the colour of an option that is not chosen.
const FILTERS_STATIC_GUARD: usize = 0xdd6cc4;
const FILTERS_STATIC_GREEN: usize = 0xdd6cc6;
/// Colour words of the screen items not named in globals.rs (the headings, the minus/plus buttons).
const FILTERS_COLOUR_3150: usize = 0xae3150;
const FILTERS_COLOUR_3154: usize = 0xae3154;
/// The shared text buffer and the empty argument of the "<%s - COMMENT ...>" texts.
const FILTERS_TEXT: usize = 0xde1f64;
const FILTERS_NO_COMMENT: usize = 0xad9c64;
/// The default general flag mask of a search (`SearchFilters` +0 / +4): with it the Clear button is plain text.
const FILTERS_DEFAULT_MASK: [u32; 2] = [0x14140081, 0x44aaa884];
/// The fields of a `SearchFilters` block the screen reads.
#[allow(dead_code)]
mod search_filters_block {
    /// u32 x2: the general flag mask (low, high).
    pub(super) const MASK_LOW: usize = 0x0;
    pub(super) const MASK_HIGH: usize = 0x4;
    /// i32: the chosen nationality nation (flag 0x10).
    pub(super) const NATIONALITY_NATION: usize = 0x18;
    /// i8: the chosen nationality region (flag 0x20).
    pub(super) const NATIONALITY_REGION: usize = 0x1c;
    /// i8: the chosen language (1..3, flag 0x80000).
    pub(super) const LANGUAGE: usize = 0x1d;
    /// i32: the chosen based nation (flag 0x800000).
    pub(super) const BASED_NATION: usize = 0x1e;
    /// i8: the chosen based region (flag 0x1000000).
    pub(super) const BASED_REGION: usize = 0x22;
    /// i8: the chosen value band (-1..7, flag 0x8000000).
    pub(super) const VALUE_BAND: usize = 0x23;
    /// i8: the chosen staff job (3..9, high flag 0x1000).
    pub(super) const JOB: usize = 0x24;
    /// i8 x2: the player attribute ranges set and the number to match; then the non-player ones.
    pub(super) const PLAYER_RANGES_SET: usize = 0x63;
    pub(super) const PLAYER_RANGES_TO_MATCH: usize = 0x64;
    pub(super) const STAFF_RANGES_SET: usize = 0x7d;
    pub(super) const STAFF_RANGES_TO_MATCH: usize = 0x7e;
    /// The search text (0x19 bytes; high flag 0x400000).
    pub(super) const TEXT: usize = 0x7f;
}
/// The search list types: a staff search, a match-watching scout's list.
const FILTERS_LIST_STAFF: u8 = 3;
const FILTERS_LIST_SCOUT: u8 = 4;
/// Functions called by address: `search_screens_filter_bit_to_index` (cdecl(lo, hi) -> char), `draw_pitch`,
/// `key_nation_sorted_order`, `get_regions_sorted_by_name`, `database_get_language_name`, `Cash::set_from_preset_index`.
const FILTERS_FN_BIT_TO_INDEX: usize = 0x81b0e0;
const FILTERS_FN_KEY_NATION_SORTED_ORDER: usize = 0x66a0c0;
const FILTERS_FN_REGIONS_SORTED_BY_NAME: usize = 0x545dc0;
const FILTERS_FN_LANGUAGE_NAME: usize = 0x53cee0;
const FILTERS_FN_CASH_SET_FROM_PRESET: usize = 0x43f620;
/// The key nations table (`G_LEAGUE_TABLE`, 0x48 bytes per key nation; +0 the nation): 0x23 entries.
const FILTERS_KEY_NATION_SIZE: usize = 0x48;
const FILTERS_KEY_NATION_COUNT: usize = 0x23;
/// A nation's byte the menus need above 8 to list it [inference: its database coverage level].
const FILTERS_NATION_LEVEL: usize = 0x85;
/// A nation's EU membership byte (2 = member; the "EU National" option) and its languages.
const FILTERS_NATION_EU: usize = 0x7f;
const FILTERS_NATION_LANGUAGES: [usize; 3] = [0x77, 0x78, 0x79];
/// The sort comparers of the attribute lists (tel21_3), by address as the original passes them.
const FILTERS_FN_SORT_STAFF_ATTRIBUTES: usize = 0x81b040;
const FILTERS_FN_SORT_PLAYER_ATTRIBUTES: usize = 0x81afa0;
/// `database_get_player_attribute_name` / `database_get_non_player_attribute_name` (cdecl(id, text, size, short)).
const FILTERS_FN_PLAYER_ATTRIBUTE_NAME: usize = 0x5428a0;
const FILTERS_FN_NON_PLAYER_ATTRIBUTE_NAME: usize = 0x543b10;

/// The screen being built: the filters, the session's current search, the key nation and the seven column layouts
/// that share the body rectangle.
struct FiltersBuild {
    filters: usize,
    staff: bool,
    list_type: u8,
    manager: Staff,
    nation: usize,
    /// The body areas (Display::add_area, low 16 bits): 2,1,1,1,1,1,1 / 8,4,4,4,7,1,4 / 8,4,4,4,4,7,1 / 8,4,4,7,1,4,4
    /// (7 columns), 6,1,1,1,1,6,1,1,1 / 6,1,1,1,1,5,1,2,1 (9 columns), 8,4,4,1,15,0,0.
    age: u32,
    nationality: u32,
    based: u32,
    job: u32,
    attributes: u32,
    attributes_match: u32,
    text: u32,
}

/// `langlib_sprintf` into the shared text without and with one argument.
unsafe fn filters_text(fmt: usize) {
    let f: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    f(FILTERS_TEXT, fmt);
}
unsafe fn filters_text1(fmt: usize, a: usize) {
    let f: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    f(FILTERS_TEXT, fmt, a);
}
/// `crt_sprintf(out, fmt, a)`.
unsafe fn filters_format(out: usize, fmt: usize, a: usize) {
    let f: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    f(out, fmt, a);
}
/// `strcpy` into the shared text.
unsafe fn filters_copy_text(src: usize) {
    tel26_copy_text(FILTERS_TEXT as *mut u8, src as *const u8);
}
/// The index of a one-bit filter mask (`search_screens_filter_bit_to_index`), sign-extended.
unsafe fn filters_bit_index(lo: u32, hi: u32) -> u32 {
    let f: unsafe extern "C" fn(u32, u32) -> i8 = core::mem::transmute(FILTERS_FN_BIT_TO_INDEX);
    f(lo, hi) as i32 as u32
}
/// `Display::add_item` on the current human's display.
unsafe fn filters_item(f: DisplayItemFields) -> u32 {
    tel26_add_item(current_human_display(), f)
}
/// `Display::add_area` on the current human's display; the area index is the low 16 bits.
unsafe fn filters_area(f: [u32; 11]) -> u32 {
    tel26_add_area(current_human_display(), f) & 0xffff
}
/// A popup menu attached to item `owner` (`Display::add_area` with only the flags and the menu colour).
unsafe fn filters_menu(flags: u32, owner: u32) -> u32 {
    tel26_add_area(current_human_display(), [0, 0, 0, 0, 0, 0, 0, 0, flags, job_colour(JOB_COLOUR_31AA), owner])
}
/// One entry of a popup menu (`Display::add_item` flags 2, style 0x10): the text, the id and its value.
unsafe fn filters_menu_entry(text: usize, id: u32, value: u32, menu: u32) -> u32 {
    let c = job_colour;
    filters_item([2, 0, 0, 0, 0, 0, 0, 0x10, c(JOB_COLOUR_31AA), c(JOB_COLOUR_BDEC), 1, 1, c(JOB_COLOUR_BDEA), text as u32, 0, id, value, menu])
}
/// A separator line of a popup menu.
unsafe fn filters_menu_separator(menu: u32) {
    let c = job_colour;
    filters_item([1, 0, 0, 0, 0, 0, 0, 0x1000010, c(JOB_COLOUR_31AA), c(JOB_COLOUR_BDEC), 1, 1, c(JOB_COLOUR_BDEA), 0, 0, 0, 0, menu]);
}
/// A row heading: the shared text in column 0 (`add_area_text(area, 0, row, 1, 0, 1, 3, colour 0xae3154, text)`).
unsafe fn filters_heading(area: u32, row: u32) {
    tel26_area_text([area, 0, row, 1, 0, 1, 3, job_colour(FILTERS_COLOUR_3154), FILTERS_TEXT as u32]);
}
/// A shown choice: the shared text in yellow at `col` (`add_area_text(.., 1, 0, 1, 2, yellow, text)`).
unsafe fn filters_choice(area: u32, col: u32, row: u32) {
    tel26_area_text([area, col, row, 1, 0, 1, 2, job_colour(JOB_COLOUR_BDEC), FILTERS_TEXT as u32]);
}
/// The text of a chosen option at `col`: a yellow item (flags 6, style 0x221) with `id` and `value`.
unsafe fn filters_chosen(area: u32, col: u32, row: u32, id: u32, value: u32) {
    let y = job_colour(JOB_COLOUR_BDEC);
    filters_item([6, 0, 0, 0, 0, col, row, 0x221, y, y, 0xc, 2, y, FILTERS_TEXT as u32, 0, id, value, area]);
}
/// An option that can be clicked at `col`: a green button with `id` and `value`.
unsafe fn filters_option_button(area: u32, col: u32, row: u32, id: u32, value: u32) {
    tel26_area_button([area, col, row, 1, 0, 0xc, 2, job_colour(FILTERS_STATIC_GREEN), FILTERS_TEXT as u32, id, value]);
}
/// The usual option: id 10 with the index of its flag bit; chosen (`set`) it is the yellow item, otherwise the green
/// button.
unsafe fn filters_option(area: u32, col: u32, row: u32, set: bool, lo: u32, hi: u32) {
    let index = filters_bit_index(lo, hi);
    if set {
        filters_chosen(area, col, row, 10, index);
    } else {
        filters_option_button(area, col, row, 10, index);
    }
}
/// The button that opens a popup menu at `col` (`add_area_button(.., 0x30, black, 0xc, 1, pale grey, no text, 0, 0)`)
/// and the menu (flags 0x500030) attached to it.
unsafe fn filters_menu_button(area: u32, col: u32, row: u32) -> u32 {
    let c = job_colour;
    let b = tel26_area_button([area, col, row, 0x30, c(JOB_COLOUR_BDEA), 0xc, 1, c(JOB_COLOUR_3184), 0, 0, 0]);
    filters_menu(0x500030, b)
}

/// Builds the search Filters screen of the session's current search (params: 0 the session, 1 the filters being
/// edited, 2 the page: 1 General, 2 Attributes).
///
/// The function-static green (`FILTERS_STATIC_GREEN`) is made on the first call. The current slot's human manager is
/// needed (none: the "Error" box of search_screens.cpp line 0x104e) and so is his nation: his club's nation, else his
/// national side's nation, else his own (none: line 0x105d). The screen: no background picture, the whole-screen area,
/// a frame 0x6b..0x30d and centred vertically (0x15e high for a staff search, 500 otherwise) in `rgb(0, 0x38, 0)`, the
/// title "Filters for <search name>", the General / Attributes toggle (the page shown is plain text, the other a button
/// with id 1 and the page as value), Clear (plain text while the general mask is the default, else a button with id 2),
/// Cancel (id 8) and Ok (id 9), the pitch drawn behind the body (`draw_pitch`) and the seven column layouts of the body
/// (7 rows for a staff search and 14 otherwise; the 9-column ones 7 or 16).
///
/// The Attributes page (`filters_screen_attributes_page`) and the General page (`filters_screen_general_page`) follow.
/// Original: search_screens_build_filters_screen (0x8133a0, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_build_filters_screen() {
    covered!("tel21_5 build the filters screen");
    let c = job_colour;
    let guard = read_u8(FILTERS_STATIC_GUARD);
    if guard & 1 == 0 {
        write_u8(FILTERS_STATIC_GUARD, guard | 1);
        write_u16(FILTERS_STATIC_GREEN, exe::graphics_rgb_to_pixel(0, 0xc0, 0, 0) as u16);
    }
    let manager = tel26_slot_manager();
    if manager.addr() == 0 {
        tel26_error_box(SEARCH_SCREENS_CPP, 0x104e);
        return;
    }
    let club = manager.club_job();
    let nation = if !club.is_null() && club.nation().addr() != 0 {
        club.nation().addr()
    } else if !manager.national_job().is_null() {
        manager.national_job().nation().addr()
    } else {
        manager.nation().addr()
    };
    if nation == 0 {
        tel26_error_box(SEARCH_SCREENS_CPP, 0x105d);
        return;
    }
    let session = get_screen_param(0);
    let filters = get_screen_param(1);
    let page = get_screen_param(2) as u8;
    exe::background_set_picture(0usize);
    tel26_add_area(current_human_display(), [0, 0, 0x31f, 0x257, 1, 0, 1, 0, 2, 0, u32::MAX]);
    let search = search_session_current(session);
    let list_type = read_u8(search + player_search_fields::LIST_TYPE);
    let staff = list_type == FILTERS_LIST_STAFF;
    let (height, rows) = if staff { (0x15e, 7) } else { (500, 0xe) };
    let mut y1: i32 = (600 - height) / 2;
    let mut y2: i32 = y1 + height - 1;
    let frame = exe::graphics_rgb_to_pixel(0, 0x38, 0, 0);
    tel26_add_area(current_human_display(), [0x6b, y1 as u32, 0x30d, y2 as u32, 1, 0, 1, 0, 0x30, frame, u32::MAX]);
    y1 += 10;
    y2 -= 10;
    filters_text1(0xa8019c, search_session_current(session) + search_filters_fields::NAME); // "Filters for <%s - search name>"
    let text_item: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_SCREENMANAGER_ADD_TEXT_ITEM);
    text_item(G_SCREEN_MANAGER, 0x75, y1 as u32, 0x303, (y1 + 0x2d) as u32, 0x30, c(JOB_COLOUR_BDEA), 0xc, 4, c(JOB_COLOUR_3184), FILTERS_TEXT as u32, 0);
    y1 += 0x37;
    let toggle = filters_area([0x75, y1 as u32, 0x16f, (y1 + 0x14) as u32, 2, 0, 1, 0, 1, 0, u32::MAX]);
    filters_text1(0xa8016c, FILTERS_NO_COMMENT); // "General<..>"
    if page == 1 {
        tel26_area_text([toggle, 0, 0, 0x70, c(JOB_COLOUR_317E), 0xc, 1, c(JOB_COLOUR_3184), FILTERS_TEXT as u32]);
    } else {
        tel26_area_button([toggle, 0, 0, 0x30, c(JOB_COLOUR_317E), 0xc, 1, c(JOB_COLOUR_3184), FILTERS_TEXT as u32, 1, 1]);
    }
    filters_text1(0xa80138, FILTERS_NO_COMMENT); // "Attributes<..>"
    if page == 2 {
        tel26_area_text([toggle, 1, 0, 0x70, c(JOB_COLOUR_317E), 0xc, 1, c(JOB_COLOUR_3184), FILTERS_TEXT as u32]);
    } else {
        tel26_area_button([toggle, 1, 0, 0x30, c(JOB_COLOUR_317E), 0xc, 1, c(JOB_COLOUR_3184), FILTERS_TEXT as u32, 1, 2]);
    }
    filters_text1(0xa80110, FILTERS_NO_COMMENT); // "Clear<..>"
    if read_u32(filters) == FILTERS_DEFAULT_MASK[0] && read_u32(filters + 4) == FILTERS_DEFAULT_MASK[1] {
        text_item(G_SCREEN_MANAGER, 0x286, y1 as u32, 0x303, (y1 + 0x14) as u32, 0x30, c(JOB_COLOUR_317E), 0x2c, 1, c(JOB_COLOUR_3184), FILTERS_TEXT as u32, 0);
    } else {
        tel26_add_button([0x286, y1 as u32, 0x303, (y1 + 0x14) as u32, 0x30, c(JOB_COLOUR_317E), 0xc, 1, c(JOB_COLOUR_3184), FILTERS_TEXT as u32, 2, 0]);
    }
    let weights_buttons: [u8; 2] = [3, 1];
    let buttons = filters_area([0x75, (y2 - 0x1e) as u32, 0x303, y2 as u32, 2, weights_buttons.as_ptr() as u32, 1, 0, 1, 0, u32::MAX]);
    filters_text(0x98a1a0); // "Cancel"
    tel26_area_button([buttons, 0, 0, 0x30, c(JOB_COLOUR_317E), 0xc, 3, c(JOB_COLOUR_3184), FILTERS_TEXT as u32, 8, 0]);
    filters_text(0x990f7c); // "Ok"
    tel26_area_button([buttons, 1, 0, 0x30, c(JOB_COLOUR_317E), 0xc, 3, c(JOB_COLOUR_3184), FILTERS_TEXT as u32, 9, 0]);
    y1 += 0x23;
    y2 -= 0x32;
    let mut pitch = [0i32; 4];
    let draw_pitch: unsafe extern "C" fn(u32, u32, u32, u32, *mut i32, *mut i32, *mut i32, *mut i32, u32, u32, f32) = core::mem::transmute(FN_DRAW_PITCH);
    let p = pitch.as_mut_ptr();
    draw_pitch(0x7f, y1 as u32, 0x2f9, y2 as u32, p, p.add(1), p.add(2), p.add(3), 1, buttons, 4.0);
    y1 += 5;
    y2 -= 5;
    let w_age: [u8; 7] = [2, 1, 1, 1, 1, 1, 1];
    let w_nationality: [u8; 7] = [8, 4, 4, 4, 7, 1, 4];
    let w_based: [u8; 7] = [8, 4, 4, 4, 4, 7, 1];
    let w_job: [u8; 7] = [8, 4, 4, 7, 1, 4, 4];
    let w_attributes: [u8; 9] = [6, 1, 1, 1, 1, 6, 1, 1, 1];
    let w_match: [u8; 9] = [6, 1, 1, 1, 1, 5, 1, 2, 1];
    let w_text: [u8; 7] = [8, 4, 4, 1, 0xf, 0, 0];
    let body = |cols: u32, weights: *const u8, rows: u32| filters_area([0x84, y1 as u32, 0x2f4, y2 as u32, cols, weights as u32, rows, 0, 1, 0, u32::MAX]);
    let rows9 = if staff { 7 } else { 0x10 };
    let mut b = FiltersBuild {
        filters,
        staff,
        list_type,
        manager,
        nation,
        age: body(7, w_age.as_ptr(), rows),
        nationality: 0,
        based: 0,
        job: 0,
        attributes: 0,
        attributes_match: 0,
        text: 0,
    };
    b.nationality = body(7, w_nationality.as_ptr(), rows);
    b.based = body(7, w_based.as_ptr(), rows);
    b.job = body(7, w_job.as_ptr(), rows);
    b.attributes = body(9, w_attributes.as_ptr(), rows9);
    b.attributes_match = body(9, w_match.as_ptr(), rows9);
    b.text = body(7, w_text.as_ptr(), rows);
    if page == 2 {
        filters_screen_attributes_page(&b);
    } else {
        filters_screen_general_page(&b);
    }
}

/// The Attributes page of the Filters screen: every attribute (12 non-player ones on a staff search, 31 player ones
/// otherwise) sorted by name, two per row (columns 0 and 5 of the 9-column layout).
///
/// Each attribute: its name (" %s") as a button with id 3 (yellow when its minimum is above 1, else colour 0xae3154);
/// "-" (id 4, plain text while the minimum is below 2); the minimum ("%d", id 0x12, yellow above 1, else the static
/// green); "+" (id 5, plain text from 20). When at least two ranges are set, "Match" with "-" (id 6, plain text below 2
/// to match), "<to match>/<set>" and "+" (id 7, plain text when all of them must match).
unsafe fn filters_screen_attributes_page(b: &FiltersBuild) {
    covered!("tel21_5 filters attributes page");
    let c = job_colour;
    let (count, sort, name_of) = if b.staff {
        (12usize, FILTERS_FN_SORT_STAFF_ATTRIBUTES, FILTERS_FN_NON_PLAYER_ATTRIBUTE_NAME)
    } else {
        (31, FILTERS_FN_SORT_PLAYER_ATTRIBUTES, FILTERS_FN_PLAYER_ATTRIBUTE_NAME)
    };
    let table = b.staff as u32;
    let mut list = [0u8; 31];
    for (k, a) in list.iter_mut().enumerate().take(count) {
        *a = (k + 1) as u8;
    }
    exe::crt_qsort(list.as_mut_ptr() as usize, count as u32, 1, sort);
    let name: unsafe extern "C" fn(u32, *mut u8, u32, u32) -> u32 = core::mem::transmute(name_of);
    let minimum = |a: u8| search_filters_get_minimum(b.filters, a as u32, table) as i8;
    let area = b.attributes;
    let (mut col, mut row) = (0u32, 0u32);
    let mut buf = [0u8; 0xc8];
    for &a in list.iter().take(count) {
        let value = a as i8 as i32 as u32;
        name(a as u32, buf.as_mut_ptr(), 200, 0);
        filters_format(FILTERS_TEXT, 0x98ed14, buf.as_ptr() as usize); // " %s"
        let colour = if minimum(a) > 1 { c(JOB_COLOUR_BDEC) } else { c(FILTERS_COLOUR_3154) };
        tel26_area_button([area, col, row, 1, 0, 1, 2, colour, FILTERS_TEXT as u32, 3, value]);
        if minimum(a) < 2 {
            tel26_area_text([area, col + 1, row, 0x30, c(FILTERS_COLOUR_3150), 0xc, 1, c(JOB_COLOUR_3184), 0x98ba0c]);
        } else {
            tel26_area_button([area, col + 1, row, 0x30, c(FILTERS_COLOUR_3150), 0xc, 1, c(JOB_COLOUR_3184), 0x98ba0c, 4, value]);
        }
        filters_format(FILTERS_TEXT, 0x98806c, minimum(a) as i32 as usize); // "%d"
        let colour = if minimum(a) > 1 { c(JOB_COLOUR_BDEC) } else { c(FILTERS_STATIC_GREEN) };
        tel26_area_button([area, col + 2, row, 1, 0, 0xc, 2, colour, FILTERS_TEXT as u32, 0x12, value]);
        if minimum(a) < 0x14 {
            tel26_area_button([area, col + 3, row, 0x30, c(FILTERS_COLOUR_3150), 0xc, 1, c(JOB_COLOUR_3184), 0x9afe7c, 5, value]);
        } else {
            tel26_area_text([area, col + 3, row, 0x30, c(FILTERS_COLOUR_3150), 0xc, 1, c(JOB_COLOUR_3184), 0x9afe7c]);
        }
        if col == 0 {
            col = 5;
        } else {
            col = 0;
            row += 1;
        }
    }
    let (set, to_match) = if b.staff {
        (read_i8(b.filters + search_filters_block::STAFF_RANGES_SET), read_i8(b.filters + search_filters_block::STAFF_RANGES_TO_MATCH))
    } else {
        (read_i8(b.filters + search_filters_block::PLAYER_RANGES_SET), read_i8(b.filters + search_filters_block::PLAYER_RANGES_TO_MATCH))
    };
    if set < 2 {
        return;
    }
    filters_text1(0xa800ec, FILTERS_NO_COMMENT); // " Match<..>"
    let area = b.attributes_match;
    tel26_area_text([area, 5, row, 1, 0, 1, 2, c(FILTERS_COLOUR_3154), FILTERS_TEXT as u32]);
    if to_match < 2 {
        tel26_area_text([area, 6, row, 0x30, c(FILTERS_COLOUR_3150), 0xc, 1, c(JOB_COLOUR_3184), 0x98ba0c]);
    } else {
        tel26_area_button([area, 6, row, 0x30, c(FILTERS_COLOUR_3150), 0xc, 1, c(JOB_COLOUR_3184), 0x98ba0c, 6, 0]);
    }
    let f: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    f(FILTERS_TEXT, 0xa800cc, to_match as i32, set as i32); // "<%d - number1>/<%d - number2>"
    tel26_area_text([area, 7, row, 1, 0, 0xc, 2, c(JOB_COLOUR_BDE8), FILTERS_TEXT as u32]);
    if set <= to_match {
        tel26_area_text([area, 8, row, 0x30, c(FILTERS_COLOUR_3150), 0xc, 1, c(JOB_COLOUR_3184), 0x9afe7c]);
    } else {
        tel26_area_button([area, 8, row, 0x30, c(FILTERS_COLOUR_3150), 0xc, 1, c(JOB_COLOUR_3184), 0x9afe7c, 7, 0]);
    }
}

/// The General page of the Filters screen, row by row (headings in column 0, the options from column 1; a chosen
/// option is the yellow item, the others green buttons with id 10 and their flag's index; see the options' order in
/// the header of each piece):
/// - Text (not on a scout's match list): Any / Specified (high 0x200000 / 0x400000) and, when specified, the text field
///   (0x19 characters of the filters' +0x7f);
/// - Nationality (`filters_screen_nationality_row`), Age (`filters_screen_age_row`), Language
///   (`filters_screen_language_row`), Based (`filters_screen_based_row`);
/// - a staff search: Job (`filters_screen_job_row`); the others: Value (`filters_screen_value_row`), Status (Any, Listed,
///   For Loan: 0x10000000, 0x20000000, 0x40000000), Contract (Any is chosen when no contract flag is set; Expired,
///   Expiring: high 1, 2; Unprotected 0x80000000 only while the FIFA rules are in force), Position (Any, GK, Def, Mid,
///   Att: high 4..0x40) and Side (Any, Left, Centre, Right: high 0x80..0x400);
/// - a scout's match list: Ability (Any, Current, Future: high 0x800000..0x2000000);
/// - not a staff search: Injuries (Any, None: high 0x2000, 0x4000);
/// - Interested (Any, Yes: high 0x8000, 0x10000);
/// - not a staff search: Scouted Only (No, Yes: high 0x40000000, 0x80000000) and, when the manager has a club, Include
///   Squad (No, Yes: high 0x4000000, 0x8000000) with Senior / Reserve when the club has a reserve squad, else A / B when
///   it has a linked B team (high 0x10000000, 0x20000000; the last is the chosen item or, else, the button).
unsafe fn filters_screen_general_page(b: &FiltersBuild) {
    covered!("tel21_5 filters general page");
    let f = b.filters;
    let hi = read_u32(f + search_filters_block::MASK_HIGH);
    let mut row = 0u32;
    if b.list_type != FILTERS_LIST_SCOUT {
        let area = b.text;
        filters_text(0xa800c4); // " Text"
        filters_heading(area, 0);
        filters_text1(0xa8009c, FILTERS_NO_COMMENT); // "Any<..>"
        filters_option(area, 1, 0, hi & 0x200000 != 0, 0, 0x200000);
        filters_text(0xa80090); // "Specified"
        filters_option(area, 2, 0, hi & 0x400000 != 0, 0, 0x400000);
        if read_u32(f + search_filters_block::MASK_HIGH) & 0x400000 != 0 {
            let g = job_colour(FILTERS_STATIC_GREEN);
            filters_item([0x80018, 0, 0, 0, 0, 4, 0, 1, 0, 0, 1, 2, g, (f + search_filters_block::TEXT) as u32, 0x19, 0, 0, area]);
        }
        row = 1;
    }
    filters_screen_nationality_row(b, row);
    filters_screen_age_row(b, row + 1);
    filters_screen_language_row(b, row + 2);
    filters_screen_based_row(b, row + 3);
    let area = b.age;
    let option = |col: u32, row: u32, fmt: usize, comment: bool, lo_bit: u32, hi_bit: u32| {
        if comment { filters_text1(fmt, FILTERS_NO_COMMENT) } else { filters_text(fmt) }
        let set = (read_u32(f) & lo_bit) != 0 || (read_u32(f + 4) & hi_bit) != 0;
        filters_option(area, col, row, set, lo_bit, hi_bit);
    };
    if b.staff {
        row += 4;
        filters_screen_job_row(b, row);
    } else {
        filters_screen_value_row(b, row + 4);
        let r = row + 5;
        filters_text(0xa7fef0); // " Status"
        filters_heading(area, r);
        option(1, r, 0xa7fed4, true, 0x10000000, 0);
        option(2, r, 0xa7fecc, false, 0x20000000, 0); // "Listed"
        option(3, r, 0xa7fec0, false, 0x40000000, 0); // "For Loan"
        let r = row + 6;
        filters_text(0x98eec4); // " Contract"
        filters_heading(area, r);
        filters_text1(0xa7fea0, FILTERS_NO_COMMENT); // "Any<..contract>"
        // Any is the chosen item when no contract flag is set (the inverse of the other options)
        let index = filters_bit_index(0x80000000, 3);
        if read_u32(f) & 0x80000000 == 0 && read_u32(f + 4) & 3 == 0 {
            filters_chosen(area, 1, r, 10, index);
        } else {
            filters_option_button(area, 1, r, 10, index);
        }
        option(2, r, 0xa7fe98, false, 0, 1); // "Expired"
        option(3, r, 0xa7fe8c, false, 0, 2); // "Expiring"
        if read_u8(G_FIFA_TRANSFER_RULES_IN_FORCE) == 1 {
            option(4, r, 0xa7fe58, true, 0x80000000, 0); // "Unprotected<..>"
        }
        let r = row + 7;
        filters_text(0xa7fe4c); // " Position"
        filters_heading(area, r);
        option(1, r, 0xa7fe2c, true, 0, 4);
        option(2, r, 0xa7fe28, false, 0, 8); // "GK"
        option(3, r, 0xa7fe24, false, 0, 0x10); // "Def"
        option(4, r, 0xa7fe20, false, 0, 0x20); // "Mid"
        option(5, r, 0xa7fe1c, false, 0, 0x40); // "Att"
        row += 8;
        filters_text(0xa7fe14); // " Side"
        filters_heading(area, row);
        option(1, row, 0xa7fdf8, true, 0, 0x80);
        option(2, row, 0x9b7444, false, 0, 0x100); // "Left"
        option(3, row, 0x9b7420, false, 0, 0x200); // "Centre"
        option(4, row, 0x9b7464, false, 0, 0x400); // "Right"
    }
    if b.list_type == FILTERS_LIST_SCOUT {
        row += 1;
        filters_text(0xa7ff48); // " Ability"
        filters_heading(area, row);
        option(1, row, 0xa7ff2c, true, 0, 0x800000);
        option(2, row, 0x99ccdc, false, 0, 0x1000000); // "Current"
        option(3, row, 0xa7fdf0, false, 0, 0x2000000); // "Future"
    }
    if !b.staff {
        row += 1;
        filters_text(0xa7fde4); // " Injuries"
        filters_heading(area, row);
        option(1, row, 0xa7fdc4, true, 0, 0x2000);
        option(2, row, 0xa7fda4, true, 0, 0x4000); // "None<..>"
    }
    let mut r = row + 1;
    filters_text1(0xa7fd70, FILTERS_NO_COMMENT); // " Interested<..>"
    filters_heading(area, r);
    option(1, r, 0xa7fd50, true, 0, 0x8000);
    option(2, r, 0x98e8d0, false, 0, 0x10000); // "Yes"
    if !b.staff {
        r = row + 2;
        filters_text(0xa7fd40); // " Scouted Only"
        filters_heading(area, r);
        option(1, r, 0xa7fd1c, true, 0, 0x40000000); // "No<..>"
        option(2, r, 0xa7fcf8, true, 0, 0x80000000); // "Yes<..>"
    }
    if b.staff {
        return;
    }
    let club = b.manager.club_job();
    if club.is_null() {
        return;
    }
    r += 1;
    filters_text(0xa7fce8); // " Include Squad"
    filters_heading(area, r);
    option(1, r, 0x98e8cc, false, 0, 0x4000000); // "No"
    option(2, r, 0x98e8d0, false, 0, 0x8000000); // "Yes"
    if exe::database_club_has_reserve_squad(club) == 0 {
        if exe::club_get_linked_b_team(club, core::ptr::null_mut(), 1) == 0 {
            return;
        }
        option(3, r, 0xa7fcbc, true, 0, 0x10000000); // "A<..>"
        option(4, r, 0xa7fc98, true, 0, 0x20000000); // "B<..>"
    } else {
        option(3, r, 0xa7fce0, false, 0, 0x10000000); // "Senior"
        option(4, r, 0x9b5b40, false, 0, 0x20000000); // "Reserve"
    }
}

/// The Nationality row: Any (low 1), the manager's nation's three-letter code (low 2) and Other: plain yellow text when
/// a nation, a region, EU National or Basque is chosen (low 4..0x20), else a button for low 0x40. With one of those
/// chosen, a menu button at column 5 opens: EU National (the key nation is an EU member) and Basque (the manager's club
/// is Basque) with a separator, the key nations whose level byte is above 8 (by nationality adjective, id 0xb), a
/// separator and "Region" opening the regions sorted by name (id 0xc). The chosen nation (link 0x7d5 to its national
/// side), region, "  EU National" or "  Basque" is shown at column 4.
unsafe fn filters_screen_nationality_row(b: &FiltersBuild, row: u32) {
    covered!("tel21_5 filters nationality row");
    let c = job_colour;
    let f = b.filters;
    let area = b.nationality;
    filters_text(0x98edb8); // " Nationality"
    filters_heading(area, row);
    filters_text1(0xa80070, FILTERS_NO_COMMENT); // "Any<..nationality>"
    filters_option(area, 1, row, read_u32(f) & 1 != 0, 1, 0);
    filters_copy_text(b.nation + nation::THREE_LETTER_NAME);
    filters_option(area, 2, row, read_u32(f) & 2 != 0, 2, 0);
    filters_text(0xa80068); // "Other"
    if read_u32(f) & 0x3c == 0 {
        filters_option_button(area, 3, row, 10, filters_bit_index(0x40, 0));
    } else {
        tel26_area_text([area, 3, row, 0x221, c(JOB_COLOUR_BDEC), 0xc, 2, c(JOB_COLOUR_BDEC), FILTERS_TEXT as u32]);
    }
    if read_u32(f) & 0x3c == 0 {
        return;
    }
    let menu = filters_menu_button(area, 5, row);
    let eu = read_u8(b.nation + FILTERS_NATION_EU) == 2;
    if eu {
        filters_text(0xa8005c); // "EU National"
        filters_menu_entry(FILTERS_TEXT, 10, filters_bit_index(4, 0), menu);
    }
    let club = b.manager.club_job();
    if !club.is_null() && exe::database_is_basque_club(club) != 0 {
        filters_text(0xa80054); // "Basque"
        filters_menu_entry(FILTERS_TEXT, 10, filters_bit_index(8, 0), menu);
        filters_menu_separator(menu);
    } else if eu {
        filters_menu_separator(menu);
    }
    let order: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FILTERS_FN_KEY_NATION_SORTED_ORDER);
    let list = order(0);
    let mut buf = [0u8; 0xc8];
    for k in 0..FILTERS_KEY_NATION_COUNT {
        let n = read_usize((G_LEAGUE_TABLE as isize + read_i8(list + k) as isize * FILTERS_KEY_NATION_SIZE as isize) as usize);
        if n == 0 || read_i8(n + FILTERS_NATION_LEVEL) <= 8 {
            continue;
        }
        if read_u32(f) & 0x10 != 0 && read_u32(f + search_filters_block::NATIONALITY_NATION) == read_u32(n) {
            filters_format(FILTERS_TEXT, 0xa78880, 0); // "  "
            exe::nation_get_nationality_adjective(n, FILTERS_TEXT + 2, 0x7d0, 0xc);
            let team = exe::database_get_national_team(n);
            tel26_area_button([area, 4, row, 1, 0, 1, 2, c(JOB_COLOUR_BDEC), FILTERS_TEXT as u32, 0x7d5, team as u32]);
        }
        exe::nation_get_nationality_adjective(n, buf.as_mut_ptr() as usize, 0xc8, 0xc);
        filters_menu_entry(buf.as_ptr() as usize, 0xb, read_u32(n), menu);
    }
    filters_menu_separator(menu);
    filters_text(0xa7e80c); // "Region"
    let entry = filters_menu_entry(FILTERS_TEXT, 0, 0, menu);
    let regions_menu = filters_menu(0x30, entry);
    let sorted: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FILTERS_FN_REGIONS_SORTED_BY_NAME);
    let regions = sorted(0xc);
    for k in 0..0xe {
        let region = read_i8(regions + k);
        exe::get_region_name(region as i32 as u32, buf.as_mut_ptr() as usize, 200, 0xc);
        if read_u32(f) & 0x20 != 0 && read_i8(f + search_filters_block::NATIONALITY_REGION) == region {
            filters_format(FILTERS_TEXT, 0x987ff0, buf.as_ptr() as usize); // "  %s"
            filters_choice(area, 4, row);
        }
        filters_menu_entry(buf.as_ptr() as usize, 0xc, region as i32 as u32, regions_menu);
    }
    if read_u32(f) & 4 != 0 {
        filters_text(0xa80044); // "  EU National"
        filters_choice(area, 4, row);
    } else if read_u32(f) & 8 != 0 {
        filters_text(0xa80038); // "  Basque"
        filters_choice(area, 4, row);
    }
}

/// The Age row: Any (low 0x80) and the bands: a staff search 0-30, 30-40, 40-50, 50-60, 60+ (low 0x2000..0x20000), the
/// others 0-21, 19-25, 20-30, 25-32, 30+ (low 0x100..0x1000).
unsafe fn filters_screen_age_row(b: &FiltersBuild, row: u32) {
    let f = b.filters;
    let area = b.age;
    filters_text(0x98eda8); // " Age"
    filters_heading(area, row);
    filters_text1(0xa80020, FILTERS_NO_COMMENT); // "Any<..age>"
    filters_option(area, 1, row, read_u32(f) & 0x80 != 0, 0x80, 0);
    let bands: [(usize, u32); 5] = if b.staff {
        [(0xa80018, 0x2000), (0xa80010, 0x4000), (0xa80008, 0x8000), (0xa80000, 0x10000), (0xa7fffc, 0x20000)]
    } else {
        [(0xa7fff4, 0x100), (0xa7ffec, 0x200), (0xa7ffe4, 0x400), (0xa7ffdc, 0x800), (0xa7ffd8, 0x1000)]
    };
    for (k, &(fmt, bit)) in bands.iter().enumerate() {
        filters_text(fmt);
        filters_option(area, 2 + k as u32, row, read_u32(f) & bit != 0, bit, 0);
    }
}

/// The Language row: Any (low 0x40000; chosen it is the yellow item without id) and the key nation's languages (the
/// first always, the second and third when between 1 and 0x40), each chosen when low 0x80000 is set and the filters'
/// language is its number (1..3), else a button with id 0xd and the number.
unsafe fn filters_screen_language_row(b: &FiltersBuild, row: u32) {
    let f = b.filters;
    let area = b.age;
    filters_text(0xa7ffcc); // " Language"
    filters_heading(area, row);
    filters_text1(0xa7ffac, FILTERS_NO_COMMENT); // "Any<..language>"
    if read_u32(f) & 0x40000 == 0 {
        filters_option_button(area, 1, row, 10, filters_bit_index(0x40000, 0));
    } else {
        filters_chosen(area, 1, row, 0, 0);
    }
    let name: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FILTERS_FN_LANGUAGE_NAME);
    for (k, &field) in FILTERS_NATION_LANGUAGES.iter().enumerate() {
        let language = read_i8(b.nation + field);
        if k > 0 && !(language > 0 && language < 0x41) {
            continue;
        }
        filters_copy_text(name(language as u8 as u32));
        let n = k as u32 + 1;
        if read_u32(f) & 0x80000 == 0 || read_i8(f + search_filters_block::LANGUAGE) as i32 != n as i32 {
            filters_option_button(area, 2 + k as u32, row, 0xd, n);
        } else {
            filters_chosen(area, 2 + k as u32, row, 0, 0);
        }
    }
}

/// The Based row: Any (low 0x100000), the manager's nation's three-letter code (0x200000), Abroad (0x400000) and Other
/// (plain yellow text when a based nation or region is chosen, low 0x800000 / 0x1000000, else a button for 0x2000000).
/// With one of those chosen, a menu button at column 6 opens the key nations above level 8 (by short name, id 0xe), a
/// separator and "Region" with the regions (id 0xf); the chosen nation (link 0x7d5) or region is shown at column 5.
unsafe fn filters_screen_based_row(b: &FiltersBuild, row: u32) {
    covered!("tel21_5 filters based row");
    let c = job_colour;
    let f = b.filters;
    let area = b.based;
    filters_text(0xa7ffa4); // " Based"
    filters_heading(area, row);
    filters_text1(0xa7ff88, FILTERS_NO_COMMENT); // "Any<..based>"
    filters_option(area, 1, row, read_u32(f) & 0x100000 != 0, 0x100000, 0);
    filters_copy_text(b.nation + nation::THREE_LETTER_NAME);
    filters_option(area, 2, row, read_u32(f) & 0x200000 != 0, 0x200000, 0);
    filters_text(0xa7ff80); // "Abroad"
    filters_option(area, 3, row, read_u32(f) & 0x400000 != 0, 0x400000, 0);
    filters_text(0xa80068); // "Other"
    if read_u32(f) & 0x800000 == 0 && read_u32(f) & 0x1000000 == 0 {
        filters_option_button(area, 4, row, 10, filters_bit_index(0x2000000, 0));
    } else {
        tel26_area_text([area, 4, row, 0x221, c(JOB_COLOUR_BDEC), 0xc, 2, c(JOB_COLOUR_BDEC), FILTERS_TEXT as u32]);
    }
    if read_u32(f) & 0x800000 == 0 && read_u32(f) & 0x1000000 == 0 {
        return;
    }
    let menu = filters_menu_button(area, 6, row);
    let order: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FILTERS_FN_KEY_NATION_SORTED_ORDER);
    let list = order(0);
    for k in 0..FILTERS_KEY_NATION_COUNT {
        let n = read_usize((G_LEAGUE_TABLE as isize + read_i8(list + k) as isize * FILTERS_KEY_NATION_SIZE as isize) as usize);
        if n == 0 || read_i8(n + FILTERS_NATION_LEVEL) <= 8 {
            continue;
        }
        if read_u32(f) & 0x800000 != 0 && read_u32(f + search_filters_block::BASED_NATION) == read_u32(n) {
            filters_format(FILTERS_TEXT, 0x987ff0, n + nation::SHORT_NAME); // "  %s"
            let team = exe::database_get_national_team(n);
            tel26_area_button([area, 5, row, 1, 0, 1, 2, c(JOB_COLOUR_BDEC), FILTERS_TEXT as u32, 0x7d5, team as u32]);
        }
        filters_menu_entry(n + nation::SHORT_NAME, 0xe, read_u32(n), menu);
    }
    filters_menu_separator(menu);
    filters_text(0xa7e80c); // "Region"
    let entry = filters_menu_entry(FILTERS_TEXT, 0, 0, menu);
    let regions_menu = filters_menu(0x30, entry);
    let sorted: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FILTERS_FN_REGIONS_SORTED_BY_NAME);
    let regions = sorted(u32::MAX);
    let mut buf = [0u8; 0xc8];
    for k in 0..0xe {
        let region = read_i8(regions + k);
        exe::get_region_name(region as i32 as u32, buf.as_mut_ptr() as usize, 200, u32::MAX);
        if read_u32(f) & 0x1000000 != 0 && read_i8(f + search_filters_block::BASED_REGION) == region {
            filters_format(FILTERS_TEXT, 0x987ff0, buf.as_ptr() as usize); // "  %s"
            filters_choice(area, 5, row);
        }
        filters_menu_entry(buf.as_ptr() as usize, 0xf, region as i32 as u32, regions_menu);
    }
}

/// The Job row of a staff search: Any (high 0x800), Specified (high 0x1000) and, when specified, a menu button at
/// column 4 with Manager, Assistant, Coach, Scout, Physio, Player and Unemployed (id 0x10, values 3..9); the chosen job
/// ("  %s") at column 3. DEFECT of the original kept apart: with a job value outside 3..9 the shown text is an
/// uninitialised stack buffer (here empty).
unsafe fn filters_screen_job_row(b: &FiltersBuild, row: u32) {
    covered!("tel21_5 filters job row");
    let f = b.filters;
    let area = b.job;
    filters_text(0xa7ff78); // " Job"
    filters_heading(area, row);
    filters_text1(0xa7ff60, FILTERS_NO_COMMENT); // "Any<..job>"
    filters_option(area, 1, row, read_u32(f + 4) & 0x800 != 0, 0, 0x800);
    filters_text(0xa80090); // "Specified"
    filters_option(area, 2, row, read_u32(f + 4) & 0x1000 != 0, 0, 0x1000);
    if read_u32(f + 4) & 0x1000 == 0 {
        return;
    }
    let menu = filters_menu_button(area, 4, row);
    let mut chosen = [0u8; 0xc8];
    let jobs: [(usize, u32); 7] = [(0x9b6440, 3), (0xa7ff54, 4), (0x9b6448, 5), (0x9b67d8, 6), (0x9b6710, 7), (0x9ca078, 8), (0x9b6b04, 9)];
    for &(fmt, value) in jobs.iter() {
        filters_text(fmt);
        filters_menu_entry(FILTERS_TEXT, 0x10, value, menu);
        if read_i8(f + search_filters_block::JOB) as i32 == value as i32 {
            tel26_copy_text(chosen.as_mut_ptr(), FILTERS_TEXT as *const u8);
        }
    }
    filters_format(FILTERS_TEXT, 0x987ff0, chosen.as_ptr() as usize); // "  %s"
    filters_choice(area, 3, row);
}

/// The Value row of a player search: Any (low 0x4000000), Range (low 0x8000000) and, with a range, a menu button at
/// column 4 with the value bands from 7 down to -1 (id 0x11): -1 "Free", below 7 "0 - <amount of preset band>", 7
/// "<amount of preset band 6> +" (`Cash::set_from_preset_index`, `cash_get_display_amount(5)`, `cash_format_money`);
/// the chosen band ("  %s") at column 3.
unsafe fn filters_screen_value_row(b: &FiltersBuild, row: u32) {
    covered!("tel21_5 filters value row");
    let f = b.filters;
    let area = b.job;
    filters_text(0x98ed54); // " Value"
    filters_heading(area, row);
    filters_text1(0xa7ff10, FILTERS_NO_COMMENT); // "Any<..value>"
    filters_option(area, 1, row, read_u32(f) & 0x4000000 != 0, 0x4000000, 0);
    filters_text(0xa7ff08); // "Range"
    filters_option(area, 2, row, read_u32(f) & 0x8000000 != 0, 0x8000000, 0);
    if read_u32(f) & 0x8000000 == 0 {
        return;
    }
    let menu = filters_menu_button(area, 4, row);
    let preset: unsafe extern "thiscall" fn(*mut u32, u32) = core::mem::transmute(FILTERS_FN_CASH_SET_FROM_PRESET);
    let mut cash = [0u32; 2];
    let mut amount = [0u8; 0xc8];
    let mut band_text = [0u8; 0xc8];
    let mut band: i8 = 7;
    loop {
        if band == -1 {
            let t: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
            t(band_text.as_mut_ptr() as usize, 0x98e614); // "Free"
        } else {
            let (index, fmt) = if band < 7 { (band, 0xa7ff00) } else { (band - 1, 0xa7fef8) }; // "0 - %s" / "%s +"
            preset(cash.as_mut_ptr(), index as u8 as u32);
            let value = exe::cash_get_display_amount(cash.as_ptr(), 5);
            exe::cash_format_money(amount.as_mut_ptr() as usize, 0xc8, value, 5);
            filters_format(band_text.as_mut_ptr() as usize, fmt, amount.as_ptr() as usize);
        }
        if band == read_i8(f + search_filters_block::VALUE_BAND) {
            filters_format(FILTERS_TEXT, 0x987ff0, band_text.as_ptr() as usize); // "  %s"
            filters_choice(area, 3, row);
        }
        filters_menu_entry(band_text.as_ptr() as usize, 0x11, band as i32 as u32, menu);
        band -= 1;
        if band < -1 {
            break;
        }
    }
}

/// Comparer of `search_screens_build_filters_screen`: side by side over the display list (with the screen params) and
/// the function-static green and its guard.
pub(crate) unsafe extern "C" fn compare_search_screens_build_filters_screen() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I0%%);
    let extra = [(FILTERS_STATIC_GUARD, 1), (FILTERS_STATIC_GREEN, 2)];
    compare_by_display_list_with("Search filters screen", &extra, || o(), || search_screens_build_filters_screen());
}
