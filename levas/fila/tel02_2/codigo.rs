// ---------- batch %%LEVA%%: the club screen's info tab: General Info, Stats, Finances, Income and Expenditure (club_screens.cpp) ----------

/// The shared text buffer the screen builders format into.
const INFO_TAB_TEXT: usize = 0xde1f64;
/// The empty argument the "<%s - COMMENT - ...>" texts take.
const INFO_TAB_NO_COMMENT: usize = 0xad9c64;
/// The menu background colour word (unnamed in globals.rs; match_screens uses the literal).
const INFO_TAB_COLOUR_MENU: usize = 0xae31aa;
/// g_gui_colour_orange (the "Non-Playing Staff" heading).
const INFO_TAB_COLOUR_ORANGE: usize = 0xaebdd4;
/// The function-static green: bit 0 of the guard byte, then the pixel word (`graphics_rgb_to_pixel(0, 0xc0, 0, 0)`).
const INFO_TAB_STATIC_GUARD: usize = 0xadab32;
const INFO_TAB_STATIC_GREEN: usize = 0xadab48;
/// A byte the General Info view sets to 1 before sorting the non-playing staff [inference: read by the sort
/// comparers 0x487660/0x4875a0].
const INFO_TAB_STAFF_SORTED_FLAG: usize = 0xadab33;
/// The Welsh clubs of the English pyramid (g_key_club_swansea_city, g_key_club_cardiff_city, g_key_club_wrexham): their
/// nation reads "Wales".
const INFO_TAB_KEY_SWANSEA_CITY: usize = 0x9d0430;
const INFO_TAB_KEY_CARDIFF_CITY: usize = 0x9d0434;
const INFO_TAB_KEY_WREXHAM: usize = 0x9d043c;
/// club_collect_non_playing_staff (0x544c70, cdecl(club, out, max) -> count in AL).
const INFO_TAB_FN_COLLECT_NON_PLAYING_STAFF: usize = 0x544c70;
/// The qsort comparers of the non-playing staff: national team and club.
const INFO_TAB_FN_SORT_NATIONAL_STAFF: usize = 0x487660;
const INFO_TAB_FN_SORT_CLUB_STAFF: usize = 0x4875a0;
/// The job code whose (non-regen) holder is listed as plain text, not a link [inference: the chairman].
const INFO_TAB_JOB_PLAIN: u8 = 1;

/// What the info tab's frame holds when it hands over to the Stats view (0x4619d3) or to the money views (0x462a80).
/// Both entries see the same frame (esp = the body's esp, E): the fields say which register or slot they mirror.
pub(crate) struct InfoTabFrame {
    /// [E+0x1c] and edi: the club (screen param 0).
    pub(crate) club: usize,
    /// ebp: the club's finance record (`FinanceTable::get_club_finance`).
    pub(crate) finance: usize,
    /// [E+0x14]: the view flags (screen param 0x1b, forced to 1 = General Info as described in the header).
    pub(crate) view: u32,
    /// [E+0x30]: screen param 1.
    pub(crate) param1: usize,
    /// esi and [E+0x2dc] (the `top` argument's slot, rewritten): the y below the title, `top + 0x41`.
    pub(crate) y: i32,
    /// [E+0x2e0]: the `bottom` argument.
    pub(crate) bottom: i32,
    /// ebx: the view menu area (`Display::add_area` attached to the "View" button).
    pub(crate) menu_area: usize,
    /// [E+0x58..E+0x60]: the column widths 2, 3, 2, 5, 2, 1, 1, 1, 1 (General Info uses +2 and +0).
    pub(crate) columns: [u8; 9],
    /// [E+0x24]: view & 1 (0 at both hand-overs); [E+0x38]: view & 0x10.
    pub(crate) general: u32,
    pub(crate) stats: u32,
    /// [E+0x10]: the Cash object (int64 amount, low dword zeroed at entry) the money views use; the frame's second
    /// Cash at [E+0x2c] is not read by any view here. Their destructor (0x5a53c0) is the empty function, so leaving
    /// needs nothing.
    pub(crate) cash_a: [u32; 2],
    /// [E+0xac]: the title (the selected view's menu text); [E+0x1c8]: the badge path / staff name buffer;
    /// [E+0x174]: the 0x15 non-playing staff pointers.
    pub(crate) title: [u8; 0x100],
    pub(crate) path: [u8; 0x104],
    pub(crate) staff: [usize; 0x15],
}

