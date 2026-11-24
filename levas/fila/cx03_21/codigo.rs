// ---------- cx03_21: the Swedish Cup's club pool, entrants and round tables (swe_cup.cpp) ----------

/// The Swedish Cup's pool of clubs (+0xb2): an array of club pointers, dealt into sections by `swe_cup_build_club_pool`
/// and emptied slot by slot as the entrants and the preliminary stages take them.
pub(crate) const SWE_CUP_CLUB_POOL: usize = 0xb2;
/// The size of the pool (+0xb6, a signed word): 244.
pub(crate) const SWE_CUP_CLUB_POOL_SIZE: usize = 0xb6;

/// The club pool of a Swedish Cup and its size.
pub(crate) unsafe fn swe_cup_pool(this: Cup) -> (usize, i16) {
    (read_usize(this.addr() + SWE_CUP_CLUB_POOL), read_i16(this.addr() + SWE_CUP_CLUB_POOL_SIZE))
}

/// One pass of the Swedish Cup's club pool over the club table: each club `accept`ed (with the count so far) goes to
/// slot `k` of the pool.
unsafe fn swe_cup_pool_pass(pool: usize, k: &mut i32, accept: impl Fn(Club, i32) -> bool) {
    let mut i = 0i32;
    while i < read_i32(G_CLUB_COUNT) {
        let c = Club(eng_cc_cup_club_by_index(i as i16));
        if accept(c, *k) {
            write_usize(pool + *k as usize * 4, c.addr());
            *k += 1;
        }
        i += 1;
    }
}

/// rule: the Swedish Cup's 244 clubs are all the Second Division clubs, the Swedish clubs of the lower divisions (up to
/// 214 in all), the First Division clubs (the last 16 added shuffled) and the Allsvenskan clubs; the first 232 are
/// sorted (`exe::crt_qsort` with 0x53fd40) and dealt in order into eight sections, alternately 30 (with 2 seeds) and 31
/// (with 1 seed) clubs: the unseeded slots from the front of the pool, the seeds at the back of the section, drawn at
/// random among the last 12 of the pool (`exe::utils_random`(12) + 232, again while the slot is taken). The dealt
/// sections become the new pool. Returns 1; out of memory 0.
///
/// `SWE_CUP_CLUB_POOL_SIZE` = 244; the pool (`exe::crt_malloc` of 0x3d0 into `SWE_CUP_CLUB_POOL`; none: the box of
/// line 0x29e of swe_cup.cpp and 0); four passes over the club table: the Second Division (key
/// `KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION`, no size check), the lower division clubs of Sweden (`club.nation()` key
/// `KEY_NATION_SWEDEN` and division `KEY_CLUB_COMP_SWEDISH_LOWER_DIVISION`) while fewer than size − 30, the First
/// Division while fewer than the size, then 16 entries before the end shuffled (`exe::comp_util_shuffle_pointer_array`,
/// cdecl), and the Premier Division while fewer than the size. The sections (`exe::crt_malloc` of 4 per club; none: the
/// box of line 0x2d9 and 0) as above; the old pool freed (`exe::crt_free`) and the sections stored in its place.
/// Original: `SweCup::build_club_pool` (0x88da20, swe_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn swe_cup_build_club_pool(this: usize) -> u32 {
    let this = Cup(this);
    write_i16(this.addr() + SWE_CUP_CLUB_POOL_SIZE, 0xf4);
    let pool = exe::crt_malloc(0x3d0);
    write_usize(this.addr() + SWE_CUP_CLUB_POOL, pool);
    if pool == 0 {
        show_error_at(0xa8b4a0, 0x29e);
        return 0;
    }
    let size = || read_i16(this.addr() + SWE_CUP_CLUB_POOL_SIZE) as i32;
    let in_division = |c: Club, key: usize| division_comp_id(c.addr()) == Some(read_usize(key));
    let mut k = 0i32;
    swe_cup_pool_pass(pool, &mut k, |c, _| in_division(c, KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION));
    swe_cup_pool_pass(pool, &mut k, |c, k| {
        let nation = c.nation().addr();
        nation != 0 && read_usize(nation) == read_usize(KEY_NATION_SWEDEN) && in_division(c, KEY_CLUB_COMP_SWEDISH_LOWER_DIVISION) && k < size() - 0x1e
    });
    swe_cup_pool_pass(pool, &mut k, |c, k| in_division(c, KEY_CLUB_COMP_SWEDISH_FIRST_DIVISION) && k < size());
    exe::comp_util_shuffle_pointer_array(pool + k as usize * 4 - 0x40, 0x10); // the last 16 added
    swe_cup_pool_pass(pool, &mut k, |c, k| in_division(c, KEY_CLUB_COMP_SWEDISH_PREMIER_DIVISION) && k < size());
    exe::crt_qsort(pool, 0xe8, 4, 0x53fd40usize);
    let sections = exe::crt_malloc(size() as u32 * 4);
    if sections == 0 {
        show_error_at(0xa8b4a0, 0x2d9);
        return 0;
    }
    covered!("cx03_21 swedish cup club pool");
    let mut start: i16 = 0;
    for g in 0..8i32 {
        let (count, seeds): (i8, i8) = if g % 2 == 0 { (0x1e, 2) } else { (0x1f, 1) };
        let unseeded = count as i32 - seeds as i32;
        let (mut taken, mut i) = (0i32, 0i32);
        while taken < unseeded && i < size() {
            let club = read_usize(pool + i as usize * 4);
            if club != 0 {
                write_usize(sections + (start as i32 + taken) as usize * 4, club);
                taken += 1;
                write_usize(pool + i as usize * 4, 0);
            }
            i += 1;
        }
        let mut slot = count as i32;
        for _ in 0..seeds {
            let mut r = exe::utils_random(0xc) + 0xe8; // one of the last 12 of the pool
            while read_usize(pool + r as usize * 4) == 0 {
                r = exe::utils_random(0xc) + 0xe8;
            }
            write_usize(sections + (start as i32 + slot - 1) as usize * 4, read_usize(pool + r as usize * 4));
            slot -= 1;
            write_usize(pool + r as usize * 4, 0);
        }
        start = start.wrapping_add(count as i16);
    }
    exe::crt_free(pool);
    write_usize(this.addr() + SWE_CUP_CLUB_POOL, sections);
    1
}

