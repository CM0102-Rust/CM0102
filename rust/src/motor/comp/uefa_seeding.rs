use super::*;

/// Address of entry `i` in the UEFA seeding nation table (records of 0x12 bytes from the object).
///
/// +0 nation id, +4 the nation, +8 the 5-season coefficient (float), +0xc Champions League slots, +0xd UEFA Cup slots,
/// +0xe Intertoto slots, +0xf the season's points (short), +0x11 the nation's clubs in the cups (char).
pub(crate) fn seeding_nation_entry(this: usize, i: i32) -> usize {
    this.wrapping_add((i * 0x12) as usize)
}

/// the UEFA coefficient rule: a nation adds up its last 5 seasons (each season = its clubs' points in the European cups
/// ÷ the number of clubs); at the turn of the year the seasons shift by one (in 2001 the game uses the database ones,
/// without shifting); the table is sorted from the highest coefficient down and the position is stored in the nation's
/// national team
/// Original: `UefaSeeding::roll_coefficients` (0x903b50, uefa_seeding.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn roll_coefficients(this: usize) {
    let count = || read_i8(this + uefa_seeding::NATION_COUNT);
    let year = read_i16(G_CURRENT_DATE_YEAR);
    let mut i = 0i8;
    while i < count() {
        let e = seeding_nation_entry(this, i as i32);
        let p = Nation(read_u32(e + 4) as usize);
        if year != 0x7d1 {
            for k in 0..10usize {
                write_u32(p.addr() + nation::UEFACOEFFICIENT91 + k * 4, read_u32(p.addr() + nation::UEFACOEFFICIENT92 + k * 4));
            }
            let clubs = read_i8(e + uefa_seeding_nation::CLUBS_IN_CUPS);
            if clubs != 0 {
                let points = read_i16(e + uefa_seeding_nation::SEASON_POINTS) as i32;
                p.set_uefacoefficient96(points as f64 / clubs as i32 as f64);
            } else {
                write_u64(p.addr() + nation::UEFACOEFFICIENT96, 0);
            }
        }
        covered!("cqi_1 coefficient sum");
        let sum = p.uefacoefficient95() + p.uefacoefficient94() + p.uefacoefficient93() + p.uefacoefficient92() + p.uefacoefficient91(); // cqi1: as 5 temporadas
        write_f32(e + 8, sum as f32);
        write_u8(e + uefa_seeding_nation::CLUBS_IN_CUPS, 0);
        write_u16(e + uefa_seeding_nation::SEASON_POINTS, 0);
        i += 1;
    }
    exe::crt_qsort(this, count() as i32 as u32, 0x12, 0x905cb0);
    let mut i = 0i8;
    while i < count() {
        let s = Club(exe::database_get_national_team(read_u32(seeding_nation_entry(this, i as i32) + 4) as usize));
        s.set_euro_seeding((i) as u8);
        i += 1;
    }
}

