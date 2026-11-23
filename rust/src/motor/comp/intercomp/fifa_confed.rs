use super::*;

/// rule: the Confederations Cup has two groups (stages −1 and 0, ids 0x3e8 and 0x3e9) of three rounds on 29 July, 1 and
/// 4 August, with fixed matches; the knockout (stage 1) in single matches: semi-final on 6/8, third place on 8/8 and
/// final on 9/8 (venue 4); category 10; tie-break 0x383 in the semi-final and 0x83 after
/// Original: `FifaConfed::create_schedule` (0x58b560, fifa_confed.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn fifa_confed_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    build_phase_table(League(this), confed_cup_phase(stage), rounds, extra, league, 0x9c7474)
}

/// Rule: opening the Football Association meeting screen of a club: parameter 0 = the club's index, 1 and 2 cleared.
///
/// `open_screen` (builder 0x58ad30, handler 0x58b0f0, single instance, leave 0x5a8f60 (the shared empty routine), no
/// exit); when it opens, parameter 0 = the club record's id (+0), parameters 1 and 2 = 0.
/// Original: `fa_meeting_open_screen` (0x58acd0, fifa_confed.cpp, cdecl).
pub(crate) unsafe extern "C" fn fa_meeting_open_screen(club: usize) {
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x58ad30usize, 0x58b0f0usize, 1, 0x5a8f60usize, 0) != 0 {
        covered!("tel34_5 open fa meeting");
        set_screen_param(0, read_usize(club));
        set_screen_param(1, 0);
        set_screen_param(2, 0);
    }
}

/// Rule: the Football Association meeting screen: the club's manager in the title, the explanation, the two requests
/// to choose from (the chosen one highlighted) and Cancel and Confirm.
///
/// A local `Text` (`Text::Text`, 0); the club = parameter 0 in the club table; no manager (`club::MANAGER`): the Text
/// destroyed and nothing else. The chosen request = parameter 1 (byte); parameter 2 read and not used. The club's
/// background (`background_set_club_picture`, 0, 0); the title: the manager's name (500 bytes) in the club's kit
/// colours; the subtitle "Meet with the Football Association"; the rows = 0x195 / font 3's height (signed, a byte);
/// a 7-column area (weights 2,20,2,10,2,4,3) from (0x6e,0x8c) to (0x30c,0x221) where the explanation ("Specify the
/// details of requests from the Football Association:", translated) is wrapped (`Text::set_string`,
/// `Text::draw_wrapped`, font 3, from line 0); then a 2-column area (weights 2,17) over the same rectangle and, from the
/// line after the explanation + 2, the requests 1 and 2 (`get_fa_request_option_text` 0x6a9e20, 2000 bytes) as buttons
/// in column 1 (id 100, value the request), in the highlight colour [0xaebdec] when chosen, otherwise [0xae317e];
/// the first button's item goes to parameter 2. The menu bar (4, 0); a 2-column area (weights 3/1) in the kit's second
/// colour with "Cancel" (id 4) and "Confirm" (id 5); the Text destroyed.
/// Original: `fa_meeting_screen_draw` (0x58ad30, fifa_confed.cpp, cdecl).
pub(crate) unsafe extern "C" fn fa_meeting_screen_draw() {
    type DrawWrapped = unsafe extern "thiscall" fn(*mut usize, u32, u32, u32, *mut i32, u32, u32, u32, u32) -> u32;
    type RequestText = unsafe extern "thiscall" fn(usize, u32, usize, u32);
    let weights_explanation: [u8; 7] = [2, 0x14, 2, 0xa, 2, 4, 3];
    let weights_requests: [u8; 2] = [2, 0x11];
    let weights_buttons: [u8; 2] = [3, 1];
    let mut text = [0usize; 8];
    let mut name = [0u8; 500];
    // the wrapped text's line counter (zeroed again by the original just before the wrap)
    let mut line: i32 = 0;
    exe::new_text(text.as_mut_ptr(), 0usize);
    let club = club_at(get_screen_param(0) as i32 as isize);
    let manager = club.manager().addr();
    if manager == 0 {
        exe::drop_text(text.as_mut_ptr());
        return;
    }
    covered!("tel34_5 fa meeting");
    let chosen = get_screen_param(1) as u8;
    get_screen_param(2);
    exe::background_set_club_picture(club, 0, 0);
    let (mut fore, mut back) = (0u32, 0u32);
    exe::database_get_club_kit_colours(club, 1, &mut fore, &mut back);
    exe::database_format_staff_name(manager, name.as_mut_ptr() as usize, 0x1f4, 1);
    exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, back, fore, name.as_ptr() as usize);
    tel34_lang_text(0x9c7450); // "Meet with the Football Association"
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, JOB_TEXT, 1);
    let rows = 0x195i32.wrapping_div(exe::font_get_height(3)) as u8 as u32;
    let explanation = tel26_add_area(current_human_display(), [0x6e, 0x8c, 0x30c, 0x221, 7, weights_explanation.as_ptr() as u32, rows, 0, 2, 0, u32::MAX]);
    tel34_lang_text(0x9c740c); // "  Specify the details of requests from the Football Association:"
    exe::text_set_string(text.as_mut_ptr() as usize, JOB_TEXT as *const u8);
    let wrap: DrawWrapped = core::mem::transmute(FN_TEXT_DRAW_WRAPPED);
    wrap(text.as_mut_ptr(), explanation, 0x6e, 0x30c, &mut line, 3, job_colour(JOB_COLOUR_3184), 0, 0);
    let requests = tel26_add_area(current_human_display(), [0x6e, 0x8c, 0x30c, 0x221, 2, weights_requests.as_ptr() as u32, rows, 0, 1, 0, u32::MAX]);
    line = line.wrapping_add(2);
    let request_text: RequestText = core::mem::transmute(FN_GET_FA_REQUEST_OPTION_TEXT);
    for request in 1u8..=2 {
        request_text(read_usize(0xb67a44), request as u32, JOB_TEXT, 0x7d0);
        let colour = if chosen == request { job_colour(JOB_COLOUR_BDEC) } else { job_colour(JOB_COLOUR_317E) };
        let item = tel26_area_button([requests, 1, line as u32, 1, 0, 1, 3, colour, JOB_TEXT as u32, 0x64, request as i8 as i32 as u32]);
        if request == 1 {
            set_screen_param(2, item as u16 as i16 as isize as usize);
        }
        line = line.wrapping_add(1);
    }
    exe::menubar_build(4, 0);
    let buttons = tel26_add_area(current_human_display(), [0x64, 0x22b, 0x316, 0x24e, 2, weights_buttons.as_ptr() as u32, 1, 0, 1, back, u32::MAX]);
    tel34_lang_text(0x98a1a0); // "Cancel"
    tel26_area_button([buttons, 0, 0, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 3, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 4, 0]);
    tel34_lang_text(0x98a198); // "Confirm"
    tel26_area_button([buttons, 1, 0, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 3, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 5, 0]);
    exe::drop_text(text.as_mut_ptr());
}

