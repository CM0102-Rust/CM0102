// ---------- batch %%LEVA%%: the club screens (3): the club screen's squad tab (club_screens.cpp) ----------

/// The squad tab's state: what the original keeps in registers and stack slots across its pieces.
struct SquadTab {
    sm: usize,
    club: usize,
    reserves: u32,
    last_match: usize,
    next_match: usize,
    view: u32,
    sort: u32,
    category: u8,
    filter: u32,
    p19: u8,
    p1a: u8,
    best: [u32; 3],
    human: usize,
    top: i32,
    bottom: i32,
    bar: u32,
    title: [u8; 0xc8],
    list: [u8; 0x20],
    from_record: bool,
    attributes: usize,
    /// the area the original keeps in ebp (the rows go there when the view has none of its own)
    ebp: u32,
}

/// A colour word of the interface palette, zero-extended (the original pushes it in a register whose high half is
/// whatever the register held: garbage that `Display` keeps out, since it stores the colours as words).
unsafe fn squad_colour(a: usize) -> u32 {
    read_u16(a) as u32
}

impl SquadTab {
    unsafe fn display(&self) -> usize {
        screens_current_display(self.sm)
    }
    unsafe fn add_area(&self, a: [u32; 11]) -> u32 {
        let f: DisplayAddAreaFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
        f(self.display(), a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7], a[8], a[9], a[10])
    }
    unsafe fn add_item(&self, a: [u32; 18]) -> u32 {
        let f: DisplayAddItemFn = core::mem::transmute(FN_DISPLAY_ADD_ITEM);
        f(self.display(), a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7], a[8], a[9], a[10], a[11], a[12], a[13], a[14], a[15], a[16], a[17])
    }
    unsafe fn text(&self, area: u32, col: i32, row: i32, a4: u32, a5: u32, a6: u32, a7: u32, colour: u32, text: usize) {
        let f: ListCellTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
        f(self.sm, area as usize, col as usize, row as usize, a4 as usize, a5 as usize, a6 as usize, a7 as usize, colour as usize, text);
    }
    unsafe fn button(&self, area: u32, col: i32, row: i32, a4: u32, a5: u32, a6: u32, a7: u32, colour: u32, text: usize, link: u32, data: usize) -> u32 {
        let f: ListCellFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_BUTTON);
        f(self.sm, area as usize, col as usize, row as usize, a4 as usize, a5 as usize, a6 as usize, a7 as usize, colour as usize, text, link as usize, data) as u32
    }
    /// A popup menu's item (ticked 0x2002 or plain 2) with the text at 0xde1f64.
    unsafe fn menu_item(&self, area: u32, ticked: bool, link: u32, data: u32) {
        self.add_item([if ticked { 0x2002 } else { 2 }, 0, 0, 0, 0, 0, 0, 0x10, squad_colour(0xae31aa), squad_colour(0xaebdec), 1, 1,
            squad_colour(0xaebdea), 0xde1f64, 0, link, data, area]);
    }
    /// A popup menu's separator line (flags 1, 0x1000010).
    unsafe fn separator(&self, area: u32) {
        self.add_item([1, 0, 0, 0, 0, 0, 0, 0x1000010, squad_colour(0xae31aa), squad_colour(0xaebdec), 1, 1, squad_colour(0xaebdea), 0, 0, 0, 0, area]);
    }
    /// A menu button in a cell of `area` and its popup area.
    unsafe fn popup_button(&self, area: u32, col: i32, label: usize) -> u32 {
        squad_text(0xde1f64, label);
        let b = self.button(area, col, 0, 0x30, squad_colour(0xae317e), 0xc, 1, squad_colour(0xae3184), 0xde1f64, 0, 0);
        self.add_area([0, 0, 0, 0, 0, 0, 0, 0, 0x130, squad_colour(0xae31aa), b])
    }
    /// A column heading button (font `style`, link `link`, data `data`) in row 0 of `area`.
    unsafe fn heading(&self, area: u32, col: i32, style: u32, label: usize, link: u32, data: u32) {
        if label != 0 {
            squad_text(0xde1f64, label);
        }
        self.button(area, col, 0, 0x30, squad_colour(0xae317e), style, 1, squad_colour(0xae3184), 0xde1f64, link, data as usize);
    }
    /// The title text: the ticked item's text without its first space.
    unsafe fn ticked_title(&mut self) {
        copy_text(self.title.as_mut_ptr(), 0xde1f65 as *const u8);
    }
}

/// The game's text formatter (`FN_LANGLIB_SPRINTF`) with one string or a few arguments.
unsafe fn squad_text(out: usize, format: usize) -> i32 {
    let f: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    f(out, format, 0xad9c64usize)
}

/// sprintf (`FN_CRT_SPRINTF`) of one integer into 0xde1f64.
unsafe fn squad_number_text(format: usize, v: i32) {
    let f: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    f(0xde1f64, format, v);
}

/// sprintf (`FN_CRT_SPRINTF`) of one text into 0xde1f64.
unsafe fn squad_string_text(format: usize, s: usize) {
    let f: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    f(0xde1f64, format, s);
}

/// The PlayerStats object ([0xb74350]) and its methods used by the squad tab.
unsafe fn squad_stat(pdata: usize, category: u32, field: u32, out: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x7c25b0usize);
    f(read_usize(0xb74350), pdata, category, field, out, 0x7d0, 1, 0, 1);
}

/// `PlayerStats::format_recent_ratings` (0x7c1660, thiscall, ret 0xc) into 0xde1f64.
unsafe fn squad_recent_ratings(pdata: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x7c1660usize);
    f(read_usize(0xb74350), pdata, 0xde1f64, 0x7d0);
}

/// `contract_manager_write_morale_text` (0x4cc2e0, ret 0xc) into 0xde1f64 (called with ecx = [0xae1050]).
unsafe fn squad_morale(player: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x4cc2e0usize);
    f(read_usize(G_CONTRACT_MANAGER), player, 0xde1f64, 0x7d0);
}

/// `formation_get_position_abbreviation(position, 0xde1f64, 0x7d0, 0)` (0x5b7810, cdecl).
unsafe fn squad_position_text(position: u32) {
    let f: unsafe extern "C" fn(u32, usize, u32, u32) -> i32 = core::mem::transmute(0x5b7810usize);
    f(position, 0xde1f64, 0x7d0, 0);
}

/// `database_format_player_positions(pdata, out, size, style)` (0x53ee50, cdecl).
unsafe fn squad_positions(pdata: usize, out: usize, size: u32) {
    let f: unsafe extern "C" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x53ee50usize);
    f(pdata, out, size, 1);
}

/// `STAFF_CONTRACT::format_wage_text(out, style, flags)` (0x85e750, thiscall, ret 0xc).
unsafe fn squad_wage(contract: usize, out: usize, flags: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(0x85e750usize);
    f(contract, out, 0, flags);
}

/// `STAFF_CONTRACT::format_end_date_text(out)` (0x85e8f0, thiscall, ret 4).
unsafe fn squad_end_date(contract: usize, out: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x85e8f0usize);
    f(contract, out);
}

/// A person whose id is at or past the database's person count ([0xae2364]: one made in the game, a regen).
unsafe fn squad_is_new_person(player: usize) -> bool {
    player != 0 && read_i32(player) >= read_i32(0xae2364)
}

/// The view's column weights (13 columns) and the positions view's (8, or what the 1080 exe puts there).
struct SquadWeights {
    two: [u8; 8],
    contract: [u8; 13],
    stats: [u8; 13],
    attributes: [u8; 13],
    other: [u8; 13],
    selection: [u8; 13],
}

