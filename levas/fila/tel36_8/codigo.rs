// ---------- batch %%LEVA%%: the competition's official stats tab (officials_manager.cpp) ----------

/// One official stats record of the list (`OfficialsManager::copy_comp_official_stats`, 0x11 bytes): the official's id
/// (+0), the average rating (f32, +4), the yellow cards (i16, +8), the games (i16, +0xc), the harshness (i8, +0xe) and
/// the red cards (u8, +0xf).
const OFFICIAL_STATS_SIZE: usize = 0x11;
const OFFICIAL_STATS_RATING: usize = 0x4;
const OFFICIAL_STATS_YELLOW: usize = 0x8;
const OFFICIAL_STATS_GAMES: usize = 0xc;
const OFFICIAL_STATS_HARSHNESS: usize = 0xe;
const OFFICIAL_STATS_RED: usize = 0xf;
/// The officials.dat record size.
const OFFICIAL_SIZE: u32 = 0x2b;

/// Rule: a competition's official stats tab lists its referees (sorted by games), with their games, red and yellow
/// cards, harshness and average rating, or says there are none.
///
/// `show_comp_official_stats_screen(&comp_id, top, bottom)`: the list of parameter 0x2e and its count (0x2f, a short).
/// `background_set_random_picture`, the menu bar (4, 0) and the heading "Official Stats" as a text item (0x6e, top)-
/// (0x30c, top + 0x23) (font 3, `JOB_COLOUR_BDEC`); the content starts at top + 0x28. With no list: the competition
/// (`competition` record `comp_id`), its count (`OfficialsManager::get_comp_official_count` on `G_OFFICIALS_MANAGER`)
/// and, with any, a `malloc`ed list of count × 0x11 bytes filled by `copy_comp_official_stats` and `qsort`ed by
/// `official_stats_compare_games_desc`; parameters 0x2e and 0x2f get the list (or 0) and the count. With a list and a
/// count: rows = (bottom − start − 0x10) / the height of font 3; area (0x6e, start)-(0x30c, bottom), seven columns (3,
/// 10, 4, 4, 4, 5, 3), kind 2; the headers "Name", "Games", "Red", "Yellow", "Harshness", "Rating" as buttons ids
/// 0x1389..0x138e (Harshness 0x138e, Rating 0x138d) on row 0; then one row per record from row 1: the ordinal of the
/// row, the name ("  <second>" without a first name, "  <i>.<second>" for a one-letter first name, else "  <first>
/// <second>"), the games, the reds, the yellows, the harshness (1 Harsh, 2 Strict, 4 Lenient, otherwise Fair) and the
/// rating ("  %1.2f", or "  ----" when it is 0.0); the area scrolls back to row parameter 0x2c and parameter 0x30 keeps
/// the area. Otherwise the text "There are no official stats available." as a text area. Then the Back / Next
/// buttons.
/// Original: `show_comp_official_stats_screen` (0x79a530, officials_manager.cpp, cdecl).
pub(crate) unsafe extern "C" fn show_comp_official_stats_screen(comp_id: usize, top: i32, bottom: i32) {
    let text = tel36_langlib();
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let columns: [u8; 7] = [3, 0xa, 4, 4, 4, 5, 3];
    let id = read_u32(comp_id);
    let _scroll = get_screen_param(0x2c);
    let mut list = get_screen_param(0x2e);
    let mut count = get_screen_param(0x2f) as i16;
    exe::background_set_random_picture();
    exe::menubar_build(4, 0);
    text(JOB_TEXT, 0xa6a21c);
    type AddTextItem = unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize, u32) -> u32;
    let add_text_item: AddTextItem = core::mem::transmute(FN_SCREENMANAGER_ADD_TEXT_ITEM);
    add_text_item(G_SCREEN_MANAGER, 0x6e, top as u32, 0x30c, (top + 0x23) as u32, 2, 0, 0xc, 3, job_colour(JOB_COLOUR_BDEC), JOB_TEXT, 1);
    let start = top + 0x28;
    if list == 0 {
        covered!("tel36_8 official stats list built");
        let comp = read_usize(G_CLUB_COMP_TABLE).wrapping_add(id.wrapping_mul(TEL36_CLUB_COMP_SIZE) as usize);
        let count_of: unsafe extern "thiscall" fn(usize, usize) -> i16 = core::mem::transmute(0x79d300usize);
        count = count_of(read_usize(G_OFFICIALS_MANAGER), comp);
        if count != 0 {
            let n = count as i32;
            list = exe::crt_malloc((n * OFFICIAL_STATS_SIZE as i32) as u32);
            let copy: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x79d1f0usize);
            copy(read_usize(G_OFFICIALS_MANAGER), comp, list);
            exe::crt_qsort(list, n as u32, OFFICIAL_STATS_SIZE as u32, 0x79b150usize);
        } else {
            list = 0;
        }
        set_screen_param(0x2e, list);
        set_screen_param(0x2f, count as isize as usize);
    }
    if list != 0 && count != 0 {
        covered!("tel36_8 official stats table");
        let rows = ((bottom - start - 0x10) / exe::font_get_height(3)) as u8 as u32;
        let area = tel26_add_area(current_human_display(), [0x6e, start as u32, 0x30c, bottom as u32, 7, columns.as_ptr() as u32, rows, 0, 2, 0, u32::MAX]);
        let pale = job_colour(JOB_COLOUR_3184);
        let grey = job_colour(JOB_COLOUR_317E);
        for (k, (title, id)) in [(0xa6a214usize, 0x1389u32), (0xa6a20c, 0x138a), (0xa6a204, 0x138b), (0xa6a1f8, 0x138c), (0xa6a1ec, 0x138e), (0xa6a1e0, 0x138d)]
            .into_iter().enumerate() {
            text(JOB_TEXT, title);
            tel26_area_button([area, k as u32 + 1, 0, 0x30, grey, 1, 1, pale, JOB_TEXT as u32, id, 0]);
        }
        let zero = read_f64(rdata::F64_0);
        for k in 0..count as i32 {
            let row = (k + 1) as u32;
            let record = list + k as usize * OFFICIAL_STATS_SIZE;
            let official = read_usize(G_OFFICIAL_TABLE).wrapping_add(read_u32(record).wrapping_mul(OFFICIAL_SIZE) as usize);
            let ordinal = exe::utils_format_ordinal(row as i32, -1);
            tel26_area_text([area, 0, row, 0x30, job_colour(JOB_COLOUR_BE04), 0xc, 1, pale, ordinal as u32]);
            let first = read_usize(official + official::FIRST_NAME);
            let second = read_usize(official + official::SECOND_NAME);
            if read_u8(first) == 0 {
                format(JOB_TEXT, 0x987ff0, second);
            } else if read_u8(first + 1) == 0 {
                format(JOB_TEXT, 0xa6a1d8, read_u8(first) as i8 as i32, second);
            } else {
                format(JOB_TEXT, 0xa6a1d0, first, second);
            }
            tel26_area_text([area, 1, row, 1, 0, 1, 2, pale, JOB_TEXT as u32]);
            for (column, value) in [(2u32, read_i16(record + OFFICIAL_STATS_GAMES) as i32), (3, read_u8(record + OFFICIAL_STATS_RED) as i32),
                (4, read_i16(record + OFFICIAL_STATS_YELLOW) as i32)] {
                format(JOB_TEXT, 0x990b48, value);
                tel26_area_text([area, column, row, 1, 0, 1, 2, pale, JOB_TEXT as u32]);
            }
            let harshness = match read_u8(record + OFFICIAL_STATS_HARSHNESS) as i8 {
                1 => 0xa6a1a8,
                2 => 0xa6a1b0,
                4 => 0xa6a1bc,
                _ => 0xa6a1c8,
            };
            text(JOB_TEXT, harshness);
            tel26_area_text([area, 5, row, 1, 0, 1, 2, pale, JOB_TEXT as u32]);
            let rating = read_f32(record + OFFICIAL_STATS_RATING) as f64;
            if rating == zero || rating.is_nan() {
                format(JOB_TEXT, 0xa6a198);
            } else {
                format(JOB_TEXT, 0xa6a1a0, rating);
            }
            tel26_area_text([area, 6, row, 0x30, job_colour(JOB_COLOUR_3192), 0xc, 1, job_colour(JOB_COLOUR_BDEC), JOB_TEXT as u32]);
        }
        let scroll = get_screen_param(0x2c);
        if area as i16 != -1 {
            exe::area_set_scroll_row(display_area_at(current_human_display(), area as i16), scroll);
        }
        set_screen_param(0x30, area as i16 as isize as usize);
    } else {
        let rows = ((bottom - start - 0x10) / exe::font_get_height(3)) as u8 as u32;
        text(JOB_TEXT, 0xa6a170);
        tel26_text_area([0x6e, start as u32, 0x30c, bottom as u32, 2, 0, rows, 3, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 0]);
    }
    tel26_back_next();
}

/// Comparer for `show_comp_official_stats_screen`: the tab side by side over the display list when the list already
/// exists; building the list (`malloc`) only the original.
pub(crate) unsafe extern "C" fn compare_show_comp_official_stats_screen(comp_id: usize, top: i32, bottom: i32) {
    let o = original_fn::<unsafe extern "C" fn(usize, i32, i32)>(%%I0%%);
    if get_screen_param(0x2e) == 0 {
        // simplification: the allocation of the list does not run twice; proven in Rust mode
        return o(comp_id, top, bottom);
    }
    compare_by_display_list_with("official stats tab (show_comp_official_stats_screen)", &[], || o(comp_id, top, bottom), || show_comp_official_stats_screen(comp_id, top, bottom));
}
