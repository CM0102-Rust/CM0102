// ---------- batch %%LEVA%%: the league progress, FIFA world rankings and UEFA coefficients screens (comp_screens.cpp, part 2) ----------

/// A competition record of the club competition table by the id held in a key (`G_CLUB_COMP_TABLE` +
/// [key] × 0x6b).
unsafe fn comp_record_of_key(key: usize) -> usize {
    read_usize(G_CLUB_COMP_TABLE) + read_u32(key) as usize * 0x6b
}

/// A competition's two colours (`database_get_comp_colours`).
unsafe fn comp_colours(comp: usize) -> (u32, u32) {
    let (mut c1, mut c2) = (0u32, 0u32);
    exe::database_get_comp_colours(comp, &mut c1, &mut c2);
    (c1, c2)
}

/// The screen title in a competition's colours (`add_screen_title`, the second colour first).
unsafe fn comp_colours_title(colours: (u32, u32), text: usize) {
    exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, colours.1, colours.0, text);
}

/// Area `area` of the current human's Display gets its scroll row back from param `param` (when there is an area).
unsafe fn comp_restore_area_scroll(area: u32, param: u32) {
    let scroll = get_screen_param(param);
    if area as u16 != 0xffff {
        exe::area_set_scroll_row(display_area_at(current_human_display(), area as i16), scroll);
    }
}

/// Rule: the league's two clubs of a fixture: the league (or league stage) it is played in and the two clubs' table
/// rows.
///
/// Any missing argument shows the comp_screens.cpp error box (line 0x2342) and answers 0. The three outputs start at
/// 0. The competition is the object of the competition table at the fixture's id (none, or an id below 0: 0); a reserve
/// or minor competition (`comp_is_reserve_or_minor_comp` of the fixture's competition) answers 0. The league is the
/// fixture's stage (its stage byte ≥ 0, from the stages table) or the competition itself; it must be a league (type 2).
/// Its table rows (`League::get_table_row`) are searched for the home and the away club. Answers 1 when both were found.
/// Original: `comp_fixture_resolve_league_clubs` (0x49ecd0, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_fixture_resolve_league_clubs(fixture: usize, league_out: *mut usize, home_out: *mut usize, away_out: *mut usize) -> u32 {
    if fixture == 0 || league_out.is_null() || home_out.is_null() || away_out.is_null() {
        comp_screens_error_box(0x2342);
        return 0;
    }
    *league_out = 0;
    *home_out = 0;
    *away_out = 0;
    let id = read_i32(fixture + fixture::COMP_ID);
    if id <= -1 {
        return 0;
    }
    let object = read_usize(read_usize(G_COMP_OBJECT_TABLE) + id as usize * 4);
    if object == 0 || exe::comp_is_reserve_or_minor_comp(read_usize(G_RESERVE_COMP), read_usize(fixture + fixture::COMPETITION)) != 0 {
        return 0;
    }
    let stage = read_u8(fixture + fixture::STAGE_INDEX) as i8;
    *league_out = if stage > -1 { read_usize(read_usize(object + league::STAGES) + stage as usize * 4) } else { object };
    let league = *league_out;
    if league == 0 || read_u8(league + league::COMP_TYPE) != 2 {
        return 0;
    }
    let mut k: i8 = 0;
    while (k as i16) < read_i16(league + league::CLUB_COUNT) {
        let row = exe::league_get_table_row(league, k as i32);
        if read_usize(row + league_table_row::CLUB) == read_usize(fixture + fixture::HOME_CLUB) {
            *home_out = row;
        }
        if read_usize(row + league_table_row::CLUB) == read_usize(fixture + fixture::AWAY_CLUB) {
            *away_out = row;
        }
        k = k.wrapping_add(1);
    }
    if *home_out == 0 || *away_out == 0 {
        return 0;
    }
    covered!("tel23_2 fixture league clubs");
    1
}

