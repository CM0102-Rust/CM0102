// ---------- rx12_12: the Croatian awards: the object, its nine awards and their day (award/croatia_awards.cpp) ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\award\croatia_awards.cpp" (the error box).
const CROATIA_AWARDS_CPP: usize = 0x9b5538;
/// `CroatiaAwards::init_awards(file)` (0x517fa0, thiscall, ret 4), through its hook.
const FN_CROATIA_AWARDS_INIT: usize = 0x517fa0;

/// The Croatian awards' class: the vtable 0x969380, Croatia (`KEY_NATION_CROATIA`), nation id 0x17, type 1, no month
/// ratings.
const CROATIA_AWARDS: NationAwardsClass = NationAwardsClass { vtable: 0x969380, nation_key: Some(KEY_NATION_CROATIA), nation_id: 0x17, kind: 1, month_ratings: None };

/// The three Croatian divisions in award order: the competition key and the first award of its block (the team of the
/// week, then the player of the year and the top goalscorer).
const CROATIA_DIVISIONS: [(usize, usize); 3] = [(KEY_CLUB_COMP_CROATIAN_FIRST_DIVISION_9CF8A4, 0), (KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_NORTH, 3),
    (KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_SOUTH, 6)];

/// Builds the Croatian awards (`nation_awards_construct` with `CROATIA_AWARDS`): the saved awards with a file, else
/// `CroatiaAwards::init_awards(file)`. Returns this.
/// Original: `CroatiaAwards::CroatiaAwards` (0x517f10, croatia_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn croatia_awards_construct(this: usize, file: usize) -> usize {
    nation_awards_construct(this, &CROATIA_AWARDS, file, core::mem::transmute(FN_CROATIA_AWARDS_INIT))
}

/// Builds the nine Croatian awards (`nation_awards_build`, the error box at line 0x68), kind 1, no scope: for each
/// division (`CROATIA_DIVISIONS`) the team of the week (team award, 0/0, span 1, mode 2), the player of the year and
/// the top goalscorer (option 1): the First Division's at national and world reputations 1500/800 and 1200/600, the
/// Second Division North's and South's at 700/400.
/// Original: `CroatiaAwards::init_awards` (0x517fa0, croatia_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn croatia_awards_init(this: usize, file: usize) {
    let n = read_u32(this + nation_awards_fields::NATION_ID);
    let f = file as u32;
    let k = |key: usize| read_u32(key);
    let (d1, d2, d3) = (k(KEY_CLUB_COMP_CROATIAN_FIRST_DIVISION_9CF8A4), k(KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_NORTH), k(KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_SOUTH));
    let year = |i: u32, key: usize, national: u32, world: u32, comp: u32, option: u32| AwardSpec::Year([n, i, k(key), national, world, comp, 1, 0, option, f, 1]);
    let team = |i: u32, key: usize, comp: u32| AwardSpec::Team([n, i, k(key), 0, 0, comp, 1, 2, f, 1]);
    let specs = [
        team(0, KEY_STAFF_COMP_CROATIAN_FIRST_DIVISION_TEAM_OF_THE_WEEK, d1),
        year(1, KEY_STAFF_COMP_CROATIAN_FIRST_DIVISION_PLAYER_OF_THE_YE, 1500, 800, d1, 0), // rx12_12: the player of the year
        year(2, KEY_STAFF_COMP_CROATIAN_FIRST_DIVISION_TOP_GOALSCORER, 1200, 600, d1, 1),
        team(3, KEY_STAFF_COMP_CROATIAN_SECOND_DIVISION_A_TEAM_OF_THE_W, d2),
        year(4, KEY_STAFF_COMP_CROATIAN_SECOND_DIVISION_A_PLAYER_OF_THE, 700, 400, d2, 0),
        year(5, KEY_STAFF_COMP_CROATIAN_SECOND_DIVISION_A_TOP_GOALSCORE, 700, 400, d2, 1),
        team(6, KEY_STAFF_COMP_CROATIAN_SECOND_DIVISION_B_TEAM_OF_THE_W, d3),
        year(7, KEY_STAFF_COMP_CROATIAN_SECOND_DIVISION_B_PLAYER_OF_THE, 700, 400, d3, 0),
        year(8, KEY_STAFF_COMP_CROATIAN_SECOND_DIVISION_B_TOP_GOALSCORE, 700, 400, d3, 1),
    ];
    covered!("rx12_12 croatian awards built");
    nation_awards_build(this, &specs, CROATIA_AWARDS_CPP, 0x68);
}

/// The Croatian awards' day: on 30 May (`awards_day_is`, month index 4), for each division (`CROATIA_DIVISIONS`) from
/// its season statistics (`award_player_of_year`), the player of the year (statistic 0x11) and the top goalscorer
/// (statistic 2); then, every day, the base daily update.
/// Original: `CroatiaAwards::daily_update` (0x5183a0, croatia_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn croatia_awards_daily_update(this: usize, today: usize) {
    if awards_day_is(today, 0x1e, 4) { // rx12_12: 30 May
        covered!("rx12_12 croatian awards day");
        for (comp, first) in CROATIA_DIVISIONS {
            award_player_of_year(this, comp, today, nation_award(this, first + 1), [-1, 0, 0x11, -1, -1, -1, -1]); // rx12_12: the player of the year
            award_player_of_year(this, comp, today, nation_award(this, first + 2), [-1, 0, 2, -1, -1, -1, -1]);
        }
    }
    nation_awards_base_daily_update(this, today);
}

pub(crate) unsafe extern "thiscall" fn compare_croatia_awards_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, file);
    compare_nation_awards_built(r, this, file, &CROATIA_AWARDS, 9, "croatia_awards_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_croatia_awards_init(this: usize, file: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%);
    compare_nation_awards_init(this, || o(this, file), "croatia_awards_init");
}

/// Comparer for `croatia_awards_daily_update`: the awards cannot be given twice; only the original runs (the copy is
/// proven by the career).
pub(crate) unsafe extern "thiscall" fn compare_croatia_awards_daily_update(this: usize, today: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I2%%)(this, today)
}