/// rule: the Swedish Cup's own entrants are the first 30 clubs of the pool (the first section), taken out of it.
///
/// `cup::TEAM_COUNT` = 30; the list (`exe::crt_malloc` of 0xb4 into `cup::TEAM_TABLE`; none: the box of line 0x1a4 and
/// −1); entry k the pool's club k with class and status 0, and the pool slot cleared. Returns 1.
/// Original: `SweCup::collect_entrants` (0x88d5a0, swe_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn swe_cup_collect_entrants(this: usize) -> i32 {
    collect_swedish_cup_entrants(Cup(this)).0
}

/// The body of `swe_cup_collect_entrants`: the result and the number of entrants.
unsafe fn collect_swedish_cup_entrants(this: Cup) -> (i32, i32) {
    this.set_team_count(0x1e);
    let list = exe::crt_malloc(0xb4);
    this.set_team_table(list);
    if list == 0 {
        show_error_at(0xa8b4a0, 0x1a4);
        return (-1, 0);
    }
    covered!("cx03_21 swedish cup entrants");
    let pool = read_usize(this.addr() + SWE_CUP_CLUB_POOL);
    let mut k = 0i32;
    while k < this.team_count() as i32 {
        eng_cc_cup_write_entrant(this, k, read_usize(pool + k as usize * 4), 0);
        write_usize(pool + k as usize * 4, 0);
        k += 1;
    }
    (1, k)
}

/// A preliminary section of the Swedish Cup with an even stage: a first round of 30 clubs.
pub(crate) const SWEDISH_CUP_EVEN_SECTION_ROUNDS: [FraSuperCupRound; 2] = [
    cql12r((0xf, 6, 0, 5), (0x1b, 6, 0, 6, 1, 0), 0xa, (1, 0x83, 0), 4, (0x1e, 0xf, 0x1e, 0), (0, 1, 0), (0, 0xffff_ffff, 0)),
    cql12r((0x1c, 6, 0, 0), (0x1b, 7, 0, 2, 2, 0), 0x14, (1, 0x83, 0), 4, (0x10, 8, 1, 0x1e), (0, 1, 0), (0, 0xffff_ffff, 0)),
];

