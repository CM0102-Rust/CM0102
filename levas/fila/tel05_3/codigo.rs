// ---------- batch %%LEVA%%: the club screens (5c): the squad numbers and European squad screen (club_screens.cpp) ----------

/// The squad numbers screen's state: what the original keeps in registers and stack slots across its pieces.
struct SquadNumbersScreen {
    club: usize,
    /// param 2: 1 the squad numbers, otherwise the European squad
    mode: u8,
    view: u32,
    sort: u32,
    category: u8,
    option9: u8,
    /// param 0xa (byte), then the row count, then the attribute table (the original reuses the slot)
    slot_a: usize,
    /// params 0xb..0xf (bytes): the European rule byte, the squad size asked, and the registered counts
    rule_b: u8,
    squad_size: u8,
    rule_d: u8,
    registered: u8,
    reserves: u8,
    /// param 0x11: the squad copy (50 player slots); its slot later holds the Traditional view's row count
    squad: usize,
    fixture: usize,
    /// the club's side of the fixture (1 when its side byte is 0)
    side: u32,
    title: [u8; 0xc8],
    list: [u8; 0x20],
    /// the last area made (ebp in the original: the rows go there when there is no view of its own)
    last_area: u32,
    top: i32,
    bottom: i32,
}

/// Rule: the squad numbers screen (and its European squad mode) shows the club's players in the chosen view with their
/// squad numbers (the 50 numbers on two rows above and below the list, who holds each) or their European registration,
/// sorted and filtered as the menus say; for a match only the selected players are listed.
///
/// `squad_numbers_screen_build()`, from the screen parameters 0 (the human manager: his club, `staff::CLUB_JOB`), 2 (the
/// mode), 4 (view), 5 (sort), 6, 9, 0xa (list options), 0xb..0xf (the European rule bytes and counts), 0x11 (the squad
/// copy), 0x12 (the fixture), 7 (the scroll row). The pieces: `squad_numbers_build_header` (the squad copy, the title, the
/// menus), `squad_numbers_build_top` (the numbers grid or the selected count, the club's list), `squad_numbers_build_area`
/// (the list's area and headings), `squad_numbers_build_rows` and the footer (the scroll row from param 7, the area
/// written to param 8, the Back (link 0xb) and Submit (link 0xc) buttons). A local list (`LLIST`), destroyed at the end.
/// Original: `squad_numbers_screen_build` (0x475520, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn squad_numbers_screen_build() {
    covered!("tel05_3 squad numbers screen");
    let mut s = SquadNumbersScreen {
        club: 0,
        mode: 0,
        view: 0,
        sort: 0,
        category: 0,
        option9: 0,
        slot_a: 0,
        rule_b: 0,
        squad_size: 0,
        rule_d: 0,
        registered: 0,
        reserves: 0,
        squad: 0,
        fixture: 0,
        side: 0,
        title: [0; 0xc8],
        list: [0; 0x20],
        last_area: 0,
        top: 0,
        bottom: 0,
    };
    part_exchange_llist(&mut s.list, true);
    let mut w = PartExchangeWeights {
        two: [6, 5, 0x1e, 0x10, 6, 5, 0x1e, 0x10],
        contract: [5, 5, 0x18, 0xe, 0xe, 0xf, 0xb, 0, 0, 0, 0, 0, 0xa],
        stats: [5, 5, 0x18, 6, 6, 6, 6, 6, 6, 6, 6, 6, 0xa],
        attributes: [5, 5, 0x18, 6, 6, 6, 6, 6, 6, 6, 6, 6, 0xa],
        other: [5, 5, 0x18, 8, 6, 6, 6, 0xa, 0xc, 6, 0, 0, 0xa],
        selection: [5, 5, 0x18, 0xe, 0xa, 0xc, 6, 6, 6, 0, 0, 0, 0xa],
    };
    let bottom_weights = [3u8, 1];
    if squad_numbers_build_header(&mut s) {
        squad_numbers_build_top(&mut s);
        let area = squad_numbers_build_area(&mut s, &mut w);
        squad_numbers_build_rows(&mut s, area);
        let scroll = get_screen_param(7);
        if area as u16 != 0xffff {
            let d = screens_current_display(G_SCREEN_MANAGER);
            exe::area_set_scroll_row(d.wrapping_add(4).wrapping_add((area as i16 as isize as usize).wrapping_mul(0xbf1)), scroll);
        }
        set_screen_param(8, area as i16 as i32 as usize);
        let bar = tel26_add_area(screens_current_display(G_SCREEN_MANAGER), [100, 0x22b, 0x316, 0x24e, 2, bottom_weights.as_ptr() as u32, 1, 0, 1, 0, u32::MAX]);
        let (c317e, c3184) = (part_exchange_colour(0xae317e), part_exchange_colour(0xae3184));
        part_exchange_lang()(0xde1f64, 0x9880a8); // "Back"
        tel26_area_button([bar, 0, 0, 0x30, c317e, 0xc, 3, c3184, 0xde1f64, 0xb, 0]);
        part_exchange_lang()(0xde1f64, 0x9910c0); // "Submit"
        tel26_area_button([bar, 1, 0, 0x30, c317e, 0xc, 3, c3184, 0xde1f64, 0xc, 0]);
    }
    part_exchange_llist(&mut s.list, false);
}

