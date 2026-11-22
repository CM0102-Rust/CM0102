// ---------- batch %%LEVA%%: the competition statistics (1): the record copies, the club and player entries, the sorts and the records the season keeps (comp_screens.cpp) ----------

/// The `Comp_Stats` object (one per competition, at `comp + 8`): the offsets the comp_screens.cpp helpers use.
#[allow(dead_code)]
pub(crate) mod comp_stats_obj {
    /// Pointer to the club entry table (`CLUB_ENTRY_SIZE` bytes each), its capacity and its count.
    pub(crate) const CLUBS: usize = 0xc;
    pub(crate) const CLUB_CAPACITY: usize = 0x10;
    pub(crate) const CLUB_COUNT: usize = 0x14;
    /// u8: the statistic the club table is sorted by (0xff by id, 0xfe unsorted).
    pub(crate) const CLUB_SORT: usize = 0x18;
    /// Pointer to the player entry table (`PLAYER_ENTRY_SIZE` bytes each), its capacity and its count.
    pub(crate) const PLAYERS: usize = 0x19;
    pub(crate) const PLAYER_CAPACITY: usize = 0x1d;
    pub(crate) const PLAYER_COUNT: usize = 0x21;
    /// u8: the statistic the player table is sorted by (0xff by id, 0xfe unsorted).
    pub(crate) const PLAYER_SORT: usize = 0x25;
    /// The all-time records (`SEASON_RECORDS_SIZE` bytes) [inference: listed with year 0].
    pub(crate) const ALL_TIME_RECORDS: usize = 0x26;
    /// u8 / u32 inside the all-time records: the most titles and the club holding them.
    pub(crate) const MOST_TITLES: usize = 0x26;
    pub(crate) const MOST_TITLES_CLUB: usize = 0x27;
    /// The current season's records (`SEASON_RECORDS_SIZE` bytes).
    pub(crate) const SEASON_RECORDS: usize = 0x28f;
    /// u32 / u32 / u16 inside the season records (+0x6b): the most points, the club and a word cleared with them.
    pub(crate) const SEASON_MOST_POINTS: usize = 0x2fa;
    pub(crate) const SEASON_MOST_POINTS_CLUB: usize = 0x2fe;
    pub(crate) const SEASON_MOST_POINTS_EXTRA: usize = 0x302;
    /// The season rating record inside the season records (+0x189): f32 rating, the holder's name reference, the
    /// appearances (byte), the club (dword) and a word cleared with them.
    pub(crate) const SEASON_RATING: usize = 0x418;
    pub(crate) const SEASON_RATING_HOLDER: usize = 0x41c;
    pub(crate) const SEASON_RATING_GAMES: usize = 0x434;
    pub(crate) const SEASON_RATING_CLUB: usize = 0x435;
    pub(crate) const SEASON_RATING_EXTRA: usize = 0x439;
    /// The two keys of the archived seasons in the file list (`COMP_STATS_ARCHIVE_LIST`).
    pub(crate) const ARCHIVE_KEY1: usize = 0x4f8;
    pub(crate) const ARCHIVE_KEY2: usize = 0x4fc;
    /// The competition.
    pub(crate) const COMP: usize = 0x502;
    pub(crate) const CLUB_ENTRY_SIZE: usize = 0x76;
    pub(crate) const PLAYER_ENTRY_SIZE: usize = 0x2d;
    pub(crate) const SEASON_RECORDS_SIZE: usize = 0x269;
}

/// Fields of a player entry of the competition statistics (0x2d bytes).
#[allow(dead_code)]
pub(crate) mod comp_stats_player {
    pub(crate) const PERSON_ID: usize = 0x0;
    pub(crate) const CLUB_ID: usize = 0x4;
    pub(crate) const CLUB: usize = 0x8;
    /// u8: 0xff when the entry is new.
    pub(crate) const NEW_MARK: usize = 0x1c;
    /// i32: the appearances (the minimum the 0xd..0x10 statistics ask for).
    pub(crate) const APPEARANCES: usize = 0x29;
}

/// The comp_stats.CPP path for the error box.
pub(crate) const COMP_STATS_CPP: usize = 0x99ca94;
/// The game's file list that holds the archived seasons of the competition statistics.
pub(crate) const COMP_STATS_ARCHIVE_LIST: usize = 0xadae14;
/// `database_compare_comp_history_rows` (0x540780), the qsort order of the competition history table.
const COMPARE_COMP_HISTORY_ROWS: usize = 0x540780;
/// The two qsort orders of this item, by address (the hooks send them to the copy).
const COMPARE_TEAM_ENTRIES: usize = 0x4ad1a0;
const COMPARE_PLAYER_ENTRIES: usize = 0x4ad250;

/// `comp_stats_get_team_entry_value` (0x4a2ec0, thiscall(stats, entry, stat byte), ret 8, float in st0).
pub(crate) unsafe fn team_entry_value(stats: usize, entry: usize, stat: u8) -> f64 {
    let f: unsafe extern "thiscall" fn(usize, usize, u32) -> f64 = core::mem::transmute(0x4a2ec0usize);
    f(stats, entry, stat as u32)
}

