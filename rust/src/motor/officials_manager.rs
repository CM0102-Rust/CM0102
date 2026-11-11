use super::*;

/// adds the grade to the referee's record (17 bytes at [this] + id × 17, id in 0..[0xae2390]): +0xf the cards, words +8
/// and +0xa the fouls and the grade, +0x10 the last grade, +0xc the matches and +4 the average (+0xa / +0xc, float).
/// Compared side by side (the record)
/// Original: `record_referee_grade` (0x79ce10, officials_manager.cpp (208 bytes), thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn record_referee_grade(this: usize, r: *const u8) {
    let base = *(this as *const usize);
    if base == 0 || r.is_null() {
        return;
    }
    let id = *(r as *const i32);
    if id < 0 || id >= *(G_OFFICIAL_COUNT as *const i32) {
        return;
    }
    let f = base + id as usize * 17;
    *((f + official_stats::CARDS) as *mut u8) = (*((f + official_stats::CARDS) as *mut u8)).wrapping_add(*r.add(4));
    *((f + 8) as *mut i16) = (*((f + 8) as *mut i16)).wrapping_add(*r.add(5) as i8 as i16);
    *((f + official_stats::GRADE_SUM) as *mut i16) = (*((f + official_stats::GRADE_SUM) as *mut i16)).wrapping_add(*r.add(6) as i8 as i16);
    *((f + official_stats::LAST_GRADE) as *mut u8) = *r.add(6);
    *((f + official_stats::MATCHES) as *mut i16) = (*((f + official_stats::MATCHES) as *mut i16)).wrapping_add(1);
    if *((f + official_stats::MATCHES) as *mut i16) != 0 {
        *((f + 4) as *mut f32) = (*((f + official_stats::GRADE_SUM) as *mut i16) as f64 / *((f + official_stats::MATCHES) as *mut i16) as f64) as f32;
    }
}

/// Region of the official's 17-byte record that `record_referee_grade` touches (empty if id invalid).
pub(crate) unsafe fn official_record_region(manager: usize, id: i32) -> Vec<(usize, usize)> {
    let base = read_usize(manager);
    if base == 0 || id < 0 || id >= read_i32(G_OFFICIAL_COUNT) {
        return Vec::new();
    }
    vec![(base + id as usize * 17, 17)]
}

/// Record of official i (0x2b bytes in [0xae23e8]).
pub(crate) unsafe fn official_record(i: i32) -> usize {
    read_usize(G_OFFICIAL_TABLE).wrapping_add((i as isize * 0x2b) as usize)
}

/// Original's fatal out-of-memory end: box at officials_manager.cpp line, cleanup and exit(-1).
pub(crate) unsafe fn officials_out_of_memory(row: u32) -> ! {
    show_error_at(0xa6a22c, row);
    let exit: unsafe extern "C" fn(i32) -> ! = core::mem::transmute(FN_CRT_EXIT);
    exe::game_fatal_cleanup();
    exe::shared_noop_ret0_5a8f60();
    exit(-1)
}

/// Calls the game's qsort (`exe::crt_qsort`) with the given comparison routine.
pub(crate) unsafe fn officials_manager_crt_qsort(base: usize, n: u32, size: u32, cmp: usize) {
    exe::crt_qsort(base, n, size, cmp);
}

/// Calls the game's calloc (`exe::crt_calloc`).
pub(crate) unsafe fn crt_calloc(n: u32, size: u32) -> usize {
    exe::crt_calloc(n, size)
}

/// Calls the game's free (`exe::crt_free`).
pub(crate) unsafe fn crt_free(p: usize) {
    exe::crt_free(p);
}

