// ---------- batch %%LEVA%%: a played cup match (Cup.cpp) ----------

/// Exe routines that `cup_process_fixture` calls without an `exe::` wrapper (Rust hooked, called through the exe), and
/// the two Cup virtual slots past Comp that decide a match (Cup vtable 0x9693c8: 0x51c020 and 0x51f2f0).
pub(crate) mod cup_fixture_fns {
    pub(crate) const FIND_TREE_SLOT_FOR_FIXTURE: usize = 0x519c90; // Cup::find_tree_slot_for_fixture: thiscall(cup, fixture), ret 4
    pub(crate) const RECORD_TEAM_PROGRESS: usize = 0x521b30; // Cup::record_team_progress: thiscall(cup, team, round), ret 8
    pub(crate) const PAY_ROUND_PRIZE_MONEY: usize = 0x522240; // Cup::pay_round_prize_money: thiscall(cup, winner, loser, round, fixture), ret 0x10
    pub(crate) const APPLY_ROUND_RULES_TO_MATCH: usize = 0x521d60; // Cup::apply_round_rules_to_match: thiscall(cup, fixture, slot), ret 8
    pub(crate) const FIND_NEXT_FIXTURE: usize = 0x5ac920; // FIXTURE_MANAGER::find_next_fixture: thiscall, 11 arguments, ret 0x2c
    pub(crate) const DECIDE_FIXTURE_WINNER: usize = 0x98; // virtual: Cup::decide_fixture_winner(fixture), ret 4
    pub(crate) const DECIDE_TIE_WINNER: usize = 0x9c; // virtual: Cup::decide_tie_winner(tie, fixture), ret 8
    pub(crate) const CUP_CPP: usize = 0x9b55ac; // "E:\dev\CM3\cm3 00-01\cm3\code\comp\Cup.cpp"
    pub(crate) const DRAWN_GAME_TEXT: usize = 0x9b5678; // "CUP::process_fixture() - drawn game in cup (Round %d of %s)"
    pub(crate) const THIRD_MATCH_TEXT: usize = 0x9b5628; // "CUP::process_fixture() : third match should not have a replay (round %d in %s)"
}

/// Round record and tie offsets that only `cup_process_fixture` names.
pub(crate) mod cup_fixture_fields {
    /// i32 in the round record: paid to both clubs of every match of the round when above 0 [inference: a match fee].
    pub(crate) const ROUND_MATCH_PRIZE: usize = 0x5c;
    /// u16 in the round record: the tie rules of the second replay and of the deciding match after two legs (the first
    /// replay takes `cup_round::TWO_LEG_TIE_RULES`) [inference].
    pub(crate) const ROUND_LATER_TIE_RULES: usize = 0x11;
    /// i8 in the tie: the result marks (fixture `HOME_RESULT_MARK`/`AWAY_RESULT_MARK`) of side A (`cup_tie::HOME_TEAM`)
    /// and side B in the tie's last match (docs/MAPA_SAVE.md §6.1).
    pub(crate) const TIE_HOME_MARK: usize = 4;
    pub(crate) const TIE_AWAY_MARK: usize = 5;
    /// The tie's three result blocks (10 bytes each: the fixture's goals, from `fixture::HOME_GOALS`), by matches played.
    pub(crate) const TIE_RESULT_BLOCKS: [usize; 3] = [0x14, 0x1e, 0x28];
}

/// What the cup's decision (`cup_fixture_fns::DECIDE_FIXTURE_WINNER` / `DECIDE_TIE_WINNER`) returns.
pub(crate) mod cup_decision {
    pub(crate) const ERROR: i32 = -1;
    pub(crate) const DRAW: i32 = 0; // a single match drawn: a replay
    pub(crate) const HOME: i32 = 1; // the home side goes through
    pub(crate) const AWAY: i32 = 2; // the away side goes through
    pub(crate) const FIRST_LEG: i32 = 3; // a first leg played: the second leg is already in the calendar
    pub(crate) const LEVEL: i32 = 4; // level after two legs: a deciding match
}

/// The team outcome byte (`cup_team_entry::STATUS`) that `cup_process_fixture` writes.
pub(crate) mod cup_team_status {
    pub(crate) const WINNER: u8 = 1;
    pub(crate) const RUNNER_UP: u8 = 2;
    pub(crate) const THIRD: u8 = 3;
    pub(crate) const FOURTH: u8 = 4;
    pub(crate) const OUT: u8 = 0xff;
}

/// The address of virtual slot `slot` of the cup.
unsafe fn cup_fixture_virtual(cup: Cup, slot: usize) -> usize {
    read_usize(read_usize(cup.addr()) + slot)
}

/// The cup's decision on a played match: the tie rule (`cup_fixture_fns::DECIDE_TIE_WINNER`) in a two-legged round
/// (`cup_round::LEGS` 2) before its third match, otherwise the match rule (`cup_fixture_fns::DECIDE_FIXTURE_WINNER`).
pub(crate) unsafe fn cup_fixture_decision(cup: Cup, round: i32, tie: usize, fixture: usize, played: i32) -> i32 {
    if read_u8(round_record(cup, round) + cup_round::LEGS) == 2 && played != 2 {
        let f: unsafe extern "thiscall" fn(usize, usize, usize) -> i32 = core::mem::transmute(cup_fixture_virtual(cup, cup_fixture_fns::DECIDE_TIE_WINNER));
        f(cup.addr(), tie, fixture)
    } else {
        let f: unsafe extern "thiscall" fn(usize, usize) -> i32 = core::mem::transmute(cup_fixture_virtual(cup, cup_fixture_fns::DECIDE_FIXTURE_WINNER));
        f(cup.addr(), fixture)
    }
}

/// The tie's result marks in side order: the fixture's (`HOME_RESULT_MARK`, `AWAY_RESULT_MARK`) when its home club is the
/// club of side A (`cup_tie::HOME_TEAM`), otherwise swapped.
pub(crate) unsafe fn cup_tie_marks(cup: Cup, tie: usize, fixture: usize) -> (u8, u8) {
    let (home, away) = (read_u8(fixture + fixture::HOME_RESULT_MARK), read_u8(fixture + fixture::AWAY_RESULT_MARK));
    if read_usize(fixture + fixture::HOME_CLUB) == read_usize(team_entry(cup, read_i16(tie + cup_tie::HOME_TEAM) as i32)) {
        (home, away)
    } else {
        (away, home)
    }
}

/// The tie's winner and loser (team indices) after decision `r` (`cup_decision::HOME`/`AWAY`): side A
/// (`cup_tie::HOME_TEAM`) wins when it is the fixture's home club and the home side went through, or when it is not and
/// the away side went through; otherwise side B (`cup_tie::AWAY_TEAM`).
pub(crate) unsafe fn cup_tie_sides(cup: Cup, tie: usize, fixture: usize, r: i32) -> (i16, i16) {
    let (a, b) = (read_i16(tie + cup_tie::HOME_TEAM), read_i16(tie + cup_tie::AWAY_TEAM));
    let a_at_home = read_usize(team_entry(cup, a as i32)) == read_usize(fixture + fixture::HOME_CLUB);
    if (r == cup_decision::HOME) == a_at_home {
        (a, b)
    } else {
        (b, a)
    }
}

