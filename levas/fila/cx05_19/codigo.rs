// ---------- cx05_19: the hosts of the Club World Championship and the Confederations Cup (host_country.cpp) ----------

/// Draws again the hosts of the yearly world competitions: up to 20 nations of league standard 13 or more that are
/// not England, not Spain and not of Oceania (`nation::CONTINENT` the record of [`KEY_CONTINENT_OCEANIA`]); none: the
/// box at line 0x34f and nothing more. Then each entry of the host table: the Club World Championship and the
/// Confederations Cup get a random candidate as their default host (`host_entry::HOST`, the nation's id); the World
/// Cup Oceania section swaps its host between Australia and New Zealand (Australia becomes New Zealand, anything else
/// Australia).
/// Original: `HostCountry::redraw_world_club_and_confed_hosts` (0x5fab70, host_country.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn redraw_world_club_and_confed_hosts(this: usize) {
    let mut candidates = [0usize; 20];
    let mut n = 0i8;
    let oceania = read_usize(G_CONTINENT_TABLE).wrapping_add(read_usize(KEY_CONTINENT_OCEANIA).wrapping_mul(0xc6));
    let mut i = 0i32;
    while i < read_i32(G_NATION_COUNT) && n < 0x14 {
        let nation = Nation(read_usize(G_NATION_TABLE) + i as usize * nation::SIZE);
        let id = read_u32(nation.addr());
        if read_i8(nation.addr() + nation::LEAGUE_STANDARD) >= 0xd // cx05_19: the strong leagues
            && id != read_u32(KEY_NATION_ENGLAND) && id != read_u32(KEY_NATION_SPAIN) && nation.continent() != oceania {
            candidates[n as usize] = nation.addr();
            n += 1;
        }
        i += 1;
    }
    if n < 1 {
        return show_error_at(0x9cf0d8, 0x34f);
    }
    covered!("cx05_19 world hosts redrawn");
    let (entries, count) = (read_usize(this), read_i16(this + 4) as i32);
    let mut k = 0i32;
    while k < count {
        let e = entries + k as usize * HOST_ENTRY_SIZE;
        let comp = read_u32(e + host_entry::COMP);
        if comp == read_u32(KEY_CLUB_COMP_FIFA_CLUB_WORLD_CHAMPIONSHIP) || comp == read_u32(KEY_CLUB_COMP_CONFEDERATIONS_CUP) {
            let r = exe::utils_random(n as i32) as usize;
            write_u32(e + host_entry::HOST, read_u32(candidates[r])); // cx05_19: a random strong nation
        } else if comp == read_u32(KEY_CLUB_COMP_WORLD_CUP_OCEANIA_QUALIFYING_SECTION) {
            let australia = read_u32(KEY_NATION_AUSTRALIA);
            let host = if read_u32(e + host_entry::HOST) == australia { read_u32(KEY_NATION_NEW_ZEALAND) } else { australia };
            write_u32(e + host_entry::HOST, host);
        }
        k += 1;
    }
}

/// Comparer for `redraw_world_club_and_confed_hosts`: it draws at random, so only the original runs; then the plan:
/// the Oceania section's host is Australia or New Zealand.
pub(crate) unsafe extern "thiscall" fn compare_redraw_world_club_and_confed_hosts(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    o(this);
    let (entries, count) = (read_usize(this), read_i16(this + 4).max(0) as usize);
    let section = read_u32(KEY_CLUB_COMP_WORLD_CUP_OCEANIA_QUALIFYING_SECTION);
    let pair = [read_u32(KEY_NATION_AUSTRALIA), read_u32(KEY_NATION_NEW_ZEALAND)];
    let ok = (0..count).map(|k| entries + k * HOST_ENTRY_SIZE).filter(|&e| read_u32(e + host_entry::COMP) == section).all(|e| pair.contains(&read_u32(e + host_entry::HOST)));
    SCOREBOARD.record(ok, || format!("redraw_world_club_and_confed_hosts({:#x}): the Oceania section host is not Australia or New Zealand", this));
}
