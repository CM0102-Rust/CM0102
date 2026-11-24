use super::*;

/// Loads a shortlist from a saved game: the count and capacity (4 bytes at +4), the entries (14 bytes each, when the
/// count is not 0), the 16 vote slots and the date; 1, or 0 after the error box of the first short read.
///
/// Each read is `exe::zread`(dest, size, count, file), cdecl, which returns the number of items read; a short read
/// shows the box (`award_shortlist.cpp` 103, 113, 122, 129 in that order) and returns 0.
/// Original: `AWARD_SHORTLIST::load` (0x417590, award_shortlist.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn award_shortlist_load(this: usize, file: usize) -> u32 {
    use award_shortlist_fields as asl;
    if exe::zread(this + asl::COUNT, 4, 1, file) != 1 {
        show_error_at(AWARD_SHORTLIST_CPP, 0x67);
        return 0;
    }
    let count = read_i16(this + asl::COUNT);
    if count != 0 && exe::zread(read_usize(this + asl::ENTRIES), asl::ENTRY_SIZE as u32, count as i32 as u32, file) != count as i32 as u32 {
        show_error_at(AWARD_SHORTLIST_CPP, 0x71);
        return 0;
    }
    if exe::zread(this + AWARD_SHORTLIST_VOTES, 2, 16, file) != 16 {
        show_error_at(AWARD_SHORTLIST_CPP, 0x7a);
        return 0;
    }
    if exe::zread(this + asl::DATE, 8, 1, file) != 1 {
        show_error_at(AWARD_SHORTLIST_CPP, 0x81);
        return 0;
    }
    covered!("tel29_3 shortlist loaded");
    1
}

/// Comparer for `award_shortlist_load`.
pub(crate) unsafe extern "thiscall" fn compare_award_shortlist_load(this: usize, file: usize) -> u32 {
    use award_shortlist_fields as asl;
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_AWARD_SHORTLIST_LOAD)(this, file);
    if r != 1 {
        return r;
    }
    let count = read_i16(this + asl::COUNT);
    // only the count within the capacity: the vote slots come as saved, and a save had a vote 0 on an empty shortlist
    // (a vote left from an earlier round; the original reads it as is), 22/11/2026
    let ok = count <= read_i16(this + asl::CAPACITY);
    SCOREBOARD.record(ok, || format!("award_shortlist_load({:#x}): count {} capacity {}", this, count, read_i16(this + asl::CAPACITY)));
    r
}

/// Saves a shortlist to the saved game, as `award_shortlist_load` reads it: the count and capacity, the entries (when
/// the count is not 0), the 16 vote slots and the date (`exe::zwrite`, cdecl); a short write shows the box
/// (`award_shortlist.cpp` 147, 156, 165, 172) and stops there.
/// Original: `AWARD_SHORTLIST::save` (0x417720, award_shortlist.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn award_shortlist_save(this: usize, file: usize) {
    use award_shortlist_fields as asl;
    if exe::zwrite(this + asl::COUNT, 4, 1, file) != 1 {
        return show_error_at(AWARD_SHORTLIST_CPP, 0x93);
    }
    let count = read_i16(this + asl::COUNT);
    if count != 0 && exe::zwrite(read_usize(this + asl::ENTRIES), asl::ENTRY_SIZE as u32, count as i32 as u32, file) != count as i32 as u32 {
        return show_error_at(AWARD_SHORTLIST_CPP, 0x9c);
    }
    if exe::zwrite(this + AWARD_SHORTLIST_VOTES, 2, 16, file) != 16 {
        return show_error_at(AWARD_SHORTLIST_CPP, 0xa5);
    }
    if exe::zwrite(this + asl::DATE, 8, 1, file) != 1 { // tel29_3: the date closes the shortlist's block
        return show_error_at(AWARD_SHORTLIST_CPP, 0xac);
    }
    covered!("tel29_3 shortlist saved");
}

/// Comparer for `award_shortlist_save`.
pub(crate) unsafe extern "thiscall" fn compare_award_shortlist_save(this: usize, file: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_AWARD_SHORTLIST_SAVE)(this, file)
}

