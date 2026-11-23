use super::*;

/// Rule: the Korean Super Cup is born with discipline rule 42 and the bench (5 substitutes named, 3 substitutions); with
/// no saved state, its two clubs (KorSuperCup::create_table), the round table, the tie tree (a failure is a negative
/// return), the statistics and no predicted positions (`construct_nation_cup`, checked instruction by instruction: the
/// FraSuper/HolSuper body with the bench written).
/// Original: `KorSuperCup::KorSuperCup` (0x66d820, kor_super_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_kor_super_cup(this: usize, year: u32, record: usize) -> usize {
    construct_nation_cup(Cup(this), year, record, &KOR_SUPER_CUP_PARAMS)
}

/// Rule: the Korean Super Cup is a single match between its 2 clubs (tie rules 0x83), drawn on 1 February (weekday 5)
/// and played on 10 March (moved to weekday 6) at venue 0, category 6; no prize money. Outside stage −1 it writes
/// nothing and returns 0; out of memory, the box at line 0x96 and 0 (`fra_super_create_single_stage_round_table`). The
/// original also writes 0 in the round's +0x11 word [inference: a field the other cups leave to the round's date].
/// Original: `KorSuperCup::create_round_table` (0x66dad0, kor_super_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn kor_super_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let t = fra_super_create_single_stage_round_table(Cup(this), stage, rounds, extra, out, &KOREAN_SUPER_CUP_ROUNDS, KOR_SUPER_CUP_FILE, 0x96, "mwf_21 korean super cup rounds", |_| {});
    if t != 0 {
        write_u16(t + 0x11, 0);
    }
    t
}

/// Rule: the Korean Super Cup is between the K League champion (`KEY_CLUB_COMP_KOREAN_LEAGUE`) and the Korean FA Cup
/// winner (`KEY_CLUB_COMP_KOREAN_FA_CUP`); a missing one is replaced by the first club of the nation; if the same club
/// won both, the FA Cup runner-up comes in; if there is none, the error box (`select_league_and_cup_winners`, the
/// GreSuper/PolSuper body checked instruction by instruction, lines 0xc1 and 0xef). Returns 1, 0 without a second club
/// and −1 out of memory.
/// Original: `KorSuperCup::create_table` (0x66dc10, kor_super_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn kor_super_cup_create_table(this: usize) -> i32 {
    korean_super_cup_clubs(Cup(this)).0
}

/// Comparer for `construct_kor_super_cup`: only the original runs (`compare_nation_cup_ctor`).
pub(crate) unsafe extern "thiscall" fn compare_construct_kor_super_cup(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_KOR_SUPER_CUP);
    compare_nation_cup_ctor("construct_kor_super_cup", Cup(this), year, record, &KOR_SUPER_CUP_PARAMS, || o(this, year, record))
}

/// Comparer for `kor_super_cup_create_round_table` (side by side, the written rounds compared).
pub(crate) unsafe extern "thiscall" fn compare_kor_super_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_KOR_SUPER_CUP_CREATE_ROUND_TABLE);
    compare_cup_round_table("kor_super_cup_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out), |r, e, s| kor_super_cup_create_round_table(this, stage, r, e, s))
}

/// Comparer for `kor_super_cup_create_table` (side by side: `compare_team_list`).
pub(crate) unsafe extern "thiscall" fn compare_kor_super_cup_create_table(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_KOR_SUPER_CUP_CREATE_TABLE);
    compare_team_list("kor_super_cup_create_table", Cup(this), || o(this), || korean_super_cup_clubs(Cup(this)))
}

/// The season turnover of the Korean Super Cup: last season's entrant, round and tie tables freed, the season archived in the
/// statistics, the year moved on by one and the entrants, the round table and the tie tree rebuilt; no predicted positions: it returns the eax of the tie tree.
///
/// The cup turnover template (`usa_mls_all_stars_cup_start_new_season`, checked instruction by instruction against the sisters): `cup::SEASON_CLOSED` = 0;
/// the team table (`cup::TEAM_TABLE`) and the round table (`cup::ROUND_TABLE`) freed (`exe::crt_free`) and zeroed; the
/// ties of each round (`cup::TIE_TABLES`, `cup::ROUND_COUNT` rounds) freed and zeroed, then the array; with statistics,
/// Comp_Stats::archive_season_and_reset (`exe::comp_stats_archive_season_and_reset`, thiscall(stats, 1), ret 4); the
/// year (`cup::YEAR`) up by one; `cup::CURRENT_ROUND` = 0 and `cup::RUN_BY_STAGES` = 0. Then, each with its own box
/// (kor_super_cup.cpp, lines 0x134, 0x13b and 0x142) when it returns a negative value: the entrants (KorSuperCup::create_table at 0x66dc10, thiscall, ret 0), the round
/// table (virtual `comp_vtable::SET_UP_RULES`) and the tie tree (virtual `cup_vtable::MAP_FIXTURE_TREE`).
/// After a box the copy returns 0 (the original returns the box's eax; the turnover's caller does not read it).
/// Original: `KorSuperCup::start_new_season` (0x66de90, kor_super_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn kor_super_cup_start_new_season(this: usize) -> u32 {
    usa_mls_all_stars_cup_start_new_season(Cup(this), 0x66dc10, 0x9eb198, [0x134, 0x13b, 0x142], 1, false, "mwf_4 Korean Super Cup rollover") // mwf4: the turnover of the Korean Super Cup
}

/// Comparer for `kor_super_cup_start_new_season`: only the original runs (it frees and rebuilds the cup); the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_kor_super_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_KOR_SUPER_CUP_START_NEW_SEASON);
    compare_cup_new_season("kor_super_cup_start_new_season", Cup(this), || o(this))
}

/// The body of `kor_super_cup_create_table` (and of its comparer's copy).
pub(crate) unsafe fn korean_super_cup_clubs(this: Cup) -> (i32, i32) {
    covered!("mwf_21 korean super cup clubs");
    select_league_and_cup_winners(this, KEY_CLUB_COMP_KOREAN_LEAGUE, KEY_CLUB_COMP_KOREAN_FA_CUP, KEY_CLUB_COMP_KOREAN_FA_CUP, KOR_SUPER_CUP_FILE, [0xc1, 0xef]) // mwf21: champion and cup winner
}

/// The Korean Super Cup: 5 substitutes named and 3 substitutions, discipline rule 42 (Korea), no predicted positions.
pub(crate) const KOR_SUPER_CUP_PARAMS: NationCup = NationCup {
    vtable: 0x96cf38,
    bench: Some((5, 3)), // mwf21: the Korean Super Cup bench
    discipline: discipline_rule::KOREA,
    entrants: 0x66dc10,
    zero_fails: false,
    predicted: false,
    source_file: KOR_SUPER_CUP_FILE,
    box_lines: [0x40, 0x4e, 0x55],
};

/// The Korean Super Cup: one match in March, no prize.
pub(crate) const KOREAN_SUPER_CUP_ROUNDS: [FraSuperCupRound; 1] = [
    cql12r((1, 1, 0, 5), (0xa, 2, 0, 6, 1, 0), 0, (0, 0x83, 0), 6, (2, 1, 2, 0), (0, 1, 0), (0, 0, 0)), // mwf21: the Korean Super Cup match
];

/// The source file of the Korean Super Cup's error boxes.
pub(crate) const KOR_SUPER_CUP_FILE: usize = 0x9eb198;
