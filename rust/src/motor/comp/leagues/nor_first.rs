use super::*;

/// Builds the Norwegian First Division: discipline rule 21 (the Norwegian league one) and one stage after the league
/// (the promotion play-off); with no saved state, the rules, the table, the fixtures, the order, the stats and the
/// predicted positions.
///
/// `NorFirst::NorFirst(year, record)`: `construct_league_with_stages` with vtable 0x96de20, discipline byte 0x15, one
/// extra stage (malloc of 4), `NorFirst::set_up_rules` (0x790b00) and `NorFirst::create_table` (0x7909a0); the boxes of
/// nor_first.cpp lines 0x49, 0x50, 0x57, 0x60, 0x67; League::set_predicted_positions at the end.
/// Original: `NorFirst::NorFirst` (0x78fd40, nor_first.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_nor_first(this: usize, year: u32, record: usize) -> usize {
    construct_league_with_stages(League(this), year, record, &NORWEGIAN_FIRST_DIVISION)
}

/// Comparer for `construct_nor_first`.
pub(crate) unsafe extern "thiscall" fn compare_construct_nor_first(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_NOR_FIRST);
    compare_league_with_stages_ctor("construct_nor_first", League(this), year, record, &NORWEGIAN_FIRST_DIVISION, || o(this, year, record))
}

/// Sets up the rules of the Norwegian First Division (16 clubs): two legs, 3 points for a win; the top 2 go up, the
/// 3rd plays the promotion play-off against the Premier Division's 12th and the bottom 4 go down; 7 substitutes and 3
/// substitutions.
///
/// `NorFirst::set_up_rules()`: the writes of `norwegian_first_rules` (the bands 2 / 1 / 0 / 4, the Premier Division
/// above, nothing below) and the main-stage round table (virtual `comp_vtable::CREATE_SCHEDULE`) into
/// `league::ROUND_DATES`. Returns 1.
/// Original: `NorFirst::set_up_rules` (0x790b00, nor_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nor_first_set_up_rules(this: usize) -> u32 {
    covered!("mwe_3 norwegian first division rules");
    eng_prm_apply_league_rules(League(this), &norwegian_first_rules())
}

/// Comparer for `nor_first_set_up_rules`.
pub(crate) unsafe extern "thiscall" fn compare_nor_first_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_NOR_FIRST_SET_UP_RULES);
    eng_prm_compare_league_rules("nor_first_set_up_rules", League(this), norwegian_first_rules(), || o(this), || nor_first_set_up_rules(this))
}

/// Creates the Norwegian First Division table: 16 clubs; when the database has exactly 14 clubs in the division, 14
/// clubs and only 2 relegated; those of the division in database order; no prize fund nor TV money.
///
/// `NorFirst::create_table()`: `.set_club_count()` = 16 and the table (`exe::crt_malloc` of 0x3b0, the size of 16
/// rows in any case; out of memory: the box of nor_first.cpp (0xa69740) line 0x251 and 0); the division's clubs counted
/// over the whole database (`norwegian_first_division_count`, a byte); with exactly 14: `.set_club_count()` = 14,
/// `league::RELEGATED_COUNT` = 2 and `league::GAMES_PER_CLUB` = −1 (to be recomputed); then the clubs of
/// `KEY_CLUB_COMP_NORWEGIAN_FIRST_DIVISION` in database order; returns 1 if it filled, otherwise 0.
/// Original: `NorFirst::create_table` (0x7909a0, nor_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nor_first_create_table(this: usize) -> u32 {
    let this = League(this);
    this.set_club_count(0x10);
    if !allocate_first_division_table(this, 0x3b0, 0xa69740, 0x251) {
        return 0;
    }
    if norwegian_first_division_count() == 0xe { // mwe7: the 14-club Norwegian First Division
        covered!("mwe_7 norwegian first division of 14");
        this.set_club_count(0xe);
        this.set_relegated_count(2);
        this.set_games_per_club(0xffffu16 as i16);
    }
    covered!("mwe_7 norwegian first division table");
    fill_first_division_rows(this, |c| club_in_division(c, KEY_CLUB_COMP_NORWEGIAN_FIRST_DIVISION)) as u32
}