/// Draws the club screen's info tab between `top` and `bottom`: the "View" menu, the title and one of five views.
///
/// The function-static green (`INFO_TAB_STATIC_GREEN`, `exe::graphics_rgb_to_pixel(0, 0xc0, 0, 0)`) is made on the
/// first call (bit 0 of `INFO_TAB_STATIC_GUARD`). The screen params: 0x1b the view flags (1 General Info, 0x10 Stats,
/// 2 Finances, 4 Income, 8 Expenditure), 0 the club, 1 a value the views read. Unless the view has bit 1 or 0x10, a
/// national team (`exe::database_is_national_team`) or a club the current human does not control
/// (`exe::human_manager_current_controls_club(club, 0, 1)`) falls back to General Info. The club's finance record comes
/// from `exe::financetable_get_club_finance`.
///
/// The "View" button (x 0x6e..0xeb, y `top`..`top + 0x14`, grey on pale grey) opens a menu (`Display::add_area` with
/// 0x130) of the views, each an item with id 0x20 and the view's bit as value, the selected one flagged 0x2002 and its
/// text kept as the title: General Info and Stats always, Finances, Income and Expenditure only for a club the human
/// controls that is not a national team. The title goes in yellow at y `top + 0x19`..`top + 0x3c` (font 3) and the
/// view starts at `top + 0x41`. With no view bit at all the title is the original's uninitialised stack buffer (here
/// empty).
///
/// General Info: a two-column table (0x6e..0x30c, 0x8e high, 6 rows) and, when `find_club_badge_picture_path` finds the
/// badge, the picture in an area at 0x27e. Rows (labels pale grey, values yellow): Continent (national team: the
/// nation's continent name, or "Unknown") or Nation (the Welsh clubs Swansea, Cardiff and Wrexham read "Wales";
/// otherwise the nation's name as a link 0x7d5 to its national team, or "Unknown"); Status ("-" for a national or
/// all-star team, `exe::nationalteams_is_national_or_all_star_team`; otherwise Professional, Semi-Professional, Amateur
/// by `club::PROFESSIONAL_STATUS` 1/2/3, or "Unknown"); Finances ("-" likewise; otherwise by
/// `exe::finance_get_wealth_band`: 2 Rich, 1 Secure, 0 Ok, −2 Bankrupt when the finance's +0x165 flag is 1, Insecure for
/// −1 or when that flag is not 1); Stadium (a club that does not own it, `club::OWN_STADIUM` 0: the first other club of
/// the table with the same stadium that owns it is named as the ground's owner, otherwise "(Rented Stadium)"; with the
/// stadium's city when it has one; no stadium: "Various" for a national team, else "Unknown"); Facilities (the capacity,
/// the seated capacity when it is below the capacity and not 0, "all seater" when the seated capacity reaches it, with
/// retractable roof and/or under-soil heating; "-" with no stadium or capacity 0); Training Ground (not for national
/// teams: above 15 "Top facilities plus youth academy", otherwise reputation / max(training, 1) up to 400 Good, 600
/// Adequate, 800 Need some investment, 1000 Need major investment, beyond Awful).
///
/// Then the "Non-Playing Staff" list (0x93 lower, down to `bottom`, two columns, as many rows as fit in font 3 after 16
/// pixels): an orange heading 0x73 pixels in, then the non-playing staff (`club_collect_non_playing_staff`, at most 0x15)
/// sorted (national or club comparer) after setting `INFO_TAB_STAFF_SORTED_FLAG`, a person repeated right after
/// himself shown once, from row 2: the name ("  %s") as a link (0x7dc national, 0x7d7 club) unless his club job
/// (`staff::JOB_FOR_CLUB`) is 1 and he has no regen mark (`staff_state::STATUS_FLAGS` 0x40), and his job name
/// (`staff::JOB_FOR_NATION` for a national team, `staff::JOB_FOR_CLUB` otherwise). No staff: "Unknown". The list is
/// scrolled to param 0x1c and its index goes to param 0x1d.
///
/// Stats (`info_tab_stats_view`) and Finances, Income and Expenditure (`info_tab_money_views`) are drawn from the same
/// frame.
/// Original: club_screen_draw_info_tab (0x460590, club_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn club_screen_draw_info_tab(top: i32, bottom: i32) {
    covered!("tel02_2 info tab");
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let area: DisplayListFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    let item: DisplayLineFn = core::mem::transmute(FN_DISPLAY_ADD_ITEM);
    let button: ManagerBoxFn = core::mem::transmute(FN_SCREENMANAGER_ADD_BUTTON_ITEM);
    let heading: ManagerTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_TEXT_ITEM);
    let sm = SCREEN_MANAGER;
    let c = colour_word;

    let guard = read_u8(INFO_TAB_STATIC_GUARD);
    if guard & 1 == 0 {
        write_u8(INFO_TAB_STATIC_GUARD, guard | 1);
        write_u16(INFO_TAB_STATIC_GREEN, exe::graphics_rgb_to_pixel(0, 0xc0, 0, 0) as u16);
    }
    let mut f = InfoTabFrame {
        club: 0,
        finance: 0,
        view: 0,
        param1: 0,
        y: top,
        bottom,
        menu_area: 0,
        columns: [2, 3, 2, 5, 2, 1, 1, 1, 1],
        general: 0,
        stats: 0,
        cash_a: [0, 0],
        title: [0; 0x100], // the original leaves it uninitialised (stack) until a view is selected
        path: [0; 0x104],
        staff: [0; 0x15],
    };
    f.view = exe::screenmanager_get_screen_param(sm, 0x1b) as u32;
    f.club = exe::screenmanager_get_screen_param(sm, 0);
    f.param1 = exe::screenmanager_get_screen_param(sm, 1);
    let club = f.club;
    if f.view & 0x11 == 0 {
        if exe::database_is_national_team(club) != 0 {
            f.view = 1;
        }
        if exe::human_manager_current_controls_club(club, 0, 1) == 0 {
            f.view = 1;
        }
    }
    f.finance = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club);

    // the "View" button and its menu
    text(INFO_TAB_TEXT, 0x988004); // "View"
    let b = button(sm, 0x6e, top as usize, 0xeb, (top + 0x14) as usize, 0x30, c(G_GUI_COLOUR_GREY), 0xc, 1, c(G_GUI_COLOUR_PALE_GREY), INFO_TAB_TEXT, 0, 0);
    f.menu_area = area(current_human_display(), 0, 0, 0, 0, 0, 0, 0, 0, 0x130, c(INFO_TAB_COLOUR_MENU), b);
    let title = f.title.as_mut_ptr() as usize;
    let menu = f.menu_area;
    // one menu entry: id 0x20, value `bit`; the selected one flagged 0x2002 and copied as the title
    let entry = |bit: u32, selected: bool| {
        let mark = if selected { 0x2002 } else { 2 };
        item(current_human_display(), mark, 0, 0, 0, 0, 0, 0, 0x10, c(INFO_TAB_COLOUR_MENU), c(G_GUI_COLOUR_YELLOW), 1, 1, c(G_GUI_COLOUR_BLACK), INFO_TAB_TEXT, 0, 0x20, bit as usize, menu);
        if selected {
            copy_c_string(title, INFO_TAB_TEXT);
        }
    };
    text(INFO_TAB_TEXT, 0x990598); // " General Info"
    f.general = f.view & 1;
    entry(1, f.general != 0);
    text(INFO_TAB_TEXT, 0x98ee70); // " Stats"
    f.stats = f.view & 0x10;
    entry(0x10, f.stats != 0);
    if exe::human_manager_current_controls_club(club, 0, 1) != 0 && exe::database_is_national_team(club) == 0 {
        text(INFO_TAB_TEXT, 0x99058c); // " Finances"
        entry(2, f.view & 2 != 0);
        text(INFO_TAB_TEXT, 0x990584); // " Income"
        entry(4, f.view & 4 != 0);
        text(INFO_TAB_TEXT, 0x990574); // " Expenditure"
        entry(8, f.view & 8 != 0);
    }
    let y = top + 0x19;
    heading(sm, 0x6e, y as usize, 0x30c, (y + 0x23) as usize, 2, 0, 0xc, 3, c(G_GUI_COLOUR_YELLOW), title, 1);
    f.y = y + 0x28;

    if f.general != 0 {
        info_tab_general_view(&mut f);
    } else if f.stats != 0 {
        // 0x4619d3: esi = f.y, [E+0x2e0] = f.bottom, edi = f.club, [E+0x30] = f.param1
        info_tab_stats_view(f.y, f.bottom, f.club, f.param1);
    } else {
        // 0x462a80: esi = f.y, [E+0x2e0] = f.bottom, [E+0x14] = f.view, ebp = f.finance, [E+0x10] = f.cash_a
        info_tab_money_views(f.y, f.bottom, f.view as u8, Finance(f.finance), &mut f.cash_a[0]);
    }
    // 0x467166: the two Cash objects' destructors are the empty 0x5a53c0
}

