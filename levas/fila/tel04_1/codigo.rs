// ---------- batch %%LEVA%%: past meetings and the friendly dates (club_screens.cpp, part 4) ----------

/// The club_screens.cpp source path the error boxes print.
pub(crate) const CLUB_SCREENS_CPP_FILE: usize = 0x98e598;
/// The past meetings caches: one 42-byte record per human manager (by staff id minus the staff count) and the one
/// used when the slot has no manager [inference].
const PAST_MEETINGS_CACHES: usize = 0xadadf0;
const PAST_MEETINGS_DEFAULT_CACHE: usize = 0xadab50;
/// The past meetings cache (42 bytes): the date of the search, the two clubs, the reserves flag and the filter it
/// was made for, the list of the meetings found (LLIST of 0x4f-byte fixture copies) and their head-to-head table
/// (a SubLeague of two teams).
mod past_meetings_cache {
    pub(crate) const DATE: usize = 0x0;
    pub(crate) const CLUB: usize = 0x8;
    pub(crate) const OPPOSITION: usize = 0xc;
    pub(crate) const RESERVES: usize = 0x10;
    pub(crate) const FILTER: usize = 0x14;
    pub(crate) const LIST: usize = 0x18;
    pub(crate) const TABLE: usize = 0x26;
    pub(crate) const SIZE: usize = 0x2a;
}
/// The fixture record fields the club screens read (the copies of the fixture lists hold the first 0x4f bytes).
#[allow(dead_code)]
mod screen_fixture {
    pub(crate) const MATCH_ID: usize = 0x8;
    pub(crate) const COMP: usize = 0x14;
    pub(crate) const HOME: usize = 0x1c;
    pub(crate) const AWAY: usize = 0x20;
    pub(crate) const YEAR: usize = 0x28;
    pub(crate) const DAY: usize = 0x2a;
    pub(crate) const HOME_GOALS: usize = 0x43;
    pub(crate) const AWAY_GOALS: usize = 0x44;
    /// i8 x2: the score shown instead when the first is not negative (after extra time / penalties) [inference].
    pub(crate) const FINAL_HOME_GOALS: usize = 0x47;
    pub(crate) const FINAL_AWAY_GOALS: usize = 0x48;
}
/// Colour words of the screens that have no name in `globals.rs`.
pub(crate) const SCREEN_COLOUR_BDDE: usize = 0xaebdde;
pub(crate) const SCREEN_COLOUR_3154: usize = 0xae3154;
/// The text the friendly dates rows end with (0x98f858).
const FRIENDLY_DATES_ROW_END: usize = 0x98f858;

/// The club_screens.cpp "Error" box at `line` (the game goes on).
pub(crate) unsafe fn club_pages_error_box(line: u32) {
    tel26_error_box(CLUB_SCREENS_CPP_FILE, line);
}

/// `langlib_sprintf` (cdecl, variadic).
pub(crate) unsafe fn club_pages_lang() -> unsafe extern "C" fn(usize, usize, ...) -> i32 {
    core::mem::transmute(FN_LANGLIB_SPRINTF)
}

/// The CRT `sprintf` (cdecl, variadic).
pub(crate) unsafe fn club_pages_format() -> unsafe extern "C" fn(usize, usize, ...) -> i32 {
    core::mem::transmute(FN_CRT_SPRINTF)
}

/// `Display::add_item` on the current human's Display.
pub(crate) unsafe fn club_pages_item(f: DisplayItemFields) -> u32 {
    tel26_add_item(current_human_display(), f)
}

/// `Display::add_area` on the current human's Display (returns eax, the area in ax).
pub(crate) unsafe fn club_pages_area(f: [u32; 11]) -> u32 {
    tel26_add_area(current_human_display(), f)
}

/// The id of the clicked widget on the current human's Display (0 and −1 become the default command id) as the
/// handlers compare it.
pub(crate) unsafe fn club_pages_id(widget: u32) -> i16 {
    tel26_item_id(widget as i16) as i16
}

/// The scroll row of the area in screen param `area_param` saved in param `save` (when the widget is real).
pub(crate) unsafe fn club_pages_save_scroll(widget: u32, save: u32, area_param: u32) {
    if widget as i16 != -1 {
        set_screen_param(save, exe::screenmanager_get_area_scroll_row(G_SCREEN_MANAGER, get_screen_param(area_param)));
    }
}

/// The scroll row of screen param `row_param` put back on area `area` (when there is one) and the area written to
/// param `area_param`.
pub(crate) unsafe fn club_pages_restore_scroll(area: u32, row_param: u32, area_param: u32) {
    let row = get_screen_param(row_param);
    if area as i16 != -1 {
        exe::area_set_scroll_row(display_area_at(current_human_display(), area as i16), row);
    }
    set_screen_param(area_param, area as i16 as isize as usize);
}

/// The menu bar's answer to a click no screen handler took: its commands, then the object links.
pub(crate) unsafe fn club_pages_menu_bar(widget: u32) -> i32 {
    let r = exe::menubar_handle_command(widget);
    if r != 0 {
        return r;
    }
    exe::menubar_handle_object_link(widget)
}

/// The kit colours of `club` (`database_get_club_kit_colours`, mode 1) and the screen title with them.
pub(crate) unsafe fn club_pages_title(club: usize, text: usize) {
    let (mut first, mut second) = (0u32, 0u32);
    exe::database_get_club_kit_colours(club, 1, &mut first, &mut second);
    exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, second, first, text);
}

/// `comp_screens_open_comp_fixtures(comp, day, year, mode)` (0x48f8e0, cdecl).
pub(crate) unsafe fn club_pages_open_comp_fixtures(comp: usize, day: u16, year: u16, mode: u32) {
    let f: unsafe extern "C" fn(usize, u32, u32, u32) = core::mem::transmute(0x48f8e0usize);
    f(comp, day as u32, year as u32, mode);
}

/// `Date::Date(this, day, year)` (0x549ef0, thiscall): returns `this`.
pub(crate) unsafe fn club_pages_date(date: *mut u32, day: u16, year: u16) -> usize {
    let f: unsafe extern "thiscall" fn(*mut u32, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE);
    f(date, day as u32, year as u32)
}

/// `Date::get_short_date_string(this, out, size, time)` (0x54a650, thiscall).
unsafe fn club_pages_short_date(date: usize, out: usize, size: u32, time: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> i32 = core::mem::transmute(0x54a650usize);
    f(date, out, size, time);
}

/// `Date::add_days(this, out, days)` (0x54c5b0, thiscall): returns `out`.
pub(crate) unsafe fn club_pages_add_days(date: usize, out: *mut u32, days: i16) -> usize {
    let f: unsafe extern "thiscall" fn(usize, *mut u32, i32) -> usize = core::mem::transmute(0x54c5b0usize);
    f(date, out, days as i32)
}

/// A Date copied like `Date::operator=` (0x417c10) does it (the 8 bytes; the hidden result is a scratch copy).
pub(crate) unsafe fn club_pages_copy_date(dst: usize, src: usize) {
    let mut scratch = [0u32; 2];
    exe::date_operator_3d(dst, scratch.as_mut_ptr() as usize, src);
}

/// The past meetings cache of the current screen slot: the record of its human manager (by staff id minus the staff
/// count), or the default one without a manager.
unsafe fn past_meetings_cache_of_slot() -> usize {
    let manager = read_usize((G_SCREEN_RECORDS_MANAGER as isize + read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize * 0x300) as usize);
    if manager == 0 {
        return PAST_MEETINGS_DEFAULT_CACHE;
    }
    (read_u32(manager).wrapping_sub(read_u32(G_STAFF_COUNT)).wrapping_mul(past_meetings_cache::SIZE as u32) as usize).wrapping_add(PAST_MEETINGS_CACHES)
}

