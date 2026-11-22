// ---------- batch %%LEVA%%: the competition screen (comp_screens.cpp, part 2): the league Table tab and the cup Draw tab ----------

/// "The draw will be made on {}<%s - date >." with the long date of `date` (an 8-byte Date): the long date string into
/// `buf` (0x7d0, 0xff), the weekday's grammatical gender in the game's language, then the sentence into the screens'
/// text buffer (the weekday asked before the language, as the original).
unsafe fn league_tab_draw_date_sentence(date: usize, buf: *mut u8) {
    exe::date_get_long_date_string(date, buf as usize, 0x7d0, 0xff);
    let weekday = exe::date_get_weekday(date);
    let gender = exe::date_get_weekday_gender(weekday as u32, exe::langlib_get_language() as u32);
    comp_lang_format()(JOB_TEXT, 0x99c2e8, gender as i32, buf);
}

/// `comp_screens_add_stage_selector(comp, stage, top)` (0x49d380, cdecl): the stage buttons above a tab; returns
/// nonzero when it added them.
unsafe fn league_tab_stage_selector(comp: usize, stage: usize, top: i32) -> u32 {
    let f: unsafe extern "C" fn(usize, usize, i32) -> u32 = core::mem::transmute(0x49d380usize);
    f(comp, stage, top)
}

/// The rows a text area of `bottom - top - 0x10` pixels holds in font 3 (signed division, the low byte kept).
unsafe fn league_tab_rows(top: i32, bottom: i32) -> u32 {
    ((bottom - top - 0x10) / exe::font_get_height(3)) as u8 as u32
}

