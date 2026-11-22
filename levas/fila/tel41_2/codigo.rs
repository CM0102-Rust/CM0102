// ---------- tel41_2: the competition statistics: a player's statistic and the sorted team and player tables (comp_screens.cpp) ----------

/// `Comp_Stats::find_player_entry` (0x4a4a30, thiscall(stats, player), ret 4), through its address.
unsafe fn comp_stats_player_entry_of(stats: usize, player: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4a30usize);
    f(stats, player)
}

/// Rule: a player's statistic in a competition is the value of his entry; a player with no entry has 0.
///
/// `Comp_Stats::get_player_stat(player, stat)`: no player: the comp_stats.CPP 2485 box and 0.0. The entry
/// (`Comp_Stats::find_player_entry`); none: 0.0; otherwise `Comp_Stats::get_player_entry_value(entry, stat)`. The value
/// comes back in st0.
/// Original: `Comp_Stats::get_player_stat` (0x4a31a0, comp_screens.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn comp_stats_get_player_stat(stats: usize, player: usize, stat: u32) -> f64 {
    if player == 0 {
        show_error_box(COMP_STATS_CPP, 0x9b5);
        return read_f32(rdata::F32_0) as f64;
    }
    let entry = comp_stats_player_entry_of(stats, player);
    if entry == 0 {
        covered!("tel41_2 player stat with no entry");
        return read_f32(rdata::F32_0) as f64;
    }
    exe::comp_stats_get_player_entry_value(stats, entry, stat) // tel41_2: the player statistic
}

/// Copies one club entry (0x76 bytes) as the original does it, field by field: the id, the two match counts, the
/// home attendance, the three 5-byte arrays at +0xa, the four runs (`run_record_assign`) and the bytes +0x69..+0x75.
unsafe fn comp_stats_copy_club_entry(dst: usize, src: usize) {
    write_u32(dst, read_u32(src));
    write_u8(dst + 4, read_u8(src + 4));
    write_u8(dst + 5, read_u8(src + 5));
    write_u32(dst + 6, read_u32(src + 6));
    for o in 0xa..0x19 {
        write_u8(dst + o, read_u8(src + o));
    }
    for o in [comp_stats_club::WINNING_RUN, comp_stats_club::LOSING_RUN, comp_stats_club::UNBEATEN_RUN, comp_stats_club::WINLESS_RUN] {
        run_record_assign(dst + o, src + o);
    }
    for o in 0x69..comp_stats_obj::CLUB_ENTRY_SIZE {
        write_u8(dst + o, read_u8(src + o));
    }
}

