// ---------- rx12_1: the Finnish awards: the object, its fourteen awards and the season's end (award/finland_awards.cpp) ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\award\finland_awards.cpp" (the error boxes).
const FINLAND_AWARDS_CPP: usize = 0x9cc0fc;
/// `FinlandAwards::init_awards(file)` (0x5a90b0, thiscall, ret 4), through its hook.
const FN_FINLAND_AWARDS_INIT: usize = 0x5a90b0;

/// The Finnish awards' class: the vtable 0x96a748, Finland (`KEY_NATION_FINLAND`), nation id 8, type 1, with month
/// ratings (the error box at line 0x51 when they cannot be built).
const FINLAND_AWARDS: NationAwardsClass = NationAwardsClass { vtable: 0x96a748, nation_key: Some(KEY_NATION_FINLAND), nation_id: 8, kind: 1,
    month_ratings: Some((FINLAND_AWARDS_CPP, 0x51)) };

/// Builds the Finnish awards (`nation_awards_construct` with `FINLAND_AWARDS`): the saved awards with a file, else the
/// month ratings and `FinlandAwards::init_awards(file)`. Returns this.
/// Original: `FinlandAwards::FinlandAwards` (0x5a8f70, finland_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn finland_awards_construct(this: usize, file: usize) -> usize {
    nation_awards_construct(this, &FINLAND_AWARDS, file, core::mem::transmute(FN_FINLAND_AWARDS_INIT))
}

/// Builds the fourteen Finnish awards (`nation_awards_build`, the error box at line 0x79), kind 0, no scope: the
/// Finnish player (national and world reputations 3000/1500), young player (1500/1000), goalkeeper (1000/500) and
/// manager of the year (2000/1000, not for players), no competition; for the Premier Division the top goalscorer
/// (2500/1250, option 1), the player and the manager of the month (month awards from May to October of the current
/// year, 1000/500), the team of the year and the team of the week (team awards, 0/0, span 0, mode 0); the same five for
/// the First Division (1200/600 and 500/200).
/// Original: `FinlandAwards::init_awards` (0x5a90b0, finland_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn finland_awards_init(this: usize, file: usize) {
    let y = read_u16(G_CURRENT_DATE_YEAR) as u32;
    let n = read_u32(this + nation_awards_fields::NATION_ID);
    let f = file as u32;
    let k = |key: usize| read_u32(key);
    let (d1, d2) = (k(KEY_CLUB_COMP_FINNISH_PREMIER_DIVISION), k(KEY_CLUB_COMP_FINNISH_FIRST_DIVISION));
    let year = |i: u32, key: usize, national: u32, world: u32, comp: u32, option: u32, players: u32| AwardSpec::Year([n, i, k(key), national, world, comp, 0, 0, option, f, players]);
    let month = |i: u32, key: usize, national: u32, world: u32, comp: u32, players: u32| AwardSpec::Month([n, i, k(key), national, world, comp, 4, y, 9, y, f, players]);
    let team = |i: u32, key: usize, comp: u32| AwardSpec::Team([n, i, k(key), 0, 0, comp, 0, 0, f, 1]);
    let specs = [
        year(0, KEY_STAFF_COMP_FINNISH_PLAYER_OF_THE_YEAR, 3000, 1500, u32::MAX, 0, 1), // rx12_1: the player of the year
        year(1, KEY_STAFF_COMP_FINNISH_YOUNG_PLAYER_OF_THE_YEAR, 1500, 1000, u32::MAX, 0, 1),
        year(2, KEY_STAFF_COMP_FINNISH_GOALKEEPER_OF_THE_YEAR, 1000, 500, u32::MAX, 0, 1),
        year(3, KEY_STAFF_COMP_FINNISH_MANAGER_OF_THE_YEAR, 2000, 1000, u32::MAX, 0, 0),
        year(4, KEY_STAFF_COMP_FINNISH_PREMIER_DIVISION_TOP_GOALSCORER, 2500, 1250, d1, 1, 1),
        month(5, KEY_STAFF_COMP_FINNISH_PREMIER_DIVISION_PLAYER_OF_THE_M, 1000, 500, d1, 1),
        month(6, KEY_STAFF_COMP_FINNISH_PREMIER_DIVISION_MANAGER_OF_THE, 1000, 500, d1, 0),
        team(7, KEY_STAFF_COMP_FINNISH_PREMIER_DIVISION_TEAM_OF_THE_YEA, d1),
        team(8, KEY_STAFF_COMP_FINNISH_PREMIER_DIVISION_TEAM_OF_THE_WEE, d1),
        year(9, KEY_STAFF_COMP_FINNISH_FIRST_DIVISION_TOP_GOALSCORER, 1200, 600, d2, 1, 1),
        month(10, KEY_STAFF_COMP_FINNISH_FIRST_DIVISION_PLAYER_OF_THE_MON, 500, 200, d2, 1),
        month(11, KEY_STAFF_COMP_FINNISH_FIRST_DIVISION_MANAGER_OF_THE_MO, 500, 200, d2, 0),
        team(12, KEY_STAFF_COMP_FINNISH_FIRST_DIVISION_TEAM_OF_THE_YEAR, d2),
        team(13, KEY_STAFF_COMP_FINNISH_FIRST_DIVISION_TEAM_OF_THE_WEEK, d2),
    ];
    covered!("rx12_1 finnish awards built");
    nation_awards_build(this, &specs, FINLAND_AWARDS_CPP, 0x79);
}

