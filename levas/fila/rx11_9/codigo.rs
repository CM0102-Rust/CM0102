// ---------- rx11_9: the Irish awards: the object, its eighteen awards and the awards day (award/ireland_awards.cpp) ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\award\ireland_awards.cpp" (the error boxes).
const IRELAND_AWARDS_CPP: usize = 0x9ea0f4;
/// `IrelandAwards::init_awards(file)` (0x63b790, thiscall, ret 4), through its hook.
const FN_IRELAND_AWARDS_INIT: usize = 0x63b790;

/// The Irish awards' class: the vtable 0x96c17c, the Republic of Ireland (`KEY_NATION_REPUBLIC_OF_IRELAND`), nation id
/// 0xc, type 1, with month ratings (the error box at line 0x57 when they cannot be built).
const IRELAND_AWARDS: NationAwardsClass = NationAwardsClass { vtable: 0x96c17c, nation_key: Some(KEY_NATION_REPUBLIC_OF_IRELAND), nation_id: 0xc, kind: 1,
    month_ratings: Some((IRELAND_AWARDS_CPP, 0x57)) };

/// Ireland's two divisions: the competition key and the first slot of the division's block (team of the week, player,
/// young player, top goalscorer (boot), team of the year, manager of the year, manager of the month).
const IRELAND_DIVISIONS: [(usize, usize); 2] = [(KEY_CLUB_COMP_IRISH_PREMIER_DIVISION, 4), (KEY_CLUB_COMP_IRISH_FIRST_DIVISION, 11)];

/// rule: the Irish awards object is built for the Republic of Ireland with month ratings; from a saved game of version
/// 0x3e028 or later it loads them, otherwise it creates its awards
///
/// `nation_awards_construct_from_version` with `IRELAND_AWARDS` and `IrelandAwards::init_awards`. Returns this.
/// Original: `IrelandAwards::IrelandAwards` (0x63b640, ireland_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn ireland_awards_construct(this: usize, file: usize) -> usize {
    nation_awards_construct_from_version(this, &IRELAND_AWARDS, file, core::mem::transmute(FN_IRELAND_AWARDS_INIT))
}

/// rule: Ireland has eighteen awards: the Irish player, young player and personality of the year and the player of the
/// month (September to April); then for each division the team of the week, the player, the young player, the boot
/// (golden for the Premier, bronze for the First), the team of the year, the manager of the year and the manager of the
/// month
///
/// The count +8 = 0x12, the table (`nation_awards_build`; the error box at line 0x7f), kind 1, no scope: the player
/// (3000/1500), the young player (1500/750) and the personality (0/0) of the year, no competition; the player of the
/// month (a month award from September (8) of the current year to April (3) of the next, 3000/1500, for players); then
/// per division from slot 4 + 7k the team of the week (team award, span 1), the player, young player and boot (option 1)
/// of the year, the team of the year, the manager of the year (not for players) and the manager of the month
/// (September to April, not for players), by the reputations of `IRELAND_DIVISION_REPUTATIONS`.
/// Original: `IrelandAwards::init_awards` (0x63b790, ireland_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn ireland_awards_init(this: usize, file: usize) {
    let y = read_u16(G_CURRENT_DATE_YEAR) as u32;
    let n = read_u32(this + nation_awards_fields::NATION_ID);
    let f = file as u32;
    let k = |key: usize| read_u32(key);
    let year = |i: u32, key: usize, rep: (u32, u32), comp: u32, option: u32, players: u32| AwardSpec::Year([n, i, k(key), rep.0, rep.1, comp, 1, 0, option, f, players]);
    let team = |i: u32, key: usize, rep: (u32, u32), comp: u32| AwardSpec::Team([n, i, k(key), rep.0, rep.1, comp, 1, 0, f, 1]);
    let month = |i: u32, key: usize, rep: (u32, u32), comp: u32, players: u32| AwardSpec::Month([n, i, k(key), rep.0, rep.1, comp, 8, y, 3, y + 1, f, players]);
    let mut specs = vec![
        year(0, KEY_STAFF_COMP_IRISH_PLAYER_OF_THE_YEAR, (3000, 1500), u32::MAX, 0, 1),
        year(1, KEY_STAFF_COMP_IRISH_YOUNG_PLAYER_OF_THE_YEAR, (1500, 750), u32::MAX, 0, 1),
        year(2, KEY_STAFF_COMP_IRISH_PERSONALITY_OF_THE_YEAR, (0, 0), u32::MAX, 0, 1),
        month(3, KEY_STAFF_COMP_IRISH_PLAYER_OF_THE_MONTH, (3000, 1500), u32::MAX, 1), // rx11_9: September to April
    ];
    for (d, (keys, r)) in IRELAND_DIVISION_REPUTATIONS.iter().enumerate() {
        let comp = k(IRELAND_DIVISIONS[d].0);
        let i = IRELAND_DIVISIONS[d].1 as u32;
        specs.push(team(i, keys[0], r[0], comp));
        specs.push(year(i + 1, keys[1], r[1], comp, 0, 1));
        specs.push(year(i + 2, keys[2], r[2], comp, 0, 1));
        specs.push(year(i + 3, keys[3], r[3], comp, 1, 1));
        specs.push(team(i + 4, keys[4], r[4], comp));
        specs.push(year(i + 5, keys[5], r[5], comp, 0, 0));
        specs.push(month(i + 6, keys[6], r[6], comp, 0));
    }
    covered!("rx11_9 irish awards built");
    nation_awards_build(this, &specs, IRELAND_AWARDS_CPP, 0x7f);
}