/// Picks the winners: the entries sorted by score, one vote from each computer-run club in the award's scope, then
/// sorted by votes and the first `count` players written to `out`.
///
/// Nothing with no entries. The entries sorted by score (`exe::crt_qsort`, comparer 0x4183f0). Then, by the scope type
/// (`award_shortlist_fields::SCOPE`), over the club table ([`G_CLUB_TABLE`], 0x245 bytes each; the first `club count −
/// 2 × nation count` are the clubs, the next `nation count` the national teams): 0 the clubs of the scope's division
/// (`club::DIVISION` → id = `SCOPE_DIVISION`), 1 of its nation (`club::NATION` → id = `SCOPE_NATION`), 2 of its continent
/// (the nation's `CONTINENT` → id = `SCOPE_CONTINENT`), 3 the national teams of its continent, each one only when no
/// human runs it (`exe::human_manager_is_human_club`(club, 0, 1, 0, 0) = 0); 4 as many votes as there are clubs, 5 as
/// many as national teams, 6 as many as the whole table, without looking at any club; other types no votes. Each vote:
/// `shortlist_computer_vote`, the turn starting at 0. Then sorted by votes (comparer 0x418420) and the first `count`
/// entries (at most the entry count) go to `out` as staff records (`staff_at`(id)).
/// Original: `AWARD_SHORTLIST::pick_winners` (0x417c50, award_shortlist.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn award_shortlist_pick_winners(this: usize, out: *mut usize, count: i16) {
    use award_shortlist_fields as asl;
    let entries = read_i16(this + asl::COUNT);
    if entries == 0 {
        return;
    }
    covered!("tel29_3 winners picked");
    exe::crt_qsort(read_usize(this + asl::ENTRIES), entries as i32 as u32, asl::ENTRY_SIZE as u32, 0x4183f0usize);
    let nations = read_i32(G_NATION_COUNT);
    let clubs = read_i32(G_CLUB_COUNT);
    let club_part = clubs.wrapping_sub(nations.wrapping_mul(2));
    let club_rec = |k: i32| read_usize(G_CLUB_TABLE).wrapping_add((k as isize * club::SIZE as isize) as usize);
    let computer = |c: usize| exe::human_manager_is_human_club(c, 0, 1, 0, 0) == 0;
    let id_of = |p: usize| if p == 0 { None } else { Some(read_u32(p)) };
    let mut turn: i16 = 0;
    match read_u8(this + asl::SCOPE) {
        0 => for k in 0..club_part {
            let c = club_rec(k);
            if id_of(read_usize(c + club::DIVISION)) == Some(read_u32(this + asl::SCOPE_DIVISION)) && computer(c) {
                shortlist_computer_vote(this, &mut turn);
            }
        },
        1 => for k in 0..club_part {
            let c = club_rec(k);
            if id_of(read_usize(c + club::NATION)) == Some(read_u32(this + asl::SCOPE_NATION)) && computer(c) {
                shortlist_computer_vote(this, &mut turn);
            }
        },
        scope @ (2 | 3) => {
            let continent = |c: usize| {
                let n = Nation(read_usize(c + club::NATION));
                if n.is_null() { None } else { id_of(n.continent()) }
            };
            // continent clubs: a fixed count of clubs; continent national teams: up to the bound read anew each time
            let mut k = if scope == 2 { 0 } else { club_part };
            let end = || if scope == 2 { club_part } else { read_i32(G_CLUB_COUNT).wrapping_sub(read_i32(G_NATION_COUNT)) };
            while k < end() {
                let c = club_rec(k);
                if continent(c) == Some(read_u32(this + asl::SCOPE_CONTINENT)) && computer(c) {
                    shortlist_computer_vote(this, &mut turn);
                }
                k += 1;
            }
        }
        4 => for _ in 0..club_part {
            shortlist_computer_vote(this, &mut turn);
        },
        5 => {
            let mut k = club_part;
            while k < read_i32(G_CLUB_COUNT).wrapping_sub(read_i32(G_NATION_COUNT)) {
                shortlist_computer_vote(this, &mut turn);
                k += 1;
            }
        }
        6 => {
            let mut k = 0;
            while k < read_i32(G_CLUB_COUNT) {
                shortlist_computer_vote(this, &mut turn);
                k += 1;
            }
        }
        _ => {}
    }
    exe::crt_qsort(read_usize(this + asl::ENTRIES), read_i16(this + asl::COUNT) as i32 as u32, asl::ENTRY_SIZE as u32, 0x418420usize);
    let mut k = 0i32;
    while k < count as i32 && k < read_i16(this + asl::COUNT) as i32 {
        let id = read_i32(read_usize(this + asl::ENTRIES) + k as usize * asl::ENTRY_SIZE);
        *out.add(k as i16 as isize as usize) = staff_at(id as isize).addr();
        k += 1;
    }
}

