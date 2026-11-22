// ---------- batch %%LEVA%%: the club screen's next match and fixtures tabs and the Spanish badge pictures (club_screens.cpp) ----------
// ---------- the club screen's next match tab (club_screens.cpp, part 2: tel02_1) ----------

const NEXT_MATCH_RECORD_KEY: usize = 0x7d6; // the screen record's manager (staff) pointer, +0x300 per slot (scrman's record KEY)
const NEXT_MATCH_COMP_GROUP: usize = 0x50; // byte of the competition object: its group of related competitions (0xff none) [inference]
const NEXT_MATCH_CITY_NAME: usize = 0x4; // the city record's name (text)
const NEXT_MATCH_OFFICIAL_SIZE: usize = 0x2b; // one record of G_OFFICIAL_TABLE
const NEXT_MATCH_COLOUR_POPUP: usize = 0xae31aa; // the popup list background colour word
const NEXT_MATCH_COLOUR_ORANGE: usize = 0xaebdd4; // g_gui_colour_orange
const NEXT_MATCH_EMPTY_TEXT: usize = 0xad9c64; // "" (the langlib comment argument)
const NEXT_MATCH_TEXT: usize = 0xde1f64; // the shared text buffer

/// Draws the next match tab of the club screen (screen params: 0 the club, 1 the reserves/'B' flag, 3 the next fixture)
/// between `top` and `bottom`.
///
/// Without a fixture: only the message "There are no forthcoming matches to be played." (reserves: "...reserve
/// matches...", or "...'B' international matches..." for a national team; `FN_GUI_ADD_TEXT_AREA`, as many lines of font
/// 3 as fit in `bottom - top - 0x10`).
///
/// With a fixture: the arrows row (x 0x6e..0x168): the previous fixtures of the club between the match date minus 24
/// weeks (national team) or 8 weeks and the day before (`FIXTURE_MANAGER::find_fixtures`, type −2, side 1 unless
/// reserves) give "<< Date" as button 0x26 when the last of them is still unplayed (home goals 0xff), the next ones from
/// the day after to the match date plus 24/8 weeks give "Date >>" as button 0x27 when there is any; otherwise grey text.
/// The fixtures from today to the match date are counted: the unplayed ones and, among them, those whose competition is
/// in the same group (`NEXT_MATCH_COMP_GROUP`) as the match's. Then (x 0x212..0x30c) "Progress" (button 0x1d) when the
/// fixture resolves to a league (`comp_fixture_resolve_league_clubs`) that is not a reserve or minor competition, and
/// "Past Meetings": a popup (button with a list of "Against <club>" items, id 0x1e) against the opponent and against the
/// human manager's national team (for a 'B' national team, when it is of another nation) or his club (when it is not
/// this club nor its linked B team); otherwise a plain button 0x1e. The title row: the opponent with "(Home)"/"(Away)"
/// (`comp_util_get_fixture_venue_side`; neutral: the name alone), button 0x7d5 to the opponent.
///
/// The table (two columns, 3:5) is laid out twice: the first pass only counts the rows, the second adds them, with the
/// right edge at 0x30a, or 0x2f6 (room for the scroll bar) when the first pass counted more rows than fit. Rows: the
/// round (`comp_util_format_fixture_round_label`, orange, a link 0x7e5 to the competition when it is a real current
/// competition: in the table, not the reserve key, not a friendly's own and current on the match date) and its underline;
/// Date (long date, with "(today)", "(tomorrow)" or "(N days)"); Venue (stadium, city); Aggregate (best of three: the
/// series; or the aggregate score when there is one); Match Rules (the foreign, non-EU or over-age squad limit of
/// `transfer_manager_get_selection_limit`, else "No player restrictions"); the tie rules of `fixture::TIE_RULES` and
/// `fixture::FLAGS` (away goals after 90 minutes, extra time or golden goal, away goals after extra time, penalties,
/// US-style shoot-out, replay, the seeded home or away team going through; "still level" once an earlier rule was
/// listed); the substitutes (named, used); Last Meeting (the latest played meeting of the two clubs before today,
/// `FIXTURE_MANAGER::scan_fixtures`: result, round and date with links, wrapped; or "-"); Weather or Weather Forecast
/// (known when the match is within 2 weeks); Pitch Condition (match day only); the referee with his strictness (Harsh 20,
/// Strict over 14, Lenient under 6; plain names with the database names language set). Outside the reserves, the team
/// news of both teams ("<team> News" header once per team), each squad player at most once, not on loan out, needing no
/// work permit and in the right squad (national team: not bit 8, else not bit 2 of `staff::PLAYING_SQUAD`), in this
/// order: away on international duty, ineligible, suspended (`Discipline::is_player_banned` with the same-group count),
/// injured beyond the match date, doubtful (back in light training but more than 3 days out), set for debut (joined
/// under 100 days ago, no appearances for the club, not a friendly, this is the next unplayed fixture, and a first team,
/// rotation or backup squad status).
///
/// Defect of the original (harmless): the first pass reads the selection limit before it is filled (stack leftover); the
/// rules block always counts exactly one row, so the layout does not change. The copy's frame starts zeroed.
/// Original: `club_screen_draw_next_match_tab` (0x45ccf0, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn club_screen_draw_next_match_tab(top: i32, bottom: i32) {
    let lang: unsafe extern "C" fn(usize, usize, ...) -> usize = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let message: unsafe extern "C" fn(usize, usize, usize, usize, usize, usize, usize, usize, usize, usize, usize) -> usize =
        core::mem::transmute(FN_GUI_ADD_TEXT_AREA);
    let add_area: DisplayListFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    let add_item: DisplayLineFn = core::mem::transmute(FN_DISPLAY_ADD_ITEM);
    let cell_button: ListCellFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_BUTTON);
    let cell_text: ListCellTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
    let button_item: ManagerBoxFn = core::mem::transmute(FN_SCREENMANAGER_ADD_BUTTON_ITEM);
    // thiscall helpers (checked at the call sites: ret = 4 x the arguments after this)
    let llist_drop: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let llist_clear: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_CLEAR);
    let llist_next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let llist_prev: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_PREV);
    let text_new: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_TEXT_TEXT);
    let text_drop: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_TEXT_DESTRUCTOR_TEXT);
    let text_clear: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x4afa40usize); // Text::clear
    let text_set: unsafe extern "thiscall" fn(usize, usize) -> i32 = core::mem::transmute(FN_TEXT_SET_STRING);
    let text_link: unsafe extern "thiscall" fn(usize, usize, u32, usize) -> i32 = core::mem::transmute(FN_TEXT_ADD_LINK);
    // Text::draw_wrapped(slot, left, right, &line, font, colour, style, flags), ret 0x20
    let text_draw: unsafe extern "thiscall" fn(usize, usize, i32, i32, usize, u32, usize, u32, u32) -> i32 = core::mem::transmute(0x4aeaf0usize);
    let date_new: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE);
    let date_sub_days: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_DATE_SUB_DAYS);
    let date_sub_weeks: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_DATE_SUB_WEEKS);
    let date_add_days: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_DATE_ADD_DAYS);
    let date_add_weeks: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_DATE_ADD_WEEKS);
    let days_since: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
    let long_date: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> i32 = core::mem::transmute(FN_DATE_GET_LONG_DATE_STRING_NO_YEAR);
    let short_date: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> i32 = core::mem::transmute(0x54a650usize); // Date::get_short_date_string
    let find_fixtures: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, usize, u32, u32, u32) -> u32 =
        core::mem::transmute(FN_FIXTURE_MANAGER_FIND_FIXTURES);
    let scan_fixtures: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize, u32, u32, u32, u32) -> u32 =
        core::mem::transmute(FN_FIXTURE_MANAGER_SCAN_FIXTURES);
    let is_national: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let club_is_national: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_CLUB_IS_NATIONAL_TEAM);
    let linked_b_team: unsafe extern "C" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
    let resolve_league: unsafe extern "C" fn(usize, usize, usize, usize) -> u32 = core::mem::transmute(0x49ecd0usize); // comp_fixture_resolve_league_clubs
    // comp_is_reserve_or_minor_comp is stdcall(comp) ret 4; called with ecx = G_RESERVE_COMP (same stack as thiscall)
    let reserve_or_minor: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_COMP_IS_RESERVE_OR_MINOR_COMP);
    let venue_side: unsafe extern "C" fn(usize, usize) -> u8 = core::mem::transmute(FN_COMP_UTIL_GET_FIXTURE_VENUE_SIDE);
    let round_label: unsafe extern "C" fn(usize, u32, usize, u32) -> i32 = core::mem::transmute(FN_COMP_UTIL_FORMAT_FIXTURE_ROUND_LABEL);
    let round_variant: unsafe extern "C" fn(i32) -> u8 = core::mem::transmute(FN_COMP_UTIL_GET_ROUND_VARIANT);
    let text_width: unsafe extern "C" fn(u32, usize) -> i32 = core::mem::transmute(FN_FONT_TEXT_WIDTH);
    let font_height: unsafe extern "C" fn(u32) -> i32 = core::mem::transmute(FN_FONT_GET_HEIGHT);
    let own_comp: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_FRIENDLY_IS_OWN_COMP_RECORD);
    let comp_current: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x5c29b0usize); // Friendly::is_comp_current_on_date
    let friendly_comp: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_FRIENDLY_IS_FRIENDLY_COMPETITION);
    // transfer_manager_get_selection_limit is stdcall(fixture, out) ret 8 (ecx unused)
    let selection_limit: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_TRANSFER_MANAGER_GET_SELECTION_LIMIT);
    let entry_matches: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_MATCH_STATS_FIXTURE_ENTRY_MATCHES);
    let match_weather: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_WEATHER_GET_MATCH_WEATHER);
    // weather_format_description and pitch_condition_text are stdcall(out, ..) ret 8, called with ecx = G_WEATHER
    let weather_text: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_WEATHER_FORMAT_DESCRIPTION);
    let pitch_condition: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(FN_WEATHER_GET_PITCH_CONDITION);
    let pitch_text: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x923b60usize); // pitch_condition_text
    let loan_contract: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_LOAN_CONTRACT);
    let club_contract: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let needs_permit: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_PLAYER_NEEDS_WORK_PERMIT);
    let ineligible: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_INELIGIBILITY_CODE);
    let banned: unsafe extern "thiscall" fn(usize, usize, usize, u32, usize) -> u32 = core::mem::transmute(FN_DISCIPLINE_IS_PLAYER_BANNED);
    let duty_club: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize) -> usize = core::mem::transmute(FN_NATIONALTEAMS_GET_INTERNATIONAL_DUTY_CLUB);
    let injury_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_INJURY_GET_PLAYER_INJURY);
    let injury_status: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_INJURY_GET_PLAYER_INJURY_STATUS);
    let injury_text: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(0x62ee30usize); // Injury::describe_player_injury
    // player_stats_get_apps_for_club is stdcall(staff, club) ret 8, called with ecx = G_PLAYER_STATS
    let apps_for_club: unsafe extern "thiscall" fn(usize, usize, usize) -> u16 = core::mem::transmute(0x7c3a40usize);
    // database_format_name_from_strings: the caller pushes a 7th dword ([staff+0]); cdecl, so it is passed too
    let format_name: unsafe extern "C" fn(usize, usize, usize, usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_DATABASE_FORMAT_NAME_FROM_STRINGS);
    let language: unsafe extern "C" fn() -> i32 = core::mem::transmute(FN_LANGLIB_GET_LANGUAGE);

    let sm = SCREEN_MANAGER;
    const MAX: usize = usize::MAX;
    const BUF: usize = NEXT_MATCH_TEXT;
    // the original's frame ([esp+N] after the four registers): the objects passed by address in the same places
    // (+0x14 the row, +0x20 the Text, +0x30 the columns 3:5, +0x3c the fixture date, +0x4c the match date, +0x5c and
    // +0xb4 the fixture lists, +0x6c the selection limit, +0x84 the weather, +0xfc the shown players, +0x158 the name)
    let mut frame = [0u32; 0x400 / 4];
    let s = frame.as_mut_ptr() as usize;
    let at = |o: usize| s + o;
    let gender = |c: usize| read_i8(c + club::SHORT_GENDER_NAME) as i32;
    let pale = colour_word(G_GUI_COLOUR_PALE_GREY);
    let grey = colour_word(G_GUI_COLOUR_GREY);
    let yellow = colour_word(G_GUI_COLOUR_YELLOW);
    let orange = colour_word(NEXT_MATCH_COLOUR_ORANGE);

    write_u8(at(0x30), 3);
    write_u8(at(0x31), 5);
    write_u16(at(0x38), read_u16(G_GUI_COLOUR_YELLOW));
    exe::new_llist(at(0xb4));
    exe::new_llist(at(0x5c));
    text_new(at(0x20), 0);
    let club = exe::screenmanager_get_screen_param(sm, 0);
    let reserves = exe::screenmanager_get_screen_param(sm, 1);
    let fixture = exe::screenmanager_get_screen_param(sm, 3);
    let slot = read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize;
    let manager = read_usize((sm as isize + NEXT_MATCH_RECORD_KEY as isize + slot * 0x300) as usize);

    if fixture == 0 {
        let lines = ((bottom - top - 0x10) / font_height(3)) as u8;
        let text = if reserves != 0 {
            if is_national(club) != 0 { 0x98f7f0 } else { 0x98f7b8 } // 'B' international / reserve
        } else {
            0x98f788
        };
        lang(BUF, text);
        message(0x6e, top as usize, 0x30c, bottom as usize, 2, 0, lines as usize, 3, pale, BUF, 0);
    } else {
        covered!("tel02_1 next match tab");
        let fm = read_usize(G_FIXTURE_MANAGER);
        let club_id = read_u32(club);
        let arrows = add_area(current_human_display(), 0x6e, top as usize, 0x168, (top + 0x14) as usize, 2, 0, 1, 0, 1, 0, MAX);
        date_new(at(0x3c), read_u16(fixture + fixture::DAY) as u32, read_u16(fixture + fixture::YEAR) as u32);
        let weeks = if is_national(club) != 0 { 0x18 } else { 8 };
        let side = (reserves == 0) as u32;
        write_u8(at(0x88), side as u8);
        // the previous fixtures: from the match date minus the weeks to the day before
        let y1 = read_u16(date_sub_days(at(0x3c), at(0x9c), 1) + 2) as u32;
        let d1 = read_u16(date_sub_days(at(0x3c), at(0xa4), 1)) as u32;
        let yw = read_u16(date_sub_weeks(at(0x3c), at(0x94), weeks) + 2) as u32;
        let dw = read_u16(date_sub_weeks(at(0x3c), at(0x8c), weeks)) as u32;
        find_fixtures(fm, dw, yw, d1, y1, club_id, at(0x5c), u32::MAX, 0xffff_fffe, side);
        lang(at(0x1e4), 0x988078); // "Date"
        format(BUF, 0x988090, at(0x1e4)); // "<< %s"
        write_u32(at(0x5c + llist::CURSOR), read_u32(at(0x5c + llist::TAIL)));
        let last = if read_u16(at(0x5c)) != 0 { llist_prev(at(0x5c)) } else { 0 };
        if last != 0 && read_u8(last + fixture::HOME_GOALS) == 0xff {
            cell_button(sm, arrows, 0, 0, 0x30, grey, 0xc, 1, pale, BUF, 0x26, 0);
        } else {
            cell_text(sm, arrows, 0, 0, 0x30, grey, 0x2c, 1, pale, BUF);
        }
        llist_clear(at(0x5c));
        // the next fixtures: from the day after to the match date plus the weeks
        let yw = read_u16(date_add_weeks(at(0x3c), at(0x9c), weeks) + 2) as u32;
        let dw = read_u16(date_add_weeks(at(0x3c), at(0xa4), weeks)) as u32;
        let y1 = read_u16(date_add_days(at(0x3c), at(0x94), 1) + 2) as u32;
        let d1 = read_u16(date_add_days(at(0x3c), at(0x8c), 1)) as u32;
        find_fixtures(fm, d1, y1, dw, yw, club_id, at(0x5c), u32::MAX, 0xffff_fffe, side);
        lang(at(0x1e4), 0x988078); // "Date"
        format(BUF, 0x988088, at(0x1e4)); // "%s >>"
        if read_u16(at(0x5c)) != 0 {
            cell_button(sm, arrows, 1, 0, 0x30, grey, 0xc, 1, pale, BUF, 0x27, 0);
        } else {
            cell_text(sm, arrows, 1, 0, 0x30, grey, 0x2c, 1, pale, BUF);
        }
        llist_clear(at(0x5c));
        // the fixtures from today to the match date: the unplayed ones and those of the same competition group
        find_fixtures(fm, read_u16(G_CURRENT_DATE) as u32, read_u16(G_CURRENT_DATE_YEAR) as u32, read_u16(at(0x3c)) as u32,
            read_u16(at(0x3e)) as u32, club_id, at(0x5c), u32::MAX, 0xffff_fffe, side);
        write_u32(at(0x5c + llist::CURSOR), read_u32(at(0x5c + llist::HEAD)));
        let mut same_group = 0u32;
        let mut unplayed = 0u32;
        loop {
            let it = llist_next(at(0x5c));
            if it == 0 {
                break;
            }
            if read_u8(it + fixture::HOME_GOALS) != 0xff {
                continue;
            }
            unplayed += 1;
            let a = read_i32(it + fixture::COMP_ID);
            let b = read_i32(fixture + fixture::COMP_ID);
            if a >= 0 && b >= 0 {
                let table = read_usize(G_COMP_OBJECT_TABLE);
                let g = read_u8(read_usize(table + a as usize * 4) + NEXT_MATCH_COMP_GROUP);
                if g != 0xff && g == read_u8(read_usize(table + b as usize * 4) + NEXT_MATCH_COMP_GROUP) {
                    same_group += 1;
                }
            }
        }
        llist_clear(at(0x5c));

        let links = add_area(current_human_display(), 0x212, top as usize, 0x30c, (top + 0x14) as usize, 2, 0, 1, 0, 1, 0, MAX);
        if resolve_league(fixture, at(0xf8), at(0xf4), at(0xf0)) != 0
            && reserve_or_minor(read_usize(G_RESERVE_COMP), read_usize(fixture + fixture::COMPETITION)) == 0
        {
            lang(BUF, 0x98f77c); // "Progress"
            cell_button(sm, links, 0, 0, 0x30, grey, 0xc, 1, pale, BUF, 0x1d, 0);
        }
        lang(BUF, 0x98f76c); // "Past Meetings"
        let home = read_usize(fixture + fixture::HOME_CLUB);
        let opponent = if club == home { read_usize(fixture + fixture::AWAY_CLUB) } else { home };
        // the "Past Meetings" popup: against the opponent and against `other` when it is someone else
        let popup = |other: usize| {
            let b = cell_button(sm, links, 1, 0, 0x30, grey, 0xc, 1, pale, BUF, 0, 0);
            let list = add_area(current_human_display(), 0, 0, 0, 0, 0, 0, 0, 0, 0x130, colour_word(NEXT_MATCH_COLOUR_POPUP), b);
            lang(BUF, 0x98f74c, gender(opponent), gender(opponent), opponent + club::SHORT_NAME); // "Against {}<%s - Club Name>{}"
            add_item(current_human_display(), 2, 0, 0, 0, 0, 0, 0, 0x10, colour_word(NEXT_MATCH_COLOUR_POPUP), yellow, 1, 1,
                colour_word(G_GUI_COLOUR_BLACK), BUF, 0, 0x1e, 0, list);
            if other != opponent {
                lang(BUF, 0x98f74c, gender(other), gender(other), other + club::SHORT_NAME);
                add_item(current_human_display(), 2, 0, 0, 0, 0, 0, 0, 0x10, colour_word(NEXT_MATCH_COLOUR_POPUP), yellow, 1, 1,
                    colour_word(G_GUI_COLOUR_BLACK), BUF, 0, 0x1e, other, list);
            }
        };
        let mut shown = false;
        if is_national(club) != 0 && club_is_national(club) == 0 && manager != 0 {
            let national = read_usize(manager + staff::NATIONAL_JOB);
            if national != 0 && read_usize(national + club::NATION) != read_usize(club + club::NATION) {
                popup(national);
                shown = true;
            }
        }
        if !shown {
            let job = if manager != 0 { read_usize(manager + staff::CLUB_JOB) } else { 0 };
            if is_national(club) == 0 && job != 0 && job != club && job != linked_b_team(club, 0, 1) {
                popup(job);
            } else {
                cell_button(sm, links, 1, 0, 0x30, grey, 0xc, 1, pale, BUF, 0x1e, 0);
            }
        }

        // the title: the opponent and the venue
        let y = top + 0x19;
        date_new(at(0x4c), read_u16(fixture + fixture::DAY) as u32, read_u16(fixture + fixture::YEAR) as u32);
        match venue_side(club, fixture) {
            1 => { lang(BUF, 0x98f734, gender(opponent), gender(opponent), opponent + club::SHORT_NAME); } // "(Home)"
            2 => { lang(BUF, 0x98f71c, gender(opponent), gender(opponent), opponent + club::SHORT_NAME); } // "(Away)"
            _ => next_match_copy_text(BUF, opponent + club::SHORT_NAME),
        }
        button_item(sm, 0x6e, y as usize, 0x30c, (y + 0x23) as usize, 2, 0, 0xc, 3, yellow, BUF, 0x7d5, opponent);
        let y = y + 0x28;
        let lines = ((bottom - y - 0x10) / font_height(3)) as u8;
        write_u8(at(0x48), lines);
        let table = add_area(current_human_display(), 0x6e, y as usize, 0x30c, bottom as usize, 2, at(0x30), read_u32(at(0x48)) as usize, 0, 2, 0, MAX);
        let visible = lines as i32;

        let row = || read_i32(at(0x14));
        let next_row = || write_i32(at(0x14), read_i32(at(0x14)) + 1);
        let label = |t: usize| { cell_text(sm, table, 0, row() as usize, 1, 0, 1, 3, pale, t); };
        let value = |t: usize| { cell_text(sm, table, 1, row() as usize, 1, 0, 1, 2, yellow, t); };
        // a link to a fixture's competition: in the table, not the reserve key, not a friendly's own, current on its date
        let real_comp = |f: usize, date: usize| {
            let comp = read_usize(f + fixture::COMPETITION);
            comp != 0
                && read_usize(read_usize(G_COMP_OBJECT_TABLE) + read_u32(f + fixture::COMP_ID) as usize * 4) != 0
                && read_u32(comp) != read_u32(KEY_CLUB_COMP_RESERVE)
                && own_comp(read_usize(G_FRIENDLY), comp) == 0
                && {
                    date_new(date, read_u16(f + fixture::DAY) as u32, read_u16(f + fixture::YEAR) as u32);
                    comp_current(read_usize(G_FRIENDLY), read_usize(f + fixture::COMPETITION), date) != 0
                }
        };
        let match_day = || read_u16(G_CURRENT_DATE_YEAR) == read_u16(at(0x4e)) && read_u16(G_CURRENT_DATE) == read_u16(at(0x4c));
        // a date is past the match date (years unsigned, days signed)
        let after_match = |d: usize| {
            let (day, year) = (read_u16(d), read_u16(d + 2));
            year > read_u16(at(0x4e)) || (year == read_u16(at(0x4e)) && (day as i16) > read_i16(at(0x4c)))
        };

        let mut rows_counted = 0i32;
        let mut round_right = 0i32;
        for pass in 0..2 {
            let draw = pass != 0;
            let width: i32 = if rows_counted <= visible { 0x30a } else { 0x2f6 };
            write_i32(at(0x14), 0);
            // the round and its underline
            if draw {
                round_label(fixture, 2, BUF, 0x7d0);
                round_right = text_width(3, BUF) + 0x73;
                if real_comp(fixture, at(0x130)) {
                    let id = (read_u32(fixture + fixture::COMP_ID) << 11 | (read_u16(fixture + fixture::ROUND_PREFIX) as u32 & 0x7ff)) << 11
                        | (read_u16(fixture + fixture::ROUND_NAME) as u32 & 0x7ff);
                    add_item(current_human_display(), 2, 0x73, 0, round_right as usize, 0, 0, row() as usize, 1, 0, 0, 1, 3, orange, BUF, 0, 0x7e5, id as usize, table);
                } else {
                    add_item(current_human_display(), 1, 0x73, 0, round_right as usize, 0, 0, row() as usize, 1, 0, 0, 1, 3, orange, BUF, 0, 0, 0, table);
                }
            }
            next_row();
            if draw {
                add_item(current_human_display(), 1, 0x73, 0, round_right as usize, 0, 0, row() as usize, 0xc400, orange, 0, 0, 0, 0, 0, 0, 0, 0, table);
            }
            next_row();
            // Date
            if draw {
                lang(BUF, 0x98f714); // "  Date"
                label(BUF);
                long_date(at(0x4c), at(0x1e4), 0xc8, 0xff);
                if match_day() {
                    lang(BUF, 0x98f6fc, 0x64, at(0x1e4)); // "(today)"
                } else if days_since(at(0x4c), read_u32(G_CURRENT_DATE), read_u32(G_CURRENT_DATE_LEAP)) == 1 {
                    lang(BUF, 0x98f6e0, 0x64, at(0x1e4)); // "(tomorrow)"
                } else {
                    let d = days_since(at(0x4c), read_u32(G_CURRENT_DATE), read_u32(G_CURRENT_DATE_LEAP));
                    lang(BUF, 0x98f6bc, 0x64, at(0x1e4), d); // "(<%d - days> days)"
                }
                value(BUF);
            }
            next_row();
            // Venue
            if draw {
                lang(BUF, 0x98f6b4); // "  Venue"
                label(BUF);
                let stadium = read_usize(fixture + fixture::STADIUM);
                if stadium != 0 {
                    let city = read_usize(stadium + stadium::CITY);
                    if city != 0 {
                        format(BUF, 0x98f6ac, stadium + stadium::NAME, city + NEXT_MATCH_CITY_NAME); // "%s, %s"
                    } else {
                        next_match_copy_text(BUF, stadium + stadium::NAME);
                    }
                } else {
                    lang(BUF, 0x98f68c, NEXT_MATCH_EMPTY_TEXT); // "Unknown<%s - COMMENT - venue>"
                }
                value(BUF);
            }
            next_row();
            // Aggregate: the best of three series, or the aggregate score
            if read_u8(fixture + fixture::TIE_RULES) & 0x10 != 0 {
                if draw {
                    lang(BUF, 0x98f680); // "  Aggregate"
                    label(BUF);
                    let (h, a) = (read_u8(fixture + fixture::HOME_RESULT_MARK), read_u8(fixture + fixture::AWAY_RESULT_MARK));
                    let leader = if h == 1 && a == 0 { fixture::HOME_CLUB } else if h == 0 && a == 1 { fixture::AWAY_CLUB } else { 0 };
                    if h == 1 && a == 1 {
                        lang(BUF, 0x98f664); // "1-1 in best of three games"
                    } else if leader != 0 {
                        let c = read_usize(fixture + leader);
                        lang(BUF, 0x98f634, gender(c), gender(c), c + club::SHORT_NAME); // "... lead 1-0 in best of three games"
                    } else {
                        lang(BUF, 0x98f610); // "Best of three games to be played"
                    }
                    value(BUF);
                }
                next_row();
            } else if read_i8(fixture + fixture::HOME_AGGREGATE) > -1 {
                if draw {
                    lang(BUF, 0x98f680); // "  Aggregate"
                    label(BUF);
                    let h = read_i8(fixture + fixture::HOME_AGGREGATE) as i32;
                    let a = read_i8(fixture + fixture::AWAY_AGGREGATE) as i32;
                    if h == a {
                        lang(BUF, 0x98f5dc, h, a); // "<%d>-<%d> on aggregate"
                    } else if h > a {
                        let c = read_usize(fixture + fixture::HOME_CLUB);
                        lang(BUF, 0x98f594, gender(c), gender(c), c + club::SHORT_NAME, h, a); // "... lead <%d>-<%d> on aggregate"
                    } else {
                        let c = read_usize(fixture + fixture::AWAY_CLUB);
                        lang(BUF, 0x98f594, gender(c), gender(c), c + club::SHORT_NAME, a, h);
                    }
                    value(BUF);
                }
                next_row();
            }
            // Match Rules: the squad limit
            if draw {
                lang(BUF, 0x98f584); // "  Match Rules"
                label(BUF);
                selection_limit(read_usize(G_TRANSFER_MANAGER), fixture, at(0x6c));
            }
            let first_rule = row();
            if read_u8(at(0x6c)) == 2 {
                if draw {
                    lang(BUF, 0x98f550, read_i8(at(0x71)) as i32); // "Max <%d> foreign players in the match squad"
                    value(BUF);
                }
                next_row();
            }
            if read_u8(at(0x6c)) == 1 {
                if draw {
                    if row() > first_rule {
                        cell_text(sm, table, 0, row() as usize, 1, 0, 1, 3, pale, 0);
                    }
                    lang(BUF, 0x98f51c, read_i8(at(0x71)) as i32); // "Max <%d> non-EU players in the match squad"
                    value(BUF);
                }
                next_row();
            }
            if read_u8(at(0x6c)) == 3 {
                if draw {
                    if row() > first_rule {
                        cell_text(sm, table, 0, row() as usize, 1, 0, 1, 3, pale, 0);
                    }
                    lang(BUF, 0x98f4dc, read_i8(at(0x71)) as i32, read_i8(at(0x6d)) as i32); // "Max <%d> players over <%d> yrs ..."
                    value(BUF);
                }
                next_row();
            }
            if row() == first_rule {
                if draw {
                    lang(BUF, 0x98f4c4); // "No player restrictions"
                    value(BUF);
                }
                next_row();
            }
            // the tie rules; once one is listed, the later ones say "still level"
            let rules = || read_u16(fixture + fixture::TIE_RULES);
            let mut listed = 0u32;
            let rule = |text: usize| {
                if draw {
                    lang(BUF, text);
                    value(BUF);
                }
                next_row();
            };
            if round_variant(read_i16(fixture + fixture::ROUND_NAME) as i32) == 4 && rules() & 0x4004 == 0 {
                rule(0x98f488); // "Away goals used if scores are level after ninety minutes"
            }
            if rules() & 2 != 0 {
                rule(if rules() & 0x80 != 0 { 0x98f440 } else { 0x98f404 }); // golden goal / extra time
                listed = 1;
            }
            if round_variant(read_i16(fixture + fixture::ROUND_NAME) as i32) == 4 && rules() & 0x4000 != 0 && rules() & 2 != 0 {
                rule(0x98f3cc); // "Away goals used if scores are level after extra time"
            }
            if rules() & 1 != 0 {
                rule(if listed != 0 { 0x98f3a0 } else { 0x98f364 }); // penalty shoot-out
                listed += 1;
            }
            if rules() & 0x20 != 0 {
                rule(if listed != 0 { 0x98f334 } else { 0x98f2f8 }); // US-style shoot-out
                listed += 1;
            }
            if read_u8(fixture + fixture::FLAGS) & 4 != 0 {
                rule(if listed != 0 { 0x98f2cc } else { 0x98f294 }); // replay
                listed += 1;
            }
            for (bit, team) in [(0x800u16, fixture::HOME_CLUB), (0x1000, fixture::AWAY_CLUB)] {
                if rules() & bit != 0 {
                    if draw {
                        let c = read_usize(fixture + team);
                        let text = if listed != 0 { 0x98f24c } else { 0x98f1f8 }; // the seeded team wins
                        lang(BUF, text, gender(c), gender(c), c + club::SHORT_NAME);
                        value(BUF);
                    }
                    next_row();
                    if bit == 0x800 {
                        listed += 1; // tel02_1 the away seeding does not count as listed
                    }
                }
            }
            if draw {
                let w = read_u16(fixture + fixture::SQUAD_RULES) as u32;
                lang(BUF, 0x98f1bc, w & 0xf, (w >> 4) & 0xf); // "<%d> subs named, maximum <%d> used"
                value(BUF);
            }
            next_row();
            // Last Meeting
            if draw {
                lang(BUF, 0x98f1ac); // "  Last Meeting"
                label(BUF);
                write_u32(at(0xd8), read_u32(G_CURRENT_DATE));
                write_u16(at(0x82), read_u16(G_CURRENT_DATE_YEAR));
                write_u8(at(0x57), 2);
                scan_fixtures(fm, at(0x82), at(0xd8), at(0x57), 0, u32::MAX, u32::MAX, u32::MAX, club_id, read_u32(opponent),
                    u32::MAX, u32::MAX, u32::MAX, 1, at(0xb4), 1, 0xffff_fffe, read_u32(at(0x88)), 1);
                let mut it = 0;
                if read_u16(at(0xb4)) != 0 {
                    text_clear(at(0x20));
                    write_u32(at(0xb4 + llist::CURSOR), read_u32(at(0xb4 + llist::HEAD)));
                    loop {
                        it = llist_next(at(0xb4));
                        if it == 0 || read_u8(it + fixture::HOME_GOALS) != 0xfe {
                            break;
                        }
                    }
                }
                if it != 0 {
                    let (a, b) = if read_i8(it + fixture::HOME_GOALS_EXTRA_TIME) > -1 {
                        (read_i8(it + fixture::HOME_GOALS_EXTRA_TIME), read_i8(it + fixture::AWAY_GOALS_EXTRA_TIME))
                    } else {
                        (read_i8(it + fixture::HOME_GOALS), read_i8(it + fixture::AWAY_GOALS))
                    };
                    format(at(0xdc), 0x98c114, a as i32, b as i32); // "%d-%d"
                    round_label(it, 3, at(0x324), 0xc8);
                    date_new(at(0x148), read_u16(it + fixture::DAY) as u32, read_u16(it + fixture::YEAR) as u32);
                    short_date(at(0x148), at(0x1d0), 0x14, 0xff);
                    let (h, w) = (read_usize(it + fixture::HOME_CLUB), read_usize(it + fixture::AWAY_CLUB));
                    lang(BUF, 0x98f154, gender(h), gender(h), gender(w), gender(w), 0x64, h + club::SHORT_NAME, at(0xdc),
                        w + club::SHORT_NAME, at(0x324), at(0x1d0)); // "<home> <result> <away>, <fixture> <date>"
                    text_set(at(0x20), BUF);
                    text_link(at(0x20), h + club::SHORT_NAME, 0x7d5, h);
                    if entry_matches(read_usize(G_MATCH_STATS), read_u32(it + fixture::MATCH_STATS_SLOT), read_u32(it + fixture::HOME_CLUB_ID),
                        read_u32(it + fixture::AWAY_CLUB_ID), read_u16(it + fixture::DAY) as u32, read_u16(it + fixture::YEAR) as u32) != 0
                    {
                        text_link(at(0x20), at(0xdc), 0x7d3, read_usize(it + fixture::MATCH_STATS_SLOT));
                    }
                    text_link(at(0x20), w + club::SHORT_NAME, 0x7d5, w);
                    if real_comp(it, at(0x140)) {
                        text_link(at(0x20), at(0x324), 0x7d0, read_usize(it + fixture::COMPETITION));
                    }
                    text_draw(at(0x20), table, (width * 3 - 0x150) / 8 + 0x71, width, at(0x14), 2, yellow, 2, 0);
                } else {
                    value(0x98ba0c); // "-"
                }
            }
            next_row();
            // Weather or Weather Forecast: known within two weeks of the match
            if draw {
                lang(BUF, if match_day() { 0x98f148 } else { 0x98f134 });
                label(BUF);
                if after_match(date_add_weeks(G_CURRENT_DATE, at(0x150), 2)) {
                    match_weather(read_usize(G_WEATHER), at(0x84), fixture);
                    weather_text(read_usize(G_WEATHER), BUF, at(0x84));
                } else {
                    lang(BUF, 0x98f110, NEXT_MATCH_EMPTY_TEXT); // "Unknown<%s - COMMENT - conditions>"
                }
                value(BUF);
            }
            next_row();
            // Pitch Condition: on match day
            if match_day() {
                if draw {
                    lang(BUF, 0x98f0fc); // "  Pitch Condition"
                    label(BUF);
                    if after_match(date_add_weeks(G_CURRENT_DATE, at(0x138), 6)) {
                        match_weather(read_usize(G_WEATHER), at(0x84), fixture);
                        write_u8(at(0xc8), pitch_condition(read_usize(G_WEATHER), at(0x84), fixture));
                        pitch_text(read_usize(G_WEATHER), BUF, read_u32(at(0xc8)));
                    } else {
                        lang(BUF, 0x98f110, NEXT_MATCH_EMPTY_TEXT);
                    }
                    value(BUF);
                }
                next_row();
            }
            // Referee
            if read_i16(fixture + fixture::REFEREE) != -1 {
                if draw {
                    lang(BUF, 0x98f0f0); // "  Referee"
                    label(BUF);
                    let r = (read_usize(G_OFFICIAL_TABLE) as isize + read_i16(fixture + fixture::REFEREE) as isize * NEXT_MATCH_OFFICIAL_SIZE as isize) as usize;
                    let text = if read_u32(G_DATABASE_NAMES_LANGUAGE) != 0 {
                        0x98f0b8 // "%s %s"
                    } else {
                        let d = read_i8(r + official::DISCIPLINE);
                        if d == 0x14 { 0x98f0e0 } else if d > 0xe { 0x98f0d0 } else if d >= 6 { 0x98f0b8 } else { 0x98f0c0 } // Harsh, Strict, -, Lenient
                    };
                    format(BUF, text, read_usize(r + official::FIRST_NAME), read_usize(r + official::SECOND_NAME));
                    value(BUF);
                }
                next_row();
            }
            // the team news of both teams
            if reserves == 0 {
                for side_k in 0..2 {
                    let team = read_usize(fixture + if side_k != 0 { fixture::AWAY_CLUB } else { fixture::HOME_CLUB });
                    let mut header = false;
                    core::ptr::write_bytes(at(0xfc) as *mut u8, 0, 0x32);
                    for case in 0..6 {
                        for k in 0..0x32usize {
                            let p = read_usize(team + club::SQUAD + k * 4);
                            if p == 0 || read_u8(at(0xfc) + k) != 0 {
                                continue;
                            }
                            if loan_contract(read_usize(G_CONTRACT_MANAGER), p) != 0 || needs_permit(read_usize(G_TRANSFER_MANAGER), p, team, 0) != 0 {
                                continue;
                            }
                            let squad = read_u8(p + staff::PLAYING_SQUAD);
                            let other_squad = if is_national(club) != 0 { squad & 8 != 0 } else { squad & 2 != 0 };
                            if other_squad {
                                continue;
                            }
                            format_name(read_usize(p + staff::FIRST_NAME), read_usize(p + staff::SECOND_NAME), read_usize(p + staff::COMMON_NAME), at(0x158), 0x78, 1, read_usize(p));
                            let mut duty = 0usize;
                            let today = || days_since(at(0x4c), read_u32(G_CURRENT_DATE), read_u32(G_CURRENT_DATE_LEAP));
                            let injury = || injury_of(read_usize(G_INJURY), p);
                            let news = match case {
                                0 => team != read_usize(p + staff::NATIONAL_JOB) && {
                                    duty = duty_club(read_usize(G_NATIONAL_TEAMS), p, at(0x4c), team, 0, 0);
                                    duty != 0
                                },
                                1 => ineligible(read_usize(G_TRANSFER_MANAGER), p, fixture) != 0,
                                2 => {
                                    let comp = read_usize(fixture + fixture::COMPETITION);
                                    comp != 0 && banned(read_usize(G_DISCIPLINE), p, comp, same_group, fixture) != 0
                                }
                                3 => injury() != 0 && injury_status(read_usize(G_INJURY), p) == 2
                                    && read_i16(injury() + injury_entry::TRAINING_RETURN_DAYS) as i32 > today(),
                                4 => injury() != 0 && read_i16(injury() + injury_entry::TRAINING_DAYS_LEFT) as i32 <= today()
                                    && read_i16(injury() + injury_entry::DAYS_LEFT) as i32 > today() + 3,
                                _ => is_national(team) == 0
                                    && days_since(G_CURRENT_DATE, read_u32(p + staff::DATE_JOINED_CLUB), read_u32(p + staff::DATE_JOINED_CLUB_LEAP_YEAR)) < 100
                                    && apps_for_club(read_usize(G_PLAYER_STATS), p, team) == 0
                                    && friendly_comp(read_usize(G_FRIENDLY), read_usize(fixture + fixture::COMPETITION)) == 0
                                    && unplayed as u16 == 1
                                    && {
                                        let c = club_contract(read_usize(G_CONTRACT_MANAGER), p, team);
                                        c != 0 && matches!(read_u8(c + contract::SQUAD_STATUS) & 0xf0, 0x10 | 0x20 | 0x30)
                                    },
                            };
                            if !news {
                                continue;
                            }
                            if draw {
                                if !header {
                                    lang(BUF, 0x98f090, gender(team), gender(team), team + club::SHORT_NAME); // "  <team> News"
                                    text_clear(at(0x20));
                                    text_set(at(0x20), BUF);
                                    text_link(at(0x20), team + club::SHORT_NAME, 0x7d5, team);
                                    text_draw(at(0x20), table, 0x71, width, at(0x14), 3, pale, 2, 0);
                                    header = true;
                                }
                                if case == 0 {
                                    if is_national(duty) == 0 && (language() == 0 || language() == 1) {
                                        format(BUF, 0x98f078, at(0x158), duty + club::SHORT_NAME); // "%s away playing with %s"
                                    } else {
                                        lang(BUF, 0x98f014, gender(duty), gender(duty), at(0x158), duty + club::SHORT_NAME); // "... on international duty with ..."
                                    }
                                } else {
                                    if case == 3 || case == 4 {
                                        injury_text(read_usize(G_INJURY), p, at(0x2ac), 0x78, 0);
                                    }
                                    let text = [0, 0x98efe4, 0x98efb8, 0x98ef6c, 0x98ef1c, 0x98eeec][case.min(5)]; // ineligible, suspended, out, doubtful, debut
                                    lang(BUF, text, at(0x158), at(0x2ac));
                                }
                                text_clear(at(0x20));
                                text_set(at(0x20), BUF);
                                text_link(at(0x20), at(0x158), 0x7d7, p);
                                if case == 0 {
                                    text_link(at(0x20), duty + club::SHORT_NAME, 0x7d5, duty);
                                }
                                let (c0, c1) = (read_u8(at(0x30)) as i32, read_u8(at(0x31)) as i32);
                                text_draw(at(0x20), table, (width - 0x70) * c0 / (c0 + c1) + 0x71, width, at(0x14), 2, yellow, 2, 0);
                            }
                            next_row();
                            write_u8(at(0xfc) + k, 1);
                        }
                    }
                }
            }
            rows_counted = row();
        }
    }
    text_drop(at(0x20));
    llist_drop(at(0x5c));
    llist_drop(at(0xb4));
}

