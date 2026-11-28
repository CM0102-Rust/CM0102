// --- sx09c_1: the Fine manager: its daily pass, its construction and its release (Fine.cpp) ---

/// `Fine::load` (0x5a5180, Fine.cpp; thiscall(fine, save), ret 4, the answer in al): reads fine.dat (sx09c_2).
const FN_FINE_LOAD: usize = 0x5a5180;

/// Drops the first expired fine and, when a coach feedback is due, posts it.
///
/// `Fine::daily_update()`: the fine list (the 14-byte records at `[this]`) from its head: the first of the first
/// `count` records whose date (+6) today is past (`exe::date_operator_3e` on `G_CURRENT_DATE`) leaves the list
/// (`exe::llist_remove`); a record walk that runs out of nodes ends the whole pass. Then the coach feedback list (the
/// 17-byte records at `[this + 4]`) from its head: the first of the first `count` records whose date (+9) today is
/// past: when the person of +0 (the club's assistant manager when the fine was recorded) joined his club more than 30
/// days ago (`exe::date_days_since` on `date_joined_club()`) and his injury status (`exe::injury_get_player_injury_status`
/// on `G_INJURY`) is not 2, `post_coach_fine_feedback_news(+0, the player of +4, the outcome byte +8)`; then the
/// feedback list removes the node whose data equals, byte for byte, the original's stale stack slot (see below), and
/// the pass ends.
///
/// Defect of the original: the removal of the feedback record passes a stack slot that still holds the last fine the
/// first walk looked at (freed already when it was the expired one), or the Fine object itself when the fine list was
/// empty, instead of the feedback record. `LLIST::remove` compares node data byte for byte, so the feedback record
/// stays and its news is posted again every day while the conditions hold. The copy passes the same slot.
/// Original: `Fine::daily_update` (0x5a5820, Fine.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn fine_daily_update(this: usize) {
    let fines = Llist(read_usize(this));
    fines.set_cursor(fines.head());
    let mut last_fine = this; // the original's stack slot: `push ecx` (this), then each fine the walk reads
    if fines.count() > 0 {
        let mut k = 0i32;
        loop {
            let fine = exe::llist_get_next(fines);
            last_fine = fine;
            if fine == 0 {
                return;
            }
            if exe::date_operator_3e(G_CURRENT_DATE, read_u32(fine + 6), read_u32(fine + 0xa)) != 0 { // +6: the fine's end date
                covered!("sx09c_1 expired fine dropped");
                exe::llist_remove(fines, fine);
                break;
            }
            k += 1;
            if k >= fines.count() as i32 {
                break;
            }
        }
    }
    let feedback = Llist(read_usize(this + 4));
    feedback.set_cursor(feedback.head());
    if feedback.count() <= 0 {
        return;
    }
    let mut k = 0i32;
    loop {
        let entry = exe::llist_get_next(feedback);
        if entry == 0 {
            return;
        }
        if exe::date_operator_3e(G_CURRENT_DATE, read_u32(entry + 9), read_u32(entry + 0xd)) != 0 { // +9: the report date
            let first = staff_at(read_i32(entry) as isize); // +0: the club's assistant manager at the fine
            let player = staff_at(read_i32(entry + 4) as isize); // +4: the fined player
            if exe::date_days_since(G_CURRENT_DATE, first.date_joined_club() as u32, first.date_joined_club_leap_year() as u32) > 0x1e
                && exe::injury_get_player_injury_status(read_usize(G_INJURY), first) != 2 { // sx09c_1: the report is posted
                covered!("sx09c_1 coach feedback posted");
                post_coach_fine_feedback_news(first.addr(), player.addr(), read_u8(entry + 8) as u32);
            }
            exe::llist_remove(feedback, last_fine); // the original's defect: the stale slot, not `entry`
            return;
        }
        k += 1;
        if k >= feedback.count() as i32 {
            return;
        }
    }
}

/// Comparer for `fine_daily_update`: it frees list nodes and posts news, so only the original runs; the plan is
/// checked afterwards: the fine list loses one node exactly when the walk finds an expired fine (counted before), and
/// the feedback list keeps its nodes when none is due.
pub(crate) unsafe extern "thiscall" fn compare_fine_daily_update(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_FINE_DAILY_UPDATE);
    let due = |list: Llist, date: usize| {
        let mut node = list.head();
        let mut k = 0i32;
        while node != 0 && k < list.count() as i32 {
            let d = read_usize(node + llist_node::DATA);
            if exe::date_operator_3e(G_CURRENT_DATE, read_u32(d + date), read_u32(d + date + 4)) != 0 {
                return true;
            }
            node = read_usize(node + llist_node::NEXT);
            k += 1;
        }
        false
    };
    let (fines, feedback) = (Llist(read_usize(this)), Llist(read_usize(this + 4)));
    let (n1, n2) = (fines.count(), feedback.count());
    let (expired, report) = (due(fines, 6), due(feedback, 9));
    o(this);
    let ok = fines.count() == n1 - expired as i16 && (report || feedback.count() == n2);
    SCOREBOARD.record(ok, || format!("fine_daily_update: fines {} -> {} (expired {}), feedback {} -> {} (due {})",
        n1, fines.count(), expired, n2, feedback.count(), report));
}

/// Builds the Fine manager: the fine list and the coach feedback list, from the save's fine.dat (`Fine::load`) or new
/// and empty, and the per-club fine count table.
///
/// `Fine::Fine(save)`: both list pointers null; with a save, `Fine::load` (`FN_FINE_LOAD`); when it answers, nothing else (it built the
/// lists and the table). Otherwise (no save, or the load failed): with a save, the lists the load may have left are
/// destroyed (`LLIST::~LLIST` and `operator delete`, the pointers kept), and in both cases two new empty lists
/// (`operator new(0xe)` and `LLIST::LLIST`, null when the allocation fails) and the fine count table: one zeroed byte
/// per club (`calloc(G_CLUB_COUNT, 1)`) in +8. Returns the object.
/// Original: `Fine::Fine` (0x5a4fc0, Fine.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn new_fine(this: usize, save: usize) -> usize {
    write_usize(this, 0);
    write_usize(this + 4, 0);
    if save != 0 {
        let load: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_FINE_LOAD);
        if load(this, save) != 0 {
            return this;
        }
        covered!("sx09c_1 fine.dat not loaded");
        for field in [0usize, 4] {
            let list = read_usize(this + field);
            if list != 0 {
                exe::drop_llist(list);
                exe::crt_operator_delete(list);
            }
        }
    }
    write_usize(this, club_history_new_list()); // the fine list
    write_usize(this + 4, club_history_new_list()); // the coach feedback list
    write_usize(this + 8, exe::crt_calloc(read_u32(G_CLUB_COUNT), 1)); // sx09c_1: the per-club fine counts
    this
}

/// Comparer for `new_fine`: it allocates (and, with a save, reads the save item), so it cannot run twice; only the
/// original runs. The copy is proven by the new career (Rust mode).
pub(crate) unsafe extern "thiscall" fn compare_new_fine(this: usize, save: usize) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_NEW_FINE)(this, save)
}

/// Frees the Fine manager's data: each list present (`LLIST::~LLIST` and `operator delete`) and the fine count table
/// (`free`), each pointer set to null.
/// Original: `Fine::free_data` (0x5a5120, Fine.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn fine_free_data(this: usize) {
    for field in [0usize, 4] {
        let list = read_usize(this + field);
        if list != 0 {
            exe::drop_llist(list);
            exe::crt_operator_delete(list);
            write_usize(this + field, 0);
        }
    }
    let counts = read_usize(this + 8);
    if counts != 0 {
        exe::crt_free(counts);
        write_usize(this + 8, 0);
    }
}

/// Comparer for `fine_free_data`: it frees, so it cannot run twice; only the original runs, and the three pointers must
/// end null.
pub(crate) unsafe extern "thiscall" fn compare_fine_free_data(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_FINE_FREE_DATA)(this);
    SCOREBOARD.record(read_usize(this) == 0 && read_usize(this + 4) == 0 && read_usize(this + 8) == 0,
        || format!("fine_free_data({:#x}): pointers left {:#x} {:#x} {:#x}", this, read_usize(this), read_usize(this + 4), read_usize(this + 8)));
}

// --- sx09c_5: the discipline object's construction and release, and the ban for assaulting a referee (discipline.cpp) ---

/// `Discipline::load` (0x555860, discipline.cpp; thiscall(discipline, save), ret 4, eax): reads discipline.dat (sx09c_4).
const FN_DISCIPLINE_LOAD: usize = 0x555860;
/// `RulingBody::~RulingBody` (0x7e7860, ruling_body.cpp; thiscall, ret).
const FN_RULING_BODY_DESTRUCTOR: usize = 0x7e7860;
/// `Discipline::post_ban_news` (0x557280, discipline.cpp; thiscall(discipline, person, ban, fixture), ret 0xc; sx09b_1).
const FN_DISCIPLINE_POST_BAN_NEWS: usize = 0x557280;

/// The CD search of a routine that requires the game CD: each CD drive (`GetLogicalDriveStringsA` of 200 bytes,
/// `GetDriveTypeA` = 5) is looked at for "<root>cm0102.exe" (`file_find_first`, closed with `file_find_close` when not
/// found) until one has it, with the "Insert CD" box (langlib texts 0x987df4 and 0x987dc4, OK) in between, forever. A
/// no-CD exe patched in the calling routine (as `cm0102-1080`: the drive type's `jne` made `nop`, the found test
/// `test eax, eax` made `xor`) is honoured by reading those two bytes from the running exe.
pub(crate) unsafe fn require_game_cd(drive_type_jump: *const u8, found_test: *const u8) {
    let drives: unsafe extern "system" fn(u32, *mut u8) -> u32 = core::mem::transmute(read_usize(0x9670f8));
    let kind: unsafe extern "system" fn(usize) -> u32 = core::mem::transmute(read_usize(0x967100));
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let any_drive = drive_type_jump.read() == 0x90;
    let always_found = found_test.read() != 0x85;
    loop {
        // simplification: the buffer starts zeroed (the original's is left over on the stack; only the drive list
        // GetLogicalDriveStringsA writes is read)
        let mut list = [0u8; 0xc8];
        if drives(0xc8, list.as_mut_ptr()) != 0 && list[0] != 0 {
            let mut i = 0usize;
            loop {
                let root = list.as_ptr() as usize + i;
                if kind(root) == 5 || any_drive {
                    let (mut name, mut found) = ([0u8; 0x104], [0u8; 0x140]);
                    format(name.as_mut_ptr() as usize, 0x987e00, root, 0x63u32, 0x6du32, 0x30u32, 0x31u32, 0x30u32, 0x32u32, 0x2eu32, 0x65u32, 0x78u32, 0x65u32);
                    if exe::file_find_first(name.as_ptr() as usize, found.as_mut_ptr() as usize, 0) == 0 || always_found {
                        return;
                    }
                    exe::file_find_close(found.as_mut_ptr() as usize);
                }
                // simplification: the original steps with a signed byte (only past 32 drives would it differ)
                i += 4;
                if i >= 0xc8 || list[i] == 0 {
                    break;
                }
            }
        }
        covered!("sx09c_5 insert CD");
        let (mut title, mut message) = ([0u8; 0x100], [0u8; 0x100]);
        text(title.as_mut_ptr() as usize, 0x987df4); // "Insert CD"
        text(message.as_mut_ptr() as usize, 0x987dc4); // "Please insert the CM01/02 CD and click on OK"
        exe::gui_show_message_box(title.as_ptr() as usize, message.as_ptr() as usize, 1);
    }
}

/// Builds the discipline object: the held-ban list, the CD check, the empty ban array and nation index, the ban
/// history file, and the per-person records and rule sets, from the save or new.
///
/// `Discipline::Discipline(save)`: `discipline_fields::HELD_BANS` constructed (`LLIST::LLIST`); the game CD required
/// (`require_game_cd`, honouring the 1080 no-CD patch at 0x55550f/0x555551); `BANS`, `BAN_CAPACITY`, `BAN_COUNT`,
/// `NATION_INDEX`, `NATION_INDEX_CAPACITY` and `NATION_INDEX_COUNT` zeroed; the ban history file ("CM3_TEMP\ban_history.tmp",
/// 20-byte records; `club_history_new_file_list`) in `BAN_HISTORY`; failing, the discipline.cpp "Error" box (line 0x7c)
/// and nothing else. With a save, `Discipline::load` (`FN_DISCIPLINE_LOAD`); when it answers, nothing else. Otherwise
/// the per-person records (`malloc` of `G_STAFF_COUNT` × 29 bytes in `PERSON_RECORDS`: every byte 0xff and the held-ban
/// count `discipline_record::HELD_BAN_COUNT` 0; failing, the box of line 0x1603) and the new rule sets
/// (`discipline_create_rule_sets` with 0). Returns the object.
/// Original: `Discipline::Discipline` (0x5554a0, discipline.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn new_discipline(this: usize, save: usize) -> usize {
    use discipline_fields as d;
    exe::new_llist(this + d::HELD_BANS);
    require_game_cd(0x55550f as *const u8, 0x555551 as *const u8);
    write_usize(this + d::BANS, 0);
    write_i16(this + d::BAN_CAPACITY, 0);
    write_i16(this + d::BAN_COUNT, 0);
    write_usize(this + d::NATION_INDEX, 0);
    write_i16(this + d::NATION_INDEX_CAPACITY, 0);
    write_i16(this + d::NATION_INDEX_COUNT, 0);
    let history = club_history_new_file_list(0x9b8f48, 0x14); // "ban_history.tmp"
    write_usize(this + d::BAN_HISTORY, history);
    if history == 0 {
        show_error_box(0x9b8f18, 0x7c); // "E:\dev\CM3\cm3 00-01\cm3\code\discipline.cpp"
        return this;
    }
    if save != 0 {
        let load: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_DISCIPLINE_LOAD);
        if load(this, save) != 0 {
            return this;
        }
    }
    covered!("sx09c_5 new discipline records");
    let n = read_u32(G_STAFF_COUNT);
    let records = exe::crt_malloc(n.wrapping_mul(0x1d));
    write_usize(this + d::PERSON_RECORDS, records);
    if records == 0 {
        show_error_box(0x9b8f18, 0x1603);
    } else {
        core::ptr::write_bytes(records as *mut u8, 0xff, n.wrapping_mul(0x1d) as usize);
        for p in 0..read_i32(G_STAFF_COUNT).max(0) as usize {
            write_u8(records + p * 0x1d + discipline_record::HELD_BAN_COUNT, 0); // sx09c_5: no held bans
        }
    }
    discipline_create_rule_sets(this, 0);
    this
}

/// Comparer for `new_discipline`: it allocates, opens the history file and (with a save) reads the save item, so it
/// cannot run twice; only the original runs. The copy is proven by the new career (Rust mode).
pub(crate) unsafe extern "thiscall" fn compare_new_discipline(this: usize, save: usize) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_NEW_DISCIPLINE)(this, save)
}

/// Releases the discipline object: frees the ban array, the nation index and the per-person records (`free`, the
/// pointers kept); each of the 43 rule sets present (`RulingBody::~RulingBody` and `operator delete`); the rule set
/// table (`free`); the ban history file (`FILE_LLIST::~FILE_LLIST` and `operator delete`); and the held-ban list
/// (`LLIST::~LLIST`).
/// Original: `Discipline::~Discipline` (0x555790, discipline.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn drop_discipline(this: usize) {
    use discipline_fields as d;
    for field in [d::BANS, d::NATION_INDEX, d::PERSON_RECORDS] {
        if read_usize(this + field) != 0 {
            exe::crt_free(read_usize(this + field));
        }
    }
    let destroy: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_RULING_BODY_DESTRUCTOR);
    for k in 0..0x2busize {
        let body = read_usize(read_usize(this + d::RULING_BODIES) + 4 * k);
        if body != 0 {
            destroy(body);
            exe::crt_operator_delete(body);
        }
    }
    if read_usize(this + d::RULING_BODIES) != 0 {
        exe::crt_free(read_usize(this + d::RULING_BODIES));
    }
    let history = read_usize(this + d::BAN_HISTORY);
    if history != 0 {
        let destroy_file: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FILE_LLIST_DESTRUCT);
        destroy_file(history);
        exe::crt_operator_delete(history);
    }
    exe::drop_llist(this + d::HELD_BANS);
}

/// Comparer for `drop_discipline`: it frees, so it cannot run twice; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_drop_discipline(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_DROP_DISCIPLINE)(this)
}

/// Bans a player who assaulted the referee: his current bans are dropped and a world-wide misconduct ban of 90 days (180
/// when his club's season ends within 80 days) starts today; the ban news follows, and the chairman of a big club may
/// complain to the FA.
///
/// `Discipline::handle_referee_assault(row, fixture, _)`: nothing (0) unless one of the row's card offences
/// (`match_record_row::FIRST_CARD_OFFENCE`, `SECOND_CARD_OFFENCE`) is 12. Then, for each of the person's 4 slots in use,
/// the rule set's entry (`exe::rulingbody_find_or_add_discipline_entry` with 0) with a ban (`BAN_INDEX` above −1)
/// releases it (`exe::discipline_release_ban`). The new ban (a local `BanRecord`): by days (matches 0xff), 90 days, or
/// 180 when `exe::date_days_since` from the end date of the club's season (`exe::key_nation_club_in_season` on
/// `club_job()`) to today is below 80; served 0; today's date; the rules {−1, −1, −1, FIFA 0}; reason 3 (misconduct),
/// amount 0; the flags bit 1 set and bit 2 when the row's byte +0x48 is 0. The FIFA rule set's entry is made
/// (`exe::rulingbody_find_or_add_discipline_entry` with 1), the ban added (`add_ban` on `G_DISCIPLINE`) and its news
/// posted (`Discipline::post_ban_news` with the fixture). Then, when the player's club has a reputation above 5250 and
/// a manager, is not human (`exe::human_manager_is_human_club`), is not Argentinian (`club_nation_is_not_argentina`),
/// its club state byte +4 (`G_CLUB_STATE_TABLE`, 6 bytes per club) is below −3 and the board confidence
/// (`board_record::CONFIDENCE`) is below 4250: random(20) = 0 queues the manager's complaint story to the FA
/// (`queue_manager_fa_complaint_story` on `G_MEDIA`) of kind 1 with the ban's days word and today's day as its fifth
/// value; otherwise, when the state byte plus the manager's pressure is below 10 and random(professionalism +
/// pressure) = 0, kind 3; otherwise, with pressure and ambition both above 10 and random(20) = 0, kind 2. Returns 1.
///
/// The flags byte of the local ban is not initialised by the original: bits 3..7 are left over from the stack and bit 0
/// is set by `add_ban`; only bits 0..2 are copied into the ban array (`BanRecord::operator=`) and the news reads bit 1.
/// The copy starts the byte at 0.
/// Original: `Discipline::handle_referee_assault` (0x55e300, discipline.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn handle_referee_assault(this: usize, row: usize, fixture: u32, _unused: u32) -> u32 {
    let id = read_u32(row + match_record_row::STAFF_ID);
    let person = staff_at(id as isize);
    if read_u8(row + match_record_row::FIRST_CARD_OFFENCE) != 0xc && read_u8(row + match_record_row::SECOND_CARD_OFFENCE) != 0xc {
        return 0;
    }
    covered!("sx09c_5 referee assault");
    let record = read_usize(this + discipline_fields::PERSON_RECORDS).wrapping_add((id as usize).wrapping_mul(0x1d));
    for k in 0..4usize {
        let rule = read_u8(record + 5 * k + ban_slot::RULE);
        if rule == 0xff {
            continue;
        }
        let body = read_usize(read_usize(this + discipline_fields::RULING_BODIES) + 4 * (rule as i8 as i32 as usize));
        let entry = exe::rulingbody_find_or_add_discipline_entry(body, id, 0);
        if entry != 0 && read_i16(entry + ruling_body_entry::BAN_INDEX) > -1 {
            // the index goes in ax; the high part of eax is that of the entry's address (0x556d00 only reads the word)
            exe::discipline_release_ban(this, id, (entry as u32 & 0xffff_0000) | read_u16(entry + ruling_body_entry::BAN_INDEX) as u32);
        }
    }
    let (mut season_start, mut season_end) = ([0u8; 8], [0u8; 8]);
    exe::date_date_54a110(season_start.as_mut_ptr() as usize);
    exe::date_date_54a110(season_end.as_mut_ptr() as usize);
    let mut ban = [0u8; 0x14];
    let b = ban.as_mut_ptr() as usize;
    exe::date_date_54a110(b + ban_record::START_DATE);
    write_u8(b + ban_record::MATCHES, 0xff);
    write_u16(b + ban_record::DAYS, 0x5a);
    exe::key_nation_club_in_season(person.club_job(), season_start.as_mut_ptr() as usize, season_end.as_mut_ptr() as usize);
    if exe::date_days_since(season_end.as_ptr() as usize, read_u32(G_CURRENT_DATE), read_u32(G_CURRENT_DATE + 4)) < 0x50 {
        covered!("sx09c_5 referee assault ban of 180 days");
        write_u16(b + ban_record::DAYS, 0xb4);
    }
    write_u8(b + ban_record::MATCHES_SERVED, 0);
    let mut scratch = [0u8; 8];
    exe::date_operator_3d(b + ban_record::START_DATE, scratch.as_mut_ptr() as usize, G_CURRENT_DATE);
    let all_rules = (read_u8(row + 0x48) == 0) as u8; // +0x48: the row's last byte
    write_u8(b + ban_record::FLAGS, (read_u8(b + ban_record::FLAGS) | 2) & 0xfb | all_rules << 2);
    write_u8(b + ban_record::REASON, 3); // misconduct
    write_u16(b + ban_record::REASON_AMOUNT, 0);
    for (k, rule) in [0xffu8, 0xff, 0xff, 0].into_iter().enumerate() {
        write_u8(b + ban_record::RULES + k, rule); // sx09c_5: the FIFA rule only
    }
    let fifa = read_usize(read_usize(this + discipline_fields::RULING_BODIES));
    exe::rulingbody_find_or_add_discipline_entry(fifa, id, 1);
    add_ban(read_usize(G_DISCIPLINE), id, b);
    let post: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(FN_DISCIPLINE_POST_BAN_NEWS);
    post(read_usize(G_DISCIPLINE), staff_at(id as isize).addr(), b, fixture);
    let club = person.club_job();
    if club.is_null() || club.reputation() as i16 <= 0x1482 || club.manager().is_null() {
        return 1;
    }
    if exe::human_manager_is_human_club(club, 0, 1, 0, 0) != 0 || club_nation_is_not_argentina(club.addr()) as u8 == 0 {
        return 1;
    }
    let state = read_i8(read_usize(G_CLUB_STATE_TABLE).wrapping_add((read_i32(club.addr()) as usize).wrapping_mul(6)) + 4); // +4: the club state byte
    if state >= -3 || read_i16(club_board_row(read_usize(G_MANAGER_MANAGER), club) + board_record::CONFIDENCE) >= 0x109a {
        return 1;
    }
    let manager = club.manager();
    let (kind, value) = if exe::utils_random(0x14) == 0 {
        (1, read_u32(b + ban_record::DAYS)) // the days word and the start date's day
    } else {
        let pressure = manager.pressure();
        if (state as i32) + (pressure as i32) < 10 && exe::utils_random(manager.professionalism().wrapping_add(pressure) as i32) == 0 {
            (3, u32::MAX)
        } else if pressure <= 10 || manager.ambition() <= 10 || exe::utils_random(0x14) != 0 {
            return 1;
        } else {
            (2, u32::MAX)
        }
    };
    covered!("sx09c_5 complaint to the FA");
    queue_manager_fa_complaint_story(read_usize(G_MEDIA), club.addr(), staff_at(id as isize).addr(), kind, u32::MAX, value, u32::MAX);
    1
}

/// Comparer for `handle_referee_assault`: it releases and adds bans, posts news and draws at random, so only the
/// original runs; afterwards, when the row has the offence 12, the FIFA rule set's entry of the person must hold a ban
/// by days of reason 3 starting today.
pub(crate) unsafe extern "thiscall" fn compare_handle_referee_assault(this: usize, row: usize, fixture: u32, unused: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32>(IDX_HANDLE_REFEREE_ASSAULT)(this, row, fixture, unused);
    let assault = read_u8(row + match_record_row::FIRST_CARD_OFFENCE) == 0xc || read_u8(row + match_record_row::SECOND_CARD_OFFENCE) == 0xc;
    let id = read_u32(row + match_record_row::STAFF_ID);
    let ok = if !assault {
        r == 0
    } else {
        let fifa = read_usize(read_usize(this + discipline_fields::RULING_BODIES));
        let e = ruling_body_entry(fifa, id);
        let index = if e == 0 { -1 } else { read_i16(e + ruling_body_entry::BAN_INDEX) };
        let ban = read_usize(this + discipline_fields::BANS).wrapping_add((index as i32 as usize).wrapping_mul(0x14));
        r == 1 && index > -1 && read_u8(ban + ban_record::MATCHES) == 0xff && read_u8(ban + ban_record::REASON) == 3
            && read_u32(ban + ban_record::START_DATE) == read_u32(G_CURRENT_DATE)
    };
    SCOREBOARD.record(ok, || format!("handle_referee_assault(person {}, assault {}) returned {}", id, assault, r));
    r
}

// --- sx09c_6: the dispute object's construction and release, and filing a complaint to the Player Status Committee (dispute.cpp) ---

/// `DISPUTE::load` (0x569910, dispute.cpp; thiscall(dispute, save), ret 4, al): reads dispute.dat (sx09c_3).
const FN_DISPUTE_LOAD: usize = 0x569910;
/// `dispute_post_news` (0x56a3b0, dispute.cpp; thiscall(dispute, person, club, outcome, a, b), ret 0x14; sx09b_6).
const FN_DISPUTE_POST_COMPLAINT_NEWS: usize = 0x56a3b0;

/// Frees the dispute object's flag table (`free`) and destroys its list (`LLIST::~LLIST` and `operator delete`), each
/// present one, setting the pointers to null.
unsafe fn dispute_free_tables(this: usize) {
    if read_usize(this + 4) != 0 {
        exe::crt_free(read_usize(this + 4));
        write_usize(this + 4, 0);
    }
    let list = read_usize(this);
    if list != 0 {
        exe::drop_llist(list);
        exe::crt_operator_delete(list);
        write_usize(this, 0);
    }
}

/// Builds the dispute object: from the save's dispute.dat (`DISPUTE::load`) or new: an empty per-person flag table and
/// an empty dispute list.
///
/// `DISPUTE::DISPUTE(save)`: the flag table (+4) and the list (+0) null; with a save, `DISPUTE::load`
/// (`FN_DISPUTE_LOAD`); when it answers, nothing else; when it fails, what it left is freed (`dispute_free_tables`).
/// Then (also without a save) the flag table: one zeroed byte per person (`calloc(G_STAFF_COUNT, 1)`), and a new list
/// (`operator new(0xe)` and `LLIST::LLIST`, null when the allocation fails). Returns the object.
/// Original: `DISPUTE::DISPUTE` (0x5697e0, dispute.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn new_dispute(this: usize, save: usize) -> usize {
    write_usize(this + 4, 0);
    write_usize(this, 0);
    if save != 0 {
        let load: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_DISPUTE_LOAD);
        if load(this, save) != 0 {
            return this;
        }
        covered!("sx09c_6 dispute.dat not loaded");
        dispute_free_tables(this);
    }
    write_usize(this + 4, exe::crt_calloc(read_u32(G_STAFF_COUNT), 1)); // sx09c_6: the per-person dispute flags
    write_usize(this, club_history_new_list());
    this
}

/// Comparer for `new_dispute`: it allocates (and, with a save, reads the save item), so it cannot run twice; only the
/// original runs. The copy is proven by the new career (Rust mode).
pub(crate) unsafe extern "thiscall" fn compare_new_dispute(this: usize, save: usize) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_NEW_DISPUTE)(this, save)
}

/// Releases the dispute object: the flag table and the list (`dispute_free_tables`).
/// Original: `DISPUTE::~DISPUTE` (0x5698d0, dispute.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn drop_dispute(this: usize) {
    dispute_free_tables(this);
}

/// Comparer for `drop_dispute`: it frees, so it cannot run twice; only the original runs, and both pointers must end
/// null.
pub(crate) unsafe extern "thiscall" fn compare_drop_dispute(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_DROP_DISPUTE)(this);
    SCOREBOARD.record(read_usize(this) == 0 && read_usize(this + 4) == 0, || format!("drop_dispute({:#x}): pointers left", this));
}

/// Whether `file_complaint` goes ahead for the person: a person with a player record and an id below `G_STAFF_COUNT`,
/// not already in a dispute (his flag byte), not absent without leave (`exe::awol_is_player_awol` on `G_AWOL` with 0)
/// and the FIFA transfer rules in force (`G_FIFA_TRANSFER_RULES_IN_FORCE`).
unsafe fn complaint_goes_ahead(this: usize, person: Staff) -> bool {
    if person.is_null() || person.player().is_null() || read_i32(person.addr()) >= read_i32(G_STAFF_COUNT) {
        return false;
    }
    let flag = read_usize(this + 4).wrapping_add(read_usize(person.addr()));
    read_u8(flag) == 0 && exe::awol_is_player_awol(read_usize(G_AWOL), person, 0) == 0 && read_u8(flag) == 0
        && read_u8(G_FIFA_TRANSFER_RULES_IN_FORCE) != 0
}

