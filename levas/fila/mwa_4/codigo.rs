// ---------- batch %%LEVA%%: the Irish President's Cup: the round table, the two clubs and the season turnover (ire_pres_cup.cpp) ----------

/// The President's Cup: one match between two clubs, drawn on 5/7 and played on 4/8 [inference: home side code 0x400];
/// tie rule 3 [inference: penalties]; category 6; no prize.
pub(crate) const IRISH_PRESIDENTS_CUP_ROUNDS: [EngCcCupCupRound; 1] = [
    cql10r((5, 6, 0, 5), (4, 7, 0, 4, 2, 0x400), 0, (0, 3, 0), 6, (2, 1, 2, 0), (0, 1, 0), (0, 0, 0)), // mwa4: the President's Cup final
];

/// Rule: the President's Cup is a single match between two clubs, drawn on 5/7 and played on 4/8 (moved to weekday 4)
/// [inference: home side code 0x400]; tie rule 3 [inference: penalties]; category 6; no prize.
///
/// The cup's virtual `comp_vtable::CREATE_SCHEDULE` (create_round_table(stage, &rounds, &extra, &out)): only for stage
/// −1; `out` (when given) = 0, `rounds` = 1, `extra` = 0; the table (`exe::crt_malloc` of 0x68; out of memory, the
/// error box at line 0x95 and 0) with `IRISH_PRESIDENTS_CUP_ROUNDS` (`eng_cc_cup_create_single_stage_round_table`,
/// checked instruction by instruction: the original writes the same fields in another order). Returns the table.
/// Original: `IrePresCup::create_round_table` (0x639530, ire_pres_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn ire_pres_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    eng_cc_cup_create_single_stage_round_table(Cup(this), stage, rounds, extra, out, &IRISH_PRESIDENTS_CUP_ROUNDS, 0x9ea000, 0x95, "mwa_4 presidents cup rounds")
}

unsafe extern "thiscall" fn compare_ire_pres_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%);
    compare_cup_round_table("ire_pres_cup_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out),
        |r, e, s| ire_pres_cup_create_round_table(this, stage, r, e, s))
}

/// The body of `ire_pres_cup_collect_entrants`; returns (the routine's return, the entries it writes).
///
/// `cup::TEAM_COUNT` = 2. The clubs whose last season's division (`club::LAST_DIVISION`) was the Premier Division
/// (`KEY_CLUB_COMP_IRISH_PREMIER_DIVISION`) go into a 12-place table by last season's position (`club::LAST_POSITION`,
/// a signed byte, 1 to 12; the last club of a position wins it; all `G_CLUB_COUNT` clubs are scanned). The entrant table (`exe::crt_calloc`(2, 6), cdecl; out of memory, the error box at line 0xd9
/// and −1). From the top, the first two clubs based in Leinster (`ire_club_is_in_leinster`) take entries 0 and 1 (each
/// leaves the table); if the second entry is still empty, the remaining clubs from the top fill the empty entry 0 (it
/// stays in the table) and then entry 1. The class and status bytes of both entries are zeroed. Returns 1.
pub(crate) unsafe fn collect_presidents_cup_entrants(this: Cup) -> (i32, i32) {
    this.set_team_count(2);
    let mut by_position = [0usize; 12];
    let mut i = 0i32;
    while i < read_i32(G_CLUB_COUNT) {
        let c = club_at(i as isize);
        let p = c.last_position() as i8;
        let last = c.last_division();
        if !last.is_null() && read_usize(last.addr()) == read_usize(KEY_CLUB_COMP_IRISH_PREMIER_DIVISION) && (1..=12).contains(&p) {
            by_position[p as usize - 1] = c.addr();
        }
        i += 1;
    }
    let t = exe::crt_calloc(2, 6);
    this.set_team_table(t);
    if t == 0 {
        show_error_at(0x9ea000, 0xd9);
        return (-1, 0);
    }
    covered!("mwa_4 presidents cup clubs");
    for k in 0..12usize { // mwa4: the two best placed Leinster clubs
        let c = by_position[k];
        if c != 0 && ire_club_is_in_leinster(Club(c)) {
            if read_usize(t) != 0 {
                write_usize(t + 6, c);
                by_position[k] = 0;
                break;
            }
            write_usize(t, c);
            by_position[k] = 0;
        }
    }
    if read_usize(t + 6) == 0 {
        covered!("mwa_4 presidents cup without two leinster clubs");
        for &c in by_position.iter() {
            if c != 0 {
                if read_usize(t) != 0 {
                    write_usize(t + 6, c);
                    break;
                }
                write_usize(t, c);
            }
        }
    }
    write_u8(t + 4, 0);
    write_u8(t + 5, 0);
    write_u8(t + 0xa, 0);
    write_u8(t + 0xb, 0);
    (1, 2)
}

/// Rule: the President's Cup is between the two best placed clubs of last season's Premier Division based in Leinster
/// (Dublin, Bray, Longford, Dundalk, Drogheda, Athlone or Kilkenny); with fewer than two, the best placed of the others
/// complete it.
/// Original: `IrePresCup::collect_entrants` (0x639670, ire_pres_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ire_pres_cup_collect_entrants(this: usize) -> i32 {
    collect_presidents_cup_entrants(Cup(this)).0
}

unsafe extern "thiscall" fn compare_ire_pres_cup_collect_entrants(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I1%%);
    compare_irish_cup_entrants("ire_pres_cup_collect_entrants", Cup(this), true, || o(this), || collect_presidents_cup_entrants(Cup(this)))
}

/// Rule: at the season turn the President's Cup is rebuilt: last edition's tables are freed, the statistics archived,
/// the year goes up by one and the two clubs, the rules and the tie tree are made again (no predicted positions).
///
/// The knockout cups' turnover without the predicted positions (`usa_mls_all_stars_cup_start_new_season` with
/// `predicted` false, checked instruction by instruction against UsaMlsAllStars's: only the entrant routine,
/// IrePresCup::collect_entrants 0x639670, the file and the error box lines 0x14d, 0x154 and 0x15b change).
/// Original: `IrePresCup::start_new_season` (0x639820, ire_pres_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ire_pres_cup_start_new_season(this: usize) -> u32 {
    usa_mls_all_stars_cup_start_new_season(Cup(this), 0x639670, 0x9ea000, [0x14d, 0x154, 0x15b], 1, false, "mwa_4 presidents cup turnover")
}

unsafe extern "thiscall" fn compare_ire_pres_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    compare_cup_new_season("ire_pres_cup_start_new_season", Cup(this), || o(this))
}