/// Rule: the club screen's squad tab lists the club's players in the chosen view (positions in two columns, contract,
/// selection, statistics, attributes or other information), sorted and filtered as the menus say, with each player's
/// squad number or selection slot, his status, his name (starred among the club's three most reputed players for a
/// club no human runs) and the view's columns.
///
/// `club_screen_draw_squad_tab(top, bottom)`, from the screen parameters 0 (club), 1 (reserves), 2 and 3 (the last and
/// next fixture), 7 (the view flags), 8 (the sort flags), 9 (the statistics category), 0x18 (the filter flags), 0x19,
/// 0x1a, 0x20..0x22 (the three best known players' ids) and 0xa (the scroll row). The pieces:
/// `club_screen_squad_tab_menus` (the View, Sort By, Competitions, Attributes and Filter menus, the clear button and the
/// title), `club_screen_squad_tab_selection` (the squad numbers or the team sheet row, and the list), the header and the
/// list area (`club_screen_squad_tab_header`) and the rows (`club_screen_squad_tab_rows`). Last, the list area's
/// scroll row from param 0xa (`screenmanager_set_area_scroll_row`) and the area written to param 0xb.
///
/// The cm0102-1080.exe changes this routine: the positions view's area gets 12 columns with the weights at 0x601eeb
/// and three players per row. The copy reads those bytes from the exe (0x459a22, 0x459a3b, 0x45c018), so it builds what
/// each exe builds.
/// Original: `club_screen_draw_squad_tab` (0x456900, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn club_screen_draw_squad_tab(top: i32, bottom: i32) {
    covered!("tel01_3 squad tab");
    let sm = G_SCREEN_MANAGER;
    let mut t = SquadTab {
        sm,
        club: 0,
        reserves: 0,
        last_match: 0,
        next_match: 0,
        view: 0,
        sort: 0,
        category: 0,
        filter: 0,
        p19: 0,
        p1a: 0,
        best: [0; 3],
        human: 0,
        top,
        bottom,
        bar: 0,
        title: [0; 0xc8],
        list: [0; 0x20],
        from_record: false,
        attributes: 0,
        ebp: 0,
    };
    club_screens_llist(&mut t.list, true);
    let mut w = SquadWeights {
        two: [6, 5, 0x1e, 0x10, 6, 5, 0x1e, 0x10],
        contract: [5, 5, 0x18, 0xe, 0xe, 0xf, 0xb, 0, 0, 0, 0, 0, 0xa],
        stats: [5, 5, 0x18, 6, 6, 6, 6, 6, 6, 6, 6, 6, 0xa],
        attributes: [5, 5, 0x18, 6, 6, 6, 6, 6, 6, 6, 6, 6, 0xa],
        other: [5, 5, 0x18, 8, 6, 6, 6, 0xa, 0xc, 6, 0, 0, 0xa],
        selection: [5, 5, 0x18, 0xe, 0xa, 0xc, 6, 6, 6, 0, 0, 0, 0xa],
    };
    t.human = read_usize(sm + 0x7d6 + read_i16(sm + screen_manager::CURRENT_SLOT) as isize as usize * 0x300);
    t.club = get_screen_param(0);
    t.reserves = get_screen_param(1) as u32;
    t.last_match = get_screen_param(2);
    t.next_match = get_screen_param(3);
    t.view = get_screen_param(7) as u32;
    t.sort = get_screen_param(8) as u32;
    t.category = get_screen_param(9) as u8;
    t.filter = get_screen_param(0x18) as u32;
    t.p19 = get_screen_param(0x19) as u8;
    t.p1a = get_screen_param(0x1a) as u8;
    t.best = [get_screen_param(0x20) as u32, get_screen_param(0x21) as u32, get_screen_param(0x22) as u32];
    if exe::human_manager_current_controls_club(t.club, 0, 1) != 0 {
        let v = club_screens_squad_view_settings();
        write_u32(v, t.view);
        write_u32(v + 4, t.sort);
        write_u8(v + 0xc, t.category);
        write_u32(v + 8, t.filter);
        write_u8(v + 0xd, t.p19);
        write_u8(v + 0xe, t.p1a);
        exe::human_manager_select_tactic_slot(read_usize(0xb63c98), t.reserves as i32, t.club, 0usize);
    }
    club_screen_squad_tab_menus(&mut t);
    club_screen_squad_tab_selection(&mut t);
    let area = club_screen_squad_tab_header(&mut t, &mut w);
    club_screen_squad_tab_rows(&mut t, area);
    let scroll: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(0x4567a0usize);
    scroll(sm, area, get_screen_param(0xa) as u32);
    set_screen_param(0xb, area as i16 as i32 as u32 as usize);
    club_screens_llist(&mut t.list, false);
}

