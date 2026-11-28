// ---------- sx05b_16: the USA awards: the object, its seven awards and the MLS season's end (award/usa_awards.cpp) ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\award\usa_awards.cpp" (the error boxes).
const USA_AWARDS_CPP: usize = 0xaa1f80;
/// `UsaAwards::init_awards(file)` (0x906ac0, thiscall, ret 4), through its hook.
const FN_USA_AWARDS_INIT: usize = 0x906ac0;

/// The USA awards' class: the vtable 0x970830, the United States (`KEY_NATION_UNITED_STATES`), nation id 0x14, type 1,
/// with month ratings (the error box at line 0x48 when they cannot be built).
const USA_AWARDS: NationAwardsClass = NationAwardsClass { vtable: 0x970830, nation_key: Some(KEY_NATION_UNITED_STATES), nation_id: 0x14, kind: 1,
    month_ratings: Some((USA_AWARDS_CPP, 0x48)) };

/// Builds the USA awards (`nation_awards_construct` with `USA_AWARDS`): the saved awards with a file, else the month
/// ratings and `UsaAwards::init_awards(file)`. Returns this.
/// Original: `UsaAwards::UsaAwards` (0x906980, usa_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn usa_awards_construct(this: usize, file: usize) -> usize {
    nation_awards_construct(this, &USA_AWARDS, file, core::mem::transmute(FN_USA_AWARDS_INIT))
}

/// Builds the seven USA awards (`nation_awards_build`, the error box at line 0x70), all of the MLS: the player of the
/// month (a month award from March to October of the current year, national reputation 3000), the manager of the year
/// (5000, not for players), the scoring champion (5000/500, option 1), the goalkeeper, the defender (5000/500) and the
/// rookie of the year (4000/500), and the all-pro team (team award, 3500/100); kind 0, no scope.
/// Original: `UsaAwards::init_awards` (0x906ac0, usa_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn usa_awards_init(this: usize, file: usize) {
    let year = read_u16(G_CURRENT_DATE_YEAR) as u32;
    let n = read_u32(this + nation_awards_fields::NATION_ID);
    let f = file as u32;
    let k = |key: usize| read_u32(key);
    let mls = k(KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE);
    let specs = [
        AwardSpec::Month([n, 0, k(KEY_STAFF_COMP_MLS_PLAYER_OF_THE_MONTH), 3000, 0, mls, 3, year, 0xa, year, f, 1]),
        AwardSpec::Year([n, 1, k(KEY_STAFF_COMP_MLS_MANAGER_OF_THE_YEAR), 5000, 0, mls, 0, 0, 0, f, 0]),
        AwardSpec::Year([n, 2, k(KEY_STAFF_COMP_MLS_SCORING_CHAMPION), 5000, 500, mls, 0, 0, 1, f, 1]),
        AwardSpec::Year([n, 3, k(KEY_STAFF_COMP_MLS_GOALKEEPER_OF_THE_YEAR), 5000, 500, mls, 0, 0, 0, f, 1]),
        AwardSpec::Year([n, 4, k(KEY_STAFF_COMP_MLS_DEFENDER_OF_THE_YEAR), 5000, 500, mls, 0, 0, 0, f, 1]),
        AwardSpec::Year([n, 5, k(KEY_STAFF_COMP_MLS_ROOKIE_OF_THE_YEAR), 4000, 500, mls, 0, 0, 0, f, 1]),
        AwardSpec::Team([n, 6, k(KEY_STAFF_COMP_MLS_ALL_PRO_TEAM), 3500, 100, mls, 0, 0, f, 1]),
    ];
    covered!("sx05b_16 usa awards built");
    nation_awards_build(this, &specs, USA_AWARDS_CPP, 0x70);
}

/// The USA awards' day: on 25 November (`awards_day_is`), the MLS manager of the year, the goalkeeper (positions 1),
/// the defender (6) and the rookie (age 21) of the year by the season's ratings (statistic 0x11), the scoring champion
/// (statistic 2) and the all-pro team (`NationAwards::award_team_of_season`). Every day, on the nation's season end day
/// (`is_season_end_day`), the month award moves on (`MONTH_AWARD::start_new_season` with today's year); then the base
/// daily update.
/// Original: `UsaAwards::daily_update` (0x906e10, usa_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn usa_awards_daily_update(this: usize, today: usize) {
    if awards_day_is(today, 0x19, 0xa) {
        covered!("sx05b_16 usa awards day");
        award_manager_of_year(this, today, nation_award(this, 1), KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE);
        for (k, age, positions, stat) in [(3, -1, 1, 0x11), (4, -1, 6, 0x11), (5, 0x15, 0, 0x11), (2, -1, 0, 2)] {
            award_player_of_year(this, KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE, today, nation_award(this, k), [age, positions, stat, -1, -1, -1, -1]);
        }
        award_team_of_season(this, KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE, nation_award(this, 6));
    }
    if is_season_end_day(read_usize(this + nation_awards_fields::NATION), today) {
        covered!("sx05b_16 month award moves on");
        month_award_start_new_season(nation_award(this, 0), read_u16(today + cm_date::YEAR));
    }
    nation_awards_base_daily_update(this, today);
}

pub(crate) unsafe extern "thiscall" fn compare_usa_awards_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_USA_AWARDS_CONSTRUCT)(this, file);
    compare_nation_awards_built(r, this, file, &USA_AWARDS, 7, "usa_awards_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_usa_awards_init(this: usize, file: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_USA_AWARDS_INIT);
    compare_nation_awards_init(this, || o(this, file), "usa_awards_init");
}

/// Comparer for `usa_awards_daily_update`: the awards cannot be given twice; only the original runs (the copy is
/// proven by the career).
pub(crate) unsafe extern "thiscall" fn compare_usa_awards_daily_update(this: usize, today: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_USA_AWARDS_DAILY_UPDATE)(this, today)
}

// ---------- sx05b_17: the MLS player of the month (award/usa_awards.cpp) ----------

/// The USA month: the MLS player of the month (award 0, a month award) from the nation's month ratings
/// (`award_month_player`), with no age limit (−1).
/// Original: `UsaAwards::on_month_change` (0x906f60, usa_awards.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn usa_awards_on_month_change(this: usize) {
    covered!("sx05b_17 mls player of the month");
    award_month_player(this, nation_award(this, 0), -1);
}

/// Comparer for `usa_awards_on_month_change`: the award cannot be given twice; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_usa_awards_on_month_change(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_USA_AWARDS_ON_MONTH_CHANGE)(this)
}

// ---------- sx04c_2: the Japanese awards: the object, its eight awards and the season's end (award/japan_awards.cpp) ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\award\japan_awards.cpp" (the error box).
const JAPAN_AWARDS_CPP: usize = 0x9eaf84;
/// `JapanAwards::init_awards(file)` (0x666720, thiscall, ret 4), through its hook.
const FN_JAPAN_AWARDS_INIT: usize = 0x666720;

/// The Japanese awards' class: the vtable 0x96cce8, Japan (`KEY_NATION_JAPAN`), nation id 0xe, type 1, no month
/// ratings.
const JAPAN_AWARDS: NationAwardsClass = NationAwardsClass { vtable: 0x96cce8, nation_key: Some(KEY_NATION_JAPAN), nation_id: 0xe, kind: 1, month_ratings: None };

/// Builds the Japanese awards (`nation_awards_construct` with `JAPAN_AWARDS`): the saved awards with a file, else
/// `JapanAwards::init_awards(file)`. Returns this.
/// Original: `JapanAwards::JapanAwards` (0x666690, japan_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn japan_awards_construct(this: usize, file: usize) -> usize {
    nation_awards_construct(this, &JAPAN_AWARDS, file, core::mem::transmute(FN_JAPAN_AWARDS_INIT))
}

/// Builds the eight Japanese awards (`nation_awards_build`, the error box at line 0x66), kind 0 and no competition for
/// the year awards: the manager (national and world reputations 5000/0, not for players), the player (5500/1500), the
/// top goalscorer (5000/500) and the young player of the year (3000/100); then the J1 League team of the week (2000/0,
/// mode 1) and team of the year (3000/100) and the J2 League team of the week (1000/0, mode 1) and team of the year
/// (1500/50), span 0.
/// Original: `JapanAwards::init_awards` (0x666720, japan_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn japan_awards_init(this: usize, file: usize) {
    let n = read_u32(this + nation_awards_fields::NATION_ID);
    let f = file as u32;
    let k = |key: usize| read_u32(key);
    let year = |i: u32, key: usize, national: u32, world: u32, players: u32| AwardSpec::Year([n, i, k(key), national, world, u32::MAX, 0, 0, 0, f, players]);
    let team = |i: u32, key: usize, national: u32, world: u32, comp: usize, mode: u32| AwardSpec::Team([n, i, k(key), national, world, k(comp), 0, mode, f, 1]);
    let (j1, j2) = (KEY_CLUB_COMP_JAPANESE_J_LEAGUE_1, KEY_CLUB_COMP_JAPANESE_J_LEAGUE_2);
    let specs = [
        year(0, KEY_STAFF_COMP_JAPANESE_MANAGER_OF_THE_YEAR, 5000, 0, 0),
        year(1, KEY_STAFF_COMP_JAPANESE_PLAYER_OF_THE_YEAR, 5500, 1500, 1),
        year(2, KEY_STAFF_COMP_JAPANESE_TOP_GOALSCORER, 5000, 500, 1),
        year(3, KEY_STAFF_COMP_JAPANESE_YOUNG_PLAYER_OF_THE_YEAR, 3000, 100, 1),
        team(4, KEY_STAFF_COMP_JAPANESE_J_LEAGUE_1_TEAM_OF_THE_WEEK, 2000, 0, j1, 1),
        team(5, KEY_STAFF_COMP_JAPANESE_J_LEAGUE_1_TEAM_OF_THE_YEAR, 3000, 100, j1, 0),
        team(6, KEY_STAFF_COMP_JAPANESE_J_LEAGUE_2_TEAM_OF_THE_WEEK, 1000, 0, j2, 1),
        team(7, KEY_STAFF_COMP_JAPANESE_J_LEAGUE_2_TEAM_OF_THE_YEAR, 1500, 50, j2, 0),
    ];
    covered!("sx04c_2 japanese awards built");
    nation_awards_build(this, &specs, JAPAN_AWARDS_CPP, 0x66);
}

/// The Japanese awards' day: on 20 December (`awards_day_is`), from the J1 League's stats (`award_player_of_year`) the
/// player (statistic 0x11), the top goalscorer (2) and the young player (up to 21) of the year, the J1 manager of the
/// year and the teams of the season of the J1 and J2 Leagues; then, every day, the base daily update.
/// Original: `JapanAwards::daily_update` (0x666ab0, japan_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn japan_awards_daily_update(this: usize, today: usize) {
    if awards_day_is(today, 0x14, 0xb) {
        covered!("sx04c_2 japanese awards day");
        let j1 = KEY_CLUB_COMP_JAPANESE_J_LEAGUE_1;
        for (k, age, stat) in [(1, -1, 0x11), (2, -1, 2), (3, 0x15, 0x11)] {
            award_player_of_year(this, j1, today, nation_award(this, k), [age, 0, stat, -1, -1, -1, -1]);
        }
        award_manager_of_year(this, today, nation_award(this, 0), j1);
        award_team_of_season(this, j1, nation_award(this, 5));
        award_team_of_season(this, KEY_CLUB_COMP_JAPANESE_J_LEAGUE_2, nation_award(this, 7));
    }
    nation_awards_base_daily_update(this, today);
}

pub(crate) unsafe extern "thiscall" fn compare_japan_awards_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_JAPAN_AWARDS_CONSTRUCT)(this, file);
    compare_nation_awards_built(r, this, file, &JAPAN_AWARDS, 8, "japan_awards_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_japan_awards_init(this: usize, file: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_JAPAN_AWARDS_INIT);
    compare_nation_awards_init(this, || o(this, file), "japan_awards_init");
}

/// Comparer for `japan_awards_daily_update`: the awards cannot be given twice; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_japan_awards_daily_update(this: usize, today: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_JAPAN_AWARDS_DAILY_UPDATE)(this, today)
}

// ---------- cx01_10: the Brazilian Champions Cup: constructor, entrants and predicted positions (bra_champ_cup.cpp) ----------

/// The Champions Cup's vtable.
const BRA_CHAMP_CUP_VTABLE: usize = 0x967c44;
/// "E:\dev\CM3\cm3 00-01\cm3\code\comp\cups\bra_champ_cup.cpp", for the error boxes.
const BRA_CHAMP_CUP_SOURCE: usize = 0x98a1e8;
/// `BraChampCup::set_up_rules` (0x426230, bra_champ_cup.cpp, thiscall, ret 0; 0 on failure).
const FN_BRA_CHAMP_CUP_SET_UP_RULES: usize = 0x426230;
/// `compare_clubs_by_reputation_desc_nulls_last` (0x4b07d0, comp_util.cpp, cdecl): the qsort order by reputation.
const FN_COMPARE_CLUBS_BY_REPUTATION_DESC_NULLS_LAST: usize = 0x4b07d0;

/// The nine places of the Champions Cup and the long names of the clubs named for each; the name tests run in this
/// order for every Brazilian club, a later match overwriting the place (Vasco and Vila Nova both go to place 0).
const BRA_CHAMP_CUP_NAMED_CLUBS: [(usize, usize); 10] = [
    (0x98a398, 0), // "Associação Desportiva Vasco da Gama"
    (0x98a380, 0), // "Vila Nova Futebol Clube"
    (0x98a364, 1), // "Clube Náutico Capibaribe"
    (0x98a344, 2), // "São Raimundo Esporte Clube (AM)"
    (0x98a328, 3), // "América Futebol Clube (MG)"
    (0x98a304, 4), // "Grêmio Foot-Ball Porto-Alegrense"
    (0x98a2e4, 5), // "Clube de Regatas do Flamengo"
    (0x98a2c8, 6), // "Clube Atlético Paranaense"
    (0x98a2a8, 7), // "Sport Club Corinthians Paulista"
    (0x98a294, 8), // "Esporte Clube Bahia"
];

/// The Brazilian state championships whose strongest club fills an empty place, with that place.
const BRA_CHAMP_CUP_STATE_PLACES: [(usize, usize); 12] = [
    (KEY_CLUB_COMP_BRAZILIAN_BAHIA_STATE_CHAMPIONSHIP, 8),
    (KEY_CLUB_COMP_BRAZILIAN_CENTRAL_STATE_CHAMPIONSHIP, 0),
    (KEY_CLUB_COMP_BRAZILIAN_GAUCHO_STATE_CHAMPIONSHIP, 6),
    (KEY_CLUB_COMP_BRAZILIAN_GOIAS_STATE_CHAMPIONSHIP, 1),
    (KEY_CLUB_COMP_BRAZILIAN_MINAS_GERAIS_STATE_CHAMPIONSHI, 5),
    (KEY_CLUB_COMP_BRAZILIAN_NORTH_STATE_CHAMPIONSHIP, 0),
    (KEY_CLUB_COMP_BRAZILIAN_NORTHEAST_STATE_CHAMPIONSHIP, 0),
    (KEY_CLUB_COMP_BRAZILIAN_PARANA_STATE_CHAMPIONSHIP, 7),
    (KEY_CLUB_COMP_BRAZILIAN_PERNAMBUCO_STATE_CHAMPIONSHIP, 2),
    (KEY_CLUB_COMP_BRAZILIAN_RIO_DE_JANEIRO_STATE_CHAMPIONS, 3),
    (KEY_CLUB_COMP_BRAZILIAN_SANTA_CATARINA_STATE_CHAMPIONS, 1),
    (KEY_CLUB_COMP_BRAZILIAN_SAO_PAULO_STATE_CHAMP, 4),
];

/// The predicted final positions of the nine entrants, in entrant order.
const BRA_CHAMP_CUP_PREDICTED: [u8; 9] = [1, 2, 3, 3, 5, 5, 7, 7, 9];

/// The place of a state championship id in `BRA_CHAMP_CUP_STATE_PLACES`.
unsafe fn bra_champ_cup_state_place(id: u32) -> Option<usize> {
    BRA_CHAMP_CUP_STATE_PLACES.iter().find(|&&(key, _)| read_u32(key) == id).map(|&(_, place)| place)
}

/// Builds the Brazilian Champions Cup: League::League (`exe::new_league`), the competition record, the vtable, the
/// year (word), 3 substitutions allowed, 7 substitutes named, discipline rule 6 and importance 12. A saved state
/// (`exe::league_load_from_temp_file` with 1) ends it. Otherwise the league flags 0x28, the main stage (0xff), the
/// current stage −1, three stages and their vector (`exe::crt_malloc` of 12, unchecked), the entrants
/// (`bra_champ_cup_select_entrants`), the statistics (operator new 0x5ce and `exe::new_comp_stats`; none: the error box
/// of line 0x53) and the rules (`BraChampCup::set_up_rules`; 0: the box of line 0x5a); with them, the competition is
/// not active yet and has no table nor clubs. Returns the object.
/// Original: `BraChampCup::BraChampCup` (0x4259a0, bra_champ_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_bra_champ_cup(this: usize, year: u32, record: usize) -> usize {
    let cup = League(this);
    exe::new_league(this);
    write_usize(this + league::COMPETITION, record);
    write_usize(this, BRA_CHAMP_CUP_VTABLE);
    cup.set_year(year as u16 as i16);
    cup.set_substitutions_allowed(3);
    cup.set_substitutes_named(7);
    cup.set_discipline_rule(6);
    cup.set_importance(0xc);
    if exe::league_load_from_temp_file(this, 1) != 0 {
        return this;
    }
    covered!("cx01_10 new champions cup");
    cup.set_league_flags(0x28);
    cup.set_stage_index(stage_index::MAIN);
    write_i32(this + league::CURRENT_STAGE, -1);
    cup.set_stage_count(3);
    cup.set_stages(exe::crt_malloc(0xc));
    bra_champ_cup_select_entrants(this);
    let p = exe::crt_operator_new(0x5ce);
    let stats = if p != 0 { exe::new_comp_stats(p, this) } else { 0 };
    cup.set_comp_stats(stats);
    if stats == 0 {
        show_error_at(BRA_CHAMP_CUP_SOURCE, 0x53);
        return this;
    }
    let rules: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_BRA_CHAMP_CUP_SET_UP_RULES);
    if rules(this) == 0 {
        show_error_at(BRA_CHAMP_CUP_SOURCE, 0x5a);
        return this;
    }
    cup.set_active(0);
    cup.set_table(0);
    cup.set_club_count(0);
    this
}

/// Picks the nine entrants of the Brazilian Champions Cup into a new entrant book.
///
/// First, in table order and up to 200, the clubs whose state championship (`club::RESERVE_DIVISION`) is one of the
/// twelve of `BRA_CHAMP_CUP_STATE_PLACES`, sorted by reputation (`exe::crt_qsort` with
/// `compare_clubs_by_reputation_desc_nulls_last`). The book: `ENTRANTS_ARE_QUALIFIERS` = 0xff, capacity and count 9,
/// nine 6-byte entries (`exe::crt_calloc`; none: the error box of line 0x3d7). Every Brazilian club (`club::NATION`
/// id = Brazil) whose long name is one of `BRA_CHAMP_CUP_NAMED_CLUBS` takes that place. If a place is still empty,
/// each listed club in order takes its state's place when that place is empty.
/// Original: `BraChampCup::select_entrants` (0x426d80, bra_champ_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_champ_cup_select_entrants(this: usize) {
    let cup = League(this);
    let mut listed = [0usize; 0xc8];
    let mut n = 0usize;
    let mut i = 0i16;
    while (i as i32) < read_i32(G_CLUB_COUNT) && n < 0xc8 {
        let c = club_at(i as isize);
        let state = c.reserve_division();
        if !state.is_null() && bra_champ_cup_state_place(read_u32(state.addr())).is_some() {
            listed[n] = c.addr();
            n += 1;
        }
        i += 1;
    }
    exe::crt_qsort(listed.as_mut_ptr() as usize, n as u32, 4, FN_COMPARE_CLUBS_BY_REPUTATION_DESC_NULLS_LAST);
    cup.set_entrants_are_qualifiers(0xff);
    cup.set_entrant_capacity(9);
    cup.set_entrant_count(9);
    let book = exe::crt_calloc(6, 9);
    cup.set_entrant_book(book);
    if book == 0 {
        show_error_at(BRA_CHAMP_CUP_SOURCE, 0x3d7);
        return;
    }
    let brazil = read_u32(KEY_NATION_BRAZIL);
    let mut i = 0i16;
    while (i as i32) < read_i32(G_CLUB_COUNT) {
        let c = club_at(i as isize);
        if !c.nation().is_null() && read_u32(c.nation().addr()) == brazil {
            let name = c_string_bytes(c.name_addr());
            for &(named, place) in &BRA_CHAMP_CUP_NAMED_CLUBS {
                if name == c_string_bytes(named) {
                    covered!("cx01_10 named entrant");
                    write_usize(book + 6 * place, c.addr());
                }
            }
        }
        i += 1;
    }
    if (0..9).all(|k| read_usize(book + 6 * k) != 0) {
        return;
    }
    for &c in &listed[..n] {
        if let Some(place) = bra_champ_cup_state_place(read_u32(Club(c).reserve_division().addr())) {
            if read_usize(book + 6 * place) == 0 {
                covered!("cx01_10 state entrant");
                write_usize(book + 6 * place, c);
            }
        }
    }
}

/// Stores the predicted final positions of the nine entrants (`BRA_CHAMP_CUP_PREDICTED`, in entrant order) in the
/// competition statistics (`exe::comp_stats_set_club_final_position`). Without statistics the error box of line 0x319;
/// the clubs are first copied into a list of nine (`exe::crt_malloc`; none: the box of line 0x321), freed at the end.
/// Original: `BraChampCup::set_predicted_positions` (0x426ab0, bra_champ_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_champ_cup_set_predicted_positions(this: usize) {
    let cup = League(this);
    if cup.comp_stats() == 0 {
        show_error_at(BRA_CHAMP_CUP_SOURCE, 0x319);
        return;
    }
    let list = exe::crt_malloc(0x24);
    if list == 0 {
        show_error_at(BRA_CHAMP_CUP_SOURCE, 0x321);
        return;
    }
    covered!("cx01_10 predicted positions");
    for k in 0..9 {
        write_usize(list + 4 * k, read_usize(cup.entrant_book() + 6 * k));
    }
    for (k, &position) in BRA_CHAMP_CUP_PREDICTED.iter().enumerate() {
        exe::comp_stats_set_club_final_position(cup.comp_stats(), read_usize(list + 4 * k), position as u32);
    }
    exe::crt_free(list);
}

/// Comparer for `construct_bra_champ_cup`: only the original runs (it allocates the stages, the book and the
/// statistics); the return, the vtable, the year, the record and the bench and discipline values must be the ones the
/// copy writes.
pub(crate) unsafe extern "thiscall" fn compare_construct_bra_champ_cup(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_BRA_CHAMP_CUP)(this, year, record);
    let cup = League(this);
    let ok = r == this && read_usize(this) == BRA_CHAMP_CUP_VTABLE && cup.year() == year as u16 as i16 && cup.competition().addr() == record
        && cup.substitutions_allowed() == 3 && cup.substitutes_named() == 7 && cup.discipline_rule() == 6 && cup.importance() == 0xc;
    SCOREBOARD.record(ok, || format!("construct_bra_champ_cup({:#x}, {}, {:#x}): returned {:#x}, vtable {:#x}, year {}, record {:#x}, bench {}/{}, discipline {}, importance {}",
        this, year as u16, record, r, read_usize(this), cup.year(), cup.competition().addr(), cup.substitutes_named(), cup.substitutions_allowed(), cup.discipline_rule(), cup.importance()));
    r
}

/// Comparer for `bra_champ_cup_select_entrants`: the original builds its book, then the copy builds another; the nine
/// entries and the book fields must match. The copy's book is freed and the original's put back.
pub(crate) unsafe extern "thiscall" fn compare_bra_champ_cup_select_entrants(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_CHAMP_CUP_SELECT_ENTRANTS)(this);
    let cup = League(this);
    let book = cup.entrant_book();
    if book == 0 {
        return;
    }
    let fields = |c: League| (c.entrants_are_qualifiers(), c.entrant_capacity(), c.entrant_count());
    let original = (fields(cup), core::slice::from_raw_parts(book as *const u8, 6 * 9).to_vec());
    bra_champ_cup_select_entrants(this);
    let mine = cup.entrant_book();
    let copy = (fields(cup), if mine != 0 { core::slice::from_raw_parts(mine as *const u8, 6 * 9).to_vec() } else { Vec::new() });
    if mine != 0 {
        exe::crt_free(mine);
    }
    cup.set_entrant_book(book);
    SCOREBOARD.record(original == copy, || format!("bra_champ_cup_select_entrants({:#x}): original {:x?} rust {:x?}", this, original, copy));
}

/// Comparer for `bra_champ_cup_set_predicted_positions`: only the original runs (it may add the clubs to the
/// statistics); each entrant's final position must then be the copy's (`BRA_CHAMP_CUP_PREDICTED`).
pub(crate) unsafe extern "thiscall" fn compare_bra_champ_cup_set_predicted_positions(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_CHAMP_CUP_SET_PREDICTED_POSITIONS)(this);
    let cup = League(this);
    if cup.comp_stats() == 0 || cup.entrant_book() == 0 {
        return;
    }
    let got: Vec<u8> = (0..9).map(|k| {
        let e = exe::comp_stats_find_club_entry(cup.comp_stats(), read_usize(cup.entrant_book() + 6 * k));
        if e != 0 { CompStatsClub(e).final_position() } else { 0 }
    }).collect();
    SCOREBOARD.record(got == BRA_CHAMP_CUP_PREDICTED, || format!("bra_champ_cup_set_predicted_positions({:#x}): {:?}", this, got));
}

