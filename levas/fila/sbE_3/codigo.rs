// ---------- sbE_3: the Bundesliga table and the Regionalliga split into north and south (ger_first.cpp) ----------

/// Rule: the Bundesliga has 18 clubs; prize fund by the share of 200,000 per place; TV pays 5,000,000 to each club. In
/// 2001 the clubs of the four old regional divisions are first gathered into the Regionalliga (their division kept as
/// the reserve division).
///
/// `GerFirst::create_table()`: `league::CLUB_COUNT` = 18; in 2001 (`league::YEAR` = 2001) `move_group_clubs_to_division`
/// (sbE_2) of `KEY_CLUB_COMP_GERMAN_REGIONAL_DIVISION_WEST_SOUTHWEST`, `_EAST`, `_NORTH` and `_SOUTH` into
/// `KEY_CLUB_COMP_GERMAN_REGIONAL`; then `eng_prm_fill_league_table` with the clubs of
/// `KEY_CLUB_COMP_GERMAN_FIRST_DIVISION` (the box of ger_first.cpp (0x9cd414) at line 0x26e; not full: 0), the share
/// 0x30d40 and the TV money (League::pay_tv_rights, 0x4c4b40 to every club).
/// Original: `GerFirst::create_table` (0x5db230, ger_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ger_first_create_table(this: usize) -> u32 {
    let l = League(this);
    l.set_club_count(0x12);
    if l.year() == 0x7d1 {
        covered!("sbE_3 german regional divisions gathered");
        move_group_clubs_to_division(&[KEY_CLUB_COMP_GERMAN_REGIONAL_DIVISION_WEST_SOUTHWEST, KEY_CLUB_COMP_GERMAN_REGIONAL_DIVISION_EAST,
            KEY_CLUB_COMP_GERMAN_REGIONAL_DIVISION_NORTH, KEY_CLUB_COMP_GERMAN_REGIONAL_DIVISION_SOUTH], KEY_CLUB_COMP_GERMAN_REGIONAL); // sbE_3: the four old divisions
    }
    eng_prm_fill_league_table(l, 0x12, 0x9cd414, 0x26e, KEY_CLUB_COMP_GERMAN_FIRST_DIVISION, 0x30d40, || {
        eng_prm_pay_tv_rights(this, 0x4c4b40, 0); // sbE_3: the TV money of each club
    })
}

/// Comparer for `ger_first_create_table`: only the original runs, against the plan (`compare_nation_league_table`);
/// in 2001 no club may still be in one of the four old regional divisions.
pub(crate) unsafe extern "thiscall" fn compare_ger_first_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    let r = compare_nation_league_table("ger_first_create_table", League(this), 0x12, KEY_CLUB_COMP_GERMAN_FIRST_DIVISION, Some(0x30d40), || o(this));
    if League(this).year() == 0x7d1 {
        let old = [KEY_CLUB_COMP_GERMAN_REGIONAL_DIVISION_WEST_SOUTHWEST, KEY_CLUB_COMP_GERMAN_REGIONAL_DIVISION_EAST, KEY_CLUB_COMP_GERMAN_REGIONAL_DIVISION_NORTH, KEY_CLUB_COMP_GERMAN_REGIONAL_DIVISION_SOUTH].map(|k| read_u32(k));
        let left = (0..injury_club_count_without_nations()).map(|i| club_at(i as isize)).filter(|c| !c.division().is_null() && old.contains(&read_u32(c.division().addr()))).count();
        SCOREBOARD.record(left == 0, || format!("ger_first_create_table({:#x}): {} clubs left in the old regional divisions", this, left));
    }
    r
}

/// The Regionalliga clubs in database order (index up to `injury_club_count_without_nations`, read again at each
/// step), at most 36: those whose division (`club::DIVISION`) has the id of `KEY_CLUB_COMP_GERMAN_REGIONAL`; with the
/// number of them whose stadium (`club::STADIUM`) has a city (the stadium's +0x38, [inference] its city record).
pub(crate) unsafe fn german_regional_clubs() -> (Vec<usize>, i8) {
    let (mut v, mut located) = (Vec::new(), 0i8);
    let mut i = 0i32;
    while i < injury_club_count_without_nations() && v.len() < 0x24 {
        let c = club_at(i as isize);
        let d = c.division().addr();
        if d != 0 && read_u32(d) == read_u32(KEY_CLUB_COMP_GERMAN_REGIONAL) {
            v.push(c.addr());
            if club_city(c.addr()) != 0 {
                located = located.wrapping_add(1);
            }
        }
        i += 1;
    }
    (v, located)
}

/// The city of a club's stadium (`club::STADIUM`, then the stadium's +0x38), or 0 when either is missing.
pub(crate) unsafe fn club_city(club: usize) -> usize {
    let stadium = read_usize(club + club::STADIUM);
    if stadium == 0 { 0 } else { read_usize(stadium + 0x38) } // stadium +0x38: the city [inference]
}

/// Puts `club` in a section list of at most 18 (`list`); false when the list is full.
fn take_into_section(list: &mut Vec<usize>, club: usize) -> bool {
    if list.len() >= 0x12 {
        return false;
    }
    list.push(club);
    true
}

