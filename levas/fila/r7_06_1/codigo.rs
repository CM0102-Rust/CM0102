// ---------- r7_06_1: the fixed matches of the 2002 World Cup qualifying campaigns and the South American one (wc_europe_league.cpp, wc_south_american_league.cpp) ----------

/// One fixed 2002 South American fixture: (day, month from 0, year, home nation key, home goals, away nation key, away
/// goals); goals −1: not played.
pub(crate) type Fixed2002Fixture = (u8, u8, u16, usize, i8, usize, i8);

/// WcSouthAmericanLeague::add_fixed_fixture (thiscall, ret 0x20; hooked: the copy below), called by the campaign load.
pub(crate) const FN_WC_SOUTH_AMERICAN_LEAGUE_ADD_FIXED_FIXTURE: usize = 0x922350;

/// Builds one fixed 2002 qualifying match of section `league` in record `r` (zeroed, `fixture::SIZE` bytes) and adds it
/// to the calendar; true when it carries a result (home goals not −1).
///
/// The record: `fixture::COMP_ID` and `fixture::COMPETITION` from the section's competition, the stats slot −1, the
/// season year (`year()`), round name 0, round prefix `stage_id`, round number 0, the round rules, tie rules and
/// category of the section (`round_draw_rules()`, `match_rules()`, `match_category()`), stage 0xff, result marks 0,
/// side marks 1, stage index `stage_index`, the goals given (0xff: none) and the half-time, extra-time, shoot-out and
/// aggregate bytes 0xff; the squad rules: substitutes named (`substitutes_named()`, low nibble) and substitutions
/// allowed (`substitutions_allowed()`, next nibble), bits 8..11 cleared. The date (Date::Date `exe::date_date_549f70`
/// of day, month, year, −1) as the match date and the next round date; the time slice 1 on weekday 5 or 6
/// (`exe::date_get_weekday`, asked a second time when the first is not 5), else 2. The clubs are the national teams of
/// the nation ids `home` and `away` (`national_team_of_id`; their ids read with no null check). The venue
/// (`exe::stadium_set_fixture_venue` of `G_STADIUM`: venue 0, the record, stadium −1). A date after the game's start
/// date (`G_GAME_START_DATE`: a later year, unsigned, or the same year and a later day, signed) clears both goals to
/// 0xff. Then `exe::fixture_manager_add_fixture`(record, 1).
///
/// Defect of the original: it reads the squad-rules word of its stack record before writing it and keeps bits 12..15
/// (the high nibble of `fixture::FLAGS`) from that uninitialised stack; the copy leaves them 0.
pub(crate) unsafe fn add_fixed_qualifying_fixture(league: League, r: Fixture, stage_id: i16, stage_index: u8, day: u32, month: u32, year: u32, home: u32,
    home_goals: u32, away: u32, away_goals: u32) -> bool {
    let competition = league.competition().addr();
    r.set_comp_id(read_i32(competition));
    r.set_competition(Competition(competition));
    r.set_match_stats_slot(u32::MAX as usize);
    r.set_season_year(league.year() as u16);
    r.set_round_name(round_name::NONE);
    r.set_round_prefix(stage_id);
    r.set_round_number(0);
    r.set_round_draw_rules(league.round_draw_rules() as u16);
    r.set_tie_rules(league.match_rules() as u16);
    r.set_category(league.match_category());
    r.set_stage(0xff);
    r.set_home_result_mark(result_mark::NONE);
    r.set_away_result_mark(result_mark::NONE);
    r.set_home_side_mark(1);
    r.set_away_side_mark(1);
    r.set_stage_index(stage_index);
    r.set_home_goals(home_goals as u8 as i8);
    r.set_away_goals(away_goals as u8 as i8);
    for k in fixture::HOME_GOALS_HALF_TIME..=fixture::AWAY_AGGREGATE {
        write_u8(r.addr() + k, 0xff);
    }
    r.set_squad_rules((league.substitutes_named() & 0xf) as u16 | ((league.substitutions_allowed() & 0xf) as u16) << 4);
    let mut date = [0u16; 4];
    exe::date_date_549f70(&mut date, day, month, year, u32::MAX);
    r.set_day(date[0]);
    r.set_year(date[1]);
    let weekend = exe::date_get_weekday(date.as_mut_ptr() as usize) as i16 == 5 || exe::date_get_weekday(date.as_mut_ptr() as usize) as i16 == 6;
    r.set_time_slice(if weekend { 1 } else { 2 }); // r7_06_1: weekend slice
    r.set_next_round_day(date[0]);
    r.set_next_round_year(date[1]);
    let (home_team, away_team) = (Club(national_team_of_id(home as usize)), Club(national_team_of_id(away as usize)));
    r.set_home_club(home_team);
    r.set_away_club(away_team);
    r.set_home_club_id(read_usize(home_team.addr()));
    r.set_away_club_id(read_usize(away_team.addr()));
    exe::stadium_set_fixture_venue(read_usize(G_STADIUM), 0usize, r.addr(), u32::MAX as usize);
    let (start_day, start_year) = (read_i16(G_GAME_START_DATE), read_u16(G_GAME_START_DATE + 2));
    if start_year < date[1] || (start_year == date[1] && start_day < date[0] as i16) {
        covered!("r7_06_1 fixed match still to play");
        r.set_home_goals(-1);
        r.set_away_goals(-1); // r7_06_1: future match without goals
    }
    exe::fixture_manager_add_fixture(read_usize(G_FIXTURE_MANAGER), r.addr(), 1);
    r.home_goals() != -1
}