// ---------- cx01_11: the Brazilian Champions Cup: the preliminary stage and the next stage (bra_champ_cup.cpp) ----------

/// `BraChampCup::create_group_stages` (0x426470, bra_champ_cup.cpp, thiscall, ret 0).
const FN_BRA_CHAMP_CUP_CREATE_GROUP_STAGES: usize = 0x426470;
/// `BraChampCup::create_semi_final_stage` (0x426720, bra_champ_cup.cpp, thiscall, ret 0).
const FN_BRA_CHAMP_CUP_CREATE_SEMI_FINAL_STAGE: usize = 0x426720;
/// `BraChampCup::create_preliminary_table` (0x426130, bra_champ_cup.cpp, thiscall, ret 0).
const FN_BRA_CHAMP_CUP_CREATE_PRELIMINARY_TABLE: usize = 0x426130;

/// Builds the table of the three-club preliminary (the cup itself is the league): three clubs (`CLUB_COUNT`), the table
/// (`exe::crt_malloc` of 0xb1 bytes; none: the error box of line 0x195 and 0), the first three entrants in rows 0 to 2
/// (`exe::league_init_table_row` with 0, 0), each taken out of the entrant book as it goes in, and the group draw news
/// (`exe::league_send_group_draw_news`). Returns 1.
/// Original: `BraChampCup::create_preliminary_table` (0x426130, bra_champ_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_champ_cup_create_preliminary_table(this: usize) -> u32 {
    let cup = League(this);
    cup.set_club_count(3);
    let table = exe::crt_malloc(0xb1);
    cup.set_table(table);
    if table == 0 {
        show_error_at(BRA_CHAMP_CUP_SOURCE, 0x195);
        return 0;
    }
    covered!("cx01_11 preliminary table");
    let mut row = 0i8;
    while (row as i16) < cup.club_count() {
        let entry = cup.entrant_book() + row as isize as usize * 6;
        // the original passes the row as a dword whose high bytes are whatever was on the stack: only the byte is read
        exe::league_init_table_row(this, row as u8 as u32, read_usize(entry), 0, 0);
        write_usize(entry, 0); // the club leaves the entrant book
        row = row.wrapping_add(1);
    }
    exe::league_send_group_draw_news(this);
    1
}

/// Starts the preliminary: the predicted positions (`comp_vtable::SET_PREDICTED_POSITIONS`), then, each with its error
/// box when it answers 0, the preliminary table (`BraChampCup::create_preliminary_table`; line 0x491), the fixtures
/// (`exe::league_create_fixtures`; line 0x497) and the table order (`exe::league_sort_table` with 0; line 0x49e);
/// finally the competition is active.
/// Original: `BraChampCup::start_preliminary_stage` (0x427320, bra_champ_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_champ_cup_start_preliminary_stage(this: usize) {
    let cup = League(this);
    let predicted: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::SET_PREDICTED_POSITIONS));
    predicted(this);
    let table: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_BRA_CHAMP_CUP_CREATE_PRELIMINARY_TABLE);
    if table(this) == 0 {
        return show_error_at(BRA_CHAMP_CUP_SOURCE, 0x491);
    }
    if exe::league_create_fixtures(this) == 0 {
        return show_error_at(BRA_CHAMP_CUP_SOURCE, 0x497);
    }
    if exe::league_sort_table(this, 0) == 0 {
        return show_error_at(BRA_CHAMP_CUP_SOURCE, 0x49e);
    }
    covered!("cx01_11 preliminary started");
    cup.set_active(1);
}

/// Moves the Champions Cup on by one stage: at the last stage (`CURRENT_STAGE` ≥ `STAGE_COUNT` − 1, signed) the error
/// box of line 0x219 and nothing else; otherwise the stage goes up by one and, at 0 (the end of the preliminary), the
/// groups are built (`BraChampCup::create_group_stages`); at 2 (the end of the groups), the semi-finals
/// (`BraChampCup::create_semi_final_stage`).
/// Original: `BraChampCup::advance_stage` (0x4263c0, bra_champ_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_champ_cup_advance_stage(this: usize) {
    let cup = League(this);
    let stage = cup.current_stage() as i32;
    if stage >= (cup.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(BRA_CHAMP_CUP_SOURCE, 0x219);
    }
    let stage = stage + 1;
    write_i32(this + league::CURRENT_STAGE, stage);
    let next = match stage {
        0 => FN_BRA_CHAMP_CUP_CREATE_GROUP_STAGES, // the groups after the preliminary
        2 => FN_BRA_CHAMP_CUP_CREATE_SEMI_FINAL_STAGE, // the semi-finals after the groups
        _ => return,
    };
    covered!("cx01_11 next stage built");
    let build: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(next);
    build(this);
}

/// Comparer for `bra_champ_cup_create_preliminary_table`: only the original runs (the rows allocate and the news goes
/// to the humans); the plan made before must hold: 1 returned, three clubs, the first three entrants in the rows in
/// order and taken out of the book.
pub(crate) unsafe extern "thiscall" fn compare_bra_champ_cup_create_preliminary_table(this: usize) -> u32 {
    let cup = League(this);
    let plan: Vec<usize> = (0..3).map(|k| read_usize(cup.entrant_book() + k * 6)).collect();
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_CHAMP_CUP_CREATE_PRELIMINARY_TABLE)(this);
    let table = cup.table();
    let ok = r == 1 && cup.club_count() == 3 && table != 0
        && plan.iter().enumerate().all(|(k, &c)| read_usize(table + k * league_table_row::SIZE) == c && read_usize(cup.entrant_book() + k * 6) == 0);
    SCOREBOARD.record(ok, || format!("bra_champ_cup_create_preliminary_table({:#x}): returned {}, plan {:x?}", this, r, plan));
    r
}

/// Comparer for `bra_champ_cup_start_preliminary_stage`: only the original runs (the table, the fixtures and the order
/// allocate and draw); then the competition must be active with three clubs.
pub(crate) unsafe extern "thiscall" fn compare_bra_champ_cup_start_preliminary_stage(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_CHAMP_CUP_START_PRELIMINARY_STAGE)(this);
    let cup = League(this);
    SCOREBOARD.record(cup.active() == 1 && cup.club_count() == 3, || format!("bra_champ_cup_start_preliminary_stage({:#x}): active {} clubs {}", this, cup.active(), cup.club_count()));
}

/// Comparer for `bra_champ_cup_advance_stage`: only the original runs (the groups and the semi-finals allocate and
/// draw); the stage stays at the last one and otherwise goes up by one (the groups write their own stage afterwards:
/// there it only has to move on).
pub(crate) unsafe extern "thiscall" fn compare_bra_champ_cup_advance_stage(this: usize) {
    let cup = League(this);
    let before = cup.current_stage() as i32;
    let last = before >= (cup.stage_count() as i32).wrapping_sub(1);
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_CHAMP_CUP_ADVANCE_STAGE)(this);
    let after = cup.current_stage() as i32;
    let ok = if last { after == before } else if before + 1 == 0 { after > before } else { after == before + 1 };
    SCOREBOARD.record(ok, || format!("bra_champ_cup_advance_stage({:#x}): stage {} -> {}", this, before, after));
}

// ---------- cx01_12: the Brazilian Champions Cup: the groups, the semi-finals and the season turnover (bra_champ_cup.cpp) ----------

/// The places of a Champions Cup group (the first two go through), passed to League_Stage::League_Stage.
const BRA_CHAMP_CUP_GROUP_PLACES: [u8; 4] = [2, 0, 0, 0];

/// The four clubs of each Champions Cup group: the preliminary's winner with entrants 3, 4 and 5 (group A), the
/// preliminary's runner-up with entrants 6, 7 and 8 (group B).
unsafe fn bra_champ_cup_group_clubs(cup: League) -> [[usize; 4]; 2] {
    let (table, book) = (cup.table(), cup.entrant_book());
    let row = |k: usize| read_usize(table + k * league_table_row::SIZE);
    let entrant = |k: usize| read_usize(book + k * 6);
    [[row(0), entrant(3), entrant(4), entrant(5)], [row(1), entrant(6), entrant(7), entrant(8)]]
}

/// Builds group `g` of the Champions Cup as stage `g`: its round table (`comp_vtable::CREATE_SCHEDULE` for stage g),
/// the League_Stage (operator new 0xee; `FN_LEAGUE_STAGE_LEAGUE_STAGE` with the cup, 4 clubs, `clubs`, 1, the record,
/// the table, the rounds, the win and draw points, the match category, the tie-breaks, `BRA_CHAMP_CUP_GROUP_PLACES`,
/// the year, g, the stage label, the importance, 1, 0, the league flags, −1, 0, 2) into stage g and the round table
/// freed. Returns the stage (null when it was not built).
unsafe fn bra_champ_cup_build_group(cup: League, g: u32, clubs: &[usize; 4]) -> usize {
    let schedule: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(cup.addr()) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut label) = (0i16, 0i16);
    let table = schedule(cup.addr(), g, &mut rounds, &mut label, 0);
    let p = exe::crt_operator_new(0xee);
    let mut stage = 0usize;
    if p != 0 {
        type LeagueStageCtor = unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize;
        let ctor: LeagueStageCtor = core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
        // simplification: the original pushes the rounds, the label, the year and the flags as dwords with stale high
        // bytes; League_Stage::League_Stage reads the words and bytes
        stage = ctor(p, cup.addr(), 4, clubs.as_ptr() as usize, 1, cup.competition().addr(), table, rounds as u16 as u32, cup.win_points() as u32,
            cup.draw_points() as u32, cup.match_category() as u32, cup.addr() + league::TIEBREAKS, BRA_CHAMP_CUP_GROUP_PLACES.as_ptr() as usize,
            cup.year() as u16 as u32, g, label as u16 as u32, cup.importance() as u32, 1, 0, cup.league_flags() as u16 as u32, 0xffff_ffff, 0, 2);
    }
    write_usize(cup.stages() + 4 * g as usize, stage);
    exe::crt_free(table);
    read_usize(cup.stages() + 4 * g as usize)
}

/// Builds the two four-club groups of the Brazilian Champions Cup (`bra_champ_cup_group_clubs`), each as a stage
/// (`bra_champ_cup_build_group`) followed by its group draw news (`exe::league_send_group_draw_news`); a group not
/// built shows the error box (group A line 0x252, group B line 0x268) and answers 0. Otherwise the current stage is 1
/// and it answers 1.
/// Original: `BraChampCup::create_group_stages` (0x426470, bra_champ_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_champ_cup_create_group_stages(this: usize) -> u32 {
    let cup = League(this);
    let groups = bra_champ_cup_group_clubs(cup);
    for (g, row) in [(0u32, 0x252u32), (1, 0x268)] {
        let stage = bra_champ_cup_build_group(cup, g, &groups[g as usize]);
        if stage == 0 {
            show_error_at(BRA_CHAMP_CUP_SOURCE, row);
            return 0;
        }
        exe::league_send_group_draw_news(stage);
    }
    covered!("cx01_12 champions cup groups");
    write_i32(this + league::CURRENT_STAGE, 1);
    1
}

/// The four semi-finalists in draw order: group A's winner, group B's runner-up, group A's runner-up and group B's
/// winner (the first two rows of each group's table).
unsafe fn bra_champ_cup_semi_finalists(cup: League) -> [usize; 4] {
    let row = |g: usize, k: usize| read_usize(League(read_usize(cup.stages() + 4 * g)).table() + k * league_table_row::SIZE);
    [row(0, 0), row(1, 1), row(0, 1), row(1, 0)]
}

/// Builds the four-club knockout of the Champions Cup as stage 2: the semi-finalists (`bra_champ_cup_semi_finalists`)
/// in a list of four (`exe::crt_malloc`; none: the error box of line 0x286), then `create_knockout_stage` (the round
/// table of stage 2, the Cup_Stage with no seeds, the list and the table freed, the box of line 0x29d without a stage,
/// otherwise the first draw).
/// Original: `BraChampCup::create_semi_final_stage` (0x426720, bra_champ_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_champ_cup_create_semi_final_stage(this: usize) {
    let cup = League(this);
    let clubs = exe::crt_malloc(0x10);
    if clubs == 0 {
        return show_error_at(BRA_CHAMP_CUP_SOURCE, 0x286);
    }
    covered!("cx01_12 champions cup semi-finals");
    for (k, c) in bra_champ_cup_semi_finalists(cup).into_iter().enumerate() {
        write_usize(clubs + 4 * k, c);
    }
    create_knockout_stage(cup, 4, clubs, 2, core::ptr::null(), BRA_CHAMP_CUP_SOURCE, 0x29d);
}

/// Turns the Champions Cup over to a new season: not closed, the table freed (`exe::league_free_table` with 0), the
/// round dates freed and cleared, the statistics archived (`exe::comp_stats_archive_season_and_reset` with 1), every
/// stage up to the current one destroyed (virtual slot 0 with 1), the year up by one and the current stage −1; then
/// the rules (`comp_vtable::SET_UP_RULES`; 0: the error box of line 0x1fc and 0); with them, no table, no clubs and not
/// active, and it answers the rules' value.
/// Original: `BraChampCup::start_new_season` (0x4262c0, bra_champ_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_champ_cup_start_new_season(this: usize) -> u32 {
    let cup = League(this);
    cup.set_season_closed(0);
    exe::league_free_table(this, 0);
    if cup.round_dates() != 0 {
        exe::crt_free(cup.round_dates());
        cup.set_round_dates(0);
    }
    if cup.comp_stats() != 0 {
        exe::comp_stats_archive_season_and_reset(cup.comp_stats(), 1);
    }
    let mut k = 0i32;
    while k <= cup.current_stage() as i32 {
        let stage = read_usize(cup.stages() + 4 * k as usize);
        if stage != 0 {
            let destroy: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(stage)));
            destroy(stage, 1);
        }
        k += 1;
    }
    cup.set_year(cup.year().wrapping_add(1));
    write_i32(this + league::CURRENT_STAGE, -1);
    let rules: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::SET_UP_RULES));
    let r = rules(this);
    if r == 0 {
        // simplification: the original returns the box's eax; the season turnover does not read it
        show_error_at(BRA_CHAMP_CUP_SOURCE, 0x1fc);
        return 0;
    }
    covered!("cx01_12 champions cup new season");
    cup.set_table(0);
    cup.set_club_count(0);
    cup.set_active(0);
    r
}

/// The clubs of a stage's table (league rows) or team table (cup), as a sorted list.
unsafe fn stage_clubs(stage: usize) -> Vec<usize> {
    let s = League(stage);
    let mut v: Vec<usize> = if s.comp_type() == 1 {
        (0..s.club_count().max(0) as usize).map(|k| read_usize(Cup(stage).team_table() + 6 * k)).collect()
    } else {
        (0..s.club_count().max(0) as usize).map(|k| read_usize(s.table() + k * league_table_row::SIZE)).collect()
    };
    v.sort_unstable();
    v
}

/// Comparer for `bra_champ_cup_create_group_stages`: only the original runs (the stages allocate and the news goes
/// out); the groups the copy plans before must be the clubs of the two stages built, and the stage 1.
pub(crate) unsafe extern "thiscall" fn compare_bra_champ_cup_create_group_stages(this: usize) -> u32 {
    let cup = League(this);
    let plan = bra_champ_cup_group_clubs(cup).map(|mut g| { g.sort_unstable(); g.to_vec() });
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_CHAMP_CUP_CREATE_GROUP_STAGES)(this);
    let built: Vec<Vec<usize>> = (0..2).map(|g| { let s = read_usize(cup.stages() + 4 * g); if s != 0 { stage_clubs(s) } else { Vec::new() } }).collect();
    let ok = r != 1 || (cup.current_stage() as i32 == 1 && built[0] == plan[0] && built[1] == plan[1]);
    SCOREBOARD.record(ok, || format!("bra_champ_cup_create_group_stages({:#x}): returned {}, plan {:x?}, built {:x?}", this, r, plan, built));
    r
}

/// Comparer for `bra_champ_cup_create_semi_final_stage`: only the original runs (the stage allocates and draws); the
/// semi-finalists the copy plans before must be the clubs of the knockout stage built.
pub(crate) unsafe extern "thiscall" fn compare_bra_champ_cup_create_semi_final_stage(this: usize) {
    let cup = League(this);
    let mut plan = bra_champ_cup_semi_finalists(cup).to_vec();
    plan.sort_unstable();
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_CHAMP_CUP_CREATE_SEMI_FINAL_STAGE)(this);
    let stage = read_usize(cup.stages() + 8);
    let built = if stage != 0 { stage_clubs(stage) } else { Vec::new() };
    SCOREBOARD.record(stage == 0 || built == plan, || format!("bra_champ_cup_create_semi_final_stage({:#x}): plan {:x?}, built {:x?}", this, plan, built));
}

/// Comparer for `bra_champ_cup_start_new_season`: only the original runs (`compare_new_season`: the year up by one,
/// three stages and the current one −1).
pub(crate) unsafe extern "thiscall" fn compare_bra_champ_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BRA_CHAMP_CUP_START_NEW_SEASON);
    compare_new_season("bra_champ_cup_start_new_season", League(this), || o(this))
}

// ---------- cx01_13: the Brazilian Champions Cup: the staff records, a club's progress and the fate news (bra_champ_cup.cpp) ----------

/// Writes a club's staff records for its status in the Champions Cup and returns 0.
///
/// `BraChampCup::update_staff_records_for_status(club, outcome, stage, _, fixture, extra)` (thiscall, ret 0x18). In the
/// preliminary (stage 0): qualified (outcome 6) gets `add_qualified_for_round_record` (stage 0, round 0xa, weight 40),
/// out (0xfe) `add_failed_to_qualify_record` (0x41c, weight 40). At the other stages below 2 (signed): qualified gets
/// round 0x82 (weight 40), out gets 0xa (weight 40). At stage 2 (the knockout): out (0xff) the knocked-out record with
/// the fixture's round prefix and name, weighted 50 to 100 by its round number of 2 (`scale_by_round`); through (0) the
/// qualified record with the round prefix and the next round's code in the knockout stage's round table, the same
/// weight; the winner (1) the cup winners record and the club as the CONMEBOL seeding's Champions Cup winner
/// (`set_bra_champ_cup_winner`); the runner-up (2) the runners-up record. Stages above 2 write nothing.
/// Original: `BraChampCup::update_staff_records_for_status` (0x426900, bra_champ_cup.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn bra_champ_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, _a4: u32, fixture: usize, extra: u32) -> u32 {
    let cup = League(this);
    let comp = cup.competition().addr();
    let (outcome, stage) = (outcome as u8, stage as u8 as i8);
    if stage < 2 {
        let (round, out_stage) = if stage == 0 { (0xa, 0x41c) } else { (0x82, 0xa) };
        match outcome {
            6 => {
                covered!("cx01_13 records: qualified");
                european_cup_add_qualified_record(club, comp, 0, round, 0x28);
            }
            0xfe => {
                covered!("cx01_13 records: failed to qualify");
                european_cup_add_failed_to_qualify_record(club, comp, out_stage, 0x28);
            }
            _ => {}
        }
        return 0;
    }
    if stage != 2 {
        return 0;
    }
    let game = Fixture(fixture);
    match outcome {
        0xff => {
            let weight = scale_by_round(0x32, 0x64, game.round_number(), 2);
            // simplification: the round prefix and name go in registers with stale high halves; 0x86c000 reads the words
            european_cup_add_knocked_out_record(club, comp, game.round_prefix() as u16 as u32, game.round_name() as u16 as u32, weight);
        }
        0 => {
            let weight = scale_by_round(0x32, 0x64, game.round_number(), 2);
            let knockout = Cup(read_usize(cup.stages() + 8));
            let next = read_i16(knockout.round_table() + (game.round_number() as isize as usize).wrapping_mul(cup_round::SIZE) + cup_round::SIZE + cup_round::ROUND_CODE);
            european_cup_add_qualified_record(club, comp, game.round_prefix() as u16 as u32, next as u16 as u32, weight);
        }
        1 => {
            covered!("cx01_13 records: champions cup winner");
            african_nations_record_final_result(0x86a800, club, fixture, extra); // Staff_Records::add_cup_winners_record
            set_bra_champ_cup_winner(read_usize(G_CONMEBOL_SEEDING), club);
        }
        2 => african_nations_record_final_result(0x86b0b0, club, fixture, extra), // Staff_Records::add_cup_runners_up_record
        _ => {}
    }
    0
}

/// A club's position code in the Champions Cup statistics from a raw stage code: in the preliminary 1 and 2 give
/// `first_two`, 3 gives 9; in the groups 1 and 2 give `first_two`, 3 gives 5 and 4 gives 7; 0 and the other codes stay.
fn bra_champ_cup_position_code(preliminary: bool, code: u8, first_two: u8) -> u8 {
    match (preliminary, code) {
        (_, 1 | 2) => first_two,
        (true, 3) => 9,
        (false, 3) => 5,
        (false, 4) => 7,
        _ => code,
    }
}

/// Stores a club's current, best and worst position codes in the competition statistics (the club's entry,
/// `exe::comp_stats_find_club_entry`, not checked), turning the raw stage codes into positions: in the preliminary
/// (stage 0xff) the current and best codes 1 and 2 become 1 and the worst 7, a 3 becomes 9; in the groups (stages
/// below 2, signed) the current and best codes 1 and 2 become 1 and the worst 3, a 3 becomes 5 and a 4 becomes 7; from
/// stage 2 on the codes are stored as they are.
/// Original: `BraChampCup::set_club_progress` (0x426c20, bra_champ_cup.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn bra_champ_cup_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let entry = CompStatsClub(exe::comp_stats_find_club_entry(League(this).comp_stats(), club));
    let stage = stage as u8;
    let (current, best, worst) = (current as u8, best as u8, worst as u8);
    let codes = if stage == 0xff {
        covered!("cx01_13 progress in the preliminary");
        (bra_champ_cup_position_code(true, current, 1), bra_champ_cup_position_code(true, best, 1), bra_champ_cup_position_code(true, worst, 7))
    } else if (stage as i8) < 2 {
        covered!("cx01_13 progress in the groups");
        (bra_champ_cup_position_code(false, current, 1), bra_champ_cup_position_code(false, best, 1), bra_champ_cup_position_code(false, worst, 3))
    } else {
        (current, best, worst)
    };
    entry.set_best_position(codes.1);
    entry.set_current_position(codes.0);
    entry.set_worst_position(codes.2);
}

/// Writes the news text of a club's fate in the Champions Cup into Text `text`.
///
/// Same arguments as `comp_build_fate_news_text` (club, outcome, stage, the stage's name id, fixture, next stage, _,
/// verdict, body, text; thiscall, ret 0x28). In the preliminary (stage 0xff) and the groups (other stages below 2):
/// through (outcome 6) the body is the group qualification line (`exe::comp_util_write_group_qualify_news(club, stage,
/// competition, 6, body, text)`) and the headline "through to group phase" (from the preliminary) or "through to semi
/// final" (from the groups) with the club, linked; out (0xfe) the group line with the stage name id instead of the
/// stage (`exe::comp_util_write_group_qualify_news(club, name, competition, −2, body, text)`); other outcomes nothing.
/// From stage 2 on, the shared cup lines: the tie result in the body (`exe::comp_util_write_cup_tie_result_news`), the
/// win or defeat in the headline (`exe::comp_util_write_cup_win_or_defeat_news`).
/// Defect of the original [inference]: the qualified line passes the stage index where the out line passes the stage's
/// name id.
/// Original: `BraChampCup::build_fate_news_text` (0x425fd0, bra_champ_cup.cpp, thiscall, ret 0x28).
pub(crate) unsafe extern "thiscall" fn bra_champ_cup_build_fate_news_text(this: usize, club: i32, outcome: u32, stage: u32, name: u32, fixture: usize, next_stage: u32,
    _a7: u32, verdict: u32, body: u32, text: usize) {
    let comp = League(this).competition();
    let s = stage as u8;
    if s == 0xff || (s as i8) < 2 {
        match outcome as u8 {
            6 if body != 0 => {
                covered!("cx01_13 fate: group qualified");
                exe::comp_util_write_group_qualify_news(club as u32, stage, comp, 6, body, text);
            }
            6 => {
                covered!("cx01_13 fate: through headline");
                let lang_sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
                let c = club_at(club as isize);
                let cg = c.short_gender_name() as i8 as i32;
                let team = c.addr() + club::SHORT_NAME;
                // "through to group phase" from the preliminary, "through to semi final" from the groups
                lang_sprintf(0xde1f64, if s == 0xff { 0x98a25c } else { 0x98a224 }, cg, cg, team);
                exe::text_set_string(text, 0xde1f64 as *const u8);
                exe::text_add_link(text, team as *const u8, 0x7d5, c);
            }
            0xfe => {
                covered!("cx01_13 fate: group out");
                exe::comp_util_write_group_qualify_news(club as u32, name, comp, -2, body, text);
            }
            _ => {}
        }
        return;
    }
    if body != 0 {
        exe::comp_util_write_cup_tie_result_news(club as u32, fixture as u32, verdict, outcome, next_stage, text);
    } else {
        exe::comp_util_write_cup_win_or_defeat_news(club as u32, fixture as u32, outcome, text);
    }
}

/// Comparer for `bra_champ_cup_update_staff_records_for_status`: the records go to the managers' history and the
/// winner to the CONMEBOL seeding (they cannot run twice); only the original runs and it must answer 0.
pub(crate) unsafe extern "thiscall" fn compare_bra_champ_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, a4: u32, fixture: usize, extra: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(IDX_BRA_CHAMP_CUP_UPDATE_STAFF_RECORDS_FOR_STATUS)(this, club, outcome, stage, a4, fixture, extra);
    SCOREBOARD.record(r == 0, || format!("bra_champ_cup_update_staff_records_for_status({:#x}, club {:#x}, {}, stage {}): returned {}", this, club, outcome as u8, stage as u8 as i8, r));
    r
}

