// ---------- cx01_8: the orders of the relegation average rows (comp.cpp, used by the Argentine leagues) ----------

/// The relegation average row (0x13 bytes): the club +0, the points average +4 (float) and, from +0xa, the season
/// columns 3 bytes apart, each beginning with its points word.
const AVERAGE_ROW_POINTS_AVERAGE: usize = 0x4;
const AVERAGE_ROW_SEASON_POINTS: usize = 0xa;
/// The season column the by-season order compares (a signed byte the Argentine second division sets to 2).
const AVERAGE_SORT_SEASON: usize = 0xadae04;

/// qsort order of two relegation average rows: the higher points average first; an unordered pair (NaN) or a lower
/// average after; equal averages by the club pointer, ascending (the difference of the pointers).
/// Original: `comp_compare_average_rows` (0x48f410, comp.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_compare_average_rows(a: *const usize, b: *const usize) -> i32 {
    let (a, b) = (a as usize, b as usize);
    let (x, y) = (read_f32(a + AVERAGE_ROW_POINTS_AVERAGE), read_f32(b + AVERAGE_ROW_POINTS_AVERAGE));
    if x > y {
        return -1;
    }
    if !(x >= y) {
        return 1;
    }
    covered!("cx01_8 averages equal");
    read_u32(a).wrapping_sub(read_u32(b)) as i32
}

/// qsort order of two relegation average rows by the points of one season column (`AVERAGE_SORT_SEASON`, 0..3; any
/// other answers 0): more points first, equal points by the club pointer, ascending.
/// Defect of the original: column 3 reads the word at +0x13, past the 0x13-byte row (the next row's first bytes).
/// Original: `comp_compare_average_rows_by_season` (0x48f450, comp.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_compare_average_rows_by_season(a: *const usize, b: *const usize) -> i32 {
    let (a, b) = (a as usize, b as usize);
    let season = read_i8(AVERAGE_SORT_SEASON);
    if !(0..=3).contains(&season) {
        return 0;
    }
    let column = AVERAGE_ROW_SEASON_POINTS + 3 * season as usize;
    let (x, y) = (read_i16(a + column), read_i16(b + column));
    if x > y {
        return -1;
    }
    if x < y {
        return 1;
    }
    covered!("cx01_8 season points equal");
    read_u32(a).wrapping_sub(read_u32(b)) as i32
}

compare_order!(compare_comp_compare_average_rows, comp_compare_average_rows, %%I0%%);
compare_order!(compare_comp_compare_average_rows_by_season, comp_compare_average_rows_by_season, %%I1%%);
