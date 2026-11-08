use super::*;

// The comparers of the routines in ../human_manager.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../human_manager.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `get_active_tactic` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a devolução (rotina pura); sem técnico ou fora da conta, só a original (caixa).
pub(crate) unsafe extern "thiscall" fn comparar_get_active_tactic(this: usize, manager: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_GET_ACTIVE_TACTIC);
    let t = manager_or_current_human(manager);
    if t == 0 || human_manager_human_manager_index(this, t).is_none() {
        return o(this, manager);
    }
    let r = o(this, manager);
    let n = get_active_tactic(this, manager);
    PLACAR.registrar(r == n, || format!("get_active_tactic(tecnico {:#x}) original={:#x} rust={:#x}", manager, r, n));
    r
}

/// Comparer for `select_tactic_slot` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelo byte da vaga ativa do registro; sem técnico (nada) ou fora da conta (caixa), só a
/// original.
pub(crate) unsafe extern "thiscall" fn comparar_select_tactic_slot(this: usize, reserve: u32, clube: usize, manager: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize, usize)>(IDX_SELECT_TACTIC_SLOT);
    let t = manager_or_current_human(manager);
    let Some(k) = (if t != 0 { human_manager_human_manager_index(this, t) } else { None }) else {
        return o(this, reserve, clube, manager);
    };
    let reg = [(human_manager_record_at(this, k) + 0x9a69, 1)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, reserve, clube, manager), || select_tactic_slot(this, reserve, clube, manager));
    PLACAR.registrar(igual, || format!("select_tactic_slot(reserva {}, clube {:#x}, tecnico {:#x}) {:x?} x {:x?}", reserve, clube, manager, mo, mr));
}

/// Comparer for `clear_invalid_watched_fixtures` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelas 16 partidas assistidas (a 0x5fe700 só lê).
pub(crate) unsafe extern "thiscall" fn comparar_clear_invalid_watched_fixtures(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_CLEAR_INVALID_WATCHED_FIXTURES);
    let reg: Vec<(usize, usize)> = (0..0x10).map(|i| (human_manager_watched_match_record(this, i), 79)).collect();
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this), || clear_invalid_watched_fixtures(this));
    PLACAR.registrar(igual && r == n, || format!("clear_invalid_watched_fixtures {:x?} x {:x?}", mo, mr));
    r
}

/// Comparer for `is_fixture_watched` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_is_fixture_watched(this: usize, partida: usize) -> u32 {
    // a 0x5fe290 só lê: pode rodar nos dois lados
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_IS_FIXTURE_WATCHED);
    if partida == 0 {
        return o(this, partida);
    }
    let r = o(this, partida);
    let n = is_fixture_watched(this, partida);
    PLACAR.registrar(r == n, || format!("is_fixture_watched original={} rust={}", r, n));
    r
}

/// Comparer for `has_nickname` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_has_nickname(this: usize, person: usize, human: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_HAS_NICKNAME);
    if person != 0 && matches!(human_index_for_nickname(this, human), Some(None)) {
        return o(this, person, human);
    }
    let r = o(this, person, human);
    let n = has_nickname(this, person, human);
    PLACAR.registrar(r == n, || format!("has_nickname original={} rust={}", r, n));
    r
}

/// Comparer for `get_holiday_stamp` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_get_holiday_stamp(manager: usize, person: usize) -> u32 {
    if human_manager_record(manager, person).is_none() {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_GET_HOLIDAY_STAMP)(manager, person);
    }
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_GET_HOLIDAY_STAMP)(manager, person);
    let n = get_holiday_stamp(manager, person);
    PLACAR.registrar(r == n, || format!("get_holiday_stamp original={} rust={}", r, n));
    r
}

/// Comparer for `get_saved_lineup` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_get_saved_lineup(manager: usize, person: usize) -> usize {
    if human_manager_record(manager, person).is_none() {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_GET_SAVED_LINEUP)(manager, person);
    }
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_GET_SAVED_LINEUP)(manager, person);
    let n = get_saved_lineup(manager, person);
    PLACAR.registrar(r == n, || format!("get_saved_lineup original={:#x} rust={:#x}", r, n));
    r
}