/// Rule: the Table tab of a league stage of the competition screen. Before the draw date: "The draw will be made on
/// ..."; a competition not active: "The competition has not been scheduled yet."; otherwise the View popup (Overall,
/// Home Record, Away Record: the table type in screen parameter 4), the stage buttons, the title, and the table:
/// position, status letter, club, nation (national team competitions), played, won, drawn (or SOW / Pen by the match
/// rules), lost, for, against, points, with a divider row before each promotion/play-off/relegation band. A small
/// league (8 clubs or fewer, played in legs) also shows its fixtures from a week before its first round to three
/// weeks after its last (the list kept in screen parameter 5).
///
/// `comp_screens_draw_league_table_tab(stage, top, bottom)`: no stage: the comp_screens.cpp box at line 0x36f; no
/// competition (parameter 0): the box at 0x37b. Parameters read: 2 the stage index, 4 the table type, 5 the fixture
/// list. With a draw day (`league::NEXT_DRAW_DAY` ≠ 0xffff) on a date (year `league::YEAR` + the offset) still ahead of
/// today (years unsigned, days signed): with a `league::STAGE_ID` its name (the parent stage's when
/// `exe::comp_util_get_parent_stage_id` gives one) as a yellow title of 0x23 pixels and 0x28 down, then the sentence in
/// a pale grey text area (font 3). With the competition's `league::ACTIVE` zero: the not-scheduled sentence the same
/// way. Otherwise, with `league::LEGS` ≠ 0: [0xadae0c] = 1, the View button (0x6e, top, 0xeb, top + 0x14), its popup
/// area (flags 0x130) and the three popup items (id 6, data 0/1/2; the current type ticked, its text without the
/// leading space kept for the title). The stage buttons (`comp_screens_add_stage_selector`), 0x19 down; the title:
/// the stage name (or "League Table"), with the table type in brackets when not Overall; 0x28 down. Small league
/// (`league::CLUB_COUNT` ≤ 8, `league::LEGS` > 0, not the American major league): rows = clubs + 1 + one per band
/// (`league::PROMOTED_COUNT` or the champion flag of `league::LEAGUE_FLAGS`, and each of the three other counts), the
/// area exactly as high; otherwise the rows that fit down to `bottom`. National team competitions
/// (`league::CLUB_SIDES` = 2) move 4 of width from the club column to the nation column. The 11-column area, the
/// headings (Pld, Won, SOW/Pen/Drn, Lst, For, Ag, Pts); `league::TABLE_COLUMN` = the table type and
/// `exe::league_sort_table`. Each row k: the divider (an item of kind 1, flags 0x6400, x2 0x30a or 0x2f4 when the
/// table does not fit) when k is the first row after a band (yellow for promotion/champion and relegation, 0xaebdd4
/// for the play-off bands), then the position (an ordinal; a button with id 7 when the league has legs), the status
/// letter (C, Q, P, Pl, R by `league_table_row::STATUS`), the club (yellow when human), the nation, and the numbers of
/// the block of the table type (14 bytes per type; the Polish first division's second stage adds 14 or 7 to the
/// played games). Small league: the fixture list is built when parameter 5 is empty (the competition's virtual round
/// dates +0x3c, a week before the first, three weeks after the last, `exe::fixture_manager_find_fixtures` into a new
/// LLIST), then a 6-column area below the table: date (a button, id 8), home club, score (the extra-time score when
/// there is one; a button with id 0x7d3 when the match has statistics; "view" with id 0x7d2 for a watched match, "v"
/// otherwise), away club, the stadium (club competitions) or city, "-----"; empty rows fill the area. Last: the scroll
/// row of parameter 6 on the last area, parameter 7 = that area.
/// Original: `comp_screens_draw_league_table_tab` (0x491380, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_screens_draw_league_table_tab(stage: usize, top: i32, bottom: i32) {
    use league as l;
    use league_table_row as row;
    let mut weights: [u8; 11] = [4, 2, 0x10, 0, 4, 4, 4, 4, 4, 4, 4];
    let fixtures_weights: [u8; 6] = [3, 8, 2, 8, 8, 3];
    let mut text_buf = [0u8; 0x7d0];
    let mut title = [0u8; 0x7d0];
    if stage == 0 {
        return comp_screens_error_box(0x36f);
    }
    let comp = get_screen_param(0);
    if comp == 0 {
        return comp_screens_error_box(0x37b);
    }
    let stage_index = get_screen_param(2);
    let table_type = get_screen_param(4);
    let mut list = get_screen_param(5);
    let pale = job_colour(G_GUI_COLOUR_PALE_GREY);
    let grey = job_colour(G_GUI_COLOUR_GREY);
    let yellow = job_colour(G_GUI_COLOUR_YELLOW);
    let black = job_colour(G_GUI_COLOUR_BLACK);
    let popup = job_colour(JOB_COLOUR_31AA);
    let draw_day = read_u16(stage + l::NEXT_DRAW_DAY);
    if draw_day != 0xffff {
        let mut date = [0u8; 8];
        let d = date.as_mut_ptr() as usize;
        exe::new_date(d, draw_day as u32, read_u16(stage + l::NEXT_DRAW_YEAR_OFFSET).wrapping_add(read_u16(stage + l::YEAR)) as u32);
        let (today_day, today_year) = (read_i16(G_CURRENT_DATE), read_u16(G_CURRENT_DATE + 2));
        let year = read_u16(d + 2);
        if today_year < year || (today_year == year && today_day < read_i16(d)) {
            covered!("tel22_2 league draw ahead");
            let mut y = top;
            let label = read_i16(stage + l::STAGE_ID);
            if label != 0 {
                let mut id = exe::comp_util_get_parent_stage_id(label as i32);
                if id == -1 {
                    id = label as i32;
                }
                exe::comp_util_get_stage_name(JOB_TEXT as *mut u8, 0x7d0, id as u32, 0);
                comp_text_item([0x6e, y as u32, 0x30c, (y + 0x23) as u32, 2, 0, 0xc, 3, yellow, JOB_TEXT as u32, 1]);
                y += 0x28;
            }
            let rows = league_tab_rows(y, bottom);
            league_tab_draw_date_sentence(d, text_buf.as_mut_ptr());
            tel26_text_area([0x6e, y as u32, 0x30c, bottom as u32, 2, 0, rows, 3, pale, JOB_TEXT as u32, 0]);
            return;
        }
    }
    if read_u32(comp + l::ACTIVE) == 0 {
        let rows = league_tab_rows(top, bottom);
        comp_lang_format()(JOB_TEXT, 0x99c2bc); // "The competition has not been scheduled yet."
        tel26_text_area([0x6e, top as u32, 0x30c, bottom as u32, 2, 0, rows, 3, pale, JOB_TEXT as u32, 0]);
        return;
    }
    let mut y = top;
    let display = current_human_display();
    if read_i16(stage + l::LEGS) != 0 {
        comp_lang_format()(JOB_TEXT, 0x988004); // "View"
        write_u32(0xadae0c, 1);
        let button = tel26_add_button([0x6e, y as u32, 0xeb, (y + 0x14) as u32, 0x30, grey, 0xc, 1, pale, JOB_TEXT as u32, 0, 0]);
        let area = tel26_add_area(display, [0, 0, 0, 0, 0, 0, 0, 0, 0x130, popup, button]);
        // the three popup items (id 6, data = the table type), the current one ticked and its text kept for the title
        for (kind, label) in [(0u32, 0x99c2b0usize), (1, 0x99c2a0), (2, 0x99c290)] { // " Overall", " Home Record", " Away Record"
            comp_lang_format()(JOB_TEXT, label);
            let flags = if table_type as u32 == kind { 0x2002 } else { 2 };
            tel26_add_item(display, [flags, 0, 0, 0, 0, 0, 0, 0x10, popup, yellow, 1, 1, black, JOB_TEXT as u32, 0, 6, kind, area]);
            if table_type as u32 == kind {
                copy_c_string(text_buf.as_mut_ptr() as usize, JOB_TEXT + 1);
            }
        }
    }
    league_tab_stage_selector(comp, stage_index, y);
    y += 0x19;
    let label = read_u16(stage + l::STAGE_ID);
    if label != 0 {
        exe::comp_util_get_stage_name(title.as_mut_ptr(), 0x7d0, label as u32, 0);
    } else {
        comp_lang_format()(title.as_mut_ptr() as usize, 0x99c280); // "League Table"
    }
    if table_type != 0 {
        comp_lang_format()(JOB_TEXT, 0x99c250, title.as_ptr(), text_buf.as_ptr()); // "%s (%s)"
    } else {
        copy_c_string(JOB_TEXT, title.as_ptr() as usize);
    }
    comp_text_item([0x6e, y as u32, 0x30c, (y + 0x23) as u32, 2, 0, 0xc, 3, yellow, JOB_TEXT as u32, 1]);
    y += 0x28;
    let clubs = read_i16(stage + l::CLUB_COUNT);
    let small = clubs <= 8 && read_i16(stage + l::LEGS) > 0
        && read_u32(read_usize(stage + l::COMPETITION)) != read_u32(KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE);
    let (rows, area_bottom);
    if small {
        covered!("tel22_2 league small table");
        let mut r = (clubs as u8).wrapping_add(1);
        if read_u8(stage + l::PROMOTED_COUNT) != 0 || read_u8(stage + l::LEAGUE_FLAGS) & 1 != 0 {
            r = r.wrapping_add(1);
        }
        for f in [l::PROMOTION_PLAYOFF_COUNT, l::RELEGATION_PLAYOFF_COUNT, l::RELEGATED_COUNT] {
            if read_u8(stage + f) != 0 {
                r = r.wrapping_add(1);
            }
        }
        rows = r;
        area_bottom = y + exe::font_get_height(3) * r as i32 + 0x10;
    } else {
        rows = ((bottom - y - 0x10) / exe::font_get_height(3)) as u8;
        area_bottom = bottom;
    }
    if read_u8(comp + l::CLUB_SIDES) == 2 {
        weights[3] = weights[3].wrapping_add(4);
        weights[2] = weights[2].wrapping_sub(4);
    }
    let mut area = tel26_add_area(display, [0x6e, y as u32, 0x30c, area_bottom as u32, 0xb, weights.as_ptr() as u32, rows as u32, 0, 2, 0, u32::MAX]);
    let heading = |col: u32| tel26_area_text([area, col, 0, 0x30, grey, 0xc, 1, pale, JOB_TEXT as u32]);
    comp_lang_format()(JOB_TEXT, 0x990b3c); // "Pld"
    heading(4);
    comp_lang_format()(JOB_TEXT, 0x990b1c, 0xad9c64usize); // "Won"
    heading(5);
    let rules = read_u16(stage + l::MATCH_RULES);
    comp_lang_format()(JOB_TEXT, if rules & 0x20 != 0 { 0x990b18 } else if rules & 1 != 0 { 0x99c24c } else { 0x990b14 }); // SOW, Pen, Drn
    heading(6);
    comp_lang_format()(JOB_TEXT, 0x990af4, 0xad9c64usize); // "Lst"
    heading(7);
    comp_lang_format()(JOB_TEXT, 0x990af0); // "For"
    heading(8);
    comp_lang_format()(JOB_TEXT, 0x990aec); // "Ag"
    heading(9);
    comp_lang_format()(JOB_TEXT, 0x990ae8); // "Pts"
    heading(0xa);
    write_u32(stage + l::TABLE_COLUMN, table_type as u32);
    exe::league_sort_table(stage, 0usize);
    let block = (table_type as u32).wrapping_mul(14) as usize;
    let mut line: i32 = 1;
    let mut k: i32 = 0;
    while k < read_i16(stage + l::CLUB_COUNT) as i32 {
        let teams = read_i16(stage + l::CLUB_COUNT) as i32;
        let promoted = read_i8(stage + l::PROMOTED_COUNT) as i32;
        let playoff = read_i8(stage + l::PROMOTION_PLAYOFF_COUNT) as i32;
        let relegation_playoff = read_i8(stage + l::RELEGATION_PLAYOFF_COUNT) as i32;
        let relegated = read_i8(stage + l::RELEGATED_COUNT) as i32;
        // the band divider before row k
        let divider = if promoted == 0 && read_u8(stage + l::LEAGUE_FLAGS) & 1 != 0 && k == 1 {
            Some(yellow)
        } else if promoted > 0 && k == promoted {
            Some(yellow)
        } else if playoff > 0 && k == playoff + promoted {
            Some(job_colour(JOB_COLOUR_BDD4))
        } else if relegation_playoff > 0 && k == teams - relegation_playoff - relegated {
            Some(job_colour(JOB_COLOUR_BDD4))
        } else if relegated > 0 && k == teams - relegated {
            Some(yellow)
        } else {
            None
        };
        if let Some(colour) = divider {
            covered!("tel22_2 league band divider");
            let x2 = if teams as i16 <= rows as i16 { 0x30a } else { 0x30a - 0x16 }; // tel22_2 the divider width
            tel26_add_item(display, [1, 0x70, 0, x2, 0, 0, line as u32, 0x6400, colour, 0, 0, 0, 0, 0, 0, 0, 0, area]);
            line += 1;
        }
        let r = exe::league_get_table_row(stage, k);
        let club = read_usize(r + row::CLUB);
        k += 1;
        let ordinal = exe::utils_format_ordinal(k, 3);
        if read_i16(stage + l::LEGS) != 0 {
            tel26_area_button([area, 0, line as u32, 0x30, job_colour(G_GUI_COLOUR_DARK_BLUE), 0xc, 1, pale, ordinal as u32, 7, r as u32]);
        } else {
            tel26_area_text([area, 0, line as u32, 0x30, job_colour(G_GUI_COLOUR_DARK_BLUE), 0xc, 1, pale, ordinal as u32]);
        }
        write_u8(JOB_TEXT, 0);
        let status = match read_u8(r + row::STATUS) {
            5 => Some(0x99c228usize), // "C"
            6 | 7 | 8 => Some(0x99c1b8), // "Q"
            0 => Some(0x99c20c), // "P"
            1 | 2 => Some(0x99c1d4), // "Pl"
            3 => Some(0x99c1f0), // "R"
            _ => None,
        };
        if let Some(fmt) = status {
            comp_lang_format()(JOB_TEXT, fmt, 0xad9c64usize);
            if read_u8(JOB_TEXT) != 0 {
                tel26_area_text([area, 1, line as u32, 0x30, job_colour(0xaebdcc), 0xc, 1, pale, JOB_TEXT as u32]);
            }
        }
        let club_colour = if exe::human_manager_is_human_club(club, 0, 1, 0, 0) != 0 { yellow } else { pale };
        tel26_area_button([area, 2, line as u32, 1, 0, 1, 3, club_colour, (club + club::SHORT_NAME) as u32, 0x7d5, club as u32]);
        if read_u8(comp + l::CLUB_SIDES) == 2 && read_usize(club + club::NATION) != 0 {
            let nation = exe::database_get_national_team(read_usize(club + club::NATION));
            tel26_area_button([area, 3, line as u32, 1, 0, 0xc, 2, job_colour(JOB_COLOUR_BDD4), (read_usize(club + club::NATION) + nation::THREE_LETTER_NAME) as u32, 0x7d5, nation as u32]);
        }
        let played = if read_u32(read_usize(stage + l::COMPETITION)) == read_u32(KEY_CLUB_COMP_POLISH_FIRST_DIVISION) && read_u8(stage + l::STAGE_INDEX) == 1 {
            covered!("tel22_2 league polish second stage");
            if table_type == 0 { read_i8(r + row::PLAYED) as i32 + 0xe } else { read_i8(r + block + row::PLAYED) as i32 + 7 }
        } else {
            read_i8(r + block + row::PLAYED) as i32
        };
        let number = |col: u32, v: i32| {
            comp_crt_format()(JOB_TEXT, 0x98806c, v); // "%d"
            tel26_area_text([area, col, line as u32, 1, 0, 0xc, 2, pale, JOB_TEXT as u32]);
        };
        number(4, played);
        number(5, read_i8(r + block + row::WON) as i32);
        number(6, read_i8(r + block + row::DRAWN) as i32);
        number(7, read_i8(r + block + row::LOST) as i32);
        number(8, read_i16(r + block + row::GOALS_FOR) as i32);
        number(9, read_i16(r + block + row::GOALS_AGAINST) as i32);
        comp_crt_format()(JOB_TEXT, 0x98806c, read_i16(r + block + row::POINTS) as i32);
        tel26_area_text([area, 0xa, line as u32, 0x30, job_colour(JOB_COLOUR_3192), 0xc, 1, pale, JOB_TEXT as u32]);
        line += 1;
    }
    if small {
        let y = area_bottom + 5;
        if list == 0 {
            covered!("tel22_2 league fixture list built");
            // the competition's round dates (virtual +0x3c): an array of 0x41-byte entries, day and year offset
            let (mut count, mut a, mut b) = (0u32, 0u32, 0u32);
            let rounds: unsafe extern "thiscall" fn(usize, usize, *mut u32, *mut u32, *mut u32) -> usize = core::mem::transmute(read_usize(read_usize(comp) + 0x3c));
            let dates = rounds(comp, stage_index, &mut count, &mut a, &mut b);
            let count = count as i16 as isize;
            let (mut first, mut last, mut tmp) = ([0u8; 8], [0u8; 8], [0u8; 8]);
            let first_p = first.as_mut_ptr() as usize;
            let last_p = last.as_mut_ptr() as usize;
            let tmp_p = tmp.as_mut_ptr() as usize;
            let mut shifted = [0u8; 8];
            exe::new_date(first_p, read_u16(dates) as u32, read_u16(dates + 2).wrapping_add(read_u16(comp + l::YEAR)) as u32);
            let t = exe::date_sub_weeks(first_p, &mut shifted, 1);
            exe::date_operator_3d(first_p, tmp_p, t);
            let e = (dates as isize + count * 0x41 - 0x41) as usize;
            exe::new_date(last_p, read_u16(e) as u32, read_u16(e + 2).wrapping_add(read_u16(comp + l::YEAR)) as u32);
            let t = exe::date_add_weeks(last_p, &mut shifted, 3);
            exe::date_operator_3d(last_p, tmp_p, t);
            exe::crt_free(dates);
            let node = exe::crt_operator_new(0xe);
            list = if node != 0 { exe::new_llist(node) } else { 0 };
            set_screen_param(5, list);
            exe::fixture_manager_find_fixtures(read_usize(G_FIXTURE_MANAGER), read_u32(first_p), read_u32(first_p + 2), read_u32(last_p),
                read_u32(last_p + 2), u32::MAX, list, read_u32(read_usize(comp + l::COMPETITION)), stage_index as u32, u32::MAX);
        }
        let rows = ((bottom - y) / exe::font_get_height(2)) as u8 as i32;
        area = tel26_add_area(display, [0x6e, y as u32, 0x30c, bottom as u32, 6, fixtures_weights.as_ptr() as u32, rows as u32, 0, 1, 0, u32::MAX]);
        let mut line: i32 = 0;
        write_u32(list + llist::CURSOR, read_u32(list + llist::HEAD));
        let mut f = exe::llist_get_next(list);
        while f != 0 {
            if read_u8(f + fixture::HOME_GOALS) != 0xfe && read_u8(f + fixture::AWAY_GOALS) != 0xfd {
                league_tab_fixture_row(comp, area, line, f);
                line += 1;
            }
            f = exe::llist_get_next(list);
        }
        while line < rows {
            for col in 0..8u32 {
                tel26_area_text([area, col, line as u32, 2, 0, 1, 2, pale, 0]);
            }
            line += 1;
        }
    }
    let scroll = get_screen_param(6);
    if area as u16 != 0xffff {
        exe::area_set_scroll_row((display as isize + 4 + area as i16 as isize * 0xbf1) as usize, scroll);
    }
    set_screen_param(7, area as i16 as isize as usize);
}