/// Sets up the referees on a new game: reviews them, sorts them by nation and quality, keeps 6 per nation (61 in some
/// nations), builds the per-nation blocks with one slot per league, draws the missing referees and shares the nation's
/// referees among the slots.
///
/// `OfficialsManager::set_up_officials()`, from the constructor (0x79b580) on a new game. It reviews the records
/// (`officials_manager_validate_officials`), sorts them by nation and quality (qsort `officials_manager_compare_nation_then_quality_desc`) and, per nation, keeps the first 6 (nation with +0x11c
/// zero) or 61 (the first of each nation after the first one is not kept: the count restarts on it), copied (`officials_manager_operator`)
/// into a new table; the per-nation blocks (this+0xc, 0x102 bytes: +0 the nation, +4 the referees, +8 the first, +0xc
/// the demand, +0x10 the competitions and 15 slots of 16 bytes at +0x12): each league (`exe::comp_util_is_league_division`, not a friendly
/// `exe::friendly_is_friendly_competition`, with nation +0x5d) gets a slot with the demand (+0x3e of the competition) / 2 + 10; a nation with
/// competitions asks for the largest × the number and, if short, draws the missing ones (`officials_manager_add_random_officials`); without
/// competitions and with fewer than 5, it draws up to the nation's +0x8e (1..5). The indexes (+0), the first of each
/// nation and, per nation, the slots sorted (qsort `officials_manager_compare_comp_level`) with each one's share; this+4 the count and this+8 the
/// flags (calloc).
/// Original: `OfficialsManager::set_up_officials` (0x79bf60, officials_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn set_up_officials(this: usize) {
    exe::officials_manager_validate_officials();
    officials_manager_crt_qsort(read_usize(G_OFFICIAL_TABLE), read_i32(G_OFFICIAL_COUNT) as u32, 0x2b, 0x79d4e0);
    let n = read_i32(G_OFFICIAL_COUNT);
    let marks = crt_calloc(n as u32, 1);
    let mut ficam = 0i32;
    if n > 0 {
        let mut current = read_usize(official_record(0) + official::NATION);
        let mut group = 0i32;
        for i in 0..n {
            let nation = read_usize(official_record(i) + official::NATION);
            if nation != current {
                group = 0;
                current = nation;
                continue;
            }
            let m = (marks + i as usize) as *mut u8;
            if nation == 0 {
                *m = 0;
                continue;
            }
            let limit = if read_u8(nation + nation::LEAGUE_SELECTED) == 0 { 5 } else { 0x3c }; // cqg9: the referees who stay per nation
            if group <= limit {
                ficam += 1;
                *m = 1;
                covered!("cqg_9 referee kept");
            } else {
                *m = 0;
            }
            group += 1;
        }
    }
    let new = crt_calloc(ficam as u32, 0x2b);
    let mut dst = new;
    let mut i = 0i32;
    while i < read_i32(G_OFFICIAL_COUNT) {
        if read_u8(marks + i as usize) != 0 {
            exe::official_operator_3d(dst, official_record(i));
            dst += 0x2b;
        }
        i += 1;
    }
    crt_free(read_usize(G_OFFICIAL_TABLE));
    write_u32(G_OFFICIAL_TABLE, new as u32);
    write_u32(G_OFFICIAL_COUNT, ficam as u32);
    let nations = read_i32(G_NATION_COUNT);
    let blocos = exe::crt_malloc((nations as u32).wrapping_mul(0x102));
    write_u32(this + officials_manager_fields::NATION_BLOCKS, blocos as u32);
    if blocos == 0 {
        officials_out_of_memory(0x1ff);
    }
    for k in 0..nations.max(0) {
        let b = blocos + k as usize * 0x102;
        write_i16(b + officials_nation_block::COMP_COUNT, 0);
        write_u32(b + officials_nation_block::REFEREE_COUNT, 0);
        write_u32(b + officials_nation_block::FIRST_REFEREE, 1_000_000);
        write_u32(b + officials_nation_block::DEMAND, 0);
        write_u32(b, k as u32);
        for j in 0..0xf {
            let v = b + officials_nation_block::COMP_SLOTS + 16 * j;
            write_u32(v, 1_000_000);
            write_u32(v + 4, 0);
            write_u32(v + 8, 0);
            write_u32(v + 0xc, 0xffff_ffff);
        }
    }
    for k in 0..read_i32(G_COMP_OBJECT_COUNT).max(0) {
        if read_usize(read_usize(G_COMP_OBJECT_TABLE) + 4 * k as usize) == 0 || exe::comp_util_is_league_division(k) == 0 {
            continue;
        }
        let comp = Competition(read_usize(G_CLUB_COMP_TABLE).wrapping_add(k as usize * 0x6b));
        if exe::friendly_is_friendly_competition(read_usize(G_FRIENDLY), comp) != 0 {
            continue;
        }
        let nation = comp.nation();
        if nation.is_null() {
            continue;
        }
        let b = blocos.wrapping_add((read_i32(nation.addr()) as isize * 0x102) as usize);
        let v = b.wrapping_add((read_i16(b + officials_nation_block::COMP_COUNT) as isize * 0x10) as usize);
        write_u32(v + 0x1e, k as u32);
        let request = read_i16(read_usize(read_usize(G_COMP_OBJECT_TABLE) + 4 * k as usize) + league::CLUB_COUNT) as i32 / 2 + 0xa; // cqg9: the slot request
        covered!("cqg_9 competition slot");
        write_u32(v + 0x1a, request as u32);
        if read_i32(v + 0x1a) > read_i32(b + officials_nation_block::DEMAND) {
            write_u32(b + officials_nation_block::DEMAND, read_usize(v + 0x1a) as u32);
        }
        write_i16(b + officials_nation_block::COMP_COUNT, read_i16(b + officials_nation_block::COMP_COUNT).wrapping_add(1));
    }
    for i in 0..read_i32(G_OFFICIAL_COUNT).max(0) {
        let b = blocos.wrapping_add((read_i32(read_usize(official_record(i) + official::NATION)) as isize * 0x102) as usize);
        write_u32(b + officials_nation_block::REFEREE_COUNT, read_usize(b + officials_nation_block::REFEREE_COUNT).wrapping_add(1) as u32);
    }
    for k in 0..read_i32(G_NATION_COUNT).max(0) {
        let b = blocos + k as usize * 0x102;
        let nation = read_usize(G_NATION_TABLE).wrapping_add(k as usize * nation::SIZE);
        let c = read_i16(b + officials_nation_block::COMP_COUNT);
        if c > 0 {
            write_u32(b + officials_nation_block::DEMAND, (c as i32).wrapping_mul(read_i32(b + officials_nation_block::DEMAND)) as u32);
            if read_i32(b + officials_nation_block::DEMAND) > read_i32(b + officials_nation_block::REFEREE_COUNT) {
                exe::officials_manager_add_random_officials(read_i32(b + officials_nation_block::DEMAND) - read_i32(b + officials_nation_block::REFEREE_COUNT), nation);
                write_u32(b + officials_nation_block::REFEREE_COUNT, read_usize(b + officials_nation_block::DEMAND) as u32);
            }
        } else if read_i32(b + officials_nation_block::REFEREE_COUNT) < 5 {
            if read_i16(nation + nation::REPUTATION) == 0 {
                write_i16(nation + nation::REPUTATION, 1);
            }
            exe::officials_manager_add_random_officials((read_i16(nation + nation::REPUTATION) as i32).min(5), nation);
            write_u32(b + officials_nation_block::REFEREE_COUNT, (read_i16(read_usize(G_NATION_TABLE).wrapping_add(k as usize * nation::SIZE) + nation::REPUTATION) as i32).min(5) as u32);
        }
    }
    for i in 0..read_i32(G_OFFICIAL_COUNT).max(0) {
        write_u32(official_record(i), i as u32);
    }
    for i in 0..read_i32(G_OFFICIAL_COUNT).max(0) {
        let r = official_record(i);
        let b = blocos.wrapping_add((read_i32(read_usize(r + official::NATION)) as isize * 0x102) as usize);
        if read_i32(r) < read_i32(b + officials_nation_block::FIRST_REFEREE) {
            write_u32(b + officials_nation_block::FIRST_REFEREE, read_usize(r) as u32);
        }
    }
    for k in 0..read_i32(G_NATION_COUNT).max(0) {
        let b = read_usize(this + officials_manager_fields::NATION_BLOCKS) + k as usize * 0x102;
        let c = read_i16(b + officials_nation_block::COMP_COUNT);
        if c == 0 {
            continue;
        }
        officials_manager_crt_qsort(b + officials_nation_block::COMP_SLOTS, c as i32 as u32, 0x10, 0x79d410);
        let mut j = 0i32;
        while j < read_i16(b + officials_nation_block::COMP_COUNT) as i32 {
            let c = read_i16(b + officials_nation_block::COMP_COUNT) as i32;
            covered!("cqg_9 part of the slot");
            write_u32(b + 0x16 + 16 * j as usize, (read_i32(b + officials_nation_block::REFEREE_COUNT) / c) as u32); // cqg9: the share of each slot
            let c = read_i16(b + officials_nation_block::COMP_COUNT) as i32;
            write_u32(b + officials_nation_block::COMP_SLOTS + 16 * j as usize, (read_i32(b + officials_nation_block::REFEREE_COUNT).wrapping_mul(j) / c).wrapping_add(read_i32(b + officials_nation_block::FIRST_REFEREE)) as u32);
            j += 1;
        }
    }
    let total = read_i32(G_OFFICIAL_COUNT);
    write_u32(this + officials_manager_fields::BUSY_COUNT, total as u32);
    let m = crt_calloc(total as u32, 1);
    write_u32(this + officials_manager_fields::BUSY_FLAGS, m as u32);
    if m == 0 {
        officials_out_of_memory(0x2dd);
    }
    crt_free(marks);
}

