// ---------- batch %%LEVA%%: building the player statistics (player_stats.cpp) ----------

/// The CD check of the player statistics: among the drives (GetLogicalDriveStringsA, 200 bytes into `drives`) the first
/// CD drive (GetDriveTypeA 5) holding "cm0102.exe" (`exe::file_find_first` 0, then closed, `exe::file_find_close`);
/// none: the "Insert CD" box (langlib_sprintf of 0x987df4 and 0x987dc4, the box with 1) and again, forever. The buffer
/// keeps what the last GetLogicalDriveStringsA wrote: the original reuses those stack bytes as the language counters
/// of the history pass (`PlayerStatsHistoryPass::days`). cm0102-1080 has no CD requirement here: the jne on the drive
/// type (0x7bfdc4) became nops and the found test (0x7bfe06) became xor; the copy reads those bytes from the running exe.
/// Original: part of `PlayerStats::PlayerStats` (0x7bfd50, player_stats.cpp).
unsafe fn player_stats_check_cd(drives: &mut [u8; 0xc8]) {
    let drive_strings: unsafe extern "system" fn(u32, *mut u8) -> u32 = core::mem::transmute(read_usize(0x9670f8));
    let drive_type: unsafe extern "system" fn(usize) -> u32 = core::mem::transmute(read_usize(0x967100));
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let any_drive = read_u8(0x7bfdc4) == 0x90;
    let always_found = read_u8(0x7bfe06) != 0x85; // the original has `test eax, eax` (85 c0)
    loop {
        if drive_strings(0xc8, drives.as_mut_ptr()) != 0 && drives[0] != 0 {
            let mut i = 0usize;
            loop {
                let root = drives.as_ptr() as usize + i;
                if drive_type(root) == 5 || any_drive {
                    let mut name = [0u8; 0x104];
                    let mut found = [0u8; 0x140];
                    let nm = name.as_mut_ptr() as usize;
                    format(nm, 0x987e00, root, 0x63u32, 0x6du32, 0x30u32, 0x31u32, 0x30u32, 0x32u32, 0x2eu32, 0x65u32, 0x78u32, 0x65u32);
                    let f = found.as_mut_ptr() as usize;
                    if exe::file_find_first(nm, f, 0) == 0 || always_found {
                        return;
                    }
                    exe::file_find_close(f);
                }
                // simplification: the original walks with an i8 (steps of 4): it would only pass 127 with more than 32
                // drives, impossible
                i += 4;
                if i >= 0xc8 || drives[i] == 0 {
                    break;
                }
            }
        }
        let mut title = [0u8; 0x100];
        let mut msg = [0u8; 0x100];
        text(title.as_mut_ptr() as usize, 0x987df4); // "Insert CD"
        text(msg.as_mut_ptr() as usize, 0x987dc4); // "Please insert the CM01/02 CD and click on OK"
        exe::gui_show_message_box(title.as_ptr() as usize, msg.as_ptr() as usize, 1);
    }
}

/// The fluency a language weight or a days counter gives over the person's threshold: (w − threshold) / 50, at most 10
/// (16-bit values, signed).
fn player_stats_fluency_over(w: u16, threshold: u16) -> u32 {
    let v = (w as i16 as i32 - threshold as i16 as i32) / 50; // gra_27 the fluency divisor
    if v > 10 { 10 } else { v as u8 as u32 }
}

/// The person's language threshold: (20 − adaptability) × 10, in 16 bits.
unsafe fn player_stats_language_threshold(p: Staff) -> u16 {
    (20i16.wrapping_sub(p.adaptability() as i16)).wrapping_mul(10) as u16
}

