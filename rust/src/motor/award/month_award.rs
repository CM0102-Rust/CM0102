use super::*;

/// Club record with the given id (0x245 bytes each).
pub(crate) unsafe fn month_award_club_by_id(id: i32) -> Club {
    Club(read_usize(G_CLUB_TABLE).wrapping_add((id as usize).wrapping_mul(club::SIZE)))
}

/// The comparers' rehearsal run. The copy runs without the news item (`exe::month_award_post_winner_news`), without the award record (`add_month_award_record`)
/// and without the club lists (`add_player_to_club_shortlists`), because these allocate and cannot run twice.
pub(crate) static MONTH_AWARD_DRY_RUN: AtomicBool = AtomicBool::new(false);

/// Picks the best player of the month ratings with the requested age and records him as the month winner, with his
/// club. `MONTH_AWARD::award_best_rated_player(notes, age)`, called at the turn of the month by the nations' awards
/// (England's at 0x579a37..0x579adc). It takes the best of the ratings (`exe::month_ratings_find_best_player`) in the
/// award's league (`[+9]`, or −1) and with the requested age; when one is found, it records the winner
/// (`exe::month_award_set_winner`) with the person's club (`staff::CLUB_JOB`, or −1), and the month
/// (`month_ratings_fields::MONTH`) and year (`month_ratings_fields::YEAR`) of the ratings. The high bytes of the month
/// and the year are passed exactly as the original pushes them (taken from the person table, `G_STAFF_TABLE`, in the
/// register).
/// Original: `MONTH_AWARD::award_best_rated_player` (0x767150, month_award.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn award_best_rated_player(this: usize, notes: usize, age: u32) {
    let league = {
        let l = read_usize(this + 9);
        if l != 0 {
            read_i32(l)
        } else {
            -1
        }
    };
    let r = exe::month_ratings_find_best_player(notes, league, age);
    if r == 0 {
        return;
    }
    covered!("cpw_6 awarded the best of the month");
    let id = read_i32(r);
    let table = read_usize(G_STAFF_TABLE) as u32;
    let club = read_usize(month_ratings_staff_by_id(id) + staff::CLUB_JOB);
    let year = (table & 0xffff0000) | read_i16(notes + month_ratings_fields::YEAR) as u16 as u32;
    let month = read_u8(notes + month_ratings_fields::MONTH) as u32;
    if club != 0 {
        exe::month_award_set_winner(this, id, read_i32(club), (year & 0xffffff00) | month, year);
    } else {
        exe::month_award_set_winner(this, id, -1, month, year);
    }
}

// simplification: its job is to call the player of the month and the winner’s slot, which have their own comparer (side by side and in
// rehearsal); the slot calls the news and the award record, which allocate: original only; the copy takes the branch.

/// Raises the award winner's reputation: the port directly in dry run, else the original `award_raise_winner_reputation`.
pub(crate) unsafe fn raise_winner_reputation(person: usize, club: usize, nation: u32, world: u32, player: u32) {
    if MONTH_AWARD_DRY_RUN.load(Relaxed) {
        return award_raise_winner_reputation(person, club, nation, world, player);
    }
    exe::award_raise_winner_reputation(person, club, nation, world, player);
}

