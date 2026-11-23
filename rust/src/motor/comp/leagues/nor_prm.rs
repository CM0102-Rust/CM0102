use super::*;

/// The Norwegian turnover, led by the Tippeligaen: the promotion and relegation chain (with 1), the First and Second
/// Division clubs swapped (`nor_prm_swap_first_and_second_division_clubs` 0x792960) and the unnamed step 0x792b40
/// [inference: the Second Division groups regrouped], the Tippeligaen rebuilt for the next year, then the First
/// Division turnover, the predicted positions, the board expectations and the referee renewal.
///
/// `NorPrm::start_new_season()`: `top_division_turnover` with, before, virtual
/// `league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN`(1) and the direct calls 0x792960 and 0x792b40 (thiscall, ret 0);
/// the table `NorPrm::create_table` (0x7925e0) and the boxes of nor_prm.cpp (0xa6977c) at 0x212, 0x219, 0x21f, 0x226;
/// after, the Norwegian First Division's start_new_season (`KEY_CLUB_COMP_NORWEGIAN_FIRST_DIVISION`, no null check).
/// Original: `NorPrm::start_new_season` (0x7927a0, nor_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nor_prm_start_new_season(this: usize) -> u32 {
    let this = League(this);
    top_division_turnover(this, || {
        covered!("mwe_14 norwegian turnover");
        promotion_relegation_chain(this, 1); // mwe14: the Norwegian chain
        call_on_league(0x792960, this); // mwe14: the Norwegian First and Second Division clubs swapped
        call_on_league(0x792b40, this);
    }, 0x7925e0, 0xa6977c, [0x212, 0x219, 0x21f, 0x226], || {
        bra_nat_first_call_virtual_no_args(comp_by_global(KEY_CLUB_COMP_NORWEGIAN_FIRST_DIVISION), comp_vtable::START_NEW_SEASON);
    })
}

/// Comparer for `nor_prm_start_new_season`.
pub(crate) unsafe extern "thiscall" fn compare_nor_prm_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_NOR_PRM_START_NEW_SEASON);
    compare_state_league_new_season("nor_prm_start_new_season", League(this), || o(this))
}

/// Builds the Norwegian Premier Division: discipline rule 21 (the Norwegian league one), the league only (no
/// play-off stage); with no saved state, the rules, the table, the fixtures, the order, the stats and the predicted
/// positions.
///
/// `NorPrm::NorPrm(year, record)`: the body of `construct_division_league` (checked instruction by instruction against
/// the French and Spanish constructors: only the vtable 0x96ded4, the discipline byte +0x50 = 0x15, the two calls and
/// the source and lines change): League::League (`exe::new_league`), the record (`league::COMPETITION`), the vtable,
/// the year (`.set_year()`, word) and the discipline rule; the saved state (`exe::league_load_from_temp_file`, push 1)
/// ends it. Otherwise stage index 0xff, current stage −1, no extra stages (`+0x2c` = 0, `+0xc` = 0), then
/// `NorPrm::set_up_rules` and `NorPrm::create_table` (direct calls, thiscall, ret 0), `exe::league_create_fixtures`,
/// `exe::league_sort_table`(0), the Comp_Stats (operator new of 0x5ce and `exe::new_comp_stats`) into `+8` and
/// `exe::league_set_predicted_positions`; each step that fails shows the box of its line (nor_prm.cpp 0x45, 0x4c, 0x53,
/// 0x5c, 0x63) and returns. Returns this.
/// Original: `NorPrm::NorPrm` (0x791500, nor_prm.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_nor_prm(this: usize, year: u32, record: usize) -> usize {
    covered!("mwe_1 norwegian premier division");
    construct_division_league(League(this), year, record, &NORWEGIAN_PREMIER_DIVISION)
}

/// Comparer for `construct_nor_prm`: only the original runs (it allocates the league), then the fields
/// (`compare_division_league_ctor`).
pub(crate) unsafe extern "thiscall" fn compare_construct_nor_prm(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_NOR_PRM);
    compare_division_league_ctor("construct_nor_prm", League(this), year, record, &NORWEGIAN_PREMIER_DIVISION, || o(this, year, record))
}

/// Sets up the rules of the Norwegian Premier Division (14 clubs): two legs, 3 points for a win; nobody goes up, the
/// 12th goes to the relegation play-off and the bottom 2 go down to the First Division; 7 substitutes and 3
/// substitutions.
///
/// `NorPrm::set_up_rules()`: the writes of `norwegian_premier_rules` (legs, points, category, flags, tie-breaks 1, 2,
/// 3, the bands 0 / 0 / 1 / 2, no division above, the First Division below, substitutions, squad flags 2 and
/// substitutes) and the main-stage round table (virtual `comp_vtable::CREATE_SCHEDULE`: push 0; push &+0x3a; push
/// &+0xa9; push −1; mov ecx -> thiscall, ret 0x10) into `league::ROUND_DATES`. Returns 1.
/// Original: `NorPrm::set_up_rules` (0x792700, nor_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nor_prm_set_up_rules(this: usize) -> u32 {
    covered!("mwe_3 norwegian premier rules");
    eng_prm_apply_league_rules(League(this), &norwegian_premier_rules())
}

