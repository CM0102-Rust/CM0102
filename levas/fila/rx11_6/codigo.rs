// ---------- rx11_6: the English awards: the object and its 26 or 32 awards (award/england_awards.cpp) ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\award\england_awards.cpp" (the error boxes).
const ENGLAND_AWARDS_CPP: usize = 0x9c4460;
/// `EnglandAwards::init_awards(file)` (0x5788c0, thiscall, ret 4), through its hook.
const FN_ENGLAND_AWARDS_INIT: usize = 0x5788c0;

/// The English awards' class: the vtable 0x969f38, England (`KEY_NATION_ENGLAND`), nation id 7, type 1, with month
/// ratings (the error box at line 0x6a when they cannot be built).
const ENGLAND_AWARDS: NationAwardsClass = NationAwardsClass { vtable: 0x969f38, nation_key: Some(KEY_NATION_ENGLAND), nation_id: 7, kind: 1,
    month_ratings: Some((ENGLAND_AWARDS_CPP, 0x6a)) };

/// rule: the English awards object is built for England with month ratings; from a saved game it loads them, otherwise
/// it creates its awards
///
/// `nation_awards_construct` with `ENGLAND_AWARDS`: the saved awards with a file, else the month ratings and
/// `EnglandAwards::init_awards(file)`. Returns this.
/// Original: `EnglandAwards::EnglandAwards` (0x578780, england_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn england_awards_construct(this: usize, file: usize) -> usize {
    nation_awards_construct(this, &ENGLAND_AWARDS, file, core::mem::transmute(FN_ENGLAND_AWARDS_INIT))
}

/// One English division's block of awards (team of the week, manager, player and young player of the month, manager
/// of the year, the division's team of the year; six awards from `first`): the staff competition keys in that order,
/// the club competition `comp` and the reputations (national, world) of each.
pub(crate) struct EnglandDivisionAwards {
    pub(crate) keys: [usize; 6],
    pub(crate) reputations: [(u32, u32); 6],
}

/// rule: England has 26 awards (32 with the Conference): the English players' player and young player of the year;
/// then for each division (Premier, First, Second, Third and, with the lower divisions, the Conference) the team of the
/// week, the manager, player and young player of the month (August to May), the manager of the year and the
/// division's team of the year
///
/// The count +8 (0x20 with `league_selected::LOWER_DIVISIONS_LOADED` in the nation's `nation::LEAGUE_SELECTED`, else
/// 0x1a; the error box at line 0x95 without the table) and the awards (`nation_awards_build`), kind 1, no scope: the
/// players' player of the year (national and world reputations 8000/5000, no competition) and young player (7000/3500);
/// then per division, from slot 2 + 6k: the team of the week (team award, span 1), the manager of the month (a month
/// award from August (7) of the current year, the dword [`G_CURRENT_DATE_YEAR`] with the original's stack high bytes
/// left clean, to May (4) of the next; not for players), the player and the young player of the month (month awards
/// for players), the manager of the year (not for players) and the players' select team of the year (team award); the
/// reputations by division as in `ENGLAND_DIVISION_AWARDS`. The original counts the slot index only when an award's
/// new succeeds (so one failed new shifts the rest); the copy keeps the fixed slots.
/// Original: `EnglandAwards::init_awards` (0x5788c0, england_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn england_awards_init(this: usize, file: usize) {
    let y = read_u16(G_CURRENT_DATE_YEAR) as u32;
    let n = read_u32(this + nation_awards_fields::NATION_ID);
    let f = file as u32;
    let k = |key: usize| read_u32(key);
    let lower = awards_lower_divisions(this);
    let mut specs = vec![
        AwardSpec::Year([n, 0, k(KEY_STAFF_COMP_ENGLISH_PLAYERS_PLAYER_OF_THE_YEAR), 8000, 5000, u32::MAX, 1, 0, 0, f, 1]),
        AwardSpec::Year([n, 1, k(KEY_STAFF_COMP_ENGLISH_PLAYERS_YOUNG_PLAYER_OF_THE_YEAR), 7000, 3500, u32::MAX, 1, 0, 0, f, 1]),
    ];
    let divisions = if lower { &ENGLAND_DIVISION_AWARDS[..] } else { &ENGLAND_DIVISION_AWARDS[..4] };
    for (d, (comp_key, block)) in divisions.iter().enumerate() {
        let comp = k(*comp_key);
        let i = 2 + 6 * d as u32;
        let r = &block.reputations;
        let month = |j: u32, key: usize, rep: (u32, u32), players: u32| AwardSpec::Month([n, i + j, k(key), rep.0, rep.1, comp, 7, y, 4, y + 1, f, players]);
        specs.push(AwardSpec::Team([n, i, k(block.keys[0]), r[0].0, r[0].1, comp, 1, 0, f, 1]));
        specs.push(month(1, block.keys[1], r[1], 0));
        specs.push(month(2, block.keys[2], r[2], 1));
        specs.push(month(3, block.keys[3], r[3], 1)); // rx11_6: the young player of the month
        specs.push(AwardSpec::Year([n, i + 4, k(block.keys[4]), r[4].0, r[4].1, comp, 1, 0, 0, f, 0]));
        specs.push(AwardSpec::Team([n, i + 5, k(block.keys[5]), r[5].0, r[5].1, comp, 1, 0, f, 1]));
    }
    covered!("rx11_6 english awards built");
    if lower {
        covered!("rx11_6 conference awards built");
    }
    nation_awards_build(this, &specs, ENGLAND_AWARDS_CPP, 0x95);
}

