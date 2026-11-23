use super::*;

/// Rule: the Dutch Super Cup (Johan Cruijff Schaal) is between the Eredivisie champion
/// (`KEY_CLUB_COMP_DUTCH_KPN_TELECOMPETITION`) and the Dutch Cup winner (`KEY_CLUB_COMP_DUTCH_AMSTEL_CUP`), both class
/// 0; if the same club won both, the Eredivisie runner-up comes in; a missing club is an error box (lines 0xbe, 0xc7,
/// 0xd0, 0xdc of hol_super.cpp) and 0.
/// Original: `HolSuper::select_teams` (0x5f8c10, hol_super.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn hol_super_select_teams(this: usize) -> i32 {
    select_champion_and_cup_winner(Cup(this), KEY_CLUB_COMP_DUTCH_KPN_TELECOMPETITION, KEY_CLUB_COMP_DUTCH_AMSTEL_CUP, 0x9cefdc, [0xbe, 0xc7, 0xd0, 0xdc], (0, 0), false).0
}

/// Comparer for `hol_super_select_teams`.
pub(crate) unsafe extern "thiscall" fn compare_hol_super_select_teams(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_HOL_SUPER_SELECT_TEAMS);
    compare_team_list("hol_super_select_teams", Cup(this), || o(this), || select_champion_and_cup_winner(Cup(this), KEY_CLUB_COMP_DUTCH_KPN_TELECOMPETITION, KEY_CLUB_COMP_DUTCH_AMSTEL_CUP,
        0x9cefdc, [0xbe, 0xc7, 0xd0, 0xdc], (0, 0), false))
}

/// At the season turn the Dutch Super Cup is rebuilt: last edition's tables freed, the history archived, the year +1,
/// the entrants (HolSuper::select_teams), the rounds, the tie tree (no predicted positions).
///
/// The knockout-cup turnover template (`usa_mls_all_stars_cup_start_new_season`), checked instruction by instruction
/// against `JapSuper::start_new_season`: only the entrants routine (`HolSuper::select_teams`, thiscall, ret 0), the
/// source file and the box lines (0x121, 0x128, 0x12f) change; no predicted positions: it returns the eax of the tie
/// tree.
/// Original: `HolSuper::start_new_season` (0x5f8e40, hol_super.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn hol_super_start_new_season(this: usize) -> u32 {
    usa_mls_all_stars_cup_start_new_season(Cup(this), 0x5f8c10, 0x9cefdc, [0x121, 0x128, 0x12f], 1, false, "mwc_15 hol super turnover") // mwc15: the Dutch Super Cup
}

/// Comparer for `hol_super_start_new_season`: only the original runs; the year goes up by one
/// (`compare_cup_new_season`).
pub(crate) unsafe extern "thiscall" fn compare_hol_super_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_HOL_SUPER_START_NEW_SEASON);
    compare_cup_new_season("hol_super_start_new_season", Cup(this), || o(this))
}

/// The Dutch Super Cup is born with discipline rule 15; with no saved state, the entrants (HolSuper::select_teams), the
/// round table, the tie tree (a failure is a negative return), the statistics (no predicted positions)
/// (`construct_nation_cup`).
/// Original: `HolSuper::HolSuper` (0x5f8820, hol_super.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_hol_super(this: usize, year: u32, record: usize) -> usize {
    construct_nation_cup(Cup(this), year, record, &HOL_SUPER_CUP_PARAMS)
}

/// Comparer for `construct_hol_super`: only the original runs (`compare_nation_cup_ctor`).
pub(crate) unsafe extern "thiscall" fn compare_construct_hol_super(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_HOL_SUPER);
    compare_nation_cup_ctor("construct_hol_super", Cup(this), year, record, &HOL_SUPER_CUP_PARAMS, || o(this, year, record))
}

/// The Dutch Super Cup has 1 round(s): code 0: draw 5 Jul, match 8 Aug, 2 teams (2 new).
///
/// `HolSuper::create_round_table(stage, &rounds, &extra, &out)` (`single_stage_round_table`): only stage -1 (the others
/// return 0 without writing); out = 0, rounds = the table size, extra = 0 and the table of 0x68-byte rounds
/// (`exe::crt_malloc`; out of memory, the error box of line 0x95 and 0), each with its draw and match dates
/// (`cup_set_round_draw_date`, `cup_set_round_match_date`, cdecl) and its fields (`fra_cup_write_cup_round`).
/// Original: `HolSuper::create_round_table` (0x5f8ad0, hol_super.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn hol_super_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    single_stage_round_table(Cup(this), stage, rounds, extra, out, &HOL_SUPER_ROUNDS, 0x9cefdc, 0x95, "mwc_13 hol super round table")
}

/// Comparer for `hol_super_create_round_table`: side by side (`compare_cup_round_table`).
pub(crate) unsafe extern "thiscall" fn compare_hol_super_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_HOL_SUPER_CREATE_ROUND_TABLE);
    compare_cup_round_table("hol_super_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out), |r, e, s| hol_super_create_round_table(this, stage, r, e, s))
}

pub(crate) const HOL_SUPER_CUP_PARAMS: NationCup = NationCup {
    vtable: 0x96b9d4,
    bench: None,
    discipline: discipline_rule::HOLLAND, // mwc17: the Dutch Super Cup discipline rule
    entrants: 0x5f8c10,
    zero_fails: false,
    predicted: false,
    source_file: 0x9cefdc,
    box_lines: [0x3f, 0x4d, 0x54],
};

/// The Dutch Super Cup: one match.
pub(crate) const HOL_SUPER_ROUNDS: [FraCupCupRound; 1] = [
    cql11r((5, 6, 0, 5), (8, 7, 0, 6, 1, 4), 0, (0, 0x83, 0), 6, (2, 1, 2, 0), (0, 1, 0), (0, 0, 0)), // mwc13 hol_super round 0
];
