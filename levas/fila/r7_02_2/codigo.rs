// ---------- r7_02_2: freeing the database and loading it from the Data folder (Database.cpp) ----------

/// What `database_free_all_tables` lets go of: a malloc block, a GlobalAlloc block, the English names object
/// (`eng_lng_free_tables` and `delete`) and the *.lng files of the temporary folder.
#[derive(Clone, Copy)]
pub(crate) enum DatabaseRelease {
    Free(usize),
    GlobalFree(usize),
    EnglishNames(usize),
    LanguageFiles,
}

/// The import of GlobalFree (replaced by the DLL's allocator, see allocator.rs).
const FREE_DB_IAT_GLOBAL_FREE: usize = 0x9670f4;
/// "*.lng".
const FREE_DB_LNG_PATTERN: usize = 0x9b5b38;

/// The order and the rules of `database_free_all_tables`, with what is let go of handed to `release` (the comparer
/// runs it without letting go of anything). A table freed by the mode (GlobalFree in the linked mode, free otherwise)
/// or always by free; each freed table is zeroed with its count. In the linked mode the tables inside the shared
/// blocks are only zeroed with their counts.
pub(crate) unsafe fn database_free_all_tables_with(release: &mut dyn FnMut(DatabaseRelease)) {
    let linked = || read_u32(G_DATABASE_SHARED_ALLOC_MODE) != 0;
    let by_mode = |release: &mut dyn FnMut(DatabaseRelease), table: usize, counts: &[usize]| {
        let p = read_usize(table);
        if p != 0 {
            release(if linked() { DatabaseRelease::GlobalFree(p) } else { DatabaseRelease::Free(p) });
            write_usize(table, 0);
            counts.iter().for_each(|&c| write_i32(c, 0));
        }
    };
    by_mode(&mut *release, G_CONTINENT_TABLE, &[G_CONTINENT_COUNT]);
    by_mode(&mut *release, G_STAFF_HISTORY_TABLE, &[G_STAFF_HISTORY_COUNT]);
    by_mode(&mut *release, G_STAFF_COMP_HISTORY_TABLE, &[G_STAFF_COMP_HISTORY_COUNT]);
    let by_free = |release: &mut dyn FnMut(DatabaseRelease), table: usize, zeroed: &[usize]| {
        let p = read_usize(table);
        if p != 0 {
            release(DatabaseRelease::Free(p));
            write_usize(table, 0);
            zeroed.iter().for_each(|&c| write_i32(c, 0));
        }
    };
    by_free(&mut *release, G_CLUB_COMP_HISTORY_TABLE, &[G_CLUB_COMP_HISTORY_COUNT]);
    by_free(&mut *release, G_PREFERENCES_TABLE, &[0xae2370]); // the preferences count
    by_free(&mut *release, 0xae2404, &[0xae2400]); // the version 1 people and their count
    by_free(&mut *release, G_STAFF_TABLE, &[G_STAFF_COUNT]);
    by_free(&mut *release, G_PLAYER_DATA_TABLE, &[G_PLAYER_DATA_COUNT]);
    by_free(&mut *release, G_NON_PLAYER_TABLE, &[G_NON_PLAYER_COUNT]);
    by_free(&mut *release, G_OFFICIAL_TABLE, &[G_OFFICIAL_COUNT]);
    by_free(&mut *release, G_CLUB_TABLE, &[G_CLUB_COUNT, G_NAT_CLUB_TABLE, 0xae2360]); // with the national teams and their count
    if linked() {
        covered!("r7_02_2 linked tables zeroed");
        for t in [G_FIRST_NAME_TABLE, G_SECOND_NAME_TABLE, G_COMMON_NAME_TABLE, G_CITY_TABLE, G_STAFF_COMP_TABLE,
            G_CLUB_COMP_TABLE, 0xae23d8, G_NATION_COMP_HISTORY_TABLE, G_COLOUR_TABLE, G_NATION_TABLE, G_STADIUM_TABLE,
            G_NON_PLAYER_TABLE, G_PLAYER_DATA_TABLE] {
            write_usize(t, 0);
        }
        for c in [0xae2398, 0xae239c, G_COMMON_NAME_COUNT, G_CITY_COUNT, 0xae2388, G_CLUB_COMP_COUNT, 0xae2380,
            G_NATION_COMP_HISTORY_COUNT, 0xae2394, G_NATION_COUNT, G_STADIUM_COUNT, G_NON_PLAYER_COUNT, G_PLAYER_DATA_COUNT] {
            write_i32(c, 0);
        }
    } else {
        covered!("r7_02_2 separate tables freed");
        by_free(&mut *release, G_FIRST_NAME_TABLE, &[0xae2398]); // the first names: always free
        by_mode(&mut *release, G_SECOND_NAME_TABLE, &[0xae239c]);
        by_mode(&mut *release, G_COMMON_NAME_TABLE, &[G_COMMON_NAME_COUNT]);
        by_mode(&mut *release, G_CITY_TABLE, &[G_CITY_COUNT]);
        by_mode(&mut *release, G_STAFF_COMP_TABLE, &[0xae2388]);
        by_mode(&mut *release, G_CLUB_COMP_TABLE, &[G_CLUB_COMP_COUNT]);
        by_mode(&mut *release, 0xae23d8, &[0xae2380]); // the nation competitions
        by_mode(&mut *release, G_NATION_COMP_HISTORY_TABLE, &[G_NATION_COMP_HISTORY_COUNT]);
        by_mode(&mut *release, G_COLOUR_TABLE, &[0xae2394]);
        by_mode(&mut *release, G_NATION_TABLE, &[G_NATION_COUNT]);
        by_mode(&mut *release, G_STADIUM_TABLE, &[G_STADIUM_COUNT]);
        by_mode(&mut *release, G_NON_PLAYER_TABLE, &[G_NON_PLAYER_COUNT]);
        by_mode(&mut *release, G_PLAYER_DATA_TABLE, &[G_PLAYER_DATA_COUNT]);
    }
    let index = read_usize(G_CLUB_ENGLISH_NAME_INDEX_TABLE);
    if index != 0 {
        release(DatabaseRelease::Free(index));
        write_usize(G_CLUB_ENGLISH_NAME_INDEX_TABLE, 0);
    }
    let names = read_usize(G_ENGLISH_NAME_TABLES);
    if names != 0 {
        release(DatabaseRelease::EnglishNames(names));
        write_usize(G_ENGLISH_NAME_TABLES, 0);
    }
    write_i32(G_DATABASE_NAMES_LANGUAGE, -1);
    release(DatabaseRelease::LanguageFiles);
}

