// ---------- sx21b_4: building the virtual staff manager, the training manager and a training schedule (virtual_staff.cpp, training_manager.cpp) ----------

/// `TRAINING_MANAGER::load_training_dat` (thiscall(path), ret 4; item sx21b_6).
const FN_TRAINING_MANAGER_LOAD_TRAINING_DAT: usize = 0x8b10d0;
/// `TRAINING_MANAGER::init_player_entries` (thiscall, Rust).
const FN_TRAINING_MANAGER_INIT_PLAYER_ENTRIES: usize = 0x8b3c50;
/// `TRAINING_MANAGER::set_default_schedules` (thiscall, Rust).
const FN_TRAINING_MANAGER_SET_DEFAULT_SCHEDULES: usize = 0x8b3d40;
/// `TRAINING_SCHEDULE::~TRAINING_SCHEDULE` (thiscall), the vector destructor's element destructor.
const FN_TRAINING_SCHEDULE_DESTRUCTOR: usize = 0x5a53c0;
/// The schedule file header's initializer (thiscall, at schedule +0x18): magic word 0x1011, byte 0xff, two zero
/// dwords and 0x21.
const FN_TRAINING_FILE_HEADER_INIT: usize = 0x8bba10;

/// The fewest players of each of the 8 position groups (`get_player_position_group` numbering) a side needs, as the
/// virtual staff constructor writes them at the start of the object: a club or main national squad (`MIN_BY_GROUP`),
/// a B national squad (+8) and a club outside the selected leagues (`MIN_BY_GROUP_LOWER_CLUB`).
const VIRTUAL_STAFF_GROUP_MINIMUMS: [u8; 0x18] = [
    4, 4, 4, 8, 4, 8, 4, 8, // sx21b_4: four of each line and eight centre players
    6, 4, 4, 0xc, 4, 8, 4, 8,
    2, 2, 2, 4, 2, 4, 2, 4,
];

/// The CD search at the start of a subsystem constructor: each CD drive (`GetLogicalDriveStringsA`, `GetDriveTypeA`
/// = 5) is looked at for "<root>cm0102.exe" (`file_find_first`) until one has it, with the "Insert CD" box (OK) in
/// between. A no-CD exe patched in the routine (as `cm0102-1080`: the drive type's `jne` at `drive_type_jump` made
/// `nop`, the found test at `found_test` made `xor`) is honoured by reading those bytes.
pub(crate) unsafe fn wait_for_game_cd(drive_type_jump: usize, found_test: usize) {
    let drives: unsafe extern "system" fn(u32, *mut u8) -> u32 = core::mem::transmute(read_usize(0x9670f8));
    let kind: unsafe extern "system" fn(usize) -> u32 = core::mem::transmute(read_usize(0x967100));
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let any_drive = read_u8(drive_type_jump) == 0x90;
    let always_found = read_u8(found_test) != 0x85;
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
        let (mut title, mut message) = ([0u8; 0x100], [0u8; 0x100]);
        text(title.as_mut_ptr() as usize, 0x987df4); // "Insert CD"
        text(message.as_mut_ptr() as usize, 0x987dc4); // "Please insert the CM01/02 CD and click on OK"
        exe::gui_show_message_box(title.as_ptr() as usize, message.as_ptr() as usize, 1);
    }
}

/// Builds the virtual staff manager: the empty fixture list, the CD check, the position group minimums and no viewed
/// clubs.
///
/// `VIRTUAL_STAFF::VIRTUAL_STAFF()`: `FIXTURE_LIST` built (`exe::new_llist`); `wait_for_game_cd`; the 24 bytes of
/// `VIRTUAL_STAFF_GROUP_MINIMUMS` at `MIN_BY_GROUP`; the 17 `VIEWED_CLUBS` slots cleared. Returns `this`.
/// Original: `VIRTUAL_STAFF::VIRTUAL_STAFF` (0x90d140, virtual_staff.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn construct_virtual_staff(this: usize) -> usize {
    exe::new_llist(this + virtual_fixture_fns::FIXTURE_LIST);
    wait_for_game_cd(0x90d1a6, 0x90d1e9);
    covered!("sx21b_4 virtual staff built");
    core::ptr::copy_nonoverlapping(VIRTUAL_STAFF_GROUP_MINIMUMS.as_ptr(), (this + virtual_squad_fns::MIN_BY_GROUP) as *mut u8, 0x18);
    core::ptr::write_bytes((this + virtual_fixture_fns::VIEWED_CLUBS) as *mut u8, 0, 0x44);
    this
}

/// Comparer for `construct_virtual_staff`: the original builds the game's object; the copy builds a scratch one (its
/// empty list destroyed after), and the 0x6a bytes are compared.
pub(crate) unsafe extern "thiscall" fn compare_construct_virtual_staff(this: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I0%%)(this);
    let mut scratch = [0u8; 0x6a];
    let s = scratch.as_mut_ptr() as usize;
    construct_virtual_staff(s);
    let (a, b) = (core::slice::from_raw_parts(this as *const u8, 0x6a), &scratch[..]);
    let equal = a == b;
    SCOREBOARD.record(equal, || format!("construct_virtual_staff: original {:02x?} rust {:02x?}", a, b));
    exe::drop_llist(s + virtual_fixture_fns::FIXTURE_LIST);
    r
}

