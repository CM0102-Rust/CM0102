// ---------- rxdb_1: the whole database written into the saved game (Database.cpp) ----------

/// `g_index_dat_entries`: the 22 index.dat entries as the database loaded them (67 bytes each).
const DATABASE_SAVE_INDEX_ENTRIES: usize = 0xae1a80;
/// Number of index.dat entries (one per table file).
const DATABASE_SAVE_INDEX_SLOTS: usize = 22;
/// Size of an index.dat entry: the file name (51 bytes), the table id, the count, the offset and the version.
const DATABASE_SAVE_ENTRY_SIZE: usize = 0x43;
/// index.dat entry: the table id (4 bytes).
const DATABASE_SAVE_ENTRY_TABLE_ID: usize = 0x33;
/// index.dat entry: where the table starts in its file (4 bytes).
const DATABASE_SAVE_ENTRY_OFFSET: usize = 0x3b;
/// The bytes after the entries in index.dat (the original writes 460 bytes of its own stack there, never set).
const DATABASE_SAVE_INDEX_TAIL: usize = 0x1cc;
/// `g_reserve_comp_row_count`: the reserve competition rows at the end of the club competition table.
const DATABASE_SAVE_RESERVE_COMP_COUNT: usize = 0xae2344;
/// `g_reserve_manager_row_count`, written into the index.dat header.
const DATABASE_SAVE_RESERVE_MANAGER_COUNT: usize = 0xae2348;
/// `g_nat_club_count`: the national teams at the end of the club table.
const DATABASE_SAVE_NAT_CLUB_COUNT: usize = 0xae2360;
/// `g_preferences_count`.
const DATABASE_SAVE_PREFERENCES_COUNT: usize = 0xae2370;
/// `g_nation_comp_count`.
const DATABASE_SAVE_NATION_COMP_COUNT: usize = 0xae2380;
/// `g_staff_comp_count`.
const DATABASE_SAVE_STAFF_COMP_COUNT: usize = 0xae2388;
/// `g_colour_count`.
const DATABASE_SAVE_COLOUR_COUNT: usize = 0xae2394;
/// `g_first_name_count`.
const DATABASE_SAVE_FIRST_NAME_COUNT: usize = 0xae2398;
/// `g_second_name_count`.
const DATABASE_SAVE_SECOND_NAME_COUNT: usize = 0xae239c;
/// `g_nation_comp_table` (inside the club competition table).
const DATABASE_SAVE_NATION_COMP_TABLE: usize = 0xae23d8;
/// "index.dat".
const DATABASE_SAVE_INDEX_DAT: usize = 0x9b5c48;
/// "Saving database" (the progress text, through the language library).
const DATABASE_SAVE_PROGRESS_TEXT: usize = 0x9b604c;
/// The progress steps: 0 at the start, one per table (1..22) and index.dat (23).
const DATABASE_SAVE_STEPS: u32 = 0x18;
/// "CM3_TEMP" and "": the temporary folder where the language files are.
const DATABASE_SAVE_TEMP_KEY: usize = 0x987da0;
const DATABASE_SAVE_EMPTY_TEXT: usize = 0xad9c64;
/// "*.lng": the language files packed into the saved game.
const DATABASE_SAVE_LNG_PATTERN: usize = 0x9b5b38;
/// The size of a club competition record.
const DATABASE_SAVE_COMP_SIZE: usize = 0x6b;
/// `zipdir_ztell` (0x933170, cdecl(z)): the position in the container item (`position_in_container`).
const FN_DATABASE_SAVE_ZTELL: usize = 0x933170;

/// A pointer field of a database record turned back into the target's id: (offset, consecutive slots, where the id
/// is in the target record).
type DatabaseSaveIdField = (usize, usize, usize);

const DATABASE_SAVE_COMP_FIELDS: [DatabaseSaveIdField; 4] = [
    (competition::NATION, 1, 0), (competition::CONTINENT, 1, 0), (competition::FOREGROUND_COLOUR, 1, 0),
    (competition::BACKGROUND_COLOUR, 1, 0),
];