/// Up to 16 differing addresses between two memory snapshots.
pub(crate) fn memory_diff_addresses(mo: &crate::comparison::MemorySnapshot, mr: &crate::comparison::MemorySnapshot) -> Vec<usize> {
    let (a, b) = (&mo.0, &mr.0);
    (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect()
}

/// Comparer regions: the nation table and each nation's 5 seasons (+0xe0..+0x110) and national team +0x1df.
pub(crate) unsafe fn seeding_regions(this: usize) -> Vec<(usize, usize)> {
    let n = read_i8(this + uefa_seeding::NATION_COUNT).max(0) as i32;
    let mut reg = vec![(this, 0x43b)];
    for i in 0..n {
        let p = Nation(read_u32(seeding_nation_entry(this, i) + 4) as usize);
        reg.push((p.addr() + nation::UEFACOEFFICIENT91, 0x30));
        reg.push((exe::database_get_national_team(p) + club::EURO_SEEDING, 1));
    }
    reg
}

/// places by coefficient position. Champions League: 1st to 3rd 4 places, 4th to 6th 3, 7th to 15th 2, 16th to 48th 1.
/// UEFA Cup: 1st to 6th 3, 7th and 8th 4, 9th to 15th 2, 16th to 21st 3, 22nd to 48th 2, 49th and 50th 1 (Liechtenstein
/// and San Marino none). Intertoto: Moldova, San Marino, Andorra, Bosnia and Liechtenstein 0, Scotland 1; in 2001
/// France 4, Germany 3, Italy 1, Portugal, Norway, Spain and Russia 0; outside 2001 England 1; the others, in
/// coefficient order, 2 places up to the 16th of them, 1 up to the 43rd, then 0. Fair play: three nations get one extra
/// UEFA Cup place (2001: Finland, Slovakia and Belarus; later, three drawn at random, never Italy). Only the Intertoto
/// places are read by the distribution (`assign_intertoto_places`); the Champions League and UEFA Cup ones (and fair play) are stored,
/// but the distribution uses fixed bands by position [inference: they feed the screens]
/// Original: `UefaSeeding::set_place_counts` (0x903cc0, uefa_seeding.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn set_place_counts(this: usize) {
    let g = |a: usize| read_u32(a);
    let count = || read_i8(this + uefa_seeding::NATION_COUNT);
    let year = read_i16(G_CURRENT_DATE_YEAR);
    let (mut fa, mut fb, mut fc, mut cont) = (-1i8, -1i8, -1i8, 0i8);
    let mut i = 0i8;
    while i < count() {
        let e = seeding_nation_entry(this, i as i32);
        let id = read_u32(e);
        if id == g(KEY_NATION_FINLAND) {
            fa = i;
        } else if id == g(KEY_NATION_SLOVAKIA) {
            fb = i;
        } else if id == g(KEY_NATION_BELARUS) {
            fc = i;
        }
        if id == g(KEY_NATION_LIECHTENSTEIN) {
            write_u8(e + uefa_seeding_nation::CHAMPIONS_LEAGUE_PLACES, 0);
        }
        write_u8(e + uefa_seeding_nation::CHAMPIONS_LEAGUE_PLACES, if i < 3 {
            4
        } else if i < 6 {
            3
        } else if i < 0xf {
            2
        } else if i < 0x30 {
            1
        } else {
            0
        });
        let id = read_u32(e);
        if id == g(KEY_NATION_LIECHTENSTEIN) || id == g(KEY_NATION_SAN_MARINO) {
            write_u8(e + uefa_seeding_nation::UEFA_CUP_PLACES, 0);
        }
        let uefa = if i < 6 {
            Some(3)
        } else if i < 8 {
            Some(4)
        } else if i < 0xf {
            Some(2)
        } else if i < 0x15 {
            Some(3)
        } else if i < 0x30 {
            Some(2)
        } else if i < 0x32 {
            Some(1)
        } else {
            None
        };
        if let Some(v) = uefa {
            write_u8(e + uefa_seeding_nation::UEFA_CUP_PLACES, v);
        }
        let id = read_u32(e);
        let intertoto = if id == g(KEY_NATION_MOLDOVA) || id == g(KEY_NATION_SAN_MARINO) || id == g(KEY_NATION_ANDORRA) || id == g(KEY_NATION_BOSNIA_HERZEGOVINA) || id == g(KEY_NATION_LIECHTENSTEIN) {
            0
        } else if id == g(KEY_NATION_SCOTLAND) {
            1
        } else if id == g(KEY_NATION_FRANCE) && year == 0x7d1 {
            covered!("cqi_1 france intertoto");
            4 // cqi1: France in the Intertoto in 2001
        } else if id == g(KEY_NATION_GERMANY) && year == 0x7d1 {
            3
        } else if (id == g(KEY_NATION_PORTUGAL) || id == g(KEY_NATION_NORWAY) || id == g(KEY_NATION_SPAIN) || id == g(KEY_NATION_RUSSIA)) && year == 0x7d1 {
            0
        } else if id == g(KEY_NATION_ENGLAND) && year != 0x7d1 {
            1
        } else if (id == g(KEY_NATION_ITALY) || id == g(KEY_NATION_PORTUGAL)) && year == 0x7d1 {
            1
        } else if cont < 0x10 || (id == g(KEY_NATION_BELGIUM) && year == 0x7d0) {
            cont += 1;
            2
        } else if cont < 0x2b {
            cont += 1;
            1
        } else {
            cont += 1;
            0
        };
        write_u8(e + uefa_seeding_nation::INTERTOTO_PLACES, intertoto);
        i += 1;
    }
    let italia = |k: i8| read_u32(seeding_nation_entry(this, k as i32)) == g(KEY_NATION_ITALY);
    if year == 0x7d1 {
        if fa == -1 || fb == -1 || fc == -1 {
            show_error_at(0xaa1a1c, 0x285);
        }
    } else {
        fa = -1;
    }
    while fa == -1 || fb == -1 || fc == -1 || fa == fb || fa == fc || fb == fc || italia(fa) || italia(fb) || italia(fc) {
        fa = exe::utils_random(count() as i32) as u8 as i8;
        fb = exe::utils_random(count() as i32) as u8 as i8;
        fc = exe::utils_random(count() as i32) as u8 as i8;
    }
    for k in [fa, fb, fc] {
        let p = seeding_nation_entry(this, k as i32) + uefa_seeding_nation::UEFA_CUP_PLACES;
        write_u8(p, read_u8(p).wrapping_add(1));
    }
}

/// `uefa_find_nth_unplaced_club_of_nation` (`this` is not read): in the European club table sorted by nation, returns
/// the nth club of the nation, skipping on request those already holding a European place and those with a human
/// manager; this is how each place goes to the nation's next club (the order within the nation comes from
/// `collect_european_clubs`).
///
/// In the European clubs table (12-byte records sorted by nation, `collect_european_clubs`: +0 the club, +4 the nation
/// id, +8, +9, +0xb), the binary search (bsearch `FN_CRT_BSEARCH` with 0x905ce0; the key has only the nation and +8 =
/// 0xff, so only the nation counts) finds a club of the nation; it steps back to the nation's first one; it walks to
/// the `n`th (char, 1 = the first) that counts, skipping, if asked (`without_slot`), those that already have a place in
/// the European cups (`.continental_comp_id()` ≠ −1) and (`without_human`) those with a human manager
/// (`exe::human_manager_is_human_club`(club, 0, 1, 0, 0)). The last record of the table is never examined by the loop:
/// if it gets there with the count at zero, that one is chosen without the tests. Returns the club if it belongs to the
/// nation, otherwise 0.
/// Original: `uefa_find_nth_unplaced_club_of_nation` (0x9050d0, uefa_seeding.cpp, thiscall, ret 0x18 (this não lido)).
pub(crate) unsafe extern "thiscall" fn uefa_find_nth_unplaced_club_of_nation(_this: usize, clubs: usize, count: u32, nation: u32, n: u32, without_slot: u32, without_human: u32) -> usize {
    let search: unsafe extern "C" fn(*const u8, usize, i32, u32, usize) -> usize = core::mem::transmute(FN_CRT_BSEARCH);
    let k = count as u16 as i16 as i32;
    let last = clubs.wrapping_add((k * 12 - 12) as usize);
    let mut key = [0u8; 12];
    key[4..8].copy_from_slice(&nation.to_le_bytes());
    key[8] = 0xff;
    let mut p = search(key.as_ptr(), clubs, k, 0xc, 0x905ce0);
    if p == 0 {
        return 0;
    }
    while p > clubs && read_u32(p - 8) == nation {
        p -= 12;
    }
    let mut foul = (n as u8).wrapping_sub(1);
    let mut found = false;
    if p < last {
        loop {
            let c = Club(read_u32(p) as usize);
            if without_slot != 0 && !c.is_null() && c.continental_comp_id() != u32::MAX {
                covered!("cqi_1 already with a place");
                p += 12; // cqi1: skips the club that already has a place
            } else if without_human != 0 && !c.is_null() && exe::human_manager_is_human_club(c, 0, 1, 0, 0) != 0 {
                p += 12;
            } else {
                if foul == 0 {
                    found = true;
                    break;
                }
                p += 12;
                foul = foul.wrapping_sub(1);
            }
            if p >= last {
                break;
            }
        }
    }
    if !found && foul != 0 {
        return 0;
    }
    if read_u32(p + 4) == nation {
        return read_u32(p) as usize;
    }
    0
}

