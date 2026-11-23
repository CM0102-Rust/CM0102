use super::*;

/// rule: the FA Cup has 9 single-leg rounds with replays (+0x20 = 1, 10 days later) up to the quarter-finals:
/// preliminary (code 0xdc) drawn 9/10, played 16/10, 56 teams; 1st round 17/10 and 18/11; 2nd 19/11 and 9/12; 3rd 10/12
/// and 6/1; 4th 7/1 and 27/1; 5th 28/1 and 17/2; quarter-finals 18/2 and 10/3; semi-final 11/3 and 8/4 at its own
/// ground (venue 0x2a, no replay); final 9/4 and 12/5 at a neutral ground; the team counts from the 2nd to the 5th
/// round depend on the exemptions (`eng_fa_cup::EXEMPTION_COUNT`) and on the choice of entering at the 5th round
/// (`eng_fa_cup::FIFTH_ROUND_ENTRY` = 1); prize for winning a round from 10 thousand to 400 thousand, and the final 2
/// million to the winner and 1 million to the runner-up; category 4 up to the 5th round and 6 from the quarter-finals
/// Original: `EngFaCup::create_round_table` (0x56f320, eng_fa_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn eng_fa_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let this = Cup(this);
    let t = fa_cup_rounds(read_i8(this.addr() + eng_fa_cup::EXEMPTION_COUNT) as i16, read_u8(this.addr() + eng_fa_cup::FIFTH_ROUND_ENTRY), this.team_count());
    eng_cc_cup_create_single_stage_round_table(this, stage, rounds, extra, out, &t, 0x9c42cc, 0xb6, "cql_10 tabela da copa da inglaterra")
}

/// Daily update of the FA Cup. Rule: the FA Cup gathers its entrants on 3 October each year (Date(3, 9): month 9 is
/// October; the community map says September), rebuilds the rounds and the tree and draws the preliminary round, once
/// per season (`.season_set_up()`); on 1 October 1999 it asked the human manager whether the club enters at the 5th
/// round (`eng_fa_cup::FIFTH_ROUND_QUESTION_ASKED` marks the question); then, every day, the cup update.
///
/// Ret 8; virtual +0x14 of the English cup; SEH frame for the news of the 1999 path, no destructor on the normal path:
/// `today.update_for_date(stage)`. With `.season_set_up()` = 0 and the stage different from 0: on Date(1, 9, 1999)
/// (month 9 is October) with `eng_fa_cup::FIFTH_ROUND_QUESTION_ASKED` = 0, the question to the human manager about
/// entering at the 5th round (only in 1999: the original routine through the trampoline); on Date(3, 9, today's year)
/// (3 October), frees the tables, `.set_season_set_up(1)`, the entrants (`exe::engfacup_collect_entrants`, thiscall),
/// the rounds (virtual `comp_vtable::SET_UP_RULES`), the tree (virtual `cup_vtable::MAP_FIXTURE_TREE`) (a negative
/// return gives the box, lines 0x3d6, 0x3dd, 0x3e4, and 1), the predicted order (virtual
/// `comp_vtable::SET_PREDICTED_POSITIONS`) and the draw of the 1st round (`exe::cup_set_up_round`: push 0; mov ecx, cup
/// -> thiscall, ret 4). Always, at the end, the cup update.
/// Original: `EngFaCup::update_for_date` (0x570550, eng_fa_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn eng_fa_cup_update_for_date(this: usize, today: usize, stage: u32) -> u32 {
    let this = Cup(this);
    if this.season_set_up() == 0 && stage != 0 {
        if is_date_today(make_date_dword(1, 9, 0x7cf, 0xffff_ffff), today) && *((this.addr() + eng_fa_cup::FIFTH_ROUND_QUESTION_ASKED) as *const u8) == 0 {
            // simplification: the 1999 question builds and delivers a news item (only on that day, outside any new career)
            return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_ENG_FA_CUP_UPDATE_FOR_DATE)(this.addr(), today, stage);
        }
        // simplification: the year goes in ax with the rest of eax in the high bytes; 0x549f70 reads it as a word
        if is_date_today(make_date_dword(3, 9, read_i16(today + 2) as u16 as u32, 0xffff_ffff), today) { // cqn10: the FA Cup entries on 3/10
            free_season_tables(this);
            covered!("cqn_10 fa cup entries");
            this.set_season_set_up((1) as usize);
            if exe::engfacup_collect_entrants(this) < 0 {
                show_error_at(0x9c42cc, 0x3d6);
                return 1;
            }
            if call_cup_virtual(this, comp_vtable::SET_UP_RULES) < 0 {
                show_error_at(0x9c42cc, 0x3dd);
                return 1;
            }
            if call_cup_virtual(this, cup_vtable::MAP_FIXTURE_TREE) < 0 {
                show_error_at(0x9c42cc, 0x3e4);
                return 1;
            }
            call_cup_virtual(this, comp_vtable::SET_PREDICTED_POSITIONS);
            exe::cup_set_up_round(this, 0);
        }
    }
    eng_cc_cup_cup_update_for_date(this, today, stage)
}

