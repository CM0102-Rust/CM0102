// ---------- tel41_1: the competition statistics: finding a club's and a player's entry and a club's statistic (comp_screens.cpp) ----------

/// The regions the two entry finders may rewrite: a table (it may be re-sorted by id), its sort byte and the sort
/// context.
unsafe fn comp_stats_find_regions(stats: usize, players: bool) -> [(usize, usize); 3] {
    if players {
        [(read_usize(stats + comp_stats_obj::PLAYERS), read_u32(stats + comp_stats_obj::PLAYER_COUNT) as usize * comp_stats_obj::PLAYER_ENTRY_SIZE),
         (stats + comp_stats_obj::PLAYER_SORT, 1), (G_COMP_STATS_SORT_CONTEXT, 4)]
    } else {
        [(read_usize(stats + comp_stats_obj::CLUBS), read_u32(stats + comp_stats_obj::CLUB_COUNT) as usize * comp_stats_obj::CLUB_ENTRY_SIZE),
         (stats + comp_stats_obj::CLUB_SORT, 1), (G_COMP_STATS_SORT_CONTEXT, 4)]
    }
}

/// `Comp_Stats` +0x566: the temporary club entry `get_club_projected_positions` lends to the stage's projection; while it
/// is set, `find_club_entry` returns it for any club.
const COMP_STATS_PROJECTED_ENTRY: usize = 0x566;

/// Rule: a club's entry in a competition's statistics is found by binary search on the club table sorted by club id
/// (re-sorted first when another statistic left it in another order).
///
/// `Comp_Stats::find_club_entry(club)`: while a projection lends its temporary entry (+0x566), that entry, whatever the
/// club. No club: the comp_stats.CPP 3297 box and 0. No club table: 0. Unless the table is already in id order (sort
/// byte 0xff), it is marked 0xff and sorted (`G_COMP_STATS_SORT_CONTEXT` = the stats, qsort of `CLUB_COUNT` entries
/// of 0x76 bytes with `comp_stats_compare_team_entries`). Then `bsearch` with a key entry holding the club's id (the
/// same order, the context set again). Returns the entry or 0. The original builds the key as a whole 0x76-byte entry
/// on the stack (its date and run constructors only touch the stack); the order reads only the id.
/// Original: `Comp_Stats::find_club_entry` (0x4a4850, comp_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn comp_stats_find_club_entry(stats: usize, club: usize) -> usize {
    let lent = read_usize(stats + COMP_STATS_PROJECTED_ENTRY);
    if lent != 0 {
        covered!("tel41_1 projected entry lent");
        return lent;
    }
    if club == 0 {
        show_error_box(COMP_STATS_CPP, 0xce1);
        return 0;
    }
    if read_usize(stats + comp_stats_obj::CLUBS) == 0 {
        return 0;
    }
    if read_u8(stats + comp_stats_obj::CLUB_SORT) != 0xff {
        write_u8(stats + comp_stats_obj::CLUB_SORT, 0xff);
        write_usize(G_COMP_STATS_SORT_CONTEXT, stats);
        exe::crt_qsort(read_usize(stats + comp_stats_obj::CLUBS), read_u32(stats + comp_stats_obj::CLUB_COUNT),
            comp_stats_obj::CLUB_ENTRY_SIZE as u32, COMPARE_TEAM_ENTRIES);
    }
    let mut key = [0u8; comp_stats_obj::CLUB_ENTRY_SIZE];
    write_u32(key.as_mut_ptr() as usize, read_u32(club)); // tel41_1: the key is the club id
    write_usize(G_COMP_STATS_SORT_CONTEXT, stats);
    exe::crt_bsearch(key.as_ptr() as usize, read_usize(stats + comp_stats_obj::CLUBS), read_u32(stats + comp_stats_obj::CLUB_COUNT),
        comp_stats_obj::CLUB_ENTRY_SIZE as u32, COMPARE_TEAM_ENTRIES)
}

