use super::*;

/// Builds the Swedish First Division: discipline rule 28 (the Swedish league one) and one stage after the league (the
/// promotion play-off); with no saved state, the rules, the 16-club table, the fixtures, the order, the stats and the
/// predicted positions.
///
/// `SweFirst::SweFirst(year, record)`: `construct_league_with_stages` with vtable 0x9700e4, discipline byte 0x1c, one
/// extra stage, `SweFirst::set_up_rules` (0x88f090) and `SweFirst::create_table` (0x88ef90); the boxes of swe_first.cpp
/// lines 0x4a, 0x51, 0x58, 0x5f, 0x66; League::set_predicted_positions at the end.
/// Original: `SweFirst::SweFirst` (0x88e5e0, swe_first.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_swe_first(this: usize, year: u32, record: usize) -> usize {
    construct_league_with_stages(League(this), year, record, &SWEDISH_FIRST_DIVISION)
}

/// Comparer for `construct_swe_first`.
pub(crate) unsafe extern "thiscall" fn compare_construct_swe_first(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_SWE_FIRST);
    compare_league_with_stages_ctor("construct_swe_first", League(this), year, record, &SWEDISH_FIRST_DIVISION, || o(this, year, record))
}

/// Sets up the rules of the Swedish First Division (Superettan, 16 clubs): two legs, 3 points for a win, tie-breaks
/// goal difference and goals for; the top 2 go up, the 3rd plays the promotion play-off against the Allsvenskan's 12th
/// and the bottom 3 go down (to the Second Division when it is playable); 5 substitutes and 3 substitutions.
///
/// `SweFirst::set_up_rules()`: the writes of `swedish_first_rules` and the main-stage round table (virtual
/// `comp_vtable::CREATE_SCHEDULE`: push 0; push &+0x3a; push &+0xa9; push −1; mov ecx -> thiscall, ret 0x10) into
/// `league::ROUND_DATES`. Returns 1.
/// Original: `SweFirst::set_up_rules` (0x88f090, swe_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn swe_first_set_up_rules(this: usize) -> u32 {
    covered!("mwe_4 superettan rules");
    eng_prm_apply_league_rules(League(this), &swedish_first_rules(this))
}

/// Comparer for `swe_first_set_up_rules`: the dry run of `eng_prm_compare_league_rules`.
pub(crate) unsafe extern "thiscall" fn compare_swe_first_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SWE_FIRST_SET_UP_RULES);
    eng_prm_compare_league_rules("swe_first_set_up_rules", League(this), swedish_first_rules(this), || o(this), || swe_first_set_up_rules(this))
}

/// Creates the Swedish First Division (Superettan) table: 16 clubs, those of the division in database order; no prize
/// fund nor TV money.
///
/// `SweFirst::create_table()`: `.set_club_count()` = 16, the table (`exe::crt_malloc` of 0x3b0; out of memory: the box
/// of swe_first.cpp (0xa8b53c) line 0x144 and 0), the clubs of `KEY_CLUB_COMP_SWEDISH_FIRST_DIVISION` in database order
/// (`fill_first_division_rows`); returns 1 if it filled, otherwise 0.
/// Original: `SweFirst::create_table` (0x88ef90, swe_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn swe_first_create_table(this: usize) -> u32 {
    let this = League(this);
    this.set_club_count(0x10);
    if !allocate_first_division_table(this, 0x3b0, 0xa8b53c, 0x144) {
        return 0;
    }
    covered!("mwe_7 superettan table");
    fill_first_division_rows(this, |c| club_in_division(c, KEY_CLUB_COMP_SWEDISH_FIRST_DIVISION)) as u32 // mwe7: the Superettan clubs
}

/// Comparer for `swe_first_create_table`.
pub(crate) unsafe extern "thiscall" fn compare_swe_first_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SWE_FIRST_CREATE_TABLE);
    compare_first_division_table("swe_first_create_table", League(this), 0x10, |c| club_in_division(c, KEY_CLUB_COMP_SWEDISH_FIRST_DIVISION), || o(this))
}

