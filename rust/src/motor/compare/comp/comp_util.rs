use super::*;

// The comparers of the routines in ../comp/comp_util.rs.

/// Comparer for `comp_util_sort_clubs_by_strength`.
pub(crate) unsafe extern "C" fn compare_comp_util_sort_clubs_by_strength(v: usize, n: i32) {
    let o = original_fn::<unsafe extern "C" fn(usize, i32)>(IDX_COMP_UTIL_SORT_CLUBS_BY_STRENGTH);
    if v == 0 || n <= 1 {
        return o(v, n);
    }
    let mut expected = core::slice::from_raw_parts(v as *const u32, n as usize).to_vec();
    let mut k = vec![0i32; n as usize];
    sort_clubs_by_strength(expected.as_mut_ptr(), k.as_mut_ptr(), n);
    o(v, n);
    let got = core::slice::from_raw_parts(v as *const u32, n as usize);
    SCOREBOARD.record(got == &expected[..], || format!("comp_util_sort_clubs_by_strength(n {}) original={:x?} rust={:x?}", n, got, expected));
}

/// Comparer for `set_club_final_position`.
pub(crate) unsafe extern "thiscall" fn compare_set_club_final_position(this: usize, club: usize, pos: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(IDX_SET_CLUB_FINAL_POSITION);
    if club == 0 {
        return o(this, club, pos); // simplification: the box, original only
    }
    let before = exe::comp_stats_find_club_entry(this, club);
    let r = CompStatsClub(o(this, club, pos));
    let same = !r.is_null() && (before == 0 || r.addr() == before) && r.final_position() == pos as u8;
    SCOREBOARD.record(same, || format!("set_club_final_position(club {:x}, {}) before={:x} original={:x}", club, pos as u8, before, r.addr()));
    r.addr()
}

/// Comparer for `comp_util_get_fixture_venue_side`.
pub(crate) unsafe extern "C" fn compare_comp_util_get_fixture_venue_side(club: usize, game: usize) -> u8 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize) -> u8>(IDX_COMP_UTIL_GET_FIXTURE_VENUE_SIDE);
    let r = o(club, game);
    let n = comp_util_get_fixture_venue_side(club, game);
    SCOREBOARD.record(r == n, || format!("comp_util_get_fixture_venue_side(club {:x}, game {:x}) original={} rust={}", club, game, r, n));
    r
}

/// Comparer for `compare_nation_coefficients_desc`.
pub(crate) unsafe extern "C" fn compare_compare_nation_coefficients_desc(a: usize, b: usize) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_COMPARE_NATION_COEFFICIENTS_DESC);
    let r = o(a, b);
    let n = compare_nation_coefficients_desc(a, b);
    SCOREBOARD.record(r == n, || format!("compare_nation_coefficients_desc original={} rust={}", r, n));
    r
}

/// Comparer for `comp_util_shuffle_pointer_array`.
pub(crate) unsafe extern "C" fn compare_comp_util_shuffle_pointer_array(items: usize, n: u32) {
    let o = original_fn::<unsafe extern "C" fn(usize, u32)>(IDX_COMP_UTIL_SHUFFLE_POINTER_ARRAY);
    let k = (n as u16 as i16).max(0) as usize;
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    if k > 0 {
        reg.push((items, 4 * k));
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(items, n),
        || crate::rng::without_counting(|| comp_util_shuffle_pointer_array(items, n)));
    SCOREBOARD.record(igual, || format!("comp_util_shuffle_pointer_array({}) {:x?} x {:x?}", k, mo, mr));
}

/// Comparer for `comp_util_sort_and_shuffle_top_clubs`.
pub(crate) unsafe extern "C" fn compare_comp_util_sort_and_shuffle_top_clubs(clubs: usize, n: u32, escolhas: u32) {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, u32)>(IDX_COMP_UTIL_SORT_AND_SHUFFLE_TOP_CLUBS);
    let (nn, e) = (n as u16 as i16, escolhas as u16 as i16);
    if nn < 0 || nn < e {
        return o(clubs, n, escolhas);
    }
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    if nn > 0 {
        reg.push((clubs, 4 * nn as usize));
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(clubs, n, escolhas),
        || crate::rng::without_counting(|| comp_util_sort_and_shuffle_top_clubs(clubs, n, escolhas)));
    SCOREBOARD.record(igual, || format!("comp_util_sort_and_shuffle_top_clubs({}, {}) {:x?} x {:x?}", nn, e, mo, mr));
}

