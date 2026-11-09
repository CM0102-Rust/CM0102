use super::*;

// The comparers of the routines in ../training_manager.rs.
// [pt] Os comparadores das rotinas de ../training_manager.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `update_coach_assignments` (comparison mode: see the header of motor.rs): side by side on the human's
/// staff (54 bytes) and on the club's mark `+0x244` (`0x53cb80`/`0x53cc30` via `0x540a50` may zero it).
///
/// Without a human or with a national team's club: side by side only on the mark. A manager outside the 16 humans
/// (`0x8b4cd0` shows the box and the original routine would write to null): only the original routine.
///
/// [pt] Comparador: lado a lado na comissão do humano (54 bytes) e na marca +0x244 do clube (a 0x53cb80/0x53cc30 pela
/// 0x540a50 podem zerá-la). Sem humano ou clube de seleção: lado a lado só na marca. Técnico fora dos 16 humanos
/// (a 0x8b4cd0 mostra a caixa e a original escreveria no nulo): só a original.
pub(crate) unsafe extern "thiscall" fn compare_update_coach_assignments(this: usize, club: usize, tecnico: usize) {
    let club = Club(club);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_UPDATE_COACH_ASSIGNMENTS);
    if club.is_null() {
        return o(this, club.addr(), tecnico);
    }
    let mut reg = vec![(club.addr() + club::HAS_LINKED_CLUB, 1)];
    if (tecnico != 0 || exe::human_manager_is_human_club(club.addr(), 1, (1) as u32, (0) as u32, 0) != 0) && exe::database_is_national_team(club.addr()) == 0 {
        let manager = if tecnico == 0 { club.manager().addr() } else { tecnico };
        if manager == 0 || !(0..0x10).contains(&read_i32(G_STAFF_COUNT).wrapping_sub(read_i32(manager)).wrapping_sub(1)) {
            return o(this, club.addr(), tecnico);
        }
        reg.push((this + 0x652d + 54 * read_i32(G_STAFF_COUNT).wrapping_sub(read_i32(manager)).wrapping_sub(1) as usize, 54));
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, club.addr(), tecnico), || update_coach_assignments(this, club.addr(), tecnico));
    SCOREBOARD.record(igual, || format!("update_coach_assignments(clube {:x}, tecnico {:x}) {:x?} x {:x?}", club.addr(), tecnico, mo.0, mr.0));
}

/// Comparer for `reassign_squad_schedules`.
///
/// Side by side over the training entry and fitness load of each player of the two squads, the manager's staff
/// (`update_coach_assignments` at the end) and the clubs' mark +0x244 (`database_collect_club_squad` via `exe::club_get_linked_b_team`). With an error box (a player with no player
/// sheet in the old squad, or a manager outside the 16 humans with a new club): only the original runs.
///
/// [pt] Comparador: lado a lado na entrada do treino e na carga de forma de cada jogador dos dois elencos, na comissão do
/// técnico (a 0x8b2880 do fim) e na marca +0x244 dos clubes (a 0x541f90 pela 0x540a50). Com caixa (jogador sem
/// ficha no elenco antigo ou técnico fora dos 16 humanos com clube novo): só a original.
pub(crate) unsafe extern "thiscall" fn compare_reassign_squad_schedules(this: usize, manager: usize, old: usize, novo: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(IDX_REASSIGN_SQUAD_SCHEDULES);
    let k = if manager == 0 { -1 } else { read_i32(G_STAFF_COUNT).wrapping_sub(read_i32(manager)).wrapping_sub(1) };
    if novo != 0 && !(0..0x10).contains(&k) {
        return o(this, manager, old, novo);
    }
    let les = read_usize(read_usize(G_INJURY));
    let mut reg = Vec::new();
    for (clube, antigo_) in [(old, true), (novo, false)] {
        if clube == 0 {
            continue;
        }
        reg.push((clube + club::HAS_LINKED_CLUB, 1));
        let mut v = [0usize; 50];
        exe::database_collect_club_squad(clube, v.as_mut_ptr(), 0);
        for &p in &v {
            if p == 0 || read_i32(p) >= read_i32(G_STAFF_COUNT) {
                continue;
            }
            let f = read_usize(p + staff::PLAYER);
            if f == 0 {
                if antigo_ {
                    return o(this, manager, old, novo);
                }
                continue;
            }
            reg.push((training_entry_of_player_data(this, f), 6));
            reg.push((les.wrapping_add((read_i32(p) as isize * 31 + 0x1e) as usize), 1));
        }
    }
    if novo != 0 {
        reg.push((this + 0x652d + 54 * k as usize, 54));
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, manager, old, novo), || reassign_squad_schedules(this, manager, old, novo));
    SCOREBOARD.record(igual, || format!("reassign_squad_schedules(tecnico {:x}, {:x} -> {:x}) {:x?} x {:x?}", manager, old, novo, mo.0, mr.0));
}

