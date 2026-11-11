use super::*;

// The comparers of the routines in ../record_utils.rs.

/// Comparer for `fill_match_record_from_fixture`.
pub(crate) unsafe extern "C" fn compare_fill_match_record_from_fixture(game: usize, club: usize, out: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, usize) -> u32>(IDX_FILL_MATCH_RECORD_FROM_FIXTURE);
    if game == 0 || club == 0 || out == 0 {
        return o(game, club, out);
    }
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(out, 0x1f)], || o(game, club, out), || fill_match_record_from_fixture(game, club, out));
    SCOREBOARD.record(igual && r == n, || format!("fill_match_record_from_fixture(game {:x}, club {:x}) original={} rust={} {:x?} x {:x?}", game, club, r, n, mo, mr));
    r
}

/// Comparer for `fill_staff_name_ref`.
pub(crate) unsafe extern "C" fn compare_fill_staff_name_ref(person: usize, out: usize) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(IDX_FILL_STAFF_NAME_REF);
    if person == 0 || out == 0 {
        return o(person, out);
    }
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(out, 0x18)], || o(person, out), || fill_staff_name_ref(person, out));
    SCOREBOARD.record(igual && r == n, || format!("fill_staff_name_ref(person {:x}) original={} rust={} {:x?} x {:x?}", person, r, n, mo, mr));
    r
}

/// Comparer for `match_record_result_code`.
pub(crate) unsafe extern "C" fn compare_match_record_result_code(record: usize) -> u8 {
    let o = original_fn::<unsafe extern "C" fn(usize) -> u8>(IDX_MATCH_RECORD_RESULT_CODE);
    if record == 0 {
        return o(record);
    }
    let r = o(record);
    let n = match_record_result_code(record);
    SCOREBOARD.record(r == n, || format!("match_record_result_code(record {:x}) original={} rust={}", record, r as i8, n as i8));
    r
}

/// Comparer for `match_record_is_bigger_win`.
pub(crate) unsafe extern "C" fn compare_match_record_is_bigger_win(a: usize, b: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(IDX_MATCH_RECORD_IS_BIGGER_WIN)(a, b);
    if a == 0 || b == 0 {
        return r;
    }
    let n = match_record_is_bigger_win(a, b);
    SCOREBOARD.record(r == n, || format!("match_record_is_bigger_win({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `match_record_is_bigger_defeat`.
pub(crate) unsafe extern "C" fn compare_match_record_is_bigger_defeat(a: usize, b: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(IDX_MATCH_RECORD_IS_BIGGER_DEFEAT)(a, b);
    if a == 0 || b == 0 {
        return r;
    }
    let n = match_record_is_bigger_defeat(a, b);
    SCOREBOARD.record(r == n, || format!("match_record_is_bigger_defeat({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}