/// Comparer for `bra_champ_cup_set_club_progress`: the three position bytes of the club's entry side by side.
pub(crate) unsafe extern "thiscall" fn compare_bra_champ_cup_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(IDX_BRA_CHAMP_CUP_SET_CLUB_PROGRESS);
    let stats = League(this).comp_stats();
    let entry = if stats != 0 { exe::comp_stats_find_club_entry(stats, club) } else { 0 };
    if entry == 0 {
        return o(this, club, stage, current, best, worst);
    }
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&[(entry + comp_stats_club::CURRENT_POSITION, 3)], || o(this, club, stage, current, best, worst),
        || bra_champ_cup_set_club_progress(this, club, stage, current, best, worst));
    SCOREBOARD.record(same, || format!("bra_champ_cup_set_club_progress({:#x}, club {:#x}, stage {}, {}, {}, {}): original {:x?} rust {:x?}", this, club, stage as u8 as i8,
        current as u8, best as u8, worst as u8, mo, mr));
}

/// Comparer for `bra_champ_cup_build_fate_news_text` (`compare_text_writer`).
pub(crate) unsafe extern "thiscall" fn compare_bra_champ_cup_build_fate_news_text(this: usize, club: i32, outcome: u32, stage: u32, name: u32, fixture: usize, next_stage: u32,
    a7: u32, verdict: u32, body: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32, u32, u32, u32, usize, u32, u32, u32, u32, usize)>(IDX_BRA_CHAMP_CUP_BUILD_FATE_NEWS_TEXT);
    compare_text_writer(text, || o(this, club, outcome, stage, name, fixture, next_stage, a7, verdict, body, text),
        |t| bra_champ_cup_build_fate_news_text(this, club, outcome, stage, name, fixture, next_stage, a7, verdict, body, t),
        || format!("bra_champ_cup_build_fate_news_text({:#x}, club {}, outcome {}, stage {}, body {})", this, club, outcome as u8, stage as u8 as i8, body));
}

// ---------- sbD_6: the Brazilian Cup and Champions Cup entrants (bra_nat_first.cpp) ----------

/// The twelve state leagues in the order the Brazilian Cup reads them, with how many clubs each may send besides its
/// champion: São Paulo, Rio, Minas Gerais, Paraná, Rio Grande do Sul, Goiás, Pernambuco and Bahia 5; Northeast 7;
/// Santa Catarina 5; Centre-West and North 7.
pub(crate) const BRAZILIAN_CUP_STATE_LEAGUES: [(usize, u8); 12] = [
    (KEY_CLUB_COMP_BRAZILIAN_SAO_PAULO_STATE_CHAMP, 5),
    (KEY_CLUB_COMP_BRAZILIAN_RIO_DE_JANEIRO_STATE_CHAMPIONS, 5),
    (KEY_CLUB_COMP_BRAZILIAN_MINAS_GERAIS_STATE_CHAMPIONSHI, 5),
    (KEY_CLUB_COMP_BRAZILIAN_PARANA_STATE_CHAMPIONSHIP, 5),
    (KEY_CLUB_COMP_BRAZILIAN_GAUCHO_STATE_CHAMPIONSHIP, 5),
    (KEY_CLUB_COMP_BRAZILIAN_GOIAS_STATE_CHAMPIONSHIP, 5),
    (KEY_CLUB_COMP_BRAZILIAN_PERNAMBUCO_STATE_CHAMPIONSHIP, 5),
    (KEY_CLUB_COMP_BRAZILIAN_BAHIA_STATE_CHAMPIONSHIP, 5),
    (KEY_CLUB_COMP_BRAZILIAN_NORTHEAST_STATE_CHAMPIONSHIP, 7),
    (KEY_CLUB_COMP_BRAZILIAN_SANTA_CATARINA_STATE_CHAMPIONS, 5),
    (KEY_CLUB_COMP_BRAZILIAN_CENTRAL_STATE_CHAMPIONSHIP, 7),
    (KEY_CLUB_COMP_BRAZILIAN_NORTH_STATE_CHAMPIONSHIP, 7),
];

/// The last entrant slot of the Brazilian Cup: the slots are filled from 70 down to 0.
const BRAZILIAN_CUP_LAST_SLOT: i16 = 0x46;

/// Writes `club` into entrant slot `slot` (6-byte entries) of a competition's entrant book; a negative slot writes
/// before the book, as the original's arithmetic does.
unsafe fn write_entrant_slot(comp: League, slot: i16, club: u32) {
    write_u32(comp.entrant_book().wrapping_add((slot as isize as usize).wrapping_mul(6)), club);
}

/// Fills the Brazilian Cup entrant book at the Brazilian turnover (`this` is not read). Left out: the champion and the
/// runner-up of Series A, the Brazilian Cup winner and the Champions Cup winner (`exe::comp_util_get_latest_placed_club`
/// of their competition records). The slots go from 70 down to 0, each step one slot lower:
/// 1. each state league (`BRAZILIAN_CUP_STATE_LEAGUES` order) sends the first club of its table by position
///    (`exe::league_get_table_row`) that is not left out; a league with none still uses up its slot;
/// 2. Series A sends its clubs from the third row of its table on (`league::TABLE`, rows of 0x3b bytes), skipping the
///    clubs left out and those already in a higher slot;
/// 3. then, row by row from the second to the tenth, each state league in turn sends the club of that row of its table,
///    if it has not yet sent its quota, the club is not left out nor already in.
/// It stops when the slots run out. Defect of the original: step 3 starts without checking the slot, so it would write
/// one club before the book if steps 1 and 2 had used up all the slots (not with the real leagues).
/// Original: `bra_nat_first_fill_brazilian_cup_entrants` (0x42b760, bra_nat_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_nat_first_fill_brazilian_cup_entrants(_this: usize) {
    let cup = League(comp_by_global(KEY_CLUB_COMP_BRAZILIAN_CUP));
    let serie_a = League(comp_by_global(KEY_CLUB_COMP_BRAZILIAN_NATIONAL_FIRST_DIVISION));
    let leagues: [League; 12] = core::array::from_fn(|k| League(comp_by_global(BRAZILIAN_CUP_STATE_LEAGUES[k].0)));
    let champions_cup = read_usize(G_CLUB_COMP_TABLE).wrapping_add(read_usize(KEY_CLUB_COMP_BRAZILIAN_CHAMPIONS_CUP).wrapping_mul(0x6b));
    let left_out = [
        exe::comp_util_get_latest_placed_club(serie_a.competition(), 0) as u32,
        exe::comp_util_get_latest_placed_club(serie_a.competition(), 1) as u32,
        exe::comp_util_get_latest_placed_club(cup.competition(), 0) as u32,
        exe::comp_util_get_latest_placed_club(champions_cup, 0) as u32,
    ];
    let already_in = |club: u32, slot: i16| (slot + 1..=BRAZILIAN_CUP_LAST_SLOT).any(|s| read_u32(cup.entrant_book() + s as usize * 6) == club);
    let table_club = |league: League, row: i8| read_u32(league.table().wrapping_add((row as isize as usize).wrapping_mul(0x3b)));
    let mut slot = BRAZILIAN_CUP_LAST_SLOT;
    covered!("sbD_6 brazilian cup entrants");
    for &league in &leagues {
        if slot < 0 {
            break;
        }
        let champion = (0..league.club_count()).map(|r| read_u32(exe::league_get_table_row(league, r as i32))).find(|c| !left_out.contains(c));
        if let Some(club) = champion {
            write_entrant_slot(cup, slot, club);
        }
        slot -= 1;
    }
    let mut row: i8 = 2;
    if serie_a.club_count() > 2 {
        while slot >= 0 {
            let club = table_club(serie_a, row);
            if !already_in(club, slot) && !left_out.contains(&club) {
                write_entrant_slot(cup, slot, club);
                slot -= 1;
            }
            row = row.wrapping_add(1);
            if row as i16 >= serie_a.club_count() {
                break;
            }
        }
    }
    let mut sent = [0u8; 12];
    for row in 1..10i8 {
        for (k, &league) in leagues.iter().enumerate() {
            if sent[k] as i8 >= BRAZILIAN_CUP_STATE_LEAGUES[k].1 as i8 || row as i16 >= league.club_count() {
                continue;
            }
            let club = table_club(league, row);
            if left_out.contains(&club) || already_in(club, slot) {
                continue;
            }
            write_entrant_slot(cup, slot, club);
            slot -= 1;
            if slot < 0 {
                return;
            }
            sent[k] += 1;
        }
        if slot < 0 {
            return;
        }
    }
}

/// BraChampCup's season set-up (bra_champ_cup.cpp, thiscall, ret 0): virtual `comp_vtable::SET_PREDICTED_POSITIONS`
/// (rank), the table, the fixtures and the order (each with its error box), then `league::ACTIVE` = 1.
pub(crate) const FN_BRA_CHAMP_CUP_SET_UP: usize = 0x427320;

/// The twelve state leagues in the order the Champions Cup reads their champions: São Paulo, Rio, Minas Gerais, Paraná,
/// Centre-West, Pernambuco, Bahia, Santa Catarina, Goiás, North, Northeast and Rio Grande do Sul.
pub(crate) const CHAMPIONS_CUP_STATE_LEAGUES: [usize; 12] = [
    KEY_CLUB_COMP_BRAZILIAN_SAO_PAULO_STATE_CHAMP,
    KEY_CLUB_COMP_BRAZILIAN_RIO_DE_JANEIRO_STATE_CHAMPIONS,
    KEY_CLUB_COMP_BRAZILIAN_MINAS_GERAIS_STATE_CHAMPIONSHI,
    KEY_CLUB_COMP_BRAZILIAN_PARANA_STATE_CHAMPIONSHIP,
    KEY_CLUB_COMP_BRAZILIAN_CENTRAL_STATE_CHAMPIONSHIP,
    KEY_CLUB_COMP_BRAZILIAN_PERNAMBUCO_STATE_CHAMPIONSHIP,
    KEY_CLUB_COMP_BRAZILIAN_BAHIA_STATE_CHAMPIONSHIP,
    KEY_CLUB_COMP_BRAZILIAN_SANTA_CATARINA_STATE_CHAMPIONS,
    KEY_CLUB_COMP_BRAZILIAN_GOIAS_STATE_CHAMPIONSHIP,
    KEY_CLUB_COMP_BRAZILIAN_NORTH_STATE_CHAMPIONSHIP,
    KEY_CLUB_COMP_BRAZILIAN_NORTHEAST_STATE_CHAMPIONSHIP,
    KEY_CLUB_COMP_BRAZILIAN_GAUCHO_STATE_CHAMPIONSHIP,
];

/// The champion of a state league and his points per game (`f32`, as the original stores them): the table row whose
/// club has the best final position (`comp_stats_club::CURRENT_POSITION` of its statistics entry,
/// `exe::comp_stats_find_club_entry` on the league's statistics) below 100, the first of equals. With no such row the
/// original leaves both unset (stack garbage); the copy gives (0, 0.0).
unsafe fn state_league_champion(league: League) -> (u32, f32) {
    let (mut best, mut champion) = (100u16, (0u32, 0f32));
    for r in 0..league.club_count().max(0) as usize {
        let row = league.table() + r * 0x3b;
        let entry = exe::comp_stats_find_club_entry(league.comp_stats(), read_usize(row));
        if entry != 0 && (read_u8(entry + comp_stats_club::CURRENT_POSITION) as u16) < best {
            best = read_u8(entry + comp_stats_club::CURRENT_POSITION) as u16;
            let points = read_i16(row + league_table_row::POINTS) as f64;
            let played = read_i8(row + league_table_row::PLAYED) as f64;
            champion = (read_u32(row), (points / played) as f32);
        }
    }
    champion
}

/// The nine Champions Cup entrants as (slot, club), in the order the original writes them, or None while a state
/// league is not complete (virtual `comp_vtable::CHECK_COMPLETE`(1, 0)) or the cup is already active
/// (`league::ACTIVE`). Slots 3 to 7: the champions of Rio, São Paulo, Minas Gerais, Rio Grande do Sul and Paraná;
/// slot 0: the better of the North and Northeast champions by points per game (the Northeast on a tie) if strictly
/// better than the Centre-West champion, otherwise the Centre-West one; slot 1: the Goiás champion if strictly better
/// than the Santa Catarina one, otherwise the latter; slots 8 and 2: the Bahia and Pernambuco champions, the strictly
/// better of the two in slot 8.
pub(crate) unsafe fn champions_cup_entrants() -> Option<[(usize, u32); 9]> {
    let leagues: [League; 12] = core::array::from_fn(|k| League(comp_by_global(CHAMPIONS_CUP_STATE_LEAGUES[k])));
    for league in leagues {
        let complete: unsafe extern "thiscall" fn(usize, u32, u32) -> u8 = core::mem::transmute(read_usize(read_usize(league.addr()) + comp_vtable::CHECK_COMPLETE));
        if complete(league.addr(), 1, 0) == 0 {
            return None;
        }
    }
    if League(comp_by_global(KEY_CLUB_COMP_BRAZILIAN_CHAMPIONS_CUP)).active() != 0 {
        return None;
    }
    let c: [(u32, f32); 12] = core::array::from_fn(|k| state_league_champion(leagues[k]));
    let (sao_paulo, rio, minas, parana, centre_west, pernambuco, bahia, santa_catarina, goias, north, northeast, gaucho) =
        (c[0], c[1], c[2], c[3], c[4], c[5], c[6], c[7], c[8], c[9], c[10], c[11]);
    let north_best = if north.1 > northeast.1 { north } else { northeast };
    let first = if north_best.1 > centre_west.1 { north_best.0 } else { centre_west.0 };
    let second = if goias.1 > santa_catarina.1 { goias.0 } else { santa_catarina.0 };
    let (better, other) = if bahia.1 > pernambuco.1 { (bahia.0, pernambuco.0) } else { (pernambuco.0, bahia.0) };
    Some([(3, rio.0), (4, sao_paulo.0), (5, minas.0), (6, gaucho.0), (7, parana.0), (0, first), (1, second), (8, better), (2, other)])
}

/// Puts the state champions into the Champions Cup entrant book once all twelve state leagues are complete
/// (`champions_cup_entrants`), then sets the cup up (`FN_BRA_CHAMP_CUP_SET_UP`: rank, table, fixtures; active = 1).
/// Defect of the original: a state league with no club of final position below 100 leaves its champion and ratio as
/// stack garbage.
/// Original: `bra_nat_first_fill_champions_cup_entrants` (0x42bba0, bra_nat_first.cpp, cdecl).
pub(crate) unsafe extern "C" fn bra_nat_first_fill_champions_cup_entrants() {
    let Some(entrants) = champions_cup_entrants() else {
        return;
    };
    covered!("sbD_6 champions cup entrants");
    let cup = League(comp_by_global(KEY_CLUB_COMP_BRAZILIAN_CHAMPIONS_CUP));
    for (slot, club) in entrants {
        write_u32(cup.entrant_book() + slot * 6, club);
    }
    let set_up: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_BRA_CHAMP_CUP_SET_UP);
    set_up(cup.addr());
}

/// Comparer for `bra_nat_first_fill_brazilian_cup_entrants`: side by side over the 71 entrant slots of the Brazilian
/// Cup (the routine only writes there).
pub(crate) unsafe extern "thiscall" fn compare_bra_nat_first_fill_brazilian_cup_entrants(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_NAT_FIRST_FILL_BRAZILIAN_CUP_ENTRANTS);
    let book = League(comp_by_global(KEY_CLUB_COMP_BRAZILIAN_CUP)).entrant_book();
    let reg = [(book, (BRAZILIAN_CUP_LAST_SLOT as usize + 1) * 6)];
    let (_, _, same, a, b) = crate::comparison::side_by_side(&reg, || o(this), || bra_nat_first_fill_brazilian_cup_entrants(this));
    SCOREBOARD.record(same, || format!("bra_nat_first_fill_brazilian_cup_entrants: entrant slots {:?} x {:?}", a, b));
}

/// Comparer for `bra_nat_first_fill_champions_cup_entrants`: only the original runs (it sets the cup up); the copy's
/// entrants, worked out before, must be the nine slots the original wrote (nothing when the copy expects nothing).
pub(crate) unsafe extern "C" fn compare_bra_nat_first_fill_champions_cup_entrants() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_BRA_NAT_FIRST_FILL_CHAMPIONS_CUP_ENTRANTS);
    let plan = champions_cup_entrants();
    let book = League(comp_by_global(KEY_CLUB_COMP_BRAZILIAN_CHAMPIONS_CUP)).entrant_book();
    let before: [u32; 9] = core::array::from_fn(|s| if book == 0 { 0 } else { read_u32(book + s * 6) });
    o();
    let after: [u32; 9] = core::array::from_fn(|s| if book == 0 { 0 } else { read_u32(book + s * 6) });
    let expected = match plan {
        Some(e) => {
            let mut v = before;
            for (s, c) in e {
                v[s] = c;
            }
            v
        }
        None => before,
    };
    SCOREBOARD.record(after == expected, || format!("bra_nat_first_fill_champions_cup_entrants: original {:x?} copy {:x?}", after, expected));
}

// ---------- sbG_1: filling a mini league from the fixtures its clubs played among themselves (sub_league.cpp) ----------

/// The size of the SubLeague object (operator new 0xee before SubLeague::SubLeague at 0x46f4db).
pub(crate) const SUB_LEAGUE_SIZE: usize = 0xee;

/// Fills a mini league's table from the played matches between its own clubs.
///
/// The fixtures come from `list` or, when it is null, from FIXTURE_MANAGER::scan_fixtures (`FN_FIXTURE_MANAGER_SCAN_FIXTURES`
/// on [`G_FIXTURE_MANAGER`], 18 arguments, into a local LLIST: from the day and year of `from`, forwards, competition
/// `comp_id`, rounds `round_min`..`round_max` (both −1 when `round_min` is −1, with the three unread arguments 0 instead of
/// −1), any clubs, at most `max`, played only (1), stage −2, either side, without strangers). The list cursor goes back to
/// its head; for each fixture, when `to` is given, one dated after it (Date::Date `exe::new_date`(day, year) of the
/// fixture; the year compared unsigned, the day signed) is skipped; an unplayed one (`fixture::HOME_GOALS` = 0xff) too.
/// Otherwise the table (`league_table_row::SIZE`-byte rows, `club_count()`) is searched for the home and the away club
/// (`fixture::HOME_CLUB`, `fixture::AWAY_CLUB`), stopping when both are found; with both, the match goes into the table
/// through virtual `comp_vtable::ADD_RESULT`(fixture, 0). The local list is destroyed; returns 1.
/// Original: `SubLeague::add_results_between_members` (0x88c9f0, sub_league.cpp, thiscall, ret 0x1c).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn sub_league_add_results_between_members(this: usize, round_min: i32, round_max: i32, from: usize, to: usize, comp_id: u32,
    max: u32, list: usize) -> u32 {
    let league = League(this);
    let mut own = [0u8; 0x10];
    let own = own.as_mut_ptr() as usize;
    exe::new_llist(own);
    let list = if list != 0 {
        list
    } else {
        covered!("sbG_1 sub league scan");
        // simplification: the original passes dwords with stack leftovers above the words; scan_fixtures only reads the words
        let (mut year, mut day, mut slice) = (read_i16(from + 2) as u16 as u32, read_i16(from) as u16 as u32, 0u8);
        let (round_min, round_max, unread) = if round_min == -1 { (-1, -1, 0) } else { (round_min, round_max, -1) };
        let scan: unsafe extern "thiscall" fn(usize, *mut u32, *mut u32, *mut u8, u32, u32, i32, i32, i32, i32, i32, i32, i32, u32, usize, u32, i32, i32, u32) -> u32 =
            core::mem::transmute(FN_FIXTURE_MANAGER_SCAN_FIXTURES);
        scan(read_usize(G_FIXTURE_MANAGER), &mut year, &mut day, &mut slice, 1, comp_id, round_min, round_max, -1, -1, unread, unread, unread, max, own, 1, -2, -1, 0);
        own
    };
    write_usize(list + llist::CURSOR, read_usize(list + llist::HEAD));
    let mut f = exe::llist_get_next(list);
    while f != 0 {
        let mut date = [0u8; 8];
        let after = to != 0 && {
            exe::new_date(date.as_mut_ptr() as usize, read_i16(f + fixture::DAY) as u16 as u32, read_i16(f + fixture::YEAR) as u16 as u32);
            let (day, year) = (i16::from_le_bytes([date[0], date[1]]), u16::from_le_bytes([date[2], date[3]]));
            year > read_u16(to + 2) || (year == read_u16(to + 2) && day > read_i16(to))
        };
        if !after && read_u8(f + fixture::HOME_GOALS) != 0xff {
            let (mut home, mut away) = (false, false);
            let mut i = 0i32;
            while i < league.club_count() as i32 {
                let club = read_usize(league.table() + i as usize * league_table_row::SIZE);
                home |= club == read_usize(f + fixture::HOME_CLUB);
                away |= club == read_usize(f + fixture::AWAY_CLUB);
                if home && away {
                    break;
                }
                i += 1;
            }
            if home && away { // sbG_1: both clubs in the table
                covered!("sbG_1 sub league result added");
                let add: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::ADD_RESULT));
                add(this, f, 0);
            }
        }
        f = exe::llist_get_next(list);
    }
    exe::drop_llist(own);
    1
}

/// Comparer for `sub_league_add_results_between_members`: side by side. Before the original fills `this`, a scratch
/// copy of the object (`SUB_LEAGUE_SIZE` bytes, `exe::crt_malloc`) gets its own copy of the table (League::copy_table
/// `exe::league_copy_table`, with new head-to-head and form buffers; the `league::GAMES_PER_CLUB` it may set is put
/// back on `this`); the copy fills the scratch object; the counts, points and rows (`sub_league_bytes`) and the return
/// must be equal. The scratch table (`exe::league_free_table`) and object are freed.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_sub_league_add_results_between_members(this: usize, round_min: i32, round_max: i32, from: usize, to: usize, comp_id: u32,
    max: u32, list: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32, i32, usize, usize, u32, u32, usize) -> u32>(IDX_SUB_LEAGUE_ADD_RESULTS_BETWEEN_MEMBERS);
    let league = League(this);
    let scratch = if league.table() != 0 && league.club_count() > 0 { exe::crt_malloc(SUB_LEAGUE_SIZE as u32) } else { 0 };
    if scratch == 0 {
        return o(this, round_min, round_max, from, to, comp_id, max, list);
    }
    core::ptr::copy_nonoverlapping(this as *const u8, scratch as *mut u8, SUB_LEAGUE_SIZE);
    let games = league.games_per_club();
    League(scratch).set_table(exe::league_copy_table(this));
    league.set_games_per_club(games);
    let r = o(this, round_min, round_max, from, to, comp_id, max, list);
    let c = sub_league_add_results_between_members(scratch, round_min, round_max, from, to, comp_id, max, list);
    let (a, b) = (sub_league_bytes(this), sub_league_bytes(scratch));
    if League(scratch).table() != 0 {
        exe::league_free_table(League(scratch), 0);
    }
    exe::crt_free(scratch);
    SCOREBOARD.record(r == c && a == b, || format!("sub_league_add_results_between_members({:#x}, rounds {}..{}, comp {}, list {:#x}): original {} rust {}, the tables differ: {}",
        this, round_min, round_max, comp_id as i32, list, r, c, a != b));
    r
}

// ---------- cx03_19: the constructors of the Copa Libertadores and the Copa Mercosur, and the Mercosur rules (conmebol_liber.cpp, conmebol_merc.cpp) ----------

/// ConmebolLiber::set_up_rules (thiscall, ret 0; 0 = failure).
const FN_CONMEBOLLIBER_SET_UP_RULES: usize = 0x4c3890;

/// What differs between the two South American cups' constructors: the vtable, the stage count, whether the
/// Conmebol seeding moves to the new season first, the rules routine and the source file with the box lines of the
/// rules and the statistics.
pub(crate) struct SouthAmericanCup {
    pub(crate) vtable: usize,
    pub(crate) stages: usize,
    pub(crate) seeding: bool,
    pub(crate) rules: usize,
    pub(crate) file: usize,
    pub(crate) rows: [u32; 2],
}

/// The Copa Libertadores: 8 stages, the seeding moved on, the boxes of lines 0x52 and 0x60.
pub(crate) const COPA_LIBERTADORES: SouthAmericanCup = SouthAmericanCup {
    vtable: 0x968bf4, stages: 8, seeding: true, rules: FN_CONMEBOLLIBER_SET_UP_RULES, file: 0x9a7de4, rows: [0x52, 0x60],
};

/// The Copa Mercosur: 5 stages, the boxes of lines 0x4a and 0x58.
pub(crate) const COPA_MERCOSUR: SouthAmericanCup = SouthAmericanCup {
    vtable: 0x968ca8, stages: 5, seeding: false, rules: 0x4c4f50, file: 0x9a7e5c, rows: [0x4a, 0x58],
};