/// Rule: opening the league progress screen of a fixture's league with its two clubs highlighted.
///
/// No fixture: the comp_screens.cpp box (line 0x1bb1). A pointer below 10000 in absolute value does nothing. The league
/// and rows come from `comp_fixture_resolve_league_clubs`; when it fails with neither row found, the box of line
/// 0x1bbd. The screen opens with `open_screen` (builder `comp_league_progress_screen_draw`, input
/// `comp_league_progress_screen_handle_event`, no flag, no leave, no exit); then param 0 the league, 1 the home row,
/// 2 the away row, 3 and 4 zero.
/// Original: `comp_open_league_progress_for_fixture` (0x49b830, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_open_league_progress_for_fixture(fixture: usize) {
    let (mut league, mut home, mut away) = (0usize, 0usize, 0usize);
    if fixture == 0 {
        comp_screens_error_box(0x1bb1);
        return;
    }
    if (fixture as i32).wrapping_abs() < 10000 {
        return;
    }
    if comp_fixture_resolve_league_clubs(fixture, &mut league, &mut home, &mut away) == 0 && home == 0 && away == 0 {
        comp_screens_error_box(0x1bbd);
        return;
    }
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x49b990usize, 0x49bef0usize, 0, 0usize, 0) == 0 {
        return;
    }
    covered!("tel23_2 open league progress");
    for (k, v) in league_progress_planned_params(league, home, away).into_iter().enumerate() {
        set_screen_param(k as u32, v);
    }
}

/// The five params of the league progress screen as `comp_open_league_progress_for_fixture` writes them.
fn league_progress_planned_params(league: usize, home: usize, away: usize) -> [usize; 5] {
    [league, home, away, 0, 0]
}

/// `Plot::Plot` (0x7c6fc0, thiscall, ret 0x14).
unsafe fn plot_new(p: usize, a: u32, clubs: i32, games: i32, b: u32, series: u32) -> usize {
    let f: unsafe extern "thiscall" fn(usize, u32, i32, i32, u32, u32) -> usize = core::mem::transmute(0x7c6fc0usize);
    f(p, a, clubs, games, b, series)
}

/// `Plot::add_series` (0x7c7100, thiscall, ret 0xc): `n` points in a colour with a name.
unsafe fn plot_add_series(plot: usize, n: i32, colour: u32, name: usize) {
    let f: unsafe extern "thiscall" fn(usize, i32, u32, usize) = core::mem::transmute(0x7c7100usize);
    f(plot, n, colour, name)
}

/// `Plot::add_point` (0x7c7240, thiscall, ret 0xc): point (x, y) of a series.
unsafe fn plot_add_point(plot: usize, series: u32, x: i32, y: i32) {
    let f: unsafe extern "thiscall" fn(usize, u32, i32, i32) = core::mem::transmute(0x7c7240usize);
    f(plot, series, x, y)
}

/// A table row's position history as a plot series: one point per game played (+0x11), x the game (1..), y the
/// position after it (the history byte + 1).
unsafe fn league_progress_series(plot: usize, row: usize, series: u32, colour: u32) {
    let n = read_u8(row + league_table_row::PLAYED) as i8 as i32;
    plot_add_series(plot, n, colour, read_usize(row + league_table_row::CLUB) + club::SHORT_NAME);
    for k in 0..n.max(0) {
        let position = read_u8(read_usize(row + league_table_row::POSITION_HISTORY) + k as usize) as i8 as i32;
        plot_add_point(plot, series, k + 1, position + 1);
    }
}