/// The General Info view of `club_screen_draw_info_tab` (0x460c02..0x4619ce): see its header.
unsafe fn info_tab_general_view(f: &mut InfoTabFrame) {
    covered!("tel02_2 info tab general");
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let label: ListCellTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
    let link: ListCellFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_BUTTON);
    let area: DisplayListFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    let item: DisplayLineFn = core::mem::transmute(FN_DISPLAY_ADD_ITEM);
    let collect: unsafe extern "C" fn(usize, usize, i32) -> u8 = core::mem::transmute(INFO_TAB_FN_COLLECT_NON_PLAYING_STAFF);
    let sm = SCREEN_MANAGER;
    let c = colour_word;
    let club = f.club;
    let y = f.y;
    let t = INFO_TAB_TEXT;

    let table = area(current_human_display(), 0x6e, y as usize, 0x30c, (y + 0x8e) as usize, 2, f.columns.as_mut_ptr().add(2) as usize, 6, 0, 2, 0, usize::MAX);
    if find_club_badge_picture_path(club, f.path.as_mut_ptr() as usize) != 0 {
        let pic = area(current_human_display(), 0x27e, y as usize, 0x30c, (y + 0x8e) as usize, 1, 0, 1, 0, 1, 0, usize::MAX);
        item(current_human_display(), 0x400, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0xc, 1, 0, f.path.as_ptr() as usize, 0, 0, 0, pic);
    }
    // a row's label (pale grey) and value (yellow) in the shared text
    let key = |row: usize| label(sm, table, 0, row, 1, c(G_GUI_COLOUR_BLACK), 1, 3, c(G_GUI_COLOUR_PALE_GREY), t);
    let value = |row: usize| label(sm, table, 1, row, 1, c(G_GUI_COLOUR_BLACK), 1, 2, c(G_GUI_COLOUR_YELLOW), t);
    let national = || exe::database_is_national_team(club) != 0;
    let all_star = || exe::nationalteams_is_national_or_all_star_team(read_usize(G_NATIONAL_TEAMS), club) != 0;

    // row 0: Continent or Nation
    let nation = read_usize(club + club::NATION);
    if national() {
        text(t, 0x990568); // "  Continent"
        key(0);
        let continent = read_usize(nation + nation::CONTINENT);
        if continent != 0 {
            label(sm, table, 1, 0, 1, c(G_GUI_COLOUR_BLACK), 1, 2, c(G_GUI_COLOUR_YELLOW), continent + 4);
        } else {
            text(t, 0x990544, INFO_TAB_NO_COMMENT); // "Unknown<%s - COMMENT - continent>"
            value(0);
        }
    } else {
        text(t, 0x990538); // "  Nation"
        key(0);
        let id = read_u32(club);
        if id == read_u32(INFO_TAB_KEY_SWANSEA_CITY) || id == read_u32(INFO_TAB_KEY_CARDIFF_CITY) || id == read_u32(INFO_TAB_KEY_WREXHAM) {
            text(t, 0x9904f4, INFO_TAB_NO_COMMENT); // "Wales<%s - COMMENT - Country Name>"
            value(0);
        } else if nation != 0 {
            let team = exe::database_get_national_team(nation);
            link(sm, table, 1, 0, 1, c(G_GUI_COLOUR_BLACK), 1, 2, c(G_GUI_COLOUR_YELLOW), nation + 4, 0x7d5, team);
        } else {
            text(t, 0x990518, INFO_TAB_NO_COMMENT); // "Unknown<%s - COMMENT - nation>"
            value(0);
        }
    }

    // row 1: Status
    text(t, 0x9904c0, INFO_TAB_NO_COMMENT); // "  Status<%s - COMMENT - club professional status>"
    key(1);
    if all_star() {
        copy_c_string(t, 0x98ba0c); // "-"
    } else {
        match read_u8(club + club::PROFESSIONAL_STATUS) {
            1 => { text(t, 0x9904b0); } // "Professional"
            2 => { text(t, 0x99049c); } // "Semi-Professional"
            3 => { text(t, 0x990494); } // "Amateur"
            _ => { text(t, 0x990460, INFO_TAB_NO_COMMENT); } // "Unknown<%s - COMMENT - club professional status>"
        }
    }
    value(1);

    // row 2: Finances (each band test asks the finance again, as the original)
    text(t, 0x990454); // "  Finances"
    key(2);
    let fin = f.finance;
    let band = || exe::finance_get_wealth_band(fin) as i8;
    if all_star() {
        copy_c_string(t, 0x98ba0c); // "-"
    } else if band() == 2 {
        text(t, 0x99044c); // "Rich"
    } else if band() == 1 {
        text(t, 0x990444); // "Secure"
    } else if band() == 0 {
        text(t, 0x990428, INFO_TAB_NO_COMMENT); // "Ok<%s - COMMENT - finances>"
    } else if band() != -1 && read_u8(fin + 0x165) == 1 {
        if band() == -2 {
            text(t, 0x99041c); // "Bankrupt"
        }
        // tel02_2: any other band keeps the text as it is ("  Finances")
    } else {
        text(t, 0x9903f8, INFO_TAB_NO_COMMENT); // "Insecure<%s - COMMENT - finances>"
    }
    value(2);

    // row 3: Stadium
    text(t, 0x9903ec); // "  Stadium"
    key(3);
    let stadium = read_usize(club + club::STADIUM);
    let sname = stadium + stadium::NAME;
    let sg = || read_i8(stadium + stadium::GENDER_NAME) as i32;
    let unknown_stadium = || {
        if national() {
            text(t, 0x99029c); // "Various"
        } else {
            text(t, 0x99027c, INFO_TAB_NO_COMMENT); // "Unknown<%s - COMMENT - stadium>"
        }
    };
    if read_u8(club + club::OWN_STADIUM) == 0 {
        let mut owner = 0usize;
        let clubs = read_usize(G_CLUB_TABLE);
        for k in 0..read_i32(G_CLUB_COUNT).max(0) as usize {
            let other = clubs + k * club::SIZE;
            if other != club && read_usize(other + club::STADIUM) == stadium && read_u8(other + club::OWN_STADIUM) != 0 {
                owner = other;
                break;
            }
        }
        if stadium == 0 {
            unknown_stadium();
        } else if owner != 0 {
            let city = read_usize(stadium + stadium::CITY);
            let og = read_i8(owner + club::SHORT_GENDER_NAME) as i32;
            if city != 0 {
                let cg = read_i8(city + city::GENDER_NAME) as i32;
                text(t, 0x990348, sg(), sg(), cg, cg, og, og, sname, city + 4, owner + club::SHORT_NAME); // ground owned by, with the city
            } else {
                text(t, 0x9902ec, sg(), sg(), og, og, sname, owner + club::SHORT_NAME); // "{}<%s - stadium>{} (Ground owned by {}<..>{})"
            }
        } else {
            let city = read_usize(stadium + stadium::CITY);
            if city != 0 {
                let cg = read_i8(city + city::GENDER_NAME) as i32;
                text(t, 0x9903b4, sg(), sg(), cg, cg, sname, city + 4); // "{}<%s - stadium>{} (Rented Stadium), {}<%s - city>{}"
            } else {
                text(t, 0x9902c8, sg(), sg(), sname); // "{}<%s - stadium>{} (Rented Stadium)"
            }
        }
    } else if stadium == 0 {
        unknown_stadium();
    } else {
        let city = read_usize(stadium + stadium::CITY);
        if city != 0 {
            let cg = read_i8(city + city::GENDER_NAME) as i32;
            text(t, 0x9902a4, sg(), sg(), cg, cg, sname, city + 4); // "{}<%s - stadium>{}, {}<%s - city>{}"
        } else {
            copy_c_string(t, sname);
        }
    }
    value(3);

    // row 4: Facilities
    text(t, 0x99026c); // "  Facilities"
    key(4);
    let cap = if stadium != 0 { read_i32(stadium + stadium::CAPACITY) } else { 0 };
    if cap == 0 {
        copy_c_string(t, 0x98ba0c); // "-"
    } else {
        let seats = read_i32(stadium + stadium::SEATING_CAPACITY);
        let roof = read_u8(stadium + stadium::COVERED) != 0;
        let heating = read_u8(stadium + stadium::UNDER_SOIL_HEATING) != 0;
        // (roof and heating, roof, heating, neither)
        let pick = |fs: [usize; 4]| if roof && heating { fs[0] } else if roof { fs[1] } else if heating { fs[2] } else { fs[3] };
        if seats >= cap {
            text(t, pick([0x990220, 0x9901ec, 0x9901b8, 0x990198]), cap); // "... all seater ..."
        } else if seats != 0 {
            text(t, pick([0x990138, 0x9900f0, 0x9900a4, 0x990070]), cap, seats); // "... (<%ld> seated) ..."
        } else {
            text(t, pick([0x990030, 0x990008, 0x98ffdc, 0x98ffc8]), cap); // "<%ld - capacity> ..."
        }
    }
    value(4);

    // row 5: Training Ground
    if !national() {
        text(t, 0x98ffb4); // "  Training Ground"
        key(5);
        let training = read_i8(club + club::TRAINING) as i32;
        if training > 0xf {
            text(t, 0x98ff90); // "Top facilities plus youth academy"
        } else {
            let ratio = read_i16(club + club::REPUTATION) as i32 / training.max(1);
            let fmt = if ratio <= 400 {
                0x98ff80 // "Good facilities"
            } else if ratio <= 600 {
                0x98ff6c // "Adequate facilities"
            } else if ratio <= 800 {
                0x98ff54 // "Need some investment"
            } else if ratio <= 1000 {
                0x98ff3c // "Need major investment"
            } else {
                0x98ff28 // "Awful facilities"
            };
            text(t, fmt);
        }
        value(5);
    }

    // the non-playing staff list
    let y2 = f.y + 0x93;
    let rows = ((f.bottom - y2 - 0x10) / exe::font_get_height(3)) as u8;
    f.general = (f.general & !0xff) | rows as u32; // [E+0x24]: the row count byte written over view & 1
    let list = area(current_human_display(), 0x6e, y2 as usize, 0x30c, f.bottom as usize, 2, f.columns.as_mut_ptr() as usize, f.general as usize, 0, 2, 0, usize::MAX);
    text(t, 0x98ff14); // "Non-Playing Staff"
    let w = exe::font_text_width(3usize, t as *const u8) + 0x73;
    let orange = c(INFO_TAB_COLOUR_ORANGE);
    item(current_human_display(), 1, 0x73, 0, w as usize, 0, 0, 0, 1, 0, 0, 1, 3, orange, t, 0, 0, 0, list);
    item(current_human_display(), 1, 0x73, 0, w as usize, 0, 0, 1, 0xc400, orange, 0, 0, 0, 0, 0, 0, 0, 0, list);
    let count = collect(club, f.staff.as_mut_ptr() as usize, 0x15) as i8 as i32;
    if count == 0 {
        text(t, 0x98fee8, INFO_TAB_NO_COMMENT); // "  Unknown<%s - COMMENT - non-playing staff>"
        label(sm, list, 0, 2, 1, c(G_GUI_COLOUR_BLACK), 1, 3, c(G_GUI_COLOUR_PALE_GREY), t);
    } else {
        write_u8(INFO_TAB_STAFF_SORTED_FLAG, 1);
        let sorter = if national() { INFO_TAB_FN_SORT_NATIONAL_STAFF } else { INFO_TAB_FN_SORT_CLUB_STAFF };
        exe::crt_qsort(f.staff.as_mut_ptr() as usize, count as u32, 4, sorter);
        let mut row = 1usize;
        let mut previous = 0usize;
        for k in 0..count.max(0) as usize {
            let p = f.staff[k];
            if p == previous {
                continue; // tel02_2 a person repeated right after himself is shown once
            }
            previous = p;
            row += 1;
            let name = f.path.as_mut_ptr() as usize;
            exe::database_format_staff_name(p, name, 0xc8, 1);
            format(t, 0x987ff0, name); // "  %s"
            let job = read_u8(p + staff::JOB_FOR_CLUB);
            let regen = read_u8(read_usize(G_STAFF_STATE_TABLE) + read_i32(p) as usize * staff_state::SIZE + staff_state::STATUS_FLAGS) & 0x40 != 0;
            let nat = national();
            if job == INFO_TAB_JOB_PLAIN && !regen {
                label(sm, list, 0, row, 1, c(G_GUI_COLOUR_BLACK), 1, 3, c(G_GUI_COLOUR_PALE_GREY), t);
            } else {
                link(sm, list, 0, row, 1, c(G_GUI_COLOUR_BLACK), 1, 3, c(G_GUI_COLOUR_PALE_GREY), t, if nat { 0x7dc } else { 0x7d7 }, p);
            }
            let job_byte = if nat { read_u8(p + staff::JOB_FOR_NATION) } else { read_u8(p + staff::JOB_FOR_CLUB) };
            let job_name = exe::database_get_job_name(job_byte as i8 as u32, 0, 1, core::ptr::null_mut());
            label(sm, list, 1, row, 1, c(G_GUI_COLOUR_BLACK), 1, 2, c(G_GUI_COLOUR_YELLOW), job_name);
        }
    }
    let scroll = exe::screenmanager_get_screen_param(sm, 0x1c);
    if list as i16 != -1 {
        exe::area_set_scroll_row(current_human_display() + 4 + (list as i16 as isize * 0xbf1) as usize, scroll);
    }
    exe::screenmanager_set_screen_param(sm, 0x1d, list as i16 as isize as usize, 0);
}