/// Comparer for `comp_util_get_league_clash_type`.
pub(crate) unsafe extern "C" fn compare_comp_util_get_league_clash_type(a: usize, b: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> u8>(IDX_COMP_UTIL_GET_LEAGUE_CLASH_TYPE)(a, b);
    let n = comp_util_get_league_clash_type(a, b);
    SCOREBOARD.record(r == n, || format!("comp_util_get_league_clash_type({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `comp_util_get_club_strength_score`.
pub(crate) unsafe extern "C" fn compare_comp_util_get_club_strength_score(club: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> i32>(IDX_COMP_UTIL_GET_CLUB_STRENGTH_SCORE)(club);
    let n = comp_util_get_club_strength_score(club);
    SCOREBOARD.record(r == n, || format!("comp_util_get_club_strength_score({:x}) original={} rust={}", club, r, n));
    r
}

/// Comparer for `comp_util_get_latest_placed_club`.
pub(crate) unsafe extern "C" fn compare_comp_util_get_latest_placed_club(comp: usize, place: u32) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize, u32) -> usize>(IDX_COMP_UTIL_GET_LATEST_PLACED_CLUB);
    if place as u8 > 2 {
        return o(comp, place);
    }
    let n = read_i32(G_CLUB_COMP_HISTORY_COUNT).max(0) as usize;
    let reg = [(read_usize(G_CLUB_COMP_HISTORY_TABLE), 0x1a * n)];
    let (r, x, igual, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(comp, place), || comp_util_get_latest_placed_club(comp, place));
    SCOREBOARD.record(igual && r == x, || format!("comp_util_get_latest_placed_club({:x}, {}) {:x}/{:x} {:x?} x {:x?}", comp, place as u8, r, x, mo, mr));
    r
}

/// Comparer for `comp_util_read_fixture_from_news_slots`.
pub(crate) unsafe extern "C" fn compare_comp_util_read_fixture_from_news_slots(news: usize, fixture: usize, cursor: *mut u8) {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, *mut u8)>(IDX_COMP_UTIL_READ_FIXTURE_FROM_NEWS_SLOTS);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side_light(&[(fixture, 0x4d), (cursor as usize, 1)],
        || o(news, fixture, cursor), || comp_util_read_fixture_from_news_slots(news, fixture, cursor));
    SCOREBOARD.record(igual, || format!("comp_util_read_fixture_from_news_slots {:x?} x {:x?}", mo, mr));
}

/// Comparer for `comp_util_classify_tie_result_for_news`.
pub(crate) unsafe extern "C" fn compare_comp_util_classify_tie_result_for_news(club: usize, fixture: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> u8>(IDX_COMP_UTIL_CLASSIFY_TIE_RESULT_FOR_NEWS)(club, fixture);
    let n = comp_util_classify_tie_result_for_news(club, fixture);
    SCOREBOARD.record(r == n, || format!("comp_util_classify_tie_result_for_news original={} rust={}", r, n));
    r
}

/// Comparer for `comp_util_write_fixture_to_news_slots`.
pub(crate) unsafe extern "C" fn compare_comp_util_write_fixture_to_news_slots(game: usize, news: usize, cursor: *mut u8) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, *mut u8) -> u32>(IDX_COMP_UTIL_WRITE_FIXTURE_TO_NEWS_SLOTS);
    let k = *cursor as i8;
    // index outside 0..49 in any of the 18: 0x785790 opens the error box; original only
    if !(0..0x32 - 17).contains(&k) {
        return o(game, news, cursor);
    }
    let reg = [(news + 5, 200), (cursor as usize, 1)];
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(game, news, cursor), || comp_util_write_fixture_to_news_slots(game, news, cursor));
    SCOREBOARD.record(igual && r == n, || format!("comp_util_write_fixture_to_news_slots(cursor {}) original={} rust={} {:x?} x {:x?}", k, r, n, mo, mr));
    r
}

