// ---------- batch %%LEVA%%: the human manager and injury constructors (human_manager.cpp, injury.cpp; sx12b_2) ----------

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
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, stream)
}

/// Comparer for `construct_human_record`: the record (0xa6ef bytes), side by side (the constructors only write it).
pub(crate) unsafe extern "thiscall" fn compare_construct_human_record(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I1%%);
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&[(this, HUMAN_RECORD_SIZE)], || o(this), || construct_human_record(this));
    SCOREBOARD.record(equal && r == n, || format!("HUMAN_RECORD::HUMAN_RECORD({:#x}): {:?}", this, first_differences(&mo, &mr)));
    r
}

/// Comparer for `construct_injury`: it searches the CD, opens the history file list and builds (or loads) the records:
/// it cannot run twice. Only the original runs; the copy is proven by the new career (Rust mode).
pub(crate) unsafe extern "thiscall" fn compare_construct_injury(this: usize, stream: usize) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I2%%)(this, stream)
}