/// Opens the past meetings screen of `club` against `opposition` (builder `past_meetings_screen_build`, handler
/// `past_meetings_screen_input`); either club missing shows the club_screens.cpp 11728 box instead. Params: 0 the club,
/// 1 the opposition, 2 the filter 0x221 (all time, anywhere, ignore friendlies), 3 the slot's past meetings cache, 4
/// and 5 the scroll row and area (0), 6 the reserves flag.
/// Original: `open_past_meetings_screen` (0x46e6b0, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn open_past_meetings_screen(club: usize, opposition: usize, reserves: u32) {
    if club == 0 || opposition == 0 {
        club_pages_error_box(0x2dd0);
        return;
    }
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x46e820usize, 0x46ffd0usize, 0, 0usize, 0) == 0 {
        return;
    }
    covered!("tel04_1 open past meetings");
    let cache = past_meetings_cache_of_slot();
    for (k, v) in [(0u32, club), (1, opposition), (2, 0x221), (3, cache), (4, 0), (5, 0), (6, reserves as usize)] {
        set_screen_param(k, v);
    }
}

/// One choice of the past meetings popups: ticked (0x2002, no id) when its filter bit is on, else a plain item with
/// the id and value that pick it.
unsafe fn past_meetings_choice(on: bool, id: u32, value: u32, area: u32) {
    let (blue, yellow, black) = (job_colour(0xae31aa), job_colour(G_GUI_COLOUR_YELLOW), job_colour(G_GUI_COLOUR_BLACK));
    let kind = if on { 0x2002 } else { 2 };
    club_pages_item([kind, 0, 0, 0, 0, 0, 0, 0x10, blue, yellow, 1, 1, black, JOB_TEXT as u32, 0, id, value, area]);
}

/// "At <club>" or, with a stadium, "At <stadium>" into the text buffer.
unsafe fn past_meetings_at(club: usize) {
    let lang = club_pages_lang();
    let stadium = read_usize(club + club::STADIUM);
    if stadium == 0 {
        let c = read_i8(club + club::SHORT_GENDER_NAME) as i32;
        lang(JOB_TEXT, 0x990c00, c, c, club + club::SHORT_NAME); // "At {}<%s - club>{}"
    } else {
        let c = read_i8(stadium + 0x37) as i32;
        lang(JOB_TEXT, 0x990c14, c, c, stadium + 4); // "At {}<%s - stadium>{}"
    }
}

/// Whether a meeting's competition gets a link: a competition with an object (0xadadfc by the fixture's first field),
/// not a reserve or minor one, not a friendly's own record, and current on the meeting's date.
unsafe fn past_meetings_comp_linked(f: usize) -> bool {
    let comp = read_usize(f + screen_fixture::COMP);
    if comp == 0 || read_u32(read_usize(G_COMP_OBJECT_TABLE) + read_u32(f) as usize * 4) == 0 {
        return false;
    }
    if exe::comp_is_reserve_or_minor_comp(read_usize(G_RESERVE_COMP), comp) != 0 || exe::friendly_is_own_comp_record(read_usize(G_FRIENDLY), comp) != 0 {
        return false;
    }
    let mut date = [0u32; 2];
    let d = club_pages_date(date.as_mut_ptr(), read_u16(f + screen_fixture::DAY), read_u16(f + screen_fixture::YEAR));
    let current: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x5c29b0usize);
    current(read_usize(G_FRIENDLY), comp, d) != 0
}

/// The past meetings search: the date, the clubs, the reserves flag and the filter are stored in the cache, its list
/// is emptied and the fixtures of the club from the period's start (game start, or today minus 3, 5, 10 or 25 years)
/// to today are found (`FIXTURE_MANAGER::find_fixtures`, reserves when the flag is 0); the played ones against the
/// opposition that pass the venue filters (0x40 home, 0x80 away, 0x100 neutral) and, with 0x400, are not friendlies
/// go into the cache's list; then the head-to-head table (a SubLeague of the two clubs over that list) replaces the
/// old one.
unsafe fn past_meetings_search(cache: usize, club: usize, opposition: usize, reserves: usize, filter: u32) {
    let ctor: unsafe extern "thiscall" fn(*mut u8) = core::mem::transmute(FN_LLIST_LLIST);
    let dtor: unsafe extern "thiscall" fn(*mut u8) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let mut found = [0u8; 0xe];
    let list = found.as_mut_ptr();
    ctor(list);
    club_pages_copy_date(cache + past_meetings_cache::DATE, G_CURRENT_DATE);
    write_usize(cache + past_meetings_cache::CLUB, club);
    write_usize(cache + past_meetings_cache::OPPOSITION, opposition);
    write_usize(cache + past_meetings_cache::RESERVES, reserves);
    write_u32(cache + past_meetings_cache::FILTER, filter);
    exe::llist_clear(cache + past_meetings_cache::LIST);
    let today = read_u32(G_CURRENT_DATE);
    let (day, year) = (today & 0xffff, today >> 16);
    let id = read_u32(club);
    let first_team = (reserves == 0) as u32;
    // simplification: the original passes the start and end dates as dwords whose high halves are register
    // garbage (the club id, a stack address); find_fixtures reads their words
    let back = [(2u32, 3u32), (4, 5), (8, 10), (0x10, 25)].iter().find(|(bit, _)| filter & bit != 0).map(|&(_, years)| years);
    let (from_day, from_year) = match back {
        Some(years) => (today, year.wrapping_sub(years) & 0xffff), // tel04_1 the period start
        None => (read_u16(G_GAME_START_DATE) as u32, (id & 0xffff_0000) | read_u16(G_GAME_START_DATE + 2) as u32),
    };
    exe::fixture_manager_find_fixtures(read_usize(G_FIXTURE_MANAGER), from_day, from_year, (id & 0xffff_0000) | day, year, id, list as usize, u32::MAX, 0xffff_fffe, first_team);
    if read_i16(list as usize + llist::COUNT) != 0 {
        write_u32(list as usize + llist::CURSOR, read_u32(list as usize + llist::HEAD));
        loop {
            let f = exe::llist_get_next(list as usize);
            if f == 0 {
                break;
            }
            if read_i8(f + screen_fixture::HOME_GOALS) <= -1 || (read_usize(f + screen_fixture::HOME) != opposition && read_usize(f + screen_fixture::AWAY) != opposition) {
                continue;
            }
            let side = exe::comp_util_get_fixture_venue_side(club, f);
            if (filter & 0x40 != 0 && side != 1) || (filter & 0x80 != 0 && side != 2) || (filter & 0x100 != 0 && side != 3) {
                continue;
            }
            if filter & 0x400 != 0 && exe::friendly_is_friendly_competition(read_usize(G_FRIENDLY), read_usize(f + screen_fixture::COMP)) != 0 {
                continue;
            }
            exe::llist_add_tail(cache + past_meetings_cache::LIST, f, 0x4f);
        }
    }
    let ids = [read_u32(club), read_u32(opposition)];
    let rules: [u8; 3] = [1, 2, 3];
    let old = read_usize(cache + past_meetings_cache::TABLE);
    if old != 0 {
        let destroy: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(read_usize(read_usize(old)));
        destroy(old, 1);
    }
    let room = exe::crt_operator_new(0xee);
    let table = if room != 0 {
        type SubLeague = unsafe extern "thiscall" fn(usize, u32, *const u32, i32, i32, u32, u32, i32, u32, u32, *const u8, i32, usize) -> usize;
        let make: SubLeague = core::mem::transmute(0x88c6d0usize);
        make(room, 2, ids.as_ptr(), -1, -1, 0, 0, -1, 3, 1, rules.as_ptr(), -1, cache + past_meetings_cache::LIST)
    } else {
        0
    };
    write_usize(cache + past_meetings_cache::TABLE, table);
    dtor(list);
}

