// ---------- tel41_3: the competition statistics at the end of a season: the all-time records, the archive and the record news (comp_screens.cpp) ----------

/// The `Comp_Stats` fields of the season end (offsets in the object; the all-time records start at +0x26 and the
/// season records at +0x28f, `comp_stats_obj`).
#[allow(dead_code)]
mod comp_stats_season_end {
    /// i32 / u32 / u16 in the all-time records (+0x6b): the most points in a season, the club and the season's year.
    pub(crate) const ALL_TIME_MOST_POINTS: usize = 0x91;
    pub(crate) const ALL_TIME_MOST_POINTS_CLUB: usize = 0x95;
    pub(crate) const ALL_TIME_MOST_POINTS_YEAR: usize = 0x99;
    /// The all-time rating record (+0x189): f32 rating, the holder's name reference, the games (byte), the club and
    /// the season's year (word).
    pub(crate) const ALL_TIME_RATING: usize = 0x1af;
    pub(crate) const ALL_TIME_RATING_HOLDER: usize = 0x1b3;
    pub(crate) const ALL_TIME_RATING_GAMES: usize = 0x1cb;
    pub(crate) const ALL_TIME_RATING_CLUB: usize = 0x1cc;
    pub(crate) const ALL_TIME_RATING_YEAR: usize = 0x1d0;
    /// i16: how many season ends the statistics went through; the record news waits for the third.
    pub(crate) const SEASONS_ENDED: usize = 0x500;
    /// The 22 "record broken" flags (dword each, record kinds 1..22) and the dword "some flag is set".
    pub(crate) const RECORD_FLAGS: usize = 0x50a;
    pub(crate) const ANY_RECORD_FLAG: usize = 0x562;
}

/// Record kind of a flag: `RECORD_FLAGS + 4 × (kind − 1)`.
const fn comp_stats_record_flag(kind: usize) -> usize {
    comp_stats_season_end::RECORD_FLAGS + 4 * (kind - 1)
}

/// The record kinds the season end flags: most titles (1), most points in a season (8) and the best average
/// rating (0x12).
const COMP_STATS_KIND_MOST_TITLES: usize = 1;
const COMP_STATS_KIND_MOST_POINTS: usize = 8;
const COMP_STATS_KIND_RATING: usize = 0x12;

/// x87 `fcomp` + `test ah, 0x40`: C3 set, i.e. equal or unordered.
fn comp_stats_c3(a: f64, b: f64) -> bool {
    a == b || a.is_nan() || b.is_nan()
}

