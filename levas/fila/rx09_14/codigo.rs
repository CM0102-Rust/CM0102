// ---------- rx09_14: the 58 Brazilian awards (award/brazil_awards.cpp) ----------

/// Builds the Brazilian awards (`nation_awards_build`, 58, the error box at line 0xa3), all with the nation id the object
/// carries, kind 0, no scope and for players (the managers too), team awards with mode 0:
/// - First Division (0 to 4): the team of the week (team award, reputations 2000/0, span 1), the player of the year
///   (6000/3500), the top goalscorer (7000/3000, option 1), the young player of the year (5000/2500) and the team of
///   the year (team award, 5000/2000, span 0);
/// - Second Division (5 to 9): the same with 1000/0, 3000/500, 3500/500, 2500/1000 and 2500/100;
/// - each state championship of `BRAZIL_STATES` (10 + 4k): the team of the week (500/0, span 1), the best player
///   (5000/1500), the top goalscorer (4500/1000, option 1) and the best goalkeeper (3000/0).
/// Original: `BrazilAwards::init_awards` (0x43d340, brazil_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn brazil_awards_init_awards(this: usize, file: usize) {
    let n = read_u32(this + nation_awards_fields::NATION_ID);
    let f = file as u32;
    let k = |key: usize| read_u32(key);
    let team = |i: usize, key: usize, national: u32, world: u32, comp: usize, span: u32| AwardSpec::Team([n, i as u32, k(key), national, world, k(comp), span, 0, f, 1]);
    let year = |i: usize, key: usize, national: u32, world: u32, comp: usize, option: u32| AwardSpec::Year([n, i as u32, k(key), national, world, k(comp), 0, 0, option, f, 1]);
    let (first, second) = (KEY_CLUB_COMP_BRAZILIAN_NATIONAL_FIRST_DIVISION, KEY_CLUB_COMP_BRAZILIAN_NATIONAL_SECOND_DIVISION);
    let mut specs = vec![
        team(0, KEY_STAFF_COMP_BRAZILIAN_1ST_DIVISION_TEAM_OF_THE_WEEK, 2000, 0, first, 1),
        year(1, KEY_STAFF_COMP_BRAZILIAN_1ST_DIVISION_PLAYER_OF_THE_YEA, 6000, 3500, first, 0),
        year(2, KEY_STAFF_COMP_BRAZILIAN_1ST_DIVISION_TOP_GOALSCORER, 7000, 3000, first, 1),
        year(3, KEY_STAFF_COMP_BRAZILIAN_1ST_DIVISION_YOUNG_PLAYER_OF_T, 5000, 2500, first, 0),
        team(4, KEY_STAFF_COMP_BRAZILIAN_1ST_DIVISION_TEAM_OF_THE_YEAR, 5000, 2000, first, 0),
        team(5, KEY_STAFF_COMP_BRAZILIAN_2ND_DIVISION_TEAM_OF_THE_WEEK, 1000, 0, second, 1),
        year(6, KEY_STAFF_COMP_BRAZILIAN_2ND_DIVISION_PLAYER_OF_THE_YEA, 3000, 500, second, 0),
        year(7, KEY_STAFF_COMP_BRAZILIAN_2ND_DIVISION_TOP_GOALSCORER, 3500, 500, second, 1),
        year(8, KEY_STAFF_COMP_BRAZILIAN_2ND_DIVISION_YOUNG_PLAYER_OF_T, 2500, 1000, second, 0),
        team(9, KEY_STAFF_COMP_BRAZILIAN_2ND_DIVISION_TEAM_OF_THE_YEAR, 2500, 100, second, 0),
    ];
    for (s, &(comp, goalkeeper)) in BRAZIL_STATES.iter().enumerate() {
        let i = 10 + 4 * s;
        specs.push(team(i, goalkeeper + 8, 500, 0, comp, 1));
        specs.push(year(i + 1, goalkeeper + 4, 5000, 1500, comp, 0)); // rx09_14: the state best player
        specs.push(year(i + 2, goalkeeper + 0xc, 4500, 1000, comp, 1));
        specs.push(year(i + 3, goalkeeper, 3000, 0, comp, 0));
    }
    covered!("rx09_14 brazilian awards built");
    nation_awards_build(this, &specs, BRAZIL_AWARDS_CPP, 0xa3);
}

/// Comparer for `brazil_awards_init_awards` (`compare_nation_awards_init`): only the original runs (it allocates the
/// awards); each award built for this nation in its slot.
pub(crate) unsafe extern "thiscall" fn compare_brazil_awards_init_awards(this: usize, file: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%);
    compare_nation_awards_init(this, || o(this, file), "brazil_awards_init_awards");
}