/// Starts a player's complaint to the Player Status Committee: his dispute flag is set, his reputations drop, he and
/// his club fall out, the dispute is queued for a hearing 2 to 21 days ahead and the complaint news is posted.
///
/// `DISPUTE::file_complaint(person, club, a, b)` (from the fine 0x5a692b and the media 0x741ebe): nothing unless
/// `complaint_goes_ahead`. The flag byte set to 1. The player record's current, home and world reputations (in that
/// order) each become `_ftol((1.0 − random(21) × 0.01) × reputation)`. With a club (`club_job()`): the player dislikes
/// its manager (`exe::staff_add_disliked_staff` with 1), the manager dislikes him unless the club is human
/// (`exe::human_manager_is_human_club` with 1, 1, 0, 0); when the low byte of `b` is 0, each squad member (50 slots)
/// other than him with random(loyalty) above 5 and random(professionalism) = 0 dislikes him; and the chairman dislikes
/// him. The 17-byte dispute (person id; the id of `club`, read even when it is null; the hearing date: today plus
/// random(20) + 2 days, `exe::date_add_days`; the byte +0x10: the low byte of `b`, or 0 when random(10) = 0 or `b` is 0,
/// random(10) drawn only when it is not) goes to the end of the list. The news (`dispute_post_news` with `club`, or the
/// player's club when it is null, outcome 0, `a` and `b`).
/// Original: `DISPUTE::file_complaint` (0x56a100, dispute.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn file_complaint(this: usize, person: usize, club: usize, a: u32, b: u32) {
    let person = Staff(person);
    if !complaint_goes_ahead(this, person) {
        return;
    }
    covered!("sx09c_6 complaint filed");
    write_u8(read_usize(this + 4).wrapping_add(read_usize(person.addr())), 1);
    let cut = |r: i16| ftol_x87((read_f64(rdata::F64_1) - exe::utils_random(0x15) as f64 * read_f64(rdata::F64_0_01)) * r as f64) as i16;
    let v = cut(person.player().current_reputation()); // sx09c_6: the reputations drop up to 20%
    person.player().set_current_reputation(v);
    let v = cut(person.player().home_reputation());
    person.player().set_home_reputation(v);
    let v = cut(person.player().world_reputation());
    person.player().set_world_reputation(v);
    let keep = b as u8;
    if !person.club_job().is_null() {
        exe::staff_add_disliked_staff(person, person.club_job().manager(), 1);
        if exe::human_manager_is_human_club(person.club_job(), 1, 1, 0, 0) == 0 {
            exe::staff_add_disliked_staff(person.club_job().manager(), person, 1);
        }
        if keep == 0 {
            for k in 0..50usize {
                let mate = Staff(read_usize(person.club_job().addr() + club::SQUAD + 4 * k));
                if !mate.is_null() && mate.addr() != person.addr() && exe::utils_random(mate.loyalty() as i32) > 5
                    && exe::utils_random(mate.professionalism() as i32) == 0 {
                    exe::staff_add_disliked_staff(mate, person, 1);
                }
            }
        }
        exe::staff_add_disliked_staff(person.club_job().chairman(), person, 1);
    }
    let mut record = [0u8; 0x11];
    let r = record.as_mut_ptr() as usize;
    let mut scratch = [0u8; 8];
    exe::date_date_54a110(r + 8);
    write_u32(r, read_u32(person.addr()));
    write_u32(r + 4, read_u32(club)); // the club's id
    exe::date_operator_3d(r + 8, scratch.as_mut_ptr() as usize, G_CURRENT_DATE);
    let days = exe::utils_random(0x14) + 2;
    let mut later = [0u8; 8];
    let hearing = exe::date_add_days(r + 8, later.as_mut_ptr() as usize, days as u32);
    exe::date_operator_3d(r + 8, scratch.as_mut_ptr() as usize, hearing);
    write_u8(r + 0x10, if keep != 0 && exe::utils_random(10) != 0 { keep } else { 0 });
    exe::llist_add_tail(read_usize(this), r, 0x11);
    let news_club = if club == 0 { person.club_job().addr() } else { club };
    let post: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(FN_DISPUTE_POST_COMPLAINT_NEWS);
    post(this, person.addr(), news_club, 0, a, b);
}

/// Comparer for `file_complaint`: it draws at random, changes relations and posts news, so only the original runs;
/// when the complaint goes ahead (checked before), the person's flag must end at 1 and the list must have one more
/// dispute; otherwise neither changes.
pub(crate) unsafe extern "thiscall" fn compare_file_complaint(this: usize, person: usize, club: usize, a: u32, b: u32) {
    let goes = complaint_goes_ahead(this, Staff(person));
    let list = Llist(read_usize(this));
    let before = list.count();
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32)>(IDX_FILE_COMPLAINT)(this, person, club, a, b);
    let ok = if goes {
        read_u8(read_usize(this + 4).wrapping_add(read_usize(person))) == 1 && list.count() == before + 1
    } else {
        list.count() == before
    };
    SCOREBOARD.record(ok, || format!("file_complaint(person {:#x}): goes ahead {}, disputes {} -> {}", person, goes, before, list.count()));
}

// ---------- batch 348: building the key nation table (key_nation.cpp; sx12b_1) ----------

/// A season date of a key nation row: day and month (0-based) of the current year (`G_CURRENT_DATE_YEAR`) or of the
/// next, and the weekday the date is moved to (-1: none), as `Date::Date` (0x549f70) takes them.
#[derive(Clone, Copy)]
pub(crate) struct KeyNationDate {
    day: u32,
    month: u32,
    next_year: bool,
    weekday: i32,
}

const fn date(day: u32, month: u32, next_year: bool, weekday: i32) -> KeyNationDate {
    KeyNationDate { day, month, next_year, weekday }
}

/// One of the 35 rows `key_nation_build_table` writes: the nation (`KEY_NATION_*`, 0 = none) or continent
/// (`KEY_CONTINENT_*`), the competition creation function (an exe address), the season end (day and month of 2001),
/// the split season flag, the cup and league cup (`KEY_CLUB_COMP_*`, 0 = none) and the season start and end dates.
pub(crate) struct KeyNationRow {
    nation: usize,
    continent: usize,
    create: usize,
    season_end: Option<(u32, u32)>,
    split_season: bool,
    cup: usize,
    league_cup: usize,
    start: KeyNationDate,
    end: KeyNationDate,
}

/// The rows of the key nation table, in order (the original writes them one by one, unrolled).
pub(crate) const KEY_NATION_ROWS: [KeyNationRow; 0x23] = [
    KeyNationRow { nation: 0, continent: 0, create: 0x830620, // create_world_cup_qualifying_comps
        season_end: None, split_season: true, cup: 0, league_cup: 0,
        start: date(1, 6, false, -1), end: date(1, 6, false, -1) },
    KeyNationRow { nation: 0, continent: 0, create: 0x830ac0, // create_other_int_comps
        season_end: None, split_season: true, cup: 0, league_cup: 0,
        start: date(1, 6, false, -1), end: date(1, 6, false, -1) },
    KeyNationRow { nation: 0, continent: 0, create: 0x830fe0, // create_world_club_comps
        season_end: Some((21, 5)), split_season: true, cup: 0, league_cup: 0,
        start: date(23, 5, false, -1), end: date(20, 4, false, -1) },
    KeyNationRow { nation: 0, continent: KEY_CONTINENT_EUROPE, create: 0x8312e0, // create_european_club_comps
        season_end: Some((21, 5)), split_season: true, cup: KEY_CLUB_COMP_UEFA_EUROPEAN_CHAMPIONS_LEAGUE, league_cup: 0,
        start: date(23, 5, false, -1), end: date(20, 4, false, -1) },
    KeyNationRow { nation: 0, continent: KEY_CONTINENT_NORTH_AMERICA, create: 0x831650, // create_north_american_club_comps
        season_end: Some((21, 1)), split_season: true, cup: KEY_CLUB_COMP_CONCACAF_CHAMPIONS_CUP, league_cup: 0,
        start: date(1, 6, false, -1), end: date(1, 6, false, -1) },
    KeyNationRow { nation: 0, continent: KEY_CONTINENT_SOUTH_AMERICA, create: 0x8318f0, // create_south_american_club_comps
        season_end: Some((21, 11)), split_season: true, cup: KEY_CLUB_COMP_SOUTH_AMERICAN_COPA_LIBERTADORES, league_cup: 0,
        start: date(1, 6, false, -1), end: date(1, 6, false, -1) },
    KeyNationRow { nation: 0, continent: KEY_CONTINENT_ASIA, create: 0x831ba0, // create_asian_club_comps
        season_end: Some((21, 1)), split_season: true, cup: 0, league_cup: 0,
        start: date(1, 6, false, -1), end: date(1, 6, false, -1) },
    KeyNationRow { nation: 0, continent: KEY_CONTINENT_OCEANIA, create: 0x831eb0, // create_oceania_club_comps
        season_end: Some((21, 6)), split_season: true, cup: 0, league_cup: 0,
        start: date(1, 6, false, -1), end: date(1, 6, false, -1) },
    KeyNationRow { nation: KEY_NATION_ARGENTINA, continent: 0, create: 0x832100, // create_argentine_comps
        season_end: Some((1, 7)), split_season: true, cup: 0, league_cup: 0,
        start: date(9, 7, false, -1), end: date(19, 6, false, -1) },
    KeyNationRow { nation: KEY_NATION_AUSTRALIA, continent: 0, create: 0x832380, // create_australian_comps
        season_end: Some((1, 7)), split_season: true, cup: 0, league_cup: 0,
        start: date(2, 9, false, -1), end: date(10, 5, false, -1) },
    KeyNationRow { nation: KEY_NATION_BELGIUM, continent: 0, create: 0x8325d0, // create_belgian_comps
        season_end: Some((20, 5)), split_season: true, cup: KEY_CLUB_COMP_BELGIAN_CUP, league_cup: 0,
        start: date(7, 7, false, 5), end: date(1, 5, false, -1) },
    KeyNationRow { nation: KEY_NATION_BRAZIL, continent: 0, create: 0x832980, // create_brazilian_comps
        season_end: Some((1, 0)), split_season: false, cup: KEY_CLUB_COMP_BRAZILIAN_CUP, league_cup: 0,
        start: date(18, 0, true, -1), end: date(14, 11, true, -1) },
    KeyNationRow { nation: KEY_NATION_CROATIA, continent: 0, create: 0x8330b0, // create_croatian_comps
        season_end: Some((20, 5)), split_season: true, cup: KEY_CLUB_COMP_CROATIAN_CUP, league_cup: 0,
        start: date(24, 6, false, 5), end: date(28, 4, false, -1) },
    KeyNationRow { nation: KEY_NATION_DENMARK, continent: 0, create: 0x833400, // create_danish_comps
        season_end: Some((20, 5)), split_season: true, cup: KEY_CLUB_COMP_DANISH_CUP, league_cup: 0,
        start: date(27, 6, false, 6), end: date(14, 5, false, -1) },
    KeyNationRow { nation: KEY_NATION_ENGLAND, continent: 0, create: 0x833750, // create_english_comps
        season_end: Some((20, 5)), split_season: true, cup: KEY_CLUB_COMP_ENGLISH_FA_CUP, league_cup: KEY_CLUB_COMP_ENGLISH_LEAGUE_CUP,
        start: date(10, 7, false, 5), end: date(31, 4, false, -1) },
    KeyNationRow { nation: KEY_NATION_FINLAND, continent: 0, create: 0x833cf0, // create_finnish_comps
        season_end: Some((27, 11)), split_season: false, cup: KEY_CLUB_COMP_FINNISH_CUP, league_cup: 0,
        start: date(29, 3, true, 5), end: date(12, 10, true, -1) },
    KeyNationRow { nation: KEY_NATION_FRANCE, continent: 0, create: 0x833ff0, // create_french_comps
        season_end: Some((20, 5)), split_season: true, cup: KEY_CLUB_COMP_FRENCH_CUP, league_cup: KEY_CLUB_COMP_FRENCH_LEAGUE_CUP,
        start: date(9, 7, false, 5), end: date(24, 4, false, -1) },
    KeyNationRow { nation: KEY_NATION_GERMANY, continent: 0, create: 0x8343f0, // create_german_comps
        season_end: Some((20, 5)), split_season: true, cup: KEY_CLUB_COMP_GERMAN_CUP, league_cup: 0,
        start: date(29, 6, false, 5), end: date(26, 4, false, -1) },
    KeyNationRow { nation: KEY_NATION_GREECE, continent: 0, create: 0x8347c0, // create_greek_comps
        season_end: Some((20, 5)), split_season: true, cup: KEY_CLUB_COMP_GREEK_CUP, league_cup: 0,
        start: date(15, 7, false, 5), end: date(21, 4, false, -1) },
    KeyNationRow { nation: KEY_NATION_HOLLAND, continent: 0, create: 0x834b10, // create_dutch_comps
        season_end: Some((20, 5)), split_season: true, cup: KEY_CLUB_COMP_DUTCH_AMSTEL_CUP, league_cup: 0,
        start: date(16, 7, false, 5), end: date(7, 5, false, -1) },
    KeyNationRow { nation: KEY_NATION_REPUBLIC_OF_IRELAND, continent: 0, create: 0x834e60, // create_irish_comps
        season_end: Some((20, 5)), split_season: true, cup: KEY_CLUB_COMP_IRISH_SENIOR_CHALLENGE_CUP, league_cup: 0,
        start: date(1, 7, false, 6), end: date(20, 4, false, 6) },
    KeyNationRow { nation: KEY_NATION_ITALY, continent: 0, create: 0x835320, // create_italian_comps
        season_end: Some((20, 5)), split_season: true, cup: KEY_CLUB_COMP_ITALIAN_CUP, league_cup: 0,
        start: date(25, 7, false, 6), end: date(8, 5, false, -1) },
    KeyNationRow { nation: KEY_NATION_JAPAN, continent: 0, create: 0x835900, // create_japanese_comps
        season_end: Some((25, 0)), split_season: false, cup: 0, league_cup: 0,
        start: date(6, 2, true, 5), end: date(28, 10, false, -1) },
    KeyNationRow { nation: KEY_NATION_NORTHERN_IRELAND, continent: 0, create: 0x835c90, // create_northern_irish_comps
        season_end: Some((20, 5)), split_season: true, cup: KEY_CLUB_COMP_NORTHERN_IRISH_CUP, league_cup: 0,
        start: date(12, 7, false, 5), end: date(8, 4, false, -1) },
    KeyNationRow { nation: KEY_NATION_NORWAY, continent: 0, create: 0x836050, // create_norwegian_comps
        season_end: Some((27, 10)), split_season: false, cup: KEY_CLUB_COMP_NORWEGIAN_CUP, league_cup: 0,
        start: date(12, 3, true, 6), end: date(31, 9, true, -1) },
    KeyNationRow { nation: KEY_NATION_POLAND, continent: 0, create: 0x836350, // create_polish_comps
        season_end: Some((20, 5)), split_season: true, cup: KEY_CLUB_COMP_POLISH_FA_CUP, league_cup: 0,
        start: date(17, 6, false, 5), end: date(5, 5, false, -1) },
    KeyNationRow { nation: KEY_NATION_PORTUGAL, continent: 0, create: 0x8366e0, // create_portuguese_comps
        season_end: Some((20, 5)), split_season: true, cup: KEY_CLUB_COMP_PORTUGUESE_CUP, league_cup: 0,
        start: date(25, 7, false, 6), end: date(8, 5, false, -1) },
    KeyNationRow { nation: KEY_NATION_RUSSIA, continent: 0, create: 0x836ac0, // create_russian_comps
        season_end: Some((27, 11)), split_season: false, cup: KEY_CLUB_COMP_RUSSIAN_CUP, league_cup: 0,
        start: date(25, 2, true, 5), end: date(12, 10, true, 6) },
    KeyNationRow { nation: KEY_NATION_SCOTLAND, continent: 0, create: 0x836dc0, // create_scottish_comps
        season_end: Some((20, 5)), split_season: true, cup: KEY_CLUB_COMP_SCOTTISH_CUP, league_cup: 0,
        start: date(2, 7, false, 5), end: date(24, 4, false, -1) },
    KeyNationRow { nation: KEY_NATION_SOUTH_KOREA, continent: 0, create: 0x837200, // create_south_korean_comps
        season_end: Some((20, 0)), split_season: false, cup: KEY_CLUB_COMP_KOREAN_FA_CUP, league_cup: 0,
        start: date(25, 2, true, 6), end: date(25, 10, true, -1) },
    KeyNationRow { nation: KEY_NATION_SPAIN, continent: 0, create: 0x837560, // create_spanish_comps
        season_end: Some((20, 5)), split_season: true, cup: KEY_CLUB_COMP_SPANISH_CUP, league_cup: 0,
        start: date(1, 8, false, 6), end: date(14, 5, false, -1) },
    KeyNationRow { nation: KEY_NATION_SWEDEN, continent: 0, create: 0x8379f0, // create_swedish_comps
        season_end: Some((27, 11)), split_season: false, cup: KEY_CLUB_COMP_SWEDISH_CUP, league_cup: 0,
        start: date(6, 3, true, 6), end: date(26, 9, true, -1) },
    KeyNationRow { nation: KEY_NATION_TURKEY, continent: 0, create: 0x837d70, // create_turkish_comps
        season_end: Some((20, 5)), split_season: true, cup: KEY_CLUB_COMP_TURKISH_FA_CUP, league_cup: 0,
        start: date(8, 7, false, 6), end: date(21, 4, false, -1) },
    KeyNationRow { nation: KEY_NATION_UNITED_STATES, continent: 0, create: 0x8380c0, // create_united_states_comps
        season_end: Some((20, 1)), split_season: false, cup: 0, league_cup: 0,
        start: date(22, 2, true, 5), end: date(6, 0, true, -1) },
    KeyNationRow { nation: KEY_NATION_WALES, continent: 0, create: 0x8383b0, // create_welsh_comps
        season_end: Some((20, 5)), split_season: true, cup: KEY_CLUB_COMP_WELSH_CUP, league_cup: 0,
        start: date(19, 7, false, 5), end: date(6, 4, false, -1) },
];

/// The cup comp pointer (+0x1e) and the league cup comp pointer (+0x22) of a key nation row.
const KEY_NATION_CUP: usize = 0x1e;
const KEY_NATION_LEAGUE_CUP: usize = 0x22;

/// The record of a fixed key in a database table of `size`-byte records (`table` holds the table's address): table +
/// key x size, in 32-bit arithmetic as the original.
unsafe fn keyed_record(table: usize, key: usize, size: usize) -> usize {
    read_usize(table).wrapping_add((read_i32(key) as usize).wrapping_mul(size))
}

/// Writes a season date into a key nation row: `Date::Date` (0x549f70) of day, month, year and weekday into a
/// temporary, then `Date::operator=` into the row.
unsafe fn write_key_nation_date(at: usize, d: KeyNationDate) {
    let year = read_u16(G_CURRENT_DATE_YEAR).wrapping_add(d.next_year as u16);
    let (mut built, mut tmp) = ([0u16; 4], [0u8; 8]);
    let p = exe::date_date_549f70(&mut built, d.day, d.month, year as u32, d.weekday as u32);
    exe::date_operator_3d(at, tmp.as_mut_ptr() as usize, p);
}

/// Fills the table of the 35 key nations (`G_LEAGUE_TABLE`, 72-byte rows) for a new game; returns 1, or 0 without the
/// nation data.
///
/// `key_nation_build_table()`: with no nation table (`G_NATION_TABLE` 0) the "Error" box (key_nation.cpp line 0x36)
/// and 0. Otherwise the whole table is cleared (0x9d8 bytes) and each row of `KEY_NATION_ROWS` written: the nation
/// record (+0, `G_NATION_TABLE` + key x `nation::SIZE`) or the continent record (+4, `G_CONTINENT_TABLE` + key x 0xc6),
/// the competition creation function (+8, called later by `key_nation_create_all_comps`), the season end day
/// (`key_nation_record::SEASON_END_DAY`, the day of the year of that day and month in 2001), the split season flag
/// (`key_nation_record::SPLIT_SEASON_FLAG`, a dword), the cup (+0x1e) and league cup (+0x22) records
/// (`G_CLUB_COMP_TABLE` + key x 0x6b) and the season start (`key_nation_record::START_DATE`) and end
/// (`key_nation_record::END_DATE`) dates of the current or the next year. The rows are: World Cup qualifying, the
/// other international competitions, the world club competitions, the five continents' club competitions (Europe,
/// North America, South America, Asia, Oceania) and the 27 league nations from Argentina to Wales. Returns 1. Called by
/// the game setup (0x81ce00, 0x827110, 0x82a9c0).
/// The original pushes each year as a dword whose high word is stack residue; `Date::Date` reads only the word.
/// Original: `key_nation_build_table` (0x667150, key_nation.cpp, cdecl).
pub(crate) unsafe extern "C" fn key_nation_build_table() -> u32 {
    if read_usize(G_NATION_TABLE) == 0 {
        show_error_at(0x9eaff4, 0x36); // "...\key_nation.cpp"
        return 0;
    }
    core::ptr::write_bytes(G_LEAGUE_TABLE as *mut u8, 0, 0x23 * 72);
    for (k, row) in KEY_NATION_ROWS.iter().enumerate() {
        let at = G_LEAGUE_TABLE + 72 * k;
        if row.nation != 0 {
            write_usize(at, keyed_record(G_NATION_TABLE, row.nation, nation::SIZE));
        }
        if row.continent != 0 {
            write_usize(at + 4, keyed_record(G_CONTINENT_TABLE, row.continent, 0xc6));
        }
        write_usize(at + 8, row.create);
        write_u32(at + key_nation_record::SPLIT_SEASON_FLAG, row.split_season as u32); // sx12b_1: the split season flag
        for (field, cup) in [(KEY_NATION_CUP, row.cup), (KEY_NATION_LEAGUE_CUP, row.league_cup)] {
            if cup != 0 {
                write_usize(at + field, keyed_record(G_CLUB_COMP_TABLE, cup, 0x6b));
            }
        }
        if let Some((day, month)) = row.season_end {
            let mut end = [0u16; 4];
            exe::date_date_549f70(&mut end, day, month, 2001, u32::MAX);
            write_u16(at + key_nation_record::SEASON_END_DAY, end[0]); // sx12b_1: the season end day
        }
        write_key_nation_date(at + key_nation_record::START_DATE, row.start);
        write_key_nation_date(at + key_nation_record::END_DATE, row.end);
    }
    covered!("sx12b_1 key nation table built");
    1
}

/// Comparer for `key_nation_build_table`: the table (35 x 72 bytes) and the return, side by side (the routine only
/// writes the table). Without the nation data (the "Error" box): only the original.
pub(crate) unsafe extern "C" fn compare_key_nation_build_table() -> u32 {
    let o = original_fn::<unsafe extern "C" fn() -> u32>(IDX_KEY_NATION_BUILD_TABLE);
    if read_usize(G_NATION_TABLE) == 0 {
        return o();
    }
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&[(G_LEAGUE_TABLE, 0x23 * 72)], || o(), || key_nation_build_table());
    SCOREBOARD.record(equal && r == n, || format!("key_nation_build_table original {} rust {}: {:?}", r, n, first_differences(&mo, &mr)));
    r
}

// ---------- batch 348: the human manager and injury constructors (human_manager.cpp, injury.cpp; sx12b_2) ----------

/// `HUMAN_MANAGER::load` (thiscall(this, stream), ret 4, eax 1/0) and `Injury::load` (the same), called through the exe
/// so that their own detours run.
const FN_HUMAN_MANAGER_LOAD: usize = 0x5fee50;
const FN_INJURY_LOAD: usize = 0x62ab00;
/// The constructor and destructor of one human record and the constructor of one tactic slot.
const FN_HUMAN_RECORD_CONSTRUCTOR: usize = 0x5fb690;
const FN_HUMAN_RECORD_DESTRUCTOR: usize = 0x5fb700;
const FN_HUMAN_TACTIC_SLOT_CONSTRUCTOR: usize = 0x5fb720;
/// A human record (0xa6ef bytes) and one of its five tactic slots (0x1ecd bytes).
pub(crate) const HUMAN_RECORD_SIZE: usize = 0xa6ef;
const HUMAN_TACTIC_SLOT_SIZE: u32 = 0x1ecd;

/// Builds the human manager table, for a new game (`stream` 0) or from the saved game.
///
/// `HUMAN_MANAGER::HUMAN_MANAGER(stream)`: the CD search (`find_game_cd`, the no-CD patch read at 0x5fb47f and
/// 0x5fb4c2). With a `stream`, the table is read from the saved game (`HUMAN_MANAGER::load`); when that works, nothing
/// else. Otherwise the count (`human_manager_fields::RECORD_COUNT`) is 16 and the records an array of 16 `HUMAN_RECORD`
/// (`operator new` of 16 x 0xa6ef + 4 bytes, the count in the first dword, the constructors and destructors by the
/// vector iterator); none: the "Error" box (human_manager.cpp line 0x4b) and nothing more. The records are then zeroed
/// and each one reset (`HUMAN_MANAGER::reset_human_record`) for the staff rows from the last one down (the 16 human
/// manager rows at the end of the staff table); the count at +8 and `human_manager_fields::HOLIDAY_COUNT` are 0.
/// Returns this.
/// Original: `HUMAN_MANAGER::HUMAN_MANAGER` (0x5fb430, human_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn construct_human_manager(this: usize, stream: usize) -> usize {
    find_game_cd(0x5fb47f, 0x5fb4c2);
    if stream != 0 {
        let load: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_LOAD);
        if load(this, stream) != 0 {
            covered!("sx12b_2 human managers from the save");
            return this;
        }
    }
    write_u32(this + human_manager_fields::RECORD_COUNT, 0x10);
    let block = exe::crt_operator_new(0x10 * HUMAN_RECORD_SIZE as u32 + 4);
    let records = if block != 0 {
        write_u32(block, 0x10);
        exe::crt_vector_constructor_iterator(block + 4, HUMAN_RECORD_SIZE as u32, 0x10, FN_HUMAN_RECORD_CONSTRUCTOR, FN_HUMAN_RECORD_DESTRUCTOR);
        block + 4
    } else {
        0
    };
    write_usize(this, records);
    if records == 0 {
        show_error_at(0x9cf11c, 0x4b); // "...\human_manager.cpp"
        return this;
    }
    let count = read_i32(this + human_manager_fields::RECORD_COUNT);
    core::ptr::write_bytes(records as *mut u8, 0, (count as usize).wrapping_mul(HUMAN_RECORD_SIZE));
    let mut k = 0;
    while k < read_i32(this + human_manager_fields::RECORD_COUNT) {
        let row = read_i32(G_STAFF_COUNT) - k - 1;
        exe::human_manager_reset_human_record(this, read_usize(G_STAFF_TABLE).wrapping_add((row as usize).wrapping_mul(staff::SIZE))); // sx12b_2: each record reset
        k += 1;
    }
    covered!("sx12b_2 new human managers");
    write_u32(this + 8, 0); // the count of computer controlled managers
    write_u32(this + human_manager_fields::HOLIDAY_COUNT, 0);
    this
}

/// Builds one human record: its five tactic slots (each a Formation block, by the vector iterator), the draft contract
/// (+0x9a09, `STAFF_CONTRACT::STAFF_CONTRACT`) and the squad view settings (+0xa6d4, their defaults: 0x41, 8, 0x154a21,
/// 6, -1, -1). Returns the record.
/// Original: `HUMAN_RECORD::HUMAN_RECORD` (0x5fb690, human_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn construct_human_record(this: usize) -> usize {
    exe::crt_vector_constructor_iterator(this, HUMAN_TACTIC_SLOT_SIZE, 5, FN_HUMAN_TACTIC_SLOT_CONSTRUCTOR, 0x5a53c0usize); // the empty destructor
    exe::new_staff_contract(this + 0x9a09); // the draft contract
    let defaults: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x487810usize); // SQUAD_VIEW_SETTINGS::SQUAD_VIEW_SETTINGS
    defaults(this + 0xa6d4); // sx12b_2: the squad view settings
    covered!("sx12b_2 human record");
    this
}

/// Builds the injury object, for a new game (`stream` 0) or from the saved game.
///
/// `Injury::Injury(stream)`: the CD search (`find_game_cd`, the no-CD patch read at 0x62a6cf and 0x62a713). The injury
/// history list (`injury_fields::INJURY_HISTORY`, "CM3_TEMP\injury_history.tmp", 0x2c-byte records,
/// `club_history_new_file_list`); none stops the game (injury.cpp line 0xb4). The injury array, its count, the recovery
/// news queue (+0x10, +0x14, `injury_fields::RECOVERY_NEWS_COUNT`), the outbreaks (array, capacity, count) and the match
/// hit list are zeroed (`injury_fields::PHYSICAL_RECORDS` is left as it was). With a `stream`, the state is read from
/// the saved game (`Injury::load`); otherwise, or when that fails, the records of a new game (`init_records`). Returns
/// this.
/// Original: `Injury::Injury` (0x62a680, injury.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn construct_injury(this: usize, stream: usize) -> usize {
    find_game_cd(0x62a6cf, 0x62a713);
    let history = club_history_new_file_list(0x9e7f54, 0x2c); // "injury_history.tmp"
    write_usize(this + injury_fields::INJURY_HISTORY, history);
    if history == 0 {
        fatal_error_at(0x9e7f28, 0xb4); // "...\injury.cpp"
    }
    use injury_fields as f;
    for field in [f::INJURY_ENTRIES, f::INJURY_ENTRY_COUNT, f::RECOVERY_NEWS_QUEUE, 0x14, f::RECOVERY_NEWS_COUNT, f::OUTBREAKS,
                  f::OUTBREAK_CAPACITY, f::OUTBREAK_COUNT, f::MATCH_HIT_LIST] {
        write_u32(this + field, 0); // 0x14: the recovery news capacity [inference]
    }
    if stream != 0 {
        let load: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_INJURY_LOAD);
        if load(this, stream) != 0 {
            covered!("sx12b_2 injuries from the save");
            return this;
        }
    }
    covered!("sx12b_2 new injury records");
    init_records(this);
    this
}

/// Comparer for `construct_human_manager`: it searches the CD, allocates and resets (or loads) the records: it cannot
/// run twice. Only the original runs; the copy is proven by the new career (Rust mode).
pub(crate) unsafe extern "thiscall" fn compare_construct_human_manager(this: usize, stream: usize) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_CONSTRUCT_HUMAN_MANAGER)(this, stream)
}

/// Comparer for `construct_human_record`: the record (0xa6ef bytes), side by side (the constructors only write it).
pub(crate) unsafe extern "thiscall" fn compare_construct_human_record(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_CONSTRUCT_HUMAN_RECORD);
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&[(this, HUMAN_RECORD_SIZE)], || o(this), || construct_human_record(this));
    SCOREBOARD.record(equal && r == n, || format!("HUMAN_RECORD::HUMAN_RECORD({:#x}): {:?}", this, first_differences(&mo, &mr)));
    r
}

/// Comparer for `construct_injury`: it searches the CD, opens the history file list and builds (or loads) the records:
/// it cannot run twice. Only the original runs; the copy is proven by the new career (Rust mode).
pub(crate) unsafe extern "thiscall" fn compare_construct_injury(this: usize, stream: usize) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_CONSTRUCT_INJURY)(this, stream)
}

// ---------- batch 348: the hall of fame and key nation constructors (hall_of_fame.cpp, key_nation.cpp; sx12b_3) ----------

/// `HallOfFame::load` (thiscall, eax 1/0) and `HallOfFameEntry::HallOfFameEntry`, called through the exe so that their
/// own detours run.
const FN_HALL_OF_FAME_LOAD: usize = 0x5efaa0;
pub(crate) const FN_HALL_OF_FAME_ENTRY_CONSTRUCTOR: usize = 0x5ef120;

/// Builds the hall of fame: the 2800 entries (28 tables of 100), cleared, then read from `hall_of_fame.bin`; when that
/// fails (no file or a bad one), a new file is written from the cleared tables.
///
/// `HallOfFame::HallOfFame()`: the entry constructors (`HallOfFameEntry::HallOfFameEntry`, by the vector iterator, the
/// empty destructor), the whole table zeroed (0x52080 bytes), `HallOfFame::load`, and `HallOfFame::save` when it
/// returns 0. Returns this. Called once at the program start (0x5cc773).
/// Original: `HallOfFame::HallOfFame` (0x5ef0b0, hall_of_fame.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn construct_hall_of_fame(this: usize) -> usize {
    use hall_of_fame_consts as hf;
    exe::crt_vector_constructor_iterator(this, hf::ENTRY_SIZE as u32, (hf::TABLES * hf::ENTRIES) as u32,
        FN_HALL_OF_FAME_ENTRY_CONSTRUCTOR, 0x5a53c0usize); // the empty destructor
    core::ptr::write_bytes(this as *mut u8, 0, hf::TABLES * hf::TABLE_SIZE);
    let load: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_HALL_OF_FAME_LOAD);
    if load(this) == 0 {
        covered!("sx12b_3 hall of fame file written anew");
        let save: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_HALLOFFAME_SAVE);
        save(this);
    }
    this
}