/// The Finnish awards' day: on 15 November (`awards_day_is`, month index 10), the Finnish player, young player (up to
/// 21) and goalkeeper (position mask 1) of the year from the year ratings' top three (`award_top_three_from_ratings`,
/// with the filters as the original passes them), the manager of the year (from the Premier Division), the Premier
/// Division top goalscorer (statistic 2) and team of the season, and the First Division's. Every day, on the nation's
/// season end day (`is_season_end_day`), the four month awards move on to next year (today's year + 1); then the base
/// daily update.
/// Original: `FinlandAwards::daily_update` (0x5a96e0, finland_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn finland_awards_daily_update(this: usize, today: usize) {
    if awards_day_is(today, 0xf, 0xa) { // rx12_1: 15 November
        covered!("rx12_1 finnish awards day");
        let fin = read_i32(KEY_NATION_FINLAND);
        award_top_three_from_ratings(this, today, nation_award(this, 0), [-1, -1, -1, fin, -1, 0, 0x11]); // rx12_1: the player
        award_top_three_from_ratings(this, today, nation_award(this, 1), [fin, fin, -1, -1, 0x15, 0, 0x11]); // rx12_1: the young player
        award_top_three_from_ratings(this, today, nation_award(this, 2), [-1, fin, -1, -1, -1, 1, 0x11]); // rx12_1: the goalkeeper
        let (d1, d2) = (KEY_CLUB_COMP_FINNISH_PREMIER_DIVISION, KEY_CLUB_COMP_FINNISH_FIRST_DIVISION);
        award_manager_of_year(this, today, nation_award(this, 3), d1);
        award_player_of_year(this, d1, today, nation_award(this, 4), [-1, 0, 2, -1, -1, -1, -1]);
        award_team_of_season(this, d1, nation_award(this, 7));
        award_player_of_year(this, d2, today, nation_award(this, 9), [-1, 0, 2, -1, -1, -1, -1]);
        award_team_of_season(this, d2, nation_award(this, 12));
    }
    if is_season_end_day(read_usize(this + nation_awards_fields::NATION), today) {
        covered!("rx12_1 month awards move on");
        let next = read_u16(today + cm_date::YEAR).wrapping_add(1);
        for k in [5, 6, 10, 11] {
            month_award_start_new_season(nation_award(this, k), next);
        }
    }
    nation_awards_base_daily_update(this, today);
}

pub(crate) unsafe extern "thiscall" fn compare_finland_awards_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, file);
    compare_nation_awards_built(r, this, file, &FINLAND_AWARDS, 14, "finland_awards_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_finland_awards_init(this: usize, file: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%);
    compare_nation_awards_init(this, || o(this, file), "finland_awards_init");
}

/// Comparer for `finland_awards_daily_update`: the awards cannot be given twice; only the original runs (the copy is
/// proven by the career).
pub(crate) unsafe extern "thiscall" fn compare_finland_awards_daily_update(this: usize, today: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I2%%)(this, today)
}