/// Rule: on the FA meeting screen a request button picks that request (the two buttons recoloured, the screen
/// redrawn); Confirm sends the chosen request to the FA and closes; Cancel closes; other widgets go to the menu bar.
///
/// The club = parameter 0 in the club table, the chosen request = parameter 1 (parameter 2 read and not used). With
/// the dialog flag (0xb77c84) set: the pending command argument cleared and -9. Id 100: the button's value (0 for -1),
/// parameter 2 = the first button's item; value 1: the first button in the highlight colour [0xaebdec] and the second in
/// [0xae317e]; value 2: the other way round (`ScreenManager::set_widget_text_colour`); any value: parameter 1 = it,
/// `ScreenManager::redraw_screen` (0, 0), 0. Id 5: with a chosen request, `MANAGER_MANAGER::handle_fa_request`
/// (0x6a9e60, club, request); -9. Id 4: -9.
/// Original: `fa_meeting_screen_handle_event` (0x58b0f0, fifa_confed.cpp, cdecl).
pub(crate) unsafe extern "C" fn fa_meeting_screen_handle_event(raw: u32) -> i32 {
    let club = club_at(get_screen_param(0) as i32 as isize);
    let chosen = get_screen_param(1);
    get_screen_param(2);
    if read_u32(TEL34_SCREEN_DIALOG_FLAG) != 0 {
        write_u32(G_PENDING_COMMAND_ARG, 0);
        return -9;
    }
    let item = raw as i16;
    match tel26_item_id(item) {
        100 => {
            covered!("tel34_5 fa meeting pick");
            let value = tel26_item_value(item);
            let first = get_screen_param(2) as u32;
            let colour = |widget: u32, c: usize| exe::screenmanager_set_widget_text_colour(G_SCREEN_MANAGER, widget, job_colour(c));
            match value {
                1 => {
                    colour(first, JOB_COLOUR_BDEC);
                    colour(first.wrapping_add(1), JOB_COLOUR_317E);
                }
                2 => {
                    colour(first, JOB_COLOUR_317E);
                    colour(first.wrapping_add(1), JOB_COLOUR_BDEC);
                }
                _ => {}
            }
            set_screen_param(1, value as usize);
            exe::screenmanager_redraw_screen(G_SCREEN_MANAGER, 0, 0);
            0
        }
        5 => {
            if chosen != 0 {
                covered!("tel34_5 fa meeting confirm");
                exe::manager_manager_handle_fa_request(read_usize(0xb67a44), club, chosen as u32);
            }
            -9
        }
        4 => -9,
        _ => {
            let r = exe::menubar_handle_command(raw);
            if r != 0 { r } else { exe::menubar_handle_object_link(raw) }
        }
    }
}

