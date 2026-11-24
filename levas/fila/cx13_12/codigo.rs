// ---------- cx13_12: the Russian Premier League's staff records and title play-off result (rus_prm.cpp) ----------

/// Swaps two table rows of 0x3b bytes (`league_table_row::SIZE`), through a copy of the first.
pub(crate) unsafe fn swap_table_rows(a: usize, b: usize) {
    let mut tmp = [0u8; league_table_row::SIZE];
    core::ptr::copy_nonoverlapping(a as *const u8, tmp.as_mut_ptr(), league_table_row::SIZE);
    core::ptr::copy_nonoverlapping(b as *const u8, a as *mut u8, league_table_row::SIZE);
    core::ptr::copy_nonoverlapping(tmp.as_ptr(), b as *mut u8, league_table_row::SIZE);
}

/// The staff records by a club's status in the Russian Premier League. In the league (stage 0xff) the common routine
/// (League::update_staff_records_for_status `exe::league_update_staff_records_for_status` with the stage −1, the other
/// arguments as they came), whose result it returns.
///
/// In the title play-off the winner (status 1) is put at the top of the pair: going down the table (a signed word
/// count) to the club's row k, when the row above (k − 1, only for k > 0) holds one of the game's clubs
/// (`fixture::HOME_CLUB`, `fixture::AWAY_CLUB` of `game`), rows k and k − 1 are swapped and given their positions
/// (`league_table_row::POSITION` k and k − 1); otherwise, when the row below (k + 1, read even past the last row) holds
/// one of them, rows k and k + 1 keep their places with the positions k and k + 1; either way both rows get the status
/// 0xff (not decided) and the search ends. Then, once no row has the status 1, the tie is settled: no tie-break
/// matches (`league::TIEBREAK_MATCHES` = 0), the stage reopened (`league::COMPLETE` = 0) for the positions and
/// statuses (virtual `league_vtable::UPDATE_POSITIONS_AND_STATUS`) and closed again (1). The play-off always returns
/// −1; a loser (any other status) changes nothing.
/// Original: `RusPrm::update_staff_records_for_status` (0x7ebc40, rus_prm.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn rus_prm_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, a4: u32, game: usize, a6: u32) -> u32 {
    if stage as u8 == 0xff {
        covered!("cx13_12 russian staff records");
        return exe::league_update_staff_records_for_status(this, club, status, 0xffff_ffff, a4, game, a6); // cx13_12: the common records
    }
    if status as u8 != 1 {
        return 0xffff_ffff;
    }
    covered!("cx13_12 russian play-off winner");
    let l = League(this);
    let in_game = |c: usize| c == read_usize(game + fixture::HOME_CLUB) || c == read_usize(game + fixture::AWAY_CLUB);
    let row = |k: i16| l.table() + (k as isize as usize).wrapping_mul(league_table_row::SIZE);
    let mut k: i16 = 0;
    while k < l.club_count() as i16 {
        if read_usize(row(k)) == club {
            if k > 0 && in_game(read_usize(row(k - 1))) {
                swap_table_rows(row(k), row(k - 1)); // cx13_12: the winner goes above
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
    let n = l.club_count() as i16;
    if (0..n.max(0)).any(|k| read_u8(row(k) + league_table_row::STATUS) == 1) {
        return 0xffff_ffff;
    }
    l.set_tiebreak_matches(0);
    l.set_complete(0);
    bra_nat_first_call_virtual_no_args(this, league_vtable::UPDATE_POSITIONS_AND_STATUS);
    l.set_complete(1);
    0xffff_ffff
}

/// Comparer for `rus_prm_update_staff_records_for_status`: only the original (it adds records, or reorders the table
/// and recomputes the statuses); in the play-off the result must be −1.
pub(crate) unsafe extern "thiscall" fn compare_rus_prm_update_staff_records_for_status(this: usize, club: usize, status: u32, stage: u32, a4: u32, game: usize, a6: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(%%I0%%)(this, club, status, stage, a4, game, a6);
    SCOREBOARD.record(stage as u8 == 0xff || r == 0xffff_ffff, || format!("rus_prm_update_staff_records_for_status({:#x}, {:#x}, status {}, stage {}) original={}", this, club, status as u8, stage as u8 as i8, r as i32));
    r
}
