// ---------- batch %%LEVA%%: the award shortlists: building one, adding a candidate and announcing it (award_shortlist.cpp) ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\award\award_shortlist.cpp" (the error boxes).
pub(crate) const AWARD_SHORTLIST_CPP: usize = 0x9880b0;

/// Fields of an award shortlist (`AWARD_SHORTLIST`) and of its 14-byte candidate entries.
pub(crate) mod award_shortlist_fields {
    /// The candidate entries (+0), their count (+4) and capacity (+6), signed words.
    pub(crate) const ENTRIES: usize = 0x0;
    pub(crate) const COUNT: usize = 0x4;
    pub(crate) const CAPACITY: usize = 0x6;
    /// The 14-byte scope record copied from the constructor's argument (+0x28): +0 the scope type (0 a division, 1 a
    /// nation, 2 a continent by the club's nation, 3 a continent by the national team's nation, 4 every club manager, 5
    /// every national manager, other every manager), +1 the division's id, +5 the nation's, +9 the continent's, +0xd the
    /// capacity (signed byte).
    pub(crate) const SCOPE: usize = 0x28;
    pub(crate) const SCOPE_DIVISION: usize = 0x29;
    pub(crate) const SCOPE_NATION: usize = 0x2d;
    pub(crate) const SCOPE_CONTINENT: usize = 0x31;
    pub(crate) const SCOPE_SIZE: usize = 0xe;
    /// The shortlist's index in the award manager (+0x36, word; −1 until announced) and the announcement date (+0x38).
    pub(crate) const MANAGER_INDEX: usize = 0x36;
    pub(crate) const DATE: usize = 0x38;
    /// An entry: the player's id (+0), the club's id or −1 (+4), the score (+8, float), the votes (+0xc, word).
    pub(crate) const ENTRY_SIZE: usize = 0xe;
    pub(crate) const ENTRY_CLUB_ID: usize = 0x4;
    pub(crate) const ENTRY_SCORE: usize = 0x8;
    pub(crate) const ENTRY_VOTES: usize = 0xc;
    /// AWARD_SHORTLIST::load (thiscall, ret 4) and the qsort comparer by name (cdecl).
    pub(crate) const FN_AWARD_SHORTLIST_LOAD: usize = 0x417590;
    pub(crate) const FN_AWARD_SHORTLIST_COMPARE_BY_NAME: usize = 0x418340;
    /// The news item type of a shortlist announcement.
    pub(crate) const SHORTLIST_NEWS: u32 = 0x1c;
}

/// Builds an award shortlist from its 14-byte scope record (and loads it from a saved game when given the file).
///
/// The date (`DATE`) built empty (Date::Date `exe::date_date_54a110`, thiscall). Without a scope, the error box (line
/// 0x30) and nothing else. Otherwise `MANAGER_INDEX` = −1, the capacity = the scope's +0xd (signed byte, as a word), the
/// count 0 and the entries (`exe::crt_malloc`(14 × capacity), cdecl; out of memory, the box of line 0x3e and nothing
/// more); the scope copied (three dwords and a word, `SCOPE`); with a file, AWARD_SHORTLIST::load(file) (thiscall, ret
/// 4; its return not read). Returns this (the original builds the date under an SEH frame; nothing here throws).
/// Original: `AWARD_SHORTLIST::AWARD_SHORTLIST` (0x417440, award_shortlist.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_award_shortlist(this: usize, scope: usize, file: usize) -> usize {
    use award_shortlist_fields as asl;
    exe::date_date_54a110(this + asl::DATE);
    if scope == 0 {
        show_error_at(AWARD_SHORTLIST_CPP, 0x30);
        return this;
    }
    write_i16(this + asl::MANAGER_INDEX, -1);
    let capacity = read_i8(scope + 0xd) as i16; // mwg16: the shortlist's size from its scope
    write_i16(this + asl::CAPACITY, capacity);
    write_i16(this + asl::COUNT, 0);
    let entries = exe::crt_malloc((capacity as i32 * asl::ENTRY_SIZE as i32) as u32);
    write_usize(this + asl::ENTRIES, entries);
    if entries == 0 {
        show_error_at(AWARD_SHORTLIST_CPP, 0x3e);
        return this;
    }
    covered!("mwg_16 shortlist built");
    for k in (0..12).step_by(4) {
        write_u32(this + asl::SCOPE + k, read_u32(scope + k));
    }
    write_u16(this + asl::SCOPE + 12, read_u16(scope + 12));
    if file != 0 {
        let load: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(asl::FN_AWARD_SHORTLIST_LOAD);
        load(this, file);
    }
    this
}