/// `Comp_Stats::get_player_entry_value` (0x4a3250, thiscall(stats, entry, stat byte), ret 8, float in st0).
pub(crate) unsafe fn player_entry_value(stats: usize, entry: usize, stat: u8) -> f64 {
    let f: unsafe extern "thiscall" fn(usize, usize, u32) -> f64 = core::mem::transmute(0x4a3250usize);
    f(stats, entry, stat as u32)
}

/// `_ftol`: the x87 value truncated towards zero (64-bit, the caller keeps the low dword).
fn ftol(v: f64) -> i32 {
    v as i64 as i32
}

/// Rule: a run record of the competition statistics is copied whole: the run (game count and dates) and the dword
/// after it.
///
/// `comp_run_record_assign(source)`: `run_record_assign` (the game count, `run_record::START_DATE` and
/// `run_record::END_DATE`) and the dword at +0x14. Returns `this`.
/// Original: `comp_run_record_assign` (0x4ab5d0, comp_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn comp_run_record_assign(this: usize, source: usize) -> usize {
    run_record_assign(this, source);
    write_u32(this + 0x14, read_u32(source + 0x14));
    this
}

/// Rule: the season records of a competition are copied field by field (the compiler's `operator=`).
///
/// `comp_season_records_assign(source)`, in order: the dword at +0 and the byte at +4, the eight dwords +5..+0x25,
/// the match records at +0x29 and +0x4c (with the dword +0x48 between them), the two dwords and the word at +0x6b,
/// the match records +0x75 and +0x94, the four run records +0xb3, +0xcb, +0xe3 and +0xfb (`comp_run_record_assign`),
/// the tally records at +0x113, +0x16a and +0x1ac (byte, record holder, dword, word), the goals-in-match record at
/// +0x132 (byte, holder, match record), the rating record at +0x189 (dword, holder, byte, dword, word), the
/// discipline record at +0x1cb (two bytes, holder, dword, word) and the two age records at +0x1eb and +0x22a
/// (`player_age_record_assign`, 0x4a1fb0). Returns `this`.
/// Original: `comp_season_records_assign` (0x4ab5f0, comp_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn comp_season_records_assign(this: usize, source: usize) -> usize {
    let d = |o: usize| write_u32(this + o, read_u32(source + o));
    let w = |o: usize| write_u16(this + o, read_u16(source + o));
    let b = |o: usize| write_u8(this + o, read_u8(source + o));
    let age: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a1fb0usize);
    d(0);
    b(4);
    for o in [5, 9, 0xd, 0x11, 0x15, 0x19, 0x1d, 0x21, 0x25] {
        d(o);
    }
    match_record_assign(this + 0x29, source + 0x29);
    d(0x48);
    match_record_assign(this + 0x4c, source + 0x4c);
    d(0x6b);
    d(0x6f);
    w(0x73);
    match_record_assign(this + 0x75, source + 0x75);
    match_record_assign(this + 0x94, source + 0x94);
    for o in [0xb3, 0xcb, 0xe3, 0xfb] {
        comp_run_record_assign(this + o, source + o);
    }
    b(0x113);
    record_holder_assign(this + 0x114, source + 0x114);
    d(0x12c);
    w(0x130);
    b(0x132);
    record_holder_assign(this + 0x133, source + 0x133);
    match_record_assign(this + 0x14b, source + 0x14b);
    b(0x16a);
    record_holder_assign(this + 0x16b, source + 0x16b);
    d(0x183);
    w(0x187);
    d(0x189);
    record_holder_assign(this + 0x18d, source + 0x18d);
    b(0x1a5);
    d(0x1a6);
    w(0x1aa);
    b(0x1ac);
    record_holder_assign(this + 0x1ad, source + 0x1ad);
    d(0x1c5);
    w(0x1c9);
    b(0x1cb);
    b(0x1cc);
    record_holder_assign(this + 0x1cd, source + 0x1cd);
    d(0x1e5);
    w(0x1e9);
    age(this + 0x1eb, source + 0x1eb);
    age(this + 0x22a, source + 0x22a);
    this
}

/// One row of the season records list: the year (word) and the records.
const SEASON_RECORDS_ROW: usize = 2 + comp_stats_obj::SEASON_RECORDS_SIZE;
/// At most 500 rows.
const SEASON_RECORDS_ROWS: u32 = 500;

