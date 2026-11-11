use super::*;

/// Counts the results in the national side's history between two months (year × 12 + month), from the end, for the work
/// permit.
///
/// Ret 0x14: `FIFA_RANKINGS::count_results_in_period(nation, month_from, year_from, month_to, year_to)` (used by the
/// work permit `is_work_permit_granted`, with the two years up to today): in the nation's entry (id `[nation]`, 0x2c
/// bytes in table `[this]`), the history (+4, 5-byte entries: points, month +2, year +3; count
/// `fifa_ranking_entry::HISTORY_COUNT`) from the end to the start; a year × 12 + month before the start of the period
/// stops the search, up to the end of the period each one counts. In 16 bits. TapeCall on every permit that reaches the
/// national side matches: no counter.
/// Original: `FIFA_RANKINGS::count_results_in_period` (0x58d710, fifa_rankings.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn count_results_in_period(this: usize, nation: usize, month_of: u32, year_of: u32, month_until: u32, year_until: u32) -> u16 {
    let of = (year_of & 0xffff) as i32 * 12 + month_of as u8 as i8 as i32;
    let ate = (year_until & 0xffff) as i32 * 12 + month_until as u8 as i8 as i32;
    let e = read_usize(this).wrapping_add((read_usize(nation) as u32).wrapping_mul(0x2c) as usize);
    let hist = read_usize(e + 4);
    let mut j = read_i16(e + fifa_ranking_entry::HISTORY_COUNT).wrapping_sub(1);
    let mut n = 0u16;
    while j >= 0 {
        let p = hist.wrapping_add(j as isize as usize * 5);
        let v = read_u16(p + 3) as i32 * 12 + read_u8(p + 2) as i32;
        if v < of {
            break;
        }
        if v <= ate { // cpt1: ate o mes final
            n = n.wrapping_add(1); // cpt1: one more match
        }
        j = j.wrapping_sub(1);
    }
    n // cpt1: the matches in the period
}

