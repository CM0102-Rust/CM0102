use super::*;

// The comparers of the routines in ../match_pl.rs.
// [pt] Os comparadores das rotinas de ../match_pl.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `shot`.
pub(crate) unsafe extern "thiscall" fn compare_shot(j: usize, strength: u32, x: u32, y: u32) {
    let j = MatchPlayer(j);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32)>(IDX_SHOT);
    if is_recording_top_level() {
        return o(j.addr(), strength, x, y);
    }
    let m = j.engine();
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.extend([(m, ENGINE_SIZE), (G_MOUSE_LEFT_BUTTON_DOWN, 4), (G_CONTRACT_LOOKUP_RESULT, 4)]);
    let t = Fixture(*((m + engine::MATCH) as *const usize));
    for clube in [t.home_club(), t.away_club()] {
        reg.extend(club_rows(clube));
    }
    let write = RECORD_716.swap(true, std::sync::atomic::Ordering::Relaxed);
    let (_, _, igual, sequencia, k) = compare_com_fita(reg, || o(j.addr(), strength, x, y), |chamar| {
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
    SCOREBOARD.record(igual && sequencia, || format!("shot(forca {}, alvo {} {}) {} chamadas: {}", strength as i16, x as i8, y as i8, k, tape_detail()));
}

/// Comparer for `shot_outcome`.
pub(crate) unsafe extern "thiscall" fn compare_shot_outcome(p: usize, f: f32) {
    if is_recording() {
        // unit of the outer tape; inside, its own comparison (nested tape)
        call_recording(FN_MATCH_PLAYER_RESOLVE_OWN_GOAL_CHANCE, p, [f.to_bits(), 0, 0, 0, 0], || {
            compare_shot_outcome_inner(MatchPlayer(p), f);
            0
        });
        return;
    }
    compare_shot_outcome_inner(MatchPlayer(p), f);
}

/// Comparer for `shot_on_goal`.
pub(crate) unsafe extern "thiscall" fn compare_shot_on_goal(p: usize, f: f32) {
    if is_recording() {
        // unit of the outer tape; inside, its own comparison (nested tape)
        call_recording(FN_MATCH_PLAYER_RESOLVE_FLUKE_GOAL_CHANCE, p, [f.to_bits(), 0, 0, 0, 0], || {
            compare_shot_on_goal_inner(MatchPlayer(p), f);
            0
        });
        return;
    }
    compare_shot_on_goal_inner(MatchPlayer(p), f);
}

/// Comparer for `mover`.
pub(crate) unsafe extern "thiscall" fn compare_mover(p: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> u32 {
    if is_recording() {
        // unit of the outer tape; inside, its own comparison (nested tape)
        return call_recording(FN_MATCH_PLAYER_MOVE_TO, p, [a1, a2, a3, a4, a5], || compare_player_move(MatchPlayer(p), a1, a2, a3, a4, a5));
    }
    compare_player_move(MatchPlayer(p), a1, a2, a3, a4, a5)
}

/// Comparer for `dribble`.
pub(crate) unsafe extern "thiscall" fn compare_dribble(p: usize, mode: u32) -> u32 {
    if is_recording() {
        // outer tape unit (penalties, player decision); inside, the comparison itself
        return call_recording(FN_MATCH_PLAYER_DECIDE_BALL_ACTION, p, [mode, 0, 0, 0, 0], || compare_dribble_inner(MatchPlayer(p), mode));
    }
    compare_dribble_inner(MatchPlayer(p), mode)
}

/// Comparer for `execute_pass`.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_execute_pass(p: usize, x: u32, y: u32, kind: u32, out: *mut u8, receiver: usize, tx: u32, ty: u32) -> u8 {
    if is_recording() {
        // outer tape unit (pass, long ball): the answer with the output byte; inside, the comparison itself
        let mut r = 0u8;
        call_recording(FN_MATCH_PLAYER_PASS, p, [x, y, kind, receiver as u32, tx], || {
            r = compare_execute_pass_inner(MatchPlayer(p), x, y, kind, out, receiver, tx, ty);
            r as u32 | ((*out as u32) << 8)
        });
        return r;
    }
    compare_execute_pass_inner(MatchPlayer(p), x, y, kind, out, receiver, tx, ty)
}

/// Comparer for `player_side`.
pub(crate) unsafe extern "thiscall" fn compare_player_side(j: usize) -> u8 {
    original_fn::<unsafe extern "thiscall" fn(usize) -> u8>(IDX_PLAYER_SIDE)(j)
}

/// Comparer for `shoot`.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_shoot(p: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32, a6: u32) {
    if is_recording() {
        // outer tape unit (dribbling, penalties, the shot itself again); inside, its own comparison
        call_recording(FN_MATCH_PLAYER_TAKE_SHOT, p, [a1, a2, a3, a4, a6], || {
            compare_shoot_tape(MatchPlayer(p), a1, a2, a3, a4, a5, a6);
            0
        });
        return;
    }
    compare_shoot_tape(MatchPlayer(p), a1, a2, a3, a4, a5, a6)
}