/// Rule: building the league progress screen: the league's position graph of the two clubs and its table.
///
/// The league is param 0 (none: the comp_screens.cpp box, line 0x1bea), the home row param 1 and the away row param 2.
/// The competition's picture and colours, the title (the competition's name), the subtitle "<stage name> Progress"
/// (`comp_util_get_stage_name` of the league's stage id) or "League Progress" (no stage id), the menu bar (4). An empty
/// area for the graph (x 0x6e..0x267, y 0x91..0x217). The table column goes back to 0 (+0xad) and the table is sorted
/// (`League::sort_table`). A Plot (operator new 0x1c: 1, the club count, the games per club, 1, the number of rows
/// given) gets the home row's history in yellow (series 0) and the away row's in red (series 1 when there is a home
/// row, else 0), drawn as a line graph (x 0x73..0x25d, y 0x96..0x212, "Played" and "Position"), then destroyed. The
/// table: a 2-column area (weights 3, 10; x 0x26c..0x30c, y 0x91..0x217) with as many rows as 0x17c pixels hold in font
/// 2; each row its position (`utils_format_ordinal`, dark blue, a button id 1 with the row) and "  <club>" (yellow
/// for the home row, red for the away row, else pale grey; a button id 1 with the row); empty pale grey cells to the
/// last row. Back/Next; the scroll row from param 3 and the area to param 4.
/// Kept from the original: the row count goes as its byte (the original's dword carries the freed Plot pointer's high
/// bytes; Display keeps the byte); the colour words go in dword slots whose high half is register garbage (0 here).
/// Original: `comp_league_progress_screen_draw` (0x49b990, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_league_progress_screen_draw() {
    let weights: [u8; 2] = [3, 0xa];
    let league = get_screen_param(0);
    if league == 0 {
        comp_screens_error_box(0x1bea);
        return;
    }
    let home = get_screen_param(1);
    let away = get_screen_param(2);
    let comp = read_usize(league + league::COMPETITION);
    exe::background_set_comp_picture(comp, 0, 0);
    comp_colours_title(comp_colours(comp), comp + 4);
    let stage_id = read_u16(league + league::STAGE_ID);
    let mut label = [0u8; 0x7d0];
    if stage_id != 0 {
        exe::comp_util_get_stage_name(label.as_mut_ptr(), 0x7d0, stage_id as u32, 0);
        comp_lang_format()(JOB_TEXT, 0x99c874, label.as_ptr()); // "<%s - table name> Progress"
    } else {
        comp_lang_format()(JOB_TEXT, 0x99c864); // "League Progress"
    }
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, JOB_TEXT, 1);
    exe::menubar_build(4, 0);
    tel26_add_area(current_human_display(), [0x6e, 0x91, 0x267, 0x217, 1, 0, 1, 0, 2, 0, u32::MAX]);
    write_u32(league + league::TABLE_COLUMN, 0);
    exe::league_sort_table(league, 0usize);
    let p = exe::crt_operator_new(0x1c);
    let plot = if p != 0 {
        let series = (away != 0) as u32 + (home != 0) as u32;
        plot_new(p, 1, read_i16(league + league::CLUB_COUNT) as i32, read_i16(league + league::GAMES_PER_CLUB) as i32, 1, series)
    } else {
        0
    };
    if home != 0 {
        league_progress_series(plot, home, 0, job_colour(G_GUI_COLOUR_YELLOW));
    }
    if away != 0 {
        league_progress_series(plot, away, (home != 0) as u32, job_colour(G_GUI_COLOUR_RED));
    }
    let mut position_label = [0u8; 0x7d0];
    comp_lang_format()(label.as_mut_ptr() as usize, 0x99c4a0, 0xad9c64usize); // "Played<%s - COMMENT - games played>"
    comp_lang_format()(position_label.as_mut_ptr() as usize, 0x99c838, 0xad9c64usize); // "Position<%s - COMMENT - league position>"
    let draw: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, usize, usize) = core::mem::transmute(0x7c7340usize);
    draw(plot, 0x73, 0x96, 0x25d, 0x212, label.as_ptr() as usize, position_label.as_ptr() as usize);
    if plot != 0 {
        let drop_plot: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x7c70a0usize);
        drop_plot(plot);
        exe::crt_operator_delete(plot);
    }
    let rows = (0x17c / exe::font_get_height(2)) as u8;
    let area = tel26_add_area(current_human_display(), [0x26c, 0x91, 0x30c, 0x217, 2, weights.as_ptr() as u32, rows as u32, 0, 1, 0, u32::MAX]);
    let mut r: i32 = 0;
    while r < read_i16(league + league::CLUB_COUNT) as i32 {
        let row = exe::league_get_table_row(league, r);
        let ordinal = exe::utils_format_ordinal(read_u8(row + league_table_row::POSITION) as i8 as i32 + 1, 3);
        tel26_area_button([area, 0, r as u32, 0x30, job_colour(G_GUI_COLOUR_DARK_BLUE), 0xc, 1, job_colour(G_GUI_COLOUR_PALE_GREY), ordinal as u32, 1, row as u32]);
        comp_crt_format()(JOB_TEXT, 0x987ff0, read_usize(row + league_table_row::CLUB) + club::SHORT_NAME); // "  %s"
        let colour = if row == home {
            job_colour(G_GUI_COLOUR_YELLOW)
        } else if row == away {
            job_colour(G_GUI_COLOUR_RED)
        } else {
            job_colour(G_GUI_COLOUR_PALE_GREY)
        };
        tel26_area_button([area, 1, r as u32, 2, 0, 1, 2, colour, JOB_TEXT as u32, 1, row as u32]); // tel23_2 the club cell's style
        r += 1;
    }
    while r < rows as i32 {
        tel26_area_text([area, 0, r as u32, 2, 0, 1, 2, job_colour(G_GUI_COLOUR_PALE_GREY), 0]);
        tel26_area_text([area, 1, r as u32, 2, 0, 1, 2, job_colour(G_GUI_COLOUR_PALE_GREY), 0]);
        r += 1;
    }
    tel26_back_next();
    comp_restore_area_scroll(area, 3);
    set_screen_param(4, area as i16 as isize as usize);
}

