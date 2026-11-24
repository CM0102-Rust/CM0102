// ---------- batch %%LEVA%%: the board object and the loading of man_conf.dat (manager_manager.cpp) ----------

/// The source path of manager_manager.cpp in the "Error" box ("E:\dev\CM3\cm3 00-01\cm3\code\manager_manager.cpp").
const MANAGER_MANAGER_CPP_PATH: usize = 0x9f18a4;
/// The save item "man_conf.dat".
const MAN_CONF_DAT: usize = 0x9f18d8;
/// The game data versions of the man_conf.dat layout: board records of 0x49 bytes above the first (before, 0x41 bytes
/// without the last ultimatum date); the ultimatum and ban appeal lists above the second; 0x19-byte ban appeals from
/// the third (before, 0x15 bytes).
const MAN_CONF_49_BYTE_RECORDS_VERSION: i32 = 0x3e040;
const MAN_CONF_LISTS_VERSION: i32 = 0x3e033;
const MAN_CONF_25_BYTE_APPEALS_VERSION: i32 = 0x3e035;
/// The bytes the original patches to drop the CD requirement in this routine: the drive type's `jne` (75 56, `nop`s in
/// cm0102-1080) and the found test (85 c0, `xor` in cm0102-1080).
const MANAGER_MANAGER_CD_TYPE_JUMP: usize = 0x68dc41;
const MANAGER_MANAGER_CD_FOUND_TEST: usize = 0x68dc85;

/// The CD search at the start of the constructor: each CD drive (`GetLogicalDriveStringsA`, `GetDriveTypeA` = 5) is
/// looked at for "<root>cm0102.exe" (`exe::file_find_first`) until one has it, with the "Insert CD" box (OK) in
/// between. A no-CD exe patched in this routine (as cm0102-1080) is honoured by reading those bytes.
unsafe fn manager_manager_find_cd() {
    let drives: unsafe extern "system" fn(u32, *mut u8) -> u32 = core::mem::transmute(read_usize(0x9670f8));
    let kind: unsafe extern "system" fn(usize) -> u32 = core::mem::transmute(read_usize(0x967100));
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let any_drive = read_u8(MANAGER_MANAGER_CD_TYPE_JUMP) == 0x90;
    let always_found = read_u8(MANAGER_MANAGER_CD_FOUND_TEST) != 0x85;
    loop {
        // simplification: the buffer starts zeroed (as check_game_cd)
        let mut list = [0u8; 0x100];
        if drives(200, list.as_mut_ptr()) != 0 && list[0] != 0 {
            let mut i = 0usize;
            loop {
                let root = list.as_ptr() as usize + i;
                let t = kind(root);
                if any_drive || t == 5 {
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
        covered!("sx14b_2 board insert CD");
        let (mut title, mut message) = ([0u8; 0x100], [0u8; 0x100]);
        text(title.as_mut_ptr() as usize, 0x987df4); // "Insert CD"
        text(message.as_mut_ptr() as usize, 0x987dc4); // "Please insert the CM01/02 CD and click on OK"
        exe::gui_show_message_box(title.as_ptr() as usize, message.as_ptr() as usize, 1);
    }
}

/// The five owned lists, in the original's order: vacancies (+8), departures (+4), +0xc (0xe-byte items), ultimatums
/// (+0x10) and ban appeals (+0x14); each `operator new(0xe)` + `LLIST::LLIST`, 0 when the allocation fails.
unsafe fn manager_manager_new_lists(this: usize) {
    use manager_manager_fields as f;
    for field in [f::VACANCY_LIST, f::DEPARTURE_LIST, 0xc, f::ULTIMATUM_LIST, f::BAN_APPEAL_LIST] {
        let list = exe::crt_operator_new(0xe);
        write_usize(this + field, if list != 0 { exe::new_llist(list) } else { 0 });
    }
}

/// The board record table for every club (`G_CLUB_COUNT` × 0x49 bytes, `malloc`); out of memory, the fatal
/// manager_manager.cpp box at `row` (the box, `game_fatal_cleanup`, exit(−1)).
unsafe fn manager_manager_new_table(this: usize, row: u32) {
    let table = exe::crt_malloc(read_u32(G_CLUB_COUNT).wrapping_mul(0x49));
    write_usize(this + manager_manager_fields::BOARD_RECORDS, table);
    if table == 0 {
        let exit: unsafe extern "C" fn(i32) -> ! = core::mem::transmute(FN_CRT_EXIT);
        show_error_box(MANAGER_MANAGER_CPP_PATH, row);
        exe::game_fatal_cleanup();
        exe::shared_noop_ret0_5a8f60();
        exit(-1);
    }
}

/// Builds the board object (MANAGER_MANAGER, `G_MANAGER_MANAGER`): waits for the game CD, creates the five lists and,
/// with a save, reads them and the board records from man_conf.dat (`load_man_conf`); without a save, or when the
/// reading fails (everything it built freed, as `drop_manager_manager`, and the lists made again after the table),
/// the table for every club (`manager_manager_new_table`, fatal box at line 0x69 or 0x9f) filled fresh by
/// `init_club_board_records`. Returns the object.
/// Original: `MANAGER_MANAGER::MANAGER_MANAGER` (0x68dbf0, manager_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn new_manager_manager(this: usize, save: usize) -> usize {
    manager_manager_find_cd();
    manager_manager_new_lists(this);
    if save == 0 {
        manager_manager_new_table(this, 0x69);
    } else {
        if load_man_conf(this, save) != 0 {
            covered!("sx14b_2 board from the save");
            return this;
        }
        covered!("sx14b_2 board save unreadable");
        drop_manager_manager(this);
        manager_manager_new_table(this, 0x9f);
        manager_manager_new_lists(this);
    }
    init_club_board_records(this);
    this
}

/// Comparer for `new_manager_manager`: it allocates and reads the save item, so it cannot run twice; only the original
/// runs. The copy is proven by the new career (Rust mode) and by loading the holiday save (levas/N/CARGA).
pub(crate) unsafe extern "thiscall" fn compare_new_manager_manager(this: usize, save: usize) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, save)
}