/// The menus of the squad tab, in the original's order.
///
/// The view bar area (0x6e..0x168 by 0x14) with the "View" button and its popup (link 7, the item of a set bit ticked):
/// Traditional 1 (title "Position(s)"), Contract 2 ("Contract Info"), Selection 0x1000 ("Selection Info"), Stats 4, More
/// Stats 8, Attributes 0x10, Other 0x20 ("Other Info"), and for a club the human runs a separator and Penalty Takers
/// 0x800. In the positions view, "Sort By" (link 0xc, the ticked one's text, without the first space, as the title):
/// Name 4, Position(s) 8, Squad Number 0x20, Form 0x80, Morale 0x100, Condition 0x200, Nationality (Club for a national
/// or all-star team) 0x2000, Age 0x4000, Int. Caps 0x8000, Int. Goals 0x10000, Basic Wage 0x20000, Contract Expiry
/// 0x40000, Contract Protected 0x20000000 (only with [0xae233c] = 1), Value 0x80000, Goals 0x100000, Conceded 0x200000,
/// Assists 0x400000, Av. Rating 0x800000. With Stats or More Stats, "Competitions" (link 9): the six categories
/// (`player_stats_get_category_name`), the one of param 9 ticked with "<category> Stats" as the title. With
/// Attributes, "Attributes" (link 7): Physical 0x40, Mental 0x80, Goalkeeping 0x100, Defensive 0x200, Attacking 0x400,
/// each ticked with its "... Attributes" title. Then the filter area (0x212..0x30c): for a club the human runs, "Undo
/// Clear" (when there is an undo copy of the selection, `Tactics::has_selection_undo`, and no penalty takers view, data
/// 1) or "Clear Squad" (data 0), link 0x25; "Filter" (link 8) with All Positions 1, Goalkeepers 2, Defenders 4,
/// Midfielders 8, Attackers 0x10, a separator, All Sides 0x20, Left Sided 0x40, Central 0x80, Right Sided 0x100, a
/// separator, Available Only (bit 0x1000: ticked with data 0x800, plain with 0x1000), Unavailable (0x2000: 0x800 or
/// 0x2000); for a club (not a national team) of a European nation (nation +0x71 continent id = [0x9cfa10]) with a
/// European competition (+0x1db, its own or its B team's, `club_get_linked_b_team`) a separator and European Squad
/// (0x8000: 0x4000 or 0x8000); with a reserve squad (`exe::database_club_has_reserve_squad`): for a club the human runs
/// (not a national team) a separator and Include Reserves (Include Seniors on the reserves; 0x400: 0x200 or 0x400),
/// otherwise bit 0x400 dropped from the filter. The title (font 3) under the bars; the top moves down 0x41. For a
/// national team the human runs, "<n> players selected (maximum <m>)" (`exe::database_count_club_squad` with 4, or 8
/// for the B squad; `NationalTeams::get_max_squad_size`) at the bottom, which moves up 0x19.
unsafe fn club_screen_squad_tab_menus(t: &mut SquadTab) {
    let sm = t.sm;
    t.bar = t.add_area([0x6e, t.top as u32, 0x168, (t.top + 0x14) as u32, 2, 0, 1, 0, 1, 0, u32::MAX]);
    t.title[0] = 0;
    let view_menu = t.popup_button(t.bar, 0, 0x988004); // "View"
    let v = t.view;
    for (label, bit, title) in [(0x98eedcusize, 1u32, 0x98eed0usize), (0x98eec4, 2, 0x98eeb4), (0x98ee88, 0x1000, 0x98ee78), (0x98ee70, 4, 0),
                                (0x98ee64, 8, 0), (0x98ee58, 0x10, 0), (0x98ee30, 0x20, 0x98ee24)] {
        squad_text(0xde1f64, label);
        let ticked = v & bit != 0;
        if ticked && title != 0 {
            squad_text(t.title.as_mut_ptr() as usize, title);
        }
        t.menu_item(view_menu, ticked, 7, bit);
    }
    if exe::human_manager_current_controls_club(t.club, 0, 1) != 0 {
        t.separator(view_menu);
        squad_text(0xde1f64, 0x98ee14); // " Penalty Takers"
        t.menu_item(view_menu, v & 0x800 != 0, 7, 0x800);
    }
    if v & 1 != 0 {
        covered!("tel01_3 sort menu");
        let sort_menu = t.popup_button(t.bar, 1, 0x98ee0c); // "Sort By"
        let club_label = if exe::nationalteams_is_national_or_all_star_team(read_usize(G_NATIONAL_TEAMS), t.club) == 0 { 0x98edb8 } else { 0x98edb0 };
        let mut items: [(usize, u32); 18] = [(0x98ee04, 4), (0x98edf4, 8), (0x98ede4, 0x20), (0x98eddc, 0x80), (0x98edd4, 0x100), (0x98edc8, 0x200),
            (club_label, 0x2000), (0x98eda8, 0x4000), (0x98ed9c, 0x8000), (0x98ed90, 0x10000), (0x98ed84, 0x20000), (0x98ed70, 0x40000),
            (0x98ed5c, 0x20000000), (0x98ed54, 0x80000), (0x98ed4c, 0x100000), (0x98ed40, 0x200000), (0x98ed34, 0x400000), (0x98ed28, 0x800000)];
        if read_u8(0xae233c) != 1 {
            items[12].0 = 0; // without the protection rule, no "Contract Protected"
        }
        for (label, bit) in items {
            if label == 0 {
                continue;
            }
            squad_text(0xde1f64, label);
            let ticked = t.sort & bit != 0;
            t.menu_item(sort_menu, ticked, 0xc, bit);
            if ticked {
                t.ticked_title();
            }
        }
    }
    if v & 4 != 0 || v & 8 != 0 {
        let comp_menu = t.popup_button(t.bar, 1, 0x98ed18); // "Competitions"
        let name: unsafe extern "thiscall" fn(usize, u32, *mut u8, u32) -> u32 = core::mem::transmute(0x7c14b0usize);
        let mut buf = [0u8; 0xc8];
        for cat in 1u8..=6 {
            name(read_usize(0xb74350), cat as u32, buf.as_mut_ptr(), 0xc8);
            squad_string_text(0x98ed14, buf.as_ptr() as usize); // " %s"
            if cat == t.category {
                let f: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
                f(t.title.as_mut_ptr() as usize, 0x98ecf4, buf.as_ptr());
                t.menu_item(comp_menu, true, 9, cat as u32);
            } else {
                t.menu_item(comp_menu, false, 9, cat as u32);
            }
        }
    }
    if v & 0x10 != 0 {
        let attr_menu = t.popup_button(t.bar, 1, 0x98ece8); // "Attributes"
        for (label, bit, title) in [(0x98ecdcusize, 0x40u32, 0x98ecc8usize), (0x98ecc0, 0x80, 0x98ecac), (0x98eca0, 0x100, 0x98ec88),
                                    (0x98ec7c, 0x200, 0x98ec64), (0x98ec58, 0x400, 0x98ec40)] {
            squad_text(0xde1f64, label);
            let ticked = v & bit != 0;
            if ticked {
                squad_text(t.title.as_mut_ptr() as usize, title);
            }
            t.menu_item(attr_menu, ticked, 7, bit);
        }
    }
    let filter_area = t.add_area([0x212, t.top as u32, 0x30c, (t.top + 0x14) as u32, 2, 0, 1, 0, 1, 0, u32::MAX]);
    t.ebp = filter_area;
    if exe::human_manager_current_controls_club(t.club, 0, 1) != 0 {
        let undo: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x897250usize);
        if v & 0x800 == 0 && undo(read_usize(0xde1d08), t.club, t.reserves) != 0 {
            squad_text(0xde1f64, 0x98ec0c); // "Undo Clear"
            t.button(filter_area, 0, 0, 0x30, squad_colour(0xae317e), 0xc, 1, squad_colour(0xae3184), 0xde1f64, 0x25, 1);
        } else {
            squad_text(0xde1f64, 0x98ec00); // "Clear Squad"
            t.button(filter_area, 0, 0, 0x30, squad_colour(0xae317e), 0xc, 1, squad_colour(0xae3184), 0xde1f64, 0x25, 0);
        }
    }
    let filter_menu = t.popup_button(filter_area, 1, 0x98ebf8); // "Filter"
    let mut f = t.filter;
    for (label, bit) in [(0x98ebe8usize, 1u32), (0x98ebd8, 2), (0x98ebcc, 4), (0x98ebbc, 8), (0x98ebb0, 0x10)] {
        squad_text(0xde1f64, label);
        t.menu_item(filter_menu, f & bit != 0, 8, bit);
    }
    t.separator(filter_menu);
    for (label, bit) in [(0x98eba4usize, 0x20u32), (0x98eb98, 0x40), (0x98eb8c, 0x80), (0x98eb7c, 0x100)] {
        squad_text(0xde1f64, label);
        t.menu_item(filter_menu, f & bit != 0, 8, bit);
    }
    t.separator(filter_menu);
    // the toggles: ticked they carry 0x800 (or 0x4000, 0x200), plain their own bit
    squad_text(0xde1f64, 0x98eb6c); // " Available Only"
    t.menu_item(filter_menu, f & 0x1000 != 0, 8, if f & 0x1000 != 0 { 0x800 } else { 0x1000 });
    squad_text(0xde1f64, 0x98eb40); // " Unavailable"
    t.menu_item(filter_menu, f & 0x2000 != 0, 8, if f & 0x2000 != 0 { 0x800 } else { 0x2000 });
    let club = t.club;
    if exe::database_is_national_team(club) == 0 {
        let nation = read_usize(club + 0x53);
        if nation != 0 && read_usize(nation + 0x71) != 0 && read_u32(read_usize(nation + 0x71)) == read_u32(0x9cfa10) {
            let european = if read_i32(club + 0x1db) != -1 {
                true
            } else {
                let b = exe::club_get_linked_b_team(club, core::ptr::null_mut(), 1);
                b != 0 && read_i32(b + 0x1db) != -1
            };
            if european {
                t.separator(filter_menu);
                squad_text(0xde1f64, 0x98eb30); // " European Squad"
                t.menu_item(filter_menu, f & 0x8000 != 0, 8, if f & 0x8000 != 0 { 0x4000 } else { 0x8000 });
            }
        }
    }
    if exe::database_club_has_reserve_squad(club) != 0 {
        if exe::database_is_national_team(club) != 0 {
            if f & 0x400 != 0 {
                f -= 0x400;
                t.filter = f;
            }
        } else if exe::human_manager_current_controls_club(club, 0, 1) != 0 {
            t.separator(filter_menu);
            squad_text(0xde1f64, if t.reserves != 0 { 0x98eb1c } else { 0x98eb08 });
            t.menu_item(filter_menu, f & 0x400 != 0, 8, if f & 0x400 != 0 { 0x200 } else { 0x400 });
        } else if f & 0x400 != 0 {
            f -= 0x400;
            t.filter = f;
        }
    }
    let manager_text: ManagerTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_TEXT_ITEM);
    t.top += 0x19;
    manager_text(sm, 0x6e, t.top as usize, 0x30c, (t.top + 0x23) as usize, 2, 0, 0xc, 3, squad_colour(0xaebdec) as usize, t.title.as_ptr() as usize, 1);
    t.top += 0x28;
    if exe::human_manager_current_controls_club(club, 0, 1) != 0 && exe::database_is_national_team(club) != 0 {
        let count = exe::database_count_club_squad(club, if t.reserves == 0 { 4 } else { 8 }, 0) as i8 as i32;
        let max: unsafe extern "thiscall" fn(usize, usize, u32) -> u8 = core::mem::transmute(0x7754c0usize);
        let m = max(read_usize(G_NATIONAL_TEAMS), club, t.reserves) as i8 as i32;
        let fmt: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
        if count == 1 {
            fmt(0xde1f64, 0x98ead4, m);
        } else {
            fmt(0xde1f64, 0x98ea88, count, m);
        }
        manager_text(sm, 0x6e, (t.bottom - 0x14) as usize, 0x30c, t.bottom as usize, 2, 0, 0xc, 2, squad_colour(0xaebdec) as usize, 0xde1f64, 1);
        t.bottom -= 0x19;
    }
}

