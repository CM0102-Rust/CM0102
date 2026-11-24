// ---------- sx03_2: England's awards: the yearly awards, the month awards and the team of the week (england_awards.cpp) ----------

/// NationAwards::daily_update (0x768310, thiscall(today), ret 4): the month change check of every nation's awards.
const FN_NATION_AWARDS_DAILY_UPDATE: usize = 0x768310;
/// nation_awards_award_player_of_year (0x768cf0, thiscall(comp id, today, award, 7 filters), ret 0x28).
const FN_NATION_AWARDS_AWARD_PLAYER_OF_YEAR: usize = 0x768cf0;
/// NationAwards::award_team_of_season (0x7698f0, thiscall(comp id, award), ret 8).
const FN_NATION_AWARDS_AWARD_TEAM_OF_SEASON: usize = 0x7698f0;
/// nation_awards_award_manager_of_year (0x768940, thiscall(today, award, comp id), ret 0xc).
const FN_NATION_AWARDS_AWARD_MANAGER_OF_YEAR: usize = 0x768940;
/// nation_awards_build_team_of_the_week (0x769c70, thiscall(comp id, award), ret 8).
const FN_NATION_AWARDS_BUILD_TEAM_OF_THE_WEEK: usize = 0x769c70;
/// MONTH_AWARD::start_new_season (0x7672e0, thiscall(year), ret 4).
const FN_MONTH_AWARD_START_NEW_SEASON: usize = 0x7672e0;
/// MONTH_AWARD::award_best_rated_player (0x767150, thiscall(month ratings, age), ret 8).
const FN_MONTH_AWARD_AWARD_BEST_RATED_PLAYER: usize = 0x767150;
/// Staff_Records::find_manager_of_the_month (0x86f950, thiscall(comp id), ret 4): the person, or 0.
const FN_STAFF_RECORDS_FIND_MANAGER_OF_THE_MONTH: usize = 0x86f950;

/// A nation's awards object (`NationAwards`): the award table (+4, pointers), the month ratings (+0xa) and the nation
/// record (+0xe, 0 for the international and European awards).
const NATION_AWARDS_TABLE: usize = 0x4;
const NATION_AWARDS_MONTH_RATINGS: usize = 0xa;
const NATION_AWARDS_NATION: usize = 0xe;

/// Award `k` of a nation's awards object.
pub(crate) unsafe fn nation_award(this: usize, k: usize) -> usize {
    read_usize(read_usize(this + NATION_AWARDS_TABLE) + k * 4)
}

/// Whether `today` (a Date: day of the year, year) is day `day` of month `month` (zero-based) of its own year
/// (`exe::date_date_549f70` with any weekday).
pub(crate) unsafe fn awards_day_is(today: usize, day: u32, month: u32) -> bool {
    is_date_today(make_date_dword(day, month, read_u16(today + cm_date::YEAR) as u32, u32::MAX), today)
}

/// The yearly player award of a competition from its stats table (`nation_awards_award_player_of_year`): the last seven
/// arguments are the filters (min age or −1, position flags, appearances...) as each nation passes them.
pub(crate) unsafe fn award_player_of_year(this: usize, comp_key: usize, today: usize, award: usize, filters: [i32; 7]) {
    let f: unsafe extern "thiscall" fn(usize, u32, usize, usize, i32, i32, i32, i32, i32, i32, i32) = core::mem::transmute(FN_NATION_AWARDS_AWARD_PLAYER_OF_YEAR);
    let [a, b, c, d, e, g, h] = filters;
    f(this, read_u32(comp_key), today, award, a, b, c, d, e, g, h)
}

/// The team of the season of a league (`NationAwards::award_team_of_season`).
pub(crate) unsafe fn award_team_of_season(this: usize, comp_key: usize, award: usize) {
    let f: unsafe extern "thiscall" fn(usize, u32, usize) = core::mem::transmute(FN_NATION_AWARDS_AWARD_TEAM_OF_SEASON);
    f(this, read_u32(comp_key), award)
}

/// The manager of the year of a league (`nation_awards_award_manager_of_year`).
pub(crate) unsafe fn award_manager_of_year(this: usize, today: usize, award: usize, comp_key: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(FN_NATION_AWARDS_AWARD_MANAGER_OF_YEAR);
    f(this, today, award, read_u32(comp_key))
}

/// Moves a month award's slots on to the new season (`MONTH_AWARD::start_new_season(year)`).
pub(crate) unsafe fn month_award_start_new_season(award: usize, year: u16) {
    let f: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_MONTH_AWARD_START_NEW_SEASON);
    f(award, year as u32)
}