/// The constructor of the two South American cups (`Libertadores(year, record)`, `Mercosur(year, record)`), run once, at
/// career creation; the same body in both (checked instruction by instruction), `SouthAmericanCup` sets them apart.
///
/// On the League base (`exe::new_league`, thiscall, ret 0): the competition record (`league::COMPETITION`), the vtable,
/// the year (`set_year`), `discipline_rule` 2 (the South American rules), the drawn club list (+0xee) = 0 and
/// `importance` 0xf. If the saved state loads (`exe::league_load_from_temp_file`: push 1 -> thiscall, ret 4; nonzero),
/// returns this. On a new cup, in order: `league_flags` 0x28, `stage_index` 0xff, `current_stage` −1, the stage count
/// and the stage vector (`exe::crt_malloc` of 4 per stage, unchecked); in the Libertadores the Conmebol seeding's new
/// season (ConmebolSeeding::update_for_new_season on `G_CONMEBOL_SEEDING` with 0, thiscall, ret 4); the rules (thiscall,
/// ret 0; 0: the box of `rows[0]` and this); `club_count` 0, `table` 0 and `active` 0; the statistics
/// (`create_comp_stats_at_8`; without them the box of `rows[1]`). Returns this.
pub(crate) unsafe fn build_south_american_cup(this: League, year: u32, record: usize, cup: &SouthAmericanCup, branch: &'static str) -> usize {
    exe::new_league(this);
    write_u32(this.addr() + league::COMPETITION, record as u32);
    write_u32(this.addr(), cup.vtable as u32);
    this.set_year(year as u16 as i16);
    this.set_discipline_rule(2);
    write_u32(this.addr() + conmebol_liber::DRAWN_CLUBS, 0);
    this.set_importance(0xf);
    if exe::league_load_from_temp_file(this, 1) != 0 {
        return this.addr();
    }
    covered!(branch);
    this.set_league_flags(0x28);
    this.set_stage_index(stage_index::MAIN);
    this.set_current_stage(0xffff_ffff);
    this.set_stage_count(cup.stages);
    this.set_stages(exe::crt_malloc(cup.stages as u32 * 4));
    if cup.seeding {
        let seed: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_CONMEBOLSEEDING_UPDATE_FOR_NEW_SEASON); // return unused
        seed(read_usize(G_CONMEBOL_SEEDING), 0);
    }
    let rules: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(cup.rules);
    if rules(this.addr()) == 0 {
        show_error_at(cup.file, cup.rows[0]);
        return this.addr();
    }
    this.set_club_count(0);
    this.set_table(0);
    this.set_active(0);
    if !create_comp_stats_at_8(this.addr()) {
        show_error_at(cup.file, cup.rows[1]);
    }
    this.addr()
}

/// The Copa Libertadores at career creation: discipline rule 2 and importance 15; a new cup has eight stages, the
/// Conmebol seeding of the new season, the rules and the statistics (`build_south_american_cup`).
/// Original: `ConmebolLiber::ConmebolLiber` (0x4c2f50, conmebol_liber.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_conmebol_liber(this: usize, year: u32, record: usize) -> usize {
    build_south_american_cup(League(this), year, record, &COPA_LIBERTADORES, "cx03_19 libertadores new")
}

/// The Copa Mercosur at career creation: as the Libertadores, with five stages and no seeding step
/// (`build_south_american_cup`).
/// Original: `ConmebolMerc::ConmebolMerc` (0x4c46c0, conmebol_merc.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_conmebol_merc(this: usize, year: u32, record: usize) -> usize {
    build_south_american_cup(League(this), year, record, &COPA_MERCOSUR, "cx03_19 mercosur new")
}

/// The Copa Mercosur rules: the Merconorte's fields with 7 substitutes named and 3 substitutions allowed, one qualifier
/// per group, the start on day 15 of month 5 [inference: 15 June] and the prize fund of 2,000,000.
pub(crate) const MERCOSUR_RULES: ContinentalGroupRules = ContinentalGroupRules {
    fields: &[(0xc2, 1, 3), (0xc7, 1, 3), (0x4a, 1, 3), (0x3c, 2, 2), (0xc6, 1, 2), (0x42, 1, 2), (0xc3, 1, 1), (0xc4, 1, 7), (0xc5, 1, 1),
        (0xbe, 1, 1), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 0), (0x1c, 4, 0xffff_ffff), (0x20, 4, 0xffff_ffff), (0x49, 1, 7)],
    day: 0xf, month: 5, week: 0xffff_ffff, prize: Some(0x1e8480),
};

/// rule: the Copa Mercosur groups are home and away, 3 points a win and 1 a draw, tiebreak by codes 1, 2 and 3, one
/// club going through directly from each group, clubs from several nations, category 7, 7 substitutes named and 3
/// changes; the season starts on day 15 of month 5 of the competition year [inference: 15 June]; prize fund of 2,000,000
/// (`apply_cup_rules` with `MERCOSUR_RULES`: the fields, the start day word, its year offset 0, the prize and the
/// round table from virtual +0x3c into `league::ROUND_DATES`). Returns 1.
/// Original: `ConmebolMerc::set_up_rules` (0x4c4f50, conmebol_merc.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn conmebol_merc_set_up_rules(this: usize) -> u32 {
    covered!("cx03_19 mercosur rules");
    apply_cup_rules(League(this), &MERCOSUR_RULES, MERCOSUR_RULES.day, 3)
}

/// The fixed fields the South American cups' constructors leave on a new cup, for the comparers.
pub(crate) const fn south_american_cup_ctor_fields(stages: u32) -> [(usize, u8, u32); 10] {
    [(league::STAGE_COUNT, 4, stages), (league::CURRENT_STAGE, 4, 0xffff_ffff), (league::CLUB_COUNT, 2, 0), (league::STAGE_INDEX, 1, 0xff), (league::ACTIVE, 4, 0),
        (league::DISCIPLINE_RULE, 1, 2), (league::IMPORTANCE, 1, 0xf), (league::TABLE, 4, 0), (league::LEAGUE_FLAGS, 2, 0x28), (conmebol_liber::DRAWN_CLUBS, 4, 0)]
}

/// Comparer for `construct_conmebol_liber`: only the original runs (it allocates and reads the save file), then the
/// plan is checked (`cup_constructor_plan_mismatch`, without the round table).
pub(crate) unsafe extern "thiscall" fn compare_construct_conmebol_liber(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_CONMEBOL_LIBER)(this, year, record);
    let error = cup_constructor_plan_mismatch(this, r, year, record, COPA_LIBERTADORES.vtable, &south_american_cup_ctor_fields(8), false);
    SCOREBOARD.record(error.is_none(), || format!("construct_conmebol_liber({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
    r
}

/// Comparer for `construct_conmebol_merc` (as above).
pub(crate) unsafe extern "thiscall" fn compare_construct_conmebol_merc(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_CONMEBOL_MERC)(this, year, record);
    let error = cup_constructor_plan_mismatch(this, r, year, record, COPA_MERCOSUR.vtable, &south_american_cup_ctor_fields(5), false);
    SCOREBOARD.record(error.is_none(), || format!("construct_conmebol_merc({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
    r
}

/// Comparer for `conmebol_merc_set_up_rules`: only the original runs (virtual +0x3c allocates the round table); the plan
/// checks the rules (`check_cup_rules`).
pub(crate) unsafe extern "thiscall" fn compare_conmebol_merc_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_CONMEBOL_MERC_SET_UP_RULES);
    check_cup_rules("conmebol_merc_set_up_rules", League(this), &MERCOSUR_RULES, || o(this))
}

// ---------- cx03_20: the squads of the MLS All-Star Game (usa_mls_all_stars.cpp) ----------

/// The 20 position masks of an All-Star squad, in the order `match_man_assign_players_to_positions` fills them.
pub(crate) const ALL_STAR_SQUAD_POSITIONS: [u16; 20] = [1, 1, 1, 0x804, 0x84, 4, 4, 0x204, 0x204, 0x204, 0x810, 0x90, 0x10, 0x10, 0x210, 0x210, 0x40, 0x40, 0x40, 0x40];

/// The players one All-Star selection can call up: for each of the first four rows of the tables, two passes; the
/// first over `first`'s row, the second over the same row of `second` when that club's All-Star conference
/// (`usa_mls_get_club_all_star_conference`) is `conference`; each player of the club's squad (`club.squad`, 50 slots)
/// with a player record, not out injured (`exe::injury_get_player_injury_status` ≠ 2), free of international duty on
/// `date` (`exe::nationalteams_get_international_duty_club`(…, player, date, 0, 0, 0) = 0) and still at that club
/// (`staff.club_job()`) joins `list`. A club met in both passes is collected twice, as in the original. Returns the count
/// (a signed word in the original).
unsafe fn collect_all_star_candidates(list: &mut [usize], first: League, second: League, conference: u16, date: &[u16; 4]) -> i16 {
    let mut n: i16 = 0;
    for row in 0..4usize {
        for pass in 0..2 {
            let table = if pass == 0 { first } else { second };
            let club = read_usize(table.table() + row * league_table_row::SIZE);
            if pass == 1 && usa_mls_get_club_all_star_conference(club) != conference {
                continue;
            }
            for slot in 0..50usize {
                let person = Staff(Club(club).squad(slot));
                if person.is_null() || person.player().is_null() {
                    continue;
                }
                if exe::injury_get_player_injury_status(read_usize(G_INJURY), person) == 2 {
                    continue;
                }
                if exe::nationalteams_get_international_duty_club(read_usize(G_NATIONAL_TEAMS), person, date.as_ptr() as usize, 0usize, 0, 0) != 0 {
                    continue;
                }
                if person.club_job().addr() == club {
                    list[n as usize] = person.addr();
                    n = n.wrapping_add(1);
                }
            }
        }
    }
    n
}

/// Fills one All-Star club's squad: the club (`club_at` of the id kept in `key`) loses its whole squad (the 50 slots
/// cleared); the candidates are sorted (`exe::crt_qsort`, 4-byte entries, `qsort_comparator_squad_by_player_0xb`
/// 0x5402f0); at most 127 are used, and fewer than 30 show the box of `row` (the squad is still filled); then the squad
/// is filled from them (`exe::match_man_assign_players_to_positions`(list, n, the 20 `ALL_STAR_SQUAD_POSITIONS`, 20, 0,
/// the club's squad, 2, then eight zeros), cdecl).
unsafe fn fill_all_star_squad(list: &mut [usize], n: i16, key: usize, row: u32) {
    let club = club_at(read_usize(key) as isize);
    for slot in 0..50usize {
        write_usize(club.addr() + 0xd7 + slot * 4, 0); // the squad slots
    }
    exe::crt_qsort(list.as_mut_ptr() as usize, n as i32 as u32, 4, 0x5402f0usize);
    let mut n = n;
    if n > 0x7f {
        n = 0x7f;
    } else if n < 0x1e {
        show_error_at(0xaa20a0, row);
    }
    exe::match_man_assign_players_to_positions(list.as_mut_ptr() as usize, n as i32 as u32, ALL_STAR_SQUAD_POSITIONS.as_ptr() as usize, 0x14, 0,
        club.addr() + 0xd7, 2, 0usize, 0, 0, 0, 0, 0, 0, 0);
}

/// rule: the two MLS All-Star selections are called up from the clubs in the first four places: the East from the
/// MLS's own table and from the first conference table's clubs of the East conference, the West from the second
/// conference table and from the first conference table's clubs of the West conference; each selection takes the fit
/// players free of international duty on the day of the game, sorted, at most 127 of them, and picks a squad of 20 by
/// position (`ALL_STAR_SQUAD_POSITIONS`).
///
/// The game day: the first round's date (`cup::ROUND_TABLE`: the day +3 and the year offset +5 on `cup::YEAR`;
/// Date::Date `exe::new_date`, thiscall, ret 8). The MLS (`G_COMP_OBJECT_TABLE`[`KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE`])
/// and its stages 0 and 1 (`cup::STAGES`). The East (`collect_all_star_candidates` over the MLS table and stage 0, the
/// conference 0x413; `fill_all_star_squad` into `G_KEY_CLUB_MLS_ALL_STARS_EAST`, the box of line 0x1dc), then the West
/// (over stage 1 and stage 0, 0x414; into `G_KEY_CLUB_MLS_ALL_STARS_WEST`, line 0x221). The original keeps both lists of
/// 300 on the stack one after the other: the copy's are larger, which only matters past 300 candidates.
/// Original: `UsaMlsAllStars::select_squads` (0x90a980, usa_mls_all_stars.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn usa_mls_all_stars_select_squads(this: usize) {
    let cup = Cup(this);
    let mls = League(read_usize(read_usize(G_COMP_OBJECT_TABLE) + read_usize(KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE) * 4));
    let first_stage = League(read_usize(mls.stages()));
    let second_stage = League(read_usize(mls.stages() + 4));
    let round = cup.round_table();
    let mut date = [0u16; 4];
    exe::new_date(date.as_mut_ptr() as usize, read_u16(round + cup_round::DAY) as u32,
        (read_u8(round + cup_round::YEAR_OFFSET) as i8 as i16).wrapping_add(cup.year()) as u16 as u32);
    covered!("cx03_20 all-star squads");
    let mut east = [0usize; 400];
    let n = collect_all_star_candidates(&mut east, mls, first_stage, 0x413, &date);
    fill_all_star_squad(&mut east, n, G_KEY_CLUB_MLS_ALL_STARS_EAST, 0x1dc);
    let mut west = [0usize; 400];
    let n = collect_all_star_candidates(&mut west, second_stage, first_stage, 0x414, &date);
    fill_all_star_squad(&mut west, n, G_KEY_CLUB_MLS_ALL_STARS_WEST, 0x221);
}

/// Comparer for `usa_mls_all_stars_select_squads`: side by side over the two All-Star clubs' squads (the 50 slots each);
/// the injury and duty checks and the position picker only read.
pub(crate) unsafe extern "thiscall" fn compare_usa_mls_all_stars_select_squads(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_USA_MLS_ALL_STARS_SELECT_SQUADS);
    let east = club_at(read_usize(G_KEY_CLUB_MLS_ALL_STARS_EAST) as isize).addr() + 0xd7;
    let west = club_at(read_usize(G_KEY_CLUB_MLS_ALL_STARS_WEST) as isize).addr() + 0xd7;
    let (_, _, same, mo, mc) = crate::comparison::side_by_side(&[(east, 200), (west, 200), (0xb67a34, 4)], || o(this), || usa_mls_all_stars_select_squads(this));
    SCOREBOARD.record(same, || format!("usa_mls_all_stars_select_squads({:#x}): original {:x?} copy {:x?}", this, mo, mc));
}

// ---------- cx04_5: the Oceania Club Championship's fate news text and club progress (oceania_club_champ.cpp) ----------

/// The final-stage fate text shared by the Oceania Club Championship and the Club World Championship: the template of
/// `finals` for the outcome, if any, with the club's and the competition's short names (`european_cup_fate_text`, the
/// competition record at +4); with no template for the outcome the original still hands the Text the shared buffer
/// 0xde1f64 as it is (stale text, a defect of the original) and adds the club's (0x7d5) and the competition's (0x7d0)
/// links.
unsafe fn final_stage_fate_text(this: usize, club_id: u32, outcome: u8, finals: &[(u8, usize)], text: usize) {
    let comp = read_usize(this + 4);
    if let Some(&(_, format)) = finals.iter().find(|f| f.0 == outcome) {
        return european_cup_fate_text(text, club_id, format, read_u8(comp + competition::SHORT_GENDER_NAME), comp + competition::SHORT_NAME, comp);
    }
    let club = club_at(club_id as isize);
    exe::text_set_string(text, PRAISE_TEXT_BUFFER as *const u8);
    exe::text_add_link(text, (club.addr() + club::SHORT_NAME) as *const u8, 0x7d5, club.addr());
    exe::text_add_link(text, (comp + competition::SHORT_NAME) as *const u8, 0x7d0, comp);
}

/// The fate text of a club that goes through its group (`format`: "{club} qualify for the semi final/final"), with the
/// club's short name (gender byte +0x52) and link (0x7d5).
unsafe fn group_qualify_text(club_id: u32, format: usize, text: usize) {
    covered!("cx04_5 group qualify text");
    let club = club_at(club_id as isize);
    let g = club.short_gender_name() as i8 as i32;
    story_lang!(PRAISE_TEXT_BUFFER, format, g, g, club.addr() + club::SHORT_NAME);
    exe::text_set_string(text, PRAISE_TEXT_BUFFER as *const u8);
    exe::text_add_link(text, (club.addr() + club::SHORT_NAME) as *const u8, 0x7d5, club.addr());
}

/// The fate news text of the two club championships run in groups and a final stage (Oceania, Club World): in the group
/// stages (signed stage below `groups`) outcome 6 (through) with `news` the group news
/// (`exe::comp_util_write_group_qualify_news`(club_id, a4, competition, 6, news, text)), without it `qualify`; outcome
/// 0xfe (out) the group news with −2; at stage `middle` (Oceania's 1) the base text
/// (`exe::comp_build_fate_news_text` with a7 = 0); at the final stage (`groups`), with `news` the cup tie result news
/// (`exe::comp_util_write_cup_tie_result_news`(club_id, fixture, a8, outcome, a6, text)), without it
/// `final_stage_fate_text`. Other stages and outcomes: nothing.
#[allow(clippy::too_many_arguments)]
unsafe fn group_championship_fate_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, fixture: usize, a6: u32, a8: u32, news: u32, text: usize,
        groups: i8, middle: Option<i8>, qualify: usize, finals: &[(u8, usize)]) {
    let (s, d) = (stage as u8 as i8, outcome as u8);
    let comp = read_usize(this + 4);
    if s < groups {
        if d == 6 {
            if news != 0 {
                exe::comp_util_write_group_qualify_news(club_id, a4, comp, 6, news, text);
            } else {
                group_qualify_text(club_id, qualify, text);
            }
        } else if d == 0xfe {
            exe::comp_util_write_group_qualify_news(club_id, a4, comp, -2, news, text);
        }
    } else if Some(s) == middle {
        exe::comp_build_fate_news_text(this, club_id, outcome, s as i32, a4, fixture, a6, 0, a8, news, text);
    } else if s == groups {
        if news != 0 {
            exe::comp_util_write_cup_tie_result_news(club_id, fixture as u32, a8, outcome, a6, text);
        } else {
            covered!("cx04_5 club championship final text");
            final_stage_fate_text(this, club_id, d, finals, text);
        }
    }
}

/// Whether `group_championship_fate_text` only writes the text on this call (what its comparers compare side by side).
fn group_championship_text_only(outcome: u32, stage: u32, news: u32, text: usize, groups: i8) -> bool {
    let (s, d) = (stage as u8 as i8, outcome as u8);
    text != 0 && news == 0 && ((s < groups && d == 6) || s == groups)
}

/// The club championships' progress codes: each of the three codes (current, best, worst) passes through its table when
/// it is 1 to 4 and the stage (signed) is below `groups`; the bytes go to `comp_stats_club::BEST_POSITION`,
/// `CURRENT_POSITION` and `WORST_POSITION` of the club's entry (`exe::comp_stats_find_club_entry` on the statistics,
/// not checked for null), in that order.
unsafe fn group_championship_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32, groups: i8, tables: [[u8; 4]; 3]) {
    let e = exe::comp_stats_find_club_entry(League(this).comp_stats(), club);
    let code = |p: u32, t: &[u8; 4]| {
        let p = p as u8;
        if (stage as u8 as i8) < groups && (1..=4).contains(&p) { t[p as usize - 1] } else { p }
    };
    write_u8(e + comp_stats_club::BEST_POSITION, code(best, &tables[1]));
    write_u8(e + comp_stats_club::CURRENT_POSITION, code(current, &tables[0]));
    write_u8(e + comp_stats_club::WORST_POSITION, code(worst, &tables[2]));
}

/// The comparer of the club championships' progress: side by side on the 3 bytes of the entry; only the original
/// without an entry.
unsafe fn compare_group_championship_progress(name: &str, this: usize, club: usize, original: impl FnOnce(), copy: impl FnOnce()) {
    let stats = League(this).comp_stats();
    let e = if stats != 0 && club != 0 { exe::comp_stats_find_club_entry(stats, club) } else { 0 };
    if e == 0 {
        return original();
    }
    let (_, _, same, mo, mr) = crate::comparison::side_by_side_light(&[(e + comp_stats_club::CURRENT_POSITION, 3)], original, copy);
    SCOREBOARD.record(same, || format!("{}({:#x}, club {:#x}): original {:x?} rust {:x?}", name, this, club, mo, mr));
}

/// The Oceania Club Championship's fate texts: groups before stage 1, the base text at stage 1, the final at stage 2.
const OCEANIA_FINALS: [(u8, usize); 2] = [(1, 0x98732c), (2, 0x9872c8)]; // "win" / "finish runners-up in"

/// Writes the news text of a club's fate in the Oceania Club Championship (`group_championship_fate_text`): through
/// from its group, "{club} qualify for semi final" (0xa6a0ac); at stage 1 the base text; in the final stage the winner
/// (0x98732c) and the runner-up (0x9872c8).
/// Original: `OceaniaClubChamp::build_fate_news_text` (0x7976b0, oceania_club_champ.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn oceania_club_champ_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, fixture: usize, a6: u32, _a7: u32, a8: u32, news: u32, text: usize) {
    group_championship_fate_text(this, club_id, outcome, stage, a4, fixture, a6, a8, news, text, 1, Some(1), 0xa6a0ac, &OCEANIA_FINALS);
}

/// Stores a club's progress in the Oceania Club Championship (`group_championship_progress`): in the group stage
/// (stage below 1) current 1→1, 2→3, 3→5, 4→7; best 1, 2→1, 3→5, 4→7; worst 1, 2→3, 3→5, 4→7.
/// Original: `OceaniaClubChamp::set_club_progress` (0x7987a0, oceania_club_champ.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn oceania_club_champ_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    covered!("cx04_5 oceania club progress");
    group_championship_progress(this, club, stage, current, best, worst, 1, [[1, 3, 5, 7], [1, 1, 5, 7], [3, 3, 5, 7]]);
}

/// Comparer for `oceania_club_champ_build_fate_news_text`: the text-only paths side by side (`compare_text_writer`);
/// the group and tie news only the original.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_oceania_club_champ_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, fixture: usize, a6: u32, a7: u32, a8: u32, news: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, usize, u32, u32, u32, u32, usize)>(IDX_OCEANIA_CLUB_CHAMP_BUILD_FATE_NEWS_TEXT);
    if !group_championship_text_only(outcome, stage, news, text, 1) {
        return o(this, club_id, outcome, stage, a4, fixture, a6, a7, a8, news, text);
    }
    compare_text_writer(text, || o(this, club_id, outcome, stage, a4, fixture, a6, a7, a8, news, text),
        |t| oceania_club_champ_build_fate_news_text(this, club_id, outcome, stage, a4, fixture, a6, a7, a8, news, t),
        || format!("oceania_club_champ_build_fate_news_text(club {}, outcome {}, stage {})", club_id, outcome as u8, stage as u8 as i8));
}

pub(crate) unsafe extern "thiscall" fn compare_oceania_club_champ_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(IDX_OCEANIA_CLUB_CHAMP_SET_CLUB_PROGRESS);
    compare_group_championship_progress("oceania_club_champ_set_club_progress", this, club, || o(this, club, stage, current, best, worst),
        || oceania_club_champ_set_club_progress(this, club, stage, current, best, worst));
}

// ---------- cx05_6: the African Nations Cup progress codes (african_nations.cpp) ----------

/// Stores a nation's progress in the African Nations Cup statistics (`group_championship_progress` of cx04_5): in the
/// group stages (below 3) current 1→5, 2→9, 3→9, 4→13; best 1, 2→1, 3→9, 4→13; worst 1→5, 2, 3→9, 4→13; afterwards
/// as they come.
/// Original: `AfricanNations::set_club_progress` (0x4029c0, african_nations.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn african_nations_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    covered!("cx05_6 african nations progress");
    group_championship_progress(this, club, stage, current, best, worst, 3, [[5, 9, 9, 0xd], [1, 1, 9, 0xd], [5, 9, 9, 0xd]]); // cx05_6: the African Nations Cup codes
}

/// Comparer for `african_nations_set_club_progress`: side by side on the 3 bytes of the nation's entry
/// (`compare_group_championship_progress` of cx04_5).
pub(crate) unsafe extern "thiscall" fn compare_african_nations_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(IDX_AFRICAN_NATIONS_SET_CLUB_PROGRESS);
    compare_group_championship_progress("african_nations_set_club_progress", this, club, || o(this, club, stage, current, best, worst),
        || african_nations_set_club_progress(this, club, stage, current, best, worst));
}

// ---------- cx06_3: the European Championship qualifiers' constructor, entrants and predicted positions (euro_champ_qual.cpp) ----------

/// The qualifiers' edition: from `start`, up one year at a time while the remainder of (year − 1990) by 4 is not 0
/// (signed, as the original's `and 0x80000003` with its sign adjustment): the year of the next finals.
pub(crate) fn euro_qual_edition(start: i16) -> i16 {
    let mut year = start;
    while (year as i32 - 0x7c6) % 4 != 0 {
        year = year.wrapping_add(1);
    }
    year
}