/// A preliminary section of the Swedish Cup with an odd stage: a first round of 28 clubs.
pub(crate) const SWEDISH_CUP_ODD_SECTION_ROUNDS: [FraSuperCupRound; 2] = [
    cql12r((0xf, 6, 0, 5), (0x1b, 6, 0, 6, 1, 0), 0xa, (1, 0x83, 0), 4, (0x1c, 0xe, 0x1c, 0), (0, 1, 0), (0, 0xffff_ffff, 0)),
    cql12r((0x1c, 6, 0, 0), (0x1b, 7, 0, 2, 2, 0), 0x14, (1, 0x83, 0), 4, (0x10, 8, 2, 0x1c), (0, 1, 0), (0, 0xffff_ffff, 0)),
];

/// The Swedish Cup's main draw: six rounds of 64 clubs, from August to the final in May.
pub(crate) const SWEDISH_CUP_MAIN_DRAW_ROUNDS: [FraSuperCupRound; 6] = [
    cql12r((0x1d, 7, 0, 3), (0x19, 8, 0, 2, 2, 0), 0x1e, (1, 0x83, 0), 4, (0x40, 0x20, 0x40, 0), (0, 1, 0), (0, 0xffff_ffff, 0)),
    cql12r((0x1a, 8, 0, 3), (0x1d, 2, 1, 6, 1, 0), 0x28, (1, 0x83, 0), 4, (0x20, 0x10, 0, 0), (0, 0, 0), (0, 0xffff_ffff, 0)),
    cql12r((0x1e, 2, 1, 0), (9, 3, 1, 2, 2, 0), 0x32, (1, 0x83, 3), 4, (0x10, 8, 0, 0), (0, 1, 0), (0, 0xffff_ffff, 0)),
    cql12r((0xa, 3, 1, 3), (0x1e, 3, 1, 2, 2, 0), 0x78, (0, 0x83, 0), 6, (8, 4, 0, 0), (0, 1, 0), (0, 0xffff_ffff, 0)),
    cql12r((1, 4, 1, 3), (7, 4, 1, 2, 1, 0), 0x82, (0, 0x83, 0), 6, (4, 2, 0, 0), (0, 0, 0), (0, 0xffff_ffff, 0)),
    cql12r((8, 4, 1, 3), (0x15, 4, 1, 2, 1, 4), 0x96, (0, 0, 0x83), 6, (2, 1, 0, 0), (0, 2, 7), (0, 0xffff_ffff, 0)),
];

/// rule: each of the Swedish Cup's seven preliminary sections (stages below 7) plays two single-match rounds: 30 clubs
/// (even stages) or 28 (odd stages) on 27/7, and the 16 left on 27/8 with one or two clubs entering there; the main
/// draw (stage 7) has six rounds: 64 clubs on 25/9, then 29/3, 9/4, the quarter-finals on 30/4, the semi-finals on 7/5
/// of the next year and the two-legged final from 21/5; the round prizes a share of the fund (−1); category 4 up to the
/// round of 16, 6 after.
///
/// `create_round_table(stage, &rounds, &extra, &out)` (virtual `comp_vtable::CREATE_SCHEDULE`). Stage (a signed byte)
/// below 7: `out` (if requested) = 0, 2 rounds, the extra label 0x464 + stage, the table (`exe::crt_malloc` of 0xd0;
/// none: the box of line 0xcb and 0) with `SWEDISH_CUP_EVEN_SECTION_ROUNDS` or `SWEDISH_CUP_ODD_SECTION_ROUNDS` by the
/// stage's parity (the remainder of a negative stage is −1: odd). Stage 7: `out` = 0, 6 rounds, extra 0, the table
/// (0x270; the box of line 0x119) with `SWEDISH_CUP_MAIN_DRAW_ROUNDS`. Above: 0, nothing written. Each round through
/// `fra_super_write_cup_round` (the dates, then the fields, as the original).
/// Original: `SweCup::create_round_table` (0x88cf40, swe_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn swe_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let cup = Cup(this);
    let s = stage as u8 as i8;
    let (table, label, row): (&[FraSuperCupRound], i16, u32) = if s < 7 {
        (if s % 2 == 0 { &SWEDISH_CUP_EVEN_SECTION_ROUNDS } else { &SWEDISH_CUP_ODD_SECTION_ROUNDS }, 0x464 + s as i16, 0xcb)
    } else if s == 7 {
        (&SWEDISH_CUP_MAIN_DRAW_ROUNDS, 0, 0x119)
    } else {
        return 0;
    };
    if !out.is_null() {
        *out = 0;
    }
    *rounds = table.len() as i16;
    *extra = label;
    let t = exe::crt_malloc(table.len() as u32 * 0x68);
    if t == 0 {
        show_error_at(0xa8b4a0, row);
        return 0;
    }
    covered!("cx03_21 swedish cup round table");
    let year = cup.year() as i32 as u32;
    for (i, r) in table.iter().enumerate() {
        fra_super_write_cup_round(t, i as u32, r, year);
    }
    t
}