/// Rule: the league progress screen's clicks: with a widget the table keeps its scroll row (param 3 from the area in
/// 4). Id 1 (a table row, the row as value): the home row (param 1) or the away row (param 2) clicked again is cleared;
/// another row fills param 1 when it is empty, else param 2; −11. Other ids go to the menu bar.
/// Original: `comp_league_progress_screen_handle_event` (0x49bef0, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_league_progress_screen_handle_event(widget: u32) -> i32 {
    let w = widget as i16;
    if w != -1 {
        comp_click_keep_scroll(3, 4);
    }
    if tel26_item_id(w) != 1 {
        return comp_click_menubar(widget);
    }
    let row = tel26_item_value(w) as usize;
    covered!("tel23_2 league progress row");
    if row == get_screen_param(1) {
        set_screen_param(1, 0);
    } else if row == get_screen_param(2) {
        set_screen_param(2, 0);
    } else if get_screen_param(1) == 0 {
        set_screen_param(1, row);
    } else {
        set_screen_param(2, row);
    }
    -11
}

/// Rule: opening the FIFA world rankings screen (builder `comp_fifa_rankings_screen_draw`, input
/// `comp_fifa_rankings_screen_handle_event`, no flag, leave `comp_fifa_rankings_screen_close`): params 0 and 1 (the
/// ranked list and its count) 0, 2 (the page) and 3 (the pages) 1.
/// Original: `comp_open_fifa_rankings_screen` (0x49c0b0, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_open_fifa_rankings_screen() {
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x49c120usize, 0x49c510usize, 0, 0x49c6c0usize, 0) == 0 {
        return;
    }
    covered!("tel23_2 open fifa rankings");
    for (k, v) in FIFA_RANKINGS_OPEN_PARAMS.into_iter().enumerate() {
        set_screen_param(k as u32, v);
    }
}

/// The four params `comp_open_fifa_rankings_screen` writes.
const FIFA_RANKINGS_OPEN_PARAMS: [usize; 4] = [0, 0, 1, 1];