// simplification: allocates the entries: it does not run twice. Original only; the plan: the capacity from the scope,
// count 0, the index −1, the scope copied and the entries there.
/// Comparer for `construct_award_shortlist`.
pub(crate) unsafe extern "thiscall" fn compare_construct_award_shortlist(this: usize, scope: usize, file: usize) -> usize {
    use award_shortlist_fields as asl;
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> usize>(%%I0%%)(this, scope, file);
    if scope == 0 || file != 0 || read_usize(this + asl::ENTRIES) == 0 {
        return r;
    }
    let ok = r == this && read_i16(this + asl::CAPACITY) == read_i8(scope + 0xd) as i16 && read_i16(this + asl::COUNT) == 0
        && read_i16(this + asl::MANAGER_INDEX) == -1 && (0..asl::SCOPE_SIZE).all(|k| read_u8(this + asl::SCOPE + k) == read_u8(scope + k));
    SCOREBOARD.record(ok, || format!("construct_award_shortlist({:#x}): capacity {} count {} index {}", scope, read_i16(this + asl::CAPACITY), read_i16(this + asl::COUNT), read_i16(this + asl::MANAGER_INDEX)));
    r
}

/// Appends a candidate to a shortlist: the player's id, the club's id (or −1), the score and no votes; whether the list
/// is now full.
///
/// A full list (count ≥ capacity, signed words): the error box (line 0xbf) and 1. Without the player: the box (line
/// 0xc6) and 0. Otherwise the entry at the count: the player's id, the club's id or −1 without a club, the score (the
/// original copies it through the x87 stack: a signalling NaN comes out quiet, as here) and 0 votes; the count goes up
/// by one and the result is whether it reached the capacity.
/// Original: `AWARD_SHORTLIST::add_entry` (0x4178a0, award_shortlist.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn award_shortlist_add_entry(this: usize, player: usize, club: usize, score: u32) -> u32 {
    use award_shortlist_fields as asl;
    let count = read_i16(this + asl::COUNT);
    if count >= read_i16(this + asl::CAPACITY) {
        show_error_at(AWARD_SHORTLIST_CPP, 0xbf);
        return 1;
    }
    if player == 0 {
        show_error_at(AWARD_SHORTLIST_CPP, 0xc6);
        return 0;
    }
    covered!("mwg_16 shortlist candidate");
    let e = read_usize(this + asl::ENTRIES).wrapping_add((count as isize * asl::ENTRY_SIZE as isize) as usize);
    write_u32(e, read_u32(player));
    write_u32(e + asl::ENTRY_CLUB_ID, if club != 0 { read_u32(club) } else { u32::MAX });
    let quiet = if score & 0x7f80_0000 == 0x7f80_0000 && score & 0x007f_ffff != 0 { score | 0x0040_0000 } else { score };
    write_u32(e + asl::ENTRY_SCORE, quiet);
    write_u16(e + asl::ENTRY_VOTES, 0); // mwg16: a new candidate without votes
    let count = count.wrapping_add(1);
    write_i16(this + asl::COUNT, count);
    (count == read_i16(this + asl::CAPACITY)) as u32
}

/// Comparer for `award_shortlist_add_entry`: side by side over the count and the new entry.
pub(crate) unsafe extern "thiscall" fn compare_award_shortlist_add_entry(this: usize, player: usize, club: usize, score: u32) -> u32 {
    use award_shortlist_fields as asl;
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32>(%%I1%%);
    let count = read_i16(this + asl::COUNT);
    let slot = read_usize(this + asl::ENTRIES).wrapping_add((count.max(0) as usize) * asl::ENTRY_SIZE);
    let full = count >= read_i16(this + asl::CAPACITY) || player == 0;
    let regions = if full { vec![(this + asl::COUNT, 2)] } else { vec![(this + asl::COUNT, 2), (slot, asl::ENTRY_SIZE)] };
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, player, club, score), || award_shortlist_add_entry(this, player, club, score));
    SCOREBOARD.record(equal && r == n, || format!("award_shortlist_add_entry(player {:#x}, club {:#x}) original={} {:x?} rust={} {:x?}", player, club, r, mo, n, mr));
    r
}

/// Whether a human manager (a staff record) is in a shortlist's scope (the announcement goes to him): by the scope type
/// (`SCOPE`): 0 his club's division is the scope's (`SCOPE_DIVISION`), 1 his club's nation (`SCOPE_NATION`), 2 his club's
/// nation's continent (`SCOPE_CONTINENT`), 3 his national team's nation's continent, 4 he has a club, 5 he has a national
/// team, any other everyone. A missing link on the way, not in scope.
pub(crate) unsafe fn shortlist_scope_has_manager(this: usize, s: usize) -> bool {
    use award_shortlist_fields as asl;
    let club = read_usize(s + staff::CLUB_JOB);
    let national = read_usize(s + staff::NATIONAL_JOB);
    let continent_of = |team: usize| -> Option<u32> {
        if team == 0 {
            return None;
        }
        let nation = read_usize(team + club::NATION);
        if nation == 0 {
            return None;
        }
        let continent = read_usize(nation + nation::CONTINENT);
        if continent == 0 { None } else { Some(read_u32(continent)) }
    };
    match read_u8(this + asl::SCOPE) {
        0 => club != 0 && read_usize(club + club::DIVISION) != 0 && read_u32(read_usize(club + club::DIVISION)) == read_u32(this + asl::SCOPE_DIVISION),
        1 => club != 0 && read_usize(club + club::NATION) != 0 && read_u32(read_usize(club + club::NATION)) == read_u32(this + asl::SCOPE_NATION),
        2 => continent_of(club) == Some(read_u32(this + asl::SCOPE_CONTINENT)),
        3 => continent_of(national) == Some(read_u32(this + asl::SCOPE_CONTINENT)),
        4 => club != 0,
        5 => national != 0,
        _ => true,
    }
}

