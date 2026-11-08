use super::*;

// The comparers of the routines in ../weather.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../weather.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `check_postponement` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_check_postponement(this: usize, game: usize) -> u8 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_CHECK_POSTPONEMENT)(this, game)
}

/// Comparer for `get_club_finance` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor (clube nulo: só a original, com a caixa de erro).
pub(crate) unsafe extern "thiscall" fn comparar_get_club_finance(this: usize, club: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_GET_CLUB_FINANCE);
    if club == 0 {
        return o(this, club);
    }
    let r = o(this, club);
    let n = get_club_finance(this, club);
    SCOREBOARD.record(r == n, || format!("get_club_finance(clube {:#x}) original={:#x} rust={:#x}", club, r, n));
    r
}

/// Comparer for `get_match_weather` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado nos 4 bytes da saída.
pub(crate) unsafe extern "thiscall" fn comparar_get_match_weather(this: usize, s: *mut u8, game: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *mut u8, usize)>(IDX_GET_MATCH_WEATHER);
    let regions = [(s as usize, 4)];
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side_light(&regions, || o(this, s, game), || get_match_weather(this, s, game));
    SCOREBOARD.record(igual, || format!("get_match_weather(jogo {:#x}) {:x?} x {:x?}", game, mo.0, mr.0));
}

/// Comparer for `get_temperature` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor.
pub(crate) unsafe extern "thiscall" fn comparar_get_temperature(this: usize, band: u32, game: usize) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> u8>(IDX_GET_TEMPERATURE)(this, band, game);
    let n = get_temperature(this, band, game);
    SCOREBOARD.record(r == n, || format!("get_temperature(band {}, jogo {:#x}) original={} rust={}", band as u8 as i8, game, r as i8, n as i8));
    r
}

/// Comparer for `get_pitch_condition` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor.
pub(crate) unsafe extern "thiscall" fn comparar_get_pitch_condition(this: usize, clima: *const u8, game: usize) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, *const u8, usize) -> u8>(IDX_GET_PITCH_CONDITION)(this, clima, game);
    let n = get_pitch_condition(this, clima, game);
    SCOREBOARD.record(r == n, || format!("get_pitch_condition(clima {:x?}, jogo {:#x}) original={:#x} rust={:#x}", core::slice::from_raw_parts(clima, 4), game, r, n));
    r
}

/// Comparer for `update_city_weather` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_update_city_weather(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_UPDATE_CITY_WEATHER)(this)
}

/// Comparer for `read_region` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_read_region(this: usize, f: usize, regiao: usize) -> u8 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u8>(IDX_READ_REGION)(this, f, regiao)
}

/// Comparer for `read_region_period` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_read_region_period(this: usize, f: usize, period: usize) -> u8 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u8>(IDX_READ_REGION_PERIOD)(this, f, period)
}

/// Comparer for `load_config` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_load_config(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_LOAD_CONFIG)(this)
}
