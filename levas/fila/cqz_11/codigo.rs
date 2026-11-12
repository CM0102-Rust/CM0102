// ---------- cqz_11: the World Cup finals: the constructor and the nation list (world_cup.cpp) ----------

/// The World Cup edition: from `start`, up one year at a time while the remainder of (year − 1994) by 4 is not 0
/// (signed, as the original's `and 0x80000003` with its sign adjustment).
fn world_cup_edition(start: i16) -> i16 {
    let mut year = start;
    while (year as i32 - 0x7ca) % 4 != 0 {
        year = year.wrapping_add(1);
    }
    year
}

/// The World Cup finals at career creation.
///
/// League::League (`exe::new_league`, mov ecx -> thiscall, ret 0); the record (`league::COMPETITION`), vtable 0x9712f0,
/// `club_sides` 4, no upper or lower division (−1), `league::ENTRANTS_ARE_QUALIFIERS` 1, discipline rule 0, the edition
/// (`world_cup_edition` of the year after the game's: in a 2001 game, 2002), importance 0xf; the entrant book of 32
/// places (`league::ENTRANT_COUNT` 0, `league::ENTRANT_CAPACITY` 0x20, `exe::crt_calloc(0x20, 6)`: 2 pushes; add esp ->
/// cdecl) into `league::ENTRANT_BOOK`; without it, the error box of line 0x58 of world_cup.cpp and returns this. If the
/// saved state loads (`exe::league_load_from_temp_file(this, 1)`), returns this.
///
/// Otherwise: `league_flags` 0x28, the main stage, `current_stage` −1, 8 stages and the stage vector
/// (`exe::crt_malloc(0x20)`, unchecked); the nation list (WorldCup::set_up_team_list 0x92da40, thiscall, ret 0; return
/// value not read); the rules (WorldCup::set_up_rules 0x92d200; 0: the box of line 0x72 and returns this); the
/// statistics (operator new `exe::crt_operator_new(0x5ce)` and Comp_Stats::Comp_Stats `exe::new_comp_stats(new,
/// this)`), null: the box of line 0x7a (and it goes on). Then, when the edition is the game's year (words), the entrant
/// count = 32, else `league::ACTIVE` = 0; `league::TABLE` = null and `league::CLUB_COUNT` = 0. Returns this.
/// Original: `WorldCup::WorldCup` (0x92bf50, world_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_world_cup(this: usize, year: u32, record: usize) -> usize {
    let league = League(this);
    exe::new_league(league);
    write_u32(this + league::COMPETITION, record as u32);
    write_u32(this, 0x9712f0);
    league.set_club_sides(4);
    write_u32(this + league::UPPER_DIVISION_ID, 0xffff_ffff);
    write_u32(this + league::LOWER_DIVISION_ID, 0xffff_ffff);
    write_u8(this + league::ENTRANTS_ARE_QUALIFIERS, 1);
    league.set_discipline_rule(0);
    let edition = world_cup_edition((year as u16).wrapping_add(1) as i16); // cqz11: the next World Cup
    league.set_year(edition);
    league.set_importance(0xf);
    write_u16(this + league::ENTRANT_COUNT, 0);
    write_u16(this + league::ENTRANT_CAPACITY, 0x20);
    let book = exe::crt_calloc(0x20, 6);
    write_u32(this + league::ENTRANT_BOOK, book as u32);
    if book == 0 {
        show_error_at(0xad5888, 0x58);
        return this;
    }
    if exe::league_load_from_temp_file(league, 1) != 0 {
        return this;
    }
    covered!("cqz_11 world cup");
    league.set_league_flags(0x28);
    league.set_stage_index(stage_index::MAIN);
    write_u32(this + league::CURRENT_STAGE, 0xffff_ffff);
    write_u32(this + league::STAGE_COUNT, 8);
    write_u32(this + league::STAGES, exe::crt_malloc(0x20) as u32);
    let team_list: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x92da40usize);
    team_list(this);
    let rules: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x92d200usize);
    if rules(this) == 0 {
        show_error_at(0xad5888, 0x72);
        return this;
    }
    let p = exe::crt_operator_new(0x5ce);
    let e = if p != 0 { exe::new_comp_stats(p, league) } else { 0 };
    write_u32(this + league::COMP_STATS, e as u32);
    if e == 0 {
        show_error_at(0xad5888, 0x7a);
    }
    if league.year() as u16 == year as u16 { // cqz11: the World Cup of the game's own year has its 32
        write_u16(this + league::ENTRANT_COUNT, 0x20);
    } else {
        write_u32(this + league::ACTIVE, 0);
    }
    write_u32(this + league::TABLE, 0);
    write_u16(this + league::CLUB_COUNT, 0);
    this
}

/// Comparer for `construct_world_cup`: only the original runs (it allocates the book, the stages and the statistics),
/// then the plan (`compare_nations_tournament_ctor`: this, vtable, record, edition year and importance).
pub(crate) unsafe extern "thiscall" fn compare_construct_world_cup(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
    compare_nations_tournament_ctor("construct_world_cup", League(this), record, 0x9712f0, world_cup_edition((year as u16).wrapping_add(1) as i16), || o(this, year, record))
}