/// Rule: the news item asking whether the club enters the FA Cup at the fifth round opens the confirm screen and keeps
/// the news item and its slot.
///
/// `open_screen` (builder 0x570940, handler 0x570ae0, single instance, no leave/exit); when it opens, parameter 0 =
/// the first argument (the manager the news belongs to) and parameter 1 = the second (the news id).
/// Original: `fa_cup_open_fifth_round_entry_screen` (0x5708f0, eng_fa_cup.cpp, cdecl).
pub(crate) unsafe extern "C" fn fa_cup_open_fifth_round_entry_screen(manager: usize, news_id: usize) {
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x570940usize, 0x570ae0usize, 1, 0usize, 0) != 0 {
        covered!("tel34_4 open fa cup fifth round");
        set_screen_param(0, manager);
        set_screen_param(1, news_id);
    }
}

/// Rule: the "Enter the FA Cup at fifth round ?" confirm screen, with No and Yes.
///
/// The competition = the FA Cup's record (`KEY_CLUB_COMP_ENGLISH_FA_CUP` in `G_CLUB_COMP_TABLE`, 0x6b bytes each):
/// its background (`background_set_comp_picture`, 0, 0); the menu bar (4, 0); the title: its name in its colours
/// (`database_get_comp_colours`); the subtitle "Please Confirm"; the question (translated, with the gender char +0x37
/// twice and the name) in a text area (0x6e,0x91..0x30c,0x217) where the name links to the competition (2000 chars);
/// a two-column area (weights 3/1) with "No" (id 2) on the left and "Yes" (id 1) on the right.
/// Original: `fa_cup_draw_fifth_round_entry_screen` (0x570940, eng_fa_cup.cpp, cdecl).
pub(crate) unsafe extern "C" fn fa_cup_draw_fifth_round_entry_screen() {
    let lang: Tel34FormatFn = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let weights: [u8; 2] = [3, 1];
    covered!("tel34_4 fa cup fifth round");
    let comp = read_usize(G_CLUB_COMP_TABLE).wrapping_add((read_i32(KEY_CLUB_COMP_ENGLISH_FA_CUP) as usize).wrapping_mul(CLUB_COMP_RECORD_SIZE));
    exe::background_set_comp_picture(comp, 0, 0);
    exe::menubar_build(4, 0);
    let (mut first, mut second) = (0u32, 0u32);
    exe::database_get_comp_colours(comp, &mut first, &mut second);
    let name = comp + 4;
    exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, second, first, name);
    tel34_lang_text(0x9909b8); // "Please Confirm"
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, JOB_TEXT, 1);
    let gender = read_u8(comp + competition::GENDER_NAME) as i8 as i32;
    lang(JOB_TEXT, 0x9c4304, gender, gender, name); // "Enter the {}<%s - competition>{} at fifth round ?"
    tel34_text_area_with_link([0x6e, 0x91, 0x30c, 0x217, 2, 0, 0x10, 4, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32], name, 0x7d0, comp);
    let area = tel26_add_area(current_human_display(), [0x64, 0x22b, 0x316, 0x24e, 2, weights.as_ptr() as u32, 1, 0, 1, 0, u32::MAX]);
    tel34_lang_text(0x98e8cc); // "No"
    tel26_area_button([area, 0, 0, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 3, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 2, 0]);
    tel34_lang_text(0x98e8d0); // "Yes"
    tel26_area_button([area, 1, 0, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 3, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 1, 0]);
}

