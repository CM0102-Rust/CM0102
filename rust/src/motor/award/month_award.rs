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

/// Builds a month award: the AWARD base (`AWARD::AWARD` with the nation index, the award index, the staff competition,
/// the reputations, the club competition, the file and the player flag), the `MONTH_AWARD_VTABLE`, the type
/// `MONTH_AWARD_KIND`, and one month slot (no winner, no club: −1) for each month from (`start_month`,
/// `start_year`) up to (`end_month`, `end_year`), at most twelve: the slot count (`MONTH_AWARD_SLOT_COUNT`) goes up
/// with each slot; month 11 is followed by month 0 of the next year. The first slot is always written; the end month
/// is compared signed, the years unsigned (16 bits). With a save file the slots are then read from it
/// (`MONTH_AWARD::load_from_disk`). Returns `this`.
/// Original: `MONTH_AWARD::MONTH_AWARD` (0x766e10, month_award.cpp, thiscall, ret 0x30).
pub(crate) unsafe extern "thiscall" fn construct_month_award(this: usize, nation: u32, index: u32, staff_comp: u32, national_reputation: u32, world_reputation: u32, club_comp: u32, start_month: u32, start_year: u32, end_month: u32, end_year: u32, file: usize, for_players: u32) -> usize {
    award_construct_base(this, [nation, index, staff_comp, national_reputation, world_reputation, club_comp, file as u32, for_players]);
    covered!("sx04b_2 month award built");
    write_u32(this, MONTH_AWARD_VTABLE);
    write_u8(this + award_base::TYPE, MONTH_AWARD_KIND);
    write_u8(this + MONTH_AWARD_SLOT_COUNT, 0);
    let (mut month, mut year) = (start_month as u8 as i8, start_year as u16);
    let end_year = end_year as u16;
    loop {
        let slot = this.wrapping_add((read_i8(this + MONTH_AWARD_SLOT_COUNT) as isize as usize).wrapping_mul(MONTH_AWARD_SLOT_SIZE));
        write_u8(slot + month_award_fields::SLOT_MONTH, month as u8);
        write_u16(slot + month_award_fields::SLOT_YEAR, year);
        write_u32(slot + month_award_fields::SLOT_WINNER_ID, u32::MAX);
        write_u32(slot + month_award_fields::SLOT_CLUB_ID, u32::MAX);
        let count = read_u8(this + MONTH_AWARD_SLOT_COUNT).wrapping_add(1);
        month = month.wrapping_add(1);
        if month > 11 { // sx04b_2: December is followed by January
            month = 0;
            year = year.wrapping_add(1);
        }
        write_u8(this + MONTH_AWARD_SLOT_COUNT, count);
        if (year == end_year && month > end_month as u8 as i8) || year > end_year || count as i8 >= 12 {
            break;
        }
    }
    if file != 0 {
        covered!("sx04b_2 month award loaded");
        let load: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_MONTH_AWARD_LOAD_FROM_DISK);
        load(this, file);
    }
    this
}

/// Comparer for `construct_month_award`: side by side over the 0xa2 bytes of the award (base, slots and count); with a
/// save file (the load allocates) only the original.
pub(crate) unsafe extern "thiscall" fn compare_construct_month_award(this: usize, nation: u32, index: u32, staff_comp: u32, national_reputation: u32, world_reputation: u32, club_comp: u32, start_month: u32, start_year: u32, end_month: u32, end_year: u32, file: usize, for_players: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize, u32) -> usize>(IDX_CONSTRUCT_MONTH_AWARD);
    let original = || o(this, nation, index, staff_comp, national_reputation, world_reputation, club_comp, start_month, start_year, end_month, end_year, file, for_players);
    if file != 0 {
        return original();
    }
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&[(this, MONTH_AWARD_SLOT_COUNT + 1)], original, || {
        construct_month_award(this, nation, index, staff_comp, national_reputation, world_reputation, club_comp, start_month, start_year, end_month, end_year, file, for_players)
    });
    SCOREBOARD.record(equal && r == n, || format!("construct_month_award(from {}/{} to {}/{}) original={:x?} rust={:x?}", start_month as u8, start_year as u16, end_month as u8, end_year as u16, mo, mr));
    r
}

