use super::*;

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

/// `BrazilAwards::init_awards(file)` (0x43d340, thiscall, ret 4), through its hook.
pub(crate) const FN_BRAZIL_AWARDS_INIT: usize = 0x43d340;

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