/// Rule: "Yes" (id 1) enters the club at the fifth round (the FA Cup object's `eng_fa_cup::FIFTH_ROUND_QUESTION_ASKED`
/// = 2) and answers the news item with 2; "No" (id 2) answers it with 3; both close (-9); other ids go to the menu bar.
///
/// Parameters 0 (the manager) and 1 (the news id) are read first; the answer = `News::set_item_slot` (manager, news
/// id, slot 0, the value, 0). The FA Cup object = the competition object table [`G_COMP_OBJECT_TABLE`] at
/// `KEY_CLUB_COMP_ENGLISH_FA_CUP`.
/// Original: `fa_cup_handle_fifth_round_entry_screen` (0x570ae0, eng_fa_cup.cpp, cdecl).
pub(crate) unsafe extern "C" fn fa_cup_handle_fifth_round_entry_screen(raw: u32) -> i32 {
    let manager = get_screen_param(0);
    let news_id = get_screen_param(1);
    match tel26_item_id(raw as i16) {
        1 => {
            covered!("tel34_4 fa cup fifth round yes");
            let cup = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add((read_i32(KEY_CLUB_COMP_ENGLISH_FA_CUP) as usize).wrapping_mul(4)));
            write_u8(cup + eng_fa_cup::FIFTH_ROUND_QUESTION_ASKED, 2);
            exe::news_set_item_slot(read_usize(0xb7431c), manager, news_id, 0, 2, 0);
            -9
        }
        2 => {
            exe::news_set_item_slot(read_usize(0xb7431c), manager, news_id, 0, 3, 0);
            -9
        }
        _ => {
            let r = exe::menubar_handle_command(raw);
            if r != 0 { r } else { exe::menubar_handle_object_link(raw) }
        }
    }
}

/// Comparer for `fa_cup_open_fifth_round_entry_screen`.
pub(crate) unsafe extern "C" fn compare_fa_cup_open_fifth_round_entry_screen(manager: usize, news_id: usize) {
    // simplification: opening a screen does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(usize, usize)>(IDX_FA_CUP_OPEN_FIFTH_ROUND_ENTRY_SCREEN)(manager, news_id)
}

/// Comparer for `fa_cup_draw_fifth_round_entry_screen`: the confirm screen side by side over the display list.
pub(crate) unsafe extern "C" fn compare_fa_cup_draw_fifth_round_entry_screen() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_FA_CUP_DRAW_FIFTH_ROUND_ENTRY_SCREEN);
    compare_by_display_list_with("FA Cup fifth round entry screen (fa_cup_draw_fifth_round_entry_screen)", &[], || o(),
        || fa_cup_draw_fifth_round_entry_screen());
}

/// Comparer for `fa_cup_handle_fifth_round_entry_screen`.
pub(crate) unsafe extern "C" fn compare_fa_cup_handle_fifth_round_entry_screen(raw: u32) -> i32 {
    // simplification: the answer to the news item and the menu bar commands do not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_FA_CUP_HANDLE_FIFTH_ROUND_ENTRY_SCREEN)(raw)
}

/// The season turnover of the FA Cup: as the League Cup's, and it also clears the next round to draw and the question
/// about entering at the 5th round (no predicted positions).
///
/// `start_new_cup_season_empty_teams` with `fa_cup` (`cup::NEXT_DRAW_ROUND` and
/// `eng_fa_cup::FIFTH_ROUND_QUESTION_ASKED` zeroed) and the eng_fa_cup.cpp boxes at lines 0x372, 0x379 and 0x380.
/// Original: `EngFaCup::start_new_season` (0x570390, eng_fa_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn eng_fa_cup_start_new_season(this: usize) -> u32 {
    start_new_cup_season_empty_teams(Cup(this), 0x9c42cc, [0x372, 0x379, 0x380], 1, true, false, "mwf_5 fa cup rollover") // mwf5: the FA Cup turnover
}

/// Comparer for `eng_fa_cup_start_new_season` (original only; the year plus one).
pub(crate) unsafe extern "thiscall" fn compare_eng_fa_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ENG_FA_CUP_START_NEW_SEASON);
    compare_cup_new_season("eng_fa_cup_start_new_season", Cup(this), || o(this))
}

/// The FA Cup's 152 entrants: see `fa_cup_collect_entrants`.
/// Original: `EngFaCup::collect_entrants` (0x56fab0, eng_fa_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn eng_fa_cup_collect_entrants(this: usize) -> i32 {
    fa_cup_collect_entrants(this)
}

