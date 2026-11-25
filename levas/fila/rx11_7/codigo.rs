// ---------- rx11_7: the French awards: the object, its 21 awards and the awards days (award/france_awards.cpp) ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\award\france_awards.cpp" (the error box).
const FRANCE_AWARDS_CPP: usize = 0x9cccbc;
/// `FranceAwards::init_awards(file)` (0x5c0a20, thiscall, ret 4), through its hook.
const FN_FRANCE_AWARDS_INIT: usize = 0x5c0a20;

/// The French awards' class: the vtable 0x96ae34, France (`KEY_NATION_FRANCE`), nation id 9, type 1, no month ratings.
const FRANCE_AWARDS: NationAwardsClass = NationAwardsClass { vtable: 0x96ae34, nation_key: Some(KEY_NATION_FRANCE), nation_id: 9, kind: 1, month_ratings: None };

/// `NationAwards::NationAwards` (0x768290, thiscall, ret 0), `NationAwards::load(file)` (0x76a280, thiscall, ret 4:
/// nonzero when read) and `MONTH_RATINGS::MONTH_RATINGS(file)` (0x767af0, thiscall, ret 4; 0xf bytes), for the
/// constructors that check the save's version (the shared `nation_awards_construct` loads from any save).
const FN_NATION_AWARDS_NEW: usize = 0x768290;
const FN_NATION_AWARDS_LOAD: usize = 0x76a280;
const FN_MONTH_RATINGS_NEW: usize = 0x767af0;

/// The save data version from which a nation's saved awards are read (0x3e028); older saves build them again.
pub(crate) const AWARDS_SAVED_FROM_VERSION: i32 = 0x3e028;

/// The constructor of the nations whose awards entered the save later (France, Ireland): as `nation_awards_construct`,
/// but the saved awards are read only from a save of version `AWARDS_SAVED_FROM_VERSION` or later
/// (`G_GAME_DATA_VERSION`, absolute value: `cdq; xor; sub`); from an older one (or when the read fails) the month
/// ratings (when the class has them) and `init(this, file)` with the file all the same.
pub(crate) unsafe fn nation_awards_construct_from_version(this: usize, class: &NationAwardsClass, file: usize, init: unsafe extern "thiscall" fn(usize, usize)) -> usize {
    use nation_awards_fields as na;
    let base: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_NATION_AWARDS_NEW);
    base(this);
    write_usize(this, class.vtable);
    let nation = class.nation_key.map_or(0, |key| read_usize(G_NATION_TABLE) + read_u32(key) as usize * nation::SIZE);
    write_usize(this + na::NATION, nation);
    write_u32(this + na::NATION_ID, class.nation_id);
    write_u8(this + na::TYPE, class.kind);
    if file != 0 && read_i32(G_GAME_DATA_VERSION).wrapping_abs() >= AWARDS_SAVED_FROM_VERSION {
        let load: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_NATION_AWARDS_LOAD);
        if load(this, file) != 0 {
            return this;
        }
    }
    if let Some((cpp, line)) = class.month_ratings {
        let p = exe::crt_operator_new(0xf);
        let ratings = if p != 0 {
            let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_MONTH_RATINGS_NEW);
            f(p, 0)
        } else {
            0
        };
        write_usize(this + na::MONTH_RATINGS, ratings);
        if ratings == 0 {
            show_error_at(cpp, line);
            return this;
        }
    }
    covered!("rx11_7 nation awards created by version");
    init(this, file);
    this
}

/// rule: the French awards object is built for France; from a saved game of version 0x3e028 or later it loads them,
/// otherwise it creates its awards
///
/// `nation_awards_construct_from_version` with `FRANCE_AWARDS` and `FranceAwards::init_awards`. Returns this.
/// Original: `FranceAwards::FranceAwards` (0x5c0980, france_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn france_awards_construct(this: usize, file: usize) -> usize {
    nation_awards_construct_from_version(this, &FRANCE_AWARDS, file, core::mem::transmute(FN_FRANCE_AWARDS_INIT))
}

