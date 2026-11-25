// ---------- rx12_5: the Australian awards: the object, its nine awards and the season's end (award/australia_awards.cpp) ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\award\australia_awards.cpp" (the error boxes).
const AUSTRALIA_AWARDS_CPP: usize = 0x987cc0;
/// `AustraliaAwards::init_awards(file)` (0x412680, thiscall, ret 4), through its hook.
const FN_AUSTRALIA_AWARDS_INIT: usize = 0x412680;

/// The Australian awards' class: the vtable 0x96781c, Australia (`KEY_NATION_AUSTRALIA`), nation id 0x18, type 1, with
/// month ratings (the error box at line 0x4c when they cannot be built).
const AUSTRALIA_AWARDS: NationAwardsClass = NationAwardsClass { vtable: 0x96781c, nation_key: Some(KEY_NATION_AUSTRALIA), nation_id: 0x18, kind: 1,
    month_ratings: Some((AUSTRALIA_AWARDS_CPP, 0x4c)) };

/// Builds the Australian awards (`nation_awards_construct` with `AUSTRALIA_AWARDS`): the saved awards with a file, else
/// the month ratings and `AustraliaAwards::init_awards(file)`. Returns this.
/// Original: `AustraliaAwards::AustraliaAwards` (0x412540, australia_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn australia_awards_construct(this: usize, file: usize) -> usize {
    nation_awards_construct(this, &AUSTRALIA_AWARDS, file, core::mem::transmute(FN_AUSTRALIA_AWARDS_INIT))
}

/// Builds the nine Australian awards (`nation_awards_build`, the error box at line 0x74), all of the National Soccer
/// League: the team of the week (team award, national and world reputations 1000/500, span 1, mode 0); the manager
/// (1500/700, not for players), the player (1000/500) and the young player of the month (800/400), month awards from
/// October of the current year to May of the next; the player (2000/1000), under 21 player, goalkeeper and top
/// goalscorer (option 1) of the year (1500/750) and the manager of the year (1500/750, not for players), kind 1, no
/// scope.
/// Original: `AustraliaAwards::init_awards` (0x412680, australia_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn australia_awards_init(this: usize, file: usize) {
    let y = read_u16(G_CURRENT_DATE_YEAR) as u32;
    let n = read_u32(this + nation_awards_fields::NATION_ID);
    let f = file as u32;
    let k = |key: usize| read_u32(key);
    let nsl = k(KEY_CLUB_COMP_AUSTRALIAN_NATIONAL_SOCCER_LEAGUE);
    let year = |i: u32, key: usize, national: u32, world: u32, option: u32, players: u32| AwardSpec::Year([n, i, k(key), national, world, nsl, 1, 0, option, f, players]);
    let month = |i: u32, key: usize, national: u32, world: u32, players: u32| AwardSpec::Month([n, i, k(key), national, world, nsl, 9, y, 4, y + 1, f, players]);
    let specs = [
        AwardSpec::Team([n, 0, k(KEY_STAFF_COMP_AUSTRALIAN_NSL_TEAM_OF_THE_WEEK), 1000, 500, nsl, 1, 0, f, 1]),
        month(1, KEY_STAFF_COMP_AUSTRALIAN_MANAGER_OF_THE_MONTH, 1500, 700, 0),
        month(2, KEY_STAFF_COMP_AUSTRALIAN_PLAYER_OF_THE_MONTH, 1000, 500, 1),
        month(3, KEY_STAFF_COMP_AUSTRALIAN_NSL_YOUNG_PLAYER_OF_THE_MONTH, 800, 400, 1),
        year(4, KEY_STAFF_COMP_AUSTRALIAN_PLAYER_OF_THE_YEAR, 2000, 1000, 0, 1), // rx12_5: the player of the year
        year(5, KEY_STAFF_COMP_AUSTRALIAN_NSL_U21_PLAYER_OF_THE_YEAR, 1500, 750, 0, 1),
        year(6, KEY_STAFF_COMP_AUSTRALIAN_GOALKEEPER_OF_THE_YEAR, 1500, 750, 0, 1),
        year(7, KEY_STAFF_COMP_AUSTRALIAN_TOP_GOALSCORER, 1500, 750, 1, 1),
        year(8, KEY_STAFF_COMP_AUSTRALIAN_MANAGER_OF_THE_YEAR, 1500, 750, 0, 0),
    ];
    covered!("rx12_5 australian awards built");
    nation_awards_build(this, &specs, AUSTRALIA_AWARDS_CPP, 0x74);
}

/// The Australian awards' day: on 20 June (`awards_day_is`, month index 5), from the National Soccer League's season
/// statistics (`award_player_of_year`): the player (any position, statistic 0x11), the under 21 player (up to 21), the
/// goalkeeper (positions 1) and the top goalscorer (positions −1, statistic 2) of the year, then the manager of the
/// year. Every day, on the nation's season end day (`is_season_end_day`), the three month awards move on (today's
/// year); then the base daily update.
/// Original: `AustraliaAwards::daily_update` (0x412ac0, australia_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn australia_awards_daily_update(this: usize, today: usize) {
    if awards_day_is(today, 0x14, 5) { // rx12_5: 20 June
        covered!("rx12_5 australian awards day");
        let nsl = KEY_CLUB_COMP_AUSTRALIAN_NATIONAL_SOCCER_LEAGUE;
        award_player_of_year(this, nsl, today, nation_award(this, 4), [-1, 0, 0x11, -1, -1, -1, -1]);
        award_player_of_year(this, nsl, today, nation_award(this, 5), [0x15, 0, 0x11, -1, -1, -1, -1]); // rx12_5: the under 21 player
        award_player_of_year(this, nsl, today, nation_award(this, 6), [-1, 1, 0x11, -1, -1, -1, -1]); // rx12_5: the goalkeeper
        award_player_of_year(this, nsl, today, nation_award(this, 7), [-1, -1, 2, -1, -1, -1, -1]);
        award_manager_of_year(this, today, nation_award(this, 8), nsl);
    }
    if is_season_end_day(read_usize(this + nation_awards_fields::NATION), today) {
        covered!("rx12_5 month awards move on");
        let year = read_u16(today + cm_date::YEAR);
        for k in [1, 2, 3] {
            month_award_start_new_season(nation_award(this, k), year);
        }
    }
    nation_awards_base_daily_update(this, today);
}

pub(crate) unsafe extern "thiscall" fn compare_australia_awards_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, file);
    compare_nation_awards_built(r, this, file, &AUSTRALIA_AWARDS, 9, "australia_awards_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_australia_awards_init(this: usize, file: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%);
    compare_nation_awards_init(this, || o(this, file), "australia_awards_init");
}

/// Comparer for `australia_awards_daily_update`: the awards cannot be given twice; only the original runs (the copy is
/// proven by the career).
pub(crate) unsafe extern "thiscall" fn compare_australia_awards_daily_update(this: usize, today: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I2%%)(this, today)
}