/// Comparador: lado a lado nos 167 programas (0x9b bytes cada).
pub(crate) unsafe extern "thiscall" fn compare_training_manager_cpp_0x8b2230(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_TRAINING_MANAGER_CPP_0X8B2230);
    let (_, _, igual, _, _) = crate::comparison::side_by_side(&[(this, 0xa7 * 0x9b)], || o(this), || training_manager_cpp_0x8b2230(this));
    SCOREBOARD.record(igual, || format!("training_manager_cpp_0x8b2230({:x})", this));
}

/// Comparer: side by side on the random generator's state, on [0xae1080], on the training entry (6 bytes), on the
/// player sheet (+0xf..+0x41, what `apply_training_progress` writes) and on the load of the fitness sheet (31 bytes per player at
/// [[0xb63d28]], +0x1e), with the copy's draws not counted.
///
/// Only the original routine: the error cases and the player with position training (`find_position_training` before running), because
/// there `exe::training_manager_set_position_training_rate`, the contract mark (`exe::staff_contract_apply_event_flags`) and `exe::news_post_player_unhappy_with_training` write outside these regions.
///
/// [pt] Comparador: lado a lado no estado do sorteador, em [0xae1080], na entrada do treino (6 bytes), na ficha (+0xf..+0x41,
/// o que a 0x8b2be0 grava) e na carga da ficha de forma (31 bytes por jogador em [[0xb63d28]], +0x1e), com os sorteios
/// da cópia fora da contagem. Só a original: os casos de erro e o jogador com treino de posição (0x8b4b20 antes de
/// rodar): ali a 0x8b4450, a marca do contrato (0x867700) e a 0x5885c0 mexem fora dessas regiões.
pub(crate) unsafe extern "thiscall" fn compare_update_player_training(this: usize, p: usize, skip: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_UPDATE_PLAYER_TRAINING);
    if p == 0 || read_i32(p) >= read_i32(G_STAFF_COUNT) || read_usize(p + staff::PLAYER) == 0 {
        return o(this, p, skip);
    }
    let mut q = 0usize;
    if exe::training_manager_get_player_schedule(this, p) == 0 || exe::training_manager_find_position_training(this, p, &mut q) > 0 {
        return o(this, p, skip);
    }
    let f = read_usize(p + staff::PLAYER);
    let mut reg = Vec::from(crate::rng::STATE_REGIONS);
    reg.extend_from_slice(&[(G_CONTRACT_LOOKUP_RESULT, 4), (training_entry_of_staff(this, p), 6), (f + player_sheet::GOALKEEPER, 0x33)]);
    let les = read_usize(G_INJURY);
    if les != 0 {
        reg.push((read_usize(les).wrapping_add((read_i32(p) as isize * 31 + 0x1e) as usize), 1));
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, p, skip),
        || crate::rng::without_counting(|| update_player_training(this, p, skip)));
    SCOREBOARD.record(igual, || format!("update_player_training(pessoa {:x}, pular {:x}) {:x?} x {:x?}", p, skip as u16, mo.0, mr.0));
}