/// Orders a nation's referee slots by competition level (the invalid ones last or first), tie by id.
///
/// `officials_manager_compare_comp_level(a, b)` (cdecl), the order of a nation's competition slots (qsort of `set_up_officials`,
/// 16-byte entries with the competition at +0xc): `a`'s competition outside 0..[0xae2378]: 1; `b`'s outside: −1; by
/// level (+0x69, word, of the 0x6b-byte competition in [0xae23d0]); same level: the difference of the ids.
/// Original: `officials_manager_compare_comp_level` (0x79d410, officials_manager.cpp, cdecl).
pub(crate) unsafe extern "C" fn officials_manager_compare_comp_level(a: usize, b: usize) -> i32 {
    let (ca, cb) = (read_i32(a + officials_comp_slot::COMP_ID), read_i32(b + officials_comp_slot::COMP_ID));
    let max = read_i32(G_CLUB_COMP_COUNT);
    if ca < 0 || ca > max {
        return 1;
    }
    if cb < 0 || cb > max {
        return -1;
    }
    let base = read_i32(G_CLUB_COMP_TABLE) as u32 as usize;
    let sa = read_i16(base.wrapping_add((ca as isize * 0x6b + (competition::REPUTATION as isize)) as usize));
    let signed_byte_at = read_i16(base.wrapping_add((cb as isize * 0x6b + (competition::REPUTATION as isize)) as usize));
    if sa == signed_byte_at {
        return ca.wrapping_sub(cb);
    }
    sa as i32 - signed_byte_at as i32
}

/// Official's sorting quality: byte +0x28 x 50 + word +0x22, in 16 bits.
pub(crate) unsafe fn official_quality(r: usize) -> i16 {
    (read_i8(r + official::REFEREEING) as i16).wrapping_mul(0x32).wrapping_add(read_i16(r + official::REPUTATION)) // cqh7: the quality weight
}

/// Nation id of the official ([+0x16] + 0).
pub(crate) unsafe fn official_nation_id(r: usize) -> i32 {
    read_i32(read_i32(r + official::NATION) as u32 as usize)
}

/// Orders the referees by nation and, within it, by quality (+0x28 × 50 + +0x22) from highest to lowest; tie by index,
/// descending.
///
/// `officials_manager_compare_nation_then_quality_desc(a, b)` (cdecl), the qsort order of `set_up_officials` (those kept per
/// nation): by nation (id) and, within it, by quality from highest to lowest; tie: `b`'s index +0 minus `a`'s.
/// Original: `officials_manager_compare_nation_then_quality_desc` (0x79d4e0, officials_manager.cpp, cdecl).
pub(crate) unsafe extern "C" fn officials_manager_compare_nation_then_quality_desc(a: usize, b: usize) -> i32 {
    let (na, nb) = (official_nation_id(a), official_nation_id(b));
    if na != nb {
        return na.wrapping_sub(nb);
    }
    let (ka, kb) = (official_quality(a), official_quality(b));
    covered!("cqh_7 descending order");
    if ka == kb {
        return read_i32(b).wrapping_sub(read_i32(a)); // cqh7: the tie by the index
    }
    kb as i32 - ka as i32
}

