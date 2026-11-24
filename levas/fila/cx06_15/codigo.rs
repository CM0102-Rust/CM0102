// ---------- cx06_15: the Euro 2000 qualifying groups and results of a 1999 game (euro_champ_qual.cpp) ----------

/// One past qualifying fixture: the stage (−1 is group 1), the date (month from 0 = January), the home nation's key
/// and goals, the away nation's key and goals.
#[derive(Clone, Copy)]
pub(crate) struct HistoricFixture {
    pub(crate) stage: i8,
    pub(crate) day: u8,
    pub(crate) month: u8,
    pub(crate) year: u16,
    pub(crate) home: usize,
    pub(crate) home_goals: u8,
    pub(crate) away: usize,
    pub(crate) away_goals: u8,
}

/// A `HistoricFixture` (the order of the original's arguments).
#[allow(clippy::too_many_arguments)]
const fn hf(stage: i8, day: u8, month: u8, year: u16, home: usize, home_goals: u8, away: usize, away_goals: u8) -> HistoricFixture {
    HistoricFixture { stage, day, month, year, home, home_goals, away, away_goals }
}

/// Builds the Euro 2000 qualifying groups and enters their 1998 and 1999 results (only in a game that starts in 1999,
/// from the constructor).
///
/// `euro_champ_qual::GROUP_FORMAT` 2 and an entrant list of 51 (`exe::crt_malloc`(0xcc); out of memory: the box at
/// line 0x7f9 and return), filled from 0. Each group of `EURO_2000_GROUPS` (the nation ids read from the keys at the
/// start; an empty slot or an absent nation, −1, is skipped) gives a team list of its national teams
/// (`national_team_of_id`; each is also added to the entrants): group 1 is the qualifiers' own table (`.club_count()`,
/// `exe::crt_malloc`; out of memory: the box at line 0x815 and return; `exe::league_init_table_row` for each row), the
/// others stages 0 to 7 (EuroChampQual::create_group_stage 0x57ed10 with the list and its size). Then the runners-up
/// stage (EuroChampQual::create_runners_up_stage 0x57f920), the predicted positions
/// (`comp_vtable::SET_PREDICTED_POSITIONS`), the entrant list freed and cleared (its count is kept), `active` 1, and every
/// result of `EURO_2000_RESULTS` in order (`euro_champ_qual_add_historic_fixture`).
/// Original: `EuroChampQual::create_groups_and_results` (0x580bd0, euro_champ_qual.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn euro_champ_qual_create_groups_and_results(this: usize) {
    let l = League(this);
    let ids: Vec<[i32; 6]> = EURO_2000_GROUPS.iter().map(|g| g.map(|key| key.map_or(-1, |k| read_i32(k)))).collect();
    write_u8(this + euro_champ_qual::GROUP_FORMAT, 2);
    write_u32(this + 0xf2, 0x33); // the entrant count
    let entrants = exe::crt_malloc(0xcc);
    write_usize(this + 0xee, entrants);
    if entrants == 0 {
        return show_error_at(0x9c4674, 0x7f9); // euro_champ_qual.cpp
    }
    write_u32(this + 0xf2, 0);
    for (g, group) in ids.iter().enumerate() {
        let mut teams = [0usize; 6];
        let mut n = 0i8;
        for &id in group {
            if id == -1 {
                continue;
            }
            teams[n as usize] = national_team_of_id(id as usize);
            n += 1;
            let count = read_u32(this + 0xf2);
            write_usize(read_usize(this + 0xee) + 4 * count as usize, national_team_of_id(id as usize));
            write_u32(this + 0xf2, count + 1);
        }
        if g == 0 {
            l.set_club_count(n as i16);
            let table = exe::crt_malloc((l.club_count() as i32 as u32).wrapping_mul(league_table_row::SIZE as u32));
            l.set_table(table);
            if table == 0 {
                return show_error_at(0x9c4674, 0x815);
            }
            let mut row = 0i8;
            while (row as i16) < l.club_count() {
                exe::league_init_table_row(this, row as u8 as u32, teams[row as usize], 0, 0);
                row += 1;
            }
        } else {
            let create_group_stage: unsafe extern "thiscall" fn(usize, u32, usize, u32) -> u32 = core::mem::transmute(0x57ed10usize); // EuroChampQual::create_group_stage
            create_group_stage(this, g as u32 - 1, teams.as_ptr() as usize, n as i16 as u16 as u32);
        }
    }
    covered!("cx06_15 euro 2000 groups");
    let create_runners_up_stage: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x57f920usize); // EuroChampQual::create_runners_up_stage
    create_runners_up_stage(this);
    let predicted: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::SET_PREDICTED_POSITIONS));
    predicted(this);
    exe::crt_free(read_usize(this + 0xee));
    write_usize(this + 0xee, 0);
    l.set_active(1);
    for f in EURO_2000_RESULTS.iter() {
        euro_champ_qual_add_historic_fixture(this, f.stage as u8 as u32, f.day as u32, f.month as u32, f.year as u32, read_u32(f.home), f.home_goals as u32,
            read_u32(f.away), f.away_goals as u32);
    }
}