/// The Superettan turnover: archives the season in the statistics, clears the table, the round dates and the stages,
/// moves to the next year and rebuilds the rules, the 16-club table, the fixtures and the order; then the predicted
/// positions, the board expectations and the referee renewal.
///
/// `SweFirst::start_new_season()`: `league::SEASON_CLOSED` = 0, the body of `rebuild_season` (checked instruction by
/// instruction; the table is `SweFirst::create_table` 0x88ef90 and the boxes of swe_first.cpp (0xa8b53c) at 0x1aa,
/// 0x1b1, 0x1b7, 0x1be) and `set_expectations_and_renew_officials`, whose eax is returned. After a box: 0.
/// Original: `SweFirst::start_new_season` (0x88f150, swe_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn swe_first_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    if !rebuild_season(this, 0x88ef90, 0xa8b53c, [0x1aa, 0x1b1, 0x1b7, 0x1be], 1) { // mwe9: the Superettan next year
        return 0;
    }
    covered!("mwe_9 superettan turnover");
    set_expectations_and_renew_officials(this)
}

/// Comparer for `swe_first_start_new_season`: only the original runs (it frees and allocates); the year must go up by
/// one (`compare_state_league_new_season`).
pub(crate) unsafe extern "thiscall" fn compare_swe_first_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SWE_FIRST_START_NEW_SEASON);
    compare_state_league_new_season("swe_first_start_new_season", League(this), || o(this))
}

/// Moves the Superettan to the next stage once the Allsvenskan is complete: the Allsvenskan's statuses are updated and,
/// on reaching stage 0, the promotion play-off is built; at the last stage, the error box (line 0x1da).
///
/// `SweFirst::advance_stage()`: `advance_first_division_stage` with swe_first.cpp (0xa8b53c), line 0x1da, the key
/// `KEY_CLUB_COMP_SWEDISH_PREMIER_DIVISION` and `SweFirst::create_play_off_stage` (0x88f3e0).
/// Original: `SweFirst::advance_stage` (0x88f310, swe_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn swe_first_advance_stage(this: usize) {
    advance_first_division_stage(League(this), 0xa8b53c, 0x1da, KEY_CLUB_COMP_SWEDISH_PREMIER_DIVISION, 0x88f3e0);
}

/// Builds the Superettan promotion play-off (stage 0): the Superettan club with the promotion play-off status (the
/// 3rd) against the Allsvenskan club with the relegation play-off status (the 12th), as a two-legged cup tie.
///
/// `SweFirst::create_play_off_stage()`: the list of 2 (`exe::crt_malloc` of 8; out of memory: the box of
/// swe_first.cpp (0xa8b53c) line 0x206 and return); slot 0 = the club of the first Superettan row with status
/// `table_status::PROMOTION_PLAYOFF`, slot 1 = the club of the first Allsvenskan row (`G_COMP_OBJECT_TABLE` by the id
/// of `KEY_CLUB_COMP_SWEDISH_PREMIER_DIVISION`) with status `table_status::RELEGATION_PLAYOFF` (a slot without such a
/// row keeps what malloc gave); the stage (`build_two_club_play_off`), then Cup::set_up_round (`exe::cup_set_up_round`
/// with 0) on the first slot, called before the slot is checked (a defect of the original: with no stage it runs on
/// a null object); with no stage, the box of line 0x22e; otherwise `.current_stage()` = 0.
/// Original: `SweFirst::create_play_off_stage` (0x88f3e0, swe_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn swe_first_create_play_off_stage(this: usize) {
    let this = League(this);
    let list = exe::crt_malloc(8);
    if list == 0 {
        return show_error_at(0xa8b53c, 0x206);
    }
    if let Some(c) = first_row_club_with_status(this, table_status::PROMOTION_PLAYOFF) { // mwe10: the Superettan play-off club
        write_usize(list, c);
    }
    if let Some(c) = premier_row_club_with_status(KEY_CLUB_COMP_SWEDISH_PREMIER_DIVISION, table_status::RELEGATION_PLAYOFF) {
        write_usize(list + 4, c);
    }
    covered!("mwe_10 superettan play-off");
    let stage = build_two_club_play_off(this, list);
    exe::cup_set_up_round(stage, 0);
    if read_usize(this.stages()) == 0 {
        return show_error_at(0xa8b53c, 0x22e);
    }
    this.set_current_stage(0);
}

