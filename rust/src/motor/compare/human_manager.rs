use super::*;

// The comparers of the routines in ../human_manager.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../human_manager.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `get_active_tactic` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a devolução (rotina pura); sem técnico ou fora da conta, só a original (caixa).
pub(crate) unsafe extern "thiscall" fn comparar_get_active_tactic(this: usize, tecnico: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_GET_ACTIVE_TACTIC);
    let t = manager_or_current_human(tecnico);
    if t == 0 || human_manager_human_manager_index(this, t).is_none() {
        return o(this, tecnico);
    }
    let r = o(this, tecnico);
    let n = get_active_tactic(this, tecnico);
    PLACAR.registrar(r == n, || format!("get_active_tactic(tecnico {:#x}) original={:#x} rust={:#x}", tecnico, r, n));
    r
}

/// Comparer for `select_tactic_slot` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelo byte da vaga ativa do registro; sem técnico (nada) ou fora da conta (caixa), só a
/// original.
pub(crate) unsafe extern "thiscall" fn comparar_select_tactic_slot(this: usize, reserva: u32, clube: usize, tecnico: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize, usize)>(IDX_SELECT_TACTIC_SLOT);
    let t = manager_or_current_human(tecnico);
    let Some(k) = (if t != 0 { human_manager_human_manager_index(this, t) } else { None }) else {
        return o(this, reserva, clube, tecnico);
    };
    let reg = [(human_manager_record_at(this, k) + 0x9a69, 1)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, reserva, clube, tecnico), || select_tactic_slot(this, reserva, clube, tecnico));
    PLACAR.registrar(igual, || format!("select_tactic_slot(reserva {}, clube {:#x}, tecnico {:#x}) {:x?} x {:x?}", reserva, clube, tecnico, mo, mr));
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
pub(crate) unsafe extern "thiscall" fn comparar_has_nickname(this: usize, pessoa: usize, humano: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_HAS_NICKNAME);
    if pessoa != 0 && matches!(human_index_for_nickname(this, humano), Some(None)) {
        return o(this, pessoa, humano);
    }
    let r = o(this, pessoa, humano);
    let n = has_nickname(this, pessoa, humano);
    PLACAR.registrar(r == n, || format!("has_nickname original={} rust={}", r, n));
    r
}

/// Comparer for `get_holiday_stamp` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_get_holiday_stamp(gerente: usize, pessoa: usize) -> u32 {
    if human_manager_record(gerente, pessoa).is_none() {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_GET_HOLIDAY_STAMP)(gerente, pessoa);
    }
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_GET_HOLIDAY_STAMP)(gerente, pessoa);
    let n = get_holiday_stamp(gerente, pessoa);
    PLACAR.registrar(r == n, || format!("get_holiday_stamp original={} rust={}", r, n));
    r
}

/// Comparer for `get_saved_lineup` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_get_saved_lineup(gerente: usize, pessoa: usize) -> usize {
    if human_manager_record(gerente, pessoa).is_none() {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_GET_SAVED_LINEUP)(gerente, pessoa);
    }
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_GET_SAVED_LINEUP)(gerente, pessoa);
    let n = get_saved_lineup(gerente, pessoa);
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
pub(crate) unsafe extern "thiscall" fn comparar_is_watching_fixture(this: usize, partida: usize, humano: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_IS_WATCHING_FIXTURE);
    let r = o(this, partida, humano);
    // simplification: os casos de erro (caixa) só na original
    if partida == 0 || formation_watched_match_record(this, humano).is_err() {
        return r;
    }
    let n = is_watching_fixture(this, partida, humano);
    PLACAR.registrar(r == n, || format!("is_watching_fixture original={} rust={}", r, n));
    r
}

/// Comparer for `can_watch_fixture` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a original e a cópia (as duas só leem; a volta da lista recomeça o cursor), pelo valor.
pub(crate) unsafe extern "thiscall" fn comparar_can_watch_fixture(this: usize, partida: usize, humano: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_CAN_WATCH_FIXTURE)(this, partida, humano);
    if partida == 0 {
        return r;
    }
    let n = can_watch_fixture(this, partida, humano);
    PLACAR.registrar(r == n, || format!("can_watch_fixture(partida {:x}, humano {:x}) original={} rust={}", partida, humano, r, n));
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
pub(crate) unsafe extern "C" fn comparar_human_runs_club(clube: usize, qualquer: u32, forcar: i8) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize, u32, i8) -> u32>(IDX_HUMAN_RUNS_CLUB)(clube, qualquer, forcar);
    let n = human_runs_club(clube, qualquer, forcar);
    PLACAR.registrar(r == n, || format!("human_runs_club({:#x}, {}, {}) original={} rust={}", clube, qualquer, forcar, r, n));
    r
}