/// Side by side from the same state (the random generator, `cup::TEAM_COUNT`, `eng_fa_cup::EXEMPTION_COUNT` and
/// `FIFTH_ROUND_ENTRY`, `G_MOUSE_LEFT_BUTTON_DOWN`): the original leaves its table in `cup::TEAM_TABLE`, the copy builds its own
/// (without counting the draws), compared on the filled slots (the club of each one; the table is not zeroed) and
/// freed; the return value too.
pub(crate) unsafe extern "thiscall" fn compare_eng_fa_cup_collect_entrants(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_ENG_FA_CUP_COLLECT_ENTRANTS);
    let before = read_usize(this + cup::TEAM_TABLE);
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.extend([(this + cup::TEAM_COUNT, 2usize), (this + eng_fa_cup::EXEMPTION_COUNT, 2), (G_MOUSE_LEFT_BUTTON_DOWN, 4)]);
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let (a, b, same, _, _) = crate::comparison::side_by_side(&reg, || {
        let r = o(this);
        lo.set(read_usize(this + cup::TEAM_TABLE));
        write_usize(this + cup::TEAM_TABLE, before);
        r
    }, || {
        let r = crate::rng::without_counting(|| fa_cup_collect_entrants(this));
        lr.set(read_usize(this + cup::TEAM_TABLE));
        write_usize(this + cup::TEAM_TABLE, lo.get());
        r
    });
    let (lo, lr) = (lo.get(), lr.get());
    let n = (read_i16(this + cup::TEAM_COUNT).max(0) as usize).min(0x98);
    let table = lo == 0 || lr == 0 || a != 1 || (0..n).all(|j| read_usize(lo + j * 6) == read_usize(lr + j * 6) && read_u16(lo + j * 6 + 4) == read_u16(lr + j * 6 + 4));
    if lr != 0 && lr != lo {
        exe::crt_free(lr);
    }
    SCOREBOARD.record(same && a == b && table, || format!("eng_fa_cup_collect_entrants({:#x}) returned {} x {} memory {} table {}", this, a, b, same, table));
    a
}

/// The constructor of the FA Cup: with a table of 152 entrants (6-byte records, zeroed); type 1 (clubs of one nation),
/// 152 teams, prize fund of 2,000,000, bench and substitutions from the base, discipline rule 9; on a new cup, in
/// order: the rounds (the cup's own virtual `comp_vtable::CREATE_SCHEDULE`, into `cup::ROUND_TABLE`); the tie tree
/// (Cup::map_fixture_tree, `map_fixture_tree`, 0x518790, thiscall, ret 0; the error box of line 0x71 if it returns a
/// negative value); the statistics at Comp +8 (the error box of line 0x79 without them). Over the base Cup::Cup
/// (`exe::new_cup`, thiscall, ret 0) it writes the fixed fields in the original's order (the vtable 0x969b28, the
/// competition record at Comp +4, `cup::YEAR` and those of `FA_CUP_CONSTRUCTOR_FIELDS`); if the saved state loads
/// (Cup::load_from_disk 0x51fc00: push 1; mov ecx, this -> thiscall, ret 4; returns ≠ 0) it returns this. Each step
/// that fails shows its own error box (`show_error_at`) and returns this. Returns this.
/// Original: `EngFaCup::EngFaCup` (0x56f040, eng_fa_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_eng_fa_cup(this: usize, year: u32, record: usize) -> usize {
    construct_eng_fa_cup_body(Cup(this), year, record) // cqu5: the FA Cup
}