/// Comparer for `destroy_match_player`.
pub(crate) unsafe extern "thiscall" fn compare_destroy_match_player(j: usize) {
    let j = MatchPlayer(j);
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_DESTROY_MATCH_PLAYER);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side_light(&[(j.addr() + match_player::ENGINE, 4)], || o(j.addr()), || destroy_match_player(j.addr()));
    SCOREBOARD.record(igual, || format!("destroy_match_player({:#x}) {:x?} x {:x?}", j.addr(), mo, mr));
}

/// Comparer for `construct_match_player`.
pub(crate) unsafe extern "thiscall" fn compare_construct_match_player(j: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_CONSTRUCT_MATCH_PLAYER);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&[(j, 0x1bc)], || o(j), || construct_match_player(j));
    SCOREBOARD.record(igual && r == n, || format!("construct_match_player({:#x}) {:x?} x {:x?}", j, mo, mr));
    r
}

/// Comparer for `clear_stored`.
pub(crate) unsafe extern "thiscall" fn compare_clear_stored(this: usize, l: u8) {
    let f = original_fn::<unsafe extern "thiscall" fn(usize, u8)>(IDX_CLEAR_STORED);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&[(this + engine::GOALKEEPER, 8)], || f(this, l), || clear_stored(this, l));
    SCOREBOARD.record(igual, || format!("clear_stored({}) original={:x?} rust={:x?}", l, mo, mr));
}

/// Comparer for `write_19c`.
pub(crate) unsafe extern "thiscall" fn compare_write_19c(this: usize, v: i16) {
    let this = MatchPlayer(this);
    let f = original_fn::<unsafe extern "thiscall" fn(usize, i16)>(IDX_WRITE_19C);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&[(this.addr() + match_player::BALL_CONTROL, 2)], || f(this.addr(), v), || write_19c(this.addr(), v));
    SCOREBOARD.record(igual, || format!("write_19c({}) original={:x?} rust={:x?}", v, mo, mr));
}

/// Comparer for `clear_move`.
pub(crate) unsafe extern "thiscall" fn compare_clear_move(this: usize) {
    let f = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CLEAR_MOVE);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&[(this + engine::PENDING_PLAY_KIND, 16)], || f(this), || clear_move(this));
    SCOREBOARD.record(igual, || format!("clear_move original={:x?} rust={:x?}", mo, mr));
}

/// Comparer for `add_2b`.
pub(crate) unsafe extern "thiscall" fn compare_add_2b(this: usize, v: u8) {
    let this = MatchPlayer(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u8)>(IDX_ADD_2B);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&[(this.addr() + match_player::BUSY, 1)], || o(this.addr(), v), || add_2b(this.addr(), v));
    SCOREBOARD.record(igual, || format!("add_2b({}) original={:x?} rust={:x?}", v, mo, mr));
}

/// Comparer for `write_2b`.
pub(crate) unsafe extern "thiscall" fn compare_write_2b(this: usize, v: u8) {
    let this = MatchPlayer(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u8)>(IDX_WRITE_2B);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&[(this.addr() + match_player::BUSY, 1)], || o(this.addr(), v), || write_2b(this.addr(), v));
    SCOREBOARD.record(igual, || format!("write_2b({}) original={:x?} rust={:x?}", v, mo, mr));
}

