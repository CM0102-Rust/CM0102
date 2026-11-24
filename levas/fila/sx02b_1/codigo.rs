// The Shortlist_Manager constructor (shortlist_manager.cpp, thiscall(this, load_stream), ret 4, returns this) and the
// transfer manager's own load from the saved game (thiscall(this, stream), ret 4, non-zero when loaded).
const FN_SHORTLIST_MANAGER_CONSTRUCTOR: usize = 0x839a80;
const FN_TRANSFER_MANAGER_LOAD_FROM_DISK: usize = 0x8bc020;

/// The 27 national transfer rules objects after the European one, in the order the original builds them: (slot in
/// `transfer_manager_fields::TRANSFER_RULES`, rules id, constructor, nation key). Slots 0x10 and 0x11 are built in
/// the opposite order (Norway before Northern Ireland). Each constructor is thiscall(this, id, nation, −1, 1, 0), ret
/// 0x14.
const NATIONAL_TRANSFER_RULES: [(usize, u32, usize, usize); 27] = [
    (1, 0x1, 0x40a580, KEY_NATION_ARGENTINA),
    (2, 0x2, 0x412c80, KEY_NATION_AUSTRALIA),
    (3, 0x3, 0x424b90, KEY_NATION_BELGIUM),
    (4, 0x4, 0x43f0e0, KEY_NATION_BRAZIL),
    (5, 0x5, 0x518500, KEY_NATION_CROATIA),
    (6, 0x6, 0x555460, KEY_NATION_DENMARK),
    (7, 0x7, 0x579b30, KEY_NATION_ENGLAND),
    (8, 0x8, 0x5a9950, KEY_NATION_FINLAND),
    (9, 0x9, 0x5c1570, KEY_NATION_FRANCE),
    (10, 0xa, 0x5dff70, KEY_NATION_GERMANY),
    (11, 0xb, 0x5eceb0, KEY_NATION_GREECE),
    (12, 0xc, 0x5f9690, KEY_NATION_HOLLAND),
    (13, 0xd, 0x63c210, KEY_NATION_REPUBLIC_OF_IRELAND),
    (14, 0xe, 0x661380, KEY_NATION_ITALY),
    (15, 0xf, 0x666bd0, KEY_NATION_JAPAN),
    (17, 0x11, 0x7944f0, KEY_NATION_NORWAY),
    (16, 0x10, 0x7936d0, KEY_NATION_NORTHERN_IRELAND),
    (18, 0x12, 0x7cd830, KEY_NATION_POLAND),
    (19, 0x13, 0x7d3d80, KEY_NATION_PORTUGAL),
    (20, 0x14, 0x7ec770, KEY_NATION_RUSSIA),
    (21, 0x15, 0x7f6390, KEY_NATION_SCOTLAND),
    (22, 0x16, 0x859400, KEY_NATION_SPAIN),
    (23, 0x17, 0x893d80, KEY_NATION_SWEDEN),
    (24, 0x18, 0x9020a0, KEY_NATION_TURKEY),
    (25, 0x19, 0x90b940, KEY_NATION_UNITED_STATES),
    (26, 0x1a, 0x90f080, KEY_NATION_WALES),
    (27, 0x1b, 0x66e630, KEY_NATION_SOUTH_KOREA),
];

/// The European rules constructor (slot 0): thiscall(this, 0, −1, continent, 2, 0), ret 0x14.
const FN_EUROPEAN_TRANSFER_RULES_CONSTRUCTOR: usize = 0x583280;

/// Size of a transfer rules object (`operator new` of each one).
const TRANSFER_RULES_OBJECT_SIZE: u32 = 0x19;

