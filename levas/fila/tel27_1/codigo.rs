// ---------- batch %%LEVA%%: the award screens (award_screens.cpp): the award screen (the year, month and team awards), its menus and clicks, the nominations screen ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\award_screens.cpp", the path the award_screens.cpp error boxes print.
const AWARD_SCREENS_CPP: usize = 0x987fa4;

/// The award object fields the screens read (the AWARD base of `award_fields`): the kind (byte: 0 team award, 1 year
/// award, 2 month award), the staff competition (its name at +4), the club competition (0 for a nation's own awards),
/// the nation award object's index in the award manager and the award's index in it, and the shortlist (the
/// nominations, AWARD_SHORTLIST) [inference: from the screens and the vote calls].
const AWARD_KIND: usize = 4;
const AWARD_STAFF_COMP: usize = 5;
const AWARD_CLUB_COMP: usize = 9;
const AWARD_NATION_INDEX: usize = 0xd;
const AWARD_INDEX: usize = 0x11;
const AWARD_SHORTLIST: usize = 0x1f;
/// YEAR_AWARD: how many years the season spans back (0: one year, "%d"; else "%d/%02d") and whether the value column
/// (goals and the like) is shown [inference].
const YEAR_AWARD_SEASON_SPAN: usize = 0x1d;
const YEAR_AWARD_SHOW_VALUE: usize = 0x1e;
/// MONTH_AWARD: the number of month slots filled (byte; the slots are `month_award_fields`, 11 bytes from +0x1d).
const MONTH_AWARD_SLOT_COUNT: usize = 0xa1;
/// TEAM_AWARD: the season span byte (as the year award's) and the number of substitutes past the eleven (signed byte).
const TEAM_AWARD_SEASON_SPAN: usize = 0x25;
const TEAM_AWARD_SUBSTITUTES: usize = 0x253;
/// A nation award object (NationAwards, one of the 30 in the award manager's +8 table): its award array, the count
/// (i16), the nation and the kind byte (1: a nation's own awards) [inference].
const NATION_AWARDS_TABLE: usize = 8;
const NATION_AWARDS_LIST: usize = 4;
const NATION_AWARDS_COUNT: usize = 8;
const NATION_AWARDS_NATION: usize = 0xe;
const NATION_AWARDS_NATIONAL: usize = 0x12;
/// The no-op virtual (`shared_noop_ret4`) the nation award objects without a screen refresh keep in slot 4.
const NATION_AWARDS_NOOP_REFRESH: usize = 0x583470;

/// The award_screens.cpp "Error" box at a line.
unsafe fn award_screens_error_box(line: u32) {
    tel26_error_box(AWARD_SCREENS_CPP, line);
}

/// The nation award object `index` of the award manager (its +8 table).
unsafe fn award_nation_awards(index: u32) -> usize {
    read_usize(read_usize(read_usize(G_AWARD_MANAGER) + NATION_AWARDS_TABLE) + index as usize * 4)
}

/// Award `k` of a nation award object.
unsafe fn award_in(group: usize, k: isize) -> usize {
    read_usize((read_usize(group + NATION_AWARDS_LIST) as isize + k * 4) as usize)
}

/// `AWARD_MANAGER::find_award_by_staff_comp` (0x414a60, thiscall, ret 4): the award of a staff competition, or 0.
unsafe fn award_find_by_staff_comp(staff_comp: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x414a60usize);
    f(read_usize(G_AWARD_MANAGER), staff_comp)
}

/// `background_set_award_picture` (0x41c560, cdecl): the background for an award's staff competition (`exact_only` 1
/// only tries its own picture and answers whether it found one).
unsafe fn award_background_picture(staff_comp: usize, variant: u32, exact_only: u32) -> i32 {
    let f: unsafe extern "C" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x41c560usize);
    f(staff_comp, variant, exact_only)
}

/// The background of an award screen: the staff competition's own picture; without one, the club competition's
/// (when the award has one), else the staff competition's with the fallbacks.
unsafe fn award_screen_background(award: usize) {
    if award_background_picture(read_usize(award + AWARD_STAFF_COMP), 0, 1) == 0 && read_usize(award + AWARD_CLUB_COMP) != 0 {
        exe::background_set_comp_picture(read_usize(award + AWARD_CLUB_COMP), 0, 0);
    } else {
        award_background_picture(read_usize(award + AWARD_STAFF_COMP), 0, 0);
    }
}

/// The title colours of an award screen (the first and the second, as `database_get_*_colours` write them): the club
/// competition's; for a nation's own awards the national team's kit (`database_get_club_kit_colours`, kit 1); else
/// the World Cup's (`KEY_CLUB_COMP_FIFA_WORLD_CUP`).
unsafe fn award_title_colours(award: usize, group: usize, first: &mut u32, second: &mut u32) {
    let comp = read_usize(award + AWARD_CLUB_COMP);
    if comp != 0 {
        exe::database_get_comp_colours(comp, first, second);
    } else if read_u8(group + NATION_AWARDS_NATIONAL) == 1 {
        let team = exe::database_get_national_team(read_usize(group + NATION_AWARDS_NATION));
        exe::database_get_club_kit_colours(team, 1, first, second);
    } else {
        exe::database_get_comp_colours(award_world_cup(), first, second);
    }
}

/// The World Cup's competition record (`G_CLUB_COMP_TABLE`, 0x6b bytes each, index at `KEY_CLUB_COMP_FIFA_WORLD_CUP`).
unsafe fn award_world_cup() -> usize {
    read_usize(G_CLUB_COMP_TABLE).wrapping_add((read_i32(KEY_CLUB_COMP_FIFA_WORLD_CUP) as usize).wrapping_mul(0x6b))
}

/// The colour of a person's name on the award screens: highlighted (0xaebdec) when a human runs his club or his
/// national team or he is one of the 16 human managers (the last 16 ids), else the plain colour (0xae3184).
unsafe fn award_person_colour(person: Staff) -> u32 {
    let club = person.club_job();
    if !club.is_null() && exe::human_manager_is_human_club(club, 0, 1, 0, 0) != 0 {
        return job_colour(JOB_COLOUR_BDEC);
    }
    let nation = person.national_job();
    if !nation.is_null() && exe::human_manager_is_human_club(nation, 0, 1, 0, 0) != 0 {
        return job_colour(JOB_COLOUR_BDEC);
    }
    if read_i32(person.addr()) >= read_i32(G_STAFF_COUNT) - 0x10 {
        job_colour(JOB_COLOUR_BDEC)
    } else {
        job_colour(JOB_COLOUR_3184)
    }
}

/// The season text of a year or team award in [`JOB_TEXT`]: "%d" of the year, or "%d/%02d" of the year minus the
/// span and the year's last two digits.
unsafe fn award_season_text(year: u32, span: i8) {
    if span != 0 {
        comp_crt_format()(JOB_TEXT, 0x988070, year as i32 - span as i32, year % 100);
    } else {
        comp_crt_format()(JOB_TEXT, 0x98806c, year);
    }
}

/// The text area "No awards have been made." between the two y bounds (pale grey, font 3), as the three award
/// draws show it when there is nothing to list.
unsafe fn award_no_awards(y: i32, y2: i32) {
    let rows = ((y2 - y - 0x10) / exe::font_get_height(3)) as u8;
    comp_lang_format()(JOB_TEXT, 0x987fd4);
    tel26_text_area([0x6e, y as u32, 0x30c, y2 as u32, 2, 0, rows as u32, 3, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 0]);
}