/// One row of a small league's fixture list: the date (a button, id 8), the home club (yellow when human), the score
/// or "view"/"v", the away club, the stadium or city, "-----".
unsafe fn league_tab_fixture_row(comp: usize, area: u32, line: i32, f: usize) {
    let pale = job_colour(G_GUI_COLOUR_PALE_GREY);
    let yellow = job_colour(G_GUI_COLOUR_YELLOW);
    let short_date: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(0x54a650usize);
    let mut date = [0u8; 8];
    exe::new_date(date.as_mut_ptr() as usize, read_u16(f + fixture::DAY) as u32, read_u16(f + fixture::YEAR) as u32);
    short_date(date.as_mut_ptr() as usize, JOB_TEXT, 0x7d0, 0xff);
    tel26_area_button([area, 0, line as u32, 0x30, job_colour(G_GUI_COLOUR_DARK_BLUE), 0xc, 1, pale, JOB_TEXT as u32, 8, f as u32]);
    let side = |col: u32, club: usize| {
        comp_crt_format()(JOB_TEXT, 0x987ff0, club + club::SHORT_NAME); // "  %s"
        let colour = if exe::human_manager_is_human_club(club, 0, 1, 0, 0) != 0 { yellow } else { pale };
        tel26_area_button([area, col, line as u32, 2, 0, 1, 2, colour, JOB_TEXT as u32, 0x7d5, club as u32]);
    };
    side(1, read_usize(f + fixture::HOME_CLUB));
    let score = if read_u8(f + fixture::HOME_GOALS_EXTRA_TIME) != 0xff {
        Some((fixture::HOME_GOALS_EXTRA_TIME, fixture::AWAY_GOALS_EXTRA_TIME))
    } else if read_u8(f + fixture::HOME_GOALS) != 0xff {
        Some((fixture::HOME_GOALS, fixture::AWAY_GOALS))
    } else {
        None
    };
    if let Some((h, a)) = score {
        comp_lang_format()(JOB_TEXT, 0x990bb4, read_i8(f + h) as i32, read_i8(f + a) as i32); // "%d:%d"
        let has_stats = exe::match_stats_fixture_entry_matches(read_usize(G_MATCH_STATS), read_i32(f + fixture::MATCH_STATS_SLOT),
            read_i32(f + fixture::HOME_CLUB_ID), read_i32(f + fixture::AWAY_CLUB_ID), read_u16(f + fixture::DAY) as i32, read_u16(f + fixture::YEAR) as i32);
        if has_stats != 0 {
            tel26_area_button([area, 2, line as u32, 2, 0, 0xc, 2, yellow, JOB_TEXT as u32, 0x7d3, read_u32(f + fixture::MATCH_STATS_SLOT)]);
        } else {
            tel26_area_text([area, 2, line as u32, 2, 0, 0xc, 2, yellow, JOB_TEXT as u32]);
        }
    } else if exe::human_manager_is_watching_fixture(read_usize(G_HUMAN_MANAGER), f, 0usize) != 0 {
        comp_lang_format()(JOB_TEXT, 0x9918d4); // "view"
        tel26_area_button([area, 2, line as u32, 2, 0, 0xc, 1, job_colour(JOB_COLOUR_BDE8), JOB_TEXT as u32, 0x7d2, f as u32]);
    } else {
        comp_lang_format()(JOB_TEXT, 0x99c1b4); // "v"
        tel26_area_button([area, 2, line as u32, 2, 0, 0xc, 2, yellow, JOB_TEXT as u32, 0x7d2, f as u32]);
    }
    side(3, read_usize(f + fixture::AWAY_CLUB));
    let stadium = read_usize(f + fixture::STADIUM);
    if read_u8(comp + league::CLUB_SIDES) == 1 {
        if stadium != 0 {
            comp_crt_format()(JOB_TEXT, 0x987ff0, stadium + stadium::NAME);
        } else {
            comp_lang_format()(JOB_TEXT, 0x99c1a4); // "Unknown Stadium"
        }
    } else if stadium != 0 && read_usize(stadium + stadium::CITY) != 0 {
        comp_crt_format()(JOB_TEXT, 0x987ff0, read_usize(stadium + stadium::CITY) + 4); // the city's name
    } else {
        comp_lang_format()(JOB_TEXT, 0x99c194); // "Unknown City"
    }
    tel26_area_text([area, 4, line as u32, 2, 0, 0xc, 2, job_colour(JOB_COLOUR_BDD4), JOB_TEXT as u32]);
    tel26_area_text([area, 5, line as u32, 0x30, job_colour(JOB_COLOUR_3192), 0xc, 1, pale, 0x99c18c]); // "-----"
}

