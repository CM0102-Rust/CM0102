// ---------- cx07_8: the draw of the Olympic football finalists (olympics.cpp) ----------

/// The 16 finalists of the 2000 Olympic football, in list order (the globals of the nations' ids; the list is then
/// cut into the four groups in order).
const OLYMPICS_2000_NATIONS: [usize; 16] = [
    KEY_NATION_AUSTRALIA, KEY_NATION_ITALY, KEY_NATION_NIGERIA, KEY_NATION_HONDURAS,
    KEY_NATION_UNITED_STATES, KEY_NATION_CZECH_REPUBLIC, KEY_NATION_CAMEROON, KEY_NATION_KUWAIT,
    KEY_NATION_SOUTH_AFRICA, KEY_NATION_JAPAN, KEY_NATION_BRAZIL, KEY_NATION_SLOVAKIA,
    KEY_NATION_CHILE, KEY_NATION_MOROCCO, KEY_NATION_SOUTH_KOREA, KEY_NATION_SPAIN,
];

/// The nations that never enter the Olympic football: the four British associations and Northern Ireland, Norway and
/// Sweden (the nations of region 6 are left out too).
const OLYMPICS_EXCLUDED_NATIONS: [usize; 7] = [
    KEY_NATION_SCOTLAND, KEY_NATION_ENGLAND, KEY_NATION_WALES, KEY_NATION_REPUBLIC_OF_IRELAND, KEY_NATION_NORTHERN_IRELAND,
    KEY_NATION_NORWAY, KEY_NATION_SWEDEN,
];

/// The continental quotas, in the order they are filled: each continent takes the best-ranked nations of the pool
/// until the list reaches its mark (Europe 5, Africa 3, North America 2, South America 2, Asia 3, Oceania 1).
const OLYMPICS_QUOTAS: [(usize, i16); 6] = [
    (KEY_CONTINENT_EUROPE, 5), (KEY_CONTINENT_AFRICA, 8), (KEY_CONTINENT_NORTH_AMERICA, 0xa),
    (KEY_CONTINENT_SOUTH_AMERICA, 0xc), (KEY_CONTINENT_ASIA, 0xf), (KEY_CONTINENT_OCEANIA, 0x10),
]; // cx07_8: the continental quotas

/// The seeding of the 16 into the four groups: group slot k takes entry `OLYMPICS_GROUP_SEEDING[k]` of the
/// continental list.
const OLYMPICS_GROUP_SEEDING: [usize; 16] = [0, 5, 9, 13, 1, 6, 10, 14, 2, 7, 11, 15, 3, 8, 12, 4];

/// The under-21 national team of the nation with the id held by the global `key` (`exe::database_get_national_u21_team`
/// on [`G_NATION_TABLE`] + `nation::SIZE` × id, cdecl).
unsafe fn u21_team_of_key(key: usize) -> usize {
    exe::database_get_national_u21_team(read_usize(G_NATION_TABLE).wrapping_add(read_usize(key).wrapping_mul(nation::SIZE)))
}

/// Whether the club `club` belongs to the continent whose id is held by the global `key` (its nation +0x53 and the
/// nation's continent, both checked).
unsafe fn olympic_team_of_continent(club: usize, key: usize) -> bool {
    let nation = read_usize(club + club::NATION);
    if nation == 0 {
        return false;
    }
    let continent = read_usize(nation + nation::CONTINENT);
    continent != 0 && read_usize(continent) == read_usize(key)
}

