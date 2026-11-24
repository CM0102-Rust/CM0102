// ---------- sbG_3: dealing the Swedish Second Division clubs into the six regional sections (swe_prm.cpp) ----------

/// The six Swedish Second Division sections in the layout of the original's contiguous array of 72 clubs (twelve each):
/// North, South Götaland, East Svealand, West Svealand, East Götaland, West Götaland.
pub(crate) const SWEDISH_SECTION_LAYOUT: [usize; 6] = [KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION_NORTH, KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION_SOUTH_GOTALAND,
    KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION_EAST_SVEALAND, KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION_WEST_SVEALAND, KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION_EAST_GOTALAND,
    KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION_WEST_GOTALAND];

/// The order the dealing tries the sections in (indices into `SWEDISH_SECTION_LAYOUT`): North, South Götaland, East
/// Götaland, West Götaland, East Svealand, West Svealand.
pub(crate) const SWEDISH_SECTION_TRY_ORDER: [usize; 6] = [0, 1, 4, 5, 2, 3];

/// How well a club's city fits a Swedish Second Division section, from 0 to 100, by the latitude and longitude of its
/// stadium's city.
///
/// No stadium (`club::STADIUM`) or no city (`stadium::CITY`): 20. South Götaland: latitude below 54 gives 100, below 55
/// 80, below 56 70, below 57 60, below 59 40. North: above 62 100, above 61 90, above 60 70, above 58 50. East Götaland:
/// latitude between 57 and 59 and longitude between 14 and 17, 100; between 56 and 60 and 13 and 18, 80. West Götaland:
/// latitude between 58 and 58.5 and longitude below 15, 100; latitude between 57.5 and 59 and longitude below 16, 80.
/// East Svealand: latitude between 57 and 59 and longitude above 17, 100; between 56 and 60 and above 16, 80. West
/// Svealand: latitude between 59.3 and 59.7 and longitude between 14 and 17, 100; between 59 and 60 and 13 and 18, 80.
/// Anything else (another section or no fit) 0. All bounds strict; the original's x87 compares take an unordered
/// (NaN) coordinate as below every bound and above none (`below`, `above`).
/// Original: `swe_prm_region_fit_score` (0x891370, swe_prm.cpp, thiscall with ecx unread, ret 8).
pub(crate) unsafe extern "thiscall" fn swe_prm_region_fit_score(_this: usize, club: usize, section: u32) -> i32 {
    let stadium = Club(club).stadium();
    if stadium.0 == 0 || stadium.city() == 0 {
        return 0x14;
    }
    let city = City(stadium.city());
    let (lat, lon) = (city.latitude(), city.longitude());
    let below = |a: f64, b: f64| !(a >= b);
    let above = |a: f64, b: f64| a > b;
    let is = |key: usize| section == read_u32(key);
    if is(KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION_SOUTH_GOTALAND) {
        covered!("sbG_3 swedish region score");
        return [(54.0, 0x64), (55.0, 0x50), (56.0, 0x46), (57.0, 0x3c), (59.0, 0x28)].iter().find(|&&(b, _)| below(lat, b)).map_or(0, |&(_, s)| s); // sbG_3: the south by latitude
    }
    if is(KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION_NORTH) {
        return [(62.0, 0x64), (61.0, 0x5a), (60.0, 0x46), (58.0, 0x32)].iter().find(|&&(b, _)| above(lat, b)).map_or(0, |&(_, s)| s);
    }
    let fits = |lat_from: f64, lat_to: f64, lon_from: Option<f64>, lon_to: Option<f64>| {
        above(lat, lat_from) && below(lat, lat_to) && lon_from.map_or(true, |b| above(lon, b)) && lon_to.map_or(true, |b| below(lon, b))
    };
    let (core, wide) = if is(KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION_EAST_GOTALAND) {
        (fits(57.0, 59.0, Some(14.0), Some(17.0)), fits(56.0, 60.0, Some(13.0), Some(18.0)))
    } else if is(KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION_WEST_GOTALAND) {
        (fits(58.0, 58.5, None, Some(15.0)), fits(57.5, 59.0, None, Some(16.0)))
    } else if is(KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION_EAST_SVEALAND) {
        (fits(57.0, 59.0, Some(17.0), None), fits(56.0, 60.0, Some(16.0), None))
    } else if is(KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION_WEST_SVEALAND) {
        (fits(59.3, 59.7, Some(14.0), Some(17.0)), fits(59.0, 60.0, Some(13.0), Some(18.0)))
    } else {
        (false, false)
    };
    if core { 0x64 } else if wide { 0x50 } else { 0 }
}