/// The parameters, the squad copy, the title and the menus of the squad numbers screen.
///
/// Without a squad copy (param 0x11) the club's squad is copied into a 200-byte block (`database_collect_club_squad`;
/// the "Error" box at line 0x398a and nothing else when it cannot be allocated, returns false) written to param 0x11; in
/// the European mode (2) a player on loan (`CONTRACT_MANAGER::get_loan_contract`) or whose contract
/// (`CONTRACT_MANAGER::get_contract`, kept in [0xae1080] as the original) is of type 5 leaves the copy. The club's
/// background picture (`background_set_club_picture`). The title "<club> Squad Numbers" or "<club> European Squad" in the
/// club's kit colours (locals the original leaves uninitialised; here zero) and the subtitle "Please Select"
/// (`ScreenManager::add_screen_subtitle`, 0x803f80). The menu bar (`exe::menubar_build` 4), the view bar (0x6e..0x168 by
/// 0x91..0xa5) and its menus (`squad_list_menus`, links 1, 6, 2 and 1; "Club" instead of "Nationality" for a national or
/// all-star team), the Auto (link 3) and Clear (link 4) buttons (0x212..0x30c) and the title (font 3, 0x6e..0x30c by
/// 0xaa..0xcd).
unsafe fn squad_numbers_build_header(s: &mut SquadNumbersScreen) -> bool {
    let sm = G_SCREEN_MANAGER;
    s.club = read_usize(get_screen_param(0) + staff::CLUB_JOB);
    s.mode = get_screen_param(2) as u8;
    s.view = get_screen_param(4) as u32;
    s.sort = get_screen_param(5) as u32;
    s.category = get_screen_param(6) as u8;
    s.option9 = get_screen_param(9) as u8;
    s.slot_a = get_screen_param(0xa) as u8 as usize;
    s.rule_b = get_screen_param(0xb) as u8;
    s.squad_size = get_screen_param(0xc) as u8;
    s.rule_d = get_screen_param(0xd) as u8;
    s.registered = get_screen_param(0xe) as u8;
    s.reserves = get_screen_param(0xf) as u8;
    s.squad = get_screen_param(0x11);
    s.fixture = get_screen_param(0x12);
    let club = s.club;
    if s.fixture != 0 {
        let side = if club == read_usize(s.fixture + fixture::HOME_CLUB) { fixture::HOME_SIDE_MARK } else { fixture::AWAY_SIDE_MARK };
        s.side = (read_u8(s.fixture + side) == 0) as u32;
    }
    if s.squad == 0 {
        let block = exe::crt_malloc(200);
        s.squad = block;
        if block == 0 {
            show_error_at(SQUAD_NUMBERS_SOURCE_FILE, 0x398a);
            return false;
        }
        exe::database_collect_club_squad(club, block as *mut usize, 0);
        set_screen_param(0x11, block);
        if s.mode == 2 {
            covered!("tel05_3 european squad copy");
            for k in 0..SQUAD_NUMBERS_SLOTS {
                let p = read_usize(block + 4 * k);
                if p == 0 {
                    continue;
                }
                if exe::contract_manager_get_loan_contract(read_usize(G_CONTRACT_MANAGER), p) != 0 {
                    write_usize(block + 4 * k, 0);
                    continue;
                }
                let contract = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), p);
                write_usize(G_CONTRACT_LOOKUP_RESULT, contract);
                if contract != 0 && read_u8(contract + contract::CONTRACT_TYPE) & 0x3f == 5 {
                    write_usize(block + 4 * k, 0);
                }
            }
        }
    }
    exe::background_set_club_picture(club, 0, 0);
    let lang = part_exchange_lang();
    let gender = read_i8(club + club::SHORT_GENDER_NAME) as i32;
    // "{}<%s - club>{} Squad Numbers" / "{}<%s - club>{} European Squad"
    lang(0xde1f64, if s.mode == 1 { 0x9911f8 } else { 0x9911d8 }, gender, gender, club + club::SHORT_NAME);
    let (mut kit_a, mut kit_b) = (0u32, 0u32);
    exe::database_get_club_kit_colours(club, 1, &mut kit_a, &mut kit_b);
    exe::screenmanager_add_screen_title(sm, kit_b, kit_a, 0xde1f64usize);
    lang(0xde1f64, 0x9911c8); // "Please Select"
    let subtitle: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x803f80usize);
    subtitle(sm, 0xde1f64, 1);
    exe::menubar_build(4, 0);
    s.title[0] = 0;
    let d = screens_current_display(sm);
    let bar = tel26_add_area(d, [0x6e, 0x91, 0x168, 0xa5, 2, 0, 1, 0, 1, 0, u32::MAX]);
    let club_label = if s.view & 1 != 0 && exe::nationalteams_is_national_or_all_star_team(read_usize(G_NATIONAL_TEAMS), club) != 0 {
        0x98edb0 // " Club"
    } else {
        0x98edb8 // " Nationality"
    };
    squad_list_menus(bar, s.view, s.sort, s.category, s.title.as_mut_ptr(), [1, 6, 2, 1], club_label);
    let buttons = tel26_add_area(screens_current_display(sm), [0x212, 0x91, 0x30c, 0xa5, 2, 0, 1, 0, 1, 0, u32::MAX]);
    let (c317e, c3184) = (part_exchange_colour(0xae317e), part_exchange_colour(0xae3184));
    lang(0xde1f64, 0x9911a0, 0xad9c64usize); // "Auto"
    tel26_area_button([buttons, 0, 0, 0x30, c317e, 0xc, 1, c3184, 0xde1f64, 3, 0]);
    lang(0xde1f64, 0x991174, 0xad9c64usize); // "Clear"
    tel26_area_button([buttons, 1, 0, 0x30, c317e, 0xc, 1, c3184, 0xde1f64, 4, 0]);
    s.last_area = buttons;
    let manager_text: ManagerTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_TEXT_ITEM);
    manager_text(sm, 0x6e, 0xaa, 0x30c, 0xcd, 2, 0, 0xc, 3, part_exchange_colour(0xaebdec) as usize, s.title.as_ptr() as usize, 1);
    true
}

