// ---------- batch %%LEVA%%: the compare players screen (staff_screens.cpp, part 1) ----------

/// The rows of the compare players tables (`database_get_player_attribute_name` fills the name, the byte after it holds
/// the value): the first player's values and the code of the extra rows (0xddf570), the second player's (0xdda680),
/// the differences (0xddcdf8) and the "masked" marks (0xdd7f6c, the byte of each 101-byte row). Rows of 0x65 bytes.
const COMPARE_ROW: usize = 0x65;
const COMPARE_TABLE_FIRST: usize = 0xddf570;
const COMPARE_TABLE_SECOND: usize = 0xdda680;
const COMPARE_TABLE_DIFF: usize = 0xddcdf8;
const COMPARE_MASKED: usize = 0xdd7f6c;
/// The position comparison words of the compare screen (i16, positive = the first player better) and their "both can
/// play it" marks (u8): goalkeeper, centre back, full back, wing back, sweeper, defensive midfielder, midfielder,
/// winger, plus the two partial sums (defence and attack) and the striker; the attacking midfielder pair lives apart
/// (0xdda67c / 0xdda67e). 0xde1d04 is set when both players are goalkeepers.
const COMPARE_POS_GK: usize = 0xde1ce4;
const COMPARE_POS_CB: usize = 0xde1ce6;
const COMPARE_POS_FB: usize = 0xde1ce8;
const COMPARE_POS_WB: usize = 0xde1cea;
const COMPARE_POS_SW: usize = 0xde1cec;
const COMPARE_POS_DM: usize = 0xde1cee;
const COMPARE_POS_MC: usize = 0xde1cf0;
const COMPARE_POS_WG: usize = 0xde1cf2;
const COMPARE_POS_DEFENCE: usize = 0xde1cf4;
const COMPARE_POS_ATTACK: usize = 0xde1cf6;
const COMPARE_POS_ST: usize = 0xde1cf8;
const COMPARE_POS_AM: usize = 0xdda67c;
const COMPARE_MARK_GK: usize = 0xde1cfa;
const COMPARE_MARK_CB: usize = 0xde1cfb;
const COMPARE_MARK_FB: usize = 0xde1cfc;
const COMPARE_MARK_WB: usize = 0xde1cfd;
const COMPARE_MARK_SW: usize = 0xde1cfe;
const COMPARE_MARK_DM: usize = 0xde1cff;
const COMPARE_MARK_MC: usize = 0xde1d00;
const COMPARE_MARK_WG: usize = 0xde1d01;
const COMPARE_MARK_ST: usize = 0xde1d02;
const COMPARE_MARK_AM: usize = 0xdda67e;
const COMPARE_BOTH_GOALKEEPERS: usize = 0xde1d04;

/// The CRT's inline strcpy of the original (dwords then bytes: the same bytes).
unsafe fn compare_screen_strcpy(dst: usize, src: usize) {
    let mut i = 0;
    loop {
        let c = read_u8(src + i);
        write_u8(dst + i, c);
        if c == 0 {
            break;
        }
        i += 1;
    }
}

/// The CRT's inline strcat of the original.
unsafe fn compare_screen_strcat(dst: usize, src: usize) {
    let mut n = 0;
    while read_u8(dst + n) != 0 {
        n += 1;
    }
    compare_screen_strcpy(dst + n, src);
}

/// The CRT's inline strcmp of the original, as "equal".
unsafe fn compare_screen_streq(a: usize, b: usize) -> bool {
    let mut i = 0;
    loop {
        let (x, y) = (read_u8(a + i), read_u8(b + i));
        if x != y {
            return false;
        }
        if x == 0 {
            return true;
        }
        i += 1;
    }
}

/// `SCOUT_MANAGER::get_compare_player` (0x7f7920, thiscall, ret 4): compare player 1 or 2 of the current human.
unsafe fn compare_screen_player(which: u32) -> usize {
    let f: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(0x7f7920usize);
    f(read_usize(G_SCOUT_MANAGER), which)
}

/// `Text::draw_wrapped` (0x4aeaf0, thiscall, ret 0x20): draws a Text into area `area` from x1 to x2, starting at row
/// `*row` (which it moves on), with the font, the colour and two more flags.
type TextDrawWrappedFn = unsafe extern "thiscall" fn(*mut usize, usize, usize, usize, *mut i32, usize, usize, usize, usize) -> usize;

/// `Display::add_area` on the current human's Display, parent −1: the boxes and tables of the compare screen.
unsafe fn compare_screen_area(x1: i32, y1: i32, x2: i32, y2: i32, cols: u32, align: usize, rows: u32, a8: u32, a9: u32, colour: usize) -> usize {
    let f: DisplayListFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    f(current_human_display(), x1 as usize, y1 as usize, x2 as usize, y2 as usize, cols as usize, align, rows as usize, a8 as usize, a9 as usize, colour, usize::MAX)
}

/// `ScreenManager::add_area_text` of the compare screen: cell (`col`, `row`) of table `area` (the fixed 2 and 0, then
/// the font flag `font` and the style `style`).
unsafe fn compare_screen_cell(area: usize, col: u32, row: i32, font: u32, style: u32, colour: usize, text: usize) -> usize {
    let f: ListCellTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
    f(SCREEN_MANAGER, area, col as usize, row as usize, 2, 0, font as usize, style as usize, colour, text)
}

/// The nationality line of one player of the compare screen into `out` (200 bytes): the adjective of his nation
/// (`exe::nation_get_nationality_adjective` with 0xc, the first letter upper-cased by the CRT `toupper` 0x9471cd)
/// and, when he has a second nation that is not the first one, has no international caps (`staff::INT_APPS` 0) and is
/// not Portugal (`KEY_NATION_PORTUGAL`), "/" and the second adjective, also capitalised (`extra`, 200 bytes, keeps
/// it). No nation: empty.
unsafe fn compare_screen_nationality(p: usize, out: &mut [u8; 0xc8], extra: &mut [u8; 0xc8]) {
    let toupper: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x9471cdusize);
    let nation = read_usize(p + staff::NATION);
    let second = read_usize(p + staff::SECOND_NATION);
    if nation != 0 && second != nation && second != 0 && read_u8(p + staff::INT_APPS) == 0 && read_i32(second) != read_i32(KEY_NATION_PORTUGAL) {
        exe::nation_get_nationality_adjective(nation, out.as_mut_ptr() as usize, 0xc8, 0xc);
        out[0] = toupper(out[0] as i8 as i32) as u8;
        exe::nation_get_nationality_adjective(second, extra.as_mut_ptr() as usize, 0xc8, 0xc);
        extra[0] = toupper(extra[0] as i8 as i32) as u8;
        compare_screen_strcat(out.as_mut_ptr() as usize, 0x9b7630); // "/"
        compare_screen_strcat(out.as_mut_ptr() as usize, extra.as_ptr() as usize);
    } else if nation != 0 {
        exe::nation_get_nationality_adjective(nation, out.as_mut_ptr() as usize, 0xc8, 0xc);
        out[0] = toupper(out[0] as i8 as i32) as u8;
    } else {
        out[0] = 0;
    }
}

