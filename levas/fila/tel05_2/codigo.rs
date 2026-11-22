// ---------- batch %%LEVA%%: the club screens (5b): the part exchange screen, Add Player To Transfer Bid (club_screens.cpp) ----------

/// A colour word of the interface palette, zero-extended (the original pushes it in a register whose high half is
/// whatever the register held: garbage that `Display` keeps out, since it stores the colours as words).
unsafe fn part_exchange_colour(a: usize) -> u32 {
    read_u16(a) as u32
}

/// The game's text formatter (`FN_LANGLIB_SPRINTF`), variadic.
unsafe fn part_exchange_lang() -> unsafe extern "C" fn(usize, usize, ...) -> usize {
    core::mem::transmute(FN_LANGLIB_SPRINTF)
}

/// The C runtime's sprintf (`FN_CRT_SPRINTF`), variadic.
unsafe fn part_exchange_sprintf() -> unsafe extern "C" fn(usize, usize, ...) -> i32 {
    core::mem::transmute(FN_CRT_SPRINTF)
}

/// `LLIST::LLIST` (0x68d020) and `LLIST::~LLIST` (0x68d040), thiscall, on a local list.
unsafe fn part_exchange_llist(l: &mut [u8; 0x20], construct: bool) {
    let f: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(if construct { FN_LLIST_LLIST } else { FN_LLIST_DESTRUCTOR_LLIST });
    f(l.as_mut_ptr() as usize);
}

/// Puts a local list's walk back at its head (the iterator +0xa takes the head +2).
unsafe fn part_exchange_rewind(l: usize) {
    write_u32(l + 0xa, read_u32(l + 2));
}

/// The part exchange screen's state: what the original keeps in registers and stack slots across its pieces.
struct PartExchange {
    /// param 0: the transfer bid (+8 the bidding club's id, +0xc the player's id, +4 the list of the players already
    /// offered in exchange)
    bid: usize,
    /// param 3: nonzero while choosing (the names link to 2 instead of the person) [inference]
    choosing: u32,
    /// param 4: the view flags; in the attributes view the slot then holds the attribute table, as the original
    view: u32,
    /// param 4 as read (the tests of the rows; the original keeps each one in its own stack slot)
    view_flags: u32,
    /// param 5: the sort flags
    sort: u32,
    /// params 6, 7 and 8 (bytes): the statistics category and two list options
    category: u8,
    option7: u8,
    option8: u8,
    /// param 9: the filter flags; its low byte later holds the row count (the original reuses the slot)
    filter: u32,
    /// the bidding club (its players are listed) and the bid's player and his club (the seller)
    club: usize,
    player: usize,
    seller: usize,
    /// the slot's human manager
    human: usize,
    /// the bidding club's kit colours (the original's words in dword slots)
    kit_back: u32,
    kit_text: u32,
    title: [u8; 0xc8],
    name: [u8; 0x17c],
    /// the view bar (the rows go there when there is no view of its own)
    bar: u32,
    /// players of the club the selling club does not want / whose wage demand is too high / rows shown
    not_interested: u8,
    too_dear: u8,
    shown: u8,
    /// the first dword of the club's filtered squad list (its count in the low word)
    squad_head: u32,
    /// the offer and negotiation lists of a player, the club's filtered squad, the candidates
    offers: [u8; 0x20],
    negotiations: [u8; 0x20],
    squad: [u8; 0x20],
    candidates: [u8; 0x20],
    /// the local contract used to ask for the wage (STAFF_CONTRACT, 0x50 bytes)
    contract: [u32; 0x14],
}

/// The column weights of the views (13 columns; the positions view's 8).
struct PartExchangeWeights {
    two: [u8; 8],
    contract: [u8; 13],
    stats: [u8; 13],
    attributes: [u8; 13],
    other: [u8; 13],
    selection: [u8; 13],
}

impl PartExchange {
    unsafe fn display(&self) -> usize {
        screens_current_display(G_SCREEN_MANAGER)
    }
    unsafe fn add_area(&self, f: [u32; 11]) -> u32 {
        tel26_add_area(self.display(), f)
    }
    /// A column heading button in row 0 of `area` (the label already at 0xde1f64); returns eax.
    unsafe fn heading(&self, area: u32, col: u32, font: u32, link: u32, data: u32) -> u32 {
        tel26_area_button([area, col, 0, 0x30, part_exchange_colour(0xae317e), font, 1, part_exchange_colour(0xae3184), 0xde1f64, link, data])
    }
}

/// `PlayerStats::format_player_stat(pdata, category, field, 0xde1f64, 0x7d0, 1, 0, 1)` (0x7c25b0, thiscall on
/// [0xb74350], ret 0x20).
unsafe fn part_exchange_stat(pdata: usize, category: u32, field: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x7c25b0usize);
    f(read_usize(G_PLAYER_STATS), pdata, category, field, 0xde1f64, 0x7d0, 1, 0, 1);
}

/// The player's form: "-" for a person made in the game, else `PlayerStats::format_recent_ratings` (0x7c1660) into
/// 0xde1f64.
unsafe fn part_exchange_form(player: usize) {
    if player != 0 && read_i32(player) >= read_i32(G_STAFF_COUNT) {
        copy_text(0xde1f64 as *mut u8, 0x98ba0c as *const u8);
    } else {
        let f: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x7c1660usize);
        f(read_usize(G_PLAYER_STATS), read_usize(player + staff::PLAYER), 0xde1f64, 0x7d0);
    }
}

/// The player's morale text (`contract_manager_write_morale_text`, 0x4cc2e0, with ecx = [0xae1050]) into 0xde1f64.
unsafe fn part_exchange_morale(player: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x4cc2e0usize);
    f(read_usize(G_CONTRACT_MANAGER), player, 0xde1f64, 0x7d0);
}

/// The player's condition: "Inj" when injured (status 2), else "<condition>%" (`exe::injury_get_condition`, 1).
unsafe fn part_exchange_condition(player: usize, injury: u8) {
    if injury == 2 {
        part_exchange_lang()(0xde1f64, 0x98e98c); // "Inj"
    } else {
        part_exchange_sprintf()(0xde1f64, 0x98e8d4, exe::injury_get_condition(read_usize(G_INJURY), player, 1) as i32); // "%d%%"
    }
}

/// `database_format_player_positions(pdata, out, size, 1)` (0x53ee50, cdecl).
unsafe fn part_exchange_positions(pdata: usize, out: usize, size: u32) {
    let f: unsafe extern "C" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x53ee50usize);
    f(pdata, out, size, 1);
}

/// A contract's wage text into `buf` (`STAFF_CONTRACT::format_wage_text`, 0x85e750, style 0, cash flags 2; again with
/// flags 1 when longer than 8 characters), then "  %s" into 0xde1f64.
unsafe fn part_exchange_wage(contract: usize, buf: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(0x85e750usize);
    f(contract, buf, 0, 2);
    if core::ffi::CStr::from_ptr(buf as *const core::ffi::c_char).to_bytes().len() > 8 {
        f(contract, buf, 0, 1);
    }
    part_exchange_sprintf()(0xde1f64, 0x987ff0, buf); // "  %s"
}

/// A contract's end date (`STAFF_CONTRACT::format_end_date_text`, 0x85e8f0) into `buf`, then "  %s" into 0xde1f64.
unsafe fn part_exchange_end_date(contract: usize, buf: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x85e8f0usize);
    f(contract, buf);
    part_exchange_sprintf()(0xde1f64, 0x987ff0, buf);
}

/// The player's value as money into 0xde1f64 (`Cash::get_display_amount` 5 on a local amount, then
/// `cash_format_money`, 5) when he has a club and a value, else "Free".
unsafe fn part_exchange_value(player: usize) {
    if read_usize(player + staff::CLUB_JOB) != 0 && read_i32(player + staff::VALUE) != 0 {
        let cash = read_u32(player + staff::VALUE);
        let amount = exe::cash_get_display_amount(&cash, 5);
        exe::cash_format_money(0xde1f64usize, 0x7d0, amount, 5);
    } else {
        part_exchange_lang()(0xde1f64, 0x98e614); // "Free"
    }
}