/// Builds the past meetings screen of club (param 0) against the opposition (param 1).
///
/// The club's background picture, its name (with the reserves flag, param 6) in its kit colours as the title, "Past
/// Meetings With <opposition>" as the subtitle and the menu bar. The View popup: All Time (bit 1), Last 3, 5, 10 and 25
/// Years (2, 4, 8, 0x10); the Filter popup: Anywhere (0x20), At <club's stadium or club> (0x40), At <opposition's>
/// (0x80), On Neutral Ground (0x100), a separator and Ignore Friendlies (0x400, its value 0x200 when ticked); the ticked
/// choices are the bits of the filter (param 2). When the cache (param 3) was made for another day, clubs, reserves
/// flag or filter, the search runs again (`past_meetings_search`). Then the head-to-head table (Pld, Won, Drn, Lst,
/// For, Ag, Pts; a row per club of the sorted table: place, linked name in yellow for a human's club, the figures) and
/// one row per meeting: the date (linked to the fixture when the competition is linked), the home club, the score (the
/// final one when there is one; linked to the match when it has stats), the away club, H/A/N for the club and the
/// round (linked to the competition); empty rows fill the area, whose scroll row comes from param 4 and goes to param 5;
/// the Back and Next buttons.
/// Original: `past_meetings_screen_build` (0x46e820, club_screens.cpp, cdecl, SEH).
pub(crate) unsafe extern "C" fn past_meetings_screen_build() {
    let lang = club_pages_lang();
    let format = club_pages_format();
    let table_weights: [u8; 11] = [4, 2, 0x10, 0, 4, 4, 4, 4, 4, 4, 4];
    let list_weights: [u8; 7] = [3, 7, 2, 7, 1, 10, 3];
    let club = get_screen_param(0);
    let opposition = get_screen_param(1);
    let filter = get_screen_param(2) as u32;
    let cache = get_screen_param(3);
    let reserves = get_screen_param(6);
    covered!("tel04_1 past meetings build");
    exe::background_set_club_picture(club, 0, 0);
    exe::club_get_national_team_name(club, reserves, JOB_TEXT, 0x7d0, 3, 1);
    club_pages_title(club, JOB_TEXT);
    let c = read_i8(opposition + club::SHORT_GENDER_NAME) as i32;
    lang(JOB_TEXT, 0x990c84, c, c, opposition + club::SHORT_NAME); // "Past Meetings With {}<%s - opposition club>{}"
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, JOB_TEXT, 1);
    exe::menubar_build(4, 0);
    let (grey, pale) = (job_colour(G_GUI_COLOUR_GREY), job_colour(G_GUI_COLOUR_PALE_GREY));
    let blue = job_colour(0xae31aa);

    lang(JOB_TEXT, 0x988004); // "View"
    let button = tel26_add_button([0x6e, 0x91, 0xeb, 0xa5, 0x30, grey, 0xc, 1, pale, JOB_TEXT as u32, 0, 0]);
    let view = club_pages_area([0, 0, 0, 0, 0, 0, 0, 0, 0x130, blue, button]);
    lang(JOB_TEXT, 0x990c78); // " All Time"
    past_meetings_choice(filter & 1 != 0, (filter & 1 == 0) as u32, (filter & 1 == 0) as u32, view);
    for (bit, name) in [(2u32, 0x990c68usize), (4, 0x990c58), (8, 0x990c48), (0x10, 0x990c38)] {
        lang(JOB_TEXT, name); // " Last 3/5/10/25 Years"
        let on = filter & bit != 0;
        past_meetings_choice(on, !on as u32, if on { 0 } else { bit }, view);
    }
    lang(JOB_TEXT, 0x98ebf8); // "Filter"
    let button = tel26_add_button([0x28f, 0x91, 0x30c, 0xa5, 0x30, grey, 0xc, 1, pale, JOB_TEXT as u32, 0, 0]);
    let venue = club_pages_area([0, 0, 0, 0, 0, 0, 0, 0, 0x130, blue, button]);
    lang(JOB_TEXT, 0x990c2c); // " Anywhere"
    let on = filter & 0x20 != 0;
    past_meetings_choice(on, !on as u32, if on { 0 } else { 0x20 }, venue);
    for (bit, who) in [(0x40u32, club), (0x80, opposition)] {
        past_meetings_at(who);
        let on = filter & bit != 0;
        past_meetings_choice(on, !on as u32, if on { 0 } else { bit }, venue);
    }
    lang(JOB_TEXT, 0x990bec); // " On Neutral Ground"
    let on = filter & 0x100 != 0;
    past_meetings_choice(on, !on as u32, if on { 0 } else { 0x100 }, venue);
    club_pages_item([1, 0, 0, 0, 0, 0, 0, 0x1000010, blue, job_colour(G_GUI_COLOUR_YELLOW), 1, 1, job_colour(G_GUI_COLOUR_BLACK), 0, 0, 0, 0, venue]);
    lang(JOB_TEXT, 0x990bd8); // " Ignore Friendlies"
    let on = filter & 0x400 != 0;
    past_meetings_choice(on, 1, if on { 0x200 } else { 0x400 }, venue);

    let today = read_u32(G_CURRENT_DATE);
    if read_u16(cache + 2) != (today >> 16) as u16 || read_u16(cache) != today as u16 || read_usize(cache + past_meetings_cache::CLUB) != club
        || read_usize(cache + past_meetings_cache::OPPOSITION) != opposition || read_usize(cache + past_meetings_cache::RESERVES) != reserves
        || read_u32(cache + past_meetings_cache::FILTER) != filter {
        covered!("tel04_1 past meetings search");
        past_meetings_search(cache, club, opposition, reserves, filter);
    }

    let h3 = exe::font_get_height(3);
    let table_area = club_pages_area([0x6e, 0xaa, 0x30c, (h3 * 3 + 0xba) as u32, 0xb, table_weights.as_ptr() as u32, 3, 0, 2, 0, u32::MAX]);
    for (col, title, arg) in [(4u32, 0x990b3cusize, 0usize), (5, 0x990b1c, 0xad9c64), (6, 0x990b14, 0), (7, 0x990af4, 0xad9c64), (8, 0x990af0, 0), (9, 0x990aec, 0), (10, 0x990ae8, 0)] {
        if arg != 0 {
            lang(JOB_TEXT, title, arg); // "Won" / "Lst"
        } else {
            lang(JOB_TEXT, title); // "Pld", "Drn", "For", "Ag", "Pts"
        }
        tel26_area_text([table_area, col, 0, 0x30, grey, 0xc, 1, pale, JOB_TEXT as u32]);
    }
    let table = read_usize(cache + past_meetings_cache::TABLE);
    if table != 0 {
        write_u32(table + 0xad, 0);
        exe::league_sort_table(table, 0usize);
        let mut k: i8 = 0;
        let mut row = 0u32;
        while (k as i16) < read_i16(read_usize(cache + past_meetings_cache::TABLE) + 0x3e) {
            let r = exe::league_get_table_row(read_usize(cache + past_meetings_cache::TABLE), k as i32);
            row += 1;
            let place = exe::utils_format_ordinal(k as i32 + 1, -1);
            tel26_area_text([table_area, 0, row, 0x30, job_colour(G_GUI_COLOUR_DARK_BLUE), 0xc, 1, pale, place as u32]);
            let c = read_usize(r);
            let colour = if exe::human_manager_is_human_club(c, 0, 1, 0, 0) != 0 { job_colour(G_GUI_COLOUR_YELLOW) } else { pale };
            tel26_area_button([table_area, 2, row, 1, 0, 1, 3, colour, (c + club::SHORT_NAME) as u32, 0x7d5, c as u32]);
            let figures = [
                read_i8(r + 0x11) as i32,
                read_i8(r + 0x12) as i32,
                read_i8(r + 0x13) as i32,
                read_i8(r + 0x14) as i32,
                read_i16(r + 0x15) as i32,
                read_i16(r + 0x17) as i32,
            ];
            for (n, v) in figures.iter().enumerate() {
                format(JOB_TEXT, 0x98806c, *v); // "%d"
                tel26_area_text([table_area, 4 + n as u32, row, 1, 0, 0xc, 2, pale, JOB_TEXT as u32]);
            }
            format(JOB_TEXT, 0x98806c, read_i16(r + 0x19) as i32);
            tel26_area_text([table_area, 10, row, 0x30, job_colour(0xae3192), 0xc, 1, pale, JOB_TEXT as u32]);
            k = k.wrapping_add(1);
        }
    }

    let top = h3 * 3 + 0xbf;
    let rows = ((0x217 - top) / exe::font_get_height(2)) as u8 as u32;
    let area = club_pages_area([0x6e, top as u32, 0x30c, 0x217, 7, list_weights.as_ptr() as u32, rows, 0, 1, 0, u32::MAX]);
    let list = cache + past_meetings_cache::LIST;
    write_u32(list + llist::CURSOR, read_u32(list + llist::HEAD));
    let mut row = 0u32;
    let (yellow, dark_blue) = (job_colour(G_GUI_COLOUR_YELLOW), job_colour(G_GUI_COLOUR_DARK_BLUE));
    loop {
        let f = exe::llist_get_next(list);
        if f == 0 {
            break;
        }
        let mut date = [0u32; 2];
        let d = club_pages_date(date.as_mut_ptr(), read_u16(f + screen_fixture::DAY), read_u16(f + screen_fixture::YEAR));
        club_pages_short_date(d, JOB_TEXT, 0x7d0, 0xff);
        if past_meetings_comp_linked(f) {
            tel26_area_button([area, 0, row, 0x30, dark_blue, 0xc, 1, pale, JOB_TEXT as u32, 2, f as u32]);
        } else {
            tel26_area_text([area, 0, row, 0x30, dark_blue, 0xc, 1, pale, JOB_TEXT as u32]);
        }
        for (col, side) in [(1u32, screen_fixture::HOME), (3, screen_fixture::AWAY)] {
            let c = read_usize(f + side);
            format(JOB_TEXT, 0x987ff0, c + club::SHORT_NAME); // "  %s"
            let colour = if exe::human_manager_is_human_club(c, 0, 1, 0, 0) != 0 { yellow } else { pale };
            tel26_area_button([area, col, row, 2, 0, 1, 2, colour, JOB_TEXT as u32, 0x7d5, c as u32]);
            if col == 1 {
                let (h, a) = if read_i8(f + screen_fixture::FINAL_HOME_GOALS) > -1 {
                    (read_i8(f + screen_fixture::FINAL_HOME_GOALS), read_i8(f + screen_fixture::FINAL_AWAY_GOALS))
                } else {
                    (read_i8(f + screen_fixture::HOME_GOALS), read_i8(f + screen_fixture::AWAY_GOALS))
                };
                lang(JOB_TEXT, 0x990bb4, h as i32, a as i32); // "<%d - home score>:<%d - away score>"
                let stats = exe::match_stats_fixture_entry_matches(read_usize(G_MATCH_STATS), read_i32(f + screen_fixture::MATCH_ID), read_i32(f + 0xc), read_i32(f + 0x10),
                    read_u16(f + screen_fixture::DAY) as i32, read_u16(f + screen_fixture::YEAR) as i32);
                if stats != 0 {
                    tel26_area_button([area, 2, row, 2, 0, 0xc, 2, yellow, JOB_TEXT as u32, 0x7d3, read_u32(f + screen_fixture::MATCH_ID)]);
                } else {
                    tel26_area_text([area, 2, row, 2, 0, 0xc, 2, yellow, JOB_TEXT as u32]);
                }
            }
        }
        let venue = match exe::comp_util_get_fixture_venue_side(club, f) {
            1 => 0x98f880usize, // "H<%s - COMMENT - home>"
            2 => 0x98f868,      // "A<%s - COMMENT - away>"
            _ => 0x990b98,      // "N<%s - COMMENT - neutral>"
        };
        lang(JOB_TEXT, venue, 0xad9c64usize);
        tel26_area_text([area, 4, row, 2, 0, 0xc, 2, job_colour(0xaebde8), JOB_TEXT as u32]);
        let mut round = [0u8; 0xc8];
        exe::comp_util_format_fixture_round_label(f, 3, round.as_mut_ptr() as usize, 0xc8);
        format(JOB_TEXT, 0x987ff0, round.as_ptr());
        if past_meetings_comp_linked(f) {
            tel26_area_button([area, 5, row, 2, 0, 0xc, 2, job_colour(0xaebdd4), JOB_TEXT as u32, 0x7d0, read_u32(f + screen_fixture::COMP)]);
        } else {
            tel26_area_text([area, 5, row, 2, 0, 0xc, 2, job_colour(0xaebdd4), JOB_TEXT as u32]);
        }
        write_u8(JOB_TEXT, 0);
        tel26_area_text([area, 6, row, 0x30, job_colour(0xae3192), 0xc, 1, pale, JOB_TEXT as u32]);
        row += 1;
    }
    while row < rows {
        for col in 0..7u32 {
            tel26_area_text([area, col, row, 2, 0, 1, 2, pale, 0]);
        }
        row += 1;
    }
    club_pages_restore_scroll(area, 4, 5);
    tel26_back_next();
}

