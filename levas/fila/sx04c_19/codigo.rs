// ---------- sx04c_19: the old French awards: the object, its fourteen awards and the season's end (award/old_france_awards.cpp) ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\award\old_france_awards.cpp" (the error box).
const OLD_FRANCE_AWARDS_CPP: usize = 0xa6a2a4;
/// `OldFranceAwards::init_awards(file)` (0x79dd80, thiscall, ret 4), through its hook.
const FN_OLD_FRANCE_AWARDS_INIT: usize = 0x79dd80;

/// The old French awards' class: the vtable 0x96e194, France (`KEY_NATION_FRANCE`), nation id 9, type 1, no month
/// ratings.
const OLD_FRANCE_AWARDS: NationAwardsClass = NationAwardsClass { vtable: 0x96e194, nation_key: Some(KEY_NATION_FRANCE), nation_id: 9, kind: 1, month_ratings: None };

/// Builds the old French awards (`nation_awards_construct` with `OLD_FRANCE_AWARDS`): the saved awards with a file, else
/// `OldFranceAwards::init_awards(file)`. Returns this. The award manager builds this class instead of the current
/// French one only with a database whose newer award keys are missing (−1).
/// Original: `OldFranceAwards::OldFranceAwards` (0x79dcf0, old_france_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn old_france_awards_construct(this: usize, file: usize) -> usize {
    nation_awards_construct(this, &OLD_FRANCE_AWARDS, file, core::mem::transmute(FN_OLD_FRANCE_AWARDS_INIT))
}

/// Builds the fourteen old French awards (`nation_awards_build`, the error box at line 0x6b), no scope: with no
/// competition, the French manager (national and world reputations 6000/2500, kind 1, not for players), player
/// (7000/4500), foreign player (6500/3500), under-23 player (5000/2000, kind 1), most popular (5000/1500) and most
/// unpopular player of the year (0/0), the others kind 0; then, kind 1, for each division the team of the week (team
/// award, 0/0, span 1), the players' player of the year (First Division 6000/2000, Second 4500/1000), the player of the
/// year (6000/2500, 4500/1500) and the goalkeeper of the year (5500/1500, 4000/500).
/// Original: `OldFranceAwards::init_awards` (0x79dd80, old_france_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn old_france_awards_init(this: usize, file: usize) {
    let n = read_u32(this + nation_awards_fields::NATION_ID);
    let f = file as u32;
    let k = |key: usize| read_u32(key);
    let (d1, d2) = (k(KEY_CLUB_COMP_FRENCH_FIRST_DIVISION), k(KEY_CLUB_COMP_FRENCH_SECOND_DIVISION));
    let year = |i: u32, key: usize, national: u32, world: u32, comp: u32, kind: u32, players: u32| AwardSpec::Year([n, i, k(key), national, world, comp, kind, 0, 0, f, players]);
    let team = |i: u32, key: usize, comp: u32| AwardSpec::Team([n, i, k(key), 0, 0, comp, 1, 0, f, 1]);
    let none = u32::MAX;
    let specs = [
        year(0, KEY_STAFF_COMP_FRENCH_MANAGER_OF_THE_YEAR, 6000, 2500, none, 1, 0),
        year(1, KEY_STAFF_COMP_FRENCH_PLAYER_OF_THE_YEAR, 7000, 4500, none, 0, 1),
        year(2, KEY_STAFF_COMP_FRENCH_FOREIGN_PLAYER_OF_THE_YEAR, 6500, 3500, none, 0, 1),
        year(3, KEY_STAFF_COMP_FRENCH_UNDER_23_PLAYER_OF_THE_YEAR, 5000, 2000, none, 1, 1),
        year(4, KEY_STAFF_COMP_FRENCH_MOST_POPULAR_PLAYER_OF_THE_YEAR, 5000, 1500, none, 0, 1),
        year(5, KEY_STAFF_COMP_FRENCH_MOST_UNPOPULAR_PLAYER_OF_THE_YEAR, 0, 0, none, 0, 1),
        team(6, KEY_STAFF_COMP_FRENCH_FIRST_DIVISION_TEAM_OF_THE_WEEK, d1),
        year(7, KEY_STAFF_COMP_FRENCH_PLAYERS_FIRST_DIVISION_PLAYER_OF, 6000, 2000, d1, 1, 1),
        year(8, KEY_STAFF_COMP_FRENCH_FIRST_DIVISION_PLAYER_OF_THE_YEAR, 6000, 2500, d1, 1, 1),
        year(9, KEY_STAFF_COMP_FRENCH_FIRST_DIVISION_GOALKEEPER_OF_THE, 5500, 1500, d1, 1, 1),
        team(10, KEY_STAFF_COMP_FRENCH_SECOND_DIVISION_TEAM_OF_THE_WEEK, d2),
        year(11, KEY_STAFF_COMP_FRENCH_PLAYERS_SECOND_DIVISION_PLAYER_OF, 4500, 1000, d2, 1, 1),
        year(12, KEY_STAFF_COMP_FRENCH_SECOND_DIVISION_PLAYER_OF_THE_YEA, 4500, 1500, d2, 1, 1),
        year(13, KEY_STAFF_COMP_FRENCH_SECOND_DIVISION_GOALKEEPER_OF_THE, 4000, 500, d2, 1, 1),
    ];
    covered!("sx04c_19 old french awards built");
    nation_awards_build(this, &specs, OLD_FRANCE_AWARDS_CPP, 0x6b);
}