/// The section's result method (`comp_vtable::ADD_RESULT`: thiscall(record, 0)) on a fixed match.
unsafe fn add_fixed_match_result(league: League, r: Fixture) {
    let add: unsafe extern "thiscall" fn(usize, usize, u32) -> i32 = core::mem::transmute(vtable_method(league, comp_vtable::ADD_RESULT));
    add(league.addr(), r.addr(), 0);
}

/// rule: a historical European qualifying match of the 2002 campaign goes into the calendar with its score; when it was
/// already played by the game's start date, its group table takes the result, and the runners-up table is redone
///
/// `this.add_fixed_result(stage, day, month, year, home, home_goals, away, away_goals)`, with stage a signed byte (−1:
/// the section's own group). Without a stage vector (`stages()`), with a stage after the current one
/// (`current_stage()`, signed) or a missing stage object: the error box of line 0xa32. A year outside 2000..=2001
/// (unsigned word) or a stage outside −1..=7: the box of line 0xa39. Otherwise the match
/// (`add_fixed_qualifying_fixture`, the round prefix the `stage_id()` of the stage object, or of the section for −1,
/// stage index the stage); with a result, the section's `comp_vtable::ADD_RESULT` and the stage's table sorted
/// (`exe::league_sort_table`, rows 0). In both cases the runners-up table filled again
/// (`exe::wceuropeleague_fill_runners_up_table`).
/// Original: `WcEuropeLeague::add_fixed_result` (0x91db60, wc_europe_league.cpp, thiscall, ret 0x20).
pub(crate) unsafe extern "thiscall" fn wc_europe_league_add_fixed_result(this: usize, stage: u32, day: u32, month: u32, year: u32, home: u32, home_goals: u32,
    away: u32, away_goals: u32) {
    let league = League(this);
    let stage = stage as u8 as i8;
    let stage_object = |s: i8| read_usize(league.stages().wrapping_add((s as isize as usize).wrapping_mul(4)));
    if league.stages() == 0 || stage as i32 > league.current_stage() as i32 || (stage != -1 && stage_object(stage) == 0) {
        show_error_at(wc_cup_turnover_consts::WC_EUROPE_LEAGUE_CPP, 0xa32);
        return;
    }
    if !(0x7d0..=0x7d1).contains(&(year as u16)) || !(-1..=7).contains(&stage) {
        show_error_at(wc_cup_turnover_consts::WC_EUROPE_LEAGUE_CPP, 0xa39);
        return;
    }
    covered!("r7_06_1 european fixed match");
    let stage_id = League(if stage == -1 { this } else { stage_object(stage) }).stage_id();
    let mut record = [0u8; 0x60];
    let r = Fixture(record.as_mut_ptr() as usize);
    if add_fixed_qualifying_fixture(league, r, stage_id, stage as u8, day, month, year, home, home_goals, away, away_goals) {
        add_fixed_match_result(league, r);
        exe::league_sort_table(if stage == -1 { this } else { stage_object(stage) }, 0usize); // r7_06_1: the group table sorted
    }
    exe::wceuropeleague_fill_runners_up_table(this);
}

