// ---------- cx15_11: the UEFA seeding table for the screens and the English Intertoto volunteers (uefa_seeding.cpp) ----------

/// The seeding table for a screen: `*table` = the seeding object (its 0x12-byte nation entries start at +0) and the
/// number of entries (`uefa_seeding::NATION_COUNT`, a signed byte, in al only) returned.
/// Original: `UefaSeeding::get_nation_table` (0x903590, uefa_seeding.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn uefa_seeding_get_nation_table(this: usize, table: *mut usize) -> i8 {
    *table = this;
    read_i8(this + uefa_seeding::NATION_COUNT)
}

/// Adds a club's id to the English Intertoto volunteers (`uefa_seeding::INTERTOTO_VOLUNTEERS`, up to 30 ids). Without a
/// list, the count (`uefa_seeding::INTERTOTO_VOLUNTEER_COUNT`) goes to 0 and the 0x78-byte list is allocated
/// (`exe::crt_malloc`; out of memory, the error box of uefa_seeding.cpp line 0xf9 and nothing more); then, below 30
/// (signed byte), the id goes to the next slot and the count goes up; a full list ignores the club.
///
/// Called by the Intertoto volunteer screen (the human manager of a Premier League club answering the request news).
/// Original: `UefaSeeding::add_intertoto_volunteer` (0x9037b0, uefa_seeding.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn uefa_seeding_add_intertoto_volunteer(this: usize, club: usize) {
    if read_usize(this + uefa_seeding::INTERTOTO_VOLUNTEERS) == 0 {
        write_u8(this + uefa_seeding::INTERTOTO_VOLUNTEER_COUNT, 0);
        let list = exe::crt_malloc(0x78);
        write_usize(this + uefa_seeding::INTERTOTO_VOLUNTEERS, list);
        if list == 0 {
            show_error_at(0xaa1a1c, 0xf9); // "...\comp\uefa_seeding.cpp"
            return;
        }
    }
    let count = read_i8(this + uefa_seeding::INTERTOTO_VOLUNTEER_COUNT);
    if count < 0x1e {
        covered!("cx15_11 intertoto volunteer added");
        write_u32(read_usize(this + uefa_seeding::INTERTOTO_VOLUNTEERS) + 4 * count as usize, read_u32(club));
        write_i8(this + uefa_seeding::INTERTOTO_VOLUNTEER_COUNT, count.wrapping_add(1));
    }
}

/// Comparer for `uefa_seeding_get_nation_table`: both run, the out pointer and the count must be equal.
pub(crate) unsafe extern "thiscall" fn compare_uefa_seeding_get_nation_table(this: usize, table: *mut usize) -> i8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, *mut usize) -> i8>(%%I0%%)(this, table);
    let mut t = 0usize;
    let n = uefa_seeding_get_nation_table(this, &mut t);
    SCOREBOARD.record(r == n && *table == t, || format!("uefa_seeding_get_nation_table({:#x}): original {:#x} {} rust {:#x} {}", this, *table, r, t, n));
    r
}

/// Comparer for `uefa_seeding_add_intertoto_volunteer`: with the list already made, side by side on the list field, the
/// count and the list; without it (the routine allocates), only the original.
pub(crate) unsafe extern "thiscall" fn compare_uefa_seeding_add_intertoto_volunteer(this: usize, club: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%);
    let list = read_usize(this + uefa_seeding::INTERTOTO_VOLUNTEERS);
    if list == 0 {
        return o(this, club);
    }
    let reg = [(this + uefa_seeding::INTERTOTO_VOLUNTEERS, 5), (list, 0x78)];
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, club), || uefa_seeding_add_intertoto_volunteer(this, club));
    SCOREBOARD.record(same, || format!("uefa_seeding_add_intertoto_volunteer(club {:#x}): differences at {:x?}", club, memory_diff_addresses(&mo, &mr)));
}
