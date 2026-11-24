// ---------- cx02_17: a cup stage in the cup's temporary file (cup_stage.cpp) ----------

/// The Cup_Stage vtable.
const CUP_STAGE_VTABLE: usize = 0x969468;
/// The size of a cup tie (the records of `cup::TIE_TABLES`).
const TIE_SIZE: usize = 0x35;

/// Builds an empty cup stage for loading: the Cup base (`exe::new_cup`), the Cup_Stage vtable, the team table of
/// `teams` 6-byte entries (`cup::TEAM_TABLE`), the round table of `rounds` 0x68-byte rounds (`cup::ROUND_TABLE`), the
/// tie vector of `rounds` pointers (`cup::TIE_TABLES`) and, for each round, its `tie_counts[r]` 0x35-byte ties
/// (`exe::crt_malloc`, the counts taken as 16 bits). The team and round counts are not written here (the load reads
/// them). An allocation that fails shows the cup_stage.cpp box (lines 0x7e, 0x85, 0x8d, 0x97 for a round's ties) and
/// stops there. Returns the object.
/// Original: `Cup_Stage::Cup_Stage` (0x5230e0, cup_stage.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn new_cup_stage_for_load(this: usize, teams: u32, rounds: u32, tie_counts: *const i16) -> usize {
    let cup = Cup(this);
    exe::new_cup(this);
    write_usize(this, CUP_STAGE_VTABLE);
    let (teams, rounds) = (teams as i16 as i32, rounds as i16 as i32);
    cup.set_team_table(exe::crt_malloc((teams * 6) as u32));
    if cup.team_table() == 0 {
        show_error_at(CUP_STAGE_SOURCE, 0x7e);
        return this;
    }
    cup.set_round_table(exe::crt_malloc((rounds * cup_round::SIZE as i32) as u32));
    if cup.round_table() == 0 {
        show_error_at(CUP_STAGE_SOURCE, 0x85);
        return this;
    }
    cup.set_tie_tables(exe::crt_malloc((rounds * 4) as u32));
    if cup.tie_tables() == 0 {
        show_error_at(CUP_STAGE_SOURCE, 0x8d);
        return this;
    }
    covered!("cx02_17 cup stage built for loading");
    for r in 0..rounds.max(0) as usize {
        let ties = exe::crt_malloc((*tie_counts.add(r) as i32 * TIE_SIZE as i32) as u32);
        write_usize(cup.tie_tables() + 4 * r, ties);
        if ties == 0 {
            show_error_at(CUP_STAGE_SOURCE, 0x97);
            break;
        }
    }
    this
}

/// One read of a cup stage's load: `count` items of `size` bytes at `at` (`exe::crt_fread`); fewer than `expected`
/// shows the cup_stage.cpp box at `line` and the game stops (`fatal_error_at`).
unsafe fn cup_stage_read(at: usize, size: u32, count: u32, file: usize, expected: u32, line: u32) {
    if exe::crt_fread(at, size, count, file) != expected {
        fatal_error_at(CUP_STAGE_SOURCE, line);
    }
}

/// The fields of a cup stage in its file, in order: (field, size, the box line of a short read).
const CUP_STAGE_FILE_FIELDS: [(usize, u32, u32); 15] = [(cup::PRIZE_FUND, 4, 0xe9), (cup::IMPORTANCE_BIAS, 1, 0xef), (cup::SUBSTITUTES_NAMED, 1, 0xf5),
    (cup::SUBSTITUTIONS_ALLOWED, 1, 0xfb), (cup::RUN_BY_STAGES, 1, 0x101), (cup::ROUND_COUNT, 2, 0x107), (cup::TEAM_COUNT, 2, 0x10d), (cup::YEAR, 2, 0x113),
    (cup::CURRENT_ROUND, 1, 0x11a), (cup::NEXT_DRAW_ROUND, 1, 0x121), (cup::STAGE_INDEX, 1, 0x128), (cup::CLUB_SIDES, 1, 0x12f), (cup::STAGE_LABEL, 2, 0x136),
    (league::UPPER_DIVISION_ID, 4, 0x13d), (league::LOWER_DIVISION_ID, 4, 0x144)];