/// Answers a click on the past meetings screen. A real widget first saves the list's scroll row (param 4 from the area
/// of param 5). Id 1 (a View or Filter choice): its value goes into the filter (param 2): 1, 2, 4, 8 and 0x10 replace
/// the period bits (0x1f), 0x20, 0x40, 0x80 and 0x100 the venue bits (0x1e0), 0x200 and 0x400 the friendlies bits
/// (0x600); any value makes the cache stale (its filter 0) and the screen is rebuilt (−11). Id 2 (a meeting's date):
/// the competition's fixtures on that date (mode 1, −4). Anything else goes to the menu bar.
/// Original: `past_meetings_screen_input` (0x46ffd0, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn past_meetings_screen_input(widget: u32) -> i32 {
    let filter = get_screen_param(2) as u32;
    let cache = get_screen_param(3);
    club_pages_save_scroll(widget, 4, 5);
    match club_pages_id(widget) {
        1 => {
            let value = tel26_item_value(widget as i16);
            let filter = match value {
                1 | 2 | 4 | 8 | 0x10 => Some(filter & !0x1f | value),
                0x20 | 0x40 | 0x80 | 0x100 => Some(filter & !0x1e0 | value), // tel04_1 the venue bits
                0x200 | 0x400 => Some(filter & !0x600 | value),
                _ => None,
            };
            if let Some(f) = filter {
                set_screen_param(2, f as usize);
            }
            write_u32(cache + past_meetings_cache::FILTER, 0);
            -11
        }
        2 => {
            let f = tel26_item_value(widget as i16) as usize;
            club_pages_open_comp_fixtures(read_usize(f + screen_fixture::COMP), read_u16(f + screen_fixture::DAY), read_u16(f + screen_fixture::YEAR), 1);
            -4
        }
        _ => club_pages_menu_bar(widget),
    }
}

/// The tournament settings (0xd1 bytes) of a new CM Cup: the name "CM Cup", kind 2 (league) at +0xc8, 3 points for
/// a win (+0xc9), goal difference (+0xca), no extra time (+0xcb), no decider (+0xcd) and 4 teams (+0xcf).
unsafe fn friendly_new_cup_settings(s: usize) {
    club_pages_lang()(s, 0x990cb4); // "CM Cup"
    write_u8(s + 0xc8, 2);
    write_u8(s + 0xc9, 3);
    write_u8(s + 0xca, 1);
    write_u16(s + 0xcb, 0);
    write_u16(s + 0xcd, 0);
    write_u16(s + 0xcf, 4);
}

/// Opens the arrange friendly dates screen for `club` (builder `arrange_friendly_dates_screen_build`, handler
/// `arrange_friendly_dates_screen_input`, leave `arrange_friendly_dates_screen_leave`): kind 3 a home friendly, 4 away,
/// 0xb a tour (it needs the tour record, and the screen is not a new history entry), 10 a CM Cup tournament (new
/// settings, `friendly_new_cup_settings`, in params 9 and 10). No club, or a tour without its record: the
/// club_screens.cpp 12257 box. Params: 0 the club, 1 the page 1, 2 and 3 the chosen dates and their count (none yet),
/// 4 the kind, 5 the rows per page and 6 the fixture list (0), 7 the page count 20, 8 the tour record.
/// Original: `open_arrange_friendly_screen` (0x4701f0, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn open_arrange_friendly_screen(club: usize, kind: u32, tour: usize) {
    let kind = kind as u8;
    if club == 0 || (kind == 0xb && tour == 0) {
        club_pages_error_box(0x2fe1);
        return;
    }
    let mut settings = 0usize;
    if kind == 0xb {
        if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x470400usize, 0x470fe0usize, 0, 0x4718f0usize, 0) == 0 {
            return;
        }
    } else {
        if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x470400usize, 0x470fe0usize, 1, 0x4718f0usize, 0) == 0 {
            return;
        }
        if kind == 10 {
            covered!("tel04_1 open the CM Cup dates");
            settings = exe::crt_operator_new(0xd1);
            friendly_new_cup_settings(settings);
        }
    }
    covered!("tel04_1 open the friendly dates");
    for (k, v) in [(0u32, club), (1, 1), (2, 0), (3, 0), (4, kind as i8 as isize as usize), (5, 0), (6, 0), (7, 0x14), (8, tour), (9, settings), (10, settings)] {
        set_screen_param(k, v);
    }
}

