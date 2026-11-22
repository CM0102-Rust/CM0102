// ---------- batch %%LEVA%%: the search screens (1): the sort orders of the search results (search_screens.cpp) ----------

/// The keys the search screens sort by in a search result (`search_result`, 0x2d bytes): the staff id and the bytes the
/// result rows show.
#[allow(dead_code)]
pub(crate) mod search_result_keys {
    /// i32: the person's staff id (row of `G_STAFF_TABLE`).
    pub(crate) const STAFF_ID: usize = 0xc;
    /// i16: the person's reputation as the search stored it [inference: the reputation column's sort].
    pub(crate) const REPUTATION: usize = 0x21;
    /// i8: the current ability stars.
    pub(crate) const ABILITY_STARS: usize = 0x29;
    /// i8: the potential stars.
    pub(crate) const POTENTIAL_STARS: usize = 0x2a;
    /// i8: the player status code (ascending order).
    pub(crate) const STATUS_CODE: usize = 0x2b;
    /// i8: the manager's note rating, -1 when unrated.
    pub(crate) const NOTE_RATING: usize = 0x2c;
}

/// `player_search_slot_get_match_entry` (0x7bd830, thiscall(slot, char index) -> the match entry).
const SEARCH_FN_SLOT_GET_MATCH_ENTRY: usize = 0x7bd830;
/// The scouted player slot sums (thiscall(slot) -> short): goals, assists, goals conceded.
const SEARCH_FN_SLOT_SUM_GOALS: usize = 0x7bd870;
const SEARCH_FN_SLOT_SUM_ASSISTS: usize = 0x7bd8c0;
const SEARCH_FN_SLOT_SUM_GOALS_CONCEDED: usize = 0x7bd910;
/// `player_search_slot_average_rating` (0x7bd960, thiscall(slot) -> double in st0).
const SEARCH_FN_SLOT_AVERAGE_RATING: usize = 0x7bd960;
/// `national_teams_get_player_squad_slot` (0x76e070, thiscall(national teams, person) -> char, -1 outside the squad).
const SEARCH_FN_NATIONAL_SQUAD_SLOT: usize = 0x76e070;

/// The person of a search result: the row of `G_STAFF_TABLE` of its staff id.
pub(crate) unsafe fn search_result_person(entry: usize) -> Staff {
    staff_at(read_i32(entry + search_result_keys::STAFF_ID) as isize)
}

/// The tie-break of every search sort: the two people's names in style 3 (`exe::database_format_staff_name` into 0xc8
/// bytes, `a` first) through `exe::utils_string_compare(name a, name b, 1, 0)`.
pub(crate) unsafe fn search_results_compare_names(a: usize, b: usize) -> i32 {
    let (mut na, mut nb) = ([0u8; 0xc8], [0u8; 0xc8]);
    exe::database_format_staff_name(search_result_person(a), na.as_mut_ptr() as usize, 0xc8, 3);
    exe::database_format_staff_name(search_result_person(b), nb.as_mut_ptr() as usize, 0xc8, 3);
    exe::utils_string_compare(na.as_ptr(), nb.as_ptr(), 1, 0)
}

/// The scouted player slot of a result's person in the scouted club records (`find_club_search_entry`), 0 when no scout
/// follows his club.
unsafe fn search_result_scouted_slot(entry: usize) -> usize {
    find_club_search_entry(read_usize(G_SCOUT_MANAGER), search_result_person(entry).addr())
}

/// A thiscall on a scouted player slot returning a short (the match sums).
unsafe fn search_slot_sum(f: usize, slot: usize) -> i16 {
    let g: unsafe extern "thiscall" fn(usize) -> i16 = core::mem::transmute(f);
    g(slot)
}