/// Comparer of `club_screen_draw_info_tab`: side by side over the display list (with the screen params, where the
/// staff list's index goes), plus the function-static guard, the staff-sorted flag and the static green.
pub(crate) unsafe extern "C" fn compare_club_screen_draw_info_tab(top: i32, bottom: i32) {
    let o = original_fn::<unsafe extern "C" fn(i32, i32)>(%%I0%%);
    let extra = [(INFO_TAB_STATIC_GUARD, 2), (INFO_TAB_STATIC_GREEN, 2)];
    compare_by_display_list_with("Club screen info tab", &extra, || o(top, bottom), || club_screen_draw_info_tab(top, bottom));
}

// ---------- the Stats view of the club info tab (club_screens.cpp, a piece of club_screen_draw_info_tab 0x460590,
// the stretch 0x4619df..0x462a7f; reader tel02) ----------

/// The shared text buffer the club screens format into.
const STATS_VIEW_TEXT: usize = 0xde1f64;
/// "-" (shown when there is no such player).
const STATS_VIEW_DASH: usize = 0x98ba0c;
/// The button id of a person's name (opens the profile; the person goes as the button value).
const STATS_VIEW_PERSON_BUTTON: usize = 0x7d7;
/// The display-currency options in the Config's +4 flags: wages per week, per month (neither: per year).
const STATS_VIEW_CONFIG_FLAGS: usize = 4;
const STATS_VIEW_WAGES_PER_WEEK: u32 = 0x200000;
const STATS_VIEW_WAGES_PER_MONTH: u32 = 0x400000;
/// club_screen_collect_squad_stats (0x487840, club_screens.cpp): cdecl(club, reserves, out) ret 0.
const STATS_VIEW_FN_COLLECT_SQUAD_STATS: usize = 0x487840;
/// database_club_nation_is_eec (0x53d7d0, Database.cpp): cdecl(club) ret 0, 1 when the club's nation is an EEC member.
const STATS_VIEW_FN_CLUB_NATION_IS_EEC: usize = 0x53d7d0;

/// The squad statistics block that `club_screen_collect_squad_stats` fills (0x45 bytes, packed; offsets as the view
/// reads them).
mod squad_stats {
    pub(super) const SIZE: usize = 0x45;
    pub(super) const PLAYERS: usize = 0x0; // i8
    pub(super) const INJURED: usize = 0x1; // i8
    pub(super) const AVERAGE_AGE_FIRST_TEAM: usize = 0x2; // f32
    pub(super) const AVERAGE_AGE_SQUAD: usize = 0x6; // f32
    pub(super) const WAGE_BILL_SQUAD: usize = 0xa; // i32, per week
    pub(super) const WAGE_BILL_FIRST_TEAM: usize = 0xe; // i32, per week
    pub(super) const AVERAGE_WAGE: usize = 0x12; // i32, per week
    pub(super) const OLDEST: usize = 0x1a; // Staff
    pub(super) const YOUNGEST: usize = 0x1e; // Staff
    pub(super) const HIGHEST_WAGE_PLAYER: usize = 0x22; // Staff
    pub(super) const LOWEST_WAGE_PLAYER: usize = 0x26; // Staff
    pub(super) const HIGHEST_WAGE: usize = 0x2a; // i32, per week
    pub(super) const LOWEST_WAGE: usize = 0x2e; // i32, per week
    pub(super) const TOTAL_CAPS: usize = 0x32; // i16
    pub(super) const HIGHEST_CAPPED: usize = 0x34; // Staff
    pub(super) const LOWEST_CAPPED: usize = 0x38; // Staff
    pub(super) const HIGHEST_VALUED: usize = 0x3c; // Staff
    pub(super) const INTERNATIONALS: usize = 0x40; // i8
    pub(super) const UNDER_21S: usize = 0x41; // i8
    pub(super) const FOREIGN: usize = 0x42; // i8
    pub(super) const NON_EU: usize = 0x43; // i8
}

/// The end of the shared text (where the original appends with strlen).
unsafe fn stats_view_text_end() -> usize {
    let mut p = STATS_VIEW_TEXT;
    while read_u8(p) != 0 {
        p += 1;
    }
    p
}