/// Posts the news of the winner of one month slot of a month award.
///
/// The news item (type `MONTH_AWARD_NEWS_TYPE`, local) carries: 0 the award's nation id (`award_base::NATION_ID`)
/// and 1 its index in the nation's awards (`award_base::INDEX`), which `AWARD_MANAGER::write_award_news` uses to find
/// the award again; 2 the winner (`month_award_fields::SLOT_WINNER_ID`) and 3..5 the `names::ID` of his first, second and common names (read without
/// a null check, as the original does); 6 the club id (`SLOT_CLUB_ID`); 7 the month (signed byte) and 8 the year
/// (unsigned word) of the slot; 9 `MONTH_AWARD_NEWS_VERSION`; 10 a draw `utils_random(3)` that picks the text
/// template; 11 whether the winner has no player sheet; with a sheet, 12 his position mask
/// (`exe::formation_get_position_mask`) and 13 his strengths (`exe::staff_get_player_strength_flags` with the slot's
/// club); 14 whether he is a player-manager (`job_for_club()`). Then the receivers: with a transfer manager
/// (`G_TRANSFER_MANAGER`) and a player sheet, the clubs interested in the player
/// (`exe::transfer_manager_collect_player_news_recipients`, kind 0x48, up to 30). The item goes (`news_post`) to the
/// managers of the award's league (`award_base::CLUB_COMP`), or, without one, to those of the staff competition's
/// nation (`award_base::STAFF_COMP`, `STAFF_COMP_NATION`); then to each interested club not playing in the award's
/// league (`club::DIVISION`).
/// Defect of the original: with a player sheet and no club (slot club id −1) the strengths are read with the record
/// before the club table (`G_CLUB_TABLE` − `club::SIZE`); the copy does the same.
/// Original: `MONTH_AWARD::post_winner_news` (0x767330, month_award.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn month_award_post_winner_news(this: usize, slot: u32) {
    month_award_winner_news(this, slot, &mut |item, to| news_post(item, to));
}

/// Comparer for `month_award_post_winner_news`: the news is posted once, so the original runs and the copy is
/// rehearsed from the same random generator (`rehearse_news`: it fills the item and draws without posting); the
/// rehearsal's draws must be the start of the original's.
pub(crate) unsafe extern "thiscall" fn compare_month_award_post_winner_news(this: usize, slot: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_MONTH_AWARD_POST_WINNER_NEWS);
    let mut planned = Vec::new();
    let (same, from_original, from_copy) = rehearse_news(|| o(this, slot), || month_award_winner_news(this, slot, &mut |_, to| planned.push(to)));
    SCOREBOARD.record(same, || format!("month_award_post_winner_news(slot {}) draws original={:?} rust={:?}, rust posts {:?}", slot as u8, from_original, from_copy, planned));
}