/// Reads a cup stage back from the cup's temporary file, in the order `cup_stage_save_to_file` wrote it: the fields
/// (`CUP_STAGE_FILE_FIELDS`, one by one), the competition id (the record of that id in [`G_CLUB_COMP_TABLE`] into
/// `league::COMPETITION`, −1 none), the round table (`ROUND_COUNT` rounds of 0x68 bytes), the team table (`TEAM_COUNT`
/// 6-byte entries, their club ids turned into clubs: `club_entries_from_ids`) and each round's ties (the round's
/// `cup_round::TIES` × 0x35 bytes, one read). Any short read shows the box of its line and the game stops. Returns 1.
/// Defect of the original: the team table's read is checked against the round table's count read (the check of line
/// 0x163 repeats the round table's), so a short team table goes unnoticed.
/// Original: `Cup_Stage::load_from_file` (0x523380, cup_stage.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn cup_stage_load_from_file(this: usize, file: usize) -> u32 {
    let cup = Cup(this);
    for (field, size, line) in CUP_STAGE_FILE_FIELDS {
        cup_stage_read(this + field, size, 1, file, 1, line);
    }
    let mut competition = 0i32;
    cup_stage_read(&mut competition as *mut i32 as usize, 4, 1, file, 1, 0x14b);
    write_usize(this + league::COMPETITION, if competition == -1 { 0 } else { read_usize(G_CLUB_COMP_TABLE).wrapping_add((competition as usize).wrapping_mul(0x6b)) });
    let rounds = cup.round_count() as i32 as u32;
    cup_stage_read(cup.round_table(), cup_round::SIZE as u32, rounds, file, rounds, 0x15b);
    exe::crt_fread(cup.team_table(), 6, cup.team_count() as i32 as u32, file); // checked against the round count: passes
    club_entries_from_ids(cup.team_table(), cup.team_count());
    covered!("cx02_17 cup stage loaded");
    let mut r = 0i32;
    while r < cup.round_count() as i32 {
        let ties = read_i16(cup.round_table() + r as usize * cup_round::SIZE + cup_round::TIES) as i32;
        cup_stage_read(read_usize(cup.tie_tables() + 4 * r as usize), (ties * TIE_SIZE as i32) as u32, 1, file, 1, 0x17a);
        r += 1;
    }
    1
}

/// Writes a cup stage into the cup's temporary file (`exe::crt_fwrite`, unchecked), in the order
/// `cup_stage_load_from_file` reads it: without a round table, team table or tie vector, the cup_stage.cpp box (line
/// 0x19a) and nothing. The fields (`CUP_STAGE_FILE_FIELDS`), the competition record's id (its first dword; without a
/// record the box of line 0x1b9 and −1), the round table, the team table with its clubs written as ids and turned back
/// into clubs, and each round's ties.
/// Original: `Cup_Stage::save_to_file` (0x523eb0, cup_stage.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn cup_stage_save_to_file(this: usize, file: usize) {
    let cup = Cup(this);
    let no_competition = u32::MAX;
    if cup.round_table() == 0 || cup.team_table() == 0 || cup.tie_tables() == 0 {
        show_error_at(CUP_STAGE_SOURCE, 0x19a);
        return;
    }
    covered!("cx02_17 cup stage saved");
    for (field, size, _) in CUP_STAGE_FILE_FIELDS {
        exe::crt_fwrite(this + field, size, 1, file);
    }
    let competition = read_usize(this + league::COMPETITION);
    if competition != 0 {
        exe::crt_fwrite(competition, 4, 1, file);
    } else {
        show_error_at(CUP_STAGE_SOURCE, 0x1b9);
        exe::crt_fwrite(&no_competition as *const u32 as usize, 4, 1, file);
    }
    exe::crt_fwrite(cup.round_table(), cup_round::SIZE as u32, cup.round_count() as i32 as u32, file);
    club_entries_to_ids(cup.team_table(), cup.team_count());
    exe::crt_fwrite(cup.team_table(), 6, cup.team_count() as i32 as u32, file);
    club_entries_from_ids(cup.team_table(), cup.team_count());
    let mut r = 0i32;
    while r < cup.round_count() as i32 {
        let ties = read_i16(cup.round_table() + r as usize * cup_round::SIZE + cup_round::TIES) as i32;
        exe::crt_fwrite(read_usize(cup.tie_tables() + 4 * r as usize), (ties * TIE_SIZE as i32) as u32, 1, file);
        r += 1;
    }
}

