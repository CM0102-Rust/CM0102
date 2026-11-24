// ---------- cx06_6: the Confederations Cup's teams and predicted positions (fifa_confed.cpp) ----------

/// The big nations a continental place falls back on, in order: Brazil, Italy, Spain, Argentina, the United States,
/// Australia, France and Germany.
pub(crate) const CONFED_FALLBACK_NATIONS: [usize; 8] = [KEY_NATION_BRAZIL, KEY_NATION_ITALY, KEY_NATION_SPAIN, KEY_NATION_ARGENTINA,
    KEY_NATION_UNITED_STATES, KEY_NATION_AUSTRALIA, KEY_NATION_FRANCE, KEY_NATION_GERMANY];

/// The CONCACAF nations the North American place is drawn from: the United States, Canada, Mexico and Jamaica.
pub(crate) const CONFED_CONCACAF_NATIONS: [usize; 4] = [KEY_NATION_UNITED_STATES, KEY_NATION_CANADA, KEY_NATION_MEXICO, KEY_NATION_JAMAICA];

/// The predicted final positions of the 8 Confederations Cup nations, by reputation.
pub(crate) const CONFED_PREDICTED_POSITIONS: [u32; 8] = [1, 2, 3, 4, 5, 5, 5, 5];

/// A random club (`exe::utils_random` over the clubs that are not national teams, `G_CLUB_COUNT` − 2 ×
/// `G_NATION_COUNT`), the stand-in when a Confederations Cup place has no team.
unsafe fn confed_random_club() -> usize {
    club_at(exe::utils_random(read_i32(G_CLUB_COUNT).wrapping_sub(read_i32(G_NATION_COUNT).wrapping_mul(2))) as isize).addr()
}

/// Picks the nation of a continental place in the Confederations Cup, one not already in the first `count` of `taken`.
///
/// `comp` −1 is the CONCACAF place: before 2002 (the edition `year()`, signed) Canada when free; otherwise a nation drawn
/// at random (`exe::utils_random`(4)) from `CONFED_CONCACAF_NATIONS`, again until a free one comes. Another `comp` is
/// the competition id of a continental championship (`club_comp_record`): its latest winner
/// (`exe::comp_util_get_latest_placed_club` with 0), when taken its runner-up (1), and when that is taken its third
/// (2, returned even if taken: a defect of the original); with none, the first free national team of
/// `CONFED_FALLBACK_NATIONS`, or null.
/// Original: `FifaConfed::pick_continental_champion` (0x58ccb0, fifa_confed.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn fifa_confed_pick_continental_champion(this: usize, comp: i32, taken: usize, count: u32) -> usize {
    let count = count as u16 as i16;
    let is_taken = |team: usize| (0..count.max(0) as usize).any(|i| read_usize(taken + 4 * i) == team);
    let team_of = |key: usize| national_team_of_id(read_u32(key) as usize);
    if comp == -1 {
        if League(this).year() < 0x7d2 {
            let canada = team_of(KEY_NATION_CANADA);
            if !is_taken(canada) {
                return canada;
            }
        }
        loop {
            covered!("cx06_6 concacaf place drawn");
            let team = team_of(CONFED_CONCACAF_NATIONS[exe::utils_random(4) as usize]);
            if !is_taken(team) {
                return team;
            }
        }
    }
    let record = club_comp_record(comp as usize);
    covered!("cx06_6 continental champion");
    let mut team = exe::comp_util_get_latest_placed_club(record, 0);
    if is_taken(team) {
        team = exe::comp_util_get_latest_placed_club(record, 1);
    }
    if is_taken(team) {
        covered!("cx06_6 continental third");
        team = exe::comp_util_get_latest_placed_club(record, 2);
    }
    let mut k = 0;
    while team == 0 && k < CONFED_FALLBACK_NATIONS.len() {
        covered!("cx06_6 continental fallback");
        let t = team_of(CONFED_FALLBACK_NATIONS[k]);
        team = if is_taken(t) { 0 } else { t };
        k += 1;
    }
    team
}