/// Rule: the records screen lists the all-time records (year 0), the current season (the competition's year) and
/// then the archived seasons, newest first, at most 500 rows.
///
/// `Comp_Stats::build_season_records_list(&count)`: `count` = 0; a block of 500 rows of 0x26b bytes (`crt_malloc`;
/// on failure the comp_stats.CPP 5909 box and 0). Row 0: year 0 and the all-time records
/// (`comp_stats_obj::ALL_TIME_RECORDS`). Then, with no season record of points yet (`SEASON_MOST_POINTS` = -1) and
/// a club table, the club table sorted by points (statistic 2, qsort with `comp_stats_compare_team_entries` unless
/// already sorted by it) and the leader's points (`_ftol`), its club and 0 written as the season record; with no
/// season rating (-1.0) `Comp_Stats::update_season_rating_record`. Row 1: the competition's year (word +0x40 of the
/// competition) and the season records. Then the archived seasons of the file list (`file_llist_get_nodes` with the
/// two archive keys), from the tail backwards (`llist_get_prev`): the year word and the records of each, until 500
/// rows; the list is deleted (`LLIST::scalar_deleting_destructor` 0x497910 with 1). Returns the block.
/// Original: `Comp_Stats::build_season_records_list` (0x4ab0c0, comp_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn comp_stats_build_season_records_list(stats: usize, count: *mut u32) -> usize {
    *count = 0;
    let block = exe::crt_malloc(SEASON_RECORDS_ROW as u32 * SEASON_RECORDS_ROWS);
    if block == 0 {
        show_error_box(COMP_STATS_CPP, 0x1715);
        return 0;
    }
    let row = |n: u32| block + n as usize * SEASON_RECORDS_ROW;
    write_u16(row(*count), 0);
    comp_season_records_assign(row(*count) + 2, stats + comp_stats_obj::ALL_TIME_RECORDS);
    *count += 1;
    if read_i32(stats + comp_stats_obj::SEASON_MOST_POINTS) == -1 && read_usize(stats + comp_stats_obj::CLUBS) != 0 {
        covered!("tel31_1 season points record from the table");
        if read_u8(stats + comp_stats_obj::CLUB_SORT) != 2 {
            write_u8(stats + comp_stats_obj::CLUB_SORT, 2);
            write_usize(G_COMP_STATS_SORT_CONTEXT, stats);
            exe::crt_qsort(read_usize(stats + comp_stats_obj::CLUBS), read_u32(stats + comp_stats_obj::CLUB_COUNT),
                comp_stats_obj::CLUB_ENTRY_SIZE as u32, COMPARE_TEAM_ENTRIES);
        }
        let leader = read_usize(stats + comp_stats_obj::CLUBS);
        write_i32(stats + comp_stats_obj::SEASON_MOST_POINTS, ftol(team_entry_value(stats, leader, 2)));
        write_u32(stats + comp_stats_obj::SEASON_MOST_POINTS_CLUB, read_u32(leader));
        write_u16(stats + comp_stats_obj::SEASON_MOST_POINTS_EXTRA, 0);
    }
    // fcomp sets C3 for equal and for unordered: a NaN rating also asks for the record
    let rating = read_f32(stats + comp_stats_obj::SEASON_RATING);
    if rating == -1.0 || rating.is_nan() {
        comp_stats_update_season_rating_record(stats);
    }
    write_u16(row(*count), read_u16(read_usize(stats + comp_stats_obj::COMP) + 0x40));
    comp_season_records_assign(row(*count) + 2, stats + comp_stats_obj::SEASON_RECORDS);
    *count += 1;
    let list = exe::file_llist_get_nodes(read_usize(COMP_STATS_ARCHIVE_LIST), read_u32(stats + comp_stats_obj::ARCHIVE_KEY1),
        read_u32(stats + comp_stats_obj::ARCHIVE_KEY2));
    if list != 0 {
        write_usize(list + llist::CURSOR, read_usize(list + llist::TAIL));
        let mut data = exe::llist_get_prev(list);
        while data != 0 && *count != SEASON_RECORDS_ROWS {
            covered!("tel31_1 archived season row");
            let r = row(*count);
            *count += 1;
            write_u16(r, read_u16(data));
            comp_season_records_assign(r + 2, data + 2);
            data = exe::llist_get_prev(list);
        }
        let delete: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(0x497910usize);
        delete(list, 1);
    }
    block
}

/// Rule: a new club entry of the competition statistics is zeroed with the club's id; the table grows two at a time.
///
/// `Comp_Stats::add_club_entry(club)`: no club: the comp_stats.CPP 6345 box and 0. A full table
/// (`comp_stats_obj::CLUB_COUNT` = capacity) grows by 2 (`crt_realloc` to capacity × 0x76; on failure the
/// comp_stats.CPP 6359 box, `game_fatal_cleanup` and `exit(-1)`). The entry after the last one is zeroed (0x76 bytes),
/// takes the club's id, the count goes up, the table is marked unsorted (0xfe) and `Comp_Stats::restore_team_runs`
/// (0x4a22f0) fills the entry's runs. Returns the entry.
/// Original: `Comp_Stats::add_club_entry` (0x4aceb0, comp_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn comp_stats_add_club_entry(stats: usize, club: usize) -> usize {
    if club == 0 {
        show_error_box(COMP_STATS_CPP, 0x18c9);
        return 0;
    }
    let capacity = read_u32(stats + comp_stats_obj::CLUB_CAPACITY);
    if read_u32(stats + comp_stats_obj::CLUB_COUNT) == capacity {
        covered!("tel31_1 club table grows");
        let capacity = capacity.wrapping_add(2);
        write_u32(stats + comp_stats_obj::CLUB_CAPACITY, capacity);
        let table = exe::crt_realloc(read_usize(stats + comp_stats_obj::CLUBS), capacity.wrapping_mul(comp_stats_obj::CLUB_ENTRY_SIZE as u32));
        write_usize(stats + comp_stats_obj::CLUBS, table);
        if table == 0 {
            comp_stats_fatal(0x18d7);
        }
    }
    let entry = read_usize(stats + comp_stats_obj::CLUBS) + read_u32(stats + comp_stats_obj::CLUB_COUNT) as usize * comp_stats_obj::CLUB_ENTRY_SIZE;
    core::ptr::write_bytes(entry as *mut u8, 0, comp_stats_obj::CLUB_ENTRY_SIZE);
    write_u32(entry, read_u32(club));
    write_u32(stats + comp_stats_obj::CLUB_COUNT, read_u32(stats + comp_stats_obj::CLUB_COUNT).wrapping_add(1));
    write_u8(stats + comp_stats_obj::CLUB_SORT, 0xfe);
    let restore: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x4a22f0usize);
    restore(stats, entry);
    entry
}