/// rule: France has 21 awards: the French manager of the year and five players of the year (player, foreign player,
/// under 23, most popular and most unpopular); then for each division (First, Second, National) the team of the week,
/// the team of the year, the players' player of the year, the player of the year and the goalkeeper of the year
///
/// The count +8 = 0x15, the table (`nation_awards_build`; the error box at line 0x74), no scope: the manager of the
/// year (6000/2500, kind 1, not for players), the player (7000/4500), the foreign player (6500/3500), the under 23
/// player (5000/2000, kind 1), the most popular (5000/1500) and the most unpopular (0/0) players of the year (the four
/// others kind 0), no competition; then per division from slot 6 + 5k: the team of the week and the team of the year
/// (team awards, 0/0, span 1) and three year awards of kind 1 by the reputations of `FRANCE_DIVISION_AWARDS`.
/// Original: `FranceAwards::init_awards` (0x5c0a20, france_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn france_awards_init(this: usize, file: usize) {
    let n = read_u32(this + nation_awards_fields::NATION_ID);
    let f = file as u32;
    let k = |key: usize| read_u32(key);
    let year = |i: u32, key: usize, national: u32, world: u32, comp: u32, kind: u32, players: u32| AwardSpec::Year([n, i, k(key), national, world, comp, kind, 0, 0, f, players]);
    let mut specs = vec![
        year(0, KEY_STAFF_COMP_FRENCH_MANAGER_OF_THE_YEAR, 6000, 2500, u32::MAX, 1, 0),
        year(1, KEY_STAFF_COMP_FRENCH_PLAYER_OF_THE_YEAR, 7000, 4500, u32::MAX, 0, 1),
        year(2, KEY_STAFF_COMP_FRENCH_FOREIGN_PLAYER_OF_THE_YEAR, 6500, 3500, u32::MAX, 0, 1),
        year(3, KEY_STAFF_COMP_FRENCH_UNDER_23_PLAYER_OF_THE_YEAR, 5000, 2000, u32::MAX, 1, 1),
        year(4, KEY_STAFF_COMP_FRENCH_MOST_POPULAR_PLAYER_OF_THE_YEAR, 5000, 1500, u32::MAX, 0, 1),
        year(5, KEY_STAFF_COMP_FRENCH_MOST_UNPOPULAR_PLAYER_OF_THE_YEAR, 0, 0, u32::MAX, 0, 1),
    ];
    for (d, (comp_key, keys, reps)) in FRANCE_DIVISION_AWARDS.iter().enumerate() {
        let comp = k(*comp_key);
        let i = 6 + 5 * d as u32;
        specs.push(AwardSpec::Team([n, i, k(keys[0]), 0, 0, comp, 1, 0, f, 1]));
        specs.push(AwardSpec::Team([n, i + 1, k(keys[1]), 0, 0, comp, 1, 0, f, 1]));
        for j in 0..3 {
            specs.push(year(i + 2 + j as u32, keys[2 + j], reps[j].0, reps[j].1, comp, 1, 1)); // rx11_7: the division's year awards
        }
    }
    covered!("rx11_7 french awards built");
    nation_awards_build(this, &specs, FRANCE_AWARDS_CPP, 0x74);
}

/// The French divisions' award blocks for `france_awards_init`: the club competition key; the team of the week, the
/// team of the year, the players' player, the player and the goalkeeper of the year; the three year awards'
/// (national, world) reputations.
pub(crate) const FRANCE_DIVISION_AWARDS: [(usize, [usize; 5], [(u32, u32); 3]); 3] = [
    (KEY_CLUB_COMP_FRENCH_FIRST_DIVISION, [KEY_STAFF_COMP_FRENCH_FIRST_DIVISION_TEAM_OF_THE_WEEK, KEY_STAFF_COMP_FRENCH_FIRST_DIVISION_TEAM_OF_THE_YEAR,
        KEY_STAFF_COMP_FRENCH_PLAYERS_FIRST_DIVISION_PLAYER_OF, KEY_STAFF_COMP_FRENCH_FIRST_DIVISION_PLAYER_OF_THE_YEAR,
        KEY_STAFF_COMP_FRENCH_FIRST_DIVISION_GOALKEEPER_OF_THE], [(6000, 2000), (6000, 2500), (5500, 1500)]),
    (KEY_CLUB_COMP_FRENCH_SECOND_DIVISION, [KEY_STAFF_COMP_FRENCH_SECOND_DIVISION_TEAM_OF_THE_WEEK, KEY_STAFF_COMP_FRENCH_SECOND_DIVISION_TEAM_OF_THE_YEAR,
        KEY_STAFF_COMP_FRENCH_PLAYERS_SECOND_DIVISION_PLAYER_OF, KEY_STAFF_COMP_FRENCH_SECOND_DIVISION_PLAYER_OF_THE_YEA,
        KEY_STAFF_COMP_FRENCH_SECOND_DIVISION_GOALKEEPER_OF_THE], [(4500, 1000), (4500, 1500), (4000, 500)]),
    (KEY_CLUB_COMP_FRENCH_NATIONAL, [KEY_STAFF_COMP_FRENCH_NATIONAL_TEAM_OF_THE_WEEK, KEY_STAFF_COMP_FRENCH_NATIONAL_TEAM_OF_THE_YEAR,
        KEY_STAFF_COMP_FRENCH_PLAYERS_NATIONAL_PLAYER_OF_THE_YE, KEY_STAFF_COMP_FRENCH_NATIONAL_PLAYER_OF_THE_YEAR,
        KEY_STAFF_COMP_FRENCH_NATIONAL_GOALKEEPER_OF_THE_YEAR], [(4500, 1000), (1500, 700), (1000, 500)]),
];