/// Rule: building the FIFA world rankings screen: one page of the ranked nations.
///
/// Params: 0 the ranked list (built the first time: `fifa_rankings_collect_ranked_order` of the FIFA rankings object,
/// [G_NATIONAL_TEAMS] + 0x1c; none: the comp_screens.cpp box, line 0x1d0f), 1 its count, 2 the page. The nation picture,
/// the World Cup's colours with the title "FIFA World Rankings", the menu bar (4) and the subtitle "<month> <year>"
/// (the rankings date at +5). The page holds as many rows as 0x15d pixels in font 3; param 3 = the pages (count / rows,
/// one more for a remainder); the page buttons (`gui_add_page_buttons`, ids 1..4) at (0x30c, 0x91). A 4-column area
/// (weights 4, 0x16, 0x13, 5; x 0x6e..0x30c, y 0xaa..0x217): each nation of the page its rank (ordinal, dark blue),
/// "  <name>" (a button id 0x7d5 to its national team), its continent's name in yellow when it has one, and its
/// points (+0xc × 10, "%4.2f", colour 0xae3192). Back/Next.
/// Original: `comp_fifa_rankings_screen_draw` (0x49c120, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_fifa_rankings_screen_draw() {
    let weights: [u8; 4] = [4, 0x16, 0x13, 5];
    let mut list = get_screen_param(0);
    let mut count = get_screen_param(1) as i32;
    let page = get_screen_param(2);
    let nation_picture: unsafe extern "C" fn(u32, u32, u32) = core::mem::transmute(0x41c100usize);
    nation_picture(0, 0, 0);
    let colours = comp_colours(comp_record_of_key(KEY_CLUB_COMP_FIFA_WORLD_CUP));
    comp_lang_format()(JOB_TEXT, 0x99c8ac); // "FIFA World Rankings"
    comp_colours_title(colours, JOB_TEXT);
    exe::menubar_build(4, 0);
    let rankings = read_usize(read_usize(G_NATIONAL_TEAMS) + 0x1c);
    let year = read_u16(rankings + 7) as u32;
    let month_name: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x54d3b0usize);
    let month = month_name(exe::date_get_month(rankings + 5) as u32);
    comp_lang_format()(JOB_TEXT, 0x99c890, month, year); // "<%s - month> <%d - year>"
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, JOB_TEXT, 1);
    let rows = (0x15d / exe::font_get_height(3)) as u8;
    if list == 0 {
        list = exe::fifa_rankings_collect_ranked_order(rankings, &mut count);
        if list == 0 {
            comp_screens_error_box(0x1d0f);
            return;
        }
        covered!("tel23_2 fifa rankings list built");
        set_screen_param(0, list);
        set_screen_param(1, count as usize);
    }
    let per_page = rows as i32;
    let pages = count / per_page + (count % per_page != 0) as i32;
    set_screen_param(3, pages as i16 as isize as usize);
    let page_buttons: unsafe extern "C" fn(u32, u32, usize, i32, u32, u32, u32, u32) = core::mem::transmute(0x5ede20usize);
    page_buttons(0x30c, 0x91, page, pages, 1, 2, 3, 4);
    let area = tel26_add_area(current_human_display(), [0x6e, 0xaa, 0x30c, 0x217, 4, weights.as_ptr() as u32, rows as u32, 0, 2, 0, u32::MAX]);
    let first = (page as i16 as i32 - 1) * per_page;
    for k in 0..per_page {
        if first + k >= count {
            break;
        }
        let rank = exe::utils_format_ordinal(first + k + 1, -1);
        tel26_area_text([area, 0, k as u32, 0x30, job_colour(G_GUI_COLOUR_DARK_BLUE), 0xc, 1, job_colour(G_GUI_COLOUR_PALE_GREY), rank as u32]);
        let entry = read_usize(list + (first + k) as usize * 4);
        let nation = read_usize(G_NATION_TABLE) + read_u32(entry) as usize * nation::SIZE;
        comp_crt_format()(JOB_TEXT, 0x987ff0, nation + 4); // "  %s"
        let team = exe::database_get_national_team(nation);
        tel26_area_button([area, 1, k as u32, 1, 0, 1, 3, job_colour(G_GUI_COLOUR_PALE_GREY), JOB_TEXT as u32, 0x7d5, team as u32]);
        let continent = read_usize(nation + nation::CONTINENT);
        if continent != 0 {
            tel26_area_text([area, 2, k as u32, 1, 0, 1, 2, job_colour(G_GUI_COLOUR_YELLOW), (continent + 4) as u32]);
        }
        let points = read_f32(entry + 0xc) as f64 * read_f32(rdata::F32_10) as f64;
        comp_crt_format()(JOB_TEXT, 0x98fe94, points); // "%4.2f"
        tel26_area_text([area, 3, k as u32, 0x30, job_colour(JOB_COLOUR_3192), 0xc, 1, job_colour(G_GUI_COLOUR_PALE_GREY), JOB_TEXT as u32]);
    }
    tel26_back_next();
}

/// Rule: the FIFA world rankings screen's clicks: the page buttons move param 2: id 1 ten pages back, id 2 one back
/// (not below 1), id 3 one on, id 4 ten on (not past param 3, the pages); −11. Other ids go to the menu bar.
/// Original: `comp_fifa_rankings_screen_handle_event` (0x49c510, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_fifa_rankings_screen_handle_event(widget: u32) -> i32 {
    let page = get_screen_param(2) as i16 as i32;
    let pages = get_screen_param(3) as i16 as i32;
    let new = match tel26_item_id(widget as i16) {
        1 => (page - 10).max(1),
        2 => (page - 1).max(1),
        3 => (page + 1).min(pages),
        4 => (page + 10).min(pages),
        _ => return comp_click_menubar(widget),
    };
    covered!("tel23_2 fifa rankings page");
    set_screen_param(2, new as i16 as isize as usize);
    -11
}

/// Rule: leaving the FIFA world rankings screen frees the ranked list (param 0, crt free) and clears params 0 and 1.
/// Original: `comp_fifa_rankings_screen_close` (0x49c6c0, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_fifa_rankings_screen_close() {
    comp_free_block_param(0);
}

/// Rule: opening the UEFA coefficients screen (builder `comp_uefa_coefficients_screen_draw`, input
/// `comp_uefa_coefficients_screen_handle_event`, no flag, no leave): params 0 (the scroll row) and 1 (the area) 0.
/// Original: `comp_open_uefa_coefficients_screen` (0x49c700, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_open_uefa_coefficients_screen() {
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x49c740usize, 0x49cc10usize, 0, 0usize, 0) == 0 {
        return;
    }
    covered!("tel23_2 open uefa coefficients");
    set_screen_param(0, 0);
    set_screen_param(1, 0);
}