/// Increments the byte at an address.
pub(crate) unsafe fn increment_byte(a: usize) {
    write_u8(a, read_u8(a).wrapping_add(1));
}

/// Club `c` gets the place in competition `comp`, at phase `fase`, if it has a nation and has no place yet (+0x1db =
/// −1).
///
/// Sets +0x1db = comp, +0x1df = fase and counts one more club for nation `i` (+0x11). Returns whether it got the place.
/// (The case of a nation with no entry, −1, which the original looks up by the club's nation, does not happen: `i`
/// always comes from the loop over the nations.)
pub(crate) unsafe fn assign_comp_place(this: usize, c: Club, comp: u32, stage: u8, i: i32) -> bool {
    if c.is_null() || (c.nation().addr() as u32) == 0 || c.continental_comp_id() != u32::MAX {
        return false;
    }
    c.set_euro_seeding(stage);
    c.set_continental_comp_id(comp);
    increment_byte(seeding_nation_entry(this, i) + uefa_seeding_nation::CLUBS_IN_CUPS);
    true
}

/// `UefaSeeding::collect_european_clubs`: builds the table of European clubs (national teams excluded), each without a
/// European place, sorted by nation and, within it, by division, position and reputation; in 2001 euro.cfg can move
/// clubs to the front of their nation.
///
/// It allocates 12 bytes per club in the game (`exe::crt_malloc`; without memory, the box of line 0x2ed and returns 0)
/// and stores the pointer in `*out`; for each club (`club::SIZE`-byte records in `G_CLUB_TABLE`, `G_CLUB_COUNT` clubs)
/// with a nation (`club::NATION`), that is not a national side (`exe::database_is_national_team`) and whose nation is
/// in Europe (`.continent()` with the id of `KEY_CONTINENT_EUROPE`): it clears the European place
/// (`club::CONTINENTAL_COMP_ID` = −1 and the dword of the club's row in `G_CLUB_STATE_TABLE`, 6 bytes per club, = −1)
/// and writes the record (+0 the club, +4 the nation id, +8 the reputation `club::REPUTATION` / 500, +9 the
/// `.reputation()` of the division `club::LAST_DIVISION` [inference: the division's level],
/// `european_club_entry::LAST_POSITION` the `club::LAST_POSITION` [inference: the position in it]; without a division,
/// −1 and 0x7f; with the row of a competition without `.league_selected()`, the row and the club's
/// `club::LAST_POSITION` are zeroed). Sorts by nation (qsort `exe::crt_qsort` with 0x905ce0) and, in 2001
/// (`uefa_seeding::SEASON_YEAR`), applies the euro.cfg order (`exe::uefaseeding_apply_euro_cfg_order`, original).
/// Returns the number of clubs (only the low 16 bits count: the caller reads si).
/// Original: `UefaSeeding::collect_european_clubs` (0x904040, uefa_seeding.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn collect_european_clubs(this: usize, out: *mut usize) -> u32 {
    let t = exe::crt_malloc((read_u32(G_CLUB_COUNT) as i32).wrapping_mul(12) as u32);
    *out = t;
    if t == 0 {
        show_error_at(0xaa1a1c, 0x2ed);
        return 0;
    }
    let mut k = 0u32;
    let mut c = read_u32(G_CLUB_TABLE) as usize;
    let mut i = 0u32;
    if read_u32(G_CLUB_COUNT) as i32 > 0 {
        loop {
            let nation = Nation(read_u32(c + club::NATION) as usize);
            if !nation.is_null() && exe::database_is_national_team(c) == 0 {
                let count = (nation.continent() as u32) as usize;
                if count != 0 && read_u32(count) == read_u32(KEY_CONTINENT_EUROPE) {
                    write_u32(c + club::CONTINENTAL_COMP_ID, u32::MAX);
                    write_u32((read_u32(G_CLUB_STATE_TABLE) as usize).wrapping_add((read_u32(c) as usize).wrapping_mul(6)), u32::MAX);
                    let r = t.wrapping_add((k as u16 as i16 as i32 * 12) as usize);
                    write_u32(r, c as u32);
                    write_u32(r + 4, read_u32(nation.addr()));
                    write_u8(r + 8, (read_i16(c + club::REPUTATION) as i32 / 500) as u8);
                    let row = Competition(read_u32(c + club::LAST_DIVISION) as usize);
                    if !row.is_null() {
                        covered!("cqi_2 division level");
                        write_u16(r + 9, row.reputation() as u16); // cqi2: the division level
                        write_u8(r + european_club_entry::LAST_POSITION, read_u8(c + club::LAST_POSITION));
                        let comp = Nation(read_u32(read_u32(c + club::LAST_DIVISION) as usize + competition::NATION) as usize);
                        if comp.is_null() || comp.league_selected() == 0 {
                            write_u32(c + club::LAST_DIVISION, 0);
                            write_u8(c + club::LAST_POSITION, 0);
                        }
                    } else {
                        write_u16(r + 9, 0xffff);
                        write_u8(r + european_club_entry::LAST_POSITION, 0x7f);
                    }
                    k += 1;
                }
            }
            c += club::SIZE;
            i += 1;
            if (i as u16 as i16 as i32) >= read_u32(G_CLUB_COUNT) as i32 {
                break;
            }
        }
    }
    exe::crt_qsort(t, (k as u16 as i16 as i32) as u32, 0xc, 0x905ce0);
    if read_i16(this + uefa_seeding::SEASON_YEAR) == 0x7d1 {
        exe::uefaseeding_apply_euro_cfg_order(this, t, k);
    }
    // simplification: the original returns ax with the leftover high bytes of eax (the caller reads only si); the copy, zero
    k as u16 as u32
}