/// Orders the referees by nation and by quality from lowest to highest; tie by index, ascending.
///
/// `officials_manager_compare_nation_then_quality_asc(a, b)` (cdecl), the qsort order of `officials_manager_add_random_officials` (after the new
/// referees): by nation and by quality from lowest to highest; tie: `a`'s index +0 minus `b`'s.
/// Original: `officials_manager_compare_nation_then_quality_asc` (0x79d480, officials_manager.cpp, cdecl).
pub(crate) unsafe extern "C" fn officials_manager_compare_nation_then_quality_asc(a: usize, b: usize) -> i32 {
    let (na, nb) = (official_nation_id(a), official_nation_id(b));
    if na != nb {
        return na.wrapping_sub(nb);
    }
    let (ka, kb) = (official_quality(a), official_quality(b));
    covered!("cqh_7 ascending order");
    if ka == kb {
        return read_i32(a).wrapping_sub(read_i32(b));
    }
    ka as i32 - kb as i32 // cqh7: a ordem crescente
}

/// The nation's officials block in the manager (0x102 bytes in [this+0xc]: counts, first index, league bands).
pub(crate) unsafe fn nation_officials_block(this: usize, nation: i32) -> usize {
    read_usize(this + officials_manager_fields::NATION_BLOCKS).wrapping_add(nation.wrapping_mul(0x102) as usize)
}

/// Nation id of the club (+0x53), or None without nation.
pub(crate) unsafe fn officials_manager_club_nation_id(club: Club) -> Option<i32> {
    let n = club.nation().addr();
    if n != 0 { Some(read_i32(n)) } else { None }
}

/// Chooses a match referee: the referee range for the competition, a random one in it and the next free one.
///
/// `OfficialsManager::choose_match_official(match, out)`, from setting up the day (`set_up_the_day`, output at +0x2e of the
/// match). Removed match (−2): 0 (eax 0xffffff00, as in the original). The referee range: without a competition, the
/// home club's nation (+0x1c, +0x53) or a random one; a friendly (`exe::friendly_is_friendly_competition` on [0xae2c74]), the home club's; a
/// competition without an object ([0xadadfc]), as without a competition; a competition of types 2 and 4 (+0x42), the
/// top quarter (ftol of 0.75 and 0.25 of the count) of the first nation, starting from a random one, of the continent
/// (+0x59 of the competition, or +0x71 of the home club's nation, of the away club's, or a random one) that is not that
/// of either club (a neutral nation); in the others, the competition's range in its nation's block (+0x5d; or the home
/// club's, the away club's, or a random one), the last that matches, or the whole block. Count 1: the first; otherwise
/// first + random(count − 1). Busy (the byte of the referee's id in [this+8]): the next in the range, wrapping around;
/// all busy, the drawn one. Writes the index (word) and returns 1 in al (the rest of eax with the index, as in the
/// original).
/// Original: `OfficialsManager::choose_match_official` (0x79c7b0, officials_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn choose_match_official(this: usize, game: usize, out: usize) -> u32 {
    let game = Fixture(game);
    if read_i32(game.addr()) == -2 {
        return 0xffff_ff00;
    }
    let nations = read_i32(G_NATION_COUNT);
    let home = game.home_club();
    let away = game.away_club();
    let comp = game.competition();
    let band_of_nation = |nacao: i32| {
        let b = nation_officials_block(this, nacao);
        (read_i32(b + officials_nation_block::FIRST_REFEREE), read_i32(b + officials_nation_block::REFEREE_COUNT))
    };
    let (ini, n) = if comp.is_null() {
        band_of_nation(officials_manager_club_nation_id(home).unwrap_or_else(|| random(nations)))
    } else {
        let friendly: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_FRIENDLY_IS_FRIENDLY_COMPETITION);
        if friendly(read_usize(G_FRIENDLY), comp.addr()) != 0 {
            band_of_nation(read_i32(home.nation().addr())) // like the original, without checking the nation
        } else {
            let c = Cup(read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_i32(comp.addr()).wrapping_mul(4) as usize)));
            if c.is_null() {
                band_of_nation(officials_manager_club_nation_id(home).unwrap_or_else(|| random(nations)))
            } else {
                let kind = c.club_sides() as i8;
                if kind == 2 || kind == 4 {
                    let continent_of = |club: Club| {
                        let n = club.nation().addr();
                        if n != 0 && read_usize(n + nation::CONTINENT) != 0 { Some(read_i32(read_usize(n + nation::CONTINENT))) } else { None }
                    };
                    let continent = if comp.continent() != 0 {
                        read_i32(comp.continent())
                    } else if let Some(k) = continent_of(home) {
                        k
                    } else if let Some(k) = continent_of(away) {
                        k
                    } else {
                        random(read_i32(G_CONTINENT_COUNT))
                    };
                    let start = random(nations);
                    let mut k = start;
                    loop {
                        let nation = read_usize(G_NATION_TABLE).wrapping_add(k.wrapping_mul(0x122) as usize);
                        let cont = read_usize(nation + nation::CONTINENT);
                        if cont != 0 && read_i32(cont) == continent
                            && officials_manager_club_nation_id(home) != Some(read_i32(nation))
                            && officials_manager_club_nation_id(away) != Some(read_i32(nation))
                        {
                            break;
                        }
                        k += 1;
                        if k >= read_i32(G_NATION_COUNT) {
                            k = 0;
                        }
                        if k == start {
                            break;
                        }
                    }
                    let b = nation_officials_block(this, k);
                    let count = read_i32(b + officials_nation_block::REFEREE_COUNT) as f64;
                    covered!("cpy_8 neutral referee");
                    (read_i32(b + officials_nation_block::FIRST_REFEREE).wrapping_sub(ftol(count * -0.75)), ftol(count * 0.25)) // cpy8: the top quarter
                } else {
                    let nacao = if !comp.nation().is_null() {
                        read_i32(comp.nation().addr())
                    } else if let Some(k) = officials_manager_club_nation_id(home) {
                        k
                    } else if let Some(k) = officials_manager_club_nation_id(away) {
                        k
                    } else {
                        random(nations)
                    };
                    let b = nation_officials_block(this, nacao);
                    let id = read_i32(comp.addr());
                    let mut found = None;
                    for f in 0..read_i16(b + officials_nation_block::COMP_COUNT).max(0) as usize {
                        let e = b + officials_nation_block::COMP_SLOTS + f * 0x10;
                        if read_i32(e + officials_comp_slot::COMP_ID) == id {
                            found = Some((read_i32(e), read_i32(e + officials_comp_slot::SHARE)));
                        }
                    }
                    match found {
                        Some(band) => {
                            covered!("cpy_8 league band");
                            band // cpy8: the league band
                        }
                        None => (read_i32(b + officials_nation_block::FIRST_REFEREE), read_i32(b + officials_nation_block::REFEREE_COUNT)),
                    }
                }
            }
        }
    };
    let drawn = if n == 1 {
        ini
    } else {
        covered!("cpy_8 drawn referee");
        ini.wrapping_add(random(n.wrapping_sub(1))) // cpy8: the random draw within the band
    };
    let occupied = |k: i32| {
        let sheet = read_usize(G_OFFICIAL_TABLE).wrapping_add(k.wrapping_mul(0x2b) as usize);
        read_u8(read_usize(this + officials_manager_fields::BUSY_FLAGS).wrapping_add(read_usize(sheet))) != 0
    };
    let mut k = drawn;
    if occupied(k) {
        let end = n.wrapping_add(ini);
        loop {
            k += 1;
            if k >= end {
                k = ini;
            }
            if k == drawn || !occupied(k) {
                break;
            }
        }
    }
    write_i16(out, k as i16);
    (k as u32 & 0xffff_ff00) | 1
}