/// Comparer for `release_ball`.
pub(crate) unsafe extern "thiscall" fn compare_release_ball(this: usize) {
    let this = MatchPlayer(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_RELEASE_BALL);
    let m = this.engine();
    let reg = [(m + engine::MOMENTUM_B, 4), (m + engine::POSSESSION_SIDE, 1), (m + engine::BALL_HOLDER, 4), (m + engine::SHOT_PENDING, 1), (this.addr() + 0x29, 2), (this.addr() + match_player::BALL_CONTROL, 2)];
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this.addr()), || release_ball(this.addr()));
    SCOREBOARD.record(igual, || format!("release_ball original={:x?} rust={:x?}", mo, mr));
}

/// Comparer for `code_byte`.
pub(crate) unsafe extern "stdcall" fn compare_code_byte(c: u32) -> i32 {
    let r = original_fn::<unsafe extern "stdcall" fn(u32) -> i32>(IDX_CODE_BYTE)(c);
    let n = code_byte(c);
    SCOREBOARD.record(r == n, || format!("code_byte({:#x}) original={} rust={}", c, r, n));
    r
}

/// Comparer for `adjusted_rating`.
pub(crate) unsafe extern "thiscall" fn compare_adjusted_rating(this: usize, base: i8) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i8) -> i32>(IDX_ADJUSTED_RATING);
    let reg = crate::rng::STATE_REGIONS;
    let (r, n, igual, _, _) = crate::comparison::side_by_side(&reg, || o(this, base), || crate::rng::without_counting(|| adjusted_rating(this, base)));
    SCOREBOARD.record(igual && r as u8 == n as u8, || format!("adjusted_rating({}) original={} rust={}", base, r, n));
    r
}

/// Comparer for `adjust_17`.
pub(crate) unsafe extern "thiscall" fn compare_adjust_17(this: usize, delta: i8) -> u8 {
    let this = MatchPlayer(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i8) -> u8>(IDX_ADJUST_17);
    let mut reg = vec![(this.addr() + match_player::MATCH_FORM, 1)];
    reg.extend(crate::rng::STATE_REGIONS);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this.addr(), delta), || crate::rng::without_counting(|| adjust_17(this.addr(), delta)));
    SCOREBOARD.record(igual && r == n, || format!("adjust_17({}) original={} {:x?} rust={} {:x?}", delta, r, mo, n, mr));
    r
}

/// Comparer for `take_ball`.
pub(crate) unsafe extern "thiscall" fn compare_take_ball(this: usize) {
    let this = MatchPlayer(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_TAKE_BALL);
    if is_recording_top_level() {
        call_recording(FN_MATCH_PLAYER_TAKE_POSSESSION, this.addr(), [0; 5], || {
            o(this.addr());
            0
        });
        return;
    }
    let m = this.engine();
    let mut reg = vec![(this.addr(), 0x1be)];
    let engine = |mm: usize, reg: &mut Vec<(usize, usize)>| {
        for (a, n) in [(0x1a0usize, 0x30usize), (0x4780, 0x10), (0x8d40, 0x10), (0x8ea0, 0x40), (0xf570, 0x40)] {
            reg.push((mm + a, n));
        }
    };
    if m != 0 {
        engine(m, &mut reg);
        for campo in [0xf582usize, 0xf576] {
            let p = MatchPlayer(*((m + campo) as *const usize));
            if !p.is_null() && p != this {
                reg.push((p.addr(), 0x40));
                reg.push((p.addr() + 0x190, 0x10));
                let mp = p.engine();
                if mp != 0 && mp != m {
                    engine(mp, &mut reg);
                }
            }
        }
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this.addr()), || take_ball(this.addr()));
    SCOREBOARD.record(igual, || format!("take_ball(lado {}, {} {}) original={:x?} rust={:x?}", side_of_player(this), this.cell_x(), this.cell_y(), mo, mr));
}

/// Comparer for `move_to`.
pub(crate) unsafe extern "thiscall" fn compare_move_to(this: usize, x: i8, y: i8) {
    let this = MatchPlayer(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i8, i8)>(IDX_MOVE_TO);
    if is_recording_top_level() {
        call_recording(FN_MATCH_PLAYER_MOVE_TO_GRID_CELL, this.addr(), [x as u8 as u32, y as u8 as u32, 0, 0, 0], || {
            o(this.addr(), x, y);
            0
        });
        return;
    }
    let engine = this.engine();
    let old = occupancy_grid_cell(engine, this.cell_x(), this.cell_y()) as usize;
    let new = occupancy_grid_cell(engine, x, y) as usize;
    let mut reg = vec![(this.addr() + match_player::CELL_SLOT, 1), (this.addr() + match_player::CELL_X, 2), (old, 90), (new, 90), (engine + engine::BALL_X, 2)];
    let (l, v) = (side_of_player(this), this.cell_slot());
    let n = read_i8(old + (0x58 + l as usize));
    if v >= 0 && n > 0 {
        let movido = *((old as isize + 4 * (11 * l as isize + n as isize - 1)) as *const usize);
        if movido != 0 {
            reg.push((movido + 0x28, 1));
        }
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this.addr(), x, y), || move_to(this.addr(), x, y));
    SCOREBOARD.record(igual, || format!("move_to({}, {}) original={:x?} rust={:x?}", x, y, mo, mr));
}