/// The shared order of the scouted-match sorts: a person no scout follows goes last (`a` unfollowed 1, `b` unfollowed
/// -1, both: by name); otherwise `diff(slot a, slot b)` and, when it is 0, the name.
unsafe fn search_results_compare_scouted(a: usize, b: usize, diff: impl Fn(usize, usize) -> i32) -> i32 {
    let sa = search_result_scouted_slot(a);
    let sb = search_result_scouted_slot(b);
    if sa == 0 {
        if sb == 0 {
            return search_results_compare_names(a, b);
        }
        return 1;
    }
    if sb == 0 {
        return -1;
    }
    let d = diff(sa, sb);
    if d != 0 {
        return d;
    }
    search_results_compare_names(a, b)
}

/// Orders two search results by the manager's note rating, highest first, unrated (-1) last, then by name.
///
/// A result rated -1 goes after one that is rated (1 / -1); otherwise the byte difference `rating(b) - rating(a)`
/// (8 bits, sign-extended) and, when equal, `search_results_compare_names`.
/// Original: search_screens_compare_results_by_note_rating (0x8189f0, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_results_by_note_rating(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_1 sort by note rating");
    let (a, b) = (a as usize, b as usize);
    let ra = read_u8(a + search_result_keys::NOTE_RATING);
    let rb = read_u8(b + search_result_keys::NOTE_RATING);
    if ra == 0xff && rb != 0xff {
        return 1;
    }
    if ra != 0xff && rb == 0xff {
        return -1;
    }
    let d = rb.wrapping_sub(ra) as i8;
    if d != 0 {
        return d as i32;
    }
    search_results_compare_names(a, b)
}

/// Orders two search results by the higher of their ability and potential stars, best first, then by name.
///
/// Each side's key is max(ability stars, potential stars) (signed bytes); the byte difference `key(b) - key(a)`
/// (8 bits, sign-extended), then `search_results_compare_names`.
/// Original: search_screens_compare_results_by_best_stars (0x818ab0, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_results_by_best_stars(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_1 sort by best stars");
    let (a, b) = (a as usize, b as usize);
    let best = |e: usize| read_i8(e + search_result_keys::ABILITY_STARS).max(read_i8(e + search_result_keys::POTENTIAL_STARS));
    let d = (best(b) as u8).wrapping_sub(best(a) as u8) as i8;
    if d != 0 {
        return d as i32;
    }
    search_results_compare_names(a, b)
}

/// Orders two search results by the matches the scout watched (the slot's history count, +0x1a1), most first.
///
/// A person no scout follows (`find_club_search_entry` 0) goes last; both unfollowed: by name. Otherwise the byte
/// difference of the two history counts `count(b) - count(a)` (8 bits, sign-extended), then the name.
/// Original: search_screens_compare_results_by_matches_watched (0x818b60, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_results_by_matches_watched(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_1 sort by matches watched");
    search_results_compare_scouted(a as usize, b as usize, |sa, sb| {
        read_u8(sb + scouted_player_slot::HISTORY_COUNT).wrapping_sub(read_u8(sa + scouted_player_slot::HISTORY_COUNT)) as i8 as i32
    })
}

/// Orders two search results by the goals in the matches the scout watched (`player_search_slot_sum_goals`), most
/// first: an unfollowed person last, the 16-bit difference `goals(b) - goals(a)`, then the name.
/// Original: search_screens_compare_results_by_scouted_goals (0x818ce0, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_results_by_scouted_goals(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_1 sort by scouted goals");
    search_results_compare_scouted_sum(a as usize, b as usize, SEARCH_FN_SLOT_SUM_GOALS)
}

/// The three scouted sums share their order: an unfollowed person last, the 16-bit difference of the sums
/// `sum(b) - sum(a)` (sign-extended), then the name.
unsafe fn search_results_compare_scouted_sum(a: usize, b: usize, f: usize) -> i32 {
    search_results_compare_scouted(a, b, |sa, sb| search_slot_sum(f, sb).wrapping_sub(search_slot_sum(f, sa)) as i32)
}

/// Orders two search results by the assists in the matches the scout watched (`player_search_slot_sum_assists`), as
/// the goals.
/// Original: search_screens_compare_results_by_scouted_assists (0x818e70, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_results_by_scouted_assists(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_1 sort by scouted assists");
    search_results_compare_scouted_sum(a as usize, b as usize, SEARCH_FN_SLOT_SUM_ASSISTS)
}

