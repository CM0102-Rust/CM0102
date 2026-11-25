// ---------- rx12_15: the Serie B tie-break play-off and its result (ita_ser_b.cpp) ----------

/// The Serie B tie-break play-off: two clubs level on points in different bands (the title, the promotion places, the
/// four relegated: `serie_b_zones`) play off for the place, as `create_italian_tiebreak_playoff` (the box at line
/// 0x28e). Unlike Serie A's, with no tie the positions are not recomputed (the play-off is only built after a tie was
/// found, so the branch does not run).
/// Original: `ItaSerB::create_tiebreak_playoff_stage` (0x646bb0, ita_ser_b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ita_ser_b_create_tiebreak_playoff_stage(this: usize) {
    create_italian_tiebreak_playoff(League(this), &serie_b_zones(), 0x9ea384, 0x28e, false); // rx12_15: the Serie B bands
}

/// Comparer for `ita_ser_b_create_tiebreak_playoff_stage` (`compare_italian_tiebreak_playoff` with the Serie B bands).
pub(crate) unsafe extern "thiscall" fn compare_ita_ser_b_create_tiebreak_playoff_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    compare_italian_tiebreak_playoff("ita_ser_b_create_tiebreak_playoff_stage", League(this), &serie_b_zones(), || o(this));
}

/// The staff records by a club's status in Serie B. In the league (stage 0xff) the common routine
/// (`exe::league_update_staff_records_for_status` with the stage −1, the other arguments as they came), whose result it
/// returns.
///
/// In the tie-break play-off the winner (status 1) is put at the top of the pair: going down the table (a signed word
/// count) to the club's row k, when the row above (k − 1, only for k > 0) holds one of the game's clubs
/// (`fixture::HOME_CLUB`, `fixture::AWAY_CLUB` of `game`), rows k and k − 1 are swapped (`swap_table_rows`) and given
/// their positions (`league_table_row::POSITION` k and k − 1); otherwise, when the row below (k + 1, read even past the
/// last row, a defect of the original) holds one of them, rows k and k + 1 keep their places with the positions k and k
/// + 1; either way both rows get the status 0xff (not decided) and the search ends. Then, once no row has the status 1,
/// the season closes: no tie-break matches (`league::TIEBREAK_MATCHES` = 0), the stage reopened (`league::COMPLETE` =
/// 0) for the positions and statuses (virtual `league_vtable::UPDATE_POSITIONS_AND_STATUS`) and the completion check
/// (virtual `comp_vtable::CHECK_COMPLETE`(0, 0)), then closed again (1). The play-off always returns −1; a loser (any
/// other status) changes nothing. Unlike Serie A's, the table is not sorted first.
/// Original: `ItaSerB::update_staff_records_for_status` (0x646e70, ita_ser_b.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn ita_ser_b_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, a4: u32, game: usize, a6: u32) -> u32 {
    if stage as u8 == 0xff {
        covered!("rx12_15 serie b staff records");
        return exe::league_update_staff_records_for_status(this, club, status, 0xffff_ffff, a4, game, a6);
    }
    if status as u8 != 1 {
        return 0xffff_ffff;
    }
    covered!("rx12_15 serie b play-off winner");
    let l = League(this);
    let in_game = |c: usize| c == read_usize(game + fixture::HOME_CLUB) || c == read_usize(game + fixture::AWAY_CLUB);
    let row = |k: i16| l.table() + (k as isize as usize).wrapping_mul(league_table_row::SIZE);
    let mut k: i16 = 0;
    while k < l.club_count() {
        if read_usize(row(k)) == club {
            if k > 0 && in_game(read_usize(row(k - 1))) {
                swap_table_rows(row(k), row(k - 1)); // rx12_15: the winner goes above
                write_u8(row(k) + league_table_row::POSITION, k as u8);
                write_u8(row(k - 1) + league_table_row::POSITION, (k as u8).wrapping_sub(1));
                write_u8(row(k) + league_table_row::STATUS, 0xff);
                write_u8(row(k - 1) + league_table_row::STATUS, 0xff);
                break;
            }
            if in_game(read_usize(row(k + 1))) {
                write_u8(row(k) + league_table_row::POSITION, k as u8);
                write_u8(row(k + 1) + league_table_row::POSITION, (k as u8).wrapping_add(1));
                write_u8(row(k) + league_table_row::STATUS, 0xff);
                write_u8(row(k + 1) + league_table_row::STATUS, 0xff);
                break;
            }
        }
        k += 1;
    }
    let n = l.club_count();
    if (0..n.max(0)).any(|k| read_u8(row(k) + league_table_row::STATUS) == 1) {
        return 0xffff_ffff;
    }
    l.set_tiebreak_matches(0);
    l.set_complete(0);
    bra_nat_first_call_virtual_no_args(this, league_vtable::UPDATE_POSITIONS_AND_STATUS);
    let check: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::CHECK_COMPLETE));
    check(this, 0, 0);
    l.set_complete(1);
    0xffff_ffff
}

/// Comparer for `ita_ser_b_update_staff_records_for_status`: only the original (it adds records, or reorders the table
/// and recomputes the statuses); in the play-off the result must be −1.
pub(crate) unsafe extern "thiscall" fn compare_ita_ser_b_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, a4: u32, game: usize, a6: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(%%I1%%)(this, club, status, stage, a4, game, a6);
    SCOREBOARD.record(stage as u8 == 0xff || r == 0xffff_ffff, || format!("ita_ser_b_update_staff_records_for_status({:#x}, {:#x}, status {}, stage {}) original={}", this, club, status as u8, stage as u8 as i8, r as i32));
    r
}
