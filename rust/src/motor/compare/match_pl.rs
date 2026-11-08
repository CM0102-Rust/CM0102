use super::*;

// The comparers of the routines in ../match_pl.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../match_pl.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `shot` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_shot(j: usize, strength: u32, x: u32, y: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32)>(IDX_SHOT);
    if is_recording_top_level() {
        return o(j, strength, x, y);
    }
    let m = *((j + match_player::ENGINE) as *const usize);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(m, MOTOR), (0xb67a34, 4), (G_CONTRACT_LOOKUP_RESULT, 4)]);
    let t = *((m + engine::MATCH) as *const usize);
    for clube in [*((t + 0x1c) as *const usize), *((t + 0x20) as *const usize)] {
        reg.extend(linhas_do_clube(clube));
    }
    let write = RECORD_716.swap(true, std::sync::atomic::Ordering::Relaxed);
    let (_, _, igual, sequencia, k) = comparar_com_fita(reg, || o(j, strength, x, y), |chamar| {
        shoot_injected(j, strength as i16, x as i8, y as i8, &mut |rotina, this, args| {
            let mask = match rotina {
                0x6d6f70 => [0xffff, u32::MAX, 0xff, 0xff, 0xff],
                0x6cd770 => [0xff, 0xff, 0xffff, 0xff, u32::MAX],
                0x62ed70 => [u32::MAX, 0, 0, 0, 0],
                0x6d0980 => [0xff, 0xff, u32::MAX, u32::MAX, 0],
                0x897760 => [u32::MAX, 0xff, u32::MAX, 0, 0],
                0x6e1aa0 | 0x716c90 | 0x6f0630 | 0x7103e0 => [0, 0, 0, 0, 0],
                _ => [0xff, 0, 0, 0, 0],
            };
            chamar(rotina, this, args, mask)
        })
    });
    RECORD_716.store(write, std::sync::atomic::Ordering::Relaxed);
    PLACAR.registrar(igual && sequencia, || format!("shot(forca {}, alvo {} {}) {} chamadas: {}", strength as i16, x as i8, y as i8, k, tape_detail()));
}

/// Comparer for `shot_outcome` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_shot_outcome(p: usize, f: f32) {
    if is_recording() {
        // unit of the outer tape; inside, its own comparison (nested tape)
        call_recording(FN_MATCH_PLAYER_RESOLVE_OWN_GOAL_CHANCE, p, [f.to_bits(), 0, 0, 0, 0], || {
            compare_shot_outcome(p, f);
            0
        });
        return;
    }
    compare_shot_outcome(p, f);
}

/// Comparer for `shot_on_goal` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_shot_on_goal(p: usize, f: f32) {
    if is_recording() {
        // unit of the outer tape; inside, its own comparison (nested tape)
        call_recording(FN_MATCH_PLAYER_RESOLVE_FLUKE_GOAL_CHANCE, p, [f.to_bits(), 0, 0, 0, 0], || {
            compare_shot_on_goal(p, f);
            0
        });
        return;
    }
    compare_shot_on_goal(p, f);
}

/// Comparer for `mover` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_mover(p: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> u32 {
    if is_recording() {
        // unit of the outer tape; inside, its own comparison (nested tape)
        return call_recording(FN_MATCH_PLAYER_MOVE_TO, p, [a1, a2, a3, a4, a5], || compare_player_move(p, a1, a2, a3, a4, a5));
    }
    compare_player_move(p, a1, a2, a3, a4, a5)
}

/// Comparer for `dribble` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_dribble(p: usize, mode: u32) -> u32 {
    if is_recording() {
        // outer tape unit (penalties, player decision); inside, the comparison itself
        return call_recording(FN_MATCH_PLAYER_DECIDE_BALL_ACTION, p, [mode, 0, 0, 0, 0], || compare_dribble(p, mode));
    }
    compare_dribble(p, mode)
}