/// A UEFA coefficients table entry (0x12 bytes): +4 the nation, +8 the coefficient (f32), +0xc/+0xd/+0xe the places in
/// the European Cup, the UEFA Cup and the Intertoto Cup [inference].
const UEFA_COEFFICIENT_ENTRY: usize = 0x12;

/// Rule: building the UEFA coefficients screen: the nations' coefficients and their European places.
///
/// The European Cup's picture and colours with the title "UEFA Coefficients", the menu bar (4) and the subtitle
/// "<year>/<next year % 100>" (the seeding's year, +0x439). A 6-column area (weights 4, 0x15, 6, 6, 6, 7; x 0x6e..0x30c,
/// y 0x91..0x217, as many rows as 0x176 pixels in font 3). Row 0 heads the columns: "ECC", "UEFA" and "INTER" as
/// buttons (id 0x7d0) to the European Cup, the UEFA Cup and the Intertoto Cup, and "Coef.". Then each nation of the
/// seeding's table (`UefaSeeding::get_nation_table`, its count as a signed byte), from row 1: its rank (ordinal, dark
/// blue), "  <short name>" (a button id 0x7d5 to its national team), its three places in yellow and its coefficient
/// ("%4.2f", colour 0xae3192). The scroll row from param 0 and the area to param 1; Back/Next.
/// Original: `comp_uefa_coefficients_screen_draw` (0x49c740, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_uefa_coefficients_screen_draw() {
    let weights: [u8; 6] = [4, 0x15, 6, 6, 6, 7];
    let european_cup = comp_record_of_key(KEY_CLUB_COMP_UEFA_EUROPEAN_CHAMPIONS_LEAGUE);
    exe::background_set_comp_picture(european_cup, 0, 0);
    let colours = comp_colours(comp_record_of_key(KEY_CLUB_COMP_UEFA_EUROPEAN_CHAMPIONS_LEAGUE));
    comp_lang_format()(JOB_TEXT, 0x99c924); // "UEFA Coefficients"
    comp_colours_title(colours, JOB_TEXT);
    exe::menubar_build(4, 0);
    let year = read_u16(read_usize(G_UEFA_SEEDING) + 0x439) as i32;
    comp_lang_format()(JOB_TEXT, 0x99c908, year, (year + 1) % 100); // "<%d - year 1>/<%d - year 2>"
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, JOB_TEXT, 1);
    let rows = (0x176 / exe::font_get_height(3)) as u8;
    let area = tel26_add_area(current_human_display(), [0x6e, 0x91, 0x30c, 0x217, 6, weights.as_ptr() as u32, rows as u32, 0, 2, 0, u32::MAX]);
    let (grey, pale) = (job_colour(G_GUI_COLOUR_GREY), job_colour(G_GUI_COLOUR_PALE_GREY));
    comp_lang_format()(JOB_TEXT, 0x99c904); // "ECC"
    tel26_area_button([area, 2, 0, 0x30, grey, 0xc, 1, pale, JOB_TEXT as u32, 0x7d0, comp_record_of_key(KEY_CLUB_COMP_UEFA_EUROPEAN_CHAMPIONS_LEAGUE) as u32]);
    comp_lang_format()(JOB_TEXT, 0x99c8fc); // "UEFA"
    tel26_area_button([area, 3, 0, 0x30, grey, 0xc, 1, pale, JOB_TEXT as u32, 0x7d0, comp_record_of_key(KEY_CLUB_COMP_UEFA_CUP) as u32]);
    comp_lang_format()(JOB_TEXT, 0x99c8c8, 0xad9c64usize); // "INTER<%s - COMMENT - Intertoto Cup abbreviation>"
    tel26_area_button([area, 4, 0, 0x30, grey, 0xc, 1, pale, JOB_TEXT as u32, 0x7d0, comp_record_of_key(KEY_CLUB_COMP_INTER_TOTO_CUP) as u32]);
    comp_lang_format()(JOB_TEXT, 0x99c8c0); // "Coef."
    tel26_area_text([area, 5, 0, 0x30, grey, 0xc, 1, pale, JOB_TEXT as u32]);
    let mut table = 0usize;
    let nation_table: unsafe extern "thiscall" fn(usize, *mut usize) -> u8 = core::mem::transmute(0x903590usize);
    let n = nation_table(read_usize(G_UEFA_SEEDING), &mut table) as i8 as i32;
    for k in 1..=n {
        let entry = table + (k - 1) as usize * UEFA_COEFFICIENT_ENTRY;
        let rank = exe::utils_format_ordinal(k, -1);
        tel26_area_text([area, 0, k as u32, 0x30, job_colour(G_GUI_COLOUR_DARK_BLUE), 0xc, 1, job_colour(G_GUI_COLOUR_PALE_GREY), rank as u32]);
        let nation = read_usize(entry + 4);
        comp_crt_format()(JOB_TEXT, 0x987ff0, nation + nation::SHORT_NAME); // "  %s"
        let team = exe::database_get_national_team(nation);
        tel26_area_button([area, 1, k as u32, 1, 0, 1, 3, job_colour(G_GUI_COLOUR_PALE_GREY), JOB_TEXT as u32, 0x7d5, team as u32]);
        for (column, offset) in [(2u32, 0xcusize), (3, 0xd), (4, 0xe)] {
            comp_crt_format()(JOB_TEXT, 0x98806c, read_u8(entry + offset) as i8 as i32); // "%d"
            tel26_area_text([area, column, k as u32, 1, 0, 0xc, 2, job_colour(G_GUI_COLOUR_YELLOW), JOB_TEXT as u32]);
        }
        comp_crt_format()(JOB_TEXT, 0x98fe94, read_f32(entry + 8) as f64); // "%4.2f"
        tel26_area_text([area, 5, k as u32, 0x30, job_colour(JOB_COLOUR_3192), 0xc, 1, job_colour(G_GUI_COLOUR_PALE_GREY), JOB_TEXT as u32]);
    }
    comp_restore_area_scroll(area, 0);
    set_screen_param(1, area as i16 as isize as usize);
    tel26_back_next();
}

