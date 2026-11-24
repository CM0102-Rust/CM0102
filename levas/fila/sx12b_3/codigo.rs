// ---------- batch %%LEVA%%: the hall of fame and key nation constructors (hall_of_fame.cpp, key_nation.cpp; sx12b_3) ----------

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
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I0%%);
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&[(this, bytes)], || o(this), || construct_hall_of_fame(this));
    SCOREBOARD.record(equal && r == n, || format!("HallOfFame::HallOfFame: {:?}", first_differences(&mo, &mr)));
    r
}

/// Comparer for `construct_hall_of_fame_entry`: the entry, side by side (it runs 2800 times at the start and at each
/// hall of fame construction).
pub(crate) unsafe extern "thiscall" fn compare_construct_hall_of_fame_entry(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I1%%);
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side_light(&[(this, hall_of_fame_consts::ENTRY_SIZE)], || o(this),
        || construct_hall_of_fame_entry(this));
    SCOREBOARD.record(equal && r == n, || format!("HallOfFameEntry::HallOfFameEntry({:#x}): {:?}", this, first_differences(&mo, &mr)));
    r
}

/// Comparer for `construct_key_nation`: the entry, side by side.
pub(crate) unsafe extern "thiscall" fn compare_construct_key_nation(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I2%%);
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side_light(&[(this, 72)], || o(this), || construct_key_nation(this));
    SCOREBOARD.record(equal && r == n, || format!("KEY_NATION::KEY_NATION({:#x}): {:?}", this, first_differences(&mo, &mr)));
    r
}