/// Comparer for `get_player_schedule`.
///
/// [pt] Comparador: a devolução e [0xae1080] lado a lado; sem pessoa ou sem ficha (a caixa), só a original.
pub(crate) unsafe extern "thiscall" fn compare_get_player_schedule(this: usize, p: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_GET_PLAYER_SCHEDULE);
    if p == 0 || read_usize(p + staff::PLAYER) == 0 {
        return o(this, p);
    }
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(G_CONTRACT_LOOKUP_RESULT, 4)], || o(this, p), || get_player_schedule(this, p));
    SCOREBOARD.record(igual && r == n, || format!("get_player_schedule(pessoa {:x}) original={:x} rust={:x} {:x?} x {:x?}", p, r, n, mo.0, mr.0));
    r
}

/// Comparer for `get_category_level`.
///
/// [pt] Comparador: a devolução inteira (só lê).
pub(crate) unsafe extern "thiscall" fn compare_get_category_level(this: usize, category: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(IDX_GET_CATEGORY_LEVEL)(this, category);
    let n = get_category_level(this, category);
    SCOREBOARD.record(r == n, || format!("get_category_level({:x}, {}) original={:x} rust={:x}", this, category as u8 as i8, r, n));
    r
}

/// Comparador: lado a lado na ficha (+0xf..+0x41), na entrada do treino (6 bytes) e no guardado (0xde1f3c, 5 bytes);
/// sem pessoa ou sem ficha (a caixa), só a original.
pub(crate) unsafe extern "thiscall" fn compare_remove_or_restore_training_progress(this: usize, p: usize, devolver: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_REMOVE_OR_RESTORE_TRAINING_PROGRESS);
    if p == 0 || read_usize(p + staff::PLAYER) == 0 {
        return o(this, p, devolver);
    }
    let f = read_usize(p + staff::PLAYER);
    let reg = [(f + player_sheet::GOALKEEPER, 0x33), (player_training_entry(this, p), 6), (G_TRAINING_PROGRESS_SAVED_DELTA_3, 5)];
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(this, p, devolver), || remove_or_restore_training_progress(this, p, devolver));
    SCOREBOARD.record(igual, || format!("remove_or_restore_training_progress(pessoa {:x}, devolver {}) {:x?} x {:x?}", p, devolver, mo.0, mr.0));
}

/// Comparer for `get_training_attribute_change`.
///
/// [pt] Comparador: lado a lado na ficha e na entrada do treino, com a devolução; sem pessoa ou sem ficha, só a original.
pub(crate) unsafe extern "thiscall" fn compare_get_training_attribute_change(this: usize, p: usize, attribute: u32) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u8>(IDX_GET_TRAINING_ATTRIBUTE_CHANGE);
    if p == 0 || read_usize(p + staff::PLAYER) == 0 {
        return o(this, p, attribute);
    }
    let f = read_usize(p + staff::PLAYER);
    let reg = [(f + player_sheet::GOALKEEPER, 0x33), (player_training_entry(this, p), 6)];
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(this, p, attribute), || get_training_attribute_change(this, p, attribute));
    SCOREBOARD.record(igual && r == n, || format!("get_training_attribute_change(pessoa {:x}, atributo {}) original={} rust={} {:x?} x {:x?}", p, attribute as u8 as i8, r as i8, n as i8, mo.0, mr.0));
    r
}

/// Comparer for `get_position_aptitude`.
///
/// [pt] Comparador: a devolução inteira (só lê); máscara fora (a caixa), só a original.
pub(crate) unsafe extern "thiscall" fn compare_get_position_aptitude(this: usize, p: usize, mask: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_GET_POSITION_APTITUDE);
    if !is_valid_position_mask(mask as u16 as i16 as i32) {
        return o(this, p, mask);
    }
    let r = o(this, p, mask);
    let n = get_position_aptitude(this, p, mask);
    SCOREBOARD.record(r == n, || format!("get_position_aptitude(pessoa {:x}, mascara {:x}) original={:x} rust={:x}", p, mask as u16, r, n));
    r
}

