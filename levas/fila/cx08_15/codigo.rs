// ---------- cx08_15: the Belgian promotion and relegation chain and the Third Division moves and groups (bel_first.cpp) ----------

/// The four clubs the Belgian Third Division keeps for promotion into it (club ids at +0xee, +0xf2, +0xf6, +0xfa of the
/// Third Division object) [inference: the play-off and group winners of the Fourth Divisions].
pub(crate) const BEL_THIRD_INCOMING_CLUBS: usize = 0xee;

/// Rule: the Belgian Third Division moves: the play-off winners (outcome 1 in the play-off cup, stage 9) that are not
/// yet in it come into the Third Division; the clubs marked relegated (3) in its group A table go down to Fourth
/// Division A and those of its group B (stage 0) to Fourth Division B; the four stored clubs come into the Third
/// Division.
///
/// `bel_first_move_third_division_clubs()` on the Third Division object (`KEY_CLUB_COMP_BELGIAN_THIRD_DIVISION`): over
/// stage 9's team table (`cup::TEAM_TABLE`, 6-byte entries, `cup::TEAM_COUNT` of them, a signed word index), an entry
/// with status (+5) 1 whose club's division (`club::DIVISION`) is not the Third Division's record goes there
/// (League::move_club_to_division `exe::league_move_club_to_division`(this, club, the Third Division's record, 1)).
/// Then for each row k below the Third Division's `league::CLUB_COUNT`: its row k with status 3 goes down to Fourth
/// Division A (League::relegate_club_to_division with the record of `KEY_CLUB_COMP_BELGIAN_FOURTH_DIVISION_A`, 1), and
/// stage 0's row k with status 3 to Fourth Division B. Then the clubs of `BEL_THIRD_INCOMING_CLUBS` (by id, unchecked)
/// move into the Third Division (the record of `KEY_CLUB_COMP_BELGIAN_THIRD_DIVISION`, 1).
/// Defect of the original: stage 0's table is read with the Third Division's club count, past its own rows when it is
/// shorter.
/// Original: `bel_first_move_third_division_clubs` (0x41ec90, bel_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bel_first_move_third_division_clubs(this: usize) {
    let this = League(this);
    let third = League(comp_by_global(KEY_CLUB_COMP_BELGIAN_THIRD_DIVISION));
    let (group_b, playoff) = (League(read_usize(third.stages())), Cup(read_usize(third.stages() + 0x24)));
    let mut k: i16 = 0;
    while k < playoff.team_count() {
        let entry = playoff.team_table() + k as usize * 6;
        let club = Club(read_usize(entry));
        if read_u8(entry + 5) == 1 && club.division().addr() != third.competition().addr() {
            exe::league_move_club_to_division(this, club, third.competition().addr(), 1);
        }
        k += 1;
    }
    covered!("cx08_15 belgian third division moves");
    let mut k: i16 = 0;
    while k < third.club_count() {
        for (table, key) in [(third.table(), KEY_CLUB_COMP_BELGIAN_FOURTH_DIVISION_A), (group_b.table(), KEY_CLUB_COMP_BELGIAN_FOURTH_DIVISION_B)] {
            let row = LeagueTableRow(table + k as usize * league_table_row::SIZE);
            if row.status() == 3 {
                exe::league_relegate_club_to_division(this, row.club() as usize, comp_record_by_key(key), 1);
            }
        }
        k += 1;
    }
    for slot in 0..4 {
        let id = read_i32(third.addr() + BEL_THIRD_INCOMING_CLUBS + 4 * slot);
        exe::league_move_club_to_division(this, club_at(id as isize), comp_record_by_key(KEY_CLUB_COMP_BELGIAN_THIRD_DIVISION), 1);
    }
}

/// Comparer for `bel_first_move_third_division_clubs`: only the original (it moves clubs between divisions).
pub(crate) unsafe extern "thiscall" fn compare_bel_first_move_third_division_clubs(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this)
}