/// Builds the European Championship qualifiers on the League base: League::League (`exe::new_league`), the competition
/// record in `league::COMPETITION`, the vtable 0x96a0d4, `club_sides` 4, `upper_division_id` and `lower_division_id`
/// −1, `active` 0, the drawn teams, the entrants, their count and the qualified flag (+0xfb) 0, the hosts count
/// (`euro_champ_qual::GROUP_FORMAT`) 1, `discipline_rule` 0, the last group stage index (+0xff) 7, the runners-up stage
/// index (+0x100) 8 and the play-off stage index (+0x101) 9, the edition `year` (the next finals year from `year` − 1,
/// `euro_qual_edition`), `league_flags` 0x24, the stored group results (+0x10, a new `LLIST` of 0xe bytes:
/// `exe::crt_operator_new` and `exe::new_llist`, or null) and `importance` 0xf. If the saved state loads
/// (`exe::league_load_from_temp_file`(this, 1) nonzero), returns this.
///
/// Otherwise: the rules (`euro_champ_qual_set_up_rules`; 0: the box at line 0x5e and this), the statistics
/// (`exe::crt_operator_new`(0x5ce) and `exe::new_comp_stats`; null: the box at line 0x66 and this), `stage_index`
/// `stage_index::MAIN`, `current_stage` −1, `stage_count` 10 and the stage vector (`exe::crt_malloc`(0x28),
/// unchecked). When the season `year` (signed word) is after the edition (only a 1999 start: the 1998 edition), the
/// Euro 2000 groups and results (EuroChampQual::create_groups_and_results 0x580bd0). When the edition is more than two
/// years after `year`, `active` 0 again. Returns this.
/// Original: `EuroChampQual::EuroChampQual` (0x57ce10, euro_champ_qual.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_euro_champ_qual(this: usize, year: u32, record: usize) -> usize {
    let l = League(this);
    exe::new_league(this);
    write_usize(this + league::COMPETITION, record);
    write_usize(this, 0x96a0d4); // the qualifiers' vtable
    l.set_club_sides(4);
    l.set_upper_division_id(-1);
    l.set_lower_division_id(0xffff_ffff);
    l.set_active(0);
    write_usize(this + euro_champ_qual::DRAWN_TEAMS, 0);
    write_usize(this + 0xee, 0); // the entrants
    write_u32(this + 0xf2, 0); // their count
    write_u32(this + 0xfb, 0); // the qualified flag
    write_u8(this + euro_champ_qual::GROUP_FORMAT, 1);
    l.set_discipline_rule(0);
    write_u8(this + 0xff, 7); // the last group stage index
    write_u8(this + 0x100, 8); // the runners-up stage index
    write_u8(this + 0x101, 9); // the play-off stage index
    let season = year as u16 as i16;
    l.set_year(euro_qual_edition(season.wrapping_sub(1)));
    l.set_league_flags(0x24);
    let p = exe::crt_operator_new(0xe);
    write_usize(this + 0x10, if p != 0 { exe::new_llist(p) } else { 0 }); // the stored group results
    l.set_importance(0xf);
    if exe::league_load_from_temp_file(this, 1) != 0 {
        return this;
    }
    if euro_champ_qual_set_up_rules(this) == 0 {
        show_error_at(0x9c4674, 0x5e); // euro_champ_qual.cpp
        return this;
    }
    let p = exe::crt_operator_new(0x5ce);
    let stats = if p != 0 { exe::new_comp_stats(p, this) } else { 0 };
    l.set_comp_stats(stats);
    if stats == 0 {
        show_error_at(0x9c4674, 0x66);
        return this;
    }
    covered!("cx06_3 euro qualifiers built");
    l.set_stage_index(stage_index::MAIN);
    l.set_current_stage(0xffff_ffff);
    l.set_stage_count(10);
    l.set_stages(exe::crt_malloc(0x28));
    if season > l.year() {
        covered!("cx06_3 the 1998 edition");
        let groups_and_results: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x580bd0usize); // EuroChampQual::create_groups_and_results
        groups_and_results(this);
    }
    if (l.year() as i32) - (season as i32) > 2 {
        l.set_active(0);
    }
    this
}

/// Lists the entrants of the European Championship qualifiers: the European nations' national teams but the hosts.
///
/// `euro_champ_qual::GROUP_FORMAT` counts the European hosts of the finals: without the host table
/// (`G_HOST_COUNTRY`), the box at line 0x4c3; otherwise HostCountry::find_hosts (`exe::hostcountry_find_hosts`: the
/// competition id in `KEY_CLUB_COMP_EUROPEAN_FOOTBALL_CHAMPIONSHIP`, the edition `year()` + 2, the two host ids, 1) and
/// each host (up to the count, stopping at −1) whose nation's continent (`nation::CONTINENT`, its id) is
/// `KEY_CONTINENT_EUROPE` adds one. Below 0 or above 2: the box at line 0x4dd and 1. The entrant count (+0xf2) is 51 −
/// hosts; the old list (+0xee) is freed and a new one allocated (`exe::crt_malloc`; out of memory: the box at line 0x4ec
/// and 0); every European nation in table order (`G_NATION_TABLE`, `G_NATION_COUNT`), while the list has room, gives
/// its national team (`exe::database_get_national_team`) unless its id is one of the two hosts. A short list: the box at
/// line 0x506 and 0. The list is sorted by reputation (`exe::crt_qsort` with 0x4b07d0). Returns 1.
/// Original: `EuroChampQual::collect_entrants` (0x57f600, euro_champ_qual.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn euro_champ_qual_collect_entrants(this: usize) -> u32 {
    let l = League(this);
    write_u32(this + 0xf2, 0x33); // the entrant count
    write_u8(this + euro_champ_qual::GROUP_FORMAT, 0);
    let mut hosts = [-1i32; 2];
    let is_european = |nation: usize| {
        let continent = read_usize(nation + nation::CONTINENT);
        continent != 0 && read_u32(continent) == read_u32(KEY_CONTINENT_EUROPE)
    };
    let nation_at = |id: usize| read_usize(G_NATION_TABLE).wrapping_add(id.wrapping_mul(nation::SIZE));
    if read_usize(G_HOST_COUNTRY) == 0 {
        show_error_at(0x9c4674, 0x4c3); // euro_champ_qual.cpp
    } else {
        let (first, second) = hosts.split_at_mut(1);
        let n = exe::hostcountry_find_hosts(read_usize(G_HOST_COUNTRY), read_u32(KEY_CLUB_COMP_EUROPEAN_FOOTBALL_CHAMPIONSHIP) as usize,
            l.year().wrapping_add(2) as u16 as u32, &mut first[0], &mut second[0], 1) as i8;
        let mut k = 0i8;
        while k < n {
            let Some(&host) = hosts.get(k as usize) else { break };
            if host == -1 {
                break;
            }
            if is_european(nation_at(host as usize)) {
                covered!("cx06_3 european host");
                write_u8(this + euro_champ_qual::GROUP_FORMAT, read_u8(this + euro_champ_qual::GROUP_FORMAT).wrapping_add(1));
            }
            k += 1;
        }
    }
    let european_hosts = read_u8(this + euro_champ_qual::GROUP_FORMAT) as i8;
    if !(0..=2).contains(&european_hosts) {
        show_error_at(0x9c4674, 0x4dd);
        write_u8(this + euro_champ_qual::GROUP_FORMAT, 1);
    }
    let count = 0x33 - read_u8(this + euro_champ_qual::GROUP_FORMAT) as i8 as i32;
    write_i32(this + 0xf2, count);
    if read_usize(this + 0xee) != 0 {
        exe::crt_free(read_usize(this + 0xee));
    }
    let list = exe::crt_malloc((count as u32).wrapping_mul(4));
    write_usize(this + 0xee, list); // the entrants
    if list == 0 {
        show_error_at(0x9c4674, 0x4ec);
        return 0;
    }
    let mut added = 0i32;
    let mut k: i16 = 0;
    while (k as i32) < read_i32(G_NATION_COUNT) {
        let nation = nation_at(k as usize);
        if is_european(nation) && added < read_i32(this + 0xf2) {
            let team = exe::database_get_national_team(nation);
            let id = read_i32(nation);
            if id != hosts[0] && id != hosts[1] {
                write_usize(read_usize(this + 0xee) + 4 * added as usize, team);
                added += 1;
            }
        }
        k = k.wrapping_add(1);
    }
    if added < read_i32(this + 0xf2) {
        show_error_at(0x9c4674, 0x506);
        return 0;
    }
    covered!("cx06_3 qualifiers entrants");
    exe::crt_qsort(read_usize(this + 0xee), added as u32, 4, 0x4b07d0usize); // by reputation, descending
    1
}

/// The predicted final position of the `k`-th entrant (by reputation) of the qualifiers, by the number of European
/// hosts (`format`): one host, 9 group winners (1), then the best runners-up and the play-off bands 10, 13 and 16 (3
/// each), then 19, 28 and 37 (9 each) and 46; two hosts, 1 (9), 10 (1), 11 (4), 15 (4), 19, 28, 37 (9 each) and 46. No
/// position for any other format.
pub(crate) fn euro_qual_predicted_band(format: u8, k: i16) -> Option<u32> {
    let bands: &[(i16, u32)] = match format {
        1 => &[(9, 1), (0xc, 0xa), (0xf, 0xd), (0x12, 0x10), (0x1b, 0x13), (0x24, 0x1c), (0x2d, 0x25)],
        2 => &[(9, 1), (0xa, 0xa), (0xe, 0xb), (0x12, 0xf), (0x1b, 0x13), (0x24, 0x1c), (0x2d, 0x25)],
        _ => return None,
    };
    Some(bands.iter().find(|b| k < b.0).map_or(0x2e, |b| b.1))
}

/// Sets the predicted final positions of the qualifiers' entrants: without statistics, the box at line 0x6f3. The
/// entrants (+0xee, count +0xf2) sorted by reputation (`exe::crt_qsort` with 0x4b07d0); the national teams freed from
/// their European competitions (NationalTeams::release_competitions_of_continent
/// `exe::nationalteams_release_competitions_of_continent` on the continent `KEY_CONTINENT_EUROPE`, 0xc6-byte records of
/// `G_CONTINENT_TABLE`, with 1); then each entrant k, in order, its band (`euro_qual_predicted_band`, Comp_Stats
/// set_club_final_position `exe::comp_stats_set_club_final_position`) and the qualifiers as its nation's current
/// competition (NationalTeams::set_current_competition `exe::nationalteams_set_current_competition`).
/// Original: `EuroChampQual::set_predicted_positions` (0x5806d0, euro_champ_qual.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn euro_champ_qual_set_predicted_positions(this: usize) {
    let l = League(this);
    if l.comp_stats() == 0 {
        show_error_at(0x9c4674, 0x6f3); // euro_champ_qual.cpp
        return;
    }
    exe::crt_qsort(read_usize(this + 0xee), read_u32(this + 0xf2), 4, 0x4b07d0usize);
    let europe = read_usize(G_CONTINENT_TABLE).wrapping_add((read_u32(KEY_CONTINENT_EUROPE) as usize).wrapping_mul(0xc6)); // continent records
    exe::nationalteams_release_competitions_of_continent(read_usize(G_NATIONAL_TEAMS), europe, 1);
    let mut k: i16 = 0;
    while (k as i32) < read_i32(this + 0xf2) {
        let team = Club(read_usize(read_usize(this + 0xee) + 4 * k as usize));
        if let Some(band) = euro_qual_predicted_band(read_u8(this + euro_champ_qual::GROUP_FORMAT), k) {
            covered!("cx06_3 qualifiers predicted band");
            exe::comp_stats_set_club_final_position(l.comp_stats(), team, band);
        }
        exe::nationalteams_set_current_competition(read_usize(G_NATIONAL_TEAMS), team.nation(), l.competition());
        k = k.wrapping_add(1);
    }
}

/// Comparer for `construct_euro_champ_qual`: only the original runs (it allocates and loads); the plan: this returned,
/// the vtable, the record, the edition year (`euro_qual_edition`; any year when loading a saved game) and importance
/// 0xf (`compare_nations_tournament_ctor`).
pub(crate) unsafe extern "thiscall" fn compare_construct_euro_champ_qual(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_EURO_CHAMP_QUAL);
    compare_nations_tournament_ctor("construct_euro_champ_qual", League(this), record, 0x96a0d4, euro_qual_edition((year as u16 as i16).wrapping_sub(1)), || o(this, year, record))
}

/// Comparer for `euro_champ_qual_collect_entrants`: only the original runs (it frees and allocates the list); the plan:
/// on success the count is 51 minus the hosts (0 to 2), every entrant a national team, sorted by reputation
/// (`club::REPUTATION`, descending).
pub(crate) unsafe extern "thiscall" fn compare_euro_champ_qual_collect_entrants(this: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_EURO_CHAMP_QUAL_COLLECT_ENTRANTS)(this);
    if r != 1 {
        return r;
    }
    let (list, count, hosts) = (read_usize(this + 0xee), read_i32(this + 0xf2), read_u8(this + euro_champ_qual::GROUP_FORMAT) as i32);
    let teams: Vec<Club> = (0..count.clamp(0, 64) as usize).map(|k| Club(read_usize(list + 4 * k))).collect();
    let sorted = teams.windows(2).all(|w| w[0].reputation() >= w[1].reputation());
    let ok = count == 0x33 - hosts && (0..=2).contains(&hosts) && teams.iter().all(|t| t.addr() != 0) && sorted;
    SCOREBOARD.record(ok, || format!("euro_champ_qual_collect_entrants({:#x}): {} entrants, {} hosts, sorted {}", this, count, hosts, sorted));
    r
}

/// Comparer for `euro_champ_qual_set_predicted_positions`: only the original runs (the national teams' competitions
/// change); the plan: by reputation the entrants' final positions make the bands (`predicted_by_reputation_check`).
pub(crate) unsafe extern "thiscall" fn compare_euro_champ_qual_set_predicted_positions(this: usize) {
    let (list, count) = (read_usize(this + 0xee), read_i32(this + 0xf2));
    let clubs: Vec<usize> = if list != 0 && (0..=64).contains(&count) { (0..count as usize).map(|k| read_usize(list + 4 * k)).collect() } else { Vec::new() };
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_EURO_CHAMP_QUAL_SET_PREDICTED_POSITIONS)(this);
    let format = read_u8(this + euro_champ_qual::GROUP_FORMAT);
    let bands: Vec<u32> = (0..clubs.len() as i16).filter_map(|k| euro_qual_predicted_band(format, k)).collect();
    if bands.len() == clubs.len() {
        predicted_by_reputation_check("euro_champ_qual_set_predicted_positions", this, &clubs, &bands);
    }
}

// ---------- cx06_4: the European Championship qualifiers' round tables, group stages and day (euro_champ_qual.cpp) ----------

/// The qualifiers' group round table: ten matchdays (`D`: day, month from 0 = January, years after the edition year,
/// weekday, code 1 or 2, value 0), each with its first match slot empty (`L`): 2 and 6 September, 7 and 11 October,
/// 24 and 28 March, 6 June, 2 and 6 September and 7 October.
pub(crate) const EURO_QUAL_GROUP_OPS: [GoldcupScheduleOp; 20] = [
    Cqp12D(0, 2, 8, 0, 5, 1, 0), Cqp12L(0, 0, -1, -1, -1, 0), Cqp12D(1, 6, 8, 0, 2, 2, 0), Cqp12L(1, 0, -1, -1, -1, 0),
    Cqp12D(2, 7, 9, 0, 5, 1, 0), Cqp12L(2, 0, -1, -1, -1, 0), Cqp12D(3, 0xb, 9, 0, 2, 2, 0), Cqp12L(3, 0, -1, -1, -1, 0),
    Cqp12D(4, 0x18, 2, 1, 5, 1, 0), Cqp12L(4, 0, -1, -1, -1, 0), Cqp12D(5, 0x1c, 2, 1, 2, 2, 0), Cqp12L(5, 0, -1, -1, -1, 0),
    Cqp12D(6, 6, 5, 1, 2, 2, 0), Cqp12L(6, 0, -1, -1, -1, 0), Cqp12D(7, 2, 8, 1, 5, 1, 0), Cqp12L(7, 0, -1, -1, -1, 0),
    Cqp12D(8, 6, 8, 1, 2, 2, 0), Cqp12L(8, 0, -1, -1, -1, 0), Cqp12D(9, 7, 9, 1, 5, 1, 0), Cqp12L(9, 0, -1, -1, -1, 0),
];

/// The qualifiers' play-off round: the draw on 11 October of the year after the edition year (`S`, weekday 2), the
/// two legs from 11 November (`J`, weekday 5, time 1), round code 0xa0, tie rules 3, category 10, two legs (+0x21 2),
/// +0x22 4 and the prize words cleared.
pub(crate) const EURO_QUAL_PLAYOFF_OPS: [GoldcupScheduleOp; 15] = [
    Cqp12S(0, 0xb, 9, 1, 2), Cqp12J(0, 0xb, 0xa, 1, 5, 1, 0), Cqp12W(0x7, 0xa0), Cqp12W(0x9, 0), Cqp12W(0xb, 0), Cqp12W(0xd, 0),
    Cqp12B(0x17, 0xa), Cqp12W(0xf, 3), Cqp12W(0x1e, 0), Cqp12B(0x20, 0), Cqp12B(0x21, 2), Cqp12B(0x22, 4), Cqp12X(0x5c, 0),
    Cqp12X(0x60, 0), Cqp12X(0x64, 0),
];

/// The play-off size with one European host: 6 nations, 3 ties.
pub(crate) const EURO_QUAL_PLAYOFF_SIX: [GoldcupScheduleOp; 3] = [Cqp12W(0x18, 6), Cqp12W(0x1a, 3), Cqp12W(0x1c, 6)];

/// The play-off size with two European hosts: 8 nations, 4 ties.
pub(crate) const EURO_QUAL_PLAYOFF_EIGHT: [GoldcupScheduleOp; 3] = [Cqp12W(0x18, 8), Cqp12W(0x1a, 4), Cqp12W(0x1c, 8)];

/// The round table of the qualifiers' stage `stage` (signed byte): up to the last group stage (+0xff) the group table
/// (a league, 10 rounds, the stage id 0x3e9 + stage, 0x28a bytes, box line 0xbb); the play-off stage (+0x101, unless
/// it is also the runners-up stage +0x100) the play-off round (not a league, 1 round, id 0, 0x68 bytes, box line 0xf5)
/// with the size of the format (`euro_champ_qual::GROUP_FORMAT` 1 or 2); any other stage none.
pub(crate) unsafe fn euro_qual_phase(this: usize, stage: u32) -> Option<GoldcupStagePlan> {
    let f = stage as u8 as i8;
    if f <= read_u8(this + 0xff) as i8 { // the last group stage
        return Some(GoldcupStagePlan { league: 1, rounds: 10, extra: (f as i16).wrapping_add(0x3e9), extra_check: false, size: 0x28a, row: 0xbb,
            ops: &EURO_QUAL_GROUP_OPS, ops2: &[], name: "cx06_4 qualifiers group rounds" });
    }
    if f == read_u8(this + 0x100) as i8 || f != read_u8(this + 0x101) as i8 { // the runners-up / play-off stages
        return None;
    }
    let ops2: &'static [GoldcupScheduleOp] = match read_u8(this + euro_champ_qual::GROUP_FORMAT) {
        1 => &EURO_QUAL_PLAYOFF_SIX,
        2 => &EURO_QUAL_PLAYOFF_EIGHT,
        _ => &[],
    };
    Some(GoldcupStagePlan { league: 0, rounds: 1, extra: 0, extra_check: false, size: 0x68, row: 0xf5, ops: &EURO_QUAL_PLAYOFF_OPS, ops2,
        name: "cx06_4 qualifiers play-off round" })
}

/// Builds the round table of a qualifiers' stage (`euro_qual_phase` with the round-table machine `build_phase_table`):
/// ten group matchdays for each group, one play-off round of two legs for 6 or 8 nations. With the play-off and a
/// format other than 1 or 2, the box at line 0x11b after the table (which is still returned).
/// Original: `EuroChampQual::create_schedule` (0x57d190, euro_champ_qual.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn euro_champ_qual_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let plan = euro_qual_phase(this, stage);
    let unknown_format = plan.is_some_and(|p| p.league == 0) && !matches!(read_u8(this + euro_champ_qual::GROUP_FORMAT), 1 | 2);
    let table = build_phase_table(League(this), plan, rounds, extra, league, 0x9c4674);
    if table != 0 && unknown_format {
        show_error_at(0x9c4674, 0x11b); // euro_champ_qual.cpp
    }
    table
}

/// Creates the League_Stage of qualifying group `g` + 1 (stage `g`).
///
/// The round table of stage `g` (`comp_vtable::CREATE_SCHEDULE`: g, &rounds, &id, 0). Without a team list, the group's
/// size (`exe::eurochampqual_get_group_team_count`(g)) and a list of its drawn teams (`euro_champ_qual::DRAWN_TEAMS`,
/// slots 6 × (g + 1) on; `exe::crt_malloc`; out of memory: the box at line 0x3b4 and 0); with `list`, the round table
/// is freed (the stage gets none) and `count` teams of `list` are used. The stage (`exe::crt_operator_new`(0xee) and
/// League_Stage::League_Stage `FN_LEAGUE_STAGE_LEAGUE_STAGE`, thiscall, ret 0x58: the qualifiers, the teams, the
/// list, 2, `.competition()`, the round table, the rounds, the win, draw and category bytes, &tie-breaks, &promoted,
/// the year, g, the id, the importance, 4, 0, `.league_flags()`, −1, 0, 2) into `.stages()`[g]; none: the box at line
/// 0x3d0 and 0 (the list and the round table are not freed). Then the own list and the round table freed,
/// `.current_stage()` = g, the group draw news (`exe::league_send_group_draw_news` on the stage) and 1.
/// Original: `EuroChampQual::create_group_stage` (0x57ed10, euro_champ_qual.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn euro_champ_qual_create_group_stage(this: usize, g: u32, list: usize, count: u32) -> u32 {
    let l = League(this);
    let schedule: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut id) = (0i16, 0i16);
    let mut table = schedule(this, g, &mut rounds, &mut id, 0);
    let (teams, n) = if list == 0 {
        let n = exe::eurochampqual_get_group_team_count(this, g);
        let teams = exe::crt_malloc((n as i32 as u32).wrapping_mul(4));
        if teams == 0 {
            show_error_at(0x9c4674, 0x3b4); // euro_champ_qual.cpp
            return 0;
        }
        covered!("cx06_4 group drawn teams");
        let drawn = read_usize(this + euro_champ_qual::DRAWN_TEAMS);
        for k in 0..n.max(0) as usize {
            write_usize(teams + 4 * k, read_usize(drawn + 4 * (6 * (g as usize + 1) + k)));
        }
        (teams, n as u16 as u32)
    } else {
        if table != 0 {
            exe::crt_free(table);
        }
        table = 0;
        (list, count as u16 as u32)
    };
    #[allow(clippy::type_complexity)]
    let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    let p = exe::crt_operator_new(0xee);
    let stage = if p != 0 {
        construct(p, this, n, teams, 2, l.competition().addr(), table, rounds as u16 as u32, l.win_points() as u32, l.draw_points() as u32,
            l.match_category() as u32, this + league::TIEBREAKS, this + league::PROMOTED_COUNT, l.year() as u16 as u32, g, id as u16 as u32,
            l.importance() as u32, 4, 0, l.league_flags() as u16 as u32, -1, 0, 2)
    } else {
        0
    };
    write_usize(l.stages() + 4 * g as usize, stage);
    if read_usize(l.stages() + 4 * g as usize) == 0 {
        show_error_at(0x9c4674, 0x3d0);
        return 0;
    }
    if list == 0 {
        exe::crt_free(teams);
    }
    if table != 0 {
        exe::crt_free(table);
    }
    l.set_current_stage(g as usize);
    exe::league_send_group_draw_news(read_usize(l.stages() + 4 * g as usize));
    1
}

/// The qualifiers' day. Before the draw date (`oceania_nations_draw_date`: `.next_draw_day()` of `.year()` +
/// `.next_draw_year_offset()`; today's year unsigned, its day signed) nothing happens and 0 is returned (al; the
/// League day does not run).
///
/// On the draw date itself, while not `active` and with `full`: the entrants (EuroChampQual::collect_entrants 0x57f600),
/// the groups draw (`euro_champ_qual_draw_groups`), group 1's table (`init_group_one_table`; 0: the box at line 0x5a8 and
/// 0), its fixtures (`exe::league_create_fixtures`; 0: the box at line 0x5af and 0), its table sorted
/// (`exe::league_sort_table`(this, 0); 0: the box at line 0x5b6 and 0), the stages of groups 2 to 9
/// (`euro_champ_qual_create_group_stage`(g, 0, 6), g = 0..7), the drawn teams freed and cleared, the runners-up stage
/// (EuroChampQual::create_runners_up_stage 0x57f920), the predicted positions (`comp_vtable::SET_PREDICTED_POSITIONS`),
/// the entrants freed and cleared with their count, and `active` 1. On other days, when the competition has a fixture
/// on the date (`fix_man_comp_has_fixture_on_date` on a copy of today), the runners-up table
/// (EuroChampQual::update_runners_up_table 0x57fa10). Then the League day (`exe::league_update_for_date`(this, today,
/// `full`)), whose result it returns.
/// Original: `EuroChampQual::update_for_date` (0x57fd60, euro_champ_qual.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn euro_champ_qual_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let l = League(this);
    let (draw_day, draw_year) = oceania_nations_draw_date(l);
    let (day, year) = (read_i16(today), read_u16(today + 2));
    if year < draw_year || (year == draw_year && day < draw_day as i16) {
        return 0;
    }
    if year == draw_year && day as u16 == draw_day && l.active() == 0 && full != 0 {
        covered!("cx06_4 qualifiers draw day");
        let collect_entrants: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x57f600usize); // EuroChampQual::collect_entrants
        collect_entrants(this);
        euro_champ_qual_draw_groups(this);
        if init_group_one_table(this) == 0 {
            show_error_at(0x9c4674, 0x5a8); // euro_champ_qual.cpp
            return 0;
        }
        if exe::league_create_fixtures(this) == 0 {
            show_error_at(0x9c4674, 0x5af);
            return 0;
        }
        if exe::league_sort_table(this, 0) == 0 {
            show_error_at(0x9c4674, 0x5b6);
            return 0;
        }
        for g in 0..8 {
            euro_champ_qual_create_group_stage(this, g, 0, 6);
        }
        exe::crt_free(read_usize(this + euro_champ_qual::DRAWN_TEAMS));
        write_usize(this + euro_champ_qual::DRAWN_TEAMS, 0);
        let create_runners_up_stage: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x57f920usize); // EuroChampQual::create_runners_up_stage
        create_runners_up_stage(this);
        let predicted: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::SET_PREDICTED_POSITIONS));
        predicted(this);
        exe::crt_free(read_usize(this + 0xee)); // the entrants
        write_usize(this + 0xee, 0);
        write_u32(this + 0xf2, 0);
        l.set_active(1);
    } else {
        let date = [read_u32(today), read_u32(today + 4)];
        if fix_man_comp_has_fixture_on_date(date.as_ptr() as usize, read_u32(l.competition().addr())) != 0 {
            covered!("cx06_4 qualifiers match day");
            let update_runners_up_table: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x57fa10usize); // EuroChampQual::update_runners_up_table
            update_runners_up_table(this);
        }
    }
    exe::league_update_for_date(this, today, full) as u32
}

