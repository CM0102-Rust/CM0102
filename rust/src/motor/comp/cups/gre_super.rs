use super::*;

/// Rule: the Greek Super Cup at career creation: discipline rule 0x20, 7 substitutes and 3 substitutions and, with no
/// saved state, its two clubs (the league champion and the cup winner), the round, the fixture tree and the statistics
/// (`construct_east_nation_cup`, without prize fund nor predicted positions).
/// Original: `GreSuper::GreSuper` (0x5ec040, gre_super.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_gre_super(this: usize, year: u32, record: usize) -> usize {
    construct_east_nation_cup(Cup(this), year, record, &GREEK_SUPER_CUP, "mwd_20 greek super cup new")
}

/// Rule: the Greek Super Cup is a single match between 2 clubs, drawn on 5/7 and played on 2/8 at a neutral ground
/// [inference: venue 4], tie rule 3; no prizes; category 6.
/// Original: `GreSuper::create_round_table` (0x5ec2f0, gre_super.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn gre_super_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    super_cup_round_table(Cup(this), stage, rounds, extra, out, &GREEK_SUPER_CUP_ROUNDS, 0x9cdb58, 0x97, "mwd_20 greek super cup round")
}

/// Comparer for `construct_gre_super` (only the original runs).
pub(crate) unsafe extern "thiscall" fn compare_construct_gre_super(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_GRE_SUPER);
    compare_east_nation_cup_ctor("construct_gre_super", Cup(this), year, record, &GREEK_SUPER_CUP, || o(this, year, record))
}

/// Comparer for `gre_super_create_round_table` (side by side, the written round compared).
pub(crate) unsafe extern "thiscall" fn compare_gre_super_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_GRE_SUPER_CREATE_ROUND_TABLE);
    compare_cup_round_table("gre_super_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out), |r, e, s| gre_super_create_round_table(this, stage, r, e, s))
}

/// Rule: the Greek Super Cup is between the Greek First Division champion (`KEY_CLUB_COMP_GREEK_PREMIER_DIVISION`)
/// and the Greek Cup winner (`KEY_CLUB_COMP_GREEK_CUP`); a missing one is replaced by the first club of the nation; if
/// the same club won both, the First Division runner-up comes in; if there is none, the error box
/// (`select_league_and_cup_winners`, lines 0xc3 and 0xf1).
/// Original: `GreSuper::select_teams` (0x5ec430, gre_super.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn gre_super_select_teams(this: usize) -> i32 {
    select_league_and_cup_winners(Cup(this), KEY_CLUB_COMP_GREEK_PREMIER_DIVISION, KEY_CLUB_COMP_GREEK_CUP, KEY_CLUB_COMP_GREEK_PREMIER_DIVISION, 0x9cdb58, [0xc3, 0xf1]).0
}

/// Comparer for `gre_super_select_teams` (side by side: `compare_team_list`).
pub(crate) unsafe extern "thiscall" fn compare_gre_super_select_teams(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_GRE_SUPER_SELECT_TEAMS);
    compare_team_list("gre_super_select_teams", Cup(this), || o(this), || select_league_and_cup_winners(Cup(this), KEY_CLUB_COMP_GREEK_PREMIER_DIVISION, KEY_CLUB_COMP_GREEK_CUP,
        KEY_CLUB_COMP_GREEK_PREMIER_DIVISION, 0x9cdb58, [0xc3, 0xf1]))
}

/// Rule: the Greek Super Cup season turnover: the finished edition's tables freed, its statistics archived, the year
/// + 1, the two clubs (`GreSuper::select_teams` 0x5ec430: the league champion and the cup winner), the round and the
/// fixture tree; no predicted positions (`usa_mls_all_stars_cup_start_new_season`; lines 0x136, 0x13d, 0x144 of
/// gre_super.cpp).
/// Original: `GreSuper::start_new_season` (0x5ec6b0, gre_super.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn gre_super_start_new_season(this: usize) -> u32 {
    usa_mls_all_stars_cup_start_new_season(Cup(this), 0x5ec430, 0x9cdb58, [0x136, 0x13d, 0x144], 1, false, "mwd_10 greek super cup turnover")
}

/// Comparer for `gre_super_start_new_season`.
pub(crate) unsafe extern "thiscall" fn compare_gre_super_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_GRE_SUPER_START_NEW_SEASON);
    compare_cup_new_season("gre_super_start_new_season", Cup(this), || o(this))
}