/// One position of the compare screen's table: first player rating `a`, second `b` (the ratings 1..20 of the player
/// sheet; 15 = can play there). Both can (≥ 15 for the first, ≤ 15 for the second: the first is better, 35 (0x23) in
/// `word`; the other way round (the first ≤ 15 and the second ≥ 15): −35; both above 15: twice the difference, and 1
/// in `extra` (the goalkeepers' 0xde1d04; the others pass their own mark, already 1); both below 15: the mark `mark`
/// goes to 0 ("n/a").
/// Original: part of the compare players screen builder (0x876520, staff_screens.cpp).
unsafe fn compare_screen_position(a: i8, b: i8, word: usize, mark: usize, extra: usize) {
    const CAN_PLAY: i8 = 15; // tel07_2 the "can play there" rating
    if a >= CAN_PLAY && b <= CAN_PLAY {
        write_u16(word, 0x23);
    } else if a <= CAN_PLAY && b >= CAN_PLAY {
        write_u16(word, 0xffdd);
    } else if a > CAN_PLAY && b > CAN_PLAY {
        write_u16(word, ((a as i16 - b as i16) * 2) as u16);
        write_u8(extra, 1);
    } else {
        write_u8(mark, 0);
    }
}

/// The heading and the identity lines of the compare players screen: the background photo of the first player, the
/// two names ("%s ~ %s", each linked 0x7d7), the menu bar, the ages and nationalities ("%s. %s ~ %s. %s", the
/// nations linked 0x7d5 to their national teams) and the positions ("%s ~ %s" of
/// `database_format_player_positions`) or, for a non-player or in mode 1, the first one's job ("Unemployed").
/// Original: part of the compare players screen builder (0x876520, staff_screens.cpp).
unsafe fn compare_screen_heading(text: *mut usize, p1: usize, p2: usize, mode: i32) {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let lang: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let draw: TextDrawWrappedFn = core::mem::transmute(0x4aeaf0usize);
    let clear: unsafe extern "thiscall" fn(*mut usize) = core::mem::transmute(0x4afa40usize);
    let short_date: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> usize = core::mem::transmute(0x54a650usize);
    let positions: unsafe extern "C" fn(usize, usize, u32, u32) -> usize = core::mem::transmute(0x53ee50usize);
    let text_item: ManagerTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_TEXT_ITEM);
    const BUF: usize = 0xde1f64;
    let (mut name1, mut name2) = ([0u8; 0xc8], [0u8; 0xc8]);
    let (n1, n2) = (name1.as_mut_ptr() as usize, name2.as_mut_ptr() as usize);

    let picture: unsafe extern "C" fn(usize, u32, u32) = core::mem::transmute(0x41c790usize);
    picture(p1, 0, 0);
    exe::database_format_staff_name(p1, n1, 0xc8, 1);
    if p2 != 0 {
        exe::database_format_staff_name(p2, n2, 0xc8, 1);
    }
    // simplification: with no second player the original prints its stack garbage after the "~"; here empty
    format(BUF, 0xa859d0, n1, n2); // "%s ~ %s"
    exe::text_set_string(text as usize, BUF as *const u8);
    if p1 != 0 {
        exe::text_add_link(text as usize, n1 as *const u8, 0x7d7, p1);
    }
    if p2 != 0 {
        exe::text_add_link(text as usize, n2 as *const u8, 0x7d7, p2);
    }
    // the colours are words in dword slots: the high half is stack garbage in the original, here 0
    let red = colour_word(G_GUI_COLOUR_RED);
    let grey = colour_word(G_GUI_COLOUR_PALE_GREY);
    compare_screen_area(0x64, 0xa, 0x316, 0x46, 1, 0, 1, 0, 0x30, red);
    let wide = exe::font_text_width(4, BUF as *const u8) >= 0x2b2;
    let mut row: i32 = 0;
    let a = compare_screen_area(0x64, 0x19, 0x316, 0x37, 1, 0, 1, 0, 1, red);
    draw(text, a, 0x64, 0x316, &mut row, if wide { 6 } else { 7 }, grey, 3, 0);
    exe::menubar_build(4, 0);
    // the date of birth goes into the first name buffer and is never shown (the positions overwrite it)
    short_date(p1 + staff::DATE_OF_BIRTH, n1, 0xc8, 0xff);

    let (mut age1, mut age2) = ([0u8; 0xc8], [0u8; 0xc8]);
    lang(age1.as_mut_ptr() as usize, 0xa859c0, read_i8(p1 + staff::AGE) as i32); // "Age <%d - age>"
    lang(age2.as_mut_ptr() as usize, 0xa859c0, read_i8(p2 + staff::AGE) as i32);
    let (mut nat1, mut nat2, mut extra1, mut extra2) = ([0u8; 0xc8], [0u8; 0xc8], [0u8; 0xc8], [0u8; 0xc8]);
    compare_screen_nationality(p1, &mut nat1, &mut extra1);
    compare_screen_nationality(p2, &mut nat2, &mut extra2);
    format(BUF, 0xa859b0, age1.as_ptr() as usize, nat1.as_ptr() as usize, age2.as_ptr() as usize, nat2.as_ptr() as usize); // "%s. %s ~ %s. %s"
    clear(text);
    exe::text_set_string(text as usize, BUF as *const u8);
    // the links carry the lower-case adjective (the line has it capitalised), as in the original
    for (p, buf) in [(p1, &mut nat1), (p2, &mut nat2)] {
        let nation = read_usize(p + staff::NATION);
        if nation != 0 {
            exe::nation_get_nationality_adjective(nation, buf.as_mut_ptr() as usize, 0xc8, 0xc);
            let team = exe::database_get_national_team(nation);
            exe::text_add_link(text as usize, buf.as_ptr(), 0x7d5, team);
        }
    }
    let a = compare_screen_area(0x6e, 0x50, 0x30c, 0x6e, 1, 0, 1, 0, 2, 0);
    row = 0;
    draw(text, a, 0x6e, 0x30c, &mut row, 3, colour_word(G_GUI_COLOUR_YELLOW), 3, 0);

    if read_usize(p1 + staff::PLAYER) != 0 && mode != 1 {
        positions(read_usize(p1 + staff::PLAYER), n1, 0x7d0, 1);
        positions(read_usize(p2 + staff::PLAYER), n2, 0x7d0, 1);
        format(BUF, 0xa859d0, n1, n2); // "%s ~ %s"
    } else if read_usize(p1 + staff::CLUB_JOB) != 0 {
        compare_screen_strcpy(BUF, exe::database_get_job_name(read_u8(p1 + staff::JOB_FOR_CLUB) as u32, 0, 1, core::ptr::null_mut()));
    } else if read_usize(p1 + staff::NATIONAL_JOB) != 0 {
        compare_screen_strcpy(BUF, exe::database_get_job_name(read_u8(p1 + staff::JOB_FOR_NATION) as u32, 0, 1, core::ptr::null_mut()));
    } else {
        lang(BUF, 0x9b6b04); // "Unemployed"
    }
    text_item(SCREEN_MANAGER, 0x6e, 0x203, 0x30c, 0x221, 2, 0, 0xc, 3, colour_word(0xaebde8), BUF, 1);
}