/// Comparer for `commentary_a`.
pub(crate) unsafe extern "thiscall" fn compare_commentary_a(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32)>(IDX_COMMENTARY_A);
    let (igual, n) = compare_with_commentary(|| o(this, a1, a2, a3, a4, a5), |c| commentary_two_blank(this, a1, a2, a3, a4, a5, c));
    SCOREBOARD.record(igual, || format!("commentary_a({:#x} {:#x} {:#x} {:#x} {:#x}) {} chamadas", a1, a2, a3, a4, a5, n));
}

/// Comparer for `commentary_b`.
pub(crate) unsafe extern "thiscall" fn compare_commentary_b(this: usize, a1: u32, a2: u32, a3: u32, a4: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32)>(IDX_COMMENTARY_B);
    let (igual, n) = compare_with_commentary(|| o(this, a1, a2, a3, a4), |c| commentary_three_blank(this, a1, a2, a3, a4, c));
    SCOREBOARD.record(igual, || format!("commentary_b({:#x} {:#x} {:#x} {:#x}) {} chamadas", a1, a2, a3, a4, n));
}

/// Comparer for `condition_test`.
pub(crate) unsafe extern "thiscall" fn compare_condition_test(this: usize, code: i8, target: usize) -> u8 {
    let this = MatchPlayer(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i8, usize) -> u8>(IDX_CONDITION_TEST);
    let reg = crate::rng::STATE_REGIONS;
    let (r, n, igual, _, _) = crate::comparison::side_by_side(&reg, || o(this.addr(), code, target), || crate::rng::without_counting(|| condition_test(this.addr(), code, target)));
    SCOREBOARD.record(igual && r == n, || format!("condition_test({}) match_condition {} original={} rust={}", code, this.confidence(), r, n));
    r
}

/// Comparer for `update_attributes`.
pub(crate) unsafe extern "thiscall" fn compare_update_attributes(this: usize) {
    let this = MatchPlayer(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_UPDATE_ATTRIBUTES);
    // only the player’s record changes: word +0x3b and the fields from +0x71 to +0xf8
    let mut reg: Vec<(usize, usize)> = crate::rng::STATE_REGIONS.to_vec();
    reg.push((this.addr() + match_player::EFFECTIVE_ABILITY, 2));
    reg.push((this.addr() + match_player::ACCELERATION, 0x88));
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this.addr()), || crate::rng::without_counting(|| update_attributes(this.addr())));
    SCOREBOARD.record(igual, || format!("update_attributes original={:x?} rust={:x?}", mo, mr));
}

/// Comparer for `match_performance`.
pub(crate) unsafe extern "thiscall" fn compare_match_performance(this: usize, mark: u8) {
    let this = MatchPlayer(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u8)>(IDX_MATCH_PERFORMANCE);
    // the player’s whole record (0x6ec010, itself and 0x6f44a0 write to it), the side byte at
    // +0xaf of the engine and the random state
    let mut reg: Vec<(usize, usize)> = crate::rng::STATE_REGIONS.to_vec();
    reg.push((this.addr(), 0x1be));
    let engine = this.engine();
    if engine != 0 {
        reg.push(((engine as isize + 0xaf + this.side() as isize) as usize, 1));
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this.addr(), mark), || crate::rng::without_counting(|| match_performance(this.addr(), mark)));
    SCOREBOARD.record(igual, || format!("match_performance({}) original={:x?} rust={:x?}", mark, mo, mr));
}