/// rule: on 20 December the French player, foreign player, most popular and most unpopular player of the year are
/// given from the award manager's year ratings; on 6 June the manager of the year and the under 23 player (from the
/// First Division) and, for each division, the players' player, the goalkeeper and the player of the year from the
/// stats and the team of the season; then, every day, the base daily update
///
/// 20 December (`awards_day_is`(today, 0x14, 0xb)): `award_player_of_year_from_ratings` (the award manager's year
/// ratings) for awards 1, 2, 4 and 5 with the filters [France, −1, −1, −1, −1, 0, −1], [−1, France, France, −1, −1, 0,
/// −1], [−1, France, −1, −1, −1, 3, −1] and [−1, France, −1, −1, −1, 4, −1] (France = [`KEY_NATION_FRANCE`]). 6 June
/// (`awards_day_is`(today, 6, 5)): `award_manager_of_year`(award 0, the First Division) and the under 23 player
/// (award 3, the First Division, age 0x17, statistic 0x11); per division (`award_player_of_year`) the players' player
/// (statistic 0x11), the goalkeeper (position flags 1, statistic 0x11) and the player of the year (flags 0x7e,
/// statistic 2), in that order, then `award_team_of_season` (the team of the year). Last
/// `nation_awards_base_daily_update`.
/// Original: `FranceAwards::daily_update` (0x5c1260, france_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn france_awards_daily_update(this: usize, today: usize) {
    if awards_day_is(today, 0x14, 0xb) { // rx11_7: 20 December
        covered!("rx11_7 french december awards");
        let fr = read_i32(KEY_NATION_FRANCE);
        for (k, filters) in [(1, [fr, -1, -1, -1, -1, 0, -1]), (2, [-1, fr, fr, -1, -1, 0, -1]), (4, [-1, fr, -1, -1, -1, 3, -1]), (5, [-1, fr, -1, -1, -1, 4, -1])] {
            award_player_of_year_from_ratings(this, today, nation_award(this, k), filters);
        }
    }
    if awards_day_is(today, 6, 5) {
        covered!("rx11_7 french june awards");
        award_manager_of_year(this, today, nation_award(this, 0), KEY_CLUB_COMP_FRENCH_FIRST_DIVISION);
        let under_23 = [0x17, 0, 0x11, -1, -1, -1, -1]; // rx11_7: the under 23 player from the First Division
        award_player_of_year(this, KEY_CLUB_COMP_FRENCH_FIRST_DIVISION, today, nation_award(this, 3), under_23);
        for (d, (comp_key, _, _)) in FRANCE_DIVISION_AWARDS.iter().enumerate() {
            let first = 6 + 5 * d;
            award_player_of_year(this, *comp_key, today, nation_award(this, first + 2), [-1, 0, 0x11, -1, -1, -1, -1]);
            award_player_of_year(this, *comp_key, today, nation_award(this, first + 4), [-1, 1, 0x11, -1, -1, -1, -1]);
            award_player_of_year(this, *comp_key, today, nation_award(this, first + 3), [-1, 0x7e, 2, -1, -1, -1, -1]);
            award_team_of_season(this, *comp_key, nation_award(this, first + 1));
        }
    }
    nation_awards_base_daily_update(this, today);
}

pub(crate) unsafe extern "thiscall" fn compare_france_awards_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, file);
    compare_nation_awards_built(r, this, file, &FRANCE_AWARDS, 0x15, "france_awards_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_france_awards_init(this: usize, file: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%);
    compare_nation_awards_init(this, || o(this, file), "france_awards_init");
}

/// Comparer for `france_awards_daily_update`: the awards cannot be given twice; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_france_awards_daily_update(this: usize, today: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I2%%)(this, today)
}