/// Adds one past qualifying fixture of the 1998 edition and, when it was played before the game start, its result.
///
/// The stage must exist: with no stage vector, a stage after `.current_stage()` or a missing stage, the box at line 0x936;
/// a year other than 1998 or 1999 or a stage outside −1..7, the box at line 0x93d; nothing else then. The fixture
/// record: the competition and its id, the season `year()`, round name 0, the round prefix (the stage id of the stage,
/// `league::STAGE_ID`), the category, the draw and tie rules, the stage index, the squad rules (the named substitutes and
/// the substitutions of the qualifiers, `league::SUBSTITUTES_NAMED` and `SUBSTITUTIONS_ALLOWED`), the goals, the
/// half-time, extra-time, penalty and aggregate bytes 0xff, the match statistics slot −1, the result marks 0, the stage
/// byte 0xff, the side marks 1, the date (Date::Date `exe::date_date_549f70` with the day, the month, the year and −1)
/// also as the next round's date, round number 0, the time slice 1 on Fridays and Saturdays (`exe::date_get_weekday` 5
/// or 6) and 2 otherwise, the two national teams (`national_team_of_id`) and their ids; the venue
/// (Stadium::set_fixture_venue `exe::stadium_set_fixture_venue` on `G_STADIUM` with 0, the fixture, −1). A fixture dated
/// after the game start (`G_GAME_START_DATE`) is unplayed (both goals 0xff). The fixture goes to the fixture manager
/// (`exe::fixture_manager_add_fixture` with 1); a played one is added as a result (`comp_vtable::ADD_RESULT` with 0) and
/// its group sorted (`exe::league_sort_table`); either way the runners-up table is rebuilt
/// (EuroChampQual::update_runners_up_table 0x57fa10).
///
/// Defect of the original: the referee word (`fixture::REFEREE`), the neutral-venue and flag bytes
/// (`fixture::NEUTRAL_VENUE`, `fixture::FLAGS`, unless the venue routine writes them) and the top 4 bits of the squad
/// rules keep what the stack held; the copy's record starts at zero.
/// Original: `EuroChampQual::add_historic_fixture` (0x582ef0, euro_champ_qual.cpp, thiscall, ret 0x20).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn euro_champ_qual_add_historic_fixture(this: usize, stage: u32, day: u32, month: u32, year: u32, home: u32, home_goals: u32, away: u32,
    away_goals: u32) {
    let l = League(this);
    let stage = stage as u8 as i8;
    let stage_of = |s: i8| read_usize(l.stages() + 4 * s as isize as usize);
    if l.stages() == 0 || stage as i32 > l.current_stage() as i32 || (stage != -1 && stage_of(stage) == 0) {
        return show_error_at(0x9c4674, 0x936); // euro_champ_qual.cpp
    }
    if !(0x7ce..=0x7cf).contains(&(year as u16)) || !(-1..=7).contains(&stage) {
        return show_error_at(0x9c4674, 0x93d);
    }
    let mut record = [0u8; fixture::SIZE];
    let f = record.as_mut_ptr() as usize;
    let squad = (l.substitutes_named() & 0xf) as u16 | (((l.substitutions_allowed() & 0xf) as u16) << 4);
    write_u16(f + fixture::SQUAD_RULES, squad);
    write_usize(f + fixture::COMPETITION, l.competition().addr());
    write_u32(f + fixture::COMP_ID, read_u32(l.competition().addr()));
    write_i16(f + fixture::SEASON_YEAR, l.year());
    write_u16(f + fixture::ROUND_NAME, 0);
    let prefix = if stage == -1 { l.stage_id() } else { League(stage_of(stage)).stage_id() };
    write_u16(f + fixture::ROUND_PREFIX, prefix as u16);
    write_u8(f + fixture::CATEGORY, l.match_category());
    write_u16(f + fixture::ROUND_DRAW_RULES, l.round_draw_rules() as u16);
    write_u16(f + fixture::TIE_RULES, l.match_rules() as u16);
    write_u8(f + fixture::HOME_GOALS, home_goals as u8);
    write_u8(f + fixture::STAGE_INDEX, stage as u8);
    write_u8(f + fixture::AWAY_GOALS, away_goals as u8);
    for b in fixture::HOME_GOALS_HALF_TIME..=fixture::AWAY_AGGREGATE {
        write_u8(f + b, 0xff);
    }
    write_i32(f + fixture::MATCH_STATS_SLOT, -1);
    write_u8(f + fixture::HOME_RESULT_MARK, 0);
    write_u8(f + fixture::AWAY_RESULT_MARK, 0);
    write_u8(f + fixture::STAGE, 0xff);
    write_u8(f + fixture::HOME_SIDE_MARK, 1);
    write_u8(f + fixture::AWAY_SIDE_MARK, 1);
    let mut date = [0u16; 4];
    exe::date_date_549f70(&mut date, day, month, year, 0xffff_ffff);
    write_u16(f + fixture::DAY, date[0]);
    write_u16(f + fixture::YEAR, date[1]);
    let weekday = exe::date_get_weekday(date.as_ptr() as usize) as u16;
    let weekend = weekday == 5 || exe::date_get_weekday(date.as_ptr() as usize) as u16 == 6;
    write_u8(f + fixture::TIME_SLICE, if weekend { 1 } else { 2 });
    write_u16(f + fixture::NEXT_ROUND_DAY, date[0]);
    write_u16(f + fixture::NEXT_ROUND_YEAR, date[1]);
    write_u16(f + fixture::ROUND_NUMBER, 0);
    let (home_team, away_team) = (national_team_of_id(home as usize), national_team_of_id(away as usize));
    write_usize(f + fixture::HOME_CLUB, home_team);
    write_usize(f + fixture::AWAY_CLUB, away_team);
    write_u32(f + fixture::HOME_CLUB_ID, read_u32(home_team));
    write_u32(f + fixture::AWAY_CLUB_ID, read_u32(away_team));
    exe::stadium_set_fixture_venue(read_usize(G_STADIUM), 0, f, 0xffff_ffffusize);
    let (start_day, start_year) = (read_i16(G_GAME_START_DATE), read_u16(G_GAME_START_DATE + 2));
    if start_year < date[1] || (start_year == date[1] && start_day < date[0] as i16) {
        covered!("cx06_15 fixture after the start");
        write_u8(f + fixture::HOME_GOALS, 0xff);
        write_u8(f + fixture::AWAY_GOALS, 0xff);
    }
    exe::fixture_manager_add_fixture(read_usize(G_FIXTURE_MANAGER), f, 1);
    if read_u8(f + fixture::HOME_GOALS) != 0xff {
        covered!("cx06_15 past result");
        let add_result: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::ADD_RESULT));
        add_result(this, f, 0);
        exe::league_sort_table(if read_u8(f + fixture::STAGE_INDEX) as i8 == -1 { this } else { stage_of(read_u8(f + fixture::STAGE_INDEX) as i8) }, 0);
    }
    let update_runners_up_table: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x57fa10usize); // EuroChampQual::update_runners_up_table
    update_runners_up_table(this);
}

