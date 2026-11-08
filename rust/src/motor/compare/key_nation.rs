use super::*;

// The comparers of the routines in ../key_nation.rs.
// [pt] Os comparadores das rotinas de ../key_nation.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `key_nation_index_of_nation`.
pub(crate) unsafe extern "C" fn compare_key_nation_index_of_nation(nation: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u8>(IDX_KEY_NATION_INDEX_OF_NATION)(nation);
    let n = key_nation_index_of_nation(nation);
    SCOREBOARD.record(r == n, || format!("key_nation_index_of_nation(nação {:x}) original={} rust={}", nation, r, n));
    r
}

/// Comparer for `key_nation_index_of_club`.
pub(crate) unsafe extern "C" fn compare_key_nation_index_of_club(club: usize) -> u8 {
    let o = original_fn::<unsafe extern "C" fn(usize) -> u8>(IDX_KEY_NATION_INDEX_OF_CLUB);
    if hits_key_nation_error(club) {
        return o(club);
    }
    let r = o(club);
    let n = key_nation_index_of_club(club);
    SCOREBOARD.record(r == n, || format!("key_nation_index_of_club(clube {:x}) original={} rust={}", club, r, n));
    r
}

/// Comparer for `key_nation_get_club_season_year`.
pub(crate) unsafe extern "C" fn compare_key_nation_get_club_season_year(club: usize) -> u16 {
    let o = original_fn::<unsafe extern "C" fn(usize) -> u16>(IDX_KEY_NATION_GET_CLUB_SEASON_YEAR);
    if club != 0 && hits_key_nation_error(club) {
        return o(club);
    }
    let r = o(club);
    let n = key_nation_get_club_season_year(club);
    SCOREBOARD.record(r == n, || format!("key_nation_get_club_season_year(clube {:x}) original={} rust={}", club, r, n));
    r
}

/// Comparer for `key_nation_club_in_season`.
pub(crate) unsafe extern "C" fn compare_key_nation_club_in_season(club: usize, start: usize, end: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize, usize) -> u32>(IDX_KEY_NATION_CLUB_IN_SEASON)(club, start, end);
    let (mut i, mut f) = ([0u8; 8], [0u8; 8]);
    let n = key_nation_club_in_season(
        club,
        if start != 0 { i.as_mut_ptr() as usize } else { 0 },
        if end != 0 { f.as_mut_ptr() as usize } else { 0 },
    );
    let read = |p: usize| if p != 0 { *(p as *const [u8; 8]) } else { [0u8; 8] };
    let (oi, of) = (read(start), read(end));
    let ok = r == n && (start == 0 || oi == i) && (end == 0 || of == f);
    SCOREBOARD.record(ok, || {
        format!("key_nation_club_in_season(clube {:x}) original={} {:02x?} {:02x?} rust={} {:02x?} {:02x?}", club, r, oi, of, n, i, f)
    });
    r
}

/// Comparer for `key_nation_get_main_continent`.
pub(crate) unsafe extern "C" fn compare_key_nation_get_main_continent() -> usize {
    let r = original_fn::<unsafe extern "C" fn() -> usize>(IDX_KEY_NATION_GET_MAIN_CONTINENT)();
    let n = key_nation_get_main_continent();
    SCOREBOARD.record(r == n, || format!("key_nation_get_main_continent() original={:x} rust={:x}", r, n));
    r
}

/// Comparer for `key_nation_get_main_nation`.
pub(crate) unsafe extern "C" fn compare_key_nation_get_main_nation() -> usize {
    let r = original_fn::<unsafe extern "C" fn() -> usize>(IDX_KEY_NATION_GET_MAIN_NATION)();
    let n = key_nation_get_main_nation();
    SCOREBOARD.record(r == n, || format!("key_nation_get_main_nation() original={:x} rust={:x}", r, n));
    r
}

/// Comparer for `key_nation_is_world_cup_comp`.
pub(crate) unsafe extern "C" fn compare_key_nation_is_world_cup_comp(comp: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_KEY_NATION_IS_WORLD_CUP_COMP);
    if comp == 0 {
        return o(comp);
    }
    let r = o(comp);
    let n = key_nation_is_world_cup_comp(comp);
    SCOREBOARD.record(r == n, || format!("key_nation_is_world_cup_comp(competição {}) original={} rust={}", read_usize(comp) as i32, r, n));
    r
}

/// Comparer for `key_nation_is_international_comp`.
pub(crate) unsafe extern "C" fn compare_key_nation_is_international_comp(comp: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_KEY_NATION_IS_INTERNATIONAL_COMP);
    if comp == 0 {
        return o(comp);
    }
    let r = o(comp);
    let n = key_nation_is_international_comp(comp);
    SCOREBOARD.record(r == n, || format!("key_nation_is_international_comp(competição {}) original={} rust={}", read_usize(comp) as i32, r, n));
    r
}

/// Comparer for `key_nation_index_of_comp`.
pub(crate) unsafe extern "C" fn compare_key_nation_index_of_comp(comp: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u8>(IDX_KEY_NATION_INDEX_OF_COMP)(comp);
    let n = key_nation_index_of_comp(comp);
    SCOREBOARD.record(r == n, || format!("key_nation_index_of_comp(competição {}) original={} rust={}", read_usize(comp) as i32, r, n));
    r
}