/// Updates the FIFA rankings on the 18th: drops the results older than a year, sorts the histories, rolls the yearly
/// totals and redoes scores and positions.
///
/// Ret 4: `FIFA_RANKINGS::update_rankings(date)` (from the national sides' day `national_teams_daily_update`): only on
/// the 18th (`exe::date_get_day_of_month`). Stores the date in +5 (`exe::date_operator_3d`) and, in each nation
/// (`[G_NATION_COUNT]` entries), removes from the history the results older than a year (year + 1 before today's, or
/// the same year with the month before today's, `exe::date_get_month`), putting the last one in its place; the loop
/// goes from the low byte of the count (`fifa_ranking_entry::HISTORY_COUNT`) − 1 down to 0, as in the original. Then
/// sorts the history by points (qsort `exe::crt_qsort` with `compare_result_points_desc`). In the month stored in +4,
/// with the year different from `[G_GAME_START_DATE_YEAR]`, the yearly totals move one slot
/// (`fifa_ranking_entry::YEAR_TOTALS` → +0x28) and `YEAR_TOTALS` is zeroed. Finally the scores
/// (`exe::fifa_rankings_recalculate_scores`) and the positions: the order (`exe::fifa_rankings_collect_ranked_order`)
/// gives position + 1 in `fifa_ranking_entry::POSITION`, with the previous one in
/// `fifa_ranking_entry::PREVIOUS_POSITION`; the order vector is freed (`exe::crt_free`).
/// Original: `FIFA_RANKINGS::update_rankings` (0x58d460, fifa_rankings.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn update_rankings(this: usize, data: usize) {
    if (exe::date_get_day_of_month(data) as u16) != 0x12 {
        return;
    }
    let month = exe::date_get_month(data);
    let mut stored = [0u8; 8];
    exe::date_operator_3d(this + 5, stored.as_mut_ptr() as usize, data);
    let year = read_u16(data + 2) as i32;
    let mut i = 0i32;
    while i < read_usize(G_NATION_COUNT) as i32 {
        let e = read_usize(this) + i as usize * 0x2c;
        let mut k = read_u8(e + fifa_ranking_entry::HISTORY_COUNT).wrapping_sub(1) as i8 as i32;
        while k >= 0 {
            let hist = read_usize(e + 4);
            let p = hist + k as usize * 5;
            let y = read_u16(p + 3) as i32 + 1;
            if y < year || (y == year && *((p + 2) as *const u8) < month) {
                let last = hist + read_i16(e + fifa_ranking_entry::HISTORY_COUNT) as isize as usize * 5 - 5;
                core::ptr::copy(last as *const u8, p as *mut u8, 5);
                write_i16(e + fifa_ranking_entry::HISTORY_COUNT, read_i16(e + fifa_ranking_entry::HISTORY_COUNT).wrapping_sub(1));
            }
            k -= 1;
        }
        exe::crt_qsort(read_usize(e + 4), read_i16(e + fifa_ranking_entry::HISTORY_COUNT) as i32 as u32, 5, 0x58e220);
        i += 1;
    }
    if month == read_u8(this + 4) && read_u16(data + 2) != read_u16(G_GAME_START_DATE_YEAR) {
        let mut i = 0i32;
        while i < read_usize(G_NATION_COUNT) as i32 {
            let e = read_usize(this) + i as usize * 0x2c;
            for o in [0x24usize, 0x20, 0x1c, 0x18, 0x14] {
                write_u32(e + o + 4, read_u32(e + o));
            }
            write_u32(e + fifa_ranking_entry::YEAR_TOTALS, 0);
            i += 1;
        }
    }
    exe::fifa_rankings_recalculate_scores(this);
    // the original keeps the count in the argument’s place (the date): without the vector, the count stays as the pointer
    let mut n = data as i32;
    let v = exe::fifa_rankings_collect_ranked_order(this, &mut n);
    let mut k = 0i32;
    while k < n {
        let r = read_usize(v + k as usize * 4);
        write_u16(r + fifa_ranking_entry::PREVIOUS_POSITION, read_u16(r + fifa_ranking_entry::POSITION));
        write_u16(r + fifa_ranking_entry::POSITION, (k + 1) as u16);
        k += 1;
    }
    exe::crt_free(v);
}

/// qsort order of the histories: the points of b minus those of a (highest first)
/// Original: `compare_result_points_desc` (0x58e220, fifa_rankings.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_result_points_desc(a: usize, b: usize) -> i32 {
    read_i16(b) as i32 - read_i16(a) as i32
}

/// Nation's entry in the FIFA ranking table [this] (0x2c bytes each).
pub(crate) unsafe fn ranking_entry(this: usize, nation: usize) -> usize {
    read_usize(this).wrapping_add((read_usize(nation) as u32).wrapping_mul(0x2c) as usize)
}

/// Month and year of a match: Date from day of year (+0x2a) and year (+0x28), then its month.
pub(crate) unsafe fn match_month_year(fixture: Fixture) -> (u8, u16) {
    let mut d = [0u8; 8];
    let p = exe::new_date((d.as_mut_ptr()) as *mut u8 as usize, fixture.day() as u32, fixture.year() as u32);
    (exe::date_get_month(p), fixture.year())
}