/// Comparer for `human_manager_get_stand_in_manager` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_human_manager_get_stand_in_manager(clube: usize) -> usize {
    let r = original_fn::<unsafe extern "C" fn(usize) -> usize>(IDX_HUMAN_MANAGER_GET_STAND_IN_MANAGER)(clube);
    let n = human_manager_get_stand_in_manager(clube);
    PLACAR.registrar(r == n, || format!("human_manager_get_stand_in_manager({:#x}) original={:#x} rust={:#x}", clube, r, n));
    r
}

/// Comparer for `is_watching_fixture` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_is_watching_fixture(this: usize, partida: usize, human: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_IS_WATCHING_FIXTURE);
    let r = o(this, partida, human);
    // simplification: os casos de erro (caixa) só na original
    if partida == 0 || formation_watched_match_record(this, human).is_err() {
        return r;
    }
    let n = is_watching_fixture(this, partida, human);
    PLACAR.registrar(r == n, || format!("is_watching_fixture original={} rust={}", r, n));
    r
}

/// Comparer for `can_watch_fixture` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a original e a cópia (as duas só leem; a volta da lista recomeça o cursor), pelo valor.
pub(crate) unsafe extern "thiscall" fn comparar_can_watch_fixture(this: usize, partida: usize, human: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_CAN_WATCH_FIXTURE)(this, partida, human);
    if partida == 0 {
        return r;
    }
    let n = can_watch_fixture(this, partida, human);
    PLACAR.registrar(r == n, || format!("can_watch_fixture(partida {:x}, humano {:x}) original={} rust={}", partida, human, r, n));
    r
}

/// Comparer for `remove_nickname_from_all` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelos vetores de apelidos e contagens de todos os técnicos humanos; sem técnicos (o qsort
/// leria pelo `id`), só a original.
pub(crate) unsafe extern "thiscall" fn comparar_remove_nickname_from_all(this: usize, id: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_REMOVE_NICKNAME_FROM_ALL);
    let n = read_u32(this + 4) as i32;
    if id == 0 || n <= 0 {
        return o(this, id);
    }
    let mut reg = Vec::new();
    for c in 0..n.min(128) {
        let g = (read_u32(this) as usize).wrapping_add(c as usize * 0xa6ef);
        if g != 0 && read_u32(g + 0xa6e3) != 0 {
            reg.push((g + 0xa6e3, 0xc));
            reg.push((read_u32(g + 0xa6e3) as usize, (read_u32(g + 0xa6eb) as i32).max(0) as usize * 30));
        }
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, id), || remove_nickname_from_all(this, id));
    PLACAR.registrar(igual, || format!("remove_nickname_from_all({:x}) {:x?} x {:x?}", id, mo, mr));
}

/// Comparer for `human_runs_club` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_human_runs_club(clube: usize, qualquer: u32, force: i8) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize, u32, i8) -> u32>(IDX_HUMAN_RUNS_CLUB)(clube, qualquer, force);
    let n = human_runs_club(clube, qualquer, force);
    PLACAR.registrar(r == n, || format!("human_runs_club({:#x}, {}, {}) original={} rust={}", clube, qualquer, force, r, n));
    r
}

/// Comparer for `set_human_field` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_set_human_field(t: usize, value: u32, manager: *const i32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *const i32) -> u32>(IDX_SET_HUMAN_FIELD);
    let mut p = manager;
    if p.is_null() {
        let vez = *(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as isize;
        p = core::ptr::read_unaligned((0xb74c26 + vez * 0x300) as usize as *const *const i32);
    }
    let reg = if p.is_null() {
        vec![]
    } else {
        let i = (*p).wrapping_sub(*(G_STAFF_COUNT as *const i32)).wrapping_add(0x10);
        if i < 0 || i > (*((t + 4) as *const i32)).wrapping_sub(1) { vec![] } else { vec![(*(t as *const usize) + i as usize * 0xa6ef + 0x9a65, 4)] }
    };
    let (r, v, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(t, value, manager), || set_human_field(t, value, manager));
    PLACAR.registrar(igual && r == v, || format!("set_human_field({:#x}, {:#x}, {:p}) original={} {:x?} rust={} {:x?}", t, value, manager, r, mo, v, mr));
    r
}