/// Clears the daily referee busy bytes.
///
/// `OfficialsManager::clear_busy_flags`, in the day loop (0x5cccd0) and in the run up to the start date (0x5cc850):
/// zeroes the [this+4] busy bytes at [this+8] (the count and the vector re-read at each step). Returns the count (0 if
/// not positive), like the original's eax.
/// Original: `OfficialsManager::clear_busy_flags` (0x79bf40, officials_manager.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn clear_busy_flags(this: usize) -> u32 {
    let mut i = 0i32;
    while i < read_i32(this + officials_manager_fields::BUSY_COUNT) {
        write_u8(read_usize(this + officials_manager_fields::BUSY_FLAGS).wrapping_add(i as usize), 0);
        i += 1;
    }
    i as u32
}

/// At each league's season start, replaces the referees of its range over 45 (or over 40, one in three) with new random
/// ones and clears the range's statistics rows.
///
/// `OfficialsManager::renew_officials(comp)`, at the start of each league's season (83 callers). Without a competition,
/// without an object ([0xadadfc]), of types 4 and 2 (+0x42), or without a exe::comp_util_is_league_division(`exe::comp_util_is_league_division`): 0. The competition's range
/// in its nation's block (+0x5d, or a random one), the first that matches, or the whole block. Each referee of the
/// range (0x2b-byte records in [0xae23e8]; null record: the "Error" box): over 45 years old (the year as a byte, minus
/// 1 before his birthday), or over 40 and random(3) zero, becomes another: two names from his nation (`database_pick_first_name_for_nation`, +4 and
/// +8), +0x25 random(16) + 5 (or, with random(10) zero, random(11) + random(11)), the year (+0xe) game year − 27 +
/// random(5) and the day (+0xc) random(340). Then it zeroes the statistics rows (17 bytes at [this]) of each referee of
/// the range (+4, +8, +0xa, +0xc, +0xf; the `add eax, 0x11` at 0x79d149 stays inside the loop). Returns 1 (only al
/// counts).
/// Original: `OfficialsManager::renew_officials` (0x79cee0, officials_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn renew_officials(this: usize, comp: usize) -> u8 {
    let comp = Competition(comp);
    if comp.is_null() {
        return 0;
    }
    let c = Cup(read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_i32(comp.addr()).wrapping_mul(4) as usize)));
    if c.is_null() || c.club_sides() == 4 || c.club_sides() == 2 {
        return 0;
    }
    if exe::comp_util_is_league_division(read_i32(comp.addr())) == 0 {
        return 0;
    }
    let nacao = if !comp.nation().is_null() { read_i32(comp.nation().addr()) } else { random(read_i32(G_NATION_COUNT)) };
    let b = nation_officials_block(this, nacao);
    let mut band = None;
    for f in 0..read_i16(b + officials_nation_block::COMP_COUNT).max(0) as usize {
        let e = b + officials_nation_block::COMP_SLOTS + f * 0x10;
        if read_i32(e + officials_comp_slot::COMP_ID) == read_i32(comp.addr()) {
            band = Some((read_i32(e), read_i32(e + officials_comp_slot::SHARE)));
            break;
        }
    }
    let (ini, n) = band.unwrap_or((read_i32(b + officials_nation_block::FIRST_REFEREE), read_i32(b + officials_nation_block::REFEREE_COUNT)));
    for k in 0..n.max(0) {
        let referee = Official(read_usize(G_OFFICIAL_TABLE).wrapping_add(ini.wrapping_add(k).wrapping_mul(0x2b) as usize));
        if referee.is_null() {
            show_error_at(0xa6a22c, 0xa61);
            continue;
        }
        let year = read_u8(G_CURRENT_DATE_YEAR);
        let dif = (referee.birth_day() as i32).wrapping_sub(read_i16(G_CURRENT_DATE) as i32);
        let age = if dif > 0 { year.wrapping_sub(referee.birth_year() as u8).wrapping_sub(1) } else { year.wrapping_sub(referee.birth_year() as u8) } as i8;
        if age <= 0x28 || (age <= 0x2d && random(3) != 0) {
            continue;
        }
        covered!("cpy_8 referee renewed");
        let nation = referee.nation();
        let n1 = exe::database_pick_first_name_for_nation(nation);
        referee.set_first_name(n1);
        // the original picks the second name with the first-name routine as well (DEFEITOS_DO_ORIGINAL: 0x79cee0)
        let n2 = exe::database_pick_first_name_for_nation(referee.nation());
        referee.set_second_name(n2);
        referee.set_discipline(if random(10) != 0 {
            (random(0x10) as u8).wrapping_add(5)
        } else {
            let a = random(0xb) as u8;
            a.wrapping_add(random(0xb) as u8)
        });
        let r5 = random(5);
        let base = (read_i16(G_CURRENT_DATE_YEAR) as u16).wrapping_sub(0x1b);
        referee.set_birth_year(((r5 as u16).wrapping_add(base)) as i16);
        referee.set_birth_day((random(0x154) as u16) as i16);
    }
    for k in 0..n.max(0) {
        let row = read_usize(this).wrapping_add(ini.wrapping_add(k).wrapping_mul(0x11) as usize);
        write_u8(row + official_stats::CARDS, 0);
        write_u16(row + official_stats::FOULS, 0);
        write_u16(row + official_stats::GRADE_SUM, 0);
        write_u16(row + official_stats::MATCHES, 0);
        write_u32(row + official_stats::AVERAGE_GRADE, 0);
    }
    1
}