/// Comparer for `euro_champ_qual_create_schedule`: side by side (`compare_phase_table`: the outputs and the bytes
/// written to the two tables).
pub(crate) unsafe extern "thiscall" fn compare_euro_champ_qual_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_EURO_CHAMP_QUAL_CREATE_SCHEDULE);
    compare_phase_table("euro_champ_qual_create_schedule", stage, euro_qual_phase(this, stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| euro_champ_qual_create_schedule(this, stage, r, e, l))
}

/// Comparer for `euro_champ_qual_create_group_stage`: it allocates the stage and sends the draw news (it cannot run
/// twice), so only the original runs; the plan: on success the stage at `.stages()`[g] with the group's teams (the
/// drawn ones or the list, as a set: the stage sorts its table), `.current_stage()` = g.
pub(crate) unsafe extern "thiscall" fn compare_euro_champ_qual_create_group_stage(this: usize, g: u32, list: usize, count: u32) -> u32 {
    let l = League(this);
    let expected: Vec<usize> = if list == 0 {
        let drawn = read_usize(this + euro_champ_qual::DRAWN_TEAMS);
        let n = exe::eurochampqual_get_group_team_count(this, g).clamp(0, 6) as usize;
        if drawn != 0 { (0..n).map(|k| read_usize(drawn + 4 * (6 * (g as usize + 1) + k))).collect() } else { Vec::new() }
    } else {
        (0..(count as u16 as i16).clamp(0, 8) as usize).map(|k| read_usize(list + 4 * k)).collect()
    };
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize, u32) -> u32>(IDX_EURO_CHAMP_QUAL_CREATE_GROUP_STAGE)(this, g, list, count);
    if r != 1 {
        return r;
    }
    let stage = League(read_usize(l.stages() + 4 * g as usize));
    let mut got: Vec<usize> = (0..stage.club_count().clamp(0, 8) as usize).map(|k| read_usize(stage.table() + k * league_table_row::SIZE)).collect();
    let mut want = expected.clone();
    got.sort_unstable();
    want.sort_unstable();
    SCOREBOARD.record(got == want && l.current_stage() == g as usize, || format!("euro_champ_qual_create_group_stage({:#x}, {}): teams {:x?} expected {:x?}", this, g, got, want));
    r
}

/// Comparer for `euro_champ_qual_update_for_date`: the day draws, allocates and updates the league (it cannot run
/// twice), so only the original runs; the plan: after the draw day, `active` 1, the drawn teams and the entrants
/// cleared and group 1's table built.
pub(crate) unsafe extern "thiscall" fn compare_euro_champ_qual_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let l = League(this);
    let draws = full != 0 && l.active() == 0 && {
        let (day, year) = oceania_nations_draw_date(l);
        read_u16(today) == day && read_u16(today + 2) == year
    };
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_EURO_CHAMP_QUAL_UPDATE_FOR_DATE)(this, today, full);
    if draws {
        let ok = l.active() == 1 && read_usize(this + euro_champ_qual::DRAWN_TEAMS) == 0 && read_usize(this + 0xee) == 0 && l.table() != 0;
        SCOREBOARD.record(ok, || format!("euro_champ_qual_update_for_date({:#x}): the draw day left active {}, table {:#x}", this, l.active(), l.table()));
    }
    r
}

// ---------- cx07_2: the Copa América knockout and the progress codes (goldcup.cpp) ----------

/// The file name of goldcup.cpp for the error boxes.
const GOLDCUP_SOURCE: usize = 0x9cd640;

/// The knockout stage `stage` of a national-team tournament from the `count` teams in the vector `teams`: the round
/// table through the virtual `comp_vtable::CREATE_SCHEDULE` (push 0; push &id; push &rounds; push stage; mov ecx ->
/// thiscall, ret 0x10), the stage (`exe::crt_operator_new`(0xb2); Cup_Stage::Cup_Stage `FN_CUP_STAGE_CUP_STAGE`,
/// thiscall, ret 0x3c: the tournament, `count`, the vector, the rounds, `.competition()`, the round table, `.year()`,
/// `stage`, 4, the stage id, 0x14, 0, 0, 0, 0; the words with garbage in the high bytes, which the constructor does not
/// read) into `.stages()`[stage]; the vector and the round table freed (`exe::crt_free`). No stage: the box at line
/// `line` of `file`. Otherwise its first round is set up (`exe::cup_set_up_round`(stage, 0)) and
/// `.set_current_stage(stage)`.
pub(crate) unsafe fn nations_knockout_from(this: League, stage: u32, count: u32, teams: usize, file: usize, line: u32) {
    #[allow(clippy::type_complexity)]
    let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize =
        core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
    let create: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut id) = (0i16, 0i16);
    let table = create(this.addr(), stage, &mut rounds, &mut id, 0);
    let p = exe::crt_operator_new(0xb2);
    let knockout = if p != 0 {
        construct(p, this.addr(), count, teams, rounds as u16 as u32, this.competition().addr(), table, this.year() as u16 as u32, stage, 4, id as u16 as u32, 0x14, 0, 0, 0, 0)
    } else {
        0
    };
    write_usize(this.stages() + stage as usize * 4, knockout);
    exe::crt_free(teams);
    exe::crt_free(table);
    let knockout = read_usize(this.stages() + stage as usize * 4);
    if knockout == 0 {
        return show_error_at(file, line);
    }
    exe::cup_set_up_round(knockout, 0);
    this.set_current_stage(stage as usize);
}

/// The plan of a knockout builder that only the original runs: the stage at `.stages()`[stage] and
/// `.current_stage()` = stage.
pub(crate) unsafe fn check_nations_knockout(name: &str, this: League, stage: usize) {
    let ok = this.stages() != 0 && read_usize(this.stages() + stage * 4) != 0 && this.current_stage() == stage;
    SCOREBOARD.record(ok, || format!("{}({:#x}): stage {} (knockout expected at {})", name, this.addr(), this.current_stage() as i32, stage));
}

/// The table of group `g` of the Copa América: group A (−1) is the cup's own table, B and C the stages 0 and 1.
unsafe fn copa_america_group(this: League, g: i8) -> usize {
    if g == -1 { this.addr() } else { read_usize(this.stages() + g as usize * 4) }
}

/// The Copa América knockout (stage 2): the eight quarter-finalists are the three group winners, the three
/// runners-up and the two best thirds, in a shuffled draw.
///
/// The thirds are ranked in a temporary League_Stage of 3 teams (`exe::crt_operator_new`(0xee); League_Stage::
/// League_Stage `FN_LEAGUE_STAGE_LEAGUE_STAGE`, thiscall, ret 0x58: the cup, 3, no list, 0, `.competition()`, no table,
/// 0, `.win_points()`, `.draw_points()`, 0, &`league::TIEBREAKS`, &`league::PROMOTED_COUNT`, `.year()`, −2, 0, 0, 1, 0,
/// 2, −1, 0, 2; the new is not checked): its row g + 1 is a copy of row 2 of group g's table with the position history
/// pointer cleared, and the table is sorted (`exe::league_sort_table`(stage, 0)). A vector of 8 (`exe::crt_malloc`(0x20);
/// out of memory: the box at line 0x30b and nothing more): A's winner and runner-up, B's, C's, then the two best thirds.
/// The worst third goes out of the statistics (the cup's virtual `comp_vtable::SET_CLUB_PROGRESS`(club, −1, 3, 3, 3)).
/// Each group then updates its positions and statuses (its virtual `league_vtable::UPDATE_POSITIONS_AND_STATUS`,
/// thiscall, ret 0) and announces through its own virtual `comp_vtable::ANNOUNCE_TEAM_STATUS` (5 pushes, thiscall):
/// its third, when one of the two best, through (6, one more place in its `league::PROMOTED_COUNT`), otherwise out
/// (−2); its fourth always out (−2). The temporary stage is destroyed (virtual 0 with 1), the vector shuffled
/// (`exe::comp_util_shuffle_pointer_array`(vector, 8), cdecl) and the knockout built (`nations_knockout_from`, stage
/// 2, 8 teams; no stage: the box at line 0x35e).
/// Original: `GoldCup::create_knockout_stage` (0x5e18a0, goldcup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn goldcup_create_knockout_stage(this: usize) {
    #[allow(clippy::type_complexity)]
    let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    let league = League(this);
    let p = exe::crt_operator_new(0xee);
    let thirds = if p != 0 {
        construct(p, this, 3, 0, 0, league.competition().addr(), 0, 0, league.win_points() as u32, league.draw_points() as u32, 0, this + league::TIEBREAKS,
            this + league::PROMOTED_COUNT, league.year() as u16 as u32, (-2i32) as u32, 0, 0, 1, 0, 2, -1, 0, 2)
    } else {
        0
    };
    // the original reads the temporary stage's table without checking the new (a null stage faults, as here)
    for g in -1i8..2 {
        let row = League(thirds).table() + (g + 1) as usize * league_table_row::SIZE;
        let third = League(copa_america_group(league, g)).table() + 2 * league_table_row::SIZE; // cx07_2: the third of each group
        core::ptr::copy_nonoverlapping(third as *const u8, row as *mut u8, league_table_row::SIZE);
        write_usize(row + league_table_row::POSITION_HISTORY, 0);
    }
    exe::league_sort_table(thirds, 0);
    let teams = exe::crt_malloc(0x20);
    if teams == 0 {
        return show_error_at(GOLDCUP_SOURCE, 0x30b);
    }
    let mut k = 0usize;
    for g in -1i8..2 {
        let group = copa_america_group(league, g);
        for r in 0..2 {
            write_usize(teams + k * 4, table_row_club(group, r));
            k += 1;
        }
    }
    let best_thirds = 2; // cx07_2: the best thirds that go through
    for r in 0..best_thirds {
        write_usize(teams + (6 + r as usize) * 4, table_row_club(thirds, r));
    }
    covered!("cx07_2 copa america quarter-finalists");
    let progress: unsafe extern "thiscall" fn(usize, usize, i32, u32, u32, u32) = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::SET_CLUB_PROGRESS));
    progress(this, table_row_club(thirds, 2), -1, 3, 3, 3);
    for g in -1i8..2 {
        let group = copa_america_group(league, g);
        let update: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(group) + league_vtable::UPDATE_POSITIONS_AND_STATUS));
        update(group);
        let announce: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) = core::mem::transmute(read_usize(read_usize(group) + comp_vtable::ANNOUNCE_TEAM_STATUS));
        // simplification: the original pushes a dword whose low byte is the group and whose high bytes are stack
        // leftovers; the announcement reads the byte
        let third = table_row_club(group, 2);
        if third != table_row_club(thirds, 0) && third != table_row_club(thirds, 1) {
            announce(group, third, 0xfe, g as u8 as u32, 0, 0);
        } else {
            covered!("cx07_2 third through");
            write_u8(group + league::PROMOTED_COUNT, read_u8(group + league::PROMOTED_COUNT).wrapping_add(1));
            announce(group, third, 6, g as u8 as u32, 0, 0);
        }
        announce(group, table_row_club(group, 3), 0xfe, g as u8 as u32, 0, 0);
    }
    if thirds != 0 {
        let destroy: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(read_usize(read_usize(thirds) + comp_vtable::SCALAR_DELETING_DESTRUCTOR));
        destroy(thirds, 1);
    }
    exe::comp_util_shuffle_pointer_array(teams, 8);
    nations_knockout_from(league, 2, 8, teams, GOLDCUP_SOURCE, 0x35e);
}

/// Comparer for `goldcup_create_knockout_stage`: only the original (the stages allocate, the draw is shuffled and the
/// first round drawn, the statuses announced); then the plan (`check_nations_knockout`).
pub(crate) unsafe extern "thiscall" fn compare_goldcup_create_knockout_stage(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_GOLDCUP_CREATE_KNOCKOUT_STAGE)(this);
    check_nations_knockout("goldcup_create_knockout_stage", League(this), 2);
}

/// Stores a nation's progress in the Copa América statistics (`group_championship_progress` of cx04_5): in the group
/// stages (below 2) current 1, 2→5, 3→9, 4→10; best 1, 2, 3→1, 4→10; worst 1, 2→5, 3→9, 4→10; afterwards as they
/// come.
/// Original: `GoldCup::set_club_progress` (0x5e2500, goldcup.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn goldcup_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    covered!("cx07_2 copa america progress");
    group_championship_progress(this, club, stage, current, best, worst, 2, [[5, 5, 9, 0xa], [1, 1, 1, 0xa], [5, 5, 9, 0xa]]); // cx07_2: the Copa América codes
}

/// Comparer for `goldcup_set_club_progress`: side by side on the 3 bytes of the nation's entry
/// (`compare_group_championship_progress` of cx04_5).
pub(crate) unsafe extern "thiscall" fn compare_goldcup_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(IDX_GOLDCUP_SET_CLUB_PROGRESS);
    compare_group_championship_progress("goldcup_set_club_progress", this, club, || o(this, club, stage, current, best, worst),
        || goldcup_set_club_progress(this, club, stage, current, best, worst));
}

// ---------- cx07_9: the League base constructor and a fixed fixture set by the clubs' names (league.cpp) ----------

/// The League base vtable (League::League writes it, League::~League writes it back).
const LEAGUE_VTABLE: usize = 0x96d020;
/// The League object size (the bytes League::League and the display copy take, through `league::FIXED_FIXTURES`).
const LEAGUE_OBJECT_SIZE: usize = 0xee;

/// Builds the League base of every league, cup stage group and national-team tournament: the Comp base
/// (`exe::new_comp`, Comp::Comp, thiscall, through its detour), the dates `league::LAST_RESULT_DATE` and
/// `league::LAST_UPDATE_DATE` (Date::Date 0x54a110, thiscall), the League vtable, a league (`league::COMP_TYPE` 2) with
/// the match category 2 and the prize mode 2; no draw day (`league::NEXT_DRAW_DAY` −1), no games per club, required
/// capacity or seats (−1 each); the rule bits `league::LEAGUE_FLAGS` 3 (champion and promotion statuses); no stages,
/// table, custom compare, tie-break codes, fixed fixtures or tie-break matches; not stopped nor complete. Returns
/// `this`. The original's SEH frame covers only the unwinding of the base.
/// Original: `League::League` (0x682200, league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn new_league(this: usize) -> usize {
    covered!("cx07_9 league built");
    exe::new_comp(this);
    exe::date_date_54a110(this + league::LAST_RESULT_DATE);
    exe::date_date_54a110(this + league::LAST_UPDATE_DATE);
    write_u8(this + league::COMP_TYPE, 2);
    write_u8(this + league::MATCH_CATEGORY, 2);
    write_u8(this + league::PRIZE_MODE, 2);
    for field in [league::NEXT_DRAW_DAY, league::GAMES_PER_CLUB, league::REQUIRED_CAPACITY, league::REQUIRED_SEATS] {
        write_u16(this + field, 0xffff);
    }
    write_usize(this, LEAGUE_VTABLE);
    for field in [league::MATCH_RULES, league::ROUND_DRAW_RULES, league::NEXT_DRAW_YEAR_OFFSET] {
        write_u16(this + field, 0);
    }
    for field in [league::TABLE_COLUMN, league::STAGES, league::TABLE, league::TIEBREAK_MATCHES, league::FIXED_FIXTURES, league::COMPLETE,
        league::TIEBREAKS, league::CUSTOM_COMPARE] {
        write_usize(this + field, 0);
    }
    write_u8(this + league::STOPPED, 0);
    write_u16(this + league::LEAGUE_FLAGS, 3); // cx07_9: the default rule bits
    this
}

/// Comparer for `new_league`: side by side over the 0xee bytes of the object and the return.
pub(crate) unsafe extern "thiscall" fn compare_new_league(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_NEW_LEAGUE);
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&[(this, LEAGUE_OBJECT_SIZE)], || o(this), || new_league(this));
    SCOREBOARD.record(same && r == n, || format!("new_league({:#x}): original {:x?} rust {:x?}", this, mo, mr));
    r
}

/// A fixed fixture of a league (an entry of `league::FIXED_FIXTURES`, 0x10 bytes) from its date and the two clubs'
/// names: the date (`exe::date_set`(entry, day, month, year, −1), Date::set, thiscall, ret 0x10) at +0, the home club id
/// at +8 and `fixed_fixture::AWAY_CLUB_ID` −1; then the league's table rows in order, comparing each club's short name
/// (`club_get_english_short_name`, the English one when another language is loaded) with `home` and with `away` (byte
/// by byte, as strcmp); a match stores the club's id; the search stops once both are found. A name that matches no
/// club leaves −1.
/// Original: `fixed_fixture_set_by_club_names` (0x68aff0, league.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn fixed_fixture_set_by_club_names(this: usize, league: usize, day: u32, month: u32, year: u32, home: usize, away: usize) {
    exe::date_set(this, day, month, year, -1);
    let home_id = this + 8; // the home club id of the fixed fixture
    write_u32(this + fixed_fixture::AWAY_CLUB_ID, u32::MAX);
    write_u32(home_id, u32::MAX);
    let league = League(league);
    let mut i = 0i32;
    while i < league.club_count() as i32 {
        let club = read_usize(league.table() + i as usize * league_table_row::SIZE + league_table_row::CLUB);
        if guio_same_c_string(club_get_english_short_name(club), home) {
            write_u32(home_id, read_u32(club)); // cx07_9: the home club's id
        }
        if guio_same_c_string(club_get_english_short_name(club), away) {
            write_u32(this + fixed_fixture::AWAY_CLUB_ID, read_u32(club));
        }
        if read_i32(home_id) != -1 && read_i32(this + fixed_fixture::AWAY_CLUB_ID) != -1 {
            break;
        }
        i += 1;
    }
}

/// Comparer for `fixed_fixture_set_by_club_names`: side by side over the 0x10 bytes of the entry.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_fixed_fixture_set_by_club_names(this: usize, league: usize, day: u32, month: u32, year: u32, home: usize, away: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, usize)>(IDX_FIXED_FIXTURE_SET_BY_CLUB_NAMES);
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&[(this, 0x10)], || o(this, league, day, month, year, home, away),
        || fixed_fixture_set_by_club_names(this, league, day, month, year, home, away));
    SCOREBOARD.record(same, || format!("fixed_fixture_set_by_club_names({:#x}, {:?} v {:?}): original {:x?} rust {:x?}", this,
        String::from_utf8_lossy(&c_string_bytes(home)), String::from_utf8_lossy(&c_string_bytes(away)), mo, mr));
}

// ---------- cx08_13: the Australian league's fate news text, club progress and bench (aus_nsl.cpp) ----------

/// Rule: the news of a club's fate in the Australian league: through from the regular season, "{club} through to
/// playoff phase"; through from the finals series (stage 0), "{club} through to final"; in the grand final, the cup
/// lines of a tie.
///
/// `AusNsl::build_fate_news_text(club, outcome, stage, a4, fixture, a6, a7, a8, body, text)`: at the stage byte 0xff
/// or 0 only outcome 6 writes: with `body` the group news (`exe::comp_util_write_group_qualify_news`(club, a4, the
/// competition record, 6, body, text)), without it `group_qualify_text` with 0x987c88 (0xff) or 0x987c58 (0). Any other
/// stage: with `body` `exe::comp_util_write_cup_tie_result_news`(club, fixture, a8, outcome, a6, text), without it
/// `exe::comp_util_write_cup_win_or_defeat_news`(club, fixture, outcome, text).
/// Original: `AusNsl::build_fate_news_text` (0x411a30, aus_nsl.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn aus_nsl_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, fixture: usize, a6: u32, _a7: u32, a8: u32, body: u32, text: usize) {
    let format = match stage as u8 {
        0xff => 0x987c88, // "{club} through to playoff phase"
        0 => 0x987c58, // "{club} through to final"
        _ => {
            if body != 0 {
                return exe::comp_util_write_cup_tie_result_news(club_id, fixture as u32, a8, outcome, a6, text);
            }
            return exe::comp_util_write_cup_win_or_defeat_news(club_id, fixture as u32, outcome, text);
        }
    };
    if outcome as u8 != 6 {
        return;
    }
    if body != 0 {
        return exe::comp_util_write_group_qualify_news(club_id, a4, read_usize(this + league::COMPETITION), 6, body, text);
    }
    covered!("cx08_13 australian qualify text");
    group_qualify_text(club_id, format, text);
}

/// Comparer for `aus_nsl_build_fate_news_text`: the text-only paths (outcome 6 without `body` at stage −1 or 0) side by
/// side (`compare_text_writer`); the news writers only the original.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_aus_nsl_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, fixture: usize, a6: u32, a7: u32, a8: u32, body: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, usize, u32, u32, u32, u32, usize)>(IDX_AUS_NSL_BUILD_FATE_NEWS_TEXT);
    if text == 0 || body != 0 || outcome as u8 != 6 || !matches!(stage as u8, 0xff | 0) {
        return o(this, club_id, outcome, stage, a4, fixture, a6, a7, a8, body, text);
    }
    compare_text_writer(text, || o(this, club_id, outcome, stage, a4, fixture, a6, a7, a8, body, text),
        |t| aus_nsl_build_fate_news_text(this, club_id, outcome, stage, a4, fixture, a6, a7, a8, body, t),
        || format!("aus_nsl_build_fate_news_text(club {}, outcome {}, stage {})", club_id, outcome as u8, stage as u8 as i8));
}

/// Rule: the Australian league's progress codes: in the regular season the top two become 1 (best and current) or 7
/// (worst) and third 9; in the finals series (stages below 1) places 1 and 2 become 1 (current, best) or 3 (worst), 3
/// becomes 5 and 4 becomes 7; in the grand final as they come.
///
/// `AusNsl::set_club_progress(club, stage, current, best, worst)`: the club's entry (`exe::comp_stats_find_club_entry`,
/// unchecked). Stage byte 0xff: current 1, 2 → 1 and 3 → 9; best 1, 2 → 1 and 3 → 9; worst 1, 2 → 7 and 3 → 9. Other
/// stages below 1 (signed; the jump tables 0x4124ac, 0x4124bc, 0x4124cc): current 1, 2 → 1, 3 → 5, 4 → 7; best the same;
/// worst 1, 2 → 3, 3 → 5, 4 → 7. Any other code, and every code from stage 1, stays. The bytes go to
/// `comp_stats_club::BEST_POSITION`, `CURRENT_POSITION` and `WORST_POSITION`.
/// Original: `AusNsl::set_club_progress` (0x412380, aus_nsl.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn aus_nsl_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let e = exe::comp_stats_find_club_entry(League(this).comp_stats(), club);
    let s = stage as u8;
    let code = |v: u32, table: [u8; 4]| -> u8 {
        let v = v as u8;
        if (1..=4).contains(&v) && table[v as usize - 1] != 0 { table[v as usize - 1] } else { v }
    };
    let (c, b, w) = if s == 0xff {
        covered!("cx08_13 australian season progress");
        (code(current, [1, 1, 9, 0]), code(best, [1, 1, 9, 0]), code(worst, [7, 7, 9, 0]))
    } else if (s as i8) < 1 {
        covered!("cx08_13 australian finals progress");
        (code(current, [1, 1, 5, 7]), code(best, [1, 1, 5, 7]), code(worst, [3, 3, 5, 7]))
    } else {
        (current as u8, best as u8, worst as u8)
    };
    write_u8(e + comp_stats_club::BEST_POSITION, b);
    write_u8(e + comp_stats_club::CURRENT_POSITION, c);
    write_u8(e + comp_stats_club::WORST_POSITION, w);
}

/// Comparer for `aus_nsl_set_club_progress`: side by side on the 3 bytes (`compare_group_championship_progress`).
pub(crate) unsafe extern "thiscall" fn compare_aus_nsl_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(IDX_AUS_NSL_SET_CLUB_PROGRESS);
    compare_group_championship_progress("aus_nsl_set_club_progress", this, club, || o(this, club, stage, current, best, worst),
        || aus_nsl_set_club_progress(this, club, stage, current, best, worst));
}

/// Rule: the Australian league's bench: a fixture of a stage (the finals series and the grand final) names 5
/// substitutes and allows 3; a league fixture takes the league's numbers.
///
/// `AusNsl::set_fixture_substitutes(fixture)`: with the fixture's `fixture::STAGE_INDEX` (signed) at 0 or above, the
/// low byte of `fixture::SQUAD_RULES` becomes 0x35 (the word & 0xff35 | 0x35); otherwise the league's
/// `league::SUBSTITUTES_NAMED` (& 0xf) into the low nibble and `league::SUBSTITUTIONS_ALLOWED` (& 0xf) into the next,
/// the other bits kept.
/// Original: `AusNsl::set_fixture_substitutes` (0x4124e0, aus_nsl.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn aus_nsl_set_fixture_substitutes(this: usize, fixture: usize) {
    let (l, fixture) = (League(this), Fixture(fixture));
    if fixture.stage_index() as i8 >= 0 {
        covered!("cx08_13 australian stage bench");
        return fixture.set_squad_rules((fixture.squad_rules() & 0xff35) | 0x35);
    }
    fixture.set_squad_rules((fixture.squad_rules() & 0xfff0) | (l.substitutes_named() & 0xf) as u16);
    fixture.set_squad_rules((fixture.squad_rules() & 0xff0f) | ((l.substitutions_allowed() & 0xf) as u16) << 4);
}

/// Comparer for `aus_nsl_set_fixture_substitutes`: the fixture's `SQUAD_RULES` word side by side.
pub(crate) unsafe extern "thiscall" fn compare_aus_nsl_set_fixture_substitutes(this: usize, fixture: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_AUS_NSL_SET_FIXTURE_SUBSTITUTES);
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&[(fixture + fixture::SQUAD_RULES, 2)], || o(this, fixture), || aus_nsl_set_fixture_substitutes(this, fixture));
    SCOREBOARD.record(same, || format!("aus_nsl_set_fixture_substitutes({:#x}): original {:x?} rust {:x?}", fixture, mo, mr));
}