/// The failed reallocation of a statistics table: the comp_stats.CPP box at `line`, `game_fatal_cleanup`, the no-op
/// 0x5a8f60 and `exit(-1)`.
unsafe fn comp_stats_fatal(line: u32) -> ! {
    show_error_box(COMP_STATS_CPP, line);
    exe::game_fatal_cleanup();
    exe::shared_noop_ret0_5a8f60();
    exe::crt_exit(-1);
    unreachable!()
}

/// Rule: a new player entry of the competition statistics is zeroed with the player's id and the club he played
/// for; the table grows 22 at a time.
///
/// `Comp_Stats::add_player_entry(player, club)`: no player: the comp_stats.CPP 6402 box and 0. A full table grows by
/// 0x16 (`crt_realloc` to capacity × 0x2d; on failure the comp_stats.CPP 6416 box and `exit(-1)`). The entry after the
/// last is zeroed (0x2d bytes), takes the player's id and the `club` dword as passed, the new mark 0xff at +0x1c, the
/// table is marked unsorted (0xfe) and the count goes up. Returns the entry.
/// Original: `Comp_Stats::add_player_entry` (0x4ad030, comp_screens.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn comp_stats_add_player_entry(stats: usize, player: usize, club: u32) -> usize {
    if player == 0 {
        show_error_box(COMP_STATS_CPP, 0x1902);
        return 0;
    }
    let capacity = read_u32(stats + comp_stats_obj::PLAYER_CAPACITY);
    if read_u32(stats + comp_stats_obj::PLAYER_COUNT) == capacity {
        covered!("tel31_1 player table grows");
        let capacity = capacity.wrapping_add(0x16);
        write_u32(stats + comp_stats_obj::PLAYER_CAPACITY, capacity);
        let table = exe::crt_realloc(read_usize(stats + comp_stats_obj::PLAYERS), capacity.wrapping_mul(comp_stats_obj::PLAYER_ENTRY_SIZE as u32));
        write_usize(stats + comp_stats_obj::PLAYERS, table);
        if table == 0 {
            comp_stats_fatal(0x1910);
        }
    }
    let entry = read_usize(stats + comp_stats_obj::PLAYERS) + read_u32(stats + comp_stats_obj::PLAYER_COUNT) as usize * comp_stats_obj::PLAYER_ENTRY_SIZE;
    core::ptr::write_bytes(entry as *mut u8, 0, comp_stats_obj::PLAYER_ENTRY_SIZE);
    write_u32(entry + comp_stats_player::PERSON_ID, read_u32(player));
    write_u32(entry + comp_stats_player::CLUB_ID, club);
    write_u8(entry + comp_stats_player::NEW_MARK, 0xff);
    write_u8(stats + comp_stats_obj::PLAYER_SORT, 0xfe);
    write_u32(stats + comp_stats_obj::PLAYER_COUNT, read_u32(stats + comp_stats_obj::PLAYER_COUNT).wrapping_add(1));
    entry
}

/// The three-way float order of the statistics sorts: `a` below `b` (or unordered) gives `below`, above gives
/// `-below`, equal gives `None` (the tie goes to the ids).
fn stat_order(a: f32, b: f32, below: i32) -> Option<i32> {
    if !(a >= b) {
        Some(below)
    } else if a > b {
        Some(-below)
    } else {
        None
    }
}

/// The tie-break of the statistics sorts: the first dword (the id), ascending, signed.
unsafe fn id_order(a: usize, b: usize) -> i32 {
    let (x, y) = (read_i32(a), read_i32(b));
    if x > y {
        1
    } else if x >= y {
        0
    } else {
        -1
    }
}

/// Rule: the club table of the competition statistics sorts by the chosen statistic, highest first (statistic 11,
/// lowest first), then by club id.
///
/// `comp_stats_compare_team_entries(a, b)`, the qsort order: the statistic is the club sort byte of the stats in
/// `G_COMP_STATS_SORT_CONTEXT`; with 0xff only the ids count. Each value is `comp_stats_get_team_entry_value`
/// stored as a float; with statistic 11 a smaller value goes first, otherwise a larger one; equal values (or 0xff):
/// the larger id goes after.
/// Original: `comp_stats_compare_team_entries` (0x4ad1a0, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_stats_compare_team_entries(a: *const usize, b: *const usize) -> i32 {
    let (a, b) = (a as usize, b as usize);
    let stats = read_usize(G_COMP_STATS_SORT_CONTEXT);
    let stat = read_u8(stats + comp_stats_obj::CLUB_SORT);
    if stat != 0xff {
        let va = team_entry_value(stats, a, stat) as f32;
        let vb = team_entry_value(read_usize(G_COMP_STATS_SORT_CONTEXT), b, read_u8(read_usize(G_COMP_STATS_SORT_CONTEXT) + comp_stats_obj::CLUB_SORT)) as f32;
        let below = if read_u8(read_usize(G_COMP_STATS_SORT_CONTEXT) + comp_stats_obj::CLUB_SORT) == 0xb { -1 } else { 1 };
        if let Some(r) = stat_order(va, vb, below) {
            return r;
        }
    }
    id_order(a, b)
}