/// Rule: a competition's team table for one statistic lists the clubs best first, at most `max` (all when negative);
/// for every statistic but 3, 4 and 5 the clubs at the bottom with 0 are left out.
///
/// `Comp_Stats::get_sorted_team_table(stat, &count, max)`: no club table: count 0 and 0. `max` below 0 is the club
/// count. Unless already sorted by `stat`, the club table is sorted by it (sort byte = stat, context = the stats,
/// qsort with `comp_stats_compare_team_entries`). Count = the smaller of the club count and `max`; for statistics
/// other than 3, 4 and 5, while the count is positive and the last counted club's value
/// (`comp_stats_get_team_entry_value`) is 0.0, the count goes down. Count 0: 0. Otherwise a block of count × 0x76
/// bytes (`crt_malloc`; failing, the comp_stats.CPP 3436 box, count 0 and 0) takes the first entries, copied field by
/// field. Returns the block (the caller frees it).
/// Original: `Comp_Stats::get_sorted_team_table` (0x4a4b20, comp_screens.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn comp_stats_get_sorted_team_table(stats: usize, stat_arg: u32, count: *mut i32, max: i32) -> usize {
    if read_usize(stats + comp_stats_obj::CLUBS) == 0 {
        *count = 0;
        return 0;
    }
    let max = if max < 0 { read_i32(stats + comp_stats_obj::CLUB_COUNT) } else { max };
    let stat = stat_arg as u8;
    if read_u8(stats + comp_stats_obj::CLUB_SORT) != stat {
        write_u8(stats + comp_stats_obj::CLUB_SORT, stat);
        write_usize(G_COMP_STATS_SORT_CONTEXT, stats);
        exe::crt_qsort(read_usize(stats + comp_stats_obj::CLUBS), read_u32(stats + comp_stats_obj::CLUB_COUNT),
            comp_stats_obj::CLUB_ENTRY_SIZE as u32, COMPARE_TEAM_ENTRIES);
    }
    let n = read_i32(stats + comp_stats_obj::CLUB_COUNT);
    *count = if n < max { n } else { max };
    if stat != 4 && stat != 5 && stat != 3 {
        while *count > 0 {
            let last = read_usize(stats + comp_stats_obj::CLUBS) + (*count as usize - 1) * comp_stats_obj::CLUB_ENTRY_SIZE;
            // tel41_2: the trailing clubs with 0 leave the table
            if exe::comp_stats_get_team_entry_value(stats, last, stat_arg) != read_f32(rdata::F32_0) as f64 {
                break;
            }
            *count -= 1;
        }
    }
    if *count == 0 {
        return 0;
    }
    let block = exe::crt_malloc((*count as u32).wrapping_mul(comp_stats_obj::CLUB_ENTRY_SIZE as u32));
    if block == 0 {
        show_error_box(COMP_STATS_CPP, 0xd6c);
        *count = 0;
        return 0;
    }
    let mut i = 0;
    while i < *count {
        let k = i as usize * comp_stats_obj::CLUB_ENTRY_SIZE;
        comp_stats_copy_club_entry(block + k, read_usize(stats + comp_stats_obj::CLUBS) + k);
        i += 1;
    }
    block
}