/// Comparer for `fa_meeting_open_screen`.
pub(crate) unsafe extern "C" fn compare_fa_meeting_open_screen(club: usize) {
    // simplification: opening a screen does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(usize)>(IDX_FA_MEETING_OPEN_SCREEN)(club)
}

/// Comparer for `fa_meeting_screen_draw`: the FA meeting screen side by side over the display list.
pub(crate) unsafe extern "C" fn compare_fa_meeting_screen_draw() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_FA_MEETING_SCREEN_DRAW);
    compare_by_display_list_with("FA meeting screen (fa_meeting_screen_draw)", &[], || o(), || fa_meeting_screen_draw());
}

/// Comparer for `fa_meeting_screen_handle_event`.
pub(crate) unsafe extern "C" fn compare_fa_meeting_screen_handle_event(raw: u32) -> i32 {
    // simplification: the redraw (network), the FA request and the menu bar commands do not run twice; proven in Rust
    // mode
    original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_FA_MEETING_SCREEN_HANDLE_EVENT)(raw)
}

/// rule: the Confederations Cup groups are single round robin, 3/1 points, tiebreak codes 4, 1 and 2, two qualifiers per
/// group, national teams, category 10, 7 substitutes and 3 changes; the draw date is 20 February of the cup year
/// (weekday 5).
///
/// Writes `FIFA_CONFED_RULE_FIELDS` (the win at `+0xc2`; `.league_flags()` = 0x28), then, unlike its sisters, the
/// stage −1 round table first (virtual +0x3c: push 0; push &+0x3a; push &+0xa9; push −1 -> thiscall, ret 0x10) into
/// `.round_dates()`, and after it Date::Date(0x14, 1, year, 5) (0x549f70) into `.next_draw_day()` with
/// `.next_draw_year_offset()` 0. Returns 1. No SEH frame.
/// Original: `FifaConfed::set_up_rules` (0x58bdb0, fifa_confed.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn fifa_confed_set_up_rules(this: usize) -> u32 {
    let this = League(this);
    let league = this;
    covered!("cqz_2 confederations cup rules");
    let win = 3; // cqz2: the win in the Confederations Cup groups
    goldcup_write_rule_fields(this.addr(), &FIFA_CONFED_RULE_FIELDS, win);
    let table: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let t = table(this.addr(), 0xffff_ffff, this.addr() + league::ROUND_COUNT, this.addr() + league::STAGE_ID, 0);
    league.set_round_dates(t);
    let day = 0x14; // cqz2: the day of the Confederations Cup draw date
    let word = competition_date_word(league, day, 1, 0, 5);
    league.set_next_draw_year_offset(0);
    league.set_next_draw_day(word as i16);
    1
}

/// Comparer for `fifa_confed_set_up_rules` (original only, then the plan).
pub(crate) unsafe extern "thiscall" fn compare_fifa_confed_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_FIFA_CONFED_SET_UP_RULES);
    check_set_up_rules("fifa_confed_set_up_rules", this, &FIFA_CONFED_RULE_FIELDS, (0x14, 1, 0, 5), 0, || o(this))
}

/// The Confederations Cup at career creation.
///
/// League::League (`exe::new_league`, mov ecx -> thiscall, ret 0); vtable 0x96a270, the record (`league::COMPETITION`),
/// `club_sides` 4, no upper or lower division (−1), the edition (`confed_cup_edition` of the game's year), discipline
/// rule 0 and importance 0xf. If the saved state loads (`exe::league_load_from_temp_file(this, 1)`: push 1; mov ecx ->
/// thiscall, ret 4; nonzero), returns this.
///
/// Otherwise: inactive, the main stage (0xff), `current_stage` −1, 2 stages and the stage vector (`exe::crt_malloc(8)`:
/// push; add esp -> cdecl, unchecked); then, unlike its sisters, the statistics before the rules: operator new
/// (`exe::crt_operator_new(0x5ce)`) and Comp_Stats::Comp_Stats (`exe::new_comp_stats(new, this)`: push this; mov ecx,
/// new -> thiscall, ret 4) into `comp_stats`; null: the error box of line 0x53 of fifa_confed.cpp and no rules;
/// otherwise the rules FifaConfed::set_up_rules (0x58bdb0, thiscall, ret 0; its return value is not read). The rules set
/// `league_flags` (0x28). Returns this.
/// Original: `FifaConfed::FifaConfed` (0x58b310, fifa_confed.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_fifa_confed(this: usize, year: u32, record: usize) -> usize {
    let this = League(this);
    let league = this;
    exe::new_league(league);
    write_u32(this.addr(), 0x96a270);
    write_u32(this.addr() + league::COMPETITION, record as u32);
    league.set_club_sides(4);
    this.set_upper_division_id(-1);
    this.set_lower_division_id((0xffff_ffff) as usize);
    let edition = confed_cup_edition(year as i16); // cqz3: the Confederations Cup in an odd year after 2001
    league.set_year(edition);
    league.set_discipline_rule(0);
    league.set_importance(0xf);
    if exe::league_load_from_temp_file(league, 1) != 0 {
        return this.addr();
    }
    covered!("cqz_3 confederations cup");
    this.set_active((0) as usize);
    league.set_stage_index(stage_index::MAIN);
    this.set_current_stage((0xffff_ffff) as usize);
    this.set_stage_count((2) as usize);
    this.set_stages((exe::crt_malloc(8) as u32) as usize);
    let p = exe::crt_operator_new(0x5ce);
    let e = if p != 0 { exe::new_comp_stats(p, league) } else { 0 };
    this.set_comp_stats((e as u32) as usize);
    if e == 0 {
        show_error_at(0x9c7474, 0x53);
    } else {
        let set_up_rules: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_FIFACONFED_SET_UP_RULES);
        set_up_rules(this.addr());
    }
    this.addr()
}

