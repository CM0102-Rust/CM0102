// ---------- batch %%LEVA%%: the squad list comparators (3) and the staff, attribute and date orders (club_screens.cpp) ----------

/// Squad list order "contract expiry": the players by the contract the list shows (`player_sort_list_contract`):
/// expired, then rolling, then by the days to its end, then by name.
///
/// A null first entry (or one without that contract) sorts last (1), a null second one first (-1). An expired
/// second contract (`STAFF_CONTRACT::is_expired`) gives 1, an expired first -1; then a rolling second
/// (`STAFF_CONTRACT::is_rolling`) 1, a rolling first -1; then the days to the end (`days_to_end_date`, signed words)
/// first - second; equal days go to the names (`player_sort_by_name`).
/// Original: `compare_players_by_contract_expiry_then_name` (0x486c70, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_players_by_contract_expiry_then_name(pa: *const usize, pb: *const usize) -> i32 {
    let (a, b) = player_sort_pair(pa, pb);
    if a == 0 {
        return 1;
    }
    if b == 0 {
        return -1;
    }
    let ca = player_sort_list_contract(a);
    let cb = player_sort_list_contract(b);
    if ca == 0 {
        return 1;
    }
    if cb == 0 {
        return -1;
    }
    let expired: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x85f9f0usize);
    let rolling: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x85fa20usize);
    if expired(cb) != 0 {
        return 1;
    }
    if expired(ca) != 0 {
        return -1;
    }
    if rolling(cb) != 0 {
        return 1;
    }
    if rolling(ca) != 0 {
        return -1;
    }
    let db = exe::staff_contract_days_to_end_date(cb) as i32;
    let da = exe::staff_contract_days_to_end_date(ca) as i32;
    if da - db != 0 {
        return da - db;
    }
    let (a, b) = player_sort_pair(pa, pb);
    player_sort_by_name(a, b)
}

/// Squad list order "FIFA protected period": the players by the byte their contract returns from
/// `STAFF_CONTRACT::check_fifa_protected_period(0, 0, 0)` (0 without a contract), then by name.
///
/// A null first entry sorts last (1), a null second one first (-1); then first - second (signed bytes); equal
/// values go to the names.
/// Original: `compare_players_by_fifa_protected_period_then_name` (0x486e60, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_players_by_fifa_protected_period_then_name(pa: *const usize, pb: *const usize) -> i32 {
    let (a, b) = player_sort_pair(pa, pb);
    if a == 0 {
        return 1;
    }
    if b == 0 {
        return -1;
    }
    let period = |p: usize| {
        let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), p);
        if c != 0 { exe::staff_contract_check_fifa_protected_period(c, 0usize, 0usize, 0usize) as i8 as i32 } else { 0 }
    };
    let va = period(a);
    let d = va - period(b);
    if d != 0 {
        return d;
    }
    let (a, b) = player_sort_pair(pa, pb);
    player_sort_by_name(a, b)
}

/// Squad list order "value": the players by `staff::VALUE`, highest first, then by name.
///
/// A null first entry sorts last (1), a null second one first (-1); then second - first; equal values go to the
/// names.
/// Original: `compare_players_by_value_then_name` (0x486f80, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_players_by_value_then_name(pa: *const usize, pb: *const usize) -> i32 {
    let (a, b) = player_sort_pair(pa, pb);
    if a == 0 {
        return 1;
    }
    if b == 0 {
        return -1;
    }
    let d = read_i32(b + staff::VALUE).wrapping_sub(read_i32(a + staff::VALUE));
    if d != 0 {
        return d;
    }
    player_sort_by_name(a, b)
}

/// The season statistic orders: `PlayerStats::get_stat(player record, 6, field, 6.5)` of the second player stored
/// as a float, minus the first's, truncated (`_ftol`) and narrowed to a signed word; zero goes to the names.
unsafe fn player_sort_by_season_stat(pa: *const usize, pb: *const usize, field: u32) -> i32 {
    let (a, b) = player_sort_pair(pa, pb);
    if a == 0 {
        return 1;
    }
    if b == 0 {
        return -1;
    }
    let stats = read_usize(G_PLAYER_STATS);
    let sb = exe::playerstats_get_stat(stats, read_usize(b + staff::PLAYER), 6, field, 6.5) as f32 as f64;
    let sa = exe::playerstats_get_stat(stats, read_usize(a + staff::PLAYER), 6, field, 6.5);
    let d = ftol(sb - sa) as i16;
    if d != 0 {
        return d as i32;
    }
    let (a, b) = player_sort_pair(pa, pb);
    player_sort_by_name(a, b)
}

/// Squad list order "appearances": the players by the season statistic 1 (`player_sort_by_season_stat`), most
/// first, then by name; a null first entry sorts last (1), a null second one first (-1).
/// Original: `compare_players_by_appearances_then_name` (0x487030, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_players_by_appearances_then_name(pa: *const usize, pb: *const usize) -> i32 {
    player_sort_by_season_stat(pa, pb, 1)
}