/// strcpy of the original (rep movs): `src` with its terminator into `dst`.
unsafe fn next_match_copy_text(dst: usize, src: usize) {
    let mut k = 0;
    loop {
        let c = read_u8(src + k);
        write_u8(dst + k, c);
        if c == 0 {
            break;
        }
        k += 1;
    }
}

/// Comparer of `club_screen_draw_next_match_tab`: side by side over the display list (nothing is written outside it;
/// the fixture lists and the Text are the frame's own, built and freed inside).
pub(crate) unsafe extern "C" fn compare_club_screen_draw_next_match_tab(top: i32, bottom: i32) {
    let o = original_fn::<unsafe extern "C" fn(i32, i32)>(%%I0%%);
    let extra: [(usize, usize); 0] = [];
    compare_by_display_list_with("Club screen next match tab", &extra, || o(top, bottom), || club_screen_draw_next_match_tab(top, bottom));
}

/// `key_nation_format_club_season` (0x669630): cdecl(year, club, buffer, size) -> int, ret 0.
const FIXTURES_FN_FORMAT_CLUB_SEASON: usize = 0x669630;
/// `add_club_fixture_row` (0x483a90): cdecl(club, fixture, area, row, date_link_mode), ret 0; nonzero when the row is
/// the club's next fixture [inference: the screen keeps that row to scroll to].
const FIXTURES_FN_ADD_CLUB_FIXTURE_ROW: usize = 0x483a90;
/// The colours this screen pushes that have no name in globals.rs (the words at these addresses).
const FIXTURES_COLOUR_POPUP: usize = 0xae31aa;
const FIXTURES_COLOUR_VENUE: usize = 0xaebde8;
const FIXTURES_COLOUR_COMP: usize = 0xaebdde;
const FIXTURES_COLOUR_RESULT_BG: usize = 0xae3192;
/// "---": the empty result column of a friendly, tour or tournament row.
const FIXTURES_NO_RESULT_TEXT: usize = 0x98f858;
/// The empty langlib argument (`""`).
const FIXTURES_EMPTY_ARG: usize = 0xad9c64;
/// A friendly match record (`friendly_match`): nonzero once it can no longer be cancelled [inference: with it set the
/// cancel mark (item 0x16) is not offered].
const FIXTURES_FRIENDLY_LOCKED: usize = 0x8;
/// A tour record (0x5c75a0 status 6): the touring club, the nation toured and a flag that hides the cancel mark
/// [inference].
const FIXTURES_TOUR_CLUB: usize = 0x0;
const FIXTURES_TOUR_NATION: usize = 0x4;
const FIXTURES_TOUR_LOCKED: usize = 0x103;
/// The competition's id inside the tournament competition object [inference: the value of the 0x7d0 link].
const FIXTURES_COMP_ID: usize = 0x4;
/// The shared text buffer.
const FIXTURES_TEXT: usize = 0xde1f64;