/// The position table of the compare screen (the first player is a player of the database): the position words and
/// marks from the two player sheets (`compare_screen_position`; the full back and the winger by their own rules), then
/// the 31 attributes into the three tables, each adding its difference to the positions it matters for, and the two
/// partial sums (defence, attack) spread over the outfield positions.
///
/// Full back (the defender rating with a side, left or right): "n/a" when neither is a defender or nobody has a side;
/// 35 when the first is a full back and the second has no side and is no defender; −35 when the second is a full back,
/// the first has no side and is no defender; otherwise twice the defender difference, except that a first player with
/// a side but no defender rating against a second with both gives 0 and "n/a". Winger (midfielder or attacking
/// midfielder with a side): "n/a" when neither, ±35 when only one, twice the attacking midfielder difference when both.
/// Attribute values: with masking on (`G_ATTRIBUTE_MASKING_ENABLED`), what the viewer (the screen record's human,
/// `G_SCREEN_RECORDS_MANAGER` of the slot) knows (`staff_get_player_attribute_for_viewer`, negative = masked: the row
/// is marked and adds nothing); otherwise `exe::staff_compute_player_attribute`.
/// Original: part of the compare players screen builder (0x876520, staff_screens.cpp).
unsafe fn compare_screen_positions(p1: usize, p2: usize) {
    let viewer_value: unsafe extern "C" fn(usize, usize, u32) -> u8 = core::mem::transmute(0x543440usize);
    let attribute_name: unsafe extern "C" fn(u32, usize, u32, u32) -> usize = core::mem::transmute(0x5428a0usize);
    let (a, b) = (read_usize(p1 + staff::PLAYER), read_usize(p2 + staff::PLAYER));
    let ra = |o: usize| read_i8(a + o);
    let rb = |o: usize| read_i8(b + o);
    for w in [COMPARE_POS_GK, COMPARE_POS_CB, COMPARE_POS_FB, COMPARE_POS_SW, COMPARE_POS_WB, COMPARE_POS_DM, COMPARE_POS_MC, COMPARE_POS_AM, COMPARE_POS_WG, COMPARE_POS_DEFENCE, COMPARE_POS_ATTACK, COMPARE_POS_ST] {
        write_u16(w, 0);
    }
    for m in [COMPARE_MARK_GK, COMPARE_MARK_CB, COMPARE_MARK_FB, COMPARE_MARK_SW, COMPARE_MARK_WB, COMPARE_MARK_DM, COMPARE_MARK_MC, COMPARE_MARK_AM, COMPARE_MARK_WG, COMPARE_MARK_ST] {
        write_u8(m, 1);
    }
    write_u32(COMPARE_BOTH_GOALKEEPERS, 0);
    let viewer = read_usize(G_SCREEN_RECORDS_MANAGER + read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize as usize * 0x300);
    const T: i8 = 15;
    use player_sheet as ps;
    compare_screen_position(ra(ps::GOALKEEPER), rb(ps::GOALKEEPER), COMPARE_POS_GK, COMPARE_MARK_GK, COMPARE_BOTH_GOALKEEPERS);
    compare_screen_position(ra(ps::DEFENDER), rb(ps::DEFENDER), COMPARE_POS_CB, COMPARE_MARK_CB, COMPARE_MARK_CB);
    compare_screen_position(ra(ps::SWEEPER), rb(ps::SWEEPER), COMPARE_POS_SW, COMPARE_MARK_SW, COMPARE_MARK_SW);

    // the full back (0x876fc1..0x8770b6 of the original, branch for branch)
    let (ad, al, ar) = (ra(ps::DEFENDER), ra(ps::LEFT_SIDE), ra(ps::RIGHT_SIDE));
    let (bd, bl, br) = (rb(ps::DEFENDER), rb(ps::LEFT_SIDE), rb(ps::RIGHT_SIDE));
    let a_side = al >= T || ar >= T;
    let b_side = bl >= T || br >= T;
    'full_back: {
        if (ad < T && bd < T) || (!a_side && !b_side) {
            write_u8(COMPARE_MARK_FB, 0);
            break 'full_back;
        }
        if ad >= T && a_side && bl < T && br < T && bd < T {
            write_u16(COMPARE_POS_FB, 0x23);
            break 'full_back;
        }
        let mut difference = false;
        if bd >= T && b_side && !a_side {
            if ad >= T {
                difference = true;
            } else {
                write_u16(COMPARE_POS_FB, 0xffdd);
                break 'full_back;
            }
        }
        if !difference {
            difference = ad >= T || !a_side || bd >= T || !b_side;
        }
        if difference {
            write_u8(COMPARE_MARK_FB, 1);
            write_u16(COMPARE_POS_FB, ((ad as i16 - bd as i16) * 2) as u16);
        } else {
            write_u16(COMPARE_POS_FB, 0);
            write_u8(COMPARE_MARK_FB, 0);
        }
    }
    compare_screen_position(ra(ps::WING_BACK), rb(ps::WING_BACK), COMPARE_POS_WB, COMPARE_MARK_WB, COMPARE_MARK_WB);
    compare_screen_position(ra(ps::MIDFIELDER), rb(ps::MIDFIELDER), COMPARE_POS_MC, COMPARE_MARK_MC, COMPARE_MARK_MC);
    compare_screen_position(ra(ps::DEFENSIVE_MIDFIELDER), rb(ps::DEFENSIVE_MIDFIELDER), COMPARE_POS_DM, COMPARE_MARK_DM, COMPARE_MARK_DM);
    compare_screen_position(ra(ps::ATTACKING_MIDFIELDER), rb(ps::ATTACKING_MIDFIELDER), COMPARE_POS_AM, COMPARE_MARK_AM, COMPARE_MARK_AM);
    // the winger (0x87728f..0x8773a4)
    let winger = |m: i8, am: i8, l: i8, r: i8| (m >= T || am >= T) && (l >= T || r >= T);
    let wa = winger(ra(ps::MIDFIELDER), ra(ps::ATTACKING_MIDFIELDER), ra(ps::LEFT_SIDE), ra(ps::RIGHT_SIDE));
    let wb = winger(rb(ps::MIDFIELDER), rb(ps::ATTACKING_MIDFIELDER), rb(ps::LEFT_SIDE), rb(ps::RIGHT_SIDE));
    match (wa, wb) {
        (false, false) => write_u8(COMPARE_MARK_WG, 0),
        (true, false) => write_u16(COMPARE_POS_WG, 0x23),
        (false, true) => write_u16(COMPARE_POS_WG, 0xffdd),
        (true, true) => {
            write_u8(COMPARE_MARK_WG, 1);
            write_u16(COMPARE_POS_WG, ((ra(ps::ATTACKING_MIDFIELDER) as i16 - rb(ps::ATTACKING_MIDFIELDER) as i16) * 2) as u16);
        }
    }
    compare_screen_position(ra(ps::ATTACKER), rb(ps::ATTACKER), COMPARE_POS_ST, COMPARE_MARK_ST, COMPARE_MARK_ST);

    // the 31 attributes (rows 0..30)
    let add = |w: usize, v: i16| write_u16(w, read_u16(w).wrapping_add(v as u16));
    for row in 0..31u8 {
        let attr = row as u32 + 1;
        let k = row as usize * COMPARE_ROW;
        // simplification: the original pushes the attribute as a dword whose high bytes are stack garbage (the
        // variable is a char, the callees read the byte); here zero-extended
        attribute_name(attr, COMPARE_TABLE_FIRST + k, 0x64, 0);
        write_u8(COMPARE_MASKED + k, 0);
        if read_u8(G_ATTRIBUTE_MASKING_ENABLED) != 0 {
            if p1 != 0 && p2 != 0 {
                let v2 = viewer_value(viewer, p2, attr);
                let v1 = viewer_value(viewer, p1, attr);
                write_u8(COMPARE_TABLE_DIFF + 0x64 + k, v1.wrapping_sub(v2));
                write_u8(COMPARE_TABLE_SECOND + 0x64 + k, viewer_value(viewer, p2, attr));
                let v1 = viewer_value(viewer, p1, attr);
                write_u8(COMPARE_TABLE_FIRST + 0x64 + k, v1);
                write_u8(COMPARE_MASKED + k, ((v1 as i8) < 0) as u8);
                if read_i8(COMPARE_TABLE_SECOND + 0x64 + k) < 0 {
                    write_u8(COMPARE_MASKED + k, 1);
                }
            }
        } else if p1 != 0 && p2 != 0 {
            let v2 = exe::staff_compute_player_attribute(p2, read_usize(p2 + staff::PLAYER), attr);
            let v1 = exe::staff_compute_player_attribute(p1, read_usize(p1 + staff::PLAYER), attr);
            write_u8(COMPARE_TABLE_DIFF + 0x64 + k, v1.wrapping_sub(v2));
            write_u8(COMPARE_TABLE_SECOND + 0x64 + k, exe::staff_compute_player_attribute(p2, read_usize(p2 + staff::PLAYER), attr));
            write_u8(COMPARE_TABLE_FIRST + 0x64 + k, exe::staff_compute_player_attribute(p1, read_usize(p1 + staff::PLAYER), attr));
        }
        if read_u8(COMPARE_MASKED + k) != 0 {
            continue;
        }
        let d = read_i8(COMPARE_TABLE_FIRST + 0x64 + k) as i16 - read_i8(COMPARE_TABLE_SECOND + 0x64 + k) as i16;
        if matches!(row, 0 | 2 | 0x10 | 0x14 | 0x15 | 0x16 | 0x17 | 0x1a) {
            add(COMPARE_POS_GK, d);
        }
        if row == 0xd {
            add(COMPARE_POS_GK, d * 2); // tel07_2 the goalkeeper's doubled attribute
            continue;
        }
        if matches!(row, 0xe | 0x10 | 0x14 | 0x19 | 0x16 | 0x1a) {
            add(COMPARE_POS_DEFENCE, d);
        }
        if matches!(row, 0x12 | 0x1b) {
            add(COMPARE_POS_CB, d * 2);
        }
        if matches!(row, 0x12 | 0x15 | 0) {
            add(COMPARE_POS_FB, d * 2);
        }
        if matches!(row, 0x12 | 0x15 | 0x1b) {
            add(COMPARE_POS_SW, d * 2);
        }
        if matches!(row, 0 | 0x15 | 7) {
            add(COMPARE_POS_WB, d * 2);
        }
        if matches!(row, 0 | 0xe | 0x10 | 0x14 | 0x19 | 0x1a | 9 | 0x1c) {
            add(COMPARE_POS_ATTACK, d);
        }
        if matches!(row, 0x1b | 0x12 | 0x15) {
            add(COMPARE_POS_DM, d * 2);
        }
        if matches!(row, 0xa | 0x15 | 0x19 | 0x1d) {
            add(COMPARE_POS_MC, d * 2);
        }
        if matches!(row, 6 | 0xa | 0x13 | 0x11 | 0x15) {
            add(COMPARE_POS_AM, d * 2);
        }
        if matches!(row, 6 | 7 | 0x14 | 0xa) {
            add(COMPARE_POS_WG, d * 2);
        }
        if matches!(row, 6 | 7 | 0xa | 0xc | 0x13 | 0x15 | 0xe | 0x10 | 0x18) {
            add(COMPARE_POS_ST, d);
        }
        if matches!(row, 0xb | 0x11 | 0x14) {
            add(COMPARE_POS_ST, d * 2);
        }
    }
    let defence = read_u16(COMPARE_POS_DEFENCE) as i16;
    for w in [COMPARE_POS_CB, COMPARE_POS_FB, COMPARE_POS_WB, COMPARE_POS_SW] {
        add(w, defence);
    }
    let attack = read_u16(COMPARE_POS_ATTACK) as i16;
    for w in [COMPARE_POS_DM, COMPARE_POS_MC, COMPARE_POS_AM, COMPARE_POS_WG] {
        add(w, attack);
    }
}