/// One row of the friendly dates calendar: the date, then by the date's status for the club
/// (`Friendly::get_club_date_status`): 5 a friendly match (the opponent "?" linked, his national team when he is a club
/// of another nation, H/A, "Friendly"), 6 a tour request (Tba, the nation's national team, A, "Tour Of <nation>"), 3 a
/// tournament (Tba, H/A for its host, its name linked to its competition), 2 unavailable, 4 chosen (click to drop, id
/// 7), 1 available (click to choose, id 8); other statuses leave the row empty.
unsafe fn friendly_dates_status_row(area: u32, row: u32, date: *mut u32, club: usize, chosen: usize, count: u8, kind: u8, settings: usize) {
    let lang = club_pages_lang();
    let format = club_pages_format();
    let (pale, yellow, dark_blue) = (job_colour(G_GUI_COLOUR_PALE_GREY), job_colour(G_GUI_COLOUR_YELLOW), job_colour(G_GUI_COLOUR_DARK_BLUE));
    let code = match kind {
        10 => read_u8(settings + 0xc8) as u32,
        3 | 4 => 0,
        _ => kind as u32,
    };
    let (mut tournament, mut fixture, mut request) = (0usize, 0usize, 0usize);
    type Status = unsafe extern "thiscall" fn(usize, usize, *mut u32, *mut usize, *mut usize, *mut usize, usize, i32, u32, u32) -> u8;
    let status: Status = core::mem::transmute(0x5c75a0usize);
    let s = status(read_usize(G_FRIENDLY), club, date, &mut tournament, &mut fixture, &mut request, chosen, count as i8 as i32, code, 1);
    if !matches!(s, 1..=6) {
        return;
    }
    let weekday: unsafe extern "thiscall" fn(*mut u32, usize, u32, u32) -> i32 = core::mem::transmute(0x54b550usize);
    weekday(date, JOB_TEXT, 0x7d0, 0xff);
    tel26_area_text([area, 0, row, 0x30, dark_blue, 0xc, 1, pale, JOB_TEXT as u32]);
    match s {
        5 => {
            let mut opponent = read_usize(fixture + 0x10);
            if club == opponent {
                opponent = read_usize(fixture + 0x14);
            }
            format(JOB_TEXT, 0x98f898, opponent + club::SHORT_NAME); // "  %s ?"
            let colour = if exe::human_manager_is_human_club(opponent, 0, 1, 0, 0) != 0 { yellow } else { pale };
            tel26_area_button([area, 1, row, 1, 0, 1, 3, colour, JOB_TEXT as u32, 0x7d5, opponent as u32]);
            let nation = read_usize(opponent + club::NATION);
            if exe::database_is_national_team(opponent) == 0 && nation != read_usize(club + club::NATION) {
                let team = exe::database_get_national_team(nation);
                tel26_area_button([area, 2, row, 1, 0, 1, 2, yellow, (nation + 0x53) as u32, 0x7d5, team as u32]);
            }
            lang(JOB_TEXT, if club == read_usize(fixture + 0x10) { 0x98f880usize } else { 0x98f868 }, 0xad9c64usize); // H / A
            tel26_area_text([area, 3, row, 1, 0, 0xc, 2, job_colour(0xaebde8), JOB_TEXT as u32]);
            lang(JOB_TEXT, 0x98f85c); // "  Friendly"
            tel26_area_text([area, 4, row, 1, 0, 1, 2, job_colour(SCREEN_COLOUR_BDDE), JOB_TEXT as u32]);
        }
        6 => {
            lang(JOB_TEXT, 0x98f850); // "  Tba"
            tel26_area_text([area, 1, row, 1, 0, 1, 3, pale, JOB_TEXT as u32]);
            let nation = read_usize(request + 4);
            let team = exe::database_get_national_team(nation);
            tel26_area_button([area, 2, row, 1, 0, 1, 2, yellow, (nation + 0x53) as u32, 0x7d5, team as u32]);
            lang(JOB_TEXT, 0x98f868, 0xad9c64usize); // "A"
            tel26_area_text([area, 3, row, 1, 0, 0xc, 2, job_colour(0xaebde8), JOB_TEXT as u32]);
            let c = read_i8(nation + nation::SHORT_GENDER_NAME) as i32;
            lang(JOB_TEXT, 0x98f834, c, c, nation + nation::SHORT_NAME); // "  Tour Of {}<%s - nation>{}"
            tel26_area_text([area, 4, row, 1, 0, 1, 2, job_colour(SCREEN_COLOUR_BDDE), JOB_TEXT as u32]);
        }
        3 => {
            lang(JOB_TEXT, 0x98f850); // "  Tba"
            tel26_area_text([area, 1, row, 1, 0, 1, 3, pale, JOB_TEXT as u32]);
            lang(JOB_TEXT, if club == read_usize(tournament) { 0x98f880usize } else { 0x98f868 }, 0xad9c64usize);
            tel26_area_text([area, 3, row, 1, 0, 0xc, 2, job_colour(0xaebde8), JOB_TEXT as u32]);
            format(JOB_TEXT, 0x987ff0, tournament + 0x31); // "  %s"
            let comp = read_usize(tournament + 0x65);
            if comp != 0 {
                tel26_area_button([area, 4, row, 1, 0, 1, 2, job_colour(SCREEN_COLOUR_BDDE), JOB_TEXT as u32, 0x7d0, read_u32(comp + 4)]);
            } else {
                tel26_area_text([area, 4, row, 1, 0, 1, 2, job_colour(SCREEN_COLOUR_BDDE), JOB_TEXT as u32]);
            }
        }
        2 => {
            lang(JOB_TEXT, 0x990ce0); // "  Date Unavailable"
            tel26_area_text([area, 1, row, 1, 0, 1, 3, job_colour(G_GUI_COLOUR_GREY), JOB_TEXT as u32]);
        }
        4 => {
            lang(JOB_TEXT, 0x990cd0); // "  Date Selected"
            tel26_area_button([area, 1, row, 1, 0, 1, 3, yellow, JOB_TEXT as u32, 7, row]);
        }
        _ => {
            lang(JOB_TEXT, 0x990cbc); // "  Date Available"
            tel26_area_button([area, 1, row, 1, 0, 1, 3, pale, JOB_TEXT as u32, 8, row]); // tel04_1 the available date
        }
    }
    tel26_copy_text(JOB_TEXT as *mut u8, FRIENDLY_DATES_ROW_END as *const u8);
    tel26_area_text([area, 5, row, 0x30, job_colour(0xae3192), 0xc, 1, pale, JOB_TEXT as u32]);
}