/// Rule: opens the award screen of a staff competition; without one, the award_screens.cpp 90 box.
///
/// `award_screens_open_award_screen(staff_comp)`: `open_screen` with `award_screen_draw` and
/// `award_screen_handle_action` (flag 0, no leave or exit routine); when it opens, parameter 0 is the award of that
/// staff competition (`AWARD_MANAGER::find_award_by_staff_comp`) and parameter 1 (the team award's year) 0.
/// Original: `award_screens_open_award_screen` (0x414c20, award_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn award_screens_open_award_screen(staff_comp: usize) {
    if staff_comp == 0 {
        award_screens_error_box(0x5a);
        return;
    }
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x414cf0usize, 0x416960usize, 0, 0usize, 0) == 0 {
        return;
    }
    covered!("tel27_1 award screen opened");
    exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, 0, award_find_by_staff_comp(staff_comp), 0);
    exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, 1, 0usize, 0);
}

/// The screen nodes of the current channel whose build routine is `build` (to tell a new screen from one already
/// in the history).
pub(crate) unsafe fn screen_nodes_built_by(build: usize) -> Vec<usize> {
    let mut v = Vec::new();
    let mut node = read_usize(screen_record(G_SCREEN_MANAGER) + screen_history::FIRST_NODE);
    while node != 0 {
        if read_usize(node + screen_node_fields::BUILD) == build {
            v.push(node);
        }
        node = read_usize(node + screen_node_fields::NEXT);
    }
    v
}

/// Whether the call just made opened a new screen with the build routine `build` (the current node is one, and it
/// was not in the history before).
pub(crate) unsafe fn screen_newly_opened(before: &[usize], build: usize) -> bool {
    let node = current_screen_node(G_SCREEN_MANAGER);
    node != 0 && read_usize(node + screen_node_fields::BUILD) == build && !before.contains(&node)
}

/// Compares the opened screen's first parameters with a plan (the opener's comparers: only the original runs, it
/// opens a screen).
pub(crate) unsafe fn compare_opened_params(name: &str, plan: &[usize]) {
    let got: Vec<usize> = (0..plan.len() as u32).map(|k| get_screen_param(k)).collect();
    SCOREBOARD.record(got == plan, || format!("{}: params {:x?} x plan {:x?}", name, got, plan));
}

/// Comparer for `award_screens_open_award_screen`: only the original runs (it opens a screen); when a new award screen
/// opened, its parameters 0 and 1 are checked against the plan.
pub(crate) unsafe extern "C" fn compare_award_screens_open_award_screen(staff_comp: usize) {
    let before = screen_nodes_built_by(0x414cf0);
    let plan = if staff_comp != 0 { [award_find_by_staff_comp(staff_comp), 0] } else { [0, 0] };
    original_fn::<unsafe extern "C" fn(usize)>(%%I0%%)(staff_comp);
    if staff_comp != 0 && screen_newly_opened(&before, 0x414cf0) {
        compare_opened_params("award screen (open)", &plan);
    }
}

/// Rule: the award screen shows one award: the title, a menu of the other competitions' awards of the same kind, a
/// "View" list of the awards of this competition, "Nominations" when there are any, and the award's own list (the
/// year award's winners, the month award's months or the team award's pitch).
///
/// The award is parameter 0 and its nation award object the award manager's +8 entry at `AWARD_NATION_INDEX`. The
/// background (`award_screen_background`); the title: "{}<%s - competition>{} Awards" of the club competition (its
/// gender byte twice), else for a nation's own awards "{}<%s - nationality>{} Awards" of the nationality adjective
/// (`nation_get_nationality_adjective`, 0x64 bytes, form 0x10), else "International Awards", in the title colours
/// (`award_title_colours`; `add_screen_title(second, first, text)`). Without an award the award_screens.cpp 1181 box.
///
/// The menu: counting back from `AWARD_INDEX` how many awards just before it share its club competition (its place in
/// that run), and how many runs of the same club competition the nation award object has; with two runs or more, a
/// button (0x69, 0xf)-(0x78, 0x23) carries a menu area (`ScreenManager::add_menu_area`, made once) with one item per
/// run (`add_menu_item`, id 1): the award at the same place of that run when there is one (else the run's first), named
/// by its club competition or "Overall". Then the menu bar (4) and the subtitle (the staff competition's name, flag 1).
///
/// The "View" list (y from 0x91, the bottom 0x217): with more than one award of the same club competition, a "View"
/// button (0x6e, y)-(0xeb, y + 0x14) carries a popup (flags 0x130) with each of them named by its staff competition
/// (+4 when its nation index is 1, else +0x38): the current one ticked (id 0), the others plain items (id 2, value the
/// award); y moves 0x19 down. "Nominations" (0x28f, y)-(0x30c, y + 0x14), id 3, for a year award whose shortlist has
/// entries (with the View button the same row, else y moves 0x19 down). Then the nation award object's refresh virtual
/// (slot 4, with the award) and the award's list: a year award `award_screen_draw_year_award`, a team award
/// `award_screen_draw_team_award`, a month award `award_screen_month_award_list`. The Back and Next buttons last.
/// Original: `award_screen_draw` (0x414cf0, award_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn award_screen_draw() {
    let award = get_screen_param(0);
    award_screen_background(award);
    let group = award_nation_awards(read_u32(award + AWARD_NATION_INDEX));
    let comp = read_usize(award + AWARD_CLUB_COMP);
    let (mut first, mut second) = (0u32, 0u32);
    if comp != 0 {
        let gender = read_i8(comp + competition::SHORT_GENDER_NAME) as i32;
        comp_lang_format()(JOB_TEXT, 0x98804c, gender, gender, comp + competition::SHORT_NAME);
    } else if read_u8(group + NATION_AWARDS_NATIONAL) == 1 {
        let mut adjective = [0u8; 100];
        exe::nation_get_nationality_adjective(read_usize(group + NATION_AWARDS_NATION), adjective.as_mut_ptr() as usize, 0x64, 0x10);
        comp_lang_format()(JOB_TEXT, 0x98802c, 0x64, 0x64, adjective.as_ptr());
    } else {
        comp_lang_format()(JOB_TEXT, 0x988014);
    }
    award_title_colours(award, group, &mut first, &mut second);
    exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, second, first, JOB_TEXT);
    if award == 0 {
        // the original reads the award before this test: unreachable
        award_screens_error_box(0x49d);
        return;
    }
    award_screen_menu(award, group, first, second);
    exe::menubar_build(4, 0);
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, read_usize(award + AWARD_STAFF_COMP) + 4, 1);
    let mut y: i32 = 0x91;
    let mut y2: i32 = 0x217;
    let count = read_i16(group + NATION_AWARDS_COUNT);
    let mut same: u8 = 0;
    for k in 0..count.max(0) as isize {
        if read_usize(award_in(group, k) + AWARD_CLUB_COMP) == comp {
            same = same.wrapping_add(1);
        }
    }
    if same as i8 > 1 {
        covered!("tel27_1 award view popup");
        comp_lang_format()(JOB_TEXT, 0x988004);
        let button = tel26_add_button([0x6e, y as u32, 0xeb, (y + 0x14) as u32, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 1,
            job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 0, 0]);
        let popup = tel26_add_area(current_human_display(), [0, 0, 0, 0, 0, 0, 0, 0, 0x130, job_colour(JOB_COLOUR_31AA), button]);
        for k in 0..read_i16(group + NATION_AWARDS_COUNT).max(0) as isize {
            let other = award_in(group, k);
            if read_usize(other + AWARD_CLUB_COMP) != comp {
                continue;
            }
            let staff_comp = read_usize(other + AWARD_STAFF_COMP);
            let mut name = [0u8; 0xc8];
            let src = if read_i32(other + AWARD_NATION_INDEX) == 1 { staff_comp + 4 } else { staff_comp + 0x38 };
            tel26_copy_text(name.as_mut_ptr(), src as *const u8);
            if other == award {
                screenmanager_add_ticked_popup_item(G_SCREEN_MANAGER, popup, name.as_ptr() as u32, 0, 0);
            } else {
                screen_manager_add_menu_item(G_SCREEN_MANAGER, popup, name.as_ptr() as u32, 2, other as u32);
            }
        }
        y += 0x19;
    }
    if read_u8(award + AWARD_KIND) == 1 {
        let list = read_usize(award + AWARD_SHORTLIST);
        if list != 0 && read_i16(list + 4) != 0 {
            if same as i8 > 1 {
                y -= 0x19;
            }
            comp_lang_format()(JOB_TEXT, 0x987ff8);
            tel26_add_button([0x28f, y as u32, 0x30c, (y + 0x14) as u32, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 1,
                job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 3, 0]);
            y += 0x19;
        }
    }
    let refresh: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(read_usize(read_usize(group) + 0x10));
    refresh(group, award);
    match read_u8(award + AWARD_KIND) {
        1 => award_screen_draw_year_award(&mut y, &mut y2),
        2 => award_screen_month_award_list(y, y2),
        0 => award_screen_draw_team_award(&mut y, &mut y2),
        _ => {}
    }
    tel26_back_next();
}