/// Rule: the Belgian promotion and relegation chain: the final positions of the First, Second and Third Divisions
/// (and of the Third's group B) are recorded; then promotion and relegation between the First and the Second (one each
/// way) and between the Second and each of the Third's two groups.
///
/// `BelFirst::process_promotion_relegation_chain(pairs)` (`pairs` not read): virtual
/// `league_vtable::RECORD_FINAL_POSITIONS` on the objects of `KEY_CLUB_COMP_BELGIAN_FIRST_DIVISION`, `..._SECOND_..` and
/// `..._THIRD_DIVISION` and on the Third's stage 0 (none checked for null); then League::promote_and_relegate
/// (`exe::league_promote_and_relegate`(this, upper, lower, 1, 1, −1, −1)) for First/Second, Second/Third and Second/the
/// Third's stage 0; the last one's eax is returned.
/// Original: `BelFirst::process_promotion_relegation_chain` (0x41ee80, bel_first.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn bel_first_process_promotion_relegation_chain(this: usize, _pairs: u32) -> u32 {
    let (first, second, third) = (comp_by_global(KEY_CLUB_COMP_BELGIAN_FIRST_DIVISION), comp_by_global(KEY_CLUB_COMP_BELGIAN_SECOND_DIVISION), comp_by_global(KEY_CLUB_COMP_BELGIAN_THIRD_DIVISION));
    covered!("cx08_15 belgian chain");
    for l in [first, second, third, read_usize(League(third).stages())] {
        bra_nat_first_call_virtual_no_args(l, league_vtable::RECORD_FINAL_POSITIONS);
    }
    exe::league_promote_and_relegate(this, first, second, 1, 1, -1, -1);
    exe::league_promote_and_relegate(this, comp_by_global(KEY_CLUB_COMP_BELGIAN_SECOND_DIVISION), comp_by_global(KEY_CLUB_COMP_BELGIAN_THIRD_DIVISION), 1, 1, -1, -1);
    let third = League(comp_by_global(KEY_CLUB_COMP_BELGIAN_THIRD_DIVISION));
    exe::league_promote_and_relegate(this, comp_by_global(KEY_CLUB_COMP_BELGIAN_SECOND_DIVISION), read_usize(third.stages()), 1, 1, -1, -1)
}

/// Comparer for `bel_first_process_promotion_relegation_chain`: only the original (it moves the clubs).
pub(crate) unsafe extern "thiscall" fn compare_bel_first_process_promotion_relegation_chain(this: usize, pairs: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(%%I1%%)(this, pairs)
}

/// The Belgian Third Division clubs in club table order (up to 32): those whose division's id is that of
/// `KEY_CLUB_COMP_BELGIAN_THIRD_DIVISION`.
unsafe fn belgian_third_division_clubs() -> ([usize; 32], usize) {
    let mut list = [0usize; 32];
    let mut n = 0usize;
    let mut i: i16 = 0;
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        let club = club_at(i as isize);
        let division = club.division().addr();
        if division != 0 && read_usize(division) == read_usize(KEY_CLUB_COMP_BELGIAN_THIRD_DIVISION) {
            list[n] = club.addr();
            n += 1;
            if n == 32 {
                break;
            }
        }
        i += 1;
    }
    (list, n)
}

/// Rule: the 32 Belgian Third Division clubs are split by longitude: the first 16 (west) go to group A, the rest to
/// group B.
///
/// `bel_first_split_third_division_groups()`: the clubs of `belgian_third_division_clubs` sorted
/// (`exe::crt_qsort`(list, n, 4, `database_compare_clubs_by_longitude` 0x53fe50)); slots 0..15 get the database record
/// of `KEY_CLUB_COMP_BELGIAN_THIRD_DIVISION_A` and 16..31 that of `KEY_CLUB_COMP_BELGIAN_THIRD_DIVISION_B` at
/// `club::RESERVE_DIVISION` (+0x60) [inference: here the club's regional group].
/// Defect of the original: it always walks the 32 slots of its stack list, so with fewer than 32 Third Division clubs it
/// writes through stale stack values; the copy stops at the clubs found.
/// Original: `bel_first_split_third_division_groups` (0x41ef70, bel_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bel_first_split_third_division_groups(_this: usize) {
    let (a, b) = (comp_record_by_key(KEY_CLUB_COMP_BELGIAN_THIRD_DIVISION_A), comp_record_by_key(KEY_CLUB_COMP_BELGIAN_THIRD_DIVISION_B));
    let (mut list, n) = belgian_third_division_clubs();
    exe::crt_qsort(list.as_mut_ptr() as usize, n as u32, 4, 0x53fe50usize);
    covered!("cx08_15 belgian third division groups");
    for (k, &club) in list.iter().enumerate().take(n) {
        write_usize(club + club::RESERVE_DIVISION, if k < 16 { a } else { b });
    }
}

/// Comparer for `bel_first_split_third_division_groups`: side by side over `club::RESERVE_DIVISION` of the Third
/// Division clubs found beforehand.
pub(crate) unsafe extern "thiscall" fn compare_bel_first_split_third_division_groups(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    let (list, n) = belgian_third_division_clubs();
    let regions: Vec<(usize, usize)> = list[..n].iter().map(|&c| (c + club::RESERVE_DIVISION, 4)).collect();
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&regions, || o(this), || bel_first_split_third_division_groups(this));
    SCOREBOARD.record(same, || format!("bel_first_split_third_division_groups: {} clubs, original {:x?} rust {:x?}", n, mo, mr));
}