/// Adds `fluency` to the person's language: with a language table, the record found by bsearch gains it (at most 10,
/// signed); without the table or the record, a new record with `sort` 1 (`exe::playerstats_add_language`).
unsafe fn player_stats_raise_language(this: usize, p: Staff, language: u8, fluency: u8) {
    if read_usize(this + player_stats_fields::LANGUAGES) != 0 {
        let r = find_language_record(this, p.addr(), language);
        if r != 0 {
            let f = read_u8(r + language_entry::FLUENCY).wrapping_add(fluency);
            write_u8(r + language_entry::FLUENCY, f);
            if (f as i8) > 10 {
                write_u8(r + language_entry::FLUENCY, 10);
            }
            return;
        }
    }
    exe::playerstats_add_language(this, p, language as u32, fluency as u32, 1);
}

/// The storage of a new game: the category records (100 of 0x20 bytes, malloc 0xc80), the per-player entries
/// (`G_PLAYER_DATA_COUNT` of 0x27 bytes: the eight dwords −1, the league games and goals and the ratings 0), the rating
/// averages (680.0 over 100.0 twice) and the empty language table. Out of memory: the fatal error box (lines 0x79, 0x85).
unsafe fn player_stats_init_storage(this: usize) {
    write_u32(this + player_stats_fields::CATEGORY_CAPACITY, 0x64);
    write_u32(this + player_stats_fields::CATEGORY_COUNT, 0);
    write_u32(this + player_stats_fields::FREE_CATEGORY_COUNT, 0);
    let c = exe::crt_malloc(0xc80);
    write_usize(this + player_stats_fields::CATEGORY_RECORDS, c);
    if c == 0 {
        show_player_stats_error(0x79);
        fatal_exit();
    }
    let n = read_i32(G_PLAYER_DATA_COUNT);
    write_i32(this + 0x34, n); // the number of per-player entries
    let e = exe::crt_malloc((n as u32).wrapping_mul(0x27));
    write_usize(this + player_stats_fields::PLAYER_ENTRIES, e);
    if e == 0 {
        show_player_stats_error(0x85);
        fatal_exit();
    }
    write_f64(this + player_stats_fields::RATING_SUM, 680.0);
    write_f64(this + player_stats_fields::RATED_COUNT, 100.0);
    write_f64(this + player_stats_fields::FLAGGED_RATING_SUM, 680.0);
    write_f64(this + player_stats_fields::FLAGGED_RATED_COUNT, 100.0);
    for k in 0..n.max(0) as usize {
        let r = e + k * 0x27;
        for j in 0..8 {
            write_u32(r + 4 * j, 0xffff_ffff);
        }
        write_u16(r + player_stats_entry::CLUB_LEAGUE_GAMES, 0);
        write_u16(r + player_stats_entry::CLUB_LEAGUE_GOALS, 0);
        write_u8(r + 0x26, 0);
        write_u8(r + 0x25, 0);
        write_u8(r + 0x24, 0);
    }
    write_u32(this + player_stats_fields::LANGUAGES, 0);
    write_u32(this + player_stats_fields::LANGUAGE_CAPACITY, 0);
    write_u32(this + player_stats_fields::LANGUAGE_COUNT, 0);
}

