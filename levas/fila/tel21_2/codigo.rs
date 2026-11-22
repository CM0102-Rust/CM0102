// ---------- batch %%LEVA%%: the search screens (2): the sort orders of the staff search columns (search_screens.cpp) ----------

/// The search settings of the screen being sorted (`g_search_session_settings_ptr`): +8 the statistic column, +9 the
/// attribute column.
const SEARCH_SETTINGS: usize = 0xdd6cc8;
const SEARCH_SETTINGS_STAT: usize = 0x8;
const SEARCH_SETTINGS_ATTRIBUTE: usize = 0x9;
/// The person who looks at the search (`g_search_screen_manager_staff`): what he can see of the attributes.
const SEARCH_VIEWER: usize = 0xdd6ccc;
/// `database_format_player_positions` (0x53ee50, cdecl(player, buffer, size, style)).
const SEARCH_FN_FORMAT_PLAYER_POSITIONS: usize = 0x53ee50;
/// The national job code that does not count as a job (the employer and job columns skip it).
const SEARCH_NATIONAL_JOB_NONE: u8 = 0xb;
/// The job key of a person without one (sorted last).
const SEARCH_NO_JOB: u8 = 0x64;
/// The reputation of an established person (3750): a person under 35 below it is ranked at it, one exactly at it just
/// above.
const SEARCH_REPUTATION_ESTABLISHED: i16 = 0xea6;

/// Sorts two search entries by the short name of the person's club, a person without a club last (`a` 1, `b` -1),
/// through `exe::utils_string_compare(name a, name b, 1, 0)`, then by name.
/// Original: search_screens_compare_staff_by_club (0x819990, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_staff_by_club(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_2 sort by club");
    let (a, b) = (a as usize, b as usize);
    let ca = search_result_person(a).club_job();
    let cb = search_result_person(b).club_job();
    if ca.is_null() {
        return 1;
    }
    if cb.is_null() {
        return -1;
    }
    let r = exe::utils_string_compare((ca.addr() + club::SHORT_NAME) as *const u8, (cb.addr() + club::SHORT_NAME) as *const u8, 1, 0);
    if r != 0 {
        return r;
    }
    search_results_compare_names(a, b)
}

/// Sorts two search entries by the short name of the employer: the club, or without one the national side when the
/// national job is not 0xb; no employer last (`a` 1, `b` -1); then by name.
/// Original: search_screens_compare_staff_by_employer (0x819a80, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_staff_by_employer(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_2 sort by employer");
    let (a, b) = (a as usize, b as usize);
    let employer = |p: Staff| {
        let c = p.club_job();
        if c.is_null() && !p.national_job().is_null() && p.job_for_nation() != SEARCH_NATIONAL_JOB_NONE {
            p.national_job()
        } else {
            c
        }
    };
    let ea = employer(search_result_person(a));
    let eb = employer(search_result_person(b));
    if ea.is_null() {
        return 1;
    }
    if eb.is_null() {
        return -1;
    }
    let r = exe::utils_string_compare((ea.addr() + club::SHORT_NAME) as *const u8, (eb.addr() + club::SHORT_NAME) as *const u8, 1, 0);
    if r != 0 {
        return r;
    }
    search_results_compare_names(a, b)
}

/// Sorts two search entries by league: the division of a club in the nation of the current human's club by its
/// reputation, everything else by nation name.
///
/// For each side (`a` first): a person with a club whose division's nation is the nation of the club of the current
/// slot's human manager (`tel26_slot_manager`) keys on that division; otherwise on the club's nation name, or without
/// a club nation on the person's own nation name; a person without any of them stops the sort (`a` 1, `b` -1). Two
/// names: `exe::utils_string_compare(name a, name b, 1, 0)`; a name against a division: the name side goes last (`a` 1,
/// `b` -1); two divisions: `reputation(b) - reputation(a)` (16-bit values, 32-bit difference). Equal: by name.
/// Original: search_screens_compare_staff_by_league (0x819ba0, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_staff_by_league(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_2 sort by league");
    let (a, b) = (a as usize, b as usize);
    let manager = tel26_slot_manager();
    // Some((division, 0)) or Some((0, nation name)); None: no nation at all
    let key = |p: Staff| -> Option<(usize, usize)> {
        let club = p.club_job();
        if !club.is_null() {
            let division = club.division();
            if !division.is_null() && !manager.club_job().is_null() && division.nation().addr() == manager.club_job().nation().addr() {
                return Some((division.addr(), 0));
            }
            let n = club.nation();
            if !n.is_null() {
                return Some((0, n.addr() + nation::SHORT_NAME));
            }
        }
        let n = p.nation();
        if n.is_null() { None } else { Some((0, n.addr() + nation::SHORT_NAME)) }
    };
    let Some((da, na)) = key(search_result_person(a)) else { return 1 };
    let Some((db, nb)) = key(search_result_person(b)) else { return -1 };
    let r = if da == 0 {
        if db != 0 {
            return 1;
        }
        exe::utils_string_compare(na as *const u8, nb as *const u8, 1, 0)
    } else {
        if db == 0 {
            return -1;
        }
        (Competition(db).reputation() as i32).wrapping_sub(Competition(da).reputation() as i32)
    };
    if r != 0 {
        return r;
    }
    search_results_compare_names(a, b)
}