/// Comparer for `execute_pass` (comparison mode: see the header of motor.rs).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn comparar_execute_pass(p: usize, x: u32, y: u32, kind: u32, out: *mut u8, receptor: usize, tx: u32, ty: u32) -> u8 {
    if is_recording() {
        // outer tape unit (pass, long ball): the answer with the output byte; inside, the comparison itself
        let mut r = 0u8;
        call_recording(FN_MATCH_PLAYER_PASS, p, [x, y, kind, receptor as u32, tx], || {
            r = compare_execute_pass(p, x, y, kind, out, receptor, tx, ty);
            r as u32 | ((*out as u32) << 8)
        });
        return r;
    }
    compare_execute_pass(p, x, y, kind, out, receptor, tx, ty)
}

/// Comparer for `player_side` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_player_side(j: usize) -> u8 {
    original_fn::<unsafe extern "thiscall" fn(usize) -> u8>(IDX_PLAYER_SIDE)(j)
}

/// Comparer for `shoot` (comparison mode: see the header of motor.rs).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn comparar_shoot(p: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32) {
    if is_recording() {
        // outer tape unit (dribbling, penalties, the shot itself again); inside, its own comparison
        call_recording(FN_MATCH_PLAYER_TAKE_SHOT, p, [a1, a2, a3, a4, a6], || {
            compare_shoot_tape(p, a1, a2, a3, a4, a5, a6);
            0
        });
        return;
    }
    compare_shoot_tape(p, a1, a2, a3, a4, a5, a6)
}

/// Comparer for `destroy_match_player` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_destroy_match_player(j: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_DESTROY_MATCH_PLAYER);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(j + match_player::ENGINE, 4)], || o(j), || destroy_match_player(j));
    PLACAR.registrar(igual, || format!("destroy_match_player({:#x}) {:x?} x {:x?}", j, mo, mr));
}

/// Comparer for `construct_match_player` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_construct_match_player(j: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_CONSTRUCT_MATCH_PLAYER);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(j, 0x1bc)], || o(j), || construct_match_player(j));
    PLACAR.registrar(igual && r == n, || format!("construct_match_player({:#x}) {:x?} x {:x?}", j, mo, mr));
    r
}

/// Comparer for `clear_stored` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_clear_stored(this: usize, l: u8) {
    let f = original_fn::<unsafe extern "thiscall" fn(usize, u8)>(IDX_CLEAR_STORED);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this + 0xf59e, 8)], || f(this, l), || clear_stored(this, l));
    PLACAR.registrar(igual, || format!("clear_stored({}) original={:x?} rust={:x?}", l, mo, mr));
}

/// Comparer for `write_19c` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_write_19c(this: usize, v: i16) {
    let f = original_fn::<unsafe extern "thiscall" fn(usize, i16)>(IDX_WRITE_19C);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this + 0x19c, 2)], || f(this, v), || write_19c(this, v));
    PLACAR.registrar(igual, || format!("write_19c({}) original={:x?} rust={:x?}", v, mo, mr));
}

/// Comparer for `clear_move` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_clear_move(this: usize) {
    let f = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CLEAR_MOVE);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this + 0x475a, 16)], || f(this), || clear_move(this));
    PLACAR.registrar(igual, || format!("clear_move original={:x?} rust={:x?}", mo, mr));
}

/// Comparer for `add_2b` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_add_2b(this: usize, v: u8) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u8)>(IDX_ADD_2B);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this + 0x2b, 1)], || o(this, v), || add_2b(this, v));
    PLACAR.registrar(igual, || format!("add_2b({}) original={:x?} rust={:x?}", v, mo, mr));
}

/// Comparer for `write_2b` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_write_2b(this: usize, v: u8) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u8)>(IDX_WRITE_2B);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this + 0x2b, 1)], || o(this, v), || write_2b(this, v));
    PLACAR.registrar(igual, || format!("write_2b({}) original={:x?} rust={:x?}", v, mo, mr));
}

