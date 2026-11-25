// ---------- batch %%LEVA%%: the Club World Championship's entrants (world_club_champ.cpp) ----------

/// The Club World Championship's drawn entrant list (+0xee, 12 club pointers) and its size (+0xf2), as
/// `world_club_champ_update_for_date` frees them after the draw.
pub(crate) const WORLD_CLUB_CHAMP_ENTRANTS: usize = 0xee;
pub(crate) const WORLD_CLUB_CHAMP_ENTRANT_COUNT: usize = 0xf2;

/// `WorldClubChamp::pick_entrant(continent, nation, list, slot, club comp)` (0x92ae60, thiscall, ret 0x14).
pub(crate) const FN_WORLD_CLUB_CHAMP_PICK_ENTRANT: usize = 0x92ae60;

/// The continent key of each of the first 11 entrants: two from Africa, two from Asia, two from Europe (the second
/// one passing the UEFA Cup), two from North America, one from Oceania and two from South America.
pub(crate) const WORLD_CLUB_CHAMP_CONTINENTS: [usize; 11] = [
    KEY_CONTINENT_AFRICA, KEY_CONTINENT_AFRICA, KEY_CONTINENT_ASIA, KEY_CONTINENT_ASIA, KEY_CONTINENT_EUROPE,
    KEY_CONTINENT_EUROPE, KEY_CONTINENT_NORTH_AMERICA, KEY_CONTINENT_NORTH_AMERICA, KEY_CONTINENT_OCEANIA,
    KEY_CONTINENT_SOUTH_AMERICA, KEY_CONTINENT_SOUTH_AMERICA,
];

/// Rule: the Club World Championship has 12 entrants: two clubs each from Africa, Asia, Europe, North America and
/// South America, one from Oceania and one from the host nation; Europe's second place goes through the UEFA Cup.
///
/// `WorldClubChamp::pick_entrants()`: the size (`WORLD_CLUB_CHAMP_ENTRANT_COUNT`) = 12 and the list
/// (`WORLD_CLUB_CHAMP_ENTRANTS`, `malloc(0x30)`; none: the world_club_champ.cpp box at line 0x3fb and 0). The host
/// nation of this edition (`HostCountry::find_hosts` on `G_HOST_COUNTRY` with the competition's id and the year; none:
/// the box at line 0x402 and a random nation, `utils_random(G_NATION_COUNT)`). Slots 0..10 by continent
/// (`WORLD_CLUB_CHAMP_CONTINENTS`; `pick_entrant(continent, -1, list, slot, -1)`, slot 5 with the UEFA Cup's id
/// instead of -1) and slot 11 the host's (`pick_entrant(-1, host, list, 11, -1)`), each answer stored in its slot.
/// Returns 1.
/// Original: `WorldClubChamp::pick_entrants` (0x92a8b0, world_club_champ.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn world_club_champ_pick_entrants(this: usize) -> u32 {
    write_u32(this + WORLD_CLUB_CHAMP_ENTRANT_COUNT, 0xc);
    let list = exe::crt_malloc(0x30);
    write_usize(this + WORLD_CLUB_CHAMP_ENTRANTS, list);
    if list == 0 {
        show_error_at(0xad5764, 0x3fb);
        return 0;
    }
    let l = League(this);
    let (mut host, mut second) = (0i32, 0i32);
    if exe::hostcountry_find_hosts(read_usize(G_HOST_COUNTRY), read_i32(read_usize(this + league::COMPETITION)) as usize, l.year() as u16 as u32, &mut host, &mut second, 1) == 0 {
        show_error_at(0xad5764, 0x402);
        host = exe::utils_random(read_i32(G_NATION_COUNT));
    }
    covered!("rxf_6 world club championship entrants");
    let pick: unsafe extern "thiscall" fn(usize, i32, i32, usize, u32, i32) -> usize = core::mem::transmute(FN_WORLD_CLUB_CHAMP_PICK_ENTRANT);
    for (slot, &key) in WORLD_CLUB_CHAMP_CONTINENTS.iter().enumerate() {
        let comp = if slot == 5 { read_i32(KEY_CLUB_COMP_UEFA_CUP) } else { -1 }; // rxf_6: Europe's second through the UEFA Cup
        let club = pick(this, read_i32(key), -1, read_usize(this + WORLD_CLUB_CHAMP_ENTRANTS), slot as u32, comp);
        write_usize(read_usize(this + WORLD_CLUB_CHAMP_ENTRANTS) + 4 * slot, club);
    }
    let club = pick(this, -1, host, read_usize(this + WORLD_CLUB_CHAMP_ENTRANTS), 0xb, -1);
    write_usize(read_usize(this + WORLD_CLUB_CHAMP_ENTRANTS) + 4 * 0xb, club);
    1
}

/// Comparer for `world_club_champ_pick_entrants`: it allocates and draws, so only the original runs, with the plan: on 1
/// the size is 12 and the list exists.
pub(crate) unsafe extern "thiscall" fn compare_world_club_champ_pick_entrants(this: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%)(this);
    if r == 1 {
        let ok = read_u32(this + WORLD_CLUB_CHAMP_ENTRANT_COUNT) == 0xc && read_usize(this + WORLD_CLUB_CHAMP_ENTRANTS) != 0;
        SCOREBOARD.record(ok, || format!("world_club_champ_pick_entrants({:#x}): size {} list {:#x}", this, read_u32(this + WORLD_CLUB_CHAMP_ENTRANT_COUNT), read_usize(this + WORLD_CLUB_CHAMP_ENTRANTS)));
    }
    r
}
