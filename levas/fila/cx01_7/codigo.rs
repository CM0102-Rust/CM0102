// ---------- cx01_7: the CONMEBOL seeding constructor and a cup's temporary file (conmebol_seeding.cpp, Cup.cpp) ----------

/// `ConmebolSeeding::load_from_temp_file` (0x4c6970, conmebol_seeding.cpp, thiscall, ret 0): 1 when the seeding came
/// back from its temporary file.
const FN_CONMEBOL_SEEDING_LOAD_FROM_TEMP_FILE: usize = 0x4c6970;
/// `Comp_Stats::save` (0x4ac4e0, comp_stats.cpp, thiscall(file), ret 4).
const FN_COMP_STATS_SAVE: usize = 0x4ac4e0;
/// `Cup_Stage::save_to_file` (0x523eb0, cup_stage.cpp, thiscall(file), ret 4).
const FN_CUP_STAGE_SAVE_TO_FILE: usize = 0x523eb0;
/// `League_Stage::save_to_file` (0x68ca40, league_stage.cpp, thiscall(file), ret 4).
const FN_LEAGUE_STAGE_SAVE_TO_FILE: usize = 0x68ca40;

/// Builds the CONMEBOL seeding: the game CD check (`game_cd_check`, the drive type jump 0x4c601a and the found test
/// 0x4c605d); then, when the seeding comes back from its temporary file (`ConmebolSeeding::load_from_temp_file`), that
/// is all. Otherwise the year of the last update and the row count are cleared and the nations are walked in table
/// order (`G_NATION_TABLE`, 0x122 bytes each) while there are fewer than 11 rows: Mexico and every nation of the South
/// American continent get a 10-byte row (+0 the nation id, +4 the nation, +8 its Libertadores places: 4 for Brazil and
/// Argentina, 1 for Venezuela and Mexico, 3 for the others). The Copa dos Campeões winner (`BRA_CHAMP_CUP_WINNER_ID`)
/// becomes −1. Returns the object.
/// Original: `ConmebolSeeding::ConmebolSeeding` (0x4c5fe0, conmebol_seeding.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn new_conmebol_seeding(this: usize) -> usize {
    game_cd_check(0x4c601a, 0x4c605d);
    let load: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_CONMEBOL_SEEDING_LOAD_FROM_TEMP_FILE);
    if load(this) != 0 {
        covered!("cx01_7 conmebol seeding from the file");
        return this;
    }
    covered!("cx01_7 conmebol seeding built");
    write_u16(this + conmebol_seeding::LAST_SEASON_YEAR, 0);
    write_u8(this + conmebol_seeding::NATION_COUNT, 0);
    let mut k = 0i16;
    while (k as i32) < read_i32(G_NATION_COUNT) {
        let rows = read_u8(this + conmebol_seeding::NATION_COUNT) as i8;
        if rows >= 0xb {
            break;
        }
        let nation = Nation(read_usize(G_NATION_TABLE) + k as usize * 0x122);
        let id = read_u32(nation.addr());
        let continent = nation.continent();
        if id == read_u32(KEY_NATION_MEXICO) || (continent != 0 && read_u32(continent) == read_u32(KEY_CONTINENT_SOUTH_AMERICA)) {
            let row = this + rows as isize as usize * 10;
            write_u32(row, id); // the row's nation id
            write_usize(row + 4, nation.addr()); // the row's nation
            let places = if id == read_u32(KEY_NATION_BRAZIL) || id == read_u32(KEY_NATION_ARGENTINA) {
                4
            } else if id == read_u32(KEY_NATION_VENEZUELA) || id == read_u32(KEY_NATION_MEXICO) {
                1
            } else {
                3
            };
            write_u8(row + 8, places); // the row's Libertadores places
            write_u8(this + conmebol_seeding::NATION_COUNT, rows as u8 + 1);
        }
        k += 1;
    }
    write_u32(this + conmebol_seeding::BRA_CHAMP_CUP_WINNER_ID, u32::MAX);
    this
}

/// Writes `count` items of `size` bytes from `data` into the open file `f` (`exe::crt_fwrite`).
unsafe fn temp_write(f: usize, data: usize, size: u32, count: u32) {
    exe::crt_fwrite(data, size, count, f);
}

/// Turns the clubs of the first `count` 6-byte entries of `book` into their ids (−1 for none), in place.
unsafe fn club_entries_to_ids(book: usize, count: i16) {
    for i in 0..count.max(0) as usize {
        let e = book + 6 * i;
        let club = read_usize(e);
        write_u32(e, if club != 0 { read_u32(club) } else { u32::MAX });
    }
}

/// Turns the ids of the first `count` 6-byte entries of `book` back into club pointers (null for −1), in place.
unsafe fn club_entries_from_ids(book: usize, count: i16) {
    for i in 0..count.max(0) as usize {
        let e = book + 6 * i;
        let id = read_i32(e);
        write_usize(e, if id != -1 { club_at(id as isize).addr() } else { 0 });
    }
}