/// Builds the arrange friendly dates screen: a calendar of the days from today, a page at a time.
///
/// The chosen dates (param 2, a vector of 10 Dates) and the fixture list (param 6) are made on the first build. The
/// club's background picture; the title (kind 3 "Arrange Friendly At Home", 4 "Away", 0xb "Arrange Tour Of <nation>",
/// else "Arrange Friendly League" or "Cup" by the tournament's kind) in the club's kit colours, "Select Date" (3, 4)
/// or "Select Dates" as the subtitle, the page buttons (page param 1 of param 7) and the menu bar; a tournament has a
/// Configure button (id 5). As many rows as fit in font 3 (param 5) from (page − 1) × rows days after today: a day
/// with a fixture of the club shows it (`add_club_fixture_row`, mode 6), the others their status
/// (`friendly_dates_status_row`). At the bottom Cancel (id 9; Back, −2, for a tour) and Next (id 10, only with a
/// chosen date). With param 10 set, a screen-wide area holds the Configure screen that opens on the first click.
/// Original: `arrange_friendly_dates_screen_build` (0x470400, club_screens.cpp, cdecl, SEH).
pub(crate) unsafe extern "C" fn arrange_friendly_dates_screen_build() {
    let lang = club_pages_lang();
    let mut date = [0u32; 2];
    let make_date: unsafe extern "thiscall" fn(*mut u32) = core::mem::transmute(0x54a110usize);
    make_date(date.as_mut_ptr());
    let row_weights: [u8; 6] = [5, 10, 2, 1, 0xc, 3];
    let button_weights: [u8; 2] = [3, 1];
    let club = get_screen_param(0);
    let page = get_screen_param(1) as u8;
    let mut chosen = get_screen_param(2);
    let count = get_screen_param(3) as u8;
    let kind = get_screen_param(4) as u8;
    let mut list = get_screen_param(6);
    let pages = get_screen_param(7) as u8;
    let tour = get_screen_param(8);
    let settings = get_screen_param(9);
    if chosen == 0 {
        covered!("tel04_1 friendly dates vector");
        let room = exe::crt_operator_new(0x54);
        chosen = if room != 0 {
            write_u32(room, 10);
            exe::crt_vector_constructor_iterator(room + 4, 8, 10, 0x54a110usize, 0x5a53c0usize);
            room + 4
        } else {
            0
        };
        set_screen_param(2, chosen);
    }
    if list == 0 {
        let room = exe::crt_operator_new(0xe);
        list = if room != 0 {
            let ctor: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
            ctor(room)
        } else {
            0
        };
        set_screen_param(6, list);
    }
    covered!("tel04_1 friendly dates build");
    exe::background_set_club_picture(club, 0, 0);
    match kind {
        3 => {
            lang(JOB_TEXT, 0x990d88); // "Arrange Friendly At Home"
        }
        4 => {
            lang(JOB_TEXT, 0x990d70); // "Arrange Friendly Away"
        }
        0xb => {
            let c = read_i8(tour + nation::SHORT_GENDER_NAME) as i32;
            lang(JOB_TEXT, 0x990d4c, c, c, tour + nation::SHORT_NAME); // "Arrange Tour Of {}<%s - nation>{}"
        }
        _ => {
            lang(JOB_TEXT, if read_u8(settings + 0xc8) == 2 { 0x990d34usize } else { 0x990d1c }); // League / Cup
        }
    }
    club_pages_title(club, JOB_TEXT);
    lang(JOB_TEXT, if kind == 3 || kind == 4 { 0x990d00usize } else { 0x990d0c }); // "Select Date" / "Select Dates"
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, JOB_TEXT, 1);
    let page_buttons: unsafe extern "C" fn(u32, u32, u32, u32, u32, u32, u32, u32) -> i32 = core::mem::transmute(0x5ede20usize);
    page_buttons(0x311, 0x55, page as i8 as i16 as u16 as u32, pages as i8 as i16 as u16 as u32, 1, 2, 3, 4);
    exe::menubar_build(4, 0);
    let (grey, pale) = (job_colour(G_GUI_COLOUR_GREY), job_colour(G_GUI_COLOUR_PALE_GREY));
    let mut top = 0x91;
    if kind == 10 {
        lang(JOB_TEXT, 0x990cf4); // "Configure"
        tel26_add_button([0x28f, 0x91, 0x30c, 0xa5, 0x30, grey, 0xc, 1, pale, JOB_TEXT as u32, 5, 0]);
        top = 0xaa;
    }
    let rows = ((0x207 - top) / exe::font_get_height(3)) as u8;
    set_screen_param(5, rows as usize);
    let area = club_pages_area([0x6e, top as u32, 0x30c, 0x217, 6, row_weights.as_ptr() as u32, rows as u32, 0, 2, 0, u32::MAX]);
    exe::llist_clear(list);
    let first = (page as i8 as i16).wrapping_sub(1).wrapping_mul(rows as i16);
    for row in 0..rows as u32 {
        let mut later = [0u32; 2];
        let d = club_pages_add_days(G_CURRENT_DATE, later.as_mut_ptr(), first.wrapping_add(row as i16));
        club_pages_copy_date(date.as_mut_ptr() as usize, d);
        let before = read_i16(list + llist::COUNT);
        let (lo, hi) = (date[0], read_u32(date.as_ptr() as usize + 2));
        exe::fixture_manager_find_fixtures(read_usize(G_FIXTURE_MANAGER), lo, hi, lo, hi, read_u32(club), list, u32::MAX, 0xffff_fffe, u32::MAX);
        if before < read_i16(list + llist::COUNT) {
            write_u32(list + llist::CURSOR, read_u32(list + llist::TAIL));
            let current: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d2e0usize);
            let fixture = current(list);
            let fixture_row: unsafe extern "C" fn(usize, usize, u32, u32, u32) = core::mem::transmute(0x483a90usize);
            fixture_row(club, fixture, area, row, 6);
        } else {
            friendly_dates_status_row(area, row, date.as_mut_ptr(), club, chosen, count, kind, settings);
        }
    }
    let buttons = club_pages_area([100, 0x22b, 0x316, 0x24e, 2, button_weights.as_ptr() as u32, 1, 0, 1, 0, u32::MAX]);
    if kind == 0xb {
        lang(JOB_TEXT, 0x9880a8); // "Back"
        tel26_area_button([buttons, 0, 0, 0x30, grey, 0xc, 3, pale, JOB_TEXT as u32, 0xffff_fffe, 0]);
    } else {
        lang(JOB_TEXT, 0x98a1a0); // "Cancel"
        tel26_area_button([buttons, 0, 0, 0x30, grey, 0xc, 3, pale, JOB_TEXT as u32, 9, 0]);
    }
    lang(JOB_TEXT, 0x9880a0); // "Next"
    if count == 0 {
        tel26_area_text([buttons, 1, 0, 0x30, grey, 0x2c, 3, pale, JOB_TEXT as u32]);
    } else {
        tel26_area_button([buttons, 1, 0, 0x30, grey, 0xc, 3, pale, JOB_TEXT as u32, 10, 0]);
    }
    if get_screen_param(10) != 0 {
        let (w, h) = (exe_immediate(0x470fb3 as *const i32), exe_immediate(0x470fae as *const i32));
        club_pages_area([0, 0, w as u32, h as u32, 1, 0, 1, 0, 2, 0, u32::MAX]);
    }
}

/// The 8-byte Date `k` of the chosen dates vector.
unsafe fn friendly_dates_slot(chosen: usize, k: i32) -> usize {
    (chosen as isize + k as isize * 8) as usize
}

