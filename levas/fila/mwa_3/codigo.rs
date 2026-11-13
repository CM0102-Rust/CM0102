// ---------- batch %%LEVA%%: the Munster Senior Cup: the round table, the entrants and the season turnover (ire_munster_cup.cpp) ----------

/// The Munster Senior Cup: five knockout rounds of single matches (no replay) for 20 clubs, from August to a final in
/// May of the next year; tie rule 3 [inference]; category 4, and 6 in the final.
pub(crate) const MUNSTER_SENIOR_CUP_ROUNDS: [EngCcCupCupRound; 5] = [
    cql10r((1, 7, 0, 1), (0x17, 7, 0, 2, 2, 0), 0xa, (0, 3, 0), 4, (0x10, 8, 0x10, 0), (0, 1, 0), (0, 0xffffffff, 0)), // mwa3: the preliminary round of the Munster cup
    cql10r((0x19, 7, 0, 4), (0x12, 9, 0, 2, 2, 0), 0x14, (0, 3, 0), 4, (8, 4, 0, 0x14), (0, 1, 0), (0, 0xffffffff, 0)),
    cql10r((0x14, 9, 0, 4), (0x11, 0, 1, 2, 2, 0), 0x78, (0, 3, 0), 4, (8, 4, 4, 0x10), (0, 1, 0), (0, 0xffffffff, 0)),
    cql10r((0x13, 0, 1, 4), (0xe, 2, 1, 2, 2, 0), 0x82, (0, 3, 0), 4, (4, 2, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
    cql10r((0xf, 2, 1, 3), (9, 4, 1, 2, 1, 0x200), 0x96, (0, 3, 0), 6, (2, 1, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
];

/// Rule: the Munster Senior Cup has five knockout rounds of single matches: the preliminary round drawn on 1/8 and
/// played on 23/8 (16 clubs, 8 ties), the next drawn 25/8 and played 18/10 (the 8 winners), the quarter-finals drawn
/// 20/10 and played 17/1 of the next year (the 4 winners and the 4 entrants still out), the semi-finals drawn 19/1 and
/// played 14/3 and the final drawn 15/3 and played on 9/5 [inference: home side code 0x200], each moved to its weekday;
/// tie rule 3 [inference]; category 4, and 6 in the final; the tie winner's prize is a share of the prize fund (−1).
///
/// The cup's virtual `comp_vtable::CREATE_SCHEDULE` (create_round_table(stage, &rounds, &extra, &out)): only for stage
/// −1; `out` (when given) = 0, `rounds` = 5, `extra` = 0; the table (`exe::crt_malloc` of 5 × 0x68; out of memory, the
/// error box at line 0x9c and 0) with `MUNSTER_SENIOR_CUP_ROUNDS` (`eng_cc_cup_create_single_stage_round_table`,
/// checked instruction by instruction). Returns the table.
/// Original: `IreMunsterCup::create_round_table` (0x638a60, ire_munster_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn ire_munster_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    eng_cc_cup_create_single_stage_round_table(Cup(this), stage, rounds, extra, out, &MUNSTER_SENIOR_CUP_ROUNDS, 0x9e9f84, 0x9c, "mwa_3 munster cup rounds")
}

unsafe extern "thiscall" fn compare_ire_munster_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%);
    compare_cup_round_table("ire_munster_cup_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out),
        |r, e, s| ire_munster_cup_create_round_table(this, stage, r, e, s))
}

/// The four League of Ireland clubs that play the Munster Senior Cup, by English name (the exe's strings at 0x9e9ff4,
/// 0x9e9fe4, 0x9e9fd0 and 0x9e9fc0, compared with an inline strcmp).
pub(crate) const MUNSTER_LEAGUE_OF_IRELAND_CLUBS: [&[u8]; 4] = [b"Cork City", b"Cobh Ramblers", b"Waterford United", b"Limerick F.C."];

/// Whether the C string at `p` is `name` (byte for byte, up to its terminating zero).
pub(crate) unsafe fn c_string_is(p: usize, name: &[u8]) -> bool {
    for (k, &b) in name.iter().enumerate() {
        if read_u8(p + k) != b {
            return false;
        }
    }
    read_u8(p + name.len()) == 0
}

/// The body of `ire_munster_cup_collect_entrants`; returns (the routine's return, the entries it writes).
///
/// `cup::TEAM_COUNT` = 20; the entrant table (`exe::crt_malloc` of 0x78 into `cup::TEAM_TABLE`; out of memory, the
/// error box at line 0x11a and −1). One pass over the clubs (`irish_cup_club_count`): a club of the Munster Senior
/// League (`KEY_CLUB_COMP_IRISH_MUNSTER_SENIOR_LEAGUE`) goes into a local pool while it holds fewer than 40; otherwise
/// a Premier or First Division club whose English name (`exe::club_get_english_name`, cdecl) is Cork City, Cobh
/// Ramblers, Waterford United or Limerick F.C. takes the next entry from entry 16 on (club and status 0), up to four.
/// Then the pool is sorted and its best shuffled (`exe::comp_util_sort_and_shuffle_top_clubs`(pool, size, 16)) and its
/// first sixteen take entries 0 to 15 (status 0). The class byte (+4) is not written. Returns 1.
pub(crate) unsafe fn collect_munster_cup_entrants(this: Cup) -> (i32, i32) {
    this.set_team_count(0x14);
    let t = exe::crt_malloc(0x78);
    this.set_team_table(t);
    if t == 0 {
        show_error_at(0x9e9f84, 0x11a);
        return (-1, 0);
    }
    // simplification: the original's pool is the uninitialised stack; with fewer than sixteen Munster Senior League
    // clubs the copy would take nulls where the original took leftovers (the career's league has more)
    let mut munster = [0usize; 40];
    let mut n = 0i16;
    let mut taken = 0usize;
    let mut i = 0i32;
    while i < irish_cup_club_count() {
        let c = club_at(i as isize);
        if let Some(id) = irish_club_division_id(c) {
            if n < 0x28 && id == read_usize(KEY_CLUB_COMP_IRISH_MUNSTER_SENIOR_LEAGUE) {
                munster[n as usize] = c.addr();
                n += 1;
            } else if id == read_usize(KEY_CLUB_COMP_IRISH_PREMIER_DIVISION) || id == read_usize(KEY_CLUB_COMP_IRISH_FIRST_DIVISION) {
                let name = exe::club_get_english_name(c);
                if MUNSTER_LEAGUE_OF_IRELAND_CLUBS.iter().any(|x| c_string_is(name, x)) && taken < 0x18 { // mwa3: the four Munster clubs of the League of Ireland
                    write_usize(this.team_table() + taken + 0x60, c.addr());
                    write_u8(this.team_table() + taken + 0x65, 0);
                    taken += 6;
                }
            }
        }
        i += 1;
    }
    covered!("mwa_3 munster cup entrants");
    exe::comp_util_sort_and_shuffle_top_clubs(munster.as_mut_ptr() as usize, n as i32 as u32, 0x10);
    for (k, &club) in munster.iter().take(16).enumerate() { // mwa3: the sixteen best of the Munster Senior League
        write_usize(this.team_table() + k * 6, club);
        write_u8(this.team_table() + k * 6 + 5, 0);
    }
    (1, 16 + (taken / 6) as i32)
}

/// Rule: the Munster Senior Cup takes 20 clubs: sixteen of the Munster Senior League (the best, shuffled among the top
/// forty-eight), entries 0 to 15, and the four Munster clubs of the League of Ireland (Cork City, Cobh Ramblers,
/// Waterford United and Limerick), entries 16 to 19.
/// Original: `IreMunsterCup::collect_entrants` (0x638e30, ire_munster_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ire_munster_cup_collect_entrants(this: usize) -> i32 {
    collect_munster_cup_entrants(Cup(this)).0
}

unsafe extern "thiscall" fn compare_ire_munster_cup_collect_entrants(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I1%%);
    compare_irish_cup_entrants("ire_munster_cup_collect_entrants", Cup(this), false, || o(this), || collect_munster_cup_entrants(Cup(this)))
}

/// Rule: at the season turn the Munster Senior Cup is rebuilt: last edition's tables are freed, the statistics
/// archived, the year goes up by one and the entrants, the rules, the tie tree and the predicted positions are made
/// again.
///
/// The knockout cups' turnover (`start_new_cup_season`, checked instruction by instruction against BraCup's: only the
/// entrant routine, IreMunsterCup::collect_entrants 0x638e30, the file and the error box lines 0x179, 0x180 and 0x187
/// change).
/// Original: `IreMunsterCup::start_new_season` (0x6390d0, ire_munster_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ire_munster_cup_start_new_season(this: usize) -> u32 {
    start_new_cup_season(Cup(this), 0x638e30, 0x9e9f84, [0x179, 0x180, 0x187], 1, "mwa_3 munster cup turnover")
}

unsafe extern "thiscall" fn compare_ire_munster_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    compare_cup_new_season("ire_munster_cup_start_new_season", Cup(this), || o(this))
}