/// Comparer for `swe_prm_region_fit_score`: a pure function, the two results side by side.
pub(crate) unsafe extern "thiscall" fn compare_swe_prm_region_fit_score(this: usize, club: usize, section: u32) -> i32 {
    let a = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> i32>(%%I1%%)(this, club, section);
    let b = swe_prm_region_fit_score(this, club, section);
    SCOREBOARD.record(a == b, || format!("swe_prm_region_fit_score(club {:#x}, section {}): original {} rust {}", club, section as i32, a, b));
    a
}

/// The clubs the dealing works on: in database order (the first [`G_CLUB_COUNT`] − 2 × [`G_NATION_COUNT`] records),
/// those whose division (`club::DIVISION`) is the Second Division ([`KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION`]), up to 72;
/// and how many of them have a stadium with a city.
pub(crate) unsafe fn swedish_second_division_clubs() -> (Vec<usize>, i8) {
    let (mut v, mut with_city) = (Vec::new(), 0i8);
    let n = read_i32(G_CLUB_COUNT).wrapping_sub(2 * read_i32(G_NATION_COUNT));
    let mut i = 0;
    while i < n && v.len() < 0x48 {
        let club = read_usize(G_CLUB_TABLE) + i as usize * club::SIZE;
        let d = read_usize(club + club::DIVISION);
        if d != 0 && read_u32(d) == read_u32(KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION) {
            v.push(club);
            if Club(club).stadium().0 != 0 && Club(club).stadium().city() != 0 {
                with_city = with_city.wrapping_add(1);
            }
        }
        i += 1;
    }
    (v, with_city)
}

/// Deals the 72 Swedish Second Division clubs into its six regional sections of twelve (`club::RESERVE_DIVISION`), by
/// where their city lies, keeping as many clubs as it can in the section they had.
///
/// The clubs of `swedish_second_division_clubs`; fewer than 71: the error box of swe_prm.cpp (0xa8b578) line 0x2ca and
/// nothing else. With 71, the first club (database order) of the Lower Division ([`KEY_CLUB_COMP_SWEDISH_LOWER_DIVISION`])
/// fills the gap: its division goes to `club::LAST_DIVISION`, the Second Division's record to its division and South
/// Götaland's record to its section. First pass, with a threshold from 100 down in steps of 10: each club not yet
/// placed goes to the first section, in `SWEDISH_SECTION_TRY_ORDER`, where `swe_prm_region_fit_score` reaches the
/// threshold and fewer than twelve are placed; the passes stop when every club with a city is placed or all 72 are.
/// Then the rest go to their own section (the one they had) and, in a last pass, to any section with room. Each club's
/// old section is kept; every placed club gets its new section's record; then, to keep clubs where they were, each club
/// that moved is swapped with a later one that moved the opposite way between the same two sections, and three clubs
/// that moved in a cycle are put back. With 71 clubs and no Lower Division club, one of the 72 places stays empty and the
/// original reads a stale stack dword as a club there (a defect); the copy takes the place as empty.
/// Original: `swe_prm_assign_second_division_sections` (0x890c70, swe_prm.cpp, thiscall: ecx = the Allsvenskan, ret 0).
pub(crate) unsafe extern "thiscall" fn swe_prm_assign_second_division_sections(this: usize) {
    let record = |k: usize| comp_record_of(SWEDISH_SECTION_LAYOUT[k]);
    let (mut clubs, with_city) = swedish_second_division_clubs();
    if clubs.len() < 0x47 {
        return show_error_at(0xa8b578, 0x2ca);
    }
    covered!("sbG_3 swedish sections dealt");
    if clubs.len() == 0x47 {
        if let Some(c) = swedish_lower_division_club() {
            write_usize(c + club::LAST_DIVISION, read_usize(c + club::DIVISION));
            write_usize(c + club::DIVISION, comp_record_of(KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION));
            write_usize(c + club::RESERVE_DIVISION, comp_record_of(KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION_SOUTH_GOTALAND));
            clubs.push(c);
        }
    }
    let n = clubs.len();
    let mut all = [0usize; 0x48];
    let mut count = [0usize; 6];
    fn place(e: &mut usize, s: usize, all: &mut [usize; 0x48], count: &mut [usize; 6]) {
        all[s * 12 + count[s]] = *e;
        count[s] += 1;
        *e = 0;
    }
    let mut threshold = 0x64i32;
    loop {
        for e in clubs.iter_mut().filter(|e| **e != 0) {
            for &s in &SWEDISH_SECTION_TRY_ORDER {
                if swe_prm_region_fit_score(this, *e, read_u32(SWEDISH_SECTION_LAYOUT[s])) >= threshold && count[s] < 12 {
                    place(e, s, &mut all, &mut count);
                    break;
                }
            }
        }
        let total: usize = count.iter().sum();
        if with_city as i32 == total as i32 || total == 0x48 {
            break;
        }
        threshold -= 0xa; // sbG_3: the threshold steps down by 10
    }
    for pass in 0..2 {
        for e in clubs.iter_mut().filter(|e| **e != 0) {
            for &s in &SWEDISH_SECTION_TRY_ORDER {
                if (pass == 1 || read_usize(*e + club::RESERVE_DIVISION) == record(s)) && count[s] < 12 {
                    place(e, s, &mut all, &mut count);
                    break;
                }
            }
        }
    }
    let section_of = |c: usize| if c != 0 { read_usize(c + club::RESERVE_DIVISION) } else { 0 };
    let set_section = |c: usize, v: usize| if c != 0 { write_usize(c + club::RESERVE_DIVISION, v) };
    let old: Vec<usize> = (0..n).map(|k| section_of(all[k])).collect();
    for (s, &placed) in count.iter().enumerate() {
        for &c in &all[s * 12..s * 12 + placed] {
            set_section(c, record(s));
        }
    }
    swap_swedish_section_pairs(&all[..n], &old);
    undo_swedish_section_cycles(&all[..n], &old);
}