/// Answers a click on the arrange friendly dates screen.
///
/// With param 10 set (the first click of a new CM Cup) it is cleared and the Configure screen opens over the
/// settings (param 9, as a new tournament; −4). Ids 1 and 2 go back 10 pages and 1 (not below 1), 3 and 4 forward 1
/// and 10 (not past the page count, param 7) (−11). Id 5 opens Configure (−4); id 6 the competition's fixtures on the
/// day (mode 0, −4). Id 7 drops a date (the value is its row: today + (page − 1) × rows + row): a tour takes it out of
/// the chosen dates (the last one moves into its place), keeps them sorted and lowers the count; the others clear the
/// count. Id 8 chooses a date: home and away friendlies have one date, a tour adds it (up to 10, sorted), a tournament
/// takes as many days from it as its kind needs (`friendly_get_rest_days_for_kind`). Id 9 cancels (a friendly's open
/// invitation closed, else the tournament settings freed; −9). Id 10 goes on: a home or away friendly is added
/// (`Friendly::add_friendly_match`) and the opponent screen opens (by nation for a club, by continent for a national
/// team; −4); a tour asks the nation for dates (`add_friendly_dates_request`, −10); a tournament is added
/// (`Friendly::add_tournament`, the settings freed and command 0x11 queued with it; −10); a failure shows the
/// club_screens.cpp 12841, 12863 or 12882 box (0). Anything else goes to the menu bar.
/// Original: `arrange_friendly_dates_screen_input` (0x470fe0, club_screens.cpp, cdecl, SEH).
pub(crate) unsafe extern "C" fn arrange_friendly_dates_screen_input(widget: u32) -> i32 {
    let mut date = [0u32; 2];
    let make_date: unsafe extern "thiscall" fn(*mut u32) = core::mem::transmute(0x54a110usize);
    make_date(date.as_mut_ptr());
    let d = date.as_mut_ptr() as usize;
    let club = get_screen_param(0);
    let page = get_screen_param(1) as u8 as i8;
    let chosen = get_screen_param(2);
    let count = get_screen_param(3) as u8 as i8;
    let kind = get_screen_param(4) as u8;
    let rows = get_screen_param(5) as u8;
    let pages = get_screen_param(7) as u8 as i8;
    let tour = get_screen_param(8);
    let settings = get_screen_param(9);
    if get_screen_param(10) != 0 {
        set_screen_param(10, 0);
        open_configure_tournament_screen_by_address(settings, 1);
        return -4;
    }
    let item_date = |date: usize| {
        let row = tel26_item_value(widget as i16);
        let days = (page as i16).wrapping_sub(1).wrapping_mul(rows as i16).wrapping_add(row as i16);
        let mut later = [0u32; 2];
        let r = club_pages_add_days(G_CURRENT_DATE, later.as_mut_ptr(), days);
        club_pages_copy_date(date, r);
    };
    let qsort = |n: i32| exe::crt_qsort(chosen, n as u32, 8, 0x4877c0usize);
    match club_pages_id(widget) {
        1 => {
            set_screen_param(1, (page as i32 - 10).max(1) as usize);
            -11
        }
        2 => {
            set_screen_param(1, (page as i32 - 1).max(1) as usize);
            -11
        }
        3 => {
            set_screen_param(1, (page as i32 + 1).min(pages as i32) as usize);
            -11
        }
        4 => {
            set_screen_param(1, (page as i32 + 10).min(pages as i32) as usize);
            -11
        }
        5 => {
            open_configure_tournament_screen_by_address(settings, 0);
            -4
        }
        6 => {
            let f = tel26_item_value(widget as i16) as usize;
            club_pages_open_comp_fixtures(read_usize(f + screen_fixture::COMP), read_u16(f + screen_fixture::DAY), read_u16(f + screen_fixture::YEAR), 0);
            -4
        }
        7 => {
            item_date(d);
            if kind != 0xb {
                set_screen_param(3, 0);
                return -11;
            }
            covered!("tel04_1 drop a tour date");
            for k in 0..count.max(0) as i32 {
                let slot = friendly_dates_slot(chosen, k);
                if read_u16(d + 2) == read_u16(slot + 2) && read_u16(d) == read_u16(slot) {
                    club_pages_copy_date(slot, friendly_dates_slot(chosen, count as i32 - 1));
                }
            }
            let left = count.wrapping_sub(1);
            set_screen_param(3, left as isize as usize);
            if left != 0 {
                qsort(left as i32);
            }
            -11
        }
        8 => {
            item_date(d);
            if kind == 3 || kind == 4 {
                club_pages_copy_date(chosen, d);
                set_screen_param(3, 1);
                return -11;
            }
            if kind == 0xb {
                if count >= 10 {
                    return -11;
                }
                club_pages_copy_date(friendly_dates_slot(chosen, count as i32), d);
                let n = count + 1;
                set_screen_param(3, n as isize as usize);
                qsort(n as i32);
                return -11;
            }
            let days: unsafe extern "thiscall" fn(usize, u32) -> i8 = core::mem::transmute(0x5c74d0usize);
            let mut k: i8 = 0;
            while k < days(read_usize(G_FRIENDLY), read_u8(settings + 0xc8) as u32) {
                let mut later = [0u32; 2];
                let r = club_pages_add_days(d, later.as_mut_ptr(), k as i16);
                club_pages_copy_date(friendly_dates_slot(chosen, k as i32), r);
                k += 1;
            }
            set_screen_param(3, k as isize as usize);
            -11
        }
        9 => {
            if kind == 3 || kind == 4 {
                let open: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x5c68b0usize);
                let invitation = open(read_usize(G_FRIENDLY));
                if invitation != 0 {
                    write_u32(invitation + 8, 0);
                }
            } else {
                exe::crt_operator_delete(settings);
            }
            -9
        }
        10 => friendly_dates_next(club, chosen, count, kind, tour, settings),
        _ => club_pages_menu_bar(widget),
    }
}

/// `open_configure_tournament_screen` (0x472900) by address.
unsafe fn open_configure_tournament_screen_by_address(settings: usize, arg: u32) {
    let f: unsafe extern "C" fn(usize, u32) = core::mem::transmute(0x472900usize);
    f(settings, arg);
}

/// The Next button of the friendly dates (id 10): see `arrange_friendly_dates_screen_input`.
unsafe fn friendly_dates_next(club: usize, chosen: usize, count: i8, kind: u8, tour: usize, settings: usize) -> i32 {
    if kind == 3 || kind == 4 {
        let made = exe::friendly_add_friendly_match(read_usize(G_FRIENDLY), club, chosen, 0usize, (kind == 3) as u32);
        if made == 0 {
            club_pages_error_box(0x3229);
            return 0;
        }
        write_u32(made + 8, 1);
        let national = exe::database_is_national_team(club);
        let nation = read_usize(club + club::NATION);
        let opponents: unsafe extern "C" fn(u32, usize, usize, u32) = core::mem::transmute(0x5a0650usize);
        if national != 0 {
            opponents(1, read_usize(nation + nation::CONTINENT), nation, 2);
        } else {
            opponents(4, read_usize(nation + nation::CONTINENT), nation, 1);
        }
        return -4;
    }
    if kind == 0xb {
        let request: unsafe extern "thiscall" fn(usize, usize, usize, u32, usize) -> usize = core::mem::transmute(0x5c71f0usize);
        if request(read_usize(G_FRIENDLY), club, tour, count as u8 as u32, chosen) != 0 {
            return -10;
        }
        club_pages_error_box(0x323f);
        return 0;
    }
    let invites: [usize; 4] = [club, 0, 0, 0];
    type AddTournament = unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, u32, u32, u32, u32, *const usize, usize) -> usize;
    let add: AddTournament = core::mem::transmute(0x5c6e00usize);
    let made = add(read_usize(G_FRIENDLY), club, read_u8(settings + 0xc8) as u32, chosen, read_u8(settings + 0xc9) as u32, read_u8(settings + 0xca) as u32,
        read_u16(settings + 0xcb) as u32, read_u16(settings + 0xcd) as u32, read_u16(settings + 0xcf) as u32, invites.as_ptr(), settings);
    if made == 0 {
        club_pages_error_box(0x3252);
        return 0;
    }
    exe::crt_operator_delete(settings);
    write_u32(G_PENDING_COMMAND_CODE, 0x11);
    write_usize(G_PENDING_COMMAND_ARG, made);
    -10
}

/// Leaves the arrange friendly dates screen: the chosen dates vector (param 2, the count before it) is destroyed and
/// freed and params 2 and 3 cleared; the fixture list (param 6) is destroyed and freed and param 6 cleared.
/// Original: `arrange_friendly_dates_screen_leave` (0x4718f0, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn arrange_friendly_dates_screen_leave() {
    if get_screen_param(2) != 0 {
        let chosen = get_screen_param(2);
        if chosen != 0 {
            exe::crt_vector_destructor_iterator(chosen, 8, read_u32(chosen - 4), 0x5a53c0usize);
            exe::crt_operator_delete(chosen - 4);
        }
        set_screen_param(2, 0);
        set_screen_param(3, 0);
    }
    if get_screen_param(6) != 0 {
        let list = get_screen_param(6);
        if list != 0 {
            let dtor: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
            dtor(list);
            exe::crt_operator_delete(list);
        }
        set_screen_param(6, 0);
    }
}

/// The regions a club screen build is compared over (as `compare_match_screen_build`): the screen manager, the
/// Display, the screen params, the text buffer, 0xae290c, `extra` and the generator.
pub(crate) unsafe fn club_pages_build_regions(extra: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let mut reg = screen_build_regions();
    reg.push((G_MATCH_SCREEN_PANEL_REFRESHED_FLAG, 12));
    let slot = read_i16(SCREEN_MANAGER + screen_manager::CURRENT_SLOT) as isize;
    let block = read_usize((SCREEN_MANAGER as isize + screen_manager::PARAM_BLOCK as isize + slot * 0x300) as usize);
    if block != 0 {
        reg.push((block, 0xb4));
        reg.push((block + 0xb8, 0x200 - 0xb8));
    }
    reg.push((JOB_TEXT, 0x100));
    reg.push((0xae290c, 4));
    reg.extend_from_slice(extra);
    reg.extend_from_slice(&crate::rng::STATE_REGIONS);
    reg
}