/// Comparer for `new_cup_stage_for_load`: it allocates (it cannot run twice), so only the original runs; the vtable,
/// the return and the three tables must be there.
pub(crate) unsafe extern "thiscall" fn compare_new_cup_stage_for_load(this: usize, teams: u32, rounds: u32, tie_counts: *const i16) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, *const i16) -> usize>(%%I0%%)(this, teams, rounds, tie_counts);
    let cup = Cup(this);
    SCOREBOARD.record(r == this && read_usize(this) == CUP_STAGE_VTABLE && cup.team_table() != 0 && cup.round_table() != 0 && cup.tie_tables() != 0,
        || format!("new_cup_stage_for_load({:#x}, {}, {}) returned {:#x}, vtable {:#x}", this, teams as i16, rounds as i16, r, read_usize(this)));
    r
}

/// The bytes a cup stage's load leaves: the object's first 0xb2 bytes, the round table, the team table and each
/// round's ties.
unsafe fn cup_stage_loaded_regions(this: usize) -> Vec<(usize, usize)> {
    let cup = Cup(this);
    let rounds = cup.round_count().max(0) as usize;
    let mut regions = vec![(this, 0xb2usize), (cup.round_table(), rounds * cup_round::SIZE), (cup.team_table(), cup.team_count().max(0) as usize * 6)];
    for r in 0..rounds {
        let ties = read_i16(cup.round_table() + r * cup_round::SIZE + cup_round::TIES).max(0) as usize;
        regions.push((read_usize(cup.tie_tables() + 4 * r), ties * TIE_SIZE));
    }
    regions
}

/// Comparer for `cup_stage_load_from_file`: the original reads; the file goes back to where it was and the copy reads
/// the same bytes again over the same tables; the object, the tables, the return and the file position must match.
/// The original's state is put back.
pub(crate) unsafe extern "thiscall" fn compare_cup_stage_load_from_file(this: usize, file: usize) -> u32 {
    let at = exe::crt_ftell(file);
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I1%%)(this, file);
    let end = exe::crt_ftell(file);
    let regions = cup_stage_loaded_regions(this);
    let original = crate::comparison::MemorySnapshot::read(&regions);
    exe::crt_fseek(file, at, 0);
    let c = cup_stage_load_from_file(this, file);
    let copy = crate::comparison::MemorySnapshot::read(&regions);
    let end_c = exe::crt_ftell(file);
    original.write_back();
    exe::crt_fseek(file, end, 0);
    SCOREBOARD.record(r == c && end == end_c && original == copy, || format!("cup_stage_load_from_file({:#x}) returned {} x {}, file at {} x {} (from {}), state equal {}", this, r, c, end, end_c, at, original == copy));
    r
}

/// Comparer for `cup_stage_save_to_file`: it writes the cup's temporary file (it cannot run twice), so only the
/// original runs; the team table must come back with its clubs. The copy is proven by the save proof (and by the
/// cup's file comparer of `cup_save_cup_to_disk`, which calls it).
pub(crate) unsafe extern "thiscall" fn compare_cup_stage_save_to_file(this: usize, file: usize) {
    let cup = Cup(this);
    let n = if cup.team_table() != 0 { cup.team_count().max(0) as usize } else { 0 };
    let before: Vec<usize> = (0..n).map(|k| read_usize(cup.team_table() + 6 * k)).collect();
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I2%%)(this, file);
    let after: Vec<usize> = (0..n).map(|k| read_usize(cup.team_table() + 6 * k)).collect();
    SCOREBOARD.record(before == after, || format!("cup_stage_save_to_file({:#x}): the team table changed", this));
}