/// The club entry of a player entry's club in the sort stats, or `None` (no club: the comp_stats.CPP 2274 box).
unsafe fn player_club_entry(stats: usize, player: usize) -> Option<usize> {
    let club = club_at(read_i32(player + comp_stats_player::CLUB) as isize).0;
    if club == 0 {
        show_error_box(COMP_STATS_CPP, 0x8e2);
        return None;
    }
    let entry = exe::comp_stats_find_club_entry(stats, club);
    if entry == 0 { None } else { Some(entry) }
}

/// Statistic 0x11's value of a player entry: 0 when his club's games (team statistic 1) × 0.5 are more than his
/// appearances (player statistic 1), i.e. he played under half his club's games.
unsafe fn rated_player_value(player: usize, value: f32) -> f32 {
    let stats = read_usize(G_COMP_STATS_SORT_CONTEXT);
    let club_games = match player_club_entry(stats, player) {
        Some(e) => team_entry_value(stats, e, 1) as f32,
        None => 0.0,
    };
    let games = player_entry_value(read_usize(G_COMP_STATS_SORT_CONTEXT), player, 1);
    if club_games as f64 * 0.5 > games {
        covered!("tel31_1 rating below half the games");
        0.0
    } else {
        value
    }
}

/// Statistics 0xd..0x10's value of a player entry: 0 when his club's games (team statistic 1, 0 with no entry) × 35
/// are more than his appearances (`comp_stats_player::APPEARANCES`).
unsafe fn per_game_player_value(player: usize, value: f32) -> f32 {
    let stats = read_usize(G_COMP_STATS_SORT_CONTEXT);
    let club_games = match player_club_entry(stats, player) {
        Some(e) => team_entry_value(stats, e, 1),
        None => 0.0,
    };
    if club_games * 35.0 > read_i32(player + comp_stats_player::APPEARANCES) as f64 {
        covered!("tel31_1 per-game stat below the minimum");
        0.0
    } else {
        value
    }
}

/// Rule: the player table of the competition statistics sorts by the chosen statistic, highest first (statistic
/// 0x12, lowest first), then by person id; the average rating (0x11) only counts players with half their club's
/// games, and the per-game statistics (0xd..0x10) those with 35 times their club's games in appearances.
///
/// `comp_stats_compare_player_entries(a, b)`, the qsort order: the statistic is the player sort byte of the stats in
/// `G_COMP_STATS_SORT_CONTEXT`; with 0xff only the ids count. Each value is `Comp_Stats::get_player_entry_value`
/// stored as a float; statistic 0x11: a player whose club (`comp_stats_find_club_entry` of the club at +8; no club:
/// the comp_stats.CPP 2274 box) has team statistic 1 × 0.5 above his player statistic 1 counts 0; statistics
/// 0xd..0x10: one whose club's team statistic 1 (0 with no entry) × 35.0 is above his appearances (+0x29) counts 0.
/// Statistic 0x12: smaller first, otherwise larger first; equal values: the larger id goes after.
/// Original: `comp_stats_compare_player_entries` (0x4ad250, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_stats_compare_player_entries(a: *const usize, b: *const usize) -> i32 {
    let (a, b) = (a as usize, b as usize);
    let sort = || read_u8(read_usize(G_COMP_STATS_SORT_CONTEXT) + comp_stats_obj::PLAYER_SORT);
    let stat = sort();
    if stat != 0xff {
        let mut va = player_entry_value(read_usize(G_COMP_STATS_SORT_CONTEXT), a, stat) as f32;
        let mut vb = player_entry_value(read_usize(G_COMP_STATS_SORT_CONTEXT), b, sort()) as f32;
        let stat = sort();
        if stat == 0x11 {
            va = rated_player_value(a, va);
            vb = rated_player_value(b, vb);
        } else if (0xd..=0x10).contains(&stat) {
            va = per_game_player_value(a, va);
            vb = per_game_player_value(b, vb);
        }
        let below = if sort() == 0x12 { -1 } else { 1 };
        if let Some(r) = stat_order(va, vb, below) {
            return r;
        }
    }
    id_order(a, b)
}

/// One row of the competition history table (0x1a bytes): the competition id at +4 and the champion club at +0xa.
const COMP_HISTORY_ROW_SIZE: usize = 0x1a;