/// The third-place round: the last round when its code is `round_name::THIRD_PLACE_PLAYOFF`, otherwise −1.
pub(crate) unsafe fn cup_third_place_round(cup: Cup) -> i32 {
    let n = cup.round_count() as i32;
    if read_i16(round_record(cup, n - 1) + cup_round::ROUND_CODE) == round_name::THIRD_PLACE_PLAYOFF {
        n - 1
    } else {
        -1
    }
}

/// Whether every one of the cup's `total` teams (the low word, signed) has an outcome (`cup_team_entry::STATUS` ≠ 0),
/// with `status(k)` the outcome of team k: 1, or 0 at the first team without one. The value of `Cup::run_by_stages`
/// after a final.
pub(crate) unsafe fn cup_all_teams_decided(total: i32, status: impl Fn(i16) -> u8) -> u8 {
    let n = total as i16;
    let mut k: i16 = 0;
    while k < n {
        if status(k) == 0 {
            return 0;
        }
        k += 1;
    }
    1
}

/// A one-round cup without legs whose tie has had its replays (`cup_tie::REPLAYS_PLAYED` ≥ `cup_round::SINGLE_LEG_FLAG`):
/// a draw (or a level tie) ends it there.
pub(crate) unsafe fn cup_replays_exhausted(cup: Cup, round: i32, tie: usize) -> bool {
    let r = round_record(cup, round);
    cup.round_count() == 1 && (read_u8(r + cup_round::LEGS) as i8) < 2 && read_i8(tie + cup_tie::REPLAYS_PLAYED) >= read_i8(r + cup_round::SINGLE_LEG_FLAG)
}

/// Writes `text` (`"%d"` the round, `"%s"` the competition's name) into the news text buffer, then the error box at
/// `line`.
unsafe fn cup_fixture_error(cup: Cup, text: usize, round: i32, line: u32) {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    format(NEWS_TEXT_BUFFER, text, round, read_usize(cup.addr() + league::COMPETITION) + competition::NAME);
    show_error_at(cup_fixture_fns::CUP_CPP, line);
}

/// Calls the cup's virtual `comp_vtable::ANNOUNCE_TEAM_STATUS` (`Cup::add_team_result_news`) for team `team`'s club with
/// outcome `status`, the cup's `stage_index()`, the fixture and the caller's argument.
unsafe fn cup_announce_team(cup: Cup, team: i16, status: u32, fixture: usize, a2: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, u32) -> u32 = core::mem::transmute(cup_fixture_virtual(cup, comp_vtable::ANNOUNCE_TEAM_STATUS));
    f(cup.addr(), read_usize(team_entry(cup, team as i32)), status, cup.stage_index() as u32, fixture, a2);
}

/// `Cup::record_team_progress(team, round)` through the exe.
unsafe fn cup_record_progress(cup: Cup, team: i16, round: i16) {
    let f: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(cup_fixture_fns::RECORD_TEAM_PROGRESS);
    f(cup.addr(), team as i32 as u32, round as u16 as u32);
}

/// The time slice of a match on `date`: 1 on the weekday codes 5 and 6 (the weekend), otherwise 2.
unsafe fn cup_weekend_slice(date: usize) -> u8 {
    let w = exe::date_get_weekday(date) as i16;
    if w == 5 || w == 6 {
        1
    } else {
        2
    }
}

/// `date` + `n` days (`exe::date_add_days`; the count is read as a word).
unsafe fn cup_date_plus(date: usize, n: i32) -> [u8; 8] {
    let mut out = [0u8; 8];
    exe::date_add_days(date, out.as_mut_ptr() as usize, n as u32);
    out
}

/// Swaps the clubs and the club ids of a fixture being built.
unsafe fn cup_swap_fixture_sides(p: Fixture) {
    let (home, away, home_id, away_id) = (p.home_club(), p.away_club(), p.home_club_id(), p.away_club_id());
    p.set_home_club(away);
    p.set_away_club(home);
    p.set_home_club_id(away_id);
    p.set_away_club_id(home_id);
}

/// A fixture being built that is still at the away club's stadium (`club::STADIUM`) moves to the home club's: the
/// stadium and its id (−1 with no stadium).
unsafe fn cup_follow_home_stadium(p: Fixture) {
    if read_usize(p.addr() + fixture::STADIUM) != read_usize(p.away_club().addr() + club::STADIUM) {
        return;
    }
    let stadium = read_usize(p.home_club().addr() + club::STADIUM);
    write_usize(p.addr() + fixture::STADIUM, stadium);
    write_usize(p.addr() + fixture::STADIUM_ID, if stadium != 0 { read_usize(stadium) } else { usize::MAX });
}

/// Sets the ten result bytes of a fixture being built (`fixture::HOME_GOALS` to `fixture::AWAY_AGGREGATE`) to 0xff, "not
/// played".
unsafe fn cup_clear_fixture_result(p: Fixture) {
    for k in 0..10 {
        write_u8(p.addr() + fixture::HOME_GOALS + k, 0xff);
    }
}

/// A first leg played whose second leg is already in the calendar (`cup_decision::FIRST_LEG`).
///
/// The tie's next match is the second (`cup_tie::NEXT_MATCH` 1). The second leg: the tie's next fixture from the day
/// after the match on (`find_next_fixture`, forward, the competition, round and both clubs). Not found: the error box
/// (line 0x4fe). Found: unless the rules (`fixture::TIE_RULES`) have 0x10 (points) or 0x2000, it takes this match's
/// score as its first-leg score, reversed (`fixture::HOME_AGGREGATE` = this match's away goals, `AWAY_AGGREGATE` = its
/// home goals) and the round rules (`apply_round_rules_to_match` with the tree slot); with them, this match's result
/// marks, reversed. Returns 3.
unsafe fn cup_first_leg_played(cup: Cup, f: Fixture, tie: usize, slot: i32) -> i32 {
    write_u8(tie + cup_tie::NEXT_MATCH, 1);
    let day_after = cup_date_plus(make_date(f.day() as u32, f.year() as u32).as_ptr() as usize, 1);
    let (mut day, mut year) = cup_date_day_year(&day_after);
    let mut slice = 0u8;
    let find: unsafe extern "thiscall" fn(usize, *mut u16, *mut u16, *mut u8, u32, i32, i32, i32, i32, u32, u32, u32) -> usize = core::mem::transmute(cup_fixture_fns::FIND_NEXT_FIXTURE);
    let next = find(read_usize(G_FIXTURE_MANAGER), &mut year, &mut day, &mut slice, 1, f.comp_id(), f.round_number() as i32, f.away_club_id() as i32, f.home_club_id() as i32, u32::MAX, u32::MAX, u32::MAX);
    if next == 0 {
        show_error_at(cup_fixture_fns::CUP_CPP, 0x4fe);
        return cup_decision::FIRST_LEG;
    }
    let rules = f.tie_rules();
    if rules & 0x10 == 0 && rules & 0x2000 == 0 {
        covered!("gra_25 second leg takes the first-leg score");
        write_u8(next + fixture::HOME_AGGREGATE, read_u8(f.addr() + fixture::AWAY_GOALS));
        write_u8(next + fixture::AWAY_AGGREGATE, read_u8(f.addr() + fixture::HOME_GOALS));
        let apply: unsafe extern "thiscall" fn(usize, usize, i32) = core::mem::transmute(cup_fixture_fns::APPLY_ROUND_RULES_TO_MATCH);
        apply(cup.addr(), next, slot);
        return cup_decision::FIRST_LEG;
    }
    write_u8(next + fixture::HOME_RESULT_MARK, read_u8(f.addr() + fixture::AWAY_RESULT_MARK));
    write_u8(next + fixture::AWAY_RESULT_MARK, read_u8(f.addr() + fixture::HOME_RESULT_MARK));
    cup_decision::FIRST_LEG
}