/// Random byte: random(n) + k.
pub(crate) unsafe fn random_byte_plus(n: i32, k: u8) -> u8 {
    (random(n) as u8).wrapping_add(k)
}

/// A new referee record in `r` (0x2b bytes) for nation `nacao`, in the order of `0x79c6aa`.
///
/// First and second name by weight (`0x539b90`, `0x53a0e0`), the nation (`+0x16`), the words `+0x22` random draw(80) +
/// 1, `+0x1e` and `+0x20` random draw(80) + 40 (the byte extended), bytes `+0x24`..`+0x26` random draw(10) + 5 and
/// `+0x27`..`+0x2a` random draw(10) + 7, `+0x1a` zero, the year (`+0xe`) 1960 + random draw(10) and the day (`+0xc`)
/// random draw(300) + 1.
///
/// It does not write `+0` and `+0x10`..`+0x15` (the original routine leaves whatever the realloc brought).
pub(crate) unsafe fn generate_official(r: usize, nation: Nation) {
    write_usize(r + 4, exe::database_pick_first_name_by_weight(nation));
    write_usize(r + 8, exe::database_pick_second_name_by_weight(nation));
    write_usize(r + official::NATION, nation.addr());
    write_i16(r + official::REPUTATION, random_byte_plus(0x50, 1) as i8 as i16); // cpy9: a nota +0x22
    write_i16(r + official::CURRENT_ABILITY, random_byte_plus(0x50, 0x28) as i8 as i16);
    write_i16(r + official::POTENTIAL_ABILITY, random_byte_plus(0x50, 0x28) as i8 as i16);
    for o in [official::ALLOWING_FLOW, official::DISCIPLINE, official::IMPORTANT_MATCHES] {
        write_u8(r + o, random_byte_plus(10, 5));
    }
    write_u8(r + official::PRESSURE, random_byte_plus(10, 7));
    write_u8(r + official::REFEREEING, random_byte_plus(10, 7)); // cpy9: o atributo +0x28
    write_u8(r + official::RUNNING_LINE, random_byte_plus(10, 7));
    write_u8(r + official::TIMEKEEPING, random_byte_plus(10, 7));
    write_usize(r + official::CITY, 0);
    write_i16(r + official::BIRTH_YEAR, random(10).wrapping_add(0x7a8) as i16);
    write_i16(r + official::BIRTH_DAY, random(0x12c).wrapping_add(1) as i16);
}