/// Comparer for `swe_cup_build_club_pool`: side by side (the random generator in the compared regions, the copy's draws
/// not counted: `without_counting`); the original's
/// sections kept, the copy's compared with them (the 244 club pointers) and freed.
pub(crate) unsafe extern "thiscall" fn compare_swe_cup_build_club_pool(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    let cup = Cup(this);
    let before = read_usize(this + SWE_CUP_CLUB_POOL);
    let (lo, lr) = (core::cell::Cell::new(0usize), core::cell::Cell::new(0usize));
    let mut regions = crate::rng::STATE_REGIONS.to_vec();
    regions.push((this + SWE_CUP_CLUB_POOL_SIZE, 2));
    regions.push((0xb67a34, 4));
    let (a, b, same, mo, mc) = crate::comparison::side_by_side(&regions, || {
        let r = o(this);
        lo.set(read_usize(this + SWE_CUP_CLUB_POOL));
        write_usize(this + SWE_CUP_CLUB_POOL, before);
        r
    }, || crate::rng::without_counting(|| {
        let r = swe_cup_build_club_pool(this);
        lr.set(read_usize(this + SWE_CUP_CLUB_POOL));
        write_usize(this + SWE_CUP_CLUB_POOL, lo.get());
        r
    }));
    let (lo, lr) = (lo.get(), lr.get());
    let n = 4 * swe_cup_pool(cup).1.max(0) as usize;
    let pools = a == 0 || (lo != 0 && lr != 0 && core::slice::from_raw_parts(lo as *const u8, n) == core::slice::from_raw_parts(lr as *const u8, n));
    if lr != 0 && lr != lo && a != 0 {
        exe::crt_free(lr);
    }
    SCOREBOARD.record(same && a == b && pools, || format!("swe_cup_build_club_pool({:#x}) returned {} x {} memory {} pools {} {:x?} {:x?}", this, a, b, same, pools, mo, mc));
    a
}

/// Comparer for `swe_cup_collect_entrants`: side by side over the team count and the first 30 pool slots (which it
/// clears), the return and the list (`eng_cc_cup_compare_entrant_list`, the copy's list freed).
pub(crate) unsafe extern "thiscall" fn compare_swe_cup_collect_entrants(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I1%%);
    let cup = Cup(this);
    let pool = read_usize(this + SWE_CUP_CLUB_POOL);
    eng_cc_cup_compare_entrant_list("swe_cup_collect_entrants", cup, &[(this + cup::TEAM_COUNT, 2), (pool, 0x78)], || o(this),
        || collect_swedish_cup_entrants(cup), |_, n| 6 * n.max(0) as usize)
}

/// Comparer for `swe_cup_create_round_table`: side by side for every stage (the table bytes the rounds write, the
/// outputs and the return); the copy's table freed. Unlike the single-stage cups, every stage up to 7 builds a table.
pub(crate) unsafe extern "thiscall" fn compare_swe_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I2%%);
    if (stage as u8 as i8) > 7 {
        let a = o(this, stage, rounds, extra, out);
        SCOREBOARD.record(a == 0, || format!("swe_cup_create_round_table(stage {}) returned {:#x}", stage as u8 as i8, a));
        return a;
    }
    let (mut r2, mut e2) = (*rounds, *extra);
    let mut s2 = if out.is_null() { 0 } else { *out };
    let (a, b, same, _, _) = crate::comparison::side_by_side(&[(0xb67a34usize, 4usize)], || o(this, stage, rounds, extra, out),
        || swe_cup_create_round_table(this, stage, &mut r2, &mut e2, if out.is_null() { core::ptr::null_mut() } else { &mut s2 }));
    let outs = *rounds == r2 && *extra == e2 && (out.is_null() || *out == s2);
    let table = if a != 0 && b != 0 { eng_cc_cup_written_round_bytes(a, *rounds) == eng_cc_cup_written_round_bytes(b, r2) } else { a == 0 && b == 0 };
    if b != 0 {
        exe::crt_free(b);
    }
    SCOREBOARD.record(same && outs && table, || format!("swe_cup_create_round_table(stage {}): outs {} ({} x {}) table {}", stage as u8 as i8, outs, *rounds, r2, table));
    a
}