/// Finds the month slot, stores the person and the club, raises the winner's reputation, sends the news and records the
/// award in the history. `MONTH_AWARD::set_winner(id, club_id, month, year)`. It looks for the month's slot (12 slots
/// of 11 bytes: month at `month_award_fields::SLOT_MONTH`, year at `month_award_fields::SLOT_YEAR`); without one, it
/// does nothing. It stores the person (`month_award_fields::SLOT_WINNER_ID`) and the club
/// (`month_award_fields::SLOT_CLUB_ID`); when there is a person, it raises the award winner's reputation
/// (`raise_winner_reputation`, with the award's reputations `award_fields::NATIONAL_REPUTATION` and
/// `award_fields::WORLD_REPUTATION` and the player flag `award_fields::FOR_PLAYERS`); then the news
/// (`exe::month_award_post_winner_news`) and the award record (`exe::staff_records_add_month_award_record` on
/// `G_STAFF_RECORDS`, with the award's +5).
/// Original: `MONTH_AWARD::set_winner` (0x7671c0, month_award.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn set_winner(this: usize, id: i32, club_id: i32, month: u32, year: u32) {
    let mut slot = None;
    for k in 0..12usize {
        let v = this + k * 11;
        if read_u8(v + month_award_fields::SLOT_MONTH) == month as u8 && read_i16(v + month_award_fields::SLOT_YEAR) == year as u16 as i16 {
            slot = Some(k);
            break;
        }
    }
    let k = match slot {
        Some(k) => k,
        None => return,
    };
    let v = this + k * 11;
    write_u32(v + month_award_fields::SLOT_WINNER_ID, id as u32);
    write_u32(v + month_award_fields::SLOT_CLUB_ID, club_id as u32);
    if id > -1 {
        let club = if club_id > -1 { month_award_club_by_id(club_id) } else { Club(0) };
        // the high bytes of the reputations come from dword +0x19 in the register, as the original pushes
        let mark = read_usize(this + award_fields::FOR_PLAYERS) as u32;
        let high = mark & 0xffff0000;
        covered!("cpw_6 winner of the month");
        raise_winner_reputation(month_ratings_staff_by_id(id), club.addr(), high | read_i16(this + award_fields::NATIONAL_REPUTATION) as u16 as u32, high | read_i16(this + award_fields::WORLD_REPUTATION) as u16 as u32, mark);
    }
    if MONTH_AWARD_DRY_RUN.load(Relaxed) {
        return;
    }
    exe::month_award_post_winner_news(this, (this as u32 & 0xffffff00) | k as u32);
    let cid = read_i32(v + month_award_fields::SLOT_CLUB_ID);
    let club = if cid != -1 { month_award_club_by_id(cid) } else { Club(0) };
    exe::staff_records_add_month_award_record(read_usize(G_STAFF_RECORDS), month_ratings_staff_by_id(read_i32(v + month_award_fields::SLOT_WINNER_ID)), club, read_usize(this + 5) as u32, month, year);
}

/// Adds the regions the winner's reputation writes: player record +9..+0xe, non-player record +8..+0xd and the club's
/// board row word (+6 of 73 bytes).
pub(crate) unsafe fn winner_reputation_regions(regions: &mut Vec<(usize, usize)>, person: usize, club: usize) {
    let person = Staff(person);
    let p = person.player();
    if !p.is_null() {
        regions.push((p.addr() + 9, 6));
    }
    let np = person.non_player();
    if !np.is_null() {
        regions.push((np.addr() + 8, 6));
    }
    if club != 0 {
        let rows = read_usize(read_usize(G_MANAGER_MANAGER));
        regions.push((rows.wrapping_add((read_i32(club) as usize).wrapping_mul(73)).wrapping_add(6), 2));
    }
}

/// Dry-run comparer: runs the port without allocating calls, then restores and runs the original from the same region
/// state, comparing the regions.
pub(crate) unsafe fn compare_month_award(name: &str, regions: &[(usize, usize)], original: impl FnOnce(), copy: impl FnOnce()) {
    let before = crate::comparison::MemorySnapshot::read(regions);
    MONTH_AWARD_DRY_RUN.store(true, Relaxed);
    crate::rng::pause_trace(copy);
    MONTH_AWARD_DRY_RUN.store(false, Relaxed);
    let mr = crate::comparison::MemorySnapshot::read(regions);
    before.write_back();
    original();
    let mo = crate::comparison::MemorySnapshot::read(regions);
    SCOREBOARD.record(mo == mr, || format!("{} original={:x?} rust={:x?}", name, mo, mr));
}

/// Adds the low 16 bits of v to the 16-bit word at a.
pub(crate) unsafe fn add_to_word(a: usize, v: i32) {
    write_i16(a, read_i16(a).wrapping_add(v as i16));
}

/// Clamps the signed 16-bit word at a to at most 10000 and at least `piso`.
pub(crate) unsafe fn clamp_word(a: usize, floor: i16) {
    if read_i16(a) > 10000 {
        write_i16(a, 10000);
    }
    if read_i16(a) <= floor {
        write_i16(a, floor);
    }
}

/// Moves a month award on to the next season: unless the first month slot is already in `year` (or no slot is in
/// use), every used slot (`MONTH_AWARD_SLOT_COUNT`, read again at each step) goes one year on
/// (`month_award_fields::SLOT_YEAR` + 1) and loses its winner and club (`SLOT_WINNER_ID`, `SLOT_CLUB_ID` = −1). Called
/// by each nation's `daily_update` on the nation's season start day, with the current year.
/// Original: `MONTH_AWARD::start_new_season` (0x7672e0, month_award.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn month_award_new_season(this: usize, year: u16) {
    if read_i8(this + MONTH_AWARD_SLOT_COUNT) <= 0 || read_u16(this + month_award_fields::SLOT_YEAR) == year {
        return;
    }
    covered!("sx04_1 month slots moved on");
    let mut k = 0;
    while k < read_i8(this + MONTH_AWARD_SLOT_COUNT) as i32 {
        let slot = this + k as usize * MONTH_AWARD_SLOT_SIZE;
        write_u16(slot + month_award_fields::SLOT_YEAR, read_u16(slot + month_award_fields::SLOT_YEAR).wrapping_add(1)); // sx04_1: one year on
        write_i32(slot + month_award_fields::SLOT_WINNER_ID, -1);
        write_i32(slot + month_award_fields::SLOT_CLUB_ID, -1);
        k += 1;
    }
}

