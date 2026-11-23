use super::*;

/// Rule: the Belgian Super Cup is between the Belgian First Division champion (`KEY_CLUB_COMP_BELGIAN_FIRST_DIVISION`,
/// class 2) and the Belgian Cup winner (`KEY_CLUB_COMP_BELGIAN_CUP`, class 1); if the same club won both, the First
/// Division runner-up comes in; a missing club is an error box (lines 0xc6, 0xcf, 0xd8, 0xe4 of bel_super.cpp) and 0.
/// In 2001 the two clubs swap places (the champion second).
/// Original: `BelSuper::select_teams` (0x4212f0, bel_super.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bel_super_select_teams(this: usize) -> i32 {
    select_champion_and_cup_winner(Cup(this), KEY_CLUB_COMP_BELGIAN_FIRST_DIVISION, KEY_CLUB_COMP_BELGIAN_CUP, 0x989ccc, [0xc6, 0xcf, 0xd8, 0xe4], (2, 1), true).0
}

/// Comparer for `bel_super_select_teams` (side by side: `compare_team_list`).
pub(crate) unsafe extern "thiscall" fn compare_bel_super_select_teams(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_BEL_SUPER_SELECT_TEAMS);
    compare_team_list("bel_super_select_teams", Cup(this), || o(this), || select_champion_and_cup_winner(Cup(this), KEY_CLUB_COMP_BELGIAN_FIRST_DIVISION, KEY_CLUB_COMP_BELGIAN_CUP,
        0x989ccc, [0xc6, 0xcf, 0xd8, 0xe4], (2, 1), true))
}

/// At the season turn the Belgian Super Cup is rebuilt: last edition's tables freed, the history archived, the year +1,
/// the entrants (BelSuper::select_teams), the rounds, the tie tree and the predicted positions.
///
/// The knockout-cup turnover template (`usa_mls_all_stars_cup_start_new_season`), checked instruction by instruction
/// against `JapSuper::start_new_season`: only the entrants routine (`BelSuper::select_teams`, thiscall, ret 0), the
/// source file and the box lines (0x134, 0x13b, 0x142) change.
/// Original: `BelSuper::start_new_season` (0x421540, bel_super.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bel_super_start_new_season(this: usize) -> u32 {
    usa_mls_all_stars_cup_start_new_season(Cup(this), 0x4212f0, 0x989ccc, [0x134, 0x13b, 0x142], 1, true, "mwc_15 bel super turnover") // mwc15: the Belgian Super Cup
}

/// Comparer for `bel_super_start_new_season`: only the original runs; the year goes up by one
/// (`compare_cup_new_season`).
pub(crate) unsafe extern "thiscall" fn compare_bel_super_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BEL_SUPER_START_NEW_SEASON);
    compare_cup_new_season("bel_super_start_new_season", Cup(this), || o(this))
}

/// The Belgian Super Cup is born with discipline rule 5, the bench (7 substitutes named, 7 substitutions); with no
/// saved state, the entrants (BelSuper::select_teams), the round table, the tie tree (a failure is a negative return),
/// the statistics and the predicted positions (`construct_nation_cup`).
/// Original: `BelSuper::BelSuper` (0x420ef0, bel_super.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_bel_super(this: usize, year: u32, record: usize) -> usize {
    construct_nation_cup(Cup(this), year, record, &BEL_SUPER_CUP_PARAMS)
}

/// Comparer for `construct_bel_super`: only the original runs (`compare_nation_cup_ctor`).
pub(crate) unsafe extern "thiscall" fn compare_construct_bel_super(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_BEL_SUPER);
    compare_nation_cup_ctor("construct_bel_super", Cup(this), year, record, &BEL_SUPER_CUP_PARAMS, || o(this, year, record))
}

/// The Belgian Super Cup has 1 round(s): code 0: draw 5 Jul, match 5 Aug, 2 teams (2 new).
///
/// `BelSuper::create_round_table(stage, &rounds, &extra, &out)` (`single_stage_round_table`): only stage -1 (the others
/// return 0 without writing); out = 0, rounds = the table size, extra = 0 and the table of 0x68-byte rounds
/// (`exe::crt_malloc`; out of memory, the error box of line 0x9d and 0), each with its draw and match dates
/// (`cup_set_round_draw_date`, `cup_set_round_match_date`, cdecl) and its fields (`fra_cup_write_cup_round`).
/// Original: `BelSuper::create_round_table` (0x4211b0, bel_super.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn bel_super_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    single_stage_round_table(Cup(this), stage, rounds, extra, out, &BEL_SUPER_ROUNDS, 0x989ccc, 0x9d, "mwc_12 bel super round table")
}