/// Rule: the all-time record of most titles is the club that won the competition most often in the history table;
/// on a tie the first to reach the count keeps it.
///
/// `Comp_Stats::update_most_titles_record()`: the history table (`G_CLUB_COMP_HISTORY_TABLE`, 0x1a-byte rows) sorted
/// (qsort with `database_compare_comp_history_rows`); a word per club (`crt_operator_new` of 2 × `G_CLUB_COUNT`; on
/// failure the comp_stats.CPP 6619 box and 0), zeroed. The rows of this competition (row +4 = the competition's id;
/// the sort keeps them together: the walk stops at the first other row after them): with a champion (+0xa), his count
/// goes up and, when the record is empty (0xff) or the count is above it (signed), the record takes the count's low
/// byte and the club's id. Returns 1 when the record's club ended up different from the one before, else 0.
/// Original: `Comp_Stats::update_most_titles_record` (0x4ad680, comp_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn comp_stats_update_most_titles_record(stats: usize) -> u32 {
    let before = read_u32(stats + comp_stats_obj::MOST_TITLES_CLUB);
    let mut changed = 0;
    exe::crt_qsort(read_usize(G_CLUB_COMP_HISTORY_TABLE), read_u32(G_CLUB_COMP_HISTORY_COUNT), COMP_HISTORY_ROW_SIZE as u32,
        COMPARE_COMP_HISTORY_ROWS);
    let size = read_u32(G_CLUB_COUNT).wrapping_mul(2);
    let tally = exe::crt_operator_new(size);
    if tally == 0 {
        show_error_box(COMP_STATS_CPP, 0x19db);
        return 0;
    }
    core::ptr::write_bytes(tally as *mut u8, 0, size as usize);
    let mut found = false;
    let mut i = 0;
    while i < read_i32(G_CLUB_COMP_HISTORY_COUNT) {
        let row = read_usize(G_CLUB_COMP_HISTORY_TABLE) + i as usize * COMP_HISTORY_ROW_SIZE;
        if read_u32(row + 4) == read_u32(read_usize(stats + comp_stats_obj::COMP) + 4) {
            let club = read_usize(row + 0xa);
            if club != 0 {
                let id = read_u32(club);
                let slot = tally.wrapping_add((id as usize).wrapping_mul(2));
                write_i16(slot, read_i16(slot).wrapping_add(1));
                let n = read_i16(slot);
                let record = read_u8(stats + comp_stats_obj::MOST_TITLES);
                if record == 0xff || n > record as i8 as i16 {
                    covered!("tel31_1 most titles record taken");
                    write_u8(stats + comp_stats_obj::MOST_TITLES, n as u8);
                    write_u32(stats + comp_stats_obj::MOST_TITLES_CLUB, id);
                    changed = (id != before) as u32;
                }
            }
            found = true;
        } else if found {
            break;
        }
        i += 1;
    }
    exe::crt_operator_delete(tally);
    changed
}

/// Rule: the season's best average rating is the player at the top of the rating sort (statistic 0x11).
///
/// `Comp_Stats::update_season_rating_record()`: no player table: 0. Unless the players are already sorted by 0x11:
/// the clubs sorted by id (sort byte 0xff, qsort with `comp_stats_compare_team_entries`) when they were not, then the
/// players by 0x11 (qsort with `comp_stats_compare_player_entries`, 0x2d bytes). The first player gives the season
/// rating record: the rating (player statistic 0x11, as a float), his games (statistic 1 through `_ftol`, a byte),
/// the name reference (`fill_staff_name_ref` of his person), his club (+8) and 0 in the word after. Returns 1.
/// Original: `Comp_Stats::update_season_rating_record` (0x4ad7f0, comp_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn comp_stats_update_season_rating_record(stats: usize) -> u32 {
    if read_usize(stats + comp_stats_obj::PLAYERS) == 0 {
        return 0;
    }
    if read_u8(stats + comp_stats_obj::PLAYER_SORT) != 0x11 {
        if read_u8(stats + comp_stats_obj::CLUB_SORT) != 0xff {
            write_u8(stats + comp_stats_obj::CLUB_SORT, 0xff);
            write_usize(G_COMP_STATS_SORT_CONTEXT, stats);
            exe::crt_qsort(read_usize(stats + comp_stats_obj::CLUBS), read_u32(stats + comp_stats_obj::CLUB_COUNT),
                comp_stats_obj::CLUB_ENTRY_SIZE as u32, COMPARE_TEAM_ENTRIES);
        }
        write_u8(stats + comp_stats_obj::PLAYER_SORT, 0x11);
        write_usize(G_COMP_STATS_SORT_CONTEXT, stats);
        exe::crt_qsort(read_usize(stats + comp_stats_obj::PLAYERS), read_u32(stats + comp_stats_obj::PLAYER_COUNT),
            comp_stats_obj::PLAYER_ENTRY_SIZE as u32, COMPARE_PLAYER_ENTRIES);
    }
    let top = read_usize(stats + comp_stats_obj::PLAYERS);
    write_f32(stats + comp_stats_obj::SEASON_RATING, player_entry_value(stats, top, 0x11) as f32);
    write_u8(stats + comp_stats_obj::SEASON_RATING_GAMES, ftol(player_entry_value(stats, top, 1)) as u8);
    exe::fill_staff_name_ref(staff_at(read_i32(top + comp_stats_player::PERSON_ID) as isize).0, stats + comp_stats_obj::SEASON_RATING_HOLDER);
    write_u32(stats + comp_stats_obj::SEASON_RATING_CLUB, read_u32(top + comp_stats_player::CLUB));
    write_u16(stats + comp_stats_obj::SEASON_RATING_EXTRA, 0);
    1
}