/// Draws the Stats view of the club info tab into a two-column grid from `top` to `bottom`.
///
/// The grid (`Display::add_area` on the current human's display: x 0x6e..0x30c, `top`..`bottom`, 2 columns, as many
/// rows as font 2 fits, `(bottom − top) / font_get_height(2)` kept as a byte) gets one label (pale grey, column 0) and
/// one value (yellow, column 1) per row; the numbers come from `club_screen_collect_squad_stats(club, reserves, ..)`.
/// Rows: 0 Number Of Players, 1 Number Of Players Injured (both "%d"), 2 Average Age - First Team and 3 Average Age -
/// Squad ("%4.2f", "-" when exactly 0.0). Unless the club is a national team (`database_is_national_team`): 4 Total
/// Wage Bill - First Team, 5 Total Wage Bill - Squad, 6 Average Wage, 7 Highest Wage and 8 Lowest Wage, each in the
/// Config's wage period (per week when +4 has 0x200000, per month = ×4 when 0x400000, otherwise per year = ×52; the
/// label says p/w, p/m or p/a) formatted as money (`Cash::get_display_amount` with option 8 per week/month and 4 per
/// year, `cash_format_money`); the highest and lowest wage rows also append " - " and the player's name
/// (`database_format_staff_name`) as a name button (id 0x7d7, the player as value), or "-" without a player; their
/// option is the one the Average Wage row used. Then Oldest Player and Youngest Player ("%d - " with the age read as a
/// signed byte, and the name button, or "-"). For a national team: Total Number Of Caps ("%d"), Highest Capped Player
/// and Lowest Capped Player ("%d - " with the caps as an unsigned byte, and the name button, or "-"), and the view ends.
/// For a club: Highest Valued Player (the value as money with option 6, " - " and the name button, or "-"), Number Of
/// Current International Players and Number Of Current Under 21 Players (the label translated when the database
/// names language `G_DATABASE_NAMES_LANGUAGE` is set, otherwise the untranslated "  No. Of Current ..." copied as is),
/// Number Of Foreign Players, and Number Of Non-EU Players only when the club's nation is an EEC member
/// (`database_club_nation_is_eec`). Labels go through `langlib_sprintf`, numbers through `sprintf`.
///
/// Frame of the original at the entry 0x4619df (E = esp there): `top` = esi (the y below the view buttons, also in
/// the top argument slot [E+0x2dc]); `bottom` = the bottom argument [E+0x2e0]; `club` = edi and [E+0x1c] (screen
/// param 0; the original passes edi to the collector and [E+0x1c] to the other calls: the same value when the view is
/// entered from the header, i.e. when the General Info view bit is not also set); `reserves` = [E+0x30] (screen param
/// 1). The row count byte overwrites the low byte of [E+0x24] (the General Info flag, `view & 1`, zero here) and the
/// grid gets that dword; the money goes through the Cash object at [E+0x2c] and the option byte through the low byte of
/// [E+0x14] (the copy keeps both in locals: nothing reads them after the view, which leaves through 0x467166).
/// Original: a piece of `club_screen_draw_info_tab` (0x460590, club_screens.cpp, cdecl(top, bottom)), 0x4619df..0x462a7f.
pub(crate) unsafe fn info_tab_stats_view(top: i32, bottom: i32, club: usize, reserves: usize) {
    covered!("tel02_2 stats view");
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let collect: unsafe extern "C" fn(usize, usize, usize) = core::mem::transmute(STATS_VIEW_FN_COLLECT_SQUAD_STATS);
    let is_eec: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(STATS_VIEW_FN_CLUB_NATION_IS_EEC);
    let add_area: DisplayListFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    let button: ListCellFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_BUTTON);
    let label: ListCellTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
    let sm = SCREEN_MANAGER;

    let h = exe::font_get_height(2);
    let rows = ((bottom - top) / h) as u8;
    let area = add_area(current_human_display(), 0x6e, top as usize, 0x30c, bottom as usize, 2, 0, rows as usize, 0, 2, 0, usize::MAX);
    let mut stats = [0u8; squad_stats::SIZE];
    let s = stats.as_mut_ptr() as usize;
    collect(club, reserves, s);

    let black = || colour_word(G_GUI_COLOUR_BLACK);
    // the label of row `row` (already in the shared text): column 0, pale grey
    let put_label = |row: usize| {
        label(sm, area, 0, row, 1, black(), 1, 2, colour_word(G_GUI_COLOUR_PALE_GREY), STATS_VIEW_TEXT);
    };
    // a value (text at `t`) in column 1, yellow
    let put_value = |row: usize, t: usize| {
        label(sm, area, 1, row, 1, black(), 1, 2, colour_word(G_GUI_COLOUR_YELLOW), t);
    };
    // the shared text in column 1 as the name button of `person`
    let put_person = |row: usize, person: usize| {
        button(sm, area, 1, row, 1, black(), 1, 2, colour_word(G_GUI_COLOUR_YELLOW), STATS_VIEW_TEXT, STATS_VIEW_PERSON_BUTTON, person);
    };
    let money = |amount: u32, option: u32| {
        let cash = amount;
        let shown = exe::cash_get_display_amount(&cash, option);
        exe::cash_format_money(STATS_VIEW_TEXT, 0x7d0, shown, option);
    };
    let name_after = |person: usize| {
        exe::database_format_staff_name(person, stats_view_text_end(), 0x7d0, 1);
    };
    // "%d" of a small count in column 1
    let count_row = |row: usize, caption: usize, n: i32| {
        text(STATS_VIEW_TEXT, caption);
        put_label(row);
        format(STATS_VIEW_TEXT, 0x98806c, n); // "%d"
        put_value(row, STATS_VIEW_TEXT);
    };
    // "%4.2f", or "-" when exactly 0.0 (fcomp + C3: NaN also gives "-")
    let age_row = |row: usize, caption: usize, age: f32| {
        text(STATS_VIEW_TEXT, caption);
        put_label(row);
        let zero = read_f32(rdata::F32_0);
        if !(age == zero || age.is_nan()) {
            format(STATS_VIEW_TEXT, 0x98fe94, age as f64); // "%4.2f"
        } else {
            format(STATS_VIEW_TEXT, STATS_VIEW_DASH);
        }
        put_value(row, STATS_VIEW_TEXT);
    };
    // a wage row in the Config's period: (amount, option); the captions are (p/w, p/m, p/a)
    let period = |captions: (usize, usize, usize), weekly: i32| -> (u32, u32) {
        let flags = read_u32(read_usize(G_CONFIG) + STATS_VIEW_CONFIG_FLAGS);
        if flags & STATS_VIEW_WAGES_PER_WEEK != 0 {
            text(STATS_VIEW_TEXT, captions.0);
            (weekly as u32, 8)
        } else if flags & STATS_VIEW_WAGES_PER_MONTH != 0 {
            text(STATS_VIEW_TEXT, captions.1);
            (weekly.wrapping_mul(4) as u32, 8)
        } else {
            text(STATS_VIEW_TEXT, captions.2);
            (weekly.wrapping_mul(52) as u32, 4)
        }
    };
    // a person row: "%d - " with `number`, the name button, or "-" without a person
    let person_row = |row: usize, caption: usize, person: usize, number: i32| {
        text(STATS_VIEW_TEXT, caption);
        put_label(row);
        if person != 0 {
            format(STATS_VIEW_TEXT, 0x98fcbc, number); // "%d - "
            name_after(person);
            put_person(row, person);
        } else {
            put_value(row, STATS_VIEW_DASH);
        }
    };

    count_row(0, 0x98fed4, read_i8(s + squad_stats::PLAYERS) as i32); // "  Number Of Players"
    count_row(1, 0x98feb8, read_i8(s + squad_stats::INJURED) as i32); // "  Number Of Players Injured"
    age_row(2, 0x98fe9c, read_f32(s + squad_stats::AVERAGE_AGE_FIRST_TEAM)); // "  Average Age - First Team"
    age_row(3, 0x98fe7c, read_f32(s + squad_stats::AVERAGE_AGE_SQUAD)); // "  Average Age - Squad"

    let national = exe::database_is_national_team(club) != 0;
    let mut row = 4usize;
    if !national {
        // "  Total Wage Bill - First Team (p/w)", "(p/m)", "(p/a)"
        let (amount, option) = period((0x98fe54, 0x98fe2c, 0x98fe04), read_i32(s + squad_stats::WAGE_BILL_FIRST_TEAM));
        put_label(4);
        money(amount, option);
        put_value(4, STATS_VIEW_TEXT);
        // "  Total Wage Bill - Squad (p/w)", "(p/m)", "(p/a)"
        let (amount, option) = period((0x98fde4, 0x98fdc4, 0x98fda4), read_i32(s + squad_stats::WAGE_BILL_SQUAD));
        put_label(5);
        money(amount, option);
        put_value(5, STATS_VIEW_TEXT);
        // "  Average Wage (p/w)", "(p/m)", "(p/a)"; its option also serves the highest and lowest wage rows
        let (amount, option) = period((0x98fd8c, 0x98fd74, 0x98fd5c), read_i32(s + squad_stats::AVERAGE_WAGE));
        put_label(6);
        money(amount, option);
        put_value(6, STATS_VIEW_TEXT);
        // "  Highest Wage (p/w)", "(p/m)", "(p/a)" and "  Lowest Wage ..."
        for (r, captions, wage, player) in [
            (7usize, (0x98fd44, 0x98fd2c, 0x98fd14), squad_stats::HIGHEST_WAGE, squad_stats::HIGHEST_WAGE_PLAYER),
            (8, (0x98fcfc, 0x98fce8, 0x98fcd4), squad_stats::LOWEST_WAGE, squad_stats::LOWEST_WAGE_PLAYER),
        ] {
            let (amount, _) = period(captions, read_i32(s + wage));
            put_label(r);
            let person = read_usize(s + player);
            if person != 0 {
                money(amount, option);
                text(stats_view_text_end(), 0x98fd10); // " - "
                name_after(person);
                put_person(r, person);
            } else {
                put_value(r, STATS_VIEW_DASH);
            }
        }
        row = 9;
    }
    let oldest = Staff(read_usize(s + squad_stats::OLDEST));
    person_row(row, 0x98fcc4, oldest.0, if oldest.0 != 0 { oldest.age() as i8 as i32 } else { 0 }); // "  Oldest Player"
    row += 1;
    let youngest = Staff(read_usize(s + squad_stats::YOUNGEST));
    person_row(row, 0x98fca8, youngest.0, if youngest.0 != 0 { youngest.age() as i8 as i32 } else { 0 }); // "  Youngest Player"
    row += 1;
    if exe::database_is_national_team(club) != 0 {
        count_row(row, 0x98fc90, read_i16(s + squad_stats::TOTAL_CAPS) as i32); // "  Total Number Of Caps"
        row += 1;
        let most = Staff(read_usize(s + squad_stats::HIGHEST_CAPPED));
        person_row(row, 0x98fc78, most.0, if most.0 != 0 { most.int_apps() as i32 } else { 0 }); // "  Highest Capped Player"
        row += 1;
        let least = Staff(read_usize(s + squad_stats::LOWEST_CAPPED));
        person_row(row, 0x98fc60, least.0, if least.0 != 0 { least.int_apps() as i32 } else { 0 }); // "  Lowest Capped Player"
        row += 1;
    }
    if exe::database_is_national_team(club) != 0 {
        return;
    }
    text(STATS_VIEW_TEXT, 0x98fc48); // "  Highest Valued Player"
    put_label(row);
    let valued = Staff(read_usize(s + squad_stats::HIGHEST_VALUED));
    if valued.0 != 0 {
        money(valued.value() as u32, 6);
        // the original's sprintf passes the value after " - " (unused by the format)
        format(stats_view_text_end(), 0x98fd10, valued.value() as u32); // " - "
        name_after(valued.0);
        put_person(row, valued.0);
    } else {
        put_value(row, STATS_VIEW_DASH);
    }
    row += 1;
    // the untranslated caption copied as is when the database names language is off
    let caption = |translated: usize, plain: usize| {
        if read_u32(G_DATABASE_NAMES_LANGUAGE) != 0 {
            text(STATS_VIEW_TEXT, translated);
        } else {
            core::ptr::copy_nonoverlapping(plain as *const u8, STATS_VIEW_TEXT as *mut u8, stats_view_c_len(plain) + 1);
        }
    };
    caption(0x98fbf4, 0x98fc20); // "  Number Of Current International Players" / "  No. Of Current International Players"
    put_label(row);
    format(STATS_VIEW_TEXT, 0x98806c, read_i8(s + squad_stats::INTERNATIONALS) as i32);
    put_value(row, STATS_VIEW_TEXT);
    row += 1;
    caption(0x98fba8, 0x98fbd0); // "  Number Of Current Under 21 Players" / "  No. Of Current Under 21 Players"
    put_label(row);
    format(STATS_VIEW_TEXT, 0x98806c, read_i8(s + squad_stats::UNDER_21S) as i32);
    put_value(row, STATS_VIEW_TEXT);
    row += 1;
    count_row(row, 0x98fb8c, read_i8(s + squad_stats::FOREIGN) as i32); // "  Number Of Foreign Players"
    row += 1;
    if is_eec(club) != 0 {
        count_row(row, 0x98fb70, read_i8(s + squad_stats::NON_EU) as i32); // "  Number Of Non-EU Players"
    }
}

