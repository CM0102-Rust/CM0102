// ---------- sbI_2: the friendlies competition, the host table and the weather objects ----------

pub(crate) const FN_FRIENDLY_LOAD_FROM_TEMP_FILE_SBI: usize = 0x5c8a00; // Friendly::load_from_temp_file thiscall(this, unused) ret 4 -> 1/0

/// Builds the friendlies competition, for a new game or from Friendly.tmp.
///
/// - The Comp base (`exe::new_comp`), `preseason_base_date_addr()` an empty date (`exe::date_date_54a110`), the 127
/// tournament slots (`friendly::TOURNAMENT_SLOTS`, 0x69 bytes each; `exe::crt_vector_constructor_iterator` with the
/// slot constructor 0x5c22c0 and the empty destructor 0x5a53c0), the Friendly vtable (0x96ae7c) and
/// `league::STAGE_INDEX` 0xff (the main competition).
/// - The CD check (`find_game_cd`, the no-CD patch read at 0x5c2026 and 0x5c2068).
/// - `league::COMPETITION` = the friendlies record (`KEY_CLUB_COMP_FRIENDLY` in `G_CLUB_COMP_TABLE`, 0x6b bytes each);
/// `first_tournament_comp_id()` = `G_CLUB_COMP_COUNT` − 127.
/// - `friendly::PRESEASON_PLAN_DAY_A` = the day of 24 June 1998 and `friendly::PRESEASON_PLAN_DAY_B` the day of 3
/// January 1998 (`exe::date_date_549f70`, months from 0).
/// - The friendly, scheduled slot, pending tournament and request reply lists, the planned friendlies, their count and
/// capacity 0 (`planned_next_index()` is not touched).
/// - The international date table (`build_international_date_table`), the sorted nation list (`build_nation_list`) and
/// the banned nation pairs (`init_banned_nation_pairs`).
/// - `friendly::NATION_LISTS` = malloc of 4 × `G_NATION_COUNT`; none: the "Error" box (friendly.cpp line 0x69) and stop.
/// - When `Friendly::load_from_temp_file` (with 1) reads the temporary file, nothing else. Otherwise each tournament
/// slot is emptied (owner 0, its index at +0x64, created competition +0x65 0) and each nation gets a new empty list
/// (`exe::crt_operator_new` of 0xe bytes, `exe::new_llist`; 0 when the allocation fails).
///
/// Returns this.
/// Original: `Friendly::Friendly` (0x5c1f80, friendly.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn new_friendly(this: usize) -> usize {
    let fr = Friendly(this);
    exe::new_comp(this);
    exe::date_date_54a110(fr.preseason_base_date_addr());
    exe::crt_vector_constructor_iterator(this + friendly::TOURNAMENT_SLOTS, 0x69, 0x7f, 0x5c22c0usize, 0x5a53c0usize); // the slot constructor, the empty destructor
    write_usize(this, 0x96ae7c); // the Friendly vtable
    write_u8(this + league::STAGE_INDEX, 0xff);
    find_game_cd(0x5c2026, 0x5c2068);
    write_usize(this + league::COMPETITION, read_usize(G_CLUB_COMP_TABLE) + read_usize(KEY_CLUB_COMP_FRIENDLY) * 0x6b);
    fr.set_first_tournament_comp_id(read_i32(G_CLUB_COMP_COUNT).wrapping_sub(0x7f) as i16);
    let (mut june, mut january) = ([0u16; 4], [0u16; 4]);
    exe::date_date_549f70(&mut june, 0x18, 5, 0x7ce, u32::MAX); // sbI_2: 24 June 1998
    exe::date_date_549f70(&mut january, 3, 0, 0x7ce, u32::MAX); // 3 January 1998
    write_u16(this + friendly::PRESEASON_PLAN_DAY_A, june[0]);
    write_u16(this + friendly::PRESEASON_PLAN_DAY_B, january[0]);
    for field in [friendly::FRIENDLY_MATCH_LIST, friendly::SCHEDULED_SLOT_LIST, friendly::PENDING_TOURNAMENT_LIST,
                  friendly::REQUEST_REPLY_LIST, friendly::PLANNED_FRIENDLIES, friendly::PLANNED_COUNT, friendly::PLANNED_CAPACITY] {
        write_u32(this + field, 0);
    }
    build_international_date_table(this);
    build_nation_list(this);
    init_banned_nation_pairs(this);
    let lists = exe::crt_malloc(read_u32(G_NATION_COUNT).wrapping_mul(4));
    write_usize(this + friendly::NATION_LISTS, lists);
    if lists == 0 {
        show_error_box(0x9cd0a0, 0x69); // friendly.cpp
        return this;
    }
    let load: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_FRIENDLY_LOAD_FROM_TEMP_FILE_SBI);
    if load(this, 1) != 0 {
        covered!("sbI_2 friendlies from the temporary file");
        return this;
    }
    covered!("sbI_2 new friendlies");
    for k in 0..0x7f {
        let slot = this + friendly::TOURNAMENT_SLOTS + k * 0x69;
        write_u32(slot, 0); // +0: the owner
        write_u8(slot + 0x64, k as u8); // +0x64: the slot index
        write_u32(slot + 0x65, 0); // +0x65: the created competition
    }
    let mut k = 0usize;
    while (k as i32) < read_i32(G_NATION_COUNT) {
        let mut list = exe::crt_operator_new(0xe);
        if list != 0 {
            list = exe::new_llist(list);
        }
        write_usize(read_usize(this + friendly::NATION_LISTS) + 4 * k, list);
        k += 1;
    }
    this
}