/// Comparer for `construct_eng_fa_cup`: only the original runs (it allocates, draws and reads the save file), and the plan
/// is checked (`eng_fa_cup_check_constructor_plan`).
pub(crate) unsafe extern "thiscall" fn compare_construct_eng_fa_cup(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_ENG_FA_CUP)(this, year, record);
    let error = eng_fa_cup_check_constructor_plan(Cup(this), r, year, record, 0x969b28, &FA_CUP_CONSTRUCTOR_FIELDS, true);
    SCOREBOARD.record(error.is_none(), || format!("construct_eng_fa_cup({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
    r
}

/// `gui_add_text_area` (0x5ed070, cdecl) with one link triple: x1, y1, x2, y2, two colours, the height, the font, the
/// text colour, the text, then the link (its text, its length, its object) and the closing 0.
pub(crate) unsafe fn tel34_text_area_with_link(f: [u32; 10], link_text: usize, link_len: u32, object: usize) {
    type TextAreaLink = unsafe extern "C" fn(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize, u32, usize, u32);
    let g: TextAreaLink = core::mem::transmute(FN_GUI_ADD_TEXT_AREA);
    g(f[0], f[1], f[2], f[3], f[4], f[5], f[6], f[7], f[8], f[9], link_text, link_len, object, 0);
}

/// Builds the 152 FA Cup entrants. Called by the cup's daily update (`eng_fa_cup_update_for_date`, 0x570550) every 3
/// October; n = 1 up to 2002-06-01.
///
/// 1) The English clubs (`club::NATION` = `KEY_NATION_ENGLAND`) of the competition `KEY_CLUB_COMP_FIFA_CLUB_WORLD_CHAMPIONSHIP`
/// of the same year (`cup::YEAR`), searched among the [0xb63dfc] competitions of the list [0xb63e00] (the community's
/// `g_league_world_club_comp_count` and `g_league_world_club_comp_list`) by the id of its record (`league::COMPETITION`):
/// its list through the competition's virtual +0x58 (push 0; push &list; mov ecx -> thiscall, ret 8; returns how many;
/// the list is freed with `exe::crt_free`), up to 2 (the Club World Championship: Manchester United's withdrawal in
/// 1999/2000 [inference]). In 1999, with 1 found and `eng_fa_cup::FIFTH_ROUND_QUESTION_ASKED` != 2,
/// `eng_fa_cup::FIFTH_ROUND_ENTRY` = 1 and `eng_fa_cup::EXEMPTION_COUNT` = 0 (the club leaves the cup); otherwise
/// `FIFTH_ROUND_ENTRY` = 0 and `EXEMPTION_COUNT` = how many.
/// 2) `cup::TEAM_COUNT` = 152 and the table (`exe::crt_malloc` of 0x390, not zeroed) in `cup::TEAM_TABLE` (out of memory:
/// the box for line 0x1d2 and -1).
/// 3) With the nation rule (the competition's nation `league_selected` has `LOWER_DIVISIONS_LOADED`) or in 2001 (year
/// 0x7d1): the winner and runner-up of the last season of `KEY_CLUB_COMP_ENGLISH_FA_TROPHY` (`exe::comp_util_get_latest_placed_club`,
/// cdecl, on the record `G_CLUB_COMP_TABLE` + 0x6b x the key's id) get a place; otherwise only the first club of the
/// table, if it is in the Conference (the original tests the same club over and over without advancing its loop).
/// 4) In club table order: the Conference clubs (`KEY_CLUB_COMP_ENGLISH_CONFERENCE`) in one list; the English clubs
/// (`club::NATION` = the record `G_NATION_TABLE` + `nation::SIZE` x the England id) outside the league divisions, up to
/// 300, in another (the non-league clubs).
/// 5) The two prize places leave their own list: from the Conference, with 21 places instead of 22 for each one; from
/// the non-league list, with 37 instead of 38 (when not found, the first of the list is replaced by the last).
/// 6) Two Conference clubs drawn (`exe::utils_random`(n - 2), cdecl, twice) go to the last two positions; the
/// non-league clubs sorted by reputation with the best ones shuffled (`exe::comp_util_sort_and_shuffle_top_clubs`: push
/// places; push n; push list -> cdecl).
/// 7) The table: the non-league places (38), the Conference places (22), the two prize clubs, the Football League's
/// third (`KEY_CLUB_COMP_ENGLISH_THIRD_DIVISION`), second and first divisions (except the clubs of step 1 and, in the
/// third, the prize clubs), each until the table is full; the last 24 (the first division) shuffled by 60 swaps of two
/// drawn positions (`exe::utils_random`(0x18) twice); the Premier (`KEY_CLUB_COMP_ENGLISH_PREMIER_DIVISION`); last,
/// the clubs of step 1. With `FIFTH_ROUND_ENTRY` = 1, `cup::TEAM_COUNT` drops by one. Returns 1 if the table is full
/// (k == `cup::TEAM_COUNT`), otherwise 0.
/// Original: `EngFaCup::collect_entrants` (0x56fab0, eng_fa_cup.cpp, thiscall, ret 0).
pub(crate) unsafe fn fa_cup_collect_entrants(this: usize) -> i32 {
    let this = League(this);
    let cup = Cup(this.addr());
    // 1) the English clubs of the Club World Championship of the same year
    let mut same_year = [0usize; 2];
    let mut nf: i16 = 0;
    let n_comp = read_usize(0xb63dfc) as i32;
    if n_comp > 0 {
        let v = read_usize(0xb63e00);
        let mut i: i16 = 0;
        let mut found = 0usize;
        loop {
            let c = read_usize(v + i as i32 as usize * 4);
            if read_usize(read_usize(c + league::COMPETITION)) == read_usize(KEY_CLUB_COMP_FIFA_CLUB_WORLD_CHAMPIONSHIP) {
                found = c;
                break;
            }
            i = i.wrapping_add(1);
            if i as i32 >= n_comp {
                break;
            }
        }
        if found != 0 && read_i16(found + cup::YEAR) == cup.year() {
            // the competition's virtual +0x58 (no name yet): fills a list of its clubs and returns how many
            let club_list: unsafe extern "thiscall" fn(usize, *mut usize, u32) -> i32 = core::mem::transmute(read_usize(read_usize(found) + 0x58));
            let mut list = 0usize;
            let n = club_list(found, &mut list, 0) as i16;
            let mut j: i16 = 0;
            while j < n && nf < 2 {
                let c = read_usize(list + j as i32 as usize * 4);
                let club_nation = read_usize(c + club::NATION);
                if club_nation != 0 && read_usize(club_nation) == read_usize(KEY_NATION_ENGLAND) {
                    same_year[nf as usize] = c;
                    nf += 1;
                }
                j += 1;
            }
            if list != 0 {
                exe::crt_free(list);
            }
        }
    }
    if cup.year() == 0x7cf && nf == 1 && read_u8(this.addr() + eng_fa_cup::FIFTH_ROUND_QUESTION_ASKED) != 2 {
        write_u8(this.addr() + eng_fa_cup::FIFTH_ROUND_ENTRY, nf as u8);
        write_u8(this.addr() + eng_fa_cup::EXEMPTION_COUNT, 0);
    } else {
        write_u8(this.addr() + eng_fa_cup::FIFTH_ROUND_ENTRY, 0);
        write_u8(this.addr() + eng_fa_cup::EXEMPTION_COUNT, nf as u8);
    }
    // 2) the table
    cup.set_team_count(0x98);
    let t = exe::crt_malloc(0x390);
    cup.set_team_table(t);
    if t == 0 {
        show_error_at(0x9c42cc, 0x1d2);
        return -1;
    }
    // 3) the prize places
    let base = read_usize(G_CLUB_TABLE);
    let mut a = 0usize;
    let mut b = 0usize;
    let cup_nation = Nation(read_usize(this.competition().addr() + competition::NATION));
    if cup_nation.league_selected() & league_selected::LOWER_DIVISIONS_LOADED != 0 || cup.year() == 0x7d1 {
        covered!("cqv_7 trophy winner and runner-up");
        let record = read_usize(G_CLUB_COMP_TABLE) + read_usize(KEY_CLUB_COMP_ENGLISH_FA_TROPHY).wrapping_mul(0x6b);
        a = exe::comp_util_get_latest_placed_club(record, 0); // cqv7: the winner
        let record = read_usize(G_CLUB_COMP_TABLE) + read_usize(KEY_CLUB_COMP_ENGLISH_FA_TROPHY).wrapping_mul(0x6b);
        b = exe::comp_util_get_latest_placed_club(record, 1);
    } else if read_usize(G_CLUB_COUNT) as i32 > 0 && eng_auto_cup_is_in_comp(base, KEY_CLUB_COMP_ENGLISH_CONFERENCE) {
        // simplification: the original repeats the test of the first club without advancing its loop; one test is enough
        a = base;
    }
    // 4) the Conference and the non-league clubs
    // simplification: the original does not zero the lists (stack leftovers); it only reads past what it filled when
    // clubs are missing (fewer than 38 non-league), which the career never has; the copy uses 0, which the original
    // also skips
    let mut v = [0usize; 322];
    let mut nconf = 0i32;
    let mut nout = 0i32;
    let england = read_usize(G_NATION_TABLE) + read_usize(KEY_NATION_ENGLAND).wrapping_mul(nation::SIZE);
    let mut i: i16 = 0;
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        let c = base + i as u16 as usize * club::SIZE;
        let l = read_usize(c + club::DIVISION);
        if l != 0 {
            let id = read_usize(l);
            if id == read_usize(KEY_CLUB_COMP_ENGLISH_CONFERENCE) {
                // simplification: the original does not check the size of the Conference list (22 places; the Conference has 22)
                v[nconf as i16 as usize] = c;
                nconf += 1;
            } else if !fa_cup_is_league_division(id) && read_usize(c + club::NATION) == england && (nout as i16) < 0x12c {
                v[(FA_CUP_NON_LEAGUE_START + nout as i16 as i32) as usize] = c;
                nout += 1;
            }
        }
        i = i.wrapping_add(1);
    }
    // 5) the prize clubs leave their own list
    let mut conf_places: i8 = 0x16;
    let mut non_league_places: i8 = 0x26;
    for (k, &p) in [a, b].iter().enumerate() {
        if p == 0 || read_usize(p + club::DIVISION) == 0 {
            continue;
        }
        let id = read_usize(read_usize(p + club::DIVISION));
        if id == read_usize(KEY_CLUB_COMP_ENGLISH_CONFERENCE) {
            fa_cup_remove_club(&mut v, 0, &mut nconf, p);
            if k == 0 {
                conf_places = 0x15;
            } else {
                conf_places -= 1;
            }
        } else if !fa_cup_is_league_division(id) {
            if !fa_cup_remove_club(&mut v, FA_CUP_NON_LEAGUE_START, &mut nout, p) {
                nout -= 1;
                v[FA_CUP_NON_LEAGUE_START as usize + k] = v[(FA_CUP_NON_LEAGUE_START + nout as i16 as i32) as usize];
            }
            if k == 0 {
                non_league_places = 0x25;
            } else {
                non_league_places -= 1;
            }
        }
    }
    // 6) two drawn Conference clubs go to the end; the non-league clubs by reputation
    let n = nconf as i16 as i32;
    let r1 = exe::utils_random(n - 2) as i8 as i32;
    let r2 = exe::utils_random(n - 2) as i8 as i32;
    v.swap(r1 as usize, (n - 1) as usize);
    v.swap(r2 as usize, (n - 2) as usize);
    // simplification: the original pushes the places in an edi with garbage in the high 16 bits; 0x4afbd0 reads the word
    exe::comp_util_sort_and_shuffle_top_clubs(v.as_mut_ptr() as usize + FA_CUP_NON_LEAGUE_START as usize * 4, nout as u32, non_league_places as i16 as u16 as u32); // cqv7: the non-league clubs by reputation
    // 7) the table
    let mut k = 0i32;
    for j in 0..(non_league_places as i32).max(0) {
        let c = v[(FA_CUP_NON_LEAGUE_START + j) as usize];
        if c != 0 {
            fa_cup_add_entrant(cup, &mut k, c);
        }
    }
    for j in 0..(conf_places as i32).max(0) {
        let c = v[j as usize];
        if c != 0 {
            fa_cup_add_entrant(cup, &mut k, c);
        }
    }
    if a != 0 {
        fa_cup_add_entrant(cup, &mut k, a);
    }
    if b != 0 {
        fa_cup_add_entrant(cup, &mut k, b);
    }
    let (x1, x2) = (same_year[0], same_year[1]);
    fa_cup_add_division(cup, &mut k, KEY_CLUB_COMP_ENGLISH_THIRD_DIVISION, &[a, b, x1, x2]); fa_cup_add_division(cup, &mut k, KEY_CLUB_COMP_ENGLISH_SECOND_DIVISION, &[x1, x2]); // cqv7: the third and the second
    fa_cup_add_division(cup, &mut k, KEY_CLUB_COMP_ENGLISH_FIRST_DIVISION, &[x1, x2]);
    covered!("cqv_7 first division shuffled");
    for _ in 0..0x3c { // cqv7: 60 swaps in the first division
        let r1 = exe::utils_random(0x18) as i8 as i32;
        let r2 = exe::utils_random(0x18) as i8 as i32;
        let t = cup.team_table();
        let p1 = t + (k - r1 - 1) as usize * 6;
        let p2 = t + (k - r2 - 1) as usize * 6;
        let v1 = read_usize(p1);
        write_usize(p1, read_usize(p2));
        write_usize(p2, v1);
    }
    fa_cup_add_division(cup, &mut k, KEY_CLUB_COMP_ENGLISH_PREMIER_DIVISION, &[x1, x2]);
    let nb = read_i8(this.addr() + eng_fa_cup::EXEMPTION_COUNT);
    let mut j: i16 = 0;
    while j < nb as i16 {
        fa_cup_add_entrant(cup, &mut k, same_year[j as usize]);
        j += 1;
    }
    if read_u8(this.addr() + eng_fa_cup::FIFTH_ROUND_ENTRY) == 1 {
        cup.set_team_count(cup.team_count().wrapping_sub(1));
    }
    (k == cup.team_count() as i32) as i32
}

