// ---------- batch %%LEVA%%: the German League Cup entrants and the Belgian and Dutch super cup clubs (ger_lge_cup.cpp, bel_super.cpp, hol_super.cpp) ----------

/// Rule: the German League Cup (Ligapokal) takes 6 clubs: the Bundesliga's top six of last season minus the German Cup
/// holder, who enters as the sixth; entries 4 and 5 (the best-placed one after the holder, and the holder) are class 1.
///
/// `team_count` = 6. A local list of 6 slots, zeroed; over the clubs (up to [`G_CLUB_COUNT`] read once) while fewer
/// than 6 are counted: a club whose `club::LAST_DIVISION` is the Bundesliga's record (`KEY_CLUB_COMP_GERMAN_FIRST_DIVISION`)
/// and whose `club::LAST_POSITION` (a signed byte) is 1..6 goes to slot position − 1 (the count goes up even when a
/// slot is overwritten). Short of 6: the box of ger_lge_cup.cpp at line 0x11d, then over the clubs (the count read
/// again) while short of 6: a German club (nation id `KEY_NATION_GERMANY`) of the same last division with a last
/// position above 6 goes to the first empty slot (the count goes up even when none is empty). The list
/// (`exe::crt_calloc`(team_count, 6) into `cup::TEAM_TABLE`; out of memory: the box at line 0x13d and −1). Entry 5 =
/// the latest winner of the German Cup (`exe::comp_util_get_latest_placed_club`(record of `KEY_CLUB_COMP_GERMAN_CUP`,
/// 0)); none: the box at line 0x146 and 0. Then entries 4, 3, 2, 1, 0 each take the first slot that is set and is not
/// the holder (the slot is cleared); none: the entry stays empty. Class and status 0 for all, class 1 for entries 4
/// and 5. Returns 1.
/// Original: `GerLgeCup::collect_entrants` (0x5dc700, ger_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ger_lge_cup_collect_entrants(this: usize) -> i32 {
    let this = Cup(this);
    let mut slots = [0usize; 6];
    this.set_team_count(6);
    let first = comp_record_of(KEY_CLUB_COMP_GERMAN_FIRST_DIVISION);
    let mut n: i16 = 0;
    let count = read_usize(G_CLUB_COUNT) as i32;
    let mut club = read_usize(G_CLUB_TABLE);
    let mut i = 0i32;
    while i < count && n < 6 {
        let pos = read_i8(club + club::LAST_POSITION);
        if read_usize(club + club::LAST_DIVISION) == first && pos < 7 && pos > 0 {
            slots[(pos - 1) as usize] = club;
            n += 1;
        }
        i += 1;
        club = club.wrapping_add(club::SIZE);
    }
    if n != 6 {
        show_error_at(0x9cd450, 0x11d);
        let mut club = read_usize(G_CLUB_TABLE);
        let mut i = 0i32;
        while i < read_usize(G_CLUB_COUNT) as i32 && n < 6 {
            let nation = read_usize(club + club::NATION);
            if nation != 0 && read_usize(nation) == read_usize(KEY_NATION_GERMANY) && read_usize(club + club::LAST_DIVISION) == first && read_i8(club + club::LAST_POSITION) > 6 {
                if let Some(s) = slots.iter_mut().find(|s| **s == 0) {
                    *s = club;
                }
                n += 1;
            }
            i += 1;
            club = club.wrapping_add(club::SIZE);
        }
    }
    let l = exe::crt_calloc(this.team_count() as i32 as u32, 6);
    this.set_team_table(l);
    if l == 0 {
        show_error_at(0x9cd450, 0x13d);
        return -1;
    }
    write_usize(l + 0x1e, exe::comp_util_get_latest_placed_club(comp_record_of(KEY_CLUB_COMP_GERMAN_CUP), 0));
    if read_usize(this.team_table() + 0x1e) == 0 {
        show_error_at(0x9cd450, 0x146);
        return 0;
    }
    covered!("mwc_23 german league cup entrants");
    let mut e = 4i32;
    while e >= 0 {
        if let Some(s) = slots.iter_mut().find(|s| **s != 0 && **s != read_usize(this.team_table() + 0x1e)) {
            write_usize(this.team_table() + 6 * e as usize, *s);
            *s = 0;
        }
        e -= 1;
    }
    for k in 0..6usize {
        write_u8(this.team_table() + 6 * k + 4, 0);
        write_u8(this.team_table() + 6 * k + 5, 0);
        if k >= 4 { // mwc23: the holder and the best placed after him seeded
            write_u8(this.team_table() + 6 * k + 4, 1);
        }
    }
    1
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
    if swap_2001 && read_i16(this.addr() + 0x40) == 0x7d1 {
        let (a, b) = (read_usize(t), read_usize(t + 6));
        write_usize(t, b);
        write_usize(t + 6, a);
    }
    (1, 0)
}