/// strcpy(dst, src) the way the original inlines it (repne scasb + rep movs, the terminator included).
unsafe fn fixtures_copy_text(dst: usize, src: usize) {
    let mut k = 0usize;
    loop {
        let b = read_u8(src + k);
        write_u8(dst + k, b);
        if b == 0 {
            break;
        }
        k += 1;
    }
}

/// Draws the fixtures tab of the club screen: the season navigation, the friendly buttons and one row per day of the
/// season that has a fixture, friendly, tour or tournament.
///
/// Screen params: 0 the club, 1 nonzero for the read-only view [inference: no friendly buttons, no rows for friendly
/// dates], 0xc the season year shown, 0xd the fixture list (created here when 0), 0xe nonzero while a friendly is
/// being cancelled (Cancel highlighted, cancel marks on the rows), 0xf nonzero to scroll to the club's next fixture,
/// 0x10 the scroll row otherwise.
///
/// The header area (x 0x6e..0x168, `top`..`top`+0x14) holds "<< Season" (or "<< Year" for a national team) and
/// "Season >>": the first is a button (id 0x10) when the season year is after the club's first fixture year
/// (`get_club_first_fixture_year`) or after the current year, grey text otherwise; the second a button (id 0x11) while
/// the year is before the last one. The last year is `get_club_last_fixture_year`, at least the current year; for a
/// club whose nation's season spans two years (`key_nation_record::SPLIT_SEASON_FLAG` of its key nation entry), before
/// the nation's season end day (`SEASON_END_DAY`) the shown year drops by one (as a dword, + 0xffff, unless it is
/// 2001) and so does the last year; from that day on the last year drops by one unless it is the current year. For a
/// club the human controls (`human_manager_current_controls_club`), outside the read-only view, a second area
/// (0x212..0x30c) holds Cancel (id 0x12, style 0x70 while cancelling, 0x30 otherwise) and Arrange, a popup under
/// Arrange with Home Friendly (3), Away Friendly (4), Tour (0xb) and Tournament (0xa), all of id 0x13.
///
/// The dates run from the season start (the key nation's season end day of the shown year for a split season club,
/// day 0 otherwise; day 0 of 2001 when the year is 2001) up to the same day one year later. The caption "Season
/// <label>" (`key_nation_format_club_season`; the bare label for a national team) goes in a text item 0x23 high, then
/// the list area of 6 columns (5, 0xa, 2, 1, 0xc, 3) fills the rest, as many rows as font 3 fits in
/// `bottom` − y − 0x10 (the row count is the low byte of a dword whose upper bytes are those of the key nation entry
/// pointer, as the original pushes it). Without a list (param 0xd = 0) a new LLIST is created, stored in param 0xd and
/// filled by `FIXTURE_MANAGER::find_fixtures` (start, end, the club's id, the list, −1, −2, read-only view = 0).
///
/// Then for each day: the fixtures dated that day each get a row (`add_club_fixture_row`, link mode 0x14); a fixture
/// row whose helper returns nonzero is remembered as the row to scroll to; a fixture of the club's own friendly
/// competitions (`Friendly::is_own_comp_record`) still to be played (today before its date) gets the cancel mark (item
/// 0x15 over x 0x6e..0x2f6) while cancelling. Fixtures before the day are skipped. A day with no fixture, outside the
/// read-only view, asks `Friendly::get_club_date_status`: 5 (a friendly) gives the date (dark blue), the opponent
/// "  <short name> ?" as a link (yellow when a human manages it), its nation's three-letter name as a link to the
/// national team when it differs from the club's, H or A, "  Friendly", "---" and, while cancelling and not locked,
/// the cancel mark (0x16); 6 (a tour) the date, "  Tba", the nation toured, A, "  Tour Of <nation>", "---" and the
/// cancel mark (0x17) for the touring club the human controls (while cancelling, only when not locked); 3 (a
/// tournament) the date, "  Tba", H or A (host or not), the tournament name (a link 0x7d0 to its competition when it
/// exists), "---" and, with no competition yet, the cancel mark (0x18; while cancelling only when not under way).
///
/// At the end the list scrolls (`Area::set_scroll_row`) to the remembered row − 7 (at least 0) with param 0xf set,
/// which is then cleared, or to param 0x10; param 0x11 gets the list area. While cancelling, a full-screen button item
/// (0, 0, 0x31f, 0x257: these two are patched in the 1920x1080 exe and are read from the exe) with id 0x19 catches the
/// clicks.
/// Original: club_screen_draw_fixtures_tab (0x45f380, club_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn club_screen_draw_fixtures_tab(top: i32, bottom: i32) {
    covered!("tel02_1 fixtures tab");
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let season_label: unsafe extern "C" fn(u32, usize, usize, i32) -> i32 = core::mem::transmute(FIXTURES_FN_FORMAT_CLUB_SEASON);
    let fixture_row: unsafe extern "C" fn(usize, usize, usize, usize, u32) -> u32 = core::mem::transmute(FIXTURES_FN_ADD_CLUB_FIXTURE_ROW);
    let date_status: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, u32, u32, u32, u32) -> u8 =
        core::mem::transmute(FN_FRIENDLY_GET_CLUB_DATE_STATUS);
    let area: DisplayListFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    let item: DisplayLineFn = core::mem::transmute(FN_DISPLAY_ADD_ITEM);
    let button: ListCellFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_BUTTON);
    let label: ListCellTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
    let text_item: ManagerTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_TEXT_ITEM);
    let button_item: ManagerBoxFn = core::mem::transmute(FN_SCREENMANAGER_ADD_BUTTON_ITEM);
    let sm = SCREEN_MANAGER;
    let c = |a: usize| colour_word(a);
    let mut d1 = [0u8; 8];
    let mut d2 = [0u8; 8];
    let mut out = [0u8; 8];
    let mut tmp = [0u8; 8];
    let d1p = d1.as_mut_ptr() as usize;
    let d2p = d2.as_mut_ptr() as usize;
    let outp = out.as_mut_ptr() as usize;
    let tmpp = tmp.as_mut_ptr() as usize;
    exe::date_date_54a110(d1p);
    exe::date_date_54a110(d2p);
    let mut columns: [u8; 6] = [5, 0xa, 2, 1, 0xc, 3];
    let club = exe::screenmanager_get_screen_param(sm, 0);
    let read_only = exe::screenmanager_get_screen_param(sm, 1);
    let mut year = exe::screenmanager_get_screen_param(sm, 0xc) as u32;
    let mut list = exe::screenmanager_get_screen_param(sm, 0xd);
    let cancelling = exe::screenmanager_get_screen_param(sm, 0xe);
    let scroll_to_next = exe::screenmanager_get_screen_param(sm, 0xf);
    let header = area(current_human_display(), 0x6e, top as usize, 0x168, (top + 0x14) as usize, 2, 0, 1, 0, 1, 0, usize::MAX);
    let nation_entry = (G_LEAGUE_TABLE as isize + exe::key_nation_index_of_club(club) as i8 as isize * 0x48) as usize;
    let fm = read_usize(G_FIXTURE_MANAGER);
    let mut last = get_club_last_fixture_year(fm, club);
    let current_year = read_u16(G_CURRENT_DATE_YEAR);
    if last < current_year {
        last = current_year;
    }
    let national = |x: usize| exe::database_is_national_team(x) != 0;
    if !national(club) && read_u32(nation_entry + key_nation_record::SPLIT_SEASON_FLAG) != 0 {
        let day = read_i16(G_CURRENT_DATE) as i32;
        if day < read_u16(nation_entry + key_nation_record::SEASON_END_DAY) as i32 && year as u16 != 0x7d1 {
            year = year.wrapping_add(0xffff);
        } else if last == current_year {
            last = last.wrapping_add(1);
        }
        last = last.wrapping_add(0xffff);
    }
    // the previous / next season buttons
    text(FIXTURES_TEXT, if national(club) { 0x98f914 } else { 0x98e7ac }, 0x98e7c4); // "<%s - ignore> Year|Season", "<<"
    let first = get_club_first_fixture_year(fm, club);
    if year as u16 > first || year as u16 > current_year {
        button(sm, header, 0, 0, 0x30, c(G_GUI_COLOUR_GREY), 0xc, 1, c(G_GUI_COLOUR_PALE_GREY), FIXTURES_TEXT, 0x10, 0);
    } else {
        label(sm, header, 0, 0, 0x30, c(G_GUI_COLOUR_GREY), 0x2c, 1, c(G_GUI_COLOUR_PALE_GREY), FIXTURES_TEXT);
    }
    text(FIXTURES_TEXT, if national(club) { 0x98f900 } else { 0x98e790 }, 0x98e7a8); // "Year|Season <%s - ignore>", ">>"
    if (year as u16) < last {
        button(sm, header, 1, 0, 0x30, c(G_GUI_COLOUR_GREY), 0xc, 1, c(G_GUI_COLOUR_PALE_GREY), FIXTURES_TEXT, 0x11, 0);
    } else {
        label(sm, header, 1, 0, 0x30, c(G_GUI_COLOUR_GREY), 0x2c, 1, c(G_GUI_COLOUR_PALE_GREY), FIXTURES_TEXT);
    }
    // Cancel / Arrange and the friendly popup, for a club the human controls
    if !national(club) && exe::human_manager_current_controls_club(club, 0, 1) != 0 && read_only == 0 {
        let a = area(current_human_display(), 0x212, top as usize, 0x30c, (top + 0x14) as usize, 2, 0, 1, 0, 1, 0, usize::MAX);
        text(FIXTURES_TEXT, 0x98a1a0); // "Cancel"
        let style = if cancelling != 0 { 0x70 } else { 0x30 };
        button(sm, a, 0, 0, style, c(G_GUI_COLOUR_GREY), 0xc, 1, c(G_GUI_COLOUR_PALE_GREY), FIXTURES_TEXT, 0x12, 0);
        text(FIXTURES_TEXT, 0x98f8f8); // "Arrange"
        let arrange = button(sm, a, 1, 0, 0x30, c(G_GUI_COLOUR_GREY), 0xc, 1, c(G_GUI_COLOUR_PALE_GREY), FIXTURES_TEXT, 0, 0);
        let popup = area(current_human_display(), 0, 0, 0, 0, 0, 0, 0, 0, 0x130, c(FIXTURES_COLOUR_POPUP), arrange);
        for (caption, kind) in [(0x98f8e8usize, 3usize), (0x98f8d8, 4), (0x98f8d0, 0xb), (0x98f8c4, 0xa)] {
            text(FIXTURES_TEXT, caption); // "Home Friendly", "Away Friendly", "Tour", "Tournament"
            item(current_human_display(), 2, 0, 0, 0, 0, 0, 0, 0x10, c(FIXTURES_COLOUR_POPUP), c(G_GUI_COLOUR_YELLOW), 1, 1,
                c(G_GUI_COLOUR_BLACK), FIXTURES_TEXT, 0, 0x13, kind, popup);
        }
    }
    let mut y = top + 0x19;
    // the season's dates: from the start day of the shown year to the same day a year later
    let start_day = if !national(club) && read_u32(nation_entry + key_nation_record::SPLIT_SEASON_FLAG) != 0 {
        read_u16(nation_entry + key_nation_record::SEASON_END_DAY) as u32
    } else {
        0
    };
    let t = exe::new_date(tmpp, start_day, year);
    exe::date_operator_3d(d1p, outp, t);
    let t = exe::new_date(tmpp, read_u32(d1p), read_u32(d1p + 2).wrapping_add(1));
    exe::date_operator_3d(d2p, outp, t);
    if year as u16 == 0x7d1 {
        let t = exe::new_date(tmpp, 0, 0x7d1);
        exe::date_operator_3d(d1p, outp, t);
    }
    let mut label_buffer = [0u8; 0xc8];
    let lb = label_buffer.as_mut_ptr() as usize;
    season_label(year, club, lb, 0xc8);
    if !national(club) {
        text(FIXTURES_TEXT, 0x98f8a0, lb); // "Season <%s - season e.g. 1998/9>"
    } else {
        fixtures_copy_text(FIXTURES_TEXT, lb);
    }
    text_item(sm, 0x6e, y as usize, 0x30c, (y + 0x23) as usize, 2, 0, 0xc, 3, c(G_GUI_COLOUR_YELLOW), FIXTURES_TEXT, 1);
    y += 0x28;
    let h = exe::font_get_height(3);
    let rows = ((bottom - y - 0x10) / h) as u8;
    let rows_dword = (nation_entry & !0xff) | rows as usize; // the original pushes the dword whose low byte it set
    let lst = area(current_human_display(), 0x6e, y as usize, 0x30c, bottom as usize, 6, columns.as_mut_ptr() as usize, rows_dword, 0, 2, 0, usize::MAX);
    let mut row = 0usize;
    if list == 0 {
        let p = exe::crt_operator_new(0xe);
        list = if p != 0 { exe::new_llist(p) } else { 0 };
        exe::screenmanager_set_screen_param(sm, 0xd, list, 0);
        exe::fixture_manager_find_fixtures(fm, read_u32(d1p), read_u32(d1p + 2), read_u32(d2p), read_u32(d2p + 2),
            read_u32(club), list, u32::MAX, 0xffff_fffe, (read_only == 0) as u32);
    }
    let mut next_row: i32 = -1;
    write_u32(list + 0xa, read_u32(list + 2)); // the list cursor back to the first node
    let mut fx = exe::llist_get_next(list);
    let friendly = read_usize(G_FRIENDLY);
    loop {
        let (y1, y2) = (read_u16(d1p + 2), read_u16(d2p + 2));
        let before_end = y2 > y1 || (y2 == y1 && read_i16(d2p) > read_i16(d1p));
        if !before_end {
            break;
        }
        let (day, yr) = (read_i16(d1p), read_u16(d1p + 2));
        if fx != 0 {
            let f = exe::new_date(tmpp, read_u16(fx + fixture::DAY) as u32, read_u16(fx + fixture::YEAR) as u32);
            if yr == read_u16(f + 2) && day == read_i16(f) {
                // a fixture on this day
                let r = fixture_row(club, fx, lst, row, 0x14);
                if exe::friendly_is_own_comp_record(friendly, read_usize(fx + fixture::COMPETITION)) == 0 {
                    if r != 0 {
                        next_row = row as i32;
                    }
                } else if r != 0 {
                    next_row = row as i32;
                } else {
                    let f = exe::new_date(tmpp, read_u16(fx + fixture::DAY) as u32, read_u16(fx + fixture::YEAR) as u32);
                    let (cd, cy) = (read_i16(G_CURRENT_DATE), read_u16(G_CURRENT_DATE_YEAR));
                    let fy = read_u16(f + 2);
                    let still_to_play = cy < fy || (cy == fy && cd < read_i16(f));
                    if still_to_play && cancelling != 0 {
                        covered!("tel02_1 fixtures cancel mark of an own friendly fixture");
                        item(current_human_display(), 2, 0x6e, 0, 0x2f6, 0, 0, row, 1, 0, 0, 0xc, 1, c(G_GUI_COLOUR_PALE_GREY), 0, 0, 0x15, fx, lst);
                    }
                }
                row += 1;
                fx = exe::llist_get_next(list);
                fixtures_next_day(d1p, outp, tmpp);
                continue;
            }
            // a fixture before this day: skip it
            let f = exe::new_date(tmpp, read_u16(fx + fixture::DAY) as u32, read_u16(fx + fixture::YEAR) as u32);
            let fy = read_u16(f + 2);
            if yr > fy || (yr == fy && day > read_i16(f)) {
                fx = exe::llist_get_next(list);
            }
        }
        if read_only == 0 {
            let (mut tournament, mut friendly_match, mut tour) = (0usize, 0usize, 0usize);
            let status = date_status(friendly, club, d1p, &mut tournament as *mut usize as usize,
                &mut friendly_match as *mut usize as usize, &mut tour as *mut usize as usize, 0, 0, 4, 1);
            let mut drawn = true;
            match status {
                5 => {
                    covered!("tel02_1 fixtures friendly row");
                    fixtures_date_cell(d1p, lst, row);
                    let fm_ = friendly_match;
                    let mut opponent = read_usize(fm_ + friendly_match::HOME_CLUB);
                    if club == opponent {
                        opponent = read_usize(fm_ + friendly_match::AWAY_CLUB);
                    }
                    format(FIXTURES_TEXT, 0x98f898, opponent + club::SHORT_NAME); // "  %s ?"
                    let tc = if exe::human_manager_is_human_club(opponent, 0, 1, 0, 0) != 0 { c(G_GUI_COLOUR_YELLOW) } else { c(G_GUI_COLOUR_PALE_GREY) };
                    button(sm, lst, 1, row, 1, 0, 1, 3, tc, FIXTURES_TEXT, 0x7d5, opponent);
                    if !national(opponent) && read_usize(opponent + club::NATION) != read_usize(club + club::NATION) {
                        let team = exe::database_get_national_team(read_usize(opponent + club::NATION));
                        button(sm, lst, 2, row, 1, 0, 1, 2, c(G_GUI_COLOUR_YELLOW), read_usize(opponent + club::NATION) + nation::THREE_LETTER_NAME, 0x7d5, team);
                    }
                    let home = club == read_usize(fm_ + friendly_match::HOME_CLUB);
                    text(FIXTURES_TEXT, if home { 0x98f880 } else { 0x98f868 }, FIXTURES_EMPTY_ARG); // "H<..>" / "A<..>"
                    label(sm, lst, 3, row, 1, 0, 0xc, 2, c(FIXTURES_COLOUR_VENUE), FIXTURES_TEXT);
                    text(FIXTURES_TEXT, 0x98f85c); // "  Friendly"
                    label(sm, lst, 4, row, 1, 0, 1, 2, c(FIXTURES_COLOUR_COMP), FIXTURES_TEXT);
                    fixtures_no_result_cell(lst, row);
                    if cancelling != 0 && read_u32(fm_ + FIXTURES_FRIENDLY_LOCKED) == 0 {
                        item(current_human_display(), 2, 0x6e, 0, 0x2f6, 0, 0, row, 1, 0, 0, 0xc, 1, c(G_GUI_COLOUR_PALE_GREY), 0, 0, 0x16, fm_, lst);
                    }
                }
                6 => {
                    covered!("tel02_1 fixtures tour row");
                    fixtures_date_cell(d1p, lst, row);
                    text(FIXTURES_TEXT, 0x98f850); // "  Tba"
                    label(sm, lst, 1, row, 1, 0, 1, 3, c(G_GUI_COLOUR_PALE_GREY), FIXTURES_TEXT);
                    let n = read_usize(tour + FIXTURES_TOUR_NATION);
                    let team = exe::database_get_national_team(n);
                    button(sm, lst, 2, row, 1, 0, 1, 2, c(G_GUI_COLOUR_YELLOW), read_usize(tour + FIXTURES_TOUR_NATION) + nation::THREE_LETTER_NAME, 0x7d5, team);
                    text(FIXTURES_TEXT, 0x98f868, FIXTURES_EMPTY_ARG); // "A<%s - COMMENT - away>"
                    label(sm, lst, 3, row, 1, 0, 0xc, 2, c(FIXTURES_COLOUR_VENUE), FIXTURES_TEXT);
                    let n = read_usize(tour + FIXTURES_TOUR_NATION);
                    let gender = read_i8(n + nation::SHORT_GENDER_NAME) as i32;
                    text(FIXTURES_TEXT, 0x98f834, gender, gender, n + nation::SHORT_NAME); // "  Tour Of {}<%s - nation>{}"
                    label(sm, lst, 4, row, 1, 0, 1, 2, c(FIXTURES_COLOUR_COMP), FIXTURES_TEXT);
                    fixtures_no_result_cell(lst, row);
                    if exe::human_manager_current_controls_club(club, 0, 1) != 0
                        && club == read_usize(tour + FIXTURES_TOUR_CLUB)
                        && (cancelling == 0 || read_u32(tour + FIXTURES_TOUR_LOCKED) == 0)
                    {
                        item(current_human_display(), 2, 0x6e, 0, 0x2f6, 0, 0, row, 1, 0, 0, 0xc, 1, c(G_GUI_COLOUR_PALE_GREY), 0, 0, 0x17, tour, lst);
                    }
                }
                3 => {
                    covered!("tel02_1 fixtures tournament row");
                    fixtures_date_cell(d1p, lst, row);
                    text(FIXTURES_TEXT, 0x98f850); // "  Tba"
                    label(sm, lst, 1, row, 1, 0, 1, 3, c(G_GUI_COLOUR_PALE_GREY), FIXTURES_TEXT);
                    let host = club == read_usize(tournament + tournament_slot::HOST);
                    text(FIXTURES_TEXT, if host { 0x98f880 } else { 0x98f868 }, FIXTURES_EMPTY_ARG); // "H<..>" / "A<..>"
                    label(sm, lst, 3, row, 1, 0, 0xc, 2, c(FIXTURES_COLOUR_VENUE), FIXTURES_TEXT);
                    format(FIXTURES_TEXT, 0x987ff0, tournament + tournament_slot::NAME); // "  %s"
                    let comp = read_usize(tournament + tournament_slot::COMPETITION);
                    if comp != 0 {
                        button(sm, lst, 4, row, 1, 0, 1, 2, c(FIXTURES_COLOUR_COMP), FIXTURES_TEXT, 0x7d0, read_usize(comp + FIXTURES_COMP_ID));
                    } else {
                        label(sm, lst, 4, row, 1, 0, 1, 2, c(FIXTURES_COLOUR_COMP), FIXTURES_TEXT);
                    }
                    fixtures_no_result_cell(lst, row);
                    if read_usize(tournament + tournament_slot::COMPETITION) == 0
                        && (cancelling == 0 || read_u32(tournament + tournament_slot::IN_USE) == 0)
                    {
                        item(current_human_display(), 2, 0x6e, 0, 0x2f6, 0, 0, row, 1, 0, 0, 0xc, 1, c(G_GUI_COLOUR_PALE_GREY), 0, 0, 0x18, tournament, lst);
                    }
                }
                _ => drawn = false,
            }
            if drawn {
                row += 1;
            }
        }
        fixtures_next_day(d1p, outp, tmpp);
    }
    // the scroll and the screen params
    let display = current_human_display();
    let lst16 = lst as i16;
    let area_of = |k: i16| (display as isize + k as isize * 0xbf1 + 4) as usize;
    if scroll_to_next != 0 {
        let r = next_row - 7;
        let r = if r <= 0 { 0 } else { r };
        if lst16 != -1 {
            exe::area_set_scroll_row(area_of(lst16), r as usize);
        }
        exe::screenmanager_set_screen_param(sm, 0x11, lst16 as isize as usize, 0);
        exe::screenmanager_set_screen_param(sm, 0xf, 0usize, 0);
    } else {
        let r = exe::screenmanager_get_screen_param(sm, 0x10);
        if lst16 != -1 {
            exe::area_set_scroll_row(area_of(lst16), r);
        }
        exe::screenmanager_set_screen_param(sm, 0x11, lst16 as isize as usize, 0);
    }
    if cancelling != 0 {
        // the 1920x1080 exe patches these two immediates (push 0x257 at 0x46041b, push 0x31f at 0x460420)
        let bottom_y = core::ptr::read_unaligned(0x46041c as *const u32) as usize;
        let right_x = core::ptr::read_unaligned(0x460421 as *const u32) as usize;
        button_item(sm, 0, 0, right_x, bottom_y, 1, 0, 0xc, 1, c(G_GUI_COLOUR_PALE_GREY), 0, 0x19, 0);
    }
}

