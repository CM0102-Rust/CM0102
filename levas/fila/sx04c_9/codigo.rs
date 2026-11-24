// ---------- sx04c_9: the Polish awards: the object, its ten awards and the season's end (award/poland_awards.cpp) ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\award\poland_awards.cpp" (the error box).
const POLAND_AWARDS_CPP: usize = 0xa78680;
/// `PolandAwards::init_awards(file)` (0x7cd1c0, thiscall, ret 4), through its hook.
const FN_POLAND_AWARDS_INIT: usize = 0x7cd1c0;

/// The Polish awards' class: the vtable 0x96e608, Poland (`KEY_NATION_POLAND`), nation id 0x15, type 1, no month
/// ratings (so the base daily update never runs `PolandAwards::on_month_change`: defect of the original, the two
/// managers of the month are never given).
const POLAND_AWARDS: NationAwardsClass = NationAwardsClass { vtable: 0x96e608, nation_key: Some(KEY_NATION_POLAND), nation_id: 0x15, kind: 1, month_ratings: None };

/// Poland's two divisions: the competition key and the first slot of the division's block (team of the week, player,
/// young player, manager of the year, manager of the month).
const POLAND_DIVISIONS: [(usize, usize); 2] = [(KEY_CLUB_COMP_POLISH_FIRST_DIVISION, 0), (KEY_CLUB_COMP_POLISH_SECOND_DIVISION, 5)];

/// Builds the Polish awards (`nation_awards_construct` with `POLAND_AWARDS`): the saved awards with a file, else
/// `PolandAwards::init_awards(file)`. Returns this.
/// Original: `PolandAwards::PolandAwards` (0x7cd130, poland_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn poland_awards_construct(this: usize, file: usize) -> usize {
    nation_awards_construct(this, &POLAND_AWARDS, file, core::mem::transmute(FN_POLAND_AWARDS_INIT))
}

/// Builds the ten Polish awards (`nation_awards_build`, the error box at line 0x6b), kind 1, no scope; for each division
/// the team of the week (team award, national and world reputations 0/0, span 1), the player (First Division 3000/0,
/// Second 2000/0), the young player (2000/0, 1000/0) and the manager of the year (2500/1200, 1500/1000, not for
/// players) and the manager of the month (a month award from July of the current year to May of the next, 2000/1000,
/// 1000/500).
/// Original: `PolandAwards::init_awards` (0x7cd1c0, poland_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn poland_awards_init(this: usize, file: usize) {
    let y = read_u16(G_CURRENT_DATE_YEAR) as u32;
    let n = read_u32(this + nation_awards_fields::NATION_ID);
    let f = file as u32;
    let k = |key: usize| read_u32(key);
    let (d1, d2) = (k(KEY_CLUB_COMP_POLISH_FIRST_DIVISION), k(KEY_CLUB_COMP_POLISH_SECOND_DIVISION));
    let year = |i: u32, key: usize, national: u32, world: u32, comp: u32, players: u32| AwardSpec::Year([n, i, k(key), national, world, comp, 1, 0, 0, f, players]);
    let team = |i: u32, key: usize, comp: u32| AwardSpec::Team([n, i, k(key), 0, 0, comp, 1, 0, f, 1]);
    let month = |i: u32, key: usize, national: u32, world: u32, comp: u32| AwardSpec::Month([n, i, k(key), national, world, comp, 6, y, 4, y + 1, f, 0]);
    let specs = [
        team(0, KEY_STAFF_COMP_POLISH_FIRST_DIVISION_TEAM_OF_THE_WEEK, d1),
        year(1, KEY_STAFF_COMP_POLISH_FIRST_DIVISION_PLAYER_OF_THE_YEAR, 3000, 0, d1, 1),
        year(2, KEY_STAFF_COMP_POLISH_FIRST_DIVISION_YOUNG_PLAYER_OF_TH, 2000, 0, d1, 1),
        year(3, KEY_STAFF_COMP_POLISH_FIRST_DIVISION_MANAGER_OF_THE_YEA, 2500, 1200, d1, 0),
        month(4, KEY_STAFF_COMP_POLISH_FIRST_DIVISION_MANAGER_OF_THE_MON, 2000, 1000, d1),
        team(5, KEY_STAFF_COMP_POLISH_SECOND_DIVISION_TEAM_OF_THE_WEEK, d2),
        year(6, KEY_STAFF_COMP_POLISH_SECOND_DIVISION_PLAYER_OF_THE_YEA, 2000, 0, d2, 1),
        year(7, KEY_STAFF_COMP_POLISH_SECOND_DIVISION_YOUNG_PLAYER_OF_T, 1000, 0, d2, 1),
        year(8, KEY_STAFF_COMP_POLISH_SECOND_DIVISION_MANAGER_OF_THE_YE, 1500, 1000, d2, 0),
        month(9, KEY_STAFF_COMP_POLISH_SECOND_DIVISION_MANAGER_OF_THE_MO, 1000, 500, d2),
    ];
    covered!("sx04c_9 polish awards built");
    nation_awards_build(this, &specs, POLAND_AWARDS_CPP, 0x6b);
}

/// The Polish awards' day: on 30 May (`awards_day_is`), for each division the player (statistic 0x11) and the young
/// player (up to 21) of the year from its stats and its manager of the year. Every day, on the nation's season end day
/// (`is_season_end_day`), the two managers of the month move on (`month_award_start_new_season` with today's year);
/// then the base daily update.
/// Original: `PolandAwards::daily_update` (0x7cd630, poland_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn poland_awards_daily_update(this: usize, today: usize) {
    if awards_day_is(today, 0x1e, 4) {
        covered!("sx04c_9 polish awards day");
        for &(key, first) in &POLAND_DIVISIONS {
            award_player_of_year(this, key, today, nation_award(this, first + 1), [-1, 0, 0x11, -1, -1, -1, -1]);
            award_player_of_year(this, key, today, nation_award(this, first + 2), [0x15, 0, 0x11, -1, -1, -1, -1]);
            award_manager_of_year(this, today, nation_award(this, first + 3), key);
        }
    }
    if is_season_end_day(read_usize(this + nation_awards_fields::NATION), today) {
        covered!("sx04c_9 month awards move on");
        for &(_, first) in &POLAND_DIVISIONS {
            month_award_start_new_season(nation_award(this, first + 4), read_u16(today + cm_date::YEAR));
        }
    }
    nation_awards_base_daily_update(this, today);
}

pub(crate) unsafe extern "thiscall" fn compare_poland_awards_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, file);
    compare_nation_awards_built(r, this, file, &POLAND_AWARDS, 10, "poland_awards_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_poland_awards_init(this: usize, file: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%);
    compare_nation_awards_init(this, || o(this, file), "poland_awards_init");
}

/// Comparer for `poland_awards_daily_update`: the awards cannot be given twice; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_poland_awards_daily_update(this: usize, today: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I2%%)(this, today)
}