/// The CD search of a constructor: among the drives (GetLogicalDriveStringsA, 200 bytes) the first CD drive
/// (GetDriveTypeA = 5, always asked) holding "<root>cm0102.exe" (`exe::file_find_first`; on the others the search is
/// closed, `exe::file_find_close`); none: the "Insert CD" box (langlib_sprintf of 0x987df4 and 0x987dc4, the box with
/// 1) and again, forever. A no-CD exe (as cm0102-1080) patched this copy of the check: the `jne` on the drive type at
/// `drive_type_jump` made nops and the found test at `found_test` (85 c0) made xor; the copy reads those bytes of the
/// running exe.
unsafe fn find_game_cd(drive_type_jump: usize, found_test: usize) {
    let drives: unsafe extern "system" fn(u32, *mut u8) -> u32 = core::mem::transmute(read_usize(0x9670f8)); // GetLogicalDriveStringsA
    let kind: unsafe extern "system" fn(usize) -> u32 = core::mem::transmute(read_usize(0x967100)); // GetDriveTypeA
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let any_drive = read_u8(drive_type_jump) == 0x90;
    let always_found = read_u8(found_test) != 0x85;
    loop {
        // simplification: the buffer starts zeroed; the original would only read garbage with more than 200 bytes of drives
        let mut list = [0u8; 0x100];
        if drives(200, list.as_mut_ptr()) != 0 && list[0] != 0 {
            let mut i = 0usize;
            loop {
                let root = list.as_ptr() as usize + i;
                let t = kind(root);
                if any_drive || t == 5 {
                    let (mut name, mut found) = ([0u8; 0x104], [0u8; 0x114]);
                    let nm = name.as_mut_ptr() as usize;
                    format(nm, 0x987e00, root, 0x63u32, 0x6du32, 0x30u32, 0x31u32, 0x30u32, 0x32u32, 0x2eu32, 0x65u32, 0x78u32, 0x65u32);
                    let f = found.as_mut_ptr() as usize;
                    if exe::file_find_first(nm, f, 0) == 0 || always_found {
                        return;
                    }
                    exe::file_find_close(f);
                }
                // simplification: the original walks with an i8 (steps of 4): it would only pass 127 with more than 32 drives
                i += 4;
                if list[i] == 0 {
                    break;
                }
            }
        }
        let (mut title, mut msg) = ([0u8; 0x100], [0u8; 0x100]);
        text(title.as_mut_ptr() as usize, 0x987df4); // "Insert CD"
        text(msg.as_mut_ptr() as usize, 0x987dc4); // "Please insert the CM01/02 CD and click on OK"
        exe::gui_show_message_box(title.as_ptr() as usize, msg.as_ptr() as usize, 1);
    }
}