/// D1 = D1 + 1 day (`Date::add_days` into a temporary, then `Date::operator=`).
unsafe fn fixtures_next_day(d1: usize, out: usize, tmp: usize) {
    let t = exe::date_add_days(d1, tmp, 1);
    exe::date_operator_3d(d1, out, t);
}

/// Column 0 of a friendly/tour/tournament row: the long date (`Date::get_long_date_string`, 0x7d0, 0xff), dark blue
/// background.
unsafe fn fixtures_date_cell(d1: usize, lst: usize, row: usize) {
    let label: ListCellTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
    exe::date_get_long_date_string(d1, FIXTURES_TEXT, 0x7d0, 0xff);
    label(SCREEN_MANAGER, lst, 0, row, 0x30, colour_word(G_GUI_COLOUR_DARK_BLUE), 0xc, 1, colour_word(G_GUI_COLOUR_PALE_GREY), FIXTURES_TEXT);
}

/// Column 5 of a friendly/tour/tournament row: "---".
unsafe fn fixtures_no_result_cell(lst: usize, row: usize) {
    let label: ListCellTextFn = core::mem::transmute(FN_SCREENMANAGER_ADD_AREA_TEXT);
    fixtures_copy_text(FIXTURES_TEXT, FIXTURES_NO_RESULT_TEXT);
    label(SCREEN_MANAGER, lst, 5, row, 0x30, colour_word(FIXTURES_COLOUR_RESULT_BG), 0xc, 1, colour_word(G_GUI_COLOUR_PALE_GREY), FIXTURES_TEXT);
}