/// The Irish divisions' award blocks for `ireland_awards_init`, Premier then First: the seven staff competition keys
/// and their (national, world) reputations.
pub(crate) const IRELAND_DIVISION_REPUTATIONS: [([usize; 7], [(u32, u32); 7]); 2] = [
    ([KEY_STAFF_COMP_IRISH_PREMIER_DIVISION_TEAM_OF_THE_WEEK, KEY_STAFF_COMP_IRISH_PREMIER_DIVISION_PLAYER_OF_THE_YEA,
        KEY_STAFF_COMP_IRISH_PREMIER_DIVISION_YOUNG_PLAYER_OF_T, KEY_STAFF_COMP_IRISH_LEAGUE_GOLDEN_BOOT,
        KEY_STAFF_COMP_IRISH_PREMIER_DIVISION_TEAM_OF_THE_YEAR, KEY_STAFF_COMP_IRISH_PREMIER_DIVISION_MANAGER_OF_THE_YE,
        KEY_STAFF_COMP_IRISH_PREMIER_DIVISION_MANAGER_OF_THE_MO],
        [(1000, 500), (1500, 750), (500, 250), (500, 250), (1500, 750), (2500, 1200), (2000, 1000)]),
    ([KEY_STAFF_COMP_IRISH_FIRST_DIVISION_TEAM_OF_THE_WEEK, KEY_STAFF_COMP_IRISH_FIRST_DIVISION_PLAYER_OF_THE_YEAR,
        KEY_STAFF_COMP_IRISH_FIRST_DIVISION_YOUNG_PLAYER_OF_THE, KEY_STAFF_COMP_IRISH_LEAGUE_BRONZE_BOOT,
        KEY_STAFF_COMP_IRISH_FIRST_DIVISION_TEAM_OF_THE_YEAR, KEY_STAFF_COMP_IRISH_FIRST_DIVISION_MANAGER_OF_THE_YEAR,
        KEY_STAFF_COMP_IRISH_FIRST_DIVISION_MANAGER_OF_THE_MONT],
        [(500, 250), (750, 400), (300, 150), (500, 250), (500, 250), (1000, 500), (1000, 500)]),
];

/// rule: on 1 May the Irish player, young player (up to 23) and personality of the year come from the world year
/// ratings and, in each division, the player, the young player (up to 23) and the boot from the stats, the team of the
/// season and the manager of the year; on the nation's season end day the three month awards move on to the next
/// season; then, every day, the base daily update
///
/// 1 May (`awards_day_is`(today, 1, 4)): `award_player_of_year_from_world_ratings` for awards 0, 1 and 2 with the filters
/// [Ireland, −1, −1, −1, −1, 0, −1], [Ireland, −1, −1, −1, −1, 0, 0x17] and [−1, −1, −1, Ireland, −1, 5, −1] (Ireland =
/// [`KEY_NATION_REPUBLIC_OF_IRELAND`]); per division (`award_player_of_year`) the player ([−1, 0, 0x11]), the young
/// player ([0x17, 0, 0x11]) and the boot ([−1, 0, 2]), then `award_team_of_season` and `award_manager_of_year`. The
/// season end day of the nation (`is_season_end_day`): `month_award_start_new_season` for awards 3, 10 and 17 with
/// today's year. Last `nation_awards_base_daily_update`.
/// Original: `IrelandAwards::daily_update` (0x63bf10, ireland_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn ireland_awards_daily_update(this: usize, today: usize) {
    if awards_day_is(today, 1, 4) { // rx11_9: 1 May
        covered!("rx11_9 irish awards day");
        let ie = read_i32(KEY_NATION_REPUBLIC_OF_IRELAND);
        for (k, filters) in [(0, [ie, -1, -1, -1, -1, 0, -1]), (1, [ie, -1, -1, -1, -1, 0, 0x17]), (2, [-1, -1, -1, ie, -1, 5, -1])] {
            award_player_of_year_from_world_ratings(this, today, nation_award(this, k), filters);
        }
        for &(key, first) in &IRELAND_DIVISIONS {
            for (j, filters) in [(1, [-1, 0, 0x11, -1, -1, -1, -1]), (2, [0x17, 0, 0x11, -1, -1, -1, -1]), (3, [-1, 0, 2, -1, -1, -1, -1])] {
                award_player_of_year(this, key, today, nation_award(this, first + j), filters); // rx11_9: player, young player, boot
            }
            award_team_of_season(this, key, nation_award(this, first + 4));
            award_manager_of_year(this, today, nation_award(this, first + 5), key);
        }
    }
    if is_season_end_day(read_usize(this + nation_awards_fields::NATION), today) {
        covered!("rx11_9 irish month awards move on");
        for k in [3, 10, 17] {
            month_award_start_new_season(nation_award(this, k), read_u16(today + cm_date::YEAR));
        }
    }
    nation_awards_base_daily_update(this, today);
}

pub(crate) unsafe extern "thiscall" fn compare_ireland_awards_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, file);
    compare_nation_awards_built(r, this, file, &IRELAND_AWARDS, 0x12, "ireland_awards_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_ireland_awards_init(this: usize, file: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%);
    compare_nation_awards_init(this, || o(this, file), "ireland_awards_init");
}

/// Comparer for `ireland_awards_daily_update`: the awards cannot be given twice; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_ireland_awards_daily_update(this: usize, today: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I2%%)(this, today)
}