// ---------- cx09_16: the Brazilian Serie B staff records and progress, and the Serie C stage advance (bra_nat_second.cpp, bra_nat_third.cpp) ----------

/// Updates the staff records by a club's outcome in the Brazilian Serie B, on the Staff_Records object
/// (`G_STAFF_RECORDS`) with this competition. In the league (stage 0xff): status 0 promotion (weight 0x64), status 3
/// relegation. In the play-off (stage 0), status 1: the club's row (League::find_club_row `exe::league_find_club_row`,
/// not checked: a club without a row would crash the original) and the champions record, then the row's status 5.
/// Returns 0.
/// Original: `BraNatSecond::update_staff_records_for_status` (0x42d040, bra_nat_second.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn bra_nat_second_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, _a4: u32, _a5: u32, _a6: u32) -> u32 {
    let l = League(this);
    let (records, comp) = (read_usize(G_STAFF_RECORDS), l.competition().addr());
    match (stage as u8, status as u8) {
        (0xff, 0) => {
            covered!("cx09_16 serie b promotion");
            exe::staff_records_add_promotion_record(records, club, comp, 0x64);
        }
        (0xff, 3) => exe::staff_records_add_relegation_record(records, club, comp),
        (0, 1) => {
            covered!("cx09_16 serie b champions");
            let row = exe::league_find_club_row(this, club, 0);
            exe::staff_records_add_league_champions_record(records, club, comp);
            LeagueTableRow(row).set_status(5);
        }
        _ => {}
    }
    0
}

/// The progress codes of the Brazilian leagues that only rewrite them in the league stage: the club's Comp_Stats entry
/// (`bra_reg_minas_gerais_comp_stats_club_entry`, unchecked); at stage 0xff a best code below `limit` becomes 1 and a
/// worst code below `limit` becomes `low_worst` (unsigned bytes); then the three codes (`write_progress_codes`).
/// Returns the entry.
unsafe fn league_stage_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32, limit: u8, low_worst: u8) -> usize {
    let e = bra_reg_minas_gerais_comp_stats_club_entry(this, club);
    let (c, mut b, mut w) = (current as u8, best as u8, worst as u8);
    if stage as u8 == 0xff {
        if b < limit {
            b = 1;
        }
        if w < limit {
            w = low_worst;
        }
    }
    write_progress_codes(e, c, b, w);
    e
}

/// The comparer of `league_stage_progress`: side by side on the entry's 3 bytes (`compare_group_championship_progress`),
/// returning the original's value.
unsafe fn compare_league_stage_progress(name: &str, this: usize, club: usize, original: impl FnOnce() -> usize, copy: impl FnOnce() -> usize) -> usize {
    let mut r = 0;
    compare_group_championship_progress(name, this, club, || r = original(), || {
        copy();
    });
    r
}

/// Stores a club's progress in the Brazilian Serie B (`league_stage_progress`: in the league a best code below 5 becomes
/// 1 and a worst code below 5 becomes 4).
/// Original: `BraNatSecond::set_club_progress` (0x42d260, bra_nat_second.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn bra_nat_second_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) -> usize {
    covered!("cx09_16 serie b progress");
    league_stage_progress(this, club, stage, current, best, worst, 5, 4)
}

/// Moves the Brazilian Serie C to its next stage: past the last stage (`league::CURRENT_STAGE` at or past
/// `league::STAGE_COUNT` − 1) the bra_nat_third.cpp box of line 0x2b7; otherwise the current stage goes up by one and
/// stage 7 builds the second round (0x42e430), stage 8 the third-round groups (0x42e700) and stage 10 the final
/// (0x42e980), through their detours; stage 9 builds nothing.
/// Original: `BraNatThird::advance_stage` (0x42e380, bra_nat_third.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_nat_third_advance_stage(this: usize) {
    let l = League(this);
    if l.current_stage() as i32 >= l.stage_count() as i32 - 1 {
        return show_error_at(0x98aba4, 0x2b7); // bra_nat_third.cpp
    }
    l.set_current_stage((l.current_stage() as i32 + 1) as usize);
    let build = match l.current_stage() as i32 {
        7 => 0x42e430usize,
        8 => 0x42e700,
        10 => 0x42e980,
        _ => return,
    };
    covered!("cx09_16 serie c next stage");
    let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(build);
    f(this);
}

/// Comparer for `bra_nat_second_update_staff_records_for_status`: only the original (it adds the records); it must
/// return 0 and, for the play-off champion, leave the row with status 5.
pub(crate) unsafe extern "thiscall" fn compare_bra_nat_second_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, a4: u32, a5: u32, a6: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) -> u32>(IDX_BRA_NAT_SECOND_UPDATE_STAFF_RECORDS_FOR_STATUS)(this, club, status, stage, a4, a5, a6);
    let row_ok = (stage as u8, status as u8) != (0, 1) || LeagueTableRow(exe::league_find_club_row(this, club, 0)).status() == 5;
    SCOREBOARD.record(r == 0 && row_ok, || format!("bra_nat_second_update_staff_records_for_status({:#x}, {:#x}, status {}, stage {:#x}): returned {}, row ok {}", this, club, status as u8, stage as u8, r, row_ok));
    r
}

pub(crate) unsafe extern "thiscall" fn compare_bra_nat_second_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) -> usize>(IDX_BRA_NAT_SECOND_SET_CLUB_PROGRESS);
    compare_league_stage_progress("bra_nat_second_set_club_progress", this, club, || o(this, club, stage, current, best, worst),
        || bra_nat_second_set_club_progress(this, club, stage, current, best, worst))
}

/// Comparer for `bra_nat_third_advance_stage` (`compare_state_league_advance_stage`).
pub(crate) unsafe extern "thiscall" fn compare_bra_nat_third_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_NAT_THIRD_ADVANCE_STAGE);
    compare_state_league_advance_stage("bra_nat_third_advance_stage", League(this), || o(this))
}

// ---------- cx09_17: the progress of the Gaucho and Goias state championships (bra_reg_gaucho.cpp, bra_reg_goias.cpp) ----------

/// Stores a club's progress in the Rio Grande do Sul state championship (`league_stage_progress`: in the league a best
/// code below 5 becomes 1 and a worst code below 5 becomes 3).
/// Original: `BraRegGaucho::set_club_progress` (0x432af0, bra_reg_gaucho.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn bra_reg_gaucho_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) -> usize {
    covered!("cx09_17 gaucho progress");
    league_stage_progress(this, club, stage, current, best, worst, 5, 3)
}

/// Stores a club's progress in the Goias state championship (`league_stage_progress`: in the league a best code below 9
/// becomes 1 and a worst code below 9 becomes 5).
/// Original: `BraRegGoias::set_club_progress` (0x433cf0, bra_reg_goias.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn bra_reg_goias_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) -> usize {
    covered!("cx09_17 goias progress");
    league_stage_progress(this, club, stage, current, best, worst, 9, 5)
}

pub(crate) unsafe extern "thiscall" fn compare_bra_reg_gaucho_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) -> usize>(IDX_BRA_REG_GAUCHO_SET_CLUB_PROGRESS);
    compare_league_stage_progress("bra_reg_gaucho_set_club_progress", this, club, || o(this, club, stage, current, best, worst),
        || bra_reg_gaucho_set_club_progress(this, club, stage, current, best, worst))
}

pub(crate) unsafe extern "thiscall" fn compare_bra_reg_goias_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) -> usize>(IDX_BRA_REG_GOIAS_SET_CLUB_PROGRESS);
    compare_league_stage_progress("bra_reg_goias_set_club_progress", this, club, || o(this, club, stage, current, best, worst),
        || bra_reg_goias_set_club_progress(this, club, stage, current, best, worst))
}

// ---------- cx11_10: the fate news text of the Serie C1 and C2 groups (ita_ser_c2a.cpp) ----------

/// "{club} win promotion" (langlib template with the club's short name and gender).
const WIN_PROMOTION_TEXT: usize = 0x989c18;
/// "{club} relegated".
const RELEGATED_TEXT: usize = 0x987564;

/// Writes the news text of a club's fate in a Serie C1 or C2 group (the five groups share it at virtual +0x48).
///
/// With a news body (`news`), the base text (`exe::comp_build_fate_news_text`) with a7 = 0. Without one: outcome 1 in
/// stage 0 (the promotion play-off won), "{club} win promotion"; outcome 0xff in stage 1 (the relegation play-out lost),
/// "{club} relegated"; both with the club's short name and gender byte (`group_qualify_text`: the shared buffer, the
/// Text and the club's link 0x7d5). Any other outcome or stage: the base text with a7 = 0 and no news. The outcome and
/// the stage are compared as bytes; the base text gets them as the caller passed them.
/// Original: `ItaSerC2a::build_fate_news_text` (0x654390, ita_ser_c2a.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn ita_ser_c_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, fixture: usize, a6: u32, _a7: u32, a8: u32, news: u32, text: usize) {
    if news != 0 {
        return exe::comp_build_fate_news_text(this, club_id, outcome, stage as i32, a4, fixture, a6, 0, a8, news, text);
    }
    covered!("cx11_10 serie c own fate text");
    match (outcome as u8, stage as u8) {
        (1, 0) => group_qualify_text(club_id, WIN_PROMOTION_TEXT, text), // cx11_10: the play-off winner goes up
        (0xff, 1) => group_qualify_text(club_id, RELEGATED_TEXT, text),
        _ => exe::comp_build_fate_news_text(this, club_id, outcome, stage as i32, a4, fixture, a6, 0, a8, 0, text),
    }
}

/// Comparer for `ita_ser_c_build_fate_news_text`: the two text-only paths side by side (`compare_text_writer`; with the
/// CM_TEXTO_NOTICIAS mode); the base text only the original.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_ita_ser_c_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, fixture: usize, a6: u32, a7: u32, a8: u32, news: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, usize, u32, u32, u32, u32, usize)>(IDX_ITA_SER_C_BUILD_FATE_NEWS_TEXT);
    let own = matches!((outcome as u8, stage as u8), (1, 0) | (0xff, 1));
    if news != 0 || text == 0 || !own {
        return o(this, club_id, outcome, stage, a4, fixture, a6, a7, a8, news, text);
    }
    compare_text_writer(text, || o(this, club_id, outcome, stage, a4, fixture, a6, a7, a8, news, text),
        |t| ita_ser_c_build_fate_news_text(this, club_id, outcome, stage, a4, fixture, a6, a7, a8, news, t),
        || format!("ita_ser_c_build_fate_news_text(club {}, outcome {}, stage {})", club_id, outcome as u8, stage as u8 as i8));
}

// ---------- cx12_7: the J1 League staff records by outcome, fate news text and club progress (jap_j1.cpp) ----------

/// The staff record a J1 League outcome adds.
#[derive(Clone, Copy, PartialEq, Debug)]
enum J1Record {
    None,
    /// A stage won (Staff_Records::add_qualified_for_round_record with 0, 0x96, 0x28).
    StageWinner,
    Champions,
    /// The championship final lost (Staff_Records::add_knocked_out_record with 0, 0x96, 0x50).
    FinalLoser,
}

/// What `jap_j1_update_staff_records_for_status` does for an outcome: the record, the status of the club's row in the
/// main table (`row`, 0 when the club has none), whether the club's rows in the two stage tables also get status 5, the
/// new `league::STAGE_COUNT`, whether the caller's flag is set, and the value returned.
#[derive(Clone, Copy, Debug)]
struct J1StatusPlan {
    record: J1Record,
    row: usize,
    status: Option<u8>,
    stage_rows: bool,
    stage_count: Option<usize>,
    flag: bool,
    result: u32,
}

/// The plan of the J1 League status update (no side effects). The stage and the status are bytes.
///
/// Stage 0 (the first stage): status 6 (stage won) is the stage winner record, the flag and the club's main row status
/// 6; 0. Stage 1 (the second stage), whatever the status: when the club also topped the first stage (row 0 of the first
/// stage's table), it is champion with no final: its main row and its rows in both stage tables get status 5, the
/// champions record, `league::STAGE_COUNT` = 2 and the flag; 1. Otherwise the stage winner record, the flag and the
/// main row gets the status passed; 2. Stage 2 (the championship final): status 1 (final won) is the champions record,
/// the same status 5 rows and the flag; 0; status 2 (final lost) is the final loser record; 0. Anything else: 0.
unsafe fn j1_status_plan(l: League, club: usize, status: u8, stage: u8) -> J1StatusPlan {
    let row = find_club_row(l.addr(), club, 0);
    let plan = |record, status: Option<u8>, stage_rows, stage_count, flag, result| J1StatusPlan { record, row, status, stage_rows, stage_count, flag, result };
    let nothing = plan(J1Record::None, None, false, None, false, 0);
    match stage {
        0 if status == 6 => plan(J1Record::StageWinner, Some(6), false, None, true, 0),
        1 => {
            let first_stage = League(read_usize(l.stages()));
            if read_usize(first_stage.table()) == club { // cx12_7: the first stage winner too
                plan(J1Record::Champions, Some(5), true, Some(2), true, 1)
            } else {
                plan(J1Record::StageWinner, Some(status), false, None, true, 2)
            }
        }
        2 if status == 1 => plan(J1Record::Champions, Some(5), true, None, true, 0),
        2 if status == 2 => plan(J1Record::FinalLoser, None, false, None, false, 0),
        _ => nothing,
    }
}

/// Updates the staff records by a club's outcome in the J1 League (`j1_status_plan`): the club's rows first, then the
/// record on the Staff_Records object (`G_STAFF_RECORDS`) with this competition, `league::STAGE_COUNT` and the byte
/// `flag` points to (set to 1). The rows of the two stage tables are scanned with the main table's club count. Returns
/// the plan's value.
/// Original: `JapJ1::update_staff_records_for_status` (0x663db0, jap_j1.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn jap_j1_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, flag: usize, _a5: u32, _a6: u32) -> u32 {
    let l = League(this);
    let p = j1_status_plan(l, club, status as u8, stage as u8);
    if let (Some(s), true) = (p.status, p.row != 0) {
        LeagueTableRow(p.row).set_status(s);
    }
    if p.stage_rows {
        covered!("cx12_7 j1 champions rows");
        for k in 0..l.club_count().max(0) as usize {
            for stage_league in [read_usize(l.stages()), read_usize(l.stages() + 4)] {
                let row = LeagueTableRow(League(stage_league).table() + k * league_table_row::SIZE);
                if row.club() as usize == club {
                    row.set_status(5);
                }
            }
        }
    }
    let (records, comp) = (read_usize(G_STAFF_RECORDS), l.competition().addr());
    match p.record {
        J1Record::None => {}
        J1Record::StageWinner => exe::staff_records_add_qualified_for_round_record(records, club, comp, 0, 0x96, 0x28),
        J1Record::Champions => exe::staff_records_add_league_champions_record(records, club, comp),
        J1Record::FinalLoser => exe::staff_records_add_knocked_out_record(records, club, comp, 0, 0x96, 0x50),
    }
    if let Some(n) = p.stage_count {
        l.set_stage_count(n);
    }
    if p.flag {
        write_u8(flag, 1);
    }
    p.result
}

/// "{club} win first stage", "{club} win second stage": the short texts, with the club only.
const WIN_FIRST_STAGE_TEXT: usize = 0x9eae2c;
const WIN_SECOND_STAGE_TEXT: usize = 0x9eacc0;
/// "{club} have won the first stage of the {league}.", "{club} have won the {league} by finishing top of both
/// stages.", "{club} have qualified for the final of the {league} having won the second stage of the competition.":
/// the long texts.
const WON_FIRST_STAGE_TEXT: usize = 0x9eae64;
const WON_BOTH_STAGES_TEXT: usize = 0x9eada8;
const REACHED_FINAL_TEXT: usize = 0x9eacfc;
/// "{club} win {league}", "{club} lose {league} final".
const WIN_LEAGUE_TEXT: usize = 0x9eac64;
const LOSE_FINAL_TEXT: usize = 0x9eac00;

/// Writes a J1 fate text into `text`: `template` (langlib_sprintf into the shared buffer) with the club's gender
/// (`club::SHORT_GENDER_NAME`, signed) twice, then, with `with_league`, the competition's gender twice, then the club's
/// short name and, with `with_league`, the competition's short name; set as the text, with the club's link (0x7d5) and
/// the competition's (0x7d0) in both forms.
unsafe fn j1_fate_text(club_id: u32, comp: usize, template: usize, with_league: bool, text: usize) {
    let club = club_at(club_id as isize);
    let g = club.short_gender_name() as i8 as i32;
    let name = club.addr() + club::SHORT_NAME;
    if with_league {
        let cg = read_u8(comp + competition::SHORT_GENDER_NAME) as i8 as i32;
        story_lang!(PRAISE_TEXT_BUFFER, template, g, g, cg, cg, name, comp + competition::SHORT_NAME);
    } else {
        story_lang!(PRAISE_TEXT_BUFFER, template, g, g, name);
    }
    exe::text_set_string(text, PRAISE_TEXT_BUFFER as *const u8);
    exe::text_add_link(text, name as *const u8, 0x7d5, club);
    exe::text_add_link(text, (comp + competition::SHORT_NAME) as *const u8, 0x7d0, comp);
}

/// The text (template, with the league) a J1 fate gets, or None for the base text or nothing (see
/// `jap_j1_build_fate_news_text`).
fn j1_fate_template(stage: u8, outcome: u8, update_result: u32, news: bool) -> Option<(usize, bool)> {
    match (stage, outcome) {
        (0, 6) if news => Some((WON_FIRST_STAGE_TEXT, true)),
        (0, 6) => Some((WIN_FIRST_STAGE_TEXT, false)),
        (1, 6) => match (update_result, news) {
            (1, true) => Some((WON_BOTH_STAGES_TEXT, true)),
            (1, false) => Some((WIN_LEAGUE_TEXT, true)),
            (2, true) => Some((REACHED_FINAL_TEXT, true)),
            (2, false) => Some((WIN_SECOND_STAGE_TEXT, false)),
            _ => None,
        },
        (2, 1) if !news => Some((WIN_LEAGUE_TEXT, true)), // cx12_7: the final won
        (2, _) if !news => Some((LOSE_FINAL_TEXT, true)),
        _ => None,
    }
}

/// Writes the news text of a club's fate in the J1 League. Stage 0xff (the whole season): the base text
/// (`exe::comp_build_fate_news_text`) with stage −1 and a7 = 0. Stage 0, outcome 6: the first stage won (the long text
/// with `news`, else the short one). Stage 1, outcome 6, by the status update's result (`update_result`, the 7th
/// argument): 1, top of both stages (the long text with `news`, else "win {league}"); 2, the final reached (the long text
/// with `news`, else "win second stage"). Stage 2 with `news`: the base text with the outcome (1 when it is 1), stage 2
/// and a7 = 0; without it, outcome 1 "win {league}", any other "lose {league} final". Everything else writes nothing.
/// The stage and the outcome are compared as bytes.
/// Original: `JapJ1::build_fate_news_text` (0x6640a0, jap_j1.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn jap_j1_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, fixture: usize, a6: u32, update_result: u32, a8: u32, news: u32, text: usize) {
    let comp = League(this).competition().addr();
    let (s, o) = (stage as u8, outcome as u8);
    if s == 0xff {
        return exe::comp_build_fate_news_text(this, club_id, outcome, -1, a4, fixture, a6, 0, a8, news, text);
    }
    if s == 2 && news != 0 {
        let outcome = if o == 1 { 1 } else { outcome };
        return exe::comp_build_fate_news_text(this, club_id, outcome, 2, a4, fixture, a6, 0, a8, news, text);
    }
    if let Some((template, with_league)) = j1_fate_template(s, o, update_result, news != 0) {
        covered!("cx12_7 j1 own fate text");
        j1_fate_text(club_id, comp, template, with_league, text);
    }
}

/// The club's place forecast in the J1 League: its entry in the competition statistics (`exe::comp_stats_find_club_entry`,
/// unchecked) gets the current, best and worst codes. At the stage byte 0xff (the whole season), a club whose main table
/// row has status 1 writes nothing; otherwise best 2..=5 becomes 2 and worst 2..=5 becomes 5. Other stages write the
/// codes as they come.
///
/// Defect of the original: the row status is kept in the stack slot of the `club` argument (its low byte); without a
/// row in the table that byte is the low byte of the club pointer, so a club with no row whose pointer ends in 0x01
/// writes nothing. The copy reproduces it.
/// Original: `JapJ1::set_club_progress` (0x664400, jap_j1.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn jap_j1_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let l = League(this);
    let e = exe::comp_stats_find_club_entry(l.comp_stats(), club);
    let row = find_club_row(this, club, 0);
    let status = if row != 0 { LeagueTableRow(row).status() } else { club as u8 };
    let (c, mut b, mut w) = (current as u8, best as u8, worst as u8);
    if stage as u8 == 0xff {
        if status == 1 {
            return;
        }
        covered!("cx12_7 j1 season progress");
        if (2..=5).contains(&b) {
            b = 2;
        }
        if (2..=5).contains(&w) { // cx12_7: the worst code of the band
            w = 5;
        }
    }
    write_u8(e + comp_stats_club::CURRENT_POSITION, c);
    write_u8(e + comp_stats_club::BEST_POSITION, b);
    write_u8(e + comp_stats_club::WORST_POSITION, w);
}

/// Comparer for `jap_j1_update_staff_records_for_status`: only the original (it adds the records); the copy's plan,
/// made before, must give the same return, the same main row status, the stage count and the flag.
pub(crate) unsafe extern "thiscall" fn compare_jap_j1_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, flag: usize, a5: u32, a6: u32) -> u32 {
    let l = League(this);
    let p = j1_status_plan(l, club, status as u8, stage as u8);
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, u32, u32) -> u32>(IDX_JAP_J1_UPDATE_STAFF_RECORDS_FOR_STATUS)(this, club, status, stage, flag, a5, a6);
    let row_ok = match (p.status, p.row) {
        (Some(s), row) if row != 0 => LeagueTableRow(row).status() == s,
        _ => true,
    };
    let count_ok = p.stage_count.map_or(true, |n| l.stage_count() == n);
    let flag_ok = !p.flag || read_u8(flag) == 1;
    SCOREBOARD.record(r == p.result && row_ok && count_ok && flag_ok, || format!("jap_j1_update_staff_records_for_status({:#x}, {:#x}, status {}, stage {}): original {} copy {:?}", this, club, status as u8, stage as u8, r, p));
    r
}

/// Comparer for `jap_j1_build_fate_news_text`: the text-only paths side by side (`compare_text_writer`; with the
/// CM_TEXTO_NOTICIAS mode); the base text only the original.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_jap_j1_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, fixture: usize, a6: u32, update_result: u32, a8: u32, news: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, usize, u32, u32, u32, u32, usize)>(IDX_JAP_J1_BUILD_FATE_NEWS_TEXT);
    if text == 0 || j1_fate_template(stage as u8, outcome as u8, update_result, news != 0).is_none() {
        return o(this, club_id, outcome, stage, a4, fixture, a6, update_result, a8, news, text);
    }
    compare_text_writer(text, || o(this, club_id, outcome, stage, a4, fixture, a6, update_result, a8, news, text),
        |t| jap_j1_build_fate_news_text(this, club_id, outcome, stage, a4, fixture, a6, update_result, a8, news, t),
        || format!("jap_j1_build_fate_news_text(club {}, outcome {}, stage {}, result {}, news {})", club_id, outcome as u8, stage as u8 as i8, update_result, news));
}

/// Comparer for `jap_j1_set_club_progress`: side by side on the 3 bytes of the club's entry (`compare_c1_club_progress`).
pub(crate) unsafe extern "thiscall" fn compare_jap_j1_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(IDX_JAP_J1_SET_CLUB_PROGRESS);
    compare_c1_club_progress("jap_j1_set_club_progress", this, club, || o(this, club, stage, current, best, worst), || jap_j1_set_club_progress(this, club, stage, current, best, worst))
}

// ---------- cx12_8: the J1 League season honours (jap_j1.cpp) ----------

/// Records the J1 League's season honours. When only two stages ran (`league::STAGE_COUNT` = 2: the same club won both
/// stages, no final), the history record (`exe::comp_util_add_history_record`(this, champion, runner-up, 0, 0)) with the
/// first stage's top club as champion and as runner-up the club second in whichever stage table has more points in its
/// second row (the first stage's on a tie; `exe::league_get_table_row`(stage, 1)). Otherwise the final stage (stage 2)
/// records them: its virtual `comp_vtable::RECORD_SEASON_HONOURS` (a tail jump in the original), whose value is
/// returned; the history path returns the record's value.
/// Original: `JapJ1::record_season_honours` (0x664390, jap_j1.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn jap_j1_record_season_honours(this: usize) -> usize {
    let l = League(this);
    let stages = l.stages();
    if l.stage_count() != 2 {
        let last = read_usize(stages + 8);
        let honours: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(read_usize(read_usize(last) + comp_vtable::RECORD_SEASON_HONOURS));
        return honours(last);
    }
    covered!("cx12_8 j1 won both stages");
    let (first, second) = (read_usize(stages), read_usize(stages + 4));
    let champion = read_usize(League(first).table());
    let second_row = LeagueTableRow(exe::league_get_table_row(second, 1));
    let first_row = LeagueTableRow(exe::league_get_table_row(first, 1));
    let runner_up_stage = if second_row.points() > first_row.points() { second } else { first }; // cx12_8: the better second place
    let runner_up = LeagueTableRow(exe::league_get_table_row(runner_up_stage, 1)).club() as usize;
    exe::comp_util_add_history_record(this, champion, runner_up, 0, 0)
}

/// Comparer for `jap_j1_record_season_honours`: only the original (it writes the history).
pub(crate) unsafe extern "thiscall" fn compare_jap_j1_record_season_honours(this: usize) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_JAP_J1_RECORD_SEASON_HONOURS)(this)
}

