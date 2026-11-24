// ---------- cx01_9: the order of clubs by their last division (comp_util.cpp, used by the Turkish cup) ----------

/// qsort order of two club pointers by their last season: a club without a last division goes after one with it (two
/// without are equal); between two with one, the division of higher reputation first, then the higher competition id,
/// then the better (lower) last position, by the differences of those values.
/// Original: `compare_clubs_by_last_division` (0x4c0a40, comp_util.cpp, cdecl).
pub(crate) unsafe extern "C" fn compare_clubs_by_last_division(a: *const usize, b: *const usize) -> i32 {
    let (ca, cb) = (Club(read_usize(a as usize)), Club(read_usize(b as usize)));
    let (da, db) = (ca.last_division(), cb.last_division());
    match (da.is_null(), db.is_null()) {
        (true, true) => 0,
        (true, false) => 1,
        (false, true) => -1,
        (false, false) => {
            let by_reputation = db.reputation() as i32 - da.reputation() as i32;
            if by_reputation != 0 {
                return by_reputation;
            }
            let by_id = read_u32(db.addr()).wrapping_sub(read_u32(da.addr())) as i32;
            if by_id != 0 {
                return by_id;
            }
            covered!("cx01_9 same last division");
            ca.last_position() as i8 as i32 - cb.last_position() as i8 as i32
        }
    }
}

compare_order!(compare_compare_clubs_by_last_division, compare_clubs_by_last_division, %%I0%%);