/// Comparer for `nor_first_create_table`: the size by the count of the division's clubs (14 when exactly 14,
/// otherwise 16), and with 14 also `league::RELEGATED_COUNT` = 2.
pub(crate) unsafe extern "thiscall" fn compare_nor_first_create_table(this: usize) -> u32 {
    let this = League(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_NOR_FIRST_CREATE_TABLE);
    let fourteen = norwegian_first_division_count() == 0xe;
    let r = compare_first_division_table("nor_first_create_table", this, if fourteen { 0xe } else { 0x10 }, |c| club_in_division(c, KEY_CLUB_COMP_NORWEGIAN_FIRST_DIVISION), || o(this.addr()));
    if fourteen {
        let ok = this.relegated_count() == 2;
        SCOREBOARD.record(ok, || format!("nor_first_create_table({:#x}) 14 clubs, relegated {}", this.addr(), this.relegated_count()));
    }
    r
}

/// The Norwegian First Division turnover: the same as the Superettan's, with `NorFirst::create_table` (0x7909a0) and the
/// boxes of nor_first.cpp (0xa69740) at 0x2c9, 0x2d0, 0x2d6, 0x2dd.
/// Original: `NorFirst::start_new_season` (0x790b90, nor_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nor_first_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    if !rebuild_season(this, 0x7909a0, 0xa69740, [0x2c9, 0x2d0, 0x2d6, 0x2dd], 1) { // mwe9: the Norwegian First Division next year
        return 0;
    }
    covered!("mwe_9 norwegian first division turnover");
    set_expectations_and_renew_officials(this)
}

/// Comparer for `nor_first_start_new_season`.
pub(crate) unsafe extern "thiscall" fn compare_nor_first_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_NOR_FIRST_START_NEW_SEASON);
    compare_state_league_new_season("nor_first_start_new_season", League(this), || o(this))
}

/// Moves the Norwegian First Division to the next stage once the Tippeligaen is complete: the same as the Superettan's
/// (nor_first.cpp line 0x2fa, `KEY_CLUB_COMP_NORWEGIAN_PREMIER_DIVISION`, `NorFirst::create_play_off_stage` 0x790e20).
/// Original: `NorFirst::advance_stage` (0x790d50, nor_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nor_first_advance_stage(this: usize) {
    advance_first_division_stage(League(this), 0xa69740, 0x2fa, KEY_CLUB_COMP_NORWEGIAN_PREMIER_DIVISION, 0x790e20);
}

/// Comparer for `nor_first_advance_stage`.
pub(crate) unsafe extern "thiscall" fn compare_nor_first_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_NOR_FIRST_ADVANCE_STAGE);
    compare_first_division_advance("nor_first_advance_stage", League(this), || o(this))
}