/// Rule: the UEFA coefficients screen's clicks: with a widget the table keeps its scroll row (param 0 from the area in
/// 1); every click goes to the menu bar.
/// Original: `comp_uefa_coefficients_screen_handle_event` (0x49cc10, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_uefa_coefficients_screen_handle_event(widget: u32) -> i32 {
    if widget as i16 != -1 {
        comp_click_keep_scroll(0, 1);
    }
    comp_click_menubar(widget)
}

/// Comparer for `comp_fixture_resolve_league_clubs`: side by side by value (the answer and the three outputs), the
/// error box (an argument missing) only the original.
pub(crate) unsafe extern "C" fn compare_comp_fixture_resolve_league_clubs(fixture: usize, league_out: *mut usize, home_out: *mut usize, away_out: *mut usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, *mut usize, *mut usize, *mut usize) -> u32>(%%I3%%);
    if fixture == 0 || league_out.is_null() || home_out.is_null() || away_out.is_null() {
        return o(fixture, league_out, home_out, away_out);
    }
    let (mut lo, mut ho, mut ao) = (0usize, 0usize, 0usize);
    let (mut lr, mut hr, mut ar) = (0usize, 0usize, 0usize);
    let ro = o(fixture, &mut lo, &mut ho, &mut ao);
    let rr = comp_fixture_resolve_league_clubs(fixture, &mut lr, &mut hr, &mut ar);
    SCOREBOARD.record((ro, lo, ho, ao) == (rr, lr, hr, ar), || format!("comp_fixture_resolve_league_clubs({:#x}): {} {:#x} {:#x} {:#x} x {} {:#x} {:#x} {:#x}", fixture, ro, lo, ho, ao, rr, lr, hr, ar));
    *league_out = lo;
    *home_out = ho;
    *away_out = ao;
    ro
}

/// Comparer for `comp_open_league_progress_for_fixture`: only the original (it opens a screen); the params checked
/// against the plan afterwards when the screen opened (param 0 the league found).
pub(crate) unsafe extern "C" fn compare_comp_open_league_progress_for_fixture(fixture: usize) {
    let (mut league, mut home, mut away) = (0usize, 0usize, 0usize);
    let checkable = fixture != 0 && (fixture as i32).wrapping_abs() >= 10000;
    if checkable {
        comp_fixture_resolve_league_clubs(fixture, &mut league, &mut home, &mut away);
    }
    original_fn::<unsafe extern "C" fn(usize)>(%%I0%%)(fixture);
    if checkable && league != 0 && get_screen_param(0) == league {
        let plan = league_progress_planned_params(league, home, away);
        let got: [usize; 5] = core::array::from_fn(|k| get_screen_param(k as u32));
        SCOREBOARD.record(got == plan, || format!("league progress screen (open {:#x}): params {:x?} x plan {:x?}", fixture, got, plan));
    }
}