/// Orders two search results by the goals conceded in the matches the scout watched
/// (`player_search_slot_sum_goals_conceded`), most first like the goals.
/// Original: search_screens_compare_results_by_scouted_goals_conceded (0x819000, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_results_by_scouted_goals_conceded(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_1 sort by scouted goals conceded");
    search_results_compare_scouted_sum(a as usize, b as usize, SEARCH_FN_SLOT_SUM_GOALS_CONCEDED)
}

/// Orders two search results by the average rating in the matches the scout watched, highest first.
///
/// An unfollowed person last (both: by name); otherwise `average(b) - average(a)` (`player_search_slot_average_rating`,
/// doubles): above 0 gives 1, equal goes to the name, below 0 or not a number gives -1.
/// Original: search_screens_compare_results_by_scouted_average_rating (0x819190, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_results_by_scouted_average_rating(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_1 sort by scouted average rating");
    let average: unsafe extern "thiscall" fn(usize) -> f64 = core::mem::transmute(SEARCH_FN_SLOT_AVERAGE_RATING);
    search_results_compare_scouted(a as usize, b as usize, |sa, sb| {
        let d = average(sb) - average(sa);
        if d > 0.0 {
            1
        } else if d == 0.0 {
            0 // equal: the name
        } else {
            -1 // below 0 or not a number (the x87 compare sets C0 for both)
        }
    })
}

/// Orders two search results by the date of the last match the scout watched, latest first.
///
/// An unfollowed person last (both: by name). Each slot's last match is entry `count - 1` of its history
/// (`player_search_slot_get_match_entry`), none with an empty history. `a` without one: `b` with one gives 1, neither
/// goes to the name. `a` with one: both dates (`Date::Date(day, year)` from the entry's +0 and +2) compared, the year
/// first (unsigned) then the day (signed): `b` later 1, earlier -1, equal by name. DEFECT of the original kept: `a`
/// with a last match and `b` with an empty history reads the date at address 0 (the original crashes there too).
/// Original: search_screens_compare_results_by_last_scouted_match (0x819340, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_results_by_last_scouted_match(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_1 sort by last scouted match");
    let entry: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(SEARCH_FN_SLOT_GET_MATCH_ENTRY);
    let last = |s: usize| {
        let n = read_i8(s + scouted_player_slot::HISTORY_COUNT);
        if n > 0 { entry(s, (n - 1) as u8 as u32) } else { 0 }
    };
    search_results_compare_scouted(a as usize, b as usize, |sa, sb| {
        let ea = last(sa);
        let eb = last(sb);
        if ea == 0 {
            return (eb != 0) as i32; // neither: the name
        }
        let (mut da, mut db) = ([0u8; 8], [0u8; 8]);
        exe::new_date(da.as_mut_ptr() as usize, read_u16(ea) as u32, read_u16(ea + 2) as u32);
        exe::new_date(db.as_mut_ptr() as usize, read_u16(eb) as u32, read_u16(eb + 2) as u32);
        let year = |d: &[u8; 8]| u16::from_le_bytes([d[2], d[3]]);
        let day = |d: &[u8; 8]| i16::from_le_bytes([d[0], d[1]]);
        if year(&db) > year(&da) {
            1
        } else if year(&db) < year(&da) {
            -1
        } else if day(&db) > day(&da) {
            1
        } else if day(&db) < day(&da) {
            -1
        } else {
            0
        }
    })
}

/// Orders two search results by their ability stars, best first: a null result goes last (`a` null 1, `b` null -1),
/// then the byte difference `stars(b) - stars(a)` and the name.
/// Original: search_screens_compare_results_by_ability_stars (0x819620, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_results_by_ability_stars(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_1 sort by ability stars");
    let (a, b) = (a as usize, b as usize);
    if a == 0 {
        return 1;
    }
    if b == 0 {
        return -1;
    }
    let d = read_u8(b + search_result_keys::ABILITY_STARS).wrapping_sub(read_u8(a + search_result_keys::ABILITY_STARS)) as i8;
    if d != 0 {
        return d as i32;
    }
    search_results_compare_names(a, b)
}