/// One star cell of the compare screen's position table: "n/a" (grey) when the mark is not 1; otherwise the word in
/// stars, "  -" (the "same" colour) between −6 and 6, one to five stars by the bands 6/12/19/27/36 in the colour of
/// the better player (`better1` positive, `better2` negative).
unsafe fn compare_screen_stars(area: usize, col: u32, row: i32, mark: usize, word: usize, better1: usize, same: usize, better2: usize) {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let lang: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    const BUF: usize = 0xde1f64;
    let colour = if read_u8(mark) == 1 {
        let v = read_u16(word) as i16;
        let (s, c) = match v {
            i16::MIN..=-37 => (0xa8591cusize, better2), // "  *****"
            -36..=-28 => (0xa85914, better2),           // "  ****"
            -27..=-20 => (0xa8590c, better2),           // "  ***"
            -19..=-13 => (0xa85904, better2),           // "  **"
            -12..=-7 => (0xa85900, better2),            // "  *"
            -6..=5 => (0x98e8c8, same),                 // "  -"
            6..=11 => (0xa85900, better1),
            12..=18 => (0xa85904, better1),
            19..=26 => (0xa8590c, better1),
            27..=35 => (0xa85914, better1),
            _ => (0xa8591c, better1),
        };
        // the winger's "  ****" is formatted without "%s" in the original: same text
        format(BUF, 0x991944, s); // "%s"
        c
    } else {
        lang(BUF, 0xa858f8); // "  n/a"
        colour_word(G_GUI_COLOUR_PALE_GREY)
    };
    compare_screen_cell(area, col, row, 0xc, 2, colour, BUF);
}