/// Comparer for `comp_util_format_stage_round_name`.
pub(crate) unsafe extern "C" fn compare_comp_util_format_stage_round_name(stage: u32, round: u32, out: *mut u8, size: i32, style: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32, u32, *mut u8, i32, u32) -> i32>(IDX_COMP_UTIL_FORMAT_STAGE_ROUND_NAME);
    if out.is_null() || size < 0xc8 {
        return o(stage, round, out, size, style); // the message box: original only
    }
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&[(out as usize, (size as usize).min(0x400))],
        || o(stage, round, out, size, style), || comp_util_format_stage_round_name(stage, round, out, size, style));
    SCOREBOARD.record(igual && r == n, || format!("comp_util_format_stage_round_name({}, {}) {} x {} {:x?} x {:x?}", stage as i16, round as i16, r, n, mo, mr));
    r
}

/// Comparer for `comp_util_scale_by_round`.
pub(crate) unsafe extern "C" fn compare_comp_util_scale_by_round(low: u32, high: u32, round: u32, rounds: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(u32, u32, u32, u32) -> u32>(IDX_COMP_UTIL_SCALE_BY_ROUND);
    if !(0..=100).contains(&interpolate_by_round(low, high, round, rounds).1) {
        return o(low, high, round, rounds);
    }
    let r = o(low, high, round, rounds);
    let n = comp_util_scale_by_round(low, high, round, rounds);
    SCOREBOARD.record(r == n, || format!("comp_util_scale_by_round({}, {}, {}, {}) original={:x} rust={:x}",
        low as u8 as i8, high as u8 as i8, round as u16 as i16, rounds as u16 as i16, r, n));
    r
}

/// Comparer for `comp_util_add_history_record`.
pub(crate) unsafe extern "C" fn compare_comp_util_add_history_record(league: usize, a: u32, b: u32, c: u32, d: u32) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, u32, u32, u32) -> usize>(IDX_COMP_UTIL_ADD_HISTORY_RECORD);
    if free_comp_history_row() == 0 {
        return o(league, a, b, c, d); // grows (realloc): original only
    }
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&comp_history_regions(), || o(league, a, b, c, d), || comp_util_add_history_record(league, a, b, c, d));
    SCOREBOARD.record(igual && r == n, || format!("comp_util_add_history_record({:x}, {:x}, {:x}, {:x}, {:x}) {:x}/{:x} {:x?} x {:x?}", league, a, b, c, d, r, n, mo, mr));
    r
}

/// Comparer for `comp_util_get_parent_stage_id`.
pub(crate) unsafe extern "C" fn compare_comp_util_get_parent_stage_id(id: i32) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(i32) -> i32>(IDX_COMP_UTIL_GET_PARENT_STAGE_ID)(id);
    let n = comp_util_get_parent_stage_id(id);
    SCOREBOARD.record(r == n, || format!("comp_util_get_parent_stage_id({:#x}): original={:#x} rust={:#x}", id, r, n));
    r
}

/// Comparer for `units_digit`.
pub(crate) unsafe extern "C" fn compare_units_digit(n: i32) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(i32) -> u8>(IDX_UNITS_DIGIT)(n);
    let v = units_digit(n);
    SCOREBOARD.record(r == v, || format!("units_digit({}) original={} rust={}", n, r, v));
    r
}

/// Comparer for `is_national_team`.
pub(crate) unsafe extern "C" fn compare_is_national_team(club: usize) -> u32 {
    if is_recording_post_match() {
        return call_recording(FN_DATABASE_IS_NATIONAL_TEAM, 0, [club as u32, 0, 0, 0, 0], || original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_IS_NATIONAL_TEAM)(club));
    }
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_IS_NATIONAL_TEAM)(club);
    let v = is_national_team(club);
    SCOREBOARD.record(r == v, || format!("is_national_team original={} rust={}", r, v));
    r
}

/// Comparer for `competition_counts`.
pub(crate) unsafe extern "C" fn compare_competition_counts(id: i32) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(i32) -> u32>(IDX_COMPETITION_COUNTS)(id);
    let v = competition_counts(id);
    SCOREBOARD.record(r == v, || format!("competition_counts({}) original={} rust={}", id, r, v));
    r
}