/// Comparer for `evolve_moment`.
pub(crate) unsafe extern "thiscall" fn compare_evolve_moment(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_EVOLVE_MOMENT);
    let mut reg: Vec<(usize, usize)> = crate::rng::STATE_REGIONS.to_vec();
    reg.push((this, 0x1be));
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this), || crate::rng::without_counting(|| evolve_moment(this)));
    SCOREBOARD.record(igual, || format!("evolve_moment original={:x?} rust={:x?}", mo, mr));
}

/// Comparer for `match_of_interest`.
pub(crate) unsafe extern "C" fn compare_match_of_interest(p: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(IDX_MATCH_OF_INTEREST)(p);
    let v = match_of_interest(p);
    SCOREBOARD.record(r == v, || format!("match_of_interest({:#x}) original={} rust={}", p, r, v));
    r
}

/// Comparer for `tackle`.
pub(crate) unsafe extern "thiscall" fn compare_tackle(j: usize, strong: u32) {
    let j = MatchPlayer(j);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_TACKLE);
    if is_recording_top_level() {
        return o(j.addr(), strong);
    }
    let m = j.engine();
    // the whole engine, the random roll, the error, 0xae1080 and the tactic row of both clubs (the sending-off
    // pode trocar o capitao)
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.extend([(m, ENGINE_SIZE), (G_MOUSE_LEFT_BUTTON_DOWN, 4), (G_CONTRACT_LOOKUP_RESULT, 4)]);
    let t = Fixture(*((m + engine::MATCH) as *const usize));
    for clube in [t.home_club(), t.away_club()] {
        reg.extend(club_rows(clube));
    }
    let write = RECORD_716.swap(true, std::sync::atomic::Ordering::Relaxed);
    let (_, _, igual, sequencia, k) = compare_com_fita(reg, || o(j.addr(), strong), |chamar| {
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
    SCOREBOARD.record(igual && sequencia, || format!("tackle(forte {}) {} chamadas: {}", strong as u8, k, tape_detail()));
}

/// Comparer for `recover_stamina`.
pub(crate) unsafe extern "thiscall" fn compare_recover_stamina(j: usize, ceiling: u8) {
    let j = MatchPlayer(j);
    let f = original_fn::<unsafe extern "thiscall" fn(usize, u8)>(IDX_RECOVER_STAMINA);
    let regions = [(j.addr() + match_player::STAMINA, 8)];
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side_light(&regions, || f(j.addr(), ceiling), || recover_stamina(j.addr(), ceiling));
    SCOREBOARD.record(igual, || format!("recover_stamina(teto {}) original={:x?} rust={:x?}", ceiling, mo, mr));
}

/// Comparer for `save_shot`.
pub(crate) unsafe extern "thiscall" fn compare_save_shot(j: usize, strength: i16) -> u32 {
    let j = MatchPlayer(j);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i16) -> u32>(IDX_SAVE_SHOT);
    if is_recording_top_level() {
        return o(j.addr(), strength);
    }
    let m = j.engine();
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.extend([(m + 0x1a0, 0x30), (m + engine::GRID, 0x8f00 - 0x215e), (m + engine::FORMATION_TABLE, 0xa0)]);
    let (r, n, igual, sequencia, k) = compare_com_fita(reg, || o(j.addr(), strength), |chamar| {
        save_shot_injected(j, strength, &mut |rotina, this, args| {
            chamar(rotina, this, args, if rotina == 0x6d6f70 { [0xffff, u32::MAX, 0xff, 0xff, 0xff] } else { [0xff, 0xff, 0xffff, 0xff, u32::MAX] })
        })
    });
    SCOREBOARD.record(igual && sequencia && r == n, || format!("save_shot(forca {}) original={} rust={} {} chamadas: {}", strength, r, n, k, tape_detail()));
    r
}