/// Comparer for `swe_first_advance_stage`.
pub(crate) unsafe extern "thiscall" fn compare_swe_first_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_SWE_FIRST_ADVANCE_STAGE);
    compare_first_division_advance("swe_first_advance_stage", League(this), || o(this))
}

/// Comparer for `swe_first_create_play_off_stage`: only the original runs (it allocates and draws); the plan (the two
/// clubs, made beforehand) against the stage in the first slot: two clubs, the same as the plan (as a set), the
/// competition and `.current_stage()` = 0.
pub(crate) unsafe extern "thiscall" fn compare_swe_first_create_play_off_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_SWE_FIRST_CREATE_PLAY_OFF_STAGE);
    let l = League(this);
    let plan = [first_row_club_with_status(l, table_status::PROMOTION_PLAYOFF), premier_row_club_with_status(KEY_CLUB_COMP_SWEDISH_PREMIER_DIVISION, table_status::RELEGATION_PLAYOFF)];
    o(this);
    compare_two_club_play_off("swe_first_create_play_off_stage", l, plan);
}

/// The Superettan schedule: the league (stage −1) has 30 rounds (16 clubs, two legs), from 6 April to 8 November, the
/// Allsvenskan dates plus four Wednesday rounds; the promotion play-off (stage 0) is one two-legged tie from 15 November
/// (draw on 9 November).
///
/// `SweFirst::create_schedule(stage, &rounds, &extra, &league)`: stage −1 (0xff): `*league` = 1 (when given), 30 rounds,
/// extra 0, the table (`exe::crt_malloc` of 0x79e; out of memory: the box of swe_first.cpp (0xa8b53c) line 0xa8 and 0)
/// filled from `SUPERETTAN_ROUNDS` (`eng_first_set_league_rounds`). Stage 0: `*league` = 0 (when given), 1 round,
/// extra 0, the cup round table (`exe::crt_malloc` of 0x68; out of memory: the box of line 0x116 and 0) with round 0 =
/// `SUPERETTAN_PLAY_OFF` (Cup round draw date `exe::cup_set_round_draw_date` and match date
/// `exe::cup_set_round_match_date`, cdecl, with the year sign-extended; then the round fields). Any other stage: 0.
/// Original: `SweFirst::create_schedule` (0x88e8e0, swe_first.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn swe_first_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let this = League(this);
    match stage as u8 {
        0xff => {
            if !league.is_null() {
                *league = 1;
            }
            *rounds = 0x1e;
            *extra = 0;
            let t = exe::crt_malloc(0x79e);
            if t == 0 {
                show_error_at(0xa8b53c, 0xa8);
                return 0;
            }
            covered!("mwe_13 superettan schedule");
            eng_first_set_league_rounds(t, (this.year() as u16) as u32, 0, &SUPERETTAN_ROUNDS);
            t
        }
        0 => {
            if !league.is_null() {
                *league = 0;
            }
            *rounds = 1;
            *extra = 0;
            let t = exe::crt_malloc(0x68);
            if t == 0 {
                show_error_at(0xa8b53c, 0x116);
                return 0;
            }
            covered!("mwe_13 superettan play-off round");
            write_play_off_round(this, t, 0, &SUPERETTAN_PLAY_OFF);
            t
        }
        _ => 0,
    }
}

/// Comparer for `swe_first_create_schedule`: side by side (`compare_league_round_table`: the original with the caller's outputs, the copy
/// with its own; the outputs and the written round bytes equal; the copy's table freed).
pub(crate) unsafe extern "thiscall" fn compare_swe_first_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_SWE_FIRST_CREATE_SCHEDULE);
    compare_league_round_table("swe_first_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| swe_first_create_schedule(this, stage, r, e, l))
}