/// Comparer regions: club table, club state table, random seed and competition history table.
pub(crate) unsafe fn club_regions() -> Vec<(usize, usize)> {
    let n = (read_u32(G_CLUB_COUNT) as i32).max(0) as usize;
    vec![(read_u32(G_CLUB_TABLE) as usize, n * club::SIZE), (read_u32(G_CLUB_STATE_TABLE) as usize, n * 6), (G_MOUSE_LEFT_BUTTON_DOWN, 4), (read_u32(G_CLUB_COMP_HISTORY_TABLE) as usize, (read_u32(G_CLUB_COMP_HISTORY_COUNT) as i32).max(0) as usize * 0x1a)]
}

/// the Champions League rule (the game's rounds 1 to 4 read as groups, third, second and first qualifying round
/// [inference]): the last winner goes straight to the groups; nations, by coefficient position: 1st to 3rd 2 clubs in
/// the groups and 2 in the third qualifying round; 4th to 6th 2 and 1; 7th to 9th 1 and 1; 10th 1 in the groups and 1
/// in the second qualifying round; 11th to 15th 1 in the third and 1 in the second; 16th 1 in the third; 17th to 29th 1
/// in the second; 30th onwards the champion in the first qualifying round. Clubs go in the nation's table order
/// (champion, runner-up...); each round has a capacity (groups 16, or 15 with the holder; third 18, second 19, first
/// 18) and a place in a full round moves to the next one; a club that already has a place loses its own
/// Original: `UefaSeeding::assign_champions_league_places` (0x904210, uefa_seeding.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn assign_champions_league_places(this: usize, clubs: usize, count: u32) {
    let lc = read_u32(KEY_CLUB_COMP_UEFA_EUROPEAN_CHAMPIONS_LEAGUE);
    let mut cap: [u8; 5] = [0, 0x10, 0x12, 0x13, 0x12];
    let h = Club(exe::comp_util_get_latest_placed_club((read_u32(G_CLUB_COMP_TABLE) as usize).wrapping_add(lc.wrapping_mul(0x6b) as usize), 0));
    if !h.is_null() && (h.nation().addr() as u32) != 0 && h.continental_comp_id() == u32::MAX {
        h.set_euro_seeding(1);
        h.set_continental_comp_id(lc);
        let id = read_u32((h.nation().addr() as u32) as usize);
        let n = read_i8(this + uefa_seeding::NATION_COUNT) as i32;
        let mut i = -1i32;
        for k in 0..n {
            if read_u32(seeding_nation_entry(this, k)) == id {
                i = k;
                break;
            }
        }
        increment_byte(seeding_nation_entry(this, i) + uefa_seeding_nation::CLUBS_IN_CUPS);
        cap[1] = 0xf;
    } else {
        show_error_at(0xaa1a1c, 0x35d);
    }
    let mut r = 0i8;
    while r < read_i8(this + uefa_seeding::NATION_COUNT) {
        let (band, ultima): ([u8; 4], u8) = if r < 3 {
            covered!("cqi_2 two in the groups");
            ([0, 2, 2, 0], 0) // cqi2: 1st to 3rd nation
        } else if r < 6 {
            ([0, 2, 1, 0], 0)
        } else if r < 9 {
            ([0, 1, 1, 0], 0)
        } else if r < 0xa {
            ([0, 1, 0, 1], 0)
        } else if r < 0xf {
            ([0, 0, 1, 1], 0)
        } else if r < 0x10 {
            ([0, 0, 1, 0], 0)
        } else if r < 0x1d {
            ([0, 0, 0, 1], 0)
        } else {
            ([0, 0, 0, 0], 1)
        };
        let mut slots = [band[0], band[1], band[2], band[3], ultima];
        let e = seeding_nation_entry(this, r as i32);
        let mut k = 1u8;
        while k < 8 {
            let c = exe::uefa_find_nth_unplaced_club_of_nation(this, clubs, count, read_u32(e), k as u32, 0, 0);
            let mut f = 1usize;
            while f < 5 {
                if slots[f] != 0 {
                    if cap[f] != 0 {
                        break;
                    }
                    covered!("cqi_2 full stage");
                    if f + 1 < 5 {
                        slots[f + 1] = slots[f + 1].wrapping_add(1); // cqi2: the place moves to the next stage
                    }
                    slots[f] = slots[f].wrapping_sub(1);
                }
                f += 1;
            }
            if f < 5 {
                if assign_comp_place(this, Club(c), lc, f as u8, r as i32) {
                    cap[f] = cap[f].wrapping_sub(1);
                }
                slots[f] = slots[f].wrapping_sub(1);
            }
            k += 1;
        }
        r += 1;
    }
}