/// A draw (or a level tie) in a one-round cup whose replays are over (`cup_replays_exhausted`).
///
/// A tie fed from another (`cup_tie::FEEDER_TIE` ≠ −1): the "drawn game" text and the error box at `line`; 0. Otherwise
/// both teams get outcome 1, their progress is recorded (`record_team_progress`, the round) and the cup is marked
/// finished when every team has an outcome (`Cup::set_run_by_stages`, `cup_all_teams_decided`); 1.
unsafe fn cup_end_on_draw(cup: Cup, f: Fixture, tie: usize, feeder_tie: i32, total: i32, line: u32) -> i32 {
    let round = f.round_number();
    if feeder_tie != -1 {
        cup_fixture_error(cup, cup_fixture_fns::DRAWN_GAME_TEXT, round as i32, line);
        return 0;
    }
    covered!("gra_25 cup ends on a draw");
    let (a, b) = (read_i16(tie + cup_tie::HOME_TEAM), read_i16(tie + cup_tie::AWAY_TEAM));
    write_u8(team_entry(cup, a as i32) + cup_team_entry::STATUS, cup_team_status::WINNER);
    write_u8(team_entry(cup, b as i32) + cup_team_entry::STATUS, cup_team_status::WINNER);
    cup_record_progress(cup, a, round);
    cup_record_progress(cup, b, round);
    cup.set_run_by_stages(cup_all_teams_decided(total, |k| read_u8(team_entry(cup, k as i32) + cup_team_entry::STATUS)));
    1
}

/// A single match drawn (`cup_decision::DRAW`): the replay.
///
/// A copy of the match; before the first replay its date moves `cup_round::LEG_GAP_DAYS` days on (the byte unsigned);
/// the fixture manager finds it a free date (`exe::fixture_manager_find_new_fixture_date`); the time slice by the
/// weekday (`cup_weekend_slice`); one more replay in the tie (`cup_tie::REPLAYS_PLAYED`); the round name the base code
/// (`exe::comp_util_get_base_round_id`) + 1 for the first replay, + 2 for the next; the sides swapped (the replay at the
/// other ground) and the stadium following the home club (`cup_follow_home_stadium`); the bits 0x100, 0x200 and 0x800 of
/// `fixture::SQUAD_RULES` cleared, the result bytes "not played", no statistics slot. The rules: after the first match
/// `cup_round::TWO_LEG_TIE_RULES`, with bit 0x400 of `SQUAD_RULES` when `cup_round::SINGLE_LEG_FLAG` is above 1; after
/// the second `cup_fixture_fields::ROUND_LATER_TIE_RULES`, without 0x400; after a third: the "third match" text and the
/// error box (line 0x5aa), 0. The stage index of the match, the result marks 0, the date at least tomorrow and into the
/// calendar (`add_fixture_to_calendar`). Returns 1.
unsafe fn cup_schedule_replay(cup: Cup, f: Fixture, slot: i32, played: i32) -> i32 {
    let round = f.round_number() as i32;
    let tie = || round_tie(cup, round, slot);
    let mut buf = [0u8; fixture::SIZE];
    core::ptr::copy_nonoverlapping(f.addr() as *const u8, buf.as_mut_ptr(), fixture::SIZE);
    let p = Fixture(buf.as_mut_ptr() as usize);
    if read_u8(tie() + cup_tie::REPLAYS_PLAYED) == 0 {
        let gap = read_u8(round_record(cup, round) + cup_round::LEG_GAP_DAYS) as i32;
        let d = cup_date_plus(make_date(f.day() as u32, f.year() as u32).as_ptr() as usize, gap);
        let (day, year) = cup_date_day_year(&d);
        p.set_day(day);
        p.set_year(year);
    }
    exe::fixture_manager_find_new_fixture_date(read_usize(G_FIXTURE_MANAGER), p);
    p.set_time_slice(cup_weekend_slice(make_date(p.day() as u32, p.year() as u32).as_ptr() as usize));
    write_u8(tie() + cup_tie::REPLAYS_PLAYED, read_u8(tie() + cup_tie::REPLAYS_PLAYED).wrapping_add(1));
    let step = if read_u8(tie() + cup_tie::REPLAYS_PLAYED) == 1 { 1 } else { 2 };
    p.set_round_name((exe::comp_util_get_base_round_id(read_u32(p.addr() + fixture::ROUND_NAME)) as u16).wrapping_add(step) as i16);
    cup_swap_fixture_sides(p); // the replay at the other ground
    cup_follow_home_stadium(p);
    p.set_flags(p.flags() & 0xf4);
    cup_clear_fixture_result(p);
    p.set_match_stats_slot(usize::MAX);
    let r = round_record(cup, round);
    match played {
        0 => {
            covered!("gra_25 first replay");
            p.set_tie_rules(read_u16(r + cup_round::TWO_LEG_TIE_RULES));
            if (read_u8(r + cup_round::SINGLE_LEG_FLAG) as i8) > 1 {
                p.set_flags(p.flags() | 4);
            } else {
                p.set_flags(p.flags() & !4);
            }
        }
        1 => {
            covered!("gra_25 second replay");
            p.set_tie_rules(read_u16(r + cup_fixture_fields::ROUND_LATER_TIE_RULES));
            p.set_flags(p.flags() & !4);
        }
        _ => {
            cup_fixture_error(cup, cup_fixture_fns::THIRD_MATCH_TEXT, round, 0x5aa);
            return 0;
        }
    }
    p.set_stage_index(f.stage_index());
    p.set_home_result_mark(result_mark::NONE);
    p.set_away_result_mark(result_mark::NONE);
    add_fixture_to_calendar(p);
    1
}