/// The rule writes of the Swedish First Division (Superettan, 16 clubs), in the original's order.
///
/// Two legs, 3 points for a win and 1 for a draw, league matches, no match rule flags and no round draw rules, club
/// sides; tie-breaks goal difference and goals for (the third code 0 ends the list); 2 promoted directly, 1 to the
/// promotion play-off (the 3rd, against the Allsvenskan's 12th), no relegation play-off and 3 relegated; the
/// Allsvenskan above and the Second Division below when the Swedish lower leagues are playable (bit 4), otherwise −1;
/// 3 substitutions, the squad flags 2 and 5 substitutes named.
pub(crate) unsafe fn swedish_first_rules(this: usize) -> Vec<(usize, u8, u32)> {
    let mut v = vec![(league::LEGS, 2, 2), (league::WIN_POINTS, 1, 3), (league::DRAW_POINTS, 1, 1), (league::MATCH_CATEGORY, 1, 2), (league::MATCH_RULES, 2, 0), (league::ROUND_DRAW_RULES, 2, 0), (league::CLUB_SIDES, 1, 1)];
    v.extend_from_slice(&[(league::TIEBREAKS, 1, 1), (league::TIEBREAKS + 1, 1, 2), (league::TIEBREAKS + 2, 1, 0)]);
    v.extend_from_slice(&[(league::PROMOTED_COUNT, 1, 2), (league::PROMOTION_PLAYOFF_COUNT, 1, 1), (league::RELEGATION_PLAYOFF_COUNT, 1, 0)]);
    v.push((league::RELEGATED_COUNT, 1, 3)); // mwe4: the Superettan relegates 3
    v.extend_from_slice(&[(league::UPPER_DIVISION_ID, 4, read_usize(KEY_CLUB_COMP_SWEDISH_PREMIER_DIVISION) as u32), (league::LOWER_DIVISION_ID, 4, lower_division_when_playable(this, KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION))]);
    v.extend_from_slice(&[(league::SUBSTITUTIONS_ALLOWED, 1, 3), (league::SQUAD_FLAGS, 1, 2), (league::SUBSTITUTES_NAMED, 1, 5)]);
    v
}

/// Fills the table rows with the clubs that `belongs` accepts, in database order, until `.club_count()` rows; returns
/// whether it filled.
///
/// The clubs of `G_CLUB_TABLE` (`club::SIZE` bytes, `G_CLUB_COUNT` of them, a signed word counter), each accepted one
/// into the next row (`exe::league_init_table_row`: push 0; push 0; push club; push row -> thiscall, ret 0x10; the row
/// is a byte), until the signed byte count equals `.club_count()`.
pub(crate) unsafe fn fill_first_division_rows(this: League, belongs: impl Fn(usize) -> bool) -> bool {
    let mut k: u8 = 0;
    let mut i: i16 = 0;
    let mut c = read_usize(G_CLUB_TABLE);
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        if belongs(c) {
            exe::league_init_table_row(this, k as u32, c, 0, 0);
            k = k.wrapping_add(1);
            if k as i8 as i16 == this.club_count() {
                break;
            }
        }
        i = i.wrapping_add(1);
        c = c.wrapping_add(club::SIZE);
    }
    k as i8 as i16 == this.club_count()
}

/// The stage advance of a First Division whose promotion play-off is against the Premier Division's play-off club.
///
/// At the last stage (`.current_stage()` ≥ `.stage_count()` − 1, signed), the error box of `file` at `row` and
/// nothing else. Otherwise, only when the Premier Division (key `premier`) is complete
/// (`premier_division_complete`): its positions and statuses are updated (`update_premier_division_status`), the
/// current stage goes up by one and, on reaching 0, the play-off is built (`play_off`, a direct call, thiscall, ret
/// 0). With the Premier Division still playing, nothing (the league asks again later).
pub(crate) unsafe fn advance_first_division_stage(this: League, file: usize, row: u32, premier: usize, play_off: usize) {
    if (this.current_stage() as i32) >= (this.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(file, row);
    }
    if !premier_division_complete(premier) { // mwe10: waits for the Premier Division
        return;
    }
    update_premier_division_status(premier); // mwe10: the Premier Division statuses brought up to date
    let stage = (this.current_stage() as i32).wrapping_add(1);
    this.set_current_stage(stage as usize);
    if stage == 0 {
        covered!("mwe_10 first division play-off reached");
        let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(play_off);
        f(this.addr());
    }
}