/// Rule: at a season's end the all-time records of a competition take the season's most points and best average
/// rating when they beat them, and the most titles is recounted; from the third season end on, a broken record of a
/// league division (most points, rating with more than 3 games, most titles from 5) raises its news flag.
///
/// `Comp_Stats::update_season_end_records()`, for a competition that is not a friendly one
/// (`Friendly::is_friendly_competition` of the competition's record, asked again before each part):
/// 1. `Comp_Stats::update_most_titles_record`; when the record's club changed and the record (signed byte +0x26) is
///    at least 5: flag 1 and the "any flag" dword set.
/// 2. Unless the competition's byte +0x42 is 4: with no season most points yet (-1) and a club table, the club table
///    sorted by points (statistic 2, unless already) and the leader's points (`_ftol` of statistic 2), its id and 0
///    written as the season record. A season record above -1 that beats the all-time one (signed; or the all-time is
///    -1) is copied to it (points, club, the word, then the competition's year in the word); with at least 3 season
///    ends (signed word +0x500) and a league division (`comp_util_is_league_division` of the competition id): flag 8.
/// 3. With no season rating (-1.0, or unordered) `Comp_Stats::update_season_rating_record`. A season rating above
///    -1.0 that beats the all-time one (or the all-time is -1.0/unordered) is copied to it: the rating, the holder
///    (`record_holder_assign`), the games byte, the club, the word and then the competition's year; with at least 3
///    season ends, a league division and more than 3 games (signed byte): flag 0x12.
/// The season end count goes up by 1 (always). Returns 1.
/// Original: `Comp_Stats::update_season_end_records` (0x4a19f0, comp_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn comp_stats_update_season_end_records(stats: usize) -> u32 {
    use comp_stats_season_end as f;
    let comp = read_usize(stats + comp_stats_obj::COMP);
    let friendly = || exe::friendly_is_friendly_competition(read_usize(G_FRIENDLY), read_usize(read_usize(stats + comp_stats_obj::COMP) + league::COMPETITION)) != 0;
    let league_division = || exe::comp_util_is_league_division(read_i32(read_usize(read_usize(stats + comp_stats_obj::COMP) + league::COMPETITION))) != 0;
    if !friendly() && comp_stats_update_most_titles_record(stats) != 0 && read_u8(stats + comp_stats_obj::MOST_TITLES) as i8 >= 5 {
        covered!("tel41_3 most titles record news");
        write_u32(stats + comp_stats_record_flag(COMP_STATS_KIND_MOST_TITLES), 1);
        write_u32(stats + f::ANY_RECORD_FLAG, 1);
    }
    if !friendly() && read_u8(comp + league::CLUB_SIDES) != 4 {
        if read_i32(stats + comp_stats_obj::SEASON_MOST_POINTS) == -1 && read_usize(stats + comp_stats_obj::CLUBS) != 0 {
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
        let season = read_i32(stats + comp_stats_obj::SEASON_MOST_POINTS);
        let all_time = read_i32(stats + f::ALL_TIME_MOST_POINTS);
        // tel41_3: the season's most points beats the all-time record
        if season > -1 && (all_time == -1 || season > all_time) {
            write_i32(stats + f::ALL_TIME_MOST_POINTS, season);
            write_u32(stats + f::ALL_TIME_MOST_POINTS_CLUB, read_u32(stats + comp_stats_obj::SEASON_MOST_POINTS_CLUB));
            write_u16(stats + f::ALL_TIME_MOST_POINTS_YEAR, read_u16(stats + comp_stats_obj::SEASON_MOST_POINTS_EXTRA));
            write_u16(stats + f::ALL_TIME_MOST_POINTS_YEAR, read_u16(read_usize(stats + comp_stats_obj::COMP) + league::YEAR));
            if read_i16(stats + f::SEASONS_ENDED) >= 3 && league_division() {
                covered!("tel41_3 most points record news");
                write_u32(stats + comp_stats_record_flag(COMP_STATS_KIND_MOST_POINTS), 1);
                write_u32(stats + f::ANY_RECORD_FLAG, 1);
            }
        }
    }
    if !friendly() {
        let minus_one = read_f64(rdata::F64_NEG_1);
        if comp_stats_c3(read_f32(stats + comp_stats_obj::SEASON_RATING) as f64, minus_one) {
            comp_stats_update_season_rating_record(stats);
        }
        let season = read_f32(stats + comp_stats_obj::SEASON_RATING) as f64;
        if season > minus_one {
            let all_time = read_f32(stats + f::ALL_TIME_RATING) as f64;
            if comp_stats_c3(all_time, minus_one) || season > all_time {
                write_u32(stats + f::ALL_TIME_RATING, read_u32(stats + comp_stats_obj::SEASON_RATING));
                exe::record_holder_assign(stats + f::ALL_TIME_RATING_HOLDER, stats + comp_stats_obj::SEASON_RATING_HOLDER);
                write_u8(stats + f::ALL_TIME_RATING_GAMES, read_u8(stats + comp_stats_obj::SEASON_RATING_GAMES));
                write_u32(stats + f::ALL_TIME_RATING_CLUB, read_u32(stats + comp_stats_obj::SEASON_RATING_CLUB));
                write_u16(stats + f::ALL_TIME_RATING_YEAR, read_u16(stats + comp_stats_obj::SEASON_RATING_EXTRA));
                write_u16(stats + f::ALL_TIME_RATING_YEAR, read_u16(read_usize(stats + comp_stats_obj::COMP) + league::YEAR));
                if read_i16(stats + f::SEASONS_ENDED) >= 3 && league_division() && read_u8(stats + f::ALL_TIME_RATING_GAMES) as i8 > 3 {
                    covered!("tel41_3 rating record news");
                    write_u32(stats + comp_stats_record_flag(COMP_STATS_KIND_RATING), 1);
                    write_u32(stats + f::ANY_RECORD_FLAG, 1);
                }
            }
        }
    }
    write_i16(stats + f::SEASONS_ENDED, read_i16(stats + f::SEASONS_ENDED).wrapping_add(1));
    1
}

/// `comp_season_records_construct` (0x49f160, thiscall, ret 0): the constructors of the sub-records of a season
/// record block (dates, runs, holders); writes only inside the block.
unsafe fn comp_season_records_construct(records: usize) {
    let f: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x49f160usize);
    f(records);
}