/// The award screen's menu of the other runs of the nation award object (a part of `award_screen_draw`, see there).
unsafe fn award_screen_menu(award: usize, group: usize, first: u32, second: u32) {
    let comp = read_usize(award + AWARD_CLUB_COMP);
    let index = read_i32(award + AWARD_INDEX);
    let mut place: i8 = 0;
    if index > 0 {
        loop {
            if read_usize(award_in(group, (index - place as i32 - 1) as isize) + AWARD_CLUB_COMP) != comp {
                break;
            }
            place = place.wrapping_add(1);
            if index - place as i32 <= 0 {
                break;
            }
        }
    }
    let count = read_i16(group + NATION_AWARDS_COUNT);
    if count <= 0 {
        return;
    }
    let same_comp = |a: isize, b: isize| read_usize(award_in(group, a) + AWARD_CLUB_COMP) == read_usize(award_in(group, b) + AWARD_CLUB_COMP);
    let mut runs: i8 = 0;
    let mut k: i8 = 0;
    while (k as i16) < count {
        if k == 0 || !same_comp(k as isize, k as isize - 1) {
            runs = runs.wrapping_add(1);
        }
        k = k.wrapping_add(1);
    }
    if runs < 2 {
        return;
    }
    covered!("tel27_1 award competition menu");
    let mut menu: u32 = u32::MAX;
    let mut k: i8 = 0;
    while (k as i16) < read_i16(group + NATION_AWARDS_COUNT) {
        let i = k as isize;
        if k == 0 || !same_comp(i, i - 1) {
            if menu as u16 == 0xffff {
                let button = tel26_add_button([0x69, 0xf, 0x78, 0x23, 0x30, second, 0xc, 1, first, 0, 0, 0]);
                menu = screen_manager_add_menu_area(G_SCREEN_MANAGER, button);
            }
            let mut entry = award_in(group, i);
            let at = place as isize + i;
            if at < read_i16(group + NATION_AWARDS_COUNT) as isize {
                let candidate = award_in(group, at);
                if read_usize(candidate + AWARD_CLUB_COMP) == read_usize(entry + AWARD_CLUB_COMP) {
                    entry = candidate;
                }
            }
            let entry_comp = read_usize(entry + AWARD_CLUB_COMP);
            let text = if entry_comp != 0 {
                entry_comp + competition::SHORT_NAME
            } else {
                comp_lang_format()(JOB_TEXT, 0x98800c);
                JOB_TEXT
            };
            screen_manager_add_menu_item(G_SCREEN_MANAGER, menu, text as u32, 1, entry as u32);
        }
        k = k.wrapping_add(1);
    }
}

/// The month award's list (a part of `award_screen_draw`): with a winner in the first slot, a 3-column area (0x6e,
/// y)-(0x30c, y2), weights 5/12/10, rows as many as font 3 fits in y2 − y − 0x10, with one row per filled slot
/// (`MONTH_AWARD_SLOT_COUNT`): the month name (`date_get_month_name`, column 0, 0xaebe04), the winner "  <name>"
/// (`database_format_staff_name`, a link 0x7d7 to him in `award_person_colour`) and, with a club, the club's short name
/// (a link 0x7d5); a slot without a winner keeps only its month. Without a winner in the first slot, "No awards have
/// been made.".
/// Original: part of `award_screen_draw` (0x414cf0, award_screens.cpp).
unsafe fn award_screen_month_award_list(y: i32, y2: i32) {
    let award = get_screen_param(0);
    let weights: [u8; 3] = [5, 0xc, 0xa];
    if read_i32(award + month_award_fields::SLOT_WINNER_ID) <= -1 {
        award_no_awards(y, y2);
        return;
    }
    covered!("tel27_1 month award list");
    let rows = ((y2 - y - 0x10) / exe::font_get_height(3)) as u8;
    let area = tel26_add_area(current_human_display(), [0x6e, y as u32, 0x30c, y2 as u32, 3, weights.as_ptr() as u32, rows as u32, 0, 2, 0, u32::MAX]);
    let month_name: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x54d3b0usize);
    let mut k: i8 = 0;
    let mut row: u32 = 0;
    while k < read_i8(award + MONTH_AWARD_SLOT_COUNT) {
        let slot = award + k as usize * 0xb;
        let month = month_name(read_u8(slot + month_award_fields::SLOT_MONTH) as u32);
        tel26_area_text([area, 0, row, 0x30, job_colour(JOB_COLOUR_BE04), 0xc, 1, job_colour(JOB_COLOUR_3184), month as u32]);
        let winner = read_i32(slot + month_award_fields::SLOT_WINNER_ID);
        if winner > -1 {
            let person = staff_at(winner as isize);
            let mut name = [0u8; 0xc8];
            exe::database_format_staff_name(person, name.as_mut_ptr() as usize, 0xc8, 1);
            comp_crt_format()(JOB_TEXT, 0x987ff0, name.as_ptr());
            tel26_area_button([area, 1, row, 1, 0, 1, 3, award_person_colour(person), JOB_TEXT as u32, 0x7d7, person.addr() as u32]);
            let club_id = read_i32(slot + month_award_fields::SLOT_CLUB_ID);
            if club_id > -1 {
                let club = club_at(club_id as isize);
                tel26_area_button([area, 2, row, 1, 0, 1, 2, job_colour(JOB_COLOUR_BDEC), (club.addr() + club::SHORT_NAME) as u32, 0x7d5, club.addr() as u32]);
            }
        }
        row += 1;
        k = k.wrapping_add(1);
    }
}

/// Comparer for `award_screen_draw`: side by side over the display list (`compare_match_screen_build`); only the
/// original runs when the nation award object has a refresh of its own (it may rebuild the award, e.g. a team of the
/// week).
pub(crate) unsafe extern "C" fn compare_award_screen_draw() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I1%%);
    let award = get_screen_param(0);
    if award == 0 {
        return o();
    }
    let group = award_nation_awards(read_u32(award + AWARD_NATION_INDEX));
    // simplification: a refresh other than the no-op rebuilds the award; it does not run twice
    if read_usize(read_usize(group) + 0x10) != NATION_AWARDS_NOOP_REFRESH {
        return o();
    }
    compare_match_screen_build("award screen", &[], || o(), || award_screen_draw());
}

/// Rule: a menu area hangs from a button (flags 0x30, the popup colour).
///
/// `ScreenManager::add_menu_area(owner)`: `Display::add_area` of the current slot's `Display` with an empty rectangle
/// and nothing else, the flags 0x30, the popup colour (0xae31aa; the original's high word is the caller's edx, here 0)
/// and the owner widget; the area index comes back.
/// Original: `ScreenManager::add_menu_area` (0x415600, award_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn screen_manager_add_menu_area(sm: usize, owner: u32) -> u32 {
    let f: DisplayAddAreaFn = core::mem::transmute(FN_DISPLAY_ADD_AREA);
    f(screens_current_display(sm), 0, 0, 0, 0, 0, 0, 0, 0, 0x30, read_u16(JOB_COLOUR_31AA) as u32, owner)
}