/// The squad numbers grid (squad numbers mode) or the selected count (European mode), then the club's list.
///
/// Squad numbers: two areas of 25 columns (0x6e..0x30c by 0xd2..0xe6 and 0x203..0x217), the numbers 1..25 and 26..50:
/// each one free (flags 0x46, link 5, data the number, the pale colour), held for good (0x46, 0x70, white) or held
/// provisionally (an item 1 with font 0x2c and no link); the list between 0xeb and 0x1fe, filter 0x164c21. European: "No
/// Players Selected", "1 Player Selected" or "<n> Players Selected" (registered - reserves + rule byte 0xd) under the
/// list (0x6e..0x30c by 500..0x217); the list between 0xd2 and 0x1ef, filter 0x224c21. The list
/// (`build_filtered_sorted_squad_list`, 0x4841d0: the club, the sort, style 3, params 6, 9 and 0xa, the squad copy) and
/// its count written to param 0x10.
unsafe fn squad_numbers_build_top(s: &mut SquadNumbersScreen) {
    let sm = G_SCREEN_MANAGER;
    let filter;
    if s.mode == 1 {
        covered!("tel05_3 numbers grid");
        let d = screens_current_display(sm);
        let first = tel26_add_area(d, [0x6e, 0xd2, 0x30c, 0xe6, 0x19, 0, 1, 0, 1, 0, u32::MAX]);
        let second = tel26_add_area(screens_current_display(sm), [0x6e, 0x203, 0x30c, 0x217, 0x19, 0, 1, 0, 1, 0, u32::MAX]);
        let mut area = first;
        let mut col = 0u32;
        let (c3150, c3184, cbdec) = (part_exchange_colour(0xae3150), part_exchange_colour(0xae3184), part_exchange_colour(0xaebdec));
        for n in 1u32..=0x32 {
            let holder = squad_numbers_holder(s.squad, s.club, n as i32);
            part_exchange_sprintf()(0xde1f64, 0x98806c, n); // "%d"
            let f = if holder == 0 {
                [0x46, 0, 0, 0, 0, col, 0, 0x30, c3150, c3150, 0xc, 1, c3184, 0xde1f64, 0, 5, n, area]
            } else if squad_numbers_number(holder, s.club) > 0 {
                [0x46, 0, 0, 0, 0, col, 0, 0x70, c3150, c3150, 0xc, 1, cbdec, 0xde1f64, 0, 5, n, area]
            } else {
                [1, 0, 0, 0, 0, col, 0, 0x70, c3150, c3150, 0x2c, 1, cbdec, 0xde1f64, 0, 0, 0, area]
            };
            tel26_add_item(screens_current_display(sm), f);
            col += 1;
            if col >= 0x19 {
                area = second;
                col = 0;
            }
        }
        s.last_area = area;
        filter = 0x164c21;
        s.top = 0xeb;
        s.bottom = 0x1fe;
    } else {
        let count = s.registered as i8 as i32 - s.reserves as i8 as i32 + s.rule_d as i8 as i32;
        let lang = part_exchange_lang();
        if count == 0 {
            lang(0xde1f64, 0x991160); // "No Players Selected"
        } else if count == 1 {
            lang(0xde1f64, 0x99114c); // "1 Player Selected"
        } else {
            lang(0xde1f64, 0x99112c, count); // "<%d - number> Players Selected"
        }
        let manager_text: ManagerTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_TEXT_ITEM);
        manager_text(sm, 0x6e, 500, 0x30c, 0x217, 2, 0, 0xc, 3, part_exchange_colour(0xaebdec) as usize, 0xde1f64, 1);
        filter = 0x224c21;
        s.top = 0xd2;
        s.bottom = 0x1ef;
    }
    let build: unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, u32, u32, usize, usize, usize, u32, usize, u32) -> u32 =
        core::mem::transmute(0x4841d0usize);
    // simplification: params 6, 9 and 0xa are bytes in stack slots whose upper bytes are garbage in the original (chars)
    build(s.club, 0, s.sort, 3, s.category as u32, s.option9 as u32, s.slot_a as u32, filter, s.list.as_mut_ptr() as usize, 0, 0, 0, s.squad, 0);
    set_screen_param(0x10, read_i16(s.list.as_ptr() as usize) as i32 as usize);
}