const DATABASE_SAVE_CLUB_FIELDS: [DatabaseSaveIdField; 26] = [
    (club::NATION, 1, 0), (club::DIVISION, 1, 0), (club::LAST_DIVISION, 1, 0), (club::RESERVE_DIVISION, 1, 0),
    (club::STADIUM, 1, 0), (0x6e, 1, 0), // +0x6e: a second stadium (a stadium pointer for resolve_database_indexes)
    (club::FORE_COLOUR1, 1, 0), (club::FORE_COLOUR2, 1, 0), (club::FORE_COLOUR3, 1, 0),
    (club::BACK_COLOUR1, 1, 0), (club::BACK_COLOUR2, 1, 0), (club::BACK_COLOUR3, 1, 0),
    (club::FAV_STAFF1, 3, 0), (club::DIS_STAFF1, 3, 0), (club::RIVAL1, 1, 0), (club::RIVAL2, 1, 0), (club::RIVAL3, 1, 0),
    (club::SQUAD, 50, 0), (club::SCOUTS, 7, 0), (club::COACHES, 5, 0), (club::PHYSIOS, 3, 0), (club::DIRECTORS, 3, 0),
    (club::MANAGER, 1, 0), (club::CHAIRMAN, 1, 0), (club::ASSISTANT_MANAGER, 1, 0), (club::TEAM_SELECTED, 20, 0),
];

const DATABASE_SAVE_NATION_FIELDS: [DatabaseSaveIdField; 10] = [
    (nation::CONTINENT, 1, 0), (nation::CAPITAL_CITY, 1, 0), (nation::NATIONAL_STADIUM, 1, 0),
    (nation::FOREGROUND_COLOUR1, 1, 0), (nation::FOREGROUND_COLOUR1 + 8, 1, 0), (nation::FOREGROUND_COLOUR1 + 0x10, 1, 0),
    (nation::BACKGROUND_COLOUR1, 1, 0), (nation::BACKGROUND_COLOUR1 + 8, 1, 0), (nation::BACKGROUND_COLOUR1 + 0x10, 1, 0),
    (0x110, 3, 0), // +0x110..+0x118: three nation pointers (resolve_database_indexes)
];

const DATABASE_SAVE_OFFICIAL_FIELDS: [DatabaseSaveIdField; 4] = [
    (official::FIRST_NAME, 1, names::ID), (official::SECOND_NAME, 1, names::ID), (official::NATION, 1, 0), (official::CITY, 1, 0),
];

const DATABASE_SAVE_STADIUM_FIELDS: [DatabaseSaveIdField; 2] = [(stadium::CITY, 1, 0), (stadium::NEARBY_STADIUM, 1, 0)];

/// The six liked and disliked clubs and the six liked and disliked people.
const DATABASE_SAVE_PREFERENCES_FIELDS: [DatabaseSaveIdField; 1] = [(preferences::STAFF_FAVOURITE_CLUBS1, 12, 0)];

/// The eight people of the non-player sheet.
const DATABASE_SAVE_NON_PLAYER_FIELDS: [DatabaseSaveIdField; 1] = [(non_player::GOALKEEPER, 8, 0)];

const DATABASE_SAVE_STAFF_FIELDS: [DatabaseSaveIdField; 10] = [
    (staff::FIRST_NAME, 1, names::ID), (staff::SECOND_NAME, 1, names::ID), (staff::COMMON_NAME, 1, names::ID),
    (staff::NATION, 1, 0), (staff::SECOND_NATION, 1, 0), (staff::NATIONAL_JOB, 1, 0), (staff::CLUB_JOB, 1, 0),
    (staff::PLAYER, 1, 0), (staff::STAFF_PREFERENCES, 1, 0), (staff::NON_PLAYER, 1, 0),
];

/// +0x53 continent, +0x57 nation, +0x5b and +0x5f colours (as resolve_database_indexes reads them).
const DATABASE_SAVE_STAFF_COMP_FIELDS: [DatabaseSaveIdField; 1] = [(0x53, 4, 0)];

const DATABASE_SAVE_STAFF_HISTORY_FIELDS: [DatabaseSaveIdField; 2] = [(staff_history::STAFF_ID, 1, 0), (staff_history::CLUB, 1, 0)];

const DATABASE_SAVE_CITY_FIELDS: [DatabaseSaveIdField; 1] = [(city::NATION, 1, 0)];

