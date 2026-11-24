// ---------- cx05_7: the European Championship seeding and group draw (euro_champ.cpp) ----------

/// The source file of the European Championship's error boxes.
const EURO_CHAMP_FILE: usize = 0x9c4638;

/// The seeded list of the European Championship: 16 entries of 5 bytes inside the object (the national team's id,
/// then its pot from 0 or, in a drawn copy, its group from 1).
const EURO_CHAMP_SEEDED: usize = 0xee;

/// `EuroChamp::draw_groups` (thiscall, ret 0): the 16-entry draw list, called by the group draw through its detour.
const FN_EUROCHAMP_DRAW_GROUPS: usize = 0x57c320;
/// `EuroChamp::create_group_table(list)` (thiscall, ret 4).
const FN_EUROCHAMP_CREATE_GROUP_TABLE: usize = 0x57b530;
/// `EuroChamp::create_group_stage(g, list)` (thiscall, ret 8).
const FN_EUROCHAMP_CREATE_GROUP_STAGE: usize = 0x57b910;

/// Swaps the two 6-byte entrant entries `a` and `b` (the team pointer and the status word), as the original's moves.
unsafe fn swap_entrants(a: usize, b: usize) {
    let (team, word) = (read_u32(a), read_u16(a + 4));
    write_u32(a, read_u32(b));
    write_u16(a + 4, read_u16(b + 4));
    write_u32(b, team);
    write_u16(b + 4, word);
}

/// The national team of the nation of index `index` of the nation table, or null for −1 (no host).
unsafe fn host_national_team(index: i32) -> usize {
    if index == -1 {
        return 0;
    }
    exe::database_get_national_team(read_usize(G_NATION_TABLE).wrapping_add((index as usize).wrapping_mul(nation::SIZE)))
}

/// Seeds the European Championship's entrants into four pots of four.
///
/// A copy of the entrant book (`exe::crt_malloc`(6 × `.entrant_count()`); out of memory: the box at line 0x462 and
/// nothing more) sorted by reputation (qsort with 0x4b07b0, 6-byte entries). The hosts of the finals year
/// (HostCountry::find_hosts on [`G_HOST_COUNTRY`], default 1) and the holder (`exe::comp_util_get_latest_placed_club`
/// of the competition record, 0): over the 16 entries of the copy, the first host is swapped into entry 0 (at least 2
/// fixed seeds), the second host into entry 1 (at least 3) and the holder into entry 2 (at least 1). Then 2 fixed
/// seeds when the second host is the first host or the holder, and 1 when the first host is the holder; the entries
/// after the fixed ones are sorted again by reputation. Each entry k goes to the seeded list (`EURO_CHAMP_SEEDED` + 5k:
/// the team's id, then pot k / 4); the copy is freed.
/// Original: `EuroChamp::seed_teams` (0x57c710, euro_champ.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn euro_champ_seed_teams(this: usize) {
    let league = League(this);
    let count = league.entrant_count() as i32;
    let copy = exe::crt_malloc((count * 6) as u32);
    if copy == 0 {
        return show_error_at(EURO_CHAMP_FILE, 0x462);
    }
    core::ptr::copy_nonoverlapping(league.entrant_book() as *const u8, copy as *mut u8, (count * 6) as usize);
    exe::crt_qsort(copy, count as u32, 6, 0x4b07b0); // compare_clubs_by_reputation_desc
    // simplification: the original leaves the two host indexes uninitialised (find_hosts writes both when it finds the
    // edition); the copy starts them at −1 (no host)
    let mut hosts = [-1i32; 2];
    let (a, b) = hosts.split_at_mut(1);
    exe::hostcountry_find_hosts(read_usize(G_HOST_COUNTRY), read_usize(KEY_CLUB_COMP_EUROPEAN_FOOTBALL_CHAMPIONSHIP), league.year() as u16 as u32, &mut a[0], &mut b[0], 1);
    let (first_host, second_host) = (host_national_team(hosts[0]), host_national_team(hosts[1]));
    let holder = exe::comp_util_get_latest_placed_club(comp_record_of_key(KEY_CLUB_COMP_EUROPEAN_FOOTBALL_CHAMPIONSHIP), 0);
    covered!("cx05_7 euro seeding");
    let mut fixed = 0i8;
    for k in 0..16usize {
        let e = copy + k * 6;
        if read_usize(e) == first_host {
            swap_entrants(e, copy);
            fixed = fixed.max(2);
        }
        if read_usize(e) == second_host {
            swap_entrants(e, copy + 6);
            fixed = fixed.max(3);
        }
        if read_usize(e) == holder {
            swap_entrants(e, copy + 0xc); // cx05_7: the holder in the third seed
            fixed = fixed.max(1);
        }
    }
    if second_host == first_host || second_host == holder {
        fixed = 2;
    }
    if first_host == holder {
        fixed = 1;
    }
    exe::crt_qsort(copy + fixed as usize * 6, (count - fixed as i32) as u32, 6, 0x4b07b0);
    let mut k = 0i16;
    while k < league.entrant_count() {
        let seeded = this + EURO_CHAMP_SEEDED + k as usize * 5;
        write_u32(seeded, read_u32(read_usize(copy + k as usize * 6))); // the team's id
        write_u8(seeded + 4, (k / 4) as u8); // cx05_7: the pot of four
        k += 1;
    }
    exe::crt_free(copy);
}

