// --- sx16_4: the Official Stats screen's sorting (officials_manager.cpp: the header click and the 12 row orders) ---

/// The harshness code of an official stats row (+0xe, a signed byte: 1 harsh, 2 strict, 4 lenient, otherwise fair).
const OFFICIAL_STATS_HARSHNESS: usize = 0xe;

/// The official of a stats row (its +0 is the official's id in `G_OFFICIAL_TABLE`).
unsafe fn official_of_row(row: usize) -> usize {
    official_record(read_i32(row))
}

/// The tie-break of every order: the two officials' second names (`official::SECOND_NAME`) by `utils_string_compare(a,
/// b, 1, 0)`, ascending.
unsafe fn official_stats_by_surname(a: usize, b: usize) -> i32 {
    exe::utils_string_compare(read_usize(official_of_row(a) + official::SECOND_NAME) as *const u8,
        read_usize(official_of_row(b) + official::SECOND_NAME) as *const u8, 1, 0)
}

/// Orders two official stats rows by the officials' second names, ascending.
/// Original: `official_stats_compare_surname_asc` (0x79b040, officials_manager.cpp, cdecl qsort callback).
pub(crate) unsafe extern "C" fn official_stats_compare_surname_asc(a: usize, b: usize) -> i32 {
    official_stats_by_surname(a, b)
}

/// Orders two official stats rows by the officials' second names, descending.
/// Original: `official_stats_compare_surname_desc` (0x79b090, officials_manager.cpp, cdecl qsort callback).
pub(crate) unsafe extern "C" fn official_stats_compare_surname_desc(a: usize, b: usize) -> i32 {
    official_stats_by_surname(b, a)
}

/// The order by one value: different values by `a − b` (ascending) or `b − a`, equal ones by the second name.
unsafe fn official_stats_by_value(a: usize, b: usize, value: impl Fn(usize) -> i32, ascending: bool) -> i32 {
    let (va, vb) = (value(a), value(b));
    if va == vb {
        official_stats_by_surname(a, b)
    } else if ascending {
        va - vb
    } else {
        vb - va
    }
}

/// Orders two rows by games (`official_stats::MATCHES`, signed word), ascending, then by second name.
/// Original: `official_stats_compare_games_asc` (0x79b0e0, officials_manager.cpp, cdecl).
pub(crate) unsafe extern "C" fn official_stats_compare_games_asc(a: usize, b: usize) -> i32 {
    official_stats_by_value(a, b, |r| read_i16(r + official_stats::MATCHES) as i32, true)
}

/// Orders two rows by games, descending, then by second name (the screen's first order).
/// Original: `official_stats_compare_games_desc` (0x79b150, officials_manager.cpp, cdecl).
pub(crate) unsafe extern "C" fn official_stats_compare_games_desc(a: usize, b: usize) -> i32 {
    official_stats_by_value(a, b, |r| read_i16(r + official_stats::MATCHES) as i32, false)
}

/// Orders two rows by red cards (`official_stats::CARDS`, unsigned byte), ascending, then by second name.
/// Original: `official_stats_compare_red_cards_asc` (0x79b1c0, officials_manager.cpp, cdecl).
pub(crate) unsafe extern "C" fn official_stats_compare_red_cards_asc(a: usize, b: usize) -> i32 {
    official_stats_by_value(a, b, |r| read_u8(r + official_stats::CARDS) as i32, true)
}

/// Orders two rows by red cards, descending, then by second name.
/// Original: `official_stats_compare_red_cards_desc` (0x79b230, officials_manager.cpp, cdecl).
pub(crate) unsafe extern "C" fn official_stats_compare_red_cards_desc(a: usize, b: usize) -> i32 {
    official_stats_by_value(a, b, |r| read_u8(r + official_stats::CARDS) as i32, false)
}

/// Orders two rows by yellow cards (`official_stats::FOULS`, signed word), ascending, then by second name.
/// Original: `official_stats_compare_yellow_cards_asc` (0x79b2a0, officials_manager.cpp, cdecl).
pub(crate) unsafe extern "C" fn official_stats_compare_yellow_cards_asc(a: usize, b: usize) -> i32 {
    official_stats_by_value(a, b, |r| read_i16(r + official_stats::FOULS) as i32, true)
}

/// Orders two rows by yellow cards, descending, then by second name.
/// Original: `official_stats_compare_yellow_cards_desc` (0x79b310, officials_manager.cpp, cdecl).
pub(crate) unsafe extern "C" fn official_stats_compare_yellow_cards_desc(a: usize, b: usize) -> i32 {
    official_stats_by_value(a, b, |r| read_i16(r + official_stats::FOULS) as i32, false)
}