/// Builds one hall of fame entry (120 bytes): only its date (`hall_of_fame_consts::DATE`, `Date::Date` empty).
/// Returns the entry.
/// Original: `HallOfFameEntry::HallOfFameEntry` (0x5ef120, hall_of_fame.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn construct_hall_of_fame_entry(this: usize) -> usize {
    exe::date_date_54a110(this + hall_of_fame_consts::DATE); // sx12b_3: the entry date
    this
}

/// Builds one key nation entry (72 bytes): only its season start (`key_nation_record::START_DATE`) and end
/// (`key_nation_record::END_DATE`) dates (`Date::Date` empty). Returns the entry.
/// Original: `KEY_NATION::KEY_NATION` (0x667100, key_nation.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn construct_key_nation(this: usize) -> usize {
    exe::date_date_54a110(this + key_nation_record::START_DATE);
    exe::date_date_54a110(this + key_nation_record::END_DATE); // sx12b_3: the end date
    covered!("sx12b_3 key nation entry");
    this
}

/// Comparer for `construct_hall_of_fame`: the whole table and the return, side by side (the load reads the same file
/// twice; when it fails, the original writes a new file and the copy then reads it back: the table is the same).
pub(crate) unsafe extern "thiscall" fn compare_construct_hall_of_fame(this: usize) -> usize {
    let bytes = hall_of_fame_consts::TABLES * hall_of_fame_consts::TABLE_SIZE;
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_CONSTRUCT_HALL_OF_FAME);
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&[(this, bytes)], || o(this), || construct_hall_of_fame(this));
    SCOREBOARD.record(equal && r == n, || format!("HallOfFame::HallOfFame: {:?}", first_differences(&mo, &mr)));
    r
}

/// Comparer for `construct_hall_of_fame_entry`: the entry, side by side (it runs 2800 times at the start and at each
/// hall of fame construction).
pub(crate) unsafe extern "thiscall" fn compare_construct_hall_of_fame_entry(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_CONSTRUCT_HALL_OF_FAME_ENTRY);
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side_light(&[(this, hall_of_fame_consts::ENTRY_SIZE)], || o(this),
        || construct_hall_of_fame_entry(this));
    SCOREBOARD.record(equal && r == n, || format!("HallOfFameEntry::HallOfFameEntry({:#x}): {:?}", this, first_differences(&mo, &mr)));
    r
}

/// Comparer for `construct_key_nation`: the entry, side by side.
pub(crate) unsafe extern "thiscall" fn compare_construct_key_nation(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_CONSTRUCT_KEY_NATION);
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side_light(&[(this, 72)], || o(this), || construct_key_nation(this));
    SCOREBOARD.record(equal && r == n, || format!("KEY_NATION::KEY_NATION({:#x}): {:?}", this, first_differences(&mo, &mr)));
    r
}

// ---------- batch 348: the requests to the FA (manager_manager.cpp) ----------

/// `Media::queue_manager_fa_complaint_story` (0x758de0; thiscall(media, club, person, kind, a, b, c), ret 0x18).
const FN_MEDIA_QUEUE_MANAGER_FA_COMPLAINT_STORY: usize = 0x758de0;
/// `Finance::pay_league_fine` (0x599850; thiscall(finance, amount), ret 4).
const FN_FINANCE_PAY_LEAGUE_FINE: usize = 0x599850;
/// `OfficialsManager::get_average_rating` (0x79d3e0; thiscall(om, official), ret 4; the float in st0).
const FN_OFFICIALSMANAGER_GET_AVERAGE_RATING: usize = 0x79d3e0;
/// `OfficialsManager::get_last_match_rating` (0x79d3b0; thiscall(om, official), ret 4; the char in al).
const FN_OFFICIALSMANAGER_GET_LAST_MATCH_RATING: usize = 0x79d3b0;
/// `OfficialsManager::reset_official_stats` (0x79d190; thiscall(om, official), ret 4).
const FN_OFFICIALSMANAGER_RESET_OFFICIAL_STATS: usize = 0x79d190;
/// `MANAGER_MANAGER::post_fa_request_reply_news` (0x6aaa90; thiscall(this, club, request, outcome, standing), ret 0x10).
const FN_MANAGER_MANAGER_POST_FA_REQUEST_REPLY_NEWS: usize = 0x6aaa90;
/// `MANAGER_MANAGER::post_match_rescheduled_news` (0x6aa7d0; thiscall(this, fixture, club id, count), ret 0xc).
const FN_MANAGER_MANAGER_POST_MATCH_RESCHEDULED_NEWS: usize = 0x6aa7d0;
/// The two 0x625-byte match team blocks the routine builds and destroys without using them.
const FA_REQUEST_TEAM_BLOCK_CONSTRUCT: usize = 0x453c30;
const FA_REQUEST_TEAM_BLOCK_DESTRUCT: usize = 0x453c10;
/// Size of an official record in `G_OFFICIAL_TABLE`.
const FA_REQUEST_OFFICIAL_SIZE: usize = 0x2b;

/// The FA's reply news to `club` (through the exe address, so the hook and its comparer run).
unsafe fn fa_request_reply(this: usize, club: Club, request: i8, outcome: u32, standing: u32) {
    let post: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) = core::mem::transmute(FN_MANAGER_MANAGER_POST_FA_REQUEST_REPLY_NEWS);
    post(this, club.addr(), request as i32 as u32, outcome, standing);
}

/// The board's count of the club's FA requests (board record +0, unnamed; signed byte).
unsafe fn fa_request_count(row: BoardRecord) -> i8 {
    read_i8(row.addr())
}

/// The club's sanction after one complaint too many: the media story of the outburst (kind 5, random(3) + 1), a
/// 25000 league fine, the board confidence (in `G_MANAGER_MANAGER`'s table) moved by 250 (or, with the club state +4
/// below 0, by limit × 250 − the count), kept within 1..10000, and the sacking check.
unsafe fn fa_request_sanction(this: usize, club: Club, limit: i8) {
    covered!("sx14b_1 FA sanction");
    let story: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32) = core::mem::transmute(FN_MEDIA_QUEUE_MANAGER_FA_COMPLAINT_STORY);
    let r = exe::utils_random(3);
    story(read_usize(G_MEDIA), club.addr(), club.manager().addr(), 5, u32::MAX, u32::MAX, (r as u8).wrapping_add(1) as u32);
    let finance = exe::financetable_get_club_finance(read_usize(G_FINANCE_TABLE), club);
    if finance != 0 {
        let fine: unsafe extern "thiscall" fn(usize, i32) = core::mem::transmute(FN_FINANCE_PAY_LEAGUE_FINE);
        fine(finance, 25000);
    }
    let manager_manager = read_usize(G_MANAGER_MANAGER);
    let row = BoardRecord(board_row(manager_manager, club));
    let delta = if read_i8(club_state_record(club.addr()) + 4) < 0 {
        // the count from this object's table, the confidence in G_MANAGER_MANAGER's (the same object in the game)
        (limit as i16).wrapping_mul(250).wrapping_sub(fa_request_count(BoardRecord(board_row(this, club))) as i16)
    } else {
        250
    };
    row.set_confidence(row.confidence().wrapping_add(delta).clamp(1, 10000));
    exe::manager_manager_consider_sacking_manager(manager_manager, club, 0);
}

/// A postponement request: the first fixture in the next 8 weeks that is not today and not played. A fixture of another
/// competition than the club's league, or the key nation's season ending within 60 days of today (or, when it ends in
/// the first 60 days of the year, today past day 340): outcome 6. Granted with more than 2 squad players on
/// international duty that day, or with 2 of them or more than 4 happy (happiness above 20) and injured players when
/// the club's reputation beats 4750 + random(1250): the fixture is moved (removed, a new date, added again) with the
/// reply (outcome 0) and the rescheduling news; otherwise outcome 1.
unsafe fn fa_postponement_request(this: usize, club: Club, request: i8, list: usize, date: usize) {
    let key = exe::key_nation_index_of_club(club) as i8;
    let (mut a, mut b) = ([0u8; 8], [0u8; 8]);
    let year = read_u16(exe::date_add_weeks(G_CURRENT_DATE, &mut a, 8) + 2);
    let day = read_u16(exe::date_add_weeks(G_CURRENT_DATE, &mut b, 8));
    let (today, this_year) = (read_i16(G_CURRENT_DATE), read_u16(G_CURRENT_DATE_YEAR));
    // simplification: the high halves of the date words pushed to find_fixtures are register garbage in the original
    exe::fixture_manager_find_fixtures(read_usize(G_FIXTURE_MANAGER), today as u16 as u32, this_year as u32, day as u32, year as u32,
        read_u32(club.addr()), list, u32::MAX, (-2i32) as u32, 1);
    if read_u16(list) == 0 {
        return;
    }
    write_u32(list + 0xa, read_u32(list + 2)); // the cursor at the head
    let fixture = loop {
        let f = Fixture(exe::llist_get_next(list));
        if f.is_null() {
            return;
        }
        let is_today = f.day() as i32 == today as i32 && f.year() == this_year;
        if !is_today && f.home_goals() == -1 {
            break f;
        }
    };
    covered!("sx14b_1 postponement");
    let date_new: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE);
    let (mut made, mut out) = ([0u8; 8], [0u8; 8]);
    let d = date_new(made.as_mut_ptr() as usize, fixture.day() as u32, fixture.year() as u32);
    exe::date_operator_3d(date, out.as_mut_ptr() as usize, d);
    let mut on_duty = 0u8;
    for k in 0..50 {
        let p = club.squad(k);
        if p != 0 && exe::nationalteams_get_international_duty_club(read_usize(G_NATIONAL_TEAMS), p, date, club, NEWS_TEXT_BUFFER as u32, 0) != 0 {
            on_duty = on_duty.wrapping_add(1);
        }
    }
    let mut injured = 0u8;
    for k in 0..50 {
        let p = club.squad(k);
        if p == 0 {
            continue;
        }
        let c = Contract(exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), p));
        if !c.is_null() && c.happiness() > 20 && exe::injury_get_player_injury_status(read_usize(G_INJURY), p) == 2 {
            injured = injured.wrapping_add(1);
        }
    }
    if fixture.competition().addr() != club.division().addr() {
        return fa_request_reply(this, club, request, 6, 0);
    }
    if (0..0x23).contains(&key) {
        let e = KeyNationRecord(G_LEAGUE_TABLE + key as usize * 0x48);
        let end = e.end_date();
        if e.end_date_year() as u16 == this_year && if end > 0x3c { (end as i32 - 0x3c) < today as i32 } else { today > 0x154 } {
            covered!("sx14b_1 postponement too late");
            return fa_request_reply(this, club, request, 6, 0);
        }
    }
    let on_duty = on_duty as i8;
    let granted = on_duty > 2 || ((on_duty > 1 || injured as i8 > 4) && club.reputation() > (exe::utils_random(0x4e2) + 0x128e) as i16);
    if !granted {
        return fa_request_reply(this, club, request, 1, 0);
    }
    covered!("sx14b_1 postponement granted");
    let mut copy = [0u8; fixture::SIZE];
    core::ptr::copy_nonoverlapping(fixture.addr() as *const u8, copy.as_mut_ptr(), fixture::SIZE);
    let fm = read_usize(G_FIXTURE_MANAGER);
    exe::fixture_manager_remove_fixture(fm, fixture);
    fa_request_reply(this, club, request, 0, 0);
    exe::fixture_manager_find_new_fixture_date(fm, copy.as_mut_ptr() as usize);
    exe::fixture_manager_add_fixture(fm, copy.as_mut_ptr() as usize, 1);
    // defect of the original: the news reads the fixture that remove_fixture just took out of the calendar
    let news: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(FN_MANAGER_MANAGER_POST_MATCH_RESCHEDULED_NEWS);
    news(this, fixture.addr(), read_u32(club.addr()), on_duty as u8 as u32);
}

/// A referee complaint: the last played fixture of the past 8 weeks; its referee with an average rating below 5.5 and
/// a last match rating below 7 is replaced in 1 of 200 cases (a new name from his nation, discipline random(16) + 5 or
/// random(11) + random(11), born in a random day of (this year − 27 + random(5)), his stats reset, and the news 0x2b to
/// the managers of the club's nation); otherwise the reply (outcome 1) with his last match rating.
unsafe fn fa_referee_complaint(this: usize, club: Club, request: i8, list: usize) {
    let (mut a, mut b) = ([0u8; 8], [0u8; 8]);
    let year = read_u16(exe::date_sub_weeks(G_CURRENT_DATE, &mut a, 8) + 2);
    let day = read_u16(exe::date_sub_weeks(G_CURRENT_DATE, &mut b, 8));
    let (today, this_year) = (read_u16(G_CURRENT_DATE), read_u16(G_CURRENT_DATE_YEAR));
    exe::fixture_manager_find_fixtures(read_usize(G_FIXTURE_MANAGER), day as u32, year as u32, today as u32, this_year as u32,
        read_u32(club.addr()), list, u32::MAX, (-2i32) as u32, 1);
    if read_u16(list) == 0 {
        return;
    }
    write_u32(list + 0xa, read_u32(list + 6)); // the cursor at the tail
    let fixture = loop {
        let f = Fixture(exe::llist_get_prev(list));
        if f.is_null() {
            return;
        }
        if f.home_goals() > -1 {
            break f;
        }
    };
    covered!("sx14b_1 referee complaint");
    let official = Official(read_usize(G_OFFICIAL_TABLE).wrapping_add((fixture.referee() as i32 as usize).wrapping_mul(FA_REQUEST_OFFICIAL_SIZE)));
    let om = read_usize(G_OFFICIALS_MANAGER);
    let average: unsafe extern "thiscall" fn(usize, usize) -> f32 = core::mem::transmute(FN_OFFICIALSMANAGER_GET_AVERAGE_RATING);
    let last: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(FN_OFFICIALSMANAGER_GET_LAST_MATCH_RATING);
    if !((average(om, official.addr()) as f64) < read_f64(rdata::F64_5_5) && (last(om, official.addr()) as i8) < 7) || exe::utils_random(200) != 0 {
        return fa_request_reply(this, club, request, 1, last(om, official.addr()) as u32);
    }
    covered!("sx14b_1 referee replaced");
    let old = read_u32(official.second_name() + names::ID);
    official.set_first_name(exe::database_pick_first_name_for_nation(official.nation()));
    official.set_second_name(exe::database_pick_second_name_for_nation(official.nation()));
    let discipline = if exe::utils_random(10) != 0 {
        (exe::utils_random(0x10) as u8).wrapping_add(5)
    } else {
        let r = exe::utils_random(0xb) as u8;
        r.wrapping_add(exe::utils_random(0xb) as u8)
    };
    official.set_discipline(discipline);
    let r = exe::utils_random(5) as i16;
    official.set_birth_year(r.wrapping_add((this_year as i16).wrapping_sub(0x1b)));
    official.set_birth_day(exe::utils_random(0x154) as i16);
    let reset: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_OFFICIALSMANAGER_RESET_OFFICIAL_STATS);
    reset(om, official.addr());
    let new = read_u32(official.second_name() + names::ID);
    let (nation, manager) = (club.nation(), club.manager());
    if nation.is_null() {
        return;
    }
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0x2b, 0);
    exe::newsitem_set_slot(n, 0, read_u32(nation.addr()));
    exe::newsitem_set_slot(n, 1, old);
    exe::newsitem_set_slot(n, 2, new);
    exe::newsitem_set_slot(n, 3, read_u32(manager.addr()));
    exe::news_send_to_nation_managers(read_usize(G_NEWS), n, nation);
}

/// The request itself (inside the frame of `handle_fa_request`).
unsafe fn fa_request(this: usize, club: Club, request: i8, list: usize, date: usize) {
    if club.is_null() || club.division().is_null() || club.manager().is_null() || club.manager().non_player().is_null() {
        return;
    }
    if exe::database_is_national_team(club) != 0 {
        return;
    }
    let row = BoardRecord(board_row(this, club));
    write_i8(row.addr(), fa_request_count(row).wrapping_add(1));
    // defect of the original: the bands are tested from the lowest, so above 3250 the limit is always 3 (the 4 above
    // 5250 and the 7 above 7750 never apply)
    let limit: i8 = if club.reputation() > 0xcb2 { 3 } else { 2 };
    if request == 1 {
        let count = fa_request_count(row);
        if count == limit && read_i8(club_state_record(club.addr()) + 4) < -1 {
            covered!("sx14b_1 FA complaint story");
            let story: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32) = core::mem::transmute(FN_MEDIA_QUEUE_MANAGER_FA_COMPLAINT_STORY);
            return story(read_usize(G_MEDIA), club.addr(), club.manager().addr(), 4, u32::MAX, u32::MAX, u32::MAX);
        }
        if count > limit && (manager_manager_is_human_club(club.addr(), 1, 0) != 0 || count as i32 == limit as i32 + 1) {
            return fa_request_sanction(this, club, limit);
        }
    }
    let count = fa_request_count(row);
    if request != 5 && count == limit {
        covered!("sx14b_1 request at the limit");
        return fa_request_reply(this, club, request, 3, 0);
    }
    if request != 5 && count > limit {
        covered!("sx14b_1 request over the limit");
        fa_request_reply(this, club, request, 5, 0);
        let row = BoardRecord(board_row(this, club));
        return row.set_confidence(row.confidence().wrapping_sub(100));
    }
    if manager_manager_is_human_club(club.addr(), 1, 0) == 0 {
        return;
    }
    match request {
        1 => fa_referee_complaint(this, club, request, list),
        2 => fa_postponement_request(this, club, request, list, date),
        _ => {}
    }
}

/// Handles a manager's request to the FA: 1 a referee complaint, 2 a postponement.
///
/// `MANAGER_MANAGER::handle_fa_request(club, request)`, called after a match for the machine's clubs (request 1,
/// `update_board_confidence_after_match`) and from the FA request screen. SEH frame for the locals: two unused match
/// team blocks (`exe::crt_vector_constructor_iterator`, 0x625 bytes each), a fixture list (`exe::new_llist`) and a
/// date (`exe::date_date_54a110`), destroyed at the end. A club without a league (`.division()`), a manager or a
/// non-player record, or a national team: nothing. The club's request count (board record +0) goes up; the limit is
/// 3 above reputation 3250, else 2. A referee complaint at the limit with the club state +4 below −1: the media story
/// of the complaint (kind 4). Over the limit, for a human club or at the limit + 1: the sanction (the story of the
/// outburst, the 25000 fine, the board confidence and the sacking check). Otherwise (other requests, but 5) at the
/// limit the reply with outcome 3; over it, outcome 5 and the board confidence − 100. Below the limit, only a human
/// club's request goes on: the referee complaint (`fa_referee_complaint`) or the postponement
/// (`fa_postponement_request`).
///
/// Defects of the original: the reputation bands are tested from the lowest, so the limits 4 (above 5250) and 7
/// (above 7750) never apply; the rescheduling news reads the fixture after `remove_fixture` took it out of the
/// calendar.
/// Original: `MANAGER_MANAGER::handle_fa_request` (0x6a9e60, manager_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn handle_fa_request(this: usize, club: usize, request: u32) {
    let mut blocks = [0u8; 2 * 0x625];
    let blocks = blocks.as_mut_ptr() as usize;
    exe::crt_vector_constructor_iterator(blocks, 0x625, 2, FA_REQUEST_TEAM_BLOCK_CONSTRUCT, FA_REQUEST_TEAM_BLOCK_DESTRUCT);
    let mut list = [0u8; 0x20];
    let list = list.as_mut_ptr() as usize;
    exe::new_llist(list);
    let mut date = [0u8; 8];
    let date = date.as_mut_ptr() as usize;
    exe::date_date_54a110(date);
    fa_request(this, Club(club), request as i8, list, date);
    exe::drop_llist(list);
    exe::crt_vector_destructor_iterator(blocks, 0x625, 2, FA_REQUEST_TEAM_BLOCK_DESTRUCT);
}

/// Comparer for `handle_fa_request`: it moves fixtures, renames referees, fines, posts news and draws, so only the
/// original runs (its effects are checked by the career).
pub(crate) unsafe extern "thiscall" fn compare_handle_fa_request(this: usize, club: usize, request: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_HANDLE_FA_REQUEST)(this, club, request)
}

/// Posts the 0x1789 news of the FA's reply to a club manager's request.
///
/// SEH frame with no destructor. `MANAGER_MANAGER::post_fa_request_reply_news(club, request, outcome, standing)`: no
/// club or a club without a nation (`.nation()`): nothing. News 0x1789 (local, `exe::new_newsitem`): 0 the club's id,
/// 1 the request, 2 the outcome, 3 the nation's id, 4 the referee's standing (the three as signed bytes); to the club's
/// manager (`exe::news_send_to_club_manager`, which only delivers to a human).
/// Original: `MANAGER_MANAGER::post_fa_request_reply_news` (0x6aaa90, manager_manager.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn post_fa_request_reply_news(_this: usize, club: usize, request: u32, outcome: u32, standing: u32) {
    let club = Club(club);
    if club.is_null() || club.nation().is_null() {
        return;
    }
    covered!("sx14b_1 FA reply news");
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0x1789, 0);
    exe::newsitem_set_slot(n, 0, read_u32(club.addr()));
    exe::newsitem_set_slot(n, 1, request as i8 as i32 as u32);
    exe::newsitem_set_slot(n, 2, outcome as i8 as i32 as u32);
    exe::newsitem_set_slot(n, 3, read_u32(club.nation().addr()));
    exe::newsitem_set_slot(n, 4, standing as i8 as i32 as u32);
    exe::news_send_to_club_manager(read_usize(G_NEWS), n, club);
}

/// Comparer for `post_fa_request_reply_news`: the news goes to the manager's inbox (allocates), so only the original
/// runs.
pub(crate) unsafe extern "thiscall" fn compare_post_fa_request_reply_news(this: usize, club: usize, request: u32, outcome: u32, standing: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32)>(IDX_POST_FA_REQUEST_REPLY_NEWS)(this, club, request, outcome, standing)
}

/// Posts the 0x29 news that the FA moved a fixture, to the managers of both clubs.
///
/// SEH frame with no destructor. `MANAGER_MANAGER::post_match_rescheduled_news(fixture, club id, count)`: no fixture,
/// or before the news start date (`before_news_start_date`): nothing. News 0x29 (local, `exe::new_newsitem`): 0 and 1
/// the home and away clubs' ids, 2 the competition's id, 3 the requesting club's id, 4 the id of the competition's
/// nation, 5 the count (signed byte); to the home club's manager and to the away club's
/// (`exe::news_send_to_club_manager`).
/// Original: `MANAGER_MANAGER::post_match_rescheduled_news` (0x6aa7d0, manager_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn post_match_rescheduled_news(_this: usize, fixture: usize, club: u32, count: u32) {
    let fixture = Fixture(fixture);
    if fixture.is_null() || before_news_start_date() {
        return;
    }
    covered!("sx14b_1 rescheduled news");
    let mut local = [0u32; 0x3a]; // the local news item (0xe8 bytes)
    let n = local.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0x29, 0);
    exe::newsitem_set_slot(n, 0, read_u32(fixture.home_club().addr()));
    exe::newsitem_set_slot(n, 1, read_u32(fixture.away_club().addr()));
    exe::newsitem_set_slot(n, 2, read_u32(fixture.competition().addr()));
    exe::newsitem_set_slot(n, 3, club);
    exe::newsitem_set_slot(n, 4, read_u32(fixture.competition().nation().addr()));
    exe::newsitem_set_slot(n, 5, count as i8 as i32 as u32);
    exe::news_send_to_club_manager(read_usize(G_NEWS), n, fixture.home_club());
    exe::news_send_to_club_manager(read_usize(G_NEWS), n, fixture.away_club());
}

/// Comparer for `post_match_rescheduled_news`: the news goes to the managers' inboxes (allocates), so only the original
/// runs.
pub(crate) unsafe extern "thiscall" fn compare_post_match_rescheduled_news(this: usize, fixture: usize, club: u32, count: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32)>(IDX_POST_MATCH_RESCHEDULED_NEWS)(this, fixture, club, count)
}

// ---------- sx04c_17: the old Finnish awards: the object, its eleven awards and the season's end (award/old_finland_awards.cpp) ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\award\old_finland_awards.cpp" (the error boxes).
const OLD_FINLAND_AWARDS_CPP: usize = 0xa6a268;
/// `OldFinlandAwards::init_awards(file)` (0x79d680, thiscall, ret 4), through its hook.
const FN_OLD_FINLAND_AWARDS_INIT: usize = 0x79d680;

/// The old Finnish awards' class: the vtable 0x96e180, Finland (`KEY_NATION_FINLAND`), nation id 8, type 1, with month
/// ratings (the error box at line 0x4d when they cannot be built).
const OLD_FINLAND_AWARDS: NationAwardsClass = NationAwardsClass { vtable: 0x96e180, nation_key: Some(KEY_NATION_FINLAND), nation_id: 8, kind: 1,
    month_ratings: Some((OLD_FINLAND_AWARDS_CPP, 0x4d)) };

/// Builds the old Finnish awards (`nation_awards_construct` with `OLD_FINLAND_AWARDS`): the saved awards with a file,
/// else the month ratings and `OldFinlandAwards::init_awards(file)`. Returns this. The award manager builds this class
/// instead of the current Finnish one only with a database whose newer award keys are missing (−1).
/// Original: `OldFinlandAwards::OldFinlandAwards` (0x79d540, old_finland_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn old_finland_awards_construct(this: usize, file: usize) -> usize {
    nation_awards_construct(this, &OLD_FINLAND_AWARDS, file, core::mem::transmute(FN_OLD_FINLAND_AWARDS_INIT))
}

/// Builds the eleven old Finnish awards (`nation_awards_build`, the error box at line 0x75), kind 0, no scope: the
/// Finnish player (national and world reputations 3000/1500), young player (1500/1000), goalkeeper (1000/500) and
/// manager of the year (2000/1000, not for players), no competition; the Premier Division top goalscorer (2500/1250,
/// option 1), player of the month (a month award from May to October of the current year, 1000/500), team of the year
/// and team of the week (team awards, 0/0, span 0, mode 0); the First Division top goalscorer (1200/600), team of the
/// year and team of the week.
/// Original: `OldFinlandAwards::init_awards` (0x79d680, old_finland_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn old_finland_awards_init(this: usize, file: usize) {
    let y = read_u16(G_CURRENT_DATE_YEAR) as u32;
    let n = read_u32(this + nation_awards_fields::NATION_ID);
    let f = file as u32;
    let k = |key: usize| read_u32(key);
    let (d1, d2) = (k(KEY_CLUB_COMP_FINNISH_PREMIER_DIVISION), k(KEY_CLUB_COMP_FINNISH_FIRST_DIVISION));
    let year = |i: u32, key: usize, national: u32, world: u32, comp: u32, option: u32, players: u32| AwardSpec::Year([n, i, k(key), national, world, comp, 0, 0, option, f, players]);
    let team = |i: u32, key: usize, comp: u32| AwardSpec::Team([n, i, k(key), 0, 0, comp, 0, 0, f, 1]);
    let specs = [
        year(0, KEY_STAFF_COMP_FINNISH_PLAYER_OF_THE_YEAR, 3000, 1500, u32::MAX, 0, 1),
        year(1, KEY_STAFF_COMP_FINNISH_YOUNG_PLAYER_OF_THE_YEAR, 1500, 1000, u32::MAX, 0, 1),
        year(2, KEY_STAFF_COMP_FINNISH_GOALKEEPER_OF_THE_YEAR, 1000, 500, u32::MAX, 0, 1),
        year(3, KEY_STAFF_COMP_FINNISH_MANAGER_OF_THE_YEAR, 2000, 1000, u32::MAX, 0, 0),
        year(4, KEY_STAFF_COMP_FINNISH_PREMIER_DIVISION_TOP_GOALSCORER, 2500, 1250, d1, 1, 1),
        AwardSpec::Month([n, 5, k(KEY_STAFF_COMP_FINNISH_PREMIER_DIVISION_PLAYER_OF_THE_M), 1000, 500, d1, 4, y, 9, y, f, 1]),
        team(6, KEY_STAFF_COMP_FINNISH_PREMIER_DIVISION_TEAM_OF_THE_YEA, d1),
        team(7, KEY_STAFF_COMP_FINNISH_PREMIER_DIVISION_TEAM_OF_THE_WEE, d1),
        year(8, KEY_STAFF_COMP_FINNISH_FIRST_DIVISION_TOP_GOALSCORER, 1200, 600, d2, 1, 1),
        team(9, KEY_STAFF_COMP_FINNISH_FIRST_DIVISION_TEAM_OF_THE_YEAR, d2),
        team(10, KEY_STAFF_COMP_FINNISH_FIRST_DIVISION_TEAM_OF_THE_WEEK, d2),
    ];
    covered!("sx04c_17 old finnish awards built");
    nation_awards_build(this, &specs, OLD_FINLAND_AWARDS_CPP, 0x75);
}

/// The old Finnish awards' day: on 15 November (`awards_day_is`), the Finnish player, young player (up to 21) and
/// goalkeeper (position mask 1) of the year from the year ratings' top three (`award_top_three_from_ratings`, with the
/// filters as the original passes them), the manager of the year (from the Premier Division), the Premier Division top
/// goalscorer (statistic 2) and team of the season, and the First Division's. Every day, on the nation's season end day
/// (`is_season_end_day`), the player of the month award moves on (with today's year); then the base daily update.
/// Original: `OldFinlandAwards::daily_update` (0x79db30, old_finland_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn old_finland_awards_daily_update(this: usize, today: usize) {
    if awards_day_is(today, 0xf, 0xa) {
        covered!("sx04c_17 old finnish awards day");
        let fin = read_i32(KEY_NATION_FINLAND);
        award_top_three_from_ratings(this, today, nation_award(this, 0), [-1, -1, -1, fin, -1, 0, 0x11]);
        award_top_three_from_ratings(this, today, nation_award(this, 1), [fin, fin, -1, -1, 0x15, 0, 0x11]);
        award_top_three_from_ratings(this, today, nation_award(this, 2), [-1, fin, -1, -1, -1, 1, 0x11]);
        let (d1, d2) = (KEY_CLUB_COMP_FINNISH_PREMIER_DIVISION, KEY_CLUB_COMP_FINNISH_FIRST_DIVISION);
        award_manager_of_year(this, today, nation_award(this, 3), d1);
        award_player_of_year(this, d1, today, nation_award(this, 4), [-1, 0, 2, -1, -1, -1, -1]);
        award_team_of_season(this, d1, nation_award(this, 6));
        award_player_of_year(this, d2, today, nation_award(this, 8), [-1, 0, 2, -1, -1, -1, -1]);
        award_team_of_season(this, d2, nation_award(this, 9));
    }
    if is_season_end_day(read_usize(this + nation_awards_fields::NATION), today) {
        covered!("sx04c_17 month award moves on");
        month_award_start_new_season(nation_award(this, 5), read_u16(today + cm_date::YEAR));
    }
    nation_awards_base_daily_update(this, today);
}