/// A tie level after two legs (`cup_decision::LEVEL`): the deciding match.
///
/// A copy of the match, 4 days later with rule 0x2000, otherwise 7; no first-leg score; the time slice by the weekday;
/// one more replay in the tie (`cup_tie::REPLAYS_PLAYED`); the round name the base code + 1 when
/// `cup_round::SINGLE_LEG_FLAG` is set, otherwise + 5 (the decider); the sides swapped unless rule 0x2000, the stadium
/// following the home club; the result bytes "not played", no statistics slot, the rules
/// `cup_fixture_fields::ROUND_LATER_TIE_RULES`, the bits 0x100 to 0x800 of `fixture::SQUAD_RULES` cleared; the date at
/// least tomorrow and into the calendar. The result marks stay the match's. Returns 1.
unsafe fn cup_schedule_deciding_match(cup: Cup, f: Fixture, slot: i32) -> i32 {
    let round = f.round_number() as i32;
    let mut buf = [0u8; fixture::SIZE];
    core::ptr::copy_nonoverlapping(f.addr() as *const u8, buf.as_mut_ptr(), fixture::SIZE);
    let p = Fixture(buf.as_mut_ptr() as usize);
    let points = f.tie_rules() & 0x2000 != 0;
    let d = cup_date_plus(make_date(p.day() as u32, p.year() as u32).as_ptr() as usize, if points { 4 } else { 7 });
    let (day, year) = cup_date_day_year(&d);
    p.set_year(year);
    p.set_day(day);
    write_u8(p.addr() + fixture::HOME_AGGREGATE, 0xff);
    write_u8(p.addr() + fixture::AWAY_AGGREGATE, 0xff);
    p.set_time_slice(cup_weekend_slice(d.as_ptr() as usize));
    let tie = round_tie(cup, round, slot);
    write_u8(tie + cup_tie::REPLAYS_PLAYED, read_u8(tie + cup_tie::REPLAYS_PLAYED).wrapping_add(1));
    let step = if read_u8(round_record(cup, round) + cup_round::SINGLE_LEG_FLAG) != 0 { 1 } else { 5 };
    p.set_round_name((exe::comp_util_get_base_round_id(read_u32(p.addr() + fixture::ROUND_NAME)) as u16).wrapping_add(step) as i16);
    if !points {
        covered!("gra_25 deciding match");
        cup_swap_fixture_sides(p);
    }
    cup_follow_home_stadium(p);
    cup_clear_fixture_result(p);
    p.set_match_stats_slot(usize::MAX);
    p.set_tie_rules(read_u16(round_record(cup, round) + cup_fixture_fields::ROUND_LATER_TIE_RULES));
    p.set_flags(p.flags() & 0xf0);
    add_fixture_to_calendar(p);
    1
}