/// Comparer for `euro_champ_qual_create_groups_and_results`: it allocates the groups and enters fixtures (it cannot run
/// twice), so only the original runs; the plan: 9 groups (group 1 with its rows, stages 0 to 7) and `active` 1.
pub(crate) unsafe extern "thiscall" fn compare_euro_champ_qual_create_groups_and_results(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this);
    let l = League(this);
    let ok = l.table() != 0 && (0..8).all(|g| read_usize(l.stages() + 4 * g) != 0) && l.active() == 1;
    SCOREBOARD.record(ok, || format!("euro_champ_qual_create_groups_and_results({:#x}): table {:#x}, active {}", this, l.table(), l.active()));
}

/// Comparer for `euro_champ_qual_add_historic_fixture`: it adds a fixture and a result (it cannot run twice), so only the
/// original runs.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_euro_champ_qual_add_historic_fixture(this: usize, stage: u32, day: u32, month: u32, year: u32, home: u32, home_goals: u32,
    away: u32, away_goals: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32)>(%%I1%%)(this, stage, day, month, year, home, home_goals, away, away_goals)
}

/// The Euro 2000 qualifying groups (the 1998 edition), group 1 first, by nation key (`None`: an empty slot).
pub(crate) const EURO_2000_GROUPS: [[Option<usize>; 6]; 9] = [
    [Some(KEY_NATION_ITALY), Some(KEY_NATION_DENMARK), Some(KEY_NATION_SWITZERLAND), Some(KEY_NATION_WALES), Some(KEY_NATION_BELARUS), None],
    [Some(KEY_NATION_NORWAY), Some(KEY_NATION_SLOVENIA), Some(KEY_NATION_LATVIA), Some(KEY_NATION_GREECE), Some(KEY_NATION_GEORGIA), Some(KEY_NATION_ALBANIA)],
    [Some(KEY_NATION_GERMANY), Some(KEY_NATION_TURKEY), Some(KEY_NATION_FINLAND), Some(KEY_NATION_NORTHERN_IRELAND), Some(KEY_NATION_MOLDOVA), None],
    [Some(KEY_NATION_UKRAINE), Some(KEY_NATION_RUSSIA), Some(KEY_NATION_FRANCE), Some(KEY_NATION_ICELAND), Some(KEY_NATION_ARMENIA), Some(KEY_NATION_ANDORRA)],
    [Some(KEY_NATION_SWEDEN), Some(KEY_NATION_POLAND), Some(KEY_NATION_ENGLAND), Some(KEY_NATION_BULGARIA), Some(KEY_NATION_LUXEMBOURG), None],
    [Some(KEY_NATION_SPAIN), Some(KEY_NATION_ISRAEL), Some(KEY_NATION_AUSTRIA), Some(KEY_NATION_CYPRUS), Some(KEY_NATION_SAN_MARINO), None],
    [Some(KEY_NATION_PORTUGAL), Some(KEY_NATION_ROMANIA), Some(KEY_NATION_SLOVAKIA), Some(KEY_NATION_HUNGARY), Some(KEY_NATION_AZERBAIJAN), Some(KEY_NATION_LIECHTENSTEIN)],
    [Some(KEY_NATION_YUGOSLAVIA), Some(KEY_NATION_REPUBLIC_OF_IRELAND), Some(KEY_NATION_FYR_OF_MACEDONIA), Some(KEY_NATION_CROATIA), Some(KEY_NATION_MALTA), None],
    [Some(KEY_NATION_CZECH_REPUBLIC), Some(KEY_NATION_SCOTLAND), Some(KEY_NATION_BOSNIA_HERZEGOVINA), Some(KEY_NATION_LITHUANIA), Some(KEY_NATION_ESTONIA), Some(KEY_NATION_FAROE_ISLANDS)],
];