/// Comparer for `release_ball` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_release_ball(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_RELEASE_BALL);
    let m = *((this + 0x19e) as *const usize);
    let reg = [(m + engine::MOMENTUM_B, 4), (m + engine::POSSESSION_SIDE, 1), (m + engine::BALL_HOLDER, 4), (m + engine::SHOT_PENDING, 1), (this + 0x29, 2), (this + 0x19c, 2)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this), || release_ball(this));
    PLACAR.registrar(igual, || format!("release_ball original={:x?} rust={:x?}", mo, mr));
}

/// Comparer for `code_byte` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "stdcall" fn comparar_code_byte(c: u32) -> i32 {
    let r = original_fn::<unsafe extern "stdcall" fn(u32) -> i32>(IDX_CODE_BYTE)(c);
    let n = code_byte(c);
    PLACAR.registrar(r == n, || format!("code_byte({:#x}) original={} rust={}", c, r, n));
    r
}

/// Comparer for `adjusted_rating` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_adjusted_rating(this: usize, base: i8) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i8) -> i32>(IDX_ADJUSTED_RATING);
    let reg = crate::aleatorio::REGIOES_DO_ESTADO;
    let (r, n, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || o(this, base), || crate::aleatorio::sem_contar(|| adjusted_rating(this, base)));
    PLACAR.registrar(igual && r as u8 == n as u8, || format!("adjusted_rating({}) original={} rust={}", base, r, n));
    r
}

/// Comparer for `adjust_17` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_adjust_17(this: usize, delta: i8) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i8) -> u8>(IDX_ADJUST_17);
    let mut reg = vec![(this + 0x17, 1)];
    reg.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, delta), || crate::aleatorio::sem_contar(|| adjust_17(this, delta)));
    PLACAR.registrar(igual && r == n, || format!("adjust_17({}) original={} {:x?} rust={} {:x?}", delta, r, mo, n, mr));
    r
}

/// Comparer for `take_ball` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_take_ball(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_TAKE_BALL);
    if is_recording_top_level() {
        call_recording(FN_MATCH_PLAYER_TAKE_POSSESSION, this, [0; 5], || {
            o(this);
            0
        });
        return;
    }
    let m = *((this + 0x19e) as *const usize);
    let mut reg = vec![(this, 0x1be)];
    let engine = |mm: usize, reg: &mut Vec<(usize, usize)>| {
        for (a, n) in [(0x1a0usize, 0x30usize), (0x4780, 0x10), (0x8d40, 0x10), (0x8ea0, 0x40), (0xf570, 0x40)] {
            reg.push((mm + a, n));
        }
    };
    if m != 0 {
        engine(m, &mut reg);
        for campo in [0xf582usize, 0xf576] {
            let p = *((m + campo) as *const usize);
            if p != 0 && p != this {
                reg.push((p, 0x40));
                reg.push((p + 0x190, 0x10));
                let mp = *((p + 0x19e) as *const usize);
                if mp != 0 && mp != m {
                    engine(mp, &mut reg);
                }
            }
        }
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this), || take_ball(this));
    PLACAR.registrar(igual, || format!("take_ball(lado {}, {} {}) original={:x?} rust={:x?}", lado(this), byte(this, 0x102), byte(this, 0x103), mo, mr));
}

/// Comparer for `move_to` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_move_to(this: usize, x: i8, y: i8) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i8, i8)>(IDX_MOVE_TO);
    if is_recording_top_level() {
        call_recording(FN_MATCH_PLAYER_MOVE_TO_GRID_CELL, this, [x as u8 as u32, y as u8 as u32, 0, 0, 0], || {
            o(this, x, y);
            0
        });
        return;
    }
    let engine = *((this + 0x19e) as *const usize);
    let antiga = occupancy_grid_cell(engine, byte(this, 0x102), byte(this, 0x103)) as usize;
    let new = occupancy_grid_cell(engine, x, y) as usize;
    let mut reg = vec![(this + 0x28, 1), (this + 0x102, 2), (antiga, 90), (new, 90), (engine + engine::BALL_X, 2)];
    let (l, v) = (lado(this), byte(this, 0x28));
    let n = byte(antiga, 0x58 + l as usize);
    if v >= 0 && n > 0 {
        let movido = *((antiga as isize + 4 * (11 * l as isize + n as isize - 1)) as *const usize);
        if movido != 0 {
            reg.push((movido + 0x28, 1));
        }
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, x, y), || move_to(this, x, y));
    PLACAR.registrar(igual, || format!("move_to({}, {}) original={:x?} rust={:x?}", x, y, mo, mr));
}