/// The name of a cup's temporary file, written at +0x53 (80 bytes): the competition's long name with '/' and '.' made
/// '_', then "%s_%d.tmp" with the number.
unsafe fn cup_temp_file_name(this: League, number: i32) {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let mut name = [0u8; 0x100];
    format(name.as_mut_ptr() as usize, 0x991944, this.competition().name_addr()); // "%s"
    for c in name.iter_mut().take_while(|c| **c != 0) {
        if *c == b'/' || *c == b'.' {
            *c = b'_';
        }
    }
    format(this.addr() + 0x53, 0x99b838, name.as_ptr() as usize, number); // "%s_%d.tmp"
}

/// The header of a cup's temporary file, after the name and the statistics: the class's own data (virtual +0x24) and
/// the fields in the order `Cup::load_from_disk` reads them back.
unsafe fn cup_save_header(this: League, f: usize) {
    let class_data: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(read_usize(read_usize(this.addr()) + 0x24));
    class_data(this.addr(), f);
    let a = this.addr();
    for (field, size) in [(cup::PRIZE_FUND, 4), (league::SQUAD_FLAGS, 1), (cup::IMPORTANCE_BIAS, 1), (league::ENTRANTS_ARE_QUALIFIERS, 1),
        (cup::SEASON_CLOSED, 4), (cup::VENUE_STADIUM_ID, 4), (cup::SUBSTITUTES_NAMED, 1), (cup::SUBSTITUTIONS_ALLOWED, 1), (cup::SEASON_SET_UP, 4),
        (cup::RUN_BY_STAGES, 1), (cup::ROUND_COUNT, 2), (cup::TEAM_COUNT, 2), (cup::YEAR, 2), (cup::CURRENT_ROUND, 1), (cup::NEXT_DRAW_ROUND, 1),
        (cup::CLUB_SIDES, 1), (cup::STAGE_LABEL, 2), (league::UPPER_DIVISION_ID, 4), (league::LOWER_DIVISION_ID, 4)] {
        temp_write(f, a + field, size, 1);
    }
    // the points-average table: its pointer +0x18 as it is, its row count +0x34 and its 0x13-byte rows
    temp_write(f, a + 0x18, 4, 1);
    temp_write(f, a + 0x34, 2, 1);
    let averages = read_usize(a + 0x18);
    if averages != 0 {
        temp_write(f, averages, 0x13, read_i16(a + 0x34) as i32 as u32);
    }
}

/// The stages of a cup in its temporary file: for each stage up to the current one, its type and, for a cup stage, its
/// team and round counts, each round's tie count and `Cup_Stage::save_to_file`; for a league stage, its club and round
/// counts and `League_Stage::save_to_file`.
unsafe fn cup_save_stages(this: League, f: usize) {
    let mut k = 0i32;
    while k <= this.current_stage() as i32 {
        let stage = League(read_usize(this.stages() + k as usize * 4));
        temp_write(f, stage.addr() + league::COMP_TYPE, 1, 1);
        match stage.comp_type() {
            1 => {
                temp_write(f, stage.addr() + cup::TEAM_COUNT, 2, 1);
                temp_write(f, stage.addr() + cup::ROUND_COUNT, 2, 1);
                let s = Cup(stage.addr());
                for r in 0..s.round_count().max(0) as usize {
                    temp_write(f, s.round_table() + r * cup_round::SIZE + cup_round::TIES, 2, 1);
                }
                let save: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_CUP_STAGE_SAVE_TO_FILE);
                save(stage.addr(), f);
            }
            2 => {
                temp_write(f, stage.addr() + league::CLUB_COUNT, 2, 1);
                temp_write(f, stage.addr() + league::ROUND_COUNT, 2, 1);
                let save: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_LEAGUE_STAGE_SAVE_TO_FILE);
                save(stage.addr(), f);
            }
            _ => {}
        }
        k += 1;
    }
}