/// Writes the headline (`body` 0) or the body of a month award's winner news into `out` (the slots are those of
/// `month_award_post_winner_news`).
///
/// The winner's name is formatted twice from the three name ids (`exe::database_format_name_from_ids`, 0xc8 bytes):
/// long (style 1) and short (style 5). When the winner has a player sheet and the item carries
/// `MONTH_AWARD_NEWS_VERSION`, his position name (`exe::write_position_name`) is written first, with its gender.
/// The headline: by (winner / (draw + 1)) % 6, "<player> takes award", "<player> named as <award>", "Award for
/// <player>", "<player> scoops prize", "<player> wins <month> award", or (5, or a negative remainder) "<award> award
/// decided" (the award by the short name of the staff competition, +5). The body: without a club "<player> has won the
/// <award> award for the month of <month>"; with a club, when the winner has a player sheet, the item carries the mark
/// and the draw is not 0, the player's description (`exe::write_player_description`, replaced by "player-boss" for a
/// player-manager) goes in "<club> <description> <player> is the winner of <month> <award> award" (draw 1) or "The
/// winner of <month> <award> award is <club> <description> <player>" (draw 2); otherwise "<club> <player> has won the
/// <award> award for <month>". The names, the genders and the month (`exe::date_get_month_name`) go in the order the
/// original pushes them. Then the links: the club (short name, 0x7d5), the winner's long and short names (0x7d7) when
/// his names still match the item's ids (`exe::database_staff_matches_names`), and the award's long and short names
/// (0x7d4). Returns the last link's result.
/// Defect of the original: the description's gender goes as a whole dword of which only the low byte was written (the
/// rest is stack leftover); the description reads the byte only.
/// Original: `MONTH_AWARD::write_news` (0x767580, month_award.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn month_award_write_news(this: usize, news: usize, out: usize, body: u32) -> i32 {
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let winner = slot(2) as i32;
    let ids = [slot(3), slot(4), slot(5)];
    let club_id = slot(6) as i32;
    let month = slot(7) as u8 as u32;
    let reworded = slot(9) != MONTH_AWARD_NEWS_VERSION;
    let draw = slot(0xa) as u8 as i8;
    let in_the_player = slot(0xb);
    let positions = slot(0xc);
    let strengths = slot(0xd);
    let player_boss = slot(0xe);
    let mut description = [0u8; 0xc8];
    let desc = description.as_mut_ptr() as usize;
    let mut gender: u8 = 0;
    if in_the_player == 0 && !reworded {
        gender = exe::write_position_name(positions, desc, 0xc8, 1);
    }
    let mut long_name = [0u8; 0xc8];
    let mut short_name = [0u8; 0xc8];
    exe::database_format_name_from_ids(ids[0], ids[1], ids[2], long_name.as_mut_ptr(), 0xc8, 1, winner as u32);
    exe::database_format_name_from_ids(ids[0], ids[1], ids[2], short_name.as_mut_ptr(), 0xc8, 5, winner as u32);
    let (long, short) = (long_name.as_ptr() as usize, short_name.as_ptr() as usize);
    let comp = read_usize(this + award_base::STAFF_COMP); // a TStaffComp: its names as in `competition`
    let fmt = news_text_format();
    let buf = NEWS_TEXT_BUFFER;
    if body != 0 {
        covered!("sx04b_1 body");
        let comp_gender = read_i8(comp + competition::GENDER_NAME) as i32 as u32;
        let comp_name = comp + competition::NAME;
        if club_id == -1 {
            let month_name = exe::date_get_month_name(month);
            // "<player> has won the <award> award for the month of <month>."
            fmt(buf, 0xa5ad8c, 0x64, comp_gender, comp_gender, 0x64, 0x64, long, comp_name, month_name);
        } else if in_the_player == 0 && draw != 0 && !reworded {
            covered!("sx04b_1 body with the description");
            let club = club_at(club_id as isize);
            gender = exe::write_player_description(winner, positions, strengths, desc, 0xc8, gender as u32, 1);
            if player_boss != 0 {
                fmt(desc, 0xa5b0f8); // "player-boss"
            }
            let desc_gender = gender as i8 as i32 as u32;
            let club_gender = (club.short_gender_name() as i8) as i32 as u32;
            let club_name = club.addr() + club::SHORT_NAME;
            let month_name = exe::date_get_month_name(month);
            if draw == 1 { // sx04b_1: the club-first template
                // "<club> <description> <player> is the winner of <month> <award> award."
                fmt(buf, 0xa5b004, club_gender, club_gender, desc_gender, desc_gender, 0x64, 0x64, comp_gender, comp_gender, club_name, desc, long, month_name, comp_name);
            } else {
                // "The winner of <month> <award> award is <club> <description> <player>."
                fmt(buf, 0xa5af10, 0x64, 0x64, comp_gender, comp_gender, club_gender, club_gender, desc_gender, desc_gender, month_name, comp_name, club_name, desc, long);
            }
        } else {
            let club = club_at(club_id as isize);
            let club_gender = (club.short_gender_name() as i8) as i32 as u32;
            let month_name = exe::date_get_month_name(month);
            // "<club> <player> has won the <award> award for <month>."
            fmt(buf, 0xa5ae44, club_gender, club_gender, 0x64, comp_gender, comp_gender, 0x64, 0x64, club.addr() + club::SHORT_NAME, long, comp_name, month_name);
        }
    } else {
        covered!("sx04b_1 headline");
        let award_gender = read_i8(comp + competition::SHORT_GENDER_NAME) as i32 as u32;
        let award = comp + competition::SHORT_NAME;
        match (winner / (draw as i32 + 1)) % 6 {
            0 => fmt(buf, 0xa5ad60, short), // "<player> takes award"
            1 => fmt(buf, 0xa5ad04, award_gender, award_gender, short, award), // "<player> named as <award>"
            2 => fmt(buf, 0xa5acd8, short), // "Award for <player>"
            3 => fmt(buf, 0xa5acac, short), // "<player> scoops prize"
            4 => fmt(buf, 0xa5ac5c, 0x64, 0x64, short, exe::date_get_month_name(month)), // "<player> wins <month> award"
            _ => fmt(buf, 0xa5ac18, award_gender, award_gender, award), // "<award> award decided"
        };
    }
    exe::text_set_string(out, buf as *const u8);
    if club_id != -1 {
        let club = club_at(club_id as isize);
        exe::text_add_link(out, (club.addr() + club::SHORT_NAME) as *const u8, 0x7d5, club);
    }
    let person = staff_at(winner as isize);
    if exe::database_staff_matches_names(person, ids[0], ids[1], ids[2], 0usize) != 0 {
        exe::text_add_link(out, long as *const u8, 0x7d7, person);
        exe::text_add_link(out, short as *const u8, 0x7d7, person);
    }
    exe::text_add_link(out, (comp + competition::NAME) as *const u8, 0x7d4, comp);
    exe::text_add_link(out, (comp + competition::SHORT_NAME) as *const u8, 0x7d4, comp)
}

/// Comparer for `month_award_write_news`: side by side into two Texts (`compare_text_writer`: the text, the links and
/// the random generator).
pub(crate) unsafe extern "thiscall" fn compare_month_award_write_news(this: usize, news: usize, out: usize, body: u32) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32) -> i32>(IDX_MONTH_AWARD_WRITE_NEWS);
    let mut r = 0;
    compare_text_writer(out, || r = o(this, news, out, body), |t| {
        month_award_write_news(this, news, t, body);
    }, || format!("month_award_write_news(winner {}, body {})", exe::newsitem_get_slot(news, 2) as i32, body));
    r
}

