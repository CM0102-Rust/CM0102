// ---------- cx05_2: the Asian Cup draw: the twelve finalists, the predicted positions and group A (asia_nations.cpp) ----------

/// The source file of the Asian Cup's error boxes.
const ASIA_NATIONS_FILE: usize = 0x987ba0;

/// The predicted final places of the twelve Asian Cup finalists, by reputation.
pub(crate) const ASIA_NATIONS_POSITIONS: [u32; 12] = [1, 2, 3, 3, 5, 5, 5, 5, 9, 9, 0xb, 0xb];

/// The national team of nation `index` of the nation table (`exe::database_get_national_team`, cdecl).
unsafe fn national_team_of_nation_index(index: i32) -> usize {
    exe::database_get_national_team(read_usize(G_NATION_TABLE).wrapping_add((index as usize).wrapping_mul(nation::SIZE)))
}

/// Picks the twelve Asian Cup finalists and draws them into three groups of four.
///
/// `.drawn_count` (+0xf2, word) = 12 and the list +0xee (`exe::crt_malloc`(0x30); out of memory: the box at line 0x35b
/// and 0). The hosts of the edition first (HostCountry::find_hosts on [`G_HOST_COUNTRY`]: the competition id, the year
/// `.year()` as a word, the two host indexes, default 1; thiscall, ret 0x14), each one's national team in the next slot.
/// Then every Asian nation (`nation::CONTINENT` equal to [`KEY_CONTINENT_ASIA`]) that is not one of the hosts (the host
/// teams' `club::NATION`), up to 100, by its national team; fewer candidates than 12: 0 (the list stays). The
/// candidates are sorted and the best `12 - hosts` shuffled (`exe::comp_util_sort_and_shuffle_top_clubs`, cdecl) and
/// fill the list after the hosts; the list is sorted by reputation (qsort with 0x4b07d0) into four pots of three. A new
/// list (`exe::crt_malloc`(0x30); out of memory: the box at line 0x39a and 0) takes the groups: group g (0 to 2) gets,
/// from each pot in turn, a random team still in it (`exe::utils_random`(3) until the slot is not empty; the slot is
/// then emptied). The old list is freed and the new one goes to +0xee. Returns 1.
/// Original: `AsiaNations::select_and_draw_teams` (0x40fdf0, asia_nations.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn asia_nations_select_and_draw_teams(this: usize) -> u32 {
    let league = League(this);
    write_i16(this + 0xf2, 12); // the number of finalists
    let list = exe::crt_malloc(0x30);
    write_usize(this + 0xee, list);
    if list == 0 {
        show_error_at(ASIA_NATIONS_FILE, 0x35b);
        return 0;
    }
    // simplification: the original leaves the two host indexes uninitialised and reads only the ones find_hosts wrote
    let mut hosts = [-1i32; 2];
    let (a, b) = hosts.split_at_mut(1);
    let n = exe::hostcountry_find_hosts(read_usize(G_HOST_COUNTRY), read_u32(league.competition().addr()) as usize, league.year() as u16 as u32, &mut a[0], &mut b[0], 1) as i8;
    let mut filled = 0usize;
    for &h in hosts.iter().take(n.max(0) as usize) {
        write_usize(list + filled * 4, national_team_of_nation_index(h));
        filled += 1;
    }
    let mut candidates = [0usize; 100];
    let mut count = 0i16;
    let asia = read_usize(KEY_CONTINENT_ASIA);
    let mut i = 0i16;
    while (i as i32) < read_i32(G_NATION_COUNT) {
        let nation = read_usize(G_NATION_TABLE).wrapping_add((i as isize as usize).wrapping_mul(nation::SIZE));
        let continent = Nation(nation).continent();
        if continent != 0 && read_usize(continent) == asia && count < 100 {
            let is_host = (0..n.max(0) as usize).any(|k| Club(read_usize(list + k * 4)).nation().addr() == nation);
            if !is_host {
                candidates[count as usize] = exe::database_get_national_team(nation);
                count += 1;
            }
        }
        i = i.wrapping_add(1);
    }
    if count < read_i16(this + 0xf2) {
        return 0;
    }
    covered!("cx05_2 asian cup finalists");
    let places = (read_u8(this + 0xf2).wrapping_sub(n as u8)) as i8 as i16; // cx05_2: the places after the hosts
    exe::comp_util_sort_and_shuffle_top_clubs(candidates.as_mut_ptr() as usize, count as i32 as u32, places as u16 as u32);
    let mut c = 0i16;
    while (c as i32) < read_i16(this + 0xf2) as i32 - n as i32 {
        write_usize(list + filled * 4, candidates[c as usize]);
        filled += 1;
        c += 1;
    }
    exe::crt_qsort(list, filled as u32, 4, 0x4b07d0); // compare_clubs_by_reputation_desc: the four pots of three
    let groups = exe::crt_malloc(0x30);
    if groups == 0 {
        show_error_at(ASIA_NATIONS_FILE, 0x39a);
        return 0;
    }
    for g in 0..3usize {
        for pot in 0..4usize {
            let first = pot * 3; // cx05_2: the pot of the group's slot
            let mut r = exe::utils_random(3) as i8 as i32;
            while read_usize(list + (r as usize + first) * 4) == 0 {
                r = exe::utils_random(3) as i8 as i32;
            }
            let slot = list + (r as usize + first) * 4;
            write_usize(groups + (g * 4 + pot) * 4, read_usize(slot));
            write_usize(slot, 0);
        }
    }
    exe::crt_free(read_usize(this + 0xee));
    write_usize(this + 0xee, groups);
    1
}