/// The key and the position table under the attributes: the "better" header ("<%s - player>{s} better", "Both the
/// same"), the legend ("Legend:", the masked attribute note) and the five rows of two positions each with their stars.
/// Returns the top of the last table less 10 (where the statistics end).
/// Original: part of the compare players screen builder (0x876520, staff_screens.cpp).
unsafe fn compare_screen_position_table(p1: usize, p2: usize, align: &[u8; 8], legend_align: &u8, better1: usize, same: usize, better2: usize) -> i32 {
    let lang: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    const BUF: usize = 0xde1f64;
    let (mut name1, mut name2) = ([0u8; 0xc8], [0u8; 0xc8]);
    exe::database_format_staff_name(p1, name1.as_mut_ptr() as usize, 0xc8, 1);
    exe::database_format_staff_name(p2, name2.as_mut_ptr() as usize, 0xc8, 1);
    let grey = colour_word(G_GUI_COLOUR_PALE_GREY);
    let bottom = 0x1f9 - exe::font_get_height(2);
    compare_screen_area(0x6e, bottom, 0x30c, 0x1f9, 1, 0, 1, 0, 2, 0);
    let t = compare_screen_area(0x6e, bottom, 0x30c, 0x1f9, 3, align.as_ptr() as usize, 1, 0, 1, 0);
    lang(BUF, 0xa85998, 0x64u32, name1.as_ptr() as usize); // "<%s - player>{s} better"
    compare_screen_cell(t, 0, 0, 0xc, 2, better1, BUF);
    lang(BUF, 0xa85988); // "Both the same"
    compare_screen_cell(t, 1, 0, 0xc, 2, same, BUF);
    lang(BUF, 0xa85998, 0x64u32, name2.as_ptr() as usize);
    compare_screen_cell(t, 2, 0, 0xc, 2, better2, BUF);

    let top = bottom - exe::font_get_height(2) * 2;
    compare_screen_area(0x6e, top, 0x30c, bottom, 1, 0, 1, 0, 2, 0);
    let t = compare_screen_area(0x6e, top, 0x30c, bottom, 1, legend_align as *const u8 as usize, 2, 0, 1, 0);
    lang(BUF, 0xa85980); // "Legend:"
    compare_screen_cell(t, 0, 0, 0xc, 2, grey, BUF);
    lang(BUF, 0xa85934); // "( - ) indicates that this attribute is masked and requires further scouting"
    compare_screen_cell(t, 0, 1, 0xc, 2, colour_word(0xaebdd4), BUF);

    let table_top = top - exe::font_get_height(2) * 5;
    compare_screen_area(0x6e, table_top, 0x30c, top, 1, 0, 1, 0, 2, 0);
    let t = compare_screen_area(0x6e, table_top, 0x30c, top, 4, align[4..].as_ptr() as usize, 5, 0, 1, 0);
    let rows: [(usize, usize, usize, usize, usize, usize); 5] = [
        (0xa85924, COMPARE_MARK_GK, COMPARE_POS_GK, 0xa858ec, COMPARE_MARK_DM, COMPARE_POS_DM), // Goalkeeper, Def. Mid.
        (0xa858e0, COMPARE_MARK_FB, COMPARE_POS_FB, 0xa858d0, COMPARE_MARK_MC, COMPARE_POS_MC), // Full Back, Midfielder
        (0xa858c0, COMPARE_MARK_CB, COMPARE_POS_CB, 0xa858b4, COMPARE_MARK_AM, COMPARE_POS_AM), // Centre Back, Att. Mid.
        (0xa858a8, COMPARE_MARK_SW, COMPARE_POS_SW, 0xa8589c, COMPARE_MARK_WG, COMPARE_POS_WG), // Sweeper, Winger
        (0xa85890, COMPARE_MARK_WB, COMPARE_POS_WB, 0xa85884, COMPARE_MARK_ST, COMPARE_POS_ST), // Wing Back, Striker
    ];
    for (row, &(left, lm, lw, right, rm, rw)) in rows.iter().enumerate() {
        let row = row as i32;
        lang(BUF, left);
        compare_screen_cell(t, 0, row, 0xc, 2, grey, BUF);
        compare_screen_stars(t, 1, row, lm, lw, better1, same, better2);
        lang(BUF, right);
        compare_screen_cell(t, 2, row, 0xc, 2, grey, BUF);
        compare_screen_stars(t, 3, row, rm, rw, better1, same, better2);
    }
    table_top - 10
}

/// The colours of the compare screen by the colour-blind option (`G_PLAYER_SETUP_COLOURBLIND_FLAG`): (the second
/// player better, the same) = yellow and 0xae3152 with it, red and yellow without.
unsafe fn compare_screen_colours() -> (usize, usize) {
    if read_u8(G_PLAYER_SETUP_COLOURBLIND_FLAG) != 0 {
        (colour_word(G_GUI_COLOUR_YELLOW), colour_word(0xae3152))
    } else {
        (colour_word(G_GUI_COLOUR_RED), colour_word(G_GUI_COLOUR_YELLOW))
    }
}

/// Copies the C string at `src` into `dst` (the CRT's inline strcpy of the original).
unsafe fn compare_screen_copy(dst: &mut [u8; 0xc8], src: usize) {
    compare_screen_strcpy(dst.as_mut_ptr() as usize, src);
}

/// The statistics of one player of the compare screen: for the six statistics periods 1..6, each code of his list
/// (the goalkeeper list 0xa84a90 when the player sheet's goalkeeper rating is 20, otherwise 0xa84a78; 0xff ends it)
/// formatted by `PlayerStats::format_player_stat` (`G_PLAYER_STATS`, 2000 bytes into the text buffer, 1, 0, 1), and
/// kept by code: 0x11 the average rating, 2 the goals, 3 the goals conceded, 1 the appearances, 5 the assists, 0xc the
/// man of the match awards (only the last period stays). A "-" becomes "0".
/// Original: part of the compare players screen builder (0x876520, staff_screens.cpp).
unsafe fn compare_screen_player_stats(p: usize, out: &mut [[u8; 0xc8]; 6]) {
    let stat: unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, u32, u32, u32, u32) -> usize = core::mem::transmute(0x7c25b0usize);
    const BUF: usize = 0xde1f64;
    const CODES: [u8; 6] = [0x11, 2, 3, 1, 5, 0xc];
    for period in 1..=6u32 {
        let player = read_usize(p + staff::PLAYER);
        let list = if player != 0 && read_u8(player + player_sheet::GOALKEEPER) == 0x14 { 0xa84a90usize } else { 0xa84a78 };
        let mut k = list;
        while read_u8(k) != 0xff {
            let code = read_u8(k);
            stat(read_usize(G_PLAYER_STATS), read_usize(p + staff::PLAYER), period, code as u32, BUF, 0x7d0, 1, 0, 1);
            if let Some(i) = CODES.iter().position(|&c| c == code) {
                compare_screen_copy(&mut out[i], BUF);
            }
            k += 1;
        }
    }
    for s in out.iter_mut() {
        if compare_screen_streq(s.as_ptr() as usize, 0x98ba0c) {
            // "-"
            compare_screen_copy(s, 0x99cc54); // "0"
        }
    }
}

/// The foot line of one player ("LO", "L", "B", "RO" or "R" with their comments): left foot 15 or more: right foot below
/// 8 left only, below 14 left, otherwise both; left foot below 8 right only, otherwise right.
unsafe fn compare_screen_foot(p: usize, out: &mut [u8; 0xc8]) {
    let lang: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let sheet = read_usize(p + staff::PLAYER);
    let left = read_i8(sheet + player_sheet::LEFT_FOOT);
    let right = read_i8(sheet + player_sheet::RIGHT_FOOT);
    let fmt = if left >= 0xf {
        if right < 8 { 0xa857f4usize } else if right < 0xe { 0xa857d0 } else { 0xa857ac }
    } else if left < 8 {
        0xa85780
    } else {
        0xa85758
    };
    lang(out.as_mut_ptr() as usize, fmt, 0xad9c64usize);
}