/// Rule: the Olympic football has 16 under-21 national teams. In 2000 a fixed list (the Sydney finalists). In other
/// years the hosts first, then every nation except the British associations, Norway, Sweden and the nations of region 6,
/// ranked by reputation after the first host; the list takes 5 from Europe, 3 from Africa, 2 from North America, 2 from
/// South America, 3 from Asia and 1 from Oceania, in that order, and is cut into four groups (slots 0, 5, 9, 13 to group
/// A; 1, 6, 10, 14 to B; 2, 7, 11, 15 to C; 3, 8, 12, 4 to D).
///
/// `OLYMPICS_DRAWN_COUNT` = 16. In 2000 (`.year()` 0x7d0): the list `OLYMPICS_DRAWN_CLUBS` (`exe::crt_malloc`(0x40);
/// out of memory, the box at line 0x337 and 0) with `OLYMPICS_2000_NATIONS` and returns 1.
///
/// Otherwise a pool of [`G_NATION_COUNT`] pointers (`exe::crt_malloc`; out of memory: the box at line 0x356 and 0). The
/// hosts of the year (`exe::hostcountry_find_hosts` on [`G_HOST_COUNTRY`] with
/// [`KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_OLYMPIC_GAMES`]: push 1; push &second; push &first; push year; push
/// competition -> thiscall, ret 0x14; returns how many, a signed byte) enter the pool first; then every nation in table
/// order not excluded (`OLYMPICS_EXCLUDED_NATIONS` and `nation::REGION` 6) and not the nation of a host (+0x53 of the
/// host team), all as under-21 teams (`exe::database_get_national_u21_team`). The pool after its first entry is sorted
/// by reputation (`exe::crt_qsort` with 0x4b07d0, cdecl). The continental list (`exe::crt_malloc`(16 × 4) into
/// `OLYMPICS_DRAWN_CLUBS`; out of memory: the box at line 0x38f and 0, the pool not freed) takes the pool's nations of
/// each continent in `OLYMPICS_QUOTAS` order up to each mark. The pool is freed; the list is reordered into the groups
/// (`OLYMPICS_GROUP_SEEDING`, a new list of 16 × 4; out of memory: the box at line 0x3dd and 0), the old one freed, and
/// returns 1.
/// Original: `Olympics::draw_finalists` (0x7a09e0, olympics.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn olympics_draw_finalists(this: usize) -> u32 {
    let league = League(this);
    write_u32(this + OLYMPICS_DRAWN_COUNT, 0x10);
    let drawn = || read_usize(this + OLYMPICS_DRAWN_CLUBS);
    if league.year() == 0x7d0 {
        let list = exe::crt_malloc(0x40);
        write_usize(this + OLYMPICS_DRAWN_CLUBS, list);
        if list == 0 {
            show_error_at(OLYMPICS_SOURCE, 0x337);
            return 0;
        }
        covered!("cx07_8 olympics 2000 finalists");
        for (k, &key) in OLYMPICS_2000_NATIONS.iter().enumerate() {
            let team = u21_team_of_key(key);
            write_usize(drawn() + k * 4, team);
        }
        return 1;
    }
    let pool = exe::crt_malloc((read_i32(G_NATION_COUNT) as u32).wrapping_mul(4));
    if pool == 0 {
        show_error_at(OLYMPICS_SOURCE, 0x356);
        return 0;
    }
    let mut ids = [-1i32; 2]; // the original leaves the two locals unset: find_hosts writes those it returns
    let (first, second) = ids.split_at_mut(1);
    let hosts = exe::hostcountry_find_hosts(read_usize(G_HOST_COUNTRY), read_usize(KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_OLYMPIC_GAMES),
        league.year() as u16 as u32, first.as_mut_ptr(), second.as_mut_ptr(), 1) as i8;
    let mut count = 0i16;
    while count < hosts as i16 {
        let nation = read_usize(G_NATION_TABLE).wrapping_add((ids[count as usize] as usize).wrapping_mul(nation::SIZE));
        write_usize(pool + count as usize * 4, exe::database_get_national_u21_team(nation));
        count += 1;
    }
    let mut i = 0i16;
    let mut nation = read_usize(G_NATION_TABLE);
    while (i as i32) < read_i32(G_NATION_COUNT) {
        let id = read_usize(nation);
        let excluded = OLYMPICS_EXCLUDED_NATIONS.iter().any(|&k| id == read_usize(k)) || read_u8(nation + nation::REGION) == 6;
        let host = (0..hosts.max(0) as usize).any(|j| read_usize(read_usize(pool + j * 4) + club::NATION) == nation);
        if !excluded && !host {
            write_usize(pool + count as usize * 4, exe::database_get_national_u21_team(nation));
            count += 1;
        }
        i = i.wrapping_add(1);
        nation += nation::SIZE;
    }
    exe::crt_qsort(pool + 4, (count as i32 - 1) as u32, 4, 0x4b07d0);
    let list = exe::crt_malloc((read_u32(this + OLYMPICS_DRAWN_COUNT)).wrapping_mul(4));
    write_usize(this + OLYMPICS_DRAWN_CLUBS, list);
    if list == 0 {
        show_error_at(OLYMPICS_SOURCE, 0x38f);
        return 0;
    }
    covered!("cx07_8 olympics finalists by continent");
    let mut k = 0i16;
    for &(continent, mark) in OLYMPICS_QUOTAS.iter() {
        let mut j = 0i16;
        while j < count && k < mark {
            let team = read_usize(pool + j as usize * 4);
            if olympic_team_of_continent(team, continent) {
                write_usize(drawn() + k as usize * 4, team);
                k += 1;
            }
            j += 1;
        }
    }
    exe::crt_free(pool);
    let groups = exe::crt_malloc((read_u32(this + OLYMPICS_DRAWN_COUNT)).wrapping_mul(4));
    if groups == 0 {
        show_error_at(OLYMPICS_SOURCE, 0x3dd);
        return 0;
    }
    for (slot, &entry) in OLYMPICS_GROUP_SEEDING.iter().enumerate() {
        write_usize(groups + slot * 4, read_usize(drawn() + entry * 4));
    }
    exe::crt_free(drawn());
    write_usize(this + OLYMPICS_DRAWN_CLUBS, groups);
    1
}

/// Comparer for `olympics_draw_finalists`, side by side: the original draws first; then the copy draws again over the
/// same competition (the draw is a pure function of the database: no random numbers) and the two lists of 16 and the
/// returns must match. The copy's list is freed and the original's put back. When the original returns 0 (out of
/// memory) only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_olympics_draw_finalists(this: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%)(this);
    if r == 0 {
        return r;
    }
    let kept = read_usize(this + OLYMPICS_DRAWN_CLUBS);
    let original: Vec<usize> = (0..16).map(|k| read_usize(kept + k * 4)).collect();
    let c = olympics_draw_finalists(this);
    let list = read_usize(this + OLYMPICS_DRAWN_CLUBS);
    let copy: Vec<usize> = if c == 1 && list != 0 { (0..16).map(|k| read_usize(list + k * 4)).collect() } else { Vec::new() };
    if list != 0 && list != kept {
        exe::crt_free(list);
    }
    write_usize(this + OLYMPICS_DRAWN_CLUBS, kept);
    SCOREBOARD.record(c == r && copy == original, || format!("olympics_draw_finalists({:#x}): original {} {:x?}, copy {} {:x?}", this, r, original, c, copy));
    r
}
