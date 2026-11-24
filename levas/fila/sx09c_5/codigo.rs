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
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, save)
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
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this)
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
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32>(%%I2%%)(this, row, fixture, unused);
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