/// The Norwegian promotion play-off (stage 0): the Norwegian First Division club with the promotion play-off status
/// (the 3rd) against the Tippeligaen club with the relegation play-off status (the 12th), as a cup tie.
///
/// `NorFirst::create_play_off_stage()`: the list of 2 (`exe::crt_malloc` of 8; out of memory: the box of nor_first.cpp
/// (0xa69740) line 0x326 and return). Slot 0 = the club of the LAST First Division row with status
/// `table_status::PROMOTION_PLAYOFF` (the loop does not stop at the first; the count re-read each turn); none: the box
/// of line 0x336 and return (the list is not freed). Slot 1 = the club of the last Tippeligaen row
/// (`G_COMP_OBJECT_TABLE` by the id of `KEY_CLUB_COMP_NORWEGIAN_PREMIER_DIVISION`, League::get_table_row
/// `exe::league_get_table_row`, called twice for the chosen row) with status `table_status::RELEGATION_PLAYOFF`; a
/// Tippeligaen without clubs, or with no such row, ends it silently (the list is not freed). Then the stage
/// (`build_two_club_play_off`: the round table of stage 0, the Cup_Stage of the 2 clubs into the first slot, the list
/// and the round table freed); with no stage, the box of line 0x357; otherwise Cup::set_up_round
/// (`exe::cup_set_up_round` with 0). Unlike the Superettan's, it does not write `.current_stage()` (the advance already
/// set it to 0).
/// Original: `NorFirst::create_play_off_stage` (0x790e20, nor_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nor_first_create_play_off_stage(this: usize) {
    let this = League(this);
    let list = exe::crt_malloc(8);
    if list == 0 {
        return show_error_at(0xa69740, 0x326);
    }
    let Some(c) = last_row_club_with_status(this, table_status::PROMOTION_PLAYOFF) else { // mwe11: the Norwegian play-off club
        return show_error_at(0xa69740, 0x336);
    };
    write_usize(list, c);
    let Some(c) = last_premier_row_club_with_status(KEY_CLUB_COMP_NORWEGIAN_PREMIER_DIVISION, table_status::RELEGATION_PLAYOFF) else {
        return;
    };
    write_usize(list + 4, c);
    covered!("mwe_11 norwegian play-off");
    let stage = build_norwegian_play_off(this, list);
    if stage == 0 {
        return show_error_at(0xa69740, 0x357);
    }
    exe::cup_set_up_round(stage, 0);
}

/// Comparer for `nor_first_create_play_off_stage`: only the original runs (it allocates and draws); the planned
/// clubs (the last rows with each status) against the stage, when both exist.
pub(crate) unsafe extern "thiscall" fn compare_nor_first_create_play_off_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_NOR_FIRST_CREATE_PLAY_OFF_STAGE);
    let l = League(this);
    let plan = [last_row_club_with_status(l, table_status::PROMOTION_PLAYOFF), last_premier_row_club_with_status(KEY_CLUB_COMP_NORWEGIAN_PREMIER_DIVISION, table_status::RELEGATION_PLAYOFF)];
    o(this);
    if plan.iter().all(|c| c.is_some()) {
        let stage = Cup(read_usize(l.stages()));
        let ok = !stage.is_null() && stage.team_count() == 2 && {
            let list = stage.team_table();
            let mut found = [read_usize(list), read_usize(list + 6)];
            let mut want = [plan[0].unwrap(), plan[1].unwrap()];
            found.sort_unstable();
            want.sort_unstable();
            found == want
        };
        SCOREBOARD.record(ok, || format!("nor_first_create_play_off_stage({:#x}) plan={:x?} stage={:#x}", this, plan, stage.addr()));
    }
}

/// The rule writes of the Norwegian First Division (Adeccoligaen, 16 clubs; 14 when the database has only 14), in the
/// original's order.
///
/// Two legs, 3 points for a win and 1 for a draw, league matches, club sides; tie-breaks goal difference, goals for and
/// code 3; 2 promoted directly and 1 to the promotion play-off (the 3rd against the Premier Division's 12th), no
/// relegation play-off and 4 relegated (no division below: −1); the Premier Division above; 3 substitutions, the squad
/// flags 2 and 7 substitutes named. It does not touch the match rule flags nor the round draw rules.
pub(crate) unsafe fn norwegian_first_rules() -> Vec<(usize, u8, u32)> {
    let mut v = vec![(league::LEGS, 2, 2), (league::WIN_POINTS, 1, 3), (league::DRAW_POINTS, 1, 1), (league::MATCH_CATEGORY, 1, 2), (league::CLUB_SIDES, 1, 1)];
    v.extend_from_slice(&[(league::TIEBREAKS, 1, 1), (league::TIEBREAKS + 1, 1, 2), (league::TIEBREAKS + 2, 1, 3)]);
    v.push((league::PROMOTED_COUNT, 1, 2)); // mwe3: the Norwegian First Division promotes 2
    v.extend_from_slice(&[(league::PROMOTION_PLAYOFF_COUNT, 1, 1), (league::RELEGATION_PLAYOFF_COUNT, 1, 0), (league::RELEGATED_COUNT, 1, 4)]);
    v.extend_from_slice(&[(league::SUBSTITUTIONS_ALLOWED, 1, 3), (league::UPPER_DIVISION_ID, 4, read_usize(KEY_CLUB_COMP_NORWEGIAN_PREMIER_DIVISION) as u32), (league::SQUAD_FLAGS, 1, 2)]);
    v.extend_from_slice(&[(league::LOWER_DIVISION_ID, 4, 0xffff_ffff), (league::SUBSTITUTES_NAMED, 1, 7)]);
    v
}

