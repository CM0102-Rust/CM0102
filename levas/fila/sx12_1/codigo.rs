// ---------- batch %%LEVA%%: the player's current injury and two key nation table counts (injury.cpp, key_nation.cpp; sx12_1) ----------

/// Returns the address of a person's current injury entry, or 0 when he has none.
///
/// `Injury::get_player_injury(person)`: a null person shows the "Error" box (injury.cpp line 0xe3a) and returns 0; a
/// person id at or above `G_STAFF_COUNT` returns 0. Otherwise it runs the day's fitness update
/// (`exe::injury_update_player_fitness`, once a day) and returns the 0x30-byte entry of `injury_fields::INJURY_ENTRIES`
/// indexed by the 31-byte physical record's `physical_record::CURRENT_INJURY`, or 0 when that index is -1.
/// Original: `Injury::get_player_injury` (0x62ed70, injury.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn get_player_injury(this: usize, person: usize) -> usize {
    if person == 0 {
        show_error_at(0x9e7f28, 0xe3a); // "...\injury.cpp"
        return 0;
    }
    if read_i32(person) >= read_i32(G_STAFF_COUNT) {
        return 0;
    }
    exe::injury_update_player_fitness(this, person);
    let id = read_i32(person) as usize;
    let record = read_usize(this + injury_fields::PHYSICAL_RECORDS).wrapping_add(id.wrapping_mul(31));
    let k = read_i32(record + physical_record::CURRENT_INJURY);
    if k == -1 {
        return 0;
    }
    covered!("sx12_1 current injury");
    read_usize(this + injury_fields::INJURY_ENTRIES).wrapping_add((k as usize).wrapping_mul(0x30)) // sx12_1: the injury entry
}

/// Returns 1 when the key nation table holds a nation of the given continent chosen to be played, else 0.
///
/// `key_nation_continent_has_selected_nation(continent)`: over the 35 rows of `G_LEAGUE_TABLE` (72 bytes, nation at
/// +0), the first row whose nation is not null, has `.continent()` equal to the given one and `.league_selected()` not
/// zero gives 1. Called by the awards (0x4135b0), the discipline (0x55d420), the continental cups and the setup.
/// Original: `key_nation_continent_has_selected_nation` (0x6699d0, key_nation.cpp, cdecl).
pub(crate) unsafe extern "C" fn key_nation_continent_has_selected_nation(continent: usize) -> u32 {
    for k in 0..0x23usize {
        let n = Nation(read_usize(G_LEAGUE_TABLE + 72 * k));
        if !n.is_null() && n.continent() == continent && n.league_selected() != 0 {
            covered!("sx12_1 continent has a selected nation");
            return 1; // sx12_1: a chosen nation of the continent
        }
    }
    0
}

/// Numbers the active rows of the key nation table and stores how many there are; returns 1.
///
/// `key_nation_number_active_entries()`: clears the count (`G_KEY_NATION_ACTIVE_COUNT`, a byte); a row of
/// `G_LEAGUE_TABLE` is active when its nation (+0) is null (the world and continent rows) or chosen to be played
/// (`.league_selected()` not zero); each active row gets the running count at `key_nation_record::ACTIVE_INDEX`, and
/// the count rises. The inactive rows keep their old index. Called by the game setup (0x828110, 0x82a9c0).
/// Original: `key_nation_number_active_entries` (0x669140, key_nation.cpp, cdecl).
pub(crate) unsafe extern "C" fn key_nation_number_active_entries() -> u32 {
    let mut count: u8 = 0;
    write_u8(G_KEY_NATION_ACTIVE_COUNT, 0);
    for k in 0..0x23usize {
        let row = KeyNationRecord(G_LEAGUE_TABLE + 72 * k);
        let n = Nation(read_usize(row.addr())); // sx12_1: the row's nation
        if n.is_null() || n.league_selected() != 0 {
            covered!("sx12_1 active key nation row");
            row.set_active_index(count);
            count = count.wrapping_add(1); // sx12_1: the next active index
            write_u8(G_KEY_NATION_ACTIVE_COUNT, count);
        }
    }
    1
}

/// Comparer for `get_player_injury`: the returned entry, side by side. The original runs first and does the day's
/// fitness update (with its draws), so the copy finds the day already done. A null person (the "Error" box): only the
/// original runs.
pub(crate) unsafe extern "thiscall" fn compare_get_player_injury(this: usize, person: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%);
    if person == 0 {
        return o(this, person);
    }
    let r = o(this, person);
    let n = get_player_injury(this, person);
    SCOREBOARD.record(r == n, || format!("get_player_injury(person {:#x}) original {:#x} rust {:#x}", person, r, n));
    r
}

/// Comparer for `key_nation_continent_has_selected_nation`: the return, side by side (the routine only reads).
pub(crate) unsafe extern "C" fn compare_key_nation_continent_has_selected_nation(continent: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(%%I1%%)(continent);
    let n = key_nation_continent_has_selected_nation(continent);
    SCOREBOARD.record(r == n, || format!("key_nation_continent_has_selected_nation({:#x}) original {} rust {}", continent, r, n));
    r
}

/// Comparer for `key_nation_number_active_entries`: the 35 rows of the table and the count, side by side.
pub(crate) unsafe extern "C" fn compare_key_nation_number_active_entries() -> u32 {
    let o = original_fn::<unsafe extern "C" fn() -> u32>(%%I2%%);
    let regions = [(G_LEAGUE_TABLE, 0x23 * 72), (G_KEY_NATION_ACTIVE_COUNT, 1)];
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(), || key_nation_number_active_entries());
    SCOREBOARD.record(equal && r == n, || format!("key_nation_number_active_entries() original {} rust {}: {:?}", r, n, first_differences(&mo, &mr)));
    r
}
