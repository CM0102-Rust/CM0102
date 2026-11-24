// --- sx16_5: the officials' statistics rows read by the screens (officials_manager.cpp) ---

/// The 17-byte statistics row of an official (`officials_manager_fields::GRADE_ROWS` + id × 17).
unsafe fn official_stats_row(this: usize, official: usize) -> usize {
    read_usize(this + officials_manager_fields::GRADE_ROWS) + read_i32(official) as usize * OFFICIAL_STATS_SIZE
}

/// Clears the statistics row of one official: the cards (`official_stats::CARDS`), the fouls, the grade sum, the
/// matches and the average grade; the last grade stays. Nothing without an official.
/// Original: `OfficialsManager::reset_official_stats` (0x79d190, officials_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn reset_official_stats(this: usize, official: usize) {
    if official == 0 {
        return;
    }
    let row = official_stats_row(this, official);
    write_u8(row + official_stats::CARDS, 0);
    write_u16(row + official_stats::FOULS, 0);
    write_u16(row + official_stats::GRADE_SUM, 0);
    write_u16(row + official_stats::MATCHES, 0);
    write_u32(row + official_stats::AVERAGE_GRADE, 0);
}

/// Comparer for `reset_official_stats`: side by side over the official's row.
pub(crate) unsafe extern "thiscall" fn compare_reset_official_stats(this: usize, official: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%);
    if official == 0 {
        return o(this, official);
    }
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&[(official_stats_row(this, official), OFFICIAL_STATS_SIZE)], || o(this, official), || reset_official_stats(this, official));
    SCOREBOARD.record(same, || format!("OfficialsManager::reset_official_stats(official {}): differences {:?}", read_i32(official), first_differences(&mo, &mr)));
}

/// The officials' range of a league competition in its nation's block: the competition's object
/// (`G_COMP_OBJECT_TABLE`) must exist, not be of types 2 or 4 (`cup::CLUB_SIDES`) and be a league division
/// (`comp_util_is_league_division`). The block of the competition's nation (`competition::NATION`; without one, a
/// random nation: `utils_random(G_NATION_COUNT)`); the first of its competition slots (`COMP_COUNT` of them) with the
/// competition's id, else the whole block (`FIRST_REFEREE`, `REFEREE_COUNT`): (first, count). None for no competition.
unsafe fn comp_officials_range(this: usize, comp: usize) -> Option<(i32, i32)> {
    if comp == 0 {
        return None;
    }
    let id = read_i32(comp);
    let object = read_usize(read_usize(G_COMP_OBJECT_TABLE) + id as usize * 4);
    if object == 0 {
        return None;
    }
    let kind = read_u8(object + cup::CLUB_SIDES);
    if kind == 4 || kind == 2 || exe::comp_util_is_league_division(id) == 0 {
        return None;
    }
    let nation = read_usize(comp + competition::NATION);
    let nation_id = if nation != 0 { read_i32(nation) } else { friendly_random(read_i32(G_NATION_COUNT)) };
    let block = nation_officials_block(this, nation_id);
    let slots = read_i16(block + officials_nation_block::COMP_COUNT) as i32;
    let slot = (0..slots.max(0) as usize).map(|k| block + officials_nation_block::COMP_SLOTS + k * 0x10)
        .find(|&s| read_i32(s + 0xc) == id); // the slot's competition id at +0xc
    Some(match slot {
        Some(s) => (read_i32(s + officials_comp_slot::FIRST), read_i32(s + officials_comp_slot::SHARE)),
        None => (read_i32(block + officials_nation_block::FIRST_REFEREE), read_i32(block + officials_nation_block::REFEREE_COUNT)),
    })
}

/// Copies the statistics rows of the officials who referee a league competition (`comp_officials_range`) into `out`
/// (17 bytes each) and returns how many (a word; 0 for no competition or a non-positive count).
/// Original: `OfficialsManager::copy_comp_official_stats` (0x79d1f0, officials_manager.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn copy_comp_official_stats(this: usize, comp: usize, out: usize) -> i16 {
    let Some((first, count)) = comp_officials_range(this, comp) else { return 0 };
    covered!("sx16_5 copy the officials' rows");
    let rows = read_usize(this + officials_manager_fields::GRADE_ROWS);
    for k in 0..count.max(0) as usize {
        let from = rows + (first as usize + k) * OFFICIAL_STATS_SIZE;
        core::ptr::copy_nonoverlapping(from as *const u8, (out + k * OFFICIAL_STATS_SIZE) as *mut u8, OFFICIAL_STATS_SIZE);
    }
    count as i16
}

