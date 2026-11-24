// ---------- sx04c_12: the Russian awards: the object, its seven awards and the season's end (award/russia_awards.cpp) ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\award\russia_awards.cpp" (the error box).
const RUSSIA_AWARDS_CPP: usize = 0xa7ad00;
/// `RussiaAwards::init_awards(file)` (0x7ec2a0, thiscall, ret 4), through its hook.
const FN_RUSSIA_AWARDS_INIT: usize = 0x7ec2a0;

/// The Russian awards' class: the vtable 0x96f32c, Russia (`KEY_NATION_RUSSIA`), nation id 0x1b, type 1, no month
/// ratings (so `RussiaAwards::on_month_change` never runs: defect of the original, the two players of the month are
/// never given).
const RUSSIA_AWARDS: NationAwardsClass = NationAwardsClass { vtable: 0x96f32c, nation_key: Some(KEY_NATION_RUSSIA), nation_id: 0x1b, kind: 1, month_ratings: None };

/// Builds the Russian awards (`nation_awards_construct` with `RUSSIA_AWARDS`): the saved awards with a file, else
/// `RussiaAwards::init_awards(file)`. Returns this.
/// Original: `RussiaAwards::RussiaAwards` (0x7ec210, russia_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn russia_awards_construct(this: usize, file: usize) -> usize {
    nation_awards_construct(this, &RUSSIA_AWARDS, file, core::mem::transmute(FN_RUSSIA_AWARDS_INIT))
}

/// Builds the seven Russian awards (`nation_awards_build`, the error box at line 0x69): the Russian player of the year
/// (national and world reputations 4000/2000, no competition, kind 0); then, for the Premier Division and the First
/// Division, the team of the week and the team of the year (team awards, 0/0, span 0, formation mode 3) and the player
/// of the month (a month award from April to November of the current year, 4000/2000 and 1000/500).
/// Original: `RussiaAwards::init_awards` (0x7ec2a0, russia_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn russia_awards_init(this: usize, file: usize) {
    let y = read_u16(G_CURRENT_DATE_YEAR) as u32;
    let n = read_u32(this + nation_awards_fields::NATION_ID);
    let f = file as u32;
    let k = |key: usize| read_u32(key);
    let (prem, first) = (k(KEY_CLUB_COMP_RUSSIAN_PREMIER_DIVISION), k(KEY_CLUB_COMP_RUSSIAN_FIRST_DIVISION));
    let team = |i: u32, key: usize, comp: u32| AwardSpec::Team([n, i, k(key), 0, 0, comp, 0, 3, f, 1]);
    let month = |i: u32, key: usize, national: u32, world: u32, comp: u32| AwardSpec::Month([n, i, k(key), national, world, comp, 3, y, 0xa, y, f, 1]);
    let specs = [
        AwardSpec::Year([n, 0, k(KEY_STAFF_COMP_RUSSIAN_PLAYER_OF_THE_YEAR), 4000, 2000, u32::MAX, 0, 0, 0, f, 1]),
        team(1, KEY_STAFF_COMP_RUSSIAN_PREMIER_DIVISION_TEAM_OF_THE_WEE, prem),
        team(2, KEY_STAFF_COMP_RUSSIAN_PREMIER_DIVISION_TEAM_OF_THE_YEA, prem),
        month(3, KEY_STAFF_COMP_RUSSIAN_PREMIER_DIVISION_PLAYER_OF_THE_M, 4000, 2000, prem),
        team(4, KEY_STAFF_COMP_RUSSIAN_FIRST_DIVISION_TEAM_OF_THE_WEEK, first),
        team(5, KEY_STAFF_COMP_RUSSIAN_FIRST_DIVISION_TEAM_OF_THE_YEAR, first),
        month(6, KEY_STAFF_COMP_RUSSIAN_FIRST_DIVISION_PLAYER_OF_THE_MON, 1000, 500, first),
    ];
    covered!("sx04c_12 russian awards built");
    nation_awards_build(this, &specs, RUSSIA_AWARDS_CPP, 0x69);
}

/// The Russian awards' day: on 30 November (`awards_day_is`), the Russian player of the year from the year ratings'
/// top three (`award_top_three_from_ratings`: Russian players, mode 0x11) and the teams of the season of the Premier
/// Division and the First Division; then, every day, the base daily update (the month awards are never moved on).
/// Original: `RussiaAwards::daily_update` (0x7ec600, russia_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn russia_awards_daily_update(this: usize, today: usize) {
    if awards_day_is(today, 0x1e, 0xa) {
        covered!("sx04c_12 russian awards day");
        award_top_three_from_ratings(this, today, nation_award(this, 0), [-1, -1, -1, read_i32(KEY_NATION_RUSSIA), -1, 0, 0x11]);
        award_team_of_season(this, KEY_CLUB_COMP_RUSSIAN_PREMIER_DIVISION, nation_award(this, 2));
        award_team_of_season(this, KEY_CLUB_COMP_RUSSIAN_FIRST_DIVISION, nation_award(this, 5));
    }
    nation_awards_base_daily_update(this, today);
}

pub(crate) unsafe extern "thiscall" fn compare_russia_awards_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, file);
    compare_nation_awards_built(r, this, file, &RUSSIA_AWARDS, 7, "russia_awards_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_russia_awards_init(this: usize, file: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%);
    compare_nation_awards_init(this, || o(this, file), "russia_awards_init");
}

/// Comparer for `russia_awards_daily_update`: the awards cannot be given twice; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_russia_awards_daily_update(this: usize, today: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I2%%)(this, today)
}