const DATABASE_SAVE_COMP_HISTORY_FIELDS: [DatabaseSaveIdField; 5] = [
    (4, 1, 0), // +4: the competition
    (comp_history_row::CHAMPION, 1, 0), (comp_history_row::RUNNER_UP, 1, 0), (comp_history_row::THIRD_PLACE, 1, 0),
    (comp_history_row::HOST, 1, 0),
];

/// The tables whose pointers go back to ids, in the original's order: (table, count, record size, fields).
const DATABASE_SAVE_LINKED_TABLES: [(usize, usize, usize, &[DatabaseSaveIdField]); 12] = [
    (G_CLUB_COMP_TABLE, G_CLUB_COMP_COUNT, DATABASE_SAVE_COMP_SIZE, &DATABASE_SAVE_COMP_FIELDS),
    (G_CLUB_TABLE, G_CLUB_COUNT, club::SIZE, &DATABASE_SAVE_CLUB_FIELDS),
    (G_NATION_TABLE, G_NATION_COUNT, nation::SIZE, &DATABASE_SAVE_NATION_FIELDS),
    (G_OFFICIAL_TABLE, G_OFFICIAL_COUNT, 0x2b, &DATABASE_SAVE_OFFICIAL_FIELDS),
    (G_STADIUM_TABLE, G_STADIUM_COUNT, stadium::SIZE, &DATABASE_SAVE_STADIUM_FIELDS),
    (G_PREFERENCES_TABLE, DATABASE_SAVE_PREFERENCES_COUNT, 0x34, &DATABASE_SAVE_PREFERENCES_FIELDS),
    (G_NON_PLAYER_TABLE, G_NON_PLAYER_COUNT, 0x44, &DATABASE_SAVE_NON_PLAYER_FIELDS),
    (G_STAFF_TABLE, G_STAFF_COUNT, staff::SIZE, &DATABASE_SAVE_STAFF_FIELDS),
    (G_STAFF_COMP_TABLE, DATABASE_SAVE_STAFF_COMP_COUNT, 0x65, &DATABASE_SAVE_STAFF_COMP_FIELDS),
    (G_STAFF_HISTORY_TABLE, G_STAFF_HISTORY_COUNT, 0x11, &DATABASE_SAVE_STAFF_HISTORY_FIELDS),
    (G_CITY_TABLE, G_CITY_COUNT, 0x38, &DATABASE_SAVE_CITY_FIELDS),
    (G_CLUB_COMP_HISTORY_TABLE, G_CLUB_COMP_HISTORY_COUNT, 0x1a, &DATABASE_SAVE_COMP_HISTORY_FIELDS),
];

/// Turns every pointer field of the database back into the id of the record it points to (the id at +0, or at
/// `names::ID` for the name records; a null pointer becomes −1): the inverse of `resolve_database_indexes`.
unsafe fn database_save_pointers_to_ids() {
    covered!("rxdb_1 database pointers to ids");
    for &(table, count, size, fields) in DATABASE_SAVE_LINKED_TABLES.iter() {
        for i in 0..read_i32(count).max(0) as usize {
            let record = read_usize(table) + i * size;
            for &(offset, slots, id_at) in fields {
                for k in 0..slots {
                    let field = record + offset + 4 * k;
                    let target = read_usize(field);
                    write_i32(field, if target == 0 { -1 } else { read_i32(target + id_at) });
                }
            }
        }
    }
}

/// How a table goes out besides the plain write.
#[derive(Clone, Copy, PartialEq)]
enum DatabaseSaveKind {
    Plain,
    /// The staff: its entry is the one the three tables below compare their file name with.
    Staff,
    /// A table that the database keeps inside the staff file: when its entry names the staff's file, it gets its own
    /// file (the name), the table id (when given) and the position where it starts.
    OwnFile(usize, Option<i32>),
    /// The nation competitions, followed in the same file by the reserve competition rows.
    NationComps,
}

/// The writer of a table: `db_files_write_*(stream, use_zip, records, count)`, 1 when everything went out.
type DatabaseSaveWriter = unsafe extern "C" fn(usize, i32, usize, u32) -> i32;