/// Frees every database table (malloc or GlobalAlloc by the layout, see `database_free_all_tables_with`) and zeroes
/// the counts, then the clubs' English name index, the English names object, the names language (−1) and the *.lng
/// files copied into CM3_TEMP. Called by `load_database` before loading and by `load_game_data` and `free_game_data`.
/// Original: `database_free_all_tables` (0x528080, Database.cpp, cdecl).
pub(crate) unsafe extern "C" fn database_free_all_tables() {
    covered!("r7_02_2 free all tables");
    database_free_all_tables_with(&mut |r| match r {
        DatabaseRelease::Free(p) => exe::crt_free(p),
        DatabaseRelease::GlobalFree(p) => {
            let f: unsafe extern "system" fn(usize) -> usize = core::mem::transmute(read_usize(FREE_DB_IAT_GLOBAL_FREE));
            f(p);
        }
        DatabaseRelease::EnglishNames(t) => {
            eng_lng_free_tables(t);
            exe::crt_operator_delete(t);
        }
        DatabaseRelease::LanguageFiles => exe::utils_clear_temp_folder(FREE_DB_LNG_PATTERN),
    });
}

/// Loads the database from the Data folder: `load_database`(folder, shared, 0).
/// Original: `load_database_from_files` (0x524d50, Database.cpp, cdecl).
pub(crate) unsafe extern "C" fn load_database_from_files(folder: usize, shared: u32) -> u32 {
    covered!("r7_02_2 load database from files");
    exe::load_database(folder, shared, 0)
}

/// Comparer: the original frees; the copy runs on the globals as they were without letting go of anything (each block
/// is freed once); the table and count globals, the names language must match. The original's state stays.
pub(crate) unsafe extern "C" fn compare_database_free_all_tables() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I0%%);
    let regions = [(G_DATABASE_SHARED_ALLOC_MODE, 0xcc), (G_DATABASE_NAMES_LANGUAGE, 4)];
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(), || database_free_all_tables_with(&mut |_| {}));
    SCOREBOARD.record(equal, || format!("database_free_all_tables: {:x?}", first_memory_difference(&mo, &mr)));
}

/// Comparer: a pass-through; the load has its own comparer.
pub(crate) unsafe extern "C" fn compare_load_database_from_files(folder: usize, shared: u32) -> u32 {
    original_fn::<unsafe extern "C" fn(usize, u32) -> u32>(%%I1%%)(folder, shared)
}