/// rule: a historical South American qualifying fixture of the 2002 campaign goes into the calendar with its score; when
/// it was already played by the game's start date, the table takes the result
///
/// `this.add_fixed_fixture(stage, day, month, year, home, home_goals, away, away_goals)`, with stage a signed byte. A
/// stage after the current one (`current_stage()`, signed) or a missing stage object (the stage vector read with no null
/// check): the error box of line 0x3c7. A year outside 2000..=2001 (unsigned word) or a stage other than −1: the box of
/// line 0x3ce. Otherwise the fixture (`add_fixed_qualifying_fixture`: round prefix the section's `stage_id()`, stage
/// index 0xff); with a result, the section's `comp_vtable::ADD_RESULT` and its table sorted (`exe::league_sort_table`,
/// rows 0: the stage index 0xff always names the section's own table).
/// Original: `WcSouthAmericanLeague::add_fixed_fixture` (0x922350, wc_south_american_league.cpp, thiscall, ret 0x20).
pub(crate) unsafe extern "thiscall" fn wc_south_american_league_add_fixed_fixture(this: usize, stage: u32, day: u32, month: u32, year: u32, home: u32,
    home_goals: u32, away: u32, away_goals: u32) {
    let league = League(this);
    let stage = stage as u8 as i8;
    if stage as i32 > league.current_stage() as i32 || (stage != -1 && read_usize(league.stages().wrapping_add((stage as isize as usize).wrapping_mul(4))) == 0) {
        show_error_at(wc_turnover_consts::WC_SOUTH_AMERICAN_LEAGUE_CPP, 0x3c7);
        return;
    }
    if !(0x7d0..=0x7d1).contains(&(year as u16)) || stage != -1 {
        show_error_at(wc_turnover_consts::WC_SOUTH_AMERICAN_LEAGUE_CPP, 0x3ce);
        return;
    }
    covered!("r7_06_1 south american fixed fixture");
    let mut record = [0u8; 0x60];
    let r = Fixture(record.as_mut_ptr() as usize);
    if add_fixed_qualifying_fixture(league, r, league.stage_id(), 0xff, day, month, year, home, home_goals, away, away_goals) {
        add_fixed_match_result(league, r);
        exe::league_sort_table(this, 0usize);
    }
}

/// rule: the South American qualifying of a game whose year has reached the campaign year loads the real 2002 campaign:
/// the 90 fixtures of March 2000 to November 2001 with the scores known when the game was made
///
/// The predicted positions (`comp_vtable::SET_PREDICTED_POSITIONS`), the section active (`set_active(1)`), then each
/// fixture of `WC_SOUTH_AMERICA_2002_RESULTS` in order through WcSouthAmericanLeague::add_fixed_fixture (stage −1; the
/// nation ids read from the keys at each call; goals −1 pushed as −1). Only the constructor calls it, when the game's
/// year is not before the campaign year, which is never before 2004: with the game starting in 2001 it never runs.
/// Defects of the original: dead code in the game as it is (the 2004 floor of the campaign year); with a start year
/// patched to one after a campaign year (e.g. 2005), the 2000-2001 fixtures go into a later campaign, the played ones
/// into its table and the 15 without a score stay in the calendar, in the past, never played.
/// Original: `WcSouthAmericanLeague::load_2002_campaign` (0x9216d0, wc_south_american_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_south_american_league_load_2002_campaign(this: usize) {
    let league = League(this);
    let predicted: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(vtable_method(league, comp_vtable::SET_PREDICTED_POSITIONS));
    predicted(this);
    league.set_active(1);
    covered!("r7_06_1 south american 2002 campaign");
    let add: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32) = core::mem::transmute(FN_WC_SOUTH_AMERICAN_LEAGUE_ADD_FIXED_FIXTURE);
    for &(day, month, year, home, home_goals, away, away_goals) in WC_SOUTH_AMERICA_2002_RESULTS.iter() {
        add(this, u32::MAX, day as u32, month as u32, year as u32, read_u32(home), home_goals as i32 as u32, read_u32(away), away_goals as i32 as u32);
    }
}