/// One table of the save: the ids its index.dat entry may have, the global holding it, the box text and line when the
/// entry is missing, the entry's version, the record count, the writer and the kind.
struct DatabaseSaveTable {
    ids: &'static [i32],
    table: usize,
    no_entry_text: usize,
    line: u32,
    version: i32,
    count: unsafe fn() -> i32,
    writer: DatabaseSaveWriter,
    kind: DatabaseSaveKind,
}

unsafe fn database_save_count_continents() -> i32 { read_i32(G_CONTINENT_COUNT) }
unsafe fn database_save_count_officials() -> i32 { read_i32(G_OFFICIAL_COUNT) }
unsafe fn database_save_count_first_names() -> i32 { read_i32(DATABASE_SAVE_FIRST_NAME_COUNT) }
unsafe fn database_save_count_second_names() -> i32 { read_i32(DATABASE_SAVE_SECOND_NAME_COUNT) }
unsafe fn database_save_count_common_names() -> i32 { read_i32(G_COMMON_NAME_COUNT) }
unsafe fn database_save_count_cities() -> i32 { read_i32(G_CITY_COUNT) }
/// The clubs without the national teams at the end of the table.
unsafe fn database_save_count_clubs() -> i32 { read_i32(G_CLUB_COUNT).wrapping_sub(read_i32(DATABASE_SAVE_NAT_CLUB_COUNT)) }
unsafe fn database_save_count_national_teams() -> i32 { read_i32(DATABASE_SAVE_NAT_CLUB_COUNT) }
unsafe fn database_save_count_staff_comps() -> i32 { read_i32(DATABASE_SAVE_STAFF_COMP_COUNT) }
/// The club competitions without the reserve rows and the nation competitions.
unsafe fn database_save_count_club_comps() -> i32 {
    read_i32(G_CLUB_COMP_COUNT).wrapping_sub(read_i32(DATABASE_SAVE_RESERVE_COMP_COUNT)).wrapping_sub(read_i32(DATABASE_SAVE_NATION_COMP_COUNT))
}
unsafe fn database_save_count_nation_comps() -> i32 { read_i32(DATABASE_SAVE_NATION_COMP_COUNT) }
unsafe fn database_save_count_staff_comp_history() -> i32 { read_i32(G_STAFF_COMP_HISTORY_COUNT) }
/// The club competition history without the nation competitions' rows at the end.
unsafe fn database_save_count_club_comp_history() -> i32 {
    read_i32(G_CLUB_COMP_HISTORY_COUNT).wrapping_sub(read_i32(G_NATION_COMP_HISTORY_COUNT))
}
unsafe fn database_save_count_nation_comp_history() -> i32 { read_i32(G_NATION_COMP_HISTORY_COUNT) }
unsafe fn database_save_count_colours() -> i32 { read_i32(DATABASE_SAVE_COLOUR_COUNT) }
unsafe fn database_save_count_nations() -> i32 { read_i32(G_NATION_COUNT) }
unsafe fn database_save_count_stadiums() -> i32 { read_i32(G_STADIUM_COUNT) }
unsafe fn database_save_count_staff() -> i32 { read_i32(G_STAFF_COUNT) }
unsafe fn database_save_count_preferences() -> i32 { read_i32(DATABASE_SAVE_PREFERENCES_COUNT) }
unsafe fn database_save_count_non_players() -> i32 { read_i32(G_NON_PLAYER_COUNT) }
unsafe fn database_save_count_players() -> i32 { read_i32(G_PLAYER_DATA_COUNT) }
unsafe fn database_save_count_staff_history() -> i32 { read_i32(G_STAFF_HISTORY_COUNT) }