/// The languages every person starts with, and the last threshold computed (the original keeps it in a local the
/// history pass reads before setting it).
///
/// For each person: languages 1..64 by the weight of his two nations (`exe::player_stats_get_native_language_points`):
/// a weight that is not 0 and reaches the threshold (`player_stats_language_threshold`) adds the language with
/// `player_stats_fluency_over` (unsorted). A non-player (no `.player()`, with `.non_player()`) at a club whose nation is
/// not his own: the years at the club (low byte of the game's start year, `G_GAME_START_DATE_YEAR`, minus the low byte of
/// `.date_joined_club_year()`; negative or 0: nothing; at most 10) × adaptability × 0.05 (truncated) in the club nation's
/// first language (`player_stats_raise_language`). In the end the table is sorted (qsort with 0x7c6d40).
unsafe fn player_stats_initial_languages(this: usize) -> u16 {
    let mut threshold = 0u16;
    for i in 0..read_i32(G_STAFF_COUNT).max(0) as isize {
        let p = staff_at(i);
        threshold = player_stats_language_threshold(p);
        for k in 1..=0x40u8 {
            let w = exe::player_stats_get_native_language_points(p, k as i8) as u16;
            if w != 0 && (w as i16) >= (threshold as i16) {
                covered!("gra_27 native language");
                exe::playerstats_add_language(this, p, k as u32, player_stats_fluency_over(w, threshold), 0);
            }
        }
        if !p.player().is_null() || p.non_player().is_null() {
            continue;
        }
        let club = p.club_job();
        if club.is_null() {
            continue;
        }
        let nation = club.nation();
        if nation.is_null() || nation.addr() == p.nation().addr() {
            continue;
        }
        let years = read_u8(G_GAME_START_DATE_YEAR).wrapping_sub(p.date_joined_club_year() as u8) as i8;
        if years < 0 {
            continue;
        }
        let years = if years > 10 { 10 } else { years }; // gra_27 the years abroad
        if years == 0 {
            continue;
        }
        covered!("gra_27 language of the club abroad");
        let fluency = ftol((years as i32 * p.adaptability() as i32) as f64 * read_f64(rdata::F64_0_05)) as u8;
        player_stats_raise_language(this, p, nation.first_language() as u8, fluency);
    }
    exe::crt_qsort(read_usize(this + player_stats_fields::LANGUAGES), read_u32(this + player_stats_fields::LANGUAGE_COUNT), 6, 0x7c6d40);
    threshold
}

/// The state the history pass carries from record to record (registers and locals of the original).
struct PlayerStatsHistoryPass {
    /// The person of the previous record (0 none).
    previous: usize,
    /// The career list being built (`exe::new_llist`), 0 none.
    list: usize,
    /// The two nodes of the previous record (category 6 and category 2).
    nodes: [usize; 2],
    /// The previous record's year and the low word of its club id (only the year is compared).
    last_year: u32,
    /// The 0x27-byte entry of the current player.
    entry: usize,
    /// The language threshold of the person being counted.
    threshold: u16,
    /// Days per language 1..64 (index 0 unused): the original's stack buffer, which starts with the drive list left by
    /// the CD check and is only zeroed when a person with a player sheet begins.
    days: [u16; 0x41],
}

/// Writes the finished career list of the player to the history file (FILE_LLIST 0x58e9b0 on
/// `player_stats_fields::HISTORY_FILE`, the positions in the entry's `player_stats_entry::HISTORY_START` /
/// `player_stats_entry::HISTORY_LAST`) and destroys it.
unsafe fn player_stats_flush_history(this: usize, s: &mut PlayerStatsHistoryPass) {
    let write_list: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(0x58e9b0usize);
    write_list(read_usize(this + player_stats_fields::HISTORY_FILE), s.entry + player_stats_entry::HISTORY_START, s.entry + player_stats_entry::HISTORY_LAST, s.list);
    exe::drop_llist(s.list);
    exe::crt_operator_delete(s.list);
}

/// The days abroad of the person who has just finished (`s.previous`): for each language 1..64 while he has a nation,
/// at a club whose nation speaks it and his own nation does not, the counter becomes at least the days since he joined
/// that club (`exe::date_days_since` from `G_GAME_START_DATE`, asked twice when it wins, as in the original); each
/// counter that is not 0 and reaches his threshold raises the language by `player_stats_fluency_over`.
unsafe fn player_stats_languages_abroad(this: usize, s: &mut PlayerStatsHistoryPass) {
    let p = Staff(s.previous);
    for k in 1..=0x40u8 {
        if p.nation().is_null() {
            break;
        }
        let club = p.club_job();
        if !club.is_null() && read_u8(club.nation().addr() + nation::FIRST_LANGUAGE) == k && read_u8(p.nation().addr() + nation::FIRST_LANGUAGE) != k {
            let since = || exe::date_days_since(G_GAME_START_DATE, p.date_joined_club() as u32, p.date_joined_club_leap_year() as u32) as u16;
            let d = since();
            if !((s.days[k as usize] as i16) > (d as i16)) {
                s.days[k as usize] = since();
            }
        }
        let w = s.days[k as usize];
        if w != 0 && (w as i16) >= (s.threshold as i16) {
            covered!("gra_27 language from the career");
            player_stats_raise_language(this, p, k, player_stats_fluency_over(w, s.threshold) as u8);
        }
    }
}

