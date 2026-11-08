use super::*;

// The comparers of the routines in ../game.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../game.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `game_update_nation_coefficients` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a original aloca e solta o vetor (e o ordena), então roda só ela, conferida contra o plano feito antes
/// pelas mesmas contas da cópia: a história (+0xb0..+0xd8) e a marca (+0x121) de cada nação e [0x9cd218] zerado.
pub(crate) unsafe extern "C" fn comparar_game_update_nation_coefficients() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_GAME_UPDATE_NATION_COEFFICIENTS);
    let iniciar = read_u8(G_NATION_COEFFICIENT_HISTORY_NEEDS_INIT) != 0;
    let today = read_usize(G_CURRENT_DATE) as i16;
    let dez = read_i16(G_NATION_COEFFICIENT_UPDATE_DAY_TABLE + (current_month() * 2) as usize) == today && read_i16(G_NATION_COEFFICIENT_UPDATE_DAY_DECEMBER) == today;
    let n = read_usize(G_NATION_COUNT) as i32;
    let base = read_usize(G_NATION_TABLE);
    let mut plan = Vec::new();
    for k in 0..n.max(0) as usize {
        let na = base + k * 0x122;
        let mut h = nation_coefficient_history(na);
        if iniciar {
            let v = read_f64(na + 0xa8);
            for x in h.iter_mut().take(5) {
                *x = v;
            }
        }
        let mut mark = read_u8(na + 0x121);
        if dez {
            roll_coefficient_year(na, &mut h, &mut mark);
        }
        plan.push((h, mark));
    }
    o();
    let mut error = None;
    for (k, (h, mark)) in plan.iter().enumerate() {
        let na = base + k * 0x122;
        let g = nation_coefficient_history(na);
        if (0..6).any(|j| g[j].to_bits() != h[j].to_bits()) || read_u8(na + 0x121) != *mark {
            error = Some(format!("nação {} ({:x}): {:?}/{} x plano {:?}/{}", k, na, g, read_u8(na + 0x121), h, mark));
            break;
        }
    }
    if read_u8(G_NATION_COEFFICIENT_HISTORY_NEEDS_INIT) != 0 {
        error = Some(format!("[0x9cd218] = {}", read_u8(G_NATION_COEFFICIENT_HISTORY_NEEDS_INIT)));
    }
    PLACAR.record(error.is_none(), || format!("game_update_nation_coefficients(dezembro {}): {}", dez, error.clone().unwrap_or_default()));
}

/// Comparer for `is_player_regen_day` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo al (o único byte que os chamadores leem).
pub(crate) unsafe extern "C" fn comparar_is_player_regen_day(day: u32) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(u32) -> u32>(IDX_IS_PLAYER_REGEN_DAY)(day);
    let n = is_player_regen_day(day);
    PLACAR.record(r as u8 == n as u8, || format!("is_player_regen_day({}) {} x {}", day as u16, r as u8, n as u8));
    r
}

/// Comparer for `clamp_char_sum` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo ax (os 16 bits altos do eax da original são os do chamador).
pub(crate) unsafe extern "C" fn comparar_clamp_char_sum(a: u32, b: u32, min: u32, max: u32) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(u32, u32, u32, u32) -> u32>(IDX_CLAMP_CHAR_SUM)(a, b, min, max);
    let n = clamp_char_sum(a, b, min, max);
    PLACAR.record(r as u16 == n as u16, || format!("clamp_char_sum({}, {}, {}, {}) {:x} x {:x}", a as u8 as i8, b as u8 as i8, min as u8 as i8, max as u8 as i8, r as u16, n as u16));
    r
}

/// Comparer for `game_update_squad_relationships` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_game_update_squad_relationships() {
    original_fn::<unsafe extern "C" fn()>(IDX_GAME_UPDATE_SQUAD_RELATIONSHIPS)()
}

/// Comparer for `game_check_club_dressing_room_unrest` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_game_check_club_dressing_room_unrest(clube: usize) {
    original_fn::<unsafe extern "C" fn(usize)>(IDX_GAME_CHECK_CLUB_DRESSING_ROOM_UNREST)(clube)
}

/// Comparer for `game_spread_squad_morale_by_leadership` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_game_spread_squad_morale_by_leadership() {
    original_fn::<unsafe extern "C" fn()>(IDX_GAME_SPREAD_SQUAD_MORALE_BY_LEADERSHIP)()
}

/// Comparer for `game_check_players_missing_training` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_game_check_players_missing_training() {
    original_fn::<unsafe extern "C" fn()>(IDX_GAME_CHECK_PLAYERS_MISSING_TRAINING)()
}

/// Comparer for `game_update_comps_for_date` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_game_update_comps_for_date(mark: u32) {
    original_fn::<unsafe extern "C" fn(u32)>(IDX_GAME_UPDATE_COMPS_FOR_DATE)(mark)
}

/// Comparer for `manager_manager_adjust_board_confidence` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pela confiança (2 bytes).
pub(crate) unsafe extern "thiscall" fn comparar_manager_manager_adjust_board_confidence(this: usize, clube: usize, delta: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_MANAGER_MANAGER_ADJUST_BOARD_CONFIDENCE);
    if clube == 0 {
        return o(this, clube, delta);
    }
    let c = board_confidence_ptr(this, clube);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side_light(&[(c, 2)], || o(this, clube, delta), || manager_manager_adjust_board_confidence(this, clube, delta));
    PLACAR.record(igual, || format!("manager_manager_adjust_board_confidence(clube {:x}, {}) {:x?} x {:x?}", clube, delta as i32, mo, mr));
}

/// Comparer for `transfer_manager_get_shortlist_manager` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_transfer_manager_get_shortlist_manager(manager: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_TRANSFER_MANAGER_GET_SHORTLIST_MANAGER)(manager);
    let n = transfer_manager_get_shortlist_manager(manager);
    PLACAR.record(r == n, || format!("transfer_manager_get_shortlist_manager original={:#x} rust={:#x}", r, n));
    r
}

/// Comparer for `training_manager_get_player_entry` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_training_manager_get_player_entry(treino: usize, player: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_TRAINING_MANAGER_GET_PLAYER_ENTRY)(treino, player);
    let n = training_manager_get_player_entry(treino, player);
    PLACAR.record(r == n, || format!("training_manager_get_player_entry original={:#x} rust={:#x}", r, n));
    r
}

/// Comparer for `game_average` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_game_average(t: usize, second: u8) -> f64 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u8) -> f64>(IDX_GAME_AVERAGE)(t, second);
    let n = game_average(t, second);
    PLACAR.record(o.to_bits() == n.to_bits(), || format!("game_average({:#x}, {}) original={} rust={}", t, second, o, n));
    o
}

/// Comparer for `pow_double` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_pow_double(base: f64, expoente: f64) -> f64 {
    let r = original_fn::<unsafe extern "C" fn(f64, f64) -> f64>(IDX_POW_DOUBLE)(base, expoente);
    let n = pow_double(base, expoente);
    PLACAR.record(r.to_bits() == n.to_bits(), || format!("pow_double({}, {}) original={} rust={}", base, expoente, r, n));
    r
}

/// Comparer for `compare_players_by_ability` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor.
pub(crate) unsafe extern "C" fn comparar_compare_players_by_ability(a: *const usize, b: *const usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(*const usize, *const usize) -> i32>(IDX_COMPARE_PLAYERS_BY_ABILITY)(a, b);
    let n = compare_players_by_ability(a, b);
    PLACAR.record(r == n, || format!("compare_players_by_ability({:#x}, {:#x}) original={} rust={}", *a, *b, r, n));
    r
}