/// Sorts two search entries by the short positions text of the player record (`database_format_player_positions`
/// style 4 into 0xc8 bytes) through `exe::utils_string_compare(text a, text b, 1, 0)`, then by name.
/// Original: search_screens_compare_staff_by_position (0x819da0, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_staff_by_position(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_2 sort by position");
    let (a, b) = (a as usize, b as usize);
    let positions: unsafe extern "C" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(SEARCH_FN_FORMAT_PLAYER_POSITIONS);
    let (mut ta, mut tb) = ([0u8; 0xc8], [0u8; 0xc8]);
    positions(search_result_person(a).player().addr(), ta.as_mut_ptr() as usize, 0xc8, 4);
    positions(search_result_person(b).player().addr(), tb.as_mut_ptr() as usize, 0xc8, 4);
    let r = exe::utils_string_compare(ta.as_ptr(), tb.as_ptr(), 1, 0);
    if r != 0 {
        return r;
    }
    search_results_compare_names(a, b)
}

/// Sorts two search entries by job code, ascending: the club job (`job_for_club`) with a club, otherwise the national
/// job when there is a national side and it is not 0xb; none, or a code 0, counts as 0x64 (last). The byte difference
/// `job(a) - job(b)`, then the name.
/// Original: search_screens_compare_staff_by_job (0x819ea0, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_staff_by_job(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_2 sort by job");
    let (a, b) = (a as usize, b as usize);
    let job = |p: Staff| {
        let j = if !p.club_job().is_null() {
            p.job_for_club()
        } else if !p.national_job().is_null() && p.job_for_nation() != SEARCH_NATIONAL_JOB_NONE {
            p.job_for_nation()
        } else {
            SEARCH_NO_JOB
        };
        if j == 0 { SEARCH_NO_JOB } else { j }
    };
    let ja = job(search_result_person(a));
    let d = ja.wrapping_sub(job(search_result_person(b))) as i8;
    if d != 0 {
        return d as i32;
    }
    search_results_compare_names(a, b)
}

/// Sorts two search entries by estimated value, highest first: `value(b) - value(a)` (32 bits), then the name.
/// Original: search_screens_compare_staff_by_value (0x819f70, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_staff_by_value(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_2 sort by value");
    let (a, b) = (a as usize, b as usize);
    let d = (search_result_person(b).value() as i32).wrapping_sub(search_result_person(a).value() as i32);
    if d != 0 {
        return d;
    }
    search_results_compare_names(a, b)
}

/// Sorts two search entries by the stored reputation (`search_result_keys::REPUTATION`), highest first.
///
/// Below 3750 a person under 35 (the age byte, signed) is ranked at 3750; exactly 3750 counts as 3751. The 16-bit
/// difference `key(b) - key(a)` (sign-extended), then the name.
/// Original: search_screens_compare_staff_by_reputation (0x81a000, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_staff_by_reputation(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_2 sort by reputation");
    let (a, b) = (a as usize, b as usize);
    let key = |e: usize| {
        let r = read_i16(e + search_result_keys::REPUTATION);
        if r < SEARCH_REPUTATION_ESTABLISHED {
            if read_i8(search_result_person(e).addr() + staff::AGE) < 0x23 { SEARCH_REPUTATION_ESTABLISHED } else { r }
        } else if r == SEARCH_REPUTATION_ESTABLISHED {
            r + 1
        } else {
            r
        }
    };
    let ka = key(a);
    let d = key(b).wrapping_sub(ka);
    if d != 0 {
        return d as i32;
    }
    search_results_compare_names(a, b)
}

/// Sorts two search entries by the selected player statistic (`exe::playerstats_get_stat(player, 6, settings + 8,
/// 6.5)`), highest first: `a`'s value kept as a float, `b`'s above it 1, equal by name, below (or not a number) -1.
/// Original: search_screens_compare_staff_by_stat (0x81a0f0, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_staff_by_stat(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_2 sort by stat");
    let (a, b) = (a as usize, b as usize);
    let stat = |e: usize| {
        let column = read_u8(read_usize(SEARCH_SETTINGS) + SEARCH_SETTINGS_STAT) as u32;
        exe::playerstats_get_stat(read_usize(G_PLAYER_STATS), search_result_person(e).player().addr(), 6, column, 6.5)
    };
    let sa = stat(a) as f32;
    let sb = stat(b);
    if sb > sa as f64 {
        return 1;
    }
    if sb != sa as f64 {
        return -1; // below, or not a number (the x87 compare sets C0 for both)
    }
    search_results_compare_names(a, b)
}

