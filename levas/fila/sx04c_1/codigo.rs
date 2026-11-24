// ---------- sx04c_1: the Italian awards: the object, its 25 or 40 awards and the season's end (award/italy_awards.cpp); the helpers the sx04c nations share ----------

/// The award manager's second year ratings table (+0x10 of `G_AWARD_MANAGER`, the `YEAR_RATINGS` the manager builds
/// with the day of 20 June 2001; the international awards call it the world ratings) [inference]; the first one (+0xc)
/// is `AWARD_MANAGER_YEAR_RATINGS`.
pub(crate) const AWARD_MANAGER_WORLD_RATINGS: usize = 0x10;

/// The player of the year of `award` from the award manager's world ratings (`AWARD_MANAGER_WORLD_RATINGS`,
/// `NationAwards::award_player_of_year_from_ratings`) with the seven filters as the nation passes them.
pub(crate) unsafe fn award_player_of_year_from_world_ratings(this: usize, today: usize, award: usize, filters: [i32; 7]) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, usize, i32, i32, i32, i32, i32, i32, i32) = core::mem::transmute(FN_NATION_AWARDS_PLAYER_OF_YEAR_FROM_RATINGS);
    let [a, b, c, d, e, g, h] = filters;
    f(this, today, award, read_usize(read_usize(G_AWARD_MANAGER) + AWARD_MANAGER_WORLD_RATINGS), a, b, c, d, e, g, h)
}

/// The file an award added in a later version of the save is loaded from: `file` when the save's data version
/// (`G_GAME_DATA_VERSION`, absolute value as the original takes it: `cdq; xor; sub`) reaches `version`, else none (the
/// award is created fresh).
pub(crate) unsafe fn awards_file_from_version(file: usize, version: i32) -> usize {
    if read_i32(G_GAME_DATA_VERSION).wrapping_abs() >= version {
        file
    } else {
        0
    }
}

/// The player of the month of a league from the nation's month ratings, as Russia and Scotland give it inline: the best
/// of the ratings in `comp_key`'s competition with the age limit (−1 none, `exe::month_ratings_find_best_player`) takes
/// `award` with his club's id (0 without a club) and the month and year the ratings cover
/// (`month_ratings_fields::MONTH`, `YEAR`); nobody, nothing.
pub(crate) unsafe fn award_month_best_player(this: usize, award: usize, comp_key: usize, age: i32) {
    let best = exe::month_ratings_find_best_player(read_usize(this + nation_awards_fields::MONTH_RATINGS), read_i32(comp_key), age as u32);
    if best == 0 {
        return;
    }
    let id = read_i32(best);
    let club = staff_at(id as isize).club_job();
    let club_id = if club.is_null() { 0 } else { read_i32(club.addr()) };
    let ratings = read_usize(this + nation_awards_fields::MONTH_RATINGS);
    // the high bytes of the month and year arguments are the registers' (set_winner keeps the low byte and word)
    exe::month_award_set_winner(award, id, club_id, read_u8(ratings + month_ratings_fields::MONTH) as u32, read_u16(ratings + month_ratings_fields::YEAR) as u32);
}

/// "E:\dev\CM3\cm3 00-01\cm3\code\award\italy_awards.cpp" (the error box).
const ITALY_AWARDS_CPP: usize = 0x9ea96c;
/// `ItalyAwards::init_awards(file)` (0x65fec0, thiscall, ret 4), through its hook.
const FN_ITALY_AWARDS_INIT: usize = 0x65fec0;
/// The data version from which the Italian top goalscorer awards are in the save.
const ITALY_TOP_GOALSCORERS_SAVED: i32 = 0x3e007;

/// The Italian awards' class: the vtable 0x96c958, Italy (`KEY_NATION_ITALY`), nation id 0xd, type 1, no month ratings.
const ITALY_AWARDS: NationAwardsClass = NationAwardsClass { vtable: 0x96c958, nation_key: Some(KEY_NATION_ITALY), nation_id: 0xd, kind: 1, month_ratings: None };

/// Italy's divisions in award order: the competition key and the first slot of the division's block (Serie A: team
/// of the week, team of the year, manager, player, young player, top goalscorer, defender, goalkeeper, Italian player,
/// foreign player of the year; the others: team of the week, team of the year, manager, player, top goalscorer).
const ITALY_DIVISIONS: [(usize, usize); 7] = [(KEY_CLUB_COMP_ITALIAN_SERIE_A, 0), (KEY_CLUB_COMP_ITALIAN_SERIE_B, 10),
    (KEY_CLUB_COMP_ITALIAN_SERIE_C1_A, 15), (KEY_CLUB_COMP_ITALIAN_SERIE_C1_B, 20), (KEY_CLUB_COMP_ITALIAN_SERIE_C2_A, 25),
    (KEY_CLUB_COMP_ITALIAN_SERIE_C2_B, 30), (KEY_CLUB_COMP_ITALIAN_SERIE_C2_C, 35)];