/// Rule: a player's entry in a competition's statistics is found by binary search on the player table sorted by
/// player id (re-sorted first when another statistic left it in another order).
///
/// `Comp_Stats::find_player_entry(player)`: no player: the comp_stats.CPP 3342 box and 0. No player table: 0. Unless
/// the table is already in id order (sort byte 0xff), it is marked 0xff and sorted (context = the stats, qsort of
/// `PLAYER_COUNT` entries of 0x2d bytes with `comp_stats_compare_player_entries`). Then `bsearch` with the player's id
/// as the key (the context set again). Returns the entry or 0.
/// Original: `Comp_Stats::find_player_entry` (0x4a4a30, comp_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn comp_stats_find_player_entry(stats: usize, player: usize) -> usize {
    if player == 0 {
        show_error_box(COMP_STATS_CPP, 0xd0e);
        return 0;
    }
    if read_usize(stats + comp_stats_obj::PLAYERS) == 0 {
        return 0;
    }
    if read_u8(stats + comp_stats_obj::PLAYER_SORT) != 0xff {
        write_u8(stats + comp_stats_obj::PLAYER_SORT, 0xff);
        write_usize(G_COMP_STATS_SORT_CONTEXT, stats);
        exe::crt_qsort(read_usize(stats + comp_stats_obj::PLAYERS), read_u32(stats + comp_stats_obj::PLAYER_COUNT),
            comp_stats_obj::PLAYER_ENTRY_SIZE as u32, COMPARE_PLAYER_ENTRIES);
    }
    let key = read_u32(player); // tel41_1: the key is the player id
    write_usize(G_COMP_STATS_SORT_CONTEXT, stats);
    exe::crt_bsearch(&key as *const u32 as usize, read_usize(stats + comp_stats_obj::PLAYERS), read_u32(stats + comp_stats_obj::PLAYER_COUNT),
        comp_stats_obj::PLAYER_ENTRY_SIZE as u32, COMPARE_PLAYER_ENTRIES)
}

/// Rule: a club's statistic in a competition is the value of its entry; a club with no entry has 0.
///
/// `Comp_Stats::get_club_stat(club, stat)`: no club: the comp_stats.CPP 2274 box and 0.0. The entry
/// (`comp_stats_find_club_entry`); none: 0.0; otherwise `comp_stats_get_team_entry_value(entry, stat)`. The value
/// comes back in st0 (0.0 is the float constant `rdata::F32_0`).
/// Original: `Comp_Stats::get_club_stat` (0x4a2e10, comp_screens.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn comp_stats_get_club_stat(stats: usize, club: usize, stat: u32) -> f64 {
    if club == 0 {
        show_error_box(COMP_STATS_CPP, 0x8e2);
        return read_f32(rdata::F32_0) as f64;
    }
    let entry = comp_stats_find_club_entry(stats, club);
    if entry == 0 {
        covered!("tel41_1 club stat with no entry");
        return read_f32(rdata::F32_0) as f64;
    }
    exe::comp_stats_get_team_entry_value(stats, entry, stat) // tel41_1: the club statistic
}

// ---------- comparers ----------

/// Comparer for `comp_stats_find_club_entry`: side by side over the club table (it may be sorted), the sort byte, the
/// context and the entry returned.
pub(crate) unsafe extern "thiscall" fn compare_comp_stats_find_club_entry(stats: usize, club: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%);
    if club == 0 {
        return o(stats, club);
    }
    let regions = comp_stats_find_regions(stats, false);
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(stats, club), || comp_stats_find_club_entry(stats, club));
    SCOREBOARD.record(equal && ro == rr, || format!("Comp_Stats::find_club_entry({:#x}, {:#x}): entry {:#x} x {:#x}, differences in {:?}", stats, club, ro, rr, first_differences(&mo, &mr)));
    ro
}

/// Comparer for `comp_stats_find_player_entry`: side by side over the player table, the sort byte, the context and
/// the entry returned.
pub(crate) unsafe extern "thiscall" fn compare_comp_stats_find_player_entry(stats: usize, player: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I1%%);
    if player == 0 {
        return o(stats, player);
    }
    let regions = comp_stats_find_regions(stats, true);
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(stats, player), || comp_stats_find_player_entry(stats, player));
    SCOREBOARD.record(equal && ro == rr, || format!("Comp_Stats::find_player_entry({:#x}, {:#x}): entry {:#x} x {:#x}, differences in {:?}", stats, player, ro, rr, first_differences(&mo, &mr)));
    ro
}

/// Comparer for `comp_stats_get_club_stat`: side by side over the club table, its sort byte, the context and the
/// value (bit for bit).
pub(crate) unsafe extern "thiscall" fn compare_comp_stats_get_club_stat(stats: usize, club: usize, stat: u32) -> f64 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> f64>(%%I2%%);
    if club == 0 {
        return o(stats, club, stat);
    }
    let regions = comp_stats_find_regions(stats, false);
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(stats, club, stat), || comp_stats_get_club_stat(stats, club, stat));
    SCOREBOARD.record(equal && ro.to_bits() == rr.to_bits(), || format!("Comp_Stats::get_club_stat({:#x}, {:#x}, {}): {} x {}, differences in {:?}", stats, club, stat as u8, ro, rr, first_differences(&mo, &mr)));
    ro
}