/// The body of `construct_eng_fa_cup` (its arguments are what the mutation tests change).
pub(crate) unsafe fn construct_eng_fa_cup_body(this: Cup, year: u32, record: usize) -> usize {
    exe::new_cup(this);
    write_usize(this.addr() + 4, record); // the competition record (Comp +4)
    write_usize(this.addr(), 0x969b28); // the vtable
    write_u16(this.addr() + cup::YEAR, year as u16);
    this.set_current_round(0);
    this.set_stage_index(stage_index::MAIN);
    this.set_stage_count(0);
    write_i32(this.addr() + cup::CURRENT_STAGE, -1);
    this.set_club_sides(1);
    this.set_discipline_index(discipline_rule::ENGLAND);
    this.set_run_by_stages(0);
    this.set_season_set_up(0);
    let load: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_CUP_LOAD_FROM_DISK); // Cup::load_from_disk
    if load(this.addr(), 1) != 0 {
        return this.addr();
    }
    covered!("cqu_5 fa cup new");
    write_u8(this.addr() + eng_fa_cup::FIFTH_ROUND_QUESTION_ASKED, 0);
    write_u8(this.addr() + eng_fa_cup::FIFTH_ROUND_ENTRY, 1);
    write_u8(this.addr() + eng_fa_cup::EXEMPTION_COUNT, 0);
    this.set_team_count(0x98);
    let v = exe::crt_calloc(0x98, 6);
    this.set_team_table(v);
    if v == 0 {
        show_error_at(0x9c42cc, 0x63);
        return this.addr();
    }
    this.set_prize_fund(0x1e8480);
    eng_fa_cup_create_rounds(this);
    let map_fixture_tree: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(FN_CUP_MAP_FIXTURE_TREE); // Cup::map_fixture_tree
    let r = map_fixture_tree(this.addr());
    if r < 0 {
        show_error_at(0x9c42cc, 0x71);
        return this.addr();
    }
    if !eng_fa_cup_create_comp_stats(this) {
        show_error_at(0x9c42cc, 0x79);
        return this.addr();
    }
    this.addr()
}