/// Builds the Cup_Stage of the play-off between the two clubs of `list` into the first stage slot and frees the list
/// and the round table; returns the stage left in the slot.
///
/// The round table of stage 0 (virtual `comp_vtable::CREATE_SCHEDULE`: push 0; push &extra; push &rounds; push 0;
/// mov ecx -> thiscall, ret 0x10); the stage (operator new `exe::crt_operator_new` of 0xb2; Cup_Stage::Cup_Stage
/// `FN_CUP_STAGE_CUP_STAGE`, thiscall, ret 0x3c: the league, 2, the list, the rounds, the competition, the round
/// table, the year, 0, 1, the extra, 0x14, 0, 0, 0, 0; 0 when out of memory) into the first slot of `.stages()`; the
/// list and the round table freed (`exe::crt_free`).
pub(crate) unsafe fn build_two_club_play_off(this: League, list: usize) -> usize {
    let table: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut extra) = (0i16, 0i16);
    let t = table(this.addr(), 0, &mut rounds, &mut extra, 0);
    let p = exe::crt_operator_new(0xb2);
    // simplification: the original pushes the rounds, the extra and the year as dwords with garbage above the word;
    // the constructor only reads the words.
    let stage = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construct(p, this.addr(), 2, list, rounds as u16 as u32, this.competition().addr(), t, this.year() as u16 as u32, 0, 1, extra as u16 as u32, 0x14, 0, 0, 0, 0)
    } else {
        0
    };
    write_usize(this.stages(), stage);
    exe::crt_free(list);
    exe::crt_free(t);
    read_usize(this.stages())
}

/// The club of the first table row (in table order, a signed word count) of `league` whose status is `status`.
pub(crate) unsafe fn first_row_club_with_status(league: League, status: u8) -> Option<usize> {
    let t = league.table();
    (0..(league.club_count() as i32).max(0) as usize).map(|i| t + i * league_table_row::SIZE).find(|&r| read_u8(r + league_table_row::STATUS) == status).map(|r| read_usize(r))
}

/// The club of the first row of the Premier Division of key `premier` (through League::get_table_row
/// `exe::league_get_table_row`, thiscall, ret 4) whose status is `status`.
pub(crate) unsafe fn premier_row_club_with_status(premier: usize, status: u8) -> Option<usize> {
    let p = League(comp_by_global(premier));
    for i in 0..(p.club_count() as i32).max(0) {
        if read_u8(exe::league_get_table_row(p, i) + league_table_row::STATUS) == status {
            return Some(read_usize(exe::league_get_table_row(p, i)));
        }
    }
    None
}

/// Checks the two-club play-off stage of the first slot against the plan (as a set, the round draw may swap them).
pub(crate) unsafe fn compare_two_club_play_off(name: &str, this: League, plan: [Option<usize>; 2]) {
    let stage = Cup(read_usize(this.stages()));
    if stage.is_null() {
        SCOREBOARD.record(false, || format!("{}({:#x}) without stage", name, this.addr()));
        return;
    }
    let list = stage.team_table();
    let mut found = [read_usize(list), read_usize(list + 6)];
    let mut want = [plan[0].unwrap_or(0), plan[1].unwrap_or(0)];
    found.sort_unstable();
    want.sort_unstable();
    let ok = plan.iter().all(|c| c.is_some()) && found == want && stage.team_count() == 2 && read_usize(stage.addr() + 4) == this.competition().addr() && this.current_stage() == 0;
    SCOREBOARD.record(ok, || format!("{}({:#x}) plan={:x?} stage={:#x} list={:x?} +0x3e={}", name, this.addr(), plan, stage.addr(), found, stage.team_count()));
}

/// The Swedish First Division (Superettan): discipline rule 28 (the Swedish league one), one stage after the league
/// (the promotion play-off), swe_first.cpp (0xa8b53c) lines 0x4a, 0x51, 0x58, 0x5f, 0x66.
pub(crate) const SWEDISH_FIRST_DIVISION: LeagueWithStages = LeagueWithStages {
    division: LeagueDivision { vtabela: 0x9700e4, discipline: discipline_rule::SWEDEN_LEAGUE, rules: 0x88f090, table: 0x88ef90, file: 0xa8b53c, rows: [0x4a, 0x51, 0x58, 0x5f, 0x66], low: false },
    stages: 1, // mwe2: the Swedish play-off stage
    finish: None,
};

