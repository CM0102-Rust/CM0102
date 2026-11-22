// ---------- batch %%LEVA%%: the printouts (5): a club's fixtures and results with the scorers (printouts.cpp) ----------

/// The fixtures' column titles (0xa78e74) and the dashes under them (0xa78e04).
const PRINTOUT_FIXTURE_TITLES: usize = 0xa78e74;
const PRINTOUT_FIXTURE_DASHES: usize = 0xa78e04;
/// The longest scorers text on one line: past it the line is written and the scorers go on under the column.
const PRINTOUT_SCORERS_WIDTH: usize = 50;
/// The match statistics record (`Match_Stats`, `MATCH_STATS_RECORD_SIZE` bytes) the fixtures printout reads each
/// played match into: only its two team blocks (`match_record::HOME_TEAM`, 0x625 bytes each) are constructed
/// (0x453c30, which only gives each block's 20 rows the default date) and destroyed (0x453c10, empty): no global
/// effect, so the comparer lets the original and the copy each build their own.
const PRINTOUT_TEAM_BLOCK_SIZE: u32 = 0x625;
const PRINTOUT_TEAM_BLOCK_CONSTRUCTOR: usize = 0x453c30;
const PRINTOUT_TEAM_BLOCK_DESTRUCTOR: usize = 0x453c10;
/// The kinds of a match record's events the scorers list (`match_record::EVENT_KIND`).
const PRINTOUT_EVENT_GOAL: u8 = 1;
const PRINTOUT_EVENT_OWN_GOAL: u8 = 2;
const PRINTOUT_EVENT_PENALTY_GOAL: u8 = 3;
/// The number of event slots of a match record (`match_record::EVENTS`, 4 bytes each; side 0xff ends the list).
const PRINTOUT_EVENT_SLOTS: usize = 50;

/// A scorer's entry in `t`: the player's name `name` with his `count` goals (`pens` of them penalties), or his own
/// goals; an impossible count gives "???".
unsafe fn printout_scorer_entry(t: usize, name: usize, own_goal: bool, count: i8, pens: i8) {
    let (count, pens) = (count as i32, pens as i32);
    if own_goal {
        match count {
            1 => { comp_lang_format()(t, 0xa78d7c, name); }
            c if c > 1 => { comp_lang_format()(t, 0xa78d58, name, c); }
            _ => printout_copy(t, 0xa14f84),
        }
        return;
    }
    if count == 1 {
        match pens {
            0 => printout_copy(t, name),
            1 => { comp_lang_format()(t, 0xa78d40, name); }
            _ => printout_copy(t, 0xa14f84),
        }
    } else if count >= 2 {
        if pens == 0 {
            comp_lang_format()(t, 0xa78d24, name, count);
        } else if pens == 1 {
            comp_lang_format()(t, 0xa78d00, name, count);
        } else if pens == count {
            comp_lang_format()(t, 0xa78ce0, name, count);
        } else if pens >= 2 {
            comp_lang_format()(t, 0xa78cac, name, count, pens);
        } else {
            printout_copy(t, 0xa14f84);
        }
    } else {
        printout_copy(t, 0xa14f84);
    }
}

/// The scorers of the club's side in the match record `record` (the club `side` 0 home, 1 away) appended to
/// [`PRINTOUT_LINE`], breaking lines into the file: see `printouts_write_club_fixtures`.
unsafe fn printout_scorers(record: usize, side: u8, file: usize) {
    let mut scorers = [0u8; 0x400];
    let mut name = [0u8; 0x100];
    let mut entry = [0u8; 0x200];
    let (s, n, t) = (scorers.as_mut_ptr() as usize, name.as_mut_ptr() as usize, entry.as_mut_ptr() as usize);
    let indent = printout_text_length(PRINTOUT_LINE) as i16;
    let mut done = [0u32; PRINTOUT_EVENT_SLOTS];
    let mut cut = 0i16;
    for k in 0..PRINTOUT_EVENT_SLOTS {
        let event = record + k * 4;
        let who = read_u8(event + match_record::EVENTS);
        if who == 0xff {
            break;
        }
        if done[k] != 0 {
            continue;
        }
        let kind = read_u8(event + match_record::EVENT_KIND);
        let own_goal = match kind {
            PRINTOUT_EVENT_GOAL | PRINTOUT_EVENT_PENALTY_GOAL if who == side => false,
            PRINTOUT_EVENT_OWN_GOAL if who as i8 as i32 == 1 - side as i8 as i32 => true,
            _ => continue,
        };
        let player = read_u8(event + match_record::EVENT_PLAYER);
        let (mut count, mut pens) = (0i8, 0i8);
        for (j, flag) in done.iter_mut().enumerate() {
            let other = record + j * 4;
            let other_kind = read_u8(other + match_record::EVENT_KIND);
            let same = if own_goal { other_kind == PRINTOUT_EVENT_OWN_GOAL } else { other_kind == PRINTOUT_EVENT_GOAL || other_kind == PRINTOUT_EVENT_PENALTY_GOAL };
            if read_u8(other + match_record::EVENTS) == who && read_u8(other + match_record::EVENT_PLAYER) == player && same {
                count = count.wrapping_add(1);
                if other_kind == PRINTOUT_EVENT_PENALTY_GOAL {
                    pens = pens.wrapping_add(1);
                }
                *flag = 1;
            }
        }
        if own_goal {
            covered!("tel25_9 own goal scorer");
        }
        exe::match_events_format_player_name(record, player as i8 as i32, who as i8 as i32, n as *mut u8, 0xc8, 0, 0, 2);
        printout_scorer_entry(t, n, own_goal, count, pens);
        if read_u8(s) != 0 {
            printout_append(s, 0xa78d54);
        }
        printout_append(s, t);
        if printout_text_length(s) > PRINTOUT_SCORERS_WIDTH {
            covered!("tel25_9 scorers line break");
            write_u8((s as isize + cut as isize) as usize, 0);
            printout_append(PRINTOUT_LINE, s);
            printout_write_row(file);
            let mut end = 0i16;
            if indent > 0 {
                core::ptr::write_bytes(PRINTOUT_LINE as *mut u8, b' ', indent as usize);
                end = indent;
            }
            write_u8((PRINTOUT_LINE as isize + end as isize) as usize, 0);
            printout_copy(s, t);
        }
        cut = printout_text_length(s) as i16;
    }
    printout_append(PRINTOUT_LINE, s);
}