/// True if the club's division id is one of the five English league divisions (the generic lower division
/// `KEY_CLUB_COMP_A_LOWER_DIVISION`, the Premier and the three Football League divisions) or the Conference
/// [inference from the 20 + 24 x 3 + 22].
pub(crate) unsafe fn fa_cup_is_league_division(id: usize) -> bool {
    [KEY_CLUB_COMP_A_LOWER_DIVISION, KEY_CLUB_COMP_ENGLISH_PREMIER_DIVISION, KEY_CLUB_COMP_ENGLISH_FIRST_DIVISION, KEY_CLUB_COMP_ENGLISH_SECOND_DIVISION, KEY_CLUB_COMP_ENGLISH_THIRD_DIVISION, KEY_CLUB_COMP_ENGLISH_CONFERENCE].iter().any(|&g| id == read_usize(g))
}

/// Removes club `c` from the list starting at `start` (`n` used): swaps it with the last one and decrements the count;
/// returns whether it was found.
pub(crate) fn fa_cup_remove_club(v: &mut [usize; 322], start: i32, n: &mut i32, c: usize) -> bool {
    let mut found = false;
    let mut j = 0i32;
    while (j as i16) < *n as i16 {
        if v[(start + j) as usize] == c {
            *n -= 1;
            v[(start + j) as usize] = v[(start + *n as i16 as i32) as usize];
            found = true;
        }
        j += 1;
    }
    found
}

