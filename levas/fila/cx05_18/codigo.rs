// ---------- cx05_18: the European club finals' hosts and the order of the host table (host_country.cpp) ----------

/// The entries of the host table (`this`: +0 the entries of `HOST_ENTRY_SIZE`, +4 their count, a signed word).
unsafe fn host_entries(this: usize) -> impl Iterator<Item = usize> {
    let (entries, count) = (read_usize(this), read_i16(this + 4).max(0) as usize);
    (0..count).map(move |k| entries + k * HOST_ENTRY_SIZE)
}

/// Picks the hosts of the three European club finals: up to 50 European nations (`nation::CONTINENT` equal to
/// [`KEY_CONTINENT_EUROPE`]) with state of development 1, league standard above 7 and a national stadium of more than
/// 40000 seats (`stadium::CAPACITY`), each once; fewer than 3: the box at line 0x3b5 and nothing more. The candidates
/// are shuffled (each position k swapped with a random one, `exe::utils_random`(n)) and the first three go, in table
/// order, as the default host (`host_entry::HOST`) of the entries of the Champions League, the UEFA Cup and the Cup
/// Winners' Cup.
/// Original: `HostCountry::pick_european_final_hosts` (0x5fad20, host_country.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn pick_european_final_hosts(this: usize) {
    let mut candidates = [0usize; 50];
    let mut n = 0i8;
    let europe = read_usize(KEY_CONTINENT_EUROPE);
    let mut i = 0i32;
    while i < read_i32(G_NATION_COUNT) && n < 50 {
        let nation = Nation(read_usize(G_NATION_TABLE) + i as usize * nation::SIZE);
        let continent = nation.continent();
        let stadium = read_usize(nation.addr() + nation::NATIONAL_STADIUM);
        if continent != 0 && read_usize(continent) == europe && read_i8(nation.addr() + nation::STATE_OF_DEVELOPMENT) == 1
            && read_i8(nation.addr() + nation::LEAGUE_STANDARD) > 7 && stadium != 0 && read_i32(stadium + stadium::CAPACITY) > 40000 // cx05_18: a big enough stadium
            && !candidates[..n as usize].contains(&nation.addr()) {
            candidates[n as usize] = nation.addr();
            n += 1;
        }
        i += 1;
    }
    if n < 3 {
        return show_error_at(0x9cf0d8, 0x3b5);
    }
    covered!("cx05_18 european finals hosts");
    for k in 0..n as usize {
        let r = exe::utils_random(n as i32) as i8 as isize as usize;
        candidates.swap(k, r);
    }
    let finals = [KEY_CLUB_COMP_UEFA_EUROPEAN_CHAMPIONS_LEAGUE, KEY_CLUB_COMP_UEFA_CUP, KEY_CLUB_COMP_EUROPEAN_CUP_WINNERS_CUP];
    let mut next = 0usize;
    for e in host_entries(this) {
        if next < 3 && finals.iter().any(|&key| read_u32(e + host_entry::COMP) == read_u32(key)) {
            write_u32(e + host_entry::HOST, read_u32(candidates[next])); // cx05_18: the nation's id as the host
            next += 1;
        }
    }
}

/// Comparer for `pick_european_final_hosts`: it draws at random (the hosts and the generator cannot be compared twice),
/// so only the original runs; then the plan: the three finals' hosts are distinct European nations.
pub(crate) unsafe extern "thiscall" fn compare_pick_european_final_hosts(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    o(this);
    let finals = [KEY_CLUB_COMP_UEFA_EUROPEAN_CHAMPIONS_LEAGUE, KEY_CLUB_COMP_UEFA_CUP, KEY_CLUB_COMP_EUROPEAN_CUP_WINNERS_CUP];
    let hosts: Vec<u32> = host_entries(this).filter(|&e| finals.iter().any(|&key| read_u32(e + host_entry::COMP) == read_u32(key))).take(3).map(|e| read_u32(e + host_entry::HOST)).collect();
    let europe = read_usize(KEY_CONTINENT_EUROPE);
    let european = hosts.iter().all(|&h| {
        let c = Nation(read_usize(G_NATION_TABLE) + h as usize * nation::SIZE).continent();
        c != 0 && read_usize(c) == europe
    });
    let distinct = hosts.len() < 2 || (hosts[0] != hosts[1] && hosts.get(2).map_or(true, |&h| h != hosts[0] && h != hosts[1]));
    SCOREBOARD.record(european && distinct, || format!("pick_european_final_hosts({:#x}): hosts {:?}", this, hosts));
}

/// The order of the host table (qsort and bsearch): by year (`host_entry::YEAR`, unsigned word), then by decision day
/// (`host_entry::DECISION_DAY`, unsigned word); the difference.
/// Original: `host_country_compare_entries` (0x5fb3f0, host_country.cpp, cdecl).
pub(crate) unsafe extern "C" fn host_country_compare_entries(a: usize, b: usize) -> i32 {
    let (ya, yb) = (read_u16(a + host_entry::YEAR), read_u16(b + host_entry::YEAR));
    if ya != yb {
        return ya as i32 - yb as i32; // cx05_18: the year first
    }
    read_u16(a + host_entry::DECISION_DAY) as i32 - read_u16(b + host_entry::DECISION_DAY) as i32
}

/// Comparer for `host_country_compare_entries`: both run (pure) and return the same.
pub(crate) unsafe extern "C" fn compare_host_country_compare_entries(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(%%I1%%)(a, b);
    let n = host_country_compare_entries(a, b);
    SCOREBOARD.record(r == n, || format!("host_country_compare_entries({:#x}, {:#x}) original={} rust={}", a, b, r, n));
    r
}