/// The winner news of `month_award_post_winner_news`, its items going to `sink` (the game's `news_post`, or a
/// comparer's rehearsal).
pub(crate) unsafe fn month_award_winner_news(this: usize, slot: u32, sink: NewsSink) {
    let v = this.wrapping_add((slot as u8 as i8 as isize as usize).wrapping_mul(11));
    let winner = read_i32(v + month_award_fields::SLOT_WINNER_ID);
    let person = staff_at(winner as isize);
    covered!("sx04b_1 winner news");
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let item = local.as_mut_ptr() as usize;
    exe::new_newsitem(item, MONTH_AWARD_NEWS_TYPE, 0);
    exe::newsitem_set_slot(item, 0, read_u32(this + award_base::NATION_ID));
    exe::newsitem_set_slot(item, 1, read_u32(this + award_base::INDEX));
    exe::newsitem_set_slot(item, 2, winner as u32);
    exe::newsitem_set_slot(item, 3, read_u32(person.first_name() + names::ID));
    exe::newsitem_set_slot(item, 4, read_u32(person.second_name() + names::ID));
    exe::newsitem_set_slot(item, 5, read_u32(person.common_name() + names::ID));
    exe::newsitem_set_slot(item, 6, read_u32(v + month_award_fields::SLOT_CLUB_ID));
    exe::newsitem_set_slot(item, 7, read_i8_as_i32(v + month_award_fields::SLOT_MONTH) as u32);
    exe::newsitem_set_slot(item, 8, read_u16(v + month_award_fields::SLOT_YEAR) as u32);
    exe::newsitem_set_slot(item, 9, MONTH_AWARD_NEWS_VERSION);
    exe::newsitem_set_slot(item, 0xa, exe::utils_random(3) as u32); // sx04b_1: the template draw
    let sheet = person.player();
    exe::newsitem_set_slot(item, 0xb, sheet.is_null() as u32);
    if !sheet.is_null() {
        covered!("sx04b_1 winner with a player sheet");
        exe::newsitem_set_slot(item, 0xc, exe::formation_get_position_mask(sheet) as i16 as i32 as u32);
        let club = club_at(read_i32(v + month_award_fields::SLOT_CLUB_ID) as isize);
        exe::newsitem_set_slot(item, 0xd, exe::staff_get_player_strength_flags(person, club));
    }
    exe::newsitem_set_slot(item, 0xe, (person.job_for_club() == job_kind::PLAYER_MANAGER) as u32);
    let mut clubs = [0usize; 30];
    let mut n: i8 = 0;
    let tm = read_usize(G_TRANSFER_MANAGER);
    if tm != 0 && !person.player().is_null() {
        n = exe::transfer_manager_collect_player_news_recipients(tm, person, 0x48, clubs.as_mut_ptr(), 0x1e) as i8;
    }
    let league = read_usize(this + award_base::CLUB_COMP);
    let comp = read_usize(this + award_base::STAFF_COMP);
    if league != 0 {
        sink(item, NewsPost::Division(league));
    } else if comp != 0 {
        sink(item, NewsPost::Nation(read_usize(comp + STAFF_COMP_NATION)));
    }
    for &club in clubs.iter().take(n.max(0) as usize) {
        if read_usize(club + club::DIVISION) != read_usize(this + award_base::CLUB_COMP) { // sx04b_1: clubs of other leagues
            sink(item, NewsPost::Club(club));
        }
    }
}

/// `MONTH_AWARD`'s vtable.
pub(crate) const MONTH_AWARD_VTABLE: u32 = 0x96da00;

/// The award type byte (`award_base::TYPE`) of a month award.
pub(crate) const MONTH_AWARD_KIND: u8 = 2;

/// `MONTH_AWARD::load_from_disk(file)` (0x766f50, month_award.cpp, thiscall, ret 4).
pub(crate) const FN_MONTH_AWARD_LOAD_FROM_DISK: usize = 0x766f50;

/// The news type of a month award's winner (category 2, written by `AWARD_MANAGER::write_award_news`).
pub(crate) const MONTH_AWARD_NEWS_TYPE: u32 = 0x1b;

/// The version mark the news item carries in slot 9; the text writer rewords nothing while it is there.
pub(crate) const MONTH_AWARD_NEWS_VERSION: u32 = 0x420e9;

/// A month award's staff competition record (`TStaffComp`, award_fields +5): +4 name, +0x37 its gender, +0x38 short
/// name, +0x52 its gender, +0x57 nation.
pub(crate) const STAFF_COMP_NATION: usize = 0x57;