/// Comparer for `comp_screens_draw_league_table_tab`: side by side over the display list (`compare_match_screen_build`)
/// plus the stage's table type.
pub(crate) unsafe extern "C" fn compare_comp_screens_draw_league_table_tab(stage: usize, top: i32, bottom: i32) {
    let o = original_fn::<unsafe extern "C" fn(usize, i32, i32)>(%%I0%%);
    // simplification: the error boxes and the first build of a small league's fixture list (operator new, kept in
    // parameter 5) do not run twice: original only; the copy is proven in Rust mode
    if stage == 0 || get_screen_param(0) == 0 || (get_screen_param(5) == 0 && read_i16(stage + league::CLUB_COUNT) <= 8 && read_i16(stage + league::LEGS) > 0) {
        return o(stage, top, bottom);
    }
    compare_match_screen_build("competition screen, league table tab", &[(stage + league::TABLE_COLUMN, 4)], || o(stage, top, bottom),
        || comp_screens_draw_league_table_tab(stage, top, bottom));
}

/// The division (club competitions) or national team (national team competitions) column of a cup team, or an empty
/// cell: the division's three-letter name as a button (id 0x7d0) when its competition object is loaded, otherwise the
/// empty string; the nation's three-letter name as a button (id 0x7d5) with its national team; nothing else.
unsafe fn cup_tab_team_origin(comp: usize, area: u32, col: u32, line: i32, club: usize) {
    let pale = job_colour(G_GUI_COLOUR_PALE_GREY);
    let blue = job_colour(G_GUI_COLOUR_DARK_BLUE);
    let sides = read_u8(comp + league::CLUB_SIDES);
    if sides == 1 && read_usize(club + club::DIVISION) != 0 {
        let division = read_usize(club + club::DIVISION);
        if read_usize(read_usize(G_COMP_OBJECT_TABLE) + read_u32(division) as usize * 4) != 0 {
            tel26_area_button([area, col, line as u32, 0x30, blue, 0xc, 1, pale, (division + 0x53) as u32, 0x7d0, division as u32]); // the three-letter name [inference]
        } else {
            tel26_area_text([area, col, line as u32, 0x30, blue, 0xc, 1, pale, 0xad9c64]);
        }
    } else if sides == 2 && read_usize(club + club::NATION) != 0 {
        let nation = exe::database_get_national_team(read_usize(club + club::NATION));
        tel26_area_button([area, col, line as u32, 0x30, blue, 0xc, 1, pale, (read_usize(club + club::NATION) + nation::THREE_LETTER_NAME) as u32, 0x7d5, nation as u32]);
    } else {
        tel26_area_text([area, col, line as u32, 0x30, blue, 0xc, 1, pale, 0]);
    }
}