/// strlen of the C string at `p`.
unsafe fn stats_view_c_len(p: usize) -> usize {
    let mut n = 0;
    while read_u8(p + n) != 0 {
        n += 1;
    }
    n
}

// ---------- the club screen's info tab: the money views (the table, Finances, the dispatch to Income/Expenditure) ----------
// Piece of `club_screen_draw_info_tab` (0x460590): 0x462a80..0x463bd8 and the shared tail 0x46715b..0x467161.

/// The shared text buffer and its size as the money cells pass it.
const MONEY_VIEW_TEXT: usize = 0xde1f64;
const MONEY_VIEW_TEXT_SIZE: u32 = 0x7d0;
/// The currency symbol text (0x43f440, a Cash method that ignores `this`, returns its length) [inference: the symbol
/// `cash_format_money` prints].
const MONEY_VIEW_CURRENCY_SYMBOL: usize = 0xad9ca8;
/// The green the info tab computes on its first call (`graphics_rgb_to_pixel(0, 0xc0, 0, 0)`, guard bit 0 of 0xadab32).
const MONEY_VIEW_GREEN: usize = 0xadab48;
/// `g_gui_colour_orange` (the column titles and the line under them).
const MONEY_VIEW_ORANGE: usize = 0xaebdd4;
/// The empty text passed to the "Balance" label's `%s`.
const MONEY_VIEW_EMPTY_TEXT: usize = 0xad9c64;
/// Cash::set from a display amount (0x43f4c0): thiscall(Cash*, i64) ret 8; the cash becomes ftol(amount / the display
/// factor at 0xad9cb8), the inverse of `Cash::get_display_amount`.
const MONEY_VIEW_FN_CASH_SET_DISPLAY: usize = 0x43f4c0;
/// The four 0x50-byte blocks of the club's `Finance` record, in column order: this month, last month (+0x20, no name
/// yet), this season, last season.
const MONEY_VIEW_BLOCKS: [usize; 4] = [finance::MONTH_PLAYERS_BOUGHT, 0x20, finance::SEASON_PLAYERS_BOUGHT, finance::LAST_SEASON_TOTALS];

/// Length of the NUL-terminated text at `a`.
unsafe fn money_view_len(a: usize) -> usize {
    let mut n = 0;
    while read_u8(a + n) != 0 {
        n += 1;
    }
    n
}

/// `cash` formatted (`exe::cash_format_money` of `exe::cash_get_display_amount` with the same `flags`) into `buf`.
unsafe fn money_view_format(cash: &u32, buf: usize, size: u32, flags: u32) {
    exe::cash_format_money(buf, size, exe::cash_get_display_amount(cash as *const u32, flags), flags);
}

/// A finance block's income: the low dword of the players sold (+8, an i64) plus the eight dwords +0x30..+0x4c (gate
/// receipts, +0x34, prize money, season tickets, +0x40, other income, TV, merchandising), wrapping 32-bit.
unsafe fn money_view_income_total(fin: Finance, block: usize) -> u32 {
    let b = fin.addr() + block;
    (0..8).fold(read_u32(b + 8), |s, k| s.wrapping_add(read_u32(b + 0x30 + 4 * k)))
}

/// A finance block's expenditure: the low dword of the players bought (+0, an i64) plus the eight dwords +0x10..+0x2c
/// (other costs, +0x14, fines, ground, wages, dividends, bonuses, signing-on fees), wrapping 32-bit.
unsafe fn money_view_expenditure_total(fin: Finance, block: usize) -> u32 {
    let b = fin.addr() + block;
    (0..8).fold(read_u32(b), |s, k| s.wrapping_add(read_u32(b + 0x10 + 4 * k)))
}

/// A row title in column 0: the translated `fmt` (with `arg`) in pale grey, left aligned (1), font 3.
unsafe fn money_view_label(area: usize, row: usize, fmt: usize, arg: usize) {
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let cell: ListCellTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
    text(MONEY_VIEW_TEXT, fmt, arg);
    cell(SCREEN_MANAGER, area, 0, row, 1, colour_word(G_GUI_COLOUR_BLACK), 1, 3, colour_word(G_GUI_COLOUR_PALE_GREY), MONEY_VIEW_TEXT);
}

/// A money cell of the table: the shared text in `colour`, font 2, aligned `align`.
unsafe fn money_view_cell(area: usize, col: usize, row: usize, align: usize, colour: usize) {
    let cell: ListCellTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
    cell(SCREEN_MANAGER, area, col, row, 1, colour_word(G_GUI_COLOUR_BLACK), align, 2, colour_word(colour), MONEY_VIEW_TEXT);
}

/// A total cell (Income, Expenditure): `cash` = `value`, formatted with flags 0x22 (0x21 when longer than 13
/// characters) in yellow; returns its display amount with flags 0x20 (read before the cell is added).
unsafe fn money_view_total_cell(cash: &mut u32, area: usize, col: usize, row: usize, align: usize, value: u32) -> i64 {
    *cash = value;
    money_view_format(cash, MONEY_VIEW_TEXT, MONEY_VIEW_TEXT_SIZE, 0x22);
    if money_view_len(MONEY_VIEW_TEXT) > 0xd {
        money_view_format(cash, MONEY_VIEW_TEXT, MONEY_VIEW_TEXT_SIZE, 0x21);
    }
    let shown = exe::cash_get_display_amount(cash as *const u32, 0x20);
    money_view_cell(area, col, row, align, G_GUI_COLOUR_YELLOW);
    shown
}

/// The text of a signed amount already in `cash` (its magnitude): positive with flags 0x22 (0x21 when longer than 13)
/// in the shared text, green; negative formatted into a local buffer with flags 0x22 and size 0xc8 (0x21 and size 0x7d0
/// when longer than 12), then "(%s)" into the shared text, red. Returns the colour.
unsafe fn money_view_signed_text(cash: &u32, negative: bool) -> usize {
    if !negative {
        money_view_format(cash, MONEY_VIEW_TEXT, MONEY_VIEW_TEXT_SIZE, 0x22);
        if money_view_len(MONEY_VIEW_TEXT) > 0xd {
            money_view_format(cash, MONEY_VIEW_TEXT, MONEY_VIEW_TEXT_SIZE, 0x21);
        }
        return MONEY_VIEW_GREEN;
    }
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    // the original's buffer (frame +0xac) has 0x11c bytes but the second format says 0x7d0: room for it here
    let mut buf = [0u8; 0x7d0];
    let b = buf.as_mut_ptr() as usize;
    money_view_format(cash, b, 0xc8, 0x22);
    if money_view_len(b) > 0xc {
        money_view_format(cash, b, 0x7d0, 0x21);
    }
    sprintf(MONEY_VIEW_TEXT, 0x98fb04, b); // "(%s)"
    G_GUI_COLOUR_RED
}