/// Comparer for `new_friendly`: only the original runs (it builds the Comp base and the lists, which cannot be built
/// twice). It checks what the constructor sets whatever the path: the vtable, the competition record, the first
/// tournament id, both planning days and the per-nation list array.
pub(crate) unsafe extern "thiscall" fn compare_new_friendly(this: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I0%%)(this);
    let fr = Friendly(this);
    let (mut june, mut january) = ([0u16; 4], [0u16; 4]);
    exe::date_date_549f70(&mut june, 0x18, 5, 0x7ce, u32::MAX);
    exe::date_date_549f70(&mut january, 3, 0, 0x7ce, u32::MAX);
    let competition = read_usize(G_CLUB_COMP_TABLE) + read_usize(KEY_CLUB_COMP_FRIENDLY) * 0x6b;
    SCOREBOARD.record(r == this && read_usize(this) == 0x96ae7c && read_usize(this + league::COMPETITION) == competition
        && fr.first_tournament_comp_id() == read_i32(G_CLUB_COMP_COUNT).wrapping_sub(0x7f) as i16
        && read_u16(this + friendly::PRESEASON_PLAN_DAY_A) == june[0] && read_u16(this + friendly::PRESEASON_PLAN_DAY_B) == january[0]
        && read_usize(this + friendly::NATION_LISTS) != 0,
        || format!("new_friendly: vtable {:#x}, competition {:#x} (plan {:#x}), first tournament {}, days {}/{} (plan {}/{}), nation lists {:#x}",
            read_usize(this), read_usize(this + league::COMPETITION), competition, fr.first_tournament_comp_id(),
            read_u16(this + friendly::PRESEASON_PLAN_DAY_A), read_u16(this + friendly::PRESEASON_PLAN_DAY_B), june[0], january[0],
            read_usize(this + friendly::NATION_LISTS)));
    r
}

/// Builds the host table of the international competitions, for a new game or from HostCountry.tmp.
///
/// - The CD check (`find_game_cd`, the no-CD patch read at 0x5f97fa and 0x5f983d).
/// - When `load_temp_file` reads the temporary file, nothing else.
/// - Otherwise the count (+4, short) is 57 and the table ([this]) a malloc of 57 × `HOST_ENTRY_SIZE`; none: the
/// "Error" box (host_country.cpp line 0x50) and stop. The default table (`init_default_table`) sorted by edition year,
/// then decision day (`exe::crt_qsort` with the game's comparer 0x5fb3f0).
///
/// Returns this.
/// Original: `HostCountry::HostCountry` (0x5f97c0, host_country.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn new_host_country(this: usize) -> usize {
    find_game_cd(0x5f97fa, 0x5f983d);
    if load_temp_file(this) != 0 {
        covered!("sbI_2 hosts from the temporary file");
        return this;
    }
    covered!("sbI_2 new host table");
    write_i16(this + 4, 0x39); // +4: the entry count, 57
    let table = exe::crt_malloc((0x39 * HOST_ENTRY_SIZE) as u32);
    write_usize(this, table); // +0: the entries
    if table == 0 {
        show_error_box(0x9cf0d8, 0x50); // host_country.cpp
        return this;
    }
    init_default_table(this);
    exe::crt_qsort(read_usize(this), read_i16(this + 4) as i32 as u32, HOST_ENTRY_SIZE as u32, 0x5fb3f0usize); // sbI_2: sorted
    this
}

/// Comparer for `new_host_country`: only the original runs (it allocates the table and draws the decision days); the
/// table must be there and in the order of the game's comparer 0x5fb3f0.
pub(crate) unsafe extern "thiscall" fn compare_new_host_country(this: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I1%%)(this);
    let order: unsafe extern "C" fn(usize, usize) -> i32 = core::mem::transmute(0x5fb3f0usize);
    let (table, n) = (read_usize(this), read_i16(this + 4) as i32);
    let unsorted = if table == 0 { None } else {
        (1..n.max(0) as usize).find(|&k| order(table + (k - 1) * HOST_ENTRY_SIZE, table + k * HOST_ENTRY_SIZE) > 0)
    };
    SCOREBOARD.record(r == this && table != 0 && n > 0 && unsorted.is_none(),
        || format!("new_host_country: table {:#x}, {} entries, first out of order {:?}", table, n, unsorted));
    r
}

/// Builds the weather object: the CD check (`find_game_cd`, the no-CD patch read at 0x9226fe and 0x922741), the region
/// count (+8), the regions (+0) and the city table (+4) zeroed, and the regions and the cities' weather from
/// CM3_DATA\weather.cfg (`load_config`). Returns this.
/// Original: `Weather::Weather` (0x9226c0, weather.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn new_weather(this: usize) -> usize {
    find_game_cd(0x9226fe, 0x922741);
    write_u32(this + 8, 0); // +8: the region count
    write_u32(this, 0); // +0: the regions
    write_u32(this + 4, 0); // +4: the city table
    covered!("sbI_2 new weather");
    load_config(this); // sbI_2: the weather.cfg
    this
}

/// Comparer for `new_weather`: only the original runs (it reads weather.cfg and draws each city's weather); the regions,
/// the city table and the region count must be there.
pub(crate) unsafe extern "thiscall" fn compare_new_weather(this: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I2%%)(this);
    SCOREBOARD.record(r == this && read_usize(this) != 0 && read_usize(this + 4) != 0 && read_i32(this + 8) > 0,
        || format!("new_weather: regions {:#x}, cities {:#x}, {} regions", read_usize(this), read_usize(this + 4), read_i32(this + 8)));
    r
}