/// the Intertoto rule: 60 clubs; it cycles through the nations in coefficient order, each nation with its places (from
/// `set_place_counts`) and its first clubs still without a European place; the first 8 nations enter round 1, the 9th to 12th
/// round 2, then round 3 (a nation's second club one round later). England puts in the volunteer club (`pick_english_intertoto_club`) or the
/// first one without a place and without a human manager, and a second one when it has 2 places
/// Original: `UefaSeeding::assign_intertoto_places` (0x904e30, uefa_seeding.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn assign_intertoto_places(this: usize, clubs: usize, count: u32) {
    let it = read_u32(KEY_CLUB_COMP_INTER_TOTO_CUP);
    let mut colocados = 0i32;
    let mut cl = 0u8;
    loop {
        let stage: u8 = if (cl as i8) < 8 { 1 } else if (cl as i8) >= 0xc { 3 } else { 2 };
        let mut i = cl;
        if cl == read_u8(this + uefa_seeding::NATION_COUNT) {
            i = 0;
        }
        let e = seeding_nation_entry(this, i as i8 as i32);
        if read_u32(e) == read_u32(KEY_NATION_ENGLAND) {
            let mut c = exe::uefaseeding_pick_english_intertoto_club(this);
            if c == 0 {
                c = exe::uefa_find_nth_unplaced_club_of_nation(this, clubs, count, read_u32(e), 1, 1, 1);
            }
            if assign_comp_place(this, Club(c), it, stage, i as i8 as i32) {
                colocados += 1;
            }
            if read_u8(e + uefa_seeding_nation::INTERTOTO_PLACES) == 2 {
                let mut c = exe::uefaseeding_pick_english_intertoto_club(this);
                if c == 0 {
                    c = exe::uefa_find_nth_unplaced_club_of_nation(this, clubs, count, read_u32(e), 1, 1, 1);
                }
                if assign_comp_place(this, Club(c), it, stage.wrapping_add(1), i as i8 as i32) {
                    colocados += 1;
                }
            }
            let list = read_u32(this + uefa_seeding::INTERTOTO_VOLUNTEERS) as usize;
            if list != 0 {
                exe::crt_free(list);
                write_u32(this + uefa_seeding::INTERTOTO_VOLUNTEERS, 0);
                write_u8(this + uefa_seeding::INTERTOTO_VOLUNTEER_COUNT, 0);
            }
        } else if (read_u8(e + uefa_seeding_nation::INTERTOTO_PLACES) as i8) > 0 {
            let mut j = 0i8;
            loop {
                if colocados as i16 >= 0x3c {
                    break;
                }
                let c = exe::uefa_find_nth_unplaced_club_of_nation(this, clubs, count, read_u32(e), 1, 1, 0);
                if assign_comp_place(this, Club(c), it, (j as u8).wrapping_add(stage), i as i8 as i32) {
                    covered!("cqi_2 club in intertoto");
                    colocados += 1; // cqi2: mais um na Intertoto
                }
                j += 1;
                if j >= read_u8(e + uefa_seeding_nation::INTERTOTO_PLACES) as i8 {
                    break;
                }
            }
        }
        cl = i.wrapping_add(1);
        if colocados as i16 >= 0x3c {
            break;
        }
    }
}

/// Club c (with nation, no place) enters the UEFA Cup at a phase; counts it for nation r.
pub(crate) unsafe fn enter_uefa_cup(this: usize, c: Club, stage: u8, r: i32, colocados: &mut i32) -> bool {
    if (c.nation().addr() as u32) == 0 || c.continental_comp_id() != u32::MAX {
        return false;
    }
    c.set_continental_comp_id(read_u32(KEY_CLUB_COMP_UEFA_CUP));
    c.set_euro_seeding(stage);
    *colocados += 1;
    let p = seeding_nation_entry(this, r) + uefa_seeding_nation::CLUBS_IN_CUPS;
    write_u8(p, read_u8(p).wrapping_add(1));
    true
}