/// Selects the 8 nations of the Confederations Cup into a new list (+0xee, count +0xf2 = 8; `exe::crt_malloc`(0x20);
/// out of memory: the box at line 0x35f and 0).
///
/// 0: the host's national team (HostCountry::find_hosts `exe::hostcountry_find_hosts` on `G_HOST_COUNTRY` with the
/// competition id, the edition `year()`, the two hosts and 1; no host: the box at line 0x366 and a random nation,
/// `exe::utils_random`(`G_NATION_COUNT`)). 1: the latest winner of this cup (`exe::comp_util_get_latest_placed_club`
/// with 0; none: the box at line 0x371 and `confed_random_club`), and when it is the host its runner-up (1; none: the
/// box at line 0x37c and a random club). 2 to 7: the continental places (`fifa_confed_pick_continental_champion` with
/// the list and 2 to 7 taken): the European Championship, CONCACAF (−1), the Copa América, the African Cup of Nations,
/// the Oceania Nations Cup and the Asian Cup. Returns 1.
/// Original: `FifaConfed::select_teams` (0x58c6b0, fifa_confed.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn fifa_confed_select_teams(this: usize) -> u32 {
    let l = League(this);
    write_u32(this + 0xf2, 8); // the entrant count
    let list = exe::crt_malloc(0x20);
    write_usize(this + 0xee, list); // the entrants
    if list == 0 {
        show_error_at(0x9c7474, 0x35f); // fifa_confed.cpp
        return 0;
    }
    let comp = l.competition().addr();
    let (mut first, mut second) = (0i32, 0i32);
    let host = if exe::hostcountry_find_hosts(read_usize(G_HOST_COUNTRY), read_usize(comp), l.year() as u16 as u32, &mut first, &mut second, 1) == 0 {
        show_error_at(0x9c7474, 0x366);
        exe::utils_random(read_i32(G_NATION_COUNT))
    } else {
        first
    };
    write_usize(list, national_team_of_id(host as usize));
    let mut holders = exe::comp_util_get_latest_placed_club(comp, 0);
    if holders == 0 {
        show_error_at(0x9c7474, 0x371);
        holders = confed_random_club();
    }
    write_usize(list + 4, holders);
    if read_usize(list) == read_usize(list + 4) {
        covered!("cx06_6 host holds the cup");
        let mut runner_up = exe::comp_util_get_latest_placed_club(comp, 1);
        if runner_up == 0 {
            show_error_at(0x9c7474, 0x37c);
            runner_up = confed_random_club();
        }
        write_usize(list + 4, runner_up);
    }
    covered!("cx06_6 confederations cup teams");
    let places = [(read_i32(KEY_CLUB_COMP_EUROPEAN_FOOTBALL_CHAMPIONSHIP), 2u32), (-1, 3), (read_i32(KEY_CLUB_COMP_COPA_AMERICA), 4),
        (read_i32(KEY_CLUB_COMP_AFRICAN_CUP_OF_NATIONS), 5), (read_i32(KEY_CLUB_COMP_OCEANIA_NATIONS_CUP), 6), (read_i32(KEY_CLUB_COMP_ASIAN_CUP), 7)];
    for (comp, slot) in places {
        let team = fifa_confed_pick_continental_champion(this, comp, read_usize(this + 0xee), slot);
        write_usize(read_usize(this + 0xee) + 4 * slot as usize, team);
    }
    1
}

/// Sets the predicted final positions of the Confederations Cup: without statistics, the box at line 0x39c. The 8
/// nations (+0xee, count +0xf2) sorted by reputation (`exe::crt_qsort` with 0x4b07d0), given
/// `CONFED_PREDICTED_POSITIONS` in that order (Comp_Stats set_club_final_position
/// `exe::comp_stats_set_club_final_position`), then shuffled (`exe::comp_util_shuffle_pointer_array` with the count's
/// word).
/// Original: `FifaConfed::set_predicted_positions` (0x58ca40, fifa_confed.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn fifa_confed_set_predicted_positions(this: usize) {
    let l = League(this);
    if l.comp_stats() == 0 {
        show_error_at(0x9c7474, 0x39c); // fifa_confed.cpp
        return;
    }
    exe::crt_qsort(read_usize(this + 0xee), read_u32(this + 0xf2), 4, 0x4b07d0usize);
    for (k, &position) in CONFED_PREDICTED_POSITIONS.iter().enumerate() {
        exe::comp_stats_set_club_final_position(l.comp_stats(), read_usize(read_usize(this + 0xee) + 4 * k), position);
    }
    covered!("cx06_6 confederations cup predicted");
    exe::comp_util_shuffle_pointer_array(read_usize(this + 0xee), read_u16(this + 0xf2) as u32);
}