pub(crate) unsafe extern "thiscall" fn compare_old_finland_awards_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_OLD_FINLAND_AWARDS_CONSTRUCT)(this, file);
    compare_nation_awards_built(r, this, file, &OLD_FINLAND_AWARDS, 11, "old_finland_awards_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_old_finland_awards_init(this: usize, file: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_OLD_FINLAND_AWARDS_INIT);
    compare_nation_awards_init(this, || o(this, file), "old_finland_awards_init");
}

/// Comparer for `old_finland_awards_daily_update`: the awards cannot be given twice; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_old_finland_awards_daily_update(this: usize, today: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_OLD_FINLAND_AWARDS_DAILY_UPDATE)(this, today)
}

// ---------- sx04c_18: the old Finnish player of the month (award/old_finland_awards.cpp) ----------

/// The old Finnish month: the Premier Division player of the month (award 5) from the nation's month ratings
/// (`award_month_player`), with no age limit.
/// Original: `OldFinlandAwards::on_month_change` (0x79dcb0, old_finland_awards.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn old_finland_awards_on_month_change(this: usize) {
    covered!("sx04c_18 old finnish player of the month");
    award_month_player(this, nation_award(this, 5), -1);
}

/// Comparer for `old_finland_awards_on_month_change`: the award cannot be given twice; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_old_finland_awards_on_month_change(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_OLD_FINLAND_AWARDS_ON_MONTH_CHANGE)(this)
}

// ---------- sx04c_19: the old French awards: the object, its fourteen awards and the season's end (award/old_france_awards.cpp) ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\award\old_france_awards.cpp" (the error box).
const OLD_FRANCE_AWARDS_CPP: usize = 0xa6a2a4;
/// `OldFranceAwards::init_awards(file)` (0x79dd80, thiscall, ret 4), through its hook.
const FN_OLD_FRANCE_AWARDS_INIT: usize = 0x79dd80;

/// The old French awards' class: the vtable 0x96e194, France (`KEY_NATION_FRANCE`), nation id 9, type 1, no month
/// ratings.
const OLD_FRANCE_AWARDS: NationAwardsClass = NationAwardsClass { vtable: 0x96e194, nation_key: Some(KEY_NATION_FRANCE), nation_id: 9, kind: 1, month_ratings: None };

/// Builds the old French awards (`nation_awards_construct` with `OLD_FRANCE_AWARDS`): the saved awards with a file, else
/// `OldFranceAwards::init_awards(file)`. Returns this. The award manager builds this class instead of the current
/// French one only with a database whose newer award keys are missing (−1).
/// Original: `OldFranceAwards::OldFranceAwards` (0x79dcf0, old_france_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn old_france_awards_construct(this: usize, file: usize) -> usize {
    nation_awards_construct(this, &OLD_FRANCE_AWARDS, file, core::mem::transmute(FN_OLD_FRANCE_AWARDS_INIT))
}

/// Builds the fourteen old French awards (`nation_awards_build`, the error box at line 0x6b), no scope: with no
/// competition, the French manager (national and world reputations 6000/2500, kind 1, not for players), player
/// (7000/4500), foreign player (6500/3500), under-23 player (5000/2000, kind 1), most popular (5000/1500) and most
/// unpopular player of the year (0/0), the others kind 0; then, kind 1, for each division the team of the week (team
/// award, 0/0, span 1), the players' player of the year (First Division 6000/2000, Second 4500/1000), the player of the
/// year (6000/2500, 4500/1500) and the goalkeeper of the year (5500/1500, 4000/500).
/// Original: `OldFranceAwards::init_awards` (0x79dd80, old_france_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn old_france_awards_init(this: usize, file: usize) {
    let n = read_u32(this + nation_awards_fields::NATION_ID);
    let f = file as u32;
    let k = |key: usize| read_u32(key);
    let (d1, d2) = (k(KEY_CLUB_COMP_FRENCH_FIRST_DIVISION), k(KEY_CLUB_COMP_FRENCH_SECOND_DIVISION));
    let year = |i: u32, key: usize, national: u32, world: u32, comp: u32, kind: u32, players: u32| AwardSpec::Year([n, i, k(key), national, world, comp, kind, 0, 0, f, players]);
    let team = |i: u32, key: usize, comp: u32| AwardSpec::Team([n, i, k(key), 0, 0, comp, 1, 0, f, 1]);
    let none = u32::MAX;
    let specs = [
        year(0, KEY_STAFF_COMP_FRENCH_MANAGER_OF_THE_YEAR, 6000, 2500, none, 1, 0),
        year(1, KEY_STAFF_COMP_FRENCH_PLAYER_OF_THE_YEAR, 7000, 4500, none, 0, 1),
        year(2, KEY_STAFF_COMP_FRENCH_FOREIGN_PLAYER_OF_THE_YEAR, 6500, 3500, none, 0, 1),
        year(3, KEY_STAFF_COMP_FRENCH_UNDER_23_PLAYER_OF_THE_YEAR, 5000, 2000, none, 1, 1),
        year(4, KEY_STAFF_COMP_FRENCH_MOST_POPULAR_PLAYER_OF_THE_YEAR, 5000, 1500, none, 0, 1),
        year(5, KEY_STAFF_COMP_FRENCH_MOST_UNPOPULAR_PLAYER_OF_THE_YEAR, 0, 0, none, 0, 1),
        team(6, KEY_STAFF_COMP_FRENCH_FIRST_DIVISION_TEAM_OF_THE_WEEK, d1),
        year(7, KEY_STAFF_COMP_FRENCH_PLAYERS_FIRST_DIVISION_PLAYER_OF, 6000, 2000, d1, 1, 1),
        year(8, KEY_STAFF_COMP_FRENCH_FIRST_DIVISION_PLAYER_OF_THE_YEAR, 6000, 2500, d1, 1, 1),
        year(9, KEY_STAFF_COMP_FRENCH_FIRST_DIVISION_GOALKEEPER_OF_THE, 5500, 1500, d1, 1, 1),
        team(10, KEY_STAFF_COMP_FRENCH_SECOND_DIVISION_TEAM_OF_THE_WEEK, d2),
        year(11, KEY_STAFF_COMP_FRENCH_PLAYERS_SECOND_DIVISION_PLAYER_OF, 4500, 1000, d2, 1, 1),
        year(12, KEY_STAFF_COMP_FRENCH_SECOND_DIVISION_PLAYER_OF_THE_YEA, 4500, 1500, d2, 1, 1),
        year(13, KEY_STAFF_COMP_FRENCH_SECOND_DIVISION_GOALKEEPER_OF_THE, 4000, 500, d2, 1, 1),
    ];
    covered!("sx04c_19 old french awards built");
    nation_awards_build(this, &specs, OLD_FRANCE_AWARDS_CPP, 0x6b);
}

/// The old French awards' days: on 20 December (`awards_day_is`), the French player, foreign player, most popular and
/// most unpopular player of the year from the year ratings (`award_player_of_year_from_ratings`, with the filters as
/// the original passes them: French players; nationality other than French; modes 3 and 4). On 6 June, the manager of
/// the year (from the First Division); the under-23 player (up to 23), the players' player (statistic 0x11), the
/// goalkeeper (positions 1) and the player of the year (positions 0x7e, statistic 2) of the First Division from its
/// stats; and the Second Division's players' player, goalkeeper and player of the year. Every day, the base daily
/// update.
/// Original: `OldFranceAwards::daily_update` (0x79e330, old_france_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn old_france_awards_daily_update(this: usize, today: usize) {
    if awards_day_is(today, 0x14, 0xb) {
        covered!("sx04c_19 old french december awards");
        let fra = read_i32(KEY_NATION_FRANCE);
        for (k, filters) in [(1, [fra, -1, -1, -1, -1, 0, -1]), (2, [-1, fra, fra, -1, -1, 0, -1]), (4, [-1, fra, -1, -1, -1, 3, -1]),
            (5, [-1, fra, -1, -1, -1, 4, -1])] {
            award_player_of_year_from_ratings(this, today, nation_award(this, k), filters);
        }
    }
    if awards_day_is(today, 6, 5) {
        covered!("sx04c_19 old french june awards");
        let (d1, d2) = (KEY_CLUB_COMP_FRENCH_FIRST_DIVISION, KEY_CLUB_COMP_FRENCH_SECOND_DIVISION);
        award_manager_of_year(this, today, nation_award(this, 0), d1);
        for (key, k, age, positions, stat) in [(d1, 3, 0x17, 0, 0x11), (d1, 7, -1, 0, 0x11), (d1, 9, -1, 1, 0x11), (d1, 8, -1, 0x7e, 2),
            (d2, 11, -1, 0, 0x11), (d2, 13, -1, 1, 0x11), (d2, 12, -1, 0x7e, 2)] {
            award_player_of_year(this, key, today, nation_award(this, k), [age, positions, stat, -1, -1, -1, -1]);
        }
    }
    nation_awards_base_daily_update(this, today);
}

pub(crate) unsafe extern "thiscall" fn compare_old_france_awards_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_OLD_FRANCE_AWARDS_CONSTRUCT)(this, file);
    compare_nation_awards_built(r, this, file, &OLD_FRANCE_AWARDS, 14, "old_france_awards_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_old_france_awards_init(this: usize, file: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_OLD_FRANCE_AWARDS_INIT);
    compare_nation_awards_init(this, || o(this, file), "old_france_awards_init");
}

/// Comparer for `old_france_awards_daily_update`: the awards cannot be given twice; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_old_france_awards_daily_update(this: usize, today: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_OLD_FRANCE_AWARDS_DAILY_UPDATE)(this, today)
}

// ---------- sx04c_22: the old Irish managers of the month (award/old_ireland_awards.cpp) ----------

/// The old Irish month: the manager of the month (`award_month_manager`) of the Premier Division (award 9) and of the
/// First Division (award 16).
/// Original: `OldIrelandAwards::on_month_change` (0x79f490, old_ireland_awards.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn old_ireland_awards_on_month_change(this: usize) {
    covered!("sx04c_22 old irish managers of the month");
    award_month_manager(nation_award(this, 9), KEY_CLUB_COMP_IRISH_PREMIER_DIVISION);
    award_month_manager(nation_award(this, 16), KEY_CLUB_COMP_IRISH_FIRST_DIVISION);
}

/// Comparer for `old_ireland_awards_on_month_change`: the awards cannot be given twice; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_old_ireland_awards_on_month_change(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_OLD_IRELAND_AWARDS_ON_MONTH_CHANGE)(this)
}

// --- sx16_1: player_regen_regenerate_as_non_player (0x7ab340, player_regen.cpp) ---

/// Exe routines the non-player regeneration calls by address (Rust hooks among them, called through the exe so the
/// comparison runs their comparers).
pub(crate) mod regen_non_player_fns {
    pub(crate) const REWIND_STAFF_HISTORY: usize = 0x86fb80; // Staff_Records::rewind_staff_history, thiscall(this, person), ret 4
    pub(crate) const REMOVE_NICKNAME_FROM_ALL: usize = 0x5fff30; // HUMAN_MANAGER::remove_nickname_from_all, thiscall(this, person), ret 4
    pub(crate) const REMOVE_TALKS_RECORDS: usize = 0x4ca1c0; // CONTRACT_MANAGER::remove_talks_records, thiscall(this, person, manager, club), ret 0xc
    pub(crate) const STAFF_CAN_TAKE_JOB: usize = 0x4cb990; // staff_can_take_job, stdcall(job, person), ret 8
}

/// The age band of a regenerated non-player by his job (the job byte minus 1 indexes the original's jump table 0x7aba98):
/// (random range, base). Chairman (1) is handled apart; the manager (5), the scout (9) and anything else fall to the
/// default (15, 0x27).
const NON_PLAYER_AGE_BANDS: [(i32, i8); 10] = [
    (0x14, 0x24), // 1 chairman (the attributes by the club, see regen_chairman_attributes)
    (0x19, 0x28), // 2 managing director
    (0x19, 0x28), // 3 general manager
    (0xa, 0x25),  // 4 director of football
    (0xf, 0x27),  // 5 manager (default)
    (0xf, 0x23),  // 6 assistant manager
    (0xf, 0x23),  // 7 reserve team manager
    (0xf, 0x20),  // 8 coach
    (0xf, 0x27),  // 9 scout (default)
    (0x19, 0x23), // 10 physio
];

/// The formation preference of a regenerated non-player from a roll of 0..124: the first band the roll is below.
const NON_PLAYER_FORMATION_BANDS: [i32; 9] = [0x22, 0x28, 0x3c, 0x5a, 0x5e, 0x60, 0x6c, 0x6f, 0x79];

/// The business attributes of a regenerated chairman by his club's reputation (`club::REPUTATION`, signed):
/// - above 6250 (0x186a) with `club::PLC` set: business 10..20, directness, discipline and interference 1..20, man
///   handling (0..99) − (0..99), patience and motivating 1..20, resources 6..20;
/// - above 6250 without it: business 1..20, the same middle, resources 10..20;
/// - above 3250 (0xcb2): business 1..20, the same middle, patience 5..20 in 4 of 5 (else 1..20), motivating 1..20,
///   resources 1..15 in 9 of 10 (else 6..20);
/// - otherwise: business 1..20, the same middle, patience 6..20 in 9 of 10 (else 1..20), motivating (0..99) −
///   (0..99), resources 6..20 in 99 of 100 (else 1..10).
/// The rolls go in this order, one `utils_random` each.
unsafe fn regen_chairman_attributes(np: NonPlayer, club: Club) {
    let r = |n: i32| friendly_random(n) as u8;
    let reputation = club.reputation();
    let top = reputation > 0x186a;
    let plc = top && club.plc() != 0;
    np.set_business(if plc { r(0xb).wrapping_add(0xa) } else { r(0x14).wrapping_add(1) });
    np.set_directness(r(0x14).wrapping_add(1) as i8);
    np.set_discipline(r(0x14).wrapping_add(1));
    np.set_interference(r(0x14).wrapping_add(1) as i8);
    let a = r(0x64);
    np.set_man_handling(a.wrapping_sub(r(0x64)));
    if top {
        covered!("sx16_1 chairman of a big club");
        np.set_patience(r(0x14).wrapping_add(1));
        np.set_motivating(r(0x14).wrapping_add(1));
        np.set_resources((if plc { r(0xf).wrapping_add(6) } else { r(0xb).wrapping_add(0xa) }) as i8);
    } else if reputation > 0xcb2 {
        covered!("sx16_1 chairman of a middle club");
        np.set_patience(if r(5) != 0 { r(0x10).wrapping_add(5) } else { r(0x14).wrapping_add(1) });
        np.set_motivating(r(0x14).wrapping_add(1));
        np.set_resources((if r(0xa) != 0 { r(0xf).wrapping_add(1) } else { r(0xf).wrapping_add(6) }) as i8);
    } else {
        covered!("sx16_1 chairman of a small club");
        np.set_patience(if r(0xa) != 0 { r(0xf).wrapping_add(6) } else { r(0x14).wrapping_add(1) });
        let a = r(0x64);
        np.set_motivating(a.wrapping_sub(r(0x64)));
        np.set_resources((if r(0x64) != 0 { r(0xf).wrapping_add(6) } else { r(0xa).wrapping_add(1) }) as i8);
    }
}

/// The languages of a regenerated person: every language removed (1..=64), then each native language whose points
/// (`player_stats_get_native_language_points`) are nonzero and at least (20 − adaptability) × 10 added back with the
/// fluency (points − that base) / 75, at most 10. The talks records go between the two loops, as in the original.
unsafe fn regen_non_player_languages(p: Staff) {
    let stats = read_usize(G_PLAYER_STATS);
    let base = 0x14i16.wrapping_sub(p.adaptability() as i16).wrapping_mul(10);
    for k in 1..=0x40i8 {
        let v = exe::player_stats_get_native_language_points(p, k);
        if v != 0 && v >= base {
            let q = (v as i32 - base as i32) / 0x4b;
            exe::playerstats_add_language(stats, p, k as u8 as u32, q.min(0xa) as u32, 1);
        }
    }
}

/// The new contract of a regenerated non-player who keeps a club job: when he can take the job
/// (`staff_can_take_job(job_for_club, 0)`), the old contract goes (`CONTRACT_MANAGER::remove_contract`), the wage is 0,
/// the expiry date is day 1..28 of the club's season-end month (`contract_manager_season_end_month`) in the year now +
/// 1..6 (rolled in that order: the year, the month, the day) and, with a contract record, a fresh initial contract
/// (`STAFF_CONTRACT::generate_initial_contract(contract, person, 0)`).
unsafe fn regen_non_player_contract(p: Staff, club: Club) {
    let can_take: unsafe extern "stdcall" fn(u32, usize) -> u32 = core::mem::transmute(regen_non_player_fns::STAFF_CAN_TAKE_JOB);
    if can_take(p.job_for_club() as u32, 0) == 0 {
        return;
    }
    covered!("sx16_1 new contract");
    let manager = read_usize(G_CONTRACT_MANAGER);
    exe::contract_manager_remove_contract(manager, p);
    p.set_wage(0);
    let years = friendly_random(6);
    let month = exe::contract_manager_season_end_month(manager, club);
    let day = (friendly_random(0x1c) as i8).wrapping_add(1) as i16;
    let year = (read_i16(G_CURRENT_DATE_YEAR) as i32 + years + 1) as u16;
    let mut expiry = [0u16; 4];
    let mut scratch = [0u16; 4];
    let date = exe::date_date_549f70(&mut expiry, day as u16 as u32, month as u32, year as u32, u32::MAX);
    exe::date_operator_3d(p.date_expires_club_addr(), scratch.as_mut_ptr() as usize, date);
    let contract = exe::contract_manager_get_contract(manager, p);
    if contract != 0 {
        exe::staff_contract_generate_initial_contract(contract, p, 0);
    }
}

/// Rebuilds a retired staff slot as a new non-player (the regeneration after a retirement, from `process_retirements`).
///
/// The retirement bits (0x30 of the state's `staff_state::STATUS_FLAGS`) cleared; `CLASSIFICATION` 1 (non-player);
/// the staff history rewound (`Staff_Records::rewind_staff_history`); the caps and international goals 0; the low
/// nibble of the state flags and the state bytes +8 and +9 set to 0 and 0x9c (156). New first, second and common names
/// for his nation; his nickname removed from every human (`HUMAN_MANAGER::remove_nickname_from_all`); joined the club
/// today. Every language removed, the talks records removed (`remove_talks_records(person, 0, 0)`) and the native
/// languages added back (`regen_non_player_languages`).
///
/// The non-player sheet (`staff::NON_PLAYER`): current ability 1 + roll(potential × 0.75), raised to potential × 0.5
/// when below; home reputation roll(750) + ability × 25 + 1, world reputation roll(200) + ability × 15 + 1 (a home
/// below the world becomes world + 10 and the world 1); current reputation roll(50) + home. The age by the job
/// (`NON_PLAYER_AGE_BANDS`, the club job, else the national job): a chairman 36..55 with the business attributes by his
/// club (`regen_chairman_attributes`, when he has a club and a sheet). Birth year this year − age, birth day roll(365),
/// the age recomputed (`database_get_staff_age`), the formation preference by a roll of 125
/// (`NON_PLAYER_FORMATION_BANDS`). With a club and a sheet, the new contract (`regen_non_player_contract`). Last, the
/// attributes normalised (`staff_normalise_non_player_attributes`).
///
/// Defects of the original: without a club job or a national job the job byte is the argument slot's low byte, which
/// the language loops left at 0x41, so every jobless person gets the default band (manager's 39..53); the sheet
/// (`NON_PLAYER`) is written without a null test (only the chairman and the contract test it).
/// Original: `player_regen_regenerate_as_non_player` (0x7ab340, player_regen.cpp, thiscall, ret 4; `this` unused).
pub(crate) unsafe extern "thiscall" fn regenerate_as_non_player(_this: usize, person: usize) {
    let p = Staff(person);
    covered!("sx16_1 regenerate as non-player");
    *staff_state_byte(p, staff_state::STATUS_FLAGS) &= 0xcf;
    p.set_classification(1);
    call_thiscall_1(regen_non_player_fns::REWIND_STAFF_HISTORY, read_usize(G_STAFF_RECORDS), p);
    p.set_int_apps(0);
    p.set_int_goals(0);
    *staff_state_byte(p, staff_state::STATUS_FLAGS) &= 0xf0;
    *staff_state_byte(p, 8) = 0; // state byte +8 (not named yet)
    *staff_state_byte(p, 9) = 0x9c; // state byte +9 (not named yet)
    p.set_first_name(exe::database_pick_first_name_for_nation(p.nation()));
    p.set_second_name(exe::database_pick_second_name_for_nation(p.nation()));
    p.set_common_name(exe::database_get_empty_common_name(p.nation()));
    call_thiscall_1(regen_non_player_fns::REMOVE_NICKNAME_FROM_ALL, read_usize(G_HUMAN_MANAGER), p);
    p.set_date_joined_club_year(read_i16(G_CURRENT_DATE_YEAR) as u16);
    write_i16(p.addr() + staff::DATE_JOINED_CLUB, read_i16(G_CURRENT_DATE));
    for k in 1..=0x40i8 {
        exe::playerstats_remove_language(read_usize(G_PLAYER_STATS), p, k);
    }
    let remove_talks: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(regen_non_player_fns::REMOVE_TALKS_RECORDS);
    remove_talks(read_usize(G_CONTRACT_MANAGER), p.addr(), 0, 0);
    regen_non_player_languages(p);

    let np = p.non_player();
    let potential = np.potential_ability() as i16 as f64;
    np.set_current_ability((friendly_random(ftol(potential * read_f64(rdata::F64_0_75))) + 1) as i16);
    let half = potential * read_f32(rdata::F32_0_5) as f64;
    if (np.current_ability() as f64) < half {
        np.set_current_ability(ftol(half) as i16);
    }
    let r = friendly_random(0x2ee) as i16;
    np.set_home_reputation(r.wrapping_add(np.current_ability().wrapping_mul(0x19)).wrapping_add(1)); // sx16_1: home reputation
    let r = friendly_random(0xc8) as i16;
    np.set_world_reputation(r.wrapping_add(np.current_ability().wrapping_mul(0xf)).wrapping_add(1));
    if np.home_reputation() < np.world_reputation() {
        np.set_home_reputation(np.world_reputation().wrapping_add(0xa));
        np.set_world_reputation(1);
    }
    let r = friendly_random(0x32) as i16;
    np.set_current_reputation(r.wrapping_add(np.home_reputation()));

    let club = p.club_job();
    let job = if !club.is_null() {
        p.job_for_club()
    } else if !p.national_job().is_null() {
        p.job_for_nation()
    } else {
        covered!("sx16_1 jobless");
        0x41 // the original reads the argument slot the language loops left at 0x41
    };
    let (range, base) = NON_PLAYER_AGE_BANDS.get((job as i8 as i32 - 1) as usize).copied().unwrap_or((0xf, 0x27));
    let age = (friendly_random(range) as i8).wrapping_add(base); // sx16_1: the age by the job
    if job == job_kind::CHAIRMAN && !club.is_null() && !np.is_null() {
        regen_chairman_attributes(np, club);
    }
    p.set_date_of_birth_year(read_i16(G_CURRENT_DATE_YEAR).wrapping_sub(age as i16) as u16);
    p.set_date_of_birth(friendly_random(0x16d) as i16);
    p.set_age(exe::database_get_staff_age(p, G_CURRENT_DATE as *const u16) as i16);
    let roll = friendly_random(0x7d);
    let formation = NON_PLAYER_FORMATION_BANDS.iter().position(|&b| roll < b).unwrap_or(9);
    np.set_formation(formation as i8);
    if !club.is_null() && !np.is_null() {
        regen_non_player_contract(p, club);
    }
    exe::staff_normalise_non_player_attributes(p);
}

/// Comparer for `regenerate_as_non_player`: it rolls the generator, rewrites the person and calls routines with side
/// effects (contracts, staff history, nicknames), so it cannot run twice; only the original runs. The copy is proven by
/// the Rust careers and the mutations.
pub(crate) unsafe extern "thiscall" fn compare_regenerate_as_non_player(this: usize, person: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_REGENERATE_AS_NON_PLAYER)(this, person)
}

// --- sx16_6: the machine checks (os.cpp: physical and free memory, CPU speed class, free disk space) ---

/// The Windows functions os.cpp calls, read from the exe's import table.
pub(crate) mod os_imports {
    pub(crate) const GET_DISK_FREE_SPACE_A: usize = 0x9670b0;
    pub(crate) const LOAD_LIBRARY_A: usize = 0x9670b4;
    pub(crate) const GET_PROC_ADDRESS: usize = 0x9670b8;
    pub(crate) const FREE_LIBRARY: usize = 0x9670bc;
    pub(crate) const GET_SYSTEM_INFO: usize = 0x9670c0;
    pub(crate) const GLOBAL_MEMORY_STATUS: usize = 0x9670c4;
    pub(crate) const GET_VERSION_EX_A: usize = 0x9670c8;
}

/// The source path of os.cpp in the "Error" box ("E:\dev\CM3\cm3 00-01\si\code\os.cpp").
const OS_CPP_PATH: usize = 0xa6a39c;
/// What the machine checks answer when they cannot measure: 150 (MB of disk, MHz of CPU).
const OS_UNKNOWN: u32 = 0x96;

/// `GlobalMemoryStatus` into a MEMORYSTATUS (32 bytes; only the length is set before the call, as in the original).
unsafe fn os_memory_status() -> [u32; 8] {
    let status: unsafe extern "system" fn(*mut u32) = core::mem::transmute(read_usize(os_imports::GLOBAL_MEMORY_STATUS));
    let mut m = [0u32; 8];
    m[0] = 0x20;
    status(m.as_mut_ptr());
    m
}

/// The total physical memory in MB: `dwTotalPhys` as a signed number divided by 2^20 (rounded toward zero).
/// Original: `os_get_total_physical_memory_mb` (0x7a16d0, os.cpp, cdecl).
pub(crate) unsafe extern "C" fn os_get_total_physical_memory_mb() -> i32 {
    os_memory_status()[2] as i32 / 0x100000
}

/// Comparer for `os_get_total_physical_memory_mb`: the same value.
pub(crate) unsafe extern "C" fn compare_os_get_total_physical_memory_mb() -> i32 {
    let r = original_fn::<unsafe extern "C" fn() -> i32>(IDX_OS_GET_TOTAL_PHYSICAL_MEMORY_MB)();
    let n = os_get_total_physical_memory_mb();
    SCOREBOARD.record(r == n, || format!("os_get_total_physical_memory_mb: original={} rust={}", r, n));
    r
}

/// The free memory in MB for the game's checks: `dwAvailPhys` + `dwAvailPageFile` (32-bit sum) shifted by 20.
/// Original: `os_get_free_memory_mb` (0x7a1710, os.cpp, cdecl).
pub(crate) unsafe extern "C" fn os_get_free_memory_mb() -> u32 {
    let m = os_memory_status();
    m[3].wrapping_add(m[5]) >> 20
}

/// The comparer of a machine reading that may move between two calls (free memory, free disk): the original, the copy
/// and the original again; the copy must give one of the two original answers.
unsafe fn compare_os_reading<R: PartialEq + Copy + core::fmt::Debug>(name: &str, original: impl Fn() -> R, rust: impl Fn() -> R) -> R {
    let r = original();
    let n = rust();
    let again = if n == r { r } else { original() };
    SCOREBOARD.record(n == r || n == again, || format!("{}: original={:?}/{:?} rust={:?}", name, r, again, n));
    r
}

/// Comparer for `os_get_free_memory_mb` (the reading may move between the calls).
pub(crate) unsafe extern "C" fn compare_os_get_free_memory_mb() -> u32 {
    let o = original_fn::<unsafe extern "C" fn() -> u32>(IDX_OS_GET_FREE_MEMORY_MB);
    compare_os_reading("os_get_free_memory_mb", || o(), || os_get_free_memory_mb())
}

/// The CPU speed class of the machine for the default settings: 0 fast, 1 middle, 2 slow.
///
/// The speed in MHz from `cpuinf32.dll` in CM3_DATA (`utils_build_file_path`, the path copied into the shared buffer
/// 0xde1f64 with "%s", `LoadLibraryA`, its `cpuspeed(&scratch, 0)`, then `FreeLibrary`): the third dword of the
/// answer, or 150 when the first two are 0 or the library does not load. Then `GetSystemInfo`: a processor other than
/// a 586 (386, 486, anything else) is slow (2); a 586 above 300 MHz is fast (0), up to 200 slow (2), between them 1
/// (the speed compared as a signed word).
/// Defect of the original: `GetProcAddress` is not checked (a library without `cpuspeed` crashes the game).
/// Original: `os_get_cpu_speed_class` (0x7a1750, os.cpp, cdecl, the answer in al).
pub(crate) unsafe extern "C" fn os_get_cpu_speed_class() -> u8 {
    let load: unsafe extern "system" fn(usize) -> usize = core::mem::transmute(read_usize(os_imports::LOAD_LIBRARY_A));
    let proc_address: unsafe extern "system" fn(usize, usize) -> usize = core::mem::transmute(read_usize(os_imports::GET_PROC_ADDRESS));
    let free: unsafe extern "system" fn(usize) -> u32 = core::mem::transmute(read_usize(os_imports::FREE_LIBRARY));
    let system_info: unsafe extern "system" fn(*mut u32) = core::mem::transmute(read_usize(os_imports::GET_SYSTEM_INFO));
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let path = exe::utils_build_file_path(0x9b5348usize, 0xa6a3ccusize); // "CM3_DATA", "cpuinf32.dll"
    sprintf(GUI_SCRATCH_TEXT, 0x991944, path); // "%s"
    let library = load(GUI_SCRATCH_TEXT);
    let mut speed = OS_UNKNOWN as i16;
    if library != 0 {
        let cpuspeed: unsafe extern "C" fn(*mut u32, u32) -> *const u32 = core::mem::transmute(proc_address(library, 0xa6a3c0)); // "cpuspeed"
        let mut scratch = [0u32; 8];
        let answer = cpuspeed(scratch.as_mut_ptr(), 0);
        let (first, second) = (*answer, *answer.add(1));
        speed = *answer.add(2) as i16;
        free(library);
        if first == 0 && second == 0 {
            speed = OS_UNKNOWN as i16;
        }
    }
    let mut info = [0u32; 9];
    system_info(info.as_mut_ptr());
    match info[6] {
        0x24a if speed > 0x12c => 0, // a 586 above 300 MHz
        0x24a => 1 + (speed <= 0xc8) as u8,
        _ => 2,
    }
}