/// Comparers: the three add to the calendar and the tables (they cannot run twice), so only the original runs. They
/// never run in the game as it is (the 2002 campaign is dead code); the copies were checked against the originals by
/// emulation. The campaign load is then checked on its plan: the section active.
pub(crate) unsafe extern "thiscall" fn compare_wc_europe_league_add_fixed_result(this: usize, stage: u32, day: u32, month: u32, year: u32, home: u32,
    home_goals: u32, away: u32, away_goals: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32)>(%%I0%%)(this, stage, day, month, year, home, home_goals, away, away_goals)
}

pub(crate) unsafe extern "thiscall" fn compare_wc_south_american_league_add_fixed_fixture(this: usize, stage: u32, day: u32, month: u32, year: u32, home: u32,
    home_goals: u32, away: u32, away_goals: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32)>(%%I1%%)(this, stage, day, month, year, home, home_goals, away, away_goals)
}

pub(crate) unsafe extern "thiscall" fn compare_wc_south_american_league_load_2002_campaign(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this);
    SCOREBOARD.record(League(this).active() == 1, || format!("wc_south_american_league_load_2002_campaign({:#x}): active {}", this, League(this).active()));
}

/// The 90 South American matches of the 2002 campaign (28 March 2000 to 14 November 2001), in the original's order:
/// (day; month from 0; year; home nation key; its goals; away nation key; its goals); all in stage −1 (the section's
/// own table). Goals −1: the last three rounds (6 October, 7 and 14 November 2001), not played yet when the data was made.
pub(crate) const WC_SOUTH_AMERICA_2002_RESULTS: [Fixed2002Fixture; 90] = [
    (28, 2, 2000, KEY_NATION_COLOMBIA, 0, KEY_NATION_BRAZIL, 0), // r7_06_1: Colombia-Brazil
    (29, 2, 2000, KEY_NATION_ECUADOR, 2, KEY_NATION_VENEZUELA, 0),
    (29, 2, 2000, KEY_NATION_URUGUAY, 1, KEY_NATION_BOLIVIA, 0),
    (29, 2, 2000, KEY_NATION_PERU, 2, KEY_NATION_PARAGUAY, 0),
    (29, 2, 2000, KEY_NATION_ARGENTINA, 4, KEY_NATION_CHILE, 1),
    (26, 3, 2000, KEY_NATION_BOLIVIA, 1, KEY_NATION_COLOMBIA, 1),
    (26, 3, 2000, KEY_NATION_PARAGUAY, 1, KEY_NATION_URUGUAY, 0),
    (26, 3, 2000, KEY_NATION_VENEZUELA, 0, KEY_NATION_ARGENTINA, 4),
    (26, 3, 2000, KEY_NATION_CHILE, 1, KEY_NATION_PERU, 1),
    (26, 3, 2000, KEY_NATION_BRAZIL, 3, KEY_NATION_ECUADOR, 2),
    (3, 5, 2000, KEY_NATION_URUGUAY, 2, KEY_NATION_CHILE, 1),
    (3, 5, 2000, KEY_NATION_PARAGUAY, 3, KEY_NATION_ECUADOR, 1),
    (4, 5, 2000, KEY_NATION_ARGENTINA, 1, KEY_NATION_BOLIVIA, 0),
    (4, 5, 2000, KEY_NATION_PERU, 0, KEY_NATION_BRAZIL, 1),
    (4, 5, 2000, KEY_NATION_COLOMBIA, 3, KEY_NATION_VENEZUELA, 0),
    (28, 5, 2000, KEY_NATION_VENEZUELA, 4, KEY_NATION_BOLIVIA, 2),
    (28, 5, 2000, KEY_NATION_BRAZIL, 1, KEY_NATION_URUGUAY, 1),
    (29, 5, 2000, KEY_NATION_ECUADOR, 2, KEY_NATION_PERU, 1),
    (29, 5, 2000, KEY_NATION_CHILE, 3, KEY_NATION_PARAGUAY, 1),
    (29, 5, 2000, KEY_NATION_COLOMBIA, 1, KEY_NATION_ARGENTINA, 3),
    (18, 6, 2000, KEY_NATION_URUGUAY, 3, KEY_NATION_VENEZUELA, 1),
    (18, 6, 2000, KEY_NATION_PARAGUAY, 2, KEY_NATION_BRAZIL, 1),
    (19, 6, 2000, KEY_NATION_BOLIVIA, 1, KEY_NATION_CHILE, 0),
    (19, 6, 2000, KEY_NATION_ARGENTINA, 2, KEY_NATION_ECUADOR, 0),
    (19, 6, 2000, KEY_NATION_PERU, 0, KEY_NATION_COLOMBIA, 1),
    (25, 6, 2000, KEY_NATION_ECUADOR, 0, KEY_NATION_COLOMBIA, 0),
    (25, 6, 2000, KEY_NATION_VENEZUELA, 0, KEY_NATION_CHILE, 2),
    (26, 6, 2000, KEY_NATION_URUGUAY, 0, KEY_NATION_PERU, 0),
    (26, 6, 2000, KEY_NATION_BRAZIL, 3, KEY_NATION_ARGENTINA, 1),
    (27, 6, 2000, KEY_NATION_BOLIVIA, 0, KEY_NATION_PARAGUAY, 0),
    (15, 7, 2000, KEY_NATION_COLOMBIA, 1, KEY_NATION_URUGUAY, 0),
    (15, 7, 2000, KEY_NATION_CHILE, 3, KEY_NATION_BRAZIL, 0),
    (16, 7, 2000, KEY_NATION_ECUADOR, 2, KEY_NATION_BOLIVIA, 0),
    (16, 7, 2000, KEY_NATION_ARGENTINA, 1, KEY_NATION_PARAGUAY, 1),
    (16, 7, 2000, KEY_NATION_PERU, 1, KEY_NATION_VENEZUELA, 0),
    (2, 8, 2000, KEY_NATION_PARAGUAY, 3, KEY_NATION_VENEZUELA, 0),
    (2, 8, 2000, KEY_NATION_CHILE, 0, KEY_NATION_COLOMBIA, 1),
    (3, 8, 2000, KEY_NATION_URUGUAY, 4, KEY_NATION_ECUADOR, 0),
    (3, 8, 2000, KEY_NATION_PERU, 1, KEY_NATION_ARGENTINA, 2),
    (3, 8, 2000, KEY_NATION_BRAZIL, 5, KEY_NATION_BOLIVIA, 0),
    (7, 9, 2000, KEY_NATION_COLOMBIA, 0, KEY_NATION_PARAGUAY, 2),
    (8, 9, 2000, KEY_NATION_VENEZUELA, 0, KEY_NATION_BRAZIL, 6),
    (8, 9, 2000, KEY_NATION_BOLIVIA, 1, KEY_NATION_PERU, 0),
    (8, 9, 2000, KEY_NATION_ECUADOR, 1, KEY_NATION_CHILE, 0),
    (8, 9, 2000, KEY_NATION_ARGENTINA, 2, KEY_NATION_URUGUAY, 1),
    (15, 10, 2000, KEY_NATION_BRAZIL, 1, KEY_NATION_COLOMBIA, 0),
    (15, 10, 2000, KEY_NATION_BOLIVIA, 0, KEY_NATION_URUGUAY, 0),
    (15, 10, 2000, KEY_NATION_VENEZUELA, 1, KEY_NATION_ECUADOR, 2),
    (15, 10, 2000, KEY_NATION_PARAGUAY, 5, KEY_NATION_PERU, 1),
    (15, 10, 2000, KEY_NATION_CHILE, 0, KEY_NATION_ARGENTINA, 2),
    (27, 2, 2001, KEY_NATION_COLOMBIA, 2, KEY_NATION_BOLIVIA, 0),
    (27, 2, 2001, KEY_NATION_PERU, 3, KEY_NATION_CHILE, 1),
    (28, 2, 2001, KEY_NATION_ECUADOR, 1, KEY_NATION_BRAZIL, 0),
    (28, 2, 2001, KEY_NATION_URUGUAY, 0, KEY_NATION_PARAGUAY, 1),
    (28, 2, 2001, KEY_NATION_ARGENTINA, 5, KEY_NATION_VENEZUELA, 0),
    (24, 3, 2001, KEY_NATION_ECUADOR, 2, KEY_NATION_PARAGUAY, 1),
    (24, 3, 2001, KEY_NATION_VENEZUELA, 2, KEY_NATION_COLOMBIA, 2),
    (24, 3, 2001, KEY_NATION_CHILE, 0, KEY_NATION_URUGUAY, 1),
    (25, 3, 2001, KEY_NATION_BOLIVIA, 3, KEY_NATION_ARGENTINA, 3),
    (25, 3, 2001, KEY_NATION_BRAZIL, 1, KEY_NATION_PERU, 1),
    (2, 5, 2001, KEY_NATION_PARAGUAY, 1, KEY_NATION_CHILE, 0),
    (2, 5, 2001, KEY_NATION_PERU, 1, KEY_NATION_ECUADOR, 2),
    (3, 5, 2001, KEY_NATION_BOLIVIA, 5, KEY_NATION_VENEZUELA, 0),
    (3, 5, 2001, KEY_NATION_ARGENTINA, 3, KEY_NATION_COLOMBIA, 0),
    (1, 6, 2001, KEY_NATION_URUGUAY, 1, KEY_NATION_BRAZIL, 0),
    (14, 7, 2001, KEY_NATION_ECUADOR, 0, KEY_NATION_ARGENTINA, 2),
    (15, 7, 2001, KEY_NATION_VENEZUELA, 2, KEY_NATION_URUGUAY, 0),
    (15, 7, 2001, KEY_NATION_CHILE, 2, KEY_NATION_BOLIVIA, 2),
    (16, 7, 2001, KEY_NATION_BRAZIL, 2, KEY_NATION_PARAGUAY, 0),
    (17, 7, 2001, KEY_NATION_COLOMBIA, 0, KEY_NATION_PERU, 1),
    (5, 8, 2001, KEY_NATION_CHILE, 0, KEY_NATION_VENEZUELA, 2),
    (5, 8, 2001, KEY_NATION_PERU, 0, KEY_NATION_URUGUAY, 2),
    (6, 8, 2001, KEY_NATION_PARAGUAY, 5, KEY_NATION_BOLIVIA, 1),
    (6, 8, 2001, KEY_NATION_ARGENTINA, 2, KEY_NATION_BRAZIL, 1),
    (6, 8, 2001, KEY_NATION_COLOMBIA, 0, KEY_NATION_ECUADOR, 0),
    (6, 9, 2001, KEY_NATION_URUGUAY, -1, KEY_NATION_COLOMBIA, -1),
    (6, 9, 2001, KEY_NATION_BRAZIL, -1, KEY_NATION_CHILE, -1),
    (6, 9, 2001, KEY_NATION_BOLIVIA, -1, KEY_NATION_ECUADOR, -1),
    (6, 9, 2001, KEY_NATION_PARAGUAY, -1, KEY_NATION_ARGENTINA, -1),
    (6, 9, 2001, KEY_NATION_VENEZUELA, -1, KEY_NATION_PERU, -1),
    (7, 10, 2001, KEY_NATION_ARGENTINA, -1, KEY_NATION_PERU, -1),
    (7, 10, 2001, KEY_NATION_BOLIVIA, -1, KEY_NATION_BRAZIL, -1),
    (7, 10, 2001, KEY_NATION_ECUADOR, -1, KEY_NATION_URUGUAY, -1),
    (7, 10, 2001, KEY_NATION_COLOMBIA, -1, KEY_NATION_CHILE, -1),
    (7, 10, 2001, KEY_NATION_VENEZUELA, -1, KEY_NATION_PARAGUAY, -1),
    (14, 10, 2001, KEY_NATION_URUGUAY, -1, KEY_NATION_ARGENTINA, -1),
    (14, 10, 2001, KEY_NATION_BRAZIL, -1, KEY_NATION_VENEZUELA, -1),
    (14, 10, 2001, KEY_NATION_PARAGUAY, -1, KEY_NATION_COLOMBIA, -1),
    (14, 10, 2001, KEY_NATION_CHILE, -1, KEY_NATION_ECUADOR, -1),
    (14, 10, 2001, KEY_NATION_PERU, -1, KEY_NATION_BOLIVIA, -1), // r7_06_1: the last fixture
];