/// Rule: the part exchange screen (Add Player To Transfer Bid) lists the players of the bidding club the selling club
/// would take in a part exchange, in the chosen view, sorted and filtered as the menus say; when none can be offered, it
/// says whether the selling club is not interested or the players' wage demands are too high.
///
/// `part_exchange_screen_build()`, from the screen parameters 0 (the bid), 3, 4 (the view), 5 (the sort), 6..8 and 9 (the
/// filter), 1 (the scroll row). The pieces: `part_exchange_header` (the title, the bid text and the menus),
/// `part_exchange_candidates` (the club's filtered squad and the players that can be offered), `part_exchange_list_area`
/// (the list's area and the column headings), `part_exchange_rows` (one row per candidate) and `part_exchange_footer`
/// (the message when nobody can be offered, the scroll row, the area in param 2 and the Back and Next buttons). Local
/// objects as the original: the bid text (`Text::Text(0)`, destroyed at the end), four lists (`LLIST`) and a contract
/// (`STAFF_CONTRACT::STAFF_CONTRACT`, no destructor).
/// Original: `part_exchange_screen_build` (0x47c180, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn part_exchange_screen_build() {
    covered!("tel05_2 part exchange screen");
    let mut text = [0usize; 4]; // the bid text (Text, 8 bytes)
    exe::new_text(text.as_mut_ptr(), 0usize);
    let mut s = PartExchange {
        bid: 0,
        choosing: 0,
        view: 0,
        view_flags: 0,
        sort: 0,
        category: 0,
        option7: 0,
        option8: 0,
        filter: 0,
        club: 0,
        player: 0,
        seller: 0,
        human: 0,
        kit_back: part_exchange_colour(0xae3184),
        kit_text: part_exchange_colour(0xaebdf0),
        title: [0; 0xc8],
        name: [0; 0x17c],
        bar: 0,
        not_interested: 0,
        too_dear: 0,
        shown: 0,
        squad_head: 0,
        offers: [0; 0x20],
        negotiations: [0; 0x20],
        squad: [0; 0x20],
        candidates: [0; 0x20],
        contract: [0; 0x14],
    };
    part_exchange_llist(&mut s.offers, true);
    part_exchange_llist(&mut s.negotiations, true);
    exe::new_staff_contract(s.contract.as_mut_ptr() as usize);
    part_exchange_llist(&mut s.squad, true);
    let mut w = PartExchangeWeights {
        two: [6, 5, 0x1e, 0x10, 6, 5, 0x1e, 0x10],
        contract: [5, 5, 0x18, 0xe, 0xe, 0xf, 0xb, 0, 0, 0, 0, 0, 0xa],
        stats: [5, 5, 0x18, 6, 6, 6, 6, 6, 6, 6, 6, 6, 0xa],
        attributes: [5, 5, 0x18, 6, 6, 6, 6, 6, 6, 6, 6, 6, 0xa],
        other: [5, 5, 0x18, 8, 6, 6, 6, 0xa, 0xc, 6, 0, 0, 0xa],
        selection: [5, 5, 0x18, 0xe, 0xa, 0xc, 6, 6, 6, 0, 0, 0, 0xa],
    };
    part_exchange_llist(&mut s.candidates, true);
    part_exchange_header(&mut s, text.as_mut_ptr() as usize);
    part_exchange_candidates(&mut s);
    let area = part_exchange_list_area(&mut s, &mut w);
    part_exchange_rows(&mut s, area, &w);
    part_exchange_footer(&mut s, area);
    part_exchange_llist(&mut s.candidates, false);
    part_exchange_llist(&mut s.squad, false);
    part_exchange_llist(&mut s.negotiations, false);
    part_exchange_llist(&mut s.offers, false);
    exe::drop_text(text.as_mut_ptr());
}

/// The parameters, the title and the bid text, and the menus of the part exchange screen, in the original's order.
///
/// The club is the club table's bid +8, the player the person table's bid +0xc; his contract
/// (`exe::contract_manager_get_contract`) is written to [0xae1080] (`G_CONTRACT_LOOKUP_RESULT`, as the original) and its
/// club (+4) is the seller. The player's value × 1.2 less each offered player's value is worked out and never used, as
/// the original. The player's background picture (`background_set_staff_picture`, 0x41c790). The club's kit colours
/// (kit 1) as the title colours (`exe::screenmanager_add_screen_title` with the club's short name). The text "Add Player
/// To <player> Transfer Bid" (his name in style 5, a link 0x7d7 to him) wrapped (`Text::draw_wrapped`, 0x4aeaf0, font 6)
/// in an area 0x64..0x316 by 0x50..0x7d. The menu bar (`exe::menubar_build` 4). The view bar (0x6e..0x168 by 0x91..0xa5)
/// with "View" (link 4): Traditional 1 (title "Position(s)"), Contract 2 ("Contract Info"), Selection 0x1000 ("Selection
/// Info"), Stats 4, More Stats 8, Attributes 0x10, Other 0x20 ("Other Info"); in the Traditional view "Sort By" (link 6,
/// the ticked one's text without the first space as the title): Name 4, Position(s) 8, Squad Number 0x20, Form 0x80,
/// Morale 0x100, Condition 0x200, Nationality 0x2000, Age 0x4000, Int. Caps 0x8000, Int. Goals 0x10000, Basic Wage
/// 0x20000, Contract Expiry 0x40000, Contract Protected 0x20000000, Value 0x80000, a separator, Goals 0x100000, Conceded
/// 0x200000, Assists 0x400000, Av. Rating 0x800000; with Stats or More Stats "Competitions" (link 5): the six categories
/// (`player_stats_get_category_name`, 0x7c14b0), the one of param 6 ticked with "<category> Stats" as the title; with
/// Attributes "Attributes" (link 4): Physical 0x40, Mental 0x80, Goalkeeping 0x100, Defensive 0x200, Attacking 0x400,
/// each ticked with its "... Attributes" title. The "Exchange" button (0x294..0x311 by 4..0x18, the kit colours,
/// flags 0x70 and data 0 while choosing, else 0x30 and 1). The "Filter" button (0x28f..0x30c by 0x91..0xa5, link 9):
/// All Positions 1, Goalkeepers 2, Defenders 4, Midfielders 8, Attackers 0x10, a separator, All Sides 0x20, Left Sided
/// 0x40, Central 0x80, Right Sided 0x100, a separator. The title (font 3) at 0x6e..0x30c by 0xaa..0xcd.
unsafe fn part_exchange_header(s: &mut PartExchange, text: usize) {
    let sm = G_SCREEN_MANAGER;
    s.bid = get_screen_param(0);
    s.choosing = get_screen_param(3) as u32;
    s.view = get_screen_param(4) as u32;
    s.view_flags = s.view;
    s.sort = get_screen_param(5) as u32;
    s.category = get_screen_param(6) as u8;
    s.option7 = get_screen_param(7) as u8;
    s.option8 = get_screen_param(8) as u8;
    s.filter = get_screen_param(9) as u32;
    s.club = club_screens_club(read_i32(s.bid + 8));
    s.player = club_screens_staff(read_i32(s.bid + 0xc));
    let contract = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), s.player);
    write_usize(G_CONTRACT_LOOKUP_RESULT, contract);
    s.seller = if contract != 0 { club_screens_club(read_i32(contract + contract::CLUB)) } else { 0 };
    s.human = tel26_slot_manager().addr();
    // the bid's budget (value × 1.2, less the offered players' values later): worked out and never read, as the original
    let _budget = read_i32(s.player + staff::VALUE) as f64 * read_f64(rdata::F64_1_2);
    let picture: unsafe extern "C" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x41c790usize);
    picture(s.player, 0, 0);
    exe::database_get_club_kit_colours(s.club, 1, &mut s.kit_text, &mut s.kit_back);
    exe::screenmanager_add_screen_title(sm, s.kit_back, s.kit_text, s.club + club::SHORT_NAME);
    exe::database_format_staff_name(s.player, s.name.as_mut_ptr() as usize, 0x12c, 5);
    part_exchange_lang()(0xde1f64, 0x991494, 100u32, s.name.as_ptr()); // "Add Player To <%s ...>{} Transfer Bid"
    exe::text_set_string(text, 0xde1f64 as *const u8);
    exe::text_add_link(text, s.name.as_ptr(), 0x7d7, s.player);
    let mut line = 0i32;
    let a = s.add_area([0x64, 0x50, 0x316, 0x7d, 1, 0, 1, 0, 2, 0, u32::MAX]);
    let wrapped: unsafe extern "thiscall" fn(usize, u32, u32, u32, *mut i32, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x4aeaf0usize);
    wrapped(text, a, 0x64, 0x316, &mut line, 6, part_exchange_colour(0xaebdec), 1, 0);
    exe::menubar_build(4, 0);
    s.title[0] = 0;
    s.bar = s.add_area([0x6e, 0x91, 0x168, 0xa5, 2, 0, 1, 0, 1, 0, u32::MAX]);
    squad_list_menus(s.bar, s.view, s.sort, s.category, s.title.as_mut_ptr(), [4, 6, 5, 4], 0x98edb8);
    let lang = part_exchange_lang();
    lang(0xde1f64, 0x991488); // "Exchange"
    let flags = if s.choosing != 0 { 0x70 } else { 0x30 };
    tel26_add_button([0x294, 4, 0x311, 0x18, flags, s.kit_text, 0xc, 1, s.kit_back, 0xde1f64, 1, (s.choosing == 0) as u32]);
    lang(0xde1f64, 0x98ebf8); // "Filter"
    let filter_button = tel26_add_button([0x28f, 0x91, 0x30c, 0xa5, 0x30, part_exchange_colour(0xae317e), 0xc, 1, part_exchange_colour(0xae3184), 0xde1f64, 0, 0]);
    let filter_menu = s.add_area([0, 0, 0, 0, 0, 0, 0, 0, 0x130, part_exchange_colour(0xae31aa), filter_button]);
    for (label, bit) in [(0x98ebe8usize, 1u32), (0x98ebd8, 2), (0x98ebcc, 4), (0x98ebbc, 8), (0x98ebb0, 0x10)] {
        lang(0xde1f64, label);
        squad_list_menu_item(filter_menu, s.filter & bit != 0, 9, bit);
    }
    squad_list_separator(filter_menu);
    for (label, bit) in [(0x98eba4usize, 0x20u32), (0x98eb98, 0x40), (0x98eb8c, 0x80), (0x98eb7c, 0x100)] {
        lang(0xde1f64, label);
        squad_list_menu_item(filter_menu, s.filter & bit != 0, 9, bit);
    }
    squad_list_separator(filter_menu);
    let manager_text: ManagerTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_TEXT_ITEM);
    manager_text(sm, 0x6e, 0xaa, 0x30c, 0xcd, 2, 0, 0xc, 3, part_exchange_colour(0xaebdec) as usize, s.title.as_ptr() as usize, 1);
}