/// Reallocates the referee table, generates `count` new referees of the nation and sorts the table by nation and
/// quality.
///
/// `officials_manager_add_random_officials(count, nation)` (thiscall with this unused, ret 8), from setting up the
/// referees of a new game (`set_up_officials`, in nations with few referees). A nation without a continent (+0x71): nothing. It
/// reallocates the table ([0xae23e8], `exe::crt_realloc`) to [0xae2390] + count records (out of memory: error and the game
/// stops), generates `count` new records at the end (generate_official: names by weight, random ratings and attributes,
/// born in the 1960s), adds the count to [0xae2390] and sorts the whole table (qsort `exe::crt_qsort` with `officials_manager_compare_nation_then_quality_asc`: nation,
/// +0x28 × 50 + +0x22, id).
/// Original: `officials_manager_add_random_officials` (0x79c5c0, officials_manager.cpp, thiscall (this sem uso), ret 8).
pub(crate) unsafe extern "thiscall" fn officials_manager_add_random_officials(_this: usize, count: i32, nation: usize) {
    let nation = Nation(nation);
    if !nation.is_null() && nation.continent() == 0 {
        return;
    }
    let before = read_i32(G_OFFICIAL_COUNT);
    let t = exe::crt_realloc(read_usize(G_OFFICIAL_TABLE), (before.wrapping_add(count).wrapping_mul(0x2b)) as u32);
    write_usize(G_OFFICIAL_TABLE, t);
    if t == 0 {
        fatal_error_at(0xa6a22c, 0x342);
    }
    let mut r = t.wrapping_add(before.wrapping_mul(0x2b) as usize);
    for _ in 0..count.max(0) {
        covered!("cpy_9 new referee");
        generate_official(r, nation);
        r += 0x2b;
    }
    let n = read_i32(G_OFFICIAL_COUNT).wrapping_add(count);
    write_i32(G_OFFICIAL_COUNT, n);
    covered!("cpy_9 sort");
    exe::crt_qsort(read_usize(G_OFFICIAL_TABLE), (n) as u32, 0x2b, 0x79d480); // cpy9: ordenar a tabela
}

/// Does the dry-run generated record match the table record in the fields `officials_manager_add_random_officials` writes (+4..+0x10, +0x16..+0x2b)?
pub(crate) unsafe fn same_official_record(g: &[u8; 0x2b], f: usize) -> bool {
    let t = core::slice::from_raw_parts(f as *const u8, 0x2b);
    g[4..0x10] == t[4..0x10] && g[0x16..] == t[0x16..]
}

/// If an attribute byte is outside 1..20, replaces it with the value from f.
pub(crate) unsafe fn fix_official_attribute(e: usize, o: usize, f: impl FnOnce() -> u8) {
    let v = read_u8(e + o) as i8;
    if v <= 0 || v > 0x14 {
        write_u8(e + o, f());
    }
}

/// Repairs each referee record: the nation from the names, out-of-range ratings and attributes drawn again, and a
/// too-old birth date.
///
/// `officials_manager_validate_officials()` (cdecl), at the start of setting up the referees of a new game (`set_up_officials`).
/// For each of the [0xae2390] records (0x2b bytes in [0xae23e8]): without a nation (+0x16), the one from the nation
/// table by the index (+0x37) of the first name (+4), of the second (+8), or a random one; the words +0x22, +0x1e and
/// +0x20 outside 1..200: random(80) + 1 plus the nation's byte +0x8e (+0x22) or 4 times it (+0x1e, +0x20; the redone
/// +0x20 is at least +0x1e); the bytes +0x24..+0x2a outside 1..20 drawn by the bands of +0x1e (above 120, above 80);
/// born before the date `exe::date_date_549f70`(1, 1, 1910, −1) (year unsigned, then the day signed), year 1960 + random(10) and day
/// random(300) + 1.
/// Original: `officials_manager_validate_officials` (0x79caa0, officials_manager.cpp, cdecl).
pub(crate) unsafe extern "C" fn officials_manager_validate_officials() {
    let base = read_usize(G_OFFICIAL_TABLE);
    let mut i = 0i32;
    while i < read_i32(G_OFFICIAL_COUNT) {
        let e = base.wrapping_add((i as usize).wrapping_mul(0x2b));
        if read_usize(e + official::NATION) == 0 {
            let (n1, n2) = (read_usize(e + 4), read_usize(e + 8));
            let k = if read_i32(n1 + names::NATION) >= 0 {
                read_i32(n1 + names::NATION)
            } else if read_i32(n2 + names::NATION) >= 0 {
                read_i32(n2 + names::NATION)
            } else {
                random(read_i32(G_NATION_COUNT))
            };
            write_usize(e + official::NATION, read_usize(G_NATION_TABLE).wrapping_add(k.wrapping_mul(0x122) as usize));
        }
        let away = |o: usize| {
            let v = read_i16(e + o);
            v <= 0 || v > 0xc8
        };
        if away(0x22) {
            let nation = read_usize(e + official::NATION);
            let v = if nation != 0 {
                let s = random(0x50) as u8;
                s.wrapping_add(read_u8(read_usize(e + official::NATION) + nation::REPUTATION)).wrapping_add(1)
            } else {
                (random(0x50) as u8).wrapping_add(1)
            };
            write_i16(e + official::REPUTATION, v as i8 as i16);
        }
        for o in [0x1eusize, 0x20] {
            if away(o) {
                covered!("cpy_9 rating fixed");
                let nation = read_usize(e + official::NATION);
                let v = if nation != 0 {
                    let s = random(0x50) as u8;
                    s.wrapping_add(read_u8(read_usize(e + official::NATION) + nation::REPUTATION) << 2).wrapping_add(1)
                } else {
                    (random(0x50) as u8).wrapping_add(1)
                };
                write_i16(e + o, v as i8 as i16);
                if o == 0x20 && read_i16(e + official::POTENTIAL_ABILITY) < read_i16(e + official::CURRENT_ABILITY) {
                    write_i16(e + official::POTENTIAL_ABILITY, read_i16(e + official::CURRENT_ABILITY));
                }
            }
        }
        let band = |high: (i32, u8), medio: (i32, u8), low: (i32, u8)| {
            let w = read_i16(e + official::CURRENT_ABILITY);
            let (n, k) = if w > 0x78 { high } else if w > 0x50 { medio } else { low };
            random_byte_plus(n, k)
        };
        fix_official_attribute(e, 0x24, || if read_i16(e + official::CURRENT_ABILITY) > 0x64 { random_byte_plus(0xc, 7) } else { random_byte_plus(0xa, 5) });
        fix_official_attribute(e, 0x25, || band((0xe, 7), (0xf, 5), (0xa, 5)));
        fix_official_attribute(e, 0x26, || band((0xb, 0xa), (0xf, 5), (0xa, 5)));
        fix_official_attribute(e, 0x27, || if read_i16(e + official::CURRENT_ABILITY) > 0x50 { random_byte_plus(0xf, 6) } else { random_byte_plus(0x14, 1) });
        fix_official_attribute(e, 0x28, || band((0xa, 0xb), (0xf, 5), (0xa, 7)));
        fix_official_attribute(e, 0x29, || band((0xd, 8), (0xe, 7), (0xf, 6)));
        fix_official_attribute(e, 0x2a, || if random(0x64) != 0 { random_byte_plus(0xa, 0xb) } else { random_byte_plus(0xa, 5) });
        let mut limit = [0u8; 8];
        let d = exe::date_date_549f70((limit.as_mut_ptr()) as *mut u8 as *mut [u16; 4], 1, 1, 0x776, (-1) as i32 as u32) as *const u8;
        let (dia, ano) = (read_i16(d as usize), read_i16(d as usize + 2) as u16);
        let year_and = read_i16(e + official::BIRTH_YEAR) as u16;
        if ano > year_and || (ano == year_and && dia > read_i16(e + official::BIRTH_DAY)) {
            write_i16(e + official::BIRTH_YEAR, random(10).wrapping_add(0x7a8) as i16);
            write_i16(e + official::BIRTH_DAY, random(0x12c).wrapping_add(1) as i16);
        }
        i += 1;
    }
}