/// Rule: clearing the season records leaves every record empty (-1, the rating -1.0).
///
/// `comp_season_records_clear(records)`: no records: the comp_stats.CPP 6788 box and 0. Otherwise -1 in the rating
/// float (+0x189 as -1.0), the dwords +1, +0x25, +0x48, +0x6b, +0x7d, +0x9c, +0xb3, +0xcb, +0xe3, +0xfb, +0x213 and
/// +0x252, the words +5, +0xd and +0x15 and the bytes +0, +0x1d, +0x113, +0x132, +0x16a, +0x1ac and +0x1cb. Returns 1.
/// Original: `comp_season_records_clear` (0x4ad8c0, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_season_records_clear(records: usize) -> u32 {
    if records == 0 {
        show_error_box(COMP_STATS_CPP, 0x1a84);
        return 0;
    }
    write_u32(records + 0x189, 0xbf800000);
    for o in [1, 0x25, 0x48, 0x6b, 0x7d, 0x9c, 0xb3, 0xcb, 0xe3, 0xfb, 0x213, 0x252] {
        write_u32(records + o, u32::MAX);
    }
    for o in [5, 0xd, 0x15] {
        write_u16(records + o, u16::MAX);
    }
    for o in [0, 0x1d, 0x113, 0x132, 0x16a, 0x1ac, 0x1cb] {
        write_u8(records + o, 0xff);
    }
    1
}

/// Rule: a player's position group for the statistics: goalkeeper (1), defender (4), midfielder (0x10) or attacker
/// (0x40), by the positions rated 20.
///
/// `comp_stats_get_player_position_group(sheet)`: goalkeeper 20: 1. A sweeper, defender or wing back 20: defender (4)
/// unless he is also a midfielder, defensive or attacking midfielder 20 and the sum of those three is not below the
/// defender rating × 1.5. Then a midfielder of any kind (20) is a midfielder (0x10) unless he is an attacker 20 with
/// the central side at least as good as the left and the right sides, which makes him an attacker (0x40); anybody
/// else is an attacker (0x40). The group comes back in ax.
/// Original: `comp_stats_get_player_position_group` (0x4ae4f0, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_stats_get_player_position_group(sheet: usize) -> u16 {
    let p = PlayerSheet(sheet);
    if p.goalkeeper() == 20 {
        return 1;
    }
    let midfield = p.midfielder() == 20 || p.defensive_midfielder() == 20 || p.attacking_midfielder() == 20;
    if p.sweeper() == 20 || p.defender() == 20 || p.wing_back() == 20 {
        if !midfield {
            return 4;
        }
        let sum = p.midfielder() as i32 + p.defensive_midfielder() as i32 + p.attacking_midfielder() as i32;
        if (sum as f64) < p.defender() as f64 * read_f64(rdata::F64_1_5) {
            covered!("tel31_1 defender before midfielder");
            return 4;
        }
    }
    if !midfield {
        return 0x40;
    }
    if p.attacker() < 20 || p.central() < p.left_side() || p.central() < p.right_side() {
        return 0x10;
    }
    0x40
}

// ---------- comparers ----------

/// Comparer for `comp_run_record_assign`: side by side over the 0x18 bytes copied and the return.
pub(crate) unsafe extern "thiscall" fn compare_comp_run_record_assign(this: usize, source: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%);
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&[(this, 0x18)], || o(this, source), || comp_run_record_assign(this, source));
    SCOREBOARD.record(equal && ro == rr, || format!("comp_run_record_assign({:#x}, {:#x}): differences in {:?}", this, source, first_differences(&mo, &mr)));
    ro
}

/// Comparer for `comp_season_records_assign`: side by side over the 0x269 bytes copied and the return.
pub(crate) unsafe extern "thiscall" fn compare_comp_season_records_assign(this: usize, source: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I1%%);
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&[(this, comp_stats_obj::SEASON_RECORDS_SIZE)], || o(this, source),
        || comp_season_records_assign(this, source));
    SCOREBOARD.record(equal && ro == rr, || format!("comp_season_records_assign({:#x}, {:#x}): differences in {:?}", this, source, first_differences(&mo, &mr)));
    ro
}

/// Comparer for `comp_stats_build_season_records_list`: the block is allocated and the archive list is read from
/// disk, so only the original runs; the copy rebuilds the rows into its own block afterwards and the two blocks are
/// compared row by row (the stats it may write were already written by the original, so the copy sees the same).
pub(crate) unsafe extern "thiscall" fn compare_comp_stats_build_season_records_list(stats: usize, count: *mut u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *mut u32) -> usize>(%%I2%%);
    let block = o(stats, count);
    if block == 0 {
        return block;
    }
    let mut n = 0u32;
    let mine = comp_stats_build_season_records_list(stats, &mut n);
    if mine != 0 {
        let size = *count as usize * SEASON_RECORDS_ROW;
        let equal = n == *count && core::slice::from_raw_parts(block as *const u8, size) == core::slice::from_raw_parts(mine as *const u8, size);
        SCOREBOARD.record(equal, || format!("comp_stats_build_season_records_list({:#x}): rows {} x {}", stats, *count, n));
        exe::crt_free(mine);
    }
    block
}