/// A popup menu's item (ticked 0x2002 or plain 2) with the text at 0xde1f64.
unsafe fn squad_list_menu_item(area: u32, ticked: bool, link: u32, data: u32) {
    tel26_add_item(screens_current_display(G_SCREEN_MANAGER), [if ticked { 0x2002 } else { 2 }, 0, 0, 0, 0, 0, 0, 0x10, part_exchange_colour(0xae31aa),
        part_exchange_colour(0xaebdec), 1, 1, part_exchange_colour(0xaebdea), 0xde1f64, 0, link, data, area]);
}

/// A popup menu's separator line (flags 1, 0x1000010).
unsafe fn squad_list_separator(area: u32) {
    tel26_add_item(screens_current_display(G_SCREEN_MANAGER), [1, 0, 0, 0, 0, 0, 0, 0x1000010, part_exchange_colour(0xae31aa),
        part_exchange_colour(0xaebdec), 1, 1, part_exchange_colour(0xaebdea), 0, 0, 0, 0, area]);
}

/// A menu button in column `col` of the bar `bar` and its popup area (the label already at 0xde1f64).
unsafe fn squad_list_popup(bar: u32, col: u32) -> u32 {
    let b = tel26_area_button([bar, col, 0, 0x30, part_exchange_colour(0xae317e), 0xc, 1, part_exchange_colour(0xae3184), 0xde1f64, 0, 0]);
    tel26_add_area(screens_current_display(G_SCREEN_MANAGER), [0, 0, 0, 0, 0, 0, 0, 0, 0x130, part_exchange_colour(0xae31aa), b])
}

/// The menus of a squad list's view bar `bar`, in the original's order (the links of the View, Sort By, Competitions
/// and Attributes menus in `links`): "View": Traditional 1 (title "Position(s)"), Contract 2 ("Contract Info"),
/// Selection 0x1000 ("Selection Info"), Stats 4, More Stats 8, Attributes 0x10, Other 0x20 ("Other Info"); in the
/// Traditional view "Sort By" (the ticked one's text without the first space as the title): Name 4, Position(s) 8, Squad
/// Number 0x20, Form 0x80, Morale 0x100, Condition 0x200, Nationality (or `club_label`) 0x2000, Age 0x4000, Int. Caps
/// 0x8000, Int. Goals 0x10000, Basic Wage 0x20000, Contract Expiry 0x40000, Contract Protected 0x20000000, Value 0x80000,
/// a separator, Goals 0x100000, Conceded 0x200000, Assists 0x400000, Av. Rating 0x800000; with Stats or More Stats
/// "Competitions": the six categories (`player_stats_get_category_name`, 0x7c14b0), the `category` one ticked with
/// "<category> Stats" as the title; with Attributes "Attributes": Physical 0x40, Mental 0x80, Goalkeeping 0x100,
/// Defensive 0x200, Attacking 0x400, each ticked with its "... Attributes" title.
unsafe fn squad_list_menus(bar: u32, v: u32, sort: u32, category: u8, title: *mut u8, links: [u32; 4], club_label: usize) {
    let lang = part_exchange_lang();
    lang(0xde1f64, 0x988004); // "View"
    let view_menu = squad_list_popup(bar, 0);
    for (label, comment, bit, t) in [(0x98eedcusize, false, 1u32, 0x98eed0usize), (0x98eec4, false, 2, 0x98eeb4), (0x98ee88, true, 0x1000, 0x98ee78),
                                     (0x98ee70, false, 4, 0), (0x98ee64, false, 8, 0), (0x98ee58, false, 0x10, 0), (0x98ee30, true, 0x20, 0x98ee24)] {
        if comment {
            lang(0xde1f64, label, 0xad9c64usize);
        } else {
            lang(0xde1f64, label);
        }
        let ticked = v & bit != 0;
        if ticked && t != 0 {
            lang(title as usize, t);
        }
        squad_list_menu_item(view_menu, ticked, links[0], bit);
    }
    if v & 1 != 0 {
        covered!("tel05_2 sort menu");
        lang(0xde1f64, 0x98ee0c); // "Sort By"
        let sort_menu = squad_list_popup(bar, 1);
        let items: [(usize, u32); 18] = [(0x98ee04, 4), (0x98edf4, 8), (0x98ede4, 0x20), (0x98eddc, 0x80), (0x98edd4, 0x100), (0x98edc8, 0x200),
            (club_label, 0x2000), (0x98eda8, 0x4000), (0x98ed9c, 0x8000), (0x98ed90, 0x10000), (0x98ed84, 0x20000), (0x98ed70, 0x40000),
            (0x98ed5c, 0x20000000), (0x98ed54, 0x80000), (0x98ed4c, 0x100000), (0x98ed40, 0x200000), (0x98ed34, 0x400000), (0x98ed28, 0x800000)];
        for (label, bit) in items {
            if bit == 0x100000 {
                squad_list_separator(sort_menu);
            }
            lang(0xde1f64, label);
            let ticked = sort & bit != 0;
            squad_list_menu_item(sort_menu, ticked, links[1], bit);
            if ticked {
                copy_text(title, 0xde1f65 as *const u8);
            }
        }
    }
    if v & 4 != 0 || v & 8 != 0 {
        lang(0xde1f64, 0x98ed18); // "Competitions"
        let comp_menu = squad_list_popup(bar, 1);
        let name: unsafe extern "thiscall" fn(usize, u32, *mut u8, u32) -> u32 = core::mem::transmute(0x7c14b0usize);
        let mut buf = [0u8; 0xc8];
        for cat in 1i32..=6 {
            name(read_usize(G_PLAYER_STATS), cat as u32, buf.as_mut_ptr(), 0xc8);
            part_exchange_sprintf()(0xde1f64, 0x98ed14, buf.as_ptr()); // " %s"
            let ticked = cat == category as i8 as i32;
            if ticked {
                lang(title as usize, 0x98ecf4, buf.as_ptr()); // "<%s> Stats"
            }
            squad_list_menu_item(comp_menu, ticked, links[2], cat as u32);
        }
    }
    if v & 0x10 != 0 {
        lang(0xde1f64, 0x98ece8); // "Attributes"
        let attr_menu = squad_list_popup(bar, 1);
        for (label, bit, t) in [(0x98ecdcusize, 0x40u32, 0x98ecc8usize), (0x98ecc0, 0x80, 0x98ecac), (0x98eca0, 0x100, 0x98ec88),
                                (0x98ec7c, 0x200, 0x98ec64), (0x98ec58, 0x400, 0x98ec40)] {
            lang(0xde1f64, label);
            let ticked = v & bit != 0;
            if ticked {
                lang(title as usize, t);
            }
            squad_list_menu_item(attr_menu, ticked, links[3], bit);
        }
    }
}