/// The player of the month of a month award from the nation's month ratings, for an age limit or −1
/// (`MONTH_AWARD::award_best_rated_player`).
pub(crate) unsafe fn award_month_player(this: usize, award: usize, age: i32) {
    let f: unsafe extern "thiscall" fn(usize, usize, i32) = core::mem::transmute(FN_MONTH_AWARD_AWARD_BEST_RATED_PLAYER);
    f(award, read_usize(this + NATION_AWARDS_MONTH_RATINGS), age)
}

/// The manager of the month of a league: the best manager of the month (`Staff_Records::find_manager_of_the_month`)
/// takes the month award with his club, in the month and year of the staff records' last daily update
/// (`staff_records_fields::CURRENT_MONTH`/`CURRENT_YEAR`); none, nothing. The club is read without a check, as in the
/// original.
pub(crate) unsafe fn award_month_manager(award: usize, comp_key: usize) {
    let find: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(FN_STAFF_RECORDS_FIND_MANAGER_OF_THE_MONTH);
    let manager = find(read_usize(G_STAFF_RECORDS), read_u32(comp_key));
    if manager == 0 {
        return;
    }
    let records = read_usize(G_STAFF_RECORDS);
    let manager = Staff(manager);
    // the high bytes of the month and year arguments are the registers' (Staff_Records::add_month_award_record keeps
    // only the low byte and word)
    exe::month_award_set_winner(award, read_i32(manager.addr()), read_i32(manager.club_job().addr()), read_u8(records + staff_records_fields::CURRENT_MONTH) as u32, read_u16(records + staff_records_fields::CURRENT_YEAR) as u32);
}

/// The team of the week of an award's competition (`nation_awards_build_team_of_the_week(comp id, award)`).
pub(crate) unsafe fn build_team_of_the_week(this: usize, award: usize) {
    let f: unsafe extern "thiscall" fn(usize, u32, usize) = core::mem::transmute(FN_NATION_AWARDS_BUILD_TEAM_OF_THE_WEEK);
    f(this, read_u32(read_usize(award + 9)), award)
}

/// Whether the awards' nation plays its lower divisions (the Conference for England: `league_selected` with
/// `LOWER_DIVISIONS_LOADED`).
pub(crate) unsafe fn awards_lower_divisions(this: usize) -> bool {
    Nation(read_usize(this + NATION_AWARDS_NATION)).league_selected() & league_selected::LOWER_DIVISIONS_LOADED != 0
}

/// England's four divisions in award order: the competition key, the first award slot of the division (a block of six:
/// team, manager of the month, player of the month, young player of the month, manager of the year, team of the season
/// in the order the routines below use).
const ENGLAND_DIVISIONS: [usize; 4] = [KEY_CLUB_COMP_ENGLISH_PREMIER_DIVISION, KEY_CLUB_COMP_ENGLISH_FIRST_DIVISION, KEY_CLUB_COMP_ENGLISH_SECOND_DIVISION, KEY_CLUB_COMP_ENGLISH_THIRD_DIVISION];

/// Gives England's yearly awards on their dates, moves the month awards on at the season's end and runs the base check.
///
/// `EnglandAwards::daily_update(today)`, the virtual +4 of England's awards (from `award_manager_process_date`).
/// - 5 April: the player of the year (award 0) and the young player of the year (award 1, up to 21: filters −1, 0,
///   0x11, −1... and 0x15, 0, 0x11, −1...) from the Premier Division's stats (`award_player_of_year`), then the team of
///   the season of each division (awards 7, 13, 19, 25) and, with the lower divisions (`awards_lower_divisions`), the
///   Conference (award 31).
/// - 30 May: the manager of the year of each division (awards 6, 12, 18, 24; the Conference's 30).
/// - The day of the year the nation's season ends (`key_nation_record::SEASON_END_DAY` of the nation's
///   `G_LEAGUE_TABLE` row, `exe::key_nation_index_of_nation`, signed day against the unsigned end day): the month
///   awards (3, 9, 15, 21, then 4, 5, 10, 11, 16, 17, 22, 23; with the lower divisions 28, 29, 27) start the new season
///   with today's year.
/// - Always: the base month change check (`NationAwards::daily_update`).
/// Original: `EnglandAwards::daily_update` (0x579610, england_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn england_awards_daily_update(this: usize, today: usize) {
    if awards_day_is(today, 5, 3) {
        covered!("sx03_2 england year awards");
        award_player_of_year(this, KEY_CLUB_COMP_ENGLISH_PREMIER_DIVISION, today, nation_award(this, 0), [-1, 0, 0x11, -1, -1, -1, -1]);
        award_player_of_year(this, KEY_CLUB_COMP_ENGLISH_PREMIER_DIVISION, today, nation_award(this, 1), [0x15, 0, 0x11, -1, -1, -1, -1]); // sx03_2: young player up to 21
        for (k, key) in ENGLAND_DIVISIONS.iter().enumerate() {
            award_team_of_season(this, *key, nation_award(this, 7 + 6 * k));
        }
        if awards_lower_divisions(this) {
            award_team_of_season(this, KEY_CLUB_COMP_ENGLISH_CONFERENCE, nation_award(this, 31));
        }
    }
    if awards_day_is(today, 0x1e, 4) {
        covered!("sx03_2 england managers of the year");
        for (k, key) in ENGLAND_DIVISIONS.iter().enumerate() {
            award_manager_of_year(this, today, nation_award(this, 6 + 6 * k), *key); // sx03_2: manager of the year
        }
        if awards_lower_divisions(this) {
            award_manager_of_year(this, today, nation_award(this, 30), KEY_CLUB_COMP_ENGLISH_CONFERENCE);
        }
    }
    let row = G_LEAGUE_TABLE.wrapping_add((exe::key_nation_index_of_nation(read_usize(this + NATION_AWARDS_NATION)) as isize * 0x48) as usize);
    if read_i16(today) as i32 == read_u16(row + key_nation_record::SEASON_END_DAY) as i32 {
        covered!("sx03_2 england month awards new season");
        let year = read_u16(today + cm_date::YEAR);
        for k in [3, 9, 15, 21, 4, 5, 10, 11, 16, 17, 22, 23] {
            month_award_start_new_season(nation_award(this, k), year); // sx03_2: month awards new season
        }
        if awards_lower_divisions(this) {
            for k in [28, 29, 27] {
                month_award_start_new_season(nation_award(this, k), year);
            }
        }
    }
    let base: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_NATION_AWARDS_DAILY_UPDATE);
    base(this, today);
}