/// Appends the points, month and year of the match to the national side's history, growing the array by 5 entries when
/// full.
///
/// Ret 0xc; SEH only for the local date, with an empty destructor: `FIFA_RANKINGS::append_result(nation, points,
/// fixture)` (from the international result `exe::fifa_rankings_record_match_result`, for each national side): with the count (+0xa) equal to the
/// capacity (+8), the capacity grows by 5 and the history (+4) is reallocated (`exe::crt_realloc`) or allocated (`exe::crt_malloc`) with
/// capacity × 5 bytes. The new entry, in the place of the old count: the points (16 bits), the match month (+2) and the
/// year (+3).
/// Original: `FIFA_RANKINGS::append_result` (0x58dea0, fifa_rankings.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn append_result(this: usize, nation: usize, points: u32, fixture: usize) {
    let e = ranking_entry(this, nation);
    if read_u16(e + fifa_ranking_entry::HISTORY_COUNT) == read_u16(e + 8) {
        write_u16(e + 8, read_u16(e + 8).wrapping_add(5));
        let h = read_usize(e + 4);
        let n = read_i16(e + 8) as i32 * 5;
        let new = if h != 0 {
            exe::crt_realloc(h, n as u32)
        } else {
            exe::crt_malloc(n as u32)
        };
        write_usize(e + 4, new);
    }
    let c = read_i16(e + fifa_ranking_entry::HISTORY_COUNT);
    write_i16(e + fifa_ranking_entry::HISTORY_COUNT, c.wrapping_add(1)); // cpt2: mais um resultado
    let p = read_usize(e + 4).wrapping_add(c as isize as usize * 5);
    write_u16(p, points as u16);
    let (mes, ano) = match_month_year(Fixture(fixture));
    write_u8(p + 2, mes); // cpt2: the match month
    write_u16(p + 3, ano); // cpt2: the match year
    covered!("cpt_2 result");
}

// simplification: the original may reallocate the history (does not run twice): the plan beforehand (count, capacity, points, month and
// year), the original runs and the new entry is checked; the copy takes the branch.

/// Returns the national side's strength tier from 1 to 4, by reputation and ranking position.
///
/// Ret 4: `FIFA_RANKINGS::get_nation_strength_tier(nation)` (twice per international result
/// `exe::fifa_rankings_record_match_result`): reputation (`reputation()`) below 3000 and position
/// (`fifa_ranking_entry::POSITION` of the entry, `ranking_entry`) below 50: 3; reputation below 6000 and position below
/// 15: 2; otherwise by position: below 21, 1; below 61, 2; below 101, 3; else 4.
/// Original: `FIFA_RANKINGS::get_nation_strength_tier` (0x58e190, fifa_rankings.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn get_nation_strength_tier(this: usize, nation: usize) -> u8 {
    let nation = Nation(nation);
    let rep = nation.reputation();
    let pos = || read_i16(ranking_entry(this, nation.addr()) + fifa_ranking_entry::POSITION);
    if rep < 0xbb8 && pos() < 0x32 {
        return 3;
    }
    if rep < 0x1770 && pos() < 0xf {
        return 2;
    }
    let p = pos();
    if p < 0x15 {
        1
    } else if p < 0x3d {
        2
    } else if p >= 0x65 {
        4
    } else {
        3
    }
}

