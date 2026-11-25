// ---------- sbI_3: the stadium bookings and venues object ----------

/// Builds the stadium object (bookings and neutral venues), for a new game or from Stadium.tmp.
///
/// - The CD check (`find_game_cd`, the no-CD patch read at 0x85b6b0 and 0x85b6f4).
/// - The bookings, their count and capacity and the venues, their count and capacity (`stadium_obj`) 0
/// (`stadium_obj::VENUE_SORT` is not touched).
/// - When `stadium_load_from_temp_file` reads the temporary file, nothing else.
/// - Otherwise the national teams lose their ground: from the last club of `G_CLUB_TABLE` down to the first of the
/// national team rows (`G_CLUB_COUNT` − `G_NATION_COUNT`, both read again at each step), each one that is a national
/// team (`exe::club_is_national_team`) gets no stadium (`set_stadium`); then the neutral venues list
/// (`build_venue_list`).
///
/// Returns this.
/// Original: `Stadium::Stadium` (0x85b670, stadium.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn new_stadium_manager(this: usize) -> usize {
    find_game_cd(0x85b6b0, 0x85b6f4);
    for field in [stadium_obj::BOOKINGS, stadium_obj::BOOKING_COUNT, stadium_obj::BOOKING_CAPACITY,
                  stadium_obj::VENUES, stadium_obj::VENUE_COUNT, stadium_obj::VENUE_CAPACITY] {
        write_u32(this + field, 0);
    }
    if stadium_load_from_temp_file(this) != 0 {
        covered!("sbI_3 stadiums from the temporary file");
        return this;
    }
    covered!("sbI_3 new stadium object");
    let mut i = read_i32(G_CLUB_COUNT) - 1;
    while i >= read_i32(G_CLUB_COUNT) - read_i32(G_NATION_COUNT) {
        let club = club_at(i as isize);
        if exe::club_is_national_team(club) != 0 {
            club.set_stadium(Stadium(0)); // sbI_3: a national team without a ground
        }
        i -= 1;
    }
    build_venue_list(this);
    this
}

/// Comparer for `new_stadium_manager`: only the original runs (it waits for the CD, reads the temporary file and
/// allocates the venues); the bookings must be within their capacity and the venues there.
pub(crate) unsafe extern "thiscall" fn compare_new_stadium_manager(this: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I0%%)(this);
    let (bookings, capacity) = (read_i32(this + stadium_obj::BOOKING_COUNT), read_i32(this + stadium_obj::BOOKING_CAPACITY));
    let (venues, count) = (read_usize(this + stadium_obj::VENUES), read_i32(this + stadium_obj::VENUE_COUNT));
    SCOREBOARD.record(r == this && bookings <= capacity && venues != 0 && count > 0,
        || format!("new_stadium_manager: bookings {}/{}, venues {:#x} ({})", bookings, capacity, venues, count));
    r
}