/// Comparer of `club_screen_draw_fixtures_tab`: side by side over the display list (and the screen params it sets,
/// 0xf and 0x11), with the fixture list's 0xe bytes (its cursor moves). With no list yet (param 0xd = 0) only the
/// original runs: it allocates the list and stores it in param 0xd.
pub(crate) unsafe extern "C" fn compare_club_screen_draw_fixtures_tab(top: i32, bottom: i32) {
    let o = original_fn::<unsafe extern "C" fn(i32, i32)>(%%I1%%);
    let list = exe::screenmanager_get_screen_param(SCREEN_MANAGER, 0xd);
    if list == 0 {
        // simplification: allocating the list (operator new, param 0xd) does not run twice
        return o(top, bottom);
    }
    let extra = [(list, 0xe)];
    compare_by_display_list_with("Club screen fixtures tab", &extra, || o(top, bottom), || club_screen_draw_fixtures_tab(top, bottom));
}


/// The Spanish clubs' badge pictures: 8-byte entries (the club's English name, the picture file under `CM3_PICTURES`),
/// ended by an entry with an empty name.
const BADGE_PICTURE_TABLE: usize = 0x98dedc;
const BADGE_PICTURES_KEY: usize = 0x9899e0; // "CM3_PICTURES"
const BADGE_OPEN_MODE: usize = 0x98f928; // "rb"