/// Comparer for `screen_manager_add_menu_area`.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_add_menu_area(sm: usize, owner: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(%%I2%%);
    // simplification: the colour's high word is garbage in the original; the area keeps the colour as a word
    compare_screens_forwarder("screen_manager_add_menu_area", sm, || o(sm, owner), || screen_manager_add_menu_area(sm, owner))
}

/// Rule: an item of a menu is a plain text item (kind 2) with the popup colours, carrying its id and data.
///
/// `ScreenManager::add_menu_item(area, text, id, data)`: `Display::add_item` of the current slot's `Display` with 2,
/// six zeros, 0x10, the popup colour (0xae31aa), the text colour (0xaebdec), 1, 1, the highlight colour (0xaebdea),
/// the text, 0, the id, the data and the area; the colours go as the original pushes them (the word in the low half,
/// the id's high word for 0xae31aa and 0xaebdea, the text pointer's for 0xaebdec). The item index comes back.
/// Original: `ScreenManager::add_menu_item` (0x415640, award_screens.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn screen_manager_add_menu_item(sm: usize, area: u32, text: u32, id: u32, data: u32) -> u32 {
    let f: DisplayAddItemFn = core::mem::transmute(FN_DISPLAY_ADD_ITEM);
    let high = |v: u32, colour: usize| (v & 0xffff_0000) | read_u16(colour) as u32;
    f(screens_current_display(sm), 2, 0, 0, 0, 0, 0, 0, 0x10, high(id, JOB_COLOUR_31AA), high(text, JOB_COLOUR_BDEC), 1, 1,
      high(id, JOB_COLOUR_BDEA), text, 0, id, data, area)
}

/// Comparer for `screen_manager_add_menu_item`.
pub(crate) unsafe extern "thiscall" fn compare_screen_manager_add_menu_item(sm: usize, area: u32, text: u32, id: u32, data: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u32>(%%I3%%);
    compare_screens_item("screen_manager_add_menu_item", sm, area, || o(sm, area, text, id, data),
        || screen_manager_add_menu_item(sm, area, text, id, data))
}

/// `YEAR_AWARD::get_history_list` (0x9304d0, thiscall): a new list of the past winners (0 when there are none).
unsafe fn award_year_history(award: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x9304d0usize);
    f(award)
}

/// The person of a winner record when its id is valid and still has the recorded names
/// (`database_staff_matches_names`), else 0 (the name then is only text).
unsafe fn award_record_person(rec: usize) -> usize {
    let id = read_i32(rec + 0xc);
    if id > -1 && exe::database_staff_matches_names(staff_at(id as isize), read_u32(rec), read_u32(rec + 4), read_u32(rec + 8), 0usize) != 0 {
        staff_at(id as isize).addr()
    } else {
        0
    }
}

/// Rule: the year award's list: the latest season's top three in a header block, then one row per earlier season with
/// its winner.
///
/// `award_screen_draw_year_award(&y, &y2)`: the past winners (`YEAR_AWARD::get_history_list`, read from the newest);
/// none: "No awards have been made.". Each record: +0 the year (u16), then three places of 0x18 bytes from +2 (the
/// name ids +0, +4, +8, the person id +0xc, the value +0x10, the club id +0x14). The newest: its season
/// (`award_season_text`) as a yellow heading (0x6e, y)-(0x30c, y + 0x23), font 2; y moves 0x28; a 4-column area
/// (weights 2/12/10/3) of three rows (3 × font 3 height + 0x10 high) with "1st", "2nd", "3rd" (`utils_format_ordinal`)
/// and each place (copied over the first place of the list's copy): "  <name>" (`database_format_name_from_ids`, a
/// link 0x7d7 to the person in `award_person_colour` when `award_record_person` finds him, else text), the club (a link
/// 0x7d5) and the value ("%d" when `YEAR_AWARD_SHOW_VALUE`, else empty); a place without a name gets an empty value
/// only. y moves by the block's height + 5; then a 4-column area (weights 3/11/10/3) down to y2, rows as many as font 2
/// fits: one row per earlier season (its season, its winner in font 2 and style 2, the club or an empty highlighted
/// cell, the value; without a winner three empty cells), then empty rows up to the area's rows. The list is freed.
/// Defect of the original: a history list with no record leaves the row counters uninitialised (here no rows).
/// Original: `award_screen_draw_year_award` (0x4156a0, award_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn award_screen_draw_year_award(py: *mut i32, py2: *mut i32) {
    let award = get_screen_param(0);
    let header_weights: [u8; 4] = [2, 0xc, 0xa, 3];
    let list_weights: [u8; 4] = [3, 0xb, 0xa, 3];
    let list = award_year_history(award);
    if list == 0 {
        award_no_awards(*py, *py2);
        return;
    }
    write_usize(list + 0xa, read_usize(list + 6));
    let (mut area, mut row, mut rows) = (0u32, 0u32, 0u32);
    let mut n = 0;
    let mut rec = exe::llist_get_prev(list);
    while rec != 0 {
        award_season_text(read_u16(rec) as u32, read_i8(award + YEAR_AWARD_SEASON_SPAN));
        if n == 0 {
            covered!("tel27_1 year award header");
            comp_text_item([0x6e, *py as u32, 0x30c, (*py + 0x23) as u32, 2, 0, 0xc, 3, job_colour(JOB_COLOUR_BDE8), JOB_TEXT as u32, 1]);
            *py += 0x28;
            let height = exe::font_get_height(3) * 3 + 0x10;
            let header = tel26_add_area(current_human_display(), [0x6e, *py as u32, 0x30c, (*py + height) as u32, 4, header_weights.as_ptr() as u32, 3, 0, 2, 0, u32::MAX]);
            for k in 0..3u32 {
                let ordinal = exe::utils_format_ordinal(k as i32 + 1, -1);
                tel26_area_text([header, 0, k, 0x30, job_colour(JOB_COLOUR_BE04), 0xc, 1, job_colour(JOB_COLOUR_3184), ordinal as u32]);
                if k > 0 {
                    core::ptr::copy(((rec + 2) + k as usize * 0x18) as *const u8, (rec + 2) as *mut u8, 0x18);
                }
                award_year_place(award, rec + 2, header, k, false);
            }
            *py += height + 5;
            let y = *py;
            let y2 = *py2;
            rows = ((y2 - y) / exe::font_get_height(2)) as u8 as u32;
            area = tel26_add_area(current_human_display(), [0x6e, y as u32, 0x30c, y2 as u32, 4, list_weights.as_ptr() as u32, rows, 0, 1, 0, u32::MAX]);
            row = 0;
        } else {
            tel26_area_text([area, 0, row, 0x30, job_colour(JOB_COLOUR_BE04), 0xc, 1, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32]);
            award_year_place(award, rec + 2, area, row, true);
            row += 1;
        }
        n += 1;
        rec = exe::llist_get_prev(list);
    }
    while row < rows {
        tel26_area_text([area, 0, row, 2, 0, 0xc, 1, job_colour(JOB_COLOUR_3184), STR_EMPTY as u32]);
        tel26_area_text([area, 1, row, 2, 0, 1, 2, job_colour(JOB_COLOUR_3184), STR_EMPTY as u32]);
        tel26_area_text([area, 2, row, 2, 0, 1, 2, job_colour(JOB_COLOUR_BDEC), STR_EMPTY as u32]);
        tel26_area_text([area, 3, row, 2, 0, 0xc, 1, job_colour(JOB_COLOUR_3184), STR_EMPTY as u32]);
        row += 1;
    }
    exe::drop_llist(list);
    exe::crt_operator_delete(list);
}