/// Rule: the 36 Regionalliga clubs are split into its North and South sections of 18 by the latitude of their city:
/// north of a line that moves south and south of a line that moves north until every club with a city is placed; the
/// clubs without one fill the sections by their old section, then wherever there is room; last, clubs that changed
/// section swap back in pairs (or rotate in threes) when that keeps both sections at 18.
///
/// `german_regional_clubs`; not exactly 36: the box of ger_first.cpp (0x9cd414) at line 0x419 and nothing else. The
/// lines start at 54.321 (north) and 47.55 (south): a club whose city latitude (`city::LATITUDE`) is above the north line
/// goes to the North list (while it has room), otherwise one below the south line to the South list; each pass moves the
/// north line down by 0.3 (`rdata::F64_0_3`) and the south line up by 0.278 until the placed clubs equal the clubs with
/// a city. Then two passes over the rest: the first by the old section (`club::RESERVE_DIVISION` the record of
/// `KEY_CLUB_COMP_GERMAN_REGIONAL_DIVISION_NORTH` for North, `_SOUTH` for South), the second wherever there is room.
/// The old sections of the South list then the North list (in that order, 36 places) are noted, and each club gets its
/// new section in `club::RESERVE_DIVISION`. Then, for each place k (0 to 34) whose club changed section, the first later
/// place j that changed the other way (its old section is k's new one and its new one is k's old one) and both take
/// their old section back; then, for each place i that changed, the first j ≠ i that changed with i's new section
/// equal to j's old one, and the first m ≠ i, j that changed with j's new section equal to m's old one and m's new
/// section equal to i's old one: the three take their old sections back. The object it is called on is not read.
/// Original: `ger_first_split_regional_north_south` (0x5dbbd0, ger_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ger_first_split_regional_north_south(_this: usize) {
    let north_section = comp_record_of(KEY_CLUB_COMP_GERMAN_REGIONAL_DIVISION_NORTH);
    let south_section = comp_record_of(KEY_CLUB_COMP_GERMAN_REGIONAL_DIVISION_SOUTH);
    let (mut pool, located) = german_regional_clubs();
    if pool.len() != 0x24 {
        return show_error_at(0x9cd414, 0x419);
    }
    covered!("sbE_3 german regional split");
    let (mut north, mut south) = (Vec::new(), Vec::new());
    let (mut north_line, mut south_line) = (read_f64(0x96b1a8), read_f64(0x96b1a0)); // 54.321 and 47.55
    loop {
        for slot in pool.iter_mut() {
            let city = if *slot == 0 { 0 } else { club_city(*slot) };
            if city == 0 {
                continue;
            }
            let latitude = read_f64(city + city::LATITUDE);
            if (latitude > north_line && take_into_section(&mut north, *slot)) || (latitude < south_line && take_into_section(&mut south, *slot)) {
                *slot = 0;
            }
        }
        if located as i32 == (north.len() + south.len()) as i32 {
            break;
        }
        north_line -= read_f64(rdata::F64_0_3);
        south_line += read_f64(0x96b198); // sbE_3: 0.278 a pass
    }
    for pass in 0..2 {
        for slot in pool.iter_mut() {
            if *slot == 0 {
                continue;
            }
            let old = read_usize(*slot + club::RESERVE_DIVISION);
            if ((pass == 1 || old == north_section) && take_into_section(&mut north, *slot)) || ((pass == 1 || old == south_section) && take_into_section(&mut south, *slot)) {
                *slot = 0;
            }
        }
    }
    let placed: Vec<usize> = south.iter().chain(north.iter()).copied().collect();
    let old: Vec<usize> = placed.iter().map(|&c| read_usize(c + club::RESERVE_DIVISION)).collect();
    for &c in &north {
        write_usize(c + club::RESERVE_DIVISION, north_section);
    }
    for &c in &south {
        write_usize(c + club::RESERVE_DIVISION, south_section);
    }
    let new = |k: usize| read_usize(placed[k] + club::RESERVE_DIVISION);
    let set = |k: usize, v: usize| write_usize(placed[k] + club::RESERVE_DIVISION, v);
    for k in 0..0x23 {
        if new(k) == old[k] {
            continue;
        }
        let new_k = new(k);
        if let Some(j) = (k + 1..0x24).find(|&j| new(j) != old[j] && old[j] == new_k && new(j) == old[k]) {
            set(k, old[k]); // sbE_3: the pair takes its old sections back
            set(j, old[j]);
        }
    }
    for i in 0..0x24 {
        if new(i) == old[i] {
            continue;
        }
        let found = (0..0x24).filter(|&j| j != i && new(j) != old[j]).find_map(|j| {
            (0..0x24).find(|&m| m != i && m != j && new(m) != old[m] && new(i) == old[j] && new(j) == old[m] && new(m) == old[i]).map(|m| (j, m))
        });
        if let Some((j, m)) = found {
            set(i, old[i]);
            set(j, old[j]);
            set(m, old[m]);
        }
    }
}

/// Comparer for `ger_first_split_regional_north_south`: side by side on the reserve division of the Regionalliga's
/// clubs (both run: it only reads the clubs and writes their sections); with other than 36 clubs, only the original
/// (the box).
pub(crate) unsafe extern "thiscall" fn compare_ger_first_split_regional_north_south(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    let (pool, _) = german_regional_clubs();
    if pool.len() != 0x24 {
        return o(this);
    }
    let regions: Vec<(usize, usize)> = pool.iter().map(|&c| (c + club::RESERVE_DIVISION, 4)).collect();
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&regions, || o(this), || ger_first_split_regional_north_south(this));
    SCOREBOARD.record(same, || format!("ger_first_split_regional_north_south: original={:x?} rust={:x?}", mo, mr));
}