/// The old French awards' days: on 20 December (`awards_day_is`), the French player, foreign player, most popular and
/// most unpopular player of the year from the year ratings (`award_player_of_year_from_ratings`, with the filters as
/// the original passes them: French players; nationality other than French; modes 3 and 4). On 6 June, the manager of
/// the year (from the First Division); the under-23 player (up to 23), the players' player (statistic 0x11), the
/// goalkeeper (positions 1) and the player of the year (positions 0x7e, statistic 2) of the First Division from its
/// stats; and the Second Division's players' player, goalkeeper and player of the year. Every day, the base daily
/// update.
/// Original: `OldFranceAwards::daily_update` (0x79e330, old_france_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn old_france_awards_daily_update(this: usize, today: usize) {
    if awards_day_is(today, 0x14, 0xb) {
        covered!("sx04c_19 old french december awards");
        let fra = read_i32(KEY_NATION_FRANCE);
        for (k, filters) in [(1, [fra, -1, -1, -1, -1, 0, -1]), (2, [-1, fra, fra, -1, -1, 0, -1]), (4, [-1, fra, -1, -1, -1, 3, -1]),
            (5, [-1, fra, -1, -1, -1, 4, -1])] {
            award_player_of_year_from_ratings(this, today, nation_award(this, k), filters);
        }
    }
    if awards_day_is(today, 6, 5) {
        covered!("sx04c_19 old french june awards");
        let (d1, d2) = (KEY_CLUB_COMP_FRENCH_FIRST_DIVISION, KEY_CLUB_COMP_FRENCH_SECOND_DIVISION);
        award_manager_of_year(this, today, nation_award(this, 0), d1);
        for (key, k, age, positions, stat) in [(d1, 3, 0x17, 0, 0x11), (d1, 7, -1, 0, 0x11), (d1, 9, -1, 1, 0x11), (d1, 8, -1, 0x7e, 2),
            (d2, 11, -1, 0, 0x11), (d2, 13, -1, 1, 0x11), (d2, 12, -1, 0x7e, 2)] {
            award_player_of_year(this, key, today, nation_award(this, k), [age, positions, stat, -1, -1, -1, -1]);
        }
    }
    nation_awards_base_daily_update(this, today);
}

pub(crate) unsafe extern "thiscall" fn compare_old_france_awards_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, file);
    compare_nation_awards_built(r, this, file, &OLD_FRANCE_AWARDS, 14, "old_france_awards_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_old_france_awards_init(this: usize, file: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%);
    compare_nation_awards_init(this, || o(this, file), "old_france_awards_init");
}

/// Comparer for `old_france_awards_daily_update`: the awards cannot be given twice; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_old_france_awards_daily_update(this: usize, today: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I2%%)(this, today)
}