/// Comparer for `award_screen_draw_year_award`: side by side over the display list and the two y bounds.
pub(crate) unsafe extern "C" fn compare_award_screen_draw_year_award(py: *mut i32, py2: *mut i32) {
    let o = original_fn::<unsafe extern "C" fn(*mut i32, *mut i32)>(%%I4%%);
    compare_match_screen_build("award screen (year award)", &[(py as usize, 4), (py2 as usize, 4)], || o(py, py2),
        || award_screen_draw_year_award(py, py2));
}

/// One place of a year award record (the 0x18 bytes at `place`) in row `row` of `area`: the header's style (font 3,
/// style 1) or the list's (`list`: font 2, style 2, an empty club cell); see `award_screen_draw_year_award`.
unsafe fn award_year_place(award: usize, place: usize, area: u32, row: u32, list: bool) {
    let (s, f) = if list { (2u32, 2u32) } else { (1u32, 3u32) };
    if read_i32(place) <= -1 {
        if list {
            tel26_area_text([area, 1, row, 2, 0, 1, 2, job_colour(JOB_COLOUR_3184), STR_EMPTY as u32]);
            tel26_area_text([area, 2, row, 2, 0, 1, 2, job_colour(JOB_COLOUR_BDEC), STR_EMPTY as u32]);
        }
        tel26_area_text([area, 3, row, 0x30, job_colour(JOB_COLOUR_3192), 0xc, 1, job_colour(JOB_COLOUR_3184), STR_EMPTY as u32]);
        return;
    }
    let person = award_record_person(place);
    let mut name = [0u8; 0xc8];
    exe::database_format_name_from_ids(read_u32(place), read_u32(place + 4), read_u32(place + 8), name.as_mut_ptr(), 0xc8, 1, read_u32(place + 0xc));
    comp_crt_format()(JOB_TEXT, 0x987ff0, name.as_ptr());
    if person != 0 {
        tel26_area_button([area, 1, row, s, 0, 1, f, award_person_colour(Staff(person)), JOB_TEXT as u32, 0x7d7, person as u32]);
    } else {
        tel26_area_text([area, 1, row, s, 0, 1, f, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32]);
    }
    let club_id = read_i32(place + 0x14);
    if club_id > -1 && !(list && club_at(club_id as isize).is_null()) {
        let club = club_at(club_id as isize);
        tel26_area_button([area, 2, row, s, 0, 1, 2, job_colour(JOB_COLOUR_BDEC), (club.addr() + club::SHORT_NAME) as u32, 0x7d5, club.addr() as u32]);
    } else if list {
        tel26_area_text([area, 2, row, 2, 0, 1, 2, job_colour(JOB_COLOUR_BDEC), STR_EMPTY as u32]);
    }
    if read_u8(award + YEAR_AWARD_SHOW_VALUE) != 0 {
        comp_crt_format()(JOB_TEXT, 0x98806c, read_i32(place + 0x10));
    } else {
        write_u8(JOB_TEXT, 0);
    }
    tel26_area_text([area, 3, row, 0x30, job_colour(JOB_COLOUR_3192), 0xc, 1, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32]);
}

/// The team award's dark blue for the substitutes' numbers (made once: `graphics_rgb_to_pixel(0, 0, 0x40)`, the word at
/// 0xad9c68 with its made flag bit 0 of 0xad9c6a).
const TEAM_AWARD_SUBS_COLOUR: usize = 0xad9c68;
const TEAM_AWARD_SUBS_COLOUR_MADE: usize = 0xad9c6a;

