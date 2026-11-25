// ---------- batch %%LEVA%%: the award manager: building it, destroying it and loading it (award_manager.cpp) ----------

/// Strings and addresses of the award manager's constructor and load.
pub(crate) mod award_manager_build {
    /// "past_year_awards.tmp" and "past_team_awards.tmp" (in "CM3_TEMP"), the history files of the year awards and of
    /// the team awards, and their record sizes.
    pub(crate) const YEAR_AWARD_TMP: usize = 0x987dac;
    pub(crate) const TEAM_AWARD_TMP: usize = 0x987d88;
    pub(crate) const TEMP_FOLDER: usize = 0x987da0;
    pub(crate) const YEAR_AWARD_RECORD_SIZE: u32 = 0x4a;
    pub(crate) const TEAM_AWARD_RECORD_SIZE: u32 = 0x205;
    /// The save item "awards.dat".
    pub(crate) const AWARDS_DAT: usize = 0x987e18;
    /// The 16 dwords cleared from +0x1c and the dword at +0xdc (not read by the ported code; [inference] per-award
    /// state of the vote).
    pub(crate) const CLEARED: usize = 0x1c;
    pub(crate) const CLEARED_DWORDS: usize = 0x10;
    pub(crate) const LAST_DWORD: usize = 0xdc;
    /// The two bytes a no-CD exe patches in this routine's CD search (as `cm0102-1080`): the drive type's `jne` and the
    /// found test.
    pub(crate) const CD_DRIVE_TYPE_JUMP: usize = 0x4131db;
    pub(crate) const CD_FOUND_TEST: usize = 0x41321d;
    /// The day word of the selected nations' year ratings (+0xc): day 364, the last of the year.
    pub(crate) const SELECTED_RATINGS_DAY: u32 = 0x16c;
}

/// `AWARD_MANAGER::create_nation_awards(file)` (0x4135b0, thiscall, ret 4).
pub(crate) const FN_AWARD_MANAGER_CREATE_NATION_AWARDS: usize = 0x4135b0;

/// `YEAR_RATINGS::YEAR_RATINGS(day, file)` (0x931360, thiscall, ret 8).
pub(crate) const FN_YEAR_RATINGS_CONSTRUCT: usize = 0x931360;

/// `database_free_staff_comp_history_table()` (0x527f20, cdecl).
pub(crate) const FN_DATABASE_FREE_STAFF_COMP_HISTORY_TABLE: usize = 0x527f20;

/// `NationAwards::~NationAwards()` (0x7682b0, thiscall): the base destructor, called directly (not the virtual one).
pub(crate) const FN_NATION_AWARDS_DESTRUCT: usize = 0x7682b0;

/// A year ratings table on the heap: `operator new(0xe)` and `YEAR_RATINGS::YEAR_RATINGS(day, file)` (loaded from the
/// open save item when there is one); a failed allocation gives 0.
pub(crate) unsafe fn award_manager_new_year_ratings(day: u32, file: usize) -> usize {
    let construct: unsafe extern "thiscall" fn(usize, u32, usize) -> usize = core::mem::transmute(FN_YEAR_RATINGS_CONSTRUCT);
    let p = exe::crt_operator_new(0xe);
    if p == 0 {
        return 0;
    }
    construct(p, day, file)
}

/// The day word of the world year ratings (+0x10): the first word of the date 20 May 2001 (`Date::Date(20, 5, 2001,
/// -1)`), the day the world ratings turn over.
pub(crate) unsafe fn award_manager_world_ratings_day() -> u32 {
    let mut date = [0u16; 4];
    read_u16(exe::date_date_549f70(&mut date, 0x14, 5, 0x7d1, u32::MAX)) as u32
}

/// A history file list in the temporary folder: `operator new(8)` and `FILE_LLIST::FILE_LLIST("CM3_TEMP\<name>",
/// record size)`; a failed allocation gives 0.
unsafe fn award_manager_new_file_list(name: usize, record_size: u32) -> usize {
    let list = exe::crt_operator_new(8);
    if list == 0 {
        return 0;
    }
    exe::new_file_llist(list, exe::utils_build_file_path(award_manager_build::TEMP_FOLDER, name), record_size)
}