/// Comparer for `os_get_cpu_speed_class`: the same class (the speed is measured again; the classes are wide).
pub(crate) unsafe extern "C" fn compare_os_get_cpu_speed_class() -> u8 {
    let o = original_fn::<unsafe extern "C" fn() -> u8>(IDX_OS_GET_CPU_SPEED_CLASS);
    compare_os_reading("os_get_cpu_speed_class", || o(), || os_get_cpu_speed_class())
}

/// The free disk space of a path in MB.
///
/// `GetVersionExA` (148 bytes) failing: 150. Windows NT, or Windows 9x above 4.0 build 1000: `GetDiskFreeSpaceExA`
/// looked up in KERNEL32.DLL (`LoadLibraryA`, `GetProcAddress`); the space free to the caller shifted by 20, or 4096
/// when it is 4 GB or more; the library freed. The old Windows 95: `GetDiskFreeSpaceA` (sectors per cluster × bytes
/// per sector × free clusters, 32-bit, shifted by 20). Any other platform: 0. A failure shows the os.cpp "Error" box
/// (no KERNEL32 line 0x13d, no function 0x14b, the call 0x158) and answers 150; the old call failing answers 150 without
/// a box.
/// Original: `os_get_free_disk_space_mb` (0x7a1820, os.cpp, cdecl).
pub(crate) unsafe extern "C" fn os_get_free_disk_space_mb(path: usize) -> u32 {
    let version_ex: unsafe extern "system" fn(*mut u32) -> u32 = core::mem::transmute(read_usize(os_imports::GET_VERSION_EX_A));
    let mut version = [0u32; 0x25];
    version[0] = 0x94;
    if version_ex(version.as_mut_ptr()) == 0 {
        return OS_UNKNOWN;
    }
    let (major, minor, build, platform) = (version[1], version[2], version[3] & 0xffff, version[4]);
    let old_95 = platform == 1 && (major < 4 || (major == 4 && minor == 0)) && build <= 0x3e8;
    if platform != 2 && platform != 1 {
        return 0;
    }
    if old_95 {
        covered!("sx16_6 old Windows 95 disk space");
        let free_space: unsafe extern "system" fn(usize, *mut u32, *mut u32, *mut u32, *mut u32) -> u32 = core::mem::transmute(read_usize(os_imports::GET_DISK_FREE_SPACE_A));
        let (mut per_cluster, mut per_sector, mut clusters, mut total) = (0u32, 0u32, 0u32, 0u32);
        if free_space(path, &mut per_cluster, &mut per_sector, &mut clusters, &mut total) == 0 {
            return OS_UNKNOWN;
        }
        return per_cluster.wrapping_mul(per_sector).wrapping_mul(clusters) >> 20;
    }
    let load: unsafe extern "system" fn(usize) -> usize = core::mem::transmute(read_usize(os_imports::LOAD_LIBRARY_A));
    let proc_address: unsafe extern "system" fn(usize, usize) -> usize = core::mem::transmute(read_usize(os_imports::GET_PROC_ADDRESS));
    let free: unsafe extern "system" fn(usize) -> u32 = core::mem::transmute(read_usize(os_imports::FREE_LIBRARY));
    let kernel = load(0xa6a3f0); // "KERNEL32.DLL"
    if kernel == 0 {
        show_error_box(OS_CPP_PATH, 0x13d);
        return OS_UNKNOWN;
    }
    let function = proc_address(kernel, 0xa6a3dc); // "GetDiskFreeSpaceExA"
    if function == 0 {
        show_error_box(OS_CPP_PATH, 0x14b);
        free(kernel);
        return OS_UNKNOWN;
    }
    let free_space_ex: unsafe extern "system" fn(usize, *mut u64, *mut u64, *mut u64) -> u32 = core::mem::transmute(function);
    let (mut available, mut total, mut free_total) = (0u64, 0u64, 0u64);
    if free_space_ex(path, &mut available, &mut total, &mut free_total) == 0 {
        show_error_box(OS_CPP_PATH, 0x158);
        free(kernel);
        return OS_UNKNOWN;
    }
    covered!("sx16_6 disk space");
    free(kernel);
    if available >> 32 != 0 { 0x1000 } else { (available as u32) >> 20 }
}

/// Comparer for `os_get_free_disk_space_mb` (the reading may move between the calls; the error boxes only through
/// the original, which shows them first).
pub(crate) unsafe extern "C" fn compare_os_get_free_disk_space_mb(path: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_OS_GET_FREE_DISK_SPACE_MB);
    compare_os_reading("os_get_free_disk_space_mb", || o(path), || os_get_free_disk_space_mb(path))
}

// --- sx16_12: the youth promotion news text (player_regen.cpp 0x7a8eb0) ---

/// The staff verdict sentence on a promoted youngster, by the verdict slot (12) and, for 1, the coach slot (13):
/// (verdict, coach slot 2?, with the verdict flag slot 7 set, without). Anything else uses the last pair.
const YOUTH_VERDICT_TEXTS: [((u32, bool), usize, usize); 3] = [
    ((2, false), 0xa6c3dc, 0xa6c348),
    ((1, true), 0xa6c29c, 0xa6c204),
    ((1, false), 0xa6c160, 0xa6c0cc),
];

/// Writes the headline or the body of the news that a youth player joins the reserves or the B team.
///
/// The slots: 0 the player's id, 1..3 his name ids, 4 the club's id, 6 the B team flag (from the youth academy), 7 the
/// verdict flag, 8 the staff member giving the verdict (−1 none), 9..11 his name ids, 12 the verdict, 13 the coach
/// kind (slots 5 and 10.. not used). The player's name (`database_format_name_from_ids`, 400 bytes; style 5 in the
/// headline, 1 in the body).
///
/// Headline: "<name> promoted from youth academy" (slot 6 set) or "promoted to reserve squad"; the text and the
/// player's link (0x7d7) when he still has those names.
/// Body: the club (`club::SIZE` × slot 4) and `club_get_linked_b_team(club, &scratch, 1)`; the sentence with the
/// club's short gender twice, the name and the club's short name: with slot 6, 0xa6c63c with a linked B team else
/// 0xa6c5a4; without, 0xa6c514 / 0xa6c480. The text set. With a staff member (slot 8 above −1): his name (400 bytes,
/// style 1), the verdict sentence (`YOUTH_VERDICT_TEXTS`) appended (`Text::append_string`) and his link when he still
/// has those names. Then, with a club (slot 4 above −1), the club's link (0x7d5, its name), and the player's link.
/// Defect of the original: the club record is read (slot 4 × 581 from the club table) before the test of slot 4, so a
/// news item without a club reads the record before the table.
/// Original: `player_regen_write_youth_promotion_news_text` (0x7a8eb0, player_regen.cpp, stdcall(news, body, out), ret
/// 0xc; the news writer table calls it as a method of the regen object, ecx unused).
pub(crate) unsafe extern "stdcall" fn player_regen_write_youth_promotion_news_text(news: usize, body: u32, out: usize) {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let slot = |k: u32| exe::newsitem_get_slot(news, k);
    let id = slot(0);
    let names = [slot(1), slot(2), slot(3)];
    let club_id = slot(4) as i32;
    let academy = slot(6);
    let flag = slot(7);
    let staff_id = slot(8) as i32;
    let staff_names = [slot(9), slot(10), slot(11)];
    let verdict = slot(12);
    let coach = slot(13);
    let mut name = [0u8; 0x190];
    let staff_row = |k: u32| read_usize(G_STAFF_TABLE).wrapping_add((k as usize).wrapping_mul(staff::SIZE));
    let link_person = |name: &[u8; 0x190], person: usize, ids: [u32; 3]| {
        if exe::database_staff_matches_names(person, ids[0], ids[1], ids[2], 0usize) != 0 {
            exe::text_add_link(out, name.as_ptr(), 0x7d7, person);
        }
    };
    if body == 0 {
        covered!("sx16_12 youth promotion headline");
        exe::database_format_name_from_ids(names[0], names[1], names[2], name.as_mut_ptr(), 0x190, 5, id);
        format(GUI_SCRATCH_TEXT, if academy != 0 { 0xa6c708 } else { 0xa6c6cc }, name.as_ptr()); // promoted from youth academy / to reserve squad
        exe::text_set_string(out, GUI_SCRATCH_TEXT as *const u8);
        link_person(&name, staff_row(id), names);
        return;
    }
    covered!("sx16_12 youth promotion body");
    exe::database_format_name_from_ids(names[0], names[1], names[2], name.as_mut_ptr(), 0x190, 1, id);
    let club = Club(read_usize(G_CLUB_TABLE).wrapping_add((club_id as isize * club::SIZE as isize) as usize));
    let mut scratch = 0u32;
    let b_team = exe::club_get_linked_b_team(club, &mut scratch, 1) != 0;
    let text = match (academy != 0, b_team) {
        (true, true) => 0xa6c63c,
        (true, false) => 0xa6c5a4,
        (false, true) => 0xa6c514,
        (false, false) => 0xa6c480,
    };
    let g = club.short_gender_name() as i8 as i32;
    format(GUI_SCRATCH_TEXT, text, g, g, name.as_ptr(), club.addr() + club::SHORT_NAME); // sx16_12: the promotion sentence
    exe::text_set_string(out, GUI_SCRATCH_TEXT as *const u8);
    if staff_id > -1 {
        covered!("sx16_12 staff verdict");
        let mut staff_name = [0u8; 0x190];
        exe::database_format_name_from_ids(staff_names[0], staff_names[1], staff_names[2], staff_name.as_mut_ptr(), 0x190, 1, staff_id as u32);
        let key = (verdict, verdict == 1 && coach == 2);
        let (_, with_flag, without) = YOUTH_VERDICT_TEXTS.iter().copied().find(|e| e.0 == key).unwrap_or(((0, false), 0xa6c054, 0xa6bfec));
        format(GUI_SCRATCH_TEXT, if flag != 0 { with_flag } else { without }, staff_name.as_ptr());
        exe::text_append_string(out, GUI_SCRATCH_TEXT);
        link_person(&staff_name, staff_row(staff_id as u32), staff_names);
    }
    if club_id > -1 {
        exe::text_add_link(out, (club.addr() + club::NAME) as *const u8, 0x7d5, club);
    }
    link_person(&name, staff_row(id), names);
}

/// Comparer for `player_regen_write_youth_promotion_news_text`: the text, its links and the generator
/// (`compare_text_writer`).
pub(crate) unsafe extern "stdcall" fn compare_player_regen_write_youth_promotion_news_text(news: usize, body: u32, out: usize) {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, u32, usize)>(IDX_PLAYER_REGEN_WRITE_YOUTH_PROMOTION_NEWS_TEXT);
    compare_text_writer(out, || o(news, body, out), |t| player_regen_write_youth_promotion_news_text(news, body, t),
        || format!("player_regen_write_youth_promotion_news_text(body {})", body));
}

// ---------- sx18_1: the English ruling body (its constructor and its card rules) and the pieces every country's ruling body shares (ruling_body/rb_*.cpp) ----------

/// `Discipline::post_ban_news(player, ban, fixture)` (0x557280, discipline.cpp, thiscall, ret 0xc): the ban news.
const FN_POST_BAN_NEWS: usize = 0x557280;

/// One ruling body class as its constructor sets it on the RulingBody base: the vtable, the slot of the person's
/// discipline record it uses (`ruling_body::RECORD_SLOT`), its index in the discipline table
/// (`ruling_body::RULE_SET_INDEX`), the ban scale (`ruling_body::BAN_SCALE`), the nation and continent it covers (the
/// key, or None for −1) and the indexes of the league, cup and continental ruling bodies of its area (+0x1f, +0x20,
/// +0x21; 0xff none).
pub(crate) struct RulingBodyClass {
    pub(crate) vtable: u32,
    pub(crate) record_slot: u8,
    pub(crate) rule_set: u8,
    pub(crate) scale: u8,
    pub(crate) nation: Option<usize>,
    pub(crate) continent: Option<usize>,
    pub(crate) league_rule: u8,
    pub(crate) cup_rule: u8,
    pub(crate) continental_rule: u8,
}

/// The value of an optional key (−1 without one).
unsafe fn ruling_body_key(key: Option<usize>) -> u32 {
    key.map_or(u32::MAX, |k| read_u32(k))
}

/// Builds a ruling body of the class: the RulingBody base with the file (`construct_ruling_body`), then the class
/// fields (`ruling_body_class_fields`). Returns this.
pub(crate) unsafe fn ruling_body_class_construct(this: usize, file: usize, class: &RulingBodyClass) -> usize {
    construct_ruling_body(this, file);
    ruling_body_class_fields(this, class)
}

/// Sets the class fields on a built RulingBody base: the vtable, the record slot, the rule set, the ban scale, the
/// nation and continent ids, the area's league, cup and continental rule sets and the byte +0x22 cleared. Returns this.
pub(crate) unsafe fn ruling_body_class_fields(this: usize, class: &RulingBodyClass) -> usize {
    write_u32(this, class.vtable);
    write_u8(this + ruling_body::RECORD_SLOT, class.record_slot);
    write_u8(this + ruling_body::RULE_SET_INDEX, class.rule_set);
    write_u8(this + ruling_body::BAN_SCALE, class.scale);
    write_u32(this + ruling_body::NATION_ID, ruling_body_key(class.nation));
    write_u32(this + ruling_body::CONTINENT_ID, ruling_body_key(class.continent));
    write_u8(this + 0x1f, class.league_rule); // the area's league ruling body
    write_u8(this + 0x20, class.cup_rule); // the area's cup ruling body
    write_u8(this + 0x21, class.continental_rule); // the area's continental ruling body
    write_u8(this + 0x22, 0); // a flag byte, cleared
    this
}

/// Comparer of the ruling body constructors: they are built once per game, so only the original runs and the fields
/// it leaves are checked against the class (the copy's constants).
pub(crate) unsafe fn compare_ruling_body_class_built(r: usize, this: usize, class: &RulingBodyClass, name: &str) -> usize {
    let ok = r == this && read_u32(this) == class.vtable
        && read_u8(this + ruling_body::RECORD_SLOT) == class.record_slot
        && read_u8(this + ruling_body::RULE_SET_INDEX) == class.rule_set
        && read_u8(this + ruling_body::BAN_SCALE) == class.scale
        && read_u32(this + ruling_body::NATION_ID) == ruling_body_key(class.nation)
        && read_u32(this + ruling_body::CONTINENT_ID) == ruling_body_key(class.continent)
        && read_u8(this + 0x1f) == class.league_rule && read_u8(this + 0x20) == class.cup_rule
        && read_u8(this + 0x21) == class.continental_rule && read_u8(this + 0x22) == 0;
    SCOREBOARD.record(ok, || format!("{}({:#x}): vtable {:#x} slot {} rule {} scale {} nation {} continent {} rules {:x} {:x} {:x}",
        name, this, read_u32(this), read_u8(this + ruling_body::RECORD_SLOT), read_u8(this + ruling_body::RULE_SET_INDEX),
        read_u8(this + ruling_body::BAN_SCALE), read_i32(this + ruling_body::NATION_ID), read_i32(this + ruling_body::CONTINENT_ID),
        read_u8(this + 0x1f), read_u8(this + 0x20), read_u8(this + 0x21)));
    r
}

/// Whether date `a` comes after date `b` (the year word unsigned, then the day word signed), as the card rules
/// compare their cut-off dates.
pub(crate) unsafe fn cm_date_after(a: usize, b: usize) -> bool {
    let (ya, yb) = (read_u16(a + 2), read_u16(b + 2));
    ya > yb || (ya == yb && read_i16(a) > read_i16(b))
}

/// The ban a ruling body hands out on the spot: the matches, the reason (0 points, 1 bookings, 2 red card) and its
/// amount, the four rules it applies under and the start date.
pub(crate) struct ImmediateBan {
    pub(crate) matches: u8,
    pub(crate) reason: u8,
    pub(crate) amount: i16,
    pub(crate) rules: [u8; 4],
    pub(crate) start: [u8; 8],
}

/// What a ruling body's card rules decide for one player after a match: the 0x70-byte pending bans queued on the
/// body's list, in order (bans that start on a later date), and the ban handed out on the spot.
pub(crate) struct CardVerdict {
    pub(crate) pending: Vec<[u8; 0x70]>,
    pub(crate) ban: Option<ImmediateBan>,
}

/// A copy of a date.
pub(crate) unsafe fn date_copy(source: usize) -> [u8; 8] {
    let mut d = [0u8; 8];
    copy_record_date(d.as_mut_ptr() as usize, source);
    d
}

/// Today's date, as the start of a ban.
pub(crate) unsafe fn ban_start_today() -> [u8; 8] {
    date_copy(G_CURRENT_DATE)
}

/// Today plus `days` (`Date::add_days`).
pub(crate) unsafe fn ban_start_in(days: u32) -> [u8; 8] {
    let mut t = [0u8; 8];
    date_copy(exe::date_add_days(G_CURRENT_DATE, t.as_mut_ptr() as usize, days))
}

/// Today plus `weeks` (`Date::add_weeks`).
pub(crate) unsafe fn ban_start_in_weeks(weeks: u32) -> [u8; 8] {
    let mut t = [0u8; 8];
    date_copy(exe::date_add_weeks(G_CURRENT_DATE, &mut t, weeks))
}

/// The date one day later (`Date::add_days` into `d`).
pub(crate) unsafe fn date_next_day(d: &mut [u8; 8]) {
    let mut t = [0u8; 8];
    let p = exe::date_add_days(d.as_ptr() as usize, t.as_mut_ptr() as usize, 1);
    copy_record_date(d.as_mut_ptr() as usize, p);
}

/// The first day from `d` on (`d` itself included) whose `Date::get_weekday` is `weekday`.
pub(crate) unsafe fn date_on_weekday(mut d: [u8; 8], weekday: u16) -> [u8; 8] {
    while exe::date_get_weekday(d.as_ptr() as usize) as u16 != weekday {
        date_next_day(&mut d);
    }
    d
}

/// The first day after today whose `Date::get_weekday` is `weekday` (tomorrow, then day by day).
pub(crate) unsafe fn first_weekday_after_today(weekday: u16) -> [u8; 8] {
    date_on_weekday(ban_start_in(1), weekday)
}

/// Day `day` of the current month (`Date::set` with today's month and year); when today is past it, day `day` of the
/// next month (January of the next year after December).
pub(crate) unsafe fn day_of_this_or_next_month(day: u32) -> [u8; 8] {
    let year = read_u16(G_CURRENT_DATE_YEAR) as u32;
    let mut d = [0u8; 8];
    exe::date_set(d.as_mut_ptr() as usize, day, exe::date_get_month(G_CURRENT_DATE) as u32, year, -1);
    if cm_date_after(G_CURRENT_DATE, d.as_ptr() as usize) {
        let month = exe::date_get_month(G_CURRENT_DATE).wrapping_add(1);
        if (month as i8) < 0xc {
            exe::date_set(d.as_mut_ptr() as usize, day, month as u32, year, -1);
        } else {
            exe::date_set(d.as_mut_ptr() as usize, day, 0, (year as u16).wrapping_add(1) as u32, -1);
        }
    }
    d
}

/// The 20-byte ban record of an immediate ban: matches, none served, no days, the start date, the rules, the reason
/// and its amount, and the flags with bit 2 = (the row's byte +0x48 == 0). The original leaves the other flag bits as
/// stack garbage (bit 0 is then set or cleared by `Discipline::add_ban`); here they are 0.
pub(crate) unsafe fn immediate_ban_record(b: &ImmediateBan, row: MatchRecordRow) -> [u8; 0x14] {
    let mut r = [0u8; 0x14];
    let p = r.as_mut_ptr() as usize;
    write_u8(p + ban_record::MATCHES, b.matches);
    write_u8(p + ban_record::MATCHES_SERVED, 0);
    write_u16(p + ban_record::DAYS, 0xffff);
    copy_record_date(p + ban_record::START_DATE, b.start.as_ptr() as usize);
    for k in 0..4 {
        write_u8(p + ban_record::RULES + k, b.rules[k]);
    }
    write_u8(p + ban_record::REASON, b.reason);
    write_u16(p + ban_record::REASON_AMOUNT, b.amount as u16);
    write_u8(p + ban_record::FLAGS, ((read_u8(row.addr() + 0x48) == 0) as u8) << 2); // row +0x48: a per-player byte of the match
    r
}

/// The pending ban of a red card: `RulingBody::make_points_ban` (by matches) into a 0x70-byte node; None when the
/// offence brings none. Otherwise the node's start date (+0x14) is `start()`, its rules are `rules` and the fixture's
/// first 0x4f bytes go to +0x20 (the match, for the news). The original leaves the node's amount and flags (+0x11 to
/// +0x13) as stack garbage; here they are 0.
pub(crate) unsafe fn pending_points_ban(this: usize, row: MatchRecordRow, fixture: usize, rules: [u8; 4], start: impl FnOnce() -> [u8; 8]) -> Option<[u8; 0x70]> {
    let mut node = [0u8; 0x70];
    let p = node.as_mut_ptr() as usize;
    if make_points_ban(this, p, row.addr(), 1) == 0 {
        return None;
    }
    let d = start();
    copy_record_date(p + 0x14, d.as_ptr() as usize); // the pending ban's start date
    for k in 0..4 {
        write_u8(p + ban_record::RULES + k, rules[k]);
    }
    core::ptr::copy_nonoverlapping(fixture as *const u8, (p + 0x20) as *mut u8, 0x4f); // the match, for the news
    Some(node)
}

/// Carries out a verdict: the pending bans go to the end of the body's list (`LLIST::add_tail`, 0x70 bytes); the
/// immediate ban goes to the discipline (`Discipline::add_ban` with the player's id) and its news is posted
/// (`Discipline::post_ban_news` with the player, the ban and the fixture).
pub(crate) unsafe fn apply_card_verdict(this: usize, row: MatchRecordRow, fixture: usize, v: &CardVerdict) {
    for node in &v.pending {
        exe::llist_add_tail(read_usize(this + RULING_BODY_LIST), node.as_ptr() as usize, 0x70);
    }
    if let Some(b) = &v.ban {
        let mut r = immediate_ban_record(b, row);
        add_ban(read_usize(G_DISCIPLINE), row.staff_id() as u32, r.as_mut_ptr() as usize);
        let post: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(FN_POST_BAN_NEWS);
        post(read_usize(G_DISCIPLINE), staff_at(row.staff_id() as isize).addr(), r.as_mut_ptr() as usize, fixture);
    }
}

/// The offence code of a player's red card: the first card's with no booking, the second card's with one booking, −1
/// otherwise.
pub(crate) unsafe fn red_card_offence(row: MatchRecordRow) -> i8 {
    match row.yellow_cards() {
        0 => row.first_card_offence(),
        1 => row.second_card_offence(),
        _ => -1,
    }
}

/// Whether the row has a booking or a red card (the card rules do nothing otherwise).
pub(crate) unsafe fn row_has_cards(row: MatchRecordRow) -> bool {
    row.yellow_cards() != 0 || row.red_card() != 0
}

/// Runs a body's card rules for one player: the player's entry in the body (`RulingBody::find_or_add_discipline_entry`,
/// created when missing), the verdict and its effects.
pub(crate) unsafe fn run_entry_card_rules(this: usize, row: usize, fixture: usize, verdict: impl FnOnce(usize) -> CardVerdict) {
    let row = MatchRecordRow(row);
    let e = exe::rulingbody_find_or_add_discipline_entry(this, row.staff_id() as u32, 1);
    let v = verdict(e);
    apply_card_verdict(this, row, fixture, &v);
}

/// Runs a body's card rules for one player (`run_entry_card_rules`), nothing without a booking or a red card.
pub(crate) unsafe fn run_card_rules(this: usize, row: usize, fixture: usize, verdict: impl FnOnce(usize) -> CardVerdict) {
    if row_has_cards(MatchRecordRow(row)) {
        run_entry_card_rules(this, row, fixture, verdict);
    }
}

/// Comparer of the card rules on the player's entry: the bans cannot be handed out twice, so the copy's verdict is taken
/// first on a copy of the entry (found or created as the original does first), then only the original runs. Checked
/// afterwards: the entry's bookings, red cards, points and history bytes (+4, +5, +8 to +0xd); the verdict's pending
/// bans more on the body's list; and, when the player had no ban in the body before, the new ban the entry
/// points to (matches, days, start, rules, reason, amount and flag bit 2).
pub(crate) unsafe fn compare_entry_card_rules(this: usize, row: usize, original: impl FnOnce(), verdict: impl FnOnce(usize) -> CardVerdict, name: &str) {
    let row = MatchRecordRow(row);
    let id = row.staff_id() as u32;
    let e = exe::rulingbody_find_or_add_discipline_entry(this, id, 1);
    let mut local = [0u8; 0xe];
    core::ptr::copy_nonoverlapping(e as *const u8, local.as_mut_ptr(), 0xe);
    let had_ban = read_i16(e + ruling_body_entry::BAN_INDEX) > -1;
    let list = read_usize(this + RULING_BODY_LIST);
    let queued = read_u16(list + llist::COUNT);
    let v = verdict(local.as_mut_ptr() as usize);
    original();
    let e = ruling_body_entry(this, id);
    let counts = |p: usize| [read_u16(p + ruling_body_entry::YELLOW_CARDS), read_u16(p + 8), read_u16(p + 0xa), read_u16(p + 0xc)];
    let (got, expected) = (if e != 0 { counts(e) } else { [0; 4] }, counts(local.as_ptr() as usize));
    let mut ok = e != 0 && got == expected && read_u16(list + llist::COUNT) == queued.wrapping_add(v.pending.len() as u16);
    if let (Some(b), false, true) = (&v.ban, had_ban, ok) {
        let s = read_i16(e + ruling_body_entry::BAN_INDEX);
        let r = immediate_ban_record(b, row);
        let p = r.as_ptr() as usize;
        let ban = read_usize(read_usize(G_DISCIPLINE) + discipline_fields::BANS).wrapping_add((s as i32 as usize).wrapping_mul(20));
        ok = s > -1 && read_u8(ban) == b.matches && read_u8(ban + ban_record::MATCHES_SERVED) == 0
            && read_i16(ban + ban_record::DAYS) == -1 && read_u32(ban + ban_record::START_DATE) == read_u32(p + ban_record::START_DATE)
            && read_u32(ban + ban_record::RULES) == read_u32(p + ban_record::RULES) && read_u8(ban + ban_record::REASON) == b.reason
            && read_u16(ban + ban_record::REASON_AMOUNT) == read_u16(p + ban_record::REASON_AMOUNT)
            && read_u8(ban + ban_record::FLAGS) & 4 == read_u8(p + ban_record::FLAGS) & 4;
    }
    SCOREBOARD.record(ok, || format!("{}(player {}): entry {:x?} (expected {:x?}), queued {} (before {}), ban expected {:?}",
        name, id, got, expected, read_u16(list + llist::COUNT), queued, v.ban.as_ref().map(|b| (b.matches, b.reason, b.amount))));
}

/// Comparer of the card rules (`compare_entry_card_rules`); without a booking or a red card only the original runs.
pub(crate) unsafe fn compare_card_rules(this: usize, row: usize, original: impl FnOnce(), verdict: impl FnOnce(usize) -> CardVerdict, name: &str) {
    if row_has_cards(MatchRecordRow(row)) {
        compare_entry_card_rules(this, row, original, verdict, name);
    } else {
        original();
    }
}

/// The English ruling body: vtable 0x96eb14, record slot 0, rule set 9, ban scale 5, England in Europe, league rule 9,
/// no cup rule, the European rule 1.
pub(crate) const ENGLAND_RULING_BODY: RulingBodyClass = RulingBodyClass { vtable: 0x96eb14, record_slot: 0, rule_set: 9, scale: 5,
    nation: Some(KEY_NATION_ENGLAND), continent: Some(KEY_CONTINENT_EUROPE), league_rule: 9, cup_rule: 0xff, continental_rule: 1 };

/// Builds the English ruling body: the RulingBody base with the file, an empty date at +0x23 (`Date::Date`; the English
/// object is the only one allocated with 0x2b bytes) and the class fields of `ENGLAND_RULING_BODY`. Returns this.
/// Original: `RbEngland::RbEngland` (0x7dcf90, rb_england.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn rb_england_construct(this: usize, file: usize) -> usize {
    construct_ruling_body(this, file);
    exe::date_date_54a110(this + 0x23); // the English body's own date
    covered!("sx18_1 english ruling body built");
    ruling_body_class_fields(this, &ENGLAND_RULING_BODY)
}

/// The English card rules' verdict on one player's match (`entry`: his entry in the body).
///
/// Bookings: the entry's count grows by the match's bookings; the ban threshold is the next of 5, 10 and 15 above the
/// count before the match (from 15 on, (count mod 5 + 1) × 5). Reaching it bans for threshold / 5 matches with reason 1
/// and the new count as amount, except that the 5th booking only counts in a match played before 1 March of the
/// season's second year and the 10th only before the day after the second weekday-6 from 1 April of that year (the
/// match date is the fixture's day and year; the season year is the fixture's `.season_year()`). A red card adds one
/// match, two more for an offence code from 9 and three more for code 12 (the first card's offence with no booking,
/// the second's with one booking, none otherwise), plus one per red card already in the entry, whose count then grows;
/// reason 2, amount 1. The ban starts in 14 days, under the English rule set alone.
pub(crate) unsafe fn england_card_verdict(row: MatchRecordRow, fixture: Fixture, entry: usize) -> CardVerdict {
    let mut played = [0u8; 8];
    exe::new_date(played.as_mut_ptr() as usize, fixture.day() as u32, fixture.year() as u32);
    let next_year = fixture.season_year().wrapping_add(1) as u32;
    let mut april = [0u16; 4];
    exe::date_date_549f70(&mut april, 1, 3, next_year, u32::MAX);
    let mut weekday_6 = 0;
    loop {
        if exe::date_get_weekday(april.as_ptr() as usize) as u16 == 6 {
            weekday_6 += 1;
        }
        let mut t = [0u8; 8];
        let p = exe::date_add_days(april.as_ptr() as usize, t.as_mut_ptr() as usize, 1);
        copy_record_date(april.as_mut_ptr() as usize, p);
        if weekday_6 >= 2 {
            break;
        }
    }
    let mut march = [0u16; 4];
    exe::date_date_549f70(&mut march, 1, 2, next_year, u32::MAX);
    let (mut matches, mut reason, mut amount) = (0u8, 0xffu8, 0i16);
    let bookings = row.yellow_cards();
    if bookings != 0 {
        let before = read_u8(entry + ruling_body_entry::YELLOW_CARDS);
        let threshold: u8 = match before {
            0..=4 => 5, // sx18_1: the first booking ban
            5..=9 => 10,
            10..=14 => 15,
            _ => (before % 5 + 1).wrapping_mul(5),
        };
        let total = before.wrapping_add(bookings);
        write_u8(entry + ruling_body_entry::YELLOW_CARDS, total);
        if total as i32 >= threshold as i8 as i32 {
            let due = match threshold {
                5 => cm_date_after(march.as_ptr() as usize, played.as_ptr() as usize),
                10 => cm_date_after(april.as_ptr() as usize, played.as_ptr() as usize),
                t => t as i8 > 10,
            };
            if due {
                covered!("sx18_1 english booking ban");
                reason = 1;
                matches = (threshold as i8 / 5) as u8;
                amount = total as i8 as i16;
            }
        }
    }
    if row.red_card() != 0 {
        covered!("sx18_1 english red card");
        let offence = red_card_offence(row);
        matches = matches.wrapping_add(1);
        if offence == 0xc {
            matches = matches.wrapping_add(3);
        } else if offence != -1 && offence >= 9 {
            matches = matches.wrapping_add(2);
        }
        let reds = read_u8(entry + ruling_body_entry::RED_CARDS);
        write_u8(entry + ruling_body_entry::RED_CARDS, reds.wrapping_add(1));
        reason = 2;
        amount = 1;
        matches = matches.wrapping_add(reds); // one more match per earlier red card
    }
    let ban = (matches != 0).then(|| ImmediateBan { matches, reason, amount, rules: [ENGLAND_RULING_BODY.rule_set, 0xff, 0xff, 0xff],
        start: ban_start_in(14) }); // sx18_1: the English ban starts in two weeks
    CardVerdict { pending: Vec::new(), ban }
}