/// The order by the average rating (`official_stats::AVERAGE_GRADE`, f32): equal by second name; the higher first
/// (`descending`) or the lower first; unordered (NaN) 0.
unsafe fn official_stats_by_rating(a: usize, b: usize, descending: bool) -> i32 {
    let (ra, rb) = (read_f32(a + official_stats::AVERAGE_GRADE), read_f32(b + official_stats::AVERAGE_GRADE));
    if ra == rb {
        official_stats_by_surname(a, b)
    } else if ra > rb {
        if descending { -1 } else { 1 }
    } else if ra < rb {
        if descending { 1 } else { -1 }
    } else {
        0
    }
}

/// Orders two rows by rating, the higher first, then by second name.
/// Original: `official_stats_compare_rating_desc` (0x79b380, officials_manager.cpp, cdecl).
pub(crate) unsafe extern "C" fn official_stats_compare_rating_desc(a: usize, b: usize) -> i32 {
    official_stats_by_rating(a, b, true)
}

/// Orders two rows by rating, the lower first, then by second name.
/// Original: `official_stats_compare_rating_asc` (0x79b410, officials_manager.cpp, cdecl).
pub(crate) unsafe extern "C" fn official_stats_compare_rating_asc(a: usize, b: usize) -> i32 {
    official_stats_by_rating(a, b, false)
}

/// Orders two rows by the harshness code (+0xe, signed byte), ascending, then by second name.
/// Original: `official_stats_compare_harshness_asc` (0x79b4a0, officials_manager.cpp, cdecl).
pub(crate) unsafe extern "C" fn official_stats_compare_harshness_asc(a: usize, b: usize) -> i32 {
    official_stats_by_value(a, b, |r| read_i8(r + OFFICIAL_STATS_HARSHNESS) as i32, true)
}

/// Orders two rows by the harshness code, descending, then by second name.
/// Original: `official_stats_compare_harshness_desc` (0x79b510, officials_manager.cpp, cdecl).
pub(crate) unsafe extern "C" fn official_stats_compare_harshness_desc(a: usize, b: usize) -> i32 {
    official_stats_by_value(a, b, |r| read_i8(r + OFFICIAL_STATS_HARSHNESS) as i32, false)
}

/// The column headers of the Official Stats table: (button id, column number kept in parameter 0x2d, first order,
/// order on a second click of the same column).
const OFFICIAL_STATS_COLUMNS: [(u16, u16, usize, usize); 6] = [
    (0x1389, 5, 0x79b040, 0x79b090), // Name: surname ascending, then descending
    (0x138a, 6, 0x79b150, 0x79b0e0), // Games: descending, then ascending
    (0x138b, 3, 0x79b230, 0x79b1c0), // Red
    (0x138c, 2, 0x79b310, 0x79b2a0), // Yellow
    (0x138d, 4, 0x79b380, 0x79b410), // Rating
    (0x138e, 1, 0x79b510, 0x79b4a0), // Harshness
];

/// Handles a click on the Official Stats screen: a column header sorts the table, a second click on the same column
/// reverses it; anything else goes to the menu bar.
///
/// The parameters: 0x2d the sorted column, 0x2e the rows (17 bytes each), 0x2f their count, 0x31 the reversed flag (a
/// byte). Without rows, the menu bar. With a widget, parameter 0x2c keeps the scroll row of the area in parameter 0x30
/// (`ScreenManager::get_area_scroll_row`). The id is the widget's (`item_id_or_default`, the default command id for 0
/// and −1). A header (`OFFICIAL_STATS_COLUMNS`): the same column not reversed sorts by its second order and sets the
/// flag; otherwise its first order and the flag cleared (`qsort`, 17 bytes); the flag and the column stored; −11 (redraw).
/// Otherwise `menubar_handle_command(widget)` and, when it answers 0, `menubar_handle_object_link(widget)`.
/// Original: `official_stats_screen_handle_click` (0x79aba0, officials_manager.cpp, cdecl).
pub(crate) unsafe extern "C" fn official_stats_screen_handle_click(widget: u32) -> i32 {
    let w = widget as i16;
    let column = get_screen_param(0x2d) as u16;
    let rows = get_screen_param(0x2e);
    let count = get_screen_param(0x2f) as i16;
    let reversed = get_screen_param(0x31) as u8;
    if rows != 0 {
        if w != -1 {
            let area = get_screen_param(0x30);
            set_screen_param(0x2c, exe::screenmanager_get_area_scroll_row(G_SCREEN_MANAGER, area));
        }
        let id = item_id_or_default(w);
        if let Some(&(_, number, first, second)) = OFFICIAL_STATS_COLUMNS.iter().find(|c| c.0 == id) {
            covered!("sx16_4 official stats sort");
            let again = column == number && reversed == 0;
            exe::crt_qsort(rows, count as i32 as u32, OFFICIAL_STATS_SIZE as u32, if again { second } else { first });
            set_screen_param(0x31, again as usize);
            set_screen_param(0x2d, number as usize);
            return -11;
        }
    }
    let r = exe::menubar_handle_command(widget);
    if r != 0 { r } else { exe::menubar_handle_object_link(widget) }
}