/// The players that can be offered: the bidding club's squad filtered and sorted, less the ones already offered.
///
/// The club's list (`build_filtered_sorted_squad_list`, 0x4841d0: the club, no reserves, the sort, style 3, params 6..8,
/// the filter). For each person already offered (the bid's list +4, staff ids), the bid's budget loses his value and his
/// entry in the club's list is emptied. Then each player of the list made by the database, with a player record, without
/// a loan contract nor a contract of type 5, a player by job (`staff::JOB_FOR_CLUB` 0xb), with no transfer offer
/// (`TRANSFER_MANAGER::get_offers_for_player`) and no exchange offer or negotiation, or with exactly one exchange offer
/// (or else exactly one exchange negotiation) that is empty or belongs to this bid (+0xc, the bid's player): when the
/// club may bid for him at the seller (`TRANSFER_MANAGER::can_club_bid_for_player`, 1), for the human's own club with a
/// computer seller that does not want him (`SHORTLIST_MANAGER::club_wants_player` of the transfer manager's +0x856) he
/// counts as "not interested"; otherwise his desired wage at the seller (`STAFF_CONTRACT::set_to_desired_contract`, -1,
/// status 2, 0xb) above 1.5 × the seller's maximum wage (`Finance::get_max_wage`, 2, of
/// `FinanceTable::get_club_finance`) counts as "too dear", else he becomes a candidate.
unsafe fn part_exchange_candidates(s: &mut PartExchange) {
    let build: unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, u32, u32, usize, usize, usize, u32, u32, u32) -> u32 =
        core::mem::transmute(0x4841d0usize);
    let squad = s.squad.as_mut_ptr() as usize;
    // simplification: params 6..8 are bytes in stack slots whose upper bytes are garbage in the original (chars)
    build(s.club, 0, s.sort, 3, s.category as u32, s.option7 as u32, s.option8 as u32, s.filter, squad, 0, 0, 0, 0, 0);
    s.squad_head = read_u32(squad);
    let offered = read_usize(s.bid + 4);
    if offered != 0 {
        part_exchange_rewind(offered);
        loop {
            let e = exe::llist_get_next(offered);
            if e == 0 {
                break;
            }
            let id = read_u32(e);
            part_exchange_rewind(squad);
            loop {
                let f = exe::llist_get_next(squad);
                if f == 0 {
                    break;
                }
                let p = read_usize(f);
                if p != 0 && read_u32(p) == id {
                    write_usize(f, 0);
                    break;
                }
            }
        }
    }
    let tm = || read_usize(G_TRANSFER_MANAGER);
    let (offers, negotiations) = (s.offers.as_mut_ptr() as usize, s.negotiations.as_mut_ptr() as usize);
    let contract = s.contract.as_mut_ptr() as usize;
    let seller = s.seller;
    part_exchange_rewind(squad);
    loop {
        let f = exe::llist_get_next(squad);
        if f == 0 {
            break;
        }
        let p = read_usize(f);
        if p == 0 || read_i32(p) >= read_i32(G_STAFF_COUNT) || read_usize(p + staff::PLAYER) == 0 {
            continue;
        }
        if exe::contract_manager_get_loan_contract(read_usize(G_CONTRACT_MANAGER), p) != 0 {
            continue;
        }
        if exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), p) != 0
            && read_u8(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), p) + contract::CONTRACT_TYPE) & 0x3f == 5 {
            continue;
        }
        if read_u8(p + staff::JOB_FOR_CLUB) != 0xb {
            continue;
        }
        exe::llist_clear(offers);
        exe::llist_clear(negotiations);
        let free = exe::transfer_manager_get_offers_for_player(tm(), p, 0usize, 0) == 0
            && exe::transfer_manager_get_exchange_offers_for_player(tm(), p, offers) == 0
            && exe::transfer_manager_get_exchange_negotiations_for_player(tm(), p, negotiations) == 0;
        if !free {
            // one exchange offer (or else one negotiation) that is empty or this bid's
            let (list, slot_of) = if read_u16(offers) == 1 {
                (offers, 0x8c61e0usize)
            } else if read_u16(negotiations) == 1 {
                (negotiations, 0x8c6200usize)
            } else {
                continue;
            };
            part_exchange_rewind(list);
            let e = exe::llist_get_next(list);
            if e == 0 {
                continue;
            }
            let slot_fn: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(slot_of);
            let slot = slot_fn(tm(), read_u32(e));
            if slot != 0 && read_usize(slot) != 0 && read_u32(s.bid + 0xc) != read_u32(read_usize(slot) + 0xc) {
                continue;
            }
        }
        if exe::transfer_manager_can_club_bid_for_player(tm(), p, seller, core::ptr::null_mut(), 1) == 0 {
            continue;
        }
        if read_usize(s.human + staff::CLUB_JOB) == s.club && exe::human_manager_is_human_club(seller, 0, 1, 0, 0) == 0
            && exe::shortlist_manager_club_wants_player(read_usize(tm() + 0x856), p, seller, 0, 0) == 0 {
            covered!("tel05_2 not interested");
            s.not_interested = s.not_interested.wrapping_add(1);
            continue;
        }
        let finance = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), seller);
        let max_wage = exe::finance_get_max_wage(finance, p, 2);
        let limit = ftol(max_wage as f64 * read_f64(rdata::F64_1_5));
        if exe::staff_contract_set_to_desired_contract(contract, p, seller, -1, 2, 0xb, 0, 0) > limit {
            covered!("tel05_2 too dear");
            s.too_dear = s.too_dear.wrapping_add(1);
        } else {
            let cell = p;
            exe::llist_add_tail(s.candidates.as_mut_ptr() as usize, &cell as *const usize as usize, 4);
        }
    }
}