/// Applies a player's cards of one match under the English rules (`england_card_verdict`): nothing without a booking or
/// a red card; the ban, when there is one, goes to the discipline with its news.
/// Original: `RbEngland::process_match_cards` (0x7dd010, rb_england.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn rb_england_process_match_cards(this: usize, row: usize, fixture: usize) {
    run_card_rules(this, row, fixture, |e| england_card_verdict(MatchRecordRow(row), Fixture(fixture), e));
}

pub(crate) unsafe extern "thiscall" fn compare_rb_england_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_RB_ENGLAND_CONSTRUCT)(this, file);
    compare_ruling_body_class_built(r, this, &ENGLAND_RULING_BODY, "rb_england_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_rb_england_process_match_cards(this: usize, row: usize, fixture: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_RB_ENGLAND_PROCESS_MATCH_CARDS);
    compare_card_rules(this, row, || o(this, row, fixture), |e| england_card_verdict(MatchRecordRow(row), Fixture(fixture), e), "rb_england_process_match_cards");
}

/// The plain card rules several bodies share (Argentina, Asia, Europe, International): exactly one red card bans for one
/// match (reason 2, amount 1); a booking adds to the entry's count (all the match's bookings with `add_all`, otherwise
/// one) and every `every`-th booking bans for one more match (reason 1, amount `every`). The ban starts today, under
/// `rules`.
pub(crate) unsafe fn plain_card_verdict(row: MatchRecordRow, entry: usize, add_all: bool, every: u8, rules: [u8; 4]) -> CardVerdict {
    let (mut matches, mut reason, mut amount) = (0u8, 0xffu8, 0i16);
    if row.red_card() == 1 {
        matches = 1;
        reason = 2;
        amount = 1;
    }
    let bookings = row.yellow_cards();
    if bookings != 0 {
        let count = read_u8(entry + ruling_body_entry::YELLOW_CARDS).wrapping_add(if add_all { bookings } else { 1 });
        write_u8(entry + ruling_body_entry::YELLOW_CARDS, count);
        if count % every == 0 {
            matches += 1;
            reason = 1;
            amount = every as i16;
        }
    }
    let ban = (matches != 0).then(|| ImmediateBan { matches, reason, amount, rules, start: ban_start_today() });
    CardVerdict { pending: Vec::new(), ban }
}

/// Europe's card rules (`plain_card_verdict`): one match for a red card and one for every third booking (counted one
/// per match), under the European rule set in the confederation slot (rules ff ff 01 ff).
pub(crate) unsafe fn europe_card_verdict(row: MatchRecordRow, entry: usize) -> CardVerdict {
    covered!("sx18_1 european cards");
    plain_card_verdict(row, entry, false, 3, [0xff, 0xff, 1, 0xff]) // sx18_1: every third European booking
}

/// Applies a player's cards of one match under the European rules (`europe_card_verdict`): nothing without a booking
/// or a red card; the ban, when there is one, starts today and goes to the discipline with its news.
/// Original: `RbEurope::process_match_cards` (0x7dd450, rb_europe.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn rb_europe_process_match_cards(this: usize, row: usize, fixture: usize) {
    run_card_rules(this, row, fixture, |e| europe_card_verdict(MatchRecordRow(row), e));
}

pub(crate) unsafe extern "thiscall" fn compare_rb_europe_process_match_cards(this: usize, row: usize, fixture: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_RB_EUROPE_PROCESS_MATCH_CARDS);
    compare_card_rules(this, row, || o(this, row, fixture), |e| europe_card_verdict(MatchRecordRow(row), e), "rb_europe_process_match_cards");
}

// ---------- sx18_2: the three general ruling bodies built in every game: Europe, International (FIFA) and Asia (ruling_body/rb_europe.cpp, rb_international.cpp, rb_asia.cpp) ----------

/// The European ruling body: vtable 0x96eb34, record slot 2 (the confederations'), rule set 1, ban scale 10, no
/// nation, Europe, no league or cup rule, itself as the continental rule.
pub(crate) const EUROPE_RULING_BODY: RulingBodyClass = RulingBodyClass { vtable: 0x96eb34, record_slot: 2, rule_set: 1, scale: 0xa,
    nation: None, continent: Some(KEY_CONTINENT_EUROPE), league_rule: 0xff, cup_rule: 0xff, continental_rule: 1 };

/// The international (FIFA) ruling body: vtable 0x96ec34, record slot 3, rule set 0, ban scale 10, no nation, no
/// continent, no league, cup or continental rule.
pub(crate) const INTERNATIONAL_RULING_BODY: RulingBodyClass = RulingBodyClass { vtable: 0x96ec34, record_slot: 3, rule_set: 0, scale: 0xa,
    nation: None, continent: None, league_rule: 0xff, cup_rule: 0xff, continental_rule: 0xff };

/// The Asian ruling body: vtable 0x96ea14, record slot 2, rule set 0x29, ban scale 10, no nation, Asia, no league or
/// cup rule, itself as the continental rule.
pub(crate) const ASIA_RULING_BODY: RulingBodyClass = RulingBodyClass { vtable: 0x96ea14, record_slot: 2, rule_set: 0x29, scale: 0xa,
    nation: None, continent: Some(KEY_CONTINENT_ASIA), league_rule: 0xff, cup_rule: 0xff, continental_rule: 0x29 };

/// Builds the European ruling body (`ruling_body_class_construct` with `EUROPE_RULING_BODY`). Returns this.
/// Original: `RbEurope::RbEurope` (0x7dd400, rb_europe.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn rb_europe_construct(this: usize, file: usize) -> usize {
    covered!("sx18_2 european ruling body built");
    ruling_body_class_construct(this, file, &EUROPE_RULING_BODY)
}

/// Builds the international ruling body (`ruling_body_class_construct` with `INTERNATIONAL_RULING_BODY`). Returns this.
/// Original: `RbInternational::RbInternational` (0x7df180, rb_international.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn rb_international_construct(this: usize, file: usize) -> usize {
    covered!("sx18_2 international ruling body built");
    ruling_body_class_construct(this, file, &INTERNATIONAL_RULING_BODY)
}

/// Builds the Asian ruling body (`ruling_body_class_construct` with `ASIA_RULING_BODY`); with data older than version
/// 0x3e044 the discipline passes it no file. Returns this.
/// Original: `RbAsia::RbAsia` (0x7db360, rb_asia.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn rb_asia_construct(this: usize, file: usize) -> usize {
    covered!("sx18_2 asian ruling body built");
    ruling_body_class_construct(this, file, &ASIA_RULING_BODY)
}

pub(crate) unsafe extern "thiscall" fn compare_rb_europe_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_RB_EUROPE_CONSTRUCT)(this, file);
    compare_ruling_body_class_built(r, this, &EUROPE_RULING_BODY, "rb_europe_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_rb_international_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_RB_INTERNATIONAL_CONSTRUCT)(this, file);
    compare_ruling_body_class_built(r, this, &INTERNATIONAL_RULING_BODY, "rb_international_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_rb_asia_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_RB_ASIA_CONSTRUCT)(this, file);
    compare_ruling_body_class_built(r, this, &ASIA_RULING_BODY, "rb_asia_construct")
}

// ---------- sx18_3: the international (FIFA) ruling body's card rules and new season (ruling_body/rb_international.cpp) ----------

/// The international card rules (`plain_card_verdict`): one match for a red card and one for every second booking
/// (counted one per match), under the FIFA rule set in the fourth slot (rules ff ff ff 00).
pub(crate) unsafe fn international_card_verdict(row: MatchRecordRow, entry: usize) -> CardVerdict {
    covered!("sx18_3 international cards");
    plain_card_verdict(row, entry, false, 2, [0xff, 0xff, 0xff, 0]) // sx18_3: every second international booking
}

/// Applies a player's cards of one match under the international rules (`international_card_verdict`): nothing
/// without a booking or a red card; the ban, when there is one, starts today and goes to the discipline with its news.
/// Original: `RbInternational::process_match_cards` (0x7df1c0, rb_international.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn rb_international_process_match_cards(this: usize, row: usize, fixture: usize) {
    run_card_rules(this, row, fixture, |e| international_card_verdict(MatchRecordRow(row), e));
}

/// The international body's new season: for each occupied entry (14 bytes in `ruling_body::ENTRIES`, count
/// `ruling_body::ENTRY_COUNT`; id −1 = free): without a ban (`ruling_body_entry::BAN_INDEX` −1) and with no booking nor
/// red card, the player's slot is cleared (`Discipline::set_slot` with the body's record slot and rule set, −1) and the
/// entry freed; with a ban whose start year (+6 of the 20-byte ban) is 4 or more years before today's year, the ban is
/// released (`Discipline::release_ban`), the slot cleared and the entry freed; the others stay as they are (their counts
/// are not cleared, unlike the base `RulingBody::start_new_season`). At the end the entries are compacted
/// (`RulingBody::compact_entries`).
/// Original: `RbInternational::start_new_season` (0x7df3c0, rb_international.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn rb_international_start_new_season(this: usize) {
    covered!("sx18_3 international new season");
    let d = read_usize(G_DISCIPLINE);
    let mut e = read_usize(this + ruling_body::ENTRIES);
    let mut i = 0i32;
    while i < read_i32(this + ruling_body::ENTRY_COUNT) {
        let id = read_i32(e + ruling_body_entry::PERSON_ID);
        if id != -1 {
            let s = read_i16(e + ruling_body_entry::BAN_INDEX);
            let free = if s == -1 {
                read_u8(e + ruling_body_entry::YELLOW_CARDS) == 0 && read_u8(e + ruling_body_entry::RED_CARDS) == 0
            } else {
                let ban = read_usize(d + discipline_fields::BANS).wrapping_add((s as i32 as usize).wrapping_mul(20));
                let old = (read_u16(G_CURRENT_DATE_YEAR) as i32 - read_u16(ban + ban_record::START_DATE + 2) as i32) >= 4;
                if old {
                    // simplification: the original pushes the index with stale high 16 bits; release_ban reads only the word.
                    exe::discipline_release_ban(d, id as u32, s as u16 as u32);
                }
                old
            };
            if free {
                exe::discipline_set_slot(d, id, read_i8(this + ruling_body::RECORD_SLOT), read_u8(this + ruling_body::RULE_SET_INDEX), u32::MAX);
                write_u32(e + ruling_body_entry::PERSON_ID, u32::MAX);
            }
        }
        i += 1;
        e += 0xe;
    }
    exe::rulingbody_compact_entries(this);
}

pub(crate) unsafe extern "thiscall" fn compare_rb_international_process_match_cards(this: usize, row: usize, fixture: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_RB_INTERNATIONAL_PROCESS_MATCH_CARDS);
    compare_card_rules(this, row, || o(this, row, fixture), |e| international_card_verdict(MatchRecordRow(row), e), "rb_international_process_match_cards");
}

/// Comparer for `rb_international_start_new_season`: it releases bans and reallocates (it cannot run twice); the
/// entries the copy would free are listed first, then only the original runs, and the compacted table must hold
/// exactly the others, in order.
pub(crate) unsafe extern "thiscall" fn compare_rb_international_start_new_season(this: usize) {
    let d = read_usize(G_DISCIPLINE);
    let mut kept = Vec::new();
    let e0 = read_usize(this + ruling_body::ENTRIES);
    for i in 0..read_i32(this + ruling_body::ENTRY_COUNT).max(0) as usize {
        let e = e0 + i * 0xe;
        let id = read_i32(e + ruling_body_entry::PERSON_ID);
        if id == -1 {
            continue;
        }
        let s = read_i16(e + ruling_body_entry::BAN_INDEX);
        let free = if s == -1 {
            read_u8(e + ruling_body_entry::YELLOW_CARDS) == 0 && read_u8(e + ruling_body_entry::RED_CARDS) == 0
        } else {
            let ban = read_usize(d + discipline_fields::BANS).wrapping_add((s as i32 as usize).wrapping_mul(20));
            read_u16(G_CURRENT_DATE_YEAR) as i32 - read_u16(ban + ban_record::START_DATE + 2) as i32 >= 4
        };
        if !free {
            kept.push(id);
        }
    }
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_RB_INTERNATIONAL_START_NEW_SEASON)(this);
    let e0 = read_usize(this + ruling_body::ENTRIES);
    let n = read_i32(this + ruling_body::ENTRY_COUNT);
    let ok = n as usize == kept.len() && kept.iter().enumerate().all(|(i, &id)| read_i32(e0 + i * 0xe) == id);
    SCOREBOARD.record(ok, || format!("rb_international_start_new_season: {} entries kept (expected {})", n, kept.len()));
}

// ---------- sx18_4: a club match record and a staff name reference into and out of a news item's numbered slots (record_utils.cpp) ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\record_utils.cpp" (the error boxes).
const RECORD_UTILS_CPP: usize = 0xa7a088;

/// Stores `values` in the news item's slots from `*k` on (`NewsItem::set_slot`), advancing `*k` past them.
pub(crate) unsafe fn news_put_slots(news: usize, k: usize, values: &[u32]) {
    for &v in values {
        let slot = read_u8(k);
        write_u8(k, slot.wrapping_add(1));
        exe::newsitem_set_slot(news, slot as i8, v);
    }
}

/// Reads `N` slots of the news item from `*k` on (`NewsItem::get_slot`), advancing `*k` past them.
pub(crate) unsafe fn news_take_slots<const N: usize>(news: usize, k: usize) -> [u32; N] {
    let mut v = [0u32; N];
    for x in v.iter_mut() {
        let slot = read_u8(k);
        write_u8(k, slot.wrapping_add(1));
        *x = exe::newsitem_get_slot(news, slot as u32);
    }
    v
}

/// Writes the date (day, year) read from two slots into `at` (`Date::Date` then `Date::operator=`).
pub(crate) unsafe fn news_slot_date(at: usize, day: u32, year: u32) {
    let mut d = [0u8; 8];
    let p = exe::new_date(d.as_mut_ptr() as usize, day, year);
    copy_record_date(at, p);
}

/// The two slot values of a date: the day word sign-extended, the year word zero-extended.
pub(crate) unsafe fn date_slot_values(at: usize) -> [u32; 2] {
    [read_i16(at) as i32 as u32, read_u16(at + 2) as u32]
}

/// The eleven slot values of a club match record, in the original's order: the date (day, year), the fixture id, the
/// round prefix and name words (sign-extended), the venue byte (sign-extended), the club and opponent ids, the goals for
/// and against (sign-extended) and the match statistics slot.
pub(crate) unsafe fn match_record_slot_values(record: usize) -> [u32; 11] {
    let [day, year] = date_slot_values(record);
    [day, year, read_u32(record + club_match_record::FIXTURE_ID),
     read_i16(record + club_match_record::ROUND_PREFIX) as i32 as u32, read_i16(record + club_match_record::ROUND_NAME) as i32 as u32,
     read_i8(record + club_match_record::VENUE) as i32 as u32, read_u32(record + club_match_record::CLUB_ID),
     read_u32(record + club_match_record::OPPONENT_ID), read_i8(record + club_match_record::GOALS_FOR) as i32 as u32, // sx18_4: the goals for
     read_i8(record + club_match_record::GOALS_AGAINST) as i32 as u32, read_u32(record + club_match_record::MATCH_STATS_SLOT)]
}

/// The six slot values of a staff name reference: the three name ids, the staff id and the date (day, year).
pub(crate) unsafe fn staff_name_ref_slot_values(r: usize) -> [u32; 6] {
    let [day, year] = date_slot_values(r + record_holder::DATE);
    [read_u32(r), read_u32(r + record_holder::SECOND_NAME_ID), read_u32(r + record_holder::COMMON_NAME_ID),
     read_u32(r + record_holder::STAFF_ID), day, year]
}

/// Stores the eleven fields of a club match record (`match_record_slot_values`) in the news item's slots from `*k` on
/// and advances `*k`; returns 1. A null record or news item shows the error box (lines 0x160, 0x168) and returns 0.
/// Original: `write_match_record_to_news` (0x7e4de0, record_utils.cpp, cdecl).
pub(crate) unsafe extern "C" fn write_match_record_to_news(record: usize, news: usize, k: usize) -> u32 {
    if record == 0 {
        show_error_at(RECORD_UTILS_CPP, 0x160);
        return 0;
    }
    if news == 0 {
        show_error_at(RECORD_UTILS_CPP, 0x168);
        return 0;
    }
    covered!("sx18_4 match record to news");
    news_put_slots(news, k, &match_record_slot_values(record));
    1
}

/// Reads a club match record back from the news item's slots from `*k` on and advances `*k`: the date from the first
/// two (day, year), then the fixture id, the round prefix and name (words), the venue (byte), the club and opponent
/// ids, the goals for and against (bytes) and the match statistics slot; returns 1. A null news item or record shows
/// the error box (lines 0x18e, 0x196) and returns 0.
/// Original: `read_match_record_from_news` (0x7e4fa0, record_utils.cpp, cdecl).
pub(crate) unsafe extern "C" fn read_match_record_from_news(news: usize, k: usize, record: usize) -> u32 {
    if news == 0 {
        show_error_at(RECORD_UTILS_CPP, 0x18e);
        return 0;
    }
    if record == 0 {
        show_error_at(RECORD_UTILS_CPP, 0x196);
        return 0;
    }
    covered!("sx18_4 match record from news");
    let [day, year] = news_take_slots::<2>(news, k);
    news_slot_date(record, day, year);
    let v = news_take_slots::<9>(news, k);
    write_u32(record + club_match_record::FIXTURE_ID, v[0]);
    write_u16(record + club_match_record::ROUND_PREFIX, v[1] as u16);
    write_u16(record + club_match_record::ROUND_NAME, v[2] as u16);
    write_u8(record + club_match_record::VENUE, v[3] as u8);
    write_u32(record + club_match_record::CLUB_ID, v[4]);
    write_u32(record + club_match_record::OPPONENT_ID, v[5]);
    write_u8(record + club_match_record::GOALS_FOR, v[6] as u8);
    write_u8(record + club_match_record::GOALS_AGAINST, v[7] as u8); // sx18_4: the goals against
    write_u32(record + club_match_record::MATCH_STATS_SLOT, v[8]);
    1
}

/// Stores a staff name reference (`staff_name_ref_slot_values`: the three name ids, the staff id, the date) in the news
/// item's slots from `*k` on and advances `*k`; returns 1. A null reference or news item shows the error box (lines
/// 0x1bb, 0x1c3) and returns 0.
/// Original: `write_staff_name_ref_to_news` (0x7e5160, record_utils.cpp, cdecl).
pub(crate) unsafe extern "C" fn write_staff_name_ref_to_news(r: usize, news: usize, k: usize) -> u32 {
    if r == 0 {
        show_error_at(RECORD_UTILS_CPP, 0x1bb);
        return 0;
    }
    if news == 0 {
        show_error_at(RECORD_UTILS_CPP, 0x1c3);
        return 0;
    }
    covered!("sx18_4 name reference to news");
    news_put_slots(news, k, &staff_name_ref_slot_values(r));
    1
}

/// Comparer of the slot writers: the copy's values are taken first, then only the original writes (a news item's slots
/// are written once); the slots it wrote from the old `*k` must hold the copy's values, and `*k` must have moved past
/// them.
pub(crate) unsafe fn compare_news_slot_writer(news: usize, k: usize, values: &[u32], original: impl FnOnce() -> u32, name: &str) -> u32 {
    let k0 = read_u8(k);
    let r = original();
    let got: Vec<u32> = (0..values.len()).map(|i| exe::newsitem_get_slot(news, k0.wrapping_add(i as u8) as u32)).collect();
    let ok = r == 1 && read_u8(k) == k0.wrapping_add(values.len() as u8) && got == values;
    SCOREBOARD.record(ok, || format!("{}(news {:#x}, slot {}): original {:x?}, copy {:x?}", name, news, k0, got, values));
    r
}

/// Comparer of the slot readers: the copy reads into a copy of the record with its own `*k` first, then the original
/// reads into the record; the `size` bytes of both records and the two `*k` must match.
pub(crate) unsafe fn compare_news_slot_reader(k: usize, record: usize, size: usize, original: impl FnOnce() -> u32, copy: impl FnOnce(usize, usize) -> u32, name: &str) -> u32 {
    let mut local = vec![0u8; size];
    core::ptr::copy_nonoverlapping(record as *const u8, local.as_mut_ptr(), size);
    let mut k_copy = read_u8(k);
    let n = copy(&mut k_copy as *mut u8 as usize, local.as_mut_ptr() as usize);
    let r = original();
    let same = core::slice::from_raw_parts(record as *const u8, size) == &local[..];
    SCOREBOARD.record(r == n && same && read_u8(k) == k_copy, || format!("{}: original {:x?} k {}, copy {:x?} k {}", name,
        core::slice::from_raw_parts(record as *const u8, size), read_u8(k), local, k_copy));
    r
}

pub(crate) unsafe extern "C" fn compare_write_match_record_to_news(record: usize, news: usize, k: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, usize) -> u32>(IDX_WRITE_MATCH_RECORD_TO_NEWS);
    if record == 0 || news == 0 {
        return o(record, news, k);
    }
    compare_news_slot_writer(news, k, &match_record_slot_values(record), || o(record, news, k), "write_match_record_to_news")
}

pub(crate) unsafe extern "C" fn compare_read_match_record_from_news(news: usize, k: usize, record: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, usize) -> u32>(IDX_READ_MATCH_RECORD_FROM_NEWS);
    if news == 0 || record == 0 {
        return o(news, k, record);
    }
    compare_news_slot_reader(k, record, 0x1f, || o(news, k, record), |kc, rc| read_match_record_from_news(news, kc, rc), "read_match_record_from_news")
}

pub(crate) unsafe extern "C" fn compare_write_staff_name_ref_to_news(r: usize, news: usize, k: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, usize) -> u32>(IDX_WRITE_STAFF_NAME_REF_TO_NEWS);
    if r == 0 || news == 0 {
        return o(r, news, k);
    }
    compare_news_slot_writer(news, k, &staff_name_ref_slot_values(r), || o(r, news, k), "write_staff_name_ref_to_news")
}

// ---------- sx18_5: a staff name reference out of a news item's slots, and a dated span record into and out of them (record_utils.cpp) ----------

/// The five slot values of a span record: the value (+0), the start date (day, year) and the end date (day, year).
pub(crate) unsafe fn date_span_slot_values(r: usize) -> [u32; 5] {
    let [d0, y0] = date_slot_values(r + run_record::START_DATE);
    let [d1, y1] = date_slot_values(r + run_record::END_DATE);
    [read_u32(r), d0, y0, d1, y1]
}

/// Reads a staff name reference back from the news item's slots from `*k` on and advances `*k`: the three name ids and
/// the staff id (dwords), then the date from the next two (day, year); returns 1. A null news item or reference shows
/// the error box (lines 0x1e4, 0x1ec) and returns 0.
/// Original: `read_staff_name_ref_from_news` (0x7e52c0, record_utils.cpp, cdecl).
pub(crate) unsafe extern "C" fn read_staff_name_ref_from_news(news: usize, k: usize, r: usize) -> u32 {
    if news == 0 {
        show_error_at(RECORD_UTILS_CPP, 0x1e4);
        return 0;
    }
    if r == 0 {
        show_error_at(RECORD_UTILS_CPP, 0x1ec);
        return 0;
    }
    covered!("sx18_5 name reference from news");
    let v = news_take_slots::<6>(news, k);
    write_u32(r, v[0]); // sx18_5: the first name id
    write_u32(r + record_holder::SECOND_NAME_ID, v[1]);
    write_u32(r + record_holder::COMMON_NAME_ID, v[2]);
    write_u32(r + record_holder::STAFF_ID, v[3]);
    news_slot_date(r + record_holder::DATE, v[4], v[5]);
    1
}

/// Stores a span record (`date_span_slot_values`: the value, the start and end dates) in the news item's slots from
/// `*k` on and advances `*k`; returns 1. A null record or news item shows the error box (lines 0x20c, 0x214) and
/// returns 0.
/// Original: `write_date_span_record_to_news` (0x7e5420, record_utils.cpp, cdecl).
pub(crate) unsafe extern "C" fn write_date_span_record_to_news(r: usize, news: usize, k: usize) -> u32 {
    if r == 0 {
        show_error_at(RECORD_UTILS_CPP, 0x20c);
        return 0;
    }
    if news == 0 {
        show_error_at(RECORD_UTILS_CPP, 0x214);
        return 0;
    }
    covered!("sx18_5 span record to news");
    news_put_slots(news, k, &date_span_slot_values(r));
    1
}

/// Reads a span record back from the news item's slots from `*k` on and advances `*k`: the value (+0), then the start
/// date and the end date, each from two slots (day, year); returns 1. A null news item or record shows the error box
/// (lines 0x234, 0x23c) and returns 0.
/// Original: `read_date_span_record_from_news` (0x7e5570, record_utils.cpp, cdecl).
pub(crate) unsafe extern "C" fn read_date_span_record_from_news(news: usize, k: usize, r: usize) -> u32 {
    if news == 0 {
        show_error_at(RECORD_UTILS_CPP, 0x234);
        return 0;
    }
    if r == 0 {
        show_error_at(RECORD_UTILS_CPP, 0x23c);
        return 0;
    }
    covered!("sx18_5 span record from news");
    let [value, d0, y0] = news_take_slots::<3>(news, k);
    write_u32(r, value);
    news_slot_date(r + run_record::START_DATE, d0, y0);
    let [d1, y1] = news_take_slots::<2>(news, k);
    news_slot_date(r + run_record::END_DATE, d1, y1); // sx18_5: the end date
    1
}

pub(crate) unsafe extern "C" fn compare_read_staff_name_ref_from_news(news: usize, k: usize, r: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, usize) -> u32>(IDX_READ_STAFF_NAME_REF_FROM_NEWS);
    if news == 0 || r == 0 {
        return o(news, k, r);
    }
    compare_news_slot_reader(k, r, 0x18, || o(news, k, r), |kc, rc| read_staff_name_ref_from_news(news, kc, rc), "read_staff_name_ref_from_news")
}

pub(crate) unsafe extern "C" fn compare_write_date_span_record_to_news(r: usize, news: usize, k: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, usize) -> u32>(IDX_WRITE_DATE_SPAN_RECORD_TO_NEWS);
    if r == 0 || news == 0 {
        return o(r, news, k);
    }
    compare_news_slot_writer(news, k, &date_span_slot_values(r), || o(r, news, k), "write_date_span_record_to_news")
}

pub(crate) unsafe extern "C" fn compare_read_date_span_record_from_news(news: usize, k: usize, r: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, usize) -> u32>(IDX_READ_DATE_SPAN_RECORD_FROM_NEWS);
    if news == 0 || r == 0 {
        return o(news, k, r);
    }
    compare_news_slot_reader(k, r, 0x14, || o(news, k, r), |kc, rc| read_date_span_record_from_news(news, kc, rc), "read_date_span_record_from_news")
}

// ---------- sx18_28: the reserve competition: whether two clubs can field reserve sides, today's unplayable reserve fixtures and the B and Under 21 sub competitions (record_utils.cpp) ----------

/// A Turkish competition key the reserve rules require of a Turkish home club's division (no name in `keys.rs` yet).
const KEY_RESERVE_TURKISH_DIVISION: usize = 0x9cf8d4;

/// The competition record (0x6b bytes in `G_CLUB_COMP_TABLE`) of the competition whose id the key holds.
unsafe fn reserve_comp_record(key: usize) -> usize {
    read_usize(G_CLUB_COMP_TABLE).wrapping_add((read_i32(key) as usize).wrapping_mul(0x6b))
}

/// One club's squad for a reserve match, over the 50 `club::SQUAD` slots: the empty slots; and, among the players at
/// the club (`staff::CLUB_JOB` = the club) and not injured (`Injury::get_player_injury_status`), the listed ones (id
/// below `G_STAFF_COUNT`), the young ones (age at `when` up to 21, counted only when `min_young` is not −1), the
/// available ones (bit 0 of `staff::PLAYING_SQUAD` clear) and those of them listed.
struct ReserveSquad {
    empty: i8,
    listed: i8,
    young: i8,
    available: i8,
    available_listed: i8,
}

unsafe fn reserve_squad(club: usize, min_young: i8, when: *const u16) -> ReserveSquad {
    let mut s = ReserveSquad { empty: 0, listed: 0, young: 0, available: 0, available_listed: 0 };
    for k in 0..50 {
        let p = read_usize(club + club::SQUAD + 4 * k);
        if p == 0 {
            s.empty += 1;
            continue;
        }
        if read_usize(p + staff::CLUB_JOB) != club || exe::injury_get_player_injury_status(read_usize(G_INJURY), p) != 0 {
            continue;
        }
        let listed = read_i32(p) < read_i32(G_STAFF_COUNT);
        if listed {
            s.listed += 1;
        }
        if min_young != -1 && exe::database_get_staff_age(p, when) <= 0x15 {
            s.young += 1;
        }
        if read_u8(p + staff::PLAYING_SQUAD) & 1 == 0 {
            if listed {
                s.available_listed += 1;
            }
            s.available += 1;
        }
    }
    s
}

/// Whether a club can field its reserve side: with fewer than 11 available players, no more than 36 listed and enough
/// empty slots to make up 14; a human club needs 11 available and one of them listed; and the young players must reach
/// `min_young`.
unsafe fn reserve_side_ok(club: usize, s: &ReserveSquad, min_young: i8) -> bool {
    if s.available < 0xb && (s.listed > 0x24 || (s.empty as i32) < 0xe - s.available as i32) {
        return false;
    }
    if exe::human_manager_is_human_club(club, 1, 1, 0, 0) != 0 && (s.available < 0xb || s.available_listed < 1) { // sx18_28: a human club's reserve side
        return false;
    }
    s.young >= min_young
}