/// Comparer for `copy_comp_official_stats`: side by side over the output (the count the copy gives, 17 bytes each) and
/// the generator (a competition without a nation draws one).
pub(crate) unsafe extern "thiscall" fn compare_copy_comp_official_stats(this: usize, comp: usize, out: usize) -> i16 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> i16>(%%I1%%);
    // the row count, without leaving a trace on the generator (a competition without a nation draws one)
    let rng = crate::comparison::MemorySnapshot::read(&crate::rng::STATE_REGIONS);
    let n = crate::rng::without_counting(|| comp_officials_range(this, comp)).map(|(_, c)| c.max(0) as usize).unwrap_or(0);
    rng.write_back();
    let mut regions = crate::rng::STATE_REGIONS.to_vec();
    regions.push((out, n.max(1) * OFFICIAL_STATS_SIZE));
    let (r, c, same, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, comp, out), || copy_comp_official_stats(this, comp, out));
    SCOREBOARD.record(same && r == c, || format!("OfficialsManager::copy_comp_official_stats: {} x {}, differences {:?}", r, c, first_differences(&mo, &mr)));
    r
}

/// How many officials referee a league competition (`comp_officials_range`), at most 100; 0 for no competition.
/// Original: `OfficialsManager::get_comp_official_count` (0x79d300, officials_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn get_comp_official_count(this: usize, comp: usize) -> i16 {
    match comp_officials_range(this, comp) {
        Some((_, count)) => count.min(0x64) as i16,
        None => 0,
    }
}

/// Comparer for `get_comp_official_count`: the same count and the same generator state.
pub(crate) unsafe extern "thiscall" fn compare_get_comp_official_count(this: usize, comp: usize) -> i16 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> i16>(%%I2%%);
    let (r, n, same, _, _) = crate::comparison::side_by_side(&crate::rng::STATE_REGIONS, || o(this, comp), || get_comp_official_count(this, comp));
    SCOREBOARD.record(same && r == n, || format!("OfficialsManager::get_comp_official_count: original={} rust={}", r, n));
    r
}

/// An official's last match grade (`official_stats::LAST_GRADE`), 7 when none is recorded (0xff), 6 without an
/// official.
/// Original: `OfficialsManager::get_last_match_rating` (0x79d3b0, officials_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn official_last_match_rating(this: usize, official: usize) -> u8 {
    if official == 0 {
        return 6;
    }
    match read_u8(official_stats_row(this, official) + official_stats::LAST_GRADE) {
        0xff => 7,
        g => g,
    }
}

/// Comparer for `official_last_match_rating`: the same grade.
pub(crate) unsafe extern "thiscall" fn compare_official_last_match_rating(this: usize, official: usize) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(%%I3%%)(this, official);
    let n = official_last_match_rating(this, official);
    SCOREBOARD.record(r == n, || format!("OfficialsManager::get_last_match_rating: original={} rust={}", r, n));
    r
}

/// An official's average grade (`official_stats::AVERAGE_GRADE`), 6.0 (`rdata::F32_6`) without an official.
/// Original: `OfficialsManager::get_average_rating` (0x79d3e0, officials_manager.cpp, thiscall, ret 4, the answer in
/// st(0)).
pub(crate) unsafe extern "thiscall" fn official_average_rating(this: usize, official: usize) -> f32 {
    if official == 0 {
        return read_f32(rdata::F32_6);
    }
    read_f32(official_stats_row(this, official) + official_stats::AVERAGE_GRADE)
}

/// Comparer for `official_average_rating`: the same value, bit for bit.
pub(crate) unsafe extern "thiscall" fn compare_official_average_rating(this: usize, official: usize) -> f32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> f32>(%%I4%%)(this, official);
    let n = official_average_rating(this, official);
    SCOREBOARD.record(r.to_bits() == n.to_bits(), || format!("OfficialsManager::get_average_rating: original={} rust={}", r, n));
    r
}