/// The list's area and the column headings.
///
/// Traditional view: one area 0x6e..0x30c by 0xd2..0x217 with 8 columns (two players per row) and the rows that fit in
/// font 3 (0x135 / the font's height; the row count dword keeps the filter's upper bytes, as the original). Other views:
/// the background area, the rows that fit in font 2, their height (309 / rows + 0.5, `ftol`); the headings in an area
/// from 0xda, one row high, 0x2f6 wide when there are more candidates than rows (the scroll bar), else 0x30c: "Inf" 2,
/// "   Name" 4 (link 6) and the view's columns: contract (Squad Status 0x10, Basic Wage 0x20000, Contract Expiry 0x40000,
/// Releases 0x2000000); selection (Position 8, Form 0x80, Morale 0x100, Cond. 0x200, the stat fields 1 and 0x11 as
/// 0x10000000 and 0x800000); statistics (the fields of [0x98de84] or, More Stats, [0x98de90], link 7); attributes (the
/// table of the chosen group, sorted in place by qsort with 0x487720, link 8; with no group chosen the view flags
/// themselves are taken as the table, as the original); other (Nat. 0x1000, Age 0x4000, Caps 0x8000, Goals 0x10000, Form,
/// Morale, Cond.); "Value" 0x80000 in column 0xc. Then the list area below to 0x20f, its row count the rows less one in
/// the low byte of the last heading's return (as the original). No view: the view bar.
unsafe fn part_exchange_list_area(s: &mut PartExchange, w: &mut PartExchangeWeights) -> u32 {
    let lang = part_exchange_lang();
    let v = s.view;
    if v & 1 != 0 {
        covered!("tel05_2 traditional view");
        let rows = (0x135 / exe::font_get_height(3)) as u8;
        s.filter = (s.filter & 0xffff_ff00) | rows as u32;
        return s.add_area([0x6e, 0xd2, 0x30c, 0x217, 8, w.two.as_ptr() as u32, s.filter, 0, 2, 0, u32::MAX]);
    }
    s.add_area([0x6e, 0xd2, 0x30c, 0x217, 1, 0, 1, 0, 2, 0, u32::MAX]);
    let rows = (0x135 / exe::font_get_height(2)) as u8;
    s.filter = (s.filter & 0xffff_ff00) | rows as u32;
    let row_h = ftol(read_f32(0x968974) as f64 / rows as u32 as f64 + read_f64(rdata::F64_0_5)); // 309.0 / rows + 0.5
    let count = read_i16(s.candidates.as_ptr() as usize) as i32;
    let x2 = if count > rows as i32 - 1 { 0x2f6 } else { 0x30c };
    let head = |s: &PartExchange, weights: *const u8| s.add_area([0x6e, 0xda, x2, (row_h + 0xd9) as u32, 0xd, weights as u32, 1, 0, 1, 0, u32::MAX]);
    let first_two = |s: &PartExchange, area: u32| {
        lang(0xde1f64, 0x98ea64); // "Inf"
        s.heading(area, 1, 0xc, 6, 2);
        lang(0xde1f64, 0x98ea5c); // "   Name"
        s.heading(area, 2, 1, 6, 4);
    };
    let value_heading = |s: &PartExchange, area: u32| -> u32 {
        lang(0xde1f64, 0x98ea0c); // "Value"
        s.heading(area, 0xc, 0xc, 6, 0x80000)
    };
    let stat_label: unsafe extern "thiscall" fn(usize, u32, usize, u32) = core::mem::transmute(0x7c1b20usize);
    let (weights, last): (*const u8, u32);
    if v & 2 != 0 {
        covered!("tel05_2 contract view");
        weights = w.contract.as_ptr();
        let area = head(s, weights);
        first_two(s, area);
        for (col, label, data) in [(3u32, 0x98ea4cusize, 0x10u32), (4, 0x98ea34, 0x20000), (5, 0x98ea20, 0x40000), (6, 0x98ea14, 0x2000000)] {
            lang(0xde1f64, label);
            s.heading(area, col, 1, 6, data);
        }
        last = value_heading(s, area);
    } else if v & 0x1000 != 0 {
        covered!("tel05_2 selection view");
        weights = w.selection.as_ptr();
        let area = head(s, weights);
        first_two(s, area);
        lang(0xde1f64, 0x98ea00); // "Position"
        s.heading(area, 3, 1, 6, 8);
        for (col, label, data) in [(4u32, 0x98e9f8usize, 0x80u32), (5, 0x98e9f0, 0x100), (6, 0x98e9e8, 0x200)] {
            lang(0xde1f64, label);
            s.heading(area, col, 0xc, 6, data);
        }
        stat_label(read_usize(G_PLAYER_STATS), 1, 0xde1f64, 0x7d0);
        s.heading(area, 7, 0xc, 6, 0x10000000);
        stat_label(read_usize(G_PLAYER_STATS), 0x11, 0xde1f64, 0x7d0);
        s.heading(area, 8, 0xc, 6, 0x800000);
        last = value_heading(s, area);
    } else if v & 4 != 0 || v & 8 != 0 {
        covered!("tel05_2 stats view");
        weights = w.stats.as_ptr();
        let area = head(s, weights);
        first_two(s, area);
        let table = if v & 4 != 0 { 0x98de84 } else { 0x98de90 };
        let mut k = 0usize;
        while read_u8(table + k) != 0xff {
            let field = read_u8(table + k);
            // simplification: the field goes in a register whose upper bytes are garbage in the original (a char)
            stat_label(read_usize(G_PLAYER_STATS), field as u32, 0xde1f64, 0x7d0);
            s.heading(area, k as u32 + 3, 0xc, 7, field as i8 as i32 as u32);
            k += 1;
        }
        last = value_heading(s, area);
    } else if v & 0x10 != 0 {
        covered!("tel05_2 attributes view");
        weights = w.attributes.as_ptr();
        let area = head(s, weights);
        first_two(s, area);
        // the slot of the view flags takes the table of the chosen group (none: the flags stay, used as the table)
        let mut table = s.view as usize;
        for (bit, t) in [(0x40u32, 0x98de9cusize), (0x80, 0x98dea8), (0x100, 0x98deb4), (0x200, 0x98dec0), (0x400, 0x98decc)] {
            if v & bit != 0 {
                table = t;
            }
        }
        s.view = table as u32;
        exe::crt_qsort(table, 9, 1, 0x487720usize);
        let name: unsafe extern "C" fn(u32, usize, u32, u32) -> u32 = core::mem::transmute(0x5428a0usize);
        let mut k = 0usize;
        while read_u8(table + k) != 0xff {
            let attr = read_u8(table + k);
            name(attr as u32, 0xde1f64, 0x7d0, 1);
            s.heading(area, k as u32 + 3, 0xc, 8, attr as i8 as i32 as u32);
            k += 1;
        }
        last = value_heading(s, area);
    } else if v & 0x20 != 0 {
        covered!("tel05_2 other view");
        weights = w.other.as_ptr();
        let area = head(s, weights);
        first_two(s, area);
        for (col, label, data) in [(3u32, 0x98e9e0usize, 0x1000u32), (4, 0x98e9dc, 0x4000), (5, 0x98e9d4, 0x8000), (6, 0x98e9cc, 0x10000),
                                   (7, 0x98e9f8, 0x80), (8, 0x98e9f0, 0x100), (9, 0x98e9e8, 0x200)] {
            lang(0xde1f64, label);
            s.heading(area, col, 0xc, 6, data);
        }
        last = value_heading(s, area);
    } else {
        // no view of its own: the rows go to the view bar (ebx in the original)
        return s.bar;
    }
    s.add_area([0x6e, (row_h + 0xda) as u32, 0x30c, 0x20f, 0xd, weights as u32, (last & 0xffff_ff00) | rows.wrapping_sub(1) as u32, 0, 1, 0, u32::MAX])
}