/// The squad numbers or the team sheet row, then the player list.
///
/// For a club the human runs: his squad numbers pruned (`HUMAN_MANAGER::prune_squad_numbers`); in the penalty takers view
/// (0x800) two rows of 25 numbers (1..25 at the top, 26..50 at the bottom; the top moves down and the bottom up 0x19),
/// each white (0x70) when a player holds it (`exe::human_manager_get_squad_number_player`), link 0x23 with the number;
/// otherwise the team sheet row: the active tactic's eleven positions in order (`Formation::get_slots_in_position_order`)
/// and the substitutes "SB<n>" up to 20 slots, the ones past the fixture's allowed substitutes (+0x4d & 0xf, 9 without a
/// next fixture) greyed; each slot's selected player (`exe::tactics_get_selected_player`) marks it white, available
/// (`is_player_available_for_match` with the current date as the competition, as the original) in the text colour,
/// otherwise 0xae31a8; link 0xa with the slot; the top moves down 0x19. For a club he does not run, the penalty takers
/// view is dropped and, with the last fixture's match record (`exe::match_stats_read_match_record` into 0xad9db8), its
/// team sheet is used. Then the list (`build_filtered_sorted_squad_list`, 0x4841d0) with the club, reserves, sort, style
/// 3, params 9, 0x19, 0x1a, the filter, the record's team sheet or none, the next fixture and the penalty takers bit.
unsafe fn club_screen_squad_tab_selection(t: &mut SquadTab) {
    t.from_record = false;
    let club = t.club;
    let human_manager = read_usize(0xb63c98);
    if exe::human_manager_current_controls_club(club, 0, 1) != 0 {
        let prune: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x600ee0usize);
        prune(human_manager);
        if t.view & 0x800 != 0 {
            covered!("tel01_3 squad numbers");
            let grid = |t: &SquadTab, area: u32, k: u32, col: u32| {
                squad_number_text(0x98806c, k as i32 + 1);
                let held = exe::human_manager_get_squad_number_player(read_usize(0xb63c98), k as i32, 0usize) != 0;
                let (flags, colour) = if held { (0x70, squad_colour(0xaebdec)) } else { (0x30, squad_colour(0xae3184)) };
                t.add_item([0x46, 0, 0, 0, 0, col, 0, flags, squad_colour(0xae3192), squad_colour(0xae3192), 0xc, 1, colour, 0xde1f64, 0, 0x23, k, area]);
            };
            let a = t.add_area([0x6e, t.top as u32, 0x30c, (t.top + 0x14) as u32, 0x19, 0, 1, 0, 1, 0, u32::MAX]);
            for k in 0..0x19 {
                grid(t, a, k, k);
            }
            t.top += 0x19;
            let b = t.add_area([0x6e, (t.bottom - 0x14) as u32, 0x30c, t.bottom as u32, 0x19, 0, 1, 0, 1, 0, u32::MAX]);
            for k in 0x19..0x32 {
                grid(t, b, k, k - 0x19);
            }
            t.bottom -= 0x19;
            t.ebp = b;
        } else {
            covered!("tel01_3 team sheet row");
            let a = t.add_area([0x6e, t.top as u32, 0x30c, (t.top + 0x14) as u32, 0x14, 0, 1, 0, 1, 0, u32::MAX]);
            t.ebp = a;
            let tactic = exe::human_manager_get_active_tactic(human_manager, 0usize);
            let mut slots = [0u8; 0xc];
            let order: unsafe extern "thiscall" fn(usize, *mut u8, u32) = core::mem::transmute(0x5b31b0usize);
            order(tactic, slots.as_mut_ptr(), 0xb);
            let shown = if t.next_match != 0 { (read_u8(t.next_match + 0x4d) & 0xf).wrapping_add(0xb) } else { 0x14 } as i8;
            for k in 0u8..0x14 {
                if k < 0xb {
                    let pos = exe::formation_get_slot_position(tactic, slots[k as usize] as u32, 0);
                    squad_position_text(pos as u32);
                } else {
                    let f: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
                    f(0xde1f64, 0x98ea6c, k as i32 - 0xa);
                }
                let slot = if k < 0xb { slots[k as usize] } else { k };
                let player = exe::tactics_get_selected_player(read_usize(0xde1d08), club, slot as u32, t.reserves as i32);
                let style = if (k as i8) >= shown { 0x2c } else { 0xc };
                if player != 0 {
                    let available: unsafe extern "C" fn(usize, usize, usize, usize) -> u32 = core::mem::transmute(0x45cbe0usize);
                    let colour = if available(player, t.next_match, club, G_CURRENT_DATE) != 0 { squad_colour(0xaebdec) } else { squad_colour(0xae31a8) };
                    t.add_item([0x46, 0, 0, 0, 0, k as u32, 0, 0x70, squad_colour(0xae3150), squad_colour(0xae3150), style, 1, colour, 0xde1f64, 0, 0xa,
                        slot as i8 as i32 as u32, a]);
                } else {
                    t.add_item([0x46, 0, 0, 0, 0, k as u32, 0, 0x30, squad_colour(0xae3150), squad_colour(0xae3150), style, 1, squad_colour(0xae3184),
                        0xde1f64, 0, 0xa, slot as i8 as i32 as u32, a]);
                }
            }
            t.top += 0x19;
        }
    } else {
        if t.view & 0x800 != 0 {
            t.view -= 0x800;
        }
        if t.last_match != 0 && exe::match_stats_read_match_record(read_usize(0xb6b7c0), read_u32(t.last_match + 8), 0xad9db8usize) != 0 {
            covered!("tel01_3 team sheet of the last match");
            t.from_record = true;
        }
    }
    let build: unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, u32, u32, usize, usize, usize, u32, u32, u32) -> u32 =
        core::mem::transmute(0x4841d0usize);
    // simplification: params 9, 0x19 and 0x1a are bytes in stack slots whose upper bytes are garbage in the original
    // (the list takes them as chars)
    build(t.club, t.reserves, t.sort, 3, t.category as u32, t.p19 as u32, t.p1a as u32, t.filter, t.list.as_mut_ptr() as usize,
        if t.from_record { 0xad9db8 } else { 0 }, t.next_match, 0, 0, t.view & 0x800);
}