/// Writes a cup's state into its CM3_TEMP temporary file, in the order `Cup::load_from_disk` reads it.
///
/// `Cup::save_cup_to_disk(by_id)`. The file name at +0x53 (`cup_temp_file_name`) with the competition record's id;
/// without `by_id` (its low byte), a cup whose season is not set up (`SEASON_SET_UP`) writes nothing, otherwise the
/// name takes the cup's `YEAR`. Without a round table, team table or tie tables, the Cup.cpp error box (line 0x147e).
/// The file opened in CM3_TEMP ("wb", `open_temp_file_for_write`); unopened, the error box (line 0x1488; the message
/// "unable to open save file" is printed into a local the box never shows). Then: the 0x50-byte name, the statistics
/// flag (4 bytes) and the statistics (`Comp_Stats::save`), the header (`cup_save_header`), the competition id (the
/// record's first dword; without a record the error box of line 0x14c3 and −1), the round table (0x68 bytes per
/// round), the team table (6 bytes per team, its clubs written as ids and turned back into pointers), each round's
/// ties (0x35 bytes each, `cup_round::TIES`), `STAGE_COUNT`, `CURRENT_STAGE`, `STAGE_INDEX`, the stages
/// (`cup_save_stages`), `ENTRANT_CAPACITY`, `ENTRANT_COUNT` and the entrant book (its first count clubs as ids, the
/// whole capacity written, then the clubs back). The file closed.
/// Defect of the original: with entrants but no book (+0x14 null), the id pass reads through null.
/// Original: `Cup::save_cup_to_disk` (0x5210f0, Cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn cup_save_cup_to_disk(this: usize, by_id: u32) {
    let this = League(this);
    let cup = Cup(this.addr());
    let has_stats = (this.comp_stats() != 0) as u32;
    let no_competition = u32::MAX;
    cup_temp_file_name(this, read_i32(this.competition().addr()));
    if by_id as u8 == 0 {
        if cup.season_set_up() == 0 {
            return;
        }
        cup_temp_file_name(this, this.year() as i32);
    }
    if cup.round_table() == 0 || cup.team_table() == 0 || cup.tie_tables() == 0 {
        show_error_at(CUP_SOURCE, 0x147e);
        return;
    }
    let name = this.addr() + 0x53;
    let f = open_temp_file_for_write(name);
    if f == 0 {
        let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
        let mut message = [0u8; 0x100];
        format(message.as_mut_ptr() as usize, 0x9b58dc, name); // "CUP::save_cup_to_disk() - unable to open save file'%s'"
        show_error_at(CUP_SOURCE, 0x1488);
        return;
    }
    covered!("cx01_7 cup saved");
    temp_write(f, name, 0x50, 1);
    temp_write(f, &has_stats as *const u32 as usize, 4, 1);
    if has_stats != 0 {
        let save: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_COMP_STATS_SAVE);
        save(this.comp_stats(), f);
    }
    cup_save_header(this, f);
    if !this.competition().is_null() {
        temp_write(f, this.competition().addr(), 4, 1);
    } else {
        show_error_at(CUP_SOURCE, 0x14c3);
        temp_write(f, &no_competition as *const u32 as usize, 4, 1);
    }
    temp_write(f, cup.round_table(), cup_round::SIZE as u32, cup.round_count() as i32 as u32);
    club_entries_to_ids(cup.team_table(), cup.team_count());
    temp_write(f, cup.team_table(), 6, cup.team_count() as i32 as u32);
    club_entries_from_ids(cup.team_table(), cup.team_count());
    for r in 0..cup.round_count().max(0) as usize {
        let ties = read_i16(cup.round_table() + r * cup_round::SIZE + cup_round::TIES) as i32;
        temp_write(f, read_usize(cup.tie_tables() + r * 4), (ties * 0x35) as u32, 1);
    }
    temp_write(f, this.addr() + cup::STAGE_COUNT, 4, 1);
    temp_write(f, this.addr() + cup::CURRENT_STAGE, 4, 1);
    temp_write(f, this.addr() + cup::STAGE_INDEX, 1, 1);
    cup_save_stages(this, f);
    temp_write(f, this.addr() + cup::ENTRANT_CAPACITY, 2, 1);
    temp_write(f, this.addr() + cup::ENTRANT_COUNT, 2, 1);
    let book = this.entrant_book();
    club_entries_to_ids(book, this.entrant_count());
    if book != 0 {
        temp_write(f, book, 6, this.entrant_capacity() as i32 as u32);
    }
    club_entries_from_ids(book, this.entrant_count());
    exe::crt_fclose(f);
}

/// Comparer for `new_conmebol_seeding`: it waits for the CD and may read the temporary file (cannot run twice); only
/// the original runs.
pub(crate) unsafe extern "thiscall" fn compare_new_conmebol_seeding(this: usize) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I0%%)(this)
}

/// The bytes of a cup's temporary file (the name at +0x53, in CM3_TEMP); empty when it cannot be read.
unsafe fn cup_temp_file_bytes(this: usize) -> Vec<u8> {
    let path = c_string_bytes(exe::utils_build_file_path(0x987da0, this + 0x53)); // "CM3_TEMP"
    std::fs::read(String::from_utf8_lossy(&path).into_owned()).unwrap_or_default()
}

/// Comparer for `cup_save_cup_to_disk`: the original writes the file, which is read back; the copy writes it again and
/// the two contents (and the name at +0x53) must match. The file is left as the copy wrote it, the same bytes.
pub(crate) unsafe extern "thiscall" fn compare_cup_save_cup_to_disk(this: usize, by_id: u32) {
    let name = |p: usize| { let s = core::slice::from_raw_parts(p as *const u8, 0x50); s[..s.iter().position(|&c| c == 0).unwrap_or(0x50)].to_vec() };
    original_fn::<unsafe extern "thiscall" fn(usize, u32)>(%%I1%%)(this, by_id);
    let name_o = name(this + 0x53);
    let o = cup_temp_file_bytes(this);
    cup_save_cup_to_disk(this, by_id);
    let name_r = name(this + 0x53);
    let r = cup_temp_file_bytes(this);
    SCOREBOARD.record(name_o == name_r && o == r, || format!("cup_save_cup_to_disk({:#x}, {}): file {:?} {} bytes, copy {:?} {} bytes, first difference at {:?}", this, by_id as u8,
        String::from_utf8_lossy(&name_o), o.len(), String::from_utf8_lossy(&name_r), r.len(), o.iter().zip(r.iter()).position(|(a, b)| a != b)));
}