/// Comparer for `award_shortlist_pick_winners`: side by side over the entries, the winners written and the random
/// generator.
pub(crate) unsafe extern "thiscall" fn compare_award_shortlist_pick_winners(this: usize, out: *mut usize, count: i16) {
    use award_shortlist_fields as asl;
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *mut usize, i16)>(IDX_AWARD_SHORTLIST_PICK_WINNERS);
    let entries = read_i16(this + asl::COUNT);
    let mut regions = vec![(this, 0x40)];
    if entries > 0 {
        regions.push((read_usize(this + asl::ENTRIES), entries as usize * asl::ENTRY_SIZE));
        if count > 0 {
            regions.push((out as usize, count.min(entries) as usize * 4));
        }
    }
    regions.extend_from_slice(&crate::rng::STATE_REGIONS);
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, out, count),
        || crate::rng::without_counting(|| award_shortlist_pick_winners(this, out, count)));
    SCOREBOARD.record(equal, || format!("award_shortlist_pick_winners({:#x}, {}): differences em {:?}", this, count, first_differences(&mo, &mr)));
}

/// Records a human manager's vote for a shortlisted player: his vote slot (`shortlist_vote_slot`) gets the player's
/// entry and the entry one vote more; returns the address of the entry's vote count.
///
/// With the slot out of range (box `award_shortlist.cpp 540`), a vote already cast (box 547) or the player not on the
/// list (box 567): the original routine runs.
/// Original: `AWARD_SHORTLIST::cast_vote` (0x417fe0, award_shortlist.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn award_shortlist_cast_vote(this: usize, manager: usize, player: usize) -> usize {
    use award_shortlist_fields as asl;
    let slot = shortlist_vote_slot(manager);
    let entry = (0..read_i16(this + asl::COUNT)).find(|&k| read_u32(read_usize(this + asl::ENTRIES) + k as usize * asl::ENTRY_SIZE) == read_u32(player));
    let (k, true) = (entry.unwrap_or(0), (0..16).contains(&slot) && read_i16(this + AWARD_SHORTLIST_VOTES + 2 * slot as usize) <= -1 && entry.is_some()) else {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> usize>(IDX_AWARD_SHORTLIST_CAST_VOTE)(this, manager, player);
    };
    covered!("tel29_3 human vote");
    write_i16(this + AWARD_SHORTLIST_VOTES + 2 * slot as usize, k);
    let votes = read_usize(this + asl::ENTRIES) + k as usize * asl::ENTRY_SIZE + asl::ENTRY_VOTES;
    write_i16(votes, read_i16(votes).wrapping_add(1)); // tel29_3: the human's vote counts one
    votes
}

/// Comparer for `award_shortlist_cast_vote`: side by side over the vote slots and the entries (the error paths only
/// through the original: the box would show twice).
pub(crate) unsafe extern "thiscall" fn compare_award_shortlist_cast_vote(this: usize, manager: usize, player: usize) -> usize {
    use award_shortlist_fields as asl;
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> usize>(IDX_AWARD_SHORTLIST_CAST_VOTE);
    let slot = shortlist_vote_slot(manager);
    let count = read_i16(this + asl::COUNT);
    let found = (0..count).any(|k| read_u32(read_usize(this + asl::ENTRIES) + k as usize * asl::ENTRY_SIZE) == read_u32(player));
    if !(0..16).contains(&slot) || read_i16(this + AWARD_SHORTLIST_VOTES + 2 * slot as usize) > -1 || !found {
        return o(this, manager, player);
    }
    let regions = [(this + AWARD_SHORTLIST_VOTES, 0x20), (read_usize(this + asl::ENTRIES), count as usize * asl::ENTRY_SIZE)];
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, manager, player), || award_shortlist_cast_vote(this, manager, player));
    SCOREBOARD.record(equal && r == n, || format!("award_shortlist_cast_vote original={:#x} rust={:#x}: differences em {:?}", r, n, first_differences(&mo, &mr)));
    r
}

