// ---------- sbE_1: the Finnish turnover steps: the First Division clubs refreshed, its groups and the chain (fin_prm.cpp) ----------

/// The Ykkönen field holding the indices of four clubs (four dwords) [inference: the four clubs the Ykkönen's
/// create_play_off_stages (0x594080) keeps for the Premier Division].
pub(crate) const FIN_FIRST_KEPT_CLUBS: usize = 0xee;

/// Rule: at the Finnish turnover, the play-off winners and the four clubs kept by the Ykkönen go into the Ykkönen, and
/// the Ykkönen's relegated clubs (its North group table and its South group stage) go down to the Kakkonen North.
///
/// The Ykkönen (`G_COMP_OBJECT_TABLE` by `KEY_CLUB_COMP_FINNISH_FIRST_DIVISION`): each team of its play-off cup (stage
/// slot 2, `Cup::team_table`, 6-byte entries, 16-bit index up to `.team_count()`) whose outcome byte (+5) is 1 and whose
/// division (`club::DIVISION`, not checked for null) is not the Ykkönen's record (`league::COMPETITION`) is moved there
/// (League::move_club_to_division `exe::league_move_club_to_division`(this, club, record, 1), thiscall, ret 0xc). Then,
/// for the Ykkönen's own table and for its stage 0, each row with `league_table_row::STATUS` 3 goes down
/// (League::relegate_club_to_division `exe::league_relegate_club_to_division`(this, club, the record of
/// `KEY_CLUB_COMP_FINNISH_SECOND_DIVISION_NORTH`, 1)). Last, the four clubs whose indices are at
/// `FIN_FIRST_KEPT_CLUBS` are moved to the Ykkönen's record (without checking where they are).
/// Original: `FinPrm::refresh_first_division_clubs` (0x595f00, fin_prm.cpp, thiscall: ecx = the Veikkausliiga, ret 0).
pub(crate) unsafe extern "thiscall" fn fin_prm_refresh_first_division_clubs(this: usize) {
    let first = League(comp_by_global(KEY_CLUB_COMP_FINNISH_FIRST_DIVISION));
    let play_off = Cup(read_usize(first.stages() + 8));
    let mut k: i16 = 0;
    while k < play_off.team_count() {
        let entry = play_off.team_table() + 6 * k as usize;
        let club = read_usize(entry);
        if read_u8(entry + 5) == 1 && read_usize(club + club::DIVISION) != first.competition().addr() {
            covered!("sbE_1 finnish play-off winner up");
            exe::league_move_club_to_division(this, club, first.competition(), 1);
        }
        k += 1;
    }
    for group in [-1i8, 0] {
        let g = if group == -1 { first } else { League(read_usize(first.stages() + 4 * group as usize)) };
        let mut k: i16 = 0;
        while k < g.club_count() {
            let row = LeagueTableRow(g.table() + k as usize * league_table_row::SIZE);
            if row.status() == 3 { // sbE_1: the Ykkonen's relegated clubs
                exe::league_relegate_club_to_division(this, row.club() as usize, comp_record_of(KEY_CLUB_COMP_FINNISH_SECOND_DIVISION_NORTH), 1);
            }
            k += 1;
        }
    }
    for i in 0..4 {
        let club = club_at(read_i32(first.addr() + FIN_FIRST_KEPT_CLUBS + 4 * i) as isize);
        exe::league_move_club_to_division(this, club, comp_record_of(KEY_CLUB_COMP_FINNISH_FIRST_DIVISION), 1);
    }
}

/// Comparer for `fin_prm_refresh_first_division_clubs`: only the original runs (it moves clubs between divisions);
/// afterwards the four kept clubs must be in the Ykkönen.
pub(crate) unsafe extern "thiscall" fn compare_fin_prm_refresh_first_division_clubs(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this);
    let first = League(comp_by_global(KEY_CLUB_COMP_FINNISH_FIRST_DIVISION));
    let wrong: Vec<usize> = (0..4).map(|i| club_at(read_i32(first.addr() + FIN_FIRST_KEPT_CLUBS + 4 * i) as isize))
        .filter(|c| c.division().addr() != comp_record_of(KEY_CLUB_COMP_FINNISH_FIRST_DIVISION)).map(|c| c.addr()).collect();
    SCOREBOARD.record(wrong.is_empty(), || format!("fin_prm_refresh_first_division_clubs({:#x}): kept clubs outside the Ykkonen {:x?}", this, wrong));
}