/// The match of a next-round tie whose two teams are known, built when the tie's fixture was already due
/// (`cup_tie::FIXTURE_STATE` 3).
///
/// In a local fixture (not zeroed by the original: the bits 0xf0 of `fixture::FLAGS` keep stack leftovers, docs/
/// MAPA_SAVE.md; here zero): the competition and its id (−1 without one); from round `round` the code
/// (`cup_round::ROUND_CODE`, + 3 in a two-legged round), the cup's `stage_label()` and `year()`, the round number; the
/// cup's virtual `comp_vtable::SET_FIXTURE_SUBSTITUTES`; then the round's draw rules, tie rules and match category;
/// bits 0x100, 0x200 and 0x800 of `SQUAD_RULES` cleared, the result bytes "not played", no statistics slot, the match's
/// stage index, no stage (0xff), both side marks 1. Single-legged with tie rule 0x800 (the higher seed at home), the
/// rules become 0x1000 when the home team's seed (`cup_team_entry::SEED_LEVEL`) is not below the away team's, otherwise
/// 0x800. Bit 0x400 of `SQUAD_RULES` when single-legged with `cup_round::SINGLE_LEG_FLAG` above 0. The next round's date
/// (0 in the last round). The clubs of the tie's teams and their ids (no club: the error box, line 0x804 or 0x80f, and
/// −1). The date: today when the round's date is not after the day after tomorrow, otherwise the round's; the slice the
/// round's (`cup_round::TIME_SLICE`); the stadium (`exe::stadium_set_fixture_venue` with `cup_round::VENUE_VALUE` and the
/// cup's `venue_stadium_id()`); a round date not after the day after tomorrow asks the fixture manager for a free date.
/// The result bytes again, the stage index, the result marks 0; at least tomorrow and into the calendar. If the cup is
/// in [`G_COMP_OBJECT_TABLE`] (by `comp_id`): with a human manager at either club, news item 0x14 to both (0 the
/// competition id, 1 the stage label, 2 the round code, 3 and 4 the clubs, the human's first, 5 two legs, 6 the stadium
/// id, 7 the day, 8 the year); in a two-legged round, the second leg: code + 4, bit 0x400 by `SINGLE_LEG_FLAG` above 0,
/// on the later of the round's date + `cup_round::LEG_GAP_DAYS` and three days after the first leg (that one asks the
/// fixture manager for a free date), the slice by the weekday, the sides swapped, the stadium, bits 0x100 to 0x800
/// cleared, the result bytes, the stage index, the result marks 0, at least tomorrow and into the calendar. Returns 1.
unsafe fn cup_create_next_round_fixture(cup: Cup, f: Fixture, round: i32, tie_index: i32, comp_id: i32) -> i32 {
    let stage = f.stage_index();
    let mut buf = [0u8; fixture::SIZE];
    let p = Fixture(buf.as_mut_ptr() as usize);
    let comp = read_usize(cup.addr() + league::COMPETITION);
    if comp != 0 {
        write_usize(p.addr() + fixture::COMPETITION, comp);
        write_usize(p.addr() + fixture::COMP_ID, read_usize(comp));
    } else {
        write_usize(p.addr() + fixture::COMP_ID, usize::MAX);
        write_usize(p.addr() + fixture::COMPETITION, 0);
    }
    let rr = || round_record(cup, round);
    let code = read_i16(rr() + cup_round::ROUND_CODE);
    p.set_round_name(if read_u8(rr() + cup_round::LEGS) == 2 { code.wrapping_add(3) } else { code });
    p.set_round_prefix(cup.stage_label());
    p.set_season_year(cup.year() as u16);
    p.set_round_number(read_i16(rr() + cup_round::ROUND_NUMBER));
    let substitutes: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(cup_fixture_virtual(cup, comp_vtable::SET_FIXTURE_SUBSTITUTES));
    substitutes(cup.addr(), p.addr());
    let r = rr();
    p.set_round_draw_rules(read_u16(r + cup_round::DRAW_RULES));
    p.set_tie_rules(read_u16(r + cup_round::TIE_RULES));
    p.set_category(read_u8(r + cup_round::MATCH_CATEGORY));
    p.set_flags(p.flags() & 0xf4);
    cup_clear_fixture_result(p);
    p.set_match_stats_slot(usize::MAX);
    p.set_stage_index(stage);
    p.set_stage(0xff);
    p.set_home_side_mark(1);
    p.set_away_side_mark(1);
    let tie = || round_tie(cup, round, tie_index);
    if (read_u8(r + cup_round::LEGS) as i8) < 2 && p.tie_rules() & 0x800 != 0 {
        covered!("gra_25 higher seed at home");
        let seed = |t: i16| read_i8(team_entry(cup, t as i32) + cup_team_entry::SEED_LEVEL);
        p.set_tie_rules(if seed(read_i16(tie() + cup_tie::HOME_TEAM)) >= seed(read_i16(tie() + cup_tie::AWAY_TEAM)) { 0x1000 } else { 0x800 });
    }
    if (read_u8(r + cup_round::SINGLE_LEG_FLAG) as i8) > 0 && (read_u8(r + cup_round::LEGS) as i8) < 2 {
        p.set_flags(p.flags() | 4);
    } else {
        p.set_flags(p.flags() & !4);
    }
    if round < cup.round_count() as i32 - 1 {
        p.set_next_round_day(read_u16(r + cup_round::SIZE + cup_round::DAY));
        p.set_next_round_year((read_i8(r + cup_round::SIZE + cup_round::YEAR_OFFSET) as i16).wrapping_add(cup.year()) as u16);
    } else {
        p.set_next_round_day(0);
        p.set_next_round_year(0);
    }
    for (side, line) in [(cup_tie::HOME_TEAM, 0x804u32), (cup_tie::AWAY_TEAM, 0x80f)] {
        let club = read_usize(team_entry(cup, read_i16(tie() + side) as i32));
        let (at, id_at) = if side == cup_tie::HOME_TEAM { (fixture::HOME_CLUB, fixture::HOME_CLUB_ID) } else { (fixture::AWAY_CLUB, fixture::AWAY_CLUB_ID) };
        write_usize(p.addr() + at, club);
        if club != 0 {
            write_usize(p.addr() + id_at, read_usize(club));
        } else {
            show_error_at(cup_fixture_fns::CUP_CPP, line);
            write_usize(p.addr() + id_at, usize::MAX);
        }
    }
    let soon = cup_date_plus(G_CURRENT_DATE, 2);
    let (soon_day, soon_year) = (read_i16(soon.as_ptr() as usize), read_u16(soon.as_ptr() as usize + 2) as i32);
    let due = || {
        let r = rr();
        let year = read_i8(r + cup_round::YEAR_OFFSET) as i32 + cup.year() as i32;
        year < soon_year || (year == soon_year && soon_day >= read_i16(r + cup_round::DAY))
    };
    let r = rr();
    if due() {
        covered!("gra_25 next round match due now");
        p.set_year(read_u16(G_CURRENT_DATE_YEAR));
        p.set_day(read_u16(G_CURRENT_DATE));
    } else {
        p.set_year((read_i8(r + cup_round::YEAR_OFFSET) as i16).wrapping_add(cup.year()) as u16);
        p.set_day(read_u16(r + cup_round::DAY));
    }
    p.set_time_slice(read_u8(r + cup_round::TIME_SLICE));
    exe::stadium_set_fixture_venue(read_usize(G_STADIUM), read_usize(r + cup_round::VENUE_VALUE), p, cup.venue_stadium_id());
    if due() {
        exe::fixture_manager_find_new_fixture_date(read_usize(G_FIXTURE_MANAGER), p);
    }
    cup_clear_fixture_result(p);
    p.set_stage_index(stage);
    p.set_home_result_mark(result_mark::NONE);
    p.set_away_result_mark(result_mark::NONE);
    add_fixture_to_calendar(p);
    if read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add((comp_id as usize).wrapping_mul(4))) == 0 {
        return 1;
    }
    let (home, away) = (p.home_club(), p.away_club());
    if cup_has_human_manager(home.addr()) || cup_has_human_manager(away.addr()) {
        covered!("gra_25 next round news");
        let mut news = [0u32; 0x38]; // the local news item (0xe0 bytes)
        let n = news.as_mut_ptr() as usize;
        exe::new_newsitem(n, 0x14, 0);
        let r = rr();
        exe::newsitem_set_slot(n, 0, read_u32(read_usize(cup.addr() + league::COMPETITION)));
        exe::newsitem_set_slot(n, 1, cup.stage_label() as i32 as u32);
        exe::newsitem_set_slot(n, 2, read_i16(r + cup_round::ROUND_CODE) as i32 as u32);
        exe::newsitem_set_slot(n, 5, (read_u8(r + cup_round::LEGS) == 2) as u32);
        exe::newsitem_set_slot(n, 6, p.stadium_id() as u32);
        exe::newsitem_set_slot(n, 7, p.day() as u32);
        exe::newsitem_set_slot(n, 8, p.year() as u32);
        let (first, second) = if cup_has_human_manager(home.addr()) { (p.home_club_id(), p.away_club_id()) } else { (p.away_club_id(), p.home_club_id()) };
        exe::newsitem_set_slot(n, 3, first as u32);
        exe::newsitem_set_slot(n, 4, second as u32);
        exe::news_send_to_club_manager(read_usize(G_NEWS), n, home);
        exe::news_send_to_club_manager(read_usize(G_NEWS), n, away);
    }
    let r = rr();
    if read_u8(r + cup_round::LEGS) != 2 {
        return 1;
    }
    covered!("gra_25 next round second leg");
    if (read_u8(r + cup_round::SINGLE_LEG_FLAG) as i8) > 0 {
        p.set_flags(p.flags() | 4);
    } else {
        p.set_flags(p.flags() & !4);
    }
    p.set_round_name(read_i16(r + cup_round::ROUND_CODE).wrapping_add(4));
    let first_leg = make_date(p.day() as u32, p.year() as u32);
    let round_day = make_date(read_u16(r + cup_round::DAY) as u32, (read_i8(r + cup_round::YEAR_OFFSET) as i16).wrapping_add(cup.year()) as u16 as u32);
    let by_round = cup_date_plus(round_day.as_ptr() as usize, read_u8(rr() + cup_round::LEG_GAP_DAYS) as i32);
    let by_first_leg = cup_date_plus(first_leg.as_ptr() as usize, 3);
    let ((rd, ry), (fd, fy)) = (cup_date_day_year(&by_round), cup_date_day_year(&by_first_leg));
    if ry < fy || (ry == fy && (rd as i16) < (fd as i16)) {
        p.set_year(fy);
        p.set_day(fd);
        exe::fixture_manager_find_new_fixture_date(read_usize(G_FIXTURE_MANAGER), p);
    } else {
        p.set_year(ry);
        p.set_day(rd);
    }
    p.set_time_slice(cup_weekend_slice(make_date(p.day() as u32, p.year() as u32).as_ptr() as usize));
    cup_swap_fixture_sides(p);
    exe::stadium_set_fixture_venue(read_usize(G_STADIUM), read_usize(rr() + cup_round::VENUE_VALUE), p, cup.venue_stadium_id());
    p.set_flags(p.flags() & 0xf4);
    cup_clear_fixture_result(p);
    p.set_stage_index(stage);
    p.set_home_result_mark(result_mark::NONE);
    p.set_away_result_mark(result_mark::NONE);
    add_fixture_to_calendar(p);
    1
}