/// Comparer for `set_human_field` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_set_human_field(t: usize, valor: u32, tecnico: *const i32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *const i32) -> u32>(IDX_SET_HUMAN_FIELD);
    let mut p = tecnico;
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
    let (r, v, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(t, valor, tecnico), || set_human_field(t, valor, tecnico));
    PLACAR.registrar(igual && r == v, || format!("set_human_field({:#x}, {:#x}, {:p}) original={} {:x?} rust={} {:x?}", t, valor, tecnico, r, mo, v, mr));
    r
}

/// Comparer for `set_on_holiday` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_set_on_holiday(this: usize, ferias: u32, tecnico: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize)>(IDX_SET_ON_HOLIDAY);
    let t = if tecnico != 0 { tecnico } else { read_usize(G_SCREEN_RECORDS_MANAGER + 0x300 * (*(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as isize as usize)) };
    let i = if t == 0 { -1 } else { (read_usize(t) as i32).wrapping_sub(read_usize(G_STAFF_COUNT) as i32).wrapping_add(0x10) };
    if i < 0 || i > (read_usize(this + 4) as i32).wrapping_sub(1) {
        return o(this, ferias, tecnico); // a caixa: só a original
    }
    let marca = read_usize(this).wrapping_add((i as usize).wrapping_mul(0xa6ef)).wrapping_add(0x9a01);
    let (antes, conta) = (read_usize(marca), read_usize(this + 0xc) as i32);
    let esperada = conta + if ferias != 0 && antes == 0 { 1 } else if ferias == 0 && antes != 0 { -1 } else { 0 };
    o(this, ferias, tecnico);
    let (depois, conta2) = (read_usize(marca) as u32, read_usize(this + 0xc) as i32);
    PLACAR.registrar(depois == ferias && conta2 == esperada, || format!("set_on_holiday({}, técnico {:x}): marca {:x} -> {:x}, conta {} -> {} (esperava {})", ferias, t, antes, depois, conta, conta2, esperada));
}

/// Comparer for `square_bit` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "stdcall" fn comparar_square_bit(codigo: i16) -> u32 {
    let r = original_fn::<unsafe extern "stdcall" fn(i16) -> u32>(IDX_SQUARE_BIT)(codigo);
    let v = square_bit(codigo);
    PLACAR.registrar(r == v, || format!("square_bit({:#x}) original={:#x} rust={:#x}", codigo, r, v));
    r
}

/// Comparer for `split_mask` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_split_mask(m: u32, setor: *mut u16, lado: *mut u16) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(u32, *mut u16, *mut u16) -> u32>(IDX_SPLIT_MASK);
    let reg = [(setor as usize, 2), (lado as usize, 2)];
    let (r, v, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(m, setor, lado), || split_mask(m, setor, lado));
    PLACAR.registrar(igual && r == v, || format!("split_mask({:#x}) original={} {:x?} rust={} {:x?}", m, r, mo, v, mr));
    r
}

/// Comparer for `human_club` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_human_club(clube: usize, a: u32, b: u8, pessoa: usize, c: u32) -> u32 {
    if is_recording_post_match() {
        let o = original_fn::<unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32>(IDX_HUMAN_CLUB);
        return call_recording(FN_HUMAN_MANAGER_IS_HUMAN_CLUB, 0, [clube as u32, a, b as u32, pessoa as u32, c], || o(clube, a, b, pessoa, c));
    }
    let r = original_fn::<unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32>(IDX_HUMAN_CLUB)(clube, a, b, pessoa, c);
    let v = human_club(clube, a, b, pessoa, c);
    PLACAR.registrar(r == v, || format!("human_club({:#x}, {}, {}, {:#x}, {}) original={} rust={}", clube, a, b, pessoa, c, r, v));
    r
}

/// Comparer for `manager_9a06` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_manager_9a06(this: usize, pessoa: usize, saida: *mut u32) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut u32) -> u8>(IDX_MANAGER_9A06);
    let reg: Vec<(usize, usize)> = if saida.is_null() { vec![] } else { vec![(saida as usize, 4)] };
    let (r, v, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, pessoa, saida), || manager_9a06(this, pessoa, saida));
    PLACAR.registrar(igual && r == v, || format!("manager_9a06 original={} {:x?} rust={} {:x?}", r, mo, v, mr));
    r
}