/// Builds the award manager: the CD check, the history file lists of the past year awards and team awards, then the
/// award objects and the two year ratings tables, from the saved game or new.
///
/// `AWARD_MANAGER::AWARD_MANAGER(save)`: the CD search (`find_game_cd`, honouring a no-CD exe's patch at 0x4131db and
/// 0x41321d). The 16 dwords from +0x1c and the dword at +0xdc cleared. The year award history (`YEAR_AWARD_FILE`,
/// "CM3_TEMP\past_year_awards.tmp", records of 0x4a bytes) and the team award history (`TEAM_AWARD_FILE`,
/// "past_team_awards.tmp", 0x205 bytes); a failed allocation leaves 0, unchecked. The shortlists (`SHORTLISTS`) 0 and
/// their count and capacity 0. With a save, `award_manager_load` and, when it answers, that is all. Otherwise the award
/// objects (`AWARD_MANAGER::create_nation_awards(0)`), the staff competition history table freed
/// (`database_free_staff_comp_history_table`, the new game starts without it), the world year ratings
/// (`AWARD_MANAGER_WORLD_YEAR_RATINGS`, turning over on 20 May) and the selected nations' ones
/// (`AWARD_MANAGER_SELECTED_YEAR_RATINGS`, on day 364), each 0 when its allocation fails. Returns the object.
/// Original: `AWARD_MANAGER::AWARD_MANAGER` (0x413180, award_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn award_manager_construct(this: usize, save: usize) -> usize {
    use award_manager_build as b;
    use award_manager_consts as am;
    find_game_cd(b::CD_DRIVE_TYPE_JUMP, b::CD_FOUND_TEST);
    for k in 0..b::CLEARED_DWORDS {
        write_u32(this + b::CLEARED + 4 * k, 0);
    }
    write_u32(this + b::LAST_DWORD, 0);
    write_usize(this + am::YEAR_AWARD_FILE, award_manager_new_file_list(b::YEAR_AWARD_TMP, b::YEAR_AWARD_RECORD_SIZE));
    write_usize(this + am::TEAM_AWARD_FILE, award_manager_new_file_list(b::TEAM_AWARD_TMP, b::TEAM_AWARD_RECORD_SIZE));
    write_usize(this + award_manager_fields::SHORTLISTS, 0);
    write_u16(this + award_manager_fields::SHORTLIST_COUNT, 0);
    write_u16(this + award_manager_fields::SHORTLIST_CAPACITY, 0);
    if save != 0 && award_manager_load(this, save) != 0 {
        covered!("rxf_1 award manager from the save");
        return this;
    }
    covered!("rxf_1 new award manager");
    let create: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_AWARD_MANAGER_CREATE_NATION_AWARDS);
    create(this, 0); // rxf_1: the new game's award objects
    let free_history: unsafe extern "C" fn() = core::mem::transmute(FN_DATABASE_FREE_STAFF_COMP_HISTORY_TABLE);
    free_history();
    write_usize(this + AWARD_MANAGER_WORLD_YEAR_RATINGS, award_manager_new_year_ratings(award_manager_world_ratings_day(), 0));
    write_usize(this + AWARD_MANAGER_SELECTED_YEAR_RATINGS, award_manager_new_year_ratings(b::SELECTED_RATINGS_DAY, 0));
    this
}

/// Comparer for `award_manager_construct`.
///
/// It searches the CD, builds the file lists and allocates (or loads) the awards: it cannot run twice. Only the
/// original runs; the copy is proven by the new career's .qst and by loading the holiday save (levas/N/CARGA).
pub(crate) unsafe extern "thiscall" fn compare_award_manager_construct(this: usize, save: usize) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, save)
}