/// Whether both clubs of a fixture can field a reserve side (`reserve_squad`, `reserve_side_ok`).
///
/// With `check_date`: not for a fixture with a round name (`fixture::ROUND_NAME` ≠ 0), nor when the home club is
/// Turkish (`club::NATION`) and its division is not the Turkish one, nor for the round prefixes 0x418, 0x419 and 0xa0,
/// nor on 26 December of the fixture's year. A home club in the Scottish Premier Division or Turkish needs 15 young
/// players (21 or under at the fixture's date); the others none (−1). Both clubs must have a reserve squad
/// (`database_club_has_reserve_squad`); then the home club's side and the away club's are checked in turn.
/// Original: `ReserveComp::can_clubs_field_reserve_sides` (0x7e6ba0, record_utils.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn can_clubs_field_reserve_sides(_this: usize, fixture: usize, check_date: u32) -> u32 {
    let f = Fixture(fixture);
    let nation_is_turkey = |club: usize| read_i32(read_usize(club + club::NATION)) == read_i32(KEY_NATION_TURKEY);
    if check_date != 0 {
        let home = f.home_club().addr();
        if read_u16(fixture + fixture::ROUND_NAME) != 0 {
            return 0;
        }
        if nation_is_turkey(home) {
            let division = read_usize(home + club::DIVISION);
            if division == 0 || read_i32(division) != read_i32(KEY_RESERVE_TURKISH_DIVISION) {
                return 0;
            }
        }
        if matches!(read_u16(fixture + fixture::ROUND_PREFIX), 0x418 | 0x419 | 0xa0) {
            return 0;
        }
        let (mut boxing_day, mut played) = ([0u16; 4], [0u8; 8]);
        exe::date_date_549f70(&mut boxing_day, 0x1a, 0xb, f.year() as u32, u32::MAX);
        let p = exe::new_date(played.as_mut_ptr() as usize, f.day() as u32, f.year() as u32);
        if boxing_day[1] == read_u16(p + 2) && boxing_day[0] == read_u16(p) { // sx18_28: no reserve match on 26 December
            return 0;
        }
    }
    let (mut min_young, mut when) = (-1i8, [0u16; 4]);
    exe::date_date_54a110(when.as_mut_ptr() as usize);
    let home = f.home_club().addr();
    if home != 0 {
        let division = read_usize(home + club::DIVISION);
        if division != 0 && (read_i32(division) == read_i32(KEY_CLUB_COMP_SCOTTISH_PREMIER_DIVISION) || nation_is_turkey(home)) {
            covered!("sx18_28 reserve sides with young players");
            min_young = 0xf;
            let mut t = [0u8; 8];
            let p = exe::new_date(t.as_mut_ptr() as usize, f.day() as u32, f.year() as u32);
            copy_record_date(when.as_mut_ptr() as usize, p);
        }
    }
    let away = f.away_club().addr();
    if exe::database_club_has_reserve_squad(home) == 0 || exe::database_club_has_reserve_squad(away) == 0 {
        return 0;
    }
    covered!("sx18_28 reserve sides counted");
    for club in [home, away] {
        let s = reserve_squad(club, min_young, when.as_ptr());
        if !reserve_side_ok(club, &s, min_young) {
            return 0;
        }
    }
    1
}

/// Removes today's reserve fixtures that cannot be played: for each active key nation's fixture list of today
/// (`fix_man_get_fixture_list_for_date` with today, the day slice and the index), each fixture of the reserve
/// competition or of the French CFA or lower division whose clubs cannot field reserve sides
/// (`can_clubs_field_reserve_sides` without the date checks) is copied to a local list; then each one is removed
/// (`FIXTURE_MANAGER::remove_fixture`).
/// Original: `ReserveComp::cancel_unplayable_reserve_fixtures` (0x7e6a70, record_utils.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn cancel_unplayable_reserve_fixtures(this: usize) {
    for fixture in unplayable_reserve_fixtures(this) {
        covered!("sx18_28 reserve fixture cancelled");
        exe::fixture_manager_remove_fixture(read_usize(G_FIXTURE_MANAGER), fixture.as_ptr() as usize);
    }
}

/// Today's reserve fixtures whose clubs cannot field reserve sides, copied (`fixture::SIZE` bytes each), in order.
unsafe fn unplayable_reserve_fixtures(this: usize) -> Vec<[u8; fixture::SIZE]> {
    let mut out = Vec::new();
    let mut k = 0i8;
    while k < read_i8(G_KEY_NATION_ACTIVE_COUNT) {
        let list = exe::fix_man_get_fixture_list_for_date(read_usize(G_FIXTURE_MANAGER), G_CURRENT_DATE, read_u8(G_CURRENT_DAY_SLICE) as u32, k as u8 as u32);
        if list != 0 {
            write_u32(list + llist::CURSOR, read_u32(list + llist::HEAD));
            loop {
                let fixture = exe::llist_get_next(list);
                if fixture == 0 {
                    break;
                }
                let comp = read_usize(fixture + fixture::COMPETITION);
                if comp == 0 {
                    continue;
                }
                let id = read_i32(comp);
                let reserve = [KEY_CLUB_COMP_RESERVE, KEY_CLUB_COMP_FRENCH_CFA, KEY_CLUB_COMP_FRENCH_LOWER_DIVISION].iter().any(|&key| id == read_i32(key));
                if reserve && can_clubs_field_reserve_sides(this, fixture, 0) == 0 {
                    let mut copy = [0u8; fixture::SIZE];
                    core::ptr::copy_nonoverlapping(fixture as *const u8, copy.as_mut_ptr(), fixture::SIZE);
                    out.push(copy);
                }
            }
        }
        k += 1;
    }
    out
}

/// Registers the reserve competition and builds its two sub competitions: the reserve competition object goes to its
/// slot of `G_COMP_OBJECT_TABLE` (`KEY_CLUB_COMP_RESERVE`); a B international competition (0xa3 bytes, `Comp::Comp`,
/// vtable 0x96f074, its record `KEY_CLUB_COMP_B_INTERNATIONAL`) goes to +0xa3 and to its slot, and an Under 21
/// international one (vtable 0x96efe4, `KEY_CLUB_COMP_UNDER_21_INTERNATIONAL`) to +0xa7 and to its slot (0 when out of
/// memory). Then, with the French first division built, the French CFA's slot gets its object's +0xee and the French
/// lower division's its +0xf2; with the Spanish first division built, the Spanish lower division's slot gets its +0xee.
/// Original: `ReserveComp::create_sub_comps` (0x7e61c0, record_utils.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn reserve_comp_create_sub_comps(this: usize) {
    let slot = |key: usize| read_usize(G_COMP_OBJECT_TABLE).wrapping_add((read_i32(key) as usize).wrapping_mul(4));
    write_usize(slot(KEY_CLUB_COMP_RESERVE), this);
    for (at, vtable, key) in [(0xa3usize, 0x96f074u32, KEY_CLUB_COMP_B_INTERNATIONAL), (0xa7, 0x96efe4, KEY_CLUB_COMP_UNDER_21_INTERNATIONAL)] {
        let p = exe::crt_operator_new(0xa3);
        let comp = if p != 0 {
            exe::new_comp(p);
            write_u32(p, vtable);
            write_usize(p + 4, reserve_comp_record(key)); // the competition's record
            p
        } else {
            0
        };
        write_usize(this + at, comp); // the B (+0xa3) and Under 21 (+0xa7) international competitions
        write_usize(slot(key), comp);
    }
    covered!("sx18_28 reserve sub competitions");
    let french = read_usize(slot(KEY_CLUB_COMP_FRENCH_FIRST_DIVISION));
    if french != 0 {
        write_usize(slot(KEY_CLUB_COMP_FRENCH_CFA), read_usize(french + 0xee)); // the French first division's CFA object
        write_usize(slot(KEY_CLUB_COMP_FRENCH_LOWER_DIVISION), read_usize(read_usize(slot(KEY_CLUB_COMP_FRENCH_FIRST_DIVISION)) + 0xf2));
    }
    let spanish = read_usize(slot(KEY_CLUB_COMP_SPANISH_FIRST_DIVISION));
    if spanish != 0 {
        write_usize(slot(KEY_CLUB_COMP_SPANISH_LOWER_DIVISION), read_usize(spanish + 0xee)); // the Spanish first division's lower object
    }
}

compare_value!(compare_can_clubs_field_reserve_sides, can_clubs_field_reserve_sides, IDX_CAN_CLUBS_FIELD_RESERVE_SIDES, (fixture: usize, check_date: u32) -> u32);

/// Comparer for `cancel_unplayable_reserve_fixtures`: the fixtures cannot be removed twice; the copy lists the
/// fixtures it would remove first (their first 0x14 bytes: competition, stadium, statistics slot and clubs), then only
/// the original runs; none of them may be left among today's fixtures.
pub(crate) unsafe extern "thiscall" fn compare_cancel_unplayable_reserve_fixtures(this: usize) {
    let expected: Vec<[u8; 0x14]> = unplayable_reserve_fixtures(this).iter().map(|f| f[..0x14].try_into().unwrap()).collect();
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CANCEL_UNPLAYABLE_RESERVE_FIXTURES)(this);
    let mut left = 0;
    let mut k = 0i8;
    while k < read_i8(G_KEY_NATION_ACTIVE_COUNT) {
        let list = exe::fix_man_get_fixture_list_for_date(read_usize(G_FIXTURE_MANAGER), G_CURRENT_DATE, read_u8(G_CURRENT_DAY_SLICE) as u32, k as u8 as u32);
        if list != 0 {
            write_u32(list + llist::CURSOR, read_u32(list + llist::HEAD));
            loop {
                let fixture = exe::llist_get_next(list);
                if fixture == 0 {
                    break;
                }
                if expected.iter().any(|e| core::slice::from_raw_parts(fixture as *const u8, 0x14) == &e[..]) {
                    left += 1;
                }
            }
        }
        k += 1;
    }
    SCOREBOARD.record(left == 0, || format!("cancel_unplayable_reserve_fixtures: {} of {} unplayable fixtures still there", left, expected.len()));
}

/// Comparer for `reserve_comp_create_sub_comps`: it allocates the sub competitions (it cannot run twice); only the
/// original runs and the table slots it leaves are checked.
pub(crate) unsafe extern "thiscall" fn compare_reserve_comp_create_sub_comps(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_RESERVE_COMP_CREATE_SUB_COMPS)(this);
    let slot = |key: usize| read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add((read_i32(key) as usize).wrapping_mul(4)));
    let (b, u21) = (read_usize(this + 0xa3), read_usize(this + 0xa7));
    let mut ok = slot(KEY_CLUB_COMP_RESERVE) == this && slot(KEY_CLUB_COMP_B_INTERNATIONAL) == b && slot(KEY_CLUB_COMP_UNDER_21_INTERNATIONAL) == u21
        && (b == 0 || (read_u32(b) == 0x96f074 && read_usize(b + 4) == reserve_comp_record(KEY_CLUB_COMP_B_INTERNATIONAL)))
        && (u21 == 0 || (read_u32(u21) == 0x96efe4 && read_usize(u21 + 4) == reserve_comp_record(KEY_CLUB_COMP_UNDER_21_INTERNATIONAL)));
    let french = slot(KEY_CLUB_COMP_FRENCH_FIRST_DIVISION);
    if french != 0 {
        ok &= slot(KEY_CLUB_COMP_FRENCH_CFA) == read_usize(french + 0xee) && slot(KEY_CLUB_COMP_FRENCH_LOWER_DIVISION) == read_usize(french + 0xf2);
    }
    let spanish = slot(KEY_CLUB_COMP_SPANISH_FIRST_DIVISION);
    if spanish != 0 {
        ok &= slot(KEY_CLUB_COMP_SPANISH_LOWER_DIVISION) == read_usize(spanish + 0xee);
    }
    SCOREBOARD.record(ok, || format!("reserve_comp_create_sub_comps({:#x}): B {:#x}, Under 21 {:#x}", this, b, u21));
}

// ---------- sx19_1: the South American ruling body (its constructor and its card rules) and the stepped card rules several countries share (ruling_body/rb_south_america.cpp) ----------

/// The South American ruling body: vtable 0x96ee34, record slot 2, rule set 2, ban scale 10, no nation, South America,
/// no league nor cup rule, itself as the continental rule.
pub(crate) const SOUTH_AMERICA_RULING_BODY: RulingBodyClass = RulingBodyClass { vtable: 0x96ee34, record_slot: 2, rule_set: 2, scale: 0xa,
    nation: None, continent: Some(KEY_CONTINENT_SOUTH_AMERICA), league_rule: 0xff, cup_rule: 0xff, continental_rule: 2 };

/// Builds the South American ruling body (`ruling_body_class_construct` with `SOUTH_AMERICA_RULING_BODY`). Returns this.
/// Original: `RbSouthAmerica::RbSouthAmerica` (0x7e29e0, rb_south_america.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn rb_south_america_construct(this: usize, file: usize) -> usize {
    covered!("sx19_1 south american body built");
    ruling_body_class_construct(this, file, &SOUTH_AMERICA_RULING_BODY)
}

/// South America's card rules (`plain_card_verdict`): one match for a red card and one for every second booking
/// (counted one per match, amount 2), from today, under the South American rule set in the confederation slot (rules
/// ff ff 02 ff).
pub(crate) unsafe fn south_america_card_verdict(row: MatchRecordRow, entry: usize) -> CardVerdict {
    covered!("sx19_1 south american cards");
    plain_card_verdict(row, entry, false, 2, own_rules(&SOUTH_AMERICA_RULING_BODY)) // sx19_1: every second South American booking
}

/// Applies a player's cards of one match under the South American rules (`south_america_card_verdict`): nothing
/// without a booking or a red card; the ban, when there is one, starts today and goes to the discipline with its news.
/// Original: `RbSouthAmerica::process_match_cards` (0x7e2a30, rb_south_america.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn rb_south_america_process_match_cards(this: usize, row: usize, fixture: usize) {
    run_card_rules(this, row, fixture, |e| south_america_card_verdict(MatchRecordRow(row), e));
}

/// The rules (league, cup, continental, nation slots) of a country's ban under its own rule set: the rule set in the
/// body's record slot (0 league, 1 cup), 0xff in the others.
pub(crate) fn own_rules(class: &RulingBodyClass) -> [u8; 4] {
    let mut r = [0xff; 4];
    r[class.record_slot as usize] = class.rule_set;
    r
}

/// The stepped card rules several countries share (Portugal, Spain, Turkey, Wales). Exactly one red card queues a
/// points ban (`pending_points_ban`, from tomorrow) and bans for one match (reason 2, amount 1). A booking adds one to
/// the entry's count (one per match), and every `every`-th booking bans for one more match (reason 1, amount `every`).
/// The ban starts today; the pending and the immediate ban apply under the body's own rule set (`own_rules`).
pub(crate) unsafe fn stepped_card_verdict(this: usize, row: MatchRecordRow, fixture: usize, entry: usize, class: &RulingBodyClass, every: u8) -> CardVerdict {
    let rules = own_rules(class);
    let (mut matches, mut reason, mut amount, mut pending) = (0u8, 0xffu8, 0i16, Vec::new());
    if row.red_card() == 1 {
        covered!("sx19_1 stepped red card");
        pending.extend(pending_points_ban(this, row, fixture, rules, || ban_start_in(1))); // sx19_1: the points ban from tomorrow
        matches = 1;
        reason = 2;
        amount = 1;
    }
    if row.yellow_cards() != 0 {
        let count = read_u8(entry + ruling_body_entry::YELLOW_CARDS).wrapping_add(1);
        write_u8(entry + ruling_body_entry::YELLOW_CARDS, count);
        if count % every == 0 {
            covered!("sx19_1 stepped booking ban");
            matches += 1;
            reason = 1;
            amount = every as i16;
        }
    }
    let ban = (matches != 0).then(|| ImmediateBan { matches, reason, amount, rules, start: ban_start_today() });
    CardVerdict { pending, ban }
}

pub(crate) unsafe extern "thiscall" fn compare_rb_south_america_construct(this: usize, file: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_RB_SOUTH_AMERICA_CONSTRUCT)(this, file);
    compare_ruling_body_class_built(r, this, &SOUTH_AMERICA_RULING_BODY, "rb_south_america_construct")
}

pub(crate) unsafe extern "thiscall" fn compare_rb_south_america_process_match_cards(this: usize, row: usize, fixture: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_RB_SOUTH_AMERICA_PROCESS_MATCH_CARDS);
    compare_card_rules(this, row, || o(this, row, fixture), |e| south_america_card_verdict(MatchRecordRow(row), e), "rb_south_america_process_match_cards");
}

// ---------- sx19_17: a ruling body's disciplinary entries: finding or adding a player's, attaching and detaching his ban (ruling_body/ruling_body.cpp) ----------

/// The size of a ruling body's disciplinary entry (`ruling_body_entry`).
const RULING_BODY_ENTRY_SIZE: usize = 0xe;

/// The address of entry `k` of the body's entries.
unsafe fn ruling_body_entry_at(this: usize, k: i32) -> usize {
    read_usize(this + ruling_body::ENTRIES).wrapping_add((k as usize).wrapping_mul(RULING_BODY_ENTRY_SIZE))
}

/// The player's index in the body's entries, from his discipline record (`table_penalty` on the body's record slot and
/// rule set); −1 when he has none.
unsafe fn ruling_body_entry_index(this: usize, id: u32) -> i32 {
    table_penalty(read_usize(G_DISCIPLINE), id as i32, read_u8(this + ruling_body::RECORD_SLOT) as i8, read_u8(this + ruling_body::RULE_SET_INDEX) as i8)
}

/// Returns the player's disciplinary entry in this ruling body, adding one when he has none and `create` is set.
///
/// The index comes from the player's discipline record (`ruling_body_entry_index`). Without one and without `create`:
/// 0. With `create`: a held ban of the body's rule set in the record slot (`Discipline::find_held_ban`) is the error
/// box of line 0x295 (and goes on); a full array grows by 100 entries (`realloc`, or `malloc` the first time; failing,
/// the error box of line 0x2b0 and 0); the new entry at the end is zeroed with the player's id and no ban (−1), and its
/// index goes into the player's record (`Discipline::set_slot`). An index outside the array clears the record's slot
/// (−1) and gives 0; otherwise the entry's address.
/// Original: `RulingBody::find_or_add_discipline_entry` (0x7e82e0, ruling_body.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn find_or_add_discipline_entry(this: usize, id: u32, create: u32) -> usize {
    let d = read_usize(G_DISCIPLINE);
    let (slot, rule) = (read_u8(this + ruling_body::RECORD_SLOT) as i8, read_u8(this + ruling_body::RULE_SET_INDEX));
    let mut k = ruling_body_entry_index(this, id);
    if k == -1 {
        if create == 0 {
            return 0;
        }
        covered!("sx19_17 entry added");
        let held = exe::discipline_find_held_ban(d, id as i32, slot);
        if held != 0 && read_u8(held + 5) == rule { // the held ban's rule set
            show_error_at(RULING_BODY_CPP, 0x295);
        }
        let count = read_i32(this + ruling_body::ENTRY_COUNT);
        if count == read_i32(this + ruling_body::ENTRY_CAPACITY) {
            let capacity = count + 100;
            write_i32(this + ruling_body::ENTRY_CAPACITY, capacity);
            let old = read_usize(this + ruling_body::ENTRIES);
            let size = (capacity as u32).wrapping_mul(RULING_BODY_ENTRY_SIZE as u32);
            let p = if old != 0 { exe::crt_realloc(old, size) } else { exe::crt_malloc(size) };
            write_usize(this + ruling_body::ENTRIES, p);
            if p == 0 {
                show_error_at(RULING_BODY_CPP, 0x2b0);
                return 0;
            }
        }
        let e = ruling_body_entry_at(this, count);
        core::ptr::write_bytes(e as *mut u8, 0, RULING_BODY_ENTRY_SIZE);
        write_i16(e + ruling_body_entry::BAN_INDEX, -1);
        write_u32(e + ruling_body_entry::PERSON_ID, id); // sx19_17: the new entry's player
        k = count;
        write_i32(this + ruling_body::ENTRY_COUNT, count + 1);
        exe::discipline_set_slot(d, id as i32, slot, rule, k as u32);
    }
    if k < read_i32(this + ruling_body::ENTRY_CAPACITY) && k >= 0 {
        return ruling_body_entry_at(this, k);
    }
    exe::discipline_set_slot(d, id as i32, slot, rule, u32::MAX);
    0
}

/// Links ban `ban` of the discipline's ban array to the player's entry (found or added). When the entry already points
/// to a ban, what is left of the old one goes onto the new one (its days when positive, otherwise its matches when
/// positive) and the old one is released (`Discipline::release_ban`).
/// Original: `RulingBody::attach_ban` (0x7e8510, ruling_body.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn attach_ban(this: usize, id: u32, ban: i16) {
    let e = exe::rulingbody_find_or_add_discipline_entry(this, id, 1);
    if e == 0 {
        return;
    }
    let old = read_i16(e + ruling_body_entry::BAN_INDEX);
    if old != -1 {
        covered!("sx19_17 ban merged");
        let d = read_usize(G_DISCIPLINE);
        let bans = read_usize(d + discipline_fields::BANS);
        let (o, n) = (bans.wrapping_add((old as isize * 20) as usize), bans.wrapping_add((ban as isize * 20) as usize));
        let days = read_i16(o + ban_record::DAYS);
        if days > 0 {
            write_i16(n + ban_record::DAYS, read_i16(n + ban_record::DAYS).wrapping_add(days));
        } else if read_i8(o + ban_record::MATCHES) > 0 { // sx19_17: the old ban's matches go onto the new one
            write_u8(n + ban_record::MATCHES, read_u8(n + ban_record::MATCHES).wrapping_add(read_u8(o + ban_record::MATCHES)));
        }
        exe::discipline_release_ban(d, id, old as u16 as u32);
    }
    write_i16(e + ruling_body_entry::BAN_INDEX, ban);
}

/// Unlinks the player's ban from his entry (−1), when he has an entry; called when the ban is released.
/// Original: `RulingBody::detach_ban` (0x7e8590, ruling_body.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn detach_ban(this: usize, id: u32) {
    let e = exe::rulingbody_find_or_add_discipline_entry(this, id, 0);
    if e != 0 {
        covered!("sx19_17 ban detached");
        write_i16(e + ruling_body_entry::BAN_INDEX, -1); // sx19_17: the entry without a ban
    }
}

/// Comparer of `find_or_add_discipline_entry`. A lookup (no `create`, or the player has an index) runs both and
/// compares the addresses. Adding cannot happen twice: only the original runs, and the new entry is checked (the
/// player's id, no ban, zero counts, one more entry).
pub(crate) unsafe extern "thiscall" fn compare_find_or_add_discipline_entry(this: usize, id: u32, create: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> usize>(IDX_FIND_OR_ADD_DISCIPLINE_ENTRY);
    if create == 0 || ruling_body_entry_index(this, id) != -1 {
        let r = o(this, id, create);
        let n = find_or_add_discipline_entry(this, id, create);
        SCOREBOARD.record(r == n, || format!("find_or_add_discipline_entry(player {}, {}): original {:#x} rust {:#x}", id, create, r, n));
        return r;
    }
    let before = read_i32(this + ruling_body::ENTRY_COUNT);
    let r = o(this, id, create);
    let ok = r == 0 || (read_u32(r + ruling_body_entry::PERSON_ID) == id && read_i16(r + ruling_body_entry::BAN_INDEX) == -1
        && read_u16(r + ruling_body_entry::YELLOW_CARDS) == 0 && read_u32(r + ruling_body_entry::DISCIPLINE_POINTS) == 0
        && read_i32(this + ruling_body::ENTRY_COUNT) == before + 1 && ruling_body_entry_index(this, id) == before);
    SCOREBOARD.record(ok, || format!("find_or_add_discipline_entry(player {}): new entry {:#x}, count {} (before {})", id, r,
        read_i32(this + ruling_body::ENTRY_COUNT), before));
    r
}

/// Comparer of `attach_ban`: the old ban is released only once, so only the original runs; checked afterwards: the
/// entry points to the new ban and the new ban carries the old one's days or matches.
pub(crate) unsafe extern "thiscall" fn compare_attach_ban(this: usize, id: u32, ban: i16) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, i16)>(IDX_ATTACH_BAN);
    let e = exe::rulingbody_find_or_add_discipline_entry(this, id, 1);
    if e == 0 {
        return o(this, id, ban);
    }
    let bans = read_usize(read_usize(G_DISCIPLINE) + discipline_fields::BANS);
    let at = |k: i16| bans.wrapping_add((k as isize * 20) as usize);
    let old = read_i16(e + ruling_body_entry::BAN_INDEX);
    let n = at(ban);
    let (mut days, mut matches) = (read_i16(n + ban_record::DAYS), read_u8(n + ban_record::MATCHES));
    if old != -1 {
        let o = at(old);
        if read_i16(o + ban_record::DAYS) > 0 {
            days = days.wrapping_add(read_i16(o + ban_record::DAYS));
        } else if read_i8(o + ban_record::MATCHES) > 0 {
            matches = matches.wrapping_add(read_u8(o + ban_record::MATCHES));
        }
    }
    o(this, id, ban);
    let e = exe::rulingbody_find_or_add_discipline_entry(this, id, 0);
    let ok = e != 0 && read_i16(e + ruling_body_entry::BAN_INDEX) == ban && read_i16(n + ban_record::DAYS) == days
        && read_u8(n + ban_record::MATCHES) == matches;
    SCOREBOARD.record(ok, || format!("attach_ban(player {}, ban {}): old {}, days {} matches {} (expected {} {})", id, ban, old,
        read_i16(n + ban_record::DAYS), read_u8(n + ban_record::MATCHES), days, matches));
}

/// Comparer of `detach_ban`: only the original runs; afterwards the player's entry, when there is one, has no ban.
pub(crate) unsafe extern "thiscall" fn compare_detach_ban(this: usize, id: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_DETACH_BAN)(this, id);
    let e = exe::rulingbody_find_or_add_discipline_entry(this, id, 0);
    SCOREBOARD.record(e == 0 || read_i16(e + ruling_body_entry::BAN_INDEX) == -1, || format!("detach_ban(player {}): entry {:#x} ban {}", id, e,
        if e != 0 { read_i16(e + ruling_body_entry::BAN_INDEX) } else { 0 }));
}

// ---------- sx19_18: whether a ruling body's ban covers a match, clearing a club's bookings and dropping a player's pending bans (ruling_body/ruling_body.cpp) ----------

/// Whether date `a` (day i16 +0, year u16 +2) comes before date `b`: the year unsigned, then the day signed.
unsafe fn ban_date_before(a: usize, b: usize) -> bool {
    let (ya, yb) = (read_u16(a + 2), read_u16(b + 2));
    ya < yb || (ya == yb && read_i16(a) < read_i16(b))
}

/// A ban that has not started yet against a match: the player's club in the fixture (home when the fixture's home club
/// is his club or his national team, otherwise away) and that side's mark pick the club's fixtures from the ban's start
/// to the match (`FIXTURE_MANAGER::find_fixtures` into a local list, any competition and kind, the side filter = the mark
/// is set); the unplayed ones (home goals 0xff) of a competition under the same discipline rule set as the fixture's are
/// counted. Covered when the ban's matches are at least that count; a ban starting after the match covers nothing.
unsafe fn ban_covers_later_match(ban: usize, person: Staff, fixture: Fixture) -> bool {
    let mut l = [0u8; 14];
    let list = Llist(l.as_mut_ptr() as usize);
    exe::new_llist(list);
    let mut md = [0u8; 8];
    exe::new_date(md.as_mut_ptr() as usize, fixture.day() as u32, fixture.year() as u32);
    let m = md.as_ptr() as usize;
    if ban_date_before(m, ban + ban_record::START_DATE) {
        exe::drop_llist(list);
        return false;
    }
    let home = fixture.home_club().addr();
    let (club, mark) = if read_usize(person.addr() + staff::CLUB_JOB) == home || read_usize(person.addr() + staff::NATIONAL_JOB) == home {
        (home, fixture.home_side_mark())
    } else {
        (fixture.away_club().addr(), fixture.away_side_mark() as u8)
    };
    exe::fixture_manager_find_fixtures(read_usize(G_FIXTURE_MANAGER), read_u16(ban + ban_record::START_DATE) as u32,
        read_u16(ban + ban_record::START_DATE + 2) as u32, read_u16(m) as u32, read_u16(m + 2) as u32, read_u32(club), list, u32::MAX,
        (-2i32) as u32, (mark != 0) as u32);
    let rule_of = |comp: i32| League(read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add((comp as usize).wrapping_mul(4)))).discipline_rule();
    list.set_cursor(list.head());
    let mut count = 0i16;
    let mut f = exe::llist_get_next(list);
    while f != 0 {
        let other = Fixture(f);
        if other.home_goals() as u8 == 0xff && other.comp_id() >= 0 && fixture.comp_id() >= 0 {
            let r = rule_of(other.comp_id());
            if r != 0xff && r == rule_of(fixture.comp_id()) { // sx19_18: a match of the same rule set still to play
                count += 1;
            }
        }
        f = exe::llist_get_next(list);
    }
    exe::drop_llist(list);
    read_i8(ban + ban_record::MATCHES) as i16 >= count
}

/// Whether this ruling body's active ban on the player covers a match.
///
/// No entry in the body (`ruling_body_entry_index`): no. A body without entries is the error box of line 0x330 and no.
/// First the player's bans move on (`Discipline::update_ban_progress` with the body's rule set and 0 matches); then
/// the entry's ban (none: no). With `flagged_only`, a ban without flag bit 1 does not count. A ban that has not started
/// yet (today before its start) only counts against a later match (`ban_covers_later_match`; no fixture: no). A
/// started ban by days covers any match without a fixture, and a fixture's match when the days from the ban's start
/// to the match are fewer than the ban's; a ban by matches covers when the matches served plus `extra` are at most its
/// matches.
/// Original: `RulingBody::is_player_banned` (0x7e85b0, ruling_body.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn is_player_banned(this: usize, person: usize, flagged_only: u32, extra: i16, fixture: usize) -> u32 {
    let d = read_usize(G_DISCIPLINE);
    let id = read_u32(person);
    let k = ruling_body_entry_index(this, id);
    if k == -1 {
        return 0;
    }
    if read_usize(this + ruling_body::ENTRIES) == 0 {
        show_error_at(RULING_BODY_CPP, 0x330);
        return 0;
    }
    exe::discipline_update_ban_progress(d, id, read_u8(this + ruling_body::RULE_SET_INDEX) as u32, 0);
    let b = read_i16(ruling_body_entry_at(this, k) + ruling_body_entry::BAN_INDEX);
    if b == -1 {
        return 0;
    }
    let ban = read_usize(d + discipline_fields::BANS).wrapping_add((b as isize * 20) as usize);
    if flagged_only != 0 && read_u8(ban + ban_record::FLAGS) & 2 == 0 {
        return 0;
    }
    if ban_date_before(G_CURRENT_DATE, ban + ban_record::START_DATE) {
        covered!("sx19_18 ban not started");
        return (fixture != 0 && ban_covers_later_match(ban, Staff(person), Fixture(fixture))) as u32;
    }
    if read_i16(ban + ban_record::DAYS) != -1 {
        if fixture == 0 {
            return 1;
        }
        covered!("sx19_18 ban by days");
        let mut md = [0u8; 8];
        exe::new_date(md.as_mut_ptr() as usize, Fixture(fixture).day() as u32, Fixture(fixture).year() as u32);
        let days = exe::date_days_since(md.as_ptr() as usize, read_u32(ban + ban_record::START_DATE), read_u32(ban + ban_record::START_DATE + 4));
        return (days < read_i16(ban + ban_record::DAYS) as i32) as u32;
    }
    covered!("sx19_18 ban by matches");
    (read_i8(ban + ban_record::MATCHES_SERVED) as i32 + extra as i32 <= read_i8(ban + ban_record::MATCHES) as i32) as u32 // sx19_18: matches still to serve
}