/// Rule: a competition's player table for one statistic lists the players best first, at most `max` (all when
/// negative), only those of the position groups in `groups` (0xffff = all) and never one with 0.
///
/// `Comp_Stats::get_sorted_player_table(stat, &count, max, groups)`: no player table: count 0 and 0. `max` below 0 is
/// the player count. Unless already sorted by `stat`: with statistic 0x11 the club table is first put in id order
/// (sort byte 0xff, qsort with `comp_stats_compare_team_entries`) when it is not; then the player table is sorted by
/// `stat` (sort byte = stat, context = the stats, qsort with `comp_stats_compare_player_entries`). Count = the player
/// count; while it is positive and the last player's value (`Comp_Stats::get_player_entry_value`) is 0.0, it goes
/// down. Count 0: 0. Otherwise a block of count × 0x2d bytes (`crt_malloc`; failing, the comp_stats.CPP 3526 box,
/// count 0 and 0). The first `count` players are walked while fewer than `max` were taken: with `groups` other than
/// 0xffff a player whose position group (`comp_stats_get_player_position_group` of his record in
/// `G_PLAYER_DATA_TABLE`, by the id at +0) has no bit in `groups` is skipped, and so is one whose value is 0.0; the
/// others are copied whole (0x2d bytes). Count = how many were taken. Returns the block (the caller frees it).
/// Original: `Comp_Stats::get_sorted_player_table` (0x4a4dc0, comp_screens.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn comp_stats_get_sorted_player_table(stats: usize, stat_arg: u32, count: *mut i32, max: i32, groups: u32) -> usize {
    if read_usize(stats + comp_stats_obj::PLAYERS) == 0 {
        *count = 0;
        return 0;
    }
    let max = if max < 0 { read_i32(stats + comp_stats_obj::PLAYER_COUNT) } else { max };
    let stat = stat_arg as u8;
    if read_u8(stats + comp_stats_obj::PLAYER_SORT) != stat {
        if stat == 0x11 && read_u8(stats + comp_stats_obj::CLUB_SORT) != 0xff {
            write_u8(stats + comp_stats_obj::CLUB_SORT, 0xff);
            write_usize(G_COMP_STATS_SORT_CONTEXT, stats);
            exe::crt_qsort(read_usize(stats + comp_stats_obj::CLUBS), read_u32(stats + comp_stats_obj::CLUB_COUNT),
                comp_stats_obj::CLUB_ENTRY_SIZE as u32, COMPARE_TEAM_ENTRIES);
        }
        write_u8(stats + comp_stats_obj::PLAYER_SORT, stat);
        write_usize(G_COMP_STATS_SORT_CONTEXT, stats);
        exe::crt_qsort(read_usize(stats + comp_stats_obj::PLAYERS), read_u32(stats + comp_stats_obj::PLAYER_COUNT),
            comp_stats_obj::PLAYER_ENTRY_SIZE as u32, COMPARE_PLAYER_ENTRIES);
    }
    let zero = read_f32(rdata::F32_0) as f64;
    *count = read_i32(stats + comp_stats_obj::PLAYER_COUNT);
    while *count > 0 {
        let last = read_usize(stats + comp_stats_obj::PLAYERS) + (*count as usize - 1) * comp_stats_obj::PLAYER_ENTRY_SIZE;
        if exe::comp_stats_get_player_entry_value(stats, last, stat_arg) != zero {
            break;
        }
        *count -= 1;
    }
    if *count == 0 {
        return 0;
    }
    let block = exe::crt_malloc((*count as u32).wrapping_mul(comp_stats_obj::PLAYER_ENTRY_SIZE as u32));
    if block == 0 {
        show_error_box(COMP_STATS_CPP, 0xdc6);
        *count = 0;
        return 0;
    }
    let mut taken = 0i32;
    let mut i = 0i32;
    while i < *count {
        if taken >= max {
            break;
        }
        let entry = read_usize(stats + comp_stats_obj::PLAYERS) + i as usize * comp_stats_obj::PLAYER_ENTRY_SIZE;
        let wanted = groups as u16 == 0xffff || {
            let sheet = read_usize(G_PLAYER_DATA_TABLE) + (read_i32(entry) as usize).wrapping_mul(COMP_STATS_PLAYER_SHEET_SIZE);
            // tel41_2: the position group filter
            comp_stats_get_player_position_group(sheet) & groups as u16 != 0
        };
        if wanted && exe::comp_stats_get_player_entry_value(stats, entry, stat_arg) != zero {
            core::ptr::copy_nonoverlapping(entry as *const u8, (block + taken as usize * comp_stats_obj::PLAYER_ENTRY_SIZE) as *mut u8,
                comp_stats_obj::PLAYER_ENTRY_SIZE);
            taken += 1;
        }
        i += 1;
    }
    *count = taken;
    block
}

/// The player record of `G_PLAYER_DATA_TABLE` (0x46 bytes), indexed by the id at +0 of a player entry.
const COMP_STATS_PLAYER_SHEET_SIZE: usize = 0x46;

// ---------- comparers ----------

/// Comparer for `comp_stats_get_player_stat`: side by side over the player table (it may be sorted by id), its sort
/// byte, the context and the value (bit for bit).
pub(crate) unsafe extern "thiscall" fn compare_comp_stats_get_player_stat(stats: usize, player: usize, stat: u32) -> f64 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> f64>(%%I0%%);
    if player == 0 {
        return o(stats, player, stat);
    }
    let regions = [
        (read_usize(stats + comp_stats_obj::PLAYERS), read_u32(stats + comp_stats_obj::PLAYER_COUNT) as usize * comp_stats_obj::PLAYER_ENTRY_SIZE),
        (stats + comp_stats_obj::PLAYER_SORT, 1),
        (G_COMP_STATS_SORT_CONTEXT, 4),
    ];
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(stats, player, stat), || comp_stats_get_player_stat(stats, player, stat));
    SCOREBOARD.record(equal && ro.to_bits() == rr.to_bits(), || format!("Comp_Stats::get_player_stat({:#x}, {:#x}, {}): {} x {}, differences in {:?}", stats, player, stat as u8, ro, rr, first_differences(&mo, &mr)));
    ro
}