/// Comparer for `comp_stats_add_club_entry`: it reallocates, so only the original runs; the plan is checked after
/// it: the entry is the last one, with the club's id, and the table is marked unsorted.
pub(crate) unsafe extern "thiscall" fn compare_comp_stats_add_club_entry(stats: usize, club: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I3%%);
    let entry = o(stats, club);
    if entry != 0 {
        let n = read_u32(stats + comp_stats_obj::CLUB_COUNT);
        let ok = n >= 1 && entry == read_usize(stats + comp_stats_obj::CLUBS) + (n as usize - 1) * comp_stats_obj::CLUB_ENTRY_SIZE
            && read_u32(entry) == read_u32(club) && read_u8(stats + comp_stats_obj::CLUB_SORT) == 0xfe;
        SCOREBOARD.record(ok, || format!("comp_stats_add_club_entry({:#x}, {:#x}): entry {:#x} out of plan", stats, club, entry));
    }
    entry
}

/// Comparer for `comp_stats_add_player_entry`: it reallocates, so only the original runs; the plan is checked after
/// it: the last entry, the player's id, the club dword, the new mark and the table marked unsorted.
pub(crate) unsafe extern "thiscall" fn compare_comp_stats_add_player_entry(stats: usize, player: usize, club: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(%%I4%%);
    let entry = o(stats, player, club);
    if entry != 0 {
        let n = read_u32(stats + comp_stats_obj::PLAYER_COUNT);
        let ok = n >= 1 && entry == read_usize(stats + comp_stats_obj::PLAYERS) + (n as usize - 1) * comp_stats_obj::PLAYER_ENTRY_SIZE
            && read_u32(entry) == read_u32(player) && read_u32(entry + comp_stats_player::CLUB_ID) == club
            && read_u8(entry + comp_stats_player::NEW_MARK) == 0xff && read_u8(stats + comp_stats_obj::PLAYER_SORT) == 0xfe;
        SCOREBOARD.record(ok, || format!("comp_stats_add_player_entry({:#x}, {:#x}): entry {:#x} out of plan", stats, player, entry));
    }
    entry
}

compare_order!(compare_comp_stats_compare_team_entries, comp_stats_compare_team_entries, %%I5%%);
compare_order!(compare_comp_stats_compare_player_entries, comp_stats_compare_player_entries, %%I6%%);

/// Comparer for `comp_stats_update_most_titles_record`: side by side over the record (titles byte and club), the
/// history table (the sort rewrites it) and the return; the tally block is the copy's own.
pub(crate) unsafe extern "thiscall" fn compare_comp_stats_update_most_titles_record(stats: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I7%%);
    let history = (read_usize(G_CLUB_COMP_HISTORY_TABLE), read_u32(G_CLUB_COMP_HISTORY_COUNT) as usize * COMP_HISTORY_ROW_SIZE);
    let regions = [(stats + comp_stats_obj::MOST_TITLES, 5), history];
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(stats), || comp_stats_update_most_titles_record(stats));
    SCOREBOARD.record(equal && ro == rr, || format!("comp_stats_update_most_titles_record({:#x}): return {} x {}, differences in {:?}", stats, ro, rr, first_differences(&mo, &mr)));
    ro
}

/// Comparer for `comp_stats_update_season_rating_record`: side by side over the two tables (the sorts rewrite them),
/// the sort bytes, the rating record and the sort context.
pub(crate) unsafe extern "thiscall" fn compare_comp_stats_update_season_rating_record(stats: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I8%%);
    let regions = [
        (read_usize(stats + comp_stats_obj::CLUBS), read_u32(stats + comp_stats_obj::CLUB_COUNT) as usize * comp_stats_obj::CLUB_ENTRY_SIZE),
        (read_usize(stats + comp_stats_obj::PLAYERS), read_u32(stats + comp_stats_obj::PLAYER_COUNT) as usize * comp_stats_obj::PLAYER_ENTRY_SIZE),
        (stats + comp_stats_obj::CLUB_SORT, 1),
        (stats + comp_stats_obj::PLAYER_SORT, 1),
        (stats + comp_stats_obj::SEASON_RATING, 0x23),
        (G_COMP_STATS_SORT_CONTEXT, 4),
    ];
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(stats), || comp_stats_update_season_rating_record(stats));
    SCOREBOARD.record(equal && ro == rr, || format!("comp_stats_update_season_rating_record({:#x}): differences in {:?}", stats, first_differences(&mo, &mr)));
    ro
}

/// Comparer for `comp_season_records_clear`: side by side over the 0x269 bytes of the records and the return.
pub(crate) unsafe extern "C" fn compare_comp_season_records_clear(records: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize) -> u32>(%%I9%%);
    if records == 0 {
        return o(records);
    }
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&[(records, comp_stats_obj::SEASON_RECORDS_SIZE)], || o(records),
        || comp_season_records_clear(records));
    SCOREBOARD.record(equal && ro == rr, || format!("comp_season_records_clear({:#x}): differences in {:?}", records, first_differences(&mo, &mr)));
    ro
}

/// Comparer for `comp_stats_get_player_position_group`: the group, side by side (the routine only reads).
pub(crate) unsafe extern "C" fn compare_comp_stats_get_player_position_group(sheet: usize) -> u16 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u16>(%%I10%%)(sheet);
    let n = comp_stats_get_player_position_group(sheet);
    SCOREBOARD.record(r == n, || format!("comp_stats_get_player_position_group({:#x}) original={:#x} rust={:#x}", sheet, r, n));
    r
}