/// The English divisions' award blocks for `england_awards_init`: the club competition key and the six staff
/// competitions with their (national, world) reputations, Premier to Conference.
pub(crate) const ENGLAND_DIVISION_AWARDS: [(usize, EnglandDivisionAwards); 5] = [
    (KEY_CLUB_COMP_ENGLISH_PREMIER_DIVISION, EnglandDivisionAwards {
        keys: [KEY_STAFF_COMP_ENGLISH_PREMIER_DIVISION_TEAM_OF_THE_WEE, KEY_STAFF_COMP_ENGLISH_PREMIER_DIVISION_MANAGER_OF_THE_9CFBD8,
            KEY_STAFF_COMP_ENGLISH_PREMIER_DIVISION_PLAYER_OF_THE_M, KEY_STAFF_COMP_ENGLISH_PREMIER_DIVISION_YOUNG_PLAYER_OF,
            KEY_STAFF_COMP_ENGLISH_PREMIER_DIVISION_MANAGER_OF_THE_9CFBD4, KEY_STAFF_COMP_ENGLISH_PLAYERS_PREMIER_DIVISION_SELECT],
        reputations: [(4000, 500), (5000, 1500), (5000, 1500), (4500, 1000), (7500, 3500), (6500, 2500)] }),
    (KEY_CLUB_COMP_ENGLISH_FIRST_DIVISION, EnglandDivisionAwards {
        keys: [KEY_STAFF_COMP_ENGLISH_FIRST_DIVISION_TEAM_OF_THE_WEEK, KEY_STAFF_COMP_ENGLISH_FIRST_DIVISION_MANAGER_OF_THE_MO,
            KEY_STAFF_COMP_ENGLISH_FIRST_DIVISION_PLAYER_OF_THE_MON, KEY_STAFF_COMP_ENGLISH_FIRST_DIVISION_YOUNG_PLAYER_OF_T,
            KEY_STAFF_COMP_ENGLISH_FIRST_DIVISION_MANAGER_OF_THE_YE, KEY_STAFF_COMP_ENGLISH_PLAYERS_FIRST_DIVISION_SELECT],
        reputations: [(3000, 500), (3000, 500), (3500, 500), (3000, 500), (5000, 500), (4500, 500)] }),
    (KEY_CLUB_COMP_ENGLISH_SECOND_DIVISION, EnglandDivisionAwards {
        keys: [KEY_STAFF_COMP_ENGLISH_SECOND_DIVISION_TEAM_OF_THE_WEEK, KEY_STAFF_COMP_ENGLISH_SECOND_DIVISION_MANAGER_OF_THE_M,
            KEY_STAFF_COMP_ENGLISH_SECOND_DIVISION_PLAYER_OF_THE_MO, KEY_STAFF_COMP_ENGLISH_SECOND_DIVISION_YOUNG_PLAYER_OF,
            KEY_STAFF_COMP_ENGLISH_SECOND_DIVISION_MANAGER_OF_THE_Y, KEY_STAFF_COMP_ENGLISH_PLAYERS_SECOND_DIVISION_SELECT],
        reputations: [(2000, 0), (2000, 0), (2500, 0), (2000, 0), (3000, 0), (3000, 0)] }),
    (KEY_CLUB_COMP_ENGLISH_THIRD_DIVISION, EnglandDivisionAwards {
        keys: [KEY_STAFF_COMP_ENGLISH_THIRD_DIVISION_TEAM_OF_THE_WEEK, KEY_STAFF_COMP_ENGLISH_THIRD_DIVISION_MANAGER_OF_THE_MO,
            KEY_STAFF_COMP_ENGLISH_THIRD_DIVISION_PLAYER_OF_THE_MON, KEY_STAFF_COMP_ENGLISH_THIRD_DIVISION_YOUNG_PLAYER_OF_T,
            KEY_STAFF_COMP_ENGLISH_THIRD_DIVISION_MANAGER_OF_THE_YE, KEY_STAFF_COMP_ENGLISH_PLAYERS_THIRD_DIVISION_SELECT],
        reputations: [(1500, 0), (1000, 0), (1750, 0), (1500, 0), (2000, 0), (2000, 0)] }),
    (KEY_CLUB_COMP_ENGLISH_CONFERENCE, EnglandDivisionAwards {
        keys: [KEY_STAFF_COMP_ENGLISH_CONFERENCE_TEAM_OF_THE_WEEK, KEY_STAFF_COMP_ENGLISH_CONFERENCE_MANAGER_OF_THE_MONTH,
            KEY_STAFF_COMP_ENGLISH_CONFERENCE_PLAYER_OF_THE_MONTH, KEY_STAFF_COMP_ENGLISH_CONFERENCE_YOUNG_PLAYER_OF_THE_M,
            KEY_STAFF_COMP_ENGLISH_CONFERENCE_MANAGER_OF_THE_YEAR, KEY_STAFF_COMP_ENGLISH_PLAYERS_CONFERENCE_SELECT],
        reputations: [(1500, 0), (1000, 0), (1750, 0), (1500, 0), (2000, 0), (2000, 0)] }),
];

pub(crate) unsafe extern "thiscall" fn compare_england_awards_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, file);
    let count = if awards_lower_divisions(this) { 0x20 } else { 0x1a };
    compare_nation_awards_built(r, this, file, &ENGLAND_AWARDS, count, "england_awards_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_england_awards_init(this: usize, file: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%);
    compare_nation_awards_init(this, || o(this, file), "england_awards_init");
}