/// A tie decided (`cup_decision::HOME`/`AWAY`): the winner goes on, the loser goes out.
///
/// The winner and loser by `cup_tie_sides`. In the last tie of the tree (`cup_tie::FEEDER_TIE` −1): outcomes 1 and 2,
/// or 3 and 4 in the third-place round (`round_name::THIRD_PLACE_PLAYOFF`), the cup's result news for both
/// (`cup_announce_team`), their progress (`record_team_progress`), the round's prize money (`pay_round_prize_money`) and
/// the cup marked finished when every team has an outcome (`cup_all_teams_decided`). Otherwise the winner takes its side
/// of the next tie (`cup_tie::FEEDER_ROUND`/`FEEDER_TIE`; side A when `cup_tie::FEEDER_SIDE` is set, else side B) and
/// the news; in the semi-final (round = rounds − 3) of a cup with a third-place round (`cup_third_place_round`) the loser
/// takes the same side of the same tie there and the news with outcome −1, otherwise it is out (outcome 0xff) with the
/// news; then the progress of both and the prize money. When the round is before the last one drawn
/// (`next_draw_round()` − 1), the next round was drawn, its tie's fixture was due (`cup_tie::FIXTURE_STATE` 3) and both
/// its teams are known: the home sides of that tie (`exe::cup_set_home_teams_for_round`) and its match
/// (`cup_create_next_round_fixture`). Returns 1.
unsafe fn cup_advance_tie_winner(cup: Cup, f: Fixture, a2: u32, slot: i32, r: i32, feeder: (i32, i32), total: i32, comp_id: i32) -> i32 {
    let (feeder_tie, feeder_round) = feeder;
    let round = f.round_number();
    let third = cup_third_place_round(cup);
    let tie = round_tie(cup, round as i32, slot);
    let (w, l) = cup_tie_sides(cup, tie, f.addr(), r);
    covered!("gra_25 tie decided");
    let status = |t: i16| read_u8(team_entry(cup, t as i32) + cup_team_entry::STATUS);
    let pay = |w: i16, l: i16| {
        let f_: unsafe extern "thiscall" fn(usize, usize, usize, u32, usize) = core::mem::transmute(cup_fixture_fns::PAY_ROUND_PRIZE_MONEY);
        f_(cup.addr(), team_entry(cup, w as i32), team_entry(cup, l as i32), round as u16 as u32, f.addr());
    };
    if feeder_tie == -1 {
        covered!("gra_25 final decided");
        let (sw, sl) = if read_i16(round_record(cup, round as i32) + cup_round::ROUND_CODE) == round_name::THIRD_PLACE_PLAYOFF {
            (cup_team_status::THIRD, cup_team_status::FOURTH)
        } else {
            (cup_team_status::WINNER, cup_team_status::RUNNER_UP)
        };
        write_u8(team_entry(cup, w as i32) + cup_team_entry::STATUS, sw);
        write_u8(team_entry(cup, l as i32) + cup_team_entry::STATUS, sl);
        cup_announce_team(cup, w, status(w) as u32, f.addr(), a2);
        cup_announce_team(cup, l, status(l) as u32, f.addr(), a2);
        cup_record_progress(cup, w, round);
        cup_record_progress(cup, l, round);
        pay(w, l);
        cup.set_run_by_stages(cup_all_teams_decided(total, |k| status(k)));
        return 1;
    }
    let side = if read_u16(tie + cup_tie::FEEDER_SIDE) != 0 { cup_tie::HOME_TEAM } else { cup_tie::AWAY_TEAM };
    write_i16(round_tie(cup, feeder_round, feeder_tie) + side, w);
    cup_announce_team(cup, w, status(w) as u32, f.addr(), a2);
    if round as i32 == cup.round_count() as i32 - 3 && third > -1 {
        covered!("gra_25 semi-final loser to the third-place match");
        write_i16(round_tie(cup, third, feeder_tie) + side, l);
        cup_announce_team(cup, l, u32::MAX, f.addr(), a2);
    } else {
        write_u8(team_entry(cup, l as i32) + cup_team_entry::STATUS, cup_team_status::OUT);
        cup_announce_team(cup, l, status(l) as u32, f.addr(), a2);
    }
    cup_record_progress(cup, w, round);
    cup_record_progress(cup, l, round);
    pay(w, l);
    let drawn = cup.next_draw_round() as i8 as i32;
    if round as i32 >= drawn - 1 || feeder_round >= drawn {
        return 1;
    }
    let next = round_tie(cup, feeder_round, feeder_tie);
    if read_u8(next + cup_tie::FIXTURE_STATE) != 3 {
        return 1;
    }
    if read_i16(next + cup_tie::HOME_TEAM) == -1 || read_i16(next + cup_tie::AWAY_TEAM) == -1 {
        return 1;
    }
    covered!("gra_25 next round match");
    exe::cup_set_home_teams_for_round(cup, feeder_round as u32, feeder_tie as u32);
    cup_create_next_round_fixture(cup, f, feeder_round, feeder_tie, comp_id)
}