/// Builds the Italian awards (`nation_awards_construct` with `ITALY_AWARDS`): the saved awards with a file, else
/// `ItalyAwards::init_awards(file)`. Returns this.
/// Original: `ItalyAwards::ItalyAwards` (0x65fe30, italy_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn italy_awards_construct(this: usize, file: usize) -> usize {
    nation_awards_construct(this, &ITALY_AWARDS, file, core::mem::transmute(FN_ITALY_AWARDS_INIT))
}

/// Builds the Italian awards (`nation_awards_build`, the error box at line 0x9b): 40 with the lower divisions
/// (`awards_lower_divisions`: the three Serie C2 groups), else 25. Serie A: the team of the week (team award,
/// reputations 0/0), the team of the year (6500/2500), the manager (7000/2500, not for players), the player
/// (8500/5000), the young player (7500/4000), the top goalscorer (8000/5000, option 1), the defender, the goalkeeper,
/// the Italian player and the foreign player of the year (7500/5000 each). Serie B, C1 A and C1 B: the team of the
/// week (0/0), the team of the year (3500/1750, 1500/750, 1500/750), the manager (5000/1500, 3000/1000, 3000/1000), the
/// player (6000/2000, 4500/1500, 4500/1500) and the top goalscorer (5500/2000, 2000/1000, 2000/1000). Each C2 group: the
/// team of the week (0/0), the team of the year (500/250), the manager (3000/1000), the player (4500/1500) and the top
/// goalscorer (1000/500). All of their division, kind 1, no scope; team awards span 1, mode 0. The top goalscorers are
/// loaded from the file only when the save's data version reaches 0x3e007 (`awards_file_from_version`), else created.
/// Original: `ItalyAwards::init_awards` (0x65fec0, italy_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn italy_awards_init(this: usize, file: usize) {
    let n = read_u32(this + nation_awards_fields::NATION_ID);
    let f = file as u32;
    let fg = awards_file_from_version(file, ITALY_TOP_GOALSCORERS_SAVED) as u32;
    let k = |key: usize| read_u32(key);
    let team = |i: u32, key: usize, national: u32, world: u32, comp: usize| AwardSpec::Team([n, i, k(key), national, world, k(comp), 1, 0, f, 1]);
    let year = |i: u32, key: usize, national: u32, world: u32, comp: usize, players: u32| AwardSpec::Year([n, i, k(key), national, world, k(comp), 1, 0, 0, f, players]);
    let scorer = |i: u32, key: usize, national: u32, world: u32, comp: usize| AwardSpec::Year([n, i, k(key), national, world, k(comp), 1, 0, 1, fg, 1]);
    let (a, b) = (KEY_CLUB_COMP_ITALIAN_SERIE_A, KEY_CLUB_COMP_ITALIAN_SERIE_B);
    let (c1a, c1b) = (KEY_CLUB_COMP_ITALIAN_SERIE_C1_A, KEY_CLUB_COMP_ITALIAN_SERIE_C1_B);
    let (c2a, c2b, c2c) = (KEY_CLUB_COMP_ITALIAN_SERIE_C2_A, KEY_CLUB_COMP_ITALIAN_SERIE_C2_B, KEY_CLUB_COMP_ITALIAN_SERIE_C2_C);
    let specs = [
        team(0, KEY_STAFF_COMP_ITALIAN_SERIE_A_TEAM_OF_THE_WEEK, 0, 0, a),
        team(1, KEY_STAFF_COMP_ITALIAN_SERIE_A_TEAM_OF_THE_YEAR, 6500, 2500, a),
        year(2, KEY_STAFF_COMP_ITALIAN_SERIE_A_MANAGER_OF_THE_YEAR, 7000, 2500, a, 0),
        year(3, KEY_STAFF_COMP_ITALIAN_SERIE_A_PLAYER_OF_THE_YEAR, 8500, 5000, a, 1),
        year(4, KEY_STAFF_COMP_ITALIAN_SERIE_A_YOUNG_PLAYER_OF_THE_YEAR, 7500, 4000, a, 1),
        scorer(5, KEY_STAFF_COMP_ITALIAN_SERIE_A_TOP_GOALSCORER, 8000, 5000, a),
        year(6, KEY_STAFF_COMP_ITALIAN_SERIE_A_DEFENDER_OF_THE_YEAR, 7500, 5000, a, 1),
        year(7, KEY_STAFF_COMP_ITALIAN_SERIE_A_GOALKEEPER_OF_THE_YEAR, 7500, 5000, a, 1),
        year(8, KEY_STAFF_COMP_ITALIAN_SERIE_A_ITALIAN_PLAYER_OF_THE_YE, 7500, 5000, a, 1),
        year(9, KEY_STAFF_COMP_ITALIAN_SERIE_A_FOREIGN_PLAYER_OF_THE_YE, 7500, 5000, a, 1),
        team(10, KEY_STAFF_COMP_ITALIAN_SERIE_B_TEAM_OF_THE_WEEK, 0, 0, b),
        team(11, KEY_STAFF_COMP_ITALIAN_SERIE_B_TEAM_OF_THE_YEAR, 3500, 1750, b),
        year(12, KEY_STAFF_COMP_ITALIAN_SERIE_B_MANAGER_OF_THE_YEAR, 5000, 1500, b, 0),
        year(13, KEY_STAFF_COMP_ITALIAN_SERIE_B_PLAYER_OF_THE_YEAR, 6000, 2000, b, 1),
        scorer(14, KEY_STAFF_COMP_ITALIAN_SERIE_B_TOP_GOALSCORER, 5500, 2000, b),
        team(15, KEY_STAFF_COMP_ITALIAN_SERIE_C1_A_TEAM_OF_THE_WEEK, 0, 0, c1a),
        team(16, KEY_STAFF_COMP_ITALIAN_SERIE_C1_A_TEAM_OF_THE_YEAR, 1500, 750, c1a),
        year(17, KEY_STAFF_COMP_ITALIAN_SERIE_C1_A_MANAGER_OF_THE_YEAR, 3000, 1000, c1a, 0),
        year(18, KEY_STAFF_COMP_ITALIAN_SERIE_C1_A_PLAYER_OF_THE_YEAR, 4500, 1500, c1a, 1),
        scorer(19, KEY_STAFF_COMP_ITALIAN_SERIE_C1_A_TOP_GOALSCORER, 2000, 1000, c1a),
        team(20, KEY_STAFF_COMP_ITALIAN_SERIE_C1_B_TEAM_OF_THE_WEEK, 0, 0, c1b),
        team(21, KEY_STAFF_COMP_ITALIAN_SERIE_C1_B_TEAM_OF_THE_YEAR, 1500, 750, c1b),
        year(22, KEY_STAFF_COMP_ITALIAN_SERIE_C1_B_MANAGER_OF_THE_YEAR, 3000, 1000, c1b, 0),
        year(23, KEY_STAFF_COMP_ITALIAN_SERIE_C1_B_PLAYER_OF_THE_YEAR, 4500, 1500, c1b, 1),
        scorer(24, KEY_STAFF_COMP_ITALIAN_SERIE_C1_B_TOP_GOALSCORER, 2000, 1000, c1b),
        team(25, KEY_STAFF_COMP_ITALIAN_SERIE_C2_A_TEAM_OF_THE_WEEK, 0, 0, c2a),
        team(26, KEY_STAFF_COMP_ITALIAN_SERIE_C2_A_TEAM_OF_THE_YEAR, 500, 250, c2a),
        year(27, KEY_STAFF_COMP_ITALIAN_SERIE_C2_A_MANAGER_OF_THE_YEAR, 3000, 1000, c2a, 0),
        year(28, KEY_STAFF_COMP_ITALIAN_SERIE_C2_A_PLAYER_OF_THE_YEAR, 4500, 1500, c2a, 1),
        scorer(29, KEY_STAFF_COMP_ITALIAN_SERIE_C2_A_TOP_GOALSCORER, 1000, 500, c2a),
        team(30, KEY_STAFF_COMP_ITALIAN_SERIE_C2_B_TEAM_OF_THE_WEEK, 0, 0, c2b),
        team(31, KEY_STAFF_COMP_ITALIAN_SERIE_C2_B_TEAM_OF_THE_YEAR, 500, 250, c2b),
        year(32, KEY_STAFF_COMP_ITALIAN_SERIE_C2_B_MANAGER_OF_THE_YEAR, 3000, 1000, c2b, 0),
        year(33, KEY_STAFF_COMP_ITALIAN_SERIE_C2_B_PLAYER_OF_THE_YEAR, 4500, 1500, c2b, 1),
        scorer(34, KEY_STAFF_COMP_ITALIAN_SERIE_C2_B_TOP_GOALSCORER, 1000, 500, c2b),
        team(35, KEY_STAFF_COMP_ITALIAN_SERIE_C2_C_TEAM_OF_THE_WEEK, 0, 0, c2c),
        team(36, KEY_STAFF_COMP_ITALIAN_SERIE_C2_C_TEAM_OF_THE_YEAR, 500, 250, c2c),
        year(37, KEY_STAFF_COMP_ITALIAN_SERIE_C2_C_MANAGER_OF_THE_YEAR, 3000, 1000, c2c, 0),
        year(38, KEY_STAFF_COMP_ITALIAN_SERIE_C2_C_PLAYER_OF_THE_YEAR, 4500, 1500, c2c, 1),
        scorer(39, KEY_STAFF_COMP_ITALIAN_SERIE_C2_C_TOP_GOALSCORER, 1000, 500, c2c),
    ];
    let count = if awards_lower_divisions(this) { 40 } else { 25 };
    covered!("sx04c_1 italian awards built");
    nation_awards_build(this, &specs[..count], ITALY_AWARDS_CPP, 0x9b);
}