/// Comparer for `commentary_a` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_commentary_a(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32)>(IDX_COMMENTARY_A);
    let (igual, n) = comparar_com_comentarios(|| o(this, a1, a2, a3, a4, a5), |c| commentary_two_blank(this, a1, a2, a3, a4, a5, c));
    PLACAR.registrar(igual, || format!("commentary_a({:#x} {:#x} {:#x} {:#x} {:#x}) {} chamadas", a1, a2, a3, a4, a5, n));
}

/// Comparer for `commentary_b` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_commentary_b(this: usize, a1: u32, a2: u32, a3: u32, a4: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32)>(IDX_COMMENTARY_B);
    let (igual, n) = comparar_com_comentarios(|| o(this, a1, a2, a3, a4), |c| commentary_three_blank(this, a1, a2, a3, a4, c));
    PLACAR.registrar(igual, || format!("commentary_b({:#x} {:#x} {:#x} {:#x}) {} chamadas", a1, a2, a3, a4, n));
}

/// Comparer for `condition_test` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_condition_test(this: usize, code: i8, target: usize) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i8, usize) -> u8>(IDX_CONDITION_TEST);
    let reg = crate::aleatorio::REGIOES_DO_ESTADO;
    let (r, n, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || o(this, code, target), || crate::aleatorio::sem_contar(|| condition_test(this, code, target)));
    PLACAR.registrar(igual && r == n, || format!("condition_test({}) match_condition {} original={} rust={}", code, read_f32_at(this + 0x1a2), r, n));
    r
}

/// Comparer for `update_attributes` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_update_attributes(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_UPDATE_ATTRIBUTES);
    // only the player’s record changes: word +0x3b and the fields from +0x71 to +0xf8
    let mut reg: Vec<(usize, usize)> = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.push((this + 0x3b, 2));
    reg.push((this + 0x71, 0x88));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this), || crate::aleatorio::sem_contar(|| update_attributes(this)));
    PLACAR.registrar(igual, || format!("update_attributes original={:x?} rust={:x?}", mo, mr));
}

/// Comparer for `match_performance` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_match_performance(this: usize, mark: u8) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u8)>(IDX_MATCH_PERFORMANCE);
    // the player’s whole record (0x6ec010, itself and 0x6f44a0 write to it), the side byte at
    // +0xaf of the engine and the random state
    let mut reg: Vec<(usize, usize)> = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.push((this, 0x1be));
    let engine = *((this + 0x19e) as *const usize);
    if engine != 0 {
        reg.push(((engine as isize + 0xaf + byte(this, 0x27) as isize) as usize, 1));
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, mark), || crate::aleatorio::sem_contar(|| match_performance(this, mark)));
    PLACAR.registrar(igual, || format!("match_performance({}) original={:x?} rust={:x?}", mark, mo, mr));
}

/// Comparer for `evolve_moment` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_evolve_moment(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_EVOLVE_MOMENT);
    let mut reg: Vec<(usize, usize)> = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.push((this, 0x1be));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this), || crate::aleatorio::sem_contar(|| evolve_moment(this)));
    PLACAR.registrar(igual, || format!("evolve_moment original={:x?} rust={:x?}", mo, mr));
}

/// Comparer for `match_of_interest` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_match_of_interest(p: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_MATCH_OF_INTEREST)(p);
    let v = match_of_interest(p);
    PLACAR.registrar(r == v, || format!("match_of_interest({:#x}) original={} rust={}", p, r, v));
    r
}

