// ---------- batch %%LEVA%%: club records: which record types apply and the end of a nation's season (club_records.cpp) ----------

/// Who a record type applies to: club sides only.
const SCOPE_CLUBS: u8 = 0;
/// National teams only (type 27, the most caps).
const SCOPE_NATIONAL_TEAMS: u8 = 2;
/// Club sides, and only outside the all-time period (type 32, the fans' player of the year).
const SCOPE_CLUB_SEASONS: u8 = 3;
/// Everybody.
const SCOPE_ALL: u8 = 4;

/// The scope of record types 1 to 32 (the original's byte table at 0x4496a8, whose cases 0 and 1 both mean club sides).
const RECORD_TYPE_SCOPE: [u8; 32] = {
    const C: u8 = SCOPE_CLUBS;
    const A: u8 = SCOPE_ALL;
    [C, C, A, A, A, C, A, C, A, A, C, A, A, A, A, A, C, A, A, A, A, A, C, C, A, A, SCOPE_NATIONAL_TEAMS, C, C, C, C, SCOPE_CLUB_SEASONS]
};

/// Returns whether record type `kind` (1..32) applies to `club` in `period` (1 all-time, 2 the season).
///
/// `club_records_record_type_applies(kind, club, period)`: a null club gives the error box (club_records.cpp, line
/// 0x892) and 0. By the type's scope (`RECORD_TYPE_SCOPE`): club sides only (the league positions, the attendance,
/// the results, the league appearances, the transfers...): 1 unless the club is a national team
/// (`exe::database_is_national_team`); national teams only (27, the caps): 1 only for a national team; the fans'
/// player of the year (32): 0 for the all-time period (1), otherwise as club sides only; every other type, and a type
/// outside 1..32: 1. The type and the period are read as bytes.
/// Original: `club_records_record_type_applies` (0x449590, club_records.cpp, thiscall, ret 0xc; ecx is not read, so the
/// Rust callers reach it as stdcall).
pub(crate) unsafe extern "thiscall" fn club_records_record_type_applies(_this: usize, kind: u32, club: usize, period: u32) -> u32 {
    if club == 0 {
        show_error_at(record_news_fns::CLUB_RECORDS_CPP, 0x892);
        return 0;
    }
    let national = || exe::database_is_national_team(club) != 0;
    let applies = match RECORD_TYPE_SCOPE.get((kind as u8 as i8 as i32 - 1) as u32 as usize) {
        Some(&SCOPE_CLUBS) => {
            covered!("sx06_1 club-only record type");
            !national() // sx06: club sides only
        }
        Some(&SCOPE_NATIONAL_TEAMS) => national(),
        Some(&SCOPE_CLUB_SEASONS) => {
            covered!("sx06_1 club season record type");
            period as u8 != 1 && !national() // sx06: not all-time
        }
        _ => true,
    };
    applies as u32
}

/// Comparer for `club_records_record_type_applies`: the return, side by side (the routine only reads). A null club:
/// only the original (the error box).
pub(crate) unsafe extern "thiscall" fn compare_club_records_record_type_applies(this: usize, kind: u32, club: usize, period: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize, u32) -> u32>(%%I0%%)(this, kind, club, period);
    if club != 0 {
        let n = club_records_record_type_applies(this, kind, club, period);
        SCOREBOARD.record(r == n, || format!("club_records_record_type_applies(type {}, club {:#x}, period {}) original={} rust={}", kind as u8, club, period as u8, r, n));
    }
    r
}

/// The record types `club_records_end_season_for_nation` folds into the all-time records.
const RECORD_AVERAGE_ATTENDANCE: u8 = 6;
const RECORD_SEASON_RATING: u8 = 0x14;
const RECORD_FANS_PLAYER: u8 = 0x20;
/// A club's record set (the season and the all-time records).
const RECORD_SET_SIZE: usize = 0x38b;
/// One archived season in the history file: the season year (word) and the record set.
const SEASON_HISTORY_RECORD_SIZE: usize = 2 + RECORD_SET_SIZE;

/// What the comparer's dry run of `club_records_end_season_for_nation` keeps instead of doing it.
pub(crate) struct SeasonEndPlan {
    /// queue items that were there before the routine ran: only these count as already queued
    queued_before: i32,
    /// the record news it would queue (club id, record type)
    queued: Vec<(u32, u8)>,
    /// each season it would archive: the club's entry and the history record
    archived: Vec<(usize, [u8; SEASON_HISTORY_RECORD_SIZE])>,
}

/// Whether `club` belongs to the key nation `nation` (its row of `G_LEAGUE_TABLE`, `exe::key_nation_index_of_club`).
pub(crate) unsafe fn club_of_key_nation(club: Club, nation: usize) -> bool {
    G_LEAGUE_TABLE.wrapping_add((exe::key_nation_index_of_club(club) as i8 as isize * 72) as usize) == nation
}