/// The money views of the club screen's info tab (Finances, Income, Expenditure): the five-column table and the rows
/// of the view in `view`.
///
/// The table runs from `top` to `bottom` on the left of the screen (x 0x6e): as many rows as fit in font 3 below a
/// 0x10 margin, `(bottom − top − 0x10) / font_get_height(3)` (truncated to a byte), and the columns 2, 1, 1, 1, 1. Two
/// areas are added over the same rows: one to x 0x30c (mode 2, its index unused) and the grid to x 0x302 (mode 1) that
/// holds every cell. Row 0 holds the titles in orange: "  Item" (left), "This Month", "Last Month", "This Year" and
/// "Last Year"; the amounts are right aligned (1) when the currency symbol is not empty, centred (0x40) otherwise. An
/// orange line (`Display::add_item`, kind 1, 0x7d to 0x2fd, flags 0xc400) closes the titles.
///
/// Finances (bit 2 of `view`): the rows "  Income" (2), "  Expenditure" (3), "  Profit/(Loss)" (5), "  Balance" (7) and
/// "  Transfer Funds" (8). The totals of each column come from the club's `Finance` blocks (this month, last month,
/// this season, last season): the income adds the low dword of the players sold and the eight income dwords, the
/// expenditure the low dword of the players bought and the eight cost dwords, as 32-bit sums put in a `Cash` and
/// shown in yellow (flags 0x22, or 0x21 when the text has more than 13 characters). Profit/(Loss) is the difference
/// of the two display amounts (flags 0x20) of the column, set back into the `Cash`: green when not negative, red in
/// parentheses with its magnitude when negative. The balance (`finance::BALANCE`) is shown the same way from its low
/// dword only (negated when the i64 is negative). The transfer funds (`Finance::get_transfer_budget`(0, 0, 0, 1, 0))
/// show "None available" in red when negative, otherwise the low dword of a second call, flags 6 (0x21 past 13
/// characters), in green.
///
/// Income (bit 4) and Expenditure (otherwise) are the rows of `info_tab_income_rows` and `info_tab_expenditure_rows`.
///
/// Inputs, as the original holds them at 0x462a80: `top` = esi (the tab's top + 0x41, also frame +0x2dc), `bottom` =
/// the second argument (frame +0x2e0, edi from 0x462ab3), `view` = the low byte of frame +0x14 (param 0x1b, forced to
/// 1 by the header for a national team or a club the human does not control), `fin` = ebp (the club's record from
/// `FinanceTable::get_club_finance`), `cash` = the `Cash` at frame +0x10 (0 from the prologue).
/// Defects of the original: the income and expenditure totals take only the low dword of the players sold and bought
/// (i64) and add in 32 bits; the balance shows only its low dword (a balance past ±2^31 in cash units, or a negative
/// one past −2^32, comes out wrong); the negative text's second format says size 0x7d0 for a 0x11c-byte buffer.
/// Original: piece of `club_screen_draw_info_tab` (0x462a80..0x463bd8 and the tail 0x46715b, club_screens.cpp).
unsafe fn info_tab_money_views(top: i32, bottom: i32, view: u8, fin: Finance, cash: &mut u32) {
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let add_area: DisplayListFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    let add_line: DisplayLineFn = core::mem::transmute(FN_DISPLAY_ADD_ITEM);
    let set_cash: unsafe extern "thiscall" fn(*mut u32, i64) = core::mem::transmute(MONEY_VIEW_FN_CASH_SET_DISPLAY);
    // 0x43f440 returns the symbol's length: right aligned with a symbol, centred without
    let align: usize = if read_u8(MONEY_VIEW_CURRENCY_SYMBOL) != 0 { 1 } else { 0x40 };
    let rows = ((bottom - top - 0x10) / exe::font_get_height(3)) as u8 as usize;
    let mut widths: [u8; 5] = [2, 1, 1, 1, 1];
    let w = widths.as_mut_ptr() as usize;
    add_area(current_human_display(), 0x6e, top as usize, 0x30c, bottom as usize, 5, w, rows, 0, 2, 0, usize::MAX);
    let area = add_area(current_human_display(), 0x6e, top as usize, 0x302, bottom as usize, 5, w, rows, 0, 1, 0, usize::MAX);
    let cell: ListCellTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
    text(MONEY_VIEW_TEXT, 0x98fb68); // "  Item"
    cell(SCREEN_MANAGER, area, 0, 0, 1, colour_word(G_GUI_COLOUR_BLACK), 1, 3, colour_word(MONEY_VIEW_ORANGE), MONEY_VIEW_TEXT);
    for (col, title) in [(1usize, 0x98fb5cusize), (2, 0x98fb50), (3, 0x98fb44), (4, 0x98fb38)] {
        text(MONEY_VIEW_TEXT, title); // "This Month", "Last Month", "This Year", "Last Year"
        money_view_cell(area, col, 0, align, MONEY_VIEW_ORANGE);
    }
    add_line(current_human_display(), 1, 0x7d, 0, 0x2fd, 0, 0, 1, 0xc400, colour_word(MONEY_VIEW_ORANGE), 0, 0, 0, 0, 0, 0, 0, 0, area);
    if view & 2 == 0 {
        // live here (0x463bd9 / 0x4657a4): esi = `area`, ebx = `align`, ebp = `fin`, edi = `bottom`; frame +0x10 =
        // `cash`, +0x14 = `view`, +0x1c = the club, +0x24 = `rows` (dword), +0x30 = 0, +0x5c = `widths`, +0x2dc = `top`
        if view & 4 != 0 {
            covered!("tel02_2 info tab income view");
            info_tab_income_rows(fin.0, area, align);
        } else {
            covered!("tel02_2 info tab expenditure view");
            info_tab_expenditure_rows(fin.0, area, align);
        }
        return;
    }
    covered!("tel02_2 info tab finances view");
    money_view_label(area, 2, 0x98fb2c, 0); // "  Income"
    let mut income = [0i64; 4];
    for (k, &b) in MONEY_VIEW_BLOCKS.iter().enumerate() {
        income[k] = money_view_total_cell(cash, area, k + 1, 2, align, money_view_income_total(fin, b));
    }
    money_view_label(area, 3, 0x98fb1c, 0); // "  Expenditure"
    let mut spent = [0i64; 4];
    for (k, &b) in MONEY_VIEW_BLOCKS.iter().enumerate() {
        spent[k] = money_view_total_cell(cash, area, k + 1, 3, align, money_view_expenditure_total(fin, b));
    }
    money_view_label(area, 5, 0x98fb0c, 0); // "  Profit/(Loss)"
    for k in 0..4 {
        let diff = income[k].wrapping_sub(spent[k]); // tel02_2 profit is income minus expenditure
        let negative = diff < 0;
        set_cash(cash, if negative { diff.wrapping_neg() } else { diff });
        let colour = money_view_signed_text(cash, negative);
        money_view_cell(area, k + 1, 5, align, colour);
    }
    money_view_label(area, 7, 0x98fadc, MONEY_VIEW_EMPTY_TEXT); // "  Balance<%s - COMMENT - bank balance>"
    let balance = fin.balance();
    *cash = if balance < 0 { (balance as u32).wrapping_neg() } else { balance as u32 };
    let colour = money_view_signed_text(cash, balance < 0);
    money_view_cell(area, 1, 7, align, colour);
    money_view_label(area, 8, 0x98fac8, 0); // "  Transfer Funds"
    let colour = if exe::finance_get_transfer_budget(fin, 0, 0, 0usize, 1, 0) < 0 {
        text(MONEY_VIEW_TEXT, 0x98fab8); // "None available"
        G_GUI_COLOUR_RED
    } else {
        *cash = exe::finance_get_transfer_budget(fin, 0, 0, 0usize, 1, 0) as u32;
        money_view_format(cash, MONEY_VIEW_TEXT, MONEY_VIEW_TEXT_SIZE, 6);
        if money_view_len(MONEY_VIEW_TEXT) > 0xd {
            money_view_format(cash, MONEY_VIEW_TEXT, MONEY_VIEW_TEXT_SIZE, 0x21);
        }
        MONEY_VIEW_GREEN
    };
    money_view_cell(area, 1, 8, align, colour); // the shared tail 0x46715b
}

// ---------- club info tab: the Income and Expenditure views (club_screens.cpp, 0x463bd9..0x467165 of 0x460590) ----------