/// The list's area and the column headings (as `part_exchange_list_area`, between the top and the bottom of
/// `squad_numbers_build_top`).
///
/// Traditional view: 8 columns (two players per row), the rows that fit in font 3 in the low byte of the squad copy's
/// slot (as the original). Other views: the background area, the rows that fit in font 2 (into the slot of param 0xa)
/// and their height (the height over the rows + 0.5, `ftol`); the headings one row high from top + 8, 0x2f6 wide when the
/// list has more players than rows: "No." 0x20 (squad numbers) or "Reg" 0x40 (European), "Inf" 2, "   Name" 4 (link 6),
/// then the view's: contract ("Club" for a national or all-star team, else Squad Status 0x10, Basic Wage 0x20000,
/// Contract Expiry 0x40000, Releases 0x2000000 when not a national team, whose weights are 0x12, 0x12, 0x12, 0);
/// selection; statistics (link 7); attributes (link 8, the table of the chosen group sorted in place by qsort with
/// 0x487720; with none chosen the row count is taken as the table, as the original); other; "Value" 0x80000. The list area
/// from row height + top + 8 to bottom - 8. No view: the last area made.
unsafe fn squad_numbers_build_area(s: &mut SquadNumbersScreen, w: &mut PartExchangeWeights) -> u32 {
    let sm = G_SCREEN_MANAGER;
    let lang = part_exchange_lang();
    let v = s.view;
    let (top, bottom) = (s.top, s.bottom);
    let d = || screens_current_display(sm);
    if v & 1 != 0 {
        covered!("tel05_3 traditional view");
        let rows = ((bottom - top - 0x10) / exe::font_get_height(3)) as u8;
        s.squad = (s.squad & 0xffff_ff00) | rows as usize;
        return tel26_add_area(d(), [0x6e, top as u32, 0x30c, bottom as u32, 8, w.two.as_ptr() as u32, s.squad as u32, 0, 2, 0, u32::MAX]);
    }
    tel26_add_area(d(), [0x6e, top as u32, 0x30c, bottom as u32, 1, 0, 1, 0, 2, 0, u32::MAX]);
    let height = bottom - top - 0x10;
    let rows = (height / exe::font_get_height(2)) as u8;
    s.slot_a = rows as usize;
    let row_h = ftol(height as f64 / rows as u32 as f64 + read_f64(rdata::F64_0_5));
    let club = s.club;
    let heading = |area: u32, col: u32, font: u32, link: u32, data: u32| -> u32 {
        tel26_area_button([area, col, 0, 0x30, part_exchange_colour(0xae317e), font, 1, part_exchange_colour(0xae3184), 0xde1f64, link, data])
    };
    let count = read_i16(s.list.as_ptr() as usize) as i32;
    let x2 = if count > rows as i32 - 1 { 0x2f6 } else { 0x30c };
    let head = |weights: *const u8| tel26_add_area(d(), [0x6e, (top + 8) as u32, x2, (row_h + top + 7) as u32, 0xd, weights as u32, 1, 0, 1, 0, u32::MAX]);
    let mode = s.mode;
    let first_three = |area: u32| {
        if mode == 1 {
            lang(0xde1f64, 0x991128);
            heading(area, 0, 0xc, 6, 0x20);
        } else {
            lang(0xde1f64, 0x991124);
            heading(area, 0, 0xc, 6, 0x40);
        }
        lang(0xde1f64, 0x98ea64); // "Inf"
        heading(area, 1, 0xc, 6, 2);
        lang(0xde1f64, 0x98ea5c); // "   Name"
        heading(area, 2, 1, 6, 4);
    };
    let value_heading = |area: u32| {
        lang(0xde1f64, 0x98ea0c); // "Value"
        heading(area, 0xc, 0xc, 6, 0x80000);
    };
    let stat_label: unsafe extern "thiscall" fn(usize, u32, usize, u32) = core::mem::transmute(0x7c1b20usize);
    let weights: *const u8;
    if v & 2 != 0 {
        covered!("tel05_3 contract view");
        if exe::database_is_national_team(club) != 0 {
            w.contract[3] = 0x12;
            w.contract[4] = 0x12;
            w.contract[5] = 0x12;
            w.contract[6] = 0;
        }
        weights = w.contract.as_ptr();
        let area = head(weights);
        first_three(area);
        let label = if exe::nationalteams_is_national_or_all_star_team(read_usize(G_NATIONAL_TEAMS), club) == 0 { 0x98ea4c } else { 0x98ea44 };
        lang(0xde1f64, label); // "   Squad Status" / "   Club"
        heading(area, 3, 1, 6, 0x10);
        lang(0xde1f64, 0x98ea34); // "   Basic Wage"
        heading(area, 4, 1, 6, 0x20000);
        lang(0xde1f64, 0x98ea20); // "   Contract Expiry"
        heading(area, 5, 1, 6, 0x40000);
        if exe::database_is_national_team(club) == 0 {
            lang(0xde1f64, 0x98ea14); // "   Releases"
            heading(area, 6, 1, 6, 0x2000000);
        }
        value_heading(area);
    } else if v & 0x1000 != 0 {
        weights = w.selection.as_ptr();
        let area = head(weights);
        first_three(area);
        lang(0xde1f64, 0x98ea00); // "Position"
        heading(area, 3, 1, 6, 8);
        for (col, label, data) in [(4u32, 0x98e9f8usize, 0x80u32), (5, 0x98e9f0, 0x100), (6, 0x98e9e8, 0x200)] {
            lang(0xde1f64, label);
            heading(area, col, 0xc, 6, data);
        }
        stat_label(read_usize(G_PLAYER_STATS), 1, 0xde1f64, 0x7d0);
        heading(area, 7, 0xc, 6, 0x10000000);
        stat_label(read_usize(G_PLAYER_STATS), 0x11, 0xde1f64, 0x7d0);
        heading(area, 8, 0xc, 6, 0x800000);
        value_heading(area);
    } else if v & 4 != 0 || v & 8 != 0 {
        weights = w.stats.as_ptr();
        let area = head(weights);
        first_three(area);
        let table = if v & 4 != 0 { 0x98de84 } else { 0x98de90 };
        let mut k = 0usize;
        while read_u8(table + k) != 0xff {
            let field = read_u8(table + k);
            stat_label(read_usize(G_PLAYER_STATS), field as u32, 0xde1f64, 0x7d0);
            heading(area, k as u32 + 3, 0xc, 7, field as i8 as i32 as u32);
            k += 1;
        }
        value_heading(area);
    } else if v & 0x10 != 0 {
        covered!("tel05_3 attributes view");
        weights = w.attributes.as_ptr();
        let area = head(weights);
        first_three(area);
        for (bit, t) in [(0x40u32, 0x98de9cusize), (0x80, 0x98dea8), (0x100, 0x98deb4), (0x200, 0x98dec0), (0x400, 0x98decc)] {
            if v & bit != 0 {
                s.slot_a = t;
            }
        }
        let table = s.slot_a;
        exe::crt_qsort(table, 9, 1, 0x487720usize);
        let name: unsafe extern "C" fn(u32, usize, u32, u32) -> u32 = core::mem::transmute(0x5428a0usize);
        let mut k = 0usize;
        while read_u8(table + k) != 0xff {
            let attr = read_u8(table + k);
            name(attr as u32, 0xde1f64, 0x7d0, 1);
            heading(area, k as u32 + 3, 0xc, 8, attr as i8 as i32 as u32);
            k += 1;
        }
        value_heading(area);
    } else if v & 0x20 != 0 {
        weights = w.other.as_ptr();
        let area = head(weights);
        first_three(area);
        for (col, label, data) in [(3u32, 0x98e9e0usize, 0x1000u32), (4, 0x98e9dc, 0x4000), (5, 0x98e9d4, 0x8000), (6, 0x98e9cc, 0x10000),
                                   (7, 0x98e9f8, 0x80), (8, 0x98e9f0, 0x100), (9, 0x98e9e8, 0x200)] {
            lang(0xde1f64, label);
            heading(area, col, 0xc, 6, data);
        }
        value_heading(area);
    } else {
        // no view of its own: the rows go to the last area made (ebp in the original)
        return s.last_area;
    }
    tel26_add_area(d(), [0x6e, (row_h + top + 8) as u32, 0x30c, (bottom - 8) as u32, 0xd, weights as u32, rows.wrapping_sub(1) as u32, 0, 1, 0, u32::MAX])
}