/// The statistics table of the compare screen (from the y the position table left): with two different players, the
/// first one not a manager (`staff::JOB_FOR_CLUB` 2) nor one of the 16 last staff records, the statistics of both
/// (`compare_screen_player_stats`); then, with the view mode (param 4) 0, or 2 for a player outside the scouting
/// search, and a first player who is a player, the 31 attributes again into the three tables (sorted by name with
/// 0x88bc50), the eight extra rows (Preferred Foot 0xff, Avg. Rating 0xfe, Morale 0xfd, On Loan 0xfb, Goals 0xf8 or,
/// with two goalkeepers, Conceded 0xf4, Appearances 0xf7, Assists 0xf6, Man of Match 0xf5) and the table in three
/// column pairs (name, value) of 13 rows (more when they fit): the attribute difference (" %d", in the colour of the
/// better one; " -" masked), the two feet, the ratings, the morale comparison, the statistics "%s - %s", and the
/// loan ("On loan from", linked) or the free transfer over the whole width. Without two players: "Need two players to
/// compare."
/// Original: part of the compare players screen builder (0x876520, staff_screens.cpp).
unsafe fn compare_screen_statistics(y: i32) {
    let lang: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let draw: TextDrawWrappedFn = core::mem::transmute(0x4aeaf0usize);
    let row_item: DisplayLineFn = core::mem::transmute(FN_DISPLAY_ADD_ITEM);
    let viewer_value: unsafe extern "C" fn(usize, usize, u32) -> u8 = core::mem::transmute(0x543440usize);
    let attribute_name: unsafe extern "C" fn(u32, usize, u32, u32) -> usize = core::mem::transmute(0x5428a0usize);
    let in_search: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x7f8610usize);
    let recent_ratings: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> usize = core::mem::transmute(0x7c1660usize);
    let morale_text: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) -> i8 = core::mem::transmute(0x4cc3a0usize);
    let gui_text_area: unsafe extern "C" fn(i32, i32, i32, i32, u32, u32, u32, u32, usize, usize, u32) -> usize = core::mem::transmute(0x5ed070usize);
    const BUF: usize = 0xde1f64;

    let mut text = [0usize; 2];
    let t = text.as_mut_ptr();
    exe::new_text(t, 0);
    let align: [u8; 6] = [3, 2, 3, 2, 3, 2];
    let mut morale: i8 = 0;
    let viewer = read_usize(G_SCREEN_RECORDS_MANAGER + read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize as usize * 0x300);
    let (better2, same) = compare_screen_colours();
    let better1 = colour_word(0xae31aa);
    for k in [1, 2, 5, 6] {
        get_screen_param(k);
    }
    let c1 = compare_screen_player(1);
    let c2 = compare_screen_player(2);
    let first = if c1 != 0 { c1 } else { get_screen_param(0) };
    let mode = get_screen_param(4) as i32;
    let n = read_i32(G_STAFF_COUNT);
    let skip = c1 == c2 || c1 == 0 || c2 == 0 || read_u8(first + staff::JOB_FOR_CLUB) == 2 || {
        let id = read_i32(first);
        id >= n.wrapping_sub(16) && (first == 0 || id < n)
    };
    'table: {
        if skip {
            covered!("tel07_2 compare screen without two players");
            let rows = ((y - 0x73) / exe::font_get_height(3)) as u8;
            lang(BUF, 0xa85718); // "Need two players to compare."
            gui_text_area(0x6e, 0x73, 0x30c, y, 2, 0, rows as u32, 3, colour_word(G_GUI_COLOUR_PALE_GREY), BUF, 0);
            break 'table;
        }
        let mut s1 = [[0u8; 0xc8]; 6];
        let mut s2 = [[0u8; 0xc8]; 6];
        // the four "0" the original copies first (into buffers the statistics then overwrite)
        compare_screen_copy(&mut s1[2], 0x99cc54);
        compare_screen_copy(&mut s2[2], 0x99cc54);
        compare_screen_copy(&mut s1[1], 0x99cc54);
        compare_screen_copy(&mut s2[1], 0x99cc54);
        compare_screen_player_stats(c1, &mut s1);
        compare_screen_player_stats(c2, &mut s2);
        if mode == 2 {
            if in_search(read_usize(G_SCOUT_MANAGER), first) != 0 || read_usize(first + staff::PLAYER) == 0 {
                break 'table;
            }
        } else if mode != 0 {
            break 'table;
        }
        if read_usize(first + staff::PLAYER) == 0 {
            break 'table;
        }
        covered!("tel07_2 compare screen statistics");
        let fit = ((y - 0x73) / exe::font_get_height(2)) as u8;
        compare_screen_area(0x6e, 0x73, 0x30c, y, 1, 0, 1, 0, 2, 0);
        let area = compare_screen_area(0x6e, 0x73, 0x30c, y, 6, align.as_ptr() as usize, fit as u32, 0, 1, 0);
        let mut col: u32 = 0;
        let mut row: i32 = 0;
        exe::training_manager_update_player_training(read_usize(G_TRAINING_MANAGER), first, 0);
        for attr in 1..=0x1fu32 {
            let k = (attr as usize - 1) * COMPARE_ROW;
            attribute_name(attr, COMPARE_TABLE_FIRST + k, 0x64, 0);
            attribute_name(attr, COMPARE_TABLE_SECOND + k, 0x64, 0);
            attribute_name(attr, COMPARE_TABLE_DIFF + k, 0x64, 0);
            if read_u8(G_ATTRIBUTE_MASKING_ENABLED) != 0 {
                let v2 = viewer_value(viewer, c2, attr);
                let v1 = viewer_value(viewer, c1, attr);
                write_u8(COMPARE_TABLE_DIFF + 0x64 + k, v1.wrapping_sub(v2));
                write_u8(COMPARE_TABLE_SECOND + 0x64 + k, viewer_value(viewer, c2, attr));
                let v1 = viewer_value(viewer, c1, attr);
                write_u8(COMPARE_TABLE_FIRST + 0x64 + k, v1);
                write_u8(COMPARE_MASKED + k, ((v1 as i8) < 0) as u8);
                if read_i8(COMPARE_TABLE_SECOND + 0x64 + k) < 0 {
                    write_u8(COMPARE_MASKED + k, 1);
                }
            } else {
                // without masking the marks stay as the position table left them
                let v2 = exe::staff_compute_player_attribute(c2, read_usize(c2 + staff::PLAYER), attr);
                let v1 = exe::staff_compute_player_attribute(c1, read_usize(c1 + staff::PLAYER), attr);
                write_u8(COMPARE_TABLE_DIFF + 0x64 + k, v1.wrapping_sub(v2));
                write_u8(COMPARE_TABLE_SECOND + 0x64 + k, exe::staff_compute_player_attribute(c2, read_usize(c2 + staff::PLAYER), attr));
                write_u8(COMPARE_TABLE_FIRST + 0x64 + k, exe::staff_compute_player_attribute(c1, read_usize(c1 + staff::PLAYER), attr));
            }
        }
        // DEFECT of the original kept: the masked marks are not sorted with the three tables; with the attribute
        // names in alphabetical order (English) the sort keeps the order, in another language the " -" lands on
        // another row
        for table in [COMPARE_TABLE_FIRST, COMPARE_TABLE_SECOND, COMPARE_TABLE_DIFF] {
            exe::crt_qsort(table, 0x1f, 0x65, 0x88bc50);
        }
        let extras: [(usize, usize, u8); 4] = [(31, 0xa85874, 0xff), (32, 0xa85868, 0xfe), (33, 0x98e9f0, 0xfd), (34, 0xa85860, 0xfb)];
        for (r, s, code) in extras {
            lang(COMPARE_TABLE_FIRST + r * COMPARE_ROW, s); // Preferred Foot, Avg. Rating, Morale, On Loan
            write_u8(COMPARE_TABLE_FIRST + r * COMPARE_ROW + 0x64, code);
        }
        if read_u32(COMPARE_BOTH_GOALKEEPERS) == 0 {
            lang(COMPARE_TABLE_FIRST + 35 * COMPARE_ROW, 0x98e9cc); // "Goals"
            write_u8(COMPARE_TABLE_FIRST + 35 * COMPARE_ROW + 0x64, 0xf8);
        } else {
            lang(COMPARE_TABLE_FIRST + 35 * COMPARE_ROW, 0x99cb44); // "Conceded"
            write_u8(COMPARE_TABLE_FIRST + 35 * COMPARE_ROW + 0x64, 0xf4);
        }
        for (r, s, code) in [(36usize, 0x99cc40usize, 0xf7u8), (37, 0x99c06c, 0xf6), (38, 0x99c04c, 0xf5)] {
            lang(COMPARE_TABLE_FIRST + r * COMPARE_ROW, s); // Appearances, Assists, Man of Match
            write_u8(COMPARE_TABLE_FIRST + r * COMPARE_ROW + 0x64, code);
        }
        // the 39 rows dealt to the three columns; the table height is the most rows of a column or what fits
        let mut per_column = [0u8; 3];
        let mut rows = fit;
        let mut c = 0usize;
        for _ in 0..0x27 {
            per_column[c] += 1;
            if per_column[c] > rows {
                rows = per_column[c];
            }
            c = if c + 1 > 2 { 0 } else { c + 1 };
        }
        let grey = colour_word(G_GUI_COLOUR_PALE_GREY);
        let blank = |col: u32, row: i32| {
            compare_screen_cell(area, col, row, 0xc, 2, grey, 0);
            compare_screen_cell(area, col + 1, row, 0xc, 2, grey, 0);
        };
        let (mut feet1, mut feet2) = ([0u8; 0xc8], [0u8; 0xc8]);
        let mut item = 0usize;
        while item < 0x27 {
            let k = item * COMPARE_ROW;
            let code = read_u8(COMPARE_TABLE_FIRST + 0x64 + k);
            if code == 0xfb {
                row_item(current_human_display(), 1, 0x22e, 0, 0x30c, 0, 0, row as usize, 2, 0, 0, 0xc, 2, grey, 0, 0, 0, 0, area);
                let mut drawn = false;
                if exe::contract_manager_get_loan_contract(read_usize(G_CONTRACT_MANAGER), first) != 0 {
                    let contract = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), first);
                    write_usize(0xae1080, contract);
                    if contract != 0 {
                        let club = read_usize(G_CLUB_TABLE) + read_i32(contract + 4) as isize as usize * club::SIZE;
                        if club != 0 {
                            let gender = read_i8(club + club::SHORT_GENDER_NAME) as i32;
                            lang(BUF, 0xa85834, gender, gender, club + club::SHORT_NAME); // "On loan from {}<%s - Club(e.g.Falkirk)>{}"
                            exe::text_set_string(t as usize, BUF as *const u8);
                            exe::text_add_link(t as usize, (club + club::SHORT_NAME) as *const u8, 0x7d5, club);
                            draw(t, area, 0x238, 0x30c, &mut row, 2, colour_word(0xae3152), 3, 0);
                            drawn = true;
                        }
                    }
                }
                if !drawn && exe::transfer_manager_is_queued_for_free_transfer(read_usize(G_TRANSFER_MANAGER), first) != 0 {
                    lang(BUF, 0xa85820); // "Given free transfer"
                    exe::text_set_string(t as usize, BUF as *const u8);
                    draw(t, area, 0x238, 0x30c, &mut row, 2, colour_word(0xae3152), 3, 0);
                }
            } else {
                format(BUF, 0x987ff0, COMPARE_TABLE_FIRST + k); // "  %s"
                compare_screen_cell(area, col, row, 1, 2, grey, BUF);
                let value = col + 1;
                let code = code as i8;
                if code >= 0 {
                    if read_u8(COMPARE_MASKED + k) == 1 {
                        format(BUF, 0xa8573c); // " -"
                        compare_screen_cell(area, value, row, 0xc, 2, colour_word(0xaebdd4), BUF);
                    } else {
                        let d = read_i8(COMPARE_TABLE_DIFF + 0x64 + k);
                        let colour = if d > 0 { better1 } else if d == 0 { same } else { better2 };
                        format(BUF, 0xa85738, (d as i32).abs()); // " %d"
                        compare_screen_cell(area, value, row, 0xc, 2, colour, BUF);
                    }
                } else {
                    match code as u8 {
                        0xff => {
                            compare_screen_foot(c1, &mut feet1);
                            compare_screen_foot(c2, &mut feet2);
                        }
                        0xfe => {
                            if first != 0 && read_i32(first) >= n {
                                format(feet1.as_mut_ptr() as usize, 0x98ba0c); // "-"
                            } else {
                                // computed but not shown: the row prints the average ratings of the statistics
                                recent_ratings(read_usize(G_PLAYER_STATS), read_usize(c1 + staff::PLAYER), feet1.as_mut_ptr() as usize, 0xc8);
                                recent_ratings(read_usize(G_PLAYER_STATS), read_usize(c2 + staff::PLAYER), feet2.as_mut_ptr() as usize, 0xc8);
                            }
                        }
                        0xfd => {
                            morale = morale_text(read_usize(G_CONTRACT_MANAGER), c1, c2, feet1.as_mut_ptr() as usize, 0xc8);
                        }
                        _ => {}
                    }
                    let hint = colour_word(0xaebdd4);
                    match code as u8 {
                        0xfe => {
                            format(BUF, 0xa8574c, s1[0].as_ptr() as usize, s2[0].as_ptr() as usize); // " %s - %s "
                            compare_screen_cell(area, value, row, 0xc, 1, hint, BUF);
                        }
                        0xf9 => {
                            format(BUF, 0xa8573c); // " -"
                            compare_screen_cell(area, value, row, 0xc, 2, hint, BUF);
                        }
                        0xf8 | 0xf4 | 0xf7 | 0xf6 | 0xf5 => {
                            let i = match code as u8 { 0xf8 => 1, 0xf4 => 2, 0xf7 => 3, 0xf6 => 4, _ => 5 };
                            format(BUF, 0x988080, s1[i].as_ptr() as usize, s2[i].as_ptr() as usize); // "%s - %s"
                            compare_screen_cell(area, value, row, 0xc, 2, hint, BUF);
                        }
                        0xfd => {
                            format(BUF, 0x987ff0, feet1.as_ptr() as usize); // "  %s"
                            let colour = if morale < -3 { better2 } else if morale < 3 { same } else { better1 };
                            compare_screen_cell(area, value, row, 0xc, 2, colour, BUF);
                        }
                        _ => {
                            format(BUF, 0xa85740, feet1.as_ptr() as usize, feet2.as_ptr() as usize); // "  %s ~ %s"
                            compare_screen_cell(area, value, row, 0xc, 2, hint, BUF);
                        }
                    }
                }
            }
            row += 1;
            if row == per_column[(col / 2) as usize] as i32 {
                while row < rows as i32 {
                    blank(col, row);
                    row += 1;
                }
                col += 2;
                row = 0;
                if col + 1 > 5 {
                    break;
                }
            }
            item += 1;
        }
        while col < 6 {
            while row < rows as i32 {
                blank(col, row);
                row += 1;
            }
            col += 2;
            row = 0;
        }
    }
    exe::drop_text(t);
}