/// Orders two search results by their potential stars, best first: the byte difference `stars(b) - stars(a)`, then
/// the name.
/// Original: search_screens_compare_results_by_potential_stars (0x8196e0, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_results_by_potential_stars(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_1 sort by potential stars");
    let (a, b) = (a as usize, b as usize);
    let d = read_u8(b + search_result_keys::POTENTIAL_STARS).wrapping_sub(read_u8(a + search_result_keys::POTENTIAL_STARS)) as i8;
    if d != 0 {
        return d as i32;
    }
    search_results_compare_names(a, b)
}

/// Orders two search results by the slot in the national squad, lowest first, players outside it (-1, counted as
/// 0x7f) last: the byte difference `slot(a) - slot(b)` (`national_teams_get_player_squad_slot`), then the name.
/// Original: search_screens_compare_results_by_national_squad_slot (0x819780, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_results_by_national_squad_slot(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_1 sort by national squad slot");
    let (a, b) = (a as usize, b as usize);
    let slot_of: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(SEARCH_FN_NATIONAL_SQUAD_SLOT);
    let slot = |e: usize| {
        let s = slot_of(read_usize(G_NATIONAL_TEAMS), search_result_person(e).addr());
        if s == 0xff { 0x7f } else { s }
    };
    let sa = slot(a);
    let d = sa.wrapping_sub(slot(b)) as i8;
    if d != 0 {
        return d as i32;
    }
    search_results_compare_names(a, b)
}

/// Orders two search results by the player status code, ascending: the byte difference `code(a) - code(b)`, then the
/// name.
/// Original: search_screens_compare_results_by_status_code (0x819870, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_results_by_status_code(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_1 sort by status code");
    let (a, b) = (a as usize, b as usize);
    let d = read_u8(a + search_result_keys::STATUS_CODE).wrapping_sub(read_u8(b + search_result_keys::STATUS_CODE)) as i8;
    if d != 0 {
        return d as i32;
    }
    search_results_compare_names(a, b)
}

/// Orders two search results by name only (`search_results_compare_names`).
/// Original: search_screens_compare_results_by_name (0x819910, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_results_by_name(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_1 sort by name");
    search_results_compare_names(a as usize, b as usize)
}

compare_order!(compare_search_screens_compare_results_by_note_rating, search_screens_compare_results_by_note_rating, %%I0%%);
compare_order!(compare_search_screens_compare_results_by_best_stars, search_screens_compare_results_by_best_stars, %%I1%%);
compare_order!(compare_search_screens_compare_results_by_matches_watched, search_screens_compare_results_by_matches_watched, %%I2%%);
compare_order!(compare_search_screens_compare_results_by_scouted_goals, search_screens_compare_results_by_scouted_goals, %%I3%%);
compare_order!(compare_search_screens_compare_results_by_scouted_assists, search_screens_compare_results_by_scouted_assists, %%I4%%);
compare_order!(compare_search_screens_compare_results_by_scouted_goals_conceded, search_screens_compare_results_by_scouted_goals_conceded, %%I5%%);
compare_order!(compare_search_screens_compare_results_by_scouted_average_rating, search_screens_compare_results_by_scouted_average_rating, %%I6%%);
compare_order!(compare_search_screens_compare_results_by_last_scouted_match, search_screens_compare_results_by_last_scouted_match, %%I7%%);
compare_order!(compare_search_screens_compare_results_by_ability_stars, search_screens_compare_results_by_ability_stars, %%I8%%);
compare_order!(compare_search_screens_compare_results_by_potential_stars, search_screens_compare_results_by_potential_stars, %%I9%%);
compare_order!(compare_search_screens_compare_results_by_national_squad_slot, search_screens_compare_results_by_national_squad_slot, %%I10%%);
compare_order!(compare_search_screens_compare_results_by_status_code, search_screens_compare_results_by_status_code, %%I11%%);
compare_order!(compare_search_screens_compare_results_by_name, search_screens_compare_results_by_name, %%I12%%);