/// The body of `GreSuper::select_teams` and `PolSuper::select_teams` (thiscall, ret 0): the league champion and the cup
/// winner.
///
/// `team_count` = 2; the list (`exe::crt_malloc` of 12 into `cup::TEAM_TABLE`; out of memory: the box at `rows[0]` and
/// −1). Entry 0 = the latest champion of `league` (comp_util_get_latest_placed_club `exe::comp_util_get_latest_placed_club`(record,
/// 0): 2 pushes; add esp, 8 -> cdecl); none: the first club of the competition's nation (the record's +0x5d;
/// `first_club_of_nation`). Entry 1 = the latest winner of `cup`; none: the first club of the nation whose index is not
/// entry 0's id (entry 0's record +0, read even when entry 0 is null, as the original). The same club in both: entry 1
/// = the latest runner-up (place 1) of `double`; none: the box at `rows[1]` and 0. The class and status bytes (+4, +5,
/// +0xa, +0xb) = 0. Returns 1.
pub(crate) unsafe fn select_league_and_cup_winners(this: Cup, league: usize, cup: usize, double: usize, file: usize, rows: [u32; 2]) -> (i32, i32) {
    this.set_team_count(2);
    let l = exe::crt_malloc(0xc);
    this.set_team_table(l);
    if l == 0 {
        show_error_at(file, rows[0]);
        return (-1, 0);
    }
    let nation = read_usize(read_usize(this.addr() + 4) + competition::NATION);
    write_usize(l, exe::comp_util_get_latest_placed_club(comp_record_of(league), 0)); // mwd26: the league champion
    if read_usize(l) == 0 {
        let c = first_club_of_nation(nation, None);
        if c != 0 {
            write_usize(l, c);
        }
    }
    write_usize(l + 6, exe::comp_util_get_latest_placed_club(comp_record_of(cup), 0));
    if read_usize(l + 6) == 0 {
        let c = first_club_of_nation(nation, Some(read_u32(read_usize(l))));
        if c != 0 {
            write_usize(l + 6, c);
        }
    }
    if read_usize(l) == read_usize(l + 6) {
        covered!("mwd_26 super cup double");
        write_usize(l + 6, exe::comp_util_get_latest_placed_club(comp_record_of(double), 1));
        if read_usize(l + 6) == 0 {
            show_error_at(file, rows[1]);
            return (0, 0);
        }
    }
    covered!("mwd_26 super cup clubs");
    for o in [4usize, 5, 0xa, 0xb] {
        write_u8(l + o, 0);
    }
    (1, 0)
}

/// The first club (in database order, `club::SIZE` records of `G_CLUB_TABLE`) whose nation pointer (`club::NATION`) is
/// `nation` and whose database index is not `skip` (the id of an already chosen club); 0 when there is none.
pub(crate) unsafe fn first_club_of_nation(nation: usize, skip: Option<u32>) -> usize {
    let n = read_usize(G_CLUB_COUNT) as i32;
    let mut i = 0i32;
    while i < n {
        let c = Club(read_usize(G_CLUB_TABLE) + i as usize * club::SIZE);
        if c.nation().addr() == nation && skip.map_or(true, |s| i as u32 != s) {
            return c.addr();
        }
        i += 1;
    }
    0
}

/// The Greek Super Cup: one match between 2 clubs, drawn on 5/7 (weekday 5) and played on 2/8 (weekday 2, leg 2) at
/// venue 4 [inference: a neutral ground], tie rule 3, category 6, no prizes.
pub(crate) const GREEK_SUPER_CUP_ROUNDS: [FraSuperCupRound; 1] = [
    cql12r((5, 6, 0, 5), (2, 7, 0, 2, 2, 4), 0, (0, 3, 0), 6, (2, 1, 2, 0), (0, 1, 0), (0, 0, 0)), // mwd20: the Greek Super Cup match
];

/// The Greek Super Cup: discipline rule 0x20, no prize fund, no predicted positions; the two clubs from
/// `GreSuper::select_teams` (0x5ec430).
pub(crate) const GREEK_SUPER_CUP: EastNationCup = EastNationCup {
    vtable: 0x96b6d0,
    discipline: 0x20,
    entrants: 0x5ec430,
    prize: None,
    late_substitutes: false,
    stage_vector: false,
    rank: false,
    file: 0x9cdb58,
    rows: [0x41, 0x4f, 0x56],
};