/// True if the club's division (`club::DIVISION`) is the competition of id `g`.
pub(crate) unsafe fn club_in_division(c: usize, g: usize) -> bool {
    let c = Club(c);
    let comp = c.division().addr();
    comp != 0 && read_usize(comp) == read_usize(g)
}

/// How many clubs of the Norwegian First Division the database has (a byte counter over all the clubs).
pub(crate) unsafe fn norwegian_first_division_count() -> u8 {
    let mut n: u8 = 0;
    let mut i: i16 = 0;
    let mut c = read_usize(G_CLUB_TABLE);
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        if club_in_division(c, KEY_CLUB_COMP_NORWEGIAN_FIRST_DIVISION) {
            n = n.wrapping_add(1);
        }
        i = i.wrapping_add(1);
        c = c.wrapping_add(club::SIZE);
    }
    n
}

/// The club of the last table row (a signed word count, re-read each turn) of `league` whose status is `status`.
pub(crate) unsafe fn last_row_club_with_status(league: League, status: u8) -> Option<usize> {
    let mut found = None;
    let mut i = 0i32;
    while i < league.club_count() as i32 {
        let r = league.table() + i as usize * league_table_row::SIZE;
        if read_u8(r + league_table_row::STATUS) == status {
            found = Some(read_usize(r));
        }
        i += 1;
    }
    found
}

/// The club of the last row of the Premier Division of key `premier` (League::get_table_row, twice for the chosen row)
/// whose status is `status`; None without clubs or without such a row.
pub(crate) unsafe fn last_premier_row_club_with_status(premier: usize, status: u8) -> Option<usize> {
    let p = League(comp_by_global(premier));
    let mut found = None;
    let mut i = 0i32;
    while i < p.club_count() as i32 {
        if read_u8(exe::league_get_table_row(p, i) + league_table_row::STATUS) == status {
            found = Some(read_usize(exe::league_get_table_row(p, i)));
        }
        i += 1;
    }
    found
}

/// The Cup_Stage of the two clubs of `list` into the first stage slot (the same steps as the Superettan's: the round
/// table of stage 0 by virtual `comp_vtable::CREATE_SCHEDULE`, operator new of 0xb2, Cup_Stage::Cup_Stage(league, 2,
/// list, rounds, competition, round table, year, 0, 1, extra, 0x14, 0, 0, 0, 0), the slot written, the list and the
/// round table freed); returns the slot.
pub(crate) unsafe fn build_norwegian_play_off(this: League, list: usize) -> usize {
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

/// The Norwegian First Division: discipline rule 21, one stage after the league (the promotion play-off against the
/// Premier Division), nor_first.cpp (0xa69740) lines 0x49, 0x50, 0x57, 0x60, 0x67.
pub(crate) const NORWEGIAN_FIRST_DIVISION: LeagueWithStages = LeagueWithStages {
    division: LeagueDivision { vtabela: 0x96de20, discipline: discipline_rule::NORWAY_LEAGUE, rules: 0x790b00, table: 0x7909a0, file: 0xa69740, rows: [0x49, 0x50, 0x57, 0x60, 0x67], low: false },
    stages: 1, // mwe2: the Norwegian play-off stage
    finish: None,
};