// ---------- cx15_5: the MLS division of a club and the order of the play-off entries by points (usa_mls.cpp) ----------

/// The keys of the twelve MLS clubs of each division (club ids): 0x413 (East: New England, MetroStars, D.C. United,
/// Miami), 0x47b (Central: Columbus, Tampa Bay, Dallas, Chicago) and 0x414 (West: Colorado, Kansas City, Los Angeles,
/// San Jose), in the original's test order.
const MLS_DIVISION_KEYS: [(u16, [usize; 4]); 3] = [
    (0x413, [0x9d0390, 0x9d0394, 0x9d0384, 0x9d0380]),
    (0x47b, [0x9d03a4, 0x9d0388, 0x9d03a0, 0x9d037c]),
    (0x414, [0x9d03a8, 0x9d039c, 0x9d0398, 0x9d038c]),
];

/// The MLS division code of a club: 0 for a club whose league (`.division()`) is missing or is not the MLS
/// (`KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE`); for an MLS club, the code of the division whose keys hold its id
/// (`MLS_DIVISION_KEYS`); an MLS club outside the twelve: the "Error" box (usa_mls.cpp, line 0x71b) and 0. Only the
/// word is the result.
///
/// Called by the MLS table and stages (`create_table`, `create_division_stage`), the All-Star draw and two screens.
/// Original: `usa_mls_get_club_division` (0x909ec0, usa_mls.cpp, cdecl).
pub(crate) unsafe extern "C" fn usa_mls_get_club_division(club: usize) -> u16 {
    let league = Club(club).division().addr();
    if league == 0 || read_usize(league) != read_usize(KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE) {
        return 0;
    }
    let id = read_usize(club);
    for (code, keys) in MLS_DIVISION_KEYS {
        if keys.iter().any(|&k| read_usize(k) == id) {
            covered!("cx15_5 mls division of a club");
            return code;
        }
    }
    show_error_at(0xaa1fb4, 0x71b); // "...\comp\leagues\usa_mls.cpp"
    0
}

/// qsort order of the six-byte play-off entries (club, then the points as a signed word at +4): the most points first
/// (b − a).
/// Original: `usa_mls_compare_points_desc` (0x90a130, usa_mls.cpp, cdecl).
pub(crate) unsafe extern "C" fn usa_mls_compare_points_desc(a: usize, b: usize) -> i32 {
    covered!("cx15_5 play-off entries by points");
    read_i16(b + 4) as i32 - read_i16(a + 4) as i32
}

/// Comparer for `usa_mls_get_club_division`: both run on the same club (an MLS club outside the twelve would show the
/// box twice), the words must be equal.
pub(crate) unsafe extern "C" fn compare_usa_mls_get_club_division(club: usize) -> u16 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u16>(IDX_USA_MLS_GET_CLUB_DIVISION)(club);
    let n = usa_mls_get_club_division(club);
    SCOREBOARD.record(r == n, || format!("usa_mls_get_club_division({:#x}) original={:#x} rust={:#x}", club, r, n));
    r
}

/// Comparer for `usa_mls_compare_points_desc`: both on the same pair, the results must be equal.
pub(crate) unsafe extern "C" fn compare_usa_mls_compare_points_desc(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_USA_MLS_COMPARE_POINTS_DESC)(a, b);
    let n = usa_mls_compare_points_desc(a, b);
    SCOREBOARD.record(r == n, || format!("usa_mls_compare_points_desc({:#x}, {:#x}) original={} rust={}", a, b, r, n));
    r
}

// ---------- cx15_9: the MLS schedule (usa_mls.cpp) ----------

/// The stages of the MLS round table (virtual +0x3c): -3, -2 and -1 the league of 12, 22 or 32 rounds (a league, 0x41
/// bytes per round), 2 the two-round play-off stage and 3 the one-round MLS Cup final (cups, 0x68 bytes per round, with
/// the extra 0xa0 and 0x417); every other stage has no table (`None`: returns 0 without writing anything).
pub(crate) fn usa_mls_schedule_stage(stage: u32) -> Option<ScheduleStage> {
    match stage as u8 as i8 {
        -3 => Some(ScheduleStage { league: 1, rounds: 12, extra: 0, extra_check: false, size: 0x30c, row: 0xb3, ops: &MLS_12_ROUNDS, name: "cx15_9 mls 12 rounds" }),
        -2 => Some(ScheduleStage { league: 1, rounds: 22, extra: 0, extra_check: false, size: 0x596, row: 0xeb, ops: &MLS_22_ROUNDS, name: "cx15_9 mls 22 rounds" }),
        -1 => Some(ScheduleStage { league: 1, rounds: 32, extra: 0, extra_check: false, size: 0x820, row: 0x141, ops: &MLS_32_ROUNDS, name: "cx15_9 mls 32 rounds" }),
        2 => Some(ScheduleStage { league: 0, rounds: 2, extra: 0xa0, extra_check: false, size: 0xd0, row: 0x1b6, ops: &MLS_PLAY_OFF_ROUNDS, name: "cx15_9 mls play-offs" }),
        3 => Some(ScheduleStage { league: 0, rounds: 1, extra: 0x417, extra_check: false, size: 0x68, row: 0x1f2, ops: &MLS_CUP_FINAL_ROUND, name: "cx15_9 mls cup final" }),
        _ => None,
    }
}

/// Builds the date template of an MLS stage (`usa_mls_schedule_stage` run by `create_round_table`): the outputs (the
/// league flag only through a non-null pointer, the rounds, the extra), the table (`exe::crt_malloc`; out of memory, the
/// error box of usa_mls.cpp at the stage's line and 0) and its dates and slots, in the year of the league
/// (`league::YEAR`). The MLS matches themselves are built by hand (`usa_mls_create_season_fixtures`): this table gives
/// the round dates.
/// Original: `UsaMls::create_schedule` (0x9072d0, usa_mls.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn usa_mls_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_round_table(Cup(this), usa_mls_schedule_stage(stage), rounds, extra, league, 0xaa1fb4) // "...\comp\leagues\usa_mls.cpp"
}

/// Comparer for `usa_mls_create_schedule`: side by side (`asia_club_champ_compare_round_table`).
pub(crate) unsafe extern "thiscall" fn compare_usa_mls_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_USA_MLS_CREATE_SCHEDULE);
    asia_club_champ_compare_round_table("usa_mls_create_schedule", stage, usa_mls_schedule_stage(stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| usa_mls_create_schedule(this, stage, r, e, l))
}

/// The 12-round MLS league stage (-3): from 29 February to 30 July, one date per round (weekday 5 and code 1; 10 May
/// and 20 July on weekday 2), each round with one open match slot.
pub(crate) const MLS_12_ROUNDS: [AsiaClubChampScheduleOp; 24] = [
    Cqp11D(0, 0x1d, 2, 0, 5, 1, 0), Cqp11L(0, 0, -1, -1, -1, 0), Cqp11D(1, 0x13, 3, 0, 5, 1, 0),
    Cqp11L(1, 0, -1, -1, -1, 0), Cqp11D(2, 3, 4, 0, 5, 1, 0), Cqp11L(2, 0, -1, -1, -1, 0),
    Cqp11D(3, 0x11, 4, 0, 5, 1, 0), Cqp11L(3, 0, -1, -1, -1, 0), Cqp11D(4, 0x1f, 4, 0, 5, 1, 0),
    Cqp11L(4, 0, -1, -1, -1, 0), Cqp11D(5, 0xa, 5, 0, 2, 2, 0), Cqp11L(5, 0, -1, -1, -1, 0),
    Cqp11D(6, 0x15, 5, 0, 5, 1, 0), Cqp11L(6, 0, -1, -1, -1, 0), Cqp11D(7, 0xc, 6, 0, 5, 1, 0),
    Cqp11L(7, 0, -1, -1, -1, 0), Cqp11D(8, 2, 7, 0, 5, 1, 0), Cqp11L(8, 0, -1, -1, -1, 0),
    Cqp11D(9, 0x10, 7, 0, 5, 1, 0), Cqp11L(9, 0, -1, -1, -1, 0), Cqp11D(0xa, 0x14, 7, 0, 2, 1, 0),
    Cqp11L(0xa, 0, -1, -1, -1, 0), Cqp11D(0xb, 0x1e, 7, 0, 5, 1, 0), Cqp11L(0xb, 0, -1, -1, -1, 0)
];

/// The 22-round MLS league stage (-2): from 22 February to 6 August (weekday 5 and code 1, the midweek rounds on weekday
/// 2 with code 2), each round with one open match slot.
pub(crate) const MLS_22_ROUNDS: [AsiaClubChampScheduleOp; 44] = [
    Cqp11D(0, 0x16, 2, 0, 5, 1, 0), Cqp11L(0, 0, -1, -1, -1, 0), Cqp11D(1, 5, 3, 0, 5, 1, 0),
    Cqp11L(1, 0, -1, -1, -1, 0), Cqp11D(2, 0xc, 3, 0, 5, 1, 0), Cqp11L(2, 0, -1, -1, -1, 0),
    Cqp11D(3, 0x1a, 3, 0, 5, 1, 0), Cqp11L(3, 0, -1, -1, -1, 0), Cqp11D(4, 0xa, 4, 0, 5, 1, 0),
    Cqp11L(4, 0, -1, -1, -1, 0), Cqp11D(5, 0x16, 4, 0, 2, 2, 0), Cqp11L(5, 0, -1, -1, -1, 0),
    Cqp11D(6, 0x18, 4, 0, 5, 1, 0), Cqp11L(6, 0, -1, -1, -1, 0), Cqp11D(7, 0x1c, 4, 0, 2, 2, 0),
    Cqp11L(7, 0, -1, -1, -1, 0), Cqp11D(8, 7, 5, 0, 5, 1, 0), Cqp11L(8, 0, -1, -1, -1, 0),
    Cqp11D(9, 0xe, 5, 0, 5, 1, 0), Cqp11L(9, 0, -1, -1, -1, 0), Cqp11D(0xa, 0x1c, 5, 0, 5, 1, 0),
    Cqp11L(0xa, 0, -1, -1, -1, 0), Cqp11D(0xb, 2, 6, 0, 2, 2, 0), Cqp11L(0xb, 0, -1, -1, -1, 0),
    Cqp11D(0xc, 6, 6, 0, 5, 1, 0), Cqp11L(0xc, 0, -1, -1, -1, 0), Cqp11D(0xd, 0xa, 6, 0, 2, 2, 0),
    Cqp11L(0xd, 0, -1, -1, -1, 0), Cqp11D(0xe, 0x13, 6, 0, 5, 1, 0), Cqp11L(0xe, 0, -1, -1, -1, 0),
    Cqp11D(0xf, 0x17, 6, 0, 2, 2, 0), Cqp11L(0xf, 0, -1, -1, -1, 0), Cqp11D(0x10, 0x1a, 6, 0, 5, 1, 0),
    Cqp11L(0x10, 0, -1, -1, -1, 0), Cqp11D(0x11, 0x1d, 6, 0, 2, 2, 0), Cqp11L(0x11, 0, -1, -1, -1, 0),
    Cqp11D(0x12, 6, 7, 0, 2, 2, 0), Cqp11L(0x12, 0, -1, -1, -1, 0), Cqp11D(0x13, 9, 7, 0, 5, 1, 0),
    Cqp11L(0x13, 0, -1, -1, -1, 0), Cqp11D(0x14, 0x17, 7, 0, 5, 1, 0), Cqp11L(0x14, 0, -1, -1, -1, 0),
    Cqp11D(0x15, 6, 8, 0, 5, 1, 0), Cqp11L(0x15, 0, -1, -1, -1, 0)
];

/// The 32-round MLS league stage (-1), the regular season: from 22 February to 6 August (weekday 5 and code 1, the
/// midweek rounds on weekday 2 with code 2), each round with one open match slot.
pub(crate) const MLS_32_ROUNDS: [AsiaClubChampScheduleOp; 64] = [
    Cqp11D(0, 0x16, 2, 0, 5, 1, 0), Cqp11L(0, 0, -1, -1, -1, 0), Cqp11D(1, 0x1d, 2, 0, 5, 1, 0),
    Cqp11L(1, 0, -1, -1, -1, 0), Cqp11D(2, 5, 3, 0, 5, 1, 0), Cqp11L(2, 0, -1, -1, -1, 0),
    Cqp11D(3, 0xc, 3, 0, 5, 1, 0), Cqp11L(3, 0, -1, -1, -1, 0), Cqp11D(4, 0x13, 3, 0, 5, 1, 0),
    Cqp11L(4, 0, -1, -1, -1, 0), Cqp11D(5, 0x1a, 3, 0, 5, 1, 0), Cqp11L(5, 0, -1, -1, -1, 0),
    Cqp11D(6, 3, 4, 0, 5, 1, 0), Cqp11L(6, 0, -1, -1, -1, 0), Cqp11D(7, 0xa, 4, 0, 5, 1, 0),
    Cqp11L(7, 0, -1, -1, -1, 0), Cqp11D(8, 0x11, 4, 0, 5, 1, 0), Cqp11L(8, 0, -1, -1, -1, 0),
    Cqp11D(9, 0x16, 4, 0, 2, 2, 0), Cqp11L(9, 0, -1, -1, -1, 0), Cqp11D(0xa, 0x18, 4, 0, 5, 1, 0),
    Cqp11L(0xa, 0, -1, -1, -1, 0), Cqp11D(0xb, 0x1c, 4, 0, 2, 2, 0), Cqp11L(0xb, 0, -1, -1, -1, 0),
    Cqp11D(0xc, 0x1f, 4, 0, 5, 1, 0), Cqp11L(0xc, 0, -1, -1, -1, 0), Cqp11D(0xd, 7, 5, 0, 5, 1, 0),
    Cqp11L(0xd, 0, -1, -1, -1, 0), Cqp11D(0xe, 0xa, 5, 0, 2, 2, 0), Cqp11L(0xe, 0, -1, -1, -1, 0),
    Cqp11D(0xf, 0xe, 5, 0, 5, 1, 0), Cqp11L(0xf, 0, -1, -1, -1, 0), Cqp11D(0x10, 0x15, 5, 0, 5, 1, 0),
    Cqp11L(0x10, 0, -1, -1, -1, 0), Cqp11D(0x11, 0x1c, 5, 0, 5, 1, 0), Cqp11L(0x11, 0, -1, -1, -1, 0),
    Cqp11D(0x12, 2, 6, 0, 2, 2, 0), Cqp11L(0x12, 0, -1, -1, -1, 0), Cqp11D(0x13, 6, 6, 0, 5, 1, 0),
    Cqp11L(0x13, 0, -1, -1, -1, 0), Cqp11D(0x14, 0xc, 6, 0, 5, 1, 0), Cqp11L(0x14, 0, -1, -1, -1, 0),
    Cqp11D(0x15, 0x13, 6, 0, 5, 1, 0), Cqp11L(0x15, 0, -1, -1, -1, 0), Cqp11D(0x16, 0x1a, 6, 0, 5, 1, 0),
    Cqp11L(0x16, 0, -1, -1, -1, 0), Cqp11D(0x17, 0x1d, 6, 0, 2, 2, 0), Cqp11L(0x17, 0, -1, -1, -1, 0),
    Cqp11D(0x18, 2, 7, 0, 5, 1, 0), Cqp11L(0x18, 0, -1, -1, -1, 0), Cqp11D(0x19, 6, 7, 0, 2, 2, 0),
    Cqp11L(0x19, 0, -1, -1, -1, 0), Cqp11D(0x1a, 9, 7, 0, 5, 1, 0), Cqp11L(0x1a, 0, -1, -1, -1, 0),
    Cqp11D(0x1b, 0x10, 7, 0, 5, 1, 0), Cqp11L(0x1b, 0, -1, -1, -1, 0), Cqp11D(0x1c, 0x14, 7, 0, 2, 2, 0),
    Cqp11L(0x1c, 0, -1, -1, -1, 0), Cqp11D(0x1d, 0x17, 7, 0, 5, 1, 0), Cqp11L(0x1d, 0, -1, -1, -1, 0),
    Cqp11D(0x1e, 0x1e, 7, 0, 5, 1, 0), Cqp11L(0x1e, 0, -1, -1, -1, 0), Cqp11D(0x1f, 6, 8, 0, 5, 1, 0),
    Cqp11L(0x1f, 0, -1, -1, -1, 0)
];

/// The two-round MLS play-off stage (2): drawn on 7 and 21 August, played from 16 and 30 August, with the round fields
/// of the cup table (0x68 bytes per round).
pub(crate) const MLS_PLAY_OFF_ROUNDS: [AsiaClubChampScheduleOp; 38] = [
    Cqp11S(0, 7, 8, 0, 6), Cqp11J(0, 0x10, 8, 0, 5, 1, 0), Cqp11W(7, 0x78), Cqp11W(9, 0), Cqp11W(0xb, 0),
    Cqp11W(0xd, 0x230), Cqp11B(0x17, 5), Cqp11W(0x18, 8), Cqp11W(0x1a, 4), Cqp11W(0x1c, 8), Cqp11W(0xf, 0x30),
    Cqp11W(0x11, 0x20), Cqp11W(0x1e, 0), Cqp11B(0x20, 0), Cqp11B(0x21, 2), Cqp11B(0x22, 4), Cqp11X(0x5c, 0),
    Cqp11X(0x60, 0), Cqp11X(0x64, 0), Cqp11S(1, 0x15, 8, 0, 3), Cqp11J(1, 0x1e, 8, 0, 5, 1, 0), Cqp11W(0x6f, 0x82),
    Cqp11W(0x71, 1), Cqp11W(0x73, 0), Cqp11W(0x75, 0x230), Cqp11B(0x7f, 5), Cqp11W(0x80, 4), Cqp11W(0x82, 2),
    Cqp11W(0x84, 0), Cqp11W(0x77, 0x30), Cqp11W(0x79, 0x20), Cqp11W(0x86, 0), Cqp11B(0x88, 0), Cqp11B(0x89, 2),
    Cqp11B(0x8a, 7), Cqp11X(0xc4, 0), Cqp11X(0xc8, 0), Cqp11X(0xcc, 0)
];

/// The one-round MLS Cup final (3): drawn on 8 September, played on 15 September.
pub(crate) const MLS_CUP_FINAL_ROUND: [AsiaClubChampScheduleOp; 18] = [
    Cqp11S(0, 8, 9, 0, 6), Cqp11J(0, 0xf, 9, 0, 6, 2, 4), Cqp11W(0x18, 2), Cqp11W(0x1c, 2), Cqp11W(7, 0x96),
    Cqp11W(9, 0), Cqp11W(0xb, 0), Cqp11W(0xd, 0x220), Cqp11B(0x17, 5), Cqp11W(0x1a, 1), Cqp11W(0xf, 0), Cqp11W(0x1e, 0),
    Cqp11B(0x20, 0), Cqp11B(0x21, 1), Cqp11B(0x22, 0), Cqp11X(0x5c, 0), Cqp11X(0x60, 0), Cqp11X(0x64, 0)
];

// ---------- rx09_13: the Brazilian awards object and its day (award/brazil_awards.cpp) ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\award\brazil_awards.cpp" (the error box).
pub(crate) const BRAZIL_AWARDS_CPP: usize = 0x98b0cc;
/// `BrazilAwards::init_awards(file)` (0x43d340, thiscall, ret 4), through its hook.
const FN_BRAZIL_AWARDS_INIT: usize = 0x43d340;

/// The Brazilian awards' class: the vtable 0x968824, Brazil (`KEY_NATION_BRAZIL`), nation id 5, type 1, no month
/// ratings.
pub(crate) const BRAZIL_AWARDS: NationAwardsClass = NationAwardsClass { vtable: 0x968824, nation_key: Some(KEY_NATION_BRAZIL), nation_id: 5, kind: 1, month_ratings: None };

/// The twelve state championships, in award order (each block of four awards from 10 + 4k): the club competition and
/// the first of its four staff competitions (the best goalkeeper; the best player, the team of the week and the top
/// goalscorer follow 4 bytes apart). The keys from Goiás's top goalscorer on (0x9d0000) have no name in `keys.rs` yet.
pub(crate) const BRAZIL_STATES: [(usize, usize); 12] = [
    (KEY_CLUB_COMP_BRAZILIAN_BAHIA_STATE_CHAMPIONSHIP, KEY_STAFF_COMP_BRAZILIAN_BAHIA_BEST_GOALKEEPER),
    (KEY_CLUB_COMP_BRAZILIAN_CENTRAL_STATE_CHAMPIONSHIP, KEY_STAFF_COMP_BRAZILIAN_CENTRAL_BEST_GOALKEEPER),
    (KEY_CLUB_COMP_BRAZILIAN_GAUCHO_STATE_CHAMPIONSHIP, KEY_STAFF_COMP_BRAZILIAN_GAUCHO_BEST_GOALKEEPER),
    (KEY_CLUB_COMP_BRAZILIAN_GOIAS_STATE_CHAMPIONSHIP, KEY_STAFF_COMP_BRAZILIAN_GOIAS_BEST_GOALKEEPER),
    (KEY_CLUB_COMP_BRAZILIAN_MINAS_GERAIS_STATE_CHAMPIONSHI, 0x9d0004), // Minas Gerais best goalkeeper
    (KEY_CLUB_COMP_BRAZILIAN_NORTH_STATE_CHAMPIONSHIP, 0x9d0014), // North best goalkeeper
    (KEY_CLUB_COMP_BRAZILIAN_NORTHEAST_STATE_CHAMPIONSHIP, 0x9d0024), // Northeast best goalkeeper
    (KEY_CLUB_COMP_BRAZILIAN_PARANA_STATE_CHAMPIONSHIP, 0x9d0034), // Paraná best goalkeeper
    (KEY_CLUB_COMP_BRAZILIAN_PERNAMBUCO_STATE_CHAMPIONSHIP, 0x9d0044), // Pernambuco best goalkeeper
    (KEY_CLUB_COMP_BRAZILIAN_RIO_DE_JANEIRO_STATE_CHAMPIONS, 0x9d0054), // Rio de Janeiro best goalkeeper
    (KEY_CLUB_COMP_BRAZILIAN_SANTA_CATARINA_STATE_CHAMPIONS, 0x9d0064), // Santa Catarina best goalkeeper
    (KEY_CLUB_COMP_BRAZILIAN_SAO_PAULO_STATE_CHAMP, 0x9d0074), // São Paulo best goalkeeper
];

/// Builds the Brazilian awards (`nation_awards_construct` with `BRAZIL_AWARDS`): the saved awards with a file, else
/// `BrazilAwards::init_awards(file)`. Returns this.
/// Original: `BrazilAwards::BrazilAwards` (0x43d2b0, brazil_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn construct_brazil_awards(this: usize, file: usize) -> usize {
    covered!("rx09_13 brazilian awards");
    nation_awards_construct(this, &BRAZIL_AWARDS, file, core::mem::transmute(FN_BRAZIL_AWARDS_INIT))
}

/// Comparer for `construct_brazil_awards` (`compare_nation_awards_built`, 58 awards): only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_construct_brazil_awards(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_CONSTRUCT_BRAZIL_AWARDS)(this, file);
    compare_nation_awards_built(r, this, file, &BRAZIL_AWARDS, 0x3a, "construct_brazil_awards")
}

/// The Brazilian awards' day:
/// - 10 July (`awards_day_is`): for each state championship (`BRAZIL_STATES`, award block 10 + 4k) from its stats
///   (`award_player_of_year`): the best player (block + 1; filters −1, 0, 0x11), the top goalscorer (+ 2; statistic 2)
///   and the best goalkeeper (+ 3; positions 1);
/// - 16 December: the First Division's player of the year (award 1), team of the year (4, `award_team_of_season`), top
///   goalscorer (2) and young player of the year (3, up to 21); then the Second Division's (6, 9, 7 and 8);
/// - always: the base month change check (`NationAwards::daily_update`).
/// Original: `BrazilAwards::daily_update` (0x43e990, brazil_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn brazil_awards_daily_update(this: usize, today: usize) {
    if awards_day_is(today, 0xa, 6) {
        covered!("rx09_13 brazilian state awards");
        for (k, &(comp, _)) in BRAZIL_STATES.iter().enumerate() {
            let block = 10 + 4 * k;
            award_player_of_year(this, comp, today, nation_award(this, block + 1), [-1, 0, 0x11, -1, -1, -1, -1]);
            award_player_of_year(this, comp, today, nation_award(this, block + 2), [-1, 0, 2, -1, -1, -1, -1]);
            award_player_of_year(this, comp, today, nation_award(this, block + 3), [-1, 1, 0x11, -1, -1, -1, -1]); // rx09_13: the state goalkeeper
        }
    }
    if awards_day_is(today, 0x10, 0xb) {
        covered!("rx09_13 brazilian national awards");
        for (comp, first) in [(KEY_CLUB_COMP_BRAZILIAN_NATIONAL_FIRST_DIVISION, 0usize), (KEY_CLUB_COMP_BRAZILIAN_NATIONAL_SECOND_DIVISION, 5)] {
            award_player_of_year(this, comp, today, nation_award(this, first + 1), [-1, 0, 0x11, -1, -1, -1, -1]); // rx09_13: the player of the year
            award_team_of_season(this, comp, nation_award(this, first + 4));
            award_player_of_year(this, comp, today, nation_award(this, first + 2), [-1, 0, 2, -1, -1, -1, -1]);
            award_player_of_year(this, comp, today, nation_award(this, first + 3), [0x15, 0, 0x11, -1, -1, -1, -1]); // rx09_13: young player up to 21
        }
    }
    let base: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_NATION_AWARDS_DAILY_UPDATE);
    base(this, today);
}

/// Comparer for `brazil_awards_daily_update`: the awards cannot be given twice; only the original runs (the copy is
/// proven by the career).
pub(crate) unsafe extern "thiscall" fn compare_brazil_awards_daily_update(this: usize, today: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_BRAZIL_AWARDS_DAILY_UPDATE)(this, today)
}