/// Destroys the award manager: the 30 award objects, the two history file lists, the two year ratings tables and the
/// shortlists.
///
/// `AWARD_MANAGER::~AWARD_MANAGER()`: with a table (`NATION_AWARDS`), each of its 30 objects present goes through the
/// base destructor `NationAwards::~NationAwards` (called directly, not the virtual one) and `operator delete`; the table
/// is freed (`free`). Then each present of the year award history, the team award history, the world year ratings and
/// the selected nations' ratings goes through 0x58e330 (`FILE_LLIST::~FILE_LLIST`, the same code serves YEAR_RATINGS:
/// it frees the pointer at +0) and `operator delete`; the shortlists (`SHORTLISTS`) present are freed. No pointer is
/// zeroed.
/// Original: `AWARD_MANAGER::~AWARD_MANAGER` (0x413410, award_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn award_manager_destruct(this: usize) {
    use award_manager_consts as am;
    let table = read_usize(this + am::NATION_AWARDS);
    if table != 0 {
        let destroy: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_NATION_AWARDS_DESTRUCT);
        for k in 0..am::NATION_AWARD_SLOTS {
            let awards = read_usize(read_usize(this + am::NATION_AWARDS) + 4 * k);
            if awards != 0 {
                destroy(awards);
                exe::crt_operator_delete(awards);
            }
        }
        exe::crt_free(read_usize(this + am::NATION_AWARDS));
    }
    let destroy: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FILE_LLIST_DESTRUCT);
    for field in [am::YEAR_AWARD_FILE, am::TEAM_AWARD_FILE, AWARD_MANAGER_WORLD_YEAR_RATINGS, AWARD_MANAGER_SELECTED_YEAR_RATINGS] {
        let p = read_usize(this + field);
        if p != 0 {
            destroy(p);
            exe::crt_operator_delete(p);
        }
    }
    let shortlists = read_usize(this + award_manager_fields::SHORTLISTS);
    if shortlists != 0 {
        exe::crt_free(shortlists);
    }
}

/// Comparer for `award_manager_destruct`.
///
/// It frees the awards and the lists: it cannot run twice. Only the original runs; the copy is proven by loading the
/// holiday save (free_game_data destroys the manager first) and by the careers (the game creation frees it).
pub(crate) unsafe extern "thiscall" fn compare_award_manager_destruct(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this)
}

/// Reads the award manager from the saved game: the award objects, then the world and the selected nations' year
/// ratings.
///
/// `AWARD_MANAGER::load(save)`: no save: the award_manager.cpp "Error" box (line 0x21d) and 0. The "awards.dat" item of
/// the save (`zopen` with 0, 0x16, 4; missing: the box, line 0x225, and 0). The award objects built from the item
/// (`AWARD_MANAGER::create_nation_awards(item)`), the world year ratings (`AWARD_MANAGER_WORLD_YEAR_RATINGS`, turning
/// over on 20 May) and the selected nations' ones (`AWARD_MANAGER_SELECTED_YEAR_RATINGS`, day 364) read from it (0
/// when an allocation fails); the item closed and 1.
/// Original: `AWARD_MANAGER::load` (0x413fe0, award_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn award_manager_load(this: usize, save: usize) -> u32 {
    use award_manager_build as b;
    if save == 0 {
        show_error_box(award_manager_consts::AWARD_MANAGER_CPP, 0x21d);
        return 0;
    }
    let mut z = exe::zopen(save, b::AWARDS_DAT, 0, 0x16, 4);
    if z == 0 {
        show_error_box(award_manager_consts::AWARD_MANAGER_CPP, 0x225);
        return 0;
    }
    covered!("rxf_1 load awards.dat");
    let create: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_AWARD_MANAGER_CREATE_NATION_AWARDS);
    create(this, z);
    write_usize(this + AWARD_MANAGER_WORLD_YEAR_RATINGS, award_manager_new_year_ratings(award_manager_world_ratings_day(), z));
    write_usize(this + AWARD_MANAGER_SELECTED_YEAR_RATINGS, award_manager_new_year_ratings(b::SELECTED_RATINGS_DAY, z));
    exe::zclose(&mut z);
    1 // rxf_1: the awards came from the save
}

/// Comparer for `award_manager_load`.
///
/// It builds the awards from the save item: it cannot run twice. Only the original runs; the copy is proven by loading
/// the holiday save in the save proof (levas/N/CARGA).
pub(crate) unsafe extern "thiscall" fn compare_award_manager_load(this: usize, save: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I2%%)(this, save)
}