// simplification: the awards post news, write history records and raise reputations: the routine cannot run twice.
// Original only; the copy is proven by the career.
pub(crate) unsafe extern "thiscall" fn compare_england_awards_daily_update(this: usize, today: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%)(this, today)
}

/// Gives England's month awards at the turn of the month: the managers, then the players.
///
/// `EnglandAwards::on_month_change()`, the virtual +0xc of England's awards (from `NationAwards::daily_update` when the
/// month changes, before the month ratings are reset). The manager of the month (`award_month_manager`) of each
/// division (awards 3, 9, 15, 21) and, with the lower divisions, the Conference (27); then the player of the month and
/// the young player of the month (up to 20) of each division (awards 4 and 5, 10 and 11, 16 and 17, 22 and 23; the
/// Conference's 28 and 29) from the month ratings (`award_month_player`).
/// Original: `EnglandAwards::on_month_change` (0x579900, england_awards.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn england_awards_on_month_change(this: usize) {
    for (k, key) in ENGLAND_DIVISIONS.iter().enumerate() {
        award_month_manager(nation_award(this, 3 + 6 * k), *key); // sx03_2: manager of the month
    }
    let lower = awards_lower_divisions(this);
    if lower {
        covered!("sx03_2 conference manager of the month");
        award_month_manager(nation_award(this, 27), KEY_CLUB_COMP_ENGLISH_CONFERENCE);
    }
    covered!("sx03_2 england month players");
    for k in 0..4 {
        award_month_player(this, nation_award(this, 4 + 6 * k), -1);
        award_month_player(this, nation_award(this, 5 + 6 * k), 0x14); // sx03_2: young player of the month
    }
    if awards_lower_divisions(this) {
        award_month_player(this, nation_award(this, 28), -1);
        award_month_player(this, nation_award(this, 29), 0x14);
    }
}

// simplification: the month winners get news, history records and reputation: the routine cannot run twice. Original
// only; the copy is proven by the career.
pub(crate) unsafe extern "thiscall" fn compare_england_awards_on_month_change(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this)
}

/// Builds the team of the week of an award when it is the team award of one of England's divisions.
///
/// `EnglandAwards::update_team_of_the_week(award)`, the virtual +0x10 of England's awards. The team awards are 2, 8, 14,
/// 20 and, with the lower divisions, 26 (the Conference); for one of them, `build_team_of_the_week` with the award's
/// competition (+9).
/// Original: `EnglandAwards::update_team_of_the_week` (0x579af0, england_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn england_awards_update_team_of_the_week(this: usize, award: usize) {
    let team_award = [2, 8, 14, 20].iter().any(|&k| nation_award(this, k) == award) // sx03_2: the division team awards
        || awards_lower_divisions(this) && nation_award(this, 26) == award;
    if team_award {
        covered!("sx03_2 england team of the week");
        build_team_of_the_week(this, award);
    }
}

// simplification: the team of the week allocates its candidate lists and writes the nominations: it cannot run twice.
// Original only; the copy is proven by the career.
pub(crate) unsafe extern "thiscall" fn compare_england_awards_update_team_of_the_week(this: usize, award: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I2%%)(this, award)
}