/// Builds the transfer manager, for a new game (`stream` 0) or from the saved game.
///
/// - The 150 day buckets at +0x18 (LLISTs of 14 bytes, `exe::crt_vector_constructor_iterator`), the date
/// `current_date()` (`exe::date_date_54a110`) and the inline lists `INSTALMENTS` and `FREE_TRANSFER_QUEUE`
/// (`exe::new_llist`).
/// - The CD check (`find_game_cd`, the no-CD patch read at 0x8bbaf2 and 0x8bbb34).
/// - The offers and negotiations (+0..+0x14), the past transfers, the next-sale percentages, both appearance money
/// arrays, the future transfers, the work permits (pointers, capacities and counts) and the loan negotiations zeroed;
/// `day_cursor()` 0; `current_date()` = a date built with 0, 0 (`exe::new_date`, `exe::date_operator_3d`);
/// `current_slice()` 0.
/// - The transfer rules (`setup_transfer_rules`).
/// - `person_flags()` = calloc(`G_STAFF_COUNT`, 8) and `season_loan_counts()` = calloc(`G_CLUB_COUNT`, 16); either
/// failing: the "Memory Allocation Error" box (lines 0xa3 and 0xab) and the construction stops there.
/// - `lowest_free_slot()` = −1.
/// - With a `stream`, the manager is read from the saved game (`FN_TRANSFER_MANAGER_LOAD_FROM_DISK`); when that
/// works, nothing else. Otherwise a new Shortlist_Manager (`exe::crt_operator_new` of 0xe0 bytes, its constructor with
/// 0) in `shortlist_manager()`; none: the box (line 0xc0) and stop; then the work permits (`init_work_permits`).
///
/// Returns this.
/// Original: `TRANSFER_MANAGER::TRANSFER_MANAGER` (0x8bba30, transfer_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn construct_transfer_manager(this: usize, stream: usize) -> usize {
    let tm = TransferManagerFields(this);
    exe::crt_vector_constructor_iterator(this + 0x18, 0xe, 0x96, FN_LLIST_LLIST, FN_LLIST_DESTRUCTOR_LLIST); // the day buckets
    exe::date_date_54a110(this + transfer_manager_fields::CURRENT_DATE);
    exe::new_llist(this + transfer_manager_fields::INSTALMENTS);
    exe::new_llist(this + transfer_manager_fields::FREE_TRANSFER_QUEUE);
    find_game_cd(0x8bbaf2, 0x8bbb34);
    for field in [0x0usize, 0x8, 0x4, 0xc, 0x14, 0x10, // the offer and negotiation arrays
                  transfer_manager_fields::PAST_TRANSFERS, transfer_manager_fields::PAST_TRANSFER_COUNT, 0x866, // 0x866: past transfer capacity
                  transfer_manager_fields::NEXT_SALE_PERCENTAGES, transfer_manager_fields::NEXT_SALE_PERCENTAGE_COUNT, 0x884, // 0x884: their capacity
                  transfer_manager_fields::APPEARANCE_MONEY, transfer_manager_fields::APPEARANCE_MONEY_COUNT, 0x890, // 0x890: their capacity
                  transfer_manager_fields::INTERNATIONAL_APPEARANCE_MONEY, transfer_manager_fields::INTERNATIONAL_APPEARANCE_MONEY_COUNT, 0x89c, // 0x89c: their capacity
                  transfer_manager_fields::FUTURE_TRANSFERS, transfer_manager_fields::FUTURE_TRANSFER_COUNT, 0x924, // 0x924: their capacity
                  transfer_manager_fields::WORK_PERMITS, transfer_manager_fields::WORK_PERMIT_COUNT, 0x942, // 0x942: their capacity
                  transfer_manager_fields::LOAN_NEGOTIATIONS] {
        write_u32(this + field, 0);
    }
    tm.set_day_cursor(0);
    let (mut start, mut result) = ([0u8; 8], [0u8; 8]);
    let d = exe::new_date(start.as_mut_ptr() as usize, 0, 0);
    exe::date_operator_3d(this + transfer_manager_fields::CURRENT_DATE, result.as_mut_ptr() as usize, d);
    tm.set_current_slice(0);
    setup_transfer_rules(this);
    let flags = exe::crt_calloc(read_u32(G_STAFF_COUNT), 8);
    write_usize(this + transfer_manager_fields::PERSON_FLAGS, flags);
    if flags == 0 {
        show_out_of_memory_box(0xa3, 0xa8d4cc); // "TRANSFER_MANAGER::constructor() : cannot malloc staff information array"
        return this;
    }
    let loans = exe::crt_calloc(read_u32(G_CLUB_COUNT), 0x10);
    write_usize(this + transfer_manager_fields::SEASON_LOAN_COUNTS, loans);
    if loans == 0 {
        show_out_of_memory_box(0xab, 0xa8d484); // "TRANSFER_MANAGER::constructor() : cannot malloc club information array"
        return this;
    }
    write_i32(this + transfer_manager_fields::LOWEST_FREE_SLOT, -1);
    if stream != 0 {
        let load: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_LOAD_FROM_DISK);
        if load(this, stream) != 0 {
            covered!("sx02b_1 loaded from the saved game");
            return this;
        }
    }
    covered!("sx02b_1 new transfer manager");
    let mut shortlist = exe::crt_operator_new(0xe0);
    if shortlist != 0 {
        let construct: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_SHORTLIST_MANAGER_CONSTRUCTOR);
        shortlist = construct(shortlist, 0);
    }
    write_usize(this + transfer_manager_fields::SHORTLIST_MANAGER, shortlist);
    if shortlist == 0 {
        show_out_of_memory_box(0xc0, 0xa8d440); // "TRANSFER_MANAGER::constructor() : cannot create shortlist manager"
        return this;
    }
    init_work_permits(this); // sx02b_1: the work permits of a new manager
    this
}

/// Comparer for `construct_transfer_manager`: only the original runs (it builds the rules objects and the shortlist
/// manager, which cannot be built twice).
pub(crate) unsafe extern "thiscall" fn compare_construct_transfer_manager(this: usize, stream: usize) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, stream) // simplification: original only
}

/// The transfer window dates of the manager's rules objects, in rules order: for each of the 28 objects (a null one
/// skipped) and each of its windows (signed byte +8; 12-byte entries at the pointer +4), the opening date (entry +0)
/// and the closing one (entry +6).
unsafe fn collect_transfer_window_dates(this: usize) -> Vec<usize> {
    let mut dates = Vec::new();
    for k in 0..0x1c {
        let rules = transfer_rules_at(TransferManagerFields(this), k);
        if rules == 0 {
            continue;
        }
        // simplification: a negative count (never in the data) would lower the original's total and add no date
        for c in 0..read_i8(rules + 8).max(0) as usize { // +8: window count
            let window = read_usize(rules + 4) + c * 0xc; // +4: the window entries
            dates.push(window);
            dates.push(window + 6); // sx02b_1: the closing date
        }
    }
    dates
}