/// Comparer for `bel_super_create_round_table`: side by side (`compare_cup_round_table`).
pub(crate) unsafe extern "thiscall" fn compare_bel_super_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_BEL_SUPER_CREATE_ROUND_TABLE);
    compare_cup_round_table("bel_super_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out), |r, e, s| bel_super_create_round_table(this, stage, r, e, s))
}

/// The body of `BelSuper::select_teams` and `HolSuper::select_teams` (thiscall, ret 0): the league champion and the cup
/// winner, with no fallback.
///
/// `team_count` = 2; the list (`exe::crt_malloc` of 12 into `cup::TEAM_TABLE`; out of memory: the box at `rows[0]` and
/// −1). Entry 0 = the latest champion of `league` (`exe::comp_util_get_latest_placed_club`(record, 0), cdecl); none:
/// the box at `rows[1]` and 0. Entry 1 = the latest winner of `cup`; none: the box at `rows[2]` and 0. The same club in
/// both: entry 1 = the latest runner-up (place 1) of `league`; none: the box at `rows[3]` and 0. Class `classes.0` and
/// status 0 for entry 0, class `classes.1` and status 0 for entry 1; with `swap_2001` and the cup's year (+0x40) 2001,
/// the two clubs swap entries (the classes stay). Returns 1.
pub(crate) unsafe fn select_champion_and_cup_winner(this: Cup, league: usize, cup: usize, file: usize, rows: [u32; 4], classes: (u8, u8), swap_2001: bool) -> (i32, i32) {
    this.set_team_count(2);
    let l = exe::crt_malloc(0xc);
    this.set_team_table(l);
    if l == 0 {
        show_error_at(file, rows[0]);
        return (-1, 0);
    }
    write_usize(l, exe::comp_util_get_latest_placed_club(comp_record_of(league), 0)); // mwc23: the league champion
    if read_usize(this.team_table()) == 0 {
        show_error_at(file, rows[1]);
        return (0, 0);
    }
    write_usize(this.team_table() + 6, exe::comp_util_get_latest_placed_club(comp_record_of(cup), 0));
    let t = this.team_table();
    if read_usize(t + 6) == 0 {
        show_error_at(file, rows[2]);
        return (0, 0);
    }
    if read_usize(t) == read_usize(t + 6) {
        covered!("mwc_23 super cup double");
        write_usize(this.team_table() + 6, exe::comp_util_get_latest_placed_club(comp_record_of(league), 1));
        if read_usize(this.team_table() + 6) == 0 {
            show_error_at(file, rows[3]);
            return (0, 0);
        }
    }
    covered!("mwc_23 super cup clubs");
    let t = this.team_table();
    write_u8(t + 4, classes.0);
    write_u8(t + 5, 0);
    write_u8(t + 0xa, classes.1);
    write_u8(t + 0xb, 0);
    if swap_2001 && read_i16(this.addr() + cup::YEAR) == 0x7d1 {
        let (a, b) = (read_usize(t), read_usize(t + 6));
        write_usize(t, b);
        write_usize(t + 6, a);
    }
    (1, 0)
}

pub(crate) const BEL_SUPER_CUP_PARAMS: NationCup = NationCup {
    vtable: 0x967aa8,
    bench: Some((7, 7)),
    discipline: discipline_rule::BELGIUM_CUP, // mwc17: the Belgian Super Cup discipline rule
    entrants: 0x4212f0,
    zero_fails: false,
    predicted: true,
    source_file: 0x989ccc,
    box_lines: [0x44, 0x52, 0x59],
};

/// The Belgian Super Cup: one match.
pub(crate) const BEL_SUPER_ROUNDS: [FraCupCupRound; 1] = [
    cql11r((5, 6, 0, 5), (5, 7, 0, 5, 2, 0), 0, (8, 0x201, 0), 6, (2, 1, 2, 0), (0, 1, 0), (0, 0, 0)), // mwc12 bel_super round 0
];