/// The club name of one of the possible teams of a cup side: the last one plain, the others with "or" after it
/// ("{}<%s - club name>{} or" with the gender byte twice), as a button (id 0x7d5, yellow when human).
unsafe fn cup_tab_team_name(area: u32, col: u32, line: i32, club: usize, last: bool) {
    if last {
        copy_c_string(JOB_TEXT, club + club::SHORT_NAME);
    } else {
        let g = read_i8(club + club::SHORT_GENDER_NAME) as i32;
        comp_lang_format()(JOB_TEXT, 0x99c374, g, g, club + club::SHORT_NAME);
    }
    let colour = if exe::human_manager_is_human_club(club, 0, 1, 0, 0) != 0 { job_colour(G_GUI_COLOUR_YELLOW) } else { job_colour(G_GUI_COLOUR_PALE_GREY) };
    tel26_area_button([area, col, line as u32, 1, 0, 1, 3, colour, JOB_TEXT as u32, 0x7d5, club as u32]);
}

/// The cup's team table entry `team` (6-byte entries: +0 the club, +5 the status).
unsafe fn cup_tab_team_club(cup_: usize, team: i16) -> usize {
    read_usize((read_usize(cup_ + cup::TEAM_TABLE) as isize + team as isize * 6) as usize)
}

/// The status byte of the last tie of round `round` (0x35-byte ties; 3 = decided).
unsafe fn cup_tab_last_tie_status(cup_: usize, round: i32) -> u8 {
    let ties = read_i16((read_usize(cup_ + cup::ROUND_TABLE) as isize + round as isize * 0x68 + cup_round::TIES as isize) as usize) as isize;
    let table = read_usize((read_usize(cup_ + cup::TIE_TABLES) as isize + round as isize * 4) as usize);
    read_u8((table as isize + ties * 0x35 - 1) as usize)
}