/// Comparer for `asia_nations_select_and_draw_teams`: it draws at random, so only the original runs; then, when it
/// returns 1, the plan: 12 finalists, all distinct and none null.
pub(crate) unsafe extern "thiscall" fn compare_asia_nations_select_and_draw_teams(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    let r = o(this);
    if r == 1 {
        let list = read_usize(this + 0xee);
        let mut teams: Vec<usize> = (0..12).map(|k| read_usize(list + k * 4)).collect();
        teams.sort_unstable();
        teams.dedup();
        let ok = read_i16(this + 0xf2) == 12 && teams.len() == 12 && teams[0] != 0;
        SCOREBOARD.record(ok, || format!("asia_nations_select_and_draw_teams({:#x}): {} distinct finalists, +0xf2 {}", this, teams.len(), read_i16(this + 0xf2)));
    }
    r
}

/// The predicted positions of the twelve Asian Cup finalists (+0xee, the count at +0xf2 as a word) by reputation
/// (`predicted_by_reputation_of_copy`): 1, 2, 3, 3, four 5ths, two 9ths and two 11ths; the boxes at lines 0x40f (no
/// statistics) and 0x417 (out of memory).
/// Original: `AsiaNations::set_predicted_positions` (0x4103a0, asia_nations.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn asia_nations_set_predicted_positions(this: usize) {
    covered!("cx05_2 asian cup predicted");
    predicted_by_reputation_of_copy(this, read_usize(this + 0xee), read_i16(this + 0xf2) as i32, &ASIA_NATIONS_POSITIONS, ASIA_NATIONS_FILE, (0x40f, 0x417))
}

/// Comparer for `asia_nations_set_predicted_positions`: only the original (the statistics add the entries), then each
/// finalist's position in the statistics by reputation (`predicted_by_reputation_check`).
pub(crate) unsafe extern "thiscall" fn compare_asia_nations_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    let (list, n) = (read_usize(this + 0xee), read_i16(this + 0xf2));
    let teams: Vec<usize> = if list != 0 && (0..=64).contains(&n) { (0..n as usize).map(|k| read_usize(list + k * 4)).collect() } else { Vec::new() };
    o(this);
    predicted_by_reputation_check("asia_nations_set_predicted_positions", this, &teams, &ASIA_NATIONS_POSITIONS);
}

/// Group A of the Asian Cup in the cup's own table: 4 teams (`.set_club_count()`), the table (`exe::crt_malloc`(0xec):
/// four rows of `league_table_row::SIZE`; out of memory: the box at line 0x1e4 and 0), row i with drawn team i (the
/// list +0xee holds the groups one after the other) (`exe::league_init_table_row`(this, i, team, 0, 0)), the group draw
/// news (`exe::league_send_group_draw_news`) and 1.
/// Original: `AsiaNations::create_group_table` (0x40f3d0, asia_nations.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn asia_nations_create_group_table(this: usize) -> u32 {
    let league = League(this);
    league.set_club_count(4);
    let t = exe::crt_malloc(0xec);
    league.set_table(t);
    if t == 0 {
        show_error_at(ASIA_NATIONS_FILE, 0x1e4);
        return 0;
    }
    covered!("cx05_2 asian cup group a");
    let mut i = 0i8;
    while (i as i16) < league.club_count() {
        // simplification: the index goes in a dword whose 3 high bytes are stack leftovers; 0x687430 reads only the byte (char)
        exe::league_init_table_row(league, i as u8 as u32, read_usize(read_usize(this + 0xee) + i as isize as usize * 4), 0, 0); // cx05_2: group A's team in the row
        i = i.wrapping_add(1);
    }
    exe::league_send_group_draw_news(league);
    1
}

/// Comparer for `asia_nations_create_group_table`: only the original (the table and the rows allocate, the news goes
/// out), then the plan: 1, four teams, row i with drawn team i at position i.
pub(crate) unsafe extern "thiscall" fn compare_asia_nations_create_group_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    let list = read_usize(this + 0xee);
    if list == 0 {
        return o(this);
    }
    let expected: [usize; 4] = core::array::from_fn(|i| read_usize(list + i * 4));
    let r = o(this);
    let league = League(this);
    let t = league.table();
    let mut read = [(0usize, 0u8); 4];
    if t != 0 {
        for (i, l) in read.iter_mut().enumerate() {
            *l = (read_usize(t + i * league_table_row::SIZE), read_u8(t + i * league_table_row::SIZE + league_table_row::POSITION));
        }
    }
    let same = r == 1 && league.club_count() == 4 && t != 0 && (0..4).all(|i| read[i] == (expected[i], i as u8));
    SCOREBOARD.record(same, || format!("asia_nations_create_group_table({:#x}) returned {} rows {:x?} expected {:x?}", this, r, read, expected));
    r
}