/// Comparer for `official_stats_screen_handle_click`: a header sort side by side over the screen parameters, the
/// screen manager block and the rows; the menu bar only through the original.
pub(crate) unsafe extern "C" fn compare_official_stats_screen_handle_click(widget: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I0%%);
    let rows = get_screen_param(0x2e);
    let sorts = rows != 0 && OFFICIAL_STATS_COLUMNS.iter().any(|c| c.0 == item_id_or_default(widget as i16));
    if !sorts {
        return o(widget);
    }
    let mut regions = screen_param_regions();
    regions.push((G_SCREEN_MANAGER, 0x3840));
    regions.push((rows, (get_screen_param(0x2f) as i16).max(0) as usize * OFFICIAL_STATS_SIZE));
    let (ro, rr, same, mo, mr) = crate::comparison::side_by_side(&regions, || o(widget), || official_stats_screen_handle_click(widget));
    SCOREBOARD.record(same && ro == rr, || format!("Official Stats screen click(widget {}): return {} x {}, differences {:?}", widget as i16, ro, rr, first_differences(&mo, &mr)));
    ro
}

/// The comparer of a row order: the same answer for the same two rows.
unsafe fn compare_official_stats_order(name: &str, k: usize, a: usize, b: usize, rust: unsafe extern "C" fn(usize, usize) -> i32) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(k)(a, b);
    let n = rust(a, b);
    SCOREBOARD.record(r == n, || format!("{}(official {} x {}): original={} rust={}", name, read_i32(a), read_i32(b), r, n));
    r
}

pub(crate) unsafe extern "C" fn compare_official_stats_compare_surname_asc(a: usize, b: usize) -> i32 {
    compare_official_stats_order("official_stats_compare_surname_asc", %%I1%%, a, b, official_stats_compare_surname_asc)
}
pub(crate) unsafe extern "C" fn compare_official_stats_compare_surname_desc(a: usize, b: usize) -> i32 {
    compare_official_stats_order("official_stats_compare_surname_desc", %%I2%%, a, b, official_stats_compare_surname_desc)
}
pub(crate) unsafe extern "C" fn compare_official_stats_compare_games_asc(a: usize, b: usize) -> i32 {
    compare_official_stats_order("official_stats_compare_games_asc", %%I3%%, a, b, official_stats_compare_games_asc)
}
pub(crate) unsafe extern "C" fn compare_official_stats_compare_games_desc(a: usize, b: usize) -> i32 {
    compare_official_stats_order("official_stats_compare_games_desc", %%I4%%, a, b, official_stats_compare_games_desc)
}
pub(crate) unsafe extern "C" fn compare_official_stats_compare_red_cards_asc(a: usize, b: usize) -> i32 {
    compare_official_stats_order("official_stats_compare_red_cards_asc", %%I5%%, a, b, official_stats_compare_red_cards_asc)
}
pub(crate) unsafe extern "C" fn compare_official_stats_compare_red_cards_desc(a: usize, b: usize) -> i32 {
    compare_official_stats_order("official_stats_compare_red_cards_desc", %%I6%%, a, b, official_stats_compare_red_cards_desc)
}
pub(crate) unsafe extern "C" fn compare_official_stats_compare_yellow_cards_asc(a: usize, b: usize) -> i32 {
    compare_official_stats_order("official_stats_compare_yellow_cards_asc", %%I7%%, a, b, official_stats_compare_yellow_cards_asc)
}
pub(crate) unsafe extern "C" fn compare_official_stats_compare_yellow_cards_desc(a: usize, b: usize) -> i32 {
    compare_official_stats_order("official_stats_compare_yellow_cards_desc", %%I8%%, a, b, official_stats_compare_yellow_cards_desc)
}
pub(crate) unsafe extern "C" fn compare_official_stats_compare_rating_desc(a: usize, b: usize) -> i32 {
    compare_official_stats_order("official_stats_compare_rating_desc", %%I9%%, a, b, official_stats_compare_rating_desc)
}
pub(crate) unsafe extern "C" fn compare_official_stats_compare_rating_asc(a: usize, b: usize) -> i32 {
    compare_official_stats_order("official_stats_compare_rating_asc", %%I10%%, a, b, official_stats_compare_rating_asc)
}
pub(crate) unsafe extern "C" fn compare_official_stats_compare_harshness_asc(a: usize, b: usize) -> i32 {
    compare_official_stats_order("official_stats_compare_harshness_asc", %%I11%%, a, b, official_stats_compare_harshness_asc)
}
pub(crate) unsafe extern "C" fn compare_official_stats_compare_harshness_desc(a: usize, b: usize) -> i32 {
    compare_official_stats_order("official_stats_compare_harshness_desc", %%I12%%, a, b, official_stats_compare_harshness_desc)
}