/// Processes a played cup match: pays the match fee, stores the result in the tie, decides it and moves the cup on (the
/// second leg, a replay, a deciding match, or the winner to the next round and the loser out).
///
/// `Cup::process_fixture(fixture, a2)` (SEH frame without a destructor; slot `comp_vtable::ADD_RESULT`, called after
/// every cup match, also by the European cups' and the Intertoto's own `process_fixture`). A top-level cup
/// (`stage_index()` 0xff) handed a match of one of its stages (`fixture::STAGE_INDEX` ≠ 0xff) passes it to that stage's
/// `process_fixture` (the stage at `stages()`, index up to `current_stage()`; outside: the error box, line 0x473, 0); a
/// match of another stage: 0. The competition statistics record the match (`exe::comp_stats_record_match_result` on the
/// `league::COMP_STATS` of the cup's entry in [`G_COMP_OBJECT_TABLE`]). A match from before the cup's `year()`: −1. The
/// round's match fee (`cup_fixture_fields::ROUND_MATCH_PRIZE`, above 0) goes to both clubs' finances
/// (`exe::finance_add_prize_money`). The match's tie (`find_tree_slot_for_fixture`; none: the error box, line 0x4a3,
/// −1); its result block by the matches played (`cup_tie::REPLAYS_PLAYED` + `cup_tie::NEXT_MATCH`: 0, 1 or 2, otherwise
/// the box at line 0x4be and −1) takes the match's ten result bytes; the cup decides (`cup_fixture_decision`) and the
/// tie keeps the result marks in side order (`cup_tie_marks`). By the decision: −1 the box (line 0x4e6) and −1; a first
/// leg `cup_first_leg_played`; a draw `cup_end_on_draw` (line 0x51f) when the replays are exhausted, otherwise
/// `cup_schedule_replay`; a level tie `cup_end_on_draw` (line 0x5c7) or `cup_schedule_deciding_match`; a winner
/// `cup_advance_tie_winner`; anything else the box (line 0x78c) and −1.
/// Original: `Cup::process_fixture` (0x51a150, Cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn cup_process_fixture(this: usize, fixture: usize, a2: u32) -> i32 {
    if fixture == 0 {
        // the original reads the stage index (+0x42) of the null fixture before its box (line 0x467): it faults there
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> i32>(%%I0%%)(this, fixture, a2);
    }
    let cup = Cup(this);
    let f = Fixture(fixture);
    let stage = f.stage_index();
    let comp_id = read_i32(read_usize(this + league::COMPETITION));
    if cup.stage_index() == 0xff && stage != 0xff {
        if (stage as i8 as i32) <= cup.current_stage() as i32 && (stage as i8) >= 0 {
            covered!("gra_25 match of a stage");
            let s = read_usize(cup.stages() + 4 * stage as usize);
            let process: unsafe extern "thiscall" fn(usize, usize, u32) -> i32 = core::mem::transmute(read_usize(read_usize(s) + comp_vtable::ADD_RESULT));
            return process(s, fixture, a2);
        }
        show_error_at(cup_fixture_fns::CUP_CPP, 0x473);
        return 0;
    }
    if cup.stage_index() != stage {
        return 0;
    }
    let object = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add((comp_id as usize).wrapping_mul(4)));
    if object != 0 && read_usize(object + league::COMP_STATS) != 0 {
        exe::comp_stats_record_match_result(read_usize(object + league::COMP_STATS), fixture, a2 as usize, 0, 0);
    }
    if (f.year() as i32) < cup.year() as i32 {
        return -1;
    }
    let mut total = 0i32; // the teams of the whole cup (each round's new teams)
    for k in 0..(cup.round_count() as i32).max(0) {
        total = total.wrapping_add(read_u16(round_record(cup, k) + cup_round::NEW_TEAMS) as i32);
    }
    let round = f.round_number() as i32;
    let fee = read_i32(round_record(cup, round) + cup_fixture_fields::ROUND_MATCH_PRIZE);
    if fee > 0 {
        covered!("gra_25 match fee");
        exe::finance_add_prize_money(exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), f.home_club()), fee);
        let fee = read_i32(round_record(cup, round) + cup_fixture_fields::ROUND_MATCH_PRIZE);
        exe::finance_add_prize_money(exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), f.away_club()), fee);
    }
    let find: unsafe extern "thiscall" fn(usize, usize) -> i32 = core::mem::transmute(cup_fixture_fns::FIND_TREE_SLOT_FOR_FIXTURE);
    let slot = find(this, fixture);
    if slot == -1 {
        show_error_at(cup_fixture_fns::CUP_CPP, 0x4a3);
        return -1;
    }
    let tie = || round_tie(cup, round, slot);
    let feeder_tie = read_i16(tie() + cup_tie::FEEDER_TIE) as i32;
    let played = read_i8(tie() + cup_tie::REPLAYS_PLAYED) as i32 + read_i8(tie() + cup_tie::NEXT_MATCH) as i32;
    if !(0..3).contains(&played) {
        show_error_at(cup_fixture_fns::CUP_CPP, 0x4be);
        return -1;
    }
    core::ptr::copy_nonoverlapping((fixture + fixture::HOME_GOALS) as *const u8, (tie() + cup_fixture_fields::TIE_RESULT_BLOCKS[played as usize]) as *mut u8, 10);
    let feeder_round = read_i16(tie() + cup_tie::FEEDER_ROUND) as i32;
    let r = cup_fixture_decision(cup, round, tie(), fixture, played);
    let (a, b) = cup_tie_marks(cup, tie(), fixture);
    write_u8(tie() + cup_fixture_fields::TIE_HOME_MARK, a);
    write_u8(tie() + cup_fixture_fields::TIE_AWAY_MARK, b);
    match r {
        cup_decision::ERROR => {
            show_error_at(cup_fixture_fns::CUP_CPP, 0x4e6);
            -1
        }
        cup_decision::FIRST_LEG => cup_first_leg_played(cup, f, tie(), slot),
        cup_decision::DRAW if cup_replays_exhausted(cup, round, tie()) => cup_end_on_draw(cup, f, tie(), feeder_tie, total, 0x51f),
        cup_decision::DRAW => cup_schedule_replay(cup, f, slot, played),
        cup_decision::LEVEL if cup_replays_exhausted(cup, round, tie()) => cup_end_on_draw(cup, f, tie(), feeder_tie, total, 0x5c7),
        cup_decision::LEVEL => cup_schedule_deciding_match(cup, f, slot),
        cup_decision::HOME | cup_decision::AWAY => cup_advance_tie_winner(cup, f, a2, slot, r, (feeder_tie, feeder_round), total, comp_id),
        _ => {
            show_error_at(cup_fixture_fns::CUP_CPP, 0x78c);
            -1
        }
    }
}

/// What `cup_process_fixture` will do to the tie, worked out by the comparer from the state before.
struct CupFixturePlan {
    tie: usize,
    tie_after: [u8; 0x35],
    marks: (u8, u8),
    result: i32,
    statuses: Vec<(i16, u8)>,
    next: Option<(usize, usize, i16, bool)>, // the next tie, the winner's side, the winner, whether its home sides may be re-set
    third: Option<(usize, i16)>,
    finished: Option<u8>,
}