/// Rule: the Draw tab of a cup stage of the competition screen. Before the first draw: "The draw has not yet been
/// made." or "The draw will be made on ..."; afterwards the round of screen parameter 3 (−2: the last drawn) with its
/// name, the draw buttons (Draw Next Team, Draw All Teams, Automatic Draw) while the round is still to be drawn, and
/// the ties: the possible home teams (with their division or nation), the score of each leg, the away teams once the
/// tie is decided, and, when the round is drawn and the next one is not the third-place round, the teams given a bye.
///
/// `comp_screens_draw_cup_rounds_tab(cup, top, bottom)`: no cup: the box at line 0x69a; no competition (parameter 0):
/// 0x6a6. Parameters 2 (the stage) and 3 (the round; −2 becomes `cup::NEXT_DRAW_ROUND` − 1, written back). The stage
/// buttons, 0x19 down (and [0xadae0c] = 1) when there are some. No draw yet (`cup::NEXT_DRAW_ROUND` = 0): the stage
/// label's name as the title, then not active (`cup::SEASON_SET_UP` zero) "The draw has not yet been made.", otherwise
/// the draw date (round table +0 day, +2 signed year offset) sentence, in a pale grey text area. Otherwise: the round
/// name (`exe::comp_util_format_stage_round_name`) as the title when not empty. Round ≥ `cup::CURRENT_ROUND` and its
/// code not 0x8c: a 5-column area (4 without weights in languages 2, 4 and 5) with the three draw buttons (ids 9, 0xa,
/// 0xb; the automatic one with value 1 unless parameter 0x35; plain texts when the draw cannot go on: the previous round
/// still undecided, or this round's last tie decided); parameter 0x34 = the cup while this round's last tie is not
/// decided; 0x19 down. The 9-column ties area; for each tie with status ≠ 1 (`exe::get_fixture_side_teams` for both
/// sides): the home teams one per row (division/nation in column 0, the name in column 2), the score row (columns
/// 1/7 the penalties or the walkover/away-goals marks, 3/5 the result marks, 4 the goals "x : y" with ":" a button to
/// the match statistics, id 0x7d3, or "v" when not played), and with status 3 the away teams (column 6, origin in 8);
/// the next tie starts after the longer side. Then, the round decided (or not yet current) and a next round that is
/// not the third-place one: the teams of rounds 0..round (`cup_round::NEW_TEAMS` summed) not in any of this round's
/// ties and still in (status 0), under the "Teams given bye" heading. Parameters 0x32 the last tie shown, 0x33 its
/// flag, " N teams to draw" in the draw area (2 × the remaining ties − 2, + 1 with the flag), parameter 6 = the rows
/// when −1, the scroll row on the ties area and parameter 7 = that area.
/// Defect of the original: the row after a tie with status 1 is the larger of the previous ends (uninitialised stack
/// for the first tie; 0 here).
/// Original: `comp_screens_draw_cup_rounds_tab` (0x492980, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_screens_draw_cup_rounds_tab(cup_: usize, top: i32, bottom: i32) {
    use cup as c;
    let ties_weights: [u8; 9] = [3, 2, 0x12, 1, 4, 1, 0x12, 2, 3];
    let draw_weights: [u8; 5] = [2, 2, 2, 2, 2];
    let mut draw_area: u32 = u32::MAX;
    if cup_ == 0 {
        return comp_screens_error_box(0x69a);
    }
    let comp = get_screen_param(0);
    if comp == 0 {
        return comp_screens_error_box(0x6a6);
    }
    let stage_index = get_screen_param(2);
    let mut round = get_screen_param(3) as i32;
    if round == -2 {
        round = read_i8(cup_ + c::NEXT_DRAW_ROUND) as i32 - 1;
        set_screen_param(3, round as isize as usize);
    }
    let pale = job_colour(G_GUI_COLOUR_PALE_GREY);
    let grey = job_colour(G_GUI_COLOUR_GREY);
    let yellow = job_colour(G_GUI_COLOUR_YELLOW);
    let marks = job_colour(JOB_COLOUR_BDE8);
    let mut y = top;
    if league_tab_stage_selector(comp, stage_index, y) != 0 {
        write_u32(0xadae0c, 1);
        y += 0x19;
    }
    let rounds = read_usize(cup_ + c::ROUND_TABLE);
    if read_u8(cup_ + c::NEXT_DRAW_ROUND) == 0 {
        covered!("tel22_2 cup no draw yet");
        let label = read_u16(cup_ + c::STAGE_LABEL);
        if label != 0 {
            exe::comp_util_get_stage_name(JOB_TEXT as *mut u8, 0x7d0, label as u32, 0);
            comp_text_item([0x6e, y as u32, 0x30c, (y + 0x23) as u32, 2, 0, 0xc, 3, yellow, JOB_TEXT as u32, 1]);
            y += 0x28;
        }
        let rows = league_tab_rows(y, bottom);
        if read_u32(cup_ + c::SEASON_SET_UP) == 0 {
            comp_lang_format()(JOB_TEXT, 0x99c3bc); // "The draw has not yet been made."
        } else {
            let mut date = [0u8; 8];
            let mut buf = [0u8; 0x7d0];
            exe::new_date(date.as_mut_ptr() as usize, read_u16(rounds) as u32,
                (read_i8(rounds + 2) as i16).wrapping_add(read_i16(cup_ + c::YEAR)) as u16 as u32);
            league_tab_draw_date_sentence(date.as_mut_ptr() as usize, buf.as_mut_ptr());
        }
        tel26_text_area([0x6e, y as u32, 0x30c, bottom as u32, 2, 0, rows, 3, pale, JOB_TEXT as u32, 0]);
        return;
    }
    let rec = (round as isize * 0x68) as usize;
    exe::comp_util_format_stage_round_name(read_i16(cup_ + c::STAGE_LABEL) as i32, read_i16(rounds.wrapping_add(rec) + cup_round::ROUND_CODE) as i32, JOB_TEXT, 0x7d0, 0);
    if read_u8(JOB_TEXT) != 0 {
        comp_text_item([0x6e, y as u32, 0x30c, (y + 0x23) as u32, 2, 0, 0xc, 3, yellow, JOB_TEXT as u32, 1]);
        y += 0x28;
    }
    let current = read_i8(cup_ + c::CURRENT_ROUND) as i32;
    if round >= current && read_u16(rounds.wrapping_add(rec) + cup_round::ROUND_CODE) != 0x8c {
        covered!("tel22_2 cup draw buttons");
        let lang = exe::langlib_get_language();
        let narrow = lang == 5 || exe::langlib_get_language() == 4 || exe::langlib_get_language() == 2;
        let display = current_human_display();
        draw_area = if !narrow {
            tel26_add_area(display, [0x6e, y as u32, 0x30c, (y + 0x14) as u32, 5, draw_weights.as_ptr() as u32, 1, 0, 1, 0, u32::MAX])
        } else {
            tel26_add_area(display, [0x6e, y as u32, 0x30c, (y + 0x14) as u32, 4, 0, 1, 0, 1, 0, u32::MAX])
        };
        let can_draw = (round <= current || cup_tab_last_tie_status(cup_, round - 1) == 3) && cup_tab_last_tie_status(cup_, round) != 3;
        let button = |col: u32, label: usize, id: u32| {
            comp_lang_format()(JOB_TEXT, label);
            if can_draw {
                tel26_area_button([draw_area, col, 0, 0x30, grey, 0xc, 1, pale, JOB_TEXT as u32, id, 0]);
            } else {
                tel26_area_text([draw_area, col, 0, 0x30, grey, 0x2c, 1, pale, JOB_TEXT as u32]);
            }
        };
        button(0, 0x99c3ac, 9); // "Draw Next Team"
        button(1, 0x99c39c, 0xa); // "Draw All Teams"
        comp_lang_format()(JOB_TEXT, 0x99c38c); // "Automatic Draw"
        if can_draw {
            if get_screen_param(0x35) != 0 {
                tel26_area_button([draw_area, 2, 0, 0x70, grey, 0xc, 1, pale, JOB_TEXT as u32, 0xb, 0]);
            } else {
                tel26_area_button([draw_area, 2, 0, 0x30, grey, 0xc, 1, pale, JOB_TEXT as u32, 0xb, 1]);
            }
        } else {
            tel26_area_text([draw_area, 2, 0, 0x30, grey, 0x2c, 1, pale, JOB_TEXT as u32]);
        }
        if cup_tab_last_tie_status(cup_, round) != 3 {
            set_screen_param(0x34, cup_);
        }
        y += 0x19;
    }
    let rows = ((bottom - y - 0x10) / exe::font_get_height(3)) as u8;
    let area = tel26_add_area(current_human_display(), [0x6e, y as u32, 0x30c, bottom as u32, 9, ties_weights.as_ptr() as u32, rows as u32, 0, 2, 0, u32::MAX]);
    let mut line: i32 = 0;
    let mut last_tie: i32 = -1;
    let mut shown: u32 = 0;
    let (mut home_end, mut away_end): (i32, i32) = (0, 0);
    let ties = |r: usize| read_i16(read_usize(cup_ + c::ROUND_TABLE).wrapping_add(r) + cup_round::TIES) as i32;
    let side_teams: unsafe extern "thiscall" fn(usize, i32, i32, u32, *mut i16) -> i32 = core::mem::transmute(0x521790usize);
    let mut home = [0i16; 100];
    let mut away = [0i16; 100];
    let mut k: i32 = 0;
    while k < ties(rec) {
        let tie = (read_usize((read_usize(cup_ + c::TIE_TABLES) as isize + round as isize * 4) as usize) as isize + k as isize * 0x35) as usize;
        let n_home = side_teams(cup_, round, k, 1, home.as_mut_ptr());
        let n_away = side_teams(cup_, round, k, 0, away.as_mut_ptr());
        if read_u8(tie + 0x34) != 1 {
            last_tie = k;
            home_end = line;
            shown = 1;
            for j in 0..n_home.max(0) {
                let club = cup_tab_team_club(cup_, home[j as usize]);
                cup_tab_team_origin(comp, area, 0, home_end, club);
                cup_tab_team_name(area, 2, home_end, club, j == n_home - 1);
                home_end += 1;
            }
            if read_u8(tie + 0x14) == 0xff {
                comp_lang_format()(JOB_TEXT, 0x99c1b4); // "v"
                tel26_area_text([area, 4, line as u32, 1, 0, 0xc, 3, marks, JOB_TEXT as u32]);
            } else {
                cup_tab_score_row(cup_, area, line, round, k, tie);
            }
        }
        if read_u8(tie + 0x34) == 3 {
            covered!("tel22_2 cup tie decided");
            away_end = line;
            shown = 0;
            for j in 0..n_away.max(0) {
                let club = cup_tab_team_club(cup_, away[j as usize]);
                cup_tab_team_name(area, 6, away_end, club, j == n_away - 1);
                cup_tab_team_origin(comp, area, 8, away_end, club);
                away_end += 1;
            }
        }
        line = if home_end > away_end { home_end } else { away_end };
        k += 1;
    }
    let drawn = round < current || cup_tab_last_tie_status(cup_, round) == 3;
    if drawn && round + 1 < read_i16(cup_ + c::ROUND_COUNT) as i32 && read_u16(rounds.wrapping_add(rec) + 0x68 + cup_round::ROUND_CODE) != 0x8c {
        covered!("tel22_2 cup byes");
        let mut total: i16 = 0;
        for r in 0..=round {
            total = total.wrapping_add(read_i16(rounds + r as usize * 0x68 + cup_round::NEW_TEAMS));
        }
        let total = total as i32;
        let mask = exe::crt_calloc(1, total as u32);
        for t in 0..ties(rec) {
            let n_home = side_teams(cup_, round, t, 1, home.as_mut_ptr());
            let n_away = side_teams(cup_, round, t, 0, away.as_mut_ptr());
            for j in 0..n_home.max(0) {
                if (home[j as usize] as i32) < total as i16 as i32 {
                    write_u8((mask as isize + home[j as usize] as isize) as usize, 1);
                }
            }
            for j in 0..n_away.max(0) {
                if (away[j as usize] as i32) < total as i16 as i32 {
                    write_u8((mask as isize + away[j as usize] as isize) as usize, 1);
                }
            }
        }
        let mut heading = false;
        for i in 0..total {
            if read_u8(mask + i as usize) != 0 || read_u8(read_usize(cup_ + c::TEAM_TABLE) + i as usize * 6 + 5) != 0 {
                continue;
            }
            if !heading {
                comp_lang_format()(JOB_TEXT, 0x99c33c); // "Teams given bye"
                line += 1;
                let x2 = (exe::font_text_width(3usize, JOB_TEXT as *const u8) + 0x73) as u32;
                tel26_add_item(current_human_display(), [1, 0x73, 0, x2, 0, 0, line as u32, 1, 0, 0, 1, 3, yellow, JOB_TEXT as u32, 0, 0, 0, area]);
                line += 1;
                tel26_add_item(current_human_display(), [1, 0x73, 0, x2, 0, 0, line as u32, 0xc400, yellow, 0, 0, 0, 0, 0, 0, 0, 0, area]);
                line += 1;
                heading = true;
            }
            let club = read_usize(read_usize(cup_ + c::TEAM_TABLE) + i as usize * 6);
            cup_tab_team_origin(comp, area, 0, line, club);
            let colour = if exe::human_manager_is_human_club(club, 0, 1, 0, 0) != 0 { yellow } else { pale };
            tel26_area_button([area, 2, line as u32, 1, 0, 1, 3, colour, (club + club::SHORT_NAME) as u32, 0x7d5, club as u32]);
            line += 1;
        }
        exe::crt_free(mask);
    }
    set_screen_param(0x32, last_tie as i16 as isize as usize);
    set_screen_param(0x33, shown as usize);
    if draw_area as u16 != 0xffff {
        let left = (ties(rec) - last_tie as i16 as i32) * 2 - 2 + shown as i32;
        comp_lang_format()(JOB_TEXT, 0x99c314, left); // " %d teams to draw"
        let col = if exe::langlib_get_language() == 5 || exe::langlib_get_language() == 4 || exe::langlib_get_language() == 2 { 3 } else { 4 };
        tel26_area_text([draw_area, col, 0, 2, 0, 0xc, 1, job_colour(0xae3152), JOB_TEXT as u32]);
    }
    if get_screen_param(6) as i32 == -1 {
        set_screen_param(6, line as isize as usize);
    }
    let scroll = get_screen_param(6);
    if area as u16 != 0xffff {
        exe::area_set_scroll_row((current_human_display() as isize + 4 + area as i16 as isize * 0xbf1) as usize, scroll);
    }
    set_screen_param(7, area as i16 as isize as usize);
}