/// Whether a human manager may vote on a shortlist: the entry he voted for (≥ 0), −1 when he may still vote, −2 when
/// he may not.
///
/// An empty list: −2. With the vote slot (`shortlist_vote_slot`) out of range: the original routine runs (box
/// `award_shortlist.cpp 591`, −2). A vote cast: its entry. Otherwise by the scope type: 0 his club (`staff::CLUB_JOB`)
/// with an id equal to the scope's division (`SCOPE_DIVISION`) and that not −1; 1 his club's nation id equal to
/// `SCOPE_NATION` (not −1); 2 his club's nation's continent id equal to `SCOPE_NATION`, and `SCOPE_CONTINENT` not −1; 3
/// the same through his national team (`staff::NATIONAL_JOB`); 4 he has a club; 5 he has a national team; 6 always:
/// −1, otherwise −2; a type above 6: the original routine runs (box 677, −2). Defects of the original: type 0 compares
/// the club's own id with the division's id, and types 2 and 3 compare the continent's id with the nation slot of the
/// scope instead of the continent's, so a human manager is almost never allowed to vote on division or continent
/// awards (pick_winners and announce use the right fields).
/// Original: `AWARD_SHORTLIST::get_vote_status` (0x418140, award_shortlist.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn award_shortlist_get_vote_status(this: usize, manager: usize) -> i16 {
    let manager = Staff(manager);
    use award_shortlist_fields as asl;
    if read_i16(this + asl::COUNT) == 0 {
        return -2;
    }
    let slot = shortlist_vote_slot(manager.addr());
    let scope = read_u8(this + asl::SCOPE) as i8;
    if !(0..16).contains(&slot) || (read_i16(this + AWARD_SHORTLIST_VOTES + 2 * slot as usize) <= -1 && !(0..=6).contains(&scope)) {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize) -> i16>(IDX_AWARD_SHORTLIST_GET_VOTE_STATUS)(this, manager.addr());
    }
    let vote = read_i16(this + AWARD_SHORTLIST_VOTES + 2 * slot as usize);
    if vote > -1 {
        return vote;
    }
    let club = manager.club_job();
    let national = manager.national_job().addr();
    let continent_id = |team: usize| -> Option<u32> {
        if team == 0 {
            return None;
        }
        let n = Nation(read_usize(team + club::NATION));
        if n.is_null() {
            return None;
        }
        let c = n.continent();
        if c == 0 { None } else { Some(read_u32(c)) }
    };
    let may = match scope {
        0 => !club.is_null() && read_u32(club.addr()) == read_u32(this + asl::SCOPE_DIVISION) && read_i32(this + asl::SCOPE_DIVISION) != -1,
        1 => !club.is_null() && club.nation().addr() != 0 && read_u32(club.nation().addr()) == read_u32(this + asl::SCOPE_NATION)
            && read_i32(this + asl::SCOPE_NATION) != -1,
        // the original's slots: the continent compared with the nation slot (see the header)
        2 => continent_id(club.addr()) == Some(read_u32(this + asl::SCOPE_NATION)) && read_i32(this + asl::SCOPE_CONTINENT) != -1,
        3 => continent_id(national) == Some(read_u32(this + asl::SCOPE_NATION)) && read_i32(this + asl::SCOPE_CONTINENT) != -1,
        4 => !club.is_null(),
        5 => national != 0,
        _ => true,
    };
    if may { -1 } else { -2 } // tel29_3: −1 may still vote, −2 may not
}

/// Comparer for `award_shortlist_get_vote_status` (the error paths only through the original: the box would show twice).
pub(crate) unsafe extern "thiscall" fn compare_award_shortlist_get_vote_status(this: usize, manager: usize) -> i16 {
    use award_shortlist_fields as asl;
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> i16>(IDX_AWARD_SHORTLIST_GET_VOTE_STATUS);
    let slot = shortlist_vote_slot(manager);
    if read_i16(this + asl::COUNT) != 0 && (!(0..16).contains(&slot) || (read_i16(this + AWARD_SHORTLIST_VOTES + 2 * slot as usize) <= -1 && !(0..=6).contains(&(read_u8(this + asl::SCOPE) as i8)))) {
        return o(this, manager);
    }
    let r = o(this, manager);
    let n = award_shortlist_get_vote_status(this, manager);
    SCOREBOARD.record(r == n, || format!("award_shortlist_get_vote_status(scope {}) original={} rust={}", read_u8(this + asl::SCOPE), r, n));
    r
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

/// Comparer for `construct_award_shortlist`.
pub(crate) unsafe extern "thiscall" fn compare_construct_award_shortlist(this: usize, scope: usize, file: usize) -> usize {
    use award_shortlist_fields as asl;
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> usize>(IDX_CONSTRUCT_AWARD_SHORTLIST)(this, scope, file);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32>(IDX_AWARD_SHORTLIST_ADD_ENTRY);
    let count = read_i16(this + asl::COUNT);
    let slot = read_usize(this + asl::ENTRIES).wrapping_add((count.max(0) as usize) * asl::ENTRY_SIZE);
    let full = count >= read_i16(this + asl::CAPACITY) || player == 0;
    let regions = if full { vec![(this + asl::COUNT, 2)] } else { vec![(this + asl::COUNT, 2), (slot, asl::ENTRY_SIZE)] };
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, player, club, score), || award_shortlist_add_entry(this, player, club, score));
    SCOREBOARD.record(equal && r == n, || format!("award_shortlist_add_entry(player {:#x}, club {:#x}) original={} {:x?} rust={} {:x?}", player, club, r, mo, n, mr));
    r
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
    let index = exe::award_manager_add_shortlist(read_usize(G_AWARD_MANAGER), this);
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
        let s = Staff(staff_at(read_i32(G_STAFF_COUNT) as isize - k - 1).addr());
        if s.classification() == 0 || !shortlist_scope_has_manager(this, s.addr()) {
            continue;
        }
        let club = s.club_job().addr();
        let target = if club != 0 { club } else { s.national_job().addr() };
        exe::news_send_to_club_manager(read_usize(G_NEWS), item, target);
    }
}