/// The 22 tables in the original's order (progress steps 1..22). The stadiums' box reuses the continents' text, as in
/// the original.
const DATABASE_SAVE_TABLES: [DatabaseSaveTable; 22] = {
    use DatabaseSaveKind::*;
    const fn t(ids: &'static [i32], table: usize, no_entry_text: usize, line: u32, version: i32, count: unsafe fn() -> i32,
               writer: DatabaseSaveWriter, kind: DatabaseSaveKind) -> DatabaseSaveTable {
        DatabaseSaveTable { ids, table, no_entry_text, line, version, count, writer, kind }
    }
    [
        t(&[3], G_CONTINENT_TABLE, 0x9b6020, 0x103b, 1, database_save_count_continents, db_files_write_continents, Plain),
        t(&[7], G_OFFICIAL_TABLE, 0x9b5ff4, 0x106c, 1, database_save_count_officials, db_files_write_officials, Plain),
        t(&[0xd], G_FIRST_NAME_TABLE, 0x9b5fc8, 0x109e, 1, database_save_count_first_names, db_files_write_or_read_records_0x3c, Plain),
        t(&[0xe], G_SECOND_NAME_TABLE, 0x9b5f98, 0x10d0, 1, database_save_count_second_names, db_files_write_or_read_records_0x3c, Plain),
        t(&[0xf], G_COMMON_NAME_TABLE, 0x9b5f68, 0x1102, 1, database_save_count_common_names, db_files_write_or_read_records_0x3c, Plain),
        t(&[0x15], G_CITY_TABLE, 0x9b5f40, 0x1134, 2, database_save_count_cities, db_files_write_cities, Plain),
        t(&[0], G_CLUB_TABLE, 0x9b5f18, 0x1166, 2, database_save_count_clubs, db_files_write_clubs, Plain),
        t(&[1], G_NAT_CLUB_TABLE, 0x9b5ee8, 0x1198, 2, database_save_count_national_teams, db_files_write_clubs, Plain),
        t(&[0xb], G_STAFF_COMP_TABLE, 0x9b5ebc, 0x11ca, 2, database_save_count_staff_comps, db_files_write_staff_comps, Plain),
        t(&[0xc], G_CLUB_COMP_TABLE, 0x9b5e90, 0x11fc, 2, database_save_count_club_comps, db_files_write_comps, Plain),
        t(&[0x10], DATABASE_SAVE_NATION_COMP_TABLE, 0x9b5e60, 0x122e, 2, database_save_count_nation_comps, db_files_write_comps, NationComps),
        t(&[0x12], G_STAFF_COMP_HISTORY_TABLE, 0x9b5e2c, 0x1268, 1, database_save_count_staff_comp_history, db_files_write_records_0x3a, Plain),
        t(&[0x13], G_CLUB_COMP_HISTORY_TABLE, 0x9b5df8, 0x129a, 1, database_save_count_club_comp_history, db_files_write_comp_history, Plain),
        t(&[0x14], G_NATION_COMP_HISTORY_TABLE, 0x9b5dc4, 0x12cc, 1, database_save_count_nation_comp_history, db_files_write_comp_history, Plain),
        t(&[2], G_COLOUR_TABLE, 0x9b5d9c, 0x12fe, 1, database_save_count_colours, db_files_write_records_0x3a, Plain),
        t(&[4], G_NATION_TABLE, 0x9b5d74, 0x1330, 2, database_save_count_nations, db_files_write_nations, Plain),
        t(&[5], G_STADIUM_TABLE, 0x9b6020, 0x1362, 1, database_save_count_stadiums, db_files_write_stadiums, Plain),
        t(&[6], G_STAFF_TABLE, 0x9b5d4c, 0x1394, 2, database_save_count_staff, db_files_write_staff, Staff),
        t(&[8, 0x16], G_PREFERENCES_TABLE, 0x9b5d18, 0x13c6, 1, database_save_count_preferences, db_files_write_staff_extra,
          OwnFile(0x9b5d08, Some(0x16))), // "Preferences.dat"
        t(&[9], G_NON_PLAYER_TABLE, 0x9b5cdc, 0x13ff, 2, database_save_count_non_players, db_files_write_players,
          OwnFile(0x9b5ccc, None)), // "nonplayer.dat"
        t(&[10], G_PLAYER_DATA_TABLE, 0x9b5ca4, 0x1437, 2, database_save_count_players, db_files_write_non_players,
          OwnFile(0x9b5c98, None)), // "player.dat"
        t(&[0x11], G_STAFF_HISTORY_TABLE, 0x9b5c68, 0x1470, 1, database_save_count_staff_history, db_files_write_staff_history, Plain),
    ]
};