/// Builds the 28 transfer rules objects and the sorted table of all their window dates.
///
/// - Each object (`exe::crt_operator_new` of 0x19 bytes; when it fails the slot stays 0 and nothing is built): slot 0
/// the European rules (`FN_EUROPEAN_TRANSFER_RULES_CONSTRUCTOR` with 0, −1, `KEY_CONTINENT_EUROPE`, 2, 0), then the
/// 27 national ones of `NATIONAL_TRANSFER_RULES` (id, the nation, −1, 1, 0), in that order.
/// - `transfer_window_count()` = the sum of 2 × the window count (signed byte +8) of each object; `transfer_windows()`
/// = malloc of 4 × that (none: the "Memory Allocation Error" box, line 0x3bdf, and nothing more), filled with the
/// opening and closing date of each window (`collect_transfer_window_dates`) and sorted by month, then day
/// (`exe::crt_qsort` with 0x8e5590).
/// Original: `TRANSFER_MANAGER::setup_transfer_rules` (0x8d26b0, transfer_manager.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn setup_transfer_rules(this: usize) {
    let slot = |k: usize| this + transfer_manager_fields::TRANSFER_RULES + 4 * k;
    let mut rules = exe::crt_operator_new(TRANSFER_RULES_OBJECT_SIZE);
    if rules != 0 {
        let construct: unsafe extern "thiscall" fn(usize, u32, i32, usize, u32, u32) -> usize = core::mem::transmute(FN_EUROPEAN_TRANSFER_RULES_CONSTRUCTOR);
        rules = construct(rules, 0, -1, read_usize(KEY_CONTINENT_EUROPE), 2, 0);
    }
    write_usize(slot(0), rules);
    for &(k, id, constructor, nation) in NATIONAL_TRANSFER_RULES.iter() {
        let mut rules = exe::crt_operator_new(TRANSFER_RULES_OBJECT_SIZE);
        if rules != 0 {
            let construct: unsafe extern "thiscall" fn(usize, u32, usize, i32, u32, u32) -> usize = core::mem::transmute(constructor);
            rules = construct(rules, id, read_usize(nation), -1, 1, 0); // sx02b_1: a national rules object
        }
        write_usize(slot(k), rules);
    }
    let dates = collect_transfer_window_dates(this);
    write_usize(this + transfer_manager_fields::TRANSFER_WINDOW_COUNT, dates.len());
    let table = exe::crt_malloc((dates.len() * 4) as u32);
    write_usize(this + transfer_manager_fields::TRANSFER_WINDOWS, table);
    if table == 0 {
        show_out_of_memory_box(0x3bdf, 0xa8f530); // "TRANSFER_MANAGER::setup_transfer_rules() : cannot malloc deadline dates"
        return;
    }
    core::ptr::copy_nonoverlapping(dates.as_ptr(), table as *mut usize, dates.len());
    exe::crt_qsort(table, dates.len() as u32, 4, 0x8e5590usize);
}

/// Comparer for `setup_transfer_rules`: only the original runs (it allocates the rules objects); then the window
/// table is rebuilt from its objects (`collect_transfer_window_dates`, sorted with the same qsort) and checked against
/// the original's, and each national object's nation (`transfer_rules::NATION_ID`) against its key.
pub(crate) unsafe extern "thiscall" fn compare_setup_transfer_rules(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this);
    let tm = TransferManagerFields(this);
    let mut plan = collect_transfer_window_dates(this);
    if !plan.is_empty() {
        exe::crt_qsort(plan.as_mut_ptr() as usize, plan.len() as u32, 4, 0x8e5590usize);
    }
    let n = tm.transfer_window_count();
    let same_table = n == plan.len() && tm.transfer_windows() != 0
        && core::slice::from_raw_parts(tm.transfer_windows() as *const usize, n) == plan.as_slice();
    let wrong_nation = NATIONAL_TRANSFER_RULES.iter().find(|&&(k, _, _, nation)| {
        let rules = transfer_rules_at(tm, k);
        rules != 0 && read_usize(rules + transfer_rules::NATION_ID) != read_usize(nation)
    });
    SCOREBOARD.record(same_table && wrong_nation.is_none(), || format!(
        "setup_transfer_rules: {} window dates (plan {}), table equal {}, rules slot with another nation {:?}",
        n, plan.len(), same_table, wrong_nation.map(|r| r.0)));
}