/// Runs the original and the copy from the same random generator state (the copy without counting its draws) and tells
/// whether the generator ends in the same state; the generator is left as the original left it.
unsafe fn confed_side_by_side_rng<R>(original: impl FnOnce() -> R, copy: impl FnOnce() -> R) -> (R, R, bool) {
    let before = crate::comparison::MemorySnapshot::read(&crate::rng::STATE_REGIONS);
    let r = original();
    let after = crate::comparison::MemorySnapshot::read(&crate::rng::STATE_REGIONS);
    before.write_back();
    let c = crate::rng::pause_trace(|| crate::rng::without_counting(copy));
    let same = crate::comparison::MemorySnapshot::read(&crate::rng::STATE_REGIONS) == after;
    after.write_back();
    (r, c, same)
}

/// Comparer for `fifa_confed_pick_continental_champion`: side by side from the same generator state; the nation and
/// the generator after must match.
pub(crate) unsafe extern "thiscall" fn compare_fifa_confed_pick_continental_champion(this: usize, comp: i32, taken: usize, count: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32, usize, u32) -> usize>(%%I0%%);
    let (r, c, rng) = confed_side_by_side_rng(|| o(this, comp, taken, count), || fifa_confed_pick_continental_champion(this, comp, taken, count));
    SCOREBOARD.record(r == c && rng, || format!("fifa_confed_pick_continental_champion(comp {}, {} taken): original {:#x} rust {:#x}, generator equal {}", comp, count as u16, r, c, rng));
    r
}

/// Comparer for `fifa_confed_select_teams`: side by side from the same generator state; each side allocates its own
/// list (the copy's is compared with the original's and freed, and the original's put back); the result, the 8
/// nations and the generator after must match.
pub(crate) unsafe extern "thiscall" fn compare_fifa_confed_select_teams(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    let mut mine = 0usize;
    let (r, c, rng) = confed_side_by_side_rng(|| o(this), || {
        let theirs = read_usize(this + 0xee);
        let c = fifa_confed_select_teams(this);
        mine = read_usize(this + 0xee);
        write_usize(this + 0xee, theirs);
        c
    });
    let list = read_usize(this + 0xee);
    let teams = |l: usize| if l != 0 { (0..8).map(|k| read_usize(l + 4 * k)).collect::<Vec<usize>>() } else { Vec::new() };
    let (to, tr) = (teams(list), teams(mine));
    if mine != 0 && mine != list {
        exe::crt_free(mine);
    }
    SCOREBOARD.record(r == c && to == tr && rng, || format!("fifa_confed_select_teams({:#x}): original {:x?} rust {:x?}, generator equal {}", this, to, tr, rng));
    r
}

/// Comparer for `fifa_confed_set_predicted_positions`: side by side on the list of 8 (sorted and shuffled) from the same
/// generator state (both write the same positions); the list and the generator after must match, and by reputation the
/// positions make `CONFED_PREDICTED_POSITIONS` (`predicted_by_reputation_check`).
pub(crate) unsafe extern "thiscall" fn compare_fifa_confed_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    let list = read_usize(this + 0xee);
    if list == 0 || read_u32(this + 0xf2) != 8 || League(this).comp_stats() == 0 {
        return o(this);
    }
    let clubs: Vec<usize> = (0..8).map(|k| read_usize(list + 4 * k)).collect();
    let regions = [(list, 0x20)];
    let mut snapshots = None;
    let (_, _, rng) = confed_side_by_side_rng(|| o(this), || {
        let after = crate::comparison::MemorySnapshot::read(&regions);
        for (k, &c) in clubs.iter().enumerate() {
            write_usize(list + 4 * k, c);
        }
        fifa_confed_set_predicted_positions(this);
        snapshots = Some((after.clone(), crate::comparison::MemorySnapshot::read(&regions)));
        after.write_back();
    });
    let (mo, mr) = snapshots.unwrap();
    SCOREBOARD.record(mo == mr && rng, || format!("fifa_confed_set_predicted_positions({:#x}): {}, generator equal {}", this, first_differences(&mo, &mr).join(", "), rng));
    predicted_by_reputation_check("fifa_confed_set_predicted_positions", this, &clubs, &CONFED_PREDICTED_POSITIONS);
}