/// Rule: when a competition's season closes, its season records may be appended to the shared history file (the
/// records screen lists them later), then the season records are cleared, the runs kept for the next season and the
/// two tables freed.
///
/// `Comp_Stats::archive_season_and_reset(archive)`: with `archive` ≠ 0, a row of 0x26b bytes (on the stack, its
/// sub-records constructed first, `comp_season_records_construct`) takes the competition's year (word) and a copy of
/// the season records (+0x28f, field by field as `comp_season_records_assign`), and is appended to the shared file
/// list (`FILE_LLIST::add_node` of `COMP_STATS_ARCHIVE_LIST` with the two archive keys +0x4f8/+0x4fc, which it may
/// set). Then, always: `comp_season_records_clear` of the season records, `Comp_Stats::save_team_runs`, and the club
/// and player tables freed (`crt_free`; pointer, capacity and count 0, sort byte 0xfe). Returns 1.
/// Original: `Comp_Stats::archive_season_and_reset` (0x4a1c50, comp_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn comp_stats_archive_season_and_reset(stats: usize, archive: u32) -> u32 {
    let mut row = [0u8; SEASON_RECORDS_ROW];
    let r = row.as_mut_ptr() as usize;
    comp_season_records_construct(r + 2);
    if archive != 0 {
        covered!("tel41_3 season archived");
        write_u16(r, read_u16(read_usize(stats + comp_stats_obj::COMP) + league::YEAR));
        comp_season_records_assign(r + 2, stats + comp_stats_obj::SEASON_RECORDS);
        // tel41_3: the season row goes to the history file
        exe::file_llist_add_node(read_usize(COMP_STATS_ARCHIVE_LIST), stats + comp_stats_obj::ARCHIVE_KEY1,
            stats + comp_stats_obj::ARCHIVE_KEY2, row.as_ptr());
    }
    comp_season_records_clear(stats + comp_stats_obj::SEASON_RECORDS);
    comp_stats_save_team_runs(stats); // tel41_3: the runs kept for the next season
    if read_usize(stats + comp_stats_obj::CLUBS) != 0 {
        exe::crt_free(read_usize(stats + comp_stats_obj::CLUBS));
        write_usize(stats + comp_stats_obj::CLUBS, 0);
        write_u32(stats + comp_stats_obj::CLUB_CAPACITY, 0);
        write_u32(stats + comp_stats_obj::CLUB_COUNT, 0);
        write_u8(stats + comp_stats_obj::CLUB_SORT, 0xfe);
    }
    if read_usize(stats + comp_stats_obj::PLAYERS) != 0 {
        exe::crt_free(read_usize(stats + comp_stats_obj::PLAYERS));
        write_usize(stats + comp_stats_obj::PLAYERS, 0);
        write_u32(stats + comp_stats_obj::PLAYER_CAPACITY, 0);
        write_u32(stats + comp_stats_obj::PLAYER_COUNT, 0);
        write_u8(stats + comp_stats_obj::PLAYER_SORT, 0xfe);
    }
    1
}

/// The size of a news item on the stack (`NewsItem::NewsItem`, 0xe0 bytes).
const COMP_STATS_NEWS_ITEM_BYTES: usize = 0xe0;
/// The news type of a broken competition record.
const COMP_STATS_RECORD_NEWS_TYPE: u32 = 0x3e9;

/// Rule: each competition record the season broke becomes one news item for the competition's region; then the
/// flags are cleared.
///
/// `Comp_Stats::send_record_news()`: with no flag set ("any flag" dword +0x562 = 0): 0. Otherwise, for kinds 1 to 22
/// in order, each set flag (+0x50a + 4 × (kind − 1)): a news item of type 0x3e9 (`NewsItem::NewsItem` on the stack),
/// filled by `comp_stats_write_record_to_news(competition id, all-time records +0x26, kind, item)` and sent to the
/// competition's region (`News::send_to_comp_region` of `G_NEWS` with the competition's record); the flag is cleared.
/// The "any flag" dword is cleared. Returns 1.
/// Original: `Comp_Stats::send_record_news` (0x4a2450, comp_screens.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn comp_stats_send_record_news(stats: usize) -> u32 {
    if read_u32(stats + comp_stats_season_end::ANY_RECORD_FLAG) == 0 {
        return 0;
    }
    for kind in 1..=22usize {
        let flag = stats + comp_stats_record_flag(kind);
        if read_u32(flag) == 0 {
            continue;
        }
        covered!("tel41_3 record news sent");
        let mut item = [0u32; COMP_STATS_NEWS_ITEM_BYTES / 4];
        let n = item.as_mut_ptr() as usize;
        exe::new_newsitem(n, COMP_STATS_RECORD_NEWS_TYPE, 0);
        let record = read_usize(read_usize(stats + comp_stats_obj::COMP) + league::COMPETITION);
        // tel41_3: the record kind of the news
        comp_stats_write_record_to_news(read_u32(record), stats + comp_stats_obj::ALL_TIME_RECORDS, kind as u32, n);
        exe::news_send_to_comp_region(read_usize(G_NEWS), n, read_usize(read_usize(stats + comp_stats_obj::COMP) + league::COMPETITION));
        write_u32(flag, 0); // tel41_3: the flag cleared
    }
    write_u32(stats + comp_stats_season_end::ANY_RECORD_FLAG, 0);
    1
}