/// Comparer for `month_award_new_season`: side by side over the twelve month slots.
pub(crate) unsafe extern "thiscall" fn compare_month_award_new_season(this: usize, year: u16) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u16)>(IDX_MONTH_AWARD_NEW_SEASON);
    let regions = [(this + month_award_fields::SLOT_WINNER_ID, 12 * MONTH_AWARD_SLOT_SIZE)];
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, year), || month_award_new_season(this, year));
    SCOREBOARD.record(same, || format!("month_award_new_season(year {}) original={:x?} rust={:x?}", year, mo, mr));
}

/// One month slot of a month award (12 of them from `month_award_fields::SLOT_WINNER_ID`).
pub(crate) const MONTH_AWARD_SLOT_SIZE: usize = 11;

/// Reads a month award from the saved game: its twelve month slots (`MONTH_AWARD_SLOT_SIZE` bytes each, from
/// `month_award_fields::SLOT_WINNER_ID`) and its slot count (`MONTH_AWARD_SLOT_COUNT`, a byte), each with
/// `exe::zread`. Returns 1, or 0 after the Load Game Error box of the first short read (month_award.cpp 110 "cannot
/// read months", 117 "cannot read no of months").
/// Original: `MONTH_AWARD::load_from_disk` (0x766f50, month_award.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn month_award_load_from_disk(this: usize, file: usize) -> u32 {
    if exe::zread(this + month_award_fields::SLOT_WINNER_ID, MONTH_AWARD_SLOT_SIZE as u32, 12, file) != 12 {
        award_load_error_box(MONTH_AWARD_CPP, 0x6e, 0xa5abb0, LOAD_ERROR);
        return 0;
    }
    if exe::zread(this + MONTH_AWARD_SLOT_COUNT, 1, 1, file) != 1 { // sx04b_6: the slot count
        award_load_error_box(MONTH_AWARD_CPP, 0x75, 0xa5ab78, LOAD_ERROR);
        return 0;
    }
    covered!("sx04b_6 month award loaded");
    1
}

/// Comparer for `month_award_load_from_disk`: the load reads the file once; only the original runs, and a loaded
/// award must keep at most twelve slots.
pub(crate) unsafe extern "thiscall" fn compare_month_award_load_from_disk(this: usize, file: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_MONTH_AWARD_LOAD_FROM_DISK)(this, file);
    if r == 1 {
        let n = read_i8(this + MONTH_AWARD_SLOT_COUNT);
        SCOREBOARD.record((0..=12).contains(&n), || format!("month_award_load_from_disk({:#x}): {} slots", this, n));
    }
    r
}

/// Writes a month award to the saved game as `month_award_load_from_disk` reads it: the twelve month slots, then the
/// slot count (`exe::zwrite`); a short write shows the "Error" box (month_award.cpp 135, 142) and stops there.
/// Original: `MONTH_AWARD::save_to_disk` (0x767070, month_award.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn month_award_save_to_disk(this: usize, file: usize) {
    if exe::zwrite(this + month_award_fields::SLOT_WINNER_ID, MONTH_AWARD_SLOT_SIZE as u32, 12, file) != 12 { // sx04b_6: the twelve slots
        return tel26_error_box(MONTH_AWARD_CPP, 0x87);
    }
    if exe::zwrite(this + MONTH_AWARD_SLOT_COUNT, 1, 1, file) != 1 {
        return tel26_error_box(MONTH_AWARD_CPP, 0x8e);
    }
    covered!("sx04b_6 month award saved");
}

/// Comparer for `month_award_save_to_disk`: it writes the file once; only the original runs (the save proof compares
/// the written game).
pub(crate) unsafe extern "thiscall" fn compare_month_award_save_to_disk(this: usize, file: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_MONTH_AWARD_SAVE_TO_DISK)(this, file)
}

/// "E:\dev\CM3\cm3 00-01\cm3\code\award\month_award.cpp" and "...\month_ratings.cpp" (the error boxes).
pub(crate) const MONTH_AWARD_CPP: usize = 0xa5abe4;