/// Clears the bookings of every player of the club in this body (all players with a null club); entries without a
/// player (−1) are skipped.
/// Original: `RulingBody::reset_bookings_for_club` (0x7e9180, ruling_body.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn reset_bookings_for_club(this: usize, club: usize) {
    let mut k = 0;
    while k < read_i32(this + ruling_body::ENTRY_COUNT) {
        let e = ruling_body_entry_at(this, k);
        let id = read_i32(e + ruling_body_entry::PERSON_ID);
        if id != -1 && (club == 0 || read_usize(staff_at(id as isize).addr() + staff::CLUB_JOB) == club) {
            covered!("sx19_18 bookings cleared");
            write_u8(e + ruling_body_entry::YELLOW_CARDS, 0); // sx19_18: the club's bookings cleared
        }
        k += 1;
    }
}

/// Removes every pending ban of the player from this body's list (`LLIST::remove` on the nodes whose person is his).
/// Original: `RulingBody::remove_pending_bans` (0x7e91d0, ruling_body.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn remove_pending_bans(this: usize, id: u32) {
    let list = Llist(read_usize(this + RULING_BODY_LIST));
    if list.is_null() {
        return;
    }
    list.set_cursor(list.head());
    let mut node = exe::llist_get_next(list);
    while node != 0 {
        if read_u32(node + pending_ban::PERSON_ID) == id { // sx19_18: the player's pending ban
            covered!("sx19_18 pending ban removed");
            exe::llist_remove(list, node);
        }
        node = exe::llist_get_next(list);
    }
}

/// Comparer of `is_player_banned`: the original runs first (it moves the player's bans on); the copy then asks the same
/// question (moving them on again changes nothing) and the answers are compared.
pub(crate) unsafe extern "thiscall" fn compare_is_player_banned(this: usize, person: usize, flagged_only: u32, extra: i16, fixture: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, i16, usize) -> u32>(IDX_IS_PLAYER_BANNED)(this, person, flagged_only, extra, fixture);
    let n = is_player_banned(this, person, flagged_only, extra, fixture);
    SCOREBOARD.record(r == n, || format!("is_player_banned(player {}, {}, {}, fixture {:#x}): original {} rust {}", read_u32(person), flagged_only, extra, fixture, r, n));
    r
}

/// Comparer of `reset_bookings_for_club`: the copy clears a copy of the entries, the original runs, the bytes are
/// compared.
pub(crate) unsafe extern "thiscall" fn compare_reset_bookings_for_club(this: usize, club: usize) {
    let entries = read_usize(this + ruling_body::ENTRIES);
    let n = read_i32(this + ruling_body::ENTRY_COUNT).max(0) as usize;
    let mut copy = core::slice::from_raw_parts(entries as *const u8, n * RULING_BODY_ENTRY_SIZE).to_vec();
    for k in 0..n {
        let e = copy.as_mut_ptr() as usize + k * RULING_BODY_ENTRY_SIZE;
        let id = read_i32(e + ruling_body_entry::PERSON_ID);
        if id != -1 && (club == 0 || read_usize(staff_at(id as isize).addr() + staff::CLUB_JOB) == club) {
            write_u8(e + ruling_body_entry::YELLOW_CARDS, 0);
        }
    }
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_RESET_BOOKINGS_FOR_CLUB)(this, club);
    let got = core::slice::from_raw_parts(entries as *const u8, n * RULING_BODY_ENTRY_SIZE);
    SCOREBOARD.record(got == &copy[..], || format!("reset_bookings_for_club({:#x}, club {:#x}): {} entries differ", this, club, n));
}

/// Comparer of `remove_pending_bans`: only the original runs; afterwards no node of the body's list is the player's.
pub(crate) unsafe extern "thiscall" fn compare_remove_pending_bans(this: usize, id: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_REMOVE_PENDING_BANS)(this, id);
    let list = Llist(read_usize(this + RULING_BODY_LIST));
    if list.is_null() {
        return;
    }
    let mut left = 0;
    list.set_cursor(list.head());
    let mut node = exe::llist_get_next(list);
    while node != 0 {
        if read_u32(node + pending_ban::PERSON_ID) == id {
            left += 1;
        }
        node = exe::llist_get_next(list);
    }
    SCOREBOARD.record(left == 0, || format!("remove_pending_bans(player {}): {} nodes left", id, left));
}

// ---------- sx19_19: a ruling body's day: the pending bans whose date has come (ruling_body/ruling_body.cpp) ----------

/// Whether a pending ban's start (node +0x14: day i16, year u16) is after `date`: the list is in date order, so the
/// day's work stops at the first such node.
unsafe fn pending_ban_after(node: usize, date: usize) -> bool {
    let (yn, yd) = (read_u16(node + 0x16), read_u16(date + 2));
    yd < yn || (yd == yn && read_i16(date) < read_i16(node + 0x14))
}

/// Applies the body's pending bans whose start date has come.
///
/// From the start of the body's list, until the first node that starts after `date`: the node's ban record takes the
/// node's start date (+0x14 into +4) and the player's entry is looked up (not added). Without an entry the node is just
/// removed. Otherwise the player's bans move on (`Discipline::update_ban_progress` with the body's rule set and 0
/// matches); without a ban in the body, the node's ban is added as it is (flag bit 0 cleared, +0x6f = 1,
/// `Discipline::add_ban`); with one, the node's matches (or days, for a ban by days) go onto it (replacing a 0xff or −1)
/// with flag bit 2, and the node gets flag bit 0. The ban news is posted (the player, the node and the node's fixture at
/// +0x20) and the node removed; each removal restarts the walk from the head.
/// Original: `RulingBody::daily_update` (0x7e7d50, ruling_body.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn ruling_body_daily_update(this: usize, date: usize) {
    let list = Llist(read_usize(this + RULING_BODY_LIST));
    if list.is_null() {
        return;
    }
    let d = read_usize(G_DISCIPLINE);
    list.set_cursor(list.head());
    let mut node = exe::llist_get_next(list);
    while node != 0 {
        if pending_ban_after(node, date) {
            return;
        }
        covered!("sx19_19 pending ban due");
        copy_record_date(node + ban_record::START_DATE, node + 0x14); // the pending ban starts on its date
        let id = read_u32(node + pending_ban::PERSON_ID);
        let e = exe::rulingbody_find_or_add_discipline_entry(this, id, 0);
        if e != 0 {
            exe::discipline_update_ban_progress(d, id, read_u8(this + ruling_body::RULE_SET_INDEX) as u32, 0);
            let b = read_i16(e + ruling_body_entry::BAN_INDEX);
            if b == -1 {
                write_u8(node + ban_record::FLAGS, read_u8(node + ban_record::FLAGS) & 0xfe);
                write_u8(node + 0x6f, 1); // the pending ban's byte +0x6f
                add_ban(d, id, node);
            } else {
                covered!("sx19_19 pending ban merged");
                write_u8(node + ban_record::FLAGS, read_u8(node + ban_record::FLAGS) | 1);
                let ban = read_usize(d + discipline_fields::BANS).wrapping_add((b as isize * 20) as usize);
                let matches = read_u8(node + ban_record::MATCHES);
                if matches != 0xff {
                    let old = read_u8(ban + ban_record::MATCHES);
                    write_u8(ban + ban_record::MATCHES, if old == 0xff { matches } else { old.wrapping_add(matches) }); // sx19_19: the pending matches added
                } else {
                    let (old, days) = (read_i16(ban + ban_record::DAYS), read_i16(node + ban_record::DAYS));
                    write_i16(ban + ban_record::DAYS, if old == -1 { days } else { old.wrapping_add(days) });
                }
                write_u8(ban + ban_record::FLAGS, read_u8(ban + ban_record::FLAGS) | 4);
            }
            let post: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(FN_POST_BAN_NEWS);
            post(d, staff_at(id as isize).addr(), node, node + 0x20);
        }
        exe::llist_remove(list, node);
        list.set_cursor(list.head());
        node = exe::llist_get_next(list);
    }
}

/// Comparer of the day: the bans and their news cannot be handed out twice, so only the original runs; checked
/// afterwards: the nodes due that day (the leading ones not after the date) left the list and the others stayed.
pub(crate) unsafe extern "thiscall" fn compare_ruling_body_daily_update(this: usize, date: usize) {
    let list = Llist(read_usize(this + RULING_BODY_LIST));
    let (mut due, mut total) = (0u16, 0u16);
    if !list.is_null() {
        total = list.count() as u16;
        list.set_cursor(list.head());
        let mut node = exe::llist_get_next(list);
        while node != 0 && !pending_ban_after(node, date) {
            due += 1;
            node = exe::llist_get_next(list);
        }
    }
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_RULING_BODY_DAILY_UPDATE)(this, date);
    let left = if list.is_null() { 0 } else { list.count() as u16 };
    SCOREBOARD.record(left == total - due, || format!("ruling_body_daily_update({:#x}): {} nodes left of {} ({} due)", this, left, total, due));
}

// --- sx20_1: the daily simulated matches of the clubs outside the playable leagues (simulated_stats.cpp,
// SimulatedStats::play_daily_matches 0x84be20) ---

/// A SimulatedStats group (`SIMULATED_GROUP_SIZE` bytes): +0 the club count, +4 up to `SIMULATED_GROUP_CLUBS` club ids.
/// The SimulatedStats object holds the group count at +0 and the group table at +4.
pub(crate) const SIMULATED_GROUP_SIZE: usize = 0x44;
pub(crate) const SIMULATED_GROUP_CLUBS: usize = 16;
/// The static match record the simulated matches are summarised into (built on the first call, with its two team
/// blocks from `match_record::HOME_TEAM`), and its guard byte (bit 0: built).
const SIMULATED_MATCH_RECORD: usize = 0xdd7168;
const SIMULATED_MATCH_RECORD_GUARD: usize = 0xdd7160;
/// The routines that take a played simulated match (`this`, the fixture, the match record), in the original's order:
/// PlayerStats::record_match, CONTRACT_MANAGER::update_after_match, Injury::apply_match_effects,
/// AWARD_MANAGER::process_fixture and the appearance clauses of TRANSFER_MANAGER.
const SIMULATED_MATCH_RECORDERS: [(usize, usize); 5] = [
    (G_PLAYER_STATS, 0x7c0d70),
    (G_CONTRACT_MANAGER, 0x4ca6d0),
    (G_INJURY, 0x62c080),
    (G_AWARD_MANAGER, 0x4134c0),
    (G_TRANSFER_MANAGER, 0x8c39f0),
];

/// The address of group `g` of a SimulatedStats object.
pub(crate) unsafe fn simulated_group(this: usize, g: usize) -> usize {
    read_usize(this + 4) + g * SIMULATED_GROUP_SIZE
}

/// The address of club slot `k` of group `g`.
unsafe fn simulated_group_slot(this: usize, g: usize, k: usize) -> usize {
    simulated_group(this, g) + 4 + k * 4
}

/// A random index near `index` among `count`: `index` minus random(count / 150) plus random(count / 150), drawn again
/// until it falls from 0 to `count` less one (the same draw the constructor's shuffle uses).
pub(crate) unsafe fn simulated_nearby_index(index: i32, count: impl Fn() -> i32) -> i32 {
    loop {
        let back = exe::utils_random(count() / 150);
        let near = index.wrapping_sub(back).wrapping_add(exe::utils_random(count() / 150));
        if near >= 0 && near < count() {
            return near;
        }
    }
}

/// Mixes the groups: as many times as there are groups, a random group and one near it (`simulated_nearby_index`)
/// swap a random club each.
unsafe fn simulated_stats_mix_groups(this: usize) {
    let groups = || read_i32(this);
    let mut i = 0;
    while i < groups() {
        let a = exe::utils_random(groups());
        let b = simulated_nearby_index(a, groups);
        let ka = exe::utils_random(read_i32(simulated_group(this, a as usize)));
        let kb = exe::utils_random(read_i32(simulated_group(this, b as usize)));
        let (sa, sb) = (simulated_group_slot(this, a as usize, ka as usize), simulated_group_slot(this, b as usize, kb as usize));
        let club = read_u32(sa);
        write_u32(sa, read_u32(sb));
        write_u32(sb, club);
        i += 1;
    }
}

/// How many players of `club_id` could play a simulated match today: -1 when the club has a fixture from two days
/// ago to three days ahead (`exe::fixture_manager_find_fixtures` with competition -2) or no nation; otherwise each of
/// the 50 squad slots with a player sheet that is not out injured (injury status 2) and not away on international
/// duty today.
unsafe fn simulated_stats_free_players(club_id: i32) -> i8 {
    let mut before = [0u8; 8];
    let mut after = [0u8; 8];
    let from = exe::date_sub_days(G_CURRENT_DATE, before.as_mut_ptr() as usize, 2);
    let to = exe::date_add_days(G_CURRENT_DATE, after.as_mut_ptr() as usize, 3); // sx20_1: three days ahead
    let busy = exe::fixture_manager_find_fixtures(read_usize(G_FIXTURE_MANAGER), read_u32(from), read_u32(from + 2), read_u32(to),
        read_u32(to + 2), club_id as u32, 0usize, u32::MAX, 0xffff_fffe, u32::MAX);
    let club = club_at(club_id as isize);
    if busy != 0 || club.nation().is_null() {
        return -1;
    }
    let mut free = 0i8;
    for k in 0..50 {
        let person = Staff(club.squad(k));
        if person.is_null() || person.player().is_null() {
            continue;
        }
        if exe::injury_get_player_injury_status(read_usize(G_INJURY), person) == 2 { // sx20_1: out injured
            continue;
        }
        if exe::nationalteams_get_international_duty_club(read_usize(G_NATIONAL_TEAMS), person, G_CURRENT_DATE, club, 0, 0) == 0 {
            free += 1;
        }
    }
    free
}

/// Plays one simulated match between the clubs `home` and `away` in the fixture `game` (kept by the caller, as the
/// original's stack slot): competition -2, no stadium, category 2, both side marks 1, score 0-0 and no half-time,
/// extra time, shoot-out or aggregate. Returns false when the match engine could not be built (the error box) or
/// set up; the original then retries the same pair.
unsafe fn simulated_stats_play_match(game: usize, home: i32, away: i32) -> bool {
    write_i32(game + fixture::HOME_CLUB_ID, home);
    write_u8(game + fixture::SQUAD_RULES, 0);
    write_i32(game + fixture::AWAY_CLUB_ID, away);
    write_usize(game + fixture::HOME_CLUB, club_at(home as isize).addr());
    write_usize(game + fixture::AWAY_CLUB, club_at(away as isize).addr());
    write_i32(game + fixture::COMP_ID, -2);
    write_usize(game + fixture::COMPETITION, 0);
    write_u8(game + fixture::HOME_SIDE_MARK, 1);
    write_u8(game + fixture::AWAY_SIDE_MARK, 1);
    write_usize(game + fixture::STADIUM, 0);
    write_i32(game + fixture::STADIUM_ID, -1);
    write_u8(game + fixture::NEUTRAL_VENUE, 0);
    write_u8(game + fixture::CATEGORY, 2); // sx20_1: the category of a simulated match
    write_u8(game + fixture::HOME_GOALS, 0);
    write_u8(game + fixture::AWAY_GOALS, 0);
    for o in fixture::HOME_GOALS_HALF_TIME..=fixture::AWAY_AGGREGATE {
        write_u8(game + o, 0xff);
    }
    exe::virtual_staff_fill_fixture_squads(read_usize(G_VIRTUAL_STAFF), game, -2, 0);
    let mut engine = exe::crt_operator_new(0xf63b);
    if engine != 0 {
        engine = exe::new_cm3_match_engine(engine);
    }
    if engine == 0 {
        show_error_at(0xa8278c, 0x15e);
        return false;
    }
    if exe::cm3_match_engine_init_match(engine, game, 1, 0usize, 0) == 0 {
        return false;
    }
    covered!("sx20_1 simulated match");
    exe::cm3_match_engine_run_match_steps(engine, -1);
    exe::match_stats_fill_match_record(read_usize(G_MATCH_STATS), engine, SIMULATED_MATCH_RECORD, 0);
    for (owner, routine) in SIMULATED_MATCH_RECORDERS { // sx20_1: the routines that take the match
        let record: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(routine);
        record(read_usize(owner), game, SIMULATED_MATCH_RECORD);
    }
    exe::drop_cm3_match_engine(engine);
    exe::crt_operator_delete(engine);
    exe::virtual_staff_release_fixture(read_usize(G_VIRTUAL_STAFF), game);
    true
}

/// Plays one group's matches today: each club's free players (`simulated_stats_free_players`), the clubs sorted by
/// them (most first), the clubs with free players shuffled among themselves (count² random swaps, when more than two),
/// then the clubs paired in order (first with second, third with fourth...) while the first of the pair has free
/// players and the second is not busy.
unsafe fn simulated_stats_play_group(this: usize, g: usize, game: usize) {
    let count = || read_i32(simulated_group(this, g));
    let mut clubs = [0i32; SIMULATED_GROUP_CLUBS];
    let mut free = [0i8; SIMULATED_GROUP_CLUBS];
    let mut with_free = 0i8;
    let mut k = 0;
    while k < count() as usize {
        clubs[k] = read_i32(simulated_group_slot(this, g, k));
        free[k] = simulated_stats_free_players(clubs[k]);
        if free[k] > 0 {
            with_free += 1;
        }
        k += 1;
    }
    let n = count() as usize;
    for i in 0..n.saturating_sub(1) {
        for j in i + 1..n {
            if free[i] < free[j] { // sx20_1: most free players first
                free.swap(i, j);
                clubs.swap(i, j);
            }
        }
    }
    let rounds = with_free as i32 * with_free as i32;
    if rounds > 0 && with_free > 2 {
        for _ in 0..rounds {
            let a = exe::utils_random(with_free as i32) as usize;
            let b = exe::utils_random(with_free as i32) as usize;
            free.swap(a, b);
            clubs.swap(a, b);
        }
    }
    let mut k = 0usize;
    while (k as i32) < count() - 1 {
        if free[k] <= 0 || free[k + 1] <= -1 { // sx20_1: a pair needs free players
            break;
        }
        if simulated_stats_play_match(game, clubs[k], clubs[k + 1]) {
            k += 2;
        }
    }
}

/// Plays the day's simulated matches of the clubs outside the playable leagues. On the first call it builds the
/// static match record (its two team blocks, `match_team_block_construct`/`match_team_block_destruct`) and registers
/// its destruction (0x84c500) with `atexit`. Then it mixes the groups (`simulated_stats_mix_groups`) and plays the
/// groups whose index matches today's day of the year modulo 7 (`simulated_stats_play_group`), except in months 5
/// and 6 (`Date::get_month`). Every match is a fresh fixture (competition -2, category 2) played to the end by a new
/// match engine, then summarised into the static record and passed to the player statistics, contracts, injuries,
/// awards and appearance clauses (`SIMULATED_MATCH_RECORDERS`).
/// Defects of the original: when the match engine cannot be built (the error box, line 0x15e) or set up
/// (`init_match` 0, leaking the engine), the same pair is tried again without end; a club with 0 free players
/// can still be the second of a pair (only -1, a busy club, stops the pairing); the fixture's match statistics slot,
/// dates, referee, round and rule fields, stage, result marks, time slice, stage index and flags byte are never set:
/// the original passes what its stack holds there (the copy, what the previous match of the call left, else 0).
/// Original: `SimulatedStats::play_daily_matches` (0x84be20, simulated_stats.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn simulated_stats_play_daily_matches(this: usize) {
    let guard = read_u8(SIMULATED_MATCH_RECORD_GUARD);
    if guard & 1 == 0 {
        write_u8(SIMULATED_MATCH_RECORD_GUARD, guard | 1);
        exe::crt_vector_constructor_iterator(SIMULATED_MATCH_RECORD + match_record::HOME_TEAM, match_record::SIDE_SIZE as u32, 2, 0x453c30usize, 0x453c10usize);
        exe::crt_atexit(0x84c500usize);
    }
    if read_i32(this) > 0 {
        simulated_stats_mix_groups(this);
    }
    let mut game = [0u8; fixture::SIZE + 1];
    let mut g = 0i32;
    while g < read_i32(this) {
        let today = read_i16(G_CURRENT_DATE) as i32;
        if today % 7 == g % 7 { // sx20_1: each group plays one day in seven
            if exe::date_get_month(G_CURRENT_DATE) as i8 <= 4 || exe::date_get_month(G_CURRENT_DATE) as i8 >= 7 {
                simulated_stats_play_group(this, g as usize, game.as_mut_ptr() as usize);
            }
        }
        g += 1;
    }
}

/// Comparer for `simulated_stats_play_daily_matches`.
pub(crate) unsafe extern "thiscall" fn compare_simulated_stats_play_daily_matches(this: usize) {
    // simplification: plays matches and draws random numbers (cannot run twice); the copy is proven by the career
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_SIMULATED_STATS_PLAY_DAILY_MATCHES)(this)
}

// --- sx20_2: building and freeing the simulated statistics groups (simulated_stats.cpp, SimulatedStats::SimulatedStats
// 0x84b940 and SimulatedStats::~SimulatedStats 0x84be10) ---

/// simulated_stats.cpp, the source path of the error boxes.
pub(crate) const SIMULATED_STATS_SOURCE: usize = 0xa8278c;
/// The two bytes of the constructor's CD search that a no-CD exe patches (as `cm0102-1080`: the drive type's `jne`
/// made `nop`, the found test made `xor`).
const SIMULATED_STATS_CD_DRIVE_TYPE_JUMP: usize = 0x84b9a0;
const SIMULATED_STATS_CD_FOUND_TEST: usize = 0x84b9e3;

/// The CD search some constructors start with: each CD drive (`GetLogicalDriveStringsA`, `GetDriveTypeA` = 5) is
/// looked at for "<root>cm0102.exe" (`file_find_first`) until one has it, with the "Insert CD" box (OK) in between. A
/// no-CD exe patched in the calling routine is honoured by reading its two patched bytes: the drive type's `jne` at
/// `drive_type_jump` made `nop` (any drive) and the found test at `found_test` no longer a `test` (always found). The
/// same search as `club_history_find_cd`, for any routine's addresses.
pub(crate) unsafe fn wait_for_game_cd(drive_type_jump: usize, found_test: usize) {
    let drives: unsafe extern "system" fn(u32, *mut u8) -> u32 = core::mem::transmute(read_usize(0x9670f8));
    let kind: unsafe extern "system" fn(usize) -> u32 = core::mem::transmute(read_usize(0x967100));
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let any_drive = read_u8(drive_type_jump) == 0x90;
    let always_found = read_u8(found_test) != 0x85;
    loop {
        // simplification: the drive list starts zeroed (as club_history_find_cd)
        let mut list = [0u8; 0x100];
        if drives(200, list.as_mut_ptr()) != 0 && list[0] != 0 {
            let mut i = 0usize;
            loop {
                let root = list.as_ptr() as usize + i;
                if any_drive || kind(root) == 5 {
                    let (mut name, mut found) = ([0u8; 0x104], [0u8; 0x114]);
                    format(name.as_mut_ptr() as usize, 0x987e00, root, 0x63u32, 0x6du32, 0x30u32, 0x31u32, 0x30u32, 0x32u32, 0x2eu32, 0x65u32, 0x78u32, 0x65u32);
                    if exe::file_find_first(name.as_ptr() as usize, found.as_mut_ptr() as usize, 0) == 0 || always_found {
                        return;
                    }
                    exe::file_find_close(found.as_mut_ptr() as usize);
                }
                // simplification: the original steps with a signed byte (only past 32 drives would it differ)
                i += 4;
                if list[i] == 0 {
                    break;
                }
            }
        }
        covered!("sx20_2 insert CD");
        let (mut title, mut message) = ([0u8; 0x100], [0u8; 0x100]);
        text(title.as_mut_ptr() as usize, 0x987df4); // "Insert CD"
        text(message.as_mut_ptr() as usize, 0x987dc4); // "Please insert the CM01/02 CD and click on OK"
        exe::gui_show_message_box(title.as_ptr() as usize, message.as_ptr() as usize, 1);
    }
}

/// Whether a club's division or reserve division is a competition the game plays (its object in
/// `G_COMP_OBJECT_TABLE`).
unsafe fn simulated_stats_is_played(division: Competition) -> bool {
    !division.is_null() && read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add((read_i32(division.addr()) as usize).wrapping_mul(4))) != 0
}

/// Writes into `list` the ids of the clubs that play simulated matches: neither their division nor their reserve
/// division is played and they are not a national or all-star team. Returns how many.
unsafe fn simulated_stats_collect_clubs(list: usize) -> i32 {
    let mut n = 0i32;
    for i in 0..read_i32(G_CLUB_COUNT) {
        let club = club_at(i as isize);
        if simulated_stats_is_played(club.division()) || simulated_stats_is_played(club.reserve_division()) {
            continue;
        }
        if exe::nationalteams_is_national_or_all_star_team(read_usize(G_NATIONAL_TEAMS), club) != 0 {
            continue;
        }
        write_i32(list + 4 * n as usize, read_i32(club.addr())); // the club id (+0)
        n += 1;
    }
    n
}

/// Sorts the club ids of `list` by reputation, highest first (a pairwise exchange sort, as the original).
unsafe fn simulated_stats_sort_by_reputation(list: usize, n: i32) {
    for i in 0..(n - 1).max(0) as usize {
        for j in i + 1..n as usize {
            let (a, b) = (read_i32(list + 4 * i), read_i32(list + 4 * j));
            if club_at(a as isize).reputation() < club_at(b as isize).reputation() { // sx20_2: highest reputation first
                write_i32(list + 4 * i, b);
                write_i32(list + 4 * j, a);
            }
        }
    }
}

/// Shuffles the sorted clubs a little: n / 2 times a random club swaps with one near it (`simulated_nearby_index`).
unsafe fn simulated_stats_shuffle_clubs(list: usize, n: i32) {
    let swaps = n / 2; // sx20_2: how many swaps
    for _ in 0..swaps {
        let a = exe::utils_random(n);
        let b = simulated_nearby_index(a, || n);
        let club = read_i32(list + 4 * b as usize);
        write_i32(list + 4 * b as usize, read_i32(list + 4 * a as usize));
        write_i32(list + 4 * a as usize, club);
    }
}

/// Deals the clubs of `list` in order into the groups: 16 each, except that, when the club count is not a multiple
/// of 16 (`rest` = count % 16), the first 16 − `rest` groups take 15. Running out of clubs shows the error box (line
/// 0x95) once and goes on reading past the list.
unsafe fn simulated_stats_fill_groups(this: usize, list: usize, n: i32, rest: i32) {
    let mut next = 0i32;
    for g in 0..read_i32(this) {
        let group = simulated_group(this, g as usize);
        write_i32(group, 0);
        for slot in 0..SIMULATED_GROUP_CLUBS as i32 {
            if next == n {
                show_error_at(SIMULATED_STATS_SOURCE, 0x95);
            }
            let count = read_i32(group);
            write_i32(group + 4 + 4 * count as usize, read_i32(list.wrapping_add((next as usize).wrapping_mul(4))));
            write_i32(group, count + 1);
            next += 1;
            if rest != 0 && g < 16 - rest && slot == 14 { // sx20_2: the short groups take 15
                break;
            }
        }
    }
}

/// Builds the simulated statistics: first waits for the game CD (unless the interactive mode flag
/// `G_INTERACTIVE_MODE_FLAG` is set: `wait_for_game_cd`). With a saved game `save`, loads the groups from it
/// (`SimulatedStats::load`). Otherwise takes the clubs outside the played competitions that are not national teams
/// (`simulated_stats_collect_clubs`; without memory for the list, the error box, line 0x49, the fatal cleanup and
/// `exit(-1)`), the group count = ⌈count / 16⌉ at +0, sorts them by reputation, shuffles them a little and deals them
/// into a new table of groups at +4 (`simulated_stats_fill_groups`). Returns `this`.
/// Defects of the original: with fewer than 16 × (16 − count % 16) clubs the short groups ask for more clubs than
/// there are (the box, then ids read past the list); the group table is not checked for null.
/// Original: `SimulatedStats::SimulatedStats` (0x84b940, simulated_stats.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn simulated_stats_simulated_stats(this: usize, save: usize) -> usize {
    if read_u32(G_INTERACTIVE_MODE_FLAG) == 0 {
        wait_for_game_cd(SIMULATED_STATS_CD_DRIVE_TYPE_JUMP, SIMULATED_STATS_CD_FOUND_TEST);
    }
    if save != 0 {
        let load: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x84c510usize); // SimulatedStats::load
        load(this, save);
        return this;
    }
    let list = exe::crt_malloc(read_u32(G_CLUB_COUNT).wrapping_mul(4));
    if list == 0 {
        fatal_error_at(SIMULATED_STATS_SOURCE, 0x49);
    }
    let n = simulated_stats_collect_clubs(list);
    let rest = n % 16;
    write_i32(this, n / 16 + (rest != 0) as i32);
    covered!("sx20_2 simulated stats groups");
    simulated_stats_sort_by_reputation(list, n);
    simulated_stats_shuffle_clubs(list, n);
    write_usize(this + 4, exe::crt_malloc((read_i32(this) as u32).wrapping_mul(SIMULATED_GROUP_SIZE as u32)));
    simulated_stats_fill_groups(this, list, n, rest);
    exe::crt_free(list);
    this
}

/// Frees the group table (+4) when there is one.
/// Original: `SimulatedStats::~SimulatedStats` (0x84be10, simulated_stats.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn simulated_stats_destructor(this: usize) {
    if read_usize(this + 4) != 0 {
        exe::crt_free(read_usize(this + 4));
    }
}

/// Comparer for `simulated_stats_simulated_stats`.
pub(crate) unsafe extern "thiscall" fn compare_simulated_stats_simulated_stats(this: usize, save: usize) -> usize {
    // simplification: allocates and draws random numbers (cannot run twice); the copy is proven by the career
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_SIMULATED_STATS_SIMULATED_STATS)(this, save)
}

/// Comparer for `simulated_stats_destructor`.
pub(crate) unsafe extern "thiscall" fn compare_simulated_stats_destructor(this: usize) {
    // simplification: frees memory (cannot run twice)
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_SIMULATED_STATS_DESTRUCTOR)(this)
}