/// Where the save goes and the index.dat entries built on the way.
struct DatabaseSaveOutput {
    folder: usize,
    use_zip: i32,
    zip_a: u32,
    zip_b: u32,
    entries: [u8; DATABASE_SAVE_INDEX_SLOTS * DATABASE_SAVE_ENTRY_SIZE],
    used: usize,
    staff_slot: Option<usize>,
}

impl DatabaseSaveOutput {
    fn entry(&mut self, slot: usize) -> usize {
        self.entries.as_mut_ptr() as usize + slot * DATABASE_SAVE_ENTRY_SIZE
    }

    /// Opens (creates) a file of the save: an item of the container or a file in the folder; 0 when it failed.
    unsafe fn open(&self, name: usize) -> usize {
        if self.use_zip != 0 {
            exe::zopen(self.folder, name, 1, self.zip_a, self.zip_b)
        } else {
            db_files_open_for_write(self.folder, name)
        }
    }

    unsafe fn write(&self, data: usize, size: u32, n: u32, f: usize) -> u32 {
        if self.use_zip != 0 {
            exe::zwrite(data, size, n, f)
        } else {
            exe::crt_fwrite(data, size, n, f)
        }
    }

    unsafe fn close(&self, f: usize) {
        if self.use_zip != 0 {
            let mut z = f;
            exe::zclose(&mut z);
        } else {
            exe::crt_fclose(f);
        }
    }
}

/// The slot of the first index.dat entry of the database with one of `ids`.
unsafe fn database_save_find_entry(ids: &[i32]) -> Option<usize> {
    (0..DATABASE_SAVE_INDEX_SLOTS)
        .find(|&k| ids.contains(&read_i32(DATABASE_SAVE_INDEX_ENTRIES + k * DATABASE_SAVE_ENTRY_SIZE + DATABASE_SAVE_ENTRY_TABLE_ID)))
}

/// True when the C strings at `a` and `b` are equal.
unsafe fn database_save_same_name(a: usize, b: usize) -> bool {
    let n = c_string_len(a);
    n == c_string_len(b) && (0..n).all(|k| read_u8(a + k) == read_u8(b + k))
}

/// Opens the file of a table (its own one when it shares the staff's), fills its entry (count, start, version) and
/// writes the records. Err when the file did not open or the writer failed (the file closed).
unsafe fn database_save_table_file(t: &DatabaseSaveTable, out: &mut DatabaseSaveOutput, slot: usize) -> Result<(), ()> {
    let entry = out.entry(slot);
    if let DatabaseSaveKind::OwnFile(name, _) = t.kind {
        if let Some(staff) = out.staff_slot {
            let staff_entry = out.entry(staff);
            if database_save_same_name(entry, staff_entry) {
                copy_c_string(entry, name);
            }
        }
    }
    let f = out.open(entry);
    if f == 0 {
        return Err(());
    }
    let start = match t.kind {
        DatabaseSaveKind::OwnFile(_, id) => {
            if let Some(id) = id {
                write_i32(entry + DATABASE_SAVE_ENTRY_TABLE_ID, id);
            }
            if out.use_zip != 0 {
                let ztell: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(FN_DATABASE_SAVE_ZTELL);
                ztell(f)
            } else {
                exe::crt_ftell(f)
            }
        }
        _ => 0,
    };
    let n = (t.count)();
    write_i32(entry + DATABASE_SAVE_ENTRY_OFFSET, start);
    write_i32(entry + db_index_entry::VERSION, t.version);
    write_i32(entry + db_index_entry::RECORD_COUNT, n);
    let mut written = (t.writer)(f, out.use_zip, read_usize(t.table), n as u32) != 0;
    let reserve = read_i32(DATABASE_SAVE_RESERVE_COMP_COUNT);
    if written && t.kind == DatabaseSaveKind::NationComps && reserve >= 1 {
        // the reserve competition rows, at the end of the club competition table
        let first = read_i32(G_CLUB_COMP_COUNT).wrapping_sub(reserve) as usize;
        written = db_files_write_comps(f, out.use_zip, read_usize(G_CLUB_COMP_TABLE).wrapping_add(first.wrapping_mul(DATABASE_SAVE_COMP_SIZE)), reserve as u32) != 0;
    }
    out.close(f);
    if written { Ok(()) } else { Err(()) }
}