/// Comparer for `award_shortlist_announce`.
pub(crate) unsafe extern "thiscall" fn compare_award_shortlist_announce(this: usize, date: usize, a: u32, b: u32) {
    use award_shortlist_fields as asl;
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32)>(IDX_AWARD_SHORTLIST_ANNOUNCE)(this, date, a, b);
    let ok = read_i16(this + asl::MANAGER_INDEX) >= 0 && read_u32(this + asl::DATE) == read_u32(date);
    SCOREBOARD.record(ok, || format!("award_shortlist_announce({:#x}): index {} date {:#x}", this, read_i16(this + asl::MANAGER_INDEX), read_u32(this + asl::DATE)));
}

/// A human manager's vote slot in a shortlist: his id − staff count + 16 (the human managers are the last 16 staff
/// records), valid in 0..16.
pub(crate) unsafe fn shortlist_vote_slot(manager: usize) -> i32 {
    read_i32(manager).wrapping_sub(read_i32(G_STAFF_COUNT)).wrapping_add(16)
}

/// One computer vote: the entry at a random place below the turn (`exe::utils_random`(turn), so the first vote goes to
/// the best score and later ones spread down the list) gets a vote; the turn goes round the entries.
pub(crate) unsafe fn shortlist_computer_vote(this: usize, turn: &mut i16) {
    use award_shortlist_fields as asl;
    let k = exe::utils_random(*turn as i32) as i16;
    let votes = read_usize(this + asl::ENTRIES).wrapping_add((k as isize * asl::ENTRY_SIZE as isize) as usize) + asl::ENTRY_VOTES;
    write_i16(votes, read_i16(votes).wrapping_add(1)); // tel29_3: a computer club's vote
    *turn += 1;
    if *turn >= read_i16(this + asl::COUNT) {
        *turn = 0;
    }
}

/// Whether a human manager (a staff record) is in a shortlist's scope (the announcement goes to him): by the scope type
/// (`SCOPE`): 0 his club's division is the scope's (`SCOPE_DIVISION`), 1 his club's nation (`SCOPE_NATION`), 2 his club's
/// nation's continent (`SCOPE_CONTINENT`), 3 his national team's nation's continent, 4 he has a club, 5 he has a national
/// team, any other everyone. A missing link on the way, not in scope.
pub(crate) unsafe fn shortlist_scope_has_manager(this: usize, s: usize) -> bool {
    let s = Staff(s);
    use award_shortlist_fields as asl;
    let club = s.club_job();
    let national = s.national_job().addr();
    let continent_of = |team: usize| -> Option<u32> {
        if team == 0 {
            return None;
        }
        let nation = Nation(read_usize(team + club::NATION));
        if nation.is_null() {
            return None;
        }
        let continent = nation.continent();
        if continent == 0 { None } else { Some(read_u32(continent)) }
    };
    match read_u8(this + asl::SCOPE) {
        0 => !club.is_null() && club.division().addr() != 0 && read_u32(club.division().addr()) == read_u32(this + asl::SCOPE_DIVISION),
        1 => !club.is_null() && club.nation().addr() != 0 && read_u32(club.nation().addr()) == read_u32(this + asl::SCOPE_NATION),
        2 => continent_of(club.addr()) == Some(read_u32(this + asl::SCOPE_CONTINENT)),
        3 => continent_of(national) == Some(read_u32(this + asl::SCOPE_CONTINENT)),
        4 => !club.is_null(),
        5 => national != 0,
        _ => true,
    }
}