/// The list's area: the positions view's own, or the background, the column headings and the list of the other views.
///
/// Positions view (1): one area from the top to the bottom with 8 columns (two players per row; the 1080 exe 12, three
/// per row) and the rows that fit in font 3. Other views: the background area, then the rows that fit in font 2 and
/// their height (the height over the rows, rounded, `ftol`); the headings in an area from the top + 8, one row high, as
/// wide as the list (0x2f6 when the list has more players than rows, to leave the scroll bar), with the view's weights:
/// "Pkd" 1, "Inf" 2, "   Name" 4 (all link 0xc, the sort data) and the view's columns: contract (Squad Status or Club
/// 0x10, Basic Wage 0x20000, Contract Expiry 0x40000, Releases 0x2000000 when not a national team; a national team's
/// weights 0x12, 0x12, 0x12, 0); selection (Position 8, Form 0x80, Morale 0x100, Cond. 0x200, the fields 1 and 0x11
/// headings 0x10000000 and 0x800000); statistics (the fields of [0x98de84] or, More Stats, [0x98de90], link 0xd);
/// attributes (the table of the chosen attribute group, [0x98de9c] when none, sorted in place by qsort with 0x487720,
/// link 0xe; none chosen sets bit 0x40 in the local view); other (Nat. 0x1000, Age 0x4000, Caps 0x8000, Goals 0x10000,
/// Form, Morale, Cond.); "Value" 0x80000 in column 0xc. Then the list area below, to the bottom - 8.
unsafe fn club_screen_squad_tab_header(t: &mut SquadTab, w: &mut SquadWeights) -> u32 {
    let v = t.view;
    if v & 1 != 0 {
        covered!("tel01_3 positions view");
        let rows = ((t.bottom - t.top - 0x10) / exe::font_get_height(3)) as u8 as u32;
        // the 1080 exe puts its own weights (mov edx, imm32) and column count in this routine
        let weights = if *(0x459a22 as *const u8) == 0xba { core::ptr::read_unaligned(0x459a23 as *const u32) } else { w.two.as_ptr() as u32 };
        let cols = *(0x459a3b as *const u8) as u32;
        // simplification: the row count is a byte in a stack slot whose upper bytes are garbage in the original
        return t.add_area([0x6e, t.top as u32, 0x30c, t.bottom as u32, cols, weights, rows, 0, 2, 0, u32::MAX]);
    }
    let (top, bottom) = (t.top, t.bottom);
    t.add_area([0x6e, top as u32, 0x30c, bottom as u32, 1, 0, 1, 0, 2, 0, u32::MAX]);
    let height = bottom - top - 0x10;
    let rows = (height / exe::font_get_height(2)) as u8;
    let row_h = ftol(height as f64 / rows as u32 as f64 + read_f64(rdata::F64_0_5));
    let count = read_i16(t.list.as_ptr() as usize) as i32;
    let x2 = if count > rows as i32 - 1 { 0x2f6 } else { 0x30c };
    let weights: usize;
    let mut area;
    let head = |t: &SquadTab, weights: usize| t.add_area([0x6e, (top + 8) as u32, x2, (row_h + top + 7) as u32, 0xd, weights as u32, 1, 0, 1, 0, u32::MAX]);
    if v & 2 != 0 {
        covered!("tel01_3 contract view");
        if exe::database_is_national_team(t.club) != 0 {
            w.contract[3] = 0x12;
            w.contract[4] = 0x12;
            w.contract[5] = 0x12;
            w.contract[6] = 0;
        }
        weights = w.contract.as_ptr() as usize;
        area = head(t, weights);
        t.heading(area, 0, 0xc, 0x98ea68, 0xc, 1);
        t.heading(area, 1, 0xc, 0x98ea64, 0xc, 2);
        t.heading(area, 2, 1, 0x98ea5c, 0xc, 4);
        let label = if exe::nationalteams_is_national_or_all_star_team(read_usize(G_NATIONAL_TEAMS), t.club) == 0 { 0x98ea4c } else { 0x98ea44 };
        t.heading(area, 3, 1, label, 0xc, 0x10);
        t.heading(area, 4, 1, 0x98ea34, 0xc, 0x20000);
        t.heading(area, 5, 1, 0x98ea20, 0xc, 0x40000);
        if exe::database_is_national_team(t.club) == 0 {
            t.heading(area, 6, 1, 0x98ea14, 0xc, 0x2000000);
        }
        t.heading(area, 0xc, 0xc, 0x98ea0c, 0xc, 0x80000);
    } else if v & 0x1000 != 0 {
        covered!("tel01_3 selection view");
        weights = w.selection.as_ptr() as usize;
        area = head(t, weights);
        t.heading(area, 0, 0xc, 0x98ea68, 0xc, 1);
        t.heading(area, 1, 0xc, 0x98ea64, 0xc, 2);
        t.heading(area, 2, 1, 0x98ea5c, 0xc, 4);
        t.heading(area, 3, 1, 0x98ea00, 0xc, 8);
        t.heading(area, 4, 0xc, 0x98e9f8, 0xc, 0x80);
        t.heading(area, 5, 0xc, 0x98e9f0, 0xc, 0x100);
        t.heading(area, 6, 0xc, 0x98e9e8, 0xc, 0x200);
        let label: unsafe extern "thiscall" fn(usize, u32, usize, u32) = core::mem::transmute(0x7c1b20usize);
        label(read_usize(0xb74350), 1, 0xde1f64, 0x7d0);
        t.heading(area, 7, 0xc, 0, 0xc, 0x10000000);
        label(read_usize(0xb74350), 0x11, 0xde1f64, 0x7d0);
        t.heading(area, 8, 0xc, 0, 0xc, 0x800000);
        t.heading(area, 0xc, 0xc, 0x98ea0c, 0xc, 0x80000);
    } else if v & 4 != 0 || v & 8 != 0 {
        covered!("tel01_3 stats view");
        weights = w.stats.as_ptr() as usize;
        area = head(t, weights);
        t.heading(area, 0, 0xc, 0x98ea68, 0xc, 1);
        t.heading(area, 1, 0xc, 0x98ea64, 0xc, 2);
        t.heading(area, 2, 1, 0x98ea5c, 0xc, 4);
        let table = if v & 4 != 0 { 0x98de84 } else { 0x98de90 };
        let label: unsafe extern "thiscall" fn(usize, u32, usize, u32) = core::mem::transmute(0x7c1b20usize);
        let mut k = 0usize;
        while read_u8(table + k) != 0xff {
            let field = read_u8(table + k);
            label(read_usize(0xb74350), field as u32, 0xde1f64, 0x7d0);
            t.heading(area, k as i32 + 3, 0xc, 0, 0xd, field as i8 as i32 as u32);
            k += 1;
        }
        t.heading(area, 0xc, 0xc, 0x98ea0c, 0xc, 0x80000);
    } else if v & 0x10 != 0 {
        covered!("tel01_3 attributes view");
        weights = w.attributes.as_ptr() as usize;
        area = head(t, weights);
        t.heading(area, 0, 0xc, 0x98ea68, 0xc, 1);
        t.heading(area, 1, 0xc, 0x98ea64, 0xc, 2);
        t.heading(area, 2, 1, 0x98ea5c, 0xc, 4);
        t.attributes = if t.view & 0x40 != 0 {
            0x98de9c
        } else if t.view & 0x80 != 0 {
            0x98dea8
        } else if t.view & 0x100 != 0 {
            0x98deb4
        } else if t.view & 0x200 != 0 {
            0x98dec0
        } else if t.view & 0x400 != 0 {
            0x98decc
        } else {
            t.view |= 0x40;
            0x98de9c
        };
        exe::crt_qsort(t.attributes, 9, 1, 0x487720usize);
        let name: unsafe extern "C" fn(u32, usize, u32, u32) -> u32 = core::mem::transmute(0x5428a0usize);
        let mut k = 0usize;
        while read_u8(t.attributes + k) != 0xff {
            let attr = read_u8(t.attributes + k);
            name(attr as u32, 0xde1f64, 0x7d0, 1);
            t.heading(area, k as i32 + 3, 0xc, 0, 0xe, attr as i8 as i32 as u32);
            k += 1;
        }
        t.heading(area, 0xc, 0xc, 0x98ea0c, 0xc, 0x80000);
    } else if v & 0x20 != 0 {
        covered!("tel01_3 other view");
        weights = w.other.as_ptr() as usize;
        area = head(t, weights);
        t.heading(area, 0, 0xc, 0x98ea68, 0xc, 1);
        t.heading(area, 1, 0xc, 0x98ea64, 0xc, 2);
        t.heading(area, 2, 1, 0x98ea5c, 0xc, 4);
        for (col, label, data) in [(3, 0x98e9e0usize, 0x1000u32), (4, 0x98e9dc, 0x4000), (5, 0x98e9d4, 0x8000), (6, 0x98e9cc, 0x10000),
                                   (7, 0x98e9f8, 0x80), (8, 0x98e9f0, 0x100), (9, 0x98e9e8, 0x200)] {
            t.heading(area, col, 0xc, label, 0xc, data);
        }
        t.heading(area, 0xc, 0xc, 0x98ea0c, 0xc, 0x80000);
    } else {
        // no view of its own (the penalty takers alone): no list area; the rows go to the last area made (ebp)
        return t.ebp;
    }
    let _ = area;
    // the row count dword keeps the filter flags' upper bytes (ebx held them; only bl was the count)
    let rows_word = (t.filter & 0xffff_ff00) | rows.wrapping_sub(1) as u32;
    area = t.add_area([0x6e, (row_h + top + 8) as u32, 0x30c, (bottom - 8) as u32, 0xd, weights as u32, rows_word, 0, 1, 0, u32::MAX]);
    area
}

/// The player's value as money (`Cash::format_display_amount`, flags 5) when he has a club and a value (+0x52), else
/// "Free".
unsafe fn squad_value_text(player: usize) {
    let club_job = read_usize(player + staff::CLUB_JOB);
    if club_job != 0 && read_i32(player + 0x52) != 0 {
        club_screens_format_cash(read_i32(player + 0x52), 0xde1f64, 0x7d0, 5);
    } else {
        squad_text(0xde1f64, 0x98e614); // "Free"
    }
}

/// A contract's wage text into `buf` (style 0, flags 2; again with flags 1 when longer than 8 characters).
unsafe fn squad_wage_text(contract: usize, buf: &mut [u8; 0x7d0]) {
    squad_wage(contract, buf.as_mut_ptr() as usize, 2);
    if club_screens_text_length(buf.as_ptr()) > 8 {
        squad_wage(contract, buf.as_mut_ptr() as usize, 1);
    }
}