/// Copies a 0x2b-byte referee record field by field, the date through the date copy.
///
/// `Official::operator=(src)`, from setting up the referees of a new game (`set_up_officials`, the records that are kept). Copies
/// the 0x2b-byte record field by field (+0, +4, +8; the date at +0xc through `exe::date_operator_3d`; +0x14, +0x16, +0x1a, +0x1e,
/// +0x20, +0x22, the bytes +0x24..+0x2a). Returns `this`.
/// Original: `Official::operator=` (0x79c530, officials_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn officials_manager_operator(this: usize, src: usize) -> usize {
    for o in [0usize, 4, 8] {
        write_usize(this + o, read_usize(src + o));
    }
    let mut t = [0u8; 8];
    exe::date_operator_3d(this + official::BIRTH_DAY, (t.as_mut_ptr()) as usize, src + official::BIRTH_DAY);
    write_i16(this + official::YEAR_OF_BIRTH, read_i16(src + official::YEAR_OF_BIRTH));
    write_usize(this + official::NATION, read_usize(src + official::NATION));
    write_usize(this + official::CITY, read_usize(src + official::CITY));
    for o in [0x1eusize, 0x20, 0x22] {
        write_i16(this + o, read_i16(src + o));
    }
    for o in 0x24usize..0x2b {
        write_u8(this + o, read_u8(src + o));
    }
    this
}

/// Writes the referees and their statistics into the off.dat item of the save.
///
/// `OfficialsManager::save_official_stats(save)`: writes into the `off.dat` item the number of referees [0xae2390], the
/// 0x11-byte records from [+0] (one per referee), the number of nations [0xae2350], the 0x102-byte records from [+0xc]
/// (one per nation) and one byte per referee from [+8]; returns 1 (in the low byte). No container or an error on
/// opening: the "Error" box (officials_manager.cpp 0x142, 0x14a) and 0; no referees or an error on writing: the box
/// (0x150, 0x156, 0x15c, 0x164, 0x16c, 0x174), it closes the item and 0.
/// Original: `OfficialsManager::save_official_stats` (0x79bb80, officials_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn save_official_stats(this: usize, save: usize) -> u8 {
    const FONTE: usize = 0xa6a22c;
    let arbitros = || read_u32(G_OFFICIAL_COUNT);
    let nations = || read_u32(G_NATION_COUNT);
    covered!("cpx_7 save referee stats");
    if save == 0 {
        show_error_at(FONTE, 0x142);
        return 0;
    }
    let mut z = zip_open_item(save, 0xa6a260);
    if z == 0 {
        show_error_at(FONTE, 0x14a);
        return 0;
    }
    if zip_write(G_OFFICIAL_COUNT, 4, 1, z) != 1 {
        return officials_save_error(0x150, &mut z);
    }
    if arbitros() == 0 {
        return officials_save_error(0x156, &mut z);
    }
    if zip_write(read_usize(this), 0x11, arbitros(), z) != arbitros() {
        return officials_save_error(0x15c, &mut z);
    }
    if zip_write(G_NATION_COUNT, 4, 1, z) != 1 {
        return officials_save_error(0x164, &mut z);
    }
    if zip_write(read_usize(this + officials_manager_fields::NATION_BLOCKS), 0x102, nations(), z) != nations() { // cpx7: the records of the nations
        return officials_save_error(0x16c, &mut z);
    }
    if zip_write(read_usize(this + officials_manager_fields::BUSY_FLAGS), 1, arbitros(), z) != arbitros() {
        return officials_save_error(0x174, &mut z);
    }
    zip_close(&mut z);
    1
}