/// Rule: the Belgian Super Cup is between the Belgian First Division champion (`KEY_CLUB_COMP_BELGIAN_FIRST_DIVISION`,
/// class 2) and the Belgian Cup winner (`KEY_CLUB_COMP_BELGIAN_CUP`, class 1); if the same club won both, the First
/// Division runner-up comes in; a missing club is an error box (lines 0xc6, 0xcf, 0xd8, 0xe4 of bel_super.cpp) and 0.
/// In 2001 the two clubs swap places (the champion second).
/// Original: `BelSuper::select_teams` (0x4212f0, bel_super.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bel_super_select_teams(this: usize) -> i32 {
    select_champion_and_cup_winner(Cup(this), KEY_CLUB_COMP_BELGIAN_FIRST_DIVISION, KEY_CLUB_COMP_BELGIAN_CUP, 0x989ccc, [0xc6, 0xcf, 0xd8, 0xe4], (2, 1), true).0
}

/// Rule: the Dutch Super Cup (Johan Cruijff Schaal) is between the Eredivisie champion
/// (`KEY_CLUB_COMP_DUTCH_KPN_TELECOMPETITION`) and the Dutch Cup winner (`KEY_CLUB_COMP_DUTCH_AMSTEL_CUP`), both class
/// 0; if the same club won both, the Eredivisie runner-up comes in; a missing club is an error box (lines 0xbe, 0xc7,
/// 0xd0, 0xdc of hol_super.cpp) and 0.
/// Original: `HolSuper::select_teams` (0x5f8c10, hol_super.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn hol_super_select_teams(this: usize) -> i32 {
    select_champion_and_cup_winner(Cup(this), KEY_CLUB_COMP_DUTCH_KPN_TELECOMPETITION, KEY_CLUB_COMP_DUTCH_AMSTEL_CUP, 0x9cefdc, [0xbe, 0xc7, 0xd0, 0xdc], (0, 0), false).0
}

/// Comparer for `ger_lge_cup_collect_entrants` (side by side, every entry compared).
pub(crate) unsafe extern "thiscall" fn compare_ger_lge_cup_collect_entrants(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I0%%);
    compare_entrants_with_seeds("ger_lge_cup_collect_entrants", Cup(this), || o(this), || ger_lge_cup_collect_entrants(this), |_| true)
}

/// Comparer for `bel_super_select_teams` (side by side: `compare_team_list`).
pub(crate) unsafe extern "thiscall" fn compare_bel_super_select_teams(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I1%%);
    compare_team_list("bel_super_select_teams", Cup(this), || o(this), || select_champion_and_cup_winner(Cup(this), KEY_CLUB_COMP_BELGIAN_FIRST_DIVISION, KEY_CLUB_COMP_BELGIAN_CUP,
        0x989ccc, [0xc6, 0xcf, 0xd8, 0xe4], (2, 1), true))
}

/// Comparer for `hol_super_select_teams`.
pub(crate) unsafe extern "thiscall" fn compare_hol_super_select_teams(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I2%%);
    compare_team_list("hol_super_select_teams", Cup(this), || o(this), || select_champion_and_cup_winner(Cup(this), KEY_CLUB_COMP_DUTCH_KPN_TELECOMPETITION, KEY_CLUB_COMP_DUTCH_AMSTEL_CUP,
        0x9cefdc, [0xbe, 0xc7, 0xd0, 0xdc], (0, 0), false))
}