/// The rows, one per candidate, in the area `area`.
///
/// Each player's training caught up (`exe::training_manager_update_player_training`); column 0 an empty text; column 1
/// his status (`get_player_status_code`, 0x4849f0, with no fixture) as an item (flags 0x202, the colours, link and data
/// by the status): Wpm, Cup, Ine, Fgn and Sct link 0x7db; international duty with the duty club (link 0x7d5), or an item
/// 0x201 with no link and whatever text 0xde1f64 holds (as the original); Sus 0x7d9; Inj 0x7d8; Inj (8) and Tir 0x7d8 in
/// the tired colour; Yel 0x7d9 in the colour of `exe::graphics_rgb_to_pixel`(0xb4, 0xb4, 0, 0); Ctr 0x7da; Trn, Bid, Wnt,
/// Req, Lst, Loa, Ret, Bos, Unh, Fut 0x7db. Column 2 his name (style 4 and font 3 in the Traditional view, else 1 and 2)
/// as a button (link 2 while choosing, else 0x7d7 to him). The Traditional view: column 3 by the sort (`squad_list_sort_cell`),
/// two players per row (columns 0 and 4); the others: the view's columns (`squad_list_view_cells`) and the value in
/// column 0xc (a button, link 0xa, for a database player when the human runs the club), one player per row.
unsafe fn part_exchange_rows(s: &mut PartExchange, area: u32, _w: &PartExchangeWeights) {
    let list = s.candidates.as_mut_ptr() as usize;
    let (mut col, mut row) = (0u32, 0u32);
    let traditional = s.view_flags & 1 != 0;
    let mut buf = [0u8; 0x17c]; // the name buffer of the original (0x17c bytes at [esp+0x1fc])
    let rules = SquadListRules { club: s.club, sort: s.sort, view_flags: s.view_flags, category: s.category, table: s.view as usize, squad_screen: false };
    part_exchange_rewind(list);
    loop {
        let e = exe::llist_get_next(list);
        if e == 0 {
            break;
        }
        s.shown = s.shown.wrapping_add(1);
        let player = read_usize(e);
        exe::training_manager_update_player_training(read_usize(G_TRAINING_MANAGER), player, 0);
        tel26_area_text([area, col, row, 0x30, part_exchange_colour(0xaebe04), 0xc, 1, part_exchange_colour(0xaebdec), 0]);
        squad_list_status(&rules, area, player, col, row);
        let contract_club = if player == 0 || read_i32(player) < read_i32(G_STAFF_COUNT) {
            exe::contract_manager_get_contract_for_club(read_usize(G_CONTRACT_MANAGER), player, s.club)
        } else {
            0
        };
        let contract = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), player);
        let (style, font) = if traditional { (4, 3) } else { (1, 2) };
        exe::database_format_staff_name(player, 0xde1f64usize, 0x7d0, style);
        tel26_area_button([area, col + 2, row, 1, 0, 1, font, part_exchange_colour(0xae3184), 0xde1f64, if s.choosing != 0 { 2 } else { 0x7d7 },
            player as u32]);
        let injury = exe::injury_get_player_injury_status(read_usize(G_INJURY), player);
        let b = buf.as_mut_ptr() as usize;
        if traditional {
            squad_list_sort_cell(&rules, area, player, col, row, injury, contract_club, contract, b);
            col += 4;
            if col > 4 {
                row += 1;
                col = 0;
            }
        } else {
            squad_list_view_cells(&rules, area, player, col, row, injury, contract_club, contract, b);
            part_exchange_value(player);
            let (c3192, c3184) = (part_exchange_colour(0xae3192), part_exchange_colour(0xae3184));
            if (player == 0 || read_i32(player) < read_i32(G_STAFF_COUNT)) && exe::human_manager_current_controls_club(s.club, 0, 1) != 0 {
                tel26_area_button([area, col + 0xc, row, 0x30, c3192, 0xc, 1, c3184, 0xde1f64, 0xa, player as u32]);
            } else {
                tel26_area_text([area, col + 0xc, row, 0x30, c3192, 0xc, 1, c3184, 0xde1f64]);
            }
            row += 1;
        }
    }
}

/// What the row helpers need, and the differences between the part exchange screen and the squad numbers screen (the
/// latter, `squad_screen`, has national team links and its own value and contract links).
#[derive(Clone, Copy)]
struct SquadListRules {
    club: usize,
    sort: u32,
    view_flags: u32,
    category: u8,
    /// the attribute table of the attributes view (the reused view or row-count slot, as the original)
    table: usize,
    squad_screen: bool,
}

/// Column 1 of a row: the status item (see `part_exchange_rows`); on the squad numbers screen a national team's links
/// (`exe::database_is_national_team`, asked in each case as the original) are 0x7e0 (Wpm, Cup, Ine, Sct, Ret), 0x7de (Sus,
/// Yel) and 0x7dd (Inj).
unsafe fn squad_list_status(r: &SquadListRules, area: u32, player: usize, col: u32, row: u32) {
    let code: unsafe extern "C" fn(usize, usize, usize) -> u8 = core::mem::transmute(0x4849f0usize);
    let club = r.club;
    let status = code(player, club, 0);
    let lang = part_exchange_lang();
    let national = || r.squad_screen && exe::database_is_national_team(club) != 0;
    // the item: flags, the colour twice (a9, a10), font 0xc, the text colour c13, link and data
    let item = |flags: u32, a9: u32, a10: u32, c13: u32, link: u32, data: usize| {
        tel26_add_item(screens_current_display(G_SCREEN_MANAGER), [flags, 0, 0, 0, 0, col + 1, row, 0x30, a9, a10, 0xc, 1, c13, 0xde1f64, 0, link,
            data as u32, area]);
    };
    let (c3184, cbe04, cbdcc, c3190, c317c, cbdec) = (part_exchange_colour(0xae3184), part_exchange_colour(0xaebe04), part_exchange_colour(0xaebdcc),
        part_exchange_colour(0xae3190), part_exchange_colour(0xae317c), part_exchange_colour(0xaebdec));
    match status {
        1 | 2 | 3 | 0x17 => {
            match status {
                1 => lang(0xde1f64, 0x98e9c8),                // "Wpm"
                2 => lang(0xde1f64, 0x98e998, 0xad9c64usize), // "Cup"
                3 => lang(0xde1f64, 0x98e994),                // "Ine"
                _ => lang(0xde1f64, 0x98e938, 0xad9c64usize), // "Sct"
            };
            item(0x202, cbe04, cbe04, c3184, if national() { 0x7e0 } else { 0x7db }, player);
        }
        7 | 9 => {
            lang(0xde1f64, 0x98e984); // "Fgn"
            item(0x202, cbe04, cbe04, c3184, 0x7db, player);
        }
        4 => {
            // international duty: no text of its own (0xde1f64 keeps what the previous row wrote, as the original)
            let duty = exe::nationalteams_get_international_duty_club(read_usize(G_NATIONAL_TEAMS), player, G_CURRENT_DATE, club, 0, 0) as usize;
            if duty != 0 {
                item(0x202, cbe04, cbe04, c3184, 0x7d5, duty);
            } else {
                item(0x201, cbe04, cbe04, c3184, 0, 0);
            }
        }
        5 => {
            lang(0xde1f64, 0x98e990); // "Sus"
            item(0x202, cbdcc, cbdcc, c3184, if national() { 0x7de } else { 0x7d9 }, player);
        }
        6 => {
            lang(0xde1f64, 0x98e98c); // "Inj"
            item(0x202, cbdcc, cbdcc, c3184, if national() { 0x7dd } else { 0x7d8 }, player);
        }
        8 => {
            lang(0xde1f64, 0x98e98c); // "Inj"
            item(0x202, c3190, c3190, c3184, if national() { 0x7dd } else { 0x7d8 }, player);
        }
        0x16 => {
            lang(0xde1f64, 0x98e97c); // "Tir "
            item(0x202, c3190, c3190, c3184, 0x7d8, player);
        }
        0xa => {
            lang(0xde1f64, 0x98e978); // "Yel"
            let link = if national() { 0x7de } else { 0x7d9 };
            let colour = part_exchange_colour(0xaebdea);
            let first = exe::graphics_rgb_to_pixel(0xb4, 0xb4, 0, 0);
            let second = exe::graphics_rgb_to_pixel(0xb4, 0xb4, 0, 0);
            item(0x202, second, first, colour, link, player);
        }
        0x10 => {
            lang(0xde1f64, 0x98e900); // "Ctr"
            item(0x202, c317c, c317c, cbdec, 0x7da, player);
        }
        0x11 => {
            lang(0xde1f64, 0x98e8f8); // "Ret "
            item(0x202, c317c, c317c, cbdec, if national() { 0x7e0 } else { 0x7db }, player);
        }
        0xb..=0xf | 0x12..=0x14 | 0x18 => {
            match status {
                0xe => lang(0xde1f64, 0x98e908, 0xad9c64usize), // "Lst"
                _ => lang(0xde1f64, match status {
                    0x18 => 0x98e970, // "Trn"
                    0xb => 0x98e974,  // "Bid"
                    0xc => 0x98e96c,  // "Wnt"
                    0xd => 0x98e934,  // "Req"
                    0xf => 0x98e904,  // "Loa"
                    0x12 => 0x98e8f0, // "Bos "
                    0x13 => 0x98e8e8, // "Unh "
                    _ => 0x98e8e0,    // "Fut "
                }),
            };
            item(0x202, c317c, c317c, cbdec, 0x7db, player);
        }
        _ => {}
    }
}