/// Rule: the team award's screen: one team (a past season or the current one), the squad listed on the left and the
/// eleven drawn on a pitch on the right.
///
/// `award_screen_draw_team_award(&y, &y2)`: the teams (`TEAM_AWARD::get_history_list`; none: "No awards have been
/// made."); parameter 1 picks the team (counted back from the newest; past the count, 0). The pitch (0x1e0, y)-(0x30c,
/// y2) (`draw_pitch`, its inner rectangle comes back). The team record: +0x1f4 the first date (−1: a season), +0x1fc
/// the last date, and from +0 one 0x19-byte entry per player (+0 person id, +4/+8/+0xc the name ids, +0x10 club id,
/// +0x14 the shirt number). The heading (0x6e, y)-(0x1d6, y + 0x23), yellow font 2: the season (`award_season_text` of
/// +0x1f6, span `TEAM_AWARD_SEASON_SPAN`) or the date (`Date::get_long_date_string` when both dates are equal, else
/// "%s - %s" of the short dates); y moves 0x28. With more than one team, a 2-column area (0x212, y − 0x41)-(0x30c, y −
/// 0x2d): "<< Year" (or "<< Date") as a button id 4 when an older team exists, else grey text; "Year >>" id 5 when a
/// newer one exists. Then a 4-column area (weights 14/7/60/45) from y to y2 with one row per player (the eleven and
/// `TEAM_AWARD_SUBSTITUTES` more): the number (the shirt number, else the row + 1; the substitutes' in dark blue), the
/// name (font 2; a link 0x7d7 when `database_staff_matches_names` holds; highlighted when his club is human, the
/// substitutes in 0xae317e) and the club (a link 0x7d5). Then `TEAM_AWARD::apply_formation` fills a Formation and each
/// of the eleven with a person gets, at his slot's pitch point (`Formation::get_slot_position`,
/// `get_pitch_point_for_position_flags` with `has_central_slot_in_line`), a shirt (kind 1, 26 × 26 around the point,
/// flags 0x10030, the club's kit colours or the plain ones) with his number and, under it, his surname (variant 5,
/// font 1, a link 0x7d7 or text). The list is freed.
/// Original: `award_screen_draw_team_award` (0x415f60, award_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn award_screen_draw_team_award(py: *mut i32, py2: *mut i32) {
    if read_u8(TEAM_AWARD_SUBS_COLOUR_MADE) & 1 == 0 {
        write_u8(TEAM_AWARD_SUBS_COLOUR_MADE, read_u8(TEAM_AWARD_SUBS_COLOUR_MADE) | 1);
        write_u16(TEAM_AWARD_SUBS_COLOUR, exe::graphics_rgb_to_pixel(0, 0, 0x40, 0) as u16);
    }
    let weights: [u8; 4] = [0xe, 7, 0x3c, 0x2d];
    let mut formation = vec![0u8; formation_fields::SIZE + 0x10];
    let formation = formation.as_mut_ptr() as usize;
    let construct: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x5b27c0usize);
    construct(formation);
    let award = get_screen_param(0);
    let mut picked = get_screen_param(1) as i32;
    let history: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x8b00a0usize);
    let list = history(award);
    if list == 0 {
        award_no_awards(*py, *py2);
        return;
    }
    covered!("tel27_1 team award");
    if picked >= read_i16(list) as i32 {
        picked = 0;
    }
    let (mut left, mut top, mut right, mut bottom) = (0i32, 0i32, 0i32, 0i32);
    let pitch: unsafe extern "C" fn(i32, i32, i32, i32, *mut i32, *mut i32, *mut i32, *mut i32, u32, i32, u32) -> u32 = core::mem::transmute(FN_DRAW_PITCH);
    pitch(0x1e0, *py, 0x30c, *py2, &mut left, &mut top, &mut right, &mut bottom, 0, -1, 0x4080_0000);
    write_usize(list + 0xa, read_usize(list + 6));
    let mut k: i8 = 0;
    while (k as i32) < picked {
        exe::llist_get_prev(list);
        k = k.wrapping_add(1);
    }
    let current: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d2e0usize);
    let rec = current(list);
    let label = if read_i16(rec + 0x1f4) == -1 {
        award_season_text(read_u16(rec + 0x1f6) as u32, read_i8(award + TEAM_AWARD_SEASON_SPAN));
        0x988098
    } else {
        if read_u16(rec + 0x1fe) == read_u16(rec + 0x1f6) && read_u16(rec + 0x1fc) == read_u16(rec + 0x1f4) {
            exe::date_get_long_date_string(rec + 0x1f4, JOB_TEXT, 0x7d0, 0xff);
        } else {
            let short: unsafe extern "thiscall" fn(usize, *mut u8, u32, u32) -> u32 = core::mem::transmute(0x54a650usize);
            let (mut from, mut to) = ([0u8; 0xc8], [0u8; 0xc8]);
            short(rec + 0x1f4, from.as_mut_ptr(), 0xc8, 0xff);
            short(rec + 0x1fc, to.as_mut_ptr(), 0xc8, 0xff);
            comp_crt_format()(JOB_TEXT, 0x988080, from.as_ptr(), to.as_ptr());
        }
        0x988078
    };
    comp_text_item([0x6e, *py as u32, 0x1d6, (*py + 0x23) as u32, 2, 0, 0xc, 3, job_colour(JOB_COLOUR_BDE8), JOB_TEXT as u32, 1]);
    *py += 0x28;
    if read_i16(list) > 1 {
        let y = *py;
        let arrows = tel26_add_area(current_human_display(), [0x212, (y - 0x41) as u32, 0x30c, (y - 0x2d) as u32, 2, 0, 1, 0, 1, 0, u32::MAX]);
        let mut word = [0u8; 0xc8];
        comp_lang_format()(word.as_mut_ptr() as usize, label);
        comp_crt_format()(JOB_TEXT, 0x988090, word.as_ptr());
        if picked < read_i16(list) as i32 - 1 {
            tel26_area_button([arrows, 0, 0, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 1, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 4, 0]);
        } else {
            tel26_area_text([arrows, 0, 0, 0x30, job_colour(JOB_COLOUR_317E), 0x2c, 1, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32]);
        }
        comp_lang_format()(word.as_mut_ptr() as usize, label);
        comp_crt_format()(JOB_TEXT, 0x988088, word.as_ptr());
        if picked > 0 {
            tel26_area_button([arrows, 1, 0, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 1, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 5, 0]);
        } else {
            tel26_area_text([arrows, 1, 0, 0x30, job_colour(JOB_COLOUR_317E), 0x2c, 1, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32]);
        }
    }
    let (y, y2) = (*py, *py2);
    let rows = ((y2 - y - 0x10) / exe::font_get_height(2)) as u8;
    let area = tel26_add_area(current_human_display(), [0x6e, y as u32, 0x1d6, y2 as u32, 4, weights.as_ptr() as u32, rows as u32, 0, 2, 0, u32::MAX]);
    let mut k: i8 = 0;
    while (k as i32) < read_i8(award + TEAM_AWARD_SUBSTITUTES) as i32 + 0xb {
        let entry = rec + k as usize * 0x19;
        let row = k as u32;
        let number = read_i8(entry + 0x14);
        comp_crt_format()(JOB_TEXT, 0x98806c, if number != 0 { number as i32 } else { k as i32 + 1 });
        let number_colour = if k < 0xb { job_colour(JOB_COLOUR_BE04) } else { read_u32(TEAM_AWARD_SUBS_COLOUR) };
        tel26_area_text([area, 0, row, 0x30, number_colour, 0xc, 1, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32]);
        if read_i32(entry) > -1 {
            let person = staff_at(read_i32(entry) as isize);
            let club_id = read_i32(entry + 0x10);
            let club = if club_id > -1 { club_at(club_id as isize).addr() } else { 0 };
            let colour = if club != 0 && exe::human_manager_is_human_club(club, 0, 1, 0, 0) != 0 {
                job_colour(JOB_COLOUR_BDEC)
            } else if k < 0xb {
                job_colour(JOB_COLOUR_3184)
            } else {
                job_colour(JOB_COLOUR_317E)
            };
            exe::database_format_name_from_ids(read_u32(entry + 4), read_u32(entry + 8), read_u32(entry + 0xc), JOB_TEXT as *mut u8, 0x7d0, 1, read_u32(entry));
            if exe::database_staff_matches_names(person, read_u32(entry + 4), read_u32(entry + 8), read_u32(entry + 0xc), 0usize) != 0 {
                tel26_area_button([area, 2, row, 1, 0, 1, 2, colour, JOB_TEXT as u32, 0x7d7, person.addr() as u32]);
            } else {
                tel26_area_text([area, 2, row, 1, 0, 1, 2, colour, JOB_TEXT as u32]);
            }
            if club != 0 {
                tel26_area_button([area, 3, row, 1, 0, 1, 2, job_colour(JOB_COLOUR_BDEC), (club + club::SHORT_NAME) as u32, 0x7d5, club as u32]);
            }
        }
        k = k.wrapping_add(1);
    }
    let apply: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8b0730usize);
    apply(award, formation);
    award_team_pitch(rec, formation, (left, top, right, bottom));
    exe::drop_llist(list);
    exe::crt_operator_delete(list);
}

/// Comparer for `award_screen_draw_team_award`: side by side over the display list, the two y bounds and the dark blue
/// made once.
pub(crate) unsafe extern "C" fn compare_award_screen_draw_team_award(py: *mut i32, py2: *mut i32) {
    let o = original_fn::<unsafe extern "C" fn(*mut i32, *mut i32)>(%%I5%%);
    compare_match_screen_build("award screen (team award)", &[(py as usize, 4), (py2 as usize, 4), (TEAM_AWARD_SUBS_COLOUR, 4)],
        || o(py, py2), || award_screen_draw_team_award(py, py2));
}

/// The eleven of a team award record on the pitch (a part of `award_screen_draw_team_award`, see there).
unsafe fn award_team_pitch(rec: usize, formation: usize, (left, top, right, bottom): (i32, i32, i32, i32)) {
    let central: unsafe extern "thiscall" fn(usize, u32, usize) -> u32 = core::mem::transmute(0x5b2e80usize);
    let point: unsafe extern "C" fn(u32, i32, i32, i32, i32, *mut i32, *mut i32, u32) -> u32 = core::mem::transmute(0x8ad780usize);
    for slot in 0..0xbu32 {
        let entry = rec + slot as usize * 0x19;
        if read_i32(entry) <= -1 {
            continue;
        }
        let person = staff_at(read_i32(entry) as isize);
        let club_id = read_i32(entry + 0x10);
        let club = if club_id > -1 { club_at(club_id as isize).addr() } else { 0 };
        let wide = central(formation, exe::formation_get_slot_position(formation, slot, 0) as u32, 0);
        let (mut x, mut y) = (0i32, 0i32);
        point(exe::formation_get_slot_position(formation, slot, 0) as u32, left, top, right, bottom, &mut x, &mut y, wide);
        let (mut shirt, mut text) = (0u32, 0u32);
        if club != 0 {
            exe::database_get_club_kit_colours(club, 1, &mut shirt, &mut text);
        } else {
            shirt = job_colour(JOB_COLOUR_3184);
            text = job_colour(JOB_COLOUR_BE04);
        }
        let number = read_i8(entry + 0x14);
        comp_crt_format()(JOB_TEXT, 0x98806c, if number != 0 { number as i32 } else { slot as i32 + 1 });
        tel26_add_item(current_human_display(), [1, (x - 0xd) as u32, (y - 0x12) as u32, (x + 0xd) as u32, (y + 8) as u32, 0, 0, 0x10030,
            text, text, 0, 1, shirt, JOB_TEXT as u32, 0, 0, 0, u32::MAX]);
        exe::database_format_name_from_ids(read_u32(entry + 4), read_u32(entry + 8), read_u32(entry + 0xc), JOB_TEXT as *mut u8, 0x7d0, 5, read_u32(entry));
        let rect = [(x - 0x23) as u32, (y + 6) as u32, (x + 0x23) as u32, (y + 0x10) as u32];
        if exe::database_staff_matches_names(person, read_u32(entry + 4), read_u32(entry + 8), read_u32(entry + 0xc), 0usize) != 0 {
            tel26_add_button([rect[0], rect[1], rect[2], rect[3], 1, 0, 0xc, 1, job_colour(JOB_COLOUR_BDEC), JOB_TEXT as u32, 0x7d7, person.addr() as u32]);
        } else {
            comp_text_item([rect[0], rect[1], rect[2], rect[3], 1, 0, 0xc, 1, job_colour(JOB_COLOUR_BDEC), JOB_TEXT as u32, 1]);
        }
    }
}

