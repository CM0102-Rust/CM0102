use super::*;

// The comparers of the routines in ../weather.rs.

/// Comparer for `check_postponement`.
pub(crate) unsafe extern "thiscall" fn compare_check_postponement(this: usize, game: usize) -> u8 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_CHECK_POSTPONEMENT)(this, game)
}

/// Comparer for `get_club_finance`.
pub(crate) unsafe extern "thiscall" fn compare_get_club_finance(this: usize, club: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_GET_CLUB_FINANCE);
    if club == 0 {
        return o(this, club);
    }
    let r = o(this, club);
    let n = get_club_finance(this, club);
    SCOREBOARD.record(r == n, || format!("get_club_finance(club {:#x}) original={:#x} rust={:#x}", club, r, n));
    r
}

/// Comparer for `get_match_weather`.
pub(crate) unsafe extern "thiscall" fn compare_get_match_weather(this: usize, s: *mut u8, game: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *mut u8, usize)>(IDX_GET_MATCH_WEATHER);
    let regions = [(s as usize, 4)];
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side_light(&regions, || o(this, s, game), || get_match_weather(this, s, game));
    SCOREBOARD.record(igual, || format!("get_match_weather(game {:#x}) {:x?} x {:x?}", game, mo.0, mr.0));
}

/// Comparer for `get_temperature`.
pub(crate) unsafe extern "thiscall" fn compare_get_temperature(this: usize, band: u32, game: usize) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> u8>(IDX_GET_TEMPERATURE)(this, band, game);
    let n = get_temperature(this, band, game);
    SCOREBOARD.record(r == n, || format!("get_temperature(band {}, game {:#x}) original={} rust={}", band as u8 as i8, game, r as i8, n as i8));
    r
}

/// Comparer for `get_pitch_condition`.
pub(crate) unsafe extern "thiscall" fn compare_get_pitch_condition(this: usize, clima: *const u8, game: usize) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, *const u8, usize) -> u8>(IDX_GET_PITCH_CONDITION)(this, clima, game);
    let n = get_pitch_condition(this, clima, game);
    SCOREBOARD.record(r == n, || format!("get_pitch_condition(clima {:x?}, game {:#x}) original={:#x} rust={:#x}", core::slice::from_raw_parts(clima, 4), game, r, n));
    r
}

/// Comparer for `update_city_weather`.
pub(crate) unsafe extern "thiscall" fn compare_update_city_weather(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_UPDATE_CITY_WEATHER)(this)
}

/// Comparer for `read_region`.
pub(crate) unsafe extern "thiscall" fn compare_read_region(this: usize, f: usize, region: usize) -> u8 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u8>(IDX_READ_REGION)(this, f, region)
}

/// Comparer for `read_region_period`.
pub(crate) unsafe extern "thiscall" fn compare_read_region_period(this: usize, f: usize, period: usize) -> u8 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u8>(IDX_READ_REGION_PERIOD)(this, f, period)
}

/// Comparer for `load_config`.
pub(crate) unsafe extern "thiscall" fn compare_load_config(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_LOAD_CONFIG)(this)
}