/// Builds the compare players screen: the two compare players of the scouting manager
/// (`SCOUT_MANAGER::get_compare_player` 1 and 2; with no first one the screen's player, param 0; with neither,
/// nothing), the heading (`compare_screen_heading`), for a first player who is a player of the database the position
/// table (`compare_screen_positions`, `compare_screen_position_table`), the statistics table
/// (`compare_screen_statistics`) and the Back/Next buttons (`gui_add_back_next_buttons` with "back possible" and
/// "next possible"). Params read: 0, 4 (the view mode); 1, 2, 3, 5..8 and 0x14 read and ignored.
///
/// Convention: cdecl, SEH frame (0x2644 bytes of locals): a Text and an array of 25 `MenuItem` (built and destroyed,
/// never used). Split into named pieces; the whole routine is hooked.
/// Original: `staff_screens_build_compare_players_screen` (0x876520, staff_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn staff_screens_build_compare_players_screen() {
    let construct_vector: unsafe extern "stdcall" fn(usize, u32, u32, usize, usize) = core::mem::transmute(FN_CRT_VECTOR_CONSTRUCTOR_ITERATOR);
    let destroy_vector: unsafe extern "stdcall" fn(usize, u32, u32, usize) = core::mem::transmute(0x9462bbusize);
    let mut text = [0usize; 2];
    let t = text.as_mut_ptr();
    exe::new_text(t, 0);
    let mut menu = [0u8; 25 * 0xd4];
    construct_vector(menu.as_mut_ptr() as usize, 0xd4, 0x19, 0x4568f0, 0x5a53c0);
    let align: [u8; 8] = [3, 3, 3, 0, 3, 3, 3, 3];
    let legend_align: u8 = 1;

    let mut p1 = compare_screen_player(1);
    let p2 = compare_screen_player(2);
    let param0 = get_screen_param(0);
    for k in [1, 2, 3] {
        get_screen_param(k);
    }
    let mode = get_screen_param(4) as u8 as i8 as i32;
    for k in [5, 6, 7, 8, 0x14] {
        get_screen_param(k);
    }
    let (better2, same) = compare_screen_colours();
    let better1 = colour_word(0xae31aa);
    if p1 == 0 && p2 == 0 {
        covered!("tel07_2 compare screen with nobody");
    } else {
        covered!("tel07_2 compare screen");
        if p1 == 0 {
            p1 = param0;
        }
        compare_screen_heading(t, p1, p2, mode);
        let mut y = 0x1fe;
        if read_usize(p1 + staff::PLAYER) != 0 && (p1 == 0 || read_i32(p1) < read_i32(G_STAFF_COUNT)) {
            compare_screen_positions(p1, p2);
            y = compare_screen_position_table(p1, p2, &align, &legend_align, better1, same, better2);
        }
        compare_screen_statistics(y);
        let next = (exe::screenmanager_is_forward_unavailable(SCREEN_MANAGER) == 0) as u32;
        let back = (exe::screenmanager_is_back_unavailable(SCREEN_MANAGER, 0) == 0) as u32;
        exe::gui_add_back_next_buttons(back, next);
    }
    destroy_vector(menu.as_mut_ptr() as usize, 0xd4, 0x19, 0x5a53c0);
    exe::drop_text(t);
}