/// Comparer for `long_ball`.
pub(crate) unsafe extern "thiscall" fn compare_long_ball(j: usize, mode: i8, refinar: u8) {
    let j = MatchPlayer(j);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i8, u8)>(IDX_LONG_BALL);
    if is_recording_top_level() {
        return o(j.addr(), mode, refinar);
    }
    let m = j.engine();
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.extend([(m + 0x1a0, 0x30), (m + engine::GRID, 0x8f00 - 0x215e), (m + engine::FORMATION_TABLE, 0xa0)]);
    const MASCARAS: [(usize, [u32; 5]); 3] = [
        (0x6fad40, [0xff, 0xff, u32::MAX, u32::MAX, 0xff]),
        (0x6d6f70, [0xffff, u32::MAX, 0xff, 0xff, 0xff]),
        (0x6cd490, [0xff, 0, 0, 0, 0]),
    ];
    let (_, _, igual, sequencia, k) = compare_com_fita(reg, || o(j.addr(), mode, refinar), |chamar| {
        long_ball_injected(j, mode, refinar, &mut |rotina, this, args| {
            let mask = MASCARAS.iter().find(|m| m.0 == rotina).map_or([u32::MAX; 5], |m| m.1);
            // the port of 0x6fad40 writes (x, y, type, receiver, column); the target row is not included
            let args = if rotina == 0x6fad40 { [args[0], args[1], args[2], 0, args[3]] } else { args };
            chamar(rotina, this, args, mask)
        })
    });
    SCOREBOARD.record(igual && sequencia, || format!("long_ball(modo {} refinar {}) {} chamadas: {}", mode, refinar, k, tape_detail()));
}

/// Comparer for `pass_ball`.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_pass_ball(j: usize, kind: u8, receiver: usize, a3: u32, a4: u32, a5: u32) {
    let j = MatchPlayer(j);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u8, usize, u32, u32, u32)>(IDX_PASS_BALL);
    if is_recording_top_level() {
        return o(j.addr(), kind, receiver, a3, a4, a5);
    }
    let m = j.engine();
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.extend([(m + 0x1a0, 0x30), (m + engine::GRID, 0x8f00 - 0x215e), (m + engine::FORMATION_TABLE, 0xa0)]);
    const MASCARAS: [(usize, [u32; 5]); 3] = [
        (0x6fad40, [0xff, 0xff, 0xff, u32::MAX, u32::MAX]),
        (0x6d6f70, [0xffff, u32::MAX, 0xff, 0xff, 0xff]),
        (0x7103e0, [0; 5]),
    ];
    let (_, _, igual, sequencia, k) = compare_com_fita(reg, || o(j.addr(), kind, receiver, a3, a4, a5), |chamar| {
        pass_ball_injected(j, kind, receiver, a4, a5, &mut |rotina, this, args| {
            let mask = MASCARAS.iter().find(|m| m.0 == rotina).map_or([u32::MAX; 5], |m| m.1);
            chamar(rotina, this, args, mask)
        })
    });
    SCOREBOARD.record(igual && sequencia, || format!("pass_ball(tipo {:#x}) {} chamadas: {}", kind, k, tape_detail()));
}

/// Comparer for `decide_player`.
pub(crate) unsafe extern "thiscall" fn compare_decide_player(j: usize) {
    let j = MatchPlayer(j);
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_DECIDE_PLAYER);
    if is_recording_top_level() {
        call_recording(FN_MATCH_PLAYER_TAKE_TURN, j.addr(), [0; 5], || {
            o(j.addr());
            0
        });
        return;
    }
    let m = j.engine();
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    // what the decision writes and what it rereads after carrying the ball (ball owner, +0x8eb2, side’s situation)
    reg.extend([(m + 0x1a0, 0x30), (m + engine::GRID, 0x8f00 - 0x215e), (m + engine::FORMATION_TABLE, 0x48)]);
    let (_, _, igual, sequencia, k) = compare_com_fita(reg, || o(j.addr()), |chamar| {
        player_decide_injected(j, &mut |rotina, this, args| chamar(rotina, this, args, if rotina == 0x713fc0 { [u32::MAX, 0, 0, 0, 0] } else { [0; 5] }))
    });
    SCOREBOARD.record(igual && sequencia, || format!("decide_player acao {:#x} {} chamadas", j.action(), k));
}