/// The inlined strcmp of the original, as equality (byte by byte up to the first difference or the end).
unsafe fn badge_names_equal(a: usize, b: usize) -> bool {
    let mut k = 0;
    loop {
        let c = read_u8(a + k);
        if c != read_u8(b + k) {
            return false;
        }
        if c == 0 {
            return true;
        }
        k += 1;
    }
}

/// Finds the badge picture of a Spanish club and copies its path into `out`.
///
/// Only a club whose nation (`club::NATION`) is Spain (the id in `KEY_NATION_SPAIN`). The name looked up is the
/// English name (`exe::club_get_english_name`) of the club itself when `exe::club_get_linked_b_team` (asked with the
/// club in its out slot and 1) leaves that slot nonzero, otherwise of the club it returns. The table
/// `BADGE_PICTURE_TABLE` is walked in order until the first entry with an empty name; the first entry whose name is
/// equal (strcmp) and whose file builds a path (`exe::utils_build_file_path` with `CM3_PICTURES`) is the one: if that
/// file opens for reading ("rb", closed at once) its path is copied into `out` and the answer is 1. Otherwise 0 (a
/// path that does not open ends the search, it does not go on to the next entries).
/// Original: `find_club_badge_picture_path` (0x460450, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn find_club_badge_picture_path(club: usize, out: usize) -> i32 {
    if club == 0 {
        return 0;
    }
    let nation = read_usize(club + club::NATION);
    if nation == 0 || read_u32(nation) != read_u32(KEY_NATION_SPAIN) {
        return 0;
    }
    covered!("tel02_1 badge picture of a Spanish club");
    let mut slot = club as u32;
    let linked = exe::club_get_linked_b_team(club, &mut slot, 1);
    let name = exe::club_get_english_name(if slot != 0 { club } else { linked });
    if name == 0 {
        return 0;
    }
    let mut k: i16 = 0;
    loop {
        let entry = (BADGE_PICTURE_TABLE as isize + k as isize * 8) as usize;
        let entry_name = read_usize(entry);
        if read_u8(entry_name) == 0 {
            return 0;
        }
        if badge_names_equal(entry_name, name) {
            let path = exe::utils_build_file_path(BADGE_PICTURES_KEY, read_usize(entry + 4));
            if path != 0 {
                let f = exe::crt_fopen(path, BADGE_OPEN_MODE);
                if f == 0 {
                    return 0;
                }
                copy_c_string(out, path);
                exe::crt_fclose(f);
                return 1;
            }
        }
        k = k.wrapping_add(1);
    }
}

/// Comparer of `find_club_badge_picture_path`: side by side on the answer and the path written to `out` (the file is
/// only opened for reading).
pub(crate) unsafe extern "C" fn compare_find_club_badge_picture_path(club: usize, out: usize) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(%%I2%%);
    if out == 0 {
        return o(club, out);
    }
    let (r, n, equal, _, _) = crate::comparison::side_by_side(&[(out, 0x104)], || o(club, out), || find_club_badge_picture_path(club, out));
    SCOREBOARD.record(equal && r == n, || format!("find_club_badge_picture_path(club={:#x}) original={} rust={}", club, r, n));
    r
}
