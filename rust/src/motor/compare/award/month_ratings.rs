use super::*;

// The comparers of the routines in ../award/month_ratings.rs.

/// Comparer for `record_match_ratings`.
pub(crate) unsafe extern "thiscall" fn compare_record_match_ratings(this: usize, game: usize, fixture: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_RECORD_MATCH_RATINGS);
    let (tab, n, cap) = (read_usize(this), read_i32(this + 4), read_i32(this + 8));
    if tab == 0 || n < 0 || cap.wrapping_sub(n) < 40 {
        // simplification: the table may grow (realloc): original only; 0x767fa0 compares each rating
        return o(this, game, fixture);
    }
    let regions = [(this, 12), (tab, (n as usize + 40) * 12)];
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, game, fixture), || record_match_ratings(this, game, fixture));
    SCOREBOARD.record(igual, || format!("record_match_ratings(this {:#x}) original={:x?} rust={:x?}", this, mo, mr));
}

/// Comparer for `add_player_rating`.
pub(crate) unsafe extern "thiscall" fn compare_add_player_rating(this: usize, e: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_ADD_PLAYER_RATING);
    let (tab, n, cap) = (read_usize(this), read_i32(this + 4), read_i32(this + 8));
    let found = tab != 0 && (0..n.max(0) as usize).any(|k| read_i32(tab + k * 12) == read_i32(e));
    if tab == 0 || n < 0 || (n >= cap && !found) {
        // simplification: the table grows (malloc/realloc): original only
        return o(this, e);
    }
    let slots = if n < cap { n as usize + 1 } else { n as usize };
    let regions = [(this, 12), (tab, slots * 12)];
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, e), || add_player_rating(this, e));
    SCOREBOARD.record(igual, || format!("add_player_rating(person {}) original={:x?} rust={:x?}", read_i32(e), mo, mr));
}

/// Comparer for `find_best_player`.
pub(crate) unsafe extern "thiscall" fn compare_find_best_player(this: usize, league: i32, age: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32, u32) -> usize>(IDX_FIND_BEST_PLAYER);
    let (tab, n) = (read_usize(this), read_i32(this + 4));
    let regions: Vec<(usize, usize)> = if tab != 0 && n > 0 { vec![(tab, n as usize * 12)] } else { Vec::new() };
    let (r, m, igual, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, league, age), || find_best_player(this, league, age));
    SCOREBOARD.record(igual && r == m, || format!("find_best_player(league {}, age {}) original={:#x} {:x?} rust={:#x} {:x?}", league, age as u8 as i8, r, mo, m, mr));
    r
}