/// The score row of a played cup tie: columns 1 and 7 the home and away penalties (style 3) or the marks of byte +4/+5
/// ("w", "ww", or the negated value; style 1), 3 and 5 the result letters (s, p, e, a, w by +4/+5), and 4 the goals:
/// "x    " ":" "    y" (the extra-time, second-leg or first-leg goals), ":" a button to the match statistics (id
/// 0x7d3) when the tie's scheduled fixture has them.
unsafe fn cup_tab_score_row(cup_: usize, area: u32, line: i32, round: i32, k: i32, tie: usize) {
    let yellow = job_colour(G_GUI_COLOUR_YELLOW);
    let marks = job_colour(JOB_COLOUR_BDE8);
    let (mut away_pens, mut home_pens) = ([0u8; 8], [0u8; 8]);
    let (mut home_letter, mut away_letter) = ([0u8; 8], [0u8; 8]);
    let (mut home_goals, mut away_goals) = ([0u8; 8], [0u8; 8]);
    let d = |b: &mut [u8; 8], v: i8| { comp_crt_format()(b.as_mut_ptr() as usize, 0x98806c, v as i32); };
    let style: u32;
    if read_u8(tie + 0x26) != 0xff {
        d(&mut home_pens, read_i8(tie + 0x27));
        d(&mut away_pens, read_i8(tie + 0x26));
        style = 3;
    } else {
        for (b, at) in [(&mut home_pens, 4usize), (&mut away_pens, 5)] {
            let v = read_i8(tie + at);
            match v {
                1 => { comp_lang_format()(b.as_mut_ptr() as usize, 0x99c370); } // "w"
                2 => { comp_lang_format()(b.as_mut_ptr() as usize, 0x99c36c); } // "ww"
                v if v < 0 => { comp_crt_format()(b.as_mut_ptr() as usize, 0x98806c, -(v as i32)); }
                _ => {}
            }
        }
        style = 1;
    }
    for (b, at) in [(&mut home_letter, 4usize), (&mut away_letter, 5)] {
        let fmt = match read_u8(tie + at) {
            5 => 0x99c368usize, // "s"
            4 => 0x99c364, // "p"
            3 => 0x99c360, // "e"
            6 => 0x99c35c, // "a"
            8 => 0x99c370, // "w"
            _ => 0,
        };
        if fmt != 0 {
            comp_lang_format()(b.as_mut_ptr() as usize, fmt);
        }
    }
    let (h, a) = if read_u8(tie + 0x1e) == 0xff {
        if read_u8(tie + 0x18) != 0xff { (0x18, 0x19) } else { (0x14, 0x15) }
    } else if read_u8(tie + 0x28) == 0xff {
        if read_u8(tie + 0x22) != 0xff { (0x23, 0x22) } else { (0x1f, 0x1e) }
    } else if read_u8(tie + 0x2c) != 0xff {
        (0x2c, 0x2d)
    } else {
        (0x28, 0x29)
    };
    d(&mut home_goals, read_i8(tie + h));
    d(&mut away_goals, read_i8(tie + a));
    if home_pens[0] != 0 {
        tel26_area_text([area, 1, line as u32, 1, 0, 0xc, style, yellow, home_pens.as_ptr() as u32]);
    }
    if home_letter[0] != 0 {
        tel26_area_text([area, 3, line as u32, 1, 0, 0x40, 1, yellow, home_letter.as_ptr() as u32]);
    }
    let find: unsafe extern "thiscall" fn(usize, i32, i32) -> usize = core::mem::transmute(0x519d90usize);
    let f = find(cup_, round, k);
    let with_stats = f != 0 && exe::match_stats_fixture_entry_matches(read_usize(G_MATCH_STATS), read_i32(f + fixture::MATCH_STATS_SLOT),
        read_i32(f + fixture::HOME_CLUB_ID), read_i32(f + fixture::AWAY_CLUB_ID), read_u16(f + fixture::DAY) as i32, read_u16(f + fixture::YEAR) as i32) != 0;
    comp_crt_format()(JOB_TEXT, 0x99c354, home_goals.as_ptr()); // "%s    "
    tel26_area_text([area, 4, line as u32, 1, 0, 0xc, 3, marks, JOB_TEXT as u32]);
    if with_stats {
        covered!("tel22_2 cup score with stats");
        tel26_area_button([area, 4, line as u32, 1, 0, 0xc, 3, marks, 0x989adc, 0x7d3, read_u32(f + fixture::MATCH_STATS_SLOT)]); // ":"
    } else {
        tel26_area_text([area, 4, line as u32, 1, 0, 0xc, 3, marks, 0x989adc]);
    }
    comp_crt_format()(JOB_TEXT, 0x99c34c, away_goals.as_ptr()); // "    %s"
    tel26_area_text([area, 4, line as u32, 1, 0, 0xc, 3, marks, JOB_TEXT as u32]);
    if away_letter[0] != 0 {
        tel26_area_text([area, 5, line as u32, 1, 0, 1, 1, yellow, away_letter.as_ptr() as u32]);
    }
    if away_pens[0] != 0 {
        tel26_area_text([area, 7, line as u32, 1, 0, 0xc, style, yellow, away_pens.as_ptr() as u32]);
    }
}

/// Comparer for `comp_screens_draw_cup_rounds_tab`: side by side over the display list (`compare_match_screen_build`;
/// the params it writes are in the screen's parameter block).
pub(crate) unsafe extern "C" fn compare_comp_screens_draw_cup_rounds_tab(cup_: usize, top: i32, bottom: i32) {
    let o = original_fn::<unsafe extern "C" fn(usize, i32, i32)>(%%I1%%);
    // simplification: the error boxes (no cup, no competition) only through the original
    if cup_ == 0 || get_screen_param(0) == 0 {
        return o(cup_, top, bottom);
    }
    compare_match_screen_build("competition screen, cup draw tab", &[], || o(cup_, top, bottom), || comp_screens_draw_cup_rounds_tab(cup_, top, bottom));
}