/// Writes the club in slot `k` of the cup's team table (`cup::TEAM_TABLE`, 6-byte entries: the club, then +4 and +5
/// zeroed) and counts it.
pub(crate) unsafe fn fa_cup_add_entrant(cup: Cup, k: &mut i32, c: usize) {
    let t = cup.team_table() + *k as usize * 6;
    write_usize(t, c);
    write_u8(t + 4, 0);
    write_u8(t + 5, 0);
    *k += 1;
}

/// The clubs of division `key`, in club table order, except those in `skip`, until the team table is full
/// (k == `cup::TEAM_COUNT`).
pub(crate) unsafe fn fa_cup_add_division(cup: Cup, k: &mut i32, key: usize, skip: &[usize]) {
    let base = read_usize(G_CLUB_TABLE);
    let mut i: i16 = 0;
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        let c = base + i as u16 as usize * club::SIZE;
        if eng_auto_cup_is_in_comp(c, key) && !skip.contains(&c) {
            fa_cup_add_entrant(cup, k, c);
            if *k == cup.team_count() as i32 {
                return;
            }
        }
        i = i.wrapping_add(1);
    }
}

/// The size of a competition record of `G_CLUB_COMP_TABLE` (107 bytes).
pub(crate) const CLUB_COMP_RECORD_SIZE: usize = 0x6b;

/// The original keeps its two lists side by side on the stack (the Conference one, 22 places, and right after it the
/// non-league one, 300): the copy uses a single array, the second list from index 22, so it overflows as the original
/// would (it never does).
pub(crate) const FA_CUP_NON_LEAGUE_START: i32 = 22;

/// The fixed fields `construct_eng_fa_cup` writes on a new cup, for the comparer: (offset, width, value).
pub(crate) const FA_CUP_CONSTRUCTOR_FIELDS: [(usize, u8, u32); 13] = [(cup::PRIZE_FUND, 4, 0x1e8480), (cup::STAGE_COUNT, 4, 0x0), (cup::CURRENT_STAGE, 4, 0xffffffff), (cup::TEAM_COUNT, 2, 0x98), (cup::CLUB_SIDES, 1, 0x1), (cup::STAGE_INDEX, 1, 0xff), (cup::SEASON_SET_UP, 4, 0x0), (cup::DISCIPLINE_INDEX, 1, 0x9), (cup::CURRENT_ROUND, 1, 0x0), (cup::RUN_BY_STAGES, 1, 0x0), (eng_fa_cup::FIFTH_ROUND_QUESTION_ASKED, 1, 0x0), (eng_fa_cup::EXEMPTION_COUNT, 1, 0x0), (eng_fa_cup::FIFTH_ROUND_ENTRY, 1, 0x1)];