/// The 16 human managers' vote slots of a shortlist (+8, signed words: the entry he voted for, −1 none yet), indexed by
/// the manager's place among the last 16 staff records.
pub(crate) const AWARD_SHORTLIST_VOTES: usize = 0x8;

/// "E:\dev\CM3\cm3 00-01\cm3\code\award\award_shortlist.cpp" (the error boxes).
pub(crate) const AWARD_SHORTLIST_CPP: usize = 0x9880b0;

/// The qsort order of a shortlist by the players' names: each entry's player (the staff id at +0; none for −1) named
/// by `exe::database_format_staff_name` (2000 bytes, style 4; an empty name without a player), the two names compared
/// by `exe::utils_string_compare` (mode 1, 0).
/// Original: `award_shortlist_compare_by_name` (0x418340, award_shortlist.cpp, cdecl).
pub(crate) unsafe extern "C" fn award_shortlist_compare_by_name(a: *const usize, b: *const usize) -> i32 {
    let named = |entry: usize, buf: &mut [u8; 0x7d0]| {
        let id = read_i32(entry);
        if id != -1 {
            exe::database_format_staff_name(staff_at(id as isize), buf.as_mut_ptr() as usize, 0x7d0, 4);
        }
    };
    let (mut name_a, mut name_b) = ([0u8; 0x7d0], [0u8; 0x7d0]);
    named(a as usize, &mut name_a);
    named(b as usize, &mut name_b);
    exe::utils_string_compare(name_a.as_ptr(), name_b.as_ptr(), 1, 0) // tel44_2: a's name against b's
}

/// The qsort order of a shortlist by score, the highest first: −1 when a's score (float, `ENTRY_SCORE`) is above b's,
/// 1 when below, otherwise 0 (also unordered).
/// Original: `award_shortlist_compare_by_score` (0x4183f0, award_shortlist.cpp, cdecl).
pub(crate) unsafe extern "C" fn award_shortlist_compare_by_score(a: *const usize, b: *const usize) -> i32 {
    use award_shortlist_fields as asl;
    let (sa, sb) = (read_f32(a as usize + asl::ENTRY_SCORE), read_f32(b as usize + asl::ENTRY_SCORE));
    if sa > sb {
        -1 // tel44_2: the higher score first
    } else if sb > sa {
        1
    } else {
        0
    }
}

/// The qsort order of a shortlist by votes, the most voted first: b's votes minus a's (signed words, `ENTRY_VOTES`).
/// Original: `award_shortlist_compare_by_votes` (0x418420, award_shortlist.cpp, cdecl).
pub(crate) unsafe extern "C" fn award_shortlist_compare_by_votes(a: *const usize, b: *const usize) -> i32 {
    use award_shortlist_fields as asl;
    read_i16(b as usize + asl::ENTRY_VOTES) as i32 - read_i16(a as usize + asl::ENTRY_VOTES) as i32 // tel44_2: the most votes first
}

/// Empties a shortlist: no entries (`COUNT` 0) and the 16 managers' vote slots (`AWARD_SHORTLIST_VOTES`, words) −1.
/// The original leaves −1 in eax; its two callers (nation awards, 0x7683c0 and 0x768b20) do not read it.
/// Original: `AWARD_SHORTLIST::clear` (0x418320, award_shortlist.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn award_shortlist_clear(this: usize) {
    use award_shortlist_fields as asl;
    write_i16(this + asl::COUNT, 0);
    for k in 0..16 {
        write_i16(this + AWARD_SHORTLIST_VOTES + 2 * k, -1); // tel44_3: no manager has voted
    }
}

/// Comparer for `award_shortlist_clear`: side by side over the count and the vote slots (+4..+0x28).
pub(crate) unsafe extern "thiscall" fn compare_award_shortlist_clear(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_AWARD_SHORTLIST_CLEAR);
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&[(this + award_shortlist_fields::COUNT, 0x24)], || o(this), || award_shortlist_clear(this));
    SCOREBOARD.record(equal, || format!("AWARD_SHORTLIST::clear({:#x}): differences in {:?}", this, first_differences(&mo, &mr)));
}