/// A club screen build side by side, but the copy runs only when `go_on` (asked after the original) says so: the
/// original's result stays otherwise.
pub(crate) unsafe fn club_pages_compare_build_when(name: &str, extra: &[(usize, usize)], original: impl FnOnce(), rust: impl FnOnce(), go_on: impl FnOnce() -> bool) {
    let reg = club_pages_build_regions(extra);
    let before = crate::comparison::MemorySnapshot::read(&reg);
    original();
    if !go_on() {
        // simplification: this case does not run twice; proven in Rust mode
        return;
    }
    let after_original = crate::comparison::MemorySnapshot::read(&reg);
    before.write_back();
    crate::rng::pause_trace(|| crate::rng::without_counting(rust));
    let after_rust = crate::comparison::MemorySnapshot::read(&reg);
    after_original.write_back();
    let same = after_original == after_rust;
    SCOREBOARD.record(same, || format!("{}: differences {:?}", name, first_differences(&after_original, &after_rust)));
}

/// A club screen input side by side over the screen params plus `extra` (the answer too), or only the original.
pub(crate) unsafe fn club_pages_compare_input(name: &str, widget: u32, o: unsafe extern "C" fn(u32) -> i32, r: unsafe extern "C" fn(u32) -> i32, both: bool, extra: &[(usize, usize)]) -> i32 {
    if widget as u16 == 0xffff || !both {
        // simplification: opening another screen, the menu bar and the game's records do not run twice
        return o(widget);
    }
    let mut reg = screen_param_regions();
    reg.extend_from_slice(extra);
    let (ro, rr, same, mo, mr) = crate::comparison::side_by_side(&reg, || o(widget), || r(widget));
    SCOREBOARD.record(same && ro == rr, || format!("{} (widget {:#x}): original={} rust={} differences {:?}", name, widget, ro, rr, first_differences(&mo, &mr)));
    ro
}

/// The params 0..`n` of the current screen.
pub(crate) unsafe fn club_pages_params(n: u32) -> Vec<usize> {
    (0..n).map(|k| get_screen_param(k)).collect()
}

/// Comparer for `open_past_meetings_screen`: opening does not run twice; only the original runs and the new screen's
/// params are checked against the plan.
pub(crate) unsafe extern "C" fn compare_open_past_meetings_screen(club: usize, opposition: usize, reserves: u32) {
    original_fn::<unsafe extern "C" fn(usize, usize, u32)>(%%I0%%)(club, opposition, reserves);
    if club != 0 && opposition != 0 && get_screen_param(0) == club && get_screen_param(1) == opposition {
        let plan = vec![club, opposition, 0x221, past_meetings_cache_of_slot(), 0, 0, reserves as usize];
        let got = club_pages_params(7);
        SCOREBOARD.record(got == plan, || format!("past meetings screen (open): params {:x?} x plan {:x?}", got, plan));
    }
}

/// Comparer for `past_meetings_screen_build`: side by side by the display list plus the cache and its table; when the
/// cache is stale (the search makes a new list and table) only the original runs.
pub(crate) unsafe extern "C" fn compare_past_meetings_screen_build() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I1%%);
    let (club, opposition, cache) = (get_screen_param(0), get_screen_param(1), get_screen_param(3));
    let today = read_u32(G_CURRENT_DATE);
    if club == 0 || opposition == 0 || cache == 0 || read_u16(cache + 2) != (today >> 16) as u16 || read_u16(cache) != today as u16
        || read_usize(cache + past_meetings_cache::CLUB) != club || read_usize(cache + past_meetings_cache::OPPOSITION) != opposition
        || read_usize(cache + past_meetings_cache::RESERVES) != get_screen_param(6) || read_u32(cache + past_meetings_cache::FILTER) != get_screen_param(2) as u32 {
        // simplification: the search allocates the list and the table; proven in Rust mode
        return o();
    }
    let mut extra = vec![(cache, past_meetings_cache::SIZE)];
    let table = read_usize(cache + past_meetings_cache::TABLE);
    if table != 0 {
        extra.push((table, 0xee));
    }
    compare_match_screen_build("past meetings screen", &extra, || o(), || past_meetings_screen_build());
}

/// Comparer for `past_meetings_screen_input`: the View and Filter choices (id 1) side by side over the params and the
/// cache's filter; the rest only the original.
pub(crate) unsafe extern "C" fn compare_past_meetings_screen_input(widget: u32) -> i32 {
    let cache = get_screen_param(3);
    let both = cache != 0 && club_pages_id(widget) == 1;
    club_pages_compare_input("past meetings screen input", widget, original_fn(%%I2%%), past_meetings_screen_input, both, &[(cache + past_meetings_cache::FILTER, 4)])
}

/// Comparer for `open_arrange_friendly_screen`: only the original runs (it opens a screen and may allocate the CM Cup
/// settings); the params and the new settings are checked against the plan.
pub(crate) unsafe extern "C" fn compare_open_arrange_friendly_screen(club: usize, kind: u32, tour: usize) {
    original_fn::<unsafe extern "C" fn(usize, u32, usize)>(%%I3%%)(club, kind, tour);
    let k = kind as u8;
    if club == 0 || (k == 0xb && tour == 0) || get_screen_param(0) != club || get_screen_param(8) != tour {
        return;
    }
    let settings = get_screen_param(9);
    let mut plan = vec![club, 1, 0, 0, k as i8 as isize as usize, 0, 0, 0x14, tour, settings, settings];
    let mut ok = true;
    if k == 10 {
        let mut expected = [0u8; 0xd1];
        friendly_new_cup_settings(expected.as_mut_ptr() as usize);
        ok = settings != 0 && core::slice::from_raw_parts(settings as *const u8, 0xd1)[..7] == expected[..7]
            && core::slice::from_raw_parts((settings + 0xc8) as *const u8, 9) == &expected[0xc8..];
    } else {
        plan[9] = 0;
        plan[10] = 0;
    }
    let got = club_pages_params(11);
    SCOREBOARD.record(ok && got == plan, || format!("arrange friendly dates screen (open kind {}): params {:x?} x plan {:x?} settings ok {}", k, got, plan, ok));
}

/// Comparer for `arrange_friendly_dates_screen_build`: side by side by the display list plus the chosen dates, when
/// the vector and the list exist and no day of the page has a fixture of the club (the fixture rows carry the
/// pointers of the list's fresh copies); otherwise only the original.
pub(crate) unsafe extern "C" fn compare_arrange_friendly_dates_screen_build() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I4%%);
    let (chosen, list) = (get_screen_param(2), get_screen_param(6));
    if chosen == 0 || list == 0 {
        // simplification: the first build allocates the vector and the list; proven in Rust mode
        return o();
    }
    club_pages_compare_build_when("arrange friendly dates screen", &[(chosen, 0x50)], || o(), || arrange_friendly_dates_screen_build(),
        || read_i16(list + llist::COUNT) == 0);
}

/// Comparer for `arrange_friendly_dates_screen_input`: the page buttons and choosing or dropping a date (ids 1..4, 7,
/// 8) side by side over the params and the chosen dates; the rest (other screens, the friendlies' records, freeing)
/// only the original.
pub(crate) unsafe extern "C" fn compare_arrange_friendly_dates_screen_input(widget: u32) -> i32 {
    let chosen = get_screen_param(2);
    let id = club_pages_id(widget);
    let both = chosen != 0 && get_screen_param(10) == 0 && matches!(id, 1..=4 | 7 | 8);
    club_pages_compare_input("arrange friendly dates screen input", widget, original_fn(%%I5%%), arrange_friendly_dates_screen_input, both, &[(chosen, 0x50)])
}

/// Comparer for `arrange_friendly_dates_screen_leave`: freeing does not run twice; only the original runs and the
/// cleared params are checked afterwards.
pub(crate) unsafe extern "C" fn compare_arrange_friendly_dates_screen_leave() {
    original_fn::<unsafe extern "C" fn()>(%%I6%%)();
    let left = [get_screen_param(2), get_screen_param(3), get_screen_param(6)];
    SCOREBOARD.record(left == [0, 0, 0], || format!("arrange friendly dates screen leave: params 2, 3, 6 left {:x?}", left));
}