/// Recalculates the scores: the year total from the best eight results and the score from the totals of this year and
/// the five before.
///
/// `FIFA_RANKINGS::recalculate_scores()` (from the update of the 18th `update_rankings`): in each nation, the sum of
/// the points of the first eight results (the history comes sorted) and that of all; with more than eight, the year
/// total (`fifa_ranking_entry::YEAR_TOTALS`) is (all × 8 / count + eight) / 2, otherwise the eight. If the date of
/// `[G_GAME_START_DATE]` is after today (`G_CURRENT_DATE`) minus 365 days (`exe::date_sub_days`), with fewer than nine
/// results (`fifa_ranking_entry::HISTORY_COUNT`) the total is (+0x18 + eight) / (count + 8) × 8 (with no results,
/// +0x18). The score (`fifa_ranking_entry::SCORE`, f32): (`YEAR_TOTALS` + 5/6 of +0x18 + 4/6 of +0x1c + 3/6 of +0x20 +
/// 2/6 of +0x24 + 1/6 of +0x28, each part truncated) × `rdata::F64_0_001`.
/// Original: `FIFA_RANKINGS::recalculate_scores` (0x58dfc0, fifa_rankings.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn recalculate_scores(this: usize) {
    let factor = read_f64(rdata::F64_0_001);
    let mut i = 0i32;
    if read_usize(G_NATION_COUNT) as i32 <= 0 {
        return;
    }
    loop {
        let e = read_usize(this) + i as usize * 0x2c;
        let count = read_i16(e + fifa_ranking_entry::HISTORY_COUNT);
        let point = |k: i8| read_i16(read_usize(e + 4).wrapping_add(k as isize as usize * 5)) as i32;
        let mut oito = 0i32;
        let mut k = 0i8;
        if count > 0 {
            loop {
                if k >= 8 {
                    break;
                }
                oito = oito.wrapping_add(point(k));
                k = k.wrapping_add(1);
                if k as i16 >= count {
                    break;
                }
            }
        }
        let mut all = oito;
        while (k as i16) < count {
            all = all.wrapping_add(point(k));
            k = k.wrapping_add(1);
        }
        let year = if count > 8 { (all.wrapping_mul(8) / count as i32 + oito) / 2 } else { oito };
        write_i32(e + fifa_ranking_entry::YEAR_TOTALS, year);
        let base = core::ptr::read_unaligned(G_GAME_START_DATE as *const [u8; 8]);
        let mut d = [0u8; 8];
        let r = exe::date_sub_days(G_CURRENT_DATE, (&mut d) as *mut [u8; 8] as usize, 0x16d);
        let (by, ry) = (u16::from_le_bytes([base[2], base[3]]), read_u16(r + 2));
        let after = by > ry || (by == ry && i16::from_le_bytes([base[0], base[1]]) > read_i16(r));
        if after && read_i16(e + fifa_ranking_entry::HISTORY_COUNT) < 9 {
            let c = read_i16(e + fifa_ranking_entry::HISTORY_COUNT);
            let v18 = read_i32(e + 0x18);
            write_i32(e + fifa_ranking_entry::YEAR_TOTALS, if c != 0 { (v18.wrapping_add(oito) / (c as i32 + 8)).wrapping_shl(3) } else { v18 });
        }
        let t = |o: usize| read_i32(e + o);
        let sum = (t(fifa_ranking_entry::YEAR_TOTALS + 0x10).wrapping_mul(2) / 6)
            .wrapping_add(t(fifa_ranking_entry::YEAR_TOTALS + 0xc).wrapping_mul(3) / 6)
            .wrapping_add(t(fifa_ranking_entry::YEAR_TOTALS + 0x8).wrapping_mul(4) / 6)
            .wrapping_add(t(fifa_ranking_entry::YEAR_TOTALS + 0x4).wrapping_mul(5) / 6)
            .wrapping_add(t(fifa_ranking_entry::YEAR_TOTALS + 0x14) / 6)
            .wrapping_add(t(fifa_ranking_entry::YEAR_TOTALS));
        write_f32(e + fifa_ranking_entry::SCORE, (sum as f64 * factor) as f32);
        i += 1;
        if i >= read_usize(G_NATION_COUNT) as i32 {
            break;
        }
    }
}

/// Returns a new array with the ranking entries of the nations that have a national side, sorted by score, and the
/// count.
///
/// Ret 4: `FIFA_RANKINGS::collect_ranked_order(&count)` (from the update of the 18th `update_rankings` and from the
/// ranking screen): a vector (`exe::crt_malloc`) of `[G_NATION_COUNT]` pointers; without it, the "Error" box (line
/// 0x1e3) and 0, without storing the count. The table entries (0x2c bytes) of the nations with a national side
/// (`nation::CONTINENT` of the nation, `nation::SIZE`-byte records in `[G_NATION_TABLE]`), sorted by score (qsort
/// `exe::crt_qsort` with `compare_ranking_score_desc`); the count goes in `count`.
/// Original: `FIFA_RANKINGS::collect_ranked_order` (0x58d610, fifa_rankings.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn collect_ranked_order(this: usize, count: *mut i32) -> usize {
    let v = exe::crt_malloc((read_usize(G_NATION_COUNT) as u32).wrapping_mul(4));
    if v == 0 {
        show_error_at(0x9c74f4, 0x1e3);
        return 0;
    }
    let nations = read_usize(G_NATION_TABLE);
    let mut n = 0usize;
    let mut i = 0usize;
    while (i as i32) < read_usize(G_NATION_COUNT) as i32 {
        if read_usize(nations + i * nation::SIZE + nation::CONTINENT) != 0 {
            write_usize(v + n * 4, read_usize(this) + i * 0x2c);
            n += 1;
        }
        i += 1;
    }
    exe::crt_qsort(v, n as u32, 4, 0x58e240);
    *count = n as i32;
    v
}