/// Comparer for `tackle` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_tackle(j: usize, strong: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_TACKLE);
    if is_recording_top_level() {
        return o(j, strong);
    }
    let m = *((j + match_player::ENGINE) as *const usize);
    // the whole engine, the random roll, the error, 0xae1080 and the tactic row of both clubs (the sending-off
    // pode trocar o capitao)
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(m, MOTOR), (0xb67a34, 4), (G_CONTRACT_LOOKUP_RESULT, 4)]);
    let t = *((m + engine::MATCH) as *const usize);
    for clube in [*((t + 0x1c) as *const usize), *((t + 0x20) as *const usize)] {
        reg.extend(linhas_do_clube(clube));
    }
    let write = RECORD_716.swap(true, std::sync::atomic::Ordering::Relaxed);
    let (_, _, igual, sequencia, k) = comparar_com_fita(reg, || o(j, strong), |chamar| {
        tackle_injected(j, strong as u8, &mut |rotina, this, args| {
            let mask = match rotina {
                0x6d6f70 => [0xffff, u32::MAX, 0xff, 0xff, 0xff],
                0x6cd770 => [0xff, 0xff, 0xffff, 0xff, u32::MAX],
                0x62ed70 => [u32::MAX, 0, 0, 0, 0],
                0x6d0980 => [0xff, 0xff, u32::MAX, u32::MAX, 0],
                0x897760 => [u32::MAX, 0xff, u32::MAX, 0, 0],
                0x6e1aa0 | 0x716c90 | 0x6f0630 => [0, 0, 0, 0, 0],
                _ => [0xff, 0, 0, 0, 0],
            };
            chamar(rotina, this, args, mask)
        })
    });
    RECORD_716.store(write, std::sync::atomic::Ordering::Relaxed);
    PLACAR.registrar(igual && sequencia, || format!("tackle(forte {}) {} chamadas: {}", strong as u8, k, tape_detail()));
}

/// Comparer for `recover_stamina` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_recover_stamina(j: usize, ceiling: u8) {
    let f = original_fn::<unsafe extern "thiscall" fn(usize, u8)>(IDX_RECOVER_STAMINA);
    let regions = [(j + match_player::STAMINA, 8)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&regions, || f(j, ceiling), || recover_stamina(j, ceiling));
    PLACAR.registrar(igual, || format!("recover_stamina(teto {}) original={:x?} rust={:x?}", ceiling, mo, mr));
}

/// Comparer for `save_shot` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_save_shot(j: usize, strength: i16) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i16) -> u32>(IDX_SAVE_SHOT);
    if is_recording_top_level() {
        return o(j, strength);
    }
    let m = *((j + match_player::ENGINE) as *const usize);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(m + 0x1a0, 0x30), (m + engine::GRID, 0x8f00 - 0x215e), (m + engine::FORMATION_TABLE, 0xa0)]);
    let (r, n, igual, sequencia, k) = comparar_com_fita(reg, || o(j, strength), |chamar| {
        save_shot_injected(j, strength, &mut |rotina, this, args| {
            chamar(rotina, this, args, if rotina == 0x6d6f70 { [0xffff, u32::MAX, 0xff, 0xff, 0xff] } else { [0xff, 0xff, 0xffff, 0xff, u32::MAX] })
        })
    });
    PLACAR.registrar(igual && sequencia && r == n, || format!("save_shot(forca {}) original={} rust={} {} chamadas: {}", strength, r, n, k, tape_detail()));
    r
}

/// Comparer for `long_ball` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_long_ball(j: usize, mode: i8, refinar: u8) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i8, u8)>(IDX_LONG_BALL);
    if is_recording_top_level() {
        return o(j, mode, refinar);
    }
    let m = *((j + match_player::ENGINE) as *const usize);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(m + 0x1a0, 0x30), (m + engine::GRID, 0x8f00 - 0x215e), (m + engine::FORMATION_TABLE, 0xa0)]);
    const MASCARAS: [(usize, [u32; 5]); 3] = [
        (0x6fad40, [0xff, 0xff, u32::MAX, u32::MAX, 0xff]),
        (0x6d6f70, [0xffff, u32::MAX, 0xff, 0xff, 0xff]),
        (0x6cd490, [0xff, 0, 0, 0, 0]),
    ];
    let (_, _, igual, sequencia, k) = comparar_com_fita(reg, || o(j, mode, refinar), |chamar| {
        long_ball_injected(j, mode, refinar, &mut |rotina, this, args| {
            let mask = MASCARAS.iter().find(|m| m.0 == rotina).map_or([u32::MAX; 5], |m| m.1);
            // the port of 0x6fad40 writes (x, y, type, receiver, column); the target row is not included
            let args = if rotina == 0x6fad40 { [args[0], args[1], args[2], 0, args[3]] } else { args };
            chamar(rotina, this, args, mask)
        })
    });
    PLACAR.registrar(igual && sequencia, || format!("long_ball(modo {} refinar {}) {} chamadas: {}", mode, refinar, k, tape_detail()));
}