/// Comparer for `set_on_holiday` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_set_on_holiday(this: usize, holiday: u32, manager: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize)>(IDX_SET_ON_HOLIDAY);
    let t = if manager != 0 { manager } else { read_usize(G_SCREEN_RECORDS_MANAGER + 0x300 * (*(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as isize as usize)) };
    let i = if t == 0 { -1 } else { (read_usize(t) as i32).wrapping_sub(read_usize(G_STAFF_COUNT) as i32).wrapping_add(0x10) };
    if i < 0 || i > (read_usize(this + 4) as i32).wrapping_sub(1) {
        return o(this, holiday, manager); // a caixa: só a original
    }
    let mark = read_usize(this).wrapping_add((i as usize).wrapping_mul(0xa6ef)).wrapping_add(0x9a01);
    let (antes, conta) = (read_usize(mark), read_usize(this + 0xc) as i32);
    let esperada = conta + if holiday != 0 && antes == 0 { 1 } else if holiday == 0 && antes != 0 { -1 } else { 0 };
    o(this, holiday, manager);
    let (depois, conta2) = (read_usize(mark) as u32, read_usize(this + 0xc) as i32);
    PLACAR.registrar(depois == holiday && conta2 == esperada, || format!("set_on_holiday({}, técnico {:x}): marca {:x} -> {:x}, conta {} -> {} (esperava {})", holiday, t, antes, depois, conta, conta2, esperada));
}

/// Comparer for `square_bit` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "stdcall" fn comparar_square_bit(code: i16) -> u32 {
    let r = original_fn::<unsafe extern "stdcall" fn(i16) -> u32>(IDX_SQUARE_BIT)(code);
    let v = square_bit(code);
    PLACAR.registrar(r == v, || format!("square_bit({:#x}) original={:#x} rust={:#x}", code, r, v));
    r
}

/// Comparer for `split_mask` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_split_mask(m: u32, setor: *mut u16, side: *mut u16) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(u32, *mut u16, *mut u16) -> u32>(IDX_SPLIT_MASK);
    let reg = [(setor as usize, 2), (side as usize, 2)];
    let (r, v, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(m, setor, side), || split_mask(m, setor, side));
    PLACAR.registrar(igual && r == v, || format!("split_mask({:#x}) original={} {:x?} rust={} {:x?}", m, r, mo, v, mr));
    r
}

/// Comparer for `human_club` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_human_club(clube: usize, a: u32, b: u8, person: usize, c: u32) -> u32 {
    if is_recording_post_match() {
        let o = original_fn::<unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32>(IDX_HUMAN_CLUB);
        return call_recording(FN_HUMAN_MANAGER_IS_HUMAN_CLUB, 0, [clube as u32, a, b as u32, person as u32, c], || o(clube, a, b, person, c));
    }
    let r = original_fn::<unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32>(IDX_HUMAN_CLUB)(clube, a, b, person, c);
    let v = human_club(clube, a, b, person, c);
    PLACAR.registrar(r == v, || format!("human_club({:#x}, {}, {}, {:#x}, {}) original={} rust={}", clube, a, b, person, c, r, v));
    r
}

/// Comparer for `manager_9a06` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_manager_9a06(this: usize, person: usize, out: *mut u32) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut u32) -> u8>(IDX_MANAGER_9A06);
    let reg: Vec<(usize, usize)> = if out.is_null() { vec![] } else { vec![(out as usize, 4)] };
    let (r, v, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, person, out), || manager_9a06(this, person, out));
    PLACAR.registrar(igual && r == v, || format!("manager_9a06 original={} {:x?} rust={} {:x?}", r, mo, v, mr));
    r
}