/// One record of the history pass (17 bytes of staff_history.dat, already sorted by person and year), from 1900 up to
/// the year before the game's start (`G_GAME_START_DATE_YEAR`).
///
/// No person: the list of the previous player is written (only when there was one) and the previous person is
/// forgotten (the two nodes are not: the original keeps their pointers into the freed list). A person without a player
/// sheet only becomes the previous person. A new person: the list of the previous player is written; the days abroad
/// of the previous person (`player_stats_languages_abroad`); the counters are zeroed, the threshold is the new one's
/// and the entry is his player's. Then the record: a 0x2b-byte node (year, club id or −2, games, goals, fee −1, mark
/// 0x30) whose previous pair of nodes gets the loan bit of the record (+0xe: mark & 0xf9 | 1; otherwise & 0xf8); a
/// year earlier than the previous record's is the error box (line 0x1e0); the node goes into the list twice (category
/// 6, mark 0x30, and category 2, mark 0x10; `exe::llist_add_tail`). A club whose nation is neither of the person's two
/// and whose language is 1..64 adds 2 × (3 × (games + 30) − today's year + the year) days to that language, and for
/// the person's current club at least (today's year − the year he joined) × 365 − the day he joined + today's day, ×
/// 0.7; the current club also adds the games and goals into the entry. In `plan` the list (only marked as there), the
/// nodes and the box are skipped.
unsafe fn player_stats_history_record(this: usize, rec: usize, s: &mut PlayerStatsHistoryPass, plan: bool) {
    let year = read_u16(rec + staff_history::YEAR);
    if (year as i16) < 0x76c || (year as i16 as i32) >= read_u16(G_GAME_START_DATE_YEAR) as i32 {
        return;
    }
    let person = read_usize(rec + staff_history::STAFF_ID);
    if person == 0 {
        if s.previous != 0 && s.list != 0 {
            if !plan {
                player_stats_flush_history(this, s);
            }
            s.list = 0;
            s.last_year = 0;
            s.previous = 0;
        }
        return;
    }
    let p = Staff(person);
    if p.player().is_null() {
        s.previous = person;
        return;
    }
    if person != s.previous {
        if s.list != 0 {
            if !plan {
                player_stats_flush_history(this, s);
            }
            s.nodes = [0, 0];
            s.list = 0;
            s.last_year = 0;
        }
        s.entry = read_usize(this + player_stats_fields::PLAYER_ENTRIES).wrapping_add((read_i32(p.player().addr()) as usize).wrapping_mul(0x27));
        if s.previous != 0 {
            player_stats_languages_abroad(this, s);
        }
        s.days = [0; 0x41];
        s.threshold = player_stats_language_threshold(p);
    }
    let games = read_u8(rec + 0xf);
    let goals = read_u8(rec + 0x10);
    let mut node = [0u8; 0x2b];
    node[0..2].copy_from_slice(&year.to_le_bytes());
    let club = read_usize(rec + staff_history::CLUB);
    let id = if club != 0 { read_u32(club) } else { 0xffff_fffe };
    node[2..6].copy_from_slice(&id.to_le_bytes());
    node[0x14..0x16].copy_from_slice(&0xffffu16.to_le_bytes());
    node[0x2a] = 0x30;
    node[6] = games;
    node[8] = goals;
    node[0x26..0x2a].copy_from_slice(&0xffff_ffffu32.to_le_bytes());
    if !plan && s.nodes[0] != 0 && s.nodes[1] != 0 {
        for &n in &s.nodes {
            let b = read_u8(n + 0x2a);
            write_u8(n + 0x2a, if read_u8(rec + 0xe) != 0 { (b & 0xf9) | 1 } else { b & 0xf8 });
        }
    }
    if !plan && year < s.last_year as u16 {
        show_player_stats_error(0x1e0);
    }
    s.last_year = u32::from_le_bytes([node[0], node[1], node[2], node[3]]);
    if plan {
        s.list = 1; // the plan keeps no list, only that there is one (it decides the flushes and the forgetting)
    } else {
        if s.list == 0 {
            let l = exe::crt_operator_new(0xe);
            s.list = if l != 0 { exe::new_llist(l) } else { 0 };
        }
        node[0x2a] = (node[0x2a] & 7) | 0x30;
        s.nodes[0] = exe::llist_add_tail(s.list, node.as_ptr() as usize, 0x2b);
        node[0x2a] = (node[0x2a] & 7) | 0x10;
        s.nodes[1] = exe::llist_add_tail(s.list, node.as_ptr() as usize, 0x2b);
    }
    let own_club = p.club_job();
    if (id as i32) > -1 {
        let n = club_at(id as i32 as isize).nation();
        let second = p.second_nation();
        if !n.is_null() && n.addr() != p.nation().addr() && (second.is_null() || n.addr() != second.addr()) {
            let language = n.first_language();
            if language > 0 && language <= 0x40 {
                covered!("gra_27 days abroad from the history");
                let slot = &mut s.days[language as usize];
                let add = (3u32.wrapping_mul(games as u32 + 0x1e)).wrapping_sub(read_u16(G_CURRENT_DATE_YEAR) as u32).wrapping_add(year as u32).wrapping_mul(2);
                *slot = slot.wrapping_add(add as u16);
                if !own_club.is_null() && read_u32(own_club.addr()) == id {
                    let years = (read_u16(G_CURRENT_DATE_YEAR) as i32).wrapping_sub(p.date_joined_club_year() as i32);
                    let days = years.wrapping_mul(365).wrapping_sub(p.date_joined_club() as u16 as i16 as i32).wrapping_add(read_i16(G_CURRENT_DATE) as i32);
                    let scaled = days as f64 * read_f64(rdata::F64_0_7); // gra_27 the days at the current club
                    let have = *slot as i16 as f64;
                    *slot = ftol(if scaled > have { scaled } else { have }) as u16;
                }
            }
        }
    }
    if !own_club.is_null() && read_u32(own_club.addr()) == id {
        covered!("gra_27 games at the current club");
        let g = s.entry + player_stats_entry::CLUB_LEAGUE_GAMES;
        write_u16(g, read_u16(g).wrapping_add(games as u16));
        let a = s.entry + player_stats_entry::CLUB_LEAGUE_GOALS;
        write_u16(a, read_u16(a).wrapping_add(goals as u16));
    }
    s.previous = person;
}

