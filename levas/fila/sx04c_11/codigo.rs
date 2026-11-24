// ---------- sx04c_11: the Portuguese awards: the object, its twelve awards and the season's end (award/portugal_awards.cpp) ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\award\portugal_awards.cpp" (the error box).
const PORTUGAL_AWARDS_CPP: usize = 0xa7880c;
/// `PortugalAwards::init_awards(file)` (0x7d36c0, thiscall, ret 4), through its hook.
const FN_PORTUGAL_AWARDS_INIT: usize = 0x7d36c0;

/// The Portuguese awards' class: the vtable 0x96e9ac, Portugal (`KEY_NATION_PORTUGAL`), nation id 0x10, type 1, no
/// month ratings.
const PORTUGAL_AWARDS: NationAwardsClass = NationAwardsClass { vtable: 0x96e9ac, nation_key: Some(KEY_NATION_PORTUGAL), nation_id: 0x10, kind: 1, month_ratings: None };

/// Portugal's two divisions: the competition key and the first slot of the division's block (team of the week,
/// player, most man of the match awards, top goalscorer, team of the year, manager of the year).
const PORTUGAL_DIVISIONS: [(usize, usize); 2] = [(KEY_CLUB_COMP_PORTUGUESE_PREMIER_DIVISION, 0), (KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION, 6)];

/// Builds the Portuguese awards (`nation_awards_construct` with `PORTUGAL_AWARDS`): the saved awards with a file, else
/// `PortugalAwards::init_awards(file)`. Returns this.
/// Original: `PortugalAwards::PortugalAwards` (0x7d3630, portugal_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn portugal_awards_construct(this: usize, file: usize) -> usize {
    nation_awards_construct(this, &PORTUGAL_AWARDS, file, core::mem::transmute(FN_PORTUGAL_AWARDS_INIT))
}

/// Builds the twelve Portuguese awards (`nation_awards_build`, the error box at line 0x68), kind 1, no scope; for each
/// division the team of the week (team award, national and world reputations 0/0, span 1), the player (Premier
/// Division 5000/1750, Second 2500/500), the most man of the match awards (5000/1500, 2500/500), the top goalscorer
/// (5000/2000, 2500/750, option 1), the team of the year (4500/1500, 2200/750) and the manager of the year (6000/3000,
/// 3000/1500, not for players).
/// Original: `PortugalAwards::init_awards` (0x7d36c0, portugal_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn portugal_awards_init(this: usize, file: usize) {
    let n = read_u32(this + nation_awards_fields::NATION_ID);
    let f = file as u32;
    let k = |key: usize| read_u32(key);
    let (d1, d2) = (k(KEY_CLUB_COMP_PORTUGUESE_PREMIER_DIVISION), k(KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION));
    let year = |i: u32, key: usize, national: u32, world: u32, comp: u32, option: u32, players: u32| AwardSpec::Year([n, i, k(key), national, world, comp, 1, 0, option, f, players]);
    let team = |i: u32, key: usize, national: u32, world: u32, comp: u32| AwardSpec::Team([n, i, k(key), national, world, comp, 1, 0, f, 1]);
    let specs = [
        team(0, KEY_STAFF_COMP_PORTUGUESE_PREMIER_DIVISION_TEAM_OF_THE_9CFCA4, 0, 0, d1),
        year(1, KEY_STAFF_COMP_PORTUGUESE_PREMIER_DIVISION_PLAYER_OF_TH, 5000, 1750, d1, 0, 1),
        year(2, KEY_STAFF_COMP_PORTUGUESE_PREMIER_DIVISION_MOST_MAN_OF, 5000, 1500, d1, 0, 1),
        year(3, KEY_STAFF_COMP_PORTUGUESE_PREMIER_DIVISION_TOP_GOALSCOR, 5000, 2000, d1, 1, 1),
        team(4, KEY_STAFF_COMP_PORTUGUESE_PREMIER_DIVISION_TEAM_OF_THE_9CFC9C, 4500, 1500, d1),
        year(5, KEY_STAFF_COMP_PORTUGUESE_PREMIER_LEAGUE_MANAGER_OF_THE, 6000, 3000, d1, 0, 0),
        team(6, KEY_STAFF_COMP_PORTUGUESE_SECOND_DIVISION_TEAM_OF_THE_W, 0, 0, d2),
        year(7, KEY_STAFF_COMP_PORTUGUESE_SECOND_DIVISION_PLAYER_OF_THE, 2500, 500, d2, 0, 1),
        year(8, KEY_STAFF_COMP_PORTUGUESE_SECOND_DIVISION_MOST_MAN_OF_T, 2500, 500, d2, 0, 1),
        year(9, KEY_STAFF_COMP_PORTUGUESE_SECOND_DIVISION_TOP_GOALSCORE, 2500, 750, d2, 1, 1),
        team(10, KEY_STAFF_COMP_PORTUGUESE_SECOND_DIVISION_TEAM_OF_THE_Y, 2200, 750, d2),
        year(11, KEY_STAFF_COMP_PORTUGUESE_SECOND_LEAGUE_MANAGER_OF_THE, 3000, 1500, d2, 0, 0),
    ];
    covered!("sx04c_11 portuguese awards built");
    nation_awards_build(this, &specs, PORTUGAL_AWARDS_CPP, 0x68);
}

/// The Portuguese awards' day: on 6 June (`awards_day_is`), for each division, from its stats
/// (`award_player_of_year`), the most man of the match awards (statistic 0xc), the top goalscorer (2) and the player
/// (0x11) of the year, its team of the season and its manager of the year; then, every day, the base daily update.
/// Original: `PortugalAwards::daily_update` (0x7d3be0, portugal_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn portugal_awards_daily_update(this: usize, today: usize) {
    if awards_day_is(today, 6, 5) {
        covered!("sx04c_11 portuguese awards day");
        for &(key, first) in &PORTUGAL_DIVISIONS {
            for (k, stat) in [(2, 0xc), (3, 2), (1, 0x11)] {
                award_player_of_year(this, key, today, nation_award(this, first + k), [-1, 0, stat, -1, -1, -1, -1]);
            }
            award_team_of_season(this, key, nation_award(this, first + 4));
            award_manager_of_year(this, today, nation_award(this, first + 5), key);
        }
    }
    nation_awards_base_daily_update(this, today);
}

pub(crate) unsafe extern "thiscall" fn compare_portugal_awards_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, file);
    compare_nation_awards_built(r, this, file, &PORTUGAL_AWARDS, 12, "portugal_awards_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_portugal_awards_init(this: usize, file: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%);
    compare_nation_awards_init(this, || o(this, file), "portugal_awards_init");
}

/// Comparer for `portugal_awards_daily_update`: the awards cannot be given twice; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_portugal_awards_daily_update(this: usize, today: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I2%%)(this, today)
}