/// the UEFA Cup rule (the game's round 1 = the first round, round 2 = the qualifying round [inference]): the last
/// winner enters the first round and its nation loses one place in that round; three nations drawn at random (fair
/// play) get one place in the qualifying round; by coefficient position, places in the first round and the qualifying
/// round: 1st to 6th 3 and 0, 7th and 8th 4 and 0, 9th to 15th 1 and 1, 16th and 17th 1 and 2, 18th to 21st 0 and 3,
/// 22nd to 48th 0 and 2, 49th onwards 0 and 1. A nation's first place goes to the national cup winner (or the runner-up
/// if the winner is already in the Champions League; with both already qualified, to the finalists of the cup still to
/// be played stored in the seeding, except Spain and Belgium), the second to the league cup winner, the others to the
/// top clubs of the table still without a place
/// Original: `UefaSeeding::assign_uefa_cup_places` (0x904580, uefa_seeding.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn assign_uefa_cup_places(this: usize, clubs: usize, count: u32) {
    let n8 = || read_i8(this + uefa_seeding::NATION_COUNT);
    let uc = read_u32(KEY_CLUB_COMP_UEFA_CUP);
    let mut count_extra = 1i8;
    let mut colocados = 0i32;
    let mut nation_of_champion = 0usize;
    let h = Club(exe::comp_util_get_latest_placed_club((read_u32(G_CLUB_COMP_TABLE) as usize).wrapping_add(uc.wrapping_mul(0x6b) as usize), 0));
    if !h.is_null() && (h.nation().addr() as u32) != 0 && h.continental_comp_id() == u32::MAX {
        h.set_euro_seeding(1);
        h.set_continental_comp_id(uc);
        colocados = 1;
        let id = read_u32((h.nation().addr() as u32) as usize);
        let mut i = -1i32;
        for k in 0..n8() as i32 {
            if read_u32(seeding_nation_entry(this, k)) == id {
                i = k;
                break;
            }
        }
        let p = seeding_nation_entry(this, i) + uefa_seeding_nation::CLUBS_IN_CUPS;
        write_u8(p, read_u8(p).wrapping_add(1));
        nation_of_champion = (h.nation().addr() as u32) as usize;
    }
    let s1 = exe::utils_random(n8() as i32) as u8 as i8;
    let mut s2 = exe::utils_random(n8() as i32) as u8 as i8;
    let mut s3 = exe::utils_random(n8() as i32) as u8 as i8;
    while s1 == s2 || s1 == s3 || s2 == s3 {
        s2 = exe::utils_random(n8() as i32) as u8 as i8;
        s3 = exe::utils_random(n8() as i32) as u8 as i8;
    }
    let mut last_stage = 0u8;
    let mut r = 0i8;
    while r < n8() {
        let (f1, f2): (u8, u8) = if r < 6 {
            covered!("cqi_3 three in the first stage");
            (3, 0) // cqi3: 1st to 6th nation
        } else if r < 8 {
            (4, 0)
        } else if r < 0xf {
            (1, 1)
        } else if r < 0x11 {
            (1, 2)
        } else if r < 0x15 {
            (0, 3)
        } else if r < 0x30 {
            (0, 2)
        } else {
            (0, 1)
        };
        let mut slots = [0u8, f1, f2];
        let e = seeding_nation_entry(this, r as i32);
        let nation = Nation(read_u32(e + 4) as usize);
        if nation.addr() == nation_of_champion {
            slots[1] = slots[1].wrapping_sub(1);
        }
        if r == s1 || r == s2 || r == s3 {
            covered!("cqi_3 fair play");
            slots[2] = slots[2].wrapping_add(1); // cqi3: the fair play place
        }
        let um = || exe::uefa_find_nth_unplaced_club_of_nation(this, clubs, count, read_u32(e), 1, 1, 0);
        let mut chosen = 0u8;
        let (mut vencedor, mut vice) = (0usize, 0usize);
        let mut f = 1u8;
        while f < 3 {
            let lim = slots[f as usize] as i8;
            if lim > 0 {
                last_stage = f;
                let mut j = 0i8;
                loop {
                    let mut try_of_new = false;
                    let c = if chosen == 0 {
                        if nation.league_selected() == 0 {
                            um()
                        } else {
                            let cup = read_u32(G_LEAGUE_WORLD_CUP_CUP_COMP + (((exe::key_nation_index_of_nation(nation) as u32) as i8) as i32 * 0x48) as usize) as usize;
                            if cup == 0 {
                                show_error_at(0xaa1a1c, 0x418);
                                try_of_new = true;
                                0
                            } else {
                                let mut c = exe::comp_util_get_latest_placed_club(cup, 0);
                                if c == 0 || read_u32(c + club::NATION) == 0 {
                                    c = um();
                                }
                                if read_u32(c + club::CONTINENTAL_COMP_ID) == u32::MAX {
                                    c
                                } else {
                                    let mut c = exe::comp_util_get_latest_placed_club(cup, 1);
                                    if c == 0 || read_u32(c + club::NATION) == 0 || read_u32(nation.addr()) == read_u32(KEY_NATION_BELGIUM) {
                                        c = um();
                                    }
                                    if read_u32(c + club::CONTINENTAL_COMP_ID) == u32::MAX {
                                        c
                                    } else if read_u32(G_COMP_OBJECT_TABLE) == 0 {
                                        um()
                                    } else {
                                        let id = read_u32(cup);
                                        let year = read_u16(0xb63d7b + (((exe::key_nation_index_of_nation(nation) as u32) as i8) as i32 * 0x48) as usize) as i32;
                                        let comp = read_u32((read_u32(G_COMP_OBJECT_TABLE) as usize).wrapping_add(id as usize * 4)) as usize;
                                        if read_i16(comp + cup::YEAR) as i32 == year || read_u32(nation.addr()) == read_u32(KEY_NATION_SPAIN) || read_u32(nation.addr()) == read_u32(KEY_NATION_BELGIUM) {
                                            um()
                                        } else {
                                            let (mut v1, mut v2) = (-1i32, -1i32);
                                            for k in 0..0x32usize {
                                                if read_u32(this + uefa_seeding::CUP_FINALS + k * 12) == id {
                                                    v1 = read_u32(this + uefa_seeding::CUP_FINALS + 4 + k * 12) as i32;
                                                    v2 = read_u32(this + uefa_seeding::CUP_FINALS + 8 + k * 12) as i32;
                                                    break;
                                                }
                                            }
                                            let base = read_u32(G_CLUB_TABLE) as usize;
                                            vencedor = if v1 != -1 { base.wrapping_add((v1 as usize).wrapping_mul(club::SIZE)) } else { 0 };
                                            vice = if v2 != -1 { base.wrapping_add((v2 as usize).wrapping_mul(club::SIZE)) } else { 0 };
                                            if vencedor == 0 {
                                                um()
                                            } else if vice != 0 {
                                                0
                                            } else {
                                                c
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    } else if chosen == 1 {
                        if nation.league_selected() == 0 {
                            um()
                        } else {
                            let cup = read_u32(G_LEAGUE_WORLD_CUP_SECOND_CUP_COMP + (((exe::key_nation_index_of_nation(nation) as u32) as i8) as i32 * 0x48) as usize) as usize;
                            if cup == 0 {
                                um()
                            } else {
                                let c = exe::comp_util_get_latest_placed_club(cup, 0);
                                if c == 0 || read_u32(c + club::NATION) == 0 || read_u32(c + club::CONTINENTAL_COMP_ID) != u32::MAX {
                                    um()
                                } else {
                                    c
                                }
                            }
                        }
                    } else {
                        um()
                    };
                    if try_of_new {
                        j -= 1;
                    } else {
                        chosen = chosen.wrapping_add(1);
                        if c != 0 {
                            enter_uefa_cup(this, Club(c), f, r as i32, &mut colocados);
                            if colocados as i16 as i32 - count_extra as i32 + 1 >= 0x76 {
                                return;
                            }
                        }
                    }
                    j += 1;
                    if j >= lim {
                        break;
                    }
                }
            }
            f += 1;
        }
        if vencedor != 0 && vice != 0 {
            count_extra += 1;
            let (a, b) = (read_u32(vencedor + club::CONTINENTAL_COMP_ID) == u32::MAX, read_u32(vice + club::CONTINENTAL_COMP_ID) == u32::MAX);
            if !a && !b {
                let c = um();
                if c != 0 {
                    enter_uefa_cup(this, Club(c), last_stage, r as i32, &mut colocados);
                }
            } else if a && b {
                let stage = (count_extra as u8).wrapping_mul(10).wrapping_add(3);
                last_stage = stage;
                if read_u32(vencedor + club::NATION) != 0 {
                    enter_uefa_cup(this, Club(vencedor), stage, r as i32, &mut colocados);
                }
                enter_uefa_cup(this, Club(vice), stage, r as i32, &mut colocados);
            } else {
                enter_uefa_cup(this, if a { Club(vencedor) } else { Club(vice) }, last_stage, r as i32, &mut colocados);
            }
        }
        r += 1;
    }
}

/// `UefaSeeding::pick_english_intertoto_club` (SEH frame for the local news item, no destructor on the normal path):
/// picks England's Intertoto club, with the news to the division. In 2001 England uses the table
/// (`assign_intertoto_places`).
///
/// In 2001 (`G_CURRENT_DATE_YEAR`), or without the Premier League (`G_COMP_OBJECT_TABLE` +
/// `KEY_CLUB_COMP_ENGLISH_PREMIER_DIVISION` × 4), returns 0. Otherwise it fills the volunteers list
/// (`uefa_seeding::INTERTOTO_VOLUNTEERS`, 30 ids, allocated with 0x78 bytes; `uefa_seeding::INTERTOTO_VOLUNTEER_COUNT`
/// the count) up to 6, drawing Premier clubs (`exe::utils_random` with `.club_count()`, up to 80 draws) without a human
/// manager and without a European place (`club::CONTINENTAL_COMP_ID`); it picks the volunteer without a place from the
/// highest division (the Premier's (`club::LAST_DIVISION` = `.competition()`) counts `club::LAST_POSITION`; the others
/// +30, i.e. the best placed [inference: `club::LAST_POSITION`]); with none, it draws Premier clubs until one without a
/// place. Delivers news item 0x17 with the club (`exe::new_newsitem`, `exe::newsitem_set_slot`,
/// `exe::news_send_to_division_managers` to the Premier division) and returns the club.
/// Original: `UefaSeeding::pick_english_intertoto_club` (0x903890, uefa_seeding.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn pick_english_intertoto_club(this: usize) -> usize {
    if read_i16(G_CURRENT_DATE_YEAR) == 0x7d1 {
        return 0;
    }
    let table = read_u32(G_COMP_OBJECT_TABLE) as usize;
    if table == 0 {
        return 0;
    }
    let league = League(read_u32(table.wrapping_add(read_u32(KEY_CLUB_COMP_ENGLISH_PREMIER_DIVISION) as usize * 4)) as usize);
    if league.is_null() {
        return 0;
    }
    covered!("cqi_3 english volunteer");
    let club_drawn = || {
        let s = exe::utils_random(league.club_count() as i32) as i16 as i32;
        read_u32(((league.table() as u32) as usize).wrapping_add((s * (league_table_row::SIZE as i32)) as usize)) as usize
    };
    let mut attempts = 0i8;
    while (read_u8(this + uefa_seeding::INTERTOTO_VOLUNTEER_COUNT) as i8) < 6 && attempts < 0x50 {
        attempts += 1;
        let c = club_drawn();
        if exe::human_manager_is_human_club(c, 0, 1, 0, 0) == 0 && read_u32(c + club::CONTINENTAL_COMP_ID) == u32::MAX {
            if read_u32(this + uefa_seeding::INTERTOTO_VOLUNTEERS) == 0 {
                write_u8(this + uefa_seeding::INTERTOTO_VOLUNTEER_COUNT, 0);
                let l = exe::crt_malloc(0x78);
                write_u32(this + uefa_seeding::INTERTOTO_VOLUNTEERS, l as u32);
                if l == 0 {
                    show_error_at(0xaa1a1c, 0xf9);
                    continue;
                }
            }
            let k = read_u8(this + uefa_seeding::INTERTOTO_VOLUNTEER_COUNT) as i8;
            if k < 0x1e {
                write_u32(read_u32(this + uefa_seeding::INTERTOTO_VOLUNTEERS) as usize + k as usize * 4, read_u32(c));
                write_u8(this + uefa_seeding::INTERTOTO_VOLUNTEER_COUNT, (k as u8).wrapping_add(1));
            }
        }
    }
    let mut best = 0usize;
    let mut level = 0x64i8;
    let k = read_u8(this + uefa_seeding::INTERTOTO_VOLUNTEER_COUNT) as i8 as i32;
    if k > 0 {
        let list = read_u32(this + uefa_seeding::INTERTOTO_VOLUNTEERS) as usize;
        let base = read_u32(G_CLUB_TABLE) as usize;
        for q in 0..k as usize {
            let c = base.wrapping_add((read_u32(list + q * 4) as usize).wrapping_mul(club::SIZE));
            if read_u32(c + club::CONTINENTAL_COMP_ID) == u32::MAX {
                let v = read_u8(c + club::LAST_POSITION) as i8;
                if read_u32(c + club::LAST_DIVISION) == (league.competition().addr() as u32) {
                    if v < level {
                        best = c;
                        level = v;
                    }
                } else if v as i32 + 0x1e < level as i32 {
                    best = c;
                    level = v.wrapping_add(0x1e);
                }
            }
        }
    }
    while best == 0 {
        let c = club_drawn();
        if read_u32(c + club::CONTINENTAL_COMP_ID) == u32::MAX {
            best = c;
        }
    }
    let mut news = [0u8; 0xe8];
    let n = news.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0x17, 0);
    exe::newsitem_set_slot(n, (0) as u32 as i8, read_u32(best));
    exe::news_send_to_division_managers(read_u32(G_NEWS) as usize, n, (league.competition().addr() as u32) as usize);
    best
}

/// `UefaSeeding::update_season` (SEH frame for the local news item, no destructor on the normal path), called by the
/// European cups: the seeding calendar.
///
/// Without `diario`: when the game year (`G_CURRENT_DATE_YEAR`) differs from `uefa_seeding::SEASON_YEAR`, it stores the
/// year and redoes the seeding: the coefficients (`exe::uefaseeding_roll_coefficients`), the places per nation
/// (`exe::uefaseeding_set_place_counts`), the clubs table (`exe::uefaseeding_collect_european_clubs`; empty, the box of
/// line 0x2b7), the Champions League (`exe::uefaseeding_assign_champions_league_places`), UEFA Cup
/// (`exe::uefaseeding_assign_uefa_cup_places`) and Intertoto (`exe::uefaseeding_assign_intertoto_places`) places, frees
/// the table and delivers the qualifiers news (`exe::uefa_post_qualifiers_news`). With `diario`: on day 100 of the year
/// (`G_CURRENT_DATE`), with the Premier League (`KEY_CLUB_COMP_ENGLISH_PREMIER_DIVISION`) and the Intertoto
/// (`KEY_CLUB_COMP_INTER_TOTO_CUP`) in the game (`G_COMP_OBJECT_TABLE`), news item 0x16 asking for volunteers (with the
/// year) to each human manager of a Premier club (`exe::news_club_has_human_manager`, `exe::news_send_to_club_manager`)
/// (without the Premier, it returns with nothing else); and the European registration deadline
/// (`exe::uefa_check_registration_deadlines`).
/// Original: `UefaSeeding::update_season` (0x9035a0, uefa_seeding.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn update_season(this: usize, diario: u32) {
    if diario == 0 {
        let year = read_i16(G_CURRENT_DATE_YEAR);
        if year == read_i16(this + uefa_seeding::SEASON_YEAR) {
            return;
        }
        covered!("cqi_3 qualification rollover");
        write_i16(this + uefa_seeding::SEASON_YEAR, year);
        exe::uefaseeding_roll_coefficients(this);
        exe::uefaseeding_set_place_counts(this);
        let mut t = 0usize;
        let n = exe::uefaseeding_collect_european_clubs(this, &mut t) as u32;
        if n as u16 == 0 {
            show_error_at(0xaa1a1c, 0x2b7);
        } else {
            exe::uefaseeding_assign_champions_league_places(this, t, n);
            exe::uefaseeding_assign_uefa_cup_places(this, t, n);
            exe::uefaseeding_assign_intertoto_places(this, t, n); // cqi3: the Intertoto places
            exe::crt_free(t);
        }
        exe::uefa_post_qualifiers_news(this);
        return;
    }
    if read_i16(G_CURRENT_DATE) == 100 {
        let table = read_u32(G_COMP_OBJECT_TABLE) as usize;
        let league = League(read_u32(table.wrapping_add(read_u32(KEY_CLUB_COMP_ENGLISH_PREMIER_DIVISION) as usize * 4)) as usize);
        if league.is_null() {
            return;
        }
        if read_u32(table.wrapping_add(read_u32(KEY_CLUB_COMP_INTER_TOTO_CUP) as usize * 4)) != 0 {
            let mut news = [0u8; 0xe8];
            let p = news.as_mut_ptr() as usize;
            exe::new_newsitem(p, 0x16, 0);
            exe::newsitem_set_slot(p, (0) as u32 as i8, 0);
            exe::newsitem_set_slot(p, (2) as u32 as i8, read_i16(this + uefa_seeding::SEASON_YEAR) as u16 as u32);
            let mut i = 0i16;
            while i < league.club_count() {
                let c = read_u32(((league.table() as u32) as usize).wrapping_add(i as i32 as usize * league_table_row::SIZE)) as usize;
                if exe::news_club_has_human_manager(read_u32(G_NEWS) as usize, c) != 0 {
                    exe::newsitem_set_slot(p, (1) as u32 as i8, read_u32(c));
                    exe::news_send_to_club_manager(read_u32(G_NEWS) as usize, p, c);
                }
                i += 1;
            }
        }
    }
    exe::uefa_check_registration_deadlines(this);
}

/// Reads a signed byte count as u32 (movsx).
pub(crate) unsafe fn signed_byte_count(a: usize) -> u32 {
    read_i8(a) as i32 as u32
}

/// Opens CM3_TEMP\<name> with "wb" through the game's path builder and fopen.
pub(crate) unsafe fn open_temp_file_for_write(name: usize) -> usize {
    exe::crt_fopen(exe::utils_build_file_path(0x987da0, name), 0x9a7f28)
}

/// Calls the exe's CRT fwrite.
pub(crate) unsafe fn crt_fwrite(data: usize, size: u32, how_many: u32, f: usize) -> u32 {
    exe::crt_fwrite(data, size, how_many, f)
}

/// Calls the exe's CRT fclose.
pub(crate) unsafe fn uefa_seeding_crt_fclose(f: usize) {
    exe::crt_fclose(f);
}

/// `UefaSeeding::save_to_temp_file`: writes `CM3_TEMP\uefa_seeding.tmp`.
///
/// The country count (`uefa_seeding::NATION_COUNT`, i8), the 0x12-byte records from +0, `uefa_seeding::SEASON_YEAR`
/// (u16), the count `uefa_seeding::INTERTOTO_VOLUNTEER_COUNT` (i8) and, if not zero, that many u32 from
/// [`uefa_seeding::INTERTOTO_VOLUNTEERS`], and the 50 fixed 0xc-byte records from `uefa_seeding::CUP_FINALS`. An error
/// opening or writing: the "Error" box (uefa_seeding.cpp 0x64f, 0x657, 0x660, 0x668, 0x66f, 0x679, 0x682), without
/// closing the file (as the original does).
/// Original: `UefaSeeding::save_to_temp_file` (0x905510, uefa_seeding.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn uefa_seeding_save_to_temp_file(this: usize) {
    const FONTE: usize = 0xaa1a1c;
    covered!("cpx_6 save UEFA seeds");
    let f = open_temp_file_for_write(0xaa1a50);
    if f == 0 {
        show_error_at(FONTE, 0x64f);
        return;
    }
    if crt_fwrite(this + uefa_seeding::NATION_COUNT, 1, 1, f) != 1 {
        show_error_at(FONTE, 0x657);
        return;
    }
    if crt_fwrite(this, 0x12, signed_byte_count(this + uefa_seeding::NATION_COUNT), f) != signed_byte_count(this + uefa_seeding::NATION_COUNT) {
        show_error_at(FONTE, 0x660);
        return;
    }
    if crt_fwrite(this + uefa_seeding::SEASON_YEAR, 2, 1, f) != 1 {
        show_error_at(FONTE, 0x668);
        return;
    }
    if crt_fwrite(this + uefa_seeding::INTERTOTO_VOLUNTEER_COUNT, 1, 1, f) != 1 {
        show_error_at(FONTE, 0x66f);
        return;
    }
    if read_u8(this + uefa_seeding::INTERTOTO_VOLUNTEER_COUNT) != 0 && crt_fwrite(read_usize(this + uefa_seeding::INTERTOTO_VOLUNTEERS), 4, signed_byte_count(this + uefa_seeding::INTERTOTO_VOLUNTEER_COUNT), f) != signed_byte_count(this + uefa_seeding::INTERTOTO_VOLUNTEER_COUNT) {
        show_error_at(FONTE, 0x679);
        return;
    }
    if crt_fwrite(this + uefa_seeding::CUP_FINALS, 0xc, 0x32, f) != 0x32 { // cpx6: os 50 registros fixos
        show_error_at(FONTE, 0x682);
        return;
    }
    uefa_seeding_crt_fclose(f);
}