/// Squad list order "goals": the players by the season statistic 2, most first, then by name.
/// Original: `compare_players_by_goals_then_name` (0x487140, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_players_by_goals_then_name(pa: *const usize, pb: *const usize) -> i32 {
    player_sort_by_season_stat(pa, pb, 2)
}

/// Squad list order "goals conceded": the players by the season statistic 3, most first, then by name.
/// Original: `compare_players_by_goals_conceded_then_name` (0x487250, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_players_by_goals_conceded_then_name(pa: *const usize, pb: *const usize) -> i32 {
    player_sort_by_season_stat(pa, pb, 3)
}

/// Squad list order "assists": the players by the season statistic 5, most first, then by name.
/// Original: `compare_players_by_assists_then_name` (0x487360, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_players_by_assists_then_name(pa: *const usize, pb: *const usize) -> i32 {
    player_sort_by_season_stat(pa, pb, 5)
}

/// Squad list order "average rating": the players by `PlayerStats::get_stat(player record, 6, 0x11, 6.5)`, highest
/// first (`player_sort_by_float_stat`: the first rounded to a float, a greater second gives 1, a smaller -1), then
/// by name; a null first entry sorts last (1), a null second one first (-1).
/// Original: `compare_players_by_average_rating_then_name` (0x487470, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_players_by_average_rating_then_name(pa: *const usize, pb: *const usize) -> i32 {
    player_sort_by_float_stat(pa, pb, 6, 0x11)
}

/// A job byte for the staff orders: 12 counts as 5.
fn staff_sort_job(job: i8) -> i32 {
    if job == 0xc { 5 } else { job as i32 }
}

/// Staff list order "club job": the staff by `staff::JOB_FOR_CLUB` (12 counting as 5), first - second, then by name;
/// a null first entry sorts last (1), a null second one first (-1).
/// Original: `compare_staff_by_club_job_then_name` (0x4875a0, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_staff_by_club_job_then_name(pa: *const usize, pb: *const usize) -> i32 {
    let (a, b) = player_sort_pair(pa, pb);
    if a == 0 {
        return 1;
    }
    if b == 0 {
        return -1;
    }
    let d = staff_sort_job(read_i8(a + staff::JOB_FOR_CLUB)) - staff_sort_job(read_i8(b + staff::JOB_FOR_CLUB));
    if d != 0 {
        return d;
    }
    player_sort_by_name(a, b)
}

/// Staff list order "nation job": the staff by `staff::JOB_FOR_NATION` (12 counting as 5), first - second, then by
/// name; a null first entry sorts last (1), a null second one first (-1).
/// Original: `compare_staff_by_nation_job_then_name` (0x487660, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_staff_by_nation_job_then_name(pa: *const usize, pb: *const usize) -> i32 {
    let (a, b) = player_sort_pair(pa, pb);
    if a == 0 {
        return 1;
    }
    if b == 0 {
        return -1;
    }
    let d = staff_sort_job(read_i8(a + staff::JOB_FOR_NATION)) - staff_sort_job(read_i8(b + staff::JOB_FOR_NATION));
    if d != 0 {
        return d;
    }
    player_sort_by_name(a, b)
}

/// Attribute list order: two attribute codes (bytes) by their long names.
///
/// A null first pointer sorts last (1), a null second one first (-1); a first code outside 1..=31 gives 1, then a
/// second one outside it -1; otherwise both names (`database_get_player_attribute_name(code, buffer, 200, 1)`) are
/// compared with `utils_string_compare(first, second, 1, 0)`.
/// Original: `compare_attributes_by_name` (0x487720, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_attributes_by_name(pa: *const usize, pb: *const usize) -> i32 {
    let (pa, pb) = (pa as usize, pb as usize);
    if pa == 0 {
        return 1;
    }
    if pb == 0 {
        return -1;
    }
    let (ka, kb) = (read_i8(pa), read_i8(pb));
    if !(1..=0x1f).contains(&ka) {
        return 1;
    }
    if !(1..=0x1f).contains(&kb) {
        return -1;
    }
    let name: unsafe extern "C" fn(u32, *mut u8, u32, u32) -> u32 = core::mem::transmute(0x5428a0usize);
    let (mut x, mut y) = ([0u8; 200], [0u8; 200]);
    name(ka as u8 as u32, x.as_mut_ptr(), 200, 1);
    name(kb as u8 as u32, y.as_mut_ptr(), 200, 1);
    exe::utils_string_compare(x.as_ptr(), y.as_ptr(), 1, 0)
}

