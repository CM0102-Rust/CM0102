use super::*;

// The comparers of the routines in ../weather.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../weather.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `check_postponement` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_check_postponement(this: usize, jogo: usize) -> u8 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_CHECK_POSTPONEMENT)(this, jogo)
}

/// Comparer for `get_club_finance` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor (clube nulo: só a original, com a caixa de erro).
pub(crate) unsafe extern "thiscall" fn comparar_get_club_finance(this: usize, clube: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_GET_CLUB_FINANCE);
    if clube == 0 {
        return o(this, clube);
    }
    let r = o(this, clube);
    let n = get_club_finance(this, clube);
    PLACAR.registrar(r == n, || format!("get_club_finance(clube {:#x}) original={:#x} rust={:#x}", clube, r, n));
    r
}

/// Comparer for `get_match_weather` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado nos 4 bytes da saída.
pub(crate) unsafe extern "thiscall" fn comparar_get_match_weather(this: usize, s: *mut u8, jogo: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *mut u8, usize)>(IDX_GET_MATCH_WEATHER);
    let regions = [(s as usize, 4)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&regions, || o(this, s, jogo), || get_match_weather(this, s, jogo));
    PLACAR.registrar(igual, || format!("get_match_weather(jogo {:#x}) {:x?} x {:x?}", jogo, mo.0, mr.0));
}

/// Comparer for `get_temperature` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor.
pub(crate) unsafe extern "thiscall" fn comparar_get_temperature(this: usize, band: u32, jogo: usize) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> u8>(IDX_GET_TEMPERATURE)(this, band, jogo);
    let n = get_temperature(this, band, jogo);
    PLACAR.registrar(r == n, || format!("get_temperature(band {}, jogo {:#x}) original={} rust={}", band as u8 as i8, jogo, r as i8, n as i8));
    r
}

/// Comparer for `get_pitch_condition` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor.
pub(crate) unsafe extern "thiscall" fn comparar_get_pitch_condition(this: usize, clima: *const u8, jogo: usize) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, *const u8, usize) -> u8>(IDX_GET_PITCH_CONDITION)(this, clima, jogo);
    let n = get_pitch_condition(this, clima, jogo);
    PLACAR.registrar(r == n, || format!("get_pitch_condition(clima {:x?}, jogo {:#x}) original={:#x} rust={:#x}", core::slice::from_raw_parts(clima, 4), jogo, r, n));
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
pub(crate) unsafe extern "thiscall" fn comparar_read_region_period(this: usize, f: usize, periodo: usize) -> u8 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u8>(IDX_READ_REGION_PERIOD)(this, f, periodo)
}

/// Comparer for `load_config` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_load_config(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_LOAD_CONFIG)(this)
}
