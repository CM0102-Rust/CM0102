// ---------- batch %%LEVA%%: the Leinster Senior Cup: the round table, the clubs and the season turnover (ire_leinster_cup.cpp) ----------

/// The Leinster Senior Cup: four knockout rounds of single matches (no replay) for 16 clubs, from August to a final in
/// March of the next year; tie rule 3 [inference]; category 4.
pub(crate) const LEINSTER_SENIOR_CUP_ROUNDS: [EngCcCupCupRound; 4] = [
    cql10r((1, 7, 0, 1), (0x17, 7, 0, 2, 2, 0), 0xa, (0, 3, 0), 4, (0x10, 8, 0x10, 0), (0, 1, 0), (0, 0xffffffff, 0)), // mwa2: the first round of the Leinster cup
    cql10r((0x19, 7, 0, 4), (0x12, 9, 0, 2, 2, 0), 0x78, (0, 3, 0), 4, (8, 4, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
    cql10r((0x14, 9, 0, 4), (0x11, 0, 1, 2, 2, 0), 0x82, (0, 3, 0), 4, (4, 2, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
    cql10r((0x13, 0, 1, 2), (0xe, 2, 1, 2, 2, 0), 0x96, (0, 3, 0), 4, (2, 1, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
];

/// Rule: the Leinster Senior Cup is a 16-club knockout of single matches: the first round drawn on 1/8 and played on
/// 23/8, the quarter-finals drawn 25/8 and played 18/10, the semi-finals drawn 20/10 and played 17/1 of the next year
/// and the final drawn 19/1 and played on 14/3, each moved to its weekday; tie rule 3 [inference: extra time and
/// penalties]; category 4; the tie winner's prize is a share of the prize fund (−1).
///
/// The cup's virtual `comp_vtable::CREATE_SCHEDULE` (create_round_table(stage, &rounds, &extra, &out)): only for stage
/// −1; `out` (when given) = 0, `rounds` = 4, `extra` = 0; the table (`exe::crt_malloc` of 4 × 0x68; out of memory, the
/// error box at line 0x9d and 0) with `LEINSTER_SENIOR_CUP_ROUNDS` (`eng_cc_cup_create_single_stage_round_table`,
/// checked instruction by instruction). Returns the table.
/// Original: `IreLeinsterCup::create_round_table` (0x636760, ire_leinster_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn ire_leinster_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    eng_cc_cup_create_single_stage_round_table(Cup(this), stage, rounds, extra, out, &LEINSTER_SENIOR_CUP_ROUNDS, 0x9e9f0c, 0x9d, "mwa_2 leinster cup rounds")
}

unsafe extern "thiscall" fn compare_ire_leinster_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%);
    compare_cup_round_table("ire_leinster_cup_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out),
        |r, e, s| ire_leinster_cup_create_round_table(this, stage, r, e, s))
}

/// Whether the club's stadium town is in Leinster (the exe's ire_club_is_in_leinster 0x6399c0, push club; add esp, 4 ->
/// cdecl, a gap function of ire_pres_cup.cpp not yet ported: Dublin, Bray, Longford, Dundalk, Drogheda, Athlone or
/// Kilkenny).
pub(crate) unsafe fn ire_club_is_in_leinster(club: Club) -> bool {
    let f: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x6399c0usize);
    f(club.addr()) != 0
}

/// The body of `ire_leinster_cup_select_clubs`; returns (the routine's return, the entries it writes).
///
/// `cup::TEAM_COUNT` = 16; the entrant table (`exe::crt_malloc` of 0x60 into `cup::TEAM_TABLE`; out of memory, the
/// error box at line 0x108 and −1). One pass over the clubs (`irish_cup_club_count`): a club of the Leinster Senior
/// League (`KEY_CLUB_COMP_IRISH_LEINSTER_SENIOR_LEAGUE`) goes into a local pool while it holds fewer than 40;
/// otherwise a Premier or First Division club based in Leinster (`ire_club_is_in_leinster`) takes the next entry from
/// entry 4 on (club and status 0), while the entry offset counter is below 0x4e (so up to 13 clubs: the 13th would
/// write entry 16, past the 16-entry table, a defect of the original that the career's 12 or fewer Leinster clubs never
/// reach). Then the pool is sorted and its best shuffled (`exe::comp_util_sort_and_shuffle_top_clubs`(pool, size, 4))
/// and its first four take entries 0 to 3 (status 0). The class byte (+4) is not written. Returns 1.
pub(crate) unsafe fn select_leinster_cup_clubs(this: Cup) -> (i32, i32) {
    this.set_team_count(0x10);
    let t = exe::crt_malloc(0x60);
    this.set_team_table(t);
    if t == 0 {
        show_error_at(0x9e9f0c, 0x108);
        return (-1, 0);
    }
    // simplification: the original's pool is the uninitialised stack; with fewer than four Leinster Senior League clubs
    // the copy would take nulls where the original took leftovers (the career's league has more)
    let mut pool = [0usize; 40];
    let mut n = 0i16;
    let mut slot = 0usize;
    let mut i = 0i32;
    while i < irish_cup_club_count() {
        let c = club_at(i as isize);
        if let Some(id) = irish_club_division_id(c) {
            if n < 0x28 && id == read_usize(KEY_CLUB_COMP_IRISH_LEINSTER_SENIOR_LEAGUE) {
                pool[n as usize] = c.addr();
                n += 1;
            } else if (id == read_usize(KEY_CLUB_COMP_IRISH_PREMIER_DIVISION) || id == read_usize(KEY_CLUB_COMP_IRISH_FIRST_DIVISION)) && ire_club_is_in_leinster(c) && slot < 0x4e {
                slot += 6;
                write_usize(this.team_table() + slot + 0x12, c.addr()); // mwa2: the League of Ireland clubs from Leinster
                write_u8(this.team_table() + slot + 0x17, 0);
            }
        }
        i += 1;
    }
    covered!("mwa_2 leinster cup clubs");
    exe::comp_util_sort_and_shuffle_top_clubs(pool.as_mut_ptr() as usize, n as i32 as u32, 4);
    for k in 0..4usize { // mwa2: the four best of the Leinster Senior League
        write_usize(this.team_table() + k * 6, pool[k]);
        write_u8(this.team_table() + k * 6 + 5, 0);
    }
    (1, 4 + (slot / 6) as i32)
}

/// Rule: the Leinster Senior Cup takes 16 clubs: four of the Leinster Senior League (the best, shuffled among the top
/// twelve) and the Premier and First Division clubs based in Leinster.
/// Original: `IreLeinsterCup::select_clubs` (0x636a80, ire_leinster_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ire_leinster_cup_select_clubs(this: usize) -> i32 {
    select_leinster_cup_clubs(Cup(this)).0
}

unsafe extern "thiscall" fn compare_ire_leinster_cup_select_clubs(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I1%%);
    compare_irish_cup_entrants("ire_leinster_cup_select_clubs", Cup(this), false, || o(this), || select_leinster_cup_clubs(Cup(this)))
}

/// Rule: at the season turn the Leinster Senior Cup is rebuilt: last edition's tables are freed, the statistics
/// archived, the year goes up by one and the clubs, the rules, the tie tree and the predicted positions are made again.
///
/// The knockout cups' turnover (`start_new_cup_season`, checked instruction by instruction against BraCup's: only the
/// entrant routine, IreLeinsterCup::select_clubs 0x636a80, the file and the error box lines 0x167, 0x16e and 0x175
/// change).
/// Original: `IreLeinsterCup::start_new_season` (0x636c20, ire_leinster_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ire_leinster_cup_start_new_season(this: usize) -> u32 {
    start_new_cup_season(Cup(this), 0x636a80, 0x9e9f0c, [0x167, 0x16e, 0x175], 1, "mwa_2 leinster cup turnover")
}

unsafe extern "thiscall" fn compare_ire_leinster_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    compare_cup_new_season("ire_leinster_cup_start_new_season", Cup(this), || o(this))
}