/// Date list order: ascending. A null first pointer sorts last (1), a null second one first (-1); then -1 when the
/// first date is before the second (`Date::operator<`), 1 when after (`Date::operator>`), 0 when equal.
/// Original: `compare_dates_ascending` (0x4877c0, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_dates_ascending(pa: *const usize, pb: *const usize) -> i32 {
    let (pa, pb) = (pa as usize, pb as usize);
    if pa == 0 {
        return 1;
    }
    if pb == 0 {
        return -1;
    }
    if exe::date_operator_3c(pa, read_u32(pb), read_u32(pb + 4)) != 0 {
        return -1;
    }
    (exe::date_operator_3e(pa, read_u32(pb), read_u32(pb + 4)) != 0) as i32
}

/// Comparer for `compare_players_by_contract_expiry_then_name`: the order side by side.
pub(crate) unsafe extern "C" fn compare_compare_players_by_contract_expiry_then_name(a: *const usize, b: *const usize) -> i32 {
    player_sort_compare("contract expiry", original_fn(%%I0%%), compare_players_by_contract_expiry_then_name, a, b)
}

/// Comparer for `compare_players_by_fifa_protected_period_then_name`: the order side by side.
pub(crate) unsafe extern "C" fn compare_compare_players_by_fifa_protected_period_then_name(a: *const usize, b: *const usize) -> i32 {
    player_sort_compare("FIFA protected period", original_fn(%%I1%%), compare_players_by_fifa_protected_period_then_name, a, b)
}

/// Comparer for `compare_players_by_value_then_name`: the order side by side.
pub(crate) unsafe extern "C" fn compare_compare_players_by_value_then_name(a: *const usize, b: *const usize) -> i32 {
    player_sort_compare("value", original_fn(%%I2%%), compare_players_by_value_then_name, a, b)
}

/// Comparer for `compare_players_by_appearances_then_name`: the order side by side.
pub(crate) unsafe extern "C" fn compare_compare_players_by_appearances_then_name(a: *const usize, b: *const usize) -> i32 {
    player_sort_compare("appearances", original_fn(%%I3%%), compare_players_by_appearances_then_name, a, b)
}

/// Comparer for `compare_players_by_goals_then_name`: the order side by side.
pub(crate) unsafe extern "C" fn compare_compare_players_by_goals_then_name(a: *const usize, b: *const usize) -> i32 {
    player_sort_compare("goals", original_fn(%%I4%%), compare_players_by_goals_then_name, a, b)
}

/// Comparer for `compare_players_by_goals_conceded_then_name`: the order side by side.
pub(crate) unsafe extern "C" fn compare_compare_players_by_goals_conceded_then_name(a: *const usize, b: *const usize) -> i32 {
    player_sort_compare("goals conceded", original_fn(%%I5%%), compare_players_by_goals_conceded_then_name, a, b)
}

/// Comparer for `compare_players_by_assists_then_name`: the order side by side.
pub(crate) unsafe extern "C" fn compare_compare_players_by_assists_then_name(a: *const usize, b: *const usize) -> i32 {
    player_sort_compare("assists", original_fn(%%I6%%), compare_players_by_assists_then_name, a, b)
}

/// Comparer for `compare_players_by_average_rating_then_name`: the order side by side.
pub(crate) unsafe extern "C" fn compare_compare_players_by_average_rating_then_name(a: *const usize, b: *const usize) -> i32 {
    player_sort_compare("average rating", original_fn(%%I7%%), compare_players_by_average_rating_then_name, a, b)
}

/// Comparer for `compare_staff_by_club_job_then_name`: the order side by side.
pub(crate) unsafe extern "C" fn compare_compare_staff_by_club_job_then_name(a: *const usize, b: *const usize) -> i32 {
    player_sort_compare("staff club job", original_fn(%%I8%%), compare_staff_by_club_job_then_name, a, b)
}

/// Comparer for `compare_staff_by_nation_job_then_name`: the order side by side.
pub(crate) unsafe extern "C" fn compare_compare_staff_by_nation_job_then_name(a: *const usize, b: *const usize) -> i32 {
    player_sort_compare("staff nation job", original_fn(%%I9%%), compare_staff_by_nation_job_then_name, a, b)
}

/// Comparer for `compare_attributes_by_name`: the order side by side (the attribute codes, not staff pointers).
pub(crate) unsafe extern "C" fn compare_compare_attributes_by_name(a: *const usize, b: *const usize) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(*const usize, *const usize) -> i32>(%%I10%%);
    let r = o(a, b);
    let v = compare_attributes_by_name(a, b);
    SCOREBOARD.record(r == v, || format!("attribute list sort by name: original={} rust={}", r, v));
    r
}

/// Comparer for `compare_dates_ascending`: the order side by side (dates, not staff pointers).
pub(crate) unsafe extern "C" fn compare_compare_dates_ascending(a: *const usize, b: *const usize) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(*const usize, *const usize) -> i32>(%%I11%%);
    let r = o(a, b);
    let v = compare_dates_ascending(a, b);
    SCOREBOARD.record(r == v, || format!("date list sort ascending: original={} rust={}", r, v));
    r
}