/// One table: its index.dat entry (when the database has one) goes to the next slot; a loaded table without an entry
/// shows the box; a loaded table with one is written to its file.
unsafe fn database_save_table(t: &DatabaseSaveTable, out: &mut DatabaseSaveOutput) -> Result<(), ()> {
    let found = database_save_find_entry(t.ids);
    let slot = out.used;
    if let Some(k) = found {
        core::ptr::copy((DATABASE_SAVE_INDEX_ENTRIES + k * DATABASE_SAVE_ENTRY_SIZE) as *const u8, out.entry(slot) as *mut u8, DATABASE_SAVE_ENTRY_SIZE);
        if t.kind == DatabaseSaveKind::Staff {
            out.staff_slot = Some(slot);
        }
    }
    if read_usize(t.table) != 0 {
        match found {
            None => {
                let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
                format(0xde1f64, t.no_entry_text, DATABASE_SAVE_INDEX_DAT);
                database_error_box(t.line);
            }
            Some(_) => database_save_table_file(t, out, slot)?,
        }
    }
    if found.is_some() {
        out.used += 1;
    }
    Ok(())
}

/// Writes index.dat: the reserve counts, the 22 entries, a 1 byte, 460 empty bytes, the clubs' English name index
/// (its length and the ids, or −1 without it) and the database language. Err when it did not open or the entries did
/// not go out.
unsafe fn database_save_index(out: &mut DatabaseSaveOutput) -> Result<(), ()> {
    covered!("rxdb_1 database index.dat");
    let f = out.open(DATABASE_SAVE_INDEX_DAT);
    if f == 0 {
        return Err(());
    }
    out.write(DATABASE_SAVE_RESERVE_COMP_COUNT, 4, 1, f);
    out.write(DATABASE_SAVE_RESERVE_MANAGER_COUNT, 4, 1, f);
    let entries = out.entry(0);
    if out.write(entries, DATABASE_SAVE_ENTRY_SIZE as u32, DATABASE_SAVE_INDEX_SLOTS as u32, f) != DATABASE_SAVE_INDEX_SLOTS as u32 {
        out.close(f); // the original closes the file twice here (see the routine's header)
        return Err(());
    }
    let one = 1u8;
    out.write(&one as *const u8 as usize, 1, 1, f);
    let tail = [0u8; DATABASE_SAVE_INDEX_TAIL];
    out.write(tail.as_ptr() as usize, DATABASE_SAVE_INDEX_TAIL as u32, 1, f);
    let english = read_usize(G_CLUB_ENGLISH_NAME_INDEX_TABLE);
    if english == 0 {
        let none = -1i32;
        out.write(&none as *const i32 as usize, 4, 1, f);
    } else {
        out.write(G_CLUB_COUNT, 4, 1, f);
        out.write(english, (read_i32(G_CLUB_COUNT) as u32).wrapping_mul(4), 1, f);
    }
    out.write(G_DATABASE_NAMES_LANGUAGE, 4, 1, f);
    out.close(f);
    Ok(())
}

/// Packs the language files of the temporary folder (`CM3_TEMP`) into the saved game.
unsafe fn database_save_language_files(out: &DatabaseSaveOutput) {
    let path = exe::utils_build_file_path(DATABASE_SAVE_TEMP_KEY, DATABASE_SAVE_EMPTY_TEXT);
    let mut folder = [0u8; 0x104];
    let n = c_string_len(path).min(folder.len() - 1);
    core::ptr::copy(path as *const u8, folder.as_mut_ptr(), n);
    exe::zadd_matching_files(out.folder as *const u8, folder.as_ptr(), DATABASE_SAVE_LNG_PATTERN, out.zip_a, out.zip_b);
}