/// The squad tab's rows, one per player of the list with a player record (`staff::PLAYER`), in the area `area`.
///
/// Each player's training is caught up first (`exe::training_manager_update_player_training`). Column 0 (the
/// number): for a club the human runs, in the penalty takers view his squad number
/// (`HUMAN_MANAGER::get_player_squad_number`, link 0x23) or an empty item (link 0x24 with the player); otherwise his
/// selection slot (`exe::tactics_find_selection_slot`): the position of the active tactic's slot
/// (`formation_get_position_abbreviation`) or "SB<n>", link 0xa with the slot, or an empty item (link 0xb with the
/// player); for a club he does not run, the slot in the last match's team sheet (0xad9f48, the club's side by
/// [0xad9db8]: its position word at 0xad9f0c or "SB<n>") as a plain text. Column 1, his status
/// (`get_player_status_code`, 0x4849f0) as an item (flags 0x202, link and data by the status, for a national team other
/// links): Wpm, Cup, Ine (0x7e0 or 0x7db), international duty (with the duty club, link 0x7d5; without, an item 0x201
/// with no link; the text is whatever 0xde1f64 holds, as the original), Sus (0x7de or 0x7d9), Inj (0x7dd or 0x7d8),
/// Abs, Fgn, Tir, Yel (the colour of `exe::graphics_rgb_to_pixel`(0xb4, 0xb4, 0, 0)), Bid, Trn, Wnt, Sct, Req, Lst,
/// Loa, Ctr (0x7da), Ret, Bos, Unh, Fut (this last through `screenmanager_add_item`). Column 2, his name
/// (`exe::database_format_staff_name`, style 4 in the positions view, else 1; with "*" when he is one of the three best
/// known players of a club no human controls, `exe::human_manager_is_human_club`) as a button to him (link 0x7dc for a
/// national team, else 0x7d7), its colour: his club or nation run by the human (white), a person made in the game (the
/// greyed colour), on loan (0xae3152 at the club, else 0xaebdde), starred (0xae31a2), else 0xae3184. Then by the view:
/// in the positions view column 3 by the sort (squad number, form, morale, condition, nation or club, age, caps, goals,
/// basic wage, contract expiry, protection, value, goals, conceded, assists, rating, else the positions), two players
/// per row (three in the 1080 exe); in the others the view's columns and the value in column 0xc (a button to him for
/// a player of a club the human runs), one per row.
unsafe fn club_screen_squad_tab_rows(t: &mut SquadTab, area: u32) {
    let l = t.list.as_mut_ptr() as usize;
    let get_current: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d2e0usize);
    write_u32(l + 0xa, read_u32(l + 2));
    let (mut row, mut col) = (0i32, 0i32);
    let club = t.club;
    let positions_view = t.view & 1 != 0;
    let last_base = *(0x45c018 as *const u8) as i8 as i32; // 4, or 8 in the 1080 exe (three players per row)
    let mut buf = [0u8; 0x7d0];
    let b = buf.as_mut_ptr() as usize;
    if get_current(l) == 0 {
        return;
    }
    loop {
        let player = read_usize(exe::llist_get_next(l));
        let pdata = read_usize(player + staff::PLAYER);
        if pdata != 0 {
            exe::training_manager_update_player_training(read_usize(0xde1f44), player, 0);
            club_screen_squad_tab_number(t, area, player, col, row);
            club_screen_squad_tab_status(t, area, player, col, row);
            let contract_club = if squad_is_new_person(player) {
                0
            } else {
                exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), player, club)
            };
            let contract = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), player);
            let style = if positions_view { 3 } else { 2 };
            let mut starred = false;
            let name_style = if positions_view { 4 } else { 1 };
            if exe::human_manager_is_human_club(club, 0, 1, 0, 0) == 0 && t.best.contains(&read_u32(player)) {
                starred = true;
                exe::database_format_staff_name(player, b, 0x7d0, name_style);
                squad_string_text(0x98e8dc, b); // "%s*"
            } else {
                exe::database_format_staff_name(player, 0xde1f64usize, 0x7d0, name_style);
            }
            let national = exe::database_is_national_team(club) != 0;
            let club_job = read_usize(player + staff::CLUB_JOB);
            let national_job = read_usize(player + staff::NATIONAL_JOB);
            let (colour, link) = if national && club_job != 0 && exe::human_manager_current_controls_club(club_job, 0, 1) != 0 {
                (squad_colour(0xaebdec), 0x7dc)
            } else if !national && national_job != 0 && exe::human_manager_current_controls_club(national_job, 0, 1) != 0 {
                (squad_colour(0xaebdec), 0x7d7)
            } else if national && squad_is_new_person(player) {
                (squad_colour(0xae317e), 0x7dc)
            } else if !national && squad_is_new_person(player) {
                (squad_colour(0xae317e), 0x7d7)
            } else if national {
                (if starred { squad_colour(0xae31a2) } else { squad_colour(0xae3184) }, 0x7dc)
            } else if (contract_club != 0 && read_u8(contract_club + 0x35) & 0x3f == 5)
                || exe::contract_manager_get_loan_contract(read_usize(G_CONTRACT_MANAGER), player) != 0 {
                (if club_job == club { squad_colour(0xae3152) } else { squad_colour(0xaebdde) }, 0x7d7)
            } else {
                (if starred { squad_colour(0xae31a2) } else { squad_colour(0xae3184) }, 0x7d7)
            };
            t.button(area, col + 2, row, 1, 0, 1, style, colour, 0xde1f64, link, player);
            let injury = exe::injury_get_player_injury_status(read_usize(G_INJURY), player);
            if positions_view {
                club_screen_squad_tab_sort_cell(t, area, player, col, row, injury, contract_club, contract, &mut buf);
                col += 4;
                if col > last_base {
                    row += 1;
                    col = 0;
                }
            } else {
                club_screen_squad_tab_view_cells(t, area, player, col, row, injury, contract_club, contract, &mut buf);
                squad_value_text(player);
                if !squad_is_new_person(player) && club_job != 0 && exe::human_manager_current_controls_club(club_job, 0, 1) != 0 {
                    t.button(area, col + 0xc, row, 0x30, squad_colour(0xae3192), 0xc, 1, squad_colour(0xae3184), 0xde1f64, 0xf, player);
                } else {
                    t.text(area, col + 0xc, row, 0x30, squad_colour(0xae3192), 0xc, 1, squad_colour(0xae3184), 0xde1f64);
                }
                row += 1;
            }
        }
        if get_current(l) == 0 {
            break;
        }
    }
}

/// Column 0 of a row: the squad number, the selection slot or the last match's slot (see `club_screen_squad_tab_rows`).
unsafe fn club_screen_squad_tab_number(t: &mut SquadTab, area: u32, player: usize, col: i32, row: i32) {
    let club = t.club;
    let sb: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let (cbe04, c3184) = (squad_colour(0xaebe04), squad_colour(0xae3184));
    if exe::human_manager_current_controls_club(club, 0, 1) != 0 {
        if t.view & 0x800 != 0 {
            let number: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(0x601880usize);
            let n = number(read_usize(0xb63c98), player, 0) as i8;
            if n > -1 {
                squad_number_text(0x98806c, n as i32 + 1);
                t.add_item([0x46, 0, 0, 0, 0, col as u32, row as u32, 0x30, cbe04, cbe04, 0xc, 1, c3184, 0xde1f64, 0, 0x23, n as i32 as u32, area]);
            } else {
                t.add_item([6, 0, 0, 0, 0, col as u32, row as u32, 0x30, cbe04, cbe04, 0xc, 1, c3184, 0xad9c64, 0, 0x24, player as u32, area]);
            }
        } else {
            let slot = exe::tactics_find_selection_slot(read_usize(0xde1d08), club, player, t.reserves) as i8;
            if slot > -1 {
                if slot < 0xb {
                    let tactic = exe::human_manager_get_active_tactic(read_usize(0xb63c98), 0usize);
                    // the slot goes in a stack slot whose upper bytes are the filter's (as the original)
                    let pos = exe::formation_get_slot_position(tactic, (t.filter & 0xffff_ff00) | slot as u8 as u32, 0);
                    squad_position_text(pos as u32);
                } else {
                    sb(0xde1f64, 0x98ea6c, slot as i32 - 0xa);
                }
                t.add_item([0x46, 0, 0, 0, 0, col as u32, row as u32, 0x30, cbe04, cbe04, 0xc, 1, c3184, 0xde1f64, 0, 0xa, slot as i32 as u32, area]);
            } else {
                t.add_item([6, 0, 0, 0, 0, col as u32, row as u32, 0x30, cbe04, cbe04, 0xc, 1, c3184, 0xad9c64, 0, 0xb, player as u32, area]);
            }
        }
        return;
    }
    let mut text = 0xad9c64usize;
    if t.from_record {
        let side = if read_u32(club) != read_u32(0xad9db8) { 0x625 } else { 0 };
        let mut slot = -1i8;
        for k in 0..0x14u8 {
            if read_u32(0xad9f48 + side + k as usize * 0x49) == read_u32(player) {
                slot = k as i8;
            }
        }
        if slot > -1 {
            if slot < 0xb {
                squad_position_text(read_u16(0xad9f0c + side + slot as usize * 2) as u32);
            } else {
                sb(0xde1f64, 0x98ea6c, slot as i32 - 0xa);
            }
            text = 0xde1f64;
        }
    }
    t.text(area, col, row, 0x30, cbe04, 0xc, 1, c3184, text);
}