/// Closes the season of every club of a key nation.
///
/// `Club_Records::end_season_for_nation(nation)`: a null key nation gives the original routine (the error box, line
/// 0x58f, and 0). For each club of the game (`club_at`, up to [`G_CLUB_COUNT`]) of the key nation
/// (`club_of_key_nation`), `end_club_season`. Returns 1.
/// Original: `Club_Records::end_season_for_nation` (0x447880, club_records.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn club_records_end_season_for_nation(this: usize, nation: usize) -> u32 {
    if nation == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I1%%)(this, nation); // the error box
    }
    let mut k = 0isize;
    while k < read_i32(G_CLUB_COUNT) as isize {
        let club = club_at(k);
        if club_of_key_nation(club, nation) {
            end_club_season(this, club, KeyNationRecord(nation), None);
        }
        k += 1;
    }
    1
}

/// One club's end of season in `club_records_end_season_for_nation`: folds the season into the all-time records, archives it and
/// starts the next one.
///
/// With the season record (`club_season_record`) and the all-time record (`club_all_time_record`, neither created), the
/// club's entry (`club_records_entry`) and its season year (`exe::key_nation_get_club_season_year`):
/// - the average attendance (type 6, when it applies, `exe::club_records_record_type_applies` period 1): a season
///   value (+0x77) above −1 that beats the all-time one (−1 = none) goes to the all-time record with the year (+0x7b),
///   and the record news is queued (`queue_season_end_news`);
/// - a season rating still at −1.0 (`club_record::SEASON_RATING`) is worked out (`update_season_rating_record`);
/// - the best average rating (type 20): a season rating above −1.0 that beats the all-time one (−1.0 = none) is copied
///   (`rating_record_assign`) with the year (`club_record::SEASON_RATING_YEAR`), and the news is queued;
/// - without a season record, only `close_club_season`;
/// - the fans' player of the year (type 32, period 2): when one is picked (`pick_fans_player_of_year`), the news item
///   0x3e8 for a club with a human manager (`fans_player_news`) and the supporters' record of the person
///   (`add_supporters_player_of_year_record` with the year and the key nation's `split_season_flag()`);
/// - the season is archived (`archive_season`), the season record reset (`reset_records`) and `close_club_season`.
///
/// Defect of the original: the attendance and rating checks read the all-time record before testing it for null (and
/// only then create it), so a club with a season record and no all-time record would crash it; the two are always
/// created together by the match records, so the creation never runs.
pub(crate) unsafe fn end_club_season(this: usize, club: Club, nation: KeyNationRecord, mut plan: Option<&mut SeasonEndPlan>) {
    let season = club_season_record(this, club.addr(), 0);
    let mut all_time = club_all_time_record(this, club.addr(), 0);
    let entry = ClubRecordEntry(club_records_entry(this, club.addr()));
    let year = exe::key_nation_get_club_season_year(club);
    if exe::club_records_record_type_applies(RECORD_AVERAGE_ATTENDANCE as u32, club, 1) != 0 && season != 0 {
        let average = read_i32(season + 0x77); // the average attendance record
        if average > -1 {
            let best = read_i32(all_time + 0x77);
            if best == -1 || average > best {
                if all_time == 0 {
                    all_time = club_all_time_record(this, club.addr(), 1);
                }
                write_i32(all_time + 0x77, average);
                write_u16(all_time + 0x7b, year); // its year
                queue_season_end_news(this, club, entry, RECORD_AVERAGE_ATTENDANCE, plan.as_deref_mut());
            }
        }
    }
    if season != 0 && read_f32(season + club_record::SEASON_RATING) as f64 == read_f64(rdata::F64_NEG_1) {
        update_season_rating_record(this, club.addr());
    }
    if exe::club_records_record_type_applies(RECORD_SEASON_RATING as u32, club, 1) != 0 && season != 0 {
        let rating = read_f32(season + club_record::SEASON_RATING) as f64;
        let best = read_f32(all_time + club_record::SEASON_RATING) as f64;
        if rating > read_f64(rdata::F64_NEG_1) && (best == read_f64(rdata::F64_NEG_1) || rating > best) {
            covered!("sx06_1 all-time rating");
            if all_time == 0 {
                all_time = club_all_time_record(this, club.addr(), 1);
            }
            rating_record_assign(all_time + club_record::SEASON_RATING, season + club_record::SEASON_RATING);
            write_u16(all_time + club_record::SEASON_RATING_YEAR, year); // sx06: the all-time rating's year
            queue_season_end_news(this, club, entry, RECORD_SEASON_RATING, plan.as_deref_mut());
        }
    }
    let fans = exe::club_records_record_type_applies(RECORD_FANS_PLAYER as u32, club, 2) != 0;
    if season == 0 {
        close_club_season(entry);
        return;
    }
    if fans && pick_fans_player_of_year(this, club.addr(), season) != 0 {
        if plan.is_none() {
            fans_player_news(club, season);
        }
        let person = staff_at(read_i32(season + club_record::FANS_PLAYER + record_holder::STAFF_ID) as isize);
        SUPPORTERS_PLAYER_OF_YEAR_DRY_RUN.store(plan.is_some(), Relaxed);
        add_supporters_player_of_year_record(read_usize(G_STAFF_RECORDS), person.addr(), club.addr(), year as u32, nation.split_season_flag() as u32);
        SUPPORTERS_PLAYER_OF_YEAR_DRY_RUN.store(false, Relaxed);
    }
    archive_season(this, entry, season, year, plan);
    reset_records(season);
    close_club_season(entry);
}