/// Frees the board object: the board record table (`free`) and the five lists (`LLIST::~LLIST` and `operator
/// delete`), in the order +8, +4, +0xc, +0x10, +0x14; each present one freed and its pointer zeroed.
/// Original: `MANAGER_MANAGER::~MANAGER_MANAGER` (0x68e0e0, manager_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn drop_manager_manager(this: usize) {
    use manager_manager_fields as f;
    let table = read_usize(this + f::BOARD_RECORDS);
    if table != 0 {
        exe::crt_free(table);
        write_usize(this + f::BOARD_RECORDS, 0);
    }
    for field in [f::VACANCY_LIST, f::DEPARTURE_LIST, 0xc, f::ULTIMATUM_LIST, f::BAN_APPEAL_LIST] {
        let list = read_usize(this + field);
        if list != 0 {
            exe::drop_llist(list);
            exe::crt_operator_delete(list);
            write_usize(this + field, 0);
        }
    }
}

/// Comparer for `drop_manager_manager`: it frees, so it cannot run twice; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_drop_manager_manager(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this)
}

/// Converts the old 0x41-byte board records into the 0x49-byte `board_record`: the bytes as they are (the date pairs
/// through `Date::operator=`), except that +3 takes only its 5 low bits (the 3 high bits keep what `malloc` left, as
/// in the original), and `board_record::LAST_ULTIMATUM_DATE` becomes "never" (23/06/1971).
unsafe fn man_conf_convert_old_records(table: usize, old: usize, count: i32) {
    for k in 0..count.max(0) as usize {
        let (o, n) = (old + k * 0x41, table + k * 0x49);
        write_u8(n, read_u8(o));
        write_u8(n + 1, read_u8(o + 1));
        write_u8(n + 2, read_u8(o + 2));
        write_u8(n + 3, (read_u8(n + 3) & 0xe0) | (read_u8(o + 3) & 0x1f));
        core::ptr::copy_nonoverlapping((o + 4) as *const u8, (n + 4) as *mut u8, 0x14);
        let mut tmp = [0u8; 8];
        exe::date_operator_3d(n + board_record::LAST_REVIEW_DATE, tmp.as_mut_ptr() as usize, o + board_record::LAST_REVIEW_DATE);
        exe::date_operator_3d(n + board_record::JOB_APPLICATION_DATE, tmp.as_mut_ptr() as usize, o + board_record::JOB_APPLICATION_DATE);
        core::ptr::copy_nonoverlapping((o + 0x28) as *const u8, (n + 0x28) as *mut u8, 0x18);
        write_u8(n + board_record::DISCIPLINE_COUNTERS, read_u8(o + board_record::DISCIPLINE_COUNTERS));
        let mut never = [0u16; 4];
        let d = exe::date_date_549f70(&mut never, 0x17, 6, 0x7b3, u32::MAX);
        exe::date_operator_3d(n + board_record::LAST_ULTIMATUM_DATE, tmp.as_mut_ptr() as usize, d);
    }
}

/// Reads one list of man_conf.dat: the count (dword), then each item of `size` bytes appended to `list`
/// (`LLIST::add_tail`); false on a short read.
unsafe fn man_conf_read_list(z: usize, list: usize, size: u32) -> bool {
    let mut count = 0i32;
    if exe::zread(&mut count as *mut i32 as usize, 4, 1, z) != 1 {
        return false;
    }
    let mut item = [0u8; 0x26];
    for _ in 0..count {
        if exe::zread(item.as_mut_ptr() as usize, size, 1, z) != 1 {
            return false;
        }
        exe::llist_add_tail(list, item.as_mut_ptr() as usize, size);
    }
    true
}