/// Announces a shortlist: the candidates sorted by name, the shortlist registered with the award manager, its date set
/// and the shortlist news sent to each human manager in its scope.
///
/// The entries sorted by name (`exe::crt_qsort`(entries, count, 14, `FN_AWARD_SHORTLIST_COMPARE_BY_NAME`)); the index
/// from the award manager (AWARD_MANAGER::add_shortlist on [`G_AWARD_MANAGER`], thiscall, ret 4) into `MANAGER_INDEX`;
/// the date (`DATE`) = `date` (Date::operator= `exe::date_operator_3d`). A news item of type `SHORTLIST_NEWS`
/// (NewsItem::NewsItem `exe::new_newsitem`(item, 0x1c, 0)) with slot 0 `a`, slot 1 `b`, slot 2 the date's day (signed
/// word), slot 3 its year (word) and slot 4 0 (`exe::newsitem_set_slot`). Then the 16 human manager rows (the last 16
/// of the staff table, from the end: `staff_at`(count − 1 − k)) in use (`staff::CLASSIFICATION` ≠ 0) and in the scope
/// (`shortlist_scope_has_manager`) get the news (News::send_to_club_manager `exe::news_send_to_club_manager` on
/// [`G_NEWS`], with his club, or his national team without one). The original keeps the news item under an SEH frame;
/// nothing here throws.
/// Original: `AWARD_SHORTLIST::announce` (0x417a20, award_shortlist.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn award_shortlist_announce(this: usize, date: usize, a: u32, b: u32) {
    use award_shortlist_fields as asl;
    exe::crt_qsort(read_usize(this + asl::ENTRIES), read_i16(this + asl::COUNT) as i32 as u32, asl::ENTRY_SIZE as u32, asl::FN_AWARD_SHORTLIST_COMPARE_BY_NAME);
    // AWARD_MANAGER::add_shortlist returns a 16-bit index (some paths write only ax)
    let add: unsafe extern "thiscall" fn(usize, usize) -> u16 = core::mem::transmute(0x4147f0usize);
    let index = add(read_usize(G_AWARD_MANAGER), this);
    write_u16(this + asl::MANAGER_INDEX, index);
    let mut copy = [0u32; 2];
    exe::date_operator_3d(this + asl::DATE, copy.as_mut_ptr() as usize, date);
    let mut news = [0u32; 0x38];
    let item = news.as_mut_ptr() as usize;
    exe::new_newsitem(item, asl::SHORTLIST_NEWS, 0);
    exe::newsitem_set_slot(item, 0, a);
    exe::newsitem_set_slot(item, 1, b);
    exe::newsitem_set_slot(item, 2, read_i16(date) as i32 as u32);
    exe::newsitem_set_slot(item, 3, read_u16(date + 2) as u32);
    exe::newsitem_set_slot(item, 4, 0);
    covered!("mwg_16 shortlist announced");
    for k in 0..16isize {
        let s = staff_at(read_i32(G_STAFF_COUNT) as isize - k - 1).addr();
        if read_u8(s + staff::CLASSIFICATION) == 0 || !shortlist_scope_has_manager(this, s) {
            continue;
        }
        let club = read_usize(s + staff::CLUB_JOB);
        let target = if club != 0 { club } else { read_usize(s + staff::NATIONAL_JOB) };
        exe::news_send_to_club_manager(read_usize(G_NEWS), item, target);
    }
}

// simplification: registers the shortlist and sends news: it does not run twice. Original only; the plan: the index set
// and the date copied.
/// Comparer for `award_shortlist_announce`.
pub(crate) unsafe extern "thiscall" fn compare_award_shortlist_announce(this: usize, date: usize, a: u32, b: u32) {
    use award_shortlist_fields as asl;
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32)>(%%I2%%)(this, date, a, b);
    let ok = read_i16(this + asl::MANAGER_INDEX) >= 0 && read_u32(this + asl::DATE) == read_u32(date);
    SCOREBOARD.record(ok, || format!("award_shortlist_announce({:#x}): index {} date {:#x}", this, read_i16(this + asl::MANAGER_INDEX), read_u32(this + asl::DATE)));
}
