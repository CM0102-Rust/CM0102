// ---------- cx09_19: the Croatian promotion and relegation chain and the regrouping of the second division (cro_a1.cpp) ----------

/// The Croatian chain, run by the First Division: the final positions of the First Division and of both second division
/// groups (virtual `league_vtable::RECORD_FINAL_POSITIONS`, First, north, south), then the exchanges First–north and
/// First–south (`exe::league_promote_and_relegate`(upper, lower, 1, 1, −1, −1) on this league). The argument is not
/// read.
/// Original: `CroA1::process_promotion_relegation_chain` (0x5150f0, cro_a1.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn cro_a1_process_promotion_relegation_chain(this: usize, _arg: u32) {
    let first = comp_by_global(KEY_CLUB_COMP_CROATIAN_FIRST_DIVISION_9CF8A4);
    for key in [KEY_CLUB_COMP_CROATIAN_FIRST_DIVISION_9CF8A4, KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_NORTH, KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_SOUTH] {
        let league = comp_by_global(key);
        let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(league) + league_vtable::RECORD_FINAL_POSITIONS));
        f(league);
    }
    covered!("cx09_19 croatian chain");
    for key in [KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_NORTH, KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_SOUTH] {
        exe::league_promote_and_relegate(this, first, comp_by_global(key), 1, 1, -1, -1);
    }
}

/// The 32 Croatian second division clubs, ordered by the latitude of their stadium city: the clubs (index 0 to
/// `G_CLUB_COUNT`, read once) whose `club::DIVISION` is the north or the south group record (up to 32, in club order),
/// sorted by qsort with Database's latitude order (0x53fd40). Returns the list (32 slots) and the count.
unsafe fn croatian_second_division_by_latitude() -> ([usize; 32], usize) {
    let (north, south) = (comp_record_of_key(KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_NORTH), comp_record_of_key(KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_SOUTH));
    let mut list = [0usize; 32];
    let mut n = 0usize;
    let count = read_i32(G_CLUB_COUNT);
    let mut i = 0i32;
    while i < count {
        let club = club_at(i as isize);
        let division = club.division().addr();
        if division == north || division == south {
            list[n] = club.addr();
            n += 1;
            if n == 32 {
                break;
            }
        }
        i += 1;
    }
    exe::crt_qsort(list.as_mut_ptr() as usize, n as u32, 4, 0x53fd40usize);
    (list, n)
}

/// Regroups the Croatian second division by latitude (`croatian_second_division_by_latitude`): the first 16 clubs go
/// to the north group and the other 16 to the south group (`club::DIVISION`); a club that changes group and has a human
/// manager (`exe::news_club_has_human_manager` on `G_NEWS`) gets a local news item (`exe::new_newsitem`(0x13, 0): slot 0
/// the club's id, slot 1 the new group's id) sent to its manager (`exe::news_send_to_club_manager`). The original walks
/// all 32 slots even with fewer clubs, reading its uninitialised stack (a defect; the copy's empty slots are 0 and
/// would fault the same way).
/// Original: `cro_a1_realign_second_division_groups` (0x515190, cro_a1.cpp, cdecl).
pub(crate) unsafe extern "C" fn cro_a1_realign_second_division_groups() {
    let (north, south) = (comp_record_of_key(KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_NORTH), comp_record_of_key(KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_SOUTH));
    let (list, _) = croatian_second_division_by_latitude();
    covered!("cx09_19 croatian second division regrouped");
    for (k, &club) in list.iter().enumerate() {
        let group = if k < 16 { north } else { south };
        if read_usize(club + club::DIVISION) != group && exe::news_club_has_human_manager(read_usize(G_NEWS), club) != 0 {
            let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
            let n = local.as_mut_ptr() as usize;
            exe::new_newsitem(n, 0x13, 0);
            exe::newsitem_set_slot(n, 0, read_u32(club));
            exe::newsitem_set_slot(n, 1, read_u32(group));
            exe::news_send_to_club_manager(read_usize(G_NEWS), n, club);
        }
        write_usize(club + club::DIVISION, group);
    }
}

/// Comparer for `cro_a1_process_promotion_relegation_chain`: only the original (it moves the clubs).
pub(crate) unsafe extern "thiscall" fn compare_cro_a1_process_promotion_relegation_chain(this: usize, arg: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, u32)>(%%I0%%)(this, arg)
}

/// Comparer for `cro_a1_realign_second_division_groups`: only the original (it sends news); the copy's order, made
/// before, must match the groups the original gave.
pub(crate) unsafe extern "C" fn compare_cro_a1_realign_second_division_groups() {
    let (list, n) = croatian_second_division_by_latitude();
    original_fn::<unsafe extern "C" fn()>(%%I1%%)();
    let (north, south) = (comp_record_of_key(KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_NORTH), comp_record_of_key(KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_SOUTH));
    let wrong: Vec<(usize, usize)> = list[..n].iter().enumerate()
        .filter(|&(k, &c)| read_usize(c + club::DIVISION) != if k < 16 { north } else { south })
        .map(|(k, &c)| (k, c)).collect();
    SCOREBOARD.record(wrong.is_empty(), || format!("cro_a1_realign_second_division_groups: {} clubs, (slot, club) in the wrong group {:x?}", n, wrong));
}