/// Comparer for `comp_league_progress_screen_draw`: side by side over the display list (plus the league's table
/// column; the sort finds the same order twice), only the original without a league (the error box).
pub(crate) unsafe extern "C" fn compare_comp_league_progress_screen_draw() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I1%%);
    let league = get_screen_param(0);
    if league == 0 {
        return o();
    }
    compare_match_screen_build("league progress screen", &[(league + league::TABLE_COLUMN, 4)], || o(), || comp_league_progress_screen_draw());
}

/// Comparer for `comp_league_progress_screen_handle_event`: side by side over the screen manager and the params for
/// the table rows (id 1); the menu bar only the original.
pub(crate) unsafe extern "C" fn compare_comp_league_progress_screen_handle_event(widget: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I2%%);
    let pure = tel26_item_id(widget as i16) == 1;
    comp_click_compare("league progress handle_event", widget, pure, || o(widget), || comp_league_progress_screen_handle_event(widget))
}

/// Comparer for `comp_open_fifa_rankings_screen`: only the original, params 0..3 checked afterwards when it opened
/// (param 2 = 1 with no list).
pub(crate) unsafe extern "C" fn compare_comp_open_fifa_rankings_screen() {
    original_fn::<unsafe extern "C" fn()>(%%I4%%)();
    let got: [usize; 4] = core::array::from_fn(|k| get_screen_param(k as u32));
    if got[0] == 0 && got[2] == 1 {
        SCOREBOARD.record(got == FIFA_RANKINGS_OPEN_PARAMS, || format!("fifa rankings screen (open): params {:x?} x plan {:x?}", got, FIFA_RANKINGS_OPEN_PARAMS));
    }
}

/// Comparer for `comp_fifa_rankings_screen_draw`: side by side over the display list once the ranked list exists; its
/// first build (it allocates the list into params 0 and 1) only the original.
pub(crate) unsafe extern "C" fn compare_comp_fifa_rankings_screen_draw() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I5%%);
    if get_screen_param(0) == 0 {
        // simplification: the allocation does not run twice; proven in Rust mode
        return o();
    }
    compare_match_screen_build("fifa rankings screen", &[], || o(), || comp_fifa_rankings_screen_draw());
}

/// Comparer for `comp_fifa_rankings_screen_handle_event`: side by side for the page buttons (ids 1..4); the menu bar
/// only the original.
pub(crate) unsafe extern "C" fn compare_comp_fifa_rankings_screen_handle_event(widget: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I6%%);
    let pure = (1..=4).contains(&tel26_item_id(widget as i16));
    comp_click_compare("fifa rankings handle_event", widget, pure, || o(widget), || comp_fifa_rankings_screen_handle_event(widget))
}

/// Comparer for `comp_fifa_rankings_screen_close`: only the original (it frees the list), params 0 and 1 checked
/// afterwards (cleared when there was a list).
pub(crate) unsafe extern "C" fn compare_comp_fifa_rankings_screen_close() {
    let before = (get_screen_param(0), get_screen_param(1));
    original_fn::<unsafe extern "C" fn()>(%%I7%%)();
    let plan = if before.0 != 0 { (0, 0) } else { before };
    let got = (get_screen_param(0), get_screen_param(1));
    SCOREBOARD.record(got == plan, || format!("fifa rankings screen (close): params 0/1 {:x?} x plan {:x?}", got, plan));
}

/// Comparer for `comp_open_uefa_coefficients_screen`: only the original (it opens a screen).
pub(crate) unsafe extern "C" fn compare_comp_open_uefa_coefficients_screen() {
    // simplification: opening a screen does not run twice; the two zero params are not worth a plan
    original_fn::<unsafe extern "C" fn()>(%%I8%%)()
}

/// Comparer for `comp_uefa_coefficients_screen_draw`: side by side over the display list.
pub(crate) unsafe extern "C" fn compare_comp_uefa_coefficients_screen_draw() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I9%%);
    compare_match_screen_build("uefa coefficients screen", &[], || o(), || comp_uefa_coefficients_screen_draw());
}

/// Comparer for `comp_uefa_coefficients_screen_handle_event`: only the original (every click goes to the menu bar).
pub(crate) unsafe extern "C" fn compare_comp_uefa_coefficients_screen_handle_event(widget: u32) -> i32 {
    // simplification: the menu bar opens screens; proven in Rust mode
    original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I10%%)(widget)
}