/// Writes the whole database: into the saved game's container (`use_zip` ≠ 0; `zip_a`, `zip_b` go to `zopen`) or as
/// .dat files into the folder `folder`. Returns 1, or 0 when a file did not open or a write failed.
///
/// With the database in its linked form (`G_DATABASE_SHARED_ALLOC_MODE` ≠ 0), every pointer field of the club
/// competitions, clubs, nations, officials, stadiums, preferences, non-player sheets, people, staff competitions,
/// people history, cities and competition history first becomes the id of the record it points to (−1 when null; the
/// names give the id at `names::ID`). Then, with a progress step each, the 22 tables in a fixed order: the entry of
/// the table in the loaded index.dat (`g_index_dat_entries`, looked up by table id) is copied to the next slot of the
/// new index; a loaded table without an entry shows the "<table> table valid, but no entry in index.dat" box and is
/// not written; a loaded table with one is written to the file its entry names, with the entry's count, start (0)
/// and version (1 or 2) set. The clubs exclude the national teams (written as their own table), the club
/// competitions exclude the nation competitions and the reserve rows, which follow the nation competitions in their
/// file; the club competition history excludes the nations' rows. Preferences, non-player sheets and player sheets,
/// kept inside the staff file in the database, get their own file ("Preferences.dat", "nonplayer.dat", "player.dat")
/// when their entry names the staff's file, with the start at the position in it, and the preferences take table id
/// 22. Then index.dat (the reserve competition and manager counts, the 22 entries, a 1 byte, 460 bytes, the clubs'
/// English name index or −1, the database language), the pointers back (`resolve_database_indexes(1)`) and, into the
/// container, the `*.lng` files of the temporary folder. A failure stops the save, resolving the pointers back with
/// `resolve_database_indexes(0)`.
///
/// Defects of the original: the 460 bytes after the index.dat entries are its own stack, never set (the copy writes
/// zeros), and so are the entries of tables the loaded index lacks; the stadiums' box says "Continents"; when the
/// staff entry is missing, the preferences, non-players and players compare their file name with a slot picked by
/// an unset local (the copy then keeps the name); a failed write of the index.dat entries closes the file twice (the
/// copy closes it once); the temporary folder is copied into a 260-byte local without a bound (the copy cuts it).
/// Original: `save_database` (0x52dad0, Database.cpp, cdecl).
pub(crate) unsafe extern "C" fn save_database(folder: usize, use_zip: i32, zip_a: u32, zip_b: u32) -> u32 {
    covered!("rxdb_1 save database");
    let langlib: unsafe extern "C" fn(usize, usize, ...) -> usize = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let mut text = [0u8; 0x68];
    let text = text.as_mut_ptr() as usize;
    langlib(text, DATABASE_SAVE_PROGRESS_TEXT);
    let progress = |step: i32| exe::screenmanager_send_network_status_message(G_SCREEN_MANAGER, text, step, DATABASE_SAVE_STEPS, -1);
    progress(0);
    if read_u32(G_DATABASE_SHARED_ALLOC_MODE) != 0 {
        database_save_pointers_to_ids();
    }
    let mut out = DatabaseSaveOutput {
        folder, use_zip, zip_a, zip_b,
        entries: [0; DATABASE_SAVE_INDEX_SLOTS * DATABASE_SAVE_ENTRY_SIZE],
        used: 0,
        staff_slot: None,
    };
    let mut saved = Ok(());
    for (k, t) in DATABASE_SAVE_TABLES.iter().enumerate() {
        progress(k as i32 + 1);
        saved = database_save_table(t, &mut out);
        if saved.is_err() {
            break;
        }
    }
    if saved.is_ok() {
        progress(DATABASE_SAVE_TABLES.len() as i32 + 1);
        saved = database_save_index(&mut out);
    }
    if saved.is_err() {
        if read_u32(G_DATABASE_SHARED_ALLOC_MODE) != 0 {
            resolve_database_indexes(0);
        }
        return 0;
    }
    if read_u32(G_DATABASE_SHARED_ALLOC_MODE) != 0 {
        resolve_database_indexes(1);
    }
    if use_zip != 0 {
        database_save_language_files(&out);
    }
    1
}

/// Comparer: writing the database twice would put every table twice into the container.
pub(crate) unsafe extern "C" fn compare_save_database(folder: usize, use_zip: i32, zip_a: u32, zip_b: u32) -> u32 {
    original_fn::<unsafe extern "C" fn(usize, i32, u32, u32) -> u32>(%%I0%%)(folder, use_zip, zip_a, zip_b) // simplification: original only
}