/// Column 1 of a row: the status item (see `club_screen_squad_tab_rows`).
unsafe fn club_screen_squad_tab_status(t: &mut SquadTab, area: u32, player: usize, col: i32, row: i32) {
    let club = t.club;
    let code: unsafe extern "C" fn(usize, usize, usize) -> u8 = core::mem::transmute(0x4849f0usize);
    let status = code(player, club, t.next_match);
    // the item: flags, the colour twice (a9, a10), font 0xc, the text colour c13, link and data
    let item = |t: &SquadTab, flags: u32, a9: u32, a10: u32, c13: u32, link: u32, data: usize| {
        t.add_item([flags, 0, 0, 0, 0, (col + 1) as u32, row as u32, 0x30, a9, a10, 0xc, 1, c13, 0xde1f64, 0, link, data as u32, area]);
    };
    let say = |label: usize| {
        squad_text(0xde1f64, label);
    };
    let national = || exe::database_is_national_team(club) != 0;
    let (c3184, cbe04, cbdcc, c3190, c317c, cbdec) = (squad_colour(0xae3184), squad_colour(0xaebe04), squad_colour(0xaebdcc), squad_colour(0xae3190),
        squad_colour(0xae317c), squad_colour(0xaebdec));
    match status {
        1 | 2 | 3 => {
            say(match status { 1 => 0x98e9c8, 2 => 0x98e998, _ => 0x98e994 }); // "Wpm", "Cup", "Ine"
            let link = if national() { 0x7e0 } else { 0x7db };
            item(t, 0x202, cbe04, cbe04, c3184, link, player);
        }
        4 => {
            // international duty: no text of its own (0xde1f64 keeps what the row wrote last, as the original)
            let duty = exe::nationalteams_get_international_duty_club(read_usize(G_NATIONAL_TEAMS), player, G_CURRENT_DATE, club, 0, 0) as usize;
            if duty != 0 {
                item(t, 0x202, cbe04, cbe04, c3184, 0x7d5, duty);
            } else {
                item(t, 0x201, cbe04, cbe04, c3184, 0, 0);
            }
        }
        5 => {
            say(0x98e990); // "Sus"
            let link = if national() { 0x7de } else { 0x7d9 };
            item(t, 0x202, cbdcc, cbdcc, c3184, link, player);
        }
        6 => {
            say(0x98e98c); // "Inj"
            let link = if national() { 0x7dd } else { 0x7d8 };
            item(t, 0x202, cbdcc, cbdcc, c3184, link, player);
        }
        0x15 => {
            say(0x98e988); // "Abs"
            item(t, 0x202, cbdcc, cbdcc, c3184, 0x7db, player);
        }
        7 | 9 => {
            say(0x98e984); // "Fgn"
            item(t, 0x202, cbe04, cbe04, c3184, 0x7db, player);
        }
        8 => {
            say(0x98e98c); // "Inj"
            let link = if national() { 0x7dd } else { 0x7d8 };
            item(t, 0x202, c3190, c3190, c3184, link, player);
        }
        0x16 => {
            say(0x98e97c); // "Tir "
            item(t, 0x202, c3190, c3190, c3184, 0x7d8, player);
        }
        0xa => {
            say(0x98e978); // "Yel"
            let link = if national() { 0x7de } else { 0x7d9 };
            let colour = squad_colour(0xaebdea);
            let first = exe::graphics_rgb_to_pixel(0xb4, 0xb4, 0, 0);
            let second = exe::graphics_rgb_to_pixel(0xb4, 0xb4, 0, 0);
            item(t, 0x202, second, first, colour, link, player);
        }
        0xb | 0xc | 0xd | 0xf | 0x12 | 0x18 | 0xe | 0x13 => {
            // Bid, Wnt, Req, Loa, Bos, Trn, Lst, Unh
            say(match status { 0xb => 0x98e974, 0xc => 0x98e96c, 0xd => 0x98e934, 0xf => 0x98e904, 0x12 => 0x98e8f0, 0x18 => 0x98e970, 0xe => 0x98e908,
                _ => 0x98e8e8 });
            item(t, 0x202, c317c, c317c, cbdec, 0x7db, player);
        }
        0x17 | 0x11 => {
            say(if status == 0x17 { 0x98e938 } else { 0x98e8f8 }); // "Sct", "Ret "
            let link = if national() { 0x7e0 } else { 0x7db };
            item(t, 0x202, c317c, c317c, cbdec, link, player);
        }
        0x10 => {
            say(0x98e900); // "Ctr"
            item(t, 0x202, c317c, c317c, cbdec, 0x7da, player);
        }
        0x14 => {
            say(0x98e8e0); // "Fut "
            let add: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32 =
                core::mem::transmute(0x45cb60usize);
            add(t.sm, 0x202, 0, 0, 0, 0, (col + 1) as u32, row as u32, 0x30, c317c, c317c, 0xc, 1, cbdec, 0xde1f64, 0, 0x7db, player as u32, area);
        }
        _ => {}
    }
}

/// Column 3 of the positions view, by the sort (see `club_screen_squad_tab_rows`).
unsafe fn club_screen_squad_tab_sort_cell(t: &mut SquadTab, area: u32, player: usize, col: i32, row: i32, injury: u8, contract_club: usize, contract: usize,
                                          buf: &mut [u8; 0x7d0]) {
    let club = t.club;
    let sort = t.sort;
    let pdata = read_usize(player + staff::PLAYER);
    let cbdec = squad_colour(0xaebdec);
    let c = col + 3;
    let b = buf.as_mut_ptr() as usize;
    let cell = |t: &SquadTab, a6: u32, a7: u32| t.text(area, c, row, 1, 0, a6, a7, cbdec, 0xde1f64);
    if sort & 0x20 != 0 {
        let squad = read_usize(0xdd7ef0);
        if exe::squad_manager_club_uses_squad_numbers(squad, club, 0) != 0 && exe::squad_manager_get_squad_number(squad, player, club) != 0 {
            let n = exe::squad_manager_get_squad_number(read_usize(0xdd7ef0), player, club) as i32;
            squad_number_text(0x98806c, n.wrapping_abs());
        } else {
            squad_number_text(0x98ba0c, 0); // "-"
        }
        cell(t, 0xc, 2);
    } else if sort & 0x80 != 0 {
        if squad_is_new_person(player) {
            copy_text(0xde1f64 as *mut u8, 0x98ba0c as *const u8);
        } else {
            squad_recent_ratings(pdata);
        }
        cell(t, 0xc, 1);
    } else if sort & 0x100 != 0 {
        squad_morale(player);
        cell(t, 0xc, 2);
    } else if sort & 0x200 != 0 {
        if injury == 2 {
            squad_text(0xde1f64, 0x98e98c); // "Inj"
        } else {
            squad_number_text(0x98e8d4, exe::injury_get_condition(read_usize(G_INJURY), player, 1) as i32);
        }
        cell(t, 0xc, 2);
    } else if sort & 0x2000 != 0 {
        if exe::nationalteams_is_national_or_all_star_team(read_usize(G_NATIONAL_TEAMS), club) == 0 {
            let nation = read_usize(player + 0x1a);
            if nation != 0 {
                let team = exe::database_get_national_team(nation);
                t.button(area, c, row, 1, 0, 0xc, 2, cbdec, nation + 0x53, 0x7d5, team);
            }
        } else {
            let club_job = read_usize(player + staff::CLUB_JOB);
            if club_job != 0 {
                t.button(area, c, row, 1, 0, 1, 2, cbdec, club_job + club::SHORT_NAME, 0x7d5, club_job);
            }
        }
    } else if sort & 0x4000 != 0 {
        squad_number_text(0x98806c, read_i8(player + 0x18) as i32);
        cell(t, 0xc, 2);
    } else if sort & 0x8000 != 0 {
        squad_number_text(0x98806c, read_u8(player + 0x22) as i32);
        cell(t, 0xc, 2);
    } else if sort & 0x10000 != 0 {
        squad_number_text(0x98806c, read_u8(player + 0x23) as i32);
        cell(t, 0xc, 2);
    } else if sort & 0x20000 != 0 {
        if contract_club != 0 || contract != 0 {
            squad_wage_text(if contract_club != 0 { contract_club } else { contract }, buf);
            squad_string_text(0x987ff0, b); // "  %s"
        } else {
            write_u8(0xde1f64, 0);
        }
        cell(t, 1, 2);
    } else if sort & 0x40000 != 0 {
        if contract_club != 0 || contract != 0 {
            squad_end_date(if contract_club != 0 { contract_club } else { contract }, b);
            squad_string_text(0x987ff0, b);
        } else {
            write_u8(0xde1f64, 0);
        }
        cell(t, 1, 2);
    } else if sort & 0x20000000 != 0 {
        let yes = contract != 0 && exe::staff_contract_check_fifa_protected_period(contract, 0usize, 0usize, 0usize) != 0;
        squad_text(0xde1f64, if yes { 0x98e8d0 } else { 0x98e8cc }); // "Yes", "No"
        cell(t, 1, 2);
    } else if sort & 0x80000 != 0 {
        squad_value_text(player);
        let club_job = read_usize(player + staff::CLUB_JOB);
        if !squad_is_new_person(player) && club_job != 0 && exe::human_manager_current_controls_club(club_job, 0, 1) != 0 {
            t.button(area, c, row, 1, 0, 0xc, 2, cbdec, 0xde1f64, 0xf, player);
        } else {
            cell(t, 0xc, 2);
        }
    } else if sort & 0xf00000 != 0 {
        let field = if sort & 0x100000 != 0 { 2 } else if sort & 0x200000 != 0 { 3 } else if sort & 0x400000 != 0 { 5 } else { 0x11 };
        squad_stat(pdata, 6, field, 0xde1f64);
        cell(t, 0xc, 2);
    } else {
        squad_positions(pdata, 0xde1f64, 0x7d0);
        cell(t, 1, 2);
    }
}