/// The comparer's plan for `cup_process_fixture` (None: a case it does not plan; only the original runs).
///
/// From the state before: the tie (`find_tree_slot_for_fixture`) and its matches played; the decision taken on copies
/// (the tie with the result block written and the fixture, 0x4f bytes, so the original's own decision is the only one
/// that touches the real fixture: with rule 0x2000 the decision changes the marks it reads); then, through the same
/// helpers as the copy (`cup_tie_marks`, `cup_tie_sides`, `cup_replays_exhausted`, `cup_third_place_round`,
/// `cup_all_teams_decided`), the tie afterwards (block, marks, next match, replays), the outcomes, the next tie's and the
/// third-place tie's side, the cup's finished mark and the return value.
unsafe fn cup_process_fixture_plan(cup: Cup, fixture: usize) -> Option<CupFixturePlan> {
    if fixture == 0 {
        return None;
    }
    let f = Fixture(fixture);
    if cup.stage_index() != f.stage_index() || (f.year() as i32) < cup.year() as i32 {
        return None;
    }
    let round = f.round_number() as i32;
    let slot = find_tree_slot_for_fixture(cup.addr(), fixture);
    if slot == -1 {
        return None;
    }
    let tie = round_tie(cup, round, slot);
    let played = read_i8(tie + cup_tie::REPLAYS_PLAYED) as i32 + read_i8(tie + cup_tie::NEXT_MATCH) as i32;
    if !(0..3).contains(&played) {
        return None;
    }
    let mut t = [0u8; 0x35];
    core::ptr::copy_nonoverlapping(tie as *const u8, t.as_mut_ptr(), 0x35);
    core::ptr::copy_nonoverlapping((fixture + fixture::HOME_GOALS) as *const u8, t.as_mut_ptr().add(cup_fixture_fields::TIE_RESULT_BLOCKS[played as usize]), 10);
    let mut fl = [0u8; fixture::SIZE];
    core::ptr::copy_nonoverlapping(fixture as *const u8, fl.as_mut_ptr(), fixture::SIZE);
    let (ta, fa) = (t.as_mut_ptr() as usize, fl.as_mut_ptr() as usize);
    let r = cup_fixture_decision(cup, round, ta, fa, played);
    let marks = (read_u8(fa + fixture::HOME_RESULT_MARK), read_u8(fa + fixture::AWAY_RESULT_MARK));
    let (a, b) = cup_tie_marks(cup, ta, fa);
    t[cup_fixture_fields::TIE_HOME_MARK] = a;
    t[cup_fixture_fields::TIE_AWAY_MARK] = b;
    let feeder_tie = read_i16(ta + cup_tie::FEEDER_TIE) as i32;
    let feeder_round = read_i16(ta + cup_tie::FEEDER_ROUND) as i32;
    let mut total = 0i32;
    for k in 0..(cup.round_count() as i32).max(0) {
        total = total.wrapping_add(read_u16(round_record(cup, k) + cup_round::NEW_TEAMS) as i32);
    }
    let mut plan = CupFixturePlan { tie, tie_after: t, marks, result: -1, statuses: Vec::new(), next: None, third: None, finished: None };
    let finished = |changes: &[(i16, u8)]| {
        cup_all_teams_decided(total, |k| changes.iter().find(|c| c.0 == k).map(|c| c.1).unwrap_or_else(|| read_u8(team_entry(cup, k as i32) + cup_team_entry::STATUS)))
    };
    match r {
        cup_decision::FIRST_LEG => {
            plan.tie_after[cup_tie::NEXT_MATCH] = 1;
            plan.result = 3;
        }
        cup_decision::DRAW | cup_decision::LEVEL if cup_replays_exhausted(cup, round, ta) => {
            if feeder_tie == -1 {
                plan.statuses = vec![(read_i16(ta + cup_tie::HOME_TEAM), 1), (read_i16(ta + cup_tie::AWAY_TEAM), 1)];
                plan.finished = Some(finished(&plan.statuses));
                plan.result = 1;
            } else {
                plan.result = 0;
            }
        }
        cup_decision::DRAW | cup_decision::LEVEL => {
            plan.tie_after[cup_tie::REPLAYS_PLAYED] = plan.tie_after[cup_tie::REPLAYS_PLAYED].wrapping_add(1);
            plan.result = if r == cup_decision::DRAW && played >= 2 { 0 } else { 1 };
        }
        cup_decision::HOME | cup_decision::AWAY => {
            let (w, l) = cup_tie_sides(cup, ta, fixture, r);
            plan.result = 1;
            if feeder_tie == -1 {
                let third_round = read_i16(round_record(cup, round) + cup_round::ROUND_CODE) == round_name::THIRD_PLACE_PLAYOFF;
                plan.statuses = if third_round { vec![(w, 3), (l, 4)] } else { vec![(w, 1), (l, 2)] };
                plan.finished = Some(finished(&plan.statuses));
            } else {
                let side = if read_u16(ta + cup_tie::FEEDER_SIDE) != 0 { cup_tie::HOME_TEAM } else { cup_tie::AWAY_TEAM };
                let next = round_tie(cup, feeder_round, feeder_tie);
                plan.next = Some((next, side, w, read_u8(next + cup_tie::FIXTURE_STATE) == 3));
                let third = cup_third_place_round(cup);
                if round == cup.round_count() as i32 - 3 && third > -1 {
                    plan.third = Some((round_tie(cup, third, feeder_tie) + side, l));
                } else {
                    plan.statuses = vec![(l, cup_team_status::OUT)];
                }
            }
        }
        _ => plan.result = -1,
    }
    Some(plan)
}

/// Comparer for `cup_process_fixture`: only the original runs (it adds fixtures, posts news, pays prize money and
/// records progress: it cannot run twice), checked against the plan made beforehand (`cup_process_fixture_plan`): the
/// return value, the whole tie (0x35 bytes: the result block, the marks in side order, the next match, the replays),
/// the fixture's result marks, the outcomes of the teams it decides, the winner in the next tie (on its side, or on
/// either when the next tie's home sides are set again), the loser in the third-place tie and the cup's finished mark.
/// A match of a stage, of another stage, from before the cup's year or without a tie: only the original.
pub(crate) unsafe extern "thiscall" fn compare_cup_process_fixture(this: usize, fixture: usize, a2: u32) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> i32>(%%I0%%);
    let cup = Cup(this);
    let Some(plan) = cup_process_fixture_plan(cup, fixture) else {
        return o(this, fixture, a2);
    };
    let ret = o(this, fixture, a2);
    let mut tie = [0u8; 0x35];
    core::ptr::copy_nonoverlapping(plan.tie as *const u8, tie.as_mut_ptr(), 0x35);
    let marks = (read_u8(fixture + fixture::HOME_RESULT_MARK), read_u8(fixture + fixture::AWAY_RESULT_MARK));
    let mut wrong: Vec<String> = Vec::new();
    if ret != plan.result {
        wrong.push(format!("returned {} x plan {}", ret, plan.result));
    }
    if let Some(k) = (0..0x35).find(|&k| tie[k] != plan.tie_after[k]) {
        wrong.push(format!("tie +{:#x}: {:02x?} x plan {:02x?}", k, &tie[..], &plan.tie_after[..]));
    }
    if marks != plan.marks {
        wrong.push(format!("fixture marks {:?} x plan {:?}", marks, plan.marks));
    }
    for &(t, s) in &plan.statuses {
        let got = read_u8(team_entry(cup, t as i32) + cup_team_entry::STATUS);
        if got != s {
            wrong.push(format!("team {} outcome {} x plan {}", t, got, s));
        }
    }
    if let Some((next, side, w, may_swap)) = plan.next {
        let (h, a) = (read_i16(next + cup_tie::HOME_TEAM), read_i16(next + cup_tie::AWAY_TEAM));
        if read_i16(next + side) != w && !(may_swap && (h == w || a == w)) {
            wrong.push(format!("next tie {} {} x winner {} at +{}", h, a, w, side));
        }
    }
    if let Some((at, l)) = plan.third {
        if read_i16(at) != l {
            wrong.push(format!("third-place tie {} x loser {}", read_i16(at), l));
        }
    }
    if let Some(done) = plan.finished {
        if cup.run_by_stages() != done {
            wrong.push(format!("finished mark {} x plan {}", cup.run_by_stages(), done));
        }
    }
    SCOREBOARD.record(wrong.is_empty(), || format!("cup_process_fixture(cup {:#x}, fixture {:#x}, round {}) {:?}", this, fixture, Fixture(fixture).round_number(), wrong));
    ret
}