/// The Ykkönen clubs in database order (16-bit index up to `G_CLUB_COUNT`, at most 20): those whose division
/// (`club::DIVISION`) has the id of `KEY_CLUB_COMP_FINNISH_FIRST_DIVISION`.
pub(crate) unsafe fn finnish_first_division_clubs() -> Vec<usize> {
    let mut v = Vec::new();
    let mut i: i16 = 0;
    while (i as i32) < read_i32(G_CLUB_COUNT) && v.len() < 0x14 {
        let c = club_at(i as isize);
        let d = c.division().addr();
        if d != 0 && read_u32(d) == read_u32(KEY_CLUB_COMP_FINNISH_FIRST_DIVISION) {
            v.push(c.addr());
        }
        i = i.wrapping_add(1);
    }
    v
}

/// Rule: the Ykkönen's clubs are split into its two groups by latitude: sorted from south to north, the first ten go
/// to the South group and the rest to the North group.
///
/// `finnish_first_division_clubs` (up to 20) sorted by `exe::crt_qsort`(list, n, 4, 0x53fd40) (cdecl; 0x53fd40 is the
/// comparer of the clubs' city latitude [inference: from the map's purpose]); then each club's `club::RESERVE_DIVISION`
/// (here the regional group) is the record of `KEY_CLUB_COMP_FINNISH_FIRST_DIVISION_SOUTH` for the first ten (16-bit
/// index) and of `KEY_CLUB_COMP_FINNISH_FIRST_DIVISION_NORTH` for the others. The object it is called on is not read.
/// Original: `FinPrm::assign_first_division_groups` (0x596050, fin_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn fin_prm_assign_first_division_groups(_this: usize) {
    let north = comp_record_of(KEY_CLUB_COMP_FINNISH_FIRST_DIVISION_NORTH);
    let south = comp_record_of(KEY_CLUB_COMP_FINNISH_FIRST_DIVISION_SOUTH);
    let mut list = finnish_first_division_clubs();
    exe::crt_qsort(list.as_mut_ptr() as usize, list.len() as u32, 4, 0x53fd40usize);
    covered!("sbE_1 finnish groups by latitude");
    for (k, &club) in list.iter().enumerate() {
        let group = if k < 10 { south } else { north }; // sbE_1: the ten southernmost
        write_usize(club + club::RESERVE_DIVISION, group);
    }
}

/// Comparer for `fin_prm_assign_first_division_groups`: side by side on the reserve division of the Ykkönen's clubs
/// (the sort only reads, so both run).
pub(crate) unsafe extern "thiscall" fn compare_fin_prm_assign_first_division_groups(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    let regions: Vec<(usize, usize)> = finnish_first_division_clubs().iter().map(|&c| (c + club::RESERVE_DIVISION, 4)).collect();
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&regions, || o(this), || fin_prm_assign_first_division_groups(this));
    SCOREBOARD.record(same, || format!("fin_prm_assign_first_division_groups: original={:x?} rust={:x?}", mo, mr));
}

/// Rule: Finnish promotion and relegation: the final positions of the Veikkausliiga, the Ykkönen and the Ykkönen's
/// stage 0 recorded, and the marked clubs swapped between the Veikkausliiga and each of the two.
///
/// Virtual `league_vtable::RECORD_FINAL_POSITIONS` on the objects of `KEY_CLUB_COMP_FINNISH_PREMIER_DIVISION` and
/// `KEY_CLUB_COMP_FINNISH_FIRST_DIVISION` and on the Ykkönen's stage 0; League::promote_and_relegate
/// (`exe::league_promote_and_relegate`(this, above, below, 1, 1, −1, −1), thiscall, ret 0x18) Veikkausliiga / Ykkönen
/// and Veikkausliiga / Ykkönen stage 0, whose eax is returned. The argument is not read.
/// Original: `FinPrm::process_promotion_relegation_chain` (0x596110, fin_prm.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn fin_prm_process_promotion_relegation_chain(this: usize, _pairs: u32) -> u32 {
    let premier = || comp_by_global(KEY_CLUB_COMP_FINNISH_PREMIER_DIVISION);
    let first = || comp_by_global(KEY_CLUB_COMP_FINNISH_FIRST_DIVISION);
    bra_nat_first_call_virtual_no_args(premier(), league_vtable::RECORD_FINAL_POSITIONS);
    bra_nat_first_call_virtual_no_args(first(), league_vtable::RECORD_FINAL_POSITIONS);
    bra_nat_first_call_virtual_no_args(read_usize(League(first()).stages()), league_vtable::RECORD_FINAL_POSITIONS);
    covered!("sbE_1 finnish promotion and relegation");
    exe::league_promote_and_relegate(this, premier(), first(), 1, 1, -1, -1); // sbE_1: the Veikkausliiga and the Ykkonen
    exe::league_promote_and_relegate(this, premier(), read_usize(League(first()).stages()), 1, 1, -1, -1)
}

/// Comparer for `fin_prm_process_promotion_relegation_chain`: only the original runs (it moves clubs between divisions).
pub(crate) unsafe extern "thiscall" fn compare_fin_prm_process_promotion_relegation_chain(this: usize, pairs: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(%%I2%%)(this, pairs)
}