/// The columns 3.. of the views other than positions (see `club_screen_squad_tab_rows`).
unsafe fn club_screen_squad_tab_view_cells(t: &mut SquadTab, area: u32, player: usize, col: i32, row: i32, injury: u8, contract_club: usize, contract: usize,
                                           buf: &mut [u8; 0x7d0]) {
    let club = t.club;
    let view = t.view;
    let pdata = read_usize(player + staff::PLAYER);
    let b = buf.as_mut_ptr() as usize;
    let (cbdec, cbde8, cbdd4, c31a8) = (squad_colour(0xaebdec), squad_colour(0xaebde8), squad_colour(0xaebdd4), squad_colour(0xae31a8));
    let cell = |t: &SquadTab, c: i32, a6: u32, a7: u32, colour: u32, text: usize| t.text(area, col + c, row, 1, 0, a6, a7, colour, text);
    let form = |t: &SquadTab, c: i32| {
        if squad_is_new_person(player) {
            copy_text(0xde1f64 as *mut u8, 0x98ba0c as *const u8);
        } else {
            squad_recent_ratings(pdata);
        }
        cell(t, c, 0xc, 1, cbdec, 0xde1f64);
    };
    let condition = |t: &SquadTab, c: i32| {
        if injury == 2 {
            squad_text(0xde1f64, 0x98e98c); // "Inj"
        } else {
            squad_number_text(0x98e8d4, exe::injury_get_condition(read_usize(G_INJURY), player, 1) as i32);
        }
        cell(t, c, 0xc, 2, cbdd4, 0xde1f64);
    };
    if view & 2 != 0 {
        if exe::nationalteams_is_national_or_all_star_team(read_usize(G_NATIONAL_TEAMS), club) == 0 {
            if exe::human_manager_current_controls_club(club, 0, 1) != 0 && contract_club != 0 {
                let status: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x867ef0usize);
                status(contract_club, b, 0x7d0);
                squad_string_text(0x987ff0, b);
                t.button(area, col + 3, row, 1, 0, 1, 2, cbde8, 0xde1f64, 0xf, player);
            } else {
                cell(t, 3, 1, 2, cbde8, 0x98e8c8); // "  -"
            }
        } else {
            let club_job = read_usize(player + staff::CLUB_JOB);
            if club_job != 0 {
                squad_string_text(0x987ff0, club_job + club::SHORT_NAME);
                t.button(area, col + 3, row, 1, 0, 1, 2, cbde8, 0xde1f64, 0x7d5, club_job);
            }
        }
        if contract_club != 0 || contract != 0 {
            squad_wage_text(if contract_club != 0 { contract_club } else { contract }, buf);
            squad_string_text(0x987ff0, b);
        } else {
            squad_string_text(0x98e8c8, 0); // "  -"
        }
        cell(t, 4, 1, 2, cbdec, 0xde1f64);
        if contract_club != 0 || contract != 0 {
            squad_end_date(if contract_club != 0 { contract_club } else { contract }, b);
            squad_string_text(0x987ff0, b);
        } else {
            squad_string_text(0x98e8c8, 0);
        }
        cell(t, 5, 1, 2, cbdec, 0xde1f64);
        if exe::database_is_national_team(club) == 0 {
            if contract != 0 {
                let release: unsafe extern "thiscall" fn(usize, usize, u32) -> u8 = core::mem::transmute(0x868020usize);
                let r = release(contract, b, 0x7d0);
                squad_string_text(0x987ff0, b);
                if r == 2 {
                    t.button(area, col + 6, row, 1, 0, 1, 2, c31a8, 0xde1f64, 0x7da, player);
                } else if r == 1 {
                    t.button(area, col + 6, row, 1, 0, 1, 2, cbdd4, 0xde1f64, 0x7da, player);
                } else {
                    cell(t, 6, 1, 2, cbdd4, 0xde1f64);
                }
            } else {
                cell(t, 6, 1, 2, cbdd4, 0x98e8c8);
            }
        }
    } else if view & 0x1000 != 0 {
        squad_positions(pdata, b, 0xc8);
        squad_string_text(0x987ff0, b);
        cell(t, 3, 1, 2, cbde8, 0xde1f64);
        form(t, 4);
        squad_morale(player);
        cell(t, 5, 0xc, 2, cbdec, 0xde1f64);
        condition(t, 6);
        squad_stat(pdata, 6, 1, 0xde1f64);
        cell(t, 7, 0xc, 2, cbde8, 0xde1f64);
        squad_stat(pdata, 6, 0x11, 0xde1f64);
        cell(t, 8, 0xc, 2, cbde8, 0xde1f64);
    } else if view & 4 != 0 || view & 8 != 0 {
        let table = if view & 4 != 0 { 0x98de84 } else { 0x98de90 };
        let mut k = 0usize;
        while read_u8(table + k) != 0xff {
            let field = read_u8(table + k);
            // simplification: param 9 is a byte in a stack slot whose upper bytes are garbage in the original (a char)
            squad_stat(pdata, t.category as u32, field as u32, 0xde1f64);
            cell(t, k as i32 + 3, 0xc, 2, if field == 0x11 { cbdec } else { cbde8 }, 0xde1f64);
            k += 1;
        }
    } else if view & 0x10 != 0 {
        let table = t.attributes;
        let mut k = 0usize;
        while read_u8(table + k) != 0xff {
            let attr = read_u8(table + k);
            if t.human != 0 && exe::staff_is_player_attribute_visible(t.human, player, attr as u32) != 0 {
                squad_number_text(0x98806c, exe::staff_get_player_attribute(player, attr as u32) as i32);
                cell(t, k as i32 + 3, 0xc, 2, cbde8, 0xde1f64);
            } else {
                cell(t, k as i32 + 3, 0xc, 2, cbde8, 0x98ba0c); // "-"
            }
            k += 1;
        }
    } else if view & 0x20 != 0 {
        let nation = read_usize(player + 0x1a);
        if nation != 0 {
            let team = exe::database_get_national_team(nation);
            t.button(area, col + 3, row, 1, 0, 0xc, 2, cbde8, nation + 0x53, 0x7d5, team);
        }
        squad_number_text(0x98806c, read_i8(player + 0x18) as i32);
        cell(t, 4, 0xc, 2, cbde8, 0xde1f64);
        squad_number_text(0x98806c, read_u8(player + 0x22) as i32);
        cell(t, 5, 0xc, 2, cbde8, 0xde1f64);
        squad_number_text(0x98806c, read_u8(player + 0x23) as i32);
        cell(t, 6, 0xc, 2, cbde8, 0xde1f64);
        form(t, 7);
        squad_morale(player);
        cell(t, 8, 0xc, 2, cbdec, 0xde1f64);
        condition(t, 9);
    }
}

/// Comparer for `club_screen_draw_squad_tab`.
///
/// Through the display list (`compare_by_display_list_with`), plus the attribute tables the attributes view sorts in
/// place (0x98de84..0x98ded8), the human's squad view block and the last match's team sheet record (0xad9db8, 0xd7a
/// bytes). What it also does outside the list runs twice and is idempotent: the tactic slot, the squad numbers pruned,
/// each player's training caught up to the week, the team sheet cleared of gone players.
pub(crate) unsafe extern "C" fn compare_club_screen_draw_squad_tab(top: i32, bottom: i32) {
    let o = original_fn::<unsafe extern "C" fn(i32, i32)>(%%I0%%);
    let mut extra = vec![(0x98de84usize, 0x54usize), (0xad9db8, 0xd7a)];
    if exe::human_manager_current_controls_club(get_screen_param(0), 0, 1) != 0 {
        extra.push((club_screens_squad_view_settings(), 0xf));
    }
    compare_by_display_list_with("club_screen_draw_squad_tab (club screen, squad tab)", &extra, || o(top, bottom), || club_screen_draw_squad_tab(top, bottom));
}