/// Sorts two search entries by the selected player attribute (settings + 9) where the viewer can see it
/// (`exe::staff_is_player_attribute_visible`; hidden counts as -1), highest first: the byte difference
/// `value(b) - value(a)`, then the name.
/// Original: search_screens_compare_staff_by_player_attribute (0x81a220, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_staff_by_player_attribute(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_2 sort by player attribute");
    let (a, b) = (a as usize, b as usize);
    let value = |e: usize| {
        let p = search_result_person(e);
        let attribute = read_u8(read_usize(SEARCH_SETTINGS) + SEARCH_SETTINGS_ATTRIBUTE) as u32;
        if exe::staff_is_player_attribute_visible(read_usize(SEARCH_VIEWER), p.addr(), attribute) != 0 {
            exe::staff_get_player_attribute(p.addr(), attribute)
        } else {
            -1
        }
    };
    let va = value(a);
    let d = (value(b) as u8).wrapping_sub(va as u8) as i8;
    if d != 0 {
        return d as i32;
    }
    search_results_compare_names(a, b)
}

/// Sorts two search entries by the selected non-player attribute (settings + 9, `exe::staff_get_non_player_attribute`,
/// `b` read first), highest first: the byte difference `value(b) - value(a)`, then the name.
/// Original: search_screens_compare_staff_by_non_player_attribute (0x81a360, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_staff_by_non_player_attribute(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_2 sort by non-player attribute");
    let (a, b) = (a as usize, b as usize);
    let value = |e: usize| {
        let attribute = read_u8(read_usize(SEARCH_SETTINGS) + SEARCH_SETTINGS_ATTRIBUTE) as u32;
        exe::staff_get_non_player_attribute(search_result_person(e).addr(), attribute)
    };
    let vb = value(b);
    let d = (vb as u8).wrapping_sub(value(a) as u8) as i8;
    if d != 0 {
        return d as i32;
    }
    search_results_compare_names(a, b)
}

/// Sorts two search entries by the three-letter code of the person's nation, no nation last (`a` 1, `b` -1), through
/// `exe::utils_string_compare(code a, code b, 1, 0)` (the original compares copies), then by name.
/// Original: search_screens_compare_staff_by_nationality (0x81a450, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_staff_by_nationality(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_2 sort by nationality");
    let (a, b) = (a as usize, b as usize);
    let na = search_result_person(a).nation();
    let nb = search_result_person(b).nation();
    if na.is_null() {
        return 1;
    }
    if nb.is_null() {
        return -1;
    }
    let r = exe::utils_string_compare((na.addr() + nation::THREE_LETTER_NAME) as *const u8, (nb.addr() + nation::THREE_LETTER_NAME) as *const u8, 1, 0);
    if r != 0 {
        return r;
    }
    search_results_compare_names(a, b)
}

/// Sorts two search entries by age (the signed age byte), oldest first: `age(b) - age(a)`, then the name.
/// Original: search_screens_compare_staff_by_age (0x81a5a0, search_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn search_screens_compare_staff_by_age(a: *const usize, b: *const usize) -> i32 {
    covered!("tel21_2 sort by age");
    let (a, b) = (a as usize, b as usize);
    let age = |e: usize| read_i8(search_result_person(e).addr() + staff::AGE) as i32;
    let d = age(b) - age(a);
    if d != 0 {
        return d;
    }
    search_results_compare_names(a, b)
}

compare_order!(compare_search_screens_compare_staff_by_club, search_screens_compare_staff_by_club, %%I0%%);
compare_order!(compare_search_screens_compare_staff_by_employer, search_screens_compare_staff_by_employer, %%I1%%);
compare_order!(compare_search_screens_compare_staff_by_league, search_screens_compare_staff_by_league, %%I2%%);
compare_order!(compare_search_screens_compare_staff_by_position, search_screens_compare_staff_by_position, %%I3%%);
compare_order!(compare_search_screens_compare_staff_by_job, search_screens_compare_staff_by_job, %%I4%%);
compare_order!(compare_search_screens_compare_staff_by_value, search_screens_compare_staff_by_value, %%I5%%);
compare_order!(compare_search_screens_compare_staff_by_reputation, search_screens_compare_staff_by_reputation, %%I6%%);
compare_order!(compare_search_screens_compare_staff_by_stat, search_screens_compare_staff_by_stat, %%I7%%);
compare_order!(compare_search_screens_compare_staff_by_player_attribute, search_screens_compare_staff_by_player_attribute, %%I8%%);
compare_order!(compare_search_screens_compare_staff_by_non_player_attribute, search_screens_compare_staff_by_non_player_attribute, %%I9%%);
compare_order!(compare_search_screens_compare_staff_by_nationality, search_screens_compare_staff_by_nationality, %%I10%%);
compare_order!(compare_search_screens_compare_staff_by_age, search_screens_compare_staff_by_age, %%I11%%);