/// Comparer for `move_off_ball`.
pub(crate) unsafe extern "thiscall" fn compare_move_off_ball(j: usize) {
    let j = MatchPlayer(j);
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_MOVE_OFF_BALL);
    if is_recording_top_level() {
        call_recording(FN_MATCH_PLAYER_CHOOSE_DEFENSIVE_OFF_BALL_MOVE, j.addr(), [0; 5], || {
            o(j.addr());
            0
        });
        return;
    }
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    // the Rust copy runs after the whole original: the recorded calls (0x6f09f0 decides and moves the
    // other players) change what it reads before them; all players, the grid and the ball go in
    let m = j.engine();
    reg.extend([(m + 0x1a0, 0x30), (m + engine::GRID, 0x8f00 - 0x215e), (m + engine::FORMATION_TABLE, 0x48)]);
    const MASCARAS: [(usize, [u32; 5]); 5] = [
        (0x6f09f0, XY_ARG_MASK),
        (0x6f46b0, XY_ARG_MASK),
        (0x6f5b80, [0xff, 0, 0, 0, 0]),
        (0x6d6f70, [0xffff, u32::MAX, 0xff, 0xff, 0xff]),
        (0x6f0630, [0; 5]),
    ];
    let (_, _, igual, sequencia, k) = compare_com_fita(reg, || o(j.addr()), |chamar| {
        off_ball_move_injected(j, &mut |rotina, this, args| {
            let mask = MASCARAS.iter().find(|m| m.0 == rotina).map_or(XY_ARG_MASK, |m| m.1);
            chamar(rotina, this, args, mask)
        })
    });
    SCOREBOARD.record(igual && sequencia, || format!("move_off_ball acao {:#x} {} chamadas", j.action(), k));
}

/// Comparer for `decide_off_ball`.
pub(crate) unsafe extern "thiscall" fn compare_decide_off_ball(j: usize, a: u32) {
    let j = MatchPlayer(j);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_DECIDE_OFF_BALL);
    if is_recording_top_level() {
        call_recording(FN_MATCH_PLAYER_CHOOSE_ATTACKING_OFF_BALL_MOVE, j.addr(), [a, 0, 0, 0, 0], || {
            o(j.addr(), a);
            0
        });
        return;
    }
    // the Rust copy runs after the whole original: what it (and what it calls without writing: 0x6c2be0,
    // 0x6bce80, 0x6c4130...) reads before a recorded call and that this call changes must go back to
    // previous state. The recorded calls touch the player (cell, direction), the grid and the rows.
    let m = j.engine();
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    // the written 0x6f09f0 also moves the other players (decides for them): all of them go in
    reg.extend([(m + 0x1a0, 0x30), (m + engine::GRID, 0x8f00 - 0x215e), (m + engine::FORMATION_TABLE, 0x48)]);
    // 0x6f09f0 e 0x6f46b0 leem x e y como bytes; 0x6f5b80, so o `direto`
    const MASCARAS: [(usize, [u32; 5]); 3] = [(0x6f09f0, XY_ARG_MASK), (0x6f46b0, XY_ARG_MASK), (0x6f5b80, [0xff, 0, 0, 0, 0])];
    let (_, _, igual, sequencia, k) = compare_com_fita(reg, || o(j.addr(), a), |chamar| {
        off_ball_decide_injected(j, &mut |rotina, args| {
            let mask = MASCARAS.iter().find(|m| m.0 == rotina).map_or(XY_ARG_MASK, |m| m.1);
            chamar(rotina, j.addr(), args, mask)
        })
    });
    SCOREBOARD.record(igual && sequencia && k == 1, || format!("decide_off_ball acao {:#x} {} chamadas: {}", j.action(), k, tape_detail()));
}

/// Comparer for `advance_row`.
pub(crate) unsafe extern "thiscall" fn compare_advance_row(this: usize, direct: u8) -> u32 {
    let this = MatchPlayer(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u8) -> u32>(IDX_ADVANCE_ROW);
    if is_recording_top_level() {
        return call_recording(FN_MATCH_PLAYER_DROP_BACK_ONE_ROW, this.addr(), [direct as u32, 0, 0, 0, 0], || o(this.addr(), direct));
    }
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.push((this.addr() + match_player::CELL_X, 2));
    let (r, _, igual, sequencia, k) = compare_com_fita(reg, || o(this.addr(), direct), |chamar| {
        advance_row_injected(this, direct, &mut |rotina, a| chamar(rotina, this.addr(), a, XY_ARG_MASK))
    });
    // the answer and the call’s `eax` (the movement returns nothing): not part of the comparison
    SCOREBOARD.record(igual && sequencia && k == 1, || format!("advance_row({}) {} chamadas", direct, k));
    r
}

/// Comparer for `designated_marked`.
pub(crate) unsafe extern "thiscall" fn compare_designated_marked(this: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_DESIGNATED_MARKED)(this);
    let n = designated_marked(this);
    SCOREBOARD.record(r == n, || format!("designated_marked original={:x} rust={:x}", r, n));
    r
}