/// The first club (database order, the same records as `swedish_second_division_clubs`) of the Swedish Lower Division.
pub(crate) unsafe fn swedish_lower_division_club() -> Option<usize> {
    let n = read_i32(G_CLUB_COUNT).wrapping_sub(2 * read_i32(G_NATION_COUNT));
    (0..n.max(0) as usize).map(|i| read_usize(G_CLUB_TABLE) + i * club::SIZE).find(|&c| {
        let d = read_usize(c + club::DIVISION);
        c != 0 && d != 0 && read_u32(d) == read_u32(KEY_CLUB_COMP_SWEDISH_LOWER_DIVISION)
    })
}

/// The pair swaps after the dealing: for each club i (but the last) that moved, the first later club j that also moved,
/// from i's new section into i's old one, takes its old section back and i too.
pub(crate) unsafe fn swap_swedish_section_pairs(all: &[usize], old: &[usize]) {
    let section_of = |c: usize| if c != 0 { read_usize(c + club::RESERVE_DIVISION) } else { 0 };
    for i in 0..all.len().saturating_sub(1) {
        let now = section_of(all[i]);
        if now == old[i] {
            continue;
        }
        if let Some(j) = (i + 1..all.len()).find(|&j| section_of(all[j]) != old[j] && old[j] == now && section_of(all[j]) == old[i]) {
            covered!("sbG_3 swedish section pair swapped");
            if all[i] != 0 {
                write_usize(all[i] + club::RESERVE_DIVISION, old[i]);
            }
            if all[j] != 0 {
                write_usize(all[j] + club::RESERVE_DIVISION, old[j]); // sbG_3: both back where they were
            }
        }
    }
}

/// The three-club cycles after the pair swaps: for each club a that moved, the first club b (≠ a) that moved for which a
/// club c (≠ a, b) that moved closes a cycle (a now in b's old section, b in c's old one, c in a's old one) puts all
/// three back in their old sections.
pub(crate) unsafe fn undo_swedish_section_cycles(all: &[usize], old: &[usize]) {
    let section_of = |c: usize| if c != 0 { read_usize(c + club::RESERVE_DIVISION) } else { 0 };
    let n = all.len();
    for i in 0..n {
        if section_of(all[i]) == old[i] {
            continue;
        }
        'b: for j in 0..n {
            let b_now = section_of(all[j]);
            if j == i || b_now == old[j] {
                continue;
            }
            for k in 0..n {
                if k == i || k == j {
                    continue;
                }
                let c_now = section_of(all[k]);
                if c_now != old[k] && section_of(all[i]) == old[j] && b_now == old[k] && c_now == old[i] {
                    covered!("sbG_3 swedish section cycle undone");
                    for (c, v) in [(all[i], old[i]), (all[j], old[j]), (all[k], old[k])] {
                        if c != 0 {
                            write_usize(c + club::RESERVE_DIVISION, v);
                        }
                    }
                    break 'b;
                }
            }
        }
    }
}

/// Comparer for `swe_prm_assign_second_division_sections`: side by side over each club's divisions and section
/// (`club::DIVISION` to `club::RESERVE_DIVISION`, the records the routine may write). With fewer than 71 Second Division
/// clubs only the original runs (the error box would show twice).
pub(crate) unsafe extern "thiscall" fn compare_swe_prm_assign_second_division_sections(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    if swedish_second_division_clubs().0.len() < 0x47 {
        return o(this);
    }
    let n = read_i32(G_CLUB_COUNT).wrapping_sub(2 * read_i32(G_NATION_COUNT)).max(0) as usize;
    let regions: Vec<(usize, usize)> = (0..n).map(|i| (read_usize(G_CLUB_TABLE) + i * club::SIZE + club::DIVISION, club::RESERVE_DIVISION + 4 - club::DIVISION)).collect();
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&regions, || o(this), || swe_prm_assign_second_division_sections(this));
    SCOREBOARD.record(same, || format!("swe_prm_assign_second_division_sections({:#x}): the sections differ: original {:?} rust {:?}", this, mo, mr));
}