/// The progress bar of the history passes: every 500 records, "Player histories" at `done` of `total`
/// (`exe::screenmanager_send_network_status_message`).
unsafe fn player_stats_history_progress(i: i32, done: i32, total: i32, label: &[u8; 0x100]) {
    if i % 500 == 0 {
        exe::screenmanager_send_network_status_message(G_SCREEN_MANAGER, label.as_ptr() as usize, done, total as u32, -1);
    }
}

/// The real player histories of a new game (staff_history.dat, `G_STAFF_HISTORY_TABLE`, `G_STAFF_HISTORY_COUNT` records
/// of 17 bytes), turned into career lists, games and goals at the current club, club career records and languages.
///
/// Only without "norealplhist" on the command line, with byte 0xb74348 clear and byte 0x9b5981 set; without the table:
/// the error box (line 0xef) and nothing else. (1) Sorted by club and person (qsort with 0x7c6d90); goals without games
/// become 0; each run of records of the same person at the same club (with a player sheet and a club) sums games and
/// goals and goes to the club's career records (0x448cb0 on `G_CLUB_RECORDS`, only the words are read); with
/// "checkgks" on the command line a goalkeeper with 6 goals or more in a record is the error box (line 0x139). (2)
/// Sorted by person and year (qsort with 0x7c6de0); a record of the same person and year as the next one, at another
/// club, when the record after that has the first club again (and the same person, another year) swaps with the next
/// one. (3) The pass of `player_stats_history_record` over every record, then the last list is written. (4) The table
/// is freed (0x527ee0). `table` is the table to work on (the game's, or the comparer's copy in `plan`, which skips the
/// progress bar, the career records, the boxes, the lists and the freeing).
unsafe fn player_stats_real_histories(this: usize, table: usize, threshold: u16, drives: &[u8; 0xc8], plan: bool) {
    if exe::crt_strstr(0xae2c98, 0xa78300) != 0 || read_u8(0xb74348) != 0 || read_u8(0x9b5981) == 0 {
        return;
    }
    if read_usize(G_STAFF_HISTORY_TABLE) == 0 {
        if !plan {
            show_player_stats_error(0xef);
        }
        return;
    }
    let mut label = [0u8; 0x100];
    if !plan {
        let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
        text(label.as_mut_ptr() as usize, 0x9b5084); // "Player histories"
    }
    let count = || read_i32(G_STAFF_HISTORY_COUNT);
    let rec = |i: i32| table.wrapping_add((i as usize).wrapping_mul(0x11));
    exe::crt_qsort(table, count() as u32, 0x11, 0x7c6d90);
    let (mut person, mut club, mut games, mut goals) = (0usize, 0usize, 0u16, 0u16);
    let update_records: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(0x448cb0usize);
    let mut i = 0;
    while i <= count() - 1 {
        if !plan {
            player_stats_history_progress(i, i + 1, count().wrapping_mul(2), &label);
        }
        let r = rec(i);
        if read_u8(r + 0x10) != 0 && read_u8(r + 0xf) == 0 {
            write_u8(r + 0x10, 0);
        }
        let p = read_usize(r + staff_history::STAFF_ID);
        if p != 0 && !Staff(p).player().is_null() && read_usize(r + staff_history::CLUB) != 0 {
            let c = read_usize(r + staff_history::CLUB);
            if p != person || c != club {
                if person != 0 && club != 0 && !plan {
                    covered!("gra_27 career records of a club");
                    update_records(read_usize(G_CLUB_RECORDS), person, club, games as u32, goals as u32);
                }
                club = c;
                person = p;
                games = 0;
                goals = 0;
            }
            if !plan && exe::crt_strstr(0xae2c98, 0xa782f4) != 0 {
                let sheet = Staff(p).player();
                if !sheet.is_null() && exe::formation_get_position_mask(sheet) == 1 && read_u8(r + 0x10) >= 6 {
                    player_stats_goalkeeper_goals_box(p);
                }
            }
            games = games.wrapping_add(read_u8(r + 0xf) as u16);
            goals = goals.wrapping_add(read_u8(r + 0x10) as u16);
        }
        i += 1;
    }
    if person != 0 && club != 0 && !plan {
        update_records(read_usize(G_CLUB_RECORDS), person, club, games as u32, goals as u32);
    }
    exe::crt_qsort(table, count() as u32, 0x11, 0x7c6de0);
    let mut j = 1;
    while count() - 2 > 1 && j < count() - 2 {
        let (a, b, c) = (rec(j - 1), rec(j), rec(j + 1));
        let pa = read_u32(a + staff_history::STAFF_ID);
        let ya = read_u16(a + staff_history::YEAR);
        let ca = read_u32(a + staff_history::CLUB);
        if pa == read_u32(b + staff_history::STAFF_ID)
            && ya != read_u16(c + staff_history::YEAR)
            && ya == read_u16(b + staff_history::YEAR)
            && ca != read_u32(b + staff_history::CLUB)
            && ca == read_u32(c + staff_history::CLUB)
            && pa == read_u32(c + staff_history::STAFF_ID)
        {
            covered!("gra_27 season order swapped");
            let mut t = [0u8; 0x11];
            core::ptr::copy_nonoverlapping(a as *const u8, t.as_mut_ptr(), 0x11);
            core::ptr::copy_nonoverlapping(b as *const u8, a as *mut u8, 0x11);
            core::ptr::copy_nonoverlapping(t.as_ptr(), b as *mut u8, 0x11);
        }
        j += 1;
    }
    let mut s = PlayerStatsHistoryPass { previous: 0, list: 0, nodes: [0, 0], last_year: 0, entry: 0, threshold, days: [0; 0x41] };
    for k in 1..0x41usize {
        s.days[k] = u16::from_le_bytes([drives[2 * k - 2], drives[2 * k - 1]]);
    }
    let mut i = 0;
    while i < count() {
        if !plan {
            player_stats_history_progress(i, count() + i + 1, count().wrapping_mul(2), &label);
        }
        player_stats_history_record(this, rec(i), &mut s, plan);
        i += 1;
    }
    if s.list != 0 && !plan {
        player_stats_flush_history(this, &mut s);
    }
    if !plan {
        let free_history: unsafe extern "C" fn() = core::mem::transmute(0x527ee0usize);
        free_history();
    }
}