/// The Superettan: the 30 rounds, the Allsvenskan dates (Sundays, weekday 6) with four Wednesday rounds (weekday 2,
/// code 2) in August, September and October; no TV slots.
pub(crate) const SUPERETTAN_ROUNDS: [EngFirstScheduleRound; 30] = [
    (6, 3, 0, 6, 1, &[(-1, -1, -1)]), // 0: 6/abr
    (13, 3, 0, 6, 1, &[(-1, -1, -1)]), // 1: 13/abr
    (20, 3, 0, 6, 1, &[(-1, -1, -1)]), // 2: 20/abr
    (27, 3, 0, 6, 1, &[(-1, -1, -1)]), // 3: 27/abr
    (4, 4, 0, 6, 1, &[(-1, -1, -1)]), // 4: 4/mai
    (11, 4, 0, 6, 1, &[(-1, -1, -1)]), // 5: 11/mai
    (18, 4, 0, 6, 1, &[(-1, -1, -1)]), // 6: 18/mai
    (2, 5, 0, 1, 2, &[(-1, -1, -1)]), // 7: 2/jun
    (7, 5, 0, 6, 1, &[(-1, -1, -1)]), // 8: 7/jun
    (15, 5, 0, 6, 1, &[(-1, -1, -1)]), // 9: 15/jun
    (22, 5, 0, 6, 1, &[(-1, -1, -1)]), // 10: 22/jun
    (29, 5, 0, 6, 1, &[(-1, -1, -1)]), // 11: 29/jun
    (19, 6, 0, 6, 1, &[(-1, -1, -1)]), // 12: 19/jul (mwe13: the Superettan July round)
    (2, 7, 0, 6, 1, &[(-1, -1, -1)]), // 13: 2/ago
    (9, 7, 0, 6, 1, &[(-1, -1, -1)]), // 14: 9/ago
    (16, 7, 0, 6, 1, &[(-1, -1, -1)]), // 15: 16/ago
    (23, 7, 0, 6, 1, &[(-1, -1, -1)]), // 16: 23/ago
    (26, 7, 0, 2, 2, &[(-1, -1, -1)]), // 17: 26/ago
    (30, 7, 0, 6, 1, &[(-1, -1, -1)]), // 18: 30/ago
    (13, 8, 0, 6, 1, &[(-1, -1, -1)]), // 19: 13/set
    (16, 8, 0, 2, 2, &[(-1, -1, -1)]), // 20: 16/set
    (20, 8, 0, 6, 1, &[(-1, -1, -1)]), // 21: 20/set
    (27, 8, 0, 6, 1, &[(-1, -1, -1)]), // 22: 27/set
    (30, 8, 0, 2, 2, &[(-1, -1, -1)]), // 23: 30/set
    (4, 9, 0, 6, 1, &[(-1, -1, -1)]), // 24: 4/out
    (18, 9, 0, 6, 1, &[(-1, -1, -1)]), // 25: 18/out
    (21, 9, 0, 2, 2, &[(-1, -1, -1)]), // 26: 21/out
    (25, 9, 0, 6, 1, &[(-1, -1, -1)]), // 27: 25/out
    (31, 9, 0, 5, 1, &[(-1, -1, -1)]), // 28: 31/out
    (8, 10, 0, 6, 1, &[(-1, -1, -1)]), // 29: 8/nov
];

/// The Superettan promotion play-off round (stage 0): draw on 9 November, one tie (two clubs, two legs, 7 days apart)
/// from 15 November (Sunday), code 0xa0, category 5, the two-leg tie rules 3, no prizes.
pub(crate) const SUPERETTAN_PLAY_OFF: EngCcCupCupRound = cql10r((9, 0xa, 0, 0), (0xf, 0xa, 0, 6, 1, 0), 0xa0, (0, 0, 3), 5, (2, 1, 2, 0), (0, 2, 7), (0, 0, 0)); // mwe13: the Superettan play-off dates