/// The rows, one per player of the list (for a match, only the ones selected inside the eleven plus the substitutes
/// with no provisional number; only players with a player record), in the area `area`.
///
/// Each player's training caught up; column 0: squad numbers mode, his number (none: an empty item 6, link 9 to him; held
/// for good: 0x46, link 5 with the number; provisional: an item 1 in the greyed colour); European mode, his registration
/// "In" (1: a button, link 0xa; 2: a button when rule byte 0xb is 3 and fewer than 2 reserves, else a text), "Out" (4: a
/// button while registered - reserves + 0xd is below the squad size asked, else a text), "Ine" (8 or 0x10, a text), else
/// an empty button under the same condition or an empty text. Column 1 the status (`squad_list_status`), column 2 the name
/// (white for a player of a club the human runs: link 0x7dc on a national team's screen, else 0x7d7; a person made in the
/// game in the greyed colour; on loan or with a contract of type 5 at the club, 0xae3152; else 0xae3184). Then the
/// Traditional view's sort cell (two per row) or the view's cells and the value in column 0xc (a button, link 0xd, for a
/// database player whose club the human runs).
unsafe fn squad_numbers_build_rows(s: &mut SquadNumbersScreen, area: u32) {
    let list = s.list.as_mut_ptr() as usize;
    let club = s.club;
    let traditional = s.view & 1 != 0;
    let rules = SquadListRules { club, sort: s.sort, view_flags: s.view, category: s.category, table: s.slot_a, squad_screen: true };
    let get_current: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d2e0usize);
    let (mut col, mut row) = (0u32, 0u32);
    let mut buf = [0u8; 0xc8];
    let b = buf.as_mut_ptr() as usize;
    let (cbe04, c3184, c317e, cbdec, cbdcc) = (part_exchange_colour(0xaebe04), part_exchange_colour(0xae3184), part_exchange_colour(0xae317e),
        part_exchange_colour(0xaebdec), part_exchange_colour(0xaebdcc));
    let lang = part_exchange_lang();
    part_exchange_rewind(list);
    while get_current(list) != 0 {
        let p = read_usize(exe::llist_get_next(list));
        if s.mode == 1 && s.fixture != 0 {
            let slot = exe::tactics_find_selection_slot(read_usize(G_TACTICS), club, p, s.side) as i8;
            if slot < 0 || slot as i32 >= (read_u8(s.fixture + 0x4d) & 0xf) as i32 + 0xb || squad_numbers_number(p, club) < 0 {
                continue;
            }
        }
        if read_usize(p + staff::PLAYER) == 0 {
            continue;
        }
        exe::training_manager_update_player_training(read_usize(G_TRAINING_MANAGER), p, 0);
        let item = |f: [u32; 18]| {
            tel26_add_item(screens_current_display(G_SCREEN_MANAGER), f);
        };
        if s.mode == 1 {
            let n = squad_numbers_number(p, club);
            if n == 0 {
                item([6, 0, 0, 0, 0, col, row, 0x30, cbe04, cbe04, 0xc, 1, c3184, 0xad9c64, 0, 9, p as u32, area]);
            } else {
                part_exchange_sprintf()(0xde1f64, 0x98806c, (n as i32).wrapping_abs()); // "%d"
                if n > 0 {
                    item([0x46, 0, 0, 0, 0, col, row, 0x30, cbe04, cbe04, 0xc, 1, c3184, 0xde1f64, 0, 5, n as i32 as u32, area]);
                } else {
                    item([1, 0, 0, 0, 0, col, row, 0x30, cbe04, cbe04, 0xc, 1, c317e, 0xde1f64, 0, 0, 0, area]);
                }
            }
        } else {
            let open = (s.registered as i8 as i32 - s.reserves as i8 as i32 + s.rule_d as i8 as i32) < s.squad_size as i8 as i32;
            let button = |colour: u32, text: u32| {
                tel26_area_button([area, col, row, 0x30, cbe04, 0xc, 1, colour, text, 0xa, p as u32]);
            };
            let text = |a5: u32, colour: u32, text: u32| {
                tel26_area_text([area, col, row, 0x30, a5, 0xc, 1, colour, text]);
            };
            match read_u8(p + staff::SQUAD_SELECTED_FOR) {
                1 => {
                    lang(0xde1f64, 0x9910f8, 0xad9c64usize); // "In"
                    button(cbdec, 0xde1f64);
                }
                2 => {
                    lang(0xde1f64, 0x9910f8, 0xad9c64usize); // "In"
                    if s.rule_b == 3 && (s.reserves as i8) < 2 {
                        button(c3184, 0xde1f64);
                    } else {
                        text(cbe04, c3184, 0xde1f64);
                    }
                }
                4 => {
                    lang(0xde1f64, 0x9910c8, 0xad9c64usize); // "Out"
                    if open {
                        button(c317e, 0xde1f64);
                    } else {
                        text(cbe04, c317e, 0xde1f64);
                    }
                }
                8 | 0x10 => {
                    lang(0xde1f64, 0x98e994); // "Ine"
                    text(cbdcc, c3184, 0xde1f64);
                }
                _ => {
                    if open {
                        button(c3184, 0);
                    } else {
                        text(cbe04, c3184, 0);
                    }
                }
            }
        }
        squad_list_status(&rules, area, p, col, row);
        let new_person = p != 0 && read_i32(p) >= read_i32(G_STAFF_COUNT);
        let contract_club = if new_person { 0 } else { exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), p, club) };
        let contract = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), p);
        exe::database_format_staff_name(p, 0xde1f64usize, 0x7d0, if traditional { 4 } else { 1 });
        let national = || exe::database_is_national_team(club) != 0;
        let controls = |c: usize| c != 0 && exe::human_manager_current_controls_club(c, 0, 1) != 0;
        let (colour, link) = if national() && controls(read_usize(p + staff::CLUB_JOB)) {
            (cbdec, 0x7dc)
        } else if !national() && controls(read_usize(p + staff::NATIONAL_JOB)) {
            (cbdec, 0x7d7)
        } else if national() && new_person {
            (c317e, 0x7dc)
        } else if !national() && new_person {
            (c317e, 0x7d7)
        } else if national() {
            (c3184, 0x7dc)
        } else if (contract_club != 0 && read_u8(contract_club + contract::CONTRACT_TYPE) & 0x3f == 5)
            || exe::contract_manager_get_loan_contract(read_usize(G_CONTRACT_MANAGER), p) != 0 {
            (part_exchange_colour(0xae3152), 0x7d7)
        } else {
            (c3184, 0x7d7)
        };
        tel26_area_button([area, col + 2, row, 1, 0, 1, if traditional { 3 } else { 2 }, colour, 0xde1f64, link, p as u32]);
        let injury = exe::injury_get_player_injury_status(read_usize(G_INJURY), p);
        if traditional {
            squad_list_sort_cell(&rules, area, p, col, row, injury, contract_club, contract, b);
            col += 4;
            if col > 4 {
                row += 1;
                col = 0;
            }
        } else {
            squad_list_view_cells(&rules, area, p, col, row, injury, contract_club, contract, b);
            part_exchange_value(p);
            let (c3192, club_job) = (part_exchange_colour(0xae3192), read_usize(p + staff::CLUB_JOB));
            if !new_person && club_job != 0 && exe::human_manager_current_controls_club(club_job, 0, 1) != 0 {
                tel26_area_button([area, col + 0xc, row, 0x30, c3192, 0xc, 1, c3184, 0xde1f64, 0xd, p as u32]);
            } else {
                tel26_area_text([area, col + 0xc, row, 0x30, c3192, 0xc, 1, c3184, 0xde1f64]);
            }
            row += 1;
        }
    }
}

/// Comparer for `squad_numbers_screen_build`.
///
/// Through the display list (`compare_by_display_list_with`), plus [0xae1080], the attribute tables sorted in place and
/// the squad copy (param 0x11, 200 bytes). Without a copy the original allocates one and writes it to param 0x11: only
/// the original runs, and the copy is proven in Rust mode. What it also does outside the list runs twice and is
/// idempotent: the background picture, each player's training caught up to the week.
pub(crate) unsafe extern "C" fn compare_squad_numbers_screen_build() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I0%%);
    let squad = get_screen_param(0x11);
    if squad == 0 {
        return o();
    }
    let extra = [(G_CONTRACT_LOOKUP_RESULT, 4usize), (0x98de84usize, 0x54usize), (squad, 200)];
    compare_by_display_list_with("squad_numbers_screen_build (squad numbers screen)", &extra, || o(), || squad_numbers_screen_build());
}