/// The Euro 2000 qualifying results entered at the start of a 1999 game, in the original's order (`hf`: the stage, -1
/// for group 1; the day; the month from 0 = January; the year; the home nation and goals; the away nation and goals).
pub(crate) const EURO_2000_RESULTS: [HistoricFixture; 220] = [
    hf(-1, 5, 8, 1998, KEY_NATION_WALES, 0, KEY_NATION_ITALY, 2), hf(-1, 5, 8, 1998, KEY_NATION_BELARUS, 0, KEY_NATION_DENMARK, 0),
    hf(-1, 10, 9, 1998, KEY_NATION_DENMARK, 1, KEY_NATION_WALES, 2), hf(-1, 10, 9, 1998, KEY_NATION_ITALY, 2, KEY_NATION_SWITZERLAND, 0),
    hf(-1, 14, 9, 1998, KEY_NATION_WALES, 3, KEY_NATION_BELARUS, 2), hf(-1, 14, 9, 1998, KEY_NATION_SWITZERLAND, 1, KEY_NATION_DENMARK, 1),
    hf(-1, 27, 2, 1999, KEY_NATION_BELARUS, 0, KEY_NATION_SWITZERLAND, 1), hf(-1, 27, 2, 1999, KEY_NATION_DENMARK, 1, KEY_NATION_ITALY, 2),
    hf(-1, 31, 2, 1999, KEY_NATION_SWITZERLAND, 2, KEY_NATION_WALES, 0), hf(-1, 31, 2, 1999, KEY_NATION_ITALY, 1, KEY_NATION_BELARUS, 1),
    hf(-1, 5, 5, 1999, KEY_NATION_DENMARK, 1, KEY_NATION_BELARUS, 0), hf(-1, 5, 5, 1999, KEY_NATION_ITALY, 4, KEY_NATION_WALES, 0),
    hf(-1, 9, 5, 1999, KEY_NATION_WALES, 0, KEY_NATION_DENMARK, 2), hf(-1, 9, 5, 1999, KEY_NATION_SWITZERLAND, 0, KEY_NATION_ITALY, 0),
    hf(-1, 4, 8, 1999, KEY_NATION_BELARUS, 1, KEY_NATION_WALES, 2), hf(-1, 4, 8, 1999, KEY_NATION_DENMARK, 2, KEY_NATION_SWITZERLAND, 1),
    hf(-1, 8, 8, 1999, KEY_NATION_ITALY, 2, KEY_NATION_DENMARK, 3), hf(-1, 8, 8, 1999, KEY_NATION_SWITZERLAND, 2, KEY_NATION_BELARUS, 0),
    hf(-1, 9, 9, 1999, KEY_NATION_BELARUS, 0, KEY_NATION_ITALY, 0), hf(-1, 9, 9, 1999, KEY_NATION_WALES, 0, KEY_NATION_SWITZERLAND, 2),
    hf(0, 5, 8, 1998, KEY_NATION_GEORGIA, 1, KEY_NATION_ALBANIA, 0), hf(0, 6, 8, 1998, KEY_NATION_GREECE, 2, KEY_NATION_SLOVENIA, 2),
    hf(0, 6, 8, 1998, KEY_NATION_NORWAY, 1, KEY_NATION_LATVIA, 3), hf(0, 10, 9, 1998, KEY_NATION_LATVIA, 1, KEY_NATION_GEORGIA, 0),
    hf(0, 10, 9, 1998, KEY_NATION_SLOVENIA, 1, KEY_NATION_NORWAY, 2), hf(0, 14, 9, 1998, KEY_NATION_GREECE, 3, KEY_NATION_GEORGIA, 0),
    hf(0, 14, 9, 1998, KEY_NATION_NORWAY, 2, KEY_NATION_ALBANIA, 2), hf(0, 14, 9, 1998, KEY_NATION_SLOVENIA, 1, KEY_NATION_LATVIA, 0),
    hf(0, 18, 10, 1998, KEY_NATION_ALBANIA, 0, KEY_NATION_GREECE, 0), hf(0, 27, 2, 1999, KEY_NATION_GREECE, 0, KEY_NATION_NORWAY, 2),
    hf(0, 27, 2, 1999, KEY_NATION_GEORGIA, 1, KEY_NATION_SLOVENIA, 1), hf(0, 31, 2, 1999, KEY_NATION_LATVIA, 0, KEY_NATION_GREECE, 0),
    hf(0, 28, 3, 1999, KEY_NATION_LATVIA, 0, KEY_NATION_ALBANIA, 0), hf(0, 28, 3, 1999, KEY_NATION_GEORGIA, 1, KEY_NATION_NORWAY, 4),
    hf(0, 30, 4, 1999, KEY_NATION_NORWAY, 1, KEY_NATION_GEORGIA, 0), hf(0, 5, 5, 1999, KEY_NATION_ALBANIA, 1, KEY_NATION_NORWAY, 2),
    hf(0, 5, 5, 1999, KEY_NATION_LATVIA, 1, KEY_NATION_SLOVENIA, 2), hf(0, 5, 5, 1999, KEY_NATION_GEORGIA, 1, KEY_NATION_GREECE, 2),
    hf(0, 9, 5, 1999, KEY_NATION_ALBANIA, 0, KEY_NATION_SLOVENIA, 1), hf(0, 9, 5, 1999, KEY_NATION_GREECE, 1, KEY_NATION_LATVIA, 2),
    hf(0, 18, 7, 1999, KEY_NATION_SLOVENIA, 2, KEY_NATION_ALBANIA, 0), hf(0, 4, 8, 1999, KEY_NATION_ALBANIA, 3, KEY_NATION_LATVIA, 3),
    hf(0, 4, 8, 1999, KEY_NATION_SLOVENIA, 2, KEY_NATION_GEORGIA, 1), hf(0, 4, 8, 1999, KEY_NATION_NORWAY, 1, KEY_NATION_GREECE, 0),
    hf(0, 8, 8, 1999, KEY_NATION_GEORGIA, 2, KEY_NATION_LATVIA, 2), hf(0, 8, 8, 1999, KEY_NATION_NORWAY, 4, KEY_NATION_SLOVENIA, 0),
    hf(0, 8, 8, 1999, KEY_NATION_GREECE, 2, KEY_NATION_ALBANIA, 0), hf(0, 9, 9, 1999, KEY_NATION_SLOVENIA, 0, KEY_NATION_GREECE, 3),
    hf(0, 9, 9, 1999, KEY_NATION_LATVIA, 1, KEY_NATION_NORWAY, 2), hf(0, 9, 9, 1999, KEY_NATION_ALBANIA, 2, KEY_NATION_GEORGIA, 1),
    hf(1, 5, 8, 1998, KEY_NATION_FINLAND, 3, KEY_NATION_MOLDOVA, 2), hf(1, 5, 8, 1998, KEY_NATION_TURKEY, 3, KEY_NATION_NORTHERN_IRELAND, 0),
    hf(1, 10, 9, 1998, KEY_NATION_NORTHERN_IRELAND, 1, KEY_NATION_FINLAND, 0), hf(1, 10, 9, 1998, KEY_NATION_TURKEY, 1, KEY_NATION_GERMANY, 0),
    hf(1, 14, 9, 1998, KEY_NATION_TURKEY, 1, KEY_NATION_FINLAND, 3), hf(1, 14, 9, 1998, KEY_NATION_MOLDOVA, 1, KEY_NATION_GERMANY, 3),
    hf(1, 18, 10, 1998, KEY_NATION_NORTHERN_IRELAND, 2, KEY_NATION_MOLDOVA, 2), hf(1, 27, 2, 1999, KEY_NATION_NORTHERN_IRELAND, 0, KEY_NATION_GERMANY, 3),
    hf(1, 27, 2, 1999, KEY_NATION_TURKEY, 2, KEY_NATION_MOLDOVA, 0), hf(1, 31, 2, 1999, KEY_NATION_MOLDOVA, 0, KEY_NATION_NORTHERN_IRELAND, 0),
    hf(1, 31, 2, 1999, KEY_NATION_GERMANY, 2, KEY_NATION_FINLAND, 0), hf(1, 4, 5, 1999, KEY_NATION_GERMANY, 6, KEY_NATION_MOLDOVA, 1),
    hf(1, 5, 5, 1999, KEY_NATION_FINLAND, 2, KEY_NATION_TURKEY, 4), hf(1, 9, 5, 1999, KEY_NATION_MOLDOVA, 0, KEY_NATION_FINLAND, 0),
    hf(1, 4, 8, 1999, KEY_NATION_NORTHERN_IRELAND, 0, KEY_NATION_TURKEY, 3), hf(1, 4, 8, 1999, KEY_NATION_FINLAND, 1, KEY_NATION_GERMANY, 2),
    hf(1, 8, 8, 1999, KEY_NATION_GERMANY, 4, KEY_NATION_NORTHERN_IRELAND, 0), hf(1, 8, 8, 1999, KEY_NATION_MOLDOVA, 1, KEY_NATION_TURKEY, 1),
    hf(1, 9, 9, 1999, KEY_NATION_FINLAND, 4, KEY_NATION_NORTHERN_IRELAND, 1), hf(1, 9, 9, 1999, KEY_NATION_GERMANY, 0, KEY_NATION_TURKEY, 0),
    hf(2, 5, 8, 1998, KEY_NATION_ARMENIA, 3, KEY_NATION_ANDORRA, 1), hf(2, 5, 8, 1998, KEY_NATION_ICELAND, 1, KEY_NATION_FRANCE, 1),
    hf(2, 5, 8, 1998, KEY_NATION_UKRAINE, 3, KEY_NATION_RUSSIA, 2), hf(2, 10, 9, 1998, KEY_NATION_ANDORRA, 0, KEY_NATION_UKRAINE, 2),
    hf(2, 10, 9, 1998, KEY_NATION_ARMENIA, 0, KEY_NATION_ICELAND, 0), hf(2, 10, 9, 1998, KEY_NATION_RUSSIA, 2, KEY_NATION_FRANCE, 3),
    hf(2, 14, 9, 1998, KEY_NATION_ICELAND, 1, KEY_NATION_RUSSIA, 0), hf(2, 14, 9, 1998, KEY_NATION_UKRAINE, 2, KEY_NATION_ARMENIA, 0),
    hf(2, 14, 9, 1998, KEY_NATION_FRANCE, 2, KEY_NATION_ANDORRA, 0), hf(2, 27, 2, 1999, KEY_NATION_ANDORRA, 0, KEY_NATION_ICELAND, 2),
    hf(2, 27, 2, 1999, KEY_NATION_ARMENIA, 0, KEY_NATION_RUSSIA, 3), hf(2, 27, 2, 1999, KEY_NATION_FRANCE, 0, KEY_NATION_UKRAINE, 0),
    hf(2, 31, 2, 1999, KEY_NATION_RUSSIA, 6, KEY_NATION_ANDORRA, 1), hf(2, 31, 2, 1999, KEY_NATION_UKRAINE, 1, KEY_NATION_ICELAND, 1),
    hf(2, 31, 2, 1999, KEY_NATION_FRANCE, 2, KEY_NATION_ARMENIA, 0), hf(2, 5, 5, 1999, KEY_NATION_ICELAND, 2, KEY_NATION_ARMENIA, 0),
    hf(2, 5, 5, 1999, KEY_NATION_UKRAINE, 4, KEY_NATION_ANDORRA, 0), hf(2, 5, 5, 1999, KEY_NATION_FRANCE, 2, KEY_NATION_RUSSIA, 3),
    hf(2, 9, 5, 1999, KEY_NATION_RUSSIA, 1, KEY_NATION_ICELAND, 0), hf(2, 9, 5, 1999, KEY_NATION_ARMENIA, 0, KEY_NATION_UKRAINE, 0),
    hf(2, 9, 5, 1999, KEY_NATION_ANDORRA, 0, KEY_NATION_FRANCE, 1), hf(2, 4, 8, 1999, KEY_NATION_UKRAINE, 0, KEY_NATION_FRANCE, 0),
    hf(2, 4, 8, 1999, KEY_NATION_RUSSIA, 2, KEY_NATION_ARMENIA, 0), hf(2, 4, 8, 1999, KEY_NATION_ICELAND, 3, KEY_NATION_ANDORRA, 0),
    hf(2, 8, 8, 1999, KEY_NATION_ARMENIA, 2, KEY_NATION_FRANCE, 3), hf(2, 8, 8, 1999, KEY_NATION_ICELAND, 0, KEY_NATION_UKRAINE, 1),
    hf(2, 8, 8, 1999, KEY_NATION_ANDORRA, 1, KEY_NATION_RUSSIA, 2), hf(2, 9, 9, 1999, KEY_NATION_FRANCE, 3, KEY_NATION_ICELAND, 2),
    hf(2, 9, 9, 1999, KEY_NATION_RUSSIA, 1, KEY_NATION_UKRAINE, 1), hf(2, 9, 9, 1999, KEY_NATION_ANDORRA, 0, KEY_NATION_ARMENIA, 3),
    hf(3, 5, 8, 1998, KEY_NATION_SWEDEN, 2, KEY_NATION_ENGLAND, 1), hf(3, 6, 8, 1998, KEY_NATION_BULGARIA, 0, KEY_NATION_POLAND, 3),
    hf(3, 10, 9, 1998, KEY_NATION_ENGLAND, 0, KEY_NATION_BULGARIA, 0), hf(3, 10, 9, 1998, KEY_NATION_POLAND, 3, KEY_NATION_LUXEMBOURG, 0),
    hf(3, 14, 9, 1998, KEY_NATION_BULGARIA, 0, KEY_NATION_SWEDEN, 1), hf(3, 14, 9, 1998, KEY_NATION_LUXEMBOURG, 0, KEY_NATION_ENGLAND, 3),
    hf(3, 27, 2, 1999, KEY_NATION_ENGLAND, 3, KEY_NATION_POLAND, 1), hf(3, 27, 2, 1999, KEY_NATION_SWEDEN, 2, KEY_NATION_LUXEMBOURG, 0),
    hf(3, 31, 2, 1999, KEY_NATION_LUXEMBOURG, 0, KEY_NATION_BULGARIA, 2), hf(3, 31, 2, 1999, KEY_NATION_POLAND, 0, KEY_NATION_SWEDEN, 1),
    hf(3, 4, 5, 1999, KEY_NATION_POLAND, 2, KEY_NATION_BULGARIA, 0), hf(3, 5, 5, 1999, KEY_NATION_ENGLAND, 0, KEY_NATION_SWEDEN, 0),
    hf(3, 9, 5, 1999, KEY_NATION_BULGARIA, 1, KEY_NATION_ENGLAND, 1), hf(3, 9, 5, 1999, KEY_NATION_LUXEMBOURG, 2, KEY_NATION_POLAND, 3),
    hf(3, 4, 8, 1999, KEY_NATION_SWEDEN, 1, KEY_NATION_BULGARIA, 0), hf(3, 4, 8, 1999, KEY_NATION_ENGLAND, 6, KEY_NATION_LUXEMBOURG, 0),
    hf(3, 8, 8, 1999, KEY_NATION_POLAND, 0, KEY_NATION_ENGLAND, 0), hf(3, 8, 8, 1999, KEY_NATION_LUXEMBOURG, 0, KEY_NATION_SWEDEN, 1),
    hf(3, 9, 9, 1999, KEY_NATION_SWEDEN, 2, KEY_NATION_POLAND, 0), hf(3, 10, 9, 1999, KEY_NATION_BULGARIA, 3, KEY_NATION_LUXEMBOURG, 0),
    hf(4, 5, 8, 1998, KEY_NATION_AUSTRIA, 1, KEY_NATION_ISRAEL, 1), hf(4, 5, 8, 1998, KEY_NATION_CYPRUS, 3, KEY_NATION_SPAIN, 2),
    hf(4, 10, 9, 1998, KEY_NATION_CYPRUS, 0, KEY_NATION_AUSTRIA, 3), hf(4, 10, 9, 1998, KEY_NATION_SAN_MARINO, 0, KEY_NATION_ISRAEL, 5),
    hf(4, 14, 9, 1998, KEY_NATION_SAN_MARINO, 1, KEY_NATION_AUSTRIA, 4), hf(4, 14, 9, 1998, KEY_NATION_ISRAEL, 1, KEY_NATION_SPAIN, 2),
    hf(4, 18, 10, 1998, KEY_NATION_SAN_MARINO, 0, KEY_NATION_CYPRUS, 1), hf(4, 10, 1, 1999, KEY_NATION_CYPRUS, 4, KEY_NATION_SAN_MARINO, 0),
    hf(4, 27, 2, 1999, KEY_NATION_SPAIN, 9, KEY_NATION_AUSTRIA, 0), hf(4, 28, 2, 1999, KEY_NATION_ISRAEL, 3, KEY_NATION_CYPRUS, 0),
    hf(4, 31, 2, 1999, KEY_NATION_SAN_MARINO, 0, KEY_NATION_SPAIN, 6), hf(4, 28, 3, 1999, KEY_NATION_AUSTRIA, 7, KEY_NATION_SAN_MARINO, 0),
    hf(4, 5, 5, 1999, KEY_NATION_SPAIN, 9, KEY_NATION_SAN_MARINO, 0), hf(4, 6, 5, 1999, KEY_NATION_ISRAEL, 5, KEY_NATION_AUSTRIA, 0),
    hf(4, 4, 8, 1999, KEY_NATION_AUSTRIA, 1, KEY_NATION_SPAIN, 3), hf(4, 5, 8, 1999, KEY_NATION_CYPRUS, 3, KEY_NATION_ISRAEL, 2),
    hf(4, 8, 8, 1999, KEY_NATION_SPAIN, 8, KEY_NATION_CYPRUS, 0), hf(4, 8, 8, 1999, KEY_NATION_ISRAEL, 8, KEY_NATION_SAN_MARINO, 0),
    hf(4, 9, 9, 1999, KEY_NATION_SPAIN, 3, KEY_NATION_ISRAEL, 0), hf(4, 10, 9, 1999, KEY_NATION_AUSTRIA, 3, KEY_NATION_CYPRUS, 1),
    hf(5, 2, 8, 1998, KEY_NATION_ROMANIA, 7, KEY_NATION_LIECHTENSTEIN, 0), hf(5, 5, 8, 1998, KEY_NATION_SLOVAKIA, 3, KEY_NATION_AZERBAIJAN, 0),
    hf(5, 6, 8, 1998, KEY_NATION_HUNGARY, 1, KEY_NATION_PORTUGAL, 3), hf(5, 10, 9, 1998, KEY_NATION_LIECHTENSTEIN, 0, KEY_NATION_SLOVAKIA, 4),
    hf(5, 10, 9, 1998, KEY_NATION_AZERBAIJAN, 0, KEY_NATION_HUNGARY, 4), hf(5, 10, 9, 1998, KEY_NATION_PORTUGAL, 0, KEY_NATION_ROMANIA, 1),
    hf(5, 14, 9, 1998, KEY_NATION_LIECHTENSTEIN, 2, KEY_NATION_AZERBAIJAN, 1), hf(5, 14, 9, 1998, KEY_NATION_HUNGARY, 1, KEY_NATION_ROMANIA, 1),
    hf(5, 14, 9, 1998, KEY_NATION_SLOVAKIA, 0, KEY_NATION_PORTUGAL, 3), hf(5, 26, 2, 1999, KEY_NATION_PORTUGAL, 7, KEY_NATION_AZERBAIJAN, 0),
    hf(5, 27, 2, 1999, KEY_NATION_HUNGARY, 5, KEY_NATION_LIECHTENSTEIN, 0), hf(5, 27, 2, 1999, KEY_NATION_ROMANIA, 0, KEY_NATION_SLOVAKIA, 0),
    hf(5, 31, 2, 1999, KEY_NATION_SLOVAKIA, 0, KEY_NATION_HUNGARY, 0), hf(5, 31, 2, 1999, KEY_NATION_AZERBAIJAN, 0, KEY_NATION_ROMANIA, 1),
    hf(5, 31, 2, 1999, KEY_NATION_LIECHTENSTEIN, 0, KEY_NATION_PORTUGAL, 5), hf(5, 5, 5, 1999, KEY_NATION_AZERBAIJAN, 4, KEY_NATION_LIECHTENSTEIN, 0),
    hf(5, 5, 5, 1999, KEY_NATION_PORTUGAL, 1, KEY_NATION_SLOVAKIA, 0), hf(5, 5, 5, 1999, KEY_NATION_ROMANIA, 2, KEY_NATION_HUNGARY, 0),
    hf(5, 9, 5, 1999, KEY_NATION_HUNGARY, 0, KEY_NATION_SLOVAKIA, 1), hf(5, 9, 5, 1999, KEY_NATION_PORTUGAL, 8, KEY_NATION_LIECHTENSTEIN, 0),
    hf(5, 9, 5, 1999, KEY_NATION_ROMANIA, 4, KEY_NATION_AZERBAIJAN, 0), hf(5, 3, 8, 1999, KEY_NATION_AZERBAIJAN, 1, KEY_NATION_PORTUGAL, 1),
    hf(5, 4, 8, 1999, KEY_NATION_LIECHTENSTEIN, 0, KEY_NATION_HUNGARY, 0), hf(5, 4, 8, 1999, KEY_NATION_SLOVAKIA, 1, KEY_NATION_ROMANIA, 5),
    hf(5, 8, 8, 1999, KEY_NATION_SLOVAKIA, 2, KEY_NATION_LIECHTENSTEIN, 0), hf(5, 8, 8, 1999, KEY_NATION_ROMANIA, 1, KEY_NATION_PORTUGAL, 1),
    hf(5, 8, 8, 1999, KEY_NATION_HUNGARY, 3, KEY_NATION_AZERBAIJAN, 0), hf(5, 9, 9, 1999, KEY_NATION_LIECHTENSTEIN, 0, KEY_NATION_ROMANIA, 3),
    hf(5, 9, 9, 1999, KEY_NATION_AZERBAIJAN, 0, KEY_NATION_SLOVAKIA, 1), hf(5, 10, 9, 1999, KEY_NATION_PORTUGAL, 3, KEY_NATION_HUNGARY, 0),
    hf(6, 5, 8, 1998, KEY_NATION_REPUBLIC_OF_IRELAND, 2, KEY_NATION_CROATIA, 0), hf(6, 6, 8, 1998, KEY_NATION_FYR_OF_MACEDONIA, 4, KEY_NATION_MALTA, 0),
    hf(6, 10, 9, 1998, KEY_NATION_MALTA, 1, KEY_NATION_CROATIA, 4), hf(6, 14, 9, 1998, KEY_NATION_CROATIA, 3, KEY_NATION_FYR_OF_MACEDONIA, 2),
    hf(6, 14, 9, 1998, KEY_NATION_REPUBLIC_OF_IRELAND, 5, KEY_NATION_MALTA, 0), hf(6, 18, 10, 1998, KEY_NATION_YUGOSLAVIA, 1, KEY_NATION_REPUBLIC_OF_IRELAND, 0),
    hf(6, 18, 10, 1998, KEY_NATION_MALTA, 1, KEY_NATION_FYR_OF_MACEDONIA, 2), hf(6, 10, 1, 1999, KEY_NATION_MALTA, 0, KEY_NATION_YUGOSLAVIA, 3),
    hf(6, 5, 5, 1999, KEY_NATION_FYR_OF_MACEDONIA, 1, KEY_NATION_CROATIA, 1), hf(6, 8, 5, 1999, KEY_NATION_YUGOSLAVIA, 4, KEY_NATION_MALTA, 1),
    hf(6, 9, 5, 1999, KEY_NATION_REPUBLIC_OF_IRELAND, 1, KEY_NATION_FYR_OF_MACEDONIA, 0), hf(6, 18, 7, 1999, KEY_NATION_YUGOSLAVIA, 0, KEY_NATION_CROATIA, 0),
    hf(6, 21, 7, 1999, KEY_NATION_CROATIA, 2, KEY_NATION_MALTA, 1), hf(6, 1, 8, 1999, KEY_NATION_REPUBLIC_OF_IRELAND, 2, KEY_NATION_YUGOSLAVIA, 1),
    hf(6, 4, 8, 1999, KEY_NATION_CROATIA, 1, KEY_NATION_REPUBLIC_OF_IRELAND, 0), hf(6, 4, 8, 1999, KEY_NATION_YUGOSLAVIA, 3, KEY_NATION_FYR_OF_MACEDONIA, 1),
    hf(6, 8, 8, 1999, KEY_NATION_MALTA, 2, KEY_NATION_REPUBLIC_OF_IRELAND, 3), hf(6, 8, 8, 1999, KEY_NATION_FYR_OF_MACEDONIA, 2, KEY_NATION_YUGOSLAVIA, 4),
    hf(6, 10, 9, 1999, KEY_NATION_CROATIA, 2, KEY_NATION_YUGOSLAVIA, 2), hf(6, 10, 9, 1999, KEY_NATION_FYR_OF_MACEDONIA, 1, KEY_NATION_REPUBLIC_OF_IRELAND, 1),
    hf(7, 4, 5, 1998, KEY_NATION_ESTONIA, 5, KEY_NATION_FAROE_ISLANDS, 0), hf(7, 19, 7, 1998, KEY_NATION_BOSNIA_HERZEGOVINA, 1, KEY_NATION_FAROE_ISLANDS, 0),
    hf(7, 5, 8, 1998, KEY_NATION_LITHUANIA, 0, KEY_NATION_SCOTLAND, 0), hf(7, 5, 8, 1998, KEY_NATION_BOSNIA_HERZEGOVINA, 1, KEY_NATION_ESTONIA, 1),
    hf(7, 6, 8, 1998, KEY_NATION_FAROE_ISLANDS, 0, KEY_NATION_CZECH_REPUBLIC, 1), hf(7, 10, 9, 1998, KEY_NATION_SCOTLAND, 3, KEY_NATION_ESTONIA, 2),
    hf(7, 10, 9, 1998, KEY_NATION_BOSNIA_HERZEGOVINA, 1, KEY_NATION_CZECH_REPUBLIC, 3), hf(7, 10, 9, 1998, KEY_NATION_LITHUANIA, 0, KEY_NATION_FAROE_ISLANDS, 0),
    hf(7, 14, 9, 1998, KEY_NATION_CZECH_REPUBLIC, 4, KEY_NATION_ESTONIA, 1), hf(7, 14, 9, 1998, KEY_NATION_SCOTLAND, 2, KEY_NATION_FAROE_ISLANDS, 1),
    hf(7, 14, 9, 1998, KEY_NATION_LITHUANIA, 4, KEY_NATION_BOSNIA_HERZEGOVINA, 2), hf(7, 27, 2, 1999, KEY_NATION_CZECH_REPUBLIC, 2, KEY_NATION_LITHUANIA, 0),
    hf(7, 31, 2, 1999, KEY_NATION_LITHUANIA, 1, KEY_NATION_ESTONIA, 2), hf(7, 31, 2, 1999, KEY_NATION_SCOTLAND, 1, KEY_NATION_CZECH_REPUBLIC, 2),
    hf(7, 5, 5, 1999, KEY_NATION_FAROE_ISLANDS, 1, KEY_NATION_SCOTLAND, 1), hf(7, 5, 5, 1999, KEY_NATION_ESTONIA, 0, KEY_NATION_CZECH_REPUBLIC, 2),
    hf(7, 5, 5, 1999, KEY_NATION_BOSNIA_HERZEGOVINA, 2, KEY_NATION_LITHUANIA, 0), hf(7, 9, 5, 1999, KEY_NATION_FAROE_ISLANDS, 2, KEY_NATION_BOSNIA_HERZEGOVINA, 2),
    hf(7, 9, 5, 1999, KEY_NATION_ESTONIA, 1, KEY_NATION_LITHUANIA, 2), hf(7, 9, 5, 1999, KEY_NATION_CZECH_REPUBLIC, 3, KEY_NATION_SCOTLAND, 2),
    hf(7, 4, 8, 1999, KEY_NATION_BOSNIA_HERZEGOVINA, 1, KEY_NATION_SCOTLAND, 2), hf(7, 4, 8, 1999, KEY_NATION_FAROE_ISLANDS, 0, KEY_NATION_ESTONIA, 2),
    hf(7, 4, 8, 1999, KEY_NATION_LITHUANIA, 0, KEY_NATION_CZECH_REPUBLIC, 4), hf(7, 8, 8, 1999, KEY_NATION_ESTONIA, 0, KEY_NATION_SCOTLAND, 0),
    hf(7, 8, 8, 1999, KEY_NATION_FAROE_ISLANDS, 0, KEY_NATION_LITHUANIA, 1), hf(7, 8, 8, 1999, KEY_NATION_CZECH_REPUBLIC, 3, KEY_NATION_BOSNIA_HERZEGOVINA, 0),
    hf(7, 5, 9, 1999, KEY_NATION_SCOTLAND, 1, KEY_NATION_BOSNIA_HERZEGOVINA, 0), hf(7, 9, 9, 1999, KEY_NATION_CZECH_REPUBLIC, 2, KEY_NATION_FAROE_ISLANDS, 0),
    hf(7, 9, 9, 1999, KEY_NATION_SCOTLAND, 3, KEY_NATION_LITHUANIA, 0), hf(7, 9, 9, 1999, KEY_NATION_ESTONIA, 1, KEY_NATION_BOSNIA_HERZEGOVINA, 4),
];
