// ---------- batch %%LEVA%%: the Greek and Polish super cup clubs (gre_super.cpp, pol_super.cpp) ----------

/// The first club (in database order, `club::SIZE` records of `G_CLUB_TABLE`) whose nation pointer (`club::NATION`) is
/// `nation` and whose database index is not `skip` (the id of an already chosen club); 0 when there is none.
pub(crate) unsafe fn first_club_of_nation(nation: usize, skip: Option<u32>) -> usize {
    let n = read_usize(G_CLUB_COUNT) as i32;
    let mut i = 0i32;
    while i < n {
        let c = read_usize(G_CLUB_TABLE) + i as usize * club::SIZE;
        if read_usize(c + club::NATION) == nation && skip.map_or(true, |s| i as u32 != s) {
            return c;
        }
        i += 1;
    }
    0
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

/// Rule: the Greek Super Cup is between the Greek First Division champion (`KEY_CLUB_COMP_GREEK_PREMIER_DIVISION`)
/// and the Greek Cup winner (`KEY_CLUB_COMP_GREEK_CUP`); a missing one is replaced by the first club of the nation; if
/// the same club won both, the First Division runner-up comes in; if there is none, the error box
/// (`select_league_and_cup_winners`, lines 0xc3 and 0xf1).
/// Original: `GreSuper::select_teams` (0x5ec430, gre_super.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn gre_super_select_teams(this: usize) -> i32 {
    select_league_and_cup_winners(Cup(this), KEY_CLUB_COMP_GREEK_PREMIER_DIVISION, KEY_CLUB_COMP_GREEK_CUP, KEY_CLUB_COMP_GREEK_PREMIER_DIVISION, 0x9cdb58, [0xc3, 0xf1]).0
}

/// Rule: the Polish Super Cup is between the Polish First Division champion (`KEY_CLUB_COMP_POLISH_FIRST_DIVISION`)
/// and the Polish Cup winner (`KEY_CLUB_COMP_POLISH_FA_CUP`); a missing one is replaced by the first club of the nation;
/// if the same club won both, the Polish Cup runner-up comes in; if there is none, the error box (lines 0xc2 and 0xf0).
/// Original: `PolSuper::select_teams` (0x7ccd10, pol_super.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn pol_super_select_teams(this: usize) -> i32 {
    select_league_and_cup_winners(Cup(this), KEY_CLUB_COMP_POLISH_FIRST_DIVISION, KEY_CLUB_COMP_POLISH_FA_CUP, KEY_CLUB_COMP_POLISH_FA_CUP, 0xa78648, [0xc2, 0xf0]).0
}

/// Comparer for `gre_super_select_teams` (side by side: `compare_team_list`).
pub(crate) unsafe extern "thiscall" fn compare_gre_super_select_teams(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I0%%);
    compare_team_list("gre_super_select_teams", Cup(this), || o(this), || select_league_and_cup_winners(Cup(this), KEY_CLUB_COMP_GREEK_PREMIER_DIVISION, KEY_CLUB_COMP_GREEK_CUP,
        KEY_CLUB_COMP_GREEK_PREMIER_DIVISION, 0x9cdb58, [0xc3, 0xf1]))
}

/// Comparer for `pol_super_select_teams`.
pub(crate) unsafe extern "thiscall" fn compare_pol_super_select_teams(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I1%%);
    compare_team_list("pol_super_select_teams", Cup(this), || o(this), || select_league_and_cup_winners(Cup(this), KEY_CLUB_COMP_POLISH_FIRST_DIVISION, KEY_CLUB_COMP_POLISH_FA_CUP,
        KEY_CLUB_COMP_POLISH_FA_CUP, 0xa78648, [0xc2, 0xf0]))
}