/// Whether `TRAINING_MANAGER::update_player_training(person, 0)` (0x8b1750) would train `person` now. It returns
/// without writing anything but the contract lookup global when the person's id is past `G_STAFF_COUNT`, or when his
/// training entry (`PLAYER_ENTRIES` + sheet id * 6) already holds the current `WEEK_COUNTER` at +1 (0x8b1bce..0x8b1bea)
/// and its bit 0x20 at +5 ("at a human club") needs no setting: set, or clear with no contract club whose manager is
/// one of the 16 last staff records (0x8b188d..0x8b1948). Otherwise it rolls the week's training (`utils_random`
/// 0x90cfc0, injuries, the position training items) and stamps the week, so a second call is a no-op but a call
/// after the comparers restore the entry trains again. A null person or one without a player sheet: false (the
/// screens do not call it for them).
pub(crate) unsafe fn training_update_pending(person: usize) -> bool {
    let tm = read_usize(G_TRAINING_MANAGER);
    if person == 0 || tm == 0 || read_i32(person) >= read_i32(G_STAFF_COUNT) {
        return false;
    }
    let sheet = read_usize(person + staff::PLAYER);
    if sheet == 0 {
        return false;
    }
    let entry = read_usize(tm + training_manager_fields::PLAYER_ENTRIES) + read_i32(sheet) as usize * 6;
    if read_u8(entry + 5) & 0x20 == 0 {
        let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), person);
        if c != 0 {
            let manager = read_usize(club_at(read_i32(c + 4) as isize).addr() + club::MANAGER);
            if manager != 0 && read_i32(manager) >= read_i32(G_STAFF_COUNT) - 0x10 {
                return true;
            }
        }
    }
    read_u16(tm + training_manager_fields::WEEK_COUNTER) != read_u16(entry + 1)
}

/// Comparer for `staff_screens_build_compare_players_screen`: both build side by side and are compared by the screen
/// (the display list, `compare_match_screen_build`) plus what the screen writes outside it: the compare tables
/// (names, values, differences and masked marks), the position words and marks and the contract pointer 0xae1080.
/// When the first player's training is due (`training_update_pending`) only the original runs: the week's training
/// rolls once and the copy would start from another state.
pub(crate) unsafe extern "C" fn compare_staff_screens_build_compare_players_screen() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I0%%);
    let c1 = compare_screen_player(1);
    if training_update_pending(if c1 != 0 { c1 } else { get_screen_param(0) }) {
        // simplification: the training roll runs once, in the original
        return o();
    }
    let tables = 0x27 * COMPARE_ROW;
    let extra = [
        (COMPARE_MASKED, tables),
        (COMPARE_POS_AM, 4),
        (COMPARE_TABLE_SECOND, tables),
        (COMPARE_TABLE_DIFF, tables),
        (COMPARE_TABLE_FIRST, tables),
        (COMPARE_POS_GK, 0x24),
        (0xae1080, 4),
    ];
    compare_match_screen_build("compare players screen", &extra, || o(), || staff_screens_build_compare_players_screen());
}