/// Queues the all-time record news of `kind` for the club (`club_records_queue_record_news`: a human club or a national
/// team, 5 seasons or more, not queued yet). In the comparer's dry run the item goes to the plan instead, checked
/// against the queue as it was before the routine and the plan.
pub(crate) unsafe fn queue_season_end_news(this: usize, club: Club, entry: ClubRecordEntry, kind: u8, plan: Option<&mut SeasonEndPlan>) {
    let Some(plan) = plan else {
        return club_records_queue_record_news(this, club.addr(), entry.addr(), kind);
    };
    if exe::human_manager_is_human_club(club, 0, 1, 0, 0) == 0 && exe::database_is_national_team(club) == 0 || entry.seasons() < 5 {
        return;
    }
    let id = read_u32(club.addr());
    let queue = read_usize(this + club_records_fields::NEWS_QUEUE);
    let queued = (0..plan.queued_before.max(0) as usize).any(|k| read_u32(queue + 6 * k) == id && read_u8(queue + 6 * k + record_news_item::RECORD_TYPE) == kind);
    if !queued && !plan.queued.contains(&(id, kind)) {
        plan.queued.push((id, kind));
    }
}

/// The news item 0x3e8 of the fans' player of the year (record type 32) for a club with a human manager
/// (`exe::news_club_has_human_manager`): filled from the season record (`exe::club_records_write_record_news_fields`)
/// and sent to the club's manager (`exe::news_send_to_club_manager`).
pub(crate) unsafe fn fans_player_news(club: Club, season: usize) {
    if exe::news_club_has_human_manager(read_usize(G_NEWS), club) == 0 {
        return;
    }
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let news = local.as_mut_ptr() as usize;
    exe::new_newsitem(news, 0x3e8, 0);
    exe::club_records_write_record_news_fields(read_usize(club.addr()), season, RECORD_FANS_PLAYER as u32, news);
    exe::news_send_to_club_manager(read_usize(G_NEWS), news, club);
}

/// Appends the season to the club's history file (`exe::file_llist_add_node` on the list at +0 of the records, with the
/// entry's first and last offsets at +8 and +0xc): the season year and the whole record set. In the comparer's dry run
/// the record goes to the plan.
pub(crate) unsafe fn archive_season(this: usize, entry: ClubRecordEntry, season: usize, year: u16, plan: Option<&mut SeasonEndPlan>) {
    covered!("sx06_1 season archived");
    let mut record = [0u8; SEASON_HISTORY_RECORD_SIZE];
    record[..2].copy_from_slice(&year.to_le_bytes()); // sx06: the archived season's year
    // ponytail: the original copies the set field by field (the match, run, holder, rating, age and transfer record
    // assigns), which together cover all 0x38b bytes: the same bytes as one copy
    core::ptr::copy_nonoverlapping(season as *const u8, record[2..].as_mut_ptr(), RECORD_SET_SIZE);
    match plan {
        Some(plan) => plan.archived.push((entry.addr(), record)),
        None => {
            exe::file_llist_add_node(read_usize(this), entry.addr() + 8, entry.addr() + 0xc, record.as_ptr());
        }
    }
}

/// Starts the club's next season in its entry: one more season counted (`club_record_entry::SEASONS`), the season's
/// games (`club_record_entry::SEASON_GAMES`) and byte +0x11 cleared, and the dwords +0x14, +0x18, +0x2c, +0x40 and +0x54
/// zeroed.
pub(crate) unsafe fn close_club_season(entry: ClubRecordEntry) {
    covered!("sx06_1 seasons counted");
    entry.set_seasons(entry.seasons().wrapping_add(1)); // sx06: one more season
    entry.set_season_games(0);
    write_u8(entry.addr() + 0x11, 0);
    for o in [0x14, 0x18, 0x2c, 0x40, 0x54] {
        write_u32(entry.addr() + o, 0); // the season's running counts
    }
}

