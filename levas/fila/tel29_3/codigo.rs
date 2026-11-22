// ---------- batch %%LEVA%%: the award shortlists (2): loading and saving one, the computer clubs' votes, a human manager's vote and whether he may vote (award_shortlist.cpp) ----------

/// The 16 human managers' vote slots of a shortlist (+8, signed words: the entry he voted for, −1 none yet), indexed by
/// the manager's place among the last 16 staff records.
pub(crate) const AWARD_SHORTLIST_VOTES: usize = 0x8;

/// A human manager's vote slot in a shortlist: his id − staff count + 16 (the human managers are the last 16 staff
/// records), valid in 0..16.
pub(crate) unsafe fn shortlist_vote_slot(manager: usize) -> i32 {
    read_i32(manager).wrapping_sub(read_i32(G_STAFF_COUNT)).wrapping_add(16)
}

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

// simplification: reads the save file: it does not run twice. Original only; the plan: loaded, the count within the
// capacity and the vote slots −1 or within the count.
/// Comparer for `award_shortlist_load`.
pub(crate) unsafe extern "thiscall" fn compare_award_shortlist_load(this: usize, file: usize) -> u32 {
    use award_shortlist_fields as asl;
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I0%%)(this, file);
    if r != 1 {
        return r;
    }
    let count = read_i16(this + asl::COUNT);
    let ok = count <= read_i16(this + asl::CAPACITY)
        && (0..16).all(|k| { let v = read_i16(this + AWARD_SHORTLIST_VOTES + 2 * k); v == -1 || (0..count).contains(&v) });
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

// simplification: writes the save file: it does not run twice. Original only; the save proof (prova_save.py) checks
// the file the copy writes.
/// Comparer for `award_shortlist_save`.
pub(crate) unsafe extern "thiscall" fn compare_award_shortlist_save(this: usize, file: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%)(this, file)
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
                let n = read_usize(c + club::NATION);
                if n == 0 { None } else { id_of(read_usize(n + nation::CONTINENT)) }
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *mut usize, i16)>(%%I2%%);
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
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> usize>(%%I3%%)(this, manager, player);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> usize>(%%I3%%);
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
    use award_shortlist_fields as asl;
    if read_i16(this + asl::COUNT) == 0 {
        return -2;
    }
    let slot = shortlist_vote_slot(manager);
    let scope = read_u8(this + asl::SCOPE) as i8;
    if !(0..16).contains(&slot) || (read_i16(this + AWARD_SHORTLIST_VOTES + 2 * slot as usize) <= -1 && !(0..=6).contains(&scope)) {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize) -> i16>(%%I4%%)(this, manager);
    }
    let vote = read_i16(this + AWARD_SHORTLIST_VOTES + 2 * slot as usize);
    if vote > -1 {
        return vote;
    }
    let club = read_usize(manager + staff::CLUB_JOB);
    let national = read_usize(manager + staff::NATIONAL_JOB);
    let continent_id = |team: usize| -> Option<u32> {
        if team == 0 {
            return None;
        }
        let n = read_usize(team + club::NATION);
        if n == 0 {
            return None;
        }
        let c = read_usize(n + nation::CONTINENT);
        if c == 0 { None } else { Some(read_u32(c)) }
    };
    let may = match scope {
        0 => club != 0 && read_u32(club) == read_u32(this + asl::SCOPE_DIVISION) && read_i32(this + asl::SCOPE_DIVISION) != -1,
        1 => club != 0 && read_usize(club + club::NATION) != 0 && read_u32(read_usize(club + club::NATION)) == read_u32(this + asl::SCOPE_NATION)
            && read_i32(this + asl::SCOPE_NATION) != -1,
        // the original's slots: the continent compared with the nation slot (see the header)
        2 => continent_id(club) == Some(read_u32(this + asl::SCOPE_NATION)) && read_i32(this + asl::SCOPE_CONTINENT) != -1,
        3 => continent_id(national) == Some(read_u32(this + asl::SCOPE_NATION)) && read_i32(this + asl::SCOPE_CONTINENT) != -1,
        4 => club != 0,
        5 => national != 0,
        _ => true,
    };
    if may { -1 } else { -2 } // tel29_3: −1 may still vote, −2 may not
}

/// Comparer for `award_shortlist_get_vote_status` (the error paths only through the original: the box would show twice).
pub(crate) unsafe extern "thiscall" fn compare_award_shortlist_get_vote_status(this: usize, manager: usize) -> i16 {
    use award_shortlist_fields as asl;
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> i16>(%%I4%%);
    let slot = shortlist_vote_slot(manager);
    if read_i16(this + asl::COUNT) != 0 && (!(0..16).contains(&slot) || (read_i16(this + AWARD_SHORTLIST_VOTES + 2 * slot as usize) <= -1 && !(0..=6).contains(&(read_u8(this + asl::SCOPE) as i8)))) {
        return o(this, manager);
    }
    let r = o(this, manager);
    let n = award_shortlist_get_vote_status(this, manager);
    SCOREBOARD.record(r == n, || format!("award_shortlist_get_vote_status(scope {}) original={} rust={}", read_u8(this + asl::SCOPE), r, n));
    r
}