/// Rule: a club's fixtures and results over the printout's period, with the scorers.
///
/// `write_club_fixtures(club, reserves, file)`. The heading " <%s - season> Fixtures" (the club's season of the
/// printout's start year, 0x669630), the column titles and their dashes. The club's fixtures between
/// [`PRINTOUT_PERIOD_START`] and [`PRINTOUT_PERIOD_END`] (`FIXTURE_MANAGER::find_fixtures` by the club's id, any
/// competition and kind, side mark `reserves` = 0) fill a local list; a match statistics record is allocated with its
/// two team blocks constructed (failing: the printouts.cpp 706 box, the list destroyed and 0). Each fixture is a row:
/// its short date (8 wide); the opposition (the away club when the club is at home, else the home club) by short
/// name (20 wide); the opposition's nation by its three letters when the club is not a national team and the two
/// nations differ, else empty (3 wide); the venue H, A or N (`comp_util_get_fixture_venue_side` 1, 2, other; 3 wide);
/// the competition's round label (form 3, 30 wide); the result from the club's side, after extra time when there is
/// one (`fixture::HOME_GOALS_EXTRA_TIME` not 0xff), else at full time, "P-P" for a postponed match (0xfe) and "---"
/// when not played (5 wide). A played match (full-time goals not negative) with a statistics slot whose entry
/// matches the fixture's clubs and date (`Match_Stats::fixture_entry_matches`) and reads
/// (`Match_Stats::read_match_record`) gets its attendance (6 wide) and the scorers of the club's side (the club is
/// side 1 when its id is not the record's home club's): for each of the 50 events up to the first with side 0xff,
/// not yet counted, a goal or penalty goal of the club's side or an own goal of the other side counts every event of
/// that kind (goal and penalty goal together) by the same player and side, marking them counted; the player's name
/// (`match_events_format_player_name`, full form) gives "name", "name pen", "name n", "name n (1 pen)", "name n
/// pens", "name n (p pens)", "name og" or "name n ogs !", joined by ", ". When the scorers text passes 50
/// characters it is cut where it stood before the last scorer, written after the row, and the line restarts with
/// spaces up to the scorers column and the last scorer. Any other fixture shows "-----" (6 wide) as its attendance.
/// The record is destroyed and the section ends (two newlines and a lone space). Returns 1.
/// Original: `printouts_write_club_fixtures` (0x7d69f0, printouts.cpp, cdecl)
pub(crate) unsafe extern "C" fn printouts_write_club_fixtures(club: usize, reserves: u32, file: usize) -> u32 {
    let mut list = [0u8; 16];
    let l = list.as_mut_ptr() as usize;
    exe::new_llist(l);
    exe::fixture_manager_find_fixtures(read_usize(G_FIXTURE_MANAGER), read_u16(PRINTOUT_PERIOD_START) as u32, read_u16(PRINTOUT_PERIOD_START + 2) as u32,
                                       read_u16(PRINTOUT_PERIOD_END) as u32, read_u16(PRINTOUT_PERIOD_END + 2) as u32, read_u32(club), l, u32::MAX, 0xffff_fffe, (reserves == 0) as u32);
    let mut text = [0u8; 0x200];
    let t = text.as_mut_ptr() as usize;
    printout_club_season(read_u16(PRINTOUT_PERIOD_START + 2) as u32, club, t, 0xc8);
    comp_lang_format()(PRINTOUT_LINE, 0xa78ed8, t);
    printout_write_heading(file);
    printout_write_titles(file, PRINTOUT_FIXTURE_TITLES, 0, PRINTOUT_FIXTURE_DASHES);
    let record = exe::crt_operator_new(MATCH_STATS_RECORD_SIZE);
    if record == 0 {
        tel26_error_box(PRINTOUTS_CPP, 0x2c2);
        exe::drop_llist(l);
        return 0;
    }
    exe::crt_vector_constructor_iterator(record + match_record::HOME_TEAM, PRINTOUT_TEAM_BLOCK_SIZE, 2, PRINTOUT_TEAM_BLOCK_CONSTRUCTOR, PRINTOUT_TEAM_BLOCK_DESTRUCTOR);
    covered!("tel25_9 club fixtures");
    printout_list_rewind(l);
    loop {
        let game = exe::llist_get_next(l);
        if game == 0 {
            break;
        }
        write_u8(PRINTOUT_LINE, 0);
        let mut date = [0u8; 8];
        let d = exe::new_date(date.as_mut_ptr() as usize, read_u16(game + fixture::DAY) as u32, read_u16(game + fixture::YEAR) as u32);
        printout_short_date(d, t, 0xc8, 0xff);
        printout_column(t, 8);
        let home = read_usize(game + fixture::HOME_CLUB) == club;
        let opposition = read_usize(game + if home { fixture::AWAY_CLUB } else { fixture::HOME_CLUB });
        printout_copy(t, opposition + club::SHORT_NAME);
        printout_column(t, 0x14);
        let nation = read_usize(opposition + club::NATION);
        if exe::database_is_national_team(club) == 0 && read_usize(club + club::NATION) != nation {
            printout_copy(t, nation + nation::THREE_LETTER_NAME);
        } else {
            write_u8(t, 0);
        }
        printout_column(t, 3);
        let venue = match exe::comp_util_get_fixture_venue_side(club, game) {
            1 => 0xa78de8,
            2 => 0xa78dcc,
            _ => 0xa78db0,
        };
        comp_lang_format()(t, venue, PRINTOUT_NO_TEXT);
        printout_column(t, 3);
        exe::comp_util_format_fixture_round_label(game, 3, t, 0xc8);
        printout_column(t, 0x1e);
        let extra = read_u8(game + fixture::HOME_GOALS_EXTRA_TIME);
        let goals = read_u8(game + fixture::HOME_GOALS);
        let score = if extra != 0xff {
            Some((extra, read_u8(game + fixture::AWAY_GOALS_EXTRA_TIME)))
        } else if goals == 0xfe || goals == 0xff {
            None
        } else {
            Some((goals, read_u8(game + fixture::AWAY_GOALS)))
        };
        match score {
            Some((h, a)) => {
                let (first, second) = if home { (h, a) } else { (a, h) };
                comp_lang_format()(t, 0x990bb4, first as i8 as i32, second as i8 as i32);
            }
            None if goals == 0xfe => { comp_lang_format()(t, 0xa78d90, PRINTOUT_NO_TEXT); }
            None => printout_copy(t, 0x98f858),
        }
        printout_column(t, 5);
        let slot = read_i32(game + fixture::MATCH_STATS_SLOT);
        let read = goals as i8 > -1
            && slot > -1
            && exe::match_stats_fixture_entry_matches(read_usize(G_MATCH_STATS), slot, read_i32(game + fixture::HOME_CLUB_ID), read_i32(game + fixture::AWAY_CLUB_ID),
                                                      read_u16(game + fixture::DAY) as i32, read_u16(game + fixture::YEAR) as i32) != 0
            && exe::match_stats_read_match_record(read_usize(G_MATCH_STATS), slot as u32, record) != 0;
        if read {
            comp_crt_format()(t, 0x98c11c, read_i32(record + match_record::ATTENDANCE));
            printout_column(t, 6);
            let side = (read_u32(club) != read_u32(record + match_record::HOME_CLUB_ID)) as u8;
            printout_scorers(record, side, file);
        } else {
            printout_copy(t, 0x99c18c);
            printout_column(t, 6);
        }
        printout_write_row(file);
    }
    exe::crt_vector_destructor_iterator(record + match_record::HOME_TEAM, PRINTOUT_TEAM_BLOCK_SIZE, 2, PRINTOUT_TEAM_BLOCK_DESTRUCTOR);
    exe::crt_operator_delete(record);
    printout_end_section(file);
    exe::drop_llist(l);
    1
}

/// Comparer for `printouts_write_club_fixtures`: side by side over the text written (`compare_printout_text`); the
/// original and the copy each build and destroy their own match statistics record (no global effect).
pub(crate) unsafe extern "C" fn compare_printouts_write_club_fixtures(club: usize, reserves: u32, file: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, usize) -> u32>(%%I0%%);
    compare_printout_text(file, || format!("club fixtures printout ({:#x}, reserves {})", club, reserves),
                          |f| o(club, reserves, f), |f| printouts_write_club_fixtures(club, reserves, f))
}