/// Comparer for `nor_prm_set_up_rules`: the dry run of `eng_prm_compare_league_rules` (the original builds the round
/// table; the copy writes the rules only).
pub(crate) unsafe extern "thiscall" fn compare_nor_prm_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_NOR_PRM_SET_UP_RULES);
    eng_prm_compare_league_rules("nor_prm_set_up_rules", League(this), norwegian_premier_rules(), || o(this), || nor_prm_set_up_rules(this))
}

/// Creates the Norwegian Premier Division table: 14 clubs, those of the division in database order; the prize fund
/// by a share of 50,000 per place; no TV money.
///
/// `NorPrm::create_table()`: the body of `eng_prm_fill_league_table` (checked instruction by instruction):
/// `.set_club_count()` = 14, the table (`exe::crt_malloc` of 14 × 0x3b; out of memory: the box of nor_prm.cpp
/// (0xa6977c) line 0x1a7 and 0); the clubs whose division (`club::DIVISION`) has the id of
/// `KEY_CLUB_COMP_NORWEGIAN_PREMIER_DIVISION`, in database order, each into the next row (`exe::league_init_table_row`:
/// push 0; push 0; push club; push row -> thiscall, ret 0x10) until full; not full: 0. Full: the prize fund
/// (`exe::league_calc_prize_fund`, push 50,000), `.set_prize_mode()` = 1 and 1.
/// Original: `NorPrm::create_table` (0x7925e0, nor_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nor_prm_create_table(this: usize) -> u32 {
    covered!("mwe_6 tippeligaen table");
    eng_prm_fill_league_table(League(this), 0xe, 0xa6977c, 0x1a7, KEY_CLUB_COMP_NORWEGIAN_PREMIER_DIVISION, 0xc350, || ()) // mwe6: the Tippeligaen prize share
}

/// Comparer for `nor_prm_create_table`: only the original runs (it allocates the table), against the plan of
/// `eng_prm_compare_league_table` (the division's clubs in database order, the count, the return value and the prize
/// fund).
pub(crate) unsafe extern "thiscall" fn compare_nor_prm_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_NOR_PRM_CREATE_TABLE);
    eng_prm_compare_league_table("nor_prm_create_table", League(this), 0xe, KEY_CLUB_COMP_NORWEGIAN_PREMIER_DIVISION, 0xc350, || o(this))
}

/// The rule writes of the Norwegian Premier Division (Tippeligaen, 14 clubs), in the original's order.
///
/// Two legs (`league::LEGS`), 3 points for a win and 1 for a draw, league matches (category 2), no match rule flags
/// and no round draw rules, club sides; tie-breaks goal difference, goals for and code 3; nobody goes up (no division
/// above: −1), no promotion play-off, 1 club to the relegation play-off against the First Division (the 12th) and 2
/// relegated directly; the First Division below; 3 substitutions, the squad flags 2 and 7 substitutes named.
pub(crate) unsafe fn norwegian_premier_rules() -> Vec<(usize, u8, u32)> {
    let mut v = vec![(league::LEGS, 2, 2), (league::WIN_POINTS, 1, 3), (league::DRAW_POINTS, 1, 1), (league::MATCH_CATEGORY, 1, 2), (league::MATCH_RULES, 2, 0), (league::ROUND_DRAW_RULES, 2, 0), (league::CLUB_SIDES, 1, 1)];
    v.extend_from_slice(&[(league::TIEBREAKS, 1, 1), (league::TIEBREAKS + 1, 1, 2), (league::TIEBREAKS + 2, 1, 3)]);
    v.extend_from_slice(&[(league::PROMOTED_COUNT, 1, 0), (league::PROMOTION_PLAYOFF_COUNT, 1, 0)]);
    v.push((league::RELEGATION_PLAYOFF_COUNT, 1, 1)); // mwe3: the Norwegian Premier relegation play-off place
    v.push((league::RELEGATED_COUNT, 1, 2)); // mwe3: the Norwegian Premier relegates 2
    v.extend_from_slice(&[(league::UPPER_DIVISION_ID, 4, 0xffff_ffff), (league::SUBSTITUTIONS_ALLOWED, 1, 3), (league::SQUAD_FLAGS, 1, 2)]);
    v.extend_from_slice(&[(league::LOWER_DIVISION_ID, 4, read_usize(KEY_CLUB_COMP_NORWEGIAN_FIRST_DIVISION) as u32), (league::SUBSTITUTES_NAMED, 1, 7)]);
    v
}

/// The Norwegian Premier Division in the common division constructor: its vtable, discipline rule 21 (the Norwegian
/// league one), its rules (`nor_prm_set_up_rules`, 0x792700) and table (`nor_prm_create_table`, 0x7925e0), source
/// nor_prm.cpp (0xa6977c) and the box lines 0x45, 0x4c, 0x53, 0x5c, 0x63.
pub(crate) const NORWEGIAN_PREMIER_DIVISION: LeagueDivision = LeagueDivision {
    vtabela: 0x96ded4,
    discipline: discipline_rule::NORWAY_LEAGUE, // mwe1: the Norwegian league discipline rule
    rules: 0x792700,
    table: 0x7925e0,
    file: 0xa6977c,
    rows: [0x45, 0x4c, 0x53, 0x5c, 0x63],
    low: false,
};