/// The regions a sorted table may rewrite: both tables (the sorts), both sort bytes and the context.
unsafe fn comp_stats_sorted_table_regions(stats: usize) -> [(usize, usize); 5] {
    [
        (read_usize(stats + comp_stats_obj::CLUBS), read_u32(stats + comp_stats_obj::CLUB_COUNT) as usize * comp_stats_obj::CLUB_ENTRY_SIZE),
        (read_usize(stats + comp_stats_obj::PLAYERS), read_u32(stats + comp_stats_obj::PLAYER_COUNT) as usize * comp_stats_obj::PLAYER_ENTRY_SIZE),
        (stats + comp_stats_obj::CLUB_SORT, 1),
        (stats + comp_stats_obj::PLAYER_SORT, 1),
        (G_COMP_STATS_SORT_CONTEXT, 4),
    ]
}

/// The two blocks (and counts) of a sorted table side by side: equal counts and bytes; the copy's block is freed.
unsafe fn comp_stats_sorted_blocks_equal(ro: usize, rr: usize, no: i32, nr: i32, size: usize) -> bool {
    let equal = no == nr && (ro == 0) == (rr == 0)
        && (ro == 0 || core::slice::from_raw_parts(ro as *const u8, no as usize * size) == core::slice::from_raw_parts(rr as *const u8, nr as usize * size));
    if rr != 0 {
        exe::crt_free(rr);
    }
    equal
}

/// Comparer for `comp_stats_get_sorted_team_table`: side by side over the tables, sort bytes and context (each side
/// allocates its own block); the two blocks and counts must be equal; the copy's block is freed.
pub(crate) unsafe extern "thiscall" fn compare_comp_stats_get_sorted_team_table(stats: usize, stat: u32, count: *mut i32, max: i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i32, i32) -> usize>(%%I1%%);
    let regions = comp_stats_sorted_table_regions(stats);
    let mut mine = 0i32;
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(stats, stat, count, max),
        || comp_stats_get_sorted_team_table(stats, stat, &mut mine, max));
    let blocks = comp_stats_sorted_blocks_equal(ro, rr, *count, mine, comp_stats_obj::CLUB_ENTRY_SIZE);
    SCOREBOARD.record(equal && blocks, || format!("Comp_Stats::get_sorted_team_table({:#x}, stat {}, max {}): count {} x {}, blocks equal {}, differences in {:?}", stats, stat as u8, max, *count, mine, blocks, first_differences(&mo, &mr)));
    ro
}

/// Comparer for `comp_stats_get_sorted_player_table`: as the team table's, over the player block.
pub(crate) unsafe extern "thiscall" fn compare_comp_stats_get_sorted_player_table(stats: usize, stat: u32, count: *mut i32, max: i32, groups: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i32, i32, u32) -> usize>(%%I2%%);
    let regions = comp_stats_sorted_table_regions(stats);
    let mut mine = 0i32;
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(stats, stat, count, max, groups),
        || comp_stats_get_sorted_player_table(stats, stat, &mut mine, max, groups));
    let blocks = comp_stats_sorted_blocks_equal(ro, rr, *count, mine, comp_stats_obj::PLAYER_ENTRY_SIZE);
    SCOREBOARD.record(equal && blocks, || format!("Comp_Stats::get_sorted_player_table({:#x}, stat {}, max {}, groups {:#x}): count {} x {}, blocks equal {}, differences in {:?}", stats, stat as u8, max, groups as u16, *count, mine, blocks, first_differences(&mo, &mr)));
    ro
}