/// Rule: a click on the award screen: another award of the menu (reopens the screen on its staff competition), another
/// award of the View popup (parameter 0, rebuilt), the nominations, the team award's older or newer team; the rest
/// goes to the menu bar.
///
/// `award_screen_handle_action(widget)`: the id of the clicked item (`tel26_item_id`): 1, the item's award: its staff
/// competition (0: the award_screens.cpp 90 box) opens a new award screen (as `award_screens_open_award_screen`), −4;
/// 2, parameter 0 = the item's value (0 with no widget), −11 (rebuild); 3, `award_screens_open_nominations_screen` of
/// the award, −4. Otherwise a team award (kind 0) takes id 4 (parameter 1 + 1, the older team) and 5 (− 1), −11; the
/// rest (and every other click of a year or month award) goes to `menubar_handle_command`, else
/// `menubar_handle_object_link`, whose answer comes back.
/// Defect of the original: an award of another kind (none exists) returns a stack word instead of asking the menu bar.
/// Original: `award_screen_handle_action` (0x416960, award_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn award_screen_handle_action(widget: u32) -> i32 {
    let award = get_screen_param(0);
    let item = widget as i16;
    match tel26_item_id(item) {
        1 => {
            let staff_comp = read_usize(tel26_item_value(item) as usize + AWARD_STAFF_COMP);
            award_screens_open_award_screen(staff_comp);
            return -4;
        }
        2 => {
            exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, 0, tel26_item_value(item) as usize, 0);
            return -11;
        }
        3 => {
            award_screens_open_nominations_screen(award);
            return -4;
        }
        _ => {}
    }
    if read_u8(award + AWARD_KIND) == 0 {
        let year = get_screen_param(1);
        match tel26_item_id(item) {
            4 => {
                set_screen_param(1, year.wrapping_add(1));
                return -11;
            }
            5 => {
                set_screen_param(1, year.wrapping_sub(1));
                return -11;
            }
            _ => {}
        }
    }
    let r = exe::menubar_handle_command(widget);
    if r != 0 {
        return r;
    }
    exe::menubar_handle_object_link(widget)
}

/// Comparer for `award_screen_handle_action`: side by side over the parameters of the current screen and the answer for
/// the clicks that only write parameters (2, and 4 and 5 of a team award); the others (a screen opened, the menu bar)
/// only the original.
pub(crate) unsafe extern "C" fn compare_award_screen_handle_action(widget: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I6%%);
    let award = get_screen_param(0);
    let id = tel26_item_id(widget as i16);
    let params_only = id == 2 || (award != 0 && read_u8(award + AWARD_KIND) == 0 && (id == 4 || id == 5));
    if !params_only {
        return o(widget);
    }
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&screen_param_regions(), || o(widget), || award_screen_handle_action(widget));
    SCOREBOARD.record(equal && ro == rr, || format!("award screen (click {}): answer {} x {}, differences in {:?}", id, ro, rr, first_differences(&mo, &mr)));
    ro
}

/// `News::find_item_by_id` (0x786b70, thiscall, ret 0xc): the news item `id` of the manager into `out`.
unsafe fn award_find_news_item(manager: usize, id: u32, out: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize, u32, usize) = core::mem::transmute(0x786b70usize);
    f(read_usize(G_NEWS), manager, id, out);
}

/// Rule: the news item's "Nominations" button opens the nominations of the award it names.
///
/// `award_screens_open_nominations_from_news(manager, id)`: a local NewsItem (`NewsItem::NewsItem(0, 0)`) gets the
/// manager's news item `id` (`News::find_item_by_id`); its slot 0 is the nation award object's index and slot 1 the
/// award's index in it (`NewsItem::get_slot`); `award_screens_open_nominations_screen` of that award.
/// Original: `award_screens_open_nominations_from_news` (0x416cb0, award_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn award_screens_open_nominations_from_news(manager: usize, id: u32) {
    let mut item = [0usize; 0x38];
    let item = item.as_mut_ptr() as usize;
    exe::new_newsitem(item, 0, 0);
    award_find_news_item(manager, id, item);
    let nation = exe::newsitem_get_slot(item, 0);
    let index = exe::newsitem_get_slot(item, 1);
    award_screens_open_nominations_screen(award_in(award_nation_awards(nation), index as isize));
}

/// Comparer for `award_screens_open_nominations_from_news`: only the original runs (it opens a screen); the opener's
/// own comparer checks the parameters.
pub(crate) unsafe extern "C" fn compare_award_screens_open_nominations_from_news(manager: usize, id: u32) {
    // simplification: opening a screen does not run twice; the nominations opener compares its plan
    original_fn::<unsafe extern "C" fn(usize, u32)>(%%I7%%)(manager, id)
}

/// `AWARD_SHORTLIST::get_vote_status` (0x418140, thiscall, ret 4): the entry the manager voted for, −1 when he may
/// vote, −2 when he may not (ax).
unsafe fn award_vote_status(shortlist: usize, manager: usize) -> i16 {
    let f: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x418140usize);
    f(shortlist, manager) as i16
}

/// Rule: opens an award's nominations; the current manager may vote when he has not voted yet.
///
/// `award_screens_open_nominations_screen(award)`: without an award the award_screens.cpp 964 box. The vote flag is
/// whether `AWARD_SHORTLIST::get_vote_status` of the award's shortlist for the current slot's manager is −1; `open_screen`
/// with `award_nominations_screen_draw` and `award_nominations_screen_handle_action` (flag = the vote flag); when it
/// opens, parameter 0 the award and 1 the vote flag.
/// Original: `award_screens_open_nominations_screen` (0x416d50, award_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn award_screens_open_nominations_screen(award: usize) {
    if award == 0 {
        award_screens_error_box(0x3c4);
        return;
    }
    let may_vote = (award_vote_status(read_usize(award + AWARD_SHORTLIST), tel26_slot_manager().addr()) as u16 == 0xffff) as u32;
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x416e40usize, 0x4172f0usize, may_vote, 0usize, 0) == 0 {
        return;
    }
    covered!("tel27_1 nominations opened");
    exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, 0, award, 0);
    exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, 1, may_vote as usize, 0);
}

/// Comparer for `award_screens_open_nominations_screen`: only the original runs (it opens a screen); when a new
/// nominations screen opened, its parameters 0 and 1 against the plan.
pub(crate) unsafe extern "C" fn compare_award_screens_open_nominations_screen(award: usize) {
    let before = screen_nodes_built_by(0x416e40);
    let plan = if award != 0 {
        [award, (award_vote_status(read_usize(award + AWARD_SHORTLIST), tel26_slot_manager().addr()) as u16 == 0xffff) as usize]
    } else {
        [0, 0]
    };
    original_fn::<unsafe extern "C" fn(usize)>(%%I8%%)(award);
    if award != 0 && screen_newly_opened(&before, 0x416e40) {
        compare_opened_params("nominations screen (open)", &plan);
    }
}