/// The default World Cup field after the holder and the hosts, in entrant-book slots 3 to 31 (the keys of their
/// nations), written without status nor count.
const WORLD_CUP_DEFAULT_FIELD: [usize; 29] = [KEY_NATION_ITALY, KEY_NATION_SPAIN, KEY_NATION_ARGENTINA, KEY_NATION_ROMANIA, KEY_NATION_HOLLAND,
    KEY_NATION_NIGERIA, KEY_NATION_SOUTH_AFRICA, KEY_NATION_CAMEROON, KEY_NATION_TUNISIA, KEY_NATION_MOROCCO, KEY_NATION_MEXICO, KEY_NATION_JAMAICA,
    KEY_NATION_UNITED_STATES, KEY_NATION_ENGLAND, KEY_NATION_SCOTLAND, KEY_NATION_DENMARK, KEY_NATION_BULGARIA, KEY_NATION_YUGOSLAVIA,
    KEY_NATION_NORWAY, KEY_NATION_BELGIUM, KEY_NATION_CROATIA, KEY_NATION_AUSTRIA, KEY_NATION_COLOMBIA, KEY_NATION_PARAGUAY, KEY_NATION_CHILE,
    KEY_NATION_SOUTH_KOREA, KEY_NATION_JAPAN, KEY_NATION_SAUDI_ARABIA, KEY_NATION_IRAN];

/// The 2002 World Cup qualifiers from the CONCACAF and Africa, entered as qualified (status 6) after the holder and the
/// hosts: Costa Rica, two of the United States, Honduras and Mexico (drawn), then Cameroon, Nigeria, Senegal, Tunisia
/// and South Africa.
const WORLD_CUP_2002_AFRICA: [usize; 5] = [KEY_NATION_CAMEROON, KEY_NATION_NIGERIA, KEY_NATION_SENEGAL, KEY_NATION_TUNISIA, KEY_NATION_SOUTH_AFRICA];

/// The national team of the nation of key `key` (`exe::database_get_national_team` on `G_NATION_TABLE` +
/// `nation::SIZE` × [key]: push; add esp, 4 -> cdecl).
unsafe fn world_cup_team_of_key(key: usize) -> usize {
    exe::database_get_national_team(read_usize(G_NATION_TABLE).wrapping_add(read_usize(key).wrapping_mul(nation::SIZE)))
}

/// The entry `k` (6 bytes: +0 team, +4 status) of the World Cup entrant book.
unsafe fn world_cup_book_entry(this: usize, k: i16) -> usize {
    read_usize(this + league::ENTRANT_BOOK) + (k as isize * 6) as usize
}

/// Adds the national team of key `key` as qualified (status 6) at the entrant count: its team first, then, when the
/// count reached the capacity (signed words), the error box of line 0x456 and nothing; otherwise the status, the team
/// and the count up one.
unsafe fn world_cup_add_2002_qualifier(this: usize, key: usize) {
    let team = world_cup_team_of_key(key);
    let n = read_i16(this + league::ENTRANT_COUNT);
    if n >= read_i16(this + league::ENTRANT_CAPACITY) {
        show_error_at(0xad5888, 0x456);
        return;
    }
    write_u8(world_cup_book_entry(this, n) + 4, 6);
    write_usize(world_cup_book_entry(this, read_i16(this + league::ENTRANT_COUNT)), team);
    write_u16(this + league::ENTRANT_COUNT, read_u16(this + league::ENTRANT_COUNT).wrapping_add(1));
}