// simplification: both allocate the vector; the original runs first (its vector goes back to the caller), the copy after, and the
// the copy’s vector is freed (0x9452ca) after checking the count and the pointers.

/// qsort order of the ranking: the highest score first
/// Original: `compare_ranking_score_desc` (0x58e240, fifa_rankings.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_ranking_score_desc(a: usize, b: usize) -> i32 {
    let fa = read_f32(read_usize(a) + fifa_ranking_entry::SCORE);
    let fb = read_f32(read_usize(b) + fifa_ranking_entry::SCORE);
    if !(fa >= fb) {
        1
    } else if fa == fb {
        0
    } else {
        -1
    }
}

/// Creates the initial FIFA ranking table. Rule: each nation's points are its database strength ÷ 35, repeated as the
/// points of each of the 6 years [inference] (× 1000); the position comes from the points order.
///
/// Ret 0, `FIFA_RANKINGS::create_default_table`: the initial FIFA ranking table (0x2c-byte records, one per nation,
/// calloc `exe::crt_calloc` of `[G_NATION_COUNT]` × 0x2c in +0; out of memory: the box at line 0x2da). Each record: +0
/// the nation index; the nation's strength (`nation::FIFA_COEFFICIENT`, double, 0x122-byte nations in
/// `[G_NATION_TABLE]`) × 1/35 (`rdata::F64_0_02857142857`) gives the points (`fifa_ranking_entry::SCORE`, float) and, ×
/// 1000 (`rdata::F64_1000`, truncated), the 6 dwords `fifa_ranking_entry::YEAR_TOTALS`..+0x28 [inference: the points of
/// each of the last years]. Then the order by points (`exe::fifa_rankings_collect_ranked_order`(this, &count),
/// thiscall, the sorted list of the nations with a continent) gives the position:
/// `fifa_ranking_entry::PREVIOUS_POSITION` = the previous position (`fifa_ranking_entry::POSITION`) and `POSITION` = 1,
/// 2, 3...; the list is freed.
/// Original: `FIFA_RANKINGS::create_default_table` (0x58dd50, fifa_rankings.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn create_default_table(this: usize) {
    let t = exe::crt_calloc(read_u32(G_NATION_COUNT), 0x2c);
    write_usize(this, t);
    if t == 0 {
        show_error_at(0x9c74f4, 0x2da);
        return;
    }
    let c1 = read_f64(rdata::F64_0_02857142857);
    let c2 = read_f64(rdata::F64_1000);
    let mut k = 0i32;
    while k < read_u32(G_NATION_COUNT) as i32 {
        let r = t + 0x2c * k as usize;
        write_i32(r, k);
        let a = read_f64(read_u32(G_NATION_TABLE) as usize + 0x122 * k as usize + nation::FIFA_COEFFICIENT) * c1;
        let p = (a * c2) as i32;
        for j in 0..6 {
            write_i32(r + fifa_ranking_entry::YEAR_TOTALS + 4 * j, p);
        }
        write_f32(r + fifa_ranking_entry::SCORE, a as f32);
        k += 1;
    }
    let mut n = 0i32;
    let l = exe::fifa_rankings_collect_ranked_order(this, &mut n);
    for i in 0..n.max(0) as usize {
        let r = read_u32(l + 4 * i) as usize;
        write_u16(r + fifa_ranking_entry::PREVIOUS_POSITION, read_u16(r + fifa_ranking_entry::POSITION));
        write_u16(r + fifa_ranking_entry::POSITION, (i + 1) as u16);
    }
    exe::crt_free(l);
}