/// Comparer for `set_type`.
///
/// [pt] Comparador: lado a lado nos 0x16 bytes do programa (a vaga, os níveis e o tipo).
pub(crate) unsafe extern "thiscall" fn compare_set_type(this: usize, slot: i8, kind: i8) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i8, i8)>(IDX_SET_TYPE);
    let regions = [(this, 0x16)];
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&regions, || o(this, slot, kind), || set_type(this, slot, kind));
    SCOREBOARD.record(igual, || format!("set_type({:x}, {}, {}) {:x?} x {:x?}", this, slot, kind, mo, mr));
}

/// Comparer for `set_default_schedules`.
///
/// [pt] Comparador: lado a lado nos 167 programas (0x651d bytes a partir de this; nomes incluídos) e no marcador de erro.
pub(crate) unsafe extern "thiscall" fn compare_set_default_schedules(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_SET_DEFAULT_SCHEDULES);
    let regions = [(this, 0x651d), (G_MOUSE_LEFT_BUTTON_DOWN, 4)];
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&regions, || o(this), || set_default_schedules(this));
    SCOREBOARD.record(igual, || {
        let (a, b) = (&mo.0, &mr.0);
        let dif: Vec<usize> = (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect();
        format!("set_default_schedules({:x}) diferencas em {:x?}", this, dif)
    });
}

/// Comparer for `get_manager_schedule`.
///
/// [pt] Comparador: a devolução, sem os casos de erro (só a original).
pub(crate) unsafe extern "thiscall" fn compare_get_manager_schedule(this: usize, manager: usize, slot: i8) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i8) -> usize>(IDX_GET_MANAGER_SCHEDULE);
    if !(0..0x10).contains(&read_i32(G_STAFF_COUNT).wrapping_sub(read_i32(manager)).wrapping_sub(1)) {
        return o(this, manager, slot);
    }
    let r = o(this, manager, slot);
    let n = get_manager_schedule(this, manager, slot);
    SCOREBOARD.record(r == n, || format!("get_manager_schedule(tecnico {:x}, vaga {}) original={:x} rust={:x}", manager, slot, r, n));
    r
}

/// Comparer for `get_coaching_factor`.
///
/// [pt] Comparador: a devolução inteira (a consulta só lê; a comissão do humano é lida pelas duas).
pub(crate) unsafe extern "thiscall" fn compare_get_coaching_factor(this: usize, club: usize, category: i8) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, i8) -> u32>(IDX_GET_COACHING_FACTOR)(this, club, category);
    let n = get_coaching_factor(this, club, category);
    SCOREBOARD.record(r == n, || format!("get_coaching_factor(clube {:x}, categoria {}) original={:x} rust={:x}", club, category, r, n));
    r
}

/// Comparer for `get_coach_assignments`.
///
/// [pt] Comparador: a devolução, sem os casos de erro (só a original).
pub(crate) unsafe extern "thiscall" fn compare_get_coach_assignments(this: usize, manager: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_GET_COACH_ASSIGNMENTS);
    if manager == 0 || !(0..0x10).contains(&read_i32(G_STAFF_COUNT).wrapping_sub(read_i32(manager)).wrapping_sub(1)) {
        return o(this, manager);
    }
    let r = o(this, manager);
    let n = get_coach_assignments(this, manager);
    SCOREBOARD.record(r == n, || format!("get_coach_assignments(tecnico {:x}) original={:x} rust={:x}", manager, r, n));
    r
}

/// Comparer for `get_training_load`.
///
/// [pt] Comparador: a devolução.
pub(crate) unsafe extern "thiscall" fn compare_get_training_load(this: usize) -> i32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_GET_TRAINING_LOAD)(this);
    let n = get_training_load(this);
    SCOREBOARD.record(r == n, || format!("get_training_load({:x}) original={} rust={}", this, r, n));
    r
}