/// The "checkgks" box: "Too many goals for goalkeeper - " with the person's name (`exe::database_format_staff_name`,
/// 2000 bytes, mode 1) and the text at 0x99cae4, in the buffer at 0xde1f64, then the error box of line 0x139.
unsafe fn player_stats_goalkeeper_goals_box(person: usize) {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    format(0xde1f64, 0xa782d0);
    let len = |a: usize| (0..).take_while(|&k| read_u8(a + k) != 0).count();
    exe::database_format_staff_name(person, 0xde1f64 + len(0xde1f64), 0x7d0, 1);
    let end = 0xde1f64 + len(0xde1f64);
    let n = len(0x99cae4);
    core::ptr::copy_nonoverlapping(0x99cae4 as *const u8, end as *mut u8, n + 1);
    show_player_stats_error(0x139);
}

/// Builds the player statistics of a new game, or loads them from the saved game.
///
/// `PlayerStats::PlayerStats(path)`, the subsystem load (`setup.rs`, LoadStep 0x11): the CD check
/// (`player_stats_check_cd`); the history file (operator new of 8 bytes, FILE_LLIST 0x58e280 on "CM3_TEMP\player stats
/// history.tmp" with 0x2b-byte nodes, into `player_stats_fields::HISTORY_FILE`; failing: the fatal error box of line
/// 0x5e). With `path`, the statistics come from the saved game (0x7c48b0); when it answers, nothing else. Otherwise:
/// the storage (`player_stats_init_storage`), the languages every person starts with
/// (`player_stats_initial_languages`) and the real player histories (`player_stats_real_histories`). Returns `this`.
/// Original: `PlayerStats::PlayerStats` (0x7bfd50, player_stats.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn construct_player_stats(this: usize, path: usize) -> usize {
    let mut drives = [0u8; 0xc8];
    player_stats_check_cd(&mut drives);
    let file = exe::crt_operator_new(8);
    let file = if file != 0 {
        let construct: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x58e280usize);
        construct(file, exe::utils_build_file_path(0x987da0, 0xa78340), 0x2b)
    } else {
        0
    };
    write_usize(this + player_stats_fields::HISTORY_FILE, file);
    if file == 0 {
        show_player_stats_error(0x5e);
        fatal_exit();
    }
    if path != 0 {
        let load: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7c48b0usize);
        if load(this, path) != 0 {
            return this;
        }
    }
    player_stats_init_storage(this);
    let threshold = player_stats_initial_languages(this);
    player_stats_real_histories(this, read_usize(G_STAFF_HISTORY_TABLE), threshold, &drives, false);
    this
}