/// rule: the World Cup field: the holder first (status 2), then the hosts of the finals year (status 1; a host that is
/// also the holder does not count twice), then a fixed list of 29 nations in slots 3 to 31; for the 2002 finals the
/// CONCACAF and African qualifiers are entered as qualified (status 6) after the holder and the hosts: Costa Rica, two
/// of the United States, Honduras and Mexico (one in three United States and Honduras, else half and half United
/// States and Mexico or Honduras and Mexico), Cameroon, Nigeria, Senegal, Tunisia and South Africa
///
/// `league::ENTRANT_COUNT` = 0; entry 0's status = 2 and its team the last World Cup winner
/// (`exe::comp_util_get_latest_placed_club`(World Cup competition record, 0): 2 pushes; add esp -> cdecl); count 1. The
/// hosts of the finals year (HostCountry::find_hosts on [`G_HOST_COUNTRY`]: 5 pushes; mov ecx -> thiscall, ret 0x14;
/// the year with garbage in the high 16 bits, read as a word): with one or more, the entry at the count gets status 1 and the first host's national team, count up
/// one, and down one again when entry 1's team is entry 0's; with two or more, the same for the second host against
/// entry 2; with exactly one, entry 2's team is China's (no status, no count); with none, entries 1 and 2 get Iceland
/// and Hungary and the count is 3. Then slots 3 to 31 (+0x12 to +0xba) get `WORLD_CUP_DEFAULT_FIELD`. With the year
/// 2002, the 2002 qualifiers (`world_cup_add_2002_qualifier`, at the count, overwriting the default field): Costa Rica;
/// a draw of 3 (`exe::utils_random(3)`: push; add esp -> cdecl; the whole eax tested) giving 0, the United States and
/// Honduras, else a draw of 2: 0, the United States and Mexico, else Honduras and Mexico; then `WORLD_CUP_2002_AFRICA`.
/// Returns 1.
/// Original: `WorldCup::set_up_team_list` (0x92da40, world_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn world_cup_set_up_team_list(this: usize) -> u32 {
    let league = League(this);
    write_u16(this + league::ENTRANT_COUNT, 0);
    write_u8(read_usize(this + league::ENTRANT_BOOK) + 4, 2);
    let world_cup = read_usize(G_CLUB_COMP_TABLE).wrapping_add(read_usize(KEY_CLUB_COMP_FIFA_WORLD_CUP).wrapping_mul(0x6b));
    let holder = exe::comp_util_get_latest_placed_club(world_cup, 0);
    write_usize(world_cup_book_entry(this, read_i16(this + league::ENTRANT_COUNT)), holder);
    write_u16(this + league::ENTRANT_COUNT, read_u16(this + league::ENTRANT_COUNT).wrapping_add(1));
    let (mut second, mut first) = (0i32, 0i32);
    let n = exe::hostcountry_find_hosts(read_usize(G_HOST_COUNTRY), read_usize(KEY_CLUB_COMP_FIFA_WORLD_CUP), league.year() as u16 as u32, &mut first, &mut second, 1) as i8;
    if n > 0 {
        covered!("cqz_11 world cup hosts");
        for (k, host) in [(1usize, first), (2usize, second)].into_iter().take(if n > 1 { 2 } else { 1 }) {
            write_u8(world_cup_book_entry(this, read_i16(this + league::ENTRANT_COUNT)) + 4, 1);
            let team = exe::database_get_national_team(read_usize(G_NATION_TABLE).wrapping_add((host as usize).wrapping_mul(nation::SIZE)));
            write_usize(world_cup_book_entry(this, read_i16(this + league::ENTRANT_COUNT)), team);
            write_u16(this + league::ENTRANT_COUNT, read_u16(this + league::ENTRANT_COUNT).wrapping_add(1));
            let book = read_usize(this + league::ENTRANT_BOOK);
            if read_usize(book) == read_usize(book + k * 6) { // cqz11: a host that is the holder counts once
                write_u16(this + league::ENTRANT_COUNT, read_u16(this + league::ENTRANT_COUNT).wrapping_sub(1));
            }
        }
        if n == 1 {
            write_usize(read_usize(this + league::ENTRANT_BOOK) + 0xc, world_cup_team_of_key(KEY_NATION_CHINA));
        }
    } else {
        write_usize(read_usize(this + league::ENTRANT_BOOK) + 6, world_cup_team_of_key(KEY_NATION_ICELAND));
        write_usize(read_usize(this + league::ENTRANT_BOOK) + 0xc, world_cup_team_of_key(KEY_NATION_HUNGARY));
        write_u16(this + league::ENTRANT_COUNT, 3);
    }
    covered!("cqz_11 world cup field");
    for (k, &key) in WORLD_CUP_DEFAULT_FIELD.iter().enumerate() {
        write_usize(read_usize(this + league::ENTRANT_BOOK) + (k + 3) * 6, world_cup_team_of_key(key));
    }
    if league.year() != 0x7d2 {
        return 1;
    }
    covered!("cqz_11 world cup 2002 qualifiers");
    world_cup_add_2002_qualifier(this, KEY_NATION_COSTA_RICA);
    let pair = if exe::utils_random(3) == 0 { // cqz11: one in three, the United States and Honduras
        [KEY_NATION_UNITED_STATES, KEY_NATION_HONDURAS]
    } else if exe::utils_random(2) == 0 {
        [KEY_NATION_UNITED_STATES, KEY_NATION_MEXICO]
    } else {
        [KEY_NATION_HONDURAS, KEY_NATION_MEXICO]
    };
    for key in pair.into_iter().chain(WORLD_CUP_2002_AFRICA) {
        world_cup_add_2002_qualifier(this, key);
    }
    1
}

// simplification: the draw of the CONCACAF pair uses the game's random draws: it cannot run twice. Only the original; the plan:
// return value 1, the holder in entry 0 with status 2, the count within the capacity and, outside 2002, slots 3 to 31
// with the default field.
pub(crate) unsafe extern "thiscall" fn compare_world_cup_set_up_team_list(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    let r = o(this);
    let book = read_usize(this + league::ENTRANT_BOOK);
    let year2002 = League(this).year() == 0x7d2;
    let ok = r == 1 && read_u8(book + 4) == 2 && read_i16(this + league::ENTRANT_COUNT) <= read_i16(this + league::ENTRANT_CAPACITY)
        && (year2002 || WORLD_CUP_DEFAULT_FIELD.iter().enumerate().all(|(k, &key)| read_usize(book + (k + 3) * 6) == world_cup_team_of_key(key)));
    SCOREBOARD.record(ok, || format!("world_cup_set_up_team_list({:#x}) returned {}, {} entrants", this, r, read_i16(this + league::ENTRANT_COUNT)));
    r
}