/// Column 3 of the Traditional view, by the sort (one of them, in this order): squad number (`SQUAD_MANAGER`: when the club
/// uses numbers and he holds one, its absolute value, else empty), form, morale, condition, nationality (a button to his
/// nation's team, link 0x7d5; on the squad numbers screen for a national or all-star team, a button to his club), age,
/// caps, international goals, basic wage and contract expiry (the club's contract, else his contract, else empty),
/// protection ("Yes" when his contract is in the FIFA protected period, else "No"), value (a button for a database player
/// whose club the human runs: link 0xa, 0xd on the squad numbers screen), goals, conceded, assists, rating (the season's
/// statistics), else his positions.
#[allow(clippy::too_many_arguments)]
unsafe fn squad_list_sort_cell(r: &SquadListRules, area: u32, player: usize, col: u32, row: u32, injury: u8, contract_club: usize, contract: usize, buf: usize) {
    let sort = r.sort;
    let pdata = read_usize(player + staff::PLAYER);
    let cbdec = part_exchange_colour(0xaebdec);
    let c = col + 3;
    let cell = |a6: u32, a7: u32| {
        tel26_area_text([area, c, row, 1, 0, a6, a7, cbdec, 0xde1f64]);
    };
    let number = |n: u32| {
        part_exchange_sprintf()(0xde1f64, 0x98806c, n); // "%d"
    };
    if sort & 0x20 != 0 {
        let squad = read_usize(G_SQUAD_MANAGER);
        if exe::squad_manager_club_uses_squad_numbers(squad, r.club, 0) != 0 && exe::squad_manager_get_squad_number(squad, player, r.club) != 0 {
            let n = exe::squad_manager_get_squad_number(read_usize(G_SQUAD_MANAGER), player, r.club) as i32;
            number(n.wrapping_abs() as u32);
        } else {
            write_u8(0xde1f64, 0);
        }
        cell(0xc, 2);
    } else if sort & 0x80 != 0 {
        part_exchange_form(player);
        cell(0xc, 1);
    } else if sort & 0x100 != 0 {
        part_exchange_morale(player);
        cell(0xc, 2);
    } else if sort & 0x200 != 0 {
        part_exchange_condition(player, injury);
        cell(0xc, 2);
    } else if sort & 0x2000 != 0 {
        if !r.squad_screen || exe::nationalteams_is_national_or_all_star_team(read_usize(G_NATIONAL_TEAMS), r.club) == 0 {
            let nation = read_usize(player + staff::NATION);
            if nation != 0 {
                let team = exe::database_get_national_team(nation);
                tel26_area_button([area, c, row, 1, 0, 0xc, 2, cbdec, (nation + 0x53) as u32, 0x7d5, team as u32]);
            }
        } else {
            let club_job = read_usize(player + staff::CLUB_JOB);
            if club_job != 0 {
                tel26_area_button([area, c, row, 1, 0, 1, 2, cbdec, (club_job + club::SHORT_NAME) as u32, 0x7d5, club_job as u32]);
            }
        }
    } else if sort & 0x4000 != 0 {
        number(read_i8(player + staff::AGE) as i32 as u32);
        cell(0xc, 2);
    } else if sort & 0x8000 != 0 {
        number(read_u8(player + staff::INT_APPS) as u32);
        cell(0xc, 2);
    } else if sort & 0x10000 != 0 {
        number(read_u8(player + staff::INT_GOALS) as u32);
        cell(0xc, 2);
    } else if sort & 0x20000 != 0 {
        if contract_club != 0 || contract != 0 {
            part_exchange_wage(if contract_club != 0 { contract_club } else { contract }, buf);
        } else {
            write_u8(0xde1f64, 0);
        }
        cell(1, 2);
    } else if sort & 0x40000 != 0 {
        if contract_club != 0 || contract != 0 {
            part_exchange_end_date(if contract_club != 0 { contract_club } else { contract }, buf);
        } else {
            write_u8(0xde1f64, 0);
        }
        cell(1, 2);
    } else if sort & 0x20000000 != 0 {
        let yes = contract != 0 && exe::staff_contract_check_fifa_protected_period(contract, 0usize, 0usize, 0usize) != 0;
        part_exchange_lang()(0xde1f64, if yes { 0x98e8d0 } else { 0x98e8cc }); // "Yes", "No"
        cell(1, 2);
    } else if sort & 0x80000 != 0 {
        part_exchange_value(player);
        let club_job = read_usize(player + staff::CLUB_JOB);
        if (player == 0 || read_i32(player) < read_i32(G_STAFF_COUNT)) && club_job != 0 && exe::human_manager_current_controls_club(club_job, 0, 1) != 0 {
            tel26_area_button([area, c, row, 1, 0, 0xc, 2, cbdec, 0xde1f64, if r.squad_screen { 0xd } else { 0xa }, player as u32]);
        } else {
            cell(0xc, 2);
        }
    } else if sort & 0xf00000 != 0 {
        let field = if sort & 0x100000 != 0 { 2 } else if sort & 0x200000 != 0 { 3 } else if sort & 0x400000 != 0 { 5 } else { 0x11 };
        part_exchange_stat(pdata, 6, field);
        cell(0xc, 2);
    } else {
        part_exchange_positions(pdata, 0xde1f64, 0x7d0);
        cell(1, 2);
    }
}