/// Rule: the nominations screen lists the award's shortlist; while the manager may vote, a click on a nominee is his
/// vote, and Back and Next close the vote.
///
/// `award_nominations_screen_draw()`: parameter 0 the award, 1 the vote flag. The background and title as the award
/// screen's (`award_screen_background`, `award_title_colours`, title = the staff competition's name), the menu bar (4),
/// the subtitle "Nominations" (flag 1). A 3-column area (0x6e, 0x91)-(0x30c, 0x217), weights 3/2/2, rows 0x176 / font
/// 3 height, with one row per shortlist entry (14 bytes: +0 person id, +4 club id) with a person: his name ("  <name>",
/// `database_format_staff_name`) in yellow (0xaebde8) when he is the manager's vote, highlighted when a human runs his
/// club, else plain; a button with id 1 (the vote) while the flag is set and the manager may still vote, else a link
/// 0x7d7; his club (a link 0x7d5) and his nation (the national team's link 0x7d5, 0xaebdd4). With the vote flag, a
/// 2-column area (0x64, 0x22b)-(0x316, 0x24e) (weights 3/1): "Back" (id 2) and "Next" (id −3; grey text when there is
/// no next screen); without it the usual Back and Next buttons.
/// Original: `award_nominations_screen_draw` (0x416e40, award_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn award_nominations_screen_draw() {
    let weights: [u8; 3] = [3, 2, 2];
    let back_weights: [u8; 2] = [3, 1];
    let award = get_screen_param(0);
    let may_vote = get_screen_param(1);
    award_screen_background(award);
    let group = award_nation_awards(read_u32(award + AWARD_NATION_INDEX));
    let (mut first, mut second) = (0u32, 0u32);
    award_title_colours(award, group, &mut first, &mut second);
    exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, second, first, read_usize(award + AWARD_STAFF_COMP) + 4);
    exe::menubar_build(4, 0);
    comp_lang_format()(JOB_TEXT, 0x987ff8);
    exe::screenmanager_add_screen_subtitle(G_SCREEN_MANAGER, JOB_TEXT, 1);
    let rows = (0x176 / exe::font_get_height(3)) as u8;
    let area = tel26_add_area(current_human_display(), [0x6e, 0x91, 0x30c, 0x217, 3, weights.as_ptr() as u32, rows as u32, 0, 2, 0, u32::MAX]);
    let shortlist = read_usize(award + AWARD_SHORTLIST);
    let mut row: u32 = 0;
    let mut i: i32 = 0;
    while i < read_i16(read_usize(award + AWARD_SHORTLIST) + 4) as i32 {
        let entry = read_usize(read_usize(award + AWARD_SHORTLIST)) + i as usize * 0xe;
        if read_i32(entry) > -1 {
            let person = staff_at(read_i32(entry) as isize);
            let club_id = read_i32(entry + 4);
            let club = if club_id > -1 { club_at(club_id as isize).addr() } else { 0 };
            let colour = if i == award_vote_status(shortlist, tel26_slot_manager().addr()) as i32 {
                job_colour(JOB_COLOUR_BDE8)
            } else if club != 0 && exe::human_manager_is_human_club(club, 0, 1, 0, 0) != 0 {
                job_colour(JOB_COLOUR_BDEC)
            } else {
                job_colour(JOB_COLOUR_3184)
            };
            let mut name = [0u8; 0xc8];
            exe::database_format_staff_name(person, name.as_mut_ptr() as usize, 0xc8, 1);
            comp_crt_format()(JOB_TEXT, 0x987ff0, name.as_ptr());
            let vote = may_vote != 0 && award_vote_status(read_usize(award + AWARD_SHORTLIST), tel26_slot_manager().addr()) as u16 == 0xffff;
            tel26_area_button([area, 0, row, 1, 0, 1, 3, colour, JOB_TEXT as u32, if vote { 1 } else { 0x7d7 }, person.addr() as u32]);
            if club != 0 {
                tel26_area_button([area, 1, row, 1, 0, 1, 2, job_colour(JOB_COLOUR_BDEC), (club + club::SHORT_NAME) as u32, 0x7d5, club as u32]);
            }
            let nation = read_usize(person.addr() + staff::NATION);
            if nation != 0 {
                let team = exe::database_get_national_team(nation);
                tel26_area_button([area, 2, row, 1, 0, 1, 2, job_colour(JOB_COLOUR_BDD4), (nation + nation::SHORT_NAME) as u32, 0x7d5, team as u32]);
            }
            row += 1;
        }
        i += 1;
    }
    if may_vote == 0 {
        tel26_back_next();
        return;
    }
    covered!("tel27_1 nominations vote buttons");
    let buttons = tel26_add_area(current_human_display(), [0x64, 0x22b, 0x316, 0x24e, 2, back_weights.as_ptr() as u32, 1, 0, 1, 0, u32::MAX]);
    comp_lang_format()(JOB_TEXT, 0x9880a8);
    tel26_area_button([buttons, 0, 0, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 3, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 2, 0]);
    comp_lang_format()(JOB_TEXT, 0x9880a0);
    if exe::screenmanager_is_forward_unavailable(G_SCREEN_MANAGER) == 0 {
        tel26_area_button([buttons, 1, 0, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 3, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, (-3i32) as u32, 0]);
    } else {
        tel26_area_text([buttons, 1, 0, 0x30, job_colour(JOB_COLOUR_317E), 0x2c, 3, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32]);
    }
}

/// Comparer for `award_nominations_screen_draw`: side by side over the display list.
pub(crate) unsafe extern "C" fn compare_award_nominations_screen_draw() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I9%%);
    if get_screen_param(0) == 0 {
        return o();
    }
    compare_match_screen_build("nominations screen", &[], || o(), || award_nominations_screen_draw());
}

/// `AWARD_MANAGER::set_pending_vote` (0x4148e0, thiscall, ret 0x10): the human's vote (a player, or 0 to clear it)
/// for the award at (nation index, award index).
unsafe fn award_set_pending_vote(human: usize, player: usize, nation: u32, index: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(0x4148e0usize);
    f(read_usize(G_AWARD_MANAGER), human, player, nation, index);
}

/// Rule: a click on the nominations screen: a nominee is the manager's vote (and opens his profile), "Back" clears the
/// vote; the rest goes to the menu bar.
///
/// `award_nominations_screen_handle_action(widget)`: id 1: `AWARD_MANAGER::set_pending_vote(slot manager, the item's
/// person, AWARD_NATION_INDEX, AWARD_INDEX)` and `staff_screens_open_staff_profile(person, 1, 1, 0, 0)`, −4; id 2: the
/// vote cleared (person 0), −9; else `menubar_handle_command`, then `menubar_handle_object_link`.
/// Original: `award_nominations_screen_handle_action` (0x4172f0, award_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn award_nominations_screen_handle_action(widget: u32) -> i32 {
    let award = get_screen_param(0);
    let item = widget as i16;
    match tel26_item_id(item) {
        1 => {
            let person = tel26_item_value(item) as usize;
            award_set_pending_vote(tel26_slot_manager().addr(), person, read_u32(award + AWARD_NATION_INDEX), read_u32(award + AWARD_INDEX));
            let profile: unsafe extern "C" fn(usize, u32, u32, u32, u32) = core::mem::transmute(0x8702a0usize);
            profile(person, 1, 1, 0, 0);
            -4
        }
        2 => {
            award_set_pending_vote(tel26_slot_manager().addr(), 0, read_u32(award + AWARD_NATION_INDEX), read_u32(award + AWARD_INDEX));
            -9
        }
        _ => {
            let r = exe::menubar_handle_command(widget);
            if r != 0 {
                return r;
            }
            exe::menubar_handle_object_link(widget)
        }
    }
}

/// Comparer for `award_nominations_screen_handle_action`: only the original runs (the vote goes to the award manager
/// and the profile opens a screen); for id 2 the human's pending vote is checked cleared afterwards [inference: the
/// award manager keeps it per human].
pub(crate) unsafe extern "C" fn compare_award_nominations_screen_handle_action(widget: u32) -> i32 {
    // simplification: the vote, the profile and the menu bar do not run twice
    original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I10%%)(widget)
}