/// Comparer for `apply_training_progress`.
///
/// [pt] Comparador: lado a lado na ficha (+0xf..+0x41) e na entrada de treino do jogador (6 bytes), sem os casos de erro e
/// sem as pessoas que não são jogadores (só a original).
pub(crate) unsafe extern "thiscall" fn compare_apply_training_progress(this: usize, person: usize, d: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_APPLY_TRAINING_PROGRESS);
    if person == 0 || read_usize(person + staff::PLAYER) == 0 || read_i32(person) >= read_i32(G_STAFF_COUNT) {
        return o(this, person, d);
    }
    let f = read_usize(person + staff::PLAYER);
    let e = read_usize(this + 0x651d) + 6 * read_i32(f) as usize;
    let regions = [(f + player_sheet::GOALKEEPER, 0x33), (e, 6)];
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side_light(&regions, || o(this, person, d), || apply_training_progress(this, person, d));
    SCOREBOARD.record(igual, || format!("apply_training_progress(pessoa {:x}, d {:x?}) {:x?} x {:x?}", person, core::slice::from_raw_parts(d as *const i8, 5), mo.0, mr.0));
}

/// Comparer for `find_position_training`.
///
/// [pt] Comparador: a devolução e a saída (reposta entre as duas), sem os casos de erro (só a original).
pub(crate) unsafe extern "thiscall" fn compare_find_position_training(this: usize, person: usize, out: *mut usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut usize) -> i32>(IDX_FIND_POSITION_TRAINING);
    if person == 0 || out.is_null() || read_usize(person + staff::PLAYER) == 0 {
        return o(this, person, out);
    }
    let before = *out;
    let r = o(this, person, out);
    let so = *out;
    *out = before;
    let n = find_position_training(this, person, out);
    let sr = *out;
    *out = so;
    SCOREBOARD.record(r == n && so == sr, || format!("find_position_training(pessoa {:x}) original={} {:x} rust={} {:x}", person, r, so, n, sr));
    r
}

/// Comparer for `has_fast_position_training`.
///
/// [pt] Comparador: a devolução (as duas só leem; a busca roda pelas duas).
pub(crate) unsafe extern "thiscall" fn compare_has_fast_position_training(this: usize, person: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_HAS_FAST_POSITION_TRAINING)(this, person);
    let n = has_fast_position_training(this, person);
    SCOREBOARD.record(r == n, || format!("has_fast_position_training(pessoa {:x}) original={} rust={}", person, r, n));
    r
}

/// Comparer for `advance_week_counter`.
///
/// [pt] Comparador: lado a lado na semana do treino, com a devolução.
pub(crate) unsafe extern "thiscall" fn compare_advance_week_counter(this: usize, day: *const i16) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *const i16) -> i32>(IDX_ADVANCE_WEEK_COUNTER);
    let regions = [(this + 0x688d, 2)];
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&regions, || o(this, day), || advance_week_counter(this, day));
    SCOREBOARD.record(igual && r == n, || format!("advance_week_counter(dia {}) original={} rust={} {:x?} x {:x?}", *day, r, n, mo.0, mr.0));
    r
}

/// Comparer for `init_player_entries`.
///
/// [pt] Comparador: a original aloca (não pode rodar duas vezes): roda só ela e confere cada ficha pela regra (a vaga 6
/// ou 1 pela nota de goleiro e o bit 0x20 do +5 apagado).
pub(crate) unsafe extern "thiscall" fn compare_init_player_entries(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_INIT_PLAYER_ENTRIES)(this);
    let base = read_usize(this + 0x651d);
    if base == 0 {
        return;
    }
    let mut erradas = 0usize;
    let mut first = None;
    for k in 0..read_i32(G_PLAYER_DATA_COUNT).max(0) as usize {
        let f = read_usize(G_PLAYER_DATA_TABLE) + 0x46 * k;
        let e = base.wrapping_add((read_i32(f) as usize).wrapping_mul(6));
        let slot = if *((f + player_sheet::GOALKEEPER) as *const i8) >= 0xf { 6 } else { 1 };
        if *(e as *const u8) != slot || *((e + 5) as *const u8) & 0x20 != 0 {
            erradas += 1;
            first.get_or_insert(k);
        }
    }
    SCOREBOARD.record(erradas == 0, || format!("init_player_entries: {} fichas fora da regra (a primeira {:?})", erradas, first));
}