/// Comparer of `construct_player_stats` (comparison mode: see the header of motor.rs).
///
/// With a saved game only the original runs. In a new game the copy makes the plan first in a scratch object, with
/// the drive list of the moment and a copy of the history table (`plan`: no CD loop, no history file, no lists, no
/// career records, no boxes, the table not freed); then the original builds the real object (it sorts and frees the
/// game's table). Compared: the return, the capacities and counts, the four rating doubles, every per-player entry but
/// the two history positions, and the whole language table. The plan's arrays are freed.
pub(crate) unsafe extern "thiscall" fn compare_construct_player_stats(this: usize, path: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%);
    if path != 0 {
        return o(this, path);
    }
    let drive_strings: unsafe extern "system" fn(u32, *mut u8) -> u32 = core::mem::transmute(read_usize(0x9670f8));
    let mut drives = [0u8; 0xc8];
    drive_strings(0xc8, drives.as_mut_ptr());
    let mut plan = [0u8; 0x48];
    let po = plan.as_mut_ptr() as usize;
    let table = read_usize(G_STAFF_HISTORY_TABLE);
    let mut copy: Vec<u8> = Vec::new();
    if table != 0 {
        let n = (read_i32(G_STAFF_HISTORY_COUNT).max(0) as usize) * 0x11;
        copy.extend_from_slice(core::slice::from_raw_parts(table as *const u8, n));
    }
    player_stats_init_storage(po);
    let threshold = player_stats_initial_languages(po);
    player_stats_real_histories(po, copy.as_mut_ptr() as usize, threshold, &drives, true);
    let r = o(this, path);
    let mut wrong: Vec<String> = Vec::new();
    for off in [0x8usize, 0xc, 0x34, 0x38, 0x40, 0x44] {
        if read_u32(this + off) != read_u32(po + off) {
            wrong.push(format!("+{:#x} {} x {}", off, read_u32(this + off), read_u32(po + off)));
        }
    }
    for off in (0x10usize..0x30).step_by(8) {
        if read_u64(this + off) != read_u64(po + off) {
            wrong.push(format!("double +{:#x}", off));
        }
    }
    if read_u32(this + 0x34) == read_u32(po + 0x34) {
        let (eo, ep) = (read_usize(this + player_stats_fields::PLAYER_ENTRIES), read_usize(po + player_stats_fields::PLAYER_ENTRIES));
        for k in 0..read_i32(po + 0x34).max(0) as usize {
            let (a, b) = (eo + k * 0x27, ep + k * 0x27);
            let same = (0..0x18).chain(0x20..0x27).all(|j| read_u8(a + j) == read_u8(b + j));
            if !same && wrong.len() < 8 {
                wrong.push(format!("entry {} games {}/{} goals {}/{}", k, read_u16(a + 0x20), read_u16(b + 0x20), read_u16(a + 0x22), read_u16(b + 0x22)));
            }
        }
    }
    if read_u32(this + player_stats_fields::LANGUAGE_COUNT) == read_u32(po + player_stats_fields::LANGUAGE_COUNT) {
        let n = read_u32(po + player_stats_fields::LANGUAGE_COUNT) as usize * 6;
        let (lo, lp) = (read_usize(this + player_stats_fields::LANGUAGES), read_usize(po + player_stats_fields::LANGUAGES));
        if let Some(k) = (0..n).find(|&j| read_u8(lo + j) != read_u8(lp + j)) {
            wrong.push(format!("language record {} (person {}, language {}, fluency {} x {})", k / 6, read_u32(lo + k / 6 * 6), read_u8(lo + k / 6 * 6 + 4), read_u8(lo + k / 6 * 6 + 5), read_u8(lp + k / 6 * 6 + 5)));
        }
    }
    for off in [player_stats_fields::CATEGORY_RECORDS, player_stats_fields::PLAYER_ENTRIES, player_stats_fields::LANGUAGES] {
        if read_usize(po + off) != 0 {
            exe::crt_free(read_usize(po + off));
        }
    }
    SCOREBOARD.record(r == this && wrong.is_empty(), || format!("construct_player_stats({:#x}) {:?}", this, wrong));
    r
}