/// The columns 3.. of the views other than Traditional: contract (the squad status of the club's contract as a button,
/// link 0xa (0xd on the squad numbers screen), when the human runs the club, else "  -"; on the squad numbers screen for
/// a national or all-star team his club as a button, link 0x7d5; the wage and the end date of the club's contract or
/// "  -"; the release clause of his contract (not for a national team on the squad numbers screen): a button, link 0x7da,
/// in the red colour (2) or the cell colour (1), else a text, or "  -"); selection (positions, form, morale, condition,
/// the stat fields 1 and 0x11 of category 6); statistics (the table's fields in the chosen category, the field 0x11 in the
/// text colour); attributes (each attribute of the table, `exe::staff_get_player_attribute`); other (the nation as a
/// button to its team, age, caps, international goals, form, morale, condition).
#[allow(clippy::too_many_arguments)]
unsafe fn squad_list_view_cells(r: &SquadListRules, area: u32, player: usize, col: u32, row: u32, injury: u8, contract_club: usize, contract: usize, buf: usize) {
    let pdata = read_usize(player + staff::PLAYER);
    let (cbdec, cbde8, cbdd4, c31a8) = (part_exchange_colour(0xaebdec), part_exchange_colour(0xaebde8), part_exchange_colour(0xaebdd4), part_exchange_colour(0xae31a8));
    let cell = |c: u32, a6: u32, a7: u32, colour: u32, text: usize| {
        tel26_area_text([area, col + c, row, 1, 0, a6, a7, colour, text as u32]);
    };
    let sprintf = part_exchange_sprintf();
    let number = |n: u32| {
        sprintf(0xde1f64, 0x98806c, n); // "%d"
    };
    let flags = r.view_flags;
    if flags & 2 != 0 {
        if !r.squad_screen || exe::nationalteams_is_national_or_all_star_team(read_usize(G_NATIONAL_TEAMS), r.club) == 0 {
            if exe::human_manager_current_controls_club(r.club, 0, 1) != 0 && contract_club != 0 {
                let status: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x867ef0usize);
                status(contract_club, buf, 0x7d0);
                sprintf(0xde1f64, 0x987ff0, buf); // "  %s"
                tel26_area_button([area, col + 3, row, 1, 0, 1, 2, cbde8, 0xde1f64, if r.squad_screen { 0xd } else { 0xa }, player as u32]);
            } else {
                cell(3, 1, 2, cbde8, 0x98e8c8); // "  -"
            }
        } else {
            let club_job = read_usize(player + staff::CLUB_JOB);
            if club_job != 0 {
                sprintf(0xde1f64, 0x987ff0, club_job + club::SHORT_NAME);
                tel26_area_button([area, col + 3, row, 1, 0, 1, 2, cbde8, 0xde1f64, 0x7d5, club_job as u32]);
            }
        }
        if contract_club != 0 {
            part_exchange_wage(contract_club, buf);
        } else {
            sprintf(0xde1f64, 0x98e8c8);
        }
        cell(4, 1, 2, cbdec, 0xde1f64);
        if contract_club != 0 {
            part_exchange_end_date(contract_club, buf);
        } else {
            sprintf(0xde1f64, 0x98e8c8);
        }
        cell(5, 1, 2, cbdec, 0xde1f64);
        if r.squad_screen && exe::database_is_national_team(r.club) != 0 {
            return;
        }
        if contract != 0 {
            let release: unsafe extern "thiscall" fn(usize, usize, u32) -> u8 = core::mem::transmute(0x868020usize);
            let k = release(contract, buf, 0x7d0);
            sprintf(0xde1f64, 0x987ff0, buf);
            if k == 2 {
                tel26_area_button([area, col + 6, row, 1, 0, 1, 2, c31a8, 0xde1f64, 0x7da, player as u32]);
            } else if k == 1 {
                tel26_area_button([area, col + 6, row, 1, 0, 1, 2, cbdd4, 0xde1f64, 0x7da, player as u32]);
            } else {
                cell(6, 1, 2, cbdd4, 0xde1f64);
            }
        } else {
            cell(6, 1, 2, cbdd4, 0x98e8c8);
        }
    } else if flags & 0x1000 != 0 {
        part_exchange_positions(pdata, buf, 0xc8);
        sprintf(0xde1f64, 0x987ff0, buf);
        cell(3, 1, 2, cbde8, 0xde1f64);
        part_exchange_form(player);
        cell(4, 0xc, 1, cbdec, 0xde1f64);
        part_exchange_morale(player);
        cell(5, 0xc, 2, cbdec, 0xde1f64);
        part_exchange_condition(player, injury);
        cell(6, 0xc, 2, cbdd4, 0xde1f64);
        part_exchange_stat(pdata, 6, 1);
        cell(7, 0xc, 2, cbde8, 0xde1f64);
        part_exchange_stat(pdata, 6, 0x11);
        cell(8, 0xc, 2, cbde8, 0xde1f64);
    } else if flags & 4 != 0 || flags & 8 != 0 {
        let table = if flags & 4 != 0 { 0x98de84 } else { 0x98de90 };
        let mut k = 0usize;
        while read_u8(table + k) != 0xff {
            let field = read_u8(table + k);
            // simplification: param 6 and the field are bytes in a stack slot and a register whose upper bytes are
            // garbage in the original (chars)
            part_exchange_stat(pdata, r.category as u32, field as u32);
            cell(k as u32 + 3, 0xc, 2, if field == 0x11 { cbdec } else { cbde8 }, 0xde1f64);
            k += 1;
        }
    } else if flags & 0x10 != 0 {
        let table = r.table;
        let mut k = 0usize;
        while read_u8(table + k) != 0xff {
            let attr = read_u8(table + k);
            number(exe::staff_get_player_attribute(player, attr as u32) as i32 as u32);
            cell(k as u32 + 3, 0xc, 2, cbde8, 0xde1f64);
            k += 1;
        }
    } else if flags & 0x20 != 0 {
        let nation = read_usize(player + staff::NATION);
        if nation != 0 {
            let team = exe::database_get_national_team(nation);
            tel26_area_button([area, col + 3, row, 1, 0, 0xc, 2, cbde8, (nation + 0x53) as u32, 0x7d5, team as u32]);
        }
        number(read_i8(player + staff::AGE) as i32 as u32);
        cell(4, 0xc, 2, cbde8, 0xde1f64);
        number(read_u8(player + staff::INT_APPS) as u32);
        cell(5, 0xc, 2, cbde8, 0xde1f64);
        number(read_u8(player + staff::INT_GOALS) as u32);
        cell(6, 0xc, 2, cbde8, 0xde1f64);
        part_exchange_form(player);
        cell(7, 0xc, 1, cbdec, 0xde1f64);
        part_exchange_morale(player);
        cell(8, 0xc, 2, cbdec, 0xde1f64);
        part_exchange_condition(player, injury);
        cell(9, 0xc, 2, cbdd4, 0xde1f64);
    }
}

/// The end of the screen: when no candidate was shown and the bid offers nobody yet while the club's list was not empty,
/// the reason as a wrapped text in the list's place ("<seller> are not interested in a part-exchange deal." or the
/// wage demands one, with the clubs' names linked, both to the seller, as the original; `gui_add_text_area_from_text`,
/// font 3), otherwise the text buffer emptied. The list area's scroll row from param 1 (`Area::set_scroll_row`), the area
/// written to param 2, and the bottom bar (0x64..0x316 by 0x22b..0x24e, weights 3 and 1): "Back" (link 3) and "Next"
/// (link -3, or a greyed text when there is no next screen).
unsafe fn part_exchange_footer(s: &mut PartExchange, area: u32) {
    let lang = part_exchange_lang();
    let mut said = false;
    if s.shown == 0 && read_usize(s.bid + 4) == 0 && s.squad_head as u16 != 0 {
        let seller = s.seller;
        let gender = |c: usize| read_i8(c + club::SHORT_GENDER_NAME) as i32;
        if s.not_interested != 0 {
            lang(0xde1f64, 0x991438, gender(seller), gender(seller), seller + club::SHORT_NAME);
            said = true;
        } else if s.too_dear != 0 {
            lang(0xde1f64, 0x9913c8, gender(s.club), gender(s.club), gender(seller), gender(seller), s.club + club::SHORT_NAME, seller + club::SHORT_NAME);
            said = true;
        }
        if said && read_u8(0xde1f64) != 0 {
            covered!("tel05_2 nobody to offer");
            let mut text = [0usize; 4];
            exe::new_text(text.as_mut_ptr(), 0xde1f64usize);
            let t = text.as_mut_ptr() as usize;
            exe::text_add_link(t, (seller + club::SHORT_NAME) as *const u8, 0x7d5, seller);
            exe::text_add_link(t, (s.club + club::SHORT_NAME) as *const u8, 0x7d5, seller);
            let lines = 0x145 / exe::font_get_height(3);
            let wrapped: unsafe extern "C" fn(u32, u32, u32, u32, u32, u32, i32, u32, u32, usize) -> u32 = core::mem::transmute(0x5ed200usize);
            wrapped(0x6e, 0xd2, 0x30c, 0x217, 1, 0, lines, 3, part_exchange_colour(0xae3184), t);
            exe::drop_text(text.as_mut_ptr());
        }
    }
    if !said {
        write_u8(0xde1f64, 0);
    }
    let scroll = get_screen_param(1);
    if area as u16 != 0xffff {
        let d = s.display();
        exe::area_set_scroll_row(d.wrapping_add(4).wrapping_add((area as i16 as isize as usize).wrapping_mul(0xbf1)), scroll);
    }
    set_screen_param(2, area as i16 as i32 as usize);
    let weights = [3u8, 1];
    let bottom = s.add_area([0x64, 0x22b, 0x316, 0x24e, 2, weights.as_ptr() as u32, 1, 0, 1, 0, u32::MAX]);
    let (c317e, c3184) = (part_exchange_colour(0xae317e), part_exchange_colour(0xae3184));
    lang(0xde1f64, 0x9880a8); // "Back"
    tel26_area_button([bottom, 0, 0, 0x30, c317e, 0xc, 3, c3184, 0xde1f64, 3, 0]);
    lang(0xde1f64, 0x9880a0); // "Next"
    if exe::screenmanager_is_forward_unavailable(G_SCREEN_MANAGER) == 0 {
        tel26_area_button([bottom, 1, 0, 0x30, c317e, 0xc, 3, c3184, 0xde1f64, 0xffff_fffd, 0]);
    } else {
        tel26_area_text([bottom, 1, 0, 0x30, c317e, 0x2c, 3, c3184, 0xde1f64]);
    }
}

/// Comparer for `part_exchange_screen_build`.
///
/// Through the display list (`compare_by_display_list_with`), plus [0xae1080] (the player's contract, written), the
/// attribute tables the attributes view sorts in place (0x98de84..0x98ded8) and the bid's offered list (its walk). What
/// it also does outside the list runs twice and is idempotent: the background picture, each candidate's training caught
/// up to the week, the local lists and contract.
pub(crate) unsafe extern "C" fn compare_part_exchange_screen_build() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I0%%);
    let mut extra = vec![(G_CONTRACT_LOOKUP_RESULT, 4usize), (0x98de84usize, 0x54usize)];
    let offered = read_usize(get_screen_param(0) + 4);
    if offered != 0 {
        extra.push((offered, 0x10));
    }
    compare_by_display_list_with("part_exchange_screen_build (part exchange screen)", &extra, || o(), || part_exchange_screen_build());
}