/// The ban appeal list with the old 0x15-byte items (before `MAN_CONF_25_BYTE_APPEALS_VERSION`): each becomes the
/// 0x19-byte item with its 0x15 bytes (the date through `Date::operator=`) and 0xff in the four new ones.
unsafe fn man_conf_read_old_appeals(z: usize, list: usize, count: i32) -> bool {
    for _ in 0..count {
        let (mut old, mut item) = ([0u8; 0x15], [0u8; 0x19]);
        if exe::zread(old.as_mut_ptr() as usize, 0x15, 1, z) != 1 {
            return false;
        }
        item[..5].copy_from_slice(&old[..5]);
        let mut tmp = [0u8; 8];
        exe::date_operator_3d(item.as_mut_ptr() as usize + 5, tmp.as_mut_ptr() as usize, old.as_mut_ptr() as usize + 5);
        item[0xd..0x15].copy_from_slice(&old[0xd..0x15]);
        item[0x15..].fill(0xff);
        exe::llist_add_tail(list, item.as_mut_ptr() as usize, 0x19);
    }
    true
}

/// The body of `load_man_conf` with the item open: false on a short read, an empty table or a failed allocation.
unsafe fn man_conf_read(this: usize, z: usize) -> bool {
    use manager_manager_fields as f;
    let mut count = 0i32;
    if exe::zread(&mut count as *mut i32 as usize, 4, 1, z) != 1 || count == 0 {
        return false;
    }
    let table = exe::crt_malloc((count as u32).wrapping_mul(0x49));
    write_usize(this + f::BOARD_RECORDS, table);
    if table == 0 {
        return false;
    }
    let version = read_i32(G_GAME_DATA_VERSION).wrapping_abs();
    if version > MAN_CONF_49_BYTE_RECORDS_VERSION {
        if exe::zread(table, 0x49, count as u32, z) != count as u32 {
            return false;
        }
    } else {
        covered!("sx14b_2 old board records");
        let old = exe::crt_malloc((count as u32).wrapping_mul(0x41));
        if old == 0 {
            return false;
        }
        if exe::zread(old, 0x41, count as u32, z) != count as u32 {
            exe::crt_free(old);
            return false;
        }
        man_conf_convert_old_records(table, old, count);
        exe::crt_free(old);
    }
    if !man_conf_read_list(z, read_usize(this + f::VACANCY_LIST), 0x26)
        || !man_conf_read_list(z, read_usize(this + f::DEPARTURE_LIST), 6)
        || !man_conf_read_list(z, read_usize(this + 0xc), 0xe)
    {
        return false;
    }
    if version <= MAN_CONF_LISTS_VERSION {
        return true;
    }
    if !man_conf_read_list(z, read_usize(this + f::ULTIMATUM_LIST), 4) {
        return false;
    }
    let appeals = read_usize(this + f::BAN_APPEAL_LIST);
    if version >= MAN_CONF_25_BYTE_APPEALS_VERSION {
        return man_conf_read_list(z, appeals, 0x19);
    }
    let mut n = 0i32;
    if exe::zread(&mut n as *mut i32 as usize, 4, 1, z) != 1 {
        return false;
    }
    covered!("sx14b_2 old ban appeals");
    man_conf_read_old_appeals(z, appeals, n)
}

/// Reads the board object from the save's "man_conf.dat": nothing (0) without a save or the item. The club count and
/// the board records (`board_record`, `malloc`; 0x41-byte records before `MAN_CONF_49_BYTE_RECORDS_VERSION`,
/// converted), then the lists: vacancies (0x26-byte items), departures (6), +0xc (0xe) and, from
/// `MAN_CONF_LISTS_VERSION`, the ultimatums (4) and the ban appeals (0x19, or 0x15 converted before
/// `MAN_CONF_25_BYTE_APPEALS_VERSION`). The item is closed in every case after it opened; 1 when everything was read,
/// 0 on a short read, an empty table (club count 0) or a failed allocation (with no box: the constructor then builds
/// the board fresh).
/// Original: `MANAGER_MANAGER::load_man_conf` (0x68e180, manager_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn load_man_conf(this: usize, save: usize) -> u32 {
    if save == 0 {
        return 0;
    }
    let mut z = exe::zopen(save, MAN_CONF_DAT, 0, 0x16, 4);
    if z == 0 {
        return 0;
    }
    covered!("sx14b_2 load man_conf.dat");
    // the local dates the original builds with Date::Date (0x54a110) and never reads are left out
    let ok = man_conf_read(this, z);
    exe::zclose(&mut z);
    ok as u32
}

/// Comparer for `load_man_conf`: it reads the save item and allocates, so it cannot run twice; only the original runs.
/// The copy is proven by loading the holiday save in the save proof (levas/N/CARGA).
pub(crate) unsafe extern "thiscall" fn compare_load_man_conf(this: usize, save: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I2%%)(this, save)
}