/// Comparer for `construct_fifa_confed` (original only, then the plan).
pub(crate) unsafe extern "thiscall" fn compare_construct_fifa_confed(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_FIFA_CONFED);
    compare_nations_tournament_ctor("construct_fifa_confed", League(this), record, 0x96a270, confed_cup_edition(year as i16), || o(this, year, record))
}

/// rule: on the draw day of the Confederations Cup (20 February of the cup year: day 0x14 of month 1) the nations are selected and drawn
/// into two groups, the first gets its table and fixtures and the second its stage (`nations_tournament_day`)
/// Original: `FifaConfed::update_for_date` (0x58c4b0, fifa_confed.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn fifa_confed_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    covered!("cqz_5 confederations cup day");
    nations_tournament_day(League(this), today, full, &FIFA_CONFED_DAY)
}

/// Comparer for `fifa_confed_update_for_date` (original only, then the plan on the draw day).
pub(crate) unsafe extern "thiscall" fn compare_fifa_confed_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_FIFA_CONFED_UPDATE_FOR_DATE);
    compare_nations_tournament_day("fifa_confed_update_for_date", League(this), today, full, &FIFA_CONFED_DAY, || o(this, today, full))
}

/// The Confederations Cup edition: from `start`, one year more while the year is even or is 2001 (an odd year other
/// than 2001: in a 2001 game, 2003).
pub(crate) fn confed_cup_edition(start: i16) -> i16 {
    let mut year = start;
    while (year as i32) % 2 == 0 || year == 0x7d1 {
        year = year.wrapping_add(1);
    }
    year
}

/// The screen manager's flag at +0x3834 (0xb77c84) that the dialog handlers check first: set, the handler clears the
/// pending command argument [`G_PENDING_COMMAND_ARG`] and closes (a network game's answer) [inference].
pub(crate) const TEL34_SCREEN_DIALOG_FLAG: usize = 0xb77c84;

/// The rule fields of the Confederations Cup (offset, width, value), in the original's write order: like the European
/// Championship without the stage id, and `.league_flags()` 0x28.
pub(crate) const FIFA_CONFED_RULE_FIELDS: [(usize, u8, u32); 15] = [(0xc2, 1, 3), (0x4a, 1, 3), (0x43, 1, 2), (0xc7, 1, 2), (0xbe, 1, 2), (0x3c, 2, 1), (0xc3, 1, 1),
    (0xc4, 1, 0xa), (0xc5, 1, 4), (0xc6, 1, 1), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 0), (0x49, 1, 7), (0xd9, 2, 0x28)];

/// The Confederations Cup day: like the Olympic one (the date test even without `full`, the draw only without it); the
/// selection of the nations (FifaConfed::select_teams 0x58c6b0), the first group's table (FifaConfed::create_group_table
/// 0x58bcb0) and the second group's stage (FifaConfed::create_group_stage 0x58c100, no argument); the boxes of lines
/// 0x32b, 0x332 and 0x339 of fifa_confed.cpp; the count +0xf2 zeroed as a dword.
pub(crate) const FIFA_CONFED_DAY: NationsDayPlan = NationsDayPlan { light_day_first: false, draw_needs_light_day: true, draw: 0x58c6b0, group_table: 0x58bcb0,
    file: 0x9c7474, rows: [0x32b, 0x332, 0x339], group_stage: 0x58c100, group_stages: None, count_as_dword: true };