/// The newest record in a club's history file (`exe::file_llist_get_nodes` from the entry's offsets), or none.
pub(crate) unsafe fn newest_season_history(this: usize, entry: usize) -> Option<Vec<u8>> {
    let nodes = exe::file_llist_get_nodes(read_usize(this), read_u32(entry + 8), read_u32(entry + 0xc));
    if nodes == 0 {
        return None;
    }
    write_usize(nodes + llist::CURSOR, read_usize(nodes + llist::TAIL));
    let d = exe::llist_get_prev(nodes);
    let newest = (d != 0).then(|| (0..SEASON_HISTORY_RECORD_SIZE).map(|i| read_u8(d + i)).collect());
    exe::llist_scalar_deleting_destructor(nodes, 1);
    newest
}

/// Comparer for `club_records_end_season_for_nation`: side by side over what stays in memory, with a plan for what does not.
///
/// The original runs first; the copy runs from the same state as a dry run (`SeasonEndPlan`): it queues no news, posts
/// no news item, writes no supporters' history (`SUPPORTERS_PLAYER_OF_YEAR_DRY_RUN`) and archives nothing. Compared:
/// for each club of the key nation, its entry (all but the history offsets +8..+0x10, which the archive moves), its
/// season and all-time records, its favoured staff (`club::FAV_STAFF1`) and the favoured clubs of its 50 squad members
/// (`preferences::STAFF_FAVOURITE_CLUBS1`), with the generator (`crate::rng::STATE_REGIONS`); the news items the
/// original queued against the plan; the newest record of each archived club's history file against the planned one;
/// and the return. A null key nation: only the original. The fans' player news item is not compared (its fields are,
/// by the comparer of `club_records_write_record_news_fields`).
pub(crate) unsafe extern "thiscall" fn compare_club_records_end_season_for_nation(this: usize, nation: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I1%%);
    if nation == 0 {
        return o(this, nation);
    }
    let clubs: Vec<Club> = (0..read_i32(G_CLUB_COUNT) as isize).map(|k| club_at(k)).filter(|&c| club_of_key_nation(c, nation)).collect();
    let mut regions = crate::rng::STATE_REGIONS.to_vec();
    for &club in &clubs {
        let entry = club_records_entry(this, club.addr());
        regions.push((entry, 8));
        regions.push((entry + 0x10, 0x6a - 0x10));
        for r in [club_season_record(this, club.addr(), 0), club_all_time_record(this, club.addr(), 0)] {
            if r != 0 {
                regions.push((r, RECORD_SET_SIZE));
            }
        }
        regions.push((club.addr() + club::FAV_STAFF1, 0x18));
        for k in 0..50 {
            let s = Staff(club.squad(k));
            if !s.is_null() && s.staff_preferences() != 0 {
                regions.push((s.staff_preferences() + preferences::STAFF_FAVOURITE_CLUBS1, 0x18));
            }
        }
    }
    let queued_before = read_i32(this + club_records_fields::NEWS_QUEUE_COUNT);
    let mut plan = SeasonEndPlan { queued_before, queued: Vec::new(), archived: Vec::new() };
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, nation), || {
        crate::rng::without_counting(|| {
            for &club in &clubs {
                end_club_season(this, club, KeyNationRecord(nation), Some(&mut plan));
            }
            1
        })
    });
    let queue = read_usize(this + club_records_fields::NEWS_QUEUE);
    let queued_orig: Vec<(u32, u8)> = (queued_before.max(0)..read_i32(this + club_records_fields::NEWS_QUEUE_COUNT))
        .map(|k| (read_u32(queue + 6 * k as usize), read_u8(queue + 6 * k as usize + record_news_item::RECORD_TYPE)))
        .collect();
    let wrong_history: Vec<String> = plan.archived.iter()
        .filter_map(|(entry, record)| match newest_season_history(this, *entry) {
            Some(got) if got[..] == record[..] => None,
            got => Some(format!("entry {:#x}: planned {:02x?} in the file {:02x?}", entry, &record[..], got)),
        })
        .collect();
    SCOREBOARD.record(equal && ro == rr && queued_orig == plan.queued && wrong_history.is_empty(), || {
        let first = mo.0.iter().zip(mr.0.iter()).find(|(a, b)| a != b).map(|(a, b)| {
            let off = (0..a.1.len()).find(|&i| a.1.get(i) != b.1.get(i)).unwrap_or(0);
            format!("{:#x}+{:#x}: {:02x?} x {:02x?}", a.0, off, a.1.get(off), b.1.get(off))
        });
        format!("club_records_end_season_for_nation(nation {:#x}) original={} rust={} memory {:?}, news original {:?} rust {:?}, history {}",
            nation, ro, rr, first, queued_orig, plan.queued, wrong_history.join("; "))
    });
    ro
}