/// Comparer for `pass_ball` (comparison mode: see the header of motor.rs).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn comparar_pass_ball(j: usize, kind: u8, receptor: usize, a3: u32, a4: u32, a5: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u8, usize, u32, u32, u32)>(IDX_PASS_BALL);
    if is_recording_top_level() {
        return o(j, kind, receptor, a3, a4, a5);
    }
    let m = *((j + match_player::ENGINE) as *const usize);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(m + 0x1a0, 0x30), (m + engine::GRID, 0x8f00 - 0x215e), (m + engine::FORMATION_TABLE, 0xa0)]);
    const MASCARAS: [(usize, [u32; 5]); 3] = [
        (0x6fad40, [0xff, 0xff, 0xff, u32::MAX, u32::MAX]),
        (0x6d6f70, [0xffff, u32::MAX, 0xff, 0xff, 0xff]),
        (0x7103e0, [0; 5]),
    ];
    let (_, _, igual, sequencia, k) = comparar_com_fita(reg, || o(j, kind, receptor, a3, a4, a5), |chamar| {
        pass_ball_injected(j, kind, receptor, a4, a5, &mut |rotina, this, args| {
            let mask = MASCARAS.iter().find(|m| m.0 == rotina).map_or([u32::MAX; 5], |m| m.1);
            chamar(rotina, this, args, mask)
        })
    });
    PLACAR.registrar(igual && sequencia, || format!("pass_ball(tipo {:#x}) {} chamadas: {}", kind, k, tape_detail()));
}

/// Comparer for `decide_player` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_decide_player(j: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_DECIDE_PLAYER);
    if is_recording_top_level() {
        call_recording(FN_MATCH_PLAYER_TAKE_TURN, j, [0; 5], || {
            o(j);
            0
        });
        return;
    }
    let m = *((j + match_player::ENGINE) as *const usize);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    // what the decision writes and what it rereads after carrying the ball (ball owner, +0x8eb2, side’s situation)
    reg.extend([(m + 0x1a0, 0x30), (m + engine::GRID, 0x8f00 - 0x215e), (m + engine::FORMATION_TABLE, 0x48)]);
    let (_, _, igual, sequencia, k) = comparar_com_fita(reg, || o(j), |chamar| {
        player_decide_injected(j, &mut |rotina, this, args| chamar(rotina, this, args, if rotina == 0x713fc0 { [u32::MAX, 0, 0, 0, 0] } else { [0; 5] }))
    });
    PLACAR.registrar(igual && sequencia, || format!("decide_player acao {:#x} {} chamadas", *((j + match_player::ACTION) as *const u16), k));
}