// ---------- comparers ----------

/// The size of the `Comp_Stats` object.
const COMP_STATS_OBJECT_SIZE: usize = 0x5ce;

/// Comparer for `comp_stats_update_season_end_records`: side by side over the whole object, both tables (the sorts),
/// the history table (the most titles sort) and the sort context; the tally block of the most titles is each side's
/// own and the friendly check only reads.
pub(crate) unsafe extern "thiscall" fn compare_comp_stats_update_season_end_records(stats: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    let regions = [
        (stats, COMP_STATS_OBJECT_SIZE),
        (read_usize(stats + comp_stats_obj::CLUBS), read_u32(stats + comp_stats_obj::CLUB_COUNT) as usize * comp_stats_obj::CLUB_ENTRY_SIZE),
        (read_usize(stats + comp_stats_obj::PLAYERS), read_u32(stats + comp_stats_obj::PLAYER_COUNT) as usize * comp_stats_obj::PLAYER_ENTRY_SIZE),
        (read_usize(G_CLUB_COMP_HISTORY_TABLE), read_u32(G_CLUB_COMP_HISTORY_COUNT) as usize * COMP_HISTORY_ROW_SIZE),
        (G_COMP_STATS_SORT_CONTEXT, 4),
    ];
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(stats), || comp_stats_update_season_end_records(stats));
    SCOREBOARD.record(equal && ro == rr, || format!("Comp_Stats::update_season_end_records({:#x}): differences in {:?}", stats, first_differences(&mo, &mr)));
    ro
}

/// Comparer for `comp_stats_archive_season_and_reset`: it appends to the history file and frees, so only the original
/// runs; the plan, made before, is checked after it: the season records as `comp_season_records_clear` leaves them
/// (cleared on a copy beforehand), the saved runs one per club entry (as `Comp_Stats::save_team_runs` leaves the count), both tables freed and 1 returned.
pub(crate) unsafe extern "thiscall" fn compare_comp_stats_archive_season_and_reset(stats: usize, archive: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(%%I1%%);
    let mut cleared = [0u8; comp_stats_obj::SEASON_RECORDS_SIZE];
    core::ptr::copy_nonoverlapping((stats + comp_stats_obj::SEASON_RECORDS) as *const u8, cleared.as_mut_ptr(), cleared.len());
    comp_season_records_clear(cleared.as_mut_ptr() as usize);
    let teams = read_u32(stats + comp_stats_obj::CLUB_COUNT);
    let clubs = if teams != 0 {
        teams
    } else if read_usize(stats + comp_stats_runs::SAVED) != 0 {
        0
    } else {
        read_u32(stats + comp_stats_runs::SAVED_COUNT)
    };
    let r = o(stats, archive);
    let records = core::slice::from_raw_parts((stats + comp_stats_obj::SEASON_RECORDS) as *const u8, cleared.len());
    let ok = r == 1 && records == &cleared[..] && read_u32(stats + comp_stats_runs::SAVED_COUNT) == clubs
        && read_usize(stats + comp_stats_obj::CLUBS) == 0 && read_usize(stats + comp_stats_obj::PLAYERS) == 0;
    SCOREBOARD.record(ok, || format!("Comp_Stats::archive_season_and_reset({:#x}, {}): out of plan (return {}, saved runs {} for {} clubs)", stats, archive, r, read_u32(stats + comp_stats_runs::SAVED_COUNT), clubs));
    r
}

/// Comparer for `comp_stats_send_record_news`: it sends news, so only the original runs; the plan is checked after it:
/// with a flag set it returns 1 and clears the 22 flags and the "any flag" dword; without, 0 and nothing changes.
pub(crate) unsafe extern "thiscall" fn compare_comp_stats_send_record_news(stats: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    let any = read_u32(stats + comp_stats_season_end::ANY_RECORD_FLAG) != 0;
    let mut before = [0u8; 0x5c];
    core::ptr::copy_nonoverlapping((stats + comp_stats_season_end::RECORD_FLAGS) as *const u8, before.as_mut_ptr(), before.len());
    let r = o(stats);
    let after = core::slice::from_raw_parts((stats + comp_stats_season_end::RECORD_FLAGS) as *const u8, before.len());
    let ok = if any { r == 1 && after.iter().all(|&b| b == 0) } else { r == 0 && after == &before[..] };
    SCOREBOARD.record(ok, || format!("Comp_Stats::send_record_news({:#x}): out of plan (return {}, flags {:x?})", stats, r, after));
    r
}