/// Builds the training manager: its 167 schedules, the CD check, and either the saved training.dat or a new game's
/// training (player entries, empty coach assignments, default schedules).
///
/// `TRAINING_MANAGER::TRAINING_MANAGER(path)`: the 0xa7 schedules of 0x9b bytes built (`exe::crt_vector_constructor_iterator`
/// with `TRAINING_SCHEDULE::TRAINING_SCHEDULE` and its destructor); `wait_for_game_cd`; `POSITION_TRAINING_ITEMS`,
/// `POSITION_TRAINING_COUNT` and the dword at +0x6529 cleared. With a `path` whose training.dat loads
/// (`load_training_dat`), that is all; otherwise `init_player_entries`, the 0x60 nine-byte coach slots of
/// `COACH_ASSIGNMENTS` emptied (coach id -1, the dword at +4 and the byte at +8 cleared), `set_default_schedules` and
/// `WEEK_COUNTER` 0. Returns `this`.
/// Original: `TRAINING_MANAGER::TRAINING_MANAGER` (0x8b0ec0, training_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn construct_training_manager(this: usize, path: usize) -> usize {
    use training_manager_fields as tm;
    exe::crt_vector_constructor_iterator(this, 0x9b, 0xa7, FN_TRAINING_SCHEDULE_TRAINING_SCHEDULE, FN_TRAINING_SCHEDULE_DESTRUCTOR);
    wait_for_game_cd(0x8b0f34, 0x8b0f76);
    write_u32(this + tm::POSITION_TRAINING_ITEMS, 0);
    write_u32(this + tm::POSITION_TRAINING_COUNT, 0);
    write_u32(this + 0x6529, 0); // unnamed dword after the position training count
    if path != 0 {
        let load: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_TRAINING_MANAGER_LOAD_TRAINING_DAT);
        if load(this, path) != 0 {
            return this;
        }
    }
    covered!("sx21b_4 new game training");
    let init: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_TRAINING_MANAGER_INIT_PLAYER_ENTRIES);
    init(this);
    training_manager_clear_coach_slots(this + tm::COACH_ASSIGNMENTS);
    let defaults: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_TRAINING_MANAGER_SET_DEFAULT_SCHEDULES);
    defaults(this);
    write_u16(this + tm::WEEK_COUNTER, 0);
    this
}

/// The 0x60 nine-byte coach slots of `COACH_ASSIGNMENTS` (at `slots`) emptied: coach id -1, the dword at +4 and the byte at +8 0.
unsafe fn training_manager_clear_coach_slots(slots: usize) {
    for k in 0..0x60 {
        let slot = slots + k * 9;
        write_i32(slot, -1); // sx21b_4: no coach
        write_u32(slot + 4, 0);
        write_u8(slot + 8, 0);
    }
}

/// Comparer for `construct_training_manager`: it allocates (the player entries) and loads files, so only the original
/// runs; in a new game (no `path`) the plan is checked: the coach slots, the week counter and the cleared dwords.
pub(crate) unsafe extern "thiscall" fn compare_construct_training_manager(this: usize, path: usize) -> usize {
    use training_manager_fields as tm;
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I1%%)(this, path);
    if path == 0 {
        let size = 0x60 * 9;
        let mut plan = vec![0u8; size];
        training_manager_clear_coach_slots(plan.as_mut_ptr() as usize);
        let game = core::slice::from_raw_parts((this + tm::COACH_ASSIGNMENTS) as *const u8, size);
        let rest = read_u32(this + tm::POSITION_TRAINING_ITEMS) == 0 && read_u32(this + tm::POSITION_TRAINING_COUNT) == 0 && read_u32(this + 0x6529) == 0 && read_u16(this + tm::WEEK_COUNTER) == 0;
        SCOREBOARD.record(game == &plan[..] && rest, || format!("construct_training_manager: coach slots original {:02x?} plan {:02x?}, cleared fields {}", &game[..18], &plan[..18], rest));
    }
    r
}

/// Builds a training schedule record: the file header (magic 0x200cb, version 1) and cleared flags.
///
/// `TRAINING_SCHEDULE::TRAINING_SCHEDULE()`: the file header's start at +0x18 initialized (0x8bba10); +0x2e and the
/// word at +0x16 cleared; the dword at +0x97 0; the header magic at +0x29 0x200cb, the version byte at +0x2d 1 and the
/// dword at +0x93 1. Returns `this`.
/// Original: `TRAINING_SCHEDULE::TRAINING_SCHEDULE` (0x8b4e10, training_manager.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn construct_training_schedule(this: usize) -> usize {
    let header: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_TRAINING_FILE_HEADER_INIT);
    header(this + 0x18);
    write_u8(this + 0x2e, 0);
    write_u16(this + 0x16, 0);
    write_u32(this + 0x97, 0);
    write_u32(this + 0x29, 0x200cb); // sx21b_4: the schedule file magic
    write_u8(this + 0x2d, 1);
    write_u32(this + 0x93, 1);
    this
}

/// Comparer for `construct_training_schedule`: the 0x9b-byte record side by side.
pub(crate) unsafe extern "thiscall" fn compare_construct_training_schedule(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I2%%);
    let (r, c, equal, mo, mr) = crate::comparison::side_by_side(&[(this, 0x9busize)], || o(this), || construct_training_schedule(this));
    SCOREBOARD.record(equal && r == c, || format!("construct_training_schedule({:#x}): differences in {:?}", this, first_differences(&mo, &mr)));
    r
}