/// Comparer for `move_off_ball` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_move_off_ball(j: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_MOVE_OFF_BALL);
    if is_recording_top_level() {
        call_recording(FN_MATCH_PLAYER_CHOOSE_DEFENSIVE_OFF_BALL_MOVE, j, [0; 5], || {
            o(j);
            0
        });
        return;
    }
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    // the Rust copy runs after the whole original: the recorded calls (0x6f09f0 decides and moves the
    // other players) change what it reads before them; all players, the grid and the ball go in
    let m = *((j + match_player::ENGINE) as *const usize);
    reg.extend([(m + 0x1a0, 0x30), (m + engine::GRID, 0x8f00 - 0x215e), (m + engine::FORMATION_TABLE, 0x48)]);
    const MASCARAS: [(usize, [u32; 5]); 5] = [
        (0x6f09f0, XY_ARG_MASK),
        (0x6f46b0, XY_ARG_MASK),
        (0x6f5b80, [0xff, 0, 0, 0, 0]),
        (0x6d6f70, [0xffff, u32::MAX, 0xff, 0xff, 0xff]),
        (0x6f0630, [0; 5]),
    ];
    let (_, _, igual, sequencia, k) = comparar_com_fita(reg, || o(j), |chamar| {
        off_ball_move_injected(j, &mut |rotina, this, args| {
            let mask = MASCARAS.iter().find(|m| m.0 == rotina).map_or(XY_ARG_MASK, |m| m.1);
            chamar(rotina, this, args, mask)
        })
    });
    PLACAR.registrar(igual && sequencia, || format!("move_off_ball acao {:#x} {} chamadas", *((j + match_player::ACTION) as *const u16), k));
}

/// Comparer for `decide_off_ball` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_decide_off_ball(j: usize, a: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_DECIDE_OFF_BALL);
    if is_recording_top_level() {
        call_recording(FN_MATCH_PLAYER_CHOOSE_ATTACKING_OFF_BALL_MOVE, j, [a, 0, 0, 0, 0], || {
            o(j, a);
            0
        });
        return;
    }
    // the Rust copy runs after the whole original: what it (and what it calls without writing: 0x6c2be0,
    // 0x6bce80, 0x6c4130...) reads before a recorded call and that this call changes must go back to
    // previous state. The recorded calls touch the player (cell, direction), the grid and the rows.
    let m = *((j + match_player::ENGINE) as *const usize);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    // the written 0x6f09f0 also moves the other players (decides for them): all of them go in
    reg.extend([(m + 0x1a0, 0x30), (m + engine::GRID, 0x8f00 - 0x215e), (m + engine::FORMATION_TABLE, 0x48)]);
    // 0x6f09f0 e 0x6f46b0 leem x e y como bytes; 0x6f5b80, so o `direto`
    const MASCARAS: [(usize, [u32; 5]); 3] = [(0x6f09f0, XY_ARG_MASK), (0x6f46b0, XY_ARG_MASK), (0x6f5b80, [0xff, 0, 0, 0, 0])];
    let (_, _, igual, sequencia, k) = comparar_com_fita(reg, || o(j, a), |chamar| {
        off_ball_decide_injected(j, &mut |rotina, args| {
            let mask = MASCARAS.iter().find(|m| m.0 == rotina).map_or(XY_ARG_MASK, |m| m.1);
            chamar(rotina, j, args, mask)
        })
    });
    PLACAR.registrar(igual && sequencia && k == 1, || format!("decide_off_ball acao {:#x} {} chamadas: {}", *((j + match_player::ACTION) as *const u16), k, tape_detail()));
}

/// Comparer for `advance_row` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_advance_row(this: usize, direct: u8) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u8) -> u32>(IDX_ADVANCE_ROW);
    if is_recording_top_level() {
        return call_recording(FN_MATCH_PLAYER_DROP_BACK_ONE_ROW, this, [direct as u32, 0, 0, 0, 0], || o(this, direct));
    }
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.push((this + 0x102, 2));
    let (r, _, igual, sequencia, k) = comparar_com_fita(reg, || o(this, direct), |chamar| {
        advance_row_injected(this, direct, &mut |rotina, a| chamar(rotina, this, a, XY_ARG_MASK))
    });
    // the answer and the call’s `eax` (the movement returns nothing): not part of the comparison
    PLACAR.registrar(igual && sequencia && k == 1, || format!("advance_row({}) {} chamadas", direct, k));
    r
}

/// Comparer for `designated_marked` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_designated_marked(this: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_DESIGNATED_MARKED)(this);
    let n = designated_marked(this);
    PLACAR.registrar(r == n, || format!("designated_marked original={:x} rust={:x}", r, n));
    r
}