/// The Italian awards' day: on 6 June (`awards_day_is`), the manager of the year of each division (the C2 groups only
/// with the lower divisions); then, from Serie A's stats (`award_player_of_year`): the player (statistic 0x11), the
/// young player (up to 23), the defender (positions 6), the goalkeeper (positions 1), the top goalscorer (statistic
/// 2), the Italian player (statistic 2, Italian nationality) and the foreign player (statistic 2, Italy excluded) of
/// the year; the player (0x11) and the top goalscorer (2) of every other division (the C2 groups with the lower
/// divisions); and the team of the season of each division (the same). Every day, the base daily update.
/// Original: `ItalyAwards::daily_update` (0x660ec0, italy_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn italy_awards_daily_update(this: usize, today: usize) {
    if awards_day_is(today, 6, 5) {
        covered!("sx04c_1 italian awards day");
        let divisions = if awards_lower_divisions(this) { 7 } else { 4 };
        for &(key, first) in &ITALY_DIVISIONS[..divisions] {
            award_manager_of_year(this, today, nation_award(this, first + 2), key);
        }
        let a = KEY_CLUB_COMP_ITALIAN_SERIE_A;
        let italy = read_i32(KEY_NATION_ITALY);
        for (k, filters) in [(3, [-1, 0, 0x11, -1, -1, -1, -1]), (4, [0x17, 0, 0x11, -1, -1, -1, -1]), (6, [-1, 6, 0x11, -1, -1, -1, -1]),
            (7, [-1, 1, 0x11, -1, -1, -1, -1]), (5, [-1, 0, 2, -1, -1, -1, -1]), (8, [-1, 0, 2, italy, -1, -1, -1]),
            (9, [-1, 0, 2, -1, -1, italy, -1])] {
            award_player_of_year(this, a, today, nation_award(this, k), filters);
        }
        for &(key, first) in &ITALY_DIVISIONS[1..4] {
            award_player_of_year(this, key, today, nation_award(this, first + 3), [-1, 0, 0x11, -1, -1, -1, -1]);
            award_player_of_year(this, key, today, nation_award(this, first + 4), [-1, 0, 2, -1, -1, -1, -1]);
        }
        if awards_lower_divisions(this) {
            for &(key, first) in &ITALY_DIVISIONS[4..] {
                award_player_of_year(this, key, today, nation_award(this, first + 3), [-1, 0, 0x11, -1, -1, -1, -1]);
                award_player_of_year(this, key, today, nation_award(this, first + 4), [-1, 0, 2, -1, -1, -1, -1]);
            }
        }
        let divisions = if awards_lower_divisions(this) { 7 } else { 4 };
        for &(key, first) in &ITALY_DIVISIONS[..divisions] {
            award_team_of_season(this, key, nation_award(this, first + 1));
        }
    }
    nation_awards_base_daily_update(this, today);
}

pub(crate) unsafe extern "thiscall" fn compare_italy_awards_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, file);
    let count = if awards_lower_divisions(this) { 40 } else { 25 };
    compare_nation_awards_built(r, this, file, &ITALY_AWARDS, count, "italy_awards_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_italy_awards_init(this: usize, file: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%);
    compare_nation_awards_init(this, || o(this, file), "italy_awards_init");
}

/// Comparer for `italy_awards_daily_update`: the awards cannot be given twice; only the original runs (the copy is
/// proven by the career).
pub(crate) unsafe extern "thiscall" fn compare_italy_awards_daily_update(this: usize, today: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I2%%)(this, today)
}