/// Cash::set_from_amount (0x43f4c0, thiscall(Cash*, i64 amount) ret 8): the cash value becomes amount / the display
/// factor (fild/fdiv/_ftol).
const LEDGER_FN_CASH_SET_FROM_AMOUNT: usize = 0x43f4c0;
/// The colour word of the Total rows (label and the four values) [inference: a bright text colour].
const LEDGER_COLOUR_TOTAL: usize = 0xaebde8;
/// The empty string, the `%s` argument of the Interest, Other and Total labels.
const LEDGER_EMPTY_TEXT: usize = 0xad9c64;
const LEDGER_TEXT: usize = 0xde1f64;
/// The four money blocks of the club Finance record, in column order: this month, last month (Finance::get_last_month
/// 0x4671b0), this season (0x4671e0) and last season (0x4671d0); each 0x50 bytes with the same layout as the month block.
const LEDGER_BLOCKS: [usize; 4] = [finance::MONTH_PLAYERS_BOUGHT, 0x20, finance::SEASON_PLAYERS_BOUGHT, finance::LAST_SEASON_TOTALS];
/// Month-block fields with no name in fields.rs yet, named from the screen's labels [inference].
const LEDGER_MONTH_INTEREST_PAID: usize = 0x84; // "Interest" (expenditure)
const LEDGER_MONTH_OTHER_INCOME_ROW: usize = 0xa4; // "Other" (income)
const LEDGER_MONTH_INTEREST_RECEIVED: usize = 0xb0; // "Interest" (income)

/// The Income rows: (label text, month field, label has the `%s` comment argument).
const LEDGER_INCOME_ROWS: [(usize, usize, bool); 9] = [
    (0x98faa8, finance::MONTH_GATE_RECEIPTS, false),   // "  Gate Receipts"
    (0x98fa94, finance::MONTH_SEASON_TICKETS, false),  // "  Season Tickets"
    (0x98fa84, finance::MONTH_TV_REVENUE, false),      // "  TV Revenue"
    (0x98fa74, finance::MONTH_MERCHANDISING, false),   // "  Merchandising"
    (0x98fa64, finance::MONTH_PLAYERS_SOLD, false),    // "  Players Sold" (low dword of the i64)
    (0x98fa54, finance::MONTH_PRIZE_MONEY, false),     // "  Prize Money"
    (0x98fa44, finance::MONTH_OTHER_INCOME, false),    // "  Investments"
    (0x98fa24, LEDGER_MONTH_INTEREST_RECEIVED, true),  // "  Interest<%s - COMMENT - cash>"
    (0x98fa04, LEDGER_MONTH_OTHER_INCOME_ROW, true),   // "  Other<%s - COMMENT - income>"
];
/// The Expenditure rows, same layout.
const LEDGER_EXPENDITURE_ROWS: [(usize, usize, bool); 9] = [
    (0x98f9dc, finance::MONTH_WAGES, false),           // "  Wages"
    (0x98f9d0, finance::MONTH_BONUSES, false),         // "  Bonuses"
    (0x98f9bc, finance::MONTH_SIGN_ON_FEES, false),    // "  Signing On Fees"
    (0x98f9b0, finance::MONTH_DIVIDENDS, false),       // "  Dividends"
    (0x98f99c, finance::MONTH_PLAYERS_BOUGHT, false),  // "  Players Bought" (low dword of the i64)
    (0x98f984, finance::MONTH_GROUND_COSTS, false),    // "  Ground Maintenance"
    (0x98f974, finance::MONTH_FINES, false),           // "  League Fines"
    (0x98fa24, LEDGER_MONTH_INTEREST_PAID, true),      // "  Interest<%s - COMMENT - cash>"
    (0x98f950, finance::MONTH_OTHER_COSTS, true),      // "  Other<%s - COMMENT - expenditure>"
];

/// Formats the cash value `cash` into the shared text as money (`cash_format_money` of `Cash::get_display_amount` with
/// flags 0x22, 2000 bytes); when that is longer than 13 characters, again with flags 0x21 (the short form).
unsafe fn ledger_format(cash: &u32) {
    exe::cash_format_money(LEDGER_TEXT, 0x7d0, exe::cash_get_display_amount(cash, 0x22), 0x22);
    let mut n = 0usize;
    while read_u8(LEDGER_TEXT + n) != 0 {
        n += 1;
    }
    if n > 0xd {
        exe::cash_format_money(LEDGER_TEXT, 0x7d0, exe::cash_get_display_amount(cash, 0x21), 0x21);
    }
}

/// One cell of the money table `area`: `ScreenManager::add_area_text`(area, col, row, 1, black, align, 2, fg, text);
/// the labels go in column 0 with (1, 3) instead of (align, 2).
unsafe fn ledger_cell(area: usize, col: usize, row: usize, a5: usize, a6: usize, fg: usize) {
    let cell: ListCellTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
    cell(SCREEN_MANAGER, area, col, row, 1, colour_word(G_GUI_COLOUR_BLACK), a5, a6, colour_word(fg), LEDGER_TEXT);
}

/// The rows of one money view and its Total row (row 0xc; row 0xb stays empty).
///
/// Each row (rows 2 to 0xa): the translated label (`langlib_sprintf`, with the empty text as the `%s` argument where the
/// label has one) in column 0, pale grey; then the four columns (this month, last month, this season, last season) from
/// the club's Finance record `fin`: the dword of the row's field in that column's 0x50-byte block (the low dword for the
/// i64 transfer fields) becomes the cash value, formatted as money (`ledger_format`) in yellow, and its unrounded
/// display amount (`Cash::get_display_amount` with 0x20) is added to the column's 64-bit total. The Total row: the label
/// and each column's total turned back into a cash value (`Cash::set_from_amount`) and formatted, all in the total
/// colour.
unsafe fn ledger_rows(fin: usize, area: usize, align: usize, rows: &[(usize, usize, bool); 9], total_label: usize) {
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let set_from_amount: unsafe extern "thiscall" fn(*mut u32, i64) = core::mem::transmute(LEDGER_FN_CASH_SET_FROM_AMOUNT);
    let mut totals = [0i64; 4];
    let mut cash = 0u32;
    for (k, &(label, field, comment)) in rows.iter().enumerate() {
        let row = 2 + k;
        if comment {
            text(LEDGER_TEXT, label, LEDGER_EMPTY_TEXT);
        } else {
            text(LEDGER_TEXT, label);
        }
        ledger_cell(area, 0, row, 1, 3, G_GUI_COLOUR_PALE_GREY);
        for (c, block) in LEDGER_BLOCKS.iter().enumerate() {
            cash = read_u32(fin + block + field - finance::MONTH_PLAYERS_BOUGHT);
            ledger_format(&cash);
            totals[c] = totals[c].wrapping_add(exe::cash_get_display_amount(&cash, 0x20));
            ledger_cell(area, c + 1, row, align, 2, G_GUI_COLOUR_YELLOW);
        }
    }
    text(LEDGER_TEXT, total_label, LEDGER_EMPTY_TEXT);
    ledger_cell(area, 0, 0xc, 1, 3, LEDGER_COLOUR_TOTAL);
    for (c, total) in totals.iter().enumerate() {
        set_from_amount(&mut cash, *total);
        ledger_format(&cash);
        ledger_cell(area, c + 1, 0xc, align, 2, LEDGER_COLOUR_TOTAL);
    }
}

/// The Income view of the club info tab's money table: Gate Receipts, Season Tickets, TV Revenue, Merchandising,
/// Players Sold, Prize Money, Investments, Interest and Other, then the Total (see `ledger_rows`).
///
/// `fin` = the club's Finance record (ebp, `FinanceTable::get_club_finance` at 0x460686), `area` = the money table area
/// (esi, the second `Display::add_area` at 0x462b32), `align` = the value cells' alignment (ebx: 1 when the currency
/// text 0xad9ca8 is not empty, 0x40 otherwise, from 0x462a94). The original falls into the common exit at 0x467166.
/// Original: part of `club_screen_draw_info_tab` (0x463bd9..0x4657a3, club_screens.cpp).
unsafe fn info_tab_income_rows(fin: usize, area: usize, align: usize) {
    covered!("tel02_2 income view");
    ledger_rows(fin, area, align, &LEDGER_INCOME_ROWS, 0x98f9e4); // "  Total<%s - COMMENT - income>"
}

/// The Expenditure view of the club info tab's money table: Wages, Bonuses, Signing On Fees, Dividends, Players Bought,
/// Ground Maintenance, League Fines, Interest and Other, then the Total (see `ledger_rows`). Same parameters as
/// `info_tab_income_rows`.
/// Original: part of `club_screen_draw_info_tab` (0x4657a4..0x467165, club_screens.cpp).
unsafe fn info_tab_expenditure_rows(fin: usize, area: usize, align: usize) {
    covered!("tel02_2 expenditure view");
    ledger_rows(fin, area, align, &LEDGER_EXPENDITURE_ROWS, 0x98f92c); // "  Total<%s - COMMENT - expenditure>"
}