/// Comparer for `euro_champ_seed_teams`: side by side on the seeded list (nothing is drawn at random; the copy and the
/// sorts are the same).
pub(crate) unsafe extern "thiscall" fn compare_euro_champ_seed_teams(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&[(this + EURO_CHAMP_SEEDED, 0x50)], || o(this), || euro_champ_seed_teams(this));
    SCOREBOARD.record(same, || format!("euro_champ_seed_teams({:#x}) original={:x?} rust={:x?}", this, mo, mr));
}

/// Draws the European Championship groups: a list of 16 entries of 5 bytes (`exe::crt_calloc`(16, 5); out of memory:
/// the box at line 0x3be and null): the four top seeds (seeded entries 0 to 3) in groups 1 to 4, then each of the
/// other twelve seeded entries k, in order, in a random group (`exe::utils_random`(4) until the group holds fewer than
/// k / 4 + 1 teams). Returns the list.
/// Original: `EuroChamp::draw_groups` (0x57c320, euro_champ.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn euro_champ_draw_groups(this: usize) -> usize {
    let mut counts = [1i8; 4];
    let list = exe::crt_calloc(0x10, 5);
    if list == 0 {
        show_error_at(EURO_CHAMP_FILE, 0x3be);
        return 0;
    }
    for k in 0..4usize {
        write_u32(list + k * 5, read_u32(this + EURO_CHAMP_SEEDED + k * 5));
        write_u8(list + k * 5 + 4, k as u8 + 1);
    }
    covered!("cx05_7 euro group draw");
    for k in 4..16usize {
        let level = (k / 4) as i8 + 1; // cx05_7: the pot fills one team per group
        let mut group = exe::utils_random(4) as i16;
        while counts[group as usize] >= level {
            group = exe::utils_random(4) as i16;
        }
        write_u32(list + k * 5, read_u32(this + EURO_CHAMP_SEEDED + k * 5));
        write_u8(list + k * 5 + 4, (group as u8).wrapping_add(1));
        counts[group as usize] += 1;
    }
    list
}

/// Comparer for `euro_champ_draw_groups`: it draws at random, so only the original runs; then the plan: the four top
/// seeds in groups 1 to 4 and four teams in each group.
pub(crate) unsafe extern "thiscall" fn compare_euro_champ_draw_groups(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I1%%);
    let list = o(this);
    if list != 0 {
        let mut per_group = [0u8; 5];
        for k in 0..16usize {
            per_group[(read_u8(list + k * 5 + 4) as usize).min(4)] += 1;
        }
        let seeds = (0..4usize).all(|k| read_u8(list + k * 5 + 4) == k as u8 + 1 && read_u32(list + k * 5) == read_u32(this + EURO_CHAMP_SEEDED + k * 5));
        let ok = seeds && per_group[1..].iter().all(|&n| n == 4);
        SCOREBOARD.record(ok, || format!("euro_champ_draw_groups({:#x}): teams per group {:?}, seeds {}", this, &per_group[1..], seeds));
    }
    list
}

/// The European Championship group draw (on the draw date, from the day): the draw list (`EuroChamp::draw_groups`),
/// group 1 in the cup's own table (`EuroChamp::create_group_table`(list); 0: the box at line 0x37a and nothing more),
/// the fixtures (`exe::league_create_fixtures`; 0: the box at line 0x380), `.set_active(1)`, the sorted table
/// (`exe::league_sort_table`(this, 0); 0: the box at line 0x39b), groups 2 to 4 as stages (`EuroChamp::
/// create_group_stage`(g, list), g = 0 to 2), the predicted positions (virtual `comp_vtable::SET_PREDICTED_POSITIONS`),
/// the round table `.round_dates()` freed (not zeroed: the rules write it again), the list freed and the rules set up
/// again (virtual `comp_vtable::SET_UP_RULES`, the result not read). On the early returns the list stays allocated.
/// Original: `EuroChamp::run_group_draw` (0x57c1d0, euro_champ.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn euro_champ_run_group_draw(this: usize) {
    let league = League(this);
    let draw: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_EUROCHAMP_DRAW_GROUPS);
    let group_table: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_EUROCHAMP_CREATE_GROUP_TABLE);
    let group_stage: unsafe extern "thiscall" fn(usize, i32, usize) -> u32 = core::mem::transmute(FN_EUROCHAMP_CREATE_GROUP_STAGE);
    let list = draw(this);
    if group_table(this, list) == 0 {
        return show_error_at(EURO_CHAMP_FILE, 0x37a);
    }
    if exe::league_create_fixtures(league) == 0 {
        return show_error_at(EURO_CHAMP_FILE, 0x380);
    }
    league.set_active(1);
    if exe::league_sort_table(league, 0) == 0 {
        return show_error_at(EURO_CHAMP_FILE, 0x39b);
    }
    covered!("cx05_7 euro groups built");
    for g in 0..3i32 { // cx05_7: the three groups after the first
        group_stage(this, g, list);
    }
    let predicted: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::SET_PREDICTED_POSITIONS));
    predicted(this);
    exe::crt_free(league.round_dates());
    exe::crt_free(list);
    let rules: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::SET_UP_RULES));
    rules(this);
}

/// Comparer for `euro_champ_run_group_draw`: only the original (it draws, allocates and sends the news); then the
/// plan: active, the group 1 table and the three group stages.
pub(crate) unsafe extern "thiscall" fn compare_euro_champ_run_group_draw(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    o(this);
    let league = League(this);
    let ok = league.active() == 1 && league.table() != 0 && league.stages() != 0 && (0..3).all(|g| read_usize(league.stages() + g * 4) != 0);
    SCOREBOARD.record(ok, || format!("euro_champ_run_group_draw({:#x}): +0x45 {}, table {:#x}", this, league.active(), league.table()));
}
